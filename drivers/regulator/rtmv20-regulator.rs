//! Automatically rewritten from C to Rust
//! Source: drivers/regulator/rtmv20-regulator.c
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

pub const RTMV20_REG_DEVINFO: c_uint = 0x00;
pub const RTMV20_REG_PULSEDELAY: c_uint = 0x01;
pub const RTMV20_REG_PULSEWIDTH: c_uint = 0x03;
pub const RTMV20_REG_LDCTRL1: c_uint = 0x05;
pub const RTMV20_REG_ESPULSEWIDTH: c_uint = 0x06;
pub const RTMV20_REG_ESLDCTRL1: c_uint = 0x08;
pub const RTMV20_REG_LBP: c_uint = 0x0A;
pub const RTMV20_REG_LDCTRL2: c_uint = 0x0B;
pub const RTMV20_REG_FSIN1CTRL1: c_uint = 0x0D;
pub const RTMV20_REG_FSIN1CTRL3: c_uint = 0x0F;
pub const RTMV20_REG_FSIN2CTRL1: c_uint = 0x10;
pub const RTMV20_REG_FSIN2CTRL3: c_uint = 0x12;
pub const RTMV20_REG_ENCTRL: c_uint = 0x13;
pub const RTMV20_REG_STRBVSYNDLYL: c_uint = 0x29;
pub const RTMV20_REG_LDIRQ: c_uint = 0x30;
pub const RTMV20_REG_LDSTAT: c_uint = 0x40;
pub const RTMV20_REG_LDMASK: c_uint = 0x50;

pub const RICHTEK_VID: c_uint = 0x80;

pub const RTMV20_LSW_MINUA: c_int = 0;
pub const RTMV20_LSW_MAXUA: c_int = 6000000;
pub const RTMV20_LSW_STEPUA: c_int = 30000;
pub const RTMV20_LSW_DEFAULTUA: c_int = 3000000;
pub const RTMV20_I2CRDY_TIMEUS: c_int = 200;
pub const RTMV20_CSRDY_TIMEUS: c_int = 2000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtmv20_priv {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub enable_gpio: *mut gpio_desc,
    pub rdev: *mut regulator_dev,
}

#[no_mangle]
unsafe extern "C" fn rtmv20_lsw_enable(rdev: *mut regulator_dev) -> c_int {
    static int rtmv20_lsw_enable(struct regulator_dev *rdev)
    {
    struct rtmv20_priv *priv = rdev_get_drvdata(rdev);
    int ret;
    gpiod_set_value(priv.enable_gpio, 1);
// Wait for I2C can be accessed
    usleep_range(RTMV20_I2CRDY_TIMEUS, RTMV20_I2CRDY_TIMEUS + 100);
// HW re-enable, disable cache only and sync regcache here
    regcache_cache_only(priv.regmap, false);
    ret = regcache_sync(priv.regmap);
    if (ret)
    return ret;
    return regulator_enable_regmap(rdev);
    }
#[no_mangle]
unsafe extern "C" fn rtmv20_lsw_disable(rdev: *mut regulator_dev) -> c_int {
    static int rtmv20_lsw_disable(struct regulator_dev *rdev)
    {
    struct rtmv20_priv *priv = rdev_get_drvdata(rdev);
    int ret;
    ret = regulator_disable_regmap(rdev);
    if (ret)
    return ret;
// Mark the regcache as dirty and cache only before HW disabled
    regcache_cache_only(priv.regmap, true);
    regcache_mark_dirty(priv.regmap);
    gpiod_set_value(priv.enable_gpio, 0);
    return 0;
    }
    static int rtmv20_lsw_set_current_limit(struct regulator_dev *rdev, int min_uA,
    int max_uA)
    {
    int sel;
    if (min_uA > RTMV20_LSW_MAXUA || max_uA < RTMV20_LSW_MINUA)
    return -EINVAL;
    if (max_uA > RTMV20_LSW_MAXUA)
    max_uA = RTMV20_LSW_MAXUA;
    sel = (max_uA - RTMV20_LSW_MINUA) / RTMV20_LSW_STEPUA;
// Ensure the selected setting is still in range
    if ((sel * RTMV20_LSW_STEPUA + RTMV20_LSW_MINUA) < min_uA)
    return -EINVAL;
    sel <<= ffs(rdev.desc.csel_mask) - 1;
    return regmap_update_bits(rdev.regmap, rdev.desc.csel_reg,
    rdev.desc.csel_mask, sel);
    }
#[no_mangle]
unsafe extern "C" fn rtmv20_lsw_get_current_limit(rdev: *mut regulator_dev) -> c_int {
    static int rtmv20_lsw_get_current_limit(struct regulator_dev *rdev)
    {
    unsigned int val;
    int ret;
    ret = regmap_read(rdev.regmap, rdev.desc.csel_reg, &val);
    if (ret)
    return ret;
    val &= rdev.desc.csel_mask;
    val >>= ffs(rdev.desc.csel_mask) - 1;
    return val * RTMV20_LSW_STEPUA + RTMV20_LSW_MINUA;
    }
    static const struct regulator_ops rtmv20_regulator_ops = {
    .set_current_limit = rtmv20_lsw_set_current_limit,
    .get_current_limit = rtmv20_lsw_get_current_limit,
    .enable = rtmv20_lsw_enable,
    .disable = rtmv20_lsw_disable,
    .is_enabled = regulator_is_enabled_regmap,
    };
    static const struct regulator_desc rtmv20_lsw_desc = {
    .name = "rtmv20,lsw",
    .of_match = of_match_ptr("lsw"),
    .type = REGULATOR_CURRENT,
    .owner = THIS_MODULE,
    .ops = &rtmv20_regulator_ops,
    .csel_reg = RTMV20_REG_LDCTRL1,
    .csel_mask = RTMV20_LDCURR_MASK,
    .enable_reg = RTMV20_REG_ENCTRL,
    .enable_mask = LDENABLE_MASK,
    .enable_time = RTMV20_CSRDY_TIMEUS,
    };
#[no_mangle]
unsafe extern "C" fn rtmv20_irq_handler(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t rtmv20_irq_handler(int irq, void *data)
    {
    struct rtmv20_priv *priv = data;
    unsigned int val;
    int ret;
    ret = regmap_read(priv.regmap, RTMV20_REG_LDIRQ, &val);
    if (ret) {
    dev_err(priv.dev, "Failed to get irq flags\n");
    return IRQ_NONE;
    }
    if (val & OTPEVT_MASK)
    regulator_notifier_call_chain(priv.rdev, REGULATOR_EVENT_OVER_TEMP, core::ptr::null_mut());
    if (val & OCPEVT_MASK)
    regulator_notifier_call_chain(priv.rdev, REGULATOR_EVENT_OVER_CURRENT, core::ptr::null_mut());
    if (val & FAILEVT_MASK)
    regulator_notifier_call_chain(priv.rdev, REGULATOR_EVENT_FAIL, core::ptr::null_mut());
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn clamp_to_selector(val: u32, min: u32, max: u32, step: u32) -> u32 {
    static u32 clamp_to_selector(u32 val, u32 min, u32 max, u32 step)
    {
    let mut retval: u32 = clamp_val(val, min, max);
    return (retval - min) / step;
    }
#[no_mangle]
unsafe extern "C" fn rtmv20_properties_init(priv: *mut rtmv20_priv) -> c_int {
    static int rtmv20_properties_init(struct rtmv20_priv *priv)
    {
    const struct {
    const char *name;
    u32 def;
    u32 min;
    u32 max;
    u32 step;
    u32 addr;
    u32 mask;
    } props[] = {
    { "richtek,ld-pulse-delay-us", 0, 0, 100000, 100, RTMV20_REG_PULSEDELAY,
    RTMV20_DELAY_MASK },
    { "richtek,ld-pulse-width-us", 1200, 0, 10000, 1, RTMV20_REG_PULSEWIDTH,
    RTMV20_WIDTH_MASK },
    { "richtek,fsin1-delay-us", 23000, 0, 100000, 100, RTMV20_REG_FSIN1CTRL1,
    RTMV20_DELAY_MASK },
    { "richtek,fsin1-width-us", 160, 40, 10000, 40, RTMV20_REG_FSIN1CTRL3,
    RTMV20_WIDTH2_MASK },
    { "richtek,fsin2-delay-us", 23000, 0, 100000, 100, RTMV20_REG_FSIN2CTRL1,
    RTMV20_DELAY_MASK },
    { "richtek,fsin2-width-us", 160, 40, 10000, 40, RTMV20_REG_FSIN2CTRL3,
    RTMV20_WIDTH2_MASK },
    { "richtek,es-pulse-width-us", 1200, 0, 10000, 1, RTMV20_REG_ESPULSEWIDTH,
    RTMV20_WIDTH_MASK },
    { "richtek,es-ld-current-microamp", 3000000, 0, 6000000, 30000,
    RTMV20_REG_ESLDCTRL1, RTMV20_LDCURR_MASK },
    { "richtek,lbp-level-microvolt", 2700000, 2400000, 3700000, 100000, RTMV20_REG_LBP,
    RTMV20_LBPLVL_MASK },
    { "richtek,lbp-enable", 0, 0, 1, 1, RTMV20_REG_LBP, RTMV20_LBPEN_MASK },
    { "richtek,strobe-polarity-high", 1, 0, 1, 1, RTMV20_REG_LDCTRL2,
    RTMV20_STROBEPOL_MASK },
    { "richtek,vsync-polarity-high", 1, 0, 1, 1, RTMV20_REG_LDCTRL2,
    RTMV20_VSYNPOL_MASK },
    { "richtek,fsin-enable", 0, 0, 1, 1, RTMV20_REG_ENCTRL, RTMV20_FSINEN_MASK },
    { "richtek,fsin-output", 0, 0, 1, 1, RTMV20_REG_ENCTRL, RTMV20_FSINOUT_MASK },
    { "richtek,es-enable", 0, 0, 1, 1, RTMV20_REG_ENCTRL, RTMV20_ESEN_MASK },
    };
    int i, ret;
    for (i = 0; i < ARRAY_SIZE(props); i++) {
    __be16 bval16;
    u16 val16;
    u32 temp;
    let mut significant_bit: c_int = fls(props[i].mask);
    let mut shift: c_int = ffs(props[i].mask) - 1;
    if (props[i].max > 1) {
    ret = device_property_read_u32(priv.dev, props[i].name, &temp);
    if (ret)
    temp = props[i].def;
    } else
    temp = device_property_read_bool(priv.dev, props[i].name);
    temp = clamp_to_selector(temp, props[i].min, props[i].max, props[i].step);
// If significant bit is over 8, two byte access, others one
    if (significant_bit > 8) {
    ret = regmap_raw_read(priv.regmap, props[i].addr, &bval16, sizeof(bval16));
    if (ret)
    return ret;
    val16 = be16_to_cpu(bval16);
    val16 &= ~props[i].mask;
    val16 |= (temp << shift);
    bval16 = cpu_to_be16(val16);
    ret = regmap_raw_write(priv.regmap, props[i].addr, &bval16,
    sizeof(bval16));
    } else {
    ret = regmap_update_bits(priv.regmap, props[i].addr, props[i].mask,
    temp << shift);
    }
    if (ret)
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rtmv20_check_chip_exist(priv: *mut rtmv20_priv) -> c_int {
    static int rtmv20_check_chip_exist(struct rtmv20_priv *priv)
    {
    unsigned int val;
    int ret;
    ret = regmap_read(priv.regmap, RTMV20_REG_DEVINFO, &val);
    if (ret)
    return ret;
    if ((val & RTMV20_VID_MASK) != RICHTEK_VID)
    return -ENODEV;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rtmv20_is_accessible_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool rtmv20_is_accessible_reg(struct device *dev, unsigned int reg)
    {
    switch (reg) {
    case RTMV20_REG_DEVINFO ... RTMV20_REG_STRBVSYNDLYL:
    case RTMV20_REG_LDIRQ:
    case RTMV20_REG_LDSTAT:
    case RTMV20_REG_LDMASK:
    return true;
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn rtmv20_is_volatile_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool rtmv20_is_volatile_reg(struct device *dev, unsigned int reg)
    {
    if (reg == RTMV20_REG_LDIRQ || reg == RTMV20_REG_LDSTAT)
    return true;
    return false;
    }
    static const struct regmap_config rtmv20_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .cache_type = REGCACHE_MAPLE,
    .max_register = RTMV20_REG_LDMASK,
    .num_reg_defaults_raw = RTMV20_MAX_REGS,
    .writeable_reg = rtmv20_is_accessible_reg,
    .readable_reg = rtmv20_is_accessible_reg,
    .volatile_reg = rtmv20_is_volatile_reg,
    };
#[no_mangle]
unsafe extern "C" fn rtmv20_probe(i2c: *mut i2c_client) -> c_int {
    static int rtmv20_probe(struct i2c_client *i2c)
    {
    struct rtmv20_priv *priv;
    let mut config: regulator_config = {};
    int ret;
    priv = devm_kzalloc(&i2c.dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.dev = &i2c.dev;
// Before regmap register, configure HW enable to make I2C accessible
    priv.enable_gpio = devm_gpiod_get(&i2c.dev, "enable", GPIOD_OUT_HIGH);
    if (IS_ERR(priv.enable_gpio)) {
    dev_err(&i2c.dev, "Failed to get enable gpio\n");
    return PTR_ERR(priv.enable_gpio);
    }
// Wait for I2C can be accessed
    usleep_range(RTMV20_I2CRDY_TIMEUS, RTMV20_I2CRDY_TIMEUS + 100);
    priv.regmap = devm_regmap_init_i2c(i2c, &rtmv20_regmap_config);
    if (IS_ERR(priv.regmap)) {
    dev_err(&i2c.dev, "Failed to allocate register map\n");
    return PTR_ERR(priv.regmap);
    }
    ret = rtmv20_check_chip_exist(priv);
    if (ret) {
    dev_err(&i2c.dev, "Chip vendor info is not matched\n");
    return ret;
    }
    ret = rtmv20_properties_init(priv);
    if (ret) {
    dev_err(&i2c.dev, "Failed to init properties\n");
    return ret;
    }
//
// keep in shutdown mode to minimize the current consumption
// and also mark regcache as dirty
//
    regcache_cache_only(priv.regmap, true);
    regcache_mark_dirty(priv.regmap);
    gpiod_set_value(priv.enable_gpio, 0);
    config.dev = &i2c.dev;
    config.regmap = priv.regmap;
    config.driver_data = priv;
    priv.rdev = devm_regulator_register(&i2c.dev, &rtmv20_lsw_desc, &config);
    if (IS_ERR(priv.rdev)) {
    dev_err(&i2c.dev, "Failed to register regulator\n");
    return PTR_ERR(priv.rdev);
    }
// Unmask all events before IRQ registered
    ret = regmap_write(priv.regmap, RTMV20_REG_LDMASK, 0);
    if (ret)
    return ret;
    return devm_request_threaded_irq(&i2c.dev, i2c.irq, core::ptr::null_mut(), rtmv20_irq_handler,
    IRQF_ONESHOT, dev_name(&i2c.dev), priv);
    }
#[no_mangle]
unsafe extern "C" fn rtmv20_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused rtmv20_suspend(struct device *dev)
    {
    struct i2c_client *i2c = to_i2c_client(dev);
//
// When system suspend, disable irq to prevent interrupt trigger
// during I2C bus suspend
//
    disable_irq(i2c.irq);
    if (device_may_wakeup(dev))
    enable_irq_wake(i2c.irq);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rtmv20_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused rtmv20_resume(struct device *dev)
    {
    struct i2c_client *i2c = to_i2c_client(dev);
// Enable irq after I2C bus already resume
    enable_irq(i2c.irq);
    if (device_may_wakeup(dev))
    disable_irq_wake(i2c.irq);
    return 0;
    }
    static SIMPLE_DEV_PM_OPS(rtmv20_pm, rtmv20_suspend, rtmv20_resume);
    static const struct of_device_id __maybe_unused rtmv20_of_id[] = {
    { .compatible = "richtek,rtmv20", },
    {}
    };
    MODULE_DEVICE_TABLE(of, rtmv20_of_id);
    static struct i2c_driver rtmv20_driver = {
    .driver = {
    .name = "rtmv20",
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    .of_match_table = of_match_ptr(rtmv20_of_id),
    .pm = &rtmv20_pm,
    },
    .probe = rtmv20_probe,
    };
    module_i2c_driver(rtmv20_driver);
    MODULE_AUTHOR("ChiYuan Huang <cy_huang@richtek.com>");
    MODULE_DESCRIPTION("Richtek RTMV20 Regulator Driver");
    MODULE_LICENSE("GPL v2");
