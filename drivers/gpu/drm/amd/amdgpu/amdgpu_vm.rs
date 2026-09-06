//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/amdgpu/amdgpu_vm.h
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

//
// GPUVM handling
//
// Maximum number of PTEs the hardware can write with one command
pub const AMDGPU_VM_MAX_UPDATE_SIZE: c_uint = 0x3FFFF;
// number of entries in page table

// RV+

// VI only

// TILED for VEGA10, reserved for older ASICs

// PDE is handled as PTE for VEGA10

// PTE is handled as PDE for VEGA10 (Translate Further)

// MALL noalloc for sienna_cichlid, reserved for older ASICs

// PDE Block Fragment Size for VEGA10

// Flag combination to set no-retry with TF disabled

// Flag combination to set no-retry with TF enabled

// For GFX9

pub const AMDGPU_MTYPE_NC: c_int = 0;
pub const AMDGPU_MTYPE_CC: c_int = 2;

// gfx10

// gfx12

// PDE Block Fragment Size for gfx v12

// PDE is handled as PTE for gfx v12

// How to program VM fault handling
pub const AMDGPU_VM_FAULT_STOP_NEVER: c_int = 0;
pub const AMDGPU_VM_FAULT_STOP_FIRST: c_int = 1;
pub const AMDGPU_VM_FAULT_STOP_ALWAYS: c_int = 2;
// How much VRAM be reserved for page tables

//
// max number of VMHUB
// layout: max 8 GFXHUB + 4 MMHUB0 + 1 MMHUB1
//
pub const AMDGPU_MAX_VMHUBS: c_int = 13;
pub const AMDGPU_GFXHUB_START: c_int = 0;
pub const AMDGPU_MMHUB0_START: c_int = 8;
pub const AMDGPU_MMHUB1_START: c_int = 12;

// Reserve space at top/bottom of address space for kernel use

// See vm_update_mode

// VMPT level enumerate, and the hiberachy is:
// PDB3->PDB2->PDB1->PDB0->PTB
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amdgpu_vm_level {
    AMDGPU_VM_PDB3,
    AMDGPU_VM_PDB2,
    AMDGPU_VM_PDB1,
    AMDGPU_VM_PDB0,
    AMDGPU_VM_PTB
}

// base structure for tracking BO usage in a VM
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_vm_bo_base {
// constant after initialization
    pub vm: *mut amdgpu_vm,
    pub bo: *mut amdgpu_bo,
// protected by bo being reserved
    pub next: *mut amdgpu_vm_bo_base,
// protected by vm reservation and invalidated_lock
    pub vm_status: list_head,
// if the bo is counted as shared in mem stats
// protected by vm BO being reserved
    pub shared: bool,
// if the BO was moved and all mappings are invalid
// protected by the BO being reserved
    pub moved: bool,
}

//
// The following status lists contain amdgpu_vm_bo_base objects for
// either PD/PTs, per VM BOs or BOs with individual resv object.
//
// The state transits are: evicted -> needs_update -> idle
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_vm_bo_status {
// BOs evicted which need to move into place again
    pub evicted: list_head,
// BOs whose mappings changed but PDs/PTs haven't been updated
    pub needs_update: list_head,
// BOs done with the state machine and need no further action
    pub idle: list_head,
}

// provided by hw blocks that can write ptes, e.g., sdma
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_vm_pte_funcs {
// number of dw to reserve per operation
    pub copy_pte_num_dw: unsigned,
// copy pte entries from GART
    pub count): unsigned,
// write pte one entry at a time with addr mapping
    pub incr): u32,
// for linear pte/pde updates without addr mapping
    pub flags): uint32_t incr, uint64_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_task_info {
    pub task: drm_wedge_task_info,
    pub process_name: [c_char; TASK_COMM_LEN],
    pub tgid: pid_t,
    pub refcount: kref,
}

//
// struct amdgpu_vm_update_params
//
// Encapsulate some VM table update parameters to reduce
// the number of function parameters
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_vm_update_params {
//
// @adev: amdgpu device we do this update for
//
    pub adev: *mut amdgpu_device,
//
// @vm: optional amdgpu_vm we do this update for
//
    pub vm: *mut amdgpu_vm,
//
// @immediate: if changes should be made immediately
//
    pub immediate: bool,
//
// @unlocked: true if the root BO is not locked
//
    pub unlocked: bool,
//
// @pages_addr:
//
// DMA addresses to use for mapping
//
    pub pages_addr: *mut dma_addr_t,
//
// @job: job to used for hw submission
//
    pub job: *mut amdgpu_job,
//
// @num_dw_left: number of dw left for the IB
//
    pub num_dw_left: c_uint,
//
// @needs_flush: true whenever we need to invalidate the TLB
//
    pub needs_flush: bool,
//
// @override_pte: true for memory that is not uncached and gmc override function is
// implemented to allow MTYPE to be overridden for NUMA local memory.
//
    pub override_pte: bool,
//
// @tlb_flush_waitlist: temporary storage for BOs until tlb_flush
//
    pub tlb_flush_waitlist: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_vm_update_funcs {
    pub bo): *mut *mut int (map_table)(struct amdgpu_bo_vm,
    pub k_job_id): *mut *mut amdgpu_sync sync, u64,
    pub flags): unsigned count, uint32_t incr, uint64_t,
    pub fence): *mut dma_fence,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_vm_fault_info {
// fault address
    pub addr: u64,
// fault status register
    pub status: u32,
// which vmhub? gfxhub, mmhub, etc.
    pub vmhub: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_mem_stats {
    pub drm: drm_memory_stats,
// buffers that requested this placement but are currently evicted
    pub evicted: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_vm {
// tree of virtual addresses mapped
    pub va: rb_root_cached,
// Lock to prevent eviction while we are updating page tables
// use vm_eviction_lock/unlock(vm)
//
    pub eviction_lock: mutex,
    pub evicting: bool,
    pub saved_flags: c_uint,
// Memory statistics for this vm, protected by stats_lock
    pub stats_lock: spinlock_t,
    pub stats: [amdgpu_mem_stats; __AMDGPU_PL_NUM],
// BO's belonging to PD/PT which are internal to the kernel.
    pub kernel: amdgpu_vm_bo_status,
//
// BOs allocated by userspace where the dma_resv is shared with the
// root PD
//
    pub always_valid: amdgpu_vm_bo_status,
//
// The following lists contain amdgpu_vm_bo_base objects for BOs which
// have their own dma_resv object and not depend on the root PD.
//
// Lists are protected by the individual_lock.
//
    pub individual_lock: spinlock_t,
// Userspace BOs with individual resv object
    pub individual: amdgpu_vm_bo_status,
//
// This list contains amdgpu_bo_va_mapping objects which have been freed
// but not updated in the PTs
//
    pub freed: list_head,
// contains the page directory
    pub root: amdgpu_vm_bo_base,
    pub last_update: *mut dma_fence,
// Scheduler entities for page table updates
    pub immediate: drm_sched_entity,
    pub delayed: drm_sched_entity,
// Last finished delayed update
    pub tlb_seq: core::sync::atomic::AtomicI64,
    pub last_tlb_flush: *mut dma_fence,
    pub kfd_last_flushed_seq: core::sync::atomic::AtomicI64,
    pub tlb_fence_context: u64,
// How many times we had to re-generate the page tables
    pub generation: u64,
// Last unlocked submission to the scheduler entities
    pub last_unlocked: *mut dma_fence,
    pub pasid: c_uint,
    pub reserved_vmid: [*mut amdgpu_vmid; AMDGPU_MAX_VMHUBS],
// Flag to indicate if VM tables are updated by CPU or GPU (SDMA)
    pub use_cpu_for_update: bool,
// Functions to use for VM table updates
    pub update_funcs: *const amdgpu_vm_update_funcs,
// Up to 128 pending retry page faults
    pub 128): DECLARE_KFIFO(faults, u64,,
// Points to the KFD process VM info
    pub process_info: *mut amdkfd_process_info,
// List node in amdkfd_process_info.vm_list_head
    pub vm_list_node: list_head,
// Valid while the PD is reserved or fenced
    pub pd_phys_addr: u64,
// Some basic info about the task
    pub task_info: *mut amdgpu_task_info,
// Store positions of group of BOs
    pub lru_bulk_move: ttm_lru_bulk_move,
// Flag to indicate if VM is used for compute
    pub is_compute_context: bool,
// Flag to indicate if VM needs a TLB fence (KFD or KGD)
    pub need_tlb_fence: bool,
// Memory partition number, -1 means any partition
    pub mem_id: i8,
// cached fault info
    pub fault_info: amdgpu_vm_fault_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_vm_manager {
// Handling of VMIDs
    pub id_mgr: [amdgpu_vmid_mgr; AMDGPU_MAX_VMHUBS],
    pub first_kfd_vmid: c_uint,
    pub concurrent_flush: bool,
    pub max_pfn: u64,
    pub max_level: u32,
    pub num_level: u32,
    pub block_size: u32,
    pub fragment_size: u32,
    pub root_level: amdgpu_vm_level,
// vram base address for page table entry
    pub vram_base_offset: u64,
// vm pte handling
    pub vm_pte_funcs: *const amdgpu_vm_pte_funcs,
    pub vm_pte_scheds: [*mut drm_gpu_scheduler; AMDGPU_MAX_RINGS],
    pub vm_pte_num_scheds: unsigned,
    pub page_fault: *mut amdgpu_ring,
// partial resident texture handling
    pub prt_lock: spinlock_t,
    pub num_prt_users: core::sync::atomic::AtomicI32,
// controls how VM page tables are updated for Graphics and Compute.
// BIT0[= 0] Graphics updated by SDMA [= 1] by CPU
// BIT1[= 0] Compute updated by SDMA [= 1] by CPU
//
    pub vm_update_mode: c_int,
// Global registration of recent page fault information
    pub fault_info: amdgpu_vm_fault_info,
}

extern "C" {
    pub fn amdgpu_vm_manager_init(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_vm_manager_fini(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_vm_wait_idle(vm: *mut amdgpu_vm, timeout: c_long) -> c_long;
}
extern "C" {
    pub fn amdgpu_vm_init(adev: *mut amdgpu_device, vm: *mut amdgpu_vm, xcp_id: i32) -> c_int;
}
extern "C" {
    pub fn amdgpu_vm_make_compute(adev: *mut amdgpu_device, vm: *mut amdgpu_vm) -> c_int;
}
extern "C" {
    pub fn amdgpu_vm_fini(adev: *mut amdgpu_device, vm: *mut amdgpu_vm);
}
extern "C" {
    pub fn amdgpu_vm_ready(vm: *mut amdgpu_vm) -> bool;
}
extern "C" {
    pub fn amdgpu_vm_generation(adev: *mut amdgpu_device, vm: *mut amdgpu_vm) -> u64;
}
extern "C" {
    pub fn amdgpu_vm_evictable(bo: *mut amdgpu_bo) -> bool;
}
extern "C" {
    pub fn amdgpu_vm_bo_invalidate(bo: *mut amdgpu_bo, evicted: bool);
}
extern "C" {
    pub fn amdgpu_vm_bo_update_shared(bo: *mut amdgpu_bo);
}
extern "C" {
    pub fn amdgpu_vm_map_gart(pages_addr: *const dma_addr_t, addr: u64) -> u64;
}
extern "C" {
    pub fn amdgpu_vm_bo_trace_cs(vm: *mut amdgpu_vm, ticket: *mut ww_acquire_ctx);
}
extern "C" {
    pub fn amdgpu_vm_ioctl(dev: *mut drm_device, data: *mut c_void, filp: *mut drm_file) -> c_int;
}
extern "C" {
    pub fn amdgpu_vm_check_compute_bug(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_vm_put_task_info(task_info: *mut amdgpu_task_info);
}
extern "C" {
    pub fn amdgpu_vm_set_task_info(vm: *mut amdgpu_vm);
}
extern "C" {
    pub fn amdgpu_vm_pt_free_root(adev: *mut amdgpu_device, vm: *mut amdgpu_vm);
}
extern "C" {
    pub fn amdgpu_vm_pt_free_work(work: *mut work_struct);
}

extern "C" {
    pub fn amdgpu_debugfs_vm_bo_info(vm: *mut amdgpu_vm, m: *mut seq_file);
}

extern "C" {
    pub fn amdgpu_vm_pt_map_tables(adev: *mut amdgpu_device, vm: *mut amdgpu_vm) -> c_int;
}
extern "C" {
    pub fn amdgpu_vm_is_bo_always_valid(vm: *mut amdgpu_vm, bo: *mut amdgpu_bo) -> bool;
}
//
// amdgpu_vm_tlb_seq - return tlb flush sequence number
// @vm: the amdgpu_vm structure to query
//
// Returns the tlb flush sequence number which indicates that the VM TLBs needs
// to be invalidated whenever the sequence number change.
//
// Workaround to stop racing between the fence signaling and handling
// the cb. The lock is static after initially setting it up, just make
// sure that the dma_fence structure isn't freed up.
//
extern "C" {
    pub fn atomic64_read(_arg: &vm->tlb_seq) -> return;
}
//
// vm eviction_lock can be taken in MMU notifiers. Make sure no reclaim-FS
// happens while holding this lock anywhere to prevent deadlocks when
// an MMU notifier runs in reclaim-FS context.
//

