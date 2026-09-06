//! Automatically rewritten from C to Rust
//! Source: drivers/rtc/nvmem.c
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
// RTC subsystem, nvmem interface
//
// Copyright (C) 2017 Alexandre Belloni
//

    int devm_rtc_nvmem_register(struct rtc_device *rtc,
    struct nvmem_config *nvmem_config)
    {
    struct device *dev = rtc.dev.parent;
    struct nvmem_device *nvmem;
    if (!nvmem_config)
    return -ENODEV;
    nvmem_config.dev = dev;
    nvmem_config.owner = rtc.owner;
    nvmem_config.add_legacy_fixed_of_cells = true;
    nvmem = devm_nvmem_register(dev, nvmem_config);
    if (IS_ERR(nvmem))
    dev_err(dev, "failed to register nvmem device for RTC\n");
    return PTR_ERR_OR_ZERO(nvmem);
    }
    EXPORT_SYMBOL_GPL(devm_rtc_nvmem_register);
