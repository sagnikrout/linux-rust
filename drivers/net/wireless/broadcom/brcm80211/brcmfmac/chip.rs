//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/broadcom/brcm80211/brcmfmac/chip.h
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


// SPDX-License-Identifier: ISC
//
// Copyright (c) 2014 Broadcom Corporation
//

//
// struct brcmf_chip - chip level information.
//
// @chip: chip identifier.
// @chiprev: chip revision.
// @enum_base: base address of core enumeration space.
// @cc_caps: chipcommon core capabilities.
// @cc_caps_ext: chipcommon core extended capabilities.
// @pmucaps: PMU capabilities.
// @pmurev: PMU revision.
// @rambase: RAM base address (only applicable for ARM CR4 chips).
// @ramsize: amount of RAM on chip including retention.
// @srsize: amount of retention RAM on chip.
// @name: string representation of the chip identifier.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_chip {
    pub chip: u32,
    pub chiprev: u32,
    pub enum_base: u32,
    pub cc_caps: u32,
    pub cc_caps_ext: u32,
    pub pmucaps: u32,
    pub pmurev: u32,
    pub rambase: u32,
    pub ramsize: u32,
    pub srsize: u32,
    pub name: [c_char; 12],
}

//
// struct brcmf_core - core related information.
//
// @id: core identifier.
// @rev: core revision.
// @base: base address of core register space.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_core {
    pub id: u16,
    pub rev: u16,
    pub base: u32,
}

//
// struct brcmf_buscore_ops - buscore specific callbacks.
//
// @read32: read 32-bit value over bus.
// @write32: write 32-bit value over bus.
// @prepare: prepare bus for core configuration.
// @setup: bus-specific core setup.
// @active: chip becomes active.
// The callback should use the provided @rstvec when non-zero.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_buscore_ops {
    pub addr): *mut *mut *mut u32 (read32)(void ctx, u32,
    pub value): *mut *mut *mut void (write32)(void ctx, u32 addr, u32,
    pub ctx): *mut *mut int (prepare)(void,
    pub chip): *mut *mut *mut int (reset)(void ctx, struct brcmf_chip,
    pub chip): *mut *mut *mut int (setup)(void ctx, struct brcmf_chip,
    pub rstvec): *mut *mut *mut *mut void (activate)(void ctx, struct brcmf_chip chip, u32,
}

extern "C" {
    pub fn brcmf_chip_get_raminfo(pub: *mut brcmf_chip) -> c_int;
}
extern "C" {
    pub fn brcmf_chip_detach(chip: *mut brcmf_chip);
}
extern "C" {
    pub fn brcmf_chip_iscoreup(core: *mut brcmf_core) -> bool;
}
extern "C" {
    pub fn brcmf_chip_coredisable(core: *mut brcmf_core, prereset: u32, reset: u32);
}
extern "C" {
    pub fn brcmf_chip_set_passive(ci: *mut brcmf_chip);
}
extern "C" {
    pub fn brcmf_chip_set_active(ci: *mut brcmf_chip, rstvec: u32) -> bool;
}
extern "C" {
    pub fn brcmf_chip_sr_capable(pub: *mut brcmf_chip) -> bool;
}
extern "C" {
    pub fn brcmf_chip_enum_base(devid: u16) -> u32;
}
