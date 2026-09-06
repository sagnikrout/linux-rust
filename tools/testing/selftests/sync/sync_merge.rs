//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/sync/sync_merge.c
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
// sync fence merge tests
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
pub unsafe extern "C" fn test_fence_merge_same_fence() -> c_int {
    int test_fence_merge_same_fence(void)
    {
    int fence, valid, merged;
    let mut timeline: c_int = sw_sync_timeline_create();
    valid = sw_sync_timeline_is_valid(timeline);
    ASSERT(valid, "Failure allocating timeline\n");
    fence = sw_sync_fence_create(timeline, "allocFence", 5);
    valid = sw_sync_fence_is_valid(fence);
    ASSERT(valid, "Failure allocating fence\n");
    merged = sync_merge("mergeFence", fence, fence);
    valid = sw_sync_fence_is_valid(fence);
    ASSERT(valid, "Failure merging fence\n");
    ASSERT(sync_fence_count_with_status(merged, FENCE_STATUS_SIGNALED) == 0,
    "fence signaled too early!\n");
    sw_sync_timeline_inc(timeline, 5);
    ASSERT(sync_fence_count_with_status(merged, FENCE_STATUS_SIGNALED) == 1,
    "fence did not signal!\n");
    sw_sync_fence_destroy(merged);
    sw_sync_fence_destroy(fence);
    sw_sync_timeline_destroy(timeline);
    return 0;
    }
