//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/shared/tdx_errno.h
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
// architectural status code for SEAMCALL
pub const TDX_SEAMCALL_STATUS_MASK: c_uint = 0xFFFFFFFF00000000ULL;
//
// TDX SEAMCALL Status Codes (returned in RAX)
//
pub const TDX_NON_RECOVERABLE_VCPU: c_uint = 0x4000000100000000ULL;
pub const TDX_NON_RECOVERABLE_TD: c_uint = 0x4000000200000000ULL;
pub const TDX_NON_RECOVERABLE_TD_NON_ACCESSIBLE: c_uint = 0x6000000500000000ULL;
pub const TDX_NON_RECOVERABLE_TD_WRONG_APIC_MODE: c_uint = 0x6000000700000000ULL;
pub const TDX_INTERRUPTED_RESUMABLE: c_uint = 0x8000000300000000ULL;
pub const TDX_SYS_BUSY: c_uint = 0x8000020200000000ULL;
pub const TDX_OPERAND_INVALID: c_uint = 0xC000010000000000ULL;
pub const TDX_OPERAND_BUSY: c_uint = 0x8000020000000000ULL;
pub const TDX_PREVIOUS_TLB_EPOCH_BUSY: c_uint = 0x8000020100000000ULL;
pub const TDX_PAGE_METADATA_INCORRECT: c_uint = 0xC000030000000000ULL;
pub const TDX_VCPU_NOT_ASSOCIATED: c_uint = 0x8000070200000000ULL;
pub const TDX_KEY_GENERATION_FAILED: c_uint = 0x8000080000000000ULL;
pub const TDX_KEY_STATE_INCORRECT: c_uint = 0xC000081100000000ULL;
pub const TDX_KEY_CONFIGURED: c_uint = 0x0000081500000000ULL;
pub const TDX_NO_HKID_READY_TO_WBCACHE: c_uint = 0x0000082100000000ULL;
pub const TDX_FLUSHVP_NOT_DONE: c_uint = 0x8000082400000000ULL;
pub const TDX_EPT_WALK_FAILED: c_uint = 0xC0000B0000000000ULL;
pub const TDX_EPT_ENTRY_STATE_INCORRECT: c_uint = 0xC0000B0D00000000ULL;
pub const TDX_METADATA_FIELD_NOT_READABLE: c_uint = 0xC0000C0200000000ULL;
//
// TDX module operand ID, appears in 31:0 part of error code as
// detail information
//
pub const TDX_OPERAND_ID_RCX: c_uint = 0x01;
pub const TDX_OPERAND_ID_TDR: c_uint = 0x80;
pub const TDX_OPERAND_ID_SEPT: c_uint = 0x92;
pub const TDX_OPERAND_ID_TD_EPOCH: c_uint = 0xa9;
