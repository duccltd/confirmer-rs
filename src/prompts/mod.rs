use console::Term;

pub mod confirm;

pub trait Actionable<T> {
    fn term_interact(&self, term: &Term) -> std::io::Result<T>;

    fn interact(&self) -> std::io::Result<T>;
}