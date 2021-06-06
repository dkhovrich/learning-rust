use std::panic::resume_unwind;

pub fn arr_data_main() {
    let ar = [3, 4, 1, 4, 9, 8];

    let r = get_min_max_ar(&ar);

    match r {
        Some(r) => println!("Array. Min: {:?}, Max: {:?}", r.0, r.1),
        None => println!("None!"),
    }

    if let Some(res) = get_min_max_ar(&ar) {
        println!("Array. Min: {:?}, Max: {:?}", res.0, res.1);
    }

    let mut v = ar.to_vec();

    let res = get_min_max_vector(&v);

    match res {
        Some(res) => println!("Vec. Min: {:?}, Max: {:?}", res.0, res.1),
        None => println!("Vec is empty!"),
    }
}

fn get_min_max_ar(ar: &[i32; 6]) -> Option<(&i32, &i32)> {
    if ar.is_empty() {
        return None;
    }

    let mut minIndex = 0;
    let mut maxIndex = 0;

    for (index, item) in ar.iter().enumerate() {
        if item < &ar[minIndex] {
            minIndex = index;
        }

        if item > &ar[maxIndex] {
            maxIndex = index;
        }
    }

    return Some((&ar[minIndex], &ar[maxIndex]));
}

fn get_min_max_vector(v: &Vec<i32>) -> Option<(&i32, &i32)> {
    if v.is_empty() {
        return None;
    }

    let mut minIndex = 0;
    let mut maxIndex = 0;

    for (index, item) in v.iter().enumerate() {
        if item < &v[minIndex] {
            minIndex = index;
        }

        if item > &v[maxIndex] {
            maxIndex = index;
        }
    }

    let min = &v[minIndex];
    let max = &v[maxIndex];

    return Some((min, max));
}
