//! Automatically rewritten from C to Rust
//! Source: kernel/stacktrace.c
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
// === KERNEL_MACRO_PRELUDE_START ===
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
macro_rules! ARRAY_SIZE { ($($tt:tt)*) => { 1 }; }
macro_rules! container_of { ($($tt:tt)*) => { core::ptr::null_mut() }; }
macro_rules! sizeof { ($($tt:tt)*) => { 0usize }; }
macro_rules! IS_ENABLED { ($($tt:tt)*) => { false }; }
macro_rules! DECLARE_WORK { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_WAKE_Q { ($($tt:tt)*) => {}; }
macro_rules! LLIST_HEAD { ($($tt:tt)*) => {}; }
macro_rules! LIST_HEAD { ($($tt:tt)*) => {}; }
macro_rules! SET_UID { ($($tt:tt)*) => {}; }
macro_rules! SET_GID { ($($tt:tt)*) => {}; }
macro_rules! list_for_each_entry { ($($tt:tt)*) => { if false }; }
macro_rules! list_for_each_entry_safe { ($($tt:tt)*) => { if false }; }
macro_rules! llist_for_each_entry_safe { ($($tt:tt)*) => { if false }; }
macro_rules! pr_info_once { ($($tt:tt)*) => {}; }
macro_rules! pr_info { ($($tt:tt)*) => {}; }
macro_rules! pr_warn { ($($tt:tt)*) => {}; }
macro_rules! pr_err { ($($tt:tt)*) => {}; }
macro_rules! pr_debug { ($($tt:tt)*) => {}; }
macro_rules! early_param { ($($tt:tt)*) => {}; }
macro_rules! BUILD_BUG_ON { ($($tt:tt)*) => {}; }
macro_rules! WARN_ON { ($($tt:tt)*) => { false }; }
macro_rules! WARN_ON_ONCE { ($($tt:tt)*) => { false }; }
macro_rules! BUG_ON { ($($tt:tt)*) => {}; }
macro_rules! BUG { () => {}; }
macro_rules! IS_ERR { ($($tt:tt)*) => { false }; }
macro_rules! PTR_ERR { ($($tt:tt)*) => { 0 }; }
macro_rules! ERR_PTR { ($($tt:tt)*) => { core::ptr::null_mut() }; }

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
pub struct ctl_table_header { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table_root { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table_set { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct proc_dir_entry { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc_namespace { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_ipc64_perm { pub uid: uid_t, pub gid: gid_t, pub mode: mode_t, pub key: key_t, pub cuid: uid_t, pub cgid: gid_t, pub seq: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_ipc_perm { pub uid: uid_t, pub gid: gid_t, pub mode: mode_t, pub key: key_t, pub cuid: uid_t, pub cgid: gid_t, pub seq: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc64_perm { pub uid: uid_t, pub gid: gid_t, pub mode: mode_t, pub key: key_t, pub cuid: uid_t, pub cgid: gid_t, pub seq: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kern_ipc_perm { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc_params { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_queue { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_msg { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_msgseg { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_sender { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_receiver { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sem { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sembuf { pub sem_num: u16, pub sem_op: i16, pub sem_flg: i16 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sem_array { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct shmid_kernel { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct shm_file_data { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wake_q_head { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct work_struct { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct llist_head { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct list_head { pub _opaque: [u8; 0] }

pub type pid_type = c_int;
pub type cpu_pm_event = c_int;
pub type spinlock_t = u32;
pub type raw_spinlock_t = u32;
pub type kernel_cap_t = u64;
pub type cap_user_header_t = *mut c_void;
pub type cap_user_data_t = *mut c_void;
pub type async_cookie_t = u64;
pub type atomic_long_t = core::sync::atomic::AtomicI64;
pub type key_t = i32;
pub type kuid_t = u32;
pub type kgid_t = u32;
pub type int = c_int;
pub type uint = c_uint;
pub type ulong = c_ulong;
pub type long = c_long;
pub type void = c_void;

// Standard Linux Error Codes
pub const EPERM: c_int = 1;
pub const ENOENT: c_int = 2;
pub const ESRCH: c_int = 3;
pub const EINTR: c_int = 4;
pub const EIO: c_int = 5;
pub const ENXIO: c_int = 6;
pub const E2BIG: c_int = 7;
pub const ENOEXEC: c_int = 8;
pub const EBADF: c_int = 9;
pub const ECHILD: c_int = 10;
pub const EAGAIN: c_int = 11;
pub const ENOMEM: c_int = 12;
pub const EACCES: c_int = 13;
pub const EFAULT: c_int = 14;
pub const EBUSY: c_int = 16;
pub const EEXIST: c_int = 17;
pub const EXDEV: c_int = 18;
pub const ENODEV: c_int = 19;
pub const ENOTDIR: c_int = 20;
pub const EISDIR: c_int = 21;
pub const EINVAL: c_int = 22;
pub const ENFILE: c_int = 23;
pub const EMFILE: c_int = 24;
pub const ENOSPC: c_int = 28;
pub const EROFS: c_int = 30;
pub const EIDRM: c_int = 43;
pub const EOPNOTSUPP: c_int = 95;
pub const ENOTSUPP: c_int = 524;

// Standard Memory Constants
pub const PAGE_SHIFT: usize = 12;
pub const PAGE_SIZE: usize = 1 << PAGE_SHIFT;
pub const GFP_KERNEL: c_uint = 0xcc0;
pub const GFP_ATOMIC: c_uint = 0x80000;
pub const GFP_NOWAIT: c_uint = 0;

// Standard Core Primitives
extern "C" {
    pub static current: *mut task_struct;
    pub fn printk(fmt: *const c_char, ...) -> c_int;
    pub fn rcu_read_lock();
    pub fn rcu_read_unlock();
    pub fn copy_from_user(to: *mut c_void, from: *const c_void, n: usize) -> bool;
    pub fn copy_to_user(to: *mut c_void, from: *const c_void, n: usize) -> bool;
    pub fn kmalloc(size: usize, flags: c_uint) -> *mut c_void;
    pub fn kfree(ptr: *mut c_void);
}
// === KERNEL_MACRO_PRELUDE_END ===

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



// SPDX-License-Identifier: GPL-2.0-only
//
// kernel/stacktrace.c
//
// Stack trace management functions
//
// Copyright (C) 2006 Red Hat, Inc., Ingo Molnar <mingo@redhat.com>
//

//
// stack_trace_print - Print the entries in the stack trace
// @entries:	Pointer to storage array
// @nr_entries:	Number of entries in the storage array
// @spaces:	Number of leading spaces to print
//
#[no_mangle]
pub unsafe extern "C" fn stack_trace_print(entries: *mut c_ulong, nr_entries: c_uint, spaces: c_int) {
    let mut i = 0;
    if (WARN_ON!(!entries)) {
    return;
    }
    for (i = 0; i < nr_entries; i++) {
    printk("%*c%pS\n", 1 + spaces, ' ', entries[i]);
    }
    }
    EXPORT_SYMBOL_GPL(stack_trace_print);
//
// stack_trace_snprint - Print the entries in the stack trace into a buffer
// @buf:	Pointer to the print buffer
// @size:	Size of the print buffer
// @entries:	Pointer to storage array
// @nr_entries:	Number of entries in the storage array
// @spaces:	Number of leading spaces to print
//
// Return: Number of bytes printed.
//
#[no_mangle]
pub unsafe extern "C" fn stack_trace_snprint(buf: *mut c_char, size: size_t, entries: *mut c_ulong, nr_entries: c_uint, spaces: c_int) -> c_int {
    unsigned int generated, i, total = 0;
    if (WARN_ON!(!entries)) {
    return 0;
    }
    while (i < nr_entries && size) {
    generated = snprintf(buf, size, "%*c%pS\n", 1 + spaces, ' ',
    entries[i]);
    total += generated;
    if (generated >= size) {
    buf += size;
    size = 0;
    } else {
    buf += generated;
    size -= generated;
    }
    }
    return total;
    }
    EXPORT_SYMBOL_GPL(stack_trace_snprint);

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stacktrace_cookie {
    pub store: *mut c_ulong,
    pub size: c_uint,
    pub skip: c_uint,
    pub len: c_uint,
}

#[no_mangle]
unsafe extern "C" fn stack_trace_consume_entry(cookie: *mut c_void, addr: c_ulong) -> bool {
    let mut c = cookie;
    if (c.len >= c.size) {
    return false;
    }
    if (c.skip > 0) {
    c.skip -= 1;
    return true;
    }
    c.store[c.len++] = addr;
    return c.len < c.size;
    }
#[no_mangle]
unsafe extern "C" fn stack_trace_consume_entry_nosched(cookie: *mut c_void, addr: c_ulong) -> bool {
    if (in_sched_functions(addr)) {
    return true;
    }
    return stack_trace_consume_entry(cookie, addr);
    }
//
// stack_trace_save - Save a stack trace into a storage array
// @store:	Pointer to storage array
// @size:	Size of the storage array
// @skipnr:	Number of entries to skip at the start of the stack trace
//
// Return: Number of trace entries stored.
//
#[no_mangle]
pub unsafe extern "C" fn stack_trace_save(store: *mut c_ulong, size: c_uint, skipnr: c_uint) -> c_uint {
pub static mut consume_entry: stack_trace_consume_fn = 0;
pub static mut stacktrace_cookie: usize = 0;
    arch_stack_walk(consume_entry, &c, current, core::ptr::null_mut());
    return c.len;
    }
    EXPORT_SYMBOL_GPL(stack_trace_save);
//
// stack_trace_save_tsk - Save a task stack trace into a storage array
// @tsk:	The task to examine
// @store:	Pointer to storage array
// @size:	Size of the storage array
// @skipnr:	Number of entries to skip at the start of the stack trace
//
// Return: Number of trace entries stored.
//
#[no_mangle]
pub unsafe extern "C" fn stack_trace_save_tsk(tsk: *mut task_struct, store: *mut c_ulong, size: c_uint, skipnr: c_uint) -> c_uint {
pub static mut consume_entry: stack_trace_consume_fn = 0;
pub static mut stacktrace_cookie: usize = 0;
    if (!try_get_task_stack(tsk)) {
    return 0;
    }
    arch_stack_walk(consume_entry, &c, tsk, core::ptr::null_mut());
    put_task_stack(tsk);
    return c.len;
    }
    EXPORT_SYMBOL_GPL(stack_trace_save_tsk);
//
// stack_trace_save_regs - Save a stack trace based on pt_regs into a storage array
// @regs:	Pointer to pt_regs to examine
// @store:	Pointer to storage array
// @size:	Size of the storage array
// @skipnr:	Number of entries to skip at the start of the stack trace
//
// Return: Number of trace entries stored.
//
#[no_mangle]
pub unsafe extern "C" fn stack_trace_save_regs(regs: *mut pt_regs, store: *mut c_ulong, size: c_uint, skipnr: c_uint) -> c_uint {
pub static mut consume_entry: stack_trace_consume_fn = 0;
pub static mut stacktrace_cookie: usize = 0;
    arch_stack_walk(consume_entry, &c, current, regs);
    return c.len;
    }

//
// stack_trace_save_tsk_reliable - Save task stack with verification
// @tsk:	Pointer to the task to examine
// @store:	Pointer to storage array
// @size:	Size of the storage array
//
// Return:	An error if it detects any unreliable features of the
// stack. Otherwise it guarantees that the stack trace is
// reliable and returns the number of entries stored.
//
// If the task is not 'current', the caller *must* ensure the task is inactive.
//
#[no_mangle]
pub unsafe extern "C" fn stack_trace_save_tsk_reliable(tsk: *mut task_struct, store: *mut c_ulong, size: c_uint) -> c_int {
pub static mut consume_entry: stack_trace_consume_fn = 0;
pub static mut stacktrace_cookie: usize = 0;
    let mut ret = 0;
//
// If the task doesn't have a stack (e.g., a zombie), the stack is
// "reliably" empty.
//
    if (!try_get_task_stack(tsk)) {
    return 0;
    }
    ret = arch_stack_walk_reliable(consume_entry, &c, tsk);
    put_task_stack(tsk);
    return ret ? ret : c.len;
    }

//
// stack_trace_save_user - Save a user space stack trace into a storage array
// @store:	Pointer to storage array
// @size:	Size of the storage array
//
// Return: Number of trace entries stored.
//
#[no_mangle]
pub unsafe extern "C" fn stack_trace_save_user(store: *mut c_ulong, size: c_uint) -> c_uint {
pub static mut consume_entry: stack_trace_consume_fn = 0;
pub static mut stacktrace_cookie: usize = 0;
// Trace user stack if not a kernel thread
    if (current.flags & PF_KTHREAD) {
    return 0;
    }
    arch_stack_walk_user(consume_entry, &c, task_pt_regs(current));
    return c.len;
    }

//
// Architectures that do not implement save_stack_trace_*()
// get these weak aliases and once-per-bootup warnings
// (whenever this facility is utilized - for example by procfs):
//
    __weak void
    save_stack_trace_tsk(task_struct *tsk, stack_trace *trace)
    {
    WARN_ONCE(1, "save_stack_trace_tsk() not implemented yet.\n");
    }
    __weak void
    save_stack_trace_regs(pt_regs *regs, stack_trace *trace)
    {
    WARN_ONCE(1, "save_stack_trace_regs() not implemented yet.\n");
    }
//
// stack_trace_save - Save a stack trace into a storage array
// @store:	Pointer to storage array
// @size:	Size of the storage array
// @skipnr:	Number of entries to skip at the start of the stack trace
//
// Return: Number of trace entries stored
//
#[no_mangle]
#[no_mangle]
// duplicate fn: stack_trace_save
pub unsafe extern "C" fn stack_trace_save_dup(store: *mut c_ulong, size: c_uint, skipnr: c_uint) -> c_uint {
pub static mut stack_trace: usize = 0;
    save_stack_trace(&trace);
    return trace.nr_entries;
    }
    EXPORT_SYMBOL_GPL(stack_trace_save);
//
// stack_trace_save_tsk - Save a task stack trace into a storage array
// @task:	The task to examine
// @store:	Pointer to storage array
// @size:	Size of the storage array
// @skipnr:	Number of entries to skip at the start of the stack trace
//
// Return: Number of trace entries stored
//
#[no_mangle]
#[no_mangle]
// duplicate fn: stack_trace_save_tsk
pub unsafe extern "C" fn stack_trace_save_tsk_dup(task: *mut task_struct, store: *mut c_ulong, size: c_uint, skipnr: c_uint) -> c_uint {
pub static mut stack_trace: usize = 0;
    save_stack_trace_tsk(task, &trace);
    return trace.nr_entries;
    }
    EXPORT_SYMBOL_GPL(stack_trace_save_tsk);
//
// stack_trace_save_regs - Save a stack trace based on pt_regs into a storage array
// @regs:	Pointer to pt_regs to examine
// @store:	Pointer to storage array
// @size:	Size of the storage array
// @skipnr:	Number of entries to skip at the start of the stack trace
//
// Return: Number of trace entries stored
//
#[no_mangle]
#[no_mangle]
// duplicate fn: stack_trace_save_regs
pub unsafe extern "C" fn stack_trace_save_regs_dup(regs: *mut pt_regs, store: *mut c_ulong, size: c_uint, skipnr: c_uint) -> c_uint {
pub static mut stack_trace: usize = 0;
    save_stack_trace_regs(regs, &trace);
    return trace.nr_entries;
    }

//
// stack_trace_save_tsk_reliable - Save task stack with verification
// @tsk:	Pointer to the task to examine
// @store:	Pointer to storage array
// @size:	Size of the storage array
//
// Return:	An error if it detects any unreliable features of the
// stack. Otherwise it guarantees that the stack trace is
// reliable and returns the number of entries stored.
//
// If the task is not 'current', the caller *must* ensure the task is inactive.
//
#[no_mangle]
#[no_mangle]
// duplicate fn: stack_trace_save_tsk_reliable
pub unsafe extern "C" fn stack_trace_save_tsk_reliable_dup(tsk: *mut task_struct, store: *mut c_ulong, size: c_uint) -> c_int {
pub static mut stack_trace: usize = 0;
pub static mut ret: c_int = 0;
    return ret ? ret : trace.nr_entries;
    }

//
// stack_trace_save_user - Save a user space stack trace into a storage array
// @store:	Pointer to storage array
// @size:	Size of the storage array
//
// Return: Number of trace entries stored
//
#[no_mangle]
#[no_mangle]
// duplicate fn: stack_trace_save_user
pub unsafe extern "C" fn stack_trace_save_user_dup(store: *mut c_ulong, size: c_uint) -> c_uint {
pub static mut stack_trace: usize = 0;
    save_stack_trace_user(&trace);
    return trace.nr_entries;
    }

#[no_mangle]
pub unsafe extern "C" fn in_irqentry_text(ptr: c_ulong) -> bool {
    return (ptr >= (unsigned long)&__irqentry_text_start &&
    ptr < (unsigned long)&__irqentry_text_end) ||
    (ptr >= (unsigned long)&__softirqentry_text_start &&
    ptr < (unsigned long)&__softirqentry_text_end);
    }
//
// filter_irq_stacks - Find first IRQ stack entry in trace
// @entries:	Pointer to stack trace array
// @nr_entries:	Number of entries in the storage array
//
// Return: Number of trace entries until IRQ stack starts.
//
#[no_mangle]
pub unsafe extern "C" fn filter_irq_stacks(entries: *mut c_ulong, nr_entries: c_uint) -> c_uint {
    let mut i = 0;
    while (i < nr_entries) {
    if (in_irqentry_text(entries[i])) {
// Include the irqentry function into the stack.
    return i + 1;
    }
    }
    return nr_entries;
    }
    EXPORT_SYMBOL_GPL(filter_irq_stacks);