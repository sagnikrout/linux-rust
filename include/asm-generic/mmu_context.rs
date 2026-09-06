//! Automatically rewritten from C Header to Rust Module
//! Source: include/asm-generic/mmu_context.h
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
// Generic hooks to implement no-op functionality.
//
// enter_lazy_tlb - Called when "tsk" is about to enter lazy TLB mode.
//
// @mm:  the currently active mm context which is becoming lazy
// @tsk: task which is entering lazy tlb
//
// tsk->mm will be NULL
//

//
// init_new_context - Initialize context of a new mm_struct.
// @tsk: task struct for the mm
// @mm:  the new mm struct
// @return: 0 on success, -errno on failure
//

//
// destroy_context - Undo init_new_context when the mm is going away
// @mm: old mm struct
//

//
// activate_mm - called after exec switches the current task to a new mm, to switch to it
// @prev_mm: previous mm of this task
// @next_mm: new mm
//

//
// dectivate_mm - called when an mm is released after exit or exec switches away from it
// @tsk: the task
// @mm:  the old mm
//

