//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/pinctrl/intel/pinctrl-intel.h
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
// Core pinctrl/GPIO driver for Intel GPIO controllers
//
// Copyright (C) 2015 Intel Corporation
// Authors: Mathias Nyman <mathias.nyman@linux.intel.com>
// Mika Westerberg <mika.westerberg@linux.intel.com>
//

//
// struct intel_pingroup - Description about group of pins
// @grp: Generic data of the pin group (name and pins)
// @mode: Native mode in which the group is muxed out @pins. Used if @modes is %NULL.
// @modes: If not %NULL this will hold mode for each pin in @pins
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_pingroup {
    pub grp: pingroup,
    pub mode: c_ushort,
    pub modes: *const c_uint,
}

//
// struct intel_function - Description about a function
// @func: Generic data of the pin function (name and groups of pins)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_function {
    pub func: pinfunction,
}

pub const INTEL_PINCTRL_MAX_GPP_SIZE: c_int = 32;
//
// struct intel_padgroup - Hardware pad group information
// @reg_num: GPI_IS register number
// @base: Starting pin of this group
// @size: Size of this group (maximum is %INTEL_PINCTRL_MAX_GPP_SIZE).
// @gpio_base: Starting GPIO base of this group
// @padown_num: PAD_OWN register number (assigned by the core driver)
//
// If pad groups of a community are not the same size, use this structure
// to specify them.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_padgroup {
    pub reg_num: c_uint,
    pub base: c_uint,
    pub size: c_uint,
    pub gpio_base: c_int,
    pub padown_num: c_uint,
}

//
// enum - Special treatment for GPIO base in pad group
//
// @INTEL_GPIO_BASE_ZERO:	force GPIO base to be 0
// @INTEL_GPIO_BASE_NOMAP:	no GPIO mapping should be created
// @INTEL_GPIO_BASE_MATCH:	matches with starting pin number
//
// Initialise struct intel_padgroup

//
// struct intel_community - Intel pin community description
// @barno: MMIO BAR number where registers for this community reside
// @padown_offset: Register offset of PAD_OWN register from @regs. If %0
// then there is no support for owner.
// @padcfglock_offset: Register offset of PADCFGLOCK from @regs. If %0 then
// locking is not supported.
// @hostown_offset: Register offset of HOSTSW_OWN from @regs. If %0 then it
// is assumed that the host owns the pin (rather than
// ACPI).
// @is_offset: Register offset of GPI_IS from @regs.
// @ie_offset: Register offset of GPI_IE from @regs.
// @features: Additional features supported by the hardware
// @pin_base: Starting pin of pins in this community
// @npins: Number of pins in this community
// @gpp_size: Maximum number of pads in each group, such as PADCFGLOCK,
// HOSTSW_OWN, GPI_IS, GPI_IE. Used when @gpps is %NULL.
// @gpp_num_padown_regs: Number of pad registers each pad group consumes at
// minimum. Used when @gpps is %NULL.
// @gpps: Pad groups if the controller has variable size pad groups
// @ngpps: Number of pad groups in this community
// @pad_map: Optional non-linear mapping of the pads
// @nirqs: Optional total number of IRQs this community can generate
// @acpi_space_id: Optional address space ID for ACPI OpRegion handler
// @regs: Community specific common registers (reserved for core driver)
// @pad_regs: Community specific pad registers (reserved for core driver)
//
// In older Intel GPIO host controllers, this driver supports, each pad group
// is of equal size (except the last one). In that case the driver can just
// fill in @gpp_size and @gpp_num_padown_regs fields and let the core driver
// to handle the rest.
//
// In newer Intel GPIO host controllers each pad group is of variable size,
// so the client driver can pass custom @gpps and @ngpps instead.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_community {
    pub barno: c_uint,
    pub padown_offset: c_uint,
    pub padcfglock_offset: c_uint,
    pub hostown_offset: c_uint,
    pub is_offset: c_uint,
    pub ie_offset: c_uint,
    pub features: c_uint,
    pub pin_base: c_uint,
    pub npins: usize,
    pub gpp_size: c_uint,
    pub gpp_num_padown_regs: c_uint,
    pub gpps: *const intel_padgroup,
    pub ngpps: usize,
    pub pad_map: *const c_uint,
    pub nirqs: c_ushort,
    pub acpi_space_id: c_ushort,
// Reserved for the core driver
    pub regs: *mut void __iomem,
    pub pad_regs: *mut void __iomem,
}

// Additional features supported by the hardware

//
// PIN_GROUP - Declare a pin group
// @n: Name of the group
// @p: An array of pins this group consists
// @m: Mode which the pins are put when this group is active. Can be either
// a single integer or an array of integers in which case mode is per
// pin.
//

//
// struct intel_pinctrl_soc_data - Intel pin controller per-SoC configuration
// @uid: ACPI _UID for the probe driver use if needed
// @pins: Array if pins this pinctrl controls
// @npins: Number of pins in the array
// @groups: Array of pin groups
// @ngroups: Number of groups in the array
// @functions: Array of functions
// @nfunctions: Number of functions in the array
// @communities: Array of communities this pinctrl handles
// @ncommunities: Number of communities in the array
//
// The @communities is used as a template by the core driver. It will make
// copy of all communities and fill in rest of the information.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_pinctrl_soc_data {
    pub uid: *const c_char,
    pub pins: *const pinctrl_pin_desc,
    pub npins: usize,
    pub groups: *const intel_pingroup,
    pub ngroups: usize,
    pub functions: *const intel_function,
    pub nfunctions: usize,
    pub communities: *const intel_community,
    pub ncommunities: usize,
}

//
// struct intel_pinctrl_context - context to be saved during suspend-resume
// @pads: Opaque context per pad (driver dependent)
// @communities: Opaque context per community (driver dependent)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_pinctrl_context {
    pub pads: *mut intel_pad_context,
    pub communities: *mut intel_community_context,
}

//
// struct intel_pinctrl - Intel pinctrl private structure
// @dev: Pointer to the device structure
// @lock: Lock to serialize register access
// @pctldesc: Pin controller description
// @pctldev: Pointer to the pin controller device
// @chip: GPIO chip in this pin controller
// @soc: SoC/PCH specific pin configuration data
// @communities: All communities in this pin controller
// @ncommunities: Number of communities in this pin controller
// @context: Configuration saved over system sleep
// @irq: pinctrl/GPIO chip irq number
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_pinctrl {
    pub dev: *mut device,
    pub lock: raw_spinlock_t,
    pub pctldesc: pinctrl_desc,
    pub pctldev: *mut pinctrl_dev,
    pub chip: gpio_chip,
    pub soc: *const intel_pinctrl_soc_data,
    pub communities: *mut intel_community,
    pub ncommunities: usize,
    pub context: intel_pinctrl_context,
    pub irq: c_int,
}

extern "C" {
    pub fn intel_pinctrl_probe_by_hid(pdev: *mut platform_device) -> c_int;
}
extern "C" {
    pub fn intel_pinctrl_probe_by_uid(pdev: *mut platform_device) -> c_int;
}
extern "C" {
    pub fn intel_gpio_add_pin_ranges(gc: *mut gpio_chip) -> c_int;
}
extern "C" {
    pub fn intel_get_groups_count(pctldev: *mut pinctrl_dev) -> c_int;
}
extern "C" {
    pub fn intel_get_functions_count(pctldev: *mut pinctrl_dev) -> c_int;
}
