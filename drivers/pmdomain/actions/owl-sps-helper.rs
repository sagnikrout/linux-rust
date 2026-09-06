//! Automatically rewritten from C to Rust
//! Source: drivers/pmdomain/actions/owl-sps-helper.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// Actions Semi Owl Smart Power System (SPS) shared helpers
//
// Copyright 2012 Actions Semi Inc.
// Author: Actions Semi, Inc.
//
// Copyright (c) 2017 Andreas Färber
//

pub const OWL_SPS_PG_CTL: c_uint = 0x0;
#[no_mangle]
pub unsafe extern "C" fn owl_sps_set_pg(base: *mut void __iomem, pwr_mask: u32, ack_mask: u32, enable: bool) -> c_int {
    int owl_sps_set_pg(void __iomem *base, u32 pwr_mask, u32 ack_mask, bool enable)
    {
    u32 val;
    bool ack;
    int timeout;
    val = readl(base + OWL_SPS_PG_CTL);
    ack = val & ack_mask;
    if (ack == enable)
    return 0;
    if (enable)
    val |= pwr_mask;
    else
    val &= ~pwr_mask;
    writel(val, base + OWL_SPS_PG_CTL);
    for (timeout = 5000; timeout > 0; timeout -= 50) {
    val = readl(base + OWL_SPS_PG_CTL);
    if ((val & ack_mask) == (enable ? ack_mask : 0))
    break;
    udelay(50);
    }
    if (timeout <= 0)
    return -ETIMEDOUT;
    udelay(10);
    return 0;
    }
    EXPORT_SYMBOL_GPL(owl_sps_set_pg);
