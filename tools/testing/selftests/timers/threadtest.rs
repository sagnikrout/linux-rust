//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/timers/threadtest.c
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


// threadtest.c
// by: john stultz (johnstul@us.ibm.com)
// (C) Copyright IBM 2004, 2005, 2006, 2012
// Licensed under the GPLv2
//
// To build:
// $ gcc threadtest.c -o threadtest -lrt
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 2 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//

// serializes shared list access
    let mut list_lock: pthread_mutex_t = PTHREAD_MUTEX_INITIALIZER;
// serializes console output
    let mut print_lock: pthread_mutex_t = PTHREAD_MUTEX_INITIALIZER;
pub const MAX_THREADS: c_int = 128;
pub const LISTSIZE: c_int = 128;
    let mut done: c_int = 0;
    struct timespec global_list[LISTSIZE];
    let mut listcount: c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn checklist(list: *const timespec, size: c_int) {
    void checklist(const struct timespec *list, int size)
    {
    int i, j;
    const struct timespec *a, *b;
// scan the list
    for (i = 0; i < size-1; i++) {
    a = &list[i];
    b = &list[i+1];
// look for any time inconsistencies
    if ((b.tv_sec <= a.tv_sec) &&
    (b.tv_nsec < a.tv_nsec)) {
// flag other threads
    done = 1;
// serialize printing to avoid junky output
    pthread_mutex_lock(&print_lock);
// dump the list
    printf("\n");
    for (j = 0; j < size; j++) {
    if (j == i)
    printf("---------------\n");
    printf("%lu:%lu\n", list[j].tv_sec, list[j].tv_nsec);
    if (j == i+1)
    printf("---------------\n");
    }
    printf("[FAILED]\n");
    pthread_mutex_unlock(&print_lock);
    }
    }
    }
// The shared thread shares a global list
// that each thread fills while holding the lock.
// This stresses clock synchronization across cpus.
//
    void *shared_thread(void *arg)
    {
    while (!done) {
// protect the list
    pthread_mutex_lock(&list_lock);
// see if we're ready to check the list
    if (listcount >= LISTSIZE) {
    checklist(global_list, LISTSIZE);
    listcount = 0;
    }
    clock_gettime(CLOCK_MONOTONIC, &global_list[listcount++]);
    pthread_mutex_unlock(&list_lock);
    }
    return core::ptr::null_mut();
    }
// Each independent thread fills in its own
// list. This stresses clock_gettime() lock contention.
//
    void *independent_thread(void *arg)
    {
    struct timespec my_list[LISTSIZE];
    int count;
    while (!done) {
// fill the list
    for (count = 0; count < LISTSIZE; count++)
    clock_gettime(CLOCK_MONOTONIC, &my_list[count]);
    checklist(my_list, LISTSIZE);
    }
    return core::ptr::null_mut();
    }
pub const DEFAULT_THREAD_COUNT: c_int = 8;
pub const DEFAULT_RUNTIME: c_int = 30;
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    int thread_count, i;
    time_t start, now, runtime;
    char buf[255];
    pthread_t pth[MAX_THREADS];
    int opt;
    void *tret;
    let mut ret: c_int = 0;
    void *(*thread)(void *) = shared_thread;
    thread_count = DEFAULT_THREAD_COUNT;
    runtime = DEFAULT_RUNTIME;
// Process arguments
    while ((opt = getopt(argc, argv, "t:n:i")) != -1) {
    switch (opt) {
    case 't':
    runtime = atoi(optarg);
    break;
    case 'n':
    thread_count = atoi(optarg);
    break;
    case 'i':
    thread = independent_thread;
    printf("using independent threads\n");
    break;
    default:
    printf("Usage: %s [-t <secs>] [-n <numthreads>] [-i]\n", argv[0]);
    printf("	-t: time to run\n");
    printf("	-n: number of threads\n");
    printf("	-i: use independent threads\n");
    return -1;
    }
    }
    if (thread_count > MAX_THREADS)
    thread_count = MAX_THREADS;
    setbuf(stdout, core::ptr::null_mut());
    start = time(0);
    strftime(buf, 255, "%a, %d %b %Y %T %z", localtime(&start));
    printf("%s\n", buf);
    printf("Testing consistency with %i threads for %ld seconds: ", thread_count, runtime);
    fflush(stdout);
// spawn
    for (i = 0; i < thread_count; i++)
    pthread_create(&pth[i], 0, thread, 0);
    while (time(&now) < start + runtime) {
    sleep(1);
    if (done) {
    ret = 1;
    strftime(buf, 255, "%a, %d %b %Y %T %z", localtime(&now));
    printf("%s\n", buf);
    goto out;
    }
    }
    printf("[OK]\n");
    done = 1;
    out:
// wait
    for (i = 0; i < thread_count; i++)
    pthread_join(pth[i], &tret);
// die
    if (ret)
    ksft_exit_fail();
    ksft_exit_pass();
    }
