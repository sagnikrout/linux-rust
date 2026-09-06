//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/kvm-stat-arch/arm64_exception_types.h
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
// Per asm/virt.h
pub const HVC_STUB_ERR: c_uint = 0xbadca11;
// Per asm/kvm_asm.h
pub const ARM_EXCEPTION_IRQ: c_int = 0;
pub const ARM_EXCEPTION_EL1_SERROR: c_int = 1;
pub const ARM_EXCEPTION_TRAP: c_int = 2;
pub const ARM_EXCEPTION_IL: c_int = 3;
// The hyp-stub will return this for any kvm_call_hyp() call

// Per asm/esr.h

// Unallocated EC: 0x02

// Unallocated EC: 0x0B

// Unallocated EC: 0x0F - 0x10

// Unallocated EC: 0x14

// Unallocated EC: 0x1B

// Unallocated EC: 0x1E

// Unallocated EC: 0x23

// Unallocated EC: 0x29 - 0x2B

// Unallocated EC: 0x2E

// Unallocated EC: 0x36 - 0x37

// Unallocated EC: 0x39

// Unallocated EC: 0x3B

// Unallocated EC: 0x3D - 0x3F

