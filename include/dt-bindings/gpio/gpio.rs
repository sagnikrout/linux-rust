//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/gpio/gpio.h
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


// SPDX-License-Identifier: (GPL-2.0 OR BSD-2-Clause)
//
// This header provides constants for most GPIO bindings.
//
// Most GPIO bindings include a flags cell as part of the GPIO specifier.
// In most cases, the format of the flags cell uses the standard values
// defined in this header.
//
// Bit 0 express polarity
pub const GPIO_ACTIVE_HIGH: c_int = 0;
pub const GPIO_ACTIVE_LOW: c_int = 1;
// Bit 1 express single-endedness
pub const GPIO_PUSH_PULL: c_int = 0;
pub const GPIO_SINGLE_ENDED: c_int = 2;
// Bit 2 express Open drain or open source
pub const GPIO_LINE_OPEN_SOURCE: c_int = 0;
pub const GPIO_LINE_OPEN_DRAIN: c_int = 4;
//
// Open Drain/Collector is the combination of single-ended open drain interface.
// Open Source/Emitter is the combination of single-ended open source interface.
//

// Bit 3 express GPIO suspend/resume and reset persistence
pub const GPIO_PERSISTENT: c_int = 0;
pub const GPIO_TRANSITORY: c_int = 8;
// Bit 4 express pull up
pub const GPIO_PULL_UP: c_int = 16;
// Bit 5 express pull down
pub const GPIO_PULL_DOWN: c_int = 32;
// Bit 6 express pull disable
pub const GPIO_PULL_DISABLE: c_int = 64;
