//! Automatically rewritten from C to Rust
//! Source: lib/hweight.c
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
// DOC: __sw_hweightN - returns the hamming weight of a N-bit word
// @w: the word to weigh
//
// The Hamming Weight of a number is the total number of bits set in it.
//
#[no_mangle]
pub unsafe extern "C" fn __sw_hweight32(w: c_uint) -> c_uint {
    unsigned int __sw_hweight32(unsigned int w)
    {

    w -= (w >> 1) & 0x55555555;
    w =  (w & 0x33333333) + ((w >> 2) & 0x33333333);
    w =  (w + (w >> 4)) & 0x0f0f0f0f;
    return (w * 0x01010101) >> 24;

    let mut res: c_uint = w - ((w >> 1) & 0x55555555);
    res = (res & 0x33333333) + ((res >> 2) & 0x33333333);
    res = (res + (res >> 4)) & 0x0F0F0F0F;
    res = res + (res >> 8);
    return (res + (res >> 16)) & 0x000000FF;

    }
    EXPORT_SYMBOL(__sw_hweight32);
#[no_mangle]
pub unsafe extern "C" fn __sw_hweight16(w: c_uint) -> c_uint {
    unsigned int __sw_hweight16(unsigned int w)
    {
    let mut res: c_uint = w - ((w >> 1) & 0x5555);
    res = (res & 0x3333) + ((res >> 2) & 0x3333);
    res = (res + (res >> 4)) & 0x0F0F;
    return (res + (res >> 8)) & 0x00FF;
    }
    EXPORT_SYMBOL(__sw_hweight16);
#[no_mangle]
pub unsafe extern "C" fn __sw_hweight8(w: c_uint) -> c_uint {
    unsigned int __sw_hweight8(unsigned int w)
    {
    let mut res: c_uint = w - ((w >> 1) & 0x55);
    res = (res & 0x33) + ((res >> 2) & 0x33);
    return (res + (res >> 4)) & 0x0F;
    }
    EXPORT_SYMBOL(__sw_hweight8);
#[no_mangle]
pub unsafe extern "C" fn __sw_hweight64(w: __u64) -> c_ulong {
    unsigned long __sw_hweight64(__u64 w)
    {

    return __sw_hweight32((unsigned int)(w >> 32)) +
    __sw_hweight32((unsigned int)w);

    w -= (w >> 1) & 0x5555555555555555ul;
    w =  (w & 0x3333333333333333ul) + ((w >> 2) & 0x3333333333333333ul);
    w =  (w + (w >> 4)) & 0x0f0f0f0f0f0f0f0ful;
    return (w * 0x0101010101010101ul) >> 56;

    let mut res: __u64 = w - ((w >> 1) & 0x5555555555555555ul);
    res = (res & 0x3333333333333333ul) + ((res >> 2) & 0x3333333333333333ul);
    res = (res + (res >> 4)) & 0x0F0F0F0F0F0F0F0Ful;
    res = res + (res >> 8);
    res = res + (res >> 16);
    return (res + (res >> 32)) & 0x00000000000000FFul;

    }
    EXPORT_SYMBOL(__sw_hweight64);
