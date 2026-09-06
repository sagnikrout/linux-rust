//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/mmc-omap.h
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
// MMC definitions for OMAP2
//
// Copyright (C) 2006 Nokia Corporation
//
pub const OMAP_MMC_MAX_SLOTS: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap_mmc_platform_data {
// back-link to device
    pub dev: *mut device,
// number of slots per controller
    pub nr_slots:2: unsigned,
// set if your board has components or wiring that limits the
// maximum frequency on the MMC bus
    pub max_freq: c_uint,
// initialize board-specific MMC functionality, can be NULL if
// not supported
    pub dev): *mut *mut int (init)(struct device,
    pub dev): *mut *mut void (cleanup)(struct device,
    pub dev): *mut *mut void (shutdown)(struct device,
// Return context loss count due to PM states changing
    pub dev): *mut *mut int (get_context_loss_count)(struct device,
// Integrating attributes from the omap_hwmod layer
    pub controller_flags: u8,
// Register offset deviation
    pub reg_offset: u16,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap_mmc_slot_data {
//
// 4/8 wires and any additional host capabilities
// need to OR'd all capabilities (ref. linux/mmc/host.h)
//
    pub /: *mut *mut u8 wires; / Used for the MMC driver on omap1 and 2420,
    pub /: *mut *mut u32 caps; / Used for the MMC driver on 2430 and later,
    pub /: *mut *mut u32 pm_caps; / PM capabilities of the mmc,
//
// nomux means "standard" muxing is wrong on this board, and
// that board-specific code handled it before common init logic.
//
    pub nomux:1: unsigned,
// switch pin can be for card detect (default) or card cover
    pub cover:1: unsigned,
// use the internal clock
    pub internal_clock:1: unsigned,
// nonremovable e.g. eMMC
    pub nonremovable:1: unsigned,
// Try to sleep or power off when possible
    pub power_saving:1: unsigned,
// If using power_saving and the MMC power is not to go off
    pub no_off:1: unsigned,
// eMMC does not handle power off when not in sleep state
    pub no_regulator_off_init:1: unsigned,
// Regulator off remapped to sleep
    pub vcc_aux_disable_is_sleep:1: unsigned,
// we can put the features above into this variable

    pub features: unsigned,
    pub /: *mut *mut int switch_pin; / gpio (card detect),
    pub /: *mut *mut int gpio_wp; / gpio (write protect),
    pub bus_mode): *mut *mut *mut int (set_bus_mode)(struct device dev, int slot, int,
    pub vdd): int power_on, int,
    pub slot): *mut *mut *mut int (get_ro)(struct device dev, int,
    pub power_on): *mut *mut *mut void (remux)(struct device dev, int slot, int,
// Call back before enabling / disabling regulators
    pub vdd): int power_on, int,
// Call back after enabling / disabling regulators
    pub vdd): int power_on, int,
// if we have special card, init it using this callback
    pub card): *mut *mut void (init_card)(struct mmc_card,
// return MMC cover switch state, can be NULL if not supported.
//
// possible return values:
// 0 - closed
// 1 - open
//
    pub slot): *mut *mut *mut int (get_cover_state)(struct device dev, int,
    pub name: *const c_char,
    pub ocr_mask: u32,
// Card detection
    pub slot): *mut *mut *mut int (card_detect)(struct device dev, int,
    pub ban_openended:1: c_uint,
    pub slots: [}; OMAP_MMC_MAX_SLOTS],
}
