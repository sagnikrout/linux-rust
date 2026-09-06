//! Automatically rewritten from C to Rust
//! Source: drivers/power/supply/axp20x_ac_power.c
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
// AXP20X and AXP22X PMICs' ACIN power supply driver
//
// Copyright (C) 2016 Free Electrons
// Quentin Schulz <quentin.schulz@free-electrons.com>
//

    (((((x) & AXP813_VHOLD_MASK) >> 3) + 40) * 100000)

    ((((x) & AXP813_CURR_LIMIT_MASK) + 3) * 500000)

#[repr(C)]
#[derive(Copy, Clone)]
pub struct axp20x_ac_power {
    pub regmap: *mut regmap,
    pub supply: *mut power_supply,
    pub acin_v: *mut iio_channel,
    pub acin_i: *mut iio_channel,
    pub has_acin_path_sel: bool,
    pub num_irqs: c_uint,
    pub __counted_by(num_irqs): unsigned int irqs[],
}

#[no_mangle]
unsafe extern "C" fn axp20x_ac_power_irq(irq: c_int, devid: *mut c_void) -> irqreturn_t {
    static irqreturn_t axp20x_ac_power_irq(int irq, void *devid)
    {
    struct axp20x_ac_power *power = devid;
    power_supply_changed(power.supply);
    return IRQ_HANDLED;
    }
    static int axp20x_ac_power_get_property(struct power_supply *psy,
    enum power_supply_property psp,
    union power_supply_propval *val)
    {
    struct axp20x_ac_power *power = power_supply_get_drvdata(psy);
    int ret, reg;
    switch (psp) {
    case POWER_SUPPLY_PROP_HEALTH:
    ret = regmap_read(power.regmap, AXP20X_PWR_INPUT_STATUS, &reg);
    if (ret)
    return ret;
    if (reg & AXP20X_PWR_STATUS_ACIN_PRESENT) {
    val.intval = POWER_SUPPLY_HEALTH_GOOD;
    return 0;
    }
    val.intval = POWER_SUPPLY_HEALTH_UNKNOWN;
    return 0;
    case POWER_SUPPLY_PROP_PRESENT:
    ret = regmap_read(power.regmap, AXP20X_PWR_INPUT_STATUS, &reg);
    if (ret)
    return ret;
    val.intval = !!(reg & AXP20X_PWR_STATUS_ACIN_PRESENT);
    return 0;
    case POWER_SUPPLY_PROP_ONLINE:
    ret = regmap_read(power.regmap, AXP20X_PWR_INPUT_STATUS, &reg);
    if (ret)
    return ret;
    val.intval = !!(reg & AXP20X_PWR_STATUS_ACIN_AVAIL);
// ACIN_PATH_SEL disables ACIN even if ACIN_AVAIL is set.
    if (val.intval && power.has_acin_path_sel) {
    ret = regmap_read(power.regmap, AXP813_ACIN_PATH_CTRL,
    &reg);
    if (ret)
    return ret;
    val.intval = !!(reg & AXP813_ACIN_PATH_SEL);
    }
    return 0;
    case POWER_SUPPLY_PROP_VOLTAGE_NOW:
    ret = iio_read_channel_processed(power.acin_v, &val.intval);
    if (ret)
    return ret;
// IIO framework gives mV but Power Supply framework gives uV
    val.intval *= 1000;
    return 0;
    case POWER_SUPPLY_PROP_CURRENT_NOW:
    ret = iio_read_channel_processed(power.acin_i, &val.intval);
    if (ret)
    return ret;
// IIO framework gives mA but Power Supply framework gives uA
    val.intval *= 1000;
    return 0;
    case POWER_SUPPLY_PROP_VOLTAGE_MIN:
    ret = regmap_read(power.regmap, AXP813_ACIN_PATH_CTRL, &reg);
    if (ret)
    return ret;
    val.intval = AXP813_VHOLD_REG_TO_UV(reg);
    return 0;
    case POWER_SUPPLY_PROP_INPUT_CURRENT_LIMIT:
    ret = regmap_read(power.regmap, AXP813_ACIN_PATH_CTRL, &reg);
    if (ret)
    return ret;
    val.intval = AXP813_CURR_LIMIT_REG_TO_UA(reg);
// AXP813 datasheet defines values 11x as 4000mA
    if (val.intval > 4000000)
    val.intval = 4000000;
    return 0;
    default:
    return -EINVAL;
    }
    return -EINVAL;
    }
    static int axp813_ac_power_set_property(struct power_supply *psy,
    enum power_supply_property psp,
    const union power_supply_propval *val)
    {
    struct axp20x_ac_power *power = power_supply_get_drvdata(psy);
    switch (psp) {
    case POWER_SUPPLY_PROP_ONLINE:
    return regmap_update_bits(power.regmap, AXP813_ACIN_PATH_CTRL,
    AXP813_ACIN_PATH_SEL,
    AXP813_ACIN_PATH_SEL_TO_BIT(val.intval));
    case POWER_SUPPLY_PROP_VOLTAGE_MIN:
    if (val.intval < 4000000 || val.intval > 4700000)
    return -EINVAL;
    return regmap_update_bits(power.regmap, AXP813_ACIN_PATH_CTRL,
    AXP813_VHOLD_MASK,
    AXP813_VHOLD_UV_TO_BIT(val.intval));
    case POWER_SUPPLY_PROP_INPUT_CURRENT_LIMIT:
    if (val.intval < 1500000 || val.intval > 4000000)
    return -EINVAL;
    return regmap_update_bits(power.regmap, AXP813_ACIN_PATH_CTRL,
    AXP813_CURR_LIMIT_MASK,
    AXP813_CURR_LIMIT_UA_TO_BIT(val.intval));
    default:
    return -EINVAL;
    }
    return -EINVAL;
    }
    static int axp813_ac_power_prop_writeable(struct power_supply *psy,
    enum power_supply_property psp)
    {
    return psp == POWER_SUPPLY_PROP_ONLINE ||
    psp == POWER_SUPPLY_PROP_VOLTAGE_MIN ||
    psp == POWER_SUPPLY_PROP_INPUT_CURRENT_LIMIT;
    }
    static enum power_supply_property axp20x_ac_power_properties[] = {
    POWER_SUPPLY_PROP_HEALTH,
    POWER_SUPPLY_PROP_PRESENT,
    POWER_SUPPLY_PROP_ONLINE,
    POWER_SUPPLY_PROP_VOLTAGE_NOW,
    POWER_SUPPLY_PROP_CURRENT_NOW,
    };
    static enum power_supply_property axp22x_ac_power_properties[] = {
    POWER_SUPPLY_PROP_HEALTH,
    POWER_SUPPLY_PROP_PRESENT,
    POWER_SUPPLY_PROP_ONLINE,
    };
    static enum power_supply_property axp813_ac_power_properties[] = {
    POWER_SUPPLY_PROP_HEALTH,
    POWER_SUPPLY_PROP_PRESENT,
    POWER_SUPPLY_PROP_ONLINE,
    POWER_SUPPLY_PROP_VOLTAGE_MIN,
    POWER_SUPPLY_PROP_INPUT_CURRENT_LIMIT,
    };
    static const struct power_supply_desc axp20x_ac_power_desc = {
    .name = "axp20x-ac",
    .type = POWER_SUPPLY_TYPE_MAINS,
    .properties = axp20x_ac_power_properties,
    .num_properties = ARRAY_SIZE(axp20x_ac_power_properties),
    .get_property = axp20x_ac_power_get_property,
    };
    static const struct power_supply_desc axp22x_ac_power_desc = {
    .name = "axp22x-ac",
    .type = POWER_SUPPLY_TYPE_MAINS,
    .properties = axp22x_ac_power_properties,
    .num_properties = ARRAY_SIZE(axp22x_ac_power_properties),
    .get_property = axp20x_ac_power_get_property,
    };
    static const struct power_supply_desc axp813_ac_power_desc = {
    .name = "axp813-ac",
    .type = POWER_SUPPLY_TYPE_MAINS,
    .properties = axp813_ac_power_properties,
    .num_properties = ARRAY_SIZE(axp813_ac_power_properties),
    .property_is_writeable = axp813_ac_power_prop_writeable,
    .get_property = axp20x_ac_power_get_property,
    .set_property = axp813_ac_power_set_property,
    };
    static const char * const axp20x_irq_names[] = {
    "ACIN_PLUGIN",
    "ACIN_REMOVAL",
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct axp_data {
    pub power_desc: *const power_supply_desc,
    pub irq_names: *const *const c_char,
    pub num_irq_names: c_uint,
    pub acin_adc: bool,
    pub acin_path_sel: bool,
}

    static const struct axp_data axp20x_data = {
    .power_desc	= &axp20x_ac_power_desc,
    .irq_names	= axp20x_irq_names,
    .num_irq_names	= ARRAY_SIZE(axp20x_irq_names),
    .acin_adc	= true,
    .acin_path_sel	= false,
    };
    static const struct axp_data axp22x_data = {
    .power_desc	= &axp22x_ac_power_desc,
    .irq_names	= axp20x_irq_names,
    .num_irq_names	= ARRAY_SIZE(axp20x_irq_names),
    .acin_adc	= false,
    .acin_path_sel	= false,
    };
    static const struct axp_data axp813_data = {
    .power_desc	= &axp813_ac_power_desc,
    .irq_names	= axp20x_irq_names,
    .num_irq_names	= ARRAY_SIZE(axp20x_irq_names),
    .acin_adc	= false,
    .acin_path_sel	= true,
    };

#[no_mangle]
unsafe extern "C" fn axp20x_ac_power_suspend(dev: *mut device) -> c_int {
    static int axp20x_ac_power_suspend(struct device *dev)
    {
    struct axp20x_ac_power *power = dev_get_drvdata(dev);
    let mut i: c_int = 0;
//
// Allow wake via ACIN_PLUGIN only.
//
// As nested threaded IRQs are not automatically disabled during
// suspend, we must explicitly disable the remainder of the IRQs.
//
    if (device_may_wakeup(&power.supply.dev))
    enable_irq_wake(power.irqs[i++]);
    while (i < power.num_irqs)
    disable_irq(power.irqs[i++]);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn axp20x_ac_power_resume(dev: *mut device) -> c_int {
    static int axp20x_ac_power_resume(struct device *dev)
    {
    struct axp20x_ac_power *power = dev_get_drvdata(dev);
    let mut i: c_int = 0;
    if (device_may_wakeup(&power.supply.dev))
    disable_irq_wake(power.irqs[i++]);
    while (i < power.num_irqs)
    enable_irq(power.irqs[i++]);
    return 0;
    }

    static SIMPLE_DEV_PM_OPS(axp20x_ac_power_pm_ops, axp20x_ac_power_suspend,
    axp20x_ac_power_resume);
#[no_mangle]
unsafe extern "C" fn axp20x_ac_power_probe(pdev: *mut platform_device) -> c_int {
    static int axp20x_ac_power_probe(struct platform_device *pdev)
    {
    struct axp20x_dev *axp20x = dev_get_drvdata(pdev.dev.parent);
    let mut psy_cfg: power_supply_config = {};
    struct axp20x_ac_power *power;
    const struct axp_data *axp_data;
    int i, irq, ret;
    if (!of_device_is_available(pdev.dev.of_node))
    return -ENODEV;
    if (!axp20x) {
    dev_err(&pdev.dev, "Parent drvdata not set\n");
    return -EINVAL;
    }
    axp_data = of_device_get_match_data(&pdev.dev);
    power = devm_kzalloc(&pdev.dev,
    struct_size(power, irqs, axp_data.num_irq_names),
    GFP_KERNEL);
    if (!power)
    return -ENOMEM;
    if (axp_data.acin_adc) {
    power.acin_v = devm_iio_channel_get(&pdev.dev, "acin_v");
    if (IS_ERR(power.acin_v)) {
    if (PTR_ERR(power.acin_v) == -ENODEV)
    return -EPROBE_DEFER;
    return PTR_ERR(power.acin_v);
    }
    power.acin_i = devm_iio_channel_get(&pdev.dev, "acin_i");
    if (IS_ERR(power.acin_i)) {
    if (PTR_ERR(power.acin_i) == -ENODEV)
    return -EPROBE_DEFER;
    return PTR_ERR(power.acin_i);
    }
    }
    power.regmap = dev_get_regmap(pdev.dev.parent, core::ptr::null_mut());
    power.has_acin_path_sel = axp_data.acin_path_sel;
    power.num_irqs = axp_data.num_irq_names;
    platform_set_drvdata(pdev, power);
    psy_cfg.fwnode = dev_fwnode(&pdev.dev);
    psy_cfg.drv_data = power;
    power.supply = devm_power_supply_register(&pdev.dev,
    axp_data.power_desc,
    &psy_cfg);
    if (IS_ERR(power.supply))
    return PTR_ERR(power.supply);
// Request irqs after registering, as irqs may trigger immediately
    for (i = 0; i < axp_data.num_irq_names; i++) {
    irq = platform_get_irq_byname(pdev, axp_data.irq_names[i]);
    if (irq < 0)
    return irq;
    power.irqs[i] = regmap_irq_get_virq(axp20x.regmap_irqc, irq);
    ret = devm_request_any_context_irq(&pdev.dev, power.irqs[i],
    axp20x_ac_power_irq, 0,
    DRVNAME, power);
    if (ret < 0)
    return ret;
    }
    return 0;
    }
    static const struct of_device_id axp20x_ac_power_match[] = {
    {
    .compatible = "x-powers,axp202-ac-power-supply",
    .data = &axp20x_data,
    }, {
    .compatible = "x-powers,axp221-ac-power-supply",
    .data = &axp22x_data,
    }, {
    .compatible = "x-powers,axp813-ac-power-supply",
    .data = &axp813_data,
    }, { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, axp20x_ac_power_match);
    static struct platform_driver axp20x_ac_power_driver = {
    .probe = axp20x_ac_power_probe,
    .driver = {
    .name		= DRVNAME,
    .of_match_table	= axp20x_ac_power_match,
    .pm		= &axp20x_ac_power_pm_ops,
    },
    };
    module_platform_driver(axp20x_ac_power_driver);
    MODULE_AUTHOR("Quentin Schulz <quentin.schulz@free-electrons.com>");
    MODULE_DESCRIPTION("AXP20X and AXP22X PMICs' AC power supply driver");
    MODULE_LICENSE("GPL");
    MODULE_IMPORT_NS("IIO_CONSUMER");
