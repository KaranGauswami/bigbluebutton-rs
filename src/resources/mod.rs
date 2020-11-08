#[cfg(feature = "administration")]
/// Conference administration apis
pub mod administration;

#[cfg(feature = "monitoring")]
/// Conference monitoring apis
pub mod monitoring;

#[cfg(feature = "webhook")]
/// Webhook apis
pub mod webhook;

#[cfg(feature = "recording")]
/// Recordings apis
pub mod recording;
