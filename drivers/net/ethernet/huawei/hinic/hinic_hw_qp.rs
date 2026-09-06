//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/huawei/hinic/hinic_hw_qp.h
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
//
// Huawei HiNIC PCI Express Linux driver
// Copyright(c) 2017 Huawei Technologies Co., Ltd
//

pub const HINIC_SQ_DB_INFO_PI_HI_SHIFT: c_int = 0;
pub const HINIC_SQ_DB_INFO_QID_SHIFT: c_int = 8;
pub const HINIC_SQ_DB_INFO_PATH_SHIFT: c_int = 23;
pub const HINIC_SQ_DB_INFO_COS_SHIFT: c_int = 24;
pub const HINIC_SQ_DB_INFO_TYPE_SHIFT: c_int = 27;
pub const HINIC_SQ_DB_INFO_PI_HI_MASK: c_uint = 0xFF;
pub const HINIC_SQ_DB_INFO_QID_MASK: c_uint = 0x3FF;
pub const HINIC_SQ_DB_INFO_PATH_MASK: c_uint = 0x1;
pub const HINIC_SQ_DB_INFO_COS_MASK: c_uint = 0x7;
pub const HINIC_SQ_DB_INFO_TYPE_MASK: c_uint = 0x1F;

pub const HINIC_SQ_WQEBB_SIZE: c_int = 64;
pub const HINIC_RQ_WQEBB_SIZE: c_int = 32;

pub const HINIC_MIN_QUEUE_DEPTH: c_int = 128;
// In any change to HINIC_RX_BUF_SZ, HINIC_RX_BUF_SZ_IDX must be changed
pub const HINIC_RX_BUF_SZ: c_int = 2048;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hinic_rx_buf_sz_idx {
    HINIC_RX_BUF_SZ_32_IDX,
    HINIC_RX_BUF_SZ_64_IDX,
    HINIC_RX_BUF_SZ_96_IDX,
    HINIC_RX_BUF_SZ_128_IDX,
    HINIC_RX_BUF_SZ_192_IDX,
    HINIC_RX_BUF_SZ_256_IDX,
    HINIC_RX_BUF_SZ_384_IDX,
    HINIC_RX_BUF_SZ_512_IDX,
    HINIC_RX_BUF_SZ_768_IDX,
    HINIC_RX_BUF_SZ_1024_IDX,
    HINIC_RX_BUF_SZ_1536_IDX,
    HINIC_RX_BUF_SZ_2048_IDX,
    HINIC_RX_BUF_SZ_3072_IDX,
    HINIC_RX_BUF_SZ_4096_IDX,
    HINIC_RX_BUF_SZ_8192_IDX,
    HINIC_RX_BUF_SZ_16384_IDX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_sq {
    pub hwif: *mut hinic_hwif,
    pub wq: *mut hinic_wq,
    pub qid: u16,
    pub irq: u32,
    pub msix_entry: u16,
    pub hw_ci_addr: *mut c_void,
    pub hw_ci_dma_addr: dma_addr_t,
    pub db_base: *mut void __iomem,
    pub saved_skb: *mut sk_buff,
    pub dbg: *mut hinic_debug_priv,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_rq {
    pub hwif: *mut hinic_hwif,
    pub wq: *mut hinic_wq,
    pub qid: u16,
    pub affinity_mask: cpumask,
    pub irq: u32,
    pub msix_entry: u16,
    pub buf_sz: usize,
    pub saved_skb: *mut sk_buff,
    pub cqe: *mut hinic_rq_cqe,
    pub cqe_dma: *mut dma_addr_t,
    pub pi_virt_addr: *mut u16,
    pub pi_dma_addr: dma_addr_t,
    pub dbg: *mut hinic_debug_priv,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_qp {
    pub sq: hinic_sq,
    pub rq: hinic_rq,
    pub q_id: u16,
}

extern "C" {
    pub fn hinic_clean_sq(sq: *mut hinic_sq);
}
extern "C" {
    pub fn hinic_clean_rq(rq: *mut hinic_rq);
}
extern "C" {
    pub fn hinic_get_sq_free_wqebbs(sq: *mut hinic_sq) -> c_int;
}
extern "C" {
    pub fn hinic_get_rq_free_wqebbs(rq: *mut hinic_rq) -> c_int;
}
extern "C" {
    pub fn hinic_task_set_l2hdr(task: *mut hinic_sq_task, len: u32);
}
extern "C" {
    pub fn hinic_sq_return_wqe(sq: *mut hinic_sq, wqe_size: c_uint);
}
extern "C" {
    pub fn hinic_sq_put_wqe(sq: *mut hinic_sq, wqe_size: c_uint);
}
extern "C" {
    pub fn hinic_rq_update(rq: *mut hinic_rq, prod_idx: u16);
}
