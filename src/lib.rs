/// Wall clock time abstraction.
#[derive(Debug)]
pub struct Clock {
    time: time::Time,
}

impl Clock {
    pub fn new() -> Self {
        let now_local = time::OffsetDateTime::now_local().unwrap();
        Self {
            time: now_local.time(),
        }
    }

    #[cfg(test)]
    pub fn new_test(time: time::Time) -> Self {
        Self { time }
    }

    fn hour_indicator_degree(&self) -> f32 {
        let hour_12 = (self.time.hour() % 12) as f32;
        normalize_clock_indicator_degree(hour_12, 12)
    }

    fn minute_indicator_degree(&self) -> f32 {
        normalize_clock_indicator_degree(self.time.minute() as f32, 60)
    }

    fn second_indicator_degree(&self) -> f32 {
        normalize_clock_indicator_degree(self.time.second() as f32, 60)
    }

    /// Calculates the indicator end coordinates of the hour indicator on a
    /// 12-hour wall clock.
    pub fn hour_coordinates(
        &self,
        x_orig: usize,
        y_orig: usize,
        scale_factor: usize,
    ) -> (usize /* x */, usize /* y */) {
        calc_destination_coordinates(self.hour_indicator_degree(), x_orig, y_orig, scale_factor)
    }

    /// Calculates the indicator end coordinates of the minute indicator on a
    /// 12-hour wall clock.
    pub fn minute_coordinates(
        &self,
        x_orig: usize,
        y_orig: usize,
        scale_factor: usize,
    ) -> (usize /* x */, usize /* y */) {
        calc_destination_coordinates(self.minute_indicator_degree(), x_orig, y_orig, scale_factor)
    }

    /// Calculates the indicator end coordinates of the second indicator on a
    /// 12-hour wall clock.
    pub fn second_coordinates(
        &self,
        x_orig: usize,
        y_orig: usize,
        scale_factor: usize,
    ) -> (usize /* x */, usize /* y */) {
        calc_destination_coordinates(self.second_indicator_degree(), x_orig, y_orig, scale_factor)
    }
}

impl Default for Clock {
    fn default() -> Self {
        Clock::new()
    }
}

/// Calculates the relative angle between the "midnight" line and the current
/// position of the clock indicator.
#[inline]
fn calc_clock_indicator_degree(indicator_value: f32, indicator_scale: u8) -> f32 {
    assert!(!indicator_value.is_nan());
    assert!(!indicator_value.is_infinite());
    assert!(indicator_value.is_sign_positive());
    let scale = indicator_scale as f32;
    assert!(indicator_value <= scale);
    let value = if indicator_value == scale {
        // Wrap 360° to 0°
        0.0
    } else {
        indicator_value
    };
    let fraction = value / scale;
    fraction * 360.0
}

/// Wrapper around [`calc_clock_indicator_degree`] that maps the degree to
/// the corresponding degree in the sin/cos angle world. See sin/cos
/// functions for reference.
///
/// 0 -> 90
/// 90 -> 0
/// 180 -> 270
/// 270 -> 180
#[inline]
fn normalize_clock_indicator_degree(indicator_value: f32, indicator_scale: u8) -> f32 {
    let degree = calc_clock_indicator_degree(indicator_value, indicator_scale);

    // Invert direction, as wall clock grows clock-wise while angles grows
    // counter-clock wise.
    let degree = 360.0 - degree;

    const OFFSET: f32 = 90.0;

    (degree + OFFSET) % 360.0
}

/// Calculates the coordinates of the end point / graph destination in a
/// (x,y)-coordinate system where x and y grow to the right respectively to
/// the bottom.
#[inline]
fn calc_destination_coordinates(
    degree: f32,
    x_orig: usize,
    y_orig: usize,
    scale_factor: usize,
) -> (usize /* x */, usize /* y */) {
    let (sin /* y */, cos /* x */) = degree.to_radians().sin_cos();
    (
        (x_orig as f32 + (cos * scale_factor as f32)) as usize,
        (y_orig as f32 - (sin * scale_factor as f32)) as usize,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clock_indicator_to_angle_deg() {
        assert_eq!(calc_clock_indicator_degree(0.0, 60), 0.0);
        assert_eq!(calc_clock_indicator_degree(15.0, 60), 90.0);
        assert_eq!(calc_clock_indicator_degree(30.0, 60), 180.0);
        assert_eq!(calc_clock_indicator_degree(59.9, 60), 359.4);
        assert_eq!(calc_clock_indicator_degree(60.0, 60), 0.0);

        assert_eq!(calc_clock_indicator_degree(0.0, 12), 0.0);
        assert_eq!(calc_clock_indicator_degree(3.0, 12), 90.0);
        assert_eq!(calc_clock_indicator_degree(6.0, 12), 180.0);
        assert_eq!(calc_clock_indicator_degree(9.0, 12), 270.0);
        assert_eq!(calc_clock_indicator_degree(12.0, 12), 0.0);
    }

    #[test]
    fn test_clock_indicator_to_normalized_angle_degree() {
        assert_eq!(normalize_clock_indicator_degree(0.0, 60), 90.0);
        assert_eq!(normalize_clock_indicator_degree(15.0, 60), 0.0);
        assert_eq!(normalize_clock_indicator_degree(30.0, 60), 270.0);
        assert_eq!(normalize_clock_indicator_degree(59.9, 60), 90.600006);
        assert_eq!(normalize_clock_indicator_degree(60.0, 60), 90.0);

        assert_eq!(normalize_clock_indicator_degree(0.0, 12), 90.0);
        assert_eq!(normalize_clock_indicator_degree(3.0, 12), 0.0);
        assert_eq!(normalize_clock_indicator_degree(6.0, 12), 270.0);
        assert_eq!(normalize_clock_indicator_degree(9.0, 12), 180.0);
        assert_eq!(normalize_clock_indicator_degree(12.0, 12), 90.0);
    }

    #[test]
    fn test_calc_destination_coordinates() {
        assert_eq!(calc_destination_coordinates(0.0, 10, 10, 1), (11, 10));
        assert_eq!(calc_destination_coordinates(90.0, 10, 10, 1), (10, 9));
        assert_eq!(calc_destination_coordinates(180.0, 10, 10, 1), (9, 10));
        assert_eq!(calc_destination_coordinates(270.0, 10, 10, 1), (10, 11));
        assert_eq!(calc_destination_coordinates(360.0, 10, 10, 1), (11, 10));

        assert_eq!(calc_destination_coordinates(0.0, 10, 10, 2), (12, 10));
        assert_eq!(calc_destination_coordinates(90.0, 10, 10, 2), (10, 8));
        assert_eq!(calc_destination_coordinates(180.0, 10, 10, 2), (8, 10));
        assert_eq!(calc_destination_coordinates(270.0, 10, 10, 2), (10, 12));
        assert_eq!(calc_destination_coordinates(360.0, 10, 10, 2), (12, 10));
    }
}
