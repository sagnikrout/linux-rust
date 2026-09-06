//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/broadcom/bnxt/bnxt_hwrm.h
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
// Copyright (c) 2020 Broadcom Limited
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bnxt_hwrm_ctx_flags {
// Update the HWRM_API_FLAGS right below for any new non-internal bit added here
    BNXT_HWRM_INTERNAL_CTX_OWNED	= BIT(0), /* caller owns the context */
    BNXT_HWRM_INTERNAL_RESP_DIRTY	= BIT(1), /* response contains data */
    BNXT_HWRM_CTX_SILENT		= BIT(2), /* squelch firmware errors */
    BNXT_HWRM_FULL_WAIT		= BIT(3), /* wait for full timeout of HWRM command */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_hwrm_ctx {
    pub sentinel: u64,
    pub dma_handle: dma_addr_t,
    pub resp: *mut output,
    pub req: *mut input,
    pub slice_handle: dma_addr_t,
    pub slice_addr: *mut c_void,
    pub slice_size: u32,
    pub req_len: u32,
    pub flags: bnxt_hwrm_ctx_flags,
    pub timeout: c_uint,
    pub allocated: u32,
    pub gfp: gfp_t,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bnxt_hwrm_wait_state {
    BNXT_HWRM_PENDING,
    BNXT_HWRM_DEFERRED,
    BNXT_HWRM_COMPLETE,
    BNXT_HWRM_CANCELLED,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bnxt_hwrm_chnl {

    struct bnxt_hwrm_wait_token {
    struct rcu_head rcu;
    struct hlist_node node;
    enum bnxt_hwrm_wait_state state;
    enum bnxt_hwrm_chnl dst;
    u16 seq_id;
}

extern "C" {
    pub fn hwrm_update_token(bp: *mut bnxt, seq: u16, s: bnxt_hwrm_wait_state);
}

pub const SHORT_HWRM_CMD_TIMEOUT: c_int = 20;

pub const BNXT_HWRM_TARGET: c_uint = 0xffff;

pub const BNXT_HWRM_REQ_MAX_SIZE: c_int = 128;

pub const BNXT_HWRM_DMA_ALIGN: c_int = 16;
pub const BNXT_HWRM_SENTINEL: c_uint = 0xb6e1f68a12e9a7eb /* arbitrary value */;

pub const HWRM_SHORT_MIN_TIMEOUT: c_int = 3;
pub const HWRM_SHORT_MAX_TIMEOUT: c_int = 10;
pub const HWRM_SHORT_TIMEOUT_COUNTER: c_int = 5;
pub const HWRM_MIN_TIMEOUT: c_int = 25;
pub const HWRM_MAX_TIMEOUT: c_int = 40;
pub const HWRM_VALID_BIT_DELAY_USEC: c_int = 50000;
extern "C" {
    pub fn __hwrm_req_init(bp: *mut bnxt, req: *mut c_void, req_type: u16, req_len: u32) -> c_int;
}

extern "C" {
    pub fn hwrm_req_drop(bp: *mut bnxt, req: *mut c_void);
}
extern "C" {
    pub fn hwrm_req_flags(bp: *mut bnxt, req: *mut c_void, flags: bnxt_hwrm_ctx_flags);
}
extern "C" {
    pub fn hwrm_req_timeout(bp: *mut bnxt, req: *mut c_void, timeout: c_uint);
}
extern "C" {
    pub fn hwrm_req_send(bp: *mut bnxt, req: *mut c_void) -> c_int;
}
extern "C" {
    pub fn hwrm_req_send_silent(bp: *mut bnxt, req: *mut c_void) -> c_int;
}
extern "C" {
    pub fn hwrm_req_replace(bp: *mut bnxt, req: *mut c_void, new_req: *mut c_void, len: u32) -> c_int;
}
extern "C" {
    pub fn hwrm_req_alloc_flags(bp: *mut bnxt, req: *mut c_void, flags: gfp_t);
}
// Older devices can only support req length of 128.
// HWRM_FUNC_CFG requests which don't need fields starting at
// num_quic_tx_key_ctxs can use this helper to avoid getting -E2BIG.
//
extern "C" {
    pub fn __hwrm_req_init(_arg: bp, )req: *mut (void, _arg: HWRM_FUNC_CFG, _arg: req_len) -> return;
}
