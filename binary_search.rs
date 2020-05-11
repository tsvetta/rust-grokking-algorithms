fn binary_search_in_numbers_array (list: &mut[i32], item: i32) -> Option<usize> {
  let mut lowest_position = 0;
  let mut highest_position = list.len() - 1;

  while lowest_position <= highest_position {
    let middle_position = (lowest_position + highest_position) / 2; // (0 + 10 == 10) / 2 = 5  0 1 2 3 4 [5] 6 7 8 9 10
    let guess: i32 = list[middle_position]; 

    if guess == item {
      return Some(middle_position);
    }

    if guess < item {
      lowest_position = middle_position + 1;
    }

    if guess > item {
      highest_position = middle_position - 1;
    }
  }

  None
}

fn say_result(position:Option<usize>) {
  match position {
      Some(inner)   => println!("position {}", inner),
      None          => println!("position not found"),
  }
}

pub fn run() {
  let mut numbers_array = [1,2,3,4,5,6,7,8,9,10,11,27,29,32,37,59,455,1123];

  let mut position = binary_search_in_numbers_array(&mut numbers_array, 455);

  say_result(position);

  position = binary_search_in_numbers_array(&mut numbers_array, 25);

  say_result(position);

  position = binary_search_in_numbers_array(&mut numbers_array, 1);

  say_result(position);
}