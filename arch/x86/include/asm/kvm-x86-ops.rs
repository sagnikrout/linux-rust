//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/kvm-x86-ops.h
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

//
// KVM_X86_OP() and KVM_X86_OP_OPTIONAL() are used to help generate
// both DECLARE/DEFINE_STATIC_CALL() invocations and
// "static_call_update()" calls.
//
// KVM_X86_OP_OPTIONAL() can be used for those functions that can have
// a NULL definition.  KVM_X86_OP_OPTIONAL_RET0() can be used likewise
// to make a definition optional, but in this case the default will
// be __static_call_return0.
//

