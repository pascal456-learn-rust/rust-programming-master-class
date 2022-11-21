// -------------------------------------------
// Enums
//      - General Syntax
// -------------------------------------------

fn main() {
    let participant1 = Conveyence::Car;
    println!("Value of the option is {}", participant1 as i32); // will print a sequence number of the enum member

    let participant2 = Conveyence::Air;
    let participant3 = Conveyence::Train;

    println!(
        "The participant 1 has a travel allowance of {}",
        participant1.travel_allowance(60)
    );
    println!(
        "The participant 2 has a travel allowance of {}",
        participant2.travel_allowance(120)
    );
    println!(
        "The participant 3 has a travel allowance of {}",
        participant3.travel_allowance(60)
    );

    // -------------------------------------------
    // Enums with attached data
    // -------------------------------------------
    let traveller1 = ConveyenceAttachedData::Car(60);
    let traveller2 = ConveyenceAttachedData::Train(120);
    let traveller3 = ConveyenceAttachedData::Air(60);
    println!(
        "Traveller 1 has a travel allowance of {}",
        traveller1.travel_allowance()
    );
    println!(
        "Traveller 2 has a travel allowance of {}",
        traveller2.travel_allowance()
    );
    println!(
        "Traveller 3 has a travel allowance of {}",
        traveller3.travel_allowance()
    );

    // -------------------------------------------
    // Enums to create vector with different types of data
    // -------------------------------------------

    let some_value = vec![Value::Integer(12), Value::Float(15.5)];
    println!(
        "Value of the Integer is {:?}
    and the value of the Float is {:?}",
        some_value[0], some_value[1]
    );

    for i in some_value.iter() {
        match i {
            Value::Integer(num) => println!("Value is an integer with a value of {}", num),
            Value::Float(num) => println!("Value is a Float with a value of {}", num),
        }
    }
}

enum Conveyence {
    Car = 15,   // the sequence number can be given as a fixed number
    Train = 20, // you can also give a type (Enums with attached data)
    Air = 30,
}

impl Conveyence {
    fn travel_allowance(&self, miles: i32) -> f32 {
        let allowance = match self {
            Conveyence::Car => miles as f32 * 14.0 * 2.0,
            Conveyence::Train => miles as f32 * 18.0 * 2.0,
            Conveyence::Air => miles as f32 * 30.0 * 2.0,
        };
        allowance
    }
}

enum ConveyenceAttachedData {
    // -------------------------------------------
    // Enums with attached data
    // -------------------------------------------
    Car(i32),
    Train(i32), // you can also give a type (Enums with attached data)
    Air(i32),
}

impl ConveyenceAttachedData {
    fn travel_allowance(&self) -> f32 {
        let allowance = match self {
            ConveyenceAttachedData::Car(miles) => *miles as f32 * 14.0 * 2.0,
            ConveyenceAttachedData::Train(miles) => *miles as f32 * 18.0 * 2.0,
            ConveyenceAttachedData::Air(miles) => *miles as f32 * 30.0 * 2.0,
        };
        allowance
    }
}

#[derive(Debug)]
enum Value {
    Integer(i32),
    Float(f32),
}
