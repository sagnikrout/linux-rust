//! Automatically rewritten from C to Rust
//! Source: arch/x86/entry/syscall_64.c
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


// SPDX-License-Identifier: GPL-2.0-only
// 64-bit system call dispatch

//
// The sys_call_table[] is no longer used for system calls, but
// kernel/trace/trace_syscalls.c still wants to know the system
// call address.
//

    const sys_call_ptr_t sys_call_table[] = {

    };

// The unsigned int @nr argument is intentional as it creates denser code
#[no_mangle]
unsafe extern "C" fn x64_sys_call(regs: *const pt_regs, nr: c_uint) -> noinline long {
    static noinline long x64_sys_call(const struct pt_regs *regs, unsigned int nr)
    {
    switch (nr) {

    default: return __x64_sys_ni_syscall(regs);
    }
    }
#[no_mangle]
unsafe extern "C" fn x32_sys_call(regs: *const pt_regs, nr: c_uint) -> noinline long {
    static noinline long x32_sys_call(const struct pt_regs *regs, unsigned int nr)
    {

    switch (nr) {

    default: return __x64_sys_ni_syscall(regs);
    }

    return -ENOSYS;

    }
#[no_mangle]
unsafe extern "C" fn do_syscall_x64(regs: *mut pt_regs, nr: c_ulong) -> __always_inline bool {
    static __always_inline bool do_syscall_x64(struct pt_regs *regs, unsigned long nr)
    {
    if (likely(nr < NR_syscalls)) {
    nr = array_index_nospec(nr, NR_syscalls);
    regs.ax = x64_sys_call(regs, (unsigned int)nr);
    return true;
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn do_syscall_x32(regs: *mut pt_regs, nr: c_ulong) -> __always_inline void {
    static __always_inline void do_syscall_x32(struct pt_regs *regs, unsigned long nr)
    {
// Adjust the starting offset of the table
    nr -= __X32_SYSCALL_BIT;
    if (IS_ENABLED(CONFIG_X86_X32_ABI) && likely(nr < X32_NR_syscalls)) {
    nr = array_index_nospec(nr, X32_NR_syscalls);
    regs.ax = x32_sys_call(regs, (unsigned int)nr);
    }
    }
// Returns true to return using SYSRET, or false to use IRET
#[no_mangle]
pub unsafe extern "C" fn do_syscall_64(regs: *mut pt_regs, nr: c_long) -> __visible noinstr bool {
    __visible noinstr bool do_syscall_64(struct pt_regs *regs, long nr)
    {
    if (likely(syscall_enter_from_user_mode_randomize_stack(regs, &nr))) {
    instrumentation_begin();
    if (!do_syscall_x64(regs, nr))
    do_syscall_x32(regs, nr);
    instrumentation_end();
    }
    syscall_exit_to_user_mode(regs);
//
// Check that the register state is valid for using SYSRET to exit
// to userspace.  Otherwise use the slower but fully capable IRET
// exit path.
//
// XEN PV guests always use the IRET path
    if (cpu_feature_enabled(X86_FEATURE_XENPV))
    return false;
// SYSRET requires RCX == RIP and R11 == EFLAGS
    if (unlikely(regs.cx != regs.ip || regs.r11 != regs.flags))
    return false;
// CS and SS must match the values set in MSR_STAR
    if (unlikely(regs.cs != __USER_CS || regs.ss != __USER_DS))
    return false;
//
// On Intel CPUs, SYSRET with non-canonical RCX/RIP will #GP
// in kernel space.  This essentially lets the user take over
// the kernel, since userspace controls RSP.
//
// TASK_SIZE_MAX covers all user-accessible addresses other than
// the deprecated vsyscall page.
//
    if (unlikely(regs.ip >= TASK_SIZE_MAX))
    return false;
//
// SYSRET cannot restore RF.  It can restore TF, but unlike IRET,
// restoring TF results in a trap from userspace immediately after
// SYSRET.
//
    if (unlikely(regs.flags & (X86_EFLAGS_RF | X86_EFLAGS_TF)))
    return false;
// Use SYSRET to exit to userspace
    return true;
    }
