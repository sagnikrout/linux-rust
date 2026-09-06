//! Automatically rewritten from C to Rust
//! Source: drivers/hid/hid-input-test.c
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
//
// HID to Linux Input mapping
//
// Copyright (c) 2022 José Expósito <jose.exposito89@gmail.com>
//

#[no_mangle]
unsafe extern "C" fn hid_test_input_update_battery_charge_status(test: *mut kunit) {
    static void hid_test_input_update_battery_charge_status(struct kunit *test)
    {
    struct hid_battery *bat;
    bool handled;
    bat = kunit_kzalloc(test, sizeof(*bat), GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, bat);
    handled = hidinput_update_battery_charge_status(bat, HID_DG_HEIGHT, 0);
    KUNIT_EXPECT_FALSE(test, handled);
    KUNIT_EXPECT_EQ(test, bat.charge_status, POWER_SUPPLY_STATUS_UNKNOWN);
    handled = hidinput_update_battery_charge_status(bat, HID_BAT_CHARGING, 0);
    KUNIT_EXPECT_TRUE(test, handled);
    KUNIT_EXPECT_EQ(test, bat.charge_status, POWER_SUPPLY_STATUS_DISCHARGING);
    handled = hidinput_update_battery_charge_status(bat, HID_BAT_CHARGING, 1);
    KUNIT_EXPECT_TRUE(test, handled);
    KUNIT_EXPECT_EQ(test, bat.charge_status, POWER_SUPPLY_STATUS_CHARGING);
    }
#[no_mangle]
unsafe extern "C" fn hid_test_input_get_battery_property(test: *mut kunit) {
    static void hid_test_input_get_battery_property(struct kunit *test)
    {
    struct power_supply *psy;
    struct hid_battery *bat;
    struct hid_device *dev;
    union power_supply_propval val;
    int ret;
    dev = kunit_kzalloc(test, sizeof(*dev), GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, dev);
    bat = kunit_kzalloc(test, sizeof(*bat), GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, bat);
    bat.dev = dev;
    bat.avoid_query = true;
    psy = kunit_kzalloc(test, sizeof(*psy), GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, psy);
    psy.drv_data = bat;
    bat.status = HID_BATTERY_UNKNOWN;
    bat.charge_status = POWER_SUPPLY_STATUS_CHARGING;
    ret = hidinput_get_battery_property(psy, POWER_SUPPLY_PROP_STATUS, &val);
    KUNIT_EXPECT_EQ(test, ret, 0);
    KUNIT_EXPECT_EQ(test, val.intval, POWER_SUPPLY_STATUS_UNKNOWN);
    bat.status = HID_BATTERY_REPORTED;
    bat.charge_status = POWER_SUPPLY_STATUS_CHARGING;
    ret = hidinput_get_battery_property(psy, POWER_SUPPLY_PROP_STATUS, &val);
    KUNIT_EXPECT_EQ(test, ret, 0);
    KUNIT_EXPECT_EQ(test, val.intval, POWER_SUPPLY_STATUS_CHARGING);
    bat.status = HID_BATTERY_REPORTED;
    bat.charge_status = POWER_SUPPLY_STATUS_DISCHARGING;
    ret = hidinput_get_battery_property(psy, POWER_SUPPLY_PROP_STATUS, &val);
    KUNIT_EXPECT_EQ(test, ret, 0);
    KUNIT_EXPECT_EQ(test, val.intval, POWER_SUPPLY_STATUS_DISCHARGING);
    }
    static struct kunit_case hid_input_tests[] = {
    KUNIT_CASE(hid_test_input_update_battery_charge_status),
    KUNIT_CASE(hid_test_input_get_battery_property),
    { }
    };
    static struct kunit_suite hid_input_test_suite = {
    .name = "hid_input",
    .test_cases = hid_input_tests,
    };
    kunit_test_suite(hid_input_test_suite);
    MODULE_DESCRIPTION("HID input KUnit tests");
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("José Expósito <jose.exposito89@gmail.com>");
