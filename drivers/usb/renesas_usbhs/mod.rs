//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/renesas_usbhs/mod.h
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


// SPDX-License-Identifier: GPL-1.0+
//
// Renesas USB driver
//
// Copyright (C) 2011 Renesas Solutions Corp.
// Copyright (C) 2019 Renesas Electronics Corporation
// Kuninori Morimoto <kuninori.morimoto.gx@renesas.com>
//

//
// struct
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usbhs_irq_state {
    pub intsts0: u16,
    pub intsts1: u16,
    pub brdysts: u16,
    pub nrdysts: u16,
    pub bempsts: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usbhs_mod {
    pub name: *mut c_char,
//
// entry point from common.c
//
    pub priv): *mut *mut int (start)(struct usbhs_priv,
    pub priv): *mut *mut int (stop)(struct usbhs_priv,
//
// INTSTS0
//
// DVST (DVSQ)
    pub irq_state): *mut usbhs_irq_state,
// CTRT (CTSQ)
    pub irq_state): *mut usbhs_irq_state,
// BEMP / BEMPSTS
    pub irq_state): *mut usbhs_irq_state,
    pub irq_bempsts: u16,
// BRDY / BRDYSTS
    pub irq_state): *mut usbhs_irq_state,
    pub irq_brdysts: u16,
//
// INTSTS1
//
// ATTCHE
    pub irq_state): *mut usbhs_irq_state,
// DTCHE
    pub irq_state): *mut usbhs_irq_state,
// SIGN
    pub irq_state): *mut usbhs_irq_state,
// SACK
    pub irq_state): *mut usbhs_irq_state,
    pub priv: *mut usbhs_priv,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usbhs_mod_info {
    pub mod: [*mut usbhs_mod; USBHS_MAX],
    pub /: *mut *mut *mut usbhs_mod curt; / current mod,
//
// INTSTS0 :: VBINT
//
// This function will be used as autonomy mode (runtime_pwctrl == 0)
// when the platform doesn't have own get_vbus function.
//
// This callback cannot be member of "struct usbhs_mod" because it
// will be used even though host/gadget has not been selected.
//
    pub irq_state): *mut usbhs_irq_state,
//
// This function will be used on any gadget mode. To simplify the code,
// this member is in here.
//
    pub pdev): *mut *mut int (get_vbus)(struct platform_device,
}

//
// for host/gadget module
//
extern "C" {
    pub fn usbhs_mod_register(priv: *mut usbhs_priv, usb: *mut usbhs_mod, id: c_int);
}
extern "C" {
    pub fn usbhs_mod_is_host(priv: *mut usbhs_priv) -> c_int;
}
extern "C" {
    pub fn usbhs_mod_change(priv: *mut usbhs_priv, id: c_int) -> c_int;
}
extern "C" {
    pub fn usbhs_mod_probe(priv: *mut usbhs_priv) -> c_int;
}
extern "C" {
    pub fn usbhs_mod_remove(priv: *mut usbhs_priv);
}
extern "C" {
    pub fn usbhs_mod_autonomy_mode(priv: *mut usbhs_priv);
}
extern "C" {
    pub fn usbhs_mod_non_autonomy_mode(priv: *mut usbhs_priv);
}
//
// status functions
//
extern "C" {
    pub fn usbhs_status_get_device_state(irq_state: *mut usbhs_irq_state) -> c_int;
}
extern "C" {
    pub fn usbhs_status_get_ctrl_stage(irq_state: *mut usbhs_irq_state) -> c_int;
}
//
// callback functions
//
extern "C" {
    pub fn usbhs_irq_callback_update(priv: *mut usbhs_priv, mod: *mut usbhs_mod);
}

//
// host / gadget control
//

extern "C" {
    pub fn usbhs_mod_host_probe(priv: *mut usbhs_priv) -> c_int;
}
extern "C" {
    pub fn usbhs_mod_host_remove(priv: *mut usbhs_priv) -> c_int;
}

extern "C" {
    pub fn usbhs_mod_gadget_probe(priv: *mut usbhs_priv) -> c_int;
}
extern "C" {
    pub fn usbhs_mod_gadget_remove(priv: *mut usbhs_priv);
}

