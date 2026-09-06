//! Automatically rewritten from C to Rust
//! Source: drivers/nvmem/ee1004.c
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
// ee1004 - driver for DDR4 SPD EEPROMs
//
// Copyright (C) 2017-2019 Jean Delvare
//
// Based on the at24 driver:
// Copyright (C) 2005-2007 David Brownell
// Copyright (C) 2008 Wolfram Sang, Pengutronix
//

//
// DDR4 memory modules use special EEPROMs following the Jedec EE1004
// specification. These are 512-byte EEPROMs using a single I2C address
// in the 0x50-0x57 range for data. One of two 256-byte page is selected
// by writing a command to I2C address 0x36 or 0x37 on the same I2C bus.
//
// Therefore we need to request these 2 additional addresses, and serialize
// access to all such EEPROMs with a single mutex.
//
// We assume it is safe to read up to 32 bytes at once from these EEPROMs.
// We use SMBus access even if I2C is available, these EEPROMs are small
// enough, and reading from them infrequent enough, that we favor simplicity
// over performance.
//
pub const EE1004_MAX_BUSSES: c_int = 8;
pub const EE1004_ADDR_SET_PAGE: c_uint = 0x36;
pub const EE1004_NUM_PAGES: c_int = 2;
pub const EE1004_PAGE_SIZE: c_int = 256;
pub const EE1004_PAGE_SHIFT: c_int = 8;

//
// Mutex protects ee1004_set_page and ee1004_dev_count, and must be held
// from page selection to end of read.
//
    static DEFINE_MUTEX(ee1004_bus_lock);
    static struct ee1004_bus_data {
    struct i2c_adapter *adap;
    struct i2c_client *set_page[EE1004_NUM_PAGES];
    unsigned int dev_count;
    int current_page;
    } ee1004_bus_data[EE1004_MAX_BUSSES];
    static const struct i2c_device_id ee1004_ids[] = {
    { "ee1004" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, ee1004_ids);
// -------------------------------------------------------------------------
    static struct ee1004_bus_data *ee1004_get_bus_data(struct i2c_adapter *adap)
    {
    int i;
    for (i = 0; i < EE1004_MAX_BUSSES; i++)
    if (ee1004_bus_data[i].adap == adap)
    return ee1004_bus_data + i;
// If not existent yet, create new entry
    for (i = 0; i < EE1004_MAX_BUSSES; i++)
    if (!ee1004_bus_data[i].adap) {
    ee1004_bus_data[i].adap = adap;
    return ee1004_bus_data + i;
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn ee1004_get_current_page(bd: *mut ee1004_bus_data) -> c_int {
    static int ee1004_get_current_page(struct ee1004_bus_data *bd)
    {
    int err;
    err = i2c_smbus_read_byte(bd.set_page[0]);
    if (err == -ENXIO) {
// Nack means page 1 is selected
    return 1;
    }
    if (err < 0) {
// Anything else is a real error, bail out
    return err;
    }
// Ack means page 0 is selected, returned value meaningless
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ee1004_set_current_page(client: *mut i2c_client, page: c_int) -> c_int {
    static int ee1004_set_current_page(struct i2c_client *client, int page)
    {
    struct ee1004_bus_data *bd = i2c_get_clientdata(client);
    int ret;
    if (page == bd.current_page)
    return 0;
// Data is ignored
    ret = i2c_smbus_write_byte(bd.set_page[page], 0x00);
//
// Don't give up just yet. Some memory modules will select the page
// but not ack the command. Check which page is selected now.
//
    if (ret == -ENXIO && ee1004_get_current_page(bd) == page)
    ret = 0;
    if (ret < 0) {
    dev_err(&client.dev, "Failed to select page %d (%d)\n", page, ret);
    return ret;
    }
    dev_dbg(&client.dev, "Selected page %d\n", page);
    bd.current_page = page;
    return 0;
    }
    static ssize_t ee1004_eeprom_read(struct i2c_client *client, char *buf,
    unsigned int offset, size_t count)
    {
    int status, page;
    page = offset >> EE1004_PAGE_SHIFT;
    offset &= (1 << EE1004_PAGE_SHIFT) - 1;
    status = ee1004_set_current_page(client, page);
    if (status)
    return status;
// Can't cross page boundaries
    if (offset + count > EE1004_PAGE_SIZE)
    count = EE1004_PAGE_SIZE - offset;
    if (count > I2C_SMBUS_BLOCK_MAX)
    count = I2C_SMBUS_BLOCK_MAX;
    return i2c_smbus_read_i2c_block_data_or_emulated(client, offset, count, buf);
    }
#[no_mangle]
unsafe extern "C" fn ee1004_read(priv: *mut c_void, off: c_uint, val: *mut c_void, count: usize) -> c_int {
    static int ee1004_read(void *priv, unsigned int off, void *val, size_t count)
    {
    struct i2c_client *client = priv;
    char *buf = val;
    int ret;
    if (unlikely(!count))
    return count;
    if (off + count > EE1004_EEPROM_SIZE)
    return -EINVAL;
//
// Read data from chip, protecting against concurrent access to
// other EE1004 SPD EEPROMs on the same adapter.
//
    mutex_lock(&ee1004_bus_lock);
    while (count) {
    ret = ee1004_eeprom_read(client, buf, off, count);
    if (ret < 0) {
    mutex_unlock(&ee1004_bus_lock);
    return ret;
    }
    buf += ret;
    off += ret;
    count -= ret;
    }
    mutex_unlock(&ee1004_bus_lock);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ee1004_probe_temp_sensor(client: *mut i2c_client) {
    static void ee1004_probe_temp_sensor(struct i2c_client *client)
    {
    let mut info: i2c_board_info = { .type = "jc42" };
    let mut addr: c_ushort = 0x18 | (client.addr & 7);
    unsigned short addr_list[] = { addr, I2C_CLIENT_END };
    u8 data[2];
    int ret;
// byte 14, bit 7 is set if temp sensor is present
    ret = ee1004_eeprom_read(client, data, 14, 1);
    if (ret != 1)
    return;
    if (!(data[0] & BIT(7))) {
//
// If the SPD data suggests that there is no temperature
// sensor, it may still be there for SPD revision 1.0.
// See SPD Annex L, Revision 1 and 2, for details.
// Check DIMM type and SPD revision; if it is a DDR4
// with SPD revision 1.0, check the thermal sensor address
// and instantiate the jc42 driver if a chip is found at
// that address.
// It is not necessary to check if there is a chip at the
// temperature sensor address since i2c_new_scanned_device()
// will do that and return silently if no chip is found.
//
    ret = ee1004_eeprom_read(client, data, 1, 2);
    if (ret != 2 || data[0] != 0x10 || data[1] != 0x0c)
    return;
    }
    i2c_new_scanned_device(client.adapter, &info, addr_list, core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn ee1004_cleanup(idx: c_int, bd: *mut ee1004_bus_data) {
    static void ee1004_cleanup(int idx, struct ee1004_bus_data *bd)
    {
    if (--bd.dev_count == 0) {
    while (--idx >= 0)
    i2c_unregister_device(bd.set_page[idx]);
    memset(bd, 0, sizeof(struct ee1004_bus_data));
    }
    }
#[no_mangle]
unsafe extern "C" fn ee1004_cleanup_bus_data(data: *mut c_void) {
    static void ee1004_cleanup_bus_data(void *data)
    {
    struct ee1004_bus_data *bd = data;
// Remove page select clients if this is the last device
    mutex_lock(&ee1004_bus_lock);
    ee1004_cleanup(EE1004_NUM_PAGES, bd);
    mutex_unlock(&ee1004_bus_lock);
    }
#[no_mangle]
unsafe extern "C" fn ee1004_init_bus_data(client: *mut i2c_client) -> c_int {
    static int ee1004_init_bus_data(struct i2c_client *client)
    {
    struct ee1004_bus_data *bd;
    int err, cnr = 0;
    bd = ee1004_get_bus_data(client.adapter);
    if (!bd)
    return dev_err_probe(&client.dev, -ENOSPC, "Only %d busses supported",
    EE1004_MAX_BUSSES);
    i2c_set_clientdata(client, bd);
    if (++bd.dev_count == 1) {
// Use 2 dummy devices for page select command
    for (cnr = 0; cnr < EE1004_NUM_PAGES; cnr++) {
    struct i2c_client *cl;
    cl = i2c_new_dummy_device(client.adapter, EE1004_ADDR_SET_PAGE + cnr);
    if (IS_ERR(cl)) {
    err = PTR_ERR(cl);
    goto err_out;
    }
    bd.set_page[cnr] = cl;
    }
// Remember current page to avoid unneeded page select
    err = ee1004_get_current_page(bd);
    if (err < 0)
    goto err_out;
    dev_dbg(&client.dev, "Currently selected page: %d\n", err);
    bd.current_page = err;
    }
    return 0;
    err_out:
    ee1004_cleanup(cnr, bd);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn ee1004_probe(client: *mut i2c_client) -> c_int {
    static int ee1004_probe(struct i2c_client *client)
    {
    struct nvmem_config config = {
    .dev = &client.dev,
    .name = dev_name(&client.dev),
    .id = NVMEM_DEVID_NONE,
    .owner = THIS_MODULE,
    .type = NVMEM_TYPE_EEPROM,
    .read_only = true,
    .root_only = false,
    .reg_read = ee1004_read,
    .size = EE1004_EEPROM_SIZE,
    .word_size = 1,
    .stride = 1,
    .priv = client,
    .compat = true,
    .base_dev = &client.dev,
    };
    struct nvmem_device *ndev;
    int err;
// Make sure we can operate on this adapter
    if (!i2c_check_functionality(client.adapter,
    I2C_FUNC_SMBUS_BYTE | I2C_FUNC_SMBUS_READ_I2C_BLOCK) &&
    !i2c_check_functionality(client.adapter,
    I2C_FUNC_SMBUS_BYTE | I2C_FUNC_SMBUS_READ_BYTE_DATA))
    return -EPFNOSUPPORT;
    err = i2c_smbus_read_byte(client);
    if (err < 0)
    return -ENODEV;
    mutex_lock(&ee1004_bus_lock);
    err = ee1004_init_bus_data(client);
    if (err < 0) {
    mutex_unlock(&ee1004_bus_lock);
    return err;
    }
    ee1004_probe_temp_sensor(client);
    mutex_unlock(&ee1004_bus_lock);
    err = devm_add_action_or_reset(&client.dev, ee1004_cleanup_bus_data,
    i2c_get_clientdata(client));
    if (err < 0)
    return err;
    ndev = devm_nvmem_register(&client.dev, &config);
    if (IS_ERR(ndev))
    return PTR_ERR(ndev);
    dev_info(&client.dev,
    "%u byte EE1004-compliant SPD EEPROM, read-only\n",
    EE1004_EEPROM_SIZE);
    return 0;
    }
// -------------------------------------------------------------------------
    static struct i2c_driver ee1004_driver = {
    .driver = {
    .name = "ee1004",
    },
    .probe = ee1004_probe,
    .id_table = ee1004_ids,
    };
    module_i2c_driver(ee1004_driver);
    MODULE_DESCRIPTION("Driver for EE1004-compliant DDR4 SPD EEPROMs");
    MODULE_AUTHOR("Jean Delvare");
    MODULE_LICENSE("GPL");
