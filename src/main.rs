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
        produce(&mut buffer, random_number, &mut producer_position, &mut free_space);
        println!("Producing {}", random_number);
        println!("Buffer: {:?}", buffer);

        thread::sleep(Duration::from_secs(2));

        random_number = rng.gen_range(1..5);
        consume(&mut buffer, random_number, &mut consumer_position, &mut free_space);
        println!("Consuming {}", random_number);
        println!("Buffer: {:?}", buffer);
    }
}

fn produce(buffer: &mut [bool; 20], number: u8, position: &mut u8, free_space: &mut u8) {

    if *free_space < number{ // verify enough free space
        println!("Buffer free space is {}, crabn't produce {}.", *free_space, number);
        return;
    }
    
    for _ in 0..number { // fill buffer with the number of elements
        buffer[*position as usize] = true;
        *position += 1; 
    }
}

fn consume(buffer: &mut [bool; 20], number: u8, position: &mut u8, free_space: &mut u8) {
    let product: u8 = 20 - *free_space;
    if product < number{ // verify enough products to consume
        println!("Buffer has only {} products, crabn't consume {}.", product, number);
        return;
    }

    for i in 0..number { // fill buffer with the number of elements
        buffer[(i as usize) + (*position as usize)] = false;
        *position += 1; 
    }

}
