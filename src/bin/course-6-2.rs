// -------------------------------------------
// Traits
//      - abstract description of behaviour among different types
//      - i.e., functions for example
// -------------------------------------------

const PI: f32 = std::f32::consts::PI;

fn main() {
    // -------------------------------------------
    // general explanation
    // -------------------------------------------

    let person1 = Person {
        name: String::from("john doe"),
        citizenship: String::from("lummerland"),
        age: 40,
        gender: 'd',
        salary: 13_337,
    };
    let student1 = Student {
        name_std: String::from("Student Doe"),
        age: 42,
        sex: 'd',
        country: String::from("lalaland"),
    };
    println!("basic info 'Person' includes: {:?}", person1.info());
    println!("basic info 'Student' includes: {:?}", student1.info());

    // -------------------------------------------
    // default implementations
    //      - function implementation in trait definition. (see 'ShapesGeneralInfo' for example)
    //      - e.g. Square -> function area is not implemented
    // -------------------------------------------
    let circle1 = Circle { radius: 3.2 };
    let rectacle1 = Rectangle {
        length: 5.0,
        width: 4.0,
    };
    let square1 = Square { side: 3.0 };

    circle1.area();
    circle1.perimeter();

    rectacle1.area();
    rectacle1.perimeter();

    square1.area();
    square1.perimeter();
}

struct Person {
    citizenship: String,
    name: String,
    age: u8,
    gender: char,
    salary: i32,
}

struct Student {
    name_std: String,
    age: u8,
    sex: char,
    country: String,
}

trait GeneralInfo {
    fn info(&self) -> (&str, u8, char);
    fn country_info(&self) -> &str;
}

impl GeneralInfo for Person {
    fn info(&self) -> (&str, u8, char) {
        (&(self.name), self.age, self.gender)
    }
    fn country_info(&self) -> &str {
        &self.name
    }
}

impl GeneralInfo for Student {
    fn info(&self) -> (&str, u8, char) {
        (&self.name_std, self.age, self.sex)
    }
    fn country_info(&self) -> &str {
        &self.name_std
    }
}

struct Circle {
    radius: f32,
}

struct Rectangle {
    length: f32,
    width: f32,
}

struct Square {
    side: f32,
}

trait ShapesGeneralInfo {
    fn area(&self) {
        println!(
            "Area function not implemented for {}.",
            std::any::type_name::<Self>()
        ); // default implementation
    }
    fn perimeter(&self);
}

impl ShapesGeneralInfo for Circle {
    fn area(&self) {
        let area_of_circle = PI * self.radius.powi(2);
        println!(
            "Area of '{}': {}",
            std::any::type_name::<Self>(),
            area_of_circle
        );
    }
    fn perimeter(&self) {
        let circumference = 2.0 * PI;
        println!(
            "Circumference of '{}': {}",
            std::any::type_name::<Self>(),
            circumference
        );
    }
}

impl ShapesGeneralInfo for Rectangle {
    fn area(&self) {
        let area_of_rectangle = self.length * self.width;
        println!(
            "Area of '{}': {}",
            std::any::type_name::<Self>(),
            area_of_rectangle
        );
    }
    fn perimeter(&self) {
        let perimeter_rect = 2.0 * (self.length + self.width);
        println!(
            "Perimeter of '{}': {}",
            std::any::type_name::<Self>(),
            perimeter_rect
        );
    }
}

impl ShapesGeneralInfo for Square {
    fn perimeter(&self) {
        let perimeter = self.side * 4.0;
        println!(
            "Perimeter of '{}': {}",
            std::any::type_name::<Self>(),
            perimeter
        );
    }
}
