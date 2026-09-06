//! Automatically rewritten from C to Rust
//! Source: drivers/input/misc/drv260x.c
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
// DRV260X haptics driver family
//
// Author: Dan Murphy <dmurphy@ti.com>
//
// Copyright:   (C) 2014 Texas Instruments, Inc.
//

pub const DRV260X_STATUS: c_uint = 0x0;
pub const DRV260X_MODE: c_uint = 0x1;
pub const DRV260X_RT_PB_IN: c_uint = 0x2;
pub const DRV260X_LIB_SEL: c_uint = 0x3;
pub const DRV260X_WV_SEQ_1: c_uint = 0x4;
pub const DRV260X_WV_SEQ_2: c_uint = 0x5;
pub const DRV260X_WV_SEQ_3: c_uint = 0x6;
pub const DRV260X_WV_SEQ_4: c_uint = 0x7;
pub const DRV260X_WV_SEQ_5: c_uint = 0x8;
pub const DRV260X_WV_SEQ_6: c_uint = 0x9;
pub const DRV260X_WV_SEQ_7: c_uint = 0xa;
pub const DRV260X_WV_SEQ_8: c_uint = 0xb;
pub const DRV260X_GO: c_uint = 0xc;
pub const DRV260X_OVERDRIVE_OFF: c_uint = 0xd;
pub const DRV260X_SUSTAIN_P_OFF: c_uint = 0xe;
pub const DRV260X_SUSTAIN_N_OFF: c_uint = 0xf;
pub const DRV260X_BRAKE_OFF: c_uint = 0x10;
pub const DRV260X_A_TO_V_CTRL: c_uint = 0x11;
pub const DRV260X_A_TO_V_MIN_INPUT: c_uint = 0x12;
pub const DRV260X_A_TO_V_MAX_INPUT: c_uint = 0x13;
pub const DRV260X_A_TO_V_MIN_OUT: c_uint = 0x14;
pub const DRV260X_A_TO_V_MAX_OUT: c_uint = 0x15;
pub const DRV260X_RATED_VOLT: c_uint = 0x16;
pub const DRV260X_OD_CLAMP_VOLT: c_uint = 0x17;
pub const DRV260X_CAL_COMP: c_uint = 0x18;
pub const DRV260X_CAL_BACK_EMF: c_uint = 0x19;
pub const DRV260X_FEEDBACK_CTRL: c_uint = 0x1a;
pub const DRV260X_CTRL1: c_uint = 0x1b;
pub const DRV260X_CTRL2: c_uint = 0x1c;
pub const DRV260X_CTRL3: c_uint = 0x1d;
pub const DRV260X_CTRL4: c_uint = 0x1e;
pub const DRV260X_CTRL5: c_uint = 0x1f;
pub const DRV260X_LRA_LOOP_PERIOD: c_uint = 0x20;
pub const DRV260X_VBAT_MON: c_uint = 0x21;
pub const DRV260X_LRA_RES_PERIOD: c_uint = 0x22;
pub const DRV260X_MAX_REG: c_uint = 0x23;
pub const DRV260X_GO_BIT: c_uint = 0x01;
// Library Selection
pub const DRV260X_LIB_SEL_MASK: c_uint = 0x07;
pub const DRV260X_LIB_SEL_RAM: c_uint = 0x0;
pub const DRV260X_LIB_SEL_OD: c_uint = 0x1;
pub const DRV260X_LIB_SEL_40_60: c_uint = 0x2;
pub const DRV260X_LIB_SEL_60_80: c_uint = 0x3;
pub const DRV260X_LIB_SEL_100_140: c_uint = 0x4;
pub const DRV260X_LIB_SEL_140_PLUS: c_uint = 0x5;
pub const DRV260X_LIB_SEL_HIZ_MASK: c_uint = 0x10;
pub const DRV260X_LIB_SEL_HIZ_EN: c_uint = 0x01;
pub const DRV260X_LIB_SEL_HIZ_DIS: c_int = 0;
// Mode register

pub const DRV260X_STANDBY_MASK: c_uint = 0x40;
pub const DRV260X_INTERNAL_TRIGGER: c_uint = 0x00;
pub const DRV260X_EXT_TRIGGER_EDGE: c_uint = 0x01;
pub const DRV260X_EXT_TRIGGER_LEVEL: c_uint = 0x02;
pub const DRV260X_PWM_ANALOG_IN: c_uint = 0x03;
pub const DRV260X_AUDIOHAPTIC: c_uint = 0x04;
pub const DRV260X_RT_PLAYBACK: c_uint = 0x05;
pub const DRV260X_DIAGNOSTICS: c_uint = 0x06;
pub const DRV260X_AUTO_CAL: c_uint = 0x07;
// Audio to Haptics Control

pub const DRV260X_AUDIO_HAPTICS_FILTER_100HZ: c_uint = 0x00;
pub const DRV260X_AUDIO_HAPTICS_FILTER_125HZ: c_uint = 0x01;
pub const DRV260X_AUDIO_HAPTICS_FILTER_150HZ: c_uint = 0x02;
pub const DRV260X_AUDIO_HAPTICS_FILTER_200HZ: c_uint = 0x03;
// Min/Max Input/Output Voltages
pub const DRV260X_AUDIO_HAPTICS_MIN_IN_VOLT: c_uint = 0x19;
pub const DRV260X_AUDIO_HAPTICS_MAX_IN_VOLT: c_uint = 0x64;
pub const DRV260X_AUDIO_HAPTICS_MIN_OUT_VOLT: c_uint = 0x19;
pub const DRV260X_AUDIO_HAPTICS_MAX_OUT_VOLT: c_uint = 0xFF;
// Feedback register
pub const DRV260X_FB_REG_ERM_MODE: c_uint = 0x7f;

pub const DRV260X_BRAKE_FACTOR_MASK: c_uint = 0x1f;

pub const DRV260X_LOOP_GAIN_LOW: c_uint = 0xf3;

pub const DRV260X_BEMF_GAIN_0: c_uint = 0xfc;

// Control 1 register

// Control 2 register
pub const DRV260X_IDISS_TIME_45: c_int = 0;

pub const DRV260X_IDISS_TIME_225: c_uint = 0x03;

// Control 3 Register

// Control 4 Register

//
// Timeout for waiting for the GO status bit, in seconds. Should be reasonably
// large to wait for a auto-calibration cycle completion.
//
pub const DRV260X_GO_TIMEOUT_S: c_int = 5;
//
// struct drv260x_data -
// @input_dev: Pointer to the input device
// @client: Pointer to the I2C client
// @regmap: Register map of the device
// @work: Work item used to off load the enable/disable of the vibration
// @enable_gpio: Pointer to the gpio used for enable/disabling
// @regulator: Pointer to the regulator for the IC
// @magnitude: Magnitude of the vibration event
// @mode: The operating mode of the IC (LRA_NO_CAL, ERM or LRA)
// @library: The vibration library to be used
// @rated_voltage: The rated_voltage of the actuator
// @overdrive_voltage: The over drive voltage of the actuator
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drv260x_data {
    pub input_dev: *mut input_dev,
    pub client: *mut i2c_client,
    pub regmap: *mut regmap,
    pub work: work_struct,
    pub enable_gpio: *mut gpio_desc,
    pub regulator: *mut regulator,
    pub magnitude: u8,
    pub mode: u32,
    pub library: u32,
    pub rated_voltage: c_int,
    pub overdrive_voltage: c_int,
}

pub const DRV260X_DEF_RATED_VOLT: c_uint = 0x90;
pub const DRV260X_DEF_OD_CLAMP_VOLT: c_uint = 0x90;
//
// Rated and Overdriver Voltages:
// Calculated using the formula r = v * 255 / 5.6
// where r is what will be written to the register
// and v is the rated or overdriver voltage of the actuator
//
#[no_mangle]
unsafe extern "C" fn drv260x_calculate_voltage(voltage: c_uint) -> c_int {
    static int drv260x_calculate_voltage(unsigned int voltage)
    {
    return (voltage * 255 / 5600);
    }
#[no_mangle]
unsafe extern "C" fn drv260x_worker(work: *mut work_struct) {
    static void drv260x_worker(struct work_struct *work)
    {
    struct drv260x_data *haptics = container_of(work, struct drv260x_data, work);
    int error;
    gpiod_set_value(haptics.enable_gpio, 1);
// Data sheet says to wait 250us before trying to communicate
    udelay(250);
    error = regmap_write(haptics.regmap,
    DRV260X_MODE, DRV260X_RT_PLAYBACK);
    if (error) {
    dev_err(&haptics.client.dev,
    "Failed to write set mode: %d\n", error);
    } else {
    error = regmap_write(haptics.regmap,
    DRV260X_RT_PB_IN, haptics.magnitude);
    if (error)
    dev_err(&haptics.client.dev,
    "Failed to set magnitude: %d\n", error);
    }
    }
    static int drv260x_haptics_play(struct input_dev *input, void *data,
    struct ff_effect *effect)
    {
    struct drv260x_data *haptics = input_get_drvdata(input);
    haptics.mode = DRV260X_LRA_NO_CAL_MODE;
// Scale u16 magnitude into u8 register value
    if (effect.u.rumble.strong_magnitude > 0)
    haptics.magnitude = effect.u.rumble.strong_magnitude >> 8;
#[no_mangle]
pub unsafe extern "C" fn if(0: effect->u.rumble.weak_magnitude >) -> else {
    else if (effect.u.rumble.weak_magnitude > 0)
    haptics.magnitude = effect.u.rumble.weak_magnitude >> 8;
    else
    haptics.magnitude = 0;
    schedule_work(&haptics.work);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn drv260x_close(input: *mut input_dev) {
    static void drv260x_close(struct input_dev *input)
    {
    struct drv260x_data *haptics = input_get_drvdata(input);
    int error;
    cancel_work_sync(&haptics.work);
    error = regmap_write(haptics.regmap, DRV260X_MODE, DRV260X_STANDBY);
    if (error)
    dev_err(&haptics.client.dev,
    "Failed to enter standby mode: %d\n", error);
    gpiod_set_value(haptics.enable_gpio, 0);
    }
    static const struct reg_sequence drv260x_lra_cal_regs[] = {
    { DRV260X_MODE, DRV260X_AUTO_CAL },
    { DRV260X_CTRL3, DRV260X_NG_THRESH_2 | DRV260X_RTP_UNSIGNED_DATA },
    { DRV260X_FEEDBACK_CTRL, DRV260X_FB_REG_LRA_MODE |
    DRV260X_BRAKE_FACTOR_4X | DRV260X_LOOP_GAIN_HIGH },
    };
    static const struct reg_sequence drv260x_lra_init_regs[] = {
    { DRV260X_MODE, DRV260X_RT_PLAYBACK },
    { DRV260X_A_TO_V_CTRL, DRV260X_AUDIO_HAPTICS_PEAK_20MS |
    DRV260X_AUDIO_HAPTICS_FILTER_125HZ },
    { DRV260X_A_TO_V_MIN_INPUT, DRV260X_AUDIO_HAPTICS_MIN_IN_VOLT },
    { DRV260X_A_TO_V_MAX_INPUT, DRV260X_AUDIO_HAPTICS_MAX_IN_VOLT },
    { DRV260X_A_TO_V_MIN_OUT, DRV260X_AUDIO_HAPTICS_MIN_OUT_VOLT },
    { DRV260X_A_TO_V_MAX_OUT, DRV260X_AUDIO_HAPTICS_MAX_OUT_VOLT },
    { DRV260X_FEEDBACK_CTRL, DRV260X_FB_REG_LRA_MODE |
    DRV260X_BRAKE_FACTOR_2X | DRV260X_LOOP_GAIN_MED |
    DRV260X_BEMF_GAIN_3 },
    { DRV260X_CTRL1, DRV260X_STARTUP_BOOST },
    { DRV260X_CTRL2, DRV260X_SAMP_TIME_250 },
    { DRV260X_CTRL3, DRV260X_NG_THRESH_2 | DRV260X_RTP_UNSIGNED_DATA | DRV260X_ANALOG_IN },
    { DRV260X_CTRL4, DRV260X_AUTOCAL_TIME_500MS },
    };
    static const struct reg_sequence drv260x_erm_cal_regs[] = {
    { DRV260X_MODE, DRV260X_AUTO_CAL },
    { DRV260X_A_TO_V_MIN_INPUT, DRV260X_AUDIO_HAPTICS_MIN_IN_VOLT },
    { DRV260X_A_TO_V_MAX_INPUT, DRV260X_AUDIO_HAPTICS_MAX_IN_VOLT },
    { DRV260X_A_TO_V_MIN_OUT, DRV260X_AUDIO_HAPTICS_MIN_OUT_VOLT },
    { DRV260X_A_TO_V_MAX_OUT, DRV260X_AUDIO_HAPTICS_MAX_OUT_VOLT },
    { DRV260X_FEEDBACK_CTRL, DRV260X_BRAKE_FACTOR_3X |
    DRV260X_LOOP_GAIN_MED | DRV260X_BEMF_GAIN_2 },
    { DRV260X_CTRL1, DRV260X_STARTUP_BOOST },
    { DRV260X_CTRL2, DRV260X_SAMP_TIME_250 | DRV260X_BLANK_TIME_75 |
    DRV260X_IDISS_TIME_75 },
    { DRV260X_CTRL3, DRV260X_NG_THRESH_2 | DRV260X_RTP_UNSIGNED_DATA },
    { DRV260X_CTRL4, DRV260X_AUTOCAL_TIME_500MS },
    };
#[no_mangle]
unsafe extern "C" fn drv260x_init(haptics: *mut drv260x_data) -> c_int {
    static int drv260x_init(struct drv260x_data *haptics)
    {
    int error;
    unsigned int cal_buf;
    unsigned long timeout;
    error = regmap_write(haptics.regmap,
    DRV260X_RATED_VOLT, haptics.rated_voltage);
    if (error) {
    dev_err(&haptics.client.dev,
    "Failed to write DRV260X_RATED_VOLT register: %d\n",
    error);
    return error;
    }
    error = regmap_write(haptics.regmap,
    DRV260X_OD_CLAMP_VOLT, haptics.overdrive_voltage);
    if (error) {
    dev_err(&haptics.client.dev,
    "Failed to write DRV260X_OD_CLAMP_VOLT register: %d\n",
    error);
    return error;
    }
    switch (haptics.mode) {
    case DRV260X_LRA_MODE:
    error = regmap_register_patch(haptics.regmap,
    drv260x_lra_cal_regs,
    ARRAY_SIZE(drv260x_lra_cal_regs));
    if (error) {
    dev_err(&haptics.client.dev,
    "Failed to write LRA calibration registers: %d\n",
    error);
    return error;
    }
    break;
    case DRV260X_ERM_MODE:
    error = regmap_register_patch(haptics.regmap,
    drv260x_erm_cal_regs,
    ARRAY_SIZE(drv260x_erm_cal_regs));
    if (error) {
    dev_err(&haptics.client.dev,
    "Failed to write ERM calibration registers: %d\n",
    error);
    return error;
    }
    error = regmap_update_bits(haptics.regmap, DRV260X_LIB_SEL,
    DRV260X_LIB_SEL_MASK,
    haptics.library);
    if (error) {
    dev_err(&haptics.client.dev,
    "Failed to write DRV260X_LIB_SEL register: %d\n",
    error);
    return error;
    }
    break;
    default:
    error = regmap_register_patch(haptics.regmap,
    drv260x_lra_init_regs,
    ARRAY_SIZE(drv260x_lra_init_regs));
    if (error) {
    dev_err(&haptics.client.dev,
    "Failed to write LRA init registers: %d\n",
    error);
    return error;
    }
    error = regmap_update_bits(haptics.regmap, DRV260X_LIB_SEL,
    DRV260X_LIB_SEL_MASK,
    haptics.library);
    if (error) {
    dev_err(&haptics.client.dev,
    "Failed to write DRV260X_LIB_SEL register: %d\n",
    error);
    return error;
    }
// No need to set GO bit here
    return 0;
    }
    error = regmap_write(haptics.regmap, DRV260X_GO, DRV260X_GO_BIT);
    if (error) {
    dev_err(&haptics.client.dev,
    "Failed to write GO register: %d\n",
    error);
    return error;
    }
    timeout = jiffies + DRV260X_GO_TIMEOUT_S * HZ;
    do {
    usleep_range(15000, 15500);
    error = regmap_read(haptics.regmap, DRV260X_GO, &cal_buf);
    if (error) {
    dev_err(&haptics.client.dev,
    "Failed to read GO register: %d\n",
    error);
    return error;
    }
    if (time_after(jiffies, timeout)) {
    dev_err(&haptics.client.dev,
    "Calibration timeout. The device cannot be used.\n");
    return -ETIMEDOUT;
    }
    } while (cal_buf == DRV260X_GO_BIT);
    return 0;
    }
    static const struct regmap_config drv260x_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = DRV260X_MAX_REG,
    .cache_type = REGCACHE_NONE,
    };
#[no_mangle]
unsafe extern "C" fn drv260x_power_off(data: *mut c_void) {
    static void drv260x_power_off(void *data)
    {
    struct drv260x_data *haptics = data;
    regulator_disable(haptics.regulator);
    }
#[no_mangle]
unsafe extern "C" fn drv260x_probe(client: *mut i2c_client) -> c_int {
    static int drv260x_probe(struct i2c_client *client)
    {
    struct device *dev = &client.dev;
    struct drv260x_data *haptics;
    u32 voltage;
    int error;
    haptics = devm_kzalloc(dev, sizeof(*haptics), GFP_KERNEL);
    if (!haptics)
    return -ENOMEM;
    error = device_property_read_u32(dev, "mode", &haptics.mode);
    if (error) {
    dev_err(dev, "Can't fetch 'mode' property: %d\n", error);
    return error;
    }
    if (haptics.mode < DRV260X_LRA_MODE ||
    haptics.mode > DRV260X_ERM_MODE) {
    dev_err(dev, "Vibrator mode is invalid: %i\n", haptics.mode);
    return -EINVAL;
    }
    error = device_property_read_u32(dev, "library-sel", &haptics.library);
    if (error) {
    dev_err(dev, "Can't fetch 'library-sel' property: %d\n", error);
    return error;
    }
    if (haptics.library < DRV260X_LIB_EMPTY ||
    haptics.library > DRV260X_ERM_LIB_F) {
    dev_err(dev,
    "Library value is invalid: %i\n", haptics.library);
    return -EINVAL;
    }
    if (haptics.mode == DRV260X_LRA_MODE &&
    haptics.library != DRV260X_LIB_EMPTY &&
    haptics.library != DRV260X_LIB_LRA) {
    dev_err(dev, "LRA Mode with ERM Library mismatch\n");
    return -EINVAL;
    }
    if (haptics.mode == DRV260X_ERM_MODE &&
    (haptics.library == DRV260X_LIB_EMPTY ||
    haptics.library == DRV260X_LIB_LRA)) {
    dev_err(dev, "ERM Mode with LRA Library mismatch\n");
    return -EINVAL;
    }
    error = device_property_read_u32(dev, "vib-rated-mv", &voltage);
    haptics.rated_voltage = error ? DRV260X_DEF_RATED_VOLT :
    drv260x_calculate_voltage(voltage);
    error = device_property_read_u32(dev, "vib-overdrive-mv", &voltage);
    haptics.overdrive_voltage = error ? DRV260X_DEF_OD_CLAMP_VOLT :
    drv260x_calculate_voltage(voltage);
    haptics.regulator = devm_regulator_get(dev, "vbat");
    if (IS_ERR(haptics.regulator)) {
    error = PTR_ERR(haptics.regulator);
    dev_err(dev, "unable to get regulator, error: %d\n", error);
    return error;
    }
    error = regulator_enable(haptics.regulator);
    if (error) {
    dev_err(dev, "Failed to enable regulator: %d\n", error);
    return error;
    }
    error = devm_add_action_or_reset(dev, drv260x_power_off, haptics);
    if (error)
    return error;
    haptics.enable_gpio = devm_gpiod_get_optional(dev, "enable",
    GPIOD_OUT_HIGH);
    if (IS_ERR(haptics.enable_gpio))
    return PTR_ERR(haptics.enable_gpio);
    haptics.input_dev = devm_input_allocate_device(dev);
    if (!haptics.input_dev) {
    dev_err(dev, "Failed to allocate input device\n");
    return -ENOMEM;
    }
    haptics.input_dev.name = "drv260x:haptics";
    haptics.input_dev.close = drv260x_close;
    input_set_drvdata(haptics.input_dev, haptics);
    input_set_capability(haptics.input_dev, EV_FF, FF_RUMBLE);
    error = input_ff_create_memless(haptics.input_dev, core::ptr::null_mut(),
    drv260x_haptics_play);
    if (error) {
    dev_err(dev, "input_ff_create() failed: %d\n", error);
    return error;
    }
    INIT_WORK(&haptics.work, drv260x_worker);
    haptics.client = client;
    i2c_set_clientdata(client, haptics);
    haptics.regmap = devm_regmap_init_i2c(client, &drv260x_regmap_config);
    if (IS_ERR(haptics.regmap)) {
    error = PTR_ERR(haptics.regmap);
    dev_err(dev, "Failed to allocate register map: %d\n", error);
    return error;
    }
    error = drv260x_init(haptics);
    if (error) {
    dev_err(dev, "Device init failed: %d\n", error);
    return error;
    }
    error = input_register_device(haptics.input_dev);
    if (error) {
    dev_err(dev, "couldn't register input device: %d\n", error);
    return error;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn drv260x_suspend(dev: *mut device) -> c_int {
    static int drv260x_suspend(struct device *dev)
    {
    struct drv260x_data *haptics = dev_get_drvdata(dev);
    int error;
    guard(mutex)(&haptics.input_dev.mutex);
    if (input_device_enabled(haptics.input_dev)) {
    error = regmap_update_bits(haptics.regmap,
    DRV260X_MODE,
    DRV260X_STANDBY_MASK,
    DRV260X_STANDBY);
    if (error) {
    dev_err(dev, "Failed to set standby mode\n");
    return error;
    }
    gpiod_set_value(haptics.enable_gpio, 0);
    error = regulator_disable(haptics.regulator);
    if (error) {
    dev_err(dev, "Failed to disable regulator\n");
    regmap_update_bits(haptics.regmap,
    DRV260X_MODE,
    DRV260X_STANDBY_MASK, 0);
    return error;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn drv260x_resume(dev: *mut device) -> c_int {
    static int drv260x_resume(struct device *dev)
    {
    struct drv260x_data *haptics = dev_get_drvdata(dev);
    int error;
    guard(mutex)(&haptics.input_dev.mutex);
    if (input_device_enabled(haptics.input_dev)) {
    error = regulator_enable(haptics.regulator);
    if (error) {
    dev_err(dev, "Failed to enable regulator\n");
    return error;
    }
    error = regmap_update_bits(haptics.regmap,
    DRV260X_MODE,
    DRV260X_STANDBY_MASK, 0);
    if (error) {
    dev_err(dev, "Failed to unset standby mode\n");
    regulator_disable(haptics.regulator);
    return error;
    }
    gpiod_set_value(haptics.enable_gpio, 1);
    }
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(drv260x_pm_ops, drv260x_suspend, drv260x_resume);
    static const struct i2c_device_id drv260x_id[] = {
    { .name = "drv2604" },
    { .name = "drv2604l" },
    { .name = "drv2605" },
    { .name = "drv2605l" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, drv260x_id);

    static const struct acpi_device_id drv260x_acpi_match[] = {
    { "DRV2604" },
    { }
    };
    MODULE_DEVICE_TABLE(acpi, drv260x_acpi_match);

    static const struct of_device_id drv260x_of_match[] = {
    { .compatible = "ti,drv2604", },
    { .compatible = "ti,drv2604l", },
    { .compatible = "ti,drv2605", },
    { .compatible = "ti,drv2605l", },
    { }
    };
    MODULE_DEVICE_TABLE(of, drv260x_of_match);
    static struct i2c_driver drv260x_driver = {
    .probe		= drv260x_probe,
    .driver		= {
    .name	= "drv260x-haptics",
    .acpi_match_table = ACPI_PTR(drv260x_acpi_match),
    .of_match_table = drv260x_of_match,
    .pm	= pm_sleep_ptr(&drv260x_pm_ops),
    },
    .id_table = drv260x_id,
    };
    module_i2c_driver(drv260x_driver);
    MODULE_DESCRIPTION("TI DRV260x haptics driver");
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Dan Murphy <dmurphy@ti.com>");
