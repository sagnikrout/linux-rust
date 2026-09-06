//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/amdgpu/amdgpu_gmc.h
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
// All Rights Reserved.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the
// "Software"), to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sub license, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT. IN NO EVENT SHALL
// THE COPYRIGHT HOLDERS, AUTHORS AND/OR ITS SUPPLIERS BE LIABLE FOR ANY CLAIM,
// DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR
// OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE
// USE OR OTHER DEALINGS IN THE SOFTWARE.
//
// The above copyright notice and this permission notice (including the
// next paragraph) shall be included in all copies or substantial portions
// of the Software.
//

// VA hole for 48bit and 57bit addresses

//
// Hardware is programmed as if the hole doesn't exists with start and end
// address values.
//
// This mask is used to remove the upper 16bits of the VA and so come up with
// the linear addr value.
//

//
// Ring size as power of two for the log of recent faults.
//
pub const AMDGPU_GMC_FAULT_RING_ORDER: c_int = 8;

//
// Hash size as power of two for the log of recent faults
//
pub const AMDGPU_GMC_FAULT_HASH_ORDER: c_int = 8;

//
// Number of IH timestamp ticks until a fault is considered handled
//

// XNACK flags

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amdgpu_memory_partition {
    UNKNOWN_MEMORY_PARTITION_MODE = 0,
    AMDGPU_NPS1_PARTITION_MODE = 1,
    AMDGPU_NPS2_PARTITION_MODE = 2,
    AMDGPU_NPS3_PARTITION_MODE = 3,
    AMDGPU_NPS4_PARTITION_MODE = 4,
    AMDGPU_NPS6_PARTITION_MODE = 6,
    AMDGPU_NPS8_PARTITION_MODE = 8,
}

pub const AMDGPU_MAX_MEM_RANGES: c_int = 8;
pub const AMDGPU_GMC9_FAULT_SOURCE_DATA_RETRY: c_uint = 0x80;
pub const AMDGPU_GMC9_FAULT_SOURCE_DATA_READ: c_uint = 0x40;
pub const AMDGPU_GMC9_FAULT_SOURCE_DATA_WRITE: c_uint = 0x20;
pub const AMDGPU_GMC9_FAULT_SOURCE_DATA_EXE: c_uint = 0x10;
pub const AMDGPU_GMC121_FAULT_SOURCE_DATA_READ: c_uint = 0x400000;
pub const AMDGPU_GMC121_FAULT_SOURCE_DATA_WRITE: c_uint = 0x200000;
pub const AMDGPU_GMC121_FAULT_SOURCE_DATA_EXE: c_uint = 0x100000;
//
// GMC page fault information
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_gmc_fault {
    pub timestamp:48: u64,
    pub next:AMDGPU_GMC_FAULT_RING_ORDER: u64,
    pub key: core::sync::atomic::AtomicI64,
    pub timestamp_expiry:48: u64,
}

//
// VMHUB structures, functions & helpers
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_vmhub_funcs {
    pub status): u32,
    pub flush_type): *mut *mut uint32_t (get_invalidate_req)(unsigned int vmid, uint32_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_vmhub {
    pub ctx0_ptb_addr_lo32: u32,
    pub ctx0_ptb_addr_hi32: u32,
    pub vm_inv_eng0_sem: u32,
    pub vm_inv_eng0_req: u32,
    pub vm_inv_eng0_ack: u32,
    pub vm_context0_cntl: u32,
    pub vm_l2_pro_fault_status: u32,
    pub vm_l2_pro_fault_cntl: u32,
//
// store the register distances between two continuous context domain
// and invalidation engine.
//
    pub ctx_distance: u32,
    pub /: *mut *mut uint32_t ctx_addr_distance; / include LO32/HI32,
    pub eng_distance: u32,
    pub /: *mut *mut uint32_t eng_addr_distance; / include LO32/HI32,
    pub vm_cntx_cntl: u32,
    pub vm_cntx_cntl_vm_fault: u32,
    pub vm_l2_bank_select_reserved_cid2: u32,
    pub vm_contexts_disable: u32,
    pub sdma_invalidation_workaround: bool,
    pub vmhub_funcs: *const amdgpu_vmhub_funcs,
}

//
// GPU MC structures, functions & helpers
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_gmc_funcs {
// flush the vm tlb via mmio
    pub flush_type): uint32_t vmhub, uint32_t,
// flush the vm tlb via pasid
    pub inst): u32,
// flush the vm tlb via ring
    pub pd_addr): u64,
// Change the VMID -> PASID mapping
    pub pasid): unsigned,
// enable/disable PRT support
    pub enable): *mut *mut *mut void (set_prt)(struct amdgpu_device adev, bool,
// get the pde for a given mc addr
    pub flags): *mut *mut u64 dst, u64,
// get the pte flags to use for PTEs
    pub pte_flags): *mut u64,
// override per-page pte flags
    pub flags): *mut uint64_t addr, uint64_t,
// get the amount of memory used by the vbios for pre-OS console
    pub adev): *mut *mut unsigned int (get_vbios_fb_size)(struct amdgpu_device,
// get the DCC buffer alignment
    pub adev): *mut *mut unsigned int (get_dcc_alignment)(struct amdgpu_device,
    pub adev): *mut amdgpu_device,
// Request NPS mode
    pub nps_mode): c_int,
    pub adev): *mut *mut bool (need_reset_on_init)(struct amdgpu_device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_mem_partition_info {
    pub fpfn: u32,
    pub lpfn: u32,
    pub range: },
    pub node: c_int,
    pub numa: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_gmc_memrange {
    pub base_address: u64,
    pub limit_address: u64,
    pub flags: u32,
    pub nid_mask: c_int,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amdgpu_gart_placement {
    AMDGPU_GART_PLACEMENT_BEST_FIT = 0,
    AMDGPU_GART_PLACEMENT_HIGH,
    AMDGPU_GART_PLACEMENT_LOW,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_gmc {
// FB's physical address in MMIO space (for CPU to
// map FB). This is different compared to the agp
// gart/vram_start/end field as the later is from
// GPU's view and aper_base is from CPU's view.
//
    pub aper_size: resource_size_t,
    pub aper_base: resource_size_t,
// for some chips with <= 32MB we need to lie
// about vram size near mc fb location
    pub mc_vram_size: u64,
    pub visible_vram_size: u64,
// AGP aperture start and end in MC address space
// Driver find a hole in the MC address space
// to place AGP by setting MC_VM_AGP_BOT/TOP registers
// Under VMID0, logical address == MC address. AGP
// aperture maps to physical bus or IOVA addressed.
// AGP aperture is used to simulate FB in ZFB case.
// AGP aperture is also used for page table in system
// memory (mainly for APU).
//
    pub agp_size: u64,
    pub agp_start: u64,
    pub agp_end: u64,
// GART aperture start and end in MC address space
// Driver find a hole in the MC address space
// to place GART by setting VM_CONTEXT0_PAGE_TABLE_START/END_ADDR
// registers
// Under VMID0, logical address inside GART aperture will
// be translated through gpuvm gart page table to access
// paged system memory
//
    pub gart_size: u64,
    pub gart_start: u64,
    pub gart_end: u64,
// Frame buffer aperture of this GPU device. Different from
// fb_start (see below), this only covers the local GPU device.
// If driver uses FB aperture to access FB, driver get fb_start from
// MC_VM_FB_LOCATION_BASE (set by vbios) and calculate vram_start
// of this local device by adding an offset inside the XGMI hive.
// If driver uses GART table for VMID0 FB access, driver finds a hole in
// VMID0's virtual address space to place the SYSVM aperture inside
// which the first part is vram and the second part is gart (covering
// system ram).
//
    pub vram_start: u64,
    pub vram_end: u64,
// FB region , it's same as local vram region in single GPU, in XGMI
// configuration, this region covers all GPUs in the same hive ,
// each GPU in the hive has the same view of this FB region .
// GPU0's vram starts at offset (0 * segment size) ,
// GPU1 starts at offset (1 * segment size), etc.
//
    pub fb_start: u64,
    pub fb_end: u64,
    pub vram_width: unsigned,
    pub real_vram_size: u64,
    pub vram_mtrr: c_int,
    pub mc_mask: u64,
    pub pte_addr_mask: u64,
    pub /: *const *const *const firmware fw; / MC firmware,
    pub fw_version: u32,
    pub vm_fault: amdgpu_irq_src,
    pub vram_type: u32,
    pub vram_vendor: u8,
    pub prt_warning: bool,
    pub sdpif_register: u32,
// apertures
    pub shared_aperture_start: u64,
    pub shared_aperture_end: u64,
    pub private_aperture_start: u64,
    pub private_aperture_end: u64,
// protects concurrent invalidation
    pub invalidate_lock: spinlock_t,
    pub translate_further: bool,
    pub vm_fault_info: *mut kfd_vm_fault_info,
    pub vm_fault_info_updated: core::sync::atomic::AtomicI32,
    pub fault_ring: [amdgpu_gmc_fault; AMDGPU_GMC_FAULT_RING_SIZE],
    pub idx:AMDGPU_GMC_FAULT_RING_ORDER: u64,
    pub fault_hash: [}; AMDGPU_GMC_FAULT_HASH_SIZE],
    pub last_fault:AMDGPU_GMC_FAULT_RING_ORDER: u64,
    pub tmz_enabled: bool,
    pub is_app_apu: bool,
    pub mem_partitions: *mut amdgpu_mem_partition_info,
    pub num_mem_partitions: u8,
    pub gmc_funcs: *const amdgpu_gmc_funcs,
    pub requested_nps_mode: amdgpu_memory_partition,
    pub supported_nps_modes: u32,
    pub reset_flags: u32,
    pub xgmi: amdgpu_xgmi,
    pub ecc_irq: amdgpu_irq_src,
    pub noretry: c_int,
    pub xnack_flags: u32,
    pub vmid0_page_table_block_size: u32,
    pub vmid0_page_table_depth: u32,
    pub pdb0_bo: *mut amdgpu_bo,
// CPU kmapped address of pdb0
    pub ptr_pdb0: *mut c_void,
// MALL size
    pub mall_size: u64,
    pub m_half_use: u32,
// number of UMC instances
    pub num_umc: c_int,
// mode2 save restore
    pub VM_L2_CNTL: u64,
    pub VM_L2_CNTL2: u64,
    pub VM_DUMMY_PAGE_FAULT_CNTL: u64,
    pub VM_DUMMY_PAGE_FAULT_ADDR_LO32: u64,
    pub VM_DUMMY_PAGE_FAULT_ADDR_HI32: u64,
    pub VM_L2_PROTECTION_FAULT_CNTL: u64,
    pub VM_L2_PROTECTION_FAULT_CNTL2: u64,
    pub VM_L2_PROTECTION_FAULT_MM_CNTL3: u64,
    pub VM_L2_PROTECTION_FAULT_MM_CNTL4: u64,
    pub VM_L2_PROTECTION_FAULT_ADDR_LO32: u64,
    pub VM_L2_PROTECTION_FAULT_ADDR_HI32: u64,
    pub VM_DEBUG: u64,
    pub VM_L2_MM_GROUP_RT_CLASSES: u64,
    pub VM_L2_BANK_SELECT_RESERVED_CID: u64,
    pub VM_L2_BANK_SELECT_RESERVED_CID2: u64,
    pub VM_L2_CACHE_PARITY_CNTL: u64,
    pub VM_L2_IH_LOG_CNTL: u64,
    pub VM_CONTEXT_CNTL: [u64; 16],
    pub VM_CONTEXT_PAGE_TABLE_BASE_ADDR_LO32: [u64; 16],
    pub VM_CONTEXT_PAGE_TABLE_BASE_ADDR_HI32: [u64; 16],
    pub VM_CONTEXT_PAGE_TABLE_START_ADDR_LO32: [u64; 16],
    pub VM_CONTEXT_PAGE_TABLE_START_ADDR_HI32: [u64; 16],
    pub VM_CONTEXT_PAGE_TABLE_END_ADDR_LO32: [u64; 16],
    pub VM_CONTEXT_PAGE_TABLE_END_ADDR_HI32: [u64; 16],
    pub MC_VM_MX_L1_TLB_CNTL: u64,
    pub noretry_flags: u64,
    pub init_pte_flags: u64,
    pub flush_tlb_needs_extra_type_0: bool,
    pub flush_tlb_needs_extra_type_2: bool,
    pub flush_pasid_uses_kiq: bool,
    pub override_pte: bool,
}

//
// amdgpu_gmc_vram_full_visible - Check if full VRAM is visible through the BAR
//
// @adev: amdgpu_device pointer
//
// Returns:
// True if full VRAM is visible through the BAR
//
// amdgpu_gmc_sign_extend - sign extend the given gmc address
//
// @addr: address to extend
//

extern "C" {
    pub fn amdgpu_gmc_is_pdb0_enabled(adev: *mut amdgpu_device) -> bool;
}
extern "C" {
    pub fn amdgpu_gmc_pdb0_alloc(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_gmc_pd_addr(bo: *mut amdgpu_bo) -> u64;
}
extern "C" {
    pub fn amdgpu_gmc_agp_addr(bo: *mut ttm_buffer_object) -> u64;
}
extern "C" {
    pub fn amdgpu_gmc_sysvm_location(adev: *mut amdgpu_device, mc: *mut amdgpu_gmc);
}
extern "C" {
    pub fn amdgpu_gmc_ras_sw_init(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_gmc_allocate_vm_inv_eng(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_gmc_tmz_set(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_gmc_noretry_set(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_gmc_init_vga_resv_regions(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_gmc_init_pdb0(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_gmc_vram_mc2pa(adev: *mut amdgpu_device, mc_addr: u64) -> u64;
}
extern "C" {
    pub fn amdgpu_gmc_vram_pa(adev: *mut amdgpu_device, bo: *mut amdgpu_bo) -> u64;
}
extern "C" {
    pub fn amdgpu_gmc_vram_checking(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_gmc_sysfs_init(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_gmc_sysfs_fini(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_gmc_prepare_nps_mode_change(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_gmc_need_reset_on_init(adev: *mut amdgpu_device) -> bool;
}
extern "C" {
    pub fn amdgpu_gmc_init_mem_ranges(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_gmc_set_gart_size(adev: *mut amdgpu_device, default_size: u64);
}
