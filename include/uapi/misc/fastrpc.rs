//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/misc/fastrpc.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note

//
// enum fastrpc_map_flags - control flags for mapping memory on DSP user process
// @FASTRPC_MAP_STATIC: Map memory pages with RW- permission and CACHE WRITEBACK.
// The driver is responsible for cache maintenance when passed
// the buffer to FastRPC calls. Same virtual address will be
// assigned for subsequent FastRPC calls.
// @FASTRPC_MAP_RESERVED: Reserved
// @FASTRPC_MAP_FD: Map memory pages with RW- permission and CACHE WRITEBACK.
// Mapping tagged with a file descriptor. User is responsible for
// CPU and DSP cache maintenance for the buffer. Get virtual address
// of buffer on DSP using HAP_mmap_get() and HAP_mmap_put() APIs.
// @FASTRPC_MAP_FD_DELAYED: Mapping delayed until user call HAP_mmap() and HAP_munmap()
// functions on DSP. It is useful to map a buffer with cache modes
// other than default modes. User is responsible for CPU and DSP
// cache maintenance for the buffer.
// @FASTRPC_MAP_FD_NOMAP: This flag is used to skip CPU mapping,
// otherwise behaves similar to FASTRPC_MAP_FD_DELAYED flag.
// @FASTRPC_MAP_MAX: max count for flags
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fastrpc_map_flags {
    FASTRPC_MAP_STATIC = 0,
    FASTRPC_MAP_RESERVED,
    FASTRPC_MAP_FD = 2,
    FASTRPC_MAP_FD_DELAYED,
    FASTRPC_MAP_FD_NOMAP = 16,
    FASTRPC_MAP_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fastrpc_proc_attr {
// Macro for Debug attr
    FASTRPC_MODE_DEBUG		= (1 << 0),
// Macro for Ptrace
    FASTRPC_MODE_PTRACE		= (1 << 1),
// Macro for CRC Check
    FASTRPC_MODE_CRC		= (1 << 2),
// Macro for Unsigned PD
    FASTRPC_MODE_UNSIGNED_MODULE	= (1 << 3),
// Macro for Adaptive QoS
    FASTRPC_MODE_ADAPTIVE_QOS	= (1 << 4),
// Macro for System Process
    FASTRPC_MODE_SYSTEM_PROCESS	= (1 << 5),
// Macro for Prvileged Process
    FASTRPC_MODE_PRIVILEGED		= (1 << 6),
}

// Fastrpc attribute for memory protection of buffers

//
// FASTRPC_POLL_MODE - Enable/disable poll mode for FastRPC invocations
//
// Poll mode is an optimization that allows the CPU to poll shared memory
// for completion instead of waiting for an interrupt-based response.
// This reduces latency for fast-completing operations.
//
// Restrictions:
// - Only supported for USER_PD (User Protection Domain)
// - Only applies to dynamic modules (handle > 20)
// - Static modules always use interrupt-based completion
//
// Values:
// - 0: Disable poll mode (use interrupt-based completion)
// - 1: Enable poll mode (poll shared memory for completion)
//

// Values for FASTRPC_POLL_MODE request
pub const FASTRPC_POLL_MODE_DISABLE: c_int = 0;
pub const FASTRPC_POLL_MODE_ENABLE: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fastrpc_invoke_args {
    pub ptr: __u64,
    pub length: __u64,
    pub fd: __s32,
    pub attr: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fastrpc_invoke {
    pub handle: __u32,
    pub sc: __u32,
    pub args: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fastrpc_init_create {
    pub /: *mut *mut __u32 filelen; / elf file length,
    pub /: *mut *mut __s32 filefd; / fd for the file,
    pub attrs: __u32,
    pub siglen: __u32,
    pub /: *mut *mut __u64 file; / pointer to elf file,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fastrpc_init_create_static {
    pub /: *mut *mut __u32 namelen; / length of pd process name,
    pub memlen: __u32,
    pub /: *mut *mut __u64 name; / pd process name,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fastrpc_alloc_dma_buf {
    pub /: *mut *mut __s32 fd; / fd,
    pub /: *mut *mut __u32 flags; / flags to map with,
    pub /: *mut *mut __u64 size; / size,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fastrpc_req_mmap {
    pub fd: __s32,
    pub /: *mut *mut __u32 flags; / flags for dsp to map with,
    pub /: *mut *mut __u64 vaddrin; / optional virtual address,
    pub /: *mut *mut __u64 size; / size,
    pub /: *mut *mut __u64 vaddrout; / dsp virtual address,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fastrpc_mem_map {
    pub version: __s32,
    pub /: *mut *mut __s32 fd; / fd,
    pub /: *mut *mut __s32 offset; / buffer offset,
    pub /: *mut *mut __u32 flags; / flags defined in enum fastrpc_map_flags,
    pub /: *mut *mut __u64 vaddrin; / buffer virtual address,
    pub /: *mut *mut __u64 length; / buffer length,
    pub /: *mut *mut __u64 vaddrout; / [out] remote virtual address,
    pub /: *mut *mut __s32 attrs; / buffer attributes used for SMMU mapping,
    pub reserved: [__s32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fastrpc_req_munmap {
    pub /: *mut *mut __u64 vaddrout; / address to unmap,
    pub /: *mut *mut __u64 size; / size,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fastrpc_mem_unmap {
    pub vesion: __s32,
    pub /: *mut *mut __s32 fd; / fd,
    pub /: *mut *mut __u64 vaddr; / remote process (dsp) virtual address,
    pub /: *mut *mut __u64 length; / buffer size,
    pub reserved: [__s32; 5],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fastrpc_ioctl_set_option {
    pub /: *mut *mut __u32 request_id; / Request type (e.g., FASTRPC_POLL_MODE),
    pub /: *mut *mut __u32 value; / Request-specific value,
    pub reserved: [__s32; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fastrpc_ioctl_capability {
    pub /: *mut *mut __u32 unused; / deprecated, ignored by the kernel,
    pub attribute_id: __u32,
    pub /: *mut *mut __u32 capability; / dsp capability,
    pub reserved: [__u32; 4],
}
