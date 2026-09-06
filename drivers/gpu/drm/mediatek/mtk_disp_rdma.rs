//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/mediatek/mtk_disp_rdma.c
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

pub const DISP_REG_RDMA_INT_ENABLE: c_uint = 0x0000;
pub const DISP_REG_RDMA_INT_STATUS: c_uint = 0x0004;

pub const DISP_REG_RDMA_GLOBAL_CON: c_uint = 0x0010;

pub const DISP_REG_RDMA_SIZE_CON_0: c_uint = 0x0014;

pub const DISP_REG_RDMA_SIZE_CON_1: c_uint = 0x0018;
pub const DISP_REG_RDMA_TARGET_LINE: c_uint = 0x001c;
pub const DISP_RDMA_MEM_CON: c_uint = 0x0024;

pub const DISP_RDMA_MEM_SRC_PITCH: c_uint = 0x002c;
pub const DISP_RDMA_MEM_GMC_SETTING_0: c_uint = 0x0030;
pub const DISP_REG_RDMA_FIFO_CON: c_uint = 0x0040;

pub const DISP_RDMA_MEM_START_ADDR: c_uint = 0x0f00;
pub const RDMA_MEM_GMC: c_uint = 0x40402020;
    static const u32 mt8173_formats[] = {
    DRM_FORMAT_XRGB8888,
    DRM_FORMAT_ARGB8888,
    DRM_FORMAT_BGRX8888,
    DRM_FORMAT_BGRA8888,
    DRM_FORMAT_ABGR8888,
    DRM_FORMAT_XBGR8888,
    DRM_FORMAT_RGB888,
    DRM_FORMAT_BGR888,
    DRM_FORMAT_RGB565,
    DRM_FORMAT_UYVY,
    DRM_FORMAT_YUYV,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_disp_rdma_data {
    pub fifo_size: c_uint,
    pub formats: *const u32,
    pub num_formats: usize,
}

//
// struct mtk_disp_rdma - DISP_RDMA driver structure
// @data: local driver data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_disp_rdma {
    pub clk: *mut clk,
    pub regs: *mut void __iomem,
    pub cmdq_reg: cmdq_client_reg,
    pub data: *const mtk_disp_rdma_data,
    pub data): *mut *mut void (vblank_cb)(void,
    pub vblank_cb_data: *mut c_void,
    pub fifo_size: u32,
}

#[no_mangle]
unsafe extern "C" fn mtk_disp_rdma_irq_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t mtk_disp_rdma_irq_handler(int irq, void *dev_id)
    {
    struct mtk_disp_rdma *priv = dev_id;
// Clear frame completion interrupt
    writel(0x0, priv.regs + DISP_REG_RDMA_INT_STATUS);
    if (!priv.vblank_cb)
    return IRQ_NONE;
    priv.vblank_cb(priv.vblank_cb_data);
    return IRQ_HANDLED;
    }
    static void rdma_update_bits(struct device *dev, unsigned int reg,
    unsigned int mask, unsigned int val)
    {
    struct mtk_disp_rdma *rdma = dev_get_drvdata(dev);
    let mut tmp: c_uint = readl(rdma.regs + reg);
    tmp = (tmp & ~mask) | (val & mask);
    writel(tmp, rdma.regs + reg);
    }
    void mtk_rdma_register_vblank_cb(struct device *dev,
    void (*vblank_cb)(void *),
    void *vblank_cb_data)
    {
    struct mtk_disp_rdma *rdma = dev_get_drvdata(dev);
    rdma.vblank_cb = vblank_cb;
    rdma.vblank_cb_data = vblank_cb_data;
    }
#[no_mangle]
pub unsafe extern "C" fn mtk_rdma_unregister_vblank_cb(dev: *mut device) {
    void mtk_rdma_unregister_vblank_cb(struct device *dev)
    {
    struct mtk_disp_rdma *rdma = dev_get_drvdata(dev);
    rdma.vblank_cb = core::ptr::null_mut();
    rdma.vblank_cb_data = core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn mtk_rdma_enable_vblank(dev: *mut device) {
    void mtk_rdma_enable_vblank(struct device *dev)
    {
    rdma_update_bits(dev, DISP_REG_RDMA_INT_ENABLE, RDMA_FRAME_END_INT,
    RDMA_FRAME_END_INT);
    }
#[no_mangle]
pub unsafe extern "C" fn mtk_rdma_disable_vblank(dev: *mut device) {
    void mtk_rdma_disable_vblank(struct device *dev)
    {
    rdma_update_bits(dev, DISP_REG_RDMA_INT_ENABLE, RDMA_FRAME_END_INT, 0);
    }
    const u32 *mtk_rdma_get_formats(struct device *dev)
    {
    struct mtk_disp_rdma *rdma = dev_get_drvdata(dev);
    return rdma.data.formats;
    }
#[no_mangle]
pub unsafe extern "C" fn mtk_rdma_get_num_formats(dev: *mut device) -> usize {
    size_t mtk_rdma_get_num_formats(struct device *dev)
    {
    struct mtk_disp_rdma *rdma = dev_get_drvdata(dev);
    return rdma.data.num_formats;
    }
#[no_mangle]
pub unsafe extern "C" fn mtk_rdma_clk_enable(dev: *mut device) -> c_int {
    int mtk_rdma_clk_enable(struct device *dev)
    {
    struct mtk_disp_rdma *rdma = dev_get_drvdata(dev);
    return clk_prepare_enable(rdma.clk);
    }
#[no_mangle]
pub unsafe extern "C" fn mtk_rdma_clk_disable(dev: *mut device) {
    void mtk_rdma_clk_disable(struct device *dev)
    {
    struct mtk_disp_rdma *rdma = dev_get_drvdata(dev);
    clk_disable_unprepare(rdma.clk);
    }
#[no_mangle]
pub unsafe extern "C" fn mtk_rdma_start(dev: *mut device) {
    void mtk_rdma_start(struct device *dev)
    {
    rdma_update_bits(dev, DISP_REG_RDMA_GLOBAL_CON, RDMA_ENGINE_EN,
    RDMA_ENGINE_EN);
    }
#[no_mangle]
pub unsafe extern "C" fn mtk_rdma_stop(dev: *mut device) {
    void mtk_rdma_stop(struct device *dev)
    {
    rdma_update_bits(dev, DISP_REG_RDMA_GLOBAL_CON, RDMA_ENGINE_EN, 0);
    }
    void mtk_rdma_config(struct device *dev, unsigned int width,
    unsigned int height, unsigned int vrefresh,
    unsigned int bpc, struct cmdq_pkt *cmdq_pkt)
    {
    unsigned int threshold;
    unsigned int reg;
    struct mtk_disp_rdma *rdma = dev_get_drvdata(dev);
    u32 rdma_fifo_size;
    mtk_ddp_write_mask(cmdq_pkt, width, &rdma.cmdq_reg, rdma.regs,
    DISP_REG_RDMA_SIZE_CON_0, 0xfff);
    mtk_ddp_write_mask(cmdq_pkt, height, &rdma.cmdq_reg, rdma.regs,
    DISP_REG_RDMA_SIZE_CON_1, 0xfffff);
    if (rdma.fifo_size)
    rdma_fifo_size = rdma.fifo_size;
    else
    rdma_fifo_size = RDMA_FIFO_SIZE(rdma);
//
// Enable FIFO underflow since DSI and DPI can't be blocked.
// Keep the FIFO pseudo size reset default of 8 KiB. Set the
// output threshold to 70% of max fifo size to make sure the
// threhold will not overflow
//
    threshold = rdma_fifo_size * 7 / 10;
    reg = RDMA_FIFO_UNDERFLOW_EN |
    RDMA_FIFO_PSEUDO_SIZE(rdma_fifo_size) |
    RDMA_OUTPUT_VALID_FIFO_THRESHOLD(threshold);
    mtk_ddp_write(cmdq_pkt, reg, &rdma.cmdq_reg, rdma.regs, DISP_REG_RDMA_FIFO_CON);
    }
    static unsigned int rdma_fmt_convert(struct mtk_disp_rdma *rdma,
    unsigned int fmt)
    {
// The return value in switch "MEM_MODE_INPUT_FORMAT_XXX"
// is defined in mediatek HW data sheet.
// The alphabet order in XXX is no relation to data
// arrangement in memory.
//
    switch (fmt) {
    default:
    case DRM_FORMAT_RGB565:
    return MEM_MODE_INPUT_FORMAT_RGB565;
    case DRM_FORMAT_BGR565:
    return MEM_MODE_INPUT_FORMAT_RGB565 | MEM_MODE_INPUT_SWAP;
    case DRM_FORMAT_RGB888:
    return MEM_MODE_INPUT_FORMAT_RGB888;
    case DRM_FORMAT_BGR888:
    return MEM_MODE_INPUT_FORMAT_RGB888 | MEM_MODE_INPUT_SWAP;
    case DRM_FORMAT_RGBX8888:
    case DRM_FORMAT_RGBA8888:
    return MEM_MODE_INPUT_FORMAT_ARGB8888;
    case DRM_FORMAT_BGRX8888:
    case DRM_FORMAT_BGRA8888:
    return MEM_MODE_INPUT_FORMAT_ARGB8888 | MEM_MODE_INPUT_SWAP;
    case DRM_FORMAT_XRGB8888:
    case DRM_FORMAT_ARGB8888:
    return MEM_MODE_INPUT_FORMAT_RGBA8888;
    case DRM_FORMAT_XBGR8888:
    case DRM_FORMAT_ABGR8888:
    return MEM_MODE_INPUT_FORMAT_RGBA8888 | MEM_MODE_INPUT_SWAP;
    case DRM_FORMAT_UYVY:
    return MEM_MODE_INPUT_FORMAT_UYVY;
    case DRM_FORMAT_YUYV:
    return MEM_MODE_INPUT_FORMAT_YUYV;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn mtk_rdma_layer_nr(dev: *mut device) -> c_uint {
    unsigned int mtk_rdma_layer_nr(struct device *dev)
    {
    return 1;
    }
    void mtk_rdma_layer_config(struct device *dev, unsigned int idx,
    struct mtk_plane_state *state,
    struct cmdq_pkt *cmdq_pkt)
    {
    struct mtk_disp_rdma *rdma = dev_get_drvdata(dev);
    struct mtk_plane_pending_state *pending = &state.pending;
    let mut addr: c_uint = pending.addr;
    let mut pitch: c_uint = pending.pitch & 0xffff;
    let mut fmt: c_uint = pending.format;
    unsigned int con;
    con = rdma_fmt_convert(rdma, fmt);
    mtk_ddp_write_relaxed(cmdq_pkt, con, &rdma.cmdq_reg, rdma.regs, DISP_RDMA_MEM_CON);
    if (fmt == DRM_FORMAT_UYVY || fmt == DRM_FORMAT_YUYV) {
    mtk_ddp_write_mask(cmdq_pkt, RDMA_MATRIX_ENABLE, &rdma.cmdq_reg, rdma.regs,
    DISP_REG_RDMA_SIZE_CON_0,
    RDMA_MATRIX_ENABLE);
    mtk_ddp_write_mask(cmdq_pkt, RDMA_MATRIX_INT_MTX_BT601_to_RGB,
    &rdma.cmdq_reg, rdma.regs, DISP_REG_RDMA_SIZE_CON_0,
    RDMA_MATRIX_INT_MTX_SEL);
    } else {
    mtk_ddp_write_mask(cmdq_pkt, 0, &rdma.cmdq_reg, rdma.regs,
    DISP_REG_RDMA_SIZE_CON_0,
    RDMA_MATRIX_ENABLE);
    }
    mtk_ddp_write_relaxed(cmdq_pkt, addr, &rdma.cmdq_reg, rdma.regs,
    DISP_RDMA_MEM_START_ADDR);
    mtk_ddp_write_relaxed(cmdq_pkt, pitch, &rdma.cmdq_reg, rdma.regs,
    DISP_RDMA_MEM_SRC_PITCH);
    mtk_ddp_write(cmdq_pkt, RDMA_MEM_GMC, &rdma.cmdq_reg, rdma.regs,
    DISP_RDMA_MEM_GMC_SETTING_0);
    mtk_ddp_write_mask(cmdq_pkt, RDMA_MODE_MEMORY, &rdma.cmdq_reg, rdma.regs,
    DISP_REG_RDMA_GLOBAL_CON, RDMA_MODE_MEMORY);
    }
    static int mtk_disp_rdma_bind(struct device *dev, struct device *master,
    void *data)
    {
    return 0;
    }
    static void mtk_disp_rdma_unbind(struct device *dev, struct device *master,
    void *data)
    {
    }
    static const struct component_ops mtk_disp_rdma_component_ops = {
    .bind	= mtk_disp_rdma_bind,
    .unbind = mtk_disp_rdma_unbind,
    };
#[no_mangle]
unsafe extern "C" fn mtk_disp_rdma_probe(pdev: *mut platform_device) -> c_int {
    static int mtk_disp_rdma_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct mtk_disp_rdma *priv;
    int irq;
    int ret;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    priv.clk = devm_clk_get(dev, core::ptr::null_mut());
    if (IS_ERR(priv.clk))
    return dev_err_probe(dev, PTR_ERR(priv.clk),
    "failed to get rdma clk\n");
    priv.regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(priv.regs))
    return dev_err_probe(dev, PTR_ERR(priv.regs),
    "failed to ioremap rdma\n");

    ret = cmdq_dev_get_client_reg(dev, &priv.cmdq_reg, 0);
    if (ret)
    dev_dbg(dev, "get mediatek,gce-client-reg fail!\n");

    ret = of_property_read_u32(dev.of_node,
    "mediatek,rdma-fifo-size",
    &priv.fifo_size);
    if (ret && (ret != -EINVAL))
    return dev_err_probe(dev, ret, "Failed to get rdma fifo size\n");
// Disable and clear pending interrupts
    writel(0x0, priv.regs + DISP_REG_RDMA_INT_ENABLE);
    writel(0x0, priv.regs + DISP_REG_RDMA_INT_STATUS);
    ret = devm_request_irq(dev, irq, mtk_disp_rdma_irq_handler,
    IRQF_TRIGGER_NONE, dev_name(dev), priv);
    if (ret < 0)
    return dev_err_probe(dev, ret, "Failed to request irq %d\n", irq);
    priv.data = of_device_get_match_data(dev);
    platform_set_drvdata(pdev, priv);
    pm_runtime_enable(dev);
    ret = component_add(dev, &mtk_disp_rdma_component_ops);
    if (ret) {
    pm_runtime_disable(dev);
    return dev_err_probe(dev, ret, "Failed to add component\n");
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mtk_disp_rdma_remove(pdev: *mut platform_device) {
    static void mtk_disp_rdma_remove(struct platform_device *pdev)
    {
    component_del(&pdev.dev, &mtk_disp_rdma_component_ops);
    pm_runtime_disable(&pdev.dev);
    }
    static const struct mtk_disp_rdma_data mt2701_rdma_driver_data = {
    .fifo_size = SZ_4K,
    .formats = mt8173_formats,
    .num_formats = ARRAY_SIZE(mt8173_formats),
    };
    static const struct mtk_disp_rdma_data mt8173_rdma_driver_data = {
    .fifo_size = SZ_8K,
    .formats = mt8173_formats,
    .num_formats = ARRAY_SIZE(mt8173_formats),
    };
    static const struct mtk_disp_rdma_data mt8183_rdma_driver_data = {
    .fifo_size = 5 * SZ_1K,
    .formats = mt8173_formats,
    .num_formats = ARRAY_SIZE(mt8173_formats),
    };
    static const struct mtk_disp_rdma_data mt8195_rdma_driver_data = {
    .fifo_size = 1920,
    .formats = mt8173_formats,
    .num_formats = ARRAY_SIZE(mt8173_formats),
    };
    static const struct of_device_id mtk_disp_rdma_driver_dt_match[] = {
    { .compatible = "mediatek,mt2701-disp-rdma",
    .data = &mt2701_rdma_driver_data},
    { .compatible = "mediatek,mt8173-disp-rdma",
    .data = &mt8173_rdma_driver_data},
    { .compatible = "mediatek,mt8183-disp-rdma",
    .data = &mt8183_rdma_driver_data},
    { .compatible = "mediatek,mt8195-disp-rdma",
    .data = &mt8195_rdma_driver_data},
    {},
    };
    MODULE_DEVICE_TABLE(of, mtk_disp_rdma_driver_dt_match);
    struct platform_driver mtk_disp_rdma_driver = {
    .probe		= mtk_disp_rdma_probe,
    .remove		= mtk_disp_rdma_remove,
    .driver		= {
    .name	= "mediatek-disp-rdma",
    .of_match_table = mtk_disp_rdma_driver_dt_match,
    },
    };
