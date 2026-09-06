//! Automatically rewritten from C to Rust
//! Source: drivers/power/supply/max17040_battery.c
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
// max17040_battery.c
// fuel-gauge systems for lithium-ion (Li+) batteries
//
// Copyright (C) 2009 Samsung Electronics
// Minkyu Kang <mk7.kang@samsung.com>

pub const MAX17040_VCELL: c_uint = 0x02;
pub const MAX17040_SOC: c_uint = 0x04;
pub const MAX17040_MODE: c_uint = 0x06;
pub const MAX17040_VER: c_uint = 0x08;
pub const MAX17040_CONFIG: c_uint = 0x0C;
pub const MAX17040_STATUS: c_uint = 0x1A;
pub const MAX17040_CMD: c_uint = 0xFE;
pub const MAX17040_DELAY: c_int = 1000;
pub const MAX17040_BATTERY_FULL: c_int = 95;
pub const MAX17040_RCOMP_DEFAULT: c_uint = 0x9700;
pub const MAX17040_ATHD_MASK: c_uint = 0x3f;
pub const MAX17040_ALSC_MASK: c_uint = 0x40;
pub const MAX17040_ATHD_DEFAULT_POWER_UP: c_int = 4;
pub const MAX17040_STATUS_HD_MASK: c_uint = 0x1000;
pub const MAX17040_STATUS_SC_MASK: c_uint = 0x2000;
pub const MAX17040_CFG_RCOMP_MASK: c_uint = 0xff00;
    enum chip_id {
    ID_MAX17040,
    ID_MAX17041,
    ID_MAX17043,
    ID_MAX17044,
    ID_MAX17048,
    ID_MAX17049,
    ID_MAX17058,
    ID_MAX17059,
    };
// values that differ by chip_id
#[repr(C)]
#[derive(Copy, Clone)]
pub struct chip_data {
    pub reset_val: u16,
    pub vcell_shift: u16,
    pub vcell_mul: u16,
    pub vcell_div: u16,
    pub has_low_soc_alert: u8,
    pub rcomp_bytes: u8,
    pub has_soc_alert: u8,
}

    static struct chip_data max17040_family[] = {
    [ID_MAX17040] = {
    .reset_val = 0x0054,
    .vcell_shift = 4,
    .vcell_mul = 1250,
    .vcell_div = 1,
    .has_low_soc_alert = 0,
    .rcomp_bytes = 2,
    .has_soc_alert = 0,
    },
    [ID_MAX17041] = {
    .reset_val = 0x0054,
    .vcell_shift = 4,
    .vcell_mul = 2500,
    .vcell_div = 1,
    .has_low_soc_alert = 0,
    .rcomp_bytes = 2,
    .has_soc_alert = 0,
    },
    [ID_MAX17043] = {
    .reset_val = 0x0054,
    .vcell_shift = 4,
    .vcell_mul = 1250,
    .vcell_div = 1,
    .has_low_soc_alert = 1,
    .rcomp_bytes = 1,
    .has_soc_alert = 0,
    },
    [ID_MAX17044] = {
    .reset_val = 0x0054,
    .vcell_shift = 4,
    .vcell_mul = 2500,
    .vcell_div = 1,
    .has_low_soc_alert = 1,
    .rcomp_bytes = 1,
    .has_soc_alert = 0,
    },
    [ID_MAX17048] = {
    .reset_val = 0x5400,
    .vcell_shift = 0,
    .vcell_mul = 625,
    .vcell_div = 8,
    .has_low_soc_alert = 1,
    .rcomp_bytes = 1,
    .has_soc_alert = 1,
    },
    [ID_MAX17049] = {
    .reset_val = 0x5400,
    .vcell_shift = 0,
    .vcell_mul = 625,
    .vcell_div = 4,
    .has_low_soc_alert = 1,
    .rcomp_bytes = 1,
    .has_soc_alert = 1,
    },
    [ID_MAX17058] = {
    .reset_val = 0x5400,
    .vcell_shift = 0,
    .vcell_mul = 625,
    .vcell_div = 8,
    .has_low_soc_alert = 1,
    .rcomp_bytes = 1,
    .has_soc_alert = 0,
    },
    [ID_MAX17059] = {
    .reset_val = 0x5400,
    .vcell_shift = 0,
    .vcell_mul = 625,
    .vcell_div = 4,
    .has_low_soc_alert = 1,
    .rcomp_bytes = 1,
    .has_soc_alert = 0,
    },
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct max17040_chip {
    pub client: *mut i2c_client,
    pub regmap: *mut regmap,
    pub work: delayed_work,
    pub battery: *mut power_supply,
    pub data: chip_data,
    pub channel_temp: *mut iio_channel,
// battery capacity
    pub soc: c_int,
// Low alert threshold from 32% to 1% of the State of Charge
    pub low_soc_alert: u32,
// some devices return twice the capacity
    pub quirk_double_soc: bool,
// higher 8 bits for 17043+, 16 bits for 17040,41
    pub rcomp: u16,
}

#[no_mangle]
unsafe extern "C" fn max17040_reset(chip: *mut max17040_chip) -> c_int {
    static int max17040_reset(struct max17040_chip *chip)
    {
    return regmap_write(chip.regmap, MAX17040_CMD, chip.data.reset_val);
    }
#[no_mangle]
unsafe extern "C" fn max17040_set_low_soc_alert(chip: *mut max17040_chip, level: u32) -> c_int {
    static int max17040_set_low_soc_alert(struct max17040_chip *chip, u32 level)
    {
    level = 32 - level * (chip.quirk_double_soc ? 2 : 1);
    return regmap_update_bits(chip.regmap, MAX17040_CONFIG,
    MAX17040_ATHD_MASK, level);
    }
#[no_mangle]
unsafe extern "C" fn max17040_set_soc_alert(chip: *mut max17040_chip, enable: bool) -> c_int {
    static int max17040_set_soc_alert(struct max17040_chip *chip, bool enable)
    {
    return regmap_update_bits(chip.regmap, MAX17040_CONFIG,
    MAX17040_ALSC_MASK, enable ? MAX17040_ALSC_MASK : 0);
    }
#[no_mangle]
unsafe extern "C" fn max17040_set_rcomp(chip: *mut max17040_chip, rcomp: u16) -> c_int {
    static int max17040_set_rcomp(struct max17040_chip *chip, u16 rcomp)
    {
    u16 mask = chip.data.rcomp_bytes == 2 ?
    0xffff : MAX17040_CFG_RCOMP_MASK;
    return regmap_update_bits(chip.regmap, MAX17040_CONFIG, mask, rcomp);
    }
#[no_mangle]
unsafe extern "C" fn max17040_raw_vcell_to_uvolts(chip: *mut max17040_chip, vcell: u16) -> c_int {
    static int max17040_raw_vcell_to_uvolts(struct max17040_chip *chip, u16 vcell)
    {
    struct chip_data *d = &chip.data;
    return (vcell >> d.vcell_shift) * d.vcell_mul / d.vcell_div;
    }
#[no_mangle]
unsafe extern "C" fn max17040_get_vcell(chip: *mut max17040_chip) -> c_int {
    static int max17040_get_vcell(struct max17040_chip *chip)
    {
    u32 vcell;
    int ret;
    ret = regmap_read(chip.regmap, MAX17040_VCELL, &vcell);
    if (ret)
    return ret;
    return max17040_raw_vcell_to_uvolts(chip, vcell);
    }
#[no_mangle]
unsafe extern "C" fn max17040_get_soc(chip: *mut max17040_chip) -> c_int {
    static int max17040_get_soc(struct max17040_chip *chip)
    {
    u32 soc;
    int ret;
    ret = regmap_read(chip.regmap, MAX17040_SOC, &soc);
    if (ret)
    return ret;
    return soc >> (chip.quirk_double_soc ? 9 : 8);
    }
#[no_mangle]
unsafe extern "C" fn max17040_get_version(chip: *mut max17040_chip) -> c_int {
    static int max17040_get_version(struct max17040_chip *chip)
    {
    int ret;
    u32 version;
    ret = regmap_read(chip.regmap, MAX17040_VER, &version);
    return ret ? ret : version;
    }
#[no_mangle]
unsafe extern "C" fn max17040_get_online(chip: *mut max17040_chip) -> c_int {
    static int max17040_get_online(struct max17040_chip *chip)
    {
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn max17040_get_of_data(chip: *mut max17040_chip) -> c_int {
    static int max17040_get_of_data(struct max17040_chip *chip)
    {
    struct device *dev = &chip.client.dev;
    struct chip_data *data = &max17040_family[
    (uintptr_t) of_device_get_match_data(dev)];
    int rcomp_len;
    u8 rcomp[2];
    chip.quirk_double_soc = device_property_read_bool(dev,
    "maxim,double-soc");
    chip.low_soc_alert = MAX17040_ATHD_DEFAULT_POWER_UP;
    device_property_read_u32(dev,
    "maxim,alert-low-soc-level",
    &chip.low_soc_alert);
    if (chip.low_soc_alert <= 0 ||
    chip.low_soc_alert > (chip.quirk_double_soc ? 16 : 32)) {
    dev_err(dev, "maxim,alert-low-soc-level out of bounds\n");
    return -EINVAL;
    }
    rcomp_len = device_property_count_u8(dev, "maxim,rcomp");
    chip.rcomp = MAX17040_RCOMP_DEFAULT;
    if (rcomp_len == data.rcomp_bytes) {
    if (!device_property_read_u8_array(dev, "maxim,rcomp",
    rcomp, rcomp_len))
    chip.rcomp = rcomp_len == 2 ? rcomp[0] << 8 | rcomp[1] :
    rcomp[0] << 8;
    } else if (rcomp_len > 0) {
    dev_err(dev, "maxim,rcomp has incorrect length\n");
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn max17040_check_changes(chip: *mut max17040_chip) {
    static void max17040_check_changes(struct max17040_chip *chip)
    {
    int soc;
    soc = max17040_get_soc(chip);
    if (soc >= 0)
    chip.soc = soc;
    }
#[no_mangle]
unsafe extern "C" fn max17040_queue_work(chip: *mut max17040_chip) {
    static void max17040_queue_work(struct max17040_chip *chip)
    {
    queue_delayed_work(system_power_efficient_wq, &chip.work,
    MAX17040_DELAY);
    }
#[no_mangle]
unsafe extern "C" fn max17040_stop_work(data: *mut c_void) {
    static void max17040_stop_work(void *data)
    {
    struct max17040_chip *chip = data;
    cancel_delayed_work_sync(&chip.work);
    }
#[no_mangle]
unsafe extern "C" fn max17040_work(work: *mut work_struct) {
    static void max17040_work(struct work_struct *work)
    {
    struct max17040_chip *chip;
    int last_soc;
    chip = container_of(work, struct max17040_chip, work.work);
// store SOC to check changes
    last_soc = chip.soc;
    max17040_check_changes(chip);
// check changes and send uevent
    if (last_soc != chip.soc)
    power_supply_changed(chip.battery);
    max17040_queue_work(chip);
    }
// Returns true if alert cause was SOC change, not low SOC
#[no_mangle]
unsafe extern "C" fn max17040_handle_soc_alert(chip: *mut max17040_chip) -> bool {
    static bool max17040_handle_soc_alert(struct max17040_chip *chip)
    {
    let mut ret: bool = true;
    u32 data;
    regmap_read(chip.regmap, MAX17040_STATUS, &data);
    if (data & MAX17040_STATUS_HD_MASK) {
// this alert was caused by low soc
    ret = false;
    }
    if (data & MAX17040_STATUS_SC_MASK) {
// soc change bit -- deassert to mark as handled
    regmap_write(chip.regmap, MAX17040_STATUS,
    data & ~MAX17040_STATUS_SC_MASK);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn max17040_thread_handler(id: c_int, dev: *mut c_void) -> irqreturn_t {
    static irqreturn_t max17040_thread_handler(int id, void *dev)
    {
    struct max17040_chip *chip = dev;
    if (!(chip.data.has_soc_alert && max17040_handle_soc_alert(chip)))
    dev_warn(&chip.client.dev, "IRQ: Alert battery low level\n");
// read registers
    max17040_check_changes(chip);
// send uevent
    power_supply_changed(chip.battery);
// reset alert bit
    max17040_set_low_soc_alert(chip, chip.low_soc_alert);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn max17040_enable_alert_irq(chip: *mut max17040_chip) -> c_int {
    static int max17040_enable_alert_irq(struct max17040_chip *chip)
    {
    struct i2c_client *client = chip.client;
    int ret;
    ret = devm_request_threaded_irq(&client.dev, client.irq, core::ptr::null_mut(),
    max17040_thread_handler, IRQF_ONESHOT,
    chip.battery.desc.name, chip);
    return ret;
    }
    static int max17040_prop_writeable(struct power_supply *psy,
    enum power_supply_property psp)
    {
    switch (psp) {
    case POWER_SUPPLY_PROP_CAPACITY_ALERT_MIN:
    return 1;
    default:
    return 0;
    }
    }
    static int max17040_set_property(struct power_supply *psy,
    enum power_supply_property psp,
    const union power_supply_propval *val)
    {
    struct max17040_chip *chip = power_supply_get_drvdata(psy);
    int ret;
    switch (psp) {
    case POWER_SUPPLY_PROP_CAPACITY_ALERT_MIN:
// alert threshold can be programmed from 1% up to 16/32%
    if ((val.intval < 1) ||
    (val.intval > (chip.quirk_double_soc ? 16 : 32))) {
    ret = -EINVAL;
    break;
    }
    ret = max17040_set_low_soc_alert(chip, val.intval);
    chip.low_soc_alert = val.intval;
    break;
    default:
    ret = -EINVAL;
    }
    return ret;
    }
    static int max17040_get_property(struct power_supply *psy,
    enum power_supply_property psp,
    union power_supply_propval *val)
    {
    struct max17040_chip *chip = power_supply_get_drvdata(psy);
    int ret;
    switch (psp) {
    case POWER_SUPPLY_PROP_ONLINE:
    case POWER_SUPPLY_PROP_PRESENT:
    val.intval = max17040_get_online(chip);
    break;
    case POWER_SUPPLY_PROP_VOLTAGE_NOW:
    ret = max17040_get_vcell(chip);
    if (ret < 0)
    return ret;
    val.intval = ret;
    break;
    case POWER_SUPPLY_PROP_CAPACITY:
    ret = max17040_get_soc(chip);
    if (ret < 0)
    return ret;
    val.intval = ret;
    break;
    case POWER_SUPPLY_PROP_CAPACITY_ALERT_MIN:
    val.intval = chip.low_soc_alert;
    break;
    case POWER_SUPPLY_PROP_STATUS:
    ret = power_supply_get_property_from_supplier(psy, psp, val);
    if (ret == -ENODEV)
    val.intval = POWER_SUPPLY_STATUS_UNKNOWN;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: ret) -> else {
    else if (ret)
    return ret;
    break;
    case POWER_SUPPLY_PROP_TEMP:
    if (!chip.channel_temp)
    return -ENODATA;
    ret = iio_read_channel_processed(chip.channel_temp, &val.intval);
    if (ret)
    return ret;
    val.intval /= 100; /* Convert from milli- to deci-degree */
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
    static const struct regmap_config max17040_regmap = {
    .reg_bits	= 8,
    .reg_stride	= 2,
    .val_bits	= 16,
    .val_format_endian = REGMAP_ENDIAN_BIG,
    };
    static enum power_supply_property max17040_battery_props[] = {
    POWER_SUPPLY_PROP_ONLINE,
    POWER_SUPPLY_PROP_PRESENT,
    POWER_SUPPLY_PROP_VOLTAGE_NOW,
    POWER_SUPPLY_PROP_CAPACITY,
    POWER_SUPPLY_PROP_CAPACITY_ALERT_MIN,
    POWER_SUPPLY_PROP_STATUS,
    POWER_SUPPLY_PROP_TEMP,
    };
    static const struct power_supply_desc max17040_battery_desc = {
    .name			= "battery",
    .type			= POWER_SUPPLY_TYPE_BATTERY,
    .get_property		= max17040_get_property,
    .set_property		= max17040_set_property,
    .property_is_writeable  = max17040_prop_writeable,
    .properties		= max17040_battery_props,
    .num_properties		= ARRAY_SIZE(max17040_battery_props),
    };
#[no_mangle]
unsafe extern "C" fn max17040_probe(client: *mut i2c_client) -> c_int {
    static int max17040_probe(struct i2c_client *client)
    {
    const struct i2c_device_id *id = i2c_client_get_device_id(client);
    let mut psy_cfg: power_supply_config = {};
    struct max17040_chip *chip;
    enum chip_id chip_id;
    let mut enable_irq: bool = false;
    int ret;
    chip = devm_kzalloc(&client.dev, sizeof(*chip), GFP_KERNEL);
    if (!chip)
    return -ENOMEM;
    chip.client = client;
    chip.regmap = devm_regmap_init_i2c(client, &max17040_regmap);
    if (IS_ERR(chip.regmap))
    return PTR_ERR(chip.regmap);
    chip_id = (enum chip_id) id.driver_data;
    if (client.dev.of_node) {
    ret = max17040_get_of_data(chip);
    if (ret)
    return ret;
    chip_id = (uintptr_t)of_device_get_match_data(&client.dev);
    }
    chip.data = max17040_family[chip_id];
    i2c_set_clientdata(client, chip);
    psy_cfg.drv_data = chip;
// Switch to devm_iio_channel_get_optional when available
    chip.channel_temp = devm_iio_channel_get(&client.dev, "temp");
    if (IS_ERR(chip.channel_temp)) {
    ret = PTR_ERR(chip.channel_temp);
    if (ret != -ENODEV)
    return dev_err_probe(&client.dev, PTR_ERR(chip.channel_temp),
    "failed to get temp\n");
    else
    chip.channel_temp = core::ptr::null_mut();
    }
    chip.battery = devm_power_supply_register(&client.dev,
    &max17040_battery_desc, &psy_cfg);
    if (IS_ERR(chip.battery)) {
    dev_err(&client.dev, "failed: power supply register\n");
    return PTR_ERR(chip.battery);
    }
    ret = max17040_get_version(chip);
    if (ret < 0)
    return ret;
    dev_dbg(&chip.client.dev, "MAX17040 Fuel-Gauge Ver 0x%x\n", ret);
    if (chip_id == ID_MAX17040 || chip_id == ID_MAX17041)
    max17040_reset(chip);
    max17040_set_rcomp(chip, chip.rcomp);
// check interrupt
    if (client.irq && chip.data.has_low_soc_alert) {
    ret = max17040_set_low_soc_alert(chip, chip.low_soc_alert);
    if (ret) {
    dev_err(&client.dev,
    "Failed to set low SOC alert: err %d\n", ret);
    return ret;
    }
    enable_irq = true;
    }
    if (client.irq && chip.data.has_soc_alert) {
    ret = max17040_set_soc_alert(chip, 1);
    if (ret) {
    dev_err(&client.dev,
    "Failed to set SOC alert: err %d\n", ret);
    return ret;
    }
    enable_irq = true;
    } else {
// soc alerts negate the need for polling
    INIT_DEFERRABLE_WORK(&chip.work, max17040_work);
    ret = devm_add_action(&client.dev, max17040_stop_work, chip);
    if (ret)
    return ret;
    max17040_queue_work(chip);
    }
    if (enable_irq) {
    ret = max17040_enable_alert_irq(chip);
    if (ret) {
    client.irq = 0;
    dev_warn(&client.dev,
    "Failed to get IRQ err %d\n", ret);
    }
    }
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn max17040_suspend(dev: *mut device) -> c_int {
    static int max17040_suspend(struct device *dev)
    {
    struct i2c_client *client = to_i2c_client(dev);
    struct max17040_chip *chip = i2c_get_clientdata(client);
    if (client.irq && chip.data.has_soc_alert)
// disable soc alert to prevent wakeup
    max17040_set_soc_alert(chip, 0);
    else
    cancel_delayed_work_sync(&chip.work);
    if (client.irq && device_may_wakeup(dev))
    enable_irq_wake(client.irq);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn max17040_resume(dev: *mut device) -> c_int {
    static int max17040_resume(struct device *dev)
    {
    struct i2c_client *client = to_i2c_client(dev);
    struct max17040_chip *chip = i2c_get_clientdata(client);
    if (client.irq && device_may_wakeup(dev))
    disable_irq_wake(client.irq);
    if (client.irq && chip.data.has_soc_alert)
    max17040_set_soc_alert(chip, 1);
    else
    max17040_queue_work(chip);
    return 0;
    }
    static SIMPLE_DEV_PM_OPS(max17040_pm_ops, max17040_suspend, max17040_resume);

    static const struct i2c_device_id max17040_id[] = {
    { .name = "max17040", .driver_data = ID_MAX17040 },
    { .name = "max17041", .driver_data = ID_MAX17041 },
    { .name = "max17043", .driver_data = ID_MAX17043 },
    { .name = "max77836-battery", .driver_data = ID_MAX17043 },
    { .name = "max17044", .driver_data = ID_MAX17044 },
    { .name = "max17048", .driver_data = ID_MAX17048 },
    { .name = "max17049", .driver_data = ID_MAX17049 },
    { .name = "max17058", .driver_data = ID_MAX17058 },
    { .name = "max17059", .driver_data = ID_MAX17059 },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(i2c, max17040_id);
    static const struct of_device_id max17040_of_match[] = {
    { .compatible = "maxim,max17040", .data = (void *) ID_MAX17040 },
    { .compatible = "maxim,max17041", .data = (void *) ID_MAX17041 },
    { .compatible = "maxim,max17043", .data = (void *) ID_MAX17043 },
    { .compatible = "maxim,max77836-battery", .data = (void *) ID_MAX17043 },
    { .compatible = "maxim,max17044", .data = (void *) ID_MAX17044 },
    { .compatible = "maxim,max17048", .data = (void *) ID_MAX17048 },
    { .compatible = "maxim,max17049", .data = (void *) ID_MAX17049 },
    { .compatible = "maxim,max17058", .data = (void *) ID_MAX17058 },
    { .compatible = "maxim,max17059", .data = (void *) ID_MAX17059 },
    { /* sentinel */ },
    };
    MODULE_DEVICE_TABLE(of, max17040_of_match);
    static struct i2c_driver max17040_i2c_driver = {
    .driver	= {
    .name	= "max17040",
    .of_match_table = max17040_of_match,
    .pm	= MAX17040_PM_OPS,
    },
    .probe		= max17040_probe,
    .id_table	= max17040_id,
    };
    module_i2c_driver(max17040_i2c_driver);
    MODULE_AUTHOR("Minkyu Kang <mk7.kang@samsung.com>");
    MODULE_DESCRIPTION("MAX17040 Fuel Gauge");
    MODULE_LICENSE("GPL");
    MODULE_IMPORT_NS("IIO_CONSUMER");
