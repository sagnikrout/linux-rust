//! Automatically rewritten from C to Rust
//! Source: drivers/usb/chipidea/ulpi.c
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
// Copyright (c) 2016 Linaro Ltd.
//

#[no_mangle]
unsafe extern "C" fn ci_ulpi_wait(ci: *mut ci_hdrc, mask: u32) -> c_int {
    static int ci_ulpi_wait(struct ci_hdrc *ci, u32 mask)
    {
    let mut usec: c_ulong = 10000;
    while (usec--) {
    if (!hw_read(ci, OP_ULPI_VIEWPORT, mask))
    return 0;
    udelay(1);
    }
    return -ETIMEDOUT;
    }
#[no_mangle]
unsafe extern "C" fn ci_ulpi_read(dev: *mut device, addr: u8) -> c_int {
    static int ci_ulpi_read(struct device *dev, u8 addr)
    {
    struct ci_hdrc *ci = dev_get_drvdata(dev);
    int ret;
    hw_write(ci, OP_ULPI_VIEWPORT, 0xffffffff, ULPI_WRITE | ULPI_WAKEUP);
    ret = ci_ulpi_wait(ci, ULPI_WAKEUP);
    if (ret)
    return ret;
    hw_write(ci, OP_ULPI_VIEWPORT, 0xffffffff, ULPI_RUN | ULPI_ADDR(addr));
    ret = ci_ulpi_wait(ci, ULPI_RUN);
    if (ret)
    return ret;
    return hw_read(ci, OP_ULPI_VIEWPORT, GENMASK(15, 8)) >> 8;
    }
#[no_mangle]
unsafe extern "C" fn ci_ulpi_write(dev: *mut device, addr: u8, val: u8) -> c_int {
    static int ci_ulpi_write(struct device *dev, u8 addr, u8 val)
    {
    struct ci_hdrc *ci = dev_get_drvdata(dev);
    int ret;
    hw_write(ci, OP_ULPI_VIEWPORT, 0xffffffff, ULPI_WRITE | ULPI_WAKEUP);
    ret = ci_ulpi_wait(ci, ULPI_WAKEUP);
    if (ret)
    return ret;
    hw_write(ci, OP_ULPI_VIEWPORT, 0xffffffff,
    ULPI_RUN | ULPI_WRITE | ULPI_ADDR(addr) | val);
    return ci_ulpi_wait(ci, ULPI_RUN);
    }
#[no_mangle]
pub unsafe extern "C" fn ci_ulpi_init(ci: *mut ci_hdrc) -> c_int {
    int ci_ulpi_init(struct ci_hdrc *ci)
    {
    if (ci.platdata.phy_mode != USBPHY_INTERFACE_MODE_ULPI)
    return 0;
//
// Set PORTSC correctly so we can read/write ULPI registers for
// identification purposes
//
    hw_phymode_configure(ci);
    ci.ulpi_ops.read = ci_ulpi_read;
    ci.ulpi_ops.write = ci_ulpi_write;
    ci.ulpi = ulpi_register_interface(ci.dev, &ci.ulpi_ops);
    if (IS_ERR(ci.ulpi))
    dev_err(ci.dev, "failed to register ULPI interface");
    return PTR_ERR_OR_ZERO(ci.ulpi);
    }
#[no_mangle]
pub unsafe extern "C" fn ci_ulpi_exit(ci: *mut ci_hdrc) {
    void ci_ulpi_exit(struct ci_hdrc *ci)
    {
    if (ci.ulpi) {
    ulpi_unregister_interface(ci.ulpi);
    ci.ulpi = core::ptr::null_mut();
    }
    }
#[no_mangle]
pub unsafe extern "C" fn ci_ulpi_resume(ci: *mut ci_hdrc) -> c_int {
    int ci_ulpi_resume(struct ci_hdrc *ci)
    {
    let mut cnt: c_int = 100000;
    if (ci.platdata.phy_mode != USBPHY_INTERFACE_MODE_ULPI)
    return 0;
    while (cnt-- > 0) {
    if (hw_read(ci, OP_ULPI_VIEWPORT, ULPI_SYNC_STATE))
    return 0;
    udelay(1);
    }
    return -ETIMEDOUT;
    }
