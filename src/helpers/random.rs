use actix_web::web::Bytes;
use rand::{thread_rng, Rng};


pub fn get_unsigned(frame: &Bytes) -> u32 {
    let start = thread_rng().gen_range(0..frame.len() - 4);

    ((frame[start] as u32) << 24)
        | ((frame[start + 1] as u32) << 16)
        | ((frame[start + 2] as u32) << 8)
        | (frame[start + 3] as u32)
}

pub fn get_bool(frame: &Bytes) -> bool {
    let start = thread_rng().gen_range(0..frame.len());

    frame[start] & 1 != 0
}
