//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/radeon/radeon.h
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
// Copyright 2008 Advanced Micro Devices, Inc.
// Copyright 2008 Red Hat Inc.
// Copyright 2009 Jerome Glisse.
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
// Authors: Dave Airlie
// Alex Deucher
// Jerome Glisse
//
// TODO: Here are things that needs to be done :
// - surface allocator & initializer : (bit like scratch reg) should
// initialize HDP_ stuff on RS600, R600, R700 hw, well anythings
// related to surface
// - WB : write back stuff (do it bit like scratch reg things)
// - Vblank : look at Jesse's rework and what we should do
// - r600/r700: gart & cp
// - cs : clean cs ioctl use bitmap & things like that.
// - power management stuff
// - Barrier in gart code
// - Unmappabled vram ?
// - TESTING, TESTING, TESTING
//
// Initialization path:
// We expect that acceleration initialization might fail for various
// reasons even thought we work hard to make it works on most
// configurations. In order to still have a working userspace in such
// situation the init path must succeed up to the memory controller
// initialization point. Failure before this point are considered as
// fatal error. Here is the init callchain :
// radeon_device_init  perform common structure, mutex initialization
// asic_init           setup the GPU memory layout and perform all
// one time initialization (failure in this
// function are considered fatal)
// asic_startup        setup the GPU acceleration, in order to
// follow guideline the first thing this
// function should do is setting the GPU
// memory controller (only MC setup failure
// are considered as fatal)
//

//
// Modules parameters.
//
// Copy from radeon_drv.h so we don't have to include both and have conflicting
// symbol;
//

// RADEON_IB_POOL_SIZE must be a power of 2
pub const RADEON_IB_POOL_SIZE: c_int = 16;
pub const RADEON_DEBUGFS_MAX_COMPONENTS: c_int = 32;
pub const RADEON_BIOS_NUM_SCRATCH: c_int = 8;
// internal ring indices
// r1xx+ has gfx CP ring
pub const RADEON_RING_TYPE_GFX_INDEX: c_int = 0;
// cayman has 2 compute CP rings
pub const CAYMAN_RING_TYPE_CP1_INDEX: c_int = 1;
pub const CAYMAN_RING_TYPE_CP2_INDEX: c_int = 2;
// R600+ has an async dma ring
pub const R600_RING_TYPE_DMA_INDEX: c_int = 3;
// cayman add a second async dma ring
pub const CAYMAN_RING_TYPE_DMA1_INDEX: c_int = 4;
// R600+
pub const R600_RING_TYPE_UVD_INDEX: c_int = 5;
// TN+
pub const TN_RING_TYPE_VCE1_INDEX: c_int = 6;
pub const TN_RING_TYPE_VCE2_INDEX: c_int = 7;
// max number of rings
pub const RADEON_NUM_RINGS: c_int = 8;
// number of hw syncs before falling back on blocking
pub const RADEON_NUM_SYNCS: c_int = 4;
// hardcode those limit for now

// hard reset data
pub const RADEON_ASIC_RESET_DATA: c_uint = 0x39d5e86b;
// reset flags

// CG block flags

// CG flags

// PG flags

// max cursor sizes (in pixels)
pub const CURSOR_WIDTH: c_int = 64;
pub const CURSOR_HEIGHT: c_int = 64;
pub const CIK_CURSOR_WIDTH: c_int = 128;
pub const CIK_CURSOR_HEIGHT: c_int = 128;
//
// Errata workarounds.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum radeon_pll_errata {
    CHIP_ERRATA_R300_CG             = 0x00000001,
    CHIP_ERRATA_PLL_DUMMYREADS      = 0x00000002,
    CHIP_ERRATA_PLL_DELAY           = 0x00000004
}

//
// BIOS.
//
extern "C" {
    pub fn radeon_get_bios(rdev: *mut radeon_device) -> bool;
}
//
// Dummy page
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct radeon_dummy_page {
    pub entry: u64,
    pub page: *mut page,
    pub addr: dma_addr_t,
}

extern "C" {
    pub fn radeon_dummy_page_init(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn radeon_dummy_page_fini(rdev: *mut radeon_device);
}
//
// Clocks
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct radeon_clock {
    pub p1pll: radeon_pll,
    pub p2pll: radeon_pll,
    pub dcpll: radeon_pll,
    pub spll: radeon_pll,
    pub mpll: radeon_pll,
// 10 Khz units
    pub default_mclk: u32,
    pub default_sclk: u32,
    pub default_dispclk: u32,
    pub current_dispclk: u32,
    pub dp_extclk: u32,
    pub max_pixel_clock: u32,
    pub vco_freq: u32,
}

//
// Power management
//
extern "C" {
    pub fn radeon_pm_init(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn radeon_pm_late_init(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn radeon_pm_fini(rdev: *mut radeon_device);
}
extern "C" {
    pub fn radeon_pm_compute_clocks(rdev: *mut radeon_device);
}
extern "C" {
    pub fn radeon_pm_suspend(rdev: *mut radeon_device);
}
extern "C" {
    pub fn radeon_pm_resume(rdev: *mut radeon_device);
}
extern "C" {
    pub fn radeon_combios_get_power_modes(rdev: *mut radeon_device);
}
extern "C" {
    pub fn radeon_atombios_get_power_modes(rdev: *mut radeon_device);
}
extern "C" {
    pub fn radeon_atom_set_voltage(rdev: *mut radeon_device, voltage_level: u16, voltage_type: u8);
}
extern "C" {
    pub fn rs690_pm_info(rdev: *mut radeon_device);
}
//
// Fences.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct radeon_fence_driver {
    pub rdev: *mut radeon_device,
    pub scratch_reg: u32,
    pub gpu_addr: u64,
    pub cpu_addr: *mut volatile uint32_t,
// sync_seq is protected by ring emission lock
    pub sync_seq: [u64; RADEON_NUM_RINGS],
    pub last_seq: core::sync::atomic::AtomicI64,
    pub delayed_irq: bool initialized,,
    pub lockup_work: delayed_work,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct radeon_fence {
    pub base: dma_fence,
    pub rdev: *mut radeon_device,
    pub seq: u64,
// RB, DMA, etc.
    pub ring: unsigned,
    pub is_vm_update: bool,
    pub fence_wake: wait_queue_entry_t,
}

extern "C" {
    pub fn radeon_fence_driver_start_ring(rdev: *mut radeon_device, ring: c_int) -> c_int;
}
extern "C" {
    pub fn radeon_fence_driver_init(rdev: *mut radeon_device);
}
extern "C" {
    pub fn radeon_fence_driver_fini(rdev: *mut radeon_device);
}
extern "C" {
    pub fn radeon_fence_driver_force_completion(rdev: *mut radeon_device, ring: c_int);
}
extern "C" {
    pub fn radeon_fence_emit(rdev: *mut radeon_device, fence: *mut radeon_fence, ring: c_int) -> c_int;
}
extern "C" {
    pub fn radeon_fence_process(rdev: *mut radeon_device, ring: c_int);
}
extern "C" {
    pub fn radeon_fence_signaled(fence: *mut radeon_fence) -> bool;
}
extern "C" {
    pub fn radeon_fence_wait_timeout(fence: *mut radeon_fence, interruptible: bool, timeout: c_long) -> c_long;
}
extern "C" {
    pub fn radeon_fence_wait(fence: *mut radeon_fence, interruptible: bool) -> c_int;
}
extern "C" {
    pub fn radeon_fence_wait_next(rdev: *mut radeon_device, ring: c_int) -> c_int;
}
extern "C" {
    pub fn radeon_fence_wait_empty(rdev: *mut radeon_device, ring: c_int) -> c_int;
}
extern "C" {
    pub fn radeon_fence_unref(fence: *mut radeon_fence);
}
extern "C" {
    pub fn radeon_fence_count_emitted(rdev: *mut radeon_device, ring: c_int) -> unsigned;
}
extern "C" {
    pub fn radeon_fence_need_sync(fence: *mut radeon_fence, ring: c_int) -> bool;
}
extern "C" {
    pub fn radeon_fence_note_sync(fence: *mut radeon_fence, ring: c_int);
}
//
// Tiling registers
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct radeon_surface_reg {
    pub bo: *mut radeon_bo,
}

pub const RADEON_GEM_MAX_SURFACES: c_int = 8;
//
// TTM.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct radeon_mman {
    pub bdev: ttm_device,
    pub initialized: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct radeon_bo_list {
    pub robj: *mut radeon_bo,
    pub list: list_head,
    pub shared: bool,
    pub gpu_offset: u64,
    pub preferred_domains: unsigned,
    pub allowed_domains: unsigned,
    pub tiling_flags: u32,
}

// bo virtual address in a specific vm
#[repr(C)]
#[derive(Copy, Clone)]
pub struct radeon_bo_va {
// protected by bo being reserved
    pub bo_list: list_head,
    pub flags: u32,
    pub last_pt_update: *mut radeon_fence,
    pub ref_count: unsigned,
// protected by vm mutex
    pub it: interval_tree_node,
    pub vm_status: list_head,
// constant after initialization
    pub vm: *mut radeon_vm,
    pub bo: *mut radeon_bo,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct radeon_bo {
// Protected by gem.mutex
    pub list: list_head,
// Protected by tbo.reserved
    pub initial_domain: u32,
    pub placements: [ttm_place; 4],
    pub placement: ttm_placement,
    pub tbo: ttm_buffer_object,
    pub kmap: ttm_bo_kmap_obj,
    pub flags: u32,
    pub kptr: *mut c_void,
    pub tiling_flags: u32,
    pub pitch: u32,
    pub surface_reg: c_int,
    pub prime_shared_count: unsigned,
// list of all virtual address to which this bo
// is associated to
//
    pub va: list_head,
// Constant after initialization
    pub rdev: *mut radeon_device,
    pub pid: pid_t,

    pub notifier: mmu_interval_notifier,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct radeon_sa_manager {
    pub base: drm_suballoc_manager,
    pub bo: *mut radeon_bo,
    pub gpu_addr: u64,
    pub cpu_ptr: *mut c_void,
    pub domain: u32,
}

//
// GEM objects.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct radeon_gem {
    pub mutex: mutex,
    pub objects: list_head,
}

extern "C" {
    pub fn radeon_align_pitch(rdev: *mut radeon_device, width: c_int, cpp: c_int, tiled: bool) -> c_int;
}
extern "C" {
    pub fn radeon_gem_init(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn radeon_gem_fini(rdev: *mut radeon_device);
}
//
// Semaphores.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct radeon_semaphore {
    pub sa_bo: *mut drm_suballoc,
    pub waiters: signed,
    pub gpu_addr: u64,
}

//
// Synchronization
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct radeon_sync {
    pub semaphores: [*mut radeon_semaphore; RADEON_NUM_SYNCS],
    pub sync_to: [*mut radeon_fence; RADEON_NUM_RINGS],
    pub last_vm_update: *mut radeon_fence,
}

extern "C" {
    pub fn radeon_sync_create(sync: *mut radeon_sync);
}
//
// GART structures, functions & helpers
//
pub const RADEON_GPU_PAGE_SIZE: c_int = 4096;

pub const RADEON_GPU_PAGE_SHIFT: c_int = 12;

pub const RADEON_GART_PAGE_DUMMY: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct radeon_gart {
    pub table_addr: dma_addr_t,
    pub robj: *mut radeon_bo,
    pub ptr: *mut c_void,
    pub num_gpu_pages: unsigned,
    pub num_cpu_pages: unsigned,
    pub table_size: unsigned,
    pub pages: *mut page,
    pub pages_entry: *mut u64,
    pub ready: bool,
}

extern "C" {
    pub fn radeon_gart_table_ram_alloc(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn radeon_gart_table_ram_free(rdev: *mut radeon_device);
}
extern "C" {
    pub fn radeon_gart_table_vram_alloc(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn radeon_gart_table_vram_free(rdev: *mut radeon_device);
}
extern "C" {
    pub fn radeon_gart_table_vram_pin(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn radeon_gart_table_vram_unpin(rdev: *mut radeon_device);
}
extern "C" {
    pub fn radeon_gart_init(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn radeon_gart_fini(rdev: *mut radeon_device);
}
//
// GPU MC structures, functions & helpers
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct radeon_mc {
    pub aper_size: resource_size_t,
    pub aper_base: resource_size_t,
    pub agp_base: resource_size_t,
// for some chips with <= 32MB we need to lie
// about vram size near mc fb location
    pub mc_vram_size: u64,
    pub visible_vram_size: u64,
    pub gtt_size: u64,
    pub gtt_start: u64,
    pub gtt_end: u64,
    pub vram_start: u64,
    pub vram_end: u64,
    pub vram_width: unsigned,
    pub real_vram_size: u64,
    pub vram_mtrr: c_int,
    pub vram_is_ddr: bool,
    pub igp_sideport_enabled: bool,
    pub gtt_base_align: u64,
    pub mc_mask: u64,
}

extern "C" {
    pub fn radeon_combios_sideport_present(rdev: *mut radeon_device) -> bool;
}
extern "C" {
    pub fn radeon_atombios_sideport_present(rdev: *mut radeon_device) -> bool;
}
//
// GPU scratch registers structures, functions & helpers
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct radeon_scratch {
    pub num_reg: unsigned,
    pub reg_base: u32,
    pub free: [bool; 32],
    pub reg: [u32; 32],
}

extern "C" {
    pub fn radeon_scratch_get(rdev: *mut radeon_device, reg: *mut u32) -> c_int;
}
extern "C" {
    pub fn radeon_scratch_free(rdev: *mut radeon_device, reg: u32);
}
//
// GPU doorbell structures, functions & helpers
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct radeon_doorbell {
// doorbell mmio
    pub base: resource_size_t,
    pub size: resource_size_t,
    pub ptr: *mut u32 __iomem,
    pub /: *mut *mut u32 num_doorbells; / Number of doorbells actually reserved for radeon.,
    pub RADEON_MAX_DOORBELLS): DECLARE_BITMAP(used,,
}

extern "C" {
    pub fn radeon_doorbell_get(rdev: *mut radeon_device, page: *mut u32) -> c_int;
}
extern "C" {
    pub fn radeon_doorbell_free(rdev: *mut radeon_device, doorbell: u32);
}
//
// IRQS.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct radeon_flip_work {
    pub flip_work: work_struct,
    pub unpin_work: work_struct,
    pub rdev: *mut radeon_device,
    pub crtc_id: c_int,
    pub target_vblank: u32,
    pub base: u64,
    pub event: *mut drm_pending_vblank_event,
    pub old_rbo: *mut radeon_bo,
    pub fence: *mut dma_fence,
    pub async: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct r500_irq_stat_regs {
    pub disp_int: u32,
    pub hdmi0_status: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct r600_irq_stat_regs {
    pub disp_int: u32,
    pub disp_int_cont: u32,
    pub disp_int_cont2: u32,
    pub d1grph_int: u32,
    pub d2grph_int: u32,
    pub hdmi0_status: u32,
    pub hdmi1_status: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct evergreen_irq_stat_regs {
    pub disp_int: [u32; 6],
    pub grph_int: [u32; 6],
    pub afmt_status: [u32; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cik_irq_stat_regs {
    pub disp_int: u32,
    pub disp_int_cont: u32,
    pub disp_int_cont2: u32,
    pub disp_int_cont3: u32,
    pub disp_int_cont4: u32,
    pub disp_int_cont5: u32,
    pub disp_int_cont6: u32,
    pub d1grph_int: u32,
    pub d2grph_int: u32,
    pub d3grph_int: u32,
    pub d4grph_int: u32,
    pub d5grph_int: u32,
    pub d6grph_int: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union radeon_irq_stat_regs {
    pub r500: r500_irq_stat_regs,
    pub r600: r600_irq_stat_regs,
    pub evergreen: evergreen_irq_stat_regs,
    pub cik: cik_irq_stat_regs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct radeon_irq {
    pub installed: bool,
    pub lock: spinlock_t,
    pub ring_int: [core::sync::atomic::AtomicI32; RADEON_NUM_RINGS],
    pub crtc_vblank_int: [bool; RADEON_MAX_CRTCS],
    pub pflip: [core::sync::atomic::AtomicI32; RADEON_MAX_CRTCS],
    pub vblank_queue: wait_queue_head_t,
    pub hpd: [bool; RADEON_MAX_HPD_PINS],
    pub afmt: [bool; RADEON_MAX_AFMT_BLOCKS],
    pub stat_regs: radeon_irq_stat_regs,
    pub dpm_thermal: bool,
}

extern "C" {
    pub fn radeon_irq_kms_init(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn radeon_irq_kms_fini(rdev: *mut radeon_device);
}
extern "C" {
    pub fn radeon_irq_kms_sw_irq_get(rdev: *mut radeon_device, ring: c_int);
}
extern "C" {
    pub fn radeon_irq_kms_sw_irq_get_delayed(rdev: *mut radeon_device, ring: c_int) -> bool;
}
extern "C" {
    pub fn radeon_irq_kms_sw_irq_put(rdev: *mut radeon_device, ring: c_int);
}
extern "C" {
    pub fn radeon_irq_kms_pflip_irq_get(rdev: *mut radeon_device, crtc: c_int);
}
extern "C" {
    pub fn radeon_irq_kms_pflip_irq_put(rdev: *mut radeon_device, crtc: c_int);
}
extern "C" {
    pub fn radeon_irq_kms_enable_afmt(rdev: *mut radeon_device, block: c_int);
}
extern "C" {
    pub fn radeon_irq_kms_disable_afmt(rdev: *mut radeon_device, block: c_int);
}
extern "C" {
    pub fn radeon_irq_kms_enable_hpd(rdev: *mut radeon_device, hpd_mask: unsigned);
}
extern "C" {
    pub fn radeon_irq_kms_disable_hpd(rdev: *mut radeon_device, hpd_mask: unsigned);
}
//
// CP & rings.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct radeon_ib {
    pub sa_bo: *mut drm_suballoc,
    pub length_dw: u32,
    pub gpu_addr: u64,
    pub ptr: *mut u32,
    pub ring: c_int,
    pub fence: *mut radeon_fence,
    pub vm: *mut radeon_vm,
    pub is_const_ib: bool,
    pub sync: radeon_sync,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct radeon_ring {
    pub rdev: *mut radeon_device,
    pub ring_obj: *mut radeon_bo,
    pub ring: *mut volatile uint32_t,
    pub rptr_offs: unsigned,
    pub rptr_save_reg: unsigned,
    pub next_rptr_gpu_addr: u64,
    pub next_rptr_cpu_addr: *mut volatile u32,
    pub wptr: unsigned,
    pub wptr_old: unsigned,
    pub ring_size: unsigned,
    pub ring_free_dw: unsigned,
    pub count_dw: c_int,
    pub last_rptr: core::sync::atomic::AtomicI32,
    pub last_activity: core::sync::atomic::AtomicI64,
    pub gpu_addr: u64,
    pub align_mask: u32,
    pub ptr_mask: u32,
    pub ready: bool,
    pub nop: u32,
    pub idx: u32,
    pub last_semaphore_signal_addr: u64,
    pub last_semaphore_wait_addr: u64,
// for CIK queues
    pub me: u32,
    pub pipe: u32,
    pub queue: u32,
    pub mqd_obj: *mut radeon_bo,
    pub doorbell_index: u32,
    pub wptr_offs: unsigned,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct radeon_mec {
    pub hpd_eop_obj: *mut radeon_bo,
    pub hpd_eop_gpu_addr: u64,
    pub num_pipe: u32,
    pub num_mec: u32,
    pub num_queue: u32,
}

//
// VM
//
// maximum number of VMIDs
pub const RADEON_NUM_VM: c_int = 16;
// number of entries in page table

// PTBs (Page Table Blocks) need to be aligned to 32K
pub const RADEON_VM_PTB_ALIGN_SIZE: c_int = 32768;

// PTE (Page Table Entry) fragment field for different page sizes

// flags needed to be set so we can copy directly from the GART table

#[repr(C)]
#[derive(Copy, Clone)]
pub struct radeon_vm_pt {
    pub bo: *mut radeon_bo,
    pub addr: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct radeon_vm_id {
    pub id: unsigned,
    pub pd_gpu_addr: u64,
// last flushed PD/PT update
    pub flushed_updates: *mut radeon_fence,
// last use of vmid
    pub last_id_use: *mut radeon_fence,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct radeon_vm {
    pub mutex: mutex,
    pub va: rb_root_cached,
// protecting invalidated and freed
    pub status_lock: spinlock_t,
// BOs moved, but not yet updated in the PT
    pub invalidated: list_head,
// BOs freed, but not yet updated in the PT
    pub freed: list_head,
// BOs cleared in the PT
    pub cleared: list_head,
// contains the page directory
    pub page_directory: *mut radeon_bo,
    pub max_pde_used: unsigned,
// array of page tables, one for each page directory entry
    pub page_tables: *mut radeon_vm_pt,
    pub ib_bo_va: *mut radeon_bo_va,
// for id and flush management per ring
    pub ids: [radeon_vm_id; RADEON_NUM_RINGS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct radeon_vm_manager {
    pub active: [*mut radeon_fence; RADEON_NUM_VM],
    pub max_pfn: u32,
// number of VMIDs
    pub nvm: unsigned,
// vram base address for page table entry
    pub vram_base_offset: u64,
// is vm enabled?
    pub enabled: bool,
// for hw to save the PD addr on suspend/resume
    pub saved_table_addr: [u32; RADEON_NUM_VM],
}

//
// file private structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct radeon_fpriv {
    pub vm: radeon_vm,
}

//
// R6xx+ IH ring
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct r600_ih {
    pub ring_obj: *mut radeon_bo,
    pub ring: *mut volatile uint32_t,
    pub rptr: unsigned,
    pub ring_size: unsigned,
    pub gpu_addr: u64,
    pub ptr_mask: u32,
    pub lock: core::sync::atomic::AtomicI32,
    pub enabled: bool,
}

//
// RLC stuff
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct radeon_rlc {
// for power gating
    pub save_restore_obj: *mut radeon_bo,
    pub save_restore_gpu_addr: u64,
    pub sr_ptr: *mut volatile uint32_t,
    pub reg_list: *const u32,
    pub reg_list_size: u32,
// for clear state
    pub clear_state_obj: *mut radeon_bo,
    pub clear_state_gpu_addr: u64,
    pub cs_ptr: *mut volatile uint32_t,
    pub cs_data: *const cs_section_def,
    pub clear_state_size: u32,
// for cp tables
    pub cp_table_obj: *mut radeon_bo,
    pub cp_table_gpu_addr: u64,
    pub cp_table_ptr: *mut volatile uint32_t,
    pub cp_table_size: u32,
}

extern "C" {
    pub fn radeon_ib_free(rdev: *mut radeon_device, ib: *mut radeon_ib);
}
extern "C" {
    pub fn radeon_ib_pool_init(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn radeon_ib_pool_fini(rdev: *mut radeon_device);
}
extern "C" {
    pub fn radeon_ib_ring_tests(rdev: *mut radeon_device) -> c_int;
}
// Ring access between begin & end cannot sleep
extern "C" {
    pub fn radeon_ring_free_size(rdev: *mut radeon_device, cp: *mut radeon_ring);
}
extern "C" {
    pub fn radeon_ring_alloc(rdev: *mut radeon_device, cp: *mut radeon_ring, ndw: unsigned) -> c_int;
}
extern "C" {
    pub fn radeon_ring_lock(rdev: *mut radeon_device, cp: *mut radeon_ring, ndw: unsigned) -> c_int;
}
extern "C" {
    pub fn radeon_ring_undo(ring: *mut radeon_ring);
}
extern "C" {
    pub fn radeon_ring_unlock_undo(rdev: *mut radeon_device, cp: *mut radeon_ring);
}
extern "C" {
    pub fn radeon_ring_test(rdev: *mut radeon_device, cp: *mut radeon_ring) -> c_int;
}
extern "C" {
    pub fn radeon_ring_test_lockup(rdev: *mut radeon_device, ring: *mut radeon_ring) -> bool;
}
extern "C" {
    pub fn radeon_ring_fini(rdev: *mut radeon_device, cp: *mut radeon_ring);
}
// r600 async dma
extern "C" {
    pub fn r600_dma_stop(rdev: *mut radeon_device);
}
extern "C" {
    pub fn r600_dma_resume(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn r600_dma_fini(rdev: *mut radeon_device);
}
extern "C" {
    pub fn cayman_dma_stop(rdev: *mut radeon_device);
}
extern "C" {
    pub fn cayman_dma_resume(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn cayman_dma_fini(rdev: *mut radeon_device);
}
//
// CS.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct radeon_cs_chunk {
    pub length_dw: u32,
    pub kdata: *mut u32,
    pub user_ptr: *mut void __user,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct radeon_cs_parser {
    pub dev: *mut device,
    pub rdev: *mut radeon_device,
    pub filp: *mut drm_file,
// chunks
    pub nchunks: unsigned,
    pub chunks: *mut radeon_cs_chunk,
    pub chunks_array: *mut u64,
// IB
    pub idx: unsigned,
// relocations
    pub nrelocs: unsigned,
    pub relocs: *mut radeon_bo_list,
    pub vm_bos: *mut radeon_bo_list,
    pub validated: list_head,
    pub dma_reloc_idx: unsigned,
    pub exec: drm_exec,
// indices of various chunks
    pub chunk_ib: *mut radeon_cs_chunk,
    pub chunk_relocs: *mut radeon_cs_chunk,
    pub chunk_flags: *mut radeon_cs_chunk,
    pub chunk_const_ib: *mut radeon_cs_chunk,
    pub ib: radeon_ib,
    pub const_ib: radeon_ib,
    pub track: *mut c_void,
    pub family: unsigned,
    pub parser_error: c_int,
    pub cs_flags: u32,
    pub ring: u32,
    pub priority: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct radeon_cs_packet {
    pub idx: unsigned,
    pub type: unsigned,
    pub reg: unsigned,
    pub opcode: unsigned,
    pub count: c_int,
    pub one_reg_wr: unsigned,
}

//
// AGP
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct radeon_agp_mode {
    pub /: *mut *mut *mut unsigned long mode; /< AGP mode,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct radeon_agp_info {
    pub agp_version_major: c_int,
    pub agp_version_minor: c_int,
    pub mode: c_ulong,
    pub /: *mut *mut unsigned long aperture_base; / physical address,
    pub /: *mut *mut unsigned long aperture_size; / bytes,
    pub /: *mut *mut unsigned long memory_allowed; / bytes,
    pub memory_used: c_ulong,
// PCI information
    pub id_vendor: c_ushort,
    pub id_device: c_ushort,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct radeon_agp_head {
    pub agp_info: agp_kern_info,
    pub memory: list_head,
    pub mode: c_ulong,
    pub bridge: *mut agp_bridge_data,
    pub enabled: c_int,
    pub acquired: c_int,
    pub base: c_ulong,
    pub agp_mtrr: c_int,
    pub cant_use_aperture: c_int,
    pub page_mask: c_ulong,
}

extern "C" {
    pub fn radeon_agp_init(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn radeon_agp_resume(rdev: *mut radeon_device);
}
extern "C" {
    pub fn radeon_agp_suspend(rdev: *mut radeon_device);
}
extern "C" {
    pub fn radeon_agp_fini(rdev: *mut radeon_device);
}
//
// Writeback
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct radeon_wb {
    pub wb_obj: *mut radeon_bo,
    pub wb: *mut volatile uint32_t,
    pub gpu_addr: u64,
    pub enabled: bool,
    pub use_event: bool,
}

pub const RADEON_WB_SCRATCH_OFFSET: c_int = 0;
pub const RADEON_WB_RING0_NEXT_RPTR: c_int = 256;
pub const RADEON_WB_CP_RPTR_OFFSET: c_int = 1024;
pub const RADEON_WB_CP1_RPTR_OFFSET: c_int = 1280;
pub const RADEON_WB_CP2_RPTR_OFFSET: c_int = 1536;
pub const R600_WB_DMA_RPTR_OFFSET: c_int = 1792;
pub const R600_WB_IH_WPTR_OFFSET: c_int = 2048;
pub const CAYMAN_WB_DMA1_RPTR_OFFSET: c_int = 2304;
pub const R600_WB_EVENT_OFFSET: c_int = 3072;
pub const CIK_WB_CP1_WPTR_OFFSET: c_int = 3328;
pub const CIK_WB_CP2_WPTR_OFFSET: c_int = 3584;
pub const R600_WB_DMA_RING_TEST_OFFSET: c_int = 3588;
pub const CAYMAN_WB_DMA1_RING_TEST_OFFSET: c_int = 3592;
//
// struct radeon_pm - power management datas
// @max_bandwidth:      maximum bandwidth the gpu has (MByte/s)
// @igp_sideport_mclk:  sideport memory clock Mhz (rs690,rs740,rs780,rs880)
// @igp_system_mclk:    system clock Mhz (rs690,rs740,rs780,rs880)
// @igp_ht_link_clk:    ht link clock Mhz (rs690,rs740,rs780,rs880)
// @igp_ht_link_width:  ht link width in bits (rs690,rs740,rs780,rs880)
// @k8_bandwidth:       k8 bandwidth the gpu has (MByte/s) (IGP)
// @sideport_bandwidth: sideport bandwidth the gpu has (MByte/s) (IGP)
// @ht_bandwidth:       ht bandwidth the gpu has (MByte/s) (IGP)
// @core_bandwidth:     core GPU bandwidth the gpu has (MByte/s) (IGP)
// @sclk:          	GPU clock Mhz (core bandwidth depends of this clock)
// @needed_bandwidth:   current bandwidth needs
//
// It keeps track of various data needed to take powermanagement decision.
// Bandwidth need is used to determine minimun clock of the GPU and memory.
// Equation between gpu/memory clock and available bandwidth is hw dependent
// (type of memory, bus size, efficiency, ...)
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum radeon_pm_method {
    PM_METHOD_PROFILE,
    PM_METHOD_DYNPM,
    PM_METHOD_DPM,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum radeon_dynpm_state {
    DYNPM_STATE_DISABLED,
    DYNPM_STATE_MINIMUM,
    DYNPM_STATE_PAUSED,
    DYNPM_STATE_ACTIVE,
    DYNPM_STATE_SUSPENDED,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum radeon_dynpm_action {
    DYNPM_ACTION_NONE,
    DYNPM_ACTION_MINIMUM,
    DYNPM_ACTION_DOWNCLOCK,
    DYNPM_ACTION_UPCLOCK,
    DYNPM_ACTION_DEFAULT
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum radeon_voltage_type {
    VOLTAGE_NONE = 0,
    VOLTAGE_GPIO,
    VOLTAGE_VDDC,
    VOLTAGE_SW
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum radeon_pm_state_type {
// not used for dpm
    POWER_STATE_TYPE_DEFAULT,
    POWER_STATE_TYPE_POWERSAVE,
// user selectable states
    POWER_STATE_TYPE_BATTERY,
    POWER_STATE_TYPE_BALANCED,
    POWER_STATE_TYPE_PERFORMANCE,
// internal states
    POWER_STATE_TYPE_INTERNAL_UVD,
    POWER_STATE_TYPE_INTERNAL_UVD_SD,
    POWER_STATE_TYPE_INTERNAL_UVD_HD,
    POWER_STATE_TYPE_INTERNAL_UVD_HD2,
    POWER_STATE_TYPE_INTERNAL_UVD_MVC,
    POWER_STATE_TYPE_INTERNAL_BOOT,
    POWER_STATE_TYPE_INTERNAL_THERMAL,
    POWER_STATE_TYPE_INTERNAL_ACPI,
    POWER_STATE_TYPE_INTERNAL_ULV,
    POWER_STATE_TYPE_INTERNAL_3DPERF,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum radeon_pm_profile_type {
    PM_PROFILE_DEFAULT,
    PM_PROFILE_AUTO,
    PM_PROFILE_LOW,
    PM_PROFILE_MID,
    PM_PROFILE_HIGH,
}

pub const PM_PROFILE_DEFAULT_IDX: c_int = 0;
pub const PM_PROFILE_LOW_SH_IDX: c_int = 1;
pub const PM_PROFILE_MID_SH_IDX: c_int = 2;
pub const PM_PROFILE_HIGH_SH_IDX: c_int = 3;
pub const PM_PROFILE_LOW_MH_IDX: c_int = 4;
pub const PM_PROFILE_MID_MH_IDX: c_int = 5;
pub const PM_PROFILE_HIGH_MH_IDX: c_int = 6;
pub const PM_PROFILE_MAX: c_int = 7;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct radeon_pm_profile {
    pub dpms_off_ps_idx: c_int,
    pub dpms_on_ps_idx: c_int,
    pub dpms_off_cm_idx: c_int,
    pub dpms_on_cm_idx: c_int,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum radeon_int_thermal_type {
    THERMAL_TYPE_NONE,
    THERMAL_TYPE_EXTERNAL,
    THERMAL_TYPE_EXTERNAL_GPIO,
    THERMAL_TYPE_RV6XX,
    THERMAL_TYPE_RV770,
    THERMAL_TYPE_ADT7473_WITH_INTERNAL,
    THERMAL_TYPE_EVERGREEN,
    THERMAL_TYPE_SUMO,
    THERMAL_TYPE_NI,
    THERMAL_TYPE_SI,
    THERMAL_TYPE_EMC2103_WITH_INTERNAL,
    THERMAL_TYPE_CI,
    THERMAL_TYPE_KV,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct radeon_voltage {
    pub type: radeon_voltage_type,
// gpio voltage
    pub gpio: radeon_gpio_rec,
    pub /: *mut *mut u32 delay; / delay in usec from voltage drop to sclk change,
    pub /: *mut *mut bool active_high; / voltage drop is active when bit is high,
// VDDC voltage
    pub /: *mut *mut u8 vddc_id; / index into vddc voltage table,
    pub /: *mut *mut u8 vddci_id; / index into vddci voltage table,
    pub vddci_enabled: bool,
// r6xx+ sw
    pub voltage: u16,
// evergreen+ vddci
    pub vddci: u16,
}

// clock mode flags

#[repr(C)]
#[derive(Copy, Clone)]
pub struct radeon_pm_clock_info {
// memory clock
    pub mclk: u32,
// engine clock
    pub sclk: u32,
// voltage info
    pub voltage: radeon_voltage,
// standardized clock flags
    pub flags: u32,
}

// state flags

#[repr(C)]
#[derive(Copy, Clone)]
pub struct radeon_power_state {
    pub type: radeon_pm_state_type,
    pub clock_info: *mut radeon_pm_clock_info,
// number of valid clock modes in this power state
    pub num_clock_modes: c_int,
    pub default_clock_mode: *mut radeon_pm_clock_info,
// standardized state flags
    pub flags: u32,
    pub /: *mut *mut u32 misc; / vbios specific flags,
    pub /: *mut *mut u32 misc2; / vbios specific flags,
    pub /: *mut *mut int pcie_lanes; / pcie lanes,
}

//
// Some modes are overclocked by very low value, accept them
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum radeon_dpm_auto_throttle_src {
    RADEON_DPM_AUTO_THROTTLE_SRC_THERMAL,
    RADEON_DPM_AUTO_THROTTLE_SRC_EXTERNAL
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum radeon_dpm_event_src {
    RADEON_DPM_EVENT_SRC_ANALOG = 0,
    RADEON_DPM_EVENT_SRC_EXTERNAL = 1,
    RADEON_DPM_EVENT_SRC_DIGITAL = 2,
    RADEON_DPM_EVENT_SRC_ANALOG_OR_EXTERNAL = 3,
    RADEON_DPM_EVENT_SRC_DIGIAL_OR_EXTERNAL = 4
}

pub const RADEON_MAX_VCE_LEVELS: c_int = 6;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum radeon_vce_level {
    RADEON_VCE_LEVEL_AC_ALL = 0,     /* AC, All cases */
    RADEON_VCE_LEVEL_DC_EE = 1,      /* DC, entropy encoding */
    RADEON_VCE_LEVEL_DC_LL_LOW = 2,  /* DC, low latency queue, res <= 720 */
    RADEON_VCE_LEVEL_DC_LL_HIGH = 3, /* DC, low latency queue, 1080 >= res > 720 */
    RADEON_VCE_LEVEL_DC_GP_LOW = 4,  /* DC, general purpose queue, res <= 720 */
    RADEON_VCE_LEVEL_DC_GP_HIGH = 5, /* DC, general purpose queue, 1080 >= res > 720 */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct radeon_ps {
    pub /: *mut *mut u32 caps; / vbios flags,
    pub /: *mut *mut u32 class; / vbios flags,
    pub /: *mut *mut u32 class2; / vbios flags,
// UVD clocks
    pub vclk: u32,
    pub dclk: u32,
// VCE clocks
    pub evclk: u32,
    pub ecclk: u32,
    pub vce_active: bool,
    pub vce_level: radeon_vce_level,
// asic priv
    pub ps_priv: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct radeon_dpm_thermal {
// thermal interrupt work
    pub work: work_struct,
// low temperature threshold
    pub min_temp: c_int,
// high temperature threshold
    pub max_temp: c_int,
// was interrupt low to high or high to low
    pub high_to_low: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum radeon_clk_action {
    RADEON_SCLK_UP = 1,
    RADEON_SCLK_DOWN
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct radeon_blacklist_clocks {
    pub sclk: u32,
    pub mclk: u32,
    pub action: radeon_clk_action,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct radeon_clock_and_voltage_limits {
    pub sclk: u32,
    pub mclk: u32,
    pub vddc: u16,
    pub vddci: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct radeon_clock_array {
    pub count: u32,
    pub values: *mut u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct radeon_clock_voltage_dependency_entry {
    pub clk: u32,
    pub v: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct radeon_clock_voltage_dependency_table {
    pub count: u32,
    pub entries: *mut radeon_clock_voltage_dependency_entry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union radeon_cac_leakage_entry {
    pub vddc: u16,
    pub leakage: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct radeon_cac_leakage_table {
    pub count: u32,
    pub entries: *mut radeon_cac_leakage_entry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct radeon_phase_shedding_limits_entry {
    pub voltage: u16,
    pub sclk: u32,
    pub mclk: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct radeon_phase_shedding_limits_table {
    pub count: u32,
    pub entries: *mut radeon_phase_shedding_limits_entry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct radeon_uvd_clock_voltage_dependency_entry {
    pub vclk: u32,
    pub dclk: u32,
    pub v: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct radeon_uvd_clock_voltage_dependency_table {
    pub count: u8,
    pub entries: *mut radeon_uvd_clock_voltage_dependency_entry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct radeon_vce_clock_voltage_dependency_entry {
    pub ecclk: u32,
    pub evclk: u32,
    pub v: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct radeon_vce_clock_voltage_dependency_table {
    pub count: u8,
    pub entries: *mut radeon_vce_clock_voltage_dependency_entry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct radeon_ppm_table {
    pub ppm_design: u8,
    pub cpu_core_number: u16,
    pub platform_tdp: u32,
    pub small_ac_platform_tdp: u32,
    pub platform_tdc: u32,
    pub small_ac_platform_tdc: u32,
    pub apu_tdp: u32,
    pub dgpu_tdp: u32,
    pub dgpu_ulv_power: u32,
    pub tj_max: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct radeon_cac_tdp_table {
    pub tdp: u16,
    pub configurable_tdp: u16,
    pub tdc: u16,
    pub battery_power_limit: u16,
    pub small_power_limit: u16,
    pub low_cac_leakage: u16,
    pub high_cac_leakage: u16,
    pub maximum_power_delivery_limit: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct radeon_dpm_dynamic_state {
    pub vddc_dependency_on_sclk: radeon_clock_voltage_dependency_table,
    pub vddci_dependency_on_mclk: radeon_clock_voltage_dependency_table,
    pub vddc_dependency_on_mclk: radeon_clock_voltage_dependency_table,
    pub mvdd_dependency_on_mclk: radeon_clock_voltage_dependency_table,
    pub vddc_dependency_on_dispclk: radeon_clock_voltage_dependency_table,
    pub uvd_clock_voltage_dependency_table: radeon_uvd_clock_voltage_dependency_table,
    pub vce_clock_voltage_dependency_table: radeon_vce_clock_voltage_dependency_table,
    pub samu_clock_voltage_dependency_table: radeon_clock_voltage_dependency_table,
    pub acp_clock_voltage_dependency_table: radeon_clock_voltage_dependency_table,
    pub valid_sclk_values: radeon_clock_array,
    pub valid_mclk_values: radeon_clock_array,
    pub max_clock_voltage_on_dc: radeon_clock_and_voltage_limits,
    pub max_clock_voltage_on_ac: radeon_clock_and_voltage_limits,
    pub mclk_sclk_ratio: u32,
    pub sclk_mclk_delta: u32,
    pub vddc_vddci_delta: u16,
    pub min_vddc_for_pcie_gen2: u16,
    pub cac_leakage_table: radeon_cac_leakage_table,
    pub phase_shedding_limits_table: radeon_phase_shedding_limits_table,
    pub ppm_table: *mut radeon_ppm_table,
    pub cac_tdp_table: *mut radeon_cac_tdp_table,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct radeon_dpm_fan {
    pub t_min: u16,
    pub t_med: u16,
    pub t_high: u16,
    pub pwm_min: u16,
    pub pwm_med: u16,
    pub pwm_high: u16,
    pub t_hyst: u8,
    pub cycle_delay: u32,
    pub t_max: u16,
    pub control_mode: u8,
    pub default_max_fan_pwm: u16,
    pub default_fan_output_sensitivity: u16,
    pub fan_output_sensitivity: u16,
    pub ucode_fan_control: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum radeon_pcie_gen {
    RADEON_PCIE_GEN1 = 0,
    RADEON_PCIE_GEN2 = 1,
    RADEON_PCIE_GEN3 = 2,
    RADEON_PCIE_GEN_INVALID = 0xffff
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum radeon_dpm_forced_level {
    RADEON_DPM_FORCED_LEVEL_AUTO = 0,
    RADEON_DPM_FORCED_LEVEL_LOW = 1,
    RADEON_DPM_FORCED_LEVEL_HIGH = 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct radeon_vce_state {
// vce clocks
    pub evclk: u32,
    pub ecclk: u32,
// gpu clocks
    pub sclk: u32,
    pub mclk: u32,
    pub clk_idx: u8,
    pub pstate: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct radeon_dpm {
    pub ps: *mut radeon_ps,
// number of valid power states
    pub num_ps: c_int,
// current power state that is active
    pub current_ps: *mut radeon_ps,
// requested power state
    pub requested_ps: *mut radeon_ps,
// boot up power state
    pub boot_ps: *mut radeon_ps,
// default uvd power state
    pub uvd_ps: *mut radeon_ps,
// vce requirements
    pub vce_states: [radeon_vce_state; RADEON_MAX_VCE_LEVELS],
    pub vce_level: radeon_vce_level,
    pub state: radeon_pm_state_type,
    pub user_state: radeon_pm_state_type,
    pub platform_caps: u32,
    pub voltage_response_time: u32,
    pub backbias_response_time: u32,
    pub priv: *mut c_void,
    pub new_active_crtcs: u32,
    pub new_active_crtc_count: c_int,
    pub high_pixelclock_count: c_int,
    pub current_active_crtcs: u32,
    pub current_active_crtc_count: c_int,
    pub single_display: bool,
    pub dyn_state: radeon_dpm_dynamic_state,
    pub fan: radeon_dpm_fan,
    pub tdp_limit: u32,
    pub near_tdp_limit: u32,
    pub near_tdp_limit_adjusted: u32,
    pub sq_ramping_threshold: u32,
    pub cac_leakage: u32,
    pub tdp_od_limit: u16,
    pub tdp_adjustment: u32,
    pub load_line_slope: u16,
    pub power_control: bool,
    pub ac_power: bool,
// special states active
    pub thermal_active: bool,
    pub uvd_active: bool,
    pub vce_active: bool,
// thermal handling
    pub thermal: radeon_dpm_thermal,
// forced levels
    pub forced_level: radeon_dpm_forced_level,
// track UVD streams
    pub sd: unsigned,
    pub hd: unsigned,
}

extern "C" {
    pub fn radeon_dpm_enable_uvd(rdev: *mut radeon_device, enable: bool);
}
extern "C" {
    pub fn radeon_dpm_enable_vce(rdev: *mut radeon_device, enable: bool);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct radeon_pm {
    pub mutex: mutex,
// write locked while reprogramming mclk
    pub mclk_lock: rw_semaphore,
    pub active_crtcs: u32,
    pub active_crtc_count: c_int,
    pub req_vblank: c_int,
    pub vblank_sync: bool,
    pub max_bandwidth: fixed20_12,
    pub igp_sideport_mclk: fixed20_12,
    pub igp_system_mclk: fixed20_12,
    pub igp_ht_link_clk: fixed20_12,
    pub igp_ht_link_width: fixed20_12,
    pub k8_bandwidth: fixed20_12,
    pub sideport_bandwidth: fixed20_12,
    pub ht_bandwidth: fixed20_12,
    pub core_bandwidth: fixed20_12,
    pub sclk: fixed20_12,
    pub mclk: fixed20_12,
    pub needed_bandwidth: fixed20_12,
    pub power_state: *mut radeon_power_state,
// number of valid power states
    pub num_power_states: c_int,
    pub current_power_state_index: c_int,
    pub current_clock_mode_index: c_int,
    pub requested_power_state_index: c_int,
    pub requested_clock_mode_index: c_int,
    pub default_power_state_index: c_int,
    pub current_sclk: u32,
    pub current_mclk: u32,
    pub current_vddc: u16,
    pub current_vddci: u16,
    pub default_sclk: u32,
    pub default_mclk: u32,
    pub default_vddc: u16,
    pub default_vddci: u16,
    pub i2c_bus: *mut radeon_i2c_chan,
// selected pm method
    pub pm_method: radeon_pm_method,
// dynpm power management
    pub dynpm_idle_work: delayed_work,
    pub dynpm_state: radeon_dynpm_state,
    pub dynpm_planned_action: radeon_dynpm_action,
    pub dynpm_action_timeout: c_ulong,
    pub dynpm_can_upclock: bool,
    pub dynpm_can_downclock: bool,
// profile-based power management
    pub profile: radeon_pm_profile_type,
    pub profile_index: c_int,
    pub profiles: [radeon_pm_profile; PM_PROFILE_MAX],
// internal thermal controller on rv6xx+
    pub int_thermal_type: radeon_int_thermal_type,
    pub int_hwmon_dev: *mut device,
// fan control parameters
    pub no_fan: bool,
    pub fan_pulses_per_revolution: u8,
    pub fan_min_rpm: u8,
    pub fan_max_rpm: u8,
// dpm
    pub dpm_enabled: bool,
    pub sysfs_initialized: bool,
    pub dpm: radeon_dpm,
}

pub const RADEON_PCIE_SPEED_25: c_int = 1;
pub const RADEON_PCIE_SPEED_50: c_int = 2;
pub const RADEON_PCIE_SPEED_80: c_int = 4;
//
// UVD
//
pub const RADEON_DEFAULT_UVD_HANDLES: c_int = 10;
pub const RADEON_MAX_UVD_HANDLES: c_int = 30;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct radeon_uvd {
    pub fw_header_present: bool,
    pub vcpu_bo: *mut radeon_bo,
    pub cpu_addr: *mut c_void,
    pub gpu_addr: u64,
    pub max_handles: unsigned,
    pub handles: [core::sync::atomic::AtomicI32; RADEON_MAX_UVD_HANDLES],
    pub filp: [*mut drm_file; RADEON_MAX_UVD_HANDLES],
    pub img_size: [unsigned; RADEON_MAX_UVD_HANDLES],
    pub idle_work: delayed_work,
}

extern "C" {
    pub fn radeon_uvd_init(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn radeon_uvd_fini(rdev: *mut radeon_device);
}
extern "C" {
    pub fn radeon_uvd_suspend(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn radeon_uvd_resume(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn radeon_uvd_cs_parse(parser: *mut radeon_cs_parser) -> c_int;
}
extern "C" {
    pub fn radeon_uvd_note_usage(rdev: *mut radeon_device);
}
//
// VCE
//
pub const RADEON_MAX_VCE_HANDLES: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct radeon_vce {
    pub vcpu_bo: *mut radeon_bo,
    pub gpu_addr: u64,
    pub fw_version: unsigned,
    pub fb_version: unsigned,
    pub handles: [core::sync::atomic::AtomicI32; RADEON_MAX_VCE_HANDLES],
    pub filp: [*mut drm_file; RADEON_MAX_VCE_HANDLES],
    pub img_size: [unsigned; RADEON_MAX_VCE_HANDLES],
    pub idle_work: delayed_work,
    pub keyselect: u32,
}

extern "C" {
    pub fn radeon_vce_init(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn radeon_vce_fini(rdev: *mut radeon_device);
}
extern "C" {
    pub fn radeon_vce_suspend(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn radeon_vce_resume(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn radeon_vce_free_handles(rdev: *mut radeon_device, filp: *mut drm_file);
}
extern "C" {
    pub fn radeon_vce_note_usage(rdev: *mut radeon_device);
}
extern "C" {
    pub fn radeon_vce_cs_reloc(p: *mut radeon_cs_parser, lo: c_int, hi: c_int, size: unsigned) -> c_int;
}
extern "C" {
    pub fn radeon_vce_cs_parse(p: *mut radeon_cs_parser) -> c_int;
}
extern "C" {
    pub fn radeon_vce_ib_execute(rdev: *mut radeon_device, ib: *mut radeon_ib);
}
extern "C" {
    pub fn radeon_vce_ring_test(rdev: *mut radeon_device, ring: *mut radeon_ring) -> c_int;
}
extern "C" {
    pub fn radeon_vce_ib_test(rdev: *mut radeon_device, ring: *mut radeon_ring) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct r600_audio_pin {
    pub channels: c_int,
    pub rate: c_int,
    pub bits_per_sample: c_int,
    pub status_bits: u8,
    pub category_code: u8,
    pub offset: u32,
    pub connected: bool,
    pub id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct r600_audio {
    pub enabled: bool,
    pub pin: [r600_audio_pin; RADEON_MAX_AFMT_BLOCKS],
    pub num_pins: c_int,
    pub hdmi_funcs: *mut radeon_audio_funcs,
    pub dp_funcs: *mut radeon_audio_funcs,
    pub funcs: *mut radeon_audio_basic_funcs,
    pub component: *mut drm_audio_component,
    pub component_registered: bool,
    pub component_mutex: mutex,
}

//
// Benchmarking
//
extern "C" {
    pub fn radeon_benchmark(rdev: *mut radeon_device, test_number: c_int);
}
//
// Testing
//
extern "C" {
    pub fn radeon_test_moves(rdev: *mut radeon_device);
}
extern "C" {
    pub fn radeon_test_syncing(rdev: *mut radeon_device);
}
//
// MMU Notifier
//

extern "C" {
    pub fn radeon_mn_register(bo: *mut radeon_bo, addr: c_ulong) -> c_int;
}
extern "C" {
    pub fn radeon_mn_unregister(bo: *mut radeon_bo);
}

//
// Debugfs
//
extern "C" {
    pub fn radeon_debugfs_fence_init(rdev: *mut radeon_device);
}
extern "C" {
    pub fn radeon_gem_debugfs_init(rdev: *mut radeon_device);
}
//
// ASIC ring specific functions.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct radeon_asic_ring {
// ring read/write ptr handling
    pub ring): *mut *mut *mut u32 (get_rptr)(struct radeon_device rdev, struct radeon_ring,
    pub ring): *mut *mut *mut u32 (get_wptr)(struct radeon_device rdev, struct radeon_ring,
    pub ring): *mut *mut *mut void (set_wptr)(struct radeon_device rdev, struct radeon_ring,
// validating and patching of IBs
    pub ib): *mut *mut *mut int (ib_parse)(struct radeon_device rdev, struct radeon_ib,
    pub p): *mut *mut int (cs_parse)(struct radeon_cs_parser,
// command emmit functions
    pub ib): *mut *mut *mut void (ib_execute)(struct radeon_device rdev, struct radeon_ib,
    pub fence): *mut *mut *mut void (emit_fence)(struct radeon_device rdev, struct radeon_fence,
    pub ring): *mut *mut *mut void (hdp_flush)(struct radeon_device rdev, struct radeon_ring,
    pub emit_wait): *mut *mut radeon_semaphore semaphore, bool,
    pub pd_addr): unsigned vm_id, uint64_t,
// testing functions
    pub cp): *mut *mut *mut int (ring_test)(struct radeon_device rdev, struct radeon_ring,
    pub cp): *mut *mut *mut int (ib_test)(struct radeon_device rdev, struct radeon_ring,
    pub cp): *mut *mut *mut bool (is_lockup)(struct radeon_device rdev, struct radeon_ring,
// deprecated
    pub cp): *mut *mut *mut void (ring_start)(struct radeon_device rdev, struct radeon_ring,
}

//
// ASIC specific functions.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct radeon_asic {
    pub rdev): *mut *mut int (init)(struct radeon_device,
    pub rdev): *mut *mut void (fini)(struct radeon_device,
    pub rdev): *mut *mut int (resume)(struct radeon_device,
    pub rdev): *mut *mut int (suspend)(struct radeon_device,
    pub state): *mut *mut *mut void (vga_set_state)(struct radeon_device rdev, bool,
    pub hard): *mut *mut *mut int (asic_reset)(struct radeon_device rdev, bool,
// Flush the HDP cache via MMIO
    pub rdev): *mut *mut void (mmio_hdp_flush)(struct radeon_device,
// check if 3D engine is idle
    pub rdev): *mut *mut bool (gui_idle)(struct radeon_device,
// wait for mc_idle
    pub rdev): *mut *mut int (mc_wait_for_idle)(struct radeon_device,
// get the reference clock
    pub rdev): *mut *mut u32 (get_xclk)(struct radeon_device,
// get the gpu clock counter
    pub rdev): *mut *mut uint64_t (get_gpu_clock_counter)(struct radeon_device,
// get register for info ioctl
    pub val): *mut *mut *mut int (get_allowed_info_register)(struct radeon_device rdev, u32 reg, u32,
// gart
    pub rdev): *mut *mut void (tlb_flush)(struct radeon_device,
    pub flags): *mut *mut uint64_t (get_page_entry)(uint64_t addr, uint32_t,
    pub entry): u64,
    pub gart: },
    pub rdev): *mut *mut int (init)(struct radeon_device,
    pub rdev): *mut *mut void (fini)(struct radeon_device,
    pub count): unsigned,
    pub flags): uint32_t incr, uint32_t,
    pub flags): uint32_t incr, uint32_t,
    pub ib): *mut *mut void (pad_ib)(struct radeon_ib,
    pub vm: },
// ring specific callbacks
    pub ring: [*const radeon_asic_ring; RADEON_NUM_RINGS],
// irqs
    pub rdev): *mut *mut int (set)(struct radeon_device,
    pub rdev): *mut *mut int (process)(struct radeon_device,
    pub irq: },
// displays
// display watermarks
    pub rdev): *mut *mut void (bandwidth_update)(struct radeon_device,
// get frame count
    pub crtc): *mut *mut *mut u32 (get_vblank_counter)(struct radeon_device rdev, int,
// wait for vblank
    pub crtc): *mut *mut *mut void (wait_for_vblank)(struct radeon_device rdev, int,
// set backlight level
    pub level): *mut *mut *mut void (set_backlight_level)(struct radeon_encoder radeon_encoder, u8,
// get backlight level
    pub radeon_encoder): *mut *mut u8 (get_backlight_level)(struct radeon_encoder,
// audio callbacks
    pub enable): *mut *mut *mut void (hdmi_enable)(struct drm_encoder encoder, bool,
    pub mode): *mut *mut *mut void (hdmi_setmode)(struct drm_encoder encoder, struct drm_display_mode,
    pub display: },
// copy functions for bo handling
    pub resv): *mut dma_resv,
    pub blit_ring_index: u32,
    pub resv): *mut dma_resv,
    pub dma_ring_index: u32,
// method used for bo copy
    pub resv): *mut dma_resv,
// ring used for bo copies
    pub copy_ring_index: u32,
    pub copy: },
// surfaces
    pub obj_size): uint32_t offset, uint32_t,
    pub reg): *mut *mut *mut void (clear_reg)(struct radeon_device rdev, int,
    pub surface: },
// hotplug detect
    pub rdev): *mut *mut void (init)(struct radeon_device,
    pub rdev): *mut *mut void (fini)(struct radeon_device,
    pub hpd): *mut *mut *mut bool (sense)(struct radeon_device rdev, enum radeon_hpd_id,
    pub hpd): *mut *mut *mut void (set_polarity)(struct radeon_device rdev, enum radeon_hpd_id,
    pub hpd: },
// static power management
    pub rdev): *mut *mut void (misc)(struct radeon_device,
    pub rdev): *mut *mut void (prepare)(struct radeon_device,
    pub rdev): *mut *mut void (finish)(struct radeon_device,
    pub rdev): *mut *mut void (init_profile)(struct radeon_device,
    pub rdev): *mut *mut void (get_dynpm_state)(struct radeon_device,
    pub rdev): *mut *mut uint32_t (get_engine_clock)(struct radeon_device,
    pub eng_clock): *mut *mut *mut void (set_engine_clock)(struct radeon_device rdev, uint32_t,
    pub rdev): *mut *mut uint32_t (get_memory_clock)(struct radeon_device,
    pub mem_clock): *mut *mut *mut void (set_memory_clock)(struct radeon_device rdev, uint32_t,
    pub rdev): *mut *mut int (get_pcie_lanes)(struct radeon_device,
    pub lanes): *mut *mut *mut void (set_pcie_lanes)(struct radeon_device rdev, int,
    pub enable): *mut *mut *mut void (set_clock_gating)(struct radeon_device rdev, int,
    pub dclk): *mut *mut *mut int (set_uvd_clocks)(struct radeon_device rdev, u32 vclk, u32,
    pub ecclk): *mut *mut *mut int (set_vce_clocks)(struct radeon_device rdev, u32 evclk, u32,
    pub rdev): *mut *mut int (get_temperature)(struct radeon_device,
    pub pm: },
// dynamic power management
    pub rdev): *mut *mut int (init)(struct radeon_device,
    pub rdev): *mut *mut void (setup_asic)(struct radeon_device,
    pub rdev): *mut *mut int (enable)(struct radeon_device,
    pub rdev): *mut *mut int (late_enable)(struct radeon_device,
    pub rdev): *mut *mut void (disable)(struct radeon_device,
    pub rdev): *mut *mut int (pre_set_power_state)(struct radeon_device,
    pub rdev): *mut *mut int (set_power_state)(struct radeon_device,
    pub rdev): *mut *mut void (post_set_power_state)(struct radeon_device,
    pub rdev): *mut *mut void (display_configuration_changed)(struct radeon_device,
    pub rdev): *mut *mut void (fini)(struct radeon_device,
    pub low): *mut *mut *mut u32 (get_sclk)(struct radeon_device rdev, bool,
    pub low): *mut *mut *mut u32 (get_mclk)(struct radeon_device rdev, bool,
    pub ps): *mut *mut *mut void (print_power_state)(struct radeon_device rdev, struct radeon_ps,
    pub m): *mut *mut *mut void (debugfs_print_current_performance_level)(struct radeon_device rdev, struct seq_file,
    pub level): *mut *mut *mut int (force_performance_level)(struct radeon_device rdev, enum radeon_dpm_forced_level,
    pub rdev): *mut *mut bool (vblank_too_short)(struct radeon_device,
    pub gate): *mut *mut *mut void (powergate_uvd)(struct radeon_device rdev, bool,
    pub enable): *mut *mut *mut void (enable_bapm)(struct radeon_device rdev, bool,
    pub mode): *mut *mut *mut void (fan_ctrl_set_mode)(struct radeon_device rdev, u32,
    pub rdev): *mut *mut u32 (fan_ctrl_get_mode)(struct radeon_device,
    pub speed): *mut *mut *mut int (set_fan_speed_percent)(struct radeon_device rdev, u32,
    pub speed): *mut *mut *mut int (get_fan_speed_percent)(struct radeon_device rdev, u32,
    pub rdev): *mut *mut u32 (get_current_sclk)(struct radeon_device,
    pub rdev): *mut *mut u32 (get_current_mclk)(struct radeon_device,
    pub rdev): *mut *mut u16 (get_current_vddc)(struct radeon_device,
    pub dpm: },
// pageflipping
    pub async): *mut *mut *mut void (page_flip)(struct radeon_device rdev, int crtc, u64 crtc_base, bool,
    pub crtc): *mut *mut *mut bool (page_flip_pending)(struct radeon_device rdev, int,
    pub pflip: },
}

//
// Asic structures
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct r100_asic {
    pub reg_safe_bm: *const unsigned,
    pub reg_safe_bm_size: unsigned,
    pub hdp_cntl: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct r300_asic {
    pub reg_safe_bm: *const unsigned,
    pub reg_safe_bm_size: unsigned,
    pub resync_scratch: u32,
    pub hdp_cntl: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct r600_asic {
    pub max_pipes: unsigned,
    pub max_tile_pipes: unsigned,
    pub max_simds: unsigned,
    pub max_backends: unsigned,
    pub max_gprs: unsigned,
    pub max_threads: unsigned,
    pub max_stack_entries: unsigned,
    pub max_hw_contexts: unsigned,
    pub max_gs_threads: unsigned,
    pub sx_max_export_size: unsigned,
    pub sx_max_export_pos_size: unsigned,
    pub sx_max_export_smx_size: unsigned,
    pub sq_num_cf_insts: unsigned,
    pub tiling_nbanks: unsigned,
    pub tiling_npipes: unsigned,
    pub tiling_group_size: unsigned,
    pub tile_config: unsigned,
    pub backend_map: unsigned,
    pub active_simds: unsigned,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rv770_asic {
    pub max_pipes: unsigned,
    pub max_tile_pipes: unsigned,
    pub max_simds: unsigned,
    pub max_backends: unsigned,
    pub max_gprs: unsigned,
    pub max_threads: unsigned,
    pub max_stack_entries: unsigned,
    pub max_hw_contexts: unsigned,
    pub max_gs_threads: unsigned,
    pub sx_max_export_size: unsigned,
    pub sx_max_export_pos_size: unsigned,
    pub sx_max_export_smx_size: unsigned,
    pub sq_num_cf_insts: unsigned,
    pub sx_num_of_sets: unsigned,
    pub sc_prim_fifo_size: unsigned,
    pub sc_hiz_tile_fifo_size: unsigned,
    pub sc_earlyz_tile_fifo_fize: unsigned,
    pub tiling_nbanks: unsigned,
    pub tiling_npipes: unsigned,
    pub tiling_group_size: unsigned,
    pub tile_config: unsigned,
    pub backend_map: unsigned,
    pub active_simds: unsigned,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct evergreen_asic {
    pub num_ses: unsigned,
    pub max_pipes: unsigned,
    pub max_tile_pipes: unsigned,
    pub max_simds: unsigned,
    pub max_backends: unsigned,
    pub max_gprs: unsigned,
    pub max_threads: unsigned,
    pub max_stack_entries: unsigned,
    pub max_hw_contexts: unsigned,
    pub max_gs_threads: unsigned,
    pub sx_max_export_size: unsigned,
    pub sx_max_export_pos_size: unsigned,
    pub sx_max_export_smx_size: unsigned,
    pub sq_num_cf_insts: unsigned,
    pub sx_num_of_sets: unsigned,
    pub sc_prim_fifo_size: unsigned,
    pub sc_hiz_tile_fifo_size: unsigned,
    pub sc_earlyz_tile_fifo_size: unsigned,
    pub tiling_nbanks: unsigned,
    pub tiling_npipes: unsigned,
    pub tiling_group_size: unsigned,
    pub tile_config: unsigned,
    pub backend_map: unsigned,
    pub active_simds: unsigned,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cayman_asic {
    pub max_shader_engines: unsigned,
    pub max_pipes_per_simd: unsigned,
    pub max_tile_pipes: unsigned,
    pub max_simds_per_se: unsigned,
    pub max_backends_per_se: unsigned,
    pub max_texture_channel_caches: unsigned,
    pub max_gprs: unsigned,
    pub max_threads: unsigned,
    pub max_gs_threads: unsigned,
    pub max_stack_entries: unsigned,
    pub sx_num_of_sets: unsigned,
    pub sx_max_export_size: unsigned,
    pub sx_max_export_pos_size: unsigned,
    pub sx_max_export_smx_size: unsigned,
    pub max_hw_contexts: unsigned,
    pub sq_num_cf_insts: unsigned,
    pub sc_prim_fifo_size: unsigned,
    pub sc_hiz_tile_fifo_size: unsigned,
    pub sc_earlyz_tile_fifo_size: unsigned,
    pub num_shader_engines: unsigned,
    pub num_shader_pipes_per_simd: unsigned,
    pub num_tile_pipes: unsigned,
    pub num_simds_per_se: unsigned,
    pub num_backends_per_se: unsigned,
    pub backend_disable_mask_per_asic: unsigned,
    pub backend_map: unsigned,
    pub num_texture_channel_caches: unsigned,
    pub mem_max_burst_length_bytes: unsigned,
    pub mem_row_size_in_kb: unsigned,
    pub shader_engine_tile_size: unsigned,
    pub num_gpus: unsigned,
    pub multi_gpu_tile_size: unsigned,
    pub tile_config: unsigned,
    pub active_simds: unsigned,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct si_asic {
    pub max_shader_engines: unsigned,
    pub max_tile_pipes: unsigned,
    pub max_cu_per_sh: unsigned,
    pub max_sh_per_se: unsigned,
    pub max_backends_per_se: unsigned,
    pub max_texture_channel_caches: unsigned,
    pub max_gprs: unsigned,
    pub max_gs_threads: unsigned,
    pub max_hw_contexts: unsigned,
    pub sc_prim_fifo_size_frontend: unsigned,
    pub sc_prim_fifo_size_backend: unsigned,
    pub sc_hiz_tile_fifo_size: unsigned,
    pub sc_earlyz_tile_fifo_size: unsigned,
    pub num_tile_pipes: unsigned,
    pub backend_enable_mask: unsigned,
    pub backend_disable_mask_per_asic: unsigned,
    pub backend_map: unsigned,
    pub num_texture_channel_caches: unsigned,
    pub mem_max_burst_length_bytes: unsigned,
    pub mem_row_size_in_kb: unsigned,
    pub shader_engine_tile_size: unsigned,
    pub num_gpus: unsigned,
    pub multi_gpu_tile_size: unsigned,
    pub tile_config: unsigned,
    pub tile_mode_array: [u32; 32],
    pub active_cus: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cik_asic {
    pub max_shader_engines: unsigned,
    pub max_tile_pipes: unsigned,
    pub max_cu_per_sh: unsigned,
    pub max_sh_per_se: unsigned,
    pub max_backends_per_se: unsigned,
    pub max_texture_channel_caches: unsigned,
    pub max_gprs: unsigned,
    pub max_gs_threads: unsigned,
    pub max_hw_contexts: unsigned,
    pub sc_prim_fifo_size_frontend: unsigned,
    pub sc_prim_fifo_size_backend: unsigned,
    pub sc_hiz_tile_fifo_size: unsigned,
    pub sc_earlyz_tile_fifo_size: unsigned,
    pub num_tile_pipes: unsigned,
    pub backend_enable_mask: unsigned,
    pub backend_disable_mask_per_asic: unsigned,
    pub backend_map: unsigned,
    pub num_texture_channel_caches: unsigned,
    pub mem_max_burst_length_bytes: unsigned,
    pub mem_row_size_in_kb: unsigned,
    pub shader_engine_tile_size: unsigned,
    pub num_gpus: unsigned,
    pub multi_gpu_tile_size: unsigned,
    pub tile_config: unsigned,
    pub tile_mode_array: [u32; 32],
    pub macrotile_mode_array: [u32; 16],
    pub active_cus: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union radeon_asic_config {
    pub r300: r300_asic,
    pub r100: r100_asic,
    pub r600: r600_asic,
    pub rv770: rv770_asic,
    pub evergreen: evergreen_asic,
    pub cayman: cayman_asic,
    pub si: si_asic,
    pub cik: cik_asic,
}

//
// asic initizalization from radeon_asic.c
//
extern "C" {
    pub fn radeon_agp_disable(rdev: *mut radeon_device);
}
extern "C" {
    pub fn radeon_asic_init(rdev: *mut radeon_device) -> c_int;
}
//
// IOCTL.
//
extern "C" {
    pub fn radeon_cs_ioctl(dev: *mut drm_device, data: *mut c_void, filp: *mut drm_file) -> c_int;
}
extern "C" {
    pub fn radeon_info_ioctl(dev: *mut drm_device, data: *mut c_void, filp: *mut drm_file) -> c_int;
}
// VRAM scratch page for HDP bug, default vram page
#[repr(C)]
#[derive(Copy, Clone)]
pub struct r600_vram_scratch {
    pub robj: *mut radeon_bo,
    pub ptr: *mut volatile uint32_t,
    pub gpu_addr: u64,
}

//
// ACPI
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct radeon_atif_notification_cfg {
    pub enabled: bool,
    pub command_code: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct radeon_atif_notifications {
    pub display_switch: bool,
    pub expansion_mode_change: bool,
    pub thermal_state: bool,
    pub forced_power_state: bool,
    pub system_power_state: bool,
    pub display_conf_change: bool,
    pub px_gfx_switch: bool,
    pub brightness_change: bool,
    pub dgpu_display_event: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct radeon_atif_functions {
    pub system_params: bool,
    pub sbios_requests: bool,
    pub select_active_disp: bool,
    pub lid_state: bool,
    pub get_tv_standard: bool,
    pub set_tv_standard: bool,
    pub get_panel_expansion_mode: bool,
    pub set_panel_expansion_mode: bool,
    pub temperature_change: bool,
    pub graphics_device_types: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct radeon_atif {
    pub notifications: radeon_atif_notifications,
    pub functions: radeon_atif_functions,
    pub notification_cfg: radeon_atif_notification_cfg,
    pub encoder_for_bl: *mut radeon_encoder,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct radeon_atcs_functions {
    pub get_ext_state: bool,
    pub pcie_perf_req: bool,
    pub pcie_dev_rdy: bool,
    pub pcie_bus_width: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct radeon_atcs {
    pub functions: radeon_atcs_functions,
}

//
// Core structure, functions and helpers.
//
extern "C" {
    pub fn uint32_t(radeon_device*: *mut *mut radeon_rreg_t)(struct, _arg: u32) -> typedef;
}
extern "C" {
    pub fn void(radeon_device*: *mut *mut radeon_wreg_t)(struct, _arg: u32, _arg: u32) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct radeon_device {
    pub dev: *mut device,
    pub ddev: drm_device,
    pub pdev: *mut pci_dev,

    pub hose: *mut pci_controller,

    pub agp: *mut radeon_agp_head,
    pub exclusive_lock: rw_semaphore,
// ASIC
    pub config: radeon_asic_config,
    pub family: radeon_family,
    pub flags: c_ulong,
    pub usec_timeout: c_int,
    pub pll_errata: radeon_pll_errata,
    pub num_gb_pipes: c_int,
    pub num_z_pipes: c_int,
    pub disp_priority: c_int,
// BIOS
    pub bios: *mut u8,
    pub is_atom_bios: bool,
    pub bios_header_start: u16,
    pub stolen_vga_memory: *mut radeon_bo,
// Register mmio
    pub rmmio_base: resource_size_t,
    pub rmmio_size: resource_size_t,
// protects concurrent MM_INDEX/DATA based register access
    pub mmio_idx_lock: spinlock_t,
// protects concurrent SMC based register access
    pub smc_idx_lock: spinlock_t,
// protects concurrent PLL register access
    pub pll_idx_lock: spinlock_t,
// protects concurrent MC register access
    pub mc_idx_lock: spinlock_t,
// protects concurrent PCIE register access
    pub pcie_idx_lock: spinlock_t,
// protects concurrent PCIE_PORT register access
    pub pciep_idx_lock: spinlock_t,
// protects concurrent PIF register access
    pub pif_idx_lock: spinlock_t,
// protects concurrent CG register access
    pub cg_idx_lock: spinlock_t,
// protects concurrent UVD register access
    pub uvd_idx_lock: spinlock_t,
// protects concurrent RCU register access
    pub rcu_idx_lock: spinlock_t,
// protects concurrent DIDT register access
    pub didt_idx_lock: spinlock_t,
// protects concurrent ENDPOINT (audio) register access
    pub end_idx_lock: spinlock_t,
    pub rmmio: *mut void __iomem,
    pub mc_rreg: radeon_rreg_t,
    pub mc_wreg: radeon_wreg_t,
    pub pll_rreg: radeon_rreg_t,
    pub pll_wreg: radeon_wreg_t,
    pub pcie_reg_mask: u32,
    pub pciep_rreg: radeon_rreg_t,
    pub pciep_wreg: radeon_wreg_t,
// io port
    pub rio_mem: *mut void __iomem,
    pub rio_mem_size: resource_size_t,
    pub clock: radeon_clock,
    pub mc: radeon_mc,
    pub gart: radeon_gart,
    pub mode_info: radeon_mode_info,
    pub scratch: radeon_scratch,
    pub doorbell: radeon_doorbell,
    pub mman: radeon_mman,
    pub fence_drv: [radeon_fence_driver; RADEON_NUM_RINGS],
    pub fence_queue: wait_queue_head_t,
    pub fence_context: u64,
    pub ring_lock: mutex,
    pub ring: [radeon_ring; RADEON_NUM_RINGS],
    pub ib_pool_ready: bool,
    pub ring_tmp_bo: radeon_sa_manager,
    pub irq: radeon_irq,
    pub asic: *mut radeon_asic,
    pub gem: radeon_gem,
    pub pm: radeon_pm,
    pub uvd: radeon_uvd,
    pub vce: radeon_vce,
    pub bios_scratch: [u32; RADEON_BIOS_NUM_SCRATCH],
    pub wb: radeon_wb,
    pub dummy_page: radeon_dummy_page,
    pub shutdown: bool,
    pub need_swiotlb: bool,
    pub accel_working: bool,
    pub feature*/: *mut *mut bool fastfb_working; / IGP,
    pub in_reset: bool needs_reset,,
    pub surface_regs: [radeon_surface_reg; RADEON_GEM_MAX_SURFACES],
    pub /: *const *const *const firmware me_fw; / all family ME firmware,
    pub /: *const *const *const firmware pfp_fw; / r6/700 PFP firmware,
    pub /: *const *const *const firmware rlc_fw; / r6/700 RLC firmware,
    pub /: *const *const *const firmware mc_fw; / NI MC firmware,
    pub /: *const *const *const firmware ce_fw; / SI CE firmware,
    pub /: *const *const *const firmware mec_fw; / CIK MEC firmware,
    pub /: *const *const *const firmware mec2_fw; / KV MEC2 firmware,
    pub /: *const *const *const firmware sdma_fw; / CIK SDMA firmware,
    pub /: *const *const *const firmware smc_fw; / SMC firmware,
    pub /: *const *const *const firmware uvd_fw; / UVD firmware,
    pub /: *const *const *const firmware vce_fw; / VCE firmware,
    pub new_fw: bool,
    pub vram_scratch: r600_vram_scratch,
    pub /: *mut *mut int msi_enabled; / msi enabled,
    pub /: *mut *mut r600_ih ih; / r6/700 interrupt ring,
    pub rlc: radeon_rlc,
    pub mec: radeon_mec,
    pub hotplug_work: delayed_work,
    pub dp_work: work_struct,
    pub audio_work: work_struct,
    pub /: *mut *mut int num_crtc; / number of crtcs,
    pub /: *mut *mut mutex dc_hw_i2c_mutex; / display controller hw i2c mutex,
    pub has_uvd: bool,
    pub has_vce: bool,
    pub /: *mut *mut r600_audio audio; / audio stuff,
    pub acpi_nb: notifier_block,
// only one userspace can use Hyperz features or CMASK at a time
    pub hyperz_filp: *mut drm_file,
    pub cmask_filp: *mut drm_file,
// i2c buses
    pub i2c_bus: [*mut radeon_i2c_chan; RADEON_MAX_I2C_BUS],
// virtual memory
    pub vm_manager: radeon_vm_manager,
    pub gpu_clock_mutex: mutex,
// memory stats
    pub num_bytes_moved: core::sync::atomic::AtomicI64,
    pub gpu_reset_counter: core::sync::atomic::AtomicI32,
// ACPI interface
    pub atif: radeon_atif,
    pub atcs: radeon_atcs,
// srbm instance registers
    pub srbm_mutex: mutex,
// clock, powergating flags
    pub cg_flags: u32,
    pub pg_flags: u32,
    pub vga_pm_domain: dev_pm_domain,
    pub have_disp_power_ref: bool,
    pub px_quirk_flags: u32,
// tracking pinned memory
    pub vram_pin_size: u64,
    pub gart_pin_size: u64,
}

extern "C" {
    pub fn radeon_is_px(dev: *mut drm_device) -> bool;
}
extern "C" {
    pub fn radeon_device_fini(rdev: *mut radeon_device);
}
extern "C" {
    pub fn radeon_gpu_wait_for_idle(rdev: *mut radeon_device) -> c_int;
}
pub const RADEON_MIN_MMIO_SIZE: c_uint = 0x10000;
extern "C" {
    pub fn r100_mm_rreg_slow(rdev: *mut radeon_device, reg: u32) -> u32;
}
extern "C" {
    pub fn r100_mm_wreg_slow(rdev: *mut radeon_device, reg: u32, v: u32);
}
// The mmio size is 64kb at minimum. Allows the if to be optimized out.
extern "C" {
    pub fn readl(reg: *mut *mut ((void __iomem )rdev->rmmio) +) -> return;
}
extern "C" {
    pub fn r100_mm_rreg_slow(_arg: rdev, _arg: reg) -> return;
}
extern "C" {
    pub fn r100_io_rreg(rdev: *mut radeon_device, reg: u32) -> u32;
}
extern "C" {
    pub fn r100_io_wreg(rdev: *mut radeon_device, reg: u32, v: u32);
}
extern "C" {
    pub fn cik_mm_rdoorbell(rdev: *mut radeon_device, index: u32) -> u32;
}
extern "C" {
    pub fn cik_mm_wdoorbell(rdev: *mut radeon_device, index: u32, v: u32);
}
//
// Cast helper
//
// Registers read & write functions.
//

//
// Indirect registers accessors.
// They used to be inlined, but this increases code size by ~65 kbytes.
// Since each performs a pair of MMIO ops
// within a spin_lock_irqsave/spin_unlock_irqrestore region,
// the cost of call+ret is almost negligible. MMIO and locking
// costs several dozens of cycles each at best, call+ret is ~5 cycles.
//
extern "C" {
    pub fn rv370_pcie_rreg(rdev: *mut radeon_device, reg: u32) -> u32;
}
extern "C" {
    pub fn rv370_pcie_wreg(rdev: *mut radeon_device, reg: u32, v: u32);
}
extern "C" {
    pub fn tn_smc_rreg(rdev: *mut radeon_device, reg: u32) -> u32;
}
extern "C" {
    pub fn tn_smc_wreg(rdev: *mut radeon_device, reg: u32, v: u32);
}
extern "C" {
    pub fn r600_rcu_rreg(rdev: *mut radeon_device, reg: u32) -> u32;
}
extern "C" {
    pub fn r600_rcu_wreg(rdev: *mut radeon_device, reg: u32, v: u32);
}
extern "C" {
    pub fn eg_cg_rreg(rdev: *mut radeon_device, reg: u32) -> u32;
}
extern "C" {
    pub fn eg_cg_wreg(rdev: *mut radeon_device, reg: u32, v: u32);
}
extern "C" {
    pub fn eg_pif_phy0_rreg(rdev: *mut radeon_device, reg: u32) -> u32;
}
extern "C" {
    pub fn eg_pif_phy0_wreg(rdev: *mut radeon_device, reg: u32, v: u32);
}
extern "C" {
    pub fn eg_pif_phy1_rreg(rdev: *mut radeon_device, reg: u32) -> u32;
}
extern "C" {
    pub fn eg_pif_phy1_wreg(rdev: *mut radeon_device, reg: u32, v: u32);
}
extern "C" {
    pub fn r600_uvd_ctx_rreg(rdev: *mut radeon_device, reg: u32) -> u32;
}
extern "C" {
    pub fn r600_uvd_ctx_wreg(rdev: *mut radeon_device, reg: u32, v: u32);
}
extern "C" {
    pub fn cik_didt_rreg(rdev: *mut radeon_device, reg: u32) -> u32;
}
extern "C" {
    pub fn cik_didt_wreg(rdev: *mut radeon_device, reg: u32, v: u32);
}
extern "C" {
    pub fn r100_pll_errata_after_index(rdev: *mut radeon_device);
}
//
// ASICs helpers.
//

//
// BIOS helpers.
//

extern "C" {
    pub fn radeon_combios_init(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn radeon_combios_fini(rdev: *mut radeon_device);
}
extern "C" {
    pub fn radeon_atombios_init(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn radeon_atombios_fini(rdev: *mut radeon_device);
}
//
// RING helpers.
//
// radeon_ring_write - write a value to the ring
//
// @ring: radeon_ring structure holding ring information
// @v: dword (dw) value to write
//
// Write a value to the requested ring buffer (all asics).
//
// ASICs macro.
//

// Common functions
// AGP
extern "C" {
    pub fn radeon_gpu_reset(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn radeon_pci_config_reset(rdev: *mut radeon_device);
}
extern "C" {
    pub fn r600_set_bios_scratch_engine_hung(rdev: *mut radeon_device, hung: bool);
}
extern "C" {
    pub fn radeon_agp_disable(rdev: *mut radeon_device);
}
extern "C" {
    pub fn radeon_modeset_init(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn radeon_modeset_fini(rdev: *mut radeon_device);
}
extern "C" {
    pub fn radeon_card_posted(rdev: *mut radeon_device) -> bool;
}
extern "C" {
    pub fn radeon_update_bandwidth_info(rdev: *mut radeon_device);
}
extern "C" {
    pub fn radeon_update_display_priority(rdev: *mut radeon_device);
}
extern "C" {
    pub fn radeon_boot_test_post_card(rdev: *mut radeon_device) -> bool;
}
extern "C" {
    pub fn radeon_scratch_init(rdev: *mut radeon_device);
}
extern "C" {
    pub fn radeon_wb_fini(rdev: *mut radeon_device);
}
extern "C" {
    pub fn radeon_wb_init(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn radeon_wb_disable(rdev: *mut radeon_device);
}
extern "C" {
    pub fn radeon_surface_init(rdev: *mut radeon_device);
}
extern "C" {
    pub fn radeon_cs_parser_init(p: *mut radeon_cs_parser, data: *mut c_void) -> c_int;
}
extern "C" {
    pub fn radeon_legacy_set_clock_gating(rdev: *mut radeon_device, enable: c_int);
}
extern "C" {
    pub fn radeon_atom_set_clock_gating(rdev: *mut radeon_device, enable: c_int);
}
extern "C" {
    pub fn radeon_ttm_placement_from_domain(rbo: *mut radeon_bo, domain: u32);
}
extern "C" {
    pub fn radeon_ttm_bo_is_radeon_bo(bo: *mut ttm_buffer_object) -> bool;
}
extern "C" {
    pub fn radeon_ttm_tt_has_userptr(rdev: *mut radeon_device, ttm: *mut ttm_tt) -> bool;
}
extern "C" {
    pub fn radeon_ttm_tt_is_readonly(rdev: *mut radeon_device, ttm: *mut ttm_tt) -> bool;
}
extern "C" {
    pub fn radeon_ttm_tt_is_bound(bdev: *mut ttm_device, ttm: *mut ttm_tt) -> bool;
}
extern "C" {
    pub fn radeon_vram_location(rdev: *mut radeon_device, mc: *mut radeon_mc, base: u64);
}
extern "C" {
    pub fn radeon_gtt_location(rdev: *mut radeon_device, mc: *mut radeon_mc);
}
extern "C" {
    pub fn radeon_resume_kms(dev: *mut drm_device, resume: bool, fbcon: bool) -> c_int;
}
extern "C" {
    pub fn radeon_ttm_set_active_vram_size(rdev: *mut radeon_device, size: u64);
}
// KMS
extern "C" {
    pub fn radeon_get_vblank_counter_kms(crtc: *mut drm_crtc) -> u32;
}
extern "C" {
    pub fn radeon_enable_vblank_kms(crtc: *mut drm_crtc) -> c_int;
}
extern "C" {
    pub fn radeon_disable_vblank_kms(crtc: *mut drm_crtc);
}
//
// vm
//
extern "C" {
    pub fn radeon_vm_manager_init(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn radeon_vm_manager_fini(rdev: *mut radeon_device);
}
extern "C" {
    pub fn radeon_vm_init(rdev: *mut radeon_device, vm: *mut radeon_vm) -> c_int;
}
extern "C" {
    pub fn radeon_vm_fini(rdev: *mut radeon_device, vm: *mut radeon_vm);
}
extern "C" {
    pub fn radeon_vm_map_gart(rdev: *mut radeon_device, addr: u64) -> u64;
}
// audio
extern "C" {
    pub fn r600_audio_update_hdmi(work: *mut work_struct);
}
//
// R600 vram scratch functions
//
extern "C" {
    pub fn r600_vram_scratch_init(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn r600_vram_scratch_fini(rdev: *mut radeon_device);
}
//
// r600 cs checking helper
//
extern "C" {
    pub fn r600_mip_minify(size: unsigned, level: unsigned) -> unsigned;
}
extern "C" {
    pub fn r600_fmt_is_valid_color(format: u32) -> bool;
}
extern "C" {
    pub fn r600_fmt_is_valid_texture(format: u32, family: radeon_family) -> bool;
}
extern "C" {
    pub fn r600_fmt_get_blocksize(format: u32) -> c_int;
}
extern "C" {
    pub fn r600_fmt_get_nblocksx(format: u32, w: u32) -> c_int;
}
extern "C" {
    pub fn r600_fmt_get_nblocksy(format: u32, h: u32) -> c_int;
}
//
// r600 functions used by radeon_encoder.c
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct radeon_hdmi_acr {
    pub clock: u32,
    pub n_32khz: c_int,
    pub cts_32khz: c_int,
    pub n_44_1khz: c_int,
    pub cts_44_1khz: c_int,
    pub n_48khz: c_int,
    pub cts_48khz: c_int,
}

//
// evergreen functions used by radeon_encoder.c
//
extern "C" {
    pub fn ni_init_microcode(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn ni_mc_load_microcode(rdev: *mut radeon_device) -> c_int;
}
// radeon_acpi.c

extern "C" {
    pub fn radeon_acpi_init(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn radeon_acpi_fini(rdev: *mut radeon_device);
}
extern "C" {
    pub fn radeon_acpi_is_pcie_performance_request_supported(rdev: *mut radeon_device) -> bool;
}
extern "C" {
    pub fn radeon_acpi_pcie_notify_device_ready(rdev: *mut radeon_device) -> c_int;
}

extern "C" {
    pub fn radeon_cs_packet_next_is_pkt3_nop(p: *mut radeon_cs_parser) -> bool;
}
// interrupt control register helpers
// Audio component binding
extern "C" {
    pub fn radeon_audio_component_init(rdev: *mut radeon_device);
}
extern "C" {
    pub fn radeon_audio_component_fini(rdev: *mut radeon_device);
}

