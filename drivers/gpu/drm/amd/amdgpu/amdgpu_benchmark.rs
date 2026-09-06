//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/amd/amdgpu/amdgpu_benchmark.c
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
// Copyright 2009 Jerome Glisse.
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
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
//
// Authors: Jerome Glisse
//

pub const AMDGPU_BENCHMARK_ITERATIONS: c_int = 1024;
pub const AMDGPU_BENCHMARK_COMMON_MODES_N: c_int = 17;
    static int amdgpu_benchmark_do_move(struct amdgpu_device *adev, unsigned size,
    uint64_t saddr, uint64_t daddr, int n, s64 *time_ms)
    {
    ktime_t stime, etime;
    struct dma_fence *fence;
    int i, r;
    mutex_lock(&adev.mman.default_entity.lock);
    stime = ktime_get();
    for (i = 0; i < n; i++) {
    r = amdgpu_copy_buffer(adev, &adev.mman.default_entity,
    saddr, daddr, size, core::ptr::null_mut(), &fence,
    false, 0);
    if (r)
    goto exit_do_move;
    r = dma_fence_wait(fence, false);
    dma_fence_put(fence);
    if (r)
    goto exit_do_move;
    }
    exit_do_move:
    mutex_unlock(&adev.mman.default_entity.lock);
    etime = ktime_get();
// time_ms = ktime_ms_delta(etime, stime);
    return r;
    }
    static void amdgpu_benchmark_log_results(struct amdgpu_device *adev,
    int n, unsigned size,
    s64 time_ms,
    unsigned sdomain, unsigned ddomain,
    char *kind)
    {
    let mut throughput: i64 = (n * (size >> 10));
    throughput = div64_s64(throughput, time_ms);
    dev_info(adev.dev, " %s %u bo moves of %u kB from"
    " %d to %d in %lld ms, throughput: %lld Mb/s or %lld MB/s\n",
    kind, n, size >> 10, sdomain, ddomain, time_ms,
    throughput * 8, throughput);
    }
    static int amdgpu_benchmark_move(struct amdgpu_device *adev, unsigned size,
    unsigned sdomain, unsigned ddomain)
    {
    struct amdgpu_bo *dobj = core::ptr::null_mut();
    struct amdgpu_bo *sobj = core::ptr::null_mut();
    uint64_t saddr, daddr;
    s64 time_ms;
    int r, n;
    n = AMDGPU_BENCHMARK_ITERATIONS;
    r = amdgpu_bo_create_kernel(adev, size,
    PAGE_SIZE, sdomain,
    &sobj,
    &saddr,
    core::ptr::null_mut());
    if (r)
    goto out_cleanup;
    r = amdgpu_bo_create_kernel(adev, size,
    PAGE_SIZE, ddomain,
    &dobj,
    &daddr,
    core::ptr::null_mut());
    if (r)
    goto out_cleanup;
    if (adev.mman.buffer_funcs) {
    r = amdgpu_benchmark_do_move(adev, size, saddr, daddr, n, &time_ms);
    if (r)
    goto out_cleanup;
    else
    amdgpu_benchmark_log_results(adev, n, size, time_ms,
    sdomain, ddomain, "dma");
    }
    out_cleanup:
// Check error value now. The value can be overwritten when clean up.
    if (r < 0)
    dev_info(adev.dev, "Error while benchmarking BO move.\n");
    if (sobj)
    amdgpu_bo_free_kernel(&sobj, &saddr, core::ptr::null_mut());
    if (dobj)
    amdgpu_bo_free_kernel(&dobj, &daddr, core::ptr::null_mut());
    return r;
    }
#[no_mangle]
pub unsafe extern "C" fn amdgpu_benchmark(adev: *mut amdgpu_device, test_number: c_int) -> c_int {
    int amdgpu_benchmark(struct amdgpu_device *adev, int test_number)
    {
    int i, r;
    static const int common_modes[AMDGPU_BENCHMARK_COMMON_MODES_N] = {
    640 * 480 * 4,
    720 * 480 * 4,
    800 * 600 * 4,
    848 * 480 * 4,
    1024 * 768 * 4,
    1152 * 768 * 4,
    1280 * 720 * 4,
    1280 * 800 * 4,
    1280 * 854 * 4,
    1280 * 960 * 4,
    1280 * 1024 * 4,
    1440 * 900 * 4,
    1400 * 1050 * 4,
    1680 * 1050 * 4,
    1600 * 1200 * 4,
    1920 * 1080 * 4,
    1920 * 1200 * 4
    };
    mutex_lock(&adev.benchmark_mutex);
    switch (test_number) {
    case 1:
    dev_info(adev.dev,
    "benchmark test: %d (simple test, VRAM to GTT and GTT to VRAM)\n",
    test_number);
// simple test, VRAM to GTT and GTT to VRAM
    r = amdgpu_benchmark_move(adev, 1024*1024, AMDGPU_GEM_DOMAIN_GTT,
    AMDGPU_GEM_DOMAIN_VRAM);
    if (r)
    goto done;
    r = amdgpu_benchmark_move(adev, 1024*1024, AMDGPU_GEM_DOMAIN_VRAM,
    AMDGPU_GEM_DOMAIN_GTT);
    if (r)
    goto done;
    break;
    case 2:
    dev_info(adev.dev,
    "benchmark test: %d (simple test, VRAM to VRAM)\n",
    test_number);
// simple test, VRAM to VRAM
    r = amdgpu_benchmark_move(adev, 1024*1024, AMDGPU_GEM_DOMAIN_VRAM,
    AMDGPU_GEM_DOMAIN_VRAM);
    if (r)
    goto done;
    break;
    case 3:
    dev_info(adev.dev,
    "benchmark test: %d (GTT to VRAM, buffer size sweep, powers of 2)\n",
    test_number);
// GTT to VRAM, buffer size sweep, powers of 2
    for (i = 1; i <= 16384; i <<= 1) {
    r = amdgpu_benchmark_move(adev, i * AMDGPU_GPU_PAGE_SIZE,
    AMDGPU_GEM_DOMAIN_GTT,
    AMDGPU_GEM_DOMAIN_VRAM);
    if (r)
    goto done;
    }
    break;
    case 4:
    dev_info(adev.dev,
    "benchmark test: %d (VRAM to GTT, buffer size sweep, powers of 2)\n",
    test_number);
// VRAM to GTT, buffer size sweep, powers of 2
    for (i = 1; i <= 16384; i <<= 1) {
    r = amdgpu_benchmark_move(adev, i * AMDGPU_GPU_PAGE_SIZE,
    AMDGPU_GEM_DOMAIN_VRAM,
    AMDGPU_GEM_DOMAIN_GTT);
    if (r)
    goto done;
    }
    break;
    case 5:
    dev_info(adev.dev,
    "benchmark test: %d (VRAM to VRAM, buffer size sweep, powers of 2)\n",
    test_number);
// VRAM to VRAM, buffer size sweep, powers of 2
    for (i = 1; i <= 16384; i <<= 1) {
    r = amdgpu_benchmark_move(adev, i * AMDGPU_GPU_PAGE_SIZE,
    AMDGPU_GEM_DOMAIN_VRAM,
    AMDGPU_GEM_DOMAIN_VRAM);
    if (r)
    goto done;
    }
    break;
    case 6:
    dev_info(adev.dev,
    "benchmark test: %d (GTT to VRAM, buffer size sweep, common modes)\n",
    test_number);
// GTT to VRAM, buffer size sweep, common modes
    for (i = 0; i < AMDGPU_BENCHMARK_COMMON_MODES_N; i++) {
    r = amdgpu_benchmark_move(adev, common_modes[i],
    AMDGPU_GEM_DOMAIN_GTT,
    AMDGPU_GEM_DOMAIN_VRAM);
    if (r)
    goto done;
    }
    break;
    case 7:
    dev_info(adev.dev,
    "benchmark test: %d (VRAM to GTT, buffer size sweep, common modes)\n",
    test_number);
// VRAM to GTT, buffer size sweep, common modes
    for (i = 0; i < AMDGPU_BENCHMARK_COMMON_MODES_N; i++) {
    r = amdgpu_benchmark_move(adev, common_modes[i],
    AMDGPU_GEM_DOMAIN_VRAM,
    AMDGPU_GEM_DOMAIN_GTT);
    if (r)
    goto done;
    }
    break;
    case 8:
    dev_info(adev.dev,
    "benchmark test: %d (VRAM to VRAM, buffer size sweep, common modes)\n",
    test_number);
// VRAM to VRAM, buffer size sweep, common modes
    for (i = 0; i < AMDGPU_BENCHMARK_COMMON_MODES_N; i++) {
    r = amdgpu_benchmark_move(adev, common_modes[i],
    AMDGPU_GEM_DOMAIN_VRAM,
    AMDGPU_GEM_DOMAIN_VRAM);
    if (r)
    goto done;
    }
    break;
    default:
    dev_info(adev.dev, "Unknown benchmark %d\n", test_number);
    r = -EINVAL;
    break;
    }
    done:
    mutex_unlock(&adev.benchmark_mutex);
    return r;
    }
