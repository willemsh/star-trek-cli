use rand::prelude::*;

/*
 *	Returns an integer from 0 to spread
 */
pub fn get_rand(spread: i32) -> i32 {
    let y: f64 = rand::thread_rng().gen(); // generates a float between 0 and 1

    let r: i32 = (y * spread as f64) as i32;
    r + 1
}

/*
 *	Get a random co-ordinate
 */
pub fn rand8() -> i32 {
    get_rand(8)
}
/*
 *	Get a random co-ordinate
 */
pub fn get_rand8() -> i32 {
    let y: f64 = rand::thread_rng().gen(); // generates a float between 0 and 1

    (y * 8.0) as i32
}

/*
 *	Get a random co-ordinate
 */
pub fn get_randf32() -> f32 {
    rand::thread_rng().gen::<f32>()
}
