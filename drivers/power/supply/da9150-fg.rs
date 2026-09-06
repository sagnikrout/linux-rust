//! Automatically rewritten from C to Rust
//! Source: drivers/power/supply/da9150-fg.c
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
// DA9150 Fuel-Gauge Driver
//
// Copyright (c) 2015 Dialog Semiconductor
//
// Author: Adam Thomson <Adam.Thomson.Opensource@diasemi.com>
//

// Core2Wire

pub const DA9150_QIF_CODE_MASK: c_uint = 0x7F;
pub const DA9150_QIF_BYTE_SIZE: c_int = 8;
pub const DA9150_QIF_BYTE_MASK: c_uint = 0xFF;
pub const DA9150_QIF_SHORT_SIZE: c_int = 2;
pub const DA9150_QIF_LONG_SIZE: c_int = 4;
// QIF Codes
pub const DA9150_QIF_UAVG: c_int = 6;

pub const DA9150_QIF_IAVG: c_int = 8;

pub const DA9150_QIF_NTCAVG: c_int = 12;

pub const DA9150_QIF_SHUNT_VAL: c_int = 36;

pub const DA9150_QIF_SD_GAIN: c_int = 38;

pub const DA9150_QIF_FCC_MAH: c_int = 40;

pub const DA9150_QIF_SOC_PCT: c_int = 43;

pub const DA9150_QIF_CHARGE_LIMIT: c_int = 44;

pub const DA9150_QIF_DISCHARGE_LIMIT: c_int = 45;

pub const DA9150_QIF_FW_MAIN_VER: c_int = 118;

pub const DA9150_QIF_E_FG_STATUS: c_int = 126;

pub const DA9150_QIF_SYNC: c_int = 127;

pub const DA9150_QIF_MAX_CODES: c_int = 128;
// QIF Sync Timeout
pub const DA9150_QIF_SYNC_TIMEOUT: c_int = 1000;
pub const DA9150_QIF_SYNC_RETRIES: c_int = 10;
// QIF E_FG_STATUS

    (DA9150_FG_IRQ_LOW_SOC_MASK | DA9150_FG_IRQ_HIGH_SOC_MASK)
// Private data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct da9150_fg {
    pub da9150: *mut da9150,
    pub dev: *mut device,
    pub io_lock: mutex,
    pub battery: *mut power_supply,
    pub work: delayed_work,
    pub interval: u32,
    pub warn_soc: c_int,
    pub crit_soc: c_int,
    pub soc: c_int,
}

// Battery Properties
#[no_mangle]
unsafe extern "C" fn da9150_fg_read_attr(fg: *mut da9150_fg, code: u8, size: u8) -> u32 {
    static u32 da9150_fg_read_attr(struct da9150_fg *fg, u8 code, u8 size)
    {
    u8 buf[DA9150_QIF_LONG_SIZE];
    u8 read_addr;
    let mut res: u32 = 0;
    int i;
// Set QIF code (READ mode)
    read_addr = (code & DA9150_QIF_CODE_MASK) | DA9150_QIF_READ;
    da9150_read_qif(fg.da9150, read_addr, size, buf);
    for (i = 0; i < size; ++i)
    res |= (buf[i] << (i * DA9150_QIF_BYTE_SIZE));
    return res;
    }
    static void da9150_fg_write_attr(struct da9150_fg *fg, u8 code, u8 size,
    u32 val)
    {
    u8 buf[DA9150_QIF_LONG_SIZE];
    u8 write_addr;
    int i;
// Set QIF code (WRITE mode)
    write_addr = (code & DA9150_QIF_CODE_MASK) | DA9150_QIF_WRITE;
    for (i = 0; i < size; ++i) {
    buf[i] = (val >> (i * DA9150_QIF_BYTE_SIZE)) &
    DA9150_QIF_BYTE_MASK;
    }
    da9150_write_qif(fg.da9150, write_addr, size, buf);
    }
// Trigger QIF Sync to update QIF readable data
#[no_mangle]
unsafe extern "C" fn da9150_fg_read_sync_start(fg: *mut da9150_fg) {
    static void da9150_fg_read_sync_start(struct da9150_fg *fg)
    {
    let mut i: c_int = 0;
    let mut res: u32 = 0;
    mutex_lock(&fg.io_lock);
// Check if QIF sync already requested, and write to sync if not
    res = da9150_fg_read_attr(fg, DA9150_QIF_SYNC,
    DA9150_QIF_SYNC_SIZE);
    if (res > 0)
    da9150_fg_write_attr(fg, DA9150_QIF_SYNC,
    DA9150_QIF_SYNC_SIZE, 0);
// Wait for sync to complete
    res = 0;
    while ((res == 0) && (i++ < DA9150_QIF_SYNC_RETRIES)) {
    usleep_range(DA9150_QIF_SYNC_TIMEOUT,
    DA9150_QIF_SYNC_TIMEOUT * 2);
    res = da9150_fg_read_attr(fg, DA9150_QIF_SYNC,
    DA9150_QIF_SYNC_SIZE);
    }
// Check if sync completed
    if (res == 0)
    dev_err(fg.dev, "Failed to perform QIF read sync!\n");
    }
//
// Should always be called after QIF sync read has been performed, and all
// attributes required have been accessed.
//
#[no_mangle]
pub unsafe extern "C" fn da9150_fg_read_sync_end(fg: *mut da9150_fg) {
    static inline void da9150_fg_read_sync_end(struct da9150_fg *fg)
    {
    mutex_unlock(&fg.io_lock);
    }
// Sync read of single QIF attribute
#[no_mangle]
unsafe extern "C" fn da9150_fg_read_attr_sync(fg: *mut da9150_fg, code: u8, size: u8) -> u32 {
    static u32 da9150_fg_read_attr_sync(struct da9150_fg *fg, u8 code, u8 size)
    {
    u32 val;
    da9150_fg_read_sync_start(fg);
    val = da9150_fg_read_attr(fg, code, size);
    da9150_fg_read_sync_end(fg);
    return val;
    }
// Wait for QIF Sync, write QIF data and wait for ack
    static void da9150_fg_write_attr_sync(struct da9150_fg *fg, u8 code, u8 size,
    u32 val)
    {
    let mut i: c_int = 0;
    let mut res: u32 = 0, sync_val;
    mutex_lock(&fg.io_lock);
// Check if QIF sync already requested
    res = da9150_fg_read_attr(fg, DA9150_QIF_SYNC,
    DA9150_QIF_SYNC_SIZE);
// Wait for an existing sync to complete
    while ((res == 0) && (i++ < DA9150_QIF_SYNC_RETRIES)) {
    usleep_range(DA9150_QIF_SYNC_TIMEOUT,
    DA9150_QIF_SYNC_TIMEOUT * 2);
    res = da9150_fg_read_attr(fg, DA9150_QIF_SYNC,
    DA9150_QIF_SYNC_SIZE);
    }
    if (res == 0) {
    dev_err(fg.dev, "Timeout waiting for existing QIF sync!\n");
    mutex_unlock(&fg.io_lock);
    return;
    }
// Write value for QIF code
    da9150_fg_write_attr(fg, code, size, val);
// Wait for write acknowledgment
    i = 0;
    sync_val = res;
    while ((res == sync_val) && (i++ < DA9150_QIF_SYNC_RETRIES)) {
    usleep_range(DA9150_QIF_SYNC_TIMEOUT,
    DA9150_QIF_SYNC_TIMEOUT * 2);
    res = da9150_fg_read_attr(fg, DA9150_QIF_SYNC,
    DA9150_QIF_SYNC_SIZE);
    }
    mutex_unlock(&fg.io_lock);
// Check write was actually successful
    if (res != (sync_val + 1))
    dev_err(fg.dev, "Error performing QIF sync write for code %d\n",
    code);
    }
// Power Supply attributes
    static int da9150_fg_capacity(struct da9150_fg *fg,
    union power_supply_propval *val)
    {
    val.intval = da9150_fg_read_attr_sync(fg, DA9150_QIF_SOC_PCT,
    DA9150_QIF_SOC_PCT_SIZE);
    if (val.intval > 100)
    val.intval = 100;
    return 0;
    }
    static int da9150_fg_current_avg(struct da9150_fg *fg,
    union power_supply_propval *val)
    {
    u32 iavg, sd_gain, shunt_val;
    u64 div, res;
    da9150_fg_read_sync_start(fg);
    iavg = da9150_fg_read_attr(fg, DA9150_QIF_IAVG,
    DA9150_QIF_IAVG_SIZE);
    shunt_val = da9150_fg_read_attr(fg, DA9150_QIF_SHUNT_VAL,
    DA9150_QIF_SHUNT_VAL_SIZE);
    sd_gain = da9150_fg_read_attr(fg, DA9150_QIF_SD_GAIN,
    DA9150_QIF_SD_GAIN_SIZE);
    da9150_fg_read_sync_end(fg);
    div = 65536ULL * sd_gain * shunt_val;
    do_div(div, 1000000);
    res = 1000000ULL * iavg;
    do_div(res, div);
    val.intval = (int) res;
    return 0;
    }
    static int da9150_fg_voltage_avg(struct da9150_fg *fg,
    union power_supply_propval *val)
    {
    u64 res;
    val.intval = da9150_fg_read_attr_sync(fg, DA9150_QIF_UAVG,
    DA9150_QIF_UAVG_SIZE);
    res = (u64) (val.intval * 186ULL);
    do_div(res, 10000);
    val.intval = (int) res;
    return 0;
    }
    static int da9150_fg_charge_full(struct da9150_fg *fg,
    union power_supply_propval *val)
    {
    val.intval = da9150_fg_read_attr_sync(fg, DA9150_QIF_FCC_MAH,
    DA9150_QIF_FCC_MAH_SIZE);
    val.intval = val.intval * 1000;
    return 0;
    }
//
// Temperature reading from device is only valid if battery/system provides
// valid NTC to associated pin of DA9150 chip.
//
    static int da9150_fg_temp(struct da9150_fg *fg,
    union power_supply_propval *val)
    {
    val.intval = da9150_fg_read_attr_sync(fg, DA9150_QIF_NTCAVG,
    DA9150_QIF_NTCAVG_SIZE);
    val.intval = (val.intval * 10) / 1048576;
    return 0;
    }
    static enum power_supply_property da9150_fg_props[] = {
    POWER_SUPPLY_PROP_CAPACITY,
    POWER_SUPPLY_PROP_CURRENT_AVG,
    POWER_SUPPLY_PROP_VOLTAGE_AVG,
    POWER_SUPPLY_PROP_CHARGE_FULL,
    POWER_SUPPLY_PROP_TEMP,
    };
    static int da9150_fg_get_prop(struct power_supply *psy,
    enum power_supply_property psp,
    union power_supply_propval *val)
    {
    struct da9150_fg *fg = dev_get_drvdata(psy.dev.parent);
    int ret;
    switch (psp) {
    case POWER_SUPPLY_PROP_CAPACITY:
    ret = da9150_fg_capacity(fg, val);
    break;
    case POWER_SUPPLY_PROP_CURRENT_AVG:
    ret = da9150_fg_current_avg(fg, val);
    break;
    case POWER_SUPPLY_PROP_VOLTAGE_AVG:
    ret = da9150_fg_voltage_avg(fg, val);
    break;
    case POWER_SUPPLY_PROP_CHARGE_FULL:
    ret = da9150_fg_charge_full(fg, val);
    break;
    case POWER_SUPPLY_PROP_TEMP:
    ret = da9150_fg_temp(fg, val);
    break;
    default:
    ret = -EINVAL;
    break;
    }
    return ret;
    }
// Repeated SOC check
#[no_mangle]
unsafe extern "C" fn da9150_fg_soc_changed(fg: *mut da9150_fg) -> bool {
    static bool da9150_fg_soc_changed(struct da9150_fg *fg)
    {
    union power_supply_propval val;
    da9150_fg_capacity(fg, &val);
    if (val.intval != fg.soc) {
    fg.soc = val.intval;
    return true;
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn da9150_fg_work(work: *mut work_struct) {
    static void da9150_fg_work(struct work_struct *work)
    {
    struct da9150_fg *fg = container_of(work, struct da9150_fg, work.work);
// Report if SOC has changed
    if (da9150_fg_soc_changed(fg))
    power_supply_changed(fg.battery);
    schedule_delayed_work(&fg.work, msecs_to_jiffies(fg.interval));
    }
// SOC level event configuration
#[no_mangle]
unsafe extern "C" fn da9150_fg_soc_event_config(fg: *mut da9150_fg) {
    static void da9150_fg_soc_event_config(struct da9150_fg *fg)
    {
    int soc;
    soc = da9150_fg_read_attr_sync(fg, DA9150_QIF_SOC_PCT,
    DA9150_QIF_SOC_PCT_SIZE);
    if (soc > fg.warn_soc) {
// If SOC > warn level, set discharge warn level event
    da9150_fg_write_attr_sync(fg, DA9150_QIF_DISCHARGE_LIMIT,
    DA9150_QIF_DISCHARGE_LIMIT_SIZE,
    fg.warn_soc + 1);
    } else if ((soc <= fg.warn_soc) && (soc > fg.crit_soc)) {
//
// If SOC <= warn level, set discharge crit level event,
// and set charge warn level event.
//
    da9150_fg_write_attr_sync(fg, DA9150_QIF_DISCHARGE_LIMIT,
    DA9150_QIF_DISCHARGE_LIMIT_SIZE,
    fg.crit_soc + 1);
    da9150_fg_write_attr_sync(fg, DA9150_QIF_CHARGE_LIMIT,
    DA9150_QIF_CHARGE_LIMIT_SIZE,
    fg.warn_soc);
    } else if (soc <= fg.crit_soc) {
// If SOC <= crit level, set charge crit level event
    da9150_fg_write_attr_sync(fg, DA9150_QIF_CHARGE_LIMIT,
    DA9150_QIF_CHARGE_LIMIT_SIZE,
    fg.crit_soc);
    }
    }
#[no_mangle]
unsafe extern "C" fn da9150_fg_irq(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t da9150_fg_irq(int irq, void *data)
    {
    struct da9150_fg *fg = data;
    u32 e_fg_status;
// Read FG IRQ status info
    e_fg_status = da9150_fg_read_attr(fg, DA9150_QIF_E_FG_STATUS,
    DA9150_QIF_E_FG_STATUS_SIZE);
// Handle warning/critical threhold events
    if (e_fg_status & DA9150_FG_IRQ_SOC_MASK)
    da9150_fg_soc_event_config(fg);
// Clear any FG IRQs
    da9150_fg_write_attr(fg, DA9150_QIF_E_FG_STATUS,
    DA9150_QIF_E_FG_STATUS_SIZE, e_fg_status);
    return IRQ_HANDLED;
    }
    static struct da9150_fg_pdata *da9150_fg_dt_pdata(struct device *dev)
    {
    struct device_node *fg_node = dev.of_node;
    struct da9150_fg_pdata *pdata;
    pdata = devm_kzalloc(dev, sizeof(struct da9150_fg_pdata), GFP_KERNEL);
    if (!pdata)
    return core::ptr::null_mut();
    of_property_read_u32(fg_node, "dlg,update-interval",
    &pdata.update_interval);
    of_property_read_u8(fg_node, "dlg,warn-soc-level",
    &pdata.warn_soc_lvl);
    of_property_read_u8(fg_node, "dlg,crit-soc-level",
    &pdata.crit_soc_lvl);
    return pdata;
    }
    static const struct power_supply_desc fg_desc = {
    .name		= "da9150-fg",
    .type		= POWER_SUPPLY_TYPE_BATTERY,
    .properties	= da9150_fg_props,
    .num_properties	= ARRAY_SIZE(da9150_fg_props),
    .get_property	= da9150_fg_get_prop,
    };
#[no_mangle]
unsafe extern "C" fn da9150_fg_probe(pdev: *mut platform_device) -> c_int {
    static int da9150_fg_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct da9150 *da9150 = dev_get_drvdata(dev.parent);
    struct da9150_fg_pdata *fg_pdata = dev_get_platdata(dev);
    struct da9150_fg *fg;
    int ver, irq, ret = 0;
    fg = devm_kzalloc(dev, sizeof(*fg), GFP_KERNEL);
    if (fg == core::ptr::null_mut())
    return -ENOMEM;
    platform_set_drvdata(pdev, fg);
    fg.da9150 = da9150;
    fg.dev = dev;
    mutex_init(&fg.io_lock);
// Enable QIF
    da9150_set_bits(da9150, DA9150_CORE2WIRE_CTRL_A, DA9150_FG_QIF_EN_MASK,
    DA9150_FG_QIF_EN_MASK);
    fg.battery = devm_power_supply_register(dev, &fg_desc, core::ptr::null_mut());
    if (IS_ERR(fg.battery)) {
    ret = PTR_ERR(fg.battery);
    return ret;
    }
    ver = da9150_fg_read_attr(fg, DA9150_QIF_FW_MAIN_VER,
    DA9150_QIF_FW_MAIN_VER_SIZE);
    dev_info(dev, "Version: 0x%x\n", ver);
// Handle DT data if provided
    if (dev.of_node) {
    fg_pdata = da9150_fg_dt_pdata(dev);
    dev.platform_data = fg_pdata;
    }
// Handle any pdata provided
    if (fg_pdata) {
    fg.interval = fg_pdata.update_interval;
    if (fg_pdata.warn_soc_lvl > 100)
    dev_warn(dev, "Invalid SOC warning level provided, Ignoring");
    else
    fg.warn_soc = fg_pdata.warn_soc_lvl;
    if ((fg_pdata.crit_soc_lvl > 100) ||
    (fg_pdata.crit_soc_lvl >= fg_pdata.warn_soc_lvl))
    dev_warn(dev, "Invalid SOC critical level provided, Ignoring");
    else
    fg.crit_soc = fg_pdata.crit_soc_lvl;
    }
// Configure initial SOC level events
    da9150_fg_soc_event_config(fg);
//
// If an interval period has been provided then setup repeating
// work for reporting data updates.
//
    if (fg.interval) {
    ret = devm_delayed_work_autocancel(dev, &fg.work,
    da9150_fg_work);
    if (ret) {
    dev_err(dev, "Failed to init work\n");
    return ret;
    }
    schedule_delayed_work(&fg.work,
    msecs_to_jiffies(fg.interval));
    }
// Register IRQ
    irq = platform_get_irq_byname(pdev, "FG");
    if (irq < 0)
    return irq;
    ret = devm_request_threaded_irq(dev, irq, core::ptr::null_mut(), da9150_fg_irq,
    IRQF_ONESHOT, "FG", fg);
    if (ret)
    return ret;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn da9150_fg_resume(pdev: *mut platform_device) -> c_int {
    static int da9150_fg_resume(struct platform_device *pdev)
    {
    struct da9150_fg *fg = platform_get_drvdata(pdev);
//
// Trigger SOC check to happen now so as to indicate any value change
// since last check before suspend.
//
    if (fg.interval)
    flush_delayed_work(&fg.work);
    return 0;
    }
    static struct platform_driver da9150_fg_driver = {
    .driver = {
    .name = "da9150-fuel-gauge",
    },
    .probe = da9150_fg_probe,
    .resume = da9150_fg_resume,
    };
    module_platform_driver(da9150_fg_driver);
    MODULE_DESCRIPTION("Fuel-Gauge Driver for DA9150");
    MODULE_AUTHOR("Adam Thomson <Adam.Thomson.Opensource@diasemi.com>");
    MODULE_LICENSE("GPL");
