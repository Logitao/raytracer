pub fn nearly_equal(a: f32, b: f32) -> bool {
    let abs_a = a.abs();
    let abs_b = b.abs();
    let diff = (a - b).abs();

    if a == b {
        // Handle infinities.
        true
    } else if a == 0.0 || b == 0.0 || diff < f32::MIN_POSITIVE {
        // One of a or b is zero (or both are extremely close to it,) use absolute error.
        diff < (f32::EPSILON * f32::MIN_POSITIVE)
    } else {
        // Use relative error.
        (diff / f32::min(abs_a + abs_b, f32::MAX)) < f32::EPSILON
    }
}

pub fn clamp<T: PartialOrd>(value: T, min: T, max: T) -> T {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

pub fn min<T: PartialOrd>(value: T, min: T) -> T {
    if value <= min {
        min
    } else {
        value
    }
}

pub fn max<T: PartialOrd>(value: T, max: T) -> T {
    if value >= max {
        max
    } else {
        value
    }
}
