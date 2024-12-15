
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Location {
    pub longitude: f64,
    pub latitude: f64,
}

impl Location {
    pub fn from_city_loc(loc: maxminddb::geoip2::city::Location) -> Option<Self> {
        match (loc.longitude, loc.latitude) {
            (Some(longitude), Some(latitude)) => Some(Self { longitude, latitude }),
            _ => None,
        }
    }
}
