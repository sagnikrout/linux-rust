//! Automatically rewritten from C to Rust
//! Source: drivers/net/can/m_can/tcan4x5x-core.c
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
// SPI to CAN driver for the Texas Instruments TCAN4x5x
// Copyright (C) 2018-19 Texas Instruments Incorporated - http://www.ti.com

pub const TCAN4X5X_EXT_CLK_DEF: c_int = 40000000;
pub const TCAN4X5X_DEV_ID1: c_uint = 0x00;
pub const TCAN4X5X_DEV_ID1_TCAN: c_uint = 0x4e414354 /* ASCII TCAN */;
pub const TCAN4X5X_DEV_ID2: c_uint = 0x04;
pub const TCAN4X5X_REV: c_uint = 0x08;
pub const TCAN4X5X_STATUS: c_uint = 0x0C;
pub const TCAN4X5X_ERROR_STATUS_MASK: c_uint = 0x10;
pub const TCAN4X5X_CONTROL: c_uint = 0x14;
pub const TCAN4X5X_CONFIG: c_uint = 0x800;
pub const TCAN4X5X_TS_PRESCALE: c_uint = 0x804;
pub const TCAN4X5X_TEST_REG: c_uint = 0x808;
pub const TCAN4X5X_INT_FLAGS: c_uint = 0x820;
pub const TCAN4X5X_MCAN_INT_REG: c_uint = 0x824;
pub const TCAN4X5X_INT_EN: c_uint = 0x830;
// Interrupt bits

    (TCAN4X5X_MCAN_INT | TCAN4X5X_BUS_FAULT | \
    TCAN4X5X_CANBUS_ERR_INT_EN | TCAN4X5X_CANINT_INT_EN)
// MCAN Interrupt bits

    (TCAN4X5X_MCAN_IR_TC | TCAN4X5X_MCAN_IR_RF0N | \
    TCAN4X5X_MCAN_IR_RF1N | TCAN4X5X_MCAN_IR_RF0F | \
    TCAN4X5X_MCAN_IR_RF1F)
pub const TCAN4X5X_MRAM_START: c_uint = 0x8000;
pub const TCAN4X5X_MRAM_SIZE: c_uint = 0x800;
pub const TCAN4X5X_MCAN_OFFSET: c_uint = 0x1000;
pub const TCAN4X5X_CLEAR_ALL_INT: c_uint = 0xffffffff;
pub const TCAN4X5X_SET_ALL_INT: c_uint = 0xffffffff;

pub const TCAN4X5X_MODE_SLEEP: c_uint = 0x00;

pub const TCAN4X5X_WD_60_MS_TIMER: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcan4x5x_version_info {
    pub name: *const c_char,
    pub id2_register: u32,
    pub has_wake_pin: bool,
    pub has_state_pin: bool,
}

    enum {
    TCAN4552 = 0,
    TCAN4553,
    TCAN4X5X,
    };
    static const struct tcan4x5x_version_info tcan4x5x_versions[] = {
    [TCAN4552] = {
    .name = "4552",
    .id2_register = 0x32353534,
    },
    [TCAN4553] = {
    .name = "4553",
    .id2_register = 0x33353534,
    },
// generic version with no id2_register at the end
    [TCAN4X5X] = {
    .name = "generic",
    .has_wake_pin = true,
    .has_state_pin = true,
    },
    };
    static inline struct tcan4x5x_priv *cdev_to_priv(struct m_can_classdev *cdev)
    {
    return container_of(cdev, struct tcan4x5x_priv, cdev);
    }
#[no_mangle]
unsafe extern "C" fn tcan4x5x_check_wake(priv: *mut tcan4x5x_priv) {
    static void tcan4x5x_check_wake(struct tcan4x5x_priv *priv)
    {
    let mut wake_state: c_int = 0;
    if (priv.device_state_gpio)
    wake_state = gpiod_get_value(priv.device_state_gpio);
    if (priv.device_wake_gpio && wake_state) {
    gpiod_set_value(priv.device_wake_gpio, 0);
    usleep_range(5, 50);
    gpiod_set_value(priv.device_wake_gpio, 1);
    }
    }
#[no_mangle]
unsafe extern "C" fn tcan4x5x_reset(priv: *mut tcan4x5x_priv) -> c_int {
    static int tcan4x5x_reset(struct tcan4x5x_priv *priv)
    {
    let mut ret: c_int = 0;
    if (priv.reset_gpio) {
    gpiod_set_value(priv.reset_gpio, 1);
// tpulse_width minimum 30us
    usleep_range(30, 100);
    gpiod_set_value(priv.reset_gpio, 0);
    } else {
    ret = regmap_write(priv.regmap, TCAN4X5X_CONFIG,
    TCAN4X5X_SW_RESET);
    if (ret)
    return ret;
    }
    usleep_range(700, 1000);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn tcan4x5x_read_reg(cdev: *mut m_can_classdev, reg: c_int) -> u32 {
    static u32 tcan4x5x_read_reg(struct m_can_classdev *cdev, int reg)
    {
    struct tcan4x5x_priv *priv = cdev_to_priv(cdev);
    u32 val;
    regmap_read(priv.regmap, TCAN4X5X_MCAN_OFFSET + reg, &val);
    return val;
    }
    static int tcan4x5x_read_fifo(struct m_can_classdev *cdev, int addr_offset,
    void *val, size_t val_count)
    {
    struct tcan4x5x_priv *priv = cdev_to_priv(cdev);
    return regmap_bulk_read(priv.regmap, TCAN4X5X_MRAM_START + addr_offset, val, val_count);
    }
#[no_mangle]
unsafe extern "C" fn tcan4x5x_write_reg(cdev: *mut m_can_classdev, reg: c_int, val: c_int) -> c_int {
    static int tcan4x5x_write_reg(struct m_can_classdev *cdev, int reg, int val)
    {
    struct tcan4x5x_priv *priv = cdev_to_priv(cdev);
    return regmap_write(priv.regmap, TCAN4X5X_MCAN_OFFSET + reg, val);
    }
    static int tcan4x5x_write_fifo(struct m_can_classdev *cdev,
    int addr_offset, const void *val, size_t val_count)
    {
    struct tcan4x5x_priv *priv = cdev_to_priv(cdev);
    return regmap_bulk_write(priv.regmap, TCAN4X5X_MRAM_START + addr_offset, val, val_count);
    }
#[no_mangle]
unsafe extern "C" fn tcan4x5x_power_enable(priv: *mut tcan4x5x_priv, enable: c_int) -> c_int {
    static int tcan4x5x_power_enable(struct tcan4x5x_priv *priv, int enable)
    {
    struct regulator *reg = priv.power;
//
// Put the device into sleep mode if the RST pin is available,
// since a wake-up event, RST pin toggle, or power cycle are the only
// ways to exit sleep mode.
// Redundant if the regulator is exclusive to this device, but that
// can't be determined here.
//
// Datasheet: TCAN4550, section "8.4.3 Sleep Mode"
// https://www.ti.com/lit/gpn/tcan4550
//
    if (priv.reset_gpio && !enable) {
    int ret;
    ret = regmap_update_bits(priv.regmap, TCAN4X5X_CONFIG,
    TCAN4X5X_MODE_SEL_MASK,
    TCAN4X5X_MODE_SLEEP);
    if (ret)
    dev_err(&priv.spi.dev, "Setting sleep mode failed %pe\n",
    ERR_PTR(ret));
    }
    if (IS_ERR_OR_NULL(reg))
    return 0;
    if (enable)
    return regulator_enable(reg);
    else
    return regulator_disable(reg);
    }
    static int tcan4x5x_write_tcan_reg(struct m_can_classdev *cdev,
    int reg, int val)
    {
    struct tcan4x5x_priv *priv = cdev_to_priv(cdev);
    return regmap_write(priv.regmap, reg, val);
    }
#[no_mangle]
unsafe extern "C" fn tcan4x5x_clear_interrupts(cdev: *mut m_can_classdev) -> c_int {
    static int tcan4x5x_clear_interrupts(struct m_can_classdev *cdev)
    {
    int ret;
    ret = tcan4x5x_write_tcan_reg(cdev, TCAN4X5X_STATUS,
    TCAN4X5X_CLEAR_ALL_INT);
    if (ret)
    return ret;
    return tcan4x5x_write_tcan_reg(cdev, TCAN4X5X_INT_FLAGS,
    TCAN4X5X_CLEAR_ALL_INT);
    }
#[no_mangle]
unsafe extern "C" fn tcan4x5x_init(cdev: *mut m_can_classdev) -> c_int {
    static int tcan4x5x_init(struct m_can_classdev *cdev)
    {
    struct tcan4x5x_priv *tcan4x5x = cdev_to_priv(cdev);
    int ret;
    tcan4x5x_check_wake(tcan4x5x);
    ret = tcan4x5x_clear_interrupts(cdev);
    if (ret)
    return ret;
    ret = tcan4x5x_write_tcan_reg(cdev, TCAN4X5X_INT_EN,
    TCAN4X5X_ENABLE_TCAN_INT);
    if (ret)
    return ret;
    ret = tcan4x5x_write_tcan_reg(cdev, TCAN4X5X_ERROR_STATUS_MASK,
    TCAN4X5X_CLEAR_ALL_INT);
    if (ret)
    return ret;
    ret = regmap_update_bits(tcan4x5x.regmap, TCAN4X5X_CONFIG,
    TCAN4X5X_MODE_SEL_MASK, TCAN4X5X_MODE_NORMAL);
    if (ret)
    return ret;
    if (tcan4x5x.nwkrq_voltage_vio) {
    ret = regmap_set_bits(tcan4x5x.regmap, TCAN4X5X_CONFIG,
    TCAN4X5X_NWKRQ_VOLTAGE_VIO);
    if (ret)
    return ret;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn tcan4x5x_deinit(cdev: *mut m_can_classdev) -> c_int {
    static int tcan4x5x_deinit(struct m_can_classdev *cdev)
    {
    struct tcan4x5x_priv *tcan4x5x = cdev_to_priv(cdev);
    return regmap_update_bits(tcan4x5x.regmap, TCAN4X5X_CONFIG,
    TCAN4X5X_MODE_SEL_MASK, TCAN4X5X_MODE_STANDBY);
    };
#[no_mangle]
unsafe extern "C" fn tcan4x5x_disable_wake(cdev: *mut m_can_classdev) -> c_int {
    static int tcan4x5x_disable_wake(struct m_can_classdev *cdev)
    {
    struct tcan4x5x_priv *tcan4x5x = cdev_to_priv(cdev);
    return regmap_update_bits(tcan4x5x.regmap, TCAN4X5X_CONFIG,
    TCAN4X5X_DISABLE_WAKE_MSK, 0x00);
    }
#[no_mangle]
unsafe extern "C" fn tcan4x5x_disable_state(cdev: *mut m_can_classdev) -> c_int {
    static int tcan4x5x_disable_state(struct m_can_classdev *cdev)
    {
    struct tcan4x5x_priv *tcan4x5x = cdev_to_priv(cdev);
    return regmap_update_bits(tcan4x5x.regmap, TCAN4X5X_CONFIG,
    TCAN4X5X_DISABLE_INH_MSK, 0x01);
    }
    static const struct tcan4x5x_version_info
// tcan4x5x_find_version(struct tcan4x5x_priv *priv)
    {
    u32 val;
    int ret;
    ret = regmap_read(priv.regmap, TCAN4X5X_DEV_ID1, &val);
    if (ret)
    return ERR_PTR(ret);
    if (val != TCAN4X5X_DEV_ID1_TCAN) {
    dev_err(&priv.spi.dev, "Not a tcan device %x\n", val);
    return ERR_PTR(-ENODEV);
    }
    ret = regmap_read(priv.regmap, TCAN4X5X_DEV_ID2, &val);
    if (ret)
    return ERR_PTR(ret);
    for (int i = 0; i != ARRAY_SIZE(tcan4x5x_versions); ++i) {
    const struct tcan4x5x_version_info *vinfo = &tcan4x5x_versions[i];
    if (!vinfo.id2_register || val == vinfo.id2_register) {
    dev_info(&priv.spi.dev, "Detected TCAN device version %s\n",
    vinfo.name);
    return vinfo;
    }
    }
    return &tcan4x5x_versions[TCAN4X5X];
    }
#[no_mangle]
unsafe extern "C" fn tcan4x5x_get_dt_data(cdev: *mut m_can_classdev) {
    static void tcan4x5x_get_dt_data(struct m_can_classdev *cdev)
    {
    struct tcan4x5x_priv *tcan4x5x = cdev_to_priv(cdev);
    tcan4x5x.nwkrq_voltage_vio =
    of_property_read_bool(cdev.dev.of_node, "ti,nwkrq-voltage-vio");
    }
#[no_mangle]
unsafe extern "C" fn tcan4x5x_get_gpios(cdev: *mut m_can_classdev) -> c_int {
    static int tcan4x5x_get_gpios(struct m_can_classdev *cdev)
    {
    struct tcan4x5x_priv *tcan4x5x = cdev_to_priv(cdev);
    int ret;
    tcan4x5x.device_wake_gpio = devm_gpiod_get_optional(cdev.dev,
    "device-wake",
    GPIOD_OUT_HIGH);
    if (IS_ERR(tcan4x5x.device_wake_gpio)) {
    if (PTR_ERR(tcan4x5x.device_wake_gpio) == -EPROBE_DEFER)
    return -EPROBE_DEFER;
    tcan4x5x.device_wake_gpio = core::ptr::null_mut();
    }
    tcan4x5x.reset_gpio = devm_gpiod_get_optional(cdev.dev, "reset",
    GPIOD_OUT_LOW);
    if (IS_ERR(tcan4x5x.reset_gpio))
    tcan4x5x.reset_gpio = core::ptr::null_mut();
    ret = tcan4x5x_reset(tcan4x5x);
    if (ret)
    return ret;
    tcan4x5x.device_state_gpio = devm_gpiod_get_optional(cdev.dev,
    "device-state",
    GPIOD_IN);
    if (IS_ERR(tcan4x5x.device_state_gpio))
    tcan4x5x.device_state_gpio = core::ptr::null_mut();
    return 0;
    }
    static int tcan4x5x_check_gpios(struct m_can_classdev *cdev,
    const struct tcan4x5x_version_info *version_info)
    {
    struct tcan4x5x_priv *tcan4x5x = cdev_to_priv(cdev);
    int ret;
    if (version_info.has_wake_pin && !tcan4x5x.device_wake_gpio) {
    ret = tcan4x5x_disable_wake(cdev);
    if (ret)
    return ret;
    }
    if (version_info.has_state_pin && !tcan4x5x.device_state_gpio) {
    ret = tcan4x5x_disable_state(cdev);
    if (ret)
    return ret;
    }
    return 0;
    }
    static const struct m_can_ops tcan4x5x_ops = {
    .init = tcan4x5x_init,
    .deinit = tcan4x5x_deinit,
    .read_reg = tcan4x5x_read_reg,
    .write_reg = tcan4x5x_write_reg,
    .write_fifo = tcan4x5x_write_fifo,
    .read_fifo = tcan4x5x_read_fifo,
    .clear_interrupts = tcan4x5x_clear_interrupts,
    };
#[no_mangle]
unsafe extern "C" fn tcan4x5x_can_probe(spi: *mut spi_device) -> c_int {
    static int tcan4x5x_can_probe(struct spi_device *spi)
    {
    const struct tcan4x5x_version_info *version_info;
    struct tcan4x5x_priv *priv;
    struct m_can_classdev *mcan_class;
    int freq, ret;
    mcan_class = m_can_class_allocate_dev(&spi.dev,
    sizeof(struct tcan4x5x_priv));
    if (IS_ERR(mcan_class))
    return PTR_ERR(mcan_class);
    ret = m_can_check_mram_cfg(mcan_class, TCAN4X5X_MRAM_SIZE);
    if (ret)
    goto out_m_can_class_free_dev;
    priv = cdev_to_priv(mcan_class);
    priv.power = devm_regulator_get_optional(&spi.dev, "vsup");
    if (IS_ERR(priv.power)) {
    if (PTR_ERR(priv.power) == -EPROBE_DEFER) {
    ret = -EPROBE_DEFER;
    goto out_m_can_class_free_dev;
    }
    priv.power = core::ptr::null_mut();
    }
    mcan_class.cclk = devm_clk_get(mcan_class.dev, "cclk");
    if (IS_ERR(mcan_class.cclk)) {
    dev_err(&spi.dev, "no CAN clock source defined\n");
    freq = TCAN4X5X_EXT_CLK_DEF;
    } else {
    freq = clk_get_rate(mcan_class.cclk);
    }
// Sanity check
    if (freq < 20000000 || freq > TCAN4X5X_EXT_CLK_DEF) {
    dev_err(&spi.dev, "Clock frequency is out of supported range %d\n",
    freq);
    ret = -ERANGE;
    goto out_m_can_class_free_dev;
    }
    priv.spi = spi;
    mcan_class.pm_clock_support = 0;
    mcan_class.pm_wake_source = device_property_read_bool(&spi.dev, "wakeup-source");
    mcan_class.can.clock.freq = freq;
    mcan_class.dev = &spi.dev;
    mcan_class.ops = &tcan4x5x_ops;
    mcan_class.is_peripheral = true;
    mcan_class.net.irq = spi.irq;
    spi_set_drvdata(spi, priv);
// Configure the SPI bus
    spi.bits_per_word = 8;
    ret = spi_setup(spi);
    if (ret) {
    dev_err(&spi.dev, "SPI setup failed %pe\n", ERR_PTR(ret));
    goto out_m_can_class_free_dev;
    }
    ret = tcan4x5x_regmap_init(priv);
    if (ret) {
    dev_err(&spi.dev, "regmap init failed %pe\n", ERR_PTR(ret));
    goto out_m_can_class_free_dev;
    }
    ret = tcan4x5x_power_enable(priv, 1);
    if (ret) {
    dev_err(&spi.dev, "Enabling regulator failed %pe\n",
    ERR_PTR(ret));
    goto out_m_can_class_free_dev;
    }
    ret = tcan4x5x_get_gpios(mcan_class);
    if (ret) {
    dev_err(&spi.dev, "Getting gpios failed %pe\n", ERR_PTR(ret));
    goto out_power;
    }
    version_info = tcan4x5x_find_version(priv);
    if (IS_ERR(version_info)) {
    ret = PTR_ERR(version_info);
    goto out_power;
    }
    ret = tcan4x5x_check_gpios(mcan_class, version_info);
    if (ret) {
    dev_err(&spi.dev, "Checking gpios failed %pe\n", ERR_PTR(ret));
    goto out_power;
    }
    tcan4x5x_get_dt_data(mcan_class);
    tcan4x5x_check_wake(priv);
    ret = tcan4x5x_write_tcan_reg(mcan_class, TCAN4X5X_INT_EN, 0);
    if (ret) {
    dev_err(&spi.dev, "Disabling interrupts failed %pe\n", ERR_PTR(ret));
    goto out_power;
    }
    ret = tcan4x5x_clear_interrupts(mcan_class);
    if (ret) {
    dev_err(&spi.dev, "Clearing interrupts failed %pe\n", ERR_PTR(ret));
    goto out_power;
    }
    if (mcan_class.pm_wake_source)
    device_init_wakeup(&spi.dev, true);
    ret = m_can_class_register(mcan_class);
    if (ret) {
    dev_err(&spi.dev, "Failed registering m_can device %pe\n",
    ERR_PTR(ret));
    goto out_power;
    }
    netdev_info(mcan_class.net, "TCAN4X5X successfully initialized.\n");
    return 0;
    out_power:
    tcan4x5x_power_enable(priv, 0);
    out_m_can_class_free_dev:
    m_can_class_free_dev(mcan_class.net);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn tcan4x5x_can_remove(spi: *mut spi_device) {
    static void tcan4x5x_can_remove(struct spi_device *spi)
    {
    struct tcan4x5x_priv *priv = spi_get_drvdata(spi);
    m_can_class_unregister(&priv.cdev);
    tcan4x5x_power_enable(priv, 0);
    m_can_class_free_dev(priv.cdev.net);
    }
#[no_mangle]
unsafe extern "C" fn tcan4x5x_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused tcan4x5x_suspend(struct device *dev)
    {
    struct m_can_classdev *cdev = dev_get_drvdata(dev);
    struct spi_device *spi = to_spi_device(dev);
    if (cdev.pm_wake_source)
    enable_irq_wake(spi.irq);
    return m_can_class_suspend(dev);
    }
#[no_mangle]
unsafe extern "C" fn tcan4x5x_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused tcan4x5x_resume(struct device *dev)
    {
    struct m_can_classdev *cdev = dev_get_drvdata(dev);
    struct spi_device *spi = to_spi_device(dev);
    let mut ret: c_int = m_can_class_resume(dev);
    if (cdev.pm_wake_source)
    disable_irq_wake(spi.irq);
    return ret;
    }
    static const struct of_device_id tcan4x5x_of_match[] = {
    {
    .compatible = "ti,tcan4x5x",
    }, {
// sentinel
    },
    };
    MODULE_DEVICE_TABLE(of, tcan4x5x_of_match);
    static const struct spi_device_id tcan4x5x_id_table[] = {
    {
    .name = "tcan4x5x",
    }, {
// sentinel
    },
    };
    MODULE_DEVICE_TABLE(spi, tcan4x5x_id_table);
    static const struct dev_pm_ops tcan4x5x_pm_ops = {
    SET_SYSTEM_SLEEP_PM_OPS(tcan4x5x_suspend, tcan4x5x_resume)
    };
    static struct spi_driver tcan4x5x_can_driver = {
    .driver = {
    .name = KBUILD_MODNAME,
    .of_match_table = tcan4x5x_of_match,
    .pm = &tcan4x5x_pm_ops,
    },
    .id_table = tcan4x5x_id_table,
    .probe = tcan4x5x_can_probe,
    .remove = tcan4x5x_can_remove,
    };
    module_spi_driver(tcan4x5x_can_driver);
    MODULE_AUTHOR("Dan Murphy <dmurphy@ti.com>");
    MODULE_DESCRIPTION("Texas Instruments TCAN4x5x CAN driver");
    MODULE_LICENSE("GPL v2");
