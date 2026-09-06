//! Automatically rewritten from C to Rust
//! Source: drivers/usb/typec/tcpm/tcpci_mt6360.c
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
// Copyright (C) 2020 MediaTek Inc.
//
// Author: ChiYuan Huang <cy_huang@richtek.com>
//

pub const MT6360_REG_PHYCTRL1: c_uint = 0x80;
pub const MT6360_REG_PHYCTRL3: c_uint = 0x82;
pub const MT6360_REG_PHYCTRL7: c_uint = 0x86;
pub const MT6360_REG_VCONNCTRL1: c_uint = 0x8C;
pub const MT6360_REG_MODECTRL2: c_uint = 0x8F;
pub const MT6360_REG_SWRESET: c_uint = 0xA0;
pub const MT6360_REG_DEBCTRL1: c_uint = 0xA1;
pub const MT6360_REG_DRPCTRL1: c_uint = 0xA2;
pub const MT6360_REG_DRPCTRL2: c_uint = 0xA3;
pub const MT6360_REG_I2CTORST: c_uint = 0xBF;
pub const MT6360_REG_PHYCTRL11: c_uint = 0xCA;
pub const MT6360_REG_RXCTRL1: c_uint = 0xCE;
pub const MT6360_REG_RXCTRL2: c_uint = 0xCF;
pub const MT6360_REG_CTDCTRL2: c_uint = 0xEC;
// MT6360_REG_VCONNCTRL1

// MT6360_REG_RXCTRL2

// MT6360_REG_CTDCTRL2

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt6360_tcpc_info {
    pub tdata: tcpci_data,
    pub tcpci: *mut tcpci,
    pub dev: *mut device,
    pub irq: c_int,
}

    static inline int mt6360_tcpc_write16(struct regmap *regmap,
    unsigned int reg, u16 val)
    {
    return regmap_raw_write(regmap, reg, &val, sizeof(u16));
    }
#[no_mangle]
unsafe extern "C" fn mt6360_tcpc_init(tcpci: *mut tcpci, tdata: *mut tcpci_data) -> c_int {
    static int mt6360_tcpc_init(struct tcpci *tcpci, struct tcpci_data *tdata)
    {
    struct regmap *regmap = tdata.regmap;
    int ret;
    ret = regmap_write(regmap, MT6360_REG_SWRESET, 0x01);
    if (ret)
    return ret;
// after reset command, wait 1~2ms to wait IC action
    usleep_range(1000, 2000);
// write all alert to masked
    ret = mt6360_tcpc_write16(regmap, TCPC_ALERT_MASK, 0);
    if (ret)
    return ret;
// config I2C timeout reset enable , and timeout to 200ms
    ret = regmap_write(regmap, MT6360_REG_I2CTORST, 0x8F);
    if (ret)
    return ret;
// config CC Detect Debounce : 26.7*val us
    ret = regmap_write(regmap, MT6360_REG_DEBCTRL1, 0x10);
    if (ret)
    return ret;
// DRP Toggle Cycle : 51.2 + 6.4*val ms
    ret = regmap_write(regmap, MT6360_REG_DRPCTRL1, 4);
    if (ret)
    return ret;
// DRP Duyt Ctrl : dcSRC: /1024
    ret = mt6360_tcpc_write16(regmap, MT6360_REG_DRPCTRL2, 330);
    if (ret)
    return ret;
// Enable VCONN Current Limit function
    ret = regmap_update_bits(regmap, MT6360_REG_VCONNCTRL1, MT6360_VCONNCL_ENABLE,
    MT6360_VCONNCL_ENABLE);
    if (ret)
    return ret;
// Enable cc open 40ms when pmic send vsysuv signal
    ret = regmap_update_bits(regmap, MT6360_REG_RXCTRL2, MT6360_OPEN40M_ENABLE,
    MT6360_OPEN40M_ENABLE);
    if (ret)
    return ret;
// Enable Rpdet oneshot detection
    ret = regmap_update_bits(regmap, MT6360_REG_CTDCTRL2, MT6360_RPONESHOT_ENABLE,
    MT6360_RPONESHOT_ENABLE);
    if (ret)
    return ret;
// BMC PHY
    ret = mt6360_tcpc_write16(regmap, MT6360_REG_PHYCTRL1, 0x3A70);
    if (ret)
    return ret;
    ret = regmap_write(regmap, MT6360_REG_PHYCTRL3,  0x82);
    if (ret)
    return ret;
    ret = regmap_write(regmap, MT6360_REG_PHYCTRL7, 0x36);
    if (ret)
    return ret;
    ret = mt6360_tcpc_write16(regmap, MT6360_REG_PHYCTRL11, 0x3C60);
    if (ret)
    return ret;
    ret = regmap_write(regmap, MT6360_REG_RXCTRL1, 0xE8);
    if (ret)
    return ret;
// Set shipping mode off, AUTOIDLE on
    return regmap_write(regmap, MT6360_REG_MODECTRL2, 0x7A);
    }
#[no_mangle]
unsafe extern "C" fn mt6360_irq(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t mt6360_irq(int irq, void *dev_id)
    {
    struct mt6360_tcpc_info *mti = dev_id;
    return tcpci_irq(mti.tcpci);
    }
#[no_mangle]
unsafe extern "C" fn mt6360_tcpc_probe(pdev: *mut platform_device) -> c_int {
    static int mt6360_tcpc_probe(struct platform_device *pdev)
    {
    struct mt6360_tcpc_info *mti;
    int ret;
    mti = devm_kzalloc(&pdev.dev, sizeof(*mti), GFP_KERNEL);
    if (!mti)
    return -ENOMEM;
    mti.dev = &pdev.dev;
    mti.tdata.regmap = dev_get_regmap(pdev.dev.parent, core::ptr::null_mut());
    if (!mti.tdata.regmap) {
    dev_err(&pdev.dev, "Failed to get parent regmap\n");
    return -ENODEV;
    }
    mti.irq = platform_get_irq_byname(pdev, "PD_IRQB");
    if (mti.irq < 0)
    return mti.irq;
    mti.tdata.init = mt6360_tcpc_init;
    mti.tcpci = tcpci_register_port(&pdev.dev, &mti.tdata);
    if (IS_ERR(mti.tcpci)) {
    dev_err(&pdev.dev, "Failed to register tcpci port\n");
    return PTR_ERR(mti.tcpci);
    }
    ret = devm_request_threaded_irq(mti.dev, mti.irq, core::ptr::null_mut(), mt6360_irq, IRQF_ONESHOT,
    dev_name(&pdev.dev), mti);
    if (ret) {
    dev_err(mti.dev, "Failed to register irq\n");
    tcpci_unregister_port(mti.tcpci);
    return ret;
    }
    device_init_wakeup(&pdev.dev, true);
    platform_set_drvdata(pdev, mti);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mt6360_tcpc_remove(pdev: *mut platform_device) {
    static void mt6360_tcpc_remove(struct platform_device *pdev)
    {
    struct mt6360_tcpc_info *mti = platform_get_drvdata(pdev);
    disable_irq(mti.irq);
    tcpci_unregister_port(mti.tcpci);
    }
#[no_mangle]
unsafe extern "C" fn mt6360_tcpc_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused mt6360_tcpc_suspend(struct device *dev)
    {
    struct mt6360_tcpc_info *mti = dev_get_drvdata(dev);
    if (device_may_wakeup(dev))
    enable_irq_wake(mti.irq);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mt6360_tcpc_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused mt6360_tcpc_resume(struct device *dev)
    {
    struct mt6360_tcpc_info *mti = dev_get_drvdata(dev);
    if (device_may_wakeup(dev))
    disable_irq_wake(mti.irq);
    return 0;
    }
    static SIMPLE_DEV_PM_OPS(mt6360_tcpc_pm_ops, mt6360_tcpc_suspend, mt6360_tcpc_resume);
    static const struct of_device_id __maybe_unused mt6360_tcpc_of_id[] = {
    { .compatible = "mediatek,mt6360-tcpc", },
    {},
    };
    MODULE_DEVICE_TABLE(of, mt6360_tcpc_of_id);
    static struct platform_driver mt6360_tcpc_driver = {
    .driver = {
    .name = "mt6360-tcpc",
    .pm = &mt6360_tcpc_pm_ops,
    .of_match_table = mt6360_tcpc_of_id,
    },
    .probe = mt6360_tcpc_probe,
    .remove = mt6360_tcpc_remove,
    };
    module_platform_driver(mt6360_tcpc_driver);
    MODULE_AUTHOR("ChiYuan Huang <cy_huang@richtek.com>");
    MODULE_DESCRIPTION("MT6360 USB Type-C Port Controller Interface Driver");
    MODULE_LICENSE("GPL v2");
