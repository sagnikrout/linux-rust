//! Automatically rewritten from C to Rust
//! Source: tools/testing/cxl/config_check.c
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

#[no_mangle]
pub unsafe extern "C" fn check() {
    void check(void)
    {
//
// These kconfig symbols must be set to "m" for cxl_test to load
// and operate.
//
    BUILD_BUG_ON(!IS_ENABLED(CONFIG_64BIT));
    BUILD_BUG_ON(!IS_MODULE(CONFIG_CXL_BUS));
    BUILD_BUG_ON(!IS_MODULE(CONFIG_CXL_ACPI));
    BUILD_BUG_ON(!IS_MODULE(CONFIG_CXL_PMEM));
    BUILD_BUG_ON(!IS_ENABLED(CONFIG_CXL_REGION_INVALIDATION_TEST));
    BUILD_BUG_ON(!IS_ENABLED(CONFIG_NVDIMM_SECURITY_TEST));
    BUILD_BUG_ON(!IS_ENABLED(CONFIG_DEBUG_FS));
    BUILD_BUG_ON(!IS_ENABLED(CONFIG_MEMORY_HOTPLUG));
    }
