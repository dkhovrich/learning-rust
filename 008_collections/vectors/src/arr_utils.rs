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

    v.clear();

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

    let mut min_index = 0;
    let mut max_index = 0;

    for (index, item) in ar.iter().enumerate() {
        if item < &ar[min_index] {
            min_index = index;
        }

        if item > &ar[max_index] {
            max_index = index;
        }
    }

    return Some((&ar[min_index], &ar[max_index]));
}

fn get_min_max_vector(v: &Vec<i32>) -> Option<(&i32, &i32)> {
    if v.is_empty() {
        return None;
    }

    let mut min_index = 0;
    let mut max_index = 0;

    for (index, item) in v.iter().enumerate() {
        if item < &v[min_index] {
            min_index = index;
        }

        if item > &v[max_index] {
            max_index = index;
        }
    }

    let min = &v[min_index];
    let max = &v[max_index];

    return Some((min, max));
}
