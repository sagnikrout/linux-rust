//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/ultra45_env.c
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
// ultra45_env.c: Driver for Ultra45 PIC16F747 environmental monitor.
//
// Copyright (C) 2008 David S. Miller <davem@davemloft.net>
//

    MODULE_AUTHOR("David S. Miller <davem@davemloft.net>");
    MODULE_DESCRIPTION("Ultra45 environmental monitor driver");
    MODULE_LICENSE("GPL");
    MODULE_VERSION(DRV_MODULE_VERSION);
// PIC device registers
pub const REG_CMD: c_uint = 0x00UL;
pub const REG_CMD_RESET: c_uint = 0x80;
pub const REG_CMD_ESTAR: c_uint = 0x01;
pub const REG_STAT: c_uint = 0x01UL;
pub const REG_STAT_FWVER: c_uint = 0xf0;
pub const REG_STAT_TGOOD: c_uint = 0x08;
pub const REG_STAT_STALE: c_uint = 0x04;
pub const REG_STAT_BUSY: c_uint = 0x02;
pub const REG_STAT_FAULT: c_uint = 0x01;
pub const REG_DATA: c_uint = 0x40UL;
pub const REG_ADDR: c_uint = 0x41UL;
pub const REG_SIZE: c_uint = 0x42UL;
// Registers accessed indirectly via REG_DATA/REG_ADDR
pub const IREG_FAN0: c_uint = 0x00;
pub const IREG_FAN1: c_uint = 0x01;
pub const IREG_FAN2: c_uint = 0x02;
pub const IREG_FAN3: c_uint = 0x03;
pub const IREG_FAN4: c_uint = 0x04;
pub const IREG_FAN5: c_uint = 0x05;
pub const IREG_LCL_TEMP: c_uint = 0x06;
pub const IREG_RMT1_TEMP: c_uint = 0x07;
pub const IREG_RMT2_TEMP: c_uint = 0x08;
pub const IREG_RMT3_TEMP: c_uint = 0x09;
pub const IREG_LM95221_TEMP: c_uint = 0x0a;
pub const IREG_FIRE_TEMP: c_uint = 0x0b;
pub const IREG_LSI1064_TEMP: c_uint = 0x0c;
pub const IREG_FRONT_TEMP: c_uint = 0x0d;
pub const IREG_FAN_STAT: c_uint = 0x0e;
pub const IREG_VCORE0: c_uint = 0x0f;
pub const IREG_VCORE1: c_uint = 0x10;
pub const IREG_VMEM0: c_uint = 0x11;
pub const IREG_VMEM1: c_uint = 0x12;
pub const IREG_PSU_TEMP: c_uint = 0x13;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct env {
    pub regs: *mut void __iomem,
    pub lock: spinlock_t,
    pub hwmon_dev: *mut device,
}

#[no_mangle]
unsafe extern "C" fn env_read(p: *mut env, ireg: u8) -> u8 {
    static u8 env_read(struct env *p, u8 ireg)
    {
    u8 ret;
    spin_lock(&p.lock);
    writeb(ireg, p.regs + REG_ADDR);
    ret = readb(p.regs + REG_DATA);
    spin_unlock(&p.lock);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn env_write(p: *mut env, ireg: u8, val: u8) {
    static void env_write(struct env *p, u8 ireg, u8 val)
    {
    spin_lock(&p.lock);
    writeb(ireg, p.regs + REG_ADDR);
    writeb(val, p.regs + REG_DATA);
    spin_unlock(&p.lock);
    }
//
// There seems to be a adr7462 providing these values, thus a lot
// of these calculations are borrowed from the adt7470 driver.
//

    static ssize_t show_fan_speed(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    let mut fan_nr: c_int = to_sensor_dev_attr(attr).index;
    struct env *p = dev_get_drvdata(dev);
    int rpm, period;
    u8 val;
    val = env_read(p, IREG_FAN0 + fan_nr);
    period = (int) val << 8;
    if (FAN_DATA_VALID(period))
    rpm = FAN_PERIOD_TO_RPM(period);
    else
    rpm = 0;
    return sprintf(buf, "%d\n", rpm);
    }
    static ssize_t set_fan_speed(struct device *dev, struct device_attribute *attr,
    const char *buf, size_t count)
    {
    let mut fan_nr: c_int = to_sensor_dev_attr(attr).index;
    unsigned long rpm;
    struct env *p = dev_get_drvdata(dev);
    int period;
    u8 val;
    int err;
    err = kstrtoul(buf, 10, &rpm);
    if (err)
    return err;
    if (!rpm)
    return -EINVAL;
    period = FAN_RPM_TO_PERIOD(rpm);
    val = period >> 8;
    env_write(p, IREG_FAN0 + fan_nr, val);
    return count;
    }
    static ssize_t show_fan_fault(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    let mut fan_nr: c_int = to_sensor_dev_attr(attr).index;
    struct env *p = dev_get_drvdata(dev);
    let mut val: u8 = env_read(p, IREG_FAN_STAT);
    return sprintf(buf, "%d\n", (val & (1 << fan_nr)) ? 1 : 0);
    }

    static SENSOR_DEVICE_ATTR(fan##index##_speed, S_IRUGO | S_IWUSR,	\
    show_fan_speed, set_fan_speed, index);			\
    static SENSOR_DEVICE_ATTR(fan##index##_fault, S_IRUGO,			\
    show_fan_fault, core::ptr::null_mut(), index)
    fan(0);
    fan(1);
    fan(2);
    fan(3);
    fan(4);
    static SENSOR_DEVICE_ATTR(psu_fan_fault, S_IRUGO, show_fan_fault, core::ptr::null_mut(), 6);
    static ssize_t show_temp(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    let mut temp_nr: c_int = to_sensor_dev_attr(attr).index;
    struct env *p = dev_get_drvdata(dev);
    s8 val;
    val = env_read(p, IREG_LCL_TEMP + temp_nr);
    return sprintf(buf, "%d\n", ((int) val) - 64);
    }
    static SENSOR_DEVICE_ATTR(adt7462_local_temp, S_IRUGO, show_temp, core::ptr::null_mut(), 0);
    static SENSOR_DEVICE_ATTR(cpu0_temp, S_IRUGO, show_temp, core::ptr::null_mut(), 1);
    static SENSOR_DEVICE_ATTR(cpu1_temp, S_IRUGO, show_temp, core::ptr::null_mut(), 2);
    static SENSOR_DEVICE_ATTR(motherboard_temp, S_IRUGO, show_temp, core::ptr::null_mut(), 3);
    static SENSOR_DEVICE_ATTR(lm95221_local_temp, S_IRUGO, show_temp, core::ptr::null_mut(), 4);
    static SENSOR_DEVICE_ATTR(fire_temp, S_IRUGO, show_temp, core::ptr::null_mut(), 5);
    static SENSOR_DEVICE_ATTR(lsi1064_local_temp, S_IRUGO, show_temp, core::ptr::null_mut(), 6);
    static SENSOR_DEVICE_ATTR(front_panel_temp, S_IRUGO, show_temp, core::ptr::null_mut(), 7);
    static SENSOR_DEVICE_ATTR(psu_temp, S_IRUGO, show_temp, core::ptr::null_mut(), 13);
    static ssize_t show_stat_bit(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    let mut index: c_int = to_sensor_dev_attr(attr).index;
    struct env *p = dev_get_drvdata(dev);
    u8 val;
    val = readb(p.regs + REG_STAT);
    return sprintf(buf, "%d\n", (val & (1 << index)) ? 1 : 0);
    }
    static SENSOR_DEVICE_ATTR(fan_failure, S_IRUGO, show_stat_bit, core::ptr::null_mut(), 0);
    static SENSOR_DEVICE_ATTR(env_bus_busy, S_IRUGO, show_stat_bit, core::ptr::null_mut(), 1);
    static SENSOR_DEVICE_ATTR(env_data_stale, S_IRUGO, show_stat_bit, core::ptr::null_mut(), 2);
    static SENSOR_DEVICE_ATTR(tpm_self_test_passed, S_IRUGO, show_stat_bit, core::ptr::null_mut(),
    3);
    static ssize_t show_fwver(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    struct env *p = dev_get_drvdata(dev);
    u8 val;
    val = readb(p.regs + REG_STAT);
    return sprintf(buf, "%d\n", val >> 4);
    }
    static SENSOR_DEVICE_ATTR(firmware_version, S_IRUGO, show_fwver, core::ptr::null_mut(), 0);
    static ssize_t show_name(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    return sprintf(buf, "ultra45\n");
    }
    static SENSOR_DEVICE_ATTR(name, S_IRUGO, show_name, core::ptr::null_mut(), 0);
    static struct attribute *env_attributes[] = {
    &sensor_dev_attr_fan0_speed.dev_attr.attr,
    &sensor_dev_attr_fan0_fault.dev_attr.attr,
    &sensor_dev_attr_fan1_speed.dev_attr.attr,
    &sensor_dev_attr_fan1_fault.dev_attr.attr,
    &sensor_dev_attr_fan2_speed.dev_attr.attr,
    &sensor_dev_attr_fan2_fault.dev_attr.attr,
    &sensor_dev_attr_fan3_speed.dev_attr.attr,
    &sensor_dev_attr_fan3_fault.dev_attr.attr,
    &sensor_dev_attr_fan4_speed.dev_attr.attr,
    &sensor_dev_attr_fan4_fault.dev_attr.attr,
    &sensor_dev_attr_psu_fan_fault.dev_attr.attr,
    &sensor_dev_attr_adt7462_local_temp.dev_attr.attr,
    &sensor_dev_attr_cpu0_temp.dev_attr.attr,
    &sensor_dev_attr_cpu1_temp.dev_attr.attr,
    &sensor_dev_attr_motherboard_temp.dev_attr.attr,
    &sensor_dev_attr_lm95221_local_temp.dev_attr.attr,
    &sensor_dev_attr_fire_temp.dev_attr.attr,
    &sensor_dev_attr_lsi1064_local_temp.dev_attr.attr,
    &sensor_dev_attr_front_panel_temp.dev_attr.attr,
    &sensor_dev_attr_psu_temp.dev_attr.attr,
    &sensor_dev_attr_fan_failure.dev_attr.attr,
    &sensor_dev_attr_env_bus_busy.dev_attr.attr,
    &sensor_dev_attr_env_data_stale.dev_attr.attr,
    &sensor_dev_attr_tpm_self_test_passed.dev_attr.attr,
    &sensor_dev_attr_firmware_version.dev_attr.attr,
    &sensor_dev_attr_name.dev_attr.attr,
    core::ptr::null_mut(),
    };
    static const struct attribute_group env_group = {
    .attrs = env_attributes,
    };
#[no_mangle]
unsafe extern "C" fn env_probe(op: *mut platform_device) -> c_int {
    static int env_probe(struct platform_device *op)
    {
    struct env *p = devm_kzalloc(&op.dev, sizeof(*p), GFP_KERNEL);
    let mut err: c_int = -ENOMEM;
    if (!p)
    goto out;
    spin_lock_init(&p.lock);
    p.regs = of_ioremap(&op.resource[0], 0, REG_SIZE, "pic16f747");
    if (!p.regs)
    goto out;
    err = sysfs_create_group(&op.dev.kobj, &env_group);
    if (err)
    goto out_iounmap;
    p.hwmon_dev = hwmon_device_register(&op.dev);
    if (IS_ERR(p.hwmon_dev)) {
    err = PTR_ERR(p.hwmon_dev);
    goto out_sysfs_remove_group;
    }
    platform_set_drvdata(op, p);
    err = 0;
    out:
    return err;
    out_sysfs_remove_group:
    sysfs_remove_group(&op.dev.kobj, &env_group);
    out_iounmap:
    of_iounmap(&op.resource[0], p.regs, REG_SIZE);
    goto out;
    }
#[no_mangle]
unsafe extern "C" fn env_remove(op: *mut platform_device) {
    static void env_remove(struct platform_device *op)
    {
    struct env *p = platform_get_drvdata(op);
    if (p) {
    sysfs_remove_group(&op.dev.kobj, &env_group);
    hwmon_device_unregister(p.hwmon_dev);
    of_iounmap(&op.resource[0], p.regs, REG_SIZE);
    }
    }
    static const struct of_device_id env_match[] = {
    {
    .name = "env-monitor",
    .compatible = "SUNW,ebus-pic16f747-env",
    },
    {},
    };
    MODULE_DEVICE_TABLE(of, env_match);
    static struct platform_driver env_driver = {
    .driver = {
    .name = "ultra45_env",
    .of_match_table = env_match,
    },
    .probe		= env_probe,
    .remove		= env_remove,
    };
    module_platform_driver(env_driver);
