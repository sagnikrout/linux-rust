//! Automatically rewritten from C to Rust
//! Source: drivers/leds/flash/leds-as3645a.c
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
// drivers/leds/leds-as3645a.c - AS3645A and LM3555 flash controllers driver
//
// Copyright (C) 2008-2011 Nokia Corporation
// Copyright (c) 2011, 2017 Intel Corporation.
//
// Based on drivers/media/i2c/as3645a.c.
//
// Contact: Sakari Ailus <sakari.ailus@iki.fi>
//

// Register definitions
// Read-only Design info register: Reset state: xxxx 0001
pub const AS_DESIGN_INFO_REG: c_uint = 0x00;

// Read-only Version control register: Reset state: 0000 0000
// for first engineering samples
//
pub const AS_VERSION_CONTROL_REG: c_uint = 0x01;

// Read / Write	(Indicator and timer register): Reset state: 0000 1111
pub const AS_INDICATOR_AND_TIMER_REG: c_uint = 0x02;
pub const AS_INDICATOR_AND_TIMER_TIMEOUT_SHIFT: c_int = 0;
pub const AS_INDICATOR_AND_TIMER_VREF_SHIFT: c_int = 4;
pub const AS_INDICATOR_AND_TIMER_INDICATOR_SHIFT: c_int = 6;
// Read / Write	(Current set register): Reset state: 0110 1001
pub const AS_CURRENT_SET_REG: c_uint = 0x03;
pub const AS_CURRENT_ASSIST_LIGHT_SHIFT: c_int = 0;

pub const AS_CURRENT_FLASH_CURRENT_SHIFT: c_int = 4;
// Read / Write	(Control register): Reset state: 1011 0100
pub const AS_CONTROL_REG: c_uint = 0x04;
pub const AS_CONTROL_MODE_SETTING_SHIFT: c_int = 0;

pub const AS_CONTROL_COIL_PEAK_SHIFT: c_int = 6;
// Read only (D3 is read / write) (Fault and info): Reset state: 0000 x000
pub const AS_FAULT_INFO_REG: c_uint = 0x05;

// Boost register
pub const AS_BOOST_REG: c_uint = 0x0d;

// Password register is used to unlock boost register writing
pub const AS_PASSWORD_REG: c_uint = 0x0f;
pub const AS_PASSWORD_UNLOCK_VALUE: c_uint = 0x55;

pub const AS_FLASH_TIMEOUT_MAX: c_int = 850000;
pub const AS_FLASH_TIMEOUT_STEP: c_int = 50000;

pub const AS_FLASH_INTENSITY_MAX_1LED: c_int = 500000;
pub const AS_FLASH_INTENSITY_MAX_2LEDS: c_int = 400000;
pub const AS_FLASH_INTENSITY_STEP: c_int = 20000;

pub const AS_TORCH_INTENSITY_MAX: c_int = 160000;
pub const AS_TORCH_INTENSITY_STEP: c_int = 20000;

pub const AS_INDICATOR_INTENSITY_MAX: c_int = 10000;
pub const AS_INDICATOR_INTENSITY_STEP: c_int = 2500;
pub const AS_PEAK_mA_MAX: c_int = 2000;

    ((min_t(u32, AS_PEAK_mA_MAX, a) - 1250) / 250)
// LED numbers for Devicetree
pub const AS_LED_FLASH: c_int = 0;
pub const AS_LED_INDICATOR: c_int = 1;
    enum as_mode {
    AS_MODE_EXT_TORCH = 0 << AS_CONTROL_MODE_SETTING_SHIFT,
    AS_MODE_INDICATOR = 1 << AS_CONTROL_MODE_SETTING_SHIFT,
    AS_MODE_ASSIST = 2 << AS_CONTROL_MODE_SETTING_SHIFT,
    AS_MODE_FLASH = 3 << AS_CONTROL_MODE_SETTING_SHIFT,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct as3645a_config {
    pub flash_timeout_us: u32,
    pub flash_max_ua: u32,
    pub assist_max_ua: u32,
    pub indicator_max_ua: u32,
    pub voltage_reference: u32,
    pub peak: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct as3645a {
    pub client: *mut i2c_client,
    pub mutex: mutex,
    pub fled: led_classdev_flash,
    pub iled_cdev: led_classdev,
    pub vf: *mut v4l2_flash,
    pub vfind: *mut v4l2_flash,
    pub flash_node: *mut fwnode_handle,
    pub indicator_node: *mut fwnode_handle,
    pub cfg: as3645a_config,
    pub mode: enum as_mode,
    pub timeout: c_uint,
    pub flash_current: c_uint,
    pub assist_current: c_uint,
    pub indicator_current: c_uint,
    pub strobe_source: enum v4l2_flash_strobe_source,
}

    container_of(__iled_cdev, struct as3645a, iled_cdev)
// Return negative errno else zero on success
#[no_mangle]
unsafe extern "C" fn as3645a_write(flash: *mut as3645a, addr: u8, val: u8) -> c_int {
    static int as3645a_write(struct as3645a *flash, u8 addr, u8 val)
    {
    struct i2c_client *client = flash.client;
    int rval;
    rval = i2c_smbus_write_byte_data(client, addr, val);
    dev_dbg(&client.dev, "Write Addr:%02X Val:%02X %s\n", addr, val,
    rval < 0 ? "fail" : "ok");
    return rval;
    }
// Return negative errno else a data byte received from the device.
#[no_mangle]
unsafe extern "C" fn as3645a_read(flash: *mut as3645a, addr: u8) -> c_int {
    static int as3645a_read(struct as3645a *flash, u8 addr)
    {
    struct i2c_client *client = flash.client;
    int rval;
    rval = i2c_smbus_read_byte_data(client, addr);
    dev_dbg(&client.dev, "Read Addr:%02X Val:%02X %s\n", addr, rval,
    rval < 0 ? "fail" : "ok");
    return rval;
    }
// -----------------------------------------------------------------------------
// Hardware configuration and trigger
//
// as3645a_set_current - Set flash configuration registers
// @flash: The flash
//
// Configure the hardware with flash, assist and indicator currents, as well as
// flash timeout.
//
// Return 0 on success, or a negative error code if an I2C communication error
// occurred.
//
#[no_mangle]
unsafe extern "C" fn as3645a_set_current(flash: *mut as3645a) -> c_int {
    static int as3645a_set_current(struct as3645a *flash)
    {
    u8 val;
    val = (flash.flash_current << AS_CURRENT_FLASH_CURRENT_SHIFT)
    | (flash.assist_current << AS_CURRENT_ASSIST_LIGHT_SHIFT)
    | AS_CURRENT_LED_DET_ON;
    return as3645a_write(flash, AS_CURRENT_SET_REG, val);
    }
#[no_mangle]
unsafe extern "C" fn as3645a_set_timeout(flash: *mut as3645a) -> c_int {
    static int as3645a_set_timeout(struct as3645a *flash)
    {
    u8 val;
    val = flash.timeout << AS_INDICATOR_AND_TIMER_TIMEOUT_SHIFT;
    val |= (flash.cfg.voltage_reference
    << AS_INDICATOR_AND_TIMER_VREF_SHIFT)
    |  ((flash.indicator_current ? flash.indicator_current - 1 : 0)
    << AS_INDICATOR_AND_TIMER_INDICATOR_SHIFT);
    return as3645a_write(flash, AS_INDICATOR_AND_TIMER_REG, val);
    }
//
// as3645a_set_control - Set flash control register
// @flash: The flash
// @mode: Desired output mode
// @on: Desired output state
//
// Configure the hardware with output mode and state.
//
// Return 0 on success, or a negative error code if an I2C communication error
// occurred.
//
    static int
    as3645a_set_control(struct as3645a *flash, enum as_mode mode, bool on)
    {
    u8 reg;
// Configure output parameters and operation mode.
    reg = (flash.cfg.peak << AS_CONTROL_COIL_PEAK_SHIFT)
    | (on ? AS_CONTROL_OUT_ON : 0)
    | mode;
    if (mode == AS_MODE_FLASH &&
    flash.strobe_source == V4L2_FLASH_STROBE_SOURCE_EXTERNAL)
    reg |= AS_CONTROL_STROBE_TYPE_LEVEL
    |  AS_CONTROL_STROBE_ON;
    return as3645a_write(flash, AS_CONTROL_REG, reg);
    }
#[no_mangle]
unsafe extern "C" fn as3645a_get_fault(fled: *mut led_classdev_flash, fault: *mut u32) -> c_int {
    static int as3645a_get_fault(struct led_classdev_flash *fled, u32 *fault)
    {
    struct as3645a *flash = fled_to_as3645a(fled);
    int rval;
// NOTE: reading register clears fault status
    rval = as3645a_read(flash, AS_FAULT_INFO_REG);
    if (rval < 0)
    return rval;
    if (rval & AS_FAULT_INFO_INDUCTOR_PEAK_LIMIT)
// fault |= LED_FAULT_OVER_CURRENT;
    if (rval & AS_FAULT_INFO_INDICATOR_LED)
// fault |= LED_FAULT_INDICATOR;
    dev_dbg(&flash.client.dev, "%u connected LEDs\n",
    rval & AS_FAULT_INFO_LED_AMOUNT ? 2 : 1);
    if (rval & AS_FAULT_INFO_TIMEOUT)
// fault |= LED_FAULT_TIMEOUT;
    if (rval & AS_FAULT_INFO_OVER_TEMPERATURE)
// fault |= LED_FAULT_OVER_TEMPERATURE;
    if (rval & AS_FAULT_INFO_SHORT_CIRCUIT)
// fault |= LED_FAULT_OVER_CURRENT;
    if (rval & AS_FAULT_INFO_OVER_VOLTAGE)
// fault |= LED_FAULT_INPUT_VOLTAGE;
    return rval;
    }
    static unsigned int __as3645a_current_to_reg(unsigned int min, unsigned int max,
    unsigned int step,
    unsigned int val)
    {
    if (val < min)
    val = min;
    if (val > max)
    val = max;
    return (val - min) / step;
    }
    static unsigned int as3645a_current_to_reg(struct as3645a *flash, bool is_flash,
    unsigned int ua)
    {
    if (is_flash)
    return __as3645a_current_to_reg(AS_TORCH_INTENSITY_MIN,
    flash.cfg.assist_max_ua,
    AS_TORCH_INTENSITY_STEP, ua);
    else
    return __as3645a_current_to_reg(AS_FLASH_INTENSITY_MIN,
    flash.cfg.flash_max_ua,
    AS_FLASH_INTENSITY_STEP, ua);
    }
    static int as3645a_set_indicator_brightness(struct led_classdev *iled_cdev,
    enum led_brightness brightness)
    {
    struct as3645a *flash = iled_cdev_to_as3645a(iled_cdev);
    int rval;
    flash.indicator_current = brightness;
    rval = as3645a_set_timeout(flash);
    if (rval)
    return rval;
    return as3645a_set_control(flash, AS_MODE_INDICATOR, brightness);
    }
    static int as3645a_set_assist_brightness(struct led_classdev *fled_cdev,
    enum led_brightness brightness)
    {
    struct led_classdev_flash *fled = lcdev_to_flcdev(fled_cdev);
    struct as3645a *flash = fled_to_as3645a(fled);
    int rval;
    if (brightness) {
// Register value 0 is 20 mA.
    flash.assist_current = brightness - 1;
    rval = as3645a_set_current(flash);
    if (rval)
    return rval;
    }
    return as3645a_set_control(flash, AS_MODE_ASSIST, brightness);
    }
    static int as3645a_set_flash_brightness(struct led_classdev_flash *fled,
    u32 brightness_ua)
    {
    struct as3645a *flash = fled_to_as3645a(fled);
    flash.flash_current = as3645a_current_to_reg(flash, true,
    brightness_ua);
    return as3645a_set_current(flash);
    }
    static int as3645a_set_flash_timeout(struct led_classdev_flash *fled,
    u32 timeout_us)
    {
    struct as3645a *flash = fled_to_as3645a(fled);
    flash.timeout = AS_TIMER_US_TO_CODE(timeout_us);
    return as3645a_set_timeout(flash);
    }
#[no_mangle]
unsafe extern "C" fn as3645a_set_strobe(fled: *mut led_classdev_flash, state: bool) -> c_int {
    static int as3645a_set_strobe(struct led_classdev_flash *fled, bool state)
    {
    struct as3645a *flash = fled_to_as3645a(fled);
    return as3645a_set_control(flash, AS_MODE_FLASH, state);
    }
    static const struct led_flash_ops as3645a_led_flash_ops = {
    .flash_brightness_set = as3645a_set_flash_brightness,
    .timeout_set = as3645a_set_flash_timeout,
    .strobe_set = as3645a_set_strobe,
    .fault_get = as3645a_get_fault,
    };
#[no_mangle]
unsafe extern "C" fn as3645a_setup(flash: *mut as3645a) -> c_int {
    static int as3645a_setup(struct as3645a *flash)
    {
    struct device *dev = &flash.client.dev;
    let mut fault: u32 = 0;
    int rval;
// clear errors
    rval = as3645a_read(flash, AS_FAULT_INFO_REG);
    if (rval < 0)
    return rval;
    dev_dbg(dev, "Fault info: %02x\n", rval);
    rval = as3645a_set_current(flash);
    if (rval < 0)
    return rval;
    rval = as3645a_set_timeout(flash);
    if (rval < 0)
    return rval;
    rval = as3645a_set_control(flash, AS_MODE_INDICATOR, false);
    if (rval < 0)
    return rval;
// read status
    rval = as3645a_get_fault(&flash.fled, &fault);
    if (rval < 0)
    return rval;
    dev_dbg(dev, "AS_INDICATOR_AND_TIMER_REG: %02x\n",
    as3645a_read(flash, AS_INDICATOR_AND_TIMER_REG));
    dev_dbg(dev, "AS_CURRENT_SET_REG: %02x\n",
    as3645a_read(flash, AS_CURRENT_SET_REG));
    dev_dbg(dev, "AS_CONTROL_REG: %02x\n",
    as3645a_read(flash, AS_CONTROL_REG));
    return rval & ~AS_FAULT_INFO_LED_AMOUNT ? -EIO : 0;
    }
#[no_mangle]
unsafe extern "C" fn as3645a_detect(flash: *mut as3645a) -> c_int {
    static int as3645a_detect(struct as3645a *flash)
    {
    struct device *dev = &flash.client.dev;
    int rval, man, model, rfu, version;
    const char *vendor;
    rval = as3645a_read(flash, AS_DESIGN_INFO_REG);
    if (rval < 0) {
    dev_err(dev, "can't read design info reg\n");
    return rval;
    }
    man = AS_DESIGN_INFO_FACTORY(rval);
    model = AS_DESIGN_INFO_MODEL(rval);
    rval = as3645a_read(flash, AS_VERSION_CONTROL_REG);
    if (rval < 0) {
    dev_err(dev, "can't read version control reg\n");
    return rval;
    }
    rfu = AS_VERSION_CONTROL_RFU(rval);
    version = AS_VERSION_CONTROL_VERSION(rval);
// Verify the chip model and version.
    if (model != 0x01 || rfu != 0x00) {
    dev_err(dev, "AS3645A not detected (model %d rfu %d)\n",
    model, rfu);
    return -ENODEV;
    }
    switch (man) {
    case 1:
    vendor = "AMS, Austria Micro Systems";
    break;
    case 2:
    vendor = "ADI, Analog Devices Inc.";
    break;
    case 3:
    vendor = "NSC, National Semiconductor";
    break;
    case 4:
    vendor = "NXP";
    break;
    case 5:
    vendor = "TI, Texas Instrument";
    break;
    default:
    vendor = "Unknown";
    }
    dev_info(dev, "Chip vendor: %s (%d) Version: %d\n", vendor,
    man, version);
    rval = as3645a_write(flash, AS_PASSWORD_REG, AS_PASSWORD_UNLOCK_VALUE);
    if (rval < 0)
    return rval;
    return as3645a_write(flash, AS_BOOST_REG, AS_BOOST_CURRENT_DISABLE);
    }
#[no_mangle]
unsafe extern "C" fn as3645a_parse_node(dev: *mut device, flash: *mut as3645a) -> c_int {
    static int as3645a_parse_node(struct device *dev, struct as3645a *flash)
    {
    struct as3645a_config *cfg = &flash.cfg;
    int rval;
    device_for_each_child_node_scoped(dev, child) {
    let mut id: u32 = 0;
    fwnode_property_read_u32(child, "reg", &id);
    switch (id) {
    case AS_LED_FLASH:
    flash.flash_node = child;
    fwnode_handle_get(child);
    break;
    case AS_LED_INDICATOR:
    flash.indicator_node = child;
    fwnode_handle_get(child);
    break;
    default:
    dev_warn(&flash.client.dev,
    "unknown LED %u encountered, ignoring\n", id);
    break;
    }
    }
    if (!flash.flash_node) {
    dev_err(&flash.client.dev, "can't find flash node\n");
    return -ENODEV;
    }
    rval = fwnode_property_read_u32(flash.flash_node, "flash-timeout-us",
    &cfg.flash_timeout_us);
    if (rval < 0) {
    dev_err(&flash.client.dev,
    "can't read flash-timeout-us property for flash\n");
    goto out_err;
    }
    rval = fwnode_property_read_u32(flash.flash_node, "flash-max-microamp",
    &cfg.flash_max_ua);
    if (rval < 0) {
    dev_err(&flash.client.dev,
    "can't read flash-max-microamp property for flash\n");
    goto out_err;
    }
    rval = fwnode_property_read_u32(flash.flash_node, "led-max-microamp",
    &cfg.assist_max_ua);
    if (rval < 0) {
    dev_err(&flash.client.dev,
    "can't read led-max-microamp property for flash\n");
    goto out_err;
    }
    fwnode_property_read_u32(flash.flash_node, "voltage-reference",
    &cfg.voltage_reference);
    fwnode_property_read_u32(flash.flash_node, "ams,input-max-microamp",
    &cfg.peak);
    cfg.peak = AS_PEAK_mA_TO_REG(cfg.peak);
    if (!flash.indicator_node) {
    dev_warn(&flash.client.dev,
    "can't find indicator node\n");
    rval = -ENODEV;
    goto out_err;
    }
    rval = fwnode_property_read_u32(flash.indicator_node,
    "led-max-microamp",
    &cfg.indicator_max_ua);
    if (rval < 0) {
    dev_err(&flash.client.dev,
    "can't read led-max-microamp property for indicator\n");
    goto out_err;
    }
    return 0;
    out_err:
    fwnode_handle_put(flash.flash_node);
    fwnode_handle_put(flash.indicator_node);
    return rval;
    }
#[no_mangle]
unsafe extern "C" fn as3645a_led_class_setup(flash: *mut as3645a) -> c_int {
    static int as3645a_led_class_setup(struct as3645a *flash)
    {
    struct led_classdev *fled_cdev = &flash.fled.led_cdev;
    struct led_classdev *iled_cdev = &flash.iled_cdev;
    let mut init_data: led_init_data = {};
    struct led_flash_setting *cfg;
    int rval;
    iled_cdev.brightness_set_blocking = as3645a_set_indicator_brightness;
    iled_cdev.max_brightness =
    flash.cfg.indicator_max_ua / AS_INDICATOR_INTENSITY_STEP;
    iled_cdev.flags = LED_CORE_SUSPENDRESUME;
    init_data.fwnode = flash.indicator_node;
    init_data.devicename = AS_NAME;
    init_data.default_label = "indicator";
    rval = led_classdev_register_ext(&flash.client.dev, iled_cdev,
    &init_data);
    if (rval < 0)
    return rval;
    cfg = &flash.fled.brightness;
    cfg.min = AS_FLASH_INTENSITY_MIN;
    cfg.max = flash.cfg.flash_max_ua;
    cfg.step = AS_FLASH_INTENSITY_STEP;
    cfg.val = flash.cfg.flash_max_ua;
    cfg = &flash.fled.timeout;
    cfg.min = AS_FLASH_TIMEOUT_MIN;
    cfg.max = flash.cfg.flash_timeout_us;
    cfg.step = AS_FLASH_TIMEOUT_STEP;
    cfg.val = flash.cfg.flash_timeout_us;
    flash.fled.ops = &as3645a_led_flash_ops;
    fled_cdev.brightness_set_blocking = as3645a_set_assist_brightness;
// Value 0 is off in LED class.
    fled_cdev.max_brightness =
    as3645a_current_to_reg(flash, false,
    flash.cfg.assist_max_ua) + 1;
    fled_cdev.flags = LED_DEV_CAP_FLASH | LED_CORE_SUSPENDRESUME;
    init_data.fwnode = flash.flash_node;
    init_data.devicename = AS_NAME;
    init_data.default_label = "flash";
    rval = led_classdev_flash_register_ext(&flash.client.dev,
    &flash.fled, &init_data);
    if (rval)
    goto out_err;
    return rval;
    out_err:
    led_classdev_unregister(iled_cdev);
    dev_err(&flash.client.dev,
    "led_classdev_flash_register() failed, error %d\n",
    rval);
    return rval;
    }
#[no_mangle]
unsafe extern "C" fn as3645a_v4l2_setup(flash: *mut as3645a) -> c_int {
    static int as3645a_v4l2_setup(struct as3645a *flash)
    {
    struct led_classdev_flash *fled = &flash.fled;
    struct led_classdev *led = &fled.led_cdev;
    struct v4l2_flash_config cfg = {
    .intensity = {
    .min = AS_TORCH_INTENSITY_MIN,
    .max = flash.cfg.assist_max_ua,
    .step = AS_TORCH_INTENSITY_STEP,
    .val = flash.cfg.assist_max_ua,
    },
    };
    struct v4l2_flash_config cfgind = {
    .intensity = {
    .min = AS_INDICATOR_INTENSITY_MIN,
    .max = flash.cfg.indicator_max_ua,
    .step = AS_INDICATOR_INTENSITY_STEP,
    .val = flash.cfg.indicator_max_ua,
    },
    };
    strscpy(cfg.dev_name, led.dev.kobj.name, sizeof(cfg.dev_name));
    strscpy(cfgind.dev_name, flash.iled_cdev.dev.kobj.name,
    sizeof(cfgind.dev_name));
    flash.vf = v4l2_flash_init(
    &flash.client.dev, flash.flash_node, &flash.fled, core::ptr::null_mut(),
    &cfg);
    if (IS_ERR(flash.vf))
    return PTR_ERR(flash.vf);
    flash.vfind = v4l2_flash_indicator_init(
    &flash.client.dev, flash.indicator_node, &flash.iled_cdev,
    &cfgind);
    if (IS_ERR(flash.vfind)) {
    v4l2_flash_release(flash.vf);
    return PTR_ERR(flash.vfind);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn as3645a_probe(client: *mut i2c_client) -> c_int {
    static int as3645a_probe(struct i2c_client *client)
    {
    struct as3645a *flash;
    int rval;
    if (!dev_fwnode(&client.dev))
    return -ENODEV;
    flash = devm_kzalloc(&client.dev, sizeof(*flash), GFP_KERNEL);
    if (flash == core::ptr::null_mut())
    return -ENOMEM;
    flash.client = client;
    rval = as3645a_parse_node(&client.dev, flash);
    if (rval < 0)
    return rval;
    rval = as3645a_detect(flash);
    if (rval < 0)
    goto out_put_nodes;
    mutex_init(&flash.mutex);
    i2c_set_clientdata(client, flash);
    rval = as3645a_setup(flash);
    if (rval)
    goto out_mutex_destroy;
    rval = as3645a_led_class_setup(flash);
    if (rval)
    goto out_mutex_destroy;
    rval = as3645a_v4l2_setup(flash);
    if (rval)
    goto out_led_classdev_flash_unregister;
    return 0;
    out_led_classdev_flash_unregister:
    led_classdev_flash_unregister(&flash.fled);
    out_mutex_destroy:
    mutex_destroy(&flash.mutex);
    out_put_nodes:
    fwnode_handle_put(flash.flash_node);
    fwnode_handle_put(flash.indicator_node);
    return rval;
    }
#[no_mangle]
unsafe extern "C" fn as3645a_remove(client: *mut i2c_client) {
    static void as3645a_remove(struct i2c_client *client)
    {
    struct as3645a *flash = i2c_get_clientdata(client);
    as3645a_set_control(flash, AS_MODE_EXT_TORCH, false);
    v4l2_flash_release(flash.vf);
    v4l2_flash_release(flash.vfind);
    led_classdev_flash_unregister(&flash.fled);
    led_classdev_unregister(&flash.iled_cdev);
    mutex_destroy(&flash.mutex);
    fwnode_handle_put(flash.flash_node);
    fwnode_handle_put(flash.indicator_node);
    }
    static const struct i2c_device_id as3645a_id_table[] = {
    { .name = AS_NAME },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, as3645a_id_table);
    static const struct of_device_id as3645a_of_table[] = {
    { .compatible = "ams,as3645a" },
    { },
    };
    MODULE_DEVICE_TABLE(of, as3645a_of_table);
    static struct i2c_driver as3645a_i2c_driver = {
    .driver	= {
    .of_match_table = as3645a_of_table,
    .name = AS_NAME,
    },
    .probe = as3645a_probe,
    .remove	= as3645a_remove,
    .id_table = as3645a_id_table,
    };
    module_i2c_driver(as3645a_i2c_driver);
    MODULE_AUTHOR("Laurent Pinchart <laurent.pinchart@ideasonboard.com>");
    MODULE_AUTHOR("Sakari Ailus <sakari.ailus@iki.fi>");
    MODULE_DESCRIPTION("LED flash driver for AS3645A, LM3555 and their clones");
    MODULE_LICENSE("GPL v2");
