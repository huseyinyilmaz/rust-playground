#[macro_use]
extern crate log;

use pretty_env_logger;

use std::collections::HashMap;

/*
find first duplicate element on given list.
*/

fn find_duplicate(es: Vec<&str>) -> &str {
    // Count all elements and where they appear first.
    //
    // element -> (index, count)
    let mut seen: HashMap<&str, (usize, usize)> = HashMap::new();
    for (i, e) in es.iter().enumerate() {
        if seen.contains_key(e) {
            let (index, count) = *seen.get(e).unwrap();
            seen.insert(e, (index, count + 1));
        } else {
            seen.insert(e, (i, 1));
        }
        info!("{}: {}", e, i);
    }


    let mut els: Vec<(usize, usize, &str)> = seen.drain()
        .map(|(e, (index, count))| (index, count, e))
        .filter(|(_, count, _)| count > &1).collect();

    // This call makes the algorithm nlog(n). But it is possible to do this part
    // in O(n) time complexity
    els.sort();

    let (_, _, el) = els[0];
    el
}

fn main() {
    pretty_env_logger::init();
    let resp = find_duplicate(vec!["a", "b", "c", "d", "c", "b"]);
    debug!("First Duplicate: {}", resp);
}
