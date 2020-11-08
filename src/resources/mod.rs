#[cfg(feature = "administration")]
/// Conference administration apis
pub mod administration;

#[cfg(feature = "monitoring")]
/// Conference monitoring apis
pub mod monitoring;

/// Webhook apis
///
/// This requires ```webhook``` feature to be enabled
#[cfg(feature = "webhook")]
pub mod webhook;

/// Recordings apis
///
/// This requires ```recording``` feature to be enabled
#[cfg(feature = "recording")]
pub mod recording;
