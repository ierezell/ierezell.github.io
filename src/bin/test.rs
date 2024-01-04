use std::collections::{BTreeMap, HashMap};
fn main() {
    let mut trees: HashMap<String, BTreeMap<i64, i32>> = HashMap::from_iter([
        ("coucou".to_string(), BTreeMap::<i64, i32>::new()),
        ("lol".to_string(), BTreeMap::<i64, i32>::new()),
    ]);

    for i in 0..5 {
        let tree = trees.get_mut("coucou").expect("No name");
        let _ = *tree.entry(i).and_modify(|curr| *curr += 1).or_insert(1);

        let tree_2 = trees.get_mut("lol").expect("No name");
        let _ = *tree_2.entry(i).and_modify(|curr| *curr += 2).or_insert(2);
    }

    for i in 10..50 {
        let tree = trees.get_mut("coucou").expect("No name");
        let _ = *tree.entry(i).and_modify(|curr| *curr += 10).or_insert(10);

        let tree_2 = trees.get_mut("lol").expect("No name");
        let _ = *tree_2.entry(i).and_modify(|curr| *curr += 20).or_insert(20);
    }

    for i in 50..100 {
        let tree = trees.get_mut("coucou").expect("No name");
        let _ = *tree.entry(i).and_modify(|curr| *curr += 30).or_insert(30);

        let tree_2 = trees.get_mut("lol").expect("No name");
        let _ = *tree_2.entry(i).and_modify(|curr| *curr += 60).or_insert(60);
    }

    let span = 100;
    let span_min = 0;
    let num_buckets = 10;

    let range = span + (span % num_buckets);

    let bucket_size = (range / num_buckets).max(1);

    for (key, value) in trees["coucou"].iter() {
        println!("{} {}", key, value)
    }

    for idx in 0..num_buckets {
        let start = span_min + idx * bucket_size;
        let end = span_min + (idx + 1) * bucket_size;
        for name in ["coucou", "lol"] {
            println!(
                "{} {} {} {}",
                name,
                start,
                end,
                trees[name].range(start..end).fold(0, |acc, x| acc + x.1)
            )
        }
    }
}
