//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/musb/musb_regs.h
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
// MUSB OTG driver register defines
//
// Copyright 2005 Mentor Graphics Corporation
// Copyright (C) 2005-2006 by Texas Instruments
// Copyright (C) 2006-2007 Nokia Corporation
//

//
// MUSB Register bits
//
// POWER
pub const MUSB_POWER_ISOUPDATE: c_uint = 0x80;
pub const MUSB_POWER_SOFTCONN: c_uint = 0x40;
pub const MUSB_POWER_HSENAB: c_uint = 0x20;
pub const MUSB_POWER_HSMODE: c_uint = 0x10;
pub const MUSB_POWER_RESET: c_uint = 0x08;
pub const MUSB_POWER_RESUME: c_uint = 0x04;
pub const MUSB_POWER_SUSPENDM: c_uint = 0x02;
pub const MUSB_POWER_ENSUSPEND: c_uint = 0x01;
// INTRUSB
pub const MUSB_INTR_SUSPEND: c_uint = 0x01;
pub const MUSB_INTR_RESUME: c_uint = 0x02;
pub const MUSB_INTR_RESET: c_uint = 0x04;
pub const MUSB_INTR_BABBLE: c_uint = 0x04;
pub const MUSB_INTR_SOF: c_uint = 0x08;
pub const MUSB_INTR_CONNECT: c_uint = 0x10;
pub const MUSB_INTR_DISCONNECT: c_uint = 0x20;
pub const MUSB_INTR_SESSREQ: c_uint = 0x40;
pub const MUSB_INTR_VBUSERROR: c_uint = 0x80	/* For SESSION end */;
// DEVCTL
pub const MUSB_DEVCTL_BDEVICE: c_uint = 0x80;
pub const MUSB_DEVCTL_FSDEV: c_uint = 0x40;
pub const MUSB_DEVCTL_LSDEV: c_uint = 0x20;
pub const MUSB_DEVCTL_VBUS: c_uint = 0x18;
pub const MUSB_DEVCTL_VBUS_SHIFT: c_int = 3;
pub const MUSB_DEVCTL_HM: c_uint = 0x04;
pub const MUSB_DEVCTL_HR: c_uint = 0x02;
pub const MUSB_DEVCTL_SESSION: c_uint = 0x01;
// BABBLE_CTL
pub const MUSB_BABBLE_FORCE_TXIDLE: c_uint = 0x80;
pub const MUSB_BABBLE_SW_SESSION_CTRL: c_uint = 0x40;
pub const MUSB_BABBLE_STUCK_J: c_uint = 0x20;
pub const MUSB_BABBLE_RCV_DISABLE: c_uint = 0x04;
// MUSB ULPI VBUSCONTROL
pub const MUSB_ULPI_USE_EXTVBUS: c_uint = 0x01;
pub const MUSB_ULPI_USE_EXTVBUSIND: c_uint = 0x02;
// ULPI_REG_CONTROL

// TESTMODE
pub const MUSB_TEST_FORCE_HOST: c_uint = 0x80;
pub const MUSB_TEST_FIFO_ACCESS: c_uint = 0x40;
pub const MUSB_TEST_FORCE_FS: c_uint = 0x20;
pub const MUSB_TEST_FORCE_HS: c_uint = 0x10;
pub const MUSB_TEST_PACKET: c_uint = 0x08;
pub const MUSB_TEST_K: c_uint = 0x04;
pub const MUSB_TEST_J: c_uint = 0x02;
pub const MUSB_TEST_SE0_NAK: c_uint = 0x01;
// Allocate for double-packet buffering (effectively doubles assigned _SIZE)
pub const MUSB_FIFOSZ_DPB: c_uint = 0x10;
// Allocation size (8, 16, 32, ... 4096)
pub const MUSB_FIFOSZ_SIZE: c_uint = 0x0f;
// CSR0
pub const MUSB_CSR0_FLUSHFIFO: c_uint = 0x0100;
pub const MUSB_CSR0_TXPKTRDY: c_uint = 0x0002;
pub const MUSB_CSR0_RXPKTRDY: c_uint = 0x0001;
// CSR0 in Peripheral mode
pub const MUSB_CSR0_P_SVDSETUPEND: c_uint = 0x0080;
pub const MUSB_CSR0_P_SVDRXPKTRDY: c_uint = 0x0040;
pub const MUSB_CSR0_P_SENDSTALL: c_uint = 0x0020;
pub const MUSB_CSR0_P_SETUPEND: c_uint = 0x0010;
pub const MUSB_CSR0_P_DATAEND: c_uint = 0x0008;
pub const MUSB_CSR0_P_SENTSTALL: c_uint = 0x0004;
// CSR0 in Host mode
pub const MUSB_CSR0_H_DIS_PING: c_uint = 0x0800;
pub const MUSB_CSR0_H_WR_DATATOGGLE: c_uint = 0x0400	/* Set to allow setting: */;
pub const MUSB_CSR0_H_DATATOGGLE: c_uint = 0x0200	/* Data toggle control */;
pub const MUSB_CSR0_H_NAKTIMEOUT: c_uint = 0x0080;
pub const MUSB_CSR0_H_STATUSPKT: c_uint = 0x0040;
pub const MUSB_CSR0_H_REQPKT: c_uint = 0x0020;
pub const MUSB_CSR0_H_ERROR: c_uint = 0x0010;
pub const MUSB_CSR0_H_SETUPPKT: c_uint = 0x0008;
pub const MUSB_CSR0_H_RXSTALL: c_uint = 0x0004;
// CSR0 bits to avoid zeroing (write zero clears, write 1 ignored)

// TxType/RxType
pub const MUSB_TYPE_SPEED: c_uint = 0xc0;
pub const MUSB_TYPE_SPEED_SHIFT: c_int = 6;
pub const MUSB_TYPE_PROTO: c_uint = 0x30	/* Implicitly zero for ep0 */;
pub const MUSB_TYPE_PROTO_SHIFT: c_int = 4;
pub const MUSB_TYPE_REMOTE_END: c_uint = 0xf	/* Implicitly zero for ep0 */;
// CONFIGDATA
pub const MUSB_CONFIGDATA_MPRXE: c_uint = 0x80	/* Auto bulk pkt combining */;
pub const MUSB_CONFIGDATA_MPTXE: c_uint = 0x40	/* Auto bulk pkt splitting */;
pub const MUSB_CONFIGDATA_BIGENDIAN: c_uint = 0x20;
pub const MUSB_CONFIGDATA_HBRXE: c_uint = 0x10	/* HB-ISO for RX */;
pub const MUSB_CONFIGDATA_HBTXE: c_uint = 0x08	/* HB-ISO for TX */;
pub const MUSB_CONFIGDATA_DYNFIFO: c_uint = 0x04	/* Dynamic FIFO sizing */;
pub const MUSB_CONFIGDATA_SOFTCONE: c_uint = 0x02	/* SoftConnect */;
pub const MUSB_CONFIGDATA_UTMIDW: c_uint = 0x01	/* Data width 0/1 => 8/16bits */;
// TXCSR in Peripheral and Host mode
pub const MUSB_TXCSR_AUTOSET: c_uint = 0x8000;
pub const MUSB_TXCSR_DMAENAB: c_uint = 0x1000;
pub const MUSB_TXCSR_FRCDATATOG: c_uint = 0x0800;
pub const MUSB_TXCSR_DMAMODE: c_uint = 0x0400;
pub const MUSB_TXCSR_CLRDATATOG: c_uint = 0x0040;
pub const MUSB_TXCSR_FLUSHFIFO: c_uint = 0x0008;
pub const MUSB_TXCSR_FIFONOTEMPTY: c_uint = 0x0002;
pub const MUSB_TXCSR_TXPKTRDY: c_uint = 0x0001;
// TXCSR in Peripheral mode
pub const MUSB_TXCSR_P_ISO: c_uint = 0x4000;
pub const MUSB_TXCSR_P_INCOMPTX: c_uint = 0x0080;
pub const MUSB_TXCSR_P_SENTSTALL: c_uint = 0x0020;
pub const MUSB_TXCSR_P_SENDSTALL: c_uint = 0x0010;
pub const MUSB_TXCSR_P_UNDERRUN: c_uint = 0x0004;
// TXCSR in Host mode
pub const MUSB_TXCSR_H_WR_DATATOGGLE: c_uint = 0x0200;
pub const MUSB_TXCSR_H_DATATOGGLE: c_uint = 0x0100;
pub const MUSB_TXCSR_H_NAKTIMEOUT: c_uint = 0x0080;
pub const MUSB_TXCSR_H_RXSTALL: c_uint = 0x0020;
pub const MUSB_TXCSR_H_ERROR: c_uint = 0x0004;
// TXCSR bits to avoid zeroing (write zero clears, write 1 ignored)

// RXCSR in Peripheral and Host mode
pub const MUSB_RXCSR_AUTOCLEAR: c_uint = 0x8000;
pub const MUSB_RXCSR_DMAENAB: c_uint = 0x2000;
pub const MUSB_RXCSR_DISNYET: c_uint = 0x1000;
pub const MUSB_RXCSR_PID_ERR: c_uint = 0x1000;
pub const MUSB_RXCSR_DMAMODE: c_uint = 0x0800;
pub const MUSB_RXCSR_INCOMPRX: c_uint = 0x0100;
pub const MUSB_RXCSR_CLRDATATOG: c_uint = 0x0080;
pub const MUSB_RXCSR_FLUSHFIFO: c_uint = 0x0010;
pub const MUSB_RXCSR_DATAERROR: c_uint = 0x0008;
pub const MUSB_RXCSR_FIFOFULL: c_uint = 0x0002;
pub const MUSB_RXCSR_RXPKTRDY: c_uint = 0x0001;
// RXCSR in Peripheral mode
pub const MUSB_RXCSR_P_ISO: c_uint = 0x4000;
pub const MUSB_RXCSR_P_SENTSTALL: c_uint = 0x0040;
pub const MUSB_RXCSR_P_SENDSTALL: c_uint = 0x0020;
pub const MUSB_RXCSR_P_OVERRUN: c_uint = 0x0004;
// RXCSR in Host mode
pub const MUSB_RXCSR_H_AUTOREQ: c_uint = 0x4000;
pub const MUSB_RXCSR_H_WR_DATATOGGLE: c_uint = 0x0400;
pub const MUSB_RXCSR_H_DATATOGGLE: c_uint = 0x0200;
pub const MUSB_RXCSR_H_RXSTALL: c_uint = 0x0040;
pub const MUSB_RXCSR_H_REQPKT: c_uint = 0x0020;
pub const MUSB_RXCSR_H_ERROR: c_uint = 0x0004;
// RXCSR bits to avoid zeroing (write zero clears, write 1 ignored)

// HUBADDR
pub const MUSB_HUBADDR_MULTI_TT: c_uint = 0x80;
//
// Common USB registers
//
pub const MUSB_FADDR: c_uint = 0x00	/* 8-bit */;
pub const MUSB_POWER: c_uint = 0x01	/* 8-bit */;
pub const MUSB_INTRTX: c_uint = 0x02	/* 16-bit */;
pub const MUSB_INTRRX: c_uint = 0x04;
pub const MUSB_INTRTXE: c_uint = 0x06;
pub const MUSB_INTRRXE: c_uint = 0x08;
pub const MUSB_INTRUSB: c_uint = 0x0A	/* 8 bit */;
pub const MUSB_INTRUSBE: c_uint = 0x0B	/* 8 bit */;
pub const MUSB_FRAME: c_uint = 0x0C;
pub const MUSB_INDEX: c_uint = 0x0E	/* 8 bit */;
pub const MUSB_TESTMODE: c_uint = 0x0F	/* 8 bit */;
//
// Additional Control Registers
//
pub const MUSB_DEVCTL: c_uint = 0x60	/* 8 bit */;
pub const MUSB_BABBLE_CTL: c_uint = 0x61	/* 8 bit */;
// These are always controlled through the INDEX register
pub const MUSB_TXFIFOSZ: c_uint = 0x62	/* 8-bit (see masks) */;
pub const MUSB_RXFIFOSZ: c_uint = 0x63	/* 8-bit (see masks) */;
pub const MUSB_TXFIFOADD: c_uint = 0x64	/* 16-bit offset shifted right 3 */;
pub const MUSB_RXFIFOADD: c_uint = 0x66	/* 16-bit offset shifted right 3 */;
// REVISIT: vctrl/vstatus: optional vendor utmi+phy register at 0x68
pub const MUSB_HWVERS: c_uint = 0x6C	/* 8 bit */;
pub const MUSB_ULPI_BUSCONTROL: c_uint = 0x70	/* 8 bit */;
pub const MUSB_ULPI_INT_MASK: c_uint = 0x72	/* 8 bit */;
pub const MUSB_ULPI_INT_SRC: c_uint = 0x73	/* 8 bit */;
pub const MUSB_ULPI_REG_DATA: c_uint = 0x74	/* 8 bit */;
pub const MUSB_ULPI_REG_ADDR: c_uint = 0x75	/* 8 bit */;
pub const MUSB_ULPI_REG_CONTROL: c_uint = 0x76	/* 8 bit */;
pub const MUSB_ULPI_RAW_DATA: c_uint = 0x77	/* 8 bit */;
pub const MUSB_EPINFO: c_uint = 0x78	/* 8 bit */;
pub const MUSB_RAMINFO: c_uint = 0x79	/* 8 bit */;
pub const MUSB_LINKINFO: c_uint = 0x7a	/* 8 bit */;
pub const MUSB_VPLEN: c_uint = 0x7b	/* 8 bit */;
pub const MUSB_HS_EOF1: c_uint = 0x7c	/* 8 bit */;
pub const MUSB_FS_EOF1: c_uint = 0x7d	/* 8 bit */;
pub const MUSB_LS_EOF1: c_uint = 0x7e	/* 8 bit */;
// Offsets to endpoint registers
pub const MUSB_TXMAXP: c_uint = 0x00;
pub const MUSB_TXCSR: c_uint = 0x02;

pub const MUSB_RXMAXP: c_uint = 0x04;
pub const MUSB_RXCSR: c_uint = 0x06;
pub const MUSB_RXCOUNT: c_uint = 0x08;

pub const MUSB_TXTYPE: c_uint = 0x0A;

pub const MUSB_TXINTERVAL: c_uint = 0x0B;

pub const MUSB_RXTYPE: c_uint = 0x0C;
pub const MUSB_RXINTERVAL: c_uint = 0x0D;
pub const MUSB_FIFOSIZE: c_uint = 0x0F;

pub const MUSB_TXCSR_MODE: c_uint = 0x2000;
// "bus control"/target registers, for host side multipoint (external hubs)
pub const MUSB_TXFUNCADDR: c_uint = 0x00;
pub const MUSB_TXHUBADDR: c_uint = 0x02;
pub const MUSB_TXHUBPORT: c_uint = 0x03;
pub const MUSB_RXFUNCADDR: c_uint = 0x04;
pub const MUSB_RXHUBADDR: c_uint = 0x06;
pub const MUSB_RXHUBPORT: c_uint = 0x07;
extern "C" {
    pub fn musb_readb(_arg: mbase, MUSB_CONFIGDATA: 0x10 +) -> return;
}
