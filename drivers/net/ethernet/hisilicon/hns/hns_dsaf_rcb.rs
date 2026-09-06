//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/hisilicon/hns/hns_dsaf_rcb.h
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
// Copyright (c) 2014-2015 Hisilicon Limited.
//

pub const HNS_RCB_IRQ_NUM_PER_QUEUE: c_int = 2;
pub const HNS_RCB_IRQ_IDX_TX: c_int = 0;
pub const HNS_RCB_IRQ_IDX_RX: c_int = 1;
pub const HNS_RCB_TX_REG_OFFSET: c_uint = 0x40;

pub const HNS_RCB_DEBUG_NW_ENGINE_NUM: c_int = 1;
pub const HNS_RCB_RING_MAX_BD_PER_PKT: c_int = 3;
pub const HNS_RCB_RING_MAX_TXBD_PER_PKT: c_int = 3;
pub const HNS_RCBV2_RING_MAX_TXBD_PER_PKT: c_int = 8;

pub const HNS_RCB_RING_MAX_PENDING_BD: c_int = 1024;
pub const HNS_RCB_RING_MIN_PENDING_BD: c_int = 16;
pub const HNS_RCB_REG_OFFSET: c_uint = 0x10000;
pub const HNS_RCB_TX_FRAMES_LOW: c_int = 1;
pub const HNS_RCB_RX_FRAMES_LOW: c_int = 1;
pub const HNS_RCB_TX_FRAMES_HIGH: c_int = 1023;
pub const HNS_RCB_RX_FRAMES_HIGH: c_int = 1023;
pub const HNS_RCB_TX_USECS_LOW: c_int = 1;
pub const HNS_RCB_RX_USECS_LOW: c_int = 1;
pub const HNS_RCB_TX_USECS_HIGH: c_int = 1023;
pub const HNS_RCB_RX_USECS_HIGH: c_int = 1023;
pub const HNS_RCB_MAX_COALESCED_FRAMES: c_int = 1023;
pub const HNS_RCB_MIN_COALESCED_FRAMES: c_int = 1;
pub const HNS_RCB_DEF_RX_COALESCED_FRAMES: c_int = 50;
pub const HNS_RCB_DEF_TX_COALESCED_FRAMES: c_int = 1;
pub const HNS_RCB_CLK_FREQ_MHZ: c_int = 350;
pub const HNS_RCB_MAX_COALESCED_USECS: c_uint = 0x3ff;
pub const HNS_RCB_DEF_COALESCED_USECS: c_int = 30;
pub const HNS_RCB_DEF_GAP_TIME_USECS: c_int = 20;
pub const HNS_RCB_TX_PKTLINE_OFFSET: c_int = 8;
pub const HNS_RCB_COMMON_ENDIAN: c_int = 1;
pub const HNS_BD_SIZE_512_TYPE: c_int = 0;
pub const HNS_BD_SIZE_1024_TYPE: c_int = 1;
pub const HNS_BD_SIZE_2048_TYPE: c_int = 2;
pub const HNS_BD_SIZE_4096_TYPE: c_int = 3;
pub const HNS_RCB_COMMON_DUMP_REG_NUM: c_int = 80;
pub const HNS_RCB_RING_DUMP_REG_NUM: c_int = 40;
pub const HNS_RING_STATIC_REG_NUM: c_int = 28;
pub const HNS_DUMP_REG_NUM: c_int = 500;
pub const HNS_STATIC_REG_NUM: c_int = 12;
pub const HNS_TSO_MODE_8BD_32K: c_int = 1;
pub const HNS_TSO_MDOE_4BD_16K: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rcb_int_flag {
    RCB_INT_FLAG_TX = 0x1,
    RCB_INT_FLAG_RX = (0x1 << 1),
    RCB_INT_FLAG_MAX = (0x1 << 2),	/*must be the last element */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_ring_hw_stats {
    pub tx_pkts: u64,
    pub ppe_tx_ok_pkts: u64,
    pub ppe_tx_drop_pkts: u64,
    pub rx_pkts: u64,
    pub ppe_rx_ok_pkts: u64,
    pub ppe_rx_drop_pkts: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ring_pair_cb {
    pub /: *mut *mut *mut rcb_common_cb rcb_common; / ring belongs to,
    pub /: *mut *mut *mut device dev; /device for DMA mapping,
    pub q: hnae_queue,
    pub /: *mut *mut u16 index; / global index in a rcb common device,
    pub buf_size: u16,
    pub virq: [c_int; HNS_RCB_IRQ_NUM_PER_QUEUE],
    pub port_id_in_comm: u8,
    pub used_by_vf: u8,
    pub hw_stats: hns_ring_hw_stats,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rcb_common_cb {
    pub io_base: *mut u8 __iomem,
    pub phy_base: phys_addr_t,
    pub dsaf_dev: *mut dsaf_device,
    pub max_vfn: u16,
    pub max_q_per_vf: u16,
    pub comm_index: u8,
    pub ring_num: u32,
    pub queue*/: *mut *mut u32 desc_num; / desc num per,
    pub __counted_by(ring_num): ring_pair_cb ring_pair_cb[],
}

extern "C" {
    pub fn hns_rcb_buf_size2type(buf_size: u32) -> c_int;
}
extern "C" {
    pub fn hns_rcb_common_get_cfg(dsaf_dev: *mut dsaf_device, comm_index: c_int) -> c_int;
}
extern "C" {
    pub fn hns_rcb_common_free_cfg(dsaf_dev: *mut dsaf_device, comm_index: u32);
}
extern "C" {
    pub fn hns_rcb_common_init_hw(rcb_common: *mut rcb_common_cb) -> c_int;
}
extern "C" {
    pub fn hns_rcb_get_cfg(rcb_common: *mut rcb_common_cb) -> c_int;
}
extern "C" {
    pub fn hns_rcb_common_init_commit_hw(rcb_common: *mut rcb_common_cb);
}
extern "C" {
    pub fn hns_rcb_ring_enable_hw(q: *mut hnae_queue, val: u32);
}
extern "C" {
    pub fn hns_rcb_int_clr_hw(q: *mut hnae_queue, flag: u32);
}
extern "C" {
    pub fn hns_rcb_int_ctrl_hw(q: *mut hnae_queue, flag: u32, enable: u32);
}
extern "C" {
    pub fn hns_rcbv2_int_ctrl_hw(q: *mut hnae_queue, flag: u32, mask: u32);
}
extern "C" {
    pub fn hns_rcbv2_int_clr_hw(q: *mut hnae_queue, flag: u32);
}
extern "C" {
    pub fn hns_rcb_init_hw(ring: *mut ring_pair_cb);
}
extern "C" {
    pub fn hns_rcb_reset_ring_hw(q: *mut hnae_queue);
}
extern "C" {
    pub fn hns_rcb_wait_fbd_clean(qs: *mut hnae_queue, q_num: c_int, flag: u32);
}
extern "C" {
    pub fn hns_rcb_wait_tx_ring_clean(qs: *mut hnae_queue) -> c_int;
}
extern "C" {
    pub fn hns_rcb_update_stats(queue: *mut hnae_queue);
}
extern "C" {
    pub fn hns_rcb_get_stats(queue: *mut hnae_queue, data: *mut u64);
}
extern "C" {
    pub fn hns_rcb_get_common_regs(rcb_common: *mut rcb_common_cb, data: *mut c_void);
}
extern "C" {
    pub fn hns_rcb_get_ring_sset_count(stringset: c_int) -> c_int;
}
extern "C" {
    pub fn hns_rcb_get_common_regs_count() -> c_int;
}
extern "C" {
    pub fn hns_rcb_get_ring_regs_count() -> c_int;
}
extern "C" {
    pub fn hns_rcb_get_ring_regs(queue: *mut hnae_queue, data: *mut c_void);
}
extern "C" {
    pub fn hns_rcb_get_strings(stringset: c_int, data: *mut u8, index: c_int);
}
extern "C" {
    pub fn hns_rcb_set_rx_ring_bs(q: *mut hnae_queue, buf_size: u32);
}
extern "C" {
    pub fn hns_rcb_set_tx_ring_bs(q: *mut hnae_queue, buf_size: u32);
}
