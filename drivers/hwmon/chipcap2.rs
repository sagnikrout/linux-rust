//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/chipcap2.c
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
// cc2.c - Support for the Amphenol ChipCap 2 relative humidity, temperature sensor
//
// Part numbers supported:
// CC2D23, CC2D23S, CC2D25, CC2D25S, CC2D33, CC2D33S, CC2D35, CC2D35S
//
// Author: Javier Carrasco <javier.carrasco.cruz@gmail.com>
//
// Datasheet and application notes:
// https://www.amphenol-sensors.com/en/telaire/humidity/527-humidity-sensors/3095-chipcap-2
//

pub const CC2_START_CM: c_uint = 0xA0;
pub const CC2_START_NOM: c_uint = 0x80;
pub const CC2_R_ALARM_H_ON: c_uint = 0x18;
pub const CC2_R_ALARM_H_OFF: c_uint = 0x19;
pub const CC2_R_ALARM_L_ON: c_uint = 0x1A;
pub const CC2_R_ALARM_L_OFF: c_uint = 0x1B;
pub const CC2_RW_OFFSET: c_uint = 0x40;

pub const CC2_STATUS_VALID_DATA: c_uint = 0x00;
pub const CC2_STATUS_STALE_DATA: c_uint = 0x01;
pub const CC2_STATUS_CMD_MODE: c_uint = 0x02;

pub const CC2_RESPONSE_BUSY: c_uint = 0x00;
pub const CC2_RESPONSE_ACK: c_uint = 0x01;
pub const CC2_RESPONSE_NACK: c_uint = 0x02;

pub const CC2_EEPROM_SIZE: c_int = 10;
pub const CC2_EEPROM_DATA_LEN: c_int = 3;
pub const CC2_MEASUREMENT_DATA_LEN: c_int = 4;

// ensure clean off -> on transitions
pub const CC2_POWER_CYCLE_MS: c_int = 80;
pub const CC2_STARTUP_TO_DATA_MS: c_int = 55;
pub const CC2_RESP_START_CM_US: c_int = 100;
pub const CC2_RESP_EEPROM_R_US: c_int = 100;
pub const CC2_RESP_EEPROM_W_MS: c_int = 12;
pub const CC2_STARTUP_TIME_US: c_int = 1250;

pub const CC2_CM_RETRIES: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cc2_rh_alarm_info {
    pub low_alarm: bool,
    pub high_alarm: bool,
    pub low_alarm_visible: bool,
    pub high_alarm_visible: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cc2_data {
    pub rh_alarm: cc2_rh_alarm_info,
    pub complete: completion,
    pub hwmon: *mut device,
    pub client: *mut i2c_client,
    pub regulator: *mut regulator,
    pub name: *const c_char,
    pub label: *const c_char,
    pub irq_ready: c_int,
    pub irq_low: c_int,
    pub irq_high: c_int,
    pub process_irqs: bool,
}

    enum cc2_chan_addr {
    CC2_CHAN_TEMP = 0,
    CC2_CHAN_HUMIDITY,
    };
// %RH as a per cent mille from a register value
#[no_mangle]
unsafe extern "C" fn cc2_rh_convert(data: u16) -> c_long {
    static long cc2_rh_convert(u16 data)
    {
    let mut tmp: c_ulong = (data & CC2_RH_DATA_FIELD) * CC2_RH_MAX;
    return tmp / ((1 << 14) - 1);
    }
// convert %RH to a register value
#[no_mangle]
unsafe extern "C" fn cc2_rh_to_reg(data: c_long) -> u16 {
    static u16 cc2_rh_to_reg(long data)
    {
    return data * ((1 << 14) - 1) / CC2_RH_MAX;
    }
// temperature in milli degrees celsius from a register value
#[no_mangle]
unsafe extern "C" fn cc2_temp_convert(data: u16) -> c_long {
    static long cc2_temp_convert(u16 data)
    {
    let mut tmp: c_ulong = ((data >> 2) * 165 * 1000U) / ((1 << 14) - 1);
    return tmp - 40 * 1000U;
    }
#[no_mangle]
unsafe extern "C" fn cc2_enable(data: *mut cc2_data) -> c_int {
    static int cc2_enable(struct cc2_data *data)
    {
    int ret;
// exclusive regulator, check in case a disable failed
    if (regulator_is_enabled(data.regulator))
    return 0;
// clear any pending completion
    try_wait_for_completion(&data.complete);
    ret = regulator_enable(data.regulator);
    if (ret < 0)
    return ret;
    usleep_range(CC2_STARTUP_TIME_US, CC2_STARTUP_TIME_US + 125);
    data.process_irqs = true;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cc2_disable(data: *mut cc2_data) {
    static void cc2_disable(struct cc2_data *data)
    {
    int err;
// ignore alarms triggered by voltage toggling when powering up
    data.process_irqs = false;
// exclusive regulator, check in case an enable failed
    if (regulator_is_enabled(data.regulator)) {
    err = regulator_disable(data.regulator);
    if (err)
    dev_dbg(&data.client.dev, "Failed to disable device");
    }
    }
#[no_mangle]
unsafe extern "C" fn cc2_cmd_response_diagnostic(dev: *mut device, status: u8) -> c_int {
    static int cc2_cmd_response_diagnostic(struct device *dev, u8 status)
    {
    int resp;
    if (FIELD_GET(CC2_STATUS_FIELD, status) != CC2_STATUS_CMD_MODE) {
    dev_dbg(dev, "Command sent out of command window\n");
    return -ETIMEDOUT;
    }
    resp = FIELD_GET(CC2_RESPONSE_FIELD, status);
    switch (resp) {
    case CC2_RESPONSE_ACK:
    return 0;
    case CC2_RESPONSE_BUSY:
    return -EBUSY;
    case CC2_RESPONSE_NACK:
    if (resp & CC2_ERR_CORR_EEPROM)
    dev_dbg(dev, "Command failed: corrected EEPROM\n");
    if (resp & CC2_ERR_UNCORR_EEPROM)
    dev_dbg(dev, "Command failed: uncorrected EEPROM\n");
    if (resp & CC2_ERR_RAM_PARITY)
    dev_dbg(dev, "Command failed: RAM parity\n");
    if (resp & CC2_ERR_RAM_PARITY)
    dev_dbg(dev, "Command failed: configuration error\n");
    return -ENODATA;
    default:
    dev_dbg(dev, "Unknown command reply\n");
    return -EINVAL;
    }
    }
#[no_mangle]
unsafe extern "C" fn cc2_read_command_status(client: *mut i2c_client) -> c_int {
    static int cc2_read_command_status(struct i2c_client *client)
    {
    u8 status;
    int ret;
    ret = i2c_master_recv(client, &status, 1);
    if (ret != 1) {
    ret = ret < 0 ? ret : -EIO;
    return ret;
    }
    return cc2_cmd_response_diagnostic(&client.dev, status);
    }
//
// The command mode is only accessible after sending the START_CM command in the
// first 10 ms after power-up. Only in case the command window is missed,
// CC2_CM_RETRIES retries are attempted before giving up and returning an error.
//
#[no_mangle]
unsafe extern "C" fn cc2_command_mode_start(data: *mut cc2_data) -> c_int {
    static int cc2_command_mode_start(struct cc2_data *data)
    {
    unsigned long timeout;
    int i, ret;
    for (i = 0; i < CC2_CM_RETRIES; i++) {
    ret = cc2_enable(data);
    if (ret < 0)
    return ret;
    ret = i2c_smbus_write_word_data(data.client, CC2_START_CM, 0);
    if (ret < 0)
    return ret;
    if (data.irq_ready > 0) {
    timeout = usecs_to_jiffies(2 * CC2_RESP_START_CM_US);
    ret = wait_for_completion_timeout(&data.complete,
    timeout);
    if (!ret)
    return -ETIMEDOUT;
    } else {
    usleep_range(CC2_RESP_START_CM_US,
    2 * CC2_RESP_START_CM_US);
    }
    ret = cc2_read_command_status(data.client);
    if (ret != -ETIMEDOUT || i == CC2_CM_RETRIES)
    break;
// command window missed, prepare for a retry
    cc2_disable(data);
    msleep(CC2_POWER_CYCLE_MS);
    }
    return ret;
    }
// Sending a Start_NOM command finishes the command mode immediately with no
// reply and the device enters normal operation mode
//
#[no_mangle]
unsafe extern "C" fn cc2_command_mode_finish(data: *mut cc2_data) -> c_int {
    static int cc2_command_mode_finish(struct cc2_data *data)
    {
    int ret;
    ret = i2c_smbus_write_word_data(data.client, CC2_START_NOM, 0);
    if (ret < 0)
    return ret;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cc2_write_reg(data: *mut cc2_data, reg: u8, val: u16) -> c_int {
    static int cc2_write_reg(struct cc2_data *data, u8 reg, u16 val)
    {
    unsigned long timeout;
    int ret;
    ret = cc2_command_mode_start(data);
    if (ret < 0)
    goto disable;
    cpu_to_be16s(&val);
    ret = i2c_smbus_write_word_data(data.client, reg, val);
    if (ret < 0)
    goto disable;
    if (data.irq_ready > 0) {
    timeout = msecs_to_jiffies(2 * CC2_RESP_EEPROM_W_MS);
    ret = wait_for_completion_timeout(&data.complete, timeout);
    if (!ret) {
    ret = -ETIMEDOUT;
    goto disable;
    }
    } else {
    msleep(CC2_RESP_EEPROM_W_MS);
    }
    ret = cc2_read_command_status(data.client);
    disable:
    cc2_disable(data);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn cc2_read_reg(data: *mut cc2_data, reg: u8, val: *mut u16) -> c_int {
    static int cc2_read_reg(struct cc2_data *data, u8 reg, u16 *val)
    {
    u8 buf[CC2_EEPROM_DATA_LEN];
    unsigned long timeout;
    int ret;
    ret = cc2_command_mode_start(data);
    if (ret < 0)
    return ret;
    ret = i2c_smbus_write_word_data(data.client, reg, 0);
    if (ret < 0)
    return ret;
    if (data.irq_ready > 0) {
    timeout = usecs_to_jiffies(2 * CC2_RESP_EEPROM_R_US);
    ret = wait_for_completion_timeout(&data.complete, timeout);
    if (!ret)
    return -ETIMEDOUT;
    } else {
    usleep_range(CC2_RESP_EEPROM_R_US, CC2_RESP_EEPROM_R_US + 10);
    }
    ret = i2c_master_recv(data.client, buf, CC2_EEPROM_DATA_LEN);
    if (ret != CC2_EEPROM_DATA_LEN)
    return ret < 0 ? ret : -EIO;
// val = be16_to_cpup((__be16 *)&buf[1]);
    return cc2_read_command_status(data.client);
    }
#[no_mangle]
unsafe extern "C" fn cc2_get_reg_val(data: *mut cc2_data, reg: u8, val: *mut c_long) -> c_int {
    static int cc2_get_reg_val(struct cc2_data *data, u8 reg, long *val)
    {
    u16 reg_val;
    int ret;
    ret = cc2_read_reg(data, reg, &reg_val);
    if (!ret)
// val = cc2_rh_convert(reg_val);
    cc2_disable(data);
    return ret;
    }
    static int cc2_data_fetch(struct i2c_client *client,
    enum hwmon_sensor_types type, long *val)
    {
    u8 data[CC2_MEASUREMENT_DATA_LEN];
    u8 status;
    int ret;
    ret = i2c_master_recv(client, data, CC2_MEASUREMENT_DATA_LEN);
    if (ret != CC2_MEASUREMENT_DATA_LEN) {
    ret = ret < 0 ? ret : -EIO;
    return ret;
    }
    status = FIELD_GET(CC2_STATUS_FIELD, data[0]);
    if (status == CC2_STATUS_STALE_DATA)
    return -EBUSY;
    if (status != CC2_STATUS_VALID_DATA)
    return -EIO;
    switch (type) {
    case hwmon_humidity:
// val = cc2_rh_convert(be16_to_cpup((__be16 *)&data[0]));
    break;
    case hwmon_temp:
// val = cc2_temp_convert(be16_to_cpup((__be16 *)&data[2]));
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
    static int cc2_read_measurement(struct cc2_data *data,
    enum hwmon_sensor_types type, long *val)
    {
    unsigned long timeout;
    int ret;
    if (data.irq_ready > 0) {
    timeout = msecs_to_jiffies(CC2_STARTUP_TO_DATA_MS * 2);
    ret = wait_for_completion_timeout(&data.complete, timeout);
    if (!ret)
    return -ETIMEDOUT;
    } else {
    msleep(CC2_STARTUP_TO_DATA_MS);
    }
    ret = cc2_data_fetch(data.client, type, val);
    return ret;
    }
//
// A measurement requires enabling the device, waiting for the automatic
// measurement to finish, reading the measurement data and disabling the device
// again.
//
    static int cc2_measurement(struct cc2_data *data, enum hwmon_sensor_types type,
    long *val)
    {
    int ret;
    ret = cc2_enable(data);
    if (ret)
    return ret;
    ret = cc2_read_measurement(data, type, val);
    cc2_disable(data);
    return ret;
    }
//
// In order to check alarm status, the corresponding ALARM_OFF (hysteresis)
// register must be read and a new measurement must be carried out to trigger
// the alarm signals. Given that the device carries out a measurement after
// exiting the command mode, there is no need to force two power-up sequences.
// Instead, a NOM command is sent and the device is disabled after the
// measurement is read.
//
    static int cc2_read_hyst_and_measure(struct cc2_data *data, u8 reg,
    long *hyst, long *measurement)
    {
    u16 reg_val;
    int ret;
    ret = cc2_read_reg(data, reg, &reg_val);
    if (ret)
    goto disable;
// hyst = cc2_rh_convert(reg_val);
    ret = cc2_command_mode_finish(data);
    if (ret)
    goto disable;
    ret = cc2_read_measurement(data, hwmon_humidity, measurement);
    disable:
    cc2_disable(data);
    return ret;
    }
    static umode_t cc2_is_visible(const void *data, enum hwmon_sensor_types type,
    u32 attr, int channel)
    {
    const struct cc2_data *cc2 = data;
    switch (type) {
    case hwmon_humidity:
    switch (attr) {
    case hwmon_humidity_input:
    return 0444;
    case hwmon_humidity_label:
    return cc2.label ? 0444 : 0;
    case hwmon_humidity_min_alarm:
    return cc2.rh_alarm.low_alarm_visible ? 0444 : 0;
    case hwmon_humidity_max_alarm:
    return cc2.rh_alarm.high_alarm_visible ? 0444 : 0;
    case hwmon_humidity_min:
    case hwmon_humidity_min_hyst:
    return cc2.rh_alarm.low_alarm_visible ? 0644 : 0;
    case hwmon_humidity_max:
    case hwmon_humidity_max_hyst:
    return cc2.rh_alarm.high_alarm_visible ? 0644 : 0;
    default:
    return 0;
    }
    case hwmon_temp:
    switch (attr) {
    case hwmon_temp_input:
    return 0444;
    case hwmon_temp_label:
    return cc2.label ? 0444 : 0;
    default:
    return 0;
    }
    default:
    break;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cc2_ready_interrupt(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t cc2_ready_interrupt(int irq, void *data)
    {
    struct cc2_data *cc2 = data;
    if (cc2.process_irqs)
    complete(&cc2.complete);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn cc2_low_interrupt(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t cc2_low_interrupt(int irq, void *data)
    {
    struct cc2_data *cc2 = data;
    if (cc2.process_irqs) {
    hwmon_notify_event(cc2.hwmon, hwmon_humidity,
    hwmon_humidity_min_alarm, CC2_CHAN_HUMIDITY);
    cc2.rh_alarm.low_alarm = true;
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn cc2_high_interrupt(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t cc2_high_interrupt(int irq, void *data)
    {
    struct cc2_data *cc2 = data;
    if (cc2.process_irqs) {
    hwmon_notify_event(cc2.hwmon, hwmon_humidity,
    hwmon_humidity_max_alarm, CC2_CHAN_HUMIDITY);
    cc2.rh_alarm.high_alarm = true;
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn cc2_humidity_min_alarm_status(data: *mut cc2_data, val: *mut c_long) -> c_int {
    static int cc2_humidity_min_alarm_status(struct cc2_data *data, long *val)
    {
    long measurement, min_hyst;
    int ret;
    ret = cc2_read_hyst_and_measure(data, CC2_R_ALARM_L_OFF, &min_hyst,
    &measurement);
    if (ret < 0)
    return ret;
    if (data.rh_alarm.low_alarm) {
// val = (measurement < min_hyst) ? 1 : 0;
    data.rh_alarm.low_alarm = *val;
    } else {
// val = 0;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cc2_humidity_max_alarm_status(data: *mut cc2_data, val: *mut c_long) -> c_int {
    static int cc2_humidity_max_alarm_status(struct cc2_data *data, long *val)
    {
    long measurement, max_hyst;
    int ret;
    ret = cc2_read_hyst_and_measure(data, CC2_R_ALARM_H_OFF, &max_hyst,
    &measurement);
    if (ret < 0)
    return ret;
    if (data.rh_alarm.high_alarm) {
// val = (measurement > max_hyst) ? 1 : 0;
    data.rh_alarm.high_alarm = *val;
    } else {
// val = 0;
    }
    return 0;
    }
    static int cc2_read_string(struct device *dev, enum hwmon_sensor_types type,
    u32 attr, int channel, const char **str)
    {
    struct cc2_data *data = dev_get_drvdata(dev);
// str = data->label;
    return 0;
    }
    static int cc2_read(struct device *dev, enum hwmon_sensor_types type, u32 attr,
    int channel, long *val)
    {
    struct cc2_data *data = dev_get_drvdata(dev);
    switch (type) {
    case hwmon_temp:
    return cc2_measurement(data, type, val);
    case hwmon_humidity:
    switch (attr) {
    case hwmon_humidity_input:
    return cc2_measurement(data, type, val);
    case hwmon_humidity_min:
    return cc2_get_reg_val(data, CC2_R_ALARM_L_ON, val);
    case hwmon_humidity_min_hyst:
    return cc2_get_reg_val(data, CC2_R_ALARM_L_OFF, val);
    case hwmon_humidity_max:
    return cc2_get_reg_val(data, CC2_R_ALARM_H_ON, val);
    case hwmon_humidity_max_hyst:
    return cc2_get_reg_val(data, CC2_R_ALARM_H_OFF, val);
    case hwmon_humidity_min_alarm:
    return cc2_humidity_min_alarm_status(data, val);
    case hwmon_humidity_max_alarm:
    return cc2_humidity_max_alarm_status(data, val);
    default:
    return -EOPNOTSUPP;
    }
    default:
    return -EOPNOTSUPP;
    }
    }
    static int cc2_write(struct device *dev, enum hwmon_sensor_types type, u32 attr,
    int channel, long val)
    {
    struct cc2_data *data = dev_get_drvdata(dev);
    u16 arg;
    u8 cmd;
    if (type != hwmon_humidity)
    return -EOPNOTSUPP;
    if (val < 0 || val > CC2_RH_MAX)
    return -EINVAL;
    switch (attr) {
    case hwmon_humidity_min:
    cmd = CC2_W_ALARM_L_ON;
    arg = cc2_rh_to_reg(val);
    return cc2_write_reg(data, cmd, arg);
    case hwmon_humidity_min_hyst:
    cmd = CC2_W_ALARM_L_OFF;
    arg = cc2_rh_to_reg(val);
    return cc2_write_reg(data, cmd, arg);
    case hwmon_humidity_max:
    cmd = CC2_W_ALARM_H_ON;
    arg = cc2_rh_to_reg(val);
    return cc2_write_reg(data, cmd, arg);
    case hwmon_humidity_max_hyst:
    cmd = CC2_W_ALARM_H_OFF;
    arg = cc2_rh_to_reg(val);
    return cc2_write_reg(data, cmd, arg);
    default:
    return -EOPNOTSUPP;
    }
    }
#[no_mangle]
unsafe extern "C" fn cc2_request_ready_irq(data: *mut cc2_data, dev: *mut device) -> c_int {
    static int cc2_request_ready_irq(struct cc2_data *data, struct device *dev)
    {
    let mut ret: c_int = 0;
    data.irq_ready = fwnode_irq_get_byname(dev_fwnode(dev), "ready");
    if (data.irq_ready > 0) {
    init_completion(&data.complete);
    ret = devm_request_threaded_irq(dev, data.irq_ready, core::ptr::null_mut(),
    cc2_ready_interrupt,
    IRQF_ONESHOT |
    IRQF_TRIGGER_RISING,
    dev_name(dev), data);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn cc2_request_alarm_irqs(data: *mut cc2_data, dev: *mut device) -> c_int {
    static int cc2_request_alarm_irqs(struct cc2_data *data, struct device *dev)
    {
    let mut ret: c_int = 0;
    data.irq_low = fwnode_irq_get_byname(dev_fwnode(dev), "low");
    if (data.irq_low > 0) {
    ret = devm_request_threaded_irq(dev, data.irq_low, core::ptr::null_mut(),
    cc2_low_interrupt,
    IRQF_ONESHOT |
    IRQF_TRIGGER_RISING,
    dev_name(dev), data);
    if (ret)
    return ret;
    data.rh_alarm.low_alarm_visible = true;
    }
    data.irq_high = fwnode_irq_get_byname(dev_fwnode(dev), "high");
    if (data.irq_high > 0) {
    ret = devm_request_threaded_irq(dev, data.irq_high, core::ptr::null_mut(),
    cc2_high_interrupt,
    IRQF_ONESHOT |
    IRQF_TRIGGER_RISING,
    dev_name(dev), data);
    if (ret)
    return ret;
    data.rh_alarm.high_alarm_visible = true;
    }
    return ret;
    }
    static const struct hwmon_channel_info *cc2_info[] = {
    HWMON_CHANNEL_INFO(temp, HWMON_T_INPUT | HWMON_T_LABEL),
    HWMON_CHANNEL_INFO(humidity, HWMON_H_INPUT | HWMON_H_LABEL |
    HWMON_H_MIN | HWMON_H_MAX |
    HWMON_H_MIN_HYST | HWMON_H_MAX_HYST |
    HWMON_H_MIN_ALARM | HWMON_H_MAX_ALARM),
    core::ptr::null_mut()
    };
    static const struct hwmon_ops cc2_hwmon_ops = {
    .is_visible = cc2_is_visible,
    .read = cc2_read,
    .read_string = cc2_read_string,
    .write = cc2_write,
    };
    static const struct hwmon_chip_info cc2_chip_info = {
    .ops = &cc2_hwmon_ops,
    .info = cc2_info,
    };
#[no_mangle]
unsafe extern "C" fn cc2_probe(client: *mut i2c_client) -> c_int {
    static int cc2_probe(struct i2c_client *client)
    {
    struct cc2_data *data;
    struct device *dev = &client.dev;
    int ret;
    if (!i2c_check_functionality(client.adapter, I2C_FUNC_I2C))
    return -EOPNOTSUPP;
    data = devm_kzalloc(dev, sizeof(*data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    i2c_set_clientdata(client, data);
    data.client = client;
    data.regulator = devm_regulator_get_exclusive(dev, "vdd");
    if (IS_ERR(data.regulator))
    return dev_err_probe(dev, PTR_ERR(data.regulator),
    "Failed to get regulator\n");
    device_property_read_string(dev, "label", &data.label);
    ret = cc2_request_ready_irq(data, dev);
    if (ret)
    return dev_err_probe(dev, ret, "Failed to request ready irq\n");
    ret = cc2_request_alarm_irqs(data, dev);
    if (ret)
    return dev_err_probe(dev, ret, "Failed to request alarm irqs\n");
    data.hwmon = devm_hwmon_device_register_with_info(dev, client.name,
    data, &cc2_chip_info,
    core::ptr::null_mut());
    if (IS_ERR(data.hwmon))
    return dev_err_probe(dev, PTR_ERR(data.hwmon),
    "Failed to register hwmon device\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cc2_remove(client: *mut i2c_client) {
    static void cc2_remove(struct i2c_client *client)
    {
    struct cc2_data *data = i2c_get_clientdata(client);
    cc2_disable(data);
    }
    static const struct i2c_device_id cc2_id[] = {
    { .name = "cc2d23" },
    { .name = "cc2d23s" },
    { .name = "cc2d25" },
    { .name = "cc2d25s" },
    { .name = "cc2d33" },
    { .name = "cc2d33s" },
    { .name = "cc2d35" },
    { .name = "cc2d35s" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, cc2_id);
    static const struct of_device_id cc2_of_match[] = {
    { .compatible = "amphenol,cc2d23" },
    { .compatible = "amphenol,cc2d23s" },
    { .compatible = "amphenol,cc2d25" },
    { .compatible = "amphenol,cc2d25s" },
    { .compatible = "amphenol,cc2d33" },
    { .compatible = "amphenol,cc2d33s" },
    { .compatible = "amphenol,cc2d35" },
    { .compatible = "amphenol,cc2d35s" },
    { },
    };
    MODULE_DEVICE_TABLE(of, cc2_of_match);
    static struct i2c_driver cc2_driver = {
    .driver = {
    .name	= "cc2d23",
    .of_match_table = cc2_of_match,
    },
    .probe		= cc2_probe,
    .remove		= cc2_remove,
    .id_table = cc2_id,
    };
    module_i2c_driver(cc2_driver);
    MODULE_AUTHOR("Javier Carrasco <javier.carrasco.cruz@gamil.com>");
    MODULE_DESCRIPTION("Amphenol ChipCap 2 humidity and temperature sensor driver");
    MODULE_LICENSE("GPL");
