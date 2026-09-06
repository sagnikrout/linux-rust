//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/sync/sync_stress_merge.c
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
// sync stress test: merging
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

#[no_mangle]
pub unsafe extern "C" fn test_merge_stress_random_merge() -> c_int {
    int test_merge_stress_random_merge(void)
    {
    int i, size, ret;
    let mut timeline_count: c_int = 32;
    let mut merge_count: c_int = 1024 * 32;
    int timelines[timeline_count];
    int fence_map[timeline_count];
    int fence, tmpfence, merged, valid;
    int timeline, timeline_offset, sync_point;
    srand(time(core::ptr::null_mut()));
    for (i = 0; i < timeline_count; i++)
    timelines[i] = sw_sync_timeline_create();
    fence = sw_sync_fence_create(timelines[0], "fence", 0);
    valid = sw_sync_fence_is_valid(fence);
    ASSERT(valid, "Failure creating fence\n");
    memset(fence_map, -1, sizeof(fence_map));
    fence_map[0] = 0;
//
// Randomly create sync_points out of a fixed set of timelines,
// and merge them together
//
    for (i = 0; i < merge_count; i++) {
// Generate sync_point.
    timeline_offset = rand() % timeline_count;
    timeline = timelines[timeline_offset];
    sync_point = rand();
// Keep track of the latest sync_point in each timeline.
    if (fence_map[timeline_offset] == -1)
    fence_map[timeline_offset] = sync_point;
#[no_mangle]
pub unsafe extern "C" fn if(sync_point: fence_map[timeline_offset] <) -> else {
    else if (fence_map[timeline_offset] < sync_point)
    fence_map[timeline_offset] = sync_point;
// Merge
    tmpfence = sw_sync_fence_create(timeline, "fence", sync_point);
    merged = sync_merge("merge", tmpfence, fence);
    sw_sync_fence_destroy(tmpfence);
    sw_sync_fence_destroy(fence);
    fence = merged;
    valid = sw_sync_fence_is_valid(merged);
    ASSERT(valid, "Failure creating fence i\n");
    }
    size = 0;
    for (i = 0; i < timeline_count; i++)
    if (fence_map[i] != -1)
    size++;
// Confirm our map matches the fence.
    ASSERT(sync_fence_size(fence) == size,
    "Quantity of elements not matching\n");
// Trigger the merged fence
    for (i = 0; i < timeline_count; i++) {
    if (fence_map[i] != -1) {
    ret = sync_wait(fence, 0);
    ASSERT(ret == 0,
    "Failure waiting on fence until timeout\n");
// Increment the timeline to the last sync_point
    sw_sync_timeline_inc(timelines[i], fence_map[i]);
    }
    }
// Check that the fence is triggered.
    ret = sync_wait(fence, 0);
    ASSERT(ret > 0, "Failure triggering fence\n");
    sw_sync_fence_destroy(fence);
    for (i = 0; i < timeline_count; i++)
    sw_sync_timeline_destroy(timelines[i]);
    return 0;
    }
