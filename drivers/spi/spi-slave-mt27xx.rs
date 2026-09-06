//! Automatically rewritten from C to Rust
//! Source: drivers/spi/spi-slave-mt27xx.c
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


// SPDX-License-Identifier: GPL-2.0+
// Copyright (c) 2018 MediaTek Inc.

pub const SPIS_IRQ_EN_REG: c_uint = 0x0;
pub const SPIS_IRQ_CLR_REG: c_uint = 0x4;
pub const SPIS_IRQ_ST_REG: c_uint = 0x8;
pub const SPIS_IRQ_MASK_REG: c_uint = 0xc;
pub const SPIS_CFG_REG: c_uint = 0x10;
pub const SPIS_RX_DATA_REG: c_uint = 0x14;
pub const SPIS_TX_DATA_REG: c_uint = 0x18;
pub const SPIS_RX_DST_REG: c_uint = 0x1c;
pub const SPIS_TX_SRC_REG: c_uint = 0x20;
pub const SPIS_DMA_CFG_REG: c_uint = 0x30;
pub const SPIS_SOFT_RST_REG: c_uint = 0x40;
// SPIS_IRQ_EN_REG

// SPIS_IRQ_ST_REG

// SPIS_IRQ_MASK_REG

// SPIS_CFG_REG

// SPIS_DMA_CFG_REG

pub const TX_DMA_LEN: c_uint = 0xfffff;
// SPIS_SOFT_RST_REG

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_spi_slave {
    pub dev: *mut device,
    pub base: *mut void __iomem,
    pub spi_clk: *mut clk,
    pub xfer_done: completion,
    pub cur_transfer: *mut spi_transfer,
    pub target_aborted: bool,
    pub dev_comp: *const mtk_spi_compatible,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_spi_compatible {
    pub max_fifo_size: u32,
    pub must_rx: bool,
}

    static const struct mtk_spi_compatible mt2712_compat = {
    .max_fifo_size = 512,
    };
    static const struct mtk_spi_compatible mt8195_compat = {
    .max_fifo_size = 128,
    .must_rx = true,
    };
    static const struct of_device_id mtk_spi_slave_of_match[] = {
    { .compatible = "mediatek,mt2712-spi-slave",
    .data = (void *)&mt2712_compat,},
    { .compatible = "mediatek,mt8195-spi-slave",
    .data = (void *)&mt8195_compat,},
    {}
    };
    MODULE_DEVICE_TABLE(of, mtk_spi_slave_of_match);
#[no_mangle]
unsafe extern "C" fn mtk_spi_slave_disable_dma(mdata: *mut mtk_spi_slave) {
    static void mtk_spi_slave_disable_dma(struct mtk_spi_slave *mdata)
    {
    u32 reg_val;
    reg_val = readl(mdata.base + SPIS_DMA_CFG_REG);
    reg_val &= ~RX_DMA_EN;
    reg_val &= ~TX_DMA_EN;
    writel(reg_val, mdata.base + SPIS_DMA_CFG_REG);
    }
#[no_mangle]
unsafe extern "C" fn mtk_spi_slave_disable_xfer(mdata: *mut mtk_spi_slave) {
    static void mtk_spi_slave_disable_xfer(struct mtk_spi_slave *mdata)
    {
    u32 reg_val;
    reg_val = readl(mdata.base + SPIS_CFG_REG);
    reg_val &= ~SPIS_TX_EN;
    reg_val &= ~SPIS_RX_EN;
    writel(reg_val, mdata.base + SPIS_CFG_REG);
    }
#[no_mangle]
unsafe extern "C" fn mtk_spi_slave_wait_for_completion(mdata: *mut mtk_spi_slave) -> c_int {
    static int mtk_spi_slave_wait_for_completion(struct mtk_spi_slave *mdata)
    {
    if (wait_for_completion_interruptible(&mdata.xfer_done) ||
    mdata.target_aborted) {
    dev_err(mdata.dev, "interrupted\n");
    return -EINTR;
    }
    return 0;
    }
    static int mtk_spi_slave_prepare_message(struct spi_controller *ctlr,
    struct spi_message *msg)
    {
    struct mtk_spi_slave *mdata = spi_controller_get_devdata(ctlr);
    struct spi_device *spi = msg.spi;
    bool cpha, cpol;
    u32 reg_val;
    cpha = spi.mode & SPI_CPHA ? 1 : 0;
    cpol = spi.mode & SPI_CPOL ? 1 : 0;
    reg_val = readl(mdata.base + SPIS_CFG_REG);
    if (cpha)
    reg_val |= SPIS_CPHA;
    else
    reg_val &= ~SPIS_CPHA;
    if (cpol)
    reg_val |= SPIS_CPOL;
    else
    reg_val &= ~SPIS_CPOL;
    if (spi.mode & SPI_LSB_FIRST)
    reg_val &= ~(SPIS_TXMSBF | SPIS_RXMSBF);
    else
    reg_val |= SPIS_TXMSBF | SPIS_RXMSBF;
    reg_val &= ~SPIS_TX_ENDIAN;
    reg_val &= ~SPIS_RX_ENDIAN;
    writel(reg_val, mdata.base + SPIS_CFG_REG);
    return 0;
    }
    static int mtk_spi_slave_fifo_transfer(struct spi_controller *ctlr,
    struct spi_device *spi,
    struct spi_transfer *xfer)
    {
    struct mtk_spi_slave *mdata = spi_controller_get_devdata(ctlr);
    int reg_val, cnt, remainder, ret;
    writel(SPIS_SOFT_RST, mdata.base + SPIS_SOFT_RST_REG);
    reg_val = readl(mdata.base + SPIS_CFG_REG);
    if (xfer.rx_buf)
    reg_val |= SPIS_RX_EN;
    if (xfer.tx_buf)
    reg_val |= SPIS_TX_EN;
    writel(reg_val, mdata.base + SPIS_CFG_REG);
    cnt = xfer.len / 4;
    if (xfer.tx_buf)
    iowrite32_rep(mdata.base + SPIS_TX_DATA_REG,
    xfer.tx_buf, cnt);
    remainder = xfer.len % 4;
    if (xfer.tx_buf && remainder > 0) {
    reg_val = 0;
    memcpy(&reg_val, xfer.tx_buf + cnt * 4, remainder);
    writel(reg_val, mdata.base + SPIS_TX_DATA_REG);
    }
    ret = mtk_spi_slave_wait_for_completion(mdata);
    if (ret) {
    mtk_spi_slave_disable_xfer(mdata);
    writel(SPIS_SOFT_RST, mdata.base + SPIS_SOFT_RST_REG);
    }
    return ret;
    }
    static int mtk_spi_slave_dma_transfer(struct spi_controller *ctlr,
    struct spi_device *spi,
    struct spi_transfer *xfer)
    {
    struct mtk_spi_slave *mdata = spi_controller_get_devdata(ctlr);
    struct device *dev = mdata.dev;
    int reg_val, ret;
    writel(SPIS_SOFT_RST, mdata.base + SPIS_SOFT_RST_REG);
    if (xfer.tx_buf) {
// tx_buf is a const void* where we need a void * for
// the dma mapping
//
    void *nonconst_tx = (void *)xfer.tx_buf;
    xfer.tx_dma = dma_map_single(dev, nonconst_tx,
    xfer.len, DMA_TO_DEVICE);
    if (dma_mapping_error(dev, xfer.tx_dma)) {
    ret = -ENOMEM;
    goto disable_transfer;
    }
    }
    if (xfer.rx_buf) {
    xfer.rx_dma = dma_map_single(dev, xfer.rx_buf,
    xfer.len, DMA_FROM_DEVICE);
    if (dma_mapping_error(dev, xfer.rx_dma)) {
    ret = -ENOMEM;
    goto unmap_txdma;
    }
    }
    writel(xfer.tx_dma, mdata.base + SPIS_TX_SRC_REG);
    writel(xfer.rx_dma, mdata.base + SPIS_RX_DST_REG);
    writel(SPIS_DMA_ADDR_EN, mdata.base + SPIS_SOFT_RST_REG);
// enable config reg tx rx_enable
    reg_val = readl(mdata.base + SPIS_CFG_REG);
    if (xfer.tx_buf)
    reg_val |= SPIS_TX_EN;
    if (xfer.rx_buf)
    reg_val |= SPIS_RX_EN;
    writel(reg_val, mdata.base + SPIS_CFG_REG);
// config dma
    reg_val = 0;
    reg_val |= (xfer.len - 1) & TX_DMA_LEN;
    writel(reg_val, mdata.base + SPIS_DMA_CFG_REG);
    reg_val = readl(mdata.base + SPIS_DMA_CFG_REG);
    if (xfer.tx_buf)
    reg_val |= TX_DMA_EN;
    if (xfer.rx_buf)
    reg_val |= RX_DMA_EN;
    reg_val |= TX_DMA_TRIG_EN;
    writel(reg_val, mdata.base + SPIS_DMA_CFG_REG);
    ret = mtk_spi_slave_wait_for_completion(mdata);
    if (ret)
    goto unmap_rxdma;
    return 0;
    unmap_rxdma:
    if (xfer.rx_buf)
    dma_unmap_single(dev, xfer.rx_dma,
    xfer.len, DMA_FROM_DEVICE);
    unmap_txdma:
    if (xfer.tx_buf)
    dma_unmap_single(dev, xfer.tx_dma,
    xfer.len, DMA_TO_DEVICE);
    disable_transfer:
    mtk_spi_slave_disable_dma(mdata);
    mtk_spi_slave_disable_xfer(mdata);
    writel(SPIS_SOFT_RST, mdata.base + SPIS_SOFT_RST_REG);
    return ret;
    }
    static int mtk_spi_slave_transfer_one(struct spi_controller *ctlr,
    struct spi_device *spi,
    struct spi_transfer *xfer)
    {
    struct mtk_spi_slave *mdata = spi_controller_get_devdata(ctlr);
    reinit_completion(&mdata.xfer_done);
    mdata.target_aborted = false;
    mdata.cur_transfer = xfer;
    if (xfer.len > mdata.dev_comp.max_fifo_size)
    return mtk_spi_slave_dma_transfer(ctlr, spi, xfer);
    else
    return mtk_spi_slave_fifo_transfer(ctlr, spi, xfer);
    }
#[no_mangle]
unsafe extern "C" fn mtk_spi_slave_setup(spi: *mut spi_device) -> c_int {
    static int mtk_spi_slave_setup(struct spi_device *spi)
    {
    struct mtk_spi_slave *mdata = spi_controller_get_devdata(spi.controller);
    u32 reg_val;
    reg_val = DMA_DONE_EN | DATA_DONE_EN |
    RSTA_DONE_EN | CMD_INVALID_EN;
    writel(reg_val, mdata.base + SPIS_IRQ_EN_REG);
    reg_val = DMA_DONE_MASK | DATA_DONE_MASK |
    RSTA_DONE_MASK | CMD_INVALID_MASK;
    writel(reg_val, mdata.base + SPIS_IRQ_MASK_REG);
    mtk_spi_slave_disable_dma(mdata);
    mtk_spi_slave_disable_xfer(mdata);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mtk_target_abort(ctlr: *mut spi_controller) -> c_int {
    static int mtk_target_abort(struct spi_controller *ctlr)
    {
    struct mtk_spi_slave *mdata = spi_controller_get_devdata(ctlr);
    mdata.target_aborted = true;
    complete(&mdata.xfer_done);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mtk_spi_slave_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t mtk_spi_slave_interrupt(int irq, void *dev_id)
    {
    struct spi_controller *ctlr = dev_id;
    struct mtk_spi_slave *mdata = spi_controller_get_devdata(ctlr);
    struct spi_transfer *trans = mdata.cur_transfer;
    u32 int_status, reg_val, cnt, remainder;
    int_status = readl(mdata.base + SPIS_IRQ_ST_REG);
    writel(int_status, mdata.base + SPIS_IRQ_CLR_REG);
    if (!trans)
    return IRQ_NONE;
    if ((int_status & DMA_DONE_ST) &&
    ((int_status & DATA_DONE_ST) ||
    (int_status & RSTA_DONE_ST))) {
    writel(SPIS_SOFT_RST, mdata.base + SPIS_SOFT_RST_REG);
    if (trans.tx_buf)
    dma_unmap_single(mdata.dev, trans.tx_dma,
    trans.len, DMA_TO_DEVICE);
    if (trans.rx_buf)
    dma_unmap_single(mdata.dev, trans.rx_dma,
    trans.len, DMA_FROM_DEVICE);
    mtk_spi_slave_disable_dma(mdata);
    mtk_spi_slave_disable_xfer(mdata);
    }
    if ((!(int_status & DMA_DONE_ST)) &&
    ((int_status & DATA_DONE_ST) ||
    (int_status & RSTA_DONE_ST))) {
    cnt = trans.len / 4;
    if (trans.rx_buf)
    ioread32_rep(mdata.base + SPIS_RX_DATA_REG,
    trans.rx_buf, cnt);
    remainder = trans.len % 4;
    if (trans.rx_buf && remainder > 0) {
    reg_val = readl(mdata.base + SPIS_RX_DATA_REG);
    memcpy(trans.rx_buf + (cnt * 4),
    &reg_val, remainder);
    }
    mtk_spi_slave_disable_xfer(mdata);
    }
    if (int_status & CMD_INVALID_ST) {
    dev_warn(&ctlr.dev, "cmd invalid\n");
    return IRQ_NONE;
    }
    mdata.cur_transfer = core::ptr::null_mut();
    complete(&mdata.xfer_done);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn mtk_spi_slave_probe(pdev: *mut platform_device) -> c_int {
    static int mtk_spi_slave_probe(struct platform_device *pdev)
    {
    struct spi_controller *ctlr;
    struct mtk_spi_slave *mdata;
    int irq, ret;
    const struct of_device_id *of_id;
    ctlr = devm_spi_alloc_target(&pdev.dev, sizeof(*mdata));
    if (!ctlr)
    return -ENOMEM;
    ctlr.auto_runtime_pm = true;
    ctlr.mode_bits = SPI_CPOL | SPI_CPHA;
    ctlr.mode_bits |= SPI_LSB_FIRST;
    ctlr.prepare_message = mtk_spi_slave_prepare_message;
    ctlr.transfer_one = mtk_spi_slave_transfer_one;
    ctlr.setup = mtk_spi_slave_setup;
    ctlr.target_abort = mtk_target_abort;
    of_id = of_match_node(mtk_spi_slave_of_match, pdev.dev.of_node);
    if (!of_id) {
    dev_err(&pdev.dev, "failed to probe of_node\n");
    return -EINVAL;
    }
    mdata = spi_controller_get_devdata(ctlr);
    mdata.dev_comp = of_id.data;
    if (mdata.dev_comp.must_rx)
    ctlr.flags = SPI_CONTROLLER_MUST_RX;
    platform_set_drvdata(pdev, ctlr);
    init_completion(&mdata.xfer_done);
    mdata.dev = &pdev.dev;
    mdata.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(mdata.base))
    return PTR_ERR(mdata.base);
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    ret = devm_request_irq(&pdev.dev, irq, mtk_spi_slave_interrupt,
    IRQF_TRIGGER_NONE, dev_name(&pdev.dev), ctlr);
    if (ret) {
    dev_err(&pdev.dev, "failed to register irq (%d)\n", ret);
    return ret;
    }
    mdata.spi_clk = devm_clk_get(&pdev.dev, "spi");
    if (IS_ERR(mdata.spi_clk)) {
    ret = PTR_ERR(mdata.spi_clk);
    dev_err(&pdev.dev, "failed to get spi-clk: %d\n", ret);
    return ret;
    }
    ret = clk_prepare_enable(mdata.spi_clk);
    if (ret < 0) {
    dev_err(&pdev.dev, "failed to enable spi_clk (%d)\n", ret);
    return ret;
    }
    pm_runtime_enable(&pdev.dev);
    ret = spi_register_controller(ctlr);
    clk_disable_unprepare(mdata.spi_clk);
    if (ret) {
    dev_err(&pdev.dev,
    "failed to register slave controller(%d)\n", ret);
    goto err_disable_runtime_pm;
    }
    return 0;
    err_disable_runtime_pm:
    pm_runtime_disable(&pdev.dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mtk_spi_slave_remove(pdev: *mut platform_device) {
    static void mtk_spi_slave_remove(struct platform_device *pdev)
    {
    struct spi_controller *ctlr = platform_get_drvdata(pdev);
    spi_unregister_controller(ctlr);
    pm_runtime_disable(&pdev.dev);
    }
#[no_mangle]
unsafe extern "C" fn mtk_spi_slave_suspend(dev: *mut device) -> c_int {
    static int mtk_spi_slave_suspend(struct device *dev)
    {
    struct spi_controller *ctlr = dev_get_drvdata(dev);
    struct mtk_spi_slave *mdata = spi_controller_get_devdata(ctlr);
    int ret;
    ret = spi_controller_suspend(ctlr);
    if (ret)
    return ret;
    if (!pm_runtime_suspended(dev))
    clk_disable_unprepare(mdata.spi_clk);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mtk_spi_slave_resume(dev: *mut device) -> c_int {
    static int mtk_spi_slave_resume(struct device *dev)
    {
    struct spi_controller *ctlr = dev_get_drvdata(dev);
    struct mtk_spi_slave *mdata = spi_controller_get_devdata(ctlr);
    int ret;
    if (!pm_runtime_suspended(dev)) {
    ret = clk_prepare_enable(mdata.spi_clk);
    if (ret < 0) {
    dev_err(dev, "failed to enable spi_clk (%d)\n", ret);
    return ret;
    }
    }
    ret = spi_controller_resume(ctlr);
    if (ret < 0)
    clk_disable_unprepare(mdata.spi_clk);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mtk_spi_slave_runtime_suspend(dev: *mut device) -> c_int {
    static int mtk_spi_slave_runtime_suspend(struct device *dev)
    {
    struct spi_controller *ctlr = dev_get_drvdata(dev);
    struct mtk_spi_slave *mdata = spi_controller_get_devdata(ctlr);
    clk_disable_unprepare(mdata.spi_clk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mtk_spi_slave_runtime_resume(dev: *mut device) -> c_int {
    static int mtk_spi_slave_runtime_resume(struct device *dev)
    {
    struct spi_controller *ctlr = dev_get_drvdata(dev);
    struct mtk_spi_slave *mdata = spi_controller_get_devdata(ctlr);
    int ret;
    ret = clk_prepare_enable(mdata.spi_clk);
    if (ret < 0) {
    dev_err(dev, "failed to enable spi_clk (%d)\n", ret);
    return ret;
    }
    return 0;
    }
    static const struct dev_pm_ops mtk_spi_slave_pm = {
    SYSTEM_SLEEP_PM_OPS(mtk_spi_slave_suspend, mtk_spi_slave_resume)
    RUNTIME_PM_OPS(mtk_spi_slave_runtime_suspend,
    mtk_spi_slave_runtime_resume, core::ptr::null_mut())
    };
    static struct platform_driver mtk_spi_slave_driver = {
    .driver = {
    .name = "mtk-spi-slave",
    .pm	= pm_ptr(&mtk_spi_slave_pm),
    .of_match_table = mtk_spi_slave_of_match,
    },
    .probe = mtk_spi_slave_probe,
    .remove = mtk_spi_slave_remove,
    };
    module_platform_driver(mtk_spi_slave_driver);
    MODULE_DESCRIPTION("MTK SPI Slave Controller driver");
    MODULE_AUTHOR("Leilk Liu <leilk.liu@mediatek.com>");
    MODULE_LICENSE("GPL v2");
    MODULE_ALIAS("platform:mtk-spi-slave");
