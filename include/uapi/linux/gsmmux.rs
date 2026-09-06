//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/gsmmux.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
// Copyright (c) 2022/23 Siemens Mobility GmbH

//
// flags definition for n_gsm
//
// Used by:
// struct gsm_config_ext.flags
// struct gsm_dlci_config.flags
//
// Forces a DLCI reset if set. Otherwise, a DLCI reset is only done if
// incompatible settings were provided. Always cleared on retrieval.
//

//
// struct gsm_config - n_gsm basic configuration parameters
//
// This structure is used in combination with GSMIOC_GETCONF and GSMIOC_SETCONF
// to retrieve and set the basic parameters of an n_gsm ldisc.
// struct gsm_config_ext can be used to configure extended ldisc parameters.
//
// All timers are in units of 1/100th of a second.
//
// @adaption:      Convergence layer type
// @encapsulation: Framing (0 = basic option, 1 = advanced option)
// @initiator:     Initiator or responder
// @t1:            Acknowledgment timer
// @t2:            Response timer for multiplexer control channel
// @t3:            Response timer for wake-up procedure
// @n2:            Maximum number of retransmissions
// @mru:           Maximum incoming frame payload size
// @mtu:           Maximum outgoing frame payload size
// @k:             Window size
// @i:             Frame type (1 = UIH, 2 = UI)
// @unused:        Can not be used
//

//
// struct gsm_netconfig - n_gsm network configuration parameters
//
// This structure is used in combination with GSMIOC_ENABLE_NET and
// GSMIOC_DISABLE_NET to enable or disable a network data connection
// over a mux virtual tty channel. This is for modems that support
// data connections with raw IP frames instead of PPP.
//
// @adaption: Adaption to use in network mode.
// @protocol: Protocol to use - only ETH_P_IP supported.
// @unused2:  Can not be used.
// @if_name:  Interface name format string.
// @unused:   Can not be used.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsm_netconfig {
    pub adaption: c_uint,
    pub protocol: c_ushort,
    pub unused2: c_ushort,
    pub if_name: [c_char; IFNAMSIZ],
    pub unused: [__u8; 28],
}

// get the base tty number for a configured gsmmux tty

//
// struct gsm_config_ext - n_gsm extended configuration parameters
//
// This structure is used in combination with GSMIOC_GETCONF_EXT and
// GSMIOC_SETCONF_EXT to retrieve and set the extended parameters of an
// n_gsm ldisc.
//
// All timers are in units of 1/100th of a second.
//
// @keep_alive:  Control channel keep-alive in 1/100th of a second (0 to disable).
// @wait_config: Wait for DLCI config before opening virtual link?
// @flags:       Mux specific flags.
// @reserved:    For future use, must be initialized to zero.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsm_config_ext {
    pub keep_alive: __u32,
    pub wait_config: __u32,
    pub flags: __u32,
    pub reserved: [__u32; 5],
}

//
// struct gsm_dlci_config - n_gsm channel configuration parameters
//
// This structure is used in combination with GSMIOC_GETCONF_DLCI and
// GSMIOC_SETCONF_DLCI to retrieve and set the channel specific parameters
// of an n_gsm ldisc.
//
// Set the channel accordingly before calling GSMIOC_GETCONF_DLCI.
//
// @channel:  DLCI (0 for the associated DLCI).
// @adaption: Convergence layer type.
// @mtu:      Maximum transfer unit.
// @priority: Priority (0 for default value).
// @i:        Frame type (1 = UIH, 2 = UI).
// @k:        Window size (0 for default value).
// @flags:    DLCI specific flags.
// @reserved: For future use, must be initialized to zero.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsm_dlci_config {
    pub channel: __u32,
    pub adaption: __u32,
    pub mtu: __u32,
    pub priority: __u32,
    pub i: __u32,
    pub k: __u32,
    pub flags: __u32,
    pub reserved: [__u32; 7],
}

