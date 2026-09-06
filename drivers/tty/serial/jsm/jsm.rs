//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/tty/serial/jsm/jsm.h
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
// Copyright 2003 Digi International (www.digi.com)
//
// Copyright (C) 2004 IBM Corporation. All rights reserved.
//
// Contact Information:
// Scott H Kilau <Scott_Kilau@digi.com>
// Wendy Xiong   <wendyx@us.ibm.com>
//

//
// Debugging levels can be set using debug insmod variable
// They can also be compiled out completely.
//

pub const MAXLINES: c_int = 256;
pub const MAXPORTS: c_int = 8;
pub const MAX_STOPS_SENT: c_int = 5;
// Board ids
pub const PCI_DEVICE_ID_CLASSIC_4: c_uint = 0x0028;
pub const PCI_DEVICE_ID_CLASSIC_8: c_uint = 0x0029;
pub const PCI_DEVICE_ID_CLASSIC_4_422: c_uint = 0x00D0;
pub const PCI_DEVICE_ID_CLASSIC_8_422: c_uint = 0x00D1;
pub const PCI_DEVICE_ID_NEO_4: c_uint = 0x00B0;
pub const PCI_DEVICE_ID_NEO_1_422: c_uint = 0x00CC;
pub const PCI_DEVICE_ID_NEO_1_422_485: c_uint = 0x00CD;
pub const PCI_DEVICE_ID_NEO_2_422_485: c_uint = 0x00CE;
pub const PCIE_DEVICE_ID_NEO_8: c_uint = 0x00F0;
pub const PCIE_DEVICE_ID_NEO_4: c_uint = 0x00F1;
pub const PCIE_DEVICE_ID_NEO_4RJ45: c_uint = 0x00F2;
pub const PCIE_DEVICE_ID_NEO_8RJ45: c_uint = 0x00F3;
// Board type definitions
pub const T_NEO: c_int = 0000;
pub const T_CLASSIC: c_int = 0001;
pub const T_PCIBUS: c_int = 0400;
// Board State Definitions
pub const BD_RUNNING: c_uint = 0x0;
pub const BD_REASON: c_uint = 0x7f;
pub const BD_NOTFOUND: c_uint = 0x1;
pub const BD_NOIOPORT: c_uint = 0x2;
pub const BD_NOMEM: c_uint = 0x3;
pub const BD_NOBIOS: c_uint = 0x4;
pub const BD_NOFEP: c_uint = 0x5;
pub const BD_FAILED: c_uint = 0x6;
pub const BD_ALLOCATED: c_uint = 0x7;
pub const BD_TRIBOOT: c_uint = 0x8;
pub const BD_BADKME: c_uint = 0x80;
// 4 extra for alignment play space

//
// Per board operations structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct board_ops {
    pub intr: irq_handler_t,
    pub ch): *mut *mut void (uart_init)(struct jsm_channel,
    pub ch): *mut *mut void (uart_off)(struct jsm_channel,
    pub ch): *mut *mut void (param)(struct jsm_channel,
    pub ch): *mut *mut void (assert_modem_signals)(struct jsm_channel,
    pub ch): *mut *mut void (flush_uart_write)(struct jsm_channel,
    pub ch): *mut *mut void (flush_uart_read)(struct jsm_channel,
    pub ch): *mut *mut void (disable_receiver)(struct jsm_channel,
    pub ch): *mut *mut void (enable_receiver)(struct jsm_channel,
    pub ch): *mut *mut void (send_break)(struct jsm_channel,
    pub ch): *mut *mut void (clear_break)(struct jsm_channel,
    pub ch): *mut *mut void (send_start_character)(struct jsm_channel,
    pub ch): *mut *mut void (send_stop_character)(struct jsm_channel,
    pub ch): *mut *mut void (copy_data_from_queue_to_uart)(struct jsm_channel,
}

//
// Per-board information
//
// the interrupt routine from each other.
//
// Device flag definitions for ch_flags.
//
pub const CH_PRON: c_uint = 0x0001		/* Printer on string		*/;
pub const CH_STOP: c_uint = 0x0002		/* Output is stopped		*/;
pub const CH_STOPI: c_uint = 0x0004		/* Input is stopped		*/;
pub const CH_CD: c_uint = 0x0008		/* Carrier is present		*/;
pub const CH_FCAR: c_uint = 0x0010		/* Carrier forced on		*/;
pub const CH_HANGUP: c_uint = 0x0020		/* Hangup received		*/;
pub const CH_RECEIVER_OFF: c_uint = 0x0040		/* Receiver is off		*/;
pub const CH_OPENING: c_uint = 0x0080		/* Port in fragile open state	*/;
pub const CH_CLOSING: c_uint = 0x0100		/* Port in fragile close state	*/;
pub const CH_FIFO_ENABLED: c_uint = 0x0200		/* Port has FIFOs enabled	*/;
pub const CH_TX_FIFO_EMPTY: c_uint = 0x0400		/* TX Fifo is completely empty	*/;
pub const CH_TX_FIFO_LWM: c_uint = 0x0800		/* TX Fifo is below Low Water	*/;
pub const CH_BREAK_SENDING: c_uint = 0x1000		/* Break is being sent		*/;
pub const CH_LOOPBACK: c_uint = 0x2000		/* Channel is in lookback mode	*/;
pub const CH_BAUD0: c_uint = 0x08000		/* Used for checking B0 transitions */;
// Our Read/Error queue sizes
pub const RQUEUEMASK: c_uint = 0x1FFF		/* 8 K - 1 */;
pub const EQUEUEMASK: c_uint = 0x1FFF		/* 8 K - 1 */;

//
// Channel information structure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jsm_channel {
    pub uart_port: uart_port,
    pub /: *mut *mut *mut jsm_board ch_bd; / Board structure pointer,
    pub /: *mut *mut spinlock_t ch_lock; / provide for serialization,
    pub ch_flags_wait: wait_queue_head_t,
    pub /: *mut *mut u32 ch_portnum; / Port number, 0 offset.,
    pub /: *mut *mut u32 ch_open_count; / open count,
    pub /: *mut *mut u32 ch_flags; / Channel flags,
    pub /: *mut *mut u64 ch_close_delay; / How long we should drop RTS/DTR for,
    pub /: *mut *mut tcflag_t ch_c_iflag; / channel iflags,
    pub /: *mut *mut tcflag_t ch_c_cflag; / channel cflags,
    pub /: *mut *mut tcflag_t ch_c_oflag; / channel oflags,
    pub /: *mut *mut tcflag_t ch_c_lflag; / channel lflags,
    pub /: *mut *mut u8 ch_stopc; / Stop character,
    pub /: *mut *mut u8 ch_startc; / Start character,
    pub /: *mut *mut u8 ch_mostat; / FEP output modem status,
    pub /: *mut *mut u8 ch_mistat; / FEP input modem status,
// Pointers to the "mapped" UART structs
    pub /: *mut *mut *mut neo_uart___iomem ch_neo_uart; / NEO card,
    pub /: *mut *mut *mut cls_uart___iomem ch_cls_uart; / Classic card,
    pub /: *mut *mut u8 ch_cached_lsr; / Cached value of the LSR register,
    pub /: *mut *mut *mut u8 ch_rqueue; / Our read queue buffer - malloc'ed,
    pub /: *mut *mut u16 ch_r_head; / Head location of the read queue,
    pub /: *mut *mut u16 ch_r_tail; / Tail location of the read queue,
    pub /: *mut *mut *mut u8 ch_equeue; / Our error queue buffer - malloc'ed,
    pub /: *mut *mut u16 ch_e_head; / Head location of the error queue,
    pub /: *mut *mut u16 ch_e_tail; / Tail location of the error queue,
    pub /: *mut *mut u64 ch_rxcount; / total of data received so far,
    pub /: *mut *mut u64 ch_txcount; / total of data transmitted so far,
    pub /: *mut *mut u8 ch_r_tlevel; / Receive Trigger level,
    pub /: *mut *mut u8 ch_t_tlevel; / Transmit Trigger level,
    pub /: *mut *mut u8 ch_r_watermark; / Receive Watermark,
    pub character: *mut *mut u32 ch_stops_sent; / How many times I have sent a stop,
// to try to stop the other guy sending.
//
    pub /: *mut *mut u64 ch_err_parity; / Count of parity errors on channel,
    pub /: *mut *mut u64 ch_err_frame; / Count of framing errors on channel,
    pub /: *mut *mut u64 ch_err_break; / Count of breaks on channel,
    pub /: *mut *mut u64 ch_err_overrun; / Count of overruns on channel,
    pub /: *mut *mut u64 ch_xon_sends; / Count of xons transmitted,
    pub /: *mut *mut u64 ch_xoff_sends; / Count of xoffs transmitted,
}

//
// Per channel/port Classic UART structures
//
// Base Structure Entries Usage Meanings to Host
//
// W = read write		R = read only
// U = Unused.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cls_uart_struct {
    pub /: *mut *mut u8 txrx; / WR RHR/THR - Holding Reg,
    pub /: *mut *mut u8 ier; / WR IER - Interrupt Enable Reg,
    pub Reg*/: *mut *mut u8 isr_fcr; / WR ISR/FCR - Interrupt Status Reg/Fifo Control,
    pub /: *mut *mut u8 lcr; / WR LCR - Line Control Reg,
    pub /: *mut *mut u8 mcr; / WR MCR - Modem Control Reg,
    pub /: *mut *mut u8 lsr; / WR LSR - Line Status Reg,
    pub /: *mut *mut u8 msr; / WR MSR - Modem Status Reg,
    pub /: *mut *mut u8 spr; / WR SPR - Scratch Pad Reg,
}

// Where to read the interrupt register (8bits)
pub const UART_CLASSIC_POLL_ADDR_OFFSET: c_uint = 0x40;
pub const UART_EXAR654_ENHANCED_REGISTER_SET: c_uint = 0xBF;
pub const UART_16654_FCR_TXTRIGGER_8: c_uint = 0x0;
pub const UART_16654_FCR_TXTRIGGER_16: c_uint = 0x10;
pub const UART_16654_FCR_TXTRIGGER_32: c_uint = 0x20;
pub const UART_16654_FCR_TXTRIGGER_56: c_uint = 0x30;
pub const UART_16654_FCR_RXTRIGGER_8: c_uint = 0x0;
pub const UART_16654_FCR_RXTRIGGER_16: c_uint = 0x40;
pub const UART_16654_FCR_RXTRIGGER_56: c_uint = 0x80;
pub const UART_16654_FCR_RXTRIGGER_60: c_uint = 0xC0;
pub const UART_IIR_CTSRTS: c_uint = 0x20	/* Received CTS/RTS change of state */;
pub const UART_IIR_RDI_TIMEOUT: c_uint = 0x0C    /* Receiver data TIMEOUT */;
//
// These are the EXTENDED definitions for the Exar 654's Interrupt
// Enable Register.
//
pub const UART_EXAR654_EFR_ECB: c_uint = 0x10    /* Enhanced control bit */;
pub const UART_EXAR654_EFR_IXON: c_uint = 0x2     /* Receiver compares Xon1/Xoff1 */;
pub const UART_EXAR654_EFR_IXOFF: c_uint = 0x8     /* Transmit Xon1/Xoff1 */;
pub const UART_EXAR654_EFR_RTSDTR: c_uint = 0x40    /* Auto RTS/DTR Flow Control Enable */;
pub const UART_EXAR654_EFR_CTSDSR: c_uint = 0x80    /* Auto CTS/DSR Flow COntrol Enable */;
pub const UART_EXAR654_XOFF_DETECT: c_uint = 0x1     /* Indicates whether chip saw an incoming XOFF char  */;
pub const UART_EXAR654_XON_DETECT: c_uint = 0x2     /* Indicates whether chip saw an incoming XON char */;
pub const UART_EXAR654_IER_XOFF: c_uint = 0x20    /* Xoff Interrupt Enable */;
pub const UART_EXAR654_IER_RTSDTR: c_uint = 0x40    /* Output Interrupt Enable */;
pub const UART_EXAR654_IER_CTSDSR: c_uint = 0x80    /* Input Interrupt Enable */;
//
// Per channel/port NEO UART structure
//
// Base Structure Entries Usage Meanings to Host
//
// W = read write		R = read only
// U = Unused.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct neo_uart_struct {
    pub /: *mut *mut u8 txrx; / WR RHR/THR - Holding Reg,
    pub /: *mut *mut u8 ier; / WR IER - Interrupt Enable Reg,
    pub /: *mut *mut u8 isr_fcr; / WR ISR/FCR - Interrupt Status Reg/Fifo Control Reg,
    pub /: *mut *mut u8 lcr; / WR LCR - Line Control Reg,
    pub /: *mut *mut u8 mcr; / WR MCR - Modem Control Reg,
    pub /: *mut *mut u8 lsr; / WR LSR - Line Status Reg,
    pub /: *mut *mut u8 msr; / WR MSR - Modem Status Reg,
    pub /: *mut *mut u8 spr; / WR SPR - Scratch Pad Reg,
    pub /: *mut *mut u8 fctr; / WR FCTR - Feature Control Reg,
    pub /: *mut *mut u8 efr; / WR EFR - Enhanced Function Reg,
    pub /: *mut *mut u8 tfifo; / WR TXCNT/TXTRG - Transmit FIFO Reg,
    pub /: *mut *mut u8 rfifo; / WR RXCNT/RXTRG - Receive FIFO Reg,
    pub /: *mut *mut u8 xoffchar1; / WR XOFF 1 - XOff Character 1 Reg,
    pub /: *mut *mut u8 xoffchar2; / WR XOFF 2 - XOff Character 2 Reg,
    pub /: *mut *mut u8 xonchar1; / WR XON 1 - Xon Character 1 Reg,
    pub /: *mut *mut u8 xonchar2; / WR XON 2 - XOn Character 2 Reg,
    pub /: *mut *mut u8 reserved1[0x2ff - 0x200]; / U Reserved by Exar,
    pub /: *mut *mut u8 txrxburst[64]; / RW 64 bytes of RX/TX FIFO Data,
    pub /: *mut *mut u8 reserved2[0x37f - 0x340]; / U Reserved by Exar,
    pub /: *mut *mut u8 rxburst_with_errors[64]; / R 64 bytes of RX FIFO Data + LSR,
}

// Where to read the extended interrupt register (32bits instead of 8bits)
pub const UART_17158_POLL_ADDR_OFFSET: c_uint = 0x80;
//
// These are the redefinitions for the FCTR on the XR17C158, since
// Exar made them different than their earlier design. (XR16C854)
//
// These are only applicable when table D is selected
pub const UART_17158_FCTR_RTS_NODELAY: c_uint = 0x00;
pub const UART_17158_FCTR_RTS_4DELAY: c_uint = 0x01;
pub const UART_17158_FCTR_RTS_6DELAY: c_uint = 0x02;
pub const UART_17158_FCTR_RTS_8DELAY: c_uint = 0x03;
pub const UART_17158_FCTR_RTS_12DELAY: c_uint = 0x12;
pub const UART_17158_FCTR_RTS_16DELAY: c_uint = 0x05;
pub const UART_17158_FCTR_RTS_20DELAY: c_uint = 0x13;
pub const UART_17158_FCTR_RTS_24DELAY: c_uint = 0x06;
pub const UART_17158_FCTR_RTS_28DELAY: c_uint = 0x14;
pub const UART_17158_FCTR_RTS_32DELAY: c_uint = 0x07;
pub const UART_17158_FCTR_RTS_36DELAY: c_uint = 0x16;
pub const UART_17158_FCTR_RTS_40DELAY: c_uint = 0x08;
pub const UART_17158_FCTR_RTS_44DELAY: c_uint = 0x09;
pub const UART_17158_FCTR_RTS_48DELAY: c_uint = 0x10;
pub const UART_17158_FCTR_RTS_52DELAY: c_uint = 0x11;
pub const UART_17158_FCTR_RTS_IRDA: c_uint = 0x10;
pub const UART_17158_FCTR_RS485: c_uint = 0x20;
pub const UART_17158_FCTR_TRGA: c_uint = 0x00;
pub const UART_17158_FCTR_TRGB: c_uint = 0x40;
pub const UART_17158_FCTR_TRGC: c_uint = 0x80;
pub const UART_17158_FCTR_TRGD: c_uint = 0xC0;
// 17158 trigger table selects..
pub const UART_17158_FCTR_BIT6: c_uint = 0x40;
pub const UART_17158_FCTR_BIT7: c_uint = 0x80;
// 17158 TX/RX memmapped buffer offsets
pub const UART_17158_RX_FIFOSIZE: c_int = 64;
pub const UART_17158_TX_FIFOSIZE: c_int = 64;
// 17158 Extended IIR's
pub const UART_17158_IIR_RDI_TIMEOUT: c_uint = 0x0C	/* Receiver data TIMEOUT */;
pub const UART_17158_IIR_XONXOFF: c_uint = 0x10	/* Received an XON/XOFF char */;
pub const UART_17158_IIR_HWFLOW_STATE_CHANGE: c_uint = 0x20	/* CTS/DSR or RTS/DTR state change */;
pub const UART_17158_IIR_FIFO_ENABLED: c_uint = 0xC0	/* 16550 FIFOs are Enabled */;
//
// These are the extended interrupts that get sent
// back to us from the UART's 32bit interrupt register
//
pub const UART_17158_RX_LINE_STATUS: c_uint = 0x1	/* RX Ready */;
pub const UART_17158_RXRDY_TIMEOUT: c_uint = 0x2	/* RX Ready Timeout */;
pub const UART_17158_TXRDY: c_uint = 0x3	/* TX Ready */;
pub const UART_17158_MSR: c_uint = 0x4	/* Modem State Change */;
pub const UART_17158_TX_AND_FIFO_CLR: c_uint = 0x40	/* Transmitter Holding Reg Empty */;
pub const UART_17158_RX_FIFO_DATA_ERROR: c_uint = 0x80	/* UART detected an RX FIFO Data error */;
//
// These are the EXTENDED definitions for the 17C158's Interrupt
// Enable Register.
//
pub const UART_17158_EFR_ECB: c_uint = 0x10	/* Enhanced control bit */;
pub const UART_17158_EFR_IXON: c_uint = 0x2	/* Receiver compares Xon1/Xoff1 */;
pub const UART_17158_EFR_IXOFF: c_uint = 0x8	/* Transmit Xon1/Xoff1 */;
pub const UART_17158_EFR_RTSDTR: c_uint = 0x40	/* Auto RTS/DTR Flow Control Enable */;
pub const UART_17158_EFR_CTSDSR: c_uint = 0x80	/* Auto CTS/DSR Flow COntrol Enable */;
pub const UART_17158_XOFF_DETECT: c_uint = 0x1	/* Indicates whether chip saw an incoming XOFF char */;
pub const UART_17158_XON_DETECT: c_uint = 0x2	/* Indicates whether chip saw an incoming XON char */;
pub const UART_17158_IER_RSVD1: c_uint = 0x10	/* Reserved by Exar */;
pub const UART_17158_IER_XOFF: c_uint = 0x20	/* Xoff Interrupt Enable */;
pub const UART_17158_IER_RTSDTR: c_uint = 0x40	/* Output Interrupt Enable */;
pub const UART_17158_IER_CTSDSR: c_uint = 0x80	/* Input Interrupt Enable */;

//
// Our Global Variables.
//
// Prototypes for non-static functions used in more than one module
//
extern "C" {
    pub fn jsm_tty_init(: *mut jsm_board) -> c_int;
}
extern "C" {
    pub fn jsm_uart_port_init(: *mut jsm_board) -> c_int;
}
extern "C" {
    pub fn jsm_remove_uart_port(: *mut jsm_board) -> c_int;
}
extern "C" {
    pub fn jsm_input(ch: *mut jsm_channel);
}
extern "C" {
    pub fn jsm_check_queue_flow_control(ch: *mut jsm_channel);
}
