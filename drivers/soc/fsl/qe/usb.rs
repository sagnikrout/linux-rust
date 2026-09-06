//! Automatically rewritten from C to Rust
//! Source: drivers/soc/fsl/qe/usb.c
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
// QE USB routines
//
// Copyright 2006 Freescale Semiconductor, Inc.
// Shlomi Gridish <gridish@freescale.com>
// Jerry Huang <Chang-Ming.Huang@freescale.com>
// Copyright (c) MontaVista Software, Inc. 2008.
// Anton Vorontsov <avorontsov@ru.mvista.com>
//

#[no_mangle]
pub unsafe extern "C" fn qe_usb_clock_set(clk: enum qe_clock, rate: c_int) -> c_int {
    int qe_usb_clock_set(enum qe_clock clk, int rate)
    {
    struct qe_mux __iomem *mux = &qe_immr.qmx;
    unsigned long flags;
    u32 val;
    switch (clk) {
    case QE_CLK3:  val = QE_CMXGCR_USBCS_CLK3;  break;
    case QE_CLK5:  val = QE_CMXGCR_USBCS_CLK5;  break;
    case QE_CLK7:  val = QE_CMXGCR_USBCS_CLK7;  break;
    case QE_CLK9:  val = QE_CMXGCR_USBCS_CLK9;  break;
    case QE_CLK13: val = QE_CMXGCR_USBCS_CLK13; break;
    case QE_CLK17: val = QE_CMXGCR_USBCS_CLK17; break;
    case QE_CLK19: val = QE_CMXGCR_USBCS_CLK19; break;
    case QE_CLK21: val = QE_CMXGCR_USBCS_CLK21; break;
    case QE_BRG9:  val = QE_CMXGCR_USBCS_BRG9;  break;
    case QE_BRG10: val = QE_CMXGCR_USBCS_BRG10; break;
    default:
    pr_err("%s: requested unknown clock %d\n", __func__, clk);
    return -EINVAL;
    }
    if (qe_clock_is_brg(clk))
    qe_setbrg(clk, rate, 1);
    spin_lock_irqsave(&cmxgcr_lock, flags);
    qe_clrsetbits_be32(&mux.cmxgcr, QE_CMXGCR_USBCS, val);
    spin_unlock_irqrestore(&cmxgcr_lock, flags);
    return 0;
    }
    EXPORT_SYMBOL(qe_usb_clock_set);
