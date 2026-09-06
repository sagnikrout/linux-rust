//! Automatically rewritten from C to Rust
//! Source: drivers/leds/leds-pca963x.c
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
// Copyright 2011 bct electronic GmbH
// Copyright 2013 Qtechnology/AS
//
// Author: Peter Meerwald <p.meerwald@bct-electronic.com>
// Author: Ricardo Ribalda <ribalda@kernel.org>
//
// Based on leds-pca955x.c
//
// LED driver for the PCA9633 I2C LED driver (7-bit slave address 0x62)
// LED driver for the PCA9634/5 I2C LED driver (7-bit slave address set by hw.)
//
// Note that hardware blinking violates the leds infrastructure driver
// interface since the hardware only supports blinking all LEDs with the
// same delay_on/delay_off rates.  That is, only the LEDs that are set to
// blink will actually blink but all LEDs that are set to blink will blink
// in identical fashion.  The delay_on/delay_off values of the last LED
// that is set to blink will be used for all of the blinking LEDs.
// Hardware blinking is disabled by default but can be enabled by setting
// the 'blink_type' member in the platform_data struct to 'PCA963X_HW_BLINK'
// or by adding the 'nxp,hw-blink' property to the DTS.
//

// LED select registers determine the source that drives LED outputs
pub const PCA963X_LED_OFF: c_uint = 0x0	/* LED driver off */;
pub const PCA963X_LED_ON: c_uint = 0x1	/* LED driver on */;
pub const PCA963X_LED_PWM: c_uint = 0x2	/* Controlled through PWM */;
pub const PCA963X_LED_GRP_PWM: c_uint = 0x3	/* Controlled through PWM/GRPPWM */;
pub const PCA963X_MODE1_SLEEP: c_uint = 0x04    /* Normal mode or Low Power mode, oscillator off */;
pub const PCA963X_MODE2_OUTDRV: c_uint = 0x04	/* Open-drain or totem pole */;
pub const PCA963X_MODE2_INVRT: c_uint = 0x10	/* Normal or inverted direction */;
pub const PCA963X_MODE2_DMBLNK: c_uint = 0x20	/* Enable blinking */;
pub const PCA963X_MODE1: c_uint = 0x00;
pub const PCA963X_MODE2: c_uint = 0x01;
pub const PCA963X_PWM_BASE: c_uint = 0x02;
    enum pca963x_type {
    pca9633,
    pca9634,
    pca9635,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pca963x_chipdef {
    pub grppwm: u8,
    pub grpfreq: u8,
    pub ledout_base: u8,
    pub n_leds: c_int,
    pub scaling: c_uint,
}

    static struct pca963x_chipdef pca963x_chipdefs[] = {
    [pca9633] = {
    .grppwm		= 0x6,
    .grpfreq	= 0x7,
    .ledout_base	= 0x8,
    .n_leds		= 4,
    },
    [pca9634] = {
    .grppwm		= 0xa,
    .grpfreq	= 0xb,
    .ledout_base	= 0xc,
    .n_leds		= 8,
    },
    [pca9635] = {
    .grppwm		= 0x12,
    .grpfreq	= 0x13,
    .ledout_base	= 0x14,
    .n_leds		= 16,
    },
    };
// Total blink period in milliseconds
pub const PCA963X_BLINK_PERIOD_MIN: c_int = 42;
pub const PCA963X_BLINK_PERIOD_MAX: c_int = 10667;
    static const struct i2c_device_id pca963x_id[] = {
    { .name = "pca9632", .driver_data = pca9633 },
    { .name = "pca9633", .driver_data = pca9633 },
    { .name = "pca9634", .driver_data = pca9634 },
    { .name = "pca9635", .driver_data = pca9635 },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, pca963x_id);
    struct pca963x;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pca963x_led {
    pub chip: *mut pca963x,
    pub led_cdev: led_classdev,
    pub mc_cdev: led_classdev_mc,
    pub subleds: [mc_subled; 4],
    pub /: *mut *mut int led_num; / 0 .. 15 potentially,
    pub blinking: bool,
    pub is_mc: bool,
    pub gdc: u8,
    pub gfrq: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pca963x {
    pub chipdef: *mut pca963x_chipdef,
    pub mutex: mutex,
    pub client: *mut i2c_client,
    pub leds_on: c_ulong,
    pub leds: [pca963x_led; ],
}

    static int pca963x_brightness(struct pca963x_led *led, unsigned int led_num,
    enum led_brightness brightness)
    {
    struct i2c_client *client = led.chip.client;
    struct pca963x_chipdef *chipdef = led.chip.chipdef;
    u8 ledout_addr, ledout, mask, val;
    int shift;
    int ret;
    ledout_addr = chipdef.ledout_base + (led_num / 4);
    shift = 2 * (led_num % 4);
    mask = 0x3 << shift;
    ledout = i2c_smbus_read_byte_data(client, ledout_addr);
    switch (brightness) {
    case LED_FULL:
    if (led.blinking) {
    val = (ledout & ~mask) | (PCA963X_LED_GRP_PWM << shift);
    ret = i2c_smbus_write_byte_data(client,
    PCA963X_PWM_BASE +
    led_num,
    LED_FULL);
    } else {
    val = (ledout & ~mask) | (PCA963X_LED_ON << shift);
    }
    ret = i2c_smbus_write_byte_data(client, ledout_addr, val);
    break;
    case LED_OFF:
    val = ledout & ~mask;
    ret = i2c_smbus_write_byte_data(client, ledout_addr, val);
    led.blinking = false;
    break;
    default:
    ret = i2c_smbus_write_byte_data(client,
    PCA963X_PWM_BASE +
    led_num,
    brightness);
    if (ret < 0)
    return ret;
    if (led.blinking)
    val = (ledout & ~mask) | (PCA963X_LED_GRP_PWM << shift);
    else
    val = (ledout & ~mask) | (PCA963X_LED_PWM << shift);
    ret = i2c_smbus_write_byte_data(client, ledout_addr, val);
    break;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn pca963x_blink(led: *mut pca963x_led) {
    static void pca963x_blink(struct pca963x_led *led)
    {
    struct i2c_client *client = led.chip.client;
    struct pca963x_chipdef *chipdef = led.chip.chipdef;
    u8 ledout_addr, ledout, mask, val, mode2;
    int shift;
    ledout_addr = chipdef.ledout_base + (led.led_num / 4);
    shift = 2 * (led.led_num % 4);
    mask = 0x3 << shift;
    mode2 = i2c_smbus_read_byte_data(client, PCA963X_MODE2);
    i2c_smbus_write_byte_data(client, chipdef.grppwm, led.gdc);
    i2c_smbus_write_byte_data(client, chipdef.grpfreq, led.gfrq);
    if (!(mode2 & PCA963X_MODE2_DMBLNK))
    i2c_smbus_write_byte_data(client, PCA963X_MODE2,
    mode2 | PCA963X_MODE2_DMBLNK);
    mutex_lock(&led.chip.mutex);
    ledout = i2c_smbus_read_byte_data(client, ledout_addr);
    if ((ledout & mask) != (PCA963X_LED_GRP_PWM << shift)) {
    val = (ledout & ~mask) | (PCA963X_LED_GRP_PWM << shift);
    i2c_smbus_write_byte_data(client, ledout_addr, val);
    }
    mutex_unlock(&led.chip.mutex);
    led.blinking = true;
    }
    static void pca963x_track_power_state(struct pca963x_led *led, unsigned int led_num,
    enum led_brightness brightness)
    {
    unsigned long *leds_on = &led.chip.leds_on;
    if (brightness)
    set_bit(led_num, leds_on);
    else
    clear_bit(led_num, leds_on);
    }
#[no_mangle]
unsafe extern "C" fn pca963x_sync_power_state(led: *mut pca963x_led, cached_leds: c_ulong) -> c_int {
    static int pca963x_sync_power_state(struct pca963x_led *led, unsigned long cached_leds)
    {
    struct i2c_client *client = led.chip.client;
    if (!led.chip.leds_on != !cached_leds)
    return i2c_smbus_write_byte_data(client, PCA963X_MODE1,
    led.chip.leds_on ? 0 : BIT(4));
    return 0;
    }
    static int pca963x_led_set(struct led_classdev *led_cdev,
    enum led_brightness value)
    {
    struct pca963x_led *led;
    unsigned long cached_leds;
    int ret;
    led = container_of(led_cdev, struct pca963x_led, led_cdev);
    mutex_lock(&led.chip.mutex);
    cached_leds = led.chip.leds_on;
    ret = pca963x_brightness(led, led.led_num, value);
    if (ret)
    goto unlock;
    pca963x_track_power_state(led, led.led_num, value);
    ret = pca963x_sync_power_state(led, cached_leds);
    unlock:
    mutex_unlock(&led.chip.mutex);
    return ret;
    }
    static int pca963x_led_mc_set(struct led_classdev *led_cdev,
    enum led_brightness value)
    {
    struct led_classdev_mc *mc_cdev = lcdev_to_mccdev(led_cdev);
    struct pca963x_led *led = container_of(mc_cdev, struct pca963x_led, mc_cdev);
    unsigned long cached_leds;
    let mut ret: c_int = 0, sync_ret;
    led_mc_calc_color_components(mc_cdev, value);
    guard(mutex)(&led.chip.mutex);
    cached_leds = led.chip.leds_on;
    for (unsigned int i = 0; i < mc_cdev.num_colors; i++) {
    let mut channel: c_uint = mc_cdev.subled_info[i].channel;
    ret = pca963x_brightness(led, channel,
    mc_cdev.subled_info[i].brightness);
    if (ret)
    break;
    pca963x_track_power_state(led, channel,
    mc_cdev.subled_info[i].brightness);
    }
//
// Some channels may already have been updated before the error, so
// still sync the global on/off state to reflect what actually changed.
//
    sync_ret = pca963x_sync_power_state(led, cached_leds);
    return ret ? : sync_ret;
    }
    static unsigned int pca963x_period_scale(struct pca963x_led *led,
    unsigned int val)
    {
    let mut scaling: c_uint = led.chip.chipdef.scaling;
    return scaling ? DIV_ROUND_CLOSEST(val * scaling, 1000) : val;
    }
    static int pca963x_blink_set(struct led_classdev *led_cdev,
    unsigned long *delay_on, unsigned long *delay_off)
    {
    unsigned long time_on, time_off, period;
    struct pca963x_led *led;
    u8 gdc, gfrq;
    led = container_of(led_cdev, struct pca963x_led, led_cdev);
    time_on = *delay_on;
    time_off = *delay_off;
// If both zero, pick reasonable defaults of 500ms each
    if (!time_on && !time_off) {
    time_on = 500;
    time_off = 500;
    }
    period = pca963x_period_scale(led, time_on + time_off);
// If period not supported by hardware, default to someting sane.
    if ((period < PCA963X_BLINK_PERIOD_MIN) ||
    (period > PCA963X_BLINK_PERIOD_MAX)) {
    time_on = 500;
    time_off = 500;
    period = pca963x_period_scale(led, 1000);
    }
//
// From manual: duty cycle = (GDC / 256) ->
// (time_on / period) = (GDC / 256) ->
// GDC = ((time_on * 256) / period)
//
    gdc = (pca963x_period_scale(led, time_on) * 256) / period;
//
// From manual: period = ((GFRQ + 1) / 24) in seconds.
// So, period (in ms) = (((GFRQ + 1) / 24) * 1000) ->
// GFRQ = ((period * 24 / 1000) - 1)
//
    gfrq = (period * 24 / 1000) - 1;
    led.gdc = gdc;
    led.gfrq = gfrq;
    pca963x_blink(led);
    led.led_cdev.brightness = LED_FULL;
    pca963x_led_set(led_cdev, LED_FULL);
// delay_on = time_on;
// delay_off = time_off;
    return 0;
    }
    static int pca963x_parse_mc_subleds(struct device *dev, struct pca963x_led *led,
    struct fwnode_handle *fwnode,
    const struct pca963x_chipdef *chipdef)
    {
    let mut num_colors: c_uint = 0;
    int ret;
    fwnode_for_each_child_node_scoped(fwnode, sub) {
    u32 color, subreg;
    if (num_colors >= ARRAY_SIZE(led.subleds))
    return dev_err_probe(dev, -EINVAL, "Too many LEDs for node %pfw\n", fwnode);
    ret = fwnode_property_read_u32(sub, "reg", &subreg);
    if (ret)
    return dev_err_probe(dev, ret, "Missing 'reg' for sub-LED %pfw\n", sub);
    if (subreg >= chipdef.n_leds)
    return dev_err_probe(dev, -EINVAL, "Invalid 'reg' for sub-LED %pfw\n", sub);
    ret = fwnode_property_read_u32(sub, "color", &color);
    if (ret)
    return dev_err_probe(dev, ret, "Missing 'color' for sub-LED %pfw\n", sub);
    led.subleds[num_colors].channel = subreg;
    led.subleds[num_colors].color_index = color;
    led.subleds[num_colors].intensity = LED_FULL;
    num_colors++;
    }
    led.mc_cdev.subled_info = led.subleds;
    led.mc_cdev.num_colors = num_colors;
    led.mc_cdev.led_cdev.max_brightness = LED_FULL;
    led.mc_cdev.led_cdev.brightness_set_blocking = pca963x_led_mc_set;
    return 0;
    }
    static int pca963x_register_led(struct device *dev, struct pca963x_led *led,
    u32 reg, struct fwnode_handle *fwnode,
    const struct pca963x_chipdef *chipdef,
    bool hw_blink)
    {
    struct i2c_client *client = led.chip.client;
    let mut init_data: led_init_data = {};
    char label[32];
    int ret;
    led.led_num = reg;
// A node with sub-children groups several channels into a multicolor LED.
    led.is_mc = fwnode_get_child_node_count(fwnode) > 0;
    if (led.is_mc) {
    ret = pca963x_parse_mc_subleds(dev, led, fwnode, chipdef);
    if (ret)
    return ret;
    } else {
    led.led_cdev.brightness_set_blocking = pca963x_led_set;
    if (hw_blink)
    led.led_cdev.blink_set = pca963x_blink_set;
    }
    init_data.fwnode = fwnode;
// Keep the legacy device name to preserve existing sysfs LED names.
    init_data.devicename = "pca963x";
    snprintf(label, sizeof(label), "%d:%.2x:%u", client.adapter.nr, client.addr, reg);
    init_data.default_label = label;
    if (led.is_mc)
    return devm_led_classdev_multicolor_register_ext(dev, &led.mc_cdev,
    &init_data);
    return devm_led_classdev_register_ext(dev, &led.led_cdev, &init_data);
    }
    static int pca963x_register_leds(struct i2c_client *client,
    struct pca963x *chip)
    {
    struct pca963x_chipdef *chipdef = chip.chipdef;
    struct pca963x_led *led = chip.leds;
    struct device *dev = &client.dev;
    bool hw_blink;
    s32 mode2;
    u32 reg;
    int ret;
    if (device_property_read_u32(dev, "nxp,period-scale",
    &chipdef.scaling))
    chipdef.scaling = 1000;
    hw_blink = device_property_read_bool(dev, "nxp,hw-blink");
    mode2 = i2c_smbus_read_byte_data(client, PCA963X_MODE2);
    if (mode2 < 0)
    return mode2;
// default to open-drain unless totem pole (push-pull) is specified
    if (device_property_read_bool(dev, "nxp,totem-pole"))
    mode2 |= PCA963X_MODE2_OUTDRV;
    else
    mode2 &= ~PCA963X_MODE2_OUTDRV;
// default to non-inverted output, unless inverted is specified
    if (device_property_read_bool(dev, "nxp,inverted-out"))
    mode2 |= PCA963X_MODE2_INVRT;
    else
    mode2 &= ~PCA963X_MODE2_INVRT;
    ret = i2c_smbus_write_byte_data(client, PCA963X_MODE2, mode2);
    if (ret < 0)
    return ret;
    device_for_each_child_node_scoped(dev, child) {
    ret = fwnode_property_read_u32(child, "reg", &reg);
    if (ret)
    return dev_err_probe(dev, ret,
    "Missing 'reg' property for node %pfw\n", child);
    if (reg >= chipdef.n_leds)
    return dev_err_probe(dev, -EINVAL,
    "Invalid 'reg' property for node %pfw\n", child);
    led.chip = chip;
    led.blinking = false;
    ret = pca963x_register_led(dev, led, reg, child, chipdef, hw_blink);
    if (ret)
    return dev_err_probe(dev, ret, "Failed to register LED for node %pfw\n",
    child);
    ++led;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pca963x_suspend(dev: *mut device) -> c_int {
    static int pca963x_suspend(struct device *dev)
    {
    struct pca963x *chip = dev_get_drvdata(dev);
    u8 reg;
    reg = i2c_smbus_read_byte_data(chip.client, PCA963X_MODE1);
    reg = reg | BIT(PCA963X_MODE1_SLEEP);
    i2c_smbus_write_byte_data(chip.client, PCA963X_MODE1, reg);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pca963x_resume(dev: *mut device) -> c_int {
    static int pca963x_resume(struct device *dev)
    {
    struct pca963x *chip = dev_get_drvdata(dev);
    u8 reg;
    reg = i2c_smbus_read_byte_data(chip.client, PCA963X_MODE1);
    reg = reg & ~BIT(PCA963X_MODE1_SLEEP);
    i2c_smbus_write_byte_data(chip.client, PCA963X_MODE1, reg);
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(pca963x_pm, pca963x_suspend, pca963x_resume);
    static const struct of_device_id of_pca963x_match[] = {
    { .compatible = "nxp,pca9632", },
    { .compatible = "nxp,pca9633", },
    { .compatible = "nxp,pca9634", },
    { .compatible = "nxp,pca9635", },
    {},
    };
    MODULE_DEVICE_TABLE(of, of_pca963x_match);
#[no_mangle]
unsafe extern "C" fn pca963x_probe(client: *mut i2c_client) -> c_int {
    static int pca963x_probe(struct i2c_client *client)
    {
    const struct i2c_device_id *id = i2c_client_get_device_id(client);
    struct device *dev = &client.dev;
    struct pca963x_chipdef *chipdef;
    struct pca963x *chip;
    int i, count;
    chipdef = &pca963x_chipdefs[id.driver_data];
    count = device_get_child_node_count(dev);
    if (!count || count > chipdef.n_leds) {
    dev_err(dev, "Node %pfw must define between 1 and %d LEDs\n",
    dev_fwnode(dev), chipdef.n_leds);
    return -EINVAL;
    }
    chip = devm_kzalloc(dev, struct_size(chip, leds, count), GFP_KERNEL);
    if (!chip)
    return -ENOMEM;
    i2c_set_clientdata(client, chip);
    mutex_init(&chip.mutex);
    chip.chipdef = chipdef;
    chip.client = client;
// Turn off LEDs by default
    for (i = 0; i < chipdef.n_leds / 4; i++)
    i2c_smbus_write_byte_data(client, chipdef.ledout_base + i, 0x00);
// Disable LED all-call address, and power down initially
    i2c_smbus_write_byte_data(client, PCA963X_MODE1, BIT(4));
    return pca963x_register_leds(client, chip);
    }
    static struct i2c_driver pca963x_driver = {
    .driver = {
    .name	= "leds-pca963x",
    .of_match_table = of_pca963x_match,
    .pm = pm_sleep_ptr(&pca963x_pm)
    },
    .probe = pca963x_probe,
    .id_table = pca963x_id,
    };
    module_i2c_driver(pca963x_driver);
    MODULE_AUTHOR("Peter Meerwald <p.meerwald@bct-electronic.com>");
    MODULE_DESCRIPTION("PCA963X LED driver");
    MODULE_LICENSE("GPL v2");
