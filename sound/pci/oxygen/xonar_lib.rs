//! Automatically rewritten from C to Rust
//! Source: sound/pci/oxygen/xonar_lib.c
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
// helper functions for Asus Xonar cards
//
// Copyright (c) Clemens Ladisch <clemens@ladisch.de>
//

pub const GPIO_CS53x1_M_MASK: c_uint = 0x000c;
pub const GPIO_CS53x1_M_SINGLE: c_uint = 0x0000;
pub const GPIO_CS53x1_M_DOUBLE: c_uint = 0x0004;
pub const GPIO_CS53x1_M_QUAD: c_uint = 0x0008;
#[no_mangle]
pub unsafe extern "C" fn xonar_enable_output(chip: *mut oxygen) {
    void xonar_enable_output(struct oxygen *chip)
    {
    struct xonar_generic *data = chip.model_data;
    oxygen_set_bits16(chip, OXYGEN_GPIO_CONTROL, data.output_enable_bit);
    msleep(data.anti_pop_delay);
    oxygen_set_bits16(chip, OXYGEN_GPIO_DATA, data.output_enable_bit);
    }
#[no_mangle]
pub unsafe extern "C" fn xonar_disable_output(chip: *mut oxygen) {
    void xonar_disable_output(struct oxygen *chip)
    {
    struct xonar_generic *data = chip.model_data;
    oxygen_clear_bits16(chip, OXYGEN_GPIO_DATA, data.output_enable_bit);
    }
#[no_mangle]
unsafe extern "C" fn xonar_ext_power_gpio_changed(chip: *mut oxygen) {
    static void xonar_ext_power_gpio_changed(struct oxygen *chip)
    {
    struct xonar_generic *data = chip.model_data;
    u8 has_power;
    has_power = !!(oxygen_read8(chip, data.ext_power_reg)
    & data.ext_power_bit);
    if (has_power != data.has_power) {
    data.has_power = has_power;
    if (has_power) {
    dev_notice(chip.card.dev, "power restored\n");
    } else {
    dev_crit(chip.card.dev,
    "Hey! Don't unplug the power cable!\n");
// TODO: stop PCMs
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn xonar_init_ext_power(chip: *mut oxygen) {
    void xonar_init_ext_power(struct oxygen *chip)
    {
    struct xonar_generic *data = chip.model_data;
    oxygen_set_bits8(chip, data.ext_power_int_reg,
    data.ext_power_bit);
    chip.interrupt_mask |= OXYGEN_INT_GPIO;
    chip.model.gpio_changed = xonar_ext_power_gpio_changed;
    data.has_power = !!(oxygen_read8(chip, data.ext_power_reg)
    & data.ext_power_bit);
    }
#[no_mangle]
pub unsafe extern "C" fn xonar_init_cs53x1(chip: *mut oxygen) {
    void xonar_init_cs53x1(struct oxygen *chip)
    {
    oxygen_set_bits16(chip, OXYGEN_GPIO_CONTROL, GPIO_CS53x1_M_MASK);
    oxygen_write16_masked(chip, OXYGEN_GPIO_DATA,
    GPIO_CS53x1_M_SINGLE, GPIO_CS53x1_M_MASK);
    }
    void xonar_set_cs53x1_params(struct oxygen *chip,
    struct snd_pcm_hw_params *params)
    {
    unsigned int value;
    if (params_rate(params) <= 54000)
    value = GPIO_CS53x1_M_SINGLE;
#[no_mangle]
pub unsafe extern "C" fn if(108000: params_rate(params) <=) -> else {
    else if (params_rate(params) <= 108000)
    value = GPIO_CS53x1_M_DOUBLE;
    else
    value = GPIO_CS53x1_M_QUAD;
    oxygen_write16_masked(chip, OXYGEN_GPIO_DATA,
    value, GPIO_CS53x1_M_MASK);
    }
    int xonar_gpio_bit_switch_get(struct snd_kcontrol *ctl,
    struct snd_ctl_elem_value *value)
    {
    struct oxygen *chip = ctl.private_data;
    let mut bit: u16 = ctl.private_value;
    let mut invert: bool = ctl.private_value & XONAR_GPIO_BIT_INVERT;
    value.value.integer.value[0] =
    !!(oxygen_read16(chip, OXYGEN_GPIO_DATA) & bit) ^ invert;
    return 0;
    }
    int xonar_gpio_bit_switch_put(struct snd_kcontrol *ctl,
    struct snd_ctl_elem_value *value)
    {
    struct oxygen *chip = ctl.private_data;
    let mut bit: u16 = ctl.private_value;
    let mut invert: bool = ctl.private_value & XONAR_GPIO_BIT_INVERT;
    u16 old_bits, new_bits;
    int changed;
    guard(spinlock_irq)(&chip.reg_lock);
    old_bits = oxygen_read16(chip, OXYGEN_GPIO_DATA);
    if (!!value.value.integer.value[0] ^ invert)
    new_bits = old_bits | bit;
    else
    new_bits = old_bits & ~bit;
    changed = new_bits != old_bits;
    if (changed)
    oxygen_write16(chip, OXYGEN_GPIO_DATA, new_bits);
    return changed;
    }
