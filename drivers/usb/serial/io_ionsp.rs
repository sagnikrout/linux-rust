//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/serial/io_ionsp.h
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
// IONSP.H		Definitions for I/O Networks Serial Protocol
//
// Copyright (C) 1997-1998 Inside Out Networks, Inc.
//
// These definitions are used by both kernel-mode driver and the
// peripheral firmware and MUST be kept in sync.
//
// Define format of InterruptStatus packet returned from the
// Interrupt pipe
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct int_status_pkt {
    pub to: __u16 RxBytesAvail; // Additional bytes available,
// be read from Bulk IN pipe
    pub in: __u16 TxCredits[MAX_RS232_PORTS]; // Additional space available,
// given port's TxBuffer
}

//
// Define cmd/status header values and macros to extract them.
//
// Data:		0LLLLPPP LLLLLLLL
// Cmd/Stat:	1ccccPPP CCCCCCCC
pub const IOSP_DATA_HDR_SIZE: c_int = 2;
pub const IOSP_CMD_HDR_SIZE: c_int = 2;
pub const IOSP_MAX_DATA_LENGTH: c_uint = 0x0FFF		// 12 bits -> 4K;
pub const IOSP_PORT_MASK: c_uint = 0x07		// Mask to isolate port number;
pub const IOSP_CMD_STAT_BIT: c_uint = 0x80		// If set, this is command/status header;

//
// These macros build the 1st and 2nd bytes for a data header
//

//
// These macros build the 1st and 2nd bytes for a command header
//

// --------------------------------------------------------------
//
// Define values for commands and command parameters
// (sent from Host to Edgeport)
//
// 1ccccPPP P1P1P1P1 [ P2P2P2P2P2 ]...
//
// cccc:	00-07	2-byte commands. Write UART register 0-7 with
// value in P1. See 16650.H for definitions of
// UART register numbers and contents.
//
// 08-0B	3-byte commands:					==== P1 ====	==== P2 ====
// 08	available for expansion
// 09	1-param commands		Command Code	Param
// 0A	available for expansion
// 0B	available for expansion
//
// 0C-0D	4-byte commands.	P1 = extended cmd and P2,P3 = params
// Currently unimplemented.
//
// 0E-0F	N-byte commands:	P1 = num bytes after P1 (ie, TotalLen - 2)
// P2 = extended cmd, P3..Pn = parameters.
// Currently unimplemented.
//

// Register numbers and contents
// defined in 16554.H.
// 0x08		// Available for expansion.
pub const IOSP_EXT_CMD: c_uint = 0x09		// P1 = Command code (defined below);
// P2 = Parameter
//
// Extended Command values, used with IOSP_EXT_CMD, may
// or may not use parameter P2.
//
pub const IOSP_CMD_OPEN_PORT: c_uint = 0x00		// Enable ints, init UART. (NO PARAM);
pub const IOSP_CMD_CLOSE_PORT: c_uint = 0x01		// Disable ints, flush buffers. (NO PARAM);
pub const IOSP_CMD_CHASE_PORT: c_uint = 0x02		// Wait for Edgeport TX buffers to empty. (NO PARAM);
pub const IOSP_CMD_SET_RX_FLOW: c_uint = 0x03		// Set Rx Flow Control in Edgeport;
pub const IOSP_CMD_SET_TX_FLOW: c_uint = 0x04		// Set Tx Flow Control in Edgeport;
pub const IOSP_CMD_SET_XON_CHAR: c_uint = 0x05		// Set XON Character in Edgeport;
pub const IOSP_CMD_SET_XOFF_CHAR: c_uint = 0x06		// Set XOFF Character in Edgeport;
pub const IOSP_CMD_RX_CHECK_REQ: c_uint = 0x07		// Request Edgeport to insert a Checkpoint into;
// the receive data stream (Parameter = 1 byte sequence number)
pub const IOSP_CMD_SET_BREAK: c_uint = 0x08		// Turn on the BREAK (LCR bit 6);
pub const IOSP_CMD_CLEAR_BREAK: c_uint = 0x09		// Turn off the BREAK (LCR bit 6);
//
// Define macros to simplify building of IOSP cmds
//

// ppBuf += 2;							\
// pLen  += 2;							\

// ppBuf += 3;							\
// pLen  += 3;							\
// --------------------------------------------------------------
//
// Define format of flow control commands
// (sent from Host to Edgeport)
//
// 11001PPP FlowCmd FlowTypes
//
// Note that the 'FlowTypes' parameter is a bit mask; that is,
// more than one flow control type can be active at the same time.
// FlowTypes = 0 means 'no flow control'.
//
// IOSP_CMD_SET_RX_FLOW
//
// Tells Edgeport how it can stop incoming UART data
//
// Example for Port 0
// P0 = 11001000
// P1 = IOSP_CMD_SET_RX_FLOW
// P2 = Bit mask as follows:
pub const IOSP_RX_FLOW_RTS: c_uint = 0x01	// Edgeport drops RTS to stop incoming data;
pub const IOSP_RX_FLOW_DTR: c_uint = 0x02	// Edgeport drops DTR to stop incoming data;
pub const IOSP_RX_FLOW_DSR_SENSITIVITY: c_uint = 0x04	// Ignores Rx data unless DSR high;
// Not currently implemented by firmware.
pub const IOSP_RX_FLOW_XON_XOFF: c_uint = 0x08	// Edgeport sends XOFF char to stop incoming data.;
// Host must have previously programmed the
// XON/XOFF values with SET_XON/SET_XOFF
// before enabling this bit.
//
// IOSP_CMD_SET_TX_FLOW
//
// Tells Edgeport what signal(s) will stop it from transmitting UART data
//
// Example for Port 0
// P0 = 11001000
// P1 = IOSP_CMD_SET_TX_FLOW
// P2 = Bit mask as follows:
pub const IOSP_TX_FLOW_CTS: c_uint = 0x01	// Edgeport stops Tx if CTS low;
pub const IOSP_TX_FLOW_DSR: c_uint = 0x02	// Edgeport stops Tx if DSR low;
pub const IOSP_TX_FLOW_DCD: c_uint = 0x04	// Edgeport stops Tx if DCD low;
pub const IOSP_TX_FLOW_XON_XOFF: c_uint = 0x08	// Edgeport stops Tx upon receiving XOFF char.;
// Host must have previously programmed the
// XON/XOFF values with SET_XON/SET_XOFF
// before enabling this bit.
pub const IOSP_TX_FLOW_XOFF_CONTINUE: c_uint = 0x10	// If not set, Edgeport stops Tx when;
// sending XOFF in order to fix broken
// systems that interpret the next
// received char as XON.
// If set, Edgeport continues Tx
// normally after transmitting XOFF.
// Not currently implemented by firmware.
pub const IOSP_TX_TOGGLE_RTS: c_uint = 0x20	// Edgeport drives RTS as a true half-duplex;
// Request-to-Send signal: it is raised before
// beginning transmission and lowered after
// the last Tx char leaves the UART.
// Not currently implemented by firmware.
//
// IOSP_CMD_SET_XON_CHAR
//
// Sets the character which Edgeport transmits/interprets as XON.
// Note: This command MUST be sent before sending a SET_RX_FLOW or
// SET_TX_FLOW with the XON_XOFF bit set.
//
// Example for Port 0
// P0 = 11001000
// P1 = IOSP_CMD_SET_XON_CHAR
// P2 = 0x11
//
// IOSP_CMD_SET_XOFF_CHAR
//
// Sets the character which Edgeport transmits/interprets as XOFF.
// Note: This command must be sent before sending a SET_RX_FLOW or
// SET_TX_FLOW with the XON_XOFF bit set.
//
// Example for Port 0
// P0 = 11001000
// P1 = IOSP_CMD_SET_XOFF_CHAR
// P2 = 0x13
//
// IOSP_CMD_RX_CHECK_REQ
//
// This command is used to assist in the implementation of the
// IOCTL_SERIAL_PURGE Windows IOCTL.
// This IOSP command tries to place a marker at the end of the RX
// queue in the Edgeport. If the Edgeport RX queue is full then
// the Check will be discarded.
// It is up to the device driver to timeout waiting for the
// RX_CHECK_RSP.  If a RX_CHECK_RSP is received, the driver is
// sure that all data has been received from the edgeport and
// may now purge any internal RX buffers.
// Note tat the sequence numbers may be used to detect lost
// CHECK_REQs.
// Example for Port 0
// P0 = 11001000
// P1 = IOSP_CMD_RX_CHECK_REQ
// P2 = Sequence number
// Response will be:
// P1 = IOSP_EXT_RX_CHECK_RSP
// P2 = Request Sequence number
// --------------------------------------------------------------
//
// Define values for status and status parameters
// (received by Host from Edgeport)
//
// 1ssssPPP P1P1P1P1 [ P2P2P2P2P2 ]...
//
// ssss:	00-07	2-byte status.	ssss identifies which UART register
// has changed value, and the new value is in P1.
// Note that the ssss values do not correspond to the
// 16554 register numbers given in 16554.H. Instead,
// see below for definitions of the ssss numbers
// used in this status message.
//
// 08-0B	3-byte status:					==== P1 ====	==== P2 ====
// 08	LSR_DATA:		New LSR		Errored byte
// 09	1-param responses	Response Code	Param
// 0A	OPEN_RSP:		InitialMsr	TxBufferSize
// 0B	available for expansion
//
// 0C-0D	4-byte status.	P1 = extended status code and P2,P3 = params
// Not currently implemented.
//
// 0E-0F	N-byte status:	P1 = num bytes after P1 (ie, TotalLen - 2)
// P2 = extended status, P3..Pn = parameters.
// Not currently implemented.
//
// SSSS values for 2-byte status messages (0-8)
//
pub const IOSP_STATUS_LSR: c_uint = 0x00	// P1 is new value of LSR register.;
// Bits defined in 16554.H. Edgeport
// returns this in order to report
// line status errors (overrun,
// parity, framing, break). This form
// is used when a errored receive data
// character was NOT present in the
// UART when the LSR error occurred
// (ie, when LSR bit 0 = 0).
pub const IOSP_STATUS_MSR: c_uint = 0x01	// P1 is new value of MSR register.;
// Bits defined in 16554.H. Edgeport
// returns this in order to report
// changes in modem status lines
// (CTS, DSR, RI, CD)
//
// 0x02	// Available for future expansion
// 0x03
// 0x04
// 0x05
// 0x06
// 0x07
//
// SSSS values for 3-byte status messages (8-A)
//
pub const IOSP_STATUS_LSR_DATA: c_uint = 0x08	// P1 is new value of LSR register (same as STATUS_LSR);
// P2 is errored character read from
// RxFIFO after LSR reported an error.
pub const IOSP_EXT_STATUS: c_uint = 0x09	// P1 is status/response code, param in P2.;
// Response Codes (P1 values) for 3-byte status messages

// control from remote device).

pub const IOSP_STATUS_OPEN_RSP: c_uint = 0x0A	// Reply to OPEN_PORT cmd.;
// P1 is Initial MSR value
// P2 is encoded TxBuffer Size:
// TxBufferSize = (P2 + 1) * 64
// 0x0B	// Available for future expansion

//
// SSSS values for 4-byte status messages
//
pub const IOSP_EXT4_STATUS: c_uint = 0x0C	// Extended status code in P1,;
// Params in P2, P3
// Currently unimplemented.
// 0x0D	// Currently unused, available.
//
// Macros to parse status messages
//

