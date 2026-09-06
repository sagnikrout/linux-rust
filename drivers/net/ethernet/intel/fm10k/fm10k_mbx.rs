//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/fm10k/fm10k_mbx.h
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
// Copyright(c) 2013 - 2018 Intel Corporation.
// forward declaration

// PF Mailbox Registers

// XOR provides means of switching from Tx to Rx FIFO

pub const FM10K_MBX_REQ: c_uint = 0x00000002;
pub const FM10K_MBX_ACK: c_uint = 0x00000004;
pub const FM10K_MBX_REQ_INTERRUPT: c_uint = 0x00000008;
pub const FM10K_MBX_ACK_INTERRUPT: c_uint = 0x00000010;
pub const FM10K_MBX_INTERRUPT_ENABLE: c_uint = 0x00000020;
pub const FM10K_MBX_INTERRUPT_DISABLE: c_uint = 0x00000040;
pub const FM10K_MBX_GLOBAL_REQ_INTERRUPT: c_uint = 0x00000200;
pub const FM10K_MBX_GLOBAL_ACK_INTERRUPT: c_uint = 0x00000400;

pub const FM10K_GMBX: c_uint = 0x18842;
// VF Mailbox Registers
pub const FM10K_VFMBX: c_uint = 0x00010;

pub const FM10K_VFMBMEM_LEN: c_int = 16;

// Delays/timeouts
pub const FM10K_MBX_DISCONNECT_TIMEOUT: c_int = 500;
pub const FM10K_MBX_POLL_DELAY: c_int = 19;
pub const FM10K_MBX_INT_DELAY: c_int = 20;
// PF/VF Mailbox state machine
//
// +----------+	    connect()	+----------+
// |  CLOSED  | --------------> |  CONNECT |
// +----------+			+----------+
// ^				  ^	 |
// | rcv:	      rcv:	  |	 | rcv:
// |  Connect	       Disconnect |	 |  Connect
// |  Disconnect     Error	  |	 |  Data
// |				  |	 |
// |				  |	 V
// +----------+   disconnect()	+----------+
// |DISCONNECT| <-------------- |   OPEN   |
// +----------+			+----------+
//
// The diagram above describes the PF/VF mailbox state machine.  There
// are four main states to this machine.
// Closed: This state represents a mailbox that is in a standby state
// with interrupts disabled.  In this state the mailbox should not
// read the mailbox or write any data.  The only means of exiting
// this state is for the system to make the connect() call for the
// mailbox, it will then transition to the connect state.
// Connect: In this state the mailbox is seeking a connection.  It will
// post a connect message with no specified destination and will
// wait for a reply from the other side of the mailbox.  This state
// is exited when either a connect with the local mailbox as the
// destination is received or when a data message is received with
// a valid sequence number.
// Open: In this state the mailbox is able to transfer data between the local
// entity and the remote.  It will fall back to connect in the event of
// receiving either an error message, or a disconnect message.  It will
// transition to disconnect on a call to disconnect();
// Disconnect: In this state the mailbox is attempting to gracefully terminate
// the connection.  It will do so at the first point where it knows
// that the remote endpoint is either done sending, or when the
// remote endpoint has fallen back into connect.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fm10k_mbx_state {
    FM10K_STATE_CLOSED,
    FM10K_STATE_CONNECT,
    FM10K_STATE_OPEN,
    FM10K_STATE_DISCONNECT,
}

// PF/VF Mailbox header format
// 3			  2		      1			  0
// 1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |        Size/Err_no/CRC        | Rsvd0 | Head  | Tail  | Type  |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//
// The layout above describes the format for the header used in the PF/VF
// mailbox.  The header is broken out into the following fields:
// Type: There are 4 supported message types
// 0x8: Data header - used to transport message data
// 0xC: Connect header - used to establish connection
// 0xD: Disconnect header - used to tear down a connection
// 0xE: Error header - used to address message exceptions
// Tail: Tail index for local FIFO
// Tail index actually consists of two parts.  The MSB of
// the head is a loop tracker, it is 0 on an even numbered
// loop through the FIFO, and 1 on the odd numbered loops.
// To get the actual mailbox offset based on the tail it
// is necessary to add bit 3 to bit 0 and clear bit 3.  This
// gives us a valid range of 0x1 - 0xE.
// Head: Head index for remote FIFO
// Head index follows the same format as the tail index.
// Rsvd0: Reserved 0 portion of the mailbox header
// CRC: Running CRC for all data since connect plus current message header
// Size: Maximum message size - Applies only to connect headers
// The maximum message size is provided during connect to avoid
// jamming the mailbox with messages that do not fit.
// Err_no: Error number - Applies only to error headers
// The error number provides an indication of the type of error
// experienced.
//
// macros for retrieving and setting header values

// offsets shared between all headers
pub const FM10K_MSG_TYPE_SHIFT: c_int = 0;
pub const FM10K_MSG_TYPE_SIZE: c_int = 4;
pub const FM10K_MSG_TAIL_SHIFT: c_int = 4;
pub const FM10K_MSG_TAIL_SIZE: c_int = 4;
pub const FM10K_MSG_HEAD_SHIFT: c_int = 8;
pub const FM10K_MSG_HEAD_SIZE: c_int = 4;
pub const FM10K_MSG_RSVD0_SHIFT: c_int = 12;
pub const FM10K_MSG_RSVD0_SIZE: c_int = 4;
// offsets for data/disconnect headers
pub const FM10K_MSG_CRC_SHIFT: c_int = 16;
pub const FM10K_MSG_CRC_SIZE: c_int = 16;
// offsets for connect headers
pub const FM10K_MSG_CONNECT_SIZE_SHIFT: c_int = 16;
pub const FM10K_MSG_CONNECT_SIZE_SIZE: c_int = 16;
// offsets for error headers
pub const FM10K_MSG_ERR_NO_SHIFT: c_int = 16;
pub const FM10K_MSG_ERR_NO_SIZE: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fm10k_msg_type {
    FM10K_MSG_DATA			= 0x8,
    FM10K_MSG_CONNECT		= 0xC,
    FM10K_MSG_DISCONNECT		= 0xD,
    FM10K_MSG_ERROR			= 0xE,
}

// HNI/SM Mailbox FIFO format
// 3                   2                   1                   0
// 1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0
// +-------+-----------------------+-------+-----------------------+
// | Error |      Remote Head      |Version|      Local Tail       |
// +-------+-----------------------+-------+-----------------------+
// |                                                               |
// .                        Local FIFO Data                        .
// .                                                               .
// +-------+-----------------------+-------+-----------------------+
//
// The layout above describes the format for the FIFOs used by the host
// network interface and the switch manager to communicate messages back
// and forth.  Both the HNI and the switch maintain one such FIFO.  The
// layout in memory has the switch manager FIFO followed immediately by
// the HNI FIFO.  For this reason I am using just the pointer to the
// HNI FIFO in the mailbox ops as the offset between the two is fixed.
//
// The header for the FIFO is broken out into the following fields:
// Local Tail:  Offset into FIFO region for next DWORD to write.
// Version:  Version info for mailbox, only values of 0/1 are supported.
// Remote Head:  Offset into remote FIFO to indicate how much we have read.
// Error: Error indication, values TBD.
//
// version number for switch manager mailboxes
pub const FM10K_SM_MBX_VERSION: c_int = 1;

// offsets shared between all SM FIFO headers
pub const FM10K_MSG_SM_TAIL_SHIFT: c_int = 0;
pub const FM10K_MSG_SM_TAIL_SIZE: c_int = 12;
pub const FM10K_MSG_SM_VER_SHIFT: c_int = 12;
pub const FM10K_MSG_SM_VER_SIZE: c_int = 4;
pub const FM10K_MSG_SM_HEAD_SHIFT: c_int = 16;
pub const FM10K_MSG_SM_HEAD_SIZE: c_int = 12;
pub const FM10K_MSG_SM_ERR_SHIFT: c_int = 28;
pub const FM10K_MSG_SM_ERR_SIZE: c_int = 4;
// All error messages returned by mailbox functions
// The value -511 is 0xFE01 in hex.  The idea is to order the errors
// from 0xFE01 - 0xFEFF so error codes are easily visible in the mailbox
// messages.  This also helps to avoid error number collisions as Linux
// doesn't appear to use error numbers 256 - 511.
//

pub const FM10K_MBX_CRC_SEED: c_uint = 0xFFFF;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fm10k_mbx_ops {
    pub ): *mut *mut *mut s32 (connect)(struct fm10k_hw , struct fm10k_mbx_info,
    pub ): *mut *mut *mut void (disconnect)(struct fm10k_hw , struct fm10k_mbx_info,
    pub ): *mut *mut bool (rx_ready)(struct fm10k_mbx_info,
    pub u16): *mut *mut *mut bool (tx_ready)(struct fm10k_mbx_info ,,
    pub ): *mut *mut bool (tx_complete)(struct fm10k_mbx_info,
    pub ): *const u32,
    pub ): *mut *mut *mut s32 (process)(struct fm10k_hw , struct fm10k_mbx_info,
    pub ): *const fm10k_msg_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fm10k_mbx_fifo {
    pub buffer: *mut u32,
    pub head: u16,
    pub tail: u16,
    pub size: u16,
}

// size of buffer to be stored in mailbox for FIFOs
pub const FM10K_MBX_TX_BUFFER_SIZE: c_int = 512;
pub const FM10K_MBX_RX_BUFFER_SIZE: c_int = 128;

// minimum and maximum message size in dwords

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fm10k_mbx_info {
// function pointers for mailbox operations
    pub ops: fm10k_mbx_ops,
    pub msg_data: *const fm10k_msg_data,
// message FIFOs
    pub rx: fm10k_mbx_fifo,
    pub tx: fm10k_mbx_fifo,
// delay for handling timeouts
    pub timeout: u32,
    pub udelay: u32,
// mailbox state info
    pub mbx_hdr: u32 mbx_reg, mbmem_reg, mbx_lock,,
    pub mbmem_len: u16 max_size,,
    pub pulled: u16 tail, tail_len,,
    pub pushed: u16 head, head_len,,
    pub remote: u16 local,,
    pub state: fm10k_mbx_state,
// result of last mailbox test
    pub test_result: i32,
// statistics
    pub tx_busy: u64,
    pub tx_dropped: u64,
    pub tx_messages: u64,
    pub tx_dwords: u64,
    pub tx_mbmem_pulled: u64,
    pub rx_messages: u64,
    pub rx_dwords: u64,
    pub rx_mbmem_pushed: u64,
    pub rx_parse_err: u64,
// Buffer to store messages
    pub buffer: [u32; FM10K_MBX_BUFFER_SIZE],
}
