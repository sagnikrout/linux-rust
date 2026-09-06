//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/ps3/time.c
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
// PS3 time and rtc routines.
//
// Copyright (C) 2006 Sony Computer Entertainment Inc.
// Copyright 2006 Sony Corp.
//

#[no_mangle]
pub unsafe extern "C" fn ps3_calibrate_decr() -> void __init {
    void __init ps3_calibrate_decr(void)
    {
    int result;
    u64 tmp;
    result = ps3_repository_read_be_tb_freq(0, &tmp);
    BUG_ON(result);
    ppc_tb_freq = tmp;
    ppc_proc_freq = ppc_tb_freq * 40;
    }
#[no_mangle]
unsafe extern "C" fn read_rtc() -> u64 {
    static u64 read_rtc(void)
    {
    int result;
    u64 rtc_val;
    u64 tb_val;
    result = lv1_get_rtc(&rtc_val, &tb_val);
    BUG_ON(result);
    return rtc_val;
    }
#[no_mangle]
pub unsafe extern "C" fn ps3_get_boot_time() -> time64_t __init {
    time64_t __init ps3_get_boot_time(void)
    {
    return read_rtc() + ps3_os_area_get_rtc_diff();
    }
#[no_mangle]
unsafe extern "C" fn ps3_rtc_init() -> int __init {
    static int __init ps3_rtc_init(void)
    {
    struct platform_device *pdev;
    if (!firmware_has_feature(FW_FEATURE_PS3_LV1))
    return -ENODEV;
    pdev = platform_device_register_simple("rtc-ps3", -1, core::ptr::null_mut(), 0);
    return PTR_ERR_OR_ZERO(pdev);
    }
    device_initcall(ps3_rtc_init);
