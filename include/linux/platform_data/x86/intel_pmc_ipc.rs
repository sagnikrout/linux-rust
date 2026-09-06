//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/x86/intel_pmc_ipc.h
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
//
// Intel Core SoC Power Management Controller Header File
//
// Copyright (c) 2025, Intel Corporation.
// All Rights Reserved.
//

pub const IPC_SOC_REGISTER_ACCESS: c_uint = 0xAA;
pub const IPC_SOC_SUB_CMD_READ: c_uint = 0x00;
pub const IPC_SOC_SUB_CMD_WRITE: c_uint = 0x01;
pub const PMC_IPCS_PARAM_COUNT: c_int = 7;
pub const VALID_IPC_RESPONSE: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmc_ipc_cmd {
    pub cmd: u32,
    pub sub_cmd: u32,
    pub size: u32,
    pub wbuf: [u32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmc_ipc_rbuf {
    pub buf: [u32; 4],
}

//
// intel_pmc_ipc() - PMC IPC Mailbox accessor
// @ipc_cmd:  Prepared input command to send
// @rbuf:     Allocated array for returned IPC data
//
// Return: 0 on success. Non-zero on mailbox error
//

//
// 0: IPC Command
// 1: IPC Sub Command
// 2: Size
// 3-6: Write Buffer for offset
//

