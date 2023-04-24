pub(crate) mod wgs84 {
    pub(crate) const EARTH_RADIUS_IN_KILOMETERS: f64 = 6378.137;
    pub(crate) const SEMI_MINOR_AXIS_IN_KILOMETERS: f64 = 6356.752314245;
    pub static INVERSE_FLATTENING_FACTOR: f64 = 1. / 298.257223563;
}

pub(crate) mod nad27 {
    pub(crate) const EARTH_RADIUS_IN_KILOMETERS: f64 = 6378.2064;
    pub(crate) const SEMI_MINOR_AXIS_IN_KILOMETERS: f64 = 6356.5838;
    pub static INVERSE_FLATTENING_FACTOR: f64 = 1. / 294.9786982;
}

pub(crate) mod nad83 {
    pub(crate) const EARTH_RADIUS_IN_KILOMETERS: f64 = 6378.137;
    pub(crate) const SEMI_MINOR_AXIS_IN_KILOMETERS: f64 = 6356.752314140347;
    pub static INVERSE_FLATTENING_FACTOR: f64 = 1. / 298.257222101;
}
