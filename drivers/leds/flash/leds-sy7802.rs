//! Automatically rewritten from C to Rust
//! Source: drivers/leds/flash/leds-sy7802.c
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
// Silergy SY7802 flash LED driver with an I2C interface
//
// Copyright 2024 André Apitzsch <git@apitzsch.eu>
//

pub const SY7802_MAX_LEDS: c_int = 2;
pub const SY7802_LED_JOINT: c_int = 2;
pub const SY7802_REG_ENABLE: c_uint = 0x10;
pub const SY7802_REG_TORCH_BRIGHTNESS: c_uint = 0xa0;
pub const SY7802_REG_FLASH_BRIGHTNESS: c_uint = 0xb0;
pub const SY7802_REG_FLASH_DURATION: c_uint = 0xc0;
pub const SY7802_REG_FLAGS: c_uint = 0xd0;
pub const SY7802_REG_CONFIG_1: c_uint = 0xe0;
pub const SY7802_REG_CONFIG_2: c_uint = 0xf0;
pub const SY7802_REG_VIN_MONITOR: c_uint = 0x80;
pub const SY7802_REG_LAST_FLASH: c_uint = 0x81;
pub const SY7802_REG_VLED_MONITOR: c_uint = 0x30;
pub const SY7802_REG_ADC_DELAY: c_uint = 0x31;
pub const SY7802_REG_DEV_ID: c_uint = 0xff;
pub const SY7802_MODE_OFF: c_int = 0;
pub const SY7802_MODE_TORCH: c_int = 2;
pub const SY7802_MODE_FLASH: c_int = 3;

pub const SY7802_LEDS_SHIFT: c_int = 3;

pub const SY7802_TORCH_CURRENT_SHIFT: c_int = 3;

    (GENMASK(2, 0) << (SY7802_TORCH_CURRENT_SHIFT * (_id)))

    (SY7802_TORCH_CURRENT_MASK(0) | SY7802_TORCH_CURRENT_MASK(1))
pub const SY7802_FLASH_CURRENT_SHIFT: c_int = 4;

    (GENMASK(3, 0) << (SY7802_FLASH_CURRENT_SHIFT * (_id)))

    (SY7802_FLASH_CURRENT_MASK(0) | SY7802_FLASH_CURRENT_MASK(1))

pub const SY7802_TORCH_BRIGHTNESS_MAX: c_int = 8;
pub const SY7802_FLASH_BRIGHTNESS_DEFAULT: c_int = 14;
pub const SY7802_FLASH_BRIGHTNESS_MIN: c_int = 0;
pub const SY7802_FLASH_BRIGHTNESS_MAX: c_int = 15;
pub const SY7802_FLASH_BRIGHTNESS_STEP: c_int = 1;

pub const SY7802_CHIP_ID: c_uint = 0x51;
    static const struct reg_default sy7802_regmap_defs[] = {
    { SY7802_REG_ENABLE, SY7802_LEDS_MASK_ALL },
    { SY7802_REG_TORCH_BRIGHTNESS, 0x92 },
    { SY7802_REG_FLASH_BRIGHTNESS, SY7802_FLASH_BRIGHTNESS_DEFAULT |
    SY7802_FLASH_BRIGHTNESS_DEFAULT << SY7802_FLASH_CURRENT_SHIFT },
    { SY7802_REG_FLASH_DURATION, 0x6f },
    { SY7802_REG_FLAGS, 0x0 },
    { SY7802_REG_CONFIG_1, 0x68 },
    { SY7802_REG_CONFIG_2, 0xf0 },
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sy7802_led {
    pub flash: led_classdev_flash,
    pub chip: *mut sy7802,
    pub led_id: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sy7802 {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub mutex: mutex,
    pub enable_gpio: *mut gpio_desc,
    pub vin_regulator: *mut regulator,
    pub fled_strobe_used: c_uint,
    pub fled_torch_used: c_uint,
    pub leds_active: c_uint,
    pub num_leds: c_int,
    pub __counted_by(num_leds): sy7802_led leds[],
}

#[no_mangle]
unsafe extern "C" fn sy7802_torch_brightness_set(lcdev: *mut led_classdev, brightness: enum led_brightness) -> c_int {
    static int sy7802_torch_brightness_set(struct led_classdev *lcdev, enum led_brightness brightness)
    {
    struct sy7802_led *led = container_of(lcdev, struct sy7802_led, flash.led_cdev);
    struct sy7802 *chip = led.chip;
    u32 fled_torch_used_tmp;
    u32 led_enable_mask;
    u32 enable_mask;
    u32 torch_mask;
    u32 val;
    int ret;
    mutex_lock(&chip.mutex);
    if (chip.fled_strobe_used) {
    dev_warn(chip.dev, "Cannot set torch brightness whilst strobe is enabled\n");
    ret = -EBUSY;
    goto unlock;
    }
    if (brightness)
    fled_torch_used_tmp = chip.fled_torch_used | BIT(led.led_id);
    else
    fled_torch_used_tmp = chip.fled_torch_used & ~BIT(led.led_id);
    led_enable_mask = led.led_id == SY7802_LED_JOINT ?
    SY7802_LEDS_MASK_ALL :
    SY7802_LEDS_MASK(led.led_id);
    val = brightness ? led_enable_mask : SY7802_MODE_OFF;
    if (fled_torch_used_tmp)
    val |= SY7802_MODE_TORCH;
// Disable torch to apply brightness
    ret = regmap_update_bits(chip.regmap, SY7802_REG_ENABLE, SY7802_MODE_MASK,
    SY7802_MODE_OFF);
    if (ret)
    goto unlock;
    torch_mask = led.led_id == SY7802_LED_JOINT ?
    SY7802_TORCH_CURRENT_MASK_ALL :
    SY7802_TORCH_CURRENT_MASK(led.led_id);
// Register expects brightness between 0 and MAX_BRIGHTNESS - 1
    if (brightness)
    brightness -= 1;
    brightness |= (brightness << SY7802_TORCH_CURRENT_SHIFT);
    ret = regmap_update_bits(chip.regmap, SY7802_REG_TORCH_BRIGHTNESS, torch_mask, brightness);
    if (ret)
    goto unlock;
    enable_mask = SY7802_MODE_MASK | led_enable_mask;
    ret = regmap_update_bits(chip.regmap, SY7802_REG_ENABLE, enable_mask, val);
    if (ret)
    goto unlock;
    chip.fled_torch_used = fled_torch_used_tmp;
    unlock:
    mutex_unlock(&chip.mutex);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn sy7802_flash_brightness_set(fl_cdev: *mut led_classdev_flash, brightness: u32) -> c_int {
    static int sy7802_flash_brightness_set(struct led_classdev_flash *fl_cdev, u32 brightness)
    {
    struct sy7802_led *led = container_of(fl_cdev, struct sy7802_led, flash);
    struct led_flash_setting *s = &fl_cdev.brightness;
    let mut val: u32 = (brightness - s.min) / s.step;
    struct sy7802 *chip = led.chip;
    u32 flash_mask;
    int ret;
    val |= (val << SY7802_FLASH_CURRENT_SHIFT);
    flash_mask = led.led_id == SY7802_LED_JOINT ?
    SY7802_FLASH_CURRENT_MASK_ALL :
    SY7802_FLASH_CURRENT_MASK(led.led_id);
    mutex_lock(&chip.mutex);
    ret = regmap_update_bits(chip.regmap, SY7802_REG_FLASH_BRIGHTNESS, flash_mask, val);
    mutex_unlock(&chip.mutex);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn sy7802_strobe_set(fl_cdev: *mut led_classdev_flash, state: bool) -> c_int {
    static int sy7802_strobe_set(struct led_classdev_flash *fl_cdev, bool state)
    {
    struct sy7802_led *led = container_of(fl_cdev, struct sy7802_led, flash);
    struct sy7802 *chip = led.chip;
    u32 fled_strobe_used_tmp;
    u32 led_enable_mask;
    u32 enable_mask;
    u32 val;
    int ret;
    mutex_lock(&chip.mutex);
    if (chip.fled_torch_used) {
    dev_warn(chip.dev, "Cannot set strobe brightness whilst torch is enabled\n");
    ret = -EBUSY;
    goto unlock;
    }
    if (state)
    fled_strobe_used_tmp = chip.fled_strobe_used | BIT(led.led_id);
    else
    fled_strobe_used_tmp = chip.fled_strobe_used & ~BIT(led.led_id);
    led_enable_mask = led.led_id == SY7802_LED_JOINT ?
    SY7802_LEDS_MASK_ALL :
    SY7802_LEDS_MASK(led.led_id);
    val = state ? led_enable_mask : SY7802_MODE_OFF;
    if (fled_strobe_used_tmp)
    val |= SY7802_MODE_FLASH;
    enable_mask = SY7802_MODE_MASK | led_enable_mask;
    ret = regmap_update_bits(chip.regmap, SY7802_REG_ENABLE, enable_mask, val);
    if (ret)
    goto unlock;
    chip.fled_strobe_used = fled_strobe_used_tmp;
    unlock:
    mutex_unlock(&chip.mutex);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn sy7802_strobe_get(fl_cdev: *mut led_classdev_flash, state: *mut bool) -> c_int {
    static int sy7802_strobe_get(struct led_classdev_flash *fl_cdev, bool *state)
    {
    struct sy7802_led *led = container_of(fl_cdev, struct sy7802_led, flash);
    struct sy7802 *chip = led.chip;
    mutex_lock(&chip.mutex);
// state = !!(chip->fled_strobe_used & BIT(led->led_id));
    mutex_unlock(&chip.mutex);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sy7802_timeout_set(fl_cdev: *mut led_classdev_flash, timeout: u32) -> c_int {
    static int sy7802_timeout_set(struct led_classdev_flash *fl_cdev, u32 timeout)
    {
    struct sy7802_led *led = container_of(fl_cdev, struct sy7802_led, flash);
    struct led_flash_setting *s = &fl_cdev.timeout;
    let mut val: u32 = (timeout - s.min) / s.step;
    struct sy7802 *chip = led.chip;
    return regmap_write(chip.regmap, SY7802_REG_FLASH_DURATION, val);
    }
#[no_mangle]
unsafe extern "C" fn sy7802_fault_get(fl_cdev: *mut led_classdev_flash, fault: *mut u32) -> c_int {
    static int sy7802_fault_get(struct led_classdev_flash *fl_cdev, u32 *fault)
    {
    struct sy7802_led *led = container_of(fl_cdev, struct sy7802_led, flash);
    struct sy7802 *chip = led.chip;
    u32 val, led_faults = 0;
    int ret;
// NOTE: reading register clears fault status
    ret = regmap_read(chip.regmap, SY7802_REG_FLAGS, &val);
    if (ret)
    return ret;
    if (val & (SY7802_FLAG_FLASH_INPUT_VOLTAGE_LOW | SY7802_FLAG_INPUT_VOLTAGE_LOW))
    led_faults |= LED_FAULT_INPUT_VOLTAGE;
    if (val & SY7802_FLAG_THERMAL_SHUTDOWN)
    led_faults |= LED_FAULT_OVER_TEMPERATURE;
    if (val & SY7802_FLAG_TIMEOUT)
    led_faults |= LED_FAULT_TIMEOUT;
// fault = led_faults;
    return 0;
    }
    static const struct led_flash_ops sy7802_flash_ops = {
    .flash_brightness_set = sy7802_flash_brightness_set,
    .strobe_set = sy7802_strobe_set,
    .strobe_get = sy7802_strobe_get,
    .timeout_set = sy7802_timeout_set,
    .fault_get = sy7802_fault_get,
    };
#[no_mangle]
unsafe extern "C" fn sy7802_init_flash_brightness(fl_cdev: *mut led_classdev_flash) {
    static void sy7802_init_flash_brightness(struct led_classdev_flash *fl_cdev)
    {
    struct led_flash_setting *s;
// Init flash brightness setting
    s = &fl_cdev.brightness;
    s.min = SY7802_FLASH_BRIGHTNESS_MIN;
    s.max = SY7802_FLASH_BRIGHTNESS_MAX;
    s.step = SY7802_FLASH_BRIGHTNESS_STEP;
    s.val = SY7802_FLASH_BRIGHTNESS_DEFAULT;
    }
#[no_mangle]
unsafe extern "C" fn sy7802_init_flash_timeout(fl_cdev: *mut led_classdev_flash) {
    static void sy7802_init_flash_timeout(struct led_classdev_flash *fl_cdev)
    {
    struct led_flash_setting *s;
// Init flash timeout setting
    s = &fl_cdev.timeout;
    s.min = SY7802_TIMEOUT_MIN_US;
    s.max = SY7802_TIMEOUT_MAX_US;
    s.step = SY7802_TIMEOUT_STEPSIZE_US;
    s.val = SY7802_TIMEOUT_DEFAULT_US;
    }
    static int sy7802_led_register(struct device *dev, struct sy7802_led *led,
    struct device_node *np)
    {
    let mut init_data: led_init_data = {};
    int ret;
    init_data.fwnode = of_fwnode_handle(np);
    ret = devm_led_classdev_flash_register_ext(dev, &led.flash, &init_data);
    if (ret) {
    dev_err(dev, "Couldn't register flash %d\n", led.led_id);
    return ret;
    }
    return 0;
    }
    static int sy7802_init_flash_properties(struct device *dev, struct sy7802_led *led,
    struct device_node *np)
    {
    struct led_classdev_flash *flash = &led.flash;
    struct led_classdev *lcdev = &flash.led_cdev;
    u32 sources[SY7802_MAX_LEDS];
    int i, num, ret;
    num = of_property_count_u32_elems(np, "led-sources");
    if (num < 1) {
    dev_err(dev, "Not specified or wrong number of led-sources\n");
    return -EINVAL;
    }
    ret = of_property_read_u32_array(np, "led-sources", sources, num);
    if (ret)
    return ret;
    for (i = 0; i < num; i++) {
    if (sources[i] >= SY7802_MAX_LEDS)
    return -EINVAL;
    if (led.chip.leds_active & BIT(sources[i]))
    return -EINVAL;
    led.chip.leds_active |= BIT(sources[i]);
    }
// If both channels are specified in 'led-sources', joint flash output mode is used
    led.led_id = num == 2 ? SY7802_LED_JOINT : sources[0];
    lcdev.max_brightness = SY7802_TORCH_BRIGHTNESS_MAX;
    lcdev.brightness_set_blocking = sy7802_torch_brightness_set;
    lcdev.flags |= LED_DEV_CAP_FLASH;
    flash.ops = &sy7802_flash_ops;
    sy7802_init_flash_brightness(flash);
    sy7802_init_flash_timeout(flash);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sy7802_chip_check(chip: *mut sy7802) -> c_int {
    static int sy7802_chip_check(struct sy7802 *chip)
    {
    struct device *dev = chip.dev;
    u32 chipid;
    int ret;
    ret = regmap_read(chip.regmap, SY7802_REG_DEV_ID, &chipid);
    if (ret)
    return dev_err_probe(dev, ret, "Failed to read chip ID\n");
    if (chipid != SY7802_CHIP_ID)
    return dev_err_probe(dev, -ENODEV, "Unsupported chip detected: %x\n", chipid);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sy7802_enable(chip: *mut sy7802) {
    static void sy7802_enable(struct sy7802 *chip)
    {
    gpiod_set_value_cansleep(chip.enable_gpio, 1);
    usleep_range(200, 300);
    }
#[no_mangle]
unsafe extern "C" fn sy7802_disable(chip: *mut sy7802) {
    static void sy7802_disable(struct sy7802 *chip)
    {
    gpiod_set_value_cansleep(chip.enable_gpio, 0);
    }
#[no_mangle]
unsafe extern "C" fn sy7802_probe_dt(chip: *mut sy7802) -> c_int {
    static int sy7802_probe_dt(struct sy7802 *chip)
    {
    struct device_node *np = dev_of_node(chip.dev);
    int child_num;
    int ret;
    regmap_write(chip.regmap, SY7802_REG_ENABLE, SY7802_MODE_OFF);
    regmap_write(chip.regmap, SY7802_REG_TORCH_BRIGHTNESS, LED_OFF);
    child_num = 0;
    for_each_available_child_of_node_scoped(np, child) {
    struct sy7802_led *led = chip.leds + child_num;
    led.chip = chip;
    led.led_id = child_num;
    ret = sy7802_init_flash_properties(chip.dev, led, child);
    if (ret)
    return ret;
    ret = sy7802_led_register(chip.dev, led, child);
    if (ret)
    return ret;
    child_num++;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sy7802_chip_disable_action(data: *mut c_void) {
    static void sy7802_chip_disable_action(void *data)
    {
    struct sy7802 *chip = data;
    sy7802_disable(chip);
    }
#[no_mangle]
unsafe extern "C" fn sy7802_regulator_disable_action(data: *mut c_void) {
    static void sy7802_regulator_disable_action(void *data)
    {
    struct sy7802 *chip = data;
    regulator_disable(chip.vin_regulator);
    }
    static const struct regmap_config sy7802_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = 0xff,
    .cache_type = REGCACHE_MAPLE,
    .reg_defaults = sy7802_regmap_defs,
    .num_reg_defaults = ARRAY_SIZE(sy7802_regmap_defs),
    };
#[no_mangle]
unsafe extern "C" fn sy7802_probe(client: *mut i2c_client) -> c_int {
    static int sy7802_probe(struct i2c_client *client)
    {
    struct device *dev = &client.dev;
    struct sy7802 *chip;
    size_t count;
    int ret;
    count = device_get_child_node_count(dev);
    if (!count || count > SY7802_MAX_LEDS)
    return dev_err_probe(dev, -EINVAL, "Invalid amount of LED nodes %zu\n", count);
    chip = devm_kzalloc(dev, struct_size(chip, leds, count), GFP_KERNEL);
    if (!chip)
    return -ENOMEM;
    chip.num_leds = count;
    chip.dev = dev;
    i2c_set_clientdata(client, chip);
    chip.enable_gpio = devm_gpiod_get(dev, "enable", GPIOD_OUT_LOW);
    ret = PTR_ERR_OR_ZERO(chip.enable_gpio);
    if (ret)
    return dev_err_probe(dev, ret, "Failed to request enable gpio\n");
    chip.vin_regulator = devm_regulator_get(dev, "vin");
    ret = PTR_ERR_OR_ZERO(chip.vin_regulator);
    if (ret)
    return dev_err_probe(dev, ret, "Failed to request regulator\n");
    ret = regulator_enable(chip.vin_regulator);
    if (ret)
    return dev_err_probe(dev, ret, "Failed to enable regulator\n");
    ret = devm_add_action_or_reset(dev, sy7802_regulator_disable_action, chip);
    if (ret)
    return ret;
    ret = devm_mutex_init(dev, &chip.mutex);
    if (ret)
    return ret;
    mutex_lock(&chip.mutex);
    chip.regmap = devm_regmap_init_i2c(client, &sy7802_regmap_config);
    if (IS_ERR(chip.regmap)) {
    ret = PTR_ERR(chip.regmap);
    dev_err_probe(dev, ret, "Failed to allocate register map\n");
    goto error;
    }
    ret = sy7802_probe_dt(chip);
    if (ret < 0)
    goto error;
    sy7802_enable(chip);
    ret = devm_add_action_or_reset(dev, sy7802_chip_disable_action, chip);
    if (ret)
    goto error;
    ret = sy7802_chip_check(chip);
    error:
    mutex_unlock(&chip.mutex);
    return ret;
    }
    static const struct of_device_id __maybe_unused sy7802_leds_match[] = {
    { .compatible = "silergy,sy7802", },
    {}
    };
    MODULE_DEVICE_TABLE(of, sy7802_leds_match);
    static struct i2c_driver sy7802_driver = {
    .driver = {
    .name = "sy7802",
    .of_match_table = of_match_ptr(sy7802_leds_match),
    },
    .probe = sy7802_probe,
    };
    module_i2c_driver(sy7802_driver);
    MODULE_AUTHOR("André Apitzsch <git@apitzsch.eu>");
    MODULE_DESCRIPTION("Silergy SY7802 flash LED driver");
    MODULE_LICENSE("GPL");
