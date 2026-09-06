//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/kernel/syscall.c
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

// Has to run notrace because it is entered not completely "reconciled"
#[no_mangle]
pub unsafe extern "C" fn system_call_exception(regs: *mut pt_regs, r0: c_ulong) -> notrace long {
    notrace long system_call_exception(struct pt_regs *regs, unsigned long r0)
    {
    long ret;
    syscall_fn f;
    if (unlikely(!syscall_enter_from_user_mode_randomize_stack(regs, &r0)))
    return syscall_get_error(current, regs);
    if (unlikely(test_and_clear_thread_flag(TIF_SYSCALL_RET)))
    return syscall_get_error(current, regs);
    if (unlikely(r0 >= NR_syscalls)) {
    if (unlikely(trap_is_unsupported_scv(regs))) {
// Unsupported scv vector
    _exception(SIGILL, regs, ILL_ILLOPC, regs.nip);
    return regs.gpr[3];
    }
    return -ENOSYS;
    }
// May be faster to do array_index_nospec?
    barrier_nospec();

// No COMPAT if we have SYSCALL_WRAPPER, see Kconfig
    f = (void *)sys_call_table[r0];
    ret = f(regs);

    if (unlikely(is_compat_task())) {
    unsigned long r3, r4, r5, r6, r7, r8;
    f = (void *)compat_sys_call_table[r0];
    r3 = regs.gpr[3] & 0x00000000ffffffffULL;
    r4 = regs.gpr[4] & 0x00000000ffffffffULL;
    r5 = regs.gpr[5] & 0x00000000ffffffffULL;
    r6 = regs.gpr[6] & 0x00000000ffffffffULL;
    r7 = regs.gpr[7] & 0x00000000ffffffffULL;
    r8 = regs.gpr[8] & 0x00000000ffffffffULL;
    ret = f(r3, r4, r5, r6, r7, r8);
    } else {
    f = (void *)sys_call_table[r0];
    ret = f(regs.gpr[3], regs.gpr[4], regs.gpr[5],
    regs.gpr[6], regs.gpr[7], regs.gpr[8]);
    }

    return ret;
    }
