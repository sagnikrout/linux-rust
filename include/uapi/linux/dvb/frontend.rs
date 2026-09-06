//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/dvb/frontend.h
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


// SPDX-License-Identifier: LGPL-2.1+ WITH Linux-syscall-note
//
// frontend.h
//
// Copyright (C) 2000 Marcus Metzler <marcus@convergence.de>
// Ralph  Metzler <ralph@convergence.de>
// Holger Waechtler <holger@convergence.de>
// Andre Draszik <ad@convergence.de>
// for convergence integrated media GmbH
//

//
// enum fe_caps - Frontend capabilities
//
// @FE_IS_STUPID:			There's something wrong at the
// frontend, and it can't report its
// capabilities.
// @FE_CAN_INVERSION_AUTO:		Can auto-detect frequency spectral
// band inversion
// @FE_CAN_FEC_1_2:			Supports FEC 1/2
// @FE_CAN_FEC_2_3:			Supports FEC 2/3
// @FE_CAN_FEC_3_4:			Supports FEC 3/4
// @FE_CAN_FEC_4_5:			Supports FEC 4/5
// @FE_CAN_FEC_5_6:			Supports FEC 5/6
// @FE_CAN_FEC_6_7:			Supports FEC 6/7
// @FE_CAN_FEC_7_8:			Supports FEC 7/8
// @FE_CAN_FEC_8_9:			Supports FEC 8/9
// @FE_CAN_FEC_AUTO:			Can auto-detect FEC
// @FE_CAN_QPSK:			Supports QPSK modulation
// @FE_CAN_QAM_16:			Supports 16-QAM modulation
// @FE_CAN_QAM_32:			Supports 32-QAM modulation
// @FE_CAN_QAM_64:			Supports 64-QAM modulation
// @FE_CAN_QAM_128:			Supports 128-QAM modulation
// @FE_CAN_QAM_256:			Supports 256-QAM modulation
// @FE_CAN_QAM_AUTO:			Can auto-detect QAM modulation
// @FE_CAN_TRANSMISSION_MODE_AUTO:	Can auto-detect transmission mode
// @FE_CAN_BANDWIDTH_AUTO:		Can auto-detect bandwidth
// @FE_CAN_GUARD_INTERVAL_AUTO:		Can auto-detect guard interval
// @FE_CAN_HIERARCHY_AUTO:		Can auto-detect hierarchy
// @FE_CAN_8VSB:			Supports 8-VSB modulation
// @FE_CAN_16VSB:			Supporta 16-VSB modulation
// @FE_HAS_EXTENDED_CAPS:		Unused
// @FE_CAN_MULTISTREAM:			Supports multistream filtering
// @FE_CAN_TURBO_FEC:			Supports "turbo FEC" modulation
// @FE_CAN_2G_MODULATION:		Supports "2nd generation" modulation,
// e. g. DVB-S2, DVB-T2, DVB-C2
// @FE_NEEDS_BENDING:			Unused
// @FE_CAN_RECOVER:			Can recover from a cable unplug
// automatically
// @FE_CAN_MUTE_TS:			Can stop spurious TS data output
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fe_caps {
    FE_IS_STUPID			= 0,
    FE_CAN_INVERSION_AUTO		= 0x1,
    FE_CAN_FEC_1_2			= 0x2,
    FE_CAN_FEC_2_3			= 0x4,
    FE_CAN_FEC_3_4			= 0x8,
    FE_CAN_FEC_4_5			= 0x10,
    FE_CAN_FEC_5_6			= 0x20,
    FE_CAN_FEC_6_7			= 0x40,
    FE_CAN_FEC_7_8			= 0x80,
    FE_CAN_FEC_8_9			= 0x100,
    FE_CAN_FEC_AUTO			= 0x200,
    FE_CAN_QPSK			= 0x400,
    FE_CAN_QAM_16			= 0x800,
    FE_CAN_QAM_32			= 0x1000,
    FE_CAN_QAM_64			= 0x2000,
    FE_CAN_QAM_128			= 0x4000,
    FE_CAN_QAM_256			= 0x8000,
    FE_CAN_QAM_AUTO			= 0x10000,
    FE_CAN_TRANSMISSION_MODE_AUTO	= 0x20000,
    FE_CAN_BANDWIDTH_AUTO		= 0x40000,
    FE_CAN_GUARD_INTERVAL_AUTO	= 0x80000,
    FE_CAN_HIERARCHY_AUTO		= 0x100000,
    FE_CAN_8VSB			= 0x200000,
    FE_CAN_16VSB			= 0x400000,
    FE_HAS_EXTENDED_CAPS		= 0x800000,
    FE_CAN_MULTISTREAM		= 0x4000000,
    FE_CAN_TURBO_FEC		= 0x8000000,
    FE_CAN_2G_MODULATION		= 0x10000000,
    FE_NEEDS_BENDING		= 0x20000000,
    FE_CAN_RECOVER			= 0x40000000,
    FE_CAN_MUTE_TS			= 0x80000000
}

//
// DEPRECATED: Should be kept just due to backward compatibility.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fe_type {
    FE_QPSK,
    FE_QAM,
    FE_OFDM,
    FE_ATSC
}

//
// struct dvb_frontend_info - Frontend properties and capabilities
//
// @name:			Name of the frontend
// @type:			**DEPRECATED**.
// Should not be used on modern programs,
// as a frontend may have more than one type.
// In order to get the support types of a given
// frontend, use :c:type:`DTV_ENUM_DELSYS`
// instead.
// @frequency_min:		Minimal frequency supported by the frontend.
// @frequency_max:		Minimal frequency supported by the frontend.
// @frequency_stepsize:		All frequencies are multiple of this value.
// @frequency_tolerance:	Frequency tolerance.
// @symbol_rate_min:		Minimal symbol rate, in bauds
// (for Cable/Satellite systems).
// @symbol_rate_max:		Maximal symbol rate, in bauds
// (for Cable/Satellite systems).
// @symbol_rate_tolerance:	Maximal symbol rate tolerance, in ppm
// (for Cable/Satellite systems).
// @notifier_delay:		**DEPRECATED**. Not used by any driver.
// @caps:			Capabilities supported by the frontend,
// as specified in &enum fe_caps.
//
// .. note:
//
// #. The frequencies are specified in Hz for Terrestrial and Cable
// systems.
// #. The frequencies are specified in kHz for Satellite systems.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dvb_frontend_info {
    pub name: [c_char; 128],
    pub /: *mut *mut fe_type type; / DEPRECATED. Use DTV_ENUM_DELSYS instead,
    pub frequency_min: __u32,
    pub frequency_max: __u32,
    pub frequency_stepsize: __u32,
    pub frequency_tolerance: __u32,
    pub symbol_rate_min: __u32,
    pub symbol_rate_max: __u32,
    pub symbol_rate_tolerance: __u32,
    pub /: *mut *mut __u32 notifier_delay; / DEPRECATED,
    pub caps: fe_caps,
}

//
// struct dvb_diseqc_master_cmd - DiSEqC master command
//
// @msg:
// DiSEqC message to be sent. It contains a 3 bytes header with:
// framing + address + command, and an optional argument
// of up to 3 bytes of data.
// @msg_len:
// Length of the DiSEqC message. Valid values are 3 to 6.
//
// Check out the DiSEqC bus spec available on http://www.eutelsat.org/ for
// the possible messages that can be used.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dvb_diseqc_master_cmd {
    pub msg: [__u8; 6],
    pub msg_len: __u8,
}

//
// struct dvb_diseqc_slave_reply - DiSEqC received data
//
// @msg:
// DiSEqC message buffer to store a message received via DiSEqC.
// It contains one byte header with: framing and
// an optional argument of up to 3 bytes of data.
// @msg_len:
// Length of the DiSEqC message. Valid values are 0 to 4,
// where 0 means no message.
// @timeout:
// Return from ioctl after timeout ms with errorcode when
// no message was received.
//
// Check out the DiSEqC bus spec available on http://www.eutelsat.org/ for
// the possible messages that can be used.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dvb_diseqc_slave_reply {
    pub msg: [__u8; 4],
    pub msg_len: __u8,
    pub timeout: c_int,
}

//
// enum fe_sec_voltage - DC Voltage used to feed the LNBf
//
// @SEC_VOLTAGE_13:	Output 13V to the LNBf
// @SEC_VOLTAGE_18:	Output 18V to the LNBf
// @SEC_VOLTAGE_OFF:	Don't feed the LNBf with a DC voltage
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fe_sec_voltage {
    SEC_VOLTAGE_13,
    SEC_VOLTAGE_18,
    SEC_VOLTAGE_OFF
}

//
// enum fe_sec_tone_mode - Type of tone to be send to the LNBf.
// @SEC_TONE_ON:	Sends a 22kHz tone burst to the antenna.
// @SEC_TONE_OFF:	Don't send a 22kHz tone to the antenna (except
// if the ``FE_DISEQC_*`` ioctls are called).
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fe_sec_tone_mode {
    SEC_TONE_ON,
    SEC_TONE_OFF
}

//
// enum fe_sec_mini_cmd - Type of mini burst to be sent
//
// @SEC_MINI_A:		Sends a mini-DiSEqC 22kHz '0' Tone Burst to select
// satellite-A
// @SEC_MINI_B:		Sends a mini-DiSEqC 22kHz '1' Data Burst to select
// satellite-B
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fe_sec_mini_cmd {
    SEC_MINI_A,
    SEC_MINI_B
}

//
// enum fe_status - Enumerates the possible frontend status.
// @FE_NONE:		The frontend doesn't have any kind of lock.
// That's the initial frontend status
// @FE_HAS_SIGNAL:	Has found something above the noise level.
// @FE_HAS_CARRIER:	Has found a signal.
// @FE_HAS_VITERBI:	FEC inner coding (Viterbi, LDPC or other inner code).
// is stable.
// @FE_HAS_SYNC:	Synchronization bytes was found.
// @FE_HAS_LOCK:	Digital TV were locked and everything is working.
// @FE_TIMEDOUT:	Fo lock within the last about 2 seconds.
// @FE_REINIT:		Frontend was reinitialized, application is recommended
// to reset DiSEqC, tone and parameters.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fe_status {
    FE_NONE			= 0x00,
    FE_HAS_SIGNAL		= 0x01,
    FE_HAS_CARRIER		= 0x02,
    FE_HAS_VITERBI		= 0x04,
    FE_HAS_SYNC		= 0x08,
    FE_HAS_LOCK		= 0x10,
    FE_TIMEDOUT		= 0x20,
    FE_REINIT		= 0x40,
}

//
// enum fe_spectral_inversion - Type of inversion band
//
// @INVERSION_OFF:	Don't do spectral band inversion.
// @INVERSION_ON:	Do spectral band inversion.
// @INVERSION_AUTO:	Autodetect spectral band inversion.
//
// This parameter indicates if spectral inversion should be presumed or
// not. In the automatic setting (``INVERSION_AUTO``) the hardware will try
// to figure out the correct setting by itself. If the hardware doesn't
// support, the %dvb_frontend will try to lock at the carrier first with
// inversion off. If it fails, it will try to enable inversion.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fe_spectral_inversion {
    INVERSION_OFF,
    INVERSION_ON,
    INVERSION_AUTO
}

//
// enum fe_code_rate - Type of Forward Error Correction (FEC)
//
// @FEC_NONE: No Forward Error Correction Code
// @FEC_1_2:  Forward Error Correction Code 1/2
// @FEC_2_3:  Forward Error Correction Code 2/3
// @FEC_3_4:  Forward Error Correction Code 3/4
// @FEC_4_5:  Forward Error Correction Code 4/5
// @FEC_5_6:  Forward Error Correction Code 5/6
// @FEC_6_7:  Forward Error Correction Code 6/7
// @FEC_7_8:  Forward Error Correction Code 7/8
// @FEC_8_9:  Forward Error Correction Code 8/9
// @FEC_AUTO: Autodetect Error Correction Code
// @FEC_3_5:  Forward Error Correction Code 3/5
// @FEC_9_10: Forward Error Correction Code 9/10
// @FEC_2_5:  Forward Error Correction Code 2/5
// @FEC_1_3:  Forward Error Correction Code 1/3
// @FEC_1_4:  Forward Error Correction Code 1/4
// @FEC_5_9:  Forward Error Correction Code 5/9
// @FEC_7_9:  Forward Error Correction Code 7/9
// @FEC_8_15:  Forward Error Correction Code 8/15
// @FEC_11_15: Forward Error Correction Code 11/15
// @FEC_13_18: Forward Error Correction Code 13/18
// @FEC_9_20:  Forward Error Correction Code 9/20
// @FEC_11_20: Forward Error Correction Code 11/20
// @FEC_23_36: Forward Error Correction Code 23/36
// @FEC_25_36: Forward Error Correction Code 25/36
// @FEC_13_45: Forward Error Correction Code 13/45
// @FEC_26_45: Forward Error Correction Code 26/45
// @FEC_28_45: Forward Error Correction Code 28/45
// @FEC_32_45: Forward Error Correction Code 32/45
// @FEC_77_90: Forward Error Correction Code 77/90
// @FEC_11_45: Forward Error Correction Code 11/45
// @FEC_4_15: Forward Error Correction Code 4/15
// @FEC_14_45: Forward Error Correction Code 14/45
// @FEC_7_15: Forward Error Correction Code 7/15
//
// Please note that not all FEC types are supported by a given standard.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fe_code_rate {
    FEC_NONE = 0,
    FEC_1_2,
    FEC_2_3,
    FEC_3_4,
    FEC_4_5,
    FEC_5_6,
    FEC_6_7,
    FEC_7_8,
    FEC_8_9,
    FEC_AUTO,
    FEC_3_5,
    FEC_9_10,
    FEC_2_5,
    FEC_1_3,
    FEC_1_4,
    FEC_5_9,
    FEC_7_9,
    FEC_8_15,
    FEC_11_15,
    FEC_13_18,
    FEC_9_20,
    FEC_11_20,
    FEC_23_36,
    FEC_25_36,
    FEC_13_45,
    FEC_26_45,
    FEC_28_45,
    FEC_32_45,
    FEC_77_90,
    FEC_11_45,
    FEC_4_15,
    FEC_14_45,
    FEC_7_15,
}

//
// enum fe_modulation - Type of modulation/constellation
// @QPSK:	QPSK modulation
// @QAM_16:	16-QAM modulation
// @QAM_32:	32-QAM modulation
// @QAM_64:	64-QAM modulation
// @QAM_128:	128-QAM modulation
// @QAM_256:	256-QAM modulation
// @QAM_AUTO:	Autodetect QAM modulation
// @VSB_8:	8-VSB modulation
// @VSB_16:	16-VSB modulation
// @PSK_8:	8-PSK modulation
// @APSK_16:	16-APSK modulation
// @APSK_32:	32-APSK modulation
// @DQPSK:	DQPSK modulation
// @QAM_4_NR:	4-QAM-NR modulation
// @QAM_1024:	1024-QAM modulation
// @QAM_4096:	4096-QAM modulation
// @APSK_8_L:	8APSK-L modulation
// @APSK_16_L:	16APSK-L modulation
// @APSK_32_L:	32APSK-L modulation
// @APSK_64:	64APSK modulation
// @APSK_64_L:	64APSK-L modulation
//
// Please note that not all modulations are supported by a given standard.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fe_modulation {
    QPSK,
    QAM_16,
    QAM_32,
    QAM_64,
    QAM_128,
    QAM_256,
    QAM_AUTO,
    VSB_8,
    VSB_16,
    PSK_8,
    APSK_16,
    APSK_32,
    DQPSK,
    QAM_4_NR,
    QAM_1024,
    QAM_4096,
    APSK_8_L,
    APSK_16_L,
    APSK_32_L,
    APSK_64,
    APSK_64_L,
}

//
// enum fe_transmit_mode - Transmission mode
//
// @TRANSMISSION_MODE_AUTO:
// Autodetect transmission mode. The hardware will try to find the
// correct FFT-size (if capable) to fill in the missing parameters.
// @TRANSMISSION_MODE_1K:
// Transmission mode 1K
// @TRANSMISSION_MODE_2K:
// Transmission mode 2K
// @TRANSMISSION_MODE_8K:
// Transmission mode 8K
// @TRANSMISSION_MODE_4K:
// Transmission mode 4K
// @TRANSMISSION_MODE_16K:
// Transmission mode 16K
// @TRANSMISSION_MODE_32K:
// Transmission mode 32K
// @TRANSMISSION_MODE_C1:
// Single Carrier (C=1) transmission mode (DTMB only)
// @TRANSMISSION_MODE_C3780:
// Multi Carrier (C=3780) transmission mode (DTMB only)
//
// Please note that not all transmission modes are supported by a given
// standard.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fe_transmit_mode {
    TRANSMISSION_MODE_2K,
    TRANSMISSION_MODE_8K,
    TRANSMISSION_MODE_AUTO,
    TRANSMISSION_MODE_4K,
    TRANSMISSION_MODE_1K,
    TRANSMISSION_MODE_16K,
    TRANSMISSION_MODE_32K,
    TRANSMISSION_MODE_C1,
    TRANSMISSION_MODE_C3780,
}

//
// enum fe_guard_interval - Guard interval
//
// @GUARD_INTERVAL_AUTO:	Autodetect the guard interval
// @GUARD_INTERVAL_1_128:	Guard interval 1/128
// @GUARD_INTERVAL_1_32:	Guard interval 1/32
// @GUARD_INTERVAL_1_16:	Guard interval 1/16
// @GUARD_INTERVAL_1_8:		Guard interval 1/8
// @GUARD_INTERVAL_1_4:		Guard interval 1/4
// @GUARD_INTERVAL_19_128:	Guard interval 19/128
// @GUARD_INTERVAL_19_256:	Guard interval 19/256
// @GUARD_INTERVAL_PN420:	PN length 420 (1/4)
// @GUARD_INTERVAL_PN595:	PN length 595 (1/6)
// @GUARD_INTERVAL_PN945:	PN length 945 (1/9)
// @GUARD_INTERVAL_1_64:	Guard interval 1/64
//
// Please note that not all guard intervals are supported by a given standard.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fe_guard_interval {
    GUARD_INTERVAL_1_32,
    GUARD_INTERVAL_1_16,
    GUARD_INTERVAL_1_8,
    GUARD_INTERVAL_1_4,
    GUARD_INTERVAL_AUTO,
    GUARD_INTERVAL_1_128,
    GUARD_INTERVAL_19_128,
    GUARD_INTERVAL_19_256,
    GUARD_INTERVAL_PN420,
    GUARD_INTERVAL_PN595,
    GUARD_INTERVAL_PN945,
    GUARD_INTERVAL_1_64,
}

//
// enum fe_hierarchy - Hierarchy
// @HIERARCHY_NONE:	No hierarchy
// @HIERARCHY_AUTO:	Autodetect hierarchy (if supported)
// @HIERARCHY_1:	Hierarchy 1
// @HIERARCHY_2:	Hierarchy 2
// @HIERARCHY_4:	Hierarchy 4
//
// Please note that not all hierarchy types are supported by a given standard.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fe_hierarchy {
    HIERARCHY_NONE,
    HIERARCHY_1,
    HIERARCHY_2,
    HIERARCHY_4,
    HIERARCHY_AUTO
}

//
// enum fe_interleaving - Interleaving
// @INTERLEAVING_NONE:	No interleaving.
// @INTERLEAVING_AUTO:	Auto-detect interleaving.
// @INTERLEAVING_240:	Interleaving of 240 symbols.
// @INTERLEAVING_720:	Interleaving of 720 symbols.
//
// Please note that, currently, only DTMB uses it.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fe_interleaving {
    INTERLEAVING_NONE,
    INTERLEAVING_AUTO,
    INTERLEAVING_240,
    INTERLEAVING_720,
}

// DVBv5 property Commands
pub const DTV_UNDEFINED: c_int = 0;
pub const DTV_TUNE: c_int = 1;
pub const DTV_CLEAR: c_int = 2;
pub const DTV_FREQUENCY: c_int = 3;
pub const DTV_MODULATION: c_int = 4;
pub const DTV_BANDWIDTH_HZ: c_int = 5;
pub const DTV_INVERSION: c_int = 6;
pub const DTV_DISEQC_MASTER: c_int = 7;
pub const DTV_SYMBOL_RATE: c_int = 8;
pub const DTV_INNER_FEC: c_int = 9;
pub const DTV_VOLTAGE: c_int = 10;
pub const DTV_TONE: c_int = 11;
pub const DTV_PILOT: c_int = 12;
pub const DTV_ROLLOFF: c_int = 13;
pub const DTV_DISEQC_SLAVE_REPLY: c_int = 14;
// Basic enumeration set for querying unlimited capabilities
pub const DTV_FE_CAPABILITY_COUNT: c_int = 15;
pub const DTV_FE_CAPABILITY: c_int = 16;
pub const DTV_DELIVERY_SYSTEM: c_int = 17;
// ISDB-T and ISDB-Tsb
pub const DTV_ISDBT_PARTIAL_RECEPTION: c_int = 18;
pub const DTV_ISDBT_SOUND_BROADCASTING: c_int = 19;
pub const DTV_ISDBT_SB_SUBCHANNEL_ID: c_int = 20;
pub const DTV_ISDBT_SB_SEGMENT_IDX: c_int = 21;
pub const DTV_ISDBT_SB_SEGMENT_COUNT: c_int = 22;
pub const DTV_ISDBT_LAYERA_FEC: c_int = 23;
pub const DTV_ISDBT_LAYERA_MODULATION: c_int = 24;
pub const DTV_ISDBT_LAYERA_SEGMENT_COUNT: c_int = 25;
pub const DTV_ISDBT_LAYERA_TIME_INTERLEAVING: c_int = 26;
pub const DTV_ISDBT_LAYERB_FEC: c_int = 27;
pub const DTV_ISDBT_LAYERB_MODULATION: c_int = 28;
pub const DTV_ISDBT_LAYERB_SEGMENT_COUNT: c_int = 29;
pub const DTV_ISDBT_LAYERB_TIME_INTERLEAVING: c_int = 30;
pub const DTV_ISDBT_LAYERC_FEC: c_int = 31;
pub const DTV_ISDBT_LAYERC_MODULATION: c_int = 32;
pub const DTV_ISDBT_LAYERC_SEGMENT_COUNT: c_int = 33;
pub const DTV_ISDBT_LAYERC_TIME_INTERLEAVING: c_int = 34;
pub const DTV_API_VERSION: c_int = 35;
pub const DTV_CODE_RATE_HP: c_int = 36;
pub const DTV_CODE_RATE_LP: c_int = 37;
pub const DTV_GUARD_INTERVAL: c_int = 38;
pub const DTV_TRANSMISSION_MODE: c_int = 39;
pub const DTV_HIERARCHY: c_int = 40;
pub const DTV_ISDBT_LAYER_ENABLED: c_int = 41;
pub const DTV_STREAM_ID: c_int = 42;

pub const DTV_DVBT2_PLP_ID_LEGACY: c_int = 43;
pub const DTV_ENUM_DELSYS: c_int = 44;
// ATSC-MH
pub const DTV_ATSCMH_FIC_VER: c_int = 45;
pub const DTV_ATSCMH_PARADE_ID: c_int = 46;
pub const DTV_ATSCMH_NOG: c_int = 47;
pub const DTV_ATSCMH_TNOG: c_int = 48;
pub const DTV_ATSCMH_SGN: c_int = 49;
pub const DTV_ATSCMH_PRC: c_int = 50;
pub const DTV_ATSCMH_RS_FRAME_MODE: c_int = 51;
pub const DTV_ATSCMH_RS_FRAME_ENSEMBLE: c_int = 52;
pub const DTV_ATSCMH_RS_CODE_MODE_PRI: c_int = 53;
pub const DTV_ATSCMH_RS_CODE_MODE_SEC: c_int = 54;
pub const DTV_ATSCMH_SCCC_BLOCK_MODE: c_int = 55;
pub const DTV_ATSCMH_SCCC_CODE_MODE_A: c_int = 56;
pub const DTV_ATSCMH_SCCC_CODE_MODE_B: c_int = 57;
pub const DTV_ATSCMH_SCCC_CODE_MODE_C: c_int = 58;
pub const DTV_ATSCMH_SCCC_CODE_MODE_D: c_int = 59;
pub const DTV_INTERLEAVING: c_int = 60;
pub const DTV_LNA: c_int = 61;
// Quality parameters
pub const DTV_STAT_SIGNAL_STRENGTH: c_int = 62;
pub const DTV_STAT_CNR: c_int = 63;
pub const DTV_STAT_PRE_ERROR_BIT_COUNT: c_int = 64;
pub const DTV_STAT_PRE_TOTAL_BIT_COUNT: c_int = 65;
pub const DTV_STAT_POST_ERROR_BIT_COUNT: c_int = 66;
pub const DTV_STAT_POST_TOTAL_BIT_COUNT: c_int = 67;
pub const DTV_STAT_ERROR_BLOCK_COUNT: c_int = 68;
pub const DTV_STAT_TOTAL_BLOCK_COUNT: c_int = 69;
// Physical layer scrambling
pub const DTV_SCRAMBLING_SEQUENCE_INDEX: c_int = 70;

//
// enum fe_pilot - Type of pilot tone
//
// @PILOT_ON:	Pilot tones enabled
// @PILOT_OFF:	Pilot tones disabled
// @PILOT_AUTO:	Autodetect pilot tones
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fe_pilot {
    PILOT_ON,
    PILOT_OFF,
    PILOT_AUTO,
}

//
// enum fe_rolloff - Rolloff factor
// @ROLLOFF_35:		Roloff factor: α=35%
// @ROLLOFF_20:		Roloff factor: α=20%
// @ROLLOFF_25:		Roloff factor: α=25%
// @ROLLOFF_AUTO:	Auto-detect the roloff factor.
// @ROLLOFF_15:		Rolloff factor: α=15%
// @ROLLOFF_10:		Rolloff factor: α=10%
// @ROLLOFF_5:		Rolloff factor: α=5%
//
// .. note:
//
// Roloff factor of 35% is implied on DVB-S. On DVB-S2, it is default.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fe_rolloff {
    ROLLOFF_35,
    ROLLOFF_20,
    ROLLOFF_25,
    ROLLOFF_AUTO,
    ROLLOFF_15,
    ROLLOFF_10,
    ROLLOFF_5,
}

//
// enum fe_delivery_system - Type of the delivery system
//
// @SYS_UNDEFINED:
// Undefined standard. Generally, indicates an error
// @SYS_DVBC_ANNEX_A:
// Cable TV: DVB-C following ITU-T J.83 Annex A spec
// @SYS_DVBC_ANNEX_B:
// Cable TV: DVB-C following ITU-T J.83 Annex B spec (ClearQAM)
// @SYS_DVBC_ANNEX_C:
// Cable TV: DVB-C following ITU-T J.83 Annex C spec
// @SYS_DVBC2:
// Cable TV: DVB-C2
// @SYS_ISDBC:
// Cable TV: ISDB-C (no drivers yet)
// @SYS_DVBT:
// Terrestrial TV: DVB-T
// @SYS_DVBT2:
// Terrestrial TV: DVB-T2
// @SYS_ISDBT:
// Terrestrial TV: ISDB-T
// @SYS_ATSC:
// Terrestrial TV: ATSC
// @SYS_ATSCMH:
// Terrestrial TV (mobile): ATSC-M/H
// @SYS_DTMB:
// Terrestrial TV: DTMB
// @SYS_DVBS:
// Satellite TV: DVB-S
// @SYS_DVBS2:
// Satellite TV: DVB-S2 and DVB-S2X
// @SYS_TURBO:
// Satellite TV: DVB-S Turbo
// @SYS_ISDBS:
// Satellite TV: ISDB-S
// @SYS_DAB:
// Digital audio: DAB (not fully supported)
// @SYS_DSS:
// Satellite TV: DSS (not fully supported)
// @SYS_CMMB:
// Terrestrial TV (mobile): CMMB (not fully supported)
// @SYS_DVBH:
// Terrestrial TV (mobile): DVB-H (standard deprecated)
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fe_delivery_system {
    SYS_UNDEFINED,
    SYS_DVBC_ANNEX_A,
    SYS_DVBC_ANNEX_B,
    SYS_DVBT,
    SYS_DSS,
    SYS_DVBS,
    SYS_DVBS2,
    SYS_DVBH,
    SYS_ISDBT,
    SYS_ISDBS,
    SYS_ISDBC,
    SYS_ATSC,
    SYS_ATSCMH,
    SYS_DTMB,
    SYS_CMMB,
    SYS_DAB,
    SYS_DVBT2,
    SYS_TURBO,
    SYS_DVBC_ANNEX_C,
    SYS_DVBC2,
}

// backward compatibility definitions for delivery systems

// ATSC-MH specific parameters
//
// enum atscmh_sccc_block_mode - Type of Series Concatenated Convolutional
// Code Block Mode.
//
// @ATSCMH_SCCC_BLK_SEP:
// Separate SCCC: the SCCC outer code mode shall be set independently
// for each Group Region (A, B, C, D)
// @ATSCMH_SCCC_BLK_COMB:
// Combined SCCC: all four Regions shall have the same SCCC outer
// code mode.
// @ATSCMH_SCCC_BLK_RES:
// Reserved. Shouldn't be used.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum atscmh_sccc_block_mode {
    ATSCMH_SCCC_BLK_SEP      = 0,
    ATSCMH_SCCC_BLK_COMB     = 1,
    ATSCMH_SCCC_BLK_RES      = 2,
}

//
// enum atscmh_sccc_code_mode - Type of Series Concatenated Convolutional
// Code Rate.
//
// @ATSCMH_SCCC_CODE_HLF:
// The outer code rate of a SCCC Block is 1/2 rate.
// @ATSCMH_SCCC_CODE_QTR:
// The outer code rate of a SCCC Block is 1/4 rate.
// @ATSCMH_SCCC_CODE_RES:
// Reserved. Should not be used.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum atscmh_sccc_code_mode {
    ATSCMH_SCCC_CODE_HLF     = 0,
    ATSCMH_SCCC_CODE_QTR     = 1,
    ATSCMH_SCCC_CODE_RES     = 2,
}

//
// enum atscmh_rs_frame_ensemble - Reed Solomon(RS) frame ensemble.
//
// @ATSCMH_RSFRAME_ENS_PRI:	Primary Ensemble.
// @ATSCMH_RSFRAME_ENS_SEC:	Secondary Ensemble.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum atscmh_rs_frame_ensemble {
    ATSCMH_RSFRAME_ENS_PRI   = 0,
    ATSCMH_RSFRAME_ENS_SEC   = 1,
}

//
// enum atscmh_rs_frame_mode - Reed Solomon (RS) frame mode.
//
// @ATSCMH_RSFRAME_PRI_ONLY:
// Single Frame: There is only a primary RS Frame for all Group
// Regions.
// @ATSCMH_RSFRAME_PRI_SEC:
// Dual Frame: There are two separate RS Frames: Primary RS Frame for
// Group Region A and B and Secondary RS Frame for Group Region C and
// D.
// @ATSCMH_RSFRAME_RES:
// Reserved. Shouldn't be used.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum atscmh_rs_frame_mode {
    ATSCMH_RSFRAME_PRI_ONLY  = 0,
    ATSCMH_RSFRAME_PRI_SEC   = 1,
    ATSCMH_RSFRAME_RES       = 2,
}

//
// enum atscmh_rs_code_mode - ATSC-M/H Reed Solomon modes
// @ATSCMH_RSCODE_211_187:	Reed Solomon code (211,187).
// @ATSCMH_RSCODE_223_187:	Reed Solomon code (223,187).
// @ATSCMH_RSCODE_235_187:	Reed Solomon code (235,187).
// @ATSCMH_RSCODE_RES:		Reserved. Shouldn't be used.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum atscmh_rs_code_mode {
    ATSCMH_RSCODE_211_187    = 0,
    ATSCMH_RSCODE_223_187    = 1,
    ATSCMH_RSCODE_235_187    = 2,
    ATSCMH_RSCODE_RES        = 3,
}

//
// enum fecap_scale_params - scale types for the quality parameters.
//
// @FE_SCALE_NOT_AVAILABLE: That QoS measure is not available. That
// could indicate a temporary or a permanent
// condition.
// @FE_SCALE_DECIBEL: The scale is measured in 0.001 dB steps, typically
// used on signal measures.
// @FE_SCALE_RELATIVE: The scale is a relative percentual measure,
// ranging from 0 (0%) to 0xffff (100%).
// @FE_SCALE_COUNTER: The scale counts the occurrence of an event, like
// bit error, block error, lapsed time.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fecap_scale_params {
    FE_SCALE_NOT_AVAILABLE = 0,
    FE_SCALE_DECIBEL,
    FE_SCALE_RELATIVE,
    FE_SCALE_COUNTER
}

//
// struct dtv_stats - Used for reading a DTV status property
//
// @scale:
// Filled with enum fecap_scale_params - the scale in usage
// for that parameter
//
// @svalue:
// integer value of the measure, for %FE_SCALE_DECIBEL,
// used for dB measures. The unit is 0.001 dB.
//
// @uvalue:
// unsigned integer value of the measure, used when @scale is
// either %FE_SCALE_RELATIVE or %FE_SCALE_COUNTER.
//
// For most delivery systems, this will return a single value for each
// parameter.
//
// It should be noticed, however, that new OFDM delivery systems like
// ISDB can use different modulation types for each group of carriers.
// On such standards, up to 8 groups of statistics can be provided, one
// for each carrier group (called "layer" on ISDB).
//
// In order to be consistent with other delivery systems, the first
// value refers to the entire set of carriers ("global").
//
// @scale should use the value %FE_SCALE_NOT_AVAILABLE when
// the value for the entire group of carriers or from one specific layer
// is not provided by the hardware.
//
// @len should be filled with the latest filled status + 1.
//
// In other words, for ISDB, those values should be filled like::
//
// u.st.stat.svalue[0] = global statistics;
// u.st.stat.scale[0] = FE_SCALE_DECIBEL;
// u.st.stat.value[1] = layer A statistics;
// u.st.stat.scale[1] = FE_SCALE_NOT_AVAILABLE (if not available);
// u.st.stat.svalue[2] = layer B statistics;
// u.st.stat.scale[2] = FE_SCALE_DECIBEL;
// u.st.stat.svalue[3] = layer C statistics;
// u.st.stat.scale[3] = FE_SCALE_DECIBEL;
// u.st.len = 4;
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dtv_stats {
    pub /: *mut *mut __u8 scale; / enum fecap_scale_params type,
    pub /: *mut *mut __u64 uvalue; / for counters and relative scales,
    pub /: *mut *mut __s64 svalue; / for 0.001 dB measures,
// C attribute field omitted
// C attribute field omitted
pub const MAX_DTV_STATS: c_int = 4;
//
// struct dtv_fe_stats - store Digital TV frontend statistics
//
// @len:	length of the statistics - if zero, stats is disabled.
// @stat:	array with digital TV statistics.
//
// On most standards, @len can either be 0 or 1. However, for ISDB, each
// layer is modulated in separate. So, each layer may have its own set
// of statistics. If so, stat[0] carries on a global value for the property.
// Indexes 1 to 3 means layer A to B.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dtv_fe_stats {
    pub len: __u8,
    pub stat: [dtv_stats; MAX_DTV_STATS],
// C attribute field omitted
//
// struct dtv_property - store one of frontend command and its value
//
// @cmd:		Digital TV command.
// @reserved:		Not used.
// @u:			Union with the values for the command.
// @u.data:		A unsigned 32 bits integer with command value.
// @u.buffer:		Struct to store bigger properties.
// Currently unused.
// @u.buffer.data:	an unsigned 32-bits array.
// @u.buffer.len:	number of elements of the buffer.
// @u.buffer.reserved1:	Reserved.
// @u.buffer.reserved2:	Reserved.
// @u.st:		a &struct dtv_fe_stats array of statistics.
// @result:		Currently unused.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dtv_property {
    pub cmd: __u32,
    pub reserved: [__u32; 3],
    pub data: __u32,
    pub st: dtv_fe_stats,
    pub data: [__u8; 32],
    pub len: __u32,
    pub reserved1: [__u32; 3],
    pub reserved2: *mut c_void,
    pub buffer: },
    pub u: },
    pub result: c_int,
// C attribute field omitted
// num of properties cannot exceed DTV_IOCTL_MAX_MSGS per ioctl
pub const DTV_IOCTL_MAX_MSGS: c_int = 64;
//
// struct dtv_properties - a set of command/value pairs.
//
// @num:	amount of commands stored at the struct.
// @props:	a pointer to &struct dtv_property.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dtv_properties {
    pub num: __u32,
    pub props: *mut dtv_property,
}

//
// When set, this flag will disable any zigzagging or other "normal" tuning
// behavior. Additionally, there will be no automatic monitoring of the lock
// status, and hence no frontend events will be generated. If a frontend device
// is closed, this flag will be automatically turned off when the device is
// reopened read-write.
//
pub const FE_TUNE_MODE_ONESHOT: c_uint = 0x01;
// Digital TV Frontend API calls

//
// DEPRECATED: Everything below is deprecated in favor of DVBv5 API
//
// The DVBv3 only ioctls, structs and enums should not be used on
// newer programs, as it doesn't support the second generation of
// digital TV standards, nor supports newer delivery systems.
// They also don't support modern frontends with usually support multiple
// delivery systems.
//
// Drivers shouldn't use them.
//
// New applications should use DVBv5 delivery system instead
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fe_bandwidth {
    BANDWIDTH_8_MHZ,
    BANDWIDTH_7_MHZ,
    BANDWIDTH_6_MHZ,
    BANDWIDTH_AUTO,
    BANDWIDTH_5_MHZ,
    BANDWIDTH_10_MHZ,
    BANDWIDTH_1_712_MHZ,
}

// This is kept for legacy userspace support
pub type fe_sec_voltage_t = fe_sec_voltage;
pub type fe_caps_t = fe_caps;
pub type fe_type_t = fe_type;
pub type fe_sec_tone_mode_t = fe_sec_tone_mode;
pub type fe_sec_mini_cmd_t = fe_sec_mini_cmd;
pub type fe_status_t = fe_status;
pub type fe_spectral_inversion_t = fe_spectral_inversion;
pub type fe_code_rate_t = fe_code_rate;
pub type fe_modulation_t = fe_modulation;
pub type fe_transmit_mode_t = fe_transmit_mode;
pub type fe_bandwidth_t = fe_bandwidth;
pub type fe_guard_interval_t = fe_guard_interval;
pub type fe_hierarchy_t = fe_hierarchy;
pub type fe_pilot_t = fe_pilot;
pub type fe_rolloff_t = fe_rolloff;
pub type fe_delivery_system_t = fe_delivery_system;
// DVBv3 structs
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dvb_qpsk_parameters {
    pub /: *mut *mut __u32 symbol_rate; / symbol rate in Symbols per second,
    pub /: *mut *mut fe_code_rate_t fec_inner; / forward error correction (see above),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dvb_qam_parameters {
    pub /: *mut *mut __u32 symbol_rate; / symbol rate in Symbols per second,
    pub /: *mut *mut fe_code_rate_t fec_inner; / forward error correction (see above),
    pub /: *mut *mut fe_modulation_t modulation; / modulation type (see above),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dvb_vsb_parameters {
    pub /: *mut *mut fe_modulation_t modulation; / modulation type (see above),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dvb_ofdm_parameters {
    pub bandwidth: fe_bandwidth_t,
    pub /: *mut *mut fe_code_rate_t code_rate_HP; / high priority stream code rate,
    pub /: *mut *mut fe_code_rate_t code_rate_LP; / low priority stream code rate,
    pub /: *mut *mut fe_modulation_t constellation; / modulation type (see above),
    pub transmission_mode: fe_transmit_mode_t,
    pub guard_interval: fe_guard_interval_t,
    pub hierarchy_information: fe_hierarchy_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dvb_frontend_parameters {
    pub /: *mut *mut __u32 frequency; / (absolute) frequency in Hz for DVB-C/DVB-T/ATSC,
// intermediate frequency in kHz for DVB-S
    pub inversion: fe_spectral_inversion_t,
    pub /: *mut *mut dvb_qpsk_parameters qpsk; / DVB-S,
    pub /: *mut *mut dvb_qam_parameters qam; / DVB-C,
    pub /: *mut *mut dvb_ofdm_parameters ofdm; / DVB-T,
    pub /: *mut *mut dvb_vsb_parameters vsb; / ATSC,
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dvb_frontend_event {
    pub status: fe_status_t,
    pub parameters: dvb_frontend_parameters,
}

// DVBv3 API calls

