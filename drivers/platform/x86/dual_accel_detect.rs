//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/platform/x86/dual_accel_detect.h
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
// Helper code to detect 360 degree hinges (yoga) style 2-in-1 devices using 2 accelerometers
// to allow the OS to determine the angle between the display and the base of the device.
//
// On Windows these are read by a special HingeAngleService process which calls undocumented
// ACPI methods, to let the firmware know if the 2-in-1 is in tablet- or laptop-mode.
// The firmware may use this to disable the kbd and touchpad to avoid spurious input in
// tablet-mode as well as to report SW_TABLET_MODE info to the OS.
//
// Since Linux does not call these undocumented methods, the SW_TABLET_MODE info reported
// by various drivers/platform/x86 drivers is incorrect. These drivers use the detection
// code in this file to disable SW_TABLET_MODE reporting to avoid reporting broken info
// (instead userspace can derive the status itself by directly reading the 2 accels).
//

// Systems which use a pair of accels with KIOX010A / KIOX020A ACPI ids
// Systems which use a single DUAL250E ACPI device to model 2 accels
// Systems which use a single BOSC0200 ACPI device to model 2 accels
