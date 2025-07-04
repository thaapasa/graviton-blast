use bevy::prelude::Component;

/// Mass, in kg.
/// Determines how much gravity affects this object.
#[derive(Component, Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Mass(f32);

impl Mass {
    #[inline]
    pub fn value(&self) -> f32 {
        self.0
    }
    pub const fn tons(tons: f32) -> Self {
        Mass(tons * 1_000.0)
    }
    pub const fn kg(kilos: f32) -> Self {
        Mass(kilos)
    }
}
