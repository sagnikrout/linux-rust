//! Automatically rewritten from C Header to Rust Module
//! Source: net/mac802154/driver-ops.h
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


// SPDX-License-Identifier: GPL-2.0

// setup receive mode parameters e.g. address mode
// TODO: Requires a different receive mode setup e.g.
// at86rf233 hardware.
//
// In practice other filtering levels can be requested, but as
// for now most hardware/drivers only support
// IEEE802154_FILTERING_NONE, we fallback to this actual
// filtering level in hardware and make our own additional
// filtering in mac802154 receive path.
//
// TODO: Move this logic to the device drivers as hardware may
// support more higher level filters. Hardware may also require
// a different order how register are set, which could currently
// be buggy, so all received parameters need to be moved to the
// start() callback and let the driver go into the mode before
// it will turn on receive handling.
//
// Do not error out if IEEE802154_HW_PROMISCUOUS because we
// expect the hardware to operate at the level
// IEEE802154_FILTERING_4_FRAME_FIELDS anyway.
//
// sync away all work on the tasklet before clearing started
