//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/pinctrl/pinctrl-lantiq.h
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
// linux/drivers/pinctrl/pinctrl-lantiq.h
// based on linux/drivers/pinctrl/pinctrl-pxa3xx.h
//
// Copyright (C) 2012 John Crispin <john@phrozen.org>
//

pub const LTQ_MAX_MUX: c_int = 4;
pub const MFPR_FUNC_MASK: c_uint = 0x3;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ltq_pinconf_param {
    LTQ_PINCONF_PARAM_PULL,
    LTQ_PINCONF_PARAM_OPEN_DRAIN,
    LTQ_PINCONF_PARAM_DRIVE_CURRENT,
    LTQ_PINCONF_PARAM_SLEW_RATE,
    LTQ_PINCONF_PARAM_OUTPUT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ltq_cfg_param {
    pub property: *const c_char,
    pub param: ltq_pinconf_param,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ltq_mfp_pin {
    pub name: *const c_char,
    pub pin: c_uint,
    pub func: [c_ushort; LTQ_MAX_MUX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ltq_pin_group {
    pub name: *const c_char,
    pub mux: unsigned,
    pub pins: *const unsigned,
    pub npins: unsigned,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ltq_pmx_func {
    pub name: *const c_char,
    pub groups: *const *const c_char,
    pub num_groups: unsigned,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ltq_pinmux_info {
    pub dev: *mut device,
    pub pctrl: *mut pinctrl_dev,
// we need to manage up to 5 pad controllers
    pub membase: [*mut void __iomem; 5],
// the descriptor for the subsystem
    pub desc: *mut pinctrl_desc,
// we expose our pads to the subsystem
    pub pads: *mut pinctrl_pin_desc,
// the number of pads. this varies between socs
    pub num_pads: c_uint,
// these are our multifunction pins
    pub mfp: *const ltq_mfp_pin,
    pub num_mfp: c_uint,
// a number of multifunction pins can be grouped together
    pub grps: *const ltq_pin_group,
    pub num_grps: c_uint,
// a mapping between function string and id
    pub funcs: *const ltq_pmx_func,
    pub num_funcs: c_uint,
// the pinconf options that we are able to read from the DT
    pub params: *const ltq_cfg_param,
    pub num_params: c_uint,
// the pad controller can have a irq mapping
    pub exin: *const unsigned,
    pub num_exin: c_uint,
// we need 5 clocks max
    pub clk: [*mut clk; 5],
// soc specific callback used to apply muxing
    pub mux): *mut *mut *mut int (apply_mux)(struct pinctrl_dev pctrldev, int pin, int,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ltq_pin {
    GPIO0 = 0,
    GPIO1,
    GPIO2,
    GPIO3,
    GPIO4,
    GPIO5,
    GPIO6,
    GPIO7,
    GPIO8,
    GPIO9,
    GPIO10, /* 10 */
    GPIO11,
    GPIO12,
    GPIO13,
    GPIO14,
    GPIO15,
    GPIO16,
    GPIO17,
    GPIO18,
    GPIO19,
    GPIO20, /* 20 */
    GPIO21,
    GPIO22,
    GPIO23,
    GPIO24,
    GPIO25,
    GPIO26,
    GPIO27,
    GPIO28,
    GPIO29,
    GPIO30, /* 30 */
    GPIO31,
    GPIO32,
    GPIO33,
    GPIO34,
    GPIO35,
    GPIO36,
    GPIO37,
    GPIO38,
    GPIO39,
    GPIO40, /* 40 */
    GPIO41,
    GPIO42,
    GPIO43,
    GPIO44,
    GPIO45,
    GPIO46,
    GPIO47,
    GPIO48,
    GPIO49,
    GPIO50, /* 50 */
    GPIO51,
    GPIO52,
    GPIO53,
    GPIO54,
    GPIO55,
    GPIO56,
    GPIO57,
    GPIO58,
    GPIO59,
    GPIO60, /* 60 */
    GPIO61,
    GPIO62,
    GPIO63,

    GPIO64,
    GPIO65,
    GPIO66,
    GPIO67,
    GPIO68,
    GPIO69,
    GPIO70,
    GPIO71,
    GPIO72,
    GPIO73,
    GPIO74,
    GPIO75,
    GPIO76,
    GPIO77,
    GPIO78,
    GPIO79,
    GPIO80,
    GPIO81,
    GPIO82,
    GPIO83,
    GPIO84,
    GPIO85,
    GPIO86,
    GPIO87,
    GPIO88,
}
