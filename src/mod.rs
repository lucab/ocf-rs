#![cfg_attr(feature = "unstable", plugin(serde_macros))]
#![cfg_attr(feature = "unstable", feature(plugin, custom_derive, custom_attribute))]
#![doc(html_root_url="https://arcnmx.github.io/ocf-rs")]

#[cfg(feature = "unstable")]
include!("lib.rs");

#[cfg(feature = "syntex")]
include!(concat!(env!("OUT_DIR"), "/lib.rs"));

#[cfg(not(any(feature = "unstable", feature = "syntex")))]
error_expected_unstable_or_stable_feature!();
