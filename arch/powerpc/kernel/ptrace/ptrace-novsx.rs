//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/kernel/ptrace/ptrace-novsx.c
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


// SPDX-License-Identifier: GPL-2.0-or-later

//
// Regardless of transactions, 'fp_state' holds the current running
// value of all FPR registers and 'ckfp_state' holds the last checkpointed
// value of all FPR registers for the current transaction.
//
// Userspace interface buffer layout:
//
// struct data {
// u64	fpr[32];
// u64	fpscr;
// };
//
    int fpr_get(struct task_struct *target, const struct user_regset *regset,
    struct membuf to)
    {

    BUILD_BUG_ON(offsetof(struct thread_fp_state, fpscr) !=
    offsetof(struct thread_fp_state, fpr[32]));
    flush_fp_to_thread(target);
    return membuf_write(&to, &target.thread.fp_state, 33 * sizeof(u64));

    return membuf_write(&to, &empty_zero_page, 33 * sizeof(u64));

    }
//
// Regardless of transactions, 'fp_state' holds the current running
// value of all FPR registers and 'ckfp_state' holds the last checkpointed
// value of all FPR registers for the current transaction.
//
// Userspace interface buffer layout:
//
// struct data {
// u64	fpr[32];
// u64	fpscr;
// };
//
    int fpr_set(struct task_struct *target, const struct user_regset *regset,
    unsigned int pos, unsigned int count,
    const void *kbuf, const void __user *ubuf)
    {

    BUILD_BUG_ON(offsetof(struct thread_fp_state, fpscr) !=
    offsetof(struct thread_fp_state, fpr[32]));
    flush_fp_to_thread(target);
    return user_regset_copyin(&pos, &count, &kbuf, &ubuf,
    &target.thread.fp_state, 0, -1);

    return 0;

    }
