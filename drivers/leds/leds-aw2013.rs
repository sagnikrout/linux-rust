//! Automatically rewritten from C to Rust
//! Source: drivers/leds/leds-aw2013.c
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
// Driver for Awinic AW2013 3-channel LED driver

pub const AW2013_MAX_LEDS: c_int = 3;
// Reset and ID register
pub const AW2013_RSTR: c_uint = 0x00;
pub const AW2013_RSTR_RESET: c_uint = 0x55;
pub const AW2013_RSTR_CHIP_ID: c_uint = 0x33;
// Global control register
pub const AW2013_GCR: c_uint = 0x01;

// LED channel enable register
pub const AW2013_LCTR: c_uint = 0x30;

// LED channel control registers

// LED channel PWM registers

// LED channel timing registers

pub const AW2013_REG_MAX: c_uint = 0x77;

    struct aw2013;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aw2013_led {
    pub chip: *mut aw2013,
    pub cdev: led_classdev,
    pub num: u32,
    pub imax: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aw2013 {
    pub /: *mut *mut mutex mutex; / held when writing to registers,
    pub regulators: [regulator_bulk_data; 2],
    pub client: *mut i2c_client,
    pub leds: [aw2013_led; AW2013_MAX_LEDS],
    pub regmap: *mut regmap,
    pub num_leds: c_int,
    pub enabled: bool,
}

#[no_mangle]
unsafe extern "C" fn aw2013_chip_init(chip: *mut aw2013) -> c_int {
    static int aw2013_chip_init(struct aw2013 *chip)
    {
    int i, ret;
    ret = regmap_write(chip.regmap, AW2013_GCR, AW2013_GCR_ENABLE);
    if (ret) {
    dev_err(&chip.client.dev, "Failed to enable the chip: %d\n",
    ret);
    return ret;
    }
    for (i = 0; i < chip.num_leds; i++) {
    ret = regmap_update_bits(chip.regmap,
    AW2013_LCFG(chip.leds[i].num),
    AW2013_LCFG_IMAX_MASK,
    chip.leds[i].imax);
    if (ret) {
    dev_err(&chip.client.dev,
    "Failed to set maximum current for led %d: %d\n",
    chip.leds[i].num, ret);
    return ret;
    }
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn aw2013_chip_disable(chip: *mut aw2013) {
    static void aw2013_chip_disable(struct aw2013 *chip)
    {
    int ret;
    if (!chip.enabled)
    return;
    regmap_write(chip.regmap, AW2013_GCR, 0);
    ret = regulator_bulk_disable(ARRAY_SIZE(chip.regulators),
    chip.regulators);
    if (ret) {
    dev_err(&chip.client.dev,
    "Failed to disable regulators: %d\n", ret);
    return;
    }
    chip.enabled = false;
    }
#[no_mangle]
unsafe extern "C" fn aw2013_chip_enable(chip: *mut aw2013) -> c_int {
    static int aw2013_chip_enable(struct aw2013 *chip)
    {
    int ret;
    if (chip.enabled)
    return 0;
    ret = regulator_bulk_enable(ARRAY_SIZE(chip.regulators),
    chip.regulators);
    if (ret) {
    dev_err(&chip.client.dev,
    "Failed to enable regulators: %d\n", ret);
    return ret;
    }
    chip.enabled = true;
    ret = aw2013_chip_init(chip);
    if (ret)
    aw2013_chip_disable(chip);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn aw2013_chip_in_use(chip: *mut aw2013) -> bool {
    static bool aw2013_chip_in_use(struct aw2013 *chip)
    {
    int i;
    for (i = 0; i < chip.num_leds; i++)
    if (chip.leds[i].cdev.brightness)
    return true;
    return false;
    }
    static int aw2013_brightness_set(struct led_classdev *cdev,
    enum led_brightness brightness)
    {
    struct aw2013_led *led = container_of(cdev, struct aw2013_led, cdev);
    int ret, num;
    mutex_lock(&led.chip.mutex);
    if (aw2013_chip_in_use(led.chip)) {
    ret = aw2013_chip_enable(led.chip);
    if (ret)
    goto error;
    }
    num = led.num;
    ret = regmap_write(led.chip.regmap, AW2013_REG_PWM(num), brightness);
    if (ret)
    goto error;
    if (brightness) {
    ret = regmap_update_bits(led.chip.regmap, AW2013_LCTR,
    AW2013_LCTR_LE(num), 0xFF);
    } else {
    ret = regmap_update_bits(led.chip.regmap, AW2013_LCTR,
    AW2013_LCTR_LE(num), 0);
    if (ret)
    goto error;
    ret = regmap_update_bits(led.chip.regmap, AW2013_LCFG(num),
    AW2013_LCFG_MD, 0);
    }
    if (ret)
    goto error;
    if (!aw2013_chip_in_use(led.chip))
    aw2013_chip_disable(led.chip);
    error:
    mutex_unlock(&led.chip.mutex);
    return ret;
    }
    static int aw2013_blink_set(struct led_classdev *cdev,
    unsigned long *delay_on, unsigned long *delay_off)
    {
    struct aw2013_led *led = container_of(cdev, struct aw2013_led, cdev);
    int ret, num = led.num;
    let mut off: c_ulong = 0, on = 0;
// If no blink specified, default to 1 Hz.
    if (!*delay_off && !*delay_on) {
// delay_off = 500;
// delay_on = 500;
    }
    if (!led.cdev.brightness) {
    led.cdev.brightness = LED_FULL;
    ret = aw2013_brightness_set(&led.cdev, led.cdev.brightness);
    if (ret)
    return ret;
    }
// Never on - just set to off
    if (!*delay_on) {
    led.cdev.brightness = LED_OFF;
    return aw2013_brightness_set(&led.cdev, LED_OFF);
    }
    mutex_lock(&led.chip.mutex);
// Never off - brightness is already set, disable blinking
    if (!*delay_off) {
    ret = regmap_update_bits(led.chip.regmap, AW2013_LCFG(num),
    AW2013_LCFG_MD, 0);
    goto out;
    }
// Convert into values the HW will understand.
    off = min(5, ilog2((*delay_off - 1) / AW2013_TIME_STEP) + 1);
    on = min(7, ilog2((*delay_on - 1) / AW2013_TIME_STEP) + 1);
// delay_off = BIT(off) * AW2013_TIME_STEP;
// delay_on = BIT(on) * AW2013_TIME_STEP;
// Set timings
    ret = regmap_write(led.chip.regmap,
    AW2013_LEDT0(num), AW2013_LEDT0_T2(on));
    if (ret)
    goto out;
    ret = regmap_write(led.chip.regmap,
    AW2013_LEDT1(num), AW2013_LEDT1_T4(off));
    if (ret)
    goto out;
// Finally, enable the LED
    ret = regmap_update_bits(led.chip.regmap, AW2013_LCFG(num),
    AW2013_LCFG_MD, 0xFF);
    if (ret)
    goto out;
    ret = regmap_update_bits(led.chip.regmap, AW2013_LCTR,
    AW2013_LCTR_LE(num), 0xFF);
    out:
    mutex_unlock(&led.chip.mutex);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn aw2013_probe_dt(chip: *mut aw2013) -> c_int {
    static int aw2013_probe_dt(struct aw2013 *chip)
    {
    struct device_node *np = dev_of_node(&chip.client.dev);
    int count, ret = 0, i = 0;
    struct aw2013_led *led;
    count = of_get_available_child_count(np);
    if (!count || count > AW2013_MAX_LEDS)
    return -EINVAL;
    regmap_write(chip.regmap, AW2013_RSTR, AW2013_RSTR_RESET);
    for_each_available_child_of_node_scoped(np, child) {
    let mut init_data: led_init_data = {};
    u32 source;
    u32 imax;
    ret = of_property_read_u32(child, "reg", &source);
    if (ret != 0 || source >= AW2013_MAX_LEDS) {
    dev_err(&chip.client.dev,
    "Couldn't read LED address: %d\n", ret);
    count--;
    continue;
    }
    led = &chip.leds[i];
    led.num = source;
    led.chip = chip;
    init_data.fwnode = of_fwnode_handle(child);
    if (!of_property_read_u32(child, "led-max-microamp", &imax)) {
    led.imax = min_t(u32, imax / 5000, 3);
    } else {
    led.imax = 1; // 5mA
    dev_info(&chip.client.dev,
    "DT property led-max-microamp is missing\n");
    }
    led.cdev.brightness_set_blocking = aw2013_brightness_set;
    led.cdev.blink_set = aw2013_blink_set;
    ret = devm_led_classdev_register_ext(&chip.client.dev,
    &led.cdev, &init_data);
    if (ret < 0)
    return ret;
    i++;
    }
    if (!count)
    return -EINVAL;
    chip.num_leds = i;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn aw2013_chip_disable_action(data: *mut c_void) {
    static void aw2013_chip_disable_action(void *data)
    {
    aw2013_chip_disable(data);
    }
    static const struct regmap_config aw2013_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = AW2013_REG_MAX,
    };
#[no_mangle]
unsafe extern "C" fn aw2013_probe(client: *mut i2c_client) -> c_int {
    static int aw2013_probe(struct i2c_client *client)
    {
    struct aw2013 *chip;
    int ret;
    unsigned int chipid;
    chip = devm_kzalloc(&client.dev, sizeof(*chip), GFP_KERNEL);
    if (!chip)
    return -ENOMEM;
    ret = devm_mutex_init(&client.dev, &chip.mutex);
    if (ret)
    return ret;
    mutex_lock(&chip.mutex);
    chip.client = client;
    i2c_set_clientdata(client, chip);
    chip.regmap = devm_regmap_init_i2c(client, &aw2013_regmap_config);
    if (IS_ERR(chip.regmap)) {
    ret = PTR_ERR(chip.regmap);
    dev_err(&client.dev, "Failed to allocate register map: %d\n",
    ret);
    goto error;
    }
    chip.regulators[0].supply = "vcc";
    chip.regulators[1].supply = "vio";
    ret = devm_regulator_bulk_get(&client.dev,
    ARRAY_SIZE(chip.regulators),
    chip.regulators);
    if (ret < 0) {
    if (ret != -EPROBE_DEFER)
    dev_err(&client.dev,
    "Failed to request regulators: %d\n", ret);
    goto error;
    }
    ret = regulator_bulk_enable(ARRAY_SIZE(chip.regulators),
    chip.regulators);
    if (ret) {
    dev_err(&client.dev,
    "Failed to enable regulators: %d\n", ret);
    goto error;
    }
    ret = regmap_read(chip.regmap, AW2013_RSTR, &chipid);
    if (ret) {
    dev_err(&client.dev, "Failed to read chip ID: %d\n",
    ret);
    goto error_reg;
    }
    if (chipid != AW2013_RSTR_CHIP_ID) {
    dev_err(&client.dev, "Chip reported wrong ID: %x\n",
    chipid);
    ret = -ENODEV;
    goto error_reg;
    }
    ret = devm_add_action(&client.dev, aw2013_chip_disable_action, chip);
    if (ret)
    goto error_reg;
    ret = aw2013_probe_dt(chip);
    if (ret < 0)
    goto error_reg;
    ret = regulator_bulk_disable(ARRAY_SIZE(chip.regulators),
    chip.regulators);
    if (ret) {
    dev_err(&client.dev,
    "Failed to disable regulators: %d\n", ret);
    goto error;
    }
    mutex_unlock(&chip.mutex);
    return 0;
    error_reg:
    regulator_bulk_disable(ARRAY_SIZE(chip.regulators),
    chip.regulators);
    error:
    mutex_unlock(&chip.mutex);
    return ret;
    }
    static const struct of_device_id aw2013_match_table[] = {
    { .compatible = "awinic,aw2013", },
    { /* sentinel */ },
    };
    MODULE_DEVICE_TABLE(of, aw2013_match_table);
    static struct i2c_driver aw2013_driver = {
    .driver = {
    .name = "leds-aw2013",
    .of_match_table = aw2013_match_table,
    },
    .probe = aw2013_probe,
    };
    module_i2c_driver(aw2013_driver);
    MODULE_AUTHOR("Nikita Travkin <nikitos.tr@gmail.com>");
    MODULE_DESCRIPTION("AW2013 LED driver");
    MODULE_LICENSE("GPL v2");
