use ordered_float::OrderedFloat;
use serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Coordinates {
    latitude: OrderedFloat<f64>,
    longitude: OrderedFloat<f64>,
}

impl Coordinates {
    pub fn new(latitude: f64, longitude: f64) -> Self {
        Coordinates {
            latitude: OrderedFloat(latitude),
            longitude: OrderedFloat(longitude),
        }
    }
    pub fn get_latitude(&self) -> f64 {
        self.latitude.into_inner()
    }
    pub fn get_longitude(&self) -> f64 {
        self.longitude.into_inner()
    }
}

impl Serialize for Coordinates {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("coordinates", 2)?;
        state.serialize_field("latitude", &self.latitude.into_inner())?;
        state.serialize_field("longitude", &self.longitude.into_inner())?;
        state.end()
    }
}
