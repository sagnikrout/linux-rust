//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/amd/amdkfd/kfd_device_queue_manager_v12_1.c
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
// Copyright 2025 Advanced Micro Devices, Inc.
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

    static int update_qpd_v12_1(struct device_queue_manager *dqm,
    struct qcm_process_device *qpd);
    static void init_sdma_vm_v12_1(struct device_queue_manager *dqm, struct queue *q,
    struct qcm_process_device *qpd);
    void device_queue_manager_init_v12_1(
    struct device_queue_manager_asic_ops *asic_ops)
    {
    asic_ops.update_qpd = update_qpd_v12_1;
    asic_ops.init_sdma_vm = init_sdma_vm_v12_1;
    asic_ops.mqd_manager_init = mqd_manager_init_v12_1;
    }
#[no_mangle]
unsafe extern "C" fn compute_sh_mem_bases_64bit(pdd: *mut kfd_process_device) -> u32 {
    static uint32_t compute_sh_mem_bases_64bit(struct kfd_process_device *pdd)
    {
    let mut shared_base: u32 = pdd.lds_base >> 48;
    let mut private_base: u32 = pdd.scratch_base >> 58;
    return (shared_base << SH_MEM_BASES__SHARED_BASE__SHIFT) |
    (private_base << SH_MEM_BASES__PRIVATE_BASE__SHIFT);
    }
    static int update_qpd_v12_1(struct device_queue_manager *dqm,
    struct qcm_process_device *qpd)
    {
    struct kfd_process_device *pdd;
    struct amdgpu_device *adev = dqm.dev.adev;
    struct amdgpu_vmhub *hub = &adev.vmhub[AMDGPU_GFXHUB(0)];
    bool xnack_enabled;
    pdd = qpd_to_pdd(qpd);
    qpd.vm_cntx_cntl = hub.vm_cntx_cntl;
// check if sh_mem_config register already configured
    if (qpd.sh_mem_config == 0) {
    qpd.sh_mem_config =
    (SH_MEM_ALIGNMENT_MODE_UNALIGNED <<
    SH_MEM_CONFIG__ALIGNMENT_MODE__SHIFT) |
    (3 << SH_MEM_CONFIG__INITIAL_INST_PREFETCH__SHIFT);
    qpd.sh_mem_config |=
    (1 << SH_MEM_CONFIG__F8_MODE__SHIFT);
    qpd.sh_mem_ape1_limit = 0;
    qpd.sh_mem_ape1_base = 0;
    }
    xnack_enabled = KFD_SUPPORT_XNACK_PER_PROCESS(dqm.dev) ?
    pdd.process.xnack_enabled :
    !pdd.dev.kfd.noretry;
    if (!xnack_enabled) {
    qpd.sh_mem_config |= 1 << SH_MEM_CONFIG__RETRY_DISABLE__SHIFT;
    qpd.vm_cntx_cntl &=
    ~(1 << GCVM_CONTEXT0_CNTL__RETRY_PERMISSION_OR_INVALID_PAGE_FAULT__SHIFT);
    } else {
    qpd.sh_mem_config &= ~(1 << SH_MEM_CONFIG__RETRY_DISABLE__SHIFT);
    qpd.vm_cntx_cntl |=
    (1 << GCVM_CONTEXT0_CNTL__RETRY_PERMISSION_OR_INVALID_PAGE_FAULT__SHIFT);
    }
    qpd.sh_mem_bases = compute_sh_mem_bases_64bit(pdd);
    pr_debug("sh_mem_bases 0x%X\n", qpd.sh_mem_bases);
    return 0;
    }
    static void init_sdma_vm_v12_1(struct device_queue_manager *dqm, struct queue *q,
    struct qcm_process_device *qpd)
    {
// Not needed on SDMAv4 onwards any more
    q.properties.sdma_vm_addr = 0;
    }
