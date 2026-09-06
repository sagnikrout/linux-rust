//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/comedi/drivers/s626.h
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
// comedi/drivers/s626.h
// Sensoray s626 Comedi driver, header file
//
// COMEDI - Linux Control and Measurement Device Interface
// Copyright (C) 2000 David A. Schleef <ds@schleef.org>
//
// Based on Sensoray Model 626 Linux driver Version 0.2
// Copyright (C) 2002-2004 Sensoray Co., Inc.
//

// Macro flag: #define S626_H_INCLUDED

pub const S626_ADC_CHANNELS: c_int = 16;
pub const S626_DAC_CHANNELS: c_int = 4;
pub const S626_ENCODER_CHANNELS: c_int = 6;
pub const S626_DIO_CHANNELS: c_int = 48;

// Number of extended-capability
// DIO channels.
//

// PCI bus interface types.

pub const S626_RANGE_5V: c_uint = 0x10	/* +/-5V range */;
pub const S626_RANGE_10V: c_uint = 0x00	/* +/-10V range */;
pub const S626_EOPL: c_uint = 0x80	/* End of ADC poll list marker. */;
pub const S626_GSEL_BIPOLAR5V: c_uint = 0x00F0	/* S626_LP_GSEL setting 5V bipolar. */;
pub const S626_GSEL_BIPOLAR10V: c_uint = 0x00A0	/* S626_LP_GSEL setting 10V bipolar. */;
// Error codes that must be visible to this base class.
pub const S626_ERR_ILLEGAL_PARM: c_uint = 0x00010000	/*;
// Illegal function parameter
// value was specified.
//
pub const S626_ERR_I2C: c_uint = 0x00020000	/* I2C error. */;
pub const S626_ERR_COUNTERSETUP: c_uint = 0x00200000	/*;
// Illegal setup specified for
// counter channel.
//
pub const S626_ERR_DEBI_TIMEOUT: c_uint = 0x00400000	/* DEBI transfer timed out. */;
//
// Organization (physical order) and size (in DWORDs) of logical DMA buffers
// contained by ANA_DMABUF.
//

// ADC DMA buffer must hold 16 samples,
// plus pre/post garbage samples.
//

// DAC output DMA buffer holds a single
// sample.
//
// All remaining space in 4KB DMA buffer is available for the RPS1 program.
// Address offsets, in DWORDS, from base of DMA buffer.

// Interrupt enable bit in ISR and IER.
pub const S626_IRQ_GPIO3: c_uint = 0x00000040	/* IRQ enable for GPIO3. */;
pub const S626_IRQ_RPS1: c_uint = 0x10000000;
pub const S626_ISR_AFOU: c_uint = 0x00000800;
// Audio fifo under/overflow  detected.
pub const S626_IRQ_COINT1A: c_uint = 0x0400	/* counter 1A overflow interrupt mask */;
pub const S626_IRQ_COINT1B: c_uint = 0x0800	/* counter 1B overflow interrupt mask */;
pub const S626_IRQ_COINT2A: c_uint = 0x1000	/* counter 2A overflow interrupt mask */;
pub const S626_IRQ_COINT2B: c_uint = 0x2000	/* counter 2B overflow interrupt mask */;
pub const S626_IRQ_COINT3A: c_uint = 0x4000	/* counter 3A overflow interrupt mask */;
pub const S626_IRQ_COINT3B: c_uint = 0x8000	/* counter 3B overflow interrupt mask */;
// RPS command codes.
pub const S626_RPS_CLRSIGNAL: c_uint = 0x00000000	/* CLEAR SIGNAL */;
pub const S626_RPS_SETSIGNAL: c_uint = 0x10000000	/* SET SIGNAL */;
pub const S626_RPS_NOP: c_uint = 0x00000000	/* NOP */;
pub const S626_RPS_PAUSE: c_uint = 0x20000000	/* PAUSE */;
pub const S626_RPS_UPLOAD: c_uint = 0x40000000	/* UPLOAD */;
pub const S626_RPS_JUMP: c_uint = 0x80000000	/* JUMP */;
pub const S626_RPS_LDREG: c_uint = 0x90000100	/* LDREG (1 uint32_t only) */;
pub const S626_RPS_STREG: c_uint = 0xA0000100	/* STREG (1 uint32_t only) */;
pub const S626_RPS_STOP: c_uint = 0x50000000	/* STOP */;
pub const S626_RPS_IRQ: c_uint = 0x60000000	/* IRQ */;
pub const S626_RPS_LOGICAL_OR: c_uint = 0x08000000	/* Logical OR conditionals. */;
pub const S626_RPS_INVERT: c_uint = 0x04000000	/*;
// Test for negated
// semaphores.
//
pub const S626_RPS_DEBI: c_uint = 0x00000002	/* DEBI done */;
pub const S626_RPS_SIG0: c_uint = 0x00200000	/*;
// RPS semaphore 0
// (used by ADC).
//
pub const S626_RPS_SIG1: c_uint = 0x00400000	/*;
// RPS semaphore 1
// (used by DAC).
//
pub const S626_RPS_SIG2: c_uint = 0x00800000	/*;
// RPS semaphore 2
// (not used).
//
pub const S626_RPS_GPIO2: c_uint = 0x00080000	/* RPS GPIO2 */;
pub const S626_RPS_GPIO3: c_uint = 0x00100000	/* RPS GPIO3 */;

// Trigger/status for
// ADC's RPS program.
//

// Trigger/status for
// DAC's RPS program.
//
// RPS clock parameters.

// This is apparent ratio of
// PCI/RPS clks (undocumented!!).
//

//
// Number of RPS clocks in one
// microsecond.
//
// Event counter source addresses.
pub const S626_SBA_RPS_A0: c_uint = 0x27	/* Time of RPS0 busy, in PCI clocks. */;
// GPIO constants.
pub const S626_GPIO_BASE: c_uint = 0x10004000	/*;
// GPIO 0,2,3 = inputs,
// GPIO3 = IRQ; GPIO1 = out.
//
pub const S626_GPIO1_LO: c_uint = 0x00000000	/* GPIO1 set to LOW. */;
pub const S626_GPIO1_HI: c_uint = 0x00001000	/* GPIO1 set to HIGH. */;
// Primary Status Register (PSR) constants.
pub const S626_PSR_DEBI_E: c_uint = 0x00040000	/* DEBI event flag. */;
pub const S626_PSR_DEBI_S: c_uint = 0x00080000	/* DEBI status flag. */;
pub const S626_PSR_A2_IN: c_uint = 0x00008000	/*;
// Audio output DMA2 protection
// address reached.
//
pub const S626_PSR_AFOU: c_uint = 0x00000800	/*;
// Audio FIFO under/overflow
// detected.
//
pub const S626_PSR_GPIO2: c_uint = 0x00000020	/*;
// GPIO2 input pin: 0=AdcBusy,
// 1=AdcIdle.
//
pub const S626_PSR_EC0S: c_uint = 0x00000001	/*;
// Event counter 0 threshold
// reached.
//
// Secondary Status Register (SSR) constants.
pub const S626_SSR_AF2_OUT: c_uint = 0x00000200	/*;
// Audio 2 output FIFO
// under/overflow detected.
//
// Master Control Register 1 (MC1) constants.
pub const S626_MC1_SOFT_RESET: c_uint = 0x80000000	/* Invoke 7146 soft reset. */;
pub const S626_MC1_SHUTDOWN: c_uint = 0x3FFF0000	/*;
// Shut down all MC1-controlled
// enables.
//
pub const S626_MC1_ERPS1: c_uint = 0x2000	/* Enab/disable RPS task 1. */;
pub const S626_MC1_ERPS0: c_uint = 0x1000	/* Enab/disable RPS task 0. */;
pub const S626_MC1_DEBI: c_uint = 0x0800	/* Enab/disable DEBI pins. */;
pub const S626_MC1_AUDIO: c_uint = 0x0200	/* Enab/disable audio port pins. */;
pub const S626_MC1_I2C: c_uint = 0x0100	/* Enab/disable I2C interface. */;
pub const S626_MC1_A2OUT: c_uint = 0x0008	/* Enab/disable transfer on A2 out. */;
pub const S626_MC1_A2IN: c_uint = 0x0004	/* Enab/disable transfer on A2 in. */;
pub const S626_MC1_A1IN: c_uint = 0x0001	/* Enab/disable transfer on A1 in. */;
// Master Control Register 2 (MC2) constants.
pub const S626_MC2_UPLD_DEBI: c_uint = 0x0002	/* Upload DEBI. */;
pub const S626_MC2_UPLD_IIC: c_uint = 0x0001	/* Upload I2C. */;
pub const S626_MC2_RPSSIG2: c_uint = 0x2000	/* RPS signal 2 (not used). */;
pub const S626_MC2_RPSSIG1: c_uint = 0x1000	/* RPS signal 1 (DAC RPS busy). */;
pub const S626_MC2_RPSSIG0: c_uint = 0x0800	/* RPS signal 0 (ADC RPS busy). */;

// PCI BUS (SAA7146) REGISTER ADDRESS OFFSETS
pub const S626_P_PCI_BT_A: c_uint = 0x004C	/* Audio DMA burst/threshold control. */;
pub const S626_P_DEBICFG: c_uint = 0x007C	/* DEBI configuration. */;
pub const S626_P_DEBICMD: c_uint = 0x0080	/* DEBI command. */;
pub const S626_P_DEBIPAGE: c_uint = 0x0084	/* DEBI page. */;
pub const S626_P_DEBIAD: c_uint = 0x0088	/* DEBI target address. */;
pub const S626_P_I2CCTRL: c_uint = 0x008C	/* I2C control. */;
pub const S626_P_I2CSTAT: c_uint = 0x0090	/* I2C status. */;
pub const S626_P_BASEA2_IN: c_uint = 0x00AC	/*;
// Audio input 2 base physical DMAbuf
// address.
//
pub const S626_P_PROTA2_IN: c_uint = 0x00B0	/*;
// Audio input 2 physical DMAbuf
// protection address.
//
pub const S626_P_PAGEA2_IN: c_uint = 0x00B4	/* Audio input 2 paging attributes. */;
pub const S626_P_BASEA2_OUT: c_uint = 0x00B8	/*;
// Audio output 2 base physical DMAbuf
// address.
//
pub const S626_P_PROTA2_OUT: c_uint = 0x00BC	/*;
// Audio output 2 physical DMAbuf
// protection address.
//
pub const S626_P_PAGEA2_OUT: c_uint = 0x00C0	/* Audio output 2 paging attributes. */;
pub const S626_P_RPSPAGE0: c_uint = 0x00C4	/* RPS0 page. */;
pub const S626_P_RPSPAGE1: c_uint = 0x00C8	/* RPS1 page. */;
pub const S626_P_RPS0_TOUT: c_uint = 0x00D4	/* RPS0 time-out. */;
pub const S626_P_RPS1_TOUT: c_uint = 0x00D8	/* RPS1 time-out. */;
pub const S626_P_IER: c_uint = 0x00DC	/* Interrupt enable. */;
pub const S626_P_GPIO: c_uint = 0x00E0	/* General-purpose I/O. */;
pub const S626_P_EC1SSR: c_uint = 0x00E4	/* Event counter set 1 source select. */;
pub const S626_P_ECT1R: c_uint = 0x00EC	/* Event counter threshold set 1. */;
pub const S626_P_ACON1: c_uint = 0x00F4	/* Audio control 1. */;
pub const S626_P_ACON2: c_uint = 0x00F8	/* Audio control 2. */;
pub const S626_P_MC1: c_uint = 0x00FC	/* Master control 1. */;
pub const S626_P_MC2: c_uint = 0x0100	/* Master control 2. */;
pub const S626_P_RPSADDR0: c_uint = 0x0104	/* RPS0 instruction pointer. */;
pub const S626_P_RPSADDR1: c_uint = 0x0108	/* RPS1 instruction pointer. */;
pub const S626_P_ISR: c_uint = 0x010C	/* Interrupt status. */;
pub const S626_P_PSR: c_uint = 0x0110	/* Primary status. */;
pub const S626_P_SSR: c_uint = 0x0114	/* Secondary status. */;
pub const S626_P_EC1R: c_uint = 0x0118	/* Event counter set 1. */;
pub const S626_P_ADP4: c_uint = 0x0138	/*;
// Logical audio DMA pointer of audio
// input FIFO A2_IN.
//
pub const S626_P_FB_BUFFER1: c_uint = 0x0144	/* Audio feedback buffer 1. */;
pub const S626_P_FB_BUFFER2: c_uint = 0x0148	/* Audio feedback buffer 2. */;
pub const S626_P_TSL1: c_uint = 0x0180	/* Audio time slot list 1. */;
pub const S626_P_TSL2: c_uint = 0x01C0	/* Audio time slot list 2. */;
// LOCAL BUS (GATE ARRAY) REGISTER ADDRESS OFFSETS
// Analog I/O registers:
pub const S626_LP_DACPOL: c_uint = 0x0082	/* Write DAC polarity. */;
pub const S626_LP_GSEL: c_uint = 0x0084	/* Write ADC gain. */;
pub const S626_LP_ISEL: c_uint = 0x0086	/* Write ADC channel select. */;
// Digital I/O registers

// Counter registers (read/write): 0A 1A 2A 0B 1B 2B

// Counter PreLoad (write) and Latch (read) Registers: 0A 1A 2A 0B 1B 2B

// Miscellaneous Registers (read/write):
pub const S626_LP_MISC1: c_uint = 0x0088	/* Read/write Misc1. */;
pub const S626_LP_WRMISC2: c_uint = 0x0090	/* Write Misc2. */;
pub const S626_LP_RDMISC2: c_uint = 0x0082	/* Read Misc2. */;
// Bit masks for MISC1 register that are the same for reads and writes.
pub const S626_MISC1_WENABLE: c_uint = 0x8000	/*;
// enab writes to MISC2 (except Clear
// Watchdog bit).
//
pub const S626_MISC1_WDISABLE: c_uint = 0x0000	/* Disable writes to MISC2. */;
pub const S626_MISC1_EDCAP: c_uint = 0x1000	/*;
// Enable edge capture on DIO chans
// specified by S626_LP_WRCAPSELx.
//
pub const S626_MISC1_NOEDCAP: c_uint = 0x0000	/*;
// Disable edge capture on specified
// DIO chans.
//
// Bit masks for MISC1 register reads.
pub const S626_RDMISC1_WDTIMEOUT: c_uint = 0x4000	/* Watchdog timer timed out. */;
// Bit masks for MISC2 register writes.
pub const S626_WRMISC2_WDCLEAR: c_uint = 0x8000	/* Reset watchdog timer to zero. */;
pub const S626_WRMISC2_CHARGE_ENABLE: c_uint = 0x4000 /* Enable battery trickle charging. */;
// Bit masks for MISC2 register that are the same for reads and writes.
pub const S626_MISC2_BATT_ENABLE: c_uint = 0x0008	/* Backup battery enable. */;
pub const S626_MISC2_WDENABLE: c_uint = 0x0004	/* Watchdog timer enable. */;
pub const S626_MISC2_WDPERIOD_MASK: c_uint = 0x0003	/* Watchdog interval select mask. */;
// Bit masks for ACON1 register.
pub const S626_A2_RUN: c_uint = 0x40000000	/* Run A2 based on TSL2. */;
pub const S626_A1_RUN: c_uint = 0x20000000	/* Run A1 based on TSL1. */;
pub const S626_A1_SWAP: c_uint = 0x00200000	/* Use big-endian for A1. */;
pub const S626_A2_SWAP: c_uint = 0x00100000	/* Use big-endian for A2. */;
pub const S626_WS_MODES: c_uint = 0x00019999	/*;
// WS0 = TSL1 trigger input,
// WS1-WS4 = CS* outputs.
//

// Base ACON1 config: always run
// A1 based on TSL1.
//

// Start ADC: run A1
// based on TSL1.
//

// Start transmit to DAC: run A2 based on TSL2.

// Bit masks for ACON2 register.
pub const S626_A1_CLKSRC_BCLK1: c_uint = 0x00000000	/* A1 bit rate = BCLK1 (ADC). */;
pub const S626_A2_CLKSRC_X1: c_uint = 0x00800000	/*;
// A2 bit rate = ACLK/1
// (DACs).
//
pub const S626_A2_CLKSRC_X2: c_uint = 0x00C00000	/*;
// A2 bit rate = ACLK/2
// (DACs).
//
pub const S626_A2_CLKSRC_X4: c_uint = 0x01400000	/*;
// A2 bit rate = ACLK/4
// (DACs).
//
pub const S626_INVERT_BCLK2: c_uint = 0x00100000	/* Invert BCLK2 (DACs). */;
pub const S626_BCLK2_OE: c_uint = 0x00040000	/* Enable BCLK2 (DACs). */;
pub const S626_ACON2_XORMASK: c_uint = 0x000C0000	/*;
// XOR mask for ACON2
// active-low bits.
//

// Bit masks for timeslot records.
pub const S626_WS1: c_uint = 0x40000000	/* WS output to assert. */;
pub const S626_WS2: c_uint = 0x20000000;
pub const S626_WS3: c_uint = 0x10000000;
pub const S626_WS4: c_uint = 0x08000000;
pub const S626_RSD1: c_uint = 0x01000000	/* Shift A1 data in on SD1. */;
pub const S626_SDW_A1: c_uint = 0x00800000	/*;
// Store rcv'd char at next char
// slot of DWORD1 buffer.
//
pub const S626_SIB_A1: c_uint = 0x00400000	/*;
// Store rcv'd char at next
// char slot of FB1 buffer.
//
pub const S626_SF_A1: c_uint = 0x00200000	/*;
// Write unsigned long
// buffer to input FIFO.
//
// Select parallel-to-serial converter's data source:
pub const S626_XFIFO_0: c_uint = 0x00000000	/* Data fifo byte 0. */;
pub const S626_XFIFO_1: c_uint = 0x00000010	/* Data fifo byte 1. */;
pub const S626_XFIFO_2: c_uint = 0x00000020	/* Data fifo byte 2. */;
pub const S626_XFIFO_3: c_uint = 0x00000030	/* Data fifo byte 3. */;
pub const S626_XFB0: c_uint = 0x00000040	/* FB_BUFFER byte 0. */;
pub const S626_XFB1: c_uint = 0x00000050	/* FB_BUFFER byte 1. */;
pub const S626_XFB2: c_uint = 0x00000060	/* FB_BUFFER byte 2. */;
pub const S626_XFB3: c_uint = 0x00000070	/* FB_BUFFER byte 3. */;
pub const S626_SIB_A2: c_uint = 0x00000200	/*;
// Store next dword from A2's
// input shifter to FB2
// buffer.
//
pub const S626_SF_A2: c_uint = 0x00000100	/*;
// Store next dword from A2's
// input shifter to its input
// fifo.
//
pub const S626_LF_A2: c_uint = 0x00000080	/*;
// Load next dword from A2's
// output fifo into its
// output dword buffer.
//
pub const S626_XSD2: c_uint = 0x00000008	/* Shift data out on SD2. */;
pub const S626_RSD3: c_uint = 0x00001800	/* Shift data in on SD3. */;
pub const S626_RSD2: c_uint = 0x00001000	/* Shift data in on SD2. */;
pub const S626_LOW_A2: c_uint = 0x00000002	/*;
// Drive last SD low for 7 clks,
// then tri-state.
//
pub const S626_EOS: c_uint = 0x00000001	/* End of superframe. */;
// I2C configuration constants.
pub const S626_I2C_CLKSEL: c_uint = 0x0400		/*;
// I2C bit rate =
// PCIclk/480 = 68.75 KHz.
//

// I2C bus data bit rate
// (determined by
// S626_I2C_CLKSEL) in KHz.
//

// Worst case time, in msec,
// for EEPROM internal write
// op.
//
// I2C manifest constants.
// Max retries to wait for EEPROM write.

pub const S626_I2C_ERR: c_uint = 0x0002	/* I2C control/status flag ERROR. */;
pub const S626_I2C_BUSY: c_uint = 0x0001	/* I2C control/status flag BUSY. */;
pub const S626_I2C_ABORT: c_uint = 0x0080	/* I2C status flag ABORT. */;
pub const S626_I2C_ATTRSTART: c_uint = 0x3	/* I2C attribute START. */;
pub const S626_I2C_ATTRCONT: c_uint = 0x2	/* I2C attribute CONT. */;
pub const S626_I2C_ATTRSTOP: c_uint = 0x1	/* I2C attribute STOP. */;
pub const S626_I2C_ATTRNOP: c_uint = 0x0	/* I2C attribute NOP. */;
// Code macros used for constructing I2C command bytes.

// DEBI command constants.

// Transfer size is always
// 2 bytes.
//
pub const S626_DEBI_CMD_READ: c_uint = 0x00010000	/* Read operation. */;
pub const S626_DEBI_CMD_WRITE: c_uint = 0x00000000	/* Write operation. */;
// Read immediate 2 bytes.

// Write immediate 2 bytes.

// DEBI configuration constants.
pub const S626_DEBI_CFG_XIRQ_EN: c_uint = 0x80000000	/*;
// Enable external interrupt
// on GPIO3.
//
pub const S626_DEBI_CFG_XRESUME: c_uint = 0x40000000	/* Resume block */;
//
// Transfer when XIRQ
// deasserted.
//
pub const S626_DEBI_CFG_TOQ: c_uint = 0x03C00000	/* Timeout (15 PCI cycles). */;
pub const S626_DEBI_CFG_FAST: c_uint = 0x10000000	/* Fast mode enable. */;
// 4-bit field that specifies DEBI timeout value in PCI clock cycles:

// Finish DEBI cycle after this many
// clocks.
//
// 2-bit field that specifies Endian byte lane steering:
pub const S626_DEBI_CFG_SWAP_NONE: c_uint = 0x00000000	/*;
// Straight - don't swap any
// bytes (Intel).
//
pub const S626_DEBI_CFG_SWAP_2: c_uint = 0x00100000	/* 2-byte swap (Motorola). */;
pub const S626_DEBI_CFG_SWAP_4: c_uint = 0x00200000	/* 4-byte swap. */;
pub const S626_DEBI_CFG_SLAVE16: c_uint = 0x00080000	/*;
// Slave is able to serve
// 16-bit cycles.
//
pub const S626_DEBI_CFG_INC: c_uint = 0x00040000	/*;
// Enable address increment
// for block transfers.
//
pub const S626_DEBI_CFG_INTEL: c_uint = 0x00020000	/* Intel style local bus. */;
pub const S626_DEBI_CFG_TIMEROFF: c_uint = 0x00010000	/* Disable timer. */;

// Wait 7 PCI clocks (212 ns) before
// polling RDY.
//
// Intel byte lane steering (pass through all byte lanes).

// Wait 15 PCI clocks (454 ns) maximum
// before timing out.
//
// Motorola byte lane steering.

// DEBI page table constants.
pub const S626_DEBI_PAGE_DISABLE: c_uint = 0x00000000	/* Paging disable. */;
// ******* EXTRA FROM OTHER SENSORAY  * .h  *******
// LoadSrc values:

// Preload core in response to
// Overflow.
//

// Preload B core in response to
// A Overflow.
//

// IntSrc values:

// LatchSrc values:

// IndxSrc values:

// IndxPol values:

// Logical encoder mode values:

// Physical CntSrc values (for Counter A source and Counter B source):

// ClkPol values:

// Counter/Extender clock is
// active high.
//

// Counter/Extender clock is
// active low.
//

// ClkEnab values:

// ClkMult values:

// Sanity-check limits for parameters.

// Maximum valid counter
// logical channel number.
//
pub const S626_NUM_INTSOURCES: c_int = 4;
pub const S626_NUM_LATCHSOURCES: c_int = 4;
pub const S626_NUM_CLKMULTS: c_int = 4;
pub const S626_NUM_CLKSOURCES: c_int = 4;
pub const S626_NUM_CLKPOLS: c_int = 2;
pub const S626_NUM_INDEXPOLS: c_int = 2;
pub const S626_NUM_INDEXSOURCES: c_int = 2;
pub const S626_NUM_LOADTRIGS: c_int = 4;
// General macros for manipulating bitfields:

// Bit field positions in CRA:

// Bit field widths in CRA:
pub const S626_CRAWID_INDXSRC_B: c_int = 2;
pub const S626_CRAWID_CNTSRC_B: c_int = 2;
pub const S626_CRAWID_INDXPOL_A: c_int = 1;
pub const S626_CRAWID_LOADSRC_A: c_int = 2;
pub const S626_CRAWID_CLKMULT_A: c_int = 2;
pub const S626_CRAWID_INTSRC_A: c_int = 2;
pub const S626_CRAWID_CLKPOL_A: c_int = 1;
pub const S626_CRAWID_INDXSRC_A: c_int = 2;
pub const S626_CRAWID_CNTSRC_A: c_int = 2;
// Bit field masks for CRA:

// Construct parts of the CRA value:

// Extract parts of the CRA value:

// Bit field positions in CRB:

// Bit field widths in CRB:
pub const S626_CRBWID_INTRESETCMD: c_int = 1;
pub const S626_CRBWID_CNTDIR_B: c_int = 1;
pub const S626_CRBWID_INTRESET_B: c_int = 1;
pub const S626_CRBWID_OVERDO_A: c_int = 1;
pub const S626_CRBWID_INTRESET_A: c_int = 1;
pub const S626_CRBWID_OVERDO_B: c_int = 1;
pub const S626_CRBWID_CLKENAB_A: c_int = 1;
pub const S626_CRBWID_INTSRC_B: c_int = 2;
pub const S626_CRBWID_LATCHSRC: c_int = 2;
pub const S626_CRBWID_LOADSRC_B: c_int = 2;
pub const S626_CRBWID_CLEAR_B: c_int = 1;
pub const S626_CRBWID_CLKMULT_B: c_int = 2;
pub const S626_CRBWID_CLKENAB_B: c_int = 1;
pub const S626_CRBWID_INDXPOL_B: c_int = 1;
pub const S626_CRBWID_CLKPOL_B: c_int = 1;
// Bit field masks for CRB:

// Interrupt reset control bits.

// Construct parts of the CRB value:

// Extract parts of the CRB value:

// Bit field positions for standardized SETUP structure:
pub const S626_STDBIT_INTSRC: c_int = 13;
pub const S626_STDBIT_LATCHSRC: c_int = 11;
pub const S626_STDBIT_LOADSRC: c_int = 9;
pub const S626_STDBIT_INDXSRC: c_int = 7;
pub const S626_STDBIT_INDXPOL: c_int = 6;
pub const S626_STDBIT_ENCMODE: c_int = 4;
pub const S626_STDBIT_CLKPOL: c_int = 3;
pub const S626_STDBIT_CLKMULT: c_int = 1;
pub const S626_STDBIT_CLKENAB: c_int = 0;
// Bit field widths for standardized SETUP structure:
pub const S626_STDWID_INTSRC: c_int = 2;
pub const S626_STDWID_LATCHSRC: c_int = 2;
pub const S626_STDWID_LOADSRC: c_int = 2;
pub const S626_STDWID_INDXSRC: c_int = 2;
pub const S626_STDWID_INDXPOL: c_int = 1;
pub const S626_STDWID_ENCMODE: c_int = 2;
pub const S626_STDWID_CLKPOL: c_int = 1;
pub const S626_STDWID_CLKMULT: c_int = 2;
pub const S626_STDWID_CLKENAB: c_int = 1;
// Bit field masks for standardized SETUP structure:

// Construct parts of standardized SETUP structure:

// Extract parts of standardized SETUP structure:

