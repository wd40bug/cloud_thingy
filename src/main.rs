use names::Generator;
use rand::Rng;

const POS_PEOPLE: usize = 3;
const NEG_PEOPLE: usize = 3;
const PEOPLE: usize = 10;

fn main() {
    let mut generator = Generator::default();
    let mut people = Vec::new();
    for _ in 0..PEOPLE {
        people.push(Person::new(&mut generator));
    }
    for i in 0..PEOPLE {
        for j in NEG_PEOPLE..POS_PEOPLE {
            let mut num = i;
            while num == i {
                num = rand::thread_rng().gen_range(0..PEOPLE);
            }
        }
    }
}

struct Person {
    name: String,
}

impl Person {
    fn new(gen: &mut Generator) -> Self {
        Person {
            name: gen.next().unwrap(),
        }
    }
}

struct Connection {
    start: usize,
    end: usize,
    weight: i8,
}
struct ConnectionHandler {
    connections: Vec<Connection>,
}
