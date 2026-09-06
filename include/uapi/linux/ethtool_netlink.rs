//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/ethtool_netlink.h
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


// SPDX-License-Identifier: GPL-2.0-only WITH Linux-syscall-note
//
// include/uapi/linux/ethtool_netlink.h - netlink interface for ethtool
//
// See Documentation/networking/ethtool-netlink.rst in kernel source tree for
// doucumentation of the interface.
//

// CABLE TEST NOTIFY
// detected reflection caused by the impedance discontinuity between
// a regular 100 Ohm cable and a part with the abnormal impedance value
//
// TDR not possible due to high noise level
// TDR resolution not possible / out of distance
// Information source for specific results.
// Results provided by the Time Domain Reflectometry (TDR)
// Results provided by the Active Link Cable Diagnostic (ALCD)
// CABLE TEST TDR NOTIFY
// add new constants above here
// 30.3.2.1.5 aSymbolErrorDuringCarrier
// add new constants above here
// 30.3.1.1.2 aFramesTransmittedOK
// 30.3.1.1.3 aSingleCollisionFrames
// 30.3.1.1.4 aMultipleCollisionFrames
// 30.3.1.1.5 aFramesReceivedOK
// 30.3.1.1.6 aFrameCheckSequenceErrors
// 30.3.1.1.7 aAlignmentErrors
// 30.3.1.1.8 aOctetsTransmittedOK
// 30.3.1.1.9 aFramesWithDeferredXmissions
// 30.3.1.1.10 aLateCollisions
// 30.3.1.1.11 aFramesAbortedDueToXSColls
// 30.3.1.1.12 aFramesLostDueToIntMACXmitError
// 30.3.1.1.13 aCarrierSenseErrors
// 30.3.1.1.14 aOctetsReceivedOK
// 30.3.1.1.15 aFramesLostDueToIntMACRcvError
// 30.3.1.1.18 aMulticastFramesXmittedOK
// 30.3.1.1.19 aBroadcastFramesXmittedOK
// 30.3.1.1.20 aFramesWithExcessiveDeferral
// 30.3.1.1.21 aMulticastFramesReceivedOK
// 30.3.1.1.22 aBroadcastFramesReceivedOK
// 30.3.1.1.23 aInRangeLengthErrors
// 30.3.1.1.24 aOutOfRangeLengthField
// 30.3.1.1.25 aFrameTooLongErrors
// add new constants above here
// 30.3.3.3 aMACControlFramesTransmitted
// 30.3.3.4 aMACControlFramesReceived
// 30.3.3.5 aUnsupportedOpcodesReceived
// add new constants above here
// etherStatsUndersizePkts
// etherStatsOversizePkts
// etherStatsFragments
// etherStatsJabbers
// add new constants above here
// Basic packet counters if PHY has separate counters from the MAC
// add new constants above here
