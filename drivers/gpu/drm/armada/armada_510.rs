//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/armada/armada_510.c
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
// Copyright (C) 2012 Russell King
//
// Armada 510 (aka Dove) variant support
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct armada510_variant_data {
    pub clks: [*mut clk; 4],
    pub sel_clk: *mut clk,
}

#[no_mangle]
unsafe extern "C" fn armada510_crtc_init(dcrtc: *mut armada_crtc, dev: *mut device) -> c_int {
    static int armada510_crtc_init(struct armada_crtc *dcrtc, struct device *dev)
    {
    struct armada510_variant_data *v;
    struct clk *clk;
    int idx;
    v = devm_kzalloc(dev, sizeof(*v), GFP_KERNEL);
    if (!v)
    return -ENOMEM;
    dcrtc.variant_data = v;
    if (dev.of_node) {
    struct property *prop;
    const char *s;
    of_property_for_each_string(dev.of_node, "clock-names", prop,
    s) {
    if (!strcmp(s, "ext_ref_clk0"))
    idx = 0;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcmp(s, _arg: "ext_ref_clk1")) -> else {
    else if (!strcmp(s, "ext_ref_clk1"))
    idx = 1;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcmp(s, _arg: "plldivider")) -> else {
    else if (!strcmp(s, "plldivider"))
    idx = 2;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcmp(s, _arg: "axibus")) -> else {
    else if (!strcmp(s, "axibus"))
    idx = 3;
    else
    continue;
    clk = devm_clk_get(dev, s);
    if (IS_ERR(clk))
    return PTR_ERR(clk) == -ENOENT ? -EPROBE_DEFER :
    PTR_ERR(clk);
    v.clks[idx] = clk;
    }
    } else {
    clk = devm_clk_get(dev, "ext_ref_clk1");
    if (IS_ERR(clk))
    return PTR_ERR(clk) == -ENOENT ? -EPROBE_DEFER :
    PTR_ERR(clk);
    v.clks[1] = clk;
    }
//
// Lower the watermark so to eliminate jitter at higher bandwidths.
// Disable SRAM read wait state to avoid system hang with external
// clock.
//
    armada_updatel(CFG_DMA_WM(0x20), CFG_SRAM_WAIT | CFG_DMA_WM_MASK,
    dcrtc.base + LCD_CFG_RDREG4F);
// Initialise SPU register
    writel_relaxed(ADV_HWC32ENABLE | ADV_HWC32ARGB | ADV_HWC32BLEND,
    dcrtc.base + LCD_SPU_ADV_REG);
    return 0;
    }
    static const u32 armada510_clk_sels[] = {
    SCLK_510_EXTCLK0,
    SCLK_510_EXTCLK1,
    SCLK_510_PLL,
    SCLK_510_AXI,
    };
    static const struct armada_clocking_params armada510_clocking = {
// HDMI requires -0.6%..+0.5%
    .permillage_min = 994,
    .permillage_max = 1005,
    .settable = BIT(0) | BIT(1),
    .div_max = SCLK_510_INT_DIV_MASK,
    };
//
// Armada510 specific SCLK register selection.
// This gets called with sclk = NULL to test whether the mode is
// supportable, and again with sclk != NULL to set the clocks up for
// that.  The former can return an error, but the latter is expected
// not to.
//
    static int armada510_crtc_compute_clock(struct armada_crtc *dcrtc,
    const struct drm_display_mode *mode, uint32_t *sclk)
    {
    struct armada510_variant_data *v = dcrtc.variant_data;
    let mut desired_khz: c_ulong = mode.crtc_clock;
    struct armada_clk_result res;
    int ret, idx;
    idx = armada_crtc_select_clock(dcrtc, &res, &armada510_clocking,
    v.clks, ARRAY_SIZE(v.clks),
    desired_khz);
    if (idx < 0)
    return idx;
    ret = clk_prepare_enable(res.clk);
    if (ret)
    return ret;
    if (sclk) {
    clk_set_rate(res.clk, res.desired_clk_hz);
// sclk = res.div | armada510_clk_sels[idx];
// We are now using this clock
    v.sel_clk = res.clk;
    swap(dcrtc.clk, res.clk);
    }
    clk_disable_unprepare(res.clk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn armada510_crtc_disable(dcrtc: *mut armada_crtc) {
    static void armada510_crtc_disable(struct armada_crtc *dcrtc)
    {
    if (dcrtc.clk) {
    clk_disable_unprepare(dcrtc.clk);
    dcrtc.clk = core::ptr::null_mut();
    }
    }
    static void armada510_crtc_enable(struct armada_crtc *dcrtc,
    const struct drm_display_mode *mode)
    {
    struct armada510_variant_data *v = dcrtc.variant_data;
    if (!dcrtc.clk && v.sel_clk) {
    if (!WARN_ON(clk_prepare_enable(v.sel_clk)))
    dcrtc.clk = v.sel_clk;
    }
    }
    const struct armada_variant armada510_ops = {
    .has_spu_adv_reg = true,
    .init = armada510_crtc_init,
    .compute_clock = armada510_crtc_compute_clock,
    .disable = armada510_crtc_disable,
    .enable = armada510_crtc_enable,
    };
