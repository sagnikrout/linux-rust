//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/amdgpu/amdgpu_ring.h
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
// Copyright 2016 Advanced Micro Devices, Inc.
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
// Authors: Christian König
//

// max number of rings
pub const AMDGPU_MAX_RINGS: c_int = 149;
pub const AMDGPU_MAX_HWIP_RINGS: c_int = 64;
pub const AMDGPU_MAX_GFX_RINGS: c_int = 2;
pub const AMDGPU_MAX_SW_GFX_RINGS: c_int = 2;
pub const AMDGPU_MAX_COMPUTE_RINGS: c_int = 8;
pub const AMDGPU_MAX_VCE_RINGS: c_int = 3;
pub const AMDGPU_MAX_UVD_ENC_RINGS: c_int = 2;
pub const AMDGPU_MAX_VPE_RINGS: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amdgpu_ring_priority_level {
    AMDGPU_RING_PRIO_0,
    AMDGPU_RING_PRIO_1,
    AMDGPU_RING_PRIO_DEFAULT = 1,
    AMDGPU_RING_PRIO_2,
    AMDGPU_RING_PRIO_MAX
}

// some special values for the owner field

// Ensure the execution in case of preemption or reset

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amdgpu_ring_type {
    AMDGPU_RING_TYPE_GFX		= AMDGPU_HW_IP_GFX,
    AMDGPU_RING_TYPE_COMPUTE	= AMDGPU_HW_IP_COMPUTE,
    AMDGPU_RING_TYPE_SDMA		= AMDGPU_HW_IP_DMA,
    AMDGPU_RING_TYPE_UVD		= AMDGPU_HW_IP_UVD,
    AMDGPU_RING_TYPE_VCE		= AMDGPU_HW_IP_VCE,
    AMDGPU_RING_TYPE_UVD_ENC	= AMDGPU_HW_IP_UVD_ENC,
    AMDGPU_RING_TYPE_VCN_DEC	= AMDGPU_HW_IP_VCN_DEC,
    AMDGPU_RING_TYPE_VCN_ENC	= AMDGPU_HW_IP_VCN_ENC,
    AMDGPU_RING_TYPE_VCN_JPEG	= AMDGPU_HW_IP_VCN_JPEG,
    AMDGPU_RING_TYPE_VPE		= AMDGPU_HW_IP_VPE,
    AMDGPU_RING_TYPE_KIQ,
    AMDGPU_RING_TYPE_MES,
    AMDGPU_RING_TYPE_UMSCH_MM,
    AMDGPU_RING_TYPE_CPER,
    AMDGPU_RING_TYPE_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amdgpu_ib_pool_type {
// Normal submissions to the top of the pipeline.
    AMDGPU_IB_POOL_DELAYED,
// Immediate submissions to the bottom of the pipeline.
    AMDGPU_IB_POOL_IMMEDIATE,
// Direct submission to the ring buffer during init and reset.
    AMDGPU_IB_POOL_DIRECT,

    AMDGPU_IB_POOL_MAX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_ib {
    pub sa_bo: *mut drm_suballoc,
    pub length_dw: u32,
    pub gpu_addr: u64,
    pub ptr: *mut u32,
    pub flags: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_sched {
    pub num_scheds: u32,
    pub sched: [*mut drm_gpu_scheduler; AMDGPU_MAX_HWIP_RINGS],
}

//
// Fences.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_fence_driver {
    pub gpu_addr: u64,
    pub cpu_addr: *mut u32,
// sync_seq is protected by ring emission lock
    pub sync_seq: u32,
    pub last_seq: core::sync::atomic::AtomicI32,
    pub initialized: bool,
    pub irq_src: *mut amdgpu_irq_src,
    pub irq_type: unsigned,
    pub fallback_timer: timer_list,
    pub num_fences_mask: unsigned,
    pub lock: spinlock_t,
    pub fences: *mut dma_fence,
}

//
// Fences mark an event in the GPUs pipeline and are used
// for GPU/CPU synchronization.  When the fence is written,
// it is expected that all buffers associated with that fence
// are no longer in use by the associated ring on the GPU and
// that the relevant GPU caches have been flushed.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_fence {
    pub base: dma_fence,
// RB, DMA, etc.
    pub ring: *mut amdgpu_ring,
    pub start_timestamp: ktime_t,
// location and size of the IB
    pub ib_wptr: u64,
    pub ib_dw_size: c_uint,
    pub skip_ib_dw_start_offset: c_uint,
    pub skip_ib_dw_end_offset: c_uint,
// fence context for resets
    pub context: u64,
// idx for ring backups
    pub backup_idx: c_uint,
}

extern "C" {
    pub fn amdgpu_fence_driver_set_error(ring: *mut amdgpu_ring, error: c_int);
}
extern "C" {
    pub fn amdgpu_fence_driver_init_ring(ring: *mut amdgpu_ring) -> c_int;
}
extern "C" {
    pub fn amdgpu_fence_driver_hw_init(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_fence_driver_hw_fini(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_fence_driver_sw_init(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_fence_driver_sw_fini(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_fence_process(ring: *mut amdgpu_ring) -> bool;
}
extern "C" {
    pub fn amdgpu_fence_wait_empty(ring: *mut amdgpu_ring) -> c_int;
}
extern "C" {
    pub fn amdgpu_fence_count_emitted(ring: *mut amdgpu_ring) -> unsigned;
}
extern "C" {
    pub fn amdgpu_fence_driver_isr_toggle(adev: *mut amdgpu_device, stop: bool);
}
extern "C" {
    pub fn amdgpu_fence_last_unsignaled_time_us(ring: *mut amdgpu_ring) -> u64;
}
//
// Rings.
//
// provided by hw blocks that expose a ring buffer for commands
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_ring_funcs {
//
// @type:
//
// GFX, Compute, SDMA, UVD, VCE, VCN, VPE, KIQ, MES, UMSCH, and CPER
// use ring buffers. The type field just identifies which component the
// ring buffer is associated with.
//
    pub type: amdgpu_ring_type,
    pub align_mask: u32,
//
// @nop:
//
// Every block in the amdgpu has no-op instructions (e.g., GFX 10
// uses PACKET3(PACKET3_NOP, 0x3FFF), VCN 5 uses VCN_ENC_CMD_NO_OP,
// etc). This field receives the specific no-op for the component
// that initializes the ring.
//
    pub nop: u32,
    pub support_64bit_ptrs: bool,
    pub no_user_fence: bool,
    pub secure_submission_supported: bool,
//
// @extra_bytes:
//
// Optional extra space in bytes that is added to the ring size
// when allocating the BO that holds the contents of the ring.
// This space isn't used for command submission to the ring,
// but is just there to satisfy some hardware requirements or
// implement workarounds. It's up to the implementation of each
// specific ring to initialize this space.
//
    pub extra_bytes: unsigned,
// ring read/write ptr handling
    pub ring): *mut *mut u64 (get_rptr)(struct amdgpu_ring,
    pub ring): *mut *mut u64 (get_wptr)(struct amdgpu_ring,
    pub ring): *mut *mut void (set_wptr)(struct amdgpu_ring,
// validating and patching of IBs
    pub ib): *mut amdgpu_ib,
    pub ib): *mut amdgpu_ib,
// constants to calculate how many DW are needed for an emit
    pub emit_frame_size: unsigned,
    pub emit_ib_size: unsigned,
// command emit functions
    pub flags): u32,
    pub flags): uint64_t seq, unsigned,
    pub ring): *mut *mut void (emit_pipeline_sync)(struct amdgpu_ring,
    pub pd_addr): u64,
    pub ring): *mut *mut void (emit_hdp_flush)(struct amdgpu_ring,
    pub oa_size): uint32_t oa_base, uint32_t,
// testing functions
    pub ring): *mut *mut int (test_ring)(struct amdgpu_ring,
    pub timeout): *mut *mut *mut int (test_ib)(struct amdgpu_ring ring, long,
// insert NOP packets
    pub count): *mut *mut *mut void (insert_nop)(struct amdgpu_ring ring, uint32_t,
    pub ring): *mut *mut void (insert_start)(struct amdgpu_ring,
    pub ring): *mut *mut void (insert_end)(struct amdgpu_ring,
// pad the indirect buffer to the necessary number of dw
    pub ib): *mut *mut *mut void (pad_ib)(struct amdgpu_ring ring, struct amdgpu_ib,
    pub addr): *mut *mut *mut unsigned (init_cond_exec)(struct amdgpu_ring ring, uint64_t,
// note usage for clock and power gating
    pub ring): *mut *mut void (begin_use)(struct amdgpu_ring,
    pub ring): *mut *mut void (end_use)(struct amdgpu_ring,
    pub ring): *mut *mut void (emit_switch_buffer) (struct amdgpu_ring,
    pub flags): *mut *mut *mut void (emit_cntxcntl) (struct amdgpu_ring ring, uint32_t,
    pub vmid): u64 gds_va, bool init_shadow, int,
    pub reg_val_offs): u32,
    pub val): *mut *mut *mut void (emit_wreg)(struct amdgpu_ring ring, uint32_t reg, uint32_t,
    pub mask): uint32_t val, uint32_t,
    pub mask): uint32_t ref, uint32_t,
    pub secure): bool,
// Try to soft recover the ring to make the fence signal
    pub vmid): *mut *mut *mut void (soft_recovery)(struct amdgpu_ring ring, unsigned,
    pub ring): *mut *mut int (preempt_ib)(struct amdgpu_ring,
    pub ring): *mut *mut void (emit_mem_sync)(struct amdgpu_ring,
    pub enable): *mut *mut *mut void (emit_wave_limit)(struct amdgpu_ring ring, bool,
    pub offset): *mut *mut *mut void (patch_cntl)(struct amdgpu_ring ring, unsigned,
    pub offset): *mut *mut *mut void (patch_ce)(struct amdgpu_ring ring, unsigned,
    pub offset): *mut *mut *mut void (patch_de)(struct amdgpu_ring ring, unsigned,
    pub timedout_fence): *mut amdgpu_fence,
    pub ring): *mut *mut void (emit_cleaner_shader)(struct amdgpu_ring,
}

//
// amdgpu_ring - Holds ring information
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_ring {
    pub adev: *mut amdgpu_device,
    pub funcs: *const amdgpu_ring_funcs,
    pub fence_drv: amdgpu_fence_driver,
    pub sched: drm_gpu_scheduler,
    pub ring_obj: *mut amdgpu_bo,
    pub ring: *mut u32,
// backups for resets
    pub ring_backup: *mut u32,
    pub ring_backup_entries_to_copy: c_uint,
    pub reemit: bool,
    pub guilty_fence: *mut amdgpu_fence,
    pub rptr_offs: unsigned,
    pub rptr_gpu_addr: u64,
    pub rptr_cpu_addr: *mut u32,
//
// @wptr:
//
// This is part of the Ring buffer implementation and represents the
// write pointer. The wptr determines where the host has written.
//
    pub wptr: u64,
//
// @wptr_old:
//
// Before update wptr with the new value, usually the old value is
// stored in the wptr_old.
//
    pub wptr_old: u64,
    pub ring_size: unsigned,
//
// @max_dw:
//
// Maximum number of DWords for ring allocation. This information is
// provided at the ring initialization time, and each IP block can
// specify a specific value. Check places that invoke
// amdgpu_ring_init() to see the maximum size per block.
//
    pub max_dw: unsigned,
//
// @count_dw:
//
// This value starts with the maximum amount of DWords supported by the
// ring. This value is updated based on the ring manipulation.
//
    pub count_dw: c_int,
    pub gpu_addr: u64,
//
// @ptr_mask:
//
// Some IPs provide support for 64-bit pointers and others for 32-bit
// only; this behavior is component-specific and defined by the field
// support_64bit_ptr. If the IP block supports 64-bits, the mask
// 0xffffffffffffffff is set; otherwise, this value assumes buf_mask.
// Notice that this field is used to keep wptr under a valid range.
//
    pub ptr_mask: u64,
//
// @buf_mask:
//
// Buffer mask is a value used to keep wptr count under its
// thresholding. Buffer mask initialized during the ring buffer
// initialization time, and it is defined as (ring_size / 4) -1.
//
    pub buf_mask: u32,
    pub idx: u32,
    pub xcc_id: u32,
    pub xcp_id: u32,
    pub me: u32,
    pub pipe: u32,
    pub queue: u32,
    pub mqd_obj: *mut amdgpu_bo,
    pub mqd_gpu_addr: u64,
    pub mqd_ptr: *mut c_void,
    pub mqd_size: unsigned,
    pub eop_gpu_addr: u64,
    pub doorbell_index: u32,
    pub use_doorbell: bool,
    pub use_pollmem: bool,
    pub wptr_offs: unsigned,
    pub wptr_gpu_addr: u64,
//
// @wptr_cpu_addr:
//
// This is the CPU address pointer in the writeback slot. This is used
// to commit changes to the GPU.
//
    pub wptr_cpu_addr: *mut u32,
    pub fence_offs: unsigned,
    pub fence_gpu_addr: u64,
    pub fence_cpu_addr: *mut u32,
    pub current_ctx: u64,
    pub name: [c_char; 16],
    pub trail_seq: u32,
    pub trail_fence_offs: unsigned,
    pub trail_fence_gpu_addr: u64,
    pub trail_fence_cpu_addr: *mut u32,
    pub cond_exe_offs: unsigned,
    pub cond_exe_gpu_addr: u64,
    pub cond_exe_cpu_addr: *mut u32,
    pub set_q_mode_offs: c_uint,
    pub set_q_mode_ptr: *mut u32,
    pub set_q_mode_token: u64,
    pub vm_hub: unsigned,
    pub vm_inv_eng: unsigned,
    pub vmid_wait: *mut dma_fence,
    pub has_compute_vm_bug: bool,
    pub no_scheduler: bool,
    pub no_user_submission: bool,
    pub hw_prio: c_int,
    pub num_hw_submission: unsigned,
    pub sched_score: *mut core::sync::atomic::AtomicI32,
    pub is_sw_ring: bool,
    pub entry_index: c_uint,
// store the cached rptr to restore after reset
    pub cached_rptr: u64,
}

extern "C" {
    pub fn amdgpu_ring_max_ibs(type: amdgpu_ring_type) -> c_uint;
}
extern "C" {
    pub fn amdgpu_ring_alloc(ring: *mut amdgpu_ring, ndw: unsigned) -> c_int;
}
extern "C" {
    pub fn amdgpu_ring_ib_begin(ring: *mut amdgpu_ring);
}
extern "C" {
    pub fn amdgpu_ring_ib_end(ring: *mut amdgpu_ring);
}
extern "C" {
    pub fn amdgpu_ring_ib_on_emit_cntl(ring: *mut amdgpu_ring);
}
extern "C" {
    pub fn amdgpu_ring_ib_on_emit_ce(ring: *mut amdgpu_ring);
}
extern "C" {
    pub fn amdgpu_ring_ib_on_emit_de(ring: *mut amdgpu_ring);
}
extern "C" {
    pub fn amdgpu_ring_insert_nop(ring: *mut amdgpu_ring, count: u32);
}
extern "C" {
    pub fn amdgpu_ring_generic_pad_ib(ring: *mut amdgpu_ring, ib: *mut amdgpu_ib);
}
extern "C" {
    pub fn amdgpu_ring_commit(ring: *mut amdgpu_ring);
}
extern "C" {
    pub fn amdgpu_ring_undo(ring: *mut amdgpu_ring);
}
extern "C" {
    pub fn amdgpu_ring_fini(ring: *mut amdgpu_ring);
}
// ring->cond_exe_cpu_addr = cond_exec;
//
// amdgpu_ring_patch_cond_exec - patch dw count of conditional execute
// @ring: amdgpu_ring structure
// @offset: offset returned by amdgpu_ring_init_cond_exec
//
// Calculate the dw count and patch it into a cond_exec command.
//
extern "C" {
    pub fn amdgpu_ring_test_helper(ring: *mut amdgpu_ring) -> c_int;
}
extern "C" {
    pub fn amdgpu_ring_init_mqd(ring: *mut amdgpu_ring) -> c_int;
}
extern "C" {
    pub fn amdgpu_ib_free(ib: *mut amdgpu_ib, f: *mut dma_fence);
}
extern "C" {
    pub fn amdgpu_ib_pool_init(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_ib_pool_fini(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_ib_ring_tests(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_ring_sched_ready(ring: *mut amdgpu_ring) -> bool;
}
