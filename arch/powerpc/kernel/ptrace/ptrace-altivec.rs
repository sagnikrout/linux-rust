//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/kernel/ptrace/ptrace-altivec.c
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
// Get/set all the altivec registers vr0..vr31, vscr, vrsave, in one go.
// The transfer totals 34 quadword.  Quadwords 0-31 contain the
// corresponding vector registers.  Quadword 32 contains the vscr as the
// last word (offset 12) within that quadword.  Quadword 33 contains the
// vrsave as the first word (offset 0) within the quadword.
//
// This definition of the VMX state is compatible with the current PPC32
// ptrace interface.  This allows signal handling and ptrace to use the
// same structures.  This also simplifies the implementation of a bi-arch
// (combined (32- and 64-bit) gdb.
//
#[no_mangle]
pub unsafe extern "C" fn vr_active(target: *mut task_struct, regset: *const user_regset) -> c_int {
    int vr_active(struct task_struct *target, const struct user_regset *regset)
    {
    flush_altivec_to_thread(target);
    return target.thread.used_vr ? regset.n : 0;
    }
//
// Regardless of transactions, 'vr_state' holds the current running
// value of all the VMX registers and 'ckvr_state' holds the last
// checkpointed value of all the VMX registers for the current
// transaction to fall back on in case it aborts.
//
// Userspace interface buffer layout:
//
// struct data {
// vector128	vr[32];
// vector128	vscr;
// vector128	vrsave;
// };
//
    int vr_get(struct task_struct *target, const struct user_regset *regset,
    struct membuf to)
    {
    union {
    elf_vrreg_t reg;
    u32 word;
    } vrsave;
    flush_altivec_to_thread(target);
    BUILD_BUG_ON(offsetof(struct thread_vr_state, vscr) !=
    offsetof(struct thread_vr_state, vr[32]));
    membuf_write(&to, &target.thread.vr_state, 33 * sizeof(vector128));
//
// Copy out only the low-order word of vrsave.
//
    memset(&vrsave, 0, sizeof(vrsave));
    vrsave.word = target.thread.vrsave;
    return membuf_write(&to, &vrsave, sizeof(vrsave));
    }
//
// Regardless of transactions, 'vr_state' holds the current running
// value of all the VMX registers and 'ckvr_state' holds the last
// checkpointed value of all the VMX registers for the current
// transaction to fall back on in case it aborts.
//
// Userspace interface buffer layout:
//
// struct data {
// vector128	vr[32];
// vector128	vscr;
// vector128	vrsave;
// };
//
    int vr_set(struct task_struct *target, const struct user_regset *regset,
    unsigned int pos, unsigned int count,
    const void *kbuf, const void __user *ubuf)
    {
    int ret;
    flush_altivec_to_thread(target);
    BUILD_BUG_ON(offsetof(struct thread_vr_state, vscr) !=
    offsetof(struct thread_vr_state, vr[32]));
    ret = user_regset_copyin(&pos, &count, &kbuf, &ubuf,
    &target.thread.vr_state, 0,
    33 * sizeof(vector128));
    if (!ret && count > 0) {
//
// We use only the first word of vrsave.
//
    int start, end;
    union {
    elf_vrreg_t reg;
    u32 word;
    } vrsave;
    memset(&vrsave, 0, sizeof(vrsave));
    vrsave.word = target.thread.vrsave;
    start = 33 * sizeof(vector128);
    end = start + sizeof(vrsave);
    ret = user_regset_copyin(&pos, &count, &kbuf, &ubuf, &vrsave,
    start, end);
    if (!ret)
    target.thread.vrsave = vrsave.word;
    }
    return ret;
    }
