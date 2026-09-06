//! Automatically rewritten from C to Rust
//! Source: drivers/i3c/master/mipi-i3c-hci/dct_v1.c
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


// SPDX-License-Identifier: BSD-3-Clause
//
// Copyright (c) 2020, MIPI Alliance, Inc.
//
// Author: Nicolas Pitre <npitre@baylibre.com>
//

//
// Device Characteristic Table
//
    void i3c_hci_dct_get_val(struct i3c_hci *hci, unsigned int dct_idx,
    u64 *pid, unsigned int *dcr, unsigned int *bcr)
    {
    void __iomem *reg = hci.DCT_regs + dct_idx * 4 * 4;
    u32 dct_entry_data[4];
    unsigned int i;
    for (i = 0; i < 4; i++) {
    dct_entry_data[i] = readl(reg);
    reg += 4;
    }
// pid = ((u64)dct_entry_data[0]) << (47 - 32 + 1) |
    FIELD_GET(W1_MASK(47, 32), dct_entry_data[1]);
// dcr = FIELD_GET(W2_MASK(71, 64), dct_entry_data[2]);
// bcr = FIELD_GET(W2_MASK(79, 72), dct_entry_data[2]);
    }
