//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/phy/broadcom/phy-brcm-usb-init.h
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
//
// Copyright (C) 2014-2017 Broadcom
//

pub const USB_CTLR_MODE_HOST: c_int = 0;
pub const USB_CTLR_MODE_DEVICE: c_int = 1;
pub const USB_CTLR_MODE_DRD: c_int = 2;
pub const USB_CTLR_MODE_TYPEC_PD: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum brcmusb_reg_sel {
    BRCM_REGS_CTRL = 0,
    BRCM_REGS_XHCI_EC,
    BRCM_REGS_XHCI_GBL,
    BRCM_REGS_USB_PHY,
    BRCM_REGS_USB_MDIO,
    BRCM_REGS_BDC_EC,
    BRCM_REGS_MAX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcm_usb_init_ops {
    pub params): *mut *mut void (init_ipp)(struct brcm_usb_init_params,
    pub params): *mut *mut void (init_common)(struct brcm_usb_init_params,
    pub params): *mut *mut void (init_eohci)(struct brcm_usb_init_params,
    pub params): *mut *mut void (init_xhci)(struct brcm_usb_init_params,
    pub params): *mut *mut void (uninit_common)(struct brcm_usb_init_params,
    pub params): *mut *mut void (uninit_eohci)(struct brcm_usb_init_params,
    pub params): *mut *mut void (uninit_xhci)(struct brcm_usb_init_params,
    pub params): *mut *mut int (get_dual_select)(struct brcm_usb_init_params,
    pub params): *mut *mut void (set_dual_select)(struct brcm_usb_init_params,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcm_usb_init_params {
    pub regs: [*mut void __iomem; BRCM_REGS_MAX],
    pub ioc: c_int,
    pub ipp: c_int,
    pub supported_port_modes: c_int,
    pub port_mode: c_int,
    pub family_id: u32,
    pub product_id: u32,
    pub selected_family: c_int,
    pub family_name: *const c_char,
    pub usb_reg_bits_map: *const u32,
    pub ops: *const brcm_usb_init_ops,
    pub syscon_piarbctl: *mut regmap,
    pub wake_enabled: bool,
}

extern "C" {
    pub fn brcm_usb_dvr_init_74110(params: *mut brcm_usb_init_params);
}
extern "C" {
    pub fn brcm_usb_dvr_init_4908(params: *mut brcm_usb_init_params);
}
extern "C" {
    pub fn brcm_usb_dvr_init_7445(params: *mut brcm_usb_init_params);
}
extern "C" {
    pub fn brcm_usb_dvr_init_7216(params: *mut brcm_usb_init_params);
}
extern "C" {
    pub fn brcm_usb_dvr_init_7211b0(params: *mut brcm_usb_init_params);
}
//
// MIPS endianness is configured by boot strap, which also reverses all
// bus endianness (i.e., big-endian CPU + big endian bus ==> native
// endian I/O).
//
// Other architectures (e.g., ARM) either do not support big endian, or
// else leave I/O in little endian mode.
//
extern "C" {
    pub fn __raw_readl(_arg: addr) -> return;
}
extern "C" {
    pub fn readl_relaxed(_arg: addr) -> return;
}
// See brcmnand_readl() comments
