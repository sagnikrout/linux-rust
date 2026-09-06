//! Automatically rewritten from C to Rust
//! Source: drivers/w1/slaves/w1_ds2438.c
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
// 1-Wire implementation for the ds2438 chip
//
// Copyright (c) 2017 Mariusz Bialonczyk <manio@skyboo.net>
//

pub const W1_FAMILY_DS2438: c_uint = 0x26;
pub const W1_DS2438_RETRIES: c_int = 3;
// Memory commands
pub const W1_DS2438_READ_SCRATCH: c_uint = 0xBE;
pub const W1_DS2438_WRITE_SCRATCH: c_uint = 0x4E;
pub const W1_DS2438_COPY_SCRATCH: c_uint = 0x48;
pub const W1_DS2438_RECALL_MEMORY: c_uint = 0xB8;
// Register commands
pub const W1_DS2438_CONVERT_TEMP: c_uint = 0x44;
pub const W1_DS2438_CONVERT_VOLTAGE: c_uint = 0xB4;
pub const DS2438_PAGE_SIZE: c_int = 8;
pub const DS2438_ADC_INPUT_VAD: c_int = 0;
pub const DS2438_ADC_INPUT_VDD: c_int = 1;

// Page #0 definitions
pub const DS2438_STATUS_REG: c_uint = 0x00		/* Status/Configuration Register */;

pub const DS2438_TEMP_LSB: c_uint = 0x01;
pub const DS2438_TEMP_MSB: c_uint = 0x02;
pub const DS2438_VOLTAGE_LSB: c_uint = 0x03;
pub const DS2438_VOLTAGE_MSB: c_uint = 0x04;
pub const DS2438_CURRENT_LSB: c_uint = 0x05;
pub const DS2438_CURRENT_MSB: c_uint = 0x06;
pub const DS2438_THRESHOLD: c_uint = 0x07;
// Page #1 definitions
pub const DS2438_ETM_0: c_uint = 0x00;
pub const DS2438_ETM_1: c_uint = 0x01;
pub const DS2438_ETM_2: c_uint = 0x02;
pub const DS2438_ETM_3: c_uint = 0x03;
pub const DS2438_ICA: c_uint = 0x04;
pub const DS2438_OFFSET_LSB: c_uint = 0x05;
pub const DS2438_OFFSET_MSB: c_uint = 0x06;
#[no_mangle]
unsafe extern "C" fn w1_ds2438_get_page(sl: *mut w1_slave, pageno: c_int, buf: *mut u8) -> c_int {
    static int w1_ds2438_get_page(struct w1_slave *sl, int pageno, u8 *buf)
    {
    let mut retries: c_uint = W1_DS2438_RETRIES;
    u8 w1_buf[2];
    u8 crc;
    size_t count;
    while (retries--) {
    if (w1_reset_select_slave(sl))
    continue;
    w1_buf[0] = W1_DS2438_RECALL_MEMORY;
    w1_buf[1] = (u8)pageno;
    w1_write_block(sl.master, w1_buf, 2);
    if (w1_reset_select_slave(sl))
    continue;
    w1_buf[0] = W1_DS2438_READ_SCRATCH;
    w1_buf[1] = (u8)pageno;
    w1_write_block(sl.master, w1_buf, 2);
    count = w1_read_block(sl.master, buf, DS2438_PAGE_SIZE + 1);
    if (count == DS2438_PAGE_SIZE + 1) {
    crc = w1_calc_crc8(buf, DS2438_PAGE_SIZE);
// check for correct CRC
    if ((u8)buf[DS2438_PAGE_SIZE] == crc)
    return 0;
    }
    }
    return -1;
    }
#[no_mangle]
unsafe extern "C" fn w1_ds2438_get_temperature(sl: *mut w1_slave, temperature: *mut i16) -> c_int {
    static int w1_ds2438_get_temperature(struct w1_slave *sl, int16_t *temperature)
    {
    let mut retries: c_uint = W1_DS2438_RETRIES;
    u8 w1_buf[DS2438_PAGE_SIZE + 1 /*for CRC*/];
    let mut tm: c_uint = DS2438_MAX_CONVERSION_TIME;
    unsigned long sleep_rem;
    int ret;
    mutex_lock(&sl.master.bus_mutex);
    while (retries--) {
    if (w1_reset_select_slave(sl))
    continue;
    w1_write_8(sl.master, W1_DS2438_CONVERT_TEMP);
    mutex_unlock(&sl.master.bus_mutex);
    sleep_rem = msleep_interruptible(tm);
    if (sleep_rem != 0) {
    ret = -1;
    goto post_unlock;
    }
    if (mutex_lock_interruptible(&sl.master.bus_mutex) != 0) {
    ret = -1;
    goto post_unlock;
    }
    break;
    }
    if (w1_ds2438_get_page(sl, 0, w1_buf) == 0) {
// temperature = (((int16_t) w1_buf[DS2438_TEMP_MSB]) << 8) | ((uint16_t) w1_buf[DS2438_TEMP_LSB]);
    ret = 0;
    } else
    ret = -1;
    mutex_unlock(&sl.master.bus_mutex);
    post_unlock:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn w1_ds2438_change_config_bit(sl: *mut w1_slave, mask: u8, value: u8) -> c_int {
    static int w1_ds2438_change_config_bit(struct w1_slave *sl, u8 mask, u8 value)
    {
    let mut retries: c_uint = W1_DS2438_RETRIES;
    u8 w1_buf[3];
    u8 status;
    let mut perform_write: c_int = 0;
    while (retries--) {
    if (w1_reset_select_slave(sl))
    continue;
    w1_buf[0] = W1_DS2438_RECALL_MEMORY;
    w1_buf[1] = 0x00;
    w1_write_block(sl.master, w1_buf, 2);
    if (w1_reset_select_slave(sl))
    continue;
    w1_buf[0] = W1_DS2438_READ_SCRATCH;
    w1_buf[1] = 0x00;
    w1_write_block(sl.master, w1_buf, 2);
// reading one byte of result
    status = w1_read_8(sl.master);
// if bit0=1, set a value to a mask for easy compare
    if (value)
    value = mask;
    if ((status & mask) == value)
    return 0;	/* already set as requested */
// changing bit
    status ^= mask;
    perform_write = 1;
    break;
    }
    if (perform_write) {
    retries = W1_DS2438_RETRIES;
    while (retries--) {
    if (w1_reset_select_slave(sl))
    continue;
    w1_buf[0] = W1_DS2438_WRITE_SCRATCH;
    w1_buf[1] = 0x00;
    w1_buf[2] = status;
    w1_write_block(sl.master, w1_buf, 3);
    if (w1_reset_select_slave(sl))
    continue;
    w1_buf[0] = W1_DS2438_COPY_SCRATCH;
    w1_buf[1] = 0x00;
    w1_write_block(sl.master, w1_buf, 2);
    return 0;
    }
    }
    return -1;
    }
#[no_mangle]
unsafe extern "C" fn w1_ds2438_change_offset_register(sl: *mut w1_slave, value: *mut u8) -> c_int {
    static int w1_ds2438_change_offset_register(struct w1_slave *sl, u8 *value)
    {
    let mut retries: c_uint = W1_DS2438_RETRIES;
    u8 w1_buf[9];
    u8 w1_page1_buf[DS2438_PAGE_SIZE + 1 /*for CRC*/];
    if (w1_ds2438_get_page(sl, 1, w1_page1_buf) == 0) {
    memcpy(&w1_buf[2], w1_page1_buf, DS2438_PAGE_SIZE - 1); /* last register reserved */
    w1_buf[7] = value[0]; /* change only offset register */
    w1_buf[8] = value[1];
    while (retries--) {
    if (w1_reset_select_slave(sl))
    continue;
    w1_buf[0] = W1_DS2438_WRITE_SCRATCH;
    w1_buf[1] = 0x01; /* write to page 1 */
    w1_write_block(sl.master, w1_buf, 9);
    if (w1_reset_select_slave(sl))
    continue;
    w1_buf[0] = W1_DS2438_COPY_SCRATCH;
    w1_buf[1] = 0x01;
    w1_write_block(sl.master, w1_buf, 2);
    return 0;
    }
    }
    return -1;
    }
    static int w1_ds2438_get_voltage(struct w1_slave *sl,
    int adc_input, uint16_t *voltage)
    {
    let mut retries: c_uint = W1_DS2438_RETRIES;
    u8 w1_buf[DS2438_PAGE_SIZE + 1 /*for CRC*/];
    let mut tm: c_uint = DS2438_MAX_CONVERSION_TIME;
    unsigned long sleep_rem;
    int ret;
    mutex_lock(&sl.master.bus_mutex);
    if (w1_ds2438_change_config_bit(sl, DS2438_STATUS_AD, adc_input)) {
    ret = -1;
    goto pre_unlock;
    }
    while (retries--) {
    if (w1_reset_select_slave(sl))
    continue;
    w1_write_8(sl.master, W1_DS2438_CONVERT_VOLTAGE);
    mutex_unlock(&sl.master.bus_mutex);
    sleep_rem = msleep_interruptible(tm);
    if (sleep_rem != 0) {
    ret = -1;
    goto post_unlock;
    }
    if (mutex_lock_interruptible(&sl.master.bus_mutex) != 0) {
    ret = -1;
    goto post_unlock;
    }
    break;
    }
    if (w1_ds2438_get_page(sl, 0, w1_buf) == 0) {
// voltage = (((uint16_t) w1_buf[DS2438_VOLTAGE_MSB]) << 8) | ((uint16_t) w1_buf[DS2438_VOLTAGE_LSB]);
    ret = 0;
    } else
    ret = -1;
    pre_unlock:
    mutex_unlock(&sl.master.bus_mutex);
    post_unlock:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn w1_ds2438_get_current(sl: *mut w1_slave, voltage: *mut i16) -> c_int {
    static int w1_ds2438_get_current(struct w1_slave *sl, int16_t *voltage)
    {
    u8 w1_buf[DS2438_PAGE_SIZE + 1 /*for CRC*/];
    int ret;
    mutex_lock(&sl.master.bus_mutex);
    if (w1_ds2438_get_page(sl, 0, w1_buf) == 0) {
// The voltage measured across current sense resistor RSENS.
// voltage = (((int16_t) w1_buf[DS2438_CURRENT_MSB]) << 8) | ((int16_t) w1_buf[DS2438_CURRENT_LSB]);
    ret = 0;
    } else
    ret = -1;
    mutex_unlock(&sl.master.bus_mutex);
    return ret;
    }
    static ssize_t iad_write(struct file *filp, struct kobject *kobj,
    const struct bin_attribute *bin_attr, char *buf,
    loff_t off, size_t count)
    {
    struct w1_slave *sl = kobj_to_w1_slave(kobj);
    int ret;
    if (count != 1 || off != 0)
    return -EFAULT;
    mutex_lock(&sl.master.bus_mutex);
    if (w1_ds2438_change_config_bit(sl, DS2438_STATUS_IAD, *buf & 0x01) == 0)
    ret = 1;
    else
    ret = -EIO;
    mutex_unlock(&sl.master.bus_mutex);
    return ret;
    }
    static ssize_t iad_read(struct file *filp, struct kobject *kobj,
    const struct bin_attribute *bin_attr, char *buf,
    loff_t off, size_t count)
    {
    struct w1_slave *sl = kobj_to_w1_slave(kobj);
    int ret;
    int16_t voltage;
    if (off != 0)
    return 0;
    if (!buf)
    return -EINVAL;
    if (w1_ds2438_get_current(sl, &voltage) == 0)
    ret = snprintf(buf, count, "%i\n", voltage);
    else
    ret = -EIO;
    return ret;
    }
    static ssize_t page0_read(struct file *filp, struct kobject *kobj,
    const struct bin_attribute *bin_attr, char *buf,
    loff_t off, size_t count)
    {
    struct w1_slave *sl = kobj_to_w1_slave(kobj);
    int ret;
    u8 w1_buf[DS2438_PAGE_SIZE + 1 /*for CRC*/];
    if (off != 0)
    return 0;
    if (!buf)
    return -EINVAL;
    mutex_lock(&sl.master.bus_mutex);
// Read no more than page0 size
    if (count > DS2438_PAGE_SIZE)
    count = DS2438_PAGE_SIZE;
    if (w1_ds2438_get_page(sl, 0, w1_buf) == 0) {
    memcpy(buf, &w1_buf, count);
    ret = count;
    } else
    ret = -EIO;
    mutex_unlock(&sl.master.bus_mutex);
    return ret;
    }
    static ssize_t page1_read(struct file *filp, struct kobject *kobj,
    const struct bin_attribute *bin_attr, char *buf,
    loff_t off, size_t count)
    {
    struct w1_slave *sl = kobj_to_w1_slave(kobj);
    int ret;
    u8 w1_buf[DS2438_PAGE_SIZE + 1 /*for CRC*/];
    if (off != 0)
    return 0;
    if (!buf)
    return -EINVAL;
    mutex_lock(&sl.master.bus_mutex);
// Read no more than page1 size
    if (count > DS2438_PAGE_SIZE)
    count = DS2438_PAGE_SIZE;
    if (w1_ds2438_get_page(sl, 1, w1_buf) == 0) {
    memcpy(buf, &w1_buf, count);
    ret = count;
    } else
    ret = -EIO;
    mutex_unlock(&sl.master.bus_mutex);
    return ret;
    }
    static ssize_t offset_write(struct file *filp, struct kobject *kobj,
    const struct bin_attribute *bin_attr, char *buf,
    loff_t off, size_t count)
    {
    struct w1_slave *sl = kobj_to_w1_slave(kobj);
    int ret;
    mutex_lock(&sl.master.bus_mutex);
    if (w1_ds2438_change_offset_register(sl, buf) == 0)
    ret = count;
    else
    ret = -EIO;
    mutex_unlock(&sl.master.bus_mutex);
    return ret;
    }
    static ssize_t temperature_read(struct file *filp, struct kobject *kobj,
    const struct bin_attribute *bin_attr, char *buf,
    loff_t off, size_t count)
    {
    struct w1_slave *sl = kobj_to_w1_slave(kobj);
    int ret;
    int16_t temp;
    if (off != 0)
    return 0;
    if (!buf)
    return -EINVAL;
    if (w1_ds2438_get_temperature(sl, &temp) == 0)
    ret = snprintf(buf, count, "%i\n", temp);
    else
    ret = -EIO;
    return ret;
    }
    static ssize_t vad_read(struct file *filp, struct kobject *kobj,
    const struct bin_attribute *bin_attr, char *buf,
    loff_t off, size_t count)
    {
    struct w1_slave *sl = kobj_to_w1_slave(kobj);
    int ret;
    uint16_t voltage;
    if (off != 0)
    return 0;
    if (!buf)
    return -EINVAL;
    if (w1_ds2438_get_voltage(sl, DS2438_ADC_INPUT_VAD, &voltage) == 0)
    ret = snprintf(buf, count, "%u\n", voltage);
    else
    ret = -EIO;
    return ret;
    }
    static ssize_t vdd_read(struct file *filp, struct kobject *kobj,
    const struct bin_attribute *bin_attr, char *buf,
    loff_t off, size_t count)
    {
    struct w1_slave *sl = kobj_to_w1_slave(kobj);
    int ret;
    uint16_t voltage;
    if (off != 0)
    return 0;
    if (!buf)
    return -EINVAL;
    if (w1_ds2438_get_voltage(sl, DS2438_ADC_INPUT_VDD, &voltage) == 0)
    ret = snprintf(buf, count, "%u\n", voltage);
    else
    ret = -EIO;
    return ret;
    }
    static const BIN_ATTR_RW(iad, 0);
    static const BIN_ATTR_RO(page0, DS2438_PAGE_SIZE);
    static const BIN_ATTR_RO(page1, DS2438_PAGE_SIZE);
    static const BIN_ATTR_WO(offset, 2);
    static const BIN_ATTR_RO(temperature, 0/* real length varies */);
    static const BIN_ATTR_RO(vad, 0/* real length varies */);
    static const BIN_ATTR_RO(vdd, 0/* real length varies */);
    static const struct bin_attribute *const w1_ds2438_bin_attrs[] = {
    &bin_attr_iad,
    &bin_attr_page0,
    &bin_attr_page1,
    &bin_attr_offset,
    &bin_attr_temperature,
    &bin_attr_vad,
    &bin_attr_vdd,
    core::ptr::null_mut(),
    };
    static const struct attribute_group w1_ds2438_group = {
    .bin_attrs = w1_ds2438_bin_attrs,
    };
    static const struct attribute_group *w1_ds2438_groups[] = {
    &w1_ds2438_group,
    core::ptr::null_mut(),
    };
    static const struct w1_family_ops w1_ds2438_fops = {
    .groups		= w1_ds2438_groups,
    };
    static struct w1_family w1_ds2438_family = {
    .fid = W1_FAMILY_DS2438,
    .fops = &w1_ds2438_fops,
    };
    module_w1_family(w1_ds2438_family);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Mariusz Bialonczyk <manio@skyboo.net>");
    MODULE_DESCRIPTION("1-wire driver for Maxim/Dallas DS2438 Smart Battery Monitor");
    MODULE_ALIAS("w1-family-" __stringify(W1_FAMILY_DS2438));
