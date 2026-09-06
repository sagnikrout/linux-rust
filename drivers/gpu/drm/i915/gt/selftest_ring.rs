//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/i915/gt/selftest_ring.c
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
// Copyright © 2020 Intel Corporation
//
    static struct intel_ring *mock_ring(unsigned long sz)
    {
    struct intel_ring *ring;
    ring = kzalloc(sizeof(*ring) + sz, GFP_KERNEL);
    if (!ring)
    return core::ptr::null_mut();
    kref_init(&ring.ref);
    ring.size = sz;
    ring.wrap = BITS_PER_TYPE(ring.size) - ilog2(sz);
    ring.effective_size = sz;
    ring.vaddr = (void *)(ring + 1);
    atomic_set(&ring.pin_count, 1);
    intel_ring_update_space(ring);
    return ring;
    }
#[no_mangle]
unsafe extern "C" fn mock_ring_free(ring: *mut intel_ring) {
    static void mock_ring_free(struct intel_ring *ring)
    {
    kfree(ring);
    }
    static int check_ring_direction(struct intel_ring *ring,
    u32 next, u32 prev,
    int expected)
    {
    int result;
    result = intel_ring_direction(ring, next, prev);
    if (result < 0)
    result = -1;
#[no_mangle]
pub unsafe extern "C" fn if(0: result >) -> else {
    else if (result > 0)
    result = 1;
    if (result != expected) {
    pr_err("intel_ring_direction(%u, %u):%d != %d\n",
    next, prev, result, expected);
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn check_ring_step(ring: *mut intel_ring, x: u32, step: u32) -> c_int {
    static int check_ring_step(struct intel_ring *ring, u32 x, u32 step)
    {
    let mut prev: u32 = x, next = intel_ring_wrap(ring, x + step);
    let mut err: c_int = 0;
    err |= check_ring_direction(ring, next, next,  0);
    err |= check_ring_direction(ring, prev, prev,  0);
    err |= check_ring_direction(ring, next, prev,  1);
    err |= check_ring_direction(ring, prev, next, -1);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn check_ring_offset(ring: *mut intel_ring, x: u32, step: u32) -> c_int {
    static int check_ring_offset(struct intel_ring *ring, u32 x, u32 step)
    {
    let mut err: c_int = 0;
    err |= check_ring_step(ring, x, step);
    err |= check_ring_step(ring, intel_ring_wrap(ring, x + 1), step);
    err |= check_ring_step(ring, intel_ring_wrap(ring, x - 1), step);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn igt_ring_direction(dummy: *mut c_void) -> c_int {
    static int igt_ring_direction(void *dummy)
    {
    struct intel_ring *ring;
    let mut half: c_uint = 2048;
    int step, err = 0;
    ring = mock_ring(2 * half);
    if (!ring)
    return -ENOMEM;
    GEM_BUG_ON(ring.size != 2 * half);
// Precision of wrap detection is limited to ring->size / 2
    for (step = 1; step < half; step <<= 1) {
    err |= check_ring_offset(ring, 0, step);
    err |= check_ring_offset(ring, half, step);
    }
    err |= check_ring_step(ring, 0, half - 64);
// And check unwrapped handling for good measure
    err |= check_ring_offset(ring, 0, 2 * half + 64);
    err |= check_ring_offset(ring, 3 * half, 1);
    mock_ring_free(ring);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn intel_ring_mock_selftests() -> c_int {
    int intel_ring_mock_selftests(void)
    {
    static const struct i915_subtest tests[] = {
    SUBTEST(igt_ring_direction),
    };
    return i915_subtests(tests, core::ptr::null_mut());
    }
