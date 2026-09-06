//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/marvell/octeontx2/af/cn20k/npa.c
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
// Marvell RVU Admin Function driver
//
// Copyright (C) 2024 Marvell.
//

    int rvu_mbox_handler_npa_cn20k_aq_enq(struct rvu *rvu,
    struct npa_cn20k_aq_enq_req *req,
    struct npa_cn20k_aq_enq_rsp *rsp)
    {
    return rvu_npa_aq_enq_inst(rvu, (struct npa_aq_enq_req *)req,
    (struct npa_aq_enq_rsp *)rsp);
    }
    EXPORT_SYMBOL(rvu_mbox_handler_npa_cn20k_aq_enq);
