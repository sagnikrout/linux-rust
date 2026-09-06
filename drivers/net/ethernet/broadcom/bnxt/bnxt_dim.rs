//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/broadcom/bnxt/bnxt_dim.c
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


// Broadcom NetXtreme-C/E network driver.
//
// Copyright (c) 2017-2018 Broadcom Limited
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation.
//

#[no_mangle]
pub unsafe extern "C" fn bnxt_dim_work(work: *mut work_struct) {
    void bnxt_dim_work(struct work_struct *work)
    {
    struct dim *dim = container_of(work, struct dim, work);
    struct bnxt_cp_ring_info *cpr = container_of(dim,
    struct bnxt_cp_ring_info,
    dim);
    struct bnxt_napi *bnapi = container_of(cpr,
    struct bnxt_napi,
    cp_ring);
    struct dim_cq_moder cur_moder =
    net_dim_get_rx_moderation(dim.mode, dim.profile_ix);
    cpr.rx_ring_coal.coal_ticks = cur_moder.usec;
    cpr.rx_ring_coal.coal_bufs = cur_moder.pkts;
    bnxt_hwrm_set_ring_coal(bnapi.bp, bnapi);
    dim.state = DIM_START_MEASURE;
    }
