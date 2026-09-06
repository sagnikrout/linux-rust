//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/host/xhci-caps.h
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
//
// xHCI Host Controller Capability Registers.
// xHCI Specification Section 5.3, Revision 1.2.
//

// hc_capbase - bitmasks
// bits 7:0 - Capability Registers Length

// bits 15:8 - Rsvd
// bits 31:16 - Host Controller Interface Version Number

// HCSPARAMS1 - hcs_params1 - bitmasks
// bits 7:0 - Number of Device Slots

// bits 18:8 - Number of Interrupters, max values is 1024

// bits 23:19 - Rsvd
// bits 31:24 - Max Ports, max values is 255

// HCSPARAMS2 - hcs_params2 - bitmasks
//
// bits 3:0 - Isochronous Scheduling Threshold, frames or uframes that SW
// needs to queue transactions ahead of the HW to meet periodic deadlines.
// - Bits 2:0: Threshold value
// - Bit 3: Unit indicator
// - '1': Threshold in Frames
// - '0': Threshold in Microframes (uframes)
// Note: 1 Frame = 8 Microframes
// xHCI specification section 5.3.4.
//

// bits 7:4 - Event Ring Segment Table Max, 2^(n)

// bits 20:8 - Rsvd
// bits 25:21 - Max Scratchpad Buffers (Hi), 5 Most significant bits

// bit 26 - Scratchpad restore, for save/restore HW state
// bits 31:27 - Max Scratchpad Buffers (Lo), 5 Least significant bits

// HCSPARAMS3 - hcs_params3 - bitmasks
// bits 7:0 - U1 Device Exit Latency, Max U1 to U0 latency for the roothub ports

// bits 15:8 - Rsvd
// bits 31:16 - U2 Device Exit Latency, Max U2 to U0 latency for the roothub ports

// HCCPARAMS1 - hcc_params - bitmasks
// bit 0 - 64-bit Addressing Capability

// bit 1 - BW Negotiation Capability

// bit 2 - Context Size

// bit 3 - Port Power Control

// bit 4 - Port Indicators

// bit 5 - Light HC Reset Capability

// bit 6 - Latency Tolerance Messaging Capability

// bit 7 - No Secondary Stream ID Support

// bit 8 - Parse All Event Data
// bit 9 - Short Packet Capability

// bit 10 - Stopped EDTLA Capability
// bit 11 - Contiguous Frame ID Capability

// bits 15:12 - Max size for Primary Stream Arrays, 2^(n+1)

// bits 31:16 - xHCI Extended Capabilities Pointer, from PCI base: 2^(n)

// DBOFF - db_off - bitmasks
// bits 1:0 - Rsvd
// bits 31:2 - Doorbell Array Offset

// RTSOFF - run_regs_off - bitmasks
// bits 4:0 - Rsvd
// bits 31:5 - Runtime Register Space Offse

// HCCPARAMS2 - hcc_params2 - bitmasks
// bit 0 - U3 Entry Capability

// bit 1 - Configure Endpoint Command Max Exit Latency Too Large Capability

// bit 2 - Force Save Context Capabilitu

// bit 3 - Compliance Transition Capability, false: compliance is enabled by default

// bit 4 - Large ESIT Payload Capability, true: HC support ESIT payload > 48k

// bit 5 - Configuration Information Capability

// bit 6 - Extended TBC Capability, true: Isoc burst count > 65535

// bit 7 - Extended TBC TRB Status Capability

// bit 8 - Get/Set Extended Property Capability

// bit 9 - Virtualization Based Trusted I/O Capability

// bit 10 - Rsvd
// bit 11 - HC support Double BW on a eUSB2 HS ISOC EP

// bit 12 - HC support eUSB2V2 capability

// bits 31:13 - Rsvd
