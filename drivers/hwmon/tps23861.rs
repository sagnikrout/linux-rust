//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/tps23861.c
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
// Copyright (c) 2020 Sartura Ltd.
//
// Driver for the TI TPS23861 PoE PSE.
//
// Author: Robert Marko <robert.marko@sartura.hr>
//

pub const TEMPERATURE: c_uint = 0x2c;
pub const INPUT_VOLTAGE_LSB: c_uint = 0x2e;
pub const INPUT_VOLTAGE_MSB: c_uint = 0x2f;
pub const PORT_1_CURRENT_LSB: c_uint = 0x30;
pub const PORT_1_CURRENT_MSB: c_uint = 0x31;
pub const PORT_1_VOLTAGE_LSB: c_uint = 0x32;
pub const PORT_1_VOLTAGE_MSB: c_uint = 0x33;
pub const PORT_2_CURRENT_LSB: c_uint = 0x34;
pub const PORT_2_CURRENT_MSB: c_uint = 0x35;
pub const PORT_2_VOLTAGE_LSB: c_uint = 0x36;
pub const PORT_2_VOLTAGE_MSB: c_uint = 0x37;
pub const PORT_3_CURRENT_LSB: c_uint = 0x38;
pub const PORT_3_CURRENT_MSB: c_uint = 0x39;
pub const PORT_3_VOLTAGE_LSB: c_uint = 0x3a;
pub const PORT_3_VOLTAGE_MSB: c_uint = 0x3b;
pub const PORT_4_CURRENT_LSB: c_uint = 0x3c;
pub const PORT_4_CURRENT_MSB: c_uint = 0x3d;
pub const PORT_4_VOLTAGE_LSB: c_uint = 0x3e;
pub const PORT_4_VOLTAGE_MSB: c_uint = 0x3f;
pub const PORT_N_CURRENT_LSB_OFFSET: c_uint = 0x04;
pub const PORT_N_VOLTAGE_LSB_OFFSET: c_uint = 0x04;

pub const PORT_1_RESISTANCE_LSB: c_uint = 0x60;
pub const PORT_1_RESISTANCE_MSB: c_uint = 0x61;
pub const PORT_2_RESISTANCE_LSB: c_uint = 0x62;
pub const PORT_2_RESISTANCE_MSB: c_uint = 0x63;
pub const PORT_3_RESISTANCE_LSB: c_uint = 0x64;
pub const PORT_3_RESISTANCE_MSB: c_uint = 0x65;
pub const PORT_4_RESISTANCE_LSB: c_uint = 0x66;
pub const PORT_4_RESISTANCE_MSB: c_uint = 0x67;
pub const PORT_N_RESISTANCE_LSB_OFFSET: c_uint = 0x02;

pub const PORT_RESISTANCE_RSN_OTHER: c_int = 0;
pub const PORT_RESISTANCE_RSN_LOW: c_int = 1;
pub const PORT_RESISTANCE_RSN_OPEN: c_int = 2;
pub const PORT_RESISTANCE_RSN_SHORT: c_int = 3;
pub const PORT_1_STATUS: c_uint = 0x0c;
pub const PORT_2_STATUS: c_uint = 0x0d;
pub const PORT_3_STATUS: c_uint = 0x0e;
pub const PORT_4_STATUS: c_uint = 0x0f;

pub const PORT_CLASS_UNKNOWN: c_int = 0;
pub const PORT_CLASS_1: c_int = 1;
pub const PORT_CLASS_2: c_int = 2;
pub const PORT_CLASS_3: c_int = 3;
pub const PORT_CLASS_4: c_int = 4;
pub const PORT_CLASS_RESERVED: c_int = 5;
pub const PORT_CLASS_0: c_int = 6;
pub const PORT_CLASS_OVERCURRENT: c_int = 7;
pub const PORT_CLASS_MISMATCH: c_int = 8;
pub const PORT_DETECT_UNKNOWN: c_int = 0;
pub const PORT_DETECT_SHORT: c_int = 1;
pub const PORT_DETECT_RESERVED: c_int = 2;
pub const PORT_DETECT_RESISTANCE_LOW: c_int = 3;
pub const PORT_DETECT_RESISTANCE_OK: c_int = 4;
pub const PORT_DETECT_RESISTANCE_HIGH: c_int = 5;
pub const PORT_DETECT_OPEN_CIRCUIT: c_int = 6;
pub const PORT_DETECT_RESERVED_2: c_int = 7;
pub const PORT_DETECT_MOSFET_FAULT: c_int = 8;
pub const PORT_DETECT_LEGACY: c_int = 9;
// Measurment beyond clamp voltage
pub const PORT_DETECT_CAPACITANCE_INVALID_BEYOND: c_int = 10;
// Insufficient voltage delta
pub const PORT_DETECT_CAPACITANCE_INVALID_DELTA: c_int = 11;
pub const PORT_DETECT_CAPACITANCE_OUT_OF_RANGE: c_int = 12;
pub const POE_PLUS: c_uint = 0x40;
pub const OPERATING_MODE: c_uint = 0x12;
pub const OPERATING_MODE_OFF: c_int = 0;
pub const OPERATING_MODE_MANUAL: c_int = 1;
pub const OPERATING_MODE_SEMI: c_int = 2;
pub const OPERATING_MODE_AUTO: c_int = 3;

pub const DETECT_CLASS_RESTART: c_uint = 0x18;
pub const POWER_ENABLE: c_uint = 0x19;
pub const TPS23861_NUM_PORTS: c_int = 4;
pub const TPS23861_GENERAL_MASK_1: c_uint = 0x17;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tps23861_data {
    pub regmap: *mut regmap,
    pub shunt_resistor: u32,
    pub client: *mut i2c_client,
}

    static const struct regmap_config tps23861_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = 0x6f,
    };
#[no_mangle]
unsafe extern "C" fn tps23861_read_temp(data: *mut tps23861_data, val: *mut c_long) -> c_int {
    static int tps23861_read_temp(struct tps23861_data *data, long *val)
    {
    unsigned int regval;
    int err;
    err = regmap_read(data.regmap, TEMPERATURE, &regval);
    if (err < 0)
    return err;
// val = ((long)regval * TEMPERATURE_LSB) - 20000;
    return 0;
    }
    static int tps23861_read_voltage(struct tps23861_data *data, int channel,
    long *val)
    {
    __le16 regval;
    long raw_val;
    int err;
    if (channel < TPS23861_NUM_PORTS) {
    err = regmap_bulk_read(data.regmap,
    PORT_1_VOLTAGE_LSB + channel * PORT_N_VOLTAGE_LSB_OFFSET,
    &regval, 2);
    } else {
    err = regmap_bulk_read(data.regmap,
    INPUT_VOLTAGE_LSB,
    &regval, 2);
    }
    if (err < 0)
    return err;
    raw_val = le16_to_cpu(regval);
// val = (FIELD_GET(VOLTAGE_CURRENT_MASK, raw_val) * VOLTAGE_LSB) / 1000;
    return 0;
    }
    static int tps23861_read_current(struct tps23861_data *data, int channel,
    long *val)
    {
    long raw_val, current_lsb;
    __le16 regval;
    int err;
    if (data.shunt_resistor == SHUNT_RESISTOR_DEFAULT)
    current_lsb = CURRENT_LSB_255;
    else
    current_lsb = CURRENT_LSB_250;
    err = regmap_bulk_read(data.regmap,
    PORT_1_CURRENT_LSB + channel * PORT_N_CURRENT_LSB_OFFSET,
    &regval, 2);
    if (err < 0)
    return err;
    raw_val = le16_to_cpu(regval);
// val = (FIELD_GET(VOLTAGE_CURRENT_MASK, raw_val) * current_lsb) / 1000000;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tps23861_port_disable(data: *mut tps23861_data, channel: c_int) -> c_int {
    static int tps23861_port_disable(struct tps23861_data *data, int channel)
    {
    let mut regval: c_uint = 0;
    int err;
    regval |= BIT(channel + 4);
    err = regmap_write(data.regmap, POWER_ENABLE, regval);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn tps23861_port_enable(data: *mut tps23861_data, channel: c_int) -> c_int {
    static int tps23861_port_enable(struct tps23861_data *data, int channel)
    {
    let mut regval: c_uint = 0;
    int err;
    regval |= BIT(channel);
    regval |= BIT(channel + 4);
    err = regmap_write(data.regmap, DETECT_CLASS_RESTART, regval);
    return err;
    }
    static umode_t tps23861_is_visible(const void *data, enum hwmon_sensor_types type,
    u32 attr, int channel)
    {
    switch (type) {
    case hwmon_temp:
    switch (attr) {
    case hwmon_temp_input:
    case hwmon_temp_label:
    return 0444;
    default:
    return 0;
    }
    case hwmon_in:
    switch (attr) {
    case hwmon_in_input:
    case hwmon_in_label:
    return 0444;
    case hwmon_in_enable:
    return 0200;
    default:
    return 0;
    }
    case hwmon_curr:
    switch (attr) {
    case hwmon_curr_input:
    case hwmon_curr_label:
    return 0444;
    default:
    return 0;
    }
    default:
    return 0;
    }
    }
    static int tps23861_write(struct device *dev, enum hwmon_sensor_types type,
    u32 attr, int channel, long val)
    {
    struct tps23861_data *data = dev_get_drvdata(dev);
    int err;
    switch (type) {
    case hwmon_in:
    switch (attr) {
    case hwmon_in_enable:
    if (val == 0)
    err = tps23861_port_disable(data, channel);
#[no_mangle]
pub unsafe extern "C" fn if(1: val ==) -> else {
    else if (val == 1)
    err = tps23861_port_enable(data, channel);
    else
    err = -EINVAL;
    break;
    default:
    return -EOPNOTSUPP;
    }
    break;
    default:
    return -EOPNOTSUPP;
    }
    return err;
    }
    static int tps23861_read(struct device *dev, enum hwmon_sensor_types type,
    u32 attr, int channel, long *val)
    {
    struct tps23861_data *data = dev_get_drvdata(dev);
    int err;
    switch (type) {
    case hwmon_temp:
    switch (attr) {
    case hwmon_temp_input:
    err = tps23861_read_temp(data, val);
    break;
    default:
    return -EOPNOTSUPP;
    }
    break;
    case hwmon_in:
    switch (attr) {
    case hwmon_in_input:
    err = tps23861_read_voltage(data, channel, val);
    break;
    default:
    return -EOPNOTSUPP;
    }
    break;
    case hwmon_curr:
    switch (attr) {
    case hwmon_curr_input:
    err = tps23861_read_current(data, channel, val);
    break;
    default:
    return -EOPNOTSUPP;
    }
    break;
    default:
    return -EOPNOTSUPP;
    }
    return err;
    }
    static const char * const tps23861_port_label[] = {
    "Port1",
    "Port2",
    "Port3",
    "Port4",
    "Input",
    };
    static int tps23861_read_string(struct device *dev,
    enum hwmon_sensor_types type,
    u32 attr, int channel, const char **str)
    {
    switch (type) {
    case hwmon_in:
    case hwmon_curr:
// str = tps23861_port_label[channel];
    break;
    case hwmon_temp:
// str = "Die";
    break;
    default:
    return -EOPNOTSUPP;
    }
    return 0;
    }
    static const struct hwmon_channel_info * const tps23861_info[] = {
    HWMON_CHANNEL_INFO(chip,
    HWMON_C_REGISTER_TZ),
    HWMON_CHANNEL_INFO(temp,
    HWMON_T_INPUT | HWMON_T_LABEL),
    HWMON_CHANNEL_INFO(in,
    HWMON_I_INPUT | HWMON_I_ENABLE | HWMON_I_LABEL,
    HWMON_I_INPUT | HWMON_I_ENABLE | HWMON_I_LABEL,
    HWMON_I_INPUT | HWMON_I_ENABLE | HWMON_I_LABEL,
    HWMON_I_INPUT | HWMON_I_ENABLE | HWMON_I_LABEL,
    HWMON_I_INPUT | HWMON_I_LABEL),
    HWMON_CHANNEL_INFO(curr,
    HWMON_C_INPUT | HWMON_C_LABEL,
    HWMON_C_INPUT | HWMON_C_LABEL,
    HWMON_C_INPUT | HWMON_C_LABEL,
    HWMON_C_INPUT | HWMON_C_LABEL),
    core::ptr::null_mut()
    };
    static const struct hwmon_ops tps23861_hwmon_ops = {
    .is_visible = tps23861_is_visible,
    .write = tps23861_write,
    .read = tps23861_read,
    .read_string = tps23861_read_string,
    };
    static const struct hwmon_chip_info tps23861_chip_info = {
    .ops = &tps23861_hwmon_ops,
    .info = tps23861_info,
    };
    static char *port_operating_mode_string(uint8_t mode_reg, unsigned int port)
    {
    let mut mode: c_uint = ~0;
    if (port < TPS23861_NUM_PORTS)
    mode = (mode_reg >> (2 * port)) & OPERATING_MODE_PORT_1_MASK;
    switch (mode) {
    case OPERATING_MODE_OFF:
    return "Off";
    case OPERATING_MODE_MANUAL:
    return "Manual";
    case OPERATING_MODE_SEMI:
    return "Semi-Auto";
    case OPERATING_MODE_AUTO:
    return "Auto";
    default:
    return "Invalid";
    }
    }
    static char *port_detect_status_string(uint8_t status_reg)
    {
    switch (FIELD_GET(PORT_STATUS_DETECT_MASK, status_reg)) {
    case PORT_DETECT_UNKNOWN:
    return "Unknown device";
    case PORT_DETECT_SHORT:
    return "Short circuit";
    case PORT_DETECT_RESISTANCE_LOW:
    return "Too low resistance";
    case PORT_DETECT_RESISTANCE_OK:
    return "Valid resistance";
    case PORT_DETECT_RESISTANCE_HIGH:
    return "Too high resistance";
    case PORT_DETECT_OPEN_CIRCUIT:
    return "Open circuit";
    case PORT_DETECT_MOSFET_FAULT:
    return "MOSFET fault";
    case PORT_DETECT_LEGACY:
    return "Legacy device";
    case PORT_DETECT_CAPACITANCE_INVALID_BEYOND:
    return "Invalid capacitance, beyond clamp voltage";
    case PORT_DETECT_CAPACITANCE_INVALID_DELTA:
    return "Invalid capacitance, insufficient voltage delta";
    case PORT_DETECT_CAPACITANCE_OUT_OF_RANGE:
    return "Valid capacitance, outside of legacy range";
    case PORT_DETECT_RESERVED:
    case PORT_DETECT_RESERVED_2:
    default:
    return "Invalid";
    }
    }
    static char *port_class_status_string(uint8_t status_reg)
    {
    switch (FIELD_GET(PORT_STATUS_CLASS_MASK, status_reg)) {
    case PORT_CLASS_UNKNOWN:
    return "Unknown";
    case PORT_CLASS_RESERVED:
    case PORT_CLASS_0:
    return "0";
    case PORT_CLASS_1:
    return "1";
    case PORT_CLASS_2:
    return "2";
    case PORT_CLASS_3:
    return "3";
    case PORT_CLASS_4:
    return "4";
    case PORT_CLASS_OVERCURRENT:
    return "Overcurrent";
    case PORT_CLASS_MISMATCH:
    return "Mismatch";
    default:
    return "Invalid";
    }
    }
    static char *port_poe_plus_status_string(uint8_t poe_plus, unsigned int port)
    {
    return (BIT(port + 4) & poe_plus) ? "Yes" : "No";
    }
#[no_mangle]
unsafe extern "C" fn tps23861_port_resistance(data: *mut tps23861_data, port: c_int) -> c_int {
    static int tps23861_port_resistance(struct tps23861_data *data, int port)
    {
    unsigned int raw_val;
    __le16 regval;
    regmap_bulk_read(data.regmap,
    PORT_1_RESISTANCE_LSB + PORT_N_RESISTANCE_LSB_OFFSET * port,
    &regval,
    2);
    raw_val = le16_to_cpu(regval);
    switch (FIELD_GET(PORT_RESISTANCE_RSN_MASK, raw_val)) {
    case PORT_RESISTANCE_RSN_OTHER:
    return (FIELD_GET(PORT_RESISTANCE_MASK, raw_val) * RESISTANCE_LSB) / 10000;
    case PORT_RESISTANCE_RSN_LOW:
    return (FIELD_GET(PORT_RESISTANCE_MASK, raw_val) * RESISTANCE_LSB_LOW) / 10000;
    case PORT_RESISTANCE_RSN_SHORT:
    case PORT_RESISTANCE_RSN_OPEN:
    default:
    return 0;
    }
    }
#[no_mangle]
unsafe extern "C" fn tps23861_port_status_show(s: *mut seq_file, data: *mut c_void) -> c_int {
    static int tps23861_port_status_show(struct seq_file *s, void *data)
    {
    struct tps23861_data *priv = s.private;
    unsigned int i, mode, poe_plus, status;
    regmap_read(priv.regmap, OPERATING_MODE, &mode);
    regmap_read(priv.regmap, POE_PLUS, &poe_plus);
    for (i = 0; i < TPS23861_NUM_PORTS; i++) {
    regmap_read(priv.regmap, PORT_1_STATUS + i, &status);
    seq_printf(s, "Port: \t\t%d\n", i + 1);
    seq_printf(s, "Operating mode: %s\n", port_operating_mode_string(mode, i));
    seq_printf(s, "Detected: \t%s\n", port_detect_status_string(status));
    seq_printf(s, "Class: \t\t%s\n", port_class_status_string(status));
    seq_printf(s, "PoE Plus: \t%s\n", port_poe_plus_status_string(poe_plus, i));
    seq_printf(s, "Resistance: \t%d\n", tps23861_port_resistance(priv, i));
    seq_putc(s, '\n');
    }
    return 0;
    }
    DEFINE_SHOW_ATTRIBUTE(tps23861_port_status);
#[no_mangle]
unsafe extern "C" fn tps23861_probe(client: *mut i2c_client) -> c_int {
    static int tps23861_probe(struct i2c_client *client)
    {
    struct device *dev = &client.dev;
    struct tps23861_data *data;
    struct device *hwmon_dev;
    u32 shunt_resistor;
    data = devm_kzalloc(dev, sizeof(*data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    data.client = client;
    i2c_set_clientdata(client, data);
    data.regmap = devm_regmap_init_i2c(client, &tps23861_regmap_config);
    if (IS_ERR(data.regmap)) {
    dev_err(dev, "failed to allocate register map\n");
    return PTR_ERR(data.regmap);
    }
    if (!of_property_read_u32(dev.of_node, "shunt-resistor-micro-ohms", &shunt_resistor))
    data.shunt_resistor = shunt_resistor;
    else
    data.shunt_resistor = SHUNT_RESISTOR_DEFAULT;
    if (data.shunt_resistor == SHUNT_RESISTOR_DEFAULT)
    regmap_clear_bits(data.regmap,
    TPS23861_GENERAL_MASK_1,
    TPS23861_CURRENT_SHUNT_MASK);
    else
    regmap_set_bits(data.regmap,
    TPS23861_GENERAL_MASK_1,
    TPS23861_CURRENT_SHUNT_MASK);
    hwmon_dev = devm_hwmon_device_register_with_info(dev, client.name,
    data, &tps23861_chip_info,
    core::ptr::null_mut());
    if (IS_ERR(hwmon_dev))
    return PTR_ERR(hwmon_dev);
    debugfs_create_file("port_status", 0400, client.debugfs, data,
    &tps23861_port_status_fops);
    return 0;
    }
    static const struct of_device_id __maybe_unused tps23861_of_match[] = {
    { .compatible = "ti,tps23861", },
    { },
    };
    MODULE_DEVICE_TABLE(of, tps23861_of_match);
    static struct i2c_driver tps23861_driver = {
    .probe			= tps23861_probe,
    .driver = {
    .name		= "tps23861",
    .of_match_table	= of_match_ptr(tps23861_of_match),
    },
    };
    module_i2c_driver(tps23861_driver);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Robert Marko <robert.marko@sartura.hr>");
    MODULE_DESCRIPTION("TI TPS23861 PoE PSE");
