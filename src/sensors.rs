// gets the height according to the BMP (over sea level)
pub fn get_height_bmp_m() -> Option<f64> {
    return Some(134.0); // meters
}

// gets GPS latitude and longitude
pub fn get_gps_lat_and_lon() -> Option<[f64; 2]> {
    return Some([55.73562, 34.16498]);
}

// gets temperature from DHT11 (or 22)
pub fn get_dht_temp_c() -> Option<f32> {
    return Some(16.3);
}

// gets humidity from DHT
pub fn get_dht_humidity_percent() -> Option<f32> {
    return Some(63.2);
}

// gets orientation for gyroscope
pub fn get_gyro_orientation_p_y_r() -> Option<[f64; 3]> {
    return Some([4.32, 2.43, 84.6]); // pitch, yaw, roll
}
