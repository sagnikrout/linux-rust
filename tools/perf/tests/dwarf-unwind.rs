//! Automatically rewritten from C to Rust
//! Source: tools/perf/tests/dwarf-unwind.c
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

// For bsearch. We try to unwind functions in shared object.

//
// The test will assert frames are on the stack but tail call optimizations lose
// the frame of the caller. Clang can disable this optimization on a called
// function but GCC currently (11/2020) lacks this attribute. The barrier is
// used to inhibit tail calls in these cases.
//

// Macro flag: #define NO_TAIL_CALL_BARRIER

// Macro flag: #define NO_TAIL_CALL_ATTRIBUTE

//
// We need to keep these functions global, despite the
// fact that they are used only locally in this object,
// in order to keep them around even if the binary is
// stripped. If they are gone, the unwind check for
// symbol fails.
//
    int test_dwarf_unwind__thread(struct thread *thread);
    int test_dwarf_unwind__compare(void *p1, void *p2);
    int test_dwarf_unwind__krava_3(struct thread *thread);
    int test_dwarf_unwind__krava_2(struct thread *thread);
    int test_dwarf_unwind__krava_1(struct thread *thread);
    int test__dwarf_unwind(struct test_suite *test, int subtest);
pub const MAX_STACK: c_int = 8;
#[no_mangle]
unsafe extern "C" fn unwind_entry(entry: *mut unwind_entry, arg: *mut c_void) -> c_int {
    static int unwind_entry(struct unwind_entry *entry, void *arg)
    {
    unsigned long *cnt = (unsigned long *) arg;
    char *symbol = entry.ms.sym ? entry.ms.sym.name : core::ptr::null_mut();
    static const char *funcs[MAX_STACK] = {
    "test__arch_unwind_sample",
    "test_dwarf_unwind__thread",
    "test_dwarf_unwind__compare",
    "bsearch",
    "test_dwarf_unwind__krava_3",
    "test_dwarf_unwind__krava_2",
    "test_dwarf_unwind__krava_1",
    "test__dwarf_unwind"
    };
//
// The funcs[MAX_STACK] array index, based on the
// callchain order setup.
//
    int idx = callchain_param.order == ORDER_CALLER ?
    MAX_STACK - *cnt - 1 : *cnt;
    if (*cnt >= MAX_STACK) {
    pr_debug("failed: crossed the max stack value %d\n", MAX_STACK);
    return -1;
    }
    if (!symbol) {
    pr_debug("failed: got unresolved address 0x%" PRIx64 "\n",
    entry.ip);
    return -1;
    }
    (*cnt)++;
    pr_debug("got: %s 0x%" PRIx64 ", expecting %s\n",
    symbol, entry.ip, funcs[idx]);
    return strcmp((const char *) symbol, funcs[idx]);
    }
#[no_mangle]
pub unsafe extern "C" fn test_dwarf_unwind__thread(thread: *mut thread) -> NO_TAIL_CALL_ATTRIBUTE noinline int {
    NO_TAIL_CALL_ATTRIBUTE noinline int test_dwarf_unwind__thread(struct thread *thread)
    {
    struct perf_sample sample;
    let mut cnt: c_ulong = 0;
    let mut err: c_int = -1;
    perf_sample__init(&sample, /*all=*/true);
    if (test__arch_unwind_sample(&sample, thread)) {
    pr_debug("failed to get unwind sample\n");
    goto out;
    }
    err = unwind__get_entries(unwind_entry, &cnt, thread,
    &sample, MAX_STACK, false);
    if (err)
    pr_debug("unwind failed\n");
#[no_mangle]
pub unsafe extern "C" fn if(MAX_STACK: cnt !=) -> else {
    pr_debug("got wrong number of stack entries %lu != %d\n",
    cnt, MAX_STACK);
    err = -1;
    }
    out:
    zfree(&sample.user_stack.data);
    zfree(&sample.user_regs.regs);
    perf_sample__exit(&sample);
    return err;
    }
    let mut global_unwind_retval: static int = -INT_MAX;
#[no_mangle]
pub unsafe extern "C" fn test_dwarf_unwind__compare(p1: *mut c_void, p2: *mut c_void) -> NO_TAIL_CALL_ATTRIBUTE noinline int {
    NO_TAIL_CALL_ATTRIBUTE noinline int test_dwarf_unwind__compare(void *p1, void *p2)
    {
// Any possible value should be 'thread'
    struct thread *thread = *(struct thread **)p1;
    if (global_unwind_retval == -INT_MAX) {
// Call unwinder twice for both callchain orders.
    callchain_param.order = ORDER_CALLER;
    global_unwind_retval = test_dwarf_unwind__thread(thread);
    if (!global_unwind_retval) {
    callchain_param.order = ORDER_CALLEE;
    global_unwind_retval = test_dwarf_unwind__thread(thread);
    }
    }
    return p1 - p2;
    }
#[no_mangle]
pub unsafe extern "C" fn test_dwarf_unwind__krava_3(thread: *mut thread) -> NO_TAIL_CALL_ATTRIBUTE noinline int {
    NO_TAIL_CALL_ATTRIBUTE noinline int test_dwarf_unwind__krava_3(struct thread *thread)
    {
    struct thread *array[2] = {thread, thread};
    void *fp = &bsearch;
//
// make _bsearch a volatile function pointer to
// prevent potential optimization, which may expand
// bsearch and call compare directly from this function,
// instead of libc shared object.
//
    void *(*volatile _bsearch)(void *, void *, size_t,
    size_t, int (*)(void *, void *));
    _bsearch = fp;
    _bsearch(array, &thread, 2, sizeof(struct thread **),
    test_dwarf_unwind__compare);
    return global_unwind_retval;
    }
#[no_mangle]
pub unsafe extern "C" fn test_dwarf_unwind__krava_2(thread: *mut thread) -> NO_TAIL_CALL_ATTRIBUTE noinline int {
    NO_TAIL_CALL_ATTRIBUTE noinline int test_dwarf_unwind__krava_2(struct thread *thread)
    {
    int ret;
    ret =  test_dwarf_unwind__krava_3(thread);
    NO_TAIL_CALL_BARRIER;
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn test_dwarf_unwind__krava_1(thread: *mut thread) -> NO_TAIL_CALL_ATTRIBUTE noinline int {
    NO_TAIL_CALL_ATTRIBUTE noinline int test_dwarf_unwind__krava_1(struct thread *thread)
    {
    int ret;
    ret =  test_dwarf_unwind__krava_2(thread);
    NO_TAIL_CALL_BARRIER;
    return ret;
    }
    noinline int test__dwarf_unwind(struct test_suite *test __maybe_unused,
    int subtest __maybe_unused)
    {
    struct perf_env host_env;
    struct machine *machine;
    struct thread *thread;
    let mut err: c_int = -1;
    let mut pid: pid_t = getpid();
    callchain_param.record_mode = CALLCHAIN_DWARF;
    dwarf_callchain_users = true;
    perf_env__init(&host_env);
    machine = machine__new_live(&host_env, /*kernel_maps=*/true, pid);
    if (!machine) {
    pr_err("Could not get machine\n");
    goto out;
    }
    if (machine__create_kernel_maps(machine)) {
    pr_err("Failed to create kernel maps\n");
    goto out;
    }
    if (verbose > 1)
    machine__fprintf(machine, stderr);
    thread = machine__find_thread(machine, pid, pid);
    if (!thread) {
    pr_err("Could not get thread\n");
    goto out;
    }
    err = test_dwarf_unwind__krava_1(thread);
    thread__put(thread);
    out:
    machine__delete(machine);
    perf_env__exit(&host_env);
    return err;
    }
    DEFINE_SUITE("Test dwarf unwind", dwarf_unwind);
