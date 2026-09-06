//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/cnum.h
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
// Copyright (c) 2026 Meta Platforms, Inc. and affiliates.

//
// cnum32: a circular number.
// A unified representation for signed and unsigned ranges.
//
// Assume that a 32-bit range is a circle, with 0 being in the 12 o'clock
// position, numbers placed sequentially in clockwise order and U32_MAX
// in the 11 o'clock position. Signed values map onto the same circle:
// S32_MAX sits at 5 o'clock, S32_MIN sits at 6 o'clock (opposite 0),
// negative values occupy the left half and positive values the right half.
//
// @cnum32 represents an arc on this circle drawn clockwise.
// @base corresponds to the first value of the range.
// @size corresponds to the number of integers in the range excluding @base.
// (The @base is excluded to avoid integer overflow when representing the full
// 0..U32_MAX range, which corresponds to 2^32, which can't be stored in u32).
//
// For example: {U32_MAX, 1} corresponds to signed range [-1, 0],
// {S32_MAX, 1} corresponds to unsigned range [S32_MAX, S32_MIN].
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cnum32 {
    pub base: u32,
    pub size: u32,
}

extern "C" {
    pub fn cnum32_from_urange(min: u32, max: u32) -> cnum32;
}
extern "C" {
    pub fn cnum32_from_srange(min: i32, max: i32) -> cnum32;
}
extern "C" {
    pub fn cnum32_umin(cnum: cnum32) -> u32;
}
extern "C" {
    pub fn cnum32_umax(cnum: cnum32) -> u32;
}
extern "C" {
    pub fn cnum32_smin(cnum: cnum32) -> i32;
}
extern "C" {
    pub fn cnum32_smax(cnum: cnum32) -> i32;
}
extern "C" {
    pub fn cnum32_intersect(a: cnum32, b: cnum32) -> cnum32;
}
extern "C" {
    pub fn cnum32_intersect_with(dst: *mut cnum32, src: cnum32);
}
extern "C" {
    pub fn cnum32_intersect_with_urange(dst: *mut cnum32, min: u32, max: u32);
}
extern "C" {
    pub fn cnum32_intersect_with_srange(dst: *mut cnum32, min: i32, max: i32);
}
extern "C" {
    pub fn cnum32_contains(cnum: cnum32, v: u32) -> bool;
}
extern "C" {
    pub fn cnum32_is_const(cnum: cnum32) -> bool;
}
extern "C" {
    pub fn cnum32_is_empty(cnum: cnum32) -> bool;
}
extern "C" {
    pub fn cnum32_add(a: cnum32, b: cnum32) -> cnum32;
}
extern "C" {
    pub fn cnum32_negate(a: cnum32) -> cnum32;
}
extern "C" {
    pub fn cnum32_is_subset(outer: cnum32, inner: cnum32) -> bool;
}
// Same as cnum32 but for 64-bit ranges
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cnum64 {
    pub base: u64,
    pub size: u64,
}

extern "C" {
    pub fn cnum64_from_urange(min: u64, max: u64) -> cnum64;
}
extern "C" {
    pub fn cnum64_from_srange(min: i64, max: i64) -> cnum64;
}
extern "C" {
    pub fn cnum64_umin(cnum: cnum64) -> u64;
}
extern "C" {
    pub fn cnum64_umax(cnum: cnum64) -> u64;
}
extern "C" {
    pub fn cnum64_smin(cnum: cnum64) -> i64;
}
extern "C" {
    pub fn cnum64_smax(cnum: cnum64) -> i64;
}
extern "C" {
    pub fn cnum64_intersect(a: cnum64, b: cnum64) -> cnum64;
}
extern "C" {
    pub fn cnum64_intersect_with(dst: *mut cnum64, src: cnum64);
}
extern "C" {
    pub fn cnum64_intersect_with_urange(dst: *mut cnum64, min: u64, max: u64);
}
extern "C" {
    pub fn cnum64_intersect_with_srange(dst: *mut cnum64, min: i64, max: i64);
}
extern "C" {
    pub fn cnum64_contains(cnum: cnum64, v: u64) -> bool;
}
extern "C" {
    pub fn cnum64_is_const(cnum: cnum64) -> bool;
}
extern "C" {
    pub fn cnum64_is_empty(cnum: cnum64) -> bool;
}
extern "C" {
    pub fn cnum64_add(a: cnum64, b: cnum64) -> cnum64;
}
extern "C" {
    pub fn cnum64_negate(a: cnum64) -> cnum64;
}
extern "C" {
    pub fn cnum64_is_subset(outer: cnum64, inner: cnum64) -> bool;
}
extern "C" {
    pub fn cnum32_from_cnum64(cnum: cnum64) -> cnum32;
}
extern "C" {
    pub fn cnum64_cnum32_intersect(a: cnum64, b: cnum32) -> cnum64;
}
