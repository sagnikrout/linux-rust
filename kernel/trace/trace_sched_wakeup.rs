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

    static struct trace_array	*wakeup_trace;
    static int __read_mostly	tracer_enabled;
    static struct task_struct	*wakeup_task;
    static int			wakeup_cpu;
    static int			wakeup_current_cpu;
    let mut wakeup_prio: static unsigned = -1;
    static bool			wakeup_rt;
    static bool			wakeup_dl;
    static bool			tracing_dl;
    static arch_spinlock_t wakeup_lock =
    (arch_spinlock_t)__ARCH_SPIN_LOCK_UNLOCKED;
    static void wakeup_reset(struct trace_array *tr);
    static void __wakeup_reset(struct trace_array *tr);
    static int start_func_tracer(struct trace_array *tr, int graph);
    static void stop_func_tracer(struct trace_array *tr, int graph);
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
    static int
    func_prolog_preempt_disable(struct trace_array *tr,
    struct trace_array_cpu **data,
    unsigned int *trace_ctx)
    {
    long disabled;
    int cpu;
    if (likely(!wakeup_task))
    return 0;
// trace_ctx = tracing_gen_ctx();
    preempt_disable_notrace();
    cpu = raw_smp_processor_id();
    if (cpu != wakeup_current_cpu)
    goto out_enable;
// data = per_cpu_ptr(tr->array_buffer.data, cpu);
    disabled = local_inc_return(&(*data).disabled);
    if (unlikely(disabled != 1))
    goto out;
    return 1;
    out:
    local_dec(&(*data).disabled);
    out_enable:
    preempt_enable_notrace();
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn wakeup_display_graph(tr: *mut trace_array, set: c_int) -> c_int {
    static int wakeup_display_graph(struct trace_array *tr, int set)
    {
    if (!(is_graph(tr) ^ set))
    return 0;
    stop_func_tracer(tr, !set);
    wakeup_reset(wakeup_trace);
    tr.max_latency = 0;
    return start_func_tracer(tr, set);
    }
    static int wakeup_graph_entry(struct ftrace_graph_ent *trace,
    struct fgraph_ops *gops,
    struct ftrace_regs *fregs)
    {
    struct trace_array *tr = wakeup_trace;
    struct trace_array_cpu *data;
    unsigned int trace_ctx;
    u64 *calltime;
    let mut ret: c_int = 0;
    if (ftrace_graph_ignore_func(gops, trace))
    return 0;
//
// Do not trace a function if it's filtered by set_graph_notrace.
// Make the index of ret stack negative to indicate that it should
// ignore further functions.  But it needs its own ret stack entry
// to recover the original index in order to continue tracing after
// returning from the function.
//
    if (ftrace_graph_notrace_addr(trace.func))
    return 1;
    if (!func_prolog_preempt_disable(tr, &data, &trace_ctx))
    return 0;
    calltime = fgraph_reserve_data(gops.idx, sizeof(*calltime));
    if (calltime) {
// calltime = trace_clock_local();
    ret = __trace_graph_entry(tr, trace, trace_ctx);
    }
    local_dec(&data.disabled);
    preempt_enable_notrace();
    return ret;
    }
    static void wakeup_graph_return(struct ftrace_graph_ret *trace,
    struct fgraph_ops *gops,
    struct ftrace_regs *fregs)
    {
    struct trace_array *tr = wakeup_trace;
    struct trace_array_cpu *data;
    unsigned int trace_ctx;
    u64 *calltime;
    u64 rettime;
    int size;
    ftrace_graph_addr_finish(gops, trace);
    if (!func_prolog_preempt_disable(tr, &data, &trace_ctx))
    return;
    rettime = trace_clock_local();
    calltime = fgraph_retrieve_data(gops.idx, &size);
    if (calltime)
    __trace_graph_return(tr, trace, trace_ctx, *calltime, rettime);
    local_dec(&data.disabled);
    preempt_enable_notrace();
    return;
    }
    static struct fgraph_ops fgraph_wakeup_ops = {
    .entryfunc = &wakeup_graph_entry,
    .retfunc = &wakeup_graph_return,
    };
#[no_mangle]
unsafe extern "C" fn wakeup_trace_open(iter: *mut trace_iterator) {
    static void wakeup_trace_open(struct trace_iterator *iter)
    {
    if (is_graph(iter.tr))
    graph_trace_open(iter);
    }
#[no_mangle]
unsafe extern "C" fn wakeup_trace_close(iter: *mut trace_iterator) {
    static void wakeup_trace_close(struct trace_iterator *iter)
    {
    if (iter.private)
    graph_trace_close(iter);
    }

    TRACE_GRAPH_PRINT_CPU |  \
    TRACE_GRAPH_PRINT_REL_TIME | \
    TRACE_GRAPH_PRINT_DURATION | \
    TRACE_GRAPH_PRINT_OVERHEAD | \
    TRACE_GRAPH_PRINT_IRQS)
#[no_mangle]
unsafe extern "C" fn wakeup_print_line(iter: *mut trace_iterator) -> enum print_line_t {
    static enum print_line_t wakeup_print_line(struct trace_iterator *iter)
    {
//
// In graph mode call the graph tracer output function,
// otherwise go with the TRACE_FN event handler
//
    if (is_graph(iter.tr))
    return print_graph_function_flags(iter, GRAPH_TRACER_FLAGS);
    return TRACE_TYPE_UNHANDLED;
    }
#[no_mangle]
unsafe extern "C" fn wakeup_print_header(s: *mut seq_file) {
    static void wakeup_print_header(struct seq_file *s)
    {
    if (is_graph(wakeup_trace))
    print_graph_headers_flags(s, GRAPH_TRACER_FLAGS);
    else
    trace_default_header(s);
    }

//
// wakeup uses its own tracer function to keep the overhead down:
//
    static void
    wakeup_tracer_call(unsigned long ip, unsigned long parent_ip,
    struct ftrace_ops *op, struct ftrace_regs *fregs)
    {
    struct trace_array *tr = wakeup_trace;
    struct trace_array_cpu *data;
    unsigned long flags;
    unsigned int trace_ctx;
    if (!func_prolog_preempt_disable(tr, &data, &trace_ctx))
    return;
    local_irq_save(flags);
    trace_function(tr, ip, parent_ip, trace_ctx, fregs);
    local_irq_restore(flags);
    local_dec(&data.disabled);
    preempt_enable_notrace();
    }
#[no_mangle]
unsafe extern "C" fn register_wakeup_function(tr: *mut trace_array, graph: c_int, set: c_int) -> c_int {
    static int register_wakeup_function(struct trace_array *tr, int graph, int set)
    {
    int ret;
// 'set' is set if TRACE_ITER(FUNCTION) is about to be set
    if (function_enabled || (!set && !(tr.trace_flags & TRACE_ITER(FUNCTION))))
    return 0;
    if (graph)
    ret = register_ftrace_graph(&fgraph_wakeup_ops);
    else
    ret = register_ftrace_function(tr.ops);
    if (!ret)
    function_enabled = true;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn unregister_wakeup_function(tr: *mut trace_array, graph: c_int) {
    static void unregister_wakeup_function(struct trace_array *tr, int graph)
    {
    if (!function_enabled)
    return;
    if (graph)
    unregister_ftrace_graph(&fgraph_wakeup_ops);
    else
    unregister_ftrace_function(tr.ops);
    function_enabled = false;
    }
#[no_mangle]
unsafe extern "C" fn wakeup_function_set(tr: *mut trace_array, mask: u32, set: c_int) -> c_int {
    static int wakeup_function_set(struct trace_array *tr, u32 mask, int set)
    {
    if (!(mask & TRACE_ITER(FUNCTION)))
    return 0;
    if (set)
    register_wakeup_function(tr, is_graph(tr), 1);
    else
    unregister_wakeup_function(tr, is_graph(tr));
    return 1;
    }

#[no_mangle]
unsafe extern "C" fn register_wakeup_function(tr: *mut trace_array, graph: c_int, set: c_int) -> c_int {
    static int register_wakeup_function(struct trace_array *tr, int graph, int set)
    {
    return 0;
    }
    static void unregister_wakeup_function(struct trace_array *tr, int graph) { }
#[no_mangle]
unsafe extern "C" fn wakeup_function_set(tr: *mut trace_array, mask: u32, set: c_int) -> c_int {
    static int wakeup_function_set(struct trace_array *tr, u32 mask, int set)
    {
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn wakeup_print_line(iter: *mut trace_iterator) -> enum print_line_t {
    static enum print_line_t wakeup_print_line(struct trace_iterator *iter)
    {
    return TRACE_TYPE_UNHANDLED;
    }
    static void wakeup_trace_open(struct trace_iterator *iter) { }
    static void wakeup_trace_close(struct trace_iterator *iter) { }
#[no_mangle]
unsafe extern "C" fn wakeup_print_header(s: *mut seq_file) {
    static void wakeup_print_header(struct seq_file *s)
    {
    trace_default_header(s);
    }

    static void
    __trace_function(struct trace_array *tr,
    unsigned long ip, unsigned long parent_ip,
    unsigned int trace_ctx)
    {
    if (is_graph(tr))
    trace_graph_function(tr, ip, parent_ip, trace_ctx);
    else
    trace_function(tr, ip, parent_ip, trace_ctx, core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn wakeup_flag_changed(tr: *mut trace_array, mask: u64, set: c_int) -> c_int {
    static int wakeup_flag_changed(struct trace_array *tr, u64 mask, int set)
    {
    struct tracer *tracer = tr.current_trace;
    if (wakeup_function_set(tr, mask, set))
    return 0;

    if (mask & TRACE_ITER(DISPLAY_GRAPH))
    return wakeup_display_graph(tr, set);

    return trace_keep_overwrite(tracer, mask, set);
    }
#[no_mangle]
unsafe extern "C" fn start_func_tracer(tr: *mut trace_array, graph: c_int) -> c_int {
    static int start_func_tracer(struct trace_array *tr, int graph)
    {
    int ret;
    ret = register_wakeup_function(tr, graph, 0);
    if (!ret && tracing_is_enabled())
    tracer_enabled = 1;
    else
    tracer_enabled = 0;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn stop_func_tracer(tr: *mut trace_array, graph: c_int) {
    static void stop_func_tracer(struct trace_array *tr, int graph)
    {
    tracer_enabled = 0;
    unregister_wakeup_function(tr, graph);
    }
//
// Should this new latency be reported/recorded?
//
#[no_mangle]
unsafe extern "C" fn report_latency(tr: *mut trace_array, delta: u64) -> bool {
    static bool report_latency(struct trace_array *tr, u64 delta)
    {
    if (tracing_thresh) {
    if (delta < tracing_thresh)
    return false;
    } else {
    if (delta <= tr.max_latency)
    return false;
    }
    return true;
    }
    static void
    probe_wakeup_migrate_task(void *ignore, struct task_struct *task, int cpu)
    {
    if (task != wakeup_task)
    return;
    wakeup_current_cpu = cpu;
    }
    static void
    tracing_sched_switch_trace(struct trace_array *tr,
    struct task_struct *prev,
    struct task_struct *next,
    unsigned int trace_ctx)
    {
    struct trace_buffer *buffer = tr.array_buffer.buffer;
    struct ring_buffer_event *event;
    struct ctx_switch_entry *entry;
    event = trace_buffer_lock_reserve(buffer, TRACE_CTX,
    sizeof(*entry), trace_ctx);
    if (!event)
    return;
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
    static void
    tracing_sched_wakeup_trace(struct trace_array *tr,
    struct task_struct *wakee,
    struct task_struct *curr,
    unsigned int trace_ctx)
    {
    struct ring_buffer_event *event;
    struct ctx_switch_entry *entry;
    struct trace_buffer *buffer = tr.array_buffer.buffer;
    event = trace_buffer_lock_reserve(buffer, TRACE_WAKE,
    sizeof(*entry), trace_ctx);
    if (!event)
    return;
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
    probe_wakeup_sched_switch(void *ignore, bool preempt,
    struct task_struct *prev, struct task_struct *next,
    unsigned int prev_state)
    {
    struct trace_array_cpu *data;
    u64 T0, T1, delta;
    unsigned long flags;
    long disabled;
    int cpu;
    unsigned int trace_ctx;
    tracing_record_cmdline(prev);
    if (unlikely(!tracer_enabled))
    return;
//
// When we start a new trace, we set wakeup_task to NULL
// and then set tracer_enabled = 1. We want to make sure
// that another CPU does not see the tracer_enabled = 1
// and the wakeup_task with an older task, that might
// actually be the same as next.
//
    smp_rmb();
    if (next != wakeup_task)
    return;
// disable local data, not wakeup_cpu data
    cpu = raw_smp_processor_id();
    disabled = local_inc_return(&per_cpu_ptr(wakeup_trace.array_buffer.data, cpu).disabled);
    if (likely(disabled != 1))
    goto out;
    local_irq_save(flags);
    trace_ctx = tracing_gen_ctx_flags(flags);
    arch_spin_lock(&wakeup_lock);
// We could race with grabbing wakeup_lock
    if (unlikely(!tracer_enabled || next != wakeup_task))
    goto out_unlock;
// The task we are waiting for is waking up
    data = per_cpu_ptr(wakeup_trace.array_buffer.data, wakeup_cpu);
    __trace_function(wakeup_trace, CALLER_ADDR0, CALLER_ADDR1, trace_ctx);
    tracing_sched_switch_trace(wakeup_trace, prev, next, trace_ctx);
    __trace_stack(wakeup_trace, trace_ctx, 0);
    T0 = data.preempt_timestamp;
    T1 = ftrace_now(cpu);
    delta = T1-T0;
    if (!report_latency(wakeup_trace, delta))
    goto out_unlock;
    if (likely(!is_tracing_stopped())) {
    wakeup_trace.max_latency = delta;
    update_max_tr(wakeup_trace, wakeup_task, wakeup_cpu, core::ptr::null_mut());
    }
    out_unlock:
    __wakeup_reset(wakeup_trace);
    arch_spin_unlock(&wakeup_lock);
    local_irq_restore(flags);
    out:
    local_dec(&per_cpu_ptr(wakeup_trace.array_buffer.data, cpu).disabled);
    }
#[no_mangle]
unsafe extern "C" fn __wakeup_reset(tr: *mut trace_array) {
    static void __wakeup_reset(struct trace_array *tr)
    {
    wakeup_cpu = -1;
    wakeup_prio = -1;
    tracing_dl = false;
    if (wakeup_task)
    put_task_struct(wakeup_task);
    wakeup_task = core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn wakeup_reset(tr: *mut trace_array) {
    static void wakeup_reset(struct trace_array *tr)
    {
    unsigned long flags;
    tracing_reset_online_cpus(&tr.array_buffer);
    local_irq_save(flags);
    arch_spin_lock(&wakeup_lock);
    __wakeup_reset(tr);
    arch_spin_unlock(&wakeup_lock);
    local_irq_restore(flags);
    }
    static void
    probe_wakeup(void *ignore, struct task_struct *p)
    {
    struct trace_array_cpu *data;
    let mut cpu: c_int = smp_processor_id();
    long disabled;
    unsigned int trace_ctx;
    if (likely(!tracer_enabled))
    return;
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
    (!dl_task(p) && (p.prio >= wakeup_prio || p.prio >= current.prio)))
    return;
    disabled = local_inc_return(&per_cpu_ptr(wakeup_trace.array_buffer.data, cpu).disabled);
    if (unlikely(disabled != 1))
    goto out;
    trace_ctx = tracing_gen_ctx();
// interrupts should be off from try_to_wake_up
    arch_spin_lock(&wakeup_lock);
// check for races.
    if (!tracer_enabled || tracing_dl ||
    (!dl_task(p) && p.prio >= wakeup_prio))
    goto out_locked;
// reset the trace
    __wakeup_reset(wakeup_trace);
    wakeup_cpu = task_cpu(p);
    wakeup_current_cpu = wakeup_cpu;
    wakeup_prio = p.prio;
//
// Once you start tracing a -deadline task, don't bother tracing
// another task until the first one wakes up.
//
    if (dl_task(p))
    tracing_dl = true;
    else
    tracing_dl = false;
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
    out_locked:
    arch_spin_unlock(&wakeup_lock);
    out:
    local_dec(&per_cpu_ptr(wakeup_trace.array_buffer.data, cpu).disabled);
    }
#[no_mangle]
unsafe extern "C" fn start_wakeup_tracer(tr: *mut trace_array) {
    static void start_wakeup_tracer(struct trace_array *tr)
    {
    int ret;
    ret = register_trace_sched_wakeup(probe_wakeup, core::ptr::null_mut());
    if (ret) {
    pr_info("wakeup trace: Couldn't activate tracepoint"
    " probe to kernel_sched_wakeup\n");
    return;
    }
    ret = register_trace_sched_wakeup_new(probe_wakeup, core::ptr::null_mut());
    if (ret) {
    pr_info("wakeup trace: Couldn't activate tracepoint"
    " probe to kernel_sched_wakeup_new\n");
    goto fail_deprobe;
    }
    ret = register_trace_sched_switch(probe_wakeup_sched_switch, core::ptr::null_mut());
    if (ret) {
    pr_info("sched trace: Couldn't activate tracepoint"
    " probe to kernel_sched_switch\n");
    goto fail_deprobe_wake_new;
    }
    ret = register_trace_sched_migrate_task(probe_wakeup_migrate_task, core::ptr::null_mut());
    if (ret) {
    pr_info("wakeup trace: Couldn't activate tracepoint"
    " probe to kernel_sched_migrate_task\n");
    goto fail_deprobe_sched_switch;
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
    if (start_func_tracer(tr, is_graph(tr)))
    printk(KERN_ERR "failed to start wakeup tracer\n");
    return;
    fail_deprobe_sched_switch:
    unregister_trace_sched_switch(probe_wakeup_sched_switch, core::ptr::null_mut());
    fail_deprobe_wake_new:
    unregister_trace_sched_wakeup_new(probe_wakeup, core::ptr::null_mut());
    fail_deprobe:
    unregister_trace_sched_wakeup(probe_wakeup, core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn stop_wakeup_tracer(tr: *mut trace_array) {
    static void stop_wakeup_tracer(struct trace_array *tr)
    {
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
    static int __wakeup_tracer_init(struct trace_array *tr)
    {
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
    static int wakeup_tracer_init(struct trace_array *tr)
    {
    if (wakeup_busy)
    return -EBUSY;
    wakeup_dl = false;
    wakeup_rt = false;
    return __wakeup_tracer_init(tr);
    }
#[no_mangle]
unsafe extern "C" fn wakeup_rt_tracer_init(tr: *mut trace_array) -> c_int {
    static int wakeup_rt_tracer_init(struct trace_array *tr)
    {
    if (wakeup_busy)
    return -EBUSY;
    wakeup_dl = false;
    wakeup_rt = true;
    return __wakeup_tracer_init(tr);
    }
#[no_mangle]
unsafe extern "C" fn wakeup_dl_tracer_init(tr: *mut trace_array) -> c_int {
    static int wakeup_dl_tracer_init(struct trace_array *tr)
    {
    if (wakeup_busy)
    return -EBUSY;
    wakeup_dl = true;
    wakeup_rt = false;
    return __wakeup_tracer_init(tr);
    }
#[no_mangle]
unsafe extern "C" fn wakeup_tracer_reset(tr: *mut trace_array) {
    static void wakeup_tracer_reset(struct trace_array *tr)
    {
    let mut lat_flag: c_int = save_flags & TRACE_ITER(LATENCY_FMT);
    let mut overwrite_flag: c_int = save_flags & TRACE_ITER(OVERWRITE);
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
    static void wakeup_tracer_start(struct trace_array *tr)
    {
    wakeup_reset(tr);
    tracer_enabled = 1;
    }
#[no_mangle]
unsafe extern "C" fn wakeup_tracer_stop(tr: *mut trace_array) {
    static void wakeup_tracer_stop(struct trace_array *tr)
    {
    tracer_enabled = 0;
    }
    static struct tracer wakeup_tracer __read_mostly =
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
    static struct tracer wakeup_rt_tracer __read_mostly =
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
    static struct tracer wakeup_dl_tracer __read_mostly =
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
    __init static int init_wakeup_tracer(void)
    {
    int ret;
    ret = register_tracer(&wakeup_tracer);
    if (ret)
    return ret;
    ret = register_tracer(&wakeup_rt_tracer);
    if (ret)
    return ret;
    ret = register_tracer(&wakeup_dl_tracer);
    if (ret)
    return ret;
    return 0;
    }
    core_initcall(init_wakeup_tracer);
