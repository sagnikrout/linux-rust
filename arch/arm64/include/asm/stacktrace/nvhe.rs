//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/include/asm/stacktrace/nvhe.h
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
// KVM nVHE hypervisor stack tracing support.
//
// The unwinder implementation depends on the nVHE mode:
//
// 1) Non-protected nVHE mode - the host can directly access the
// HYP stack pages and unwind the HYP stack in EL1. This saves having
// to allocate shared buffers for the host to read the unwinded
// stacktrace.
//
// 2) pKVM (protected nVHE) mode - the host cannot directly access
// the HYP memory. The stack is unwinded in EL2 and dumped to a shared
// buffer where the host can read and print the stacktrace.
//
// Copyright (C) 2022 Google LLC
//

//
// kvm_nvhe_unwind_init() - Start an unwind from the given nVHE HYP fp and pc
//
// @state : unwind_state to initialize
// @fp    : frame pointer at which to start the unwinding.
// @pc    : program counter at which to start the unwinding.
//
// Conventional (non-protected) nVHE HYP stack unwinder
//
// In non-protected mode, the unwinding is done from kernel proper context
// (by the host in EL1).
//
extern "C" {
    pub fn kvm_nvhe_dump_backtrace(hyp_offset: c_ulong);
}

