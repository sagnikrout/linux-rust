//! Automatically rewritten from C to Rust
//! Source: kernel/events/callchain.c
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


// SPDX-License-Identifier: GPL-2.0
//
// Performance events callchain code, extracted from core.c:
//
// Copyright (C) 2008 Linutronix GmbH, Thomas Gleixner <tglx@kernel.org>
// Copyright (C) 2008-2011 Red Hat, Inc., Ingo Molnar
// Copyright (C) 2008-2011 Red Hat, Inc., Peter Zijlstra
// Copyright  ©  2009 Paul Mackerras, IBM Corp. <paulus@au1.ibm.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct callchain_cpus_entries {
    pub rcu_head: rcu_head,
    pub cpu_entries: [*mut perf_callchain_entry; ],
}

pub static mut : int sysctl_perf_event_max_stack = 0;
pub static mut : int sysctl_perf_event_max_contexts_per_stack = 0;
pub static mut six_hundred_forty_kb: int = 0;
#[no_mangle]
pub unsafe extern "C" fn perf_callchain_entry__sizeof() -> usize {
    return (sizeof!(perf_callchain_entry) +
    sizeof!(__u64) * (sysctl_perf_event_max_stack +
    sysctl_perf_event_max_contexts_per_stack));
    }
pub static mut u8: usize = 0;
    static atomic_t nr_callchain_events;
pub static mut callchain_mutex: usize = 0;
pub static mut callchain_cpus_entries: *mut c_void = core::ptr::null_mut();
    __weak void perf_callchain_kernel(perf_callchain_entry_ctx *entry, pt_regs *regs)
    {
    }
    __weak void perf_callchain_user(perf_callchain_entry_ctx *entry, pt_regs *regs)
    {
    }
#[no_mangle]
unsafe extern "C" fn release_callchain_buffers_rcu(head: *mut rcu_head) {
pub static mut entries: *mut c_void = core::ptr::null_mut();
    let mut cpu = 0;
    entries = container_of!(head, callchain_cpus_entries, rcu_head);
    for_each_possible_cpu(cpu) {
    kfree(entries.cpu_entries[cpu]);
    }
    kfree(entries);
    }
#[no_mangle]
unsafe extern "C" fn release_callchain_buffers() {
pub static mut entries: *mut c_void = core::ptr::null_mut();
    entries = callchain_cpus_entries;
    RCU_INIT_POINTER(callchain_cpus_entries, core::ptr::null_mut());
    call_rcu(&entries.rcu_head, release_callchain_buffers_rcu);
    }
#[no_mangle]
unsafe extern "C" fn alloc_callchain_buffers() -> c_int {
    let mut cpu = 0;
    let mut size = 0;
pub static mut entries: *mut c_void = core::ptr::null_mut();
//
// We can't use the percpu allocation API for data that can be
// accessed from NMI. Use a temporary manual per cpu allocation
// until that gets sorted out.
//
    size = offsetof(callchain_cpus_entries, cpu_entries[nr_cpu_ids]);
    entries = kzalloc(size, GFP_KERNEL);
    if (!entries) {
    return -ENOMEM;
    }
    size = perf_callchain_entry__sizeof() * PERF_NR_CONTEXTS;
    for_each_possible_cpu(cpu) {
    entries.cpu_entries[cpu] = kmalloc_node(size, GFP_KERNEL,
    cpu_to_node(cpu));
    if (!entries.cpu_entries[cpu]) {
// goto;
    }
    }
    rcu_assign_pointer(callchain_cpus_entries, entries);
    return 0;
// label;
    for_each_possible_cpu(cpu) {
    kfree(entries.cpu_entries[cpu]);
    }
    kfree(entries);
    return -ENOMEM;
    }
#[no_mangle]
pub unsafe extern "C" fn get_callchain_buffers(event_max_stack: c_int) -> c_int {
pub static mut err: c_int = 0;
    let mut count = 0;
    mutex_lock(&callchain_mutex);
    count = atomic_inc_return(&nr_callchain_events);
    if (WARN_ON_ONCE!(count < 1)) {
    err = -EINVAL;
// goto;
    }
//
// If requesting per event more than the global cap,
// return a different error to help userspace figure
// this out.
//
// And also do it here so that we have &callchain_mutex held.
//
    if (event_max_stack > sysctl_perf_event_max_stack) {
    err = -EOVERFLOW;
// goto;
    }
    if (count == 1) {
    err = alloc_callchain_buffers();
    }
// label;
    if (err) {
    atomic_dec(&nr_callchain_events);
    }
    mutex_unlock(&callchain_mutex);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn put_callchain_buffers() {
    if (atomic_dec_and_mutex_lock(&nr_callchain_events, &callchain_mutex)) {
    release_callchain_buffers();
    mutex_unlock(&callchain_mutex);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn get_callchain_entry(rctx: *mut c_int) -> *mut c_void {
    let mut cpu = 0;
pub static mut entries: *mut c_void = core::ptr::null_mut();
// rctx = get_recursion_context(this_cpu_ptr(callchain_recursion));
    if (*rctx == -1) {
    return core::ptr::null_mut();
    }
    entries = rcu_dereference(callchain_cpus_entries);
    if (!entries) {
    put_recursion_context(this_cpu_ptr(callchain_recursion), *rctx);
    return core::ptr::null_mut();
    }
    cpu = smp_processor_id();
    return ((entries.cpu_entries[cpu]) +
    (*rctx * perf_callchain_entry__sizeof()));
    }
#[no_mangle]
pub unsafe extern "C" fn put_callchain_entry(rctx: c_int) {
    put_recursion_context(this_cpu_ptr(callchain_recursion), rctx);
    }
#[no_mangle]
pub unsafe extern "C" fn fixup_uretprobe_trampoline_entries(entry: *mut perf_callchain_entry, start_entry_idx: c_int) {

    let mut utask = current.utask;
pub static mut ri: *mut c_void = core::ptr::null_mut();
    __u64 *cur_ip, *last_ip, tramp_addr;
    if (likely(!utask || !utask.return_instances)) {
    return;
    }
    cur_ip = &entry.ip[start_entry_idx];
    last_ip = &entry.ip[entry.nr - 1];
    ri = utask.return_instances;
    tramp_addr = uprobe_get_trampoline_vaddr();
//
// If there are pending uretprobes for the current thread, they are
// recorded in a list inside utask->return_instances; each such
// pending uretprobe replaces traced user function's return address on
// the stack, so when stack trace is captured, instead of seeing
// actual function's return address, we'll have one or many uretprobe
// trampoline addresses in the stack trace, which are not helpful and
// misleading to users.
// So here we go over the pending list of uretprobes, and each
// encountered trampoline address is replaced with actual return
// address.
//
    while (ri && cur_ip <= last_ip) {
    if (*cur_ip == tramp_addr) {
// cur_ip = ri->orig_ret_vaddr;
    ri = ri.next;
    }
    cur_ip += 1;
    }

    }
#[no_mangle]
pub unsafe extern "C" fn get_perf_callchain(regs: *mut pt_regs, kernel: bool, user: bool, max_stack: u32, crosstask: bool, add_mark: bool, defer_cookie: u64) -> *mut c_void {
pub static mut entry: *mut c_void = core::ptr::null_mut();
pub static mut ctx: usize = 0;
    let mut rctx = 0;
    let mut start_entry_idx = 0;
// crosstask is not supported for user stacks
    if (crosstask && user && !kernel) {
    return core::ptr::null_mut();
    }
    entry = get_callchain_entry(&rctx);
    if (!entry) {
    return core::ptr::null_mut();
    }
    ctx.entry		= entry;
    ctx.max_stack		= max_stack;
    ctx.nr			= entry.nr = 0;
    ctx.contexts		= 0;
    ctx.contexts_maxed	= false;
    if (kernel && !user_mode(regs)) {
    if (add_mark) {
    perf_callchain_store_context(&ctx, PERF_CONTEXT_KERNEL);
    }
    perf_callchain_kernel(&ctx, regs);
    }
    if (user && !crosstask) {
    if (!user_mode(regs)) {
    if (!is_user_task(current)) {
// goto;
    }
    regs = task_pt_regs(current);
    }
    if (defer_cookie) {
//
// Foretell the coming of PERF_RECORD_CALLCHAIN_DEFERRED
// which can be stitched to this one, and add
// the cookie after it (it will be cut off when the
// user stack is copied to the callchain).
//
    perf_callchain_store_context(&ctx, PERF_CONTEXT_USER_DEFERRED);
    perf_callchain_store_context(&ctx, defer_cookie);
// goto;
    }
    if (add_mark) {
    perf_callchain_store_context(&ctx, PERF_CONTEXT_USER);
    }
    start_entry_idx = entry.nr;
    perf_callchain_user(&ctx, regs);
    fixup_uretprobe_trampoline_entries(entry, start_entry_idx);
    }
// label;
    put_callchain_entry(rctx);
    return entry;
    }
#[no_mangle]
pub unsafe extern "C" fn perf_event_max_stack_handler(table: *mut ctl_table, write: c_int, buffer: *mut c_void, lenp: *mut size_t, ppos: *mut loff_t) -> c_int {
    let mut value = table.data;
pub static mut new_value: c_int = 0;
pub static mut new_table: ctl_table = 0;
    new_table.data = &new_value;
    ret = proc_dointvec_minmax(&new_table, write, buffer, lenp, ppos);
    if (ret || !write) {
    return ret;
    }
    mutex_lock(&callchain_mutex);
    if (atomic_read(&nr_callchain_events)) {
    ret = -EBUSY;
    }
    else {
// value = new_value;
    }
    mutex_unlock(&callchain_mutex);
    return ret;
    }
pub static mut ctl_table: usize = 0;
#[no_mangle]
unsafe extern "C" fn init_callchain_sysctls() -> c_int {
    register_sysctl_init("kernel", callchain_sysctl_table);
    return 0;
    }
    core_initcall!(init_callchain_sysctls);