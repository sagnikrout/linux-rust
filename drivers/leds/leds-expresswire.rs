//! Automatically rewritten from C to Rust
//! Source: drivers/leds/leds-expresswire.c
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
// Shared library for Kinetic's ExpressWire protocol.
// This protocol works by pulsing the ExpressWire IC's control GPIO.
// ktd2692 and ktd2801 are known to use this protocol.
//

#[no_mangle]
pub unsafe extern "C" fn expresswire_power_off(props: *mut expresswire_common_props) {
    void expresswire_power_off(struct expresswire_common_props *props)
    {
    gpiod_set_value_cansleep(props.ctrl_gpio, 0);
    fsleep(props.timing.poweroff_us);
    }
    EXPORT_SYMBOL_NS_GPL(expresswire_power_off, "EXPRESSWIRE");
#[no_mangle]
pub unsafe extern "C" fn expresswire_enable(props: *mut expresswire_common_props) {
    void expresswire_enable(struct expresswire_common_props *props)
    {
    unsigned long flags;
    local_irq_save(flags);
    gpiod_set_value(props.ctrl_gpio, 1);
    udelay(props.timing.detect_delay_us);
    gpiod_set_value(props.ctrl_gpio, 0);
    udelay(props.timing.detect_us);
    gpiod_set_value(props.ctrl_gpio, 1);
    local_irq_restore(flags);
    }
    EXPORT_SYMBOL_NS_GPL(expresswire_enable, "EXPRESSWIRE");
#[no_mangle]
unsafe extern "C" fn expresswire_start(props: *mut expresswire_common_props) {
    static void expresswire_start(struct expresswire_common_props *props)
    {
    gpiod_set_value(props.ctrl_gpio, 1);
    udelay(props.timing.data_start_us);
    }
#[no_mangle]
unsafe extern "C" fn expresswire_end(props: *mut expresswire_common_props) {
    static void expresswire_end(struct expresswire_common_props *props)
    {
    gpiod_set_value(props.ctrl_gpio, 0);
    udelay(props.timing.end_of_data_low_us);
    gpiod_set_value(props.ctrl_gpio, 1);
    udelay(props.timing.end_of_data_high_us);
    }
#[no_mangle]
unsafe extern "C" fn expresswire_set_bit(props: *mut expresswire_common_props, bit: bool) {
    static void expresswire_set_bit(struct expresswire_common_props *props, bool bit)
    {
    if (bit) {
    gpiod_set_value(props.ctrl_gpio, 0);
    udelay(props.timing.short_bitset_us);
    gpiod_set_value(props.ctrl_gpio, 1);
    udelay(props.timing.long_bitset_us);
    } else {
    gpiod_set_value(props.ctrl_gpio, 0);
    udelay(props.timing.long_bitset_us);
    gpiod_set_value(props.ctrl_gpio, 1);
    udelay(props.timing.short_bitset_us);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn expresswire_write_u8(props: *mut expresswire_common_props, val: u8) {
    void expresswire_write_u8(struct expresswire_common_props *props, u8 val)
    {
    unsigned long flags;
    local_irq_save(flags);
    expresswire_start(props);
    for (int i = 7; i >= 0; i--)
    expresswire_set_bit(props, val & BIT(i));
    expresswire_end(props);
    local_irq_restore(flags);
    }
    EXPORT_SYMBOL_NS_GPL(expresswire_write_u8, "EXPRESSWIRE");
