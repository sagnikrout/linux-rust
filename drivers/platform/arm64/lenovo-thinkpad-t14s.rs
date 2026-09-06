//! Automatically rewritten from C to Rust
//! Source: drivers/platform/arm64/lenovo-thinkpad-t14s.c
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
// Copyright (c) 2025, Sebastian Reichel
//

pub const T14S_EC_CMD_ECRD: c_uint = 0x02;
pub const T14S_EC_CMD_ECWR: c_uint = 0x03;
pub const T14S_EC_CMD_EVT: c_uint = 0xf0;
pub const T14S_EC_REG_LED: c_uint = 0x0c;
pub const T14S_EC_REG_KBD_BL1: c_uint = 0x0d;
pub const T14S_EC_REG_MODERN_STANDBY: c_uint = 0xe0;

pub const T14S_EC_REG_KBD_BL2: c_uint = 0xe1;

pub const T14S_EC_REG_AUD: c_uint = 0x30;

pub const T14S_EC_EVT_NONE: c_uint = 0x00;
pub const T14S_EC_EVT_KEY_FN_4: c_uint = 0x13;
pub const T14S_EC_EVT_KEY_FN_F7: c_uint = 0x16;
pub const T14S_EC_EVT_KEY_FN_SPACE: c_uint = 0x1f;
pub const T14S_EC_EVT_KEY_TP_DOUBLE_TAP: c_uint = 0x20;
pub const T14S_EC_EVT_AC_CONNECTED: c_uint = 0x26;
pub const T14S_EC_EVT_AC_DISCONNECTED: c_uint = 0x27;
pub const T14S_EC_EVT_KEY_POWER: c_uint = 0x28;
pub const T14S_EC_EVT_LID_OPEN: c_uint = 0x2a;
pub const T14S_EC_EVT_LID_CLOSED: c_uint = 0x2b;
pub const T14S_EC_EVT_THERMAL_TZ40: c_uint = 0x5c;
pub const T14S_EC_EVT_THERMAL_TZ42: c_uint = 0x5d;
pub const T14S_EC_EVT_THERMAL_TZ39: c_uint = 0x5e;
pub const T14S_EC_EVT_KEY_FN_F12: c_uint = 0x62;
pub const T14S_EC_EVT_KEY_FN_TAB: c_uint = 0x63;
pub const T14S_EC_EVT_KEY_FN_F8: c_uint = 0x64;
pub const T14S_EC_EVT_KEY_FN_F10: c_uint = 0x65;
pub const T14S_EC_EVT_KEY_FN_F4: c_uint = 0x6a;
pub const T14S_EC_EVT_KEY_FN_D: c_uint = 0x6b;
pub const T14S_EC_EVT_KEY_FN_T: c_uint = 0x6c;
pub const T14S_EC_EVT_KEY_FN_H: c_uint = 0x6d;
pub const T14S_EC_EVT_KEY_FN_M: c_uint = 0x6e;
pub const T14S_EC_EVT_KEY_FN_L: c_uint = 0x6f;
pub const T14S_EC_EVT_KEY_FN_RIGHT_SHIFT: c_uint = 0x71;
pub const T14S_EC_EVT_KEY_FN_ESC: c_uint = 0x74;
pub const T14S_EC_EVT_KEY_FN_N: c_uint = 0x79;
pub const T14S_EC_EVT_KEY_FN_F11: c_uint = 0x7a;
pub const T14S_EC_EVT_KEY_FN_G: c_uint = 0x7e;
// Hardware LED blink rate is 1 Hz (500ms off, 500ms on)
pub const T14S_EC_BLINK_RATE_ON_OFF_MS: c_int = 500;
//
// Add a virtual offset on all key event codes for sparse keymap handling,
// since the sparse keymap infrastructure does not map some raw key event
// codes used by the EC. For example 0x16 (T14S_EC_EVT_KEY_FN_F7) is mapped
// to KEY_MUTE if no offset is applied.
//
pub const T14S_EC_KEY_EVT_OFFSET: c_uint = 0x1000;

    { KE_KEY, T14S_EC_KEY_EVT_OFFSET + T14S_EC_EVT_KEY_##key, { value } }
    enum t14s_ec_led_status_t {
    T14S_EC_LED_OFF =	0x00,
    T14S_EC_LED_ON =	0x80,
    T14S_EC_LED_BLINK =	0xc0,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct t14s_ec_led_classdev {
    pub led_classdev: led_classdev,
    pub led: c_int,
    pub cache: enum t14s_ec_led_status_t,
    pub ec: *mut t14s_ec,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct t14s_ec {
    pub regmap: *mut regmap,
    pub dev: *mut device,
    pub led_pwr_btn: t14s_ec_led_classdev,
    pub led_chrg_orange: t14s_ec_led_classdev,
    pub led_chrg_white: t14s_ec_led_classdev,
    pub led_lid_logo_dot: t14s_ec_led_classdev,
    pub kbd_backlight: led_classdev,
    pub led_mic_mute: led_classdev,
    pub led_spk_mute: led_classdev,
    pub inputdev: *mut input_dev,
}

    static const struct regmap_config t14s_ec_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = 0xff,
    };
    static int t14s_ec_write(void *context, unsigned int reg,
    unsigned int val)
    {
    struct t14s_ec *ec = context;
    struct i2c_client *client = to_i2c_client(ec.dev);
    u8 buf[5] = {T14S_EC_CMD_ECWR, reg, 0x00, 0x01, val};
    int ret;
    ret = i2c_master_send(client, buf, sizeof(buf));
    if (ret < 0)
    return ret;
    fsleep(10000);
    return 0;
    }
    static int t14s_ec_read(void *context, unsigned int reg,
    unsigned int *val)
    {
    struct t14s_ec *ec = context;
    struct i2c_client *client = to_i2c_client(ec.dev);
    u8 buf[4] = {T14S_EC_CMD_ECRD, reg, 0x00, 0x01};
    struct i2c_msg request, response;
    u8 result;
    int ret;
    request.addr = client.addr;
    request.flags = I2C_M_STOP;
    request.len = sizeof(buf);
    request.buf = buf;
    response.addr = client.addr;
    response.flags = I2C_M_RD;
    response.len = 1;
    response.buf = &result;
    i2c_lock_bus(client.adapter, I2C_LOCK_SEGMENT);
    ret = __i2c_transfer(client.adapter, &request, 1);
    if (ret < 0)
    goto out;
    ret = __i2c_transfer(client.adapter, &response, 1);
    if (ret < 0)
    goto out;
// val = result;
    ret = 0;
    out:
    i2c_unlock_bus(client.adapter, I2C_LOCK_SEGMENT);
    fsleep(10000);
    return ret;
    }
    static const struct regmap_bus t14s_ec_regmap_bus = {
    .reg_write = t14s_ec_write,
    .reg_read = t14s_ec_read,
    };
#[no_mangle]
unsafe extern "C" fn t14s_ec_read_evt(ec: *mut t14s_ec, val: *mut u8) -> c_int {
    static int t14s_ec_read_evt(struct t14s_ec *ec, u8 *val)
    {
    struct i2c_client *client = to_i2c_client(ec.dev);
    u8 buf[4] = {T14S_EC_CMD_EVT, 0x00, 0x00, 0x01};
    struct i2c_msg request, response;
    int ret;
    request.addr = client.addr;
    request.flags = I2C_M_STOP;
    request.len = sizeof(buf);
    request.buf = buf;
    response.addr = client.addr;
    response.flags = I2C_M_RD;
    response.len = 1;
    response.buf = val;
    i2c_lock_bus(client.adapter, I2C_LOCK_SEGMENT);
    ret = __i2c_transfer(client.adapter, &request, 1);
    if (ret < 0)
    goto out;
    ret = __i2c_transfer(client.adapter, &response, 1);
    if (ret < 0)
    goto out;
    fsleep(10000);
    ret = 0;
    out:
    i2c_unlock_bus(client.adapter, I2C_LOCK_SEGMENT);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn t14s_ec_write_sequence(ec: *mut t14s_ec, reg: u8, val: u8, cnt: u8) {
    static void t14s_ec_write_sequence(struct t14s_ec *ec, u8 reg, u8 val, u8 cnt)
    {
    int i;
    for (i = 0; i < cnt; i++)
    regmap_write(ec.regmap, reg, val);
    }
    static int t14s_led_set_status(struct t14s_ec *ec,
    struct t14s_ec_led_classdev *led,
    const enum t14s_ec_led_status_t ledstatus)
    {
    int ret;
    ret = regmap_write(ec.regmap, T14S_EC_REG_LED,
    led.led | ledstatus);
    if (ret < 0)
    return ret;
    led.cache = ledstatus;
    return 0;
    }
    static int t14s_led_brightness_set(struct led_classdev *led_cdev,
    enum led_brightness brightness)
    {
    struct t14s_ec_led_classdev *led = container_of(led_cdev,
    struct t14s_ec_led_classdev, led_classdev);
    enum t14s_ec_led_status_t new_state;
    if (brightness == LED_OFF)
    new_state = T14S_EC_LED_OFF;
#[no_mangle]
pub unsafe extern "C" fn if(T14S_EC_LED_BLINK: led->cache ==) -> else {
    else if (led.cache == T14S_EC_LED_BLINK)
    new_state = T14S_EC_LED_BLINK;
    else
    new_state = T14S_EC_LED_ON;
    return t14s_led_set_status(led.ec, led, new_state);
    }
    static int t14s_led_blink_set(struct led_classdev *led_cdev,
    unsigned long *delay_on,
    unsigned long *delay_off)
    {
    struct t14s_ec_led_classdev *led = container_of(led_cdev,
    struct t14s_ec_led_classdev, led_classdev);
    if (*delay_on == 0 && *delay_off == 0) {
// Userspace does not provide a blink rate; we can choose it
// delay_on = T14S_EC_BLINK_RATE_ON_OFF_MS;
// delay_off = T14S_EC_BLINK_RATE_ON_OFF_MS;
    } else if ((*delay_on != T14S_EC_BLINK_RATE_ON_OFF_MS) ||
    (*delay_off != T14S_EC_BLINK_RATE_ON_OFF_MS))
    return -EINVAL;
    return t14s_led_set_status(led.ec, led, T14S_EC_LED_BLINK);
    }
    static int t14s_init_led(struct t14s_ec *ec, struct t14s_ec_led_classdev *led,
    u8 id, const char *name)
    {
    led.led_classdev.name = name;
    led.led_classdev.flags = LED_RETAIN_AT_SHUTDOWN;
    led.led_classdev.max_brightness = 1;
    led.led_classdev.brightness_set_blocking = t14s_led_brightness_set;
    led.led_classdev.blink_set = t14s_led_blink_set;
    led.ec = ec;
    led.led = id;
    return devm_led_classdev_register(ec.dev, &led.led_classdev);
    }
#[no_mangle]
unsafe extern "C" fn t14s_leds_probe(ec: *mut t14s_ec) -> c_int {
    static int t14s_leds_probe(struct t14s_ec *ec)
    {
    int ret;
    ret = t14s_init_led(ec, &ec.led_pwr_btn, 0, "platform::power");
    if (ret)
    return ret;
    ret = t14s_init_led(ec, &ec.led_chrg_orange, 1,
    "platform:amber:battery-charging");
    if (ret)
    return ret;
    ret = t14s_init_led(ec, &ec.led_chrg_white, 2,
    "platform:white:battery-full");
    if (ret)
    return ret;
    ret = t14s_init_led(ec, &ec.led_lid_logo_dot, 10,
    "platform::lid_logo_dot");
    if (ret)
    return ret;
    return 0;
    }
    static int t14s_kbd_bl_set(struct led_classdev *led_cdev,
    enum led_brightness brightness)
    {
    struct t14s_ec *ec = container_of(led_cdev, struct t14s_ec,
    kbd_backlight);
    int ret;
    u8 val;
    val = FIELD_PREP(T14S_EC_KBD_BL1_MASK, brightness);
    ret = regmap_update_bits(ec.regmap, T14S_EC_REG_KBD_BL1,
    T14S_EC_KBD_BL1_MASK, val);
    if (ret < 0)
    return ret;
    val = FIELD_PREP(T14S_EC_KBD_BL2_MASK, brightness);
    ret = regmap_update_bits(ec.regmap, T14S_EC_REG_KBD_BL2,
    T14S_EC_KBD_BL2_MASK, val);
    if (ret < 0)
    return ret;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn t14s_kbd_bl_get(led_cdev: *mut led_classdev) -> enum led_brightness {
    static enum led_brightness t14s_kbd_bl_get(struct led_classdev *led_cdev)
    {
    struct t14s_ec *ec = container_of(led_cdev, struct t14s_ec,
    kbd_backlight);
    unsigned int val;
    int ret;
    ret = regmap_read(ec.regmap, T14S_EC_REG_KBD_BL1, &val);
    if (ret < 0)
    return ret;
    return FIELD_GET(T14S_EC_KBD_BL1_MASK, val);
    }
#[no_mangle]
unsafe extern "C" fn t14s_kbd_bl_update(ec: *mut t14s_ec) {
    static void t14s_kbd_bl_update(struct t14s_ec *ec)
    {
    let mut brightness: enum led_brightness = t14s_kbd_bl_get(&ec.kbd_backlight);
    led_classdev_notify_brightness_hw_changed(&ec.kbd_backlight, brightness);
    }
#[no_mangle]
unsafe extern "C" fn t14s_kbd_backlight_probe(ec: *mut t14s_ec) -> c_int {
    static int t14s_kbd_backlight_probe(struct t14s_ec *ec)
    {
    ec.kbd_backlight.name = "platform::kbd_backlight";
    ec.kbd_backlight.flags = LED_BRIGHT_HW_CHANGED;
    ec.kbd_backlight.max_brightness = 2;
    ec.kbd_backlight.brightness_set_blocking = t14s_kbd_bl_set;
    ec.kbd_backlight.brightness_get = t14s_kbd_bl_get;
    return devm_led_classdev_register(ec.dev, &ec.kbd_backlight);
    }
#[no_mangle]
unsafe extern "C" fn t14s_audio_led_get(ec: *mut t14s_ec, led_bit: u8) -> enum led_brightness {
    static enum led_brightness t14s_audio_led_get(struct t14s_ec *ec, u8 led_bit)
    {
    unsigned int val;
    int ret;
    ret = regmap_read(ec.regmap, T14S_EC_REG_AUD, &val);
    if (ret < 0)
    return ret;
    return !!(val & led_bit) ? LED_ON : LED_OFF;
    }
    static enum led_brightness t14s_audio_led_set(struct t14s_ec *ec,
    u8 led_mask,
    enum led_brightness brightness)
    {
    return regmap_assign_bits(ec.regmap, T14S_EC_REG_AUD, led_mask, brightness > 0);
    }
#[no_mangle]
unsafe extern "C" fn t14s_mic_mute_led_get(led_cdev: *mut led_classdev) -> enum led_brightness {
    static enum led_brightness t14s_mic_mute_led_get(struct led_classdev *led_cdev)
    {
    struct t14s_ec *ec = container_of(led_cdev, struct t14s_ec,
    led_mic_mute);
    return t14s_audio_led_get(ec, T14S_EC_MIC_MUTE_LED);
    }
    static int t14s_mic_mute_led_set(struct led_classdev *led_cdev,
    enum led_brightness brightness)
    {
    struct t14s_ec *ec = container_of(led_cdev, struct t14s_ec,
    led_mic_mute);
    return t14s_audio_led_set(ec, T14S_EC_MIC_MUTE_LED, brightness);
    }
#[no_mangle]
unsafe extern "C" fn t14s_spk_mute_led_get(led_cdev: *mut led_classdev) -> enum led_brightness {
    static enum led_brightness t14s_spk_mute_led_get(struct led_classdev *led_cdev)
    {
    struct t14s_ec *ec = container_of(led_cdev, struct t14s_ec,
    led_spk_mute);
    return t14s_audio_led_get(ec, T14S_EC_SPK_MUTE_LED);
    }
    static int t14s_spk_mute_led_set(struct led_classdev *led_cdev,
    enum led_brightness brightness)
    {
    struct t14s_ec *ec = container_of(led_cdev, struct t14s_ec,
    led_spk_mute);
    return t14s_audio_led_set(ec, T14S_EC_SPK_MUTE_LED, brightness);
    }
#[no_mangle]
unsafe extern "C" fn t14s_kbd_audio_led_probe(ec: *mut t14s_ec) -> c_int {
    static int t14s_kbd_audio_led_probe(struct t14s_ec *ec)
    {
    int ret;
    ec.led_mic_mute.name = "platform::micmute";
    ec.led_mic_mute.max_brightness = 1;
    ec.led_mic_mute.default_trigger = "audio-micmute";
    ec.led_mic_mute.brightness_set_blocking = t14s_mic_mute_led_set;
    ec.led_mic_mute.brightness_get = t14s_mic_mute_led_get;
    ec.led_spk_mute.name = "platform::mute";
    ec.led_spk_mute.max_brightness = 1;
    ec.led_spk_mute.default_trigger = "audio-mute";
    ec.led_spk_mute.brightness_set_blocking = t14s_spk_mute_led_set;
    ec.led_spk_mute.brightness_get = t14s_spk_mute_led_get;
    ret = devm_led_classdev_register(ec.dev, &ec.led_mic_mute);
    if (ret)
    return ret;
    return devm_led_classdev_register(ec.dev, &ec.led_spk_mute);
    }
    static const struct key_entry t14s_keymap[] = {
    T14S_EC_KEY_ENTRY(FN_4, KEY_SLEEP),
    T14S_EC_KEY_ENTRY(FN_N, KEY_VENDOR),
    T14S_EC_KEY_ENTRY(FN_F4, KEY_MICMUTE),
    T14S_EC_KEY_ENTRY(FN_F7, KEY_SWITCHVIDEOMODE),
    T14S_EC_KEY_ENTRY(FN_F8, KEY_PERFORMANCE),
    T14S_EC_KEY_ENTRY(FN_F10, KEY_SELECTIVE_SCREENSHOT),
    T14S_EC_KEY_ENTRY(FN_F11, KEY_LINK_PHONE),
    T14S_EC_KEY_ENTRY(FN_F12, KEY_BOOKMARKS),
    T14S_EC_KEY_ENTRY(FN_SPACE, KEY_KBDILLUMTOGGLE),
    T14S_EC_KEY_ENTRY(FN_ESC, KEY_FN_ESC),
    T14S_EC_KEY_ENTRY(FN_TAB, KEY_ZOOM),
    T14S_EC_KEY_ENTRY(FN_RIGHT_SHIFT, KEY_FN_RIGHT_SHIFT),
    T14S_EC_KEY_ENTRY(TP_DOUBLE_TAP, KEY_PROG4),
    { KE_END }
    };
#[no_mangle]
unsafe extern "C" fn t14s_input_probe(ec: *mut t14s_ec) -> c_int {
    static int t14s_input_probe(struct t14s_ec *ec)
    {
    int ret;
    ec.inputdev = devm_input_allocate_device(ec.dev);
    if (!ec.inputdev)
    return -ENOMEM;
    ec.inputdev.name = "ThinkPad Extra Buttons";
    ec.inputdev.phys = "thinkpad/input0";
    ec.inputdev.id.bustype = BUS_HOST;
    ec.inputdev.dev.parent = ec.dev;
    ret = sparse_keymap_setup(ec.inputdev, t14s_keymap, core::ptr::null_mut());
    if (ret)
    return ret;
    return input_register_device(ec.inputdev);
    }
#[no_mangle]
unsafe extern "C" fn t14s_ec_irq_handler(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t t14s_ec_irq_handler(int irq, void *data)
    {
    struct t14s_ec *ec = data;
    int ret;
    u8 val;
    ret = t14s_ec_read_evt(ec, &val);
    if (ret < 0) {
    dev_err(ec.dev, "Failed to read event\n");
    return IRQ_HANDLED;
    }
    switch (val) {
    case T14S_EC_EVT_NONE:
    break;
    case T14S_EC_EVT_KEY_FN_SPACE:
    t14s_kbd_bl_update(ec);
    fallthrough;
    case T14S_EC_EVT_KEY_FN_F4:
    case T14S_EC_EVT_KEY_FN_F7:
    case T14S_EC_EVT_KEY_FN_4:
    case T14S_EC_EVT_KEY_FN_F8:
    case T14S_EC_EVT_KEY_FN_F12:
    case T14S_EC_EVT_KEY_FN_TAB:
    case T14S_EC_EVT_KEY_FN_F10:
    case T14S_EC_EVT_KEY_FN_N:
    case T14S_EC_EVT_KEY_FN_F11:
    case T14S_EC_EVT_KEY_FN_ESC:
    case T14S_EC_EVT_KEY_FN_RIGHT_SHIFT:
    case T14S_EC_EVT_KEY_TP_DOUBLE_TAP:
    sparse_keymap_report_event(ec.inputdev,
    T14S_EC_KEY_EVT_OFFSET + val, 1, true);
    break;
    case T14S_EC_EVT_AC_CONNECTED:
    dev_dbg(ec.dev, "AC connected\n");
    break;
    case T14S_EC_EVT_AC_DISCONNECTED:
    dev_dbg(ec.dev, "AC disconnected\n");
    break;
    case T14S_EC_EVT_KEY_POWER:
    dev_dbg(ec.dev, "power button\n");
    break;
    case T14S_EC_EVT_LID_OPEN:
    dev_dbg(ec.dev, "LID open\n");
    break;
    case T14S_EC_EVT_LID_CLOSED:
    dev_dbg(ec.dev, "LID closed\n");
    break;
    case T14S_EC_EVT_THERMAL_TZ40:
    dev_dbg(ec.dev, "Thermal Zone 40 Status Change Event (CPU/GPU)\n");
    break;
    case T14S_EC_EVT_THERMAL_TZ42:
    dev_dbg(ec.dev, "Thermal Zone 42 Status Change Event (Battery)\n");
    break;
    case T14S_EC_EVT_THERMAL_TZ39:
    dev_dbg(ec.dev, "Thermal Zone 39 Status Change Event (CPU/GPU)\n");
    break;
    case T14S_EC_EVT_KEY_FN_G:
    dev_dbg(ec.dev, "FN + G - toggle double-tapping\n");
    break;
    case T14S_EC_EVT_KEY_FN_L:
    dev_dbg(ec.dev, "FN + L - low performance mode\n");
    break;
    case T14S_EC_EVT_KEY_FN_M:
    dev_dbg(ec.dev, "FN + M - medium performance mode\n");
    break;
    case T14S_EC_EVT_KEY_FN_H:
    dev_dbg(ec.dev, "FN + H - high performance mode\n");
    break;
    case T14S_EC_EVT_KEY_FN_T:
    dev_dbg(ec.dev, "FN + T - toggle intelligent cooling mode\n");
    break;
    case T14S_EC_EVT_KEY_FN_D:
    dev_dbg(ec.dev, "FN + D - toggle privacy guard mode\n");
    break;
    default:
    dev_info(ec.dev, "Unknown EC event: 0x%02x\n", val);
    break;
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn t14s_ec_probe(client: *mut i2c_client) -> c_int {
    static int t14s_ec_probe(struct i2c_client *client)
    {
    struct device *dev = &client.dev;
    struct t14s_ec *ec;
    int ret;
    ec = devm_kzalloc(dev, sizeof(*ec), GFP_KERNEL);
    if (!ec)
    return -ENOMEM;
    ec.dev = dev;
    i2c_set_clientdata(client, ec);
    ec.regmap = devm_regmap_init(dev, &t14s_ec_regmap_bus,
    ec, &t14s_ec_regmap_config);
    if (IS_ERR(ec.regmap))
    return dev_err_probe(dev, PTR_ERR(ec.regmap),
    "Failed to init regmap\n");
    ret = t14s_leds_probe(ec);
    if (ret < 0)
    return ret;
    ret = t14s_kbd_backlight_probe(ec);
    if (ret < 0)
    return ret;
    ret = t14s_kbd_audio_led_probe(ec);
    if (ret < 0)
    return ret;
    ret = t14s_input_probe(ec);
    if (ret < 0)
    return ret;
    ret = devm_request_threaded_irq(dev, client.irq, core::ptr::null_mut(),
    t14s_ec_irq_handler,
    IRQF_ONESHOT, dev_name(dev), ec);
    if (ret < 0)
    return dev_err_probe(dev, ret, "Failed to get IRQ\n");
//
// Disable wakeup support by default, because the driver currently does
// not support masking any events and the laptop should not wake up when
// the LID is closed.
//
    device_wakeup_disable(dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn t14s_ec_suspend(dev: *mut device) -> c_int {
    static int t14s_ec_suspend(struct device *dev)
    {
    struct t14s_ec *ec = dev_get_drvdata(dev);
    led_classdev_suspend(&ec.kbd_backlight);
    t14s_ec_write_sequence(ec, T14S_EC_REG_MODERN_STANDBY,
    T14S_EC_MODERN_STANDBY_ENTRY, 3);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn t14s_ec_resume(dev: *mut device) -> c_int {
    static int t14s_ec_resume(struct device *dev)
    {
    struct t14s_ec *ec = dev_get_drvdata(dev);
    t14s_ec_write_sequence(ec, T14S_EC_REG_MODERN_STANDBY,
    T14S_EC_MODERN_STANDBY_EXIT, 3);
    led_classdev_resume(&ec.kbd_backlight);
    return 0;
    }
    static const struct of_device_id t14s_ec_of_match[] = {
    { .compatible = "lenovo,thinkpad-t14s-ec" },
    {}
    };
    MODULE_DEVICE_TABLE(of, t14s_ec_of_match);
    static const struct i2c_device_id t14s_ec_i2c_id_table[] = {
    { .name = "thinkpad-t14s-ec" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, t14s_ec_i2c_id_table);
    static const struct dev_pm_ops t14s_ec_pm_ops = {
    SYSTEM_SLEEP_PM_OPS(t14s_ec_suspend, t14s_ec_resume)
    };
    static struct i2c_driver t14s_ec_i2c_driver = {
    .driver = {
    .name = "thinkpad-t14s-ec",
    .of_match_table = t14s_ec_of_match,
    .pm = &t14s_ec_pm_ops,
    },
    .probe = t14s_ec_probe,
    .id_table = t14s_ec_i2c_id_table,
    };
    module_i2c_driver(t14s_ec_i2c_driver);
    MODULE_AUTHOR("Sebastian Reichel <sre@kernel.org>");
    MODULE_DESCRIPTION("Lenovo Thinkpad T14s Embedded Controller");
    MODULE_LICENSE("GPL");
