//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/shared/tdx.h
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

pub const TDX_HYPERCALL_STANDARD: c_int = 0;
pub const TDX_CPUID_LEAF_ID: c_uint = 0x21;

// TDX module Call Leaf IDs
pub const TDG_VP_VMCALL: c_int = 0;
pub const TDG_VP_INFO: c_int = 1;
pub const TDG_MR_RTMR_EXTEND: c_int = 2;
pub const TDG_VP_VEINFO_GET: c_int = 3;
pub const TDG_MR_REPORT: c_int = 4;
pub const TDG_MEM_PAGE_ACCEPT: c_int = 6;
pub const TDG_VM_RD: c_int = 7;
pub const TDG_VM_WR: c_int = 8;
// TDX TD attributes
pub const TDX_TD_ATTR_DEBUG_BIT: c_int = 0;

pub const TDX_TD_ATTR_HGS_PLUS_PROF_BIT: c_int = 4;

pub const TDX_TD_ATTR_PERF_PROF_BIT: c_int = 5;

pub const TDX_TD_ATTR_PMT_PROF_BIT: c_int = 6;

pub const TDX_TD_ATTR_ICSSD_BIT: c_int = 16;

pub const TDX_TD_ATTR_LASS_BIT: c_int = 27;

pub const TDX_TD_ATTR_SEPT_VE_DISABLE_BIT: c_int = 28;

pub const TDX_TD_ATTR_MIGRATABLE_BIT: c_int = 29;

pub const TDX_TD_ATTR_PKS_BIT: c_int = 30;

pub const TDX_TD_ATTR_KL_BIT: c_int = 31;

pub const TDX_TD_ATTR_TPA_BIT: c_int = 62;

pub const TDX_TD_ATTR_PERFMON_BIT: c_int = 63;

// TDX TD-Scope Metadata. To be used by TDG.VM.WR and TDG.VM.RD
pub const TDCS_CONFIG_FLAGS: c_uint = 0x1110000300000016;
pub const TDCS_TD_CTLS: c_uint = 0x1110000300000017;
pub const TDCS_NOTIFY_ENABLES: c_uint = 0x9100000000000010;
pub const TDCS_TOPOLOGY_ENUM_CONFIGURED: c_uint = 0x9100000000000019;
// TDCS_CONFIG_FLAGS bits

// TDCS_TD_CTLS bits
pub const TD_CTLS_PENDING_VE_DISABLE_BIT: c_int = 0;

pub const TD_CTLS_ENUM_TOPOLOGY_BIT: c_int = 1;

pub const TD_CTLS_VIRT_CPUID2_BIT: c_int = 2;

pub const TD_CTLS_REDUCE_VE_BIT: c_int = 3;

pub const TD_CTLS_LOCK_BIT: c_int = 63;

// TDX hypercall Leaf IDs
pub const TDVMCALL_GET_TD_VM_CALL_INFO: c_uint = 0x10000;
pub const TDVMCALL_MAP_GPA: c_uint = 0x10001;
pub const TDVMCALL_GET_QUOTE: c_uint = 0x10002;
pub const TDVMCALL_REPORT_FATAL_ERROR: c_uint = 0x10003;
pub const TDVMCALL_SETUP_EVENT_NOTIFY_INTERRUPT: c_uint = 0x10004ULL;
//
// TDG.VP.VMCALL Status Codes (returned in R10)
//
pub const TDVMCALL_STATUS_SUCCESS: c_uint = 0x0000000000000000ULL;
pub const TDVMCALL_STATUS_RETRY: c_uint = 0x0000000000000001ULL;
pub const TDVMCALL_STATUS_INVALID_OPERAND: c_uint = 0x8000000000000000ULL;
pub const TDVMCALL_STATUS_ALIGN_ERROR: c_uint = 0x8000000000000002ULL;
pub const TDVMCALL_STATUS_SUBFUNC_UNSUPPORTED: c_uint = 0x8000000000000003ULL;
//
// Bitmasks of exposed registers (with VMM).
//

//
// These registers are clobbered to hold arguments for each
// TDVMCALL. They are safe to expose to the VMM.
// Each bit in this mask represents a register ID. Bit field
// details can be found in TDX GHCI specification, section
// titled "TDCALL [TDG.VP.VMCALL] leaf".
//

// TDX supported page sizes from the TDX module ABI.
pub const TDX_PS_4K: c_int = 0;
pub const TDX_PS_2M: c_int = 1;
pub const TDX_PS_1G: c_int = 2;

//
// Used in __tdcall*() to gather the input/output registers' values of the
// TDCALL instruction when requesting services from the TDX module. This is a
// software only structure and not part of the TDX module/VMM ABI
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tdx_module_args {
// callee-clobbered
    pub rcx: u64,
    pub rdx: u64,
    pub r8: u64,
    pub r9: u64,
// extra callee-clobbered
    pub r10: u64,
    pub r11: u64,
// callee-saved + rdi/rsi
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
    pub rbx: u64,
    pub rdi: u64,
    pub rsi: u64,
}

// Used to communicate with the TDX module
extern "C" {
    pub fn __tdcall(fn: u64, args: *mut tdx_module_args) -> u64;
}
extern "C" {
    pub fn __tdcall_ret(fn: u64, args: *mut tdx_module_args) -> u64;
}
extern "C" {
    pub fn __tdcall_saved_ret(fn: u64, args: *mut tdx_module_args) -> u64;
}
// Used to request services from the VMM
extern "C" {
    pub fn __tdx_hypercall(args: *mut tdx_module_args) -> u64;
}
//
// Wrapper for standard use of __tdx_hypercall with no output aside from
// return code.
//
extern "C" {
    pub fn __tdx_hypercall(_arg: &args) -> return;
}
// Called from __tdx_hypercall() for unrecoverable failure
extern "C" {
    pub fn __tdx_hypercall_failed() -> void __noreturn;
}
extern "C" {
    pub fn tdx_accept_memory(start: phys_addr_t, end: phys_addr_t) -> bool;
}
//
// The TDG.VP.VMCALL-Instruction-execution sub-functions are defined
// independently from but are currently matched 1:1 with VMX EXIT_REASONs.
// Reusing the KVM EXIT_REASON macros makes it easier to connect the host and
// guest sides of these calls.
//

