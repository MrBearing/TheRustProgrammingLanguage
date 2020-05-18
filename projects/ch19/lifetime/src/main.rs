trait Red { }

#[derive(Debug)]
struct Ball<'a> {
    diameter: &'a i32,
}

impl<'a> Red for Ball<'a> { }

fn main() {
    let num = 5;

    let _obj = Box::new(Ball { diameter: &num }) as Box<dyn Red>;
}