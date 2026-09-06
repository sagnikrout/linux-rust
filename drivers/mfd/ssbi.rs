//! Automatically rewritten from C to Rust
//! Source: drivers/mfd/ssbi.c
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
// Copyright (c) 2009-2013, The Linux Foundation. All rights reserved.
// Copyright (c) 2010, Google Inc.
//
// Original authors: Code Aurora Forum
//
// Author: Dima Zavin <dima@android.com>
// - Largely rewritten from original to not be an i2c driver.
//

// SSBI 2.0 controller registers
pub const SSBI2_CMD: c_uint = 0x0008;
pub const SSBI2_RD: c_uint = 0x0010;
pub const SSBI2_STATUS: c_uint = 0x0014;
pub const SSBI2_MODE2: c_uint = 0x001C;
// SSBI_CMD fields

// SSBI_STATUS fields

// SSBI_MODE2 fields
pub const SSBI_MODE2_REG_ADDR_15_8_SHFT: c_uint = 0x04;

    (((MD) & 0x0F) | ((((AD) >> 8) << SSBI_MODE2_REG_ADDR_15_8_SHFT) & \
    SSBI_MODE2_REG_ADDR_15_8_MASK))
// SSBI PMIC Arbiter command registers
pub const SSBI_PA_CMD: c_uint = 0x0000;
pub const SSBI_PA_RD_STATUS: c_uint = 0x0004;
// SSBI_PA_CMD fields

pub const SSBI_PA_CMD_ADDR_MASK: c_uint = 0x7fff /* REG_ADDR_7_0, REG_ADDR_8_14*/;
// SSBI_PA_RD_STATUS fields

pub const SSBI_TIMEOUT_US: c_int = 100;
    enum ssbi_controller_type {
    MSM_SBI_CTRL_SSBI = 0,
    MSM_SBI_CTRL_SSBI2,
    MSM_SBI_CTRL_PMIC_ARBITER,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ssbi {
    pub base: *mut void __iomem,
    pub lock: spinlock_t,
    pub controller_type: enum ssbi_controller_type,
    pub len): *mut *mut *mut *mut int (read)(struct ssbi , u16 addr, u8 buf, int,
    pub len): *const *const *const *const int (write)(struct ssbi , u16 addr, u8 buf, int,
}

#[no_mangle]
pub unsafe extern "C" fn ssbi_readl(ssbi: *mut ssbi, reg: u32) -> u32 {
    static inline u32 ssbi_readl(struct ssbi *ssbi, u32 reg)
    {
    return readl(ssbi.base + reg);
    }
#[no_mangle]
pub unsafe extern "C" fn ssbi_writel(ssbi: *mut ssbi, val: u32, reg: u32) {
    static inline void ssbi_writel(struct ssbi *ssbi, u32 val, u32 reg)
    {
    writel(val, ssbi.base + reg);
    }
//
// Via private exchange with one of the original authors, the hardware
// should generally finish a transaction in about 5us.  The worst
// case, is when using the arbiter and both other CPUs have just
// started trying to use the SSBI bus will result in a time of about
// 20us.  It should never take longer than this.
//
// As such, this wait merely spins, with a udelay.
//
#[no_mangle]
unsafe extern "C" fn ssbi_wait_mask(ssbi: *mut ssbi, set_mask: u32, clr_mask: u32) -> c_int {
    static int ssbi_wait_mask(struct ssbi *ssbi, u32 set_mask, u32 clr_mask)
    {
    let mut timeout: u32 = SSBI_TIMEOUT_US;
    u32 val;
    while (timeout--) {
    val = ssbi_readl(ssbi, SSBI2_STATUS);
    if (((val & set_mask) == set_mask) && ((val & clr_mask) == 0))
    return 0;
    udelay(1);
    }
    return -ETIMEDOUT;
    }
    static int
    ssbi_read_bytes(struct ssbi *ssbi, u16 addr, u8 *buf, int len)
    {
    let mut cmd: u32 = SSBI_CMD_RDWRN | ((addr & 0xff) << 16);
    let mut ret: c_int = 0;
    if (ssbi.controller_type == MSM_SBI_CTRL_SSBI2) {
    let mut mode2: u32 = ssbi_readl(ssbi, SSBI2_MODE2);
    mode2 = SET_SSBI_MODE2_REG_ADDR_15_8(mode2, addr);
    ssbi_writel(ssbi, mode2, SSBI2_MODE2);
    }
    while (len) {
    ret = ssbi_wait_mask(ssbi, SSBI_STATUS_READY, 0);
    if (ret)
    goto err;
    ssbi_writel(ssbi, cmd, SSBI2_CMD);
    ret = ssbi_wait_mask(ssbi, SSBI_STATUS_RD_READY, 0);
    if (ret)
    goto err;
// buf++ = ssbi_readl(ssbi, SSBI2_RD) & 0xff;
    len--;
    }
    err:
    return ret;
    }
    static int
    ssbi_write_bytes(struct ssbi *ssbi, u16 addr, const u8 *buf, int len)
    {
    let mut ret: c_int = 0;
    if (ssbi.controller_type == MSM_SBI_CTRL_SSBI2) {
    let mut mode2: u32 = ssbi_readl(ssbi, SSBI2_MODE2);
    mode2 = SET_SSBI_MODE2_REG_ADDR_15_8(mode2, addr);
    ssbi_writel(ssbi, mode2, SSBI2_MODE2);
    }
    while (len) {
    ret = ssbi_wait_mask(ssbi, SSBI_STATUS_READY, 0);
    if (ret)
    goto err;
    ssbi_writel(ssbi, ((addr & 0xff) << 16) | *buf, SSBI2_CMD);
    ret = ssbi_wait_mask(ssbi, 0, SSBI_STATUS_MCHN_BUSY);
    if (ret)
    goto err;
    buf++;
    len--;
    }
    err:
    return ret;
    }
//
// See ssbi_wait_mask for an explanation of the time and the
// busywait.
//
    static inline int
    ssbi_pa_transfer(struct ssbi *ssbi, u32 cmd, u8 *data)
    {
    let mut timeout: u32 = SSBI_TIMEOUT_US;
    let mut rd_status: u32 = 0;
    ssbi_writel(ssbi, cmd, SSBI_PA_CMD);
    while (timeout--) {
    rd_status = ssbi_readl(ssbi, SSBI_PA_RD_STATUS);
    if (rd_status & SSBI_PA_RD_STATUS_TRANS_DENIED)
    return -EPERM;
    if (rd_status & SSBI_PA_RD_STATUS_TRANS_DONE) {
    if (data)
// data = rd_status & 0xff;
    return 0;
    }
    udelay(1);
    }
    return -ETIMEDOUT;
    }
    static int
    ssbi_pa_read_bytes(struct ssbi *ssbi, u16 addr, u8 *buf, int len)
    {
    u32 cmd;
    let mut ret: c_int = 0;
    cmd = SSBI_PA_CMD_RDWRN | (addr & SSBI_PA_CMD_ADDR_MASK) << 8;
    while (len) {
    ret = ssbi_pa_transfer(ssbi, cmd, buf);
    if (ret)
    goto err;
    buf++;
    len--;
    }
    err:
    return ret;
    }
    static int
    ssbi_pa_write_bytes(struct ssbi *ssbi, u16 addr, const u8 *buf, int len)
    {
    u32 cmd;
    let mut ret: c_int = 0;
    while (len) {
    cmd = (addr & SSBI_PA_CMD_ADDR_MASK) << 8 | *buf;
    ret = ssbi_pa_transfer(ssbi, cmd, core::ptr::null_mut());
    if (ret)
    goto err;
    buf++;
    len--;
    }
    err:
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn ssbi_read(dev: *mut device, addr: u16, buf: *mut u8, len: c_int) -> c_int {
    int ssbi_read(struct device *dev, u16 addr, u8 *buf, int len)
    {
    struct ssbi *ssbi = dev_get_drvdata(dev);
    unsigned long flags;
    int ret;
    spin_lock_irqsave(&ssbi.lock, flags);
    ret = ssbi.read(ssbi, addr, buf, len);
    spin_unlock_irqrestore(&ssbi.lock, flags);
    return ret;
    }
    EXPORT_SYMBOL_GPL(ssbi_read);
#[no_mangle]
pub unsafe extern "C" fn ssbi_write(dev: *mut device, addr: u16, buf: *const u8, len: c_int) -> c_int {
    int ssbi_write(struct device *dev, u16 addr, const u8 *buf, int len)
    {
    struct ssbi *ssbi = dev_get_drvdata(dev);
    unsigned long flags;
    int ret;
    spin_lock_irqsave(&ssbi.lock, flags);
    ret = ssbi.write(ssbi, addr, buf, len);
    spin_unlock_irqrestore(&ssbi.lock, flags);
    return ret;
    }
    EXPORT_SYMBOL_GPL(ssbi_write);
#[no_mangle]
unsafe extern "C" fn ssbi_probe(pdev: *mut platform_device) -> c_int {
    static int ssbi_probe(struct platform_device *pdev)
    {
    struct device_node *np = pdev.dev.of_node;
    struct ssbi *ssbi;
    const char *type;
    ssbi = devm_kzalloc(&pdev.dev, sizeof(*ssbi), GFP_KERNEL);
    if (!ssbi)
    return -ENOMEM;
    ssbi.base = devm_platform_get_and_ioremap_resource(pdev, 0, core::ptr::null_mut());
    if (IS_ERR(ssbi.base))
    return PTR_ERR(ssbi.base);
    platform_set_drvdata(pdev, ssbi);
    type = of_get_property(np, "qcom,controller-type", core::ptr::null_mut());
    if (type == core::ptr::null_mut()) {
    dev_err(&pdev.dev, "Missing qcom,controller-type property\n");
    return -EINVAL;
    }
    dev_info(&pdev.dev, "SSBI controller type: '%s'\n", type);
    if (strcmp(type, "ssbi") == 0)
    ssbi.controller_type = MSM_SBI_CTRL_SSBI;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: strcmp(type, 0: "ssbi2") ==) -> else {
    else if (strcmp(type, "ssbi2") == 0)
    ssbi.controller_type = MSM_SBI_CTRL_SSBI2;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: strcmp(type, 0: "pmic-arbiter") ==) -> else {
    else if (strcmp(type, "pmic-arbiter") == 0)
    ssbi.controller_type = MSM_SBI_CTRL_PMIC_ARBITER;
    else {
    dev_err(&pdev.dev, "Unknown qcom,controller-type\n");
    return -EINVAL;
    }
    if (ssbi.controller_type == MSM_SBI_CTRL_PMIC_ARBITER) {
    ssbi.read = ssbi_pa_read_bytes;
    ssbi.write = ssbi_pa_write_bytes;
    } else {
    ssbi.read = ssbi_read_bytes;
    ssbi.write = ssbi_write_bytes;
    }
    spin_lock_init(&ssbi.lock);
    return devm_of_platform_populate(&pdev.dev);
    }
    static const struct of_device_id ssbi_match_table[] = {
    { .compatible = "qcom,ssbi" },
    {}
    };
    MODULE_DEVICE_TABLE(of, ssbi_match_table);
    static struct platform_driver ssbi_driver = {
    .probe		= ssbi_probe,
    .driver		= {
    .name	= "ssbi",
    .of_match_table = ssbi_match_table,
    },
    };
    module_platform_driver(ssbi_driver);
    MODULE_DESCRIPTION("Qualcomm Single-wire Serial Bus Interface (SSBI) driver");
    MODULE_LICENSE("GPL v2");
    MODULE_VERSION("1.0");
    MODULE_ALIAS("platform:ssbi");
    MODULE_AUTHOR("Dima Zavin <dima@android.com>");
