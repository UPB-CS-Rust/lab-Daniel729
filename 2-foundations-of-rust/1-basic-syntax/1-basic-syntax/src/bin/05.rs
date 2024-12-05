fn main() {
    let mut data = [22, 12, 13, 17, 18];
    for el in data.iter_mut() {
        *el = floored_half(*el);
    }

    println!("{:?}", data);
}

fn floored_half(data: i32) -> i32 {
    data / 2
}
