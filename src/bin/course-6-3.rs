// -------------------------------------------
// Traits
//      - functions within a trait implementation
// -------------------------------------------
fn main() {
    let data1 = Data {
        some_data: vec![10, 45, 12, 85, 47, 25, 36, 3, 3, 2],
    };
    let data2 = Data {
        some_data: vec![5, 6, 9, 8, 7, 4, 8],
    };
    println!("Mean of the data1: {}", data1.mean());
    println!("Variance of the data1: {}", data1.variance());
    println!("Mean of the data2: {}", data2.mean());
    println!("Variance of the data2: {}", data2.variance());
}

struct Data {
    some_data: Vec<i32>,
}

trait BasicStats {
    fn mean(&self) -> f32;
    fn variance(&self) -> f32;
}

impl BasicStats for Data {
    fn mean(&self) -> f32 {
        let mut sum = 0;
        for i in self.some_data.iter() {
            sum += *i;
        }
        sum as f32 / self.some_data.len() as f32
    }

    fn variance(&self) -> f32 {
        let mu = self.mean();
        let mut sum: f32 = 0.0;
        for i in self.some_data.iter() {
            sum += (*i as f32 - mu).powi(2);
        }
        sum as f32 / self.some_data.len() as f32
    }
}
