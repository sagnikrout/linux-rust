//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/usb/ehci_def.h
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
//
// Copyright (c) 2001-2002 by David Brownell
//

// EHCI register interface, corresponds to EHCI Revision 0.95 specification
// Section 2.2 Host Controller Capability Registers
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ehci_caps {
// these fields are specified as 8 and 16 bit registers,
// but some hosts can't perform 8 or 16 bit PCI accesses.
// some hosts treat caplength and hciversion as parts of a 32-bit
// register, others treat them as two separate registers, this
// affects the memory map for big endian controllers.
//
    pub hc_capbase: u32,

    pub /: *mut *mut u32 hcs_params; / HCSPARAMS - offset 0x4,

    pub /: *mut *mut u32 hcc_params; / HCCPARAMS - offset 0x8,
// EHCI 1.1 addendum

    pub /: *mut *mut u8 portroute[8]; / nibbles for routing - offset 0xC,
}

// Section 2.3 Host Controller Operational Registers
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ehci_regs {
// USBCMD: offset 0x00
    pub command: u32,
// EHCI 1.1 addendum

// 23:16 is r/w intr rate, in microframes; default "8" == 1/msec

// 3:2 is periodic frame list size

// USBSTS: offset 0x04
    pub status: u32,

// some bits reserved
// these STS_* flags are also intr_enable bits (USBINTR)

// USBINTR: offset 0x08
    pub intr_enable: u32,
// FRINDEX: offset 0x0C
    pub /: *mut *mut u32 frame_index; / current microframe number,
// CTRLDSSEGMENT: offset 0x10
    pub /: *mut *mut u32 segment; / address bits 63:32 if needed,
// PERIODICLISTBASE: offset 0x14
    pub /: *mut *mut u32 frame_list; / points to periodic list,
// ASYNCLISTADDR: offset 0x18
    pub /: *mut *mut u32 async_next; / address of next async queue head,
    pub reserved1: [u32; 2],
// TXFILLTUNING: offset 0x24
    pub /: *mut *mut u32 txfill_tuning; / TX FIFO Tuning register,
    pub reserved2: [u32; 6],
// CONFIGFLAG: offset 0x40
    pub configured_flag: u32,

// PORTSC: offset 0x44
    pub /: *mut *mut u32 port_status[HCS_N_PORTS_MAX]; / up to N_PORTS,
// EHCI 1.1 addendum
pub const PORTSC_SUSPEND_STS_ACK: c_int = 0;
pub const PORTSC_SUSPEND_STS_NYET: c_int = 1;
pub const PORTSC_SUSPEND_STS_STALL: c_int = 2;
pub const PORTSC_SUSPEND_STS_ERR: c_int = 3;

// 31:23 reserved

// 19:16 for port testing

// 9 reserved
    pub reserved3: [u32; 9],
// USBMODE: offset 0x68
    pub /: *mut *mut u32 usbmode; / USB Device mode,
}

// Moorestown has some non-standard registers, partially due to the fact that
// its EHCI controller has both TT and LPM support. HOSTPCx are extensions to
// PORTSCx
//
// HOSTPC: offset 0x84

// Broadcom-proprietary USB_EHCI_INSNREG00 @ 0x80
// USBMODE_EX: offset 0xc8

