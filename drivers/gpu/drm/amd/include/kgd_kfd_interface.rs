//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/include/kgd_kfd_interface.h
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
// Copyright 2014 Advanced Micro Devices, Inc.
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
// This file defines the private interface between the
// AMD kernel graphics drivers and the AMD KFD.
//

// Macro flag: #define KGD_KFD_INTERFACE_H_INCLUDED

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kfd_preempt_type {
    KFD_PREEMPT_TYPE_WAVEFRONT_DRAIN = 0,
    KFD_PREEMPT_TYPE_WAVEFRONT_RESET,
    KFD_PREEMPT_TYPE_WAVEFRONT_SAVE
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kfd_vm_fault_info {
    pub page_addr: u64,
    pub vmid: u32,
    pub mc_id: u32,
    pub status: u32,
    pub prot_valid: bool,
    pub prot_read: bool,
    pub prot_write: bool,
    pub prot_exec: bool,
}

// For getting GPU local memory information from KGD
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kfd_local_mem_info {
    pub local_mem_size_private: u64,
    pub local_mem_size_public: u64,
    pub vram_width: u32,
    pub mem_clk_max: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kgd_memory_pool {
    KGD_POOL_SYSTEM_CACHEABLE = 1,
    KGD_POOL_SYSTEM_WRITECOMBINE = 2,
    KGD_POOL_FRAMEBUFFER = 3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kfd_cu_occupancy {
    pub wave_cnt: u32,
    pub doorbell_off: u32,
}

//
// enum kfd_sched_policy
//
// @KFD_SCHED_POLICY_HWS: H/W scheduling policy known as command processor (cp)
// scheduling. In this scheduling mode we're using the firmware code to
// schedule the user mode queues and kernel queues such as HIQ and DIQ.
// the HIQ queue is used as a special queue that dispatches the configuration
// to the cp and the user mode queues list that are currently running.
// the DIQ queue is a debugging queue that dispatches debugging commands to the
// firmware.
// in this scheduling mode user mode queues over subscription feature is
// enabled.
//
// @KFD_SCHED_POLICY_HWS_NO_OVERSUBSCRIPTION: The same as above but the over
// subscription feature disabled.
//
// @KFD_SCHED_POLICY_NO_HWS: no H/W scheduling policy is a mode which directly
// set the command processor registers and sets the queues "manually". This
// mode is used *ONLY* for debugging proposes.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kfd_sched_policy {
    KFD_SCHED_POLICY_HWS = 0,
    KFD_SCHED_POLICY_HWS_NO_OVERSUBSCRIPTION,
    KFD_SCHED_POLICY_NO_HWS
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kgd2kfd_shared_resources {
// Bit n == 1 means VMID n is available for KFD.
    pub compute_vmid_bitmap: c_uint,
// number of pipes per mec
    pub num_pipe_per_mec: u32,
// number of queues per pipe
    pub num_queue_per_pipe: u32,
// Bit n == 1 means Queue n is available for KFD
    pub AMDGPU_MAX_QUEUES): DECLARE_BITMAP(cp_queue_bitmap,,
// SDMA doorbell assignments (SOC15 and later chips only). Only
// specific doorbells are routed to each SDMA engine. Others
// are routed to IH and VCN. They are not usable by the CP.
//
    pub sdma_doorbell_idx: *mut u32,
// From SOC15 onward, the doorbell index range not usable for CP
// queues.
//
    pub non_cp_doorbells_start: u32,
    pub non_cp_doorbells_end: u32,
// Base address of doorbell aperture.
    pub doorbell_physical_address: phys_addr_t,
// Size in bytes of doorbell aperture.
    pub doorbell_aperture_size: usize,
// Number of bytes at start of aperture reserved for KGD.
    pub doorbell_start_offset: usize,
// GPUVM address space size in bytes
    pub gpuvm_size: u64,
// Minor device number of the render node
    pub drm_render_minor: c_int,
    pub enable_mes: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tile_config {
    pub tile_config_ptr: *mut u32,
    pub macro_tile_config_ptr: *mut u32,
    pub num_tile_configs: u32,
    pub num_macro_tile_configs: u32,
    pub gb_addr_config: u32,
    pub num_banks: u32,
    pub num_ranks: u32,
}

pub const KFD_MAX_NUM_OF_QUEUES_PER_DEVICE_DEFAULT: c_int = 4096;
//
// struct kfd2kgd_calls
//
// @program_sh_mem_settings: A function that should initiate the memory
// properties such as main aperture memory type (cache / non cached) and
// secondary aperture base address, size and memory type.
// This function is used only for no cp scheduling mode.
//
// @set_pasid_vmid_mapping: Exposes pasid/vmid pair to the H/W for no cp
// scheduling mode. Only used for no cp scheduling mode.
//
// @hqd_load: Loads the mqd structure to a H/W hqd slot. used only for no cp
// sceduling mode.
//
// @hqd_sdma_load: Loads the SDMA mqd structure to a H/W SDMA hqd slot.
// used only for no HWS mode.
//
// @hqd_dump: Dumps CPC HQD registers to an array of address-value pairs.
// Array is allocated with kmalloc, needs to be freed with kfree by caller.
//
// @hqd_sdma_dump: Dumps SDMA HQD registers to an array of address-value pairs.
// Array is allocated with kmalloc, needs to be freed with kfree by caller.
//
// @hqd_is_occupies: Checks if a hqd slot is occupied.
//
// @hqd_destroy: Destructs and preempts the queue assigned to that hqd slot.
//
// @hqd_sdma_is_occupied: Checks if an SDMA hqd slot is occupied.
//
// @hqd_sdma_destroy: Destructs and preempts the SDMA queue assigned to that
// SDMA hqd slot.
//
// @set_scratch_backing_va: Sets VA for scratch backing memory of a VMID.
// Only used for no cp scheduling mode
//
// @set_vm_context_page_table_base: Program page table base for a VMID
//
// @invalidate_tlbs: Invalidate TLBs for a specific PASID
//
// @invalidate_tlbs_vmid: Invalidate TLBs for a specific VMID
//
// @read_vmid_from_vmfault_reg: On Hawaii the VMID is not set in the
// IH ring entry. This function allows the KFD ISR to get the VMID
// from the fault status register as early as possible.
//
// @get_cu_occupancy: Function pointer that returns to caller the number
// of wave fronts that are in flight for all of the queues of a process
// as identified by its pasid. It is important to note that the value
// returned by this function is a snapshot of current moment and cannot
// guarantee any minimum for the number of waves in-flight. This function
// is defined for devices that belong to GFX9 and later GFX families. Care
// must be taken in calling this function as it is not defined for devices
// that belong to GFX8 and below GFX families.
//
// This structure contains function pointers to services that the kgd driver
// provides to amdkfd driver.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kfd2kgd_calls {
// Register access functions
    pub inst): u32,
    pub inst): unsigned int vmid, uint32_t,
    pub inst): u32,
    pub inst): *mut *mut mm_mm, uint32_t,
    pub inst): uint32_t doorbell_off, uint32_t,
    pub mm): *mut *mut uint32_t __user wptr, struct mm_struct,
    pub inst): *mut *mut *mut *mut uint32_t (dump)[2], uint32_t n_regs, uint32_t,
    pub n_regs): *mut *mut *mut uint32_t (dump)[2], uint32_t,
    pub inst): uint32_t queue_id, uint32_t,
    pub inst): uint32_t queue_id, uint32_t,
    pub mqd): *mut *mut *mut bool (hqd_sdma_is_occupied)(struct amdgpu_device adev, void,
    pub timeout): c_uint,
    pub inst): uint32_t sq_cmd, uint32_t,
    pub p_pasid): *mut u16,
// No longer needed from GFXv9 onward. The scratch base address is
// passed to the shader by the CP. It's the user mode driver's
// responsibility.
//
    pub vmid): uint64_t va, uint32_t,
    pub page_table_base): uint32_t vmid, uint64_t,
    pub adev): *mut *mut uint32_t (read_vmid_from_vmfault_reg)(struct amdgpu_device,
    pub vmid): u32,
    pub vmid): u32,
    pub trap_mask_supported): *mut u32,
    pub kfd_dbg_trap_cntl_prev): u32,
    pub vmid): u32,
    pub inst): u32,
    pub watch_id): u32,
    pub inst): u32,
    pub reg_data): *mut u32,
    pub inst): *mut *mut int max_waves_per_cu, uint32_t,
    pub inst): u32,
    pub inst): u32,
    pub utimeout): uint32_t inst, unsigned int,
    pub queue): int engine, int,
    pub fmt2): *mut amdgpu_ptl_fmt,
    pub val): *mut u64,
}
