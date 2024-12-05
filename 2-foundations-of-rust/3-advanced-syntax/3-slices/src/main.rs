// This a unfinished implementation of the well-known merge sort algorithm
//
// 1. Fix the language problems in the function merge
//
// 2. Finish the implementation of the function merge_sort
//
// 3. EXTRA: try changing the type from i32 into String everywhere; does your program still compile? What changes are necessary?

// Extra solution (using generics)
// We can't merge two String slices since String doesn't implement Copy, thus we can't easily copy it to a new vector.
// We could use Clone instead of Copy, but it would be terribly inefficient.

fn merge_generic<T: Ord + Copy>(a: &[T], b: &[T]) -> Vec<T> {
    let mut dest = Vec::new();

    let mut iter_a = a.iter().peekable();
    let mut iter_b = b.iter().peekable();

    while let (Some(_a), Some(_b)) = (iter_a.peek(), iter_b.peek()) {
        let a = **_a;
        let b = **_b;

        if a <= b {
            dest.push(a);
            iter_a.next();
        } else {
            dest.push(b);
            iter_b.next();
        }
    }

    dest.extend(iter_a);
    dest.extend(iter_b);

    dest
}

/// Merge two array slices (that have to be sorted) into a vector
fn merge(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut dest = Vec::new();

    let mut iter_a = a.iter().peekable();
    let mut iter_b = b.iter().peekable();

    while let (Some(_a), Some(_b)) = (iter_a.peek(), iter_b.peek()) {
        let a = **_a;
        let b = **_b;

        if a <= b {
            dest.push(a);
            iter_a.next();
        } else {
            dest.push(b);
            iter_b.next();
        }
    }

    dest.extend(iter_a);
    dest.extend(iter_b);

    dest
}

/// Take an array slice, and sort into a freshly constructed vector using the above function
fn merge_sort(data: &[i32]) -> Vec<i32> {
    if data.len() > 1 {
        let mid = data.len() / 2;
        let left = merge_sort(&data[..mid]);
        let right = merge_sort(&data[mid..]);

        merge(&left, &right)
    } else {
        data.to_vec()
    }
}

/// Read a bunch of numbers from standard input into a Vec<String>.
fn read_numbers() -> Vec<i32> {
    use std::io;
    let mut result = Vec::new();
    for line in io::stdin().lines().flatten() {
        for word in line.split_whitespace() {
            result.push(word.parse().unwrap())
        }
    }

    result
}

fn main() {
    let input = read_numbers();
    println!("Data to be sorted:");
    println!("{input:?}");

    let sorted_input = merge_sort(&input);
    println!("Sorted data:");
    println!("{sorted_input:?}");
}

// you can run these automatic tests by typing 'cargo test'
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sort() {
        assert_eq!(merge_sort(&[]), vec![]);
        assert_eq!(merge_sort(&[5]), vec![5]);
        assert_eq!(merge_sort(&[1, 2, 3]), vec![1, 2, 3]);
        assert_eq!(merge_sort(&[47, 42, 5, 1]), vec![1, 5, 42, 47]);
        assert_eq!(
            merge_sort(&[6, 47, 42, 5, 1, 123]),
            vec![1, 5, 6, 42, 47, 123]
        );
    }
}
