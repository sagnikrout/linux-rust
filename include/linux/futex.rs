//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/futex.h
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
// Futexes are matched on equal values of this key.
// The key type depends on whether it's a shared or private mapping.
// Don't rearrange members without looking at hash_futex().
//
// offset is aligned to a multiple of sizeof(u32) (== 4) by definition.
// We use the two low order bits of offset to tell what is the kind of key :
// 00 : Private process futex (PTHREAD_PROCESS_PRIVATE)
// (no reference on an inode or mm)
// 01 : Shared futex (PTHREAD_PROCESS_SHARED)
// mapped on a file (reference on the underlying inode)
// 10 : Shared futex (PTHREAD_PROCESS_SHARED)
// (but private mapping on an mm, and reference taken on it)
//

#[repr(C)]
#[derive(Copy, Clone)]
pub union futex_key {
    pub i_seq: u64,
    pub pgoff: c_ulong,
    pub offset: c_uint,
// unsigned int node;
    pub shared: },
    pub mm: *mut mm_struct,
    pub __tmp: u64,
}

// unsigned int node;

extern "C" {
    pub fn futex_exit_recursive(tsk: *mut task_struct);
}
extern "C" {
    pub fn futex_exit_exec_release(tsk: *mut task_struct);
}
extern "C" {
    pub fn futex_exec_done(tsk: *mut task_struct);
}
extern "C" {
    pub fn futex_hash_prctl(arg2: c_ulong, arg3: c_ulong, arg4: c_ulong) -> c_int;
}

extern "C" {
    pub fn futex_hash_allocate_default() -> c_int;
}
extern "C" {
    pub fn futex_hash_free(mm: *mut mm_struct);
}

extern "C" {
    pub fn futex_reset_cs_ranges(fd: *mut futex_mm_data);
}
extern "C" {
    pub fn __futex_fixup_robust_unlock(regs: *mut pt_regs, csr: *mut futex_unlock_cs_range);
}
//
// Avoid dereferencing current->mm if not returning from interrupt.
// current->rseq.event is going to be used subsequently, so bringing the
// cache line in is not a big deal.
//
// The loop is optimized out for !COMPAT

extern "C" {
    pub fn futex_mm_init(mm: *mut mm_struct);
}

