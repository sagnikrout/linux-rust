//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/amd/amdgpu/amdgpu_vm_tlb_fence.c
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


// SPDX-License-Identifier: GPL-2.0 OR MIT
//
// Copyright 2023 Advanced Micro Devices, Inc.
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_tlb_fence {
    pub base: dma_fence,
    pub adev: *mut amdgpu_device,
    pub dependency: *mut dma_fence,
    pub work: work_struct,
    pub lock: spinlock_t,
    pub pasid: u16,
}

    static const char *amdgpu_tlb_fence_get_driver_name(struct dma_fence *fence)
    {
    return "amdgpu tlb fence";
    }
    static const char *amdgpu_tlb_fence_get_timeline_name(struct dma_fence *f)
    {
    return "amdgpu tlb timeline";
    }
#[no_mangle]
unsafe extern "C" fn amdgpu_tlb_fence_work(work: *mut work_struct) {
    static void amdgpu_tlb_fence_work(struct work_struct *work)
    {
    struct amdgpu_tlb_fence *f = container_of(work, typeof(*f), work);
    int r;
    if (f.dependency) {
    dma_fence_wait(f.dependency, false);
    dma_fence_put(f.dependency);
    f.dependency = core::ptr::null_mut();
    }
    r = amdgpu_gmc_flush_gpu_tlb_pasid(f.adev, f.pasid, 2, true, 0);
    if (r) {
    dev_err(f.adev.dev, "TLB flush failed for PASID %d.\n",
    f.pasid);
    dma_fence_set_error(&f.base, r);
    }
    dma_fence_signal(&f.base);
    dma_fence_put(&f.base);
    }
    static const struct dma_fence_ops amdgpu_tlb_fence_ops = {
    .get_driver_name = amdgpu_tlb_fence_get_driver_name,
    .get_timeline_name = amdgpu_tlb_fence_get_timeline_name
    };
    void amdgpu_vm_tlb_fence_create(struct amdgpu_device *adev, struct amdgpu_vm *vm,
    struct dma_fence **fence)
    {
    struct amdgpu_tlb_fence *f;
    f = kmalloc_obj(*f);
    if (!f) {
//
// We can't fail since the PDEs and PTEs are already updated, so
// just block for the dependency and execute the TLB flush
//
    if (*fence)
    dma_fence_wait(*fence, false);
    amdgpu_gmc_flush_gpu_tlb_pasid(adev, vm.pasid, 2, true, 0);
// fence = dma_fence_get_stub();
    return;
    }
    f.adev = adev;
    f.dependency = *fence;
    f.pasid = vm.pasid;
    INIT_WORK(&f.work, amdgpu_tlb_fence_work);
    spin_lock_init(&f.lock);
    dma_fence_init64(&f.base, &amdgpu_tlb_fence_ops, &f.lock,
    vm.tlb_fence_context, atomic64_read(&vm.tlb_seq));
// TODO: We probably need a separate wq here
    dma_fence_get(&f.base);
    schedule_work(&f.work);
// fence = &f->base;
    }
