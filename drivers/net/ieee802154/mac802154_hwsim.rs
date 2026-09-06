//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ieee802154/mac802154_hwsim.h
#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use core::ffi::*;

// --- Linux Kernel Primitives Prelude ---
pub type uid_t = u32;
pub type gid_t = u32;
pub type uid16_t = u16;
pub type gid16_t = u16;
pub type pid_t = i32;
pub type mode_t = u32;
pub type umode_t = u16;
pub type nlink_t = u32;
pub type off_t = i64;
pub type loff_t = i64;
pub type dev_t = u32;
pub type ino_t = u64;
pub type size_t = usize;
pub type ssize_t = isize;
pub type uintptr_t = usize;
pub type intptr_t = isize;
pub type ptrdiff_t = isize;
pub type clockid_t = i32;
pub type timer_t = i32;
pub type time64_t = i64;
pub type atomic_t = core::sync::atomic::AtomicI32;
pub type atomic64_t = core::sync::atomic::AtomicI64;
// ---------------------------------------


// mac802154 hwsim netlink commands
//
// @MAC802154_HWSIM_CMD_UNSPEC: unspecified command to catch error
// @MAC802154_HWSIM_CMD_GET_RADIO: fetch information about existing radios
// @MAC802154_HWSIM_CMD_SET_RADIO: change radio parameters during runtime
// @MAC802154_HWSIM_CMD_NEW_RADIO: create a new radio with the given parameters
// returns the radio ID (>= 0) or negative on errors, if successful
// then multicast the result
// @MAC802154_HWSIM_CMD_DEL_RADIO: destroy a radio, reply is multicasted
// @MAC802154_HWSIM_CMD_GET_EDGE: fetch information about existing edges
// @MAC802154_HWSIM_CMD_SET_EDGE: change edge parameters during runtime
// @MAC802154_HWSIM_CMD_DEL_EDGE: delete edges between radios
// @MAC802154_HWSIM_CMD_NEW_EDGE: create a new edge between two radios
// @__MAC802154_HWSIM_CMD_MAX: enum limit
//

// mac802154 hwsim netlink attributes
//
// @MAC802154_HWSIM_ATTR_UNSPEC: unspecified attribute to catch error
// @MAC802154_HWSIM_ATTR_RADIO_ID: u32 attribute to identify the radio
// @MAC802154_HWSIM_ATTR_EDGE: nested attribute of edges
// @MAC802154_HWSIM_ATTR_EDGES: list if nested attributes which contains the
// edge information according the radio id
// @__MAC802154_HWSIM_ATTR_MAX: enum limit
//

// mac802154 hwsim edge netlink attributes
//
// @MAC802154_HWSIM_EDGE_ATTR_UNSPEC: unspecified attribute to catch error
// @MAC802154_HWSIM_EDGE_ATTR_ENDPOINT_ID: radio id where the edge points to
// @MAC802154_HWSIM_EDGE_ATTR_LQI: LQI value which the endpoint radio will
// receive for this edge
// @__MAC802154_HWSIM_ATTR_MAX: enum limit
//

