//! Automatically rewritten from C to Rust
//! Source: drivers/leds/rgb/leds-ktd202x.c
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
// Kinetic KTD2026/7 RGB/White LED driver with I2C interface
//
// Copyright 2023 André Apitzsch <git@apitzsch.eu>
//
// Datasheet: https://www.kinet-ic.com/uploads/KTD2026-7-04h.pdf
//

pub const KTD2026_NUM_LEDS: c_int = 3;
pub const KTD2027_NUM_LEDS: c_int = 4;
pub const KTD202X_MAX_LEDS: c_int = 4;
// Register bank
pub const KTD202X_REG_RESET_CONTROL: c_uint = 0x00;
pub const KTD202X_REG_FLASH_PERIOD: c_uint = 0x01;
pub const KTD202X_REG_PWM1_TIMER: c_uint = 0x02;
pub const KTD202X_REG_PWM2_TIMER: c_uint = 0x03;
pub const KTD202X_REG_CHANNEL_CTRL: c_uint = 0x04;
pub const KTD202X_REG_TRISE_FALL: c_uint = 0x05;

// Register 0
pub const KTD202X_TIMER_SLOT_CONTROL_TSLOT1: c_uint = 0x00;
pub const KTD202X_TIMER_SLOT_CONTROL_TSLOT2: c_uint = 0x01;
pub const KTD202X_TIMER_SLOT_CONTROL_TSLOT3: c_uint = 0x02;
pub const KTD202X_TIMER_SLOT_CONTROL_TSLOT4: c_uint = 0x03;
pub const KTD202X_RSTR_RESET: c_uint = 0x07;
pub const KTD202X_ENABLE_CTRL_WAKE: c_uint = 0x00 /* SCL High & SDA High */;
pub const KTD202X_ENABLE_CTRL_SLEEP: c_uint = 0x08 /* SCL High & SDA Toggling */;
pub const KTD202X_TRISE_FALL_SCALE_NORMAL: c_uint = 0x00;
pub const KTD202X_TRISE_FALL_SCALE_SLOW_X2: c_uint = 0x20;
pub const KTD202X_TRISE_FALL_SCALE_SLOW_X4: c_uint = 0x40;
pub const KTD202X_TRISE_FALL_SCALE_FAST_X8: c_uint = 0x60;
// Register 1
pub const KTD202X_FLASH_PERIOD_256_MS_LOG_RAMP: c_uint = 0x00;
// Register 2-3
pub const KTD202X_FLASH_ON_TIME_0_4_PERCENT: c_uint = 0x01;
// Register 4

pub const KTD202X_CHANNEL_CTRL_OFF: c_uint = 0x00;

// Register 5
pub const KTD202X_RAMP_TIMES_2_MS: c_uint = 0x00;
// Register 6-9
pub const KTD202X_LED_CURRENT_10_mA: c_uint = 0x4f;
pub const KTD202X_FLASH_PERIOD_MIN_MS: c_int = 256;
pub const KTD202X_FLASH_PERIOD_STEP_MS: c_int = 128;
pub const KTD202X_FLASH_PERIOD_MAX_STEPS: c_int = 126;
pub const KTD202X_FLASH_ON_MAX: c_int = 256;
pub const KTD202X_MAX_BRIGHTNESS: c_int = 192;
    static const struct reg_default ktd202x_reg_defaults[] = {
    { KTD202X_REG_RESET_CONTROL, KTD202X_TIMER_SLOT_CONTROL_TSLOT1 |
    KTD202X_ENABLE_CTRL_WAKE | KTD202X_TRISE_FALL_SCALE_NORMAL },
    { KTD202X_REG_FLASH_PERIOD, KTD202X_FLASH_PERIOD_256_MS_LOG_RAMP },
    { KTD202X_REG_PWM1_TIMER, KTD202X_FLASH_ON_TIME_0_4_PERCENT },
    { KTD202X_REG_PWM2_TIMER, KTD202X_FLASH_ON_TIME_0_4_PERCENT },
    { KTD202X_REG_CHANNEL_CTRL, KTD202X_CHANNEL_CTRL_OFF },
    { KTD202X_REG_TRISE_FALL, KTD202X_RAMP_TIMES_2_MS },
    { KTD202X_REG_LED_IOUT(0), KTD202X_LED_CURRENT_10_mA },
    { KTD202X_REG_LED_IOUT(1), KTD202X_LED_CURRENT_10_mA },
    { KTD202X_REG_LED_IOUT(2), KTD202X_LED_CURRENT_10_mA },
    { KTD202X_REG_LED_IOUT(3), KTD202X_LED_CURRENT_10_mA },
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ktd202x_led {
    pub chip: *mut ktd202x,
    union {
    pub cdev: led_classdev,
    pub mcdev: led_classdev_mc,
}

    u32 index;
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ktd202x {
    pub mutex: mutex,
    pub regulators: [regulator_bulk_data; 2],
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub enabled: bool,
    pub num_leds: c_ulong,
    pub __counted_by(num_leds): ktd202x_led leds[],
}

#[no_mangle]
unsafe extern "C" fn ktd202x_chip_disable(chip: *mut ktd202x) -> c_int {
    static int ktd202x_chip_disable(struct ktd202x *chip)
    {
    int ret;
    if (!chip.enabled)
    return 0;
    regmap_write(chip.regmap, KTD202X_REG_RESET_CONTROL, KTD202X_ENABLE_CTRL_SLEEP);
    ret = regulator_bulk_disable(ARRAY_SIZE(chip.regulators), chip.regulators);
    if (ret) {
    dev_err(chip.dev, "Failed to disable regulators: %d\n", ret);
    return ret;
    }
    chip.enabled = false;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ktd202x_chip_enable(chip: *mut ktd202x) -> c_int {
    static int ktd202x_chip_enable(struct ktd202x *chip)
    {
    int ret;
    if (chip.enabled)
    return 0;
    ret = regulator_bulk_enable(ARRAY_SIZE(chip.regulators), chip.regulators);
    if (ret) {
    dev_err(chip.dev, "Failed to enable regulators: %d\n", ret);
    return ret;
    }
    chip.enabled = true;
    ret = regmap_write(chip.regmap, KTD202X_REG_RESET_CONTROL, KTD202X_ENABLE_CTRL_WAKE);
    if (ret) {
    dev_err(chip.dev, "Failed to enable the chip: %d\n", ret);
    ktd202x_chip_disable(chip);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ktd202x_chip_in_use(chip: *mut ktd202x) -> bool {
    static bool ktd202x_chip_in_use(struct ktd202x *chip)
    {
    int i;
    for (i = 0; i < chip.num_leds; i++) {
    if (chip.leds[i].cdev.brightness)
    return true;
    }
    return false;
    }
    static int ktd202x_brightness_set(struct ktd202x_led *led,
    struct mc_subled *subleds,
    unsigned int num_channels)
    {
    let mut mode_blink: bool = false;
    int channel;
    int state;
    int ret;
    int i;
    if (ktd202x_chip_in_use(led.chip)) {
    ret = ktd202x_chip_enable(led.chip);
    if (ret)
    return ret;
    }
    ret = regmap_read(led.chip.regmap, KTD202X_REG_CHANNEL_CTRL, &state);
    if (ret)
    return ret;
//
// In multicolor case, assume blink mode if PWM is set for at least one
// channel because another channel cannot be in state ON at the same time
//
    for (i = 0; i < num_channels; i++) {
    int channel_state;
    channel = subleds[i].channel;
    channel_state = (state >> 2 * channel) & KTD202X_CHANNEL_CTRL_MASK(0);
    if (channel_state == KTD202X_CHANNEL_CTRL_OFF)
    continue;
    mode_blink = channel_state == KTD202X_CHANNEL_CTRL_PWM1(0);
    break;
    }
    for (i = 0; i < num_channels; i++) {
    enum led_brightness brightness;
    int mode;
    brightness = subleds[i].brightness;
    channel = subleds[i].channel;
    if (brightness) {
// Register expects brightness between 0 and MAX_BRIGHTNESS - 1
    ret = regmap_write(led.chip.regmap, KTD202X_REG_LED_IOUT(channel),
    brightness - 1);
    if (ret)
    return ret;
    if (mode_blink)
    mode = KTD202X_CHANNEL_CTRL_PWM1(channel);
    else
    mode = KTD202X_CHANNEL_CTRL_ON(channel);
    } else {
    mode = KTD202X_CHANNEL_CTRL_OFF;
    }
    ret = regmap_update_bits(led.chip.regmap, KTD202X_REG_CHANNEL_CTRL,
    KTD202X_CHANNEL_CTRL_MASK(channel), mode);
    if (ret)
    return ret;
    }
    if (!ktd202x_chip_in_use(led.chip))
    return ktd202x_chip_disable(led.chip);
    return 0;
    }
    static int ktd202x_brightness_single_set(struct led_classdev *cdev,
    enum led_brightness value)
    {
    struct ktd202x_led *led = container_of(cdev, struct ktd202x_led, cdev);
    struct mc_subled info;
    int ret;
    cdev.brightness = value;
    mutex_lock(&led.chip.mutex);
    info.brightness = value;
    info.channel = led.index;
    ret = ktd202x_brightness_set(led, &info, 1);
    mutex_unlock(&led.chip.mutex);
    return ret;
    }
    static int ktd202x_brightness_mc_set(struct led_classdev *cdev,
    enum led_brightness value)
    {
    struct led_classdev_mc *mc = lcdev_to_mccdev(cdev);
    struct ktd202x_led *led = container_of(mc, struct ktd202x_led, mcdev);
    int ret;
    cdev.brightness = value;
    mutex_lock(&led.chip.mutex);
    led_mc_calc_color_components(mc, value);
    ret = ktd202x_brightness_set(led, mc.subled_info, mc.num_colors);
    mutex_unlock(&led.chip.mutex);
    return ret;
    }
    static int ktd202x_blink_set(struct ktd202x_led *led, unsigned long *delay_on,
    unsigned long *delay_off, struct mc_subled *subleds,
    unsigned int num_channels)
    {
    unsigned long delay_total_ms;
    int ret, num_steps, on;
    let mut ctrl_mask: u8 = 0;
    let mut ctrl_pwm1: u8 = 0;
    let mut ctrl_on: u8 = 0;
    int i;
    mutex_lock(&led.chip.mutex);
    for (i = 0; i < num_channels; i++) {
    let mut channel: c_int = subleds[i].channel;
    ctrl_mask |= KTD202X_CHANNEL_CTRL_MASK(channel);
    ctrl_on |= KTD202X_CHANNEL_CTRL_ON(channel);
    ctrl_pwm1 |= KTD202X_CHANNEL_CTRL_PWM1(channel);
    }
// Never off - brightness is already set, disable blinking
    if (!*delay_off) {
    ret = regmap_update_bits(led.chip.regmap, KTD202X_REG_CHANNEL_CTRL,
    ctrl_mask, ctrl_on);
    goto out;
    }
// Convert into values the HW will understand.
// Integer representation of time of flash period
    num_steps = (*delay_on + *delay_off - KTD202X_FLASH_PERIOD_MIN_MS) /
    KTD202X_FLASH_PERIOD_STEP_MS;
    num_steps = clamp(num_steps, 0, KTD202X_FLASH_PERIOD_MAX_STEPS);
// Integer representation of percentage of LED ON time
    on = (*delay_on * KTD202X_FLASH_ON_MAX) / (*delay_on + *delay_off);
// Actually used delay_{on,off} values
    delay_total_ms = num_steps * KTD202X_FLASH_PERIOD_STEP_MS + KTD202X_FLASH_PERIOD_MIN_MS;
// delay_on = (delay_total_ms * on) / KTD202X_FLASH_ON_MAX;
// delay_off = delay_total_ms - *delay_on;
// Set timings
    ret = regmap_write(led.chip.regmap, KTD202X_REG_FLASH_PERIOD, num_steps);
    if (ret)
    goto out;
    ret = regmap_write(led.chip.regmap, KTD202X_REG_PWM1_TIMER, on);
    if (ret)
    goto out;
    ret = regmap_update_bits(led.chip.regmap, KTD202X_REG_CHANNEL_CTRL,
    ctrl_mask, ctrl_pwm1);
    out:
    mutex_unlock(&led.chip.mutex);
    return ret;
    }
    static int ktd202x_blink_single_set(struct led_classdev *cdev,
    unsigned long *delay_on,
    unsigned long *delay_off)
    {
    struct ktd202x_led *led = container_of(cdev, struct ktd202x_led, cdev);
    struct mc_subled info;
    int ret;
    if (!cdev.brightness) {
    ret = ktd202x_brightness_single_set(cdev, KTD202X_MAX_BRIGHTNESS);
    if (ret)
    return ret;
    }
// If no blink specified, default to 1 Hz.
    if (!*delay_off && !*delay_on) {
// delay_off = 500;
// delay_on = 500;
    }
// Never on - just set to off
    if (!*delay_on)
    return ktd202x_brightness_single_set(cdev, LED_OFF);
    info.channel = led.index;
    return ktd202x_blink_set(led, delay_on, delay_off, &info, 1);
    }
    static int ktd202x_blink_mc_set(struct led_classdev *cdev,
    unsigned long *delay_on,
    unsigned long *delay_off)
    {
    struct led_classdev_mc *mc = lcdev_to_mccdev(cdev);
    struct ktd202x_led *led = container_of(mc, struct ktd202x_led, mcdev);
    int ret;
    if (!cdev.brightness) {
    ret = ktd202x_brightness_mc_set(cdev, KTD202X_MAX_BRIGHTNESS);
    if (ret)
    return ret;
    }
// If no blink specified, default to 1 Hz.
    if (!*delay_off && !*delay_on) {
// delay_off = 500;
// delay_on = 500;
    }
// Never on - just set to off
    if (!*delay_on)
    return ktd202x_brightness_mc_set(cdev, LED_OFF);
    return ktd202x_blink_set(led, delay_on, delay_off, mc.subled_info,
    mc.num_colors);
    }
    static int ktd202x_setup_led_rgb(struct ktd202x *chip, struct fwnode_handle *fwnode,
    struct ktd202x_led *led, struct led_init_data *init_data)
    {
    struct fwnode_handle *child;
    struct led_classdev *cdev;
    struct mc_subled *info;
    int num_channels;
    let mut i: c_int = 0;
    num_channels = 0;
    fwnode_for_each_child_node(fwnode, child)
    num_channels++;
    if (!num_channels || num_channels > chip.num_leds)
    return -EINVAL;
    info = devm_kcalloc(chip.dev, num_channels, sizeof(*info), GFP_KERNEL);
    if (!info)
    return -ENOMEM;
    fwnode_for_each_child_node(fwnode, child) {
    u32 mono_color;
    u32 reg;
    int ret;
    ret = fwnode_property_read_u32(child, "reg", &reg);
    if (ret != 0 || reg >= chip.num_leds) {
    dev_err(chip.dev, "invalid 'reg' of %pfw\n", child);
    fwnode_handle_put(child);
    return ret;
    }
    ret = fwnode_property_read_u32(child, "color", &mono_color);
    if (ret < 0 && ret != -EINVAL) {
    dev_err(chip.dev, "failed to parse 'color' of %pfw\n", child);
    fwnode_handle_put(child);
    return ret;
    }
    info[i].color_index = mono_color;
    info[i].channel = reg;
    info[i].intensity = KTD202X_MAX_BRIGHTNESS;
    i++;
    }
    led.mcdev.subled_info = info;
    led.mcdev.num_colors = num_channels;
    cdev = &led.mcdev.led_cdev;
    cdev.brightness_set_blocking = ktd202x_brightness_mc_set;
    cdev.blink_set = ktd202x_blink_mc_set;
    return devm_led_classdev_multicolor_register_ext(chip.dev, &led.mcdev, init_data);
    }
    static int ktd202x_setup_led_single(struct ktd202x *chip, struct fwnode_handle *fwnode,
    struct ktd202x_led *led, struct led_init_data *init_data)
    {
    struct led_classdev *cdev;
    u32 reg;
    int ret;
    ret = fwnode_property_read_u32(fwnode, "reg", &reg);
    if (ret != 0 || reg >= chip.num_leds) {
    dev_err(chip.dev, "invalid 'reg' of %pfw\n", fwnode);
    return -EINVAL;
    }
    led.index = reg;
    cdev = &led.cdev;
    cdev.brightness_set_blocking = ktd202x_brightness_single_set;
    cdev.blink_set = ktd202x_blink_single_set;
    return devm_led_classdev_register_ext(chip.dev, &led.cdev, init_data);
    }
#[no_mangle]
unsafe extern "C" fn ktd202x_add_led(chip: *mut ktd202x, fwnode: *mut fwnode_handle, index: c_uint) -> c_int {
    static int ktd202x_add_led(struct ktd202x *chip, struct fwnode_handle *fwnode, unsigned int index)
    {
    struct ktd202x_led *led = &chip.leds[index];
    let mut init_data: led_init_data = {};
    struct led_classdev *cdev;
    u32 color;
    int ret;
// Color property is optional in single color case
    ret = fwnode_property_read_u32(fwnode, "color", &color);
    if (ret < 0 && ret != -EINVAL) {
    dev_err(chip.dev, "failed to parse 'color' of %pfw\n", fwnode);
    return ret;
    }
    led.chip = chip;
    init_data.fwnode = fwnode;
    if (color == LED_COLOR_ID_RGB) {
    cdev = &led.mcdev.led_cdev;
    ret = ktd202x_setup_led_rgb(chip, fwnode, led, &init_data);
    } else {
    cdev = &led.cdev;
    ret = ktd202x_setup_led_single(chip, fwnode, led, &init_data);
    }
    if (ret) {
    dev_err(chip.dev, "unable to register %s\n", cdev.name);
    return ret;
    }
    cdev.max_brightness = KTD202X_MAX_BRIGHTNESS;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ktd202x_probe_fw(chip: *mut ktd202x) -> c_int {
    static int ktd202x_probe_fw(struct ktd202x *chip)
    {
    struct device *dev = chip.dev;
    int count;
    let mut i: c_int = 0;
    count = device_get_child_node_count(dev);
    if (!count || count > chip.num_leds)
    return -EINVAL;
    regmap_write(chip.regmap, KTD202X_REG_RESET_CONTROL, KTD202X_RSTR_RESET);
// Allow the device to execute the complete reset
    usleep_range(200, 300);
    device_for_each_child_node_scoped(dev, child) {
    let mut ret: c_int = ktd202x_add_led(chip, child, i);
    if (ret)
    return ret;
    i++;
    }
    return 0;
    }
    static const struct regmap_config ktd202x_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = 0x09,
    .cache_type = REGCACHE_FLAT,
    .reg_defaults = ktd202x_reg_defaults,
    .num_reg_defaults = ARRAY_SIZE(ktd202x_reg_defaults),
    };
#[no_mangle]
unsafe extern "C" fn ktd202x_probe(client: *mut i2c_client) -> c_int {
    static int ktd202x_probe(struct i2c_client *client)
    {
    struct device *dev = &client.dev;
    struct ktd202x *chip;
    int count;
    int ret;
    count = device_get_child_node_count(dev);
    if (!count || count > KTD202X_MAX_LEDS)
    return dev_err_probe(dev, -EINVAL, "Incorrect number of leds (%d)", count);
    chip = devm_kzalloc(dev, struct_size(chip, leds, count), GFP_KERNEL);
    if (!chip)
    return -ENOMEM;
    chip.dev = dev;
    i2c_set_clientdata(client, chip);
    chip.regmap = devm_regmap_init_i2c(client, &ktd202x_regmap_config);
    if (IS_ERR(chip.regmap)) {
    ret = dev_err_probe(dev, PTR_ERR(chip.regmap),
    "Failed to allocate register map.\n");
    return ret;
    }
    ret = devm_mutex_init(dev, &chip.mutex);
    if (ret)
    return ret;
    chip.num_leds = (unsigned long)i2c_get_match_data(client);
    chip.regulators[0].supply = "vin";
    chip.regulators[1].supply = "vio";
    ret = devm_regulator_bulk_get(dev, ARRAY_SIZE(chip.regulators), chip.regulators);
    if (ret < 0) {
    dev_err_probe(dev, ret, "Failed to request regulators.\n");
    return ret;
    }
    ret = regulator_bulk_enable(ARRAY_SIZE(chip.regulators), chip.regulators);
    if (ret) {
    dev_err_probe(dev, ret, "Failed to enable regulators.\n");
    return ret;
    }
    ret = ktd202x_probe_fw(chip);
    if (ret < 0) {
    regulator_bulk_disable(ARRAY_SIZE(chip.regulators), chip.regulators);
    return ret;
    }
    ret = regulator_bulk_disable(ARRAY_SIZE(chip.regulators), chip.regulators);
    if (ret) {
    dev_err_probe(dev, ret, "Failed to disable regulators.\n");
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ktd202x_remove(client: *mut i2c_client) {
    static void ktd202x_remove(struct i2c_client *client)
    {
    struct ktd202x *chip = i2c_get_clientdata(client);
    ktd202x_chip_disable(chip);
    }
#[no_mangle]
unsafe extern "C" fn ktd202x_shutdown(client: *mut i2c_client) {
    static void ktd202x_shutdown(struct i2c_client *client)
    {
    struct ktd202x *chip = i2c_get_clientdata(client);
// Reset registers to make sure all LEDs are off before shutdown
    regmap_write(chip.regmap, KTD202X_REG_RESET_CONTROL, KTD202X_RSTR_RESET);
    }
    static const struct i2c_device_id ktd202x_id[] = {
    { .name = "ktd2026", .driver_data = KTD2026_NUM_LEDS },
    { .name = "ktd2027", .driver_data = KTD2027_NUM_LEDS },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, ktd202x_id);
    static const struct of_device_id ktd202x_match_table[] = {
    { .compatible = "kinetic,ktd2026", .data = (void *)KTD2026_NUM_LEDS },
    { .compatible = "kinetic,ktd2027", .data = (void *)KTD2027_NUM_LEDS },
    {}
    };
    MODULE_DEVICE_TABLE(of, ktd202x_match_table);
    static struct i2c_driver ktd202x_driver = {
    .driver = {
    .name = "leds-ktd202x",
    .of_match_table = ktd202x_match_table,
    },
    .probe = ktd202x_probe,
    .remove = ktd202x_remove,
    .shutdown = ktd202x_shutdown,
    .id_table = ktd202x_id,
    };
    module_i2c_driver(ktd202x_driver);
    MODULE_AUTHOR("André Apitzsch <git@apitzsch.eu>");
    MODULE_DESCRIPTION("Kinetic KTD2026/7 LED driver");
    MODULE_LICENSE("GPL");
