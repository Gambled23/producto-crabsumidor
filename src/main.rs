use rand::Rng;

fn main() {
    let mut buffer: [bool; 20] = [false; 20];
    let mut rng = rand::thread_rng();
    let mut producer_position: u8 = 0;
    let mut consumer_position: u8 = 0;

    loop {
        let random_number: u8 = rng.gen_range(1..5);
        println!("Random number: {}", random_number);
        produce(&mut buffer, random_number, &mut producer_position);
        consume(&mut buffer, random_number, &mut consumer_position);
    }
}

fn produce(buffer: &mut [bool; 20], number: u8, position: &mut u8) {
    let free_space: u8 = 20 - *position;
    if free_space < number{
        println!("Buffer is full, crabn't produce more.");
        return;
    }
    for i in 0..number {
        buffer[(i as usize) + (*position as usize)] = true;
    }
    *position += number;
}

fn consume(buffer: &mut [bool; 20], number: u8, position: &mut u8) {

}
