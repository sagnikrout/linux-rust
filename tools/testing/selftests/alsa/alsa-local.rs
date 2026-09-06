//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/alsa/alsa-local.h
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
// kselftest configuration helpers for the hw specific configuration
//
// Original author: Jaroslav Kysela <perex@perex.cz>
// Copyright (c) 2022 Red Hat Inc.

extern "C" {
    pub fn conf_load();
}
extern "C" {
    pub fn conf_free();
}
extern "C" {
    pub fn conf_get_count(root: *mut snd_config_t, key1: *const c_char, key2: *const c_char) -> c_int;
}
extern "C" {
    pub fn conf_get_long(root: *mut snd_config_t, key1: *const c_char, key2: *const c_char, def: c_long) -> c_long;
}
extern "C" {
    pub fn conf_get_bool(root: *mut snd_config_t, key1: *const c_char, key2: *const c_char, def: c_int) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct card_cfg_data {
    pub card: c_int,
    pub config: *mut snd_config_t,
    pub filename: *const c_char,
    pub config_id: *const c_char,
    pub next: *mut card_cfg_data,
}
