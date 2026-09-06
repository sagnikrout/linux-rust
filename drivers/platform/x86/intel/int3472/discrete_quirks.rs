//! Automatically rewritten from C to Rust
//! Source: drivers/platform/x86/intel/int3472/discrete_quirks.c
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
// Author: Hans de Goede <hansg@kernel.org>

    static const struct int3472_discrete_quirks lenovo_miix_510_quirks = {
    .avdd_second_sensor = "i2c-OVTI2680:00",
    };
    const struct dmi_system_id skl_int3472_discrete_quirks[] = {
    {
// Lenovo Miix 510-12IKB
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "LENOVO"),
    DMI_MATCH(DMI_PRODUCT_VERSION, "MIIX 510-12IKB"),
    },
    .driver_data = (void *)&lenovo_miix_510_quirks,
    },
    { }
    };
