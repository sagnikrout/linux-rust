//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/comedi/drivers/plx9080.h
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
// plx9080.h
//
// Copyright (C) 2002,2003 Frank Mori Hess <fmhess@users.sourceforge.net>
//
// Copyright (C) 1999 RG Studio s.c.
// Written by Krzysztof Halasa <khc@rgstudio.com.pl>
//
// Portions (C) SBE Inc., used by permission.
//

//
// struct plx_dma_desc - DMA descriptor format for PLX PCI 9080
// @pci_start_addr:	PCI Bus address for transfer (DMAPADR).
// @local_start_addr:	Local Bus address for transfer (DMALADR).
// @transfer_size:	Transfer size in bytes (max 8 MiB) (DMASIZ).
// @next:		Address of next descriptor + flags (DMADPR).
//
// Describes the format of a scatter-gather DMA descriptor for the PLX
// PCI 9080.  All members are raw, little-endian register values that
// will be transferred by the DMA engine from local or PCI memory into
// corresponding registers for the DMA channel.
//
// The DMA descriptors must be aligned on a 16-byte boundary.  Bits 3:0
// of @next contain flags describing the address space of the next
// descriptor (local or PCI), an "end of chain" marker, an "interrupt on
// terminal count" bit, and a data transfer direction.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct plx_dma_desc {
    pub pci_start_addr: __le32,
    pub local_start_addr: __le32,
    pub transfer_size: __le32,
    pub next: __le32,
}

//
// Register Offsets and Bit Definitions
//
// Local Address Space 0 Range Register
pub const PLX_REG_LAS0RR: c_uint = 0x0000;
// Local Address Space 1 Range Register
pub const PLX_REG_LAS1RR: c_uint = 0x00f0;

// bits that specify range for memory space decode bits

// bits that specify range for i/o space decode bits

// Local Address Space 0 Local Base Address (Remap) Register
pub const PLX_REG_LAS0BA: c_uint = 0x0004;
// Local Address Space 1 Local Base Address (Remap) Register
pub const PLX_REG_LAS1BA: c_uint = 0x00f4;

// bits that specify local base address for memory space

// bits that specify local base address for i/o space

// Mode/Arbitration Register
pub const PLX_REG_MARBR: c_uint = 0x0008;
// DMA Arbitration Register (alias of MARBR).
pub const PLX_REG_DMAARB: c_uint = 0x00ac;
// Local Bus Latency Timer

// Local Bus Pause Timer

// Local Bus Latency Timer Enable

// Local Bus Pause Timer Enable

// Local Bus BREQ Enable

// DMA Channel Priority

// Local Bus Direct Slave Give Up Bus Mode

// Direct Slace LLOCKo# Enable

// PCI Request Mode

// PCI Specification v2.1 Mode

// PCI Read No Write Mode

// PCI Read with Write Flush Mode

// Gate Local Bus Latency Timer with BREQ

// PCI Read No Flush Mode

//
// Make reads from PCI Configuration register 0 return Subsystem ID and
// Subsystem Vendor ID instead of Device ID and Vendor ID
//

// Big/Little Endian Descriptor Register
pub const PLX_REG_BIGEND: c_uint = 0x000c;
// Configuration Register Big Endian Mode

// Direct Master Big Endian Mode

// Direct Slave Address Space 0 Big Endian Mode

// Direct Slave Expansion ROM Big Endian Mode

// Big Endian Byte Lane Mode - use most significant byte lanes

// Direct Slave Address Space 1 Big Endian Mode

// DMA Channel 1 Big Endian Mode

// DMA Channel 0 Big Endian Mode

// DMA Channel N Big Endian Mode (N <= 1)

//
// Note: The Expansion ROM  stuff is only relevant to the PC environment.
// This expansion ROM code is executed by the host CPU at boot time.
// For this reason no bit definitions are provided here.
//
// Expansion ROM Range Register
pub const PLX_REG_EROMRR: c_uint = 0x0010;
// Expansion ROM Local Base Address (Remap) Register
pub const PLX_REG_EROMBA: c_uint = 0x0014;
// Local Address Space 0/Expansion ROM Bus Region Descriptor Register
pub const PLX_REG_LBRD0: c_uint = 0x0018;
// Local Address Space 1 Bus Region Descriptor Register
pub const PLX_REG_LBRD1: c_uint = 0x00f8;
// Memory Space Local Bus Width

// Memory Space Internal Wait States

// Memory Space Ready Input Enable

// Memory Space BTERM# Input Enable

// Memory Space 0 Prefetch Disable (LBRD0 only)

// Memory Space 1 Burst Enable (LBRD1 only)

// Expansion ROM Space Prefetch Disable (LBRD0 only)

// Memory Space 1 Prefetch Disable (LBRD1 only)

// Read Prefetch Count Enable

// Prefetch Counter

// Expansion ROM Space Local Bus Width (LBRD0 only)

// Expansion ROM Space Internal Wait States (LBRD0 only)

// Expansion ROM Space Ready Input Enable (LBDR0 only)

// Expansion ROM Space BTERM# Input Enable (LBRD0 only)

// Memory Space 0 Burst Enable (LBRD0 only)

// Extra Long Load From Serial EEPROM  (LBRD0 only)

// Expansion ROM Space Burst Enable (LBRD0 only)

// Direct Slave PCI Write Mode - assert TRDY# when FIFO full (LBRD0 only)

// PCI Target Retry Delay Clocks / 8 (LBRD0 only)

// Local Range Register for Direct Master to PCI
pub const PLX_REG_DMRR: c_uint = 0x001c;
// Local Bus Base Address Register for Direct Master to PCI Memory
pub const PLX_REG_DMLBAM: c_uint = 0x0020;
// Local Base Address Register for Direct Master to PCI IO/CFG
pub const PLX_REG_DMLBAI: c_uint = 0x0024;
// PCI Base Address (Remap) Register for Direct Master to PCI Memory
pub const PLX_REG_DMPBAM: c_uint = 0x0028;
// Direct Master Memory Access Enable

// Direct Master I/O Access Enable

// LLOCK# Input Enable

// Direct Master Read Prefetch Size Control (bits 12, 3)

// Direct Master PCI Read Mode - deassert IRDY when FIFO full

// Programmable Almost Full Level (bits 10, 8:5)

// Write And Invalidate Mode

// Direct Master Prefetch Limit

// I/O Remap Select

// Direct Master Write Delay

// Remap of Local-to-PCI Space Into PCI Address Space

// PCI Configuration Address Register for Direct Master to PCI IO/CFG
pub const PLX_REG_DMCFGA: c_uint = 0x002c;
// Congiguration Type

// Register Number

// Function Number

// Device Number

// Bus Number

// Configuration Enable

//
// Mailbox Register N (N <= 7)
//
// Note that if the I2O feature is enabled (QSR[0] is set), Mailbox Register 0
// is replaced by the Inbound Queue Port, and Mailbox Register 1 is replaced
// by the Outbound Queue Port.  However, Mailbox Register 0 and 1 are always
// accessible at alternative offsets if the I2O feature is enabled.
//

// Alternative offsets for Mailbox Registers 0 and 1 (in case I2O is enabled)

// PCI-to-Local Doorbell Register
pub const PLX_REG_P2LDBELL: c_uint = 0x0060;
// Local-to-PCI Doorbell Register
pub const PLX_REG_L2PDBELL: c_uint = 0x0064;
// Interrupt Control/Status Register
pub const PLX_REG_INTCSR: c_uint = 0x0068;
// Enable Local Bus LSERR# when PCI Bus Target Abort or Master Abort occurs

// Enable Local Bus LSERR# when PCI parity error occurs

// Generate PCI Bus SERR# when set to 1

// Mailbox Interrupt Enable (local bus interrupts on PCI write to MBOX0-3)

// PCI Interrupt Enable

// PCI Doorbell Interrupt Enable

// PCI Abort Interrupt Enable

// PCI Local Interrupt Enable

// Retry Abort Enable (for diagnostic purposes only)

// PCI Doorbell Interrupt Active (read-only)

// PCI Abort Interrupt Active (read-only)

// Local Interrupt (LINTi#) Active (read-only)

// Local Interrupt Output (LINTo#) Enable

// Local Doorbell Interrupt Enable

// DMA Channel 0 Interrupt Enable

// DMA Channel 1 Interrupt Enable

// DMA Channel N Interrupt Enable (N <= 1)

// Local Doorbell Interrupt Active (read-only)

// DMA Channel 0 Interrupt Active (read-only)

// DMA Channel 1 Interrupt Active (read-only)

// DMA Channel N Interrupt Active (N <= 1) (read-only)

// BIST Interrupt Active (read-only)

// Direct Master Not Bus Master During Master Or Target Abort (read-only)

// DMA Channel 0 Not Bus Master During Master Or Target Abort (read-only)

// DMA Channel 1 Not Bus Master During Master Or Target Abort (read-only)

// DMA Channel N Not Bus Master During Master Or Target Abort (read-only)

// Target Abort Not Generated After 256 Master Retries (read-only)

// PCI Wrote Mailbox 0 (enabled if bit 3 set) (read-only)

// PCI Wrote Mailbox 1 (enabled if bit 3 set) (read-only)

// PCI Wrote Mailbox 2 (enabled if bit 3 set) (read-only)

// PCI Wrote Mailbox 3 (enabled if bit 3 set) (read-only)

// PCI Wrote Mailbox N (N <= 3) (enabled if bit 3 set) (read-only)

//
// Serial EEPROM Control, PCI Command Codes, User I/O Control,
// Init Control Register
//
pub const PLX_REG_CNTRL: c_uint = 0x006c;
// PCI Read Command Code For DMA

// PCI Write Command Code For DMA 0

// PCI Memory Read Command Code For Direct Master

// PCI Memory Write Command Code For Direct Master

// General Purpose Output (USERO)

// General Purpose Input (USERI) (read-only)

// Serial EEPROM Clock Output (EESK)

// Serial EEPROM Chip Select Output (EECS)

// Serial EEPROM Data Write Bit (EEDI (sic))

// Serial EEPROM Data Read Bit (EEDO (sic)) (read-only)

// Serial EEPROM Present (read-only)

// Reload Configuration Registers from EEPROM

// PCI Adapter Software Reset (asserts LRESETo#)

// Local Init Status (read-only)

//
// Combined command code stuff for convenience.
//

// PCI Permanent Configuration ID Register (hard-coded PLX vendor and device)
pub const PLX_REG_PCIHIDR: c_uint = 0x0070;
// Hard-coded ID for PLX PCI 9080
pub const PLX_PCIHIDR_9080: c_uint = 0x908010b5;
// PCI Permanent Revision ID Register (hard-coded silicon revision) (8-bit).
pub const PLX_REG_PCIHREV: c_uint = 0x0074;
// DMA Channel N Mode Register (N <= 1)

pub const PLX_REG_DMAMODE0: c_uint = 0x0080;
pub const PLX_REG_DMAMODE1: c_uint = 0x0094;
// Local Bus Width

// Internal Wait States

// Ready Input Enable

// BTERM# Input Enable

// Local Burst Enable

// Chaining Enable

// Done Interrupt Enable

// Hold Local Address Constant

// Demand Mode

// Write And Invalidate Mode

// DMA EOT Enable - enables EOT0# or EOT1# input pin

// DMA Stop Data Transfer Mode - 0:BLAST; 1:EOT asserted or DREQ deasserted

// DMA Clear Count Mode - count in descriptor cleared on completion

// DMA Channel Interrupt Select - 0:local bus interrupt; 1:PCI interrupt

// DMA Channel N PCI Address Register (N <= 1)

pub const PLX_REG_DMAPADR0: c_uint = 0x0084;
pub const PLX_REG_DMAPADR1: c_uint = 0x0098;
// DMA Channel N Local Address Register (N <= 1)

pub const PLX_REG_DMALADR0: c_uint = 0x0088;
pub const PLX_REG_DMALADR1: c_uint = 0x009c;
// DMA Channel N Transfer Size (Bytes) Register (N <= 1) (first 23 bits)

pub const PLX_REG_DMASIZ0: c_uint = 0x008c;
pub const PLX_REG_DMASIZ1: c_uint = 0x00a0;
// DMA Channel N Descriptor Pointer Register (N <= 1)

pub const PLX_REG_DMADPR0: c_uint = 0x0090;
pub const PLX_REG_DMADPR1: c_uint = 0x00a4;
// Descriptor Located In PCI Address Space (not local address space)

// End Of Chain

// Interrupt After Terminal Count

// Direction Of Transfer Local Bus To PCI (not PCI to local)

// Next Descriptor Address Bits 31:4 (16 byte boundary)

// DMA Channel N Command/Status Register (N <= 1) (8-bit)

pub const PLX_REG_DMACSR0: c_uint = 0x00a8;
pub const PLX_REG_DMACSR1: c_uint = 0x00a9;
// Channel Enable

// Channel Start - write 1 to start transfer (write-only)

// Channel Abort - write 1 to abort transfer (write-only)

// Clear Interrupt - write 1 to clear DMA Channel Interrupt (write-only)

// Channel Done - transfer complete/inactive (read-only)

// DMA Threshold Register
pub const PLX_REG_DMATHR: c_uint = 0x00b0;
//
// DMA Threshold constraints:
// (C0PLAF + 1) + (C0PLAE + 1) <= 32
// (C0LPAF + 1) + (C0LPAE + 1) <= 32
// (C1PLAF + 1) + (C1PLAE + 1) <= 16
// (C1LPAF + 1) + (C1LPAE + 1) <= 16
//
// DMA Channel 0 PCI-to-Local Almost Full (divided by 2, minus 1)

// DMA Channel 0 Local-to-PCI Almost Empty (divided by 2, minus 1)

// DMA Channel 0 Local-to-PCI Almost Full (divided by 2, minus 1)

// DMA Channel 0 PCI-to-Local Almost Empty (divided by 2, minus 1)

// DMA Channel 1 PCI-to-Local Almost Full (divided by 2, minus 1)

// DMA Channel 1 Local-to-PCI Almost Empty (divided by 2, minus 1)

// DMA Channel 1 Local-to-PCI Almost Full (divided by 2, minus 1)

// DMA Channel 1 PCI-to-Local Almost Empty (divided by 2, minus 1)

//
// Messaging Queue Registers OPLFIS, OPLFIM, IQP, OQP, MQCR, QBAR, IFHPR,
// IFTPR, IPHPR, IPTPR, OFHPR, OFTPR, OPHPR, OPTPR, and QSR have been omitted.
// They are used by the I2O feature.  (IQP and OQP occupy the usual offsets of
// the MBOX0 and MBOX1 registers if the I2O feature is enabled, but MBOX0 and
// MBOX1 are accessible via alternative offsets.
//
// Queue Status/Control Register
pub const PLX_REG_QSR: c_uint = 0x00e8;
// Value of QSR after reset - disables I2O feature completely.
pub const PLX_QSR_VALUE_AFTER_RESET: c_uint = 0x00000050;
//
// Accesses near the end of memory can cause the PLX chip
// to pre-fetch data off of end-of-ram.  Limit the size of
// memory so host-side accesses cannot occur.
//
pub const PLX_PREFETCH: c_int = 32;
//
// plx9080_abort_dma - Abort a PLX PCI 9080 DMA transfer
// @iobase:	Remapped base address of configuration registers.
// @channel:	DMA channel number (0 or 1).
//
// Aborts the DMA transfer on the channel, which must have been enabled
// and started beforehand.
//
// Return:
// %0 on success.
// -%ETIMEDOUT if timed out waiting for abort to complete.
//
// abort dma transfer if necessary
// wait to make sure done bit is zero
// disable and abort channel
// wait for dma done bit
