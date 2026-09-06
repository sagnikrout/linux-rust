//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/sync/sync_alloc.c
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
// sync allocation tests
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
pub unsafe extern "C" fn test_alloc_timeline() -> c_int {
    int test_alloc_timeline(void)
    {
    int timeline, valid;
    timeline = sw_sync_timeline_create();
    valid = sw_sync_timeline_is_valid(timeline);
    ASSERT(valid, "Failure allocating timeline\n");
    sw_sync_timeline_destroy(timeline);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn test_alloc_fence() -> c_int {
    int test_alloc_fence(void)
    {
    int timeline, fence, valid;
    timeline = sw_sync_timeline_create();
    valid = sw_sync_timeline_is_valid(timeline);
    ASSERT(valid, "Failure allocating timeline\n");
    fence = sw_sync_fence_create(timeline, "allocFence", 1);
    valid = sw_sync_fence_is_valid(fence);
    ASSERT(valid, "Failure allocating fence\n");
    sw_sync_fence_destroy(fence);
    sw_sync_timeline_destroy(timeline);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn test_alloc_fence_negative() -> c_int {
    int test_alloc_fence_negative(void)
    {
    int fence, timeline;
    timeline = sw_sync_timeline_create();
    ASSERT(timeline > 0, "Failure allocating timeline\n");
    fence = sw_sync_fence_create(-1, "fence", 1);
    ASSERT(fence < 0, "Success allocating negative fence\n");
    sw_sync_fence_destroy(fence);
    sw_sync_timeline_destroy(timeline);
    return 0;
    }
