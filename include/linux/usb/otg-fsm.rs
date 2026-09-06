//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/usb/otg-fsm.h
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
// Copyright (C) 2007,2008 Freescale Semiconductor, Inc.
//

pub const OTG_STS_SELECTOR: c_uint = 0xF000	/* OTG status selector, according to;
// OTG and EH 2.0 Chapter 6.2.3
// Table:6-4
//

// OTG and EH 2.0 Charpter 6.2.3
// Table:6-5
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum otg_fsm_timer {
// Standard OTG timers
    A_WAIT_VRISE,
    A_WAIT_VFALL,
    A_WAIT_BCON,
    A_AIDL_BDIS,
    B_ASE0_BRST,
    A_BIDL_ADIS,
    B_AIDL_BDIS,

// Auxiliary timers
    B_SE0_SRP,
    B_SRP_FAIL,
    A_WAIT_ENUM,
    B_DATA_PLS,
    B_SSEND_SRP,

    NUM_OTG_FSM_TIMERS,
}

//
// struct otg_fsm - OTG state machine according to the OTG spec
//
// OTG hardware Inputs
//
// Common inputs for A and B device
// @id:		TRUE for B-device, FALSE for A-device.
// @adp_change: TRUE when current ADP measurement (n) value, compared to the
// ADP measurement taken at n-2, differs by more than CADP_THR
// @power_up:	TRUE when the OTG device first powers up its USB system and
// ADP measurement taken if ADP capable
//
// A-Device state inputs
// @a_srp_det:	TRUE if the A-device detects SRP
// @a_vbus_vld:	TRUE when VBUS voltage is in regulation
// @b_conn:	TRUE if the A-device detects connection from the B-device
// @a_bus_resume: TRUE when the B-device detects that the A-device is signaling
// a resume (K state)
// B-Device state inputs
// @a_bus_suspend: TRUE when the B-device detects that the A-device has put the
// bus into suspend
// @a_conn:	TRUE if the B-device detects a connection from the A-device
// @b_se0_srp:	TRUE when the line has been at SE0 for more than the minimum
// time before generating SRP
// @b_ssend_srp: TRUE when the VBUS has been below VOTG_SESS_VLD for more than
// the minimum time before generating SRP
// @b_sess_vld:	TRUE when the B-device detects that the voltage on VBUS is
// above VOTG_SESS_VLD
// @test_device: TRUE when the B-device switches to B-Host and detects an OTG
// test device. This must be set by host/hub driver
//
// Application inputs (A-Device)
// @a_bus_drop:	TRUE when A-device application needs to power down the bus
// @a_bus_req:	TRUE when A-device application wants to use the bus.
// FALSE to suspend the bus
//
// Application inputs (B-Device)
// @b_bus_req:	TRUE during the time that the Application running on the
// B-device wants to use the bus
//
// Auxiliary inputs (OTG v1.3 only. Obsolete now.)
// @a_sess_vld:	TRUE if the A-device detects that VBUS is above VA_SESS_VLD
// @b_bus_suspend: TRUE when the A-device detects that the B-device has put
// the bus into suspend
// @b_bus_resume: TRUE when the A-device detects that the B-device is signaling
// resume on the bus
//
// OTG Output status. Read only for users. Updated by OTG FSM helpers defined
// in this file
//
// Outputs for Both A and B device
// @drv_vbus:	TRUE when A-device is driving VBUS
// @loc_conn:	TRUE when the local device has signaled that it is connected
// to the bus
// @loc_sof:	TRUE when the local device is generating activity on the bus
// @adp_prb:	TRUE when the local device is in the process of doing
// ADP probing
//
// Outputs for B-device state
// @adp_sns:	TRUE when the B-device is in the process of carrying out
// ADP sensing
// @data_pulse: TRUE when the B-device is performing data line pulsing
//
// Internal Variables
//
// a_set_b_hnp_en: TRUE when the A-device has successfully set the
// b_hnp_enable bit in the B-device.
// Unused as OTG fsm uses otg->host->b_hnp_enable instead
// b_srp_done:	TRUE when the B-device has completed initiating SRP
// b_hnp_enable: TRUE when the B-device has accepted the
// SetFeature(b_hnp_enable) B-device.
// Unused as OTG fsm uses otg->gadget->b_hnp_enable instead
// a_clr_err:	Asserted (by application ?) to clear a_vbus_err due to an
// overcurrent condition and causes the A-device to transition
// to a_wait_vfall
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct otg_fsm {
// Input
    pub id: c_int,
    pub adp_change: c_int,
    pub power_up: c_int,
    pub a_srp_det: c_int,
    pub a_vbus_vld: c_int,
    pub b_conn: c_int,
    pub a_bus_resume: c_int,
    pub a_bus_suspend: c_int,
    pub a_conn: c_int,
    pub b_se0_srp: c_int,
    pub b_ssend_srp: c_int,
    pub b_sess_vld: c_int,
    pub test_device: c_int,
    pub a_bus_drop: c_int,
    pub a_bus_req: c_int,
    pub b_bus_req: c_int,
// Auxiliary inputs
    pub a_sess_vld: c_int,
    pub b_bus_resume: c_int,
    pub b_bus_suspend: c_int,
// Output
    pub drv_vbus: c_int,
    pub loc_conn: c_int,
    pub loc_sof: c_int,
    pub adp_prb: c_int,
    pub adp_sns: c_int,
    pub data_pulse: c_int,
// Internal variables
    pub a_set_b_hnp_en: c_int,
    pub b_srp_done: c_int,
    pub b_hnp_enable: c_int,
    pub a_clr_err: c_int,
// Informative variables. All unused as of now
    pub a_bus_drop_inf: c_int,
    pub a_bus_req_inf: c_int,
    pub a_clr_err_inf: c_int,
    pub b_bus_req_inf: c_int,
// Auxiliary informative variables
    pub a_suspend_req_inf: c_int,
// Timeout indicator for timers
    pub a_wait_vrise_tmout: c_int,
    pub a_wait_vfall_tmout: c_int,
    pub a_wait_bcon_tmout: c_int,
    pub a_aidl_bdis_tmout: c_int,
    pub b_ase0_brst_tmout: c_int,
    pub a_bidl_adis_tmout: c_int,
    pub ops: *mut otg_fsm_ops,
    pub otg: *mut usb_otg,
// Current usb protocol used: 0:undefine; 1:host; 2:client
    pub protocol: c_int,
    pub lock: mutex,
    pub host_req_flag: *mut u8,
    pub hnp_polling_work: delayed_work,
    pub hnp_work_inited: bool,
    pub state_changed: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct otg_fsm_ops {
    pub on): *mut *mut *mut void (chrg_vbus)(struct otg_fsm fsm, int,
    pub on): *mut *mut *mut void (drv_vbus)(struct otg_fsm fsm, int,
    pub on): *mut *mut *mut void (loc_conn)(struct otg_fsm fsm, int,
    pub on): *mut *mut *mut void (loc_sof)(struct otg_fsm fsm, int,
    pub fsm): *mut *mut void (start_pulse)(struct otg_fsm,
    pub fsm): *mut *mut void (start_adp_prb)(struct otg_fsm,
    pub fsm): *mut *mut void (start_adp_sns)(struct otg_fsm,
    pub timer): *mut *mut *mut void (add_timer)(struct otg_fsm fsm, enum otg_fsm_timer,
    pub timer): *mut *mut *mut void (del_timer)(struct otg_fsm fsm, enum otg_fsm_timer,
    pub on): *mut *mut *mut int (start_host)(struct otg_fsm fsm, int,
    pub on): *mut *mut *mut int (start_gadget)(struct otg_fsm fsm, int,
}

extern "C" {
    pub fn otg_statemachine(fsm: *mut otg_fsm) -> c_int;
}
