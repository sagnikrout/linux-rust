//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ipa/ipa_qmi.h
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
// Copyright (c) 2018, The Linux Foundation. All rights reserved.
// Copyright (C) 2018-2024 Linaro Ltd.
//

//
// struct ipa_qmi - QMI state associated with an IPA
// @client_handle:	Used to send an QMI requests to the modem
// @server_handle:	Used to handle QMI requests from the modem
// @modem_sq:		QMAP socket address for the modem QMI server
// @init_driver_work:	Work structure used for INIT_DRIVER message handling
// @initial_boot:	True if first boot has not yet completed
// @uc_ready:		True once DRIVER_INIT_COMPLETE request received
// @modem_ready:	True when INIT_DRIVER response received
// @indication_requested: True when INDICATION_REGISTER request received
// @indication_sent:	True when INIT_COMPLETE indication sent
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipa_qmi {
    pub client_handle: qmi_handle,
    pub server_handle: qmi_handle,
// Information used for the client handle
    pub modem_sq: sockaddr_qrtr,
    pub init_driver_work: work_struct,
// Flags used in negotiating readiness
    pub initial_boot: bool,
    pub uc_ready: bool,
    pub modem_ready: bool,
    pub indication_requested: bool,
    pub indication_sent: bool,
}

//
// ipa_qmi_setup() - Set up for QMI message exchange
// @ipa:		IPA pointer
//
// This is called at the end of ipa_setup(), to prepare for the exchange
// of QMI messages that perform a "handshake" between the AP and modem.
// When the modem QMI server announces its presence, an AP request message
// supplies operating parameters to be used to the modem, and the modem
// acknowledges receipt of those parameters.  The modem will not touch the
// IPA hardware until this handshake is complete.
//
// If the modem crashes (or shuts down) a new handshake begins when the
// modem's QMI server is started again.
//
extern "C" {
    pub fn ipa_qmi_setup(ipa: *mut ipa) -> c_int;
}
//
// ipa_qmi_teardown() - Tear down IPA QMI handles
// @ipa:		IPA pointer
//
extern "C" {
    pub fn ipa_qmi_teardown(ipa: *mut ipa);
}
