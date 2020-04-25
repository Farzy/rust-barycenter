mod values;
use crate::values::get_values;
mod body;
use crate::body::merge_all_bodies_iter;

fn main() {
    let vals = get_values();
    let barycenter = merge_all_bodies_iter(&vals);

    println!("Bacycenter: {:?}", barycenter);
}
