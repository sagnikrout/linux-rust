//! Automatically rewritten from C to Rust
//! Source: tools/perf/trace/beauty/futex_op.c
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


// SPDX-License-Identifier: LGPL-2.1

pub const FUTEX_WAIT_BITSET: c_int = 9;

pub const FUTEX_WAKE_BITSET: c_int = 10;

pub const FUTEX_WAIT_REQUEUE_PI: c_int = 11;

pub const FUTEX_CMP_REQUEUE_PI: c_int = 12;

pub const FUTEX_CLOCK_REALTIME: c_int = 256;

#[no_mangle]
pub unsafe extern "C" fn syscall_arg__scnprintf_futex_op(bf: *mut c_char, size: usize, arg: *mut syscall_arg) -> usize {
    size_t syscall_arg__scnprintf_futex_op(char *bf, size_t size, struct syscall_arg *arg)
    {
    let mut show_prefix: bool = arg.show_string_prefix;
    const char *prefix = "FUTEX_";
    enum syscall_futex_args {
    SCF_UADDR   = (1 << 0),
    SCF_OP	    = (1 << 1),
    SCF_VAL	    = (1 << 2),
    SCF_TIMEOUT = (1 << 3),
    SCF_UADDR2  = (1 << 4),
    SCF_VAL3    = (1 << 5),
    };
    let mut op: c_int = arg.val;
    let mut cmd: c_int = op & FUTEX_CMD_MASK;
    let mut printed: usize = 0;
    switch (cmd) {

    P_FUTEX_OP(WAIT);	    arg.mask |= SCF_VAL3|SCF_UADDR2;		  break;
    P_FUTEX_OP(WAKE);	    arg.mask |= SCF_VAL3|SCF_UADDR2|SCF_TIMEOUT; break;
    P_FUTEX_OP(FD);		    arg.mask |= SCF_VAL3|SCF_UADDR2|SCF_TIMEOUT; break;
    P_FUTEX_OP(REQUEUE);	    arg.mask |= SCF_VAL3|SCF_TIMEOUT;	          break;
    P_FUTEX_OP(CMP_REQUEUE);    arg.mask |= SCF_TIMEOUT;			  break;
    P_FUTEX_OP(CMP_REQUEUE_PI); arg.mask |= SCF_TIMEOUT;			  break;
    P_FUTEX_OP(WAKE_OP);							  break;
    P_FUTEX_OP(LOCK_PI);	    arg.mask |= SCF_VAL3|SCF_UADDR2|SCF_TIMEOUT; break;
    P_FUTEX_OP(UNLOCK_PI);	    arg.mask |= SCF_VAL3|SCF_UADDR2|SCF_TIMEOUT; break;
    P_FUTEX_OP(TRYLOCK_PI);	    arg.mask |= SCF_VAL3|SCF_UADDR2;		  break;
    P_FUTEX_OP(WAIT_BITSET);    arg.mask |= SCF_UADDR2;			  break;
    P_FUTEX_OP(WAKE_BITSET);    arg.mask |= SCF_UADDR2;			  break;
    P_FUTEX_OP(WAIT_REQUEUE_PI);						  break;
    default: printed = scnprintf(bf, size, "%#x", cmd);			  break;
    }
    if (op & FUTEX_PRIVATE_FLAG)
    printed += scnprintf(bf + printed, size - printed, "|%s%s", show_prefix ? prefix : "", "PRIVATE_FLAG");
    if (op & FUTEX_CLOCK_REALTIME)
    printed += scnprintf(bf + printed, size - printed, "|%s%s", show_prefix ? prefix : "", "CLOCK_REALTIME");
    return printed;
    }
