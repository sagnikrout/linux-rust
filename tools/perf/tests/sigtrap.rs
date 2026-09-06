//! Automatically rewritten from C to Rust
//! Source: tools/perf/tests/sigtrap.c
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
// Basic test for sigtrap support.
//
// Copyright (C) 2021, Google LLC.
//

pub const NUM_THREADS: c_int = 5;
    static struct {
    int tids_want_signal;		/* Which threads still want a signal. */
    int signal_count;		/* Sanity check number of signals received. */
    volatile int iterate_on;	/* Variable to set breakpoint on. */
    siginfo_t first_siginfo;	/* First observed siginfo_t. */
    } ctx;

#[no_mangle]
unsafe extern "C" fn make_event_attr() -> perf_event_attr {
    static struct perf_event_attr make_event_attr(void)
    {
    struct perf_event_attr attr = {
    .type		= PERF_TYPE_BREAKPOINT,
    .size		= sizeof(attr),
    .sample_period	= 1,
    .disabled	= 1,
    .bp_addr	= (unsigned long)&ctx.iterate_on,
    .bp_type	= HW_BREAKPOINT_RW,
    .bp_len		= HW_BREAKPOINT_LEN_1,
    .inherit	= 1, /* Children inherit events ... */
    .inherit_thread = 1, /* ... but only cloned with CLONE_THREAD. */
    .remove_on_exec = 1, /* Required by sigtrap. */
    .sigtrap	= 1, /* Request synchronous SIGTRAP on event. */
    .sig_data	= TEST_SIG_DATA,
    .exclude_kernel = 1, /* To allow */
    .exclude_hv     = 1, /* running as !root */
    };
    return attr;
    }

    static struct btf *btf;
#[no_mangle]
unsafe extern "C" fn btf__available() -> bool {
    static bool btf__available(void)
    {
    if (btf == core::ptr::null_mut())
    btf = btf__load_vmlinux_btf();
    return btf != core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn btf__exit() {
    static void btf__exit(void)
    {
    btf__free(btf);
    btf = core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn attr_has_sigtrap() -> bool {
    static bool attr_has_sigtrap(void)
    {
    int id;
    if (!btf__available()) {
// should be an old kernel
    return false;
    }
    id = btf__find_by_name_kind(btf, "perf_event_attr", BTF_KIND_STRUCT);
    if (id < 0)
    return false;
    return __btf_type__find_member_by_name(btf, id, "sigtrap") != core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn kernel_with_sleepable_spinlocks() -> bool {
    static bool kernel_with_sleepable_spinlocks(void)
    {
    const struct btf_member *member;
    const struct btf_type *type;
    const char *type_name;
    int id;
    if (!btf__available())
    return false;
    id = btf__find_by_name_kind(btf, "spinlock", BTF_KIND_STRUCT);
    if (id < 0)
    return false;
// Only RT has a "lock" member for "struct spinlock"
    member = __btf_type__find_member_by_name(btf, id, "lock");
    if (member == core::ptr::null_mut())
    return false;
// But check its type as well
    type = btf__type_by_id(btf, member.type);
    if (!type || !btf_is_struct(type))
    return false;
    type_name = btf__name_by_offset(btf, type.name_off);
    return type_name && !strcmp(type_name, "rt_mutex_base");
    }

#[no_mangle]
unsafe extern "C" fn attr_has_sigtrap() -> bool {
    static bool attr_has_sigtrap(void)
    {
    struct perf_event_attr attr = {
    .type		= PERF_TYPE_SOFTWARE,
    .config		= PERF_COUNT_SW_DUMMY,
    .size		= sizeof(attr),
    .remove_on_exec = 1, /* Required by sigtrap. */
    .sigtrap	= 1, /* Request synchronous SIGTRAP on event. */
    };
    int fd;
    let mut ret: bool = false;
    fd = sys_perf_event_open(&attr, 0, -1, -1, perf_event_open_cloexec_flag());
    if (fd >= 0) {
    ret = true;
    close(fd);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn kernel_with_sleepable_spinlocks() -> bool {
    static bool kernel_with_sleepable_spinlocks(void)
    {
    return false;
    }
#[no_mangle]
unsafe extern "C" fn btf__exit() {
    static void btf__exit(void)
    {
    }

    static void
    sigtrap_handler(int signum __maybe_unused, siginfo_t *info, void *ucontext __maybe_unused)
    {
    if (!__atomic_fetch_add(&ctx.signal_count, 1, __ATOMIC_RELAXED))
    ctx.first_siginfo = *info;
    __atomic_fetch_sub(&ctx.tids_want_signal, syscall(SYS_gettid), __ATOMIC_RELAXED);
    }
    static void *test_thread(void *arg)
    {
    pthread_barrier_t *barrier = (pthread_barrier_t *)arg;
    let mut tid: pid_t = syscall(SYS_gettid);
    int i;
    pthread_barrier_wait(barrier);
    __atomic_fetch_add(&ctx.tids_want_signal, tid, __ATOMIC_RELAXED);
    for (i = 0; i < ctx.iterate_on - 1; i++)
    __atomic_fetch_add(&ctx.tids_want_signal, tid, __ATOMIC_RELAXED);
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn run_test_threads(threads: *mut pthread_t, barrier: *mut pthread_barrier_t) -> c_int {
    static int run_test_threads(pthread_t *threads, pthread_barrier_t *barrier)
    {
    int i;
    pthread_barrier_wait(barrier);
    for (i = 0; i < NUM_THREADS; i++)
    TEST_ASSERT_EQUAL("pthread_join() failed", pthread_join(threads[i], core::ptr::null_mut()), 0);
    return TEST_OK;
    }
#[no_mangle]
unsafe extern "C" fn run_stress_test(fd: c_int, threads: *mut pthread_t, barrier: *mut pthread_barrier_t) -> c_int {
    static int run_stress_test(int fd, pthread_t *threads, pthread_barrier_t *barrier)
    {
    int ret, expected_sigtraps;
    ctx.iterate_on = 3000;
    TEST_ASSERT_EQUAL("misfired signal?", ctx.signal_count, 0);
    TEST_ASSERT_EQUAL("enable failed", ioctl(fd, PERF_EVENT_IOC_ENABLE, 0), 0);
    ret = run_test_threads(threads, barrier);
    TEST_ASSERT_EQUAL("disable failed", ioctl(fd, PERF_EVENT_IOC_DISABLE, 0), 0);
    expected_sigtraps = NUM_THREADS * ctx.iterate_on;
    if (ctx.signal_count < expected_sigtraps && kernel_with_sleepable_spinlocks()) {
    pr_debug("Expected %d sigtraps, got %d, running on a kernel with sleepable spinlocks.\n",
    expected_sigtraps, ctx.signal_count);
    pr_debug("See https://lore.kernel.org/all/e368f2c848d77fbc8d259f44e2055fe469c219cf.camel@gmx.de/\n");
    return TEST_SKIP;
    } else
    TEST_ASSERT_EQUAL("unexpected sigtraps", ctx.signal_count, expected_sigtraps);
    TEST_ASSERT_EQUAL("missing signals or incorrectly delivered", ctx.tids_want_signal, 0);
    TEST_ASSERT_VAL("unexpected si_addr", ctx.first_siginfo.si_addr == &ctx.iterate_on);

    TEST_ASSERT_EQUAL("unexpected si_perf_type", ctx.first_siginfo.si_perf_type,
    PERF_TYPE_BREAKPOINT);
    TEST_ASSERT_EQUAL("unexpected si_perf_data", ctx.first_siginfo.si_perf_data,
    TEST_SIG_DATA);

    return ret;
    }
#[no_mangle]
unsafe extern "C" fn test__sigtrap(__maybe_unused: *mut *mut test_suite test, __maybe_unused: int subtest) -> c_int {
    static int test__sigtrap(struct test_suite *test __maybe_unused, int subtest __maybe_unused)
    {
    let mut attr: perf_event_attr = make_event_attr();
    let mut action: sigaction = {};
    struct sigaction oldact;
    pthread_t threads[NUM_THREADS];
    pthread_barrier_t barrier;
    char sbuf[STRERR_BUFSIZE];
    int i, fd, ret = TEST_FAIL;
    if (!BP_SIGNAL_IS_SUPPORTED) {
    pr_debug("Test not supported on this architecture");
    return TEST_SKIP;
    }
    pthread_barrier_init(&barrier, core::ptr::null_mut(), NUM_THREADS + 1);
    action.sa_flags = SA_SIGINFO | SA_NODEFER;
    action.sa_sigaction = sigtrap_handler;
    sigemptyset(&action.sa_mask);
    if (sigaction(SIGTRAP, &action, &oldact)) {
    pr_debug("FAILED sigaction(): %s\n", str_error_r(errno, sbuf, sizeof(sbuf)));
    goto out;
    }
    fd = sys_perf_event_open(&attr, 0, -1, -1, perf_event_open_cloexec_flag());
    if (fd < 0) {
    if (attr_has_sigtrap()) {
    pr_debug("FAILED sys_perf_event_open(): %s\n",
    str_error_r(errno, sbuf, sizeof(sbuf)));
    } else {
    pr_debug("perf_event_attr doesn't have sigtrap\n");
    ret = TEST_SKIP;
    }
    goto out_restore_sigaction;
    }
    for (i = 0; i < NUM_THREADS; i++) {
    if (pthread_create(&threads[i], core::ptr::null_mut(), test_thread, &barrier)) {
    pr_debug("FAILED pthread_create(): %s\n", str_error_r(errno, sbuf, sizeof(sbuf)));
    goto out_close_perf_event;
    }
    }
    ret = run_stress_test(fd, threads, &barrier);
    out_close_perf_event:
    close(fd);
    out_restore_sigaction:
    sigaction(SIGTRAP, &oldact, core::ptr::null_mut());
    out:
    pthread_barrier_destroy(&barrier);
    btf__exit();
    return ret;
    }
    DEFINE_SUITE("Sigtrap", sigtrap);
