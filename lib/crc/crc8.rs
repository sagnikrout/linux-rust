//! Automatically rewritten from C to Rust
//! Source: lib/crc/crc8.c
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


//
// Copyright (c) 2011 Broadcom Corporation
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY
// SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION
// OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN
// CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
//

//
// crc8_populate_msb - fill crc table for given polynomial in reverse bit order.
//
// @table:	table to be filled.
// @polynomial:	polynomial for which table is to be filled.
//
#[no_mangle]
pub unsafe extern "C" fn crc8_populate_msb(table[CRC8_TABLE_SIZE]: u8, polynomial: u8) {
    void crc8_populate_msb(u8 table[CRC8_TABLE_SIZE], u8 polynomial)
    {
    int i, j;
    let mut msbit: u8 = 0x80;
    let mut t: u8 = msbit;
    table[0] = 0;
    for (i = 1; i < CRC8_TABLE_SIZE; i *= 2) {
    t = (t << 1) ^ (t & msbit ? polynomial : 0);
    for (j = 0; j < i; j++)
    table[i+j] = table[j] ^ t;
    }
    }
    EXPORT_SYMBOL(crc8_populate_msb);
//
// crc8_populate_lsb - fill crc table for given polynomial in regular bit order.
//
// @table:	table to be filled.
// @polynomial:	polynomial for which table is to be filled.
//
#[no_mangle]
pub unsafe extern "C" fn crc8_populate_lsb(table[CRC8_TABLE_SIZE]: u8, polynomial: u8) {
    void crc8_populate_lsb(u8 table[CRC8_TABLE_SIZE], u8 polynomial)
    {
    int i, j;
    let mut t: u8 = 1;
    table[0] = 0;
    for (i = (CRC8_TABLE_SIZE >> 1); i; i >>= 1) {
    t = (t >> 1) ^ (t & 1 ? polynomial : 0);
    for (j = 0; j < CRC8_TABLE_SIZE; j += 2*i)
    table[i+j] = table[j] ^ t;
    }
    }
    EXPORT_SYMBOL(crc8_populate_lsb);
//
// crc8 - calculate a crc8 over the given input data.
//
// @table: crc table used for calculation.
// @pdata: pointer to data buffer.
// @nbytes: number of bytes in data buffer.
// @crc: previous returned crc8 value.
//
#[no_mangle]
pub unsafe extern "C" fn crc8(table[CRC8_TABLE_SIZE]: u8, pdata: *const u8, nbytes: usize, crc: u8) -> u8 {
    u8 crc8(const u8 table[CRC8_TABLE_SIZE], const u8 *pdata, size_t nbytes, u8 crc)
    {
// loop over the buffer data
    while (nbytes-- > 0)
    crc = table[(crc ^ *pdata++) & 0xff];
    return crc;
    }
    EXPORT_SYMBOL(crc8);
    MODULE_DESCRIPTION("CRC8 (by Williams, Ross N.) function");
    MODULE_AUTHOR("Broadcom Corporation");
    MODULE_LICENSE("Dual BSD/GPL");
