//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/amd/isp4/isp4_interface.h
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
//
// Copyright (C) 2025 Advanced Micro Devices, Inc.
//

pub const ISP4IF_RB_MAX: c_int = 25;
pub const ISP4IF_RESP_CHAN_TO_RB_OFFSET: c_int = 9;

pub const ISP4IF_MAX_NUM_HOST2FW_COMMAND: c_int = 40;

pub const ISP4IF_MAX_STREAM_BUF_COUNT: c_int = 8;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum isp4if_stream_id {
    ISP4IF_STREAM_ID_GLOBAL = 0,
    ISP4IF_STREAM_ID_1 = 1,
    ISP4IF_STREAM_ID_MAX = 4
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum isp4if_status {
    ISP4IF_STATUS_PWR_OFF,
    ISP4IF_STATUS_PWR_ON,
    ISP4IF_STATUS_FW_RUNNING,
    ISP4IF_FSM_STATUS_MAX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct isp4if_gpu_mem_info {
    pub mem_size: u64,
    pub gpu_mc_addr: u64,
    pub sys_addr: *mut c_void,
    pub mem_handle: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct isp4if_img_buf_info {
    pub sys_addr: *mut c_void,
    pub mc_addr: u64,
    pub len: u32,
    pub planes: [}; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct isp4if_img_buf_node {
    pub node: list_head,
    pub buf_info: isp4if_img_buf_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct isp4if_cmd_element {
    pub list: list_head,
    pub seq_num: u32,
    pub cmd_id: u32,
    pub cmd_done: completion,
    pub refcnt: refcount_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct isp4_interface {
    pub dev: *mut device,
    pub mmio: *mut void __iomem,
    pub /: *mut *mut spinlock_t cmdq_lock; / used for cmdq access,
    pub /: *mut *mut spinlock_t bufq_lock; / used for bufq access,
    pub /: *mut *mut mutex isp4if_mutex; / used to send fw cmd and read fw log,
    pub /: *mut *mut list_head cmdq; / commands sent to fw,
    pub /: *mut *mut list_head bufq; / buffers sent to fw,
    pub status: isp4if_status,
    pub host2fw_seq_num: u32,
// ISP fw buffers
    pub fw_log_buf: *mut isp4if_gpu_mem_info,
    pub fw_cmd_resp_buf: *mut isp4if_gpu_mem_info,
    pub fw_mem_pool: *mut isp4if_gpu_mem_info,
    pub meta_info_buf: [*mut isp4if_gpu_mem_info; ISP4IF_MAX_STREAM_BUF_COUNT],
}

// lo = addr & 0xffffffff;
// hi = addr >> 32;
extern "C" {
    pub fn isp4if_clear_cmdq(ispif: *mut isp4_interface);
}
extern "C" {
    pub fn isp4if_clear_bufq(ispif: *mut isp4_interface);
}
extern "C" {
    pub fn isp4if_dealloc_buffer_node(buf_node: *mut isp4if_img_buf_node);
}
extern "C" {
    pub fn isp4if_stop(ispif: *mut isp4_interface) -> c_int;
}
extern "C" {
    pub fn isp4if_start(ispif: *mut isp4_interface) -> c_int;
}
extern "C" {
    pub fn isp4if_deinit(ispif: *mut isp4_interface) -> c_int;
}
