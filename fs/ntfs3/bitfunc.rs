//! Automatically rewritten from C to Rust
//! Source: fs/ntfs3/bitfunc.c
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
// Copyright (C) 2019-2021 Paragon Software GmbH, All rights reserved.
//

//
// fill_mask[i] - first i bits are '1' , i = 0,1,2,3,4,5,6,7,8
// fill_mask[i] = 0xFF >> (8-i)
//
    static const u8 fill_mask[] = { 0x00, 0x01, 0x03, 0x07, 0x0F,
    0x1F, 0x3F, 0x7F, 0xFF };
//
// zero_mask[i] - first i bits are '0' , i = 0,1,2,3,4,5,6,7,8
// zero_mask[i] = 0xFF << i
//
    static const u8 zero_mask[] = { 0xFF, 0xFE, 0xFC, 0xF8, 0xF0,
    0xE0, 0xC0, 0x80, 0x00 };
//
// are_bits_clear
//
// Return: True if all bits [bit, bit+nbits) are zeros "0".
//
#[no_mangle]
pub unsafe extern "C" fn are_bits_clear(lmap: *const c_void, bit: usize, nbits: usize) -> bool {
    bool are_bits_clear(const void *lmap, size_t bit, size_t nbits)
    {
    let mut pos: usize = bit & 7;
    const u8 *map = (u8 *)lmap + (bit >> 3);
    if (pos) {
    if (8 - pos >= nbits)
    return !nbits || !(*map & fill_mask[pos + nbits] &
    zero_mask[pos]);
    if (*map++ & zero_mask[pos])
    return false;
    nbits -= 8 - pos;
    }
    pos = ((size_t)map) & (sizeof(size_t) - 1);
    if (pos) {
    pos = sizeof(size_t) - pos;
    if (nbits >= pos * 8) {
    for (nbits -= pos * 8; pos; pos--, map++) {
    if (*map)
    return false;
    }
    }
    }
    for (pos = nbits / BITS_IN_SIZE_T; pos; pos--, map += sizeof(size_t)) {
    if (*((size_t *)map))
    return false;
    }
    for (pos = (nbits % BITS_IN_SIZE_T) >> 3; pos; pos--, map++) {
    if (*map)
    return false;
    }
    pos = nbits & 7;
    if (pos && (*map & fill_mask[pos]))
    return false;
    return true;
    }
//
// are_bits_set
//
// Return: True if all bits [bit, bit+nbits) are ones "1".
//
#[no_mangle]
pub unsafe extern "C" fn are_bits_set(lmap: *const c_void, bit: usize, nbits: usize) -> bool {
    bool are_bits_set(const void *lmap, size_t bit, size_t nbits)
    {
    u8 mask;
    let mut pos: usize = bit & 7;
    const u8 *map = (u8 *)lmap + (bit >> 3);
    if (pos) {
    if (8 - pos >= nbits) {
    mask = fill_mask[pos + nbits] & zero_mask[pos];
    return !nbits || (*map & mask) == mask;
    }
    mask = zero_mask[pos];
    if ((*map++ & mask) != mask)
    return false;
    nbits -= 8 - pos;
    }
    pos = ((size_t)map) & (sizeof(size_t) - 1);
    if (pos) {
    pos = sizeof(size_t) - pos;
    if (nbits >= pos * 8) {
    for (nbits -= pos * 8; pos; pos--, map++) {
    if (*map != 0xFF)
    return false;
    }
    }
    }
    for (pos = nbits / BITS_IN_SIZE_T; pos; pos--, map += sizeof(size_t)) {
    if (*((size_t *)map) != MINUS_ONE_T)
    return false;
    }
    for (pos = (nbits % BITS_IN_SIZE_T) >> 3; pos; pos--, map++) {
    if (*map != 0xFF)
    return false;
    }
    pos = nbits & 7;
    if (pos) {
    mask = fill_mask[pos];
    if ((*map & mask) != mask)
    return false;
    }
    return true;
    }
