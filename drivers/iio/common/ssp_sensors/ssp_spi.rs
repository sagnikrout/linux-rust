//! Automatically rewritten from C to Rust
//! Source: drivers/iio/common/ssp_sensors/ssp_spi.c
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
// Copyright (C) 2014, Samsung Electronics Co. Ltd. All Rights Reserved.
//

//
// SSP -> AP Instruction
// They tell what packet type can be expected. In the future there will
// be less of them. BYPASS means common sensor packets with accel, gyro,
// hrm etc. data. LIBRARY and META are mock-up's for now.
//
pub const SSP_MSG2AP_INST_BYPASS_DATA: c_uint = 0x37;
pub const SSP_MSG2AP_INST_LIBRARY_DATA: c_uint = 0x01;
pub const SSP_MSG2AP_INST_DEBUG_DATA: c_uint = 0x03;
pub const SSP_MSG2AP_INST_BIG_DATA: c_uint = 0x04;
pub const SSP_MSG2AP_INST_META_DATA: c_uint = 0x05;
pub const SSP_MSG2AP_INST_TIME_SYNC: c_uint = 0x06;
pub const SSP_MSG2AP_INST_RESET: c_uint = 0x07;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ssp_msg_header {
    pub cmd: u8,
    pub length: __le16,
    pub options: __le16,
    pub data: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ssp_msg {
    pub length: u16,
    pub options: u16,
    pub list: list_head,
    pub done: *mut completion,
    pub h: *mut ssp_msg_header,
    pub buffer: *mut c_char,
}

    static const int ssp_offset_map[SSP_SENSOR_MAX] = {
    [SSP_ACCELEROMETER_SENSOR] =		SSP_ACCELEROMETER_SIZE +
    SSP_TIME_SIZE,
    [SSP_GYROSCOPE_SENSOR] =		SSP_GYROSCOPE_SIZE +
    SSP_TIME_SIZE,
    [SSP_GEOMAGNETIC_UNCALIB_SENSOR] =	SSP_UNIMPLEMENTED,
    [SSP_GEOMAGNETIC_RAW] =			SSP_UNIMPLEMENTED,
    [SSP_GEOMAGNETIC_SENSOR] =		SSP_UNIMPLEMENTED,
    [SSP_PRESSURE_SENSOR] =			SSP_UNIMPLEMENTED,
    [SSP_GESTURE_SENSOR] =			SSP_UNIMPLEMENTED,
    [SSP_PROXIMITY_SENSOR] =		SSP_UNIMPLEMENTED,
    [SSP_TEMPERATURE_HUMIDITY_SENSOR] =	SSP_UNIMPLEMENTED,
    [SSP_LIGHT_SENSOR] =			SSP_UNIMPLEMENTED,
    [SSP_PROXIMITY_RAW] =			SSP_UNIMPLEMENTED,
    [SSP_ORIENTATION_SENSOR] =		SSP_UNIMPLEMENTED,
    [SSP_STEP_DETECTOR] =			SSP_UNIMPLEMENTED,
    [SSP_SIG_MOTION_SENSOR] =		SSP_UNIMPLEMENTED,
    [SSP_GYRO_UNCALIB_SENSOR] =		SSP_UNIMPLEMENTED,
    [SSP_GAME_ROTATION_VECTOR] =		SSP_UNIMPLEMENTED,
    [SSP_ROTATION_VECTOR] =			SSP_UNIMPLEMENTED,
    [SSP_STEP_COUNTER] =			SSP_UNIMPLEMENTED,
    [SSP_BIO_HRM_RAW] =			SSP_BIO_HRM_RAW_SIZE +
    SSP_TIME_SIZE,
    [SSP_BIO_HRM_RAW_FAC] =			SSP_BIO_HRM_RAW_FAC_SIZE +
    SSP_TIME_SIZE,
    [SSP_BIO_HRM_LIB] =			SSP_BIO_HRM_LIB_SIZE +
    SSP_TIME_SIZE,
    };

    static struct ssp_msg *ssp_create_msg(u8 cmd, u16 len, u16 opt, u32 data)
    {
    struct ssp_msg_header h;
    struct ssp_msg *msg;
    msg = kzalloc_obj(*msg);
    if (!msg)
    return core::ptr::null_mut();
    h.cmd = cmd;
    h.length = cpu_to_le16(len);
    h.options = cpu_to_le16(opt);
    h.data = cpu_to_le32(data);
    msg.buffer = kzalloc(SSP_HEADER_SIZE_ALIGNED + len,
    GFP_KERNEL | GFP_DMA);
    if (!msg.buffer) {
    kfree(msg);
    return core::ptr::null_mut();
    }
    msg.length = len;
    msg.options = opt;
    memcpy(msg.buffer, &h, SSP_HEADER_SIZE);
    return msg;
    }
//
// It is a bit heavy to do it this way but often the function is used to compose
// the message from smaller chunks which are placed on the stack.  Often the
// chunks are small so memcpy should be optimized.
//
    static inline void ssp_fill_buffer(struct ssp_msg *m, unsigned int offset,
    const void *src, unsigned int len)
    {
    memcpy(&m.buffer[SSP_HEADER_SIZE_ALIGNED + offset], src, len);
    }
    static inline void ssp_get_buffer(struct ssp_msg *m, unsigned int offset,
    void *dest, unsigned int len)
    {
    memcpy(dest, &m.buffer[SSP_HEADER_SIZE_ALIGNED + offset],  len);
    }

    ((m).buffer[SSP_HEADER_SIZE_ALIGNED + (index)])

    ((m).buffer[SSP_HEADER_SIZE_ALIGNED + (index)] = val)
#[no_mangle]
unsafe extern "C" fn ssp_clean_msg(m: *mut ssp_msg) {
    static void ssp_clean_msg(struct ssp_msg *m)
    {
    kfree(m.buffer);
    kfree(m);
    }
    static int ssp_print_mcu_debug(char *data_frame, int *data_index,
    int received_len)
    {
    let mut length: c_int = data_frame[(*data_index)++];
    if (length > received_len - *data_index || length <= 0) {
    ssp_dbg("[SSP]: MSG From MCU-invalid debug length(%d/%d)\n",
    length, received_len);
    return -EPROTO;
    }
    ssp_dbg("[SSP]: MSG From MCU - %s\n", &data_frame[*data_index]);
// data_index += length;
    return 0;
    }
//
// It was designed that way - additional lines to some kind of handshake,
// please do not ask why - only the firmware guy can know it.
//
#[no_mangle]
unsafe extern "C" fn ssp_check_lines(data: *mut ssp_data, state: bool) -> c_int {
    static int ssp_check_lines(struct ssp_data *data, bool state)
    {
    let mut delay_cnt: c_int = 0;
    gpiod_set_value_cansleep(data.ap_mcu_gpiod, state);
    while (gpiod_get_value_cansleep(data.mcu_ap_gpiod) != state) {
    usleep_range(3000, 3500);
    if (data.shut_down || delay_cnt++ > 500) {
    dev_err(SSP_DEV, "%s:timeout, hw ack wait fail %d\n",
    __func__, state);
    if (!state)
    gpiod_set_value_cansleep(data.ap_mcu_gpiod, 1);
    return -ETIMEDOUT;
    }
    }
    return 0;
    }
    static int ssp_do_transfer(struct ssp_data *data, struct ssp_msg *msg,
    struct completion *done, int timeout)
    {
    int status;
//
// check if this is a short one way message or the whole transfer has
// second part after an interrupt
//
    let mut use_no_irq: bool = msg.length == 0;
    if (data.shut_down)
    return -EPERM;
    msg.done = done;
    mutex_lock(&data.comm_lock);
    status = ssp_check_lines(data, false);
    if (status < 0)
    goto _error_locked;
    status = spi_write(data.spi, msg.buffer, SSP_HEADER_SIZE);
    if (status < 0) {
    gpiod_set_value_cansleep(data.ap_mcu_gpiod, 1);
    dev_err(SSP_DEV, "%s spi_write fail\n", __func__);
    goto _error_locked;
    }
    if (!use_no_irq) {
    mutex_lock(&data.pending_lock);
    list_add_tail(&msg.list, &data.pending_list);
    mutex_unlock(&data.pending_lock);
    }
    status = ssp_check_lines(data, true);
    if (status < 0) {
    if (!use_no_irq) {
    mutex_lock(&data.pending_lock);
    list_del(&msg.list);
    mutex_unlock(&data.pending_lock);
    }
    goto _error_locked;
    }
    mutex_unlock(&data.comm_lock);
    if (!use_no_irq && done)
    if (wait_for_completion_timeout(done,
    msecs_to_jiffies(timeout)) ==
    0) {
    mutex_lock(&data.pending_lock);
    list_del(&msg.list);
    mutex_unlock(&data.pending_lock);
    data.timeout_cnt++;
    return -ETIMEDOUT;
    }
    return 0;
    _error_locked:
    mutex_unlock(&data.comm_lock);
    data.timeout_cnt++;
    return status;
    }
    static inline int ssp_spi_sync_command(struct ssp_data *data,
    struct ssp_msg *msg)
    {
    return ssp_do_transfer(data, msg, core::ptr::null_mut(), 0);
    }
    static int ssp_spi_sync(struct ssp_data *data, struct ssp_msg *msg,
    int timeout)
    {
    DECLARE_COMPLETION_ONSTACK(done);
    if (WARN_ON(!msg.length))
    return -EPERM;
    return ssp_do_transfer(data, msg, &done, timeout);
    }
#[no_mangle]
unsafe extern "C" fn ssp_handle_big_data(data: *mut ssp_data, dataframe: *mut c_char, idx: *mut c_int) -> c_int {
    static int ssp_handle_big_data(struct ssp_data *data, char *dataframe, int *idx)
    {
// mock-up, it will be changed with adding another sensor types
// idx += 8;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ssp_parse_dataframe(data: *mut ssp_data, dataframe: *mut c_char, len: c_int) -> c_int {
    static int ssp_parse_dataframe(struct ssp_data *data, char *dataframe, int len)
    {
    int idx, sd;
    struct ssp_sensor_data *spd;
    struct iio_dev **indio_devs = data.sensor_devs;
    for (idx = 0; idx < len;) {
    switch (dataframe[idx++]) {
    case SSP_MSG2AP_INST_BYPASS_DATA:
    if (idx >= len)
    return -EPROTO;
    sd = dataframe[idx++];
    if (sd < 0 || sd >= SSP_SENSOR_MAX) {
    dev_err(SSP_DEV,
    "Mcu data frame1 error %d\n", sd);
    return -EPROTO;
    }
    if (indio_devs[sd]) {
    spd = iio_priv(indio_devs[sd]);
    if (spd.process_data) {
    if (idx >= len)
    return -EPROTO;
    spd.process_data(indio_devs[sd],
    &dataframe[idx],
    data.timestamp);
    }
    } else {
    dev_err(SSP_DEV, "no client for frame\n");
    }
    idx += ssp_offset_map[sd];
    break;
    case SSP_MSG2AP_INST_DEBUG_DATA:
    if (idx >= len)
    return -EPROTO;
    sd = ssp_print_mcu_debug(dataframe, &idx, len);
    if (sd) {
    dev_err(SSP_DEV,
    "Mcu data frame3 error %d\n", sd);
    return sd;
    }
    break;
    case SSP_MSG2AP_INST_LIBRARY_DATA:
    idx += len;
    break;
    case SSP_MSG2AP_INST_BIG_DATA:
    ssp_handle_big_data(data, dataframe, &idx);
    break;
    case SSP_MSG2AP_INST_TIME_SYNC:
    data.time_syncing = true;
    break;
    case SSP_MSG2AP_INST_RESET:
    ssp_queue_ssp_refresh_task(data, 0);
    break;
    }
    }
    if (data.time_syncing)
    data.timestamp = ktime_get_real_ns();
    return 0;
    }
// threaded irq
#[no_mangle]
pub unsafe extern "C" fn ssp_irq_msg(data: *mut ssp_data) -> c_int {
    int ssp_irq_msg(struct ssp_data *data)
    {
    char *buffer;
    u8 msg_type;
    int ret;
    u16 length, msg_options;
    struct ssp_msg *msg = core::ptr::null_mut(), *iter, *n;
    ret = spi_read(data.spi, data.header_buffer, SSP_HEADER_BUFFER_SIZE);
    if (ret < 0) {
    dev_err(SSP_DEV, "header read fail\n");
    return ret;
    }
    length = le16_to_cpu(data.header_buffer[1]);
    msg_options = le16_to_cpu(data.header_buffer[0]);
    if (length == 0) {
    dev_err(SSP_DEV, "length received from mcu is 0\n");
    return -EINVAL;
    }
    msg_type = SSP_GET_MESSAGE_TYPE(msg_options);
    switch (msg_type) {
    case SSP_AP2HUB_READ:
    case SSP_AP2HUB_WRITE:
//
// this is a small list, a few elements - the packets can be
// received with no order
//
    mutex_lock(&data.pending_lock);
    list_for_each_entry_safe(iter, n, &data.pending_list, list) {
    if (iter.options == msg_options) {
    list_del(&iter.list);
    msg = iter;
    break;
    }
    }
    if (!msg) {
//
// here can be implemented dead messages handling
// but the slave should not send such ones - it is to
// check but let's handle this
//
    buffer = kmalloc(length, GFP_KERNEL | GFP_DMA);
    if (!buffer) {
    ret = -ENOMEM;
    goto _unlock;
    }
// got dead packet so it is always an error
    ret = spi_read(data.spi, buffer, length);
    if (ret >= 0)
    ret = -EPROTO;
    kfree(buffer);
    dev_err(SSP_DEV, "No match error %x\n",
    msg_options);
    goto _unlock;
    }
    if (msg_type == SSP_AP2HUB_READ)
    ret = spi_read(data.spi,
    &msg.buffer[SSP_HEADER_SIZE_ALIGNED],
    msg.length);
    if (msg_type == SSP_AP2HUB_WRITE) {
    ret = spi_write(data.spi,
    &msg.buffer[SSP_HEADER_SIZE_ALIGNED],
    msg.length);
    if (msg_options & SSP_AP2HUB_RETURN) {
    msg.options =
    SSP_AP2HUB_READ | SSP_AP2HUB_RETURN;
    msg.length = 1;
    list_add_tail(&msg.list, &data.pending_list);
    goto _unlock;
    }
    }
    if (msg.done)
    if (!completion_done(msg.done))
    complete(msg.done);
    _unlock:
    mutex_unlock(&data.pending_lock);
    break;
    case SSP_HUB2AP_WRITE:
    buffer = kzalloc(length, GFP_KERNEL | GFP_DMA);
    if (!buffer)
    return -ENOMEM;
    ret = spi_read(data.spi, buffer, length);
    if (ret < 0) {
    dev_err(SSP_DEV, "spi read fail\n");
    kfree(buffer);
    break;
    }
    ret = ssp_parse_dataframe(data, buffer, length);
    kfree(buffer);
    break;
    default:
    dev_err(SSP_DEV, "unknown msg type\n");
    return -EPROTO;
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn ssp_clean_pending_list(data: *mut ssp_data) {
    void ssp_clean_pending_list(struct ssp_data *data)
    {
    struct ssp_msg *msg, *n;
    mutex_lock(&data.pending_lock);
    list_for_each_entry_safe(msg, n, &data.pending_list, list) {
    list_del(&msg.list);
    if (msg.done)
    if (!completion_done(msg.done))
    complete(msg.done);
    }
    mutex_unlock(&data.pending_lock);
    }
#[no_mangle]
pub unsafe extern "C" fn ssp_command(data: *mut ssp_data, command: c_char, arg: c_int) -> c_int {
    int ssp_command(struct ssp_data *data, char command, int arg)
    {
    int ret;
    struct ssp_msg *msg;
    msg = ssp_create_msg(command, 0, SSP_AP2HUB_WRITE, arg);
    if (!msg)
    return -ENOMEM;
    ssp_dbg("%s - command 0x%x %d\n", __func__, command, arg);
    ret = ssp_spi_sync_command(data, msg);
    ssp_clean_msg(msg);
    return ret;
    }
    int ssp_send_instruction(struct ssp_data *data, u8 inst, u8 sensor_type,
    u8 *send_buf, u8 length)
    {
    int ret;
    struct ssp_msg *msg;
    if (data.fw_dl_state == SSP_FW_DL_STATE_DOWNLOADING) {
    dev_err(SSP_DEV, "%s - Skip Inst! DL state = %d\n",
    __func__, data.fw_dl_state);
    return -EBUSY;
    } else if (!(data.available_sensors & BIT(sensor_type)) &&
    (inst <= SSP_MSG2SSP_INST_CHANGE_DELAY)) {
    dev_err(SSP_DEV, "%s - Bypass Inst Skip! - %u\n",
    __func__, sensor_type);
    return -EIO; /* just fail */
    }
    msg = ssp_create_msg(inst, length + 2, SSP_AP2HUB_WRITE, 0);
    if (!msg)
    return -ENOMEM;
    ssp_fill_buffer(msg, 0, &sensor_type, 1);
    ssp_fill_buffer(msg, 1, send_buf, length);
    ssp_dbg("%s - Inst = 0x%x, Sensor Type = 0x%x, data = %u\n",
    __func__, inst, sensor_type, send_buf[1]);
    ret = ssp_spi_sync(data, msg, 1000);
    ssp_clean_msg(msg);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn ssp_get_chipid(data: *mut ssp_data) -> c_int {
    int ssp_get_chipid(struct ssp_data *data)
    {
    int ret;
    char buffer;
    struct ssp_msg *msg;
    msg = ssp_create_msg(SSP_MSG2SSP_AP_WHOAMI, 1, SSP_AP2HUB_READ, 0);
    if (!msg)
    return -ENOMEM;
    ret = ssp_spi_sync(data, msg, 1000);
    buffer = SSP_GET_BUFFER_AT_INDEX(msg, 0);
    ssp_clean_msg(msg);
    return ret < 0 ? ret : buffer;
    }
#[no_mangle]
pub unsafe extern "C" fn ssp_set_magnetic_matrix(data: *mut ssp_data) -> c_int {
    int ssp_set_magnetic_matrix(struct ssp_data *data)
    {
    int ret;
    struct ssp_msg *msg;
    msg = ssp_create_msg(SSP_MSG2SSP_AP_SET_MAGNETIC_STATIC_MATRIX,
    data.sensorhub_info.mag_length, SSP_AP2HUB_WRITE,
    0);
    if (!msg)
    return -ENOMEM;
    ssp_fill_buffer(msg, 0, data.sensorhub_info.mag_table,
    data.sensorhub_info.mag_length);
    ret = ssp_spi_sync(data, msg, 1000);
    ssp_clean_msg(msg);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn ssp_get_sensor_scanning_info(data: *mut ssp_data) -> c_uint {
    unsigned int ssp_get_sensor_scanning_info(struct ssp_data *data)
    {
    int ret;
    __le32 result;
    let mut cpu_result: u32 = 0;
    struct ssp_msg *msg = ssp_create_msg(SSP_MSG2SSP_AP_SENSOR_SCANNING, 4,
    SSP_AP2HUB_READ, 0);
    if (!msg)
    return 0;
    ret = ssp_spi_sync(data, msg, 1000);
    if (ret < 0) {
    dev_err(SSP_DEV, "%s - spi read fail %d\n", __func__, ret);
    goto _exit;
    }
    ssp_get_buffer(msg, 0, &result, 4);
    cpu_result = le32_to_cpu(result);
    dev_info(SSP_DEV, "%s state: 0x%08x\n", __func__, cpu_result);
    _exit:
    ssp_clean_msg(msg);
    return cpu_result;
    }
#[no_mangle]
pub unsafe extern "C" fn ssp_get_firmware_rev(data: *mut ssp_data) -> c_uint {
    unsigned int ssp_get_firmware_rev(struct ssp_data *data)
    {
    int ret;
    __le32 result;
    struct ssp_msg *msg = ssp_create_msg(SSP_MSG2SSP_AP_FIRMWARE_REV, 4,
    SSP_AP2HUB_READ, 0);
    if (!msg)
    return SSP_INVALID_REVISION;
    ret = ssp_spi_sync(data, msg, 1000);
    if (ret < 0) {
    dev_err(SSP_DEV, "%s - transfer fail %d\n", __func__, ret);
    ret = SSP_INVALID_REVISION;
    goto _exit;
    }
    ssp_get_buffer(msg, 0, &result, 4);
    ret = le32_to_cpu(result);
    _exit:
    ssp_clean_msg(msg);
    return ret;
    }
