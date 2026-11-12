mod set1;
mod set2;
mod utils;

fn main() {
    let challenges = [
        set1::chall1::run,
        set1::chall2::run,
        set1::chall3::run,
        set1::chall4::run,
        set1::chall5::run,
        set1::chall6::run,
        set1::chall7::run,
        set1::chall8::run,
        //
        set2::chall9::run,
        set2::chall10::run,
        set2::chall11::run,
        set2::chall12::run,
        set2::chall13::run,
        set2::chall14::run,
        set2::chall15::run,
        set2::chall16::run,
    ];
    let run_chall = |chall: usize| challenges[chall - 1]();
    let run_challs_up_to = |max_chall: usize| {
        for idx in 1..=max_chall {
            run_chall(idx);
        }
    };

    run_challs_up_to(10);
    // run_chall(10);
}
