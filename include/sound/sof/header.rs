//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/sof/header.h
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
//
// This file is provided under a dual BSD/GPLv2 license.  When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2018 Intel Corporation
//

// \addtogroup sof_uapi uAPI
// SOF uAPI specification.
// @{
//
// IPC messages have a prefixed 32 bit identifier made up as follows :-
//
// 0xGCCCNNNN where
// G is global cmd type (4 bits)
// C is command type (12 bits)
// I is the ID number (16 bits) - monotonic and overflows
//
// This is sent at the start of the IPM message in the mailbox. Messages should
// not be sent in the doorbell (special exceptions for firmware .
//
// Global Message - Generic
pub const SOF_GLB_TYPE_SHIFT: c_int = 28;

// Command Message - Generic
pub const SOF_CMD_TYPE_SHIFT: c_int = 16;

// Global Message Types

//
// DSP Command Message Types
//
// topology

// PM

// component runtime config - multiple different types

// DAI messages

// stream

// probe

// trace

// debug

// test

// Get message component id

// maximum message size for mailbox Tx/Rx
pub const SOF_IPC_MSG_MAX_SIZE: c_int = 384;
//
// Structure Header - Header for all IPC structures except command structs.
// The size can be greater than the structure size and that means there is
// extended bespoke data beyond the end of the structure including variable
// arrays.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc_hdr {
    pub /: *mut *mut *mut uint32_t size; /< size of structure,
    pub __packed: },
//
// Command Header - Header for all IPC commands. Identifies IPC message.
// The size can be greater than the structure size and that means there is
// extended bespoke data beyond the end of the structure including variable
// arrays.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc_cmd_hdr {
    pub /: *mut *mut *mut uint32_t size; /< size of structure,
    pub /: *mut *mut *mut uint32_t cmd; /< SOF_IPC_GLB_ + cmd,
    pub __packed: },
//
// Generic reply message. Some commands override this with their own reply
// types that must include this at start.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc_reply {
    pub hdr: sof_ipc_cmd_hdr,
    pub /: *mut *mut *mut int32_t error; /< negative error numbers,
    pub __packed: },
//
// Compound commands - SOF_IPC_GLB_COMPOUND.
//
// Compound commands are sent to the DSP as a single IPC operation. The
// commands are split into blocks and each block has a header. This header
// identifies the command type and the number of commands before the next
// header.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc_compound_hdr {
    pub hdr: sof_ipc_cmd_hdr,
    pub /: *mut *mut *mut uint32_t count; /< count of 0 means end of compound sequence,
    pub __packed: },
//
// OOPS header architecture specific data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc_dsp_oops_arch_hdr {
    pub /: *mut *mut uint32_t arch; / Identifier of architecture,
    pub /: *mut *mut uint32_t totalsize; / Total size of oops message,
    pub __packed: },
//
// OOPS header platform specific data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc_dsp_oops_plat_hdr {
    pub /: *mut *mut uint32_t configidhi; / ConfigID hi 32bits,
    pub /: *mut *mut uint32_t configidlo; / ConfigID lo 32bits,
    pub /: *mut *mut uint32_t numaregs; / Special regs num,
    pub of: *mut *mut uint32_t stackoffset; / Offset to stack pointer from beginning,
// oops message
//
    pub /: *mut *mut uint32_t stackptr; / Stack ptr,
    pub __packed: },
// @}
