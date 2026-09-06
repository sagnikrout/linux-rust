//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/ccp/tee-dev.h
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


// SPDX-License-Identifier: MIT
//
// Copyright (C) 2019,2021 Advanced Micro Devices, Inc.
//
// Author: Rijo Thomas <Rijo-john.Thomas@amd.com>
// Author: Devaraj Rangasamy <Devaraj.Rangasamy@amd.com>
//
// This file describes the TEE communication interface between host and AMD
// Secure Processor
//

pub const TEE_DEFAULT_RING_TIMEOUT: c_int = 10;
pub const MAX_BUFFER_SIZE: c_int = 988;
//
// struct tee_init_ring_cmd - Command to init TEE ring buffer
// @low_addr:  bits [31:0] of the physical address of ring buffer
// @hi_addr:   bits [63:32] of the physical address of ring buffer
// @size:      size of ring buffer in bytes
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tee_init_ring_cmd {
    pub low_addr: u32,
    pub hi_addr: u32,
    pub size: u32,
}

pub const MAX_RING_BUFFER_ENTRIES: c_int = 32;
//
// struct ring_buf_manager - Helper structure to manage ring buffer.
// @ring_start:  starting address of ring buffer
// @ring_size:   size of ring buffer in bytes
// @ring_pa:     physical address of ring buffer
// @wptr:        index to the last written entry in ring buffer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ring_buf_manager {
    pub /: *mut *mut mutex mutex; / synchronizes access to ring buffer,
    pub ring_start: *mut c_void,
    pub ring_size: u32,
    pub ring_pa: phys_addr_t,
    pub wptr: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct psp_tee_device {
    pub dev: *mut device,
    pub psp: *mut psp_device,
    pub io_regs: *mut void __iomem,
    pub vdata: *mut tee_vdata,
    pub rb_mgr: ring_buf_manager,
}

//
// enum tee_cmd_state - TEE command states for the ring buffer interface
// @TEE_CMD_STATE_INIT:      initial state of command when sent from host
// @TEE_CMD_STATE_PROCESS:   command being processed by TEE environment
// @TEE_CMD_STATE_COMPLETED: command processing completed
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tee_cmd_state {
    TEE_CMD_STATE_INIT,
    TEE_CMD_STATE_PROCESS,
    TEE_CMD_STATE_COMPLETED,
}

//
// enum cmd_resp_state - TEE command's response status maintained by driver
// @CMD_RESPONSE_INVALID:      initial state when no command is written to ring
// @CMD_WAITING_FOR_RESPONSE:  driver waiting for response from TEE
// @CMD_RESPONSE_TIMEDOUT:     failed to get response from TEE
// @CMD_RESPONSE_COPIED:       driver has copied response from TEE
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cmd_resp_state {
    CMD_RESPONSE_INVALID,
    CMD_WAITING_FOR_RESPONSE,
    CMD_RESPONSE_TIMEDOUT,
    CMD_RESPONSE_COPIED,
}

//
// struct tee_ring_cmd - Structure of the command buffer in TEE ring
// @cmd_id:      refers to &enum tee_cmd_id. Command id for the ring buffer
// interface
// @cmd_state:   refers to &enum tee_cmd_state
// @status:      status of TEE command execution
// @res0:        reserved region
// @pdata:       private data (currently unused)
// @res1:        reserved region
// @buf:         TEE command specific buffer
// @flag:	 refers to &enum cmd_resp_state
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tee_ring_cmd {
    pub cmd_id: u32,
    pub cmd_state: u32,
    pub status: u32,
    pub res0: [u32; 1],
    pub pdata: u64,
    pub res1: [u32; 2],
    pub buf: [u8; MAX_BUFFER_SIZE],
    pub flag: u32,
// Total size: 1024 bytes
    pub __packed: },
    pub psp): *mut int tee_dev_init(struct psp_device,
    pub psp): *mut void tee_dev_destroy(struct psp_device,
    pub psp): *mut int tee_restore(struct psp_device,
