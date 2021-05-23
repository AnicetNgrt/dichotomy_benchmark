use std::cmp::Ordering;

pub enum Next {
    Continue,
    Stop
}

#[derive(Copy, Clone)]
pub struct Guess(u64, Ordering);

pub struct Game {
    pub min: u64,
    pub max: u64,
    pub number: u64,
    pub last_guess: Option<Guess>,
    pub turn: u64
}

pub trait Player {
    fn feed_initial(&mut self, min: u64, max: u64);
    fn feed_last_guess(&mut self, last_guess: &Guess);
    fn play(&self) -> u64;
}

pub struct Dichotomie {
    min: u64,
    max: u64
}

struct Clue {
    pub min: u64,
    pub max: u64,
    pub last_guess: Option<Guess>,
}

impl Dichotomie {
    pub fn new() -> Self {
        Dichotomie {
            min: 0,
            max: 1
        }
    }
}

impl Game {
    pub fn new(min: u64, max: u64) -> Self {
        Game {
            min: min,
            max: max,
            number: fastrand::u64(min..max),
            last_guess: Option::None,
            turn: 0
        }
    }

    pub fn play<P: Player>(&mut self, player: &mut P) -> Next {
        self.turn += 1;

        match self.clue().last_guess.as_ref() {
            Option::Some(guess) => {
                player.feed_last_guess(guess)
            },
            Option::None => {
                player.feed_initial(self.min, self.max)
            }
        };

        let number = player.play();

        match self.cmp_guess(number) {
            Ordering::Equal => Next::Stop,
            cmp => {
                let guess = Guess(number, cmp);
                self.last_guess.replace(guess);
                Next::Continue
            },
        }
    }

    fn clue(&self) -> Clue {
        Clue {
            min: self.min,
            max: self.max,
            last_guess: self.last_guess,
        }
    }

    fn cmp_guess(&self, number: u64) -> Ordering {
        self.number.cmp(&number)
    }
}

impl Dichotomie {
    fn get_middle(&self) -> u64 {
        self.min + ((self.max - self.min) / 2)
    }
}

impl Player for Dichotomie {
    fn feed_initial(&mut self, min: u64, max: u64) {
        self.min = min;
        self.max = max;
    }

    fn feed_last_guess(&mut self, last_guess: &Guess) {
        let middle = self.get_middle();

        let (min, max) = match last_guess.1 {
            Ordering::Equal => (self.min, self.max),
            Ordering::Greater => (middle, self.max),
            Ordering::Less => (self.min, middle),
        };

        self.min = min;
        self.max = max;
    }

    fn play(&self) -> u64 {
        match self.get_middle() {
            m if m == self.min => self.max,
            m if m == self.max => self.min,
            m => m,
        }
    }
}