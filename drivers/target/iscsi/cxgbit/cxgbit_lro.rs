//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/target/iscsi/cxgbit/cxgbit_lro.h
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


//
// Copyright (c) 2016 Chelsio Communications, Inc.
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation.
//

pub const LRO_FLUSH_LEN_MAX: c_int = 65535;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxgbit_lro_cb {
    pub csk: *mut cxgbit_sock,
    pub pdu_totallen: u32,
    pub offset: u32,
    pub pdu_idx: u8,
    pub complete: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cxgbit_pducb_flags {
    PDUCBF_RX_HDR		= (1 << 0), /* received pdu header */
    PDUCBF_RX_DATA		= (1 << 1), /* received pdu payload */
    PDUCBF_RX_STATUS	= (1 << 2), /* received ddp status */
    PDUCBF_RX_DATA_DDPD	= (1 << 3), /* pdu payload ddp'd */
    PDUCBF_RX_DDP_CMP	= (1 << 4), /* ddp completion */
    PDUCBF_RX_HCRC_ERR	= (1 << 5), /* header digest error */
    PDUCBF_RX_DCRC_ERR	= (1 << 6), /* data digest error */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxgbit_lro_pdu_cb {
    pub flags: u8,
    pub frags: u8,
    pub hfrag_idx: u8,
    pub nr_dfrags: u8,
    pub dfrag_idx: u8,
    pub complete: bool,
    pub seq: u32,
    pub pdulen: u32,
    pub hlen: u32,
    pub dlen: u32,
    pub doffset: u32,
    pub ddigest: u32,
    pub hdr: *mut c_void,
}

