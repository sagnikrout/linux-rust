//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/radeon/uvd_v2_2.c
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
// Copyright 2013 Advanced Micro Devices, Inc.
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
// Authors: Christian König <christian.koenig@amd.com>
//

//
// uvd_v2_2_fence_emit - emit an fence & trap command
//
// @rdev: radeon_device pointer
// @fence: fence to emit
//
// Write a fence and a trap command to the ring.
//
    void uvd_v2_2_fence_emit(struct radeon_device *rdev,
    struct radeon_fence *fence)
    {
    struct radeon_ring *ring = &rdev.ring[fence.ring];
    let mut addr: u64 = rdev.fence_drv[fence.ring].gpu_addr;
    radeon_ring_write(ring, PACKET0(UVD_CONTEXT_ID, 0));
    radeon_ring_write(ring, fence.seq);
    radeon_ring_write(ring, PACKET0(UVD_GPCOM_VCPU_DATA0, 0));
    radeon_ring_write(ring, lower_32_bits(addr));
    radeon_ring_write(ring, PACKET0(UVD_GPCOM_VCPU_DATA1, 0));
    radeon_ring_write(ring, upper_32_bits(addr) & 0xff);
    radeon_ring_write(ring, PACKET0(UVD_GPCOM_VCPU_CMD, 0));
    radeon_ring_write(ring, 0);
    radeon_ring_write(ring, PACKET0(UVD_GPCOM_VCPU_DATA0, 0));
    radeon_ring_write(ring, 0);
    radeon_ring_write(ring, PACKET0(UVD_GPCOM_VCPU_DATA1, 0));
    radeon_ring_write(ring, 0);
    radeon_ring_write(ring, PACKET0(UVD_GPCOM_VCPU_CMD, 0));
    radeon_ring_write(ring, 2);
    }
//
// uvd_v2_2_semaphore_emit - emit semaphore command
//
// @rdev: radeon_device pointer
// @ring: radeon_ring pointer
// @semaphore: semaphore to emit commands for
// @emit_wait: true if we should emit a wait command
//
// Emit a semaphore command (either wait or signal) to the UVD ring.
//
    bool uvd_v2_2_semaphore_emit(struct radeon_device *rdev,
    struct radeon_ring *ring,
    struct radeon_semaphore *semaphore,
    bool emit_wait)
    {
    let mut addr: u64 = semaphore.gpu_addr;
    radeon_ring_write(ring, PACKET0(UVD_SEMA_ADDR_LOW, 0));
    radeon_ring_write(ring, (addr >> 3) & 0x000FFFFF);
    radeon_ring_write(ring, PACKET0(UVD_SEMA_ADDR_HIGH, 0));
    radeon_ring_write(ring, (addr >> 23) & 0x000FFFFF);
    radeon_ring_write(ring, PACKET0(UVD_SEMA_CMD, 0));
    radeon_ring_write(ring, emit_wait ? 1 : 0);
    return true;
    }
//
// uvd_v2_2_resume - memory controller programming
//
// @rdev: radeon_device pointer
//
// Let the UVD memory controller know it's offsets
//
#[no_mangle]
pub unsafe extern "C" fn uvd_v2_2_resume(rdev: *mut radeon_device) -> c_int {
    int uvd_v2_2_resume(struct radeon_device *rdev)
    {
    uint64_t addr;
    uint32_t chip_id, size;
    int r;
// RV770 uses V1.0 MC
    if (rdev.family == CHIP_RV770)
    return uvd_v1_0_resume(rdev);
    r = radeon_uvd_resume(rdev);
    if (r)
    return r;
// program the VCPU memory controller bits 0-27
    addr = rdev.uvd.gpu_addr >> 3;
    size = RADEON_GPU_PAGE_ALIGN(rdev.uvd_fw.size + 4) >> 3;
    WREG32(UVD_VCPU_CACHE_OFFSET0, addr);
    WREG32(UVD_VCPU_CACHE_SIZE0, size);
    addr += size;
    size = RADEON_UVD_HEAP_SIZE >> 3;
    WREG32(UVD_VCPU_CACHE_OFFSET1, addr);
    WREG32(UVD_VCPU_CACHE_SIZE1, size);
    addr += size;
    size = (RADEON_UVD_STACK_SIZE +
    (RADEON_UVD_SESSION_SIZE * rdev.uvd.max_handles)) >> 3;
    WREG32(UVD_VCPU_CACHE_OFFSET2, addr);
    WREG32(UVD_VCPU_CACHE_SIZE2, size);
// bits 28-31
    addr = (rdev.uvd.gpu_addr >> 28) & 0xF;
    WREG32(UVD_LMI_ADDR_EXT, (addr << 12) | (addr << 0));
// bits 32-39
    addr = (rdev.uvd.gpu_addr >> 32) & 0xFF;
    WREG32(UVD_LMI_EXT40_ADDR, addr | (0x9 << 16) | (0x1 << 31));
// tell firmware which hardware it is running on
    switch (rdev.family) {
    default:
    return -EINVAL;
    case CHIP_RV710:
    chip_id = 0x01000005;
    break;
    case CHIP_RV730:
    chip_id = 0x01000006;
    break;
    case CHIP_RV740:
    chip_id = 0x01000007;
    break;
    case CHIP_CYPRESS:
    case CHIP_HEMLOCK:
    chip_id = 0x01000008;
    break;
    case CHIP_JUNIPER:
    chip_id = 0x01000009;
    break;
    case CHIP_REDWOOD:
    chip_id = 0x0100000a;
    break;
    case CHIP_CEDAR:
    chip_id = 0x0100000b;
    break;
    case CHIP_SUMO:
    case CHIP_SUMO2:
    chip_id = 0x0100000c;
    break;
    case CHIP_PALM:
    chip_id = 0x0100000e;
    break;
    case CHIP_CAYMAN:
    chip_id = 0x0100000f;
    break;
    case CHIP_BARTS:
    chip_id = 0x01000010;
    break;
    case CHIP_TURKS:
    chip_id = 0x01000011;
    break;
    case CHIP_CAICOS:
    chip_id = 0x01000012;
    break;
    case CHIP_TAHITI:
    chip_id = 0x01000014;
    break;
    case CHIP_VERDE:
    chip_id = 0x01000015;
    break;
    case CHIP_PITCAIRN:
    case CHIP_OLAND:
    chip_id = 0x01000016;
    break;
    case CHIP_ARUBA:
    chip_id = 0x01000017;
    break;
    }
    WREG32(UVD_VCPU_CHIP_ID, chip_id);
    return 0;
    }
