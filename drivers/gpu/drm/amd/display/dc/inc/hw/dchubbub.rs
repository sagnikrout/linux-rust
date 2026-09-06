//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/inc/hw/dchubbub.h
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
// Copyright 2012-15 Advanced Micro Devices, Inc.
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
// Authors: AMD
//
// DOC: overview
//
// There is only one common DCHUBBUB. It contains the common request and return
// blocks for the Data Fabric Interface that are not clock/power gated.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dcc_control {
    dcc_control__256_256_xxx,
    dcc_control__128_128_xxx,
    dcc_control__256_64_64,
    dcc_control__256_128_128,
    dcc_control__256_256,
    dcc_control__256_128,
    dcc_control__256_64,

}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum segment_order {
    segment_order__na,
    segment_order__contiguous,
    segment_order__non_contiguous,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn_hubbub_wm_set {
    pub wm_set: u32,
    pub data_urgent: u32,
    pub pte_meta_urgent: u32,
    pub sr_enter: u32,
    pub sr_exit: u32,
    pub dram_clk_change: u32,
    pub usr_retrain: u32,
    pub fclk_pstate_change: u32,
    pub sr_enter_exit_Z8: u32,
    pub sr_enter_Z8: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn_hubbub_wm {
    pub sets: [dcn_hubbub_wm_set; 4],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dcn_hubbub_page_table_depth {
    DCN_PAGE_TABLE_DEPTH_1_LEVEL,
    DCN_PAGE_TABLE_DEPTH_2_LEVEL,
    DCN_PAGE_TABLE_DEPTH_3_LEVEL,
    DCN_PAGE_TABLE_DEPTH_4_LEVEL
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dcn_hubbub_page_table_block_size {
    DCN_PAGE_TABLE_BLOCK_SIZE_4KB = 0,
    DCN_PAGE_TABLE_BLOCK_SIZE_8KB = 1,
    DCN_PAGE_TABLE_BLOCK_SIZE_16KB = 2,
    DCN_PAGE_TABLE_BLOCK_SIZE_32KB = 3,
    DCN_PAGE_TABLE_BLOCK_SIZE_64KB = 4,
    DCN_PAGE_TABLE_BLOCK_SIZE_128KB = 5,
    DCN_PAGE_TABLE_BLOCK_SIZE_256KB = 6,
    DCN_PAGE_TABLE_BLOCK_SIZE_512KB = 7,
    DCN_PAGE_TABLE_BLOCK_SIZE_1024KB = 8,
    DCN_PAGE_TABLE_BLOCK_SIZE_2048KB = 9
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn_hubbub_phys_addr_config {
    pub fb_top: u64,
    pub fb_offset: u64,
    pub fb_base: u64,
    pub agp_top: u64,
    pub agp_bot: u64,
    pub agp_base: u64,
    pub system_aperture: },
    pub page_table_start_addr: u64,
    pub page_table_end_addr: u64,
    pub page_table_base_addr: u64,
    pub gart_config: },
    pub page_table_default_page_addr: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn_hubbub_virt_addr_config {
    pub page_table_start_addr: u64,
    pub page_table_end_addr: u64,
    pub page_table_block_size: dcn_hubbub_page_table_block_size,
    pub page_table_depth: dcn_hubbub_page_table_depth,
    pub page_table_base_addr: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hubbub_addr_config {
    pub pa_config: dcn_hubbub_phys_addr_config,
    pub va_config: dcn_hubbub_virt_addr_config,
    pub aperture_check_fault: u64,
    pub generic_fault: u64,
    pub default_addrs: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn_hubbub_state {
    pub vm_fault_addr_msb: u32,
    pub vm_fault_addr_lsb: u32,
    pub vm_error_status: u32,
    pub vm_error_vmid: u32,
    pub vm_error_pipe: u32,
    pub vm_error_mode: u32,
    pub test_debug_data: u32,
    pub watermark_change_cntl: u32,
    pub dram_state_cntl: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn_hubbub_reg_state {
    pub det0_ctrl: u32,
    pub det1_ctrl: u32,
    pub det2_ctrl: u32,
    pub det3_ctrl: u32,
    pub compbuf_ctrl: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hubbub_urgent_latency_params {
    pub refclk_mhz: u32,
    pub t_win_ns: u32,
    pub bandwidth_mbps: u32,
    pub bw_factor_x1000: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hubbub_funcs {
    pub dh_data): *mut dchub_init_data,
    pub pa_config): *mut dcn_hubbub_phys_addr_config,
    pub vmid): c_int,
    pub output): *mut dc_surface_dcc_cap,
    pub segment_order_vert): *mut segment_order,
    pub segment_order_vert): *mut segment_order,
    pub plane1_bpe): *mut c_uint,
    pub bytes_per_element): *mut c_uint,
    pub wm): *mut dcn_hubbub_wm,
    pub dchub_ref_freq_inKhz): *mut c_uint,
    pub safe_to_lower): bool,
    pub hubbub): *mut *mut bool (is_allow_self_refresh_enabled)(struct hubbub,
    pub allow): *mut *mut *mut void (allow_self_refresh_control)(struct hubbub hubbub, bool,
    pub hubbub): *mut *mut bool (verify_allow_pstate_change_high)(struct hubbub,
    pub hubbub): *mut *mut void (apply_DEDCN21_147_wa)(struct hubbub,
    pub hubbub): *mut *mut void (force_wm_propagate_to_pipes)(struct hubbub,
    pub hubbub_state): *mut *mut *mut void (hubbub_read_state)(struct hubbub hubbub, struct dcn_hubbub_state,
    pub allow): *mut *mut *mut void (force_pstate_change_control)(struct hubbub hubbub, bool force, bool,
    pub hubbub): *mut *mut void (init_watermarks)(struct hubbub,
    pub hubbub_reg_state): *mut *mut *mut void (hubbub_read_reg_state)(struct hubbub hubbub, struct dcn_hubbub_reg_state,
//
// @program_det_size:
//
// DE-Tile buffers (DET) is a memory that is used to convert the tiled
// data into linear, which the rest of the display can use to generate
// the graphics output. One of the main features of this component is
// that each pipe has a configurable DET buffer which means that when a
// pipe is not enabled, the device can assign the memory to other
// enabled pipes to try to be more efficient.
//
// DET logic is handled by dchubbub. Some ASICs provide a feature named
// Configurable Return Buffer (CRB) segments which can be allocated to
// compressed or detiled buffers.
//
    pub det_buffer_size_in_kbyte): *mut *mut *mut void (program_det_size)(struct hubbub hubbub, int hubp_inst, unsigned,
    pub hubp_inst): *mut *mut *mut void (wait_for_det_apply)(struct hubbub hubbub, int,
    pub safe_to_increase): *mut *mut *mut void (program_compbuf_size)(struct hubbub hubbub, unsigned compbuf_size_kb, bool,
    pub hubbub): *mut *mut void (init_crb)(struct hubbub,
    pub allow): *mut *mut *mut void (force_usr_retraining_allow)(struct hubbub hubbub, bool,
    pub words_per_channel): *mut *mut *mut void (set_request_limit)(struct hubbub hubbub, int memory_channel_count, int,
    pub hubbub): *mut *mut void (dchubbub_init)(struct hubbub,
    pub mall_in_use): *mut *mut *mut void (get_mall_en)(struct hubbub hubbub, unsigned int,
    pub det_buffer_size_seg): *mut *mut *mut void (program_det_segments)(struct hubbub hubbub, int hubp_inst, unsigned,
    pub safe_to_increase): *mut *mut *mut void (program_compbuf_segments)(struct hubbub hubbub, unsigned compbuf_size_seg, bool,
    pub hubp_inst): *mut *mut *mut void (wait_for_det_update)(struct hubbub hubbub, int,
    pub safe_to_lower): *mut *mut *mut *mut bool (program_arbiter)(struct hubbub hubbub, struct dml2_display_arb_regs arb_regs, bool,
    pub hubbub): *mut *mut void (dchvm_init)(struct hubbub,
// Performance monitoring related functions
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hubbub_perfmon_funcs {
    pub hubbub): *mut *mut void (reset)(struct hubbub,
    pub hubbub): *mut hubbub,
    pub avg_latency_ns): *mut *mut uint32_t max_latency_ns, uint32_t,
    pub hubbub): *mut hubbub,
    pub timestamp_us): *mut u32,
    pub params): *const hubbub_urgent_latency_params,
    pub refclk_mhz): u32,
    pub hubbub): *mut hubbub,
    pub hubbub): *mut hubbub,
    pub duration_ns): *mut uint32_t refclk_mhz, uint32_t,
    pub hubbub): *mut hubbub,
    pub duration_ns): *mut u32,
    pub hubbub): *mut hubbub,
    pub hubbub): *mut *mut uint32_t (get_prefetch_data_size)(struct hubbub,
    pub perfmon: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hubbub_qos_funcs {
    pub hubbub): *mut *mut void (force_display_nominal_profile)(struct hubbub,
    pub hubbub): *mut *mut void (force_display_urgent_profile)(struct hubbub,
    pub hubbub): *mut *mut void (reset_display_qos_profile)(struct hubbub,
    pub qos: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hubbub {
    pub funcs: *const hubbub_funcs,
    pub ctx: *mut dc_context,
    pub riommu_active: bool,
}
