//! Automatically rewritten from C to Rust
//! Source: drivers/net/wireless/mediatek/mt7601u/core.c
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
// Copyright (C) 2014 Felix Fietkau <nbd@openwrt.org>
// Copyright (C) 2015 Jakub Kicinski <kubakici@wp.pl>
//

#[no_mangle]
pub unsafe extern "C" fn mt7601u_wait_asic_ready(dev: *mut mt7601u_dev) -> c_int {
    int mt7601u_wait_asic_ready(struct mt7601u_dev *dev)
    {
    let mut i: c_int = 100;
    u32 val;
    do {
    if (test_bit(MT7601U_STATE_REMOVED, &dev.state))
    return -EIO;
    val = mt7601u_rr(dev, MT_MAC_CSR0);
    if (val && ~val)
    return 0;
    udelay(10);
    } while (i--);
    return -EIO;
    }
    bool mt76_poll(struct mt7601u_dev *dev, u32 offset, u32 mask, u32 val,
    int timeout)
    {
    u32 cur;
    timeout /= 10;
    do {
    if (test_bit(MT7601U_STATE_REMOVED, &dev.state))
    return false;
    cur = mt7601u_rr(dev, offset) & mask;
    if (cur == val)
    return true;
    udelay(10);
    } while (timeout-- > 0);
    dev_err(dev.dev, "Error: Time out with reg %08x\n", offset);
    return false;
    }
    bool mt76_poll_msec(struct mt7601u_dev *dev, u32 offset, u32 mask, u32 val,
    int timeout)
    {
    u32 cur;
    timeout /= 10;
    do {
    if (test_bit(MT7601U_STATE_REMOVED, &dev.state))
    return false;
    cur = mt7601u_rr(dev, offset) & mask;
    if (cur == val)
    return true;
    msleep(10);
    } while (timeout-- > 0);
    dev_err(dev.dev, "Error: Time out with reg %08x\n", offset);
    return false;
    }
