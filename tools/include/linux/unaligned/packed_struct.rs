//! Automatically rewritten from C Header to Rust Module
//! Source: tools/include/linux/unaligned/packed_struct.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct __una_u16 {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __una_u32 {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __una_u64 {
    pub )p: *const *const __una_u16 ptr = (__una_u16,
    pub ptr->x: return,
    pub )p: *const *const __una_u32 ptr = (__una_u32,
    pub ptr->x: return,
    pub )p: *const *const __una_u64 ptr = (__una_u64,
    pub ptr->x: return,
    pub )p: *mut *mut __una_u16 ptr = (__una_u16,
    pub val: ptr->x =,
    pub )p: *mut *mut __una_u32 ptr = (__una_u32,
    pub val: ptr->x =,
    pub )p: *mut *mut __una_u64 ptr = (__una_u64,
    pub val: ptr->x =,
