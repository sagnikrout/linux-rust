//! Automatically rewritten from C to Rust
//! Source: drivers/iio/imu/adis_trigger.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Common library for ADIS16XXX devices
//
// Copyright 2012 Analog Devices Inc.
// Author: Lars-Peter Clausen <lars@metafoo.de>
//

#[no_mangle]
unsafe extern "C" fn adis_data_rdy_trigger_set_state(trig: *mut iio_trigger, state: bool) -> c_int {
    static int adis_data_rdy_trigger_set_state(struct iio_trigger *trig, bool state)
    {
    struct adis *adis = iio_trigger_get_drvdata(trig);
    return adis_enable_irq(adis, state);
    }
    static const struct iio_trigger_ops adis_trigger_ops = {
    .set_trigger_state = &adis_data_rdy_trigger_set_state,
    };
#[no_mangle]
unsafe extern "C" fn adis_validate_irq_flag(adis: *mut adis) -> c_int {
    static int adis_validate_irq_flag(struct adis *adis)
    {
    let mut direction: c_ulong = adis.irq_flag & IRQF_TRIGGER_MASK;
// We cannot mask the interrupt so ensure it's not enabled at request
    if (adis.data.unmasked_drdy)
    adis.irq_flag |= IRQF_NO_AUTOEN;
//
// Typically adis devices without FIFO have data ready either on the
// rising edge or on the falling edge of the data ready pin.
// IMU devices with FIFO support have the watermark pin level driven
// either high or low when the FIFO is filled with the desired number
// of samples.
// It defaults to IRQF_TRIGGER_RISING for backward compatibility with
// devices that don't support changing the pin polarity.
//
    if (direction == IRQF_TRIGGER_NONE) {
    adis.irq_flag |= IRQF_TRIGGER_RISING;
    return 0;
    } else if (direction != IRQF_TRIGGER_RISING &&
    direction != IRQF_TRIGGER_FALLING && !adis.data.has_fifo) {
    dev_err(&adis.spi.dev, "Invalid IRQ mask: %08lx\n",
    adis.irq_flag);
    return -EINVAL;
    } else if (direction != IRQF_TRIGGER_HIGH &&
    direction != IRQF_TRIGGER_LOW && adis.data.has_fifo) {
    dev_err(&adis.spi.dev, "Invalid IRQ mask: %08lx\n",
    adis.irq_flag);
    return -EINVAL;
    }
    return 0;
    }
//
// devm_adis_probe_trigger() - Sets up trigger for a managed adis device
// @adis: The adis device
// @indio_dev: The IIO device
//
// Returns 0 on success or a negative error code
//
#[no_mangle]
pub unsafe extern "C" fn devm_adis_probe_trigger(adis: *mut adis, indio_dev: *mut iio_dev) -> c_int {
    int devm_adis_probe_trigger(struct adis *adis, struct iio_dev *indio_dev)
    {
    int ret;
    adis.trig = devm_iio_trigger_alloc(&adis.spi.dev, "%s-dev%d",
    indio_dev.name,
    iio_device_id(indio_dev));
    if (!adis.trig)
    return -ENOMEM;
    adis.trig.ops = &adis_trigger_ops;
    iio_trigger_set_drvdata(adis.trig, adis);
    ret = adis_validate_irq_flag(adis);
    if (ret)
    return ret;
    if (adis.data.has_fifo)
    ret = devm_request_threaded_irq(&adis.spi.dev, adis.spi.irq,
    core::ptr::null_mut(),
    &iio_trigger_generic_data_rdy_poll,
    adis.irq_flag | IRQF_ONESHOT,
    indio_dev.name,
    adis.trig);
    else
    ret = devm_request_irq(&adis.spi.dev, adis.spi.irq,
    &iio_trigger_generic_data_rdy_poll,
    adis.irq_flag | IRQF_NO_THREAD,
    indio_dev.name,
    adis.trig);
    if (ret)
    return ret;
    return devm_iio_trigger_register(&adis.spi.dev, adis.trig);
    }
    EXPORT_SYMBOL_NS_GPL(devm_adis_probe_trigger, "IIO_ADISLIB");
