//! Automatically rewritten from C to Rust
//! Source: drivers/power/supply/da9150-charger.c
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
// DA9150 Charger Driver
//
// Copyright (c) 2014 Dialog Semiconductor
//
// Author: Adam Thomson <Adam.Thomson.Opensource@diasemi.com>
//

// Private data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct da9150_charger {
    pub da9150: *mut da9150,
    pub dev: *mut device,
    pub usb: *mut power_supply,
    pub battery: *mut power_supply,
    pub supply_online: *mut power_supply,
    pub usb_phy: *mut usb_phy,
    pub otg_nb: notifier_block,
    pub otg_work: work_struct,
    pub usb_event: c_ulong,
    pub ibus_chan: *mut iio_channel,
    pub vbus_chan: *mut iio_channel,
    pub tjunc_chan: *mut iio_channel,
    pub vbat_chan: *mut iio_channel,
}

    static inline int da9150_charger_supply_online(struct da9150_charger *charger,
    struct power_supply *psy,
    union power_supply_propval *val)
    {
    val.intval = (psy == charger.supply_online) ? 1 : 0;
    return 0;
    }
// Charger Properties
    static int da9150_charger_vbus_voltage_now(struct da9150_charger *charger,
    union power_supply_propval *val)
    {
    int v_val, ret;
// Read processed value - mV units
    ret = iio_read_channel_processed(charger.vbus_chan, &v_val);
    if (ret < 0)
    return ret;
// Convert voltage to expected uV units
    val.intval = v_val * 1000;
    return 0;
    }
    static int da9150_charger_ibus_current_avg(struct da9150_charger *charger,
    union power_supply_propval *val)
    {
    int i_val, ret;
// Read processed value - mA units
    ret = iio_read_channel_processed(charger.ibus_chan, &i_val);
    if (ret < 0)
    return ret;
// Convert current to expected uA units
    val.intval = i_val * 1000;
    return 0;
    }
    static int da9150_charger_tjunc_temp(struct da9150_charger *charger,
    union power_supply_propval *val)
    {
    int t_val, ret;
// Read processed value - 0.001 degrees C units
    ret = iio_read_channel_processed(charger.tjunc_chan, &t_val);
    if (ret < 0)
    return ret;
// Convert temp to expect 0.1 degrees C units
    val.intval = t_val / 100;
    return 0;
    }
    static enum power_supply_property da9150_charger_props[] = {
    POWER_SUPPLY_PROP_ONLINE,
    POWER_SUPPLY_PROP_VOLTAGE_NOW,
    POWER_SUPPLY_PROP_CURRENT_AVG,
    POWER_SUPPLY_PROP_TEMP,
    };
    static int da9150_charger_get_prop(struct power_supply *psy,
    enum power_supply_property psp,
    union power_supply_propval *val)
    {
    struct da9150_charger *charger = dev_get_drvdata(psy.dev.parent);
    int ret;
    switch (psp) {
    case POWER_SUPPLY_PROP_ONLINE:
    ret = da9150_charger_supply_online(charger, psy, val);
    break;
    case POWER_SUPPLY_PROP_VOLTAGE_NOW:
    ret = da9150_charger_vbus_voltage_now(charger, val);
    break;
    case POWER_SUPPLY_PROP_CURRENT_AVG:
    ret = da9150_charger_ibus_current_avg(charger, val);
    break;
    case POWER_SUPPLY_PROP_TEMP:
    ret = da9150_charger_tjunc_temp(charger, val);
    break;
    default:
    ret = -EINVAL;
    break;
    }
    return ret;
    }
// Battery Properties
    static int da9150_charger_battery_status(struct da9150_charger *charger,
    union power_supply_propval *val)
    {
    u8 reg;
// Check to see if battery is discharging
    reg = da9150_reg_read(charger.da9150, DA9150_STATUS_H);
    if (((reg & DA9150_VBUS_STAT_MASK) == DA9150_VBUS_STAT_OFF) ||
    ((reg & DA9150_VBUS_STAT_MASK) == DA9150_VBUS_STAT_WAIT)) {
    val.intval = POWER_SUPPLY_STATUS_DISCHARGING;
    return 0;
    }
    reg = da9150_reg_read(charger.da9150, DA9150_STATUS_J);
// Now check for other states
    switch (reg & DA9150_CHG_STAT_MASK) {
    case DA9150_CHG_STAT_ACT:
    case DA9150_CHG_STAT_PRE:
    case DA9150_CHG_STAT_CC:
    case DA9150_CHG_STAT_CV:
    val.intval = POWER_SUPPLY_STATUS_CHARGING;
    break;
    case DA9150_CHG_STAT_OFF:
    case DA9150_CHG_STAT_SUSP:
    case DA9150_CHG_STAT_TEMP:
    case DA9150_CHG_STAT_TIME:
    case DA9150_CHG_STAT_BAT:
    val.intval = POWER_SUPPLY_STATUS_NOT_CHARGING;
    break;
    case DA9150_CHG_STAT_FULL:
    val.intval = POWER_SUPPLY_STATUS_FULL;
    break;
    default:
    val.intval = POWER_SUPPLY_STATUS_UNKNOWN;
    break;
    }
    return 0;
    }
    static int da9150_charger_battery_health(struct da9150_charger *charger,
    union power_supply_propval *val)
    {
    u8 reg;
    reg = da9150_reg_read(charger.da9150, DA9150_STATUS_J);
// Check if temperature limit reached
    switch (reg & DA9150_CHG_TEMP_MASK) {
    case DA9150_CHG_TEMP_UNDER:
    val.intval = POWER_SUPPLY_HEALTH_COLD;
    return 0;
    case DA9150_CHG_TEMP_OVER:
    val.intval = POWER_SUPPLY_HEALTH_OVERHEAT;
    return 0;
    default:
    break;
    }
// Check for other health states
    switch (reg & DA9150_CHG_STAT_MASK) {
    case DA9150_CHG_STAT_ACT:
    case DA9150_CHG_STAT_PRE:
    val.intval = POWER_SUPPLY_HEALTH_DEAD;
    break;
    case DA9150_CHG_STAT_TIME:
    val.intval = POWER_SUPPLY_HEALTH_UNSPEC_FAILURE;
    break;
    default:
    val.intval = POWER_SUPPLY_HEALTH_GOOD;
    break;
    }
    return 0;
    }
    static int da9150_charger_battery_present(struct da9150_charger *charger,
    union power_supply_propval *val)
    {
    u8 reg;
// Check if battery present or removed
    reg = da9150_reg_read(charger.da9150, DA9150_STATUS_J);
    if ((reg & DA9150_CHG_STAT_MASK) == DA9150_CHG_STAT_BAT)
    val.intval = 0;
    else
    val.intval = 1;
    return 0;
    }
    static int da9150_charger_battery_charge_type(struct da9150_charger *charger,
    union power_supply_propval *val)
    {
    u8 reg;
    reg = da9150_reg_read(charger.da9150, DA9150_STATUS_J);
    switch (reg & DA9150_CHG_STAT_MASK) {
    case DA9150_CHG_STAT_CC:
    val.intval = POWER_SUPPLY_CHARGE_TYPE_FAST;
    break;
    case DA9150_CHG_STAT_ACT:
    case DA9150_CHG_STAT_PRE:
    case DA9150_CHG_STAT_CV:
    val.intval = POWER_SUPPLY_CHARGE_TYPE_TRICKLE;
    break;
    default:
    val.intval = POWER_SUPPLY_CHARGE_TYPE_NONE;
    break;
    }
    return 0;
    }
    static int da9150_charger_battery_voltage_min(struct da9150_charger *charger,
    union power_supply_propval *val)
    {
    u8 reg;
    reg = da9150_reg_read(charger.da9150, DA9150_PPR_CHGCTRL_C);
// Value starts at 2500 mV, 50 mV increments, presented in uV
    val.intval = ((reg & DA9150_CHG_VFAULT_MASK) * 50000) + 2500000;
    return 0;
    }
    static int da9150_charger_battery_voltage_now(struct da9150_charger *charger,
    union power_supply_propval *val)
    {
    int v_val, ret;
// Read processed value - mV units
    ret = iio_read_channel_processed(charger.vbat_chan, &v_val);
    if (ret < 0)
    return ret;
    val.intval = v_val * 1000;
    return 0;
    }
    static int da9150_charger_battery_current_max(struct da9150_charger *charger,
    union power_supply_propval *val)
    {
    int reg;
    reg = da9150_reg_read(charger.da9150, DA9150_PPR_CHGCTRL_D);
// 25mA increments
    val.intval = reg * 25000;
    return 0;
    }
    static int da9150_charger_battery_voltage_max(struct da9150_charger *charger,
    union power_supply_propval *val)
    {
    u8 reg;
    reg = da9150_reg_read(charger.da9150, DA9150_PPR_CHGCTRL_B);
// Value starts at 3650 mV, 25 mV increments, presented in uV
    val.intval = ((reg & DA9150_CHG_VBAT_MASK) * 25000) + 3650000;
    return 0;
    }
    static enum power_supply_property da9150_charger_bat_props[] = {
    POWER_SUPPLY_PROP_STATUS,
    POWER_SUPPLY_PROP_ONLINE,
    POWER_SUPPLY_PROP_HEALTH,
    POWER_SUPPLY_PROP_PRESENT,
    POWER_SUPPLY_PROP_CHARGE_TYPE,
    POWER_SUPPLY_PROP_VOLTAGE_MIN_DESIGN,
    POWER_SUPPLY_PROP_VOLTAGE_NOW,
    POWER_SUPPLY_PROP_CONSTANT_CHARGE_CURRENT_MAX,
    POWER_SUPPLY_PROP_CONSTANT_CHARGE_VOLTAGE_MAX,
    };
    static int da9150_charger_battery_get_prop(struct power_supply *psy,
    enum power_supply_property psp,
    union power_supply_propval *val)
    {
    struct da9150_charger *charger = dev_get_drvdata(psy.dev.parent);
    int ret;
    switch (psp) {
    case POWER_SUPPLY_PROP_STATUS:
    ret = da9150_charger_battery_status(charger, val);
    break;
    case POWER_SUPPLY_PROP_ONLINE:
    ret = da9150_charger_supply_online(charger, psy, val);
    break;
    case POWER_SUPPLY_PROP_HEALTH:
    ret = da9150_charger_battery_health(charger, val);
    break;
    case POWER_SUPPLY_PROP_PRESENT:
    ret = da9150_charger_battery_present(charger, val);
    break;
    case POWER_SUPPLY_PROP_CHARGE_TYPE:
    ret = da9150_charger_battery_charge_type(charger, val);
    break;
    case POWER_SUPPLY_PROP_VOLTAGE_MIN_DESIGN:
    ret = da9150_charger_battery_voltage_min(charger, val);
    break;
    case POWER_SUPPLY_PROP_VOLTAGE_NOW:
    ret = da9150_charger_battery_voltage_now(charger, val);
    break;
    case POWER_SUPPLY_PROP_CONSTANT_CHARGE_CURRENT_MAX:
    ret = da9150_charger_battery_current_max(charger, val);
    break;
    case POWER_SUPPLY_PROP_CONSTANT_CHARGE_VOLTAGE_MAX:
    ret = da9150_charger_battery_voltage_max(charger, val);
    break;
    default:
    ret = -EINVAL;
    break;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn da9150_charger_chg_irq(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t da9150_charger_chg_irq(int irq, void *data)
    {
    struct da9150_charger *charger = data;
    power_supply_changed(charger.battery);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn da9150_charger_tjunc_irq(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t da9150_charger_tjunc_irq(int irq, void *data)
    {
    struct da9150_charger *charger = data;
// Nothing we can really do except report this.
    dev_crit(charger.dev, "TJunc over temperature!!!\n");
    power_supply_changed(charger.usb);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn da9150_charger_vfault_irq(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t da9150_charger_vfault_irq(int irq, void *data)
    {
    struct da9150_charger *charger = data;
// Nothing we can really do except report this.
    dev_crit(charger.dev, "VSYS under voltage!!!\n");
    power_supply_changed(charger.usb);
    power_supply_changed(charger.battery);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn da9150_charger_vbus_irq(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t da9150_charger_vbus_irq(int irq, void *data)
    {
    struct da9150_charger *charger = data;
    u8 reg;
    reg = da9150_reg_read(charger.da9150, DA9150_STATUS_H);
// Charger plugged in or battery only
    switch (reg & DA9150_VBUS_STAT_MASK) {
    case DA9150_VBUS_STAT_OFF:
    case DA9150_VBUS_STAT_WAIT:
    charger.supply_online = charger.battery;
    break;
    case DA9150_VBUS_STAT_CHG:
    charger.supply_online = charger.usb;
    break;
    default:
    dev_warn(charger.dev, "Unknown VBUS state - reg = 0x%x\n",
    reg);
    charger.supply_online = core::ptr::null_mut();
    break;
    }
    power_supply_changed(charger.usb);
    power_supply_changed(charger.battery);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn da9150_charger_otg_work(data: *mut work_struct) {
    static void da9150_charger_otg_work(struct work_struct *data)
    {
    struct da9150_charger *charger =
    container_of(data, struct da9150_charger, otg_work);
    switch (charger.usb_event) {
    case USB_EVENT_ID:
// Enable OTG Boost
    da9150_set_bits(charger.da9150, DA9150_PPR_BKCTRL_A,
    DA9150_VBUS_MODE_MASK, DA9150_VBUS_MODE_OTG);
    break;
    case USB_EVENT_NONE:
// Revert to charge mode
    power_supply_changed(charger.usb);
    power_supply_changed(charger.battery);
    da9150_set_bits(charger.da9150, DA9150_PPR_BKCTRL_A,
    DA9150_VBUS_MODE_MASK, DA9150_VBUS_MODE_CHG);
    break;
    }
    }
    static int da9150_charger_otg_ncb(struct notifier_block *nb, unsigned long val,
    void *priv)
    {
    struct da9150_charger *charger =
    container_of(nb, struct da9150_charger, otg_nb);
    dev_dbg(charger.dev, "DA9150 OTG notify %lu\n", val);
    charger.usb_event = val;
    schedule_work(&charger.otg_work);
    return NOTIFY_OK;
    }
    static int da9150_charger_register_irq(struct platform_device *pdev,
    irq_handler_t handler,
    const char *irq_name)
    {
    struct device *dev = &pdev.dev;
    struct da9150_charger *charger = platform_get_drvdata(pdev);
    int irq, ret;
    irq = platform_get_irq_byname(pdev, irq_name);
    if (irq < 0)
    return irq;
    ret = request_threaded_irq(irq, core::ptr::null_mut(), handler, IRQF_ONESHOT, irq_name,
    charger);
    if (ret)
    dev_err(dev, "Failed to request IRQ %d: %d\n", irq, ret);
    return ret;
    }
    static void da9150_charger_unregister_irq(struct platform_device *pdev,
    const char *irq_name)
    {
    struct da9150_charger *charger = platform_get_drvdata(pdev);
    int irq;
    irq = platform_get_irq_byname(pdev, irq_name);
    if (irq < 0)
    return;
    free_irq(irq, charger);
    }
    static const struct power_supply_desc usb_desc = {
    .name		= "da9150-usb",
    .type		= POWER_SUPPLY_TYPE_USB,
    .properties	= da9150_charger_props,
    .num_properties	= ARRAY_SIZE(da9150_charger_props),
    .get_property	= da9150_charger_get_prop,
    };
    static const struct power_supply_desc battery_desc = {
    .name		= "da9150-battery",
    .type		= POWER_SUPPLY_TYPE_BATTERY,
    .properties	= da9150_charger_bat_props,
    .num_properties	= ARRAY_SIZE(da9150_charger_bat_props),
    .get_property	= da9150_charger_battery_get_prop,
    };
#[no_mangle]
unsafe extern "C" fn da9150_charger_probe(pdev: *mut platform_device) -> c_int {
    static int da9150_charger_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct da9150 *da9150 = dev_get_drvdata(dev.parent);
    struct da9150_charger *charger;
    u8 reg;
    int ret;
    charger = devm_kzalloc(dev, sizeof(struct da9150_charger), GFP_KERNEL);
    if (!charger)
    return -ENOMEM;
    platform_set_drvdata(pdev, charger);
    charger.da9150 = da9150;
    charger.dev = dev;
// Acquire ADC channels
    charger.ibus_chan = devm_iio_channel_get(dev, "CHAN_IBUS");
    if (IS_ERR(charger.ibus_chan))
    return PTR_ERR(charger.ibus_chan);
    charger.vbus_chan = devm_iio_channel_get(dev, "CHAN_VBUS");
    if (IS_ERR(charger.vbus_chan))
    return PTR_ERR(charger.vbus_chan);
    charger.tjunc_chan = devm_iio_channel_get(dev, "CHAN_TJUNC");
    if (IS_ERR(charger.tjunc_chan))
    return PTR_ERR(charger.tjunc_chan);
    charger.vbat_chan = devm_iio_channel_get(dev, "CHAN_VBAT");
    if (IS_ERR(charger.vbat_chan))
    return PTR_ERR(charger.vbat_chan);
// Register power supplies
    charger.usb = devm_power_supply_register(dev, &usb_desc, core::ptr::null_mut());
    if (IS_ERR(charger.usb))
    return PTR_ERR(charger.usb);
    charger.battery = devm_power_supply_register(dev, &battery_desc, core::ptr::null_mut());
    if (IS_ERR(charger.battery))
    return PTR_ERR(charger.battery);
// Get initial online supply
    reg = da9150_reg_read(da9150, DA9150_STATUS_H);
    switch (reg & DA9150_VBUS_STAT_MASK) {
    case DA9150_VBUS_STAT_OFF:
    case DA9150_VBUS_STAT_WAIT:
    charger.supply_online = charger.battery;
    break;
    case DA9150_VBUS_STAT_CHG:
    charger.supply_online = charger.usb;
    break;
    default:
    dev_warn(dev, "Unknown VBUS state - reg = 0x%x\n", reg);
    charger.supply_online = core::ptr::null_mut();
    break;
    }
// Setup OTG reporting & configuration
    charger.usb_phy = devm_usb_get_phy(dev, USB_PHY_TYPE_USB2);
    if (!IS_ERR_OR_NULL(charger.usb_phy)) {
    INIT_WORK(&charger.otg_work, da9150_charger_otg_work);
    charger.otg_nb.notifier_call = da9150_charger_otg_ncb;
    usb_register_notifier(charger.usb_phy, &charger.otg_nb);
    }
// Register IRQs
    ret = da9150_charger_register_irq(pdev, da9150_charger_chg_irq,
    "CHG_STATUS");
    if (ret < 0)
    goto chg_irq_fail;
    ret = da9150_charger_register_irq(pdev, da9150_charger_tjunc_irq,
    "CHG_TJUNC");
    if (ret < 0)
    goto tjunc_irq_fail;
    ret = da9150_charger_register_irq(pdev, da9150_charger_vfault_irq,
    "CHG_VFAULT");
    if (ret < 0)
    goto vfault_irq_fail;
    ret = da9150_charger_register_irq(pdev, da9150_charger_vbus_irq,
    "CHG_VBUS");
    if (ret < 0)
    goto vbus_irq_fail;
    return 0;
    vbus_irq_fail:
    da9150_charger_unregister_irq(pdev, "CHG_VFAULT");
    vfault_irq_fail:
    da9150_charger_unregister_irq(pdev, "CHG_TJUNC");
    tjunc_irq_fail:
    da9150_charger_unregister_irq(pdev, "CHG_STATUS");
    chg_irq_fail:
    if (!IS_ERR_OR_NULL(charger.usb_phy))
    usb_unregister_notifier(charger.usb_phy, &charger.otg_nb);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn da9150_charger_remove(pdev: *mut platform_device) {
    static void da9150_charger_remove(struct platform_device *pdev)
    {
    struct da9150_charger *charger = platform_get_drvdata(pdev);
    int irq;
// Make sure IRQs are released before unregistering power supplies
    irq = platform_get_irq_byname(pdev, "CHG_VBUS");
    free_irq(irq, charger);
    irq = platform_get_irq_byname(pdev, "CHG_VFAULT");
    free_irq(irq, charger);
    irq = platform_get_irq_byname(pdev, "CHG_TJUNC");
    free_irq(irq, charger);
    irq = platform_get_irq_byname(pdev, "CHG_STATUS");
    free_irq(irq, charger);
    if (!IS_ERR_OR_NULL(charger.usb_phy))
    usb_unregister_notifier(charger.usb_phy, &charger.otg_nb);
    cancel_work_sync(&charger.otg_work);
    }
    static struct platform_driver da9150_charger_driver = {
    .driver = {
    .name = "da9150-charger",
    },
    .probe = da9150_charger_probe,
    .remove = da9150_charger_remove,
    };
    module_platform_driver(da9150_charger_driver);
    MODULE_DESCRIPTION("Charger Driver for DA9150");
    MODULE_AUTHOR("Adam Thomson <Adam.Thomson.Opensource@diasemi.com>");
    MODULE_LICENSE("GPL");
    MODULE_IMPORT_NS("IIO_CONSUMER");
