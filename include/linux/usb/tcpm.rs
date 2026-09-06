//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/usb/tcpm.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright 2015-2017 Google, Inc
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum typec_cc_status {
    TYPEC_CC_OPEN,
    TYPEC_CC_RA,
    TYPEC_CC_RD,
    TYPEC_CC_RP_DEF,
    TYPEC_CC_RP_1_5,
    TYPEC_CC_RP_3_0,
}

// Collision Avoidance

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum typec_cc_polarity {
    TYPEC_POLARITY_CC1,
    TYPEC_POLARITY_CC2,
}

// Time to wait for TCPC to complete transmit

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tcpm_transmit_status {
    TCPC_TX_SUCCESS = 0,
    TCPC_TX_DISCARDED = 1,
    TCPC_TX_FAILED = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tcpm_transmit_type {
    TCPC_TX_SOP = 0,
    TCPC_TX_SOP_PRIME = 1,
    TCPC_TX_SOP_PRIME_PRIME = 2,
    TCPC_TX_SOP_DEBUG_PRIME = 3,
    TCPC_TX_SOP_DEBUG_PRIME_PRIME = 4,
    TCPC_TX_HARD_RESET = 5,
    TCPC_TX_CABLE_RESET = 6,
    TCPC_TX_BIST_MODE_2 = 7
}

// Mux state attributes

//
// struct tcpc_dev - Port configuration and callback functions
// @fwnode:	Pointer to port fwnode
// @get_vbus:	Called to read current VBUS state
// @get_current_limit:
// Optional; called by the tcpm core when configured as a snk
// and cc=Rp-def. This allows the tcpm to provide a fallback
// current-limit detection method for the cc=Rp-def case.
// For example, some tcpcs may include BC1.2 charger detection
// and use that in this case.
// @set_cc:	Called to set value of CC pins
// @apply_rc:	Optional; Needed to move TCPCI based chipset to APPLY_RC state
// as stated by the TCPCI specification.
// @get_cc:	Called to read current CC pin values
// @set_polarity:
// Called to set polarity
// @set_vconn:	Called to enable or disable VCONN
// @set_vbus:	Called to enable or disable VBUS
// @set_current_limit:
// Optional; called to set current limit as negotiated
// with partner.
// @set_pd_rx:	Called to enable or disable reception of PD messages
// @set_roles:	Called to set power and data roles
// @start_toggling:
// Optional; if supported by hardware, called to start dual-role
// toggling or single-role connection detection. Toggling stops
// automatically if a connection is established.
// @try_role:	Optional; called to set a preferred role
// @pd_transmit:Called to transmit PD message
// @set_bist_data: Turn on/off bist data mode for compliance testing
// @enable_frs:
// Optional; Called to enable/disable PD 3.0 fast role swap.
// Enabling frs is accessory dependent as not all PD3.0
// accessories support fast role swap.
// @frs_sourcing_vbus:
// Optional; Called to notify that vbus is now being sourced.
// Low level drivers can perform chip specific operations, if any.
// @enable_auto_vbus_discharge:
// Optional; TCPCI spec based TCPC implementations can optionally
// support hardware to autonomously dischrge vbus upon disconnecting
// as sink or source. TCPM signals TCPC to enable the mechanism upon
// entering connected state and signals disabling upon disconnect.
// @set_auto_vbus_discharge_threshold:
// Mandatory when enable_auto_vbus_discharge is implemented. TCPM
// calls this function to allow lower levels drivers to program the
// vbus threshold voltage below which the vbus discharge circuit
// will be turned on. requested_vbus_voltage is set to 0 when vbus
// is going to disappear knowingly i.e. during PR_SWAP and
// HARD_RESET etc.
// @is_vbus_vsafe0v:
// Optional; TCPCI spec based TCPC implementations are expected to
// detect VSAFE0V voltage level at vbus. When detection of VSAFE0V
// is supported by TCPC, set this callback for TCPM to query
// whether vbus is at VSAFE0V when needed.
// Returns true when vbus is at VSAFE0V, false otherwise.
// @set_partner_usb_comm_capable:
// Optional; The USB Communications Capable bit indicates if port
// partner is capable of communication over the USB data lines
// (e.g. D+/- or SS Tx/Rx). Called to notify the status of the bit.
// @check_contaminant:
// Optional; The callback is called when CC pins report open status
// at the end of the deboumce period or when the port is still
// toggling. Chip level drivers are expected to check for contaminant
// and call tcpm_clean_port when the port is clean.
// @cable_comm_capable
// Optional; Returns whether cable communication over SOP' is supported
// by the tcpc
// @attempt_vconn_swap_discovery:
// Optional; The callback is called by the TCPM when the result of
// a Discover Identity request indicates that the port partner is
// a receptacle capable of modal operation. Chip level TCPCI drivers
// can implement their own policy to determine if and when a Vconn
// swap following Discover Identity on SOP' occurs.
// Return true when the TCPM is allowed to request a Vconn swap
// after Discovery Identity on SOP.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcpc_dev {
    pub fwnode: *mut fwnode_handle,
    pub dev): *mut *mut int (init)(struct tcpc_dev,
    pub dev): *mut *mut int (get_vbus)(struct tcpc_dev,
    pub dev): *mut *mut int (get_current_limit)(struct tcpc_dev,
    pub cc): *mut *mut *mut int (set_cc)(struct tcpc_dev dev, enum typec_cc_status,
    pub polarity): typec_cc_polarity,
    pub cc2): *mut typec_cc_status,
    pub polarity): typec_cc_polarity,
    pub orientation): typec_orientation,
    pub on): *mut *mut *mut int (set_vconn)(struct tcpc_dev dev, bool,
    pub charge): *mut *mut *mut int (set_vbus)(struct tcpc_dev dev, bool on, bool,
    pub mv): *mut *mut *mut int (set_current_limit)(struct tcpc_dev dev, u32 max_ma, u32,
    pub on): *mut *mut *mut int (set_pd_rx)(struct tcpc_dev dev, bool,
    pub data): typec_role role, typec_data_role,
    pub cc): typec_cc_status,
    pub role): *mut *mut *mut int (try_role)(struct tcpc_dev dev, int,
    pub negotiated_rev): *const *const pd_message msg, unsigned int,
    pub on): *mut *mut *mut int (set_bist_data)(struct tcpc_dev dev, bool,
    pub enable): *mut *mut *mut int (enable_frs)(struct tcpc_dev dev, bool,
    pub dev): *mut *mut void (frs_sourcing_vbus)(struct tcpc_dev,
    pub enable): *mut *mut *mut int (enable_auto_vbus_discharge)(struct tcpc_dev dev, bool,
    pub pps_apdo_min_voltage): u32,
    pub dev): *mut *mut bool (is_vbus_vsafe0v)(struct tcpc_dev,
    pub enable): *mut *mut *mut void (set_partner_usb_comm_capable)(struct tcpc_dev dev, bool,
    pub dev): *mut *mut void (check_contaminant)(struct tcpc_dev,
    pub dev): *mut *mut bool (cable_comm_capable)(struct tcpc_dev,
    pub dev): *mut *mut bool (attempt_vconn_swap_discovery)(struct tcpc_dev,
}

extern "C" {
    pub fn tcpm_unregister_port(port: *mut tcpm_port);
}
extern "C" {
    pub fn tcpm_vbus_change(port: *mut tcpm_port);
}
extern "C" {
    pub fn tcpm_cc_change(port: *mut tcpm_port);
}
extern "C" {
    pub fn tcpm_sink_frs(port: *mut tcpm_port);
}
extern "C" {
    pub fn tcpm_sourcing_vbus(port: *mut tcpm_port);
}
extern "C" {
    pub fn tcpm_pd_hard_reset(port: *mut tcpm_port);
}
extern "C" {
    pub fn tcpm_tcpc_reset(port: *mut tcpm_port);
}
extern "C" {
    pub fn tcpm_port_clean(port: *mut tcpm_port);
}
extern "C" {
    pub fn tcpm_port_is_toggling(port: *mut tcpm_port) -> bool;
}
extern "C" {
    pub fn tcpm_port_error_recovery(port: *mut tcpm_port);
}
