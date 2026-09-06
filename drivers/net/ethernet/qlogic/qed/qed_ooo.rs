//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/qlogic/qed/qed_ooo.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
// QLogic qed NIC Driver
// Copyright (c) 2015-2017  QLogic Corporation
// Copyright (c) 2019-2020 Marvell International Ltd.
//

pub const QED_MAX_NUM_ISLES: c_int = 256;
pub const QED_MAX_NUM_OOO_HISTORY_ENTRIES: c_int = 512;
pub const QED_OOO_LEFT_BUF: c_int = 0;
pub const QED_OOO_RIGHT_BUF: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_ooo_buffer {
    pub list_entry: list_head,
    pub rx_buffer_virt_addr: *mut c_void,
    pub rx_buffer_phys_addr: dma_addr_t,
    pub rx_buffer_size: u32,
    pub packet_length: u16,
    pub parse_flags: u16,
    pub vlan: u16,
    pub placement_offset: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_ooo_isle {
    pub list_entry: list_head,
    pub buffers_list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_ooo_archipelago {
    pub isles_list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_ooo_history {
    pub p_cqes: *mut ooo_opaque,
    pub head_idx: u32,
    pub num_of_cqes: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_ooo_info {
    pub free_buffers_list: list_head,
    pub ready_buffers_list: list_head,
    pub free_isles_list: list_head,
    pub p_archipelagos_mem: *mut qed_ooo_archipelago,
    pub p_isles_mem: *mut qed_ooo_isle,
    pub ooo_history: qed_ooo_history,
    pub cur_isles_number: u32,
    pub max_isles_number: u32,
    pub gen_isles_number: u32,
    pub max_num_archipelagos: u16,
    pub cid_base: u16,
}

extern "C" {
    pub fn qed_ooo_alloc(p_hwfn: *mut qed_hwfn) -> c_int;
}
extern "C" {
    pub fn qed_ooo_setup(p_hwfn: *mut qed_hwfn);
}
extern "C" {
    pub fn qed_ooo_free(p_hwfn: *mut qed_hwfn);
}

