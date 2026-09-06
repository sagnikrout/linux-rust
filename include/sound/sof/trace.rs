//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/sof/trace.h
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

//
// DMA for Trace
//
pub const SOF_TRACE_FILENAME_SIZE: c_int = 32;
// DMA for Trace params info - SOF_IPC_DEBUG_DMA_PARAMS
// Deprecated - use sof_ipc_dma_trace_params_ext
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc_dma_trace_params {
    pub hdr: sof_ipc_cmd_hdr,
    pub buffer: sof_ipc_host_buffer,
    pub stream_tag: u32,
    pub __packed: },
// DMA for Trace params info - SOF_IPC_DEBUG_DMA_PARAMS_EXT
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc_dma_trace_params_ext {
    pub hdr: sof_ipc_cmd_hdr,
    pub buffer: sof_ipc_host_buffer,
    pub stream_tag: u32,
    pub /: *mut *mut uint64_t timestamp_ns; / in nanosecond,
    pub reserved: [u32; 8],
    pub __packed: },
// DMA for Trace params info - SOF_IPC_DEBUG_DMA_PARAMS
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc_dma_trace_posn {
    pub rhdr: sof_ipc_reply,
    pub /: *mut *mut uint32_t host_offset; / Offset of DMA host buffer,
    pub /: *mut *mut uint32_t overflow; / overflow bytes if any,
    pub /: *mut *mut uint32_t messages; / total trace messages,
    pub __packed: },
// Values used in sof_ipc_trace_filter_elem:
// bits 6..0
pub const SOF_IPC_TRACE_FILTER_ELEM_SET_LEVEL: c_uint = 0x01	/**< trace level for selected components */;
pub const SOF_IPC_TRACE_FILTER_ELEM_BY_UUID: c_uint = 0x02	/**< filter by uuid key */;
pub const SOF_IPC_TRACE_FILTER_ELEM_BY_PIPE: c_uint = 0x03	/**< filter by pipeline */;
pub const SOF_IPC_TRACE_FILTER_ELEM_BY_COMP: c_uint = 0x04	/**< filter by component id */;
// bit 7
pub const SOF_IPC_TRACE_FILTER_ELEM_FIN: c_uint = 0x80	/**< mark last filter in set */;
// bits 31..8: Unused
// part of sof_ipc_trace_filter, ABI3.17
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc_trace_filter_elem {
    pub /: *mut *mut *mut uint32_t key; /< SOF_IPC_TRACE_FILTER_ELEM_ {LEVEL, UUID, COMP, PIPE},
    pub /: *mut *mut *mut uint32_t value; /< element value,
    pub __packed: },
// Runtime tracing filtration data - SOF_IPC_TRACE_FILTER_UPDATE, ABI3.17
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc_trace_filter {
    pub /: *mut *mut *mut sof_ipc_cmd_hdr hdr; /< IPC command header,
    pub /: *mut *mut *mut uint32_t elem_cnt; /< number of entries in elems[] array,
    pub /: *mut *mut *mut uint32_t reserved[8]; /< reserved for future usage,
// variable size array with new filtering settings
    pub elems: [sof_ipc_trace_filter_elem; ],
    pub __packed: },
//
// Commom debug
//
// SOF panic codes
//
pub const SOF_IPC_PANIC_MAGIC: c_uint = 0x0dead000;
pub const SOF_IPC_PANIC_MAGIC_MASK: c_uint = 0x0ffff000;
pub const SOF_IPC_PANIC_CODE_MASK: c_uint = 0x00000fff;

// panic info include filename and line number
// filename array will not include null terminator if fully filled
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc_panic_info {
    pub hdr: sof_ipc_hdr,
    pub /: *mut *mut uint32_t code; / SOF_IPC_PANIC_,
    pub filename: [u8; SOF_TRACE_FILENAME_SIZE],
    pub linenum: u32,
    pub __packed: },
