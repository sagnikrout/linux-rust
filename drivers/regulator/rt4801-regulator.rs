//! Automatically rewritten from C to Rust
//! Source: drivers/regulator/rt4801-regulator.c
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


// SPDX-License-Identifier: GPL-2.0+

pub const RT4801_REG_VOP: c_uint = 0x00;
pub const RT4801_REG_VON: c_uint = 0x01;
pub const RT4801_REG_APPS: c_uint = 0x03;
pub const VOUT_MASK: c_uint = 0x1F;
pub const MIN_UV: c_int = 4000000;
pub const STEP_UV: c_int = 100000;
pub const MAX_UV: c_int = 6000000;

pub const DSV_OUT_POS: c_int = 0;
pub const DSV_OUT_NEG: c_int = 1;
pub const DSV_OUT_MAX: c_int = 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt4801_priv {
    pub dev: *mut device,
    pub enable_gpios: [*mut gpio_desc; DSV_OUT_MAX],
    pub enable_flag: c_uint,
    pub volt_sel: [c_uint; DSV_OUT_MAX],
}

    static int rt4801_of_parse_cb(struct device_node *np,
    const struct regulator_desc *desc,
    struct regulator_config *config)
    {
    struct rt4801_priv *priv = config.driver_data;
    let mut id: c_int = desc.id;
    if (priv.enable_gpios[id]) {
    dev_warn(priv.dev, "duplicated enable-gpios property\n");
    return 0;
    }
    priv.enable_gpios[id] = devm_fwnode_gpiod_get_index(priv.dev,
    of_fwnode_handle(np),
    "enable", 0,
    GPIOD_OUT_HIGH,
    "rt4801");
    if (IS_ERR(priv.enable_gpios[id]))
    priv.enable_gpios[id] = core::ptr::null_mut();
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rt4801_set_voltage_sel(rdev: *mut regulator_dev, selector: c_uint) -> c_int {
    static int rt4801_set_voltage_sel(struct regulator_dev *rdev, unsigned int selector)
    {
    struct rt4801_priv *priv = rdev_get_drvdata(rdev);
    let mut id: c_int = rdev_get_id(rdev), ret;
    if (priv.enable_flag & BIT(id)) {
    ret = regulator_set_voltage_sel_regmap(rdev, selector);
    if (ret)
    return ret;
    }
    priv.volt_sel[id] = selector;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rt4801_get_voltage_sel(rdev: *mut regulator_dev) -> c_int {
    static int rt4801_get_voltage_sel(struct regulator_dev *rdev)
    {
    struct rt4801_priv *priv = rdev_get_drvdata(rdev);
    let mut id: c_int = rdev_get_id(rdev);
    if (priv.enable_flag & BIT(id))
    return regulator_get_voltage_sel_regmap(rdev);
    return priv.volt_sel[id];
    }
#[no_mangle]
unsafe extern "C" fn rt4801_enable(rdev: *mut regulator_dev) -> c_int {
    static int rt4801_enable(struct regulator_dev *rdev)
    {
    struct rt4801_priv *priv = rdev_get_drvdata(rdev);
    let mut id: c_int = rdev_get_id(rdev), ret;
    if (!priv.enable_gpios[id]) {
    dev_warn(&rdev.dev, "no dedicated gpio can control\n");
    goto bypass_gpio;
    }
    gpiod_set_value(priv.enable_gpios[id], 1);
    bypass_gpio:
    ret = regmap_write(rdev.regmap, rdev.desc.vsel_reg, priv.volt_sel[id]);
    if (ret)
    return ret;
    priv.enable_flag |= BIT(id);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rt4801_disable(rdev: *mut regulator_dev) -> c_int {
    static int rt4801_disable(struct regulator_dev *rdev)
    {
    struct rt4801_priv *priv = rdev_get_drvdata(rdev);
    let mut id: c_int = rdev_get_id(rdev);
    if (!priv.enable_gpios[id]) {
    dev_warn(&rdev.dev, "no dedicated gpio can control\n");
    goto bypass_gpio;
    }
    gpiod_set_value(priv.enable_gpios[id], 0);
    bypass_gpio:
    priv.enable_flag &= ~BIT(id);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rt4801_is_enabled(rdev: *mut regulator_dev) -> c_int {
    static int rt4801_is_enabled(struct regulator_dev *rdev)
    {
    struct rt4801_priv *priv = rdev_get_drvdata(rdev);
    let mut id: c_int = rdev_get_id(rdev);
    return !!(priv.enable_flag & BIT(id));
    }
    static const struct regulator_ops rt4801_regulator_ops = {
    .list_voltage = regulator_list_voltage_linear,
    .set_voltage_sel = rt4801_set_voltage_sel,
    .get_voltage_sel = rt4801_get_voltage_sel,
    .enable = rt4801_enable,
    .disable = rt4801_disable,
    .is_enabled = rt4801_is_enabled,
    };
    static const struct regulator_desc rt4801_regulator_descs[] = {
    {
    .name = "DSVP",
    .ops = &rt4801_regulator_ops,
    .of_match = of_match_ptr("DSVP"),
    .of_parse_cb = rt4801_of_parse_cb,
    .type = REGULATOR_VOLTAGE,
    .id = DSV_OUT_POS,
    .min_uV = MIN_UV,
    .uV_step = STEP_UV,
    .n_voltages = N_VOLTAGES,
    .owner = THIS_MODULE,
    .vsel_reg = RT4801_REG_VOP,
    .vsel_mask = VOUT_MASK,
    },
    {
    .name = "DSVN",
    .ops = &rt4801_regulator_ops,
    .of_match = of_match_ptr("DSVN"),
    .of_parse_cb = rt4801_of_parse_cb,
    .type = REGULATOR_VOLTAGE,
    .id = DSV_OUT_NEG,
    .min_uV = MIN_UV,
    .uV_step = STEP_UV,
    .n_voltages = N_VOLTAGES,
    .owner = THIS_MODULE,
    .vsel_reg = RT4801_REG_VON,
    .vsel_mask = VOUT_MASK,
    },
    };
    static const struct regmap_config rt4801_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = RT4801_REG_APPS,
    };
#[no_mangle]
unsafe extern "C" fn rt4801_probe(i2c: *mut i2c_client) -> c_int {
    static int rt4801_probe(struct i2c_client *i2c)
    {
    struct rt4801_priv *priv;
    struct regmap *regmap;
    int i;
    priv = devm_kzalloc(&i2c.dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.dev = &i2c.dev;
// bootloader will on, driver only reconfigure enable to all output high
    priv.enable_flag = DSVALL_ENABLE;
    regmap = devm_regmap_init_i2c(i2c, &rt4801_regmap_config);
    if (IS_ERR(regmap)) {
    dev_err(&i2c.dev, "Failed to init regmap\n");
    return PTR_ERR(regmap);
    }
    for (i = 0; i < DSV_OUT_MAX; i++) {
    priv.enable_gpios[i] = devm_gpiod_get_index_optional(&i2c.dev,
    "enable",
    i,
    GPIOD_OUT_HIGH);
    if (IS_ERR(priv.enable_gpios[i])) {
    dev_err(&i2c.dev, "Failed to get gpios\n");
    return PTR_ERR(priv.enable_gpios[i]);
    }
    }
    for (i = 0; i < DSV_OUT_MAX; i++) {
    const struct regulator_desc *desc = rt4801_regulator_descs + i;
    struct regulator_config config = { .dev = &i2c.dev, .driver_data = priv,
    .regmap = regmap, };
    struct regulator_dev *rdev;
    unsigned int val;
    int ret;
// initialize volt_sel variable
    ret = regmap_read(regmap, desc.vsel_reg, &val);
    if (ret)
    return ret;
    priv.volt_sel[i] = val & desc.vsel_mask;
    rdev = devm_regulator_register(&i2c.dev, desc, &config);
    if (IS_ERR(rdev)) {
    dev_err(&i2c.dev, "Failed to register [%d] regulator\n", i);
    return PTR_ERR(rdev);
    }
    }
    return 0;
    }
    static const struct of_device_id __maybe_unused rt4801_of_id[] = {
    { .compatible = "richtek,rt4801", },
    { },
    };
    MODULE_DEVICE_TABLE(of, rt4801_of_id);
    static struct i2c_driver rt4801_driver = {
    .driver = {
    .name = "rt4801",
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    .of_match_table = of_match_ptr(rt4801_of_id),
    },
    .probe = rt4801_probe,
    };
    module_i2c_driver(rt4801_driver);
    MODULE_AUTHOR("ChiYuan Hwang <cy_huang@richtek.com>");
    MODULE_DESCRIPTION("Richtek RT4801 Display Bias Driver");
    MODULE_LICENSE("GPL v2");
