use vessels::object;

#[object]
pub trait Test: Send {
    fn test(&self, string: String) -> u32;
    fn other_test(&mut self, test: u32);
}

fn main() {}
