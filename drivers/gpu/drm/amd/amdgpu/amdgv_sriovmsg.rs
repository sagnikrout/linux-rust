//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/amdgpu/amdgv_sriovmsg.h
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
// Copyright (c) 2018-2021 Advanced Micro Devices, Inc. All rights reserved.
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.
//
pub const AMD_SRIOV_MSG_SIZE_KB: c_int = 1;
//
// layout v1
// 0           64KB        65KB        66KB           68KB                   132KB
// |   VBIOS   |   PF2VF   |   VF2PF   |   Bad Page   | RAS Telemetry Region | ...
// |   64KB    |   1KB     |   1KB     |   2KB        | 64KB                 | ...
//
// layout v2 (offsets are dynamically allocated and the offsets below are examples)
// 0           1KB         64KB        65KB        66KB           68KB                   132KB
// |  INITD_H  |   VBIOS   |   PF2VF   |   VF2PF   |   Bad Page   | RAS Telemetry Region | ...
// |   1KB     |   64KB    |   1KB     |   1KB     |   2KB        | 64KB                 | ...
//
// Note: PF2VF + VF2PF + Bad Page = DataExchange region (allocated contiguously)
//
// v1 layout sizes
pub const AMD_SRIOV_MSG_VBIOS_SIZE_KB_V1: c_int = 64;
pub const AMD_SRIOV_MSG_PF2VF_SIZE_KB_V1: c_int = 1;
pub const AMD_SRIOV_MSG_VF2PF_SIZE_KB_V1: c_int = 1;
pub const AMD_SRIOV_MSG_BAD_PAGE_SIZE_KB_V1: c_int = 2;
pub const AMD_SRIOV_MSG_RAS_TELEMETRY_SIZE_KB_V1: c_int = 64;

// v1 offsets
pub const AMD_SRIOV_MSG_VBIOS_OFFSET_V1: c_int = 0;

pub const AMD_SRIOV_MSG_TMR_OFFSET_KB: c_int = 2048;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amd_sriov_crit_region_version {
    GPU_CRIT_REGION_V1 = 1,
    GPU_CRIT_REGION_V2 = 2,
}

// v2 layout offset enum (in order of allocation)
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amd_sriov_msg_table_id_enum {
    AMD_SRIOV_MSG_IPD_TABLE_ID = 0,
    AMD_SRIOV_MSG_VBIOS_IMG_TABLE_ID,
    AMD_SRIOV_MSG_RAS_TELEMETRY_TABLE_ID,
    AMD_SRIOV_MSG_DATAEXCHANGE_TABLE_ID,
    AMD_SRIOV_MSG_BAD_PAGE_INFO_TABLE_ID,
    AMD_SRIOV_MSG_INITD_H_TABLE_ID,
    AMD_SRIOV_MSG_MAX_TABLE_ID,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amd_sriov_msg_init_data_header {
    pub /: *mut *mut char signature[4]; / "INDA",
    pub version: u32,
    pub checksum: u32,
    pub /: *mut *mut uint32_t initdata_offset; / 0,
    pub /: *mut *mut uint32_t initdata_size_in_kb; / 5MB,
    pub valid_tables: u32,
    pub vbios_img_offset: u32,
    pub vbios_img_size_in_kb: u32,
    pub dataexchange_offset: u32,
    pub dataexchange_size_in_kb: u32,
    pub ras_tele_info_offset: u32,
    pub ras_tele_info_size_in_kb: u32,
    pub ip_discovery_offset: u32,
    pub ip_discovery_size_in_kb: u32,
    pub bad_page_info_offset: u32,
    pub bad_page_size_in_kb: u32,
    pub reserved: [u32; 8],
}

//
// PF2VF history log:
// v1 defined in amdgim
// v2 current
//
// VF2PF history log:
// v1 defined in amdgim
// v2 defined in amdgim
// v3 current
//
pub const AMD_SRIOV_MSG_FW_VRAM_PF2VF_VER: c_int = 2;
pub const AMD_SRIOV_MSG_FW_VRAM_VF2PF_VER: c_int = 3;
pub const AMD_SRIOV_MSG_RESERVE_UCODE: c_int = 24;
pub const AMD_SRIOV_MSG_RESERVE_VCN_INST: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amd_sriov_ucode_engine_id {
    AMD_SRIOV_UCODE_ID_VCE = 0,
    AMD_SRIOV_UCODE_ID_UVD,
    AMD_SRIOV_UCODE_ID_MC,
    AMD_SRIOV_UCODE_ID_ME,
    AMD_SRIOV_UCODE_ID_PFP,
    AMD_SRIOV_UCODE_ID_CE,
    AMD_SRIOV_UCODE_ID_RLC,
    AMD_SRIOV_UCODE_ID_RLC_SRLC,
    AMD_SRIOV_UCODE_ID_RLC_SRLG,
    AMD_SRIOV_UCODE_ID_RLC_SRLS,
    AMD_SRIOV_UCODE_ID_MEC,
    AMD_SRIOV_UCODE_ID_MEC2,
    AMD_SRIOV_UCODE_ID_SOS,
    AMD_SRIOV_UCODE_ID_ASD,
    AMD_SRIOV_UCODE_ID_TA_RAS,
    AMD_SRIOV_UCODE_ID_TA_XGMI,
    AMD_SRIOV_UCODE_ID_SMC,
    AMD_SRIOV_UCODE_ID_SDMA,
    AMD_SRIOV_UCODE_ID_SDMA2,
    AMD_SRIOV_UCODE_ID_VCN,
    AMD_SRIOV_UCODE_ID_DMCU,
    AMD_SRIOV_UCODE_ID__MAX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union amd_sriov_msg_feature_flags {
    pub 1: uint32_t error_log_collect :,
    pub 1: uint32_t host_load_ucodes :,
    pub 1: uint32_t host_flr_vramlost :,
    pub 1: uint32_t mm_bw_management :,
    pub 1: uint32_t pp_one_vf_mode :,
    pub 1: uint32_t reg_indirect_acc :,
    pub 1: uint32_t av1_support :,
    pub 1: uint32_t vcn_rb_decouple :,
    pub 1: uint32_t mes_info_dump_enable :,
    pub 1: uint32_t ras_caps :,
    pub 1: uint32_t ras_telemetry :,
    pub 1: uint32_t ras_cper :,
    pub 1: uint32_t xgmi_ta_ext_peer_link :,
    pub 1: uint32_t xgmi_connected_to_cpu :,
    pub 1: uint32_t ptl_support :,
    pub 1: uint32_t unitid_support :,
    pub 16: uint32_t reserved :,
    pub flags: },
    pub all: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union amd_sriov_reg_access_flags {
    pub 1: uint32_t vf_reg_access_ih :,
    pub 1: uint32_t vf_reg_access_mmhub :,
    pub 1: uint32_t vf_reg_access_gc :,
    pub 1: uint32_t vf_reg_access_l1_tlb_cntl :,
    pub 1: uint32_t vf_reg_access_sq_config :,
    pub 27: uint32_t reserved :,
    pub flags: },
    pub all: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union amd_sriov_ras_caps {
    pub 1: uint64_t block_umc :,
    pub 1: uint64_t block_sdma :,
    pub 1: uint64_t block_gfx :,
    pub 1: uint64_t block_mmhub :,
    pub 1: uint64_t block_athub :,
    pub 1: uint64_t block_pcie_bif :,
    pub 1: uint64_t block_hdp :,
    pub 1: uint64_t block_xgmi_wafl :,
    pub 1: uint64_t block_df :,
    pub 1: uint64_t block_smn :,
    pub 1: uint64_t block_sem :,
    pub 1: uint64_t block_mp0 :,
    pub 1: uint64_t block_mp1 :,
    pub 1: uint64_t block_fuse :,
    pub 1: uint64_t block_mca :,
    pub 1: uint64_t block_vcn :,
    pub 1: uint64_t block_jpeg :,
    pub 1: uint64_t block_ih :,
    pub 1: uint64_t block_mpio :,
    pub 1: uint64_t block_mmsch :,
    pub 1: uint64_t poison_propogation_mode :,
    pub 1: uint64_t uniras_supported :,
    pub 42: uint64_t reserved :,
    pub bits: },
    pub all: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union amd_sriov_msg_os_info {
    pub 1: uint32_t windows :,
    pub 31: uint32_t reserved :,
    pub info: },
    pub all: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amd_sriov_msg_uuid_info {
    pub 16: uint32_t did :,
    pub 8: uint32_t fcn :,
    pub 8: uint32_t asic_7 :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amd_sriov_msg_pf2vf_info_header {
// the total structure size in byte
    pub size: u32,
// version of this structure, written by the HOST
    pub version: u32,
// reserved
    pub reserved: [u32; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amd_sriov_msg_pf2vf_info {
// header contains size and version
    pub header: amd_sriov_msg_pf2vf_info_header,
// use private key from mailbox 2 to create checksum
    pub checksum: u32,
// The features flags of the HOST driver supports
    pub feature_flags: amd_sriov_msg_feature_flags,
// (max_width * max_height * fps) / (16 * 16)
    pub hevc_enc_max_mb_per_second: u32,
// (max_width * max_height) / (16 * 16)
    pub hevc_enc_max_mb_per_frame: u32,
// (max_width * max_height * fps) / (16 * 16)
    pub avc_enc_max_mb_per_second: u32,
// (max_width * max_height) / (16 * 16)
    pub avc_enc_max_mb_per_frame: u32,
// MEC FW position in BYTE from the start of VF visible frame buffer
    pub mecfw_offset: u64,
// MEC FW size in BYTE
    pub mecfw_size: u32,
// UVD FW position in BYTE from the start of VF visible frame buffer
    pub uvdfw_offset: u64,
// UVD FW size in BYTE
    pub uvdfw_size: u32,
// VCE FW position in BYTE from the start of VF visible frame buffer
    pub vcefw_offset: u64,
// VCE FW size in BYTE
    pub vcefw_size: u32,
// Bad pages block position in BYTE
    pub bp_block_offset_low: u32,
    pub bp_block_offset_high: u32,
// Bad pages block size in BYTE
    pub bp_block_size: u32,
// frequency for VF to update the VF2PF area in msec, 0 = manual
    pub vf2pf_update_interval_ms: u32,
// identification in ROCm SMI
    pub uuid: u64,
    pub pad: u32,
// flags to indicate which register access method VF should use
    pub reg_access_flags: amd_sriov_reg_access_flags,
// MM BW management
    pub decode_max_dimension_pixels: u32,
    pub decode_max_frame_pixels: u32,
    pub encode_max_dimension_pixels: u32,
    pub encode_max_frame_pixels: u32,
    pub mm_bw_management: [}; AMD_SRIOV_MSG_RESERVE_VCN_INST],
// UUID info
    pub uuid_info: amd_sriov_msg_uuid_info,
// PCIE atomic ops support flag
    pub pcie_atomic_ops_support_flags: u32,
// Portion of GPU memory occupied by VF.  MAX value is 65535, but set to uint32_t to maintain alignment with reserved size
    pub gpu_capacity: u32,
// vf bdf on host pci tree for debug only
    pub bdf_on_host: u32,
    pub use.: uint32_t more_bp; //Reserved for future,
    pub ras_en_caps: amd_sriov_ras_caps,
    pub ras_telemetry_en_caps: amd_sriov_ras_caps,
// PTL status response for guest
    pub 1=enabled: uint32_t ptl_enabled; // PTL enable status: 0=disabled,,
    pub 1: uint32_t ptl_pref_format1; // Current preferred format,
    pub 2: uint32_t ptl_pref_format2; // Current preferred format,
// unit ID assigned by host; vf_idx [0..254] maps to unitid [1..255] (0 = pf)
    pub unitid: u8,
    pub align: uint8_t padding[3]; //use the 3 bytes to,
// reserved
    pub AMD_SRIOV_MSG_PF2VF_INFO_FILLED_SIZE]: uint32_t reserved[256 -,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amd_sriov_msg_vf2pf_info_header {
// the total structure size in byte
    pub size: u32,
// version of this structure, written by the guest
    pub version: u32,
// reserved
    pub reserved: [u32; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amd_sriov_msg_vf2pf_info {
// header contains size and version
    pub header: amd_sriov_msg_vf2pf_info_header,
    pub checksum: u32,
// driver version
    pub driver_version: [u8; 64],
// driver certification, 1=WHQL, 0=None
    pub driver_cert: u32,
// guest OS type and version
    pub os_info: amd_sriov_msg_os_info,
// guest fb information in the unit of MB
    pub fb_usage: u32,
// guest gfx engine usage percentage
    pub gfx_usage: u32,
// guest gfx engine health percentage
    pub gfx_health: u32,
// guest compute engine usage percentage
    pub compute_usage: u32,
// guest compute engine health percentage
    pub compute_health: u32,
// guest avc engine usage percentage. 0xffff means N/A
    pub avc_enc_usage: u32,
// guest avc engine health percentage. 0xffff means N/A
    pub avc_enc_health: u32,
// guest hevc engine usage percentage. 0xffff means N/A
    pub hevc_enc_usage: u32,
// guest hevc engine usage percentage. 0xffff means N/A
    pub hevc_enc_health: u32,
// combined encode/decode usage
    pub encode_usage: u32,
    pub decode_usage: u32,
// Version of PF2VF that VF understands
    pub pf2vf_version_required: u32,
// additional FB usage
    pub fb_vis_usage: u32,
    pub fb_vis_size: u32,
    pub fb_size: u32,
// guest ucode data, each one is 1.25 Dword
    pub id: u8,
    pub version: u32,
    pub ucode_info: [} __packed; AMD_SRIOV_MSG_RESERVE_UCODE],
    pub dummy_page_addr: u64,
// FB allocated for guest MES to record UQ info
    pub mes_info_addr: u64,
    pub mes_info_size: u32,
// reserved
    pub AMD_SRIOV_MSG_VF2PF_INFO_FILLED_SIZE]: uint32_t reserved[256 -,
    pub __packed: },
// mailbox message send from guest to host
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amd_sriov_mailbox_request_message {
    MB_REQ_MSG_REQ_GPU_INIT_ACCESS = 1,
    MB_REQ_MSG_REL_GPU_INIT_ACCESS,
    MB_REQ_MSG_REQ_GPU_FINI_ACCESS,
    MB_REQ_MSG_REL_GPU_FINI_ACCESS,
    MB_REQ_MSG_REQ_GPU_RESET_ACCESS,
    MB_REQ_MSG_REQ_GPU_INIT_DATA,
    MB_REQ_MSG_PSP_VF_CMD_RELAY,

    MB_REQ_MSG_LOG_VF_ERROR = 200,
    MB_REQ_MSG_READY_TO_RESET = 201,
    MB_REQ_MSG_RAS_POISON = 202,
    MB_REQ_RAS_ERROR_COUNT = 203,
    MB_REQ_RAS_CPER_DUMP = 204,
    MB_REQ_RAS_BAD_PAGES = 205,
    MB_REQ_RAS_CHK_CRITI = 206,
    MB_REQ_RAS_REMOTE_CMD = 207,
    MB_REQ_MSG_PTL_UPDATE = 208,
}

// mailbox message send from host to guest
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amd_sriov_mailbox_response_message {
    MB_RES_MSG_CLR_MSG_BUF			= 0,
    MB_RES_MSG_READY_TO_ACCESS_GPU		= 1,
    MB_RES_MSG_FLR_NOTIFICATION		= 2,
    MB_RES_MSG_FLR_NOTIFICATION_COMPLETION  = 3,
    MB_RES_MSG_SUCCESS			= 4,
    MB_RES_MSG_FAIL				= 5,
    MB_RES_MSG_QUERY_ALIVE			= 6,
    MB_RES_MSG_GPU_INIT_DATA_READY		= 7,
    MB_RES_MSG_RAS_POISON_READY		= 8,
    MB_RES_MSG_PF_SOFT_FLR_NOTIFICATION	= 9,
    MB_RES_MSG_GPU_RMA			= 10,
    MB_RES_MSG_RAS_ERROR_COUNT_READY	= 11,
    MB_REQ_RAS_CPER_DUMP_READY		= 14,
    MB_RES_MSG_RAS_BAD_PAGES_READY		= 15,
    MB_RES_MSG_RAS_BAD_PAGES_NOTIFICATION	= 16,
    MB_RES_MSG_UNRECOV_ERR_NOTIFICATION	= 17,
    MB_RES_RAS_CHK_CRITI_READY		= 18,
    MB_RES_RAS_REMOTE_CMD_READY		= 19,
    MB_RES_MSG_PTL_UPDATE_READY		= 20,
    MB_RES_MSG_TEXT_MESSAGE			= 255
}

//
// Generic response status codes for mailbox data fields.
// Used in msg_data[1..N] to indicate operation result.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amd_sriov_response_status {
    AMD_SRIOV_RESP_SUCCESS		= 0,
    AMD_SRIOV_RESP_FAIL		= 1,
    AMD_SRIOV_RESP_UNSUPPORTED	= 2,
}

//
// PTL mailbox data format:
// Request:  msg_data[1]=req_code, msg_data[2]=ptl_state, msg_data[3]=(fmt1<<16)|fmt2
// Response: msg_data[1]=(status<<16)|ptl_state, msg_data[2]=(fmt1<<16)|fmt2
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amd_sriov_ras_telemetry_gpu_block {
    RAS_TELEMETRY_GPU_BLOCK_UMC		= 0,
    RAS_TELEMETRY_GPU_BLOCK_SDMA		= 1,
    RAS_TELEMETRY_GPU_BLOCK_GFX		= 2,
    RAS_TELEMETRY_GPU_BLOCK_MMHUB		= 3,
    RAS_TELEMETRY_GPU_BLOCK_ATHUB		= 4,
    RAS_TELEMETRY_GPU_BLOCK_PCIE_BIF	= 5,
    RAS_TELEMETRY_GPU_BLOCK_HDP		= 6,
    RAS_TELEMETRY_GPU_BLOCK_XGMI_WAFL	= 7,
    RAS_TELEMETRY_GPU_BLOCK_DF		= 8,
    RAS_TELEMETRY_GPU_BLOCK_SMN		= 9,
    RAS_TELEMETRY_GPU_BLOCK_SEM		= 10,
    RAS_TELEMETRY_GPU_BLOCK_MP0		= 11,
    RAS_TELEMETRY_GPU_BLOCK_MP1		= 12,
    RAS_TELEMETRY_GPU_BLOCK_FUSE		= 13,
    RAS_TELEMETRY_GPU_BLOCK_MCA		= 14,
    RAS_TELEMETRY_GPU_BLOCK_VCN		= 15,
    RAS_TELEMETRY_GPU_BLOCK_JPEG		= 16,
    RAS_TELEMETRY_GPU_BLOCK_IH		= 17,
    RAS_TELEMETRY_GPU_BLOCK_MPIO		= 18,
    RAS_TELEMETRY_GPU_BLOCK_COUNT		= 19,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amd_sriov_ras_telemetry_header {
    pub checksum: u32,
    pub used_size: u32,
    pub reserved: [u32; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amd_sriov_ras_telemetry_error_count {
    pub ce_count: u32,
    pub ue_count: u32,
    pub de_count: u32,
    pub ce_overflow_count: u32,
    pub ue_overflow_count: u32,
    pub de_overflow_count: u32,
    pub reserved: [u32; 6],
    pub block: [}; RAS_TELEMETRY_GPU_BLOCK_COUNT],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amd_sriov_ras_cper_dump {
    pub more: u32,
    pub overflow_count: u64,
    pub count: u64,
    pub wptr: u64,
    pub buf: [u32; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amd_sriov_ras_chk_criti {
    pub hit: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union amd_sriov_ras_host_push {
    pub error_count: amd_sriov_ras_telemetry_error_count,
    pub cper_dump: amd_sriov_ras_cper_dump,
    pub chk_criti: amd_sriov_ras_chk_criti,
}

pub const AMD_SRIOV_UNIRAS_BLOCKS_BUF_SIZE: c_int = 4096;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amd_sriov_uniras_shared_mem {
    pub blocks_ecc_buf: [u8; AMD_SRIOV_UNIRAS_BLOCKS_BUF_SIZE],
    pub cmd_buf: [u8; AMD_SRIOV_UNIRAS_CMD_MAX_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdsriov_ras_telemetry {
    pub header: amd_sriov_ras_telemetry_header,
    pub body: amd_sriov_ras_host_push,
    pub uniras_shared_mem: amd_sriov_uniras_shared_mem,
}

// version data stored in MAILBOX_MSGBUF_RCV_DW1 for future expansion
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amd_sriov_gpu_init_data_version {
    GPU_INIT_DATA_READY_V1 = 1,
}

// checksum function between host and guest
// assertion at compile time

