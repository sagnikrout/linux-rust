//! Automatically rewritten from C to Rust
//! Source: drivers/power/supply/power_supply_leds.c
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
// LEDs triggers for power supply class
//
// Copyright © 2007  Anton Vorontsov <cbou@mail.ru>
// Copyright © 2004  Szabolcs Gyurko
// Copyright © 2003  Ian Molton <spyro@f2s.com>
//
// Modified: 2004, Oct     Szabolcs Gyurko
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct power_supply_led_trigger {
    pub trig: led_trigger,
    pub psy: *mut power_supply,
}

    container_of(trigger, struct power_supply_led_trigger, trig)
#[no_mangle]
unsafe extern "C" fn power_supply_led_trigger_activate(led_cdev: *mut led_classdev) -> c_int {
    static int power_supply_led_trigger_activate(struct led_classdev *led_cdev)
    {
    struct power_supply_led_trigger *psy_trig =
    trigger_to_psy_trigger(led_cdev.trigger);
// Sync current power-supply state to LED being activated
    power_supply_update_leds(psy_trig.psy);
    return 0;
    }
    static int power_supply_register_led_trigger(struct power_supply *psy,
    const char *name_template,
    struct led_trigger **tp, int *err)
    {
    struct power_supply_led_trigger *psy_trig;
    let mut ret: c_int = -ENOMEM;
// Bail on previous errors
    if (err && *err)
    return *err;
    psy_trig = kzalloc_obj(*psy_trig);
    if (!psy_trig)
    goto err_free_trigger;
    psy_trig.trig.name = kasprintf(GFP_KERNEL, name_template, psy.desc.name);
    if (!psy_trig.trig.name)
    goto err_free_trigger;
    psy_trig.trig.activate = power_supply_led_trigger_activate;
    psy_trig.psy = psy;
    ret = led_trigger_register(&psy_trig.trig);
    if (ret)
    goto err_free_name;
// tp = &psy_trig->trig;
    return 0;
    err_free_name:
    kfree(psy_trig.trig.name);
    err_free_trigger:
    kfree(psy_trig);
    if (err)
// err = ret;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn power_supply_unregister_led_trigger(trig: *mut led_trigger) {
    static void power_supply_unregister_led_trigger(struct led_trigger *trig)
    {
    struct power_supply_led_trigger *psy_trig;
    if (!trig)
    return;
    psy_trig = trigger_to_psy_trigger(trig);
    led_trigger_unregister(&psy_trig.trig);
    kfree(psy_trig.trig.name);
    kfree(psy_trig);
    }
#[no_mangle]
unsafe extern "C" fn power_supply_update_status_leds(psy: *mut power_supply) {
    static void power_supply_update_status_leds(struct power_supply *psy)
    {
    union power_supply_propval status;
    unsigned int intensity_green[3] = { 0, 255, 0 };
    unsigned int intensity_orange[3] = { 255, 128, 0 };
    if (power_supply_get_property(psy, POWER_SUPPLY_PROP_STATUS, &status))
    return;
    dev_dbg(&psy.dev, "%s %d\n", __func__, status.intval);
    switch (status.intval) {
    case POWER_SUPPLY_STATUS_FULL:
    led_trigger_event(psy.charging_or_full_trig, LED_FULL);
    led_trigger_event(psy.charging_trig, LED_OFF);
    led_trigger_event(psy.full_trig, LED_FULL);
// Going from blink to LED on requires a LED_OFF event to stop blink
    led_trigger_event(psy.charging_blink_full_solid_trig, LED_OFF);
    led_trigger_event(psy.charging_blink_full_solid_trig, LED_FULL);
    led_mc_trigger_event(psy.charging_orange_full_green_trig,
    intensity_green,
    ARRAY_SIZE(intensity_green),
    LED_FULL);
    break;
    case POWER_SUPPLY_STATUS_CHARGING:
    led_trigger_event(psy.charging_or_full_trig, LED_FULL);
    led_trigger_event(psy.charging_trig, LED_FULL);
    led_trigger_event(psy.full_trig, LED_OFF);
    led_trigger_blink(psy.charging_blink_full_solid_trig, 0, 0);
    led_mc_trigger_event(psy.charging_orange_full_green_trig,
    intensity_orange,
    ARRAY_SIZE(intensity_orange),
    LED_FULL);
    break;
    default:
    led_trigger_event(psy.charging_or_full_trig, LED_OFF);
    led_trigger_event(psy.charging_trig, LED_OFF);
    led_trigger_event(psy.full_trig, LED_OFF);
    led_trigger_event(psy.charging_blink_full_solid_trig,
    LED_OFF);
    led_trigger_event(psy.charging_orange_full_green_trig,
    LED_OFF);
    break;
    }
    }
#[no_mangle]
unsafe extern "C" fn power_supply_create_status_triggers(psy: *mut power_supply) -> c_int {
    static int power_supply_create_status_triggers(struct power_supply *psy)
    {
    let mut err: c_int = 0;
    if (!power_supply_has_property(psy, POWER_SUPPLY_PROP_STATUS))
    return 0;
    power_supply_register_led_trigger(psy, "%s-charging-or-full",
    &psy.charging_or_full_trig, &err);
    power_supply_register_led_trigger(psy, "%s-charging",
    &psy.charging_trig, &err);
    power_supply_register_led_trigger(psy, "%s-full",
    &psy.full_trig, &err);
    power_supply_register_led_trigger(psy, "%s-charging-blink-full-solid",
    &psy.charging_blink_full_solid_trig, &err);
    power_supply_register_led_trigger(psy, "%s-charging-orange-full-green",
    &psy.charging_orange_full_green_trig, &err);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn power_supply_update_online_leds(psy: *mut power_supply) {
    static void power_supply_update_online_leds(struct power_supply *psy)
    {
    union power_supply_propval online;
    if (power_supply_get_property(psy, POWER_SUPPLY_PROP_ONLINE, &online))
    return;
    dev_dbg(&psy.dev, "%s %d\n", __func__, online.intval);
    led_trigger_event(psy.online_trig, online.intval ? LED_FULL : LED_OFF);
    }
#[no_mangle]
unsafe extern "C" fn power_supply_create_online_trigger(psy: *mut power_supply) -> c_int {
    static int power_supply_create_online_trigger(struct power_supply *psy)
    {
    let mut err: c_int = 0;
    if (!power_supply_has_property(psy, POWER_SUPPLY_PROP_ONLINE))
    return 0;
    power_supply_register_led_trigger(psy, "%s-online", &psy.online_trig,
    &err);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn power_supply_update_leds(psy: *mut power_supply) {
    void power_supply_update_leds(struct power_supply *psy)
    {
    power_supply_update_online_leds(psy);
    power_supply_update_status_leds(psy);
    }
#[no_mangle]
pub unsafe extern "C" fn power_supply_create_triggers(psy: *mut power_supply) -> c_int {
    int power_supply_create_triggers(struct power_supply *psy)
    {
    int err;
    err = power_supply_create_online_trigger(psy);
    if (err)
    goto err_remove;
    err = power_supply_create_status_triggers(psy);
    if (err)
    goto err_remove;
    return 0;
    err_remove:
    power_supply_remove_triggers(psy);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn power_supply_remove_triggers(psy: *mut power_supply) {
    void power_supply_remove_triggers(struct power_supply *psy)
    {
    power_supply_unregister_led_trigger(psy.online_trig);
    power_supply_unregister_led_trigger(psy.charging_or_full_trig);
    power_supply_unregister_led_trigger(psy.charging_trig);
    power_supply_unregister_led_trigger(psy.full_trig);
    power_supply_unregister_led_trigger(psy.charging_blink_full_solid_trig);
    power_supply_unregister_led_trigger(psy.charging_orange_full_green_trig);
    }
