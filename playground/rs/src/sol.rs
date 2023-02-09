use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Input {
    pub answer: (),
    pub case: Case,
}

impl Input {
    pub fn get_output(self: &Self) {
        unimplemented!()
    }
}

#[derive(Deserialize, Debug)]
pub struct Case;

pub struct Solution;
