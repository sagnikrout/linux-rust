//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/panfrost/panfrost_device.h
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


// SPDX-License-Identifier: GPL-2.0
// Copyright 2018 Marty E. Plummer <hanetzer@startmail.com>
// Copyright 2019 Linaro, Ltd, Rob Herring <robh@kernel.org>

pub const MAX_PM_DOMAINS: c_int = 5;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum panfrost_drv_comp_bits {
    PANFROST_COMP_BIT_GPU,
    PANFROST_COMP_BIT_JOB,
    PANFROST_COMP_BIT_MMU,
    PANFROST_COMP_BIT_MAX
}

//
// enum panfrost_gpu_pm - Supported kernel power management features
// @GPU_PM_CLK_DIS:  Allow disabling clocks during system suspend
// @GPU_PM_VREG_OFF: Allow turning off regulators during system suspend
// @GPU_PM_RT: Allow disabling clocks and asserting the reset control during
// system runtime suspend
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum panfrost_gpu_pm {
    GPU_PM_CLK_DIS,
    GPU_PM_VREG_OFF,
    GPU_PM_RT
}

//
// enum panfrost_gpu_quirks - GPU optional quirks
// @GPU_QUIRK_FORCE_AARCH64_PGTABLE: Use AARCH64_4K page table format
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum panfrost_gpu_quirks {
    GPU_QUIRK_FORCE_AARCH64_PGTABLE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct panfrost_features {
    pub id: u16,
    pub revision: u16,
    pub shader_present: u64,
    pub tiler_present: u64,
    pub l2_present: u64,
    pub stack_present: u64,
    pub as_present: u32,
    pub js_present: u32,
    pub l2_features: u32,
    pub core_features: u32,
    pub tiler_features: u32,
    pub mem_features: u32,
    pub mmu_features: u32,
    pub thread_features: u32,
    pub max_threads: u32,
    pub thread_max_workgroup_sz: u32,
    pub thread_max_barrier_sz: u32,
    pub coherency_features: u32,
    pub selected_coherency: u32,
    pub afbc_features: u32,
    pub texture_features: [u32; 4],
    pub js_features: [u32; 16],
    pub nr_core_groups: u32,
    pub thread_tls_alloc: u32,
    pub BITS_PER_LONG]: unsigned long hw_features[64 /,
    pub BITS_PER_LONG]: unsigned long hw_issues[64 /,
}

//
// Features that cannot be automatically detected and need matching using the
// compatible string, typically SoC-specific.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct panfrost_compatible {
// Supplies count and names.
    pub num_supplies: c_int,
    pub supply_names: *const *const c_char,
//
// Number of power domains required, note that values 0 and 1 are
// handled identically, as only values > 1 need special handling.
//
    pub num_pm_domains: c_int,
// Only required if num_pm_domains > 1.
    pub pm_domain_names: *const *const c_char,
// Vendor implementation quirks callback
    pub pfdev): *mut *mut void (vendor_quirk)(struct panfrost_device,
// Allowed PM features
    pub pm_features: u8,
// GPU configuration quirks
    pub gpu_quirks: u8,
}

//
// struct panfrost_device_debugfs - Device-wide DebugFS tracking structures
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct panfrost_device_debugfs {
// @gems_list: Device-wide list of GEM objects owned by at least one file.
    pub gems_list: list_head,
// @gems_lock: Serializes access to the device-wide list of GEM objects.
    pub gems_lock: mutex,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct panfrost_device {
    pub base: drm_device,
    pub gpu_irq: c_int,
    pub mmu_irq: c_int,
    pub iomem: *mut void __iomem,
    pub clock: *mut clk,
    pub bus_clock: *mut clk,
    pub bus_ace_clock: *mut clk,
    pub regulators: *mut regulator_bulk_data,
    pub rstc: *mut reset_control,
// pm_domains for devices with more than one.
    pub pm_domain_devs: [*mut device; MAX_PM_DOMAINS],
    pub pm_domain_links: [*mut device_link; MAX_PM_DOMAINS],
    pub coherent: bool,
    pub features: panfrost_features,
    pub comp: *const panfrost_compatible,
    pub PANFROST_COMP_BIT_MAX): DECLARE_BITMAP(is_suspended,,
    pub as_lock: spinlock_t,
    pub as_alloc_mask: c_ulong,
    pub as_faulty_mask: c_ulong,
    pub as_lru_list: list_head,
    pub js: *mut panfrost_job_slot,
    pub jobs: [*mut panfrost_job; NUM_JOB_SLOTS][2],
    pub perfcnt: *mut panfrost_perfcnt,
    pub profile_mode: bool,
    pub sched_lock: mutex,
    pub wq: *mut workqueue_struct,
    pub work: work_struct,
    pub pending: core::sync::atomic::AtomicI32,
    pub reset: },
    pub shrinker_lock: mutex,
    pub shrinker_list: list_head,
    pub shrinker: *mut shrinker,
    pub pfdevfreq: panfrost_devfreq,
    pub use_count: core::sync::atomic::AtomicI32,
    pub lock: spinlock_t,
    pub cycle_counter: },

    pub debugfs: panfrost_device_debugfs,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct panfrost_mmu {
    pub pfdev: *mut panfrost_device,
    pub refcount: kref,
    pub pgtbl_cfg: io_pgtable_cfg,
    pub pgtbl_ops: *mut io_pgtable_ops,
    pub mm: drm_mm,
    pub mm_lock: spinlock_t,
    pub as: c_int,
    pub as_count: core::sync::atomic::AtomicI32,
    pub list: list_head,
    pub transtab: u64,
    pub memattr: u64,
    pub transcfg: u64,
    pub cfg: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct panfrost_engine_usage {
    pub elapsed_ns: [c_ulonglong; NUM_JOB_SLOTS],
    pub cycles: [c_ulonglong; NUM_JOB_SLOTS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct panfrost_file_priv {
    pub pfdev: *mut panfrost_device,
    pub jm_ctxs: xarray,
    pub mmu: *mut panfrost_mmu,
    pub engine_usage: panfrost_engine_usage,
}

// Higher priorities require CAP_SYS_NICE or DRM_MASTER
extern "C" {
    pub fn container_of(_arg: ddev, panfrost_device: struct, _arg: base) -> return;
}
extern "C" {
    pub fn panfrost_unstable_ioctl_check() -> c_int;
}
extern "C" {
    pub fn panfrost_device_init(pfdev: *mut panfrost_device) -> c_int;
}
extern "C" {
    pub fn panfrost_device_fini(pfdev: *mut panfrost_device);
}
extern "C" {
    pub fn panfrost_device_reset(pfdev: *mut panfrost_device, enable_job_int: bool);
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drm_panfrost_exception_type {
    DRM_PANFROST_EXCEPTION_OK = 0x00,
    DRM_PANFROST_EXCEPTION_DONE = 0x01,
    DRM_PANFROST_EXCEPTION_INTERRUPTED = 0x02,
    DRM_PANFROST_EXCEPTION_STOPPED = 0x03,
    DRM_PANFROST_EXCEPTION_TERMINATED = 0x04,
    DRM_PANFROST_EXCEPTION_KABOOM = 0x05,
    DRM_PANFROST_EXCEPTION_EUREKA = 0x06,
    DRM_PANFROST_EXCEPTION_ACTIVE = 0x08,
    DRM_PANFROST_EXCEPTION_MAX_NON_FAULT = 0x3f,
    DRM_PANFROST_EXCEPTION_JOB_CONFIG_FAULT = 0x40,
    DRM_PANFROST_EXCEPTION_JOB_POWER_FAULT = 0x41,
    DRM_PANFROST_EXCEPTION_JOB_READ_FAULT = 0x42,
    DRM_PANFROST_EXCEPTION_JOB_WRITE_FAULT = 0x43,
    DRM_PANFROST_EXCEPTION_JOB_AFFINITY_FAULT = 0x44,
    DRM_PANFROST_EXCEPTION_JOB_BUS_FAULT = 0x48,
    DRM_PANFROST_EXCEPTION_INSTR_INVALID_PC = 0x50,
    DRM_PANFROST_EXCEPTION_INSTR_INVALID_ENC = 0x51,
    DRM_PANFROST_EXCEPTION_INSTR_TYPE_MISMATCH = 0x52,
    DRM_PANFROST_EXCEPTION_INSTR_OPERAND_FAULT = 0x53,
    DRM_PANFROST_EXCEPTION_INSTR_TLS_FAULT = 0x54,
    DRM_PANFROST_EXCEPTION_INSTR_BARRIER_FAULT = 0x55,
    DRM_PANFROST_EXCEPTION_INSTR_ALIGN_FAULT = 0x56,
    DRM_PANFROST_EXCEPTION_DATA_INVALID_FAULT = 0x58,
    DRM_PANFROST_EXCEPTION_TILE_RANGE_FAULT = 0x59,
    DRM_PANFROST_EXCEPTION_ADDR_RANGE_FAULT = 0x5a,
    DRM_PANFROST_EXCEPTION_IMPRECISE_FAULT = 0x5b,
    DRM_PANFROST_EXCEPTION_OOM = 0x60,
    DRM_PANFROST_EXCEPTION_OOM_AFBC = 0x61,
    DRM_PANFROST_EXCEPTION_UNKNOWN = 0x7f,
    DRM_PANFROST_EXCEPTION_DELAYED_BUS_FAULT = 0x80,
    DRM_PANFROST_EXCEPTION_GPU_SHAREABILITY_FAULT = 0x88,
    DRM_PANFROST_EXCEPTION_SYS_SHAREABILITY_FAULT = 0x89,
    DRM_PANFROST_EXCEPTION_GPU_CACHEABILITY_FAULT = 0x8a,
    DRM_PANFROST_EXCEPTION_TRANSLATION_FAULT_0 = 0xc0,
    DRM_PANFROST_EXCEPTION_TRANSLATION_FAULT_1 = 0xc1,
    DRM_PANFROST_EXCEPTION_TRANSLATION_FAULT_2 = 0xc2,
    DRM_PANFROST_EXCEPTION_TRANSLATION_FAULT_3 = 0xc3,
    DRM_PANFROST_EXCEPTION_TRANSLATION_FAULT_4 = 0xc4,
    DRM_PANFROST_EXCEPTION_TRANSLATION_FAULT_IDENTITY = 0xc7,
    DRM_PANFROST_EXCEPTION_PERM_FAULT_0 = 0xc8,
    DRM_PANFROST_EXCEPTION_PERM_FAULT_1 = 0xc9,
    DRM_PANFROST_EXCEPTION_PERM_FAULT_2 = 0xca,
    DRM_PANFROST_EXCEPTION_PERM_FAULT_3 = 0xcb,
    DRM_PANFROST_EXCEPTION_TRANSTAB_BUS_FAULT_0 = 0xd0,
    DRM_PANFROST_EXCEPTION_TRANSTAB_BUS_FAULT_1 = 0xd1,
    DRM_PANFROST_EXCEPTION_TRANSTAB_BUS_FAULT_2 = 0xd2,
    DRM_PANFROST_EXCEPTION_TRANSTAB_BUS_FAULT_3 = 0xd3,
    DRM_PANFROST_EXCEPTION_ACCESS_FLAG_0 = 0xd8,
    DRM_PANFROST_EXCEPTION_ACCESS_FLAG_1 = 0xd9,
    DRM_PANFROST_EXCEPTION_ACCESS_FLAG_2 = 0xda,
    DRM_PANFROST_EXCEPTION_ACCESS_FLAG_3 = 0xdb,
    DRM_PANFROST_EXCEPTION_ADDR_SIZE_FAULT_IN0 = 0xe0,
    DRM_PANFROST_EXCEPTION_ADDR_SIZE_FAULT_IN1 = 0xe1,
    DRM_PANFROST_EXCEPTION_ADDR_SIZE_FAULT_IN2 = 0xe2,
    DRM_PANFROST_EXCEPTION_ADDR_SIZE_FAULT_IN3 = 0xe3,
    DRM_PANFROST_EXCEPTION_ADDR_SIZE_FAULT_OUT0 = 0xe4,
    DRM_PANFROST_EXCEPTION_ADDR_SIZE_FAULT_OUT1 = 0xe5,
    DRM_PANFROST_EXCEPTION_ADDR_SIZE_FAULT_OUT2 = 0xe6,
    DRM_PANFROST_EXCEPTION_ADDR_SIZE_FAULT_OUT3 = 0xe7,
    DRM_PANFROST_EXCEPTION_MEM_ATTR_FAULT_0 = 0xe8,
    DRM_PANFROST_EXCEPTION_MEM_ATTR_FAULT_1 = 0xe9,
    DRM_PANFROST_EXCEPTION_MEM_ATTR_FAULT_2 = 0xea,
    DRM_PANFROST_EXCEPTION_MEM_ATTR_FAULT_3 = 0xeb,
    DRM_PANFROST_EXCEPTION_MEM_ATTR_NONCACHE_0 = 0xec,
    DRM_PANFROST_EXCEPTION_MEM_ATTR_NONCACHE_1 = 0xed,
    DRM_PANFROST_EXCEPTION_MEM_ATTR_NONCACHE_2 = 0xee,
    DRM_PANFROST_EXCEPTION_MEM_ATTR_NONCACHE_3 = 0xef,
}
