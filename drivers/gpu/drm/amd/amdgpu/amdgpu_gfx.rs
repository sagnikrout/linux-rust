//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/amdgpu/amdgpu_gfx.h
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
// GFX stuff
//

// GFX current status
pub const AMDGPU_GFX_NORMAL_MODE: c_uint = 0x00000000L;
pub const AMDGPU_GFX_SAFE_MODE: c_uint = 0x00000001L;
pub const AMDGPU_GFX_PG_DISABLED_MODE: c_uint = 0x00000002L;
pub const AMDGPU_GFX_CG_DISABLED_MODE: c_uint = 0x00000004L;
pub const AMDGPU_GFX_LBPW_DISABLED_MODE: c_uint = 0x00000008L;
pub const AMDGPU_MAX_GC_INSTANCES: c_int = 8;
pub const AMDGPU_MAX_QUEUES: c_int = 128;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amdgpu_gfx_pipe_priority {
    AMDGPU_GFX_PIPE_PRIO_NORMAL = AMDGPU_RING_PRIO_1,
    AMDGPU_GFX_PIPE_PRIO_HIGH = AMDGPU_RING_PRIO_2
}

pub const AMDGPU_GFX_QUEUE_PRIORITY_MINIMUM: c_int = 0;
pub const AMDGPU_GFX_QUEUE_PRIORITY_MAXIMUM: c_int = 15;
// 1 second timeout

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amdgpu_gfx_partition {
    AMDGPU_SPX_PARTITION_MODE = 0,
    AMDGPU_DPX_PARTITION_MODE = 1,
    AMDGPU_TPX_PARTITION_MODE = 2,
    AMDGPU_QPX_PARTITION_MODE = 3,
    AMDGPU_CPX_PARTITION_MODE = 4,
    AMDGPU_UNKNOWN_COMPUTE_PARTITION_MODE = -1,
// Automatically choose the right mode
    AMDGPU_AUTO_COMPUTE_PARTITION_MODE = -2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amdgpu_gfx_partition_mem_alloc_mode {
    AMDGPU_PARTITION_MEM_CAPPING_EVEN = 0,
    AMDGPU_PARTITION_MEM_ALLOC_ALL  = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amdgpu_gfx_ras_mem_id_type {
    AMDGPU_GFX_CP_MEM = 0,
    AMDGPU_GFX_GCEA_MEM,
    AMDGPU_GFX_GC_CANE_MEM,
    AMDGPU_GFX_GCUTCL2_MEM,
    AMDGPU_GFX_GDS_MEM,
    AMDGPU_GFX_LDS_MEM,
    AMDGPU_GFX_RLC_MEM,
    AMDGPU_GFX_SP_MEM,
    AMDGPU_GFX_SPI_MEM,
    AMDGPU_GFX_SQC_MEM,
    AMDGPU_GFX_SQ_MEM,
    AMDGPU_GFX_TA_MEM,
    AMDGPU_GFX_TCC_MEM,
    AMDGPU_GFX_TCA_MEM,
    AMDGPU_GFX_TCI_MEM,
    AMDGPU_GFX_TCP_MEM,
    AMDGPU_GFX_TD_MEM,
    AMDGPU_GFX_TCX_MEM,
    AMDGPU_GFX_ATC_L2_MEM,
    AMDGPU_GFX_UTCL2_MEM,
    AMDGPU_GFX_VML2_MEM,
    AMDGPU_GFX_VML2_WALKER_MEM,
    AMDGPU_GFX_MEM_TYPE_NUM
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_mec {
    pub hpd_eop_obj: *mut amdgpu_bo,
    pub hpd_eop_gpu_addr: u64,
    pub mec_fw_obj: *mut amdgpu_bo,
    pub mec_fw_gpu_addr: u64,
    pub mec_fw_data_obj: *mut amdgpu_bo,
    pub mec_fw_data_gpu_addr: u64,
    pub num_mec: u32,
    pub num_pipe_per_mec: u32,
    pub num_queue_per_pipe: u32,
    pub AMDGPU_MAX_GC_INSTANCES]: *mut *mut *mut void mqd_backup[AMDGPU_MAX_COMPUTE_RINGS,
    pub use_mmio_for_reset: bool,
    pub mes_hung_db_array: *mut u32,
    pub reset_mutex: mutex,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_mec_bitmap {
// These are the resources for which amdgpu takes ownership
    pub AMDGPU_MAX_COMPUTE_QUEUES): DECLARE_BITMAP(queue_bitmap,,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amdgpu_unmap_queues_action {
    PREEMPT_QUEUES = 0,
    RESET_QUEUES,
    DISABLE_PROCESS_QUEUES,
    PREEMPT_QUEUES_NO_UNMAP,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kiq_pm4_funcs {
// Support ASIC-specific kiq pm4 packets
    pub queue_mask): u64,
    pub ring): *mut amdgpu_ring,
    pub seq): u64 gpu_addr, u64,
    pub seq): u64,
    pub all_hub): bool,
    pub vmid): uint32_t xcc_id, uint32_t,
// Packet sizes
    pub set_resources_size: c_int,
    pub map_queues_size: c_int,
    pub unmap_queues_size: c_int,
    pub query_status_size: c_int,
    pub invalidate_tlbs_size: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_kiq {
    pub eop_gpu_addr: u64,
    pub eop_obj: *mut amdgpu_bo,
    pub ring_lock: spinlock_t,
    pub ring: amdgpu_ring,
    pub irq: amdgpu_irq_src,
    pub pmf: *const kiq_pm4_funcs,
    pub mqd_backup: *mut c_void,
}

//
// GFX configurations
//
pub const AMDGPU_GFX_MAX_SE: c_int = 4;
pub const AMDGPU_GFX_MAX_SH_PER_SE: c_int = 2;
//
// amdgpu_rb_config - Configure a single Render Backend (RB)
//
// Bad RBs are fused off and there is a harvest register the driver reads to
// determine which RB(s) are fused off so that the driver can configure the
// hardware state so that nothing gets sent to them. There are also user
// harvest registers that the driver can program to disable additional RBs,
// etc., for testing purposes.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_rb_config {
//
// @rb_backend_disable:
//
// The value captured from register RB_BACKEND_DISABLE indicates if the
// RB backend is disabled or not.
//
    pub rb_backend_disable: u32,
//
// @user_rb_backend_disable:
//
// The value captured from register USER_RB_BACKEND_DISABLE indicates
// if the User RB backend is disabled or not.
//
    pub user_rb_backend_disable: u32,
//
// @raster_config:
//
// To set up all of the states, it is necessary to have two registers
// to keep all of the states. This field holds the first register.
//
    pub raster_config: u32,
//
// @raster_config_1:
//
// To set up all of the states, it is necessary to have two registers
// to keep all of the states. This field holds the second register.
//
    pub raster_config_1: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_addr_config {
    pub pipe_interleave_size: u16,
    pub num_pipes: u8,
    pub max_compress_frags: u8,
    pub num_banks: u8,
    pub num_se: u8,
    pub num_rb_per_se: u8,
    pub num_pkrs: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_gfx_config {
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
    pub mem_max_burst_length_bytes: unsigned,
    pub mem_row_size_in_kb: unsigned,
    pub shader_engine_tile_size: unsigned,
    pub num_gpus: unsigned,
    pub multi_gpu_tile_size: unsigned,
    pub mc_arb_ramcfg: unsigned,
    pub num_banks: unsigned,
    pub num_ranks: unsigned,
    pub gb_addr_config: unsigned,
    pub num_rbs: unsigned,
    pub gs_vgt_table_depth: unsigned,
    pub gs_prim_buffer_depth: unsigned,
    pub tile_mode_array: [u32; 32],
    pub macrotile_mode_array: [u32; 16],
    pub gb_addr_config_fields: gb_addr_config,
//
// @rb_config:
//
// Matrix that keeps all the Render Backend (color and depth buffer
// handling) configuration on the 3D engine.
//
    pub rb_config: [amdgpu_rb_config; AMDGPU_GFX_MAX_SE][AMDGPU_GFX_MAX_SH_PER_SE],
// gfx configure feature
    pub double_offchip_lds_buf: u32,
// cached value of DB_DEBUG2
    pub db_debug2: u32,
// gfx10 specific config
    pub num_sc_per_sh: u32,
    pub num_packer_per_sc: u32,
    pub pa_sc_tile_steering_override: u32,
// Whether texture coordinate truncation is conformant.
    pub ta_cntl2_truncate_coord_mode: bool,
    pub tcc_disabled_mask: u64,
    pub gc_num_tcp_per_sa: u32,
    pub gc_num_sdp_interface: u32,
    pub gc_num_tcps: u32,
    pub gc_num_tcp_per_wpg: u32,
    pub gc_tcp_l1_size: u32,
    pub gc_num_sqc_per_wgp: u32,
    pub gc_l1_instruction_cache_size_per_sqc: u32,
    pub gc_l1_data_cache_size_per_sqc: u32,
    pub gc_gl1c_per_sa: u32,
    pub gc_gl1c_size_per_instance: u32,
    pub gc_gl2c_per_gpu: u32,
    pub gc_tcp_size_per_cu: u32,
    pub gc_num_cu_per_sqc: u32,
    pub gc_tcc_size: u32,
    pub gc_tcp_cache_line_size: u32,
    pub gc_instruction_cache_size_per_sqc: u32,
    pub gc_instruction_cache_line_size: u32,
    pub gc_scalar_data_cache_size_per_sqc: u32,
    pub gc_scalar_data_cache_line_size: u32,
    pub gc_tcc_cache_line_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_cu_info {
    pub simd_per_cu: u32,
    pub max_waves_per_simd: u32,
    pub wave_front_size: u32,
    pub max_scratch_slots_per_cu: u32,
    pub lds_size: u32,
// total active CU number
    pub number: u32,
    pub ao_cu_mask: u32,
    pub ao_cu_bitmap: [u32; 4][4],
    pub bitmap: [u32; AMDGPU_MAX_GC_INSTANCES][4][4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_gfx_ras {
    pub ras_block: amdgpu_ras_block_object,
    pub adev): *mut *mut void (enable_watchdog_timer)(struct amdgpu_device,
    pub entry): *mut amdgpu_iv_entry,
    pub entry): *mut amdgpu_iv_entry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_gfx_shadow_info {
    pub shadow_size: u32,
    pub shadow_alignment: u32,
    pub csa_size: u32,
    pub csa_alignment: u32,
    pub eop_size: u32,
    pub eop_alignment: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_gfx_funcs {
// get the gpu clock counter
    pub adev): *mut *mut uint64_t (get_gpu_clock_counter)(struct amdgpu_device,
    pub xcc_id): u32 sh_num, u32 instance, int,
    pub no_fields): *mut *mut uint32_t wave, uint32_t dst, int,
    pub dst): *mut uint32_t size, uint32_t,
    pub dst): *mut u32,
    pub xcc_id): u32 queue, u32 vmid, u32,
    pub adev): *mut *mut void (init_spm_golden)(struct amdgpu_device,
    pub enable): *mut *mut *mut void (update_perfmon_mgcg)(struct amdgpu_device adev, bool,
    pub skip_check): bool,
    pub adev): *mut *mut (query_partition_mode)(struct amdgpu_device,
    pub num_xccs_per_xcp): c_int,
    pub ih_node): *mut *mut *mut int (ih_node_to_logical_xcc)(struct amdgpu_device adev, int,
    pub adev): *mut *mut int (get_xccs_per_xcp)(struct amdgpu_device,
    pub reg_mem_engine): *mut *mut uint32_t ref_and_mask, uint32_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sq_work {
    pub work: work_struct,
    pub ih_data: unsigned,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_pfp {
    pub pfp_fw_obj: *mut amdgpu_bo,
    pub pfp_fw_gpu_addr: u64,
    pub pfp_fw_ptr: *mut u32,
    pub pfp_fw_data_obj: *mut amdgpu_bo,
    pub pfp_fw_data_gpu_addr: u64,
    pub pfp_fw_data_ptr: *mut u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_ce {
    pub ce_fw_obj: *mut amdgpu_bo,
    pub ce_fw_gpu_addr: u64,
    pub ce_fw_ptr: *mut u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_me {
    pub me_fw_obj: *mut amdgpu_bo,
    pub me_fw_gpu_addr: u64,
    pub me_fw_ptr: *mut u32,
    pub me_fw_data_obj: *mut amdgpu_bo,
    pub me_fw_data_gpu_addr: u64,
    pub me_fw_data_ptr: *mut u32,
    pub num_me: u32,
    pub num_pipe_per_me: u32,
    pub num_queue_per_pipe: u32,
    pub mqd_backup: [*mut c_void; AMDGPU_MAX_GFX_RINGS],
    pub use_mmio_for_reset: bool,
// These are the resources for which amdgpu takes ownership
    pub AMDGPU_MAX_GFX_QUEUES): DECLARE_BITMAP(queue_bitmap,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_isolation_work {
    pub adev: *mut amdgpu_device,
    pub xcp_id: u32,
    pub work: delayed_work,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_gfx {
    pub gpu_clock_mutex: mutex,
    pub config: amdgpu_gfx_config,
    pub rlc: amdgpu_rlc,
    pub pfp: amdgpu_pfp,
    pub ce: amdgpu_ce,
    pub me: amdgpu_me,
    pub mec: amdgpu_mec,
    pub mec_bitmap: [amdgpu_mec_bitmap; AMDGPU_MAX_GC_INSTANCES],
    pub kiq: [amdgpu_kiq; AMDGPU_MAX_GC_INSTANCES],
    pub imu: amdgpu_imu,
    pub /: *mut *mut bool rs64_enable; / firmware format,
    pub /: *const *const *const firmware me_fw; / ME firmware,
    pub me_fw_version: u32,
    pub /: *const *const *const firmware pfp_fw; / PFP firmware,
    pub pfp_fw_version: u32,
    pub /: *const *const *const firmware ce_fw; / CE firmware,
    pub ce_fw_version: u32,
    pub /: *const *const *const firmware rlc_fw; / RLC firmware,
    pub rlc_fw_version: u32,
    pub /: *const *const *const firmware mec_fw; / MEC firmware,
    pub mec_fw_version: u32,
    pub /: *const *const *const firmware mec2_fw; / MEC2 firmware,
    pub mec2_fw_version: u32,
    pub /: *const *const *const firmware imu_fw; / IMU firmware,
    pub imu_fw_version: u32,
    pub me_feature_version: u32,
    pub ce_feature_version: u32,
    pub pfp_feature_version: u32,
    pub rlc_feature_version: u32,
    pub rlc_srlc_fw_version: u32,
    pub rlc_srlc_feature_version: u32,
    pub rlc_srlg_fw_version: u32,
    pub rlc_srlg_feature_version: u32,
    pub rlc_srls_fw_version: u32,
    pub rlc_srls_feature_version: u32,
    pub rlcp_ucode_version: u32,
    pub rlcp_ucode_feature_version: u32,
    pub rlcv_ucode_version: u32,
    pub rlcv_ucode_feature_version: u32,
    pub mec_feature_version: u32,
    pub mec2_feature_version: u32,
    pub mec_fw_write_wait: bool,
    pub me_fw_write_wait: bool,
    pub cp_fw_write_wait: bool,
    pub gfx_ring: [amdgpu_ring; AMDGPU_MAX_GFX_RINGS],
    pub num_gfx_rings: unsigned,
    pub AMDGPU_MAX_GC_INSTANCES]: *mut *mut amdgpu_ring compute_ring[AMDGPU_MAX_COMPUTE_RINGS,
    pub num_compute_rings: unsigned,
    pub eop_irq: amdgpu_irq_src,
    pub priv_reg_irq: amdgpu_irq_src,
    pub priv_inst_irq: amdgpu_irq_src,
    pub bad_op_irq: amdgpu_irq_src,
    pub cp_ecc_error_irq: amdgpu_irq_src,
    pub sq_irq: amdgpu_irq_src,
    pub rlc_gc_fed_irq: amdgpu_irq_src,
    pub rlc_poison_irq: amdgpu_irq_src,
    pub sq_work: sq_work,
// gfx status
    pub gfx_current_status: u32,
// ce ram size
    pub ce_ram_size: unsigned,
    pub cu_info: amdgpu_cu_info,
    pub funcs: *const amdgpu_gfx_funcs,
// reset mask
    pub gfx_supported_reset: u32,
    pub compute_supported_reset: u32,
// gfx off
    pub /: *mut *mut bool gfx_off_state; / true: enabled, false: disabled,
    pub /: *mut *mut mutex gfx_off_mutex; / mutex to change gfxoff state,
    pub /: *mut *mut uint32_t gfx_off_req_count; / default 1, enable gfx off: dec 1, disable gfx off: add 1,
    pub /: *mut *mut delayed_work gfx_off_delay_work; / async work to set gfx block off,
    pub /: *mut *mut uint32_t gfx_off_residency; / last logged residency,
    pub /: *mut *mut uint64_t gfx_off_entrycount; / count of times GPU has get into GFXOFF state,
// pipe reservation
    pub pipe_reserve_mutex: mutex,
    pub AMDGPU_MAX_COMPUTE_QUEUES): DECLARE_BITMAP (pipe_reserve_bitmap,,
// ras
    pub ras_if: *mut ras_common_if,
    pub ras: *mut amdgpu_gfx_ras,
    pub is_poweron: bool,
    pub sw_gfx_ring: [amdgpu_ring; AMDGPU_MAX_SW_GFX_RINGS],
    pub muxer: amdgpu_ring_mux,
    pub /: *mut *mut bool cp_gfx_shadow; / for gfx11,
    pub xcc_mask: u16,
    pub num_xcc_per_xcp: u32,
    pub partition_mutex: mutex,
    pub /: *mut *mut bool mcbp; / mid command buffer preemption,
// IP reg dump
    pub ip_dump_core: *mut u32,
    pub ip_dump_compute_queues: *mut u32,
    pub ip_dump_gfx_queues: *mut u32,
    pub reset_sem_mutex: mutex,
// cleaner shader
    pub cleaner_shader_obj: *mut amdgpu_bo,
    pub cleaner_shader_size: c_uint,
    pub cleaner_shader_gpu_addr: u64,
    pub cleaner_shader_cpu_ptr: *mut c_void,
    pub cleaner_shader_ptr: *const c_void,
    pub enable_cleaner_shader: bool,
    pub enforce_isolation: [amdgpu_isolation_work; MAX_XCP],
// Mutex for synchronizing KFD scheduler operations
    pub userq_sch_mutex: mutex,
    pub userq_sch_req_count: [u64; MAX_XCP],
    pub userq_sch_inactive: [bool; MAX_XCP],
// atomic bitmap of faulted gfx UQ slots (index = pipe | queue << 2)
    pub userq_priv_fault_slots: c_ulong,
    pub userq_priv_fault_work: work_struct,
    pub enforce_isolation_jiffies: [c_ulong; MAX_XCP],
    pub enforce_isolation_time: [c_ulong; MAX_XCP],
    pub total_submission_cnt: core::sync::atomic::AtomicI32,
    pub idle_work: delayed_work,
    pub workload_profile_active: bool,
    pub workload_profile_mutex: mutex,
    pub disable_kq: bool,
    pub disable_uq: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_gfx_deferred_entry {
    pub ring: *mut amdgpu_ring,
    pub fence: *mut amdgpu_fence,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_gfx_ras_reg_entry {
    pub reg_entry: amdgpu_ras_err_status_reg_entry,
    pub mem_id_type: amdgpu_gfx_ras_mem_id_type,
    pub se_num: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_gfx_ras_mem_id_entry {
    pub mem_id_ent: *const amdgpu_ras_memory_id_entry,
    pub size: u32,
}

//
// amdgpu_gfx_create_bitmask - create a bitmask
//
// @bit_width: length of the mask
//
// create a variable length bit mask.
// Returns the bitmask.
//
extern "C" {
    pub fn amdgpu_gfx_kiq_init_ring(adev: *mut amdgpu_device, xcc_id: c_int) -> c_int;
}
extern "C" {
    pub fn amdgpu_gfx_kiq_free_ring(ring: *mut amdgpu_ring);
}
extern "C" {
    pub fn amdgpu_gfx_kiq_fini(adev: *mut amdgpu_device, xcc_id: c_int);
}
extern "C" {
    pub fn amdgpu_gfx_mqd_sw_fini(adev: *mut amdgpu_device, xcc_id: c_int);
}
extern "C" {
    pub fn amdgpu_gfx_disable_kcq(adev: *mut amdgpu_device, xcc_id: c_int) -> c_int;
}
extern "C" {
    pub fn amdgpu_gfx_enable_kcq(adev: *mut amdgpu_device, xcc_id: c_int) -> c_int;
}
extern "C" {
    pub fn amdgpu_gfx_disable_kgq(adev: *mut amdgpu_device, xcc_id: c_int) -> c_int;
}
extern "C" {
    pub fn amdgpu_gfx_enable_kgq(adev: *mut amdgpu_device, xcc_id: c_int) -> c_int;
}
extern "C" {
    pub fn amdgpu_gfx_compute_queue_acquire(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_gfx_graphics_queue_acquire(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_gfx_off_ctrl(adev: *mut amdgpu_device, enable: bool);
}
extern "C" {
    pub fn amdgpu_gfx_off_ctrl_immediate(adev: *mut amdgpu_device, enable: bool);
}
extern "C" {
    pub fn amdgpu_get_gfx_off_status(adev: *mut amdgpu_device, value: *mut u32) -> c_int;
}
extern "C" {
    pub fn amdgpu_gfx_ras_late_init(adev: *mut amdgpu_device, ras_block: *mut ras_common_if) -> c_int;
}
extern "C" {
    pub fn amdgpu_gfx_ras_suspend(adev: *mut amdgpu_device, ras_block: *mut ras_common_if);
}
extern "C" {
    pub fn amdgpu_gfx_ras_fini(adev: *mut amdgpu_device, ras_block: *mut ras_common_if);
}
extern "C" {
    pub fn amdgpu_get_gfx_off_entrycount(adev: *mut amdgpu_device, value: *mut u64) -> c_int;
}
extern "C" {
    pub fn amdgpu_get_gfx_off_residency(adev: *mut amdgpu_device, residency: *mut u32) -> c_int;
}
extern "C" {
    pub fn amdgpu_set_gfx_off_residency(adev: *mut amdgpu_device, value: bool) -> c_int;
}
extern "C" {
    pub fn amdgpu_kiq_rreg(adev: *mut amdgpu_device, reg: u32, xcc_id: u32) -> u32;
}
extern "C" {
    pub fn amdgpu_kiq_wreg(adev: *mut amdgpu_device, reg: u32, v: u32, xcc_id: u32);
}
extern "C" {
    pub fn amdgpu_kiq_hdp_flush(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_gfx_get_num_kcq(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_gfx_cp_init_microcode(adev: *mut amdgpu_device, ucode_id: u32);
}
extern "C" {
    pub fn amdgpu_gfx_ras_sw_init(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_gfx_is_master_xcc(adev: *mut amdgpu_device, xcc_id: c_int) -> bool;
}
extern "C" {
    pub fn amdgpu_gfx_sysfs_init(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_gfx_sysfs_fini(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_gfx_cleaner_shader_sw_fini(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_gfx_enforce_isolation_handler(work: *mut work_struct);
}
extern "C" {
    pub fn amdgpu_gfx_enforce_isolation_ring_begin_use(ring: *mut amdgpu_ring);
}
extern "C" {
    pub fn amdgpu_gfx_enforce_isolation_ring_end_use(ring: *mut amdgpu_ring);
}
extern "C" {
    pub fn amdgpu_gfx_profile_idle_work_handler(work: *mut work_struct);
}
extern "C" {
    pub fn amdgpu_gfx_profile_ring_begin_use(ring: *mut amdgpu_ring);
}
extern "C" {
    pub fn amdgpu_gfx_profile_ring_end_use(ring: *mut amdgpu_ring);
}
extern "C" {
    pub fn amdgpu_gfx_csb_preamble_start(buffer: *mut u32) -> u32;
}
extern "C" {
    pub fn amdgpu_gfx_csb_data_parser(adev: *mut amdgpu_device, buffer: *mut u32, count: u32) -> u32;
}
extern "C" {
    pub fn amdgpu_gfx_csb_preamble_end(buffer: *mut u32, count: u32);
}
extern "C" {
    pub fn amdgpu_debugfs_gfx_sched_mask_init(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_debugfs_compute_sched_mask_init(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_gfx_ring_preempt_ib(ring: *mut amdgpu_ring) -> c_int;
}
