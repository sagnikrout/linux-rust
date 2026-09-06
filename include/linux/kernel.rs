//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/kernel.h
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
// NOTE:
//
// This header has combined a lot of unrelated to each other stuff.
// The process of splitting its content is in progress while keeping
// backward compatibility. That's why it's highly recommended NOT to
// include this header inside another header file, especially under
// generic or architectural include/ directory.
//

extern "C" {
    pub fn __cond_resched() -> c_int;
}

extern "C" {
    pub fn __cond_resched() -> c_int;
}

extern "C" {
    pub fn dynamic_might_resched() -> c_int;
}

extern "C" {
    pub fn __might_resched(file: *const c_char, line: c_int, offsets: c_uint);
}
extern "C" {
    pub fn __might_sleep(file: *const c_char, line: c_int);
}
extern "C" {
    pub fn __cant_sleep(file: *const c_char, line: c_int);
}
extern "C" {
    pub fn __cant_migrate(file: *const c_char, line: c_int);
}
//
// might_sleep - annotation for functions that can sleep
//
// this macro will print a stack trace if it is executed in an atomic
// context (spinlock, irq-handler, ...). Additional sections where blocking is
// not allowed can be annotated with non_block_start() and non_block_end()
// pairs.
//
// This is a useful debugging help to be able to catch problems early and not
// be bitten later when the calling function happens to sleep when it is not
// supposed to.
//

//
// cant_sleep - annotation for functions that cannot sleep
//
// this macro will print a stack trace if it is executed with preemption enabled
//

//
// cant_migrate - annotation for functions that cannot migrate
//
// Will print a stack trace if executed in code which is migratable
//

//
// non_block_start - annotate the start of section where sleeping is prohibited
//
// This is on behalf of the oom reaper, specifically when it is calling the mmu
// notifiers. The problem is that if the notifier were to block on, for example,
// mutex_lock() and if the process which holds that mutex were to perform a
// sleeping memory allocation, the oom reaper is now blocked on completion of
// that memory allocation. Other blocking calls like wait_event() pose similar
// issues.
//

//
// non_block_end - annotate the end of section where sleeping is prohibited
//
// Closes a section opened by non_block_start().
//

extern "C" {
    pub fn __might_fault(file: *const c_char, line: c_int);
}

extern "C" {
    pub fn core_kernel_text(addr: c_ulong) -> c_int;
}
extern "C" {
    pub fn __kernel_text_address(addr: c_ulong) -> c_int;
}
extern "C" {
    pub fn kernel_text_address(addr: c_ulong) -> c_int;
}
extern "C" {
    pub fn func_ptr_is_kernel_text(ptr: *mut c_void) -> c_int;
}
extern "C" {
    pub fn bust_spinlocks(yes: c_int);
}
//
// enum system_states - Values used for system_state.
//
// @SYSTEM_BOOTING:	%0, no init needed
// @SYSTEM_SCHEDULING: system is ready for scheduling; OK to use RCU
// @SYSTEM_FREEING_INITMEM: system is freeing all of initmem; almost running
// @SYSTEM_RUNNING:	system is up and running
// @SYSTEM_HALT:	system entered clean system halt state
// @SYSTEM_POWER_OFF:	system entered shutdown/clean power off state
// @SYSTEM_RESTART:	system entered emergency power off or normal restart
// @SYSTEM_SUSPEND:	system entered suspend or hibernate state
//
// Note:
// Ordering of the states must not be changed
// as code checks for <, <=, >, >= STATE.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum system_states {
    SYSTEM_BOOTING,
    SYSTEM_SCHEDULING,
    SYSTEM_FREEING_INITMEM,
    SYSTEM_RUNNING,
    SYSTEM_HALT,
    SYSTEM_POWER_OFF,
    SYSTEM_RESTART,
    SYSTEM_SUSPEND,
}

// Rebuild everything on CONFIG_DYNAMIC_FTRACE

