//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/include/asm/mte.h
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
// Copyright (C) 2020 ARM Ltd.
//

extern "C" {
    pub fn mte_clear_page_tags(addr: *mut c_void);
}
extern "C" {
    pub fn mte_save_tags(page: *mut page) -> c_int;
}
extern "C" {
    pub fn mte_save_page_tags(page_addr: *const c_void, tag_storage: *mut c_void);
}
extern "C" {
    pub fn mte_restore_tags(entry: swp_entry_t, page: *mut page);
}
extern "C" {
    pub fn mte_restore_page_tags(page_addr: *mut c_void, tag_storage: *const c_void);
}
extern "C" {
    pub fn mte_invalidate_tags(type: c_int, offset: pgoff_t);
}
extern "C" {
    pub fn mte_invalidate_tags_area(type: c_int);
}
extern "C" {
    pub fn mte_free_tag_storage(storage: *mut c_char);
}

// track which pages have valid allocation tags

// simple lock to avoid multiple threads tagging the same page

//
// Ensure that the tags written prior to this function are visible
// before the page flags update.
//
// If the page is tagged, ensure ordering with a likely subsequent
// read of the tags.
//
// Lock the page for tagging and return 'true' if the page can be tagged,
// 'false' if already tagged. PG_mte_tagged is never cleared and therefore the
// locking only happens once for page initialisation.
//
// The page MTE lock state:
//
// Locked:	PG_mte_lock && !PG_mte_tagged
// Unlocked:	!PG_mte_lock || PG_mte_tagged
//
// Acquire semantics only if the page is tagged (returning 'false').
//
// The tags are either being initialised or may have been initialised
// already. Check if the PG_mte_tagged flag has been set or wait
// otherwise.
//
extern "C" {
    pub fn mte_zero_clear_page_tags(addr: *mut c_void);
}
extern "C" {
    pub fn mte_sync_tags(pte: pte_t, nr_pages: c_uint);
}
extern "C" {
    pub fn mte_copy_page_tags(kto: *mut c_void, kfrom: *const c_void);
}
extern "C" {
    pub fn mte_thread_init_user();
}
extern "C" {
    pub fn mte_thread_switch(next: *mut task_struct);
}
extern "C" {
    pub fn mte_cpu_setup();
}
extern "C" {
    pub fn mte_suspend_enter();
}
extern "C" {
    pub fn mte_suspend_exit();
}
extern "C" {
    pub fn set_mte_ctrl(task: *mut task_struct, arg: c_ulong) -> c_long;
}
extern "C" {
    pub fn get_mte_ctrl(task: *mut task_struct) -> c_long;
}
extern "C" {
    pub fn mte_probe_user_range(uaddr: *const char __user, size: usize) -> usize;
}

// unused if !CONFIG_ARM64_MTE, silence the compiler
pub const PG_mte_tagged: c_int = 0;

//
// Ensure that the tags written prior to this function are visible
// before the folio flags update.
//
// If the folio is tagged, ensure ordering with a likely subsequent
// read of the tags.
//
// The tags are either being initialised or may have been initialised
// already. Check if the PG_mte_tagged flag has been set or wait
// otherwise.
//

//
// Re-enable tag checking (TCO set on exception entry). This is only
// necessary if MTE is enabled in either the kernel or the userspace
// task in synchronous or asymmetric mode (SCTLR_EL1.TCF0 bit 0 is set
// for both). With MTE disabled in the kernel and disabled or
// asynchronous in userspace, tag check faults (including in uaccesses)
// are not reported, therefore there is no need to re-enable checking.
// This is beneficial on microarchitectures where re-enabling TCO is
// expensive.
//
extern "C" {
    pub fn volatile(_arg: SET_PSTATE_TCO(0)) -> asm;
}

extern "C" {
    pub fn mte_check_tfsr_el1();
}
//
// The asynchronous faults are sync'ed automatically with
// TFSR_EL1 on kernel entry but for exit an explicit dsb()
// is required.
//

