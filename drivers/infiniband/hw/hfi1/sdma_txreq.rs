//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/hfi1/sdma_txreq.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
//
// Copyright(c) 2016 Intel Corporation.
//
// increased for AHG
pub const NUM_DESC: c_int = 6;
//
// struct sdma_desc - canonical fragment descriptor
//
// This is the descriptor carried in the tx request
// corresponding to each fragment.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdma_desc {
// private:  don't use directly
    pub qw: [u64; 2],
    pub pinning_ctx: *mut c_void,
// Release reference to @pinning_ctx. May be called in interrupt context. Must not sleep.
    pub ctx): *mut *mut void (ctx_put)(void,
}

//
// struct sdma_txreq - the sdma_txreq structure (one per packet)
// @list: for use by user and by queuing for wait
//
// This is the representation of a packet which consists of some
// number of fragments.   Storage is provided to within the structure.
// for all fragments.
//
// The storage for the descriptors are automatically extended as needed
// when the currently allocation is exceeded.
//
// The user (Verbs or PSM) may overload this structure with fields
// specific to their use by putting this struct first in their struct.
// The method of allocation of the overloaded structure is user dependent
//
// The list is the only public field in the structure.
//
pub const SDMA_TXREQ_S_OK: c_int = 0;
pub const SDMA_TXREQ_S_SENDERROR: c_int = 1;
pub const SDMA_TXREQ_S_ABORTED: c_int = 2;
pub const SDMA_TXREQ_S_SHUTDOWN: c_int = 3;
// flags bits
pub const SDMA_TXREQ_F_URGENT: c_uint = 0x0001;
pub const SDMA_TXREQ_F_AHG_COPY: c_uint = 0x0002;
pub const SDMA_TXREQ_F_USE_AHG: c_uint = 0x0004;
pub const SDMA_TXREQ_F_VIP: c_uint = 0x0010;
extern "C" {
    pub fn void(: *mut *mut callback_t)(struct sdma_txreq, _arg: c_int) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdma_txreq {
    pub list: list_head,
// private:
    pub descp: *mut sdma_desc,
// private:
    pub coalesce_buf: *mut c_void,
// private:
    pub wait: *mut iowait,
// private:
    pub complete: callback_t,

    pub sn: u64,

// private: - used in coalesce/pad processing
    pub packet_len: u16,
// private: - down-counted to trigger last
    pub tlen: u16,
// private:
    pub num_desc: u16,
// private:
    pub desc_limit: u16,
// private:
    pub next_descq_idx: u16,
// private:
    pub coalesce_idx: u16,
// private: flags
    pub flags: u16,
// private:
    pub descs: [sdma_desc; NUM_DESC],
}
