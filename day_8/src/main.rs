#![feature(dbg_macro)]

fn main() {
    let numbers: Vec<usize> = include_str!("input.txt")
        .trim()
        .split(' ')
        .map(|s| s.parse().unwrap())
        .collect();

    println!("part_1: {}", part_1(Box::new(numbers.as_slice())));
    println!("part_2: {}", part_2(Box::new(numbers.as_slice())));
}

fn part_1(numbers: Box<&[usize]>) -> usize {
    tree_meta_sum_and_width(numbers).0
}

fn tree_meta_sum_and_width(numbers: Box<&[usize]>) -> (usize, usize) {
    let n_nodes = numbers[0];
    let n_metas = numbers[1];

    // The tree is 2 headers wide, plus all children widths
    let mut tree_width: usize = 2;

    // The tree's sum is 0, plus all children sums
    let mut tree_sum: usize = 0;

    // Recurse through each child, adding its sum and width.  The offset is increased as tree_width
    // grows
    for _ in 0..n_nodes {
        let (sum, width) = tree_meta_sum_and_width(Box::new(&numbers[tree_width..]));
        tree_width += width;
        tree_sum += sum;
    }

    // Add this node's number of metas to its width
    tree_width += n_metas;

    // Add this node's meta sum to its sum
    tree_sum += numbers[(tree_width - n_metas)..tree_width].iter().sum::<usize>();

    (tree_sum, tree_width)
}

fn part_2(numbers: Box<&[usize]>) -> usize {
    tree_meta_ref_sum_and_width(numbers).0
}

fn tree_meta_ref_sum_and_width(numbers: Box<&[usize]>) -> (usize, usize) {
    let n_nodes = numbers[0];
    let n_metas = numbers[1];

    // The tree is 2 headers wide, plus all children widths
    let mut tree_width: usize = 2;

    // The tree's sum is 0, plus all children sums
    let mut tree_sum: usize = 0;

    // Recurse through each child, adding its width and recording its sum (for later filtering)
    let mut child_meta_ref_sums = vec![0];
    for _ in 0..n_nodes {
        let (sum, width) = tree_meta_ref_sum_and_width(Box::new(&numbers[tree_width..]));
        tree_width += width;
        child_meta_ref_sums.push(sum);
    }

    // Add this node's number of metas to its width
    tree_width += n_metas;

    // If no children: sum up this node's meta numbers
    // If children: sum up the valid references to other nodes
    if n_nodes == 0 {
        tree_sum += numbers[(tree_width - n_metas)..tree_width].iter().sum::<usize>();
    } else {
        for meta_idx in numbers[(tree_width - n_metas)..tree_width].iter() {
            if let Some(valid_meta_ref) = child_meta_ref_sums.get(*meta_idx) {
                tree_sum += valid_meta_ref;
            }
        }
    }

    (tree_sum, tree_width)
}

/*
2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2
A----------------------------------
    B----------- C-----------
                     D-----
*/

#[cfg(test)]
mod tests {
    use super::*;

    fn numbers() -> Vec<usize> {
        "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2"
            .split(' ')
            .map(|s| s.parse().unwrap())
            .collect()
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(Box::new(numbers().as_slice())), 138);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(Box::new(numbers().as_slice())), 66);
    }
}
