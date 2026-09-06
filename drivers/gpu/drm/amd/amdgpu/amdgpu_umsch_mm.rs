//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/amdgpu/amdgpu_umsch_mm.h
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


// SPDX-License-Identifier: MIT
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
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum UMSCH_SWIP_ENGINE_TYPE {
    UMSCH_SWIP_ENGINE_TYPE_VCN0 = 0,
    UMSCH_SWIP_ENGINE_TYPE_VCN1 = 1,
    UMSCH_SWIP_ENGINE_TYPE_VCN = 2,
    UMSCH_SWIP_ENGINE_TYPE_VPE = 3,
    UMSCH_SWIP_ENGINE_TYPE_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum UMSCH_CONTEXT_PRIORITY_LEVEL {
    CONTEXT_PRIORITY_LEVEL_IDLE = 0,
    CONTEXT_PRIORITY_LEVEL_NORMAL = 1,
    CONTEXT_PRIORITY_LEVEL_FOCUS = 2,
    CONTEXT_PRIORITY_LEVEL_REALTIME = 3,
    CONTEXT_PRIORITY_NUM_LEVELS
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct umsch_mm_set_resource_input {
    pub vmid_mask_mm_vcn: u32,
    pub vmid_mask_mm_vpe: u32,
    pub collaboration_mask_vpe: u32,
    pub logging_vmid: u32,
    pub engine_mask: u32,
    pub 1: uint32_t disable_reset :,
    pub 1: uint32_t disable_umsch_mm_log :,
    pub 1: uint32_t use_rs64mem_for_proc_ctx_csa :,
    pub 29: uint32_t reserved :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_umsch_fwlog {
    pub rptr: u32,
    pub wptr: u32,
    pub buffer_size: u32,
    pub header_size: u32,
    pub wrapped: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct umsch_mm_add_queue_input {
    pub process_id: u32,
    pub page_table_base_addr: u64,
    pub process_va_start: u64,
    pub process_va_end: u64,
    pub process_quantum: u64,
    pub process_csa_addr: u64,
    pub context_quantum: u64,
    pub context_csa_addr: u64,
    pub inprocess_context_priority: u32,
    pub context_global_priority_level: UMSCH_CONTEXT_PRIORITY_LEVEL,
    pub doorbell_offset_0: u32,
    pub doorbell_offset_1: u32,
    pub engine_type: UMSCH_SWIP_ENGINE_TYPE,
    pub affinity: u32,
    pub mqd_addr: u64,
    pub h_context: u64,
    pub h_queue: u64,
    pub vm_context_cntl: u32,
    pub process_csa_array_index: u32,
    pub context_csa_array_index: u32,
    pub 1: uint32_t is_context_suspended :,
    pub 1: uint32_t collaboration_mode :,
    pub 30: uint32_t reserved :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct umsch_mm_remove_queue_input {
    pub doorbell_offset_0: u32,
    pub doorbell_offset_1: u32,
    pub context_csa_addr: u64,
    pub context_csa_array_index: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MQD_INFO {
    pub rb_base_hi: u32,
    pub rb_base_lo: u32,
    pub rb_size: u32,
    pub wptr_val: u32,
    pub rptr_val: u32,
    pub unmapped: u32,
    pub vmid: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct umsch_mm_funcs {
    pub umsch): *mut *mut int (set_hw_resources)(struct amdgpu_umsch_mm,
    pub input): *mut umsch_mm_add_queue_input,
    pub input): *mut umsch_mm_remove_queue_input,
    pub umsch): *mut *mut int (set_regs)(struct amdgpu_umsch_mm,
    pub umsch): *mut *mut int (init_microcode)(struct amdgpu_umsch_mm,
    pub umsch): *mut *mut int (load_microcode)(struct amdgpu_umsch_mm,
    pub umsch): *mut *mut int (ring_init)(struct amdgpu_umsch_mm,
    pub umsch): *mut *mut int (ring_start)(struct amdgpu_umsch_mm,
    pub umsch): *mut *mut int (ring_stop)(struct amdgpu_umsch_mm,
    pub umsch): *mut *mut int (ring_fini)(struct amdgpu_umsch_mm,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_umsch_mm {
    pub ring: amdgpu_ring,
    pub rb_wptr: u32,
    pub rb_rptr: u32,
    pub funcs: *const umsch_mm_funcs,
    pub fw: *const firmware,
    pub fw_version: u32,
    pub feature_version: u32,
    pub ucode_fw_obj: *mut amdgpu_bo,
    pub ucode_fw_gpu_addr: u64,
    pub ucode_fw_ptr: *mut u32,
    pub irq_start_addr: u64,
    pub uc_start_addr: u64,
    pub ucode_size: u32,
    pub data_fw_obj: *mut amdgpu_bo,
    pub data_fw_gpu_addr: u64,
    pub data_fw_ptr: *mut u32,
    pub data_start_addr: u64,
    pub data_size: u32,
    pub cmd_buf_obj: *mut amdgpu_bo,
    pub cmd_buf_gpu_addr: u64,
    pub cmd_buf_ptr: *mut u32,
    pub cmd_buf_curr_ptr: *mut u32,
    pub wb_index: u32,
    pub sch_ctx_gpu_addr: u64,
    pub sch_ctx_cpu_addr: *mut u32,
    pub vmid_mask_mm_vcn: u32,
    pub vmid_mask_mm_vpe: u32,
    pub engine_mask: u32,
    pub vcn0_hqd_mask: u32,
    pub vcn1_hqd_mask: u32,
    pub vcn_hqd_mask: [u32; 2],
    pub vpe_hqd_mask: u32,
    pub agdb_index: [u32; CONTEXT_PRIORITY_NUM_LEVELS],
    pub mutex_hidden: mutex,
    pub dbglog_bo: *mut amdgpu_bo,
    pub log_cpu_addr: *mut c_void,
    pub log_gpu_addr: u64,
    pub mem_size: u32,
    pub log_offset: u32,
}

extern "C" {
    pub fn amdgpu_umsch_mm_submit_pkt(umsch: *mut amdgpu_umsch_mm, pkt: *mut c_void, ndws: c_int) -> c_int;
}
extern "C" {
    pub fn amdgpu_umsch_mm_query_fence(umsch: *mut amdgpu_umsch_mm) -> c_int;
}
extern "C" {
    pub fn amdgpu_umsch_mm_init_microcode(umsch: *mut amdgpu_umsch_mm) -> c_int;
}
extern "C" {
    pub fn amdgpu_umsch_mm_allocate_ucode_buffer(umsch: *mut amdgpu_umsch_mm) -> c_int;
}
extern "C" {
    pub fn amdgpu_umsch_mm_allocate_ucode_data_buffer(umsch: *mut amdgpu_umsch_mm) -> c_int;
}
extern "C" {
    pub fn amdgpu_umsch_mm_psp_execute_cmd_buf(umsch: *mut amdgpu_umsch_mm) -> c_int;
}
extern "C" {
    pub fn amdgpu_umsch_mm_ring_init(umsch: *mut amdgpu_umsch_mm) -> c_int;
}
extern "C" {
    pub fn amdgpu_umsch_fwlog_init(umsch_mm: *mut amdgpu_umsch_mm);
}

// adev->umsch_mm.cmd_buf_curr_ptr++ = (reg_offset << 2);			\
// adev->umsch_mm.cmd_buf_curr_ptr++ = value;				\

