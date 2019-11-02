use vessels::object;

#[object]
trait ObjectTest {
    fn test(&self, hello: String) -> u32;
}

fn main() {}
