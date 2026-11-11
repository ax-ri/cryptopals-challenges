mod set1;
mod set2;
mod utils;

fn main() {
    let challenges = [
        vec![
            set1::chall1::run,
            set1::chall2::run,
            set1::chall3::run,
            set1::chall4::run,
            set1::chall5::run,
            set1::chall6::run,
            set1::chall7::run,
            set1::chall8::run,
        ],
        vec![set2::chall9::run],
        vec![],
    ];
    let run_chall = |set: usize, chall: usize| challenges[set - 1][chall - 1]();

    run_chall(2, 1);
}
