#[macro_export]
macro_rules! benches {
    ($year:literal) => {
        $crate::benches!(@private $year, 01, 02, 03, 04, 05, 06, 07, 08, 09, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25);
    };
    (@private $year:literal, $($day:literal),*) => {
        $(
            $crate::__macro_support::concat_idents!(day_name = day, $day {
                #[$crate::__macro_support::bench(crate = $crate::__macro_support::divan)]
                fn day_name(bencher: $crate::__macro_support::Bencher<'_, '_>) {
                    let input = $crate::get_input($year, $day).expect("failed to get input");
                    // Cannot refer to outer concat_idents!() ident in nested one
                    $crate::__macro_support::concat_idents!(lib_crate_name = aoc, $year {
                        use lib_crate_name as lib_crate;
                    });
                    bencher.bench(|| lib_crate::day_name::run(&input));
                }
            });
        )*

        fn main() {
            $crate::__macro_support::main();
        }
    };
}
