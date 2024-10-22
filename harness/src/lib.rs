pub mod bench;
pub mod cli;

use std::{
    env,
    fmt::{self, Debug, Display, Formatter},
    fs,
    io::ErrorKind,
    path::PathBuf,
};

use anyhow::{Context, Result};
use directories::ProjectDirs;
use once_cell::sync::OnceCell;
use reqwest::{blocking::Client, header};

const USER_AGENT: &str =
    "Script by David Stangl (david@david-stangl.com, github.com/Felerius/advent-of-code)";

static CACHED_STATE: OnceCell<CachedState> = OnceCell::new();

struct CachedState {
    downloads_dir: PathBuf,
    http_client: Client,
    aoc_session: String,
}

pub fn get_input(year: usize, day: usize) -> Result<String> {
    let cached = CACHED_STATE.get_or_try_init(|| {
        let downloads_dir = ProjectDirs::from("", "felerius", "advent-of-code")
            .context("could not determine home directory")?
            .cache_dir()
            .to_path_buf();
        let http_client = Client::builder()
            .user_agent(USER_AGENT)
            .build()
            .context("failed to create HTTP client")?;
        let aoc_session = env::var("AOC_SESSION")
            .context("AOC_SESSION environment variable not set or invalid")?;
        anyhow::Ok(CachedState {
            downloads_dir,
            http_client,
            aoc_session,
        })
    })?;

    let path = cached.downloads_dir.join(format!("{year}-{day:02}.txt"));
    fs::read_to_string(&path)
        .or_else(|err| {
            if err.kind() == ErrorKind::NotFound {
                let url = format!("https://adventofcode.com/{year}/day/{day}/input");
                let input = cached
                    .http_client
                    .get(&url)
                    .header(header::COOKIE, format!("session={}", cached.aoc_session))
                    .send()
                    .context("failed to fetch input data from adventofcode.com")?
                    .text()
                    .context("failed to decode input data from adventofcode.com")?;

                fs::create_dir_all(&cached.downloads_dir)
                    .context("failed to create cache directory")?;
                fs::write(&path, &input)
                    .context("failed to write downloaded input data to file")?;
                Ok(input)
            } else {
                Err(err).context("failed to read input data from file")
            }
        })
        .map(|mut input| {
            input.truncate(input.trim_end().len());
            input
        })
}

pub struct Solution(Option<(Box<dyn Display>, Box<dyn Display>)>);

impl Debug for Solution {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        struct DebugWithDisplay<'a>(&'a Box<dyn Display>);

        impl Debug for DebugWithDisplay<'_> {
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        let inner = self
            .0
            .as_ref()
            .map(|(part1, part2)| (DebugWithDisplay(part1), DebugWithDisplay(part2)));
        f.debug_tuple("Solution").field(&inner).finish()
    }
}

impl From<()> for Solution {
    fn from(_: ()) -> Self {
        Self(None)
    }
}

impl<S: Display + 'static, T: Display + 'static> From<(S, T)> for Solution {
    fn from((part1, part2): (S, T)) -> Self {
        Self(Some((Box::new(part1), Box::new(part2))))
    }
}

pub trait IntoResultSolution {
    fn into(self) -> Result<Solution>;
}

impl<T: Into<Solution>> IntoResultSolution for T {
    fn into(self) -> Result<Solution> {
        anyhow::Ok(self.into())
    }
}

impl<T, E> IntoResultSolution for Result<T, E>
where
    T: Into<Solution>,
    anyhow::Error: From<E>,
{
    fn into(self) -> Result<Solution> {
        Ok(self?.into())
    }
}

#[doc(hidden)]
pub mod __macro_support {
    pub use concat_idents::concat_idents;
    pub use divan::{self, bench, main, Bencher};
}
