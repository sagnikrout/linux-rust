//! Automatically rewritten from C to Rust
//! Source: drivers/char/hw_random/imx-rngc.c
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
// RNG driver for Freescale RNGC
//
// Copyright (C) 2008-2012 Freescale Semiconductor, Inc.
// Copyright (C) 2017 Martin Kaiser <martin@kaiser.cx>
//

pub const RNGC_VER_ID: c_uint = 0x0000;
pub const RNGC_COMMAND: c_uint = 0x0004;
pub const RNGC_CONTROL: c_uint = 0x0008;
pub const RNGC_STATUS: c_uint = 0x000C;
pub const RNGC_ERROR: c_uint = 0x0010;
pub const RNGC_FIFO: c_uint = 0x0014;
// the fields in the ver id register

pub const RNGC_VER_MAJ_SHIFT: c_int = 8;
// the rng_type field
pub const RNGC_TYPE_RNGB: c_uint = 0x1;
pub const RNGC_TYPE_RNGC: c_uint = 0x2;

pub const RNGC_ERROR_STATUS_STAT_ERR: c_uint = 0x00000008;

    let mut self_test: static bool = true;
    module_param(self_test, bool, 0);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx_rngc {
    pub dev: *mut device,
    pub clk: *mut clk,
    pub base: *mut void __iomem,
    pub rng: hwrng,
    pub rng_op_done: completion,
//
// err_reg is written only by the irq handler and read only
// when interrupts are masked, we need no spinlock
//
    pub err_reg: u32,
}

#[no_mangle]
pub unsafe extern "C" fn imx_rngc_irq_mask_clear(rngc: *mut imx_rngc) {
    static inline void imx_rngc_irq_mask_clear(struct imx_rngc *rngc)
    {
    u32 ctrl, cmd;
// mask interrupts
    ctrl = readl(rngc.base + RNGC_CONTROL);
    ctrl |= RNGC_CTRL_MASK_DONE | RNGC_CTRL_MASK_ERROR;
    writel(ctrl, rngc.base + RNGC_CONTROL);
//
// CLR_INT clears the interrupt only if there's no error
// CLR_ERR clear the interrupt and the error register if there
// is an error
//
    cmd = readl(rngc.base + RNGC_COMMAND);
    cmd |= RNGC_CMD_CLR_INT | RNGC_CMD_CLR_ERR;
    writel(cmd, rngc.base + RNGC_COMMAND);
    }
#[no_mangle]
pub unsafe extern "C" fn imx_rngc_irq_unmask(rngc: *mut imx_rngc) {
    static inline void imx_rngc_irq_unmask(struct imx_rngc *rngc)
    {
    u32 ctrl;
    ctrl = readl(rngc.base + RNGC_CONTROL);
    ctrl &= ~(RNGC_CTRL_MASK_DONE | RNGC_CTRL_MASK_ERROR);
    writel(ctrl, rngc.base + RNGC_CONTROL);
    }
#[no_mangle]
unsafe extern "C" fn imx_rngc_self_test(rngc: *mut imx_rngc) -> c_int {
    static int imx_rngc_self_test(struct imx_rngc *rngc)
    {
    u32 cmd;
    int ret;
    imx_rngc_irq_unmask(rngc);
// run self test
    cmd = readl(rngc.base + RNGC_COMMAND);
    writel(cmd | RNGC_CMD_SELF_TEST, rngc.base + RNGC_COMMAND);
    ret = wait_for_completion_timeout(&rngc.rng_op_done,
    usecs_to_jiffies(RNGC_SELFTEST_TIMEOUT));
    imx_rngc_irq_mask_clear(rngc);
    if (!ret)
    return -ETIMEDOUT;
    return rngc.err_reg ? -EIO : 0;
    }
#[no_mangle]
unsafe extern "C" fn imx_rngc_read(rng: *mut hwrng, data: *mut c_void, max: usize, wait: bool) -> c_int {
    static int imx_rngc_read(struct hwrng *rng, void *data, size_t max, bool wait)
    {
    struct imx_rngc *rngc = container_of(rng, struct imx_rngc, rng);
    unsigned int status;
    int err, retval = 0;
    err = pm_runtime_resume_and_get(rngc.dev);
    if (err)
    return err;
    while (max >= sizeof(u32)) {
    status = readl(rngc.base + RNGC_STATUS);
// is there some error while reading this random number?
    if (status & RNGC_STATUS_ERROR)
    break;
    if (status & RNGC_STATUS_FIFO_LEVEL_MASK) {
// retrieve a random number from FIFO
// (u32 *)data = readl(rngc->base + RNGC_FIFO);
    retval += sizeof(u32);
    data += sizeof(u32);
    max -= sizeof(u32);
    }
    }
    pm_runtime_mark_last_busy(rngc.dev);
    pm_runtime_put(rngc.dev);
    return retval ? retval : -EIO;
    }
#[no_mangle]
unsafe extern "C" fn imx_rngc_irq(irq: c_int, priv: *mut c_void) -> irqreturn_t {
    static irqreturn_t imx_rngc_irq(int irq, void *priv)
    {
    struct imx_rngc *rngc = (struct imx_rngc *)priv;
    u32 status;
//
// clearing the interrupt will also clear the error register
// read error and status before clearing
//
    status = readl(rngc.base + RNGC_STATUS);
    rngc.err_reg = readl(rngc.base + RNGC_ERROR);
    imx_rngc_irq_mask_clear(rngc);
    if (status & (RNGC_STATUS_SEED_DONE | RNGC_STATUS_ST_DONE))
    complete(&rngc.rng_op_done);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn imx_rngc_init(rng: *mut hwrng) -> c_int {
    static int imx_rngc_init(struct hwrng *rng)
    {
    struct imx_rngc *rngc = container_of(rng, struct imx_rngc, rng);
    u32 cmd, ctrl;
    int ret, err;
    err = pm_runtime_resume_and_get(rngc.dev);
    if (err)
    return err;
// clear error
    cmd = readl(rngc.base + RNGC_COMMAND);
    writel(cmd | RNGC_CMD_CLR_ERR, rngc.base + RNGC_COMMAND);
    imx_rngc_irq_unmask(rngc);
// create seed, repeat while there is some statistical error
    do {
// seed creation
    cmd = readl(rngc.base + RNGC_COMMAND);
    writel(cmd | RNGC_CMD_SEED, rngc.base + RNGC_COMMAND);
    ret = wait_for_completion_timeout(&rngc.rng_op_done,
    msecs_to_jiffies(RNGC_SEED_TIMEOUT));
    if (!ret) {
    err = -ETIMEDOUT;
    goto out;
    }
    } while (rngc.err_reg == RNGC_ERROR_STATUS_STAT_ERR);
    if (rngc.err_reg) {
    err = -EIO;
    goto out;
    }
//
// enable automatic seeding, the rngc creates a new seed automatically
// after serving 2^20 random 160-bit words
//
    ctrl = readl(rngc.base + RNGC_CONTROL);
    ctrl |= RNGC_CTRL_AUTO_SEED;
    writel(ctrl, rngc.base + RNGC_CONTROL);
    out:
//
// if initialisation was successful, we keep the interrupt
// unmasked until imx_rngc_cleanup is called
// we mask the interrupt ourselves if we return an error
//
    if (err)
    imx_rngc_irq_mask_clear(rngc);
    pm_runtime_put(rngc.dev);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn imx_rngc_cleanup(rng: *mut hwrng) {
    static void imx_rngc_cleanup(struct hwrng *rng)
    {
    struct imx_rngc *rngc = container_of(rng, struct imx_rngc, rng);
    int err;
    err = pm_runtime_resume_and_get(rngc.dev);
    if (!err) {
    imx_rngc_irq_mask_clear(rngc);
    pm_runtime_put(rngc.dev);
    }
    }
#[no_mangle]
unsafe extern "C" fn imx_rngc_probe(pdev: *mut platform_device) -> int __init {
    static int __init imx_rngc_probe(struct platform_device *pdev)
    {
    struct imx_rngc *rngc;
    int ret;
    int irq;
    u32 ver_id;
    u8  rng_type;
    rngc = devm_kzalloc(&pdev.dev, sizeof(*rngc), GFP_KERNEL);
    if (!rngc)
    return -ENOMEM;
    rngc.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(rngc.base))
    return PTR_ERR(rngc.base);
    rngc.clk = devm_clk_get(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(rngc.clk))
    return dev_err_probe(&pdev.dev, PTR_ERR(rngc.clk), "Cannot get rng_clk\n");
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    clk_prepare_enable(rngc.clk);
    ver_id = readl(rngc.base + RNGC_VER_ID);
    rng_type = FIELD_GET(RNG_TYPE, ver_id);
//
// This driver supports only RNGC and RNGB. (There's a different
// driver for RNGA.)
//
    if (rng_type != RNGC_TYPE_RNGC && rng_type != RNGC_TYPE_RNGB) {
    clk_disable_unprepare(rngc.clk);
    return -ENODEV;
    }
    init_completion(&rngc.rng_op_done);
    rngc.rng.name = pdev.name;
    rngc.rng.init = imx_rngc_init;
    rngc.rng.read = imx_rngc_read;
    rngc.rng.cleanup = imx_rngc_cleanup;
    rngc.rng.quality = 19;
    rngc.dev = &pdev.dev;
    platform_set_drvdata(pdev, rngc);
    imx_rngc_irq_mask_clear(rngc);
    ret = devm_request_irq(&pdev.dev,
    irq, imx_rngc_irq, 0, pdev.name, (void *)rngc);
    if (ret) {
    clk_disable_unprepare(rngc.clk);
    return ret;
    }
    if (self_test) {
    ret = imx_rngc_self_test(rngc);
    if (ret) {
    clk_disable_unprepare(rngc.clk);
    return dev_err_probe(&pdev.dev, ret, "self test failed\n");
    }
    }
    pm_runtime_set_autosuspend_delay(&pdev.dev, RNGC_PM_TIMEOUT);
    pm_runtime_use_autosuspend(&pdev.dev);
    pm_runtime_set_active(&pdev.dev);
    devm_pm_runtime_enable(&pdev.dev);
    ret = devm_hwrng_register(&pdev.dev, &rngc.rng);
    if (ret) {
    clk_disable_unprepare(rngc.clk);
    return dev_err_probe(&pdev.dev, ret, "hwrng registration failed\n");
    }
    dev_info(&pdev.dev,
    "Freescale RNG%c registered (HW revision %d.%02d)\n",
    rng_type == RNGC_TYPE_RNGB ? 'B' : 'C',
    (ver_id >> RNGC_VER_MAJ_SHIFT) & 0xff, ver_id & 0xff);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn imx_rngc_suspend(dev: *mut device) -> c_int {
    static int imx_rngc_suspend(struct device *dev)
    {
    struct imx_rngc *rngc = dev_get_drvdata(dev);
    clk_disable_unprepare(rngc.clk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn imx_rngc_resume(dev: *mut device) -> c_int {
    static int imx_rngc_resume(struct device *dev)
    {
    struct imx_rngc *rngc = dev_get_drvdata(dev);
    clk_prepare_enable(rngc.clk);
    return 0;
    }
    static const struct dev_pm_ops imx_rngc_pm_ops = {
    SYSTEM_SLEEP_PM_OPS(pm_runtime_force_suspend, pm_runtime_force_resume)
    RUNTIME_PM_OPS(imx_rngc_suspend, imx_rngc_resume, core::ptr::null_mut())
    };
    static const struct of_device_id imx_rngc_dt_ids[] = {
    { .compatible = "fsl,imx25-rngb" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, imx_rngc_dt_ids);
    static struct platform_driver imx_rngc_driver = {
    .driver = {
    .name = KBUILD_MODNAME,
    .pm = pm_ptr(&imx_rngc_pm_ops),
    .of_match_table = imx_rngc_dt_ids,
    },
    };
    module_platform_driver_probe(imx_rngc_driver, imx_rngc_probe);
    MODULE_AUTHOR("Freescale Semiconductor, Inc.");
    MODULE_DESCRIPTION("H/W RNGC driver for i.MX");
    MODULE_LICENSE("GPL");
