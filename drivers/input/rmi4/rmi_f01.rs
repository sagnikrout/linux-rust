//! Automatically rewritten from C to Rust
//! Source: drivers/input/rmi4/rmi_f01.c
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
// Copyright (c) 2011-2016 Synaptics Incorporated
// Copyright (c) 2011 Unixphere
//

pub const RMI_PRODUCT_ID_LENGTH: c_int = 10;
pub const RMI_PRODUCT_INFO_LENGTH: c_int = 2;
pub const RMI_DATE_CODE_LENGTH: c_int = 3;
pub const PRODUCT_ID_OFFSET: c_uint = 0x10;
pub const PRODUCT_INFO_OFFSET: c_uint = 0x1E;
// Force a firmware reset of the sensor
pub const RMI_F01_CMD_DEVICE_RESET: c_int = 1;
// Various F01_RMI_QueryX bits

pub const RMI_F01_QRY5_YEAR_MASK: c_uint = 0x1f;
pub const RMI_F01_QRY6_MONTH_MASK: c_uint = 0x0f;
pub const RMI_F01_QRY7_DAY_MASK: c_uint = 0x1f;
pub const RMI_F01_QRY2_PRODINFO_MASK: c_uint = 0x7f;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct f01_basic_properties {
    pub manufacturer_id: u8,
    pub has_lts: bool,
    pub has_adjustable_doze: bool,
    pub has_adjustable_doze_holdoff: bool,
    pub /: *mut *mut char dom[11]; / YYYY/MM/DD + '\0',
    pub 1]: u8 product_id[RMI_PRODUCT_ID_LENGTH +,
    pub productinfo: u16,
    pub firmware_id: u32,
    pub package_id: u32,
}

// F01 device status bits
// Most recent device status event

// The device has lost its configuration for some reason.

// The device is in bootloader mode

// Control register bits
//
// Sleep mode controls power management on the device and affects all
// functions of the device.
//
pub const RMI_F01_CTRL0_SLEEP_MODE_MASK: c_uint = 0x03;
pub const RMI_SLEEP_MODE_NORMAL: c_uint = 0x00;
pub const RMI_SLEEP_MODE_SENSOR_SLEEP: c_uint = 0x01;
pub const RMI_SLEEP_MODE_RESERVED0: c_uint = 0x02;
pub const RMI_SLEEP_MODE_RESERVED1: c_uint = 0x03;
//
// This bit disables whatever sleep mode may be selected by the sleep_mode
// field and forces the device to run at full power without sleeping.
//

//
// When this bit is set, the touch controller employs a noise-filtering
// algorithm designed for use with a connected battery charger.
//

//
// Sets the report rate for the device. The effect of this setting is
// highly product dependent. Check the spec sheet for your particular
// touch sensor.
//

//
// Written by the host as an indicator that the device has been
// successfully configured.
//

//
// struct f01_device_control - controls basic sensor functions
//
// @ctrl0: see the bit definitions above.
// @doze_interval: controls the interval between checks for finger presence
// when the touch sensor is in doze mode, in units of 10ms.
// @wakeup_threshold: controls the capacitance threshold at which the touch
// sensor will decide to wake up from that low power state.
// @doze_holdoff: controls how long the touch sensor waits after the last
// finger lifts before entering the doze state, in units of 100ms.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct f01_device_control {
    pub ctrl0: u8,
    pub doze_interval: u8,
    pub wakeup_threshold: u8,
    pub doze_holdoff: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct f01_data {
    pub properties: f01_basic_properties,
    pub device_control: f01_device_control,
    pub doze_interval_addr: u16,
    pub wakeup_threshold_addr: u16,
    pub doze_holdoff_addr: u16,
    pub suspended: bool,
    pub old_nosleep: bool,
    pub num_of_irq_regs: c_uint,
}

    static int rmi_f01_read_properties(struct rmi_device *rmi_dev,
    u16 query_base_addr,
    struct f01_basic_properties *props)
    {
    u8 queries[RMI_F01_BASIC_QUERY_LEN];
    int ret;
    let mut query_offset: c_int = query_base_addr;
    let mut has_ds4_queries: bool = false;
    let mut has_query42: bool = false;
    let mut has_sensor_id: bool = false;
    let mut has_package_id_query: bool = false;
    let mut has_build_id_query: bool = false;
    u16 prod_info_addr;
    u8 ds4_query_len;
    ret = rmi_read_block(rmi_dev, query_offset,
    queries, RMI_F01_BASIC_QUERY_LEN);
    if (ret) {
    dev_err(&rmi_dev.dev,
    "Failed to read device query registers: %d\n", ret);
    return ret;
    }
    prod_info_addr = query_offset + 17;
    query_offset += RMI_F01_BASIC_QUERY_LEN;
// Now parse what we got
    props.manufacturer_id = queries[0];
    props.has_lts = queries[1] & RMI_F01_QRY1_HAS_LTS;
    props.has_adjustable_doze =
    queries[1] & RMI_F01_QRY1_HAS_ADJ_DOZE;
    props.has_adjustable_doze_holdoff =
    queries[1] & RMI_F01_QRY1_HAS_ADJ_DOZE_HOFF;
    has_query42 = queries[1] & RMI_F01_QRY1_HAS_QUERY42;
    has_sensor_id = queries[1] & RMI_F01_QRY1_HAS_SENSOR_ID;
    snprintf(props.dom, sizeof(props.dom), "20%02d/%02d/%02d",
    queries[5] & RMI_F01_QRY5_YEAR_MASK,
    queries[6] & RMI_F01_QRY6_MONTH_MASK,
    queries[7] & RMI_F01_QRY7_DAY_MASK);
    memcpy(props.product_id, &queries[11],
    RMI_PRODUCT_ID_LENGTH);
    props.product_id[RMI_PRODUCT_ID_LENGTH] = '\0';
    props.productinfo =
    ((queries[2] & RMI_F01_QRY2_PRODINFO_MASK) << 7) |
    (queries[3] & RMI_F01_QRY2_PRODINFO_MASK);
    if (has_sensor_id)
    query_offset++;
    if (has_query42) {
    ret = rmi_read(rmi_dev, query_offset, queries);
    if (ret) {
    dev_err(&rmi_dev.dev,
    "Failed to read query 42 register: %d\n", ret);
    return ret;
    }
    has_ds4_queries = !!(queries[0] & BIT(0));
    query_offset++;
    }
    if (has_ds4_queries) {
    ret = rmi_read(rmi_dev, query_offset, &ds4_query_len);
    if (ret) {
    dev_err(&rmi_dev.dev,
    "Failed to read DS4 queries length: %d\n", ret);
    return ret;
    }
    query_offset++;
    if (ds4_query_len > 0) {
    ret = rmi_read(rmi_dev, query_offset, queries);
    if (ret) {
    dev_err(&rmi_dev.dev,
    "Failed to read DS4 queries: %d\n",
    ret);
    return ret;
    }
    has_package_id_query = !!(queries[0] & BIT(0));
    has_build_id_query = !!(queries[0] & BIT(1));
    }
    if (has_package_id_query) {
    ret = rmi_read_block(rmi_dev, prod_info_addr,
    queries, sizeof(__le64));
    if (ret) {
    dev_err(&rmi_dev.dev,
    "Failed to read package info: %d\n",
    ret);
    return ret;
    }
    props.package_id = get_unaligned_le64(queries);
    prod_info_addr++;
    }
    if (has_build_id_query) {
    ret = rmi_read_block(rmi_dev, prod_info_addr, queries,
    3);
    if (ret) {
    dev_err(&rmi_dev.dev,
    "Failed to read product info: %d\n",
    ret);
    return ret;
    }
    props.firmware_id = queries[1] << 8 | queries[0];
    props.firmware_id += queries[2] * 65536;
    }
    }
    return 0;
    }
    const char *rmi_f01_get_product_ID(struct rmi_function *fn)
    {
    struct f01_data *f01 = dev_get_drvdata(&fn.dev);
    return f01.properties.product_id;
    }
    static ssize_t rmi_driver_manufacturer_id_show(struct device *dev,
    struct device_attribute *dattr,
    char *buf)
    {
    struct rmi_driver_data *data = dev_get_drvdata(dev);
    struct f01_data *f01 = dev_get_drvdata(&data.f01_container.dev);
    return sysfs_emit(buf, "%d\n", f01.properties.manufacturer_id);
    }
    static DEVICE_ATTR(manufacturer_id, 0444,
    rmi_driver_manufacturer_id_show, core::ptr::null_mut());
    static ssize_t rmi_driver_dom_show(struct device *dev,
    struct device_attribute *dattr, char *buf)
    {
    struct rmi_driver_data *data = dev_get_drvdata(dev);
    struct f01_data *f01 = dev_get_drvdata(&data.f01_container.dev);
    return sysfs_emit(buf, "%s\n", f01.properties.dom);
    }
    static DEVICE_ATTR(date_of_manufacture, 0444, rmi_driver_dom_show, core::ptr::null_mut());
    static ssize_t rmi_driver_product_id_show(struct device *dev,
    struct device_attribute *dattr,
    char *buf)
    {
    struct rmi_driver_data *data = dev_get_drvdata(dev);
    struct f01_data *f01 = dev_get_drvdata(&data.f01_container.dev);
    return sysfs_emit(buf, "%s\n", f01.properties.product_id);
    }
    static DEVICE_ATTR(product_id, 0444, rmi_driver_product_id_show, core::ptr::null_mut());
    static ssize_t rmi_driver_firmware_id_show(struct device *dev,
    struct device_attribute *dattr,
    char *buf)
    {
    struct rmi_driver_data *data = dev_get_drvdata(dev);
    struct f01_data *f01 = dev_get_drvdata(&data.f01_container.dev);
    return sysfs_emit(buf, "%d\n", f01.properties.firmware_id);
    }
    static DEVICE_ATTR(firmware_id, 0444, rmi_driver_firmware_id_show, core::ptr::null_mut());
    static ssize_t rmi_driver_package_id_show(struct device *dev,
    struct device_attribute *dattr,
    char *buf)
    {
    struct rmi_driver_data *data = dev_get_drvdata(dev);
    struct f01_data *f01 = dev_get_drvdata(&data.f01_container.dev);
    let mut package_id: u32 = f01.properties.package_id;
    return sysfs_emit(buf, "%04x.%04x\n",
    package_id & 0xffff, (package_id >> 16) & 0xffff);
    }
    static DEVICE_ATTR(package_id, 0444, rmi_driver_package_id_show, core::ptr::null_mut());
    static struct attribute *rmi_f01_attrs[] = {
    &dev_attr_manufacturer_id.attr,
    &dev_attr_date_of_manufacture.attr,
    &dev_attr_product_id.attr,
    &dev_attr_firmware_id.attr,
    &dev_attr_package_id.attr,
    core::ptr::null_mut()
    };
    static const struct attribute_group rmi_f01_attr_group = {
    .attrs = rmi_f01_attrs,
    };

    static int rmi_f01_of_probe(struct device *dev,
    struct rmi_device_platform_data *pdata)
    {
    int retval;
    u32 val;
    retval = rmi_of_property_read_u32(dev,
    (u32 *)&pdata.power_management.nosleep,
    "syna,nosleep-mode", 1);
    if (retval)
    return retval;
    retval = rmi_of_property_read_u32(dev, &val,
    "syna,wakeup-threshold", 1);
    if (retval)
    return retval;
    pdata.power_management.wakeup_threshold = val;
    retval = rmi_of_property_read_u32(dev, &val,
    "syna,doze-holdoff-ms", 1);
    if (retval)
    return retval;
    pdata.power_management.doze_holdoff = val * 100;
    retval = rmi_of_property_read_u32(dev, &val,
    "syna,doze-interval-ms", 1);
    if (retval)
    return retval;
    pdata.power_management.doze_interval = val / 10;
    return 0;
    }

    static inline int rmi_f01_of_probe(struct device *dev,
    struct rmi_device_platform_data *pdata)
    {
    return -ENODEV;
    }

#[no_mangle]
unsafe extern "C" fn rmi_f01_probe(fn: *mut rmi_function) -> c_int {
    static int rmi_f01_probe(struct rmi_function *fn)
    {
    struct rmi_device *rmi_dev = fn.rmi_dev;
    struct rmi_driver_data *driver_data = dev_get_drvdata(&rmi_dev.dev);
    struct rmi_device_platform_data *pdata = rmi_get_platform_data(rmi_dev);
    struct f01_data *f01;
    int error;
    let mut ctrl_base_addr: u16 = fn.fd.control_base_addr;
    u8 device_status;
    u8 temp;
    if (fn.dev.of_node) {
    error = rmi_f01_of_probe(&fn.dev, pdata);
    if (error)
    return error;
    }
    f01 = devm_kzalloc(&fn.dev, sizeof(struct f01_data), GFP_KERNEL);
    if (!f01)
    return -ENOMEM;
    f01.num_of_irq_regs = driver_data.num_of_irq_regs;
//
// Set the configured bit and (optionally) other important stuff
// in the device control register.
//
    error = rmi_read(rmi_dev, fn.fd.control_base_addr,
    &f01.device_control.ctrl0);
    if (error) {
    dev_err(&fn.dev, "Failed to read F01 control: %d\n", error);
    return error;
    }
    switch (pdata.power_management.nosleep) {
    case RMI_REG_STATE_DEFAULT:
    break;
    case RMI_REG_STATE_OFF:
    f01.device_control.ctrl0 &= ~RMI_F01_CTRL0_NOSLEEP_BIT;
    break;
    case RMI_REG_STATE_ON:
    f01.device_control.ctrl0 |= RMI_F01_CTRL0_NOSLEEP_BIT;
    break;
    }
//
// Sleep mode might be set as a hangover from a system crash or
// reboot without power cycle.  If so, clear it so the sensor
// is certain to function.
//
    if ((f01.device_control.ctrl0 & RMI_F01_CTRL0_SLEEP_MODE_MASK) !=
    RMI_SLEEP_MODE_NORMAL) {
    dev_warn(&fn.dev,
    "WARNING: Non-zero sleep mode found. Clearing...\n");
    f01.device_control.ctrl0 &= ~RMI_F01_CTRL0_SLEEP_MODE_MASK;
    }
    f01.device_control.ctrl0 |= RMI_F01_CTRL0_CONFIGURED_BIT;
    error = rmi_write(rmi_dev, fn.fd.control_base_addr,
    f01.device_control.ctrl0);
    if (error) {
    dev_err(&fn.dev, "Failed to write F01 control: %d\n", error);
    return error;
    }
// Dummy read in order to clear irqs
    error = rmi_read(rmi_dev, fn.fd.data_base_addr + 1, &temp);
    if (error < 0) {
    dev_err(&fn.dev, "Failed to read Interrupt Status.\n");
    return error;
    }
    error = rmi_f01_read_properties(rmi_dev, fn.fd.query_base_addr,
    &f01.properties);
    if (error < 0) {
    dev_err(&fn.dev, "Failed to read F01 properties.\n");
    return error;
    }
    dev_info(&fn.dev, "found RMI device, manufacturer: %s, product: %s, fw id: %d\n",
    f01.properties.manufacturer_id == 1 ? "Synaptics" : "unknown",
    f01.properties.product_id, f01.properties.firmware_id);
// Advance to interrupt control registers, then skip over them.
    ctrl_base_addr++;
    ctrl_base_addr += f01.num_of_irq_regs;
// read control register
    if (f01.properties.has_adjustable_doze) {
    f01.doze_interval_addr = ctrl_base_addr;
    ctrl_base_addr++;
    if (pdata.power_management.doze_interval) {
    f01.device_control.doze_interval =
    pdata.power_management.doze_interval;
    error = rmi_write(rmi_dev, f01.doze_interval_addr,
    f01.device_control.doze_interval);
    if (error) {
    dev_err(&fn.dev,
    "Failed to configure F01 doze interval register: %d\n",
    error);
    return error;
    }
    } else {
    error = rmi_read(rmi_dev, f01.doze_interval_addr,
    &f01.device_control.doze_interval);
    if (error) {
    dev_err(&fn.dev,
    "Failed to read F01 doze interval register: %d\n",
    error);
    return error;
    }
    }
    f01.wakeup_threshold_addr = ctrl_base_addr;
    ctrl_base_addr++;
    if (pdata.power_management.wakeup_threshold) {
    f01.device_control.wakeup_threshold =
    pdata.power_management.wakeup_threshold;
    error = rmi_write(rmi_dev, f01.wakeup_threshold_addr,
    f01.device_control.wakeup_threshold);
    if (error) {
    dev_err(&fn.dev,
    "Failed to configure F01 wakeup threshold register: %d\n",
    error);
    return error;
    }
    } else {
    error = rmi_read(rmi_dev, f01.wakeup_threshold_addr,
    &f01.device_control.wakeup_threshold);
    if (error < 0) {
    dev_err(&fn.dev,
    "Failed to read F01 wakeup threshold register: %d\n",
    error);
    return error;
    }
    }
    }
    if (f01.properties.has_lts)
    ctrl_base_addr++;
    if (f01.properties.has_adjustable_doze_holdoff) {
    f01.doze_holdoff_addr = ctrl_base_addr;
    ctrl_base_addr++;
    if (pdata.power_management.doze_holdoff) {
    f01.device_control.doze_holdoff =
    pdata.power_management.doze_holdoff;
    error = rmi_write(rmi_dev, f01.doze_holdoff_addr,
    f01.device_control.doze_holdoff);
    if (error) {
    dev_err(&fn.dev,
    "Failed to configure F01 doze holdoff register: %d\n",
    error);
    return error;
    }
    } else {
    error = rmi_read(rmi_dev, f01.doze_holdoff_addr,
    &f01.device_control.doze_holdoff);
    if (error) {
    dev_err(&fn.dev,
    "Failed to read F01 doze holdoff register: %d\n",
    error);
    return error;
    }
    }
    }
    error = rmi_read(rmi_dev, fn.fd.data_base_addr, &device_status);
    if (error < 0) {
    dev_err(&fn.dev,
    "Failed to read device status: %d\n", error);
    return error;
    }
    if (RMI_F01_STATUS_UNCONFIGURED(device_status)) {
    dev_err(&fn.dev,
    "Device was reset during configuration process, status: %#02x!\n",
    RMI_F01_STATUS_CODE(device_status));
    return -EINVAL;
    }
    dev_set_drvdata(&fn.dev, f01);
    error = sysfs_create_group(&fn.rmi_dev.dev.kobj, &rmi_f01_attr_group);
    if (error)
    dev_warn(&fn.dev, "Failed to create sysfs group: %d\n", error);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rmi_f01_remove(fn: *mut rmi_function) {
    static void rmi_f01_remove(struct rmi_function *fn)
    {
// Note that the bus device is used, not the F01 device
    sysfs_remove_group(&fn.rmi_dev.dev.kobj, &rmi_f01_attr_group);
    }
#[no_mangle]
unsafe extern "C" fn rmi_f01_config(fn: *mut rmi_function) -> c_int {
    static int rmi_f01_config(struct rmi_function *fn)
    {
    struct f01_data *f01 = dev_get_drvdata(&fn.dev);
    int error;
    error = rmi_write(fn.rmi_dev, fn.fd.control_base_addr,
    f01.device_control.ctrl0);
    if (error) {
    dev_err(&fn.dev,
    "Failed to write device_control register: %d\n", error);
    return error;
    }
    if (f01.properties.has_adjustable_doze) {
    error = rmi_write(fn.rmi_dev, f01.doze_interval_addr,
    f01.device_control.doze_interval);
    if (error) {
    dev_err(&fn.dev,
    "Failed to write doze interval: %d\n", error);
    return error;
    }
    error = rmi_write_block(fn.rmi_dev,
    f01.wakeup_threshold_addr,
    &f01.device_control.wakeup_threshold,
    sizeof(u8));
    if (error) {
    dev_err(&fn.dev,
    "Failed to write wakeup threshold: %d\n",
    error);
    return error;
    }
    }
    if (f01.properties.has_adjustable_doze_holdoff) {
    error = rmi_write(fn.rmi_dev, f01.doze_holdoff_addr,
    f01.device_control.doze_holdoff);
    if (error) {
    dev_err(&fn.dev,
    "Failed to write doze holdoff: %d\n", error);
    return error;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rmi_f01_suspend(fn: *mut rmi_function) -> c_int {
    static int rmi_f01_suspend(struct rmi_function *fn)
    {
    struct f01_data *f01 = dev_get_drvdata(&fn.dev);
    int error;
    f01.old_nosleep =
    f01.device_control.ctrl0 & RMI_F01_CTRL0_NOSLEEP_BIT;
    f01.device_control.ctrl0 &= ~RMI_F01_CTRL0_NOSLEEP_BIT;
    f01.device_control.ctrl0 &= ~RMI_F01_CTRL0_SLEEP_MODE_MASK;
    if (device_may_wakeup(fn.rmi_dev.xport.dev))
    f01.device_control.ctrl0 |= RMI_SLEEP_MODE_RESERVED1;
    else
    f01.device_control.ctrl0 |= RMI_SLEEP_MODE_SENSOR_SLEEP;
    error = rmi_write(fn.rmi_dev, fn.fd.control_base_addr,
    f01.device_control.ctrl0);
    if (error) {
    dev_err(&fn.dev, "Failed to write sleep mode: %d.\n", error);
    if (f01.old_nosleep)
    f01.device_control.ctrl0 |= RMI_F01_CTRL0_NOSLEEP_BIT;
    f01.device_control.ctrl0 &= ~RMI_F01_CTRL0_SLEEP_MODE_MASK;
    f01.device_control.ctrl0 |= RMI_SLEEP_MODE_NORMAL;
    return error;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rmi_f01_resume(fn: *mut rmi_function) -> c_int {
    static int rmi_f01_resume(struct rmi_function *fn)
    {
    struct f01_data *f01 = dev_get_drvdata(&fn.dev);
    int error;
    if (f01.old_nosleep)
    f01.device_control.ctrl0 |= RMI_F01_CTRL0_NOSLEEP_BIT;
    f01.device_control.ctrl0 &= ~RMI_F01_CTRL0_SLEEP_MODE_MASK;
    f01.device_control.ctrl0 |= RMI_SLEEP_MODE_NORMAL;
    error = rmi_write(fn.rmi_dev, fn.fd.control_base_addr,
    f01.device_control.ctrl0);
    if (error) {
    dev_err(&fn.dev,
    "Failed to restore normal operation: %d.\n", error);
    return error;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rmi_f01_attention(irq: c_int, ctx: *mut c_void) -> irqreturn_t {
    static irqreturn_t rmi_f01_attention(int irq, void *ctx)
    {
    struct rmi_function *fn = ctx;
    struct rmi_device *rmi_dev = fn.rmi_dev;
    int error;
    u8 device_status;
    error = rmi_read(rmi_dev, fn.fd.data_base_addr, &device_status);
    if (error) {
    dev_err(&fn.dev,
    "Failed to read device status: %d.\n", error);
    return IRQ_RETVAL(error);
    }
    if (RMI_F01_STATUS_BOOTLOADER(device_status))
    dev_warn(&fn.dev,
    "Device in bootloader mode, please update firmware\n");
    if (RMI_F01_STATUS_UNCONFIGURED(device_status)) {
    dev_warn(&fn.dev, "Device reset detected.\n");
    error = rmi_dev.driver.reset_handler(rmi_dev);
    if (error) {
    dev_err(&fn.dev, "Device reset failed: %d\n", error);
    return IRQ_RETVAL(error);
    }
    }
    return IRQ_HANDLED;
    }
    struct rmi_function_handler rmi_f01_handler = {
    .driver = {
    .name	= "rmi4_f01",
//
// Do not allow user unbinding F01 as it is critical
// function.
//
    .suppress_bind_attrs = true,
    },
    .func		= 0x01,
    .probe		= rmi_f01_probe,
    .remove		= rmi_f01_remove,
    .config		= rmi_f01_config,
    .attention	= rmi_f01_attention,
    .suspend	= rmi_f01_suspend,
    .resume		= rmi_f01_resume,
    };
