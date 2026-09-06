//! Automatically rewritten from C to Rust
//! Source: drivers/usb/typec/wusb3801.c
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
// Willsemi WUSB3801 Type-C port controller driver
//
// Copyright (C) 2022 Samuel Holland <samuel@sholland.org>
//

pub const WUSB3801_REG_DEVICE_ID: c_uint = 0x01;
pub const WUSB3801_REG_CTRL0: c_uint = 0x02;
pub const WUSB3801_REG_INT: c_uint = 0x03;
pub const WUSB3801_REG_STAT: c_uint = 0x04;
pub const WUSB3801_REG_CTRL1: c_uint = 0x05;
pub const WUSB3801_REG_TEST00: c_uint = 0x06;
pub const WUSB3801_REG_TEST01: c_uint = 0x07;
pub const WUSB3801_REG_TEST02: c_uint = 0x08;
pub const WUSB3801_REG_TEST03: c_uint = 0x09;
pub const WUSB3801_REG_TEST04: c_uint = 0x0a;
pub const WUSB3801_REG_TEST05: c_uint = 0x0b;
pub const WUSB3801_REG_TEST06: c_uint = 0x0c;
pub const WUSB3801_REG_TEST07: c_uint = 0x0d;
pub const WUSB3801_REG_TEST08: c_uint = 0x0e;
pub const WUSB3801_REG_TEST09: c_uint = 0x0f;
pub const WUSB3801_REG_TEST0A: c_uint = 0x10;
pub const WUSB3801_REG_TEST0B: c_uint = 0x11;
pub const WUSB3801_REG_TEST0C: c_uint = 0x12;
pub const WUSB3801_REG_TEST0D: c_uint = 0x13;
pub const WUSB3801_REG_TEST0E: c_uint = 0x14;
pub const WUSB3801_REG_TEST0F: c_uint = 0x15;
pub const WUSB3801_REG_TEST10: c_uint = 0x16;
pub const WUSB3801_REG_TEST11: c_uint = 0x17;
pub const WUSB3801_REG_TEST12: c_uint = 0x18;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wusb3801 {
    pub cap: typec_capability,
    pub dev: *mut device,
    pub partner: *mut typec_partner,
    pub port: *mut typec_port,
    pub regmap: *mut regmap,
    pub vbus_supply: *mut regulator,
    pub partner_type: c_uint,
    pub port_type: enum typec_port_type,
    pub pwr_opmode: enum typec_pwr_opmode,
    pub vbus_on: bool,
}

#[no_mangle]
unsafe extern "C" fn wusb3801_get_default_role(wusb3801: *mut wusb3801) -> enum typec_role {
    static enum typec_role wusb3801_get_default_role(struct wusb3801 *wusb3801)
    {
    switch (wusb3801.port_type) {
    case TYPEC_PORT_SRC:
    return TYPEC_SOURCE;
    case TYPEC_PORT_SNK:
    return TYPEC_SINK;
    case TYPEC_PORT_DRP:
    default:
    if (wusb3801.cap.prefer_role == TYPEC_SOURCE)
    return TYPEC_SOURCE;
    return TYPEC_SINK;
    }
    }
#[no_mangle]
unsafe extern "C" fn wusb3801_map_port_type(type: enum typec_port_type) -> c_int {
    static int wusb3801_map_port_type(enum typec_port_type type)
    {
    switch (type) {
    case TYPEC_PORT_SRC:
    return WUSB3801_CTRL0_ROLE_SRC;
    case TYPEC_PORT_SNK:
    return WUSB3801_CTRL0_ROLE_SNK;
    case TYPEC_PORT_DRP:
    default:
    return WUSB3801_CTRL0_ROLE_DRP;
    }
    }
#[no_mangle]
unsafe extern "C" fn wusb3801_map_pwr_opmode(mode: enum typec_pwr_opmode) -> c_int {
    static int wusb3801_map_pwr_opmode(enum typec_pwr_opmode mode)
    {
    switch (mode) {
    case TYPEC_PWR_MODE_USB:
    default:
    return WUSB3801_CTRL0_CURRENT_DEFAULT;
    case TYPEC_PWR_MODE_1_5A:
    return WUSB3801_CTRL0_CURRENT_1_5A;
    case TYPEC_PWR_MODE_3_0A:
    return WUSB3801_CTRL0_CURRENT_3_0A;
    }
    }
#[no_mangle]
unsafe extern "C" fn wusb3801_map_try_role(role: c_int) -> c_uint {
    static unsigned int wusb3801_map_try_role(int role)
    {
    switch (role) {
    case TYPEC_NO_PREFERRED_ROLE:
    default:
    return WUSB3801_CTRL0_TRY_NONE;
    case TYPEC_SINK:
    return WUSB3801_CTRL0_TRY_SNK;
    case TYPEC_SOURCE:
    return WUSB3801_CTRL0_TRY_SRC;
    }
    }
#[no_mangle]
unsafe extern "C" fn wusb3801_unmap_orientation(status: c_uint) -> enum typec_orientation {
    static enum typec_orientation wusb3801_unmap_orientation(unsigned int status)
    {
    switch (status & WUSB3801_STAT_ORIENTATION) {
    case WUSB3801_STAT_ORIENTATION_NONE:
    case WUSB3801_STAT_ORIENTATION_BOTH:
    default:
    return TYPEC_ORIENTATION_NONE;
    case WUSB3801_STAT_ORIENTATION_CC1:
    return TYPEC_ORIENTATION_NORMAL;
    case WUSB3801_STAT_ORIENTATION_CC2:
    return TYPEC_ORIENTATION_REVERSE;
    }
    }
#[no_mangle]
unsafe extern "C" fn wusb3801_unmap_pwr_opmode(status: c_uint) -> enum typec_pwr_opmode {
    static enum typec_pwr_opmode wusb3801_unmap_pwr_opmode(unsigned int status)
    {
    switch (status & WUSB3801_STAT_CURRENT) {
    case WUSB3801_STAT_CURRENT_STANDBY:
    case WUSB3801_STAT_CURRENT_DEFAULT:
    default:
    return TYPEC_PWR_MODE_USB;
    case WUSB3801_STAT_CURRENT_1_5A:
    return TYPEC_PWR_MODE_1_5A;
    case WUSB3801_STAT_CURRENT_3_0A:
    return TYPEC_PWR_MODE_3_0A;
    }
    }
#[no_mangle]
unsafe extern "C" fn wusb3801_try_role(port: *mut typec_port, role: c_int) -> c_int {
    static int wusb3801_try_role(struct typec_port *port, int role)
    {
    struct wusb3801 *wusb3801 = typec_get_drvdata(port);
    return regmap_update_bits(wusb3801.regmap, WUSB3801_REG_CTRL0,
    WUSB3801_CTRL0_TRY,
    wusb3801_map_try_role(role));
    }
    static int wusb3801_port_type_set(struct typec_port *port,
    enum typec_port_type type)
    {
    struct wusb3801 *wusb3801 = typec_get_drvdata(port);
    int ret;
    ret = regmap_update_bits(wusb3801.regmap, WUSB3801_REG_CTRL0,
    WUSB3801_CTRL0_ROLE,
    wusb3801_map_port_type(type));
    if (ret)
    return ret;
    wusb3801.port_type = type;
    return 0;
    }
    static const struct typec_operations wusb3801_typec_ops = {
    .try_role	= wusb3801_try_role,
    .port_type_set	= wusb3801_port_type_set,
    };
#[no_mangle]
unsafe extern "C" fn wusb3801_hw_init(wusb3801: *mut wusb3801) -> c_int {
    static int wusb3801_hw_init(struct wusb3801 *wusb3801)
    {
    return regmap_write(wusb3801.regmap, WUSB3801_REG_CTRL0,
    wusb3801_map_try_role(wusb3801.cap.prefer_role) |
    wusb3801_map_pwr_opmode(wusb3801.pwr_opmode) |
    wusb3801_map_port_type(wusb3801.port_type));
    }
#[no_mangle]
unsafe extern "C" fn wusb3801_hw_update(wusb3801: *mut wusb3801) {
    static void wusb3801_hw_update(struct wusb3801 *wusb3801)
    {
    struct typec_port *port = wusb3801.port;
    struct device *dev = wusb3801.dev;
    unsigned int partner_type, status;
    int ret;
    ret = regmap_read(wusb3801.regmap, WUSB3801_REG_STAT, &status);
    if (ret) {
    dev_warn(dev, "Failed to read port status: %d\n", ret);
    status = 0;
    }
    dev_dbg(dev, "status = 0x%02x\n", status);
    partner_type = status & WUSB3801_STAT_PARTNER;
    if (partner_type == WUSB3801_STAT_PARTNER_SNK) {
    if (!wusb3801.vbus_on) {
    ret = regulator_enable(wusb3801.vbus_supply);
    if (ret)
    dev_warn(dev, "Failed to enable VBUS: %d\n", ret);
    wusb3801.vbus_on = true;
    }
    } else {
    if (wusb3801.vbus_on) {
    regulator_disable(wusb3801.vbus_supply);
    wusb3801.vbus_on = false;
    }
    }
    if (partner_type != wusb3801.partner_type) {
    let mut desc: typec_partner_desc = {};
    enum typec_data_role data_role;
    let mut pwr_role: enum typec_role = wusb3801_get_default_role(wusb3801);
    switch (partner_type) {
    case WUSB3801_STAT_PARTNER_STANDBY:
    break;
    case WUSB3801_STAT_PARTNER_SNK:
    pwr_role = TYPEC_SOURCE;
    break;
    case WUSB3801_STAT_PARTNER_SRC:
    pwr_role = TYPEC_SINK;
    break;
    case WUSB3801_STAT_PARTNER_AUDIO:
    desc.accessory = TYPEC_ACCESSORY_AUDIO;
    break;
    case WUSB3801_STAT_PARTNER_DEBUG:
    desc.accessory = TYPEC_ACCESSORY_DEBUG;
    break;
    }
    if (wusb3801.partner) {
    typec_unregister_partner(wusb3801.partner);
    wusb3801.partner = core::ptr::null_mut();
    }
    if (partner_type != WUSB3801_STAT_PARTNER_STANDBY) {
    wusb3801.partner = typec_register_partner(port, &desc);
    if (IS_ERR(wusb3801.partner))
    dev_err(dev, "Failed to register partner: %ld\n",
    PTR_ERR(wusb3801.partner));
    }
    data_role = pwr_role == TYPEC_SOURCE ? TYPEC_HOST : TYPEC_DEVICE;
    typec_set_data_role(port, data_role);
    typec_set_pwr_role(port, pwr_role);
    typec_set_vconn_role(port, pwr_role);
    }
    typec_set_pwr_opmode(wusb3801.port,
    partner_type == WUSB3801_STAT_PARTNER_SRC
    ? wusb3801_unmap_pwr_opmode(status)
    : wusb3801.pwr_opmode);
    typec_set_orientation(wusb3801.port,
    wusb3801_unmap_orientation(status));
    wusb3801.partner_type = partner_type;
    }
#[no_mangle]
unsafe extern "C" fn wusb3801_irq(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t wusb3801_irq(int irq, void *data)
    {
    struct wusb3801 *wusb3801 = data;
    unsigned int dummy;
//
// The interrupt register must be read in order to clear the IRQ,
// but all of the useful information is in the status register.
//
    regmap_read(wusb3801.regmap, WUSB3801_REG_INT, &dummy);
    wusb3801_hw_update(wusb3801);
    return IRQ_HANDLED;
    }
    static const struct regmap_config config = {
    .reg_bits	= 8,
    .val_bits	= 8,
    .max_register	= WUSB3801_REG_TEST12,
    };
#[no_mangle]
unsafe extern "C" fn wusb3801_probe(client: *mut i2c_client) -> c_int {
    static int wusb3801_probe(struct i2c_client *client)
    {
    struct device *dev = &client.dev;
    struct fwnode_handle *connector;
    struct wusb3801 *wusb3801;
    const char *cap_str;
    int ret;
    wusb3801 = devm_kzalloc(dev, sizeof(*wusb3801), GFP_KERNEL);
    if (!wusb3801)
    return -ENOMEM;
    i2c_set_clientdata(client, wusb3801);
    wusb3801.dev = dev;
    wusb3801.regmap = devm_regmap_init_i2c(client, &config);
    if (IS_ERR(wusb3801.regmap))
    return PTR_ERR(wusb3801.regmap);
    wusb3801.vbus_supply = devm_regulator_get(dev, "vbus");
    if (IS_ERR(wusb3801.vbus_supply))
    return PTR_ERR(wusb3801.vbus_supply);
    connector = device_get_named_child_node(dev, "connector");
    if (!connector)
    return -ENODEV;
    ret = typec_get_fw_cap(&wusb3801.cap, connector);
    if (ret)
    goto err_put_connector;
    wusb3801.port_type = wusb3801.cap.type;
    ret = fwnode_property_read_string(connector, "typec-power-opmode", &cap_str);
    if (ret)
    goto err_put_connector;
    ret = typec_find_pwr_opmode(cap_str);
    if (ret < 0 || ret == TYPEC_PWR_MODE_PD)
    goto err_put_connector;
    wusb3801.pwr_opmode = ret;
// Initialize the hardware with the devicetree settings.
    ret = wusb3801_hw_init(wusb3801);
    if (ret)
    goto err_put_connector;
    wusb3801.cap.revision		= USB_TYPEC_REV_1_2;
    wusb3801.cap.accessory[0]	= TYPEC_ACCESSORY_AUDIO;
    wusb3801.cap.accessory[1]	= TYPEC_ACCESSORY_DEBUG;
    wusb3801.cap.orientation_aware	= true;
    wusb3801.cap.driver_data	= wusb3801;
    wusb3801.cap.ops		= &wusb3801_typec_ops;
    wusb3801.port = typec_register_port(dev, &wusb3801.cap);
    if (IS_ERR(wusb3801.port)) {
    ret = PTR_ERR(wusb3801.port);
    goto err_put_connector;
    }
// Initialize the port attributes from the hardware state.
    wusb3801_hw_update(wusb3801);
    ret = request_threaded_irq(client.irq, core::ptr::null_mut(), wusb3801_irq,
    IRQF_ONESHOT, dev_name(dev), wusb3801);
    if (ret)
    goto err_unregister_port;
    fwnode_handle_put(connector);
    return 0;
    err_unregister_port:
    typec_unregister_port(wusb3801.port);
    err_put_connector:
    fwnode_handle_put(connector);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn wusb3801_remove(client: *mut i2c_client) {
    static void wusb3801_remove(struct i2c_client *client)
    {
    struct wusb3801 *wusb3801 = i2c_get_clientdata(client);
    free_irq(client.irq, wusb3801);
    if (wusb3801.partner)
    typec_unregister_partner(wusb3801.partner);
    typec_unregister_port(wusb3801.port);
    if (wusb3801.vbus_on)
    regulator_disable(wusb3801.vbus_supply);
    }
    static const struct of_device_id wusb3801_of_match[] = {
    { .compatible = "willsemi,wusb3801" },
    {}
    };
    MODULE_DEVICE_TABLE(of, wusb3801_of_match);
    static struct i2c_driver wusb3801_driver = {
    .probe		= wusb3801_probe,
    .remove		= wusb3801_remove,
    .driver		= {
    .name		= "wusb3801",
    .of_match_table	= wusb3801_of_match,
    },
    };
    module_i2c_driver(wusb3801_driver);
    MODULE_AUTHOR("Samuel Holland <samuel@sholland.org>");
    MODULE_DESCRIPTION("Willsemi WUSB3801 Type-C port controller driver");
    MODULE_LICENSE("GPL");
