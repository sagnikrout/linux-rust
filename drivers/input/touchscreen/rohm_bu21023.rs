//! Automatically rewritten from C to Rust
//! Source: drivers/input/touchscreen/rohm_bu21023.c
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
// ROHM BU21023/24 Dual touch support resistive touch screen driver
// Copyright (C) 2012 ROHM CO.,LTD.
//

pub const MAX_CONTACTS: c_int = 2;
pub const AXIS_ADJUST: c_int = 4;
pub const AXIS_OFFSET: c_int = 8;

pub const FIRMWARE_RETRY_MAX: c_int = 4;

pub const CALIBRATION_RETRY_MAX: c_int = 6;
pub const ROHM_TS_ABS_X_MIN: c_int = 40;
pub const ROHM_TS_ABS_X_MAX: c_int = 990;
pub const ROHM_TS_ABS_Y_MIN: c_int = 160;
pub const ROHM_TS_ABS_Y_MAX: c_int = 920;

//
// BU21023GUL/BU21023MUV/BU21024FV-M registers map
//
pub const VADOUT_YP_H: c_uint = 0x00;
pub const VADOUT_YP_L: c_uint = 0x01;
pub const VADOUT_XP_H: c_uint = 0x02;
pub const VADOUT_XP_L: c_uint = 0x03;
pub const VADOUT_YN_H: c_uint = 0x04;
pub const VADOUT_YN_L: c_uint = 0x05;
pub const VADOUT_XN_H: c_uint = 0x06;
pub const VADOUT_XN_L: c_uint = 0x07;
pub const PRM1_X_H: c_uint = 0x08;
pub const PRM1_X_L: c_uint = 0x09;
pub const PRM1_Y_H: c_uint = 0x0a;
pub const PRM1_Y_L: c_uint = 0x0b;
pub const PRM2_X_H: c_uint = 0x0c;
pub const PRM2_X_L: c_uint = 0x0d;
pub const PRM2_Y_H: c_uint = 0x0e;
pub const PRM2_Y_L: c_uint = 0x0f;
pub const MLT_PRM_MONI_X: c_uint = 0x10;
pub const MLT_PRM_MONI_Y: c_uint = 0x11;
pub const DEBUG_MONI_1: c_uint = 0x12;
pub const DEBUG_MONI_2: c_uint = 0x13;
pub const VADOUT_ZX_H: c_uint = 0x14;
pub const VADOUT_ZX_L: c_uint = 0x15;
pub const VADOUT_ZY_H: c_uint = 0x16;
pub const VADOUT_ZY_L: c_uint = 0x17;
pub const Z_PARAM_H: c_uint = 0x18;
pub const Z_PARAM_L: c_uint = 0x19;
//
// Value for VADOUT_*_L
//
pub const VADOUT_L_MASK: c_uint = 0x01;
//
// Value for PRM*_*_L
//
pub const PRM_L_MASK: c_uint = 0x01;
pub const POS_X1_H: c_uint = 0x20;
pub const POS_X1_L: c_uint = 0x21;
pub const POS_Y1_H: c_uint = 0x22;
pub const POS_Y1_L: c_uint = 0x23;
pub const POS_X2_H: c_uint = 0x24;
pub const POS_X2_L: c_uint = 0x25;
pub const POS_Y2_H: c_uint = 0x26;
pub const POS_Y2_L: c_uint = 0x27;
//
// Value for POS_*_L
//
pub const POS_L_MASK: c_uint = 0x01;
pub const TOUCH: c_uint = 0x28;
pub const TOUCH_DETECT: c_uint = 0x01;
pub const TOUCH_GESTURE: c_uint = 0x29;
pub const SINGLE_TOUCH: c_uint = 0x01;
pub const DUAL_TOUCH: c_uint = 0x03;
pub const TOUCH_MASK: c_uint = 0x03;
pub const CALIBRATION_REQUEST: c_uint = 0x04;
pub const CALIBRATION_STATUS: c_uint = 0x08;
pub const CALIBRATION_MASK: c_uint = 0x0c;
pub const GESTURE_SPREAD: c_uint = 0x10;
pub const GESTURE_PINCH: c_uint = 0x20;
pub const GESTURE_ROTATE_R: c_uint = 0x40;
pub const GESTURE_ROTATE_L: c_uint = 0x80;
pub const INT_STATUS: c_uint = 0x2a;
pub const INT_MASK: c_uint = 0x3d;
pub const INT_CLEAR: c_uint = 0x3e;
//
// Values for INT_
//
pub const COORD_UPDATE: c_uint = 0x01;
pub const CALIBRATION_DONE: c_uint = 0x02;
pub const SLEEP_IN: c_uint = 0x04;
pub const SLEEP_OUT: c_uint = 0x08;
pub const PROGRAM_LOAD_DONE: c_uint = 0x10;
pub const ERROR: c_uint = 0x80;
pub const INT_ALL: c_uint = 0x9f;
pub const ERR_STATUS: c_uint = 0x2b;
pub const ERR_MASK: c_uint = 0x3f;
//
// Values for ERR_
//
pub const ADC_TIMEOUT: c_uint = 0x01;
pub const CPU_TIMEOUT: c_uint = 0x02;
pub const CALIBRATION_ERR: c_uint = 0x04;
pub const PROGRAM_LOAD_ERR: c_uint = 0x10;
pub const COMMON_SETUP1: c_uint = 0x30;
pub const PROGRAM_LOAD_HOST: c_uint = 0x02;
pub const PROGRAM_LOAD_EEPROM: c_uint = 0x03;
pub const CENSOR_4PORT: c_uint = 0x04;
pub const CENSOR_8PORT: c_uint = 0x00	/* Not supported by BU21023 */;
pub const CALIBRATION_TYPE_DEFAULT: c_uint = 0x08;
pub const CALIBRATION_TYPE_SPECIAL: c_uint = 0x00;
pub const INT_ACTIVE_HIGH: c_uint = 0x10;
pub const INT_ACTIVE_LOW: c_uint = 0x00;
pub const AUTO_CALIBRATION: c_uint = 0x40;
pub const MANUAL_CALIBRATION: c_uint = 0x00;
pub const COMMON_SETUP1_DEFAULT: c_uint = 0x4e;
pub const COMMON_SETUP2: c_uint = 0x31;
pub const MAF_NONE: c_uint = 0x00;
pub const MAF_1SAMPLE: c_uint = 0x01;
pub const MAF_3SAMPLES: c_uint = 0x02;
pub const MAF_5SAMPLES: c_uint = 0x03;
pub const INV_Y: c_uint = 0x04;
pub const INV_X: c_uint = 0x08;
pub const SWAP_XY: c_uint = 0x10;
pub const COMMON_SETUP3: c_uint = 0x32;
pub const EN_SLEEP: c_uint = 0x01;
pub const EN_MULTI: c_uint = 0x02;
pub const EN_GESTURE: c_uint = 0x04;
pub const EN_INTVL: c_uint = 0x08;
pub const SEL_STEP: c_uint = 0x10;
pub const SEL_MULTI: c_uint = 0x20;
pub const SEL_TBL_DEFAULT: c_uint = 0x40;
pub const INTERVAL_TIME: c_uint = 0x33;
pub const INTERVAL_TIME_DEFAULT: c_uint = 0x10;
pub const STEP_X: c_uint = 0x34;
pub const STEP_X_DEFAULT: c_uint = 0x41;
pub const STEP_Y: c_uint = 0x35;
pub const STEP_Y_DEFAULT: c_uint = 0x8d;
pub const OFFSET_X: c_uint = 0x38;
pub const OFFSET_X_DEFAULT: c_uint = 0x0c;
pub const OFFSET_Y: c_uint = 0x39;
pub const OFFSET_Y_DEFAULT: c_uint = 0x0c;
pub const THRESHOLD_TOUCH: c_uint = 0x3a;
pub const THRESHOLD_TOUCH_DEFAULT: c_uint = 0xa0;
pub const THRESHOLD_GESTURE: c_uint = 0x3b;
pub const THRESHOLD_GESTURE_DEFAULT: c_uint = 0x17;
pub const SYSTEM: c_uint = 0x40;
pub const ANALOG_POWER_ON: c_uint = 0x01;
pub const ANALOG_POWER_OFF: c_uint = 0x00;
pub const CPU_POWER_ON: c_uint = 0x02;
pub const CPU_POWER_OFF: c_uint = 0x00;
pub const FORCE_CALIBRATION: c_uint = 0x42;
pub const FORCE_CALIBRATION_ON: c_uint = 0x01;
pub const FORCE_CALIBRATION_OFF: c_uint = 0x00;
pub const CPU_FREQ: c_uint = 0x50	/* 10 / (reg + 1) MHz */;
pub const CPU_FREQ_10MHZ: c_uint = 0x00;
pub const CPU_FREQ_5MHZ: c_uint = 0x01;
pub const CPU_FREQ_1MHZ: c_uint = 0x09;
pub const EEPROM_ADDR: c_uint = 0x51;
pub const CALIBRATION_ADJUST: c_uint = 0x52;
pub const CALIBRATION_ADJUST_DEFAULT: c_uint = 0x00;
pub const THRESHOLD_SLEEP_IN: c_uint = 0x53;
pub const EVR_XY: c_uint = 0x56;
pub const EVR_XY_DEFAULT: c_uint = 0x10;
pub const PRM_SWOFF_TIME: c_uint = 0x57;
pub const PRM_SWOFF_TIME_DEFAULT: c_uint = 0x04;
pub const PROGRAM_VERSION: c_uint = 0x5f;
pub const ADC_CTRL: c_uint = 0x60;
pub const ADC_DIV_MASK: c_uint = 0x1f	/* The minimum value is 4 */;
pub const ADC_DIV_DEFAULT: c_uint = 0x08;
pub const ADC_WAIT: c_uint = 0x61;
pub const ADC_WAIT_DEFAULT: c_uint = 0x0a;
pub const SWCONT: c_uint = 0x62;
pub const SWCONT_DEFAULT: c_uint = 0x0f;
pub const EVR_X: c_uint = 0x63;
pub const EVR_X_DEFAULT: c_uint = 0x86;
pub const EVR_Y: c_uint = 0x64;
pub const EVR_Y_DEFAULT: c_uint = 0x64;
pub const TEST1: c_uint = 0x65;
pub const DUALTOUCH_STABILIZE_ON: c_uint = 0x01;
pub const DUALTOUCH_STABILIZE_OFF: c_uint = 0x00;
pub const DUALTOUCH_REG_ON: c_uint = 0x20;
pub const DUALTOUCH_REG_OFF: c_uint = 0x00;
pub const CALIBRATION_REG1: c_uint = 0x68;
pub const CALIBRATION_REG1_DEFAULT: c_uint = 0xd9;
pub const CALIBRATION_REG2: c_uint = 0x69;
pub const CALIBRATION_REG2_DEFAULT: c_uint = 0x36;
pub const CALIBRATION_REG3: c_uint = 0x6a;
pub const CALIBRATION_REG3_DEFAULT: c_uint = 0x32;
pub const EX_ADDR_H: c_uint = 0x70;
pub const EX_ADDR_L: c_uint = 0x71;
pub const EX_WDAT: c_uint = 0x72;
pub const EX_RDAT: c_uint = 0x73;
pub const EX_CHK_SUM1: c_uint = 0x74;
pub const EX_CHK_SUM2: c_uint = 0x75;
pub const EX_CHK_SUM3: c_uint = 0x76;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rohm_ts_data {
    pub client: *mut i2c_client,
    pub input: *mut input_dev,
    pub initialized: bool,
    pub 1]: unsigned int contact_count[MAX_CONTACTS +,
    pub finger_count: c_int,
    pub setup2: u8,
}

//
// rohm_i2c_burst_read - execute combined I2C message for ROHM BU21023/24
// @client: Handle to ROHM BU21023/24
// @start: Where to start read address from ROHM BU21023/24
// @buf: Where to store read data from ROHM BU21023/24
// @len: How many bytes to read
//
// Returns negative errno, else zero on success.
//
// Note
// In BU21023/24 burst read, stop condition is needed after "address write".
// Therefore, transmission is performed in 2 steps.
//
    static int rohm_i2c_burst_read(struct i2c_client *client, u8 start, void *buf,
    size_t len)
    {
    struct i2c_adapter *adap = client.adapter;
    struct i2c_msg msg[2];
    int i, ret = 0;
    msg[0].addr = client.addr;
    msg[0].flags = 0;
    msg[0].len = 1;
    msg[0].buf = &start;
    msg[1].addr = client.addr;
    msg[1].flags = I2C_M_RD;
    msg[1].len = len;
    msg[1].buf = buf;
    i2c_lock_bus(adap, I2C_LOCK_SEGMENT);
    for (i = 0; i < 2; i++) {
    if (__i2c_transfer(adap, &msg[i], 1) < 0) {
    ret = -EIO;
    break;
    }
    }
    i2c_unlock_bus(adap, I2C_LOCK_SEGMENT);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn rohm_ts_manual_calibration(ts: *mut rohm_ts_data) -> c_int {
    static int rohm_ts_manual_calibration(struct rohm_ts_data *ts)
    {
    struct i2c_client *client = ts.client;
    struct device *dev = &client.dev;
    u8 buf[33];	/* for PRM1_X_H(0x08)-TOUCH(0x28) */
    int retry;
    let mut success: bool = false;
    let mut first_time: bool = true;
    bool calibration_done;
    u8 reg1, reg2, reg3;
    s32 reg1_orig, reg2_orig, reg3_orig;
    s32 val;
    let mut calib_x: c_int = 0, calib_y = 0;
    int reg_x, reg_y;
    int err_x, err_y;
    int error, error2;
    int i;
    reg1_orig = i2c_smbus_read_byte_data(client, CALIBRATION_REG1);
    if (reg1_orig < 0)
    return reg1_orig;
    reg2_orig = i2c_smbus_read_byte_data(client, CALIBRATION_REG2);
    if (reg2_orig < 0)
    return reg2_orig;
    reg3_orig = i2c_smbus_read_byte_data(client, CALIBRATION_REG3);
    if (reg3_orig < 0)
    return reg3_orig;
    error = i2c_smbus_write_byte_data(client, INT_MASK,
    COORD_UPDATE | SLEEP_IN | SLEEP_OUT |
    PROGRAM_LOAD_DONE);
    if (error)
    goto out;
    error = i2c_smbus_write_byte_data(client, TEST1,
    DUALTOUCH_STABILIZE_ON);
    if (error)
    goto out;
    for (retry = 0; retry < CALIBRATION_RETRY_MAX; retry++) {
// wait 2 sampling for update
    mdelay(2 * SAMPLING_DELAY);

    error = rohm_i2c_burst_read(client, PRM1_X_H, buf, sizeof(buf));
    if (error)
    goto out;
    if (READ_CALIB_BUF(TOUCH) & TOUCH_DETECT)
    continue;
    if (first_time) {
// generate calibration parameter
    calib_x = ((int)READ_CALIB_BUF(PRM1_X_H) << 2 |
    READ_CALIB_BUF(PRM1_X_L)) - AXIS_OFFSET;
    calib_y = ((int)READ_CALIB_BUF(PRM1_Y_H) << 2 |
    READ_CALIB_BUF(PRM1_Y_L)) - AXIS_OFFSET;
    error = i2c_smbus_write_byte_data(client, TEST1,
    DUALTOUCH_STABILIZE_ON | DUALTOUCH_REG_ON);
    if (error)
    goto out;
    first_time = false;
    } else {
// generate adjustment parameter
    err_x = (int)READ_CALIB_BUF(PRM1_X_H) << 2 |
    READ_CALIB_BUF(PRM1_X_L);
    err_y = (int)READ_CALIB_BUF(PRM1_Y_H) << 2 |
    READ_CALIB_BUF(PRM1_Y_L);
// X axis adjust
    if (err_x <= 4)
    calib_x -= AXIS_ADJUST;
#[no_mangle]
pub unsafe extern "C" fn if(60: err_x >=) -> else {
    else if (err_x >= 60)
    calib_x += AXIS_ADJUST;
// Y axis adjust
    if (err_y <= 4)
    calib_y -= AXIS_ADJUST;
#[no_mangle]
pub unsafe extern "C" fn if(60: err_y >=) -> else {
    else if (err_y >= 60)
    calib_y += AXIS_ADJUST;
    }
// generate calibration setting value
    reg_x = calib_x + ((calib_x & 0x200) << 1);
    reg_y = calib_y + ((calib_y & 0x200) << 1);
// convert for register format
    reg1 = reg_x >> 3;
    reg2 = (reg_y & 0x7) << 4 | (reg_x & 0x7);
    reg3 = reg_y >> 3;
    error = i2c_smbus_write_byte_data(client,
    CALIBRATION_REG1, reg1);
    if (error)
    goto out;
    error = i2c_smbus_write_byte_data(client,
    CALIBRATION_REG2, reg2);
    if (error)
    goto out;
    error = i2c_smbus_write_byte_data(client,
    CALIBRATION_REG3, reg3);
    if (error)
    goto out;
//
// force calibration sequcence
//
    error = i2c_smbus_write_byte_data(client, FORCE_CALIBRATION,
    FORCE_CALIBRATION_OFF);
    if (error)
    goto out;
    error = i2c_smbus_write_byte_data(client, FORCE_CALIBRATION,
    FORCE_CALIBRATION_ON);
    if (error)
    goto out;
// clear all interrupts
    error = i2c_smbus_write_byte_data(client, INT_CLEAR, 0xff);
    if (error)
    goto out;
//
// Wait for the status change of calibration, max 10 sampling
//
    calibration_done = false;
    for (i = 0; i < 10; i++) {
    mdelay(SAMPLING_DELAY);
    val = i2c_smbus_read_byte_data(client, TOUCH_GESTURE);
    if (!(val & CALIBRATION_MASK)) {
    calibration_done = true;
    break;
    } else if (val < 0) {
    error = val;
    goto out;
    }
    }
    if (calibration_done) {
    val = i2c_smbus_read_byte_data(client, INT_STATUS);
    if (val == CALIBRATION_DONE) {
    success = true;
    break;
    } else if (val < 0) {
    error = val;
    goto out;
    }
    } else {
    dev_warn(dev, "calibration timeout\n");
    }
    }
    if (!success) {
    error = i2c_smbus_write_byte_data(client, CALIBRATION_REG1,
    reg1_orig);
    if (error)
    goto out;
    error = i2c_smbus_write_byte_data(client, CALIBRATION_REG2,
    reg2_orig);
    if (error)
    goto out;
    error = i2c_smbus_write_byte_data(client, CALIBRATION_REG3,
    reg3_orig);
    if (error)
    goto out;
// calibration data enable
    error = i2c_smbus_write_byte_data(client, TEST1,
    DUALTOUCH_STABILIZE_ON |
    DUALTOUCH_REG_ON);
    if (error)
    goto out;
// wait 10 sampling
    mdelay(10 * SAMPLING_DELAY);
    error = -EBUSY;
    }
    out:
    error2 = i2c_smbus_write_byte_data(client, INT_MASK, INT_ALL);
    if (!error2)
// Clear all interrupts
    error2 = i2c_smbus_write_byte_data(client, INT_CLEAR, 0xff);
    return error ? error : error2;
    }
    static const unsigned int untouch_threshold[3] = { 0, 1, 5 };
    static const unsigned int single_touch_threshold[3] = { 0, 0, 4 };
    static const unsigned int dual_touch_threshold[3] = { 10, 8, 0 };
#[no_mangle]
unsafe extern "C" fn rohm_ts_soft_irq(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t rohm_ts_soft_irq(int irq, void *dev_id)
    {
    struct rohm_ts_data *ts = dev_id;
    struct i2c_client *client = ts.client;
    struct input_dev *input_dev = ts.input;
    struct device *dev = &client.dev;
    u8 buf[10];	/* for POS_X1_H(0x20)-TOUCH_GESTURE(0x29) */
    struct input_mt_pos pos[MAX_CONTACTS];
    int slots[MAX_CONTACTS];
    u8 touch_flags;
    unsigned int threshold;
    let mut finger_count: c_int = -1;
    let mut prev_finger_count: c_int = ts.finger_count;
    int count;
    int error;
    int i;
    error = i2c_smbus_write_byte_data(client, INT_MASK, INT_ALL);
    if (error)
    return IRQ_HANDLED;
// Clear all interrupts
    error = i2c_smbus_write_byte_data(client, INT_CLEAR, 0xff);
    if (error)
    return IRQ_HANDLED;

    error = rohm_i2c_burst_read(client, POS_X1_H, buf, sizeof(buf));
    if (error)
    return IRQ_HANDLED;
    touch_flags = READ_POS_BUF(TOUCH_GESTURE) & TOUCH_MASK;
    if (touch_flags) {
// generate coordinates
    pos[0].x = ((s16)READ_POS_BUF(POS_X1_H) << 2) |
    READ_POS_BUF(POS_X1_L);
    pos[0].y = ((s16)READ_POS_BUF(POS_Y1_H) << 2) |
    READ_POS_BUF(POS_Y1_L);
    pos[1].x = ((s16)READ_POS_BUF(POS_X2_H) << 2) |
    READ_POS_BUF(POS_X2_L);
    pos[1].y = ((s16)READ_POS_BUF(POS_Y2_H) << 2) |
    READ_POS_BUF(POS_Y2_L);
    }
    switch (touch_flags) {
    case 0:
    threshold = untouch_threshold[prev_finger_count];
    if (++ts.contact_count[0] >= threshold)
    finger_count = 0;
    break;
    case SINGLE_TOUCH:
    threshold = single_touch_threshold[prev_finger_count];
    if (++ts.contact_count[1] >= threshold)
    finger_count = 1;
    if (finger_count == 1) {
    if (pos[1].x != 0 && pos[1].y != 0) {
    pos[0].x = pos[1].x;
    pos[0].y = pos[1].y;
    pos[1].x = 0;
    pos[1].y = 0;
    }
    }
    break;
    case DUAL_TOUCH:
    threshold = dual_touch_threshold[prev_finger_count];
    if (++ts.contact_count[2] >= threshold)
    finger_count = 2;
    break;
    default:
    dev_dbg(dev,
    "Three or more touches are not supported\n");
    return IRQ_HANDLED;
    }
    if (finger_count >= 0) {
    if (prev_finger_count != finger_count) {
    count = ts.contact_count[finger_count];
    memset(ts.contact_count, 0, sizeof(ts.contact_count));
    ts.contact_count[finger_count] = count;
    }
    input_mt_assign_slots(input_dev, slots, pos,
    finger_count, ROHM_TS_DISPLACEMENT_MAX);
    for (i = 0; i < finger_count; i++) {
    input_mt_slot(input_dev, slots[i]);
    input_mt_report_slot_state(input_dev,
    MT_TOOL_FINGER, true);
    input_report_abs(input_dev,
    ABS_MT_POSITION_X, pos[i].x);
    input_report_abs(input_dev,
    ABS_MT_POSITION_Y, pos[i].y);
    }
    input_mt_sync_frame(input_dev);
    input_mt_report_pointer_emulation(input_dev, true);
    input_sync(input_dev);
    ts.finger_count = finger_count;
    }
    if (READ_POS_BUF(TOUCH_GESTURE) & CALIBRATION_REQUEST) {
    error = rohm_ts_manual_calibration(ts);
    if (error)
    dev_warn(dev, "manual calibration failed: %d\n",
    error);
    }
    i2c_smbus_write_byte_data(client, INT_MASK,
    CALIBRATION_DONE | SLEEP_OUT | SLEEP_IN |
    PROGRAM_LOAD_DONE);
    return IRQ_HANDLED;
    }
    static int rohm_ts_load_firmware(struct i2c_client *client,
    const char *firmware_name)
    {
    struct device *dev = &client.dev;
    s32 status;
    unsigned int offset, len, xfer_len;
    let mut retry: c_uint = 0;
    int error, error2;
    const struct firmware *fw __free(firmware) = core::ptr::null_mut();
    error = request_firmware(&fw, firmware_name, dev);
    if (error) {
    dev_err(dev, "unable to retrieve firmware %s: %d\n",
    firmware_name, error);
    return error;
    }
    error = i2c_smbus_write_byte_data(client, INT_MASK,
    COORD_UPDATE | CALIBRATION_DONE |
    SLEEP_IN | SLEEP_OUT);
    if (error)
    goto out;
    do {
    if (retry) {
    dev_warn(dev, "retrying firmware load\n");
// settings for retry
    error = i2c_smbus_write_byte_data(client, EX_WDAT, 0);
    if (error)
    goto out;
    }
    error = i2c_smbus_write_byte_data(client, EX_ADDR_H, 0);
    if (error)
    goto out;
    error = i2c_smbus_write_byte_data(client, EX_ADDR_L, 0);
    if (error)
    goto out;
    error = i2c_smbus_write_byte_data(client, COMMON_SETUP1,
    COMMON_SETUP1_DEFAULT);
    if (error)
    goto out;
// firmware load to the device
    offset = 0;
    len = fw.size;
    while (len) {
    xfer_len = min(FIRMWARE_BLOCK_SIZE, len);
    error = i2c_smbus_write_i2c_block_data(client, EX_WDAT,
    xfer_len, &fw.data[offset]);
    if (error)
    goto out;
    len -= xfer_len;
    offset += xfer_len;
    }
// check firmware load result
    status = i2c_smbus_read_byte_data(client, INT_STATUS);
    if (status < 0) {
    error = status;
    goto out;
    }
// clear all interrupts
    error = i2c_smbus_write_byte_data(client, INT_CLEAR, 0xff);
    if (error)
    goto out;
    if (status == PROGRAM_LOAD_DONE)
    break;
    error = -EIO;
    } while (++retry <= FIRMWARE_RETRY_MAX);
    out:
    error2 = i2c_smbus_write_byte_data(client, INT_MASK, INT_ALL);
    return error ? error : error2;
    }
    static int rohm_ts_update_setting(struct rohm_ts_data *ts,
    unsigned int setting_bit, bool on)
    {
    int error;
    scoped_cond_guard(mutex_intr, return -EINTR, &ts.input.mutex) {
    if (on)
    ts.setup2 |= setting_bit;
    else
    ts.setup2 &= ~setting_bit;
    if (ts.initialized) {
    error = i2c_smbus_write_byte_data(ts.client,
    COMMON_SETUP2,
    ts.setup2);
    if (error)
    return error;
    }
    }
    return 0;
    }
    static ssize_t swap_xy_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    struct i2c_client *client = to_i2c_client(dev);
    struct rohm_ts_data *ts = i2c_get_clientdata(client);
    return sysfs_emit(buf, "%d\n", !!(ts.setup2 & SWAP_XY));
    }
    static ssize_t swap_xy_store(struct device *dev, struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct i2c_client *client = to_i2c_client(dev);
    struct rohm_ts_data *ts = i2c_get_clientdata(client);
    unsigned int val;
    int error;
    error = kstrtouint(buf, 0, &val);
    if (error)
    return error;
    error = rohm_ts_update_setting(ts, SWAP_XY, val);
    return error ?: count;
    }
    static ssize_t inv_x_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    struct i2c_client *client = to_i2c_client(dev);
    struct rohm_ts_data *ts = i2c_get_clientdata(client);
    return sysfs_emit(buf, "%d\n", !!(ts.setup2 & INV_X));
    }
    static ssize_t inv_x_store(struct device *dev, struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct i2c_client *client = to_i2c_client(dev);
    struct rohm_ts_data *ts = i2c_get_clientdata(client);
    unsigned int val;
    int error;
    error = kstrtouint(buf, 0, &val);
    if (error)
    return error;
    error = rohm_ts_update_setting(ts, INV_X, val);
    return error ?: count;
    }
    static ssize_t inv_y_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    struct i2c_client *client = to_i2c_client(dev);
    struct rohm_ts_data *ts = i2c_get_clientdata(client);
    return sysfs_emit(buf, "%d\n", !!(ts.setup2 & INV_Y));
    }
    static ssize_t inv_y_store(struct device *dev, struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct i2c_client *client = to_i2c_client(dev);
    struct rohm_ts_data *ts = i2c_get_clientdata(client);
    unsigned int val;
    int error;
    error = kstrtouint(buf, 0, &val);
    if (error)
    return error;
    error = rohm_ts_update_setting(ts, INV_Y, val);
    return error ?: count;
    }
    static DEVICE_ATTR_RW(swap_xy);
    static DEVICE_ATTR_RW(inv_x);
    static DEVICE_ATTR_RW(inv_y);
    static struct attribute *rohm_ts_attrs[] = {
    &dev_attr_swap_xy.attr,
    &dev_attr_inv_x.attr,
    &dev_attr_inv_y.attr,
    core::ptr::null_mut(),
    };
    ATTRIBUTE_GROUPS(rohm_ts);
#[no_mangle]
unsafe extern "C" fn rohm_ts_device_init(client: *mut i2c_client, setup2: u8) -> c_int {
    static int rohm_ts_device_init(struct i2c_client *client, u8 setup2)
    {
    struct device *dev = &client.dev;
    int error;
    guard(disable_irq)(&client.irq);
//
// Wait 200usec for reset
//
    udelay(200);
// Release analog reset
    error = i2c_smbus_write_byte_data(client, SYSTEM,
    ANALOG_POWER_ON | CPU_POWER_OFF);
    if (error)
    return error;
// Waiting for the analog warm-up, max. 200usec
    udelay(200);
// clear all interrupts
    error = i2c_smbus_write_byte_data(client, INT_CLEAR, 0xff);
    if (error)
    return error;
    error = i2c_smbus_write_byte_data(client, EX_WDAT, 0);
    if (error)
    return error;
    error = i2c_smbus_write_byte_data(client, COMMON_SETUP1, 0);
    if (error)
    return error;
    error = i2c_smbus_write_byte_data(client, COMMON_SETUP2, setup2);
    if (error)
    return error;
    error = i2c_smbus_write_byte_data(client, COMMON_SETUP3,
    SEL_TBL_DEFAULT | EN_MULTI);
    if (error)
    return error;
    error = i2c_smbus_write_byte_data(client, THRESHOLD_GESTURE,
    THRESHOLD_GESTURE_DEFAULT);
    if (error)
    return error;
    error = i2c_smbus_write_byte_data(client, INTERVAL_TIME,
    INTERVAL_TIME_DEFAULT);
    if (error)
    return error;
    error = i2c_smbus_write_byte_data(client, CPU_FREQ, CPU_FREQ_10MHZ);
    if (error)
    return error;
    error = i2c_smbus_write_byte_data(client, PRM_SWOFF_TIME,
    PRM_SWOFF_TIME_DEFAULT);
    if (error)
    return error;
    error = i2c_smbus_write_byte_data(client, ADC_CTRL, ADC_DIV_DEFAULT);
    if (error)
    return error;
    error = i2c_smbus_write_byte_data(client, ADC_WAIT, ADC_WAIT_DEFAULT);
    if (error)
    return error;
//
// Panel setup, these values change with the panel.
//
    error = i2c_smbus_write_byte_data(client, STEP_X, STEP_X_DEFAULT);
    if (error)
    return error;
    error = i2c_smbus_write_byte_data(client, STEP_Y, STEP_Y_DEFAULT);
    if (error)
    return error;
    error = i2c_smbus_write_byte_data(client, OFFSET_X, OFFSET_X_DEFAULT);
    if (error)
    return error;
    error = i2c_smbus_write_byte_data(client, OFFSET_Y, OFFSET_Y_DEFAULT);
    if (error)
    return error;
    error = i2c_smbus_write_byte_data(client, THRESHOLD_TOUCH,
    THRESHOLD_TOUCH_DEFAULT);
    if (error)
    return error;
    error = i2c_smbus_write_byte_data(client, EVR_XY, EVR_XY_DEFAULT);
    if (error)
    return error;
    error = i2c_smbus_write_byte_data(client, EVR_X, EVR_X_DEFAULT);
    if (error)
    return error;
    error = i2c_smbus_write_byte_data(client, EVR_Y, EVR_Y_DEFAULT);
    if (error)
    return error;
// Fixed value settings
    error = i2c_smbus_write_byte_data(client, CALIBRATION_ADJUST,
    CALIBRATION_ADJUST_DEFAULT);
    if (error)
    return error;
    error = i2c_smbus_write_byte_data(client, SWCONT, SWCONT_DEFAULT);
    if (error)
    return error;
    error = i2c_smbus_write_byte_data(client, TEST1,
    DUALTOUCH_STABILIZE_ON |
    DUALTOUCH_REG_ON);
    if (error)
    return error;
    error = rohm_ts_load_firmware(client, BU21023_FIRMWARE_NAME);
    if (error) {
    dev_err(dev, "failed to load firmware: %d\n", error);
    return error;
    }
//
// Manual calibration results are not changed in same environment.
// If the force calibration is performed,
// the controller will not require calibration request interrupt
// when the typical values are set to the calibration registers.
//
    error = i2c_smbus_write_byte_data(client, CALIBRATION_REG1,
    CALIBRATION_REG1_DEFAULT);
    if (error)
    return error;
    error = i2c_smbus_write_byte_data(client, CALIBRATION_REG2,
    CALIBRATION_REG2_DEFAULT);
    if (error)
    return error;
    error = i2c_smbus_write_byte_data(client, CALIBRATION_REG3,
    CALIBRATION_REG3_DEFAULT);
    if (error)
    return error;
    error = i2c_smbus_write_byte_data(client, FORCE_CALIBRATION,
    FORCE_CALIBRATION_OFF);
    if (error)
    return error;
    error = i2c_smbus_write_byte_data(client, FORCE_CALIBRATION,
    FORCE_CALIBRATION_ON);
    if (error)
    return error;
// Clear all interrupts
    error = i2c_smbus_write_byte_data(client, INT_CLEAR, 0xff);
    if (error)
    return error;
// Enable coordinates update interrupt
    error = i2c_smbus_write_byte_data(client, INT_MASK,
    CALIBRATION_DONE | SLEEP_OUT |
    SLEEP_IN | PROGRAM_LOAD_DONE);
    if (error)
    return error;
    error = i2c_smbus_write_byte_data(client, ERR_MASK,
    PROGRAM_LOAD_ERR | CPU_TIMEOUT |
    ADC_TIMEOUT);
    if (error)
    return error;
// controller CPU power on
    error = i2c_smbus_write_byte_data(client, SYSTEM,
    ANALOG_POWER_ON | CPU_POWER_ON);
    if (error)
    return error;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rohm_ts_power_off(client: *mut i2c_client) -> c_int {
    static int rohm_ts_power_off(struct i2c_client *client)
    {
    int error;
    error = i2c_smbus_write_byte_data(client, SYSTEM,
    ANALOG_POWER_ON | CPU_POWER_OFF);
    if (error) {
    dev_err(&client.dev,
    "failed to power off device CPU: %d\n", error);
    return error;
    }
    error = i2c_smbus_write_byte_data(client, SYSTEM,
    ANALOG_POWER_OFF | CPU_POWER_OFF);
    if (error)
    dev_err(&client.dev,
    "failed to power off the device: %d\n", error);
    return error;
    }
#[no_mangle]
unsafe extern "C" fn rohm_ts_open(input_dev: *mut input_dev) -> c_int {
    static int rohm_ts_open(struct input_dev *input_dev)
    {
    struct rohm_ts_data *ts = input_get_drvdata(input_dev);
    struct i2c_client *client = ts.client;
    int error;
    if (!ts.initialized) {
    error = rohm_ts_device_init(client, ts.setup2);
    if (error) {
    dev_err(&client.dev,
    "device initialization failed: %d\n", error);
    return error;
    }
    ts.initialized = true;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rohm_ts_close(input_dev: *mut input_dev) {
    static void rohm_ts_close(struct input_dev *input_dev)
    {
    struct rohm_ts_data *ts = input_get_drvdata(input_dev);
    rohm_ts_power_off(ts.client);
    ts.initialized = false;
    }
#[no_mangle]
unsafe extern "C" fn rohm_bu21023_i2c_probe(client: *mut i2c_client) -> c_int {
    static int rohm_bu21023_i2c_probe(struct i2c_client *client)
    {
    struct device *dev = &client.dev;
    struct rohm_ts_data *ts;
    struct input_dev *input;
    int error;
    if (!client.irq) {
    dev_err(dev, "IRQ is not assigned\n");
    return -EINVAL;
    }
    if (!client.adapter.algo.master_xfer) {
    dev_err(dev, "I2C level transfers not supported\n");
    return -EOPNOTSUPP;
    }
// Turn off CPU just in case
    error = rohm_ts_power_off(client);
    if (error)
    return error;
    ts = devm_kzalloc(dev, sizeof(struct rohm_ts_data), GFP_KERNEL);
    if (!ts)
    return -ENOMEM;
    ts.client = client;
    ts.setup2 = MAF_1SAMPLE;
    i2c_set_clientdata(client, ts);
    input = devm_input_allocate_device(dev);
    if (!input)
    return -ENOMEM;
    input.name = BU21023_NAME;
    input.id.bustype = BUS_I2C;
    input.open = rohm_ts_open;
    input.close = rohm_ts_close;
    ts.input = input;
    input_set_drvdata(input, ts);
    input_set_abs_params(input, ABS_MT_POSITION_X,
    ROHM_TS_ABS_X_MIN, ROHM_TS_ABS_X_MAX, 0, 0);
    input_set_abs_params(input, ABS_MT_POSITION_Y,
    ROHM_TS_ABS_Y_MIN, ROHM_TS_ABS_Y_MAX, 0, 0);
    error = input_mt_init_slots(input, MAX_CONTACTS,
    INPUT_MT_DIRECT | INPUT_MT_TRACK |
    INPUT_MT_DROP_UNUSED);
    if (error) {
    dev_err(dev, "failed to multi touch slots initialization\n");
    return error;
    }
    error = devm_request_threaded_irq(dev, client.irq,
    core::ptr::null_mut(), rohm_ts_soft_irq,
    IRQF_ONESHOT, client.name, ts);
    if (error) {
    dev_err(dev, "failed to request IRQ: %d\n", error);
    return error;
    }
    error = input_register_device(input);
    if (error) {
    dev_err(dev, "failed to register input device: %d\n", error);
    return error;
    }
    return error;
    }
    static const struct i2c_device_id rohm_bu21023_i2c_id[] = {
    { .name = BU21023_NAME },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(i2c, rohm_bu21023_i2c_id);
    static struct i2c_driver rohm_bu21023_i2c_driver = {
    .driver = {
    .name = BU21023_NAME,
    .dev_groups = rohm_ts_groups,
    },
    .probe = rohm_bu21023_i2c_probe,
    .id_table = rohm_bu21023_i2c_id,
    };
    module_i2c_driver(rohm_bu21023_i2c_driver);
    MODULE_DESCRIPTION("ROHM BU21023/24 Touchscreen driver");
    MODULE_LICENSE("GPL v2");
    MODULE_AUTHOR("ROHM Co., Ltd.");
