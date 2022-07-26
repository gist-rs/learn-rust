trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("Puppy")
    }
}

fn test<T: Animal>() {
    println!("{}", <T as Animal>::baby_name());
}

fn main() {
    test::<Dog>();
}
