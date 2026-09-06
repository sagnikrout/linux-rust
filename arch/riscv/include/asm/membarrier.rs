//! Automatically rewritten from C Header to Rust Module
//! Source: arch/riscv/include/asm/membarrier.h
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
// Only need the full barrier when switching between processes.
// Barrier when switching from kernel to userspace is not
// required here, given that it is implied by mmdrop(). Barrier
// when switching from userspace to kernel is not needed after
// store to rq->curr.
//
// The membarrier system call requires a full memory barrier
// after storing to rq->curr, before going back to user-space.
//
// This barrier is also needed for the SYNC_CORE command when
// switching between processes; in particular, on a transition
// from a thread belonging to another mm to a thread belonging
// to the mm for which a membarrier SYNC_CORE is done on CPU0:
//
// - [CPU0] sets all bits in the mm icache_stale_mask (in
// prepare_sync_core_cmd());
//
// - [CPU1] stores to rq->curr (by the scheduler);
//
// - [CPU0] loads rq->curr within membarrier and observes
// cpu_rq(1)->curr->mm != mm, so the IPI is skipped on
// CPU1; this means membarrier relies on switch_mm() to
// issue the sync-core;
//
// - [CPU1] switch_mm() loads icache_stale_mask; if the bit
// is zero, switch_mm() may incorrectly skip the sync-core.
//
// Matches a full barrier in the proximity of the membarrier
// system call entry.
//
