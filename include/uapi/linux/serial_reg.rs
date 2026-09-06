//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/serial_reg.h
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


// SPDX-License-Identifier: GPL-1.0+ WITH Linux-syscall-note
//
// include/linux/serial_reg.h
//
// Copyright (C) 1992, 1994 by Theodore Ts'o.
//
// Redistribution of this file is permitted under the terms of the GNU
// Public License (GPL)
//
// These are the UART port assignments, expressed as offsets from the base
// register.  These assignments should hold for any serial port based on
// a 8250, 16450, or 16550(A).
//
// DLAB=0
//

pub const UART_IER_MSI: c_uint = 0x08 /* Enable Modem status interrupt */;
pub const UART_IER_RLSI: c_uint = 0x04 /* Enable receiver line status interrupt */;
pub const UART_IER_THRI: c_uint = 0x02 /* Enable Transmitter holding register int. */;
pub const UART_IER_RDI: c_uint = 0x01 /* Enable receiver data interrupt */;
//
// Sleep mode for ST16650 and TI16750.  For the ST16650, EFR[4]=1
//
pub const UART_IERX_SLEEP: c_uint = 0x10 /* Enable sleep mode */;

pub const UART_IIR_NO_INT: c_uint = 0x01 /* No interrupts pending */;
pub const UART_IIR_ID: c_uint = 0x0e /* Mask for the interrupt ID */;
pub const UART_IIR_MSI: c_uint = 0x00 /* Modem status interrupt */;
pub const UART_IIR_THRI: c_uint = 0x02 /* Transmitter holding register empty */;
pub const UART_IIR_RDI: c_uint = 0x04 /* Receiver data interrupt */;
pub const UART_IIR_RLSI: c_uint = 0x06 /* Receiver line status interrupt */;
pub const UART_IIR_BUSY: c_uint = 0x07 /* DesignWare APB Busy Detect */;
pub const UART_IIR_RX_TIMEOUT: c_uint = 0x0c /* OMAP RX Timeout interrupt */;
pub const UART_IIR_XOFF: c_uint = 0x10 /* OMAP XOFF/Special Character */;
pub const UART_IIR_CTS_RTS_DSR: c_uint = 0x20 /* OMAP CTS/RTS/DSR Change */;
pub const UART_IIR_64BYTE_FIFO: c_uint = 0x20 /* 16750 64 bytes FIFO */;
pub const UART_IIR_FIFO_ENABLED: c_uint = 0xc0 /* FIFOs enabled / port type identification */;
pub const UART_IIR_FIFO_ENABLED_8250: c_uint = 0x00	/* 8250: no FIFO */;
pub const UART_IIR_FIFO_ENABLED_16550: c_uint = 0x80	/* 16550: (broken/unusable) FIFO */;
pub const UART_IIR_FIFO_ENABLED_16550A: c_uint = 0xc0	/* 16550A: FIFO enabled */;
pub const UART_IIR_FIFO_ENABLED_16750: c_uint = 0xe0	/* 16750: 64 bytes FIFO enabled */;

pub const UART_FCR_ENABLE_FIFO: c_uint = 0x01 /* Enable the FIFO */;
pub const UART_FCR_CLEAR_RCVR: c_uint = 0x02 /* Clear the RCVR FIFO */;
pub const UART_FCR_CLEAR_XMIT: c_uint = 0x04 /* Clear the XMIT FIFO */;
pub const UART_FCR_DMA_SELECT: c_uint = 0x08 /* For DMA applications */;
//
// Note: The FIFO trigger levels are chip specific:
// RX:76 = 00  01  10  11	TX:54 = 00  01  10  11
// PC16550D:	 1   4   8  14		xx  xx  xx  xx
// TI16C550A:	 1   4   8  14          xx  xx  xx  xx
// TI16C550C:	 1   4   8  14          xx  xx  xx  xx
// ST16C550:	 1   4   8  14		xx  xx  xx  xx
// ST16C650:	 8  16  24  28		16   8  24  30	PORT_16650V2
// NS16C552:	 1   4   8  14		xx  xx  xx  xx
// ST16C654:	 8  16  56  60		 8  16  32  56	PORT_16654
// TI16C750:	 1  16  32  56		xx  xx  xx  xx	PORT_16750
// TI16C752:	 8  16  56  60		 8  16  32  56
// OX16C950:	16  32 112 120		16  32  64 112	PORT_16C950
// Tegra:	 1   4   8  14		16   8   4   1	PORT_TEGRA
//
pub const UART_FCR_R_TRIG_00: c_uint = 0x00;
pub const UART_FCR_R_TRIG_01: c_uint = 0x40;
pub const UART_FCR_R_TRIG_10: c_uint = 0x80;
pub const UART_FCR_R_TRIG_11: c_uint = 0xc0;
pub const UART_FCR_T_TRIG_00: c_uint = 0x00;
pub const UART_FCR_T_TRIG_01: c_uint = 0x10;
pub const UART_FCR_T_TRIG_10: c_uint = 0x20;
pub const UART_FCR_T_TRIG_11: c_uint = 0x30;
pub const UART_FCR_TRIGGER_MASK: c_uint = 0xC0 /* Mask for the FIFO trigger range */;
pub const UART_FCR_TRIGGER_1: c_uint = 0x00 /* Mask for trigger set at 1 */;
pub const UART_FCR_TRIGGER_4: c_uint = 0x40 /* Mask for trigger set at 4 */;
pub const UART_FCR_TRIGGER_8: c_uint = 0x80 /* Mask for trigger set at 8 */;
pub const UART_FCR_TRIGGER_14: c_uint = 0xC0 /* Mask for trigger set at 14 */;
// 16650 definitions
pub const UART_FCR6_R_TRIGGER_8: c_uint = 0x00 /* Mask for receive trigger set at 1 */;
pub const UART_FCR6_R_TRIGGER_16: c_uint = 0x40 /* Mask for receive trigger set at 4 */;
pub const UART_FCR6_R_TRIGGER_24: c_uint = 0x80 /* Mask for receive trigger set at 8 */;
pub const UART_FCR6_R_TRIGGER_28: c_uint = 0xC0 /* Mask for receive trigger set at 14 */;
pub const UART_FCR6_T_TRIGGER_16: c_uint = 0x00 /* Mask for transmit trigger set at 16 */;
pub const UART_FCR6_T_TRIGGER_8: c_uint = 0x10 /* Mask for transmit trigger set at 8 */;
pub const UART_FCR6_T_TRIGGER_24: c_uint = 0x20 /* Mask for transmit trigger set at 24 */;
pub const UART_FCR6_T_TRIGGER_30: c_uint = 0x30 /* Mask for transmit trigger set at 30 */;
pub const UART_FCR7_64BYTE: c_uint = 0x20 /* Go into 64 byte mode (TI16C750 and;
pub const UART_FCR_R_TRIG_SHIFT: c_int = 6;

pub const UART_FCR_R_TRIG_MAX_STATE: c_int = 4;

//
// Note: if the word length is 5 bits (UART_LCR_WLEN5), then setting
// UART_LCR_STOP will select 1.5 stop bits, not 2 stop bits.
//
pub const UART_LCR_DLAB: c_uint = 0x80 /* Divisor latch access bit */;
pub const UART_LCR_SBC: c_uint = 0x40 /* Set break control */;
pub const UART_LCR_SPAR: c_uint = 0x20 /* Stick parity (?) */;
pub const UART_LCR_EPAR: c_uint = 0x10 /* Even parity select */;
pub const UART_LCR_PARITY: c_uint = 0x08 /* Parity Enable */;
pub const UART_LCR_STOP: c_uint = 0x04 /* Stop bits: 0=1 bit, 1=2 bits */;
pub const UART_LCR_WLEN5: c_uint = 0x00 /* Wordlength: 5 bits */;
pub const UART_LCR_WLEN6: c_uint = 0x01 /* Wordlength: 6 bits */;
pub const UART_LCR_WLEN7: c_uint = 0x02 /* Wordlength: 7 bits */;
pub const UART_LCR_WLEN8: c_uint = 0x03 /* Wordlength: 8 bits */;
//
// Access to some registers depends on register access / configuration
// mode.
//

pub const UART_LCR_CONF_MODE_B: c_uint = 0xBF		/* Configutation mode B */;

pub const UART_MCR_CLKSEL: c_uint = 0x80 /* Divide clock by 4 (TI16C752, EFR[4]=1) */;
pub const UART_MCR_TCRTLR: c_uint = 0x40 /* Access TCR/TLR (TI16C752, EFR[4]=1) */;
pub const UART_MCR_XONANY: c_uint = 0x20 /* Enable Xon Any (TI16C752, EFR[4]=1) */;
pub const UART_MCR_AFE: c_uint = 0x20 /* Enable auto-RTS/CTS (TI16C550C/TI16C750) */;
pub const UART_MCR_LOOP: c_uint = 0x10 /* Enable loopback test mode */;
pub const UART_MCR_OUT2: c_uint = 0x08 /* Out2 complement */;
pub const UART_MCR_OUT1: c_uint = 0x04 /* Out1 complement */;
pub const UART_MCR_RTS: c_uint = 0x02 /* RTS complement */;
pub const UART_MCR_DTR: c_uint = 0x01 /* DTR complement */;

pub const UART_LSR_FIFOE: c_uint = 0x80 /* Fifo error */;
pub const UART_LSR_TEMT: c_uint = 0x40 /* Transmitter empty */;
pub const UART_LSR_THRE: c_uint = 0x20 /* Transmit-hold-register empty */;
pub const UART_LSR_BI: c_uint = 0x10 /* Break interrupt indicator */;
pub const UART_LSR_FE: c_uint = 0x08 /* Frame error indicator */;
pub const UART_LSR_PE: c_uint = 0x04 /* Parity error indicator */;
pub const UART_LSR_OE: c_uint = 0x02 /* Overrun error indicator */;
pub const UART_LSR_DR: c_uint = 0x01 /* Receiver data ready */;

pub const UART_MSR_DCD: c_uint = 0x80 /* Data Carrier Detect */;
pub const UART_MSR_RI: c_uint = 0x40 /* Ring Indicator */;
pub const UART_MSR_DSR: c_uint = 0x20 /* Data Set Ready */;
pub const UART_MSR_CTS: c_uint = 0x10 /* Clear to Send */;
pub const UART_MSR_DDCD: c_uint = 0x08 /* Delta DCD */;
pub const UART_MSR_TERI: c_uint = 0x04 /* Trailing edge ring indicator */;
pub const UART_MSR_DDSR: c_uint = 0x02 /* Delta DSR */;
pub const UART_MSR_DCTS: c_uint = 0x01 /* Delta CTS */;

//
// DLAB=1
//

pub const UART_DIV_MAX: c_uint = 0xFFFF	/* Max divisor value */;
//
// LCR=0xBF (or DLAB=1 for 16C660)
//

pub const UART_EFR_CTS: c_uint = 0x80 /* CTS flow control */;
pub const UART_EFR_RTS: c_uint = 0x40 /* RTS flow control */;
pub const UART_EFR_SCD: c_uint = 0x20 /* Special character detect */;
pub const UART_EFR_ECB: c_uint = 0x10 /* Enhanced control bit */;
//
// the low four bits control software flow control
//
// LCR=0xBF, TI16C752, ST16650, ST16650A, ST16654
//

//
// EFR[4]=1 MCR[6]=1, TI16C752
//

//
// LCR=0xBF, XR16C85x
//

// In: Fifo count
// Out: Fifo custom trigger levels
//
// These are the definitions for the Programmable Trigger Register
//
pub const UART_TRG_1: c_uint = 0x01;
pub const UART_TRG_4: c_uint = 0x04;
pub const UART_TRG_8: c_uint = 0x08;
pub const UART_TRG_16: c_uint = 0x10;
pub const UART_TRG_32: c_uint = 0x20;
pub const UART_TRG_64: c_uint = 0x40;
pub const UART_TRG_96: c_uint = 0x60;
pub const UART_TRG_120: c_uint = 0x78;
pub const UART_TRG_128: c_uint = 0x80;

pub const UART_FCTR_RTS_NODELAY: c_uint = 0x00  /* RTS flow control delay */;
pub const UART_FCTR_RTS_4DELAY: c_uint = 0x01;
pub const UART_FCTR_RTS_6DELAY: c_uint = 0x02;
pub const UART_FCTR_RTS_8DELAY: c_uint = 0x03;
pub const UART_FCTR_IRDA: c_uint = 0x04  /* IrDa data encode select */;
pub const UART_FCTR_TX_INT: c_uint = 0x08  /* Tx interrupt type select */;
pub const UART_FCTR_TRGA: c_uint = 0x00  /* Tx/Rx 550 trigger table select */;
pub const UART_FCTR_TRGB: c_uint = 0x10  /* Tx/Rx 650 trigger table select */;
pub const UART_FCTR_TRGC: c_uint = 0x20  /* Tx/Rx 654 trigger table select */;
pub const UART_FCTR_TRGD: c_uint = 0x30  /* Tx/Rx 850 programmable trigger select */;
pub const UART_FCTR_SCR_SWAP: c_uint = 0x40  /* Scratch pad register swap */;
pub const UART_FCTR_RX: c_uint = 0x00  /* Programmable trigger mode select */;
pub const UART_FCTR_TX: c_uint = 0x80  /* Programmable trigger mode select */;
//
// LCR=0xBF, FCTR[6]=1
//

pub const UART_EMSR_FIFO_COUNT: c_uint = 0x01  /* Rx/Tx select */;
pub const UART_EMSR_ALT_COUNT: c_uint = 0x02  /* Alternating count select */;
//
// The Intel XScale on-chip UARTs define these bits
//
pub const UART_IER_DMAE: c_uint = 0x80	/* DMA Requests Enable */;
pub const UART_IER_UUE: c_uint = 0x40	/* UART Unit Enable */;
pub const UART_IER_NRZE: c_uint = 0x20	/* NRZ coding Enable */;
pub const UART_IER_RTOIE: c_uint = 0x10	/* Receiver Time Out Interrupt Enable */;
pub const UART_IIR_TOD: c_uint = 0x08	/* Character Timeout Indication Detected */;
pub const UART_FCR_PXAR1: c_uint = 0x00	/* receive FIFO threshold = 1 */;
pub const UART_FCR_PXAR8: c_uint = 0x40	/* receive FIFO threshold = 8 */;
pub const UART_FCR_PXAR16: c_uint = 0x80	/* receive FIFO threshold = 16 */;
pub const UART_FCR_PXAR32: c_uint = 0xc0	/* receive FIFO threshold = 32 */;
//
// These register definitions are for the 16C950
//
pub const UART_ASR: c_uint = 0x01	/* Additional Status Register */;
pub const UART_RFL: c_uint = 0x03	/* Receiver FIFO level */;
pub const UART_TFL: c_uint = 0x04	/* Transmitter FIFO level */;
pub const UART_ICR: c_uint = 0x05	/* Index Control Register */;
// The 16950 ICR registers
pub const UART_ACR: c_uint = 0x00	/* Additional Control Register */;
pub const UART_CPR: c_uint = 0x01	/* Clock Prescalar Register */;
pub const UART_TCR: c_uint = 0x02	/* Times Clock Register */;
pub const UART_CKS: c_uint = 0x03	/* Clock Select Register */;
pub const UART_TTL: c_uint = 0x04	/* Transmitter Interrupt Trigger Level */;
pub const UART_RTL: c_uint = 0x05	/* Receiver Interrupt Trigger Level */;
pub const UART_FCL: c_uint = 0x06	/* Flow Control Level Lower */;
pub const UART_FCH: c_uint = 0x07	/* Flow Control Level Higher */;
pub const UART_ID1: c_uint = 0x08	/* ID #1 */;
pub const UART_ID2: c_uint = 0x09	/* ID #2 */;
pub const UART_ID3: c_uint = 0x0A	/* ID #3 */;
pub const UART_REV: c_uint = 0x0B	/* Revision */;
pub const UART_CSR: c_uint = 0x0C	/* Channel Software Reset */;
pub const UART_NMR: c_uint = 0x0D	/* Nine-bit Mode Register */;
pub const UART_CTR: c_uint = 0xFF;
//
// The 16C950 Additional Control Register
//
pub const UART_ACR_RXDIS: c_uint = 0x01	/* Receiver disable */;
pub const UART_ACR_TXDIS: c_uint = 0x02	/* Transmitter disable */;
pub const UART_ACR_DSRFC: c_uint = 0x04	/* DSR Flow Control */;
pub const UART_ACR_TLENB: c_uint = 0x20	/* 950 trigger levels enable */;
pub const UART_ACR_ICRRD: c_uint = 0x40	/* ICR Read enable */;
pub const UART_ACR_ASREN: c_uint = 0x80	/* Additional status enable */;
//
// These definitions are for the RSA-DV II/S card, from
//
// Kiyokazu SUTO <suto@ks-and-ks.ne.jp>
//

//
// The RSA DSV/II board has two fixed clock frequencies.  One is the
// standard rate, and the other is 8 times faster.
//

// Extra registers for TI DA8xx/66AK2x
pub const UART_DA830_PWREMU_MGMT: c_int = 12;
// PWREMU_MGMT register bits

//
// Extra serial register definitions for the internal UARTs
// in TI OMAP processors.
//
pub const OMAP1_UART1_BASE: c_uint = 0xfffb0000;
pub const OMAP1_UART2_BASE: c_uint = 0xfffb0800;
pub const OMAP1_UART3_BASE: c_uint = 0xfffb9800;
pub const UART_OMAP_MDR1: c_uint = 0x08	/* Mode definition register */;
pub const UART_OMAP_MDR2: c_uint = 0x09	/* Mode definition register 2 */;
pub const UART_OMAP_SCR: c_uint = 0x10	/* Supplementary control register */;
pub const UART_OMAP_SSR: c_uint = 0x11	/* Supplementary status register */;
pub const UART_OMAP_EBLR: c_uint = 0x12	/* BOF length register */;
pub const UART_OMAP_OSC_12M_SEL: c_uint = 0x13	/* OMAP1510 12MHz osc select */;
pub const UART_OMAP_MVER: c_uint = 0x14	/* Module version register */;
pub const UART_OMAP_SYSC: c_uint = 0x15	/* System configuration register */;
pub const UART_OMAP_SYSS: c_uint = 0x16	/* System status register */;
pub const UART_OMAP_WER: c_uint = 0x17	/* Wake-up enable register */;
pub const UART_OMAP_TX_LVL: c_uint = 0x1a	/* TX FIFO level register */;
//
// These are the definitions for the MDR1 register
//
pub const UART_OMAP_MDR1_16X_MODE: c_uint = 0x00	/* UART 16x mode */;
pub const UART_OMAP_MDR1_SIR_MODE: c_uint = 0x01	/* SIR mode */;
pub const UART_OMAP_MDR1_16X_ABAUD_MODE: c_uint = 0x02	/* UART 16x auto-baud */;
pub const UART_OMAP_MDR1_13X_MODE: c_uint = 0x03	/* UART 13x mode */;
pub const UART_OMAP_MDR1_MIR_MODE: c_uint = 0x04	/* MIR mode */;
pub const UART_OMAP_MDR1_FIR_MODE: c_uint = 0x05	/* FIR mode */;
pub const UART_OMAP_MDR1_CIR_MODE: c_uint = 0x06	/* CIR mode */;
pub const UART_OMAP_MDR1_DISABLE: c_uint = 0x07	/* Disable (default state) */;
//
// These are definitions for the Altera ALTR_16550_F32/F64/F128
// Normalized from 0x100 to 0x40 because of shift by 2 (32 bit regs).
//
pub const UART_ALTR_AFR: c_uint = 0x40	/* Additional Features Register */;
pub const UART_ALTR_EN_TXFIFO_LW: c_uint = 0x01	/* Enable the TX FIFO Low Watermark */;
pub const UART_ALTR_TX_LOW: c_uint = 0x41	/* Tx FIFO Low Watermark */;
