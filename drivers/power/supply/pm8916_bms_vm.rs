//! Automatically rewritten from C to Rust
//! Source: drivers/power/supply/pm8916_bms_vm.c
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
// Copyright (c) 2023, Nikita Travkin <nikita@trvn.ru>
//

pub const PM8916_PERPH_TYPE: c_uint = 0x04;
pub const PM8916_BMS_VM_TYPE: c_uint = 0x020D;
pub const PM8916_SEC_ACCESS: c_uint = 0xD0;
pub const PM8916_SEC_MAGIC: c_uint = 0xA5;
pub const PM8916_BMS_VM_STATUS1: c_uint = 0x08;

pub const PM8916_BMS_VM_FSM_STATE_S2: c_uint = 0x2;
pub const PM8916_BMS_VM_MODE_CTL: c_uint = 0x40;

pub const PM8916_BMS_VM_EN_CTL: c_uint = 0x46;

pub const PM8916_BMS_VM_FIFO_LENGTH_CTL: c_uint = 0x47;
pub const PM8916_BMS_VM_S1_SAMPLE_INTERVAL_CTL: c_uint = 0x55;
pub const PM8916_BMS_VM_S2_SAMPLE_INTERVAL_CTL: c_uint = 0x56;
pub const PM8916_BMS_VM_S3_S7_OCV_DATA0: c_uint = 0x6A;
pub const PM8916_BMS_VM_BMS_FIFO_REG_0_LSB: c_uint = 0xC0;
// Using only 1 fifo is broken in hardware

pub const PM8916_BMS_VM_S1_SAMPLE_INTERVAL: c_int = 10;
pub const PM8916_BMS_VM_S2_SAMPLE_INTERVAL: c_int = 10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pm8916_bms_vm_battery {
    pub dev: *mut device,
    pub battery: *mut power_supply,
    pub info: *mut power_supply_battery_info,
    pub regmap: *mut regmap,
    pub reg: c_uint,
    pub last_ocv: c_uint,
    pub last_ocv_time: time64_t,
    pub vbat_now: c_uint,
}

    static int pm8916_bms_vm_battery_get_property(struct power_supply *psy,
    enum power_supply_property psp,
    union power_supply_propval *val)
    {
    struct pm8916_bms_vm_battery *bat = power_supply_get_drvdata(psy);
    struct power_supply_battery_info *info = bat.info;
    int supplied;
    switch (psp) {
    case POWER_SUPPLY_PROP_STATUS:
    supplied = power_supply_am_i_supplied(psy);
    if (supplied < 0 && supplied != -ENODEV)
    return supplied;
#[no_mangle]
pub unsafe extern "C" fn if(-ENODEV: supplied && supplied !=) -> else {
    else if (supplied && supplied != -ENODEV)
    val.intval = POWER_SUPPLY_STATUS_CHARGING;
    else
    val.intval = POWER_SUPPLY_STATUS_DISCHARGING;
    return 0;
    case POWER_SUPPLY_PROP_HEALTH:
    if (bat.vbat_now < info.voltage_min_design_uv)
    val.intval = POWER_SUPPLY_HEALTH_DEAD;
#[no_mangle]
pub unsafe extern "C" fn if(info->voltage_max_design_uv: bat->vbat_now >) -> else {
    else if (bat.vbat_now > info.voltage_max_design_uv)
    val.intval = POWER_SUPPLY_HEALTH_OVERVOLTAGE;
    else
    val.intval = POWER_SUPPLY_HEALTH_GOOD;
    return 0;
    case POWER_SUPPLY_PROP_VOLTAGE_NOW:
    val.intval = bat.vbat_now;
    return 0;
    case POWER_SUPPLY_PROP_VOLTAGE_OCV:
//
// Hardware only reliably measures OCV when the system is off or suspended.
// We expose the last known OCV value on boot, invalidating it after 180 seconds.
//
    if (ktime_get_seconds() - bat.last_ocv_time > 180)
    return -ENODATA;
    val.intval = bat.last_ocv;
    return 0;
    default:
    return -EINVAL;
    }
    }
    static enum power_supply_property pm8916_bms_vm_battery_properties[] = {
    POWER_SUPPLY_PROP_STATUS,
    POWER_SUPPLY_PROP_VOLTAGE_NOW,
    POWER_SUPPLY_PROP_VOLTAGE_OCV,
    POWER_SUPPLY_PROP_HEALTH,
    };
#[no_mangle]
unsafe extern "C" fn pm8916_bms_vm_fifo_update_done_irq(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t pm8916_bms_vm_fifo_update_done_irq(int irq, void *data)
    {
    struct pm8916_bms_vm_battery *bat = data;
    u16 vbat_data[PM8916_BMS_VM_FIFO_COUNT];
    int ret;
    ret = regmap_bulk_read(bat.regmap, bat.reg + PM8916_BMS_VM_BMS_FIFO_REG_0_LSB,
    &vbat_data, PM8916_BMS_VM_FIFO_COUNT * 2);
    if (ret)
    return IRQ_HANDLED;
//
// The VM-BMS hardware only collects voltage data and the software
// has to process it to calculate the OCV and SoC. Hardware provides
// up to 8 averaged measurements for software to take in account.
//
// Just use the last measured value for now to report the current
// battery voltage.
//
    bat.vbat_now = vbat_data[PM8916_BMS_VM_FIFO_COUNT - 1] * 300;
    power_supply_changed(bat.battery);
    return IRQ_HANDLED;
    }
    static const struct power_supply_desc pm8916_bms_vm_battery_psy_desc = {
    .name = "pm8916-bms-vm",
    .type = POWER_SUPPLY_TYPE_BATTERY,
    .properties = pm8916_bms_vm_battery_properties,
    .num_properties = ARRAY_SIZE(pm8916_bms_vm_battery_properties),
    .get_property = pm8916_bms_vm_battery_get_property,
    };
#[no_mangle]
unsafe extern "C" fn pm8916_bms_vm_battery_probe(pdev: *mut platform_device) -> c_int {
    static int pm8916_bms_vm_battery_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct pm8916_bms_vm_battery *bat;
    let mut psy_cfg: power_supply_config = {};
    int ret, irq;
    unsigned int tmp;
    bat = devm_kzalloc(dev, sizeof(*bat), GFP_KERNEL);
    if (!bat)
    return -ENOMEM;
    bat.dev = dev;
    bat.regmap = dev_get_regmap(pdev.dev.parent, core::ptr::null_mut());
    if (!bat.regmap)
    return -ENODEV;
    ret = device_property_read_u32(dev, "reg", &bat.reg);
    if (ret < 0)
    return -EINVAL;
    ret = regmap_bulk_read(bat.regmap, bat.reg + PM8916_PERPH_TYPE, &tmp, 2);
    if (ret)
    goto comm_error;
    if (tmp != PM8916_BMS_VM_TYPE)
    return dev_err_probe(dev, -ENODEV, "Device reported wrong type: 0x%X\n", tmp);
    ret = regmap_write(bat.regmap, bat.reg + PM8916_BMS_VM_S1_SAMPLE_INTERVAL_CTL,
    PM8916_BMS_VM_S1_SAMPLE_INTERVAL);
    if (ret)
    goto comm_error;
    ret = regmap_write(bat.regmap, bat.reg + PM8916_BMS_VM_S2_SAMPLE_INTERVAL_CTL,
    PM8916_BMS_VM_S2_SAMPLE_INTERVAL);
    if (ret)
    goto comm_error;
    ret = regmap_write(bat.regmap, bat.reg + PM8916_BMS_VM_FIFO_LENGTH_CTL,
    PM8916_BMS_VM_FIFO_COUNT << 4 | PM8916_BMS_VM_FIFO_COUNT);
    if (ret)
    goto comm_error;
    ret = regmap_write(bat.regmap,
    bat.reg + PM8916_BMS_VM_EN_CTL, PM8916_BMS_ENABLED);
    if (ret)
    goto comm_error;
    ret = regmap_bulk_read(bat.regmap,
    bat.reg + PM8916_BMS_VM_S3_S7_OCV_DATA0, &tmp, 2);
    if (ret)
    goto comm_error;
    bat.last_ocv_time = ktime_get_seconds();
    bat.last_ocv = tmp * 300;
    bat.vbat_now = bat.last_ocv;
    psy_cfg.drv_data = bat;
    psy_cfg.fwnode = dev_fwnode(dev);
    bat.battery = devm_power_supply_register(dev, &pm8916_bms_vm_battery_psy_desc, &psy_cfg);
    if (IS_ERR(bat.battery))
    return dev_err_probe(dev, PTR_ERR(bat.battery), "Unable to register battery\n");
    ret = power_supply_get_battery_info(bat.battery, &bat.info);
    if (ret)
    return dev_err_probe(dev, ret, "Unable to get battery info\n");
    irq = platform_get_irq_byname(pdev, "fifo");
    if (irq < 0)
    return irq;
    ret = devm_request_threaded_irq(dev, irq, core::ptr::null_mut(), pm8916_bms_vm_fifo_update_done_irq,
    IRQF_ONESHOT, "pm8916_vm_bms", bat);
    if (ret)
    return ret;
    platform_set_drvdata(pdev, bat);
    return 0;
    comm_error:
    return dev_err_probe(dev, ret, "Unable to communicate with device\n");
    }
#[no_mangle]
unsafe extern "C" fn pm8916_bms_vm_battery_suspend(pdev: *mut platform_device, state: pm_message_t) -> c_int {
    static int pm8916_bms_vm_battery_suspend(struct platform_device *pdev, pm_message_t state)
    {
    struct pm8916_bms_vm_battery *bat = platform_get_drvdata(pdev);
    int ret;
//
// Due to a hardware quirk the FSM doesn't switch states normally.
// Instead we unlock the debug registers and force S3 (Measure OCV/Sleep)
// mode every time we suspend.
//
    ret = regmap_write(bat.regmap,
    bat.reg + PM8916_SEC_ACCESS, PM8916_SEC_MAGIC);
    if (ret)
    goto error;
    ret = regmap_write(bat.regmap,
    bat.reg + PM8916_BMS_VM_MODE_CTL, PM8916_BMS_VM_MODE_FORCE_S3);
    if (ret)
    goto error;
    return 0;
    error:
    dev_err(bat.dev, "Failed to force S3 mode: %pe\n", ERR_PTR(ret));
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn pm8916_bms_vm_battery_resume(pdev: *mut platform_device) -> c_int {
    static int pm8916_bms_vm_battery_resume(struct platform_device *pdev)
    {
    struct pm8916_bms_vm_battery *bat = platform_get_drvdata(pdev);
    int ret;
    unsigned int tmp;
    ret = regmap_bulk_read(bat.regmap,
    bat.reg + PM8916_BMS_VM_S3_S7_OCV_DATA0, &tmp, 2);
    bat.last_ocv_time = ktime_get_seconds();
    bat.last_ocv = tmp * 300;
    ret = regmap_write(bat.regmap,
    bat.reg + PM8916_SEC_ACCESS, PM8916_SEC_MAGIC);
    if (ret)
    goto error;
    ret = regmap_write(bat.regmap,
    bat.reg + PM8916_BMS_VM_MODE_CTL, PM8916_BMS_VM_MODE_NORMAL);
    if (ret)
    goto error;
    return 0;
    error:
    dev_err(bat.dev, "Failed to return normal mode: %pe\n", ERR_PTR(ret));
    return ret;
    }
    static const struct of_device_id pm8916_bms_vm_battery_of_match[] = {
    { .compatible = "qcom,pm8916-bms-vm", },
    {}
    };
    MODULE_DEVICE_TABLE(of, pm8916_bms_vm_battery_of_match);
    static struct platform_driver pm8916_bms_vm_battery_driver = {
    .driver = {
    .name = "pm8916-bms-vm",
    .of_match_table = pm8916_bms_vm_battery_of_match,
    },
    .probe = pm8916_bms_vm_battery_probe,
    .suspend = pm8916_bms_vm_battery_suspend,
    .resume = pm8916_bms_vm_battery_resume,
    };
    module_platform_driver(pm8916_bms_vm_battery_driver);
    MODULE_DESCRIPTION("pm8916 BMS-VM driver");
    MODULE_AUTHOR("Nikita Travkin <nikita@trvn.ru>");
    MODULE_LICENSE("GPL");
