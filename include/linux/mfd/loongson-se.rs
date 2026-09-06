//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/loongson-se.h
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
// Copyright (C) 2025 Loongson Technology Corporation Limited
pub const LOONGSON_ENGINE_CMD_TIMEOUT_US: c_int = 10000;
pub const SE_SEND_CMD_REG: c_uint = 0x0;
pub const SE_SEND_CMD_REG_LEN: c_uint = 0x8;
// Controller command ID
pub const SE_CMD_START: c_uint = 0x0;
pub const SE_CMD_SET_DMA: c_uint = 0x3;
pub const SE_CMD_SET_ENGINE_CMDBUF: c_uint = 0x4;
pub const SE_S2LINT_STAT: c_uint = 0x88;
pub const SE_S2LINT_EN: c_uint = 0x8c;
pub const SE_S2LINT_CL: c_uint = 0x94;
pub const SE_L2SINT_STAT: c_uint = 0x98;
pub const SE_L2SINT_SET: c_uint = 0xa0;
pub const SE_INT_ALL: c_uint = 0xffffffff;

pub const SE_ENGINE_MAX: c_int = 16;
pub const SE_ENGINE_RNG: c_int = 1;
pub const SE_CMD_RNG: c_uint = 0x100;
pub const SE_ENGINE_TPM: c_int = 5;
pub const SE_CMD_TPM: c_uint = 0x500;
pub const SE_ENGINE_CMD_SIZE: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct loongson_se_engine {
    pub se: *mut loongson_se,
    pub id: c_int,
// Command buffer
    pub command: *mut c_void,
    pub command_ret: *mut c_void,
    pub data_buffer: *mut c_void,
    pub buffer_size: c_uint,
// Data buffer offset to DMA base
    pub buffer_off: c_uint,
    pub completion: completion,
}

extern "C" {
    pub fn loongson_se_send_engine_cmd(engine: *mut loongson_se_engine) -> c_int;
}
