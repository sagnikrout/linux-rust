//! Automatically rewritten from C to Rust
//! Source: drivers/char/hw_random/omap-rng.c
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


//
// omap-rng.c - RNG driver for TI OMAP CPU family
//
// Author: Deepak Saxena <dsaxena@plexity.net>
//
// Copyright 2005 (c) MontaVista Software, Inc.
//
// Mostly based on original driver:
//
// Copyright (C) 2005 Nokia Corporation
// Author: Juha Yrjölä <juha.yrjola@nokia.com>
//
// This file is licensed under  the terms of the GNU General Public
// License version 2. This program is licensed "as is" without any
// warranty of any kind, whether express or implied.
//

pub const RNG_CONTROL_STARTUP_CYCLES_SHIFT: c_int = 16;

pub const RNG_CONTROL_ENABLE_TRNG_SHIFT: c_int = 10;

pub const RNG_CONFIG_MAX_REFIL_CYCLES_SHIFT: c_int = 16;

pub const RNG_CONFIG_MIN_REFIL_CYCLES_SHIFT: c_int = 0;

pub const RNG_CONTROL_STARTUP_CYCLES: c_uint = 0xff;
pub const RNG_CONFIG_MIN_REFIL_CYCLES: c_uint = 0x21;
pub const RNG_CONFIG_MAX_REFIL_CYCLES: c_uint = 0x22;
pub const RNG_ALARMCNT_ALARM_TH_SHIFT: c_uint = 0x0;

pub const RNG_ALARMCNT_SHUTDOWN_TH_SHIFT: c_int = 16;

pub const RNG_ALARM_THRESHOLD: c_uint = 0xff;
pub const RNG_SHUTDOWN_THRESHOLD: c_uint = 0x4;
pub const RNG_REG_FROENABLE_MASK: c_uint = 0xffffff;
pub const RNG_REG_FRODETUNE_MASK: c_uint = 0xffffff;
pub const OMAP2_RNG_OUTPUT_SIZE: c_uint = 0x4;
pub const OMAP4_RNG_OUTPUT_SIZE: c_uint = 0x8;
pub const EIP76_RNG_OUTPUT_SIZE: c_uint = 0x10;
//
// EIP76 RNG takes approx. 700us to produce 16 bytes of output data
// as per testing results. And to account for the lack of udelay()'s
// reliability, we keep the timeout as 1000us.
//
pub const RNG_DATA_FILL_TIMEOUT: c_int = 100;
    enum {
    RNG_OUTPUT_0_REG = 0,
    RNG_OUTPUT_1_REG,
    RNG_OUTPUT_2_REG,
    RNG_OUTPUT_3_REG,
    RNG_STATUS_REG,
    RNG_INTMASK_REG,
    RNG_INTACK_REG,
    RNG_CONTROL_REG,
    RNG_CONFIG_REG,
    RNG_ALARMCNT_REG,
    RNG_FROENABLE_REG,
    RNG_FRODETUNE_REG,
    RNG_ALARMMASK_REG,
    RNG_ALARMSTOP_REG,
    RNG_REV_REG,
    RNG_SYSCONFIG_REG,
    };
    static const u16 reg_map_omap2[] = {
    [RNG_OUTPUT_0_REG]	= 0x0,
    [RNG_STATUS_REG]	= 0x4,
    [RNG_CONFIG_REG]	= 0x28,
    [RNG_REV_REG]		= 0x3c,
    [RNG_SYSCONFIG_REG]	= 0x40,
    };
    static const u16 reg_map_omap4[] = {
    [RNG_OUTPUT_0_REG]	= 0x0,
    [RNG_OUTPUT_1_REG]	= 0x4,
    [RNG_STATUS_REG]	= 0x8,
    [RNG_INTMASK_REG]	= 0xc,
    [RNG_INTACK_REG]	= 0x10,
    [RNG_CONTROL_REG]	= 0x14,
    [RNG_CONFIG_REG]	= 0x18,
    [RNG_ALARMCNT_REG]	= 0x1c,
    [RNG_FROENABLE_REG]	= 0x20,
    [RNG_FRODETUNE_REG]	= 0x24,
    [RNG_ALARMMASK_REG]	= 0x28,
    [RNG_ALARMSTOP_REG]	= 0x2c,
    [RNG_REV_REG]		= 0x1FE0,
    [RNG_SYSCONFIG_REG]	= 0x1FE4,
    };
    static const u16 reg_map_eip76[] = {
    [RNG_OUTPUT_0_REG]	= 0x0,
    [RNG_OUTPUT_1_REG]	= 0x4,
    [RNG_OUTPUT_2_REG]	= 0x8,
    [RNG_OUTPUT_3_REG]	= 0xc,
    [RNG_STATUS_REG]	= 0x10,
    [RNG_INTACK_REG]	= 0x10,
    [RNG_CONTROL_REG]	= 0x14,
    [RNG_CONFIG_REG]	= 0x18,
    [RNG_ALARMCNT_REG]	= 0x1c,
    [RNG_FROENABLE_REG]	= 0x20,
    [RNG_FRODETUNE_REG]	= 0x24,
    [RNG_ALARMMASK_REG]	= 0x28,
    [RNG_ALARMSTOP_REG]	= 0x2c,
    [RNG_REV_REG]		= 0x7c,
    };
    struct omap_rng_dev;
//
// struct omap_rng_pdata - RNG IP block-specific data
// @regs: Pointer to the register offsets structure.
// @data_size: No. of bytes in RNG output.
// @data_present: Callback to determine if data is available.
// @init: Callback for IP specific initialization sequence.
// @cleanup: Callback for IP specific cleanup sequence.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap_rng_pdata {
    pub regs: *mut u16,
    pub data_size: u32,
    pub priv): *mut *mut u32 (data_present)(struct omap_rng_dev,
    pub priv): *mut *mut int (init)(struct omap_rng_dev,
    pub priv): *mut *mut void (cleanup)(struct omap_rng_dev,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap_rng_dev {
    pub base: *mut void __iomem,
    pub dev: *mut device,
    pub pdata: *const omap_rng_pdata,
    pub rng: hwrng,
    pub clk: *mut clk,
    pub clk_reg: *mut clk,
}

#[no_mangle]
pub unsafe extern "C" fn omap_rng_read(priv: *mut omap_rng_dev, reg: u16) -> u32 {
    static inline u32 omap_rng_read(struct omap_rng_dev *priv, u16 reg)
    {
    return __raw_readl(priv.base + priv.pdata.regs[reg]);
    }
    static inline void omap_rng_write(struct omap_rng_dev *priv, u16 reg,
    u32 val)
    {
    __raw_writel(val, priv.base + priv.pdata.regs[reg]);
    }
    static int omap_rng_do_read(struct hwrng *rng, void *data, size_t max,
    bool wait)
    {
    struct omap_rng_dev *priv;
    int i, present;
    priv = (struct omap_rng_dev *)rng.priv;
    if (max < priv.pdata.data_size)
    return 0;
    for (i = 0; i < RNG_DATA_FILL_TIMEOUT; i++) {
    present = priv.pdata.data_present(priv);
    if (present || !wait)
    break;
    udelay(10);
    }
    if (!present)
    return 0;
    memcpy_fromio(data, priv.base + priv.pdata.regs[RNG_OUTPUT_0_REG],
    priv.pdata.data_size);
    if (priv.pdata.regs[RNG_INTACK_REG])
    omap_rng_write(priv, RNG_INTACK_REG, RNG_REG_INTACK_RDY_MASK);
    return priv.pdata.data_size;
    }
#[no_mangle]
unsafe extern "C" fn omap_rng_init(rng: *mut hwrng) -> c_int {
    static int omap_rng_init(struct hwrng *rng)
    {
    struct omap_rng_dev *priv;
    priv = (struct omap_rng_dev *)rng.priv;
    return priv.pdata.init(priv);
    }
#[no_mangle]
unsafe extern "C" fn omap_rng_cleanup(rng: *mut hwrng) {
    static void omap_rng_cleanup(struct hwrng *rng)
    {
    struct omap_rng_dev *priv;
    priv = (struct omap_rng_dev *)rng.priv;
    priv.pdata.cleanup(priv);
    }
#[no_mangle]
pub unsafe extern "C" fn omap2_rng_data_present(priv: *mut omap_rng_dev) -> u32 {
    static inline u32 omap2_rng_data_present(struct omap_rng_dev *priv)
    {
    return omap_rng_read(priv, RNG_STATUS_REG) ? 0 : 1;
    }
#[no_mangle]
unsafe extern "C" fn omap2_rng_init(priv: *mut omap_rng_dev) -> c_int {
    static int omap2_rng_init(struct omap_rng_dev *priv)
    {
    omap_rng_write(priv, RNG_SYSCONFIG_REG, 0x1);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn omap2_rng_cleanup(priv: *mut omap_rng_dev) {
    static void omap2_rng_cleanup(struct omap_rng_dev *priv)
    {
    omap_rng_write(priv, RNG_SYSCONFIG_REG, 0x0);
    }
    static struct omap_rng_pdata omap2_rng_pdata = {
    .regs		= (u16 *)reg_map_omap2,
    .data_size	= OMAP2_RNG_OUTPUT_SIZE,
    .data_present	= omap2_rng_data_present,
    .init		= omap2_rng_init,
    .cleanup	= omap2_rng_cleanup,
    };
#[no_mangle]
pub unsafe extern "C" fn omap4_rng_data_present(priv: *mut omap_rng_dev) -> u32 {
    static inline u32 omap4_rng_data_present(struct omap_rng_dev *priv)
    {
    return omap_rng_read(priv, RNG_STATUS_REG) & RNG_REG_STATUS_RDY;
    }
#[no_mangle]
unsafe extern "C" fn eip76_rng_init(priv: *mut omap_rng_dev) -> c_int {
    static int eip76_rng_init(struct omap_rng_dev *priv)
    {
    u32 val;
// Return if RNG is already running.
    if (omap_rng_read(priv, RNG_CONTROL_REG) & RNG_CONTROL_ENABLE_TRNG_MASK)
    return 0;
// Number of 512 bit blocks of raw Noise Source output data that must
// be processed by either the Conditioning Function or the
// SP 800-90 DRBG ‘BC_DF’ functionality to yield a ‘full entropy’
// output value.
//
    val = 0x5 << RNG_CONFIG_MIN_REFIL_CYCLES_SHIFT;
// Number of FRO samples that are XOR-ed together into one bit to be
// shifted into the main shift register
//
    val |= RNG_CONFIG_MAX_REFIL_CYCLES << RNG_CONFIG_MAX_REFIL_CYCLES_SHIFT;
    omap_rng_write(priv, RNG_CONFIG_REG, val);
// Enable all available FROs
    omap_rng_write(priv, RNG_FRODETUNE_REG, 0x0);
    omap_rng_write(priv, RNG_FROENABLE_REG, RNG_REG_FROENABLE_MASK);
// Enable TRNG
    val = RNG_CONTROL_ENABLE_TRNG_MASK;
    omap_rng_write(priv, RNG_CONTROL_REG, val);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn omap4_rng_init(priv: *mut omap_rng_dev) -> c_int {
    static int omap4_rng_init(struct omap_rng_dev *priv)
    {
    u32 val;
// Return if RNG is already running.
    if (omap_rng_read(priv, RNG_CONTROL_REG) & RNG_CONTROL_ENABLE_TRNG_MASK)
    return 0;
    val = RNG_CONFIG_MIN_REFIL_CYCLES << RNG_CONFIG_MIN_REFIL_CYCLES_SHIFT;
    val |= RNG_CONFIG_MAX_REFIL_CYCLES << RNG_CONFIG_MAX_REFIL_CYCLES_SHIFT;
    omap_rng_write(priv, RNG_CONFIG_REG, val);
    omap_rng_write(priv, RNG_FRODETUNE_REG, 0x0);
    omap_rng_write(priv, RNG_FROENABLE_REG, RNG_REG_FROENABLE_MASK);
    val = RNG_ALARM_THRESHOLD << RNG_ALARMCNT_ALARM_TH_SHIFT;
    val |= RNG_SHUTDOWN_THRESHOLD << RNG_ALARMCNT_SHUTDOWN_TH_SHIFT;
    omap_rng_write(priv, RNG_ALARMCNT_REG, val);
    val = RNG_CONTROL_STARTUP_CYCLES << RNG_CONTROL_STARTUP_CYCLES_SHIFT;
    val |= RNG_CONTROL_ENABLE_TRNG_MASK;
    omap_rng_write(priv, RNG_CONTROL_REG, val);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn omap4_rng_cleanup(priv: *mut omap_rng_dev) {
    static void omap4_rng_cleanup(struct omap_rng_dev *priv)
    {
    int val;
    val = omap_rng_read(priv, RNG_CONTROL_REG);
    val &= ~RNG_CONTROL_ENABLE_TRNG_MASK;
    omap_rng_write(priv, RNG_CONTROL_REG, val);
    }
#[no_mangle]
unsafe extern "C" fn omap4_rng_irq(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t omap4_rng_irq(int irq, void *dev_id)
    {
    struct omap_rng_dev *priv = dev_id;
    u32 fro_detune, fro_enable;
//
// Interrupt raised by a fro shutdown threshold, do the following:
// 1. Clear the alarm events.
// 2. De tune the FROs which are shutdown.
// 3. Re enable the shutdown FROs.
//
    omap_rng_write(priv, RNG_ALARMMASK_REG, 0x0);
    omap_rng_write(priv, RNG_ALARMSTOP_REG, 0x0);
    fro_enable = omap_rng_read(priv, RNG_FROENABLE_REG);
    fro_detune = ~fro_enable & RNG_REG_FRODETUNE_MASK;
    fro_detune = fro_detune | omap_rng_read(priv, RNG_FRODETUNE_REG);
    fro_enable = RNG_REG_FROENABLE_MASK;
    omap_rng_write(priv, RNG_FRODETUNE_REG, fro_detune);
    omap_rng_write(priv, RNG_FROENABLE_REG, fro_enable);
    omap_rng_write(priv, RNG_INTACK_REG, RNG_REG_INTACK_SHUTDOWN_OFLO_MASK);
    return IRQ_HANDLED;
    }
    static struct omap_rng_pdata omap4_rng_pdata = {
    .regs		= (u16 *)reg_map_omap4,
    .data_size	= OMAP4_RNG_OUTPUT_SIZE,
    .data_present	= omap4_rng_data_present,
    .init		= omap4_rng_init,
    .cleanup	= omap4_rng_cleanup,
    };
    static struct omap_rng_pdata eip76_rng_pdata = {
    .regs		= (u16 *)reg_map_eip76,
    .data_size	= EIP76_RNG_OUTPUT_SIZE,
    .data_present	= omap4_rng_data_present,
    .init		= eip76_rng_init,
    .cleanup	= omap4_rng_cleanup,
    };
    static const struct of_device_id omap_rng_of_match[] __maybe_unused = {
    {
    .compatible	= "ti,omap2-rng",
    .data		= &omap2_rng_pdata,
    },
    {
    .compatible	= "ti,omap4-rng",
    .data		= &omap4_rng_pdata,
    },
    {
    .compatible	= "inside-secure,safexcel-eip76",
    .data		= &eip76_rng_pdata,
    },
    {},
    };
    MODULE_DEVICE_TABLE(of, omap_rng_of_match);
    static int of_get_omap_rng_device_details(struct omap_rng_dev *priv,
    struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    int irq, err;
    priv.pdata = of_device_get_match_data(dev);
    if (!priv.pdata)
    return -ENODEV;
    if (of_device_is_compatible(dev.of_node, "ti,omap4-rng") ||
    of_device_is_compatible(dev.of_node, "inside-secure,safexcel-eip76")) {
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    err = devm_request_irq(dev, irq, omap4_rng_irq,
    IRQF_TRIGGER_NONE, dev_name(dev), priv);
    if (err)
    return err;
//
// On OMAP4, enabling the shutdown_oflo interrupt is
// done in the interrupt mask register. There is no
// such register on EIP76, and it's enabled by the
// same bit in the control register
//
    if (priv.pdata.regs[RNG_INTMASK_REG])
    omap_rng_write(priv, RNG_INTMASK_REG,
    RNG_SHUTDOWN_OFLO_MASK);
    else
    omap_rng_write(priv, RNG_CONTROL_REG,
    RNG_SHUTDOWN_OFLO_MASK);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn get_omap_rng_device_details(omap_rng: *mut omap_rng_dev) -> c_int {
    static int get_omap_rng_device_details(struct omap_rng_dev *omap_rng)
    {
// Only OMAP2/3 can be non-DT
    omap_rng.pdata = &omap2_rng_pdata;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn omap_rng_probe(pdev: *mut platform_device) -> c_int {
    static int omap_rng_probe(struct platform_device *pdev)
    {
    struct omap_rng_dev *priv;
    struct device *dev = &pdev.dev;
    int ret;
    priv = devm_kzalloc(dev, sizeof(struct omap_rng_dev), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.rng.read = omap_rng_do_read;
    priv.rng.init = omap_rng_init;
    priv.rng.cleanup = omap_rng_cleanup;
    priv.rng.quality = 900;
    priv.rng.priv = (unsigned long)priv;
    platform_set_drvdata(pdev, priv);
    priv.dev = dev;
    priv.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(priv.base)) {
    ret = PTR_ERR(priv.base);
    goto err_ioremap;
    }
    priv.rng.name = devm_kstrdup(dev, dev_name(dev), GFP_KERNEL);
    if (!priv.rng.name) {
    ret = -ENOMEM;
    goto err_ioremap;
    }
    pm_runtime_enable(&pdev.dev);
    ret = pm_runtime_resume_and_get(&pdev.dev);
    if (ret < 0) {
    dev_err(&pdev.dev, "Failed to runtime_get device: %d\n", ret);
    goto err_pm_disable;
    }
    priv.clk = devm_clk_get(&pdev.dev, core::ptr::null_mut());
    if (PTR_ERR(priv.clk) == -EPROBE_DEFER) {
    ret = -EPROBE_DEFER;
    goto err_pm_put;
    }
    if (!IS_ERR(priv.clk)) {
    ret = clk_prepare_enable(priv.clk);
    if (ret) {
    dev_err(&pdev.dev,
    "Unable to enable the clk: %d\n", ret);
    goto err_pm_put;
    }
    } else {
    priv.clk = core::ptr::null_mut();
    }
    priv.clk_reg = devm_clk_get(&pdev.dev, "reg");
    if (PTR_ERR(priv.clk_reg) == -EPROBE_DEFER) {
    ret = -EPROBE_DEFER;
    goto err_clk;
    }
    if (!IS_ERR(priv.clk_reg)) {
    ret = clk_prepare_enable(priv.clk_reg);
    if (ret) {
    dev_err(&pdev.dev,
    "Unable to enable the register clk: %d\n",
    ret);
    goto err_clk;
    }
    } else {
    priv.clk_reg = core::ptr::null_mut();
    }
    ret = (dev.of_node) ? of_get_omap_rng_device_details(priv, pdev) :
    get_omap_rng_device_details(priv);
    if (ret)
    goto err_register;
    ret = devm_hwrng_register(&pdev.dev, &priv.rng);
    if (ret)
    goto err_register;
    dev_info(&pdev.dev, "Random Number Generator ver. %02x\n",
    omap_rng_read(priv, RNG_REV_REG));
    return 0;
    err_register:
    clk_disable_unprepare(priv.clk_reg);
    err_clk:
    clk_disable_unprepare(priv.clk);
    err_pm_put:
    priv.base = core::ptr::null_mut();
    pm_runtime_put_sync(&pdev.dev);
    err_pm_disable:
    pm_runtime_disable(&pdev.dev);
    err_ioremap:
    dev_err(dev, "initialization failed.\n");
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn omap_rng_remove(pdev: *mut platform_device) {
    static void omap_rng_remove(struct platform_device *pdev)
    {
    struct omap_rng_dev *priv = platform_get_drvdata(pdev);
    priv.pdata.cleanup(priv);
    pm_runtime_put_sync(&pdev.dev);
    pm_runtime_disable(&pdev.dev);
    clk_disable_unprepare(priv.clk);
    clk_disable_unprepare(priv.clk_reg);
    }
#[no_mangle]
unsafe extern "C" fn omap_rng_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused omap_rng_suspend(struct device *dev)
    {
    struct omap_rng_dev *priv = dev_get_drvdata(dev);
    priv.pdata.cleanup(priv);
    pm_runtime_put_sync(dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn omap_rng_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused omap_rng_resume(struct device *dev)
    {
    struct omap_rng_dev *priv = dev_get_drvdata(dev);
    int ret;
    ret = pm_runtime_resume_and_get(dev);
    if (ret < 0) {
    dev_err(dev, "Failed to runtime_get device: %d\n", ret);
    return ret;
    }
    priv.pdata.init(priv);
    return 0;
    }
    static SIMPLE_DEV_PM_OPS(omap_rng_pm, omap_rng_suspend, omap_rng_resume);
    static struct platform_driver omap_rng_driver = {
    .driver = {
    .name		= "omap_rng",
    .pm		= &omap_rng_pm,
    .of_match_table = of_match_ptr(omap_rng_of_match),
    },
    .probe		= omap_rng_probe,
    .remove		= omap_rng_remove,
    };
    module_platform_driver(omap_rng_driver);
    MODULE_ALIAS("platform:omap_rng");
    MODULE_AUTHOR("Deepak Saxena (and others)");
    MODULE_DESCRIPTION("RNG driver for TI OMAP CPU family");
    MODULE_LICENSE("GPL");
