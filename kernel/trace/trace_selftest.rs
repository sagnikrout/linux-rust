//! Automatically rewritten from C to Rust
//! Source: kernel/trace/trace_selftest.c
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
// Include in trace.c

#[no_mangle]
pub unsafe extern "C" fn trace_valid_entry(entry: *mut trace_entry) -> c_int {
    static inline int trace_valid_entry(struct trace_entry *entry)
    {
    switch (entry.type) {
    case TRACE_FN:
    case TRACE_CTX:
    case TRACE_WAKE:
    case TRACE_STACK:
    case TRACE_PRINT:
    case TRACE_BRANCH:
    case TRACE_GRAPH_ENT:
    case TRACE_GRAPH_RETADDR_ENT:
    case TRACE_GRAPH_RET:
    return 1;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn trace_test_buffer_cpu(buf: *mut array_buffer, cpu: c_int) -> c_int {
    static int trace_test_buffer_cpu(struct array_buffer *buf, int cpu)
    {
    struct ring_buffer_event *event;
    struct trace_entry *entry;
    let mut loops: c_uint = 0;
    while ((event = ring_buffer_consume(buf.buffer, cpu, core::ptr::null_mut(), core::ptr::null_mut()))) {
    entry = ring_buffer_event_data(event);
//
// The ring buffer is a size of trace_buf_size, if
// we loop more than the size, there's something wrong
// with the ring buffer.
//
    if (loops++ > trace_buf_size) {
    printk(KERN_CONT ".. bad ring buffer ");
    goto failed;
    }
    if (!trace_valid_entry(entry)) {
    printk(KERN_CONT ".. invalid entry %d ",
    entry.type);
    goto failed;
    }
    }
    return 0;
    failed:
// disable tracing
    tracing_disabled = 1;
    printk(KERN_CONT ".. corrupted trace buffer .. ");
    return -1;
    }
//
// Test the trace buffer to see if all the elements
// are still sane.
//
#[no_mangle]
unsafe extern "C" fn trace_test_buffer(buf: *mut array_buffer, count: *mut c_ulong) -> int __maybe_unused {
    static int __maybe_unused trace_test_buffer(struct array_buffer *buf, unsigned long *count)
    {
    unsigned long flags, cnt = 0;
    int cpu, ret = 0;
// Don't allow flipping of max traces now
    local_irq_save(flags);
    arch_spin_lock(&buf.tr.max_lock);
    cnt = ring_buffer_entries(buf.buffer);
//
// The trace_test_buffer_cpu runs a while loop to consume all data.
// If the calling tracer is broken, and is constantly filling
// the buffer, this will run forever, and hard lock the box.
// We disable the ring buffer while we do this test to prevent
// a hard lock up.
//
    tracing_off();
    for_each_possible_cpu(cpu) {
    ret = trace_test_buffer_cpu(buf, cpu);
    if (ret)
    break;
    }
    tracing_on();
    arch_spin_unlock(&buf.tr.max_lock);
    local_irq_restore(flags);
    if (count)
// count = cnt;
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn warn_failed_init_tracer(trace: *mut tracer, init_ret: c_int) {
    static inline void warn_failed_init_tracer(struct tracer *trace, int init_ret)
    {
    printk(KERN_WARNING "Failed to init %s tracer, init returned %d\n",
    trace.name, init_ret);
    }

    static int trace_selftest_test_probe1_cnt;
    static void trace_selftest_test_probe1_func(unsigned long ip,
    unsigned long pip,
    struct ftrace_ops *op,
    struct ftrace_regs *fregs)
    {
    trace_selftest_test_probe1_cnt++;
    }
    static int trace_selftest_test_probe2_cnt;
    static void trace_selftest_test_probe2_func(unsigned long ip,
    unsigned long pip,
    struct ftrace_ops *op,
    struct ftrace_regs *fregs)
    {
    trace_selftest_test_probe2_cnt++;
    }
    static int trace_selftest_test_probe3_cnt;
    static void trace_selftest_test_probe3_func(unsigned long ip,
    unsigned long pip,
    struct ftrace_ops *op,
    struct ftrace_regs *fregs)
    {
    trace_selftest_test_probe3_cnt++;
    }
    static int trace_selftest_test_global_cnt;
    static void trace_selftest_test_global_func(unsigned long ip,
    unsigned long pip,
    struct ftrace_ops *op,
    struct ftrace_regs *fregs)
    {
    trace_selftest_test_global_cnt++;
    }
    static int trace_selftest_test_dyn_cnt;
    static void trace_selftest_test_dyn_func(unsigned long ip,
    unsigned long pip,
    struct ftrace_ops *op,
    struct ftrace_regs *fregs)
    {
    trace_selftest_test_dyn_cnt++;
    }
    static struct ftrace_ops test_probe1 = {
    .func			= trace_selftest_test_probe1_func,
    };
    static struct ftrace_ops test_probe2 = {
    .func			= trace_selftest_test_probe2_func,
    };
    static struct ftrace_ops test_probe3 = {
    .func			= trace_selftest_test_probe3_func,
    };
#[no_mangle]
unsafe extern "C" fn print_counts() {
    static void print_counts(void)
    {
    printk("(%d %d %d %d %d) ",
    trace_selftest_test_probe1_cnt,
    trace_selftest_test_probe2_cnt,
    trace_selftest_test_probe3_cnt,
    trace_selftest_test_global_cnt,
    trace_selftest_test_dyn_cnt);
    }
#[no_mangle]
unsafe extern "C" fn reset_counts() {
    static void reset_counts(void)
    {
    trace_selftest_test_probe1_cnt = 0;
    trace_selftest_test_probe2_cnt = 0;
    trace_selftest_test_probe3_cnt = 0;
    trace_selftest_test_global_cnt = 0;
    trace_selftest_test_dyn_cnt = 0;
    }
#[no_mangle]
unsafe extern "C" fn trace_selftest_ops(tr: *mut trace_array, cnt: c_int) -> c_int {
    static int trace_selftest_ops(struct trace_array *tr, int cnt)
    {
    let mut save_ftrace_enabled: c_int = ftrace_enabled;
    struct ftrace_ops *dyn_ops;
    char *func1_name;
    char *func2_name;
    int len1;
    int len2;
    let mut ret: c_int = -1;
    printk(KERN_CONT "PASSED\n");
    pr_info("Testing dynamic ftrace ops #%d: ", cnt);
    ftrace_enabled = 1;
    reset_counts();
// Handle PPC64 '.' name
    func1_name = "*" __stringify(DYN_FTRACE_TEST_NAME);
    func2_name = "*" __stringify(DYN_FTRACE_TEST_NAME2);
    len1 = strlen(func1_name);
    len2 = strlen(func2_name);
//
// Probe 1 will trace function 1.
// Probe 2 will trace function 2.
// Probe 3 will trace functions 1 and 2.
//
    ftrace_set_filter(&test_probe1, func1_name, len1, 1);
    ftrace_set_filter(&test_probe2, func2_name, len2, 1);
    ftrace_set_filter(&test_probe3, func1_name, len1, 1);
    ftrace_set_filter(&test_probe3, func2_name, len2, 0);
    register_ftrace_function(&test_probe1);
    register_ftrace_function(&test_probe2);
    register_ftrace_function(&test_probe3);
// First time we are running with main function
    if (cnt > 1) {
    ftrace_init_array_ops(tr, trace_selftest_test_global_func);
    register_ftrace_function(tr.ops);
    }
    DYN_FTRACE_TEST_NAME();
    print_counts();
    if (trace_selftest_test_probe1_cnt != 1)
    goto out;
    if (trace_selftest_test_probe2_cnt != 0)
    goto out;
    if (trace_selftest_test_probe3_cnt != 1)
    goto out;
    if (cnt > 1) {
    if (trace_selftest_test_global_cnt == 0)
    goto out;
    }
    DYN_FTRACE_TEST_NAME2();
    print_counts();
    if (trace_selftest_test_probe1_cnt != 1)
    goto out;
    if (trace_selftest_test_probe2_cnt != 1)
    goto out;
    if (trace_selftest_test_probe3_cnt != 2)
    goto out;
// Add a dynamic probe
    dyn_ops = kzalloc_obj(*dyn_ops);
    if (!dyn_ops) {
    printk("MEMORY ERROR ");
    goto out;
    }
    dyn_ops.func = trace_selftest_test_dyn_func;
    register_ftrace_function(dyn_ops);
    trace_selftest_test_global_cnt = 0;
    DYN_FTRACE_TEST_NAME();
    print_counts();
    if (trace_selftest_test_probe1_cnt != 2)
    goto out_free;
    if (trace_selftest_test_probe2_cnt != 1)
    goto out_free;
    if (trace_selftest_test_probe3_cnt != 3)
    goto out_free;
    if (cnt > 1) {
    if (trace_selftest_test_global_cnt == 0)
    goto out_free;
    }
    if (trace_selftest_test_dyn_cnt == 0)
    goto out_free;
    DYN_FTRACE_TEST_NAME2();
    print_counts();
    if (trace_selftest_test_probe1_cnt != 2)
    goto out_free;
    if (trace_selftest_test_probe2_cnt != 2)
    goto out_free;
    if (trace_selftest_test_probe3_cnt != 4)
    goto out_free;
// Remove trace function from probe 3
    func1_name = "!" __stringify(DYN_FTRACE_TEST_NAME);
    len1 = strlen(func1_name);
    ftrace_set_filter(&test_probe3, func1_name, len1, 0);
    DYN_FTRACE_TEST_NAME();
    print_counts();
    if (trace_selftest_test_probe1_cnt != 3)
    goto out_free;
    if (trace_selftest_test_probe2_cnt != 2)
    goto out_free;
    if (trace_selftest_test_probe3_cnt != 4)
    goto out_free;
    if (cnt > 1) {
    if (trace_selftest_test_global_cnt == 0)
    goto out_free;
    }
    if (trace_selftest_test_dyn_cnt == 0)
    goto out_free;
    DYN_FTRACE_TEST_NAME2();
    print_counts();
    if (trace_selftest_test_probe1_cnt != 3)
    goto out_free;
    if (trace_selftest_test_probe2_cnt != 3)
    goto out_free;
    if (trace_selftest_test_probe3_cnt != 5)
    goto out_free;
    ret = 0;
    out_free:
    unregister_ftrace_function(dyn_ops);
    kfree(dyn_ops);
    out:
// Purposely unregister in the same order
    unregister_ftrace_function(&test_probe1);
    unregister_ftrace_function(&test_probe2);
    unregister_ftrace_function(&test_probe3);
    if (cnt > 1)
    unregister_ftrace_function(tr.ops);
    ftrace_reset_array_ops(tr);
// Make sure everything is off
    reset_counts();
    DYN_FTRACE_TEST_NAME();
    DYN_FTRACE_TEST_NAME();
    if (trace_selftest_test_probe1_cnt ||
    trace_selftest_test_probe2_cnt ||
    trace_selftest_test_probe3_cnt ||
    trace_selftest_test_global_cnt ||
    trace_selftest_test_dyn_cnt)
    ret = -1;
    ftrace_enabled = save_ftrace_enabled;
    return ret;
    }
// Test dynamic code modification and ftrace filters
    static int trace_selftest_startup_dynamic_tracing(struct tracer *trace,
    struct trace_array *tr,
    int (*func)(void))
    {
    let mut save_ftrace_enabled: c_int = ftrace_enabled;
    unsigned long count;
    char *func_name;
    int ret;
// The ftrace test PASSED
    printk(KERN_CONT "PASSED\n");
    pr_info("Testing dynamic ftrace: ");
// enable tracing, and record the filter function
    ftrace_enabled = 1;
// passed in by parameter to fool gcc from optimizing
    func();
//
// Some archs *cough*PowerPC*cough* add characters to the
// start of the function names. We simply put a '*' to
// accommodate them.
//
    func_name = "*" __stringify(DYN_FTRACE_TEST_NAME);
// filter only on our function
    ftrace_set_global_filter(func_name, strlen(func_name), 1);
// enable tracing
    ret = tracer_init(trace, tr);
    if (ret) {
    warn_failed_init_tracer(trace, ret);
    goto out;
    }
// Sleep for a 1/10 of a second
    msleep(100);
// we should have nothing in the buffer
    ret = trace_test_buffer(&tr.array_buffer, &count);
    if (ret)
    goto out;
    if (count) {
    ret = -1;
    printk(KERN_CONT ".. filter did not filter .. ");
    goto out;
    }
// call our function again
    func();
// sleep again
    msleep(100);
// stop the tracing.
    tracing_stop();
    ftrace_enabled = 0;
// check the trace buffer
    ret = trace_test_buffer(&tr.array_buffer, &count);
    ftrace_enabled = 1;
    tracing_start();
// we should only have one item
    if (!ret && count != 1) {
    trace.reset(tr);
    printk(KERN_CONT ".. filter failed count=%ld ..", count);
    ret = -1;
    goto out;
    }
// Test the ops with global tracing running
    ret = trace_selftest_ops(tr, 1);
    trace.reset(tr);
    out:
    ftrace_enabled = save_ftrace_enabled;
// Enable tracing on all functions again
    ftrace_set_global_filter(core::ptr::null_mut(), 0, 1);
// Test the ops with global tracing off
    if (!ret)
    ret = trace_selftest_ops(tr, 2);
    return ret;
    }
    static int trace_selftest_recursion_cnt;
    static void trace_selftest_test_recursion_func(unsigned long ip,
    unsigned long pip,
    struct ftrace_ops *op,
    struct ftrace_regs *fregs)
    {
//
// This function is registered without the recursion safe flag.
// The ftrace infrastructure should provide the recursion
// protection. If not, this will crash the kernel!
//
    if (trace_selftest_recursion_cnt++ > 10)
    return;
    DYN_FTRACE_TEST_NAME();
    }
    static void trace_selftest_test_recursion_safe_func(unsigned long ip,
    unsigned long pip,
    struct ftrace_ops *op,
    struct ftrace_regs *fregs)
    {
//
// We said we would provide our own recursion. By calling
// this function again, we should recurse back into this function
// and count again. But this only happens if the arch supports
// all of ftrace features and nothing else is using the function
// tracing utility.
//
    if (trace_selftest_recursion_cnt++)
    return;
    DYN_FTRACE_TEST_NAME();
    }
    static struct ftrace_ops test_rec_probe = {
    .func			= trace_selftest_test_recursion_func,
    .flags			= FTRACE_OPS_FL_RECURSION,
    };
    static struct ftrace_ops test_recsafe_probe = {
    .func			= trace_selftest_test_recursion_safe_func,
    };
    static int
    trace_selftest_function_recursion(void)
    {
    let mut save_ftrace_enabled: c_int = ftrace_enabled;
    char *func_name;
    int len;
    int ret;
// The previous test PASSED
    pr_cont("PASSED\n");
    pr_info("Testing ftrace recursion: ");
// enable tracing, and record the filter function
    ftrace_enabled = 1;
// Handle PPC64 '.' name
    func_name = "*" __stringify(DYN_FTRACE_TEST_NAME);
    len = strlen(func_name);
    ret = ftrace_set_filter(&test_rec_probe, func_name, len, 1);
    if (ret) {
    pr_cont("*Could not set filter* ");
    goto out;
    }
    ret = register_ftrace_function(&test_rec_probe);
    if (ret) {
    pr_cont("*could not register callback* ");
    goto out;
    }
    DYN_FTRACE_TEST_NAME();
    unregister_ftrace_function(&test_rec_probe);
    ret = -1;
//
// Recursion allows for transitions between context,
// and may call the callback twice.
//
    if (trace_selftest_recursion_cnt != 1 &&
    trace_selftest_recursion_cnt != 2) {
    pr_cont("*callback not called once (or twice) (%d)* ",
    trace_selftest_recursion_cnt);
    goto out;
    }
    trace_selftest_recursion_cnt = 1;
    pr_cont("PASSED\n");
    pr_info("Testing ftrace recursion safe: ");
    ret = ftrace_set_filter(&test_recsafe_probe, func_name, len, 1);
    if (ret) {
    pr_cont("*Could not set filter* ");
    goto out;
    }
    ret = register_ftrace_function(&test_recsafe_probe);
    if (ret) {
    pr_cont("*could not register callback* ");
    goto out;
    }
    DYN_FTRACE_TEST_NAME();
    unregister_ftrace_function(&test_recsafe_probe);
    ret = -1;
    if (trace_selftest_recursion_cnt != 2) {
    pr_cont("*callback not called expected 2 times (%d)* ",
    trace_selftest_recursion_cnt);
    goto out;
    }
    ret = 0;
    out:
    ftrace_enabled = save_ftrace_enabled;
    return ret;
    }

    static enum {
    TRACE_SELFTEST_REGS_START,
    TRACE_SELFTEST_REGS_FOUND,
    TRACE_SELFTEST_REGS_NOT_FOUND,
    } trace_selftest_regs_stat;
    static void trace_selftest_test_regs_func(unsigned long ip,
    unsigned long pip,
    struct ftrace_ops *op,
    struct ftrace_regs *fregs)
    {
    struct pt_regs *regs = ftrace_get_regs(fregs);
    if (regs)
    trace_selftest_regs_stat = TRACE_SELFTEST_REGS_FOUND;
    else
    trace_selftest_regs_stat = TRACE_SELFTEST_REGS_NOT_FOUND;
    }
    static struct ftrace_ops test_regs_probe = {
    .func		= trace_selftest_test_regs_func,
    .flags		= FTRACE_OPS_FL_SAVE_REGS,
    };
    static int
    trace_selftest_function_regs(void)
    {
    let mut save_ftrace_enabled: c_int = ftrace_enabled;
    char *func_name;
    int len;
    int ret;
    let mut supported: c_int = 0;

    supported = 1;

// The previous test PASSED
    pr_cont("PASSED\n");
    pr_info("Testing ftrace regs%s: ",
    !supported ? "(no arch support)" : "");
// enable tracing, and record the filter function
    ftrace_enabled = 1;
// Handle PPC64 '.' name
    func_name = "*" __stringify(DYN_FTRACE_TEST_NAME);
    len = strlen(func_name);
    ret = ftrace_set_filter(&test_regs_probe, func_name, len, 1);
//
// If DYNAMIC_FTRACE is not set, then we just trace all functions.
// This test really doesn't care.
//
    if (ret && ret != -ENODEV) {
    pr_cont("*Could not set filter* ");
    goto out;
    }
    ret = register_ftrace_function(&test_regs_probe);
//
// Now if the arch does not support passing regs, then this should
// have failed.
//
    if (!supported) {
    if (!ret) {
    pr_cont("*registered save-regs without arch support* ");
    goto out;
    }
    test_regs_probe.flags |= FTRACE_OPS_FL_SAVE_REGS_IF_SUPPORTED;
    ret = register_ftrace_function(&test_regs_probe);
    }
    if (ret) {
    pr_cont("*could not register callback* ");
    goto out;
    }
    DYN_FTRACE_TEST_NAME();
    unregister_ftrace_function(&test_regs_probe);
    ret = -1;
    switch (trace_selftest_regs_stat) {
    case TRACE_SELFTEST_REGS_START:
    pr_cont("*callback never called* ");
    goto out;
    case TRACE_SELFTEST_REGS_FOUND:
    if (supported)
    break;
    pr_cont("*callback received regs without arch support* ");
    goto out;
    case TRACE_SELFTEST_REGS_NOT_FOUND:
    if (!supported)
    break;
    pr_cont("*callback received core::ptr::null_mut() regs* ");
    goto out;
    }
    ret = 0;
    out:
    ftrace_enabled = save_ftrace_enabled;
    return ret;
    }
//
// Simple verification test of ftrace function tracer.
// Enable ftrace, sleep 1/10 second, and then read the trace
// buffer to see if all is in order.
//
    __init int
    trace_selftest_startup_function(struct tracer *trace, struct trace_array *tr)
    {
    let mut save_ftrace_enabled: c_int = ftrace_enabled;
    unsigned long count;
    int ret;

    if (ftrace_filter_param) {
    printk(KERN_CONT " ... kernel command line filter set: force PASS ... ");
    return 0;
    }

// make sure msleep has been recorded
    msleep(1);
// start the tracing
    ftrace_enabled = 1;
    ret = tracer_init(trace, tr);
    if (ret) {
    warn_failed_init_tracer(trace, ret);
    goto out;
    }
// Sleep for a 1/10 of a second
    msleep(100);
// stop the tracing.
    tracing_stop();
    ftrace_enabled = 0;
// check the trace buffer
    ret = trace_test_buffer(&tr.array_buffer, &count);
    ftrace_enabled = 1;
    trace.reset(tr);
    tracing_start();
    if (!ret && !count) {
    printk(KERN_CONT ".. no entries found ..");
    ret = -1;
    goto out;
    }
    ret = trace_selftest_startup_dynamic_tracing(trace, tr,
    DYN_FTRACE_TEST_NAME);
    if (ret)
    goto out;
    ret = trace_selftest_function_recursion();
    if (ret)
    goto out;
    ret = trace_selftest_function_regs();
    out:
    ftrace_enabled = save_ftrace_enabled;
// kill ftrace totally if we failed
    if (ret)
    ftrace_kill();
    return ret;
    }

pub const CHAR_NUMBER: c_int = 123;
pub const SHORT_NUMBER: c_int = 12345;
pub const WORD_NUMBER: c_int = 1234567890;

pub const ERRSTR_BUFLEN: c_int = 128;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fgraph_fixture {
    pub gops: fgraph_ops,
    pub store_size: c_int,
    pub store_type_name: *const c_char,
    pub error_str_buf: [c_char; ERRSTR_BUFLEN],
    pub error_str: *mut c_char,
}

    static __init int store_entry(struct ftrace_graph_ent *trace,
    struct fgraph_ops *gops,
    struct ftrace_regs *fregs)
    {
    struct fgraph_fixture *fixture = container_of(gops, struct fgraph_fixture, gops);
    const char *type = fixture.store_type_name;
    let mut size: c_int = fixture.store_size;
    void *p;
    p = fgraph_reserve_data(gops.idx, size);
    if (!p) {
    snprintf(fixture.error_str_buf, ERRSTR_BUFLEN,
    "Failed to reserve %s\n", type);
    return 0;
    }
    switch (size) {
    case 1:
// (char *)p = CHAR_NUMBER;
    break;
    case 2:
// (short *)p = SHORT_NUMBER;
    break;
    case 4:
// (int *)p = WORD_NUMBER;
    break;
    case 8:
// (long long *)p = LONG_NUMBER;
    break;
    }
    return 1;
    }
    static __init void store_return(struct ftrace_graph_ret *trace,
    struct fgraph_ops *gops,
    struct ftrace_regs *fregs)
    {
    struct fgraph_fixture *fixture = container_of(gops, struct fgraph_fixture, gops);
    const char *type = fixture.store_type_name;
    let mut expect: c_longlong = 0;
    let mut found: c_longlong = -1;
    int size;
    char *p;
    p = fgraph_retrieve_data(gops.idx, &size);
    if (!p) {
    snprintf(fixture.error_str_buf, ERRSTR_BUFLEN,
    "Failed to retrieve %s\n", type);
    return;
    }
    if (fixture.store_size > size) {
    snprintf(fixture.error_str_buf, ERRSTR_BUFLEN,
    "Retrieved size %d is smaller than expected %d\n",
    size, (int)fixture.store_size);
    return;
    }
    switch (fixture.store_size) {
    case 1:
    expect = CHAR_NUMBER;
    found = *(char *)p;
    break;
    case 2:
    expect = SHORT_NUMBER;
    found = *(short *)p;
    break;
    case 4:
    expect = WORD_NUMBER;
    found = *(int *)p;
    break;
    case 8:
    expect = LONG_NUMBER;
    found = *(long long *)p;
    break;
    }
    if (found != expect) {
    snprintf(fixture.error_str_buf, ERRSTR_BUFLEN,
    "%s returned not %lld but %lld\n", type, expect, found);
    return;
    }
    fixture.error_str = core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn init_fgraph_fixture(fixture: *mut fgraph_fixture) -> int __init {
    static int __init init_fgraph_fixture(struct fgraph_fixture *fixture)
    {
    char *func_name;
    int len;
    snprintf(fixture.error_str_buf, ERRSTR_BUFLEN,
    "Failed to execute storage %s\n", fixture.store_type_name);
    fixture.error_str = fixture.error_str_buf;
    func_name = "*" __stringify(DYN_FTRACE_TEST_NAME);
    len = strlen(func_name);
    return ftrace_set_filter(&fixture.gops.ops, func_name, len, 1);
    }
// Test fgraph storage for each size
#[no_mangle]
unsafe extern "C" fn test_graph_storage_single(fixture: *mut fgraph_fixture) -> int __init {
    static int __init test_graph_storage_single(struct fgraph_fixture *fixture)
    {
    let mut size: c_int = fixture.store_size;
    int ret;
    pr_cont("PASSED\n");
    pr_info("Testing fgraph storage of %d byte%s: ", size, str_plural(size));
    ret = init_fgraph_fixture(fixture);
    if (ret && ret != -ENODEV) {
    pr_cont("*Could not set filter* ");
    return -1;
    }
    ret = register_ftrace_graph(&fixture.gops);
    if (ret) {
    pr_warn("Failed to init store_bytes fgraph tracing\n");
    return -1;
    }
    DYN_FTRACE_TEST_NAME();
    unregister_ftrace_graph(&fixture.gops);
    if (fixture.error_str) {
    pr_cont("*** %s ***", fixture.error_str);
    return -1;
    }
    return 0;
    }
    static struct fgraph_fixture store_bytes[4] __initdata = {
    [0] = {
    .gops = {
    .entryfunc		= store_entry,
    .retfunc		= store_return,
    },
    .store_size = 1,
    .store_type_name = "byte",
    },
    [1] = {
    .gops = {
    .entryfunc		= store_entry,
    .retfunc		= store_return,
    },
    .store_size = 2,
    .store_type_name = "short",
    },
    [2] = {
    .gops = {
    .entryfunc		= store_entry,
    .retfunc		= store_return,
    },
    .store_size = 4,
    .store_type_name = "word",
    },
    [3] = {
    .gops = {
    .entryfunc		= store_entry,
    .retfunc		= store_return,
    },
    .store_size = 8,
    .store_type_name = "long long",
    },
    };
#[no_mangle]
unsafe extern "C" fn test_graph_storage_multi() -> __init int {
    static __init int test_graph_storage_multi(void)
    {
    struct fgraph_fixture *fixture;
    let mut printed: bool = false;
    int i, j, ret;
    pr_cont("PASSED\n");
    pr_info("Testing multiple fgraph storage on a function: ");
    for (i = 0; i < ARRAY_SIZE(store_bytes); i++) {
    fixture = &store_bytes[i];
    ret = init_fgraph_fixture(fixture);
    if (ret && ret != -ENODEV) {
    pr_cont("*Could not set filter* ");
    printed = true;
    goto out2;
    }
    }
    for (j = 0; j < ARRAY_SIZE(store_bytes); j++) {
    fixture = &store_bytes[j];
    ret = register_ftrace_graph(&fixture.gops);
    if (ret) {
    pr_warn("Failed to init store_bytes fgraph tracing\n");
    printed = true;
    goto out1;
    }
    }
    DYN_FTRACE_TEST_NAME();
    out1:
    while (--j >= 0) {
    fixture = &store_bytes[j];
    unregister_ftrace_graph(&fixture.gops);
    if (fixture.error_str && !printed) {
    pr_cont("*** %s ***", fixture.error_str);
    printed = true;
    }
    }
    out2:
    while (--i >= 0) {
    fixture = &store_bytes[i];
    ftrace_free_filter(&fixture.gops.ops);
    if (fixture.error_str && !printed) {
    pr_cont("*** %s ***", fixture.error_str);
    printed = true;
    }
    }
    return printed ? -1 : 0;
    }
// Test the storage passed across function_graph entry and return
#[no_mangle]
unsafe extern "C" fn test_graph_storage() -> __init int {
    static __init int test_graph_storage(void)
    {
    int ret;
    ret = test_graph_storage_single(&store_bytes[0]);
    if (ret)
    return ret;
    ret = test_graph_storage_single(&store_bytes[1]);
    if (ret)
    return ret;
    ret = test_graph_storage_single(&store_bytes[2]);
    if (ret)
    return ret;
    ret = test_graph_storage_single(&store_bytes[3]);
    if (ret)
    return ret;
    ret = test_graph_storage_multi();
    if (ret)
    return ret;
    return 0;
    }

    static inline int test_graph_storage(void) { return 0; }

// Maximum number of functions to trace before diagnosing a hang
pub const GRAPH_MAX_FUNC_TEST: c_int = 100000000;
    static unsigned int graph_hang_thresh;
// Wrap the real function entry probe to avoid possible hanging
    static int trace_graph_entry_watchdog(struct ftrace_graph_ent *trace,
    struct fgraph_ops *gops,
    struct ftrace_regs *fregs)
    {
// This is harmlessly racy, we want to approximately detect a hang
    if (unlikely(++graph_hang_thresh > GRAPH_MAX_FUNC_TEST)) {
    ftrace_graph_stop();
    printk(KERN_WARNING "BUG: Function graph tracer hang!\n");
    if (ftrace_dump_on_oops_enabled()) {
    ftrace_dump(DUMP_ALL);
// ftrace_dump() disables tracing
    tracing_on();
    }
    return 0;
    }
    return trace_graph_entry(trace, gops, fregs);
    }
    static struct fgraph_ops fgraph_ops __initdata  = {
    .entryfunc		= &trace_graph_entry_watchdog,
    .retfunc		= &trace_graph_return,
    };

    static struct ftrace_ops direct;

//
// Pretty much the same than for the function tracer from which the selftest
// has been borrowed.
//
    __init int
    trace_selftest_startup_function_graph(struct tracer *trace,
    struct trace_array *tr)
    {
    int ret;
    unsigned long count;
    char *func_name __maybe_unused;

    if (ftrace_filter_param) {
    printk(KERN_CONT " ... kernel command line filter set: force PASS ... ");
    return 0;
    }

//
// Simulate the init() callback but we attach a watchdog callback
// to detect and recover from possible hangs
//
    tracing_reset_online_cpus(&tr.array_buffer);
    fgraph_ops.private = tr;
    ret = register_ftrace_graph(&fgraph_ops);
    if (ret) {
    warn_failed_init_tracer(trace, ret);
    goto out;
    }
    tracing_start_cmdline_record();
// Sleep for a 1/10 of a second
    msleep(100);
// Have we just recovered from a hang?
    if (graph_hang_thresh > GRAPH_MAX_FUNC_TEST) {
    disable_tracing_selftest("recovering from a hang");
    ret = -1;
    goto out;
    }
    tracing_stop();
// check the trace buffer
    ret = trace_test_buffer(&tr.array_buffer, &count);
// Need to also simulate the tr->reset to remove this fgraph_ops
    tracing_stop_cmdline_record();
    unregister_ftrace_graph(&fgraph_ops);
    tracing_start();
    if (!ret && !count) {
    printk(KERN_CONT ".. no entries found ..");
    ret = -1;
    goto out;
    }

//
// These tests can take some time to run. Make sure on non PREEMPT
// kernels, we do not trigger the softlockup detector.
//
    cond_resched();
    tracing_reset_online_cpus(&tr.array_buffer);
    fgraph_ops.private = tr;
//
// Some archs *cough*PowerPC*cough* add characters to the
// start of the function names. We simply put a '*' to
// accommodate them.
//
    func_name = "*" __stringify(DYN_FTRACE_TEST_NAME);
    ftrace_set_global_filter(func_name, strlen(func_name), 1);
//
// Register direct function together with graph tracer
// and make sure we get graph trace.
//
    ftrace_set_filter_ip(&direct, (unsigned long)DYN_FTRACE_TEST_NAME, 0, 0);
    ret = register_ftrace_direct(&direct,
    (unsigned long)ftrace_stub_direct_tramp);
    if (ret)
    goto out;
    cond_resched();
    ret = register_ftrace_graph(&fgraph_ops);
    if (ret) {
    warn_failed_init_tracer(trace, ret);
    goto out;
    }
    DYN_FTRACE_TEST_NAME();
    count = 0;
    tracing_stop();
// check the trace buffer
    ret = trace_test_buffer(&tr.array_buffer, &count);
    unregister_ftrace_graph(&fgraph_ops);
    ret = unregister_ftrace_direct(&direct,
    (unsigned long)ftrace_stub_direct_tramp,
    true);
    if (ret)
    goto out;
    cond_resched();
    tracing_start();
    if (!ret && !count) {
    ret = -1;
    goto out;
    }
// Enable tracing on all functions again
    ftrace_set_global_filter(core::ptr::null_mut(), 0, 1);

    ret = test_graph_storage();
// Don't test dynamic tracing, the function tracer already did
    out:
// Stop it if we failed
    if (ret)
    ftrace_graph_stop();
    return ret;
    }

    int
    trace_selftest_startup_irqsoff(struct tracer *trace, struct trace_array *tr)
    {
    let mut save_max: c_ulong = tr.max_latency;
    unsigned long count;
    int ret;
// start the tracing
    ret = tracer_init(trace, tr);
    if (ret) {
    warn_failed_init_tracer(trace, ret);
    return ret;
    }
// reset the max latency
    tr.max_latency = 0;
// disable interrupts for a bit
    local_irq_disable();
    udelay(100);
    local_irq_enable();
//
// Stop the tracer to avoid a warning subsequent
// to buffer flipping failure because tracing_stop()
// disables the tr and max buffers, making flipping impossible
// in case of parallels max irqs off latencies.
//
    trace.stop(tr);
// stop the tracing.
    tracing_stop();
// check both trace buffers
    ret = trace_test_buffer(&tr.array_buffer, core::ptr::null_mut());
    if (!ret)
    ret = trace_test_buffer(&tr.snapshot_buffer, &count);
    trace.reset(tr);
    tracing_start();
    if (!ret && !count) {
    printk(KERN_CONT ".. no entries found ..");
    ret = -1;
    }
    tr.max_latency = save_max;
    return ret;
    }

    int
    trace_selftest_startup_preemptoff(struct tracer *trace, struct trace_array *tr)
    {
    let mut save_max: c_ulong = tr.max_latency;
    unsigned long count;
    int ret;
//
// Now that the big kernel lock is no longer preemptible,
// and this is called with the BKL held, it will always
// fail. If preemption is already disabled, simply
// pass the test. When the BKL is removed, or becomes
// preemptible again, we will once again test this,
// so keep it in.
//
    if (preempt_count()) {
    printk(KERN_CONT "can not test ... force ");
    return 0;
    }
// start the tracing
    ret = tracer_init(trace, tr);
    if (ret) {
    warn_failed_init_tracer(trace, ret);
    return ret;
    }
// reset the max latency
    tr.max_latency = 0;
// disable preemption for a bit
    preempt_disable();
    udelay(100);
    preempt_enable();
//
// Stop the tracer to avoid a warning subsequent
// to buffer flipping failure because tracing_stop()
// disables the tr and max buffers, making flipping impossible
// in case of parallels max preempt off latencies.
//
    trace.stop(tr);
// stop the tracing.
    tracing_stop();
// check both trace buffers
    ret = trace_test_buffer(&tr.array_buffer, core::ptr::null_mut());
    if (!ret)
    ret = trace_test_buffer(&tr.snapshot_buffer, &count);
    trace.reset(tr);
    tracing_start();
    if (!ret && !count) {
    printk(KERN_CONT ".. no entries found ..");
    ret = -1;
    }
    tr.max_latency = save_max;
    return ret;
    }

    int
    trace_selftest_startup_preemptirqsoff(struct tracer *trace, struct trace_array *tr)
    {
    let mut save_max: c_ulong = tr.max_latency;
    unsigned long count;
    int ret;
//
// Now that the big kernel lock is no longer preemptible,
// and this is called with the BKL held, it will always
// fail. If preemption is already disabled, simply
// pass the test. When the BKL is removed, or becomes
// preemptible again, we will once again test this,
// so keep it in.
//
    if (preempt_count()) {
    printk(KERN_CONT "can not test ... force ");
    return 0;
    }
// start the tracing
    ret = tracer_init(trace, tr);
    if (ret) {
    warn_failed_init_tracer(trace, ret);
    goto out_no_start;
    }
// reset the max latency
    tr.max_latency = 0;
// disable preemption and interrupts for a bit
    preempt_disable();
    local_irq_disable();
    udelay(100);
    preempt_enable();
// reverse the order of preempt vs irqs
    local_irq_enable();
//
// Stop the tracer to avoid a warning subsequent
// to buffer flipping failure because tracing_stop()
// disables the tr and max buffers, making flipping impossible
// in case of parallels max irqs/preempt off latencies.
//
    trace.stop(tr);
// stop the tracing.
    tracing_stop();
// check both trace buffers
    ret = trace_test_buffer(&tr.array_buffer, core::ptr::null_mut());
    if (ret)
    goto out;
    ret = trace_test_buffer(&tr.snapshot_buffer, &count);
    if (ret)
    goto out;
    if (!ret && !count) {
    printk(KERN_CONT ".. no entries found ..");
    ret = -1;
    goto out;
    }
// do the test by disabling interrupts first this time
    tr.max_latency = 0;
    tracing_start();
    trace.start(tr);
    preempt_disable();
    local_irq_disable();
    udelay(100);
    preempt_enable();
// reverse the order of preempt vs irqs
    local_irq_enable();
    trace.stop(tr);
// stop the tracing.
    tracing_stop();
// check both trace buffers
    ret = trace_test_buffer(&tr.array_buffer, core::ptr::null_mut());
    if (ret)
    goto out;
    ret = trace_test_buffer(&tr.snapshot_buffer, &count);
    if (!ret && !count) {
    printk(KERN_CONT ".. no entries found ..");
    ret = -1;
    goto out;
    }
    out:
    tracing_start();
    out_no_start:
    trace.reset(tr);
    tr.max_latency = save_max;
    return ret;
    }

    int
    trace_selftest_startup_nop(struct tracer *trace, struct trace_array *tr)
    {
// What could possibly go wrong?
    return 0;
    }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wakeup_test_data {
    pub is_ready: completion,
    pub go: c_int,
}

#[no_mangle]
unsafe extern "C" fn trace_wakeup_test_thread(data: *mut c_void) -> c_int {
    static int trace_wakeup_test_thread(void *data)
    {
// Make this a -deadline thread
    static const struct sched_attr attr = {
    .sched_policy = SCHED_DEADLINE,
    .sched_runtime = 100000ULL,
    .sched_deadline = 10000000ULL,
    .sched_period = 10000000ULL
    };
    struct wakeup_test_data *x = data;
    sched_setattr(current, &attr);
// Make it know we have a new prio
    complete(&x.is_ready);
// now go to sleep and let the test wake us up
    set_current_state(TASK_INTERRUPTIBLE);
    while (!x.go) {
    schedule();
    set_current_state(TASK_INTERRUPTIBLE);
    }
    complete(&x.is_ready);
    set_current_state(TASK_INTERRUPTIBLE);
// we are awake, now wait to disappear
    while (!kthread_should_stop()) {
    schedule();
    set_current_state(TASK_INTERRUPTIBLE);
    }
    __set_current_state(TASK_RUNNING);
    return 0;
    }
    int
    trace_selftest_startup_wakeup(struct tracer *trace, struct trace_array *tr)
    {
    let mut save_max: c_ulong = tr.max_latency;
    struct task_struct *p;
    struct wakeup_test_data data;
    unsigned long count;
    int ret;
    memset(&data, 0, sizeof(data));
    init_completion(&data.is_ready);
// create a -deadline thread
    p = kthread_run(trace_wakeup_test_thread, &data, "ftrace-test");
    if (IS_ERR(p)) {
    printk(KERN_CONT "Failed to create ftrace wakeup test thread ");
    return -1;
    }
// make sure the thread is running at -deadline policy
    wait_for_completion(&data.is_ready);
// start the tracing
    ret = tracer_init(trace, tr);
    if (ret) {
    warn_failed_init_tracer(trace, ret);
    return ret;
    }
// reset the max latency
    tr.max_latency = 0;
    while (task_is_runnable(p)) {
//
// Sleep to make sure the -deadline thread is asleep too.
// On virtual machines we can't rely on timings,
// but we want to make sure this test still works.
//
    msleep(100);
    }
    init_completion(&data.is_ready);
    data.go = 1;
// memory barrier is in the wake_up_process()
    wake_up_process(p);
// Wait for the task to wake up
    wait_for_completion(&data.is_ready);
// stop the tracing.
    tracing_stop();
// check both trace buffers
    ret = trace_test_buffer(&tr.array_buffer, core::ptr::null_mut());
    if (!ret)
    ret = trace_test_buffer(&tr.snapshot_buffer, &count);
    trace.reset(tr);
    tracing_start();
    tr.max_latency = save_max;
// kill the thread
    kthread_stop(p);
    if (!ret && !count) {
    printk(KERN_CONT ".. no entries found ..");
    ret = -1;
    }
    return ret;
    }

    int
    trace_selftest_startup_branch(struct tracer *trace, struct trace_array *tr)
    {
    unsigned long count;
    int ret;
// start the tracing
    ret = tracer_init(trace, tr);
    if (ret) {
    warn_failed_init_tracer(trace, ret);
    return ret;
    }
// Sleep for a 1/10 of a second
    msleep(100);
// stop the tracing.
    tracing_stop();
// check the trace buffer
    ret = trace_test_buffer(&tr.array_buffer, &count);
    trace.reset(tr);
    tracing_start();
    if (!ret && !count) {
    printk(KERN_CONT ".. no entries found ..");
    ret = -1;
    }
    return ret;
    }
