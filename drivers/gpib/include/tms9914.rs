//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpib/include/tms9914.h
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
// copyright            : (C) 2002 by Frank Mori Hess
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tms9914_holdoff_mode {
    TMS9914_HOLDOFF_NONE,
    TMS9914_HOLDOFF_EOI,
    TMS9914_HOLDOFF_ALL,
}

// struct used to provide variables local to a tms9914 chip
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tms9914_priv {

    pub iobase: u32,

    pub mmiobase: *mut void __iomem,
    pub addresses: unsigned int offset; // offset between successive tms9914 io,
    pub dma_channel: c_uint,
// software copy of bits written to interrupt mask registers
    pub imr1_bits: u8 imr0_bits,,
// bits written to address mode register
    pub admr_bits: u8,
    pub A: u8 auxa_bits; // bits written to auxiliary register,
// used to keep track of board's state, bit definitions given below
    pub state: c_ulong,
    pub character: u8 eos; // eos,
    pub eos_flags: c_short,
    pub spoll_status: u8,
    pub holdoff_mode: tms9914_holdoff_mode,
    pub ppoll_line: c_uint,
    pub talker_state: talker_function_state,
    pub listener_state: listener_function_state,
    pub 1: unsigned ppoll_sense :,
    pub 1: unsigned ppoll_enable :,
    pub 1: unsigned ppoll_configure_state :,
    pub 1: unsigned primary_listen_addressed :,
    pub 1: unsigned primary_talk_addressed :,
    pub 1: unsigned holdoff_on_end :,
    pub 1: unsigned holdoff_on_all :,
    pub 1: unsigned holdoff_active :,
// wrappers for outb, inb, readb, or writeb
    pub register_number): *mut *mut *mut u8 (read_byte)(struct tms9914_priv priv, unsigned int,
}

// slightly shorter way to access read_byte and write_byte
// struct tms9914_priv.state bit numbers
// interface functions
extern "C" {
    pub fn tms9914_take_control(board: *mut gpib_board, priv: *mut tms9914_priv, syncronous: c_int) -> c_int;
}
//
// alternate version of tms9914_take_control which works around buggy tcs
// implementation.
//
extern "C" {
    pub fn tms9914_go_to_standby(board: *mut gpib_board, priv: *mut tms9914_priv) -> c_int;
}
extern "C" {
    pub fn tms9914_interface_clear(board: *mut gpib_board, priv: *mut tms9914_priv, assert: c_int);
}
extern "C" {
    pub fn tms9914_remote_enable(board: *mut gpib_board, priv: *mut tms9914_priv, enable: c_int);
}
extern "C" {
    pub fn tms9914_disable_eos(board: *mut gpib_board, priv: *mut tms9914_priv);
}
extern "C" {
    pub fn tms9914_parallel_poll(board: *mut gpib_board, priv: *mut tms9914_priv, result: *mut u8) -> c_int;
}
extern "C" {
    pub fn tms9914_serial_poll_status(board: *mut gpib_board, priv: *mut tms9914_priv) -> u8;
}
extern "C" {
    pub fn tms9914_line_status(board: *const gpib_board, priv: *mut tms9914_priv) -> c_int;
}
extern "C" {
    pub fn tms9914_return_to_local(board: *const gpib_board, priv: *mut tms9914_priv);
}
// utility functions
extern "C" {
    pub fn tms9914_board_reset(priv: *mut tms9914_priv);
}
extern "C" {
    pub fn tms9914_online(board: *mut gpib_board, priv: *mut tms9914_priv);
}
extern "C" {
    pub fn tms9914_release_holdoff(priv: *mut tms9914_priv);
}
extern "C" {
    pub fn tms9914_set_holdoff_mode(priv: *mut tms9914_priv, mode: tms9914_holdoff_mode);
}
// wrappers for io functions
extern "C" {
    pub fn tms9914_ioport_read_byte(priv: *mut tms9914_priv, register_num: c_uint) -> u8;
}
extern "C" {
    pub fn tms9914_ioport_write_byte(priv: *mut tms9914_priv, data: u8, register_num: c_uint);
}
extern "C" {
    pub fn tms9914_iomem_read_byte(priv: *mut tms9914_priv, register_num: c_uint) -> u8;
}
extern "C" {
    pub fn tms9914_iomem_write_byte(priv: *mut tms9914_priv, data: u8, register_num: c_uint);
}
// interrupt service routine
extern "C" {
    pub fn tms9914_interrupt(board: *mut gpib_board, priv: *mut tms9914_priv) -> irqreturn_t;
}
// tms9914 has 8 registers
//
// tms9914 register numbers (might need to be multiplied by
// a board-dependent offset to get actually io address offset)
//
// write registers
// read registers
// bit definitions common to tms9914 compatible registers
// ISR0   - Register bits
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum isr0_bits {
    HR_MAC = (1 << 0),   /* My Address Change           */
    HR_RLC = (1 << 1),   /* Remote/Local change         */
    HR_SPAS = (1 << 2),   /* Serial Poll active State    */
    HR_END = (1 << 3),   /* END (EOI or EOS)            */
    HR_BO = (1 << 4),   /* Byte Out                    */
    HR_BI = (1 << 5),   /* Byte In                     */
}

// IMR0   - Register bits
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum imr0_bits {
    HR_MACIE = (1 << 0),   /*        */
    HR_RLCIE = (1 << 1),   /*        */
    HR_SPASIE = (1 << 2),   /*        */
    HR_ENDIE = (1 << 3),   /*        */
    HR_BOIE = (1 << 4),   /*        */
    HR_BIIE = (1 << 5),   /*        */
}

// ISR1   - Register bits
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum isr1_bits {
    HR_IFC = (1 << 0),   /* IFC asserted                */
    HR_SRQ = (1 << 1),   /* SRQ asserted                */
    HR_MA = (1 << 2),    /* My Address                  */
    HR_DCAS = (1 << 3),  /* Device Clear active State   */
    HR_APT = (1 << 4),   /* Address pass Through        */
    HR_UNC = (1 << 5),   /* Unrecognized Command        */
    HR_ERR = (1 << 6),   /* Data Transmission Error     */
    HR_GET = (1 << 7),   /* Group execute Trigger       */
}

// IMR1   - Register bits
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum imr1_bits {
    HR_IFCIE = (1 << 0),   /*        */
    HR_SRQIE = (1 << 1),   /*        */
    HR_MAIE = (1 << 2),    /*        */
    HR_DCASIE = (1 << 3),  /*        */
    HR_APTIE = (1 << 4),   /*        */
    HR_UNCIE = (1 << 5),   /*        */
    HR_ERRIE = (1 << 6),   /*        */
    HR_GETIE = (1 << 7),   /*        */
}

// ADSR   - Register bits
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum adsr_bits {
    HR_ULPA = (1 << 0),   /* Store last address LSB       */
    HR_TA = (1 << 1),     /* Talker Adressed              */
    HR_LA = (1 << 2),     /* Listener adressed            */
    HR_TPAS = (1 << 3),   /* talker primary address state */
    HR_LPAS = (1 << 4),   /* listener    "                */
    HR_ATN = (1 << 5),    /* ATN active                   */
    HR_LLO = (1 << 6),    /* LLO active                   */
    HR_REM = (1 << 7),    /* REM active                   */
}

// ADR   - Register bits
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum adr_bits {
    ADDRESS_MASK = 0x1f,	/* mask to specify lower 5 bits for ADR */
    HR_DAT = (1 << 5),      /* disable talker */
    HR_DAL = (1 << 6),      /* disable listener */
    HR_EDPA = (1 << 7),     /* enable dual primary addressing */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bus_status_bits {
    BSR_REN_BIT = 0x1,
    BSR_IFC_BIT = 0x2,
    BSR_SRQ_BIT = 0x4,
    BSR_EOI_BIT = 0x8,
    BSR_NRFD_BIT = 0x10,
    BSR_NDAC_BIT = 0x20,
    BSR_DAV_BIT = 0x40,
    BSR_ATN_BIT = 0x80,
}

// ---------------------------------------------------------
// TMS 9914 Auxiliary Commands
// ---------------------------------------------------------
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum aux_cmd_bits {
    AUX_CS = 0x80,			/* set bit instead of clearing it, used with commands marked 'd' below */
    AUX_CHIP_RESET = 0x0,		/* d Chip reset                   */
    AUX_INVAL = 0x1,		/* release dac holdoff, invalid command byte */
    AUX_VAL = (AUX_INVAL | AUX_CS),	/* release dac holdoff, valid command byte   */
    AUX_RHDF = 0x2,			/* X Release RFD holdoff          */
    AUX_HLDA = 0x3,			/* d holdoff on all data          */
    AUX_HLDE = 0x4,			/* d holdoff on EOI only          */
    AUX_NBAF = 0x5,			/* X Set new byte available false */
    AUX_FGET = 0x6,			/* d force GET                    */
    AUX_RTL = 0x7,			/* d return to local              */
    AUX_SEOI = 0x8,			/* X send EOI with next byte      */
    AUX_LON = 0x9,			/* d Listen only                  */
    AUX_TON = 0xa,			/* d Talk only                    */
    AUX_GTS = 0xb,			/* X goto standby                 */
    AUX_TCA = 0xc,			/* X take control asynchronously  */
    AUX_TCS = 0xd,			/* X take    "     synchronously  */
    AUX_RPP = 0xe,			/* d Request parallel poll        */
    AUX_SIC = 0xf,			/* d send interface clear         */
    AUX_SRE = 0x10,			/* d send remote enable           */
    AUX_RQC = 0x11,			/* X request control              */
    AUX_RLC = 0x12,			/* X release control              */
    AUX_DAI = 0x13,			/* d disable all interrupts       */
    AUX_PTS = 0x14,			/* X pass through next secondary  */
    AUX_STDL = 0x15,		/* d short T1 delay		  */
    AUX_SHDW = 0x16,		/* d shadow handshake             */
    AUX_VSTDL = 0x17,		/* d very short T1 delay (smj9914 extension)   */
    AUX_RSV2 = 0x18,		/* d request service bit 2 (smj9914 extension) */
}
