//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/broadcom/bnge/bnge_hwrm.h
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
// Copyright (c) 2025 Broadcom

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bnge_hwrm_ctx_flags {
    BNGE_HWRM_INTERNAL_CTX_OWNED	= BIT(0),
    BNGE_HWRM_INTERNAL_RESP_DIRTY	= BIT(1),
    BNGE_HWRM_CTX_SILENT		= BIT(2),
    BNGE_HWRM_FULL_WAIT		= BIT(3),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnge_hwrm_ctx {
    pub sentinel: u64,
    pub dma_handle: dma_addr_t,
    pub resp: *mut output,
    pub req: *mut input,
    pub slice_handle: dma_addr_t,
    pub slice_addr: *mut c_void,
    pub slice_size: u32,
    pub req_len: u32,
    pub flags: bnge_hwrm_ctx_flags,
    pub timeout: c_uint,
    pub allocated: u32,
    pub gfp: gfp_t,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bnge_hwrm_wait_state {
    BNGE_HWRM_PENDING,
    BNGE_HWRM_DEFERRED,
    BNGE_HWRM_COMPLETE,
    BNGE_HWRM_CANCELLED,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bnge_hwrm_chnl {

    struct bnge_hwrm_wait_token {
    struct rcu_head rcu;
    struct hlist_node node;
    enum bnge_hwrm_wait_state state;
    enum bnge_hwrm_chnl dst;
    u16 seq_id;
}

pub const BNGE_DFLT_HWRM_CMD_TIMEOUT: c_int = 500;
pub const BNGE_GRCPF_REG_CHIMP_COMM: c_uint = 0x0;
pub const BNGE_GRCPF_REG_CHIMP_COMM_TRIGGER: c_uint = 0x100;

pub const BNGE_SHORT_HWRM_CMD_TIMEOUT: c_int = 20;

pub const BNGE_HWRM_TARGET: c_uint = 0xffff;

pub const BNGE_HWRM_REQ_MAX_SIZE: c_int = 128;

pub const BNGE_HWRM_DMA_ALIGN: c_int = 16;
pub const BNGE_HWRM_SENTINEL: c_uint = 0xb6e1f68a12e9a7eb /* arbitrary value */;
pub const BNGE_HWRM_SHORT_MIN_TIMEOUT: c_int = 3;
pub const BNGE_HWRM_SHORT_MAX_TIMEOUT: c_int = 10;
pub const BNGE_HWRM_SHORT_TIMEOUT_COUNTER: c_int = 5;
pub const BNGE_HWRM_MIN_TIMEOUT: c_int = 25;
pub const BNGE_HWRM_MAX_TIMEOUT: c_int = 40;
pub const BNGE_HWRM_FIN_WAIT_USEC: c_int = 50000;
extern "C" {
    pub fn bnge_cleanup_hwrm_resources(bd: *mut bnge_dev);
}
extern "C" {
    pub fn bnge_init_hwrm_resources(bd: *mut bnge_dev) -> c_int;
}

extern "C" {
    pub fn bnge_hwrm_req_drop(bd: *mut bnge_dev, req: *mut c_void);
}
extern "C" {
    pub fn bnge_hwrm_req_send(bd: *mut bnge_dev, req: *mut c_void) -> c_int;
}
extern "C" {
    pub fn bnge_hwrm_req_send_silent(bd: *mut bnge_dev, req: *mut c_void) -> c_int;
}
extern "C" {
    pub fn bnge_hwrm_req_alloc_flags(bd: *mut bnge_dev, req: *mut c_void, flags: gfp_t);
}
