//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/fsl_devices.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// include/linux/fsl_devices.h
//
// Definitions for any platform device related flags or structures for
// Freescale processor devices
//
// Maintainer: Kumar Gala <galak@kernel.crashing.org>
//
// Copyright 2004,2012 Freescale Semiconductor, Inc
//

//
// Some conventions on how we handle peripherals on Freescale chips
//
// unique device: a platform_device entry in fsl_plat_devs[] plus
// associated device information in its platform_data structure.
//
// A chip is described by a set of unique devices.
//
// Each sub-arch has its own master list of unique devices and
// enumerates them by enum fsl_devices in a sub-arch specific header
//
// The platform data structure is broken into two parts.  The
// first is device specific information that help identify any
// unique features of a peripheral.  The second is any
// information that may be defined by the board or how the device
// is connected externally of the chip.
//
// naming conventions:
// - platform data structures: <driver>_platform_data
// - platform data device flags: FSL_<driver>_DEV_<FLAG>
// - platform data board flags: FSL_<driver>_BRD_<FLAG>
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fsl_usb2_controller_ver {
    FSL_USB_VER_NONE = -1,
    FSL_USB_VER_OLD = 0,
    FSL_USB_VER_1_6 = 1,
    FSL_USB_VER_2_2 = 2,
    FSL_USB_VER_2_4 = 3,
    FSL_USB_VER_2_5 = 4,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fsl_usb2_operating_modes {
    FSL_USB2_MPH_HOST,
    FSL_USB2_DR_HOST,
    FSL_USB2_DR_DEVICE,
    FSL_USB2_DR_OTG,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fsl_usb2_phy_modes {
    FSL_USB2_PHY_NONE,
    FSL_USB2_PHY_ULPI,
    FSL_USB2_PHY_UTMI,
    FSL_USB2_PHY_UTMI_WIDE,
    FSL_USB2_PHY_SERIAL,
    FSL_USB2_PHY_UTMI_DUAL,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_usb2_platform_data {
// board specific information
    pub controller_ver: fsl_usb2_controller_ver,
    pub operating_mode: fsl_usb2_operating_modes,
    pub phy_mode: fsl_usb2_phy_modes,
    pub port_enables: c_uint,
    pub workaround: c_uint,
    pub ): *mut *mut int (init)(struct platform_device,
    pub ): *mut *mut void (exit)(struct platform_device,
    pub /: *mut *mut *mut void __iomem regs; / ioremap'd register base,
    pub clk: *mut clk,
    pub /: *mut *mut unsigned power_budget; / hcd->power_budget,
    pub big_endian_mmio:1: unsigned,
    pub big_endian_desc:1: unsigned,
    pub /: *mut *mut unsigned es:1; / need USBMODE:ES,
    pub le_setup_buf:1: unsigned,
    pub have_sysif_regs:1: unsigned,
    pub invert_drvvbus:1: unsigned,
    pub invert_pwr_fault:1: unsigned,
    pub suspended:1: unsigned,
    pub already_suspended:1: unsigned,
    pub has_fsl_erratum_a007792:1: unsigned,
    pub has_fsl_erratum_14:1: unsigned,
    pub has_fsl_erratum_a005275:1: unsigned,
    pub has_fsl_erratum_a005697:1: unsigned,
    pub has_fsl_erratum_a006918:1: unsigned,
    pub check_phy_clk_valid:1: unsigned,
// register save area for suspend/resume
    pub pm_command: u32,
    pub pm_status: u32,
    pub pm_intr_enable: u32,
    pub pm_frame_index: u32,
    pub pm_segment: u32,
    pub pm_frame_list: u32,
    pub pm_async_next: u32,
    pub pm_configured_flag: u32,
    pub pm_portsc: u32,
    pub pm_usbgenctrl: u32,
}

// Flags in fsl_usb2_mph_platform_data
pub const FSL_USB2_PORT0_ENABLED: c_uint = 0x00000001;
pub const FSL_USB2_PORT1_ENABLED: c_uint = 0x00000002;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_spi_platform_data {
    pub /: *mut *mut u32 initial_spmode; / initial SPMODE value,
    pub bus_num: i16,
    pub flags: c_uint,

// board specific information
    pub max_chipselect: u16,
    pub on): *mut *mut *mut void (cs_control)(struct spi_device spi, bool,
    pub sysclk: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpc8xx_pcmcia_ops {
    pub enable): *mut *mut void(hw_ctrl)(int slot, int,
    pub vpp): *mut *mut int(voltage_set)(int slot, int vcc, int,
}

// Returns non-zero if the current suspend operation would
// lead to a deep sleep (i.e. power removed from the core,
// instead of just the clock).
//

extern "C" {
    pub fn fsl_deep_sleep() -> c_int;
}

