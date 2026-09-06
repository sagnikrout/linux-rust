//! Automatically rewritten from C to Rust
//! Source: drivers/usb/misc/usb251xb.c
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
// Driver for Microchip USB251xB USB 2.0 Hi-Speed Hub Controller
// Configuration via SMBus.
//
// Copyright (c) 2017 SKIDATA AG
//
// This work is based on the USB3503 driver by Dongjin Kim and
// a not-accepted patch by Fabien Lahoudere, see:
// https://patchwork.kernel.org/patch/9257715
//

// Internal Register Set Addresses & Default Values acc. to DS00001692C
pub const USB251XB_ADDR_VENDOR_ID_LSB: c_uint = 0x00;
pub const USB251XB_ADDR_VENDOR_ID_MSB: c_uint = 0x01;
pub const USB251XB_DEF_VENDOR_ID: c_uint = 0x0424;
pub const USB251XB_ADDR_PRODUCT_ID_LSB: c_uint = 0x02;
pub const USB251XB_ADDR_PRODUCT_ID_MSB: c_uint = 0x03;
pub const USB251XB_ADDR_DEVICE_ID_LSB: c_uint = 0x04;
pub const USB251XB_ADDR_DEVICE_ID_MSB: c_uint = 0x05;
pub const USB251XB_DEF_DEVICE_ID: c_uint = 0x0BB3;
pub const USB251XB_ADDR_CONFIG_DATA_1: c_uint = 0x06;
pub const USB251XB_DEF_CONFIG_DATA_1: c_uint = 0x9B;
pub const USB251XB_ADDR_CONFIG_DATA_2: c_uint = 0x07;
pub const USB251XB_DEF_CONFIG_DATA_2: c_uint = 0x20;
pub const USB251XB_ADDR_CONFIG_DATA_3: c_uint = 0x08;
pub const USB251XB_DEF_CONFIG_DATA_3: c_uint = 0x02;
pub const USB251XB_ADDR_NON_REMOVABLE_DEVICES: c_uint = 0x09;
pub const USB251XB_DEF_NON_REMOVABLE_DEVICES: c_uint = 0x00;
pub const USB251XB_ADDR_PORT_DISABLE_SELF: c_uint = 0x0A;
pub const USB251XB_DEF_PORT_DISABLE_SELF: c_uint = 0x00;
pub const USB251XB_ADDR_PORT_DISABLE_BUS: c_uint = 0x0B;
pub const USB251XB_DEF_PORT_DISABLE_BUS: c_uint = 0x00;
pub const USB251XB_ADDR_MAX_POWER_SELF: c_uint = 0x0C;
pub const USB251XB_DEF_MAX_POWER_SELF: c_uint = 0x01;
pub const USB251XB_ADDR_MAX_POWER_BUS: c_uint = 0x0D;
pub const USB251XB_DEF_MAX_POWER_BUS: c_uint = 0x32;
pub const USB251XB_ADDR_MAX_CURRENT_SELF: c_uint = 0x0E;
pub const USB251XB_DEF_MAX_CURRENT_SELF: c_uint = 0x01;
pub const USB251XB_ADDR_MAX_CURRENT_BUS: c_uint = 0x0F;
pub const USB251XB_DEF_MAX_CURRENT_BUS: c_uint = 0x32;
pub const USB251XB_ADDR_POWER_ON_TIME: c_uint = 0x10;
pub const USB251XB_DEF_POWER_ON_TIME: c_uint = 0x32;
pub const USB251XB_ADDR_LANGUAGE_ID_HIGH: c_uint = 0x11;
pub const USB251XB_ADDR_LANGUAGE_ID_LOW: c_uint = 0x12;
pub const USB251XB_DEF_LANGUAGE_ID: c_uint = 0x0000;
pub const USB251XB_STRING_BUFSIZE: c_int = 62;
pub const USB251XB_ADDR_MANUFACTURER_STRING_LEN: c_uint = 0x13;
pub const USB251XB_ADDR_MANUFACTURER_STRING: c_uint = 0x16;

pub const USB251XB_ADDR_PRODUCT_STRING_LEN: c_uint = 0x14;
pub const USB251XB_ADDR_PRODUCT_STRING: c_uint = 0x54;
pub const USB251XB_ADDR_SERIAL_STRING_LEN: c_uint = 0x15;
pub const USB251XB_ADDR_SERIAL_STRING: c_uint = 0x92;

pub const USB251XB_ADDR_BATTERY_CHARGING_ENABLE: c_uint = 0xD0;
pub const USB251XB_DEF_BATTERY_CHARGING_ENABLE: c_uint = 0x00;
pub const USB251XB_ADDR_BOOST_UP: c_uint = 0xF6;
pub const USB251XB_DEF_BOOST_UP: c_uint = 0x00;
pub const USB251XB_ADDR_BOOST_57: c_uint = 0xF7;
pub const USB251XB_DEF_BOOST_57: c_uint = 0x00;
pub const USB251XB_ADDR_BOOST_14: c_uint = 0xF8;
pub const USB251XB_DEF_BOOST_14: c_uint = 0x00;
pub const USB251XB_ADDR_PORT_SWAP: c_uint = 0xFA;
pub const USB251XB_DEF_PORT_SWAP: c_uint = 0x00;
pub const USB251XB_ADDR_PORT_MAP_12: c_uint = 0xFB;
pub const USB251XB_DEF_PORT_MAP_12: c_uint = 0x00;
pub const USB251XB_ADDR_PORT_MAP_34: c_uint = 0xFC;
pub const USB251XB_DEF_PORT_MAP_34: c_uint = 0x00 /* USB251{3B/i,4B/i,7/i} only */;
pub const USB251XB_ADDR_PORT_MAP_56: c_uint = 0xFD;
pub const USB251XB_DEF_PORT_MAP_56: c_uint = 0x00 /* USB2517/i only */;
pub const USB251XB_ADDR_PORT_MAP_7: c_uint = 0xFE;
pub const USB251XB_DEF_PORT_MAP_7: c_uint = 0x00 /* USB2517/i only */;
pub const USB251XB_ADDR_STATUS_COMMAND: c_uint = 0xFF;
pub const USB251XB_STATUS_COMMAND_SMBUS_DOWN: c_uint = 0x04;
pub const USB251XB_STATUS_COMMAND_RESET: c_uint = 0x02;
pub const USB251XB_STATUS_COMMAND_ATTACH: c_uint = 0x01;
pub const USB251XB_I2C_REG_SZ: c_uint = 0x100;
pub const USB251XB_I2C_WRITE_SZ: c_uint = 0x10;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb251xb {
    pub dev: *mut device,
    pub i2c: *mut i2c_client,
    pub vdd: *mut regulator,
    pub skip_config: u8,
    pub gpio_reset: *mut gpio_desc,
    pub vendor_id: u16,
    pub product_id: u16,
    pub device_id: u16,
    pub conf_data1: u8,
    pub conf_data2: u8,
    pub conf_data3: u8,
    pub non_rem_dev: u8,
    pub port_disable_sp: u8,
    pub port_disable_bp: u8,
    pub max_power_sp: u8,
    pub max_power_bp: u8,
    pub max_current_sp: u8,
    pub max_current_bp: u8,
    pub power_on_time: u8,
    pub lang_id: u16,
    pub manufacturer_len: u8,
    pub product_len: u8,
    pub serial_len: u8,
    pub manufacturer: [c_char; USB251XB_STRING_BUFSIZE],
    pub product: [c_char; USB251XB_STRING_BUFSIZE],
    pub serial: [c_char; USB251XB_STRING_BUFSIZE],
    pub bat_charge_en: u8,
    pub boost_up: u8,
    pub boost_57: u8,
    pub boost_14: u8,
    pub port_swap: u8,
    pub port_map12: u8,
    pub port_map34: u8,
    pub port_map56: u8,
    pub port_map7: u8,
    pub status: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb251xb_data {
    pub product_id: u16,
    pub port_cnt: u8,
    pub led_support: bool,
    pub bat_support: bool,
    pub /: *mut *mut char product_str[USB251XB_STRING_BUFSIZE / 2]; / ASCII string,
}

    static const struct usb251xb_data usb2422_data = {
    .product_id = 0x2422,
    .port_cnt = 2,
    .led_support = false,
    .bat_support = true,
    .product_str = "USB2422",
    };
    static const struct usb251xb_data usb2512b_data = {
    .product_id = 0x2512,
    .port_cnt = 2,
    .led_support = false,
    .bat_support = true,
    .product_str = "USB2512B",
    };
    static const struct usb251xb_data usb2512bi_data = {
    .product_id = 0x2512,
    .port_cnt = 2,
    .led_support = false,
    .bat_support = true,
    .product_str = "USB2512Bi",
    };
    static const struct usb251xb_data usb2513b_data = {
    .product_id = 0x2513,
    .port_cnt = 3,
    .led_support = false,
    .bat_support = true,
    .product_str = "USB2513B",
    };
    static const struct usb251xb_data usb2513bi_data = {
    .product_id = 0x2513,
    .port_cnt = 3,
    .led_support = false,
    .bat_support = true,
    .product_str = "USB2513Bi",
    };
    static const struct usb251xb_data usb2514b_data = {
    .product_id = 0x2514,
    .port_cnt = 4,
    .led_support = false,
    .bat_support = true,
    .product_str = "USB2514B",
    };
    static const struct usb251xb_data usb2514bi_data = {
    .product_id = 0x2514,
    .port_cnt = 4,
    .led_support = false,
    .bat_support = true,
    .product_str = "USB2514Bi",
    };
    static const struct usb251xb_data usb2517_data = {
    .product_id = 0x2517,
    .port_cnt = 7,
    .led_support = true,
    .bat_support = false,
    .product_str = "USB2517",
    };
    static const struct usb251xb_data usb2517i_data = {
    .product_id = 0x2517,
    .port_cnt = 7,
    .led_support = true,
    .bat_support = false,
    .product_str = "USB2517i",
    };

#[no_mangle]
unsafe extern "C" fn usb251xb_check_dev_children(dev: *mut device, child: *mut c_void) -> c_int {
    static int usb251xb_check_dev_children(struct device *dev, void *child)
    {
    if (dev.type == &i2c_adapter_type) {
    return device_for_each_child(dev, child,
    usb251xb_check_dev_children);
    }
    return (dev == child);
    }
#[no_mangle]
unsafe extern "C" fn usb251x_check_gpio_chip(hub: *mut usb251xb) -> c_int {
    static int usb251x_check_gpio_chip(struct usb251xb *hub)
    {
    struct gpio_chip *gc = gpiod_to_chip(hub.gpio_reset);
    struct i2c_adapter *adap;
    int ret;
    if (!hub.i2c)
    return 0;
    if (!hub.gpio_reset)
    return 0;
    if (!gc)
    return -EINVAL;
    adap = hub.i2c.adapter;
    ret = usb251xb_check_dev_children(&adap.dev, gc.parent);
    if (ret) {
    dev_err(hub.dev, "Reset GPIO chip is at the same i2c-bus\n");
    return -EINVAL;
    }
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn usb251x_check_gpio_chip(hub: *mut usb251xb) -> c_int {
    static int usb251x_check_gpio_chip(struct usb251xb *hub)
    {
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn usb251xb_reset(hub: *mut usb251xb) {
    static void usb251xb_reset(struct usb251xb *hub)
    {
    if (!hub.gpio_reset)
    return;
    if (hub.i2c)
    i2c_lock_bus(hub.i2c.adapter, I2C_LOCK_SEGMENT);
    gpiod_set_value_cansleep(hub.gpio_reset, 1);
    usleep_range(1, 10);	/* >=1us RESET_N asserted */
    gpiod_set_value_cansleep(hub.gpio_reset, 0);
// wait for hub recovery/stabilization
    usleep_range(500, 750);	/* >=500us after RESET_N deasserted */
    if (hub.i2c)
    i2c_unlock_bus(hub.i2c.adapter, I2C_LOCK_SEGMENT);
    }
#[no_mangle]
unsafe extern "C" fn usb251xb_connect(hub: *mut usb251xb) -> c_int {
    static int usb251xb_connect(struct usb251xb *hub)
    {
    struct device *dev = hub.dev;
    int err, i;
    char i2c_wb[USB251XB_I2C_REG_SZ];
    if (!hub.i2c) {
    usb251xb_reset(hub);
    dev_info(dev, "hub is put in default configuration.\n");
    return 0;
    }
    memset(i2c_wb, 0, USB251XB_I2C_REG_SZ);
    if (hub.skip_config) {
    dev_info(dev, "Skip hub configuration, only attach.\n");
    i2c_wb[0] = 0x01;
    i2c_wb[1] = USB251XB_STATUS_COMMAND_ATTACH;
    usb251xb_reset(hub);
    err = i2c_smbus_write_i2c_block_data(hub.i2c,
    USB251XB_ADDR_STATUS_COMMAND, 2, i2c_wb);
    if (err) {
    dev_err(dev, "attaching hub failed: %d\n", err);
    return err;
    }
    return 0;
    }
    i2c_wb[USB251XB_ADDR_VENDOR_ID_MSB]     = (hub.vendor_id >> 8) & 0xFF;
    i2c_wb[USB251XB_ADDR_VENDOR_ID_LSB]     = hub.vendor_id & 0xFF;
    i2c_wb[USB251XB_ADDR_PRODUCT_ID_MSB]    = (hub.product_id >> 8) & 0xFF;
    i2c_wb[USB251XB_ADDR_PRODUCT_ID_LSB]    = hub.product_id & 0xFF;
    i2c_wb[USB251XB_ADDR_DEVICE_ID_MSB]     = (hub.device_id >> 8) & 0xFF;
    i2c_wb[USB251XB_ADDR_DEVICE_ID_LSB]     = hub.device_id & 0xFF;
    i2c_wb[USB251XB_ADDR_CONFIG_DATA_1]     = hub.conf_data1;
    i2c_wb[USB251XB_ADDR_CONFIG_DATA_2]     = hub.conf_data2;
    i2c_wb[USB251XB_ADDR_CONFIG_DATA_3]     = hub.conf_data3;
    i2c_wb[USB251XB_ADDR_NON_REMOVABLE_DEVICES] = hub.non_rem_dev;
    i2c_wb[USB251XB_ADDR_PORT_DISABLE_SELF] = hub.port_disable_sp;
    i2c_wb[USB251XB_ADDR_PORT_DISABLE_BUS]  = hub.port_disable_bp;
    i2c_wb[USB251XB_ADDR_MAX_POWER_SELF]    = hub.max_power_sp;
    i2c_wb[USB251XB_ADDR_MAX_POWER_BUS]     = hub.max_power_bp;
    i2c_wb[USB251XB_ADDR_MAX_CURRENT_SELF]  = hub.max_current_sp;
    i2c_wb[USB251XB_ADDR_MAX_CURRENT_BUS]   = hub.max_current_bp;
    i2c_wb[USB251XB_ADDR_POWER_ON_TIME]     = hub.power_on_time;
    i2c_wb[USB251XB_ADDR_LANGUAGE_ID_HIGH]  = (hub.lang_id >> 8) & 0xFF;
    i2c_wb[USB251XB_ADDR_LANGUAGE_ID_LOW]   = hub.lang_id & 0xFF;
    i2c_wb[USB251XB_ADDR_MANUFACTURER_STRING_LEN] = hub.manufacturer_len;
    i2c_wb[USB251XB_ADDR_PRODUCT_STRING_LEN]      = hub.product_len;
    i2c_wb[USB251XB_ADDR_SERIAL_STRING_LEN]       = hub.serial_len;
    memcpy(&i2c_wb[USB251XB_ADDR_MANUFACTURER_STRING], hub.manufacturer,
    USB251XB_STRING_BUFSIZE);
    memcpy(&i2c_wb[USB251XB_ADDR_SERIAL_STRING], hub.serial,
    USB251XB_STRING_BUFSIZE);
    memcpy(&i2c_wb[USB251XB_ADDR_PRODUCT_STRING], hub.product,
    USB251XB_STRING_BUFSIZE);
    i2c_wb[USB251XB_ADDR_BATTERY_CHARGING_ENABLE] = hub.bat_charge_en;
    i2c_wb[USB251XB_ADDR_BOOST_UP]          = hub.boost_up;
    i2c_wb[USB251XB_ADDR_BOOST_57]          = hub.boost_57;
    i2c_wb[USB251XB_ADDR_BOOST_14]          = hub.boost_14;
    i2c_wb[USB251XB_ADDR_PORT_SWAP]         = hub.port_swap;
    i2c_wb[USB251XB_ADDR_PORT_MAP_12]       = hub.port_map12;
    i2c_wb[USB251XB_ADDR_PORT_MAP_34]       = hub.port_map34;
    i2c_wb[USB251XB_ADDR_PORT_MAP_56]       = hub.port_map56;
    i2c_wb[USB251XB_ADDR_PORT_MAP_7]        = hub.port_map7;
    i2c_wb[USB251XB_ADDR_STATUS_COMMAND] = USB251XB_STATUS_COMMAND_ATTACH;
    usb251xb_reset(hub);
// write registers
    for (i = 0; i < (USB251XB_I2C_REG_SZ / USB251XB_I2C_WRITE_SZ); i++) {
    let mut offset: c_int = i * USB251XB_I2C_WRITE_SZ;
    char wbuf[USB251XB_I2C_WRITE_SZ + 1];
// The first data byte transferred tells the hub how many data
// bytes will follow (byte count).
//
    wbuf[0] = USB251XB_I2C_WRITE_SZ;
    memcpy(&wbuf[1], &i2c_wb[offset], USB251XB_I2C_WRITE_SZ);
    dev_dbg(dev, "writing %d byte block %d to 0x%02X\n",
    USB251XB_I2C_WRITE_SZ, i, offset);
    err = i2c_smbus_write_i2c_block_data(hub.i2c, offset,
    USB251XB_I2C_WRITE_SZ + 1,
    wbuf);
    if (err)
    goto out_err;
    }
    dev_info(dev, "Hub configuration was successful.\n");
    return 0;
    out_err:
    dev_err(dev, "configuring block %d failed: %d\n", i, err);
    return err;
    }
    static void usb251xb_get_ports_field(struct usb251xb *hub,
    const char *prop_name, u8 port_cnt,
    bool ds_only, u8 *fld)
    {
    struct device *dev = hub.dev;
    u32 port;
    of_property_for_each_u32(dev.of_node, prop_name, port) {
    if ((port >= ds_only ? 1 : 0) && (port <= port_cnt))
// fld |= BIT(port);
    else
    dev_warn(dev, "port %u doesn't exist\n", port);
    }
    }
    static int usb251xb_get_ofdata(struct usb251xb *hub,
    const struct usb251xb_data *data)
    {
    struct device *dev = hub.dev;
    struct device_node *np = dev.of_node;
    int len;
    let mut property_u32: u32 = 0;
    const char *cproperty_char;
    char str[USB251XB_STRING_BUFSIZE / 2];
    if (!np) {
    dev_err(dev, "failed to get ofdata\n");
    return -ENODEV;
    }
    hub.skip_config = of_property_read_bool(np, "skip-config");
    hub.gpio_reset = devm_gpiod_get_optional(dev, "reset", GPIOD_OUT_HIGH);
    if (IS_ERR(hub.gpio_reset))
    return dev_err_probe(dev, PTR_ERR(hub.gpio_reset),
    "unable to request GPIO reset pin\n");
    if (of_property_read_u16(np, "vendor-id", &hub.vendor_id))
    hub.vendor_id = USB251XB_DEF_VENDOR_ID;
    if (of_property_read_u16(np, "product-id", &hub.product_id))
    hub.product_id = data.product_id;
    if (of_property_read_u16(np, "device-id", &hub.device_id))
    hub.device_id = USB251XB_DEF_DEVICE_ID;
    hub.conf_data1 = USB251XB_DEF_CONFIG_DATA_1;
    if (of_property_read_bool(np, "self-powered")) {
    hub.conf_data1 |= BIT(7);
// Configure Over-Current sens when self-powered
    hub.conf_data1 &= ~BIT(2);
    if (of_property_read_bool(np, "ganged-sensing"))
    hub.conf_data1 &= ~BIT(1);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: of_property_read_bool(np, _arg: "individual-sensing")) -> else {
    else if (of_property_read_bool(np, "individual-sensing"))
    hub.conf_data1 |= BIT(1);
    } else if (of_property_read_bool(np, "bus-powered")) {
    hub.conf_data1 &= ~BIT(7);
// Disable Over-Current sense when bus-powered
    hub.conf_data1 |= BIT(2);
    }
    if (of_property_read_bool(np, "disable-hi-speed"))
    hub.conf_data1 |= BIT(5);
    if (of_property_read_bool(np, "multi-tt"))
    hub.conf_data1 |= BIT(4);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: of_property_read_bool(np, _arg: "single-tt")) -> else {
    else if (of_property_read_bool(np, "single-tt"))
    hub.conf_data1 &= ~BIT(4);
    if (of_property_read_bool(np, "disable-eop"))
    hub.conf_data1 |= BIT(3);
    if (of_property_read_bool(np, "individual-port-switching"))
    hub.conf_data1 |= BIT(0);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: of_property_read_bool(np, _arg: "ganged-port-switching")) -> else {
    else if (of_property_read_bool(np, "ganged-port-switching"))
    hub.conf_data1 &= ~BIT(0);
    hub.conf_data2 = USB251XB_DEF_CONFIG_DATA_2;
    if (of_property_read_bool(np, "dynamic-power-switching"))
    hub.conf_data2 |= BIT(7);
    if (!of_property_read_u32(np, "oc-delay-us", &property_u32)) {
    if (property_u32 == 100) {
// 100 us
    hub.conf_data2 &= ~BIT(5);
    hub.conf_data2 &= ~BIT(4);
    } else if (property_u32 == 4000) {
// 4 ms
    hub.conf_data2 &= ~BIT(5);
    hub.conf_data2 |= BIT(4);
    } else if (property_u32 == 16000) {
// 16 ms
    hub.conf_data2 |= BIT(5);
    hub.conf_data2 |= BIT(4);
    } else {
// 8 ms (DEFAULT)
    hub.conf_data2 |= BIT(5);
    hub.conf_data2 &= ~BIT(4);
    }
    }
    if (of_property_read_bool(np, "compound-device"))
    hub.conf_data2 |= BIT(3);
    hub.conf_data3 = USB251XB_DEF_CONFIG_DATA_3;
    if (of_property_read_bool(np, "port-mapping-mode"))
    hub.conf_data3 |= BIT(3);
    if (data.led_support && of_get_property(np, "led-usb-mode", core::ptr::null_mut()))
    hub.conf_data3 &= ~BIT(1);
    if (of_property_read_bool(np, "string-support"))
    hub.conf_data3 |= BIT(0);
    hub.non_rem_dev = USB251XB_DEF_NON_REMOVABLE_DEVICES;
    usb251xb_get_ports_field(hub, "non-removable-ports", data.port_cnt,
    true, &hub.non_rem_dev);
    hub.port_disable_sp = USB251XB_DEF_PORT_DISABLE_SELF;
    usb251xb_get_ports_field(hub, "sp-disabled-ports", data.port_cnt,
    true, &hub.port_disable_sp);
    hub.port_disable_bp = USB251XB_DEF_PORT_DISABLE_BUS;
    usb251xb_get_ports_field(hub, "bp-disabled-ports", data.port_cnt,
    true, &hub.port_disable_bp);
    hub.max_power_sp = USB251XB_DEF_MAX_POWER_SELF;
    if (!of_property_read_u32(np, "sp-max-total-current-microamp",
    &property_u32))
    hub.max_power_sp = min_t(u8, property_u32 / 2000, 50);
    hub.max_power_bp = USB251XB_DEF_MAX_POWER_BUS;
    if (!of_property_read_u32(np, "bp-max-total-current-microamp",
    &property_u32))
    hub.max_power_bp = min_t(u8, property_u32 / 2000, 255);
    hub.max_current_sp = USB251XB_DEF_MAX_CURRENT_SELF;
    if (!of_property_read_u32(np, "sp-max-removable-current-microamp",
    &property_u32))
    hub.max_current_sp = min_t(u8, property_u32 / 2000, 50);
    hub.max_current_bp = USB251XB_DEF_MAX_CURRENT_BUS;
    if (!of_property_read_u32(np, "bp-max-removable-current-microamp",
    &property_u32))
    hub.max_current_bp = min_t(u8, property_u32 / 2000, 255);
    hub.power_on_time = USB251XB_DEF_POWER_ON_TIME;
    if (!of_property_read_u32(np, "power-on-time-ms", &property_u32))
    hub.power_on_time = min_t(u8, property_u32 / 2, 255);
    if (of_property_read_u16(np, "language-id", &hub.lang_id))
    hub.lang_id = USB251XB_DEF_LANGUAGE_ID;
    if (of_property_read_u8(np, "boost-up", &hub.boost_up))
    hub.boost_up = USB251XB_DEF_BOOST_UP;
    cproperty_char = of_get_property(np, "manufacturer", core::ptr::null_mut());
    strscpy(str, cproperty_char ? : USB251XB_DEF_MANUFACTURER_STRING,
    sizeof(str));
    hub.manufacturer_len = strlen(str) & 0xFF;
    memset(hub.manufacturer, 0, USB251XB_STRING_BUFSIZE);
    len = min_t(size_t, USB251XB_STRING_BUFSIZE / 2, strlen(str));
    len = utf8s_to_utf16s(str, len, UTF16_LITTLE_ENDIAN,
    (wchar_t *)hub.manufacturer,
    USB251XB_STRING_BUFSIZE);
    cproperty_char = of_get_property(np, "product", core::ptr::null_mut());
    strscpy(str, cproperty_char ? : data.product_str, sizeof(str));
    hub.product_len = strlen(str) & 0xFF;
    memset(hub.product, 0, USB251XB_STRING_BUFSIZE);
    len = min_t(size_t, USB251XB_STRING_BUFSIZE / 2, strlen(str));
    len = utf8s_to_utf16s(str, len, UTF16_LITTLE_ENDIAN,
    (wchar_t *)hub.product,
    USB251XB_STRING_BUFSIZE);
    cproperty_char = of_get_property(np, "serial", core::ptr::null_mut());
    strscpy(str, cproperty_char ? : USB251XB_DEF_SERIAL_STRING,
    sizeof(str));
    hub.serial_len = strlen(str) & 0xFF;
    memset(hub.serial, 0, USB251XB_STRING_BUFSIZE);
    len = min_t(size_t, USB251XB_STRING_BUFSIZE / 2, strlen(str));
    len = utf8s_to_utf16s(str, len, UTF16_LITTLE_ENDIAN,
    (wchar_t *)hub.serial,
    USB251XB_STRING_BUFSIZE);
//
// The datasheet documents the register as 'Port Swap' but in real the
// register controls the USB DP/DM signal swapping for each port.
//
    hub.port_swap = USB251XB_DEF_PORT_SWAP;
    usb251xb_get_ports_field(hub, "swap-dx-lanes", data.port_cnt,
    false, &hub.port_swap);
// The following parameters are currently not exposed to devicetree, but
// may be as soon as needed.
//
    hub.bat_charge_en = USB251XB_DEF_BATTERY_CHARGING_ENABLE;
    hub.boost_57 = USB251XB_DEF_BOOST_57;
    hub.boost_14 = USB251XB_DEF_BOOST_14;
    hub.port_map12 = USB251XB_DEF_PORT_MAP_12;
    hub.port_map34 = USB251XB_DEF_PORT_MAP_34;
    hub.port_map56 = USB251XB_DEF_PORT_MAP_56;
    hub.port_map7  = USB251XB_DEF_PORT_MAP_7;
    return 0;
    }
    static const struct of_device_id usb251xb_of_match[] = {
    {
    .compatible = "microchip,usb2422",
    .data = &usb2422_data,
    }, {
    .compatible = "microchip,usb2512b",
    .data = &usb2512b_data,
    }, {
    .compatible = "microchip,usb2512bi",
    .data = &usb2512bi_data,
    }, {
    .compatible = "microchip,usb2513b",
    .data = &usb2513b_data,
    }, {
    .compatible = "microchip,usb2513bi",
    .data = &usb2513bi_data,
    }, {
    .compatible = "microchip,usb2514b",
    .data = &usb2514b_data,
    }, {
    .compatible = "microchip,usb2514bi",
    .data = &usb2514bi_data,
    }, {
    .compatible = "microchip,usb2517",
    .data = &usb2517_data,
    }, {
    .compatible = "microchip,usb2517i",
    .data = &usb2517i_data,
    }, {
// sentinel
    }
    };
    MODULE_DEVICE_TABLE(of, usb251xb_of_match);
#[no_mangle]
unsafe extern "C" fn usb251xb_regulator_disable_action(data: *mut c_void) {
    static void usb251xb_regulator_disable_action(void *data)
    {
    struct usb251xb *hub = data;
    regulator_disable(hub.vdd);
    }
#[no_mangle]
unsafe extern "C" fn usb251xb_probe(hub: *mut usb251xb) -> c_int {
    static int usb251xb_probe(struct usb251xb *hub)
    {
    struct device *dev = hub.dev;
    struct device_node *np = dev.of_node;
    const struct usb251xb_data *usb_data = of_device_get_match_data(dev);
    int err;
    if (np && usb_data) {
    err = usb251xb_get_ofdata(hub, usb_data);
    if (err)
    return dev_err_probe(dev, err, "failed to get ofdata\n");
    }
//
// usb251x SMBus-slave SCL lane is muxed with CFG_SEL0 pin. So if anyone
// tries to work with the bus at the moment the hub reset is released,
// it may cause an invalid config being latched by usb251x. Particularly
// one of the config modes makes the hub loading a default registers
// value without SMBus-slave interface activation. If the hub
// accidentally gets this mode, this will cause the driver SMBus-
// functions failure. Normally we could just lock the SMBus-segment the
// hub i2c-interface resides for the device-specific reset timing. But
// the GPIO controller, which is used to handle the hub reset, might be
// placed at the same i2c-bus segment. In this case an error should be
// returned since we can't safely use the GPIO controller to clear the
// reset state (it may affect the hub configuration) and we can't lock
// the i2c-bus segment (it will cause a deadlock).
//
    err = usb251x_check_gpio_chip(hub);
    if (err)
    return err;
    hub.vdd = devm_regulator_get(dev, "vdd");
    if (IS_ERR(hub.vdd))
    return PTR_ERR(hub.vdd);
    err = regulator_enable(hub.vdd);
    if (err)
    return err;
    err = devm_add_action_or_reset(dev,
    usb251xb_regulator_disable_action, hub);
    if (err)
    return err;
    err = usb251xb_connect(hub);
    if (err) {
    dev_err(dev, "Failed to connect hub (%d)\n", err);
    return err;
    }
    dev_info(dev, "Hub probed successfully\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn usb251xb_i2c_probe(i2c: *mut i2c_client) -> c_int {
    static int usb251xb_i2c_probe(struct i2c_client *i2c)
    {
    struct usb251xb *hub;
    hub = devm_kzalloc(&i2c.dev, sizeof(struct usb251xb), GFP_KERNEL);
    if (!hub)
    return -ENOMEM;
    i2c_set_clientdata(i2c, hub);
    hub.dev = &i2c.dev;
    hub.i2c = i2c;
    return usb251xb_probe(hub);
    }
#[no_mangle]
unsafe extern "C" fn usb251xb_suspend(hub: *mut usb251xb) -> c_int {
    static int usb251xb_suspend(struct usb251xb *hub)
    {
    return regulator_disable(hub.vdd);
    }
#[no_mangle]
unsafe extern "C" fn usb251xb_resume(hub: *mut usb251xb) -> c_int {
    static int usb251xb_resume(struct usb251xb *hub)
    {
    int err;
    err = regulator_enable(hub.vdd);
    if (err)
    return err;
    return usb251xb_connect(hub);
    }
#[no_mangle]
unsafe extern "C" fn usb251xb_i2c_suspend(dev: *mut device) -> c_int {
    static int usb251xb_i2c_suspend(struct device *dev)
    {
    struct i2c_client *client = to_i2c_client(dev);
    struct usb251xb *hub = i2c_get_clientdata(client);
    return usb251xb_suspend(hub);
    }
#[no_mangle]
unsafe extern "C" fn usb251xb_i2c_resume(dev: *mut device) -> c_int {
    static int usb251xb_i2c_resume(struct device *dev)
    {
    struct i2c_client *client = to_i2c_client(dev);
    struct usb251xb *hub = i2c_get_clientdata(client);
    return usb251xb_resume(hub);
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(usb251xb_i2c_pm_ops, usb251xb_i2c_suspend, usb251xb_i2c_resume);
    static const struct i2c_device_id usb251xb_id[] = {
    { .name = "usb2422" },
    { .name = "usb2512b" },
    { .name = "usb2512bi" },
    { .name = "usb2513b" },
    { .name = "usb2513bi" },
    { .name = "usb2514b" },
    { .name = "usb2514bi" },
    { .name = "usb2517" },
    { .name = "usb2517i" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(i2c, usb251xb_id);
    static struct i2c_driver usb251xb_i2c_driver = {
    .driver = {
    .name = DRIVER_NAME,
    .of_match_table = usb251xb_of_match,
    .pm = pm_sleep_ptr(&usb251xb_i2c_pm_ops),
    },
    .probe = usb251xb_i2c_probe,
    .id_table = usb251xb_id,
    };
#[no_mangle]
unsafe extern "C" fn usb251xb_plat_probe(pdev: *mut platform_device) -> c_int {
    static int usb251xb_plat_probe(struct platform_device *pdev)
    {
    struct usb251xb *hub;
    hub = devm_kzalloc(&pdev.dev, sizeof(*hub), GFP_KERNEL);
    if (!hub)
    return -ENOMEM;
    platform_set_drvdata(pdev, hub);
    hub.dev = &pdev.dev;
    return usb251xb_probe(hub);
    }
#[no_mangle]
unsafe extern "C" fn usb251xb_plat_suspend(dev: *mut device) -> c_int {
    static int usb251xb_plat_suspend(struct device *dev)
    {
    return usb251xb_suspend(dev_get_drvdata(dev));
    }
#[no_mangle]
unsafe extern "C" fn usb251xb_plat_resume(dev: *mut device) -> c_int {
    static int usb251xb_plat_resume(struct device *dev)
    {
    return usb251xb_resume(dev_get_drvdata(dev));
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(usb251xb_plat_pm_ops, usb251xb_plat_suspend, usb251xb_plat_resume);
    static struct platform_driver usb251xb_plat_driver = {
    .driver = {
    .name = DRIVER_NAME,
    .of_match_table = usb251xb_of_match,
    .pm = pm_sleep_ptr(&usb251xb_plat_pm_ops),
    },
    .probe		= usb251xb_plat_probe,
    };
#[no_mangle]
unsafe extern "C" fn usb251xb_init() -> int __init {
    static int __init usb251xb_init(void)
    {
    int err;
    err = i2c_add_driver(&usb251xb_i2c_driver);
    if (err)
    return err;
    err = platform_driver_register(&usb251xb_plat_driver);
    if (err) {
    i2c_del_driver(&usb251xb_i2c_driver);
    return err;
    }
    return 0;
    }
    module_init(usb251xb_init);
#[no_mangle]
unsafe extern "C" fn usb251xb_exit() -> void __exit {
    static void __exit usb251xb_exit(void)
    {
    platform_driver_unregister(&usb251xb_plat_driver);
    i2c_del_driver(&usb251xb_i2c_driver);
    }
    module_exit(usb251xb_exit);
    MODULE_AUTHOR("Richard Leitner <richard.leitner@skidata.com>");
    MODULE_DESCRIPTION("USB251x/xBi USB 2.0 Hub Controller Driver");
    MODULE_LICENSE("GPL");
