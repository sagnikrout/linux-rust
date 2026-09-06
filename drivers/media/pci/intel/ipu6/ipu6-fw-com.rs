//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/intel/ipu6/ipu6-fw-com.h
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
// Copyright (C) 2013--2024 Intel Corporation
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipu6_fw_syscom_queue_config {
    pub /: *mut *mut unsigned int queue_size; / tokens per queue,
    pub /: *mut *mut unsigned int token_size; / bytes per token,
}

pub const SYSCOM_BUTTRESS_FW_PARAMS_ISYS_OFFSET: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipu6_fw_com_cfg {
    pub num_input_queues: c_uint,
    pub num_output_queues: c_uint,
    pub input: *mut ipu6_fw_syscom_queue_config,
    pub output: *mut ipu6_fw_syscom_queue_config,
    pub dmem_addr: c_uint,
// firmware-specific configuration data
    pub specific_addr: *mut c_void,
    pub specific_size: c_uint,
    pub adev): *mut *mut int (cell_ready)(struct ipu6_bus_device,
    pub adev): *mut *mut void (cell_start)(struct ipu6_bus_device,
    pub buttress_boot_offset: c_uint,
}

extern "C" {
    pub fn ipu6_fw_com_open(ctx: *mut ipu6_fw_com_context) -> c_int;
}
extern "C" {
    pub fn ipu6_fw_com_ready(ctx: *mut ipu6_fw_com_context) -> bool;
}
extern "C" {
    pub fn ipu6_fw_com_close(ctx: *mut ipu6_fw_com_context) -> c_int;
}
extern "C" {
    pub fn ipu6_fw_com_release(ctx: *mut ipu6_fw_com_context, force: c_uint) -> c_int;
}
extern "C" {
    pub fn ipu6_recv_put_token(ctx: *mut ipu6_fw_com_context, q_nbr: c_int);
}
extern "C" {
    pub fn ipu6_send_put_token(ctx: *mut ipu6_fw_com_context, q_nbr: c_int);
}
