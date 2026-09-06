//! Automatically rewritten from C to Rust
//! Source: drivers/video/fbdev/omap2/omapfb/dss/venc.c
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
// linux/drivers/video/omap2/dss/venc.c
//
// Copyright (C) 2009 Nokia Corporation
// Author: Tomi Valkeinen <tomi.valkeinen@nokia.com>
//
// VENC settings from TI's DSS driver
//

// Venc registers
pub const VENC_REV_ID: c_uint = 0x00;
pub const VENC_STATUS: c_uint = 0x04;
pub const VENC_F_CONTROL: c_uint = 0x08;
pub const VENC_VIDOUT_CTRL: c_uint = 0x10;
pub const VENC_SYNC_CTRL: c_uint = 0x14;
pub const VENC_LLEN: c_uint = 0x1C;
pub const VENC_FLENS: c_uint = 0x20;
pub const VENC_HFLTR_CTRL: c_uint = 0x24;
pub const VENC_CC_CARR_WSS_CARR: c_uint = 0x28;
pub const VENC_C_PHASE: c_uint = 0x2C;
pub const VENC_GAIN_U: c_uint = 0x30;
pub const VENC_GAIN_V: c_uint = 0x34;
pub const VENC_GAIN_Y: c_uint = 0x38;
pub const VENC_BLACK_LEVEL: c_uint = 0x3C;
pub const VENC_BLANK_LEVEL: c_uint = 0x40;
pub const VENC_X_COLOR: c_uint = 0x44;
pub const VENC_M_CONTROL: c_uint = 0x48;
pub const VENC_BSTAMP_WSS_DATA: c_uint = 0x4C;
pub const VENC_S_CARR: c_uint = 0x50;
pub const VENC_LINE21: c_uint = 0x54;
pub const VENC_LN_SEL: c_uint = 0x58;
pub const VENC_L21__WC_CTL: c_uint = 0x5C;
pub const VENC_HTRIGGER_VTRIGGER: c_uint = 0x60;
pub const VENC_SAVID__EAVID: c_uint = 0x64;
pub const VENC_FLEN__FAL: c_uint = 0x68;
pub const VENC_LAL__PHASE_RESET: c_uint = 0x6C;
pub const VENC_HS_INT_START_STOP_X: c_uint = 0x70;
pub const VENC_HS_EXT_START_STOP_X: c_uint = 0x74;
pub const VENC_VS_INT_START_X: c_uint = 0x78;
pub const VENC_VS_INT_STOP_X__VS_INT_START_Y: c_uint = 0x7C;
pub const VENC_VS_INT_STOP_Y__VS_EXT_START_X: c_uint = 0x80;
pub const VENC_VS_EXT_STOP_X__VS_EXT_START_Y: c_uint = 0x84;
pub const VENC_VS_EXT_STOP_Y: c_uint = 0x88;
pub const VENC_AVID_START_STOP_X: c_uint = 0x90;
pub const VENC_AVID_START_STOP_Y: c_uint = 0x94;
pub const VENC_FID_INT_START_X__FID_INT_START_Y: c_uint = 0xA0;
pub const VENC_FID_INT_OFFSET_Y__FID_EXT_START_X: c_uint = 0xA4;
pub const VENC_FID_EXT_START_Y__FID_EXT_OFFSET_Y: c_uint = 0xA8;
pub const VENC_TVDETGP_INT_START_STOP_X: c_uint = 0xB0;
pub const VENC_TVDETGP_INT_START_STOP_Y: c_uint = 0xB4;
pub const VENC_GEN_CTRL: c_uint = 0xB8;
pub const VENC_OUTPUT_CONTROL: c_uint = 0xC4;
pub const VENC_OUTPUT_TEST: c_uint = 0xC8;
pub const VENC_DAC_B__DAC_C: c_uint = 0xC8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct venc_config {
    pub f_control: u32,
    pub vidout_ctrl: u32,
    pub sync_ctrl: u32,
    pub llen: u32,
    pub flens: u32,
    pub hfltr_ctrl: u32,
    pub cc_carr_wss_carr: u32,
    pub c_phase: u32,
    pub gain_u: u32,
    pub gain_v: u32,
    pub gain_y: u32,
    pub black_level: u32,
    pub blank_level: u32,
    pub x_color: u32,
    pub m_control: u32,
    pub bstamp_wss_data: u32,
    pub s_carr: u32,
    pub line21: u32,
    pub ln_sel: u32,
    pub l21__wc_ctl: u32,
    pub htrigger_vtrigger: u32,
    pub savid__eavid: u32,
    pub flen__fal: u32,
    pub lal__phase_reset: u32,
    pub hs_int_start_stop_x: u32,
    pub hs_ext_start_stop_x: u32,
    pub vs_int_start_x: u32,
    pub vs_int_stop_x__vs_int_start_y: u32,
    pub vs_int_stop_y__vs_ext_start_x: u32,
    pub vs_ext_stop_x__vs_ext_start_y: u32,
    pub vs_ext_stop_y: u32,
    pub avid_start_stop_x: u32,
    pub avid_start_stop_y: u32,
    pub fid_int_start_x__fid_int_start_y: u32,
    pub fid_int_offset_y__fid_ext_start_x: u32,
    pub fid_ext_start_y__fid_ext_offset_y: u32,
    pub tvdetgp_int_start_stop_x: u32,
    pub tvdetgp_int_start_stop_y: u32,
    pub gen_ctrl: u32,
}

// from TRM
    static const struct venc_config venc_config_pal_trm = {
    .f_control				= 0,
    .vidout_ctrl				= 1,
    .sync_ctrl				= 0x40,
    .llen					= 0x35F, /* 863 */
    .flens					= 0x270, /* 624 */
    .hfltr_ctrl				= 0,
    .cc_carr_wss_carr			= 0x2F7225ED,
    .c_phase				= 0,
    .gain_u					= 0x111,
    .gain_v					= 0x181,
    .gain_y					= 0x140,
    .black_level				= 0x3B,
    .blank_level				= 0x3B,
    .x_color				= 0x7,
    .m_control				= 0x2,
    .bstamp_wss_data			= 0x3F,
    .s_carr					= 0x2A098ACB,
    .line21					= 0,
    .ln_sel					= 0x01290015,
    .l21__wc_ctl				= 0x0000F603,
    .htrigger_vtrigger			= 0,
    .savid__eavid				= 0x06A70108,
    .flen__fal				= 0x00180270,
    .lal__phase_reset			= 0x00040135,
    .hs_int_start_stop_x			= 0x00880358,
    .hs_ext_start_stop_x			= 0x000F035F,
    .vs_int_start_x				= 0x01A70000,
    .vs_int_stop_x__vs_int_start_y		= 0x000001A7,
    .vs_int_stop_y__vs_ext_start_x		= 0x01AF0000,
    .vs_ext_stop_x__vs_ext_start_y		= 0x000101AF,
    .vs_ext_stop_y				= 0x00000025,
    .avid_start_stop_x			= 0x03530083,
    .avid_start_stop_y			= 0x026C002E,
    .fid_int_start_x__fid_int_start_y	= 0x0001008A,
    .fid_int_offset_y__fid_ext_start_x	= 0x002E0138,
    .fid_ext_start_y__fid_ext_offset_y	= 0x01380001,
    .tvdetgp_int_start_stop_x		= 0x00140001,
    .tvdetgp_int_start_stop_y		= 0x00010001,
    .gen_ctrl				= 0x00FF0000,
    };
// from TRM
    static const struct venc_config venc_config_ntsc_trm = {
    .f_control				= 0,
    .vidout_ctrl				= 1,
    .sync_ctrl				= 0x8040,
    .llen					= 0x359,
    .flens					= 0x20C,
    .hfltr_ctrl				= 0,
    .cc_carr_wss_carr			= 0x043F2631,
    .c_phase				= 0,
    .gain_u					= 0x102,
    .gain_v					= 0x16C,
    .gain_y					= 0x12F,
    .black_level				= 0x43,
    .blank_level				= 0x38,
    .x_color				= 0x7,
    .m_control				= 0x1,
    .bstamp_wss_data			= 0x38,
    .s_carr					= 0x21F07C1F,
    .line21					= 0,
    .ln_sel					= 0x01310011,
    .l21__wc_ctl				= 0x0000F003,
    .htrigger_vtrigger			= 0,
    .savid__eavid				= 0x069300F4,
    .flen__fal				= 0x0016020C,
    .lal__phase_reset			= 0x00060107,
    .hs_int_start_stop_x			= 0x008E0350,
    .hs_ext_start_stop_x			= 0x000F0359,
    .vs_int_start_x				= 0x01A00000,
    .vs_int_stop_x__vs_int_start_y		= 0x020701A0,
    .vs_int_stop_y__vs_ext_start_x		= 0x01AC0024,
    .vs_ext_stop_x__vs_ext_start_y		= 0x020D01AC,
    .vs_ext_stop_y				= 0x00000006,
    .avid_start_stop_x			= 0x03480078,
    .avid_start_stop_y			= 0x02060024,
    .fid_int_start_x__fid_int_start_y	= 0x0001008A,
    .fid_int_offset_y__fid_ext_start_x	= 0x01AC0106,
    .fid_ext_start_y__fid_ext_offset_y	= 0x01060006,
    .tvdetgp_int_start_stop_x		= 0x00140001,
    .tvdetgp_int_start_stop_y		= 0x00010001,
    .gen_ctrl				= 0x00F90000,
    };
    const struct omap_video_timings omap_dss_pal_timings = {
    .x_res		= 720,
    .y_res		= 574,
    .pixelclock	= 13500000,
    .hsw		= 64,
    .hfp		= 12,
    .hbp		= 68,
    .vsw		= 5,
    .vfp		= 5,
    .vbp		= 41,
    .interlace	= true,
    };
    EXPORT_SYMBOL(omap_dss_pal_timings);
    const struct omap_video_timings omap_dss_ntsc_timings = {
    .x_res		= 720,
    .y_res		= 482,
    .pixelclock	= 13500000,
    .hsw		= 64,
    .hfp		= 16,
    .hbp		= 58,
    .vsw		= 6,
    .vfp		= 6,
    .vbp		= 31,
    .interlace	= true,
    };
    EXPORT_SYMBOL(omap_dss_ntsc_timings);
    static struct {
    struct platform_device *pdev;
    void __iomem *base;
    struct mutex venc_lock;
    u32 wss_data;
    struct regulator *vdda_dac_reg;
    struct clk	*tv_dac_clk;
    struct omap_video_timings timings;
    enum omap_dss_venc_type type;
    bool invert_polarity;
    struct omap_dss_device output;
    } venc;
#[no_mangle]
pub unsafe extern "C" fn venc_write_reg(idx: c_int, val: u32) {
    static inline void venc_write_reg(int idx, u32 val)
    {
    __raw_writel(val, venc.base + idx);
    }
#[no_mangle]
pub unsafe extern "C" fn venc_read_reg(idx: c_int) -> u32 {
    static inline u32 venc_read_reg(int idx)
    {
    let mut l: u32 = __raw_readl(venc.base + idx);
    return l;
    }
#[no_mangle]
unsafe extern "C" fn venc_write_config(config: *const venc_config) {
    static void venc_write_config(const struct venc_config *config)
    {
    DSSDBG("write venc conf\n");
    venc_write_reg(VENC_LLEN, config.llen);
    venc_write_reg(VENC_FLENS, config.flens);
    venc_write_reg(VENC_CC_CARR_WSS_CARR, config.cc_carr_wss_carr);
    venc_write_reg(VENC_C_PHASE, config.c_phase);
    venc_write_reg(VENC_GAIN_U, config.gain_u);
    venc_write_reg(VENC_GAIN_V, config.gain_v);
    venc_write_reg(VENC_GAIN_Y, config.gain_y);
    venc_write_reg(VENC_BLACK_LEVEL, config.black_level);
    venc_write_reg(VENC_BLANK_LEVEL, config.blank_level);
    venc_write_reg(VENC_M_CONTROL, config.m_control);
    venc_write_reg(VENC_BSTAMP_WSS_DATA, config.bstamp_wss_data |
    venc.wss_data);
    venc_write_reg(VENC_S_CARR, config.s_carr);
    venc_write_reg(VENC_L21__WC_CTL, config.l21__wc_ctl);
    venc_write_reg(VENC_SAVID__EAVID, config.savid__eavid);
    venc_write_reg(VENC_FLEN__FAL, config.flen__fal);
    venc_write_reg(VENC_LAL__PHASE_RESET, config.lal__phase_reset);
    venc_write_reg(VENC_HS_INT_START_STOP_X, config.hs_int_start_stop_x);
    venc_write_reg(VENC_HS_EXT_START_STOP_X, config.hs_ext_start_stop_x);
    venc_write_reg(VENC_VS_INT_START_X, config.vs_int_start_x);
    venc_write_reg(VENC_VS_INT_STOP_X__VS_INT_START_Y,
    config.vs_int_stop_x__vs_int_start_y);
    venc_write_reg(VENC_VS_INT_STOP_Y__VS_EXT_START_X,
    config.vs_int_stop_y__vs_ext_start_x);
    venc_write_reg(VENC_VS_EXT_STOP_X__VS_EXT_START_Y,
    config.vs_ext_stop_x__vs_ext_start_y);
    venc_write_reg(VENC_VS_EXT_STOP_Y, config.vs_ext_stop_y);
    venc_write_reg(VENC_AVID_START_STOP_X, config.avid_start_stop_x);
    venc_write_reg(VENC_AVID_START_STOP_Y, config.avid_start_stop_y);
    venc_write_reg(VENC_FID_INT_START_X__FID_INT_START_Y,
    config.fid_int_start_x__fid_int_start_y);
    venc_write_reg(VENC_FID_INT_OFFSET_Y__FID_EXT_START_X,
    config.fid_int_offset_y__fid_ext_start_x);
    venc_write_reg(VENC_FID_EXT_START_Y__FID_EXT_OFFSET_Y,
    config.fid_ext_start_y__fid_ext_offset_y);
    venc_write_reg(VENC_DAC_B__DAC_C,  venc_read_reg(VENC_DAC_B__DAC_C));
    venc_write_reg(VENC_VIDOUT_CTRL, config.vidout_ctrl);
    venc_write_reg(VENC_HFLTR_CTRL, config.hfltr_ctrl);
    venc_write_reg(VENC_X_COLOR, config.x_color);
    venc_write_reg(VENC_LINE21, config.line21);
    venc_write_reg(VENC_LN_SEL, config.ln_sel);
    venc_write_reg(VENC_HTRIGGER_VTRIGGER, config.htrigger_vtrigger);
    venc_write_reg(VENC_TVDETGP_INT_START_STOP_X,
    config.tvdetgp_int_start_stop_x);
    venc_write_reg(VENC_TVDETGP_INT_START_STOP_Y,
    config.tvdetgp_int_start_stop_y);
    venc_write_reg(VENC_GEN_CTRL, config.gen_ctrl);
    venc_write_reg(VENC_F_CONTROL, config.f_control);
    venc_write_reg(VENC_SYNC_CTRL, config.sync_ctrl);
    }
#[no_mangle]
unsafe extern "C" fn venc_reset() {
    static void venc_reset(void)
    {
    let mut t: c_int = 1000;
    venc_write_reg(VENC_F_CONTROL, 1<<8);
    while (venc_read_reg(VENC_F_CONTROL) & (1<<8)) {
    if (--t == 0) {
    DSSERR("Failed to reset venc\n");
    return;
    }
    }

// the magical sleep that makes things work
// XXX more info? What bug this circumvents?
    msleep(20);

    }
#[no_mangle]
unsafe extern "C" fn venc_runtime_get() -> c_int {
    static int venc_runtime_get(void)
    {
    int r;
    DSSDBG("venc_runtime_get\n");
    r = pm_runtime_resume_and_get(&venc.pdev.dev);
    if (WARN_ON(r < 0))
    return r;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn venc_runtime_put() {
    static void venc_runtime_put(void)
    {
    int r;
    DSSDBG("venc_runtime_put\n");
    r = pm_runtime_put_sync(&venc.pdev.dev);
    WARN_ON(r < 0 && r != -ENOSYS);
    }
    static const struct venc_config *venc_timings_to_config(
    struct omap_video_timings *timings)
    {
    if (memcmp(&omap_dss_pal_timings, timings, sizeof(*timings)) == 0)
    return &venc_config_pal_trm;
    if (memcmp(&omap_dss_ntsc_timings, timings, sizeof(*timings)) == 0)
    return &venc_config_ntsc_trm;
    BUG();
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn venc_power_on(dssdev: *mut omap_dss_device) -> c_int {
    static int venc_power_on(struct omap_dss_device *dssdev)
    {
    struct omap_overlay_manager *mgr = venc.output.manager;
    u32 l;
    int r;
    r = venc_runtime_get();
    if (r)
    goto err0;
    venc_reset();
    venc_write_config(venc_timings_to_config(&venc.timings));
    dss_set_venc_output(venc.type);
    dss_set_dac_pwrdn_bgz(1);
    l = 0;
    if (venc.type == OMAP_DSS_VENC_TYPE_COMPOSITE)
    l |= 1 << 1;
    else /* S-Video */
    l |= (1 << 0) | (1 << 2);
    if (venc.invert_polarity == false)
    l |= 1 << 3;
    venc_write_reg(VENC_OUTPUT_CONTROL, l);
    dss_mgr_set_timings(mgr, &venc.timings);
    r = regulator_enable(venc.vdda_dac_reg);
    if (r)
    goto err1;
    r = dss_mgr_enable(mgr);
    if (r)
    goto err2;
    return 0;
    err2:
    regulator_disable(venc.vdda_dac_reg);
    err1:
    venc_write_reg(VENC_OUTPUT_CONTROL, 0);
    dss_set_dac_pwrdn_bgz(0);
    venc_runtime_put();
    err0:
    return r;
    }
#[no_mangle]
unsafe extern "C" fn venc_power_off(dssdev: *mut omap_dss_device) {
    static void venc_power_off(struct omap_dss_device *dssdev)
    {
    struct omap_overlay_manager *mgr = venc.output.manager;
    venc_write_reg(VENC_OUTPUT_CONTROL, 0);
    dss_set_dac_pwrdn_bgz(0);
    dss_mgr_disable(mgr);
    regulator_disable(venc.vdda_dac_reg);
    venc_runtime_put();
    }
#[no_mangle]
unsafe extern "C" fn venc_display_enable(dssdev: *mut omap_dss_device) -> c_int {
    static int venc_display_enable(struct omap_dss_device *dssdev)
    {
    struct omap_dss_device *out = &venc.output;
    int r;
    DSSDBG("venc_display_enable\n");
    mutex_lock(&venc.venc_lock);
    if (out.manager == core::ptr::null_mut()) {
    DSSERR("Failed to enable display: no output/manager\n");
    r = -ENODEV;
    goto err0;
    }
    r = venc_power_on(dssdev);
    if (r)
    goto err0;
    venc.wss_data = 0;
    mutex_unlock(&venc.venc_lock);
    return 0;
    err0:
    mutex_unlock(&venc.venc_lock);
    return r;
    }
#[no_mangle]
unsafe extern "C" fn venc_display_disable(dssdev: *mut omap_dss_device) {
    static void venc_display_disable(struct omap_dss_device *dssdev)
    {
    DSSDBG("venc_display_disable\n");
    mutex_lock(&venc.venc_lock);
    venc_power_off(dssdev);
    mutex_unlock(&venc.venc_lock);
    }
    static void venc_set_timings(struct omap_dss_device *dssdev,
    struct omap_video_timings *timings)
    {
    DSSDBG("venc_set_timings\n");
    mutex_lock(&venc.venc_lock);
// Reset WSS data when the TV standard changes.
    if (memcmp(&venc.timings, timings, sizeof(*timings)))
    venc.wss_data = 0;
    venc.timings = *timings;
    dispc_set_tv_pclk(13500000);
    mutex_unlock(&venc.venc_lock);
    }
    static int venc_check_timings(struct omap_dss_device *dssdev,
    struct omap_video_timings *timings)
    {
    DSSDBG("venc_check_timings\n");
    if (memcmp(&omap_dss_pal_timings, timings, sizeof(*timings)) == 0)
    return 0;
    if (memcmp(&omap_dss_ntsc_timings, timings, sizeof(*timings)) == 0)
    return 0;
    return -EINVAL;
    }
    static void venc_get_timings(struct omap_dss_device *dssdev,
    struct omap_video_timings *timings)
    {
    mutex_lock(&venc.venc_lock);
// timings = venc.timings;
    mutex_unlock(&venc.venc_lock);
    }
#[no_mangle]
unsafe extern "C" fn venc_get_wss(dssdev: *mut omap_dss_device) -> u32 {
    static u32 venc_get_wss(struct omap_dss_device *dssdev)
    {
// Invert due to VENC_L21_WC_CTL:INV=1
    return (venc.wss_data >> 8) ^ 0xfffff;
    }
#[no_mangle]
unsafe extern "C" fn venc_set_wss(dssdev: *mut omap_dss_device, wss: u32) -> c_int {
    static int venc_set_wss(struct omap_dss_device *dssdev, u32 wss)
    {
    const struct venc_config *config;
    int r;
    DSSDBG("venc_set_wss\n");
    mutex_lock(&venc.venc_lock);
    config = venc_timings_to_config(&venc.timings);
// Invert due to VENC_L21_WC_CTL:INV=1
    venc.wss_data = (wss ^ 0xfffff) << 8;
    r = venc_runtime_get();
    if (r)
    goto err;
    venc_write_reg(VENC_BSTAMP_WSS_DATA, config.bstamp_wss_data |
    venc.wss_data);
    venc_runtime_put();
    err:
    mutex_unlock(&venc.venc_lock);
    return r;
    }
    static void venc_set_type(struct omap_dss_device *dssdev,
    enum omap_dss_venc_type type)
    {
    mutex_lock(&venc.venc_lock);
    venc.type = type;
    mutex_unlock(&venc.venc_lock);
    }
    static void venc_invert_vid_out_polarity(struct omap_dss_device *dssdev,
    bool invert_polarity)
    {
    mutex_lock(&venc.venc_lock);
    venc.invert_polarity = invert_polarity;
    mutex_unlock(&venc.venc_lock);
    }
#[no_mangle]
unsafe extern "C" fn venc_init_regulator() -> c_int {
    static int venc_init_regulator(void)
    {
    struct regulator *vdda_dac;
    if (venc.vdda_dac_reg != core::ptr::null_mut())
    return 0;
    if (venc.pdev.dev.of_node)
    vdda_dac = devm_regulator_get(&venc.pdev.dev, "vdda");
    else
    vdda_dac = devm_regulator_get(&venc.pdev.dev, "vdda_dac");
    if (IS_ERR(vdda_dac)) {
    if (PTR_ERR(vdda_dac) != -EPROBE_DEFER)
    DSSERR("can't get VDDA_DAC regulator\n");
    return PTR_ERR(vdda_dac);
    }
    venc.vdda_dac_reg = vdda_dac;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn venc_dump_regs(s: *mut seq_file) {
    static void venc_dump_regs(struct seq_file *s)
    {

    if (venc_runtime_get())
    return;
    DUMPREG(VENC_F_CONTROL);
    DUMPREG(VENC_VIDOUT_CTRL);
    DUMPREG(VENC_SYNC_CTRL);
    DUMPREG(VENC_LLEN);
    DUMPREG(VENC_FLENS);
    DUMPREG(VENC_HFLTR_CTRL);
    DUMPREG(VENC_CC_CARR_WSS_CARR);
    DUMPREG(VENC_C_PHASE);
    DUMPREG(VENC_GAIN_U);
    DUMPREG(VENC_GAIN_V);
    DUMPREG(VENC_GAIN_Y);
    DUMPREG(VENC_BLACK_LEVEL);
    DUMPREG(VENC_BLANK_LEVEL);
    DUMPREG(VENC_X_COLOR);
    DUMPREG(VENC_M_CONTROL);
    DUMPREG(VENC_BSTAMP_WSS_DATA);
    DUMPREG(VENC_S_CARR);
    DUMPREG(VENC_LINE21);
    DUMPREG(VENC_LN_SEL);
    DUMPREG(VENC_L21__WC_CTL);
    DUMPREG(VENC_HTRIGGER_VTRIGGER);
    DUMPREG(VENC_SAVID__EAVID);
    DUMPREG(VENC_FLEN__FAL);
    DUMPREG(VENC_LAL__PHASE_RESET);
    DUMPREG(VENC_HS_INT_START_STOP_X);
    DUMPREG(VENC_HS_EXT_START_STOP_X);
    DUMPREG(VENC_VS_INT_START_X);
    DUMPREG(VENC_VS_INT_STOP_X__VS_INT_START_Y);
    DUMPREG(VENC_VS_INT_STOP_Y__VS_EXT_START_X);
    DUMPREG(VENC_VS_EXT_STOP_X__VS_EXT_START_Y);
    DUMPREG(VENC_VS_EXT_STOP_Y);
    DUMPREG(VENC_AVID_START_STOP_X);
    DUMPREG(VENC_AVID_START_STOP_Y);
    DUMPREG(VENC_FID_INT_START_X__FID_INT_START_Y);
    DUMPREG(VENC_FID_INT_OFFSET_Y__FID_EXT_START_X);
    DUMPREG(VENC_FID_EXT_START_Y__FID_EXT_OFFSET_Y);
    DUMPREG(VENC_TVDETGP_INT_START_STOP_X);
    DUMPREG(VENC_TVDETGP_INT_START_STOP_Y);
    DUMPREG(VENC_GEN_CTRL);
    DUMPREG(VENC_OUTPUT_CONTROL);
    DUMPREG(VENC_OUTPUT_TEST);
    venc_runtime_put();

    }
#[no_mangle]
unsafe extern "C" fn venc_get_clocks(pdev: *mut platform_device) -> c_int {
    static int venc_get_clocks(struct platform_device *pdev)
    {
    struct clk *clk;
    if (dss_has_feature(FEAT_VENC_REQUIRES_TV_DAC_CLK)) {
    clk = devm_clk_get(&pdev.dev, "tv_dac_clk");
    if (IS_ERR(clk)) {
    DSSERR("can't get tv_dac_clk\n");
    return PTR_ERR(clk);
    }
    } else {
    clk = core::ptr::null_mut();
    }
    venc.tv_dac_clk = clk;
    return 0;
    }
    static int venc_connect(struct omap_dss_device *dssdev,
    struct omap_dss_device *dst)
    {
    struct omap_overlay_manager *mgr;
    int r;
    r = venc_init_regulator();
    if (r)
    return r;
    mgr = omap_dss_get_overlay_manager(dssdev.dispc_channel);
    if (!mgr)
    return -ENODEV;
    r = dss_mgr_connect(mgr, dssdev);
    if (r)
    return r;
    r = omapdss_output_set_device(dssdev, dst);
    if (r) {
    DSSERR("failed to connect output to new device: %s\n",
    dst.name);
    dss_mgr_disconnect(mgr, dssdev);
    return r;
    }
    return 0;
    }
    static void venc_disconnect(struct omap_dss_device *dssdev,
    struct omap_dss_device *dst)
    {
    WARN_ON(dst != dssdev.dst);
    if (dst != dssdev.dst)
    return;
    omapdss_output_unset_device(dssdev);
    if (dssdev.manager)
    dss_mgr_disconnect(dssdev.manager, dssdev);
    }
    static const struct omapdss_atv_ops venc_ops = {
    .connect = venc_connect,
    .disconnect = venc_disconnect,
    .enable = venc_display_enable,
    .disable = venc_display_disable,
    .check_timings = venc_check_timings,
    .set_timings = venc_set_timings,
    .get_timings = venc_get_timings,
    .set_type = venc_set_type,
    .invert_vid_out_polarity = venc_invert_vid_out_polarity,
    .set_wss = venc_set_wss,
    .get_wss = venc_get_wss,
    };
#[no_mangle]
unsafe extern "C" fn venc_init_output(pdev: *mut platform_device) {
    static void venc_init_output(struct platform_device *pdev)
    {
    struct omap_dss_device *out = &venc.output;
    out.dev = &pdev.dev;
    out.id = OMAP_DSS_OUTPUT_VENC;
    out.output_type = OMAP_DISPLAY_TYPE_VENC;
    out.name = "venc.0";
    out.dispc_channel = OMAP_DSS_CHANNEL_DIGIT;
    out.ops.atv = &venc_ops;
    out.owner = THIS_MODULE;
    omapdss_register_output(out);
    }
#[no_mangle]
unsafe extern "C" fn venc_uninit_output(pdev: *mut platform_device) {
    static void venc_uninit_output(struct platform_device *pdev)
    {
    struct omap_dss_device *out = &venc.output;
    omapdss_unregister_output(out);
    }
#[no_mangle]
unsafe extern "C" fn venc_probe_of(pdev: *mut platform_device) -> c_int {
    static int venc_probe_of(struct platform_device *pdev)
    {
    struct device_node *node = pdev.dev.of_node;
    struct device_node *ep;
    u32 channels;
    int r;
    ep = of_graph_get_endpoint_by_regs(node, 0, -1);
    if (!ep)
    return 0;
    venc.invert_polarity = of_property_read_bool(ep, "ti,invert-polarity");
    r = of_property_read_u32(ep, "ti,channels", &channels);
    if (r) {
    dev_err(&pdev.dev,
    "failed to read property 'ti,channels': %d\n", r);
    goto err;
    }
    switch (channels) {
    case 1:
    venc.type = OMAP_DSS_VENC_TYPE_COMPOSITE;
    break;
    case 2:
    venc.type = OMAP_DSS_VENC_TYPE_SVIDEO;
    break;
    default:
    dev_err(&pdev.dev, "bad channel property '%d'\n", channels);
    r = -EINVAL;
    goto err;
    }
    of_node_put(ep);
    return 0;
    err:
    of_node_put(ep);
    return 0;
    }
// VENC HW IP initialisation
#[no_mangle]
unsafe extern "C" fn venc_bind(dev: *mut device, master: *mut device, data: *mut c_void) -> c_int {
    static int venc_bind(struct device *dev, struct device *master, void *data)
    {
    struct platform_device *pdev = to_platform_device(dev);
    u8 rev_id;
    struct resource *venc_mem;
    int r;
    venc.pdev = pdev;
    mutex_init(&venc.venc_lock);
    venc.wss_data = 0;
    venc_mem = platform_get_resource(venc.pdev, IORESOURCE_MEM, 0);
    if (!venc_mem) {
    DSSERR("can't get IORESOURCE_MEM VENC\n");
    return -EINVAL;
    }
    venc.base = devm_ioremap(&pdev.dev, venc_mem.start,
    resource_size(venc_mem));
    if (!venc.base) {
    DSSERR("can't ioremap VENC\n");
    return -ENOMEM;
    }
    r = venc_get_clocks(pdev);
    if (r)
    return r;
    pm_runtime_enable(&pdev.dev);
    r = venc_runtime_get();
    if (r)
    goto err_runtime_get;
    rev_id = (u8)(venc_read_reg(VENC_REV_ID) & 0xff);
    dev_dbg(&pdev.dev, "OMAP VENC rev %d\n", rev_id);
    venc_runtime_put();
    if (pdev.dev.of_node) {
    r = venc_probe_of(pdev);
    if (r) {
    DSSERR("Invalid DT data\n");
    goto err_probe_of;
    }
    }
    dss_debugfs_create_file("venc", venc_dump_regs);
    venc_init_output(pdev);
    return 0;
    err_probe_of:
    err_runtime_get:
    pm_runtime_disable(&pdev.dev);
    return r;
    }
#[no_mangle]
unsafe extern "C" fn venc_unbind(dev: *mut device, master: *mut device, data: *mut c_void) {
    static void venc_unbind(struct device *dev, struct device *master, void *data)
    {
    struct platform_device *pdev = to_platform_device(dev);
    venc_uninit_output(pdev);
    pm_runtime_disable(&pdev.dev);
    }
    static const struct component_ops venc_component_ops = {
    .bind	= venc_bind,
    .unbind	= venc_unbind,
    };
#[no_mangle]
unsafe extern "C" fn venc_probe(pdev: *mut platform_device) -> c_int {
    static int venc_probe(struct platform_device *pdev)
    {
    return component_add(&pdev.dev, &venc_component_ops);
    }
#[no_mangle]
unsafe extern "C" fn venc_remove(pdev: *mut platform_device) {
    static void venc_remove(struct platform_device *pdev)
    {
    component_del(&pdev.dev, &venc_component_ops);
    }
#[no_mangle]
unsafe extern "C" fn venc_runtime_suspend(dev: *mut device) -> c_int {
    static int venc_runtime_suspend(struct device *dev)
    {
    clk_disable_unprepare(venc.tv_dac_clk);
    dispc_runtime_put();
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn venc_runtime_resume(dev: *mut device) -> c_int {
    static int venc_runtime_resume(struct device *dev)
    {
    int r;
    r = dispc_runtime_get();
    if (r < 0)
    return r;
    return clk_prepare_enable(venc.tv_dac_clk);
    }
    static const struct dev_pm_ops venc_pm_ops = {
    .runtime_suspend = venc_runtime_suspend,
    .runtime_resume = venc_runtime_resume,
    };
    static const struct of_device_id venc_of_match[] = {
    { .compatible = "ti,omap2-venc", },
    { .compatible = "ti,omap3-venc", },
    { .compatible = "ti,omap4-venc", },
    {},
    };
    static struct platform_driver omap_venchw_driver = {
    .probe		= venc_probe,
    .remove		= venc_remove,
    .driver		= {
    .name	= "omapdss_venc",
    .pm	= &venc_pm_ops,
    .of_match_table = venc_of_match,
    .suppress_bind_attrs = true,
    },
    };
#[no_mangle]
pub unsafe extern "C" fn venc_init_platform_driver() -> int __init {
    int __init venc_init_platform_driver(void)
    {
    return platform_driver_register(&omap_venchw_driver);
    }
#[no_mangle]
pub unsafe extern "C" fn venc_uninit_platform_driver() {
    void venc_uninit_platform_driver(void)
    {
    platform_driver_unregister(&omap_venchw_driver);
    }
