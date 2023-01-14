macro_rules! my_macro {
    ( $x:expr ) => {format!("Hello {}", $x)};
}

fn main() {
    if my_macro!("world!") != "Hello world!" {
        panic!("Oh no! Wrong output!");
    }
}
