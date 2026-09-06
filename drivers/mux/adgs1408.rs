//! Automatically rewritten from C to Rust
//! Source: drivers/mux/adgs1408.c
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
// ADGS1408/ADGS1409 SPI MUX driver
//
// Copyright 2018 Analog Devices Inc.
//

    enum adgs1408_chip_id {
    ADGS1408 = 1,
    ADGS1409,
    };
    static int adgs1408_spi_reg_write(struct spi_device *spi,
    u8 reg_addr, u8 reg_data)
    {
    u8 tx_buf[2];
    tx_buf[0] = reg_addr;
    tx_buf[1] = reg_data;
    return spi_write_then_read(spi, tx_buf, sizeof(tx_buf), core::ptr::null_mut(), 0);
    }
#[no_mangle]
unsafe extern "C" fn adgs1408_set(mux: *mut mux_control, state: c_int) -> c_int {
    static int adgs1408_set(struct mux_control *mux, int state)
    {
    struct spi_device *spi = to_spi_device(mux.chip.dev.parent);
    u8 reg;
    if (state == MUX_IDLE_DISCONNECT)
    reg = ADGS1408_DISABLE;
    else
    reg = ADGS1408_MUX(state);
    return adgs1408_spi_reg_write(spi, ADGS1408_SW_DATA, reg);
    }
    static const struct mux_control_ops adgs1408_ops = {
    .set = adgs1408_set,
    };
#[no_mangle]
unsafe extern "C" fn adgs1408_probe(spi: *mut spi_device) -> c_int {
    static int adgs1408_probe(struct spi_device *spi)
    {
    struct device *dev = &spi.dev;
    enum adgs1408_chip_id chip_id;
    struct mux_chip *mux_chip;
    struct mux_control *mux;
    s32 idle_state;
    int ret;
    chip_id = (kernel_ulong_t)spi_get_device_match_data(spi);
    mux_chip = devm_mux_chip_alloc(dev, 1, 0);
    if (IS_ERR(mux_chip))
    return PTR_ERR(mux_chip);
    mux_chip.ops = &adgs1408_ops;
    ret = adgs1408_spi_reg_write(spi, ADGS1408_SW_DATA, ADGS1408_DISABLE);
    if (ret < 0)
    return ret;
    ret = device_property_read_u32(dev, "idle-state", (u32 *)&idle_state);
    if (ret < 0)
    idle_state = MUX_IDLE_AS_IS;
    mux = mux_chip.mux;
    if (chip_id == ADGS1408)
    mux.states = 8;
    else
    mux.states = 4;
    switch (idle_state) {
    case MUX_IDLE_DISCONNECT:
    case MUX_IDLE_AS_IS:
    case 0 ... 7:
// adgs1409 supports only 4 states
    if (idle_state < mux.states) {
    mux.idle_state = idle_state;
    break;
    }
    fallthrough;
    default:
    dev_err(dev, "invalid idle-state %d\n", idle_state);
    return -EINVAL;
    }
    return devm_mux_chip_register(dev, mux_chip);
    }
    static const struct spi_device_id adgs1408_spi_id[] = {
    { "adgs1408", ADGS1408 },
    { "adgs1409", ADGS1409 },
    { }
    };
    MODULE_DEVICE_TABLE(spi, adgs1408_spi_id);
    static const struct of_device_id adgs1408_of_match[] = {
    { .compatible = "adi,adgs1408", .data = (void *)ADGS1408, },
    { .compatible = "adi,adgs1409", .data = (void *)ADGS1409, },
    { }
    };
    MODULE_DEVICE_TABLE(of, adgs1408_of_match);
    static struct spi_driver adgs1408_driver = {
    .driver = {
    .name = "adgs1408",
    .of_match_table = adgs1408_of_match,
    },
    .probe = adgs1408_probe,
    .id_table = adgs1408_spi_id,
    };
    module_spi_driver(adgs1408_driver);
    MODULE_AUTHOR("Mircea Caprioru <mircea.caprioru@analog.com>");
    MODULE_DESCRIPTION("Analog Devices ADGS1408 MUX driver");
    MODULE_LICENSE("GPL");
