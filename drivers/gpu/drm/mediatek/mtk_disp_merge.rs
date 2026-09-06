//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/mediatek/mtk_disp_merge.c
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
// Copyright (c) 2021 MediaTek Inc.
//

pub const DISP_REG_MERGE_CTRL: c_uint = 0x000;
pub const MERGE_EN: c_int = 1;
pub const DISP_REG_MERGE_CFG_0: c_uint = 0x010;
pub const DISP_REG_MERGE_CFG_1: c_uint = 0x014;
pub const DISP_REG_MERGE_CFG_4: c_uint = 0x020;
pub const DISP_REG_MERGE_CFG_10: c_uint = 0x038;
// no swap
pub const SWAP_MODE: c_int = 0;

pub const DISP_REG_MERGE_CFG_12: c_uint = 0x040;
pub const CFG_10_10_1PI_2PO_BUF_MODE: c_int = 6;
pub const CFG_10_10_2PI_2PO_BUF_MODE: c_int = 8;
pub const CFG_11_10_1PI_2PO_MERGE: c_int = 18;

pub const DISP_REG_MERGE_CFG_24: c_uint = 0x070;
pub const DISP_REG_MERGE_CFG_25: c_uint = 0x074;
pub const DISP_REG_MERGE_CFG_26: c_uint = 0x078;
pub const DISP_REG_MERGE_CFG_27: c_uint = 0x07c;
pub const DISP_REG_MERGE_CFG_36: c_uint = 0x0a0;

pub const DISP_REG_MERGE_CFG_37: c_uint = 0x0a4;
// 0: Off, 1: SRAM0, 2: SRAM1, 3: SRAM0 + SRAM1
pub const BUFFER_MODE: c_int = 3;

//
// For the ultra and preultra settings, 6us ~ 9us is experience value
// and the maximum frequency of mmsys clock is 594MHz.
//
pub const DISP_REG_MERGE_CFG_40: c_uint = 0x0b0;
// 6 us, 594M pixel/sec

// 8 us, 594M pixel/sec

pub const DISP_REG_MERGE_CFG_41: c_uint = 0x0b4;
// 8 us, 594M pixel/sec

// 9 us, 594M pixel/sec

pub const DISP_REG_MERGE_MUTE_0: c_uint = 0xf00;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_disp_merge {
    pub regs: *mut void __iomem,
    pub clk: *mut clk,
    pub async_clk: *mut clk,
    pub cmdq_reg: cmdq_client_reg,
    pub fifo_en: bool,
    pub mute_support: bool,
    pub reset_ctl: *mut reset_control,
}

#[no_mangle]
pub unsafe extern "C" fn mtk_merge_start(dev: *mut device) {
    void mtk_merge_start(struct device *dev)
    {
    mtk_merge_start_cmdq(dev, core::ptr::null_mut());
    }
#[no_mangle]
pub unsafe extern "C" fn mtk_merge_stop(dev: *mut device) {
    void mtk_merge_stop(struct device *dev)
    {
    mtk_merge_stop_cmdq(dev, core::ptr::null_mut());
    }
#[no_mangle]
pub unsafe extern "C" fn mtk_merge_start_cmdq(dev: *mut device, cmdq_pkt: *mut cmdq_pkt) {
    void mtk_merge_start_cmdq(struct device *dev, struct cmdq_pkt *cmdq_pkt)
    {
    struct mtk_disp_merge *priv = dev_get_drvdata(dev);
    if (priv.mute_support)
    mtk_ddp_write(cmdq_pkt, 0x0, &priv.cmdq_reg, priv.regs,
    DISP_REG_MERGE_MUTE_0);
    mtk_ddp_write(cmdq_pkt, 1, &priv.cmdq_reg, priv.regs,
    DISP_REG_MERGE_CTRL);
    }
#[no_mangle]
pub unsafe extern "C" fn mtk_merge_stop_cmdq(dev: *mut device, cmdq_pkt: *mut cmdq_pkt) {
    void mtk_merge_stop_cmdq(struct device *dev, struct cmdq_pkt *cmdq_pkt)
    {
    struct mtk_disp_merge *priv = dev_get_drvdata(dev);
    if (priv.mute_support)
    mtk_ddp_write(cmdq_pkt, 0x1, &priv.cmdq_reg, priv.regs,
    DISP_REG_MERGE_MUTE_0);
    mtk_ddp_write(cmdq_pkt, 0, &priv.cmdq_reg, priv.regs,
    DISP_REG_MERGE_CTRL);
    if (!cmdq_pkt && priv.async_clk)
    reset_control_reset(priv.reset_ctl);
    }
    static void mtk_merge_fifo_setting(struct mtk_disp_merge *priv,
    struct cmdq_pkt *cmdq_pkt)
    {
    mtk_ddp_write(cmdq_pkt, ULTRA_EN | PREULTRA_EN,
    &priv.cmdq_reg, priv.regs, DISP_REG_MERGE_CFG_36);
    mtk_ddp_write_mask(cmdq_pkt, BUFFER_MODE,
    &priv.cmdq_reg, priv.regs, DISP_REG_MERGE_CFG_37,
    FLD_BUFFER_MODE);
    mtk_ddp_write_mask(cmdq_pkt, ULTRA_TH_LOW | ULTRA_TH_HIGH << 16,
    &priv.cmdq_reg, priv.regs, DISP_REG_MERGE_CFG_40,
    FLD_ULTRA_TH_LOW | FLD_ULTRA_TH_HIGH);
    mtk_ddp_write_mask(cmdq_pkt, PREULTRA_TH_LOW | PREULTRA_TH_HIGH << 16,
    &priv.cmdq_reg, priv.regs, DISP_REG_MERGE_CFG_41,
    FLD_PREULTRA_TH_LOW | FLD_PREULTRA_TH_HIGH);
    }
    void mtk_merge_config(struct device *dev, unsigned int w,
    unsigned int h, unsigned int vrefresh,
    unsigned int bpc, struct cmdq_pkt *cmdq_pkt)
    {
    mtk_merge_advance_config(dev, w, 0, h, vrefresh, bpc, cmdq_pkt);
    }
    void mtk_merge_advance_config(struct device *dev, unsigned int l_w, unsigned int r_w,
    unsigned int h, unsigned int vrefresh, unsigned int bpc,
    struct cmdq_pkt *cmdq_pkt)
    {
    struct mtk_disp_merge *priv = dev_get_drvdata(dev);
    let mut mode: c_uint = CFG_10_10_1PI_2PO_BUF_MODE;
    if (!h || !l_w) {
    dev_err(dev, "%s: input width(%d) or height(%d) is invalid\n", __func__, l_w, h);
    return;
    }
    if (priv.fifo_en) {
    mtk_merge_fifo_setting(priv, cmdq_pkt);
    mode = CFG_10_10_2PI_2PO_BUF_MODE;
    }
    if (r_w)
    mode = CFG_11_10_1PI_2PO_MERGE;
    mtk_ddp_write(cmdq_pkt, h << 16 | l_w, &priv.cmdq_reg, priv.regs,
    DISP_REG_MERGE_CFG_0);
    mtk_ddp_write(cmdq_pkt, h << 16 | r_w, &priv.cmdq_reg, priv.regs,
    DISP_REG_MERGE_CFG_1);
    mtk_ddp_write(cmdq_pkt, h << 16 | (l_w + r_w), &priv.cmdq_reg, priv.regs,
    DISP_REG_MERGE_CFG_4);
//
// DISP_REG_MERGE_CFG_24 is merge SRAM0 w/h
// DISP_REG_MERGE_CFG_25 is merge SRAM1 w/h.
// If r_w > 0, the merge is in merge mode (input0 and input1 merge together),
// the input0 goes to SRAM0, and input1 goes to SRAM1.
// If r_w = 0, the merge is in buffer mode, the input goes through SRAM0 and
// then to SRAM1. Both SRAM0 and SRAM1 are set to the same size.
//
    mtk_ddp_write(cmdq_pkt, h << 16 | l_w, &priv.cmdq_reg, priv.regs,
    DISP_REG_MERGE_CFG_24);
    if (r_w)
    mtk_ddp_write(cmdq_pkt, h << 16 | r_w, &priv.cmdq_reg, priv.regs,
    DISP_REG_MERGE_CFG_25);
    else
    mtk_ddp_write(cmdq_pkt, h << 16 | l_w, &priv.cmdq_reg, priv.regs,
    DISP_REG_MERGE_CFG_25);
//
// DISP_REG_MERGE_CFG_26 and DISP_REG_MERGE_CFG_27 is only used in LR merge.
// Only take effect when the merge is setting to merge mode.
//
    mtk_ddp_write(cmdq_pkt, h << 16 | l_w, &priv.cmdq_reg, priv.regs,
    DISP_REG_MERGE_CFG_26);
    mtk_ddp_write(cmdq_pkt, h << 16 | r_w, &priv.cmdq_reg, priv.regs,
    DISP_REG_MERGE_CFG_27);
    mtk_ddp_write_mask(cmdq_pkt, SWAP_MODE, &priv.cmdq_reg, priv.regs,
    DISP_REG_MERGE_CFG_10, FLD_SWAP_MODE);
    mtk_ddp_write_mask(cmdq_pkt, mode, &priv.cmdq_reg, priv.regs,
    DISP_REG_MERGE_CFG_12, FLD_CFG_MERGE_MODE);
    }
#[no_mangle]
pub unsafe extern "C" fn mtk_merge_clk_enable(dev: *mut device) -> c_int {
    int mtk_merge_clk_enable(struct device *dev)
    {
    let mut ret: c_int = 0;
    struct mtk_disp_merge *priv = dev_get_drvdata(dev);
    ret = clk_prepare_enable(priv.clk);
    if (ret) {
    dev_err(dev, "merge clk prepare enable failed\n");
    return ret;
    }
    ret = clk_prepare_enable(priv.async_clk);
    if (ret) {
// should clean up the state of priv->clk
    clk_disable_unprepare(priv.clk);
    dev_err(dev, "async clk prepare enable failed\n");
    return ret;
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn mtk_merge_clk_disable(dev: *mut device) {
    void mtk_merge_clk_disable(struct device *dev)
    {
    struct mtk_disp_merge *priv = dev_get_drvdata(dev);
    clk_disable_unprepare(priv.async_clk);
    clk_disable_unprepare(priv.clk);
    }
    enum drm_mode_status mtk_merge_mode_valid(struct device *dev,
    const struct drm_display_mode *mode)
    {
    struct mtk_disp_merge *priv = dev_get_drvdata(dev);
    unsigned long rate;
    rate = clk_get_rate(priv.clk);
// Convert to KHz and round the number
    rate = (rate + 500) / 1000;
    if (rate && mode.clock > rate) {
    dev_dbg(dev, "invalid clock: %d (>%lu)\n", mode.clock, rate);
    return MODE_CLOCK_HIGH;
    }
//
// Measure the bandwidth requirement of hardware prefetch (per frame)
//
// let N = prefetch buffer size in lines
// (ex. N=3, then prefetch buffer size = 3 lines)
//
// prefetch size = htotal * N (pixels)
// time per line = 1 / fps / vtotal (seconds)
// duration      = vbp * time per line
// = vbp / fps / vtotal
//
// data rate = prefetch size / duration
// = htotal * N / (vbp / fps / vtotal)
// = htotal * vtotal * fps * N / vbp
// = clk * N / vbp (pixels per second)
//
// Say 4K60 (CEA-861) is the maximum mode supported by the SoC
// data rate = 594000K * N / 72 = 8250 (standard)
// (remove K * N due to the same unit)
//
// For 2560x1440@144 (clk=583600K, vbp=17):
// data rate = 583600 / 17 ~= 34329 > 8250 (NG)
//
// For 2560x1440@120 (clk=497760K, vbp=77):
// data rate = 497760 / 77 ~= 6464 < 8250 (OK)
//
// A non-standard 4K60 timing (clk=521280K, vbp=54)
// data rate = 521280 / 54 ~= 9653 > 8250 (NG)
//
// Bandwidth requirement of hardware prefetch increases significantly
// when the VBP decreases (more than 4x in this example).
//
// The proposed formula is only one way to estimate whether our SoC
// supports the mode setting. The basic idea behind it is just to check
// if the data rate requirement is too high (directly proportional to
// pixel clock, inversely proportional to vbp). Please adjust the
// function if it doesn't fit your situation in the future.
//
    rate = mode.clock / (mode.vtotal - mode.vsync_end);
    if (rate > 8250) {
    dev_dbg(dev, "invalid rate: %lu (>8250): " DRM_MODE_FMT "\n",
    rate, DRM_MODE_ARG(mode));
    return MODE_BAD;
    }
    return MODE_OK;
    }
    static int mtk_disp_merge_bind(struct device *dev, struct device *master,
    void *data)
    {
    return 0;
    }
    static void mtk_disp_merge_unbind(struct device *dev, struct device *master,
    void *data)
    {
    }
    static const struct component_ops mtk_disp_merge_component_ops = {
    .bind	= mtk_disp_merge_bind,
    .unbind = mtk_disp_merge_unbind,
    };
#[no_mangle]
unsafe extern "C" fn mtk_disp_merge_probe(pdev: *mut platform_device) -> c_int {
    static int mtk_disp_merge_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct mtk_disp_merge *priv;
    int ret;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(priv.regs))
    return dev_err_probe(dev, PTR_ERR(priv.regs),
    "failed to ioremap merge\n");
    priv.clk = devm_clk_get(dev, core::ptr::null_mut());
    if (IS_ERR(priv.clk))
    return dev_err_probe(dev, PTR_ERR(priv.clk),
    "failed to get merge clk\n");
    priv.async_clk = devm_clk_get_optional(dev, "merge_async");
    if (IS_ERR(priv.async_clk))
    return dev_err_probe(dev, PTR_ERR(priv.async_clk),
    "failed to get merge async clock\n");
    if (priv.async_clk) {
    priv.reset_ctl = devm_reset_control_get_optional_exclusive(dev, core::ptr::null_mut());
    if (IS_ERR(priv.reset_ctl))
    return PTR_ERR(priv.reset_ctl);
    }

    ret = cmdq_dev_get_client_reg(dev, &priv.cmdq_reg, 0);
    if (ret)
    dev_dbg(dev, "get mediatek,gce-client-reg fail!\n");

    priv.fifo_en = of_property_read_bool(dev.of_node,
    "mediatek,merge-fifo-en");
    priv.mute_support = of_property_read_bool(dev.of_node,
    "mediatek,merge-mute");
    platform_set_drvdata(pdev, priv);
    ret = component_add(dev, &mtk_disp_merge_component_ops);
    if (ret != 0)
    return dev_err_probe(dev, ret, "Failed to add component\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mtk_disp_merge_remove(pdev: *mut platform_device) {
    static void mtk_disp_merge_remove(struct platform_device *pdev)
    {
    component_del(&pdev.dev, &mtk_disp_merge_component_ops);
    }
    static const struct of_device_id mtk_disp_merge_driver_dt_match[] = {
    { .compatible = "mediatek,mt8195-disp-merge", },
    {},
    };
    MODULE_DEVICE_TABLE(of, mtk_disp_merge_driver_dt_match);
    struct platform_driver mtk_disp_merge_driver = {
    .probe = mtk_disp_merge_probe,
    .remove = mtk_disp_merge_remove,
    .driver = {
    .name = "mediatek-disp-merge",
    .of_match_table = mtk_disp_merge_driver_dt_match,
    },
    };
