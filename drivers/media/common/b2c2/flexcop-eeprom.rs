//! Automatically rewritten from C to Rust
//! Source: drivers/media/common/b2c2/flexcop-eeprom.c
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
// Linux driver for digital TV devices equipped with B2C2 FlexcopII(b)/III
// flexcop-eeprom.c - eeprom access methods (currently only MAC address reading)
// see flexcop.c for copyright information
//

// EEPROM (Skystar2 has one "24LC08B" chip on board)
#[no_mangle]
unsafe extern "C" fn eeprom_write(adapter: *mut adapter, addr: u16, buf: *mut u8, len: u16) -> c_int {
    static int eeprom_write(struct adapter *adapter, u16 addr, u8 *buf, u16 len)
    {
    return flex_i2c_write(adapter, 0x20000000, 0x50, addr, buf, len);
    }
    static int eeprom_lrc_write(struct adapter *adapter, u32 addr,
    u32 len, u8 *wbuf, u8 *rbuf, int retries)
    {
    int i;
    for (i = 0; i < retries; i++) {
    if (eeprom_write(adapter, addr, wbuf, len) == len) {
    if (eeprom_lrc_read(adapter, addr, len, rbuf, retries) == 1)
    return 1;
    }
    }
    return 0;
    }
// These functions could be used to unlock SkyStar2 cards.
#[no_mangle]
unsafe extern "C" fn eeprom_writeKey(adapter: *mut adapter, key: *mut u8, len: u32) -> c_int {
    static int eeprom_writeKey(struct adapter *adapter, u8 *key, u32 len)
    {
    u8 rbuf[20];
    u8 wbuf[20];
    if (len != 16)
    return 0;
    memcpy(wbuf, key, len);
    wbuf[16] = 0;
    wbuf[17] = 0;
    wbuf[18] = 0;
    wbuf[19] = calc_lrc(wbuf, 19);
    return eeprom_lrc_write(adapter, 0x3e4, 20, wbuf, rbuf, 4);
    }
#[no_mangle]
unsafe extern "C" fn eeprom_readKey(adapter: *mut adapter, key: *mut u8, len: u32) -> c_int {
    static int eeprom_readKey(struct adapter *adapter, u8 *key, u32 len)
    {
    u8 buf[20];
    if (len != 16)
    return 0;
    if (eeprom_lrc_read(adapter, 0x3e4, 20, buf, 4) == 0)
    return 0;
    memcpy(key, buf, len);
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn eeprom_set_mac_addr(adapter: *mut adapter, type: c_char, mac: *mut u8) -> c_char {
    static char eeprom_set_mac_addr(struct adapter *adapter, char type, u8 *mac)
    {
    u8 tmp[8];
    if (type != 0) {
    tmp[0] = mac[0];
    tmp[1] = mac[1];
    tmp[2] = mac[2];
    tmp[3] = mac[5];
    tmp[4] = mac[6];
    tmp[5] = mac[7];
    } else {
    tmp[0] = mac[0];
    tmp[1] = mac[1];
    tmp[2] = mac[2];
    tmp[3] = mac[3];
    tmp[4] = mac[4];
    tmp[5] = mac[5];
    }
    tmp[6] = 0;
    tmp[7] = calc_lrc(tmp, 7);
    if (eeprom_write(adapter, 0x3f8, tmp, 8) == 8)
    return 1;
    return 0;
    }
    static int flexcop_eeprom_read(struct flexcop_device *fc,
    u16 addr, u8 *buf, u16 len)
    {
    return fc.i2c_request(fc,FC_READ,FC_I2C_PORT_EEPROM,0x50,addr,buf,len);
    }

#[no_mangle]
unsafe extern "C" fn calc_lrc(buf: *mut u8, len: c_int) -> u8 {
    static u8 calc_lrc(u8 *buf, int len)
    {
    int i;
    let mut sum: u8 = 0;
    for (i = 0; i < len; i++)
    sum = sum ^ buf[i];
    return sum;
    }
    static int flexcop_eeprom_request(struct flexcop_device *fc,
    flexcop_access_op_t op, u16 addr, u8 *buf, u16 len, int retries)
    {
    int i,ret = 0;
    let mut chipaddr: u8 = 0x50 | ((addr >> 8) & 3);
    for (i = 0; i < retries; i++) {
    ret = fc.i2c_request(&fc.fc_i2c_adap[1], op, chipaddr,
    addr & 0xff, buf, len);
    if (ret == 0)
    break;
    }
    return ret;
    }
    static int flexcop_eeprom_lrc_read(struct flexcop_device *fc, u16 addr,
    u8 *buf, u16 len, int retries)
    {
    let mut ret: c_int = flexcop_eeprom_request(fc, FC_READ, addr, buf, len, retries);
    if (ret == 0)
    if (calc_lrc(buf, len - 1) != buf[len - 1])
    ret = -EINVAL;
    return ret;
    }
// JJ's comment about extended == 1: it is not presently used anywhere but was
// added to the low-level functions for possible support of EUI64
#[no_mangle]
pub unsafe extern "C" fn flexcop_eeprom_check_mac_addr(fc: *mut flexcop_device, extended: c_int) -> c_int {
    int flexcop_eeprom_check_mac_addr(struct flexcop_device *fc, int extended)
    {
    u8 buf[8];
    let mut ret: c_int = 0;
    if ((ret = flexcop_eeprom_lrc_read(fc,0x3f8,buf,8,4)) == 0) {
    if (extended != 0) {
    err("TODO: extended (EUI64) MAC addresses aren't completely supported yet");
    ret = -EINVAL;
    } else
    memcpy(fc.dvb_adapter.proposed_mac,buf,6);
    }
    return ret;
    }
    EXPORT_SYMBOL(flexcop_eeprom_check_mac_addr);
