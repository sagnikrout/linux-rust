//! Automatically rewritten from C to Rust
//! Source: tools/perf/trace/beauty/sched_policy.c
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

//
// Not defined anywhere else, probably, just to make sure we
// catch future flags
//
pub const SCHED_POLICY_MASK: c_uint = 0xff;

pub const SCHED_DEADLINE: c_int = 6;

pub const SCHED_RESET_ON_FORK: c_uint = 0x40000000;

    size_t syscall_arg__scnprintf_sched_policy(char *bf, size_t size,
    struct syscall_arg *arg)
    {
    let mut show_prefix: bool = arg.show_string_prefix;
    const char *prefix = "SCHED_";
    const char *policies[] = {
    "NORMAL", "FIFO", "RR", "BATCH", "ISO", "IDLE", "DEADLINE",
    };
    size_t printed;
    int policy = arg.val,
    flags = policy & ~SCHED_POLICY_MASK;
    policy &= SCHED_POLICY_MASK;
    if (policy <= SCHED_DEADLINE)
    printed = scnprintf(bf, size, "%s%s", show_prefix ? prefix : "", policies[policy]);
    else
    printed = scnprintf(bf, size, "%#x", policy);

    if (flags & SCHED_##n) { \
    printed += scnprintf(bf + printed, size - printed, "|%s%s", show_prefix ? prefix : "",  #n); \
    flags &= ~SCHED_##n; \
    }
    P_POLICY_FLAG(RESET_ON_FORK);

    if (flags)
    printed += scnprintf(bf + printed, size - printed, "|%#x", flags);
    return printed;
    }
