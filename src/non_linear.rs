pub fn relu(pixel: f32) -> f32 {
    if pixel>=0.0 {
        pixel*0.1

    } else {
        0.0
    }
}

pub fn sigmoid(pixel: f32) -> f32 {
    1.0/(1.0 + (-pixel).exp())
}

