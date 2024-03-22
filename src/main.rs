use std::time::Duration;
use std::thread;
use rand::Rng;

fn main() {
    let mut buffer: [bool; 20] = [false; 20];
    let mut rng = rand::thread_rng();
    let mut producer_position: u8 = 0;
    let mut consumer_position: u8 = 0;
    let mut random_number: u8;
    let mut free_space: u8 = 20;

    loop {
        random_number = rng.gen_range(1..5);
        println!("Producing {}", random_number);
        produce(&mut buffer, random_number, &mut producer_position, &mut free_space);
        for &value in buffer.iter() { // print buffer
            if value {
                print!("x");
            } else {
                print!("_");
            }
        }
        println!();
        println!();
        thread::sleep(Duration::from_secs(2));

        random_number = rng.gen_range(1..5);
        println!("Consuming {}", random_number);
        consume(&mut buffer, random_number, &mut consumer_position, &mut free_space);
        
        for &value in buffer.iter() { // print buffer
            if value {
                print!("x");
            } else {
                print!("_");
            }
        }
        println!();
        println!();
        thread::sleep(Duration::from_secs(2));
    }
}

fn produce(buffer: &mut [bool; 20], number: u8, position: &mut u8, free_space: &mut u8) {

    if *free_space < number{ // verify enough free space
        println!("Buffer free space is {}, crabn't produce {}.", *free_space, number);
        return;
    }
    
    for _ in 0..number { // fill buffer with the number of elements
        if *position == 20 { // reset position if it reaches the end of the buffer
            *position = 0;
        }
        buffer[*position as usize] = true;
        *position += 1; 
        *free_space -= 1;
    }
}

fn consume(buffer: &mut [bool; 20], number: u8, position: &mut u8, free_space: &mut u8) {
    let product: u8 = 20 - *free_space;
    if product < number{ // verify enough products to consume
        println!("Buffer has only {} products, crabn't consume {}.", product, number);
        return;
    }

    for _ in 0..number { // fill buffer with the number of elements
        if *position == 20 { // reset position if it reaches the end of the buffer
            *position = 0;
        }
        buffer[*position as usize] = false;
        *position += 1; 
        *free_space += 1;
    }

}
