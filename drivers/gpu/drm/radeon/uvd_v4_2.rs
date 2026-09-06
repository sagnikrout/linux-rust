//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/radeon/uvd_v4_2.c
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
// uvd_v4_2_resume - memory controller programming
//
// @rdev: radeon_device pointer
//
// Let the UVD memory controller know it's offsets
//
#[no_mangle]
pub unsafe extern "C" fn uvd_v4_2_resume(rdev: *mut radeon_device) -> c_int {
    int uvd_v4_2_resume(struct radeon_device *rdev)
    {
    uint64_t addr;
    uint32_t size;
// program the VCPU memory controller bits 0-27
// skip over the header of the new firmware format
    if (rdev.uvd.fw_header_present)
    addr = (rdev.uvd.gpu_addr + 0x200) >> 3;
    else
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
    if (rdev.uvd.fw_header_present)
    WREG32(UVD_GP_SCRATCH4, rdev.uvd.max_handles);
    return 0;
    }
