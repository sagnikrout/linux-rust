//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/dsa/realtek/rtl8366rb.h
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

pub const RTL8366RB_PORT_NUM_CPU: c_int = 5;
pub const RTL8366RB_NUM_PORTS: c_int = 6;
pub const RTL8366RB_PHY_NO_MAX: c_int = 4;
pub const RTL8366RB_NUM_LEDGROUPS: c_int = 4;
pub const RTL8366RB_PHY_ADDR_MAX: c_int = 31;
// LED control registers
// The LED blink rate is global; it is used by all triggers in all groups.
pub const RTL8366RB_LED_BLINKRATE_REG: c_uint = 0x0430;
pub const RTL8366RB_LED_BLINKRATE_MASK: c_uint = 0x0007;
pub const RTL8366RB_LED_BLINKRATE_28MS: c_uint = 0x0000;
pub const RTL8366RB_LED_BLINKRATE_56MS: c_uint = 0x0001;
pub const RTL8366RB_LED_BLINKRATE_84MS: c_uint = 0x0002;
pub const RTL8366RB_LED_BLINKRATE_111MS: c_uint = 0x0003;
pub const RTL8366RB_LED_BLINKRATE_222MS: c_uint = 0x0004;
pub const RTL8366RB_LED_BLINKRATE_446MS: c_uint = 0x0005;
// LED trigger event for each group
pub const RTL8366RB_LED_CTRL_REG: c_uint = 0x0431;

// The RTL8366RB_LED_X_X registers are used to manually set the LED state only
// when the corresponding LED group in RTL8366RB_LED_CTRL_REG is
// RTL8366RB_LEDGROUP_FORCE. Otherwise, it is ignored.
//
pub const RTL8366RB_LED_0_1_CTRL_REG: c_uint = 0x0432;
pub const RTL8366RB_LED_2_3_CTRL_REG: c_uint = 0x0433;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtl8366_ledgroup_mode {
    RTL8366RB_LEDGROUP_OFF			= 0x0,
    RTL8366RB_LEDGROUP_DUP_COL		= 0x1,
    RTL8366RB_LEDGROUP_LINK_ACT		= 0x2,
    RTL8366RB_LEDGROUP_SPD1000		= 0x3,
    RTL8366RB_LEDGROUP_SPD100		= 0x4,
    RTL8366RB_LEDGROUP_SPD10		= 0x5,
    RTL8366RB_LEDGROUP_SPD1000_ACT		= 0x6,
    RTL8366RB_LEDGROUP_SPD100_ACT		= 0x7,
    RTL8366RB_LEDGROUP_SPD10_ACT		= 0x8,
    RTL8366RB_LEDGROUP_SPD100_10_ACT	= 0x9,
    RTL8366RB_LEDGROUP_FIBER		= 0xa,
    RTL8366RB_LEDGROUP_AN_FAULT		= 0xb,
    RTL8366RB_LEDGROUP_LINK_RX		= 0xc,
    RTL8366RB_LEDGROUP_LINK_TX		= 0xd,
    RTL8366RB_LEDGROUP_MASTER		= 0xe,
    RTL8366RB_LEDGROUP_FORCE		= 0xf,

    __RTL8366RB_LEDGROUP_MODE_MAX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl8366rb_led {
    pub port_num: u8,
    pub led_group: u8,
    pub priv: *mut realtek_priv,
    pub cdev: led_classdev,
}

extern "C" {
    pub fn rtl8366rb_setup_leds(priv: *mut realtek_priv) -> c_int;
}

//
// struct rtl8366rb - RTL8366RB-specific data
// @max_mtu: per-port max MTU setting
// @pvid_enabled: if PVID is set for respective port
// @leds: per-port and per-ledgroup led info
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl8366rb {
    pub max_mtu: [c_uint; RTL8366RB_NUM_PORTS],
    pub pvid_enabled: [bool; RTL8366RB_NUM_PORTS],    pub leds: [rtl8366rb_led; RTL8366RB_NUM_PORTS][RTL8366RB_NUM_LEDGROUPS],
}

// This code is used also with LEDs disabled
