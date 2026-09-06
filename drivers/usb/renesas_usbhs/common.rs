//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/renesas_usbhs/common.h
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


// SPDX-License-Identifier: GPL-1.0+
//
// Renesas USB driver
//
// Copyright (C) 2011 Renesas Solutions Corp.
// Copyright (C) 2019 Renesas Electronics Corporation
// Kuninori Morimoto <kuninori.morimoto.gx@renesas.com>
//

//
// register define
//
pub const SYSCFG: c_uint = 0x0000;
pub const BUSWAIT: c_uint = 0x0002;
pub const DVSTCTR: c_uint = 0x0008;
pub const TESTMODE: c_uint = 0x000C;
pub const CFIFO: c_uint = 0x0014;
pub const CFIFOSEL: c_uint = 0x0020;
pub const CFIFOCTR: c_uint = 0x0022;
pub const D0FIFO: c_uint = 0x0100;
pub const D0FIFOSEL: c_uint = 0x0028;
pub const D0FIFOCTR: c_uint = 0x002A;
pub const D1FIFO: c_uint = 0x0120;
pub const D1FIFOSEL: c_uint = 0x002C;
pub const D1FIFOCTR: c_uint = 0x002E;
pub const INTENB0: c_uint = 0x0030;
pub const INTENB1: c_uint = 0x0032;
pub const BRDYENB: c_uint = 0x0036;
pub const NRDYENB: c_uint = 0x0038;
pub const BEMPENB: c_uint = 0x003A;
pub const INTSTS0: c_uint = 0x0040;
pub const INTSTS1: c_uint = 0x0042;
pub const BRDYSTS: c_uint = 0x0046;
pub const NRDYSTS: c_uint = 0x0048;
pub const BEMPSTS: c_uint = 0x004A;
pub const FRMNUM: c_uint = 0x004C;
pub const USBREQ: c_uint = 0x0054	/* USB request type register */;
pub const USBVAL: c_uint = 0x0056	/* USB request value register */;
pub const USBINDX: c_uint = 0x0058	/* USB request index register */;
pub const USBLENG: c_uint = 0x005A	/* USB request length register */;
pub const DCPCFG: c_uint = 0x005C;
pub const DCPMAXP: c_uint = 0x005E;
pub const DCPCTR: c_uint = 0x0060;
pub const PIPESEL: c_uint = 0x0064;
pub const PIPECFG: c_uint = 0x0068;
pub const PIPEBUF: c_uint = 0x006A;
pub const PIPEMAXP: c_uint = 0x006C;
pub const PIPEPERI: c_uint = 0x006E;
pub const PIPEnCTR: c_uint = 0x0070;
pub const PIPE1TRE: c_uint = 0x0090;
pub const PIPE1TRN: c_uint = 0x0092;
pub const PIPE2TRE: c_uint = 0x0094;
pub const PIPE2TRN: c_uint = 0x0096;
pub const PIPE3TRE: c_uint = 0x0098;
pub const PIPE3TRN: c_uint = 0x009A;
pub const PIPE4TRE: c_uint = 0x009C;
pub const PIPE4TRN: c_uint = 0x009E;
pub const PIPE5TRE: c_uint = 0x00A0;
pub const PIPE5TRN: c_uint = 0x00A2;
pub const PIPEBTRE: c_uint = 0x00A4;
pub const PIPEBTRN: c_uint = 0x00A6;
pub const PIPECTRE: c_uint = 0x00A8;
pub const PIPECTRN: c_uint = 0x00AA;
pub const PIPEDTRE: c_uint = 0x00AC;
pub const PIPEDTRN: c_uint = 0x00AE;
pub const PIPEETRE: c_uint = 0x00B0;
pub const PIPEETRN: c_uint = 0x00B2;
pub const PIPEFTRE: c_uint = 0x00B4;
pub const PIPEFTRN: c_uint = 0x00B6;
pub const PIPE9TRE: c_uint = 0x00B8;
pub const PIPE9TRN: c_uint = 0x00BA;
pub const PIPEATRE: c_uint = 0x00BC;
pub const PIPEATRN: c_uint = 0x00BE;
pub const DEVADD0: c_uint = 0x00D0 /* Device address n configuration */;
pub const DEVADD1: c_uint = 0x00D2;
pub const DEVADD2: c_uint = 0x00D4;
pub const DEVADD3: c_uint = 0x00D6;
pub const DEVADD4: c_uint = 0x00D8;
pub const DEVADD5: c_uint = 0x00DA;
pub const DEVADD6: c_uint = 0x00DC;
pub const DEVADD7: c_uint = 0x00DE;
pub const DEVADD8: c_uint = 0x00E0;
pub const DEVADD9: c_uint = 0x00E2;
pub const DEVADDA: c_uint = 0x00E4;
pub const D2FIFOSEL: c_uint = 0x00F0	/* for R-Car Gen2 */;
pub const D2FIFOCTR: c_uint = 0x00F2	/* for R-Car Gen2 */;
pub const D3FIFOSEL: c_uint = 0x00F4	/* for R-Car Gen2 */;
pub const D3FIFOCTR: c_uint = 0x00F6	/* for R-Car Gen2 */;
pub const SUSPMODE: c_uint = 0x0102	/* for RZ/A */;
// SYSCFG

// DVSTCTR

// CFIFOSEL

// CFIFOCTR

// INTENB0

// INTENB1

// INTSTS0

// INTSTS1

// PIPECFG
// DCPCFG

// PIPEMAXP
// DCPMAXP

// PIPEBUF
pub const BUFSIZE_SHIFT: c_int = 10;

// PIPEnCTR
// DCPCTR

pub const PID_NAK: c_int = 0;
pub const PID_BUF: c_int = 1;
pub const PID_STALL10: c_int = 2;
pub const PID_STALL11: c_int = 3;

// PIPEnTRE

// FRMNUM

// DEVADDn

pub const USBSPD_SPEED_LOW: c_uint = 0x1;
pub const USBSPD_SPEED_FULL: c_uint = 0x2;
pub const USBSPD_SPEED_HIGH: c_uint = 0x3;
// SUSPMODE

//
// struct
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usbhs_priv {
    pub base: *mut void __iomem,
    pub irq: c_uint,
    pub pfunc: *const renesas_usbhs_platform_callback,
    pub dparam: renesas_usbhs_driver_param,
    pub notify_hotplug_work: delayed_work,
    pub pdev: *mut platform_device,
    pub edev: *mut extcon_dev,
    pub lock: spinlock_t,
//
// module control
//
    pub mod_info: usbhs_mod_info,
//
// pipe control
//
    pub pipe_info: usbhs_pipe_info,
//
// fifo control
//
    pub fifo_info: usbhs_fifo_info,
    pub phy: *mut phy,
    pub rsts: *mut reset_control,
    pub clks: [*mut clk; 2],
}

//
// common
//
extern "C" {
    pub fn usbhs_read(priv: *mut usbhs_priv, reg: u32) -> u16;
}
extern "C" {
    pub fn usbhs_write(priv: *mut usbhs_priv, reg: u32, data: u16);
}
extern "C" {
    pub fn usbhs_bset(priv: *mut usbhs_priv, reg: u32, mask: u16, data: u16);
}

extern "C" {
    pub fn usbhs_get_id_as_gadget(pdev: *mut platform_device) -> c_int;
}
//
// sysconfig
//
extern "C" {
    pub fn usbhs_sys_host_ctrl(priv: *mut usbhs_priv, enable: c_int);
}
extern "C" {
    pub fn usbhs_sys_function_ctrl(priv: *mut usbhs_priv, enable: c_int);
}
extern "C" {
    pub fn usbhs_sys_function_pullup(priv: *mut usbhs_priv, enable: c_int);
}
extern "C" {
    pub fn usbhs_sys_set_test_mode(priv: *mut usbhs_priv, mode: u16);
}
//
// usb request
//
extern "C" {
    pub fn usbhs_usbreq_get_val(priv: *mut usbhs_priv, req: *mut usb_ctrlrequest);
}
extern "C" {
    pub fn usbhs_usbreq_set_val(priv: *mut usbhs_priv, req: *mut usb_ctrlrequest);
}
//
// bus
//
extern "C" {
    pub fn usbhs_bus_send_sof_enable(priv: *mut usbhs_priv);
}
extern "C" {
    pub fn usbhs_bus_send_reset(priv: *mut usbhs_priv);
}
extern "C" {
    pub fn usbhs_bus_get_speed(priv: *mut usbhs_priv) -> c_int;
}
extern "C" {
    pub fn usbhs_vbus_ctrl(priv: *mut usbhs_priv, enable: c_int) -> c_int;
}
extern "C" {
    pub fn usbhsc_schedule_notify_hotplug(pdev: *mut platform_device) -> c_int;
}
//
// frame
//
extern "C" {
    pub fn usbhs_frame_get_num(priv: *mut usbhs_priv) -> c_int;
}
//
// device config
//
// interrupt functions
//
extern "C" {
    pub fn usbhs_xxxsts_clear(priv: *mut usbhs_priv, sts_reg: u16, bit: u16);
}
//
// data
//

