//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/psp-tee.h
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
// AMD Trusted Execution Environment (TEE) interface
//
// Author: Rijo Thomas <Rijo-john.Thomas@amd.com>
//
// Copyright 2019 Advanced Micro Devices, Inc.
//

// This file defines the Trusted Execution Environment (TEE) interface commands
// and the API exported by AMD Secure Processor driver to communicate with
// AMD-TEE Trusted OS.
//
// enum tee_cmd_id - TEE Interface Command IDs
// @TEE_CMD_ID_LOAD_TA:          Load Trusted Application (TA) binary into
// TEE environment
// @TEE_CMD_ID_UNLOAD_TA:        Unload TA binary from TEE environment
// @TEE_CMD_ID_OPEN_SESSION:     Open session with loaded TA
// @TEE_CMD_ID_CLOSE_SESSION:    Close session with loaded TA
// @TEE_CMD_ID_INVOKE_CMD:       Invoke a command with loaded TA
// @TEE_CMD_ID_MAP_SHARED_MEM:   Map shared memory
// @TEE_CMD_ID_UNMAP_SHARED_MEM: Unmap shared memory
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tee_cmd_id {
    TEE_CMD_ID_LOAD_TA = 1,
    TEE_CMD_ID_UNLOAD_TA,
    TEE_CMD_ID_OPEN_SESSION,
    TEE_CMD_ID_CLOSE_SESSION,
    TEE_CMD_ID_INVOKE_CMD,
    TEE_CMD_ID_MAP_SHARED_MEM,
    TEE_CMD_ID_UNMAP_SHARED_MEM,
}

//
// psp_tee_process_cmd() - Process command in Trusted Execution Environment
// @cmd_id:     TEE command ID (&enum tee_cmd_id)
// @buf:        Command buffer for TEE processing. On success, is updated
// with the response
// @len:        Length of command buffer in bytes
// @status:     On success, holds the TEE command execution status
//
// This function submits a command to the Trusted OS for processing in the
// TEE environment and waits for a response or until the command times out.
//
// Returns:
// 0 if TEE successfully processed the command
// -%ENODEV    if PSP device not available
// -%EINVAL    if invalid input
// -%ETIMEDOUT if TEE command timed out
// -%EBUSY     if PSP device is not responsive
//
// psp_check_tee_status() - Checks whether there is a TEE which a driver can
// talk to.
//
// This function can be used by AMD-TEE driver to query if there is TEE with
// which it can communicate.
//
// Returns:
// 0          if the device has TEE
// -%ENODEV   if there is no TEE available
//
extern "C" {
    pub fn psp_check_tee_status() -> c_int;
}

