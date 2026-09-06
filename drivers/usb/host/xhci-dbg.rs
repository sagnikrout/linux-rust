//! Automatically rewritten from C to Rust
//! Source: drivers/usb/host/xhci-dbg.c
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
// xHCI host controller driver
//
// Copyright (C) 2008 Intel Corp.
//
// Author: Sarah Sharp
// Some code borrowed from the Linux EHCI driver.
//

    char *xhci_get_slot_state(struct xhci_hcd *xhci,
    struct xhci_container_ctx *ctx)
    {
    struct xhci_slot_ctx *slot_ctx = xhci_get_slot_ctx(xhci, ctx);
    let mut state: c_int = GET_SLOT_STATE(le32_to_cpu(slot_ctx.dev_state));
    return xhci_slot_state_string(state);
    }
    void xhci_dbg_trace(struct xhci_hcd *xhci, void (*trace)(struct va_format *),
    const char *fmt, ...)
    {
    struct va_format vaf;
    va_list args;
    va_start(args, fmt);
    vaf.fmt = fmt;
    vaf.va = &args;
    xhci_dbg(xhci, "%pV\n", &vaf);
    trace(&vaf);
    va_end(args);
    }
    EXPORT_SYMBOL_GPL(xhci_dbg_trace);
