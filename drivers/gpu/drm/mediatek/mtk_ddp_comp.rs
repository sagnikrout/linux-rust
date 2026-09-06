//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/mediatek/mtk_ddp_comp.h
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
// Copyright (c) 2015 MediaTek Inc.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mtk_ddp_comp_type {
    MTK_DISP_AAL,
    MTK_DISP_BLS,
    MTK_DISP_CCORR,
    MTK_DISP_COLOR,
    MTK_DISP_DITHER,
    MTK_DISP_DSC,
    MTK_DISP_GAMMA,
    MTK_DISP_MERGE,
    MTK_DISP_MUTEX,
    MTK_DISP_OD,
    MTK_DISP_OVL,
    MTK_DISP_OVL_2L,
    MTK_DISP_OVL_ADAPTOR,
    MTK_DISP_POSTMASK,
    MTK_DISP_PWM,
    MTK_DISP_RDMA,
    MTK_DISP_UFOE,
    MTK_DISP_WDMA,
    MTK_DPI,
    MTK_DP_INTF,
    MTK_DSI,
    MTK_DDP_COMP_TYPE_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_ddp_comp_funcs {
    pub dev): *mut *mut int (power_on)(struct device,
    pub dev): *mut *mut void (power_off)(struct device,
    pub dev): *mut *mut int (clk_enable)(struct device,
    pub dev): *mut *mut void (clk_disable)(struct device,
    pub cmdq_pkt): *mut unsigned int bpc, struct cmdq_pkt,
    pub dev): *mut *mut void (start)(struct device,
    pub dev): *mut *mut void (stop)(struct device,
    pub vblank_cb_data): *mut c_void,
    pub dev): *mut *mut void (unregister_vblank_cb)(struct device,
    pub dev): *mut *mut void (enable_vblank)(struct device,
    pub dev): *mut *mut void (disable_vblank)(struct device,
    pub dev): *mut *mut unsigned int (supported_rotations)(struct device,
    pub dev): *mut *mut unsigned int (layer_nr)(struct device,
    pub state): *mut mtk_plane_state,
    pub cmdq_pkt): *mut cmdq_pkt,
    pub dev): *mut *mut unsigned int (gamma_get_lut_size)(struct device,
    pub state): *mut drm_crtc_state,
    pub dev): *mut *mut void (bgclr_in_on)(struct device,
    pub dev): *mut *mut void (bgclr_in_off)(struct device,
    pub state): *mut drm_crtc_state,
    pub dev): *mut *mut *mut device  (dma_dev_get)(device,
    pub dev): *mut *mut u32 (get_blend_modes)(struct device,
    pub dev): *const *const *const u32 (get_formats)(struct device,
    pub dev): *mut *mut size_t (get_num_formats)(struct device,
    pub dev): *mut *mut bool (is_afbc_supported)(struct device,
    pub next): *mut *mut *mut *mut void (connect)(struct device dev, struct device mmsys_dev, unsigned int,
    pub next): *mut *mut *mut *mut void (disconnect)(struct device dev, struct device mmsys_dev, unsigned int,
    pub mutex): *mut *mut *mut void (add)(struct device dev, struct mtk_mutex,
    pub mutex): *mut *mut *mut void (remove)(struct device dev, struct mtk_mutex,
    pub dev): *mut *mut unsigned int (encoder_index)(struct device,
    pub mode): *const *const *const drm_mode_status (mode_valid)(struct device dev, struct drm_display_mode,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_ddp_comp {
    pub dev: *mut device,
    pub irq: c_int,
    pub id: c_uint,
    pub encoder_index: c_int,
    pub funcs: *const mtk_ddp_comp_funcs,
}

extern "C" {
    pub fn pm_runtime_resume_and_get(_arg: comp->dev) -> return;
}
//
// In order to pass IGT tests, DRM_MODE_ROTATE_0 is required when
// rotation is not supported.
//
extern "C" {
    pub fn mtk_find_possible_crtcs(drm: *mut drm_device, dev: *mut device) -> c_int;
}
extern "C" {
    pub fn mtk_ddp_comp_get_type(comp_id: c_uint) -> mtk_ddp_comp_type;
}
