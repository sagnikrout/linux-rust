//! Automatically rewritten from C to Rust
//! Source: kernel/kstack_erase.c
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

macro_rules! EXPORT_SYMBOL { ($($tt:tt)*) => {}; }
macro_rules! EXPORT_SYMBOL_GPL { ($($tt:tt)*) => {}; }
macro_rules! MODULE_LICENSE { ($($tt:tt)*) => {}; }
macro_rules! MODULE_AUTHOR { ($($tt:tt)*) => {}; }
macro_rules! MODULE_DESCRIPTION { ($($tt:tt)*) => {}; }
macro_rules! MODULE_ALIAS { ($($tt:tt)*) => {}; }
macro_rules! module_init { ($($tt:tt)*) => {}; }
macro_rules! module_exit { ($($tt:tt)*) => {}; }
macro_rules! early_initcall { ($($tt:tt)*) => {}; }
macro_rules! core_initcall { ($($tt:tt)*) => {}; }
macro_rules! postcore_initcall { ($($tt:tt)*) => {}; }
macro_rules! arch_initcall { ($($tt:tt)*) => {}; }
macro_rules! subsys_initcall { ($($tt:tt)*) => {}; }
macro_rules! fs_initcall { ($($tt:tt)*) => {}; }
macro_rules! device_initcall { ($($tt:tt)*) => {}; }
macro_rules! late_initcall { ($($tt:tt)*) => {}; }
macro_rules! __setup { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_MUTEX { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_SPINLOCK { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_PER_CPU { ($($tt:tt)*) => {}; }
macro_rules! DECLARE_PER_CPU { ($($tt:tt)*) => {}; }
macro_rules! DEFINE { ($($tt:tt)*) => {}; }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct seq_file { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct task_struct { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct user_namespace { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cred { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct file { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inode { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct notifier_block { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct raw_notifier_head { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct proc_dir_entry { pub _opaque: [u8; 0] }

pub type pid_type = c_int;
pub type cpu_pm_event = c_int;
pub type spinlock_t = u32;
pub type raw_spinlock_t = u32;
pub type kernel_cap_t = u64;
pub type cap_user_header_t = *mut c_void;
pub type cap_user_data_t = *mut c_void;
pub type async_cookie_t = u64;
pub type atomic_long_t = core::sync::atomic::AtomicI64;


























// SPDX-License-Identifier: GPL-2.0
//
// This code fills the used part of the kernel stack with a poison value
// before returning to userspace. It's part of the STACKLEAK feature
// ported from grsecurity/PaX.
//
// Author: Alexander Popov <alex.popov@linux.com>
//
// KSTACK_ERASE reduces the information which kernel stack leak bugs can
// reveal and blocks some uninitialized stack variable attacks.
//
// static DEFINE_STATIC_KEY_FALSE(stack_erasing_bypass);

#[no_mangle]
pub unsafe extern "C" fn stack_erasing_sysctl() {
pub static mut ret: c_int = 0;
pub static mut state: c_int = !static_branch_unlikely(&stack_erasing_bypass);
pub static mut prev_state: c_int = state;
pub static mut table_copy: ctl_table = *table;
    table_copy.data = &state;
    ret = proc_dointvec_minmax(&table_copy, write, buffer, lenp, ppos);
    state = !!state;
    if (ret || !write || state == prev_state) {
    return ret;
    }
    if (state) {
    static_branch_disable(&stack_erasing_bypass);
    }
    else {
    static_branch_enable(&stack_erasing_bypass);
    }
    pr_warn("stackleak: kernel stack erasing is %s\n",
    str_enabled_disabled(state));
    return ret;
    }
pub static mut ctl_table: usize = 0;
#[no_mangle]
unsafe extern "C" fn stackleak_sysctls_init() -> c_int {
    register_sysctl_init("kernel", stackleak_sysctls);
    return 0;
    }
// late_initcall;

    static __always_inline void __stackleak_poison(unsigned long erase_low,
    unsigned long erase_high,
    unsigned long poison)
    {
    while (erase_low < erase_high) {
// erase_low = poison;
    erase_low += sizeof(unsigned long);
    }
    }

#[no_mangle]
unsafe extern "C" fn __stackleak_erase(on_task_stack: bool) -> __always_inline void {
pub static mut task_stack_low: c_ulong = stackleak_task_low_bound(current);
pub static mut task_stack_high: c_ulong = stackleak_task_high_bound(current);
    unsigned long erase_low, erase_high;
    erase_low = stackleak_find_top_of_poison(task_stack_low,
    current.lowest_stack);

    current.prev_lowest_stack = erase_low;

//
// Write poison to the task's stack between 'erase_low' and
// 'erase_high'.
//
// If we're running on a different stack (e.g. an entry trampoline
// stack) we can erase everything below the pt_regs at the top of the
// task stack.
//
// If we're running on the task stack itself, we must not clobber any
// stack used by this function and its caller. We assume that this
// function has a fixed-size stack frame, and the current stack pointer
// doesn't change while we write poison.
//
    if (on_task_stack) {
    erase_high = current_stack_pointer;
    }
    else {
    erase_high = task_stack_high;
    }
    __stackleak_poison(erase_low, erase_high, KSTACK_ERASE_POISON);
// Reset the 'lowest_stack' value for the next syscall
    current.lowest_stack = task_stack_high;
    }
//
// Erase and poison the portion of the task stack used since the last erase.
// Can be called from the task stack or an entry stack when the task stack is
// no longer in use.
//
#[no_mangle]
pub unsafe extern "C" fn stackleak_erase() -> asmlinkage void noinstr {
    if (skip_erasing()) {
    return;
    }
    __stackleak_erase(on_thread_stack());
    }
//
// Erase and poison the portion of the task stack used since the last erase.
// Can only be called from the task stack.
//
#[no_mangle]
pub unsafe extern "C" fn stackleak_erase_on_task_stack() -> asmlinkage void noinstr {
    if (skip_erasing()) {
    return;
    }
    __stackleak_erase(true);
    }
//
// Erase and poison the portion of the task stack used since the last erase.
// Can only be called from a stack other than the task stack.
//
#[no_mangle]
pub unsafe extern "C" fn stackleak_erase_off_task_stack() -> asmlinkage void noinstr {
    if (skip_erasing()) {
    return;
    }
    __stackleak_erase(false);
    }
#[no_mangle]
pub unsafe extern "C" fn __sanitizer_cov_stack_depth() -> void __used __no_caller_saved_registers noinstr {
pub static mut sp: c_ulong = current_stack_pointer;
//
// Having CONFIG_KSTACK_ERASE_TRACK_MIN_SIZE larger than
// KSTACK_ERASE_SEARCH_DEPTH makes the poison search in
// stackleak_erase() unreliable. Let's prevent that.
//
// BUILD_BUG_ON;
// 'lowest_stack' should be aligned on the register width boundary
    sp = ALIGN(sp, sizeof(unsigned long));
    if (sp < current.lowest_stack &&
    sp >= stackleak_task_low_bound(current)) {
    current.lowest_stack = sp;
    }
    }
// EXPORT_SYMBOL;