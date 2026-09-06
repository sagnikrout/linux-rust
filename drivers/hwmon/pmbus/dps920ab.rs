//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/pmbus/dps920ab.c
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
// Driver for Delta DPS920AB PSU
//
// Copyright (C) 2021 Delta Networks, Inc.
// Copyright (C) 2021 Sartura Ltd.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dps920ab_data {
    pub mfr_model: *mut c_char,
    pub mfr_id: *mut c_char,
}

#[no_mangle]
unsafe extern "C" fn dps920ab_read_word_data(client: *mut i2c_client, page: c_int, phase: c_int, reg: c_int) -> c_int {
    static int dps920ab_read_word_data(struct i2c_client *client, int page, int phase, int reg)
    {
//
// This masks commands which are not supported.
// PSU advertises that all features are supported,
// in reality that unfortunately is not true.
// So enable only those that the datasheet confirms.
//
    switch (reg) {
    case PMBUS_FAN_COMMAND_1:
    case PMBUS_IOUT_OC_WARN_LIMIT:
    case PMBUS_STATUS_WORD:
    case PMBUS_READ_VIN:
    case PMBUS_READ_IIN:
    case PMBUS_READ_VOUT:
    case PMBUS_READ_IOUT:
    case PMBUS_READ_TEMPERATURE_1:
    case PMBUS_READ_TEMPERATURE_2:
    case PMBUS_READ_TEMPERATURE_3:
    case PMBUS_READ_FAN_SPEED_1:
    case PMBUS_READ_POUT:
    case PMBUS_READ_PIN:
    case PMBUS_MFR_VOUT_MIN:
    case PMBUS_MFR_VOUT_MAX:
    case PMBUS_MFR_IOUT_MAX:
    case PMBUS_MFR_POUT_MAX:
    return pmbus_read_word_data(client, page, phase, reg);
    default:
    return -ENXIO;
    }
    }
    static int dps920ab_write_word_data(struct i2c_client *client, int page, int reg,
    u16 word)
    {
//
// This masks commands which are not supported.
// PSU only has one R/W register and that is
// for the fan.
//
    switch (reg) {
    case PMBUS_FAN_COMMAND_1:
    return pmbus_write_word_data(client, page, reg, word);
    default:
    return -EACCES;
    }
    }
    static struct pmbus_driver_info dps920ab_info = {
    .pages = 1,
    .format[PSC_VOLTAGE_IN] = linear,
    .format[PSC_VOLTAGE_OUT] = linear,
    .format[PSC_CURRENT_IN] = linear,
    .format[PSC_CURRENT_OUT] = linear,
    .format[PSC_POWER] = linear,
    .format[PSC_FAN] = linear,
    .format[PSC_TEMPERATURE] = linear,
    .func[0] =
    PMBUS_HAVE_VIN | PMBUS_HAVE_IIN | PMBUS_HAVE_PIN |
    PMBUS_HAVE_VOUT | PMBUS_HAVE_IOUT | PMBUS_HAVE_POUT |
    PMBUS_HAVE_TEMP  | PMBUS_HAVE_TEMP2 | PMBUS_HAVE_TEMP3 |
    PMBUS_HAVE_FAN12 | PMBUS_HAVE_STATUS_FAN12 |
    PMBUS_HAVE_STATUS_VOUT | PMBUS_HAVE_STATUS_IOUT |
    PMBUS_HAVE_STATUS_INPUT | PMBUS_HAVE_STATUS_TEMP,
    .read_word_data = dps920ab_read_word_data,
    .write_word_data = dps920ab_write_word_data,
    };
#[no_mangle]
unsafe extern "C" fn dps920ab_mfr_id_show(s: *mut seq_file, data: *mut c_void) -> c_int {
    static int dps920ab_mfr_id_show(struct seq_file *s, void *data)
    {
    struct dps920ab_data *priv = s.private;
    seq_printf(s, "%s\n", priv.mfr_id);
    return 0;
    }
    DEFINE_SHOW_ATTRIBUTE(dps920ab_mfr_id);
#[no_mangle]
unsafe extern "C" fn dps920ab_mfr_model_show(s: *mut seq_file, data: *mut c_void) -> c_int {
    static int dps920ab_mfr_model_show(struct seq_file *s, void *data)
    {
    struct dps920ab_data *priv = s.private;
    seq_printf(s, "%s\n", priv.mfr_model);
    return 0;
    }
    DEFINE_SHOW_ATTRIBUTE(dps920ab_mfr_model);
#[no_mangle]
unsafe extern "C" fn dps920ab_init_debugfs(data: *mut dps920ab_data, client: *mut i2c_client) {
    static void dps920ab_init_debugfs(struct dps920ab_data *data, struct i2c_client *client)
    {
    struct dentry *debugfs_dir;
    struct dentry *root;
    root = pmbus_get_debugfs_dir(client);
    if (!root)
    return;
    debugfs_dir = debugfs_create_dir(client.name, root);
    debugfs_create_file("mfr_id",
    0400,
    debugfs_dir,
    data,
    &dps920ab_mfr_id_fops);
    debugfs_create_file("mfr_model",
    0400,
    debugfs_dir,
    data,
    &dps920ab_mfr_model_fops);
    }
#[no_mangle]
unsafe extern "C" fn dps920ab_probe(client: *mut i2c_client) -> c_int {
    static int dps920ab_probe(struct i2c_client *client)
    {
    u8 buf[I2C_SMBUS_BLOCK_MAX + 1];
    struct dps920ab_data *data;
    int ret;
    data = devm_kzalloc(&client.dev, sizeof(*data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    ret = i2c_smbus_read_block_data(client, PMBUS_MFR_ID, buf);
    if (ret < 0) {
    dev_err(&client.dev, "Failed to read Manufacturer ID\n");
    return ret;
    }
    buf[ret] = '\0';
    if (ret != 5 || strncmp(buf, "DELTA", 5)) {
    buf[ret] = '\0';
    dev_err(&client.dev, "Unsupported Manufacturer ID '%s'\n", buf);
    return -ENODEV;
    }
    data.mfr_id = devm_kstrdup(&client.dev, buf, GFP_KERNEL);
    if (!data.mfr_id)
    return -ENOMEM;
    ret = i2c_smbus_read_block_data(client, PMBUS_MFR_MODEL, buf);
    if (ret < 0) {
    dev_err(&client.dev, "Failed to read Manufacturer Model\n");
    return ret;
    }
    buf[ret] = '\0';
    if (ret != 11 || strncmp(buf, "DPS-920AB", 9)) {
    dev_err(&client.dev, "Unsupported Manufacturer Model '%s'\n", buf);
    return -ENODEV;
    }
    data.mfr_model = devm_kstrdup(&client.dev, buf, GFP_KERNEL);
    if (!data.mfr_model)
    return -ENOMEM;
    ret = pmbus_do_probe(client, &dps920ab_info);
    if (ret)
    return ret;
    dps920ab_init_debugfs(data, client);
    return 0;
    }
    static const struct of_device_id __maybe_unused dps920ab_of_match[] = {
    { .compatible = "delta,dps920ab", },
    {}
    };
    MODULE_DEVICE_TABLE(of, dps920ab_of_match);
    static const struct i2c_device_id dps920ab_device_id[] = {
    { .name = "dps920ab" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, dps920ab_device_id);
    static struct i2c_driver dps920ab_driver = {
    .driver = {
    .name = "dps920ab",
    .of_match_table = of_match_ptr(dps920ab_of_match),
    },
    .probe = dps920ab_probe,
    .id_table = dps920ab_device_id,
    };
    module_i2c_driver(dps920ab_driver);
    MODULE_AUTHOR("Robert Marko <robert.marko@sartura.hr>");
    MODULE_DESCRIPTION("PMBus driver for Delta DPS920AB PSU");
    MODULE_LICENSE("GPL");
    MODULE_IMPORT_NS("PMBUS");
