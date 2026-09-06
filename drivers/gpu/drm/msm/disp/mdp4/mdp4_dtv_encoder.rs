//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/msm/disp/mdp4/mdp4_dtv_encoder.c
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
// Copyright (C) 2013 Red Hat
// Author: Rob Clark <robdclark@gmail.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdp4_dtv_encoder {
    pub base: drm_encoder,
    pub hdmi_clk: *mut clk,
    pub mdp_clk: *mut clk,
    pub pixclock: unsigned long int,
    pub enabled: bool,
    pub bsc: u32,
}

    static struct mdp4_kms *get_kms(struct drm_encoder *encoder)
    {
    struct msm_drm_private *priv = encoder.dev.dev_private;
    return to_mdp4_kms(to_mdp_kms(priv.kms));
    }
    static void mdp4_dtv_encoder_mode_set(struct drm_encoder *encoder,
    struct drm_display_mode *mode,
    struct drm_display_mode *adjusted_mode)
    {
    struct mdp4_dtv_encoder *mdp4_dtv_encoder = to_mdp4_dtv_encoder(encoder);
    struct mdp4_kms *mdp4_kms = get_kms(encoder);
    uint32_t dtv_hsync_skew, vsync_period, vsync_len, ctrl_pol;
    uint32_t display_v_start, display_v_end;
    uint32_t hsync_start_x, hsync_end_x;
    mode = adjusted_mode;
    DBG("set mode: " DRM_MODE_FMT, DRM_MODE_ARG(mode));
    mdp4_dtv_encoder.pixclock = mode.clock * 1000;
    DBG("pixclock=%lu", mdp4_dtv_encoder.pixclock);
    ctrl_pol = 0;
    if (mode.flags & DRM_MODE_FLAG_NHSYNC)
    ctrl_pol |= MDP4_DTV_CTRL_POLARITY_HSYNC_LOW;
    if (mode.flags & DRM_MODE_FLAG_NVSYNC)
    ctrl_pol |= MDP4_DTV_CTRL_POLARITY_VSYNC_LOW;
// probably need to get DATA_EN polarity from panel..
    dtv_hsync_skew = 0;  /* get this from panel? */
    hsync_start_x = (mode.htotal - mode.hsync_start);
    hsync_end_x = mode.htotal - (mode.hsync_start - mode.hdisplay) - 1;
    vsync_period = mode.vtotal * mode.htotal;
    vsync_len = (mode.vsync_end - mode.vsync_start) * mode.htotal;
    display_v_start = (mode.vtotal - mode.vsync_start) * mode.htotal + dtv_hsync_skew;
    display_v_end = vsync_period - ((mode.vsync_start - mode.vdisplay) * mode.htotal) + dtv_hsync_skew - 1;
    mdp4_write(mdp4_kms, REG_MDP4_DTV_HSYNC_CTRL,
    MDP4_DTV_HSYNC_CTRL_PULSEW(mode.hsync_end - mode.hsync_start) |
    MDP4_DTV_HSYNC_CTRL_PERIOD(mode.htotal));
    mdp4_write(mdp4_kms, REG_MDP4_DTV_VSYNC_PERIOD, vsync_period);
    mdp4_write(mdp4_kms, REG_MDP4_DTV_VSYNC_LEN, vsync_len);
    mdp4_write(mdp4_kms, REG_MDP4_DTV_DISPLAY_HCTRL,
    MDP4_DTV_DISPLAY_HCTRL_START(hsync_start_x) |
    MDP4_DTV_DISPLAY_HCTRL_END(hsync_end_x));
    mdp4_write(mdp4_kms, REG_MDP4_DTV_DISPLAY_VSTART, display_v_start);
    mdp4_write(mdp4_kms, REG_MDP4_DTV_DISPLAY_VEND, display_v_end);
    mdp4_write(mdp4_kms, REG_MDP4_DTV_BORDER_CLR, 0);
    mdp4_write(mdp4_kms, REG_MDP4_DTV_UNDERFLOW_CLR,
    MDP4_DTV_UNDERFLOW_CLR_ENABLE_RECOVERY |
    MDP4_DTV_UNDERFLOW_CLR_COLOR(0xff));
    mdp4_write(mdp4_kms, REG_MDP4_DTV_HSYNC_SKEW, dtv_hsync_skew);
    mdp4_write(mdp4_kms, REG_MDP4_DTV_CTRL_POLARITY, ctrl_pol);
    mdp4_write(mdp4_kms, REG_MDP4_DTV_ACTIVE_HCTL,
    MDP4_DTV_ACTIVE_HCTL_START(0) |
    MDP4_DTV_ACTIVE_HCTL_END(0));
    mdp4_write(mdp4_kms, REG_MDP4_DTV_ACTIVE_VSTART, 0);
    mdp4_write(mdp4_kms, REG_MDP4_DTV_ACTIVE_VEND, 0);
    }
#[no_mangle]
unsafe extern "C" fn mdp4_dtv_encoder_disable(encoder: *mut drm_encoder) {
    static void mdp4_dtv_encoder_disable(struct drm_encoder *encoder)
    {
    struct mdp4_dtv_encoder *mdp4_dtv_encoder = to_mdp4_dtv_encoder(encoder);
    struct mdp4_kms *mdp4_kms = get_kms(encoder);
    if (WARN_ON(!mdp4_dtv_encoder.enabled))
    return;
    mdp4_write(mdp4_kms, REG_MDP4_DTV_ENABLE, 0);
//
// Wait for a vsync so we know the ENABLE=0 latched before
// the (connector) source of the vsync's gets disabled,
// otherwise we end up in a funny state if we re-enable
// before the disable latches, which results that some of
// the settings changes for the new modeset (like new
// scanout buffer) don't latch properly..
//
    mdp_irq_wait(&mdp4_kms.base, MDP4_IRQ_EXTERNAL_VSYNC);
    clk_disable_unprepare(mdp4_dtv_encoder.hdmi_clk);
    clk_disable_unprepare(mdp4_dtv_encoder.mdp_clk);
    mdp4_dtv_encoder.enabled = false;
    }
#[no_mangle]
unsafe extern "C" fn mdp4_dtv_encoder_enable(encoder: *mut drm_encoder) {
    static void mdp4_dtv_encoder_enable(struct drm_encoder *encoder)
    {
    struct drm_device *dev = encoder.dev;
    struct mdp4_dtv_encoder *mdp4_dtv_encoder = to_mdp4_dtv_encoder(encoder);
    struct mdp4_kms *mdp4_kms = get_kms(encoder);
    let mut pc: c_ulong = mdp4_dtv_encoder.pixclock;
    int ret;
    if (WARN_ON(mdp4_dtv_encoder.enabled))
    return;
    mdp4_crtc_set_config(encoder.crtc,
    MDP4_DMA_CONFIG_R_BPC(BPC8) |
    MDP4_DMA_CONFIG_G_BPC(BPC8) |
    MDP4_DMA_CONFIG_B_BPC(BPC8) |
    MDP4_DMA_CONFIG_PACK(0x21));
    mdp4_crtc_set_intf(encoder.crtc, INTF_LCDC_DTV, 1);
    DBG("setting mdp_clk=%lu", pc);
    ret = clk_set_rate(mdp4_dtv_encoder.mdp_clk, pc);
    if (ret)
    DRM_DEV_ERROR(dev.dev, "failed to set mdp_clk to %lu: %d\n",
    pc, ret);
    ret = clk_prepare_enable(mdp4_dtv_encoder.mdp_clk);
    if (ret)
    DRM_DEV_ERROR(dev.dev, "failed to enabled mdp_clk: %d\n", ret);
    ret = clk_prepare_enable(mdp4_dtv_encoder.hdmi_clk);
    if (ret)
    DRM_DEV_ERROR(dev.dev, "failed to enable hdmi_clk: %d\n", ret);
    mdp4_write(mdp4_kms, REG_MDP4_DTV_ENABLE, 1);
    mdp4_dtv_encoder.enabled = true;
    }
    static const struct drm_encoder_helper_funcs mdp4_dtv_encoder_helper_funcs = {
    .mode_set = mdp4_dtv_encoder_mode_set,
    .enable = mdp4_dtv_encoder_enable,
    .disable = mdp4_dtv_encoder_disable,
    };
#[no_mangle]
pub unsafe extern "C" fn mdp4_dtv_round_pixclk(encoder: *mut drm_encoder, rate: c_ulong) -> c_long {
    long mdp4_dtv_round_pixclk(struct drm_encoder *encoder, unsigned long rate)
    {
    struct mdp4_dtv_encoder *mdp4_dtv_encoder = to_mdp4_dtv_encoder(encoder);
    return clk_round_rate(mdp4_dtv_encoder.mdp_clk, rate);
    }
// initialize encoder
    struct drm_encoder *mdp4_dtv_encoder_init(struct drm_device *dev)
    {
    struct drm_encoder *encoder;
    struct mdp4_dtv_encoder *mdp4_dtv_encoder;
    mdp4_dtv_encoder = drmm_encoder_alloc(dev, struct mdp4_dtv_encoder, base,
    core::ptr::null_mut(), DRM_MODE_ENCODER_TMDS, core::ptr::null_mut());
    if (IS_ERR(mdp4_dtv_encoder))
    return ERR_CAST(mdp4_dtv_encoder);
    encoder = &mdp4_dtv_encoder.base;
    drm_encoder_helper_add(encoder, &mdp4_dtv_encoder_helper_funcs);
    mdp4_dtv_encoder.hdmi_clk = devm_clk_get(dev.dev, "hdmi_clk");
    if (IS_ERR(mdp4_dtv_encoder.hdmi_clk)) {
    DRM_DEV_ERROR(dev.dev, "failed to get hdmi_clk\n");
    return ERR_CAST(mdp4_dtv_encoder.hdmi_clk);
    }
    mdp4_dtv_encoder.mdp_clk = devm_clk_get(dev.dev, "tv_clk");
    if (IS_ERR(mdp4_dtv_encoder.mdp_clk)) {
    DRM_DEV_ERROR(dev.dev, "failed to get tv_clk\n");
    return ERR_CAST(mdp4_dtv_encoder.mdp_clk);
    }
    return encoder;
    }
