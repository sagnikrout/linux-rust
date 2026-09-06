//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_device_types.h
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
// Copyright © 2022-2023 Intel Corporation
//

// Macro flag: #define TEST_VM_OPS_ERROR

//
// enum xe_wedged_mode - possible wedged modes
// @XE_WEDGED_MODE_NEVER: Device will never be declared wedged.
// @XE_WEDGED_MODE_UPON_CRITICAL_ERROR: Device will be declared wedged only
// when critical error occurs like GT reset failure or firmware failure.
// This is the default mode.
// @XE_WEDGED_MODE_UPON_ANY_HANG_NO_RESET: Device will be declared wedged on
// any hang. In this mode, engine resets are disabled to avoid automatic
// recovery attempts. This mode is primarily intended for debugging hangs.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xe_wedged_mode {
    XE_WEDGED_MODE_NEVER = 0,
    XE_WEDGED_MODE_UPON_CRITICAL_ERROR = 1,
    XE_WEDGED_MODE_UPON_ANY_HANG_NO_RESET = 2,
}

//
// enum xe_page_size_alloc_ctrl_mode - User BO page-size allocation control modes
// @XE_PAGE_SIZE_ALLOC_CTRL_MODE_NONE: Use the normal allocation policy
// @XE_PAGE_SIZE_ALLOC_CTRL_MODE_ONLY_2M: Force user BO allocations to 2M pages
// @XE_PAGE_SIZE_ALLOC_CTRL_MODE_ONLY_1G: Force user BO allocations to 1G pages
// @XE_PAGE_SIZE_ALLOC_CTRL_MODE_MIXED: Select page sizes in round-robin order
// (4K, 64K, 2M, 1G)
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xe_page_size_alloc_ctrl_mode {
    XE_PAGE_SIZE_ALLOC_CTRL_MODE_NONE = 0,
    XE_PAGE_SIZE_ALLOC_CTRL_MODE_ONLY_2M,
    XE_PAGE_SIZE_ALLOC_CTRL_MODE_ONLY_1G,
    XE_PAGE_SIZE_ALLOC_CTRL_MODE_MIXED
}

pub const XE_GT0: c_int = 0;
pub const XE_GT1: c_int = 1;

//
// Highest GT/tile count for any platform.  Used only for memory allocation
// sizing.  Any logic looping over GTs or mapping userspace GT IDs into GT
// structures should use the per-platform xe->info.max_gt_per_tile instead.
//
pub const XE_MAX_GT_PER_TILE: c_int = 2;

//
// struct xe_device - Top level struct of Xe device
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_device {
// @drm: drm device
    pub drm: drm_device,

// @display: display device data, must be placed after drm device member
    pub display: *mut intel_display,

// @devcoredump: device coredump
    pub devcoredump: xe_devcoredump,
// @info: device info
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_device_info {
// @info.platform_name: platform name
    pub platform_name: *const c_char,
// @info.graphics_name: graphics IP name
    pub graphics_name: *const c_char,
// @info.media_name: media IP name
    pub media_name: *const c_char,
// @info.graphics_verx100: graphics IP version
    pub graphics_verx100: u32,
// @info.media_verx100: media IP version
    pub media_verx100: u32,
// @info.mem_region_mask: mask of valid memory regions
    pub mem_region_mask: u32,
// @info.platform: Xe platform enum
    pub platform: xe_platform,
// @info.subplatform: Xe subplatform enum
    pub subplatform: xe_subplatform,
// @info.devid: device ID
    pub devid: u16,
// @info.revid: device revision
    pub revid: u8,
// @info.step: stepping information for each IP
    pub step: xe_step_info,
// @info.dma_mask_size: DMA address bits
    pub dma_mask_size: u8,
// @info.vram_flags: Vram flags
    pub vram_flags: u8,
// @info.tile_count: Number of tiles
    pub tile_count: u8,
// @info.max_gt_per_tile: Number of GT IDs allocated to each tile
    pub max_gt_per_tile: u8,
// @info.multi_lrc_mask: bitmask of engine classes which support multi-lrc
    pub multi_lrc_mask: u8,
// @info.gt_count: Total number of GTs for entire device
    pub gt_count: u8,
// @info.vm_max_level: Max VM level
    pub vm_max_level: u8,
// @info.va_bits: Maximum bits of a virtual address
    pub va_bits: u8,
//
// Keep all flags below alphabetically sorted
//
// @info.has_access_counter: Device supports access counter
    pub has_access_counter:1: u8,
// @info.has_asid: Has address space ID
    pub has_asid:1: u8,
// @info.has_atomic_enable_pte_bit: Device has atomic enable PTE bit
    pub has_atomic_enable_pte_bit:1: u8,
// @info.has_cached_pt: Supports caching pagetable
    pub has_cached_pt:1: u8,
// @info.has_device_atomics_on_smem: Supports device atomics on SMEM
    pub has_device_atomics_on_smem:1: u8,
// @info.has_drm_ras: Device supports drm_ras (Reliability, Availability, Serviceability)
    pub has_drm_ras:1: u8,
// @info.has_fan_control: Device supports fan control
    pub has_fan_control:1: u8,
// @info.has_flat_ccs: Whether flat CCS metadata is used
    pub has_flat_ccs:1: u8,
// @info.has_gsc_nvm: Device has gsc non-volatile memory
    pub has_gsc_nvm:1: u8,
// @info.has_heci_cscfi: device has heci cscfi
    pub has_heci_cscfi:1: u8,
// @info.has_heci_gscfi: device has heci gscfi
    pub has_heci_gscfi:1: u8,
// @info.has_i2c: Device has I2C controller
    pub has_i2c:1: u8,
// @info.has_late_bind: Device has firmware late binding support
    pub has_late_bind:1: u8,
// @info.has_llc: Device has a shared CPU+GPU last level cache
    pub has_llc:1: u8,
// @info.has_mbx_power_limits: Device has support to manage power limits using
// pcode mailbox commands.
//
    pub has_mbx_power_limits:1: u8,
// @info.has_mbx_thermal_info: Device supports thermal mailbox commands
    pub has_mbx_thermal_info:1: u8,
// @info.has_mem_copy_instr: Device supports MEM_COPY instruction
    pub has_mem_copy_instr:1: u8,
// @info.has_mert: Device has standalone MERT
    pub has_mert:1: u8,
// @info.has_page_reclaim_hw_assist: Device supports page reclamation feature
    pub has_page_reclaim_hw_assist:1: u8,
// @info.has_pre_prod_wa: Pre-production workarounds still present in driver
    pub has_pre_prod_wa:1: u8,
// @info.has_pxp: Device has PXP support
    pub has_pxp:1: u8,
// @info.has_ctx_tlb_inval: Has context based TLB invalidations
    pub has_ctx_tlb_inval:1: u8,
// @info.has_range_tlb_inval: Has range based TLB invalidations
    pub has_range_tlb_inval:1: u8,
// @info.has_soc_remapper_sysctrl: Has SoC remapper system controller
    pub has_soc_remapper_sysctrl:1: u8,
// @info.has_soc_remapper_telem: Has SoC remapper telemetry support
    pub has_soc_remapper_telem:1: u8,
// @info.has_sriov: Supports SR-IOV
    pub has_sriov:1: u8,
// @info.has_sysctrl: Supports System Controller
    pub has_sysctrl:1: u8,
// @info.has_usm: Device has unified shared memory support
    pub has_usm:1: u8,
// @info.has_64bit_timestamp: Device supports 64-bit timestamps
    pub has_64bit_timestamp:1: u8,
// @info.is_dgfx: is discrete device
    pub is_dgfx:1: u8,
// @info.needs_scratch: needs scratch page for oob prefetch to work
    pub needs_scratch:1: u8,
//
// @info.probe_display: Probe display hardware.  If set to
// false, the driver will behave as if there is no display
// hardware present and will not try to read/write to it in any
// way.  The display hardware, if it exists, will not be
// exposed to userspace and will be left untouched in whatever
// state the firmware or bootloader left it in.
//
    pub probe_display:1: u8,
// @info.skip_guc_pc: Skip GuC based PM feature init
    pub skip_guc_pc:1: u8,
// @info.skip_pcode: skip access to PCODE uC
    pub skip_pcode:1: u8,
// @info.needs_shared_vf_gt_wq: needs shared GT WQ on VF
    pub needs_shared_vf_gt_wq:1: u8,
    pub info: },
// @wa_active: keep track of active workarounds
// @wa_active.oob: bitmap with active OOB workarounds
    pub oob: *mut c_ulong,
//
// @wa_active.oob_initialized: Mark oob as initialized to help detecting misuse
// of XE_DEVICE_WA() - it can only be called on initialization after
// Device OOB WAs have been processed.
//
    pub oob_initialized: bool,
    pub wa_active: },
// @survivability: survivability information for device
    pub survivability: xe_survivability,
// @irq: device interrupt state
// @irq.lock: lock for processing irq's on this device
    pub lock: spinlock_t,
// @irq.enabled: interrupts enabled on this device
    pub enabled: core::sync::atomic::AtomicI32,
// @irq.msix: irq info for platforms that support MSI-X
// @irq.msix.nvec: number of MSI-X interrupts
    pub nvec: u16,
// @irq.msix.indexes: used to allocate MSI-X indexes
    pub indexes: xarray,
    pub msix: },
    pub irq: },
// @ttm: ttm device
    pub ttm: ttm_device,
// @mmio: mmio info for device
// @mmio.size: size of MMIO space for device
    pub size: usize,
// @mmio.regs: pointer to MMIO space for device
    pub regs: *mut void __iomem,
    pub mmio: },
// @mem: memory info for device
// @mem.vram: VRAM info for device
    pub vram: *mut xe_vram_region,
// @mem.sys_mgr: system TTM manager
    pub sys_mgr: ttm_resource_manager,
// @mem.shrinker: system memory shrinker.
    pub shrinker: *mut xe_shrinker,
// @mem.stolen_mgr: stolen memory manager.
    pub stolen_mgr: *mut xe_ttm_stolen_mgr,
    pub mem: },
// @sriov: device level virtualization data
// @sriov.__mode: SR-IOV mode (Don't access directly!)
    pub __mode: xe_sriov_mode,
// @sriov.pf: PF specific data
    pub pf: xe_device_pf,
// @sriov.vf: VF specific data
    pub vf: xe_device_vf,
}

// @sriov.wq: workqueue used by the virtualization workers
// @usm: unified memory state
// @usm.asid_to_vm: convert an ASID to VM
// @usm.next_asid: next ASID, used to cyclical alloc asids
// @usm.lock: protects UM state
// @usm.pf_wq: page fault work queue, unbound, high priority
//
// We pick 4 here because, in the current implementation, it
// yields the best bandwidth utilization of the kernel paging
// engine.
//
pub const XE_PAGEFAULT_QUEUE_COUNT: c_int = 4;
// @usm.pf_queue: Page fault queues

// @usm.dpagemap_shrinker: Shrinker for unused pagemaps

// @pinned: pinned BO state
// @pinned.lock: protected pinned BO list state
// @pinned.early: early pinned lists
// @pinned.early.kernel_bo_present: pinned kernel BO that are present
// @pinned.early.evicted: pinned BO that have been evicted
// @pinned.late: late pinned lists
// @pinned.late.kernel_bo_present: pinned kernel BO that are present
// @pinned.late.evicted: pinned BO that have been evicted
// @pinned.late.external: pinned external and dma-buf.
// @ufence_wq: user fence wait queue
// @preempt_fence_wq: used to serialize preempt fences
// @ordered_wq: used to serialize compute mode resume
// @unordered_wq: used to serialize unordered work
// @destroy_wq: used to serialize SVM pagemap destroy work
// @tiles: device tiles
//
// @mem_access: keep track of memory access in the device, possibly
// triggering additional actions when they occur.
//
// @mem_access.vram_userfault: Encapsulate vram_userfault
// related stuff
//
// @mem_access.vram_userfault.lock: Protects access to
// @mem_access.vram_userfault.list Using mutex instead of spinlock
// as lock is applied to entire list operation which
// may sleep
//
// @mem_access.vram_userfault.list: Keep list of userfaulted
// vram bo, which require to release their mmap mappings
// at runtime suspend path
//
// @pat: Encapsulate PAT related stuff
//
// @pat.ops: Internal operations to abstract platforms
// @pat.table: PAT table to program in the HW
// @pat.n_entries: Number of PAT entries
// @pat.pat_ats: PAT entry for PCIe ATS responses
// @pat.pat_primary_pta: primary GT PAT entry for page table accesses
// @pat.pat_media_pta: media GT PAT entry for page table accesses
// @pat.pat_primary_tr_pta: primary GT PAT entry for TRTT page table accesses
// @pat.pat_media_tr_pta: media GT PAT entry for TRTT page table accesses
// @d3cold: Encapsulate d3cold related stuff
// @d3cold.capable: Indicates if root port is d3cold capable
// @d3cold.allowed: Indicates if d3cold is a valid device state
//
// @d3cold.vram_threshold:
//
// This represents the permissible threshold(in megabytes)
// for vram save/restore. d3cold will be disallowed,
// when vram_usages is above or equals the threshold value
// to avoid the vram save/restore latency.
// Default threshold value is 300mb.
//
// @d3cold.lock: protect vram_threshold
// @pm_notifier: Our PM notifier to perform actions in response to various PM events.
// @pm_block: Completion to block validating tasks on suspend / hibernate prepare
// @rebind_resume_list: List of wq items to kick on resume.
// @rebind_resume_lock: Lock to protect the rebind_resume_list
// @pmt: Support the PMT driver callback interface
// @pmt.lock: protect access for telemetry data
// @soc_remapper: SoC remapper object
// @soc_remapper.lock: Serialize access to SoC Remapper's index registers
// @soc_remapper.set_telem_region: Set telemetry index
// @soc_remapper.set_sysctrl_region: Set system controller index
//
// @pm_callback_task: Track the active task that is running in either
// the runtime_suspend or runtime_resume callbacks.
//
// @hwmon: hwmon subsystem integration
// @heci_gsc: graphics security controller
// @nvm: discrete graphics non-volatile memory
// @late_bind: xe mei late bind interface

//
// @page_size_alloc_ctrl: User BO page-size allocation
// debug control state
//
// @page_size_alloc_ctrl.mode: xe page size allocation control mode
// @page_size_alloc_ctrl.cur_index: Round-robin index used by mixed mode
// @page_size_alloc_ctrl.lock: Protects @mode and @cur_index

// @oa: oa observation subsystem
// @pxp: Encapsulate Protected Xe Path support
// @needs_flr_on_fini: requests function-reset on fini
// @in_reset: Indicates if device is in reset
// @wedged: Struct to control Wedged States and mode
// @wedged.flag: Xe device faced a critical error and is now blocked.
// @wedged.mode: Mode controlled by kernel parameter and debugfs
// @wedged.method: Recovery method to be sent in the drm device wedged uevent
// @wedged.inconsistent_reset: Inconsistent reset policy state between GTs
// @devres_group: devres group
// @bo_device: Struct to control async free of BOs
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_bo_dev {
// @bo_device.async_free: Free worker
    pub async_free: work_struct,
// @bo_device.async_list: List of BOs to be freed
    pub async_list: llist_head,
    pub bo_device: },
// @pmu: performance monitoring unit
    pub pmu: xe_pmu,
// @ras: RAS structure for device
    pub ras: xe_drm_ras,
// @i2c: I2C host controller
    pub i2c: *mut xe_i2c,
// @sc: System Controller
    pub sc: xe_sysctrl,
// @atomic_svm_timeslice_ms: Atomic SVM fault timeslice MS
    pub atomic_svm_timeslice_ms: u32,
// @min_run_period_lr_ms: LR VM (preempt fence mode) timeslice
    pub min_run_period_lr_ms: u32,
// @min_run_period_pf_ms: LR VM (page fault mode) timeslice
    pub min_run_period_pf_ms: u32,

//
// @vm_inject_error_position: inject errors at different places in VM
// bind IOCTL based on this value
//
    pub vm_inject_error_position: u8,

//
// @global_total_pages: global GPU page usage tracked for gpu_mem
// tracepoints
//
    pub global_total_pages: core::sync::atomic::AtomicI64,

// @val: The domain for exhaustive eviction, which is currently per device.
    pub val: xe_validation_device,
// @psmi: GPU debugging via additional validation HW
// @psmi.capture_obj: PSMI buffer for VRAM
    pub 1]: *mut *mut xe_bo capture_obj[XE_MAX_TILES_PER_DEVICE +,
// @psmi.region_mask: Mask of valid memory regions
    pub region_mask: u8,
    pub psmi: },

// @g2g_test_array: for testing G2G communications
    pub g2g_test_array: *mut u32,
// @g2g_test_count: for testing G2G communications
    pub g2g_test_count: core::sync::atomic::AtomicI32,

// private:

//
// Any fields below this point are the ones used by display.
// They are temporarily added here so xe_device can be desguised as
// drm_i915_private during build. After cleanup these should go away,
// migrating to the right sub-structs
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_uncore {
    pub lock: spinlock_t,
    pub uncore: },

}

//
// struct xe_file - file handle for Xe driver
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_file {
// @xe: xe DEVICE
    pub xe: *mut xe_device,
// @drm: base DRM file
    pub drm: *mut drm_file,
// @vm: VM state for file
// @vm.xa: xarray to store VMs
    pub xa: xarray,
//
// @vm.lock: Protects VM lookup + reference and removal from
// file xarray. Not an intended to be an outer lock which does
// thing while being held.
//
    pub lock: mutex,
    pub vm: },
// @exec_queue: Submission exec queue state for file
// @exec_queue.xa: xarray to store exece queues
    pub xa: xarray,
//
// @exec_queue.lock: Protects exec queue lookup + reference and
// removal from file xarray. Not intended to be an outer lock
// which does things while being held.
//
    pub lock: mutex,
//
// @exec_queue.pending_removal: items pending to be removed to
// synchronize GPU state update with ongoing query.
//
    pub pending_removal: core::sync::atomic::AtomicI32,
    pub exec_queue: },
// @run_ticks: hw engine class run time in ticks for this drm client
    pub run_ticks: [u64; XE_ENGINE_CLASS_MAX],
// @client: drm client
    pub client: *mut xe_drm_client,
//
// @process_name: process name for file handle, used to safely output
// during error situations where xe file can outlive process
//
    pub process_name: *mut c_char,
//
// @pid: pid for file handle, used to safely output uring error
// situations where xe file can outlive process
//
    pub pid: pid_t,
// @refcount: ref count of this xe file
    pub refcount: kref,
}
