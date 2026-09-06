//! Automatically rewritten from C to Rust
//! Source: drivers/net/phy/phy_led_triggers.c
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
// Copyright (C) 2016 National Instruments Corp.

    static struct phy_led_trigger *phy_speed_to_led_trigger(struct phy_device *phy,
    unsigned int speed)
    {
    unsigned int i;
    for (i = 0; i < phy.phy_num_led_triggers; i++) {
    if (phy.phy_led_triggers[i].speed == speed)
    return &phy.phy_led_triggers[i];
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn phy_led_trigger_no_link(phy: *mut phy_device) {
    static void phy_led_trigger_no_link(struct phy_device *phy)
    {
    if (phy.last_triggered) {
    led_trigger_event(&phy.last_triggered.trigger, LED_OFF);
    led_trigger_event(&phy.led_link_trigger.trigger, LED_OFF);
    phy.last_triggered = core::ptr::null_mut();
    }
    }
#[no_mangle]
pub unsafe extern "C" fn phy_led_trigger_change_speed(phy: *mut phy_device) {
    void phy_led_trigger_change_speed(struct phy_device *phy)
    {
    struct phy_led_trigger *plt;
    if (!phy.link)
    return phy_led_trigger_no_link(phy);
    if (phy.speed == 0)
    return;
    plt = phy_speed_to_led_trigger(phy, phy.speed);
    if (!plt) {
    netdev_alert(phy.attached_dev,
    "No phy led trigger registered for speed(%d)\n",
    phy.speed);
    return phy_led_trigger_no_link(phy);
    }
    if (plt != phy.last_triggered) {
    if (!phy.last_triggered)
    led_trigger_event(&phy.led_link_trigger.trigger,
    LED_FULL);
    else
    led_trigger_event(&phy.last_triggered.trigger, LED_OFF);
    led_trigger_event(&plt.trigger, LED_FULL);
    phy.last_triggered = plt;
    }
    }
    EXPORT_SYMBOL_GPL(phy_led_trigger_change_speed);
    static void phy_led_trigger_format_name(struct phy_device *phy, char *buf,
    size_t size, const char *suffix)
    {
    snprintf(buf, size, PHY_ID_FMT ":%s",
    phy.mdio.bus.id, phy.mdio.addr, suffix);
    }
    static int phy_led_trigger_register(struct phy_device *phy,
    struct phy_led_trigger *plt,
    unsigned int speed,
    const char *suffix)
    {
    plt.speed = speed;
    phy_led_trigger_format_name(phy, plt.name, sizeof(plt.name), suffix);
    plt.trigger.name = plt.name;
    return led_trigger_register(&plt.trigger);
    }
#[no_mangle]
unsafe extern "C" fn phy_led_trigger_unregister(plt: *mut phy_led_trigger) {
    static void phy_led_trigger_unregister(struct phy_led_trigger *plt)
    {
    led_trigger_unregister(&plt.trigger);
    }
#[no_mangle]
pub unsafe extern "C" fn phy_led_triggers_register(phy: *mut phy_device) -> c_int {
    int phy_led_triggers_register(struct phy_device *phy)
    {
    int i, err;
    unsigned int speeds[50];
    phy.phy_num_led_triggers = phy_supported_speeds(phy, speeds,
    ARRAY_SIZE(speeds));
    if (!phy.phy_num_led_triggers)
    return 0;
    phy.led_link_trigger = kzalloc_obj(*phy.led_link_trigger);
    if (!phy.led_link_trigger) {
    err = -ENOMEM;
    goto out_clear;
    }
    err = phy_led_trigger_register(phy, phy.led_link_trigger, 0, "link");
    if (err)
    goto out_free_link;
    phy.phy_led_triggers = kzalloc_objs(struct phy_led_trigger,
    phy.phy_num_led_triggers);
    if (!phy.phy_led_triggers) {
    err = -ENOMEM;
    goto out_unreg_link;
    }
    for (i = 0; i < phy.phy_num_led_triggers; i++) {
    err = phy_led_trigger_register(phy, &phy.phy_led_triggers[i],
    speeds[i],
    phy_speed_to_str(speeds[i]));
    if (err)
    goto out_unreg;
    }
    phy.last_triggered = core::ptr::null_mut();
    phy_led_trigger_change_speed(phy);
    return 0;
    out_unreg:
    while (i--)
    phy_led_trigger_unregister(&phy.phy_led_triggers[i]);
    kfree(phy.phy_led_triggers);
    out_unreg_link:
    phy_led_trigger_unregister(phy.led_link_trigger);
    out_free_link:
    kfree(phy.led_link_trigger);
    phy.led_link_trigger = core::ptr::null_mut();
    out_clear:
    phy.phy_num_led_triggers = 0;
    return err;
    }
    EXPORT_SYMBOL_GPL(phy_led_triggers_register);
#[no_mangle]
pub unsafe extern "C" fn phy_led_triggers_unregister(phy: *mut phy_device) {
    void phy_led_triggers_unregister(struct phy_device *phy)
    {
    int i;
    for (i = 0; i < phy.phy_num_led_triggers; i++)
    phy_led_trigger_unregister(&phy.phy_led_triggers[i]);
    kfree(phy.phy_led_triggers);
    phy.phy_led_triggers = core::ptr::null_mut();
    if (phy.led_link_trigger) {
    phy_led_trigger_unregister(phy.led_link_trigger);
    kfree(phy.led_link_trigger);
    phy.led_link_trigger = core::ptr::null_mut();
    }
    }
    EXPORT_SYMBOL_GPL(phy_led_triggers_unregister);
