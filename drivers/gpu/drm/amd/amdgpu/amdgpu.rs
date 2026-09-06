//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/amdgpu/amdgpu.h
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

pub const MAX_GPU_INSTANCE: c_int = 64;
pub const GFX_SLICE_PERIOD_MS: c_int = 250;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_gpu_instance {
    pub adev: *mut amdgpu_device,
    pub mgpu_fan_enabled: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_mgpu_info {
    pub gpu_ins: [amdgpu_gpu_instance; MAX_GPU_INSTANCE],
    pub mutex: mutex,
    pub num_gpu: u32,
    pub num_dgpu: u32,
    pub num_apu: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_hwip_reg_entry {
    pub hwip: u32,
    pub inst: u32,
    pub seg: u32,
    pub reg_offset: u32,
    pub reg_name: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_watchdog_timer {
    pub timeout_fatal_disable: bool,
    pub /: *mut *mut uint32_t period; / maxCycles = (1 << period), the number of cycles before a timeout,
}

pub const AMDGPU_MAX_TIMEOUT_PARAM_LENGTH: c_int = 256;
//
// Modules parameters.
//

pub const AMDGPU_WAIT_IDLE_TIMEOUT_IN_MS: c_int = 3000;

pub const AMDGPU_DEBUGFS_MAX_COMPONENTS: c_int = 32;
pub const AMDGPUFB_CONN_LIMIT: c_int = 4;
pub const AMDGPU_BIOS_NUM_SCRATCH: c_int = 16;

// hard reset data
pub const AMDGPU_ASIC_RESET_DATA: c_uint = 0x39d5e86b;
// reset flags

// reset mask

// max cursor sizes (in pixels)
pub const CIK_CURSOR_WIDTH: c_int = 128;
pub const CIK_CURSOR_HEIGHT: c_int = 128;
// smart shift bias level limits

// Extra time delay(in ms) to eliminate the influence of temperature momentary fluctuation
pub const AMDGPU_SWCTF_EXTRA_DELAY: c_int = 50;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amdgpu_cp_irq {
    AMDGPU_CP_IRQ_GFX_ME0_PIPE0_EOP = 0,
    AMDGPU_CP_IRQ_GFX_ME0_PIPE1_EOP,
    AMDGPU_CP_IRQ_COMPUTE_MEC1_PIPE0_EOP,
    AMDGPU_CP_IRQ_COMPUTE_MEC1_PIPE1_EOP,
    AMDGPU_CP_IRQ_COMPUTE_MEC1_PIPE2_EOP,
    AMDGPU_CP_IRQ_COMPUTE_MEC1_PIPE3_EOP,
    AMDGPU_CP_IRQ_COMPUTE_MEC2_PIPE0_EOP,
    AMDGPU_CP_IRQ_COMPUTE_MEC2_PIPE1_EOP,
    AMDGPU_CP_IRQ_COMPUTE_MEC2_PIPE2_EOP,
    AMDGPU_CP_IRQ_COMPUTE_MEC2_PIPE3_EOP,

    AMDGPU_CP_IRQ_LAST
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amdgpu_thermal_irq {
    AMDGPU_THERMAL_IRQ_LOW_TO_HIGH = 0,
    AMDGPU_THERMAL_IRQ_HIGH_TO_LOW,

    AMDGPU_THERMAL_IRQ_LAST
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amdgpu_kiq_irq {
    AMDGPU_CP_KIQ_IRQ_DRIVER0 = 0,
    AMDGPU_CP_KIQ_IRQ_LAST
}

pub const MAX_KIQ_REG_TRY: c_int = 1000;
//
// BIOS.
//
extern "C" {
    pub fn amdgpu_get_bios(adev: *mut amdgpu_device) -> bool;
}
extern "C" {
    pub fn amdgpu_read_bios(adev: *mut amdgpu_device) -> bool;
}
extern "C" {
    pub fn amdgpu_bios_release(adev: *mut amdgpu_device);
}
//
// Clocks
//
pub const AMDGPU_MAX_PPLL: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_clock {
    pub ppll: [amdgpu_pll; AMDGPU_MAX_PPLL],
    pub spll: amdgpu_pll,
    pub mpll: amdgpu_pll,
// 10 Khz units
    pub default_mclk: u32,
    pub default_sclk: u32,
    pub default_dispclk: u32,
    pub dp_extclk: u32,
    pub max_pixel_clock: u32,
}

//
// IRQS.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_flip_work {
    pub flip_work: delayed_work,
    pub unpin_work: work_struct,
    pub adev: *mut amdgpu_device,
    pub crtc_id: c_int,
    pub target_vblank: u32,
    pub base: u64,
    pub event: *mut drm_pending_vblank_event,
    pub old_abo: *mut amdgpu_bo,
    pub shared_count: unsigned,
    pub shared: *mut dma_fence,
    pub cb: dma_fence_cb,
    pub async: bool,
}

//
// file private structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_fpriv {
    pub vm: amdgpu_vm,
    pub prt_va: *mut amdgpu_bo_va,
    pub csa_va: *mut amdgpu_bo_va,
    pub seq64_va: *mut amdgpu_bo_va,
    pub bo_list_handles: xarray,
    pub ctx_mgr: amdgpu_ctx_mgr,
    pub userq_mgr: amdgpu_userq_mgr,
// Eviction fence infra
    pub evf_mgr: amdgpu_eviction_fence_mgr,
// GPU partition selection
    pub xcp_id: u32,
}

extern "C" {
    pub fn amdgpu_file_to_fpriv(filp: *mut file, fpriv: *mut amdgpu_fpriv) -> c_int;
}
//
// Benchmarking
//
extern "C" {
    pub fn amdgpu_benchmark(adev: *mut amdgpu_device, test_number: c_int) -> c_int;
}
//
// ASIC specific functions.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_asic_funcs {
    pub adev): *mut *mut bool (read_disabled_bios)(struct amdgpu_device,
    pub length_bytes): *mut *mut u8 bios, u32,
    pub value): *mut u32 sh_num, u32 reg_offset, u32,
    pub state): *mut *mut *mut void (set_vga_state)(struct amdgpu_device adev, bool,
    pub adev): *mut *mut int (reset)(struct amdgpu_device,
    pub adev): *mut *mut amd_reset_method (reset_method)(struct amdgpu_device,
// get the reference clock
    pub adev): *mut *mut u32 (get_xclk)(struct amdgpu_device,
// MM block clocks
    pub dclk): *mut *mut *mut int (set_uvd_clocks)(struct amdgpu_device adev, u32 vclk, u32,
    pub ecclk): *mut *mut *mut int (set_vce_clocks)(struct amdgpu_device adev, u32 evclk, u32,
// static power management
    pub adev): *mut *mut int (get_pcie_lanes)(struct amdgpu_device,
    pub lanes): *mut *mut *mut void (set_pcie_lanes)(struct amdgpu_device adev, int,
// get config memsize register
    pub adev): *mut *mut u32 (get_config_memsize)(struct amdgpu_device,
// flush hdp write queue
    pub ring): *mut *mut *mut void (flush_hdp)(struct amdgpu_device adev, struct amdgpu_ring,
// invalidate hdp read cache
    pub ring): *mut amdgpu_ring,
// initialize doorbell layout for specific asic
    pub adev): *mut *mut void (init_doorbell_index)(struct amdgpu_device,
// PCIe bandwidth usage
    pub count1): *mut u64,
// do we need to reset the asic at init time (e.g., kexec)
    pub adev): *mut *mut bool (need_reset_on_init)(struct amdgpu_device,
// PCIe replay counter
    pub adev): *mut *mut uint64_t (get_pcie_replay_count)(struct amdgpu_device,
// device supports BACO
    pub adev): *mut *mut int (supports_baco)(struct amdgpu_device,
// pre asic_init quirks
    pub adev): *mut *mut void (pre_asic_init)(struct amdgpu_device,
// enter/exit umd stable pstate
    pub enter): *mut *mut *mut int (update_umd_stable_pstate)(struct amdgpu_device adev, bool,
// query video codecs
    pub codecs): *const amdgpu_video_codecs,
// encode "> 32bits" smn addressing
    pub ext_id): *mut *mut u64 (encode_ext_smn_addressing)(int,
    pub max_size): usize,
}

//
// IOCTL.
//
extern "C" {
    pub fn amdgpu_cs_ioctl(dev: *mut drm_device, data: *mut c_void, filp: *mut drm_file) -> c_int;
}
extern "C" {
    pub fn amdgpu_cs_wait_ioctl(dev: *mut drm_device, data: *mut c_void, filp: *mut drm_file) -> c_int;
}
// VRAM scratch page for HDP bug, default vram page
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_mem_scratch {
    pub robj: *mut amdgpu_bo,
    pub ptr: *mut u32,
    pub gpu_addr: u64,
}

//
// CGS
//
extern "C" {
    pub fn amdgpu_cgs_destroy_device(cgs_device: *mut cgs_device);
}
//
// Core structure, functions and helpers.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_mmio_remap {
    pub reg_offset: u32,
    pub bus_addr: resource_size_t,
    pub bo: *mut amdgpu_bo,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amdgpu_uid_type {
    AMDGPU_UID_TYPE_XCD,
    AMDGPU_UID_TYPE_AID,
    AMDGPU_UID_TYPE_SOC,
    AMDGPU_UID_TYPE_MID,
    AMDGPU_UID_TYPE_MAX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_uid {
    pub uid: [u64; AMDGPU_UID_TYPE_MAX][AMDGPU_UID_INST_MAX],
    pub adev: *mut amdgpu_device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amd_powerplay {
    pub pp_handle: *mut c_void,
    pub pp_funcs: *const amd_pm_funcs,
}

// polaris10 kickers

// polaris11 kickers

// polaris12 kickers

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_pcie_reset_ctx {
    pub in_link_reset: bool,
    pub occurs_dpc: bool,
    pub audio_suspended: bool,
    pub swus: *mut pci_dev,
    pub swus_pcistate: *mut pci_saved_state,
    pub swds_pcistate: *mut pci_saved_state,
}

//
// Custom Init levels could be defined for different situations where a full
// initialization of all hardware blocks are not expected. Sample cases are
// custom init sequences after resume after S0i3/S3, reset on initialization,
// partial reset of blocks etc. Presently, this defines only two levels. Levels
// are described in corresponding struct definitions - amdgpu_init_default,
// amdgpu_init_minimal_xgmi.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amdgpu_init_lvl_id {
    AMDGPU_INIT_LEVEL_DEFAULT,
    AMDGPU_INIT_LEVEL_MINIMAL_XGMI,
    AMDGPU_INIT_LEVEL_RESET_RECOVERY,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_init_level {
    pub level: amdgpu_init_lvl_id,
    pub hwini_ip_block_mask: u32,
}

pub const AMDGPU_RESET_MAGIC_NUM: c_int = 64;
pub const AMDGPU_MAX_DF_PERFMONS: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amdgpu_enforce_isolation_mode {
    AMDGPU_ENFORCE_ISOLATION_DISABLE = 0,
    AMDGPU_ENFORCE_ISOLATION_ENABLE = 1,
    AMDGPU_ENFORCE_ISOLATION_ENABLE_LEGACY = 2,
    AMDGPU_ENFORCE_ISOLATION_NO_CLEANER_SHADER = 3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_device {
    pub dev: *mut device,
    pub pdev: *mut pci_dev,
    pub ddev: drm_device,

    pub acp: amdgpu_acp,

    pub hive: *mut amdgpu_hive_info,
    pub xcp_mgr: *mut amdgpu_xcp_mgr,
// ASIC
    pub asic_type: amd_asic_type,
    pub family: u32,
    pub rev_id: u32,
    pub external_rev_id: u32,
    pub flags: c_ulong,
    pub apu_flags: c_ulong,
    pub usec_timeout: c_int,
    pub asic_funcs: *const amdgpu_asic_funcs,
    pub shutdown: bool,
    pub need_swiotlb: bool,
    pub accel_working: bool,
    pub acpi_nb: notifier_block,
    pub pm_nb: notifier_block,
    pub i2c_bus: [*mut amdgpu_i2c_chan; AMDGPU_MAX_I2C_BUS],
    pub debugfs_vbios_blob: debugfs_blob_wrapper,
    pub srbm_mutex: mutex,
// GRBM index mutex. Protects concurrent access to GRBM index
    pub grbm_idx_mutex: mutex,
    pub vga_pm_domain: dev_pm_domain,
    pub have_disp_power_ref: bool,
    pub have_atomics_support: bool,
    pub is_sw_smu: bool,
// BIOS
    pub is_atom_fw: bool,
    pub bios: *mut u8,
    pub bios_size: u32,
    pub bios_scratch_reg_offset: u32,
    pub bios_scratch: [u32; AMDGPU_BIOS_NUM_SCRATCH],
// Register/doorbell mmio
    pub rmmio_base: resource_size_t,
    pub rmmio_size: resource_size_t,
    pub rmmio: *mut void __iomem,
// protects concurrent MM_INDEX/DATA based register access
    pub mmio_idx_lock: spinlock_t,
    pub rmmio_remap: amdgpu_mmio_remap,
// Indirect register access blocks
    pub reg: amdgpu_reg_access,
    pub doorbell: amdgpu_doorbell,
// clock/pll info
    pub clock: amdgpu_clock,
// MC
    pub gmc: amdgpu_gmc,
    pub gart: amdgpu_gart,
    pub dummy_page_addr: dma_addr_t,
    pub vm_manager: amdgpu_vm_manager,
    pub vmhub: [amdgpu_vmhub; AMDGPU_MAX_VMHUBS],
    pub AMDGPU_MAX_VMHUBS): DECLARE_BITMAP(vmhubs_mask,,
// memory management
    pub mman: amdgpu_mman,
    pub mem_scratch: amdgpu_mem_scratch,
    pub wb: amdgpu_wb,
    pub num_bytes_moved: core::sync::atomic::AtomicI64,
    pub num_evictions: core::sync::atomic::AtomicI64,
    pub num_vram_cpu_page_faults: core::sync::atomic::AtomicI64,
    pub gpu_reset_counter: core::sync::atomic::AtomicI32,
    pub vram_lost_counter: core::sync::atomic::AtomicI32,
// data for buffer migration throttling
    pub lock: spinlock_t,
    pub last_update_us: i64,
    pub /: *mut *mut s64 accum_us; / accumulated microseconds,
    pub /: *mut *mut s64 accum_us_vis; / for visible VRAM,
    pub log2_max_MBps: u32,
    pub mm_stats: },
// discovery
    pub discovery: amdgpu_discovery_info,
// display
    pub enable_virtual_display: bool,
    pub amdgpu_vkms_output: *mut amdgpu_vkms_output,
    pub mode_info: amdgpu_mode_info,
// For pre-DCE11. DCE11 and later are in "struct amdgpu_device->dm"
    pub hotplug_work: delayed_work,
    pub crtc_irq: amdgpu_irq_src,
    pub vline0_irq: amdgpu_irq_src,
    pub vupdate_irq: amdgpu_irq_src,
    pub pageflip_irq: amdgpu_irq_src,
    pub hpd_irq: amdgpu_irq_src,
    pub dmub_trace_irq: amdgpu_irq_src,
    pub dmub_outbox_irq: amdgpu_irq_src,
// rings
    pub fence_context: u64,
    pub num_rings: unsigned,
    pub rings: [*mut amdgpu_ring; AMDGPU_MAX_RINGS],
    pub gang_submit: *mut dma_fence __rcu,
    pub ib_pool_ready: bool,
    pub ib_pools: [amdgpu_sa_manager; AMDGPU_IB_POOL_MAX],
    pub gpu_sched: [amdgpu_sched; AMDGPU_HW_IP_NUM][AMDGPU_RING_PRIO_MAX],
// interrupts
    pub irq: amdgpu_irq,
// powerplay
    pub powerplay: amd_powerplay,
    pub pm: amdgpu_pm,
    pub cg_flags: u64,
    pub pg_flags: u32,
// nbio
    pub nbio: amdgpu_nbio,
// hdp
    pub hdp: amdgpu_hdp,
// smuio
    pub smuio: amdgpu_smuio,
// mmhub
    pub mmhub: amdgpu_mmhub,
// gfxhub
    pub gfxhub: amdgpu_gfxhub,
// gfx
    pub gfx: amdgpu_gfx,
// sdma
    pub sdma: amdgpu_sdma,
// lsdma
    pub lsdma: amdgpu_lsdma,
// uvd
    pub uvd: amdgpu_uvd,
// vce
    pub vce: amdgpu_vce,
// vcn
    pub vcn: amdgpu_vcn,
// jpeg
    pub jpeg: amdgpu_jpeg,
// vpe
    pub vpe: amdgpu_vpe,
// umsch
    pub umsch_mm: amdgpu_umsch_mm,
    pub enable_umsch_mm: bool,
// firmwares
    pub firmware: amdgpu_firmware,
// PSP
    pub psp: psp_context,
// GDS
    pub gds: amdgpu_gds,
// for userq and VM fences
    pub seq64: amdgpu_seq64,
// UMC
    pub umc: amdgpu_umc,
// display related functionality
    pub dm: amdgpu_display_manager,

// isp
    pub isp: amdgpu_isp,

// mes
    pub enable_mes: bool,
    pub enable_mes_kiq: bool,
    pub enable_uni_mes: bool,
    pub mes: amdgpu_mes,
    pub mqds: [amdgpu_mqd; AMDGPU_HW_IP_NUM],
    pub userq_funcs: [*const amdgpu_userq_funcs; AMDGPU_HW_IP_NUM],
//
// @userq_doorbell_xa: Global user queue map (doorbell index → queue)
// Key: doorbell_index (unique global identifier for the queue)
// Value: struct amdgpu_usermode_queue
//
    pub userq_doorbell_xa: xarray,
// df
    pub df: amdgpu_df,
// MCA
    pub mca: amdgpu_mca,
// CPER
    pub cper: amdgpu_cper,
    pub ip_blocks: [amdgpu_ip_block; AMDGPU_MAX_IP_NUM],
    pub harvest_ip_mask: u32,
    pub num_ip_blocks: c_int,
    pub mn_lock: mutex,
    pub 7): DECLARE_HASHTABLE(mn_hash,,
// tracking pinned memory
    pub vram_pin_size: core::sync::atomic::AtomicI64,
    pub visible_pin_size: core::sync::atomic::AtomicI64,
    pub gart_pin_size: core::sync::atomic::AtomicI64,
// soc15 register offset based on ip, instance and  segment
    pub reg_offset: [*mut u32; MAX_HWIP][HWIP_MAX_INSTANCE],
    pub ip_map: amdgpu_ip_map_info,
// delayed work_func for deferring clockgating during resume
    pub delayed_init_work: delayed_work,
    pub virt: amdgpu_virt,
// record hw reset is performed
    pub has_hw_reset: bool,
    pub reset_magic: [u8; AMDGPU_RESET_MAGIC_NUM],
// s3/s4 mask
    pub in_suspend: bool,
    pub in_s3: bool,
    pub in_s4: bool,
    pub in_s0ix: bool,
    pub last_suspend_state: suspend_state_t,
    pub mp1_state: pp_mp1_state,
    pub doorbell_index: amdgpu_doorbell_index,
    pub notifier_lock: mutex,
    pub asic_reset_res: c_int,
    pub xgmi_reset_work: work_struct,
    pub reset_list: list_head,
    pub gfx_timeout: c_long,
    pub sdma_timeout: c_long,
    pub video_timeout: c_long,
    pub compute_timeout: c_long,
    pub psp_timeout: c_long,
    pub unique_id: u64,
    pub unitid: u8,
    pub df_perfmon_config_assign_mask: [u64; AMDGPU_MAX_DF_PERFMONS],
// enable runtime pm on the device
    pub in_runpm: bool,
    pub has_pr3: bool,
    pub ucode_sysfs_en: bool,
    pub fru_info: *mut amdgpu_fru_info,
    pub throttling_logging_enabled: core::sync::atomic::AtomicI32,
    pub throttling_logging_rs: ratelimit_state,
    pub ras_hw_enabled: u32,
    pub ras_enabled: u32,
    pub ras_default_ecc_enabled: bool,
    pub no_hw_access: bool,
    pub pci_state: *mut pci_saved_state,
    pub pci_channel_state: pci_channel_state_t,
    pub pcie_reset_ctx: amdgpu_pcie_reset_ctx,
// Track auto wait count on s_barrier settings
    pub barrier_has_auto_waitcnt: bool,
    pub reset_cntl: *mut amdgpu_reset_control,
    pub ip_versions: [u32; MAX_HWIP][HWIP_MAX_INSTANCE],
    pub ram_is_direct_mapped: bool,
    pub ras_list: list_head,
    pub reset_domain: *mut amdgpu_reset_domain,

    pub coredump: *mut amdgpu_coredump_info,
    pub coredump_work: work_struct,

    pub benchmark_mutex: mutex,
    pub scpm_enabled: bool,
    pub scpm_status: u32,
    pub reset_work: work_struct,
    pub dc_enabled: bool,
// Mask of active clusters
    pub aid_mask: u32,
// Debug
    pub debug_vm: bool,
    pub debug_largebar: bool,
    pub debug_disable_soft_recovery: bool,
    pub debug_use_vram_fw_buf: bool,
    pub debug_exp_resets: bool,
    pub debug_disable_gpu_ring_reset: bool,
    pub debug_vm_userptr: bool,
    pub debug_disable_ce_logs: bool,
    pub debug_enable_ce_cs: bool,
    pub debug_hibernation_thaw_resume_gpu: bool,
    pub debug_disable_ip_block_soft_reset: bool,
// Protection for the following isolation structure
    pub enforce_isolation_mutex: mutex,
    pub enforce_isolation: [amdgpu_enforce_isolation_mode; MAX_XCP],
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_isolation {
    pub owner: *mut c_void,
    pub spearhead: *mut dma_fence,
    pub active: amdgpu_sync,
    pub prev: amdgpu_sync,
    pub isolation: [}; MAX_XCP],
    pub init_lvl: *mut amdgpu_init_level,
// This flag is used to determine how VRAM allocations are handled for APUs
// in KFD: VRAM or GTT.
//
    pub apu_prefer_gtt: bool,
    pub userq_halt_for_enforce_isolation: bool,
    pub uid_info: *mut amdgpu_uid,
    pub uma_info: amdgpu_uma_carveout_info,
// KFD
// Must be last --ends in a flexible-array member.
//
    pub kfd: amdgpu_kfd_dev,
}

// This considers only major/minor/rev and ignores
// subrevision/variant fields.
//
// This returns full version - major/minor/rev/variant/subrevision
extern "C" {
    pub fn container_of(_arg: ddev, amdgpu_device: struct, _arg: ddev) -> return;
}
extern "C" {
    pub fn container_of(_arg: bdev, amdgpu_device: struct, _arg: mman.bdev) -> return;
}
extern "C" {
    pub fn amdgpu_device_fini_hw(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_device_fini_sw(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_gpu_wait_for_idle(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_device_get_rev_id(adev: *mut amdgpu_device) -> u32;
}
extern "C" {
    pub fn amdgpu_device_has_dc_support(adev: *mut amdgpu_device) -> bool;
}
extern "C" {
    pub fn amdgpu_device_set_sriov_virtual_display(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_device_reinit_after_reset(reset_context: *mut amdgpu_reset_context) -> c_int;
}
extern "C" {
    pub fn emu_soc_asic_init(adev: *mut amdgpu_device) -> c_int;
}
//
// Registers read & write functions.
//

//
// BIOS helpers.
//

//
// ASICs macro.
//

// Common functions
extern "C" {
    pub fn amdgpu_device_has_job_running(adev: *mut amdgpu_device) -> bool;
}
extern "C" {
    pub fn amdgpu_device_should_recover_gpu(adev: *mut amdgpu_device) -> bool;
}
extern "C" {
    pub fn amdgpu_device_pci_config_reset(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_device_pci_reset(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_device_need_post(adev: *mut amdgpu_device) -> bool;
}
extern "C" {
    pub fn amdgpu_device_seamless_boot_supported(adev: *mut amdgpu_device) -> bool;
}
extern "C" {
    pub fn amdgpu_device_should_use_aspm(adev: *mut amdgpu_device) -> bool;
}
extern "C" {
    pub fn amdgpu_device_resize_fb_bar(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_device_mode1_reset(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_device_link_reset(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_device_supports_atpx(adev: *mut amdgpu_device) -> bool;
}
extern "C" {
    pub fn amdgpu_device_supports_px(adev: *mut amdgpu_device) -> bool;
}
extern "C" {
    pub fn amdgpu_device_supports_boco(adev: *mut amdgpu_device) -> bool;
}
extern "C" {
    pub fn amdgpu_device_supports_smart_shift(adev: *mut amdgpu_device) -> bool;
}
extern "C" {
    pub fn amdgpu_device_supports_baco(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_device_detect_runtime_pm_mode(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_device_baco_enter(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_device_baco_exit(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_device_halt(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_device_has_display_hardware(adev: *mut amdgpu_device) -> bool;
}
extern "C" {
    pub fn amdgpu_get_soft_full_reset_mask(ring: *mut amdgpu_ring) -> isize;
}
extern "C" {
    pub fn amdgpu_show_reset_mask(buf: *mut c_char, supported_reset: u32) -> isize;
}
// atpx handler

extern "C" {
    pub fn amdgpu_register_atpx_handler();
}
extern "C" {
    pub fn amdgpu_unregister_atpx_handler();
}
extern "C" {
    pub fn amdgpu_has_atpx_dgpu_power_cntl() -> bool;
}
extern "C" {
    pub fn amdgpu_is_atpx_hybrid() -> bool;
}
extern "C" {
    pub fn amdgpu_has_atpx() -> bool;
}

//
// KMS
//
extern "C" {
    pub fn amdgpu_driver_load_kms(adev: *mut amdgpu_device, flags: c_ulong) -> c_int;
}
extern "C" {
    pub fn amdgpu_driver_unload_kms(dev: *mut drm_device);
}
extern "C" {
    pub fn amdgpu_driver_open_kms(dev: *mut drm_device, file_priv: *mut drm_file) -> c_int;
}
extern "C" {
    pub fn amdgpu_driver_release_kms(dev: *mut drm_device);
}
extern "C" {
    pub fn amdgpu_device_prepare(dev: *mut drm_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_device_complete(dev: *mut drm_device);
}
extern "C" {
    pub fn amdgpu_device_suspend(dev: *mut drm_device, fbcon: bool) -> c_int;
}
extern "C" {
    pub fn amdgpu_device_resume(dev: *mut drm_device, fbcon: bool) -> c_int;
}
extern "C" {
    pub fn amdgpu_get_vblank_counter_kms(crtc: *mut drm_crtc) -> u32;
}
extern "C" {
    pub fn amdgpu_enable_vblank_kms(crtc: *mut drm_crtc) -> c_int;
}
extern "C" {
    pub fn amdgpu_disable_vblank_kms(crtc: *mut drm_crtc);
}
//
// functions used by amdgpu_encoder.c
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_afmt_acr {
    pub clock: u32,
    pub n_32khz: c_int,
    pub cts_32khz: c_int,
    pub n_44_1khz: c_int,
    pub cts_44_1khz: c_int,
    pub n_48khz: c_int,
    pub cts_48khz: c_int,
}

extern "C" {
    pub fn amdgpu_afmt_acr(clock: u32) -> amdgpu_afmt_acr;
}
extern "C" {
    pub fn amdgpu_register_gpu_instance(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_unregister_gpu_instance(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_pci_mmio_enabled(pdev: *mut pci_dev) -> pci_ers_result_t;
}
extern "C" {
    pub fn amdgpu_pci_slot_reset(pdev: *mut pci_dev) -> pci_ers_result_t;
}
extern "C" {
    pub fn amdgpu_pci_resume(pdev: *mut pci_dev);
}
extern "C" {
    pub fn amdgpu_device_cache_pci_state(pdev: *mut pci_dev) -> bool;
}
extern "C" {
    pub fn amdgpu_device_load_pci_state(pdev: *mut pci_dev) -> bool;
}
extern "C" {
    pub fn amdgpu_device_skip_hw_access(adev: *mut amdgpu_device) -> bool;
}

extern "C" {
    pub fn amdgpu_in_reset(adev: *mut amdgpu_device) -> c_int;
}
