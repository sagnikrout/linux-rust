//! Automatically rewritten from C to Rust
//! Source: drivers/input/misc/pmic8xxx-pwrkey.c
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
// Copyright (c) 2010-2011, Code Aurora Forum. All rights reserved.
//

pub const PON_CNTL_1: c_uint = 0x1C;

pub const PON_CNTL_1_PULL_UP_EN: c_uint = 0xe0;
pub const PON_CNTL_1_USB_PWR_EN: c_uint = 0x10;
pub const PON_CNTL_1_WD_EN_RESET: c_uint = 0x08;
pub const PM8058_SLEEP_CTRL: c_uint = 0x02b;
pub const PM8921_SLEEP_CTRL: c_uint = 0x10a;
pub const SLEEP_CTRL_SMPL_EN_RESET: c_uint = 0x04;
// Regulator master enable addresses
pub const REG_PM8058_VREG_EN_MSM: c_uint = 0x018;
pub const REG_PM8058_VREG_EN_GRP_5_4: c_uint = 0x1c8;
// Regulator control registers for shutdown/reset
pub const PM8058_S0_CTRL: c_uint = 0x004;
pub const PM8058_S1_CTRL: c_uint = 0x005;
pub const PM8058_S3_CTRL: c_uint = 0x111;
pub const PM8058_L21_CTRL: c_uint = 0x120;
pub const PM8058_L22_CTRL: c_uint = 0x121;
pub const PM8058_REGULATOR_ENABLE_MASK: c_uint = 0x80;
pub const PM8058_REGULATOR_ENABLE: c_uint = 0x80;
pub const PM8058_REGULATOR_DISABLE: c_uint = 0x00;
pub const PM8058_REGULATOR_PULL_DOWN_MASK: c_uint = 0x40;
pub const PM8058_REGULATOR_PULL_DOWN_EN: c_uint = 0x40;
// Buck CTRL register
pub const PM8058_SMPS_LEGACY_VREF_SEL: c_uint = 0x20;
pub const PM8058_SMPS_LEGACY_VPROG_MASK: c_uint = 0x1f;
pub const PM8058_SMPS_ADVANCED_BAND_MASK: c_uint = 0xC0;
pub const PM8058_SMPS_ADVANCED_BAND_SHIFT: c_int = 6;
pub const PM8058_SMPS_ADVANCED_VPROG_MASK: c_uint = 0x3f;
// Buck TEST2 registers for shutdown/reset
pub const PM8058_S0_TEST2: c_uint = 0x084;
pub const PM8058_S1_TEST2: c_uint = 0x085;
pub const PM8058_S3_TEST2: c_uint = 0x11a;
pub const PM8058_REGULATOR_BANK_WRITE: c_uint = 0x80;
pub const PM8058_REGULATOR_BANK_MASK: c_uint = 0x70;
pub const PM8058_REGULATOR_BANK_SHIFT: c_int = 4;

// Buck TEST2 register bank 1
pub const PM8058_SMPS_LEGACY_VLOW_SEL: c_uint = 0x01;
// Buck TEST2 register bank 7
pub const PM8058_SMPS_ADVANCED_MODE_MASK: c_uint = 0x02;
pub const PM8058_SMPS_ADVANCED_MODE: c_uint = 0x02;
pub const PM8058_SMPS_LEGACY_MODE: c_uint = 0x00;
//
// struct pmic8xxx_pwrkey - pmic8xxx pwrkey information
// @key_press_irq: key press irq number
// @regmap: device regmap
// @shutdown_fn: shutdown configuration function
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmic8xxx_pwrkey {
    pub key_press_irq: c_int,
    pub regmap: *mut regmap,
    pub bool): *mut *mut *mut int (shutdown_fn)(struct pmic8xxx_pwrkey ,,
}

#[no_mangle]
unsafe extern "C" fn pwrkey_press_irq(irq: c_int, _pwr: *mut c_void) -> irqreturn_t {
    static irqreturn_t pwrkey_press_irq(int irq, void *_pwr)
    {
    struct input_dev *pwr = _pwr;
    input_report_key(pwr, KEY_POWER, 1);
    input_sync(pwr);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn pwrkey_release_irq(irq: c_int, _pwr: *mut c_void) -> irqreturn_t {
    static irqreturn_t pwrkey_release_irq(int irq, void *_pwr)
    {
    struct input_dev *pwr = _pwr;
    input_report_key(pwr, KEY_POWER, 0);
    input_sync(pwr);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn pmic8xxx_pwrkey_suspend(dev: *mut device) -> c_int {
    static int pmic8xxx_pwrkey_suspend(struct device *dev)
    {
    struct pmic8xxx_pwrkey *pwrkey = dev_get_drvdata(dev);
    if (device_may_wakeup(dev))
    enable_irq_wake(pwrkey.key_press_irq);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pmic8xxx_pwrkey_resume(dev: *mut device) -> c_int {
    static int pmic8xxx_pwrkey_resume(struct device *dev)
    {
    struct pmic8xxx_pwrkey *pwrkey = dev_get_drvdata(dev);
    if (device_may_wakeup(dev))
    disable_irq_wake(pwrkey.key_press_irq);
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(pm8xxx_pwr_key_pm_ops,
    pmic8xxx_pwrkey_suspend, pmic8xxx_pwrkey_resume);
#[no_mangle]
unsafe extern "C" fn pmic8xxx_pwrkey_shutdown(pdev: *mut platform_device) {
    static void pmic8xxx_pwrkey_shutdown(struct platform_device *pdev)
    {
    struct pmic8xxx_pwrkey *pwrkey = platform_get_drvdata(pdev);
    int error;
    u8 mask, val;
    let mut reset: bool = system_state == SYSTEM_RESTART;
    if (pwrkey.shutdown_fn) {
    error = pwrkey.shutdown_fn(pwrkey, reset);
    if (error)
    return;
    }
//
// Select action to perform (reset or shutdown) when PS_HOLD goes low.
// Also ensure that KPD, CBL0, and CBL1 pull ups are enabled and that
// USB charging is enabled.
//
    mask = PON_CNTL_1_PULL_UP_EN | PON_CNTL_1_USB_PWR_EN;
    mask |= PON_CNTL_1_WD_EN_RESET;
    val = mask;
    if (!reset)
    val &= ~PON_CNTL_1_WD_EN_RESET;
    regmap_update_bits(pwrkey.regmap, PON_CNTL_1, mask, val);
    }
//
// Set an SMPS regulator to be disabled in its CTRL register, but enabled
// in the master enable register.  Also set it's pull down enable bit.
// Take care to make sure that the output voltage doesn't change if switching
// from advanced mode to legacy mode.
//
    static int pm8058_disable_smps_locally_set_pull_down(struct regmap *regmap,
    u16 ctrl_addr, u16 test2_addr, u16 master_enable_addr,
    u8 master_enable_bit)
    {
    int error;
    u8 vref_sel, vlow_sel, band, vprog, bank;
    unsigned int reg;
    bank = PM8058_REGULATOR_BANK_SEL(7);
    error = regmap_write(regmap, test2_addr, bank);
    if (error)
    return error;
    error = regmap_read(regmap, test2_addr, &reg);
    if (error)
    return error;
    reg &= PM8058_SMPS_ADVANCED_MODE_MASK;
// Check if in advanced mode.
    if (reg == PM8058_SMPS_ADVANCED_MODE) {
// Determine current output voltage.
    error = regmap_read(regmap, ctrl_addr, &reg);
    if (error)
    return error;
    band = reg & PM8058_SMPS_ADVANCED_BAND_MASK;
    band >>= PM8058_SMPS_ADVANCED_BAND_SHIFT;
    switch (band) {
    case 3:
    vref_sel = 0;
    vlow_sel = 0;
    break;
    case 2:
    vref_sel = PM8058_SMPS_LEGACY_VREF_SEL;
    vlow_sel = 0;
    break;
    case 1:
    vref_sel = PM8058_SMPS_LEGACY_VREF_SEL;
    vlow_sel = PM8058_SMPS_LEGACY_VLOW_SEL;
    break;
    default:
    pr_err("%s: regulator already disabled\n", __func__);
    return -EPERM;
    }
    vprog = reg & PM8058_SMPS_ADVANCED_VPROG_MASK;
// Round up if fine step is in use.
    vprog = (vprog + 1) >> 1;
    if (vprog > PM8058_SMPS_LEGACY_VPROG_MASK)
    vprog = PM8058_SMPS_LEGACY_VPROG_MASK;
// Set VLOW_SEL bit.
    bank = PM8058_REGULATOR_BANK_SEL(1);
    error = regmap_write(regmap, test2_addr, bank);
    if (error)
    return error;
    error = regmap_update_bits(regmap, test2_addr,
    PM8058_REGULATOR_BANK_WRITE | PM8058_REGULATOR_BANK_MASK
    | PM8058_SMPS_LEGACY_VLOW_SEL,
    PM8058_REGULATOR_BANK_WRITE |
    PM8058_REGULATOR_BANK_SEL(1) | vlow_sel);
    if (error)
    return error;
// Switch to legacy mode
    bank = PM8058_REGULATOR_BANK_SEL(7);
    error = regmap_write(regmap, test2_addr, bank);
    if (error)
    return error;
    error = regmap_update_bits(regmap, test2_addr,
    PM8058_REGULATOR_BANK_WRITE |
    PM8058_REGULATOR_BANK_MASK |
    PM8058_SMPS_ADVANCED_MODE_MASK,
    PM8058_REGULATOR_BANK_WRITE |
    PM8058_REGULATOR_BANK_SEL(7) |
    PM8058_SMPS_LEGACY_MODE);
    if (error)
    return error;
// Enable locally, enable pull down, keep voltage the same.
    error = regmap_update_bits(regmap, ctrl_addr,
    PM8058_REGULATOR_ENABLE_MASK |
    PM8058_REGULATOR_PULL_DOWN_MASK |
    PM8058_SMPS_LEGACY_VREF_SEL |
    PM8058_SMPS_LEGACY_VPROG_MASK,
    PM8058_REGULATOR_ENABLE | PM8058_REGULATOR_PULL_DOWN_EN
    | vref_sel | vprog);
    if (error)
    return error;
    }
// Enable in master control register.
    error = regmap_update_bits(regmap, master_enable_addr,
    master_enable_bit, master_enable_bit);
    if (error)
    return error;
// Disable locally and enable pull down.
    return regmap_update_bits(regmap, ctrl_addr,
    PM8058_REGULATOR_ENABLE_MASK | PM8058_REGULATOR_PULL_DOWN_MASK,
    PM8058_REGULATOR_DISABLE | PM8058_REGULATOR_PULL_DOWN_EN);
    }
    static int pm8058_disable_ldo_locally_set_pull_down(struct regmap *regmap,
    u16 ctrl_addr, u16 master_enable_addr, u8 master_enable_bit)
    {
    int error;
// Enable LDO in master control register.
    error = regmap_update_bits(regmap, master_enable_addr,
    master_enable_bit, master_enable_bit);
    if (error)
    return error;
// Disable LDO in CTRL register and set pull down
    return regmap_update_bits(regmap, ctrl_addr,
    PM8058_REGULATOR_ENABLE_MASK | PM8058_REGULATOR_PULL_DOWN_MASK,
    PM8058_REGULATOR_DISABLE | PM8058_REGULATOR_PULL_DOWN_EN);
    }
#[no_mangle]
unsafe extern "C" fn pm8058_pwrkey_shutdown(pwrkey: *mut pmic8xxx_pwrkey, reset: bool) -> c_int {
    static int pm8058_pwrkey_shutdown(struct pmic8xxx_pwrkey *pwrkey, bool reset)
    {
    int error;
    struct regmap *regmap = pwrkey.regmap;
    u8 mask, val;
// When shutting down, enable active pulldowns on important rails.
    if (!reset) {
// Disable SMPS's 0,1,3 locally and set pulldown enable bits.
    pm8058_disable_smps_locally_set_pull_down(regmap,
    PM8058_S0_CTRL, PM8058_S0_TEST2,
    REG_PM8058_VREG_EN_MSM, BIT(7));
    pm8058_disable_smps_locally_set_pull_down(regmap,
    PM8058_S1_CTRL, PM8058_S1_TEST2,
    REG_PM8058_VREG_EN_MSM, BIT(6));
    pm8058_disable_smps_locally_set_pull_down(regmap,
    PM8058_S3_CTRL, PM8058_S3_TEST2,
    REG_PM8058_VREG_EN_GRP_5_4, BIT(7) | BIT(4));
// Disable LDO 21 locally and set pulldown enable bit.
    pm8058_disable_ldo_locally_set_pull_down(regmap,
    PM8058_L21_CTRL, REG_PM8058_VREG_EN_GRP_5_4,
    BIT(1));
    }
//
// Fix-up: Set regulator LDO22 to 1.225 V in high power mode. Leave its
// pull-down state intact. This ensures a safe shutdown.
//
    error = regmap_update_bits(regmap, PM8058_L22_CTRL, 0xbf, 0x93);
    if (error)
    return error;
// Enable SMPL if resetting is desired
    mask = SLEEP_CTRL_SMPL_EN_RESET;
    val = 0;
    if (reset)
    val = mask;
    return regmap_update_bits(regmap, PM8058_SLEEP_CTRL, mask, val);
    }
#[no_mangle]
unsafe extern "C" fn pm8921_pwrkey_shutdown(pwrkey: *mut pmic8xxx_pwrkey, reset: bool) -> c_int {
    static int pm8921_pwrkey_shutdown(struct pmic8xxx_pwrkey *pwrkey, bool reset)
    {
    struct regmap *regmap = pwrkey.regmap;
    let mut mask: u8 = SLEEP_CTRL_SMPL_EN_RESET;
    let mut val: u8 = 0;
// Enable SMPL if resetting is desired
    if (reset)
    val = mask;
    return regmap_update_bits(regmap, PM8921_SLEEP_CTRL, mask, val);
    }
#[no_mangle]
unsafe extern "C" fn pmic8xxx_pwrkey_probe(pdev: *mut platform_device) -> c_int {
    static int pmic8xxx_pwrkey_probe(struct platform_device *pdev)
    {
    struct input_dev *pwr;
    let mut key_release_irq: c_int = platform_get_irq(pdev, 0);
    let mut key_press_irq: c_int = platform_get_irq(pdev, 1);
    int err;
    unsigned int delay;
    unsigned int pon_cntl;
    struct regmap *regmap;
    struct pmic8xxx_pwrkey *pwrkey;
    u32 kpd_delay;
    bool pull_up;
    if (of_property_read_u32(pdev.dev.of_node, "debounce", &kpd_delay))
    kpd_delay = 15625;
// Valid range of pwr key trigger delay is 1/64 sec to 2 seconds.
    if (kpd_delay > USEC_PER_SEC * 2 || kpd_delay < USEC_PER_SEC / 64) {
    dev_err(&pdev.dev, "invalid power key trigger delay\n");
    return -EINVAL;
    }
    pull_up = of_property_read_bool(pdev.dev.of_node, "pull-up");
    regmap = dev_get_regmap(pdev.dev.parent, core::ptr::null_mut());
    if (!regmap) {
    dev_err(&pdev.dev, "failed to locate regmap for the device\n");
    return -ENODEV;
    }
    pwrkey = devm_kzalloc(&pdev.dev, sizeof(*pwrkey), GFP_KERNEL);
    if (!pwrkey)
    return -ENOMEM;
    pwrkey.shutdown_fn = of_device_get_match_data(&pdev.dev);
    pwrkey.regmap = regmap;
    pwrkey.key_press_irq = key_press_irq;
    pwr = devm_input_allocate_device(&pdev.dev);
    if (!pwr) {
    dev_dbg(&pdev.dev, "Can't allocate power button\n");
    return -ENOMEM;
    }
    input_set_capability(pwr, EV_KEY, KEY_POWER);
    pwr.name = "pmic8xxx_pwrkey";
    pwr.phys = "pmic8xxx_pwrkey/input0";
    delay = (kpd_delay << 6) / USEC_PER_SEC;
    delay = ilog2(delay);
    err = regmap_read(regmap, PON_CNTL_1, &pon_cntl);
    if (err < 0) {
    dev_err(&pdev.dev, "failed reading PON_CNTL_1 err=%d\n", err);
    return err;
    }
    pon_cntl &= ~PON_CNTL_TRIG_DELAY_MASK;
    pon_cntl |= (delay & PON_CNTL_TRIG_DELAY_MASK);
    if (pull_up)
    pon_cntl |= PON_CNTL_PULL_UP;
    else
    pon_cntl &= ~PON_CNTL_PULL_UP;
    err = regmap_write(regmap, PON_CNTL_1, pon_cntl);
    if (err < 0) {
    dev_err(&pdev.dev, "failed writing PON_CNTL_1 err=%d\n", err);
    return err;
    }
    err = devm_request_irq(&pdev.dev, key_press_irq, pwrkey_press_irq,
    IRQF_TRIGGER_RISING,
    "pmic8xxx_pwrkey_press", pwr);
    if (err) {
    dev_err(&pdev.dev, "Can't get %d IRQ for pwrkey: %d\n",
    key_press_irq, err);
    return err;
    }
    err = devm_request_irq(&pdev.dev, key_release_irq, pwrkey_release_irq,
    IRQF_TRIGGER_RISING,
    "pmic8xxx_pwrkey_release", pwr);
    if (err) {
    dev_err(&pdev.dev, "Can't get %d IRQ for pwrkey: %d\n",
    key_release_irq, err);
    return err;
    }
    err = input_register_device(pwr);
    if (err) {
    dev_err(&pdev.dev, "Can't register power key: %d\n", err);
    return err;
    }
    platform_set_drvdata(pdev, pwrkey);
    device_init_wakeup(&pdev.dev, 1);
    return 0;
    }
    static const struct of_device_id pm8xxx_pwr_key_id_table[] = {
    { .compatible = "qcom,pm8058-pwrkey", .data = &pm8058_pwrkey_shutdown },
    { .compatible = "qcom,pm8921-pwrkey", .data = &pm8921_pwrkey_shutdown },
    { }
    };
    MODULE_DEVICE_TABLE(of, pm8xxx_pwr_key_id_table);
    static struct platform_driver pmic8xxx_pwrkey_driver = {
    .probe		= pmic8xxx_pwrkey_probe,
    .shutdown	= pmic8xxx_pwrkey_shutdown,
    .driver		= {
    .name	= "pm8xxx-pwrkey",
    .pm	= pm_sleep_ptr(&pm8xxx_pwr_key_pm_ops),
    .of_match_table = pm8xxx_pwr_key_id_table,
    },
    };
    module_platform_driver(pmic8xxx_pwrkey_driver);
    MODULE_ALIAS("platform:pmic8xxx_pwrkey");
    MODULE_DESCRIPTION("PMIC8XXX Power Key driver");
    MODULE_LICENSE("GPL v2");
    MODULE_AUTHOR("Trilok Soni <tsoni@codeaurora.org>");
