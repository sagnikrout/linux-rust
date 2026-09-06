//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/pinctrl/qcom/pinctrl-msm.h
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
// Copyright (c) 2013, Sony Mobile Communications AB.
//

//
// struct msm_pingroup - Qualcomm pingroup definition
// @grp:                  Generic data of the pin group (name and pins)
// @funcs:                A list of pinmux functions that can be selected for
// this group. The index of the selected function is used
// for programming the function selector.
// Entries should be indices into the groups list of the
// struct msm_pinctrl_soc_data.
// @ctl_reg:              Offset of the register holding control bits for this group.
// @io_reg:               Offset of the register holding input/output bits for this group.
// @intr_cfg_reg:         Offset of the register holding interrupt configuration bits.
// @intr_status_reg:      Offset of the register holding the status bits for this group.
// @intr_target_reg:      Offset of the register specifying routing of the interrupts
// from this group. On most SoCs this register is the same as
// @intr_cfg_reg; leaving this field as zero causes the driver
// to fall back to @intr_cfg_reg automatically. Only set this
// explicitly on older SoCs where the interrupt target routing
// lives in a separate register (e.g. APQ8064, MSM8960).
// @mux_bit:              Offset in @ctl_reg for the pinmux function selection.
// @pull_bit:             Offset in @ctl_reg for the bias configuration.
// @drv_bit:              Offset in @ctl_reg for the drive strength configuration.
// @od_bit:               Offset in @ctl_reg for controlling open drain.
// @oe_bit:               Offset in @ctl_reg for controlling output enable.
// @in_bit:               Offset in @io_reg for the input bit value.
// @out_bit:              Offset in @io_reg for the output bit value.
// @intr_enable_bit:      Offset in @intr_cfg_reg for enabling the interrupt for this group.
// @intr_status_bit:      Offset in @intr_status_reg for reading and acking the interrupt
// status.
// @intr_wakeup_present_bit: Offset in @intr_target_reg specifying the GPIO can generate
// wakeup events.
// @intr_wakeup_enable_bit: Offset in @intr_target_reg to enable wakeup events for the GPIO.
// @intr_target_bit:      Offset in @intr_target_reg for configuring the interrupt routing.
// @intr_target_width:    Number of bits used for specifying interrupt routing target.
// @intr_target_kpss_val: Value in @intr_target_bit for specifying that the interrupt from
// this gpio should get routed to the KPSS processor.
// @intr_raw_status_bit:  Offset in @intr_cfg_reg for the raw status bit.
// @intr_polarity_bit:    Offset in @intr_cfg_reg for specifying polarity of the interrupt.
// @intr_detection_bit:   Offset in @intr_cfg_reg for specifying interrupt type.
// @intr_detection_width: Number of bits used for specifying interrupt type,
// Should be 2 for SoCs that can detect both edges in hardware,
// otherwise 1.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msm_pingroup {
    pub grp: pingroup,
    pub funcs: *mut unsigned,
    pub nfuncs: unsigned,
    pub ctl_reg: u32,
    pub io_reg: u32,
    pub intr_cfg_reg: u32,
    pub intr_status_reg: u32,
    pub intr_target_reg: u32,
    pub tile:2: c_uint,
    pub mux_bit:5: unsigned,
    pub pull_bit:5: unsigned,
    pub drv_bit:5: unsigned,
    pub i2c_pull_bit:5: unsigned,
    pub od_bit:5: unsigned,
    pub egpio_enable:5: unsigned,
    pub egpio_present:5: unsigned,
    pub oe_bit:5: unsigned,
    pub in_bit:5: unsigned,
    pub out_bit:5: unsigned,
    pub intr_enable_bit:5: unsigned,
    pub intr_status_bit:5: unsigned,
    pub intr_ack_high:1: unsigned,
    pub intr_wakeup_present_bit:5: unsigned,
    pub intr_wakeup_enable_bit:5: unsigned,
    pub intr_target_bit:5: unsigned,
    pub intr_target_width:5: unsigned,
    pub intr_target_kpss_val:5: unsigned,
    pub intr_raw_status_bit:5: unsigned,
    pub intr_polarity_bit:5: unsigned,
    pub intr_detection_bit:5: unsigned,
    pub intr_detection_width:5: unsigned,
}

//
// struct msm_gpio_wakeirq_map - Map of GPIOs and their wakeup pins
// @gpio:          The GPIOs that are wakeup capable
// @wakeirq:       The interrupt at the always-on interrupt controller
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msm_gpio_wakeirq_map {
    pub gpio: c_uint,
    pub wakeirq: c_uint,
}

//
// struct msm_pinctrl_soc_data - Qualcomm pin controller driver configuration
// @pins:	    An array describing all pins the pin controller affects.
// @npins:	    The number of entries in @pins.
// @functions:	    An array describing all mux functions the SoC supports.
// @nfunctions:	    The number of entries in @functions.
// @groups:	    An array describing all pin groups the pin SoC supports.
// @ngroups:	    The numbmer of entries in @groups.
// @ngpio:	    The number of pingroups the driver should expose as GPIOs.
// @pull_no_keeper: The SoC does not support keeper bias.
// @wakeirq_map:    The map of wakeup capable GPIOs and the pin at PDC/MPM
// @nwakeirq_map:   The number of entries in @wakeirq_map
// @wakeirq_dual_edge_errata: If true then GPIOs using the wakeirq_map need
// to be aware that their parent can't handle dual
// edge interrupts.
// @gpio_func: Which function number is GPIO (usually 0).
// @egpio_func: If non-zero then this SoC supports eGPIO. Even though in
// hardware this is a mux 1-level above the TLMM, we'll treat
// it as if this is just another mux state of the TLMM. Since
// it doesn't really map to hardware, we'll allocate a virtual
// function number for eGPIO and any time we see that function
// number used we'll treat it as a request to mux away from
// our TLMM towards another owner.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msm_pinctrl_soc_data {
    pub pins: *const pinctrl_pin_desc,
    pub npins: unsigned,
    pub functions: *const pinfunction,
    pub nfunctions: unsigned,
    pub groups: *const msm_pingroup,
    pub ngroups: unsigned,
    pub ngpios: unsigned,
    pub pull_no_keeper: bool,
    pub tiles: *const *const c_char,
    pub ntiles: c_uint,
    pub reserved_gpios: *const c_int,
    pub wakeirq_map: *const msm_gpio_wakeirq_map,
    pub nwakeirq_map: c_uint,
    pub wakeirq_dual_edge_errata: bool,
    pub gpio_func: c_uint,
    pub egpio_func: c_uint,
}
