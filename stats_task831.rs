use ::std::collections::HashMap;

fn main() {
    let input = vec![1, 2, 3];
    dbg!(stats(input.clone()));
}

fn stats(mut s: Vec<i32>) {
    s.sort();

    let indexr = s.len() / 2;
    let median = if s.len() % 2 == 0 {
        let indexl = indexr - 1;
        (s[indexl] + s[indexr]) / 2
    } else {
        s[indexr]
    };

    let mut mode = 0;
    let mut biggest = 0;
    let mut map = HashMap::new();
    for e in s {
        let count = map.entry(e).or_insert(0);
        *count += 1;
    }
    for (k, v) in &map {
        if biggest < *v {
            biggest = *v;
            mode = *k;
        }
    }

    if biggest == 1 {
        println!("All elements appear just once.");
    } else {
        println!("mode: {}", mode);
    }
    println!("median: {}", median);
    println!("Hashmap is still valid: {:?}", map);
}
