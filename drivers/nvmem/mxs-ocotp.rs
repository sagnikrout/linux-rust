//! Automatically rewritten from C to Rust
//! Source: drivers/nvmem/mxs-ocotp.c
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
// Freescale MXS On-Chip OTP driver
//
// Copyright (C) 2015 Stefan Wahren <stefan.wahren@i2se.com>
//
// Based on the driver from Huang Shijie and Christoph G. Baumann
//

// OCOTP registers and bits

pub const OCOTP_TIMEOUT: c_int = 10000;
pub const OCOTP_DATA_OFFSET: c_uint = 0x20;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mxs_ocotp {
    pub clk: *mut clk,
    pub base: *mut void __iomem,
    pub nvmem: *mut nvmem_device,
}

#[no_mangle]
unsafe extern "C" fn mxs_ocotp_wait(otp: *mut mxs_ocotp) -> c_int {
    static int mxs_ocotp_wait(struct mxs_ocotp *otp)
    {
    let mut timeout: c_int = OCOTP_TIMEOUT;
    let mut status: c_uint = 0;
    while (timeout--) {
    status = readl(otp.base);
    if (!(status & (BM_OCOTP_CTRL_BUSY | BM_OCOTP_CTRL_ERROR)))
    break;
    cpu_relax();
    }
    if (status & BM_OCOTP_CTRL_BUSY)
    return -EBUSY;
#[no_mangle]
pub unsafe extern "C" fn if(BM_OCOTP_CTRL_ERROR: status &) -> else {
    else if (status & BM_OCOTP_CTRL_ERROR)
    return -EIO;
    return 0;
    }
    static int mxs_ocotp_read(void *context, unsigned int offset,
    void *val, size_t bytes)
    {
    struct mxs_ocotp *otp = context;
    u32 *buf = val;
    int ret;
    ret = clk_enable(otp.clk);
    if (ret)
    return ret;
    writel(BM_OCOTP_CTRL_ERROR, otp.base + STMP_OFFSET_REG_CLR);
    ret = mxs_ocotp_wait(otp);
    if (ret)
    goto disable_clk;
// open OCOTP banks for read
    writel(BM_OCOTP_CTRL_RD_BANK_OPEN, otp.base + STMP_OFFSET_REG_SET);
// approximately wait 33 hclk cycles
    udelay(1);
    ret = mxs_ocotp_wait(otp);
    if (ret)
    goto close_banks;
    while (bytes) {
    if ((offset < OCOTP_DATA_OFFSET) || (offset % 16)) {
// fill up non-data register
// buf++ = 0;
    } else {
// buf++ = readl(otp->base + offset);
    }
    bytes -= 4;
    offset += 4;
    }
    close_banks:
// close banks for power saving
    writel(BM_OCOTP_CTRL_RD_BANK_OPEN, otp.base + STMP_OFFSET_REG_CLR);
    disable_clk:
    clk_disable(otp.clk);
    return ret;
    }
    static struct nvmem_config ocotp_config = {
    .name = "mxs-ocotp",
    .stride = 16,
    .word_size = 4,
    .reg_read = mxs_ocotp_read,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mxs_data {
    pub size: c_int,
}

    static const struct mxs_data imx23_data = {
    .size = 0x220,
    };
    static const struct mxs_data imx28_data = {
    .size = 0x2a0,
    };
    static const struct of_device_id mxs_ocotp_match[] = {
    { .compatible = "fsl,imx23-ocotp", .data = &imx23_data },
    { .compatible = "fsl,imx28-ocotp", .data = &imx28_data },
    { /* sentinel */},
    };
    MODULE_DEVICE_TABLE(of, mxs_ocotp_match);
#[no_mangle]
unsafe extern "C" fn mxs_ocotp_action(data: *mut c_void) {
    static void mxs_ocotp_action(void *data)
    {
    clk_unprepare(data);
    }
#[no_mangle]
unsafe extern "C" fn mxs_ocotp_probe(pdev: *mut platform_device) -> c_int {
    static int mxs_ocotp_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    const struct mxs_data *data;
    struct mxs_ocotp *otp;
    int ret;
    data = device_get_match_data(dev);
    if (!data)
    return -EINVAL;
    otp = devm_kzalloc(dev, sizeof(*otp), GFP_KERNEL);
    if (!otp)
    return -ENOMEM;
    otp.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(otp.base))
    return PTR_ERR(otp.base);
    otp.clk = devm_clk_get(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(otp.clk))
    return PTR_ERR(otp.clk);
    ret = clk_prepare(otp.clk);
    if (ret < 0) {
    dev_err(dev, "failed to prepare clk: %d\n", ret);
    return ret;
    }
    ret = devm_add_action_or_reset(&pdev.dev, mxs_ocotp_action, otp.clk);
    if (ret)
    return ret;
    ocotp_config.size = data.size;
    ocotp_config.priv = otp;
    ocotp_config.dev = dev;
    otp.nvmem = devm_nvmem_register(dev, &ocotp_config);
    if (IS_ERR(otp.nvmem))
    return PTR_ERR(otp.nvmem);
    platform_set_drvdata(pdev, otp);
    return 0;
    }
    static struct platform_driver mxs_ocotp_driver = {
    .probe = mxs_ocotp_probe,
    .driver = {
    .name = "mxs-ocotp",
    .of_match_table = mxs_ocotp_match,
    },
    };
    module_platform_driver(mxs_ocotp_driver);
    MODULE_AUTHOR("Stefan Wahren <wahrenst@gmx.net");
    MODULE_DESCRIPTION("driver for OCOTP in i.MX23/i.MX28");
    MODULE_LICENSE("GPL v2");
