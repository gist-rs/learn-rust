#[derive(Debug)]
struct Foo {
    a: i32, // Let say we had `a` from start.
    b: i32, // And will get `b` from Bar.
}

#[derive(Debug)]
struct Bar {
    b: i32, // Take me!
}

impl From<Bar> for Foo {
    fn from(bar: Bar) -> Self {
        Self {
            a: Default::default(),
            b: bar.b,
        }
    }
}

fn main() {
    let bar = Bar { b: 456 };
    let foo = Foo {
        a: 123,
        ..bar.into()
    };

    println!("{:?}", foo);
}
