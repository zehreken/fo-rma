fn linear_to_srgb(c: vec3<f32>) -> vec3<f32> {
    let gamma = 2.2;
    let gamma_corrected_color = vec3<f32>(
        pow(c.x, 1.0 / gamma),
        pow(c.y, 1.0 / gamma),
        pow(c.z, 1.0 / gamma)
    );
    return gamma_corrected_color;
}

fn srgb_to_linear(c: vec3<f32>) -> vec3<f32> {
    let gamma = 2.2;
    let expanded_color = vec3<f32>(
        pow(c.x, gamma),
        pow(c.y, gamma),
        pow(c.z, gamma)
    );
    return expanded_color;
}
