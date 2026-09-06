//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/user_events/user_events_selftests.h
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

// message = "";
// fail = false;
// umount = false;
// Ensure tracefs is installed
// message = "Tracefs is not installed";
// Ensure mounted tracefs
// message = "Cannot mount tracefs";
// fail = true;
// umount = true;
// message = "Cannot access tracefs";
// fail = true;
// message = "";
// fail = false;
// umount = false;
// message = "Must be run as root";
// fail = true;
// Ensure user_events is installed
// message = "user_events is not installed";
// message = "Cannot access user_events_data";
// fail = true;

