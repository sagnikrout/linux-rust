//! Automatically rewritten from C to Rust
//! Source: tools/perf/tests/mmap-thread-lookup.c
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

pub const THREADS: c_int = 4;
    static int go_away;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct thread_data {
    pub pt: pthread_t,
    pub tid: pid_t,
    pub map: *mut c_void,
    pub ready: [c_int; 2],
}

    static struct thread_data threads[THREADS];
#[no_mangle]
unsafe extern "C" fn thread_init(td: *mut thread_data) -> c_int {
    static int thread_init(struct thread_data *td)
    {
    void *map;
    map = mmap(core::ptr::null_mut(), page_size,
    PROT_READ|PROT_WRITE|PROT_EXEC,
    MAP_SHARED|MAP_ANONYMOUS, -1, 0);
    if (map == MAP_FAILED) {
    perror("mmap failed");
    return -1;
    }
    td.map = map;
    td.tid = syscall(SYS_gettid);
    pr_debug("tid = %d, map = %p\n", td.tid, map);
    return 0;
    }
    static void *thread_fn(void *arg)
    {
    struct thread_data *td = arg;
    ssize_t ret;
    let mut go: c_int = 0;
    if (thread_init(td))
    return core::ptr::null_mut();
// Signal thread_create thread is initialized.
    ret = write(td.ready[1], &go, sizeof(int));
    if (ret != sizeof(int)) {
    pr_err("failed to notify\n");
    return core::ptr::null_mut();
    }
    while (!go_away) {
// Waiting for main thread to kill us.
    usleep(100);
    }
    munmap(td.map, page_size);
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn thread_create(i: c_int) -> c_int {
    static int thread_create(int i)
    {
    struct thread_data *td = &threads[i];
    int err, go;
    if (pipe(td.ready))
    return -1;
    err = pthread_create(&td.pt, core::ptr::null_mut(), thread_fn, td);
    if (!err) {
// Wait for thread initialization.
    let mut ret: isize = read(td.ready[0], &go, sizeof(int));
    err = ret != sizeof(int);
    }
    close(td.ready[0]);
    close(td.ready[1]);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn threads_create() -> c_int {
    static int threads_create(void)
    {
    struct thread_data *td0 = &threads[0];
    int i, err = 0;
    go_away = 0;
// 0 is main thread
    if (thread_init(td0))
    return -1;
    for (i = 1; !err && i < THREADS; i++)
    err = thread_create(i);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn threads_destroy() -> c_int {
    static int threads_destroy(void)
    {
    struct thread_data *td0 = &threads[0];
    int i, err = 0;
// cleanup the main thread
    munmap(td0.map, page_size);
    go_away = 1;
    for (i = 1; !err && i < THREADS; i++)
    err = pthread_join(threads[i].pt, core::ptr::null_mut());
    return err;
    }
    typedef int (*synth_cb)(struct machine *machine);
#[no_mangle]
unsafe extern "C" fn synth_all(machine: *mut machine) -> c_int {
    static int synth_all(struct machine *machine)
    {
    return perf_event__synthesize_threads(core::ptr::null_mut(),
    perf_event__process,
    machine, 1, 0, 1);
    }
#[no_mangle]
unsafe extern "C" fn synth_process(machine: *mut machine) -> c_int {
    static int synth_process(struct machine *machine)
    {
    struct perf_thread_map *map;
    int err;
    map = thread_map__new_by_pid(getpid());
    err = perf_event__synthesize_thread_map(core::ptr::null_mut(), map,
    perf_event__process,
    machine, 1, 0);
    perf_thread_map__put(map);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn mmap_events(synth: synth_cb) -> c_int {
    static int mmap_events(synth_cb synth)
    {
    struct perf_env host_env;
    struct machine *machine;
    int err, i;
//
// The threads_create will not return before all threads
// are spawned and all created memory map.
//
// They will loop until threads_destroy is called, so we
// can safely run synthesizing function.
//
    TEST_ASSERT_VAL("failed to create threads", !threads_create());
    perf_env__init(&host_env);
    machine = machine__new_host(&host_env);
    dump_trace = verbose > 1 ? 1 : 0;
    err = synth(machine);
    dump_trace = 0;
    TEST_ASSERT_VAL("failed to destroy threads", !threads_destroy());
    TEST_ASSERT_VAL("failed to synthesize maps", !err);
//
// All data is synthesized, try to find map for each
// thread object.
//
    for (i = 0; i < THREADS; i++) {
    struct thread_data *td = &threads[i];
    struct addr_location al;
    struct thread *thread;
    addr_location__init(&al);
    thread = machine__findnew_thread(machine, getpid(), td.tid);
    pr_debug("looking for map %p\n", td.map);
    thread__find_map(thread, PERF_RECORD_MISC_USER,
    (unsigned long) (td.map + 1), &al);
    thread__put(thread);
    if (!al.map) {
    pr_debug("failed, couldn't find map\n");
    err = -1;
    addr_location__exit(&al);
    break;
    }
    pr_debug("map %p, addr %" PRIx64 "\n", al.map, map__start(al.map));
    addr_location__exit(&al);
    }
    machine__delete(machine);
    perf_env__exit(&host_env);
    return err;
    }
//
// This test creates 'THREADS' number of threads (including
// main thread) and each thread creates memory map.
//
// When threads are created, we synthesize them with both
// (separate tests):
// perf_event__synthesize_thread_map (process based)
// perf_event__synthesize_threads    (global)
//
// We test we can find all memory maps via:
// thread__find_map
//
// by using all thread objects.
//
#[no_mangle]
unsafe extern "C" fn test__mmap_thread_lookup(__maybe_unused: *mut *mut test_suite test, __maybe_unused: int subtest) -> c_int {
    static int test__mmap_thread_lookup(struct test_suite *test __maybe_unused, int subtest __maybe_unused)
    {
// perf_event__synthesize_threads synthesize
    TEST_ASSERT_VAL("failed with synthesizing all",
    !mmap_events(synth_all));
// perf_event__synthesize_thread_map synthesize
    TEST_ASSERT_VAL("failed with synthesizing process",
    !mmap_events(synth_process));
    return 0;
    }
    DEFINE_SUITE("Lookup mmap thread", mmap_thread_lookup);
