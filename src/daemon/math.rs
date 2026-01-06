/// Linear interpolation for Fan Curve
/// Points are (Temperature, FanSpeed%)
/// Example: [(30, 0), (50, 30), (70, 80), (80, 100)]
pub fn calculate_target_speed(current_temp: u32, curve: &[(u32, u32)]) -> u32 {
    if curve.is_empty() {
        return 0;
    }

    // 1. Sort points by temp (x-axis) just to be safe
    let mut sorted_curve = curve.to_vec();
    sorted_curve.sort_by_key(|k| k.0);

    // 2. Handle below min
    if current_temp <= sorted_curve.first().unwrap().0 {
        return sorted_curve.first().unwrap().1;
    }

    // 3. Handle above max
    if current_temp >= sorted_curve.last().unwrap().0 {
        return sorted_curve.last().unwrap().1;
    }

    // 4. Find the range [p1, p2] where p1.temp <= current_temp < p2.temp
    for window in sorted_curve.windows(2) {
        let p1 = window[0];
        let p2 = window[1];

        if current_temp >= p1.0 && current_temp < p2.0 {
            // Interpolate
            // y = y1 + (x - x1) * (y2 - y1) / (x2 - x1)
            let x_range = p2.0 - p1.0;
            let y_range = p2.1 as i32 - p1.1 as i32; // Use i32 to handle negative slopes? (Unlikely for fans but good practice)
            let x_diff = current_temp - p1.0;

            let interpolated = p1.1 as i32 + (x_diff as i32 * y_range / x_range as i32);
            return interpolated.clamp(0, 100) as u32;
        }
    }

    0 // Should be unreachable
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interpolation() {
        let curve = vec![(30, 0), (50, 40), (80, 100)];
        
        // Exact points
        assert_eq!(calculate_target_speed(30, &curve), 0);
        assert_eq!(calculate_target_speed(50, &curve), 40);
        assert_eq!(calculate_target_speed(80, &curve), 100);
        
        // Below min
        assert_eq!(calculate_target_speed(20, &curve), 0);
        
        // Above max
        assert_eq!(calculate_target_speed(90, &curve), 100);
        
        // Midpoint (Linear)
        // Between 30 (0%) and 50 (40%) at temp 40:
        // Range X=20, Y=40. Ratio = 2.
        // Diff = 10. Result = 0 + 10*2 = 20.
        assert_eq!(calculate_target_speed(40, &curve), 20);
        
        // Between 50 (40%) and 80 (100%) at temp 65:
        // Range X=30, Y=60. Ratio = 2.
        // Diff = 15. Result = 40 + 15*2 = 70.
        assert_eq!(calculate_target_speed(65, &curve), 70);
    }
}
