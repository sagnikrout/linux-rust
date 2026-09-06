//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/huawei/hinic/hinic_hw_wq.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_free_block {
    pub page_idx: c_int,
    pub block_idx: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_wq {
    pub hwif: *mut hinic_hwif,
    pub page_idx: c_int,
    pub block_idx: c_int,
    pub wqebb_size: u16,
    pub wq_page_size: u32,
    pub q_depth: u16,
    pub max_wqe_size: u16,
    pub num_wqebbs_per_page: u16,
    pub wqebbs_per_page_shift: u16,
    pub wqebb_size_shift: u16,
// The addresses are 64 bit in the HW
    pub block_paddr: u64,
    pub shadow_block_vaddr: *mut c_void,
    pub block_vaddr: *mut u64,
    pub num_q_pages: c_int,
    pub shadow_wqe: *mut u8,
    pub shadow_idx: *mut u16,
    pub cons_idx: core::sync::atomic::AtomicI32,
    pub prod_idx: core::sync::atomic::AtomicI32,
    pub delta: core::sync::atomic::AtomicI32,
    pub mask: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_wqs {
    pub hwif: *mut hinic_hwif,
    pub num_pages: c_int,
// The addresses are 64 bit in the HW
    pub page_paddr: *mut u64,
    pub page_vaddr: *mut u64,
    pub shadow_page_vaddr: *mut c_void,
    pub free_blocks: *mut hinic_free_block,
    pub alloc_blk_pos: c_int,
    pub return_blk_pos: c_int,
    pub num_free_blks: c_int,
// Lock for getting a free block from the WQ set
    pub alloc_blocks_lock: semaphore,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_cmdq_pages {
// The addresses are 64 bit in the HW
    pub page_paddr: u64,
    pub page_vaddr: *mut u64,
    pub shadow_page_vaddr: *mut c_void,
    pub hwif: *mut hinic_hwif,
}

extern "C" {
    pub fn hinic_wqs_free(wqs: *mut hinic_wqs);
}
extern "C" {
    pub fn hinic_wq_free(wqs: *mut hinic_wqs, wq: *mut hinic_wq);
}
extern "C" {
    pub fn hinic_return_wqe(wq: *mut hinic_wq, wqe_size: c_uint);
}
extern "C" {
    pub fn hinic_put_wqe(wq: *mut hinic_wq, wqe_size: c_uint);
}
