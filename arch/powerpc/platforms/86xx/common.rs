//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/86xx/common.c
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
// Routines common to most mpc86xx-based boards.
//

    static const struct of_device_id mpc86xx_common_ids[] __initconst = {
    { .type = "soc", },
    { .compatible = "soc", },
    { .compatible = "simple-bus", },
    { .name = "localbus", },
    { .compatible = "gianfar", },
    { .compatible = "fsl,mpc8641-pcie", },
    {},
    };
#[no_mangle]
pub unsafe extern "C" fn mpc86xx_common_publish_devices() -> int __init {
    int __init mpc86xx_common_publish_devices(void)
    {
    return of_platform_bus_probe(core::ptr::null_mut(), mpc86xx_common_ids, core::ptr::null_mut());
    }
#[no_mangle]
pub unsafe extern "C" fn mpc86xx_time_init() -> long __init {
    long __init mpc86xx_time_init(void)
    {
    unsigned int temp;
// Set the time base to zero
    mtspr(SPRN_TBWL, 0);
    mtspr(SPRN_TBWU, 0);
    temp = mfspr(SPRN_HID0);
    temp |= HID0_TBEN;
    mtspr(SPRN_HID0, temp);
    isync();
    return 0;
    }
