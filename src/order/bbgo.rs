use rand::{thread_rng, Rng};

pub fn handle_order(
    base: &String,
    quote: &String,
    order_type: &String,
    price: &String,
    quantity: &String,
) -> i32 {
    let mut rng = thread_rng();

    return rng.gen_range(0..1000000);
}
