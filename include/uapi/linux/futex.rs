//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/futex.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note

// Second argument to futex syscall
pub const FUTEX_WAIT: c_int = 0;
pub const FUTEX_WAKE: c_int = 1;
pub const FUTEX_FD: c_int = 2;
pub const FUTEX_REQUEUE: c_int = 3;
pub const FUTEX_CMP_REQUEUE: c_int = 4;
pub const FUTEX_WAKE_OP: c_int = 5;
pub const FUTEX_LOCK_PI: c_int = 6;
pub const FUTEX_UNLOCK_PI: c_int = 7;
pub const FUTEX_TRYLOCK_PI: c_int = 8;
pub const FUTEX_WAIT_BITSET: c_int = 9;
pub const FUTEX_WAKE_BITSET: c_int = 10;
pub const FUTEX_WAIT_REQUEUE_PI: c_int = 11;
pub const FUTEX_CMP_REQUEUE_PI: c_int = 12;
pub const FUTEX_LOCK_PI2: c_int = 13;
pub const FUTEX_PRIVATE_FLAG: c_int = 128;
pub const FUTEX_CLOCK_REALTIME: c_int = 256;
pub const FUTEX_ROBUST_UNLOCK: c_int = 512;
pub const FUTEX_ROBUST_LIST32: c_int = 1024;

//
// Operations to unlock a futex, clear the robust list pending op pointer and
// wake waiters.
//

//
// Flags for futex2 syscalls.
//
// NOTE: these are not pure flags, they can also be seen as:
//
// union {
// u32  flags;
// struct {
// u32 size    : 2,
// numa    : 1,
// : 4,
// private : 1;
// };
//
pub const FUTEX2_SIZE_U8: c_uint = 0x00;
pub const FUTEX2_SIZE_U16: c_uint = 0x01;
pub const FUTEX2_SIZE_U32: c_uint = 0x02;
pub const FUTEX2_SIZE_U64: c_uint = 0x03;
pub const FUTEX2_NUMA: c_uint = 0x04;
pub const FUTEX2_MPOL: c_uint = 0x08;
// 0x10
// 0x20
// 0x40

pub const FUTEX2_SIZE_MASK: c_uint = 0x03;
// do not use

//
// When FUTEX2_NUMA doubles the futex word, the second word is a node value.
// The special value -1 indicates no-node. This is the same value as
// NUMA_NO_NODE, except that value is not ABI, this is.
//

//
// Max numbers of elements in a futex_waitv array
//
pub const FUTEX_WAITV_MAX: c_int = 128;
//
// struct futex_waitv - A waiter for vectorized wait
// @val:	Expected value at uaddr
// @uaddr:	User address to wait on
// @flags:	Flags for this waiter
// @__reserved:	Reserved member to preserve data alignment. Should be 0.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct futex_waitv {
    pub val: __u64,
    pub uaddr: __u64,
    pub flags: __u32,
    pub __reserved: __u32,
}

//
// Support for robust futexes: the kernel cleans up held futexes at
// thread exit time.
//
// Per-lock list entry - embedded in user-space locks, somewhere close
// to the futex field. (Note: user-space uses a double-linked list to
// achieve O(1) list add and remove, but the kernel only needs to know
// about the forward link)
//
// NOTE: this structure is part of the syscall ABI, and must not be
// changed.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct robust_list {
    pub next: *mut robust_list __user,
}

//
// Per-thread list head:
//
// NOTE: this structure is part of the syscall ABI, and must only be
// changed if the change is first communicated with the glibc folks.
// (When an incompatible change is done, we'll increase the structure
// size, which glibc will detect)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct robust_list_head {
//
// The head of the list. Points back to itself if empty:
//
    pub list: robust_list,
//
// This relative offset is set by user-space, it gives the kernel
// the relative position of the futex field to examine. This way
// we keep userspace flexible, to freely shape its data-structure,
// without hardcoding any particular offset into the kernel:
//
    pub futex_offset: c_long,
//
// The death of the thread may race with userspace setting
// up a lock's links. So to handle this race, userspace first
// sets this field to the address of the to-be-taken lock,
// then does the lock acquire, and then adds itself to the
// list, and then clears this field. Hence the kernel will
// always have full knowledge of all locks that the thread
// _might_ have taken. We check the owner TID in any case,
// so only truly owned locks will be handled.
//
    pub list_op_pending: *mut robust_list __user,
}

//
// Are there any waiters for this robust futex:
//
pub const FUTEX_WAITERS: c_uint = 0x80000000;
//
// The kernel signals via this bit that a thread holding a futex
// has exited without unlocking the futex. The kernel also does
// a FUTEX_WAKE on such futexes, after setting the bit, to wake
// up any possible waiters:
//
pub const FUTEX_OWNER_DIED: c_uint = 0x40000000;
//
// The rest of the robust-futex field is for the TID:
//
pub const FUTEX_TID_MASK: c_uint = 0x3fffffff;
//
// This limit protects against a deliberately circular list.
// (Not worth introducing an rlimit for it)
//
pub const ROBUST_LIST_LIMIT: c_int = 2048;
// Modifiers for robust_list_head::list_op_pending

//
// bitset with all bits set for the FUTEX_xxx_BITSET OPs to request a
// match of any bit.
//
pub const FUTEX_BITSET_MATCH_ANY: c_uint = 0xffffffff;

// FUTEX_WAKE_OP will perform atomically
// (int *)UADDR2 = oldval OP OPARG;

