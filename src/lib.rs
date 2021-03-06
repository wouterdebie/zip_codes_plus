//! A lookup table for all primary U.S. ZIP codes.
//!
//! # Examples
//!
//! ```
//! assert_eq!(zip_codes_plus::map().get("10465").unwrap().state, "NY");
//! assert_eq!(zip_codes_plus::map().get("53186").unwrap().city, "Waukesha");
//! assert_eq!(
//! zip_codes_plus::by_city("Westwood, NJ").unwrap()
//! .iter()
//! .map(|r| r.zip_code)
//! .collect::<Vec<&str>>(),
//! vec!["07675"]
//! );
//! assert_eq!(
//!   zip_codes_plus::by_city("Whippany, NJ").unwrap()
//!     .iter()
//!     .map(|r| r.zip_code)
//!     .collect::<Vec<&str>>(),
//!   vec!["07983", "07999", "07981"]
//! );
//! assert!(
//!    zip_codes_plus::by_city("Nowhere, LA").is_none()
//! );
//! ```

pub struct Record {
    /// The 5-digit ZIP code.
    pub zip_code: &'static str,
    /// The ZIP code classification.
    pub zip_code_type: Type,
    /// The city to which the ZIP code belongs (all uppercase).
    pub city: &'static str,
    /// The state to which the ZIP code belongs (two letter abbreviation).
    pub state: &'static str,
    /// Latitude and longitude
    pub coordinates: Option<(f64, f64)>,
    /// If `true`, the ZIP code is historical; if `false`, the ZIP code is current.
    pub is_decommissioned: bool,
    /// The number of individual tax returns filed as of 2008.
    pub tax_returns_filed: Option<u64>,
    /// The estimated population of the area of the ZIP code.
    pub estimated_population: Option<u64>,
    /// Total yearly wages of the population.
    pub total_wages: Option<u64>,
}

/// The classification of a ZIP code.
#[derive(Clone, Debug)]
pub enum Type {
    /// Typical ZIP code (does not fall under any other type).
    Standard,
    /// Used only for PO Boxes at a given facility, not for any other type of delivery.
    PoBox,
    /// Assigned to a single high-volume address.
    Unique,
    /// Used to route mail for the U.S. military.
    Military,
}

include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

pub type Map = phf::Map<&'static str, Record>;

/// Returns a reference to a static lookup table for all primary U.S. ZIP codes.
#[inline]
pub fn map() -> &'static Map {
    &ZIP_CODES
}

/// Returns a `Vec<&Record>` based on a "City, State"
#[inline]
pub fn by_city(city: &str) -> Option<Vec<&Record>> {
    Some(
        CITY_MAP
            .get(city)?
            .iter()
            .map(|x| ZIP_CODES.get(*x).unwrap())
            .collect(),
    )
}
