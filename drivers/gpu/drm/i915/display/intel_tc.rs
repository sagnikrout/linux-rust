//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/display/intel_tc.h
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
//
// Copyright © 2019 Intel Corporation
//

//
// The following enum values must stay fixed, as they match the corresponding
// pin assignment fields in the PORT_TX_DFLEXPA1 and TCSS_DDI_STATUS registers.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum intel_tc_pin_assignment {
// DP    USB   Rate (b)  type
    INTEL_TC_PIN_ASSIGNMENT_NONE = 0, /* 4     -     -         -       (c)     */
    INTEL_TC_PIN_ASSIGNMENT_A,        /* 2/4   0     GEN2      TC->TC  (d,e)   */
    INTEL_TC_PIN_ASSIGNMENT_B,        /* 1/2   1     GEN2      TC->TC  (d,f,g) */
    INTEL_TC_PIN_ASSIGNMENT_C,        /* 4     0     DP2       TC->TC  (h)     */
    INTEL_TC_PIN_ASSIGNMENT_D,        /* 2     1     DP2       TC->TC  (h,g)   */
    INTEL_TC_PIN_ASSIGNMENT_E,        /* 4     0     DP2       TC->DP          */
    INTEL_TC_PIN_ASSIGNMENT_F,        /* 2     1     GEN1/DP1  TC->DP  (d,g,i) */
//
// (a) - DP unidirectional lanes, each lane using 1 differential signal
// pair.
// - USB SuperSpeed bidirectional lane, using 2 differential (TX and
// RX) signal pairs.
// - USB 2.0 (HighSpeed) unidirectional lane, using 1 differential
// signal pair. Not indicated, this lane is always present on pin
// assignments A-D and never present on pin assignments E/F.
// (b) - GEN1: USB 3.1 GEN1 bit rate (5 Gbps) and signaling. This
// is used for transferring only a USB stream.
// - GEN2: USB 3.1 GEN2 bit rate (10 Gbps) and signaling. This
// allows transferring an HBR3 (8.1 Gbps) DP stream.
// - DP1:  Display Port signaling defined by the DP v1.3 Standard,
// with a maximum bit rate of HBR3.
// - DP2:  Display Port signaling defined by the DP v2.1 Standard,
// with a maximum bit rate defined by the DP Alt Mode
// v2.1a Standard depending on the cable type as follows:
// - Passive (Full-Featured) USB 3.2 GEN1
// TC->TC cables (CC3G1-X)                        : UHBR10
// - Passive (Full-Featured) USB 3.2/4 GEN2 and
// Thunderbolt Alt Mode GEN2
// TC->TC cables (CC3G2-X)                    all : UHBR10
// DP54 logo : UHBR13.5
// - Passive (Full-Featured) USB4 GEN3+ and
// Thunderbolt Alt Mode GEN3+
// TC->TC cables (CC4G3-X)                    all : UHBR13.5
// DP80 logo : UHBR20
// - Active Re-Timed or
// Active Linear Re-driven (LRD)
// USB3.2 GEN1/2 and USB4 GEN2+
// TC->TC cables                              all : HBR3
// with DP_BR CTS : UHBR10
// DP54 logo : UHBR13.5
// DP80 logo : UHBR20
// - Passive/Active Re-Timed or
// Active Linear Re-driven (LRD)
// TC->DP cables         with DP_BR CTS/DP8K logo : HBR3
// with DP_BR CTS : UHBR10
// DP54 logo : UHBR13.5
// DP80 logo : UHBR20
// (c) Used in TBT-alt/legacy modes and on LNL+ after the sink
// disconnected in DP-alt mode.
// (d) Only defined by the DP Alt Standard v1.0a, deprecated by v1.0b,
// only supported on ICL.
// (e) GEN2 passive 1 m cable: 4 DP lanes, GEN2 active cable: 2 DP lanes.
// (f) GEN2 passive 1 m cable: 2 DP lanes, GEN2 active cable: 1 DP lane.
// (g) These pin assignments are also referred to as (USB/DP)
// multifunction or Multifunction Display Port (MFD) modes.
// (h) Also used where one end of the cable is a captive connector,
// attached to a DP->HDMI/DVI/VGA converter.
// (i) The DP end of the cable is a captive connector attached to a
// (DP/USB) multifunction dock as defined by the DockPort v1.0a
// specification.
//
}

extern "C" {
    pub fn intel_tc_port_in_tbt_alt_mode(dig_port: *mut intel_digital_port) -> bool;
}
extern "C" {
    pub fn intel_tc_port_in_dp_alt_mode(dig_port: *mut intel_digital_port) -> bool;
}
extern "C" {
    pub fn intel_tc_port_in_legacy_mode(dig_port: *mut intel_digital_port) -> bool;
}
extern "C" {
    pub fn intel_tc_port_handles_hpd_glitches(dig_port: *mut intel_digital_port) -> bool;
}
extern "C" {
    pub fn intel_tc_port_connected(encoder: *mut intel_encoder) -> bool;
}
extern "C" {
    pub fn intel_tc_port_max_lane_count(dig_port: *mut intel_digital_port) -> c_int;
}
extern "C" {
    pub fn intel_tc_port_init_mode(dig_port: *mut intel_digital_port);
}
extern "C" {
    pub fn intel_tc_port_lock(dig_port: *mut intel_digital_port);
}
extern "C" {
    pub fn intel_tc_port_unlock(dig_port: *mut intel_digital_port);
}
extern "C" {
    pub fn intel_tc_port_suspend(dig_port: *mut intel_digital_port);
}
extern "C" {
    pub fn intel_tc_port_put_link(dig_port: *mut intel_digital_port);
}
extern "C" {
    pub fn intel_tc_port_ref_held(dig_port: *mut intel_digital_port) -> bool;
}
extern "C" {
    pub fn intel_tc_port_link_needs_reset(dig_port: *mut intel_digital_port) -> bool;
}
extern "C" {
    pub fn intel_tc_port_link_reset(dig_port: *mut intel_digital_port) -> bool;
}
extern "C" {
    pub fn intel_tc_port_link_cancel_reset_work(dig_port: *mut intel_digital_port);
}
extern "C" {
    pub fn intel_tc_port_init(dig_port: *mut intel_digital_port, is_legacy: bool) -> c_int;
}
extern "C" {
    pub fn intel_tc_port_cleanup(dig_port: *mut intel_digital_port);
}
extern "C" {
    pub fn intel_tc_cold_requires_aux_pw(dig_port: *mut intel_digital_port) -> bool;
}
extern "C" {
    pub fn intel_tc_info(p: *mut drm_printer, dig_port: *mut intel_digital_port);
}
