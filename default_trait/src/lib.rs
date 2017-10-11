#[derive(Default, Debug)]
struct Bar {
    a: u8,
}

#[derive(Default, Debug)]
pub struct Foo<'a> {
    a: u8,
    b: u8,
    c: &'a str,
    d: bool,
    e: Bar,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let s: &str = Default::default();
        assert_eq!("", s);
    }
    use super::Foo;
    fn default_struct() {
        let s: Foo = Foo::default();
        assert_eq!(0, s.a);
        assert_eq!(false, s.d);
        assert_eq!(0, s.e.a);
    }
}
