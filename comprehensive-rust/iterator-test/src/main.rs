#![allow(dead_code)]
use std::collections::HashMap;

#[derive(Debug)]
struct Person {
    pub id: String,
    pub name: String,
    pub email: String,
}

#[derive(Debug)]
struct SomeDb {
    size: usize,
}

impl SomeDb {
    fn new(size: usize) -> Self {
        Self { size }
    }

    fn get_data(&self) -> Option<Vec<Person>> {
        if self.size == 0 {
            // no rows found
            None
        } else {
            (0..self.size).map(|i| Person {
                id: format!("{i}"),
                name: format!("first{i} last{i}"),
                email: format!("test{i}@walmart.com")
            }).collect::<Vec<_>>().into()
        }
    }
}

struct PeopleService {
    db: SomeDb,
}

impl PeopleService {
    pub fn new(db:SomeDb) -> Self {
        Self { db }
    }

    // Complete the get_people function.
    // We model the result as an Option since there might not be any people in our db, and None is
    // a reasonable way to signal that.

    pub fn get_people(&self) -> Option<HashMap<String, Person>> {
        // Hints:
        // Option<T>.map is called when the Option<T>'s value is Some(T), so get_data().map(..) means
        // we received data, see: https://doc.rust-lang.org/std/option/index.html .
        //
        // Hint: You can create a HashMap from an iterator of (TKey, TValue) tuples, for example:
        // let map: HashMap<&str, i32> = [("a", 0), ("b", 1)].into_iter().collect();
        //
        // Hint: you'll likely hit a borrow checker error when making your HashMap, try and solve it!
        // We'll certainly discuss ways out of it :)
        //
        let mut map = HashMap::new();
        // Uncomment the lines below to get started
        self.db.get_data().map(|rows| {
            // yay we have rows!
            map = rows.into_iter().map(|person| {
                (person.id.clone(), person)
            }).collect();
            // for person in rows {
            //     map.insert(person.id.clone(), person);
            // }
        });

        // remove the line below
        Some(map)
        // if map.is_empty() {
        //     None
        // } else {
        //     Some(map)
        // }  
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_gets_people() {
        let amount_of_people = 2;
        let db = SomeDb::new(amount_of_people);
        let service = PeopleService::new(db);

        let result = service.get_people().expect("We should have received people");

        assert_eq!(result.keys().len(), amount_of_people);
    }

    #[test]
    fn it_handle_no_people() {
        let amount_of_people = 0;
        let db = SomeDb::new(amount_of_people);
        let service = PeopleService::new(db);

        let result = service.get_people();

        assert!(result.is_none());
    }
}
