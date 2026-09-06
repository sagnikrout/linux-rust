//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/amdgpu/amdgpu_ucode.h
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
// Copyright 2012 Advanced Micro Devices, Inc.
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

pub const RS64_FW_UC_START_ADDR_LO: c_uint = 0x3000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct common_firmware_header {
    pub /: *mut *mut uint32_t size_bytes; / size of the entire header+image(s) in bytes,
    pub /: *mut *mut uint32_t header_size_bytes; / size of just the header in bytes,
    pub /: *mut *mut uint16_t header_version_major; / header version,
    pub /: *mut *mut uint16_t header_version_minor; / header version,
    pub /: *mut *mut uint16_t ip_version_major; / IP version,
    pub /: *mut *mut uint16_t ip_version_minor; / IP version,
    pub ucode_version: u32,
    pub /: *mut *mut uint32_t ucode_size_bytes; / size of ucode in bytes,
    pub /: *mut *mut uint32_t ucode_array_offset_bytes; / payload offset from the start of the header,
    pub /: *mut *mut uint32_t crc32; / crc32 checksum of the payload,
}

// version_major=1, version_minor=0
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mc_firmware_header_v1_0 {
    pub header: common_firmware_header,
    pub /: *mut *mut uint32_t io_debug_size_bytes; / size of debug array in dwords,
    pub /: *mut *mut uint32_t io_debug_array_offset_bytes; / payload offset from the start of the header,
}

// version_major=1, version_minor=0
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smc_firmware_header_v1_0 {
    pub header: common_firmware_header,
    pub ucode_start_addr: u32,
}

// version_major=2, version_minor=0
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smc_firmware_header_v2_0 {
    pub v1_0: smc_firmware_header_v1_0,
    pub /: *mut *mut uint32_t ppt_offset_bytes; / soft pptable offset,
    pub /: *mut *mut uint32_t ppt_size_bytes; / soft pptable size,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smc_soft_pptable_entry {
    pub id: u32,
    pub ppt_offset_bytes: u32,
    pub ppt_size_bytes: u32,
}

// version_major=2, version_minor=1
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smc_firmware_header_v2_1 {
    pub v1_0: smc_firmware_header_v1_0,
    pub pptable_count: u32,
    pub pptable_entry_offset: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct psp_fw_legacy_bin_desc {
    pub fw_version: u32,
    pub offset_bytes: u32,
    pub size_bytes: u32,
}

// version_major=1, version_minor=0
#[repr(C)]
#[derive(Copy, Clone)]
pub struct psp_firmware_header_v1_0 {
    pub header: common_firmware_header,
    pub sos: psp_fw_legacy_bin_desc,
}

// version_major=1, version_minor=1
#[repr(C)]
#[derive(Copy, Clone)]
pub struct psp_firmware_header_v1_1 {
    pub v1_0: psp_firmware_header_v1_0,
    pub toc: psp_fw_legacy_bin_desc,
    pub kdb: psp_fw_legacy_bin_desc,
}

// version_major=1, version_minor=2
#[repr(C)]
#[derive(Copy, Clone)]
pub struct psp_firmware_header_v1_2 {
    pub v1_0: psp_firmware_header_v1_0,
    pub res: psp_fw_legacy_bin_desc,
    pub kdb: psp_fw_legacy_bin_desc,
}

// version_major=1, version_minor=3
#[repr(C)]
#[derive(Copy, Clone)]
pub struct psp_firmware_header_v1_3 {
    pub v1_1: psp_firmware_header_v1_1,
    pub spl: psp_fw_legacy_bin_desc,
    pub rl: psp_fw_legacy_bin_desc,
    pub sys_drv_aux: psp_fw_legacy_bin_desc,
    pub sos_aux: psp_fw_legacy_bin_desc,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct psp_fw_bin_desc {
    pub fw_type: u32,
    pub fw_version: u32,
    pub offset_bytes: u32,
    pub size_bytes: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum psp_fw_type {
    PSP_FW_TYPE_UNKOWN,
    PSP_FW_TYPE_PSP_SOS,
    PSP_FW_TYPE_PSP_SYS_DRV,
    PSP_FW_TYPE_PSP_KDB,
    PSP_FW_TYPE_PSP_TOC,
    PSP_FW_TYPE_PSP_SPL,
    PSP_FW_TYPE_PSP_RL,
    PSP_FW_TYPE_PSP_SOC_DRV,
    PSP_FW_TYPE_PSP_INTF_DRV,
    PSP_FW_TYPE_PSP_DBG_DRV,
    PSP_FW_TYPE_PSP_RAS_DRV,
    PSP_FW_TYPE_PSP_IPKEYMGR_DRV,
    PSP_FW_TYPE_PSP_SPDM_DRV,
    PSP_FW_TYPE_MAX_INDEX,
}

// version_major=2, version_minor=0
#[repr(C)]
#[derive(Copy, Clone)]
pub struct psp_firmware_header_v2_0 {
    pub header: common_firmware_header,
    pub psp_fw_bin_count: u32,
    pub psp_fw_bin: [psp_fw_bin_desc; ],
}

// version_major=2, version_minor=1
#[repr(C)]
#[derive(Copy, Clone)]
pub struct psp_firmware_header_v2_1 {
    pub header: common_firmware_header,
    pub psp_fw_bin_count: u32,
    pub psp_aux_fw_bin_index: u32,
    pub psp_fw_bin: [psp_fw_bin_desc; ],
}

// version_major=1, version_minor=0
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ta_firmware_header_v1_0 {
    pub header: common_firmware_header,
    pub xgmi: psp_fw_legacy_bin_desc,
    pub ras: psp_fw_legacy_bin_desc,
    pub hdcp: psp_fw_legacy_bin_desc,
    pub dtm: psp_fw_legacy_bin_desc,
    pub securedisplay: psp_fw_legacy_bin_desc,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ta_fw_type {
    TA_FW_TYPE_UNKOWN,
    TA_FW_TYPE_PSP_ASD,
    TA_FW_TYPE_PSP_XGMI,
    TA_FW_TYPE_PSP_RAS,
    TA_FW_TYPE_PSP_HDCP,
    TA_FW_TYPE_PSP_DTM,
    TA_FW_TYPE_PSP_RAP,
    TA_FW_TYPE_PSP_SECUREDISPLAY,
    TA_FW_TYPE_PSP_XGMI_AUX,
    TA_FW_TYPE_MAX_INDEX,
}

// version_major=2, version_minor=0
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ta_firmware_header_v2_0 {
    pub header: common_firmware_header,
    pub ta_fw_bin_count: u32,
    pub ta_fw_bin: [psp_fw_bin_desc; ],
}

// version_major=1, version_minor=0
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gfx_firmware_header_v1_0 {
    pub header: common_firmware_header,
    pub ucode_feature_version: u32,
    pub /: *mut *mut uint32_t jt_offset; / jt location,
    pub /: *mut *mut uint32_t jt_size; / size of jt,
}

// version_major=2, version_minor=0
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gfx_firmware_header_v2_0 {
    pub header: common_firmware_header,
    pub ucode_feature_version: u32,
    pub ucode_size_bytes: u32,
    pub ucode_offset_bytes: u32,
    pub data_size_bytes: u32,
    pub data_offset_bytes: u32,
    pub ucode_start_addr_lo: u32,
    pub ucode_start_addr_hi: u32,
}

// version_major=1, version_minor=0
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mes_firmware_header_v1_0 {
    pub header: common_firmware_header,
    pub mes_ucode_version: u32,
    pub mes_ucode_size_bytes: u32,
    pub mes_ucode_offset_bytes: u32,
    pub mes_ucode_data_version: u32,
    pub mes_ucode_data_size_bytes: u32,
    pub mes_ucode_data_offset_bytes: u32,
    pub mes_uc_start_addr_lo: u32,
    pub mes_uc_start_addr_hi: u32,
    pub mes_data_start_addr_lo: u32,
    pub mes_data_start_addr_hi: u32,
}

// version_major=1, version_minor=0
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rlc_firmware_header_v1_0 {
    pub header: common_firmware_header,
    pub ucode_feature_version: u32,
    pub save_and_restore_offset: u32,
    pub clear_state_descriptor_offset: u32,
    pub avail_scratch_ram_locations: u32,
    pub master_pkt_description_offset: u32,
}

// version_major=2, version_minor=0
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rlc_firmware_header_v2_0 {
    pub header: common_firmware_header,
    pub ucode_feature_version: u32,
    pub /: *mut *mut uint32_t jt_offset; / jt location,
    pub /: *mut *mut uint32_t jt_size; / size of jt,
    pub save_and_restore_offset: u32,
    pub clear_state_descriptor_offset: u32,
    pub avail_scratch_ram_locations: u32,
    pub reg_restore_list_size: u32,
    pub reg_list_format_start: u32,
    pub reg_list_format_separate_start: u32,
    pub starting_offsets_start: u32,
    pub /: *mut *mut uint32_t reg_list_format_size_bytes; / size of reg list format array in bytes,
    pub /: *mut *mut uint32_t reg_list_format_array_offset_bytes; / payload offset from the start of the header,
    pub /: *mut *mut uint32_t reg_list_size_bytes; / size of reg list array in bytes,
    pub /: *mut *mut uint32_t reg_list_array_offset_bytes; / payload offset from the start of the header,
    pub /: *mut *mut uint32_t reg_list_format_separate_size_bytes; / size of reg list format array in bytes,
    pub /: *mut *mut uint32_t reg_list_format_separate_array_offset_bytes; / payload offset from the start of the header,
    pub /: *mut *mut uint32_t reg_list_separate_size_bytes; / size of reg list array in bytes,
    pub /: *mut *mut uint32_t reg_list_separate_array_offset_bytes; / payload offset from the start of the header,
}

// version_major=2, version_minor=1
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rlc_firmware_header_v2_1 {
    pub v2_0: rlc_firmware_header_v2_0,
    pub /: *mut *mut uint32_t reg_list_format_direct_reg_list_length; / length of direct reg list format array,
    pub save_restore_list_cntl_ucode_ver: u32,
    pub save_restore_list_cntl_feature_ver: u32,
    pub save_restore_list_cntl_size_bytes: u32,
    pub save_restore_list_cntl_offset_bytes: u32,
    pub save_restore_list_gpm_ucode_ver: u32,
    pub save_restore_list_gpm_feature_ver: u32,
    pub save_restore_list_gpm_size_bytes: u32,
    pub save_restore_list_gpm_offset_bytes: u32,
    pub save_restore_list_srm_ucode_ver: u32,
    pub save_restore_list_srm_feature_ver: u32,
    pub save_restore_list_srm_size_bytes: u32,
    pub save_restore_list_srm_offset_bytes: u32,
}

// version_major=2, version_minor=2
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rlc_firmware_header_v2_2 {
    pub v2_1: rlc_firmware_header_v2_1,
    pub rlc_iram_ucode_size_bytes: u32,
    pub rlc_iram_ucode_offset_bytes: u32,
    pub rlc_dram_ucode_size_bytes: u32,
    pub rlc_dram_ucode_offset_bytes: u32,
}

// version_major=2, version_minor=3
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rlc_firmware_header_v2_3 {
    pub v2_2: rlc_firmware_header_v2_2,
    pub rlcp_ucode_version: u32,
    pub rlcp_ucode_feature_version: u32,
    pub rlcp_ucode_size_bytes: u32,
    pub rlcp_ucode_offset_bytes: u32,
    pub rlcv_ucode_version: u32,
    pub rlcv_ucode_feature_version: u32,
    pub rlcv_ucode_size_bytes: u32,
    pub rlcv_ucode_offset_bytes: u32,
}

// version_major=2, version_minor=4
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rlc_firmware_header_v2_4 {
    pub v2_3: rlc_firmware_header_v2_3,
    pub global_tap_delays_ucode_size_bytes: u32,
    pub global_tap_delays_ucode_offset_bytes: u32,
    pub se0_tap_delays_ucode_size_bytes: u32,
    pub se0_tap_delays_ucode_offset_bytes: u32,
    pub se1_tap_delays_ucode_size_bytes: u32,
    pub se1_tap_delays_ucode_offset_bytes: u32,
    pub se2_tap_delays_ucode_size_bytes: u32,
    pub se2_tap_delays_ucode_offset_bytes: u32,
    pub se3_tap_delays_ucode_size_bytes: u32,
    pub se3_tap_delays_ucode_offset_bytes: u32,
}

// version_major=2, version_minor=5
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rlc_firmware_header_v2_5 {
    pub v2_2: rlc_firmware_header_v2_2,
    pub rlc_1_iram_ucode_size_bytes: u32,
    pub rlc_1_iram_ucode_offset_bytes: u32,
    pub rlc_1_dram_ucode_size_bytes: u32,
    pub rlc_1_dram_ucode_offset_bytes: u32,
}

// version_major=1, version_minor=0
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdma_firmware_header_v1_0 {
    pub header: common_firmware_header,
    pub ucode_feature_version: u32,
    pub ucode_change_version: u32,
    pub /: *mut *mut uint32_t jt_offset; / jt location,
    pub /: *mut *mut uint32_t jt_size; / size of jt,
}

// version_major=1, version_minor=1
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdma_firmware_header_v1_1 {
    pub v1_0: sdma_firmware_header_v1_0,
    pub digest_size: u32,
}

// version_major=2, version_minor=0
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdma_firmware_header_v2_0 {
    pub header: common_firmware_header,
    pub ucode_feature_version: u32,
    pub /: *mut *mut uint32_t ctx_ucode_size_bytes; / context thread ucode size,
    pub /: *mut *mut uint32_t ctx_jt_offset; / context thread jt location,
    pub /: *mut *mut uint32_t ctx_jt_size; / context thread size of jt,
    pub ctl_ucode_offset: u32,
    pub /: *mut *mut uint32_t ctl_ucode_size_bytes; / control thread ucode size,
    pub /: *mut *mut uint32_t ctl_jt_offset; / control thread jt location,
    pub /: *mut *mut uint32_t ctl_jt_size; / control thread size of jt,
}

// version_major=1, version_minor=0
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpe_firmware_header_v1_0 {
    pub header: common_firmware_header,
    pub ucode_feature_version: u32,
    pub /: *mut *mut uint32_t ctx_ucode_size_bytes; / context thread ucode size,
    pub /: *mut *mut uint32_t ctx_jt_offset; / context thread jt location,
    pub /: *mut *mut uint32_t ctx_jt_size; / context thread size of jt,
    pub ctl_ucode_offset: u32,
    pub /: *mut *mut uint32_t ctl_ucode_size_bytes; / control thread ucode size,
    pub /: *mut *mut uint32_t ctl_jt_offset; / control thread jt location,
    pub /: *mut *mut uint32_t ctl_jt_size; / control thread size of jt,
}

// version_major=1, version_minor=0
#[repr(C)]
#[derive(Copy, Clone)]
pub struct umsch_mm_firmware_header_v1_0 {
    pub header: common_firmware_header,
    pub umsch_mm_ucode_version: u32,
    pub umsch_mm_ucode_size_bytes: u32,
    pub umsch_mm_ucode_offset_bytes: u32,
    pub umsch_mm_ucode_data_version: u32,
    pub umsch_mm_ucode_data_size_bytes: u32,
    pub umsch_mm_ucode_data_offset_bytes: u32,
    pub umsch_mm_irq_start_addr_lo: u32,
    pub umsch_mm_irq_start_addr_hi: u32,
    pub umsch_mm_uc_start_addr_lo: u32,
    pub umsch_mm_uc_start_addr_hi: u32,
    pub umsch_mm_data_start_addr_lo: u32,
    pub umsch_mm_data_start_addr_hi: u32,
}

// version_major=3, version_minor=0
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdma_firmware_header_v3_0 {
    pub header: common_firmware_header,
    pub ucode_feature_version: u32,
    pub ucode_offset_bytes: u32,
    pub ucode_size_bytes: u32,
}

// gpu info payload
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpu_info_firmware_v1_0 {
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
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpu_info_firmware_v1_1 {
    pub v1_0: gpu_info_firmware_v1_0,
    pub num_sc_per_sh: u32,
    pub num_packer_per_sc: u32,
}

// gpu info payload
// version_major=1, version_minor=1
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpu_info_firmware_v1_2 {
    pub v1_1: gpu_info_firmware_v1_1,
    pub soc_bounding_box: gpu_info_soc_bounding_box_v1_0,
}

// version_major=1, version_minor=0
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpu_info_firmware_header_v1_0 {
    pub header: common_firmware_header,
    pub /: *mut *mut uint16_t version_major; / version,
    pub /: *mut *mut uint16_t version_minor; / version,
}

// version_major=1, version_minor=0
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmcu_firmware_header_v1_0 {
    pub header: common_firmware_header,
    pub /: *mut *mut uint32_t intv_offset_bytes; / interrupt vectors offset from end of header, in bytes,
    pub /: *mut *mut uint32_t intv_size_bytes; / size of interrupt vectors, in bytes,
}

// version_major=1, version_minor=0
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmcub_firmware_header_v1_0 {
    pub header: common_firmware_header,
    pub /: *mut *mut uint32_t inst_const_bytes; / size of instruction region, in bytes,
    pub /: *mut *mut uint32_t bss_data_bytes; / size of bss/data region, in bytes,
}

// version_major=1, version_minor=0
#[repr(C)]
#[derive(Copy, Clone)]
pub struct imu_firmware_header_v1_0 {
    pub header: common_firmware_header,
    pub imu_iram_ucode_size_bytes: u32,
    pub imu_iram_ucode_offset_bytes: u32,
    pub imu_dram_ucode_size_bytes: u32,
    pub imu_dram_ucode_offset_bytes: u32,
}

// header is fixed size
#[repr(C)]
#[derive(Copy, Clone)]
pub union amdgpu_firmware_header {
    pub common: common_firmware_header,
    pub mc: mc_firmware_header_v1_0,
    pub smc: smc_firmware_header_v1_0,
    pub smc_v2_0: smc_firmware_header_v2_0,
    pub psp: psp_firmware_header_v1_0,
    pub psp_v1_1: psp_firmware_header_v1_1,
    pub psp_v1_3: psp_firmware_header_v1_3,
    pub psp_v2_0: psp_firmware_header_v2_0,
    pub psp_v2_1: psp_firmware_header_v2_0,
    pub ta: ta_firmware_header_v1_0,
    pub ta_v2_0: ta_firmware_header_v2_0,
    pub gfx: gfx_firmware_header_v1_0,
    pub gfx_v2_0: gfx_firmware_header_v2_0,
    pub rlc: rlc_firmware_header_v1_0,
    pub rlc_v2_0: rlc_firmware_header_v2_0,
    pub rlc_v2_1: rlc_firmware_header_v2_1,
    pub rlc_v2_2: rlc_firmware_header_v2_2,
    pub rlc_v2_3: rlc_firmware_header_v2_3,
    pub rlc_v2_4: rlc_firmware_header_v2_4,
    pub rlc_v2_5: rlc_firmware_header_v2_5,
    pub sdma: sdma_firmware_header_v1_0,
    pub sdma_v1_1: sdma_firmware_header_v1_1,
    pub sdma_v2_0: sdma_firmware_header_v2_0,
    pub sdma_v3_0: sdma_firmware_header_v3_0,
    pub gpu_info: gpu_info_firmware_header_v1_0,
    pub dmcu: dmcu_firmware_header_v1_0,
    pub dmcub: dmcub_firmware_header_v1_0,
    pub imu: imu_firmware_header_v1_0,
    pub raw: [u8; 0x100],
}

//
// fw loading support
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum AMDGPU_UCODE_ID {
    AMDGPU_UCODE_ID_CAP = 0,
    AMDGPU_UCODE_ID_SDMA0,
    AMDGPU_UCODE_ID_SDMA1,
    AMDGPU_UCODE_ID_SDMA2,
    AMDGPU_UCODE_ID_SDMA3,
    AMDGPU_UCODE_ID_SDMA4,
    AMDGPU_UCODE_ID_SDMA5,
    AMDGPU_UCODE_ID_SDMA6,
    AMDGPU_UCODE_ID_SDMA7,
    AMDGPU_UCODE_ID_SDMA_UCODE_TH0,
    AMDGPU_UCODE_ID_SDMA_UCODE_TH1,
    AMDGPU_UCODE_ID_SDMA_RS64,
    AMDGPU_UCODE_ID_CP_CE,
    AMDGPU_UCODE_ID_CP_PFP,
    AMDGPU_UCODE_ID_CP_ME,
    AMDGPU_UCODE_ID_CP_RS64_PFP,
    AMDGPU_UCODE_ID_CP_RS64_ME,
    AMDGPU_UCODE_ID_CP_RS64_MEC,
    AMDGPU_UCODE_ID_CP_RS64_PFP_P0_STACK,
    AMDGPU_UCODE_ID_CP_RS64_PFP_P1_STACK,
    AMDGPU_UCODE_ID_CP_RS64_ME_P0_STACK,
    AMDGPU_UCODE_ID_CP_RS64_ME_P1_STACK,
    AMDGPU_UCODE_ID_CP_RS64_MEC_P0_STACK,
    AMDGPU_UCODE_ID_CP_RS64_MEC_P1_STACK,
    AMDGPU_UCODE_ID_CP_RS64_MEC_P2_STACK,
    AMDGPU_UCODE_ID_CP_RS64_MEC_P3_STACK,
    AMDGPU_UCODE_ID_CP_MEC1,
    AMDGPU_UCODE_ID_CP_MEC1_JT,
    AMDGPU_UCODE_ID_CP_MEC2,
    AMDGPU_UCODE_ID_CP_MEC2_JT,
    AMDGPU_UCODE_ID_CP_MES,
    AMDGPU_UCODE_ID_CP_MES_DATA,
    AMDGPU_UCODE_ID_CP_MES1,
    AMDGPU_UCODE_ID_CP_MES1_DATA,
    AMDGPU_UCODE_ID_IMU_I,
    AMDGPU_UCODE_ID_IMU_D,
    AMDGPU_UCODE_ID_GLOBAL_TAP_DELAYS,
    AMDGPU_UCODE_ID_SE0_TAP_DELAYS,
    AMDGPU_UCODE_ID_SE1_TAP_DELAYS,
    AMDGPU_UCODE_ID_SE2_TAP_DELAYS,
    AMDGPU_UCODE_ID_SE3_TAP_DELAYS,
    AMDGPU_UCODE_ID_RLC_RESTORE_LIST_CNTL,
    AMDGPU_UCODE_ID_RLC_RESTORE_LIST_GPM_MEM,
    AMDGPU_UCODE_ID_RLC_RESTORE_LIST_SRM_MEM,
    AMDGPU_UCODE_ID_RLC_IRAM,
    AMDGPU_UCODE_ID_RLC_DRAM,
    AMDGPU_UCODE_ID_RLC_IRAM_1,
    AMDGPU_UCODE_ID_RLC_DRAM_1,
    AMDGPU_UCODE_ID_RLC_P,
    AMDGPU_UCODE_ID_RLC_V,
    AMDGPU_UCODE_ID_RLC_G,
    AMDGPU_UCODE_ID_STORAGE,
    AMDGPU_UCODE_ID_SMC,
    AMDGPU_UCODE_ID_PPTABLE,
    AMDGPU_UCODE_ID_UVD,
    AMDGPU_UCODE_ID_UVD1,
    AMDGPU_UCODE_ID_VCE,
    AMDGPU_UCODE_ID_VCN,
    AMDGPU_UCODE_ID_VCN1,
    AMDGPU_UCODE_ID_DMCU_ERAM,
    AMDGPU_UCODE_ID_DMCU_INTV,
    AMDGPU_UCODE_ID_VCN0_RAM,
    AMDGPU_UCODE_ID_VCN1_RAM,
    AMDGPU_UCODE_ID_DMCUB,
    AMDGPU_UCODE_ID_VPE_CTX,
    AMDGPU_UCODE_ID_VPE_CTL,
    AMDGPU_UCODE_ID_VPE,
    AMDGPU_UCODE_ID_UMSCH_MM_UCODE,
    AMDGPU_UCODE_ID_UMSCH_MM_DATA,
    AMDGPU_UCODE_ID_UMSCH_MM_CMD_BUFFER,
    AMDGPU_UCODE_ID_P2S_TABLE,
    AMDGPU_UCODE_ID_JPEG_RAM,
    AMDGPU_UCODE_ID_ISP,
    AMDGPU_UCODE_ID_MAXIMUM,
}

// engine firmware status
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum AMDGPU_UCODE_STATUS {
    AMDGPU_UCODE_STATUS_INVALID,
    AMDGPU_UCODE_STATUS_NOT_LOADED,
    AMDGPU_UCODE_STATUS_LOADED,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amdgpu_firmware_load_type {
    AMDGPU_FW_LOAD_DIRECT = 0,
    AMDGPU_FW_LOAD_PSP,
    AMDGPU_FW_LOAD_SMU,
    AMDGPU_FW_LOAD_RLC_BACKDOOR_AUTO,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amdgpu_ucode_required {
    AMDGPU_UCODE_OPTIONAL,
    AMDGPU_UCODE_REQUIRED,
}

// conform to smu_ucode_xfer_cz.h
pub const AMDGPU_SDMA0_UCODE_LOADED: c_uint = 0x00000001;
pub const AMDGPU_SDMA1_UCODE_LOADED: c_uint = 0x00000002;
pub const AMDGPU_CPCE_UCODE_LOADED: c_uint = 0x00000004;
pub const AMDGPU_CPPFP_UCODE_LOADED: c_uint = 0x00000008;
pub const AMDGPU_CPME_UCODE_LOADED: c_uint = 0x00000010;
pub const AMDGPU_CPMEC1_UCODE_LOADED: c_uint = 0x00000020;
pub const AMDGPU_CPMEC2_UCODE_LOADED: c_uint = 0x00000040;
pub const AMDGPU_CPRLC_UCODE_LOADED: c_uint = 0x00000100;
// amdgpu firmware info
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_firmware_info {
// ucode ID
    pub ucode_id: AMDGPU_UCODE_ID,
// request_firmware
    pub fw: *const firmware,
// starting mc address
    pub mc_addr: u64,
// kernel linear address
    pub kaddr: *mut c_void,
// ucode_size_bytes
    pub ucode_size: u32,
// starting tmr mc address
    pub tmr_mc_addr_lo: u32,
    pub tmr_mc_addr_hi: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_firmware {
    pub ucode: [amdgpu_firmware_info; AMDGPU_UCODE_ID_MAXIMUM],
    pub load_type: amdgpu_firmware_load_type,
    pub fw_buf: *mut amdgpu_bo,
    pub fw_size: c_uint,
    pub max_ucodes: c_uint,
// firmwares are loaded by psp instead of smu from vega10
    pub funcs: *const amdgpu_psp_funcs,
    pub rbuf: *mut amdgpu_bo,
    pub mutex: mutex,
// gpu info firmware data pointer
    pub gpu_info_fw: *const firmware,
    pub fw_buf_ptr: *mut c_void,
    pub fw_buf_mc: u64,
    pub pldm_version: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kicker_device {
    pub device: c_ushort,
    pub revision: u8,
}

extern "C" {
    pub fn amdgpu_ucode_print_mc_hdr(hdr: *const common_firmware_header);
}
extern "C" {
    pub fn amdgpu_ucode_print_smc_hdr(hdr: *const common_firmware_header);
}
extern "C" {
    pub fn amdgpu_ucode_print_imu_hdr(hdr: *const common_firmware_header);
}
extern "C" {
    pub fn amdgpu_ucode_print_gfx_hdr(hdr: *const common_firmware_header);
}
extern "C" {
    pub fn amdgpu_ucode_print_rlc_hdr(hdr: *const common_firmware_header);
}
extern "C" {
    pub fn amdgpu_ucode_print_sdma_hdr(hdr: *const common_firmware_header);
}
extern "C" {
    pub fn amdgpu_ucode_print_psp_hdr(hdr: *const common_firmware_header);
}
extern "C" {
    pub fn amdgpu_ucode_print_gpu_info_hdr(hdr: *const common_firmware_header);
}
extern "C" {
    pub fn amdgpu_ucode_release(fw: *const firmware);
}
extern "C" {
    pub fn amdgpu_ucode_init_bo(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_ucode_create_bo(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_ucode_sysfs_init(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_ucode_free_bo(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_ucode_sysfs_fini(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_ucode_ip_version_decode(adev: *mut amdgpu_device, block_type: c_int, ucode_prefix: *mut c_char, len: c_int);
}
extern "C" {
    pub fn amdgpu_is_kicker_fw(adev: *mut amdgpu_device) -> bool;
}
