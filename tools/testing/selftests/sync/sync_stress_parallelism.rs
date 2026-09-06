//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/sync/sync_stress_parallelism.c
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


//
// sync stress test: parallelism
// Copyright 2015-2016 Collabora Ltd.
//
// Based on the implementation from the Android Open Source Project,
//
// Copyright 2012 Google, Inc
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
// THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
//

    static struct {
    int iterations;
    int timeline;
    int counter;
    } test_data_two_threads;
#[no_mangle]
unsafe extern "C" fn test_stress_two_threads_shared_timeline_thread(d: *mut c_void) -> c_int {
    static int test_stress_two_threads_shared_timeline_thread(void *d)
    {
    let mut thread_id: c_int = (long)d;
    let mut timeline: c_int = test_data_two_threads.timeline;
    let mut iterations: c_int = test_data_two_threads.iterations;
    int fence, valid, ret, i;
    for (i = 0; i < iterations; i++) {
    fence = sw_sync_fence_create(timeline, "fence",
    i * 2 + thread_id);
    valid = sw_sync_fence_is_valid(fence);
    ASSERT(valid, "Failure allocating fence\n");
// Wait on the prior thread to complete
    ret = sync_wait(fence, -1);
    ASSERT(ret > 0, "Problem occurred on prior thread\n");
//
// Confirm the previous thread's writes are visible
// and then increment
//
    ASSERT(test_data_two_threads.counter == i * 2 + thread_id,
    "Counter got damaged!\n");
    test_data_two_threads.counter++;
// Kick off the other thread
    ret = sw_sync_timeline_inc(timeline, 1);
    ASSERT(ret == 0, "Advancing timeline failed\n");
    sw_sync_fence_destroy(fence);
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn test_stress_two_threads_shared_timeline() -> c_int {
    int test_stress_two_threads_shared_timeline(void)
    {
    pthread_t a, b;
    int valid;
    let mut timeline: c_int = sw_sync_timeline_create();
    valid = sw_sync_timeline_is_valid(timeline);
    ASSERT(valid, "Failure allocating timeline\n");
    test_data_two_threads.iterations = 1 << 16;
    test_data_two_threads.counter = 0;
    test_data_two_threads.timeline = timeline;
//
// Use a single timeline to synchronize two threads
// hammmering on the same counter.
//
    pthread_create(&a, core::ptr::null_mut(), (void *(*)(void *))
    test_stress_two_threads_shared_timeline_thread,
    (void *)0);
    pthread_create(&b, core::ptr::null_mut(), (void *(*)(void *))
    test_stress_two_threads_shared_timeline_thread,
    (void *)1);
    pthread_join(a, core::ptr::null_mut());
    pthread_join(b, core::ptr::null_mut());
// make sure the threads did not trample on one another
    ASSERT(test_data_two_threads.counter ==
    test_data_two_threads.iterations * 2,
    "Counter has unexpected value\n");
    sw_sync_timeline_destroy(timeline);
    return 0;
    }
