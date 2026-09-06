//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/cpm2.h
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
// Communication Processor Module v2.
//
// This file contains structures and information for the communication
// processor channels found in the dual port RAM or parameter RAM.
// All CPM control and status is available through the CPM2 internal
// memory map.  See immap_cpm2.h for details.
//

// CPM Command register.
//

// Device sub-block and page codes.
//

// CPM2-specific opcodes (see cpm.h for common opcodes)
//

// The number of pages of host memory we allocate for CPM.  This is
// done early in kernel initialization to get physically contiguous
// pages.
//
pub const NUM_CPM_HOST_PAGES: c_int = 2;
// Export the base address of the communication processor registers
// and dual port ram.
//
extern "C" {
    pub fn cpm2_reset();
}
// Baud rate generators.
//

extern "C" {
    pub fn __cpm2_setbrg(brg: c_uint, rate: c_uint, clk: c_uint, div16: c_int, src: c_int);
}
// This function is used by UARTS, or anything else that uses a 16x
// oversampled clock.
//
// This function is used to set high speed synchronous baud rate
// clocks.
//
// Parameter RAM offsets from the base.
//

// The SMCs are relocated to any of the first eight DPRAM pages.
// We will fix these at the first locations of DPRAM, until we
// get some microcode patches :-).
// The parameter ram space for the SMCs is fifty-some bytes, and
// they are required to start on a 64 byte boundary.
//

// Define enough so I can at least use the serial port as a UART.
//
// SMC uart mode register (Internal memory map).
//

// SMC Event and Mask register.
//

// SCCs.
//

// SCC Event and Mask register.
//

// Function code bits.
//

// CPM Ethernet through SCC1.
//
// NOTE: Some versions of the manual have the following items
// incorrectly documented.  Below is the proper order.
//
// SCC Event register as used by Ethernet.
//

// SCC Mode Register (PSMR) as used by Ethernet.
//

// SCC as UART
//
// SCC Event and Mask registers when it is used as a UART.
//

// The SCC PSMR when used as a UART.
//

// CPM Transparent mode SCC.
//
// How about some FCCs.....
//

// Generic FCC parameter ram.
//
// Ethernet controller through FCC.
//
// FCC Event/Mask register as used by Ethernet.
//

// FCC Mode Register (FPSMR) as used by Ethernet.
//

// IIC parameter RAM.
//
// IDMA parameter RAM
//
// DMA channel mode bit fields
//

// IDMA Buffer Descriptors
//
// IDMA buffer descriptor flag bit fields
//

// per-channel IDMA registers
//
// IDMA event register bit fields
//

// RISC Controller Configuration Register (RCCR) bit fields
//

// -----------------------------------------------------------------------
// CMXFCR - CMX FCC Clock Route Register
//
pub const CMXFCR_FC1: c_uint = 0x40000000   /* FCC1 connection              */;
pub const CMXFCR_RF1CS_MSK: c_uint = 0x38000000   /* Receive FCC1 Clock Source Mask */;
pub const CMXFCR_TF1CS_MSK: c_uint = 0x07000000   /* Transmit FCC1 Clock Source Mask */;
pub const CMXFCR_FC2: c_uint = 0x00400000   /* FCC2 connection              */;
pub const CMXFCR_RF2CS_MSK: c_uint = 0x00380000   /* Receive FCC2 Clock Source Mask */;
pub const CMXFCR_TF2CS_MSK: c_uint = 0x00070000   /* Transmit FCC2 Clock Source Mask */;
pub const CMXFCR_FC3: c_uint = 0x00004000   /* FCC3 connection              */;
pub const CMXFCR_RF3CS_MSK: c_uint = 0x00003800   /* Receive FCC3 Clock Source Mask */;
pub const CMXFCR_TF3CS_MSK: c_uint = 0x00000700   /* Transmit FCC3 Clock Source Mask */;
pub const CMXFCR_RF1CS_BRG5: c_uint = 0x00000000   /* Receive FCC1 Clock Source is BRG5 */;
pub const CMXFCR_RF1CS_BRG6: c_uint = 0x08000000   /* Receive FCC1 Clock Source is BRG6 */;
pub const CMXFCR_RF1CS_BRG7: c_uint = 0x10000000   /* Receive FCC1 Clock Source is BRG7 */;
pub const CMXFCR_RF1CS_BRG8: c_uint = 0x18000000   /* Receive FCC1 Clock Source is BRG8 */;
pub const CMXFCR_RF1CS_CLK9: c_uint = 0x20000000   /* Receive FCC1 Clock Source is CLK9 */;
pub const CMXFCR_RF1CS_CLK10: c_uint = 0x28000000   /* Receive FCC1 Clock Source is CLK10 */;
pub const CMXFCR_RF1CS_CLK11: c_uint = 0x30000000   /* Receive FCC1 Clock Source is CLK11 */;
pub const CMXFCR_RF1CS_CLK12: c_uint = 0x38000000   /* Receive FCC1 Clock Source is CLK12 */;
pub const CMXFCR_TF1CS_BRG5: c_uint = 0x00000000   /* Transmit FCC1 Clock Source is BRG5 */;
pub const CMXFCR_TF1CS_BRG6: c_uint = 0x01000000   /* Transmit FCC1 Clock Source is BRG6 */;
pub const CMXFCR_TF1CS_BRG7: c_uint = 0x02000000   /* Transmit FCC1 Clock Source is BRG7 */;
pub const CMXFCR_TF1CS_BRG8: c_uint = 0x03000000   /* Transmit FCC1 Clock Source is BRG8 */;
pub const CMXFCR_TF1CS_CLK9: c_uint = 0x04000000   /* Transmit FCC1 Clock Source is CLK9 */;
pub const CMXFCR_TF1CS_CLK10: c_uint = 0x05000000   /* Transmit FCC1 Clock Source is CLK10 */;
pub const CMXFCR_TF1CS_CLK11: c_uint = 0x06000000   /* Transmit FCC1 Clock Source is CLK11 */;
pub const CMXFCR_TF1CS_CLK12: c_uint = 0x07000000   /* Transmit FCC1 Clock Source is CLK12 */;
pub const CMXFCR_RF2CS_BRG5: c_uint = 0x00000000   /* Receive FCC2 Clock Source is BRG5 */;
pub const CMXFCR_RF2CS_BRG6: c_uint = 0x00080000   /* Receive FCC2 Clock Source is BRG6 */;
pub const CMXFCR_RF2CS_BRG7: c_uint = 0x00100000   /* Receive FCC2 Clock Source is BRG7 */;
pub const CMXFCR_RF2CS_BRG8: c_uint = 0x00180000   /* Receive FCC2 Clock Source is BRG8 */;
pub const CMXFCR_RF2CS_CLK13: c_uint = 0x00200000   /* Receive FCC2 Clock Source is CLK13 */;
pub const CMXFCR_RF2CS_CLK14: c_uint = 0x00280000   /* Receive FCC2 Clock Source is CLK14 */;
pub const CMXFCR_RF2CS_CLK15: c_uint = 0x00300000   /* Receive FCC2 Clock Source is CLK15 */;
pub const CMXFCR_RF2CS_CLK16: c_uint = 0x00380000   /* Receive FCC2 Clock Source is CLK16 */;
pub const CMXFCR_TF2CS_BRG5: c_uint = 0x00000000   /* Transmit FCC2 Clock Source is BRG5 */;
pub const CMXFCR_TF2CS_BRG6: c_uint = 0x00010000   /* Transmit FCC2 Clock Source is BRG6 */;
pub const CMXFCR_TF2CS_BRG7: c_uint = 0x00020000   /* Transmit FCC2 Clock Source is BRG7 */;
pub const CMXFCR_TF2CS_BRG8: c_uint = 0x00030000   /* Transmit FCC2 Clock Source is BRG8 */;
pub const CMXFCR_TF2CS_CLK13: c_uint = 0x00040000   /* Transmit FCC2 Clock Source is CLK13 */;
pub const CMXFCR_TF2CS_CLK14: c_uint = 0x00050000   /* Transmit FCC2 Clock Source is CLK14 */;
pub const CMXFCR_TF2CS_CLK15: c_uint = 0x00060000   /* Transmit FCC2 Clock Source is CLK15 */;
pub const CMXFCR_TF2CS_CLK16: c_uint = 0x00070000   /* Transmit FCC2 Clock Source is CLK16 */;
pub const CMXFCR_RF3CS_BRG5: c_uint = 0x00000000   /* Receive FCC3 Clock Source is BRG5 */;
pub const CMXFCR_RF3CS_BRG6: c_uint = 0x00000800   /* Receive FCC3 Clock Source is BRG6 */;
pub const CMXFCR_RF3CS_BRG7: c_uint = 0x00001000   /* Receive FCC3 Clock Source is BRG7 */;
pub const CMXFCR_RF3CS_BRG8: c_uint = 0x00001800   /* Receive FCC3 Clock Source is BRG8 */;
pub const CMXFCR_RF3CS_CLK13: c_uint = 0x00002000   /* Receive FCC3 Clock Source is CLK13 */;
pub const CMXFCR_RF3CS_CLK14: c_uint = 0x00002800   /* Receive FCC3 Clock Source is CLK14 */;
pub const CMXFCR_RF3CS_CLK15: c_uint = 0x00003000   /* Receive FCC3 Clock Source is CLK15 */;
pub const CMXFCR_RF3CS_CLK16: c_uint = 0x00003800   /* Receive FCC3 Clock Source is CLK16 */;
pub const CMXFCR_TF3CS_BRG5: c_uint = 0x00000000   /* Transmit FCC3 Clock Source is BRG5 */;
pub const CMXFCR_TF3CS_BRG6: c_uint = 0x00000100   /* Transmit FCC3 Clock Source is BRG6 */;
pub const CMXFCR_TF3CS_BRG7: c_uint = 0x00000200   /* Transmit FCC3 Clock Source is BRG7 */;
pub const CMXFCR_TF3CS_BRG8: c_uint = 0x00000300   /* Transmit FCC3 Clock Source is BRG8 */;
pub const CMXFCR_TF3CS_CLK13: c_uint = 0x00000400   /* Transmit FCC3 Clock Source is CLK13 */;
pub const CMXFCR_TF3CS_CLK14: c_uint = 0x00000500   /* Transmit FCC3 Clock Source is CLK14 */;
pub const CMXFCR_TF3CS_CLK15: c_uint = 0x00000600   /* Transmit FCC3 Clock Source is CLK15 */;
pub const CMXFCR_TF3CS_CLK16: c_uint = 0x00000700   /* Transmit FCC3 Clock Source is CLK16 */;
// -----------------------------------------------------------------------
// CMXSCR - CMX SCC Clock Route Register
//
pub const CMXSCR_GR1: c_uint = 0x80000000   /* Grant Support of SCC1        */;
pub const CMXSCR_SC1: c_uint = 0x40000000   /* SCC1 connection              */;
pub const CMXSCR_RS1CS_MSK: c_uint = 0x38000000   /* Receive SCC1 Clock Source Mask */;
pub const CMXSCR_TS1CS_MSK: c_uint = 0x07000000   /* Transmit SCC1 Clock Source Mask */;
pub const CMXSCR_GR2: c_uint = 0x00800000   /* Grant Support of SCC2        */;
pub const CMXSCR_SC2: c_uint = 0x00400000   /* SCC2 connection              */;
pub const CMXSCR_RS2CS_MSK: c_uint = 0x00380000   /* Receive SCC2 Clock Source Mask */;
pub const CMXSCR_TS2CS_MSK: c_uint = 0x00070000   /* Transmit SCC2 Clock Source Mask */;
pub const CMXSCR_GR3: c_uint = 0x00008000   /* Grant Support of SCC3        */;
pub const CMXSCR_SC3: c_uint = 0x00004000   /* SCC3 connection              */;
pub const CMXSCR_RS3CS_MSK: c_uint = 0x00003800   /* Receive SCC3 Clock Source Mask */;
pub const CMXSCR_TS3CS_MSK: c_uint = 0x00000700   /* Transmit SCC3 Clock Source Mask */;
pub const CMXSCR_GR4: c_uint = 0x00000080   /* Grant Support of SCC4        */;
pub const CMXSCR_SC4: c_uint = 0x00000040   /* SCC4 connection              */;
pub const CMXSCR_RS4CS_MSK: c_uint = 0x00000038   /* Receive SCC4 Clock Source Mask */;
pub const CMXSCR_TS4CS_MSK: c_uint = 0x00000007   /* Transmit SCC4 Clock Source Mask */;
pub const CMXSCR_RS1CS_BRG1: c_uint = 0x00000000   /* SCC1 Rx Clock Source is BRG1 */;
pub const CMXSCR_RS1CS_BRG2: c_uint = 0x08000000   /* SCC1 Rx Clock Source is BRG2 */;
pub const CMXSCR_RS1CS_BRG3: c_uint = 0x10000000   /* SCC1 Rx Clock Source is BRG3 */;
pub const CMXSCR_RS1CS_BRG4: c_uint = 0x18000000   /* SCC1 Rx Clock Source is BRG4 */;
pub const CMXSCR_RS1CS_CLK11: c_uint = 0x20000000   /* SCC1 Rx Clock Source is CLK11 */;
pub const CMXSCR_RS1CS_CLK12: c_uint = 0x28000000   /* SCC1 Rx Clock Source is CLK12 */;
pub const CMXSCR_RS1CS_CLK3: c_uint = 0x30000000   /* SCC1 Rx Clock Source is CLK3 */;
pub const CMXSCR_RS1CS_CLK4: c_uint = 0x38000000   /* SCC1 Rx Clock Source is CLK4 */;
pub const CMXSCR_TS1CS_BRG1: c_uint = 0x00000000   /* SCC1 Tx Clock Source is BRG1 */;
pub const CMXSCR_TS1CS_BRG2: c_uint = 0x01000000   /* SCC1 Tx Clock Source is BRG2 */;
pub const CMXSCR_TS1CS_BRG3: c_uint = 0x02000000   /* SCC1 Tx Clock Source is BRG3 */;
pub const CMXSCR_TS1CS_BRG4: c_uint = 0x03000000   /* SCC1 Tx Clock Source is BRG4 */;
pub const CMXSCR_TS1CS_CLK11: c_uint = 0x04000000   /* SCC1 Tx Clock Source is CLK11 */;
pub const CMXSCR_TS1CS_CLK12: c_uint = 0x05000000   /* SCC1 Tx Clock Source is CLK12 */;
pub const CMXSCR_TS1CS_CLK3: c_uint = 0x06000000   /* SCC1 Tx Clock Source is CLK3 */;
pub const CMXSCR_TS1CS_CLK4: c_uint = 0x07000000   /* SCC1 Tx Clock Source is CLK4 */;
pub const CMXSCR_RS2CS_BRG1: c_uint = 0x00000000   /* SCC2 Rx Clock Source is BRG1 */;
pub const CMXSCR_RS2CS_BRG2: c_uint = 0x00080000   /* SCC2 Rx Clock Source is BRG2 */;
pub const CMXSCR_RS2CS_BRG3: c_uint = 0x00100000   /* SCC2 Rx Clock Source is BRG3 */;
pub const CMXSCR_RS2CS_BRG4: c_uint = 0x00180000   /* SCC2 Rx Clock Source is BRG4 */;
pub const CMXSCR_RS2CS_CLK11: c_uint = 0x00200000   /* SCC2 Rx Clock Source is CLK11 */;
pub const CMXSCR_RS2CS_CLK12: c_uint = 0x00280000   /* SCC2 Rx Clock Source is CLK12 */;
pub const CMXSCR_RS2CS_CLK3: c_uint = 0x00300000   /* SCC2 Rx Clock Source is CLK3 */;
pub const CMXSCR_RS2CS_CLK4: c_uint = 0x00380000   /* SCC2 Rx Clock Source is CLK4 */;
pub const CMXSCR_TS2CS_BRG1: c_uint = 0x00000000   /* SCC2 Tx Clock Source is BRG1 */;
pub const CMXSCR_TS2CS_BRG2: c_uint = 0x00010000   /* SCC2 Tx Clock Source is BRG2 */;
pub const CMXSCR_TS2CS_BRG3: c_uint = 0x00020000   /* SCC2 Tx Clock Source is BRG3 */;
pub const CMXSCR_TS2CS_BRG4: c_uint = 0x00030000   /* SCC2 Tx Clock Source is BRG4 */;
pub const CMXSCR_TS2CS_CLK11: c_uint = 0x00040000   /* SCC2 Tx Clock Source is CLK11 */;
pub const CMXSCR_TS2CS_CLK12: c_uint = 0x00050000   /* SCC2 Tx Clock Source is CLK12 */;
pub const CMXSCR_TS2CS_CLK3: c_uint = 0x00060000   /* SCC2 Tx Clock Source is CLK3 */;
pub const CMXSCR_TS2CS_CLK4: c_uint = 0x00070000   /* SCC2 Tx Clock Source is CLK4 */;
pub const CMXSCR_RS3CS_BRG1: c_uint = 0x00000000   /* SCC3 Rx Clock Source is BRG1 */;
pub const CMXSCR_RS3CS_BRG2: c_uint = 0x00000800   /* SCC3 Rx Clock Source is BRG2 */;
pub const CMXSCR_RS3CS_BRG3: c_uint = 0x00001000   /* SCC3 Rx Clock Source is BRG3 */;
pub const CMXSCR_RS3CS_BRG4: c_uint = 0x00001800   /* SCC3 Rx Clock Source is BRG4 */;
pub const CMXSCR_RS3CS_CLK5: c_uint = 0x00002000   /* SCC3 Rx Clock Source is CLK5 */;
pub const CMXSCR_RS3CS_CLK6: c_uint = 0x00002800   /* SCC3 Rx Clock Source is CLK6 */;
pub const CMXSCR_RS3CS_CLK7: c_uint = 0x00003000   /* SCC3 Rx Clock Source is CLK7 */;
pub const CMXSCR_RS3CS_CLK8: c_uint = 0x00003800   /* SCC3 Rx Clock Source is CLK8 */;
pub const CMXSCR_TS3CS_BRG1: c_uint = 0x00000000   /* SCC3 Tx Clock Source is BRG1 */;
pub const CMXSCR_TS3CS_BRG2: c_uint = 0x00000100   /* SCC3 Tx Clock Source is BRG2 */;
pub const CMXSCR_TS3CS_BRG3: c_uint = 0x00000200   /* SCC3 Tx Clock Source is BRG3 */;
pub const CMXSCR_TS3CS_BRG4: c_uint = 0x00000300   /* SCC3 Tx Clock Source is BRG4 */;
pub const CMXSCR_TS3CS_CLK5: c_uint = 0x00000400   /* SCC3 Tx Clock Source is CLK5 */;
pub const CMXSCR_TS3CS_CLK6: c_uint = 0x00000500   /* SCC3 Tx Clock Source is CLK6 */;
pub const CMXSCR_TS3CS_CLK7: c_uint = 0x00000600   /* SCC3 Tx Clock Source is CLK7 */;
pub const CMXSCR_TS3CS_CLK8: c_uint = 0x00000700   /* SCC3 Tx Clock Source is CLK8 */;
pub const CMXSCR_RS4CS_BRG1: c_uint = 0x00000000   /* SCC4 Rx Clock Source is BRG1 */;
pub const CMXSCR_RS4CS_BRG2: c_uint = 0x00000008   /* SCC4 Rx Clock Source is BRG2 */;
pub const CMXSCR_RS4CS_BRG3: c_uint = 0x00000010   /* SCC4 Rx Clock Source is BRG3 */;
pub const CMXSCR_RS4CS_BRG4: c_uint = 0x00000018   /* SCC4 Rx Clock Source is BRG4 */;
pub const CMXSCR_RS4CS_CLK5: c_uint = 0x00000020   /* SCC4 Rx Clock Source is CLK5 */;
pub const CMXSCR_RS4CS_CLK6: c_uint = 0x00000028   /* SCC4 Rx Clock Source is CLK6 */;
pub const CMXSCR_RS4CS_CLK7: c_uint = 0x00000030   /* SCC4 Rx Clock Source is CLK7 */;
pub const CMXSCR_RS4CS_CLK8: c_uint = 0x00000038   /* SCC4 Rx Clock Source is CLK8 */;
pub const CMXSCR_TS4CS_BRG1: c_uint = 0x00000000   /* SCC4 Tx Clock Source is BRG1 */;
pub const CMXSCR_TS4CS_BRG2: c_uint = 0x00000001   /* SCC4 Tx Clock Source is BRG2 */;
pub const CMXSCR_TS4CS_BRG3: c_uint = 0x00000002   /* SCC4 Tx Clock Source is BRG3 */;
pub const CMXSCR_TS4CS_BRG4: c_uint = 0x00000003   /* SCC4 Tx Clock Source is BRG4 */;
pub const CMXSCR_TS4CS_CLK5: c_uint = 0x00000004   /* SCC4 Tx Clock Source is CLK5 */;
pub const CMXSCR_TS4CS_CLK6: c_uint = 0x00000005   /* SCC4 Tx Clock Source is CLK6 */;
pub const CMXSCR_TS4CS_CLK7: c_uint = 0x00000006   /* SCC4 Tx Clock Source is CLK7 */;
pub const CMXSCR_TS4CS_CLK8: c_uint = 0x00000007   /* SCC4 Tx Clock Source is CLK8 */;
// -----------------------------------------------------------------------
// SIUMCR - SIU Module Configuration Register				 4-31
//
pub const SIUMCR_BBD: c_uint = 0x80000000	/* Bus Busy Disable		*/;
pub const SIUMCR_ESE: c_uint = 0x40000000	/* External Snoop Enable	*/;
pub const SIUMCR_PBSE: c_uint = 0x20000000	/* Parity Byte Select Enable	*/;
pub const SIUMCR_CDIS: c_uint = 0x10000000	/* Core Disable			*/;
pub const SIUMCR_DPPC00: c_uint = 0x00000000	/* Data Parity Pins Configuration*/;
pub const SIUMCR_DPPC01: c_uint = 0x04000000	/* - " -			*/;
pub const SIUMCR_DPPC10: c_uint = 0x08000000	/* - " -			*/;
pub const SIUMCR_DPPC11: c_uint = 0x0c000000	/* - " -			*/;
pub const SIUMCR_L2CPC00: c_uint = 0x00000000	/* L2 Cache Pins Configuration	*/;
pub const SIUMCR_L2CPC01: c_uint = 0x01000000	/* - " -			*/;
pub const SIUMCR_L2CPC10: c_uint = 0x02000000	/* - " -			*/;
pub const SIUMCR_L2CPC11: c_uint = 0x03000000	/* - " -			*/;
pub const SIUMCR_LBPC00: c_uint = 0x00000000	/* Local Bus Pins Configuration	*/;
pub const SIUMCR_LBPC01: c_uint = 0x00400000	/* - " -			*/;
pub const SIUMCR_LBPC10: c_uint = 0x00800000	/* - " -			*/;
pub const SIUMCR_LBPC11: c_uint = 0x00c00000	/* - " -			*/;
pub const SIUMCR_APPC00: c_uint = 0x00000000	/* Address Parity Pins Configuration*/;
pub const SIUMCR_APPC01: c_uint = 0x00100000	/* - " -			*/;
pub const SIUMCR_APPC10: c_uint = 0x00200000	/* - " -			*/;
pub const SIUMCR_APPC11: c_uint = 0x00300000	/* - " -			*/;
pub const SIUMCR_CS10PC00: c_uint = 0x00000000	/* CS10 Pin Configuration	*/;
pub const SIUMCR_CS10PC01: c_uint = 0x00040000	/* - " -			*/;
pub const SIUMCR_CS10PC10: c_uint = 0x00080000	/* - " -			*/;
pub const SIUMCR_CS10PC11: c_uint = 0x000c0000	/* - " -			*/;
pub const SIUMCR_BCTLC00: c_uint = 0x00000000	/* Buffer Control Configuration	*/;
pub const SIUMCR_BCTLC01: c_uint = 0x00010000	/* - " -			*/;
pub const SIUMCR_BCTLC10: c_uint = 0x00020000	/* - " -			*/;
pub const SIUMCR_BCTLC11: c_uint = 0x00030000	/* - " -			*/;
pub const SIUMCR_MMR00: c_uint = 0x00000000	/* Mask Masters Requests	*/;
pub const SIUMCR_MMR01: c_uint = 0x00004000	/* - " -			*/;
pub const SIUMCR_MMR10: c_uint = 0x00008000	/* - " -			*/;
pub const SIUMCR_MMR11: c_uint = 0x0000c000	/* - " -			*/;
pub const SIUMCR_LPBSE: c_uint = 0x00002000	/* LocalBus Parity Byte Select Enable*/;
// -----------------------------------------------------------------------
// SCCR - System Clock Control Register					 9-8
//
pub const SCCR_PCI_MODE: c_uint = 0x00000100	/* PCI Mode	*/;
pub const SCCR_PCI_MODCK: c_uint = 0x00000080	/* Value of PCI_MODCK pin	*/;
pub const SCCR_PCIDF_MSK: c_uint = 0x00000078	/* PCI division factor	*/;
pub const SCCR_PCIDF_SHIFT: c_int = 3;

pub const CPM_IMMR_OFFSET: c_uint = 0x101a8;

// FCC iop & clock configuration. BSP code is responsible to define Fx_RXCLK & Fx_TXCLK
// in order to use clock-computing stuff below for the FCC x
//
// Automatically generates register configurations

// I/O Pin assignment for FCC1.  I don't yet know the best way to do this,
// but there is little variation among the choices.
//
pub const PA1_COL: c_uint = 0x00000001U;
pub const PA1_CRS: c_uint = 0x00000002U;
pub const PA1_TXER: c_uint = 0x00000004U;
pub const PA1_TXEN: c_uint = 0x00000008U;
pub const PA1_RXDV: c_uint = 0x00000010U;
pub const PA1_RXER: c_uint = 0x00000020U;
pub const PA1_TXDAT: c_uint = 0x00003c00U;
pub const PA1_RXDAT: c_uint = 0x0003c000U;

// I/O Pin assignment for FCC2.  I don't yet know the best way to do this,
// but there is little variation among the choices.
//
pub const PB2_TXER: c_uint = 0x00000001U;
pub const PB2_RXDV: c_uint = 0x00000002U;
pub const PB2_TXEN: c_uint = 0x00000004U;
pub const PB2_RXER: c_uint = 0x00000008U;
pub const PB2_COL: c_uint = 0x00000010U;
pub const PB2_CRS: c_uint = 0x00000020U;
pub const PB2_TXDAT: c_uint = 0x000003c0U;
pub const PB2_RXDAT: c_uint = 0x00003c00U;

// I/O Pin assignment for FCC3.  I don't yet know the best way to do this,
// but there is little variation among the choices.
//
pub const PB3_RXDV: c_uint = 0x00004000U;
pub const PB3_RXER: c_uint = 0x00008000U;
pub const PB3_TXER: c_uint = 0x00010000U;
pub const PB3_TXEN: c_uint = 0x00020000U;
pub const PB3_COL: c_uint = 0x00040000U;
pub const PB3_CRS: c_uint = 0x00080000U;
pub const PB3_TXDAT: c_uint = 0x0f000000U;
pub const PC3_TXDAT: c_uint = 0x00000010U;
pub const PB3_RXDAT: c_uint = 0x00f00000U;

pub const PB3_PSORB1: c_int = 0;

// Handy macro to specify mem for FCCs

// Pipeline Maximum Depth
pub const MPC82XX_BCR_PLDP: c_uint = 0x00800000;
// Clocks and GRG's
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cpm_clk_dir {
    CPM_CLK_RX,
    CPM_CLK_TX,
    CPM_CLK_RTX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cpm_clk_target {
    CPM_CLK_SCC1,
    CPM_CLK_SCC2,
    CPM_CLK_SCC3,
    CPM_CLK_SCC4,
    CPM_CLK_FCC1,
    CPM_CLK_FCC2,
    CPM_CLK_FCC3,
    CPM_CLK_SMC1,
    CPM_CLK_SMC2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cpm_clk {
    CPM_CLK_NONE = 0,
    CPM_BRG1,	/* Baud Rate Generator  1 */
    CPM_BRG2,	/* Baud Rate Generator  2 */
    CPM_BRG3,	/* Baud Rate Generator  3 */
    CPM_BRG4,	/* Baud Rate Generator  4 */
    CPM_BRG5,	/* Baud Rate Generator  5 */
    CPM_BRG6,	/* Baud Rate Generator  6 */
    CPM_BRG7,	/* Baud Rate Generator  7 */
    CPM_BRG8,	/* Baud Rate Generator  8 */
    CPM_CLK1,	/* Clock  1 */
    CPM_CLK2,	/* Clock  2 */
    CPM_CLK3,	/* Clock  3 */
    CPM_CLK4,	/* Clock  4 */
    CPM_CLK5,	/* Clock  5 */
    CPM_CLK6,	/* Clock  6 */
    CPM_CLK7,	/* Clock  7 */
    CPM_CLK8,	/* Clock  8 */
    CPM_CLK9,	/* Clock  9 */
    CPM_CLK10,	/* Clock 10 */
    CPM_CLK11,	/* Clock 11 */
    CPM_CLK12,	/* Clock 12 */
    CPM_CLK13,	/* Clock 13 */
    CPM_CLK14,	/* Clock 14 */
    CPM_CLK15,	/* Clock 15 */
    CPM_CLK16,	/* Clock 16 */
    CPM_CLK17,	/* Clock 17 */
    CPM_CLK18,	/* Clock 18 */
    CPM_CLK19,	/* Clock 19 */
    CPM_CLK20,	/* Clock 20 */
    CPM_CLK_DUMMY
}

extern "C" {
    pub fn cpm2_clk_setup(target: cpm_clk_target, clock: c_int, mode: c_int) -> int __init;
}
extern "C" {
    pub fn cpm2_smc_clk_setup(target: cpm_clk_target, clock: c_int) -> int __init;
}
pub const CPM_PIN_INPUT: c_int = 0;
pub const CPM_PIN_OUTPUT: c_int = 1;
pub const CPM_PIN_PRIMARY: c_int = 0;
pub const CPM_PIN_SECONDARY: c_int = 2;
pub const CPM_PIN_GPIO: c_int = 4;
pub const CPM_PIN_OPENDRAIN: c_int = 8;
extern "C" {
    pub fn cpm2_set_pin(port: c_int, pin: c_int, flags: c_int) -> void __init;
}

