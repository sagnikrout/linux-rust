//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/vmware.h
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


// SPDX-License-Identifier: GPL-2.0 or MIT

//
// VMware hypercall ABI.
//
// - Low bandwidth (LB) hypercalls (I/O port based, vmcall and vmmcall)
// have up to 6 input and 6 output arguments passed and returned using
// registers: %eax (arg0), %ebx (arg1), %ecx (arg2), %edx (arg3),
// %esi (arg4), %edi (arg5).
// The following input arguments must be initialized by the caller:
// arg0 - VMWARE_HYPERVISOR_MAGIC
// arg2 - Hypercall command
// arg3 bits [15:0] - Port number, LB and direction flags
//
// - Low bandwidth TDX hypercalls (x86_64 only) are similar to LB
// hypercalls. They also have up to 6 input and 6 output on registers
// arguments, with different argument to register mapping:
// %r12 (arg0), %rbx (arg1), %r13 (arg2), %rdx (arg3),
// %rsi (arg4), %rdi (arg5).
//
// - High bandwidth (HB) hypercalls are I/O port based only. They have
// up to 7 input and 7 output arguments passed and returned using
// registers: %eax (arg0), %ebx (arg1), %ecx (arg2), %edx (arg3),
// %esi (arg4), %edi (arg5), %ebp (arg6).
// The following input arguments must be initialized by the caller:
// arg0 - VMWARE_HYPERVISOR_MAGIC
// arg1 - Hypercall command
// arg3 bits [15:0] - Port number, HB and direction flags
//
// For compatibility purposes, x86_64 systems use only lower 32 bits
// for input and output arguments.
//
// The hypercall definitions differ in the low word of the %edx (arg3)
// in the following way: the old I/O port based interface uses the port
// number to distinguish between high- and low bandwidth versions, and
// uses IN/OUT instructions to define transfer direction.
//
// The new vmcall interface instead uses a set of flags to select
// bandwidth mode and transfer direction. The flags should be loaded
// into arg3 by any user and are automatically replaced by the port
// number if the I/O port method is used.
//

pub const VMWARE_HYPERVISOR_PORT: c_uint = 0x5658;

pub const VMWARE_HYPERVISOR_MAGIC: c_uint = 0x564d5868U;
pub const VMWARE_CMD_GETVERSION: c_int = 10;
pub const VMWARE_CMD_GETHZ: c_int = 45;
pub const VMWARE_CMD_GETVCPU_INFO: c_int = 68;
pub const VMWARE_CMD_STEALCLOCK: c_int = 91;
//
// Hypercall command mask:
// bits [6:0] command, range [0, 127]
// bits [19:16] sub-command, range [0, 15]
//
pub const VMWARE_CMD_MASK: c_uint = 0xf007fU;

pub const VMWARE_TDX_VENDOR_LEAF: c_uint = 0x1af7e4909ULL;
pub const VMWARE_TDX_HCALL_FUNC: c_int = 1;
//
// The low bandwidth call. The low word of %edx is presumed to have OUT bit
// set. The high word of %edx may contain input data from the caller.
//

//
// High bandwidth calls are not supported on encrypted memory guests.
// The caller should check cc_platform_has(CC_ATTR_MEM_ENCRYPT) and use
// low bandwidth hypercall if memory encryption is set.
// This assumption simplifies HB hypercall implementation to just I/O port
// based approach without alternative patching.
//

