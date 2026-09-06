//! Automatically rewritten from C to Rust
//! Source: drivers/platform/chrome/wilco_ec/keyboard_leds.c
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


// SPDX-License-Identifier: GPL-2.0
//
// Keyboard backlight LED driver for the Wilco Embedded Controller
//
// Copyright 2019 Google LLC
//
// Since the EC will never change the backlight level of its own accord,
// we don't need to implement a brightness_get() method.
//

pub const WILCO_EC_COMMAND_KBBL: c_uint = 0x75;

pub const WILCO_KBBL_DEFAULT_BRIGHTNESS: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wilco_keyboard_leds {
    pub ec: *mut wilco_ec_device,
    pub keyboard: led_classdev,
}

    enum wilco_kbbl_subcommand {
    WILCO_KBBL_SUBCMD_GET_FEATURES = 0x00,
    WILCO_KBBL_SUBCMD_GET_STATE    = 0x01,
    WILCO_KBBL_SUBCMD_SET_STATE    = 0x02,
    };
//
// struct wilco_keyboard_leds_msg - Message to/from EC for keyboard LED control.
// @command: Always WILCO_EC_COMMAND_KBBL.
// @status: Set by EC to 0 on success, 0xFF on failure.
// @subcmd: One of enum wilco_kbbl_subcommand.
// @reserved3: Should be 0.
// @mode: Bit flags for used mode, we want to use WILCO_KBBL_MODE_FLAG_PWM.
// @reserved5to8: Should be 0.
// @percent: Brightness in 0-100. Only meaningful in PWM mode.
// @reserved10to15: Should be 0.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wilco_keyboard_leds_msg {
    pub command: u8,
    pub status: u8,
    pub subcmd: u8,
    pub reserved3: u8,
    pub mode: u8,
    pub reserved5to8: [u8; 4],
    pub percent: u8,
    pub reserved10to15: [u8; 6],
    pub __packed: },
// Send a request, get a response, and check that the response is good.
    static int send_kbbl_msg(struct wilco_ec_device *ec,
    struct wilco_keyboard_leds_msg *request,
    struct wilco_keyboard_leds_msg *response)
    {
    pub msg: wilco_ec_message,
    pub ret: c_int,
    pub sizeof(msg)): memset(&msg, 0,,
    pub WILCO_EC_MSG_LEGACY: msg.type =,
    pub request: msg.request_data =,
    pub sizeof(*request): *mut msg.request_size =,
    pub response: msg.response_data =,
    pub sizeof(*response): *mut msg.response_size =,
    pub &msg): ret = wilco_ec_mailbox(ec,,
    if (ret < 0) {
    dev_err(ec.dev,
    pub ret): "Failed sending keyboard LEDs command: %d\n",,
    pub ret: return,
    }
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn set_kbbl(ec: *mut wilco_ec_device, brightness: enum led_brightness) -> c_int {
    static int set_kbbl(struct wilco_ec_device *ec, enum led_brightness brightness)
    {
    pub request: wilco_keyboard_leds_msg,
    pub response: wilco_keyboard_leds_msg,
    pub ret: c_int,
    pub sizeof(request)): memset(&request, 0,,
    pub WILCO_EC_COMMAND_KBBL: request.command =,
    pub WILCO_KBBL_SUBCMD_SET_STATE: request.subcmd =,
    pub WILCO_KBBL_MODE_FLAG_PWM: request.mode =,
    pub brightness: request.percent =,
    pub &response): ret = send_kbbl_msg(ec, &request,,
    if (ret < 0)
    pub ret: return,
    if (response.status) {
    dev_err(ec.dev,
    "EC reported failure sending keyboard LEDs command: %d\n",
    pub -EIO: return,
    }
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn kbbl_exist(ec: *mut wilco_ec_device, exists: *mut bool) -> c_int {
    static int kbbl_exist(struct wilco_ec_device *ec, bool *exists)
    {
    pub request: wilco_keyboard_leds_msg,
    pub response: wilco_keyboard_leds_msg,
    pub ret: c_int,
    pub sizeof(request)): memset(&request, 0,,
    pub WILCO_EC_COMMAND_KBBL: request.command =,
    pub WILCO_KBBL_SUBCMD_GET_FEATURES: request.subcmd =,
    pub &response): ret = send_kbbl_msg(ec, &request,,
    if (ret < 0)
    pub ret: return,
// exists = response.status != 0xFF;
    pub 0: return,
    }
//
// kbbl_init() - Initialize the state of the keyboard backlight.
// @ec: EC device to talk to.
//
// Gets the current brightness, ensuring that the BIOS already initialized the
// backlight to PWM mode. If not in PWM mode, then the current brightness is
// meaningless, so set the brightness to WILCO_KBBL_DEFAULT_BRIGHTNESS.
//
// Return: Final brightness of the keyboard, or negative error code on failure.
//
#[no_mangle]
unsafe extern "C" fn kbbl_init(ec: *mut wilco_ec_device) -> c_int {
    static int kbbl_init(struct wilco_ec_device *ec)
    {
    pub request: wilco_keyboard_leds_msg,
    pub response: wilco_keyboard_leds_msg,
    pub ret: c_int,
    pub sizeof(request)): memset(&request, 0,,
    pub WILCO_EC_COMMAND_KBBL: request.command =,
    pub WILCO_KBBL_SUBCMD_GET_STATE: request.subcmd =,
    pub &response): ret = send_kbbl_msg(ec, &request,,
    if (ret < 0)
    pub ret: return,
    if (response.status) {
    dev_err(ec.dev,
    "EC reported failure sending keyboard LEDs command: %d\n",
    pub -EIO: return,
    }
    if (response.mode & WILCO_KBBL_MODE_FLAG_PWM)
    pub response.percent: return,
    pub WILCO_KBBL_DEFAULT_BRIGHTNESS): ret = set_kbbl(ec,,
    if (ret < 0)
    pub ret: return,
    pub WILCO_KBBL_DEFAULT_BRIGHTNESS: return,
    }
    static int wilco_keyboard_leds_set(struct led_classdev *cdev,
    enum led_brightness brightness)
    {
    struct wilco_keyboard_leds *wkl =
    pub keyboard): container_of(cdev, struct wilco_keyboard_leds,,
    pub brightness): return set_kbbl(wkl->ec,,
    }
#[no_mangle]
pub unsafe extern "C" fn wilco_keyboard_leds_init(ec: *mut wilco_ec_device) -> c_int {
    int wilco_keyboard_leds_init(struct wilco_ec_device *ec)
    {
    pub wkl: *mut wilco_keyboard_leds,
    pub leds_exist: bool,
    pub ret: c_int,
    pub &leds_exist): ret = kbbl_exist(ec,,
    if (ret < 0) {
    dev_err(ec.dev,
    pub ret): "Failed checking keyboard LEDs support: %d\n",,
    pub ret: return,
    }
    if (!leds_exist)
    pub 0: return,
    pub GFP_KERNEL): *mut *mut wkl = devm_kzalloc(ec->dev, sizeof(wkl),,
    if (!wkl)
    pub -ENOMEM: return,
    pub ec: wkl->ec =,
    pub "platform::kbd_backlight": wkl->keyboard.name =,
    pub 100: wkl->keyboard.max_brightness =,
    pub LED_CORE_SUSPENDRESUME: wkl->keyboard.flags =,
    pub wilco_keyboard_leds_set: wkl->keyboard.brightness_set_blocking =,
    pub kbbl_init(ec): ret =,
    if (ret < 0)
    pub ret: return,
    pub ret: wkl->keyboard.brightness =,
    pub &wkl->keyboard): return devm_led_classdev_register(ec->dev,,
    }
