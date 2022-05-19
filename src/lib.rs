use std::fmt::Display;

use names::Generator;
use rand::Rng;

//generate the handler containing the list of people and the list of connections
pub fn setup(num_people: usize, pos_people: usize, neg_people: usize) -> Handler {
    let mut generator = Generator::default();
    let mut people = Vec::new();
    for _ in 0..num_people {
        people.push(Person::new(&mut generator));
    }
    let mut connections = Vec::<Connection>::new();
    //for each person generate neg_people negative connections and pos_people positive connections each with a weight
    for i in 0..num_people {
        //what numbers have already been chosen
        let mut done = vec![i];
        for j in -(neg_people as isize)..pos_people as isize {
            let mut num = i;
            //keep regenerating until we get a unique one
            while done.contains(&num) {
                num = rand::thread_rng().gen_range(0..num_people);
            }
            done.push(num);
            connections.push(Connection {
                start: i,
                end: num,
                weight: j,
            });
        }
    }
    Handler {
        people,
        connections,
    }
}
pub struct Person {
    name: String,
}

impl Person {
    //generate a random name
    fn new(gen: &mut Generator) -> Self {
        Person {
            name: gen.next().unwrap(),
        }
    }
}
impl Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{: <25}", self.name)
    }
}

impl Display for Connection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{: <3}-> {: <3}: {: <3}",
            self.start, self.end, self.weight
        )
    }
}

pub struct Connection {
    start: usize,
    end: usize,
    weight: isize,
}

pub struct Handler {
    pub people: Vec<Person>,
    pub connections: Vec<Connection>,
}
impl Display for Handler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        for (i, person) in self.people.iter().enumerate() {
            //find connections that start with the current person
            let cons: Vec<&Connection> = self.connections.iter().filter(|x| x.start == i).collect();
            result += &format!("{} | ", person);
            for con in cons {
                result += &format!("{} | ", con);
            }
            result += "\n";
        }
        write!(f, "{}", result)
    }
}
