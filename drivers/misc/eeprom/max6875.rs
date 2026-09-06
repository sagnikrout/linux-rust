//! Automatically rewritten from C to Rust
//! Source: drivers/misc/eeprom/max6875.c
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
// max6875.c - driver for MAX6874/MAX6875
//
// Copyright (C) 2005 Ben Gardner <bgardner@wabtec.com>
//
// Based on eeprom.c
//
// The MAX6875 has a bank of registers and two banks of EEPROM.
// Address ranges are defined as follows:
// * 0x0000 - 0x0046 = configuration registers
// * 0x8000 - 0x8046 = configuration EEPROM
// * 0x8100 - 0x82FF = user EEPROM
//
// This driver makes the user EEPROM available for read.
//
// The registers & config EEPROM should be accessed via i2c-dev.
//
// The MAX6875 ignores the lowest address bit, so each chip responds to
// two addresses - 0x50/0x51 and 0x52/0x53.
//
// Note that the MAX6875 uses i2c_smbus_write_byte_data() to set the read
// address, so this driver is destructive if loaded for the wrong EEPROM chip.
//

// The MAX6875 can only read/write 16 bytes at a time
pub const SLICE_SIZE: c_int = 16;
pub const SLICE_BITS: c_int = 4;
// USER EEPROM is at addresses 0x8100 - 0x82FF
pub const USER_EEPROM_BASE: c_uint = 0x8100;
pub const USER_EEPROM_SIZE: c_uint = 0x0200;
pub const USER_EEPROM_SLICES: c_int = 32;
// MAX6875 commands
pub const MAX6875_CMD_BLK_READ: c_uint = 0x84;
// Each client has this additional data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct max6875_data {
    pub fake_client: *mut i2c_client,
    pub update_lock: mutex,
    pub valid: u32,
    pub data: [u8; USER_EEPROM_SIZE],
    pub last_updated: [c_ulong; USER_EEPROM_SLICES],
}

#[no_mangle]
unsafe extern "C" fn max6875_update_slice(client: *mut i2c_client, slice: c_int) {
    static void max6875_update_slice(struct i2c_client *client, int slice)
    {
    struct max6875_data *data = i2c_get_clientdata(client);
    int i, j, addr;
    u8 *buf;
    if (slice >= USER_EEPROM_SLICES)
    return;
    mutex_lock(&data.update_lock);
    buf = &data.data[slice << SLICE_BITS];
    if (!(data.valid & (1 << slice)) ||
    time_after(jiffies, data.last_updated[slice])) {
    dev_dbg(&client.dev, "Starting update of slice %u\n", slice);
    data.valid &= ~(1 << slice);
    addr = USER_EEPROM_BASE + (slice << SLICE_BITS);
// select the eeprom address
    if (i2c_smbus_write_byte_data(client, addr >> 8, addr & 0xFF)) {
    dev_err(&client.dev, "address set failed\n");
    goto exit_up;
    }
    if (i2c_check_functionality(client.adapter,
    I2C_FUNC_SMBUS_READ_I2C_BLOCK)) {
    if (i2c_smbus_read_i2c_block_data(client,
    MAX6875_CMD_BLK_READ,
    SLICE_SIZE,
    buf) != SLICE_SIZE) {
    goto exit_up;
    }
    } else {
    for (i = 0; i < SLICE_SIZE; i++) {
    j = i2c_smbus_read_byte(client);
    if (j < 0) {
    goto exit_up;
    }
    buf[i] = j;
    }
    }
    data.last_updated[slice] = jiffies;
    data.valid |= (1 << slice);
    }
    exit_up:
    mutex_unlock(&data.update_lock);
    }
    static ssize_t max6875_read(struct file *filp, struct kobject *kobj,
    const struct bin_attribute *bin_attr,
    char *buf, loff_t off, size_t count)
    {
    struct i2c_client *client = kobj_to_i2c_client(kobj);
    struct max6875_data *data = i2c_get_clientdata(client);
    int slice, max_slice;
// refresh slices which contain requested bytes
    max_slice = (off + count - 1) >> SLICE_BITS;
    for (slice = (off >> SLICE_BITS); slice <= max_slice; slice++)
    max6875_update_slice(client, slice);
    memcpy(buf, &data.data[off], count);
    return count;
    }
    static const struct bin_attribute user_eeprom_attr = {
    .attr = {
    .name = "eeprom",
    .mode = S_IRUGO,
    },
    .size = USER_EEPROM_SIZE,
    .read = max6875_read,
    };
#[no_mangle]
unsafe extern "C" fn max6875_probe(client: *mut i2c_client) -> c_int {
    static int max6875_probe(struct i2c_client *client)
    {
    struct i2c_adapter *adapter = client.adapter;
    struct max6875_data *data;
    int err;
    if (!i2c_check_functionality(adapter, I2C_FUNC_SMBUS_WRITE_BYTE_DATA
    | I2C_FUNC_SMBUS_READ_BYTE))
    return -ENODEV;
// Only bind to even addresses
    if (client.addr & 1)
    return -ENODEV;
    data = kzalloc_obj(struct max6875_data);
    if (!data)
    return -ENOMEM;
// A fake client is created on the odd address
    data.fake_client = i2c_new_dummy_device(client.adapter, client.addr + 1);
    if (IS_ERR(data.fake_client)) {
    err = PTR_ERR(data.fake_client);
    goto exit_kfree;
    }
// Init real i2c_client
    i2c_set_clientdata(client, data);
    mutex_init(&data.update_lock);
    err = sysfs_create_bin_file(&client.dev.kobj, &user_eeprom_attr);
    if (err)
    goto exit_remove_fake;
    return 0;
    exit_remove_fake:
    i2c_unregister_device(data.fake_client);
    exit_kfree:
    kfree(data);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn max6875_remove(client: *mut i2c_client) {
    static void max6875_remove(struct i2c_client *client)
    {
    struct max6875_data *data = i2c_get_clientdata(client);
    i2c_unregister_device(data.fake_client);
    sysfs_remove_bin_file(&client.dev.kobj, &user_eeprom_attr);
    kfree(data);
    }
    static const struct i2c_device_id max6875_id[] = {
    { .name = "max6875" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, max6875_id);
    static struct i2c_driver max6875_driver = {
    .driver = {
    .name	= "max6875",
    },
    .probe		= max6875_probe,
    .remove		= max6875_remove,
    .id_table	= max6875_id,
    };
    module_i2c_driver(max6875_driver);
    MODULE_AUTHOR("Ben Gardner <bgardner@wabtec.com>");
    MODULE_DESCRIPTION("MAX6875 driver");
    MODULE_LICENSE("GPL");
