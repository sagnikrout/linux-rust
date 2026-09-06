//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/ice/ice_controlq.h
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
// Copyright (c) 2018, Intel Corporation.

// Maximum buffer lengths for all control queue types
pub const ICE_AQ_MAX_BUF_LEN: c_int = 4096;
pub const ICE_MBXQ_MAX_BUF_LEN: c_int = 4096;
pub const ICE_SBQ_MAX_BUF_LEN: c_int = 512;

// Defines that help manage the driver vs FW API checks.
// Take a look at ice_aq_ver_check in ice_controlq.c for actual usage.
//
pub const EXP_FW_API_VER_MAJOR_E810: c_uint = 0x01;
pub const EXP_FW_API_VER_MINOR_E810: c_uint = 0x05;
pub const EXP_FW_API_VER_MAJOR_E830: c_uint = 0x01;
pub const EXP_FW_API_VER_MINOR_E830: c_uint = 0x07;

// Different control queue types: These are mainly for SW consumption.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_ctl_q {
    ICE_CTL_Q_UNKNOWN = 0,
    ICE_CTL_Q_ADMIN,
    ICE_CTL_Q_MAILBOX,
    ICE_CTL_Q_SB,
}

// Control Queue timeout settings - max delay 1s

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_ctl_q_ring {
    pub /: *mut *mut *mut void dma_head; / Virtual address to DMA head,
    pub /: *mut *mut ice_dma_mem desc_buf; / descriptor ring memory,
    pub sq_bi: *mut ice_dma_mem,
    pub rq_bi: *mut ice_dma_mem,
    pub r: },
    pub /: *mut *mut u16 count; / Number of descriptors,
// used for interrupt processing
    pub next_to_use: u16,
    pub next_to_clean: u16,
// used for queue tracking
    pub head: u32,
    pub tail: u32,
    pub len: u32,
    pub bah: u32,
    pub bal: u32,
    pub len_mask: u32,
    pub len_ena_mask: u32,
    pub len_crit_mask: u32,
    pub head_mask: u32,
}

// sq transaction details
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_sq_cd {
    pub wb_desc: *mut libie_aq_desc,
}

// rq event information
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_rq_event_info {
    pub desc: libie_aq_desc,
    pub msg_len: u16,
    pub buf_len: u16,
    pub msg_buf: *mut u8,
}

// Control Queue information
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_ctl_q_info {
    pub qtype: ice_ctl_q,
    pub /: *mut *mut ice_ctl_q_ring rq; / receive queue,
    pub /: *mut *mut ice_ctl_q_ring sq; / send queue,
    pub /: *mut *mut u16 num_rq_entries; / receive queue depth,
    pub /: *mut *mut u16 num_sq_entries; / send queue depth,
    pub /: *mut *mut u16 rq_buf_size; / receive queue buffer size,
    pub /: *mut *mut u16 sq_buf_size; / send queue buffer size,
    pub /: *mut *mut libie_aq_err sq_last_status; / last status on send queue,
    pub /: *mut *mut mutex sq_lock; / Send queue lock,
    pub /: *mut *mut mutex rq_lock; / Receive queue lock,
}
