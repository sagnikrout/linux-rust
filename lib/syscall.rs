//! Automatically rewritten from C to Rust
//! Source: lib/syscall.c
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

#[no_mangle]
unsafe extern "C" fn collect_syscall(target: *mut task_struct, info: *mut syscall_info) -> c_int {
    static int collect_syscall(struct task_struct *target, struct syscall_info *info)
    {
    unsigned long args[6] = { };
    struct pt_regs *regs;
    if (!try_get_task_stack(target)) {
// Task has no stack, so the task isn't in a syscall.
    memset(info, 0, sizeof(*info));
    info.data.nr = -1;
    return 0;
    }
    regs = task_pt_regs(target);
    if (unlikely(!regs)) {
    put_task_stack(target);
    return -EAGAIN;
    }
    info.sp = user_stack_pointer(regs);
    info.data.instruction_pointer = instruction_pointer(regs);
    info.data.nr = syscall_get_nr(target, regs);
    if (info.data.nr != -1L)
    syscall_get_arguments(target, regs, args);
    info.data.args[0] = args[0];
    info.data.args[1] = args[1];
    info.data.args[2] = args[2];
    info.data.args[3] = args[3];
    info.data.args[4] = args[4];
    info.data.args[5] = args[5];
    put_task_stack(target);
    return 0;
    }
//
// task_current_syscall - Discover what a blocked task is doing.
// @target:		thread to examine
// @info:		structure with the following fields:
// .sp        - filled with user stack pointer
// .data.nr   - filled with system call number or -1
// .data.args - filled with @maxargs system call arguments
// .data.instruction_pointer - filled with user PC
//
// If @target is blocked in a system call, returns zero with @info.data.nr
// set to the call's number and @info.data.args filled in with its
// arguments. Registers not used for system call arguments may not be available
// and it is not kosher to use &struct user_regset calls while the system
// call is still in progress.  Note we may get this result if @target
// has finished its system call but not yet returned to user mode, such
// as when it's stopped for signal handling or syscall exit tracing.
//
// If @target is blocked in the kernel during a fault or exception,
// returns zero with *@info.data.nr set to -1 and does not fill in
// @info.data.args. If so, it's now safe to examine @target using
// &struct user_regset get() calls as long as we're sure @target won't return
// to user mode.
//
// Returns -%EAGAIN if @target does not remain blocked.
//
#[no_mangle]
pub unsafe extern "C" fn task_current_syscall(target: *mut task_struct, info: *mut syscall_info) -> c_int {
    int task_current_syscall(struct task_struct *target, struct syscall_info *info)
    {
    unsigned long ncsw;
    unsigned int state;
    if (target == current)
    return collect_syscall(target, info);
    state = READ_ONCE(target.__state);
    if (unlikely(!state))
    return -EAGAIN;
    ncsw = wait_task_inactive(target, state);
    if (unlikely(!ncsw) ||
    unlikely(collect_syscall(target, info)) ||
    unlikely(wait_task_inactive(target, state) != ncsw))
    return -EAGAIN;
    return 0;
    }
