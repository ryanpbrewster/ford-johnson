use std::cmp::Ordering;
use std::collections::HashMap;
use std::ops::Range;

fn main() {
    let items: Vec<String> = DEFAULT_ITEMS
        .split_ascii_whitespace()
        .map(String::from)
        .collect();

    println!("sorting {:?}", items);
    let mut known = Vec::new();
    let order = loop {
        let status = try_sort(0..items.len(), &known);
        match status.first_unknown {
            None => break status.ordered,
            Some(pair) => {
                println!("~{} comparisons left", status.num_unknown);
                known.push((pair, prompt_user(&items[pair.0], &items[pair.1])));
            }
        }
    };
    println!("DONE! needed {} comparisons", known.len());
    for i in 0..items.len() {
        println!("{}) {}", i + 1, items[order[i]]);
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
    ordered: Vec<usize>,
}

fn try_sort(items: Range<usize>, known: &[(Pair, Ordering)]) -> SortStatus {
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
    let mut ordered: Vec<usize> = items.collect();
    ford_johnson::sort(&mut ordered, &mut cmp);
    SortStatus {
        first_unknown,
        num_unknown,
        ordered,
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
