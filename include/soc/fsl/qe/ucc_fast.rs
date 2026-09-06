//! Automatically rewritten from C Header to Rust Module
//! Source: include/soc/fsl/qe/ucc_fast.h
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
// Internal header file for UCC FAST unit routines.
//
// Copyright (C) 2006 Freescale Semiconductor, Inc. All rights reserved.
//
// Authors: 	Shlomi Gridish <gridish@freescale.com>
// Li Yang <leoli@freescale.com>
//

// Receive BD's status and length
pub const R_E: c_uint = 0x80000000	/* buffer empty */;
pub const R_W: c_uint = 0x20000000	/* wrap bit */;
pub const R_I: c_uint = 0x10000000	/* interrupt on reception */;
pub const R_L: c_uint = 0x08000000	/* last */;
pub const R_F: c_uint = 0x04000000	/* first */;
// transmit BD's status and length
pub const T_R: c_uint = 0x80000000	/* ready bit */;
pub const T_W: c_uint = 0x20000000	/* wrap bit */;
pub const T_I: c_uint = 0x10000000	/* interrupt on completion */;
pub const T_L: c_uint = 0x08000000	/* last */;
// Receive BD's status
pub const R_E_S: c_uint = 0x8000	/* buffer empty */;
pub const R_W_S: c_uint = 0x2000	/* wrap bit */;
pub const R_I_S: c_uint = 0x1000	/* interrupt on reception */;
pub const R_L_S: c_uint = 0x0800	/* last */;
pub const R_F_S: c_uint = 0x0400	/* first */;
pub const R_CM_S: c_uint = 0x0200	/* continuous mode */;
pub const R_LG_S: c_uint = 0x0020  /* frame length */;
pub const R_NO_S: c_uint = 0x0010  /* nonoctet */;
pub const R_AB_S: c_uint = 0x0008  /* abort */;
pub const R_CR_S: c_uint = 0x0004	/* crc */;
pub const R_OV_S: c_uint = 0x0002	/* overrun */;
pub const R_CD_S: c_uint = 0x0001  /* carrier detect */;
// transmit BD's status
pub const T_R_S: c_uint = 0x8000	/* ready bit */;
pub const T_W_S: c_uint = 0x2000	/* wrap bit */;
pub const T_I_S: c_uint = 0x1000	/* interrupt on completion */;
pub const T_L_S: c_uint = 0x0800	/* last */;
pub const T_TC_S: c_uint = 0x0400	/* crc */;
pub const T_TM_S: c_uint = 0x0200	/* continuous mode */;
pub const T_UN_S: c_uint = 0x0002  /* hdlc underrun */;
pub const T_CT_S: c_uint = 0x0001  /* hdlc carrier lost */;
// Rx Data buffer must be 4 bytes aligned in most cases
pub const UCC_FAST_RX_ALIGN: c_int = 4;
pub const UCC_FAST_MRBLR_ALIGNMENT: c_int = 4;
pub const UCC_FAST_VIRT_FIFO_REGS_ALIGNMENT: c_int = 8;
// Sizes
pub const UCC_FAST_URFS_MIN_VAL: c_uint = 0x88;
pub const UCC_FAST_RECEIVE_VIRTUAL_FIFO_SIZE_FUDGE_FACTOR: c_int = 8;
// ucc_fast_channel_protocol_mode - UCC FAST mode
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ucc_fast_channel_protocol_mode {
    UCC_FAST_PROTOCOL_MODE_HDLC = 0x00000000,
    UCC_FAST_PROTOCOL_MODE_RESERVED01 = 0x00000001,
    UCC_FAST_PROTOCOL_MODE_RESERVED_QMC = 0x00000002,
    UCC_FAST_PROTOCOL_MODE_RESERVED02 = 0x00000003,
    UCC_FAST_PROTOCOL_MODE_RESERVED_UART = 0x00000004,
    UCC_FAST_PROTOCOL_MODE_RESERVED03 = 0x00000005,
    UCC_FAST_PROTOCOL_MODE_RESERVED_EX_MAC_1 = 0x00000006,
    UCC_FAST_PROTOCOL_MODE_RESERVED_EX_MAC_2 = 0x00000007,
    UCC_FAST_PROTOCOL_MODE_RESERVED_BISYNC = 0x00000008,
    UCC_FAST_PROTOCOL_MODE_RESERVED04 = 0x00000009,
    UCC_FAST_PROTOCOL_MODE_ATM = 0x0000000A,
    UCC_FAST_PROTOCOL_MODE_RESERVED05 = 0x0000000B,
    UCC_FAST_PROTOCOL_MODE_ETHERNET = 0x0000000C,
    UCC_FAST_PROTOCOL_MODE_RESERVED06 = 0x0000000D,
    UCC_FAST_PROTOCOL_MODE_POS = 0x0000000E,
    UCC_FAST_PROTOCOL_MODE_RESERVED07 = 0x0000000F
}

// ucc_fast_transparent_txrx - UCC Fast Transparent TX & RX
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ucc_fast_transparent_txrx {
    UCC_FAST_GUMR_TRANSPARENT_TTX_TRX_NORMAL = 0x00000000,
    UCC_FAST_GUMR_TRANSPARENT_TTX_TRX_TRANSPARENT = 0x18000000
}

// UCC fast diagnostic mode
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ucc_fast_diag_mode {
    UCC_FAST_DIAGNOSTIC_NORMAL = 0x0,
    UCC_FAST_DIAGNOSTIC_LOCAL_LOOP_BACK = 0x40000000,
    UCC_FAST_DIAGNOSTIC_AUTO_ECHO = 0x80000000,
    UCC_FAST_DIAGNOSTIC_LOOP_BACK_AND_ECHO = 0xC0000000
}

// UCC fast Sync length (transparent mode only)
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ucc_fast_sync_len {
    UCC_FAST_SYNC_LEN_NOT_USED = 0x0,
    UCC_FAST_SYNC_LEN_AUTOMATIC = 0x00004000,
    UCC_FAST_SYNC_LEN_8_BIT = 0x00008000,
    UCC_FAST_SYNC_LEN_16_BIT = 0x0000C000
}

// UCC fast RTS mode
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ucc_fast_ready_to_send {
    UCC_FAST_SEND_IDLES_BETWEEN_FRAMES = 0x00000000,
    UCC_FAST_SEND_FLAGS_BETWEEN_FRAMES = 0x00002000
}

// UCC fast receiver decoding mode
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ucc_fast_rx_decoding_method {
    UCC_FAST_RX_ENCODING_NRZ = 0x00000000,
    UCC_FAST_RX_ENCODING_NRZI = 0x00000800,
    UCC_FAST_RX_ENCODING_RESERVED0 = 0x00001000,
    UCC_FAST_RX_ENCODING_RESERVED1 = 0x00001800
}

// UCC fast transmitter encoding mode
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ucc_fast_tx_encoding_method {
    UCC_FAST_TX_ENCODING_NRZ = 0x00000000,
    UCC_FAST_TX_ENCODING_NRZI = 0x00000100,
    UCC_FAST_TX_ENCODING_RESERVED0 = 0x00000200,
    UCC_FAST_TX_ENCODING_RESERVED1 = 0x00000300
}

// UCC fast CRC length
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ucc_fast_transparent_tcrc {
    UCC_FAST_16_BIT_CRC = 0x00000000,
    UCC_FAST_CRC_RESERVED0 = 0x00000040,
    UCC_FAST_32_BIT_CRC = 0x00000080,
    UCC_FAST_CRC_RESERVED1 = 0x000000C0
}

// Fast UCC initialization structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ucc_fast_info {
    pub ucc_num: c_int,
    pub tdm_num: c_int,
    pub rx_clock: qe_clock,
    pub tx_clock: qe_clock,
    pub rx_sync: qe_clock,
    pub tx_sync: qe_clock,
    pub regs: resource_size_t,
    pub irq: c_int,
    pub uccm_mask: u32,
    pub brkpt_support: c_int,
    pub grant_support: c_int,
    pub tsa: c_int,
    pub cdp: c_int,
    pub cds: c_int,
    pub ctsp: c_int,
    pub ctss: c_int,
    pub tci: c_int,
    pub txsy: c_int,
    pub rtsm: c_int,
    pub revd: c_int,
    pub rsyn: c_int,
    pub max_rx_buf_length: u16,
    pub urfs: u16,
    pub urfet: u16,
    pub urfset: u16,
    pub utfs: u16,
    pub utfet: u16,
    pub utftt: u16,
    pub ufpt: u16,
    pub mode: ucc_fast_channel_protocol_mode,
    pub ttx_trx: ucc_fast_transparent_txrx,
    pub tenc: ucc_fast_tx_encoding_method,
    pub renc: ucc_fast_rx_decoding_method,
    pub tcrc: ucc_fast_transparent_tcrc,
    pub synl: ucc_fast_sync_len,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ucc_fast_private {
    pub uf_info: *mut ucc_fast_info,
    pub /: *mut *mut *mut ucc_fast __iomem uf_regs; / a pointer to the UCC regs.,
    pub /: *mut *mut *mut __be32 __iomem p_ucce; / a pointer to the event register in memory.,
    pub /: *mut *mut *mut __be32 __iomem p_uccm; / a pointer to the mask register in memory.,

    pub /: *mut *mut *mut __be16 __iomem p_utodr;/ pointer to the transmit on demand register,

    pub /: *mut *mut int enabled_tx; / Whether channel is enabled for Tx (ENT),
    pub /: *mut *mut int enabled_rx; / Whether channel is enabled for Rx (ENR),
    pub Tx: *mut *mut int stopped_tx; / Whether channel has been stopped for,
    pub /: *mut *mut int stopped_rx; / Whether channel has been stopped for Rx,
    pub Tx: *mut *mut s32 ucc_fast_tx_virtual_fifo_base_offset;/ pointer to base of,
    pub Rx: *mut *mut s32 ucc_fast_rx_virtual_fifo_base_offset;/ pointer to base of,

    pub /: *mut *mut u32 tx_frames; / Transmitted frames counter.,
    pub frames: *mut *mut u32 rx_frames; / Received frames counter (only,
    pub that: *mut *mut u32 tx_discarded; / Discarded tx frames counter (frames,
//
    pub that: *mut *mut u32 rx_discarded; / Discarded rx frames counter (frames,
//

    pub /: *mut *mut u16 mrblr; / maximum receive buffer length,
}

// ucc_fast_init
// Initializes Fast UCC according to user provided parameters.
//
// uf_info  - (In) pointer to the fast UCC info structure.
// uccf_ret - (Out) pointer to the fast UCC structure.
//
extern "C" {
    pub fn ucc_fast_init(uf_info: *mut *mut ucc_fast_info, uccf_ret: *mut *mut *mut ucc_fast_private) -> c_int;
}
// ucc_fast_free
// Frees all resources for fast UCC.
//
// uccf - (In) pointer to the fast UCC structure.
//
extern "C" {
    pub fn ucc_fast_free(uccf: *mut *mut ucc_fast_private);
}
// ucc_fast_enable
// Enables a fast UCC port.
// This routine enables Tx and/or Rx through the General UCC Mode Register.
//
// uccf - (In) pointer to the fast UCC structure.
// mode - (In) TX, RX, or both.
//
extern "C" {
    pub fn ucc_fast_enable(uccf: *mut *mut ucc_fast_private, mode: comm_dir);
}
// ucc_fast_disable
// Disables a fast UCC port.
// This routine disables Tx and/or Rx through the General UCC Mode Register.
//
// uccf - (In) pointer to the fast UCC structure.
// mode - (In) TX, RX, or both.
//
extern "C" {
    pub fn ucc_fast_disable(uccf: *mut *mut ucc_fast_private, mode: comm_dir);
}
// ucc_fast_irq
// Handles interrupts on fast UCC.
// Called from the general interrupt routine to handle interrupts on fast UCC.
//
// uccf - (In) pointer to the fast UCC structure.
//
extern "C" {
    pub fn ucc_fast_irq(uccf: *mut *mut ucc_fast_private);
}
// ucc_fast_transmit_on_demand
// Immediately forces a poll of the transmitter for data to be sent.
// Typically, the hardware performs a periodic poll for data that the
// transmit routine has set up to be transmitted. In cases where
// this polling cycle is not soon enough, this optional routine can
// be invoked to force a poll right away, instead. Proper use for
// each transmission for which this functionality is desired is to
// call the transmit routine and then this routine right after.
//
// uccf - (In) pointer to the fast UCC structure.
//
extern "C" {
    pub fn ucc_fast_transmit_on_demand(uccf: *mut *mut ucc_fast_private);
}
extern "C" {
    pub fn ucc_fast_get_qe_cr_subblock(uccf_num: c_int) -> u32;
}
extern "C" {
    pub fn ucc_fast_dump_regs(uccf: *mut *mut ucc_fast_private);
}
