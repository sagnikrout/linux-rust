//! Automatically rewritten from C to Rust
//! Source: drivers/input/mouse/elan_i2c_smbus.c
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
// Elan I2C/SMBus Touchpad driver - SMBus interface
//
// Copyright (c) 2013 ELAN Microelectronics Corp.
//
// Author: 林政維 (Duson Lin) <dusonlin@emc.com.tw>
//
// Based on cyapa driver:
// copyright (c) 2011-2012 Cypress Semiconductor, Inc.
// copyright (c) 2011-2012 Google, Inc.
//
// Trademarks are the property of their respective owners.
//

// Elan SMbus commands
pub const ETP_SMBUS_IAP_CMD: c_uint = 0x00;
pub const ETP_SMBUS_ENABLE_TP: c_uint = 0x20;
pub const ETP_SMBUS_SLEEP_CMD: c_uint = 0x21;
pub const ETP_SMBUS_IAP_PASSWORD_WRITE: c_uint = 0x29;
pub const ETP_SMBUS_IAP_PASSWORD_READ: c_uint = 0x80;
pub const ETP_SMBUS_WRITE_FW_BLOCK: c_uint = 0x2A;
pub const ETP_SMBUS_IAP_RESET_CMD: c_uint = 0x2B;
pub const ETP_SMBUS_RANGE_CMD: c_uint = 0xA0;
pub const ETP_SMBUS_FW_VERSION_CMD: c_uint = 0xA1;
pub const ETP_SMBUS_XY_TRACENUM_CMD: c_uint = 0xA2;
pub const ETP_SMBUS_SM_VERSION_CMD: c_uint = 0xA3;
pub const ETP_SMBUS_UNIQUEID_CMD: c_uint = 0xA3;
pub const ETP_SMBUS_RESOLUTION_CMD: c_uint = 0xA4;
pub const ETP_SMBUS_HELLOPACKET_CMD: c_uint = 0xA7;
pub const ETP_SMBUS_PACKET_QUERY: c_uint = 0xA8;
pub const ETP_SMBUS_IAP_VERSION_CMD: c_uint = 0xAC;
pub const ETP_SMBUS_IAP_CTRL_CMD: c_uint = 0xAD;
pub const ETP_SMBUS_IAP_CHECKSUM_CMD: c_uint = 0xAE;
pub const ETP_SMBUS_FW_CHECKSUM_CMD: c_uint = 0xAF;
pub const ETP_SMBUS_MAX_BASELINE_CMD: c_uint = 0xC3;
pub const ETP_SMBUS_MIN_BASELINE_CMD: c_uint = 0xC4;
pub const ETP_SMBUS_CALIBRATE_QUERY: c_uint = 0xC5;
pub const ETP_SMBUS_REPORT_LEN: c_int = 32;
pub const ETP_SMBUS_REPORT_LEN2: c_int = 7;
pub const ETP_SMBUS_REPORT_OFFSET: c_int = 2;
pub const ETP_SMBUS_HELLOPACKET_LEN: c_int = 5;
pub const ETP_SMBUS_IAP_PASSWORD: c_uint = 0x1234;

#[no_mangle]
unsafe extern "C" fn elan_smbus_initialize(client: *mut i2c_client) -> c_int {
    static int elan_smbus_initialize(struct i2c_client *client)
    {
    u8 check[ETP_SMBUS_HELLOPACKET_LEN] = { 0x55, 0x55, 0x55, 0x55, 0x55 };
    u8 values[I2C_SMBUS_BLOCK_MAX] = {0};
    int len, error;
// Get hello packet
    len = i2c_smbus_read_block_data(client,
    ETP_SMBUS_HELLOPACKET_CMD, values);
    if (len != ETP_SMBUS_HELLOPACKET_LEN) {
    dev_err(&client.dev, "hello packet length fail: %d\n", len);
    error = len < 0 ? len : -EIO;
    return error;
    }
// compare hello packet
    if (memcmp(values, check, ETP_SMBUS_HELLOPACKET_LEN)) {
    dev_err(&client.dev, "hello packet fail [%*ph]\n",
    ETP_SMBUS_HELLOPACKET_LEN, values);
    return -ENXIO;
    }
// enable tp
    error = i2c_smbus_write_byte(client, ETP_SMBUS_ENABLE_TP);
    if (error) {
    dev_err(&client.dev, "failed to enable touchpad: %d\n", error);
    return error;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn elan_smbus_set_mode(client: *mut i2c_client, mode: u8) -> c_int {
    static int elan_smbus_set_mode(struct i2c_client *client, u8 mode)
    {
    u8 cmd[4] = { 0x00, 0x07, 0x00, mode };
    return i2c_smbus_write_block_data(client, ETP_SMBUS_IAP_CMD,
    sizeof(cmd), cmd);
    }
#[no_mangle]
unsafe extern "C" fn elan_smbus_sleep_control(client: *mut i2c_client, sleep: bool) -> c_int {
    static int elan_smbus_sleep_control(struct i2c_client *client, bool sleep)
    {
    if (sleep)
    return i2c_smbus_write_byte(client, ETP_SMBUS_SLEEP_CMD);
    else
    return 0; /* XXX should we send ETP_SMBUS_ENABLE_TP here? */
    }
#[no_mangle]
unsafe extern "C" fn elan_smbus_power_control(client: *mut i2c_client, enable: bool) -> c_int {
    static int elan_smbus_power_control(struct i2c_client *client, bool enable)
    {
    return 0; /* A no-op */
    }
#[no_mangle]
unsafe extern "C" fn elan_smbus_calibrate(client: *mut i2c_client) -> c_int {
    static int elan_smbus_calibrate(struct i2c_client *client)
    {
    u8 cmd[4] = { 0x00, 0x08, 0x00, 0x01 };
    return i2c_smbus_write_block_data(client, ETP_SMBUS_IAP_CMD,
    sizeof(cmd), cmd);
    }
#[no_mangle]
unsafe extern "C" fn elan_smbus_calibrate_result(client: *mut i2c_client, val: *mut u8) -> c_int {
    static int elan_smbus_calibrate_result(struct i2c_client *client, u8 *val)
    {
    int error;
    u8 buf[I2C_SMBUS_BLOCK_MAX] = {0};
    BUILD_BUG_ON(ETP_CALIBRATE_MAX_LEN > sizeof(buf));
    error = i2c_smbus_read_block_data(client,
    ETP_SMBUS_CALIBRATE_QUERY, buf);
    if (error < 0)
    return error;
    memcpy(val, buf, ETP_CALIBRATE_MAX_LEN);
    return 0;
    }
    static int elan_smbus_get_baseline_data(struct i2c_client *client,
    bool max_baseline, u8 *value)
    {
    int error;
    u8 val[I2C_SMBUS_BLOCK_MAX] = {0};
    error = i2c_smbus_read_block_data(client,
    max_baseline ?
    ETP_SMBUS_MAX_BASELINE_CMD :
    ETP_SMBUS_MIN_BASELINE_CMD,
    val);
    if (error < 0)
    return error;
// value = be16_to_cpup((__be16 *)val);
    return 0;
    }
    static int elan_smbus_get_version(struct i2c_client *client,
    u8 pattern, bool iap, u8 *version)
    {
    int error;
    u8 val[I2C_SMBUS_BLOCK_MAX] = {0};
    error = i2c_smbus_read_block_data(client,
    iap ? ETP_SMBUS_IAP_VERSION_CMD :
    ETP_SMBUS_FW_VERSION_CMD,
    val);
    if (error < 0) {
    dev_err(&client.dev, "failed to get %s version: %d\n",
    iap ? "IAP" : "FW", error);
    return error;
    }
// version = val[2];
    return 0;
    }
    static int elan_smbus_get_sm_version(struct i2c_client *client, u8 pattern,
    u16 *ic_type, u8 *version, u8 *clickpad)
    {
    int error;
    u8 val[I2C_SMBUS_BLOCK_MAX] = {0};
    error = i2c_smbus_read_block_data(client,
    ETP_SMBUS_SM_VERSION_CMD, val);
    if (error < 0) {
    dev_err(&client.dev, "failed to get SM version: %d\n", error);
    return error;
    }
// version = val[0];
// ic_type = val[1];
// clickpad = val[0] & 0x10;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn elan_smbus_get_product_id(client: *mut i2c_client, id: *mut u16) -> c_int {
    static int elan_smbus_get_product_id(struct i2c_client *client, u16 *id)
    {
    int error;
    u8 val[I2C_SMBUS_BLOCK_MAX] = {0};
    error = i2c_smbus_read_block_data(client,
    ETP_SMBUS_UNIQUEID_CMD, val);
    if (error < 0) {
    dev_err(&client.dev, "failed to get product ID: %d\n", error);
    return error;
    }
// id = be16_to_cpup((__be16 *)val);
    return 0;
    }
    static int elan_smbus_get_checksum(struct i2c_client *client,
    bool iap, u16 *csum)
    {
    int error;
    u8 val[I2C_SMBUS_BLOCK_MAX] = {0};
    error = i2c_smbus_read_block_data(client,
    iap ? ETP_SMBUS_FW_CHECKSUM_CMD :
    ETP_SMBUS_IAP_CHECKSUM_CMD,
    val);
    if (error < 0) {
    dev_err(&client.dev, "failed to get %s checksum: %d\n",
    iap ? "IAP" : "FW", error);
    return error;
    }
// csum = be16_to_cpup((__be16 *)val);
    return 0;
    }
    static int elan_smbus_get_max(struct i2c_client *client,
    unsigned int *max_x, unsigned int *max_y)
    {
    int ret;
    int error;
    u8 val[I2C_SMBUS_BLOCK_MAX] = {0};
    ret = i2c_smbus_read_block_data(client, ETP_SMBUS_RANGE_CMD, val);
    if (ret != 3) {
    error = ret < 0 ? ret : -EIO;
    dev_err(&client.dev, "failed to get dimensions: %d\n", error);
    return error;
    }
// max_x = (0x0f & val[0]) << 8 | val[1];
// max_y = (0xf0 & val[0]) << 4 | val[2];
    return 0;
    }
    static int elan_smbus_get_resolution(struct i2c_client *client,
    u8 *hw_res_x, u8 *hw_res_y)
    {
    int ret;
    int error;
    u8 val[I2C_SMBUS_BLOCK_MAX] = {0};
    ret = i2c_smbus_read_block_data(client, ETP_SMBUS_RESOLUTION_CMD, val);
    if (ret != 3) {
    error = ret < 0 ? ret : -EIO;
    dev_err(&client.dev, "failed to get resolution: %d\n", error);
    return error;
    }
// hw_res_x = val[1] & 0x0F;
// hw_res_y = (val[1] & 0xF0) >> 4;
    return 0;
    }
    static int elan_smbus_get_num_traces(struct i2c_client *client,
    unsigned int *x_traces,
    unsigned int *y_traces)
    {
    int ret;
    int error;
    u8 val[I2C_SMBUS_BLOCK_MAX] = {0};
    ret = i2c_smbus_read_block_data(client, ETP_SMBUS_XY_TRACENUM_CMD, val);
    if (ret != 3) {
    error = ret < 0 ? ret : -EIO;
    dev_err(&client.dev, "failed to get trace info: %d\n", error);
    return error;
    }
// x_traces = val[1];
// y_traces = val[2];
    return 0;
    }
    static int elan_smbus_get_pressure_adjustment(struct i2c_client *client,
    int *adjustment)
    {
// adjustment = ETP_PRESSURE_OFFSET;
    return 0;
    }
    static int elan_smbus_iap_get_mode(struct i2c_client *client,
    enum tp_mode *mode)
    {
    int error;
    u16 constant;
    u8 val[I2C_SMBUS_BLOCK_MAX] = {0};
    error = i2c_smbus_read_block_data(client, ETP_SMBUS_IAP_CTRL_CMD, val);
    if (error < 0) {
    dev_err(&client.dev, "failed to read iap ctrol register: %d\n",
    error);
    return error;
    }
    constant = be16_to_cpup((__be16 *)val);
    dev_dbg(&client.dev, "iap control reg: 0x%04x.\n", constant);
// mode = (constant & ETP_SMBUS_IAP_MODE_ON) ? IAP_MODE : MAIN_MODE;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn elan_smbus_iap_reset(client: *mut i2c_client) -> c_int {
    static int elan_smbus_iap_reset(struct i2c_client *client)
    {
    int error;
    error = i2c_smbus_write_byte(client, ETP_SMBUS_IAP_RESET_CMD);
    if (error) {
    dev_err(&client.dev, "cannot reset IC: %d\n", error);
    return error;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn elan_smbus_set_flash_key(client: *mut i2c_client) -> c_int {
    static int elan_smbus_set_flash_key(struct i2c_client *client)
    {
    int error;
    u8 cmd[4] = { 0x00, 0x0B, 0x00, 0x5A };
    error = i2c_smbus_write_block_data(client, ETP_SMBUS_IAP_CMD,
    sizeof(cmd), cmd);
    if (error) {
    dev_err(&client.dev, "cannot set flash key: %d\n", error);
    return error;
    }
    return 0;
    }
    static int elan_smbus_prepare_fw_update(struct i2c_client *client, u16 ic_type,
    u8 iap_version, u16 fw_page_size)
    {
    struct device *dev = &client.dev;
    int len;
    int error;
    enum tp_mode mode;
    u8 val[I2C_SMBUS_BLOCK_MAX] = {0};
    u8 cmd[4] = {0x0F, 0x78, 0x00, 0x06};
    u16 password;
// Get FW in which mode	(IAP_MODE/MAIN_MODE)
    error = elan_smbus_iap_get_mode(client, &mode);
    if (error)
    return error;
    if (mode == MAIN_MODE) {
// set flash key
    error = elan_smbus_set_flash_key(client);
    if (error)
    return error;
// write iap password
    if (i2c_smbus_write_byte(client,
    ETP_SMBUS_IAP_PASSWORD_WRITE) < 0) {
    dev_err(dev, "cannot write iap password\n");
    return -EIO;
    }
    error = i2c_smbus_write_block_data(client, ETP_SMBUS_IAP_CMD,
    sizeof(cmd), cmd);
    if (error) {
    dev_err(dev, "failed to write iap password: %d\n",
    error);
    return error;
    }
//
// Read back password to make sure we enabled flash
// successfully.
//
    len = i2c_smbus_read_block_data(client,
    ETP_SMBUS_IAP_PASSWORD_READ,
    val);
    if (len < (int)sizeof(u16)) {
    error = len < 0 ? len : -EIO;
    dev_err(dev, "failed to read iap password: %d\n",
    error);
    return error;
    }
    password = be16_to_cpup((__be16 *)val);
    if (password != ETP_SMBUS_IAP_PASSWORD) {
    dev_err(dev, "wrong iap password = 0x%X\n", password);
    return -EIO;
    }
// Wait 30ms for MAIN_MODE change to IAP_MODE
    msleep(30);
    }
    error = elan_smbus_set_flash_key(client);
    if (error)
    return error;
// Reset IC
    error = elan_smbus_iap_reset(client);
    if (error)
    return error;
    return 0;
    }
    static int elan_smbus_write_fw_block(struct i2c_client *client, u16 fw_page_size,
    u16 fw_page_delay, const u8 *page, u16 checksum,
    int idx)
    {
    struct device *dev = &client.dev;
    int error;
    u16 result;
    u8 val[I2C_SMBUS_BLOCK_MAX] = {0};
//
// Due to the limitation of smbus protocol limiting
// transfer to 32 bytes at a time, we must split block
// in 2 transfers.
//
    error = i2c_smbus_write_block_data(client,
    ETP_SMBUS_WRITE_FW_BLOCK,
    fw_page_size / 2,
    page);
    if (error) {
    dev_err(dev, "Failed to write page %d (part %d): %d\n",
    idx, 1, error);
    return error;
    }
    error = i2c_smbus_write_block_data(client,
    ETP_SMBUS_WRITE_FW_BLOCK,
    fw_page_size / 2,
    page + fw_page_size / 2);
    if (error) {
    dev_err(dev, "Failed to write page %d (part %d): %d\n",
    idx, 2, error);
    return error;
    }
// Wait for F/W to update one page ROM data.
    usleep_range(8000, 10000);
    error = i2c_smbus_read_block_data(client,
    ETP_SMBUS_IAP_CTRL_CMD, val);
    if (error < 0) {
    dev_err(dev, "Failed to read IAP write result: %d\n",
    error);
    return error;
    }
    result = be16_to_cpup((__be16 *)val);
    if (result & (ETP_FW_IAP_PAGE_ERR | ETP_FW_IAP_INTF_ERR)) {
    dev_err(dev, "IAP reports failed write: %04hx\n",
    result);
    return -EIO;
    }
    return 0;
    }
    static int elan_smbus_get_report_features(struct i2c_client *client, u8 pattern,
    unsigned int *features,
    unsigned int *report_len)
    {
//
// SMBus controllers with pattern 2 lack area info, as newer
// high-precision packets use that space for coordinates.
//
// features = pattern <= 0x01 ? ETP_FEATURE_REPORT_MK : 0;
// report_len = ETP_SMBUS_REPORT_LEN;
    return 0;
    }
    static int elan_smbus_get_report(struct i2c_client *client,
    u8 *report, unsigned int report_len)
    {
    int len;
    BUILD_BUG_ON(I2C_SMBUS_BLOCK_MAX > ETP_SMBUS_REPORT_LEN);
    len = i2c_smbus_read_block_data(client,
    ETP_SMBUS_PACKET_QUERY,
    &report[ETP_SMBUS_REPORT_OFFSET]);
    if (len < 0) {
    dev_err(&client.dev, "failed to read report data: %d\n", len);
    return len;
    }
    if (report[ETP_REPORT_ID_OFFSET] == ETP_TP_REPORT_ID2)
    report_len = ETP_SMBUS_REPORT_LEN2;
    if (len != report_len) {
    dev_err(&client.dev,
    "wrong report length (%d vs %d expected)\n",
    len, report_len);
    return -EIO;
    }
    return 0;
    }
    static int elan_smbus_finish_fw_update(struct i2c_client *client,
    struct completion *fw_completion)
    {
// No special handling unlike I2C transport
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn elan_smbus_get_pattern(client: *mut i2c_client, pattern: *mut u8) -> c_int {
    static int elan_smbus_get_pattern(struct i2c_client *client, u8 *pattern)
    {
// pattern = 0;
    return 0;
    }
    const struct elan_transport_ops elan_smbus_ops = {
    .initialize		= elan_smbus_initialize,
    .sleep_control		= elan_smbus_sleep_control,
    .power_control		= elan_smbus_power_control,
    .set_mode		= elan_smbus_set_mode,
    .calibrate		= elan_smbus_calibrate,
    .calibrate_result	= elan_smbus_calibrate_result,
    .get_baseline_data	= elan_smbus_get_baseline_data,
    .get_version		= elan_smbus_get_version,
    .get_sm_version		= elan_smbus_get_sm_version,
    .get_product_id		= elan_smbus_get_product_id,
    .get_checksum		= elan_smbus_get_checksum,
    .get_pressure_adjustment = elan_smbus_get_pressure_adjustment,
    .get_max		= elan_smbus_get_max,
    .get_resolution		= elan_smbus_get_resolution,
    .get_num_traces		= elan_smbus_get_num_traces,
    .iap_get_mode		= elan_smbus_iap_get_mode,
    .iap_reset		= elan_smbus_iap_reset,
    .prepare_fw_update	= elan_smbus_prepare_fw_update,
    .write_fw_block		= elan_smbus_write_fw_block,
    .finish_fw_update	= elan_smbus_finish_fw_update,
    .get_report_features	= elan_smbus_get_report_features,
    .get_report		= elan_smbus_get_report,
    .get_pattern		= elan_smbus_get_pattern,
    };
