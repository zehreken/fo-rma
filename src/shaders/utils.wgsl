fn gamma_correction(c: vec3<f32>) -> vec3<f32> {
    let gamma = 2.2;
    let gamma_corrected_color = vec3<f32>(
        pow(c.x, 1.0 / gamma),
        pow(c.y, 1.0 / gamma),
        pow(c.z, 1.0 / gamma)
    );
    return gamma_corrected_color;
}

fn inverse_gamma_correction(c: vec3<f32>) -> vec3<f32> {
    let gamma = 2.2;
    let expanded_color = vec3<f32>(
        pow(c.x, gamma),
        pow(c.y, gamma),
        pow(c.z, gamma)
    );
    return expanded_color;
}

// Convert from Linear to sRGB (Gamma correction)
fn linear_to_srgb(c: vec3<f32>) -> vec3<f32> {
    return pow(c, vec3<f32>(1.0 / 2.2));  // Linear to sRGB (gamma correction)
}