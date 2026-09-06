//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/kernel/ptrace/ptrace-vsx.c
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
    u64 buf[33];
    int i;
    flush_fp_to_thread(target);
// copy to local buffer then write that out
    for (i = 0; i < 32 ; i++)
    buf[i] = target.thread.TS_FPR(i);
    buf[32] = target.thread.fp_state.fpscr;
    return membuf_write(&to, buf, 33 * sizeof(u64));
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
    u64 buf[33];
    int i;
    flush_fp_to_thread(target);
    for (i = 0; i < 32 ; i++)
    buf[i] = target.thread.TS_FPR(i);
    buf[32] = target.thread.fp_state.fpscr;
// copy to local buffer then write that out
    i = user_regset_copyin(&pos, &count, &kbuf, &ubuf, buf, 0, -1);
    if (i)
    return i;
    for (i = 0; i < 32 ; i++)
    target.thread.TS_FPR(i) = buf[i];
    target.thread.fp_state.fpscr = buf[32];
    return 0;
    }
//
// Currently to set and get all the vsx state, you need to call
// the fp and VMX calls as well.  This only get/sets the lower 32
// 128bit VSX registers.
//
#[no_mangle]
pub unsafe extern "C" fn vsr_active(target: *mut task_struct, regset: *const user_regset) -> c_int {
    int vsr_active(struct task_struct *target, const struct user_regset *regset)
    {
    flush_vsx_to_thread(target);
    return target.thread.used_vsr ? regset.n : 0;
    }
//
// Regardless of transactions, 'fp_state' holds the current running
// value of all FPR registers and 'ckfp_state' holds the last
// checkpointed value of all FPR registers for the current
// transaction.
//
// Userspace interface buffer layout:
//
// struct data {
// u64	vsx[32];
// };
//
    int vsr_get(struct task_struct *target, const struct user_regset *regset,
    struct membuf to)
    {
    u64 buf[32];
    int i;
    flush_tmregs_to_thread(target);
    flush_fp_to_thread(target);
    flush_altivec_to_thread(target);
    flush_vsx_to_thread(target);
    for (i = 0; i < 32 ; i++)
    buf[i] = target.thread.fp_state.fpr[i][TS_VSRLOWOFFSET];
    return membuf_write(&to, buf, 32 * sizeof(double));
    }
//
// Regardless of transactions, 'fp_state' holds the current running
// value of all FPR registers and 'ckfp_state' holds the last
// checkpointed value of all FPR registers for the current
// transaction.
//
// Userspace interface buffer layout:
//
// struct data {
// u64	vsx[32];
// };
//
    int vsr_set(struct task_struct *target, const struct user_regset *regset,
    unsigned int pos, unsigned int count,
    const void *kbuf, const void __user *ubuf)
    {
    u64 buf[32];
    int ret, i;
    flush_tmregs_to_thread(target);
    flush_fp_to_thread(target);
    flush_altivec_to_thread(target);
    flush_vsx_to_thread(target);
    for (i = 0; i < 32 ; i++)
    buf[i] = target.thread.fp_state.fpr[i][TS_VSRLOWOFFSET];
    ret = user_regset_copyin(&pos, &count, &kbuf, &ubuf,
    buf, 0, 32 * sizeof(double));
    if (!ret)
    for (i = 0; i < 32 ; i++)
    target.thread.fp_state.fpr[i][TS_VSRLOWOFFSET] = buf[i];
    return ret;
    }
