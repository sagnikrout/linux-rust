//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/dsa/realtek/rtl83xx.h
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


// SPDX-License-Identifier: GPL-2.0+
#[repr(C)]
#[derive(Copy, Clone)]
pub struct realtek_interface_info {
    pub val): *mut *mut *mut int (reg_read)(void ctx, u32 reg, u32,
    pub val): *mut *mut *mut int (reg_write)(void ctx, u32 reg, u32,
}

extern "C" {
    pub fn rtl83xx_lock(ctx: *mut c_void);
}
extern "C" {
    pub fn rtl83xx_unlock(ctx: *mut c_void);
}
extern "C" {
    pub fn rtl83xx_setup_user_mdio(ds: *mut dsa_switch) -> c_int;
}
extern "C" {
    pub fn rtl83xx_register_switch(priv: *mut realtek_priv) -> c_int;
}
extern "C" {
    pub fn rtl83xx_unregister_switch(priv: *mut realtek_priv);
}
extern "C" {
    pub fn rtl83xx_shutdown(priv: *mut realtek_priv);
}
extern "C" {
    pub fn rtl83xx_remove(priv: *mut realtek_priv);
}
extern "C" {
    pub fn rtl83xx_reset_assert(priv: *mut realtek_priv);
}
extern "C" {
    pub fn rtl83xx_reset_deassert(priv: *mut realtek_priv);
}
extern "C" {
    pub fn rtl83xx_setup_port_flood_control(priv: *mut realtek_priv, port: c_int) -> c_int;
}
extern "C" {
    pub fn rtl83xx_port_fast_age(ds: *mut dsa_switch, port: c_int);
}
