//! Automatically rewritten from C to Rust
//! Source: lib/stmp_device.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright (C) 1999 ARM Limited
// Copyright (C) 2000 Deep Blue Solutions Ltd
// Copyright 2006-2007,2010 Freescale Semiconductor, Inc. All Rights Reserved.
// Copyright 2008 Juergen Beisert, kernel@pengutronix.de
// Copyright 2009 Ilya Yanok, Emcraft Systems Ltd, yanok@emcraft.com
// Copyright (C) 2011 Wolfram Sang, Pengutronix e.K.
//

//
// Clear the bit and poll it cleared.  This is usually called with
// a reset address and mask being either SFTRST(bit 31) or CLKGATE
// (bit 30).
//
#[no_mangle]
unsafe extern "C" fn stmp_clear_poll_bit(addr: *mut void __iomem, mask: u32) -> c_int {
    static int stmp_clear_poll_bit(void __iomem *addr, u32 mask)
    {
    let mut timeout: c_int = 0x400;
    writel(mask, addr + STMP_OFFSET_REG_CLR);
    udelay(1);
    while ((readl(addr) & mask) && --timeout)
// nothing */;
    return !timeout;
    }
#[no_mangle]
pub unsafe extern "C" fn stmp_reset_block(reset_addr: *mut void __iomem) -> c_int {
    int stmp_reset_block(void __iomem *reset_addr)
    {
    int ret;
    let mut timeout: c_int = 0x400;
// clear and poll SFTRST
    ret = stmp_clear_poll_bit(reset_addr, STMP_MODULE_SFTRST);
    if (unlikely(ret))
    goto error;
// clear CLKGATE
    writel(STMP_MODULE_CLKGATE, reset_addr + STMP_OFFSET_REG_CLR);
// set SFTRST to reset the block
    writel(STMP_MODULE_SFTRST, reset_addr + STMP_OFFSET_REG_SET);
    udelay(1);
// poll CLKGATE becoming set
    while ((!(readl(reset_addr) & STMP_MODULE_CLKGATE)) && --timeout)
// nothing */;
    if (unlikely(!timeout))
    goto error;
// clear and poll SFTRST
    ret = stmp_clear_poll_bit(reset_addr, STMP_MODULE_SFTRST);
    if (unlikely(ret))
    goto error;
// clear and poll CLKGATE
    ret = stmp_clear_poll_bit(reset_addr, STMP_MODULE_CLKGATE);
    if (unlikely(ret))
    goto error;
    return 0;
    error:
    pr_err("%s(%p): module reset timeout\n", __func__, reset_addr);
    return -ETIMEDOUT;
    }
    EXPORT_SYMBOL(stmp_reset_block);
