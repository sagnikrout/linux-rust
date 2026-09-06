//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/hisilicon/hns3/hns3_common/hclge_comm_tqp_stats.h
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


// SPDX-License-Identifier: GPL-2.0+
// Copyright (c) 2021-2021 Hisilicon Limited.

// each tqp has TX & RX two queues
pub const HCLGE_COMM_QUEUE_PAIR_SIZE: c_int = 2;
// TQP stats
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_comm_tqp_stats {
// query_tqp_tx_queue_statistics ,opcode id:  0x0B03
    pub /: *mut *mut u64 rcb_tx_ring_pktnum_rcd; / 32bit,
// query_tqp_rx_queue_statistics ,opcode id:  0x0B13
    pub /: *mut *mut u64 rcb_rx_ring_pktnum_rcd; / 32bit,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_comm_tqp {
// copy of device pointer from pci_dev,
// used when perform DMA mapping
//
    pub dev: *mut device,
    pub q: hnae3_queue,
    pub tqp_stats: hclge_comm_tqp_stats,
    pub /: *mut *mut u16 index; / Global index in a NIC controller,
    pub alloced: bool,
}

extern "C" {
    pub fn hclge_comm_tqps_get_sset_count(handle: *mut hnae3_handle) -> c_int;
}
extern "C" {
    pub fn hclge_comm_tqps_get_strings(handle: *mut hnae3_handle, data: *mut u8);
}
extern "C" {
    pub fn hclge_comm_reset_tqp_stats(handle: *mut hnae3_handle);
}
