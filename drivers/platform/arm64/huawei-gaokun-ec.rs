//! Automatically rewritten from C to Rust
//! Source: drivers/platform/arm64/huawei-gaokun-ec.c
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
// huawei-gaokun-ec - An EC driver for HUAWEI Matebook E Go
//
// Copyright (C) 2024-2025 Pengyu Luo <mitltlatltl@gmail.com>
//

pub const EC_EVENT: c_uint = 0x06;
// Also can be found in ACPI specification 12.3
pub const EC_READ: c_uint = 0x80;
pub const EC_WRITE: c_uint = 0x81;
pub const EC_BURST: c_uint = 0x82;
pub const EC_QUERY: c_uint = 0x84;
pub const EC_FN_LOCK_ON: c_uint = 0x5A;
pub const EC_FN_LOCK_OFF: c_uint = 0x55;
pub const EC_FN_LOCK_READ: c_uint = 0x6B;
pub const EC_FN_LOCK_WRITE: c_uint = 0x6C;
pub const EC_EVENT_LID: c_uint = 0x81;
pub const EC_LID_STATE: c_uint = 0x80;

pub const EC_TEMP_REG: c_uint = 0x61;
pub const EC_STANDBY_REG: c_uint = 0xB2;
pub const EC_STANDBY_ENTER: c_uint = 0xDB;
pub const EC_STANDBY_EXIT: c_uint = 0xEB;
    enum gaokun_ec_smart_charge_cmd {
    SMART_CHARGE_DATA_WRITE = 0xE3,
    SMART_CHARGE_DATA_READ,
    SMART_CHARGE_ENABLE_WRITE,
    SMART_CHARGE_ENABLE_READ,
    };
    enum gaokun_ec_ucsi_cmd {
    UCSI_REG_WRITE = 0xD2,
    UCSI_REG_READ,
    UCSI_DATA_WRITE,
    UCSI_DATA_READ,
    };
pub const UCSI_REG_SIZE: c_int = 7;
//
// For tx, command sequences are arranged as
// {master_cmd, slave_cmd, data_len, data_seq}
//
pub const REQ_HDR_SIZE: c_int = 3;
pub const INPUT_SIZE_OFFSET: c_int = 2;

//
// For rx, data sequences are arranged as
// {status, data_len(unreliable), data_seq}
//
pub const RESP_HDR_SIZE: c_int = 2;

    {							\
    REG0, REG1, SIZE,				\
// ## will remove comma when SIZE is 0 */	\

// make sure len(pkt[3:]) >= SIZE */		\
    [3 + (SIZE)] = 0,				\
    }

    {						\
    [RESP_HDR_SIZE + (SIZE) - 1] = 0,	\
    }
// Possible size 1, 4, 20, 24. Most of the time, the size is 1.
#[no_mangle]
pub unsafe extern "C" fn refill_req(dest: *mut u8, src: *const u8, size: usize) {
    static inline void refill_req(u8 *dest, const u8 *src, size_t size)
    {
    memcpy(dest + REQ_HDR_SIZE, src, size);
    }
#[no_mangle]
pub unsafe extern "C" fn refill_req_byte(dest: *mut u8, src: *const u8) {
    static inline void refill_req_byte(u8 *dest, const u8 *src)
    {
    dest[REQ_HDR_SIZE] = *src;
    }
// Possible size 1, 2, 4, 7, 20. Most of the time, the size is 1.
#[no_mangle]
pub unsafe extern "C" fn extr_resp(dest: *mut u8, src: *const u8, size: usize) {
    static inline void extr_resp(u8 *dest, const u8 *src, size_t size)
    {
    memcpy(dest, src + RESP_HDR_SIZE, size);
    }
#[no_mangle]
pub unsafe extern "C" fn extr_resp_byte(dest: *mut u8, src: *const u8) {
    static inline void extr_resp_byte(u8 *dest, const u8 *src)
    {
// dest = src[RESP_HDR_SIZE];
    }
    static inline void *extr_resp_shallow(const u8 *src)
    {
    return (void *)(src + RESP_HDR_SIZE);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gaokun_ec {
    pub client: *mut i2c_client,
    pub /: *mut *mut mutex lock; / EC transaction lock,
    pub notifier_list: blocking_notifier_head,
    pub hwmon_dev: *mut device,
    pub idev: *mut input_dev,
    pub suspended: bool,
}

    static int gaokun_ec_request(struct gaokun_ec *ec, const u8 *req,
    size_t resp_len, u8 *resp)
    {
    struct i2c_client *client = ec.client;
    struct i2c_msg msgs[] = {
    {
    .addr = client.addr,
    .flags = client.flags,
    .len = REQ_LEN(req),
    .buf = (void *)req,
    }, {
    .addr = client.addr,
    .flags = client.flags | I2C_M_RD,
    .len = resp_len,
    .buf = resp,
    },
    };
    int ret;
    guard(mutex)(&ec.lock);
    ret = i2c_transfer(client.adapter, msgs, ARRAY_SIZE(msgs));
    if (ret != ARRAY_SIZE(msgs)) {
    dev_err(&client.dev, "I2C transfer error %d\n", ret);
    goto out_after_break;
    }
    ret = *resp;
    if (ret)
    dev_err(&client.dev, "EC transaction error %d\n", ret);
    out_after_break:
    usleep_range(2000, 2500); /* have a break, ACPI did this */
    return ret;
    }
// --------------------------------------------------------------------------
// Common API
//
// gaokun_ec_read - Read from EC
// @ec: The gaokun_ec structure
// @req: The sequence to request
// @resp_len: The size to read
// @resp: The buffer to store response sequence
//
// This function is used to read data after writing a magic sequence to EC.
// All EC operations depend on this function.
//
// Huawei uses magic sequences everywhere to complete various functions, all
// these sequences are passed to ECCD(a ACPI method which is quiet similar
// to gaokun_ec_request), there is no good abstraction to generalize these
// sequences, so just wrap it for now. Almost all magic sequences are kept
// in this file.
//
// Return: 0 on success or negative error code.
//
    int gaokun_ec_read(struct gaokun_ec *ec, const u8 *req,
    size_t resp_len, u8 *resp)
    {
    return gaokun_ec_request(ec, req, resp_len, resp);
    }
    EXPORT_SYMBOL_GPL(gaokun_ec_read);
//
// gaokun_ec_write - Write to EC
// @ec: The gaokun_ec structure
// @req: The sequence to request
//
// This function has no big difference from gaokun_ec_read. When caller care
// only write status and no actual data are returned, then use it.
//
// Return: 0 on success or negative error code.
//
#[no_mangle]
pub unsafe extern "C" fn gaokun_ec_write(ec: *mut gaokun_ec, req: *const u8) -> c_int {
    int gaokun_ec_write(struct gaokun_ec *ec, const u8 *req)
    {
    u8 ec_resp[] = MKRESP(0);
    return gaokun_ec_request(ec, req, sizeof(ec_resp), ec_resp);
    }
    EXPORT_SYMBOL_GPL(gaokun_ec_write);
#[no_mangle]
pub unsafe extern "C" fn gaokun_ec_read_byte(ec: *mut gaokun_ec, req: *const u8, byte: *mut u8) -> c_int {
    int gaokun_ec_read_byte(struct gaokun_ec *ec, const u8 *req, u8 *byte)
    {
    int ret;
    u8 ec_resp[] = MKRESP(sizeof(*byte));
    ret = gaokun_ec_read(ec, req, sizeof(ec_resp), ec_resp);
    extr_resp_byte(byte, ec_resp);
    return ret;
    }
    EXPORT_SYMBOL_GPL(gaokun_ec_read_byte);
//
// gaokun_ec_register_notify - Register a notifier callback for EC events.
// @ec: The gaokun_ec structure
// @nb: Notifier block pointer to register
//
// Return: 0 on success or negative error code.
//
#[no_mangle]
pub unsafe extern "C" fn gaokun_ec_register_notify(ec: *mut gaokun_ec, nb: *mut notifier_block) -> c_int {
    int gaokun_ec_register_notify(struct gaokun_ec *ec, struct notifier_block *nb)
    {
    return blocking_notifier_chain_register(&ec.notifier_list, nb);
    }
    EXPORT_SYMBOL_GPL(gaokun_ec_register_notify);
//
// gaokun_ec_unregister_notify - Unregister notifier callback for EC events.
// @ec: The gaokun_ec structure
// @nb: Notifier block pointer to unregister
//
// Unregister a notifier callback that was previously registered with
// gaokun_ec_register_notify().
//
#[no_mangle]
pub unsafe extern "C" fn gaokun_ec_unregister_notify(ec: *mut gaokun_ec, nb: *mut notifier_block) {
    void gaokun_ec_unregister_notify(struct gaokun_ec *ec, struct notifier_block *nb)
    {
    blocking_notifier_chain_unregister(&ec.notifier_list, nb);
    }
    EXPORT_SYMBOL_GPL(gaokun_ec_unregister_notify);
// --------------------------------------------------------------------------
// API for PSY
//
// gaokun_ec_psy_multi_read - Read contiguous registers
// @ec: The gaokun_ec structure
// @reg: The start register
// @resp_len: The number of registers to be read
// @resp: The buffer to store response sequence
//
// Return: 0 on success or negative error code.
//
    int gaokun_ec_psy_multi_read(struct gaokun_ec *ec, u8 reg,
    size_t resp_len, u8 *resp)
    {
    u8 ec_req[] = MKREQ(0x02, EC_READ, 1, 0);
    u8 ec_resp[] = MKRESP(1);
    int i, ret;
    for (i = 0; i < resp_len; ++i, reg++) {
    refill_req_byte(ec_req, &reg);
    ret = gaokun_ec_read(ec, ec_req, sizeof(ec_resp), ec_resp);
    if (ret)
    return ret;
    extr_resp_byte(&resp[i], ec_resp);
    }
    return 0;
    }
    EXPORT_SYMBOL_GPL(gaokun_ec_psy_multi_read);
// Smart charge
//
// gaokun_ec_psy_get_smart_charge - Get smart charge data from EC
// @ec: The gaokun_ec structure
// @resp: The buffer to store response sequence (mode, delay, start, end)
//
// Return: 0 on success or negative error code.
//
    int gaokun_ec_psy_get_smart_charge(struct gaokun_ec *ec,
    u8 resp[GAOKUN_SMART_CHARGE_DATA_SIZE])
    {
// GBCM
    u8 ec_req[] = MKREQ(0x02, SMART_CHARGE_DATA_READ, 0);
    u8 ec_resp[] = MKRESP(GAOKUN_SMART_CHARGE_DATA_SIZE);
    int ret;
    ret = gaokun_ec_read(ec, ec_req, sizeof(ec_resp), ec_resp);
    if (ret)
    return ret;
    extr_resp(resp, ec_resp, GAOKUN_SMART_CHARGE_DATA_SIZE);
    return 0;
    }
    EXPORT_SYMBOL_GPL(gaokun_ec_psy_get_smart_charge);
#[no_mangle]
pub unsafe extern "C" fn validate_battery_threshold_range(start: u8, end: u8) -> bool {
    static inline bool validate_battery_threshold_range(u8 start, u8 end)
    {
    return end != 0 && start <= end && end <= 100;
    }
//
// gaokun_ec_psy_set_smart_charge - Set smart charge data
// @ec: The gaokun_ec structure
// @req: The sequence to request (mode, delay, start, end)
//
// Return: 0 on success or negative error code.
//
    int gaokun_ec_psy_set_smart_charge(struct gaokun_ec *ec,
    const u8 req[GAOKUN_SMART_CHARGE_DATA_SIZE])
    {
// SBCM
    u8 ec_req[] = MKREQ(0x02, SMART_CHARGE_DATA_WRITE,
    GAOKUN_SMART_CHARGE_DATA_SIZE);
    if (!validate_battery_threshold_range(req[2], req[3]))
    return -EINVAL;
    refill_req(ec_req, req, GAOKUN_SMART_CHARGE_DATA_SIZE);
    return gaokun_ec_write(ec, ec_req);
    }
    EXPORT_SYMBOL_GPL(gaokun_ec_psy_set_smart_charge);
// Smart charge enable
//
// gaokun_ec_psy_get_smart_charge_enable - Get smart charge state
// @ec: The gaokun_ec structure
// @on: The state
//
// Return: 0 on success or negative error code.
//
#[no_mangle]
pub unsafe extern "C" fn gaokun_ec_psy_get_smart_charge_enable(ec: *mut gaokun_ec, on: *mut bool) -> c_int {
    int gaokun_ec_psy_get_smart_charge_enable(struct gaokun_ec *ec, bool *on)
    {
// GBAC
    u8 ec_req[] = MKREQ(0x02, SMART_CHARGE_ENABLE_READ, 0);
    u8 state;
    int ret;
    ret = gaokun_ec_read_byte(ec, ec_req, &state);
    if (ret)
    return ret;
// on = !!state;
    return 0;
    }
    EXPORT_SYMBOL_GPL(gaokun_ec_psy_get_smart_charge_enable);
//
// gaokun_ec_psy_set_smart_charge_enable - Set smart charge state
// @ec: The gaokun_ec structure
// @on: The state
//
// Return: 0 on success or negative error code.
//
#[no_mangle]
pub unsafe extern "C" fn gaokun_ec_psy_set_smart_charge_enable(ec: *mut gaokun_ec, on: bool) -> c_int {
    int gaokun_ec_psy_set_smart_charge_enable(struct gaokun_ec *ec, bool on)
    {
// SBAC
    u8 ec_req[] = MKREQ(0x02, SMART_CHARGE_ENABLE_WRITE, 1, on);
    return gaokun_ec_write(ec, ec_req);
    }
    EXPORT_SYMBOL_GPL(gaokun_ec_psy_set_smart_charge_enable);
// --------------------------------------------------------------------------
// API for UCSI
//
// gaokun_ec_ucsi_read - Read UCSI data from EC
// @ec: The gaokun_ec structure
// @resp: The buffer to store response sequence
//
// Read CCI and MSGI (used by UCSI subdriver).
//
// Return: 0 on success or negative error code.
//
    int gaokun_ec_ucsi_read(struct gaokun_ec *ec,
    u8 resp[GAOKUN_UCSI_READ_SIZE])
    {
    u8 ec_req[] = MKREQ(0x03, UCSI_DATA_READ, 0);
    u8 ec_resp[] = MKRESP(GAOKUN_UCSI_READ_SIZE);
    int ret;
    ret = gaokun_ec_read(ec, ec_req, sizeof(ec_resp), ec_resp);
    if (ret)
    return ret;
    extr_resp(resp, ec_resp, GAOKUN_UCSI_READ_SIZE);
    return 0;
    }
    EXPORT_SYMBOL_GPL(gaokun_ec_ucsi_read);
//
// gaokun_ec_ucsi_write - Write UCSI data to EC
// @ec: The gaokun_ec structure
// @req: The sequence to request
//
// Write CTRL and MSGO (used by UCSI subdriver).
//
// Return: 0 on success or negative error code.
//
    int gaokun_ec_ucsi_write(struct gaokun_ec *ec,
    const u8 req[GAOKUN_UCSI_WRITE_SIZE])
    {
    u8 ec_req[] = MKREQ(0x03, UCSI_DATA_WRITE, GAOKUN_UCSI_WRITE_SIZE);
    refill_req(ec_req, req, GAOKUN_UCSI_WRITE_SIZE);
    return gaokun_ec_write(ec, ec_req);
    }
    EXPORT_SYMBOL_GPL(gaokun_ec_ucsi_write);
//
// gaokun_ec_ucsi_get_reg - Get UCSI register from EC
// @ec: The gaokun_ec structure
// @ureg: The gaokun ucsi register
//
// Get UCSI register data (used by UCSI subdriver).
//
// Return: 0 on success or negative error code.
//
#[no_mangle]
pub unsafe extern "C" fn gaokun_ec_ucsi_get_reg(ec: *mut gaokun_ec, ureg: *mut gaokun_ucsi_reg) -> c_int {
    int gaokun_ec_ucsi_get_reg(struct gaokun_ec *ec, struct gaokun_ucsi_reg *ureg)
    {
    u8 ec_req[] = MKREQ(0x03, UCSI_REG_READ, 0);
    u8 ec_resp[] = MKRESP(UCSI_REG_SIZE);
    int ret;
    ret = gaokun_ec_read(ec, ec_req, sizeof(ec_resp), ec_resp);
    if (ret)
    return ret;
    extr_resp((u8 *)ureg, ec_resp, UCSI_REG_SIZE);
    return 0;
    }
    EXPORT_SYMBOL_GPL(gaokun_ec_ucsi_get_reg);
//
// gaokun_ec_ucsi_pan_ack - Ack pin assignment notifications from EC
// @ec: The gaokun_ec structure
// @port_id: The port id receiving and handling the notifications
//
// Ack pin assignment notifications (used by UCSI subdriver).
//
// Return: 0 on success or negative error code.
//
#[no_mangle]
pub unsafe extern "C" fn gaokun_ec_ucsi_pan_ack(ec: *mut gaokun_ec, port_id: c_int) -> c_int {
    int gaokun_ec_ucsi_pan_ack(struct gaokun_ec *ec, int port_id)
    {
    u8 ec_req[] = MKREQ(0x03, UCSI_REG_WRITE, 1);
    let mut data: u8 = 1 << port_id;
    if (port_id == GAOKUN_UCSI_NO_PORT_UPDATE)
    data = 0;
    refill_req_byte(ec_req, &data);
    return gaokun_ec_write(ec, ec_req);
    }
    EXPORT_SYMBOL_GPL(gaokun_ec_ucsi_pan_ack);
// --------------------------------------------------------------------------
// EC Sysfs
// Fn lock
#[no_mangle]
unsafe extern "C" fn gaokun_ec_get_fn_lock(ec: *mut gaokun_ec, on: *mut bool) -> c_int {
    static int gaokun_ec_get_fn_lock(struct gaokun_ec *ec, bool *on)
    {
// GFRS
    u8 ec_req[] = MKREQ(0x02, EC_FN_LOCK_READ, 0);
    int ret;
    u8 state;
    ret = gaokun_ec_read_byte(ec, ec_req, &state);
    if (ret)
    return ret;
    if (state == EC_FN_LOCK_ON)
// on = true;
#[no_mangle]
pub unsafe extern "C" fn if(EC_FN_LOCK_OFF: state ==) -> else {
    else if (state == EC_FN_LOCK_OFF)
// on = false;
    else
    return -EIO;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn gaokun_ec_set_fn_lock(ec: *mut gaokun_ec, on: bool) -> c_int {
    static int gaokun_ec_set_fn_lock(struct gaokun_ec *ec, bool on)
    {
// SFRS
    u8 ec_req[] = MKREQ(0x02, EC_FN_LOCK_WRITE, 1,
    on ? EC_FN_LOCK_ON : EC_FN_LOCK_OFF);
    return gaokun_ec_write(ec, ec_req);
    }
    static ssize_t fn_lock_show(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    struct gaokun_ec *ec = dev_get_drvdata(dev);
    bool on;
    int ret;
    ret = gaokun_ec_get_fn_lock(ec, &on);
    if (ret)
    return ret;
    return sysfs_emit(buf, "%d\n", on);
    }
    static ssize_t fn_lock_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t size)
    {
    struct gaokun_ec *ec = dev_get_drvdata(dev);
    bool on;
    int ret;
    if (kstrtobool(buf, &on))
    return -EINVAL;
    ret = gaokun_ec_set_fn_lock(ec, on);
    if (ret)
    return ret;
    return size;
    }
    static DEVICE_ATTR_RW(fn_lock);
    static struct attribute *gaokun_ec_attrs[] = {
    &dev_attr_fn_lock.attr,
    core::ptr::null_mut(),
    };
    ATTRIBUTE_GROUPS(gaokun_ec);
// --------------------------------------------------------------------------
// Thermal Zone HwMon
// Range from 0 to 0x2C, partially valid
    static const u8 temp_reg[] = {
    0x05, 0x07, 0x08, 0x0E, 0x0F, 0x12, 0x15, 0x1E,
    0x1F, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26,
    0x27, 0x28, 0x29, 0x2A
    };
#[no_mangle]
unsafe extern "C" fn gaokun_ec_get_temp(ec: *mut gaokun_ec, idx: u8, temp: *mut c_long) -> c_int {
    static int gaokun_ec_get_temp(struct gaokun_ec *ec, u8 idx, long *temp)
    {
// GTMP
    u8 ec_req[] = MKREQ(0x02, EC_TEMP_REG, 1, temp_reg[idx]);
    u8 ec_resp[] = MKRESP(sizeof(__le16));
    __le16 *tmp;
    int ret;
    ret = gaokun_ec_read(ec, ec_req, sizeof(ec_resp), ec_resp);
    if (ret)
    return ret;
    tmp = (__le16 *)extr_resp_shallow(ec_resp);
// temp = le16_to_cpu(*tmp) * 100; /* convert to HwMon's unit
    return 0;
    }
    static umode_t
    gaokun_ec_hwmon_is_visible(const void *data, enum hwmon_sensor_types type,
    u32 attr, int channel)
    {
    let mut type: return = = hwmon_temp ? 0444 : 0;
    }
    static int
    gaokun_ec_hwmon_read(struct device *dev, enum hwmon_sensor_types type,
    u32 attr, int channel, long *val)
    {
    struct gaokun_ec *ec = dev_get_drvdata(dev);
    if (type == hwmon_temp)
    return gaokun_ec_get_temp(ec, channel, val);
    return -EINVAL;
    }
    static const struct hwmon_ops gaokun_ec_hwmon_ops = {
    .is_visible = gaokun_ec_hwmon_is_visible,
    .read = gaokun_ec_hwmon_read,
    };
    static u32 gaokun_ec_temp_config[] = {
    [0 ... ARRAY_SIZE(temp_reg) - 1] = HWMON_T_INPUT,
    0
    };
    static const struct hwmon_channel_info gaokun_ec_temp = {
    .type = hwmon_temp,
    .config = gaokun_ec_temp_config,
    };
    static const struct hwmon_channel_info * const gaokun_ec_hwmon_info[] = {
    &gaokun_ec_temp,
    core::ptr::null_mut()
    };
    static const struct hwmon_chip_info gaokun_ec_hwmon_chip_info = {
    .ops = &gaokun_ec_hwmon_ops,
    .info = gaokun_ec_hwmon_info,
    };
// --------------------------------------------------------------------------
// Modern Standby
#[no_mangle]
unsafe extern "C" fn gaokun_ec_suspend(dev: *mut device) -> c_int {
    static int gaokun_ec_suspend(struct device *dev)
    {
    struct gaokun_ec *ec = dev_get_drvdata(dev);
    u8 ec_req[] = MKREQ(0x02, EC_STANDBY_REG, 1, EC_STANDBY_ENTER);
    int ret;
    if (ec.suspended)
    return 0;
    ret = gaokun_ec_write(ec, ec_req);
    if (ret)
    return ret;
    ec.suspended = true;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn gaokun_ec_resume(dev: *mut device) -> c_int {
    static int gaokun_ec_resume(struct device *dev)
    {
    struct gaokun_ec *ec = dev_get_drvdata(dev);
    u8 ec_req[] = MKREQ(0x02, EC_STANDBY_REG, 1, EC_STANDBY_EXIT);
    int ret;
    int i;
    if (!ec.suspended)
    return 0;
    for (i = 0; i < 3; ++i) {
    ret = gaokun_ec_write(ec, ec_req);
    if (ret == 0)
    break;
    msleep(100); /* EC need time to resume */
    }
    ec.suspended = false;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn gaokun_aux_release(dev: *mut device) {
    static void gaokun_aux_release(struct device *dev)
    {
    struct auxiliary_device *adev = to_auxiliary_dev(dev);
    of_node_put(dev.of_node);
    kfree(adev);
    }
#[no_mangle]
unsafe extern "C" fn gaokun_aux_remove(data: *mut c_void) {
    static void gaokun_aux_remove(void *data)
    {
    struct auxiliary_device *adev = data;
    auxiliary_device_delete(adev);
    auxiliary_device_uninit(adev);
    }
    static int gaokun_aux_init(struct device *parent, const char *name,
    struct gaokun_ec *ec)
    {
    struct auxiliary_device *adev;
    int ret;
    adev = kzalloc_obj(*adev);
    if (!adev)
    return -ENOMEM;
    adev.name = name;
    adev.id = 0;
    adev.dev.parent = parent;
    adev.dev.release = gaokun_aux_release;
    adev.dev.platform_data = ec;
// Allow aux devices to access parent's DT nodes directly
    device_set_of_node_from_dev(&adev.dev, parent);
    ret = auxiliary_device_init(adev);
    if (ret) {
    of_node_put(adev.dev.of_node);
    kfree(adev);
    return ret;
    }
    ret = auxiliary_device_add(adev);
    if (ret) {
    auxiliary_device_uninit(adev);
    return ret;
    }
    return devm_add_action_or_reset(parent, gaokun_aux_remove, adev);
    }
// --------------------------------------------------------------------------
// EC
#[no_mangle]
unsafe extern "C" fn gaokun_ec_irq_handler(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t gaokun_ec_irq_handler(int irq, void *data)
    {
    struct gaokun_ec *ec = data;
    u8 ec_req[] = MKREQ(EC_EVENT, EC_QUERY, 0);
    u8 status, id;
    int ret;
    ret = gaokun_ec_read_byte(ec, ec_req, &id);
    if (ret)
    return IRQ_HANDLED;
    switch (id) {
    case 0x0: /* No event */
    break;
    case EC_EVENT_LID:
    gaokun_ec_psy_read_byte(ec, EC_LID_STATE, &status);
    status &= EC_LID_OPEN;
    input_report_switch(ec.idev, SW_LID, !status);
    input_sync(ec.idev);
    break;
    default:
    blocking_notifier_call_chain(&ec.notifier_list, id, ec);
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn gaokun_ec_probe(client: *mut i2c_client) -> c_int {
    static int gaokun_ec_probe(struct i2c_client *client)
    {
    struct device *dev = &client.dev;
    struct gaokun_ec *ec;
    int ret;
    ec = devm_kzalloc(dev, sizeof(*ec), GFP_KERNEL);
    if (!ec)
    return -ENOMEM;
    ret = devm_mutex_init(dev, &ec.lock);
    if (ret)
    return ret;
    ec.client = client;
    i2c_set_clientdata(client, ec);
    BLOCKING_INIT_NOTIFIER_HEAD(&ec.notifier_list);
// Lid switch
    ec.idev = devm_input_allocate_device(dev);
    if (!ec.idev)
    return -ENOMEM;
    ec.idev.name = "LID";
    ec.idev.phys = "gaokun-ec/input0";
    input_set_capability(ec.idev, EV_SW, SW_LID);
    ret = input_register_device(ec.idev);
    if (ret)
    return dev_err_probe(dev, ret, "Failed to register input device\n");
    ret = gaokun_aux_init(dev, GAOKUN_DEV_PSY, ec);
    if (ret)
    return ret;
    ret = gaokun_aux_init(dev, GAOKUN_DEV_UCSI, ec);
    if (ret)
    return ret;
    ret = devm_request_threaded_irq(dev, client.irq, core::ptr::null_mut(),
    gaokun_ec_irq_handler, IRQF_ONESHOT,
    dev_name(dev), ec);
    if (ret)
    return dev_err_probe(dev, ret, "Failed to request IRQ\n");
    ec.hwmon_dev = devm_hwmon_device_register_with_info(dev, "gaokun_ec_hwmon",
    ec, &gaokun_ec_hwmon_chip_info, core::ptr::null_mut());
    if (IS_ERR(ec.hwmon_dev))
    return dev_err_probe(dev, PTR_ERR(ec.hwmon_dev),
    "Failed to register hwmon device\n");
    return 0;
    }
    static const struct i2c_device_id gaokun_ec_id[] = {
    { .name = "gaokun-ec" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, gaokun_ec_id);
    static const struct of_device_id gaokun_ec_of_match[] = {
    { .compatible = "huawei,gaokun3-ec", },
    { }
    };
    MODULE_DEVICE_TABLE(of, gaokun_ec_of_match);
    static const struct dev_pm_ops gaokun_ec_pm_ops = {
    NOIRQ_SYSTEM_SLEEP_PM_OPS(gaokun_ec_suspend, gaokun_ec_resume)
    };
    static struct i2c_driver gaokun_ec_driver = {
    .driver = {
    .name = "gaokun-ec",
    .of_match_table = gaokun_ec_of_match,
    .pm = &gaokun_ec_pm_ops,
    .dev_groups = gaokun_ec_groups,
    },
    .probe = gaokun_ec_probe,
    .id_table = gaokun_ec_id,
    };
    module_i2c_driver(gaokun_ec_driver);
    MODULE_DESCRIPTION("HUAWEI Matebook E Go EC driver");
    MODULE_AUTHOR("Pengyu Luo <mitltlatltl@gmail.com>");
    MODULE_LICENSE("GPL");
