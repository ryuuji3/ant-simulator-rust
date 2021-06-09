use std::ops;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Rotation(f32);

impl Rotation {
    pub fn new(rotation: f32) -> Rotation {
        Rotation(rotation)
    }

    pub fn radians(&self) -> f32 {
        self.0.to_radians()
    }

    pub fn degrees(&self) -> f32 {
        self.0 // stored in degrees
    }
}

// Always make sure degrees is a value between [0.0, 360.0]
fn normalize_degrees(degrees: f32) -> f32 {
    degrees - 360.0 * (degrees * (1.0 / 360.0)).floor()
}

impl ops::Add<Rotation> for Rotation {
    type Output = Rotation;

    fn add(self, angle: Rotation) -> Rotation {
        let angle = normalize_degrees(self.0 + angle.0);

        Rotation(angle)
    }
}

impl ops::Sub<Rotation> for Rotation {
    type Output = Rotation;

    fn sub(self, angle: Rotation) -> Rotation {
        let angle = normalize_degrees(self.0 - angle.0);

        Rotation(angle)
    }
}

impl ops::Neg for Rotation {
    type Output = Rotation;

    fn neg(self) -> Rotation {
        let angle = normalize_degrees(-self.0);

        Rotation(angle)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_rotation() {
        let zero_degrees = Rotation(0.0);
        let ninety_degrees = Rotation(90.0);
        let two_hundred_seventy = Rotation(270.0);
        let three_hundred_sixty = Rotation(360.0);

        assert_eq!(zero_degrees + ninety_degrees, Rotation(90.0));
        assert_eq!(ninety_degrees + ninety_degrees, Rotation(180.0));
        // wrap-around
        assert_eq!(two_hundred_seventy + ninety_degrees, Rotation(0.0));
        assert_eq!(three_hundred_sixty + ninety_degrees, Rotation(90.0));
    }

    #[test]
    fn sub_rotation() {
        let zero_degrees = Rotation(0.0);
        let ninety_degrees = Rotation(90.0);
        let two_hundred_seventy = Rotation(270.0);
        let three_hundred_sixty = Rotation(360.0);

        assert_eq!(zero_degrees - ninety_degrees, Rotation(270.0)); // wrap-around
        assert_eq!(ninety_degrees - ninety_degrees, Rotation(0.0));
        assert_eq!(two_hundred_seventy - ninety_degrees, Rotation(180.0));
        assert_eq!(three_hundred_sixty - ninety_degrees, Rotation(270.0));
    }

    #[test]
    fn neg_rotation() {
        let zero_degrees = Rotation(0.0);
        let ninety_degrees = Rotation(90.0);
        let two_hundred_seventy = Rotation(270.0);
        let three_hundred_sixty = Rotation(360.0);

        assert_eq!(-zero_degrees, Rotation(0.0));
        assert_eq!(-ninety_degrees, Rotation(270.0));
        assert_eq!(-two_hundred_seventy, Rotation(90.0));
        assert_eq!(-three_hundred_sixty, Rotation(0.0));
    }
}