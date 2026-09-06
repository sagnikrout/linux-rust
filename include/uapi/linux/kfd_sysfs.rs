//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/kfd_sysfs.h
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


// SPDX-License-Identifier: (GPL-2.0 WITH Linux-syscall-note) OR MIT
//
// Copyright 2021 Advanced Micro Devices, Inc.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
//

// Macro flag: #define KFD_SYSFS_H_INCLUDED
// Capability bits in node properties
pub const HSA_CAP_HOT_PLUGGABLE: c_uint = 0x00000001;
pub const HSA_CAP_ATS_PRESENT: c_uint = 0x00000002;
pub const HSA_CAP_SHARED_WITH_GRAPHICS: c_uint = 0x00000004;
pub const HSA_CAP_QUEUE_SIZE_POW2: c_uint = 0x00000008;
pub const HSA_CAP_QUEUE_SIZE_32BIT: c_uint = 0x00000010;
pub const HSA_CAP_QUEUE_IDLE_EVENT: c_uint = 0x00000020;
pub const HSA_CAP_VA_LIMIT: c_uint = 0x00000040;
pub const HSA_CAP_WATCH_POINTS_SUPPORTED: c_uint = 0x00000080;
pub const HSA_CAP_WATCH_POINTS_TOTALBITS_MASK: c_uint = 0x00000f00;
pub const HSA_CAP_WATCH_POINTS_TOTALBITS_SHIFT: c_int = 8;
pub const HSA_CAP_DOORBELL_TYPE_TOTALBITS_MASK: c_uint = 0x00003000;
pub const HSA_CAP_DOORBELL_TYPE_TOTALBITS_SHIFT: c_int = 12;
pub const HSA_CAP_DOORBELL_TYPE_PRE_1_0: c_uint = 0x0;
pub const HSA_CAP_DOORBELL_TYPE_1_0: c_uint = 0x1;
pub const HSA_CAP_DOORBELL_TYPE_2_0: c_uint = 0x2;
pub const HSA_CAP_AQL_QUEUE_DOUBLE_MAP: c_uint = 0x00004000;
pub const HSA_CAP_TRAP_DEBUG_SUPPORT: c_uint = 0x00008000;
pub const HSA_CAP_TRAP_DEBUG_WAVE_LAUNCH_TRAP_OVERRIDE_SUPPORTED: c_uint = 0x00010000;
pub const HSA_CAP_TRAP_DEBUG_WAVE_LAUNCH_MODE_SUPPORTED: c_uint = 0x00020000;
pub const HSA_CAP_TRAP_DEBUG_PRECISE_MEMORY_OPERATIONS_SUPPORTED: c_uint = 0x00040000;
// Old buggy user mode depends on this being 0
pub const HSA_CAP_RESERVED_WAS_SRAM_EDCSUPPORTED: c_uint = 0x00080000;
pub const HSA_CAP_MEM_EDCSUPPORTED: c_uint = 0x00100000;
pub const HSA_CAP_RASEVENTNOTIFY: c_uint = 0x00200000;
pub const HSA_CAP_ASIC_REVISION_MASK: c_uint = 0x03c00000;
pub const HSA_CAP_ASIC_REVISION_SHIFT: c_int = 22;
pub const HSA_CAP_SRAM_EDCSUPPORTED: c_uint = 0x04000000;
pub const HSA_CAP_SVMAPI_SUPPORTED: c_uint = 0x08000000;
pub const HSA_CAP_FLAGS_COHERENTHOSTACCESS: c_uint = 0x10000000;
pub const HSA_CAP_TRAP_DEBUG_FIRMWARE_SUPPORTED: c_uint = 0x20000000;
pub const HSA_CAP_TRAP_DEBUG_PRECISE_ALU_OPERATIONS_SUPPORTED: c_uint = 0x40000000;
pub const HSA_CAP_PER_QUEUE_RESET_SUPPORTED: c_uint = 0x80000000;
pub const HSA_CAP_RESERVED: c_uint = 0x000f8000;
pub const HSA_CAP2_PER_SDMA_QUEUE_RESET_SUPPORTED: c_uint = 0x00000001;
pub const HSA_CAP2_TRAP_DEBUG_LDS_OUT_OF_ADDR_RANGE_SUPPORTED: c_uint = 0x00000002;
pub const HSA_CAP2_RESERVED: c_uint = 0xfffffffc;
// debug_prop bits in node properties
pub const HSA_DBG_WATCH_ADDR_MASK_LO_BIT_MASK: c_uint = 0x0000000f;
pub const HSA_DBG_WATCH_ADDR_MASK_LO_BIT_SHIFT: c_int = 0;
pub const HSA_DBG_WATCH_ADDR_MASK_HI_BIT_MASK: c_uint = 0x000003f0;
pub const HSA_DBG_WATCH_ADDR_MASK_HI_BIT_SHIFT: c_int = 4;
pub const HSA_DBG_DISPATCH_INFO_ALWAYS_VALID: c_uint = 0x00000400;
pub const HSA_DBG_WATCHPOINTS_EXCLUSIVE: c_uint = 0x00000800;
pub const HSA_DBG_RESERVED: c_uint = 0xfffffffffffff000ull;
// Heap types in memory properties
pub const HSA_MEM_HEAP_TYPE_SYSTEM: c_int = 0;
pub const HSA_MEM_HEAP_TYPE_FB_PUBLIC: c_int = 1;
pub const HSA_MEM_HEAP_TYPE_FB_PRIVATE: c_int = 2;
pub const HSA_MEM_HEAP_TYPE_GPU_GDS: c_int = 3;
pub const HSA_MEM_HEAP_TYPE_GPU_LDS: c_int = 4;
pub const HSA_MEM_HEAP_TYPE_GPU_SCRATCH: c_int = 5;
// Flag bits in memory properties
pub const HSA_MEM_FLAGS_HOT_PLUGGABLE: c_uint = 0x00000001;
pub const HSA_MEM_FLAGS_NON_VOLATILE: c_uint = 0x00000002;
pub const HSA_MEM_FLAGS_RESERVED: c_uint = 0xfffffffc;
// Cache types in cache properties
pub const HSA_CACHE_TYPE_DATA: c_uint = 0x00000001;
pub const HSA_CACHE_TYPE_INSTRUCTION: c_uint = 0x00000002;
pub const HSA_CACHE_TYPE_CPU: c_uint = 0x00000004;
pub const HSA_CACHE_TYPE_HSACU: c_uint = 0x00000008;
pub const HSA_CACHE_TYPE_RESERVED: c_uint = 0xfffffff0;
// Link types in IO link properties (matches CRAT link types)
pub const HSA_IOLINK_TYPE_UNDEFINED: c_int = 0;
pub const HSA_IOLINK_TYPE_HYPERTRANSPORT: c_int = 1;
pub const HSA_IOLINK_TYPE_PCIEXPRESS: c_int = 2;
pub const HSA_IOLINK_TYPE_AMBA: c_int = 3;
pub const HSA_IOLINK_TYPE_MIPI: c_int = 4;
pub const HSA_IOLINK_TYPE_QPI_1_1: c_int = 5;
pub const HSA_IOLINK_TYPE_RESERVED1: c_int = 6;
pub const HSA_IOLINK_TYPE_RESERVED2: c_int = 7;
pub const HSA_IOLINK_TYPE_RAPID_IO: c_int = 8;
pub const HSA_IOLINK_TYPE_INFINIBAND: c_int = 9;
pub const HSA_IOLINK_TYPE_RESERVED3: c_int = 10;
pub const HSA_IOLINK_TYPE_XGMI: c_int = 11;
pub const HSA_IOLINK_TYPE_XGOP: c_int = 12;
pub const HSA_IOLINK_TYPE_GZ: c_int = 13;
pub const HSA_IOLINK_TYPE_ETHERNET_RDMA: c_int = 14;
pub const HSA_IOLINK_TYPE_RDMA_OTHER: c_int = 15;
pub const HSA_IOLINK_TYPE_OTHER: c_int = 16;
// Flag bits in IO link properties (matches CRAT flags, excluding the
// bi-directional flag, which is not offially part of the CRAT spec, and
// only used internally in KFD)
//

pub const HSA_IOLINK_FLAGS_RESERVED: c_uint = 0xffffffe0;
