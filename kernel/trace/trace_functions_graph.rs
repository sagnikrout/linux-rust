//! Automatically rewritten from C to Rust
//! Source: kernel/trace/trace_functions_graph.c
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
// Function graph tracer.
// Copyright (c) 2008-2009 Frederic Weisbecker <fweisbec@gmail.com>
// Mostly borrowed from function tracer which
// is Copyright (c) Steven Rostedt <srostedt@redhat.com>
//

// When set, irq functions might be ignored
    static int ftrace_graph_skip_irqs;
// Do not record function time when task is sleeping
    let mut fgraph_no_sleep_time = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fgraph_cpu_data {
    pub last_pid: pid_t,
    pub depth: c_int,
    pub depth_irq: c_int,
    pub ignore: c_int,
    pub enter_funcs: [c_ulong; FTRACE_RETFUNC_DEPTH],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fgraph_ent_args {
    pub ent: ftrace_graph_ent_entry,
// Force the sizeof of args[] to have FTRACE_REGS_MAX_ARGS entries
    pub args: [c_ulong; FTRACE_REGS_MAX_ARGS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fgraph_retaddr_ent_args {
    pub ent: fgraph_retaddr_ent_entry,
// Force the sizeof of args[] to have FTRACE_REGS_MAX_ARGS entries
    pub args: [c_ulong; FTRACE_REGS_MAX_ARGS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fgraph_data {
    pub cpu_data: *mut fgraph_cpu_data ,
// Place to preserve last processed entry.
    union {
    pub ent: fgraph_ent_args,
    pub rent: fgraph_retaddr_ent_args,
}

pub static mut ret: usize = 0;
    let mut failed = 0;
    let mut cpu = 0;
    };
pub const TRACE_GRAPH_INDENT: c_int = 2;
    let mut fgraph_max_depth = 0;
pub static mut tracer_opt: usize = 0;
pub static mut tracer_flags: usize = 0;
#[no_mangle]
unsafe extern "C" fn tracer_flags_is_set(tr: *mut trace_array, flags: u32) -> bool {
    return (tr.current_trace_flags.val & flags) == flags;
    }
//
// DURATION column is being also used to display IRQ signs,
// following values are used by print_graph_irq and others
// to fill in space into DURATION column.
//
    enum {
    FLAGS_FILL_FULL  = 1 << TRACE_GRAPH_PRINT_FILL_SHIFT,
    FLAGS_FILL_START = 2 << TRACE_GRAPH_PRINT_FILL_SHIFT,
    FLAGS_FILL_END   = 3 << TRACE_GRAPH_PRINT_FILL_SHIFT,
    };
// forward_decl: print_graph_duration;
#[no_mangle]
pub unsafe extern "C" fn __graph_entry(tr: *mut trace_array, trace: *mut ftrace_graph_ent, trace_ctx: c_uint, fregs: *mut ftrace_regs) -> c_int {
pub static mut event: *mut c_void = core::ptr::null_mut();
    let mut buffer = tr.array_buffer.buffer;
pub static mut entry: *mut c_void = core::ptr::null_mut();
    let mut size = 0;
// If fregs is defined, add FTRACE_REGS_MAX_ARGS long size words
    size = sizeof!(*entry) + (FTRACE_REGS_MAX_ARGS * !!fregs * sizeof!(long));
    event = trace_buffer_lock_reserve(buffer, TRACE_GRAPH_ENT, size, trace_ctx);
    if (!event) {
    return 0;
    }
    entry = ring_buffer_event_data(event);
    entry.graph_ent = *trace;

    if (fregs) {
    for (int i = 0; i < FTRACE_REGS_MAX_ARGS; i++) {
    entry.args[i] = ftrace_regs_get_argument(fregs, i);
    }
    }

    trace_buffer_unlock_commit_nostack(buffer, event);
    return 1;
    }
#[no_mangle]
pub unsafe extern "C" fn __trace_graph_entry(tr: *mut trace_array, trace: *mut ftrace_graph_ent, trace_ctx: c_uint) -> c_int {
    return __graph_entry(tr, trace, trace_ctx, core::ptr::null_mut());
    }

#[no_mangle]
pub unsafe extern "C" fn __trace_graph_retaddr_entry(tr: *mut trace_array, trace: *mut ftrace_graph_ent, trace_ctx: c_uint, retaddr: c_ulong, fregs: *mut ftrace_regs) -> c_int {
pub static mut event: *mut c_void = core::ptr::null_mut();
    let mut buffer = tr.array_buffer.buffer;
pub static mut entry: *mut c_void = core::ptr::null_mut();
    let mut size = 0;
// If fregs is defined, add FTRACE_REGS_MAX_ARGS long size words
    size = sizeof!(*entry) + (FTRACE_REGS_MAX_ARGS * !!fregs * sizeof!(long));
    event = trace_buffer_lock_reserve(buffer, TRACE_GRAPH_RETADDR_ENT,
    size, trace_ctx);
    if (!event) {
    return 0;
    }
    entry	= ring_buffer_event_data(event);
    entry.graph_rent.ent = *trace;
    entry.graph_rent.retaddr = retaddr;

    if (fregs) {
    for (int i = 0; i < FTRACE_REGS_MAX_ARGS; i++) {
    entry.args[i] = ftrace_regs_get_argument(fregs, i);
    }
    }

    trace_buffer_unlock_commit_nostack(buffer, event);
    return 1;
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: __trace_graph_retaddr_entry
pub unsafe extern "C" fn __trace_graph_retaddr_entry_dup(tr: *mut trace_array, trace: *mut ftrace_graph_ent, trace_ctx: c_uint, retaddr: c_ulong, fregs: *mut ftrace_regs) -> c_int {
    return 1;
    }

#[no_mangle]
pub unsafe extern "C" fn ftrace_graph_ignore_irqs(tr: *mut trace_array) -> c_int {
    if (!ftrace_graph_skip_irqs || trace_recursion_test(TRACE_IRQ_BIT)) {
    return 0;
    }
    if (tracer_flags_is_set(tr, TRACE_GRAPH_PRINT_IRQS)) {
    return 0;
    }
    return in_hardirq();
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fgraph_times {
    pub calltime: c_ulonglong,
//     pub /: *mut *mut unsigned long long sleeptime; / may be optional!,
}

#[no_mangle]
pub unsafe extern "C" fn graph_entry(trace: *mut ftrace_graph_ent, gops: *mut fgraph_ops, fregs: *mut ftrace_regs) -> c_int {
    let mut task_var = fgraph_get_task_var(gops);
    let mut tr = gops.private;
pub static mut ftimes: *mut c_void = core::ptr::null_mut();
    let mut trace_ctx = 0;
pub static mut ret: c_int = 0;
    if (*task_var & TRACE_GRAPH_NOTRACE) {
    return 0;
    }
//
// Do not trace a function if it's filtered by set_graph_notrace.
// Make the index of ret stack negative to indicate that it should
// ignore further functions.  But it needs its own ret stack entry
// to recover the original index in order to continue tracing after
// returning from the function.
//
    if (ftrace_graph_notrace_addr(trace.func)) {
// task_var |= TRACE_GRAPH_NOTRACE;
//
// Need to return 1 to have the return called
// that will clear the NOTRACE bit.
//
    return 1;
    }
    if (ftrace_graph_ignore_func(gops, trace)) {
    return 0;
    }
    if (ftrace_graph_ignore_irqs(tr)) {
    return 0;
    }
    if (fgraph_no_sleep_time &&
    !tracer_flags_is_set(tr, TRACE_GRAPH_SLEEP_TIME)) {
    ftimes = fgraph_reserve_data(gops.idx, sizeof!(*ftimes));
    if (ftimes) {
    ftimes.sleeptime = current.ftrace_sleeptime;
    }
    } else {
// Only need to record the calltime
    ftimes = fgraph_reserve_data(gops.idx, sizeof!(ftimes.calltime));
    }
    if (!ftimes) {
    return 0;
    }
    ftimes.calltime = trace_clock_local();
//
// Stop here if tracing_threshold is set. We only write function return
// events to the ring buffer.
//
    if (tracing_thresh) {
    return 1;
    }
    trace_ctx = tracing_gen_ctx();
    if (IS_ENABLED!(CONFIG_FUNCTION_GRAPH_RETADDR) &&
    tracer_flags_is_set(tr, TRACE_GRAPH_PRINT_RETADDR)) {
pub static mut retaddr: c_ulong = 0;
    ret = __trace_graph_retaddr_entry(tr, trace, trace_ctx,
    retaddr, fregs);
    } else {
    ret = __graph_entry(tr, trace, trace_ctx, fregs);
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn trace_graph_entry(trace: *mut ftrace_graph_ent, gops: *mut fgraph_ops, fregs: *mut ftrace_regs) -> c_int {
    return graph_entry(trace, gops, core::ptr::null_mut());
    }
#[no_mangle]
pub unsafe extern "C" fn trace_graph_entry_args(trace: *mut ftrace_graph_ent, gops: *mut fgraph_ops, fregs: *mut ftrace_regs) -> c_int {
    return graph_entry(trace, gops, fregs);
    }
#[no_mangle]
pub unsafe extern "C" fn __trace_graph_function(tr: *mut trace_array, ip: c_ulong, trace_ctx: c_uint) {
pub static mut time: u64 = 0;
pub static mut ftrace_graph_ent: usize = 0;
pub static mut ftrace_graph_ret: usize = 0;
    __trace_graph_entry(tr, &ent, trace_ctx);
    __trace_graph_return(tr, &ret, trace_ctx, time, time);
    }
#[no_mangle]
pub unsafe extern "C" fn trace_graph_function(tr: *mut trace_array, ip: c_ulong, parent_ip: c_ulong, trace_ctx: c_uint) {
    __trace_graph_function(tr, ip, trace_ctx);
    }
#[no_mangle]
pub unsafe extern "C" fn __trace_graph_return(tr: *mut trace_array, trace: *mut ftrace_graph_ret, trace_ctx: c_uint, calltime: u64, rettime: u64) {
pub static mut event: *mut c_void = core::ptr::null_mut();
    let mut buffer = tr.array_buffer.buffer;
pub static mut entry: *mut c_void = core::ptr::null_mut();
    event = trace_buffer_lock_reserve(buffer, TRACE_GRAPH_RET,
    sizeof!(*entry), trace_ctx);
    if (!event) {
    return;
    }
    entry	= ring_buffer_event_data(event);
    entry.ret				= *trace;
    entry.calltime				= calltime;
    entry.rettime				= rettime;
    trace_buffer_unlock_commit_nostack(buffer, event);
    }
#[no_mangle]
pub unsafe extern "C" fn handle_nosleeptime(tr: *mut trace_array, trace: *mut ftrace_graph_ret, ftimes: *mut fgraph_times, size: c_int) {
    if (size < sizeof!(*ftimes)) {
    return;
    }
    if (!fgraph_no_sleep_time || tracer_flags_is_set(tr, TRACE_GRAPH_SLEEP_TIME)) {
    return;
    }
    ftimes.calltime += current.ftrace_sleeptime - ftimes.sleeptime;
    }
#[no_mangle]
pub unsafe extern "C" fn trace_graph_return(trace: *mut ftrace_graph_ret, gops: *mut fgraph_ops, fregs: *mut ftrace_regs) {
    let mut task_var = fgraph_get_task_var(gops);
    let mut tr = gops.private;
pub static mut ftimes: *mut c_void = core::ptr::null_mut();
    let mut trace_ctx = 0;
    u64 calltime, rettime;
    let mut size = 0;
    rettime = trace_clock_local();
    ftrace_graph_addr_finish(gops, trace);
    if (*task_var & TRACE_GRAPH_NOTRACE) {
// task_var &= ~TRACE_GRAPH_NOTRACE;
    return;
    }
    ftimes = fgraph_retrieve_data(gops.idx, &size);
    if (!ftimes) {
    return;
    }
    handle_nosleeptime(tr, trace, ftimes, size);
    calltime = ftimes.calltime;
    trace_ctx = tracing_gen_ctx();
    __trace_graph_return(tr, trace, trace_ctx, calltime, rettime);
    }
#[no_mangle]
pub unsafe extern "C" fn trace_graph_thresh_return(trace: *mut ftrace_graph_ret, gops: *mut fgraph_ops, fregs: *mut ftrace_regs) {
    let mut task_var = fgraph_get_task_var(gops);
pub static mut ftimes: *mut c_void = core::ptr::null_mut();
pub static mut tr: *mut c_void = core::ptr::null_mut();
    let mut trace_ctx = 0;
    u64 calltime, rettime;
    let mut size = 0;
    rettime = trace_clock_local();
    ftrace_graph_addr_finish(gops, trace);
    if (*task_var & TRACE_GRAPH_NOTRACE) {
// task_var &= ~TRACE_GRAPH_NOTRACE;
    return;
    }
    ftimes = fgraph_retrieve_data(gops.idx, &size);
    if (!ftimes) {
    return;
    }
    tr = gops.private;
    handle_nosleeptime(tr, trace, ftimes, size);
    calltime = ftimes.calltime;
    if (tracing_thresh && (rettime - calltime < tracing_thresh)) {
    return;
    }
    trace_ctx = tracing_gen_ctx();
    __trace_graph_return(tr, trace, trace_ctx, calltime, rettime);
    }
pub static mut fgraph_ops: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn allocate_fgraph_ops(tr: *mut trace_array, ops: *mut ftrace_ops) -> c_int {
pub static mut gops: *mut c_void = core::ptr::null_mut();
    gops = kzalloc_obj(*gops);
    if (!gops) {
    return -ENOMEM;
    }
    gops.entryfunc = &trace_graph_entry;
    gops.retfunc = &trace_graph_return;
    tr.gops = gops;
    gops.private = tr;
    fgraph_init_ops(&gops.ops, ops);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn free_fgraph_ops(tr: *mut trace_array) {
    kfree(tr.gops);
    }
#[no_mangle]
pub unsafe extern "C" fn init_array_fgraph_ops(tr: *mut trace_array, ops: *mut ftrace_ops) -> __init void {
    tr.gops = &funcgraph_ops;
    funcgraph_ops.private = tr;
    fgraph_init_ops(&tr.gops.ops, ops);
    }
#[no_mangle]
unsafe extern "C" fn graph_trace_init(tr: *mut trace_array) -> c_int {
    let mut ret = 0;
    if (tracer_flags_is_set(tr, TRACE_GRAPH_ARGS)) {
    tr.gops.entryfunc = trace_graph_entry_args;
    }
    else {
    tr.gops.entryfunc = trace_graph_entry;
    }
    if (tracing_thresh) {
    tr.gops.retfunc = trace_graph_thresh_return;
    }
    else {
    tr.gops.retfunc = trace_graph_return;
    }
    if (!tracer_flags_is_set(tr, TRACE_GRAPH_PRINT_IRQS)) {
    ftrace_graph_skip_irqs += 1;
    }
    if (!tracer_flags_is_set(tr, TRACE_GRAPH_SLEEP_TIME)) {
    fgraph_no_sleep_time += 1;
    }
// Make gops functions visible before we start tracing
    smp_mb();
    ret = register_ftrace_graph(tr.gops);
    if (ret) {
    return ret;
    }
    tracing_start_cmdline_record();
    return 0;
    }
pub static mut graph_trace: usize = 0;
#[no_mangle]
unsafe extern "C" fn ftrace_graph_trace_args(tr: *mut trace_array, set: c_int) -> c_int {
    let mut entry;
    if (set) {
    entry = trace_graph_entry_args;
    }
    else {
    entry = trace_graph_entry;
    }
// See if there's any changes
    if (tr.gops.entryfunc == entry) {
    return 0;
    }
    unregister_ftrace_graph(tr.gops);
    tr.gops.entryfunc = entry;
// Make gops functions visible before we start tracing
    smp_mb();
    return register_ftrace_graph(tr.gops);
    }
#[no_mangle]
unsafe extern "C" fn graph_trace_reset(tr: *mut trace_array) {
    if (!tracer_flags_is_set(tr, TRACE_GRAPH_PRINT_IRQS)) {
    ftrace_graph_skip_irqs -= 1;
    }
    if (WARN_ON_ONCE!(ftrace_graph_skip_irqs < 0)) {
    ftrace_graph_skip_irqs = 0;
    }
    if (!tracer_flags_is_set(tr, TRACE_GRAPH_SLEEP_TIME)) {
    fgraph_no_sleep_time -= 1;
    }
    if (WARN_ON_ONCE!(fgraph_no_sleep_time < 0)) {
    fgraph_no_sleep_time = 0;
    }
    tracing_stop_cmdline_record();
    unregister_ftrace_graph(tr.gops);
    }
#[no_mangle]
unsafe extern "C" fn graph_trace_update_thresh(tr: *mut trace_array) -> c_int {
    graph_trace_reset(tr);
    return graph_trace_init(tr);
    }
    static int max_bytes_for_cpu;
#[no_mangle]
unsafe extern "C" fn print_graph_cpu(s: *mut trace_seq, cpu: c_int) {
//
// Start with a space character - to make it stand out
// to the right a bit when trace output is pasted into
// email:
//
    trace_seq_printf(s, " %*d) ", max_bytes_for_cpu, cpu);
    }
pub const TRACE_GRAPH_PROCINFO_LENGTH: c_int = 14;
#[no_mangle]
unsafe extern "C" fn print_graph_proc(s: *mut trace_seq, pid: pid_t) {
    char comm[TASK_COMM_LEN];
// sign + log10(MAX_INT) + '\0'
    char pid_str[12];
pub static mut spaces: c_int = 0;
    let mut len = 0;
    let mut i = 0;
    trace_find_cmdline(pid, comm);
    comm[7] = '\0';
    sprintf(pid_str, "%d", pid);
// 1 stands for the "-" character
    len = strlen(comm) + strlen(pid_str) + 1;
    if (len < TRACE_GRAPH_PROCINFO_LENGTH) {
    spaces = TRACE_GRAPH_PROCINFO_LENGTH - len;
    }
// First spaces to align center
    for (i = 0; i < spaces / 2; i++) {
    trace_seq_putc(s, ' ');
    }
    trace_seq_printf(s, "%s-%s", comm, pid_str);
// Last spaces to align center
    for (i = 0; i < spaces - (spaces / 2); i++) {
    trace_seq_putc(s, ' ');
    }
    }
#[no_mangle]
unsafe extern "C" fn print_graph_lat_fmt(s: *mut trace_seq, entry: *mut trace_entry) {
    trace_seq_putc(s, ' ');
    trace_print_lat_fmt(s, entry);
    trace_seq_puts(s, " | ");
    }
// If the pid changed since the last trace, output this event
#[no_mangle]
pub unsafe extern "C" fn verif_pid(s: *mut trace_seq, pid: pid_t, cpu: c_int, data: *mut fgraph_data) {
    let mut prev_pid = 0;
pub static mut last_pid: *mut c_void = core::ptr::null_mut();
    if (!data) {
    return;
    }
    last_pid = &(per_cpu_ptr(data.cpu_data, cpu).last_pid);
    if (*last_pid == pid) {
    return;
    }
    prev_pid = *last_pid;
// last_pid = pid;
    if (prev_pid == -1) {
    return;
    }
//
// Context-switch trace line:
    ------------------------------------------
    | 1)  migration/0--1  =>  sshd-1755
    ------------------------------------------
//
    trace_seq_puts(s, " ------------------------------------------\n");
    print_graph_cpu(s, cpu);
    print_graph_proc(s, prev_pid);
    trace_seq_puts(s, " => ");
    print_graph_proc(s, pid);
    trace_seq_puts(s, "\n ------------------------------------------\n\n");
    }
#[no_mangle]
pub unsafe extern "C" fn get_return_for_leaf(iter: *mut trace_iterator, curr: *mut ftrace_graph_ent_entry) -> *mut c_void {
    let mut data = iter.private;
    let mut ring_iter = core::ptr::null_mut();
pub static mut event: *mut c_void = core::ptr::null_mut();
pub static mut next: *mut c_void = core::ptr::null_mut();
//
// If the previous output failed to write to the seq buffer,
// then we just reuse the data from before.
//
    if (data && data.failed) {
    curr = &data.ent.ent;
    next = &data.ret;
    } else {
    ring_iter = trace_buffer_iter(iter, iter.cpu);
// First peek to compare current entry and the next one
    if (ring_iter) {
    event = ring_buffer_iter_peek(ring_iter, core::ptr::null_mut());
    }
    else {
//
// We need to consume the current entry to see
// the next one.
//
    ring_buffer_consume(iter.array_buffer.buffer, iter.cpu,
    core::ptr::null_mut(), core::ptr::null_mut());
    event = ring_buffer_peek(iter.array_buffer.buffer, iter.cpu,
    core::ptr::null_mut(), core::ptr::null_mut());
    }
    if (!event) {
    return core::ptr::null_mut();
    }
    next = ring_buffer_event_data(event);
    if (data) {
//
// Save current and next entries for later reference
// if the output fails.
//
pub static mut size: c_int = 0;
    memcpy(&data.rent, curr, size);
//
// If the next event is not a return type, then
// we only care about what type it is. Otherwise we can
// safely copy the entire event.
//
    if (next.ent.type == TRACE_GRAPH_RET) {
    data.ret = *next;
    }
    else {
    data.ret.ent.type = next.ent.type;
    }
    }
    }
    if (next.ent.type != TRACE_GRAPH_RET) {
    return core::ptr::null_mut();
    }
    if (curr.ent.pid != next.ent.pid ||
    curr.graph_ent.func != next.ret.func) {
    return core::ptr::null_mut();
    }
// this is a leaf, now advance the iterator
    if (ring_iter) {
    ring_buffer_iter_advance(ring_iter);
    }
    return next;
    }
#[no_mangle]
unsafe extern "C" fn print_graph_abs_time(t: u64, s: *mut trace_seq) {
    let mut usecs_rem = 0;
    usecs_rem = do_div(t, NSEC_PER_SEC);
    usecs_rem /= 1000;
    trace_seq_printf(s, "%5lu.%06lu |  ",
    (unsigned long)t, usecs_rem);
    }
#[no_mangle]
pub unsafe extern "C" fn print_graph_rel_time(iter: *mut trace_iterator, s: *mut trace_seq) {
    unsigned long long usecs;
    usecs = iter.ts - iter.array_buffer.time_start;
    do_div(usecs, NSEC_PER_USEC);
    trace_seq_printf(s, "%9llu us |  ", usecs);
    }
#[no_mangle]
pub unsafe extern "C" fn print_graph_irq(iter: *mut trace_iterator, addr: c_ulong, type: trace_type, cpu: c_int, pid: pid_t, flags: u32) {
    let mut tr = iter.tr;
    let mut s = &iter.seq;
    let mut ent = iter.ent;
    addr += iter.tr.text_delta;
    if (addr < (unsigned long)__irqentry_text_start ||
    addr >= (unsigned long)__irqentry_text_end) {
    return;
    }
    if (tr.trace_flags & TRACE_ITER(CONTEXT_INFO)) {
// Absolute time
    if (flags & TRACE_GRAPH_PRINT_ABS_TIME) {
    print_graph_abs_time(iter.ts, s);
    }
// Relative time
    if (flags & TRACE_GRAPH_PRINT_REL_TIME) {
    print_graph_rel_time(iter, s);
    }
// Cpu
    if (flags & TRACE_GRAPH_PRINT_CPU) {
    print_graph_cpu(s, cpu);
    }
// Proc
    if (flags & TRACE_GRAPH_PRINT_PROC) {
    print_graph_proc(s, pid);
    trace_seq_puts(s, " | ");
    }
// Latency format
    if (tr.trace_flags & TRACE_ITER(LATENCY_FMT)) {
    print_graph_lat_fmt(s, ent);
    }
    }
// No overhead
    print_graph_duration(tr, 0, s, flags | FLAGS_FILL_START);
    if (type == TRACE_GRAPH_ENT) {
    trace_seq_puts(s, "==========>");
    }
    else {
    trace_seq_puts(s, "<==========");
    }
    print_graph_duration(tr, 0, s, flags | FLAGS_FILL_END);
    trace_seq_putc(s, '\n');
    }
#[no_mangle]
pub unsafe extern "C" fn trace_print_graph_duration(duration: unsigned long long, s: *mut trace_seq) {
pub static mut nsecs_rem: c_ulong = 0;
// log10(ULONG_MAX) + '\0'
    char usecs_str[21];
    char nsecs_str[5];
    let mut len = 0;
    let mut i = 0;
    sprintf(usecs_str, "%lu", (unsigned long) duration);
// Print msecs
    trace_seq_printf(s, "%s", usecs_str);
    len = strlen(usecs_str);
// Print nsecs (we don't want to exceed 7 numbers)
    if (len < 7) {
pub static mut slen: usize = 0;
    snprintf(nsecs_str, slen, "%03lu", nsecs_rem);
    trace_seq_printf(s, ".%s", nsecs_str);
    len += strlen(nsecs_str) + 1;
    }
    trace_seq_puts(s, " us ");
// Print remaining spaces to fit the row's width
    for (i = len; i < 8; i++) {
    trace_seq_putc(s, ' ');
    }
    }
#[no_mangle]
pub unsafe extern "C" fn print_graph_duration(tr: *mut trace_array, duration: unsigned long long, s: *mut trace_seq, flags: u32) {
    if (!(flags & TRACE_GRAPH_PRINT_DURATION) ||
    !(tr.trace_flags & TRACE_ITER(CONTEXT_INFO))) {
    return;
    }
// No real adata, just filling the column with spaces
    match (flags & TRACE_GRAPH_PRINT_FILL_MASK) {
    FLAGS_FILL_FULL => {
    trace_seq_puts(s, "              |  ");
    return;
    }
    FLAGS_FILL_START => {
    trace_seq_puts(s, "  ");
    return;
    }
    FLAGS_FILL_END => {
    trace_seq_puts(s, " |");
    return;
    }
    }
// Signal a overhead of time execution to the output
    if (flags & TRACE_GRAPH_PRINT_OVERHEAD) {
    trace_seq_printf(s, "%c ", trace_find_mark(duration));
    }
    else {
    trace_seq_puts(s, "  ");
    }
    trace_print_graph_duration(duration, s);
    trace_seq_puts(s, "|  ");
    }

pub const __TRACE_GRAPH_PRINT_RETVAL: c_int = 0;

#[no_mangle]
pub unsafe extern "C" fn print_graph_retaddr(s: *mut trace_seq, entry: *mut fgraph_retaddr_ent_entry, trace_flags: u32, comment: bool) {
    if (comment) {
    trace_seq_puts(s, " //");
    }
    trace_seq_puts(s, " <-");
    seq_print_ip_sym_offset(s, entry.graph_rent.retaddr, trace_flags);
    if (comment) {
    trace_seq_puts(s, " */");
    }
    }

pub const __TRACE_GRAPH_PRINT_RETADDR: c_int = 0;

#[no_mangle]
pub unsafe extern "C" fn print_graph_retval(s: *mut trace_seq, entry: *mut ftrace_graph_ent_entry, graph_ret: *mut ftrace_graph_ret, func: *mut c_void, opt_flags: u32, trace_flags: u32, args_size: c_int) {
pub static mut err_code: c_ulong = 0;
pub static mut retval: c_ulong = 0;
pub static mut print_retaddr: bool = false;
pub static mut print_retval: bool = false;
pub static mut hex_format: bool = false;

    retval = graph_ret.retval;
    print_retval = !!(opt_flags & TRACE_GRAPH_PRINT_RETVAL);

    print_retaddr = !!(opt_flags & TRACE_GRAPH_PRINT_RETADDR);

    if (print_retval && retval && !hex_format) {
// Check if the return value matches the negative format
    if (IS_ENABLED!(CONFIG_64BIT) && (retval & BIT(31)) &&
    (((u64)retval) >> 32) == 0) {
    err_code = sign_extend64(retval, 31);
    } else {
    err_code = retval;
    }
    if (!IS_ERR_VALUE(err_code)) {
    err_code = 0;
    }
    }
    if (entry) {
    if (entry.ent.type != TRACE_GRAPH_RETADDR_ENT) {
    print_retaddr = false;
    }
    trace_seq_printf(s, "%ps", func);
    if (args_size >= FTRACE_REGS_MAX_ARGS * sizeof!(long)) {
    print_function_args(s, FGRAPH_ENTRY_ARGS(entry), (unsigned long)func);
    trace_seq_putc(s, ';');
    } else {
    trace_seq_puts(s, "();");
    }
    if (print_retval || print_retaddr) {
    trace_seq_puts(s, " //");
    }
    } else {
    print_retaddr = false;
    trace_seq_printf(s, "} // %ps", func);
    }
    if (print_retaddr) {
    print_graph_retaddr(s, entry,
    trace_flags, false);
    }
    if (print_retval) {
    if (hex_format || (err_code == 0)) {
    trace_seq_printf(s, " ret=0x%lx", retval);
    }
    else {
    trace_seq_printf(s, " ret=%ld", err_code);
    }
    }
    if (!entry || print_retval || print_retaddr) {
    trace_seq_puts(s, " */");
    }
    }

    do {} while (0)

// Case of a leaf function on its call entry
    static enum print_line_t
    print_graph_entry_leaf(trace_iterator *iter, ftrace_graph_ent_entry *entry, ftrace_graph_ret_entry *ret_entry, trace_seq *s, u32 flags)
    {
    let mut data = iter.private;
    let mut tr = iter.tr;
pub static mut graph_ret: *mut c_void = core::ptr::null_mut();
pub static mut call: *mut c_void = core::ptr::null_mut();
    unsigned long long duration;
    let mut ret_func = 0;
    let mut args_size = 0;
pub static mut cpu: c_int = 0;
    let mut i = 0;
    args_size = iter.ent_size - offsetof(ftrace_graph_ent_entry, args);
    graph_ret = &ret_entry.ret;
    call = &entry.graph_ent;
    duration = ret_entry.rettime - ret_entry.calltime;
    if (data) {
pub static mut cpu_data: *mut c_void = core::ptr::null_mut();
    cpu_data = per_cpu_ptr(data.cpu_data, cpu);
//
// Comments display at + 1 to depth. Since
// this is a leaf function, keep the comments
// equal to this depth.
//
    cpu_data.depth = call.depth - 1;
// No need to keep this function around for this depth
    if (call.depth < FTRACE_RETFUNC_DEPTH &&
    !WARN_ON_ONCE!(call.depth < 0)) {
    cpu_data.enter_funcs[call.depth] = 0;
    }
    }
// Overhead and duration
    print_graph_duration(tr, duration, s, flags);
// Function
    for (i = 0; i < call.depth * TRACE_GRAPH_INDENT; i++) {
    trace_seq_putc(s, ' ');
    }
    ret_func = graph_ret.func + iter.tr.text_delta;
//
// Write out the function return value or return address
//
    if (flags & (__TRACE_GRAPH_PRINT_RETVAL | __TRACE_GRAPH_PRINT_RETADDR)) {
    print_graph_retval(s, entry, graph_ret,
    graph_ret.func + iter.tr.text_delta,
    flags, tr.trace_flags, args_size);
    } else {
    trace_seq_printf(s, "%ps", ret_func);
    if (args_size >= FTRACE_REGS_MAX_ARGS * sizeof!(long)) {
    print_function_args(s, FGRAPH_ENTRY_ARGS(entry), ret_func);
    trace_seq_putc(s, ';');
    } else {
    trace_seq_puts(s, "();");
    }
    }
    trace_seq_putc(s, '\n');
    print_graph_irq(iter, graph_ret.func, TRACE_GRAPH_RET,
    cpu, iter.ent.pid, flags);
    return trace_handle_return(s);
    }
    static enum print_line_t
    print_graph_entry_nested(trace_iterator *iter, ftrace_graph_ent_entry *entry, trace_seq *s, int cpu, u32 flags)
    {
    let mut call = &entry.graph_ent;
    let mut data = iter.private;
    let mut tr = iter.tr;
    let mut func = 0;
    let mut args_size = 0;
    let mut i = 0;
    if (data) {
pub static mut cpu_data: *mut c_void = core::ptr::null_mut();
pub static mut cpu: c_int = 0;
    cpu_data = per_cpu_ptr(data.cpu_data, cpu);
    cpu_data.depth = call.depth;
// Save this function pointer to see if the exit matches
    if (call.depth < FTRACE_RETFUNC_DEPTH &&
    !WARN_ON_ONCE!(call.depth < 0)) {
    cpu_data.enter_funcs[call.depth] = call.func;
    }
    }
// No time
    print_graph_duration(tr, 0, s, flags | FLAGS_FILL_FULL);
// Function
    for (i = 0; i < call.depth * TRACE_GRAPH_INDENT; i++) {
    trace_seq_putc(s, ' ');
    }
    func = call.func + iter.tr.text_delta;
    trace_seq_printf(s, "%ps", func);
    args_size = iter.ent_size - offsetof(ftrace_graph_ent_entry, args);
    if (args_size >= FTRACE_REGS_MAX_ARGS * sizeof!(long)) {
    print_function_args(s, FGRAPH_ENTRY_ARGS(entry), func);
    }
    else {
    trace_seq_puts(s, "()");
    }
    trace_seq_puts(s, " {");
    if (flags & __TRACE_GRAPH_PRINT_RETADDR  &&
    entry.ent.type == TRACE_GRAPH_RETADDR_ENT) {
    print_graph_retaddr(s, entry,
    tr.trace_flags, true);
    }
    trace_seq_putc(s, '\n');
    if (trace_seq_has_overflowed(s)) {
    return TRACE_TYPE_PARTIAL_LINE;
    }
//
// we already consumed the current entry to check the next one
// and see if this is a leaf.
//
    return TRACE_TYPE_NO_CONSUME;
    }
#[no_mangle]
pub unsafe extern "C" fn print_graph_prologue(iter: *mut trace_iterator, s: *mut trace_seq, type: c_int, addr: c_ulong, flags: u32) {
    let mut data = iter.private;
    let mut ent = iter.ent;
    let mut tr = iter.tr;
pub static mut cpu: c_int = 0;
// Pid
    verif_pid(s, ent.pid, cpu, data);
    if (type) {
// Interrupt
    print_graph_irq(iter, addr, type, cpu, ent.pid, flags);
    }
    if (!(tr.trace_flags & TRACE_ITER(CONTEXT_INFO))) {
    return;
    }
// Absolute time
    if (flags & TRACE_GRAPH_PRINT_ABS_TIME) {
    print_graph_abs_time(iter.ts, s);
    }
// Relative time
    if (flags & TRACE_GRAPH_PRINT_REL_TIME) {
    print_graph_rel_time(iter, s);
    }
// Cpu
    if (flags & TRACE_GRAPH_PRINT_CPU) {
    print_graph_cpu(s, cpu);
    }
// Proc
    if (flags & TRACE_GRAPH_PRINT_PROC) {
    print_graph_proc(s, ent.pid);
    trace_seq_puts(s, " | ");
    }
// Latency format
    if (tr.trace_flags & TRACE_ITER(LATENCY_FMT)) {
    print_graph_lat_fmt(s, ent);
    }
    return;
    }
//
// Entry check for irq code
//
// returns 1 if
// - we are inside irq code
// - we just entered irq code
//
// returns 0 if
// - funcgraph-interrupts option is set
// - we are not inside irq code
//
#[no_mangle]
pub unsafe extern "C" fn check_irq_entry(iter: *mut trace_iterator, flags: u32, addr: c_ulong, depth: c_int) -> c_int {
pub static mut cpu: c_int = 0;
pub static mut depth_irq: *mut c_void = core::ptr::null_mut();
    let mut data = iter.private;
    addr += iter.tr.text_delta;
//
// If we are either displaying irqs, or we got called as
// a graph event and private data does not exist,
// then we bypass the irq check.
//
    if ((flags & TRACE_GRAPH_PRINT_IRQS) ||
    (!data)) {
    return 0;
    }
    depth_irq = &(per_cpu_ptr(data.cpu_data, cpu).depth_irq);
//
// We are inside the irq code
//
    if (*depth_irq >= 0) {
    return 1;
    }
    if ((addr < (unsigned long)__irqentry_text_start) ||
    (addr >= (unsigned long)__irqentry_text_end)) {
    return 0;
    }
//
// We are entering irq code.
//
// depth_irq = depth;
    return 1;
    }
//
// Return check for irq code
//
// returns 1 if
// - we are inside irq code
// - we just left irq code
//
// returns 0 if
// - funcgraph-interrupts option is set
// - we are not inside irq code
//
#[no_mangle]
pub unsafe extern "C" fn check_irq_return(iter: *mut trace_iterator, flags: u32, depth: c_int) -> c_int {
pub static mut cpu: c_int = 0;
pub static mut depth_irq: *mut c_void = core::ptr::null_mut();
    let mut data = iter.private;
//
// If we are either displaying irqs, or we got called as
// a graph event and private data does not exist,
// then we bypass the irq check.
//
    if ((flags & TRACE_GRAPH_PRINT_IRQS) ||
    (!data)) {
    return 0;
    }
    depth_irq = &(per_cpu_ptr(data.cpu_data, cpu).depth_irq);
//
// We are not inside the irq code.
//
    if (*depth_irq == -1) {
    return 0;
    }
//
// We are inside the irq code, and this is returning entry.
// Let's not trace it and clear the entry depth, since
// we are out of irq code.
//
// This condition ensures that we 'leave the irq code' once
// we are out of the entry depth. Thus protecting us from
// the RETURN entry loss.
//
    if (*depth_irq >= depth) {
// depth_irq = -1;
    return 1;
    }
//
// We are inside the irq code, and this is not the entry.
//
    return 1;
    }
    static enum print_line_t
    print_graph_entry(ftrace_graph_ent_entry *field, trace_seq *s, trace_iterator *iter, u32 flags)
    {
    let mut data = iter.private;
pub static mut call: *mut c_void = core::ptr::null_mut();
pub static mut leaf_ret: *mut c_void = core::ptr::null_mut();
    static enum print_line_t ret;
pub static mut cpu: c_int = 0;
//
// print_graph_entry() may consume the current event,
// thus @field may become invalid, so we need to save it.
// This function is shared by ftrace_graph_ent_entry and
// fgraph_retaddr_ent_entry, the size of the latter one
// is larger, but it is very small and can be safely saved
// at the stack.
//
pub static mut entry: *mut c_void = core::ptr::null_mut();
pub static mut rentry: *mut c_void = core::ptr::null_mut();
    u8 save_buf[sizeof!(*rentry) + FTRACE_REGS_MAX_ARGS * sizeof!(long)];
// The ent_size is expected to be as big as the entry
    if (iter.ent_size > sizeof!(save_buf)) {
    iter.ent_size = sizeof!(save_buf);
    }
    entry = save_buf;
    memcpy(entry, field, iter.ent_size);
    call = &entry.graph_ent;
    if (check_irq_entry(iter, flags, call.func, call.depth)) {
    return TRACE_TYPE_HANDLED;
    }
    print_graph_prologue(iter, s, TRACE_GRAPH_ENT, call.func, flags);
    leaf_ret = get_return_for_leaf(iter, entry);
    if (leaf_ret) {
    ret = print_graph_entry_leaf(iter, entry, leaf_ret, s, flags);
    }
    else {
    ret = print_graph_entry_nested(iter, entry, s, cpu, flags);
    }
    if (data) {
//
// If we failed to write our output, then we need to make
// note of it. Because we already consumed our entry.
//
    if (s.full) {
    data.failed = 1;
    data.cpu = cpu;
    } else {
    data.failed = 0;
    }
    }
    return ret;
    }
    static enum print_line_t
    print_graph_return(ftrace_graph_ret_entry *retentry, trace_seq *s, trace_entry *ent, trace_iterator *iter,
    u32 flags)
    {
    let mut trace = &retentry.ret;
pub static mut calltime: u64 = 0;
pub static mut rettime: u64 = 0;
pub static mut duration: c_ulonglong = 0;
    let mut data = iter.private;
    let mut tr = iter.tr;
    let mut func = 0;
pub static mut pid: pid_t = 0;
pub static mut cpu: c_int = 0;
pub static mut func_match: c_int = 1;
    let mut i = 0;
    func = trace.func + iter.tr.text_delta;
    if (check_irq_return(iter, flags, trace.depth)) {
    return TRACE_TYPE_HANDLED;
    }
    if (data) {
pub static mut cpu_data: *mut c_void = core::ptr::null_mut();
pub static mut cpu: c_int = 0;
    cpu_data = per_cpu_ptr(data.cpu_data, cpu);
//
// Comments display at + 1 to depth. This is the
// return from a function, we now want the comments
// to display at the same level of the bracket.
//
    cpu_data.depth = trace.depth - 1;
    if (trace.depth < FTRACE_RETFUNC_DEPTH &&
    !WARN_ON_ONCE!(trace.depth < 0)) {
    if (cpu_data.enter_funcs[trace.depth] != trace.func) {
    func_match = 0;
    }
    cpu_data.enter_funcs[trace.depth] = 0;
    }
    }
    print_graph_prologue(iter, s, 0, 0, flags);
// Overhead and duration
    print_graph_duration(tr, duration, s, flags);
// Closing brace
    for (i = 0; i < trace.depth * TRACE_GRAPH_INDENT; i++) {
    trace_seq_putc(s, ' ');
    }
//
// Always write out the function name and its return value if the
// funcgraph-retval option is enabled.
//
    if (flags & __TRACE_GRAPH_PRINT_RETVAL) {
    print_graph_retval(s, core::ptr::null_mut(), trace, func, flags,
    tr.trace_flags, 0);
    } else {
//
// If the return function does not have a matching entry,
// then the entry was lost. Instead of just printing
// the '}' and letting the user guess what function this
// belongs to, write out the function name. Always do
// that if the funcgraph-tail option is enabled.
//
    if (func_match && !(flags & TRACE_GRAPH_PRINT_TAIL)) {
    trace_seq_putc(s, '}');
    }
    else {
    trace_seq_printf(s, "} /* %ps */", func);
    }
    }
    trace_seq_putc(s, '\n');
// Overrun
    if (flags & TRACE_GRAPH_PRINT_OVERRUN) {
    trace_seq_printf(s, " (Overruns: %u)\n",
    trace.overrun);
    }
    print_graph_irq(iter, trace.func, TRACE_GRAPH_RET,
    cpu, pid, flags);
    return trace_handle_return(s);
    }
    static enum print_line_t
    print_graph_comment(trace_seq *s, trace_entry *ent, trace_iterator *iter, u32 flags)
    {
    let mut tr = iter.tr;
pub static mut sym_flags: c_ulong = 0;
    let mut data = iter.private;
pub static mut event: *mut c_void = core::ptr::null_mut();
pub static mut depth: c_int = 0;
    let mut ret = 0;
    let mut i = 0;
    if (data) {
    depth = per_cpu_ptr(data.cpu_data, iter.cpu).depth;
    }
    print_graph_prologue(iter, s, 0, 0, flags);
// No time
    print_graph_duration(tr, 0, s, flags | FLAGS_FILL_FULL);
// Indentation
    if (depth > 0) {
    for (i = 0; i < (depth + 1) * TRACE_GRAPH_INDENT; i++)
    }
    trace_seq_putc(s, ' ');
// The comment
    trace_seq_puts(s, "// ");
    match (iter.ent.type) {
    TRACE_BPUTS => {
    ret = trace_print_bputs_msg_only(iter);
    if (ret != TRACE_TYPE_HANDLED) {
    return ret;
    }
    // break;
    }
    TRACE_BPRINT => {
    ret = trace_print_bprintk_msg_only(iter);
    if (ret != TRACE_TYPE_HANDLED) {
    return ret;
    }
    // break;
    }
    TRACE_PRINT => {
    ret = trace_print_printk_msg_only(iter);
    if (ret != TRACE_TYPE_HANDLED) {
    return ret;
    }
    // break;
    }
    _ => {
    event = ftrace_find_event(ent.type);
    if (!event) {
    return TRACE_TYPE_UNHANDLED;
    }
    ret = event.funcs.trace(iter, sym_flags, event);
    if (ret != TRACE_TYPE_HANDLED) {
    return ret;
    }
    }
    }
    if (trace_seq_has_overflowed(s)) {
// goto;
    }
// Strip ending newline
    if (s.buffer[s.seq.len - 1] == '\n') {
    s.buffer[s.seq.len - 1] = '\0';
    s.seq.len -= 1;
    }
    trace_seq_puts(s, " */\n");
// label;
    return trace_handle_return(s);
    }
    enum print_line_t
    print_graph_function_flags(trace_iterator *iter, u32 flags)
    {
pub static mut field: *mut c_void = core::ptr::null_mut();
    let mut data = iter.private;
    let mut entry = iter.ent;
    let mut s = &iter.seq;
pub static mut cpu: c_int = 0;
    let mut ret = 0;
    if (data && per_cpu_ptr(data.cpu_data, cpu).ignore) {
    per_cpu_ptr(data.cpu_data, cpu).ignore = 0;
    return TRACE_TYPE_HANDLED;
    }
//
// If the last output failed, there's a possibility we need
// to print out the missing entry which would never go out.
//
    if (data && data.failed) {
    field = &data.ent.ent;
    iter.cpu = data.cpu;
    ret = print_graph_entry(field, s, iter, flags);
    if (ret == TRACE_TYPE_HANDLED && iter.cpu != cpu) {
    per_cpu_ptr(data.cpu_data, iter.cpu).ignore = 1;
    ret = TRACE_TYPE_NO_CONSUME;
    }
    iter.cpu = cpu;
    return ret;
    }
    match (entry.type) {
    TRACE_GRAPH_ENT => {
    trace_assign_type(field, entry);
    return print_graph_entry(field, s, iter, flags);
    }
    }

    case TRACE_GRAPH_RETADDR_ENT: {
//
// ftrace_graph_ent_entry and fgraph_retaddr_ent_entry have
// similar functions and memory layouts. The only difference
// is that the latter one has an extra retaddr member, so
// they can share most of the logic.
//
pub static mut rfield: *mut c_void = core::ptr::null_mut();
    trace_assign_type(rfield, entry);
    return print_graph_entry(rfield,
    s, iter, flags);
    }

    case TRACE_GRAPH_RET: {
pub static mut field: *mut c_void = core::ptr::null_mut();
    trace_assign_type(field, entry);
    return print_graph_return(field, s, entry, iter, flags);
    }
    case TRACE_STACK:
    case TRACE_FN:
// dont trace stack and functions as comments
    return TRACE_TYPE_UNHANDLED;
// label;
    return print_graph_comment(s, entry, iter, flags);
    }
    return TRACE_TYPE_HANDLED;
    }
    static enum print_line_t
    print_graph_function(trace_iterator *iter)
    {
    let mut tr = iter.tr;
    return print_graph_function_flags(iter, tr.current_trace_flags.val);
    }
    static enum print_line_t
    print_graph_function_event(trace_iterator *iter, int flags, trace_event *event)
    {
    return print_graph_function(iter);
    }
#[no_mangle]
unsafe extern "C" fn print_lat_header(s: *mut seq_file, flags: u32) {
    static const char spaces[] = "                "	/* 16 spaces */
    "    "					/* 4 spaces */
    "                 ";			/* 17 spaces */
pub static mut size: c_int = 0;
    if (flags & TRACE_GRAPH_PRINT_ABS_TIME) {
    size += 16;
    }
    if (flags & TRACE_GRAPH_PRINT_REL_TIME) {
    size += 16;
    }
    if (flags & TRACE_GRAPH_PRINT_CPU) {
    size += 4;
    }
    if (flags & TRACE_GRAPH_PRINT_PROC) {
    size += 17;
    }
    seq_printf(s, "#%.*s  _-----=> irqs-off        \n", size, spaces);
    seq_printf(s, "#%.*s / _----=> need-resched    \n", size, spaces);
    seq_printf(s, "#%.*s| / _---=> hardirq/softirq \n", size, spaces);
    seq_printf(s, "#%.*s|| / _--=> preempt-depth   \n", size, spaces);
    seq_printf(s, "#%.*s||| /                      \n", size, spaces);
    }
#[no_mangle]
pub unsafe extern "C" fn __print_graph_headers_flags(tr: *mut trace_array, s: *mut seq_file, flags: u32) {
pub static mut lat: c_int = 0;
    if (lat) {
    print_lat_header(s, flags);
    }
// 1st line
    seq_putc(s, '#');
    if (flags & TRACE_GRAPH_PRINT_ABS_TIME) {
    seq_puts(s, "     TIME       ");
    }
    if (flags & TRACE_GRAPH_PRINT_REL_TIME) {
    seq_puts(s, "   REL TIME     ");
    }
    if (flags & TRACE_GRAPH_PRINT_CPU) {
    seq_puts(s, " CPU");
    }
    if (flags & TRACE_GRAPH_PRINT_PROC) {
    seq_puts(s, "  TASK/PID       ");
    }
    if (lat) {
    seq_puts(s, "||||   ");
    }
    if (flags & TRACE_GRAPH_PRINT_DURATION) {
    seq_puts(s, "  DURATION   ");
    }
    seq_puts(s, "               FUNCTION CALLS\n");
// 2nd line
    seq_putc(s, '#');
    if (flags & TRACE_GRAPH_PRINT_ABS_TIME) {
    seq_puts(s, "      |         ");
    }
    if (flags & TRACE_GRAPH_PRINT_REL_TIME) {
    seq_puts(s, "      |         ");
    }
    if (flags & TRACE_GRAPH_PRINT_CPU) {
    seq_puts(s, " |  ");
    }
    if (flags & TRACE_GRAPH_PRINT_PROC) {
    seq_puts(s, "   |    |        ");
    }
    if (lat) {
    seq_puts(s, "||||   ");
    }
    if (flags & TRACE_GRAPH_PRINT_DURATION) {
    seq_puts(s, "   |   |      ");
    }
    seq_puts(s, "               |   |   |   |\n");
    }
#[no_mangle]
unsafe extern "C" fn print_graph_headers(s: *mut seq_file) {
    let mut iter = s.private;
    let mut tr = iter.tr;
    print_graph_headers_flags(s, tr.current_trace_flags.val);
    }
#[no_mangle]
pub unsafe extern "C" fn print_graph_headers_flags(s: *mut seq_file, flags: u32) {
    let mut iter = s.private;
    let mut tr = iter.tr;
    if (!(tr.trace_flags & TRACE_ITER(CONTEXT_INFO))) {
    return;
    }
    if (tr.trace_flags & TRACE_ITER(LATENCY_FMT)) {
// print nothing if the buffers are empty
    if (trace_empty(iter)) {
    return;
    }
    print_trace_header(s, iter);
    }
    __print_graph_headers_flags(tr, s, flags);
    }
#[no_mangle]
pub unsafe extern "C" fn graph_trace_open(iter: *mut trace_iterator) {
// pid and depth on the last trace processed
pub static mut data: *mut c_void = core::ptr::null_mut();
    let mut gfpflags;
    let mut cpu = 0;
    iter.private = core::ptr::null_mut();
// We can be called in atomic context via ftrace_dump()
    gfpflags = (in_atomic() || irqs_disabled()) ? GFP_ATOMIC : GFP_KERNEL;
    data = kzalloc_obj(*data, gfpflags);
    if (!data) {
// goto;
    }
    data.cpu_data = alloc_percpu_gfp(fgraph_cpu_data, gfpflags);
    if (!data.cpu_data) {
// goto;
    }
    for_each_possible_cpu(cpu) {
    let mut pid = &(per_cpu_ptr(data.cpu_data, cpu).last_pid);
    let mut depth = &(per_cpu_ptr(data.cpu_data, cpu).depth);
    let mut ignore = &(per_cpu_ptr(data.cpu_data, cpu).ignore);
    let mut depth_irq = &(per_cpu_ptr(data.cpu_data, cpu).depth_irq);
// pid = -1;
// depth = 0;
// ignore = 0;
// depth_irq = -1;
    }
    iter.private = data;
    return;
// label;
    kfree(data);
// label;
    pr_warn!("function graph tracer: not enough memory\n");
    }
#[no_mangle]
pub unsafe extern "C" fn graph_trace_close(iter: *mut trace_iterator) {
    let mut data = iter.private;
    if (data) {
    free_percpu(data.cpu_data);
    kfree(data);
    iter.private = core::ptr::null_mut();
    }
    }
#[no_mangle]
pub unsafe extern "C" fn func_graph_set_flag(tr: *mut trace_array, old_flags: u32, bit: u32, set: c_int) -> c_int {
//
// The function profiler gets updated even if function graph
// isn't the current tracer. Handle it separately.
//

    if (bit == TRACE_GRAPH_SLEEP_TIME && (tr.flags & TRACE_ARRAY_FL_GLOBAL) &&
    !!set == fprofile_no_sleep_time) {
    if (set) {
    fgraph_no_sleep_time -= 1;
    if (WARN_ON_ONCE!(fgraph_no_sleep_time < 0)) {
    fgraph_no_sleep_time = 0;
    }
    fprofile_no_sleep_time = false;
    } else {
    fgraph_no_sleep_time += 1;
    fprofile_no_sleep_time = true;
    }
    }

// Do nothing if the current tracer is not this tracer
    if (tr.current_trace != &graph_trace) {
    return 0;
    }
// Do nothing if already set.
    if (!!set == !!(tr.current_trace_flags.val & bit)) {
    return 0;
    }
    match (bit) {
    TRACE_GRAPH_SLEEP_TIME => {
    if (set) {
    fgraph_no_sleep_time -= 1;
    if (WARN_ON_ONCE!(fgraph_no_sleep_time < 0)) {
    fgraph_no_sleep_time = 0;
    }
    } else {
    fgraph_no_sleep_time += 1;
    }
    // break;
    }
    TRACE_GRAPH_PRINT_IRQS => {
    if (set) {
    ftrace_graph_skip_irqs -= 1;
    }
    else {
    ftrace_graph_skip_irqs += 1;
    }
    if (WARN_ON_ONCE!(ftrace_graph_skip_irqs < 0)) {
    ftrace_graph_skip_irqs = 0;
    }
    // break;
    }
    TRACE_GRAPH_ARGS => {
    return ftrace_graph_trace_args(tr, set);
    }
    }
    return 0;
    }
pub static mut trace_event_functions: usize = 0;
pub static mut trace_event: usize = 0;

pub static mut trace_event: usize = 0;

pub static mut trace_event: usize = 0;
    static struct tracer graph_trace __tracer_data = {
    .name		= "function_graph",
    .update_thresh	= graph_trace_update_thresh,
    .open		= graph_trace_open,
    .pipe_open	= graph_trace_open,
    .close		= graph_trace_close,
    .pipe_close	= graph_trace_close,
    .init		= graph_trace_init,
    .reset		= graph_trace_reset,
    .print_line	= print_graph_function,
    .print_header	= print_graph_headers,
    .default_flags	= &tracer_flags,
    .set_flag	= func_graph_set_flag,
    .allow_instances = true,

    .selftest	= trace_selftest_startup_function_graph,

    };
#[no_mangle]
pub unsafe extern "C" fn graph_depth_write(filp: *mut file, ubuf: *mut c_char, cnt: size_t, ppos: *mut loff_t) -> ssize_t {
    let mut val = 0;
    let mut ret = 0;
    ret = kstrtoul_from_user(ubuf, cnt, 10, &val);
    if (ret) {
    return ret;
    }
    fgraph_max_depth = val;
// ppos += cnt;
    return cnt;
    }
#[no_mangle]
pub unsafe extern "C" fn graph_depth_read(filp: *mut file, ubuf: *mut c_char, cnt: size_t, ppos: *mut loff_t) -> ssize_t {
    char buf[15]; /* More than enough to hold UINT_MAX + "\n"*/
    let mut n = 0;
    n = sprintf(buf, "%d\n", fgraph_max_depth);
    return simple_read_from_buffer(ubuf, cnt, ppos, buf, n);
    }
pub static mut file_operations: usize = 0;
#[no_mangle]
unsafe extern "C" fn init_graph_tracefs() -> __init int {
    let mut ret = 0;
    ret = tracing_init_dentry();
    if (ret) {
    return 0;
    }
    trace_create_file("max_graph_depth", TRACE_MODE_WRITE, core::ptr::null_mut(),
    core::ptr::null_mut(), &graph_depth_fops);
    return 0;
    }
    fs_initcall!(init_graph_tracefs);
#[no_mangle]
unsafe extern "C" fn init_graph_trace() -> __init int {
    max_bytes_for_cpu = snprintf(core::ptr::null_mut(), 0, "%u", nr_cpu_ids - 1);
    if (!register_trace_event(&graph_trace_entry_event)) {
    pr_warn!("Warning: could not register graph trace events\n");
    return 1;
    }

    if (!register_trace_event(&graph_trace_retaddr_entry_event)) {
    pr_warn!("Warning: could not register graph trace retaddr events\n");
    return 1;
    }

    if (!register_trace_event(&graph_trace_ret_event)) {
    pr_warn!("Warning: could not register graph trace events\n");
    return 1;
    }
    return register_tracer(&graph_trace);
    }
    core_initcall!(init_graph_trace);