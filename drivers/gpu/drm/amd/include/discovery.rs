//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/include/discovery.h
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
pub const PSP_HEADER_SIZE: c_int = 256;
pub const BINARY_SIGNATURE: c_uint = 0x28211407;
pub const DISCOVERY_TABLE_SIGNATURE: c_uint = 0x53445049;
pub const GC_TABLE_ID: c_uint = 0x4347;
pub const HARVEST_TABLE_SIGNATURE: c_uint = 0x56524148;
pub const VCN_INFO_TABLE_ID: c_uint = 0x004E4356;
pub const MALL_INFO_TABLE_ID: c_uint = 0x4C4C414D;
pub const NPS_INFO_TABLE_ID: c_uint = 0x0053504E;

// psp structure should go at the top of this structure

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpu_info_header {
    pub /: *mut *mut uint32_t table_id; / table ID,
    pub /: *mut *mut uint16_t version_major; / table version,
    pub /: *mut *mut uint16_t version_minor; / table version,
    pub /: *mut *mut uint32_t size; / size of the entire header+data in bytes,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gc_info_v1_0 {
    pub header: gpu_info_header,
    pub gc_num_se: u32,
    pub gc_num_wgp0_per_sa: u32,
    pub gc_num_wgp1_per_sa: u32,
    pub gc_num_rb_per_se: u32,
    pub gc_num_gl2c: u32,
    pub gc_num_gprs: u32,
    pub gc_num_max_gs_thds: u32,
    pub gc_gs_table_depth: u32,
    pub gc_gsprim_buff_depth: u32,
    pub gc_parameter_cache_depth: u32,
    pub gc_double_offchip_lds_buffer: u32,
    pub gc_wave_size: u32,
    pub gc_max_waves_per_simd: u32,
    pub gc_max_scratch_slots_per_cu: u32,
    pub gc_lds_size: u32,
    pub gc_num_sc_per_se: u32,
    pub gc_num_sa_per_se: u32,
    pub gc_num_packer_per_sc: u32,
    pub gc_num_gl2a: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gc_info_v1_1 {
    pub header: gpu_info_header,
    pub gc_num_se: u32,
    pub gc_num_wgp0_per_sa: u32,
    pub gc_num_wgp1_per_sa: u32,
    pub gc_num_rb_per_se: u32,
    pub gc_num_gl2c: u32,
    pub gc_num_gprs: u32,
    pub gc_num_max_gs_thds: u32,
    pub gc_gs_table_depth: u32,
    pub gc_gsprim_buff_depth: u32,
    pub gc_parameter_cache_depth: u32,
    pub gc_double_offchip_lds_buffer: u32,
    pub gc_wave_size: u32,
    pub gc_max_waves_per_simd: u32,
    pub gc_max_scratch_slots_per_cu: u32,
    pub gc_lds_size: u32,
    pub gc_num_sc_per_se: u32,
    pub gc_num_sa_per_se: u32,
    pub gc_num_packer_per_sc: u32,
    pub gc_num_gl2a: u32,
    pub gc_num_tcp_per_sa: u32,
    pub gc_num_sdp_interface: u32,
    pub gc_num_tcps: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gc_info_v1_2 {
    pub header: gpu_info_header,
    pub gc_num_se: u32,
    pub gc_num_wgp0_per_sa: u32,
    pub gc_num_wgp1_per_sa: u32,
    pub gc_num_rb_per_se: u32,
    pub gc_num_gl2c: u32,
    pub gc_num_gprs: u32,
    pub gc_num_max_gs_thds: u32,
    pub gc_gs_table_depth: u32,
    pub gc_gsprim_buff_depth: u32,
    pub gc_parameter_cache_depth: u32,
    pub gc_double_offchip_lds_buffer: u32,
    pub gc_wave_size: u32,
    pub gc_max_waves_per_simd: u32,
    pub gc_max_scratch_slots_per_cu: u32,
    pub gc_lds_size: u32,
    pub gc_num_sc_per_se: u32,
    pub gc_num_sa_per_se: u32,
    pub gc_num_packer_per_sc: u32,
    pub gc_num_gl2a: u32,
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
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gc_info_v1_3 {
    pub header: gpu_info_header,
    pub gc_num_se: u32,
    pub gc_num_wgp0_per_sa: u32,
    pub gc_num_wgp1_per_sa: u32,
    pub gc_num_rb_per_se: u32,
    pub gc_num_gl2c: u32,
    pub gc_num_gprs: u32,
    pub gc_num_max_gs_thds: u32,
    pub gc_gs_table_depth: u32,
    pub gc_gsprim_buff_depth: u32,
    pub gc_parameter_cache_depth: u32,
    pub gc_double_offchip_lds_buffer: u32,
    pub gc_wave_size: u32,
    pub gc_max_waves_per_simd: u32,
    pub gc_max_scratch_slots_per_cu: u32,
    pub gc_lds_size: u32,
    pub gc_num_sc_per_se: u32,
    pub gc_num_sa_per_se: u32,
    pub gc_num_packer_per_sc: u32,
    pub gc_num_gl2a: u32,
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
    pub gc_tcp_cache_line_size: u32,
    pub gc_instruction_cache_size_per_sqc: u32,
    pub gc_instruction_cache_line_size: u32,
    pub gc_scalar_data_cache_size_per_sqc: u32,
    pub gc_scalar_data_cache_line_size: u32,
    pub gc_tcc_size: u32,
    pub gc_tcc_cache_line_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gc_info_v2_0 {
    pub header: gpu_info_header,
    pub gc_num_se: u32,
    pub gc_num_cu_per_sh: u32,
    pub gc_num_sh_per_se: u32,
    pub gc_num_rb_per_se: u32,
    pub gc_num_tccs: u32,
    pub gc_num_gprs: u32,
    pub gc_num_max_gs_thds: u32,
    pub gc_gs_table_depth: u32,
    pub gc_gsprim_buff_depth: u32,
    pub gc_parameter_cache_depth: u32,
    pub gc_double_offchip_lds_buffer: u32,
    pub gc_wave_size: u32,
    pub gc_max_waves_per_simd: u32,
    pub gc_max_scratch_slots_per_cu: u32,
    pub gc_lds_size: u32,
    pub gc_num_sc_per_se: u32,
    pub gc_num_packer_per_sc: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gc_info_v2_1 {
    pub header: gpu_info_header,
    pub gc_num_se: u32,
    pub gc_num_cu_per_sh: u32,
    pub gc_num_sh_per_se: u32,
    pub gc_num_rb_per_se: u32,
    pub gc_num_tccs: u32,
    pub gc_num_gprs: u32,
    pub gc_num_max_gs_thds: u32,
    pub gc_gs_table_depth: u32,
    pub gc_gsprim_buff_depth: u32,
    pub gc_parameter_cache_depth: u32,
    pub gc_double_offchip_lds_buffer: u32,
    pub gc_wave_size: u32,
    pub gc_max_waves_per_simd: u32,
    pub gc_max_scratch_slots_per_cu: u32,
    pub gc_lds_size: u32,
    pub gc_num_sc_per_se: u32,
    pub gc_num_packer_per_sc: u32,
// new for v2_1
    pub gc_num_tcp_per_sh: u32,
    pub gc_tcp_size_per_cu: u32,
    pub gc_num_sdp_interface: u32,
    pub gc_num_cu_per_sqc: u32,
    pub gc_instruction_cache_size_per_sqc: u32,
    pub gc_scalar_data_cache_size_per_sqc: u32,
    pub gc_tcc_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mall_info_header {
    pub /: *mut *mut uint32_t table_id; / table ID,
    pub /: *mut *mut uint16_t version_major; / table version,
    pub /: *mut *mut uint16_t version_minor; / table version,
    pub /: *mut *mut uint32_t size_bytes; / size of the entire header+data in bytes,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mall_info_v1_0 {
    pub header: mall_info_header,
    pub mall_size_per_m: u32,
    pub m_s_present: u32,
    pub m_half_use: u32,
    pub m_mall_config: u32,
    pub reserved: [u32; 5],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mall_info_v2_0 {
    pub header: mall_info_header,
    pub mall_size_per_umc: u32,
    pub reserved: [u32; 8],
}

pub const VCN_INFO_TABLE_MAX_NUM_INSTANCES: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vcn_info_header {
    pub /: *mut *mut uint32_t table_id; / table ID,
    pub /: *mut *mut uint16_t version_major; / table version,
    pub /: *mut *mut uint16_t version_minor; / table version,
    pub /: *mut *mut uint32_t size_bytes; / size of the entire header+data in bytes,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union _fuse_data {
    pub 1: uint32_t av1_disabled :,
    pub 1: uint32_t vp9_disabled :,
    pub 1: uint32_t hevc_disabled :,
    pub 1: uint32_t h264_disabled :,
    pub 28: uint32_t reserved :,
    pub bits: },
    pub all_bits: u32,
    pub fuse_data: },
    pub reserved: [u32; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vcn_info_v1_0 {
    pub header: vcn_info_header,
    pub below*/: *mut *mut uint32_t num_of_instances; / number of entries used in instance_info,
    pub instance_info: [vcn_instance_info_v1_0; VCN_INFO_TABLE_MAX_NUM_INSTANCES],
    pub reserved: [u32; 4],
}

pub const NPS_INFO_TABLE_MAX_NUM_INSTANCES: c_int = 12;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nps_info_header {
    pub /: *mut *mut uint32_t table_id; / table ID,
    pub /: *mut *mut uint16_t version_major; / table version,
    pub /: *mut *mut uint16_t version_minor; / table version,
    pub /: *mut *mut uint32_t size_bytes; / size of the entire header+data in bytes = 0x000000D4 (212),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nps_instance_info_v1_0 {
    pub base_address: u64,
    pub limit_address: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nps_info_v1_0 {
    pub header: nps_info_header,
    pub nps_type: u32,
    pub count: u32,
}

