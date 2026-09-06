//! Automatically rewritten from C to Rust
//! Source: drivers/video/backlight/arcxcnn_bl.c
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
// Backlight driver for ArcticSand ARC_X_C_0N_0N Devices
//
// Copyright 2016 ArcticSand, Inc.
// Author : Brian Dodge <bdodge@arcticsand.com>
//

    enum arcxcnn_chip_id {
    ARC2C0608
    };
//
// struct arcxcnn_platform_data
// @name		: Backlight driver name (NULL will use default)
// @initial_brightness	: initial value of backlight brightness
// @leden		: initial LED string enables, upper bit is global on/off
// @led_config_0	: fading speed (period between intensity steps)
// @led_config_1	: misc settings, see datasheet
// @dim_freq		: pwm dimming frequency if in pwm mode
// @comp_config		: misc config, see datasheet
// @filter_config	: RC/PWM filter config, see datasheet
// @trim_config		: full scale current trim, see datasheet
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arcxcnn_platform_data {
    pub name: *const c_char,
    pub initial_brightness: u16,
    pub leden: u8,
    pub led_config_0: u8,
    pub led_config_1: u8,
    pub dim_freq: u8,
    pub comp_config: u8,
    pub filter_config: u8,
    pub trim_config: u8,
}

pub const ARCXCNN_CMD: c_uint = 0x00	/* Command Register */;
pub const ARCXCNN_CMD_STDBY: c_uint = 0x80	/*   I2C Standby */;
pub const ARCXCNN_CMD_RESET: c_uint = 0x40	/*   Reset */;
pub const ARCXCNN_CMD_BOOST: c_uint = 0x10	/*   Boost */;
pub const ARCXCNN_CMD_OVP_MASK: c_uint = 0x0C	/*   --- Over Voltage Threshold */;
pub const ARCXCNN_CMD_OVP_XXV: c_uint = 0x0C	/*   <rsvrd> Over Voltage Threshold */;
pub const ARCXCNN_CMD_OVP_20V: c_uint = 0x08	/*   20v Over Voltage Threshold */;
pub const ARCXCNN_CMD_OVP_24V: c_uint = 0x04	/*   24v Over Voltage Threshold */;
pub const ARCXCNN_CMD_OVP_31V: c_uint = 0x00	/*   31.4v Over Voltage Threshold */;
pub const ARCXCNN_CMD_EXT_COMP: c_uint = 0x01	/*   part (0) or full (1) ext. comp */;
pub const ARCXCNN_CONFIG: c_uint = 0x01	/* Configuration */;
pub const ARCXCNN_STATUS1: c_uint = 0x02	/* Status 1 */;
pub const ARCXCNN_STATUS2: c_uint = 0x03	/* Status 2 */;
pub const ARCXCNN_FADECTRL: c_uint = 0x04	/* Fading Control */;
pub const ARCXCNN_ILED_CONFIG: c_uint = 0x05	/* ILED Configuration */;
pub const ARCXCNN_ILED_DIM_PWM: c_uint = 0x00	/*   config dim mode pwm */;
pub const ARCXCNN_ILED_DIM_INT: c_uint = 0x04	/*   config dim mode internal */;
pub const ARCXCNN_LEDEN: c_uint = 0x06	/* LED Enable Register */;
pub const ARCXCNN_LEDEN_ISETEXT: c_uint = 0x80	/*   Full-scale current set extern */;
pub const ARCXCNN_LEDEN_MASK: c_uint = 0x3F	/*   LED string enables mask */;
pub const ARCXCNN_LEDEN_BITS: c_uint = 0x06	/*   Bits of LED string enables */;
pub const ARCXCNN_LEDEN_LED1: c_uint = 0x01;
pub const ARCXCNN_LEDEN_LED2: c_uint = 0x02;
pub const ARCXCNN_LEDEN_LED3: c_uint = 0x04;
pub const ARCXCNN_LEDEN_LED4: c_uint = 0x08;
pub const ARCXCNN_LEDEN_LED5: c_uint = 0x10;
pub const ARCXCNN_LEDEN_LED6: c_uint = 0x20;
pub const ARCXCNN_WLED_ISET_LSB: c_uint = 0x07	/* LED ISET LSB (in upper nibble) */;
pub const ARCXCNN_WLED_ISET_LSB_SHIFT: c_uint = 0x04  /* ISET LSB Left Shift */;
pub const ARCXCNN_WLED_ISET_MSB: c_uint = 0x08	/* LED ISET MSB (8 bits) */;
pub const ARCXCNN_DIMFREQ: c_uint = 0x09;
pub const ARCXCNN_COMP_CONFIG: c_uint = 0x0A;
pub const ARCXCNN_FILT_CONFIG: c_uint = 0x0B;
pub const ARCXCNN_IMAXTUNE: c_uint = 0x0C;
pub const ARCXCNN_ID_MSB: c_uint = 0x1E;
pub const ARCXCNN_ID_LSB: c_uint = 0x1F;
pub const MAX_BRIGHTNESS: c_int = 4095;
pub const INIT_BRIGHT: c_int = 60;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arcxcnn {
    pub client: *mut i2c_client,
    pub bl: *mut backlight_device,
    pub dev: *mut device,
    pub pdata: *mut arcxcnn_platform_data,
}

#[no_mangle]
unsafe extern "C" fn arcxcnn_update_field(lp: *mut arcxcnn, reg: u8, mask: u8, data: u8) -> c_int {
    static int arcxcnn_update_field(struct arcxcnn *lp, u8 reg, u8 mask, u8 data)
    {
    int ret;
    u8 tmp;
    ret = i2c_smbus_read_byte_data(lp.client, reg);
    if (ret < 0) {
    dev_err(lp.dev, "failed to read 0x%.2x\n", reg);
    return ret;
    }
    tmp = (u8)ret;
    tmp &= ~mask;
    tmp |= data & mask;
    return i2c_smbus_write_byte_data(lp.client, reg, tmp);
    }
#[no_mangle]
unsafe extern "C" fn arcxcnn_set_brightness(lp: *mut arcxcnn, brightness: u32) -> c_int {
    static int arcxcnn_set_brightness(struct arcxcnn *lp, u32 brightness)
    {
    int ret;
    u8 val;
// lower nibble of brightness goes in upper nibble of LSB register
    val = (brightness & 0xF) << ARCXCNN_WLED_ISET_LSB_SHIFT;
    ret = i2c_smbus_write_byte_data(lp.client,
    ARCXCNN_WLED_ISET_LSB, val);
    if (ret < 0)
    return ret;
// remaining 8 bits of brightness go in MSB register
    val = (brightness >> 4);
    return i2c_smbus_write_byte_data(lp.client,
    ARCXCNN_WLED_ISET_MSB, val);
    }
#[no_mangle]
unsafe extern "C" fn arcxcnn_bl_update_status(bl: *mut backlight_device) -> c_int {
    static int arcxcnn_bl_update_status(struct backlight_device *bl)
    {
    struct arcxcnn *lp = bl_get_data(bl);
    let mut brightness: u32 = backlight_get_brightness(bl);
    int ret;
    ret = arcxcnn_set_brightness(lp, brightness);
    if (ret)
    return ret;
// set power-on/off/save modes
    return arcxcnn_update_field(lp, ARCXCNN_CMD, ARCXCNN_CMD_STDBY,
    (bl.props.power == 0) ? 0 : ARCXCNN_CMD_STDBY);
    }
    static const struct backlight_ops arcxcnn_bl_ops = {
    .options = BL_CORE_SUSPENDRESUME,
    .update_status = arcxcnn_bl_update_status,
    };
#[no_mangle]
unsafe extern "C" fn arcxcnn_backlight_register(lp: *mut arcxcnn) -> c_int {
    static int arcxcnn_backlight_register(struct arcxcnn *lp)
    {
    struct backlight_properties *props;
    const char *name = lp.pdata.name ? : "arctic_bl";
    props = devm_kzalloc(lp.dev, sizeof(*props), GFP_KERNEL);
    if (!props)
    return -ENOMEM;
    props.type = BACKLIGHT_PLATFORM;
    props.max_brightness = MAX_BRIGHTNESS;
    if (lp.pdata.initial_brightness > props.max_brightness)
    lp.pdata.initial_brightness = props.max_brightness;
    props.brightness = lp.pdata.initial_brightness;
    lp.bl = devm_backlight_device_register(lp.dev, name, lp.dev, lp,
    &arcxcnn_bl_ops, props);
    return PTR_ERR_OR_ZERO(lp.bl);
    }
#[no_mangle]
unsafe extern "C" fn arcxcnn_parse_dt(lp: *mut arcxcnn) {
    static void arcxcnn_parse_dt(struct arcxcnn *lp)
    {
    struct device *dev = lp.dev;
    struct device_node *node = dev.of_node;
    u32 prog_val, num_entry, entry, sources[ARCXCNN_LEDEN_BITS];
    int ret;
// device tree entry isn't required, defaults are OK
    if (!node)
    return;
    ret = of_property_read_string(node, "label", &lp.pdata.name);
    if (ret < 0)
    lp.pdata.name = core::ptr::null_mut();
    ret = of_property_read_u32(node, "default-brightness", &prog_val);
    if (ret == 0)
    lp.pdata.initial_brightness = prog_val;
    ret = of_property_read_u32(node, "arc,led-config-0", &prog_val);
    if (ret == 0)
    lp.pdata.led_config_0 = (u8)prog_val;
    ret = of_property_read_u32(node, "arc,led-config-1", &prog_val);
    if (ret == 0)
    lp.pdata.led_config_1 = (u8)prog_val;
    ret = of_property_read_u32(node, "arc,dim-freq", &prog_val);
    if (ret == 0)
    lp.pdata.dim_freq = (u8)prog_val;
    ret = of_property_read_u32(node, "arc,comp-config", &prog_val);
    if (ret == 0)
    lp.pdata.comp_config = (u8)prog_val;
    ret = of_property_read_u32(node, "arc,filter-config", &prog_val);
    if (ret == 0)
    lp.pdata.filter_config = (u8)prog_val;
    ret = of_property_read_u32(node, "arc,trim-config", &prog_val);
    if (ret == 0)
    lp.pdata.trim_config = (u8)prog_val;
    ret = of_property_count_u32_elems(node, "led-sources");
    if (ret < 0) {
    lp.pdata.leden = ARCXCNN_LEDEN_MASK; /* all on is default */
    } else {
    num_entry = ret;
    if (num_entry > ARCXCNN_LEDEN_BITS)
    num_entry = ARCXCNN_LEDEN_BITS;
    ret = of_property_read_u32_array(node, "led-sources", sources,
    num_entry);
    if (ret < 0) {
    dev_err(dev, "led-sources node is invalid.\n");
    return;
    }
    lp.pdata.leden = 0;
// for each enable in source, set bit in led enable
    for (entry = 0; entry < num_entry; entry++) {
    let mut onbit: u8 = 1 << sources[entry];
    lp.pdata.leden |= onbit;
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn arcxcnn_probe(cl: *mut i2c_client) -> c_int {
    static int arcxcnn_probe(struct i2c_client *cl)
    {
    struct arcxcnn *lp;
    int ret;
    if (!i2c_check_functionality(cl.adapter, I2C_FUNC_SMBUS_BYTE_DATA))
    return -EIO;
    lp = devm_kzalloc(&cl.dev, sizeof(*lp), GFP_KERNEL);
    if (!lp)
    return -ENOMEM;
    lp.client = cl;
    lp.dev = &cl.dev;
    lp.pdata = dev_get_platdata(&cl.dev);
// reset the device
    ret = i2c_smbus_write_byte_data(lp.client,
    ARCXCNN_CMD, ARCXCNN_CMD_RESET);
    if (ret)
    goto probe_err;
    if (!lp.pdata) {
    lp.pdata = devm_kzalloc(lp.dev,
    sizeof(*lp.pdata), GFP_KERNEL);
    if (!lp.pdata)
    return -ENOMEM;
// Setup defaults based on power-on defaults
    lp.pdata.name = core::ptr::null_mut();
    lp.pdata.initial_brightness = INIT_BRIGHT;
    lp.pdata.leden = ARCXCNN_LEDEN_MASK;
    lp.pdata.led_config_0 = i2c_smbus_read_byte_data(
    lp.client, ARCXCNN_FADECTRL);
    lp.pdata.led_config_1 = i2c_smbus_read_byte_data(
    lp.client, ARCXCNN_ILED_CONFIG);
// insure dim mode is not default pwm
    lp.pdata.led_config_1 |= ARCXCNN_ILED_DIM_INT;
    lp.pdata.dim_freq = i2c_smbus_read_byte_data(
    lp.client, ARCXCNN_DIMFREQ);
    lp.pdata.comp_config = i2c_smbus_read_byte_data(
    lp.client, ARCXCNN_COMP_CONFIG);
    lp.pdata.filter_config = i2c_smbus_read_byte_data(
    lp.client, ARCXCNN_FILT_CONFIG);
    lp.pdata.trim_config = i2c_smbus_read_byte_data(
    lp.client, ARCXCNN_IMAXTUNE);
    if (IS_ENABLED(CONFIG_OF))
    arcxcnn_parse_dt(lp);
    }
    i2c_set_clientdata(cl, lp);
// constrain settings to what is possible
    if (lp.pdata.initial_brightness > MAX_BRIGHTNESS)
    lp.pdata.initial_brightness = MAX_BRIGHTNESS;
// set initial brightness
    ret = arcxcnn_set_brightness(lp, lp.pdata.initial_brightness);
    if (ret)
    goto probe_err;
// set other register values directly
    ret = i2c_smbus_write_byte_data(lp.client, ARCXCNN_FADECTRL,
    lp.pdata.led_config_0);
    if (ret)
    goto probe_err;
    ret = i2c_smbus_write_byte_data(lp.client, ARCXCNN_ILED_CONFIG,
    lp.pdata.led_config_1);
    if (ret)
    goto probe_err;
    ret = i2c_smbus_write_byte_data(lp.client, ARCXCNN_DIMFREQ,
    lp.pdata.dim_freq);
    if (ret)
    goto probe_err;
    ret = i2c_smbus_write_byte_data(lp.client, ARCXCNN_COMP_CONFIG,
    lp.pdata.comp_config);
    if (ret)
    goto probe_err;
    ret = i2c_smbus_write_byte_data(lp.client, ARCXCNN_FILT_CONFIG,
    lp.pdata.filter_config);
    if (ret)
    goto probe_err;
    ret = i2c_smbus_write_byte_data(lp.client, ARCXCNN_IMAXTUNE,
    lp.pdata.trim_config);
    if (ret)
    goto probe_err;
// set initial LED Enables
    arcxcnn_update_field(lp, ARCXCNN_LEDEN,
    ARCXCNN_LEDEN_MASK, lp.pdata.leden);
    ret = arcxcnn_backlight_register(lp);
    if (ret)
    goto probe_register_err;
    backlight_update_status(lp.bl);
    return 0;
    probe_register_err:
    dev_err(lp.dev,
    "failed to register backlight.\n");
    probe_err:
    dev_err(lp.dev,
    "failure ret: %d\n", ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn arcxcnn_remove(cl: *mut i2c_client) {
    static void arcxcnn_remove(struct i2c_client *cl)
    {
    struct arcxcnn *lp = i2c_get_clientdata(cl);
// disable all strings (ignore errors)
    i2c_smbus_write_byte_data(lp.client,
    ARCXCNN_LEDEN, 0x00);
// reset the device (ignore errors)
    i2c_smbus_write_byte_data(lp.client,
    ARCXCNN_CMD, ARCXCNN_CMD_RESET);
    lp.bl.props.brightness = 0;
    backlight_update_status(lp.bl);
    }
    static const struct of_device_id arcxcnn_dt_ids[] = {
    { .compatible = "arc,arc2c0608" },
    { }
    };
    MODULE_DEVICE_TABLE(of, arcxcnn_dt_ids);
    static const struct i2c_device_id arcxcnn_ids[] = {
    { .name = "arc2c0608", .driver_data = ARC2C0608 },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, arcxcnn_ids);
    static struct i2c_driver arcxcnn_driver = {
    .driver = {
    .name = "arcxcnn_bl",
    .of_match_table = arcxcnn_dt_ids,
    },
    .probe = arcxcnn_probe,
    .remove = arcxcnn_remove,
    .id_table = arcxcnn_ids,
    };
    module_i2c_driver(arcxcnn_driver);
    MODULE_LICENSE("GPL v2");
    MODULE_AUTHOR("Brian Dodge <bdodge@arcticsand.com>");
    MODULE_DESCRIPTION("ARCXCNN Backlight driver");
