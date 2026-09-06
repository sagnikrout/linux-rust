//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/serial/io_16654.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// 16654.H		Definitions for 16C654 UART used on EdgePorts
//
// Copyright (C) 1998 Inside Out Networks, Inc.
//

//
// D e f i n e s   /   T y p e d e f s
//
// UART register numbers
// Numbers 0-7 are passed to the Edgeport directly. Numbers 8 and
// above are used internally to indicate that we must enable access
// to them via LCR bit 0x80 or LCR = 0xBF.
// The register number sent to the Edgeport is then (x & 0x7).
//
// Driver must not access registers that affect operation of the
// the EdgePort firmware -- that includes THR, RHR, IER, FCR.

// efine unused			11	// Bank2[ 3 ]

pub const NUM_16654_REGS: c_int = 16;

//
// Bit definitions for each register
//
pub const IER_RX: c_uint = 0x01	// Enable receive interrupt;
pub const IER_TX: c_uint = 0x02	// Enable transmit interrupt;
pub const IER_RXS: c_uint = 0x04	// Enable receive status interrupt;
pub const IER_MDM: c_uint = 0x08	// Enable modem status interrupt;
pub const IER_SLEEP: c_uint = 0x10	// Enable sleep mode;
pub const IER_XOFF: c_uint = 0x20	// Enable s/w flow control (XOFF) interrupt;
pub const IER_RTS: c_uint = 0x40	// Enable RTS interrupt;
pub const IER_CTS: c_uint = 0x80	// Enable CTS interrupt;
pub const IER_ENABLE_ALL: c_uint = 0xFF	// Enable all ints;
pub const FCR_FIFO_EN: c_uint = 0x01	// Enable FIFOs;
pub const FCR_RXCLR: c_uint = 0x02	// Reset Rx FIFO;
pub const FCR_TXCLR: c_uint = 0x04	// Reset Tx FIFO;
pub const FCR_DMA_BLK: c_uint = 0x08	// Enable DMA block mode;
pub const FCR_TX_LEVEL_MASK: c_uint = 0x30	// Mask for Tx FIFO Level;
pub const FCR_TX_LEVEL_8: c_uint = 0x00	// Tx FIFO Level =  8 bytes;
pub const FCR_TX_LEVEL_16: c_uint = 0x10	// Tx FIFO Level = 16 bytes;
pub const FCR_TX_LEVEL_32: c_uint = 0x20	// Tx FIFO Level = 32 bytes;
pub const FCR_TX_LEVEL_56: c_uint = 0x30	// Tx FIFO Level = 56 bytes;
pub const FCR_RX_LEVEL_MASK: c_uint = 0xC0	// Mask for Rx FIFO Level;
pub const FCR_RX_LEVEL_8: c_uint = 0x00	// Rx FIFO Level =  8 bytes;
pub const FCR_RX_LEVEL_16: c_uint = 0x40	// Rx FIFO Level = 16 bytes;
pub const FCR_RX_LEVEL_56: c_uint = 0x80	// Rx FIFO Level = 56 bytes;
pub const FCR_RX_LEVEL_60: c_uint = 0xC0	// Rx FIFO Level = 60 bytes;
pub const ISR_INT_MDM_STATUS: c_uint = 0x00	// Modem status int pending;
pub const ISR_INT_NONE: c_uint = 0x01	// No interrupt pending;
pub const ISR_INT_TXRDY: c_uint = 0x02	// Tx ready int pending;
pub const ISR_INT_RXRDY: c_uint = 0x04	// Rx ready int pending;
pub const ISR_INT_LINE_STATUS: c_uint = 0x06	// Line status int pending;
pub const ISR_INT_RX_TIMEOUT: c_uint = 0x0C	// Rx timeout int pending;
pub const ISR_INT_RX_XOFF: c_uint = 0x10	// Rx Xoff int pending;
pub const ISR_INT_RTS_CTS: c_uint = 0x20	// RTS/CTS change int pending;
pub const ISR_FIFO_ENABLED: c_uint = 0xC0	// Bits set if FIFOs enabled;
pub const ISR_INT_BITS_MASK: c_uint = 0x3E	// Mask to isolate valid int causes;
pub const LCR_BITS_5: c_uint = 0x00	// 5 bits/char;
pub const LCR_BITS_6: c_uint = 0x01	// 6 bits/char;
pub const LCR_BITS_7: c_uint = 0x02	// 7 bits/char;
pub const LCR_BITS_8: c_uint = 0x03	// 8 bits/char;
pub const LCR_BITS_MASK: c_uint = 0x03	// Mask for bits/char field;
pub const LCR_STOP_1: c_uint = 0x00	// 1 stop bit;
pub const LCR_STOP_1_5: c_uint = 0x04	// 1.5 stop bits (if 5   bits/char);
pub const LCR_STOP_2: c_uint = 0x04	// 2 stop bits   (if 6-8 bits/char);
pub const LCR_STOP_MASK: c_uint = 0x04	// Mask for stop bits field;
pub const LCR_PAR_NONE: c_uint = 0x00	// No parity;
pub const LCR_PAR_ODD: c_uint = 0x08	// Odd parity;
pub const LCR_PAR_EVEN: c_uint = 0x18	// Even parity;
pub const LCR_PAR_MARK: c_uint = 0x28	// Force parity bit to 1;
pub const LCR_PAR_SPACE: c_uint = 0x38	// Force parity bit to 0;
pub const LCR_PAR_MASK: c_uint = 0x38	// Mask for parity field;
pub const LCR_SET_BREAK: c_uint = 0x40	// Set Break condition;
pub const LCR_DL_ENABLE: c_uint = 0x80	// Enable access to divisor latch;
pub const LCR_ACCESS_EFR: c_uint = 0xBF	// Load this value to access DLL,DLM,;
// and also the '654-only registers
// EFR, XON1, XON2, XOFF1, XOFF2
pub const MCR_DTR: c_uint = 0x01	// Assert DTR;
pub const MCR_RTS: c_uint = 0x02	// Assert RTS;
pub const MCR_OUT1: c_uint = 0x04	// Loopback only: Sets state of RI;
pub const MCR_MASTER_IE: c_uint = 0x08	// Enable interrupt outputs;
pub const MCR_LOOPBACK: c_uint = 0x10	// Set internal (digital) loopback mode;
pub const MCR_XON_ANY: c_uint = 0x20	// Enable any char to exit XOFF mode;
pub const MCR_IR_ENABLE: c_uint = 0x40	// Enable IrDA functions;
pub const MCR_BRG_DIV_4: c_uint = 0x80	// Divide baud rate clk by /4 instead of /1;
pub const LSR_RX_AVAIL: c_uint = 0x01	// Rx data available;
pub const LSR_OVER_ERR: c_uint = 0x02	// Rx overrun;
pub const LSR_PAR_ERR: c_uint = 0x04	// Rx parity error;
pub const LSR_FRM_ERR: c_uint = 0x08	// Rx framing error;
pub const LSR_BREAK: c_uint = 0x10	// Rx break condition detected;
pub const LSR_TX_EMPTY: c_uint = 0x20	// Tx Fifo empty;
pub const LSR_TX_ALL_EMPTY: c_uint = 0x40	// Tx Fifo and shift register empty;
pub const LSR_FIFO_ERR: c_uint = 0x80	// Rx Fifo contains at least 1 erred char;
pub const EDGEPORT_MSR_DELTA_CTS: c_uint = 0x01	// CTS changed from last read;
pub const EDGEPORT_MSR_DELTA_DSR: c_uint = 0x02	// DSR changed from last read;
pub const EDGEPORT_MSR_DELTA_RI: c_uint = 0x04	// RI  changed from 0 -> 1;
pub const EDGEPORT_MSR_DELTA_CD: c_uint = 0x08	// CD  changed from last read;
pub const EDGEPORT_MSR_CTS: c_uint = 0x10	// Current state of CTS;
pub const EDGEPORT_MSR_DSR: c_uint = 0x20	// Current state of DSR;
pub const EDGEPORT_MSR_RI: c_uint = 0x40	// Current state of RI;
pub const EDGEPORT_MSR_CD: c_uint = 0x80	// Current state of CD;
// Tx		Rx
// -------------------------------
pub const EFR_SWFC_NONE: c_uint = 0x00	//	None		None;
pub const EFR_SWFC_RX1: c_uint = 0x02 	//	None		XOFF1;
pub const EFR_SWFC_RX2: c_uint = 0x01 	//	None		XOFF2;
pub const EFR_SWFC_RX12: c_uint = 0x03 	//	None		XOFF1 & XOFF2;
pub const EFR_SWFC_TX1: c_uint = 0x08 	//	XOFF1		None;
pub const EFR_SWFC_TX1_RX1: c_uint = 0x0a 	//	XOFF1		XOFF1;
pub const EFR_SWFC_TX1_RX2: c_uint = 0x09 	//	XOFF1		XOFF2;
pub const EFR_SWFC_TX1_RX12: c_uint = 0x0b 	//	XOFF1		XOFF1 & XOFF2;
pub const EFR_SWFC_TX2: c_uint = 0x04 	//	XOFF2		None;
pub const EFR_SWFC_TX2_RX1: c_uint = 0x06 	//	XOFF2		XOFF1;
pub const EFR_SWFC_TX2_RX2: c_uint = 0x05 	//	XOFF2		XOFF2;
pub const EFR_SWFC_TX2_RX12: c_uint = 0x07 	//	XOFF2		XOFF1 & XOFF2;
pub const EFR_SWFC_TX12: c_uint = 0x0c 	//	XOFF1 & XOFF2	None;
pub const EFR_SWFC_TX12_RX1: c_uint = 0x0e 	//	XOFF1 & XOFF2	XOFF1;
pub const EFR_SWFC_TX12_RX2: c_uint = 0x0d 	//	XOFF1 & XOFF2	XOFF2;
pub const EFR_SWFC_TX12_RX12: c_uint = 0x0f 	//	XOFF1 & XOFF2	XOFF1 & XOFF2;
pub const EFR_TX_FC_MASK: c_uint = 0x0c	// Mask to isolate Rx flow control;
pub const EFR_TX_FC_NONE: c_uint = 0x00	// No Tx Xon/Xoff flow control;
pub const EFR_TX_FC_X1: c_uint = 0x08	// Transmit Xon1/Xoff1;
pub const EFR_TX_FC_X2: c_uint = 0x04	// Transmit Xon2/Xoff2;
pub const EFR_TX_FC_X1_2: c_uint = 0x0c	// Transmit Xon1&2/Xoff1&2;
pub const EFR_RX_FC_MASK: c_uint = 0x03	// Mask to isolate Rx flow control;
pub const EFR_RX_FC_NONE: c_uint = 0x00	// No Rx Xon/Xoff flow control;
pub const EFR_RX_FC_X1: c_uint = 0x02	// Receiver compares Xon1/Xoff1;
pub const EFR_RX_FC_X2: c_uint = 0x01	// Receiver compares Xon2/Xoff2;
pub const EFR_RX_FC_X1_2: c_uint = 0x03	// Receiver compares Xon1&2/Xoff1&2;
pub const EFR_SWFC_MASK: c_uint = 0x0F	// Mask for software flow control field;
pub const EFR_ENABLE_16654: c_uint = 0x10	// Enable 16C654 features;
pub const EFR_SPEC_DETECT: c_uint = 0x20	// Enable special character detect interrupt;
pub const EFR_AUTO_RTS: c_uint = 0x40	// Use RTS for Rx flow control;
pub const EFR_AUTO_CTS: c_uint = 0x80	// Use CTS for Tx flow control;
