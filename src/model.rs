#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Location {
    pub longitude: f64,
    pub latitude: f64,
}

impl Location {
    pub fn from_city_loc(loc: maxminddb::geoip2::city::Location) -> Option<Self> {
        match (loc.longitude, loc.latitude) {
            (Some(longitude), Some(latitude)) => Some(Self {
                longitude,
                latitude,
            }),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LookupEntry {
    pub ip_str: String,
    pub loc: Location,
}

impl LookupEntry {
    pub fn from_city(ip: &str, city: maxminddb::geoip2::City) -> Option<Self> {
        if let Some(loc) = city.location {
            let loc = Location::from_city_loc(loc)?;

            return Some(Self {
                ip_str: ip.to_string(),
                loc,
            });
        }

        None
    }
}
