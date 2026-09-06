//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/amdgpu/amdgpu_virt.h
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
// Author: Monk.liu@amd.com
//

// flags for indirect register access path supported by rlcg for sriov

// error code for indirect register access path supported by rlcg for sriov
pub const AMDGPU_RLCG_VFGATE_DISABLED: c_uint = 0x4000000;
pub const AMDGPU_RLCG_WRONG_OPERATION_TYPE: c_uint = 0x2000000;
pub const AMDGPU_RLCG_REG_NOT_IN_RANGE: c_uint = 0x1000000;
pub const AMDGPU_RLCG_SCRATCH1_ADDRESS_MASK: c_uint = 0xFFFFF;
pub const AMDGPU_RLCG_SCRATCH1_ERROR_MASK: c_uint = 0xF000000;
pub const AMDGPU_RLCG_VFI_CMD__WR: c_uint = 0x0;
pub const AMDGPU_RLCG_VFI_CMD__RD: c_uint = 0x1;
pub const AMDGPU_RLCG_VFI_STAT__BUSY: c_uint = 0x0;
pub const AMDGPU_RLCG_VFI_STAT__DONE: c_uint = 0x1;
pub const AMDGPU_RLCG_VFI_STAT__INV_CMD: c_uint = 0x2;
pub const AMDGPU_RLCG_VFI_STAT__INV_ADDR: c_uint = 0x3;
pub const AMDGPU_RLCG_VFI_STAT__ERR: c_uint = 0xFF;
// all asic after AI use this offset
pub const mmRCC_IOV_FUNC_IDENTIFIER: c_uint = 0xDE5;
// tonga/fiji use this offset
pub const mmBIF_IOV_FUNC_IDENTIFIER: c_uint = 0x1503;
pub const AMDGPU_VF2PF_UPDATE_MAX_RETRY_LIMIT: c_int = 2;
// Signature used to validate the SR-IOV dynamic critical region init data header ("INDA")

pub const AMDGPU_SRIOV_CRIT_DATA_SIG_LEN: c_int = 4;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amdgpu_sriov_vf_mode {
    SRIOV_VF_MODE_BARE_METAL = 0,
    SRIOV_VF_MODE_ONE_VF,
    SRIOV_VF_MODE_MULTI_VF,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_mm_table {
    pub bo: *mut amdgpu_bo,
    pub cpu_addr: *mut u32,
    pub gpu_addr: u64,
}

pub const AMDGPU_VF_ERROR_ENTRY_SIZE: c_int = 16;
// struct error_entry - amdgpu VF error information.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_vf_error_buffer {
    pub lock: mutex,
    pub read_count: c_int,
    pub write_count: c_int,
    pub code: [u16; AMDGPU_VF_ERROR_ENTRY_SIZE],
    pub flags: [u16; AMDGPU_VF_ERROR_ENTRY_SIZE],
    pub data: [u64; AMDGPU_VF_ERROR_ENTRY_SIZE],
}

//
// struct amdgpu_virt_ops - amdgpu device virt operations
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_virt_ops {
    pub init): *mut *mut *mut int (req_full_gpu)(struct amdgpu_device adev, bool,
    pub init): *mut *mut *mut int (rel_full_gpu)(struct amdgpu_device adev, bool,
    pub adev): *mut *mut int (req_init_data)(struct amdgpu_device,
    pub adev): *mut *mut int (reset_gpu)(struct amdgpu_device,
    pub adev): *mut *mut void (ready_to_reset)(struct amdgpu_device,
    pub adev): *mut *mut int (wait_reset)(struct amdgpu_device,
    pub data3): u32 data1, u32 data2, u32,
    pub block): amdgpu_ras_block,
    pub adev): *mut *mut bool (rcvd_ras_intr)(struct amdgpu_device,
    pub adev): *mut *mut int (req_ras_err_count)(struct amdgpu_device,
    pub vf_rptr): *mut *mut *mut int (req_ras_cper_dump)(struct amdgpu_device adev, u64,
    pub adev): *mut *mut int (req_bad_pages)(struct amdgpu_device,
    pub addr): *mut *mut *mut int (req_ras_chk_criti)(struct amdgpu_device adev, u64,
    pub param3): u32 param1, u32 param2, u32,
    pub fmt2): u32 req_code, u32 ptl_state, u32 fmt1, u32,
}

//
// Firmware Reserve Frame buffer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_virt_fw_reserve {
    pub p_pf2vf: *mut amd_sriov_msg_pf2vf_info_header,
    pub p_vf2pf: *mut amd_sriov_msg_vf2pf_info_header,
    pub ras_telemetry: *mut c_void,
    pub checksum_key: c_uint,
}

//
// Legacy GIM header
//
// Defination between PF and VF
// Structures forcibly aligned to 4 to keep the same style as PF.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum AMDGIM_FEATURE_FLAG {
// GIM supports feature of Error log collecting
    AMDGIM_FEATURE_ERROR_LOG_COLLECT = 0x1,
// GIM supports feature of loading uCodes
    AMDGIM_FEATURE_GIM_LOAD_UCODES   = 0x2,
// VRAM LOST by GIM
    AMDGIM_FEATURE_GIM_FLR_VRAMLOST = 0x4,
// MM bandwidth
    AMDGIM_FEATURE_GIM_MM_BW_MGR = 0x8,
// PP ONE VF MODE in GIM
    AMDGIM_FEATURE_PP_ONE_VF = (1 << 4),
// Indirect Reg Access enabled
    AMDGIM_FEATURE_INDIRECT_REG_ACCESS = (1 << 5),
// AV1 Support MODE
    AMDGIM_FEATURE_AV1_SUPPORT = (1 << 6),
// VCN RB decouple
    AMDGIM_FEATURE_VCN_RB_DECOUPLE = (1 << 7),
// MES info
    AMDGIM_FEATURE_MES_INFO_ENABLE = (1 << 8),
    AMDGIM_FEATURE_RAS_CAPS = (1 << 9),
    AMDGIM_FEATURE_RAS_TELEMETRY = (1 << 10),
    AMDGIM_FEATURE_RAS_CPER = (1 << 11),
    AMDGIM_FEATURE_XGMI_TA_EXT_PEER_LINK = (1 << 12),
    AMDGIM_FEATURE_XGMI_CONNECTED_TO_CPU = (1 << 13),
    AMDGIM_FEATURE_PTL_SUPPORT = (1 << 14),
    AMDGIM_FEATURE_UNITID_SUPPORT = (1 << 15),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum AMDGIM_REG_ACCESS_FLAG {
// Use PSP to program IH_RB_CNTL
    AMDGIM_FEATURE_IH_REG_PSP_EN      = (1 << 0),
// Use RLC to program MMHUB regs
    AMDGIM_FEATURE_MMHUB_REG_RLC_EN   = (1 << 1),
// Use RLC to program GC regs
    AMDGIM_FEATURE_GC_REG_RLC_EN      = (1 << 2),
// Use PSP to program L1_TLB_CNTL
    AMDGIM_FEATURE_L1_TLB_CNTL_PSP_EN = (1 << 3),
// Use RLCG to program SQ_CONFIG1
    AMDGIM_FEATURE_REG_ACCESS_SQ_CONFIG = (1 << 4),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgim_pf2vf_info_v1 {
// header contains size and version
    pub header: amd_sriov_msg_pf2vf_info_header,
// max_width * max_height
    pub uvd_enc_max_pixels_count: c_uint,
// 16x16 pixels/sec, codec independent
    pub uvd_enc_max_bandwidth: c_uint,
// max_width * max_height
    pub vce_enc_max_pixels_count: c_uint,
// 16x16 pixels/sec, codec independent
    pub vce_enc_max_bandwidth: c_uint,
// MEC FW position in kb from the start of visible frame buffer
    pub mecfw_kboffset: c_uint,
// The features flags of the GIM driver supports.
    pub feature_flags: c_uint,
// use private key from mailbox 2 to create chueksum
    pub checksum: c_uint,
    pub __aligned(4): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgim_vf2pf_info_v1 {
// header contains size and version
    pub header: amd_sriov_msg_vf2pf_info_header,
// driver version
    pub driver_version: [c_char; 64],
// driver certification, 1=WHQL, 0=None
    pub driver_cert: c_uint,
// guest OS type and version: need a define
    pub os_info: c_uint,
// in the unit of 1M
    pub fb_usage: c_uint,
// guest gfx engine usage percentage
    pub gfx_usage: c_uint,
// guest gfx engine health percentage
    pub gfx_health: c_uint,
// guest compute engine usage percentage
    pub compute_usage: c_uint,
// guest compute engine health percentage
    pub compute_health: c_uint,
// guest vce engine usage percentage. 0xffff means N/A.
    pub vce_enc_usage: c_uint,
// guest vce engine health percentage. 0xffff means N/A.
    pub vce_enc_health: c_uint,
// guest uvd engine usage percentage. 0xffff means N/A.
    pub uvd_enc_usage: c_uint,
// guest uvd engine usage percentage. 0xffff means N/A.
    pub uvd_enc_health: c_uint,
    pub checksum: c_uint,
    pub __aligned(4): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgim_vf2pf_info_v2 {
// header contains size and version
    pub header: amd_sriov_msg_vf2pf_info_header,
    pub checksum: u32,
// driver version
    pub driver_version: [u8; 64],
// driver certification, 1=WHQL, 0=None
    pub driver_cert: u32,
// guest OS type and version: need a define
    pub os_info: u32,
// in the unit of 1M
    pub fb_usage: u32,
// guest gfx engine usage percentage
    pub gfx_usage: u32,
// guest gfx engine health percentage
    pub gfx_health: u32,
// guest compute engine usage percentage
    pub compute_usage: u32,
// guest compute engine health percentage
    pub compute_health: u32,
// guest vce engine usage percentage. 0xffff means N/A.
    pub vce_enc_usage: u32,
// guest vce engine health percentage. 0xffff means N/A.
    pub vce_enc_health: u32,
// guest uvd engine usage percentage. 0xffff means N/A.
    pub uvd_enc_usage: u32,
// guest uvd engine usage percentage. 0xffff means N/A.
    pub uvd_enc_health: u32,
    pub 0)]: uint32_t reserved[AMDGIM_GET_STRUCTURE_RESERVED_SIZE(256, 64, 0, (12 + sizeof(struct amd_sriov_msg_vf2pf_info_header)/sizeof(uint32_t)),,
    pub __aligned(4): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_virt_ras_err_handler_data {
// point to bad page records array
    pub bps: *mut eeprom_table_record,
// point to reserved bo array
    pub bps_bo: *mut amdgpu_bo,
// number of slots in bps[] / bps_bo[] (always >= count)
    pub capacity: c_int,
// the count of entries
    pub count: c_int,
// last reserved entry's index + 1
    pub last_reserved: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_virt_ras {
    pub ras_error_cnt_rs: ratelimit_state,
    pub ras_cper_dump_rs: ratelimit_state,
    pub ras_chk_criti_rs: ratelimit_state,
    pub ras_telemetry_mutex: mutex,
    pub cper_rptr: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_virt_region {
    pub offset: u32,
    pub size_kb: u32,
}

// GPU virtualization
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_virt {
    pub caps: u32,
    pub csa_obj: *mut amdgpu_bo,
    pub csa_cpu_addr: *mut c_void,
    pub chained_ib_support: bool,
    pub reg_val_offs: u32,
    pub ack_irq: amdgpu_irq_src,
    pub rcv_irq: amdgpu_irq_src,
    pub flr_work: work_struct,
    pub req_bad_pages_work: work_struct,
    pub handle_bad_pages_work: work_struct,
    pub mm_table: amdgpu_mm_table,
    pub ops: *const amdgpu_virt_ops,
    pub vf_errors: amdgpu_vf_error_buffer,
    pub fw_reserve: amdgpu_virt_fw_reserve,
    pub virt_caps: amdgpu_virt_caps,
    pub gim_feature: u32,
    pub reg_access_mode: u32,
    pub req_init_data_ver: c_int,
    pub tdr_debug: bool,
    pub virt_eh_data: *mut amdgpu_virt_ras_err_handler_data,
    pub ras_init_done: bool,
    pub reg_access: u32,
// dynamic(v2) critical regions
    pub init_data_header: amdgpu_virt_region,
    pub crit_regn: amdgpu_virt_region,
    pub crit_regn_tbl: [amdgpu_virt_region; AMD_SRIOV_MSG_MAX_TABLE_ID],
    pub is_dynamic_crit_regn_enabled: bool,
// vf2pf message
    pub vf2pf_work: delayed_work,
    pub vf2pf_update_interval_ms: u32,
    pub vf2pf_update_retry_cnt: c_int,
// multimedia bandwidth config
    pub is_mm_bw_enabled: bool,
    pub decode_max_dimension_pixels: u32,
    pub decode_max_frame_pixels: u32,
    pub encode_max_dimension_pixels: u32,
    pub encode_max_frame_pixels: u32,
// the ucode id to signal the autoload
    pub autoload_ucode_id: u32,
// Spinlock to protect access to the RLCG register interface
    pub rlcg_reg_lock: spinlock_t,
// PTL (Performance Throttle Limiter) response from host
    pub ptl_state: u32,
    pub ptl_pref_format1: u32,
    pub ptl_pref_format2: u32,
    pub access_req_mutex: mutex,
    pub ras_en_caps: amd_sriov_ras_caps,
    pub ras_telemetry_en_caps: amd_sriov_ras_caps,
    pub ras: amdgpu_virt_ras,
    pub count_cache: amd_sriov_ras_telemetry_error_count,
// hibernate and resume with different VF feature for xgmi enabled system
    pub is_xgmi_node_migrate_enabled: bool,
}

extern "C" {
    pub fn boot_cpu_has(_arg: X86_FEATURE_HYPERVISOR) -> return;
}

extern "C" {
    pub fn amdgpu_virt_mmio_blocked(adev: *mut amdgpu_device) -> bool;
}
extern "C" {
    pub fn amdgpu_virt_init_setting(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_virt_request_full_gpu(adev: *mut amdgpu_device, init: bool) -> c_int;
}
extern "C" {
    pub fn amdgpu_virt_release_full_gpu(adev: *mut amdgpu_device, init: bool) -> c_int;
}
extern "C" {
    pub fn amdgpu_virt_reset_gpu(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_virt_request_init_data(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_virt_ready_to_reset(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_virt_wait_reset(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_virt_alloc_mm_table(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_virt_free_mm_table(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_virt_rcvd_ras_interrupt(adev: *mut amdgpu_device) -> bool;
}
extern "C" {
    pub fn amdgpu_virt_release_ras_err_handler_data(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_virt_init_data_exchange(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_virt_exchange_data(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_virt_fini_data_exchange(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_virt_init(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_virt_init_critical_region(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_virt_can_access_debugfs(adev: *mut amdgpu_device) -> bool;
}
extern "C" {
    pub fn amdgpu_virt_enable_access_debugfs(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_virt_disable_access_debugfs(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_virt_get_sriov_vf_mode(adev: *mut amdgpu_device) -> amdgpu_sriov_vf_mode;
}
extern "C" {
    pub fn amdgpu_virt_pre_reset(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_virt_post_reset(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_sriov_xnack_support(adev: *mut amdgpu_device) -> bool;
}
extern "C" {
    pub fn amdgpu_virt_rlcg_reg_rw(adev: *mut amdgpu_device, offset: u32, v: u32, flag: u32, xcc_id: u32) -> u32;
}
extern "C" {
    pub fn amdgpu_virt_get_ras_capability(adev: *mut amdgpu_device) -> bool;
}
extern "C" {
    pub fn amdgpu_virt_req_ras_cper_dump(adev: *mut amdgpu_device, force_update: bool) -> c_int;
}
extern "C" {
    pub fn amdgpu_virt_ras_telemetry_post_reset(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_virt_request_bad_pages(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_virt_check_vf_critical_region(adev: *mut amdgpu_device, addr: u64, hit: *mut bool) -> c_int;
}
