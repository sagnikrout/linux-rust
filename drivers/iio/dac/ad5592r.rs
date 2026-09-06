//! Automatically rewritten from C to Rust
//! Source: drivers/iio/dac/ad5592r.c
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
// AD5592R Digital <-> Analog converters driver
//
// Copyright 2015-2016 Analog Devices Inc.
// Author: Paul Cercueil <paul.cercueil@analog.com>
//

#[no_mangle]
unsafe extern "C" fn ad5592r_spi_wnop_r16(st: *mut ad5592r_state, buf: *mut __be16) -> c_int {
    static int ad5592r_spi_wnop_r16(struct ad5592r_state *st, __be16 *buf)
    {
    struct spi_device *spi = container_of(st.dev, struct spi_device, dev);
    struct spi_transfer t = {
    .tx_buf	= &st.spi_msg_nop,
    .rx_buf	= buf,
    .len = 2
    };
    st.spi_msg_nop = 0; /* NOP */
    return spi_sync_transfer(spi, &t, 1);
    }
#[no_mangle]
unsafe extern "C" fn ad5592r_write_dac(st: *mut ad5592r_state, chan: unsigned, value: u16) -> c_int {
    static int ad5592r_write_dac(struct ad5592r_state *st, unsigned chan, u16 value)
    {
    struct spi_device *spi = container_of(st.dev, struct spi_device, dev);
    st.spi_msg = cpu_to_be16(BIT(15) | (chan << 12) | value);
    return spi_write(spi, &st.spi_msg, sizeof(st.spi_msg));
    }
#[no_mangle]
unsafe extern "C" fn ad5592r_read_adc(st: *mut ad5592r_state, chan: unsigned, value: *mut u16) -> c_int {
    static int ad5592r_read_adc(struct ad5592r_state *st, unsigned chan, u16 *value)
    {
    struct spi_device *spi = container_of(st.dev, struct spi_device, dev);
    int ret;
    st.spi_msg = cpu_to_be16((AD5592R_REG_ADC_SEQ << 11) | BIT(chan));
    ret = spi_write(spi, &st.spi_msg, sizeof(st.spi_msg));
    if (ret)
    return ret;
//
// Invalid data:
// See Figure 40. Single-Channel ADC Conversion Sequence
//
    ret = ad5592r_spi_wnop_r16(st, &st.spi_msg);
    if (ret)
    return ret;
    ret = ad5592r_spi_wnop_r16(st, &st.spi_msg);
    if (ret)
    return ret;
// value = be16_to_cpu(st->spi_msg);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ad5592r_reg_write(st: *mut ad5592r_state, reg: u8, value: u16) -> c_int {
    static int ad5592r_reg_write(struct ad5592r_state *st, u8 reg, u16 value)
    {
    struct spi_device *spi = container_of(st.dev, struct spi_device, dev);
    st.spi_msg = cpu_to_be16((reg << 11) | value);
    return spi_write(spi, &st.spi_msg, sizeof(st.spi_msg));
    }
#[no_mangle]
unsafe extern "C" fn ad5592r_reg_read(st: *mut ad5592r_state, reg: u8, value: *mut u16) -> c_int {
    static int ad5592r_reg_read(struct ad5592r_state *st, u8 reg, u16 *value)
    {
    struct spi_device *spi = container_of(st.dev, struct spi_device, dev);
    int ret;
    st.spi_msg = cpu_to_be16((AD5592R_REG_LDAC << 11) |
    AD5592R_LDAC_READBACK_EN | (reg << 2));
    ret = spi_write(spi, &st.spi_msg, sizeof(st.spi_msg));
    if (ret)
    return ret;
    ret = ad5592r_spi_wnop_r16(st, &st.spi_msg);
    if (ret)
    return ret;
// value = be16_to_cpu(st->spi_msg);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ad5592r_gpio_read(st: *mut ad5592r_state, value: *mut u8) -> c_int {
    static int ad5592r_gpio_read(struct ad5592r_state *st, u8 *value)
    {
    int ret;
    ret = ad5592r_reg_write(st, AD5592R_REG_GPIO_IN_EN,
    AD5592R_GPIO_READBACK_EN | st.gpio_in);
    if (ret)
    return ret;
    ret = ad5592r_spi_wnop_r16(st, &st.spi_msg);
    if (ret)
    return ret;
// value = (u8) be16_to_cpu(st->spi_msg);
    return 0;
    }
    static const struct ad5592r_rw_ops ad5592r_rw_ops = {
    .write_dac = ad5592r_write_dac,
    .read_adc = ad5592r_read_adc,
    .reg_write = ad5592r_reg_write,
    .reg_read = ad5592r_reg_read,
    .gpio_read = ad5592r_gpio_read,
    };
#[no_mangle]
unsafe extern "C" fn ad5592r_spi_probe(spi: *mut spi_device) -> c_int {
    static int ad5592r_spi_probe(struct spi_device *spi)
    {
    const struct spi_device_id *id = spi_get_device_id(spi);
    return ad5592r_probe(&spi.dev, id.name, &ad5592r_rw_ops);
    }
#[no_mangle]
unsafe extern "C" fn ad5592r_spi_remove(spi: *mut spi_device) {
    static void ad5592r_spi_remove(struct spi_device *spi)
    {
    ad5592r_remove(&spi.dev);
    }
    static const struct spi_device_id ad5592r_spi_ids[] = {
    { .name = "ad5592r", },
    { }
    };
    MODULE_DEVICE_TABLE(spi, ad5592r_spi_ids);
    static const struct of_device_id ad5592r_of_match[] = {
    { .compatible = "adi,ad5592r", },
    { }
    };
    MODULE_DEVICE_TABLE(of, ad5592r_of_match);
    static const struct acpi_device_id ad5592r_acpi_match[] = {
    {"ADS5592", },
    { }
    };
    MODULE_DEVICE_TABLE(acpi, ad5592r_acpi_match);
    static struct spi_driver ad5592r_spi_driver = {
    .driver = {
    .name = "ad5592r",
    .of_match_table = ad5592r_of_match,
    .acpi_match_table = ad5592r_acpi_match,
    },
    .probe = ad5592r_spi_probe,
    .remove = ad5592r_spi_remove,
    .id_table = ad5592r_spi_ids,
    };
    module_spi_driver(ad5592r_spi_driver);
    MODULE_AUTHOR("Paul Cercueil <paul.cercueil@analog.com>");
    MODULE_DESCRIPTION("Analog Devices AD5592R multi-channel converters");
    MODULE_LICENSE("GPL v2");
    MODULE_IMPORT_NS("IIO_AD5592R");
