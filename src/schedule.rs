pub mod sm2;

pub trait Review: Ord + Default {
    type Score: Query;

    fn review(&mut self, score: Self::Score);
}

#[deprecated]
pub trait Query {
    fn query() -> Self;
}
