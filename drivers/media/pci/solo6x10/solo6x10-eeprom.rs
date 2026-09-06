//! Automatically rewritten from C to Rust
//! Source: drivers/media/pci/solo6x10/solo6x10-eeprom.c
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
// Copyright (C) 2010-2013 Bluecherry, LLC <https://www.bluecherrydvr.com>
//
// Original author:
// Ben Collins <bcollins@ubuntu.com>
//
// Additional work by:
// John Brooks <john.brooks@bluecherry.net>
//

// Control
pub const EE_SHIFT_CLK: c_uint = 0x04;
pub const EE_CS: c_uint = 0x08;
pub const EE_DATA_WRITE: c_uint = 0x02;
pub const EE_DATA_READ: c_uint = 0x01;

    int i, ret;					\
    udelay(100);					\
    for (i = ret = 0; i < 1000 && !ret; i++)	\
    ret = solo_eeprom_reg_read(solo_dev);	\
    })

pub const ADDR_LEN: c_int = 6;
// Commands
pub const EE_EWEN_CMD: c_int = 4;
pub const EE_EWDS_CMD: c_int = 4;
pub const EE_WRITE_CMD: c_int = 5;
pub const EE_READ_CMD: c_int = 6;
pub const EE_ERASE_CMD: c_int = 7;
#[no_mangle]
unsafe extern "C" fn solo_eeprom_reg_read(solo_dev: *mut solo_dev) -> c_uint {
    static unsigned int solo_eeprom_reg_read(struct solo_dev *solo_dev)
    {
    return solo_reg_read(solo_dev, SOLO_EEPROM_CTRL) & EE_DATA_READ;
    }
#[no_mangle]
unsafe extern "C" fn solo_eeprom_reg_write(solo_dev: *mut solo_dev, data: u32) {
    static void solo_eeprom_reg_write(struct solo_dev *solo_dev, u32 data)
    {
    solo_reg_write(solo_dev, SOLO_EEPROM_CTRL, data);
    eeprom_delay();
    }
#[no_mangle]
unsafe extern "C" fn solo_eeprom_cmd(solo_dev: *mut solo_dev, cmd: c_int) {
    static void solo_eeprom_cmd(struct solo_dev *solo_dev, int cmd)
    {
    int i;
    solo_eeprom_reg_write(solo_dev, SOLO_EEPROM_ACCESS_EN);
    solo_eeprom_reg_write(solo_dev, SOLO_EEPROM_ENABLE);
    for (i = 4 + ADDR_LEN; i >= 0; i--) {
    let mut dataval: c_int = (cmd & (1 << i)) ? EE_DATA_WRITE : 0;
    solo_eeprom_reg_write(solo_dev, SOLO_EEPROM_ENABLE | dataval);
    solo_eeprom_reg_write(solo_dev, SOLO_EEPROM_ENABLE |
    EE_SHIFT_CLK | dataval);
    }
    solo_eeprom_reg_write(solo_dev, SOLO_EEPROM_ENABLE);
    }
#[no_mangle]
pub unsafe extern "C" fn solo_eeprom_ewen(solo_dev: *mut solo_dev, w_en: c_int) -> c_uint {
    unsigned int solo_eeprom_ewen(struct solo_dev *solo_dev, int w_en)
    {
    let mut ewen_cmd: c_int = (w_en ? 0x3f : 0) | (EE_EWEN_CMD << ADDR_LEN);
    let mut retval: c_uint = 0;
    int i;
    solo_eeprom_cmd(solo_dev, ewen_cmd);
    for (i = 0; i < 16; i++) {
    solo_eeprom_reg_write(solo_dev, SOLO_EEPROM_ENABLE |
    EE_SHIFT_CLK);
    retval = (retval << 1) | solo_eeprom_reg_read(solo_dev);
    solo_eeprom_reg_write(solo_dev, SOLO_EEPROM_ENABLE);
    retval = (retval << 1) | solo_eeprom_reg_read(solo_dev);
    }
    solo_eeprom_reg_write(solo_dev, ~EE_CS);
    retval = (retval << 1) | solo_eeprom_reg_read(solo_dev);
    return retval;
    }
#[no_mangle]
pub unsafe extern "C" fn solo_eeprom_read(solo_dev: *mut solo_dev, loc: c_int) -> __be16 {
    __be16 solo_eeprom_read(struct solo_dev *solo_dev, int loc)
    {
    let mut read_cmd: c_int = loc | (EE_READ_CMD << ADDR_LEN);
    let mut retval: u16 = 0;
    int i;
    solo_eeprom_cmd(solo_dev, read_cmd);
    for (i = 0; i < 16; i++) {
    solo_eeprom_reg_write(solo_dev, SOLO_EEPROM_ENABLE |
    EE_SHIFT_CLK);
    retval = (retval << 1) | solo_eeprom_reg_read(solo_dev);
    solo_eeprom_reg_write(solo_dev, SOLO_EEPROM_ENABLE);
    }
    solo_eeprom_reg_write(solo_dev, ~EE_CS);
    return ( __be16)retval;
    }
    int solo_eeprom_write(struct solo_dev *solo_dev, int loc,
    __be16 data)
    {
    let mut write_cmd: c_int = loc | (EE_WRITE_CMD << ADDR_LEN);
    unsigned int retval;
    int i;
    solo_eeprom_cmd(solo_dev, write_cmd);
    for (i = 15; i >= 0; i--) {
    let mut dataval: c_uint = (( unsigned)data >> i) & 1;
    solo_eeprom_reg_write(solo_dev, EE_ENB);
    solo_eeprom_reg_write(solo_dev,
    EE_ENB | (dataval << 1) | EE_SHIFT_CLK);
    }
    solo_eeprom_reg_write(solo_dev, EE_ENB);
    solo_eeprom_reg_write(solo_dev, ~EE_CS);
    solo_eeprom_reg_write(solo_dev, EE_ENB);
    for (i = retval = 0; i < 10000 && !retval; i++)
    retval = solo_eeprom_reg_read(solo_dev);
    solo_eeprom_reg_write(solo_dev, ~EE_CS);
    return !retval;
    }
