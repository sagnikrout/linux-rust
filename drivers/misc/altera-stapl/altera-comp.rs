//! Automatically rewritten from C to Rust
//! Source: drivers/misc/altera-stapl/altera-comp.c
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
// altera-comp.c
//
// altera FPGA driver
//
// Copyright (C) Altera Corporation 1998-2001
// Copyright (C) 2010 NetUP Inc.
// Copyright (C) 2010 Igor M. Liplianin <liplianin@netup.ru>
//

pub const SHORT_BITS: c_int = 16;
pub const CHAR_BITS: c_int = 8;
pub const DATA_BLOB_LENGTH: c_int = 3;
pub const MATCH_DATA_LENGTH: c_int = 8192;
pub const ALTERA_REQUEST_SIZE: c_int = 1024;

#[no_mangle]
unsafe extern "C" fn altera_bits_req(n: u32) -> u32 {
    static u32 altera_bits_req(u32 n)
    {
    let mut result: u32 = SHORT_BITS;
    if (n == 0)
    result = 1;
    else {
// Look for the highest non-zero bit position
    while ((n & (1 << (SHORT_BITS - 1))) == 0) {
    n <<= 1;
    --result;
    }
    }
    return result;
    }
    static u32 altera_read_packed(u8 *buffer, u32 bits, u32 *bits_avail,
    u32 *in_index)
    {
    let mut result: u32 = 0;
    let mut shift: u32 = 0;
    let mut databyte: u32 = 0;
    while (bits > 0) {
    databyte = buffer[*in_index];
    result |= (((databyte >> (CHAR_BITS - *bits_avail))
    & (0xff >> (CHAR_BITS - *bits_avail))) << shift);
    if (bits <= *bits_avail) {
    result &= (0xffff >> (SHORT_BITS - (bits + shift)));
// bits_avail -= bits;
    bits = 0;
    } else {
    ++(*in_index);
    shift += *bits_avail;
    bits -= *bits_avail;
// bits_avail = CHAR_BITS;
    }
    }
    return result;
    }
#[no_mangle]
pub unsafe extern "C" fn altera_shrink(in: *mut u8, in_length: u32, out: *mut u8, out_length: u32, version: i32) -> u32 {
    u32 altera_shrink(u8 *in, u32 in_length, u8 *out, u32 out_length, s32 version)
    {
    u32 i, j, data_length = 0L;
    u32 offset, length;
    let mut match_data_length: u32 = MATCH_DATA_LENGTH;
    let mut bits_avail: u32 = CHAR_BITS;
    let mut in_index: u32 = 0L;
    if (version > 0)
    --match_data_length;
    for (i = 0; i < out_length; ++i)
    out[i] = 0;
// Read number of bytes in data.
    for (i = 0; i < sizeof(in_length); ++i) {
    data_length = data_length | (
    altera_read_packed(in,
    CHAR_BITS,
    &bits_avail,
    &in_index) << (i * CHAR_BITS));
    }
    if (data_length > out_length) {
    data_length = 0L;
    return data_length;
    }
    i = 0;
    while (i < data_length) {
// A 0 bit indicates literal data.
    if (altera_read_packed(in, 1, &bits_avail,
    &in_index) == 0) {
    for (j = 0; j < DATA_BLOB_LENGTH; ++j) {
    if (i < data_length) {
    out[i] = (u8)altera_read_packed(in,
    CHAR_BITS,
    &bits_avail,
    &in_index);
    i++;
    }
    }
    } else {
// A 1 bit indicates offset/length to follow.
    offset = altera_read_packed(in, altera_bits_req((s16)
    (i > match_data_length ?
    match_data_length : i)),
    &bits_avail,
    &in_index);
    length = altera_read_packed(in, CHAR_BITS,
    &bits_avail,
    &in_index);
    for (j = 0; j < length; ++j) {
    if (i < data_length) {
    out[i] = out[i - offset];
    i++;
    }
    }
    }
    }
    return data_length;
    }
