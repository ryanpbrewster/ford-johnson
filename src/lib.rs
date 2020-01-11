use std::cmp::Ordering;
use std::collections::HashMap;

pub fn merge_insertion_sort<F>(xs: &mut [usize], cmp: &mut F)
where
    F: FnMut(usize, usize) -> Ordering + Sized,
{
    if xs.len() < 2 {
        return;
    }

    // First, swap all the largest elements to the front.
    let mut partner = HashMap::new();
    let half = xs.len() / 2;
    for i in 0..half {
        if cmp(xs[i], xs[i + half]) == Ordering::Less {
            xs.swap(i, i + half);
        }
        partner.insert(xs[i], xs[i + half]);
    }

    // Now recursively sort those larger elements.
    merge_insertion_sort(&mut xs[..half], cmp);

    // Now do an insertion-sort to get the latter half of the array into order.
    for i in 0..half {
        // Every step of the way we'll be inserting an extra element,
        // so `x[i]` will be located at `xs[2*i]`.
        let y = partner[&xs[2 * i]];
        // We known that y[i] < x[i], so we need to insert it to the left of x[i].
        let idx = find_insert_point(y, &xs[..2 * i], cmp);
        // Make room.
        xs[idx..half + i + 1].rotate_right(1);
        // Insert it.
        xs[idx] = y;
    }
    if xs.len() % 2 > 0 {
        let i = xs.len() - 1;
        let idx = find_insert_point(xs[i], &xs[..i], cmp);
        xs[idx..].rotate_right(1);
    }
}

fn find_insert_point<F>(x: usize, xs: &[usize], cmp: &mut F) -> usize
where
    F: FnMut(usize, usize) -> Ordering + Sized,
{
    let mut lo = 0;
    let mut hi = xs.len();
    while hi > lo {
        let mid = lo + (hi - lo) / 2;
        match cmp(x, xs[mid]) {
            Ordering::Equal => return mid,
            Ordering::Less => hi = mid,
            Ordering::Greater => lo = mid + 1,
        };
    }
    lo
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::seq::SliceRandom;
    use rand::SeedableRng;

    #[test]
    fn sorts_correctly_smoke() {
        let mut xs = vec![3, 5, 1, 2, 4];
        merge_insertion_sort(&mut xs, &mut |a, b| a.cmp(&b));
        assert_eq!(xs, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn sorts_correctly() {
        let init: Vec<usize> = (0..100).collect();
        let mut prng = rand_pcg::Pcg32::seed_from_u64(42);
        for _ in 0..1000 {
            let mut xs = init.clone();
            xs.shuffle(&mut prng);
            merge_insertion_sort(&mut xs, &mut |a, b| a.cmp(&b));
            assert_eq!(xs, init);
        }
    }

    #[test]
    fn manual() {
        let mut xs: Vec<usize> = (0..8).collect();
        merge_insertion_sort(&mut xs, &mut |a: usize, b: usize| {
            println!("cmp {} vs {}", a, b);
            a.cmp(&b)
        });
    }

    fn count_cmps(mut xs: Vec<usize>) -> usize {
        let mut cnt = 0;
        merge_insertion_sort(&mut xs, &mut |a: usize, b: usize| {
            cnt += 1;
            a.cmp(&b)
        });
        cnt
    }

    #[test]
    fn right_number_of_comparisons_smoke() {
        assert_eq!(count_cmps(vec![3, 5, 1, 2, 4]), 7);
    }

    #[test]
    fn right_number_of_comparisons_eois() {
        // From the Online Encyclopedia of Integer Sequences: https://oeis.org/A001768
        let expected = vec![
            0, 1, 3, 5, 7, 10, 13, 16, 19, 22, 26, 30, 34, 38, 42, 46, 50, 54, 58, 62, 66, 71, 76,
            81, 86, 91, 96, 101, 106, 111, 116, 121, 126, 131, 136, 141, 146, 151, 156, 161, 166,
            171, 177, 183, 189, 195, 201, 207, 213, 219, 225, 231, 237, 243, 249, 255,
        ];
        for (i, n) in expected.into_iter().enumerate() {
            let a = count_cmps((0..i + 1).collect());
            assert!(
                a <= n,
                "{} items can be sorted in {} cmps but we used {}",
                i + 1,
                n,
                a
            );
        }
    }

    #[test]
    fn right_number_of_comparisons_big() {
        let mut xs: Vec<usize> = (0..100).collect();
        let mut prng = rand_pcg::Pcg32::seed_from_u64(999);
        xs.shuffle(&mut prng);
        assert_eq!(count_cmps(xs), 530);
    }
}
