//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/intel/libeth/tx.c
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
// Copyright (C) 2025 Intel Corporation

// Tx buffer completion
    DEFINE_STATIC_CALL_NULL(bulk, libeth_xdp_return_buff_bulk);
    DEFINE_STATIC_CALL_NULL(xsk, libeth_xsk_buff_free_slow);
//
// libeth_tx_complete_any - perform Tx completion for one SQE of any type
// @sqe: Tx buffer to complete
// @cp: polling params
//
// Can be used to complete both regular and XDP SQEs, for example when
// destroying queues.
// When libeth_xdp is not loaded, XDPSQEs won't be handled.
//
#[no_mangle]
pub unsafe extern "C" fn libeth_tx_complete_any(sqe: *mut libeth_sqe, cp: *mut libeth_cq_pp) {
    void libeth_tx_complete_any(struct libeth_sqe *sqe, struct libeth_cq_pp *cp)
    {
    if (sqe.type >= __LIBETH_SQE_XDP_START)
    __libeth_xdp_complete_tx(sqe, cp, static_call(bulk),
    static_call(xsk));
    else
    libeth_tx_complete(sqe, cp);
    }
    EXPORT_SYMBOL_GPL(libeth_tx_complete_any);
// Module
#[no_mangle]
pub unsafe extern "C" fn libeth_attach_xdp(ops: *const libeth_xdp_ops) {
    void libeth_attach_xdp(const struct libeth_xdp_ops *ops)
    {
    static_call_update(bulk, ops ? ops.bulk : core::ptr::null_mut());
    static_call_update(xsk, ops ? ops.xsk : core::ptr::null_mut());
    }
    EXPORT_SYMBOL_GPL(libeth_attach_xdp);
