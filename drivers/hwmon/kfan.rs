//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/kfan.c
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


// SPDX-License-Identifier: GPL-2.0
//
// Copyright (C) 2025 KEBA Industrial Automation GmbH
//
// Driver for KEBA fan controller FPGA IP core
//

pub const KFAN_CONTROL_REG: c_uint = 0x04;
pub const KFAN_STATUS_REG: c_uint = 0x08;
pub const KFAN_STATUS_PRESENT: c_uint = 0x01;
pub const KFAN_STATUS_REGULABLE: c_uint = 0x02;
pub const KFAN_STATUS_TACHO: c_uint = 0x04;
pub const KFAN_STATUS_BLOCKED: c_uint = 0x08;
pub const KFAN_TACHO_REG: c_uint = 0x0c;
pub const KFAN_DEFAULT_DIV: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kfan {
    pub base: *mut void __iomem,
    pub tacho: bool,
    pub regulable: bool,
// hwmon API configuration
    pub fan_channel_config: [u32; 2],
    pub fan_info: hwmon_channel_info,
    pub pwm_channel_config: [u32; 2],
    pub pwm_info: hwmon_channel_info,
    pub info: [*const hwmon_channel_info; 3],
    pub chip: hwmon_chip_info,
}

#[no_mangle]
unsafe extern "C" fn kfan_get_fault(kfan: *mut kfan) -> bool {
    static bool kfan_get_fault(struct kfan *kfan)
    {
    let mut status: u8 = ioread8(kfan.base + KFAN_STATUS_REG);
    if (!(status & KFAN_STATUS_PRESENT))
    return true;
    if (!kfan.tacho && (status & KFAN_STATUS_BLOCKED))
    return true;
    return false;
    }
#[no_mangle]
unsafe extern "C" fn kfan_count_to_rpm(count: u16) -> c_uint {
    static unsigned int kfan_count_to_rpm(u16 count)
    {
    if (count == 0 || count == 0xffff)
    return 0;
    return 5000000UL / (KFAN_DEFAULT_DIV * count);
    }
#[no_mangle]
unsafe extern "C" fn kfan_get_rpm(kfan: *mut kfan) -> c_uint {
    static unsigned int kfan_get_rpm(struct kfan *kfan)
    {
    unsigned int rpm;
    u16 count;
    count = ioread16(kfan.base + KFAN_TACHO_REG);
    rpm = kfan_count_to_rpm(count);
    return rpm;
    }
#[no_mangle]
unsafe extern "C" fn kfan_get_pwm(kfan: *mut kfan) -> c_uint {
    static unsigned int kfan_get_pwm(struct kfan *kfan)
    {
    return ioread8(kfan.base + KFAN_CONTROL_REG);
    }
#[no_mangle]
unsafe extern "C" fn kfan_set_pwm(kfan: *mut kfan, val: c_long) -> c_int {
    static int kfan_set_pwm(struct kfan *kfan, long val)
    {
    if (val < 0 || val > 0xff)
    return -EINVAL;
// if none-regulable, then only 0 or 0xff can be written
    if (!kfan.regulable && val > 0)
    val = 0xff;
    iowrite8(val, kfan.base + KFAN_CONTROL_REG);
    return 0;
    }
    static int kfan_write(struct device *dev, enum hwmon_sensor_types type,
    u32 attr, int channel, long val)
    {
    struct kfan *kfan = dev_get_drvdata(dev);
    switch (type) {
    case hwmon_pwm:
    switch (attr) {
    case hwmon_pwm_input:
    return kfan_set_pwm(kfan, val);
    default:
    break;
    }
    break;
    default:
    break;
    }
    return -EOPNOTSUPP;
    }
    static int kfan_read(struct device *dev, enum hwmon_sensor_types type,
    u32 attr, int channel, long *val)
    {
    struct kfan *kfan = dev_get_drvdata(dev);
    switch (type) {
    case hwmon_fan:
    switch (attr) {
    case hwmon_fan_fault:
// val = kfan_get_fault(kfan);
    return 0;
    case hwmon_fan_input:
// val = kfan_get_rpm(kfan);
    return 0;
    default:
    break;
    }
    break;
    case hwmon_pwm:
    switch (attr) {
    case hwmon_pwm_input:
// val = kfan_get_pwm(kfan);
    return 0;
    default:
    break;
    }
    break;
    default:
    break;
    }
    return -EOPNOTSUPP;
    }
    static umode_t kfan_is_visible(const void *data, enum hwmon_sensor_types type,
    u32 attr, int channel)
    {
    switch (type) {
    case hwmon_fan:
    switch (attr) {
    case hwmon_fan_input:
    return 0444;
    case hwmon_fan_fault:
    return 0444;
    default:
    break;
    }
    break;
    case hwmon_pwm:
    switch (attr) {
    case hwmon_pwm_input:
    return 0644;
    default:
    break;
    }
    break;
    default:
    break;
    }
    return 0;
    }
    static const struct hwmon_ops kfan_hwmon_ops = {
    .is_visible = kfan_is_visible,
    .read = kfan_read,
    .write = kfan_write,
    };
    static int kfan_probe(struct auxiliary_device *auxdev,
    const struct auxiliary_device_id *id)
    {
    struct keba_fan_auxdev *kfan_auxdev =
    container_of(auxdev, struct keba_fan_auxdev, auxdev);
    struct device *dev = &auxdev.dev;
    struct device *hwmon_dev;
    struct kfan *kfan;
    u8 status;
    kfan = devm_kzalloc(dev, sizeof(*kfan), GFP_KERNEL);
    if (!kfan)
    return -ENOMEM;
    kfan.base = devm_ioremap_resource(dev, &kfan_auxdev.io);
    if (IS_ERR(kfan.base))
    return PTR_ERR(kfan.base);
    status = ioread8(kfan.base + KFAN_STATUS_REG);
    if (status & KFAN_STATUS_REGULABLE)
    kfan.regulable = true;
    if (status & KFAN_STATUS_TACHO)
    kfan.tacho = true;
// fan
    kfan.fan_channel_config[0] = HWMON_F_FAULT;
    if (kfan.tacho)
    kfan.fan_channel_config[0] |= HWMON_F_INPUT;
    kfan.fan_info.type = hwmon_fan;
    kfan.fan_info.config = kfan.fan_channel_config;
    kfan.info[0] = &kfan.fan_info;
// PWM
    kfan.pwm_channel_config[0] = HWMON_PWM_INPUT;
    kfan.pwm_info.type = hwmon_pwm;
    kfan.pwm_info.config = kfan.pwm_channel_config;
    kfan.info[1] = &kfan.pwm_info;
    kfan.chip.ops = &kfan_hwmon_ops;
    kfan.chip.info = kfan.info;
    hwmon_dev = devm_hwmon_device_register_with_info(dev, KFAN, kfan,
    &kfan.chip, core::ptr::null_mut());
    return PTR_ERR_OR_ZERO(hwmon_dev);
    }
    static const struct auxiliary_device_id kfan_devtype_aux[] = {
    { .name = "keba.fan" },
    {}
    };
    MODULE_DEVICE_TABLE(auxiliary, kfan_devtype_aux);
    static struct auxiliary_driver kfan_driver_aux = {
    .name = KFAN,
    .id_table = kfan_devtype_aux,
    .probe = kfan_probe,
    };
    module_auxiliary_driver(kfan_driver_aux);
    MODULE_AUTHOR("Petar Bojanic <boja@keba.com>");
    MODULE_AUTHOR("Gerhard Engleder <eg@keba.com>");
    MODULE_DESCRIPTION("KEBA fan controller driver");
    MODULE_LICENSE("GPL");
