//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/kernel/ptrace/ptrace-spe.c
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
// For get_evrregs/set_evrregs functions 'data' has the following layout:
//
// struct {
// u32 evr[32];
// u64 acc;
// u32 spefscr;
// }
//
#[no_mangle]
pub unsafe extern "C" fn evr_active(target: *mut task_struct, regset: *const user_regset) -> c_int {
    int evr_active(struct task_struct *target, const struct user_regset *regset)
    {
    flush_spe_to_thread(target);
    return target.thread.used_spe ? regset.n : 0;
    }
    int evr_get(struct task_struct *target, const struct user_regset *regset,
    struct membuf to)
    {
    flush_spe_to_thread(target);
    membuf_write(&to, &target.thread.evr, sizeof(target.thread.evr));
    BUILD_BUG_ON(offsetof(struct thread_struct, acc) + sizeof(u64) !=
    offsetof(struct thread_struct, spefscr));
    return membuf_write(&to, &target.thread.acc,
    sizeof(u64) + sizeof(u32));
    }
    int evr_set(struct task_struct *target, const struct user_regset *regset,
    unsigned int pos, unsigned int count,
    const void *kbuf, const void __user *ubuf)
    {
    int ret;
    flush_spe_to_thread(target);
    ret = user_regset_copyin(&pos, &count, &kbuf, &ubuf,
    &target.thread.evr,
    0, sizeof(target.thread.evr));
    BUILD_BUG_ON(offsetof(struct thread_struct, acc) + sizeof(u64) !=
    offsetof(struct thread_struct, spefscr));
    if (!ret)
    ret = user_regset_copyin(&pos, &count, &kbuf, &ubuf,
    &target.thread.acc,
    sizeof(target.thread.evr), -1);
    return ret;
    }
