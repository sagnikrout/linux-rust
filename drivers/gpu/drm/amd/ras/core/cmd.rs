//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/ras/core/cmd.h
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
// Copyright 2025 Advanced Micro Devices, Inc.
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

pub const RAS_CMD_MAX_IN_SIZE: c_int = 256;
pub const RAS_CMD_MAX_GPU_NUM: c_int = 32;
pub const RAS_CMD_MAX_BAD_PAGES_PER_GROUP: c_int = 32;
// position of instance value in sub_block_index of
// ta_ras_trigger_error_input, the sub block uses lower 12 bits
//
pub const RAS_TA_INST_MASK: c_uint = 0xfffff000;
pub const RAS_TA_INST_SHIFT: c_uint = 0xc;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ras_cmd_interface_type {
    RAS_CMD_INTERFACE_TYPE_NONE,
    RAS_CMD_INTERFACE_TYPE_AMDGPU,
    RAS_CMD_INTERFACE_TYPE_VF,
    RAS_CMD_INTERFACE_TYPE_PF,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ras_cmd_id_range {
    RAS_CMD_ID_COMMON_START = 0,
    RAS_CMD_ID_COMMON_END = 0x10000,
    RAS_CMD_ID_AMDGPU_START = RAS_CMD_ID_COMMON_END,
    RAS_CMD_ID_AMDGPU_END = 0x20000,
    RAS_CMD_ID_MXGPU_START = RAS_CMD_ID_AMDGPU_END,
    RAS_CMD_ID_MXGPU_END = 0x30000,
    RAS_CMD_ID_MXGPU_VF_START = RAS_CMD_ID_MXGPU_END,
    RAS_CMD_ID_MXGPU_VF_END = 0x40000,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ras_cmd_id {
    RAS_CMD__BEGIN = RAS_CMD_ID_COMMON_START,
    RAS_CMD__QUERY_INTERFACE_INFO,
    RAS_CMD__GET_DEVICES_INFO,
    RAS_CMD__GET_BLOCK_ECC_STATUS,
    RAS_CMD__INJECT_ERROR,
    RAS_CMD__GET_BAD_PAGES,
    RAS_CMD__CLEAR_BAD_PAGE_INFO,
    RAS_CMD__RESET_ALL_ERROR_COUNTS,
    RAS_CMD__GET_SAFE_FB_ADDRESS_RANGES,
    RAS_CMD__TRANSLATE_FB_ADDRESS,
    RAS_CMD__GET_LINK_TOPOLOGY,
    RAS_CMD__GET_CPER_SNAPSHOT,
    RAS_CMD__GET_CPER_RECORD,
    RAS_CMD__GET_BATCH_TRACE_SNAPSHOT,
    RAS_CMD__GET_BATCH_TRACE_RECORD,
    RAS_CMD__GET_ALL_BLOCK_ECC_STATUS,
    RAS_CMD__SET_CMD_AUTO_UPDATE,
    RAS_CMD__CHECK_ADDRESS_VALIDITY,
    RAS_CMD__CONVERT_RETIRED_ADDRESS,
    RAS_CMD__SUPPORTED_MAX = RAS_CMD_ID_COMMON_END,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ras_cmd_response {
    RAS_CMD__SUCCESS = 0,
    RAS_CMD__SUCCESS_EXEED_BUFFER,
    RAS_CMD__ERROR_UKNOWN_CMD,
    RAS_CMD__ERROR_INVALID_CMD,
    RAS_CMD__ERROR_VERSION,
    RAS_CMD__ERROR_INVALID_INPUT_SIZE,
    RAS_CMD__ERROR_INVALID_INPUT_DATA,
    RAS_CMD__ERROR_DRV_INIT_FAIL,
    RAS_CMD__ERROR_ACCESS_DENIED,
    RAS_CMD__ERROR_GENERIC,
    RAS_CMD__ERROR_TIMEOUT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ras_error_type {
    RAS_TYPE_ERROR__NONE = 0,
    RAS_TYPE_ERROR__PARITY = 1,
    RAS_TYPE_ERROR__SINGLE_CORRECTABLE = 2,
    RAS_TYPE_ERROR__MULTI_UNCORRECTABLE = 4,
    RAS_TYPE_ERROR__POISON = 8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_cmd_func_map {
    pub cmd_id: u32,
    pub data): *mut *mut ras_cmd_ctx cmd, void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_device_bdf {
    pub 3: uint32_t function :,
    pub 5: uint32_t device :,
    pub 8: uint32_t bus :,
    pub 16: uint32_t domain :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_cmd_param {
    pub idx_vf: u32,
    pub data: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_cmd_ctx {
    pub magic: u32,
    pub 10: uint16_t ras_cmd_minor_ver :,
    pub 6: uint16_t ras_cmd_major_ver :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_cmd_dev_handle {
    pub dev_handle: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_cmd_block_ecc_info_req {
    pub dev: ras_cmd_dev_handle,
    pub block_id: u32,
    pub subblock_id: u32,
    pub reserved: [u32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_cmd_block_ecc_info_rsp {
    pub version: u32,
    pub ce_count: u32,
    pub ue_count: u32,
    pub de_count: u32,
    pub reserved: [u32; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_cmd_inject_error_req {
    pub dev: ras_cmd_dev_handle,
    pub block_id: u32,
    pub subblock_id: u32,
    pub address: u64,
    pub error_type: u32,
    pub instance_mask: u32,
// vf index
    pub 6: uint64_t vf_idx :,
// method of error injection. i.e persistent, coherent etc
    pub 10: uint64_t method :,
    pub 48: uint64_t rsv :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_cmd_inject_error_rsp {
    pub version: u32,
    pub reserved: [u32; 5],
    pub address: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_cmd_dev_info {
    pub dev_handle: u64,
    pub location_id: u32,
    pub ecc_enabled: u32,
    pub ecc_supported: u32,
    pub vf_num: u32,
    pub asic_type: u32,
    pub oam_id: u32,
    pub reserved: [u32; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_cmd_devices_info_rsp {
    pub version: u32,
    pub dev_num: u32,
    pub reserved: [u32; 6],
    pub devs: [ras_cmd_dev_info; RAS_CMD_MAX_GPU_NUM],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_cmd_bad_page_record {
    pub address: u64,
    pub offset: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_cmd_bad_pages_info_req {
    pub device: ras_cmd_dev_handle,
    pub group_index: u32,
    pub reserved: [u32; 5],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_cmd_bad_pages_info_rsp {
    pub version: u32,
    pub group_index: u32,
    pub bp_in_group: u32,
    pub bp_total_cnt: u32,
    pub reserved: [u32; 4],
    pub records: [ras_cmd_bad_page_record; RAS_CMD_MAX_BAD_PAGES_PER_GROUP],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_query_interface_info_req {
    pub reserved: [u32; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_query_interface_info_rsp {
    pub version: u32,
    pub ras_cmd_major_ver: u32,
    pub ras_cmd_minor_ver: u32,
    pub plat_major_ver: u32,
    pub plat_minor_ver: u32,
    pub interface_type: u8,
    pub rsv: [u8; 3],
    pub reserved: [u32; 8],
}

pub const RAS_MAX_NUM_SAFE_RANGES: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_cmd_ras_safe_fb_address_ranges_rsp {
    pub version: u32,
    pub num_ranges: u32,
    pub reserved: [u32; 4],
    pub start: u64,
    pub size: u64,
    pub idx: u32,
    pub reserved: [u32; 3],
    pub range: [}; RAS_MAX_NUM_SAFE_RANGES],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ras_fb_addr_type {
    RAS_FB_ADDR_SOC_PHY, /* SPA */
    RAS_FB_ADDR_BANK,
    RAS_FB_ADDR_VF_PHY, /* GPA */
    RAS_FB_ADDR_UNKNOWN
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_fb_bank_addr {
    pub /: *mut *mut uint32_t stack_id; / SID,
    pub bank_group: u32,
    pub bank: u32,
    pub row: u32,
    pub column: u32,
    pub channel: u32,
    pub /: *mut *mut uint32_t subchannel; / Also called Pseudochannel (PC),
    pub reserved: [u32; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_fb_vf_phy_addr {
    pub vf_idx: u32,
    pub reserved: u32,
    pub addr: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union ras_translate_fb_address {
    pub bank_addr: ras_fb_bank_addr,
    pub soc_phy_addr: u64,
    pub vf_phy_addr: ras_fb_vf_phy_addr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_cmd_translate_fb_address_req {
    pub dev: ras_cmd_dev_handle,
    pub src_addr_type: ras_fb_addr_type,
    pub dest_addr_type: ras_fb_addr_type,
    pub trans_addr: ras_translate_fb_address,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_cmd_translate_fb_address_rsp {
    pub version: u32,
    pub reserved: [u32; 5],
    pub trans_addr: ras_translate_fb_address,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_dev_link_topology_req {
    pub src: ras_cmd_dev_handle,
    pub dst: ras_cmd_dev_handle,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_dev_link_topology_rsp {
    pub version: u32,
    pub /: *mut *mut uint32_t link_status; / HW status of the link,
    pub /: *mut *mut uint32_t link_type; / type of the link,
    pub /: *mut *mut uint32_t num_hops; / number of hops,
    pub reserved: [u32; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_cmd_cper_snapshot_req {
    pub dev: ras_cmd_dev_handle,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_cmd_cper_snapshot_rsp {
    pub version: u32,
    pub reserved: [u32; 4],
    pub total_cper_num: u32,
    pub start_cper_id: u64,
    pub latest_cper_id: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_cmd_cper_record_req {
    pub dev: ras_cmd_dev_handle,
    pub cper_start_id: u64,
    pub cper_num: u32,
    pub buf_size: u32,
    pub buf_ptr: u64,
    pub reserved: [u32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_cmd_cper_record_rsp {
    pub version: u32,
    pub real_data_size: u32,
    pub real_cper_num: u32,
    pub remain_num: u32,
    pub reserved: [u32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_cmd_batch_trace_snapshot_req {
    pub dev: ras_cmd_dev_handle,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_cmd_batch_trace_snapshot_rsp {
    pub version: u32,
    pub reserved: [u32; 4],
    pub total_batch_num: u32,
    pub start_batch_id: u64,
    pub latest_batch_id: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_cmd_batch_trace_record_req {
    pub dev: ras_cmd_dev_handle,
    pub start_batch_id: u64,
    pub batch_num: u32,
    pub reserved: [u32; 5],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct batch_ras_trace_info {
    pub batch_id: u64,
    pub offset: u16,
    pub trace_num: u8,
    pub rsv: u8,
    pub reserved: u32,
}

pub const RAS_CMD_MAX_BATCH_NUM: c_int = 300;
pub const RAS_CMD_MAX_TRACE_NUM: c_int = 300;
// Upper bounds for RAS_CMD__GET_CPER_RECORD to limit kernel allocations and work.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_cmd_batch_trace_record_rsp {
    pub version: u32,
    pub real_batch_num: u16,
    pub remain_num: u16,
    pub start_batch_id: u64,
    pub reserved: [u32; 2],
    pub batchs: [batch_ras_trace_info; RAS_CMD_MAX_BATCH_NUM],
    pub records: [ras_log_info; RAS_CMD_MAX_TRACE_NUM],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_cmd_auto_update_req {
    pub dev: ras_cmd_dev_handle,
    pub mode: u32,
    pub cmd_id: u32,
    pub addr: u64,
    pub len: u32,
    pub reserved: [u32; 5],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_cmd_auto_update_rsp {
    pub version: u32,
    pub reserved: [u32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_cmd_address_check_req {
    pub dev: ras_cmd_dev_handle,
    pub address: u64,
    pub flags: u32,
    pub vf_idx: u32,
    pub reserved: [u32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_cmd_address_check_rsp {
    pub version: u32,
    pub result: u32,
    pub reserved: [u32; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_cmd_convert_retired_address_req {
    pub dev: ras_cmd_dev_handle,
    pub address: u64,
    pub reserved: [u32; 6],
}

pub const RAS_CMD_MAX_RETIRED_ADDR_COUNT: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_cmd_convert_retired_address_rsp {
    pub version: u32,
    pub retired_count: u32,
    pub retired_addr: [u64; RAS_CMD_MAX_RETIRED_ADDR_COUNT],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_cmd_blocks_ecc_req {
    pub dev: ras_cmd_dev_handle,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_cmd_block_ecc {
    pub ce_count: u32,
    pub ue_count: u32,
    pub de_count: u32,
}

pub const MAX_RAS_BLOCK_NUM: c_int = 20;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_cmd_blocks_ecc_rsp {
    pub version: u32,
    pub reserved: [u32; 5],
    pub blocks: [ras_cmd_block_ecc; MAX_RAS_BLOCK_NUM],
}

extern "C" {
    pub fn ras_cmd_init(ras_core: *mut ras_core_context) -> c_int;
}
extern "C" {
    pub fn ras_cmd_fini(ras_core: *mut ras_core_context) -> c_int;
}
extern "C" {
    pub fn rascore_handle_cmd(ras_core: *mut ras_core_context, cmd: *mut ras_cmd_ctx, data: *mut c_void) -> c_int;
}
