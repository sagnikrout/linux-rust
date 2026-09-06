//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/i40e/i40e_adminq.h
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
// Copyright(c) 2013 - 2018 Intel Corporation.

pub const I40E_ADMINQ_DESC_ALIGNMENT: c_int = 4096;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_adminq_ring {
    pub /: *mut *mut i40e_virt_mem dma_head; / space for dma structures,
    pub /: *mut *mut i40e_dma_mem desc_buf; / descriptor ring memory,
    pub /: *mut *mut i40e_virt_mem cmd_buf; / command buffer memory,
    pub asq_bi: *mut i40e_dma_mem,
    pub arq_bi: *mut i40e_dma_mem,
    pub r: },
    pub /: *mut *mut u16 count; / Number of descriptors,
    pub /: *mut *mut u16 rx_buf_len; / Admin Receive Queue buffer length,
// used for interrupt processing
    pub next_to_use: u16,
    pub next_to_clean: u16,
}

// ASQ transaction details
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_asq_cmd_details {
    pub /: *mut *mut *mut void callback; / cast from type I40E_ADMINQ_CALLBACK,
    pub cookie: u64,
    pub flags_ena: u16,
    pub flags_dis: u16,
    pub async: bool,
    pub postpone: bool,
    pub wb_desc: *mut libie_aq_desc,
}

// ARQ event information
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_arq_event_info {
    pub desc: libie_aq_desc,
    pub msg_len: u16,
    pub buf_len: u16,
    pub msg_buf: *mut u8,
}

// Admin Queue information
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_adminq_info {
    pub /: *mut *mut i40e_adminq_ring arq; / receive queue,
    pub /: *mut *mut i40e_adminq_ring asq; / send queue,
    pub timeout*/: *mut *mut u32 asq_cmd_timeout; / send queue cmd write back,
    pub /: *mut *mut u16 num_arq_entries; / receive queue depth,
    pub /: *mut *mut u16 num_asq_entries; / send queue depth,
    pub /: *mut *mut u16 arq_buf_size; / receive queue buffer size,
    pub /: *mut *mut u16 asq_buf_size; / send queue buffer size,
    pub /: *mut *mut u16 fw_maj_ver; / firmware major version,
    pub /: *mut *mut u16 fw_min_ver; / firmware minor version,
    pub /: *mut *mut u32 fw_build; / firmware build number,
    pub /: *mut *mut u16 api_maj_ver; / api major version,
    pub /: *mut *mut u16 api_min_ver; / api minor version,
    pub /: *mut *mut mutex asq_mutex; / Send queue lock,
    pub /: *mut *mut mutex arq_mutex; / Receive queue lock,
// last status values on send and receive queues
    pub asq_last_status: libie_aq_err,
    pub arq_last_status: libie_aq_err,
}

//
// i40e_aq_rc_to_posix - convert errors to user-land codes
// @aq_ret: AdminQ handler error code can override aq_rc
// @aq_rc: AdminQ firmware error code to convert
//
// general information
pub const I40E_AQ_LARGE_BUF: c_int = 512;

