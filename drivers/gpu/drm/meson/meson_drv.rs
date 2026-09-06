//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/meson/meson_drv.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright (C) 2016 BayLibre, SAS
// Author: Neil Armstrong <narmstrong@baylibre.com>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vpu_compatible {
    VPU_COMPATIBLE_GXBB = 0,
    VPU_COMPATIBLE_GXL  = 1,
    VPU_COMPATIBLE_GXM  = 2,
    VPU_COMPATIBLE_G12A = 3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct meson_drm_match_data {
    pub compat: vpu_compatible,
    pub afbcd_ops: *mut meson_afbcd_ops,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct meson_drm_soc_limits {
    pub max_hdmi_phy_freq: c_ulonglong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct meson_drm {
    pub dev: *mut device,
    pub compat: vpu_compatible,
    pub io_base: *mut void __iomem,
    pub hhi: *mut regmap,
    pub vsync_irq: c_int,
    pub canvas: *mut meson_canvas,
    pub canvas_id_osd1: u8,
    pub canvas_id_vd1_0: u8,
    pub canvas_id_vd1_1: u8,
    pub canvas_id_vd1_2: u8,
    pub drm: *mut drm_device,
    pub crtc: *mut drm_crtc,
    pub primary_plane: *mut drm_plane,
    pub overlay_plane: *mut drm_plane,
    pub encoders: [*mut c_void; MESON_ENC_LAST],
    pub limits: *const meson_drm_soc_limits,
// Components Data
    pub osd1_enabled: bool,
    pub osd1_interlace: bool,
    pub osd1_commit: bool,
    pub osd1_afbcd: bool,
    pub osd1_ctrl_stat: u32,
    pub osd1_ctrl_stat2: u32,
    pub osd1_blk0_cfg: [u32; 5],
    pub osd1_blk1_cfg4: u32,
    pub osd1_blk2_cfg4: u32,
    pub osd1_addr: u32,
    pub osd1_stride: u32,
    pub osd1_height: u32,
    pub osd1_width: u32,
    pub osd_sc_ctrl0: u32,
    pub osd_sc_i_wh_m1: u32,
    pub osd_sc_o_h_start_end: u32,
    pub osd_sc_o_v_start_end: u32,
    pub osd_sc_v_ini_phase: u32,
    pub osd_sc_v_phase_step: u32,
    pub osd_sc_h_ini_phase: u32,
    pub osd_sc_h_phase_step: u32,
    pub osd_sc_h_ctrl0: u32,
    pub osd_sc_v_ctrl0: u32,
    pub osd_blend_din0_scope_h: u32,
    pub osd_blend_din0_scope_v: u32,
    pub osb_blend0_size: u32,
    pub osb_blend1_size: u32,
    pub vd1_enabled: bool,
    pub vd1_commit: bool,
    pub vd1_afbc: bool,
    pub vd1_planes: c_uint,
    pub vd1_if0_gen_reg: u32,
    pub vd1_if0_luma_x0: u32,
    pub vd1_if0_luma_y0: u32,
    pub vd1_if0_chroma_x0: u32,
    pub vd1_if0_chroma_y0: u32,
    pub vd1_if0_repeat_loop: u32,
    pub vd1_if0_luma0_rpt_pat: u32,
    pub vd1_if0_chroma0_rpt_pat: u32,
    pub vd1_range_map_y: u32,
    pub vd1_range_map_cb: u32,
    pub vd1_range_map_cr: u32,
    pub viu_vd1_fmt_w: u32,
    pub vd1_if0_canvas0: u32,
    pub vd1_if0_gen_reg2: u32,
    pub viu_vd1_fmt_ctrl: u32,
    pub vd1_addr0: u32,
    pub vd1_addr1: u32,
    pub vd1_addr2: u32,
    pub vd1_stride0: u32,
    pub vd1_stride1: u32,
    pub vd1_stride2: u32,
    pub vd1_height0: u32,
    pub vd1_height1: u32,
    pub vd1_height2: u32,
    pub vd1_afbc_mode: u32,
    pub vd1_afbc_en: u32,
    pub vd1_afbc_head_addr: u32,
    pub vd1_afbc_body_addr: u32,
    pub vd1_afbc_conv_ctrl: u32,
    pub vd1_afbc_dec_def_color: u32,
    pub vd1_afbc_vd_cfmt_ctrl: u32,
    pub vd1_afbc_vd_cfmt_w: u32,
    pub vd1_afbc_vd_cfmt_h: u32,
    pub vd1_afbc_mif_hor_scope: u32,
    pub vd1_afbc_mif_ver_scope: u32,
    pub vd1_afbc_size_out: u32,
    pub vd1_afbc_pixel_hor_scope: u32,
    pub vd1_afbc_pixel_ver_scope: u32,
    pub vd1_afbc_size_in: u32,
    pub vpp_pic_in_height: u32,
    pub vpp_postblend_vd1_h_start_end: u32,
    pub vpp_postblend_vd1_v_start_end: u32,
    pub vpp_hsc_region12_startp: u32,
    pub vpp_hsc_region34_startp: u32,
    pub vpp_hsc_region4_endp: u32,
    pub vpp_hsc_start_phase_step: u32,
    pub vpp_hsc_region1_phase_slope: u32,
    pub vpp_hsc_region3_phase_slope: u32,
    pub vpp_line_in_length: u32,
    pub vpp_preblend_h_size: u32,
    pub vpp_vsc_region12_startp: u32,
    pub vpp_vsc_region34_startp: u32,
    pub vpp_vsc_region4_endp: u32,
    pub vpp_vsc_start_phase_step: u32,
    pub vpp_vsc_ini_phase: u32,
    pub vpp_vsc_phase_ctrl: u32,
    pub vpp_hsc_phase_ctrl: u32,
    pub vpp_blend_vd2_h_start_end: u32,
    pub vpp_blend_vd2_v_start_end: u32,
    pub viu: },
    pub current_mode: c_uint,
    pub hdmi_repeat: bool,
    pub venc_repeat: bool,
    pub hdmi_use_enci: bool,
    pub venc: },
    pub addr_dma: dma_addr_t,
    pub addr: *mut u32,
    pub offset: c_uint,
    pub rdma: },
    pub ops: *mut meson_afbcd_ops,
    pub modifier: u64,
    pub format: u32,
    pub afbcd: },
}
