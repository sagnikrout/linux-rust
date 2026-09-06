//! Automatically rewritten from C to Rust
//! Source: drivers/input/touchscreen/goodix_berlin_core.c
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
// Goodix "Berlin" Touchscreen IC driver
// Copyright (C) 2020 - 2021 Goodix, Inc.
// Copyright (C) 2023 Linaro Ltd.
//
// Based on goodix_ts_berlin driver.
//
// This driver is distinct from goodix.c since hardware interface
// is different enough to require a new driver.
// None of the register address or data structure are close enough
// to the previous generations.
//
// Currently the driver only handles Multitouch events with already
// programmed firmware and "config" for "Revision A/D" Berlin IC.
//
// Support is missing for:
// - ESD Management
// - Firmware update/flashing
// - "Config" update/flashing
// - Stylus Events
// - Gesture Events
// - Support for revision B
//

pub const GOODIX_BERLIN_MAX_TOUCH: c_int = 10;
pub const GOODIX_BERLIN_NORMAL_RESET_DELAY_MS: c_int = 100;

pub const GOODIX_BERLIN_REQUEST_CODE_RESET: c_int = 3;

pub const GOODIX_BERLIN_POINT_TYPE_STYLUS_HOVER: c_int = 1;
pub const GOODIX_BERLIN_POINT_TYPE_STYLUS: c_int = 3;

pub const GOODIX_BERLIN_DEV_CONFIRM_VAL: c_uint = 0xAA;
pub const GOODIX_BERLIN_BOOTOPTION_ADDR: c_uint = 0x10000;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct goodix_berlin_fw_version {
    pub rom_pid: [u8; 6],
    pub rom_vid: [u8; 3],
    pub rom_vid_reserved: u8,
    pub patch_pid: [u8; 8],
    pub patch_vid: [u8; 4],
    pub patch_vid_reserved: u8,
    pub sensor_id: u8,
    pub reserved: [u8; 2],
    pub checksum: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct goodix_berlin_ic_info_version {
    pub info_customer_id: u8,
    pub info_version_id: u8,
    pub ic_die_id: u8,
    pub ic_version_id: u8,
    pub config_id: __le32,
    pub config_version: u8,
    pub frame_data_customer_id: u8,
    pub frame_data_version_id: u8,
    pub touch_data_customer_id: u8,
    pub touch_data_version_id: u8,
    pub reserved: [u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct goodix_berlin_ic_info_feature {
    pub freqhop_feature: __le16,
    pub calibration_feature: __le16,
    pub gesture_feature: __le16,
    pub side_touch_feature: __le16,
    pub stylus_feature: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct goodix_berlin_ic_info_misc {
    pub cmd_addr: __le32,
    pub cmd_max_len: __le16,
    pub cmd_reply_addr: __le32,
    pub cmd_reply_len: __le16,
    pub fw_state_addr: __le32,
    pub fw_state_len: __le16,
    pub fw_buffer_addr: __le32,
    pub fw_buffer_max_len: __le16,
    pub frame_data_addr: __le32,
    pub frame_data_head_len: __le16,
    pub fw_attr_len: __le16,
    pub fw_log_len: __le16,
    pub pack_max_num: u8,
    pub pack_compress_version: u8,
    pub stylus_struct_len: __le16,
    pub mutual_struct_len: __le16,
    pub self_struct_len: __le16,
    pub noise_struct_len: __le16,
    pub touch_data_addr: __le32,
    pub touch_data_head_len: __le16,
    pub point_struct_len: __le16,
    pub reserved1: __le16,
    pub reserved2: __le16,
    pub mutual_rawdata_addr: __le32,
    pub mutual_diffdata_addr: __le32,
    pub mutual_refdata_addr: __le32,
    pub self_rawdata_addr: __le32,
    pub self_diffdata_addr: __le32,
    pub self_refdata_addr: __le32,
    pub iq_rawdata_addr: __le32,
    pub iq_refdata_addr: __le32,
    pub im_rawdata_addr: __le32,
    pub im_readata_len: __le16,
    pub noise_rawdata_addr: __le32,
    pub noise_rawdata_len: __le16,
    pub stylus_rawdata_addr: __le32,
    pub stylus_rawdata_len: __le16,
    pub noise_data_addr: __le32,
    pub esd_addr: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct goodix_berlin_touch {
    pub status: u8,
    pub reserved: u8,
    pub x: __le16,
    pub y: __le16,
    pub w: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct goodix_berlin_header {
    pub status: u8,
    pub reserved1: u8,
    pub request_type: u8,
    pub reserved2: [u8; 3],
    pub checksum: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct goodix_berlin_event {
    pub hdr: goodix_berlin_header,
// The data below is u16/__le16 aligned
    u8 data[GOODIX_BERLIN_TOUCH_SIZE * GOODIX_BERLIN_MAX_TOUCH +
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct goodix_berlin_core {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub avdd: *mut regulator,
    pub vddio: *mut regulator,
    pub reset_gpio: *mut gpio_desc,
    pub props: touchscreen_properties,
    pub fw_version: goodix_berlin_fw_version,
    pub input_dev: *mut input_dev,
    pub irq: c_int,
// Runtime parameters extracted from IC_INFO buffer
    pub touch_data_addr: u32,
    pub ic_data: *const goodix_berlin_ic_data,
    pub event: goodix_berlin_event,
}

#[no_mangle]
unsafe extern "C" fn goodix_berlin_checksum_valid(data: *const u8, size: c_int) -> bool {
    static bool goodix_berlin_checksum_valid(const u8 *data, int size)
    {
    let mut cal_checksum: u32 = 0;
    u16 r_checksum;
    int i;
    if (size < GOODIX_BERLIN_CHECKSUM_SIZE)
    return false;
    for (i = 0; i < size - GOODIX_BERLIN_CHECKSUM_SIZE; i++)
    cal_checksum += data[i];
    r_checksum = get_unaligned_le16(&data[i]);
    return (u16)cal_checksum == r_checksum;
    }
    static bool goodix_berlin_is_dummy_data(struct goodix_berlin_core *cd,
    const u8 *data, int size)
    {
    int i;
//
// If the device is missing or doesn't respond the buffer
// could be filled with bus default line state, 0x00 or 0xff,
// so declare success the first time we encounter neither.
//
    for (i = 0; i < size; i++)
    if (data[i] > 0 && data[i] < 0xff)
    return false;
    return true;
    }
#[no_mangle]
unsafe extern "C" fn goodix_berlin_dev_confirm(cd: *mut goodix_berlin_core) -> c_int {
    static int goodix_berlin_dev_confirm(struct goodix_berlin_core *cd)
    {
    u8 tx_buf[8], rx_buf[8];
    let mut retry: c_int = 3;
    int error;
    memset(tx_buf, GOODIX_BERLIN_DEV_CONFIRM_VAL, sizeof(tx_buf));
    while (retry--) {
    error = regmap_raw_write(cd.regmap,
    GOODIX_BERLIN_BOOTOPTION_ADDR,
    tx_buf, sizeof(tx_buf));
    if (error)
    return error;
    error = regmap_raw_read(cd.regmap,
    GOODIX_BERLIN_BOOTOPTION_ADDR,
    rx_buf, sizeof(rx_buf));
    if (error)
    return error;
    if (!memcmp(tx_buf, rx_buf, sizeof(tx_buf)))
    return 0;
    usleep_range(5000, 5100);
    }
    dev_err(cd.dev, "device confirm failed, rx_buf: %*ph\n",
    (int)sizeof(rx_buf), rx_buf);
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn goodix_berlin_power_on(cd: *mut goodix_berlin_core) -> c_int {
    static int goodix_berlin_power_on(struct goodix_berlin_core *cd)
    {
    int error;
    error = regulator_enable(cd.vddio);
    if (error) {
    dev_err(cd.dev, "Failed to enable vddio: %d\n", error);
    return error;
    }
// Vendor waits 3ms for VDDIO to settle
    usleep_range(3000, 3100);
    error = regulator_enable(cd.avdd);
    if (error) {
    dev_err(cd.dev, "Failed to enable avdd: %d\n", error);
    goto err_vddio_disable;
    }
// Vendor waits 15ms for AVDD to settle
    usleep_range(15000, 15100);
    gpiod_set_value_cansleep(cd.reset_gpio, 0);
// Vendor waits 4ms for Firmware to initialize
    usleep_range(4000, 4100);
    error = goodix_berlin_dev_confirm(cd);
    if (error)
    goto err_dev_reset;
// Vendor waits 100ms for Firmware to fully boot
    msleep(GOODIX_BERLIN_NORMAL_RESET_DELAY_MS);
    return 0;
    err_dev_reset:
    gpiod_set_value_cansleep(cd.reset_gpio, 1);
    regulator_disable(cd.avdd);
    err_vddio_disable:
    regulator_disable(cd.vddio);
    return error;
    }
#[no_mangle]
unsafe extern "C" fn goodix_berlin_power_off(cd: *mut goodix_berlin_core) {
    static void goodix_berlin_power_off(struct goodix_berlin_core *cd)
    {
    gpiod_set_value_cansleep(cd.reset_gpio, 1);
    regulator_disable(cd.avdd);
    regulator_disable(cd.vddio);
    }
#[no_mangle]
unsafe extern "C" fn goodix_berlin_read_version(cd: *mut goodix_berlin_core) -> c_int {
    static int goodix_berlin_read_version(struct goodix_berlin_core *cd)
    {
    int error;
    error = regmap_raw_read(cd.regmap, cd.ic_data.fw_version_info_addr,
    &cd.fw_version, sizeof(cd.fw_version));
    if (error) {
    dev_err(cd.dev, "error reading fw version, %d\n", error);
    return error;
    }
    if (!goodix_berlin_checksum_valid((u8 *)&cd.fw_version,
    sizeof(cd.fw_version))) {
    dev_err(cd.dev, "invalid fw version: checksum error\n");
    return -EINVAL;
    }
    return 0;
    }
// Only extract necessary data for runtime
    static int goodix_berlin_parse_ic_info(struct goodix_berlin_core *cd,
    const u8 *data, u16 length)
    {
    struct goodix_berlin_ic_info_misc *misc;
    let mut offset: c_uint = 0;
    offset += sizeof(__le16); /* length */
    offset += sizeof(struct goodix_berlin_ic_info_version);
    offset += sizeof(struct goodix_berlin_ic_info_feature);
// IC_INFO Parameters, variable width structure
    offset += 4 * sizeof(u8); /* drv_num, sen_num, button_num, force_num */
    if (offset >= length)
    goto invalid_offset;

    do {						\
    u8 param_num = data[offset++];		\
    offset += param_num * sizeof(__le16);	\
    if (offset >= length)			\
    goto invalid_offset;		\
    } while (0)
    ADVANCE_LE16_PARAMS(); /* active_scan_rate_num */
    ADVANCE_LE16_PARAMS(); /* mutual_freq_num*/
    ADVANCE_LE16_PARAMS(); /* self_tx_freq_num */
    ADVANCE_LE16_PARAMS(); /* self_rx_freq_num */
    ADVANCE_LE16_PARAMS(); /* stylus_freq_num */

    misc = (struct goodix_berlin_ic_info_misc *)&data[offset];
    cd.touch_data_addr = le32_to_cpu(misc.touch_data_addr);
    return 0;
    invalid_offset:
    dev_err(cd.dev, "ic_info length is invalid (offset %d length %d)\n",
    offset, length);
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn goodix_berlin_get_ic_info(cd: *mut goodix_berlin_core) -> c_int {
    static int goodix_berlin_get_ic_info(struct goodix_berlin_core *cd)
    {
    u8 *afe_data __free(kfree) = core::ptr::null_mut();
    __le16 length_raw;
    u16 length;
    int error;
    afe_data = kzalloc(GOODIX_BERLIN_IC_INFO_MAX_LEN, GFP_KERNEL);
    if (!afe_data)
    return -ENOMEM;
    error = regmap_raw_read(cd.regmap, cd.ic_data.ic_info_addr,
    &length_raw, sizeof(length_raw));
    if (error) {
    dev_err(cd.dev, "failed get ic info length, %d\n", error);
    return error;
    }
    length = le16_to_cpu(length_raw);
    if (length >= GOODIX_BERLIN_IC_INFO_MAX_LEN) {
    dev_err(cd.dev, "invalid ic info length %d\n", length);
    return -EINVAL;
    }
    error = regmap_raw_read(cd.regmap, cd.ic_data.ic_info_addr, afe_data,
    length);
    if (error) {
    dev_err(cd.dev, "failed get ic info data, %d\n", error);
    return error;
    }
// check whether the data is valid (ex. bus default values)
    if (goodix_berlin_is_dummy_data(cd, afe_data, length)) {
    dev_err(cd.dev, "fw info data invalid\n");
    return -EINVAL;
    }
    if (!goodix_berlin_checksum_valid(afe_data, length)) {
    dev_err(cd.dev, "fw info checksum error\n");
    return -EINVAL;
    }
    error = goodix_berlin_parse_ic_info(cd, afe_data, length);
    if (error)
    return error;
// check some key info
    if (!cd.touch_data_addr) {
    dev_err(cd.dev, "touch_data_addr is null\n");
    return -EINVAL;
    }
    return 0;
    }
    static int goodix_berlin_get_remaining_contacts(struct goodix_berlin_core *cd,
    int n)
    {
    size_t offset = 2 * GOODIX_BERLIN_TOUCH_SIZE +
    GOODIX_BERLIN_CHECKSUM_SIZE;
    let mut addr: u32 = cd.touch_data_addr + GOODIX_BERLIN_HEADER_SIZE + offset;
    int error;
    error = regmap_raw_read(cd.regmap, addr,
    &cd.event.data[offset],
    (n - 2) * GOODIX_BERLIN_TOUCH_SIZE);
    if (error) {
    dev_err_ratelimited(cd.dev, "failed to get touch data, %d\n",
    error);
    return error;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn goodix_berlin_report_state(cd: *mut goodix_berlin_core, n: c_int) {
    static void goodix_berlin_report_state(struct goodix_berlin_core *cd, int n)
    {
    struct goodix_berlin_touch *touch_data =
    (struct goodix_berlin_touch *)cd.event.data;
    struct goodix_berlin_touch *t;
    int i;
    u8 type, id;
    for (i = 0; i < n; i++) {
    t = &touch_data[i];
    type = FIELD_GET(GOODIX_BERLIN_POINT_TYPE_MASK, t.status);
    if (type == GOODIX_BERLIN_POINT_TYPE_STYLUS ||
    type == GOODIX_BERLIN_POINT_TYPE_STYLUS_HOVER) {
    dev_warn_once(cd.dev, "Stylus event type not handled\n");
    continue;
    }
    id = FIELD_GET(GOODIX_BERLIN_TOUCH_ID_MASK, t.status);
    if (id >= GOODIX_BERLIN_MAX_TOUCH) {
    dev_warn_ratelimited(cd.dev, "invalid finger id %d\n", id);
    continue;
    }
    input_mt_slot(cd.input_dev, id);
    input_mt_report_slot_state(cd.input_dev, MT_TOOL_FINGER, true);
    touchscreen_report_pos(cd.input_dev, &cd.props,
    __le16_to_cpu(t.x), __le16_to_cpu(t.y),
    true);
    input_report_abs(cd.input_dev, ABS_MT_TOUCH_MAJOR,
    __le16_to_cpu(t.w));
    }
    input_mt_sync_frame(cd.input_dev);
    input_sync(cd.input_dev);
    }
#[no_mangle]
unsafe extern "C" fn goodix_berlin_touch_handler(cd: *mut goodix_berlin_core) {
    static void goodix_berlin_touch_handler(struct goodix_berlin_core *cd)
    {
    u8 touch_num;
    int error;
    touch_num = FIELD_GET(GOODIX_BERLIN_TOUCH_COUNT_MASK,
    cd.event.hdr.request_type);
    if (touch_num > GOODIX_BERLIN_MAX_TOUCH) {
    dev_warn(cd.dev, "invalid touch num %d\n", touch_num);
    return;
    }
    if (touch_num > 2) {
// read additional contact data if more than 2 touch events
    error = goodix_berlin_get_remaining_contacts(cd, touch_num);
    if (error)
    return;
    }
    if (touch_num) {
    int len = touch_num * GOODIX_BERLIN_TOUCH_SIZE +
    GOODIX_BERLIN_CHECKSUM_SIZE;
    if (!goodix_berlin_checksum_valid(cd.event.data, len)) {
    dev_err(cd.dev, "touch data checksum error: %*ph\n",
    len, cd.event.data);
    return;
    }
    }
    goodix_berlin_report_state(cd, touch_num);
    }
#[no_mangle]
unsafe extern "C" fn goodix_berlin_request_handle_reset(cd: *mut goodix_berlin_core) -> c_int {
    static int goodix_berlin_request_handle_reset(struct goodix_berlin_core *cd)
    {
    gpiod_set_value_cansleep(cd.reset_gpio, 1);
    usleep_range(2000, 2100);
    gpiod_set_value_cansleep(cd.reset_gpio, 0);
    msleep(GOODIX_BERLIN_NORMAL_RESET_DELAY_MS);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn goodix_berlin_irq(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t goodix_berlin_irq(int irq, void *data)
    {
    struct goodix_berlin_core *cd = data;
    int error;
//
// First, read buffer with space for 2 touch events:
// - GOODIX_BERLIN_HEADER_SIZE = 8 bytes
// - GOODIX_BERLIN_TOUCH_SIZE * 2 = 16 bytes
// - GOODIX_BERLIN_CHECKLSUM_SIZE = 2 bytes
// For a total of 26 bytes.
//
// If only a single finger is reported, we will read 8 bytes more than
// needed:
// - bytes 0-7:   Header (GOODIX_BERLIN_HEADER_SIZE)
// - bytes 8-15:  Finger 0 Data
// - bytes 24-25: Checksum
// - bytes 18-25: Unused 8 bytes
//
// If 2 fingers are reported, we would have read the exact needed
// amount of data and checksum would be at the end of the buffer:
// - bytes 0-7:   Header (GOODIX_BERLIN_HEADER_SIZE)
// - bytes 8-15:  Finger 0 Bytes 0-7
// - bytes 16-23: Finger 1 Bytes 0-7
// - bytes 24-25: Checksum
//
// If more than 2 fingers were reported, the "Checksum" bytes would
// in fact contain part of the next finger data, and then
// goodix_berlin_get_remaining_contacts() would complete the buffer
// with the missing bytes, including the trailing checksum.
// For example, if 3 fingers are reported, then we would do:
// Read 1:
// - bytes 0-7:   Header (GOODIX_BERLIN_HEADER_SIZE)
// - bytes 8-15:  Finger 0 Bytes 0-7
// - bytes 16-23: Finger 1 Bytes 0-7
// - bytes 24-25: Finger 2 Bytes 0-1
// Read 2 (with length of (3 - 2) * 8 = 8 bytes):
// - bytes 26-31: Finger 2 Bytes 2-7
// - bytes 32-33: Checksum
//
    error = regmap_raw_read(cd.regmap, cd.touch_data_addr,
    &cd.event,
    GOODIX_BERLIN_HEADER_SIZE +
    2 * GOODIX_BERLIN_TOUCH_SIZE +
    GOODIX_BERLIN_CHECKSUM_SIZE);
    if (error) {
    dev_warn_ratelimited(cd.dev,
    "failed get event head data: %d\n", error);
    goto out;
    }
    if (cd.event.hdr.status == 0)
    goto out;
    if (!goodix_berlin_checksum_valid((u8 *)&cd.event.hdr,
    GOODIX_BERLIN_HEADER_SIZE)) {
    dev_warn_ratelimited(cd.dev,
    "touch head checksum error: %*ph\n",
    (int)GOODIX_BERLIN_HEADER_SIZE,
    &cd.event.hdr);
    goto out_clear;
    }
    if (cd.event.hdr.status & GOODIX_BERLIN_TOUCH_EVENT)
    goodix_berlin_touch_handler(cd);
    if (cd.event.hdr.status & GOODIX_BERLIN_REQUEST_EVENT) {
    switch (cd.event.hdr.request_type) {
    case GOODIX_BERLIN_REQUEST_CODE_RESET:
    if (cd.reset_gpio)
    goodix_berlin_request_handle_reset(cd);
    break;
    default:
    dev_warn(cd.dev, "unsupported request code 0x%x\n",
    cd.event.hdr.request_type);
    }
    }
    out_clear:
// Clear up status field
    regmap_write(cd.regmap, cd.touch_data_addr, 0);
    out:
    return IRQ_HANDLED;
    }
    static int goodix_berlin_input_dev_config(struct goodix_berlin_core *cd,
    const struct input_id *id)
    {
    struct input_dev *input_dev;
    int error;
    input_dev = devm_input_allocate_device(cd.dev);
    if (!input_dev)
    return -ENOMEM;
    cd.input_dev = input_dev;
    input_set_drvdata(input_dev, cd);
    input_dev.name = "Goodix Berlin Capacitive TouchScreen";
    input_dev.phys = "input/ts";
    input_dev.id = *id;
    input_set_abs_params(cd.input_dev, ABS_MT_POSITION_X,
    0, SZ_64K - 1, 0, 0);
    input_set_abs_params(cd.input_dev, ABS_MT_POSITION_Y,
    0, SZ_64K - 1, 0, 0);
    input_set_abs_params(cd.input_dev, ABS_MT_TOUCH_MAJOR, 0, 255, 0, 0);
    touchscreen_parse_properties(cd.input_dev, true, &cd.props);
//
// The resolution of these touchscreens is about 10 units/mm, the actual
// resolution does not matter much since we set INPUT_PROP_DIRECT.
// Set it to 10 to ensure userspace isn't off by an order of magnitude.
//
    input_abs_set_res(cd.input_dev, ABS_MT_POSITION_X, 10);
    input_abs_set_res(cd.input_dev, ABS_MT_POSITION_Y, 10);
    error = input_mt_init_slots(cd.input_dev, GOODIX_BERLIN_MAX_TOUCH,
    INPUT_MT_DIRECT | INPUT_MT_DROP_UNUSED);
    if (error)
    return error;
    error = input_register_device(cd.input_dev);
    if (error)
    return error;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn goodix_berlin_suspend(dev: *mut device) -> c_int {
    static int goodix_berlin_suspend(struct device *dev)
    {
    struct goodix_berlin_core *cd = dev_get_drvdata(dev);
    disable_irq(cd.irq);
    goodix_berlin_power_off(cd);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn goodix_berlin_resume(dev: *mut device) -> c_int {
    static int goodix_berlin_resume(struct device *dev)
    {
    struct goodix_berlin_core *cd = dev_get_drvdata(dev);
    int error;
    error = goodix_berlin_power_on(cd);
    if (error)
    return error;
    enable_irq(cd.irq);
    return 0;
    }
    EXPORT_GPL_SIMPLE_DEV_PM_OPS(goodix_berlin_pm_ops,
    goodix_berlin_suspend, goodix_berlin_resume);
#[no_mangle]
unsafe extern "C" fn goodix_berlin_power_off_act(data: *mut c_void) {
    static void goodix_berlin_power_off_act(void *data)
    {
    struct goodix_berlin_core *cd = data;
    goodix_berlin_power_off(cd);
    }
    static ssize_t registers_read(struct file *filp, struct kobject *kobj,
    const struct bin_attribute *bin_attr,
    char *buf, loff_t off, size_t count)
    {
    struct device *dev = kobj_to_dev(kobj);
    struct goodix_berlin_core *cd = dev_get_drvdata(dev);
    int error;
    error = regmap_raw_read(cd.regmap, off, buf, count);
    return error ? error : count;
    }
    static ssize_t registers_write(struct file *filp, struct kobject *kobj,
    const struct bin_attribute *bin_attr,
    char *buf, loff_t off, size_t count)
    {
    struct device *dev = kobj_to_dev(kobj);
    struct goodix_berlin_core *cd = dev_get_drvdata(dev);
    int error;
    error = regmap_raw_write(cd.regmap, off, buf, count);
    return error ? error : count;
    }
    static const BIN_ATTR_ADMIN_RW(registers, 0);
    static const struct bin_attribute *const goodix_berlin_bin_attrs[] = {
    &bin_attr_registers,
    core::ptr::null_mut(),
    };
    static const struct attribute_group goodix_berlin_attr_group = {
    .bin_attrs = goodix_berlin_bin_attrs,
    };
    const struct attribute_group *goodix_berlin_groups[] = {
    &goodix_berlin_attr_group,
    core::ptr::null_mut(),
    };
    EXPORT_SYMBOL_GPL(goodix_berlin_groups);
    int goodix_berlin_probe(struct device *dev, int irq, const struct input_id *id,
    struct regmap *regmap,
    const struct goodix_berlin_ic_data *ic_data)
    {
    struct goodix_berlin_core *cd;
    int error;
    if (irq <= 0) {
    dev_err(dev, "Missing interrupt number\n");
    return -EINVAL;
    }
    cd = devm_kzalloc(dev, sizeof(*cd), GFP_KERNEL);
    if (!cd)
    return -ENOMEM;
    cd.dev = dev;
    cd.regmap = regmap;
    cd.irq = irq;
    cd.ic_data = ic_data;
    cd.reset_gpio = devm_gpiod_get_optional(dev, "reset", GPIOD_OUT_HIGH);
    if (IS_ERR(cd.reset_gpio))
    return dev_err_probe(dev, PTR_ERR(cd.reset_gpio),
    "Failed to request reset gpio\n");
    cd.avdd = devm_regulator_get(dev, "avdd");
    if (IS_ERR(cd.avdd))
    return dev_err_probe(dev, PTR_ERR(cd.avdd),
    "Failed to request avdd regulator\n");
    cd.vddio = devm_regulator_get(dev, "vddio");
    if (IS_ERR(cd.vddio))
    return dev_err_probe(dev, PTR_ERR(cd.vddio),
    "Failed to request vddio regulator\n");
    error = goodix_berlin_power_on(cd);
    if (error) {
    dev_err(dev, "failed power on");
    return error;
    }
    error = devm_add_action_or_reset(dev, goodix_berlin_power_off_act, cd);
    if (error)
    return error;
    error = goodix_berlin_read_version(cd);
    if (error) {
    dev_err(dev, "failed to get version info");
    return error;
    }
    error = goodix_berlin_get_ic_info(cd);
    if (error) {
    dev_err(dev, "invalid ic info, abort");
    return error;
    }
    error = goodix_berlin_input_dev_config(cd, id);
    if (error) {
    dev_err(dev, "failed set input device");
    return error;
    }
    error = devm_request_threaded_irq(dev, cd.irq, core::ptr::null_mut(), goodix_berlin_irq,
    IRQF_ONESHOT, "goodix-berlin", cd);
    if (error) {
    dev_err(dev, "request threaded irq failed: %d\n", error);
    return error;
    }
    dev_set_drvdata(dev, cd);
    dev_dbg(dev, "Goodix Berlin %s Touchscreen Controller",
    cd.fw_version.patch_pid);
    return 0;
    }
    EXPORT_SYMBOL_GPL(goodix_berlin_probe);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Goodix Berlin Core Touchscreen driver");
    MODULE_AUTHOR("Neil Armstrong <neil.armstrong@linaro.org>");
