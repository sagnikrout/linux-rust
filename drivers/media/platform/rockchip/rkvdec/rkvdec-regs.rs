//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/rockchip/rkvdec/rkvdec-regs.h
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
// REG_INTERRUPT is accessed via writel to enable the decoder after
// configuring it and clear interrupt strmd_error_status
//
pub const RKVDEC_REG_INTERRUPT: c_uint = 0x004;

pub const RKVDEC_REG_QOS_CTRL: c_uint = 0x18C;
//
// Cache configuration is not covered in the range of the register struct
//
pub const RKVDEC_REG_PREF_LUMA_CACHE_COMMAND: c_uint = 0x410;
pub const RKVDEC_REG_PREF_CHR_CACHE_COMMAND: c_uint = 0x450;
//
// Define the mode values
//
pub const RKVDEC_MODE_HEVC: c_int = 0;
pub const RKVDEC_MODE_H264: c_int = 1;
pub const RKVDEC_MODE_VP9: c_int = 2;
// rkvcodec registers
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkvdec_common_regs {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkvdec_id {
    pub 8: u32 minor_ver :,
    pub 1: u32 level :,
    pub 3: u32 dec_support :,
    pub 1: u32 profile :,
    pub 1: u32 reserved0 :,
    pub 1: u32 codec_flag :,
    pub 1: u32 reserved1 :,
    pub 16: u32 prod_num :,
    pub reg00: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkvdec_int {
    pub 1: u32 dec_e :,
    pub 1: u32 dec_clkgate_e :,
    pub 1: u32 dec_e_strmd_clkgate_dis :,
    pub 1: u32 timeout_mode :,
    pub 1: u32 dec_irq_dis :,
    pub 1: u32 dec_timeout_e :,
    pub 1: u32 buf_empty_en :,
    pub 1: u32 stmerror_waitdecfifo_empty :,
    pub 1: u32 dec_irq :,
    pub 1: u32 dec_irq_raw :,
    pub 2: u32 reserved2 :,
    pub 1: u32 dec_rdy_sta :,
    pub 1: u32 dec_bus_sta :,
    pub 1: u32 dec_error_sta :,
    pub 1: u32 dec_timeout_sta :,
    pub 1: u32 dec_empty_sta :,
    pub 1: u32 colmv_ref_error_sta :,
    pub 1: u32 cabu_end_sta :,
    pub 1: u32 h264orvp9_error_mode :,
    pub 1: u32 softrst_en_p :,
    pub 1: u32 force_softreset_valid :,
    pub 1: u32 softreset_rdy :,
    pub 1: u32 wr_ddr_align_en :,
    pub 1: u32 scl_down_en :,
    pub 1: u32 allow_not_wr_unref_bframe :,
    pub 6: u32 reserved1 :,
    pub reg01: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkvdec_sysctrl {
    pub 1: u32 in_endian :,
    pub 1: u32 in_swap32_e :,
    pub 1: u32 in_swap64_e :,
    pub 1: u32 str_endian :,
    pub 1: u32 str_swap32_e :,
    pub 1: u32 str_swap64_e :,
    pub 1: u32 out_endian :,
    pub 1: u32 out_swap32_e :,
    pub 1: u32 out_cbcr_swap :,
    pub 1: u32 reserved0 :,
    pub 1: u32 rlc_mode_direct_write :,
    pub 1: u32 rlc_mode :,
    pub 7: u32 strm_start_bit :,
    pub 1: u32 reserved1 :,
    pub 2: u32 dec_mode :,
    pub 2: u32 reserved2 :,
    pub 1: u32 rps_mode :,
    pub 1: u32 stream_mode :,
    pub 1: u32 stream_lastpacket :,
    pub 1: u32 firstslice_flag :,
    pub 1: u32 frame_orslice :,
    pub 1: u32 buspr_slot_disable :,
    pub 1: u32 colmv_mode :,
    pub 1: u32 ycacherd_prior :,
    pub reg02: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkvdec_picpar {
    pub 9: u32 y_hor_virstride :,
    pub 2: u32 reserved :,
    pub 1: u32 slice_num_highbit :,
    pub 9: u32 uv_hor_virstride :,
    pub 11: u32 slice_num_lowbits :,
    pub reg03: },
    pub strm_rlc_base: u32,
    pub stream_len: u32,
    pub cabactbl_base: u32,
    pub decout_base: u32,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkvdec_y_virstride {
    pub 20: u32 y_virstride :,
    pub 12: u32 reserved0 :,
    pub reg08: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkvdec_yuv_virstride {
    pub 21: u32 yuv_virstride :,
    pub 11: u32 reserved0 :,
    pub reg09: },
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ref_base {
    pub 1: u32 field_ref :,
    pub 1: u32 topfield_used_ref :,
    pub 1: u32 botfield_used_ref :,
    pub 1: u32 colmv_use_flag_ref :,
    pub 28: u32 base_addr :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkvdec_h26x_regs {
    pub ref0_14_base: [ref_base; 15],
    pub ref0_14_poc: [u32; 15],
    pub cur_poc: u32,
    pub rlcwrite_base: u32,
    pub pps_base: u32,
    pub rps_base: u32,
    pub strmd_error_e: u32,
    pub 28: u32 strmd_error_status :,
    pub 4: u32 colmv_error_ref_picidx :,
    pub reg45: },
    pub 8: u32 strmd_error_ctu_xoffset :,
    pub 8: u32 strmd_error_ctu_yoffset :,
    pub 7: u32 streamfifo_space2full :,
    pub 1: u32 reserved0 :,
    pub 1: u32 vp9_error_ctu0_en :,
    pub 7: u32 reserved1 :,
    pub reg46: },
    pub 9: u32 saowr_xoffet :,
    pub 7: u32 reserved0 :,
    pub 10: u32 saowr_yoffset :,
    pub 6: u32 reserved1 :,
    pub reg47: },
    pub ref15_base: ref_base,
    pub ref15_29_poc: [u32; 15],
    pub performance_cycle: u32,
    pub axi_ddr_rdata: u32,
    pub axi_ddr_wdata: u32,
    pub 1: u32 busifd_resetn :,
    pub 1: u32 cabac_resetn :,
    pub 1: u32 dec_ctrl_resetn :,
    pub 1: u32 transd_resetn :,
    pub 1: u32 intra_resetn :,
    pub 1: u32 inter_resetn :,
    pub 1: u32 recon_resetn :,
    pub 1: u32 filer_resetn :,
    pub 24: u32 reserved0 :,
    pub reg67: },
    pub 6: u32 perf_cnt0_sel :,
    pub 2: u32 reserved0 :,
    pub 6: u32 perf_cnt1_sel :,
    pub 2: u32 reserved1 :,
    pub 6: u32 perf_cnt2_sel :,
    pub 10: u32 reserved2 :,
    pub reg68: },
    pub perf_cnt0: u32,
    pub perf_cnt1: u32,
    pub perf_cnt2: u32,
    pub ref30_poc: u32,
    pub ref31_poc: u32,
    pub cur_poc1: u32,
    pub errorinfo_base: u32,
    pub 14: u32 slicedec_num :,
    pub 1: u32 reserved0 :,
    pub 1: u32 strmd_detect_error_flag :,
    pub 14: u32 error_packet_num :,
    pub 2: u32 reserved1 :,
    pub reg76: },
    pub 30: u32 error_en_highbits :,
    pub 1: u32 strmd_error_slice_en :,
    pub 1: u32 strmd_error_frame_en :,
    pub reg77: },
    pub colmv_cur_base: u32,
    pub colmv_ref_base: [u32; 16],
    pub scanlist_addr: u32,
    pub reg96_sd_decout_base: u32,
    pub sd_y_virstride: u32,
    pub sd_hor_stride: u32,
    pub qos_ctrl: u32,
    pub perf: [u32; 8],
    pub qos1: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkvdec_vp9_regs {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cprheader_offset {
    pub 16: u32 cprheader_offset :,
    pub 16: u32 reserved :,
    pub reg10: },
    pub refer_bases: [u32; 3],
    pub count_base: u32,
    pub segidlast_base: u32,
    pub segidcur_base: u32,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct frame_sizes {
    pub 16: u32 framewidth :,
    pub 16: u32 frameheight :,
    pub reg17_19: [}; 3],
#[repr(C)]
#[derive(Copy, Clone)]
pub struct segid_grp {
    pub 1: u32 segid_abs_delta :,
    pub 1: u32 segid_frame_qp_delta_en :,
    pub 9: u32 segid_frame_qp_delta :,
    pub 1: u32 segid_frame_loopfilter_value_en :,
    pub 7: u32 segid_frame_loopfilter_value :,
    pub 1: u32 segid_referinfo_en :,
    pub 2: u32 segid_referinfo :,
    pub 1: u32 segid_frame_skip_en :,
    pub 9: u32 reserved :,
    pub reg20_27: [}; 8],
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cprheader_config {
    pub 3: u32 tx_mode :,
    pub 2: u32 frame_reference_mode :,
    pub 27: u32 reserved :,
    pub reg28: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ref_scale {
    pub 16: u32 ref_hor_scale :,
    pub 16: u32 ref_ver_scale :,
    pub reg29_31: [}; 3],
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ref_deltas_lastframe {
    pub 7: u32 ref_deltas_lastframe0 :,
    pub 7: u32 ref_deltas_lastframe1 :,
    pub 7: u32 ref_deltas_lastframe2 :,
    pub 7: u32 ref_deltas_lastframe3 :,
    pub 4: u32 reserved :,
    pub reg32: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct info_lastframe {
    pub 7: u32 mode_deltas_lastframe0 :,
    pub 7: u32 mode_deltas_lastframe1 :,
    pub 2: u32 reserved0 :,
    pub 1: u32 segmentation_enable_lstframe :,
    pub 1: u32 last_show_frame :,
    pub 1: u32 last_intra_only :,
    pub 1: u32 last_widthheight_eqcur :,
    pub 3: u32 color_space_lastkeyframe :,
    pub 9: u32 reserved1 :,
    pub reg33: },
    pub intercmd_base: u32,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intercmd_num {
    pub 24: u32 intercmd_num :,
    pub 8: u32 reserved :,
    pub reg35: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lasttile_size {
    pub 24: u32 lasttile_size :,
    pub 8: u32 reserved :,
    pub reg36: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hor_virstride {
    pub 9: u32 y_hor_virstride :,
    pub 7: u32 reserved0 :,
    pub 9: u32 uv_hor_virstride :,
    pub 7: u32 reserved1 :,
    pub reg37_39: [}; 3],
    pub cur_poc: u32,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rlcwrite_base {
    pub 3: u32 reserved :,
    pub 29: u32 rlcwrite_base :,
    pub reg41: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pps_base {
    pub 4: u32 reserved :,
    pub 28: u32 pps_base :,
    pub reg42: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rps_base {
    pub 4: u32 reserved :,
    pub 28: u32 rps_base :,
    pub reg43: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct strmd_error_en {
    pub 28: u32 strmd_error_e :,
    pub 4: u32 reserved :,
    pub reg44: },
    pub vp9_error_info0: u32,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct strmd_error_ctu {
    pub 8: u32 strmd_error_ctu_xoffset :,
    pub 8: u32 strmd_error_ctu_yoffset :,
    pub 7: u32 streamfifo_space2full :,
    pub 1: u32 reserved0 :,
    pub 1: u32 error_ctu0_en :,
    pub 7: u32 reserved1 :,
    pub reg46: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sao_ctu_position {
    pub 9: u32 saowr_xoffet :,
    pub 7: u32 reserved0 :,
    pub 10: u32 saowr_yoffset :,
    pub 6: u32 reserved1 :,
    pub reg47: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ystride {
    pub 20: u32 virstride :,
    pub 12: u32 reserved :,
    pub reg48_50: [}; 3],
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lastref_yuvstride {
    pub 21: u32 lastref_yuv_virstride :,
    pub 11: u32 reserved :,
    pub reg51: },
    pub refcolmv_base: u32,
    pub reserved0: [u32; 11],
    pub performance_cycle: u32,
    pub axi_ddr_rdata: u32,
    pub axi_ddr_wdata: u32,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fpgadebug_reset {
    pub 1: u32 busifd_resetn :,
    pub 1: u32 cabac_resetn :,
    pub 1: u32 dec_ctrl_resetn :,
    pub 1: u32 transd_resetn :,
    pub 1: u32 intra_resetn :,
    pub 1: u32 inter_resetn :,
    pub 1: u32 recon_resetn :,
    pub 1: u32 filer_resetn :,
    pub 24: u32 reserved :,
    pub reg67: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct performance_sel {
    pub 6: u32 perf_cnt0_sel :,
    pub 2: u32 reserved0 :,
    pub 6: u32 perf_cnt1_sel :,
    pub 2: u32 reserved1 :,
    pub 6: u32 perf_cnt2_sel :,
    pub 10: u32 reserved :,
    pub reg68: },
    pub perf_cnt0: u32,
    pub perf_cnt1: u32,
    pub perf_cnt2: u32,
    pub reserved1: [u32; 3],
    pub vp9_error_info1: u32,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct error_ctu1 {
    pub 6: u32 vp9_error_ctu1_x :,
    pub 2: u32 reserved0 :,
    pub 6: u32 vp9_error_ctu1_y :,
    pub 1: u32 reserved1 :,
    pub 1: u32 vp9_error_ctu1_en :,
    pub 16: u32 reserved2 :,
    pub reg76: },
    pub reserved2: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkvdec_regs {
    pub common: rkvdec_common_regs,
    pub h26x: rkvdec_h26x_regs,
    pub vp9: rkvdec_vp9_regs,
}
