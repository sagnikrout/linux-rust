//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/mediatek/mdp3/mdp_sm_mt8183.h
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
// 2d995ddb5c3b0cf26e96d6a823481886
//
pub const IMG_MAX_SUBFRAMES_8183: c_int = 14;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct img_comp_frame_8183 {
    pub output_disable:1: u32,
    pub bypass:1: u32,
    pub in_width: u16,
    pub in_height: u16,
    pub out_width: u16,
    pub out_height: u16,
    pub crop: img_crop,
    pub in_total_width: u16,
    pub out_total_width: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct img_comp_subfrm_8183 {
    pub tile_disable:1: u32,
    pub in: img_region,
    pub out: img_region,
    pub luma: img_offset,
    pub chroma: img_offset,
    pub /: *mut *mut s16 out_vertical; / Output vertical index,
    pub /: *mut *mut s16 out_horizontal; / Output horizontal index,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdp_rdma_subfrm_8183 {
    pub offset: [u32; IMG_MAX_PLANES],
    pub offset_0_p: u32,
    pub src: u32,
    pub clip: u32,
    pub clip_ofst: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdp_rdma_data_8183 {
    pub src_ctrl: u32,
    pub control: u32,
    pub iova: [u32; IMG_MAX_PLANES],
    pub iova_end: [u32; IMG_MAX_PLANES],
    pub mf_bkgd: u32,
    pub mf_bkgd_in_pxl: u32,
    pub sf_bkgd: u32,
    pub ufo_dec_y: u32,
    pub ufo_dec_c: u32,
    pub transform: u32,
    pub subfrms: [mdp_rdma_subfrm_8183; IMG_MAX_SUBFRAMES_8183],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdp_rsz_subfrm_8183 {
    pub control2: u32,
    pub src: u32,
    pub clip: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdp_rsz_data_8183 {
    pub coeff_step_x: u32,
    pub coeff_step_y: u32,
    pub control1: u32,
    pub control2: u32,
    pub subfrms: [mdp_rsz_subfrm_8183; IMG_MAX_SUBFRAMES_8183],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdp_wrot_subfrm_8183 {
    pub offset: [u32; IMG_MAX_PLANES],
    pub src: u32,
    pub clip: u32,
    pub clip_ofst: u32,
    pub main_buf: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdp_wrot_data_8183 {
    pub iova: [u32; IMG_MAX_PLANES],
    pub control: u32,
    pub stride: [u32; IMG_MAX_PLANES],
    pub mat_ctrl: u32,
    pub fifo_test: u32,
    pub filter: u32,
    pub subfrms: [mdp_wrot_subfrm_8183; IMG_MAX_SUBFRAMES_8183],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdp_wdma_subfrm_8183 {
    pub offset: [u32; IMG_MAX_PLANES],
    pub src: u32,
    pub clip: u32,
    pub clip_ofst: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdp_wdma_data_8183 {
    pub wdma_cfg: u32,
    pub iova: [u32; IMG_MAX_PLANES],
    pub w_in_byte: u32,
    pub uv_stride: u32,
    pub subfrms: [mdp_wdma_subfrm_8183; IMG_MAX_SUBFRAMES_8183],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct isp_data_8183 {
    pub /: *mut *mut u64 dl_flags; / 1 << (enum mdp_comp_type),
    pub smxi_iova: [u32; 4],
    pub cq_idx: u32,
    pub cq_iova: u32,
    pub tpipe_iova: [u32; IMG_MAX_SUBFRAMES_8183],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct img_compparam_8183 {
    pub /: *mut *mut u16 type; / enum mdp_comp_id,
    pub /: *mut *mut u16 id; / engine alias_id,
    pub input: u32,
    pub outputs: [u32; IMG_MAX_HW_OUTPUTS],
    pub num_outputs: u32,
    pub frame: img_comp_frame_8183,
    pub subfrms: [img_comp_subfrm_8183; IMG_MAX_SUBFRAMES_8183],
    pub num_subfrms: u32,
    pub rdma: mdp_rdma_data_8183,
    pub rsz: mdp_rsz_data_8183,
    pub wrot: mdp_wrot_data_8183,
    pub wdma: mdp_wdma_data_8183,
    pub isp: isp_data_8183,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct img_config_8183 {
    pub components: [img_compparam_8183; IMG_MAX_COMPONENTS],
    pub num_components: u32,
    pub ctrls: [img_mmsys_ctrl; IMG_MAX_SUBFRAMES_8183],
    pub num_subfrms: u32,
    pub __packed: },
