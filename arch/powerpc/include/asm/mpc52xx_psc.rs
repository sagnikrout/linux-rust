//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/mpc52xx_psc.h
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


//
// include/asm-ppc/mpc52xx_psc.h
//
// Definitions of consts/structs to drive the Freescale MPC52xx OnChip
// PSCs. Theses are shared between multiple drivers since a PSC can be
// UART, AC97, IR, I2S, ... So this header is in asm-ppc.
//
// Maintainer : Sylvain Munaut <tnt@246tNt.com>
//
// Based/Extracted from some header of the 2.4 originally written by
// Dale Farnsworth <dfarnsworth@mvista.com>
//
// Copyright (C) 2004 Sylvain Munaut <tnt@246tNt.com>
// Copyright (C) 2003 MontaVista, Software, Inc.
//
// This file is licensed under the terms of the GNU General Public License
// version 2. This program is licensed "as is" without any warranty of any
// kind, whether express or implied.
//

// Macro flag: #define __ASM_MPC52xx_PSC_H__

// Max number of PSCs

pub const MPC52xx_PSC_MAXNUM: c_int = 12;

pub const MPC52xx_PSC_MAXNUM: c_int = 6;

// Programmable Serial Controller (PSC) status register bits
pub const MPC52xx_PSC_SR_UNEX_RX: c_uint = 0x0001;
pub const MPC52xx_PSC_SR_DATA_VAL: c_uint = 0x0002;
pub const MPC52xx_PSC_SR_DATA_OVR: c_uint = 0x0004;
pub const MPC52xx_PSC_SR_CMDSEND: c_uint = 0x0008;
pub const MPC52xx_PSC_SR_CDE: c_uint = 0x0080;
pub const MPC52xx_PSC_SR_RXRDY: c_uint = 0x0100;
pub const MPC52xx_PSC_SR_RXFULL: c_uint = 0x0200;
pub const MPC52xx_PSC_SR_TXRDY: c_uint = 0x0400;
pub const MPC52xx_PSC_SR_TXEMP: c_uint = 0x0800;
pub const MPC52xx_PSC_SR_OE: c_uint = 0x1000;
pub const MPC52xx_PSC_SR_PE: c_uint = 0x2000;
pub const MPC52xx_PSC_SR_FE: c_uint = 0x4000;
pub const MPC52xx_PSC_SR_RB: c_uint = 0x8000;
// PSC Command values
pub const MPC52xx_PSC_RX_ENABLE: c_uint = 0x0001;
pub const MPC52xx_PSC_RX_DISABLE: c_uint = 0x0002;
pub const MPC52xx_PSC_TX_ENABLE: c_uint = 0x0004;
pub const MPC52xx_PSC_TX_DISABLE: c_uint = 0x0008;
pub const MPC52xx_PSC_SEL_MODE_REG_1: c_uint = 0x0010;
pub const MPC52xx_PSC_RST_RX: c_uint = 0x0020;
pub const MPC52xx_PSC_RST_TX: c_uint = 0x0030;
pub const MPC52xx_PSC_RST_ERR_STAT: c_uint = 0x0040;
pub const MPC52xx_PSC_RST_BRK_CHG_INT: c_uint = 0x0050;
pub const MPC52xx_PSC_START_BRK: c_uint = 0x0060;
pub const MPC52xx_PSC_STOP_BRK: c_uint = 0x0070;
// PSC TxRx FIFO status bits
pub const MPC52xx_PSC_RXTX_FIFO_ERR: c_uint = 0x0040;
pub const MPC52xx_PSC_RXTX_FIFO_UF: c_uint = 0x0020;
pub const MPC52xx_PSC_RXTX_FIFO_OF: c_uint = 0x0010;
pub const MPC52xx_PSC_RXTX_FIFO_FR: c_uint = 0x0008;
pub const MPC52xx_PSC_RXTX_FIFO_FULL: c_uint = 0x0004;
pub const MPC52xx_PSC_RXTX_FIFO_ALARM: c_uint = 0x0002;
pub const MPC52xx_PSC_RXTX_FIFO_EMPTY: c_uint = 0x0001;
// PSC interrupt status/mask bits
pub const MPC52xx_PSC_IMR_UNEX_RX_SLOT: c_uint = 0x0001;
pub const MPC52xx_PSC_IMR_DATA_VALID: c_uint = 0x0002;
pub const MPC52xx_PSC_IMR_DATA_OVR: c_uint = 0x0004;
pub const MPC52xx_PSC_IMR_CMD_SEND: c_uint = 0x0008;
pub const MPC52xx_PSC_IMR_ERROR: c_uint = 0x0040;
pub const MPC52xx_PSC_IMR_DEOF: c_uint = 0x0080;
pub const MPC52xx_PSC_IMR_TXRDY: c_uint = 0x0100;
pub const MPC52xx_PSC_IMR_RXRDY: c_uint = 0x0200;
pub const MPC52xx_PSC_IMR_DB: c_uint = 0x0400;
pub const MPC52xx_PSC_IMR_TXEMP: c_uint = 0x0800;
pub const MPC52xx_PSC_IMR_ORERR: c_uint = 0x1000;
pub const MPC52xx_PSC_IMR_IPC: c_uint = 0x8000;
// PSC input port change bits
pub const MPC52xx_PSC_CTS: c_uint = 0x01;
pub const MPC52xx_PSC_DCD: c_uint = 0x02;
pub const MPC52xx_PSC_D_CTS: c_uint = 0x10;
pub const MPC52xx_PSC_D_DCD: c_uint = 0x20;
// PSC acr bits
pub const MPC52xx_PSC_IEC_CTS: c_uint = 0x01;
pub const MPC52xx_PSC_IEC_DCD: c_uint = 0x02;
// PSC output port bits
pub const MPC52xx_PSC_OP_RTS: c_uint = 0x01;
pub const MPC52xx_PSC_OP_RES: c_uint = 0x02;
// PSC mode fields
pub const MPC52xx_PSC_MODE_5_BITS: c_uint = 0x00;
pub const MPC52xx_PSC_MODE_6_BITS: c_uint = 0x01;
pub const MPC52xx_PSC_MODE_7_BITS: c_uint = 0x02;
pub const MPC52xx_PSC_MODE_8_BITS: c_uint = 0x03;
pub const MPC52xx_PSC_MODE_BITS_MASK: c_uint = 0x03;
pub const MPC52xx_PSC_MODE_PAREVEN: c_uint = 0x00;
pub const MPC52xx_PSC_MODE_PARODD: c_uint = 0x04;
pub const MPC52xx_PSC_MODE_PARFORCE: c_uint = 0x08;
pub const MPC52xx_PSC_MODE_PARNONE: c_uint = 0x10;
pub const MPC52xx_PSC_MODE_ERR: c_uint = 0x20;
pub const MPC52xx_PSC_MODE_FFULL: c_uint = 0x40;
pub const MPC52xx_PSC_MODE_RXRTS: c_uint = 0x80;
pub const MPC52xx_PSC_MODE_ONE_STOP_5_BITS: c_uint = 0x00;
pub const MPC52xx_PSC_MODE_ONE_STOP: c_uint = 0x07;
pub const MPC52xx_PSC_MODE_TWO_STOP: c_uint = 0x0f;
pub const MPC52xx_PSC_MODE_TXCTS: c_uint = 0x10;
pub const MPC52xx_PSC_RFNUM_MASK: c_uint = 0x01ff;

// Structure of the hardware registers
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpc52xx_psc {
    pub /: *mut *mut u8 mode; / PSC + 0x00,
    pub mr2: u8,
}

// BitClkDiv field of CCR is byte swapped in
// the hardware for mpc5200/b compatibility
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpc52xx_psc_fifo {
    pub /: *mut *mut u16 rfnum; / PSC + 0x58,
    pub reserved18: u16,
    pub /: *mut *mut u16 tfnum; / PSC + 0x5c,
    pub reserved19: u16,
    pub /: *mut *mut u32 rfdata; / PSC + 0x60,
    pub /: *mut *mut u16 rfstat; / PSC + 0x64,
    pub reserved20: u16,
    pub /: *mut *mut u8 rfcntl; / PSC + 0x68,
    pub reserved21: [u8; 5],
    pub /: *mut *mut u16 rfalarm; / PSC + 0x6e,
    pub reserved22: u16,
    pub /: *mut *mut u16 rfrptr; / PSC + 0x72,
    pub reserved23: u16,
    pub /: *mut *mut u16 rfwptr; / PSC + 0x76,
    pub reserved24: u16,
    pub /: *mut *mut u16 rflrfptr; / PSC + 0x7a,
    pub reserved25: u16,
    pub /: *mut *mut u16 rflwfptr; / PSC + 0x7e,
    pub /: *mut *mut u32 tfdata; / PSC + 0x80,
    pub /: *mut *mut u16 tfstat; / PSC + 0x84,
    pub reserved26: u16,
    pub /: *mut *mut u8 tfcntl; / PSC + 0x88,
    pub reserved27: [u8; 5],
    pub /: *mut *mut u16 tfalarm; / PSC + 0x8e,
    pub reserved28: u16,
    pub /: *mut *mut u16 tfrptr; / PSC + 0x92,
    pub reserved29: u16,
    pub /: *mut *mut u16 tfwptr; / PSC + 0x96,
    pub reserved30: u16,
    pub /: *mut *mut u16 tflrfptr; / PSC + 0x9a,
    pub reserved31: u16,
    pub /: *mut *mut u16 tflwfptr; / PSC + 0x9e,
}

pub const MPC512x_PSC_FIFO_EOF: c_uint = 0x100;
pub const MPC512x_PSC_FIFO_RESET_SLICE: c_uint = 0x80;
pub const MPC512x_PSC_FIFO_ENABLE_SLICE: c_uint = 0x01;
pub const MPC512x_PSC_FIFO_ENABLE_DMA: c_uint = 0x04;
pub const MPC512x_PSC_FIFO_EMPTY: c_uint = 0x1;
pub const MPC512x_PSC_FIFO_FULL: c_uint = 0x2;
pub const MPC512x_PSC_FIFO_ALARM: c_uint = 0x4;
pub const MPC512x_PSC_FIFO_URERR: c_uint = 0x8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpc512x_psc_fifo {
    pub reserved1: [u32; 10],
    pub /: *mut *mut u32 txcmd; / PSC + 0x80,
    pub /: *mut *mut u32 txalarm; / PSC + 0x84,
    pub /: *mut *mut u32 txsr; / PSC + 0x88,
    pub /: *mut *mut u32 txisr; / PSC + 0x8c,
    pub /: *mut *mut u32 tximr; / PSC + 0x90,
    pub /: *mut *mut u32 txcnt; / PSC + 0x94,
    pub /: *mut *mut u32 txptr; / PSC + 0x98,
    pub /: *mut *mut u32 txsz; / PSC + 0x9c,
    pub reserved2: [u32; 7],
    pub txdata_8: u8,
    pub txdata_16: u16,
    pub txdata_32: u32,
    pub /: *mut *mut } txdata; / PSC + 0xbc,

    pub /: *mut *mut u32 rxcmd; / PSC + 0xc0,
    pub /: *mut *mut u32 rxalarm; / PSC + 0xc4,
    pub /: *mut *mut u32 rxsr; / PSC + 0xc8,
    pub /: *mut *mut u32 rxisr; / PSC + 0xcc,
    pub /: *mut *mut u32 rximr; / PSC + 0xd0,
    pub /: *mut *mut u32 rxcnt; / PSC + 0xd4,
    pub /: *mut *mut u32 rxptr; / PSC + 0xd8,
    pub /: *mut *mut u32 rxsz; / PSC + 0xdc,
    pub reserved3: [u32; 7],
    pub rxdata_8: u8,
    pub rxdata_16: u16,
    pub rxdata_32: u32,
    pub /: *mut *mut } rxdata; / PSC + 0xfc,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpc5125_psc {
    pub /: *mut *mut u8 mr1; / PSC + 0x00,
    pub reserved0: [u8; 3],
    pub /: *mut *mut u8 mr2; / PSC + 0x04,
    pub reserved1: [u8; 3],
    pub /: *mut *mut u16 status; / PSC + 0x08,
    pub reserved2: [u8; 2],
    pub /: *mut *mut u8 clock_select; / PSC + 0x0c,
    pub reserved3: [u8; 3],
    pub sr_csr: },
    pub /: *mut *mut u8 command; / PSC + 0x10,
    pub reserved4: [u8; 3],
    pub buffer_8: u8,
    pub buffer_16: u16,
    pub buffer_32: u32,
    pub buffer: },
    pub /: *mut *mut u8 ipcr; / PSC + 0x18,
    pub reserved5: [u8; 3],
    pub /: *mut *mut u8 acr; / PSC + 0x1c,
    pub reserved6: [u8; 3],
    pub ipcr_acr: },
    pub /: *mut *mut u16 isr; / PSC + 0x20,
    pub reserved7: [u8; 2],
    pub /: *mut *mut u16 imr; / PSC + 0x24,
    pub reserved8: [u8; 2],
    pub isr_imr: },
    pub /: *mut *mut u8 ctur; / PSC + 0x28,
    pub reserved9: [u8; 3],
    pub /: *mut *mut u8 ctlr; / PSC + 0x2c,
    pub reserved10: [u8; 3],
    pub /: *mut *mut u32 ccr; / PSC + 0x30,
    pub /: *mut *mut u32 ac97slots; / PSC + 0x34,
    pub /: *mut *mut u32 ac97cmd; / PSC + 0x38,
    pub /: *mut *mut u32 ac97data; / PSC + 0x3c,
    pub reserved11: [u8; 4],
    pub /: *mut *mut u8 ip; / PSC + 0x44,
    pub reserved12: [u8; 3],
    pub /: *mut *mut u8 op1; / PSC + 0x48,
    pub reserved13: [u8; 3],
    pub /: *mut *mut u8 op0; / PSC + 0x4c,
    pub reserved14: [u8; 3],
    pub /: *mut *mut u32 sicr; / PSC + 0x50,
    pub /: *mut *mut u8 reserved15[4]; / make eq. sizeof(mpc52xx_psc),
}
