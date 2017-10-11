struct Foo<T: ?Sized>(T);

struct UseFoo(Foo<[i32]>);

fn main() {
    let f = Foo( [1; 2; 3] );
}
