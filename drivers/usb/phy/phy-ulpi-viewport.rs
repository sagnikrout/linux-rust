//! Automatically rewritten from C to Rust
//! Source: drivers/usb/phy/phy-ulpi-viewport.c
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
// Copyright (C) 2011 Google, Inc.
//

#[no_mangle]
unsafe extern "C" fn ulpi_viewport_wait(view: *mut void __iomem, mask: u32) -> c_int {
    static int ulpi_viewport_wait(void __iomem *view, u32 mask)
    {
    u32 val;
    return readl_poll_timeout_atomic(view, val, !(val & mask), 1, 2000);
    }
#[no_mangle]
unsafe extern "C" fn ulpi_viewport_read(otg: *mut usb_phy, reg: u32) -> c_int {
    static int ulpi_viewport_read(struct usb_phy *otg, u32 reg)
    {
    int ret;
    void __iomem *view = otg.io_priv;
    writel(ULPI_VIEW_WAKEUP | ULPI_VIEW_WRITE, view);
    ret = ulpi_viewport_wait(view, ULPI_VIEW_WAKEUP);
    if (ret)
    return ret;
    writel(ULPI_VIEW_RUN | ULPI_VIEW_READ | ULPI_VIEW_ADDR(reg), view);
    ret = ulpi_viewport_wait(view, ULPI_VIEW_RUN);
    if (ret)
    return ret;
    return ULPI_VIEW_DATA_READ(readl(view));
    }
#[no_mangle]
unsafe extern "C" fn ulpi_viewport_write(otg: *mut usb_phy, val: u32, reg: u32) -> c_int {
    static int ulpi_viewport_write(struct usb_phy *otg, u32 val, u32 reg)
    {
    int ret;
    void __iomem *view = otg.io_priv;
    writel(ULPI_VIEW_WAKEUP | ULPI_VIEW_WRITE, view);
    ret = ulpi_viewport_wait(view, ULPI_VIEW_WAKEUP);
    if (ret)
    return ret;
    writel(ULPI_VIEW_RUN | ULPI_VIEW_WRITE | ULPI_VIEW_DATA_WRITE(val) |
    ULPI_VIEW_ADDR(reg), view);
    return ulpi_viewport_wait(view, ULPI_VIEW_RUN);
    }
    struct usb_phy_io_ops ulpi_viewport_access_ops = {
    .read	= ulpi_viewport_read,
    .write	= ulpi_viewport_write,
    };
    EXPORT_SYMBOL_GPL(ulpi_viewport_access_ops);
