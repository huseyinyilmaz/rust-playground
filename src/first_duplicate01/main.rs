#[macro_use]
extern crate log;

use pretty_env_logger;

use std::collections::HashMap;

/*
find first duplicate element on given list.
*/

fn find_duplicate(es: Vec<&str>) -> &str {
    let mut seen: HashMap<&str, usize> = HashMap::new();
    let mut first_dup_index: usize = es.len();
    let mut first_dup: &str = "";
    for (i, e) in es.iter().enumerate() {
        if seen.contains_key(e) && seen.get(e).unwrap() < &first_dup_index {
            first_dup_index = *seen.get(e).unwrap();
            first_dup = e;
        } else {
            seen.insert(e, i);
        }


        info!("{}: {}", e, i);
    }
    return first_dup;
}

fn main() {
    pretty_env_logger::init();
    debug!("Hello, world!");
    find_duplicate(vec!["a", "b", "c", "d", "c", "b"]);

}
