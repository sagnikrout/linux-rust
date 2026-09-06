//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/rockchip/rkvdec/rkvdec-vdpu383-regs.h
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
//
// Rockchip Video Decoder VDPU383 driver registers description
//
// Copyright (C) 2025 Collabora, Ltd.
// Detlev Casanova <detlev.casanova@collabora.com>
//

pub const VDPU383_MODE_HEVC: c_int = 0;
pub const VDPU383_MODE_H264: c_int = 1;

pub const VDPU383_LINK_TIMEOUT_THRESHOLD: c_uint = 0x54;
pub const VDPU383_LINK_IP_ENABLE: c_uint = 0x58;

pub const VDPU383_LINK_DEC_ENABLE: c_uint = 0x40;

pub const VDPU383_LINK_INT_EN: c_uint = 0x048;

pub const VDPU383_LINK_STA_INT: c_uint = 0x04c;

pub const VDPU383_STA_INT_ALL: c_uint = 0x3ff;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vdpu383_regs_common {
    pub reg008_dec_mode: u32,
    pub 1: u32 fbc_e :,
    pub 1: u32 tile_e :,
    pub 2: u32 reserve0 :,
    pub 1: u32 buf_empty_en :,
    pub 1: u32 scale_down_en :,
    pub 1: u32 reserve1 :,
    pub 1: u32 pix_range_det_e :,
    pub 1: u32 av1_fgs_en :,
    pub 7: u32 reserve2 :,
    pub 1: u32 line_irq_en :,
    pub 1: u32 out_cbcr_swap :,
    pub 1: u32 fbc_force_uncompress :,
    pub 1: u32 fbc_sparse_mode :,
    pub 12: u32 reserve3 :,
    pub reg009_important_en: },
    pub 1: u32 strmd_auto_gating_e :,
    pub 1: u32 inter_auto_gating_e :,
    pub 1: u32 intra_auto_gating_e :,
    pub 1: u32 transd_auto_gating_e :,
    pub 1: u32 recon_auto_gating_e :,
    pub 1: u32 filterd_auto_gating_e :,
    pub 1: u32 bus_auto_gating_e :,
    pub 1: u32 ctrl_auto_gating_e :,
    pub 1: u32 rcb_auto_gating_e :,
    pub 1: u32 err_prc_auto_gating_e :,
    pub 22: u32 reserve0 :,
    pub reg010_block_gating_en: },
    pub 9: u32 reserve0 :,
    pub 1: u32 dec_timeout_dis :,
    pub 22: u32 reserve1 :,
    pub reg011_cfg_para: },
    pub 7: u32 reserve0 :,
    pub 25: u32 cache_hash_mask :,
    pub reg012_cache_hash_mask: },
    pub reg013_core_timeout_threshold: u32,
    pub 16: u32 dec_line_irq_step :,
    pub 16: u32 dec_line_offset_y_st :,
    pub reg014_line_irq_ctrl: },
    pub 1: u32 rkvdec_frame_rdy_sta :,
    pub 1: u32 rkvdec_strm_error_sta :,
    pub 1: u32 rkvdec_core_timeout_sta :,
    pub 1: u32 rkvdec_ip_timeout_sta :,
    pub 1: u32 rkvdec_bus_error_sta :,
    pub 1: u32 rkvdec_buffer_empty_sta :,
    pub 1: u32 rkvdec_colmv_ref_error_sta :,
    pub 1: u32 rkvdec_error_spread_sta :,
    pub 1: u32 create_core_timeout_sta :,
    pub 1: u32 wlast_miss_match_sta :,
    pub 1: u32 rkvdec_core_rst_rdy_sta :,
    pub 1: u32 rkvdec_ip_rst_rdy_sta :,
    pub 1: u32 force_busidle_rdy_sta :,
    pub 1: u32 ltb_pause_rdy_sta :,
    pub 1: u32 ltb_end_flag :,
    pub 1: u32 unsupport_decmode_error_sta :,
    pub 15: u32 wmask_bits :,
    pub 1: u32 reserve0 :,
    pub reg015_irq_sta: },
    pub 1: u32 error_proc_disable :,
    pub 7: u32 reserve0 :,
    pub 1: u32 error_spread_disable :,
    pub 15: u32 reserve1 :,
    pub 1: u32 roi_error_ctu_cal_en :,
    pub 7: u32 reserve2 :,
    pub reg016_error_ctrl_set: },
    pub 12: u32 roi_x_ctu_offset_st :,
    pub 4: u32 reserve0 :,
    pub 12: u32 roi_y_ctu_offset_st :,
    pub 4: u32 reserve1 :,
    pub reg017_err_roi_ctu_offset_start: },
    pub 12: u32 roi_x_ctu_offset_end :,
    pub 4: u32 reserve0 :,
    pub 12: u32 roi_y_ctu_offset_end :,
    pub 4: u32 reserve1 :,
    pub reg018_err_roi_ctu_offset_end: },
    pub 1: u32 avs2_ref_error_field :,
    pub 1: u32 avs2_ref_error_topfield :,
    pub 1: u32 ref_error_topfield_used :,
    pub 1: u32 ref_error_botfield_used :,
    pub 28: u32 reserve0 :,
    pub reg019_error_ref_info: },
    pub reg020_cabac_error_en_lowbits: u32,
    pub reg021_cabac_error_en_highbits: u32,
    pub reg022_reserved: u32,
    pub 10: u32 fill_y :,
    pub 10: u32 fill_u :,
    pub 10: u32 fill_v :,
    pub 2: u32 reserve0 :,
    pub reg023_invalid_pixel_fill: },
    pub reg024_026_reserved: [u32; 3],
    pub 4: u32 reserve0 :,
    pub 1: u32 ctu_align_wr_en :,
    pub 27: u32 reserve1 :,
    pub reg027_align_en: },
    pub 1: u32 axi_perf_work_e :,
    pub 2: u32 reserve0 :,
    pub 1: u32 axi_cnt_type :,
    pub 8: u32 rd_latency_id :,
    pub 4: u32 reserve1 :,
    pub 12: u32 rd_latency_thr :,
    pub 4: u32 reserve2 :,
    pub reg028_debug_perf_latency_ctrl0: },
    pub 2: u32 addr_align_type :,
    pub 1: u32 ar_cnt_id_type :,
    pub 1: u32 aw_cnt_id_type :,
    pub 8: u32 ar_count_id :,
    pub 4: u32 reserve0 :,
    pub 8: u32 aw_count_id :,
    pub 1: u32 rd_band_width_mode :,
    pub 7: u32 reserve1 :,
    pub reg029_debug_perf_latency_ctrl1: },
    pub 4: u32 axi_wr_qos_level :,
    pub 4: u32 reserve0 :,
    pub 4: u32 axi_wr_qos :,
    pub 4: u32 reserve1 :,
    pub 4: u32 axi_rd_qos_level :,
    pub 4: u32 reserve2 :,
    pub 4: u32 axi_rd_qos :,
    pub 4: u32 reserve3 :,
    pub reg030_qos_ctrl: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vdpu383_regs_common_addr {
    pub reg128_strm_base: u32,
    pub reg129_rps_base: u32,
    pub reg130_cabactbl_base: u32,
    pub reg131_gbl_base: u32,
    pub reg132_scanlist_addr: u32,
    pub reg133_scale_down_base: u32,
    pub reg134_fgs_base: u32,
    pub reg135_139_reserved: [u32; 5],
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rcb_info {
    pub offset: u32,
    pub size: u32,
    pub reg140_162_rcb_info: [}; 11],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vdpu383_regs_h26x_addr {
    pub reg168_decout_base: u32,
    pub reg169_error_ref_base: u32,
    pub reg170_185_ref_base: [u32; 16],
    pub reg186_191_reserved: [u32; 6],
    pub reg192_payload_st_cur_base: u32,
    pub reg193_fbc_payload_offset: u32,
    pub reg194_payload_st_error_ref_base: u32,
    pub reg195_210_payload_st_ref_base: [u32; 16],
    pub reg211_215_reserved: [u32; 5],
    pub reg216_colmv_cur_base: u32,
    pub reg217_232_colmv_ref_base: [u32; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vdpu383_regs_h26x_params {
    pub reg064_start_decoder: u32,
    pub reg065_strm_start_bit: u32,
    pub reg066_stream_len: u32,
    pub reg067_global_len: u32,
    pub reg068_hor_virstride: u32,
    pub reg069_raster_uv_hor_virstride: u32,
    pub reg070_y_virstride: u32,
    pub reg071_scl_ref_hor_virstride: u32,
    pub reg072_scl_ref_raster_uv_hor_virstride: u32,
    pub reg073_scl_ref_virstride: u32,
    pub reg074_fgs_ref_hor_virstride: u32,
    pub reg075_079_reserved: [u32; 5],
    pub reg080_error_ref_hor_virstride: u32,
    pub reg081_error_ref_raster_uv_hor_virstride: u32,
    pub reg082_error_ref_virstride: u32,
    pub reg083_ref0_hor_virstride: u32,
    pub reg084_ref0_raster_uv_hor_virstride: u32,
    pub reg085_ref0_virstride: u32,
    pub reg086_ref1_hor_virstride: u32,
    pub reg087_ref1_raster_uv_hor_virstride: u32,
    pub reg088_ref1_virstride: u32,
    pub reg089_ref2_hor_virstride: u32,
    pub reg090_ref2_raster_uv_hor_virstride: u32,
    pub reg091_ref2_virstride: u32,
    pub reg092_ref3_hor_virstride: u32,
    pub reg093_ref3_raster_uv_hor_virstride: u32,
    pub reg094_ref3_virstride: u32,
    pub reg095_ref4_hor_virstride: u32,
    pub reg096_ref4_raster_uv_hor_virstride: u32,
    pub reg097_ref4_virstride: u32,
    pub reg098_ref5_hor_virstride: u32,
    pub reg099_ref5_raster_uv_hor_virstride: u32,
    pub reg100_ref5_virstride: u32,
    pub reg101_ref6_hor_virstride: u32,
    pub reg102_ref6_raster_uv_hor_virstride: u32,
    pub reg103_ref6_virstride: u32,
    pub reg104_ref7_hor_virstride: u32,
    pub reg105_ref7_raster_uv_hor_virstride: u32,
    pub reg106_ref7_virstride: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vdpu383_regs_h26x {
    pub /: *mut *mut vdpu383_regs_common common; / 8-30,
    pub /: *mut *mut vdpu383_regs_h26x_params h26x_params; / 64-74, 80-106,
    pub /: *mut *mut vdpu383_regs_common_addr common_addr; / 128-134, 140-161,
    pub /: *mut *mut vdpu383_regs_h26x_addr h26x_addr; / 168-185, 192-210, 216-232,
    pub __packed: },
