//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/tty/serial/zs.h
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
// zs.h: Definitions for the DECstation Z85C30 serial driver.
//
// Adapted from drivers/sbus/char/sunserial.h by Paul Mackerras.
// Adapted from drivers/macintosh/macserial.h by Harald Koerfgen.
//
// Copyright (C) 1996 Paul Mackerras (Paul.Mackerras@cs.anu.edu.au)
// Copyright (C) 1995 David S. Miller (davem@caip.rutgers.edu)
// Copyright (C) 2004, 2005, 2007  Maciej W. Rozycki
//

pub const ZS_NUM_REGS: c_int = 16;
//
// This is our internal structure for each serial port's state.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zs_port {
    pub /: *mut *mut *mut zs_scc scc; / Containing SCC.,
    pub /: *mut *mut uart_port port; / Underlying UART.,
    pub /: *mut *mut int clk_mode; / May be 1, 16, 32, or 64.,
    pub /: *mut *mut unsigned int tty_break; / Set on BREAK condition.,
    pub /: *mut *mut int tx_stopped; / Output is suspended.,
    pub /: *mut *mut unsigned int mctrl; / State of modem lines.,
    pub /: *mut *mut u8 brk; / BREAK state from RR0.,
    pub /: *mut *mut u8 regs[ZS_NUM_REGS]; / Channel write registers.,
}

//
// Per-SCC state for locking and the interrupt handler.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zs_scc {
    pub zport: [zs_port; 2],
    pub zlock: spinlock_t,
    pub irq_guard: core::sync::atomic::AtomicI32,
}

//
// Conversion routines to/from brg time constants from/to bits per second.
//

//
// The Zilog register set.
//
// Write Register 0 (Command)

pub const R1: c_int = 1;
pub const R2: c_int = 2;
pub const R3: c_int = 3;
pub const R4: c_int = 4;
pub const R5: c_int = 5;
pub const R6: c_int = 6;
pub const R7: c_int = 7;
pub const R8: c_int = 8;
pub const R9: c_int = 9;
pub const R10: c_int = 10;
pub const R11: c_int = 11;
pub const R12: c_int = 12;
pub const R13: c_int = 13;
pub const R14: c_int = 14;
pub const R15: c_int = 15;

pub const POINT_HIGH: c_uint = 0x8	/* Select upper half of registers */;
pub const RES_EXT_INT: c_uint = 0x10	/* Reset Ext. Status Interrupts */;
pub const SEND_ABORT: c_uint = 0x18	/* HDLC Abort */;
pub const RES_RxINT_FC: c_uint = 0x20	/* Reset RxINT on First Character */;
pub const RES_Tx_P: c_uint = 0x28	/* Reset TxINT Pending */;
pub const ERR_RES: c_uint = 0x30	/* Error Reset */;
pub const RES_H_IUS: c_uint = 0x38	/* Reset highest IUS */;
pub const RES_Rx_CRC: c_uint = 0x40	/* Reset Rx CRC Checker */;
pub const RES_Tx_CRC: c_uint = 0x80	/* Reset Tx CRC Checker */;
pub const RES_EOM_L: c_uint = 0xC0	/* Reset EOM latch */;
// Write Register 1 (Tx/Rx/Ext Int Enable and WAIT/DMA Commands)
pub const EXT_INT_ENAB: c_uint = 0x1	/* Ext Int Enable */;
pub const TxINT_ENAB: c_uint = 0x2	/* Tx Int Enable */;
pub const PAR_SPEC: c_uint = 0x4	/* Parity is special condition */;

pub const RxINT_FCERR: c_uint = 0x8	/* Rx Int on First Character Only or Error */;
pub const RxINT_ALL: c_uint = 0x10	/* Int on all Rx Characters or error */;
pub const RxINT_ERR: c_uint = 0x18	/* Int on error only */;
pub const RxINT_MASK: c_uint = 0x18;
pub const WT_RDY_RT: c_uint = 0x20	/* Wait/Ready on R/T */;
pub const WT_FN_RDYFN: c_uint = 0x40	/* Wait/FN/Ready FN */;
pub const WT_RDY_ENAB: c_uint = 0x80	/* Wait/Ready Enable */;
// Write Register 2 (Interrupt Vector)
// Write Register 3 (Receive Parameters and Control)
pub const RxENABLE: c_uint = 0x1	/* Rx Enable */;
pub const SYNC_L_INH: c_uint = 0x2	/* Sync Character Load Inhibit */;
pub const ADD_SM: c_uint = 0x4	/* Address Search Mode (SDLC) */;
pub const RxCRC_ENAB: c_uint = 0x8	/* Rx CRC Enable */;
pub const ENT_HM: c_uint = 0x10	/* Enter Hunt Mode */;
pub const AUTO_ENAB: c_uint = 0x20	/* Auto Enables */;
pub const Rx5: c_uint = 0x0	/* Rx 5 Bits/Character */;
pub const Rx7: c_uint = 0x40	/* Rx 7 Bits/Character */;
pub const Rx6: c_uint = 0x80	/* Rx 6 Bits/Character */;
pub const Rx8: c_uint = 0xc0	/* Rx 8 Bits/Character */;
pub const RxNBITS_MASK: c_uint = 0xc0;
// Write Register 4 (Transmit/Receive Miscellaneous Parameters and Modes)
pub const PAR_ENA: c_uint = 0x1	/* Parity Enable */;
pub const PAR_EVEN: c_uint = 0x2	/* Parity Even/Odd* */;

pub const SB1: c_uint = 0x4	/* 1 stop bit/char */;
pub const SB15: c_uint = 0x8	/* 1.5 stop bits/char */;
pub const SB2: c_uint = 0xc	/* 2 stop bits/char */;
pub const SB_MASK: c_uint = 0xc;

pub const BISYNC: c_uint = 0x10	/* 16 bit sync character */;
pub const SDLC: c_uint = 0x20	/* SDLC Mode (01111110 Sync Flag) */;
pub const EXTSYNC: c_uint = 0x30	/* External Sync Mode */;
pub const X1CLK: c_uint = 0x0	/* x1 clock mode */;
pub const X16CLK: c_uint = 0x40	/* x16 clock mode */;
pub const X32CLK: c_uint = 0x80	/* x32 clock mode */;
pub const X64CLK: c_uint = 0xc0	/* x64 clock mode */;
pub const XCLK_MASK: c_uint = 0xc0;
// Write Register 5 (Transmit Parameters and Controls)
pub const TxCRC_ENAB: c_uint = 0x1	/* Tx CRC Enable */;
pub const RTS: c_uint = 0x2	/* RTS */;
pub const SDLC_CRC: c_uint = 0x4	/* SDLC/CRC-16 */;
pub const TxENAB: c_uint = 0x8	/* Tx Enable */;
pub const SND_BRK: c_uint = 0x10	/* Send Break */;
pub const Tx5: c_uint = 0x0	/* Tx 5 bits (or less)/character */;
pub const Tx7: c_uint = 0x20	/* Tx 7 bits/character */;
pub const Tx6: c_uint = 0x40	/* Tx 6 bits/character */;
pub const Tx8: c_uint = 0x60	/* Tx 8 bits/character */;
pub const TxNBITS_MASK: c_uint = 0x60;
pub const DTR: c_uint = 0x80	/* DTR */;
// Write Register 6 (Sync bits 0-7/SDLC Address Field)
// Write Register 7 (Sync bits 8-15/SDLC 01111110)
// Write Register 8 (Transmit Buffer)
// Write Register 9 (Master Interrupt Control)

pub const STATHI: c_uint = 0x10	/* Status high */;
pub const SOFTACK: c_uint = 0x20	/* Software Interrupt Acknowledge */;

pub const CHRB: c_uint = 0x40	/* Reset channel B */;
pub const CHRA: c_uint = 0x80	/* Reset channel A */;
pub const FHWRES: c_uint = 0xc0	/* Force hardware reset */;
// Write Register 10 (Miscellaneous Transmitter/Receiver Control Bits)

pub const GAOP: c_uint = 0x10	/* Go active on poll */;

pub const NRZI: c_uint = 0x20	/* NRZI mode */;
pub const FM1: c_uint = 0x40	/* FM1 (transition = 1) */;
pub const FM0: c_uint = 0x60	/* FM0 (transition = 0) */;
pub const CRCPS: c_uint = 0x80	/* CRC Preset I/O */;
// Write Register 11 (Clock Mode Control)

pub const TCBR: c_uint = 0x10	/* Transmit clock = BR Generator output */;
pub const TCDPLL: c_uint = 0x18	/* Transmit clock = DPLL output */;

pub const RCTRxCP: c_uint = 0x20	/* Receive clock = TRxC pin */;
pub const RCBR: c_uint = 0x40	/* Receive clock = BR Generator output */;
pub const RCDPLL: c_uint = 0x60	/* Receive clock = DPLL output */;
pub const RTxCX: c_uint = 0x80	/* RTxC Xtal/No Xtal */;
// Write Register 12 (Lower Byte of Baud Rate Generator Time Constant)
// Write Register 13 (Upper Byte of Baud Rate Generator Time Constant)
// Write Register 14 (Miscellaneous Control Bits)

pub const LOOPBAK: c_uint = 0x10	/* Local loopback */;
pub const SEARCH: c_uint = 0x20	/* Enter search mode */;
pub const RMC: c_uint = 0x40	/* Reset missing clock */;
pub const DISDPLL: c_uint = 0x60	/* Disable DPLL */;
pub const SSBR: c_uint = 0x80	/* Set DPLL source = BR generator */;
pub const SSRTxC: c_uint = 0xa0	/* Set DPLL source = RTxC */;
pub const SFMM: c_uint = 0xc0	/* Set FM mode */;
pub const SNRZI: c_uint = 0xe0	/* Set NRZI mode */;
// Write Register 15 (External/Status Interrupt Control)

pub const SYNCIE: c_uint = 0x10	/* Sync/hunt IE */;
pub const CTSIE: c_uint = 0x20	/* CTS IE */;
pub const TxUIE: c_uint = 0x40	/* Tx Underrun/EOM IE */;
pub const BRKIE: c_uint = 0x80	/* Break/Abort IE */;
// Read Register 0 (Transmit/Receive Buffer Status and External Status)
pub const Rx_CH_AV: c_uint = 0x1	/* Rx Character Available */;
pub const ZCOUNT: c_uint = 0x2	/* Zero count */;
pub const Tx_BUF_EMP: c_uint = 0x4	/* Tx Buffer empty */;
pub const DCD: c_uint = 0x8	/* DCD */;
pub const SYNC_HUNT: c_uint = 0x10	/* Sync/hunt */;
pub const CTS: c_uint = 0x20	/* CTS */;
pub const TxEOM: c_uint = 0x40	/* Tx underrun */;
pub const BRK_ABRT: c_uint = 0x80	/* Break/Abort */;
// Read Register 1 (Special Receive Condition Status)
pub const ALL_SNT: c_uint = 0x1	/* All sent */;
// Residue Data for 8 Rx bits/char programmed
pub const RES3: c_uint = 0x8	/* 0/3 */;
pub const RES4: c_uint = 0x4	/* 0/4 */;
pub const RES5: c_uint = 0xc	/* 0/5 */;
pub const RES6: c_uint = 0x2	/* 0/6 */;
pub const RES7: c_uint = 0xa	/* 0/7 */;
pub const RES8: c_uint = 0x6	/* 0/8 */;
pub const RES18: c_uint = 0xe	/* 1/8 */;
pub const RES28: c_uint = 0x0	/* 2/8 */;
// Special Rx Condition Interrupts
pub const PAR_ERR: c_uint = 0x10	/* Parity Error */;
pub const Rx_OVR: c_uint = 0x20	/* Rx Overrun Error */;
pub const FRM_ERR: c_uint = 0x40	/* CRC/Framing Error */;
pub const END_FR: c_uint = 0x80	/* End of Frame (SDLC) */;
// Read Register 2 (Interrupt Vector (WR2) -- channel A).
// Read Register 2 (Modified Interrupt Vector -- channel B).
// Read Register 3 (Interrupt Pending Bits -- channel A only).
pub const CHBEXT: c_uint = 0x1	/* Channel B Ext/Stat IP */;
pub const CHBTxIP: c_uint = 0x2	/* Channel B Tx IP */;
pub const CHBRxIP: c_uint = 0x4	/* Channel B Rx IP */;
pub const CHAEXT: c_uint = 0x8	/* Channel A Ext/Stat IP */;
pub const CHATxIP: c_uint = 0x10	/* Channel A Tx IP */;
pub const CHARxIP: c_uint = 0x20	/* Channel A Rx IP */;
// Read Register 6 (SDLC FIFO Status and Byte Count LSB)
// Read Register 7 (SDLC FIFO Status and Byte Count MSB)
// Read Register 8 (Receive Data)
// Read Register 10 (Miscellaneous Status Bits)

pub const LOOPSEND: c_uint = 0x10	/* Loop sending */;
pub const CLK2MIS: c_uint = 0x40	/* Two clocks missing */;
pub const CLK1MIS: c_uint = 0x80	/* One clock missing */;
// Read Register 12 (Lower Byte of Baud Rate Generator Constant (WR12))
// Read Register 13 (Upper Byte of Baud Rate Generator Constant (WR13)
// Read Register 15 (External/Status Interrupt Control (WR15))
