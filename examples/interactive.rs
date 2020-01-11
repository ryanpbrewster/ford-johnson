use std::cmp::Ordering;
use std::collections::HashMap;

fn main() {
    let mut items: Vec<String> = DEFAULT_ITEMS
        .split_ascii_whitespace()
        .map(String::from)
        .collect();

    println!("sorting {:?}", items);
    let mut known = Vec::new();
    loop {
        let status = try_sort(&mut items, &known);
        match status.first_unknown {
            None => break,
            Some(pair) => {
                println!("~{} comparisons left", status.num_unknown);
                known.push((pair, prompt_user(&items[pair.0], &items[pair.1])));
            }
        }
    }
    println!("DONE! needed {} comparisons", known.len());
    for (i, item) in items.into_iter().enumerate() {
        println!("{}) {}", i + 1, item);
    }
}

const DEFAULT_ITEMS: &'static str = "red blue green yellow white";

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Pair(usize, usize);
impl Pair {
    fn reverse(&self) -> Pair {
        Pair(self.1, self.0)
    }
}
struct SortStatus {
    first_unknown: Option<Pair>,
    num_unknown: usize,
}

fn try_sort(items: &mut [String], known: &[(Pair, Ordering)]) -> SortStatus {
    let index = {
        let mut builder = HashMap::new();
        for &(pair, ord) in known {
            builder.insert(pair, ord);
            builder.insert(pair.reverse(), ord.reverse());
        }
        builder
    };
    let mut first_unknown = None;
    let mut num_unknown = 0;
    let mut cmp = |i, j| {
        index.get(&Pair(i, j)).cloned().unwrap_or_else(|| {
            num_unknown += 1;
            if first_unknown.is_none() {
                first_unknown = Some(Pair(i, j));
            }
            Ordering::Less
        })
    };
    let mut xs: Vec<usize> = (0..items.len()).collect();
    ford_johnson::merge_insertion_sort(&mut xs, &mut cmp);
    SortStatus {
        first_unknown,
        num_unknown,
    }
}

fn prompt_user(a: &str, b: &str) -> Ordering {
    loop {
        println!("(a) {}", a);
        println!("(b) {}", b);
        let mut buf = String::new();
        std::io::stdin()
            .read_line(&mut buf)
            .expect("read from stdin");
        match buf.trim() {
            "a" | "A" => return Ordering::Less,
            "b" | "B" => return Ordering::Greater,
            _ => println!("pick a or b"),
        };
    }
}
