use std::collections::HashMap;
// use std::hash::Hash;

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: HashMap<u32, u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }
    fn get_value(&mut self, arg: u32) -> u32 {
        // if self.value.contains_key(&arg) {
        //     return *self.value.get(&arg).unwrap();
        // }else{
        //     let v = (self.calculation)(arg);
        //     self.value.insert(arg,v);
        //     v
        // }
        self.value.entry(arg).or_insert_with(|| (self.calculation)(arg));
        *self.value.get(&arg).unwrap()

        // match self.value{
        //     Some(v)=>v,
        //     None =>{
        //         let v = (self.calculation)(arg);
        //         self.value = Some(v);
        //         v
        //     }
        // }
    }
}

pub fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| num + 1);
    if intensity < 25 {
        println!("{}", expensive_result.get_value(intensity));
        println!("{}", expensive_result.get_value(intensity + 1));
    } else if random_number == 3 {
        println!("do nothing");
    } else {
        println!("{}", expensive_result.get_value(intensity));
    }
}

#[cfg(test)]
mod tests {
    use super::generate_workout;
    #[test]
    fn test_generate(){
        generate_workout(1, 1);
    }
}
