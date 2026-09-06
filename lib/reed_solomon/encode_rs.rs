//! Automatically rewritten from C to Rust
//! Source: lib/reed_solomon/encode_rs.c
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
// Generic Reed Solomon encoder / decoder library
//
// Copyright 2002, Phil Karn, KA9Q
// May be used under the terms of the GNU General Public License (GPL)
//
// Adaption to the kernel by Thomas Gleixner (tglx@kernel.org)
//
// Generic data width independent code which is included by the wrappers.
//
    {
    struct rs_codec *rs = rsc.codec;
    int i, j, pad;
    let mut nn: c_int = rs.nn;
    let mut nroots: c_int = rs.nroots;
    uint16_t *alpha_to = rs.alpha_to;
    uint16_t *index_of = rs.index_of;
    uint16_t *genpoly = rs.genpoly;
    uint16_t fb;
    let mut msk: u16 = (uint16_t) rs.nn;
// Check length parameter for validity
    pad = nn - nroots - len;
    if (pad < 0 || pad >= nn)
    return -ERANGE;
    for (i = 0; i < len; i++) {
    fb = index_of[((((uint16_t) data[i])^invmsk) & msk) ^ par[0]];
// feedback term is non-zero
    if (fb != nn) {
    for (j = 1; j < nroots; j++) {
    par[j] ^= alpha_to[rs_modnn(rs, fb +
    genpoly[nroots - j])];
    }
    }
// Shift
    memmove(&par[0], &par[1], sizeof(uint16_t) * (nroots - 1));
    if (fb != nn) {
    par[nroots - 1] = alpha_to[rs_modnn(rs,
    fb + genpoly[0])];
    } else {
    par[nroots - 1] = 0;
    }
    }
    return 0;
    }
