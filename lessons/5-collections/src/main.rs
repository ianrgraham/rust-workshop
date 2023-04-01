use std::collections::{HashMap, HashSet};

fn main() {
    // ******************** Vectors ********************

    // We've seen one collection quite a lot so far, the humble Vec<T>.

    // let mut my_vec = vec![1u32, 2, 3, 4, 5];
    // or without the macro
    let mut my_vec = Vec::<u32>::new();
    my_vec.extend_from_slice(&[1u32, 2, 3, 4, 5]);

    for elem in &mut my_vec {
        *elem += 10;
    }

    println!("{:?}", my_vec);

    // There are advanced ways of iterating over these collections

    my_vec.iter_mut().for_each(|x| *x += 1);

    // ******************** HashMap and HashSet ********************

    // Another common collection is the HashMap, which is essentially a strongly
    // typed Python dict

    let mut my_map = HashMap::<&str, i32>::new();

    my_map.insert("Miami", 454_279);
    my_map.insert("New York", 8_804_190);
    my_map.insert("Philadelphia", 1_603_797);

    for (key, value) in &my_map {
        println!("City: {:15} Population: {}", key, value);
    }

    if let Some(pop) = my_map.get("Philadelphia") {
        println!("The population of Philly is {}", pop)
    }

    // Derived from the HashMap is the HashSet, which is simply a HashMap with
    // v=()

    let mut my_set = HashSet::<_>::from_iter([1,2,3]);
    
    my_set.insert(4);

}
