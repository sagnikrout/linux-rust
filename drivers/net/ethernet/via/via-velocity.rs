//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/via/via-velocity.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright (c) 1996, 2003 VIA Networking Technologies, Inc.
// All rights reserved.
//
// File: via-velocity.h
//
// Purpose: Header file to define driver's private structures.
//
// Author: Chuang Liang-Shing, AJ Jiang
//
// Date: Jan 24, 2003
//
// Macro flag: #define VELOCITY_TX_CSUM_SUPPORT

pub const VELOCITY_IO_SIZE: c_int = 256;
pub const PKT_BUF_SZ: c_int = 1540;
pub const MAX_UNITS: c_int = 8;

//
// Purpose: Structures for MAX RX/TX descriptors.
//
pub const B_OWNED_BY_CHIP: c_int = 1;
pub const B_OWNED_BY_HOST: c_int = 0;
//
// Bits in the RSR0 register
//

//
// Bits in the RSR1 register
//

//
// Bits in the CSM register
//
pub const CSM_IPOK: c_uint = 0x40	//IP Checksum validation ok;
pub const CSM_TUPOK: c_uint = 0x20	//TCP/UDP Checksum validation ok;
pub const CSM_FRAG: c_uint = 0x10	//Fragment IP datagram;
pub const CSM_IPKT: c_uint = 0x04	//Received an IP packet;
pub const CSM_TCPKT: c_uint = 0x02	//Received a TCP packet;
pub const CSM_UDPKT: c_uint = 0x01	//Received a UDP packet;
//
// Bits in the TSR0 register
//

//
// Bits in the TCR0 register
//
pub const TCR0_TIC: c_uint = 0x80	// assert interrupt immediately while descriptor has been send complete;
pub const TCR0_PIC: c_uint = 0x40	// priority interrupt request, INA# is issued over adaptive interrupt scheme;
pub const TCR0_VETAG: c_uint = 0x20	// enable VLAN tag;
pub const TCR0_IPCK: c_uint = 0x10	// request IP  checksum calculation.;
pub const TCR0_UDPCK: c_uint = 0x08	// request UDP checksum calculation.;
pub const TCR0_TCPCK: c_uint = 0x04	// request TCP checksum calculation.;
pub const TCR0_JMBO: c_uint = 0x02	// indicate a jumbo packet in GMAC side;
pub const TCR0_CRC: c_uint = 0x01	// disable CRC generation;
pub const TCPLS_NORMAL: c_int = 3;
pub const TCPLS_START: c_int = 2;
pub const TCPLS_END: c_int = 1;
pub const TCPLS_MED: c_int = 0;
// max transmit or receive buffer size

// NOTE: must be multiple of 4

// for 3119

//
// If collisions excess 15 times , tx will abort, and
// if tx fifo underflow, tx will fail
// we should try to resend it
//
pub const CB_MAX_TX_ABORT_RETRY: c_int = 3;
//
// Receive descriptor
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdesc0 {
    pub /: *mut *mut __le16 RSR; / Receive status,
    pub /: *mut *mut __le16 len; / bits 0--13; bit 15 - owner,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdesc1 {
    pub PQTAG: __le16,
    pub CSM: u8,
    pub IPKT: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rx_desc {
    pub rdesc0: rdesc0,
    pub rdesc1: rdesc1,
    pub /: *mut *mut __le32 pa_low; / Low 32 bit PCI address,
    pub /: *mut *mut __le16 pa_high; / Next 16 bit PCI address (48 total),
    pub /: *mut *mut __le16 size; / bits 0--14 - frame size, bit 15 - enable int.,
    pub __packed: },
//
// Transmit descriptor
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tdesc0 {
    pub /: *mut *mut __le16 TSR; / Transmit status register,
    pub /: *mut *mut __le16 len; / bits 0--13 - size of frame, bit 15 - owner,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tdesc1 {
    pub vlan: __le16,
    pub TCR: u8,
    pub /: *mut *mut u8 cmd; / bits 0--1 - TCPLS, bits 4--7 - CMDZ,
    pub __packed: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct td_buf {
    pub pa_low: __le32,
    pub pa_high: __le16,
    pub /: *mut *mut __le16 size; / bits 0--13 - size, bit 15 - queue,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tx_desc {
    pub tdesc0: tdesc0,
    pub tdesc1: tdesc1,
    pub td_buf: [td_buf; 7],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct velocity_rd_info {
    pub skb: *mut sk_buff,
    pub skb_dma: dma_addr_t,
}

//
// Used to track transmit side buffers.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct velocity_td_info {
    pub skb: *mut sk_buff,
    pub nskb_dma: c_int,
    pub skb_dma: [dma_addr_t; 7],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum velocity_owner {
    OWNED_BY_HOST = 0,
    OWNED_BY_NIC = cpu_to_le16(0x8000)
}

//
// MAC registers and macros.
//
pub const MCAM_SIZE: c_int = 64;
pub const VCAM_SIZE: c_int = 64;
pub const TX_QUEUE_NO: c_int = 4;
pub const MAX_HW_MIB_COUNTER: c_int = 32;

//
// Registers in the MAC
//
pub const MAC_REG_PAR: c_uint = 0x00	// physical address;
pub const MAC_REG_RCR: c_uint = 0x06;
pub const MAC_REG_TCR: c_uint = 0x07;
pub const MAC_REG_CR0_SET: c_uint = 0x08;
pub const MAC_REG_CR1_SET: c_uint = 0x09;
pub const MAC_REG_CR2_SET: c_uint = 0x0A;
pub const MAC_REG_CR3_SET: c_uint = 0x0B;
pub const MAC_REG_CR0_CLR: c_uint = 0x0C;
pub const MAC_REG_CR1_CLR: c_uint = 0x0D;
pub const MAC_REG_CR2_CLR: c_uint = 0x0E;
pub const MAC_REG_CR3_CLR: c_uint = 0x0F;
pub const MAC_REG_MAR: c_uint = 0x10;
pub const MAC_REG_CAM: c_uint = 0x10;
pub const MAC_REG_DEC_BASE_HI: c_uint = 0x18;
pub const MAC_REG_DBF_BASE_HI: c_uint = 0x1C;
pub const MAC_REG_ISR_CTL: c_uint = 0x20;
pub const MAC_REG_ISR_HOTMR: c_uint = 0x20;
pub const MAC_REG_ISR_TSUPTHR: c_uint = 0x20;
pub const MAC_REG_ISR_RSUPTHR: c_uint = 0x20;
pub const MAC_REG_ISR_CTL1: c_uint = 0x21;
pub const MAC_REG_TXE_SR: c_uint = 0x22;
pub const MAC_REG_RXE_SR: c_uint = 0x23;
pub const MAC_REG_ISR: c_uint = 0x24;
pub const MAC_REG_ISR0: c_uint = 0x24;
pub const MAC_REG_ISR1: c_uint = 0x25;
pub const MAC_REG_ISR2: c_uint = 0x26;
pub const MAC_REG_ISR3: c_uint = 0x27;
pub const MAC_REG_IMR: c_uint = 0x28;
pub const MAC_REG_IMR0: c_uint = 0x28;
pub const MAC_REG_IMR1: c_uint = 0x29;
pub const MAC_REG_IMR2: c_uint = 0x2A;
pub const MAC_REG_IMR3: c_uint = 0x2B;
pub const MAC_REG_TDCSR_SET: c_uint = 0x30;
pub const MAC_REG_RDCSR_SET: c_uint = 0x32;
pub const MAC_REG_TDCSR_CLR: c_uint = 0x34;
pub const MAC_REG_RDCSR_CLR: c_uint = 0x36;
pub const MAC_REG_RDBASE_LO: c_uint = 0x38;
pub const MAC_REG_RDINDX: c_uint = 0x3C;
pub const MAC_REG_TDBASE_LO: c_uint = 0x40;
pub const MAC_REG_RDCSIZE: c_uint = 0x50;
pub const MAC_REG_TDCSIZE: c_uint = 0x52;
pub const MAC_REG_TDINDX: c_uint = 0x54;
pub const MAC_REG_TDIDX0: c_uint = 0x54;
pub const MAC_REG_TDIDX1: c_uint = 0x56;
pub const MAC_REG_TDIDX2: c_uint = 0x58;
pub const MAC_REG_TDIDX3: c_uint = 0x5A;
pub const MAC_REG_PAUSE_TIMER: c_uint = 0x5C;
pub const MAC_REG_RBRDU: c_uint = 0x5E;
pub const MAC_REG_FIFO_TEST0: c_uint = 0x60;
pub const MAC_REG_FIFO_TEST1: c_uint = 0x64;
pub const MAC_REG_CAMADDR: c_uint = 0x68;
pub const MAC_REG_CAMCR: c_uint = 0x69;
pub const MAC_REG_GFTEST: c_uint = 0x6A;
pub const MAC_REG_FTSTCMD: c_uint = 0x6B;
pub const MAC_REG_MIICFG: c_uint = 0x6C;
pub const MAC_REG_MIISR: c_uint = 0x6D;
pub const MAC_REG_PHYSR0: c_uint = 0x6E;
pub const MAC_REG_PHYSR1: c_uint = 0x6F;
pub const MAC_REG_MIICR: c_uint = 0x70;
pub const MAC_REG_MIIADR: c_uint = 0x71;
pub const MAC_REG_MIIDATA: c_uint = 0x72;
pub const MAC_REG_SOFT_TIMER0: c_uint = 0x74;
pub const MAC_REG_SOFT_TIMER1: c_uint = 0x76;
pub const MAC_REG_CFGA: c_uint = 0x78;
pub const MAC_REG_CFGB: c_uint = 0x79;
pub const MAC_REG_CFGC: c_uint = 0x7A;
pub const MAC_REG_CFGD: c_uint = 0x7B;
pub const MAC_REG_DCFG0: c_uint = 0x7C;
pub const MAC_REG_DCFG1: c_uint = 0x7D;
pub const MAC_REG_MCFG0: c_uint = 0x7E;
pub const MAC_REG_MCFG1: c_uint = 0x7F;
pub const MAC_REG_TBIST: c_uint = 0x80;
pub const MAC_REG_RBIST: c_uint = 0x81;
pub const MAC_REG_PMCC: c_uint = 0x82;
pub const MAC_REG_STICKHW: c_uint = 0x83;
pub const MAC_REG_MIBCR: c_uint = 0x84;
pub const MAC_REG_EERSV: c_uint = 0x85;
pub const MAC_REG_REVID: c_uint = 0x86;
pub const MAC_REG_MIBREAD: c_uint = 0x88;
pub const MAC_REG_BPMA: c_uint = 0x8C;
pub const MAC_REG_EEWR_DATA: c_uint = 0x8C;
pub const MAC_REG_BPMD_WR: c_uint = 0x8F;
pub const MAC_REG_BPCMD: c_uint = 0x90;
pub const MAC_REG_BPMD_RD: c_uint = 0x91;
pub const MAC_REG_EECHKSUM: c_uint = 0x92;
pub const MAC_REG_EECSR: c_uint = 0x93;
pub const MAC_REG_EERD_DATA: c_uint = 0x94;
pub const MAC_REG_EADDR: c_uint = 0x96;
pub const MAC_REG_EMBCMD: c_uint = 0x97;
pub const MAC_REG_JMPSR0: c_uint = 0x98;
pub const MAC_REG_JMPSR1: c_uint = 0x99;
pub const MAC_REG_JMPSR2: c_uint = 0x9A;
pub const MAC_REG_JMPSR3: c_uint = 0x9B;
pub const MAC_REG_CHIPGSR: c_uint = 0x9C;
pub const MAC_REG_TESTCFG: c_uint = 0x9D;
pub const MAC_REG_DEBUG: c_uint = 0x9E;
pub const MAC_REG_CHIPGCR: c_uint = 0x9F	/* Chip Operation and Diagnostic Control */;
pub const MAC_REG_WOLCR0_SET: c_uint = 0xA0;
pub const MAC_REG_WOLCR1_SET: c_uint = 0xA1;
pub const MAC_REG_PWCFG_SET: c_uint = 0xA2;
pub const MAC_REG_WOLCFG_SET: c_uint = 0xA3;
pub const MAC_REG_WOLCR0_CLR: c_uint = 0xA4;
pub const MAC_REG_WOLCR1_CLR: c_uint = 0xA5;
pub const MAC_REG_PWCFG_CLR: c_uint = 0xA6;
pub const MAC_REG_WOLCFG_CLR: c_uint = 0xA7;
pub const MAC_REG_WOLSR0_SET: c_uint = 0xA8;
pub const MAC_REG_WOLSR1_SET: c_uint = 0xA9;
pub const MAC_REG_WOLSR0_CLR: c_uint = 0xAC;
pub const MAC_REG_WOLSR1_CLR: c_uint = 0xAD;
pub const MAC_REG_PATRN_CRC0: c_uint = 0xB0;
pub const MAC_REG_PATRN_CRC1: c_uint = 0xB2;
pub const MAC_REG_PATRN_CRC2: c_uint = 0xB4;
pub const MAC_REG_PATRN_CRC3: c_uint = 0xB6;
pub const MAC_REG_PATRN_CRC4: c_uint = 0xB8;
pub const MAC_REG_PATRN_CRC5: c_uint = 0xBA;
pub const MAC_REG_PATRN_CRC6: c_uint = 0xBC;
pub const MAC_REG_PATRN_CRC7: c_uint = 0xBE;
pub const MAC_REG_BYTEMSK0_0: c_uint = 0xC0;
pub const MAC_REG_BYTEMSK0_1: c_uint = 0xC4;
pub const MAC_REG_BYTEMSK0_2: c_uint = 0xC8;
pub const MAC_REG_BYTEMSK0_3: c_uint = 0xCC;
pub const MAC_REG_BYTEMSK1_0: c_uint = 0xD0;
pub const MAC_REG_BYTEMSK1_1: c_uint = 0xD4;
pub const MAC_REG_BYTEMSK1_2: c_uint = 0xD8;
pub const MAC_REG_BYTEMSK1_3: c_uint = 0xDC;
pub const MAC_REG_BYTEMSK2_0: c_uint = 0xE0;
pub const MAC_REG_BYTEMSK2_1: c_uint = 0xE4;
pub const MAC_REG_BYTEMSK2_2: c_uint = 0xE8;
pub const MAC_REG_BYTEMSK2_3: c_uint = 0xEC;
pub const MAC_REG_BYTEMSK3_0: c_uint = 0xF0;
pub const MAC_REG_BYTEMSK3_1: c_uint = 0xF4;
pub const MAC_REG_BYTEMSK3_2: c_uint = 0xF8;
pub const MAC_REG_BYTEMSK3_3: c_uint = 0xFC;
//
// Bits in the RCR register
//
pub const RCR_AS: c_uint = 0x80;
pub const RCR_AP: c_uint = 0x40;
pub const RCR_AL: c_uint = 0x20;
pub const RCR_PROM: c_uint = 0x10;
pub const RCR_AB: c_uint = 0x08;
pub const RCR_AM: c_uint = 0x04;
pub const RCR_AR: c_uint = 0x02;
pub const RCR_SEP: c_uint = 0x01;
//
// Bits in the TCR register
//
pub const TCR_TB2BDIS: c_uint = 0x80;
pub const TCR_COLTMC1: c_uint = 0x08;
pub const TCR_COLTMC0: c_uint = 0x04;
pub const TCR_LB1: c_uint = 0x02	/* loopback[1] */;
pub const TCR_LB0: c_uint = 0x01	/* loopback[0] */;
//
// Bits in the CR0 register
//
pub const CR0_TXON: c_uint = 0x00000008UL;
pub const CR0_RXON: c_uint = 0x00000004UL;
pub const CR0_STOP: c_uint = 0x00000002UL	/* stop MAC, default = 1 */;
pub const CR0_STRT: c_uint = 0x00000001UL	/* start MAC */;
pub const CR0_SFRST: c_uint = 0x00008000UL	/* software reset */;
pub const CR0_TM1EN: c_uint = 0x00004000UL;
pub const CR0_TM0EN: c_uint = 0x00002000UL;
pub const CR0_DPOLL: c_uint = 0x00000800UL	/* disable rx/tx auto polling */;
pub const CR0_DISAU: c_uint = 0x00000100UL;
pub const CR0_XONEN: c_uint = 0x00800000UL;
pub const CR0_FDXTFCEN: c_uint = 0x00400000UL	/* full-duplex TX flow control enable */;
pub const CR0_FDXRFCEN: c_uint = 0x00200000UL	/* full-duplex RX flow control enable */;
pub const CR0_HDXFCEN: c_uint = 0x00100000UL	/* half-duplex flow control enable */;
pub const CR0_XHITH1: c_uint = 0x00080000UL	/* TX XON high threshold 1 */;
pub const CR0_XHITH0: c_uint = 0x00040000UL	/* TX XON high threshold 0 */;
pub const CR0_XLTH1: c_uint = 0x00020000UL	/* TX pause frame low threshold 1 */;
pub const CR0_XLTH0: c_uint = 0x00010000UL	/* TX pause frame low threshold 0 */;
pub const CR0_GSPRST: c_uint = 0x80000000UL;
pub const CR0_FORSRST: c_uint = 0x40000000UL;
pub const CR0_FPHYRST: c_uint = 0x20000000UL;
pub const CR0_DIAG: c_uint = 0x10000000UL;
pub const CR0_INTPCTL: c_uint = 0x04000000UL;
pub const CR0_GINTMSK1: c_uint = 0x02000000UL;
pub const CR0_GINTMSK0: c_uint = 0x01000000UL;
//
// Bits in the CR1 register
//
pub const CR1_SFRST: c_uint = 0x80	/* software reset */;
pub const CR1_TM1EN: c_uint = 0x40;
pub const CR1_TM0EN: c_uint = 0x20;
pub const CR1_DPOLL: c_uint = 0x08	/* disable rx/tx auto polling */;
pub const CR1_DISAU: c_uint = 0x01;
//
// Bits in the CR2 register
//
pub const CR2_XONEN: c_uint = 0x80;
pub const CR2_FDXTFCEN: c_uint = 0x40	/* full-duplex TX flow control enable */;
pub const CR2_FDXRFCEN: c_uint = 0x20	/* full-duplex RX flow control enable */;
pub const CR2_HDXFCEN: c_uint = 0x10	/* half-duplex flow control enable */;
pub const CR2_XHITH1: c_uint = 0x08	/* TX XON high threshold 1 */;
pub const CR2_XHITH0: c_uint = 0x04	/* TX XON high threshold 0 */;
pub const CR2_XLTH1: c_uint = 0x02	/* TX pause frame low threshold 1 */;
pub const CR2_XLTH0: c_uint = 0x01	/* TX pause frame low threshold 0 */;
//
// Bits in the CR3 register
//
pub const CR3_GSPRST: c_uint = 0x80;
pub const CR3_FORSRST: c_uint = 0x40;
pub const CR3_FPHYRST: c_uint = 0x20;
pub const CR3_DIAG: c_uint = 0x10;
pub const CR3_INTPCTL: c_uint = 0x04;
pub const CR3_GINTMSK1: c_uint = 0x02;
pub const CR3_GINTMSK0: c_uint = 0x01;
pub const ISRCTL_UDPINT: c_uint = 0x8000;
pub const ISRCTL_TSUPDIS: c_uint = 0x4000;
pub const ISRCTL_RSUPDIS: c_uint = 0x2000;
pub const ISRCTL_PMSK1: c_uint = 0x1000;
pub const ISRCTL_PMSK0: c_uint = 0x0800;
pub const ISRCTL_INTPD: c_uint = 0x0400;
pub const ISRCTL_HCRLD: c_uint = 0x0200;
pub const ISRCTL_SCRLD: c_uint = 0x0100;
//
// Bits in the ISR_CTL1 register
//
pub const ISRCTL1_UDPINT: c_uint = 0x80;
pub const ISRCTL1_TSUPDIS: c_uint = 0x40;
pub const ISRCTL1_RSUPDIS: c_uint = 0x20;
pub const ISRCTL1_PMSK1: c_uint = 0x10;
pub const ISRCTL1_PMSK0: c_uint = 0x08;
pub const ISRCTL1_INTPD: c_uint = 0x04;
pub const ISRCTL1_HCRLD: c_uint = 0x02;
pub const ISRCTL1_SCRLD: c_uint = 0x01;
//
// Bits in the TXE_SR register
//
pub const TXESR_TFDBS: c_uint = 0x08;
pub const TXESR_TDWBS: c_uint = 0x04;
pub const TXESR_TDRBS: c_uint = 0x02;
pub const TXESR_TDSTR: c_uint = 0x01;
//
// Bits in the RXE_SR register
//
pub const RXESR_RFDBS: c_uint = 0x08;
pub const RXESR_RDWBS: c_uint = 0x04;
pub const RXESR_RDRBS: c_uint = 0x02;
pub const RXESR_RDSTR: c_uint = 0x01;
//
// Bits in the ISR register
//
pub const ISR_ISR3: c_uint = 0x80000000UL;
pub const ISR_ISR2: c_uint = 0x40000000UL;
pub const ISR_ISR1: c_uint = 0x20000000UL;
pub const ISR_ISR0: c_uint = 0x10000000UL;
pub const ISR_TXSTLI: c_uint = 0x02000000UL;
pub const ISR_RXSTLI: c_uint = 0x01000000UL;
pub const ISR_HFLD: c_uint = 0x00800000UL;
pub const ISR_UDPI: c_uint = 0x00400000UL;
pub const ISR_MIBFI: c_uint = 0x00200000UL;
pub const ISR_SHDNI: c_uint = 0x00100000UL;
pub const ISR_PHYI: c_uint = 0x00080000UL;
pub const ISR_PWEI: c_uint = 0x00040000UL;
pub const ISR_TMR1I: c_uint = 0x00020000UL;
pub const ISR_TMR0I: c_uint = 0x00010000UL;
pub const ISR_SRCI: c_uint = 0x00008000UL;
pub const ISR_LSTPEI: c_uint = 0x00004000UL;
pub const ISR_LSTEI: c_uint = 0x00002000UL;
pub const ISR_OVFI: c_uint = 0x00001000UL;
pub const ISR_FLONI: c_uint = 0x00000800UL;
pub const ISR_RACEI: c_uint = 0x00000400UL;
pub const ISR_TXWB1I: c_uint = 0x00000200UL;
pub const ISR_TXWB0I: c_uint = 0x00000100UL;
pub const ISR_PTX3I: c_uint = 0x00000080UL;
pub const ISR_PTX2I: c_uint = 0x00000040UL;
pub const ISR_PTX1I: c_uint = 0x00000020UL;
pub const ISR_PTX0I: c_uint = 0x00000010UL;
pub const ISR_PTXI: c_uint = 0x00000008UL;
pub const ISR_PRXI: c_uint = 0x00000004UL;
pub const ISR_PPTXI: c_uint = 0x00000002UL;
pub const ISR_PPRXI: c_uint = 0x00000001UL;
//
// Bits in the IMR register
//
pub const IMR_TXSTLM: c_uint = 0x02000000UL;
pub const IMR_UDPIM: c_uint = 0x00400000UL;
pub const IMR_MIBFIM: c_uint = 0x00200000UL;
pub const IMR_SHDNIM: c_uint = 0x00100000UL;
pub const IMR_PHYIM: c_uint = 0x00080000UL;
pub const IMR_PWEIM: c_uint = 0x00040000UL;
pub const IMR_TMR1IM: c_uint = 0x00020000UL;
pub const IMR_TMR0IM: c_uint = 0x00010000UL;
pub const IMR_SRCIM: c_uint = 0x00008000UL;
pub const IMR_LSTPEIM: c_uint = 0x00004000UL;
pub const IMR_LSTEIM: c_uint = 0x00002000UL;
pub const IMR_OVFIM: c_uint = 0x00001000UL;
pub const IMR_FLONIM: c_uint = 0x00000800UL;
pub const IMR_RACEIM: c_uint = 0x00000400UL;
pub const IMR_TXWB1IM: c_uint = 0x00000200UL;
pub const IMR_TXWB0IM: c_uint = 0x00000100UL;
pub const IMR_PTX3IM: c_uint = 0x00000080UL;
pub const IMR_PTX2IM: c_uint = 0x00000040UL;
pub const IMR_PTX1IM: c_uint = 0x00000020UL;
pub const IMR_PTX0IM: c_uint = 0x00000010UL;
pub const IMR_PTXIM: c_uint = 0x00000008UL;
pub const IMR_PRXIM: c_uint = 0x00000004UL;
pub const IMR_PPTXIM: c_uint = 0x00000002UL;
pub const IMR_PPRXIM: c_uint = 0x00000001UL;
// 0x0013FB0FUL  =  initial value of IMR

//
// Bits in the TDCSR0/1, RDCSR0 register
//
pub const TRDCSR_DEAD: c_uint = 0x0008;
pub const TRDCSR_WAK: c_uint = 0x0004;
pub const TRDCSR_ACT: c_uint = 0x0002;
pub const TRDCSR_RUN: c_uint = 0x0001;
//
// Bits in the CAMADDR register
//
pub const CAMADDR_CAMEN: c_uint = 0x80;
pub const CAMADDR_VCAMSL: c_uint = 0x40;
//
// Bits in the CAMCR register
//
pub const CAMCR_PS1: c_uint = 0x80;
pub const CAMCR_PS0: c_uint = 0x40;
pub const CAMCR_AITRPKT: c_uint = 0x20;
pub const CAMCR_AITR16: c_uint = 0x10;
pub const CAMCR_CAMRD: c_uint = 0x08;
pub const CAMCR_CAMWR: c_uint = 0x04;
pub const CAMCR_PS_CAM_MASK: c_uint = 0x40;
pub const CAMCR_PS_CAM_DATA: c_uint = 0x80;
pub const CAMCR_PS_MAR: c_uint = 0x00;
//
// Bits in the MIICFG register
//
pub const MIICFG_MPO1: c_uint = 0x80;
pub const MIICFG_MPO0: c_uint = 0x40;
pub const MIICFG_MFDC: c_uint = 0x20;
//
// Bits in the MIISR register
//
pub const MIISR_MIDLE: c_uint = 0x80;
//
// Bits in the PHYSR0 register
//
pub const PHYSR0_PHYRST: c_uint = 0x80;
pub const PHYSR0_LINKGD: c_uint = 0x40;
pub const PHYSR0_FDPX: c_uint = 0x10;
pub const PHYSR0_SPDG: c_uint = 0x08;
pub const PHYSR0_SPD10: c_uint = 0x04;
pub const PHYSR0_RXFLC: c_uint = 0x02;
pub const PHYSR0_TXFLC: c_uint = 0x01;
//
// Bits in the PHYSR1 register
//
pub const PHYSR1_PHYTBI: c_uint = 0x01;
//
// Bits in the MIICR register
//
pub const MIICR_MAUTO: c_uint = 0x80;
pub const MIICR_RCMD: c_uint = 0x40;
pub const MIICR_WCMD: c_uint = 0x20;
pub const MIICR_MDPM: c_uint = 0x10;
pub const MIICR_MOUT: c_uint = 0x08;
pub const MIICR_MDO: c_uint = 0x04;
pub const MIICR_MDI: c_uint = 0x02;
pub const MIICR_MDC: c_uint = 0x01;
//
// Bits in the MIIADR register
//
pub const MIIADR_SWMPL: c_uint = 0x80;
//
// Bits in the CFGA register
//
pub const CFGA_PMHCTG: c_uint = 0x08;
pub const CFGA_GPIO1PD: c_uint = 0x04;
pub const CFGA_ABSHDN: c_uint = 0x02;
pub const CFGA_PACPI: c_uint = 0x01;
//
// Bits in the CFGB register
//
pub const CFGB_GTCKOPT: c_uint = 0x80;
pub const CFGB_MIIOPT: c_uint = 0x40;
pub const CFGB_CRSEOPT: c_uint = 0x20;
pub const CFGB_OFSET: c_uint = 0x10;
pub const CFGB_CRANDOM: c_uint = 0x08;
pub const CFGB_CAP: c_uint = 0x04;
pub const CFGB_MBA: c_uint = 0x02;
pub const CFGB_BAKOPT: c_uint = 0x01;
//
// Bits in the CFGC register
//
pub const CFGC_EELOAD: c_uint = 0x80;
pub const CFGC_BROPT: c_uint = 0x40;
pub const CFGC_DLYEN: c_uint = 0x20;
pub const CFGC_DTSEL: c_uint = 0x10;
pub const CFGC_BTSEL: c_uint = 0x08;
pub const CFGC_BPS2: c_uint = 0x04	/* bootrom select[2] */;
pub const CFGC_BPS1: c_uint = 0x02	/* bootrom select[1] */;
pub const CFGC_BPS0: c_uint = 0x01	/* bootrom select[0] */;
//
// Bits in the CFGD register
//
pub const CFGD_IODIS: c_uint = 0x80;
pub const CFGD_MSLVDACEN: c_uint = 0x40;
pub const CFGD_CFGDACEN: c_uint = 0x20;
pub const CFGD_PCI64EN: c_uint = 0x10;
pub const CFGD_HTMRL4: c_uint = 0x08;
//
// Bits in the DCFG1 register
//
pub const DCFG_XMWI: c_uint = 0x8000;
pub const DCFG_XMRM: c_uint = 0x4000;
pub const DCFG_XMRL: c_uint = 0x2000;
pub const DCFG_PERDIS: c_uint = 0x1000;
pub const DCFG_MRWAIT: c_uint = 0x0400;
pub const DCFG_MWWAIT: c_uint = 0x0200;
pub const DCFG_LATMEN: c_uint = 0x0100;
//
// Bits in the MCFG0 register
//
pub const MCFG_RXARB: c_uint = 0x0080;
pub const MCFG_RFT1: c_uint = 0x0020;
pub const MCFG_RFT0: c_uint = 0x0010;
pub const MCFG_LOWTHOPT: c_uint = 0x0008;
pub const MCFG_PQEN: c_uint = 0x0004;
pub const MCFG_RTGOPT: c_uint = 0x0002;
pub const MCFG_VIDFR: c_uint = 0x0001;
//
// Bits in the MCFG1 register
//
pub const MCFG_TXARB: c_uint = 0x8000;
pub const MCFG_TXQBK1: c_uint = 0x0800;
pub const MCFG_TXQBK0: c_uint = 0x0400;
pub const MCFG_TXQNOBK: c_uint = 0x0200;
pub const MCFG_SNAPOPT: c_uint = 0x0100;
//
// Bits in the PMCC  register
//
pub const PMCC_DSI: c_uint = 0x80;
pub const PMCC_D2_DIS: c_uint = 0x40;
pub const PMCC_D1_DIS: c_uint = 0x20;
pub const PMCC_D3C_EN: c_uint = 0x10;
pub const PMCC_D3H_EN: c_uint = 0x08;
pub const PMCC_D2_EN: c_uint = 0x04;
pub const PMCC_D1_EN: c_uint = 0x02;
pub const PMCC_D0_EN: c_uint = 0x01;
//
// Bits in STICKHW
//
pub const STICKHW_SWPTAG: c_uint = 0x10;
pub const STICKHW_WOLSR: c_uint = 0x08;
pub const STICKHW_WOLEN: c_uint = 0x04;
pub const STICKHW_DS1: c_uint = 0x02	/* R/W by software/cfg cycle */;
pub const STICKHW_DS0: c_uint = 0x01	/* suspend well DS write port */;
//
// Bits in the MIBCR register
//
pub const MIBCR_MIBISTOK: c_uint = 0x80;
pub const MIBCR_MIBISTGO: c_uint = 0x40;
pub const MIBCR_MIBINC: c_uint = 0x20;
pub const MIBCR_MIBHI: c_uint = 0x10;
pub const MIBCR_MIBFRZ: c_uint = 0x08;
pub const MIBCR_MIBFLSH: c_uint = 0x04;
pub const MIBCR_MPTRINI: c_uint = 0x02;
pub const MIBCR_MIBCLR: c_uint = 0x01;
//
// Bits in the EERSV register
//

//
// Bits in BPCMD
//
pub const BPCMD_BPDNE: c_uint = 0x80;
pub const BPCMD_EBPWR: c_uint = 0x02;
pub const BPCMD_EBPRD: c_uint = 0x01;
//
// Bits in the EECSR register
//
pub const EECSR_EMBP: c_uint = 0x40	/* eeprom embedded programming */;
pub const EECSR_RELOAD: c_uint = 0x20	/* eeprom content reload */;
pub const EECSR_DPM: c_uint = 0x10	/* eeprom direct programming */;
pub const EECSR_ECS: c_uint = 0x08	/* eeprom CS pin */;
pub const EECSR_ECK: c_uint = 0x04	/* eeprom CK pin */;
pub const EECSR_EDI: c_uint = 0x02	/* eeprom DI pin */;
pub const EECSR_EDO: c_uint = 0x01	/* eeprom DO pin */;
//
// Bits in the EMBCMD register
//
pub const EMBCMD_EDONE: c_uint = 0x80;
pub const EMBCMD_EWDIS: c_uint = 0x08;
pub const EMBCMD_EWEN: c_uint = 0x04;
pub const EMBCMD_EWR: c_uint = 0x02;
pub const EMBCMD_ERD: c_uint = 0x01;
//
// Bits in TESTCFG register
//
pub const TESTCFG_HBDIS: c_uint = 0x80;
//
// Bits in CHIPGCR register
//
pub const CHIPGCR_FCGMII: c_uint = 0x80	/* force GMII (else MII only) */;
pub const CHIPGCR_FCFDX: c_uint = 0x40	/* force full duplex */;
pub const CHIPGCR_FCRESV: c_uint = 0x20;
pub const CHIPGCR_FCMODE: c_uint = 0x10	/* enable MAC forced mode */;
pub const CHIPGCR_LPSOPT: c_uint = 0x08;
pub const CHIPGCR_TM1US: c_uint = 0x04;
pub const CHIPGCR_TM0US: c_uint = 0x02;
pub const CHIPGCR_PHYINTEN: c_uint = 0x01;
//
// Bits in WOLCR0
//
pub const WOLCR_MSWOLEN7: c_uint = 0x0080	/* enable pattern match filtering */;
pub const WOLCR_MSWOLEN6: c_uint = 0x0040;
pub const WOLCR_MSWOLEN5: c_uint = 0x0020;
pub const WOLCR_MSWOLEN4: c_uint = 0x0010;
pub const WOLCR_MSWOLEN3: c_uint = 0x0008;
pub const WOLCR_MSWOLEN2: c_uint = 0x0004;
pub const WOLCR_MSWOLEN1: c_uint = 0x0002;
pub const WOLCR_MSWOLEN0: c_uint = 0x0001;
pub const WOLCR_ARP_EN: c_uint = 0x0001;
//
// Bits in WOLCR1
//
pub const WOLCR_LINKOFF_EN: c_uint = 0x0800	/* link off detected enable */;
pub const WOLCR_LINKON_EN: c_uint = 0x0400	/* link on detected enable */;
pub const WOLCR_MAGIC_EN: c_uint = 0x0200	/* magic packet filter enable */;
pub const WOLCR_UNICAST_EN: c_uint = 0x0100	/* unicast filter enable */;
//
// Bits in PWCFG
//
pub const PWCFG_PHYPWOPT: c_uint = 0x80	/* internal MII I/F timing */;
pub const PWCFG_PCISTICK: c_uint = 0x40	/* PCI sticky R/W enable */;
pub const PWCFG_WOLTYPE: c_uint = 0x20	/* pulse(1) or button (0) */;
pub const PWCFG_LEGCY_WOL: c_uint = 0x10;
pub const PWCFG_PMCSR_PME_SR: c_uint = 0x08;
pub const PWCFG_PMCSR_PME_EN: c_uint = 0x04	/* control by PCISTICK */;
pub const PWCFG_LEGACY_WOLSR: c_uint = 0x02	/* Legacy WOL_SR shadow */;
pub const PWCFG_LEGACY_WOLEN: c_uint = 0x01	/* Legacy WOL_EN shadow */;
//
// Bits in WOLCFG
//
pub const WOLCFG_PMEOVR: c_uint = 0x80	/* for legacy use, force PMEEN always */;
pub const WOLCFG_SAM: c_uint = 0x20	/* accept multicast case reset, default=0 */;
pub const WOLCFG_SAB: c_uint = 0x10	/* accept broadcast case reset, default=0 */;
pub const WOLCFG_SMIIACC: c_uint = 0x08	/* ?? */;
pub const WOLCFG_SGENWH: c_uint = 0x02;
pub const WOLCFG_PHYINTEN: c_uint = 0x01	/* 0:PHYINT trigger enable, 1:use internal MII;
//
// Bits in WOLSR1
//
pub const WOLSR_LINKOFF_INT: c_uint = 0x0800;
pub const WOLSR_LINKON_INT: c_uint = 0x0400;
pub const WOLSR_MAGIC_INT: c_uint = 0x0200;
pub const WOLSR_UNICAST_INT: c_uint = 0x0100;
//
// Ethernet address filter type
//
pub const PKT_TYPE_NONE: c_uint = 0x0000	/* Turn off receiver */;
pub const PKT_TYPE_DIRECTED: c_uint = 0x0001	/* obselete, directed address is always accepted */;
pub const PKT_TYPE_MULTICAST: c_uint = 0x0002;
pub const PKT_TYPE_ALL_MULTICAST: c_uint = 0x0004;
pub const PKT_TYPE_BROADCAST: c_uint = 0x0008;
pub const PKT_TYPE_PROMISCUOUS: c_uint = 0x0020;
pub const PKT_TYPE_LONG: c_uint = 0x2000	/* NOTE.... the definition of LONG is >2048 bytes in our chip */;
pub const PKT_TYPE_RUNT: c_uint = 0x4000;
pub const PKT_TYPE_ERROR: c_uint = 0x8000	/* Accept error packets, e.g. CRC error */;
//
// Loopback mode
//
pub const MAC_LB_NONE: c_uint = 0x00;
pub const MAC_LB_INTERNAL: c_uint = 0x01;
pub const MAC_LB_EXTERNAL: c_uint = 0x02;
//
// Enabled mask value of irq
//

pub const IMR_MASK_VALUE: c_uint = 0x0033FF0FUL	/* initial value of IMR;

pub const IMR_MASK_VALUE: c_uint = 0x0013FB0FUL	/* initial value of IMR;
//

//
// Revision id
//
pub const REV_ID_VT3119_A0: c_uint = 0x00;
pub const REV_ID_VT3119_A1: c_uint = 0x01;
pub const REV_ID_VT3216_A0: c_uint = 0x10;
//
// Max time out delay time
//
pub const W_MAX_TIMEOUT: c_uint = 0x0FFFU;
//
// MAC registers as a structure. Cannot be directly accessed this
// way but generates offsets for readl/writel() calls
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mac_regs {
    pub /: *mut *mut volatile u8 PAR[6]; / 0x00,
    pub RCR: volatile u8,
    pub TCR: volatile u8,
    pub /: *mut *mut volatile __le32 CR0Set; / 0x08,
    pub /: *mut *mut volatile __le32 CR0Clr; / 0x0C,
    pub /: *mut *mut volatile u8 MARCAM[8]; / 0x10,
    pub /: *mut *mut volatile __le32 DecBaseHi; / 0x18,
    pub /: *mut *mut volatile __le16 DbfBaseHi; / 0x1C,
    pub reserved_1E: volatile __le16,
    pub /: *mut *mut volatile __le16 ISRCTL; / 0x20,
    pub TXESR: volatile u8,
    pub RXESR: volatile u8,
    pub /: *mut *mut volatile __le32 ISR; / 0x24,
    pub IMR: volatile __le32,
    pub /: *mut *mut volatile __le32 TDStatusPort; / 0x2C,
    pub /: *mut *mut volatile __le16 TDCSRSet; / 0x30,
    pub RDCSRSet: volatile u8,
    pub reserved_33: volatile u8,
    pub TDCSRClr: volatile __le16,
    pub RDCSRClr: volatile u8,
    pub reserved_37: volatile u8,
    pub /: *mut *mut volatile __le32 RDBaseLo; / 0x38,
    pub /: *mut *mut volatile __le16 RDIdx; / 0x3C,
    pub /: *mut *mut volatile u8 TQETMR; / 0x3E, VT3216 and above only,
    pub /: *mut *mut volatile u8 RQETMR; / 0x3F, VT3216 and above only,
    pub /: *mut *mut volatile __le32 TDBaseLo[4]; / 0x40,
    pub /: *mut *mut volatile __le16 RDCSize; / 0x50,
    pub /: *mut *mut volatile __le16 TDCSize; / 0x52,
    pub /: *mut *mut volatile __le16 TDIdx[4]; / 0x54,
    pub /: *mut *mut volatile __le16 tx_pause_timer; / 0x5C,
    pub /: *mut *mut volatile __le16 RBRDU; / 0x5E,
    pub /: *mut *mut volatile __le32 FIFOTest0; / 0x60,
    pub /: *mut *mut volatile __le32 FIFOTest1; / 0x64,
    pub /: *mut *mut volatile u8 CAMADDR; / 0x68,
    pub /: *mut *mut volatile u8 CAMCR; / 0x69,
    pub /: *mut *mut volatile u8 GFTEST; / 0x6A,
    pub /: *mut *mut volatile u8 FTSTCMD; / 0x6B,
    pub /: *mut *mut volatile u8 MIICFG; / 0x6C,
    pub MIISR: volatile u8,
    pub PHYSR0: volatile u8,
    pub PHYSR1: volatile u8,
    pub MIICR: volatile u8,
    pub MIIADR: volatile u8,
    pub MIIDATA: volatile __le16,
    pub /: *mut *mut volatile __le16 SoftTimer0; / 0x74,
    pub SoftTimer1: volatile __le16,
    pub /: *mut *mut volatile u8 CFGA; / 0x78,
    pub CFGB: volatile u8,
    pub CFGC: volatile u8,
    pub CFGD: volatile u8,
    pub /: *mut *mut volatile __le16 DCFG; / 0x7C,
    pub MCFG: volatile __le16,
    pub /: *mut *mut volatile u8 TBIST; / 0x80,
    pub RBIST: volatile u8,
    pub PMCPORT: volatile u8,
    pub STICKHW: volatile u8,
    pub /: *mut *mut volatile u8 MIBCR; / 0x84,
    pub reserved_85: volatile u8,
    pub rev_id: volatile u8,
    pub PORSTS: volatile u8,
    pub /: *mut *mut volatile __le32 MIBData; / 0x88,
    pub EEWrData: volatile __le16,
    pub reserved_8E: volatile u8,
    pub BPMDWr: volatile u8,
    pub BPCMD: volatile u8,
    pub BPMDRd: volatile u8,
    pub /: *mut *mut volatile u8 EECHKSUM; / 0x92,
    pub EECSR: volatile u8,
    pub /: *mut *mut volatile __le16 EERdData; / 0x94,
    pub EADDR: volatile u8,
    pub EMBCMD: volatile u8,
    pub /: *mut *mut volatile u8 JMPSR0; / 0x98,
    pub JMPSR1: volatile u8,
    pub JMPSR2: volatile u8,
    pub JMPSR3: volatile u8,
    pub /: *mut *mut volatile u8 CHIPGSR; / 0x9C,
    pub TESTCFG: volatile u8,
    pub DEBUG: volatile u8,
    pub CHIPGCR: volatile u8,
    pub /: *mut *mut volatile __le16 WOLCRSet; / 0xA0,
    pub PWCFGSet: volatile u8,
    pub WOLCFGSet: volatile u8,
    pub /: *mut *mut volatile __le16 WOLCRClr; / 0xA4,
    pub PWCFGCLR: volatile u8,
    pub WOLCFGClr: volatile u8,
    pub /: *mut *mut volatile __le16 WOLSRSet; / 0xA8,
    pub reserved_AA: volatile __le16,
    pub /: *mut *mut volatile __le16 WOLSRClr; / 0xAC,
    pub reserved_AE: volatile __le16,
    pub /: *mut *mut volatile __le16 PatternCRC[8]; / 0xB0,
    pub /: *mut *mut volatile __le32 ByteMask[4][4]; / 0xC0,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hw_mib {
    HW_MIB_ifRxAllPkts = 0,
    HW_MIB_ifRxOkPkts,
    HW_MIB_ifTxOkPkts,
    HW_MIB_ifRxErrorPkts,
    HW_MIB_ifRxRuntOkPkt,
    HW_MIB_ifRxRuntErrPkt,
    HW_MIB_ifRx64Pkts,
    HW_MIB_ifTx64Pkts,
    HW_MIB_ifRx65To127Pkts,
    HW_MIB_ifTx65To127Pkts,
    HW_MIB_ifRx128To255Pkts,
    HW_MIB_ifTx128To255Pkts,
    HW_MIB_ifRx256To511Pkts,
    HW_MIB_ifTx256To511Pkts,
    HW_MIB_ifRx512To1023Pkts,
    HW_MIB_ifTx512To1023Pkts,
    HW_MIB_ifRx1024To1518Pkts,
    HW_MIB_ifTx1024To1518Pkts,
    HW_MIB_ifTxEtherCollisions,
    HW_MIB_ifRxPktCRCE,
    HW_MIB_ifRxJumboPkts,
    HW_MIB_ifTxJumboPkts,
    HW_MIB_ifRxMacControlFrames,
    HW_MIB_ifTxMacControlFrames,
    HW_MIB_ifRxPktFAE,
    HW_MIB_ifRxLongOkPkt,
    HW_MIB_ifRxLongPktErrPkt,
    HW_MIB_ifTXSQEErrors,
    HW_MIB_ifRxNobuf,
    HW_MIB_ifRxSymbolErrors,
    HW_MIB_ifInRangeLengthErrors,
    HW_MIB_ifLateCollisions,
    HW_MIB_SIZE
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum chip_type {
    CHIP_TYPE_VT6110 = 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct velocity_info_tbl {
    pub chip_id: chip_type,
    pub name: *const c_char,
    pub txqueue: c_int,
    pub flags: u32,
}

//
// Header for WOL definitions. Used to compute hashes
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arp_packet {
    pub dest_mac: [u8; ETH_ALEN],
    pub src_mac: [u8; ETH_ALEN],
    pub type: __be16,
    pub ar_hrd: __be16,
    pub ar_pro: __be16,
    pub ar_hln: u8,
    pub ar_pln: u8,
    pub ar_op: __be16,
    pub ar_sha: [u8; ETH_ALEN],
    pub ar_sip: [u8; 4],
    pub ar_tha: [u8; ETH_ALEN],
    pub ar_tip: [u8; 4],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _magic_packet {
    pub dest_mac: [u8; 6],
    pub src_mac: [u8; 6],
    pub type: __be16,
    pub MAC: [u8; 16][6],
    pub password: [u8; 6],
    pub __packed: },
//
// Store for chip context when saving and restoring status. Not
// all fields are saved/restored currently.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct velocity_context {
    pub mac_reg: [u8; 256],
    pub cam_addr: [MCAM_ADDR; MCAM_SIZE],
    pub vcam: [u16; VCAM_SIZE],
    pub cammask: [u32; 2],
    pub patcrc: [u32; 2],
    pub pattern: [u32; 8],
}

//
// Registers in the MII (offset unit is WORD)
//
// Marvell 88E1000/88E1000S
pub const MII_REG_PSCR: c_uint = 0x10	// PHY specific control register;
//
// Bits in the Silicon revision register
//
pub const TCSR_ECHODIS: c_uint = 0x2000	//;
pub const AUXCR_MDPPS: c_uint = 0x0004	//;
// Bits in the PLED register
pub const PLED_LALBE: c_uint = 0x0004	//;
// Marvell 88E1000/88E1000S Bits in the PHY specific control register (10h)
pub const PSCR_ACRSTX: c_uint = 0x0800	// Assert CRS on Transmit;
pub const PHYID_CICADA_CS8201: c_uint = 0x000FC410UL;
pub const PHYID_VT3216_32BIT: c_uint = 0x000FC610UL;
pub const PHYID_VT3216_64BIT: c_uint = 0x000FC600UL;
pub const PHYID_MARVELL_1000: c_uint = 0x01410C50UL;
pub const PHYID_MARVELL_1000S: c_uint = 0x01410C40UL;
pub const PHYID_ICPLUS_IP101A: c_uint = 0x02430C54UL;
pub const PHYID_REV_ID_MASK: c_uint = 0x0000000FUL;

pub const VELOCITY_WOL_MAGIC: c_uint = 0x00000000UL;
pub const VELOCITY_WOL_PHY: c_uint = 0x00000001UL;
pub const VELOCITY_WOL_ARP: c_uint = 0x00000002UL;
pub const VELOCITY_WOL_UCAST: c_uint = 0x00000004UL;
pub const VELOCITY_WOL_BCAST: c_uint = 0x00000010UL;
pub const VELOCITY_WOL_MCAST: c_uint = 0x00000020UL;
pub const VELOCITY_WOL_MAGIC_SEC: c_uint = 0x00000040UL;
//
// Flags for options
//
pub const VELOCITY_FLAGS_TAGGING: c_uint = 0x00000001UL;
pub const VELOCITY_FLAGS_RX_CSUM: c_uint = 0x00000004UL;
pub const VELOCITY_FLAGS_IP_ALIGN: c_uint = 0x00000008UL;
pub const VELOCITY_FLAGS_VAL_PKT_LEN: c_uint = 0x00000010UL;
pub const VELOCITY_FLAGS_FLOW_CTRL: c_uint = 0x01000000UL;
//
// Flags for driver status
//
pub const VELOCITY_FLAGS_OPENED: c_uint = 0x00010000UL;
pub const VELOCITY_FLAGS_VMNS_CONNECTED: c_uint = 0x00020000UL;
pub const VELOCITY_FLAGS_VMNS_COMMITTED: c_uint = 0x00040000UL;
pub const VELOCITY_FLAGS_WOL_ENABLED: c_uint = 0x00080000UL;
//
// Flags for MII status
//
pub const VELOCITY_LINK_FAIL: c_uint = 0x00000001UL;
pub const VELOCITY_SPEED_10: c_uint = 0x00000002UL;
pub const VELOCITY_SPEED_100: c_uint = 0x00000004UL;
pub const VELOCITY_SPEED_1000: c_uint = 0x00000008UL;
pub const VELOCITY_DUPLEX_FULL: c_uint = 0x00000010UL;
pub const VELOCITY_AUTONEG_ENABLE: c_uint = 0x00000020UL;
pub const VELOCITY_FORCED_BY_EEPROM: c_uint = 0x00000040UL;
//
// For velocity_set_media_duplex
//
pub const VELOCITY_LINK_CHANGE: c_uint = 0x00000001UL;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum speed_opt {
    SPD_DPX_AUTO = 0,
    SPD_DPX_100_HALF = 1,
    SPD_DPX_100_FULL = 2,
    SPD_DPX_10_HALF = 3,
    SPD_DPX_10_FULL = 4,
    SPD_DPX_1000_FULL = 5
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum velocity_init_type {
    VELOCITY_INIT_COLD = 0,
    VELOCITY_INIT_RESET,
    VELOCITY_INIT_WOL
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum velocity_flow_cntl_type {
    FLOW_CNTL_DEFAULT = 1,
    FLOW_CNTL_TX,
    FLOW_CNTL_RX,
    FLOW_CNTL_TX_RX,
    FLOW_CNTL_DISABLE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct velocity_opt {
    pub /: *mut *mut int numrx; / Number of RX descriptors,
    pub /: *mut *mut int numtx; / Number of TX descriptors,
    pub /: *mut *mut speed_opt spd_dpx; / Media link mode,
    pub /: *mut *mut int DMA_length; / DMA length,
    pub /: *mut *mut int rx_thresh; / RX_THRESH,
    pub flow_cntl: c_int,
    pub /: *mut *mut int wol_opts; / Wake on lan options,
    pub td_int_count: c_int,
    pub int_works: c_int,
    pub rx_bandwidth_hi: c_int,
    pub rx_bandwidth_lo: c_int,
    pub rx_bandwidth_en: c_int,
    pub rxqueue_timer: c_int,
    pub txqueue_timer: c_int,
    pub tx_intsup: c_int,
    pub rx_intsup: c_int,
    pub flags: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct velocity_info {
    pub dev: *mut device,
    pub pdev: *mut pci_dev,
    pub netdev: *mut net_device,
    pub no_eeprom: bool,
    pub active_vlans: [c_ulong; BITS_TO_LONGS(VLAN_N_VID)],
    pub ip_addr: [u8; 4],
    pub chip_id: chip_type,
    pub mac_regs: *mut *mut mac_regs __iomem,
    pub memaddr: c_ulong,
    pub ioaddr: c_ulong,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tx_info {
    pub numq: c_int,
// FIXME: the locality of the data seems rather poor.
    pub used: [c_int; TX_QUEUE_NO],
    pub curr: [c_int; TX_QUEUE_NO],
    pub tail: [c_int; TX_QUEUE_NO],
    pub rings: [*mut tx_desc; TX_QUEUE_NO],
    pub infos: [*mut velocity_td_info; TX_QUEUE_NO],
    pub pool_dma: [dma_addr_t; TX_QUEUE_NO],
    pub tx: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rx_info {
    pub buf_sz: c_int,
    pub dirty: c_int,
    pub curr: c_int,
    pub filled: u32,
    pub ring: *mut rx_desc,
    pub /: *mut *mut *mut velocity_rd_info info; / It's an array,
    pub pool_dma: dma_addr_t,
    pub rx: },
    pub mib_counter: [u32; MAX_HW_MIB_COUNTER],
    pub options: velocity_opt,
    pub int_mask: u32,
    pub flags: u32,
    pub mii_status: u32,
    pub phy_id: u32,
    pub multicast_limit: c_int,
    pub 8)]: u8 vCAMmask[(VCAM_SIZE /,
    pub 8)]: u8 mCAMmask[(MCAM_SIZE /,
    pub lock: spinlock_t,
    pub wol_opts: c_int,
    pub wol_passwd: [u8; 6],
    pub context: velocity_context,
    pub ticks: u32,
    pub ethtool_ops_nesting: u32,
    pub rev_id: u8,
    pub napi: napi_struct,
}

//
// velocity_get_ip		-	find an IP address for the device
// @vptr: Velocity to query
//
// Dig out an IP address for this interface so that we can
// configure wakeup with WOL for ARP. If there are multiple IP
// addresses on this chain then we use the first - multi-IP WOL is not
// supported.
//
// velocity_update_hw_mibs	-	fetch MIB counters from chip
// @vptr: velocity to update
//
// The velocity hardware keeps certain counters in the hardware
// side. We need to read these when the user asks for statistics
// or when they overflow (causing an interrupt). The read of the
// statistic clears it, so we keep running master counters in user
// space.
//
// init_flow_control_register 	-	set up flow control
// @vptr: velocity to configure
//
// Configure the flow control registers for this velocity device.
//
// Set {XHITH1, XHITH0, XLTH1, XLTH0} in FlowCR1 to {1, 0, 1, 1}
// Set TxPauseTimer to 0xFFFF
// Initialize RBRDU to Rx buffer count.
