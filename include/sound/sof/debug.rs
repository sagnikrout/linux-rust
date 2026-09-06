//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/sof/debug.h
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
// Copyright(c) 2020 Intel Corporation
//
// Author: Karol Trzcinski <karolx.trzcinski@linux.intel.com>
//

// ABI3.18
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sof_ipc_dbg_mem_zone {
    SOF_IPC_MEM_ZONE_SYS		= 0,	/**< System zone */
    SOF_IPC_MEM_ZONE_SYS_RUNTIME	= 1,	/**< System-runtime zone */
    SOF_IPC_MEM_ZONE_RUNTIME	= 2,	/**< Runtime zone */
    SOF_IPC_MEM_ZONE_BUFFER		= 3,	/**< Buffer zone */
    SOF_IPC_MEM_ZONE_RUNTIME_SHARED	= 4,	/**< System runtime zone */
    SOF_IPC_MEM_ZONE_SYS_SHARED	= 5,	/**< System shared zone */
}

// ABI3.18
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc_dbg_mem_usage_elem {
    pub /: *mut *mut *mut uint32_t zone; /< see sof_ipc_dbg_mem_zone,
    pub /: *mut *mut *mut uint32_t id; /< heap index within zone,
    pub /: *mut *mut *mut uint32_t used; /< number of bytes used in zone,
    pub /: *mut *mut *mut uint32_t free; /< number of bytes free to use within zone,
    pub /: *mut *mut *mut uint32_t reserved; /< for future use,
    pub __packed: },
// ABI3.18
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc_dbg_mem_usage {
    pub /: *mut *mut *mut sof_ipc_reply rhdr; /< generic IPC reply header,
    pub /: *mut *mut *mut uint32_t reserved[4]; /< reserved for future use,
    pub /: *mut *mut *mut uint32_t num_elems; /< elems[] counter,
    pub /: *mut *mut *mut sof_ipc_dbg_mem_usage_elem elems[]; /< memory usage information,
    pub __packed: },
