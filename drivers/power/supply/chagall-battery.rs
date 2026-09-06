//! Automatically rewritten from C to Rust
//! Source: drivers/power/supply/chagall-battery.c
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

pub const CHAGALL_REG_LED_AMBER: c_uint = 0x60;
pub const CHAGALL_REG_LED_WHITE: c_uint = 0x70;
pub const CHAGALL_REG_BATTERY_TEMPERATURE: c_uint = 0xa2;
pub const CHAGALL_REG_BATTERY_VOLTAGE: c_uint = 0xa4;
pub const CHAGALL_REG_BATTERY_CURRENT: c_uint = 0xa6;
pub const CHAGALL_REG_BATTERY_CAPACITY: c_uint = 0xa8;
pub const CHAGALL_REG_BATTERY_CHARGING_CURRENT: c_uint = 0xaa;
pub const CHAGALL_REG_BATTERY_CHARGING_VOLTAGE: c_uint = 0xac;
pub const CHAGALL_REG_BATTERY_STATUS: c_uint = 0xae;

pub const CHAGALL_REG_BATTERY_REMAIN_CAPACITY: c_uint = 0xb0;
pub const CHAGALL_REG_BATTERY_FULL_CAPACITY: c_uint = 0xb2;
pub const CHAGALL_REG_MAX_COUNT: c_uint = 0xb4;
pub const CHAGALL_BATTERY_DATA_REFRESH: c_int = 5000;
pub const TEMP_CELSIUS_OFFSET: c_int = 2731;
    static const struct regmap_config chagall_battery_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = CHAGALL_REG_MAX_COUNT,
    .reg_format_endian = REGMAP_ENDIAN_LITTLE,
    .val_format_endian = REGMAP_ENDIAN_LITTLE,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct chagall_battery_data {
    pub regmap: *mut regmap,
    pub amber_led: led_classdev,
    pub white_led: led_classdev,
    pub battery: *mut power_supply,
    pub poll_work: delayed_work,
    pub last_state: u16,
}

    static void chagall_led_set_brightness_amber(struct led_classdev *led,
    enum led_brightness brightness)
    {
    struct chagall_battery_data *cg =
    container_of(led, struct chagall_battery_data, amber_led);
    regmap_write(cg.regmap, CHAGALL_REG_LED_AMBER, brightness);
    }
    static void chagall_led_set_brightness_white(struct led_classdev *led,
    enum led_brightness brightness)
    {
    struct chagall_battery_data *cg =
    container_of(led, struct chagall_battery_data, white_led);
    regmap_write(cg.regmap, CHAGALL_REG_LED_WHITE, brightness);
    }
    static const enum power_supply_property chagall_battery_properties[] = {
    POWER_SUPPLY_PROP_STATUS,
    POWER_SUPPLY_PROP_PRESENT,
    POWER_SUPPLY_PROP_VOLTAGE_NOW,
    POWER_SUPPLY_PROP_VOLTAGE_MAX,
    POWER_SUPPLY_PROP_CURRENT_NOW,
    POWER_SUPPLY_PROP_CURRENT_MAX,
    POWER_SUPPLY_PROP_CAPACITY,
    POWER_SUPPLY_PROP_TEMP,
    POWER_SUPPLY_PROP_CHARGE_FULL,
    POWER_SUPPLY_PROP_CHARGE_NOW,
    };
    static const unsigned int chagall_battery_prop_offs[] = {
    [POWER_SUPPLY_PROP_STATUS] = CHAGALL_REG_BATTERY_STATUS,
    [POWER_SUPPLY_PROP_VOLTAGE_NOW] = CHAGALL_REG_BATTERY_VOLTAGE,
    [POWER_SUPPLY_PROP_VOLTAGE_MAX] = CHAGALL_REG_BATTERY_CHARGING_VOLTAGE,
    [POWER_SUPPLY_PROP_CURRENT_NOW] = CHAGALL_REG_BATTERY_CURRENT,
    [POWER_SUPPLY_PROP_CURRENT_MAX] = CHAGALL_REG_BATTERY_CHARGING_CURRENT,
    [POWER_SUPPLY_PROP_CAPACITY] = CHAGALL_REG_BATTERY_CAPACITY,
    [POWER_SUPPLY_PROP_TEMP] = CHAGALL_REG_BATTERY_TEMPERATURE,
    [POWER_SUPPLY_PROP_CHARGE_FULL] = CHAGALL_REG_BATTERY_FULL_CAPACITY,
    [POWER_SUPPLY_PROP_CHARGE_NOW] = CHAGALL_REG_BATTERY_REMAIN_CAPACITY,
    };
    static int chagall_battery_get_value(struct chagall_battery_data *cg,
    enum power_supply_property psp, u32 *val)
    {
    if (psp >= ARRAY_SIZE(chagall_battery_prop_offs))
    return -EINVAL;
    if (!chagall_battery_prop_offs[psp])
    return -EINVAL;
// Battery data is stored in 2 consecutive registers with little-endian
    return regmap_bulk_read(cg.regmap, chagall_battery_prop_offs[psp], val, 2);
    }
#[no_mangle]
unsafe extern "C" fn chagall_battery_get_status(status_reg: u32) -> c_int {
    static int chagall_battery_get_status(u32 status_reg)
    {
    if (status_reg & BATTERY_FULL_CHARGED)
    return POWER_SUPPLY_STATUS_FULL;
#[no_mangle]
pub unsafe extern "C" fn if(BATTERY_DISCHARGING: status_reg &) -> else {
    else if (status_reg & BATTERY_DISCHARGING)
    return POWER_SUPPLY_STATUS_DISCHARGING;
    else
    return POWER_SUPPLY_STATUS_CHARGING;
    }
    static int chagall_battery_get_property(struct power_supply *psy,
    enum power_supply_property psp,
    union power_supply_propval *val)
    {
    struct chagall_battery_data *cg = power_supply_get_drvdata(psy);
    int ret;
    switch (psp) {
    case POWER_SUPPLY_PROP_PRESENT:
    val.intval = 1;
    break;
    default:
    ret = chagall_battery_get_value(cg, psp, &val.intval);
    if (ret)
    return ret;
    switch (psp) {
    case POWER_SUPPLY_PROP_TEMP:
    val.intval -= TEMP_CELSIUS_OFFSET;
    break;
    case POWER_SUPPLY_PROP_VOLTAGE_MAX:
    case POWER_SUPPLY_PROP_VOLTAGE_NOW:
    case POWER_SUPPLY_PROP_CURRENT_MAX:
    case POWER_SUPPLY_PROP_CURRENT_NOW:
    case POWER_SUPPLY_PROP_CHARGE_FULL:
    case POWER_SUPPLY_PROP_CHARGE_NOW:
    val.intval *= 1000;
    break;
    case POWER_SUPPLY_PROP_STATUS:
    val.intval = chagall_battery_get_status(val.intval);
    break;
    default:
    break;
    }
    break;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn chagall_battery_poll_work(work: *mut work_struct) {
    static void chagall_battery_poll_work(struct work_struct *work)
    {
    struct chagall_battery_data *cg =
    container_of(work, struct chagall_battery_data, poll_work.work);
    u32 state;
    int ret;
    ret = chagall_battery_get_value(cg, POWER_SUPPLY_PROP_STATUS, &state);
    if (ret)
    return;
    state = chagall_battery_get_status(state);
    if (cg.last_state != state) {
    cg.last_state = state;
    power_supply_changed(cg.battery);
    }
// continuously send uevent notification
    schedule_delayed_work(&cg.poll_work,
    msecs_to_jiffies(CHAGALL_BATTERY_DATA_REFRESH));
    }
    static const struct power_supply_desc chagall_battery_desc = {
    .name = "chagall-battery",
    .type = POWER_SUPPLY_TYPE_BATTERY,
    .properties = chagall_battery_properties,
    .num_properties = ARRAY_SIZE(chagall_battery_properties),
    .get_property = chagall_battery_get_property,
    .external_power_changed = power_supply_changed,
    };
#[no_mangle]
unsafe extern "C" fn chagall_battery_probe(client: *mut i2c_client) -> c_int {
    static int chagall_battery_probe(struct i2c_client *client)
    {
    struct chagall_battery_data *cg;
    struct device *dev = &client.dev;
    let mut cfg: power_supply_config = { };
    int ret;
    cg = devm_kzalloc(dev, sizeof(*cg), GFP_KERNEL);
    if (!cg)
    return -ENOMEM;
    cfg.drv_data = cg;
    cfg.fwnode = dev_fwnode(dev);
    i2c_set_clientdata(client, cg);
    cg.regmap = devm_regmap_init_i2c(client, &chagall_battery_regmap_config);
    if (IS_ERR(cg.regmap))
    return dev_err_probe(dev, PTR_ERR(cg.regmap), "cannot allocate regmap\n");
    cg.last_state = POWER_SUPPLY_STATUS_UNKNOWN;
    cg.battery = devm_power_supply_register(dev, &chagall_battery_desc, &cfg);
    if (IS_ERR(cg.battery))
    return dev_err_probe(dev, PTR_ERR(cg.battery),
    "failed to register power supply\n");
    cg.amber_led.name = "power::amber";
    cg.amber_led.max_brightness = 1;
    cg.amber_led.flags = LED_CORE_SUSPENDRESUME;
    cg.amber_led.brightness_set = chagall_led_set_brightness_amber;
    cg.amber_led.default_trigger = "chagall-battery-charging";
    ret = devm_led_classdev_register(dev, &cg.amber_led);
    if (ret)
    return dev_err_probe(dev, ret, "failed to register amber LED\n");
    cg.white_led.name = "power::white";
    cg.white_led.max_brightness = 1;
    cg.white_led.flags = LED_CORE_SUSPENDRESUME;
    cg.white_led.brightness_set = chagall_led_set_brightness_white;
    cg.white_led.default_trigger = "chagall-battery-full";
    ret = devm_led_classdev_register(dev, &cg.white_led);
    if (ret)
    return dev_err_probe(dev, ret, "failed to register white LED\n");
    led_set_brightness(&cg.amber_led, LED_OFF);
    led_set_brightness(&cg.white_led, LED_OFF);
    ret = devm_delayed_work_autocancel(dev, &cg.poll_work, chagall_battery_poll_work);
    if (ret)
    return ret;
    schedule_delayed_work(&cg.poll_work, msecs_to_jiffies(CHAGALL_BATTERY_DATA_REFRESH));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn chagall_battery_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused chagall_battery_suspend(struct device *dev)
    {
    struct i2c_client *client = to_i2c_client(dev);
    struct chagall_battery_data *cg = i2c_get_clientdata(client);
    cancel_delayed_work_sync(&cg.poll_work);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn chagall_battery_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused chagall_battery_resume(struct device *dev)
    {
    struct i2c_client *client = to_i2c_client(dev);
    struct chagall_battery_data *cg = i2c_get_clientdata(client);
    schedule_delayed_work(&cg.poll_work, msecs_to_jiffies(CHAGALL_BATTERY_DATA_REFRESH));
    return 0;
    }
    static SIMPLE_DEV_PM_OPS(chagall_battery_pm_ops,
    chagall_battery_suspend, chagall_battery_resume);
    static const struct of_device_id chagall_of_match[] = {
    { .compatible = "pegatron,chagall-ec" },
    { }
    };
    MODULE_DEVICE_TABLE(of, chagall_of_match);
    static struct i2c_driver chagall_battery_driver = {
    .driver = {
    .name = "chagall-battery",
    .pm = &chagall_battery_pm_ops,
    .of_match_table = chagall_of_match,
    },
    .probe = chagall_battery_probe,
    };
    module_i2c_driver(chagall_battery_driver);
    MODULE_AUTHOR("Svyatoslav Ryhel <clamor95@gmail.com>");
    MODULE_DESCRIPTION("Pegatron Chagall fuel gauge driver");
    MODULE_LICENSE("GPL");
