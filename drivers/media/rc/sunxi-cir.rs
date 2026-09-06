//! Automatically rewritten from C to Rust
//! Source: drivers/media/rc/sunxi-cir.c
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
//
// Driver for Allwinner sunXi IR controller
//
// Copyright (C) 2014 Alexsey Shestacov <wingrime@linux-sunxi.org>
// Copyright (C) 2014 Alexander Bersenev <bay@hackerdom.ru>
//
// Based on sun5i-ir.c:
// Copyright (C) 2007-2012 Daniel Wang
// Allwinner Technology Co., Ltd. <www.allwinnertech.com>
//

// Registers
// IR Control
pub const SUNXI_IR_CTL_REG: c_uint = 0x00;
// Global Enable

// RX block enable

// CIR mode

// Rx Config
pub const SUNXI_IR_RXCTL_REG: c_uint = 0x10;
// Pulse Polarity Invert flag

// Rx Data
pub const SUNXI_IR_RXFIFO_REG: c_uint = 0x20;
// Rx Interrupt Enable
pub const SUNXI_IR_RXINT_REG: c_uint = 0x2C;
// Rx FIFO Overflow Interrupt Enable

// Rx Packet End Interrupt Enable

// Rx FIFO Data Available Interrupt Enable

// Rx FIFO available byte level

// Rx Interrupt Status
pub const SUNXI_IR_RXSTA_REG: c_uint = 0x30;
// Rx FIFO Overflow

// Rx Packet End

// Rx FIFO Data Available

// RX FIFO Get Available Counter

// Clear all interrupt status value
pub const REG_RXSTA_CLEARALL: c_uint = 0xff;
// IR Sample Config
pub const SUNXI_IR_CIR_REG: c_uint = 0x34;
// CIR_REG register noise threshold

// CIR_REG register idle threshold

// Required frequency for IR0 or IR1 clock in CIR mode (default)
pub const SUNXI_IR_BASE_CLK: c_int = 8000000;
// Noise threshold in samples
pub const SUNXI_IR_RXNOISE: c_int = 1;
//
// struct sunxi_ir_quirks - Differences between SoC variants.
//
// @has_reset: SoC needs reset deasserted.
// @fifo_size: size of the fifo.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sunxi_ir_quirks {
    pub has_reset: bool,
    pub fifo_size: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sunxi_ir {
    pub rc: *mut rc_dev,
    pub base: *mut void __iomem,
    pub irq: c_int,
    pub fifo_size: c_int,
    pub clk: *mut clk,
    pub apb_clk: *mut clk,
    pub rst: *mut reset_control,
    pub map_name: *const c_char,
}

#[no_mangle]
unsafe extern "C" fn sunxi_ir_irq(irqno: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t sunxi_ir_irq(int irqno, void *dev_id)
    {
    unsigned long status;
    unsigned char dt;
    unsigned int cnt, rc;
    struct sunxi_ir *ir = dev_id;
    let mut rawir: ir_raw_event = {};
    status = readl(ir.base + SUNXI_IR_RXSTA_REG);
// clean all pending statuses
    writel(status | REG_RXSTA_CLEARALL, ir.base + SUNXI_IR_RXSTA_REG);
    if (status & (REG_RXSTA_RA | REG_RXSTA_RPE)) {
// How many messages in fifo
    rc  = REG_RXSTA_GET_AC(status);
// Sanity check
    rc = rc > ir.fifo_size ? ir.fifo_size : rc;
// If we have data
    for (cnt = 0; cnt < rc; cnt++) {
// for each bit in fifo
    dt = readb(ir.base + SUNXI_IR_RXFIFO_REG);
    rawir.pulse = (dt & 0x80) != 0;
    rawir.duration = ((dt & 0x7f) + 1) *
    ir.rc.rx_resolution;
    ir_raw_event_store_with_filter(ir.rc, &rawir);
    }
    }
    if (status & REG_RXSTA_ROI) {
    ir_raw_event_overflow(ir.rc);
    } else if (status & REG_RXSTA_RPE) {
    ir_raw_event_set_idle(ir.rc, true);
    ir_raw_event_handle(ir.rc);
    } else {
    ir_raw_event_handle(ir.rc);
    }
    return IRQ_HANDLED;
    }
// Convert idle threshold to usec
#[no_mangle]
unsafe extern "C" fn sunxi_ithr_to_usec(base_clk: c_uint, ithr: c_uint) -> c_uint {
    static unsigned int sunxi_ithr_to_usec(unsigned int base_clk, unsigned int ithr)
    {
    return DIV_ROUND_CLOSEST(USEC_PER_SEC * (ithr + 1),
    base_clk / (128 * 64));
    }
// Convert usec to idle threshold
#[no_mangle]
unsafe extern "C" fn sunxi_usec_to_ithr(base_clk: c_uint, usec: c_uint) -> c_uint {
    static unsigned int sunxi_usec_to_ithr(unsigned int base_clk, unsigned int usec)
    {
// make sure we don't end up with a timeout less than requested
    return DIV_ROUND_UP((base_clk / (128 * 64)) * usec,  USEC_PER_SEC) - 1;
    }
#[no_mangle]
unsafe extern "C" fn sunxi_ir_set_timeout(rc_dev: *mut rc_dev, timeout: c_uint) -> c_int {
    static int sunxi_ir_set_timeout(struct rc_dev *rc_dev, unsigned int timeout)
    {
    struct sunxi_ir *ir = rc_dev.priv;
    let mut base_clk: c_uint = clk_get_rate(ir.clk);
    let mut ithr: c_uint = sunxi_usec_to_ithr(base_clk, timeout);
    dev_dbg(rc_dev.dev.parent, "setting idle threshold to %u\n", ithr);
// Set noise threshold and idle threshold
    writel(REG_CIR_NTHR(SUNXI_IR_RXNOISE) | REG_CIR_ITHR(ithr),
    ir.base + SUNXI_IR_CIR_REG);
    rc_dev.timeout = sunxi_ithr_to_usec(base_clk, ithr);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sunxi_ir_hw_init(dev: *mut device) -> c_int {
    static int sunxi_ir_hw_init(struct device *dev)
    {
    struct sunxi_ir *ir = dev_get_drvdata(dev);
    u32 tmp;
    int ret;
    ret = reset_control_deassert(ir.rst);
    if (ret)
    return ret;
    ret = clk_prepare_enable(ir.apb_clk);
    if (ret) {
    dev_err(dev, "failed to enable apb clk\n");
    goto exit_assert_reset;
    }
    ret = clk_prepare_enable(ir.clk);
    if (ret) {
    dev_err(dev, "failed to enable ir clk\n");
    goto exit_disable_apb_clk;
    }
// Enable CIR Mode
    writel(REG_CTL_MD, ir.base + SUNXI_IR_CTL_REG);
// Set noise threshold and idle threshold
    sunxi_ir_set_timeout(ir.rc, ir.rc.timeout);
// Invert Input Signal
    writel(REG_RXCTL_RPPI, ir.base + SUNXI_IR_RXCTL_REG);
// Clear All Rx Interrupt Status
    writel(REG_RXSTA_CLEARALL, ir.base + SUNXI_IR_RXSTA_REG);
//
// Enable IRQ on overflow, packet end, FIFO available with trigger
// level
//
    writel(REG_RXINT_ROI_EN | REG_RXINT_RPEI_EN |
    REG_RXINT_RAI_EN | REG_RXINT_RAL(ir.fifo_size / 2 - 1),
    ir.base + SUNXI_IR_RXINT_REG);
// Enable IR Module
    tmp = readl(ir.base + SUNXI_IR_CTL_REG);
    writel(tmp | REG_CTL_GEN | REG_CTL_RXEN, ir.base + SUNXI_IR_CTL_REG);
    return 0;
    exit_disable_apb_clk:
    clk_disable_unprepare(ir.apb_clk);
    exit_assert_reset:
    reset_control_assert(ir.rst);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn sunxi_ir_hw_exit(dev: *mut device) {
    static void sunxi_ir_hw_exit(struct device *dev)
    {
    struct sunxi_ir *ir = dev_get_drvdata(dev);
    clk_disable_unprepare(ir.clk);
    clk_disable_unprepare(ir.apb_clk);
    reset_control_assert(ir.rst);
    }
#[no_mangle]
unsafe extern "C" fn sunxi_ir_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused sunxi_ir_suspend(struct device *dev)
    {
    sunxi_ir_hw_exit(dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sunxi_ir_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused sunxi_ir_resume(struct device *dev)
    {
    return sunxi_ir_hw_init(dev);
    }
    static SIMPLE_DEV_PM_OPS(sunxi_ir_pm_ops, sunxi_ir_suspend, sunxi_ir_resume);
#[no_mangle]
unsafe extern "C" fn sunxi_ir_probe(pdev: *mut platform_device) -> c_int {
    static int sunxi_ir_probe(struct platform_device *pdev)
    {
    let mut ret: c_int = 0;
    struct device *dev = &pdev.dev;
    struct device_node *dn = dev.of_node;
    const struct sunxi_ir_quirks *quirks;
    struct sunxi_ir *ir;
    let mut b_clk_freq: u32 = SUNXI_IR_BASE_CLK;
    ir = devm_kzalloc(dev, sizeof(struct sunxi_ir), GFP_KERNEL);
    if (!ir)
    return -ENOMEM;
    quirks = of_device_get_match_data(&pdev.dev);
    if (!quirks) {
    dev_err(&pdev.dev, "Failed to determine the quirks to use\n");
    return -ENODEV;
    }
    ir.fifo_size = quirks.fifo_size;
// Clock
    ir.apb_clk = devm_clk_get(dev, "apb");
    if (IS_ERR(ir.apb_clk)) {
    dev_err(dev, "failed to get a apb clock.\n");
    return PTR_ERR(ir.apb_clk);
    }
    ir.clk = devm_clk_get(dev, "ir");
    if (IS_ERR(ir.clk)) {
    dev_err(dev, "failed to get a ir clock.\n");
    return PTR_ERR(ir.clk);
    }
// Base clock frequency (optional)
    of_property_read_u32(dn, "clock-frequency", &b_clk_freq);
// Reset
    if (quirks.has_reset) {
    ir.rst = devm_reset_control_get_exclusive(dev, core::ptr::null_mut());
    if (IS_ERR(ir.rst))
    return PTR_ERR(ir.rst);
    }
    ret = clk_set_rate(ir.clk, b_clk_freq);
    if (ret) {
    dev_err(dev, "set ir base clock failed!\n");
    return ret;
    }
    dev_dbg(dev, "set base clock frequency to %d Hz.\n", b_clk_freq);
// IO
    ir.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(ir.base)) {
    return PTR_ERR(ir.base);
    }
    ir.rc = rc_allocate_device(RC_DRIVER_IR_RAW);
    if (!ir.rc) {
    dev_err(dev, "failed to allocate device\n");
    return -ENOMEM;
    }
    ir.rc.priv = ir;
    ir.rc.device_name = SUNXI_IR_DEV;
    ir.rc.input_phys = "sunxi-ir/input0";
    ir.rc.input_id.bustype = BUS_HOST;
    ir.rc.input_id.vendor = 0x0001;
    ir.rc.input_id.product = 0x0001;
    ir.rc.input_id.version = 0x0100;
    ir.map_name = of_get_property(dn, "linux,rc-map-name", core::ptr::null_mut());
    ir.rc.map_name = ir.map_name ?: RC_MAP_EMPTY;
    ir.rc.dev.parent = dev;
    ir.rc.allowed_protocols = RC_PROTO_BIT_ALL_IR_DECODER;
// Frequency after IR internal divider with sample period in us
    ir.rc.rx_resolution = (USEC_PER_SEC / (b_clk_freq / 64));
    ir.rc.timeout = IR_DEFAULT_TIMEOUT;
    ir.rc.min_timeout = sunxi_ithr_to_usec(b_clk_freq, 0);
    ir.rc.max_timeout = sunxi_ithr_to_usec(b_clk_freq, 255);
    ir.rc.s_timeout = sunxi_ir_set_timeout;
    ir.rc.driver_name = SUNXI_IR_DEV;
    ret = rc_register_device(ir.rc);
    if (ret) {
    dev_err(dev, "failed to register rc device\n");
    goto exit_free_dev;
    }
    platform_set_drvdata(pdev, ir);
// IRQ
    ir.irq = platform_get_irq(pdev, 0);
    if (ir.irq < 0) {
    ret = ir.irq;
    goto exit_unregister_dev;
    }
    ret = devm_request_irq(dev, ir.irq, sunxi_ir_irq, 0, SUNXI_IR_DEV, ir);
    if (ret) {
    dev_err(dev, "failed request irq\n");
    goto exit_unregister_dev;
    }
    ret = sunxi_ir_hw_init(dev);
    if (ret)
    goto exit_unregister_dev;
    dev_info(dev, "initialized sunXi IR driver\n");
    return 0;
    exit_unregister_dev:
    rc_unregister_device(ir.rc);
    exit_free_dev:
    rc_free_device(ir.rc);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn sunxi_ir_remove(pdev: *mut platform_device) {
    static void sunxi_ir_remove(struct platform_device *pdev)
    {
    struct sunxi_ir *ir = platform_get_drvdata(pdev);
    rc_unregister_device(ir.rc);
    rc_free_device(ir.rc);
    sunxi_ir_hw_exit(&pdev.dev);
    }
#[no_mangle]
unsafe extern "C" fn sunxi_ir_shutdown(pdev: *mut platform_device) {
    static void sunxi_ir_shutdown(struct platform_device *pdev)
    {
    sunxi_ir_hw_exit(&pdev.dev);
    }
    static const struct sunxi_ir_quirks sun4i_a10_ir_quirks = {
    .has_reset = false,
    .fifo_size = 16,
    };
    static const struct sunxi_ir_quirks sun5i_a13_ir_quirks = {
    .has_reset = false,
    .fifo_size = 64,
    };
    static const struct sunxi_ir_quirks sun6i_a31_ir_quirks = {
    .has_reset = true,
    .fifo_size = 64,
    };
    static const struct of_device_id sunxi_ir_match[] = {
    {
    .compatible = "allwinner,sun4i-a10-ir",
    .data = &sun4i_a10_ir_quirks,
    },
    {
    .compatible = "allwinner,sun5i-a13-ir",
    .data = &sun5i_a13_ir_quirks,
    },
    {
    .compatible = "allwinner,sun6i-a31-ir",
    .data = &sun6i_a31_ir_quirks,
    },
    {}
    };
    MODULE_DEVICE_TABLE(of, sunxi_ir_match);
    static struct platform_driver sunxi_ir_driver = {
    .probe          = sunxi_ir_probe,
    .remove         = sunxi_ir_remove,
    .shutdown       = sunxi_ir_shutdown,
    .driver = {
    .name = SUNXI_IR_DEV,
    .of_match_table = sunxi_ir_match,
    .pm = &sunxi_ir_pm_ops,
    },
    };
    module_platform_driver(sunxi_ir_driver);
    MODULE_DESCRIPTION("Allwinner sunXi IR controller driver");
    MODULE_AUTHOR("Alexsey Shestacov <wingrime@linux-sunxi.org>");
    MODULE_LICENSE("GPL");
