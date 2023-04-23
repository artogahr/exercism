// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration {
    seconds: f64,
}

macro_rules! sec_to_years {
    ($secs:expr ) => {
        fn years_during(d: &Duration) -> f64 {
            d.seconds / $secs
        }
    };
}
impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration { seconds: s as f64 }
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64 {
        unimplemented!(
            "convert a duration ({d:?}) to the number of years on this planet for that duration"
        );
    }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    sec_to_years!(7_600_387.7512);
}
impl Planet for Venus {
    sec_to_years!(19_413_750.404);
}
impl Planet for Earth {
    sec_to_years!(31_556_952.00);
}
impl Planet for Mars {
    sec_to_years!(59_352_813.921);
}
impl Planet for Jupiter {
    sec_to_years!(374_347_972.149);
}
impl Planet for Saturn {
    sec_to_years!(929_273_280.906);
}
impl Planet for Uranus {
    sec_to_years!(2_651_315_576.413);
}
impl Planet for Neptune {
    sec_to_years!(5_200_311_775.256);
}
