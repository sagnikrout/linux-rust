//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/mediatek/mdp3/mdp_sm_mt8195.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (c) 2023 MediaTek Inc.
// Author: Ping-Hsun Wu <ping-hsun.wu@mediatek.com>
//

//
// ISP-MDP generic output information
// MD5 of the target SCP prebuild:
// a49ec487e458b5971880f1b63dc2a9d5
//
pub const IMG_MAX_SUBFRAMES_8195: c_int = 20;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct img_comp_frame_8195 {
    pub output_disable: u32,
    pub bypass: u32,
    pub in_width: u32,
    pub in_height: u32,
    pub out_width: u32,
    pub out_height: u32,
    pub crop: img_crop,
    pub in_total_width: u32,
    pub out_total_width: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct img_comp_subfrm_8195 {
    pub tile_disable: u32,
    pub in: img_region,
    pub out: img_region,
    pub luma: img_offset,
    pub chroma: img_offset,
    pub /: *mut *mut s32 out_vertical; / Output vertical index,
    pub /: *mut *mut s32 out_horizontal; / Output horizontal index,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdp_rdma_subfrm_8195 {
    pub offset: [u32; IMG_MAX_PLANES],
    pub offset_0_p: u32,
    pub src: u32,
    pub clip: u32,
    pub clip_ofst: u32,
    pub in_tile_xleft: u32,
    pub in_tile_ytop: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdp_rdma_data_8195 {
    pub src_ctrl: u32,
    pub comp_ctrl: u32,
    pub control: u32,
    pub iova: [u32; IMG_MAX_PLANES],
    pub iova_end: [u32; IMG_MAX_PLANES],
    pub mf_bkgd: u32,
    pub mf_bkgd_in_pxl: u32,
    pub sf_bkgd: u32,
    pub ufo_dec_y: u32,
    pub ufo_dec_c: u32,
    pub transform: u32,
    pub dmabuf_con0: u32,
    pub ultra_th_high_con0: u32,
    pub ultra_th_low_con0: u32,
    pub dmabuf_con1: u32,
    pub ultra_th_high_con1: u32,
    pub ultra_th_low_con1: u32,
    pub dmabuf_con2: u32,
    pub ultra_th_high_con2: u32,
    pub ultra_th_low_con2: u32,
    pub dmabuf_con3: u32,
    pub subfrms: [mdp_rdma_subfrm_8195; IMG_MAX_SUBFRAMES_8195],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdp_fg_subfrm_8195 {
    pub info_0: u32,
    pub info_1: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdp_fg_data_8195 {
    pub ctrl_0: u32,
    pub ck_en: u32,
    pub subfrms: [mdp_fg_subfrm_8195; IMG_MAX_SUBFRAMES_8195],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdp_hdr_subfrm_8195 {
    pub win_size: u32,
    pub src: u32,
    pub clip_ofst0: u32,
    pub clip_ofst1: u32,
    pub hist_ctrl_0: u32,
    pub hist_ctrl_1: u32,
    pub hdr_top: u32,
    pub hist_addr: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdp_hdr_data_8195 {
    pub top: u32,
    pub relay: u32,
    pub subfrms: [mdp_hdr_subfrm_8195; IMG_MAX_SUBFRAMES_8195],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdp_aal_subfrm_8195 {
    pub src: u32,
    pub clip: u32,
    pub clip_ofst: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdp_aal_data_8195 {
    pub cfg_main: u32,
    pub cfg: u32,
    pub subfrms: [mdp_aal_subfrm_8195; IMG_MAX_SUBFRAMES_8195],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdp_rsz_subfrm_8195 {
    pub control2: u32,
    pub src: u32,
    pub clip: u32,
    pub hdmirx_en: u32,
    pub luma_h_int_ofst: u32,
    pub luma_h_sub_ofst: u32,
    pub luma_v_int_ofst: u32,
    pub luma_v_sub_ofst: u32,
    pub chroma_h_int_ofst: u32,
    pub chroma_h_sub_ofst: u32,
    pub rsz_switch: u32,
    pub merge_cfg: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdp_rsz_data_8195 {
    pub coeff_step_x: u32,
    pub coeff_step_y: u32,
    pub control1: u32,
    pub control2: u32,
    pub etc_control: u32,
    pub prz_enable: u32,
    pub ibse_softclip: u32,
    pub tap_adapt: u32,
    pub ibse_gaincontrol1: u32,
    pub ibse_gaincontrol2: u32,
    pub ibse_ylevel_1: u32,
    pub ibse_ylevel_2: u32,
    pub ibse_ylevel_3: u32,
    pub ibse_ylevel_4: u32,
    pub ibse_ylevel_5: u32,
    pub subfrms: [mdp_rsz_subfrm_8195; IMG_MAX_SUBFRAMES_8195],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdp_tdshp_subfrm_8195 {
    pub src: u32,
    pub clip: u32,
    pub clip_ofst: u32,
    pub hist_cfg_0: u32,
    pub hist_cfg_1: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdp_tdshp_data_8195 {
    pub cfg: u32,
    pub subfrms: [mdp_tdshp_subfrm_8195; IMG_MAX_SUBFRAMES_8195],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdp_color_subfrm_8195 {
    pub in_hsize: u32,
    pub in_vsize: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdp_color_data_8195 {
    pub start: u32,
    pub subfrms: [mdp_color_subfrm_8195; IMG_MAX_SUBFRAMES_8195],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdp_ovl_subfrm_8195 {
    pub L0_src_size: u32,
    pub roi_size: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdp_ovl_data_8195 {
    pub L0_con: u32,
    pub src_con: u32,
    pub subfrms: [mdp_ovl_subfrm_8195; IMG_MAX_SUBFRAMES_8195],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdp_pad_subfrm_8195 {
    pub pic_size: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdp_pad_data_8195 {
    pub subfrms: [mdp_pad_subfrm_8195; IMG_MAX_SUBFRAMES_8195],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdp_tcc_subfrm_8195 {
    pub pic_size: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdp_tcc_data_8195 {
    pub subfrms: [mdp_tcc_subfrm_8195; IMG_MAX_SUBFRAMES_8195],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdp_wrot_subfrm_8195 {
    pub offset: [u32; IMG_MAX_PLANES],
    pub src: u32,
    pub clip: u32,
    pub clip_ofst: u32,
    pub main_buf: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdp_wrot_data_8195 {
    pub iova: [u32; IMG_MAX_PLANES],
    pub control: u32,
    pub stride: [u32; IMG_MAX_PLANES],
    pub mat_ctrl: u32,
    pub fifo_test: u32,
    pub filter: u32,
    pub pre_ultra: u32,
    pub framesize: u32,
    pub afbc_yuvtrans: u32,
    pub scan_10bit: u32,
    pub pending_zero: u32,
    pub bit_number: u32,
    pub pvric: u32,
    pub vpp02vpp1: u32,
    pub subfrms: [mdp_wrot_subfrm_8195; IMG_MAX_SUBFRAMES_8195],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdp_wdma_subfrm_8195 {
    pub offset: [u32; IMG_MAX_PLANES],
    pub src: u32,
    pub clip: u32,
    pub clip_ofst: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdp_wdma_data_8195 {
    pub wdma_cfg: u32,
    pub iova: [u32; IMG_MAX_PLANES],
    pub w_in_byte: u32,
    pub uv_stride: u32,
    pub subfrms: [mdp_wdma_subfrm_8195; IMG_MAX_SUBFRAMES_8195],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct isp_data_8195 {
    pub /: *mut *mut u64 dl_flags; / 1 << (enum mdp_comp_type),
    pub smxi_iova: [u32; 4],
    pub cq_idx: u32,
    pub cq_iova: u32,
    pub tpipe_iova: [u32; IMG_MAX_SUBFRAMES_8195],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct img_compparam_8195 {
    pub /: *mut *mut u32 type; / enum mdp_comp_id,
    pub /: *mut *mut u32 id; / engine alias_id,
    pub input: u32,
    pub outputs: [u32; IMG_MAX_HW_OUTPUTS],
    pub num_outputs: u32,
    pub frame: img_comp_frame_8195,
    pub subfrms: [img_comp_subfrm_8195; IMG_MAX_SUBFRAMES_8195],
    pub num_subfrms: u32,
    pub rdma: mdp_rdma_data_8195,
    pub fg: mdp_fg_data_8195,
    pub hdr: mdp_hdr_data_8195,
    pub aal: mdp_aal_data_8195,
    pub rsz: mdp_rsz_data_8195,
    pub tdshp: mdp_tdshp_data_8195,
    pub color: mdp_color_data_8195,
    pub ovl: mdp_ovl_data_8195,
    pub pad: mdp_pad_data_8195,
    pub tcc: mdp_tcc_data_8195,
    pub wrot: mdp_wrot_data_8195,
    pub wdma: mdp_wdma_data_8195,
    pub isp: isp_data_8195,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct img_config_8195 {
    pub components: [img_compparam_8195; IMG_MAX_COMPONENTS],
    pub num_components: u32,
    pub ctrls: [img_mmsys_ctrl; IMG_MAX_SUBFRAMES_8195],
    pub num_subfrms: u32,
    pub __packed: },
