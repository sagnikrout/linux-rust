//! Automatically rewritten from C to Rust
//! Source: drivers/hid/hid-alps.c
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
// Copyright (c) 2016 Masaki Ota <masaki.ota@jp.alps.com>
//

// ALPS Device Product ID
pub const HID_PRODUCT_ID_T3_BTNLESS: c_uint = 0xD0C0;
pub const HID_PRODUCT_ID_COSMO: c_uint = 0x1202;
pub const HID_PRODUCT_ID_U1_PTP_1: c_uint = 0x1207;
pub const HID_PRODUCT_ID_U1: c_uint = 0x1209;
pub const HID_PRODUCT_ID_U1_PTP_2: c_uint = 0x120A;
pub const HID_PRODUCT_ID_U1_DUAL: c_uint = 0x120B;
pub const HID_PRODUCT_ID_T4_BTNLESS: c_uint = 0x120C;
pub const DEV_SINGLEPOINT: c_uint = 0x01;
pub const DEV_DUALPOINT: c_uint = 0x02;
pub const U1_MOUSE_REPORT_ID: c_uint = 0x01 /* Mouse data ReportID */;
pub const U1_ABSOLUTE_REPORT_ID: c_uint = 0x03 /* Absolute data ReportID */;
pub const U1_ABSOLUTE_REPORT_ID_SECD: c_uint = 0x02 /* FW-PTP Absolute data ReportID */;
pub const U1_FEATURE_REPORT_ID: c_uint = 0x05 /* Feature ReportID */;
pub const U1_SP_ABSOLUTE_REPORT_ID: c_uint = 0x06 /* Feature ReportID */;
pub const U1_FEATURE_REPORT_LEN: c_uint = 0x08 /* Feature Report Length */;
pub const U1_FEATURE_REPORT_LEN_ALL: c_uint = 0x0A;
pub const U1_CMD_REGISTER_READ: c_uint = 0xD1;
pub const U1_CMD_REGISTER_WRITE: c_uint = 0xD2;
pub const U1_DEVTYPE_SP_SUPPORT: c_uint = 0x10 /* SP Support */;
pub const U1_DISABLE_DEV: c_uint = 0x01;
pub const U1_TP_ABS_MODE: c_uint = 0x02;
pub const U1_SP_ABS_MODE: c_uint = 0x80;
pub const ADDRESS_U1_DEV_CTRL_1: c_uint = 0x00800040;
pub const ADDRESS_U1_DEVICE_TYP: c_uint = 0x00800043;
pub const ADDRESS_U1_NUM_SENS_X: c_uint = 0x00800047;
pub const ADDRESS_U1_NUM_SENS_Y: c_uint = 0x00800048;
pub const ADDRESS_U1_PITCH_SENS_X: c_uint = 0x00800049;
pub const ADDRESS_U1_PITCH_SENS_Y: c_uint = 0x0080004A;
pub const ADDRESS_U1_RESO_DWN_ABS: c_uint = 0x0080004E;
pub const ADDRESS_U1_PAD_BTN: c_uint = 0x00800052;
pub const ADDRESS_U1_SP_BTN: c_uint = 0x0080009F;

pub const T4_FEATURE_REPORT_ID: c_int = 7;
pub const T4_CMD_REGISTER_READ: c_uint = 0x08;
pub const T4_CMD_REGISTER_WRITE: c_uint = 0x07;
pub const T4_ADDRESS_BASE: c_uint = 0xC2C0;

pub const T4_FEEDCFG4_ADVANCED_ABS_ENABLE: c_uint = 0x01;
pub const T4_I2C_ABS: c_uint = 0x78;
pub const T4_COUNT_PER_ELECTRODE: c_int = 256;
pub const MAX_TOUCHES: c_int = 5;
    enum dev_num {
    U1,
    T4,
    UNKNOWN,
    };
//
// struct alps_dev
//
// @input: pointer to the kernel input device
// @input2: pointer to the kernel input2 device
// @hdev: pointer to the struct hid_device
//
// @dev_type: device type
// @max_fingers: total number of fingers
// @has_sp: boolean of sp existense
// @sp_btn_info: button information
// @x_active_len_mm: active area length of X (mm)
// @y_active_len_mm: active area length of Y (mm)
// @x_max: maximum x coordinate value
// @y_max: maximum y coordinate value
// @x_min: minimum x coordinate value
// @y_min: minimum y coordinate value
// @btn_cnt: number of buttons
// @sp_btn_cnt: number of stick buttons
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct alps_dev {
    pub input: *mut input_dev,
    pub input2: *mut input_dev,
    pub hdev: *mut hid_device,
    pub dev_type: enum dev_num,
    pub max_fingers: u8,
    pub has_sp: u8,
    pub sp_btn_info: u8,
    pub x_active_len_mm: u32,
    pub y_active_len_mm: u32,
    pub x_max: u32,
    pub y_max: u32,
    pub x_min: u32,
    pub y_min: u32,
    pub btn_cnt: u32,
    pub sp_btn_cnt: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct t4_contact_data {
    pub palm: u8,
    pub x_lo: u8,
    pub x_hi: u8,
    pub y_lo: u8,
    pub y_hi: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct t4_input_report {
    pub reportID: u8,
    pub numContacts: u8,
    pub contact: [t4_contact_data; 5],
    pub button: u8,
    pub track: [u8; 5],
    pub zy: [u8 zx[5],; 5],
    pub palmTime: [u8; 5],
    pub kilroy: u8,
    pub timeStamp: u16,
}

    static u16 t4_calc_check_sum(u8 *buffer,
    unsigned long offset, unsigned long length)
    {
    let mut sum1: u16 = 0xFF, sum2 = 0xFF;
    let mut i: c_ulong = 0;
    if (offset + length >= 50)
    return 0;
    while (length > 0) {
    let mut tlen: u32 = length > 20 ? 20 : length;
    length -= tlen;
    do {
    sum1 += buffer[offset + i];
    sum2 += sum1;
    i++;
    } while (--tlen > 0);
    sum1 = (sum1 & 0xFF) + (sum1 >> 8);
    sum2 = (sum2 & 0xFF) + (sum2 >> 8);
    }
    sum1 = (sum1 & 0xFF) + (sum1 >> 8);
    sum2 = (sum2 & 0xFF) + (sum2 >> 8);
    return(sum2 << 8 | sum1);
    }
    static int t4_read_write_register(struct hid_device *hdev, u32 address,
    u8 *read_val, u8 write_val, bool read_flag)
    {
    int ret;
    u16 check_sum;
    u8 *input;
    u8 *readbuf = core::ptr::null_mut();
    input = kzalloc(T4_FEATURE_REPORT_LEN, GFP_KERNEL);
    if (!input)
    return -ENOMEM;
    input[0] = T4_FEATURE_REPORT_ID;
    if (read_flag) {
    input[1] = T4_CMD_REGISTER_READ;
    input[8] = 0x00;
    } else {
    input[1] = T4_CMD_REGISTER_WRITE;
    input[8] = write_val;
    }
    put_unaligned_le32(address, input + 2);
    input[6] = 1;
    input[7] = 0;
// Calculate the checksum
    check_sum = t4_calc_check_sum(input, 1, 8);
    input[9] = (u8)check_sum;
    input[10] = (u8)(check_sum >> 8);
    input[11] = 0;
    ret = hid_hw_raw_request(hdev, T4_FEATURE_REPORT_ID, input,
    T4_FEATURE_REPORT_LEN,
    HID_FEATURE_REPORT, HID_REQ_SET_REPORT);
    if (ret < 0) {
    dev_err(&hdev.dev, "failed to read command (%d)\n", ret);
    goto exit;
    }
    if (read_flag) {
    readbuf = kzalloc(T4_FEATURE_REPORT_LEN, GFP_KERNEL);
    if (!readbuf) {
    ret = -ENOMEM;
    goto exit;
    }
    ret = hid_hw_raw_request(hdev, T4_FEATURE_REPORT_ID, readbuf,
    T4_FEATURE_REPORT_LEN,
    HID_FEATURE_REPORT, HID_REQ_GET_REPORT);
    if (ret < 0) {
    dev_err(&hdev.dev, "failed read register (%d)\n", ret);
    goto exit_readbuf;
    }
    ret = -EINVAL;
    if (*(u32 *)&readbuf[6] != address) {
    dev_err(&hdev.dev, "read register address error (%x,%x)\n",
// (u32 *)&readbuf[6], address);
    goto exit_readbuf;
    }
    if (*(u16 *)&readbuf[10] != 1) {
    dev_err(&hdev.dev, "read register size error (%x)\n",
// (u16 *)&readbuf[10]);
    goto exit_readbuf;
    }
    check_sum = t4_calc_check_sum(readbuf, 6, 7);
    if (*(u16 *)&readbuf[13] != check_sum) {
    dev_err(&hdev.dev, "read register checksum error (%x,%x)\n",
// (u16 *)&readbuf[13], check_sum);
    goto exit_readbuf;
    }
// read_val = readbuf[12];
    }
    ret = 0;
    exit_readbuf:
    kfree(readbuf);
    exit:
    kfree(input);
    return ret;
    }
    static int u1_read_write_register(struct hid_device *hdev, u32 address,
    u8 *read_val, u8 write_val, bool read_flag)
    {
    int ret, i;
    u8 check_sum;
    u8 *input;
    u8 *readbuf;
    input = kzalloc(U1_FEATURE_REPORT_LEN, GFP_KERNEL);
    if (!input)
    return -ENOMEM;
    input[0] = U1_FEATURE_REPORT_ID;
    if (read_flag) {
    input[1] = U1_CMD_REGISTER_READ;
    input[6] = 0x00;
    } else {
    input[1] = U1_CMD_REGISTER_WRITE;
    input[6] = write_val;
    }
    put_unaligned_le32(address, input + 2);
// Calculate the checksum
    check_sum = U1_FEATURE_REPORT_LEN_ALL;
    for (i = 0; i < U1_FEATURE_REPORT_LEN - 1; i++)
    check_sum += input[i];
    input[7] = check_sum;
    ret = hid_hw_raw_request(hdev, U1_FEATURE_REPORT_ID, input,
    U1_FEATURE_REPORT_LEN,
    HID_FEATURE_REPORT, HID_REQ_SET_REPORT);
    if (ret < 0) {
    dev_err(&hdev.dev, "failed to read command (%d)\n", ret);
    goto exit;
    }
    if (read_flag) {
    readbuf = kzalloc(U1_FEATURE_REPORT_LEN, GFP_KERNEL);
    if (!readbuf) {
    ret = -ENOMEM;
    goto exit;
    }
    ret = hid_hw_raw_request(hdev, U1_FEATURE_REPORT_ID, readbuf,
    U1_FEATURE_REPORT_LEN,
    HID_FEATURE_REPORT, HID_REQ_GET_REPORT);
    if (ret < 0) {
    dev_err(&hdev.dev, "failed read register (%d)\n", ret);
    kfree(readbuf);
    goto exit;
    }
// read_val = readbuf[6];
    kfree(readbuf);
    }
    ret = 0;
    exit:
    kfree(input);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn t4_raw_event(hdata: *mut alps_dev, data: *mut u8, size: c_int) -> c_int {
    static int t4_raw_event(struct alps_dev *hdata, u8 *data, int size)
    {
    unsigned int x, y, z;
    int i;
    struct t4_input_report *p_report = (struct t4_input_report *)data;
    if (!data)
    return 0;
    for (i = 0; i < hdata.max_fingers; i++) {
    x = p_report.contact[i].x_hi << 8 | p_report.contact[i].x_lo;
    y = p_report.contact[i].y_hi << 8 | p_report.contact[i].y_lo;
    y = hdata.y_max - y + hdata.y_min;
    z = (p_report.contact[i].palm < 0x80 &&
    p_report.contact[i].palm > 0) * 62;
    if (x == 0xffff) {
    x = 0;
    y = 0;
    z = 0;
    }
    input_mt_slot(hdata.input, i);
    input_mt_report_slot_state(hdata.input,
    MT_TOOL_FINGER, z != 0);
    if (!z)
    continue;
    input_report_abs(hdata.input, ABS_MT_POSITION_X, x);
    input_report_abs(hdata.input, ABS_MT_POSITION_Y, y);
    input_report_abs(hdata.input, ABS_MT_PRESSURE, z);
    }
    input_mt_sync_frame(hdata.input);
    input_report_key(hdata.input, BTN_LEFT, p_report.button);
    input_sync(hdata.input);
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn u1_raw_event(hdata: *mut alps_dev, data: *mut u8, size: c_int) -> c_int {
    static int u1_raw_event(struct alps_dev *hdata, u8 *data, int size)
    {
    unsigned int x, y, z;
    int i;
    short sp_x, sp_y;
    if (!data)
    return 0;
    switch (data[0]) {
    case U1_MOUSE_REPORT_ID:
    break;
    case U1_FEATURE_REPORT_ID:
    break;
    case U1_ABSOLUTE_REPORT_ID:
    case U1_ABSOLUTE_REPORT_ID_SECD:
    for (i = 0; i < hdata.max_fingers; i++) {
    u8 *contact = &data[i * 5];
    x = get_unaligned_le16(contact + 3);
    y = get_unaligned_le16(contact + 5);
    z = contact[7] & 0x7F;
    input_mt_slot(hdata.input, i);
    if (z != 0) {
    input_mt_report_slot_state(hdata.input,
    MT_TOOL_FINGER, 1);
    input_report_abs(hdata.input,
    ABS_MT_POSITION_X, x);
    input_report_abs(hdata.input,
    ABS_MT_POSITION_Y, y);
    input_report_abs(hdata.input,
    ABS_MT_PRESSURE, z);
    } else {
    input_mt_report_slot_inactive(hdata.input);
    }
    }
    input_mt_sync_frame(hdata.input);
    input_report_key(hdata.input, BTN_LEFT,
    data[1] & 0x1);
    input_report_key(hdata.input, BTN_RIGHT,
    (data[1] & 0x2));
    input_report_key(hdata.input, BTN_MIDDLE,
    (data[1] & 0x4));
    input_sync(hdata.input);
    return 1;
    case U1_SP_ABSOLUTE_REPORT_ID:
    sp_x = get_unaligned_le16(data+2);
    sp_y = get_unaligned_le16(data+4);
    sp_x = sp_x / 8;
    sp_y = sp_y / 8;
    input_report_rel(hdata.input2, REL_X, sp_x);
    input_report_rel(hdata.input2, REL_Y, sp_y);
    input_report_key(hdata.input2, BTN_LEFT,
    data[1] & 0x1);
    input_report_key(hdata.input2, BTN_RIGHT,
    (data[1] & 0x2));
    input_report_key(hdata.input2, BTN_MIDDLE,
    (data[1] & 0x4));
    input_sync(hdata.input2);
    return 1;
    }
    return 0;
    }
    static int alps_raw_event(struct hid_device *hdev,
    struct hid_report *report, u8 *data, int size)
    {
    let mut ret: c_int = 0;
    struct alps_dev *hdata = hid_get_drvdata(hdev);
    if (!(hdev.claimed & HID_CLAIMED_INPUT) || !hdata.input)
    return 0;
    switch (hdev.product) {
    case HID_PRODUCT_ID_T4_BTNLESS:
    ret = t4_raw_event(hdata, data, size);
    break;
    default:
    ret = u1_raw_event(hdata, data, size);
    break;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn alps_post_reset(hdev: *mut hid_device) -> int __maybe_unused {
    static int __maybe_unused alps_post_reset(struct hid_device *hdev)
    {
    let mut ret: c_int = -1;
    struct alps_dev *data = hid_get_drvdata(hdev);
    switch (data.dev_type) {
    case T4:
    ret = t4_read_write_register(hdev, T4_PRM_FEED_CONFIG_1,
    core::ptr::null_mut(), T4_I2C_ABS, false);
    if (ret < 0) {
    dev_err(&hdev.dev, "failed T4_PRM_FEED_CONFIG_1 (%d)\n",
    ret);
    goto exit;
    }
    ret = t4_read_write_register(hdev, T4_PRM_FEED_CONFIG_4,
    core::ptr::null_mut(), T4_FEEDCFG4_ADVANCED_ABS_ENABLE, false);
    if (ret < 0) {
    dev_err(&hdev.dev, "failed T4_PRM_FEED_CONFIG_4 (%d)\n",
    ret);
    goto exit;
    }
    break;
    case U1:
    ret = u1_read_write_register(hdev,
    ADDRESS_U1_DEV_CTRL_1, core::ptr::null_mut(),
    U1_TP_ABS_MODE | U1_SP_ABS_MODE, false);
    if (ret < 0) {
    dev_err(&hdev.dev, "failed to change TP mode (%d)\n",
    ret);
    goto exit;
    }
    break;
    default:
    break;
    }
    exit:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn alps_post_resume(hdev: *mut hid_device) -> int __maybe_unused {
    static int __maybe_unused alps_post_resume(struct hid_device *hdev)
    {
    return alps_post_reset(hdev);
    }
#[no_mangle]
unsafe extern "C" fn u1_init(hdev: *mut hid_device, pri_data: *mut alps_dev) -> c_int {
    static int u1_init(struct hid_device *hdev, struct alps_dev *pri_data)
    {
    int ret;
    u8 tmp, dev_ctrl, sen_line_num_x, sen_line_num_y;
    u8 pitch_x, pitch_y, resolution;
// Device initialization
    ret = u1_read_write_register(hdev, ADDRESS_U1_DEV_CTRL_1,
    &dev_ctrl, 0, true);
    if (ret < 0) {
    dev_err(&hdev.dev, "failed U1_DEV_CTRL_1 (%d)\n", ret);
    goto exit;
    }
    dev_ctrl &= ~U1_DISABLE_DEV;
    dev_ctrl |= U1_TP_ABS_MODE;
    ret = u1_read_write_register(hdev, ADDRESS_U1_DEV_CTRL_1,
    core::ptr::null_mut(), dev_ctrl, false);
    if (ret < 0) {
    dev_err(&hdev.dev, "failed to change TP mode (%d)\n", ret);
    goto exit;
    }
    ret = u1_read_write_register(hdev, ADDRESS_U1_NUM_SENS_X,
    &sen_line_num_x, 0, true);
    if (ret < 0) {
    dev_err(&hdev.dev, "failed U1_NUM_SENS_X (%d)\n", ret);
    goto exit;
    }
    ret = u1_read_write_register(hdev, ADDRESS_U1_NUM_SENS_Y,
    &sen_line_num_y, 0, true);
    if (ret < 0) {
    dev_err(&hdev.dev, "failed U1_NUM_SENS_Y (%d)\n", ret);
    goto exit;
    }
    ret = u1_read_write_register(hdev, ADDRESS_U1_PITCH_SENS_X,
    &pitch_x, 0, true);
    if (ret < 0) {
    dev_err(&hdev.dev, "failed U1_PITCH_SENS_X (%d)\n", ret);
    goto exit;
    }
    ret = u1_read_write_register(hdev, ADDRESS_U1_PITCH_SENS_Y,
    &pitch_y, 0, true);
    if (ret < 0) {
    dev_err(&hdev.dev, "failed U1_PITCH_SENS_Y (%d)\n", ret);
    goto exit;
    }
    ret = u1_read_write_register(hdev, ADDRESS_U1_RESO_DWN_ABS,
    &resolution, 0, true);
    if (ret < 0) {
    dev_err(&hdev.dev, "failed U1_RESO_DWN_ABS (%d)\n", ret);
    goto exit;
    }
    pri_data.x_active_len_mm =
    (pitch_x * (sen_line_num_x - 1)) / 10;
    pri_data.y_active_len_mm =
    (pitch_y * (sen_line_num_y - 1)) / 10;
    pri_data.x_max =
    (resolution << 2) * (sen_line_num_x - 1);
    pri_data.x_min = 1;
    pri_data.y_max =
    (resolution << 2) * (sen_line_num_y - 1);
    pri_data.y_min = 1;
    ret = u1_read_write_register(hdev, ADDRESS_U1_PAD_BTN,
    &tmp, 0, true);
    if (ret < 0) {
    dev_err(&hdev.dev, "failed U1_PAD_BTN (%d)\n", ret);
    goto exit;
    }
    if ((tmp & 0x0F) == (tmp & 0xF0) >> 4) {
    pri_data.btn_cnt = (tmp & 0x0F);
    } else {
// Button pad
    pri_data.btn_cnt = 1;
    }
    pri_data.has_sp = 0;
// Check StickPointer device
    ret = u1_read_write_register(hdev, ADDRESS_U1_DEVICE_TYP,
    &tmp, 0, true);
    if (ret < 0) {
    dev_err(&hdev.dev, "failed U1_DEVICE_TYP (%d)\n", ret);
    goto exit;
    }
    if (tmp & U1_DEVTYPE_SP_SUPPORT) {
    dev_ctrl |= U1_SP_ABS_MODE;
    ret = u1_read_write_register(hdev, ADDRESS_U1_DEV_CTRL_1,
    core::ptr::null_mut(), dev_ctrl, false);
    if (ret < 0) {
    dev_err(&hdev.dev, "failed SP mode (%d)\n", ret);
    goto exit;
    }
    ret = u1_read_write_register(hdev, ADDRESS_U1_SP_BTN,
    &pri_data.sp_btn_info, 0, true);
    if (ret < 0) {
    dev_err(&hdev.dev, "failed U1_SP_BTN (%d)\n", ret);
    goto exit;
    }
    pri_data.has_sp = 1;
    }
    pri_data.max_fingers = 5;
    exit:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn T4_init(hdev: *mut hid_device, pri_data: *mut alps_dev) -> c_int {
    static int T4_init(struct hid_device *hdev, struct alps_dev *pri_data)
    {
    int ret;
    u8 tmp, sen_line_num_x, sen_line_num_y;
    ret = t4_read_write_register(hdev, T4_PRM_ID_CONFIG_3, &tmp, 0, true);
    if (ret < 0) {
    dev_err(&hdev.dev, "failed T4_PRM_ID_CONFIG_3 (%d)\n", ret);
    goto exit;
    }
    sen_line_num_x = 16 + ((tmp & 0x0F)  | (tmp & 0x08 ? 0xF0 : 0));
    sen_line_num_y = 12 + (((tmp & 0xF0) >> 4)  | (tmp & 0x80 ? 0xF0 : 0));
    pri_data.x_max = sen_line_num_x * T4_COUNT_PER_ELECTRODE;
    pri_data.x_min = T4_COUNT_PER_ELECTRODE;
    pri_data.y_max = sen_line_num_y * T4_COUNT_PER_ELECTRODE;
    pri_data.y_min = T4_COUNT_PER_ELECTRODE;
    pri_data.x_active_len_mm = pri_data.y_active_len_mm = 0;
    pri_data.btn_cnt = 1;
    ret = t4_read_write_register(hdev, PRM_SYS_CONFIG_1, &tmp, 0, true);
    if (ret < 0) {
    dev_err(&hdev.dev, "failed PRM_SYS_CONFIG_1 (%d)\n", ret);
    goto exit;
    }
    tmp |= 0x02;
    ret = t4_read_write_register(hdev, PRM_SYS_CONFIG_1, core::ptr::null_mut(), tmp, false);
    if (ret < 0) {
    dev_err(&hdev.dev, "failed PRM_SYS_CONFIG_1 (%d)\n", ret);
    goto exit;
    }
    ret = t4_read_write_register(hdev, T4_PRM_FEED_CONFIG_1,
    core::ptr::null_mut(), T4_I2C_ABS, false);
    if (ret < 0) {
    dev_err(&hdev.dev, "failed T4_PRM_FEED_CONFIG_1 (%d)\n", ret);
    goto exit;
    }
    ret = t4_read_write_register(hdev, T4_PRM_FEED_CONFIG_4, core::ptr::null_mut(),
    T4_FEEDCFG4_ADVANCED_ABS_ENABLE, false);
    if (ret < 0) {
    dev_err(&hdev.dev, "failed T4_PRM_FEED_CONFIG_4 (%d)\n", ret);
    goto exit;
    }
    pri_data.max_fingers = 5;
    pri_data.has_sp = 0;
    exit:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn alps_sp_open(dev: *mut input_dev) -> c_int {
    static int alps_sp_open(struct input_dev *dev)
    {
    struct hid_device *hid = input_get_drvdata(dev);
    return hid_hw_open(hid);
    }
#[no_mangle]
unsafe extern "C" fn alps_sp_close(dev: *mut input_dev) {
    static void alps_sp_close(struct input_dev *dev)
    {
    struct hid_device *hid = input_get_drvdata(dev);
    hid_hw_close(hid);
    }
#[no_mangle]
unsafe extern "C" fn alps_input_configured(hdev: *mut hid_device, hi: *mut hid_input) -> c_int {
    static int alps_input_configured(struct hid_device *hdev, struct hid_input *hi)
    {
    struct alps_dev *data = hid_get_drvdata(hdev);
    struct input_dev *input = hi.input, *input2;
    int ret;
    int res_x, res_y, i;
    data.input = input;
    hid_dbg(hdev, "Opening low level driver\n");
    ret = hid_hw_open(hdev);
    if (ret)
    return ret;
// Allow incoming hid reports
    hid_device_io_start(hdev);
    switch (data.dev_type) {
    case T4:
    ret = T4_init(hdev, data);
    break;
    case U1:
    ret = u1_init(hdev, data);
    break;
    default:
    break;
    }
    if (ret)
    goto exit;
    __set_bit(EV_ABS, input.evbit);
    input_set_abs_params(input, ABS_MT_POSITION_X,
    data.x_min, data.x_max, 0, 0);
    input_set_abs_params(input, ABS_MT_POSITION_Y,
    data.y_min, data.y_max, 0, 0);
    if (data.x_active_len_mm && data.y_active_len_mm) {
    res_x = (data.x_max - 1) / data.x_active_len_mm;
    res_y = (data.y_max - 1) / data.y_active_len_mm;
    input_abs_set_res(input, ABS_MT_POSITION_X, res_x);
    input_abs_set_res(input, ABS_MT_POSITION_Y, res_y);
    }
    input_set_abs_params(input, ABS_MT_PRESSURE, 0, 64, 0, 0);
    input_mt_init_slots(input, data.max_fingers, INPUT_MT_POINTER);
    __set_bit(EV_KEY, input.evbit);
    if (data.btn_cnt == 1)
    __set_bit(INPUT_PROP_BUTTONPAD, input.propbit);
    for (i = 0; i < data.btn_cnt; i++)
    __set_bit(BTN_LEFT + i, input.keybit);
// Stick device initialization
    if (data.has_sp) {
    input2 = input_allocate_device();
    if (!input2) {
    ret = -ENOMEM;
    goto exit;
    }
    data.input2 = input2;
    input2.phys = input.phys;
    input2.name = "DualPoint Stick";
    input2.id.bustype = BUS_I2C;
    input2.id.vendor  = input.id.vendor;
    input2.id.product = input.id.product;
    input2.id.version = input.id.version;
    input2.dev.parent = input.dev.parent;
    input_set_drvdata(input2, hdev);
    input2.open = alps_sp_open;
    input2.close = alps_sp_close;
    __set_bit(EV_KEY, input2.evbit);
    data.sp_btn_cnt = (data.sp_btn_info & 0x0F);
    for (i = 0; i < data.sp_btn_cnt; i++)
    __set_bit(BTN_LEFT + i, input2.keybit);
    __set_bit(EV_REL, input2.evbit);
    __set_bit(REL_X, input2.relbit);
    __set_bit(REL_Y, input2.relbit);
    __set_bit(INPUT_PROP_POINTER, input2.propbit);
    __set_bit(INPUT_PROP_POINTING_STICK, input2.propbit);
    if (input_register_device(data.input2)) {
    input_free_device(input2);
    ret = -ENOENT;
    goto exit;
    }
    }
    exit:
    hid_device_io_stop(hdev);
    hid_hw_close(hdev);
    return ret;
    }
    static int alps_input_mapping(struct hid_device *hdev,
    struct hid_input *hi, struct hid_field *field,
    struct hid_usage *usage, unsigned long **bit, int *max)
    {
    return -1;
    }
#[no_mangle]
unsafe extern "C" fn alps_probe(hdev: *mut hid_device, id: *const hid_device_id) -> c_int {
    static int alps_probe(struct hid_device *hdev, const struct hid_device_id *id)
    {
    struct alps_dev *data = core::ptr::null_mut();
    int ret;
    data = devm_kzalloc(&hdev.dev, sizeof(struct alps_dev), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    data.hdev = hdev;
    hid_set_drvdata(hdev, data);
    hdev.quirks |= HID_QUIRK_NO_INIT_REPORTS;
    ret = hid_parse(hdev);
    if (ret) {
    hid_err(hdev, "parse failed\n");
    return ret;
    }
    switch (hdev.product) {
    case HID_DEVICE_ID_ALPS_T4_BTNLESS:
    data.dev_type = T4;
    break;
    case HID_DEVICE_ID_ALPS_U1_DUAL:
    case HID_DEVICE_ID_ALPS_U1:
    case HID_DEVICE_ID_ALPS_U1_UNICORN_LEGACY:
    data.dev_type = U1;
    break;
    default:
    data.dev_type = UNKNOWN;
    }
    ret = hid_hw_start(hdev, HID_CONNECT_DEFAULT);
    if (ret) {
    hid_err(hdev, "hw start failed\n");
    return ret;
    }
    return 0;
    }
    static const struct hid_device_id alps_id[] = {
    { HID_DEVICE(HID_BUS_ANY, HID_GROUP_ANY,
    USB_VENDOR_ID_ALPS_JP, HID_DEVICE_ID_ALPS_U1_DUAL) },
    { HID_DEVICE(HID_BUS_ANY, HID_GROUP_ANY,
    USB_VENDOR_ID_ALPS_JP, HID_DEVICE_ID_ALPS_U1) },
    { HID_DEVICE(HID_BUS_ANY, HID_GROUP_ANY,
    USB_VENDOR_ID_ALPS_JP, HID_DEVICE_ID_ALPS_U1_UNICORN_LEGACY) },
    { HID_DEVICE(HID_BUS_ANY, HID_GROUP_ANY,
    USB_VENDOR_ID_ALPS_JP, HID_DEVICE_ID_ALPS_T4_BTNLESS) },
    { }
    };
    MODULE_DEVICE_TABLE(hid, alps_id);
    static struct hid_driver alps_driver = {
    .name = "hid-alps",
    .id_table		= alps_id,
    .probe			= alps_probe,
    .raw_event		= alps_raw_event,
    .input_mapping		= alps_input_mapping,
    .input_configured	= alps_input_configured,
    .resume			= pm_ptr(alps_post_resume),
    .reset_resume		= pm_ptr(alps_post_reset),
    };
    module_hid_driver(alps_driver);
    MODULE_AUTHOR("Masaki Ota <masaki.ota@jp.alps.com>");
    MODULE_DESCRIPTION("ALPS HID driver");
    MODULE_LICENSE("GPL");
