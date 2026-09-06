//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/phy/phy-mv-usb.h
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
// Copyright (C) 2011 Marvell International Ltd. All rights reserved.
//

// Command Register Bit Masks

// otgsc Register Bit Masks
pub const OTGSC_CTRL_VUSB_DISCHARGE: c_uint = 0x00000001;
pub const OTGSC_CTRL_VUSB_CHARGE: c_uint = 0x00000002;
pub const OTGSC_CTRL_OTG_TERM: c_uint = 0x00000008;
pub const OTGSC_CTRL_DATA_PULSING: c_uint = 0x00000010;
pub const OTGSC_STS_USB_ID: c_uint = 0x00000100;
pub const OTGSC_STS_A_VBUS_VALID: c_uint = 0x00000200;
pub const OTGSC_STS_A_SESSION_VALID: c_uint = 0x00000400;
pub const OTGSC_STS_B_SESSION_VALID: c_uint = 0x00000800;
pub const OTGSC_STS_B_SESSION_END: c_uint = 0x00001000;
pub const OTGSC_STS_1MS_TOGGLE: c_uint = 0x00002000;
pub const OTGSC_STS_DATA_PULSING: c_uint = 0x00004000;
pub const OTGSC_INTSTS_USB_ID: c_uint = 0x00010000;
pub const OTGSC_INTSTS_A_VBUS_VALID: c_uint = 0x00020000;
pub const OTGSC_INTSTS_A_SESSION_VALID: c_uint = 0x00040000;
pub const OTGSC_INTSTS_B_SESSION_VALID: c_uint = 0x00080000;
pub const OTGSC_INTSTS_B_SESSION_END: c_uint = 0x00100000;
pub const OTGSC_INTSTS_1MS: c_uint = 0x00200000;
pub const OTGSC_INTSTS_DATA_PULSING: c_uint = 0x00400000;
pub const OTGSC_INTR_USB_ID: c_uint = 0x01000000;
pub const OTGSC_INTR_A_VBUS_VALID: c_uint = 0x02000000;
pub const OTGSC_INTR_A_SESSION_VALID: c_uint = 0x04000000;
pub const OTGSC_INTR_B_SESSION_VALID: c_uint = 0x08000000;
pub const OTGSC_INTR_B_SESSION_END: c_uint = 0x10000000;
pub const OTGSC_INTR_1MS_TIMER: c_uint = 0x20000000;
pub const OTGSC_INTR_DATA_PULSING: c_uint = 0x40000000;

// Timer's interval, unit 10ms
pub const T_A_WAIT_VRISE: c_int = 100;
pub const T_A_WAIT_BCON: c_int = 2000;
pub const T_A_AIDL_BDIS: c_int = 100;
pub const T_A_BIDL_ADIS: c_int = 20;
pub const T_B_ASE0_BRST: c_int = 400;
pub const T_B_SE0_SRP: c_int = 300;
pub const T_B_SRP_FAIL: c_int = 2000;
pub const T_B_DATA_PLS: c_int = 10;
pub const T_B_SRP_INIT: c_int = 100;
pub const T_A_SRP_RSPNS: c_int = 10;
pub const T_A_DRV_RSM: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum otg_function {
    OTG_B_DEVICE = 0,
    OTG_A_DEVICE
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mv_otg_timer {
    A_WAIT_BCON_TIMER = 0,
    OTG_TIMER_NUM
}

// PXA OTG state machine
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mv_otg_ctrl {
// internal variables
    pub /: *mut *mut u8 a_set_b_hnp_en; / A-Device set b_hnp_en,
    pub b_srp_done: u8,
    pub b_hnp_en: u8,
// OTG inputs
    pub a_bus_drop: u8,
    pub a_bus_req: u8,
    pub a_clr_err: u8,
    pub a_bus_resume: u8,
    pub a_bus_suspend: u8,
    pub a_conn: u8,
    pub a_sess_vld: u8,
    pub a_srp_det: u8,
    pub a_vbus_vld: u8,
    pub /: *mut *mut u8 b_bus_req; / B-Device Require Bus,
    pub b_bus_resume: u8,
    pub b_bus_suspend: u8,
    pub b_conn: u8,
    pub b_se0_srp: u8,
    pub b_sess_end: u8,
    pub b_sess_vld: u8,
    pub id: u8,
    pub a_suspend_req: u8,
// Timer event
    pub a_aidl_bdis_timeout: u8,
    pub b_ase0_brst_timeout: u8,
    pub a_bidl_adis_timeout: u8,
    pub a_wait_bcon_timeout: u8,
    pub timer: [timer_list; OTG_TIMER_NUM],
}

pub const VUSBHS_MAX_PORTS: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mv_otg_regs {
    pub /: *mut *mut u32 usbcmd; / Command register,
    pub /: *mut *mut u32 usbsts; / Status register,
    pub /: *mut *mut u32 usbintr; / Interrupt enable,
    pub /: *mut *mut u32 frindex; / Frame index,
    pub reserved1: [u32; 1],
    pub /: *mut *mut u32 deviceaddr; / Device Address,
    pub /: *mut *mut u32 eplistaddr; / Endpoint List Address,
    pub /: *mut *mut u32 ttctrl; / HOST TT status and control,
    pub /: *mut *mut u32 burstsize; / Programmable Burst Size,
    pub /: *mut *mut u32 txfilltuning; / Host Transmit Pre-Buffer Packet Tuning,
    pub reserved: [u32; 4],
    pub /: *mut *mut u32 epnak; / Endpoint NAK,
    pub /: *mut *mut u32 epnaken; / Endpoint NAK Enable,
    pub /: *mut *mut u32 configflag; / Configured Flag register,
    pub /: *mut *mut u32 portsc[VUSBHS_MAX_PORTS]; / Port Status/Control x, x = 1..8,
    pub otgsc: u32,
    pub /: *mut *mut u32 usbmode; / USB Host/Device mode,
    pub /: *mut *mut u32 epsetupstat; / Endpoint Setup Status,
    pub /: *mut *mut u32 epprime; / Endpoint Initialize,
    pub /: *mut *mut u32 epflush; / Endpoint De-initialize,
    pub /: *mut *mut u32 epstatus; / Endpoint Status,
    pub /: *mut *mut u32 epcomplete; / Endpoint Interrupt On Complete,
    pub /: *mut *mut u32 epctrlx[16]; / Endpoint Control, where x = 0.. 15,
    pub /: *mut *mut u32 mcr; / Mux Control,
    pub /: *mut *mut u32 isr; / Interrupt Status,
    pub /: *mut *mut u32 ier; / Interrupt Enable,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mv_otg {
    pub phy: usb_phy,
    pub otg_ctrl: mv_otg_ctrl,
// base address
    pub phy_regs: *mut void __iomem,
    pub cap_regs: *mut void __iomem,
    pub op_regs: *mut mv_otg_regs __iomem,
    pub pdev: *mut platform_device,
    pub irq: c_int,
    pub irq_status: u32,
    pub irq_en: u32,
    pub work: delayed_work,
    pub qwork: *mut workqueue_struct,
    pub wq_lock: spinlock_t,
    pub pdata: *mut mv_usb_platform_data,
    pub active: c_uint,
    pub clock_gating: c_uint,
    pub clk: *mut clk,
}
