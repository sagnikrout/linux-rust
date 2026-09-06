//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/panthor/panthor_device.h
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


// SPDX-License-Identifier: GPL-2.0 or MIT
// Copyright 2018 Marty E. Plummer <hanetzer@startmail.com>
// Copyright 2019 Linaro, Ltd, Rob Herring <robh@kernel.org>
// Copyright 2023 Collabora ltd.

//
// struct panthor_soc_data - Panthor SoC Data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct panthor_soc_data {
// @asn_hash_enable: True if GPU_L2_CONFIG_ASN_HASH_ENABLE must be set.
    pub asn_hash_enable: bool,
// @asn_hash: ASN_HASH values when asn_hash_enable is true.
    pub asn_hash: [u32; 3],
}

//
// enum panthor_device_pm_state - PM state
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum panthor_device_pm_state {
// @PANTHOR_DEVICE_PM_STATE_SUSPENDED: Device is suspended.
    PANTHOR_DEVICE_PM_STATE_SUSPENDED = 0,

// @PANTHOR_DEVICE_PM_STATE_RESUMING: Device is being resumed.
    PANTHOR_DEVICE_PM_STATE_RESUMING,

// @PANTHOR_DEVICE_PM_STATE_ACTIVE: Device is active.
    PANTHOR_DEVICE_PM_STATE_ACTIVE,

// @PANTHOR_DEVICE_PM_STATE_SUSPENDING: Device is being suspended.
    PANTHOR_DEVICE_PM_STATE_SUSPENDING,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum panthor_irq_state {
// @PANTHOR_IRQ_STATE_ACTIVE: IRQ is active and ready to process events.
    PANTHOR_IRQ_STATE_ACTIVE = 0,
// @PANTHOR_IRQ_STATE_PROCESSING: IRQ is currently processing events.
    PANTHOR_IRQ_STATE_PROCESSING,
// @PANTHOR_IRQ_STATE_SUSPENDED: IRQ is suspended.
    PANTHOR_IRQ_STATE_SUSPENDED,
// @PANTHOR_IRQ_STATE_SUSPENDING: IRQ is being suspended.
    PANTHOR_IRQ_STATE_SUSPENDING,
}

//
// struct panthor_irq - IRQ data
//
// Used to automate IRQ handling for the 3 different IRQs we have in this driver.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct panthor_irq {
// @ptdev: Panthor device
    pub ptdev: *mut panthor_device,
// @iomem: CPU mapping of IRQ base address
    pub iomem: *mut void __iomem,
// @irq: IRQ number.
    pub irq: c_int,
// @mask: Values to write to xxx_INT_MASK if active.
    pub mask: u32,
//
// @mask_lock: protects modifications to _INT_MASK and @mask.
//
// In paths where _INT_MASK is updated based on a state
// transition/check, it's crucial for the state update/check to be
// inside the locked section, otherwise it introduces a race window
// leading to potential _INT_MASK inconsistencies.
//
    pub mask_lock: spinlock_t,
// @state: one of &enum panthor_irq_state reflecting the current state.
    pub state: core::sync::atomic::AtomicI32,
}

//
// enum panthor_device_profiling_mode - Profiling state
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum panthor_device_profiling_flags {
// @PANTHOR_DEVICE_PROFILING_DISABLED: Profiling is disabled.
    PANTHOR_DEVICE_PROFILING_DISABLED = 0,

// @PANTHOR_DEVICE_PROFILING_CYCLES: Sampling job cycles.
    PANTHOR_DEVICE_PROFILING_CYCLES = BIT(0),

// @PANTHOR_DEVICE_PROFILING_TIMESTAMP: Sampling job timestamp.
    PANTHOR_DEVICE_PROFILING_TIMESTAMP = BIT(1),

// @PANTHOR_DEVICE_PROFILING_ALL: Sampling everything.
    PANTHOR_DEVICE_PROFILING_ALL =
    PANTHOR_DEVICE_PROFILING_CYCLES |
    PANTHOR_DEVICE_PROFILING_TIMESTAMP,
}

//
// struct panthor_device - Panthor device
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct panthor_device {
// @base: Base drm_device.
    pub base: drm_device,
// @soc_data: Optional SoC data.
    pub soc_data: *const panthor_soc_data,
// @phys_addr: Physical address of the iomem region.
    pub phys_addr: phys_addr_t,
// @iomem: CPU mapping of the IOMEM region.
    pub iomem: *mut void __iomem,
// @clks: GPU clocks.
// @core: Core clock.
    pub core: *mut clk,
// @stacks: Stacks clock. This clock is optional.
    pub stacks: *mut clk,
// @coregroup: Core group clock. This clock is optional.
    pub coregroup: *mut clk,
    pub clks: },
// @coherent: True if the CPU/GPU are memory coherent.
    pub coherent: bool,
// @gpu_info: GPU information.
    pub gpu_info: drm_panthor_gpu_info,
// @csif_info: Command stream interface information.
    pub csif_info: drm_panthor_csif_info,
// @mmu_info: MMU info
    pub mmu_info: drm_panthor_mmu_info,
// @hw: GPU-specific data.
    pub hw: *mut panthor_hw,
// @pwr: Power control management data.
    pub pwr: *mut panthor_pwr,
// @gpu: GPU management data.
    pub gpu: *mut panthor_gpu,
// @fw: FW management data.
    pub fw: *mut panthor_fw,
// @mmu: MMU management data.
    pub mmu: *mut panthor_mmu,
// @scheduler: Scheduler management data.
    pub scheduler: *mut panthor_scheduler,
// @devfreq: Device frequency scaling management data.
    pub devfreq: *mut panthor_devfreq,
// @reclaim: Reclaim related stuff
// @reclaim.shrinker: Shrinker instance
    pub shrinker: *mut shrinker,
//
// @reclaim.unused: BOs with unused pages
//
// Basically all buffers that got mmapped, vmapped or GPU mapped and
// then unmapped. There should be no contention on these buffers,
// making them ideal to reclaim.
//
    pub unused: drm_gem_lru,
//
// @reclaim.mmapped: mmap()-ed buffers
//
// Those are relatively easy to reclaim since we don't need user
// agreement, we can simply teardown the mapping and let it fault on
// the next access.
//
    pub mmapped: drm_gem_lru,
//
// @reclaim.gpu_mapped_shared: shared BO LRU list
//
// That's the most tricky BO type to reclaim, because it involves
// tearing down all mappings in all VMs where this BO is mapped,
// which increases the risk of contention and thus decreases the
// likeliness of success.
//
    pub gpu_mapped_shared: drm_gem_lru,
//
// @reclaim.vms: VM LRU list
//
// VMs that have reclaimable BOs only mapped to a single VM are placed
// in this LRU. Reclaiming such BOs implies waiting for VM idleness
// (no in-flight GPU jobs targeting this VM), meaning we can't reclaim
// those if we're in a context where we can't block/sleep.
//
    pub vms: list_head,
//
// @reclaim.gpu_mapped_count: Global counter of pages that are GPU mapped
//
// Allows us to get the number of reclaimable pages without walking
// the vms and gpu_mapped_shared LRUs.
//
    pub gpu_mapped_count: c_long,
//
// @reclaim.retry_count: Number of times we ran the shrinker without being
// able to reclaim stuff
//
// Used to stop scanning GEMs when too many attempts were made
// without progress.
//
    pub retry_count: core::sync::atomic::AtomicI32,

//
// @reclaim.nr_pages_reclaimed_on_last_scan: Number of pages reclaimed on the last
// shrinker scan
//
    pub nr_pages_reclaimed_on_last_scan: c_ulong,

    pub reclaim: },
// @unplug: Device unplug related fields.
// @lock: Lock used to serialize unplug operations.
    pub lock: mutex,
//
// @done: Completion object signaled when the unplug
// operation is done.
//
    pub done: completion,
    pub unplug: },
// @reset: Reset related fields.
// @wq: Ordered worqueud used to schedule reset operations.
    pub wq: *mut workqueue_struct,
// @work: Reset work.
    pub work: work_struct,
// @pending: Set to true if a reset is pending.
    pub pending: core::sync::atomic::AtomicI32,
//
// @fast: True if the post_reset logic can proceed with a fast reset.
//
// A fast reset is just a reset where the driver doesn't reload the FW sections.
//
// Any time the firmware is properly suspended, a fast reset can take place.
// On the other hand, if the halt operation failed, the driver will reload
// all FW sections to make sure we start from a fresh state.
//
    pub fast: bool,
    pub reset: },
// @pm: Power management related data.
// @state: Power state.
    pub state: core::sync::atomic::AtomicI32,
//
// @mmio_lock: Lock protecting MMIO userspace CPU mappings.
//
// This is needed to ensure we map the dummy IO pages when
// the device is being suspended, and the real IO pages when
// the device is being resumed. We can't just do with the
// state atomicity to deal with this race.
//
    pub mmio_lock: mutex,
//
// @dummy_latest_flush: Dummy LATEST_FLUSH page.
//
// Used to replace the real LATEST_FLUSH page when the GPU
// is suspended.
//
    pub dummy_latest_flush: *mut page,
// @recovery_needed: True when a resume attempt failed.
    pub recovery_needed: core::sync::atomic::AtomicI32,
    pub pm: },
// @profile_mask: User-set profiling flags for job accounting.
    pub profile_mask: u32,
// @fast_rate: Maximum device clock frequency. Set by DVFS
    pub fast_rate: c_ulong,

// @gems: Device-wide list of GEM objects owned by at least one file.
// @gems.lock: Protects the device-wide list of GEM objects.
    pub lock: mutex,
// @node: Used to keep track of all the device's DRM objects
    pub node: list_head,
    pub gems: },

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct panthor_gpu_usage {
    pub time: u64,
    pub cycles: u64,
}

//
// struct panthor_file - Panthor file
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct panthor_file {
// @ptdev: Device attached to this file.
    pub ptdev: *mut panthor_device,
// @user_mmio: User MMIO related fields.
//
// @offset: Offset used for user MMIO mappings.
//
// This offset should not be used to check the type of mapping
// except in panthor_mmap(). After that point, MMIO mapping
// offsets have been adjusted to match
// DRM_PANTHOR_USER_MMIO_OFFSET and that macro should be used
// instead.
// Make sure this rule is followed at all times, because
// userspace is in control of the offset, and can change the
// value behind our back. Otherwise it can lead to erroneous
// branching happening in kernel space.
//
    pub offset: u64,
    pub user_mmio: },
// @vms: VM pool attached to this file.
    pub vms: *mut panthor_vm_pool,
// @groups: Scheduling group pool attached to this file.
    pub groups: *mut panthor_group_pool,
// @stats: cycle and timestamp measures for job execution.
    pub stats: panthor_gpu_usage,
}

extern "C" {
    pub fn panthor_device_init(ptdev: *mut panthor_device) -> c_int;
}
extern "C" {
    pub fn panthor_device_unplug(ptdev: *mut panthor_device);
}
//
// panthor_device_schedule_reset() - Schedules a reset operation
//
// panthor_device_reset_is_pending() - Checks if a reset is pending.
//
// Return: true if a reset is pending, false otherwise.
//
extern "C" {
    pub fn panthor_device_resume(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn panthor_device_suspend(dev: *mut device) -> c_int;
}
// If the resume failed, we need to clear the runtime_error, which
// can done by forcing the RPM state to suspended. If multiple
// threads called panthor_device_resume_and_get(), we only want
// one of them to update the state, hence the cmpxchg. Note that a
// thread might enter panthor_device_resume_and_get() and call
// pm_runtime_resume_and_get() after another thread had attempted
// to resume and failed. This means we will end up with an error
// without even attempting a resume ourselves. The only risk here
// is to report an error when the second resume attempt might have
// succeeded. Given resume errors are not expected, this is probably
// something we can live with.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drm_panthor_exception_type {
    DRM_PANTHOR_EXCEPTION_OK = 0x00,
    DRM_PANTHOR_EXCEPTION_TERMINATED = 0x04,
    DRM_PANTHOR_EXCEPTION_KABOOM = 0x05,
    DRM_PANTHOR_EXCEPTION_EUREKA = 0x06,
    DRM_PANTHOR_EXCEPTION_ACTIVE = 0x08,
    DRM_PANTHOR_EXCEPTION_CS_RES_TERM = 0x0f,
    DRM_PANTHOR_EXCEPTION_MAX_NON_FAULT = 0x3f,
    DRM_PANTHOR_EXCEPTION_CS_CONFIG_FAULT = 0x40,
    DRM_PANTHOR_EXCEPTION_CS_UNRECOVERABLE = 0x41,
    DRM_PANTHOR_EXCEPTION_CS_ENDPOINT_FAULT = 0x44,
    DRM_PANTHOR_EXCEPTION_CS_BUS_FAULT = 0x48,
    DRM_PANTHOR_EXCEPTION_CS_INSTR_INVALID = 0x49,
    DRM_PANTHOR_EXCEPTION_CS_CALL_STACK_OVERFLOW = 0x4a,
    DRM_PANTHOR_EXCEPTION_CS_INHERIT_FAULT = 0x4b,
    DRM_PANTHOR_EXCEPTION_INSTR_INVALID_PC = 0x50,
    DRM_PANTHOR_EXCEPTION_INSTR_INVALID_ENC = 0x51,
    DRM_PANTHOR_EXCEPTION_INSTR_BARRIER_FAULT = 0x55,
    DRM_PANTHOR_EXCEPTION_DATA_INVALID_FAULT = 0x58,
    DRM_PANTHOR_EXCEPTION_TILE_RANGE_FAULT = 0x59,
    DRM_PANTHOR_EXCEPTION_ADDR_RANGE_FAULT = 0x5a,
    DRM_PANTHOR_EXCEPTION_IMPRECISE_FAULT = 0x5b,
    DRM_PANTHOR_EXCEPTION_OOM = 0x60,
    DRM_PANTHOR_EXCEPTION_CSF_FW_INTERNAL_ERROR = 0x68,
    DRM_PANTHOR_EXCEPTION_CSF_RES_EVICTION_TIMEOUT = 0x69,
    DRM_PANTHOR_EXCEPTION_GPU_BUS_FAULT = 0x80,
    DRM_PANTHOR_EXCEPTION_GPU_SHAREABILITY_FAULT = 0x88,
    DRM_PANTHOR_EXCEPTION_SYS_SHAREABILITY_FAULT = 0x89,
    DRM_PANTHOR_EXCEPTION_GPU_CACHEABILITY_FAULT = 0x8a,
    DRM_PANTHOR_EXCEPTION_TRANSLATION_FAULT_0 = 0xc0,
    DRM_PANTHOR_EXCEPTION_TRANSLATION_FAULT_1 = 0xc1,
    DRM_PANTHOR_EXCEPTION_TRANSLATION_FAULT_2 = 0xc2,
    DRM_PANTHOR_EXCEPTION_TRANSLATION_FAULT_3 = 0xc3,
    DRM_PANTHOR_EXCEPTION_TRANSLATION_FAULT_4 = 0xc4,
    DRM_PANTHOR_EXCEPTION_PERM_FAULT_0 = 0xc8,
    DRM_PANTHOR_EXCEPTION_PERM_FAULT_1 = 0xc9,
    DRM_PANTHOR_EXCEPTION_PERM_FAULT_2 = 0xca,
    DRM_PANTHOR_EXCEPTION_PERM_FAULT_3 = 0xcb,
    DRM_PANTHOR_EXCEPTION_ACCESS_FLAG_1 = 0xd9,
    DRM_PANTHOR_EXCEPTION_ACCESS_FLAG_2 = 0xda,
    DRM_PANTHOR_EXCEPTION_ACCESS_FLAG_3 = 0xdb,
    DRM_PANTHOR_EXCEPTION_ADDR_SIZE_FAULT_IN = 0xe0,
    DRM_PANTHOR_EXCEPTION_ADDR_SIZE_FAULT_OUT0 = 0xe4,
    DRM_PANTHOR_EXCEPTION_ADDR_SIZE_FAULT_OUT1 = 0xe5,
    DRM_PANTHOR_EXCEPTION_ADDR_SIZE_FAULT_OUT2 = 0xe6,
    DRM_PANTHOR_EXCEPTION_ADDR_SIZE_FAULT_OUT3 = 0xe7,
    DRM_PANTHOR_EXCEPTION_MEM_ATTR_FAULT_0 = 0xe8,
    DRM_PANTHOR_EXCEPTION_MEM_ATTR_FAULT_1 = 0xe9,
    DRM_PANTHOR_EXCEPTION_MEM_ATTR_FAULT_2 = 0xea,
    DRM_PANTHOR_EXCEPTION_MEM_ATTR_FAULT_3 = 0xeb,
}

//
// panthor_exception_is_fault() - Checks if an exception is a fault.
//
// Return: true if the exception is a fault, false otherwise.
//
pub const INT_RAWSTAT: c_uint = 0x0;
pub const INT_CLEAR: c_uint = 0x4;
pub const INT_MASK: c_uint = 0x8;
pub const INT_STAT: c_uint = 0xc;
//
// PANTHOR_IRQ_HANDLER() - Define interrupt handlers and the interrupt
// registration function.
//
// The boiler-plate to gracefully deal with shared interrupts is
// auto-generated. All you have to do is call PANTHOR_IRQ_HANDLER()
// just after the actual handler. The handler prototype is:
//
// void (*handler)(struct panthor_device *, u32 status);
//

// It's safe to access pirq->mask without the lock held here. If a new		\
// event gets added to the mask and the corresponding IRQ is pending,		\
// we'll process it right away instead of adding an extra raw -> threaded	\
// round trip. If an event is removed and the status bit is set, it will	\
// be ignored, just like it would have been if the mask had been adjusted	\
// right before the HW event kicks in. TLDR; it's all expected races we're	\
// covered for.									\
// \
// The only situation where we need to write the new mask is if the IRQ is active.	\
// If it's being processed, the mask will be restored for us in _irq_threaded_handler()	\
// on the PROCESSING -> ACTIVE transition.						\
// If the IRQ is suspended/suspending, the mask is restored at resume time.		\
// \
// The only situation where we need to write the new mask is if the IRQ is active.	\
// If it's being processed, the mask will be restored for us in _irq_threaded_handler()	\
// on the PROCESSING -> ACTIVE transition.						\
// If the IRQ is suspended/suspending, the mask is restored at resume time.		\
// \
extern "C" {
    pub fn readl(reg: iomem +) -> return;
}
extern "C" {
    pub fn readl_relaxed(reg: iomem +) -> return;
}

