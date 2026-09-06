//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/imx/dcss/dcss-dtg.c
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
// Copyright 2019 NXP.
//

pub const DCSS_DTG_TC_CONTROL_STATUS: c_uint = 0x00;

pub const CSS_PIX_COMP_SWAP_POS: c_int = 12;

pub const DEFAULT_FG_ALPHA_POS: c_int = 24;

pub const DCSS_DTG_TC_DTG: c_uint = 0x04;
pub const DCSS_DTG_TC_DISP_TOP: c_uint = 0x08;
pub const DCSS_DTG_TC_DISP_BOT: c_uint = 0x0C;
pub const DCSS_DTG_TC_CH1_TOP: c_uint = 0x10;
pub const DCSS_DTG_TC_CH1_BOT: c_uint = 0x14;
pub const DCSS_DTG_TC_CH2_TOP: c_uint = 0x18;
pub const DCSS_DTG_TC_CH2_BOT: c_uint = 0x1C;
pub const DCSS_DTG_TC_CH3_TOP: c_uint = 0x20;
pub const DCSS_DTG_TC_CH3_BOT: c_uint = 0x24;
pub const TC_X_POS: c_int = 0;

pub const TC_Y_POS: c_int = 16;

pub const DCSS_DTG_TC_CTXLD: c_uint = 0x28;
pub const TC_CTXLD_DB_Y_POS: c_int = 0;

pub const TC_CTXLD_SB_Y_POS: c_int = 16;

pub const DCSS_DTG_TC_CH1_BKRND: c_uint = 0x2C;
pub const DCSS_DTG_TC_CH2_BKRND: c_uint = 0x30;
pub const BKRND_R_Y_COMP_POS: c_int = 20;

pub const BKRND_G_U_COMP_POS: c_int = 10;

pub const BKRND_B_V_COMP_POS: c_int = 0;

pub const DCSS_DTG_BLENDER_DBY_RANGEINV: c_uint = 0x38;
pub const DCSS_DTG_BLENDER_DBY_RANGEMIN: c_uint = 0x3C;
pub const DCSS_DTG_BLENDER_DBY_BDP: c_uint = 0x40;
pub const DCSS_DTG_BLENDER_BKRND_I: c_uint = 0x44;
pub const DCSS_DTG_BLENDER_BKRND_P: c_uint = 0x48;
pub const DCSS_DTG_BLENDER_BKRND_T: c_uint = 0x4C;
pub const DCSS_DTG_LINE0_INT: c_uint = 0x50;
pub const DCSS_DTG_LINE1_INT: c_uint = 0x54;
pub const DCSS_DTG_BG_ALPHA_DEFAULT: c_uint = 0x58;
pub const DCSS_DTG_INT_STATUS: c_uint = 0x5C;
pub const DCSS_DTG_INT_CONTROL: c_uint = 0x60;
pub const DCSS_DTG_TC_CH3_BKRND: c_uint = 0x64;
pub const DCSS_DTG_INT_MASK: c_uint = 0x68;

pub const DCSS_DTG_LINE2_INT: c_uint = 0x6C;
pub const DCSS_DTG_LINE3_INT: c_uint = 0x70;
pub const DCSS_DTG_DBY_OL: c_uint = 0x74;
pub const DCSS_DTG_DBY_BL: c_uint = 0x78;
pub const DCSS_DTG_DBY_EL: c_uint = 0x7C;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcss_dtg {
    pub dev: *mut device,
    pub ctxld: *mut dcss_ctxld,
    pub base_reg: *mut void __iomem,
    pub base_ofs: u32,
    pub ctx_id: u32,
    pub in_use: bool,
    pub dis_ulc_x: u32,
    pub dis_ulc_y: u32,
    pub control_status: u32,
    pub alpha: u32,
    pub alpha_cfg: u32,
    pub ctxld_kick_irq: c_int,
    pub ctxld_kick_irq_en: bool,
}

#[no_mangle]
unsafe extern "C" fn dcss_dtg_write(dtg: *mut dcss_dtg, val: u32, ofs: u32) {
    static void dcss_dtg_write(struct dcss_dtg *dtg, u32 val, u32 ofs)
    {
    if (!dtg.in_use)
    dcss_writel(val, dtg.base_reg + ofs);
    dcss_ctxld_write(dtg.ctxld, dtg.ctx_id,
    val, dtg.base_ofs + ofs);
    }
#[no_mangle]
unsafe extern "C" fn dcss_dtg_irq_handler(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t dcss_dtg_irq_handler(int irq, void *data)
    {
    struct dcss_dtg *dtg = data;
    u32 status;
    status = dcss_readl(dtg.base_reg + DCSS_DTG_INT_STATUS);
    if (!(status & LINE0_IRQ))
    return IRQ_NONE;
    dcss_ctxld_kick(dtg.ctxld);
    dcss_writel(status & LINE0_IRQ, dtg.base_reg + DCSS_DTG_INT_CONTROL);
    return IRQ_HANDLED;
    }
    static int dcss_dtg_irq_config(struct dcss_dtg *dtg,
    struct platform_device *pdev)
    {
    int ret;
    dtg.ctxld_kick_irq = platform_get_irq_byname(pdev, "ctxld_kick");
    if (dtg.ctxld_kick_irq < 0)
    return dtg.ctxld_kick_irq;
    dcss_update(0, LINE0_IRQ | LINE1_IRQ,
    dtg.base_reg + DCSS_DTG_INT_MASK);
    ret = request_irq(dtg.ctxld_kick_irq, dcss_dtg_irq_handler,
    IRQF_NO_AUTOEN, "dcss_ctxld_kick", dtg);
    if (ret) {
    dev_err(dtg.dev, "dtg: irq request failed.\n");
    return ret;
    }
    dtg.ctxld_kick_irq_en = false;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn dcss_dtg_init(dcss: *mut dcss_dev, dtg_base: c_ulong) -> c_int {
    int dcss_dtg_init(struct dcss_dev *dcss, unsigned long dtg_base)
    {
    let mut ret: c_int = 0;
    struct dcss_dtg *dtg;
    dtg = devm_kzalloc(dcss.dev, sizeof(*dtg), GFP_KERNEL);
    if (!dtg)
    return -ENOMEM;
    dcss.dtg = dtg;
    dtg.dev = dcss.dev;
    dtg.ctxld = dcss.ctxld;
    dtg.base_reg = devm_ioremap(dtg.dev, dtg_base, SZ_4K);
    if (!dtg.base_reg) {
    dev_err(dtg.dev, "dtg: unable to remap dtg base\n");
    return -ENOMEM;
    }
    dtg.base_ofs = dtg_base;
    dtg.ctx_id = CTX_DB;
    dtg.alpha = 255;
    dtg.control_status |= OVL_DATA_MODE | BLENDER_VIDEO_ALPHA_SEL |
    ((dtg.alpha << DEFAULT_FG_ALPHA_POS) & DEFAULT_FG_ALPHA_MASK);
    ret = dcss_dtg_irq_config(dtg, to_platform_device(dtg.dev));
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn dcss_dtg_exit(dtg: *mut dcss_dtg) {
    void dcss_dtg_exit(struct dcss_dtg *dtg)
    {
    free_irq(dtg.ctxld_kick_irq, dtg);
    }
#[no_mangle]
pub unsafe extern "C" fn dcss_dtg_sync_set(dtg: *mut dcss_dtg, vm: *mut videomode) {
    void dcss_dtg_sync_set(struct dcss_dtg *dtg, struct videomode *vm)
    {
    struct dcss_dev *dcss = dcss_drv_dev_to_dcss(dtg.dev);
    u16 dtg_lrc_x, dtg_lrc_y;
    u16 dis_ulc_x, dis_ulc_y;
    u16 dis_lrc_x, dis_lrc_y;
    u32 sb_ctxld_trig, db_ctxld_trig;
    let mut pixclock: u32 = vm.pixelclock;
    u32 actual_clk;
    dtg_lrc_x = vm.hfront_porch + vm.hback_porch + vm.hsync_len +
    vm.hactive - 1;
    dtg_lrc_y = vm.vfront_porch + vm.vback_porch + vm.vsync_len +
    vm.vactive - 1;
    dis_ulc_x = vm.hsync_len + vm.hback_porch - 1;
    dis_ulc_y = vm.vsync_len + vm.vfront_porch + vm.vback_porch - 1;
    dis_lrc_x = vm.hsync_len + vm.hback_porch + vm.hactive - 1;
    dis_lrc_y = vm.vsync_len + vm.vfront_porch + vm.vback_porch +
    vm.vactive - 1;
    clk_disable_unprepare(dcss.pix_clk);
    clk_set_rate(dcss.pix_clk, vm.pixelclock);
    clk_prepare_enable(dcss.pix_clk);
    actual_clk = clk_get_rate(dcss.pix_clk);
    if (pixclock != actual_clk) {
    dev_info(dtg.dev,
    "Pixel clock set to %u kHz instead of %u kHz.\n",
    (actual_clk / 1000), (pixclock / 1000));
    }
    dcss_dtg_write(dtg, ((dtg_lrc_y << TC_Y_POS) | dtg_lrc_x),
    DCSS_DTG_TC_DTG);
    dcss_dtg_write(dtg, ((dis_ulc_y << TC_Y_POS) | dis_ulc_x),
    DCSS_DTG_TC_DISP_TOP);
    dcss_dtg_write(dtg, ((dis_lrc_y << TC_Y_POS) | dis_lrc_x),
    DCSS_DTG_TC_DISP_BOT);
    dtg.dis_ulc_x = dis_ulc_x;
    dtg.dis_ulc_y = dis_ulc_y;
    sb_ctxld_trig = ((0 * dis_lrc_y / 100) << TC_CTXLD_SB_Y_POS) &
    TC_CTXLD_SB_Y_MASK;
    db_ctxld_trig = ((99 * dis_lrc_y / 100) << TC_CTXLD_DB_Y_POS) &
    TC_CTXLD_DB_Y_MASK;
    dcss_dtg_write(dtg, sb_ctxld_trig | db_ctxld_trig, DCSS_DTG_TC_CTXLD);
// vblank trigger
    dcss_dtg_write(dtg, 0, DCSS_DTG_LINE1_INT);
// CTXLD trigger
    dcss_dtg_write(dtg, ((90 * dis_lrc_y) / 100) << 16, DCSS_DTG_LINE0_INT);
    }
    void dcss_dtg_plane_pos_set(struct dcss_dtg *dtg, int ch_num,
    int px, int py, int pw, int ph)
    {
    u16 p_ulc_x, p_ulc_y;
    u16 p_lrc_x, p_lrc_y;
    p_ulc_x = dtg.dis_ulc_x + px;
    p_ulc_y = dtg.dis_ulc_y + py;
    p_lrc_x = p_ulc_x + pw;
    p_lrc_y = p_ulc_y + ph;
    if (!px && !py && !pw && !ph) {
    dcss_dtg_write(dtg, 0, DCSS_DTG_TC_CH1_TOP + 0x8 * ch_num);
    dcss_dtg_write(dtg, 0, DCSS_DTG_TC_CH1_BOT + 0x8 * ch_num);
    } else {
    dcss_dtg_write(dtg, ((p_ulc_y << TC_Y_POS) | p_ulc_x),
    DCSS_DTG_TC_CH1_TOP + 0x8 * ch_num);
    dcss_dtg_write(dtg, ((p_lrc_y << TC_Y_POS) | p_lrc_x),
    DCSS_DTG_TC_CH1_BOT + 0x8 * ch_num);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn dcss_dtg_global_alpha_changed(dtg: *mut dcss_dtg, ch_num: c_int, alpha: c_int) -> bool {
    bool dcss_dtg_global_alpha_changed(struct dcss_dtg *dtg, int ch_num, int alpha)
    {
    if (ch_num)
    return false;
    return alpha != dtg.alpha;
    }
    void dcss_dtg_plane_alpha_set(struct dcss_dtg *dtg, int ch_num,
    const struct drm_format_info *format, int alpha)
    {
// we care about alpha only when channel 0 is concerned
    if (ch_num)
    return;
//
// Use global alpha if pixel format does not have alpha channel or the
// user explicitly chose to use global alpha (i.e. alpha is not OPAQUE).
//
    if (!format.has_alpha || alpha != 255)
    dtg.alpha_cfg = (alpha << DEFAULT_FG_ALPHA_POS) & DEFAULT_FG_ALPHA_MASK;
    else /* use per-pixel alpha otherwise */
    dtg.alpha_cfg = CH1_ALPHA_SEL;
    dtg.alpha = alpha;
    }
#[no_mangle]
pub unsafe extern "C" fn dcss_dtg_css_set(dtg: *mut dcss_dtg) {
    void dcss_dtg_css_set(struct dcss_dtg *dtg)
    {
    dtg.control_status |=
    (0x5 << CSS_PIX_COMP_SWAP_POS) & CSS_PIX_COMP_SWAP_MASK;
    }
#[no_mangle]
pub unsafe extern "C" fn dcss_dtg_enable(dtg: *mut dcss_dtg) {
    void dcss_dtg_enable(struct dcss_dtg *dtg)
    {
    dtg.control_status |= DTG_START;
    dtg.control_status &= ~(CH1_ALPHA_SEL | DEFAULT_FG_ALPHA_MASK);
    dtg.control_status |= dtg.alpha_cfg;
    dcss_dtg_write(dtg, dtg.control_status, DCSS_DTG_TC_CONTROL_STATUS);
    dtg.in_use = true;
    }
#[no_mangle]
pub unsafe extern "C" fn dcss_dtg_shutoff(dtg: *mut dcss_dtg) {
    void dcss_dtg_shutoff(struct dcss_dtg *dtg)
    {
    dtg.control_status &= ~DTG_START;
    dcss_writel(dtg.control_status,
    dtg.base_reg + DCSS_DTG_TC_CONTROL_STATUS);
    dtg.in_use = false;
    }
#[no_mangle]
pub unsafe extern "C" fn dcss_dtg_is_enabled(dtg: *mut dcss_dtg) -> bool {
    bool dcss_dtg_is_enabled(struct dcss_dtg *dtg)
    {
    return dtg.in_use;
    }
#[no_mangle]
pub unsafe extern "C" fn dcss_dtg_ch_enable(dtg: *mut dcss_dtg, ch_num: c_int, en: bool) {
    void dcss_dtg_ch_enable(struct dcss_dtg *dtg, int ch_num, bool en)
    {
    u32 ch_en_map[] = {CH1_EN, CH2_EN, CH3_EN};
    u32 control_status;
    control_status = dtg.control_status & ~ch_en_map[ch_num];
    control_status |= en ? ch_en_map[ch_num] : 0;
    control_status &= ~(CH1_ALPHA_SEL | DEFAULT_FG_ALPHA_MASK);
    control_status |= dtg.alpha_cfg;
    if (dtg.control_status != control_status)
    dcss_dtg_write(dtg, control_status, DCSS_DTG_TC_CONTROL_STATUS);
    dtg.control_status = control_status;
    }
#[no_mangle]
pub unsafe extern "C" fn dcss_dtg_vblank_irq_enable(dtg: *mut dcss_dtg, en: bool) {
    void dcss_dtg_vblank_irq_enable(struct dcss_dtg *dtg, bool en)
    {
    u32 status;
    let mut mask: u32 = en ? LINE1_IRQ : 0;
    if (en) {
    status = dcss_readl(dtg.base_reg + DCSS_DTG_INT_STATUS);
    dcss_writel(status & LINE1_IRQ,
    dtg.base_reg + DCSS_DTG_INT_CONTROL);
    }
    dcss_update(mask, LINE1_IRQ, dtg.base_reg + DCSS_DTG_INT_MASK);
    }
#[no_mangle]
pub unsafe extern "C" fn dcss_dtg_ctxld_kick_irq_enable(dtg: *mut dcss_dtg, en: bool) {
    void dcss_dtg_ctxld_kick_irq_enable(struct dcss_dtg *dtg, bool en)
    {
    u32 status;
    let mut mask: u32 = en ? LINE0_IRQ : 0;
    if (en) {
    status = dcss_readl(dtg.base_reg + DCSS_DTG_INT_STATUS);
    if (!dtg.ctxld_kick_irq_en) {
    dcss_writel(status & LINE0_IRQ,
    dtg.base_reg + DCSS_DTG_INT_CONTROL);
    enable_irq(dtg.ctxld_kick_irq);
    dtg.ctxld_kick_irq_en = true;
    dcss_update(mask, LINE0_IRQ,
    dtg.base_reg + DCSS_DTG_INT_MASK);
    }
    return;
    }
    if (!dtg.ctxld_kick_irq_en)
    return;
    disable_irq_nosync(dtg.ctxld_kick_irq);
    dtg.ctxld_kick_irq_en = false;
    dcss_update(mask, LINE0_IRQ, dtg.base_reg + DCSS_DTG_INT_MASK);
    }
#[no_mangle]
pub unsafe extern "C" fn dcss_dtg_vblank_irq_clear(dtg: *mut dcss_dtg) {
    void dcss_dtg_vblank_irq_clear(struct dcss_dtg *dtg)
    {
    dcss_update(LINE1_IRQ, LINE1_IRQ, dtg.base_reg + DCSS_DTG_INT_CONTROL);
    }
#[no_mangle]
pub unsafe extern "C" fn dcss_dtg_vblank_irq_valid(dtg: *mut dcss_dtg) -> bool {
    bool dcss_dtg_vblank_irq_valid(struct dcss_dtg *dtg)
    {
    return !!(dcss_readl(dtg.base_reg + DCSS_DTG_INT_STATUS) & LINE1_IRQ);
    }
