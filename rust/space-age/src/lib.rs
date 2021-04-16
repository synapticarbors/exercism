const EARTH_SECONDS_PER_YEAR: f64 = 31557600f64;

#[derive(Debug)]
pub struct Duration {
    pub seconds: f64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration { seconds: s as f64 }
    }
}

pub trait Planet {
    const ORBITAL_PERIOD: f64;

    fn years_during(d: &Duration) -> f64 {
        d.seconds / EARTH_SECONDS_PER_YEAR / Self::ORBITAL_PERIOD
    }
}

macro_rules! planet {
    ( $name:ident, $period:expr ) => {
        pub struct $name;

        impl Planet for $name {
            const ORBITAL_PERIOD: f64 = $period;
        }
    };
}

planet!(Earth, 1.0);
planet!(Mercury, 0.2408467);
planet!(Venus, 0.61519726);
planet!(Mars, 1.8808158);
planet!(Jupiter, 11.862615);
planet!(Saturn, 29.447498);
planet!(Uranus, 84.016846);
planet!(Neptune, 164.79132);
