use std::{
    env, fs,
    io::ErrorKind,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use collect::PuzzleId;
use directories::ProjectDirs;
use once_cell::sync::OnceCell;
use ureq::Agent;

const USER_AGENT: &str =
    "Script by David Stangl (david@david-stangl.com, github.com/Felerius/advent-of-code)";
const ORGANIZATION: &str = "felerius";
const APPLICATION: &str = "advent-of-code";

struct Downloader {
    cache_dir: PathBuf,
    http_agent: Agent,
}

impl Downloader {
    fn new() -> Result<Self> {
        let cache_dir = ProjectDirs::from("", ORGANIZATION, APPLICATION)
            .context("could not determine home directory")?
            .cache_dir()
            .to_path_buf();
        let http_agent = Agent::config_builder()
            .user_agent(USER_AGENT)
            .build()
            .new_agent();
        Ok(Self {
            cache_dir,
            http_agent,
        })
    }

    fn get(&self, puzzle_id: PuzzleId) -> Result<String> {
        let cache_file = self.cache_dir.join(format!("{puzzle_id}.txt"));
        fs::read_to_string(&cache_file).or_else(|err| {
            if err.kind() == ErrorKind::NotFound {
                self.download(puzzle_id, &cache_file)
            } else {
                Err(err).context("failed to read input data from cache file")
            }
        })
    }

    fn download(&self, puzzle_id: PuzzleId, cache_file: &Path) -> Result<String> {
        let aoc_session_cookie = env::var("AOC_SESSION")
            .context("AOC_SESSION environment variable not set or invalid")?;
        let url = format!(
            "https://adventofcode.com/{}/day/{}/input",
            puzzle_id.year, puzzle_id.day
        );
        let mut input = self
            .http_agent
            .get(url)
            .header("Cookie", format!("session={aoc_session_cookie}"))
            .call()
            .context("request to adventofcode.com failed (expired session cookie?)")?
            .into_body()
            .read_to_string()
            .context("failed to decode input data from adventofcode.com")?;
        input.truncate(input.trim_end().len());

        fs::create_dir_all(&self.cache_dir).context("failed to create cache directory")?;
        fs::write(cache_file, &input)
            .context("failed to write downloaded input data to cache file")?;
        Ok(input)
    }
}

pub(crate) fn get(puzzle_id: PuzzleId) -> Result<String> {
    static DOWNLOADER: OnceCell<Downloader> = OnceCell::new();
    DOWNLOADER.get_or_try_init(Downloader::new)?.get(puzzle_id)
}
