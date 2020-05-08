const MINUTES: f64 = 60.0;
const HOURS: f64 = 60.0;
const DAYS: f64 = 24.0;
const YEARS: f64 = 365.25;

#[derive(Debug)]
pub struct Duration {
    seconds: u64,
}

impl From<u64> for Duration {
    fn from(seconds: u64) -> Self {
        Self { seconds }
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64 {
        d.seconds as f64 / MINUTES / HOURS / DAYS / (YEARS * Self::factor())
    }

    fn factor() -> f64;
}

macro_rules! planet {
    ($name:ident, $factor: expr) => {
        pub struct $name;

        impl Planet for $name {
            fn factor() -> f64 {
                $factor
            }
        }
    };
}

planet!(Mercury, 0.240_846_7);
planet!(Venus, 0.615_197_26);
planet!(Earth, 1.0);
planet!(Mars, 1.880_815_8);
planet!(Jupiter, 11.862_615);
planet!(Saturn, 29.447_498);
planet!(Uranus, 84.016_846);
planet!(Neptune, 164.79132);
