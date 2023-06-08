use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

pub fn generate_room_code() -> String {
    let rng = thread_rng();
    let code: String = rng
        .sample_iter(&Alphanumeric)
        .take(6)
        .map(char::from)
        .collect();
    code
}
