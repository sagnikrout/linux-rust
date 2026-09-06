//! Automatically rewritten from C to Rust
//! Source: drivers/rtc/rtc-nvidia-vrs10.c
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
// NVIDIA Voltage Regulator Specification RTC
//
// SPDX-FileCopyrightText: Copyright (c) 2025 NVIDIA CORPORATION & AFFILIATES.
// All rights reserved.
//

pub const NVVRS_REG_VENDOR_ID: c_uint = 0x00;
pub const NVVRS_REG_MODEL_REV: c_uint = 0x01;
// Interrupts registers
pub const NVVRS_REG_INT_SRC1: c_uint = 0x10;
pub const NVVRS_REG_INT_SRC2: c_uint = 0x11;
pub const NVVRS_REG_INT_VENDOR: c_uint = 0x12;
// Control Registers
pub const NVVRS_REG_CTL_1: c_uint = 0x28;
pub const NVVRS_REG_CTL_2: c_uint = 0x29;
// RTC Registers
pub const NVVRS_REG_RTC_T3: c_uint = 0x70;
pub const NVVRS_REG_RTC_T2: c_uint = 0x71;
pub const NVVRS_REG_RTC_T1: c_uint = 0x72;
pub const NVVRS_REG_RTC_T0: c_uint = 0x73;
pub const NVVRS_REG_RTC_A3: c_uint = 0x74;
pub const NVVRS_REG_RTC_A2: c_uint = 0x75;
pub const NVVRS_REG_RTC_A1: c_uint = 0x76;
pub const NVVRS_REG_RTC_A0: c_uint = 0x77;
// Interrupt Mask

// Controller Register Mask

pub const NVVRS_REG_CTL_2_RST_DLY: c_uint = 0xF0;
pub const ALARM_RESET_VAL: c_uint = 0xffffffff;
pub const NVVRS_MIN_MODEL_REV: c_uint = 0x40;
    enum nvvrs_irq_regs {
    NVVRS_IRQ_REG_INT_SRC1 = 0,
    NVVRS_IRQ_REG_INT_SRC2 = 1,
    NVVRS_IRQ_REG_INT_VENDOR = 2,
    NVVRS_IRQ_REG_COUNT = 3,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvvrs_rtc_info {
    pub dev: *mut device,
    pub client: *mut i2c_client,
    pub rtc: *mut rtc_device,
    pub irq: c_uint,
}

    static int nvvrs_update_bits(struct nvvrs_rtc_info *info, u8 reg,
    u8 mask, u8 value)
    {
    int ret;
    u8 val;
    ret = i2c_smbus_read_byte_data(info.client, reg);
    if (ret < 0)
    return ret;
    val = (u8)ret;
    val &= ~mask;
    val |= (value & mask);
    return i2c_smbus_write_byte_data(info.client, reg, val);
    }
#[no_mangle]
unsafe extern "C" fn nvvrs_rtc_write_alarm(client: *mut i2c_client, time: *mut u8) -> c_int {
    static int nvvrs_rtc_write_alarm(struct i2c_client *client, u8 *time)
    {
    int ret;
    ret = i2c_smbus_write_byte_data(client, NVVRS_REG_RTC_A3, time[3]);
    if (ret < 0)
    return ret;
    ret = i2c_smbus_write_byte_data(client, NVVRS_REG_RTC_A2, time[2]);
    if (ret < 0)
    return ret;
    ret = i2c_smbus_write_byte_data(client, NVVRS_REG_RTC_A1, time[1]);
    if (ret < 0)
    return ret;
    return i2c_smbus_write_byte_data(client, NVVRS_REG_RTC_A0, time[0]);
    }
#[no_mangle]
unsafe extern "C" fn nvvrs_rtc_enable_alarm(info: *mut nvvrs_rtc_info) -> c_int {
    static int nvvrs_rtc_enable_alarm(struct nvvrs_rtc_info *info)
    {
    int ret;
// Set RTC_WAKE bit for autonomous wake from sleep
    ret = nvvrs_update_bits(info, NVVRS_REG_CTL_2, NVVRS_REG_CTL_2_RTC_WAKE,
    NVVRS_REG_CTL_2_RTC_WAKE);
    if (ret < 0)
    return ret;
// Set RTC_PU bit for autonomous wake from shutdown
    ret = nvvrs_update_bits(info, NVVRS_REG_CTL_2, NVVRS_REG_CTL_2_RTC_PU,
    NVVRS_REG_CTL_2_RTC_PU);
    if (ret < 0)
    return ret;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nvvrs_rtc_disable_alarm(info: *mut nvvrs_rtc_info) -> c_int {
    static int nvvrs_rtc_disable_alarm(struct nvvrs_rtc_info *info)
    {
    struct i2c_client *client = info.client;
    u8 val[4];
    int ret;
// Clear RTC_WAKE bit
    ret = nvvrs_update_bits(info, NVVRS_REG_CTL_2, NVVRS_REG_CTL_2_RTC_WAKE,
    0);
    if (ret < 0)
    return ret;
// Clear RTC_PU bit
    ret = nvvrs_update_bits(info, NVVRS_REG_CTL_2, NVVRS_REG_CTL_2_RTC_PU,
    0);
    if (ret < 0)
    return ret;
// Write ALARM_RESET_VAL in RTC Alarm register to disable alarm
    val[0] = 0xff;
    val[1] = 0xff;
    val[2] = 0xff;
    val[3] = 0xff;
    ret = nvvrs_rtc_write_alarm(client, val);
    if (ret < 0)
    return ret;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nvvrs_rtc_read_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int nvvrs_rtc_read_time(struct device *dev, struct rtc_time *tm)
    {
    struct nvvrs_rtc_info *info = dev_get_drvdata(dev);
    let mut secs: time64_t = 0;
    int ret;
    u8 val;
//
// Multi-byte transfers are not supported with PEC enabled
// Read MSB first to avoid coherency issues
//
    ret = i2c_smbus_read_byte_data(info.client, NVVRS_REG_RTC_T3);
    if (ret < 0)
    return ret;
    val = (u8)ret;
    secs |= (time64_t)val << 24;
    ret = i2c_smbus_read_byte_data(info.client, NVVRS_REG_RTC_T2);
    if (ret < 0)
    return ret;
    val = (u8)ret;
    secs |= (time64_t)val << 16;
    ret = i2c_smbus_read_byte_data(info.client, NVVRS_REG_RTC_T1);
    if (ret < 0)
    return ret;
    val = (u8)ret;
    secs |= (time64_t)val << 8;
    ret = i2c_smbus_read_byte_data(info.client, NVVRS_REG_RTC_T0);
    if (ret < 0)
    return ret;
    val = (u8)ret;
    secs |= val;
    rtc_time64_to_tm(secs, tm);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nvvrs_rtc_set_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int nvvrs_rtc_set_time(struct device *dev, struct rtc_time *tm)
    {
    struct nvvrs_rtc_info *info = dev_get_drvdata(dev);
    time64_t secs;
    u8 time[4];
    int ret;
    secs = rtc_tm_to_time64(tm);
    time[0] = secs & 0xff;
    time[1] = (secs >> 8) & 0xff;
    time[2] = (secs >> 16) & 0xff;
    time[3] = (secs >> 24) & 0xff;
    ret = i2c_smbus_write_byte_data(info.client, NVVRS_REG_RTC_T3, time[3]);
    if (ret < 0)
    return ret;
    ret = i2c_smbus_write_byte_data(info.client, NVVRS_REG_RTC_T2, time[2]);
    if (ret < 0)
    return ret;
    ret = i2c_smbus_write_byte_data(info.client, NVVRS_REG_RTC_T1, time[1]);
    if (ret < 0)
    return ret;
    ret = i2c_smbus_write_byte_data(info.client, NVVRS_REG_RTC_T0, time[0]);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn nvvrs_rtc_read_alarm(dev: *mut device, alrm: *mut rtc_wkalrm) -> c_int {
    static int nvvrs_rtc_read_alarm(struct device *dev, struct rtc_wkalrm *alrm)
    {
    struct nvvrs_rtc_info *info = dev_get_drvdata(dev);
    let mut alarm_val: time64_t = 0;
    int ret;
    u8 val;
// Multi-byte transfers are not supported with PEC enabled
    ret = i2c_smbus_read_byte_data(info.client, NVVRS_REG_RTC_A3);
    if (ret < 0)
    return ret;
    val = (u8)ret;
    alarm_val |= (time64_t)val << 24;
    ret = i2c_smbus_read_byte_data(info.client, NVVRS_REG_RTC_A2);
    if (ret < 0)
    return ret;
    val = (u8)ret;
    alarm_val |= (time64_t)val << 16;
    ret = i2c_smbus_read_byte_data(info.client, NVVRS_REG_RTC_A1);
    if (ret < 0)
    return ret;
    val = (u8)ret;
    alarm_val |= (time64_t)val << 8;
    ret = i2c_smbus_read_byte_data(info.client, NVVRS_REG_RTC_A0);
    if (ret < 0)
    return ret;
    val = (u8)ret;
    alarm_val |= val;
    if (alarm_val == ALARM_RESET_VAL)
    alrm.enabled = 0;
    else
    alrm.enabled = 1;
    rtc_time64_to_tm(alarm_val, &alrm.time);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nvvrs_rtc_set_alarm(dev: *mut device, alrm: *mut rtc_wkalrm) -> c_int {
    static int nvvrs_rtc_set_alarm(struct device *dev, struct rtc_wkalrm *alrm)
    {
    struct nvvrs_rtc_info *info = dev_get_drvdata(dev);
    time64_t secs;
    u8 time[4];
    int ret;
    if (!alrm.enabled) {
    ret = nvvrs_rtc_disable_alarm(info);
    if (ret < 0)
    return ret;
    }
    ret = nvvrs_rtc_enable_alarm(info);
    if (ret < 0)
    return ret;
    secs = rtc_tm_to_time64(&alrm.time);
    time[0] = secs & 0xff;
    time[1] = (secs >> 8) & 0xff;
    time[2] = (secs >> 16) & 0xff;
    time[3] = (secs >> 24) & 0xff;
    ret = nvvrs_rtc_write_alarm(info.client, time);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn nvvrs_pseq_irq_clear(info: *mut nvvrs_rtc_info) -> c_int {
    static int nvvrs_pseq_irq_clear(struct nvvrs_rtc_info *info)
    {
    unsigned int i;
    int ret;
    for (i = 0; i < NVVRS_IRQ_REG_COUNT; i++) {
    ret = i2c_smbus_read_byte_data(info.client,
    NVVRS_REG_INT_SRC1 + i);
    if (ret < 0) {
    dev_err(info.dev, "Failed to read INT_SRC%d : %d\n",
    i + 1, ret);
    return ret;
    }
    ret = i2c_smbus_write_byte_data(info.client,
    NVVRS_REG_INT_SRC1 + i,
    (u8)ret);
    if (ret < 0) {
    dev_err(info.dev, "Failed to clear INT_SRC%d : %d\n",
    i + 1, ret);
    return ret;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nvvrs_rtc_irq_handler(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t nvvrs_rtc_irq_handler(int irq, void *data)
    {
    struct nvvrs_rtc_info *info = data;
    int ret;
// Check for RTC alarm interrupt
    ret = i2c_smbus_read_byte_data(info.client, NVVRS_REG_INT_SRC1);
    if (ret < 0)
    return IRQ_NONE;
    if (ret & NVVRS_INT_SRC1_RTC_MASK) {
    rtc_lock(info.rtc);
    rtc_update_irq(info.rtc, 1, RTC_IRQF | RTC_AF);
    rtc_unlock(info.rtc);
    }
// Clear all interrupts
    if (nvvrs_pseq_irq_clear(info) < 0)
    return IRQ_NONE;
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn nvvrs_rtc_alarm_irq_enable(dev: *mut device, enabled: c_uint) -> c_int {
    static int nvvrs_rtc_alarm_irq_enable(struct device *dev, unsigned int enabled)
    {
//
// This hardware does not support enabling/disabling the alarm IRQ
// independently. The alarm is disabled by clearing the alarm time
// via set_alarm().
//
    return 0;
    }
    static const struct rtc_class_ops nvvrs_rtc_ops = {
    .read_time = nvvrs_rtc_read_time,
    .set_time = nvvrs_rtc_set_time,
    .read_alarm = nvvrs_rtc_read_alarm,
    .set_alarm = nvvrs_rtc_set_alarm,
    .alarm_irq_enable = nvvrs_rtc_alarm_irq_enable,
    };
#[no_mangle]
unsafe extern "C" fn nvvrs_pseq_vendor_info(info: *mut nvvrs_rtc_info) -> c_int {
    static int nvvrs_pseq_vendor_info(struct nvvrs_rtc_info *info)
    {
    struct i2c_client *client = info.client;
    u8 vendor_id, model_rev;
    int ret;
    ret = i2c_smbus_read_byte_data(client, NVVRS_REG_VENDOR_ID);
    if (ret < 0)
    return dev_err_probe(&client.dev, ret,
    "Failed to read Vendor ID\n");
    vendor_id = (u8)ret;
    ret = i2c_smbus_read_byte_data(client, NVVRS_REG_MODEL_REV);
    if (ret < 0)
    return dev_err_probe(&client.dev, ret,
    "Failed to read Model Revision\n");
    model_rev = (u8)ret;
    if (model_rev < NVVRS_MIN_MODEL_REV) {
    return dev_err_probe(&client.dev, -ENODEV,
    "Chip revision 0x%02x is not supported!\n",
    model_rev);
    }
    dev_dbg(&client.dev, "NVVRS Vendor ID: 0x%02x, Model Rev: 0x%02x\n",
    vendor_id, model_rev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nvvrs_rtc_probe(client: *mut i2c_client) -> c_int {
    static int nvvrs_rtc_probe(struct i2c_client *client)
    {
    struct nvvrs_rtc_info *info;
    int ret;
    info = devm_kzalloc(&client.dev, sizeof(*info), GFP_KERNEL);
    if (!info)
    return -ENOMEM;
    if (client.irq <= 0)
    return dev_err_probe(&client.dev, -EINVAL, "No IRQ specified\n");
    info.irq = client.irq;
    info.dev = &client.dev;
    client.flags |= I2C_CLIENT_PEC;
    i2c_set_clientdata(client, info);
    info.client = client;
// Check vendor info
    if (nvvrs_pseq_vendor_info(info) < 0)
    return dev_err_probe(&client.dev, -EINVAL,
    "Failed to get vendor info\n");
// Clear any pending IRQs before requesting IRQ handler
    if (nvvrs_pseq_irq_clear(info) < 0)
    return dev_err_probe(&client.dev, -EINVAL,
    "Failed to clear interrupts\n");
// Allocate RTC device
    info.rtc = devm_rtc_allocate_device(info.dev);
    if (IS_ERR(info.rtc))
    return PTR_ERR(info.rtc);
    info.rtc.ops = &nvvrs_rtc_ops;
    info.rtc.range_min = RTC_TIMESTAMP_BEGIN_2000;
    info.rtc.range_max = RTC_TIMESTAMP_END_2099;
// Request RTC IRQ
    ret = devm_request_threaded_irq(info.dev, info.irq, core::ptr::null_mut(),
    nvvrs_rtc_irq_handler, IRQF_ONESHOT,
    "nvvrs-rtc", info);
    if (ret < 0) {
    dev_err_probe(info.dev, ret, "Failed to request RTC IRQ\n");
    return ret;
    }
// RTC as a wakeup source
    devm_device_init_wakeup(info.dev);
    return devm_rtc_register_device(info.rtc);
    }

#[no_mangle]
unsafe extern "C" fn nvvrs_rtc_suspend(dev: *mut device) -> c_int {
    static int nvvrs_rtc_suspend(struct device *dev)
    {
    struct nvvrs_rtc_info *info = dev_get_drvdata(dev);
    int ret;
    if (device_may_wakeup(dev)) {
// Set RTC_WAKE bit for auto wake system from suspend state
    ret = nvvrs_update_bits(info, NVVRS_REG_CTL_2,
    NVVRS_REG_CTL_2_RTC_WAKE,
    NVVRS_REG_CTL_2_RTC_WAKE);
    if (ret < 0) {
    dev_err(info.dev, "Failed to set RTC_WAKE bit (%d)\n",
    ret);
    return ret;
    }
    return enable_irq_wake(info.irq);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nvvrs_rtc_resume(dev: *mut device) -> c_int {
    static int nvvrs_rtc_resume(struct device *dev)
    {
    struct nvvrs_rtc_info *info = dev_get_drvdata(dev);
    int ret;
    if (device_may_wakeup(dev)) {
// Clear FORCE_ACT bit
    ret = nvvrs_update_bits(info, NVVRS_REG_CTL_1,
    NVVRS_REG_CTL_1_FORCE_ACT, 0);
    if (ret < 0) {
    dev_err(info.dev, "Failed to clear FORCE_ACT bit (%d)\n",
    ret);
    return ret;
    }
    return disable_irq_wake(info.irq);
    }
    return 0;
    }

    static SIMPLE_DEV_PM_OPS(nvvrs_rtc_pm_ops, nvvrs_rtc_suspend, nvvrs_rtc_resume);
    static const struct of_device_id nvvrs_rtc_of_match[] = {
    { .compatible = "nvidia,vrs-10" },
    { },
    };
    MODULE_DEVICE_TABLE(of, nvvrs_rtc_of_match);
    static struct i2c_driver nvvrs_rtc_driver = {
    .driver		= {
    .name   = "rtc-nvidia-vrs10",
    .pm     = &nvvrs_rtc_pm_ops,
    .of_match_table = nvvrs_rtc_of_match,
    },
    .probe		= nvvrs_rtc_probe,
    };
    module_i2c_driver(nvvrs_rtc_driver);
    MODULE_AUTHOR("Shubhi Garg <shgarg@nvidia.com>");
    MODULE_DESCRIPTION("NVIDIA Voltage Regulator Specification RTC driver");
    MODULE_LICENSE("GPL");
