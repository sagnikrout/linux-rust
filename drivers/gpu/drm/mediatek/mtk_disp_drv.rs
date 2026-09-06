//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/mediatek/mtk_disp_drv.h
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
// Copyright (c) 2020 MediaTek Inc.
//

extern "C" {
    pub fn mtk_aal_clk_enable(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn mtk_aal_clk_disable(dev: *mut device);
}
extern "C" {
    pub fn mtk_aal_gamma_get_lut_size(dev: *mut device) -> c_uint;
}
extern "C" {
    pub fn mtk_aal_gamma_set(dev: *mut device, state: *mut drm_crtc_state);
}
extern "C" {
    pub fn mtk_aal_start(dev: *mut device);
}
extern "C" {
    pub fn mtk_aal_stop(dev: *mut device);
}
extern "C" {
    pub fn mtk_ccorr_ctm_set(dev: *mut device, state: *mut drm_crtc_state);
}
extern "C" {
    pub fn mtk_ccorr_clk_enable(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn mtk_ccorr_clk_disable(dev: *mut device);
}
extern "C" {
    pub fn mtk_ccorr_start(dev: *mut device);
}
extern "C" {
    pub fn mtk_ccorr_stop(dev: *mut device);
}
extern "C" {
    pub fn mtk_color_bypass_shadow(dev: *mut device);
}
extern "C" {
    pub fn mtk_color_clk_enable(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn mtk_color_clk_disable(dev: *mut device);
}
extern "C" {
    pub fn mtk_color_start(dev: *mut device);
}
extern "C" {
    pub fn mtk_dpi_start(dev: *mut device);
}
extern "C" {
    pub fn mtk_dpi_stop(dev: *mut device);
}
extern "C" {
    pub fn mtk_dpi_encoder_index(dev: *mut device) -> c_uint;
}
extern "C" {
    pub fn mtk_dsi_ddp_start(dev: *mut device);
}
extern "C" {
    pub fn mtk_dsi_ddp_stop(dev: *mut device);
}
extern "C" {
    pub fn mtk_dsi_encoder_index(dev: *mut device) -> c_uint;
}
extern "C" {
    pub fn mtk_gamma_clk_enable(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn mtk_gamma_clk_disable(dev: *mut device);
}
extern "C" {
    pub fn mtk_gamma_get_lut_size(dev: *mut device) -> c_uint;
}
extern "C" {
    pub fn mtk_gamma_set(dev: *mut device, state: *mut drm_crtc_state);
}
extern "C" {
    pub fn mtk_gamma_start(dev: *mut device);
}
extern "C" {
    pub fn mtk_gamma_stop(dev: *mut device);
}
extern "C" {
    pub fn mtk_merge_clk_enable(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn mtk_merge_clk_disable(dev: *mut device);
}
extern "C" {
    pub fn mtk_merge_start(dev: *mut device);
}
extern "C" {
    pub fn mtk_merge_stop(dev: *mut device);
}
extern "C" {
    pub fn mtk_merge_start_cmdq(dev: *mut device, cmdq_pkt: *mut cmdq_pkt);
}
extern "C" {
    pub fn mtk_merge_stop_cmdq(dev: *mut device, cmdq_pkt: *mut cmdq_pkt);
}
extern "C" {
    pub fn mtk_ovl_bgclr_in_on(dev: *mut device);
}
extern "C" {
    pub fn mtk_ovl_bgclr_in_off(dev: *mut device);
}
extern "C" {
    pub fn mtk_ovl_bypass_shadow(dev: *mut device);
}
extern "C" {
    pub fn mtk_ovl_clk_enable(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn mtk_ovl_clk_disable(dev: *mut device);
}
extern "C" {
    pub fn mtk_ovl_layer_nr(dev: *mut device) -> c_uint;
}
extern "C" {
    pub fn mtk_ovl_start(dev: *mut device);
}
extern "C" {
    pub fn mtk_ovl_stop(dev: *mut device);
}
extern "C" {
    pub fn mtk_ovl_supported_rotations(dev: *mut device) -> c_uint;
}
extern "C" {
    pub fn mtk_ovl_unregister_vblank_cb(dev: *mut device);
}
extern "C" {
    pub fn mtk_ovl_enable_vblank(dev: *mut device);
}
extern "C" {
    pub fn mtk_ovl_disable_vblank(dev: *mut device);
}
extern "C" {
    pub fn mtk_ovl_get_blend_modes(dev: *mut device) -> u32;
}
extern "C" {
    pub fn mtk_ovl_get_num_formats(dev: *mut device) -> usize;
}
extern "C" {
    pub fn mtk_ovl_is_afbc_supported(dev: *mut device) -> bool;
}
extern "C" {
    pub fn mtk_ovl_adaptor_add_comp(dev: *mut device, mutex: *mut mtk_mutex);
}
extern "C" {
    pub fn mtk_ovl_adaptor_remove_comp(dev: *mut device, mutex: *mut mtk_mutex);
}
extern "C" {
    pub fn mtk_ovl_adaptor_is_comp_present(node: *mut device_node) -> bool;
}
extern "C" {
    pub fn mtk_ovl_adaptor_power_on(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn mtk_ovl_adaptor_power_off(dev: *mut device);
}
extern "C" {
    pub fn mtk_ovl_adaptor_clk_enable(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn mtk_ovl_adaptor_clk_disable(dev: *mut device);
}
extern "C" {
    pub fn mtk_ovl_adaptor_unregister_vblank_cb(dev: *mut device);
}
extern "C" {
    pub fn mtk_ovl_adaptor_enable_vblank(dev: *mut device);
}
extern "C" {
    pub fn mtk_ovl_adaptor_disable_vblank(dev: *mut device);
}
extern "C" {
    pub fn mtk_ovl_adaptor_start(dev: *mut device);
}
extern "C" {
    pub fn mtk_ovl_adaptor_stop(dev: *mut device);
}
extern "C" {
    pub fn mtk_ovl_adaptor_layer_nr(dev: *mut device) -> c_uint;
}
extern "C" {
    pub fn mtk_ovl_adaptor_get_blend_modes(dev: *mut device) -> u32;
}
extern "C" {
    pub fn mtk_ovl_adaptor_get_num_formats(dev: *mut device) -> usize;
}
extern "C" {
    pub fn mtk_rdma_bypass_shadow(dev: *mut device);
}
extern "C" {
    pub fn mtk_rdma_clk_enable(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn mtk_rdma_clk_disable(dev: *mut device);
}
extern "C" {
    pub fn mtk_rdma_layer_nr(dev: *mut device) -> c_uint;
}
extern "C" {
    pub fn mtk_rdma_start(dev: *mut device);
}
extern "C" {
    pub fn mtk_rdma_stop(dev: *mut device);
}
extern "C" {
    pub fn mtk_rdma_unregister_vblank_cb(dev: *mut device);
}
extern "C" {
    pub fn mtk_rdma_enable_vblank(dev: *mut device);
}
extern "C" {
    pub fn mtk_rdma_disable_vblank(dev: *mut device);
}
extern "C" {
    pub fn mtk_rdma_get_num_formats(dev: *mut device) -> usize;
}
extern "C" {
    pub fn mtk_mdp_rdma_power_on(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn mtk_mdp_rdma_power_off(dev: *mut device);
}
extern "C" {
    pub fn mtk_mdp_rdma_clk_enable(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn mtk_mdp_rdma_clk_disable(dev: *mut device);
}
extern "C" {
    pub fn mtk_mdp_rdma_start(dev: *mut device, cmdq_pkt: *mut cmdq_pkt);
}
extern "C" {
    pub fn mtk_mdp_rdma_stop(dev: *mut device, cmdq_pkt: *mut cmdq_pkt);
}
extern "C" {
    pub fn mtk_mdp_rdma_get_num_formats(dev: *mut device) -> usize;
}
extern "C" {
    pub fn mtk_padding_clk_enable(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn mtk_padding_clk_disable(dev: *mut device);
}
extern "C" {
    pub fn mtk_padding_start(dev: *mut device);
}
extern "C" {
    pub fn mtk_padding_stop(dev: *mut device);
}
