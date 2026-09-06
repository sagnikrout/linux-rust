//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/amd/ariadne.h
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
// Amiga Linux/m68k Ariadne Ethernet Driver
//
// © Copyright 1995 by Geert Uytterhoeven (geert@linux-m68k.org)
// Peter De Schrijver
// (Peter.DeSchrijver@linux.cc.kuleuven.ac.be)
//
// ----------------------------------------------------------------------------------
//
// This program is based on
//
// lance.c:	An AMD LANCE ethernet driver for linux.
// Written 1993-94 by Donald Becker.
//
// Am79C960:	PCnet(tm)-ISA Single-Chip Ethernet Controller
// Advanced Micro Devices
// Publication #16907, Rev. B, Amendment/0, May 1994
//
// MC68230:	Parallel Interface/Timer (PI/T)
// Motorola Semiconductors, December, 1983
//
// ----------------------------------------------------------------------------------
//
// This file is subject to the terms and conditions of the GNU General Public
// License.  See the file COPYING in the main directory of the Linux
// distribution for more details.
//
// ----------------------------------------------------------------------------------
//
// The Ariadne is a Zorro-II board made by Village Tronic. It contains:
//
// - an Am79C960 PCnet-ISA Single-Chip Ethernet Controller with both
// 10BASE-2 (thin coax) and 10BASE-T (UTP) connectors
//
// - an MC68230 Parallel Interface/Timer configured as 2 parallel ports
//
// Am79C960 PCnet-ISA
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Am79C960 {
    pub AddressPROM: [volatile u_short; 8],
// IEEE Address PROM (Unused in the Ariadne)
    pub /: *mut *mut volatile u_short RDP; / Register Data Port,
    pub /: *mut *mut volatile u_short RAP; / Register Address Port,
    pub /: *mut *mut volatile u_short Reset; / Reset Chip on Read Access,
    pub /: *mut *mut volatile u_short IDP; / ISACSR Data Port,
}

//
// Am79C960 Control and Status Registers
//
// These values are already swap()ed!!
//
// Only registers marked with a `-' are intended for network software
// access
//
pub const CSR0: c_uint = 0x0000	/* - PCnet-ISA Controller Status */;
pub const CSR1: c_uint = 0x0100	/* - IADR[15:0] */;
pub const CSR2: c_uint = 0x0200	/* - IADR[23:16] */;
pub const CSR3: c_uint = 0x0300	/* - Interrupt Masks and Deferral Control */;
pub const CSR4: c_uint = 0x0400	/* - Test and Features Control */;
pub const CSR6: c_uint = 0x0600	/*   RCV/XMT Descriptor Table Length */;
pub const CSR8: c_uint = 0x0800	/* - Logical Address Filter, LADRF[15:0] */;
pub const CSR9: c_uint = 0x0900	/* - Logical Address Filter, LADRF[31:16] */;
pub const CSR10: c_uint = 0x0a00	/* - Logical Address Filter, LADRF[47:32] */;
pub const CSR11: c_uint = 0x0b00	/* - Logical Address Filter, LADRF[63:48] */;
pub const CSR12: c_uint = 0x0c00	/* - Physical Address Register, PADR[15:0] */;
pub const CSR13: c_uint = 0x0d00	/* - Physical Address Register, PADR[31:16] */;
pub const CSR14: c_uint = 0x0e00	/* - Physical Address Register, PADR[47:32] */;
pub const CSR15: c_uint = 0x0f00	/* - Mode Register */;
pub const CSR16: c_uint = 0x1000	/*   Initialization Block Address Lower */;
pub const CSR17: c_uint = 0x1100	/*   Initialization Block Address Upper */;
pub const CSR18: c_uint = 0x1200	/*   Current Receive Buffer Address */;
pub const CSR19: c_uint = 0x1300	/*   Current Receive Buffer Address */;
pub const CSR20: c_uint = 0x1400	/*   Current Transmit Buffer Address */;
pub const CSR21: c_uint = 0x1500	/*   Current Transmit Buffer Address */;
pub const CSR22: c_uint = 0x1600	/*   Next Receive Buffer Address */;
pub const CSR23: c_uint = 0x1700	/*   Next Receive Buffer Address */;
pub const CSR24: c_uint = 0x1800	/* - Base Address of Receive Ring */;
pub const CSR25: c_uint = 0x1900	/* - Base Address of Receive Ring */;
pub const CSR26: c_uint = 0x1a00	/*   Next Receive Descriptor Address */;
pub const CSR27: c_uint = 0x1b00	/*   Next Receive Descriptor Address */;
pub const CSR28: c_uint = 0x1c00	/*   Current Receive Descriptor Address */;
pub const CSR29: c_uint = 0x1d00	/*   Current Receive Descriptor Address */;
pub const CSR30: c_uint = 0x1e00	/* - Base Address of Transmit Ring */;
pub const CSR31: c_uint = 0x1f00	/* - Base Address of transmit Ring */;
pub const CSR32: c_uint = 0x2000	/*   Next Transmit Descriptor Address */;
pub const CSR33: c_uint = 0x2100	/*   Next Transmit Descriptor Address */;
pub const CSR34: c_uint = 0x2200	/*   Current Transmit Descriptor Address */;
pub const CSR35: c_uint = 0x2300	/*   Current Transmit Descriptor Address */;
pub const CSR36: c_uint = 0x2400	/*   Next Next Receive Descriptor Address */;
pub const CSR37: c_uint = 0x2500	/*   Next Next Receive Descriptor Address */;
pub const CSR38: c_uint = 0x2600	/*   Next Next Transmit Descriptor Address */;
pub const CSR39: c_uint = 0x2700	/*   Next Next Transmit Descriptor Address */;
pub const CSR40: c_uint = 0x2800	/*   Current Receive Status and Byte Count */;
pub const CSR41: c_uint = 0x2900	/*   Current Receive Status and Byte Count */;
pub const CSR42: c_uint = 0x2a00	/*   Current Transmit Status and Byte Count */;
pub const CSR43: c_uint = 0x2b00	/*   Current Transmit Status and Byte Count */;
pub const CSR44: c_uint = 0x2c00	/*   Next Receive Status and Byte Count */;
pub const CSR45: c_uint = 0x2d00	/*   Next Receive Status and Byte Count */;
pub const CSR46: c_uint = 0x2e00	/*   Poll Time Counter */;
pub const CSR47: c_uint = 0x2f00	/*   Polling Interval */;
pub const CSR48: c_uint = 0x3000	/*   Temporary Storage */;
pub const CSR49: c_uint = 0x3100	/*   Temporary Storage */;
pub const CSR50: c_uint = 0x3200	/*   Temporary Storage */;
pub const CSR51: c_uint = 0x3300	/*   Temporary Storage */;
pub const CSR52: c_uint = 0x3400	/*   Temporary Storage */;
pub const CSR53: c_uint = 0x3500	/*   Temporary Storage */;
pub const CSR54: c_uint = 0x3600	/*   Temporary Storage */;
pub const CSR55: c_uint = 0x3700	/*   Temporary Storage */;
pub const CSR56: c_uint = 0x3800	/*   Temporary Storage */;
pub const CSR57: c_uint = 0x3900	/*   Temporary Storage */;
pub const CSR58: c_uint = 0x3a00	/*   Temporary Storage */;
pub const CSR59: c_uint = 0x3b00	/*   Temporary Storage */;
pub const CSR60: c_uint = 0x3c00	/*   Previous Transmit Descriptor Address */;
pub const CSR61: c_uint = 0x3d00	/*   Previous Transmit Descriptor Address */;
pub const CSR62: c_uint = 0x3e00	/*   Previous Transmit Status and Byte Count */;
pub const CSR63: c_uint = 0x3f00	/*   Previous Transmit Status and Byte Count */;
pub const CSR64: c_uint = 0x4000	/*   Next Transmit Buffer Address */;
pub const CSR65: c_uint = 0x4100	/*   Next Transmit Buffer Address */;
pub const CSR66: c_uint = 0x4200	/*   Next Transmit Status and Byte Count */;
pub const CSR67: c_uint = 0x4300	/*   Next Transmit Status and Byte Count */;
pub const CSR68: c_uint = 0x4400	/*   Transmit Status Temporary Storage */;
pub const CSR69: c_uint = 0x4500	/*   Transmit Status Temporary Storage */;
pub const CSR70: c_uint = 0x4600	/*   Temporary Storage */;
pub const CSR71: c_uint = 0x4700	/*   Temporary Storage */;
pub const CSR72: c_uint = 0x4800	/*   Receive Ring Counter */;
pub const CSR74: c_uint = 0x4a00	/*   Transmit Ring Counter */;
pub const CSR76: c_uint = 0x4c00	/* - Receive Ring Length */;
pub const CSR78: c_uint = 0x4e00	/* - Transmit Ring Length */;
pub const CSR80: c_uint = 0x5000	/* - Burst and FIFO Threshold Control */;
pub const CSR82: c_uint = 0x5200	/* - Bus Activity Timer */;
pub const CSR84: c_uint = 0x5400	/*   DMA Address */;
pub const CSR85: c_uint = 0x5500	/*   DMA Address */;
pub const CSR86: c_uint = 0x5600	/*   Buffer Byte Counter */;
pub const CSR88: c_uint = 0x5800	/* - Chip ID */;
pub const CSR89: c_uint = 0x5900	/* - Chip ID */;
pub const CSR92: c_uint = 0x5c00	/*   Ring Length Conversion */;
pub const CSR94: c_uint = 0x5e00	/*   Transmit Time Domain Reflectometry Count */;
pub const CSR96: c_uint = 0x6000	/*   Bus Interface Scratch Register 0 */;
pub const CSR97: c_uint = 0x6100	/*   Bus Interface Scratch Register 0 */;
pub const CSR98: c_uint = 0x6200	/*   Bus Interface Scratch Register 1 */;
pub const CSR99: c_uint = 0x6300	/*   Bus Interface Scratch Register 1 */;
pub const CSR104: c_uint = 0x6800	/*   SWAP */;
pub const CSR105: c_uint = 0x6900	/*   SWAP */;
pub const CSR108: c_uint = 0x6c00	/*   Buffer Management Scratch */;
pub const CSR109: c_uint = 0x6d00	/*   Buffer Management Scratch */;
pub const CSR112: c_uint = 0x7000	/* - Missed Frame Count */;
pub const CSR114: c_uint = 0x7200	/* - Receive Collision Count */;
pub const CSR124: c_uint = 0x7c00	/* - Buffer Management Unit Test */;
//
// Am79C960 ISA Control and Status Registers
//
// These values are already swap()ed!!
//
pub const ISACSR0: c_uint = 0x0000	/* Master Mode Read Active */;
pub const ISACSR1: c_uint = 0x0100	/* Master Mode Write Active */;
pub const ISACSR2: c_uint = 0x0200	/* Miscellaneous Configuration */;
pub const ISACSR4: c_uint = 0x0400	/* LED0 Status (Link Integrity) */;
pub const ISACSR5: c_uint = 0x0500	/* LED1 Status */;
pub const ISACSR6: c_uint = 0x0600	/* LED2 Status */;
pub const ISACSR7: c_uint = 0x0700	/* LED3 Status */;
//
// Bit definitions for CSR0 (PCnet-ISA Controller Status)
//
// These values are already swap()ed!!
//
pub const ERR: c_uint = 0x0080	/* Error */;
pub const BABL: c_uint = 0x0040	/* Babble: Transmitted too many bits */;
pub const CERR: c_uint = 0x0020	/* No Heartbeat (10BASE-T) */;
pub const MISS: c_uint = 0x0010	/* Missed Frame */;
pub const MERR: c_uint = 0x0008	/* Memory Error */;
pub const RINT: c_uint = 0x0004	/* Receive Interrupt */;
pub const TINT: c_uint = 0x0002	/* Transmit Interrupt */;
pub const IDON: c_uint = 0x0001	/* Initialization Done */;
pub const INTR: c_uint = 0x8000	/* Interrupt Flag */;
pub const INEA: c_uint = 0x4000	/* Interrupt Enable */;
pub const RXON: c_uint = 0x2000	/* Receive On */;
pub const TXON: c_uint = 0x1000	/* Transmit On */;
pub const TDMD: c_uint = 0x0800	/* Transmit Demand */;
pub const STOP: c_uint = 0x0400	/* Stop */;
pub const STRT: c_uint = 0x0200	/* Start */;
pub const INIT: c_uint = 0x0100	/* Initialize */;
//
// Bit definitions for CSR3 (Interrupt Masks and Deferral Control)
//
// These values are already swap()ed!!
//
pub const BABLM: c_uint = 0x0040	/* Babble Mask */;
pub const MISSM: c_uint = 0x0010	/* Missed Frame Mask */;
pub const MERRM: c_uint = 0x0008	/* Memory Error Mask */;
pub const RINTM: c_uint = 0x0004	/* Receive Interrupt Mask */;
pub const TINTM: c_uint = 0x0002	/* Transmit Interrupt Mask */;
pub const IDONM: c_uint = 0x0001	/* Initialization Done Mask */;
pub const DXMT2PD: c_uint = 0x1000	/* Disable Transmit Two Part Deferral */;
pub const EMBA: c_uint = 0x0800	/* Enable Modified Back-off Algorithm */;
//
// Bit definitions for CSR4 (Test and Features Control)
//
// These values are already swap()ed!!
//
pub const ENTST: c_uint = 0x0080	/* Enable Test Mode */;
pub const DMAPLUS: c_uint = 0x0040	/* Disable Burst Transaction Counter */;
pub const TIMER: c_uint = 0x0020	/* Timer Enable Register */;
pub const DPOLL: c_uint = 0x0010	/* Disable Transmit Polling */;
pub const APAD_XMT: c_uint = 0x0008	/* Auto Pad Transmit */;
pub const ASTRP_RCV: c_uint = 0x0004	/* Auto Pad Stripping */;
pub const MFCO: c_uint = 0x0002	/* Missed Frame Counter Overflow Interrupt */;
pub const MFCOM: c_uint = 0x0001	/* Missed Frame Counter Overflow Mask */;
pub const RCVCCO: c_uint = 0x2000	/* Receive Collision Counter Overflow Interrupt */;
pub const RCVCCOM: c_uint = 0x1000	/* Receive Collision Counter Overflow Mask */;
pub const TXSTRT: c_uint = 0x0800	/* Transmit Start Status */;
pub const TXSTRTM: c_uint = 0x0400	/* Transmit Start Mask */;
pub const JAB: c_uint = 0x0200	/* Jabber Error */;
pub const JABM: c_uint = 0x0100	/* Jabber Error Mask */;
//
// Bit definitions for CSR15 (Mode Register)
//
// These values are already swap()ed!!
//
pub const PROM: c_uint = 0x0080	/* Promiscuous Mode */;
pub const DRCVBC: c_uint = 0x0040	/* Disable Receive Broadcast */;
pub const DRCVPA: c_uint = 0x0020	/* Disable Receive Physical Address */;
pub const DLNKTST: c_uint = 0x0010	/* Disable Link Status */;
pub const DAPC: c_uint = 0x0008	/* Disable Automatic Polarity Correction */;
pub const MENDECL: c_uint = 0x0004	/* MENDEC Loopback Mode */;
pub const LRTTSEL: c_uint = 0x0002	/* Low Receive Threshold/Transmit Mode Select */;
pub const PORTSEL1: c_uint = 0x0001	/* Port Select Bits */;
pub const PORTSEL2: c_uint = 0x8000	/* Port Select Bits */;
pub const INTL: c_uint = 0x4000	/* Internal Loopback */;
pub const DRTY: c_uint = 0x2000	/* Disable Retry */;
pub const FCOLL: c_uint = 0x1000	/* Force Collision */;
pub const DXMTFCS: c_uint = 0x0800	/* Disable Transmit CRC */;
pub const LOOP: c_uint = 0x0400	/* Loopback Enable */;
pub const DTX: c_uint = 0x0200	/* Disable Transmitter */;
pub const DRX: c_uint = 0x0100	/* Disable Receiver */;
//
// Bit definitions for ISACSR2 (Miscellaneous Configuration)
//
// These values are already swap()ed!!
//
pub const ASEL: c_uint = 0x0200	/* Media Interface Port Auto Select */;
//
// Bit definitions for ISACSR5-7 (LED1-3 Status)
//
// These values are already swap()ed!!
//
pub const LEDOUT: c_uint = 0x0080	/* Current LED Status */;
pub const PSE: c_uint = 0x8000	/* Pulse Stretcher Enable */;
pub const XMTE: c_uint = 0x1000	/* Enable Transmit Status Signal */;
pub const RVPOLE: c_uint = 0x0800	/* Enable Receive Polarity Signal */;
pub const RCVE: c_uint = 0x0400	/* Enable Receive Status Signal */;
pub const JABE: c_uint = 0x0200	/* Enable Jabber Signal */;
pub const COLE: c_uint = 0x0100	/* Enable Collision Signal */;
//
// Receive Descriptor Ring Entry
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RDRE {
    pub /: *mut *mut volatile u_short RMD0; / LADR[15:0],
    pub /: *mut *mut volatile u_short RMD1; / HADR[23:16] | Receive Flags,
    pub /: *mut *mut volatile u_short RMD2; / Buffer Byte Count (two's complement),
    pub /: *mut *mut volatile u_short RMD3; / Message Byte Count,
}

//
// Transmit Descriptor Ring Entry
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TDRE {
    pub /: *mut *mut volatile u_short TMD0; / LADR[15:0],
    pub /: *mut *mut volatile u_short TMD1; / HADR[23:16] | Transmit Flags,
    pub /: *mut *mut volatile u_short TMD2; / Buffer Byte Count (two's complement),
    pub /: *mut *mut volatile u_short TMD3; / Error Flags,
}

//
// Receive Flags
//
pub const RF_OWN: c_uint = 0x0080	/* PCnet-ISA controller owns the descriptor */;
pub const RF_ERR: c_uint = 0x0040	/* Error */;
pub const RF_FRAM: c_uint = 0x0020	/* Framing Error */;
pub const RF_OFLO: c_uint = 0x0010	/* Overflow Error */;
pub const RF_CRC: c_uint = 0x0008	/* CRC Error */;
pub const RF_BUFF: c_uint = 0x0004	/* Buffer Error */;
pub const RF_STP: c_uint = 0x0002	/* Start of Packet */;
pub const RF_ENP: c_uint = 0x0001	/* End of Packet */;
//
// Transmit Flags
//
pub const TF_OWN: c_uint = 0x0080	/* PCnet-ISA controller owns the descriptor */;
pub const TF_ERR: c_uint = 0x0040	/* Error */;
pub const TF_ADD_FCS: c_uint = 0x0020	/* Controls FCS Generation */;
pub const TF_MORE: c_uint = 0x0010	/* More than one retry needed */;
pub const TF_ONE: c_uint = 0x0008	/* One retry needed */;
pub const TF_DEF: c_uint = 0x0004	/* Deferred */;
pub const TF_STP: c_uint = 0x0002	/* Start of Packet */;
pub const TF_ENP: c_uint = 0x0001	/* End of Packet */;
//
// Error Flags
//
pub const EF_BUFF: c_uint = 0x0080	/* Buffer Error */;
pub const EF_UFLO: c_uint = 0x0040	/* Underflow Error */;
pub const EF_LCOL: c_uint = 0x0010	/* Late Collision */;
pub const EF_LCAR: c_uint = 0x0008	/* Loss of Carrier */;
pub const EF_RTRY: c_uint = 0x0004	/* Retry Error */;
pub const EF_TDR: c_uint = 0xff03	/* Time Domain Reflectometry */;
//
// MC68230 Parallel Interface/Timer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MC68230 {
    pub /: *mut *mut volatile u_char PGCR; / Port General Control Register,
    pub Pad1: [u_char; 1],
    pub /: *mut *mut volatile u_char PSRR; / Port Service Request Register,
    pub Pad2: [u_char; 1],
    pub /: *mut *mut volatile u_char PADDR; / Port A Data Direction Register,
    pub Pad3: [u_char; 1],
    pub /: *mut *mut volatile u_char PBDDR; / Port B Data Direction Register,
    pub Pad4: [u_char; 1],
    pub /: *mut *mut volatile u_char PCDDR; / Port C Data Direction Register,
    pub Pad5: [u_char; 1],
    pub /: *mut *mut volatile u_char PIVR; / Port Interrupt Vector Register,
    pub Pad6: [u_char; 1],
    pub /: *mut *mut volatile u_char PACR; / Port A Control Register,
    pub Pad7: [u_char; 1],
    pub /: *mut *mut volatile u_char PBCR; / Port B Control Register,
    pub Pad8: [u_char; 1],
    pub /: *mut *mut volatile u_char PADR; / Port A Data Register,
    pub Pad9: [u_char; 1],
    pub /: *mut *mut volatile u_char PBDR; / Port B Data Register,
    pub Pad10: [u_char; 1],
    pub /: *mut *mut volatile u_char PAAR; / Port A Alternate Register,
    pub Pad11: [u_char; 1],
    pub /: *mut *mut volatile u_char PBAR; / Port B Alternate Register,
    pub Pad12: [u_char; 1],
    pub /: *mut *mut volatile u_char PCDR; / Port C Data Register,
    pub Pad13: [u_char; 1],
    pub /: *mut *mut volatile u_char PSR; / Port Status Register,
    pub Pad14: [u_char; 5],
    pub /: *mut *mut volatile u_char TCR; / Timer Control Register,
    pub Pad15: [u_char; 1],
    pub /: *mut *mut volatile u_char TIVR; / Timer Interrupt Vector Register,
    pub Pad16: [u_char; 3],
    pub /: *mut *mut volatile u_char CPRH; / Counter Preload Register (High),
    pub Pad17: [u_char; 1],
    pub /: *mut *mut volatile u_char CPRM; / Counter Preload Register (Mid),
    pub Pad18: [u_char; 1],
    pub /: *mut *mut volatile u_char CPRL; / Counter Preload Register (Low),
    pub Pad19: [u_char; 3],
    pub /: *mut *mut volatile u_char CNTRH; / Count Register (High),
    pub Pad20: [u_char; 1],
    pub /: *mut *mut volatile u_char CNTRM; / Count Register (Mid),
    pub Pad21: [u_char; 1],
    pub /: *mut *mut volatile u_char CNTRL; / Count Register (Low),
    pub Pad22: [u_char; 1],
    pub /: *mut *mut volatile u_char TSR; / Timer Status Register,
    pub Pad23: [u_char; 11],
}

//
// Ariadne Expansion Board Structure
//
pub const ARIADNE_LANCE: c_uint = 0x360;
pub const ARIADNE_PIT: c_uint = 0x1000;
pub const ARIADNE_BOOTPROM: c_uint = 0x4000	/* I guess it's here :-) */;
pub const ARIADNE_BOOTPROM_SIZE: c_uint = 0x4000;
pub const ARIADNE_RAM: c_uint = 0x8000	/* Always access WORDs!! */;
pub const ARIADNE_RAM_SIZE: c_uint = 0x8000;
