//! Automatically rewritten from C to Rust
//! Source: sound/soc/codecs/adau7118-hw.c
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
// Analog Devices ADAU7118 8 channel PDM-to-I2S/TDM Converter Standalone Hw
// driver
//
// Copyright 2019 Analog Devices Inc.

#[no_mangle]
unsafe extern "C" fn adau7118_probe_hw(pdev: *mut platform_device) -> c_int {
    static int adau7118_probe_hw(struct platform_device *pdev)
    {
    return adau7118_probe(&pdev.dev, core::ptr::null_mut(), true);
    }
    static const struct of_device_id adau7118_of_match[] = {
    { .compatible = "adi,adau7118" },
    {}
    };
    MODULE_DEVICE_TABLE(of, adau7118_of_match);
    static const struct platform_device_id adau7118_id[] = {
    { .name = "adau7118" },
    { }
    };
    MODULE_DEVICE_TABLE(platform, adau7118_id);
    static struct platform_driver adau7118_driver_hw = {
    .driver = {
    .name = "adau7118",
    .of_match_table = adau7118_of_match,
    },
    .probe = adau7118_probe_hw,
    .id_table = adau7118_id,
    };
    module_platform_driver(adau7118_driver_hw);
    MODULE_AUTHOR("Nuno Sa <nuno.sa@analog.com>");
    MODULE_DESCRIPTION("ADAU7118 8 channel PDM-to-I2S/TDM Converter driver for standalone hw mode");
    MODULE_LICENSE("GPL");
