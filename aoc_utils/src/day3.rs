pub fn object_priority(object: &u8) -> i32 {
    let val: i32 = *object as i32;
    // object is between A..Z
    if val > 64 && val < 91 {
        val - 38
    } else {
        val - 96
    }
}
