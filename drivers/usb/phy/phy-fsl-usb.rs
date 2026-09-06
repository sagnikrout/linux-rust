//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/phy/phy-fsl-usb.h
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
// Copyright (C) 2007,2008 Freescale Semiconductor, Inc.

// USB Command Register Bit Masks

// bit 15,3,2 are frame list size

// bit 9-8 are async schedule park mode count

// bit 23-16 are interrupt threshold control

// USB Status Register Bit Masks

// USB Interrupt Enable Register Bit Masks

// Device Address bit masks

// PORTSC  Register Bit Masks,Only one PORT in OTG mode

// bit 11-10 are line status

// bit 15-14 are port indicator control

// bit 19-16 are port test control

// bit 27-26 are port speed

// bit 28 is parallel transceiver width for UTMI interface

// bit 31-30 are port transceiver select

// OTG Status Control Register Bit Masks

// USB MODE Register Bit Masks

// control Register Bit Masks

// BCSR5

// USB module clk cfg

// OTG interrupt enable bit masks

// OTG interrupt status bit masks

//
// A-DEVICE timing  constants
//
// Wait for VBUS Rise

// Wait for B-Connect

// This is only used to get out of
// OTG_STATE_A_WAIT_BCON state if there was
// no connection for these many milliseconds
//
// A-Idle to B-Disconnect
// It is necessary for this timer to be more than 750 ms because of a bug in OPT
// test 5.4 in which B OPT disconnects after 750 ms instead of 75ms as stated
// in the test description
//

// B-Idle to A-Disconnect

// B-device timing constants
// Data-Line Pulse Time

// SRP Initiate Time

// SRP Fail Time

// SRP result wait time

// VBus time

// Discharge time
// This time should be less than 10ms. It varies from system to system.

// A-SE0 to B-Reset

// A bus suspend timer before we can switch to b_wait_aconn

// SE0 Time Before SRP

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_dr_mmap {
// Capability register
    pub res1: [u8; 256],
    pub /: *mut *mut u16 caplength; / Capability Register Length,
    pub /: *mut *mut u16 hciversion; / Host Controller Interface Version,
    pub /: *mut *mut u32 hcsparams; / Host Controller Structual Parameters,
    pub /: *mut *mut u32 hccparams; / Host Controller Capability Parameters,
    pub res2: [u8; 20],
    pub /: *mut *mut u32 dciversion; / Device Controller Interface Version,
    pub /: *mut *mut u32 dccparams; / Device Controller Capability Parameters,
    pub res3: [u8; 24],
// Operation register
    pub /: *mut *mut u32 usbcmd; / USB Command Register,
    pub /: *mut *mut u32 usbsts; / USB Status Register,
    pub /: *mut *mut u32 usbintr; / USB Interrupt Enable Register,
    pub /: *mut *mut u32 frindex; / Frame Index Register,
    pub res4: [u8; 4],
    pub /: *mut *mut u32 deviceaddr; / Device Address,
    pub /: *mut *mut u32 endpointlistaddr; / Endpoint List Address Register,
    pub res5: [u8; 4],
    pub /: *mut *mut u32 burstsize; / Master Interface Data Burst Size Register,
    pub /: *mut *mut u32 txttfilltuning; / Transmit FIFO Tuning Controls Register,
    pub res6: [u8; 8],
    pub /: *mut *mut u32 ulpiview; / ULPI register access,
    pub res7: [u8; 12],
    pub /: *mut *mut u32 configflag; / Configure Flag Register,
    pub /: *mut *mut u32 portsc; / Port 1 Status and Control Register,
    pub res8: [u8; 28],
    pub /: *mut *mut u32 otgsc; / On-The-Go Status and Control,
    pub /: *mut *mut u32 usbmode; / USB Mode Register,
    pub /: *mut *mut u32 endptsetupstat; / Endpoint Setup Status Register,
    pub /: *mut *mut u32 endpointprime; / Endpoint Initialization Register,
    pub /: *mut *mut u32 endptflush; / Endpoint Flush Register,
    pub /: *mut *mut u32 endptstatus; / Endpoint Status Register,
    pub /: *mut *mut u32 endptcomplete; / Endpoint Complete Register,
    pub /: *mut *mut u32 endptctrl[6]; / Endpoint Control Registers,
    pub res9: [u8; 552],
    pub snoop1: u32,
    pub snoop2: u32,
    pub /: *mut *mut u32 age_cnt_thresh; / Age Count Threshold Register,
    pub /: *mut *mut u32 pri_ctrl; / Priority Control Register,
    pub /: *mut *mut u32 si_ctrl; / System Interface Control Register,
    pub res10: [u8; 236],
    pub /: *mut *mut u32 control; / General Purpose Control Register,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_otg_timer {
    pub /: *mut *mut unsigned long expires; / Number of count increase to timeout,
    pub /: *mut *mut unsigned long count; / Tick counter,
    pub /: *mut *mut *mut void (function)(unsigned long); / Timeout function,
    pub /: *mut *mut unsigned long data; / Data passed to function,
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_otg {
    pub phy: usb_phy,
    pub fsm: otg_fsm,
    pub dr_mem_map: *mut usb_dr_mmap,
    pub otg_event: delayed_work,
// used for usb host
    pub work_wq: work_struct,
    pub host_working: u8,
    pub irq: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_otg_config {
    pub otg_port: u8,
}

extern "C" {
    pub fn fsl_otg_add_timer(fsm: *mut otg_fsm, timer: *mut c_void) -> static void;
}
extern "C" {
    pub fn fsl_otg_del_timer(fsm: *mut otg_fsm, timer: *mut c_void) -> static void;
}
extern "C" {
    pub fn fsl_otg_pulse_vbus() -> static void;
}
