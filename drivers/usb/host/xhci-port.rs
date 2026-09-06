//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/host/xhci-port.h
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
// PORTSC - Port Status and Control Register - port_status_base bitmasks
// true: device connected

// true: port enabled

// bit 2 reserved and zeroed
// true: port has an over-current condition

// true: port reset signaling asserted

// Port Link State - bits 5:8
// A read gives the current link PM state of the port,
// a write with Link State Write Strobe set sets the link state.
//

// true: port has power (see HCC_PPC)

// bits 10:13 indicate device speed:
// 0 - undefined speed - port hasn't be initialized by a reset yet
// 1 - full speed
// 2 - low speed
// 3 - high speed
// 4 - super speed
// 5-15 reserved
//

// Bits 20:23 in the Slot Context are the speed for the device

// Port Indicator Control

// Port Link State Write Strobe - set this when changing link state

// true: connect status change

// true: port enable change

// true: warm reset for a USB 3.0 device is done.  A "hot" reset puts the port
// into an enabled state, and the device into the default state.  A "warm" reset
// also resets the link, forcing the device through the link training sequence.
// SW can also look at the Port Reset register to see when warm reset is done.
//

// true: over-current change

// true: reset change - 1 to 0 transition of PORT_RESET

// port link status change - set on some port link state transitions:
// Transition				Reason
// ------------------------------------------------------------------------------
// - U3 to Resume			Wakeup signaling from a device
// - Resume to Recovery to U0		USB 3.0 device resume
// - Resume to U0			USB 2.0 device resume
// - U3 to Recovery to U0		Software resume of USB 3.0 device complete
// - U3 to U0				Software resume of USB 2.0 device complete
// - U2 to U0				L1 resume of USB 2.1 device complete
// - U0 to U0 (???)			L1 entry rejection by USB 2.1 device
// - U0 to disabled			L1 entry error with USB 2.1 device
// - Any state to inactive		Error on USB 3.0 port
//

// port configure error change - port failed to configure its link partner

// Cold Attach Status - xHC can set this bit to report device attached during
// Sx state. Warm port reset should be perfomed to clear this bit and move port
// to connected state.
//

// wake on connect (enable)

// wake on disconnect (enable)

// wake on over-current (enable)

// bits 28:29 reserved
// true: device is non-removable - for USB 3.0 roothub emulation

// Initiate a warm port reset - complete when PORT_WRC is '1'

// We mark duplicate entries with -1

// Port Power Management Status and Control - port_power_base bitmasks
// Inactivity timer value for transitions into U1, in microseconds.
// Timeout can be up to 127us.  0xFF means an infinite timeout.
//

pub const PORT_U1_TIMEOUT_MASK: c_uint = 0xff;
// Inactivity timer value for transitions into U2

// Bits 24:31 for port testing
// USB2 Protocol PORTSPMSC
pub const PORT_L1S_MASK: c_int = 7;
pub const PORT_L1S_SUCCESS: c_int = 1;

pub const PORT_TEST_MODE_SHIFT: c_int = 28;
// USB3 Protocol PORTLI  Port Link Information

// eUSB2v2 protocol PORTLI Port Link information, RsvdP for normal USB2

// USB2 Protocol PORTHLPMC

// use 512 microseconds as USB2 LPM L1 default timeout.
pub const XHCI_L1_TIMEOUT: c_int = 512;
// Set default HIRD/BESL value to 4 (350/400us) for USB2 L1 LPM resume latency.
// Safe to use with mixed HIRD and BESL systems (host and device) and is used
// by other operating systems.
//
// XHCI 1.0 errata 8/14/12 Table 13 notes:
// "Software should choose xHC BESL/BESLD field values that do not violate a
// device's resume latency requirements,
// e.g. not program values > '4' if BLC = '1' and a HIRD device is attached,
// or not program values < '4' if BLC = '0' and a BESL device is attached.
//
pub const XHCI_DEFAULT_BESL: c_int = 4;
//
// USB3 specification define a 360ms tPollingLFPSTiemout for USB3 ports
// to complete link training. usually link trainig completes much faster
// so check status 10 times with 36ms sleep in places we need to wait for
// polling to complete.
//
pub const XHCI_PORT_POLLING_LFPS_TIME: c_int = 36;
