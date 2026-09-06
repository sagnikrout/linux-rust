//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/huawei/hinic3/hinic3_nic_io.h
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
// Copyright (c) Huawei Technologies Co., Ltd. 2025. All rights reserved.

pub const HINIC3_SQ_WQEBB_SHIFT: c_int = 4;
pub const HINIC3_RQ_WQEBB_SHIFT: c_int = 3;

// ******************** RQ_CTRL ********************
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hinic3_rq_wqe_type {
    HINIC3_NORMAL_RQ_WQE = 1,
}

// ******************** SQ_CTRL ********************
pub const HINIC3_TX_MSS_DEFAULT: c_uint = 0x3E00;
pub const HINIC3_TX_MSS_MIN: c_uint = 0x50;
pub const HINIC3_MAX_SQ_SGE: c_int = 18;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic3_io_queue {
    pub wq: hinic3_wq,
    pub owner: u8,
    pub q_id: u16,
    pub msix_entry_idx: u16,
    pub db_addr: *mut u8 __iomem,
    pub cons_idx_addr: *mut u16,
    pub ____cacheline_aligned: },
    pub &sq->wq: *const *const hinic3_wq wq =,
    pub wq->idx_mask: return wq->cons_idx &,
    pub &sq->wq: *const *const hinic3_wq wq =,
    pub wq->idx_mask: return wq->prod_idx &,
    pub &sq->wq: *const *const hinic3_wq wq =,
    pub wq->idx_mask: *mut *mut return READ_ONCE(sq->cons_idx_addr) &,
// ******************** DB INFO ********************

pub const DB_PI_LOW_MASK: c_uint = 0xFFU;
pub const DB_PI_HIGH_MASK: c_uint = 0xFFU;
pub const DB_PI_HI_SHIFT: c_int = 8;

pub const DB_SRC_TYPE: c_int = 1;
// CFLAG_DATA_PATH
pub const DB_CFLAG_DP_SQ: c_int = 0;
pub const DB_CFLAG_DP_RQ: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic3_nic_db {
    pub db_info: __le32,
    pub pi_hi: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic3_dyna_qp_params {
    pub num_qps: u16,
    pub sq_depth: u32,
    pub rq_depth: u32,
    pub sqs: *mut hinic3_io_queue,
    pub rqs: *mut hinic3_io_queue,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic3_nic_io {
    pub sq: *mut hinic3_io_queue,
    pub rq: *mut hinic3_io_queue,
    pub num_qps: u16,
    pub max_qps: u16,
// Base address for consumer index of all tx queues. Each queue is
// given a full cache line to hold its consumer index. HW updates
// current consumer index as it consumes tx WQEs.
//
    pub ci_vaddr_base: *mut c_void,
    pub ci_dma_base: dma_addr_t,
    pub sqs_db_addr: *mut u8 __iomem,
    pub rqs_db_addr: *mut u8 __iomem,
    pub rx_buf_len: u16,
    pub feature_cap: u64,
}

extern "C" {
    pub fn hinic3_init_nic_io(nic_dev: *mut hinic3_nic_dev) -> c_int;
}
extern "C" {
    pub fn hinic3_free_nic_io(nic_dev: *mut hinic3_nic_dev);
}
extern "C" {
    pub fn hinic3_init_nicio_res(nic_dev: *mut hinic3_nic_dev) -> c_int;
}
extern "C" {
    pub fn hinic3_free_nicio_res(nic_dev: *mut hinic3_nic_dev);
}
extern "C" {
    pub fn hinic3_init_qp_ctxts(nic_dev: *mut hinic3_nic_dev) -> c_int;
}
extern "C" {
    pub fn hinic3_free_qp_ctxts(nic_dev: *mut hinic3_nic_dev);
}
