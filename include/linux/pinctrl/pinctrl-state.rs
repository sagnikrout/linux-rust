//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/pinctrl/pinctrl-state.h
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
//
// Standard pin control state definitions
//
// @PINCTRL_STATE_DEFAULT: the state the pinctrl handle shall be put
// into as default, usually this means the pins are up and ready to
// be used by the device driver. This state is commonly used by
// hogs to configure muxing and pins at boot, and also as a state
// to go into when returning from sleep and idle in
// .pm_runtime_resume() or ordinary .resume() for example.
// @PINCTRL_STATE_INIT: normally the pinctrl will be set to "default"
// before the driver's probe() function is called.  There are some
// drivers where that is not appropriate becausing doing so would
// glitch the pins.  In those cases you can add an "init" pinctrl
// which is the state of the pins before drive probe.  After probe
// if the pins are still in "init" state they'll be moved to
// "default".
// @PINCTRL_STATE_IDLE: the state the pinctrl handle shall be put into
// when the pins are idle. This is a state where the system is relaxed
// but not fully sleeping - some power may be on but clocks gated for
// example. Could typically be set from a pm_runtime_suspend() or
// pm_runtime_idle() operation.
// @PINCTRL_STATE_SLEEP: the state the pinctrl handle shall be put into
// when the pins are sleeping. This is a state where the system is in
// its lowest sleep state. Could typically be set from an
// ordinary .suspend() function.
//

