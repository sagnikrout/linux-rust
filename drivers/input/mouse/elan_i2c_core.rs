//! Automatically rewritten from C to Rust
//! Source: drivers/input/mouse/elan_i2c_core.c
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
// Elan I2C/SMBus Touchpad driver
//
// Copyright (c) 2013 ELAN Microelectronics Corp.
//
// Author: 林政維 (Duson Lin) <dusonlin@emc.com.tw>
// Author: KT Liao <kt.liao@emc.com.tw>
// Version: 1.6.3
//
// Based on cyapa driver:
// copyright (c) 2011-2012 Cypress Semiconductor, Inc.
// copyright (c) 2011-2012 Google, Inc.
//
// Trademarks are the property of their respective owners.
//

pub const ELAN_VENDOR_ID: c_uint = 0x04f3;
pub const ETP_MAX_PRESSURE: c_int = 255;
pub const ETP_FWIDTH_REDUCE: c_int = 90;
pub const ETP_FINGER_WIDTH: c_int = 15;
pub const ETP_RETRY_COUNT: c_int = 3;
// quirks to control the device

// The main device structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct elan_tp_data {
    pub client: *mut i2c_client,
    pub input: *mut input_dev,
    pub /: *mut *mut *mut input_dev tp_input; / trackpoint input node,
    pub vcc: *mut regulator,
    pub ops: *const elan_transport_ops,
// for fw update
    pub fw_completion: completion,
    pub in_fw_update: bool,
    pub sysfs_mutex: mutex,
    pub max_x: c_uint,
    pub max_y: c_uint,
    pub width_x: c_uint,
    pub width_y: c_uint,
    pub x_res: c_uint,
    pub y_res: c_uint,
    pub pattern: u8,
    pub product_id: u16,
    pub fw_version: u8,
    pub sm_version: u8,
    pub iap_version: u8,
    pub fw_checksum: u16,
    pub report_features: c_uint,
    pub report_len: c_uint,
    pub pressure_adjustment: c_int,
    pub mode: u8,
    pub ic_type: u16,
    pub fw_validpage_count: u16,
    pub fw_page_size: u16,
    pub fw_page_delay: u16,
    pub fw_signature_address: u32,
    pub min_baseline: u8,
    pub max_baseline: u8,
    pub baseline_ready: bool,
    pub clickpad: u8,
    pub middle_button: bool,
    pub /: *mut *mut u32 quirks; / Various quirks,
}

#[no_mangle]
unsafe extern "C" fn elan_i2c_lookup_quirks(ic_type: u16, product_id: u16) -> u32 {
    static u32 elan_i2c_lookup_quirks(u16 ic_type, u16 product_id)
    {
    static const struct {
    u16 ic_type;
    u16 product_id;
    u32 quirks;
    } elan_i2c_quirks[] = {
    { 0x0D, ETP_PRODUCT_ID_DELBIN, ETP_QUIRK_QUICK_WAKEUP },
    { 0x0D, ETP_PRODUCT_ID_WHITEBOX, ETP_QUIRK_QUICK_WAKEUP },
    { 0x10, ETP_PRODUCT_ID_VOXEL, ETP_QUIRK_QUICK_WAKEUP },
    { 0x14, ETP_PRODUCT_ID_MAGPIE, ETP_QUIRK_QUICK_WAKEUP },
    { 0x14, ETP_PRODUCT_ID_BOBBA, ETP_QUIRK_QUICK_WAKEUP },
    };
    let mut quirks: u32 = 0;
    int i;
    for (i = 0; i < ARRAY_SIZE(elan_i2c_quirks); i++) {
    if (elan_i2c_quirks[i].ic_type == ic_type &&
    elan_i2c_quirks[i].product_id == product_id) {
    quirks = elan_i2c_quirks[i].quirks;
    }
    }
    if (ic_type >= 0x0D && product_id >= 0x123)
    quirks |= ETP_QUIRK_QUICK_WAKEUP;
    return quirks;
    }
    static int elan_get_fwinfo(u16 ic_type, u8 iap_version, u16 *validpage_count,
    u32 *signature_address, u16 *page_size,
    u16 *page_delay)
    {
// page_delay = 30;
    switch (ic_type) {
    case 0x00:
    case 0x06:
    case 0x08:
// validpage_count = 512;
    break;
    case 0x03:
    case 0x07:
    case 0x09:
    case 0x0A:
    case 0x0B:
    case 0x0C:
// validpage_count = 768;
    break;
    case 0x0D:
// validpage_count = 896;
    break;
    case 0x0E:
// validpage_count = 640;
    break;
    case 0x10:
// validpage_count = 1024;
    break;
    case 0x11:
// validpage_count = 1280;
    break;
    case 0x13:
// validpage_count = 2048;
    break;
    case 0x14:
    case 0x15:
// validpage_count = 1024;
    break;
    case 0x19:
// validpage_count = 2032;
// page_delay = 10;
    break;
    default:
// unknown ic type clear value
// validpage_count = 0;
// signature_address = 0;
// page_size = 0;
    return -ENXIO;
    }
// signature_address =
    (*validpage_count * ETP_FW_PAGE_SIZE) - ETP_FW_SIGNATURE_SIZE;
    if ((ic_type == 0x14 || ic_type == 0x15) && iap_version >= 2) {
// validpage_count /= 8;
// page_size = ETP_FW_PAGE_SIZE_512;
// page_delay = 50;
    } else if (ic_type >= 0x0D && iap_version >= 1) {
// validpage_count /= 2;
// page_size = ETP_FW_PAGE_SIZE_128;
    } else {
// page_size = ETP_FW_PAGE_SIZE;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn elan_set_power(data: *mut elan_tp_data, on: bool) -> c_int {
    static int elan_set_power(struct elan_tp_data *data, bool on)
    {
    let mut repeat: c_int = ETP_RETRY_COUNT;
    int error;
    do {
    error = data.ops.power_control(data.client, on);
    if (error >= 0)
    return 0;
    msleep(30);
    } while (--repeat > 0);
    dev_err(&data.client.dev, "failed to set power %s: %d\n",
    str_on_off(on), error);
    return error;
    }
#[no_mangle]
unsafe extern "C" fn elan_sleep(data: *mut elan_tp_data) -> c_int {
    static int elan_sleep(struct elan_tp_data *data)
    {
    let mut repeat: c_int = ETP_RETRY_COUNT;
    int error;
    do {
    error = data.ops.sleep_control(data.client, true);
    if (!error)
    return 0;
    msleep(30);
    } while (--repeat > 0);
    return error;
    }
#[no_mangle]
unsafe extern "C" fn elan_query_product(data: *mut elan_tp_data) -> c_int {
    static int elan_query_product(struct elan_tp_data *data)
    {
    int error;
    error = data.ops.get_product_id(data.client, &data.product_id);
    if (error)
    return error;
    error = data.ops.get_pattern(data.client, &data.pattern);
    if (error)
    return error;
    error = data.ops.get_sm_version(data.client, data.pattern,
    &data.ic_type, &data.sm_version,
    &data.clickpad);
    if (error)
    return error;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn elan_check_ASUS_special_fw(data: *mut elan_tp_data) -> c_int {
    static int elan_check_ASUS_special_fw(struct elan_tp_data *data)
    {
    if (data.ic_type == 0x0E) {
    switch (data.product_id) {
    case 0x05 ... 0x07:
    case 0x09:
    case 0x13:
    return true;
    }
    } else if (data.ic_type == 0x08 && data.product_id == 0x26) {
// ASUS EeeBook X205TA
    return true;
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn __elan_initialize(data: *mut elan_tp_data, skip_reset: bool) -> c_int {
    static int __elan_initialize(struct elan_tp_data *data, bool skip_reset)
    {
    struct i2c_client *client = data.client;
    let mut woken_up: bool = false;
    int error;
    if (!skip_reset) {
    error = data.ops.initialize(client);
    if (error) {
    dev_err(&client.dev, "device initialize failed: %d\n", error);
    return error;
    }
    }
    error = elan_query_product(data);
    if (error)
    return error;
//
// Some ASUS devices were shipped with firmware that requires
// touchpads to be woken up first, before attempting to switch
// them into absolute reporting mode.
//
    if (elan_check_ASUS_special_fw(data)) {
    error = data.ops.sleep_control(client, false);
    if (error) {
    dev_err(&client.dev,
    "failed to wake device up: %d\n", error);
    return error;
    }
    msleep(200);
    woken_up = true;
    }
    data.mode |= ETP_ENABLE_ABS;
    error = data.ops.set_mode(client, data.mode);
    if (error) {
    dev_err(&client.dev,
    "failed to switch to absolute mode: %d\n", error);
    return error;
    }
    if (!woken_up) {
    error = data.ops.sleep_control(client, false);
    if (error) {
    dev_err(&client.dev,
    "failed to wake device up: %d\n", error);
    return error;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn elan_initialize(data: *mut elan_tp_data, skip_reset: bool) -> c_int {
    static int elan_initialize(struct elan_tp_data *data, bool skip_reset)
    {
    let mut repeat: c_int = ETP_RETRY_COUNT;
    int error;
    do {
    error = __elan_initialize(data, skip_reset);
    if (!error)
    return 0;
    skip_reset = false;
    msleep(30);
    } while (--repeat > 0);
    return error;
    }
#[no_mangle]
unsafe extern "C" fn elan_query_device_info(data: *mut elan_tp_data) -> c_int {
    static int elan_query_device_info(struct elan_tp_data *data)
    {
    int error;
    error = data.ops.get_version(data.client, data.pattern, false,
    &data.fw_version);
    if (error)
    return error;
    error = data.ops.get_checksum(data.client, false,
    &data.fw_checksum);
    if (error)
    return error;
    error = data.ops.get_version(data.client, data.pattern,
    true, &data.iap_version);
    if (error)
    return error;
    error = data.ops.get_pressure_adjustment(data.client,
    &data.pressure_adjustment);
    if (error)
    return error;
    error = data.ops.get_report_features(data.client, data.pattern,
    &data.report_features,
    &data.report_len);
    if (error)
    return error;
    data.quirks = elan_i2c_lookup_quirks(data.ic_type, data.product_id);
    error = elan_get_fwinfo(data.ic_type, data.iap_version,
    &data.fw_validpage_count,
    &data.fw_signature_address,
    &data.fw_page_size,
    &data.fw_page_delay);
    if (error)
    dev_warn(&data.client.dev,
    "unexpected iap version %#04x (ic type: %#04x), firmware update will not work\n",
    data.iap_version, data.ic_type);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn elan_convert_resolution(val: u8, pattern: u8) -> c_uint {
    static unsigned int elan_convert_resolution(u8 val, u8 pattern)
    {
//
// pattern <= 0x01:
// (value from firmware) * 10 + 790 = dpi
// else
// ((value from firmware) + 3) * 100 = dpi
//
    int res = pattern <= 0x01 ?
    (int)(char)val * 10 + 790 : ((int)(char)val + 3) * 100;
//
// We also have to convert dpi to dots/mm (*10/254 to avoid floating
// point).
//
    return res * 10 / 254;
    }
#[no_mangle]
unsafe extern "C" fn elan_query_device_parameters(data: *mut elan_tp_data) -> c_int {
    static int elan_query_device_parameters(struct elan_tp_data *data)
    {
    struct i2c_client *client = data.client;
    unsigned int x_traces, y_traces;
    u32 x_mm, y_mm;
    u8 hw_x_res, hw_y_res;
    int error;
    if (device_property_read_u32(&client.dev,
    "touchscreen-size-x", &data.max_x) ||
    device_property_read_u32(&client.dev,
    "touchscreen-size-y", &data.max_y)) {
    error = data.ops.get_max(data.client,
    &data.max_x,
    &data.max_y);
    if (error)
    return error;
    } else {
// size is the maximum + 1
    --data.max_x;
    --data.max_y;
    }
    if (device_property_read_u32(&client.dev,
    "elan,x_traces",
    &x_traces) ||
    device_property_read_u32(&client.dev,
    "elan,y_traces",
    &y_traces)) {
    error = data.ops.get_num_traces(data.client,
    &x_traces, &y_traces);
    if (error)
    return error;
    }
    if (!x_traces || !y_traces) {
    dev_warn(&client.dev,
    "invalid trace numbers: x=%u, y=%u\n",
    x_traces, y_traces);
    data.width_x = 1;
    data.width_y = 1;
    } else {
    data.width_x = data.max_x / x_traces;
    data.width_y = data.max_y / y_traces;
    }
    if (device_property_read_u32(&client.dev,
    "touchscreen-x-mm", &x_mm) ||
    device_property_read_u32(&client.dev,
    "touchscreen-y-mm", &y_mm)) {
    error = data.ops.get_resolution(data.client,
    &hw_x_res, &hw_y_res);
    if (error)
    return error;
    data.x_res = elan_convert_resolution(hw_x_res, data.pattern);
    data.y_res = elan_convert_resolution(hw_y_res, data.pattern);
    } else {
    if (unlikely(x_mm == 0 || y_mm == 0)) {
    dev_warn(&client.dev,
    "invalid physical dimensions: x_mm=%u, y_mm=%u\n",
    x_mm, y_mm);
    data.x_res = 1;
    data.y_res = 1;
    } else {
    data.x_res = (data.max_x + 1) / x_mm;
    data.y_res = (data.max_y + 1) / y_mm;
    }
    }
    if (device_property_read_bool(&client.dev, "elan,clickpad"))
    data.clickpad = 1;
    if (device_property_read_bool(&client.dev, "elan,middle-button"))
    data.middle_button = true;
    return 0;
    }
//
// IAP firmware updater related routines
//
    static int elan_write_fw_block(struct elan_tp_data *data, u16 page_size,
    u16 page_delay, const u8 *page, u16 checksum,
    int idx)
    {
    let mut retry: c_int = ETP_RETRY_COUNT;
    int error;
    do {
    error = data.ops.write_fw_block(data.client, page_size,
    page_delay, page, checksum,
    idx);
    if (!error)
    return 0;
    dev_dbg(&data.client.dev,
    "IAP retrying page %d (error: %d)\n", idx, error);
    } while (--retry > 0);
    return error;
    }
    static int __elan_update_firmware(struct elan_tp_data *data,
    const struct firmware *fw)
    {
    struct i2c_client *client = data.client;
    struct device *dev = &client.dev;
    int i, j;
    int error;
    u16 iap_start_addr;
    u16 boot_page_count;
    let mut sw_checksum: u16 = 0, fw_checksum = 0;
    error = data.ops.prepare_fw_update(client, data.ic_type,
    data.iap_version,
    data.fw_page_size);
    if (error)
    return error;
    iap_start_addr = get_unaligned_le16(&fw.data[ETP_IAP_START_ADDR * 2]);
    boot_page_count = (iap_start_addr * 2) / data.fw_page_size;
    for (i = boot_page_count; i < data.fw_validpage_count; i++) {
    let mut checksum: u16 = 0;
    const u8 *page = &fw.data[i * data.fw_page_size];
    for (j = 0; j < data.fw_page_size; j += 2)
    checksum += ((page[j + 1] << 8) | page[j]);
    error = elan_write_fw_block(data, data.fw_page_size,
    data.fw_page_delay, page, checksum,
    i);
    if (error) {
    dev_err(dev, "write page %d fail: %d\n", i, error);
    return error;
    }
    sw_checksum += checksum;
    }
// Wait WDT reset and power on reset
    msleep(600);
    error = data.ops.finish_fw_update(client, &data.fw_completion);
    if (error)
    return error;
    error = data.ops.get_checksum(client, true, &fw_checksum);
    if (error)
    return error;
    if (sw_checksum != fw_checksum) {
    dev_err(dev, "checksum diff sw=[%04X], fw=[%04X]\n",
    sw_checksum, fw_checksum);
    return -EIO;
    }
    return 0;
    }
    static int elan_update_firmware(struct elan_tp_data *data,
    const struct firmware *fw)
    {
    struct i2c_client *client = data.client;
    int retval;
    dev_dbg(&client.dev, "Starting firmware update....\n");
    guard(disable_irq)(&client.irq);
    data.in_fw_update = true;
    retval = __elan_update_firmware(data, fw);
    if (retval) {
    dev_err(&client.dev, "firmware update failed: %d\n", retval);
    data.ops.iap_reset(client);
    } else {
// Reinitialize TP after fw is updated
    elan_initialize(data, false);
    elan_query_device_info(data);
    }
    data.in_fw_update = false;
    return retval;
    }
//
// SYSFS attributes
//
    static ssize_t elan_sysfs_read_fw_checksum(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    struct i2c_client *client = to_i2c_client(dev);
    struct elan_tp_data *data = i2c_get_clientdata(client);
    return sysfs_emit(buf, "0x%04x\n", data.fw_checksum);
    }
    static ssize_t elan_sysfs_read_product_id(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    struct i2c_client *client = to_i2c_client(dev);
    struct elan_tp_data *data = i2c_get_clientdata(client);
    return sysfs_emit(buf, ETP_PRODUCT_ID_FORMAT_STRING "\n",
    data.product_id);
    }
    static ssize_t elan_sysfs_read_fw_ver(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    struct i2c_client *client = to_i2c_client(dev);
    struct elan_tp_data *data = i2c_get_clientdata(client);
    return sysfs_emit(buf, "%d.0\n", data.fw_version);
    }
    static ssize_t elan_sysfs_read_sm_ver(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    struct i2c_client *client = to_i2c_client(dev);
    struct elan_tp_data *data = i2c_get_clientdata(client);
    return sysfs_emit(buf, "%d.0\n", data.sm_version);
    }
    static ssize_t elan_sysfs_read_iap_ver(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    struct i2c_client *client = to_i2c_client(dev);
    struct elan_tp_data *data = i2c_get_clientdata(client);
    return sysfs_emit(buf, "%d.0\n", data.iap_version);
    }
    static ssize_t elan_sysfs_update_fw(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct elan_tp_data *data = dev_get_drvdata(dev);
    int error;
    const u8 *fw_signature;
    static const u8 signature[] = {0xAA, 0x55, 0xCC, 0x33, 0xFF, 0xFF};
    if (data.fw_validpage_count == 0)
    return -EINVAL;
// Look for a firmware with the product id appended.
    const char *fw_name __free(kfree) =
    kasprintf(GFP_KERNEL, ETP_FW_NAME, data.product_id);
    if (!fw_name) {
    dev_err(dev, "failed to allocate memory for firmware name\n");
    return -ENOMEM;
    }
    dev_info(dev, "requesting fw '%s'\n", fw_name);
    const struct firmware *fw __free(firmware) = core::ptr::null_mut();
    error = request_firmware(&fw, fw_name, dev);
    if (error) {
    dev_err(dev, "failed to request firmware: %d\n", error);
    return error;
    }
    if (fw.size < data.fw_signature_address + sizeof(signature)) {
    dev_err(dev, "firmware file too small\n");
    return -EBADF;
    }
// Firmware file must match signature data
    fw_signature = &fw.data[data.fw_signature_address];
    if (memcmp(fw_signature, signature, sizeof(signature)) != 0) {
    dev_err(dev, "signature mismatch (expected %*ph, got %*ph)\n",
    (int)sizeof(signature), signature,
    (int)sizeof(signature), fw_signature);
    return -EBADF;
    }
    scoped_cond_guard(mutex_intr, return -EINTR, &data.sysfs_mutex) {
    error = elan_update_firmware(data, fw);
    if (error)
    return error;
    }
    return count;
    }
#[no_mangle]
unsafe extern "C" fn elan_calibrate(data: *mut elan_tp_data) -> c_int {
    static int elan_calibrate(struct elan_tp_data *data)
    {
    struct i2c_client *client = data.client;
    struct device *dev = &client.dev;
    let mut tries: c_int = 20;
    int retval;
    int error;
    u8 val[ETP_CALIBRATE_MAX_LEN];
    guard(disable_irq)(&client.irq);
    data.mode |= ETP_ENABLE_CALIBRATE;
    retval = data.ops.set_mode(client, data.mode);
    if (retval) {
    data.mode &= ~ETP_ENABLE_CALIBRATE;
    dev_err(dev, "failed to enable calibration mode: %d\n",
    retval);
    return retval;
    }
    retval = data.ops.calibrate(client);
    if (retval) {
    dev_err(dev, "failed to start calibration: %d\n",
    retval);
    goto out_disable_calibrate;
    }
    val[0] = 0xff;
    do {
// Wait 250ms before checking if calibration has completed.
    msleep(250);
    retval = data.ops.calibrate_result(client, val);
    if (retval)
    dev_err(dev, "failed to check calibration result: %d\n",
    retval);
#[no_mangle]
pub unsafe extern "C" fn if(0: val[0] ==) -> else {
    else if (val[0] == 0)
    break; /* calibration done */
    } while (--tries);
    if (tries == 0) {
    dev_err(dev, "failed to calibrate. Timeout.\n");
    retval = -ETIMEDOUT;
    }
    out_disable_calibrate:
    data.mode &= ~ETP_ENABLE_CALIBRATE;
    error = data.ops.set_mode(data.client, data.mode);
    if (error) {
    dev_err(dev, "failed to disable calibration mode: %d\n",
    error);
    if (!retval)
    retval = error;
    }
    return retval;
    }
    static ssize_t calibrate_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct i2c_client *client = to_i2c_client(dev);
    struct elan_tp_data *data = i2c_get_clientdata(client);
    int error;
    scoped_cond_guard(mutex_intr, return -EINTR, &data.sysfs_mutex) {
    error = elan_calibrate(data);
    if (error)
    return error;
    }
    return count;
    }
    static ssize_t elan_sysfs_read_mode(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    struct i2c_client *client = to_i2c_client(dev);
    struct elan_tp_data *data = i2c_get_clientdata(client);
    int error;
    enum tp_mode mode;
    scoped_cond_guard(mutex_intr, return -EINTR, &data.sysfs_mutex) {
    error = data.ops.iap_get_mode(data.client, &mode);
    if (error)
    return error;
    }
    return sysfs_emit(buf, "%d\n", (int)mode);
    }
    static DEVICE_ATTR(product_id, S_IRUGO, elan_sysfs_read_product_id, core::ptr::null_mut());
    static DEVICE_ATTR(firmware_version, S_IRUGO, elan_sysfs_read_fw_ver, core::ptr::null_mut());
    static DEVICE_ATTR(sample_version, S_IRUGO, elan_sysfs_read_sm_ver, core::ptr::null_mut());
    static DEVICE_ATTR(iap_version, S_IRUGO, elan_sysfs_read_iap_ver, core::ptr::null_mut());
    static DEVICE_ATTR(fw_checksum, S_IRUGO, elan_sysfs_read_fw_checksum, core::ptr::null_mut());
    static DEVICE_ATTR(mode, S_IRUGO, elan_sysfs_read_mode, core::ptr::null_mut());
    static DEVICE_ATTR(update_fw, S_IWUSR, core::ptr::null_mut(), elan_sysfs_update_fw);
    static DEVICE_ATTR_WO(calibrate);
    static struct attribute *elan_sysfs_entries[] = {
    &dev_attr_product_id.attr,
    &dev_attr_firmware_version.attr,
    &dev_attr_sample_version.attr,
    &dev_attr_iap_version.attr,
    &dev_attr_fw_checksum.attr,
    &dev_attr_calibrate.attr,
    &dev_attr_mode.attr,
    &dev_attr_update_fw.attr,
    core::ptr::null_mut(),
    };
    static const struct attribute_group elan_sysfs_group = {
    .attrs = elan_sysfs_entries,
    };
#[no_mangle]
unsafe extern "C" fn elan_acquire_baseline(data: *mut elan_tp_data) -> c_int {
    static int elan_acquire_baseline(struct elan_tp_data *data)
    {
    struct i2c_client *client = data.client;
    struct device *dev = &client.dev;
    int retval;
    int error;
    guard(disable_irq)(&client.irq);
    data.baseline_ready = false;
    data.mode |= ETP_ENABLE_CALIBRATE;
    retval = data.ops.set_mode(client, data.mode);
    if (retval) {
    data.mode &= ~ETP_ENABLE_CALIBRATE;
    dev_err(dev, "Failed to enable calibration mode to get baseline: %d\n",
    retval);
    return retval;
    }
    msleep(250);
    retval = data.ops.get_baseline_data(client, true,
    &data.max_baseline);
    if (retval) {
    dev_err(dev, "Failed to read max baseline from device: %d\n",
    retval);
    goto out_disable_calibrate;
    }
    retval = data.ops.get_baseline_data(client, false,
    &data.min_baseline);
    if (retval) {
    dev_err(dev, "Failed to read min baseline from device: %d\n",
    retval);
    goto out_disable_calibrate;
    }
    data.baseline_ready = true;
    out_disable_calibrate:
    data.mode &= ~ETP_ENABLE_CALIBRATE;
    error = data.ops.set_mode(client, data.mode);
    if (error) {
    dev_err(dev, "Failed to disable calibration mode after acquiring baseline: %d\n",
    error);
    if (!retval)
    retval = error;
    }
    return retval;
    }
    static ssize_t acquire_store(struct device *dev, struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct i2c_client *client = to_i2c_client(dev);
    struct elan_tp_data *data = i2c_get_clientdata(client);
    int error;
    scoped_cond_guard(mutex_intr, return -EINTR, &data.sysfs_mutex) {
    error = elan_acquire_baseline(data);
    if (error)
    return error;
    }
    return count;
    }
    static ssize_t min_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct i2c_client *client = to_i2c_client(dev);
    struct elan_tp_data *data = i2c_get_clientdata(client);
    scoped_guard(mutex_intr, &data.sysfs_mutex) {
    if (!data.baseline_ready)
    return -ENODATA;
    return sysfs_emit(buf, "%d", data.min_baseline);
    }
    return -EINTR;
    }
    static ssize_t max_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct i2c_client *client = to_i2c_client(dev);
    struct elan_tp_data *data = i2c_get_clientdata(client);
    scoped_guard(mutex_intr, &data.sysfs_mutex) {
    if (!data.baseline_ready)
    return -ENODATA;
    return sysfs_emit(buf, "%d", data.max_baseline);
    }
    return -EINTR;
    }
    static DEVICE_ATTR_WO(acquire);
    static DEVICE_ATTR_RO(min);
    static DEVICE_ATTR_RO(max);
    static struct attribute *elan_baseline_sysfs_entries[] = {
    &dev_attr_acquire.attr,
    &dev_attr_min.attr,
    &dev_attr_max.attr,
    core::ptr::null_mut(),
    };
    static const struct attribute_group elan_baseline_sysfs_group = {
    .name = "baseline",
    .attrs = elan_baseline_sysfs_entries,
    };
    static const struct attribute_group *elan_sysfs_groups[] = {
    &elan_sysfs_group,
    &elan_baseline_sysfs_group,
    core::ptr::null_mut()
    };
//
// Elan isr functions
//
    static void elan_report_contact(struct elan_tp_data *data, int contact_num,
    bool contact_valid, bool high_precision,
    u8 *packet, u8 *finger_data)
    {
    struct input_dev *input = data.input;
    unsigned int pos_x, pos_y;
    unsigned int pressure, scaled_pressure;
    if (contact_valid) {
    if (high_precision) {
    pos_x = get_unaligned_be16(&finger_data[0]);
    pos_y = get_unaligned_be16(&finger_data[2]);
    } else {
    pos_x = ((finger_data[0] & 0xf0) << 4) | finger_data[1];
    pos_y = ((finger_data[0] & 0x0f) << 8) | finger_data[2];
    }
    if (pos_x > data.max_x || pos_y > data.max_y) {
    dev_dbg(input.dev.parent,
    "[%d] x=%d y=%d over max (%d, %d)",
    contact_num, pos_x, pos_y,
    data.max_x, data.max_y);
    return;
    }
    pressure = finger_data[4];
    scaled_pressure = pressure + data.pressure_adjustment;
    if (scaled_pressure > ETP_MAX_PRESSURE)
    scaled_pressure = ETP_MAX_PRESSURE;
    input_mt_slot(input, contact_num);
    input_mt_report_slot_state(input, MT_TOOL_FINGER, true);
    input_report_abs(input, ABS_MT_POSITION_X, pos_x);
    input_report_abs(input, ABS_MT_POSITION_Y, data.max_y - pos_y);
    input_report_abs(input, ABS_MT_PRESSURE, scaled_pressure);
    if (data.report_features & ETP_FEATURE_REPORT_MK) {
    unsigned int mk_x, mk_y, area_x, area_y;
    int adj_width_x, adj_width_y;
    u8 mk_data = high_precision ?
    packet[ETP_MK_DATA_OFFSET + contact_num] :
    finger_data[3];
    mk_x = mk_data & 0x0f;
    mk_y = mk_data >> 4;
//
// To avoid treating large finger as palm, let's reduce
// the width x and y per trace.
//
    adj_width_x = data.width_x > ETP_FWIDTH_REDUCE ?
    data.width_x - ETP_FWIDTH_REDUCE : 0;
    adj_width_y = data.width_y > ETP_FWIDTH_REDUCE ?
    data.width_y - ETP_FWIDTH_REDUCE : 0;
    area_x = mk_x * adj_width_x;
    area_y = mk_y * adj_width_y;
    input_report_abs(input, ABS_TOOL_WIDTH, mk_x);
    input_report_abs(input, ABS_MT_TOUCH_MAJOR,
    max(area_x, area_y));
    input_report_abs(input, ABS_MT_TOUCH_MINOR,
    min(area_x, area_y));
    }
    } else {
    input_mt_slot(input, contact_num);
    input_mt_report_slot_inactive(input);
    }
    }
    static void elan_report_absolute(struct elan_tp_data *data, u8 *packet,
    bool high_precision)
    {
    struct input_dev *input = data.input;
    u8 *finger_data = &packet[ETP_FINGER_DATA_OFFSET];
    int i;
    let mut tp_info: u8 = packet[ETP_TOUCH_INFO_OFFSET];
    let mut hover_info: u8 = packet[ETP_HOVER_INFO_OFFSET];
    bool contact_valid, hover_event;
    pm_wakeup_event(&data.client.dev, 0);
    hover_event = hover_info & BIT(6);
    for (i = 0; i < ETP_MAX_FINGERS; i++) {
    contact_valid = tp_info & BIT(3 + i);
    elan_report_contact(data, i, contact_valid, high_precision,
    packet, finger_data);
    if (contact_valid)
    finger_data += ETP_FINGER_DATA_LEN;
    }
    input_report_key(input, BTN_LEFT,   tp_info & BIT(0));
    input_report_key(input, BTN_MIDDLE, tp_info & BIT(2));
    input_report_key(input, BTN_RIGHT,  tp_info & BIT(1));
    input_report_abs(input, ABS_DISTANCE, hover_event != 0);
    input_mt_report_pointer_emulation(input, true);
    input_sync(input);
    }
#[no_mangle]
unsafe extern "C" fn elan_report_trackpoint(data: *mut elan_tp_data, report: *mut u8) {
    static void elan_report_trackpoint(struct elan_tp_data *data, u8 *report)
    {
    struct input_dev *input = data.tp_input;
    u8 *packet = &report[ETP_REPORT_ID_OFFSET + 1];
    int x, y;
    pm_wakeup_event(&data.client.dev, 0);
    if (!data.tp_input) {
    dev_warn_once(&data.client.dev,
    "received a trackpoint report while no trackpoint device has been created. Please report upstream.\n");
    return;
    }
    input_report_key(input, BTN_LEFT, packet[0] & 0x01);
    input_report_key(input, BTN_RIGHT, packet[0] & 0x02);
    input_report_key(input, BTN_MIDDLE, packet[0] & 0x04);
    if ((packet[3] & 0x0F) == 0x06) {
    x = packet[4] - (int)((packet[1] ^ 0x80) << 1);
    y = (int)((packet[2] ^ 0x80) << 1) - packet[5];
    input_report_rel(input, REL_X, x);
    input_report_rel(input, REL_Y, y);
    }
    input_sync(input);
    }
#[no_mangle]
unsafe extern "C" fn elan_isr(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t elan_isr(int irq, void *dev_id)
    {
    struct elan_tp_data *data = dev_id;
    int error;
    u8 report[ETP_MAX_REPORT_LEN];
//
// When device is connected to i2c bus, when all IAP page writes
// complete, the driver will receive interrupt and must read
// 0000 to confirm that IAP is finished.
//
    if (data.in_fw_update) {
    complete(&data.fw_completion);
    goto out;
    }
    error = data.ops.get_report(data.client, report, data.report_len);
    if (error)
    goto out;
    switch (report[ETP_REPORT_ID_OFFSET]) {
    case ETP_REPORT_ID:
    elan_report_absolute(data, report, false);
    break;
    case ETP_REPORT_ID2:
    elan_report_absolute(data, report, true);
    break;
    case ETP_TP_REPORT_ID:
    case ETP_TP_REPORT_ID2:
    elan_report_trackpoint(data, report);
    break;
    default:
    dev_err(&data.client.dev, "invalid report id data (%x)\n",
    report[ETP_REPORT_ID_OFFSET]);
    }
    out:
    return IRQ_HANDLED;
    }
//
// Elan initialization functions
//
#[no_mangle]
unsafe extern "C" fn elan_setup_trackpoint_input_device(data: *mut elan_tp_data) -> c_int {
    static int elan_setup_trackpoint_input_device(struct elan_tp_data *data)
    {
    struct device *dev = &data.client.dev;
    struct input_dev *input;
    input = devm_input_allocate_device(dev);
    if (!input)
    return -ENOMEM;
    input.name = "Elan TrackPoint";
    input.id.bustype = BUS_I2C;
    input.id.vendor = ELAN_VENDOR_ID;
    input.id.product = data.product_id;
    input_set_drvdata(input, data);
    input_set_capability(input, EV_REL, REL_X);
    input_set_capability(input, EV_REL, REL_Y);
    input_set_capability(input, EV_KEY, BTN_LEFT);
    input_set_capability(input, EV_KEY, BTN_RIGHT);
    input_set_capability(input, EV_KEY, BTN_MIDDLE);
    __set_bit(INPUT_PROP_POINTER, input.propbit);
    __set_bit(INPUT_PROP_POINTING_STICK, input.propbit);
    data.tp_input = input;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn elan_setup_input_device(data: *mut elan_tp_data) -> c_int {
    static int elan_setup_input_device(struct elan_tp_data *data)
    {
    struct device *dev = &data.client.dev;
    struct input_dev *input;
    let mut max_width: c_uint = max(data.width_x, data.width_y);
    let mut min_width: c_uint = min(data.width_x, data.width_y);
    int error;
    input = devm_input_allocate_device(dev);
    if (!input)
    return -ENOMEM;
    input.name = "Elan Touchpad";
    input.id.bustype = BUS_I2C;
    input.id.vendor = ELAN_VENDOR_ID;
    input.id.product = data.product_id;
    input_set_drvdata(input, data);
    error = input_mt_init_slots(input, ETP_MAX_FINGERS,
    INPUT_MT_POINTER | INPUT_MT_DROP_UNUSED);
    if (error) {
    dev_err(dev, "failed to initialize MT slots: %d\n", error);
    return error;
    }
    __set_bit(EV_ABS, input.evbit);
    __set_bit(INPUT_PROP_POINTER, input.propbit);
    if (data.clickpad) {
    __set_bit(INPUT_PROP_BUTTONPAD, input.propbit);
    } else {
    __set_bit(BTN_RIGHT, input.keybit);
    if (data.middle_button)
    __set_bit(BTN_MIDDLE, input.keybit);
    }
    __set_bit(BTN_LEFT, input.keybit);
// Set up ST parameters
    input_set_abs_params(input, ABS_X, 0, data.max_x, 0, 0);
    input_set_abs_params(input, ABS_Y, 0, data.max_y, 0, 0);
    input_abs_set_res(input, ABS_X, data.x_res);
    input_abs_set_res(input, ABS_Y, data.y_res);
    input_set_abs_params(input, ABS_PRESSURE, 0, ETP_MAX_PRESSURE, 0, 0);
    if (data.report_features & ETP_FEATURE_REPORT_MK)
    input_set_abs_params(input, ABS_TOOL_WIDTH,
    0, ETP_FINGER_WIDTH, 0, 0);
    input_set_abs_params(input, ABS_DISTANCE, 0, 1, 0, 0);
// And MT parameters
    input_set_abs_params(input, ABS_MT_POSITION_X, 0, data.max_x, 0, 0);
    input_set_abs_params(input, ABS_MT_POSITION_Y, 0, data.max_y, 0, 0);
    input_abs_set_res(input, ABS_MT_POSITION_X, data.x_res);
    input_abs_set_res(input, ABS_MT_POSITION_Y, data.y_res);
    input_set_abs_params(input, ABS_MT_PRESSURE, 0,
    ETP_MAX_PRESSURE, 0, 0);
    if (data.report_features & ETP_FEATURE_REPORT_MK) {
    input_set_abs_params(input, ABS_MT_TOUCH_MAJOR,
    0, ETP_FINGER_WIDTH * max_width, 0, 0);
    input_set_abs_params(input, ABS_MT_TOUCH_MINOR,
    0, ETP_FINGER_WIDTH * min_width, 0, 0);
    }
    data.input = input;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn elan_disable_regulator(_data: *mut c_void) {
    static void elan_disable_regulator(void *_data)
    {
    struct elan_tp_data *data = _data;
    regulator_disable(data.vcc);
    }
#[no_mangle]
unsafe extern "C" fn elan_probe(client: *mut i2c_client) -> c_int {
    static int elan_probe(struct i2c_client *client)
    {
    const struct elan_transport_ops *transport_ops;
    struct device *dev = &client.dev;
    struct elan_tp_data *data;
    unsigned long irqflags;
    int error;
    if (IS_ENABLED(CONFIG_MOUSE_ELAN_I2C_I2C) &&
    i2c_check_functionality(client.adapter, I2C_FUNC_I2C)) {
    transport_ops = &elan_i2c_ops;
    } else if (IS_ENABLED(CONFIG_MOUSE_ELAN_I2C_SMBUS) &&
    i2c_check_functionality(client.adapter,
    I2C_FUNC_SMBUS_BYTE_DATA |
    I2C_FUNC_SMBUS_BLOCK_DATA |
    I2C_FUNC_SMBUS_I2C_BLOCK)) {
    transport_ops = &elan_smbus_ops;
    } else {
    dev_err(dev, "not a supported I2C/SMBus adapter\n");
    return -EIO;
    }
    data = devm_kzalloc(dev, sizeof(struct elan_tp_data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    i2c_set_clientdata(client, data);
    data.ops = transport_ops;
    data.client = client;
    init_completion(&data.fw_completion);
    mutex_init(&data.sysfs_mutex);
    data.vcc = devm_regulator_get(dev, "vcc");
    if (IS_ERR(data.vcc))
    return dev_err_probe(dev, PTR_ERR(data.vcc), "Failed to get 'vcc' regulator\n");
    error = regulator_enable(data.vcc);
    if (error) {
    dev_err(dev, "Failed to enable regulator: %d\n", error);
    return error;
    }
    error = devm_add_action_or_reset(dev, elan_disable_regulator, data);
    if (error) {
    dev_err(dev, "Failed to add disable regulator action: %d\n",
    error);
    return error;
    }
// Make sure there is something at this address
    error = i2c_smbus_read_byte(client);
    if (error < 0) {
    dev_dbg(&client.dev, "nothing at this address: %d\n", error);
    return -ENXIO;
    }
// Initialize the touchpad.
    error = elan_initialize(data, false);
    if (error)
    return error;
    error = elan_query_device_info(data);
    if (error)
    return error;
    error = elan_query_device_parameters(data);
    if (error)
    return error;
    dev_info(dev,
    "Elan Touchpad: Module ID: 0x%04x, Firmware: 0x%04x, Sample: 0x%04x, IAP: 0x%04x\n",
    data.product_id,
    data.fw_version,
    data.sm_version,
    data.iap_version);
    dev_dbg(dev,
    "Elan Touchpad Extra Information:\n"
    "    Max ABS X,Y:   %d,%d\n"
    "    Width X,Y:   %d,%d\n"
    "    Resolution X,Y:   %d,%d (dots/mm)\n"
    "    ic type: 0x%x\n"
    "    info pattern: 0x%x\n",
    data.max_x, data.max_y,
    data.width_x, data.width_y,
    data.x_res, data.y_res,
    data.ic_type, data.pattern);
// Set up input device properties based on queried parameters.
    error = elan_setup_input_device(data);
    if (error)
    return error;
    if (device_property_read_bool(&client.dev, "elan,trackpoint")) {
    error = elan_setup_trackpoint_input_device(data);
    if (error)
    return error;
    }
//
// Platform code (ACPI, DTS) should normally set up interrupt
// for us, but in case it did not let's fall back to using falling
// edge to be compatible with older Chromebooks.
//
    irqflags = irq_get_trigger_type(client.irq);
    if (!irqflags)
    irqflags = IRQF_TRIGGER_FALLING;
    error = devm_request_threaded_irq(dev, client.irq, core::ptr::null_mut(), elan_isr,
    irqflags | IRQF_ONESHOT,
    client.name, data);
    if (error) {
    dev_err(dev, "cannot register irq=%d\n", client.irq);
    return error;
    }
    error = input_register_device(data.input);
    if (error) {
    dev_err(dev, "failed to register input device: %d\n", error);
    return error;
    }
    if (data.tp_input) {
    error = input_register_device(data.tp_input);
    if (error) {
    dev_err(&client.dev,
    "failed to register TrackPoint input device: %d\n",
    error);
    return error;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn __elan_suspend(data: *mut elan_tp_data) -> c_int {
    static int __elan_suspend(struct elan_tp_data *data)
    {
    struct i2c_client *client = data.client;
    int error;
    if (device_may_wakeup(&client.dev))
    return elan_sleep(data);
// Touchpad is not a wakeup source
    error = elan_set_power(data, false);
    if (error)
    return error;
    error = regulator_disable(data.vcc);
    if (error) {
    dev_err(&client.dev,
    "failed to disable regulator when suspending: %d\n",
    error);
// Attempt to power the chip back up
    elan_set_power(data, true);
    return error;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn elan_suspend(dev: *mut device) -> c_int {
    static int elan_suspend(struct device *dev)
    {
    struct i2c_client *client = to_i2c_client(dev);
    struct elan_tp_data *data = i2c_get_clientdata(client);
    int error;
//
// We are taking the mutex to make sure sysfs operations are
// complete before we attempt to bring the device into low[er]
// power mode.
//
    scoped_cond_guard(mutex_intr, return -EINTR, &data.sysfs_mutex) {
    disable_irq(client.irq);
    error = __elan_suspend(data);
    if (error) {
    enable_irq(client.irq);
    return error;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn elan_resume(dev: *mut device) -> c_int {
    static int elan_resume(struct device *dev)
    {
    struct i2c_client *client = to_i2c_client(dev);
    struct elan_tp_data *data = i2c_get_clientdata(client);
    int error;
    if (!device_may_wakeup(dev)) {
    error = regulator_enable(data.vcc);
    if (error) {
    dev_err(dev, "error %d enabling regulator\n", error);
    goto err;
    }
    }
    error = elan_set_power(data, true);
    if (error) {
    dev_err(dev, "power up when resuming failed: %d\n", error);
    goto err;
    }
    error = elan_initialize(data, data.quirks & ETP_QUIRK_QUICK_WAKEUP);
    if (error)
    dev_err(dev, "initialize when resuming failed: %d\n", error);
    err:
    enable_irq(data.client.irq);
    return error;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(elan_pm_ops, elan_suspend, elan_resume);
    static const struct i2c_device_id elan_id[] = {
    { .name = DRIVER_NAME },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, elan_id);

    MODULE_DEVICE_TABLE(acpi, elan_acpi_id);

    static const struct of_device_id elan_of_match[] = {
    { .compatible = "elan,ekth3000" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, elan_of_match);

    static struct i2c_driver elan_driver = {
    .driver = {
    .name	= DRIVER_NAME,
    .pm	= pm_sleep_ptr(&elan_pm_ops),
    .acpi_match_table = ACPI_PTR(elan_acpi_id),
    .of_match_table = of_match_ptr(elan_of_match),
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    .dev_groups = elan_sysfs_groups,
    },
    .probe		= elan_probe,
    .id_table	= elan_id,
    };
    module_i2c_driver(elan_driver);
    MODULE_AUTHOR("Duson Lin <dusonlin@emc.com.tw>");
    MODULE_DESCRIPTION("Elan I2C/SMBus Touchpad driver");
    MODULE_LICENSE("GPL");
