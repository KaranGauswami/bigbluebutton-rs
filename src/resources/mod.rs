#[cfg(feature = "administration")]
/// Conference administration apis
pub mod administration;

#[cfg(feature = "monitoring")]
/// Conference monitoring apis
pub mod monitoring;

/// Webhook apis
#[cfg(feature = "webhook")]
pub mod webhook;

/// Recordings apis
#[cfg(feature = "recording")]
pub mod recording;
