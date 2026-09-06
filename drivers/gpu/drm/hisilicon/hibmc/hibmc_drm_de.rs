//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/hisilicon/hibmc/hibmc_drm_de.c
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
// Hisilicon Hibmc SoC drm driver
//
// Based on the bochs drm driver.
//
// Copyright (c) 2016 Huawei Limited.
//
// Author:
// Rongrong Zou <zourongrong@huawei.com>
// Rongrong Zou <zourongrong@gmail.com>
// Jianhua Li <lijianhua@huawei.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hibmc_display_panel_pll {
    pub M: u64,
    pub N: u64,
    pub OD: u64,
    pub POD: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hibmc_dislay_pll_config {
    pub hdisplay: u64,
    pub vdisplay: u64,
    pub clock: c_int,
    pub pll1_config_value: u32,
    pub pll2_config_value: u32,
}

    static const struct hibmc_dislay_pll_config hibmc_pll_table[] = {
    {640, 480, 25000, CRT_PLL1_HS_25MHZ, CRT_PLL2_HS_25MHZ},
    {800, 600, 40000, CRT_PLL1_HS_40MHZ, CRT_PLL2_HS_40MHZ},
    {1024, 768, 65000, CRT_PLL1_HS_65MHZ, CRT_PLL2_HS_65MHZ},
    {1152, 864, 78750, CRT_PLL1_HS_80MHZ_1152, CRT_PLL2_HS_80MHZ},
    {1280, 768, 80000, CRT_PLL1_HS_80MHZ, CRT_PLL2_HS_80MHZ},
    {1280, 720, 74375, CRT_PLL1_HS_74MHZ, CRT_PLL2_HS_74MHZ},
    {1280, 960, 108000, CRT_PLL1_HS_108MHZ, CRT_PLL2_HS_108MHZ},
    {1280, 1024, 108000, CRT_PLL1_HS_108MHZ, CRT_PLL2_HS_108MHZ},
    {1440, 900, 105952, CRT_PLL1_HS_106MHZ, CRT_PLL2_HS_106MHZ},
    {1600, 900, 108000, CRT_PLL1_HS_108MHZ, CRT_PLL2_HS_108MHZ},
    {1600, 1200, 162500, CRT_PLL1_HS_162MHZ, CRT_PLL2_HS_162MHZ},
    {1920, 1080, 148750, CRT_PLL1_HS_148MHZ, CRT_PLL2_HS_148MHZ},
    {1920, 1200, 193750, CRT_PLL1_HS_193MHZ, CRT_PLL2_HS_193MHZ},
    };
#[no_mangle]
unsafe extern "C" fn hibmc_get_best_clock_idx(mode: *const drm_display_mode) -> c_int {
    static int hibmc_get_best_clock_idx(const struct drm_display_mode *mode)
    {
    int i, diff;
    for (i = 0; i < ARRAY_SIZE(hibmc_pll_table); i++) {
    if (hibmc_pll_table[i].hdisplay == mode.hdisplay &&
    hibmc_pll_table[i].vdisplay == mode.vdisplay) {
    diff = abs(mode.clock - hibmc_pll_table[i].clock);
    if (diff < mode.clock / 100) /* tolerance 1/100 */
    return i;
    }
    }
    return -MODE_CLOCK_RANGE;
    }
    static int hibmc_plane_atomic_check(struct drm_plane *plane,
    struct drm_atomic_commit *state)
    {
    struct drm_plane_state *new_plane_state =
    drm_atomic_get_new_plane_state(state, plane);
    struct drm_crtc_state *new_crtc_state = core::ptr::null_mut();
    int ret;
    if (new_plane_state.crtc)
    new_crtc_state = drm_atomic_get_new_crtc_state(state, new_plane_state.crtc);
    ret = drm_atomic_helper_check_plane_state(new_plane_state, new_crtc_state,
    DRM_PLANE_NO_SCALING,
    DRM_PLANE_NO_SCALING,
    false, true);
    if (ret)
    return ret;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !new_plane_state->visible) -> else {
    else if (!new_plane_state.visible)
    return 0;
    if (new_plane_state.fb.pitches[0] % 128 != 0) {
    drm_dbg_atomic(plane.dev, "wrong stride with 128-byte aligned\n");
    return -EINVAL;
    }
    return 0;
    }
    static void hibmc_plane_atomic_update(struct drm_plane *plane,
    struct drm_atomic_commit *state)
    {
    struct hibmc_drm_private *priv = to_hibmc_drm_private(plane.dev);
    struct drm_plane_state *new_state = drm_atomic_get_new_plane_state(state, plane);
    struct drm_shadow_plane_state *shadow_plane_state = to_drm_shadow_plane_state(new_state);
    struct drm_framebuffer *fb = new_state.fb;
    struct drm_plane_state *old_state = drm_atomic_get_old_plane_state(state, plane);
    let mut gpu_addr: u32 = 0;
    u32 reg;
    u32 line_l;
    if (!fb)
    return;
    if (drm_gem_fb_begin_cpu_access(fb, DMA_FROM_DEVICE) == 0) {
    struct drm_rect damage;
    struct drm_atomic_helper_damage_iter iter;
    drm_atomic_helper_damage_iter_init(&iter, old_state, new_state);
    drm_atomic_for_each_plane_damage(&iter, &damage) {
    struct iosys_map dst[DRM_FORMAT_MAX_PLANES] = {
    IOSYS_MAP_INIT_VADDR_IOMEM(priv.vram + gpu_addr),
    };
    iosys_map_incr(&dst[0],
    drm_fb_clip_offset(fb.pitches[0], fb.format, &damage));
    drm_fb_memcpy(dst, fb.pitches, shadow_plane_state.data, fb, &damage);
    }
    drm_gem_fb_end_cpu_access(fb, DMA_FROM_DEVICE);
    }
    writel(gpu_addr, priv.mmio + HIBMC_CRT_FB_ADDRESS);
    reg = drm_format_info_min_pitch(fb.format, 0, fb.width);
    line_l = fb.pitches[0];
    writel(HIBMC_FIELD(HIBMC_CRT_FB_WIDTH_WIDTH, reg) |
    HIBMC_FIELD(HIBMC_CRT_FB_WIDTH_OFFS, line_l),
    priv.mmio + HIBMC_CRT_FB_WIDTH);
// SET PIXEL FORMAT
    reg = readl(priv.mmio + HIBMC_CRT_DISP_CTL);
    reg &= ~HIBMC_CRT_DISP_CTL_FORMAT_MASK;
    switch (fb.format.format) {
    case DRM_FORMAT_XRGB8888:
    reg |= HIBMC_FIELD(HIBMC_CRT_DISP_CTL_FORMAT, 2);
    break;
    case DRM_FORMAT_RGB565:
    reg |= HIBMC_FIELD(HIBMC_CRT_DISP_CTL_FORMAT, 1);
    break;
    }
    writel(reg, priv.mmio + HIBMC_CRT_DISP_CTL);
    }
    static const u32 channel_formats1[] = {
    DRM_FORMAT_XRGB8888,
    DRM_FORMAT_RGB565,
    };
    static const struct drm_plane_funcs hibmc_plane_funcs = {
    .update_plane	= drm_atomic_helper_update_plane,
    .disable_plane	= drm_atomic_helper_disable_plane,
    .destroy = drm_plane_cleanup,
    DRM_GEM_SHADOW_PLANE_FUNCS,
    };
    static const struct drm_plane_helper_funcs hibmc_plane_helper_funcs = {
    DRM_GEM_SHADOW_PLANE_HELPER_FUNCS,
    .atomic_check = hibmc_plane_atomic_check,
    .atomic_update = hibmc_plane_atomic_update,
    };
#[no_mangle]
unsafe extern "C" fn hibmc_crtc_dpms(crtc: *mut drm_crtc, dpms: u32) {
    static void hibmc_crtc_dpms(struct drm_crtc *crtc, u32 dpms)
    {
    struct hibmc_drm_private *priv = to_hibmc_drm_private(crtc.dev);
    u32 reg;
    reg = readl(priv.mmio + HIBMC_CRT_DISP_CTL);
    reg &= ~HIBMC_CRT_DISP_CTL_DPMS_MASK;
    reg |= HIBMC_FIELD(HIBMC_CRT_DISP_CTL_DPMS, dpms);
    reg &= ~HIBMC_CRT_DISP_CTL_TIMING_MASK;
    if (dpms == HIBMC_CRT_DPMS_ON)
    reg |= HIBMC_CRT_DISP_CTL_TIMING(1);
    writel(reg, priv.mmio + HIBMC_CRT_DISP_CTL);
    }
    static void hibmc_crtc_atomic_enable(struct drm_crtc *crtc,
    struct drm_atomic_commit *state)
    {
    u32 reg;
    struct hibmc_drm_private *priv = to_hibmc_drm_private(crtc.dev);
    hibmc_set_power_mode(priv, HIBMC_PW_MODE_CTL_MODE_MODE0);
// Enable display power gate & LOCALMEM power gate
    reg = readl(priv.mmio + HIBMC_CURRENT_GATE);
    reg &= ~HIBMC_CURR_GATE_LOCALMEM_MASK;
    reg &= ~HIBMC_CURR_GATE_DISPLAY_MASK;
    reg |= HIBMC_CURR_GATE_LOCALMEM(1);
    reg |= HIBMC_CURR_GATE_DISPLAY(1);
    hibmc_set_current_gate(priv, reg);
    drm_crtc_vblank_on(crtc);
    hibmc_crtc_dpms(crtc, HIBMC_CRT_DPMS_ON);
    }
    static void hibmc_crtc_atomic_disable(struct drm_crtc *crtc,
    struct drm_atomic_commit *state)
    {
    u32 reg;
    struct hibmc_drm_private *priv = to_hibmc_drm_private(crtc.dev);
    hibmc_crtc_dpms(crtc, HIBMC_CRT_DPMS_OFF);
    drm_crtc_vblank_off(crtc);
    hibmc_set_power_mode(priv, HIBMC_PW_MODE_CTL_MODE_SLEEP);
// Enable display power gate & LOCALMEM power gate
    reg = readl(priv.mmio + HIBMC_CURRENT_GATE);
    reg &= ~HIBMC_CURR_GATE_LOCALMEM_MASK;
    reg &= ~HIBMC_CURR_GATE_DISPLAY_MASK;
    reg |= HIBMC_CURR_GATE_LOCALMEM(0);
    reg |= HIBMC_CURR_GATE_DISPLAY(0);
    hibmc_set_current_gate(priv, reg);
    }
    static enum drm_mode_status
    hibmc_crtc_mode_valid(struct drm_crtc *crtc,
    const struct drm_display_mode *mode)
    {
    let mut vrefresh: c_int = drm_mode_vrefresh(mode);
    if (vrefresh < 59 || vrefresh > 61)
    return MODE_NOCLOCK;
    if (hibmc_get_best_clock_idx(mode) >= 0)
    return MODE_OK;
    return MODE_CLOCK_RANGE;
    }
#[no_mangle]
unsafe extern "C" fn format_pll_reg() -> u32 {
    static u32 format_pll_reg(void)
    {
    let mut pllreg: u32 = 0;
    let mut pll: hibmc_display_panel_pll = {0};
//
// Note that all PLL's have the same format. Here,
// we just use Panel PLL parameter to work out the bit
// fields in the register.On returning a 32 bit number, the value can
// be applied to any PLL in the calling function.
//
    pllreg |= HIBMC_FIELD(HIBMC_PLL_CTRL_BYPASS, 0);
    pllreg |= HIBMC_FIELD(HIBMC_PLL_CTRL_POWER, 1);
    pllreg |= HIBMC_FIELD(HIBMC_PLL_CTRL_INPUT, 0);
    pllreg |= HIBMC_FIELD(HIBMC_PLL_CTRL_POD, pll.POD);
    pllreg |= HIBMC_FIELD(HIBMC_PLL_CTRL_OD, pll.OD);
    pllreg |= HIBMC_FIELD(HIBMC_PLL_CTRL_N, pll.N);
    pllreg |= HIBMC_FIELD(HIBMC_PLL_CTRL_M, pll.M);
    return pllreg;
    }
#[no_mangle]
unsafe extern "C" fn set_vclock_hisilicon(dev: *mut drm_device, pll: u64) {
    static void set_vclock_hisilicon(struct drm_device *dev, u64 pll)
    {
    u32 val;
    struct hibmc_drm_private *priv = to_hibmc_drm_private(dev);
    val = readl(priv.mmio + CRT_PLL1_HS);
    val &= ~(CRT_PLL1_HS_OUTER_BYPASS(1));
    writel(val, priv.mmio + CRT_PLL1_HS);
    val = CRT_PLL1_HS_INTER_BYPASS(1) | CRT_PLL1_HS_POWERON(1);
    writel(val, priv.mmio + CRT_PLL1_HS);
    writel(pll, priv.mmio + CRT_PLL1_HS);
    usleep_range(1000, 2000);
    val = pll & ~(CRT_PLL1_HS_POWERON(1));
    writel(val, priv.mmio + CRT_PLL1_HS);
    usleep_range(1000, 2000);
    val &= ~(CRT_PLL1_HS_INTER_BYPASS(1));
    writel(val, priv.mmio + CRT_PLL1_HS);
    usleep_range(1000, 2000);
    val |= CRT_PLL1_HS_OUTER_BYPASS(1);
    writel(val, priv.mmio + CRT_PLL1_HS);
    }
#[no_mangle]
unsafe extern "C" fn get_pll_config(mode: *mut drm_display_mode, pll1: *mut u32, pll2: *mut u32) {
    static void get_pll_config(struct drm_display_mode *mode, u32 *pll1, u32 *pll2)
    {
    int idx;
    idx = hibmc_get_best_clock_idx(mode);
    if (idx < 0) {
// if found none, we use default value
// pll1 = CRT_PLL1_HS_25MHZ;
// pll2 = CRT_PLL2_HS_25MHZ;
    return;
    }
// pll1 = hibmc_pll_table[idx].pll1_config_value;
// pll2 = hibmc_pll_table[idx].pll2_config_value;
    }
//
// This function takes care the extra registers and bit fields required to
// setup a mode in board.
// Explanation about Display Control register:
// FPGA only supports 7 predefined pixel clocks, and clock select is
// in bit 4:0 of new register 0x802a8.
//
    static u32 display_ctrl_adjust(struct drm_device *dev,
    struct drm_display_mode *mode,
    u32 ctrl)
    {
    u64 x, y;
    u32 pll1; /* bit[31:0] of PLL */
    u32 pll2; /* bit[63:32] of PLL */
    struct hibmc_drm_private *priv = to_hibmc_drm_private(dev);
    x = mode.hdisplay;
    y = mode.vdisplay;
    get_pll_config(mode, &pll1, &pll2);
    writel(pll2, priv.mmio + CRT_PLL2_HS);
    set_vclock_hisilicon(dev, pll1);
//
// Hisilicon has to set up the top-left and bottom-right
// registers as well.
// Note that normal chip only use those two register for
// auto-centering mode.
//
    writel(HIBMC_FIELD(HIBMC_CRT_AUTO_CENTERING_TL_TOP, 0) |
    HIBMC_FIELD(HIBMC_CRT_AUTO_CENTERING_TL_LEFT, 0),
    priv.mmio + HIBMC_CRT_AUTO_CENTERING_TL);
    writel(HIBMC_FIELD(HIBMC_CRT_AUTO_CENTERING_BR_BOTTOM, y - 1) |
    HIBMC_FIELD(HIBMC_CRT_AUTO_CENTERING_BR_RIGHT, x - 1),
    priv.mmio + HIBMC_CRT_AUTO_CENTERING_BR);
//
// Assume common fields in ctrl have been properly set before
// calling this function.
// This function only sets the extra fields in ctrl.
//
// Set bit 25 of display controller: Select CRT or VGA clock
    ctrl &= ~HIBMC_CRT_DISP_CTL_CRTSELECT_MASK;
    ctrl &= ~HIBMC_CRT_DISP_CTL_CLOCK_PHASE_MASK;
    ctrl |= HIBMC_CRT_DISP_CTL_CRTSELECT(HIBMC_CRTSELECT_CRT);
// clock_phase_polarity is 0
    ctrl |= HIBMC_CRT_DISP_CTL_CLOCK_PHASE(0);
    writel(ctrl, priv.mmio + HIBMC_CRT_DISP_CTL);
    return ctrl;
    }
#[no_mangle]
unsafe extern "C" fn hibmc_crtc_mode_set_nofb(crtc: *mut drm_crtc) {
    static void hibmc_crtc_mode_set_nofb(struct drm_crtc *crtc)
    {
    u32 val;
    struct drm_display_mode *mode = &crtc.state.mode;
    struct drm_device *dev = crtc.dev;
    struct hibmc_drm_private *priv = to_hibmc_drm_private(dev);
    let mut width: u32 = mode.hsync_end - mode.hsync_start;
    let mut height: u32 = mode.vsync_end - mode.vsync_start;
    writel(format_pll_reg(), priv.mmio + HIBMC_CRT_PLL_CTRL);
    writel(HIBMC_FIELD(HIBMC_CRT_HORZ_TOTAL_TOTAL, mode.htotal - 1) |
    HIBMC_FIELD(HIBMC_CRT_HORZ_TOTAL_DISP_END, mode.hdisplay - 1),
    priv.mmio + HIBMC_CRT_HORZ_TOTAL);
    writel(HIBMC_FIELD(HIBMC_CRT_HORZ_SYNC_WIDTH, width) |
    HIBMC_FIELD(HIBMC_CRT_HORZ_SYNC_START, mode.hsync_start - 1),
    priv.mmio + HIBMC_CRT_HORZ_SYNC);
    writel(HIBMC_FIELD(HIBMC_CRT_VERT_TOTAL_TOTAL, mode.vtotal - 1) |
    HIBMC_FIELD(HIBMC_CRT_VERT_TOTAL_DISP_END, mode.vdisplay - 1),
    priv.mmio + HIBMC_CRT_VERT_TOTAL);
    writel(HIBMC_FIELD(HIBMC_CRT_VERT_SYNC_HEIGHT, height) |
    HIBMC_FIELD(HIBMC_CRT_VERT_SYNC_START, mode.vsync_start - 1),
    priv.mmio + HIBMC_CRT_VERT_SYNC);
    val = HIBMC_FIELD(HIBMC_CRT_DISP_CTL_VSYNC_PHASE, 0);
    val |= HIBMC_FIELD(HIBMC_CRT_DISP_CTL_HSYNC_PHASE, 0);
    val |= HIBMC_CRT_DISP_CTL_TIMING(1);
    val |= HIBMC_CRT_DISP_CTL_PLANE(1);
    display_ctrl_adjust(dev, mode, val);
    }
    static void hibmc_crtc_atomic_begin(struct drm_crtc *crtc,
    struct drm_atomic_commit *state)
    {
    u32 reg;
    struct drm_device *dev = crtc.dev;
    struct hibmc_drm_private *priv = to_hibmc_drm_private(dev);
    hibmc_set_power_mode(priv, HIBMC_PW_MODE_CTL_MODE_MODE0);
// Enable display power gate & LOCALMEM power gate
    reg = readl(priv.mmio + HIBMC_CURRENT_GATE);
    reg &= ~HIBMC_CURR_GATE_DISPLAY_MASK;
    reg &= ~HIBMC_CURR_GATE_LOCALMEM_MASK;
    reg |= HIBMC_CURR_GATE_DISPLAY(1);
    reg |= HIBMC_CURR_GATE_LOCALMEM(1);
    hibmc_set_current_gate(priv, reg);
// We can add more initialization as needed.
    }
    static void hibmc_crtc_atomic_flush(struct drm_crtc *crtc,
    struct drm_atomic_commit *state)
    {
    unsigned long flags;
    spin_lock_irqsave(&crtc.dev.event_lock, flags);
    if (crtc.state.event)
    drm_crtc_send_vblank_event(crtc, crtc.state.event);
    crtc.state.event = core::ptr::null_mut();
    spin_unlock_irqrestore(&crtc.dev.event_lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn hibmc_crtc_enable_vblank(crtc: *mut drm_crtc) -> c_int {
    static int hibmc_crtc_enable_vblank(struct drm_crtc *crtc)
    {
    struct hibmc_drm_private *priv = to_hibmc_drm_private(crtc.dev);
    writel(HIBMC_RAW_INTERRUPT_EN_VBLANK(1),
    priv.mmio + HIBMC_RAW_INTERRUPT_EN);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hibmc_crtc_disable_vblank(crtc: *mut drm_crtc) {
    static void hibmc_crtc_disable_vblank(struct drm_crtc *crtc)
    {
    struct hibmc_drm_private *priv = to_hibmc_drm_private(crtc.dev);
    writel(HIBMC_RAW_INTERRUPT_EN_VBLANK(0),
    priv.mmio + HIBMC_RAW_INTERRUPT_EN);
    }
#[no_mangle]
unsafe extern "C" fn hibmc_crtc_load_lut(crtc: *mut drm_crtc) {
    static void hibmc_crtc_load_lut(struct drm_crtc *crtc)
    {
    struct hibmc_drm_private *priv = to_hibmc_drm_private(crtc.dev);
    void __iomem   *mmio = priv.mmio;
    u16 *r, *g, *b;
    u32 reg;
    u32 i;
    r = crtc.gamma_store;
    g = r + crtc.gamma_size;
    b = g + crtc.gamma_size;
    for (i = 0; i < crtc.gamma_size; i++) {
    let mut offset: u32 = i << 2;
    let mut red: u8 = *r++ >> 8;
    let mut green: u8 = *g++ >> 8;
    let mut blue: u8 = *b++ >> 8;
    let mut rgb: u32 = (red << 16) | (green << 8) | blue;
    writel(rgb, mmio + HIBMC_CRT_PALETTE + offset);
    }
    reg = readl(priv.mmio + HIBMC_CRT_DISP_CTL);
    reg |= HIBMC_FIELD(HIBMC_CTL_DISP_CTL_GAMMA, 1);
    writel(reg, priv.mmio + HIBMC_CRT_DISP_CTL);
    }
    static int hibmc_crtc_gamma_set(struct drm_crtc *crtc, u16 *red, u16 *green,
    u16 *blue, uint32_t size,
    struct drm_modeset_acquire_ctx *ctx)
    {
    hibmc_crtc_load_lut(crtc);
    return 0;
    }
    static const struct drm_crtc_funcs hibmc_crtc_funcs = {
    .page_flip = drm_atomic_helper_page_flip,
    .set_config = drm_atomic_helper_set_config,
    .destroy = drm_crtc_cleanup,
    .reset = drm_atomic_helper_crtc_reset,
    .atomic_duplicate_state =  drm_atomic_helper_crtc_duplicate_state,
    .atomic_destroy_state = drm_atomic_helper_crtc_destroy_state,
    .enable_vblank = hibmc_crtc_enable_vblank,
    .disable_vblank = hibmc_crtc_disable_vblank,
    .gamma_set = hibmc_crtc_gamma_set,
    };
    static const struct drm_crtc_helper_funcs hibmc_crtc_helper_funcs = {
    .mode_set_nofb	= hibmc_crtc_mode_set_nofb,
    .atomic_begin	= hibmc_crtc_atomic_begin,
    .atomic_flush	= hibmc_crtc_atomic_flush,
    .atomic_enable	= hibmc_crtc_atomic_enable,
    .atomic_disable	= hibmc_crtc_atomic_disable,
    .mode_valid = hibmc_crtc_mode_valid,
    };
#[no_mangle]
pub unsafe extern "C" fn hibmc_de_init(priv: *mut hibmc_drm_private) -> c_int {
    int hibmc_de_init(struct hibmc_drm_private *priv)
    {
    struct drm_device *dev = &priv.dev;
    struct drm_crtc *crtc = &priv.crtc;
    struct drm_plane *plane = &priv.primary_plane;
    int ret;
    ret = drm_universal_plane_init(dev, plane, 1, &hibmc_plane_funcs,
    channel_formats1,
    ARRAY_SIZE(channel_formats1),
    core::ptr::null_mut(),
    DRM_PLANE_TYPE_PRIMARY,
    core::ptr::null_mut());
    if (ret) {
    drm_err(dev, "failed to init plane: %d\n", ret);
    return ret;
    }
    drm_plane_helper_add(plane, &hibmc_plane_helper_funcs);
    drm_plane_enable_fb_damage_clips(plane);
    ret = drm_crtc_init_with_planes(dev, crtc, plane,
    core::ptr::null_mut(), &hibmc_crtc_funcs, core::ptr::null_mut());
    if (ret) {
    drm_err(dev, "failed to init crtc: %d\n", ret);
    return ret;
    }
    ret = drm_mode_crtc_set_gamma_size(crtc, 256);
    if (ret) {
    drm_err(dev, "failed to set gamma size: %d\n", ret);
    return ret;
    }
    drm_crtc_helper_add(crtc, &hibmc_crtc_helper_funcs);
    return 0;
    }
