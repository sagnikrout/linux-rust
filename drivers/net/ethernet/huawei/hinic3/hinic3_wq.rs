//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/huawei/hinic3/hinic3_wq.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic3_sq_bufdesc {
// 31-bits Length, L2NIC only uses length[17:0]
    pub len: __le32,
    pub rsvd: __le32,
    pub hi_addr: __le32,
    pub lo_addr: __le32,
}

// Work queue is used to submit elements (tx, rx, cmd) to hw.
// Driver is the producer that advances prod_idx. cons_idx is advanced when
// HW reports completions of previously submitted elements.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic3_wq {
    pub qpages: hinic3_queue_pages,
// Unmasked producer/consumer indices that are advanced to natural
// integer overflow regardless of queue depth.
//
    pub cons_idx: u16,
    pub prod_idx: u16,
    pub q_depth: u32,
    pub idx_mask: u16,
// Work Queue (logical WQEBB array) is mapped to hw via Chip Logical
// Address (CLA) using 1 of 2 levels:
// level 0 - direct mapping of single wq page
// level 1 - indirect mapping of multiple pages via additional page
// table.
// When wq uses level 1, wq_block will hold the allocated indirection
// table.
//
    pub wq_block_paddr: dma_addr_t,
    pub wq_block_vaddr: *mut __be64,
    pub ____cacheline_aligned: },
// Get number of elements in work queue that are in-use.
    pub READ_ONCE(wq->cons_idx): return READ_ONCE(wq->prod_idx) -,
// Don't allow queue to become completely full, report (free - 1).
    pub 1: return wq->q_depth - hinic3_wq_get_used(wq) -,
// pi = wq->prod_idx & wq->idx_mask;
    pub NULL): *mut *mut return get_q_element(&wq->qpages, pi,,
    pub num_wqebbs: wq->cons_idx +=,
    pub wq->qpages.pages[0].align_paddr: return,
    pub wqebb_size): u32 q_depth, u16,
    pub wq): *mut *mut void hinic3_wq_destroy(struct hinic3_hwdev hwdev, struct hinic3_wq,
    pub wq): *mut void hinic3_wq_reset(struct hinic3_wq,
    pub first_part_wqebbs_num): *mut u16,
    pub wq): *const bool hinic3_wq_is_0_level_cla(struct hinic3_wq,
