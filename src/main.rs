use std::collections::HashMap;

fn main() {
/*    const NUMBERS: &str = "
    _  _     _  _  _  _  _  _ 
  | _| _||_||_ |_   ||_||_|| |
  ||_  _|  | _||_|  ||_| _||_|
";*/
    const NUMBERS: &str = "
 _  _  _  _  _  _  _  _  _ 
|_||_||_||_||_||_||_||_||_|
 _| _| _| _| _| _| _| _| _|
";
    // 0, 1, 2, 3, 4, 5, 6, 7, 8, 9
    const WEIGHTS: [i32; 10] = [184, 88, 144, 172, 133, 152, 175, 106, 212, 189];

    let primes: HashMap<char, i32> = [('|', 2), ('_', 3)].iter().cloned().collect();
    let positions_weights = [3, 5, 7];

    let mut result: Vec<i32> = vec![];

    for (line_nr, line) in NUMBERS.lines().enumerate() {
        if line_nr == 0 {
            continue;
        }
        assert_eq!(0, line.chars().count() % 3);
        let mut number_position: usize = 0;

        for (char_nr, char) in line.chars().enumerate() {
            if char_nr % 3 == 0 {
                if result.get(number_position).is_none() {
                    // first insert a zero on that *index*
                    result.insert(number_position, 0);
                }
                
                // then add +1 to the position to have the number, not the index
                number_position += 1;
            }
            // skip empty spaces
            if char == ' ' {
                continue;
            }
            
            let char_weight = primes.get(&char).unwrap() + positions_weights[char_nr % 3 as usize] * positions_weights[(line_nr-1) % 3];
            let range = number_position - 1;

            let new_weight: i32 = match result.get(range) {
                Some(&position_weight) => {
                    position_weight + char_weight
                }
                None => char_weight
            };
            let insert_vec = vec![new_weight];

            result.splice(range..number_position, insert_vec.iter().cloned());
        }
    }
    
    let mut numbers_result: Vec<i32> = vec![];
    for weight in result {
        let number = WEIGHTS.iter().enumerate().find(|&number_weight| &weight == number_weight.1).unwrap().0;
        numbers_result.push(number as i32);
    }
    println!("results {:?}", numbers_result);

}