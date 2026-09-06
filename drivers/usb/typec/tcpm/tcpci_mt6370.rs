//! Automatically rewritten from C to Rust
//! Source: drivers/usb/typec/tcpm/tcpci_mt6370.c
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
// Copyright (C) 2022 Richtek Technology Corp.
//
// Author: ChiYuan Huang <cy_huang@richtek.com>
//

pub const MT6370_REG_SYSCTRL8: c_uint = 0x9B;

pub const MT6370_VENDOR_ID: c_uint = 0x29CF;
pub const MT6370_TCPC_DID_A: c_uint = 0x2170;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt6370_priv {
    pub dev: *mut device,
    pub vbus: *mut regulator,
    pub tcpci: *mut tcpci,
    pub tcpci_data: tcpci_data,
}

    static const struct reg_sequence mt6370_reg_init[] = {
    REG_SEQ(0xA0, 0x1, 1000),
    REG_SEQ(0x81, 0x38, 0),
    REG_SEQ(0x82, 0x82, 0),
    REG_SEQ(0xBA, 0xFC, 0),
    REG_SEQ(0xBB, 0x50, 0),
    REG_SEQ(0x9E, 0x8F, 0),
    REG_SEQ(0xA1, 0x5, 0),
    REG_SEQ(0xA2, 0x4, 0),
    REG_SEQ(0xA3, 0x4A, 0),
    REG_SEQ(0xA4, 0x01, 0),
    REG_SEQ(0x95, 0x01, 0),
    REG_SEQ(0x80, 0x71, 0),
    REG_SEQ(0x9B, 0x3A, 1000),
    };
#[no_mangle]
unsafe extern "C" fn mt6370_tcpc_init(tcpci: *mut tcpci, data: *mut tcpci_data) -> c_int {
    static int mt6370_tcpc_init(struct tcpci *tcpci, struct tcpci_data *data)
    {
    u16 did;
    int ret;
    ret = regmap_register_patch(data.regmap, mt6370_reg_init,
    ARRAY_SIZE(mt6370_reg_init));
    if (ret)
    return ret;
    ret = regmap_raw_read(data.regmap, TCPC_BCD_DEV, &did, sizeof(u16));
    if (ret)
    return ret;
    if (did == MT6370_TCPC_DID_A)
    return regmap_write(data.regmap, TCPC_FAULT_CTRL, 0x80);
    return 0;
    }
    static int mt6370_tcpc_set_vconn(struct tcpci *tcpci, struct tcpci_data *data,
    bool enable)
    {
    return regmap_update_bits(data.regmap, MT6370_REG_SYSCTRL8,
    MT6370_AUTOIDLE_MASK,
    enable ? 0 : MT6370_AUTOIDLE_MASK);
    }
    static int mt6370_tcpc_set_vbus(struct tcpci *tcpci, struct tcpci_data *data,
    bool source, bool sink)
    {
    struct mt6370_priv *priv = container_of(data, struct mt6370_priv,
    tcpci_data);
    int ret;
    ret = regulator_is_enabled(priv.vbus);
    if (ret < 0)
    return ret;
    if (ret && !source)
    return regulator_disable(priv.vbus);
    if (!ret && source)
    return regulator_enable(priv.vbus);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mt6370_irq_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t mt6370_irq_handler(int irq, void *dev_id)
    {
    struct mt6370_priv *priv = dev_id;
    return tcpci_irq(priv.tcpci);
    }
#[no_mangle]
unsafe extern "C" fn mt6370_check_vendor_info(priv: *mut mt6370_priv) -> c_int {
    static int mt6370_check_vendor_info(struct mt6370_priv *priv)
    {
    struct regmap *regmap = priv.tcpci_data.regmap;
    u16 vid;
    int ret;
    ret = regmap_raw_read(regmap, TCPC_VENDOR_ID, &vid, sizeof(u16));
    if (ret)
    return ret;
    if (vid != MT6370_VENDOR_ID)
    return dev_err_probe(priv.dev, -ENODEV,
    "Vendor ID not correct 0x%02x\n", vid);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mt6370_unregister_tcpci_port(tcpci: *mut c_void) {
    static void mt6370_unregister_tcpci_port(void *tcpci)
    {
    tcpci_unregister_port(tcpci);
    }
#[no_mangle]
unsafe extern "C" fn mt6370_tcpc_probe(pdev: *mut platform_device) -> c_int {
    static int mt6370_tcpc_probe(struct platform_device *pdev)
    {
    struct mt6370_priv *priv;
    struct device *dev = &pdev.dev;
    int irq, ret;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.dev = dev;
    priv.tcpci_data.regmap = dev_get_regmap(dev.parent, core::ptr::null_mut());
    if (!priv.tcpci_data.regmap)
    return dev_err_probe(dev, -ENODEV, "Failed to init regmap\n");
    ret = mt6370_check_vendor_info(priv);
    if (ret)
    return ret;
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
// Assign TCPCI feature and ops
    priv.tcpci_data.auto_discharge_disconnect = 1;
    priv.tcpci_data.init = mt6370_tcpc_init;
    priv.tcpci_data.set_vconn = mt6370_tcpc_set_vconn;
    priv.vbus = devm_regulator_get_optional(dev, "vbus");
    if (!IS_ERR(priv.vbus))
    priv.tcpci_data.set_vbus = mt6370_tcpc_set_vbus;
    priv.tcpci = tcpci_register_port(dev, &priv.tcpci_data);
    if (IS_ERR(priv.tcpci))
    return dev_err_probe(dev, PTR_ERR(priv.tcpci),
    "Failed to register tcpci port\n");
    ret = devm_add_action_or_reset(dev, mt6370_unregister_tcpci_port, priv.tcpci);
    if (ret)
    return ret;
    ret = devm_request_threaded_irq(dev, irq, core::ptr::null_mut(), mt6370_irq_handler,
    IRQF_ONESHOT, dev_name(dev), priv);
    if (ret)
    return dev_err_probe(dev, ret, "Failed to allocate irq\n");
    device_init_wakeup(dev, true);
    dev_pm_set_wake_irq(dev, irq);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mt6370_tcpc_remove(pdev: *mut platform_device) {
    static void mt6370_tcpc_remove(struct platform_device *pdev)
    {
    dev_pm_clear_wake_irq(&pdev.dev);
    device_init_wakeup(&pdev.dev, false);
    }
    static const struct of_device_id mt6370_tcpc_devid_table[] = {
    { .compatible = "mediatek,mt6370-tcpc" },
    {}
    };
    MODULE_DEVICE_TABLE(of, mt6370_tcpc_devid_table);
    static struct platform_driver mt6370_tcpc_driver = {
    .driver = {
    .name = "mt6370-tcpc",
    .of_match_table = mt6370_tcpc_devid_table,
    },
    .probe = mt6370_tcpc_probe,
    .remove = mt6370_tcpc_remove,
    };
    module_platform_driver(mt6370_tcpc_driver);
    MODULE_AUTHOR("ChiYuan Huang <cy_huang@richtek.com>");
    MODULE_DESCRIPTION("MT6370 USB Type-C Port Controller Interface Driver");
    MODULE_LICENSE("GPL v2");
