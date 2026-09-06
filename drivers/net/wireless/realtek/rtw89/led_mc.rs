//! Automatically rewritten from C to Rust
//! Source: drivers/net/wireless/realtek/rtw89/led_mc.c
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
// Copyright(c) 2026 Realtek Corporation
//

    static int rtw89_led_mc_brightness_set(struct led_classdev *led,
    enum led_brightness brightness)
    {
    struct led_classdev_mc *mcdev = lcdev_to_mccdev(led);
    struct rtw89_led *rtw_led = container_of(mcdev, struct rtw89_led, led_mc);
    struct rtw89_dev *rtwdev = container_of(rtw_led, struct rtw89_dev, led);
    const struct rtw89_led_desc *desc = rtw_led.desc;
    const struct rtw89_led_gpio_entry *e;
    unsigned int i;
    led_mc_calc_color_components(mcdev, brightness);
    wiphy_lock(rtwdev.hw.wiphy);
    for (i = 0; i < mcdev.num_colors; i++) {
    e = &desc.gpios[i];
    if (rtw_led.brightness_cache[i] == mcdev.subled_info[i].brightness) {
    rtw89_debug(rtwdev, RTW89_DBG_LED,
    "led_mc_set: pin=%u skip (no change)\n", e.pin);
    continue;
    }
    rtw89_debug(rtwdev, RTW89_DBG_LED, "led_mc_set: pin=%u brightness=%u\n",
    e.pin, mcdev.subled_info[i].brightness);
    rtw89_led_gpio_set(rtwdev, e, mcdev.subled_info[i].brightness);
    rtw_led.brightness_cache[i] = mcdev.subled_info[i].brightness;
    }
    wiphy_unlock(rtwdev.hw.wiphy);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn rtw89_led_mc_init(rtwdev: *mut rtw89_dev, desc: *const rtw89_led_desc) -> c_int {
    int rtw89_led_mc_init(struct rtw89_dev *rtwdev, const struct rtw89_led_desc *desc)
    {
    struct rtw89_led *rtw_led = &rtwdev.led;
    struct led_classdev_mc *led_mc = &rtw_led.led_mc;
    int i, ret;
    led_mc.num_colors = desc.n_gpio;
    led_mc.subled_info = rtw_led.subled;
    for (i = 0; i < desc.n_gpio; i++) {
    const struct rtw89_led_gpio_entry *e = &desc.gpios[i];
    rtw_led.subled[i].color_index = e.color;
    rtw_led.subled[i].intensity = e.intensity;
    rtw_led.subled[i].brightness = 0;
    rtw_led.subled[i].channel = i;
    rtw89_led_gpio_config(rtwdev, e);
    }
    snprintf(rtw_led.name, sizeof(rtw_led.name), "rtw89-%s-multicolor",
    wiphy_name(rtwdev.hw.wiphy));
    led_mc.led_cdev.name = rtw_led.name;
    led_mc.led_cdev.brightness_set_blocking = rtw89_led_mc_brightness_set;
    led_mc.led_cdev.max_brightness = LED_ON;
    led_mc.led_cdev.default_trigger = ieee80211_get_assoc_led_name(rtwdev.hw);
    ret = led_classdev_multicolor_register(rtwdev.dev, led_mc);
    if (ret)
    rtw89_warn(rtwdev, "failed to register multicolor LED, ret=%d\n", ret);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn rtw89_led_mc_deinit(rtwdev: *mut rtw89_dev) {
    void rtw89_led_mc_deinit(struct rtw89_dev *rtwdev)
    {
    struct rtw89_led *rtw_led = &rtwdev.led;
    led_classdev_multicolor_unregister(&rtw_led.led_mc);
    }
