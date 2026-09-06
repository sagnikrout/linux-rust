//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/display/intel_hdcp_shim.h
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


// SPDX-License-Identifier: MIT
// Copyright © 2024 Intel Corporation

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum check_link_response {
    HDCP_LINK_PROTECTED	= 0,
    HDCP_TOPOLOGY_CHANGE,
    HDCP_LINK_INTEGRITY_FAILURE,
    HDCP_REAUTH_REQUEST
}

//
// This structure serves as a translation layer between the generic HDCP code
// and the bus-specific code. What that means is that HDCP over HDMI differs
// from HDCP over DP, so to account for these differences, we need to
// communicate with the receiver through this shim.
//
// For completeness, the 2 buses differ in the following ways:
// - DP AUX vs. DDC
// HDCP registers on the receiver are set via DP AUX for DP, and
// they are set via DDC for HDMI.
// - Receiver register offsets
// The offsets of the registers are different for DP vs. HDMI
// - Receiver register masks/offsets
// For instance, the ready bit for the KSV fifo is in a different
// place on DP vs HDMI
// - Receiver register names
// Seriously. In the DP spec, the 16-bit register containing
// downstream information is called BINFO, on HDMI it's called
// BSTATUS. To confuse matters further, DP has a BSTATUS register
// with a completely different definition.
// - KSV FIFO
// On HDMI, the ksv fifo is read all at once, whereas on DP it must
// be read 3 keys at a time
// - Aksv output
// Since Aksv is hidden in hardware, there's different procedures
// to send it over DP AUX vs DDC
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_hdcp_shim {
// Outputs the transmitter's An and Aksv values to the receiver.
    pub an): *mut *mut *mut int (write_an_aksv)(struct intel_digital_port dig_port, u8,
// Reads the receiver's key selection vector
    pub bksv): *mut *mut *mut int (read_bksv)(struct intel_digital_port dig_port, u8,
//
// Reads BINFO from DP receivers and BSTATUS from HDMI receivers. The
// definitions are the same in the respective specs, but the names are
// different. Call it BSTATUS since that's the name the HDMI spec
// uses and it was there first.
//
    pub bstatus): *mut u8,
// Determines whether a repeater is present downstream
    pub repeater_present): *mut bool,
// Reads the receiver's Ri' value
    pub ri): *mut *mut *mut int (read_ri_prime)(struct intel_digital_port dig_port, u8,
// Determines if the receiver's KSV FIFO is ready for consumption
    pub ksv_ready): *mut bool,
// Reads the ksv fifo for num_downstream devices
    pub ksv_fifo): *mut int num_downstream, u8,
// Reads a 32-bit part of V' from the receiver
    pub part): *mut int i, u32,
// Enables HDCP signalling on the port
    pub enable): bool,
// Enable/Disable stream encryption on DP MST Transport Link
    pub enable): bool,
// Ensures the link is still protected
    pub connector): *mut intel_connector,
// Detects panel's hdcp capability. This is optional for HDMI.
    pub hdcp_capable): *mut bool,
// HDCP adaptation(DP/HDMI) required on the port
    pub protocol: hdcp_wired_protocol,
// Detects whether sink is HDCP2.2 capable
    pub capable): *mut bool,
// Write HDCP2.2 messages
    pub size): *mut *mut void buf, size_t,
// Read HDCP2.2 messages
    pub size): *mut *mut u8 msg_id, void buf, size_t,
//
// Implementation of DP HDCP2.2 Errata for the communication of stream
// type to Receivers. In DP HDCP2.2 Stream type is one of the input to
// the HDCP2.2 Cipher for En/De-Cryption. Not applicable for HDMI.
//
    pub type): bool is_repeater, u8,
// Enable/Disable HDCP 2.2 stream encryption on DP MST Transport Link
    pub enable): bool,
// HDCP2.2 Link Integrity Check
    pub connector): *mut intel_connector,
// HDCP remote sink cap
    pub hdcp2_capable): *mut *mut bool hdcp_capable, bool,
}
