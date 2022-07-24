use std::env;

#[derive(Debug)]
pub struct ImgArgs {
    pub img_1: String,
    pub img_2: String,
    pub output: String,
}

impl ImgArgs {
    pub fn new() -> Self {
        Self {
            img_1: get_nth_arg(1),
            img_2: get_nth_arg(2),
            output: get_nth_arg(3),
        }
    }
}

fn get_nth_arg(n: usize) -> String {
    env::args().nth(n).unwrap()
}
