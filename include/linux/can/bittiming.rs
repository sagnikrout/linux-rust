//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/can/bittiming.h
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


// SPDX-License-Identifier: GPL-2.0-only
// Copyright (c) 2020 Pengutronix, Marc Kleine-Budde <kernel@pengutronix.de>
// Copyright (c) 2021-2025 Vincent Mailhol <mailhol@kernel.org>
//

pub const CAN_SYNC_SEG: c_int = 1;
pub const CAN_BITRATE_UNSET: c_int = 0;

//
// struct can_tdc - CAN FD Transmission Delay Compensation parameters
//
// At high bit rates, the propagation delay from the TX pin to the RX
// pin of the transceiver causes measurement errors: the sample point
// on the RX pin might occur on the previous bit.
//
// To solve this issue, ISO 11898-1 introduces in section 11.3.3
// "Transmitter delay compensation" a SSP (Secondary Sample Point)
// equal to the distance from the start of the bit time on the TX pin
// to the actual measurement on the RX pin.
//
// This structure contains the parameters to calculate that SSP.
//
// -+----------- one bit ----------+-- TX pin
// |<--- Sample Point --->|
//
// --+----------- one bit ----------+-- RX pin
// |<-------- TDCV -------->|
// |<------- TDCO ------->|
// |<----------- Secondary Sample Point ---------->|
//
// To increase precision, contrary to the other bittiming parameters
// which are measured in time quanta, the TDC parameters are measured
// in clock periods (also referred as "minimum time quantum" in ISO
// 11898-1).
//
// @tdcv: Transmitter Delay Compensation Value. The time needed for
// the signal to propagate, i.e. the distance, in clock periods,
// from the start of the bit on the TX pin to when it is received
// on the RX pin. @tdcv depends on the controller modes:
//
// CAN_CTRLMODE_TDC_AUTO is set: The transceiver dynamically
// measures @tdcv for each transmitted CAN FD frame and the
// value provided here should be ignored.
//
// CAN_CTRLMODE_TDC_MANUAL is set: use the fixed provided @tdcv
// value.
//
// N.B. CAN_CTRLMODE_TDC_AUTO and CAN_CTRLMODE_TDC_MANUAL are
// mutually exclusive. Only one can be set at a time. If both
// CAN_TDC_CTRLMODE_AUTO and CAN_TDC_CTRLMODE_MANUAL are unset,
// TDC is disabled and all the values of this structure should be
// ignored.
//
// @tdco: Transmitter Delay Compensation Offset. Offset value, in
// clock periods, defining the distance between the start of the
// bit reception on the RX pin of the transceiver and the SSP
// position such that SSP = @tdcv + @tdco.
//
// @tdcf: Transmitter Delay Compensation Filter window. Defines the
// minimum value for the SSP position in clock periods. If the
// SSP position is less than @tdcf, then no delay compensations
// occur and the normal sampling point is used instead. The
// feature is enabled if and only if @tdcv is set to zero
// (automatic mode) and @tdcf is configured to a value greater
// than @tdco.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct can_tdc {
    pub tdcv: u32,
    pub tdco: u32,
    pub tdcf: u32,
}

// The transceiver decoding margin corresponds to t_Decode in ISO 11898-2
pub const CAN_PWM_DECODE_NS: c_int = 5;
// Maximum PWM symbol duration. Corresponds to t_SymbolNom_MAX - t_Decode

//
// struct can_tdc_const - CAN hardware-dependent constant for
// Transmission Delay Compensation
//
// @tdcv_min: Transmitter Delay Compensation Value minimum value. If
// the controller does not support manual mode for tdcv
// (c.f. flag CAN_CTRLMODE_TDC_MANUAL) then this value is
// ignored.
// @tdcv_max: Transmitter Delay Compensation Value maximum value. If
// the controller does not support manual mode for tdcv
// (c.f. flag CAN_CTRLMODE_TDC_MANUAL) then this value is
// ignored.
//
// @tdco_min: Transmitter Delay Compensation Offset minimum value.
// @tdco_max: Transmitter Delay Compensation Offset maximum value.
// Should not be zero. If the controller does not support TDC,
// then the pointer to this structure should be NULL.
//
// @tdcf_min: Transmitter Delay Compensation Filter window minimum
// value. If @tdcf_max is zero, this value is ignored.
// @tdcf_max: Transmitter Delay Compensation Filter window maximum
// value. Should be set to zero if the controller does not
// support this feature.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct can_tdc_const {
    pub tdcv_min: u32,
    pub tdcv_max: u32,
    pub tdco_min: u32,
    pub tdco_max: u32,
    pub tdcf_min: u32,
    pub tdcf_max: u32,
}

//
// struct can_pwm - CAN Pulse-Width Modulation (PWM) parameters
//
// @pwms: pulse width modulation short phase
// @pwml: pulse width modulation long phase
// @pwmo: pulse width modulation offset
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct can_pwm {
    pub pwms: u32,
    pub pwml: u32,
    pub pwmo: u32,
}

//
// struct can_pwm - CAN hardware-dependent constants for Pulse-Width
// Modulation (PWM)
//
// @pwms_min: PWM short phase minimum value. Must be at least 1.
// @pwms_max: PWM short phase maximum value
// @pwml_min: PWM long phase minimum value. Must be at least 1.
// @pwml_max: PWM long phase maximum value
// @pwmo_min: PWM offset phase minimum value
// @pwmo_max: PWM offset phase maximum value
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct can_pwm_const {
    pub pwms_min: u32,
    pub pwms_max: u32,
    pub pwml_min: u32,
    pub pwml_max: u32,
    pub pwmo_min: u32,
    pub pwmo_max: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct data_bittiming_params {
    pub data_bittiming_const: *const can_bittiming_,
    pub data_bittiming: can_bittiming,
    pub tdc_const: *const can_tdc_,
    pub pwm_const: *const can_pwm_,
    pub tdc: can_tdc,
    pub pwm: can_pwm,
}

extern "C" {
    pub fn can_calc_pwm(dev: *mut net_device, extack: *mut netlink_ext_ack) -> c_int;
}

extern "C" {
    pub fn can_sjw_set_default(bt: *mut can_bittiming);
}
//
// can_get_relative_tdco() - TDCO relative to the sample point
//
// struct can_tdc::tdco represents the absolute offset from TDCV. Some
// controllers use instead an offset relative to the Sample Point (SP)
// such that:
//
// SSP = TDCV + absolute TDCO
// = TDCV + SP + relative TDCO
//
// -+----------- one bit ----------+-- TX pin
// |<--- Sample Point --->|
//
// --+----------- one bit ----------+-- RX pin
// |<-------- TDCV -------->|
// |<------------------------>| absolute TDCO
// |<--- Sample Point --->|
// |                      |<->| relative TDCO
// |<------------- Secondary Sample Point ------------>|
//
// can_bit_time() - Duration of one bit
//
// Please refer to ISO 11898-1:2015, section 11.3.1.1 "Bit time" for
// additional information.
//
// Return: the number of time quanta in one bit.
//
// Duration of one bit in minimum time quantum
// Convert a duration from minimum a minimum time quantum to nano seconds
