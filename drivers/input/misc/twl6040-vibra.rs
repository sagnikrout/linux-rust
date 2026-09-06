//! Automatically rewritten from C to Rust
//! Source: drivers/input/misc/twl6040-vibra.c
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
// twl6040-vibra.c - TWL6040 Vibrator driver
//
// Author:      Jorge Eduardo Candelaria <jorge.candelaria@ti.com>
// Author:      Misael Lopez Cruz <misael.lopez@ti.com>
//
// Copyright:   (C) 2011 Texas Instruments, Inc.
//
// Based on twl4030-vibra.c by Henrik Saari <henrik.saari@nokia.com>
// Felipe Balbi <felipe.balbi@nokia.com>
// Jari Vanhala <ext-javi.vanhala@nokia.com>
//

pub const EFFECT_DIR_180_DEG: c_uint = 0x8000;
// Recommended modulation index 85%
pub const TWL6040_VIBRA_MOD: c_int = 85;
pub const TWL6040_NUM_SUPPLIES: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vibra_info {
    pub dev: *mut device,
    pub input_dev: *mut input_dev,
    pub play_work: work_struct,
    pub irq: c_int,
    pub enabled: bool,
    pub weak_speed: c_int,
    pub strong_speed: c_int,
    pub direction: c_int,
    pub vibldrv_res: c_uint,
    pub vibrdrv_res: c_uint,
    pub viblmotor_res: c_uint,
    pub vibrmotor_res: c_uint,
    pub supplies: [regulator_bulk_data; TWL6040_NUM_SUPPLIES],
    pub twl6040: *mut twl6040,
}

#[no_mangle]
unsafe extern "C" fn twl6040_vib_irq_handler(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t twl6040_vib_irq_handler(int irq, void *data)
    {
    struct vibra_info *info = data;
    struct twl6040 *twl6040 = info.twl6040;
    u8 status;
    status = twl6040_reg_read(twl6040, TWL6040_REG_STATUS);
    if (status & TWL6040_VIBLOCDET) {
    dev_warn(info.dev, "Left Vibrator overcurrent detected\n");
    twl6040_clear_bits(twl6040, TWL6040_REG_VIBCTLL,
    TWL6040_VIBENA);
    }
    if (status & TWL6040_VIBROCDET) {
    dev_warn(info.dev, "Right Vibrator overcurrent detected\n");
    twl6040_clear_bits(twl6040, TWL6040_REG_VIBCTLR,
    TWL6040_VIBENA);
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn twl6040_vibra_enable(info: *mut vibra_info) {
    static void twl6040_vibra_enable(struct vibra_info *info)
    {
    struct twl6040 *twl6040 = info.twl6040;
    int ret;
    ret = regulator_bulk_enable(ARRAY_SIZE(info.supplies), info.supplies);
    if (ret) {
    dev_err(info.dev, "failed to enable regulators %d\n", ret);
    return;
    }
    twl6040_power(info.twl6040, 1);
    if (twl6040_get_revid(twl6040) <= TWL6040_REV_ES1_1) {
//
// ERRATA: Disable overcurrent protection for at least
// 3ms when enabling vibrator drivers to avoid false
// overcurrent detection
//
    twl6040_reg_write(twl6040, TWL6040_REG_VIBCTLL,
    TWL6040_VIBENA | TWL6040_VIBCTRL);
    twl6040_reg_write(twl6040, TWL6040_REG_VIBCTLR,
    TWL6040_VIBENA | TWL6040_VIBCTRL);
    usleep_range(3000, 3500);
    }
    twl6040_reg_write(twl6040, TWL6040_REG_VIBCTLL,
    TWL6040_VIBENA);
    twl6040_reg_write(twl6040, TWL6040_REG_VIBCTLR,
    TWL6040_VIBENA);
    info.enabled = true;
    }
#[no_mangle]
unsafe extern "C" fn twl6040_vibra_disable(info: *mut vibra_info) {
    static void twl6040_vibra_disable(struct vibra_info *info)
    {
    struct twl6040 *twl6040 = info.twl6040;
    twl6040_reg_write(twl6040, TWL6040_REG_VIBCTLL, 0x00);
    twl6040_reg_write(twl6040, TWL6040_REG_VIBCTLR, 0x00);
    twl6040_power(info.twl6040, 0);
    regulator_bulk_disable(ARRAY_SIZE(info.supplies), info.supplies);
    info.enabled = false;
    }
    static u8 twl6040_vibra_code(int vddvib, int vibdrv_res, int motor_res,
    int speed, int direction)
    {
    int vpk, max_code;
    u8 vibdat;
// output swing
    vpk = (vddvib * motor_res * TWL6040_VIBRA_MOD) /
    (100 * (vibdrv_res + motor_res));
// 50mV per VIBDAT code step
    max_code = vpk / 50;
    if (max_code > TWL6040_VIBDAT_MAX)
    max_code = TWL6040_VIBDAT_MAX;
// scale speed to max allowed code
    vibdat = (u8)((speed * max_code) / USHRT_MAX);
// 2's complement for direction > 180 degrees
    vibdat *= direction;
    return vibdat;
    }
#[no_mangle]
unsafe extern "C" fn twl6040_vibra_set_effect(info: *mut vibra_info) {
    static void twl6040_vibra_set_effect(struct vibra_info *info)
    {
    struct twl6040 *twl6040 = info.twl6040;
    u8 vibdatl, vibdatr;
    int volt;
// weak motor
    volt = regulator_get_voltage(info.supplies[0].consumer) / 1000;
    vibdatl = twl6040_vibra_code(volt, info.vibldrv_res,
    info.viblmotor_res,
    info.weak_speed, info.direction);
// strong motor
    volt = regulator_get_voltage(info.supplies[1].consumer) / 1000;
    vibdatr = twl6040_vibra_code(volt, info.vibrdrv_res,
    info.vibrmotor_res,
    info.strong_speed, info.direction);
    twl6040_reg_write(twl6040, TWL6040_REG_VIBDATL, vibdatl);
    twl6040_reg_write(twl6040, TWL6040_REG_VIBDATR, vibdatr);
    }
#[no_mangle]
unsafe extern "C" fn vibra_play_work(work: *mut work_struct) {
    static void vibra_play_work(struct work_struct *work)
    {
    struct vibra_info *info = container_of(work,
    struct vibra_info, play_work);
    int ret;
// Do not allow effect, while the routing is set to use audio
    ret = twl6040_get_vibralr_status(info.twl6040);
    if (ret & TWL6040_VIBSEL) {
    dev_info(info.dev, "Vibra is configured for audio\n");
    return;
    }
    if (info.weak_speed || info.strong_speed) {
    if (!info.enabled)
    twl6040_vibra_enable(info);
    twl6040_vibra_set_effect(info);
    } else if (info.enabled)
    twl6040_vibra_disable(info);
    }
    static int vibra_play(struct input_dev *input, void *data,
    struct ff_effect *effect)
    {
    struct vibra_info *info = input_get_drvdata(input);
    info.weak_speed = effect.u.rumble.weak_magnitude;
    info.strong_speed = effect.u.rumble.strong_magnitude;
    info.direction = effect.direction < EFFECT_DIR_180_DEG ? 1 : -1;
    schedule_work(&info.play_work);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn twl6040_vibra_close(input: *mut input_dev) {
    static void twl6040_vibra_close(struct input_dev *input)
    {
    struct vibra_info *info = input_get_drvdata(input);
    cancel_work_sync(&info.play_work);
    if (info.enabled)
    twl6040_vibra_disable(info);
    }
#[no_mangle]
unsafe extern "C" fn twl6040_vibra_suspend(dev: *mut device) -> c_int {
    static int twl6040_vibra_suspend(struct device *dev)
    {
    struct platform_device *pdev = to_platform_device(dev);
    struct vibra_info *info = platform_get_drvdata(pdev);
    cancel_work_sync(&info.play_work);
    if (info.enabled)
    twl6040_vibra_disable(info);
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(twl6040_vibra_pm_ops,
    twl6040_vibra_suspend, core::ptr::null_mut());
#[no_mangle]
unsafe extern "C" fn twl6040_vibra_probe(pdev: *mut platform_device) -> c_int {
    static int twl6040_vibra_probe(struct platform_device *pdev)
    {
    struct device *twl6040_core_dev = pdev.dev.parent;
    struct vibra_info *info;
    let mut vddvibl_uV: c_int = 0;
    let mut vddvibr_uV: c_int = 0;
    int error;
    struct device_node *twl6040_core_node __free(device_node) =
    of_get_child_by_name(twl6040_core_dev.of_node, "vibra");
    if (!twl6040_core_node) {
    dev_err(&pdev.dev, "parent of node is missing?\n");
    return -EINVAL;
    }
    info = devm_kzalloc(&pdev.dev, sizeof(*info), GFP_KERNEL);
    if (!info) {
    dev_err(&pdev.dev, "couldn't allocate memory\n");
    return -ENOMEM;
    }
    info.dev = &pdev.dev;
    info.twl6040 = dev_get_drvdata(pdev.dev.parent);
    of_property_read_u32(twl6040_core_node, "ti,vibldrv-res",
    &info.vibldrv_res);
    of_property_read_u32(twl6040_core_node, "ti,vibrdrv-res",
    &info.vibrdrv_res);
    of_property_read_u32(twl6040_core_node, "ti,viblmotor-res",
    &info.viblmotor_res);
    of_property_read_u32(twl6040_core_node, "ti,vibrmotor-res",
    &info.vibrmotor_res);
    of_property_read_u32(twl6040_core_node, "ti,vddvibl-uV", &vddvibl_uV);
    of_property_read_u32(twl6040_core_node, "ti,vddvibr-uV", &vddvibr_uV);
    if ((!info.vibldrv_res && !info.viblmotor_res) ||
    (!info.vibrdrv_res && !info.vibrmotor_res)) {
    dev_err(info.dev, "invalid vibra driver/motor resistance\n");
    return -EINVAL;
    }
    info.irq = platform_get_irq(pdev, 0);
    if (info.irq < 0)
    return -EINVAL;
    error = devm_request_threaded_irq(&pdev.dev, info.irq, core::ptr::null_mut(),
    twl6040_vib_irq_handler,
    IRQF_ONESHOT,
    "twl6040_irq_vib", info);
    if (error) {
    dev_err(info.dev, "VIB IRQ request failed: %d\n", error);
    return error;
    }
    info.supplies[0].supply = "vddvibl";
    info.supplies[1].supply = "vddvibr";
//
// When booted with Device tree the regulators are attached to the
// parent device (twl6040 MFD core)
//
    error = devm_regulator_bulk_get(twl6040_core_dev,
    ARRAY_SIZE(info.supplies),
    info.supplies);
    if (error) {
    dev_err(info.dev, "couldn't get regulators %d\n", error);
    return error;
    }
    if (vddvibl_uV) {
    error = regulator_set_voltage(info.supplies[0].consumer,
    vddvibl_uV, vddvibl_uV);
    if (error) {
    dev_err(info.dev, "failed to set VDDVIBL volt %d\n",
    error);
    return error;
    }
    }
    if (vddvibr_uV) {
    error = regulator_set_voltage(info.supplies[1].consumer,
    vddvibr_uV, vddvibr_uV);
    if (error) {
    dev_err(info.dev, "failed to set VDDVIBR volt %d\n",
    error);
    return error;
    }
    }
    INIT_WORK(&info.play_work, vibra_play_work);
    info.input_dev = devm_input_allocate_device(&pdev.dev);
    if (!info.input_dev) {
    dev_err(info.dev, "couldn't allocate input device\n");
    return -ENOMEM;
    }
    input_set_drvdata(info.input_dev, info);
    info.input_dev.name = "twl6040:vibrator";
    info.input_dev.id.version = 1;
    info.input_dev.close = twl6040_vibra_close;
    __set_bit(FF_RUMBLE, info.input_dev.ffbit);
    error = input_ff_create_memless(info.input_dev, core::ptr::null_mut(), vibra_play);
    if (error) {
    dev_err(info.dev, "couldn't register vibrator to FF\n");
    return error;
    }
    error = input_register_device(info.input_dev);
    if (error) {
    dev_err(info.dev, "couldn't register input device\n");
    return error;
    }
    platform_set_drvdata(pdev, info);
    return 0;
    }
    static struct platform_driver twl6040_vibra_driver = {
    .probe		= twl6040_vibra_probe,
    .driver		= {
    .name	= "twl6040-vibra",
    .pm	= pm_sleep_ptr(&twl6040_vibra_pm_ops),
    },
    };
    module_platform_driver(twl6040_vibra_driver);
    MODULE_ALIAS("platform:twl6040-vibra");
    MODULE_DESCRIPTION("TWL6040 Vibra driver");
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Jorge Eduardo Candelaria <jorge.candelaria@ti.com>");
    MODULE_AUTHOR("Misael Lopez Cruz <misael.lopez@ti.com>");
