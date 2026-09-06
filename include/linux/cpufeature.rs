//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/cpufeature.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (C) 2014 Linaro Ltd. <ard.biesheuvel@linaro.org>
//

//
// Macros imported from <asm/cpufeature.h>:
// - cpu_feature(x)		ordinal value of feature called 'x'
// - cpu_have_feature(u32 n)	whether feature #n is available
// - MAX_CPU_FEATURES		upper bound for feature ordinal values
// Optional:
// - CPU_FEATURE_TYPEFMT	format string fragment for printing the cpu type
// - CPU_FEATURE_TYPEVAL	set of values matching the format string above
//

//
// Use module_cpu_feature_match(feature, module_init_function) to
// declare that
// a) the module shall be probed upon discovery of CPU feature 'feature'
// (typically at boot time using udev)
// b) the module must not be loaded if CPU feature 'feature' is not present
// (not even by manual insmod).
//
// For a list of legal values for 'feature', please consult the file
// 'asm/cpufeature.h' of your favorite architecture.
//

