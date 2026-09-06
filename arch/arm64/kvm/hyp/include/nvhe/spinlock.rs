//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/kvm/hyp/include/nvhe/spinlock.h
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
// A stand-alone ticket spinlock implementation for use by the non-VHE
// KVM hypervisor code running at EL2.
//
// Copyright (C) 2020 Google LLC
// Author: Will Deacon <will@kernel.org>
//
// Heavily based on the implementation removed by c11090474d70 which was:
// Copyright (C) 2012 ARM Ltd.
//

// (l) = __HYP_SPIN_LOCK_UNLOCKED;				\
// Atomically increment the next ticket.
// LL/SC
// LSE atomics
// Did we get the lock?
//
// No: spin on the owner. Send a local event to avoid missing an
// unlock before the exclusive load.
//
// We got the lock. Critical section starts here.
// LL/SC
// LSE atomics

//
// The __pkvm_init() path accesses protected data-structures without
// holding locks as the other CPUs are guaranteed to not enter EL2
// concurrently at this point in time. The point by which EL2 is
// initialized on all CPUs is reflected in the pkvm static key, so
// wait until it is set before checking the lock state.
//

