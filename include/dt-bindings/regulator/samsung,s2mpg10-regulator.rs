//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/regulator/samsung,s2mpg10-regulator.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
//
// Copyright 2021 Google LLC
// Copyright 2025 Linaro Ltd.
//
// Device Tree binding constants for the Samsung S2MPG1x PMIC regulators
//
// Several regulators may be controlled via external signals instead of via
// software. These constants describe the possible signals for such regulators
// and generally correspond to the respecitve on-chip pins.
//
// S2MPG10 regulators supporting these are:
// - buck1m .. buck7m buck10m
// - ldo3m .. ldo19m
//
// ldo20m supports external control, but using a different set of control
// signals.
//
// S2MPG11 regulators supporting these are:
// - buck1s .. buck3s buck5s buck8s buck9s bucka buckd
// - ldo1s ldo2s ldo8s ldo13s
//

