//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/dma/ti/k3-udma.h
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
// Copyright (C) 2019 Texas Instruments Incorporated - http://www.ti.com
//

// Global registers
pub const UDMA_REV_REG: c_uint = 0x0;
pub const UDMA_PERF_CTL_REG: c_uint = 0x4;
pub const UDMA_EMU_CTL_REG: c_uint = 0x8;
pub const UDMA_PSIL_TO_REG: c_uint = 0x10;
pub const UDMA_UTC_CTL_REG: c_uint = 0x1c;

pub const UDMA_RX_FLOW_ID_FW_OES_REG: c_uint = 0x80;
pub const UDMA_RX_FLOW_ID_FW_STATUS_REG: c_uint = 0x88;
// BCHANRT/TCHANRT/RCHANRT registers
pub const UDMA_CHAN_RT_CTL_REG: c_uint = 0x0;
pub const UDMA_CHAN_RT_SWTRIG_REG: c_uint = 0x8;
pub const UDMA_CHAN_RT_STDATA_REG: c_uint = 0x80;

pub const UDMA_CHAN_RT_PCNT_REG: c_uint = 0x400;
pub const UDMA_CHAN_RT_BCNT_REG: c_uint = 0x408;
pub const UDMA_CHAN_RT_SBCNT_REG: c_uint = 0x410;
// UDMA_CAP Registers

// UDMA_CHAN_RT_CTL_REG

// UDMA_CHAN_RT_PEER_RT_EN_REG

//
// UDMA_TCHAN_RT_PEER_STATIC_TR_XY_REG
// UDMA_RCHAN_RT_PEER_STATIC_TR_XY_REG
//

//
// UDMA_TCHAN_RT_PEER_STATIC_TR_Z_REG
// UDMA_RCHAN_RT_PEER_STATIC_TR_Z_REG
//

// Address Space Select
pub const K3_ADDRESS_ASEL_SHIFT: c_int = 48;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum udma_rm_range {
    RM_RANGE_BCHAN = 0,
    RM_RANGE_TCHAN,
    RM_RANGE_RCHAN,
    RM_RANGE_RFLOW,
    RM_RANGE_TFLOW,
    RM_RANGE_LAST,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct udma_tisci_rm {
    pub tisci: *const ti_sci_handle,
    pub tisci_udmap_ops: *const ti_sci_rm_udmap_ops,
    pub tisci_dev_id: u32,
// tisci information for PSI-L thread pairing/unpairing
    pub tisci_psil_ops: *const ti_sci_rm_psil_ops,
    pub tisci_navss_dev_id: u32,
    pub rm_ranges: [*mut ti_sci_resource; RM_RANGE_LAST],
}

// Direct access to UDMA low lever resources for the glue layer
extern "C" {
    pub fn xudma_navss_psil_pair(ud: *mut udma_dev, src_thread: u32, dst_thread: u32) -> c_int;
}
extern "C" {
    pub fn xudma_dev_get_psil_base(ud: *mut udma_dev) -> u32;
}
extern "C" {
    pub fn xudma_alloc_gp_rflow_range(ud: *mut udma_dev, from: c_int, cnt: c_int) -> c_int;
}
extern "C" {
    pub fn xudma_free_gp_rflow_range(ud: *mut udma_dev, from: c_int, cnt: c_int) -> c_int;
}
extern "C" {
    pub fn xudma_tchan_put(ud: *mut udma_dev, p: *mut udma_tchan);
}
extern "C" {
    pub fn xudma_rchan_put(ud: *mut udma_dev, p: *mut udma_rchan);
}
extern "C" {
    pub fn xudma_rflow_put(ud: *mut udma_dev, p: *mut udma_rflow);
}
extern "C" {
    pub fn xudma_tchan_get_id(p: *mut udma_tchan) -> c_int;
}
extern "C" {
    pub fn xudma_rchan_get_id(p: *mut udma_rchan) -> c_int;
}
extern "C" {
    pub fn xudma_rflow_get_id(p: *mut udma_rflow) -> c_int;
}
extern "C" {
    pub fn xudma_tchanrt_read(tchan: *mut udma_tchan, reg: c_int) -> u32;
}
extern "C" {
    pub fn xudma_tchanrt_write(tchan: *mut udma_tchan, reg: c_int, val: u32);
}
extern "C" {
    pub fn xudma_rchanrt_read(rchan: *mut udma_rchan, reg: c_int) -> u32;
}
extern "C" {
    pub fn xudma_rchanrt_write(rchan: *mut udma_rchan, reg: c_int, val: u32);
}
extern "C" {
    pub fn xudma_rflow_is_gp(ud: *mut udma_dev, id: c_int) -> bool;
}
extern "C" {
    pub fn xudma_get_rflow_ring_offset(ud: *mut udma_dev) -> c_int;
}
extern "C" {
    pub fn xudma_is_pktdma(ud: *mut udma_dev) -> c_int;
}
extern "C" {
    pub fn xudma_pktdma_tflow_get_irq(ud: *mut udma_dev, udma_tflow_id: c_int) -> c_int;
}
extern "C" {
    pub fn xudma_pktdma_rflow_get_irq(ud: *mut udma_dev, udma_rflow_id: c_int) -> c_int;
}
