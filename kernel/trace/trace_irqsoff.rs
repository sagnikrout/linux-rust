//! Automatically rewritten from C to Rust
//! Source: kernel/trace/trace_irqsoff.c
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
// trace irqs off critical timings
//
// Copyright (C) 2007-2008 Steven Rostedt <srostedt@redhat.com>
// Copyright (C) 2008 Ingo Molnar <mingo@redhat.com>
//
// From code in the latency_tracer, that is:
//
// Copyright (C) 2004-2006 Ingo Molnar
// Copyright (C) 2004 Nadia Yvette Chambers
//

pub static mut irqsoff_trace: *mut c_void = core::ptr::null_mut();
    static int				tracer_enabled ;
pub static mut int: usize = 0;
pub static mut max_trace_lock: usize = 0;
    enum {
    TRACER_IRQS_OFF		= (1 << 1),
    TRACER_PREEMPT_OFF	= (1 << 2),
    };
    static int trace_type ;
    static int save_flags;
// forward_decl: stop_irqsoff_tracer;
// forward_decl: start_irqsoff_tracer;

#[no_mangle]
pub unsafe extern "C" fn preempt_trace(pc: c_int) -> c_int {
    return ((trace_type & TRACER_PREEMPT_OFF) && pc);
    }

#[no_mangle]
pub unsafe extern "C" fn irq_trace() -> c_int {
    return ((trace_type & TRACER_IRQS_OFF) &&
    irqs_disabled());
    }

// forward_decl: irqsoff_display_graph;

#[no_mangle]
pub unsafe extern "C" fn irqsoff_display_graph(tr: *mut trace_array, set: c_int) -> c_int {
    return -EINVAL;
    }

//
// Sequence count - we record it when starting a measurement and
// skip the latency if the sequence has changed - some other section
// did a maximum and could disturb our measurement with serial console
// printouts, etc. Truly coinciding maximum latencies should be rare
// and what happens together happens separately as well, so this doesn't
// decrease the validity of the maximum found:
//
    static __cacheline_aligned_in_smp	unsigned long max_sequence;

//
// Prologue for the preempt and irqs off function tracers.
//
// Returns 1 if it is OK to continue, and data->disabled is
// incremented.
// 0 if the trace is to be ignored, and data->disabled
// is kept the same.
//
// Note, this function is also used outside this ifdef but
// inside the #ifdef of the function graph tracer below.
// This is OK, since the function graph tracer is
// dependent on the function tracer.
//
#[no_mangle]
pub unsafe extern "C" fn func_prolog_dec(tr: *mut trace_array, data: *mut *mut trace_array_cpu, flags: *mut c_ulong) -> c_int {
    let mut disabled = 0;
    let mut cpu = 0;
//
// Does not matter if we preempt. We test the flags
// afterward, to see if irqs are disabled or not.
// If we preempt and get a false positive, the flags
// test will fail.
//
    cpu = raw_smp_processor_id();
    if (likely(!per_cpu(tracing_cpu, cpu))) {
    return 0;
    }
    local_save_flags(*flags);
//
// Slight chance to get a false positive on tracing_cpu,
// although I'm starting to think there isn't a chance.
// Leave this for now just to be paranoid.
//
    if (!irqs_disabled_flags(*flags) && !preempt_count()) {
    return 0;
    }
// data = per_cpu_ptr(tr->array_buffer.data, cpu);
    disabled = local_inc_return(&(*data).disabled);
    if (likely(disabled == 1)) {
    return 1;
    }
    local_dec(&(*data).disabled);
    return 0;
    }
//
// irqsoff uses its own tracer function to keep the overhead down:
//
#[no_mangle]
pub unsafe extern "C" fn irqsoff_tracer_call(ip: c_ulong, parent_ip: c_ulong, op: *mut ftrace_ops, fregs: *mut ftrace_regs) {
    let mut tr = irqsoff_trace;
pub static mut data: *mut c_void = core::ptr::null_mut();
    let mut flags = 0;
    let mut trace_ctx = 0;
    if (!func_prolog_dec(tr, &data, &flags)) {
    return;
    }
    trace_ctx = tracing_gen_ctx_flags(flags);
    trace_function(tr, ip, parent_ip, trace_ctx, fregs);
    local_dec(&data.disabled);
    }

#[no_mangle]
unsafe extern "C" fn irqsoff_display_graph(tr: *mut trace_array, set: c_int) -> c_int {
    let mut cpu = 0;
    if (!(is_graph(tr) ^ set)) {
    return 0;
    }
    stop_irqsoff_tracer(irqsoff_trace, !set);
    for_each_possible_cpu(cpu) {
    per_cpu(tracing_cpu, cpu) = 0;
    }
    tr.max_latency = 0;
    tracing_reset_online_cpus(&irqsoff_trace.array_buffer);
    return start_irqsoff_tracer(irqsoff_trace, set);
    }
#[no_mangle]
pub unsafe extern "C" fn irqsoff_graph_entry(trace: *mut ftrace_graph_ent, gops: *mut fgraph_ops, fregs: *mut ftrace_regs) -> c_int {
    let mut tr = irqsoff_trace;
pub static mut data: *mut c_void = core::ptr::null_mut();
    let mut flags = 0;
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
    if (!func_prolog_dec(tr, &data, &flags)) {
    return 0;
    }
    calltime = fgraph_reserve_data(gops.idx, sizeof!(*calltime));
    if (calltime) {
// calltime = trace_clock_local();
    trace_ctx = tracing_gen_ctx_flags(flags);
    ret = __trace_graph_entry(tr, trace, trace_ctx);
    }
    local_dec(&data.disabled);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn irqsoff_graph_return(trace: *mut ftrace_graph_ret, gops: *mut fgraph_ops, fregs: *mut ftrace_regs) {
    let mut tr = irqsoff_trace;
pub static mut data: *mut c_void = core::ptr::null_mut();
    let mut flags = 0;
    let mut trace_ctx = 0;
pub static mut calltime: *mut c_void = core::ptr::null_mut();
    let mut rettime = 0;
    let mut size = 0;
    ftrace_graph_addr_finish(gops, trace);
    if (!func_prolog_dec(tr, &data, &flags)) {
    return;
    }
    rettime = trace_clock_local();
    calltime = fgraph_retrieve_data(gops.idx, &size);
    if (calltime) {
    trace_ctx = tracing_gen_ctx_flags(flags);
    __trace_graph_return(tr, trace, trace_ctx, *calltime, rettime);
    }
    local_dec(&data.disabled);
    }
pub static mut fgraph_ops: usize = 0;
#[no_mangle]
unsafe extern "C" fn irqsoff_trace_open(iter: *mut trace_iterator) {
    if (is_graph(iter.tr)) {
    graph_trace_open(iter);
    }
    }
#[no_mangle]
unsafe extern "C" fn irqsoff_trace_close(iter: *mut trace_iterator) {
    if (iter.private) {
    graph_trace_close(iter);
    }
    }

    TRACE_GRAPH_PRINT_PROC | 
    TRACE_GRAPH_PRINT_REL_TIME | 
    TRACE_GRAPH_PRINT_DURATION)
#[no_mangle]
unsafe extern "C" fn irqsoff_print_line(iter: *mut trace_iterator) -> enum print_line_t {
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
unsafe extern "C" fn irqsoff_print_header(s: *mut seq_file) {
    let mut tr = irqsoff_trace;
    if (is_graph(tr)) {
    print_graph_headers_flags(s, GRAPH_TRACER_FLAGS);
    }
    else {
    trace_default_header(s);
    }
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
#[no_mangle]
// duplicate fn: __trace_function
pub unsafe extern "C" fn __trace_function_dup(tr: *mut trace_array, ip: c_ulong, parent_ip: c_ulong, trace_ctx: c_uint) {
    return trace_function(tr, ip, parent_ip, trace_ctx, core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn irqsoff_print_line(iter: *mut trace_iterator) -> enum print_line_t {
    return TRACE_TYPE_UNHANDLED;
    }
#[no_mangle]
pub unsafe extern "C" fn irqsoff_trace_open(iter: *mut trace_iterator) { }
#[no_mangle]
pub unsafe extern "C" fn irqsoff_trace_close(iter: *mut trace_iterator) { }

#[no_mangle]
unsafe extern "C" fn irqsoff_print_header(s: *mut seq_file) {
    trace_default_header(s);
    }

#[no_mangle]
unsafe extern "C" fn irqsoff_print_header(s: *mut seq_file) {
    trace_latency_header(s);
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
pub unsafe extern "C" fn check_critical_timing(tr: *mut trace_array, data: *mut trace_array_cpu, parent_ip: c_ulong, cpu: c_int) {
    u64 T0, T1, delta;
    let mut flags = 0;
    let mut trace_ctx = 0;
    T0 = data.preempt_timestamp;
    T1 = ftrace_now(cpu);
    delta = T1-T0;
    trace_ctx = tracing_gen_ctx();
    if (!report_latency(tr, delta)) {
// goto;
    }
    raw_spin_lock_irqsave(&max_trace_lock, flags);
// check if we are still the max latency
    if (!report_latency(tr, delta)) {
// goto;
    }
    __trace_function(tr, CALLER_ADDR0, parent_ip, trace_ctx);
// Skip 5 functions to get to the irq/preempt enable function
    __trace_stack(tr, trace_ctx, 5);
    if (data.critical_sequence != max_sequence) {
// goto;
    }
    data.critical_end = parent_ip;
    if (likely(!is_tracing_stopped())) {
    tr.max_latency = delta;
    update_max_tr_single(tr, current, cpu);
    }
    max_sequence += 1;
// label;
    raw_spin_unlock_irqrestore(&max_trace_lock, flags);
// label;
    data.critical_sequence = max_sequence;
    data.preempt_timestamp = ftrace_now(cpu);
    __trace_function(tr, CALLER_ADDR0, parent_ip, trace_ctx);
    }
    static nokprobe_inline void
    start_critical_timing(unsigned long ip, unsigned long parent_ip)
    {
    let mut cpu = 0;
    let mut tr = irqsoff_trace;
pub static mut data: *mut c_void = core::ptr::null_mut();
    let mut disabled = 0;
    if (!tracer_enabled || !tracing_is_enabled()) {
    return;
    }
    cpu = raw_smp_processor_id();
    if (per_cpu(tracing_cpu, cpu)) {
    return;
    }
    data = per_cpu_ptr(tr.array_buffer.data, cpu);
    if (unlikely(!data) || local_read(&data.disabled)) {
    return;
    }
    disabled = local_inc_return(&data.disabled);
    if (disabled == 1) {
    data.critical_sequence = max_sequence;
    data.preempt_timestamp = ftrace_now(cpu);
    data.critical_start = parent_ip ? : ip;
    __trace_function(tr, ip, parent_ip, tracing_gen_ctx());
    per_cpu(tracing_cpu, cpu) = 1;
    }
    local_dec(&data.disabled);
    }
    static nokprobe_inline void
    stop_critical_timing(unsigned long ip, unsigned long parent_ip)
    {
    let mut cpu = 0;
    let mut tr = irqsoff_trace;
pub static mut data: *mut c_void = core::ptr::null_mut();
    let mut trace_ctx = 0;
    let mut disabled = 0;
    cpu = raw_smp_processor_id();
// Always clear the tracing cpu on stopping the trace
    if (unlikely(per_cpu(tracing_cpu, cpu))) {
    per_cpu(tracing_cpu, cpu) = 0;
    }
    else {
    return;
    }
    if (!tracer_enabled || !tracing_is_enabled()) {
    return;
    }
    data = per_cpu_ptr(tr.array_buffer.data, cpu);
    if (unlikely(!data) ||
    !data.critical_start || local_read(&data.disabled)) {
    return;
    }
    disabled = local_inc_return(&data.disabled);
    if (disabled == 1) {
    trace_ctx = tracing_gen_ctx();
    __trace_function(tr, ip, parent_ip, trace_ctx);
    check_critical_timing(tr, data, parent_ip ? : ip, cpu);
    data.critical_start = 0;
    }
    local_dec(&data.disabled);
    }
// start and stop critical timings used to for stoppage (in idle)
#[no_mangle]
pub unsafe extern "C" fn start_critical_timings() {
    if (preempt_trace(preempt_count()) || irq_trace()) {
    start_critical_timing(CALLER_ADDR0, CALLER_ADDR1);
    }
    }
    EXPORT_SYMBOL_GPL(start_critical_timings);
    NOKPROBE_SYMBOL(start_critical_timings);
#[no_mangle]
pub unsafe extern "C" fn stop_critical_timings() {
    if (preempt_trace(preempt_count()) || irq_trace()) {
    stop_critical_timing(CALLER_ADDR0, CALLER_ADDR1);
    }
    }
    EXPORT_SYMBOL_GPL(stop_critical_timings);
    NOKPROBE_SYMBOL(stop_critical_timings);

    static bool function_enabled;
#[no_mangle]
unsafe extern "C" fn register_irqsoff_function(tr: *mut trace_array, graph: c_int, set: c_int) -> c_int {
    let mut ret = 0;
// 'set' is set if TRACE_ITER(FUNCTION) is about to be set
    if (function_enabled || (!set && !(tr.trace_flags & TRACE_ITER(FUNCTION)))) {
    return 0;
    }
    if (graph) {
    ret = register_ftrace_graph(&fgraph_ops);
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
unsafe extern "C" fn unregister_irqsoff_function(tr: *mut trace_array, graph: c_int) {
    if (!function_enabled) {
    return;
    }
    if (graph) {
    unregister_ftrace_graph(&fgraph_ops);
    }
    else {
    unregister_ftrace_function(tr.ops);
    }
    function_enabled = false;
    }
#[no_mangle]
unsafe extern "C" fn irqsoff_function_set(tr: *mut trace_array, mask: u32, set: c_int) -> c_int {
    if (!(mask & TRACE_ITER(FUNCTION))) {
    return 0;
    }
    if (set) {
    register_irqsoff_function(tr, is_graph(tr), 1);
    }
    else {
    unregister_irqsoff_function(tr, is_graph(tr));
    }
    return 1;
    }

#[no_mangle]
unsafe extern "C" fn register_irqsoff_function(tr: *mut trace_array, graph: c_int, set: c_int) -> c_int {
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn unregister_irqsoff_function(tr: *mut trace_array, graph: c_int) { }
#[no_mangle]
pub unsafe extern "C" fn irqsoff_function_set(tr: *mut trace_array, mask: u32, set: c_int) -> c_int {
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn irqsoff_flag_changed(tr: *mut trace_array, mask: u64, set: c_int) -> c_int {
    let mut tracer = tr.current_trace;
    if (irqsoff_function_set(tr, mask, set)) {
    return 0;
    }

    if (mask & TRACE_ITER(DISPLAY_GRAPH)) {
    return irqsoff_display_graph(tr, set);
    }

    return trace_keep_overwrite(tracer, mask, set);
    }
#[no_mangle]
unsafe extern "C" fn start_irqsoff_tracer(tr: *mut trace_array, graph: c_int) -> c_int {
    let mut ret = 0;
    ret = register_irqsoff_function(tr, graph, 0);
    if (!ret && tracing_is_enabled()) {
    tracer_enabled = 1;
    }
    else {
    tracer_enabled = 0;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn stop_irqsoff_tracer(tr: *mut trace_array, graph: c_int) {
    tracer_enabled = 0;
    unregister_irqsoff_function(tr, graph);
    }
    static bool irqsoff_busy;
#[no_mangle]
unsafe extern "C" fn __irqsoff_tracer_init(tr: *mut trace_array) -> c_int {
    if (irqsoff_busy) {
    return -EBUSY;
    }
    save_flags = tr.trace_flags;
// non overwrite screws up the latency tracers
    set_tracer_flag(tr, TRACE_ITER(OVERWRITE), 1);
    set_tracer_flag(tr, TRACE_ITER(LATENCY_FMT), 1);
// without pause, we will produce garbage if another latency occurs
    set_tracer_flag(tr, TRACE_ITER(PAUSE_ON_TRACE), 1);
    tr.max_latency = 0;
    irqsoff_trace = tr;
// make sure that the tracer is visible
    smp_wmb();
    ftrace_init_array_ops(tr, irqsoff_tracer_call);
// Only toplevel instance supports graph tracing
    if (start_irqsoff_tracer(tr, (tr.flags & TRACE_ARRAY_FL_GLOBAL &&
    is_graph(tr)))) {
    printk("failed to start irqsoff tracer\n");
    }
    irqsoff_busy = true;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn __irqsoff_tracer_reset(tr: *mut trace_array) {
pub static mut lat_flag: c_int = 0;
pub static mut overwrite_flag: c_int = 0;
pub static mut pause_flag: c_int = 0;
    stop_irqsoff_tracer(tr, is_graph(tr));
    set_tracer_flag(tr, TRACE_ITER(LATENCY_FMT), lat_flag);
    set_tracer_flag(tr, TRACE_ITER(OVERWRITE), overwrite_flag);
    set_tracer_flag(tr, TRACE_ITER(PAUSE_ON_TRACE), pause_flag);
    ftrace_reset_array_ops(tr);
    irqsoff_busy = false;
    }
#[no_mangle]
unsafe extern "C" fn irqsoff_tracer_start(tr: *mut trace_array) {
    tracer_enabled = 1;
    }
#[no_mangle]
unsafe extern "C" fn irqsoff_tracer_stop(tr: *mut trace_array) {
    tracer_enabled = 0;
    }

//
// We are only interested in hardirq on/off events:
//
#[no_mangle]
pub unsafe extern "C" fn tracer_hardirqs_on(a0: c_ulong, a1: c_ulong) {
    if (!preempt_trace(preempt_count()) && irq_trace()) {
    stop_critical_timing(a0, a1);
    }
    }
    NOKPROBE_SYMBOL(tracer_hardirqs_on);
#[no_mangle]
pub unsafe extern "C" fn tracer_hardirqs_off(a0: c_ulong, a1: c_ulong) {
    if (!preempt_trace(preempt_count()) && irq_trace()) {
    start_critical_timing(a0, a1);
    }
    }
    NOKPROBE_SYMBOL(tracer_hardirqs_off);
#[no_mangle]
unsafe extern "C" fn irqsoff_tracer_init(tr: *mut trace_array) -> c_int {
    trace_type = TRACER_IRQS_OFF;
    return __irqsoff_tracer_init(tr);
    }
#[no_mangle]
unsafe extern "C" fn irqsoff_tracer_reset(tr: *mut trace_array) {
    __irqsoff_tracer_reset(tr);
    }
    static struct tracer irqsoff_tracer  =
    {
    .name		= "irqsoff",
    .init		= irqsoff_tracer_init,
    .reset		= irqsoff_tracer_reset,
    .start		= irqsoff_tracer_start,
    .stop		= irqsoff_tracer_stop,
    .print_max	= true,
    .print_header   = irqsoff_print_header,
    .print_line     = irqsoff_print_line,
    .flag_changed	= irqsoff_flag_changed,

    .selftest    = trace_selftest_startup_irqsoff,

    .open           = irqsoff_trace_open,
    .close          = irqsoff_trace_close,
    .allow_instances = true,
    .use_max_tr	= true,
    };

#[no_mangle]
pub unsafe extern "C" fn tracer_preempt_on(a0: c_ulong, a1: c_ulong) {
    if (preempt_trace(preempt_count()) && !irq_trace()) {
    stop_critical_timing(a0, a1);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn tracer_preempt_off(a0: c_ulong, a1: c_ulong) {
    if (preempt_trace(preempt_count()) && !irq_trace()) {
    start_critical_timing(a0, a1);
    }
    }
#[no_mangle]
unsafe extern "C" fn preemptoff_tracer_init(tr: *mut trace_array) -> c_int {
    trace_type = TRACER_PREEMPT_OFF;
    return __irqsoff_tracer_init(tr);
    }
#[no_mangle]
unsafe extern "C" fn preemptoff_tracer_reset(tr: *mut trace_array) {
    __irqsoff_tracer_reset(tr);
    }
    static struct tracer preemptoff_tracer  =
    {
    .name		= "preemptoff",
    .init		= preemptoff_tracer_init,
    .reset		= preemptoff_tracer_reset,
    .start		= irqsoff_tracer_start,
    .stop		= irqsoff_tracer_stop,
    .print_max	= true,
    .print_header   = irqsoff_print_header,
    .print_line     = irqsoff_print_line,
    .flag_changed	= irqsoff_flag_changed,

    .selftest    = trace_selftest_startup_preemptoff,

    .open		= irqsoff_trace_open,
    .close		= irqsoff_trace_close,
    .allow_instances = true,
    .use_max_tr	= true,
    };

#[no_mangle]
unsafe extern "C" fn preemptirqsoff_tracer_init(tr: *mut trace_array) -> c_int {
    trace_type = TRACER_IRQS_OFF | TRACER_PREEMPT_OFF;
    return __irqsoff_tracer_init(tr);
    }
#[no_mangle]
unsafe extern "C" fn preemptirqsoff_tracer_reset(tr: *mut trace_array) {
    __irqsoff_tracer_reset(tr);
    }
    static struct tracer preemptirqsoff_tracer  =
    {
    .name		= "preemptirqsoff",
    .init		= preemptirqsoff_tracer_init,
    .reset		= preemptirqsoff_tracer_reset,
    .start		= irqsoff_tracer_start,
    .stop		= irqsoff_tracer_stop,
    .print_max	= true,
    .print_header   = irqsoff_print_header,
    .print_line     = irqsoff_print_line,
    .flag_changed	= irqsoff_flag_changed,

    .selftest    = trace_selftest_startup_preemptirqsoff,

    .open		= irqsoff_trace_open,
    .close		= irqsoff_trace_close,
    .allow_instances = true,
    .use_max_tr	= true,
    };

#[no_mangle]
pub unsafe extern "C" fn init_irqsoff_tracer() -> __init static int {

    register_tracer(&irqsoff_tracer);

    register_tracer(&preemptoff_tracer);

    register_tracer(&preemptirqsoff_tracer);

    return 0;
    }
    core_initcall!(init_irqsoff_tracer);