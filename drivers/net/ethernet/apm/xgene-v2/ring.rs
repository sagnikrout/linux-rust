//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/apm/xgene-v2/ring.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Applied Micro X-Gene SoC Ethernet v2 Driver
//
// Copyright (c) 2017, Applied Micro Circuits Corporation
// Author(s): Iyappan Subramanian <isubramanian@apm.com>
// Keyur Chudgar <kchudgar@apm.com>
//
pub const XGENE_ENET_DESC_SIZE: c_int = 64;
pub const XGENE_ENET_NUM_DESC: c_int = 256;
pub const NUM_BUFS: c_int = 8;
pub const SLOT_EMPTY: c_uint = 0xfff;
pub const DMATXCTRL: c_uint = 0xa180;
pub const DMATXDESCL: c_uint = 0xa184;
pub const DMATXDESCH: c_uint = 0xa1a0;
pub const DMATXSTATUS: c_uint = 0xa188;
pub const DMARXCTRL: c_uint = 0xa18c;
pub const DMARXDESCL: c_uint = 0xa190;
pub const DMARXDESCH: c_uint = 0xa1a4;
pub const DMARXSTATUS: c_uint = 0xa194;
pub const DMAINTRMASK: c_uint = 0xa198;
pub const DMAINTERRUPT: c_uint = 0xa19c;
pub const D_POS: c_int = 62;
pub const D_LEN: c_int = 2;
pub const E_POS: c_int = 63;
pub const E_LEN: c_int = 1;
pub const PKT_ADDRL_POS: c_int = 0;
pub const PKT_ADDRL_LEN: c_int = 32;
pub const PKT_ADDRH_POS: c_int = 32;
pub const PKT_ADDRH_LEN: c_int = 10;
pub const PKT_SIZE_POS: c_int = 32;
pub const PKT_SIZE_LEN: c_int = 12;
pub const NEXT_DESC_ADDRL_POS: c_int = 0;
pub const NEXT_DESC_ADDRL_LEN: c_int = 32;
pub const NEXT_DESC_ADDRH_POS: c_int = 48;
pub const NEXT_DESC_ADDRH_LEN: c_int = 10;
pub const TXPKTCOUNT_POS: c_int = 16;
pub const TXPKTCOUNT_LEN: c_int = 8;
pub const RXPKTCOUNT_POS: c_int = 16;
pub const RXPKTCOUNT_LEN: c_int = 8;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xge_raw_desc {
    pub m0: __le64,
    pub m1: __le64,
    pub m2: __le64,
    pub m3: __le64,
    pub m4: __le64,
    pub m5: __le64,
    pub m6: __le64,
    pub m7: __le64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pkt_info {
    pub skb: *mut sk_buff,
    pub dma_addr: dma_addr_t,
    pub pkt_buf: *mut c_void,
}

// software context of a descriptor ring
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xge_desc_ring {
    pub ndev: *mut net_device,
    pub dma_addr: dma_addr_t,
    pub head: u8,
    pub tail: u8,
    pub desc_addr: *mut c_void,
    pub raw_desc: *mut xge_raw_desc,
}

extern "C" {
    pub fn pkt_info(_arg: *mut pkt_info) -> struct;
}

extern "C" {
    pub fn xge_setup_desc(ring: *mut xge_desc_ring);
}
extern "C" {
    pub fn xge_update_tx_desc_addr(pdata: *mut xge_pdata);
}
extern "C" {
    pub fn xge_update_rx_desc_addr(pdata: *mut xge_pdata);
}
extern "C" {
    pub fn xge_intr_enable(pdata: *mut xge_pdata);
}
extern "C" {
    pub fn xge_intr_disable(pdata: *mut xge_pdata);
}
