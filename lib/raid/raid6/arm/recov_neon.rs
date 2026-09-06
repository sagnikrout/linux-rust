//! Automatically rewritten from C to Rust
//! Source: lib/raid/raid6/arm/recov_neon.c
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
// Copyright (C) 2012 Intel Corporation
// Copyright (C) 2017 Linaro Ltd. <ard.biesheuvel@linaro.org>
//

    static void raid6_2data_recov_neon(int disks, size_t bytes, int faila,
    int failb, void **ptrs)
    {
    u8 *p, *q, *dp, *dq;
    const u8 *pbmul;	/* P multiplier table for B data */
    const u8 *qmul;		/* Q multiplier table (for both) */
    p = (u8 *)ptrs[disks - 2];
    q = (u8 *)ptrs[disks - 1];
//
// Compute syndrome with zero for the missing data pages
// Use the dead data pages as temporary storage for
// delta p and delta q
//
    dp = (u8 *)ptrs[faila];
    ptrs[faila] = page_address(ZERO_PAGE(0));
    ptrs[disks - 2] = dp;
    dq = (u8 *)ptrs[failb];
    ptrs[failb] = page_address(ZERO_PAGE(0));
    ptrs[disks - 1] = dq;
    raid6_gen_syndrome(disks, bytes, ptrs);
// Restore pointer table
    ptrs[faila]     = dp;
    ptrs[failb]     = dq;
    ptrs[disks - 2] = p;
    ptrs[disks - 1] = q;
// Now, pick the proper data tables
    pbmul = raid6_vgfmul[raid6_gfexi[failb-faila]];
    qmul  = raid6_vgfmul[raid6_gfinv[raid6_gfexp[faila] ^
    raid6_gfexp[failb]]];
    scoped_ksimd()
    __raid6_2data_recov_neon(bytes, p, q, dp, dq, pbmul, qmul);
    }
    static void raid6_datap_recov_neon(int disks, size_t bytes, int faila,
    void **ptrs)
    {
    u8 *p, *q, *dq;
    const u8 *qmul;		/* Q multiplier table */
    p = (u8 *)ptrs[disks - 2];
    q = (u8 *)ptrs[disks - 1];
//
// Compute syndrome with zero for the missing data page
// Use the dead data page as temporary storage for delta q
//
    dq = (u8 *)ptrs[faila];
    ptrs[faila] = page_address(ZERO_PAGE(0));
    ptrs[disks - 1] = dq;
    raid6_gen_syndrome(disks, bytes, ptrs);
// Restore pointer table
    ptrs[faila]     = dq;
    ptrs[disks - 1] = q;
// Now, pick the proper data tables
    qmul = raid6_vgfmul[raid6_gfinv[raid6_gfexp[faila]]];
    scoped_ksimd()
    __raid6_datap_recov_neon(bytes, p, q, dq, qmul);
    }
    const struct raid6_recov_calls raid6_recov_neon = {
    .data2		= raid6_2data_recov_neon,
    .datap		= raid6_datap_recov_neon,
    .name		= "neon",
    };
