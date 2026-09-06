//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/amdgpu/amdgpu_job.h
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
// Copyright 2018 Advanced Micro Devices, Inc.
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

// bit set means command submit involves a preamble IB

// bit set means preamble IB is first presented in belonging context

// bit set means context switch occured

// bit set means IB is preempted

// Internal kernel job ids. (decreasing values, starting from U64_MAX).

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_job {
    pub base: drm_sched_job,
    pub vm: *mut amdgpu_vm,
    pub explicit_sync: amdgpu_sync,
    pub hw_fence: *mut amdgpu_fence,
    pub hw_vm_fence: *mut amdgpu_fence,
    pub gang_submit: *mut dma_fence,
    pub preamble_status: u32,
    pub preemption_status: u32,
    pub vm_needs_flush: bool,
    pub gds_switch_needed: bool,
    pub spm_update_needed: bool,
    pub vm_pd_addr: u64,
    pub vmid: unsigned,
    pub pasid: unsigned,
    pub gds_size: uint32_t gds_base,,
    pub gws_size: uint32_t gws_base,,
    pub oa_size: uint32_t oa_base,,
    pub generation: u64,
// user fence handling
    pub uf_addr: u64,
    pub uf_sequence: u64,
// virtual addresses for shadow/GDS/CSA
    pub shadow_va: u64,
    pub csa_va: u64,
    pub gds_va: u64,
    pub init_shadow: bool,
// job_run_counter >= 1 means a resubmit job
    pub job_run_counter: u32,
// enforce isolation
    pub enforce_isolation: bool,
    pub run_cleaner_shader: bool,
    pub num_ibs: u32,
    pub ibs: [amdgpu_ib; ],
}

extern "C" {
    pub fn to_amdgpu_ring(_arg: job->base.entity->rq->sched) -> return;
}
extern "C" {
    pub fn amdgpu_job_free_resources(job: *mut amdgpu_job);
}
extern "C" {
    pub fn amdgpu_job_free(job: *mut amdgpu_job);
}
extern "C" {
    pub fn amdgpu_job_stop_all_jobs_on_sched(sched: *mut drm_gpu_scheduler);
}
