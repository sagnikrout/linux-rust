//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/kbatt.c
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
// Driver for KEBA battery monitoring controller FPGA IP core
//

pub const KBATT_CONTROL_REG: c_uint = 0x4;
pub const KBATT_CONTROL_BAT_TEST: c_uint = 0x01;
pub const KBATT_STATUS_REG: c_uint = 0x8;
pub const KBATT_STATUS_BAT_OK: c_uint = 0x01;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kbatt {
// update lock
    pub lock: mutex,
    pub base: *mut void __iomem,
    pub /: *mut *mut unsigned long next_update; / in jiffies,
    pub alarm: bool,
}

#[no_mangle]
unsafe extern "C" fn kbatt_alarm(kbatt: *mut kbatt) -> bool {
    static bool kbatt_alarm(struct kbatt *kbatt)
    {
    mutex_lock(&kbatt.lock);
    if (!kbatt.next_update || time_after(jiffies, kbatt.next_update)) {
// switch load on
    iowrite8(KBATT_CONTROL_BAT_TEST,
    kbatt.base + KBATT_CONTROL_REG);
// wait some time to let things settle
    fsleep(KBATT_SETTLE_TIME_US);
// check battery state
    if (ioread8(kbatt.base + KBATT_STATUS_REG) &
    KBATT_STATUS_BAT_OK)
    kbatt.alarm = false;
    else
    kbatt.alarm = true;
// switch load off
    iowrite8(0, kbatt.base + KBATT_CONTROL_REG);
    kbatt.next_update = jiffies + KBATT_MAX_UPD_INTERVAL;
    }
    mutex_unlock(&kbatt.lock);
    return kbatt.alarm;
    }
    static int kbatt_read(struct device *dev, enum hwmon_sensor_types type,
    u32 attr, int channel, long *val)
    {
    struct kbatt *kbatt = dev_get_drvdata(dev);
// val = kbatt_alarm(kbatt) ? 1 : 0;
    return 0;
    }
    static umode_t kbatt_is_visible(const void *data, enum hwmon_sensor_types type,
    u32 attr, int channel)
    {
    if (channel == 0 && attr == hwmon_in_min_alarm)
    return 0444;
    return 0;
    }
    static const struct hwmon_channel_info *kbatt_info[] = {
    HWMON_CHANNEL_INFO(in,
// 0: input minimum alarm channel
    HWMON_I_MIN_ALARM),
    core::ptr::null_mut()
    };
    static const struct hwmon_ops kbatt_hwmon_ops = {
    .is_visible = kbatt_is_visible,
    .read = kbatt_read,
    };
    static const struct hwmon_chip_info kbatt_chip_info = {
    .ops = &kbatt_hwmon_ops,
    .info = kbatt_info,
    };
    static int kbatt_probe(struct auxiliary_device *auxdev,
    const struct auxiliary_device_id *id)
    {
    struct keba_batt_auxdev *kbatt_auxdev =
    container_of(auxdev, struct keba_batt_auxdev, auxdev);
    struct device *dev = &auxdev.dev;
    struct device *hwmon_dev;
    struct kbatt *kbatt;
    int retval;
    kbatt = devm_kzalloc(dev, sizeof(*kbatt), GFP_KERNEL);
    if (!kbatt)
    return -ENOMEM;
    retval = devm_mutex_init(dev, &kbatt.lock);
    if (retval)
    return retval;
    kbatt.base = devm_ioremap_resource(dev, &kbatt_auxdev.io);
    if (IS_ERR(kbatt.base))
    return PTR_ERR(kbatt.base);
    hwmon_dev = devm_hwmon_device_register_with_info(dev, KBATT, kbatt,
    &kbatt_chip_info,
    core::ptr::null_mut());
    return PTR_ERR_OR_ZERO(hwmon_dev);
    }
    static const struct auxiliary_device_id kbatt_devtype_aux[] = {
    { .name = "keba.batt" },
    {}
    };
    MODULE_DEVICE_TABLE(auxiliary, kbatt_devtype_aux);
    static struct auxiliary_driver kbatt_driver_aux = {
    .name = KBATT,
    .id_table = kbatt_devtype_aux,
    .probe = kbatt_probe,
    };
    module_auxiliary_driver(kbatt_driver_aux);
    MODULE_AUTHOR("Petar Bojanic <boja@keba.com>");
    MODULE_AUTHOR("Gerhard Engleder <eg@keba.com>");
    MODULE_DESCRIPTION("KEBA battery monitoring controller driver");
    MODULE_LICENSE("GPL");
