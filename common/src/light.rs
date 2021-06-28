const LINEAR1_END: u8 = 127;
const LINEAR1_END_WEIGHT: f32 = 32.0;
const LINEAR2_END_WEIGHT: f32 = 4096.0;

/// [0, 255] -> [0.0, 64.0]
pub fn intensity_to_weight(intensity: u8) -> f32 {
    if intensity <= LINEAR1_END {
        intensity as f32 / LINEAR1_END as f32 * LINEAR1_END_WEIGHT
    } else {
        ((intensity - LINEAR1_END) as f32 / (255 - LINEAR1_END) as f32)
            * (LINEAR2_END_WEIGHT - LINEAR1_END_WEIGHT)
            + LINEAR1_END_WEIGHT
    }
}
/// [0.0, 64.0] -> [0, 255]
pub fn weight_to_intensity(weight: f32) -> u8 {
    if weight >= LINEAR1_END_WEIGHT {
        ((weight - LINEAR1_END_WEIGHT) / (LINEAR2_END_WEIGHT - LINEAR1_END_WEIGHT)
            * ((255 - LINEAR1_END) as f32)) as u8
    } else {
        (weight / LINEAR1_END_WEIGHT * LINEAR1_END as f32).round() as u8
    }
}

#[test]
fn check_intensity_to_weight() {
    for i in 0..=255 {
        println!("{}", intensity_to_weight(i));
    }
}
