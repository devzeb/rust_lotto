use rand::distributions::Uniform;
use rand::Rng;
use std::io;

fn draw_numbers() -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let mut numbers_drawn = vec![];
    loop {
        let number = rng.sample(Uniform::new(1, 50));
        if !numbers_drawn.contains(&number) {
            numbers_drawn.push(number);
            if numbers_drawn.len() == 6 {
                break;
            }
        }
    }

    numbers_drawn.sort();

    return numbers_drawn;
}

fn get_equal_elements<T : std::cmp::Eq>(collection1 : Vec<T>, collection2 : Vec<T>) -> Vec<T>{
    let mut result = vec!{};

    for item in collection1{
        if collection2.contains(&item){
            result.push(item);
        }
    }

    return result;
}

fn convert_line_to_numbers(line : String) -> Vec<u8>{
    fn strip_line_ending(line_ : &str) -> &str {
        return line_
            .lines()
            .next()
            .unwrap();
    }

    let mut numbers: Vec<u8> = strip_line_ending(&line)
        .split(" ")
        .filter_map(|x| x.parse::<u8>().ok())
        .filter(|&x| (x <= 49) && (x > 0) )
        .collect();

    numbers.sort();
    numbers.dedup();

    numbers
}

fn main() {
    println!("Lotto 6 from 49!");

    let numbers = loop {
        println!("Please enter 6 different numbers from 1 to 49, separated by a space.");

        // String with capacity for 6*2 digits + 5 white spaces between the numbers
        let mut input = String::with_capacity(6*2 + 5);

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let numbers_input = convert_line_to_numbers(input);

        if numbers_input.len() == 6 {
            break numbers_input;
        }
    };

    println!("Your numbers: {:?}", numbers);

    let numbers_drawn = draw_numbers();

    println!("These numbers were drawn: {:?}", numbers_drawn);

    let equal_numbers = get_equal_elements(numbers, numbers_drawn);
    println!("");
    println!("You guessed {} out of 6 correctly. Correct numbers: {:?}", equal_numbers.len(), equal_numbers);
}
