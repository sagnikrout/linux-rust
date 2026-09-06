//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/kvm_book3s_32.h
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
pub const PTE_SIZE: c_int = 12;
pub const VSID_ALL: c_int = 0;
pub const SR_INVALID: c_uint = 0x00000001	/* VSID 1 should always be unused */;
pub const SR_KP: c_uint = 0x20000000;
pub const PTE_V: c_uint = 0x80000000;
pub const PTE_SEC: c_uint = 0x00000040;
pub const PTE_M: c_uint = 0x00000010;
pub const PTE_R: c_uint = 0x00000100;
pub const PTE_C: c_uint = 0x00000080;
pub const SID_SHIFT: c_int = 28;
pub const ESID_MASK: c_uint = 0xf0000000;
pub const VSID_MASK: c_uint = 0x00fffffff0000000ULL;
pub const VPN_SHIFT: c_int = 12;
