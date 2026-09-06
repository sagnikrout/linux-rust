//! Automatically rewritten from C to Rust
//! Source: drivers/usb/misc/usb3503.c
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
//
// Driver for SMSC USB3503 USB 2.0 hub controller driver
//
// Copyright (c) 2012-2013 Dongjin Kim (tobetter@gmail.com)
//

pub const USB3503_VIDL: c_uint = 0x00;
pub const USB3503_VIDM: c_uint = 0x01;
pub const USB3503_PIDL: c_uint = 0x02;
pub const USB3503_PIDM: c_uint = 0x03;
pub const USB3503_DIDL: c_uint = 0x04;
pub const USB3503_DIDM: c_uint = 0x05;
pub const USB3503_CFG1: c_uint = 0x06;

pub const USB3503_CFG2: c_uint = 0x07;
pub const USB3503_CFG3: c_uint = 0x08;
pub const USB3503_NRD: c_uint = 0x09;
pub const USB3503_PDS: c_uint = 0x0a;
pub const USB3503_SP_ILOCK: c_uint = 0xe7;

pub const USB3503_CFGP: c_uint = 0xee;

pub const USB3503_RESET: c_uint = 0xff;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb3503 {
    pub mode: enum usb3503_mode,
    pub regmap: *mut regmap,
    pub dev: *mut device,
    pub clk: *mut clk,
    pub port_off_mask: u8,
    pub bypass: *mut gpio_desc,
    pub intn: *mut gpio_desc,
    pub reset: *mut gpio_desc,
    pub connect: *mut gpio_desc,
    pub secondary_ref_clk: bool,
}

#[no_mangle]
unsafe extern "C" fn usb3503_connect(hub: *mut usb3503) -> c_int {
    static int usb3503_connect(struct usb3503 *hub)
    {
    struct device *dev = hub.dev;
    int err;
    if (hub.regmap) {
// SP_ILOCK: set connect_n, config_n for config
    err = regmap_write(hub.regmap, USB3503_SP_ILOCK,
    (USB3503_SPILOCK_CONNECT
    | USB3503_SPILOCK_CONFIG));
    if (err < 0) {
    dev_err(dev, "SP_ILOCK failed (%d)\n", err);
    return err;
    }
// PDS : Set the ports which are disabled in self-powered mode.
    if (hub.port_off_mask) {
    err = regmap_update_bits(hub.regmap, USB3503_PDS,
    hub.port_off_mask,
    hub.port_off_mask);
    if (err < 0) {
    dev_err(dev, "PDS failed (%d)\n", err);
    return err;
    }
    }
// CFG1 : Set SELF_BUS_PWR, this enables self-powered operation.
    err = regmap_update_bits(hub.regmap, USB3503_CFG1,
    USB3503_SELF_BUS_PWR,
    USB3503_SELF_BUS_PWR);
    if (err < 0) {
    dev_err(dev, "CFG1 failed (%d)\n", err);
    return err;
    }
// SP_LOCK: clear connect_n, config_n for hub connect
    err = regmap_update_bits(hub.regmap, USB3503_SP_ILOCK,
    (USB3503_SPILOCK_CONNECT
    | USB3503_SPILOCK_CONFIG), 0);
    if (err < 0) {
    dev_err(dev, "SP_ILOCK failed (%d)\n", err);
    return err;
    }
    }
    if (hub.connect)
    gpiod_set_value_cansleep(hub.connect, 1);
    hub.mode = USB3503_MODE_HUB;
    dev_info(dev, "switched to HUB mode\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn usb3503_switch_mode(hub: *mut usb3503, mode: enum usb3503_mode) -> c_int {
    static int usb3503_switch_mode(struct usb3503 *hub, enum usb3503_mode mode)
    {
    struct device *dev = hub.dev;
    int rst, bypass, conn;
    switch (mode) {
    case USB3503_MODE_HUB:
    conn = 1;
    rst = 0;
    bypass = 0;
    break;
    case USB3503_MODE_STANDBY:
    conn = 0;
    rst = 1;
    bypass = 1;
    dev_info(dev, "switched to STANDBY mode\n");
    break;
    case USB3503_MODE_BYPASS:
    conn = 0;
    rst = 0;
    bypass = 1;
    break;
    default:
    dev_err(dev, "unknown mode is requested\n");
    return -EINVAL;
    }
    if (!conn && hub.connect)
    gpiod_set_value_cansleep(hub.connect, 0);
    if (hub.reset)
    gpiod_set_value_cansleep(hub.reset, rst);
    if (hub.bypass)
    gpiod_set_value_cansleep(hub.bypass, bypass);
    if (conn) {
// Wait T_HUBINIT == 4ms for hub logic to stabilize
    usleep_range(4000, 10000);
    return usb3503_connect(hub);
    }
    return 0;
    }
    static const struct regmap_config usb3503_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = USB3503_RESET,
    };
#[no_mangle]
unsafe extern "C" fn usb3503_probe(hub: *mut usb3503) -> c_int {
    static int usb3503_probe(struct usb3503 *hub)
    {
    struct device *dev = hub.dev;
    struct usb3503_platform_data *pdata = dev_get_platdata(dev);
    struct device_node *np = dev.of_node;
    int err;
    let mut is_clk_enabled: bool = false;
    let mut mode: u32 = USB3503_MODE_HUB;
    const u32 *property;
    enum gpiod_flags flags;
    int len;
    if (pdata) {
    hub.port_off_mask	= pdata.port_off_mask;
    hub.mode		= pdata.initial_mode;
    } else if (np) {
    let mut rate: u32 = 0;
    hub.port_off_mask = 0;
    if (!of_property_read_u32(np, "refclk-frequency", &rate)) {
    switch (rate) {
    case 38400000:
    case 26000000:
    case 19200000:
    case 12000000:
    hub.secondary_ref_clk = 0;
    break;
    case 24000000:
    case 27000000:
    case 25000000:
    case 50000000:
    hub.secondary_ref_clk = 1;
    break;
    default:
    dev_err(dev,
    "unsupported reference clock rate (%d)\n",
    (int) rate);
    return -EINVAL;
    }
    }
    hub.clk = devm_clk_get_optional(dev, "refclk");
    if (IS_ERR(hub.clk)) {
    dev_err(dev, "unable to request refclk (%ld)\n",
    PTR_ERR(hub.clk));
    return PTR_ERR(hub.clk);
    }
    if (rate != 0) {
    err = clk_set_rate(hub.clk, rate);
    if (err) {
    dev_err(dev,
    "unable to set reference clock rate to %d\n",
    (int)rate);
    return err;
    }
    }
    err = clk_prepare_enable(hub.clk);
    if (err) {
    dev_err(dev, "unable to enable reference clock\n");
    return err;
    }
    is_clk_enabled = true;
    property = of_get_property(np, "disabled-ports", &len);
    if (property && (len / sizeof(u32)) > 0) {
    int i;
    for (i = 0; i < len / sizeof(u32); i++) {
    let mut port: u32 = be32_to_cpu(property[i]);
    if ((1 <= port) && (port <= 3))
    hub.port_off_mask |= (1 << port);
    }
    }
    of_property_read_u32(np, "initial-mode", &mode);
    hub.mode = mode;
    }
    if (hub.secondary_ref_clk)
    flags = GPIOD_OUT_LOW;
    else
    flags = GPIOD_OUT_HIGH;
    hub.intn = devm_gpiod_get_optional(dev, "intn", flags);
    if (IS_ERR(hub.intn)) {
    err = PTR_ERR(hub.intn);
    goto err_clk;
    }
    if (hub.intn)
    gpiod_set_consumer_name(hub.intn, "usb3503 intn");
    hub.connect = devm_gpiod_get_optional(dev, "connect", GPIOD_OUT_LOW);
    if (IS_ERR(hub.connect)) {
    err = PTR_ERR(hub.connect);
    goto err_clk;
    }
    if (hub.connect)
    gpiod_set_consumer_name(hub.connect, "usb3503 connect");
    hub.bypass = devm_gpiod_get_optional(dev, "bypass", GPIOD_OUT_HIGH);
    if (IS_ERR(hub.bypass)) {
    err = PTR_ERR(hub.bypass);
    goto err_clk;
    }
    if (hub.bypass)
    gpiod_set_consumer_name(hub.bypass, "usb3503 bypass");
    hub.reset = devm_gpiod_get_optional(dev, "reset", GPIOD_OUT_HIGH);
    if (IS_ERR(hub.reset)) {
    err = PTR_ERR(hub.reset);
    goto err_clk;
    }
    if (hub.reset) {
// Datasheet defines a hardware reset to be at least 100us
    usleep_range(100, 10000);
    gpiod_set_consumer_name(hub.reset, "usb3503 reset");
    }
    if (hub.port_off_mask && !hub.regmap)
    dev_err(dev, "Ports disabled with no control interface\n");
    usb3503_switch_mode(hub, hub.mode);
    dev_info(dev, "%s: probed in %s mode\n", __func__,
    (hub.mode == USB3503_MODE_HUB) ? "hub" : "standby");
    return 0;
    err_clk:
    if (is_clk_enabled)
    clk_disable_unprepare(hub.clk);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn usb3503_i2c_probe(i2c: *mut i2c_client) -> c_int {
    static int usb3503_i2c_probe(struct i2c_client *i2c)
    {
    struct usb3503 *hub;
    int err;
    hub = devm_kzalloc(&i2c.dev, sizeof(struct usb3503), GFP_KERNEL);
    if (!hub)
    return -ENOMEM;
    i2c_set_clientdata(i2c, hub);
    hub.regmap = devm_regmap_init_i2c(i2c, &usb3503_regmap_config);
    if (IS_ERR(hub.regmap)) {
    err = PTR_ERR(hub.regmap);
    dev_err(&i2c.dev, "Failed to initialise regmap: %d\n", err);
    return err;
    }
    hub.dev = &i2c.dev;
    return usb3503_probe(hub);
    }
#[no_mangle]
unsafe extern "C" fn usb3503_i2c_remove(i2c: *mut i2c_client) {
    static void usb3503_i2c_remove(struct i2c_client *i2c)
    {
    struct usb3503 *hub;
    hub = i2c_get_clientdata(i2c);
    clk_disable_unprepare(hub.clk);
    }
#[no_mangle]
unsafe extern "C" fn usb3503_platform_probe(pdev: *mut platform_device) -> c_int {
    static int usb3503_platform_probe(struct platform_device *pdev)
    {
    struct usb3503 *hub;
    hub = devm_kzalloc(&pdev.dev, sizeof(struct usb3503), GFP_KERNEL);
    if (!hub)
    return -ENOMEM;
    hub.dev = &pdev.dev;
    platform_set_drvdata(pdev, hub);
    return usb3503_probe(hub);
    }
#[no_mangle]
unsafe extern "C" fn usb3503_platform_remove(pdev: *mut platform_device) {
    static void usb3503_platform_remove(struct platform_device *pdev)
    {
    struct usb3503 *hub;
    hub = platform_get_drvdata(pdev);
    clk_disable_unprepare(hub.clk);
    }
#[no_mangle]
unsafe extern "C" fn usb3503_suspend(hub: *mut usb3503) -> int __maybe_unused {
    static int __maybe_unused usb3503_suspend(struct usb3503 *hub)
    {
    usb3503_switch_mode(hub, USB3503_MODE_STANDBY);
    clk_disable_unprepare(hub.clk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn usb3503_resume(hub: *mut usb3503) -> int __maybe_unused {
    static int __maybe_unused usb3503_resume(struct usb3503 *hub)
    {
    clk_prepare_enable(hub.clk);
    usb3503_switch_mode(hub, hub.mode);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn usb3503_i2c_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused usb3503_i2c_suspend(struct device *dev)
    {
    struct i2c_client *client = to_i2c_client(dev);
    return usb3503_suspend(i2c_get_clientdata(client));
    }
#[no_mangle]
unsafe extern "C" fn usb3503_i2c_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused usb3503_i2c_resume(struct device *dev)
    {
    struct i2c_client *client = to_i2c_client(dev);
    return usb3503_resume(i2c_get_clientdata(client));
    }
#[no_mangle]
unsafe extern "C" fn usb3503_platform_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused usb3503_platform_suspend(struct device *dev)
    {
    return usb3503_suspend(dev_get_drvdata(dev));
    }
#[no_mangle]
unsafe extern "C" fn usb3503_platform_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused usb3503_platform_resume(struct device *dev)
    {
    return usb3503_resume(dev_get_drvdata(dev));
    }
    static SIMPLE_DEV_PM_OPS(usb3503_i2c_pm_ops, usb3503_i2c_suspend,
    usb3503_i2c_resume);
    static SIMPLE_DEV_PM_OPS(usb3503_platform_pm_ops, usb3503_platform_suspend,
    usb3503_platform_resume);
    static const struct i2c_device_id usb3503_id[] = {
    { .name = USB3503_I2C_NAME },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, usb3503_id);

    static const struct of_device_id usb3503_of_match[] = {
    { .compatible = "smsc,usb3503", },
    { .compatible = "smsc,usb3503a", },
    { .compatible = "smsc,usb3803", },
    {},
    };
    MODULE_DEVICE_TABLE(of, usb3503_of_match);

    static struct i2c_driver usb3503_i2c_driver = {
    .driver = {
    .name = USB3503_I2C_NAME,
    .pm = pm_ptr(&usb3503_i2c_pm_ops),
    .of_match_table = of_match_ptr(usb3503_of_match),
    },
    .probe		= usb3503_i2c_probe,
    .remove		= usb3503_i2c_remove,
    .id_table	= usb3503_id,
    };
    static struct platform_driver usb3503_platform_driver = {
    .driver = {
    .name = USB3503_I2C_NAME,
    .of_match_table = of_match_ptr(usb3503_of_match),
    .pm = pm_ptr(&usb3503_platform_pm_ops),
    },
    .probe		= usb3503_platform_probe,
    .remove		= usb3503_platform_remove,
    };
#[no_mangle]
unsafe extern "C" fn usb3503_init() -> int __init {
    static int __init usb3503_init(void)
    {
    int err;
    err = i2c_add_driver(&usb3503_i2c_driver);
    if (err) {
    pr_err("usb3503: Failed to register I2C driver: %d\n", err);
    return err;
    }
    err = platform_driver_register(&usb3503_platform_driver);
    if (err) {
    pr_err("usb3503: Failed to register platform driver: %d\n",
    err);
    i2c_del_driver(&usb3503_i2c_driver);
    return err;
    }
    return 0;
    }
    module_init(usb3503_init);
#[no_mangle]
unsafe extern "C" fn usb3503_exit() -> void __exit {
    static void __exit usb3503_exit(void)
    {
    platform_driver_unregister(&usb3503_platform_driver);
    i2c_del_driver(&usb3503_i2c_driver);
    }
    module_exit(usb3503_exit);
    MODULE_AUTHOR("Dongjin Kim <tobetter@gmail.com>");
    MODULE_DESCRIPTION("USB3503 USB HUB driver");
    MODULE_LICENSE("GPL");
