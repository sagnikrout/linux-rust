//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/hfi1/opa_compat.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
//
// Copyright(c) 2015, 2016 Intel Corporation.
//
// This header file is for OPA-specific definitions which are
// required by the HFI driver, and which aren't yet in the Linux
// IB core. We'll collect these all here, then merge them into
// the kernel when that's convenient.
//
// OPA SMA attribute IDs

// OPA PMA attribute IDs

// OPA status codes

//
// OPA port physical states
// IB Volume 1, Table 146 PortInfo/IB Volume 2 Section 5.4.2(1) PortPhysState
// values are the same in OmniPath Architecture. OPA leverages some of the same
// concepts as InfiniBand, but has a few other states as well.
//
// When writing, only values 0-3 are valid, other values are ignored.
// When reading, 0 is reserved.
//
// Returned by the ibphys_portstate() routine.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum opa_port_phys_state {
// Values 0-7 have the same meaning in OPA as in InfiniBand.

    IB_PORTPHYSSTATE_NOP = 0,
// 1 is reserved
    IB_PORTPHYSSTATE_POLLING = 2,
    IB_PORTPHYSSTATE_DISABLED = 3,
    IB_PORTPHYSSTATE_TRAINING = 4,
    IB_PORTPHYSSTATE_LINKUP = 5,
    IB_PORTPHYSSTATE_LINK_ERROR_RECOVERY = 6,
    IB_PORTPHYSSTATE_PHY_TEST = 7,
// 8 is reserved

//
// Offline: Port is quiet (transmitters disabled) due to lack of
// physical media, unsupported media, or transition between link up
// and next link up attempt
//
    OPA_PORTPHYSSTATE_OFFLINE = 9,

// 10 is reserved

//
// Phy_Test: Specific test patterns are transmitted, and receiver BER
// can be monitored. This facilitates signal integrity testing for the
// physical layer of the port.
//
    OPA_PORTPHYSSTATE_TEST = 11,

    OPA_PORTPHYSSTATE_MAX = 11,
// values 12-15 are reserved/ignored
}
