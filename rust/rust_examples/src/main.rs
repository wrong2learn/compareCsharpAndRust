use models::Person;
mod models;

fn main() {
    // Create a vector of Person objects
    let mut people: Vec<Person> = Vec::new();

    // Create a Person object and push it to the vector
    let person1 = Person {
        name: String::from("John"),
        age: 25,
    };
    people.push(person1);

    let person2: Person = Person {
        name: String::from("Jane"),
        age: 22,
    };
    people.push(person2);

    // Print the list of people
    println!("List of people:");
    for person in &people {
        println!("Name: {}, Age: {}\n", person.name, person.age);
    }
}
