// -------------------------------------------
// Structures
// -------------------------------------------

fn main() {
    // -------------------------------------------
    // instanciate a structure, namely a struct
    // -------------------------------------------
    let person1 = Person {
        name: String::from("John Doe"),
        citizenship: String::from("Honeymoonland"),
        age: 42,
        gender: 'd',
        salary: 1_424_243,
    };

    // access field of a instance of a structure
    println!(
        "the values of the structure are:
        name\t\t= {},\n\tage\t\t= {},\n\tcitizenship\t= {}\n",
        person1.name, person1.age, person1.citizenship
    );

    // call structure function
    println!(
        "Taxes of '{}' are: {}\n",
        person1.name,
        person1.compute_taxes()
    );
    println!(
        "Taxes of '{}' are: {}\n",
        person1.name,
        // use as class function (must provide a reference to instance)
        Person::compute_taxes(&person1)
    );

    let person2 = Person::new();
    println!(
        "person initialized with default values has the following values:
        citizenship: {}, name: {}, age: {}, gender: {}, salary: {}\n",
        person2.citizenship, person2.name, person2.age, person2.gender, person2.salary
    );

    // instantiate new structure and copy values from another
    let person3 = Person {
        age: 50,
        name: String::from("Another Name"),
        // ..<instancename> says that all other (not specified) values should be copied
        ..person1
    };
    println!(
        "new instance with copied values:
        citizenship: {}, name: {}, age: {}, gender: {}, salary: {}\n",
        person3.citizenship, person3.name, person3.age, person3.gender, person3.salary
    );

    /* the examples above show immutable instances, so that values of the fields could not be changed */
    // create a mutable instance:
    let mut person4 = Person::new();
    println!("default of person4.name: '{}'", person4.name);
    println!("update the name of person4");
    person4.name = String::from("new changed name");
    println!("the updated name of person4 is: '{}'\n", person4.name);

    // -------------------------------------------
    // instantiate a tuple struct
    // -------------------------------------------
    let some_nums = Numbers(42, 16);
    println!(
        "the values of the two fiels in the tuple structure are:
        1={}, 2={}\n",
        some_nums.0, some_nums.1
    );

    // call tuple struct function
    println!(
        "the greater number of some_nums is: {}",
        some_nums.greater()
    );
    println!(
        "the lesser number of some_nums is: {}\n",
        some_nums.lesser()
    );
}

struct Person {
    // -------------------------------------------
    // declare a structure
    //      - camelCase convention
    //      - outside of the main function
    // -------------------------------------------

    // fields
    citizenship: String,
    name: String,
    age: i32,
    gender: char,
    salary: i32,
}

impl Person {
    // -------------------------------------------
    // implement specific behavior for structures
    //      - functions
    // -------------------------------------------

    fn new() -> Self {
        /* this function returns a structure of type Person (Self) */
        Self {
            citizenship: String::from("anycountry"),
            name: String::from("who ever"),
            age: 11,
            gender: 'd',
            salary: 0,
        }
    }

    fn compute_taxes(&self) -> f32 {
        /* an arbitrary function with output and
        reference to &self (instance) */
        (self.salary as f32 / 3.) * 0.5
    }
}

// -------------------------------------------
// declare a tuple structure
//      - they have a name
//      - the fields have no names
//      - can come in handy when typical tuples should have
//          another type for differentiation
// -------------------------------------------
struct Numbers(i32, i32);

impl Numbers {
    // -------------------------------------------
    // implement specific behavior for the tuple struct
    // -------------------------------------------
    fn greater(&self) -> i32 {
        // assuming values are never equal
        if self.0 > self.1 {
            self.0
        } else {
            self.1
        }
    }

    fn lesser(&self) -> i32 {
        // assuming values are never equal
        if self.0 < self.1 {
            self.0
        } else {
            self.1
        }
    }
}
