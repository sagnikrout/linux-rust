//! Automatically rewritten from C to Rust
//! Source: tools/lib/perf/threadmap.c
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

#[no_mangle]
unsafe extern "C" fn perf_thread_map__reset(map: *mut perf_thread_map, start: c_int, nr: c_int) {
    static void perf_thread_map__reset(struct perf_thread_map *map, int start, int nr)
    {
    let mut size: usize = (nr - start) * sizeof(map.map[0]);
    memset(&map.map[start], 0, size);
    map.err_thread = -1;
    }
    struct perf_thread_map *perf_thread_map__realloc(struct perf_thread_map *map, int nr)
    {
    let mut size: usize = sizeof(*map) + sizeof(map.map[0]) * nr;
    let mut start: c_int = map ? map.nr : 0;
    map = realloc(map, size);
//
// We only realloc to add more items, let's reset new items.
//
    if (map)
    perf_thread_map__reset(map, start, nr);
    return map;
    }

#[no_mangle]
pub unsafe extern "C" fn perf_thread_map__set_pid(map: *mut perf_thread_map, idx: c_int, pid: pid_t) {
    void perf_thread_map__set_pid(struct perf_thread_map *map, int idx, pid_t pid)
    {
    map.map[idx].pid = pid;
    }
    char *perf_thread_map__comm(struct perf_thread_map *map, int idx)
    {
    return map.map[idx].comm;
    }
    struct perf_thread_map *perf_thread_map__new_array(int nr_threads, pid_t *array)
    {
    struct perf_thread_map *threads = thread_map__alloc(nr_threads);
    int i;
    if (!threads)
    return core::ptr::null_mut();
    for (i = 0; i < nr_threads; i++)
    perf_thread_map__set_pid(threads, i, array ? array[i] : -1);
    threads.nr = nr_threads;
    refcount_set(&threads.refcnt, 1);
    return threads;
    }
    struct perf_thread_map *perf_thread_map__new_dummy(void)
    {
    return perf_thread_map__new_array(1, core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn perf_thread_map__delete(threads: *mut perf_thread_map) {
    static void perf_thread_map__delete(struct perf_thread_map *threads)
    {
    if (threads) {
    int i;
    WARN_ONCE(refcount_read(&threads.refcnt) != 0,
    "thread map refcnt unbalanced\n");
    for (i = 0; i < threads.nr; i++)
    free(perf_thread_map__comm(threads, i));
    free(threads);
    }
    }
    struct perf_thread_map *perf_thread_map__get(struct perf_thread_map *map)
    {
    if (map)
    refcount_inc(&map.refcnt);
    return map;
    }
#[no_mangle]
pub unsafe extern "C" fn perf_thread_map__put(map: *mut perf_thread_map) {
    void perf_thread_map__put(struct perf_thread_map *map)
    {
    if (map && refcount_dec_and_test(&map.refcnt))
    perf_thread_map__delete(map);
    }
#[no_mangle]
pub unsafe extern "C" fn perf_thread_map__nr(threads: *mut perf_thread_map) -> c_int {
    int perf_thread_map__nr(struct perf_thread_map *threads)
    {
    return threads ? threads.nr : 1;
    }
#[no_mangle]
pub unsafe extern "C" fn perf_thread_map__pid(map: *mut perf_thread_map, idx: c_int) -> pid_t {
    pid_t perf_thread_map__pid(struct perf_thread_map *map, int idx)
    {
    if (!map) {
    assert(idx == 0);
    return -1;
    }
    return map.map[idx].pid;
    }
#[no_mangle]
pub unsafe extern "C" fn perf_thread_map__idx(threads: *mut perf_thread_map, pid: pid_t) -> c_int {
    int perf_thread_map__idx(struct perf_thread_map *threads, pid_t pid)
    {
    if (!threads)
    let mut pid: return = = -1 ? 0 : -1;
    for (int i = 0; i < threads.nr; ++i) {
    if (threads.map[i].pid == pid)
    return i;
    }
    return -1;
    }
