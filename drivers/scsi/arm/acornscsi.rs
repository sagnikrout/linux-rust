//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/arm/acornscsi.h
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
// linux/drivers/acorn/scsi/acornscsi.h
//
// Copyright (C) 1997 Russell King
//
// Acorn SCSI driver
//
// SBIC registers
pub const SBIC_OWNID: c_int = 0;

pub const SBIC_CTRL: c_int = 1;

pub const CTRL_DMAPOLLED: c_int = 0;

pub const SBIC_TIMEOUT: c_int = 2;
pub const SBIC_TOTSECTS: c_int = 3;
pub const SBIC_TOTHEADS: c_int = 4;
pub const SBIC_TOTCYLH: c_int = 5;
pub const SBIC_TOTCYLL: c_int = 6;
pub const SBIC_LOGADDRH: c_int = 7;
pub const SBIC_LOGADDRM2: c_int = 8;
pub const SBIC_LOGADDRM1: c_int = 9;
pub const SBIC_LOGADDRL: c_int = 10;
pub const SBIC_SECTORNUM: c_int = 11;
pub const SBIC_HEADNUM: c_int = 12;
pub const SBIC_CYLH: c_int = 13;
pub const SBIC_CYLL: c_int = 14;
pub const SBIC_TARGETLUN: c_int = 15;

pub const SBIC_CMNDPHASE: c_int = 16;
pub const SBIC_SYNCHTRANSFER: c_int = 17;
pub const SYNCHTRANSFER_OF0: c_uint = 0x00;
pub const SYNCHTRANSFER_OF1: c_uint = 0x01;
pub const SYNCHTRANSFER_OF2: c_uint = 0x02;
pub const SYNCHTRANSFER_OF3: c_uint = 0x03;
pub const SYNCHTRANSFER_OF4: c_uint = 0x04;
pub const SYNCHTRANSFER_OF5: c_uint = 0x05;
pub const SYNCHTRANSFER_OF6: c_uint = 0x06;
pub const SYNCHTRANSFER_OF7: c_uint = 0x07;
pub const SYNCHTRANSFER_OF8: c_uint = 0x08;
pub const SYNCHTRANSFER_OF9: c_uint = 0x09;
pub const SYNCHTRANSFER_OF10: c_uint = 0x0A;
pub const SYNCHTRANSFER_OF11: c_uint = 0x0B;
pub const SYNCHTRANSFER_OF12: c_uint = 0x0C;
pub const SYNCHTRANSFER_8DBA: c_uint = 0x00;
pub const SYNCHTRANSFER_2DBA: c_uint = 0x20;
pub const SYNCHTRANSFER_3DBA: c_uint = 0x30;
pub const SYNCHTRANSFER_4DBA: c_uint = 0x40;
pub const SYNCHTRANSFER_5DBA: c_uint = 0x50;
pub const SYNCHTRANSFER_6DBA: c_uint = 0x60;
pub const SYNCHTRANSFER_7DBA: c_uint = 0x70;
pub const SBIC_TRANSCNTH: c_int = 18;
pub const SBIC_TRANSCNTM: c_int = 19;
pub const SBIC_TRANSCNTL: c_int = 20;
pub const SBIC_DESTID: c_int = 21;

pub const SBIC_SOURCEID: c_int = 22;

pub const SBIC_SSR: c_int = 23;
pub const SBIC_CMND: c_int = 24;
pub const CMND_RESET: c_uint = 0x00;
pub const CMND_ABORT: c_uint = 0x01;
pub const CMND_ASSERTATN: c_uint = 0x02;
pub const CMND_NEGATEACK: c_uint = 0x03;
pub const CMND_DISCONNECT: c_uint = 0x04;
pub const CMND_RESELECT: c_uint = 0x05;
pub const CMND_SELWITHATN: c_uint = 0x06;
pub const CMND_SELECT: c_uint = 0x07;
pub const CMND_SELECTATNTRANSFER: c_uint = 0x08;
pub const CMND_SELECTTRANSFER: c_uint = 0x09;
pub const CMND_RESELECTRXDATA: c_uint = 0x0A;
pub const CMND_RESELECTTXDATA: c_uint = 0x0B;
pub const CMND_WAITFORSELRECV: c_uint = 0x0C;
pub const CMND_SENDSTATCMD: c_uint = 0x0D;
pub const CMND_SENDDISCONNECT: c_uint = 0x0E;
pub const CMND_SETIDI: c_uint = 0x0F;
pub const CMND_RECEIVECMD: c_uint = 0x10;
pub const CMND_RECEIVEDTA: c_uint = 0x11;
pub const CMND_RECEIVEMSG: c_uint = 0x12;
pub const CMND_RECEIVEUSP: c_uint = 0x13;
pub const CMND_SENDCMD: c_uint = 0x14;
pub const CMND_SENDDATA: c_uint = 0x15;
pub const CMND_SENDMSG: c_uint = 0x16;
pub const CMND_SENDUSP: c_uint = 0x17;
pub const CMND_TRANSLATEADDR: c_uint = 0x18;
pub const CMND_XFERINFO: c_uint = 0x20;

pub const SBIC_DATA: c_int = 25;
pub const SBIC_ASR: c_int = 26;

// DMAC registers
pub const DMAC_INIT: c_uint = 0x00;

pub const DMAC_CHANNEL: c_uint = 0x80;
pub const CHANNEL_0: c_uint = 0x00;
pub const CHANNEL_1: c_uint = 0x01;
pub const CHANNEL_2: c_uint = 0x02;
pub const CHANNEL_3: c_uint = 0x03;
pub const DMAC_TXCNTLO: c_uint = 0x01;
pub const DMAC_TXCNTHI: c_uint = 0x81;
pub const DMAC_TXADRLO: c_uint = 0x02;
pub const DMAC_TXADRMD: c_uint = 0x82;
pub const DMAC_TXADRHI: c_uint = 0x03;
pub const DMAC_DEVCON0: c_uint = 0x04;

pub const DMAC_DEVCON1: c_uint = 0x84;

pub const DMAC_MODECON: c_uint = 0x05;
pub const MODECON_WOED: c_uint = 0x01;
pub const MODECON_VERIFY: c_uint = 0x00;
pub const MODECON_READ: c_uint = 0x04;
pub const MODECON_WRITE: c_uint = 0x08;
pub const MODECON_AUTOINIT: c_uint = 0x10;
pub const MODECON_ADDRDIR: c_uint = 0x20;
pub const MODECON_DEMAND: c_uint = 0x00;
pub const MODECON_SINGLE: c_uint = 0x40;
pub const MODECON_BLOCK: c_uint = 0x80;
pub const MODECON_CASCADE: c_uint = 0xC0;
pub const DMAC_STATUS: c_uint = 0x85;

pub const DMAC_TEMPLO: c_uint = 0x06;
pub const DMAC_TEMPHI: c_uint = 0x86;
pub const DMAC_REQREG: c_uint = 0x07;
pub const DMAC_MASKREG: c_uint = 0x87;
pub const MASKREG_M0: c_uint = 0x01;
pub const MASKREG_M1: c_uint = 0x02;
pub const MASKREG_M2: c_uint = 0x04;
pub const MASKREG_M3: c_uint = 0x08;
// miscellaneous internal variables

//
// SCSI driver phases
//
// After interrupt, what to do now
//
// DMA direction
//
// Synchronous transfer state
//
// Command type
//
// Data phase direction
//

pub const STATUS_BUFFER_SIZE: c_int = 32;
//
// This is used to dump the previous states of the SBIC
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct status_entry {
    pub when: c_ulong,
    pub ssr: c_uchar,
    pub ph: c_uchar,
    pub irq: c_uchar,
    pub unused: c_uchar,
}

//
// AcornSCSI host specific data
//
// miscellaneous
// driver information
// statistics information
// queue handling
// per-device info
// DMA info
// card info
