#[derive(Clone)]
#[derive(Debug)]
struct Dog {
  name: String,
  age: i8,
}

fn find_oldest_dog(dogs: Vec<Dog>) -> usize {
  let end = dogs.len() - 1;
  let mut oldest_dog = dogs[0].clone();
  let mut oldest_dog_index = 0;

  for i in 0..=end {
    if dogs[i].age > oldest_dog.age {
      oldest_dog = dogs[i].clone();
      oldest_dog_index = i;
    }
  }

  return oldest_dog_index;
}

fn seletion_sort_by_age(mut dogs: Vec<Dog>) -> Vec<Dog> {
  let mut sorted_dogs: Vec<Dog> = vec![];

  while dogs.len() > 0 {
    let oldest_dog_index = find_oldest_dog(dogs.clone());
    let oldest_dog = dogs[oldest_dog_index].clone();

    dogs.drain(oldest_dog_index..oldest_dog_index+1);
    sorted_dogs.push(oldest_dog);
  }

  return sorted_dogs;
}

pub fn run() {
  // Vector – re-sizable array
  let unsorted_array = vec![
    Dog { name: "Berta".to_string(), age: 4},
    Dog { name: "Jessie".to_string(), age: 7},
    Dog { name: "Timka".to_string(), age: 25},
    Dog { name: "Cerber".to_string(), age: 30},
    Dog { name: "Pyos".to_string(), age: 1},
    Dog { name: "Adel".to_string(), age: 1},
  ];

  let sorted_dogs = seletion_sort_by_age(unsorted_array.clone());

  println!("Dogs: {:?}", sorted_dogs);
}