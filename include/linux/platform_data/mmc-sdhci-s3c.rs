//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/mmc-sdhci-s3c.h
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
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cd_types {
    S3C_SDHCI_CD_INTERNAL,	/* use mmc internal CD line */
    S3C_SDHCI_CD_EXTERNAL,	/* use external callback */
    S3C_SDHCI_CD_GPIO,	/* use external gpio pin for CD line */
    S3C_SDHCI_CD_NONE,	/* no CD line, use polling to detect card */
    S3C_SDHCI_CD_PERMANENT,	/* no CD line, card permanently wired to host */
}

//
// struct s3c_sdhci_platdata() - Platform device data for Samsung SDHCI
// @max_width: The maximum number of data bits supported.
// @host_caps: Standard MMC host capabilities bit field.
// @host_caps2: The second standard MMC host capabilities bit field.
// @cd_type: Type of Card Detection method (see cd_types enum above)
// @ext_cd_init: Initialize external card detect subsystem. Called on
// sdhci-s3c driver probe when cd_type == S3C_SDHCI_CD_EXTERNAL.
// notify_func argument is a callback to the sdhci-s3c driver
// that triggers the card detection event. Callback arguments:
// dev is pointer to platform device of the host controller,
// state is new state of the card (0 - removed, 1 - inserted).
// @ext_cd_cleanup: Cleanup external card detect subsystem. Called on
// sdhci-s3c driver remove when cd_type == S3C_SDHCI_CD_EXTERNAL.
// notify_func argument is the same callback as for ext_cd_init.
// @ext_cd_gpio: gpio pin used for external CD line, valid only if
// cd_type == S3C_SDHCI_CD_GPIO
// @ext_cd_gpio_invert: invert values for external CD gpio line
// @cfg_gpio: Configure the GPIO for a specific card bit-width
//
// Initialisation data specific to either the machine or the platform
// for the device driver to use or call-back when configuring gpio or
// card speed information.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct s3c_sdhci_platdata {
    pub max_width: c_uint,
    pub host_caps: c_uint,
    pub host_caps2: c_uint,
    pub pm_caps: c_uint,
    pub cd_type: cd_types,
    pub ext_cd_gpio: c_int,
    pub ext_cd_gpio_invert: bool,
    pub state)): c_int,
    pub state)): c_int,
    pub width): *mut *mut *mut void (cfg_gpio)(struct platform_device dev, int,
}
