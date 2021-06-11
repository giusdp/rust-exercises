fn main() {
    let mut a = vec![
        4, 1, 3, 2, 5, 2, 9, 1, 2, 4, 5, 14, 349, 12, 42352, 1, 9, 0, -1, -2,
    ];
    let h = a.len();
    let sorted = mergesort(&mut a, 0, h);
    a.sort();
    if a == sorted {
        println!("{:?}", a == sorted);
    } else {
        println!("Sorted is {:?}", sorted);
    }
}

fn mergesort(a: &mut Vec<i32>, low: usize, high: usize) -> Vec<i32> {
    let length = high - low;
    if length == 0 {
        vec![]
    } else if length == 1 {
        vec![a[low]]
    } else {
        let middle = length / 2; // floor
        let left = mergesort(a, low, low + middle);
        let right = mergesort(a, low + middle, high);
        merge(left, right)
    }
}

fn merge(mut left: Vec<i32>, mut right: Vec<i32>) -> Vec<i32> {
    let mut sorted: Vec<i32> = Vec::new();

    while left.len() > 0 || right.len() > 0 {
        let left_min = left.iter().min().cloned();
        let right_min = right.iter().min().cloned();

        match (left_min, right_min) {
            (Some(l), Some(r)) => {
                if l <= r {
                    move_to(&mut sorted, &mut left, l);
                } else {
                    move_to(&mut sorted, &mut right, r);
                }
            }
            (Some(l), None) => {
                move_to(&mut sorted, &mut left, l);
            }
            (None, Some(r)) => {
                move_to(&mut sorted, &mut right, r);
            }
            (None, None) => break,
        };
    }

    sorted
}

fn move_to(sorted: &mut Vec<i32>, from: &mut Vec<i32>, el: i32) {
    if let Some(index) = from.iter_mut().position(|value| *value == el) {
        sorted.push(from.swap_remove(index));
    }
}
