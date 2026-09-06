//! Automatically rewritten from C to Rust
//! Source: kernel/trace/trace_sched_wakeup.c
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
// trace task wakeup timings
//
// Copyright (C) 2007-2008 Steven Rostedt <srostedt@redhat.com>
// Copyright (C) 2008 Ingo Molnar <mingo@redhat.com>
//
// Based on code from the latency_tracer, that is:
//
// Copyright (C) 2004-2006 Ingo Molnar
// Copyright (C) 2004 Nadia Yvette Chambers
//

pub static mut wakeup_trace: *mut c_void = core::ptr::null_mut();
    static int 	tracer_enabled;
pub static mut wakeup_task: *mut c_void = core::ptr::null_mut();
    static int			wakeup_cpu;
    static int			wakeup_current_cpu;
pub static mut wakeup_prio: unsigned = 0;
    static bool			wakeup_rt;
    static bool			wakeup_dl;
    static bool			tracing_dl;
    static arch_spinlock_t wakeup_lock =
    (arch_spinlock_t)__ARCH_SPIN_LOCK_UNLOCKED;
// forward_decl: wakeup_reset;
// forward_decl: __wakeup_reset;
// forward_decl: start_func_tracer;
// forward_decl: stop_func_tracer;
    static int save_flags;

    static bool function_enabled;
//
// Prologue for the wakeup function tracers.
//
// Returns 1 if it is OK to continue, and preemption
// is disabled and data->disabled is incremented.
// 0 if the trace is to be ignored, and preemption
// is not disabled and data->disabled is
// kept the same.
//
// Note, this function is also used outside this ifdef but
// inside the #ifdef of the function graph tracer below.
// This is OK, since the function graph tracer is
// dependent on the function tracer.
//
#[no_mangle]
pub unsafe extern "C" fn func_prolog_preempt_disable(tr: *mut trace_array, data: *mut *mut trace_array_cpu, trace_ctx: *mut c_uint) -> c_int {
    let mut disabled = 0;
    let mut cpu = 0;
    if (likely(!wakeup_task)) {
    return 0;
    }
// trace_ctx = tracing_gen_ctx();
    preempt_disable_notrace();
    cpu = raw_smp_processor_id();
    if (cpu != wakeup_current_cpu) {
// goto;
    }
// data = per_cpu_ptr(tr->array_buffer.data, cpu);
    disabled = local_inc_return(&(*data).disabled);
    if (unlikely(disabled != 1)) {
// goto;
    }
    return 1;
// label;
    local_dec(&(*data).disabled);
// label;
    preempt_enable_notrace();
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn wakeup_display_graph(tr: *mut trace_array, set: c_int) -> c_int {
    if (!(is_graph(tr) ^ set)) {
    return 0;
    }
    stop_func_tracer(tr, !set);
    wakeup_reset(wakeup_trace);
    tr.max_latency = 0;
    return start_func_tracer(tr, set);
    }
#[no_mangle]
pub unsafe extern "C" fn wakeup_graph_entry(trace: *mut ftrace_graph_ent, gops: *mut fgraph_ops, fregs: *mut ftrace_regs) -> c_int {
    let mut tr = wakeup_trace;
pub static mut data: *mut c_void = core::ptr::null_mut();
    let mut trace_ctx = 0;
pub static mut calltime: *mut c_void = core::ptr::null_mut();
pub static mut ret: c_int = 0;
    if (ftrace_graph_ignore_func(gops, trace)) {
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
    return 1;
    }
    if (!func_prolog_preempt_disable(tr, &data, &trace_ctx)) {
    return 0;
    }
    calltime = fgraph_reserve_data(gops.idx, sizeof!(*calltime));
    if (calltime) {
// calltime = trace_clock_local();
    ret = __trace_graph_entry(tr, trace, trace_ctx);
    }
    local_dec(&data.disabled);
    preempt_enable_notrace();
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn wakeup_graph_return(trace: *mut ftrace_graph_ret, gops: *mut fgraph_ops, fregs: *mut ftrace_regs) {
    let mut tr = wakeup_trace;
pub static mut data: *mut c_void = core::ptr::null_mut();
    let mut trace_ctx = 0;
pub static mut calltime: *mut c_void = core::ptr::null_mut();
    let mut rettime = 0;
    let mut size = 0;
    ftrace_graph_addr_finish(gops, trace);
    if (!func_prolog_preempt_disable(tr, &data, &trace_ctx)) {
    return;
    }
    rettime = trace_clock_local();
    calltime = fgraph_retrieve_data(gops.idx, &size);
    if (calltime) {
    __trace_graph_return(tr, trace, trace_ctx, *calltime, rettime);
    }
    local_dec(&data.disabled);
    preempt_enable_notrace();
    return;
    }
pub static mut fgraph_ops: usize = 0;
#[no_mangle]
unsafe extern "C" fn wakeup_trace_open(iter: *mut trace_iterator) {
    if (is_graph(iter.tr)) {
    graph_trace_open(iter);
    }
    }
#[no_mangle]
unsafe extern "C" fn wakeup_trace_close(iter: *mut trace_iterator) {
    if (iter.private) {
    graph_trace_close(iter);
    }
    }

    TRACE_GRAPH_PRINT_CPU |  
    TRACE_GRAPH_PRINT_REL_TIME | 
    TRACE_GRAPH_PRINT_DURATION | 
    TRACE_GRAPH_PRINT_OVERHEAD | 
    TRACE_GRAPH_PRINT_IRQS)
#[no_mangle]
unsafe extern "C" fn wakeup_print_line(iter: *mut trace_iterator) -> enum print_line_t {
//
// In graph mode call the graph tracer output function,
// otherwise go with the TRACE_FN event handler
//
    if (is_graph(iter.tr)) {
    return print_graph_function_flags(iter, GRAPH_TRACER_FLAGS);
    }
    return TRACE_TYPE_UNHANDLED;
    }
#[no_mangle]
unsafe extern "C" fn wakeup_print_header(s: *mut seq_file) {
    if (is_graph(wakeup_trace)) {
    print_graph_headers_flags(s, GRAPH_TRACER_FLAGS);
    }
    else {
    trace_default_header(s);
    }
    }

//
// wakeup uses its own tracer function to keep the overhead down:
//
#[no_mangle]
pub unsafe extern "C" fn wakeup_tracer_call(ip: c_ulong, parent_ip: c_ulong, op: *mut ftrace_ops, fregs: *mut ftrace_regs) {
    let mut tr = wakeup_trace;
pub static mut data: *mut c_void = core::ptr::null_mut();
    let mut flags = 0;
    let mut trace_ctx = 0;
    if (!func_prolog_preempt_disable(tr, &data, &trace_ctx)) {
    return;
    }
    local_irq_save(flags);
    trace_function(tr, ip, parent_ip, trace_ctx, fregs);
    local_irq_restore(flags);
    local_dec(&data.disabled);
    preempt_enable_notrace();
    }
#[no_mangle]
unsafe extern "C" fn register_wakeup_function(tr: *mut trace_array, graph: c_int, set: c_int) -> c_int {
    let mut ret = 0;
// 'set' is set if TRACE_ITER(FUNCTION) is about to be set
    if (function_enabled || (!set && !(tr.trace_flags & TRACE_ITER(FUNCTION)))) {
    return 0;
    }
    if (graph) {
    ret = register_ftrace_graph(&fgraph_wakeup_ops);
    }
    else {
    ret = register_ftrace_function(tr.ops);
    }
    if (!ret) {
    function_enabled = true;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn unregister_wakeup_function(tr: *mut trace_array, graph: c_int) {
    if (!function_enabled) {
    return;
    }
    if (graph) {
    unregister_ftrace_graph(&fgraph_wakeup_ops);
    }
    else {
    unregister_ftrace_function(tr.ops);
    }
    function_enabled = false;
    }
#[no_mangle]
unsafe extern "C" fn wakeup_function_set(tr: *mut trace_array, mask: u32, set: c_int) -> c_int {
    if (!(mask & TRACE_ITER(FUNCTION))) {
    return 0;
    }
    if (set) {
    register_wakeup_function(tr, is_graph(tr), 1);
    }
    else {
    unregister_wakeup_function(tr, is_graph(tr));
    }
    return 1;
    }

#[no_mangle]
unsafe extern "C" fn register_wakeup_function(tr: *mut trace_array, graph: c_int, set: c_int) -> c_int {
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn unregister_wakeup_function(tr: *mut trace_array, graph: c_int) { }
#[no_mangle]
unsafe extern "C" fn wakeup_function_set(tr: *mut trace_array, mask: u32, set: c_int) -> c_int {
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn wakeup_print_line(iter: *mut trace_iterator) -> enum print_line_t {
    return TRACE_TYPE_UNHANDLED;
    }
#[no_mangle]
pub unsafe extern "C" fn wakeup_trace_open(iter: *mut trace_iterator) { }
#[no_mangle]
pub unsafe extern "C" fn wakeup_trace_close(iter: *mut trace_iterator) { }
#[no_mangle]
unsafe extern "C" fn wakeup_print_header(s: *mut seq_file) {
    trace_default_header(s);
    }

#[no_mangle]
pub unsafe extern "C" fn __trace_function(tr: *mut trace_array, ip: c_ulong, parent_ip: c_ulong, trace_ctx: c_uint) {
    if (is_graph(tr)) {
    trace_graph_function(tr, ip, parent_ip, trace_ctx);
    }
    else {
    trace_function(tr, ip, parent_ip, trace_ctx, core::ptr::null_mut());
    }
    }
#[no_mangle]
unsafe extern "C" fn wakeup_flag_changed(tr: *mut trace_array, mask: u64, set: c_int) -> c_int {
    let mut tracer = tr.current_trace;
    if (wakeup_function_set(tr, mask, set)) {
    return 0;
    }

    if (mask & TRACE_ITER(DISPLAY_GRAPH)) {
    return wakeup_display_graph(tr, set);
    }

    return trace_keep_overwrite(tracer, mask, set);
    }
#[no_mangle]
unsafe extern "C" fn start_func_tracer(tr: *mut trace_array, graph: c_int) -> c_int {
    let mut ret = 0;
    ret = register_wakeup_function(tr, graph, 0);
    if (!ret && tracing_is_enabled()) {
    tracer_enabled = 1;
    }
    else {
    tracer_enabled = 0;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn stop_func_tracer(tr: *mut trace_array, graph: c_int) {
    tracer_enabled = 0;
    unregister_wakeup_function(tr, graph);
    }
//
// Should this new latency be reported/recorded?
//
#[no_mangle]
unsafe extern "C" fn report_latency(tr: *mut trace_array, delta: u64) -> bool {
    if (tracing_thresh) {
    if (delta < tracing_thresh) {
    return false;
    }
    } else {
    if (delta <= tr.max_latency) {
    return false;
    }
    }
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn probe_wakeup_migrate_task(ignore: *mut c_void, task: *mut task_struct, cpu: c_int) {
    if (task != wakeup_task) {
    return;
    }
    wakeup_current_cpu = cpu;
    }
#[no_mangle]
pub unsafe extern "C" fn tracing_sched_switch_trace(tr: *mut trace_array, prev: *mut task_struct, next: *mut task_struct, trace_ctx: c_uint) {
    let mut buffer = tr.array_buffer.buffer;
pub static mut event: *mut c_void = core::ptr::null_mut();
pub static mut entry: *mut c_void = core::ptr::null_mut();
    event = trace_buffer_lock_reserve(buffer, TRACE_CTX,
    sizeof!(*entry), trace_ctx);
    if (!event) {
    return;
    }
    entry	= ring_buffer_event_data(event);
    entry.prev_pid			= prev.pid;
    entry.prev_prio		= prev.prio;
    entry.prev_state		= task_state_index(prev);
    entry.next_pid			= next.pid;
    entry.next_prio		= next.prio;
    entry.next_state		= task_state_index(next);
    entry.next_cpu	= task_cpu(next);
    trace_buffer_unlock_commit(tr, buffer, event, trace_ctx);
    }
#[no_mangle]
pub unsafe extern "C" fn tracing_sched_wakeup_trace(tr: *mut trace_array, wakee: *mut task_struct, curr: *mut task_struct, trace_ctx: c_uint) {
pub static mut event: *mut c_void = core::ptr::null_mut();
pub static mut entry: *mut c_void = core::ptr::null_mut();
    let mut buffer = tr.array_buffer.buffer;
    event = trace_buffer_lock_reserve(buffer, TRACE_WAKE,
    sizeof!(*entry), trace_ctx);
    if (!event) {
    return;
    }
    entry	= ring_buffer_event_data(event);
    entry.prev_pid			= curr.pid;
    entry.prev_prio		= curr.prio;
    entry.prev_state		= task_state_index(curr);
    entry.next_pid			= wakee.pid;
    entry.next_prio		= wakee.prio;
    entry.next_state		= task_state_index(wakee);
    entry.next_cpu			= task_cpu(wakee);
    trace_buffer_unlock_commit(tr, buffer, event, trace_ctx);
    }
    static void notrace
    probe_wakeup_sched_switch(void *ignore, bool preempt, task_struct *prev, task_struct *next,
    unsigned int prev_state)
    {
pub static mut data: *mut c_void = core::ptr::null_mut();
    u64 T0, T1, delta;
    let mut flags = 0;
    let mut disabled = 0;
    let mut cpu = 0;
    let mut trace_ctx = 0;
    tracing_record_cmdline(prev);
    if (unlikely(!tracer_enabled)) {
    return;
    }
//
// When we start a new trace, we set wakeup_task to NULL
// and then set tracer_enabled = 1. We want to make sure
// that another CPU does not see the tracer_enabled = 1
// and the wakeup_task with an older task, that might
// actually be the same as next.
//
    smp_rmb();
    if (next != wakeup_task) {
    return;
    }
// disable local data, not wakeup_cpu data
    cpu = raw_smp_processor_id();
    disabled = local_inc_return(&per_cpu_ptr(wakeup_trace.array_buffer.data, cpu).disabled);
    if (likely(disabled != 1)) {
// goto;
    }
    local_irq_save(flags);
    trace_ctx = tracing_gen_ctx_flags(flags);
    arch_spin_lock(&wakeup_lock);
// We could race with grabbing wakeup_lock
    if (unlikely(!tracer_enabled || next != wakeup_task)) {
// goto;
    }
// The task we are waiting for is waking up
    data = per_cpu_ptr(wakeup_trace.array_buffer.data, wakeup_cpu);
    __trace_function(wakeup_trace, CALLER_ADDR0, CALLER_ADDR1, trace_ctx);
    tracing_sched_switch_trace(wakeup_trace, prev, next, trace_ctx);
    __trace_stack(wakeup_trace, trace_ctx, 0);
    T0 = data.preempt_timestamp;
    T1 = ftrace_now(cpu);
    delta = T1-T0;
    if (!report_latency(wakeup_trace, delta)) {
// goto;
    }
    if (likely(!is_tracing_stopped())) {
    wakeup_trace.max_latency = delta;
    update_max_tr(wakeup_trace, wakeup_task, wakeup_cpu, core::ptr::null_mut());
    }
// label;
    __wakeup_reset(wakeup_trace);
    arch_spin_unlock(&wakeup_lock);
    local_irq_restore(flags);
// label;
    local_dec(&per_cpu_ptr(wakeup_trace.array_buffer.data, cpu).disabled);
    }
#[no_mangle]
unsafe extern "C" fn __wakeup_reset(tr: *mut trace_array) {
    wakeup_cpu = -1;
    wakeup_prio = -1;
    tracing_dl = false;
    if (wakeup_task) {
    put_task_struct(wakeup_task);
    }
    wakeup_task = core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn wakeup_reset(tr: *mut trace_array) {
    let mut flags = 0;
    tracing_reset_online_cpus(&tr.array_buffer);
    local_irq_save(flags);
    arch_spin_lock(&wakeup_lock);
    __wakeup_reset(tr);
    arch_spin_unlock(&wakeup_lock);
    local_irq_restore(flags);
    }
#[no_mangle]
pub unsafe extern "C" fn probe_wakeup(ignore: *mut c_void, p: *mut task_struct) {
pub static mut data: *mut c_void = core::ptr::null_mut();
pub static mut cpu: c_int = 0;
    let mut disabled = 0;
    let mut trace_ctx = 0;
    if (likely(!tracer_enabled)) {
    return;
    }
    tracing_record_cmdline(p);
    tracing_record_cmdline(current);
//
// Semantic is like this:
// - wakeup tracer handles all tasks in the system, independently
// from their scheduling class;
// - wakeup_rt tracer handles tasks belonging to sched_dl and
// sched_rt class;
// - wakeup_dl handles tasks belonging to sched_dl class only.
//
    if (tracing_dl || (wakeup_dl && !dl_task(p)) ||
    (wakeup_rt && !rt_or_dl_task(p)) ||
    (!dl_task(p) && (p.prio >= wakeup_prio || p.prio >= current.prio))) {
    return;
    }
    disabled = local_inc_return(&per_cpu_ptr(wakeup_trace.array_buffer.data, cpu).disabled);
    if (unlikely(disabled != 1)) {
// goto;
    }
    trace_ctx = tracing_gen_ctx();
// interrupts should be off from try_to_wake_up
    arch_spin_lock(&wakeup_lock);
// check for races.
    if (!tracer_enabled || tracing_dl ||
    (!dl_task(p) && p.prio >= wakeup_prio)) {
// goto;
    }
// reset the trace
    __wakeup_reset(wakeup_trace);
    wakeup_cpu = task_cpu(p);
    wakeup_current_cpu = wakeup_cpu;
    wakeup_prio = p.prio;
//
// Once you start tracing a -deadline task, don't bother tracing
// another task until the first one wakes up.
//
    if (dl_task(p)) {
    tracing_dl = true;
    }
    else {
    tracing_dl = false;
    }
    wakeup_task = get_task_struct(p);
    data = per_cpu_ptr(wakeup_trace.array_buffer.data, wakeup_cpu);
    data.preempt_timestamp = ftrace_now(cpu);
    tracing_sched_wakeup_trace(wakeup_trace, p, current, trace_ctx);
    __trace_stack(wakeup_trace, trace_ctx, 0);
//
// We must be careful in using CALLER_ADDR2. But since wake_up
// is not called by an assembly function  (where as schedule is)
// it should be safe to use it here.
//
    __trace_function(wakeup_trace, CALLER_ADDR1, CALLER_ADDR2, trace_ctx);
// label;
    arch_spin_unlock(&wakeup_lock);
// label;
    local_dec(&per_cpu_ptr(wakeup_trace.array_buffer.data, cpu).disabled);
    }
#[no_mangle]
unsafe extern "C" fn start_wakeup_tracer(tr: *mut trace_array) {
    let mut ret = 0;
    ret = register_trace_sched_wakeup(probe_wakeup, core::ptr::null_mut());
    if (ret) {
    pr_info!("wakeup trace: Couldn't activate tracepoint"
    " probe to kernel_sched_wakeup\n");
    return;
    }
    ret = register_trace_sched_wakeup_new(probe_wakeup, core::ptr::null_mut());
    if (ret) {
    pr_info!("wakeup trace: Couldn't activate tracepoint"
    " probe to kernel_sched_wakeup_new\n");
// goto;
    }
    ret = register_trace_sched_switch(probe_wakeup_sched_switch, core::ptr::null_mut());
    if (ret) {
    pr_info!("sched trace: Couldn't activate tracepoint"
    " probe to kernel_sched_switch\n");
// goto;
    }
    ret = register_trace_sched_migrate_task(probe_wakeup_migrate_task, core::ptr::null_mut());
    if (ret) {
    pr_info!("wakeup trace: Couldn't activate tracepoint"
    " probe to kernel_sched_migrate_task\n");
// goto;
    }
    wakeup_reset(tr);
//
// Don't let the tracer_enabled = 1 show up before
// the wakeup_task is reset. This may be overkill since
// wakeup_reset does a spin_unlock after setting the
// wakeup_task to NULL, but I want to be safe.
// This is a slow path anyway.
//
    smp_wmb();
    if (start_func_tracer(tr, is_graph(tr))) {
    printk("failed to start wakeup tracer\n");
    }
    return;
// label;
    unregister_trace_sched_switch(probe_wakeup_sched_switch, core::ptr::null_mut());
// label;
    unregister_trace_sched_wakeup_new(probe_wakeup, core::ptr::null_mut());
// label;
    unregister_trace_sched_wakeup(probe_wakeup, core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn stop_wakeup_tracer(tr: *mut trace_array) {
    tracer_enabled = 0;
    stop_func_tracer(tr, is_graph(tr));
    unregister_trace_sched_switch(probe_wakeup_sched_switch, core::ptr::null_mut());
    unregister_trace_sched_wakeup_new(probe_wakeup, core::ptr::null_mut());
    unregister_trace_sched_wakeup(probe_wakeup, core::ptr::null_mut());
    unregister_trace_sched_migrate_task(probe_wakeup_migrate_task, core::ptr::null_mut());
    }
    static bool wakeup_busy;
#[no_mangle]
unsafe extern "C" fn __wakeup_tracer_init(tr: *mut trace_array) -> c_int {
    save_flags = tr.trace_flags;
// non overwrite screws up the latency tracers
    set_tracer_flag(tr, TRACE_ITER(OVERWRITE), 1);
    set_tracer_flag(tr, TRACE_ITER(LATENCY_FMT), 1);
    tr.max_latency = 0;
    wakeup_trace = tr;
    ftrace_init_array_ops(tr, wakeup_tracer_call);
    start_wakeup_tracer(tr);
    wakeup_busy = true;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn wakeup_tracer_init(tr: *mut trace_array) -> c_int {
    if (wakeup_busy) {
    return -EBUSY;
    }
    wakeup_dl = false;
    wakeup_rt = false;
    return __wakeup_tracer_init(tr);
    }
#[no_mangle]
unsafe extern "C" fn wakeup_rt_tracer_init(tr: *mut trace_array) -> c_int {
    if (wakeup_busy) {
    return -EBUSY;
    }
    wakeup_dl = false;
    wakeup_rt = true;
    return __wakeup_tracer_init(tr);
    }
#[no_mangle]
unsafe extern "C" fn wakeup_dl_tracer_init(tr: *mut trace_array) -> c_int {
    if (wakeup_busy) {
    return -EBUSY;
    }
    wakeup_dl = true;
    wakeup_rt = false;
    return __wakeup_tracer_init(tr);
    }
#[no_mangle]
unsafe extern "C" fn wakeup_tracer_reset(tr: *mut trace_array) {
pub static mut lat_flag: c_int = 0;
pub static mut overwrite_flag: c_int = 0;
    stop_wakeup_tracer(tr);
// make sure we put back any tasks we are tracing
    wakeup_reset(tr);
    set_tracer_flag(tr, TRACE_ITER(LATENCY_FMT), lat_flag);
    set_tracer_flag(tr, TRACE_ITER(OVERWRITE), overwrite_flag);
    ftrace_reset_array_ops(tr);
    wakeup_busy = false;
    }
#[no_mangle]
unsafe extern "C" fn wakeup_tracer_start(tr: *mut trace_array) {
    wakeup_reset(tr);
    tracer_enabled = 1;
    }
#[no_mangle]
unsafe extern "C" fn wakeup_tracer_stop(tr: *mut trace_array) {
    tracer_enabled = 0;
    }
    static struct tracer wakeup_tracer  =
    {
    .name		= "wakeup",
    .init		= wakeup_tracer_init,
    .reset		= wakeup_tracer_reset,
    .start		= wakeup_tracer_start,
    .stop		= wakeup_tracer_stop,
    .print_max	= true,
    .print_header	= wakeup_print_header,
    .print_line	= wakeup_print_line,
    .flag_changed	= wakeup_flag_changed,

    .selftest    = trace_selftest_startup_wakeup,

    .open		= wakeup_trace_open,
    .close		= wakeup_trace_close,
    .allow_instances = true,
    .use_max_tr	= true,
    };
    static struct tracer wakeup_rt_tracer  =
    {
    .name		= "wakeup_rt",
    .init		= wakeup_rt_tracer_init,
    .reset		= wakeup_tracer_reset,
    .start		= wakeup_tracer_start,
    .stop		= wakeup_tracer_stop,
    .print_max	= true,
    .print_header	= wakeup_print_header,
    .print_line	= wakeup_print_line,
    .flag_changed	= wakeup_flag_changed,

    .selftest    = trace_selftest_startup_wakeup,

    .open		= wakeup_trace_open,
    .close		= wakeup_trace_close,
    .allow_instances = true,
    .use_max_tr	= true,
    };
    static struct tracer wakeup_dl_tracer  =
    {
    .name		= "wakeup_dl",
    .init		= wakeup_dl_tracer_init,
    .reset		= wakeup_tracer_reset,
    .start		= wakeup_tracer_start,
    .stop		= wakeup_tracer_stop,
    .print_max	= true,
    .print_header	= wakeup_print_header,
    .print_line	= wakeup_print_line,
    .flag_changed	= wakeup_flag_changed,

    .selftest    = trace_selftest_startup_wakeup,

    .open		= wakeup_trace_open,
    .close		= wakeup_trace_close,
    .allow_instances = true,
    .use_max_tr	= true,
    };
#[no_mangle]
pub unsafe extern "C" fn init_wakeup_tracer() -> __init static int {
    let mut ret = 0;
    ret = register_tracer(&wakeup_tracer);
    if (ret) {
    return ret;
    }
    ret = register_tracer(&wakeup_rt_tracer);
    if (ret) {
    return ret;
    }
    ret = register_tracer(&wakeup_dl_tracer);
    if (ret) {
    return ret;
    }
    return 0;
    }
    core_initcall!(init_wakeup_tracer);