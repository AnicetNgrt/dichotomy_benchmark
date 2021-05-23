use dichotomy_benchmark::*;

use std::time::Instant;
use std::collections::LinkedList;


fn read_int() -> u64 {
    let mut int = String::new();

    std::io::stdin()
        .read_line(&mut int)
        .expect("Failed to read line");
    
    let int_parsed = int.trim().parse::<u64>();
    match int_parsed {
        Result::Err(_) => {
            if "max".to_owned().eq(int.trim()) {
                u64::MAX
            } else {
                panic!()
            }
        },
        Result::Ok(res) => res
    }
}

fn main() {
    println!("Guess the number!");

    println!("Maximum number (\"max\" for u64's max): ");
    let max = read_int();
    println!("Games count: ");
    let game_count = read_int();
    println!("Repetitions: ");
    let batch_size = read_int();

    let mut durations: LinkedList<f64> = LinkedList::new();
    let mut tries_counts: LinkedList<u64> = LinkedList::new();
    let mut player = Dichotomie::new();

    for i in 0..batch_size {
        let now = Instant::now();

        for _ in 0..game_count {
            let mut game = Game::new(0, max);
            while let Next::Continue = game.play(&mut player) {}
            tries_counts.push_front(game.turn);
        }

        let duration = now.elapsed().as_secs_f64();
        durations.push_front(duration);

        println!("{} / {}", i+1, batch_size);
    }

    let mut duration_sum: f64 = 0.0;
    let mut duration_count: f64 = 0.0;

    for duration in durations.iter() {
        duration_sum += duration;
        duration_count += 1.0;
    }

    println!("Average time for {} game(s) is {} seconds", game_count, duration_sum/duration_count);

    let mut tries_counts_sum: u64 = 0;
    let mut games_count: u64 = 0;

    for tries in tries_counts.iter() {
        tries_counts_sum += tries;
        games_count += 1;
    }

    println!("Average tries with max number {} is {}", max, tries_counts_sum/games_count);

}
