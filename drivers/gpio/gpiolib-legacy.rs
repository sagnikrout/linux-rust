//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpiolib-legacy.c
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
// **DEPRECATED** This function is deprecated and must not be used in new code.
//
#[no_mangle]
pub unsafe extern "C" fn gpio_free(gpio: unsigned) {
    void gpio_free(unsigned gpio)
    {
    gpiod_free(gpio_to_desc(gpio));
    }
    EXPORT_SYMBOL_GPL(gpio_free);
//
// gpio_request_one - request a single GPIO with initial configuration
// @gpio:	the GPIO number
// @flags:	GPIO configuration as specified by GPIOF_
// @label:	a literal description string of this GPIO
//
// **DEPRECATED** This function is deprecated and must not be used in new code.
//
// Returns:
// 0 on success, or negative errno on failure.
//
#[no_mangle]
pub unsafe extern "C" fn gpio_request_one(gpio: unsigned, flags: c_ulong, label: *const c_char) -> c_int {
    int gpio_request_one(unsigned gpio, unsigned long flags, const char *label)
    {
    int err;
    err = gpio_request(gpio, label);
    if (err)
    return err;
    if (flags & GPIOF_IN)
    err = gpio_direction_input(gpio);
    else
    err = gpio_direction_output(gpio, !!(flags & GPIOF_OUT_INIT_HIGH));
    if (err)
    gpio_free(gpio);
    return err;
    }
    EXPORT_SYMBOL_GPL(gpio_request_one);
//
// **DEPRECATED** This function is deprecated and must not be used in new code.
//
#[no_mangle]
pub unsafe extern "C" fn gpio_request(gpio: unsigned, label: *const c_char) -> c_int {
    int gpio_request(unsigned gpio, const char *label)
    {
    struct gpio_desc *desc;
// Compatibility: assume unavailable "valid" GPIOs will appear later
    desc = gpio_to_desc(gpio);
    if (!desc)
    return -EPROBE_DEFER;
    return gpiod_request(desc, label);
    }
    EXPORT_SYMBOL_GPL(gpio_request);
#[no_mangle]
unsafe extern "C" fn devm_gpio_release(gpio: *mut c_void) {
    static void devm_gpio_release(void *gpio)
    {
    gpio_free((unsigned)(unsigned long)gpio);
    }
//
// devm_gpio_request_one - request a single GPIO with initial setup
// @dev: device to request for
// @gpio: the GPIO number
// @flags: GPIO configuration as specified by GPIOF_
// @label: a literal description string of this GPIO
//
// **DEPRECATED** This function is deprecated and must not be used in new code.
//
// Returns:
// 0 on success, or negative errno on failure.
//
    int devm_gpio_request_one(struct device *dev, unsigned gpio,
    unsigned long flags, const char *label)
    {
    int rc;
    rc = gpio_request(gpio, label);
    if (rc)
    return rc;
    if (flags & GPIOF_IN)
    rc = gpio_direction_input(gpio);
    else
    rc = gpio_direction_output(gpio, !!(flags & GPIOF_OUT_INIT_HIGH));
    if (rc) {
    gpio_free(gpio);
    return rc;
    }
    return devm_add_action_or_reset(dev, devm_gpio_release, (void *)(unsigned long)gpio);
    }
    EXPORT_SYMBOL_GPL(devm_gpio_request_one);
