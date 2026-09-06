//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/stmicro/stmmac/stmmac_ptp.h
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
//
pub const PTP_XGMAC_OFFSET: c_uint = 0xd00;
pub const PTP_GMAC4_OFFSET: c_uint = 0xb00;
pub const PTP_GMAC3_X_OFFSET: c_uint = 0x700;
// IEEE 1588 PTP register offsets
pub const PTP_TCR: c_uint = 0x00	/* Timestamp Control Reg */;
pub const PTP_SSIR: c_uint = 0x04	/* Sub-Second Increment Reg */;
pub const PTP_STSR: c_uint = 0x08	/* System Time – Seconds Regr */;
pub const PTP_STNSR: c_uint = 0x0c	/* System Time – Nanoseconds Reg */;
pub const PTP_STSUR: c_uint = 0x10	/* System Time – Seconds Update Reg */;
pub const PTP_STNSUR: c_uint = 0x14	/* System Time – Nanoseconds Update Reg */;
pub const PTP_TAR: c_uint = 0x18	/* Timestamp Addend Reg */;
pub const PTP_ACR: c_uint = 0x40	/* Auxiliary Control Reg */;
pub const PTP_ATNR: c_uint = 0x48	/* Auxiliary Timestamp - Nanoseconds Reg */;
pub const PTP_ATSR: c_uint = 0x4c	/* Auxiliary Timestamp - Seconds Reg */;
pub const PTP_TS_INGR_CORR_NS: c_uint = 0x58	/* Ingress timestamp correction nanoseconds */;
pub const PTP_TS_EGR_CORR_NS: c_uint = 0x5C	/* Egress timestamp correction nanoseconds*/;
pub const PTP_TS_INGR_CORR_SNS: c_uint = 0x60	/* Ingress timestamp correction subnanoseconds */;
pub const PTP_TS_EGR_CORR_SNS: c_uint = 0x64	/* Egress timestamp correction subnanoseconds */;
pub const PTP_TS_INGR_LAT: c_uint = 0x68	/* MAC internal Ingress Latency */;
pub const PTP_TS_EGR_LAT: c_uint = 0x6c	/* MAC internal Egress Latency */;
pub const PTP_STNSUR_ADDSUB_SHIFT: c_int = 31;
pub const PTP_DIGITAL_ROLLOVER_MODE: c_uint = 0x3B9ACA00	/* 10e9-1 ns */;
pub const PTP_BINARY_ROLLOVER_MODE: c_uint = 0x80000000	/* ~0.466 ns */;
// PTP Timestamp control register defines

// Enable PTP packet Processing for Version 2 Format

// Enable Processing of PTP over Ethernet Frames

// Enable Processing of PTP Frames Sent over IPv6-UDP

// Enable Processing of PTP Frames Sent over IPv4-UDP

// Enable Timestamp Snapshot for Event Messages

// Enable Snapshot for Messages Relevant to Master

// Select PTP packets for Taking Snapshots
// On gmac4 specifically:
// Enable SYNC, Pdelay_Req, Pdelay_Resp when TSEVNTENA is enabled.
// or
// Enable  SYNC, Follow_Up, Delay_Req, Delay_Resp, Pdelay_Req, Pdelay_Resp,
// Pdelay_Resp_Follow_Up if TSEVNTENA is disabled
//

// Enable MAC address for PTP Frame Filtering

// SSIR defines
pub const PTP_SSIR_SSINC_MAX: c_uint = 0xff;
pub const GMAC4_PTP_SSIR_SSINC_SHIFT: c_int = 16;
// Auxiliary Control defines

pub const PMC_ART_VALUE0: c_uint = 0x01	/* PMC_ART[15:0] timer value */;
pub const PMC_ART_VALUE1: c_uint = 0x02	/* PMC_ART[31:16] timer value */;
pub const PMC_ART_VALUE2: c_uint = 0x03	/* PMC_ART[47:32] timer value */;
pub const PMC_ART_VALUE3: c_uint = 0x04	/* PMC_ART[63:48] timer value */;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum aux_snapshot {
    AUX_SNAPSHOT0 = 0x10,
    AUX_SNAPSHOT1 = 0x20,
    AUX_SNAPSHOT2 = 0x40,
    AUX_SNAPSHOT3 = 0x80,
}

extern "C" {
    pub fn dwmac1000_get_ptptime(ptpaddr: *mut void __iomem, ptp_time: *mut u64);
}
extern "C" {
    pub fn dwmac1000_timestamp_interrupt(priv: *mut stmmac_priv);
}
