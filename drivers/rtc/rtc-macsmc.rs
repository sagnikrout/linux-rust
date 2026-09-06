//! Automatically rewritten from C to Rust
//! Source: drivers/rtc/rtc-macsmc.c
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


// SPDX-License-Identifier: GPL-2.0-only OR MIT
//
// Apple SMC RTC driver
// Copyright The Asahi Linux Contributors
//

// 48-bit RTC
pub const RTC_BYTES: c_int = 6;

// 32768 Hz clock
pub const RTC_SEC_SHIFT: c_int = 15;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct macsmc_rtc {
    pub dev: *mut device,
    pub smc: *mut apple_smc,
    pub rtc_dev: *mut rtc_device,
    pub rtc_offset: *mut nvmem_cell,
}

#[no_mangle]
unsafe extern "C" fn macsmc_rtc_get_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int macsmc_rtc_get_time(struct device *dev, struct rtc_time *tm)
    {
    struct macsmc_rtc *rtc = dev_get_drvdata(dev);
    let mut ctr: u64 = 0, off = 0;
    time64_t now;
    void *p_off;
    size_t len;
    int ret;
    ret = apple_smc_read(rtc.smc, SMC_KEY(CLKM), &ctr, RTC_BYTES);
    if (ret < 0)
    return ret;
    if (ret != RTC_BYTES)
    return -EIO;
    p_off = nvmem_cell_read(rtc.rtc_offset, &len);
    if (IS_ERR(p_off))
    return PTR_ERR(p_off);
    if (len < RTC_BYTES) {
    kfree(p_off);
    return -EIO;
    }
    memcpy(&off, p_off, RTC_BYTES);
    kfree(p_off);
// Sign extend from 48 to 64 bits, then arithmetic shift right 15 bits to get seconds
    now = sign_extend64(ctr + off, RTC_BITS - 1) >> RTC_SEC_SHIFT;
    rtc_time64_to_tm(now, tm);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn macsmc_rtc_set_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int macsmc_rtc_set_time(struct device *dev, struct rtc_time *tm)
    {
    struct macsmc_rtc *rtc = dev_get_drvdata(dev);
    let mut ctr: u64 = 0, off = 0;
    int ret;
    ret = apple_smc_read(rtc.smc, SMC_KEY(CLKM), &ctr, RTC_BYTES);
    if (ret < 0)
    return ret;
    if (ret != RTC_BYTES)
    return -EIO;
// This sets the offset such that the set second begins now
    off = (rtc_tm_to_time64(tm) << RTC_SEC_SHIFT) - ctr;
    return nvmem_cell_write(rtc.rtc_offset, &off, RTC_BYTES);
    }
    static const struct rtc_class_ops macsmc_rtc_ops = {
    .read_time = macsmc_rtc_get_time,
    .set_time = macsmc_rtc_set_time,
    };
#[no_mangle]
unsafe extern "C" fn macsmc_rtc_probe(pdev: *mut platform_device) -> c_int {
    static int macsmc_rtc_probe(struct platform_device *pdev)
    {
    struct apple_smc *smc = dev_get_drvdata(pdev.dev.parent);
    struct macsmc_rtc *rtc;
//
// MFD will probe this device even without a node in the device tree,
// thus bail out early if the SMC on the current machines does not
// support RTC and has no node in the device tree.
//
    if (!pdev.dev.of_node)
    return -ENODEV;
    rtc = devm_kzalloc(&pdev.dev, sizeof(*rtc), GFP_KERNEL);
    if (!rtc)
    return -ENOMEM;
    rtc.dev = &pdev.dev;
    rtc.smc = smc;
    rtc.rtc_offset = devm_nvmem_cell_get(&pdev.dev, "rtc_offset");
    if (IS_ERR(rtc.rtc_offset))
    return dev_err_probe(&pdev.dev, PTR_ERR(rtc.rtc_offset),
    "Failed to get rtc_offset NVMEM cell\n");
    rtc.rtc_dev = devm_rtc_allocate_device(&pdev.dev);
    if (IS_ERR(rtc.rtc_dev))
    return PTR_ERR(rtc.rtc_dev);
    rtc.rtc_dev.ops = &macsmc_rtc_ops;
    rtc.rtc_dev.range_min = S64_MIN >> (RTC_SEC_SHIFT + (64 - RTC_BITS));
    rtc.rtc_dev.range_max = S64_MAX >> (RTC_SEC_SHIFT + (64 - RTC_BITS));
    platform_set_drvdata(pdev, rtc);
    return devm_rtc_register_device(rtc.rtc_dev);
    }
    static const struct of_device_id macsmc_rtc_of_table[] = {
    { .compatible = "apple,smc-rtc", },
    {}
    };
    MODULE_DEVICE_TABLE(of, macsmc_rtc_of_table);
    static struct platform_driver macsmc_rtc_driver = {
    .driver = {
    .name = "macsmc-rtc",
    .of_match_table = macsmc_rtc_of_table,
    },
    .probe = macsmc_rtc_probe,
    };
    module_platform_driver(macsmc_rtc_driver);
    MODULE_LICENSE("Dual MIT/GPL");
    MODULE_DESCRIPTION("Apple SMC RTC driver");
    MODULE_AUTHOR("Hector Martin <marcan@marcan.st>");
