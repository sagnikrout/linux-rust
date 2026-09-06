//! Automatically rewritten from C to Rust
//! Source: drivers/usb/phy/phy-isp1301.c
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
// NXP ISP1301 USB transceiver driver
//
// Copyright (C) 2012 Roland Stigge
//
// Author: Roland Stigge <stigge@antcom.de>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct isp1301 {
    pub phy: usb_phy,
    pub mutex: mutex,
    pub client: *mut i2c_client,
}

    static const struct i2c_device_id isp1301_id[] = {
    { .name = "isp1301" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, isp1301_id);
    static const struct of_device_id isp1301_of_match[] = {
    {.compatible = "nxp,isp1301" },
    { },
    };
    MODULE_DEVICE_TABLE(of, isp1301_of_match);
    static struct i2c_client *isp1301_i2c_client;
#[no_mangle]
unsafe extern "C" fn __isp1301_write(isp: *mut isp1301, reg: u8, value: u8, clear: u8) -> c_int {
    static int __isp1301_write(struct isp1301 *isp, u8 reg, u8 value, u8 clear)
    {
    return i2c_smbus_write_byte_data(isp.client, reg | clear, value);
    }
#[no_mangle]
unsafe extern "C" fn isp1301_write(isp: *mut isp1301, reg: u8, value: u8) -> c_int {
    static int isp1301_write(struct isp1301 *isp, u8 reg, u8 value)
    {
    return __isp1301_write(isp, reg, value, 0);
    }
#[no_mangle]
unsafe extern "C" fn isp1301_clear(isp: *mut isp1301, reg: u8, value: u8) -> c_int {
    static int isp1301_clear(struct isp1301 *isp, u8 reg, u8 value)
    {
    return __isp1301_write(isp, reg, value, ISP1301_I2C_REG_CLEAR_ADDR);
    }
#[no_mangle]
unsafe extern "C" fn isp1301_phy_init(phy: *mut usb_phy) -> c_int {
    static int isp1301_phy_init(struct usb_phy *phy)
    {
    struct isp1301 *isp = phy_to_isp(phy);
// Disable transparent UART mode first
    isp1301_clear(isp, ISP1301_I2C_MODE_CONTROL_1, MC1_UART_EN);
    isp1301_clear(isp, ISP1301_I2C_MODE_CONTROL_1, ~MC1_SPEED_REG);
    isp1301_write(isp, ISP1301_I2C_MODE_CONTROL_1, MC1_SPEED_REG);
    isp1301_clear(isp, ISP1301_I2C_MODE_CONTROL_2, ~0);
    isp1301_write(isp, ISP1301_I2C_MODE_CONTROL_2, (MC2_BI_DI | MC2_PSW_EN
    | MC2_SPD_SUSP_CTRL));
    isp1301_clear(isp, ISP1301_I2C_OTG_CONTROL_1, ~0);
    isp1301_write(isp, ISP1301_I2C_MODE_CONTROL_1, MC1_DAT_SE0);
    isp1301_write(isp, ISP1301_I2C_OTG_CONTROL_1, (OTG1_DM_PULLDOWN
    | OTG1_DP_PULLDOWN));
    isp1301_clear(isp, ISP1301_I2C_OTG_CONTROL_1, (OTG1_DM_PULLUP
    | OTG1_DP_PULLUP));
// mask all interrupts
    isp1301_clear(isp, ISP1301_I2C_INTERRUPT_LATCH, ~0);
    isp1301_clear(isp, ISP1301_I2C_INTERRUPT_FALLING, ~0);
    isp1301_clear(isp, ISP1301_I2C_INTERRUPT_RISING, ~0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn isp1301_phy_set_vbus(phy: *mut usb_phy, on: c_int) -> c_int {
    static int isp1301_phy_set_vbus(struct usb_phy *phy, int on)
    {
    struct isp1301 *isp = phy_to_isp(phy);
    if (on)
    isp1301_write(isp, ISP1301_I2C_OTG_CONTROL_1, OTG1_VBUS_DRV);
    else
    isp1301_clear(isp, ISP1301_I2C_OTG_CONTROL_1, OTG1_VBUS_DRV);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn isp1301_probe(client: *mut i2c_client) -> c_int {
    static int isp1301_probe(struct i2c_client *client)
    {
    struct isp1301 *isp;
    struct usb_phy *phy;
    isp = devm_kzalloc(&client.dev, sizeof(*isp), GFP_KERNEL);
    if (!isp)
    return -ENOMEM;
    isp.client = client;
    mutex_init(&isp.mutex);
    phy = &isp.phy;
    phy.dev = &client.dev;
    phy.label = DRV_NAME;
    phy.init = isp1301_phy_init;
    phy.set_vbus = isp1301_phy_set_vbus;
    phy.type = USB_PHY_TYPE_USB2;
    i2c_set_clientdata(client, isp);
    usb_add_phy_dev(phy);
    isp1301_i2c_client = client;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn isp1301_remove(client: *mut i2c_client) {
    static void isp1301_remove(struct i2c_client *client)
    {
    struct isp1301 *isp = i2c_get_clientdata(client);
    usb_remove_phy(&isp.phy);
    isp1301_i2c_client = core::ptr::null_mut();
    }
    static struct i2c_driver isp1301_driver = {
    .driver = {
    .name = DRV_NAME,
    .of_match_table = isp1301_of_match,
    },
    .probe = isp1301_probe,
    .remove = isp1301_remove,
    .id_table = isp1301_id,
    };
    module_i2c_driver(isp1301_driver);
    struct i2c_client *isp1301_get_client(struct device_node *node)
    {
    struct i2c_client *client;
// reference of ISP1301 I2C node via DT
    client = of_find_i2c_device_by_node(node);
    if (client)
    return client;
// non-DT: only one ISP1301 chip supported
    if (isp1301_i2c_client) {
    get_device(&isp1301_i2c_client.dev);
    return isp1301_i2c_client;
    }
    return core::ptr::null_mut();
    }
    EXPORT_SYMBOL_GPL(isp1301_get_client);
    MODULE_AUTHOR("Roland Stigge <stigge@antcom.de>");
    MODULE_DESCRIPTION("NXP ISP1301 USB transceiver driver");
    MODULE_LICENSE("GPL");
