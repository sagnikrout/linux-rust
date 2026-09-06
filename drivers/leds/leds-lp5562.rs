//! Automatically rewritten from C to Rust
//! Source: drivers/leds/leds-lp5562.c
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
// LP5562 LED driver
//
// Copyright (C) 2013 Texas Instruments
//
// Author: Milo(Woogyom) Kim <milo.kim@ti.com>
//

pub const LP5562_MAX_LEDS: c_int = 4;
// ENABLE Register 00h
pub const LP5562_REG_ENABLE: c_uint = 0x00;
pub const LP5562_EXEC_ENG1_M: c_uint = 0x30;
pub const LP5562_EXEC_ENG2_M: c_uint = 0x0C;
pub const LP5562_EXEC_ENG3_M: c_uint = 0x03;
pub const LP5562_EXEC_M: c_uint = 0x3F;
pub const LP5562_MASTER_ENABLE: c_uint = 0x40	/* Chip master enable */;
pub const LP5562_LOGARITHMIC_PWM: c_uint = 0x80	/* Logarithmic PWM adjustment */;
pub const LP5562_EXEC_RUN: c_uint = 0x2A;

    (LP5562_MASTER_ENABLE | LP5562_LOGARITHMIC_PWM)

    (LP5562_ENABLE_DEFAULT | LP5562_EXEC_RUN)
// OPMODE Register 01h
pub const LP5562_REG_OP_MODE: c_uint = 0x01;
// BRIGHTNESS Registers
pub const LP5562_REG_R_PWM: c_uint = 0x04;
pub const LP5562_REG_G_PWM: c_uint = 0x03;
pub const LP5562_REG_B_PWM: c_uint = 0x02;
pub const LP5562_REG_W_PWM: c_uint = 0x0E;
// CURRENT Registers
pub const LP5562_REG_R_CURRENT: c_uint = 0x07;
pub const LP5562_REG_G_CURRENT: c_uint = 0x06;
pub const LP5562_REG_B_CURRENT: c_uint = 0x05;
pub const LP5562_REG_W_CURRENT: c_uint = 0x0F;
// CONFIG Register 08h
pub const LP5562_REG_CONFIG: c_uint = 0x08;
pub const LP5562_PWM_HF: c_uint = 0x40;
pub const LP5562_PWRSAVE_EN: c_uint = 0x20;
pub const LP5562_CLK_INT: c_uint = 0x01	/* Internal clock */;

// RESET Register 0Dh
pub const LP5562_REG_RESET: c_uint = 0x0D;
pub const LP5562_RESET: c_uint = 0xFF;
// PROGRAM ENGINE Registers
pub const LP5562_REG_PROG_MEM_ENG1: c_uint = 0x10;
pub const LP5562_REG_PROG_MEM_ENG2: c_uint = 0x30;
pub const LP5562_REG_PROG_MEM_ENG3: c_uint = 0x50;
// LEDMAP Register 70h
pub const LP5562_REG_ENG_SEL: c_uint = 0x70;
pub const LP5562_ENG_SEL_PWM: c_int = 0;
pub const LP5562_ENG_FOR_RGB_M: c_uint = 0x3F;
pub const LP5562_ENG_SEL_RGB: c_uint = 0x1B	/* R:ENG1, G:ENG2, B:ENG3 */;
pub const LP5562_ENG_FOR_W_M: c_uint = 0xC0;
pub const LP5562_ENG1_FOR_W: c_uint = 0x40	/* W:ENG1 */;
pub const LP5562_ENG2_FOR_W: c_uint = 0x80	/* W:ENG2 */;
pub const LP5562_ENG3_FOR_W: c_uint = 0xC0	/* W:ENG3 */;
// Program Commands
pub const LP5562_CMD_DISABLE: c_uint = 0x00;
pub const LP5562_CMD_LOAD: c_uint = 0x15;
pub const LP5562_CMD_RUN: c_uint = 0x2A;
pub const LP5562_CMD_DIRECT: c_uint = 0x3F;
pub const LP5562_PATTERN_OFF: c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn lp5562_wait_opmode_done() {
    static inline void lp5562_wait_opmode_done(void)
    {
// operation mode change needs to be longer than 153 us
    usleep_range(200, 300);
    }
#[no_mangle]
pub unsafe extern "C" fn lp5562_wait_enable_done() {
    static inline void lp5562_wait_enable_done(void)
    {
// it takes more 488 us to update ENABLE register
    usleep_range(500, 600);
    }
#[no_mangle]
unsafe extern "C" fn lp5562_set_led_current(led: *mut lp55xx_led, led_current: u8) {
    static void lp5562_set_led_current(struct lp55xx_led *led, u8 led_current)
    {
    static const u8 addr[] = {
    LP5562_REG_R_CURRENT,
    LP5562_REG_G_CURRENT,
    LP5562_REG_B_CURRENT,
    LP5562_REG_W_CURRENT,
    };
    led.led_current = led_current;
    lp55xx_write(led.chip, addr[led.chan_nr], led_current);
    }
#[no_mangle]
unsafe extern "C" fn lp5562_run_engine(chip: *mut lp55xx_chip, start: bool) {
    static void lp5562_run_engine(struct lp55xx_chip *chip, bool start)
    {
    int ret;
// stop engine
    if (!start) {
    lp55xx_write(chip, LP5562_REG_ENABLE, LP5562_ENABLE_DEFAULT);
    lp5562_wait_enable_done();
    lp55xx_stop_all_engine(chip);
    lp55xx_write(chip, LP5562_REG_ENG_SEL, LP5562_ENG_SEL_PWM);
    lp55xx_write(chip, LP5562_REG_OP_MODE, LP5562_CMD_DIRECT);
    lp5562_wait_opmode_done();
    return;
    }
    ret = lp55xx_run_engine_common(chip);
    if (!ret)
    lp5562_wait_enable_done();
    }
#[no_mangle]
unsafe extern "C" fn lp5562_post_init_device(chip: *mut lp55xx_chip) -> c_int {
    static int lp5562_post_init_device(struct lp55xx_chip *chip)
    {
    int ret;
    let mut cfg: u8 = LP5562_DEFAULT_CFG;
// Set all PWMs to direct control mode
    ret = lp55xx_write(chip, LP5562_REG_OP_MODE, LP5562_CMD_DIRECT);
    if (ret)
    return ret;
    lp5562_wait_opmode_done();
// Update configuration for the clock setting
    if (!lp55xx_is_extclk_used(chip))
    cfg |= LP5562_CLK_INT;
    ret = lp55xx_write(chip, LP5562_REG_CONFIG, cfg);
    if (ret)
    return ret;
// Initialize all channels PWM to zero -> leds off
    lp55xx_write(chip, LP5562_REG_R_PWM, 0);
    lp55xx_write(chip, LP5562_REG_G_PWM, 0);
    lp55xx_write(chip, LP5562_REG_B_PWM, 0);
    lp55xx_write(chip, LP5562_REG_W_PWM, 0);
// Set LED map as register PWM by default
    lp55xx_write(chip, LP5562_REG_ENG_SEL, LP5562_ENG_SEL_PWM);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lp5562_multicolor_brightness(led: *mut lp55xx_led) -> c_int {
    static int lp5562_multicolor_brightness(struct lp55xx_led *led)
    {
    struct lp55xx_chip *chip = led.chip;
    static const u8 addr[] = {
    LP5562_REG_R_PWM,
    LP5562_REG_G_PWM,
    LP5562_REG_B_PWM,
    LP5562_REG_W_PWM,
    };
    int ret;
    int i;
    guard(mutex)(&chip.lock);
    for (i = 0; i < led.mc_cdev.num_colors; i++) {
    ret = lp55xx_write(chip,
    addr[led.mc_cdev.subled_info[i].channel],
    led.mc_cdev.subled_info[i].brightness);
    if (ret)
    break;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn lp5562_led_brightness(led: *mut lp55xx_led) -> c_int {
    static int lp5562_led_brightness(struct lp55xx_led *led)
    {
    struct lp55xx_chip *chip = led.chip;
    static const u8 addr[] = {
    LP5562_REG_R_PWM,
    LP5562_REG_G_PWM,
    LP5562_REG_B_PWM,
    LP5562_REG_W_PWM,
    };
    int ret;
    guard(mutex)(&chip.lock);
    ret = lp55xx_write(chip, addr[led.chan_nr], led.brightness);
    return ret;
    }
    static void lp5562_write_program_memory(struct lp55xx_chip *chip,
    u8 base, const u8 *rgb, int size)
    {
    int i;
    if (!rgb || size <= 0)
    return;
    for (i = 0; i < size; i++)
    lp55xx_write(chip, base + i, *(rgb + i));
    lp55xx_write(chip, base + i, 0);
    lp55xx_write(chip, base + i + 1, 0);
    }
// check the size of program count
#[no_mangle]
pub unsafe extern "C" fn _is_pc_overflow(ptn: *mut lp55xx_predef_pattern) -> bool {
    static inline bool _is_pc_overflow(struct lp55xx_predef_pattern *ptn)
    {
    return ptn.size_r >= LP55xx_BYTES_PER_PAGE ||
    ptn.size_g >= LP55xx_BYTES_PER_PAGE ||
    ptn.size_b >= LP55xx_BYTES_PER_PAGE;
    }
#[no_mangle]
unsafe extern "C" fn lp5562_run_predef_led_pattern(chip: *mut lp55xx_chip, mode: c_int) -> c_int {
    static int lp5562_run_predef_led_pattern(struct lp55xx_chip *chip, int mode)
    {
    struct lp55xx_predef_pattern *ptn;
    int i;
    if (mode == LP5562_PATTERN_OFF) {
    lp5562_run_engine(chip, false);
    return 0;
    }
    ptn = chip.pdata.patterns + (mode - 1);
    if (!ptn || _is_pc_overflow(ptn)) {
    dev_err(&chip.cl.dev, "invalid pattern data\n");
    return -EINVAL;
    }
    lp55xx_stop_all_engine(chip);
// Set LED map as RGB
    lp55xx_write(chip, LP5562_REG_ENG_SEL, LP5562_ENG_SEL_RGB);
// Load engines
    for (i = LP55XX_ENGINE_1; i <= LP55XX_ENGINE_3; i++) {
    chip.engine_idx = i;
    lp55xx_load_engine(chip);
    }
// Clear program registers
    lp55xx_write(chip, LP5562_REG_PROG_MEM_ENG1, 0);
    lp55xx_write(chip, LP5562_REG_PROG_MEM_ENG1 + 1, 0);
    lp55xx_write(chip, LP5562_REG_PROG_MEM_ENG2, 0);
    lp55xx_write(chip, LP5562_REG_PROG_MEM_ENG2 + 1, 0);
    lp55xx_write(chip, LP5562_REG_PROG_MEM_ENG3, 0);
    lp55xx_write(chip, LP5562_REG_PROG_MEM_ENG3 + 1, 0);
// Program engines
    lp5562_write_program_memory(chip, LP5562_REG_PROG_MEM_ENG1,
    ptn.r, ptn.size_r);
    lp5562_write_program_memory(chip, LP5562_REG_PROG_MEM_ENG2,
    ptn.g, ptn.size_g);
    lp5562_write_program_memory(chip, LP5562_REG_PROG_MEM_ENG3,
    ptn.b, ptn.size_b);
// Run engines
    lp5562_run_engine(chip, true);
    return 0;
    }
    static ssize_t lp5562_store_pattern(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t len)
    {
    struct lp55xx_led *led = i2c_get_clientdata(to_i2c_client(dev));
    struct lp55xx_chip *chip = led.chip;
    struct lp55xx_predef_pattern *ptn = chip.pdata.patterns;
    let mut num_patterns: c_int = chip.pdata.num_patterns;
    unsigned long mode;
    int ret;
    ret = kstrtoul(buf, 0, &mode);
    if (ret)
    return ret;
    if (mode > num_patterns || !ptn)
    return -EINVAL;
    guard(mutex)(&chip.lock);
    ret = lp5562_run_predef_led_pattern(chip, mode);
    if (ret)
    return ret;
    return len;
    }
    static ssize_t lp5562_store_engine_mux(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t len)
    {
    struct lp55xx_led *led = i2c_get_clientdata(to_i2c_client(dev));
    struct lp55xx_chip *chip = led.chip;
    u8 mask;
    u8 val;
// LED map
// R ... Engine 1 (fixed)
// G ... Engine 2 (fixed)
// B ... Engine 3 (fixed)
// W ... Engine 1 or 2 or 3
//
    if (sysfs_streq(buf, "RGB")) {
    mask = LP5562_ENG_FOR_RGB_M;
    val = LP5562_ENG_SEL_RGB;
    } else if (sysfs_streq(buf, "W")) {
    let mut idx: enum lp55xx_engine_index = chip.engine_idx;
    mask = LP5562_ENG_FOR_W_M;
    switch (idx) {
    case LP55XX_ENGINE_1:
    val = LP5562_ENG1_FOR_W;
    break;
    case LP55XX_ENGINE_2:
    val = LP5562_ENG2_FOR_W;
    break;
    case LP55XX_ENGINE_3:
    val = LP5562_ENG3_FOR_W;
    break;
    default:
    return -EINVAL;
    }
    } else {
    dev_err(dev, "choose RGB or W\n");
    return -EINVAL;
    }
    guard(mutex)(&chip.lock);
    lp55xx_update_bits(chip, LP5562_REG_ENG_SEL, mask, val);
    return len;
    }
    static LP55XX_DEV_ATTR_WO(led_pattern, lp5562_store_pattern);
    static LP55XX_DEV_ATTR_WO(engine_mux, lp5562_store_engine_mux);
    static struct attribute *lp5562_attributes[] = {
    &dev_attr_led_pattern.attr,
    &dev_attr_engine_mux.attr,
    core::ptr::null_mut(),
    };
    static const struct attribute_group lp5562_group = {
    .attrs = lp5562_attributes,
    };
// Chip specific configurations
    static struct lp55xx_device_config lp5562_cfg = {
    .max_channel  = LP5562_MAX_LEDS,
    .reg_op_mode = {
    .addr = LP5562_REG_OP_MODE,
    },
    .reg_exec = {
    .addr = LP5562_REG_ENABLE,
    },
    .reset = {
    .addr = LP5562_REG_RESET,
    .val  = LP5562_RESET,
    },
    .enable = {
    .addr = LP5562_REG_ENABLE,
    .val  = LP5562_ENABLE_DEFAULT,
    },
    .prog_mem_base = {
    .addr = LP5562_REG_PROG_MEM_ENG1,
    },
    .post_init_device   = lp5562_post_init_device,
    .set_led_current    = lp5562_set_led_current,
    .brightness_fn      = lp5562_led_brightness,
    .multicolor_brightness_fn = lp5562_multicolor_brightness,
    .run_engine         = lp5562_run_engine,
    .firmware_cb        = lp55xx_firmware_loaded_cb,
    .dev_attr_group     = &lp5562_group,
    };
    static const struct i2c_device_id lp5562_id[] = {
    { .name = "lp5562", .driver_data = (kernel_ulong_t)&lp5562_cfg },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, lp5562_id);
    static const struct of_device_id of_lp5562_leds_match[] = {
    { .compatible = "ti,lp5562", .data = &lp5562_cfg, },
    {},
    };
    MODULE_DEVICE_TABLE(of, of_lp5562_leds_match);
    static struct i2c_driver lp5562_driver = {
    .driver = {
    .name	= "lp5562",
    .of_match_table = of_lp5562_leds_match,
    },
    .probe		= lp55xx_probe,
    .remove		= lp55xx_remove,
    .id_table	= lp5562_id,
    };
    module_i2c_driver(lp5562_driver);
    MODULE_DESCRIPTION("Texas Instruments LP5562 LED Driver");
    MODULE_AUTHOR("Milo Kim");
    MODULE_LICENSE("GPL");
