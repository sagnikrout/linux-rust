//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/can/netlink.h
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
// linux/can/netlink.h
//
// Definitions for the CAN netlink interface
//
// Copyright (c) 2009 Wolfgang Grandegger <wg@grandegger.com>
// Copyright (c) 2021-2025 Vincent Mailhol <mailhol@kernel.org>
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the version 2 of the GNU General Public License
// as published by the Free Software Foundation
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//

//
// CAN bit-timing parameters
//
// For further information, please read chapter "8 BIT TIMING
// REQUIREMENTS" of the "Bosch CAN Specification version 2.0"
// at http://www.semiconductors.bosch.de/pdf/can2spec.pdf.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct can_bittiming {
    pub /: *mut *mut __u32 bitrate; / Bit-rate in bits/second,
    pub /: *mut *mut __u32 sample_point; / Sample point in one-tenth of a percent,
    pub /: *mut *mut __u32 tq; / Time quanta (TQ) in nanoseconds,
    pub /: *mut *mut __u32 prop_seg; / Propagation segment in TQs,
    pub /: *mut *mut __u32 phase_seg1; / Phase buffer segment 1 in TQs,
    pub /: *mut *mut __u32 phase_seg2; / Phase buffer segment 2 in TQs,
    pub /: *mut *mut __u32 sjw; / Synchronisation jump width in TQs,
    pub /: *mut *mut __u32 brp; / Bit-rate prescaler,
}

//
// CAN hardware-dependent bit-timing constant
//
// Used for calculating and checking bit-timing parameters
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct can_bittiming_const {
    pub /: *mut *mut char name[16]; / Name of the CAN controller hardware,
    pub /: *mut *mut __u32 tseg1_min; / Time segment 1 = prop_seg + phase_seg1,
    pub tseg1_max: __u32,
    pub /: *mut *mut __u32 tseg2_min; / Time segment 2 = phase_seg2,
    pub tseg2_max: __u32,
    pub /: *mut *mut __u32 sjw_max; / Synchronisation jump width,
    pub /: *mut *mut __u32 brp_min; / Bit-rate prescaler,
    pub brp_max: __u32,
    pub brp_inc: __u32,
}

//
// CAN clock parameters
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct can_clock {
    pub /: *mut *mut __u32 freq; / CAN system clock frequency in Hz,
}

//
// CAN operational and error states
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum can_state {
    CAN_STATE_ERROR_ACTIVE = 0,	/* RX/TX error count < 96 */
    CAN_STATE_ERROR_WARNING,	/* RX/TX error count < 128 */
    CAN_STATE_ERROR_PASSIVE,	/* RX/TX error count < 256 */
    CAN_STATE_BUS_OFF,		/* RX/TX error count >= 256 */
    CAN_STATE_STOPPED,		/* Device is stopped */
    CAN_STATE_SLEEPING,		/* Device is sleeping */
    CAN_STATE_MAX
}

//
// CAN bus error counters
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct can_berr_counter {
    pub txerr: __u16,
    pub rxerr: __u16,
}

//
// CAN controller mode
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct can_ctrlmode {
    pub mask: __u32,
    pub flags: __u32,
}

pub const CAN_CTRLMODE_LOOPBACK: c_uint = 0x01	/* Loopback mode */;
pub const CAN_CTRLMODE_LISTENONLY: c_uint = 0x02	/* Listen-only mode */;
pub const CAN_CTRLMODE_3_SAMPLES: c_uint = 0x04	/* Triple sampling mode */;
pub const CAN_CTRLMODE_ONE_SHOT: c_uint = 0x08	/* One-Shot mode */;
pub const CAN_CTRLMODE_BERR_REPORTING: c_uint = 0x10	/* Bus-error reporting */;
pub const CAN_CTRLMODE_FD: c_uint = 0x20	/* CAN FD mode */;
pub const CAN_CTRLMODE_PRESUME_ACK: c_uint = 0x40	/* Ignore missing CAN ACKs */;
pub const CAN_CTRLMODE_FD_NON_ISO: c_uint = 0x80	/* CAN FD in non-ISO mode */;
pub const CAN_CTRLMODE_CC_LEN8_DLC: c_uint = 0x100	/* Classic CAN DLC option */;
pub const CAN_CTRLMODE_TDC_AUTO: c_uint = 0x200	/* FD transceiver automatically calculates TDCV */;
pub const CAN_CTRLMODE_TDC_MANUAL: c_uint = 0x400	/* FD TDCV is manually set up by user */;
pub const CAN_CTRLMODE_RESTRICTED: c_uint = 0x800	/* Restricted operation mode */;
pub const CAN_CTRLMODE_XL: c_uint = 0x1000	/* CAN XL mode */;
pub const CAN_CTRLMODE_XL_TDC_AUTO: c_uint = 0x2000	/* XL transceiver automatically calculates TDCV */;
pub const CAN_CTRLMODE_XL_TDC_MANUAL: c_uint = 0x4000	/* XL TDCV is manually set up by user */;
pub const CAN_CTRLMODE_XL_TMS: c_uint = 0x8000	/* Transceiver Mode Switching */;
//
// CAN device statistics
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct can_device_stats {
    pub /: *mut *mut __u32 bus_error; / Bus errors,
    pub /: *mut *mut __u32 error_warning; / Changes to error warning state,
    pub /: *mut *mut __u32 error_passive; / Changes to error passive state,
    pub /: *mut *mut __u32 bus_off; / Changes to bus off state,
    pub /: *mut *mut __u32 arbitration_lost; / Arbitration lost errors,
    pub /: *mut *mut __u32 restarts; / CAN controller re-starts,
}

//
// CAN netlink interface
//
// add new constants above here
//
// CAN FD/XL Transmitter Delay Compensation (TDC)
//
// Please refer to struct can_tdc_const and can_tdc in
// include/linux/can/bittiming.h for further details.
//
// add new constants above here
//
// IFLA_CAN_CTRLMODE_EXT nest: controller mode extended parameters
//
// add new constants above here
//
// CAN FD/XL Pulse-Width Modulation (PWM)
//
// Please refer to struct can_pwm_const and can_pwm in
// include/linux/can/bittiming.h for further details.
//
// add new constants above here
// u16 termination range: 1..65535 Ohms
pub const CAN_TERMINATION_DISABLED: c_int = 0;
