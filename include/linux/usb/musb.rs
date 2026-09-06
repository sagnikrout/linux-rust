//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/usb/musb.h
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
// This is used to for host and peripheral modes of the driver for
// Inventra (Multidrop) Highspeed Dual-Role Controllers:  (M)HDRC.
//
// Board initialization should put one of these into dev->platform_data,
// probably on some platform_device named "musb-hdrc".  It encapsulates
// key configuration differences between boards.
//
// The USB role is defined by the connector used on the board, so long as
// standards are being followed.  (Developer boards sometimes won't.)
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum musb_mode {
    MUSB_UNDEFINED = 0,
    MUSB_HOST,		/* A or Mini-A connector */
    MUSB_PERIPHERAL,	/* B or Mini-B connector */
    MUSB_OTG		/* Mini-AB connector */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum musb_fifo_style {
    FIFO_RXTX,
    FIFO_TX,
    FIFO_RX
    } __attribute__ ((packed));

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum musb_buf_mode {
    BUF_SINGLE,
    BUF_DOUBLE
    } __attribute__ ((packed));

    struct musb_fifo_cfg {
    u8			hw_ep_num;
    enum musb_fifo_style	style;
    enum musb_buf_mode	mode;
    u16			maxpacket;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct musb_hdrc_eps_bits {
    pub name: [c_char; 16],
    pub bits: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct musb_hdrc_config {
    pub /: *const *const *const musb_fifo_cfg fifo_cfg; / board fifo configuration,
    pub /: *mut *mut unsigned fifo_cfg_size; / size of the fifo configuration,
// MUSB configuration-specific details
    pub /: *mut *mut unsigned multipoint:1; / multipoint device,
    pub /: *mut *mut unsigned dyn_fifo:1 __deprecated; / supports dynamic fifo sizing,
// need to explicitly de-assert the port reset after resume?
    pub host_port_deassert_reset_at_resume:1: unsigned,
    pub /: *mut *mut u8 num_eps; / number of endpoints _with_ ep0,
    pub /: *mut *mut u8 ram_bits; / ram address size,
    pub maximum_speed: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct musb_hdrc_platform_data {
// MUSB_HOST, MUSB_PERIPHERAL, or MUSB_OTG
    pub mode: u8,
// for clk_get()
    pub clock: *const c_char,
// (HOST or OTG) switch VBUS on/off
    pub is_on): *mut *mut *mut int (set_vbus)(struct device dev, int,
// (HOST or OTG) mA/2 power supplied on (default = 8mA)
    pub power: u8,
// (PERIPHERAL) mA/2 max power consumed (default = 100mA)
    pub min_power: u8,
// (HOST or OTG) msec/2 after VBUS on till power good
    pub potpgt: u8,
// (HOST or OTG) program PHY for external Vbus
    pub extvbus:1: unsigned,
// MUSB configuration-specific details
    pub config: *const musb_hdrc_config,
// Architecture specific board data
    pub board_data: *mut c_void,
// Platform specific struct musb_ops pointer
    pub platform_ops: *const c_void,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum musb_vbus_id_status {
    MUSB_UNKNOWN = 0,
    MUSB_ID_GROUND,
    MUSB_ID_FLOAT,
    MUSB_VBUS_VALID,
    MUSB_VBUS_OFF,
}

extern "C" {
    pub fn musb_mailbox(status: musb_vbus_id_status) -> c_int;
}

// TUSB 6010 support

