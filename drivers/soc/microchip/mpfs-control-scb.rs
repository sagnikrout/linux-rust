//! Automatically rewritten from C to Rust
//! Source: drivers/soc/microchip/mpfs-control-scb.c
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

    static const struct mfd_cell mpfs_control_scb_devs[] = {
    MFD_CELL_NAME("mpfs-tvs"),
    };
#[no_mangle]
unsafe extern "C" fn mpfs_control_scb_probe(pdev: *mut platform_device) -> c_int {
    static int mpfs_control_scb_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    return devm_mfd_add_devices(dev, PLATFORM_DEVID_NONE,
    mpfs_control_scb_devs,
    ARRAY_SIZE(mpfs_control_scb_devs), core::ptr::null_mut(), 0,
    core::ptr::null_mut());
    }
    static const struct of_device_id mpfs_control_scb_of_match[] = {
    { .compatible = "microchip,mpfs-control-scb", },
    {},
    };
    MODULE_DEVICE_TABLE(of, mpfs_control_scb_of_match);
    static struct platform_driver mpfs_control_scb_driver = {
    .driver = {
    .name = "mpfs-control-scb",
    .of_match_table = mpfs_control_scb_of_match,
    },
    .probe = mpfs_control_scb_probe,
    };
    module_platform_driver(mpfs_control_scb_driver);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Conor Dooley <conor.dooley@microchip.com>");
    MODULE_DESCRIPTION("PolarFire SoC control scb driver");
