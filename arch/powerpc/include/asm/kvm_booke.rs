//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/kvm_booke.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright SUSE Linux Products GmbH 2010
//
// Authors: Alexander Graf <agraf@suse.de>
//

//
// Number of available lpids. Only the low-order 6 bits of LPID rgister are
// implemented on e500mc+ cores.
//
pub const KVMPPC_NR_LPIDS: c_int = 64;
pub const KVMPPC_INST_EHPRIV: c_uint = 0x7c00021c;
pub const EHPRIV_OC_SHIFT: c_int = 11;
// "ehpriv 1" : ehpriv with OC = 1 is used for debug emulation
pub const EHPRIV_OC_DEBUG: c_int = 1;
// XXX Would need to check TLB entry

// Magic page is only supported on e500v2

