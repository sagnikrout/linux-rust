//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/reg_booke.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Contains register definitions common to the Book E PowerPC
// specification.
//
// Copyright 2009-2010 Freescale Semiconductor, Inc.
//

// Machine State Register (MSR) Fields

// Special Purpose Registers (SPRNs)
pub const SPRN_DECAR: c_uint = 0x036	/* Decrementer Auto Reload Register */;
pub const SPRN_IVPR: c_uint = 0x03F	/* Interrupt Vector Prefix Register */;
pub const SPRN_USPRG0: c_uint = 0x100	/* User Special Purpose Register General 0 */;
pub const SPRN_SPRG3R: c_uint = 0x103	/* Special Purpose Register General 3 Read */;
pub const SPRN_SPRG4R: c_uint = 0x104	/* Special Purpose Register General 4 Read */;
pub const SPRN_SPRG5R: c_uint = 0x105	/* Special Purpose Register General 5 Read */;
pub const SPRN_SPRG6R: c_uint = 0x106	/* Special Purpose Register General 6 Read */;
pub const SPRN_SPRG7R: c_uint = 0x107	/* Special Purpose Register General 7 Read */;
pub const SPRN_SPRG4W: c_uint = 0x114	/* Special Purpose Register General 4 Write */;
pub const SPRN_SPRG5W: c_uint = 0x115	/* Special Purpose Register General 5 Write */;
pub const SPRN_SPRG6W: c_uint = 0x116	/* Special Purpose Register General 6 Write */;
pub const SPRN_SPRG7W: c_uint = 0x117	/* Special Purpose Register General 7 Write */;
pub const SPRN_EPCR: c_uint = 0x133	/* Embedded Processor Control Register */;
pub const SPRN_DBCR2: c_uint = 0x136	/* Debug Control Register 2 */;
pub const SPRN_DBCR4: c_uint = 0x233	/* Debug Control Register 4 */;
pub const SPRN_MSRP: c_uint = 0x137	/* MSR Protect Register */;
pub const SPRN_IAC3: c_uint = 0x13A	/* Instruction Address Compare 3 */;
pub const SPRN_IAC4: c_uint = 0x13B	/* Instruction Address Compare 4 */;
pub const SPRN_DVC1: c_uint = 0x13E	/* Data Value Compare Register 1 */;
pub const SPRN_DVC2: c_uint = 0x13F	/* Data Value Compare Register 2 */;
pub const SPRN_LPID: c_uint = 0x152	/* Logical Partition ID */;
pub const SPRN_MAS8: c_uint = 0x155	/* MMU Assist Register 8 */;
pub const SPRN_TLB0PS: c_uint = 0x158	/* TLB 0 Page Size Register */;
pub const SPRN_TLB1PS: c_uint = 0x159	/* TLB 1 Page Size Register */;
pub const SPRN_MAS5_MAS6: c_uint = 0x15c	/* MMU Assist Register 5 || 6 */;
pub const SPRN_MAS8_MAS1: c_uint = 0x15d	/* MMU Assist Register 8 || 1 */;
pub const SPRN_EPTCFG: c_uint = 0x15e	/* Embedded Page Table Config */;
pub const SPRN_GSPRG0: c_uint = 0x170	/* Guest SPRG0 */;
pub const SPRN_GSPRG1: c_uint = 0x171	/* Guest SPRG1 */;
pub const SPRN_GSPRG2: c_uint = 0x172	/* Guest SPRG2 */;
pub const SPRN_GSPRG3: c_uint = 0x173	/* Guest SPRG3 */;
pub const SPRN_MAS7_MAS3: c_uint = 0x174	/* MMU Assist Register 7 || 3 */;
pub const SPRN_MAS0_MAS1: c_uint = 0x175	/* MMU Assist Register 0 || 1 */;
pub const SPRN_GSRR0: c_uint = 0x17A	/* Guest SRR0 */;
pub const SPRN_GSRR1: c_uint = 0x17B	/* Guest SRR1 */;
pub const SPRN_GEPR: c_uint = 0x17C	/* Guest EPR */;
pub const SPRN_GDEAR: c_uint = 0x17D	/* Guest DEAR */;
pub const SPRN_GPIR: c_uint = 0x17E	/* Guest PIR */;
pub const SPRN_GESR: c_uint = 0x17F	/* Guest Exception Syndrome Register */;
pub const SPRN_IVOR0: c_uint = 0x190	/* Interrupt Vector Offset Register 0 */;
pub const SPRN_IVOR1: c_uint = 0x191	/* Interrupt Vector Offset Register 1 */;
pub const SPRN_IVOR2: c_uint = 0x192	/* Interrupt Vector Offset Register 2 */;
pub const SPRN_IVOR3: c_uint = 0x193	/* Interrupt Vector Offset Register 3 */;
pub const SPRN_IVOR4: c_uint = 0x194	/* Interrupt Vector Offset Register 4 */;
pub const SPRN_IVOR5: c_uint = 0x195	/* Interrupt Vector Offset Register 5 */;
pub const SPRN_IVOR6: c_uint = 0x196	/* Interrupt Vector Offset Register 6 */;
pub const SPRN_IVOR7: c_uint = 0x197	/* Interrupt Vector Offset Register 7 */;
pub const SPRN_IVOR8: c_uint = 0x198	/* Interrupt Vector Offset Register 8 */;
pub const SPRN_IVOR9: c_uint = 0x199	/* Interrupt Vector Offset Register 9 */;
pub const SPRN_IVOR10: c_uint = 0x19A	/* Interrupt Vector Offset Register 10 */;
pub const SPRN_IVOR11: c_uint = 0x19B	/* Interrupt Vector Offset Register 11 */;
pub const SPRN_IVOR12: c_uint = 0x19C	/* Interrupt Vector Offset Register 12 */;
pub const SPRN_IVOR13: c_uint = 0x19D	/* Interrupt Vector Offset Register 13 */;
pub const SPRN_IVOR14: c_uint = 0x19E	/* Interrupt Vector Offset Register 14 */;
pub const SPRN_IVOR15: c_uint = 0x19F	/* Interrupt Vector Offset Register 15 */;
pub const SPRN_IVOR38: c_uint = 0x1B0	/* Interrupt Vector Offset Register 38 */;
pub const SPRN_IVOR39: c_uint = 0x1B1	/* Interrupt Vector Offset Register 39 */;
pub const SPRN_IVOR40: c_uint = 0x1B2	/* Interrupt Vector Offset Register 40 */;
pub const SPRN_IVOR41: c_uint = 0x1B3	/* Interrupt Vector Offset Register 41 */;
pub const SPRN_IVOR42: c_uint = 0x1B4	/* Interrupt Vector Offset Register 42 */;
pub const SPRN_GIVOR2: c_uint = 0x1B8	/* Guest IVOR2 */;
pub const SPRN_GIVOR3: c_uint = 0x1B9	/* Guest IVOR3 */;
pub const SPRN_GIVOR4: c_uint = 0x1BA	/* Guest IVOR4 */;
pub const SPRN_GIVOR8: c_uint = 0x1BB	/* Guest IVOR8 */;
pub const SPRN_GIVOR13: c_uint = 0x1BC	/* Guest IVOR13 */;
pub const SPRN_GIVOR14: c_uint = 0x1BD	/* Guest IVOR14 */;
pub const SPRN_GIVPR: c_uint = 0x1BF	/* Guest IVPR */;
pub const SPRN_SPEFSCR: c_uint = 0x200	/* SPE & Embedded FP Status & Control */;
pub const SPRN_BBEAR: c_uint = 0x201	/* Branch Buffer Entry Address Register */;
pub const SPRN_BBTAR: c_uint = 0x202	/* Branch Buffer Target Address Register */;
pub const SPRN_L1CFG0: c_uint = 0x203	/* L1 Cache Configure Register 0 */;
pub const SPRN_L1CFG1: c_uint = 0x204	/* L1 Cache Configure Register 1 */;
pub const SPRN_ATB: c_uint = 0x20E	/* Alternate Time Base */;
pub const SPRN_ATBL: c_uint = 0x20E	/* Alternate Time Base Lower */;
pub const SPRN_ATBU: c_uint = 0x20F	/* Alternate Time Base Upper */;
pub const SPRN_IVOR32: c_uint = 0x210	/* Interrupt Vector Offset Register 32 */;
pub const SPRN_IVOR33: c_uint = 0x211	/* Interrupt Vector Offset Register 33 */;
pub const SPRN_IVOR34: c_uint = 0x212	/* Interrupt Vector Offset Register 34 */;
pub const SPRN_IVOR35: c_uint = 0x213	/* Interrupt Vector Offset Register 35 */;
pub const SPRN_IVOR36: c_uint = 0x214	/* Interrupt Vector Offset Register 36 */;
pub const SPRN_IVOR37: c_uint = 0x215	/* Interrupt Vector Offset Register 37 */;
pub const SPRN_MCARU: c_uint = 0x239	/* Machine Check Address Register Upper */;
pub const SPRN_MCSRR0: c_uint = 0x23A	/* Machine Check Save and Restore Register 0 */;
pub const SPRN_MCSRR1: c_uint = 0x23B	/* Machine Check Save and Restore Register 1 */;
pub const SPRN_MCSR: c_uint = 0x23C	/* Machine Check Status Register */;
pub const SPRN_MCAR: c_uint = 0x23D	/* Machine Check Address Register */;
pub const SPRN_DSRR0: c_uint = 0x23E	/* Debug Save and Restore Register 0 */;
pub const SPRN_DSRR1: c_uint = 0x23F	/* Debug Save and Restore Register 1 */;
pub const SPRN_SPRG8: c_uint = 0x25C	/* Special Purpose Register General 8 */;
pub const SPRN_SPRG9: c_uint = 0x25D	/* Special Purpose Register General 9 */;
pub const SPRN_L1CSR2: c_uint = 0x25E	/* L1 Cache Control and Status Register 2 */;
pub const SPRN_MAS0: c_uint = 0x270	/* MMU Assist Register 0 */;
pub const SPRN_MAS1: c_uint = 0x271	/* MMU Assist Register 1 */;
pub const SPRN_MAS2: c_uint = 0x272	/* MMU Assist Register 2 */;
pub const SPRN_MAS3: c_uint = 0x273	/* MMU Assist Register 3 */;
pub const SPRN_MAS4: c_uint = 0x274	/* MMU Assist Register 4 */;
pub const SPRN_MAS5: c_uint = 0x153	/* MMU Assist Register 5 */;
pub const SPRN_MAS6: c_uint = 0x276	/* MMU Assist Register 6 */;
pub const SPRN_PID1: c_uint = 0x279	/* Process ID Register 1 */;
pub const SPRN_PID2: c_uint = 0x27A	/* Process ID Register 2 */;
pub const SPRN_TLB0CFG: c_uint = 0x2B0	/* TLB 0 Config Register */;
pub const SPRN_TLB1CFG: c_uint = 0x2B1	/* TLB 1 Config Register */;
pub const SPRN_TLB2CFG: c_uint = 0x2B2	/* TLB 2 Config Register */;
pub const SPRN_TLB3CFG: c_uint = 0x2B3	/* TLB 3 Config Register */;
pub const SPRN_EPR: c_uint = 0x2BE	/* External Proxy Register */;
pub const SPRN_CCR1: c_uint = 0x378	/* Core Configuration Register 1 */;
pub const SPRN_MAS7: c_uint = 0x3B0	/* MMU Assist Register 7 */;
pub const SPRN_MMUCR: c_uint = 0x3B2	/* MMU Control Register */;
pub const SPRN_CCR0: c_uint = 0x3B3	/* Core Configuration Register 0 */;
pub const SPRN_EPLC: c_uint = 0x3B3	/* External Process ID Load Context */;
pub const SPRN_EPSC: c_uint = 0x3B4	/* External Process ID Store Context */;
pub const SPRN_SGR: c_uint = 0x3B9	/* Storage Guarded Register */;
pub const SPRN_DCWR: c_uint = 0x3BA	/* Data Cache Write-thru Register */;
pub const SPRN_SLER: c_uint = 0x3BB	/* Little-endian real mode */;
pub const SPRN_DCMP: c_uint = 0x3D1	/* Data TLB Compare Register */;
pub const SPRN_ICDBDR: c_uint = 0x3D3	/* Instruction Cache Debug Data Register */;
pub const SPRN_EVPR: c_uint = 0x3D6	/* Exception Vector Prefix Register */;
pub const SPRN_L1CSR0: c_uint = 0x3F2	/* L1 Cache Control and Status Register 0 */;
pub const SPRN_L1CSR1: c_uint = 0x3F3	/* L1 Cache Control and Status Register 1 */;
pub const SPRN_MMUCSR0: c_uint = 0x3F4	/* MMU Control and Status Register 0 */;
pub const SPRN_MMUCFG: c_uint = 0x3F7	/* MMU Configuration Register */;
pub const SPRN_BUCSR: c_uint = 0x3F5	/* Branch Unit Control and Status */;
pub const SPRN_L2CSR0: c_uint = 0x3F9	/* L2 Data Cache Control and Status Register 0 */;
pub const SPRN_L2CSR1: c_uint = 0x3FA	/* L2 Data Cache Control and Status Register 1 */;
pub const SPRN_DCCR: c_uint = 0x3FA	/* Data Cache Cacheability Register */;
pub const SPRN_ICCR: c_uint = 0x3FB	/* Instruction Cache Cacheability Register */;
pub const SPRN_PWRMGTCR0: c_uint = 0x3FB	/* Power management control register 0 */;
pub const SPRN_SVR: c_uint = 0x3FF	/* System Version Register */;
//
// SPRs which have conflicting definitions on true Book E versus classic.
//
pub const SPRN_CSRR0: c_uint = 0x03A	/* Critical Save and Restore Register 0 */;
pub const SPRN_CSRR1: c_uint = 0x03B	/* Critical Save and Restore Register 1 */;
pub const SPRN_DEAR: c_uint = 0x03D	/* Data Error Address Register */;
pub const SPRN_ESR: c_uint = 0x03E	/* Exception Syndrome Register */;
pub const SPRN_PIR: c_uint = 0x11E	/* Processor Identification Register */;
pub const SPRN_DBSR: c_uint = 0x130	/* Debug Status Register */;
pub const SPRN_DBCR0: c_uint = 0x134	/* Debug Control Register 0 */;
pub const SPRN_DBCR1: c_uint = 0x135	/* Debug Control Register 1 */;
pub const SPRN_IAC1: c_uint = 0x138	/* Instruction Address Compare 1 */;
pub const SPRN_IAC2: c_uint = 0x139	/* Instruction Address Compare 2 */;
pub const SPRN_DAC1: c_uint = 0x13C	/* Data Address Compare 1 */;
pub const SPRN_DAC2: c_uint = 0x13D	/* Data Address Compare 2 */;
pub const SPRN_TSR: c_uint = 0x150	/* Timer Status Register */;
pub const SPRN_TCR: c_uint = 0x154	/* Timer Control Register */;
pub const SPRN_HACOP: c_uint = 0x15F	/* Hypervisor Available Coprocessor Register */;
// Bit definitions for CCR1.
pub const CCR1_DPC: c_uint = 0x00000100 /* Disable L1 I-Cache/D-Cache parity checking */;
pub const CCR1_TCS: c_uint = 0x00000080 /* Timer Clock Select */;
// Bit definitions for PWRMGTCR0.

pub const PWRMGTCR0_PW20_ENT_SHIFT: c_int = 8;
pub const PWRMGTCR0_PW20_ENT: c_uint = 0x3F00;

pub const PWRMGTCR0_AV_IDLE_CNT_SHIFT: c_int = 16;
pub const PWRMGTCR0_AV_IDLE_CNT: c_uint = 0x3F0000;
// Bit definitions for the MCSR.
pub const MCSR_MCS: c_uint = 0x80000000 /* Machine Check Summary */;
pub const MCSR_IB: c_uint = 0x40000000 /* Instruction PLB Error */;
pub const MCSR_DRB: c_uint = 0x20000000 /* Data Read PLB Error */;
pub const MCSR_DWB: c_uint = 0x10000000 /* Data Write PLB Error */;
pub const MCSR_TLBP: c_uint = 0x08000000 /* TLB Parity Error */;
pub const MCSR_ICP: c_uint = 0x04000000 /* I-Cache Parity Error */;
pub const MCSR_DCSP: c_uint = 0x02000000 /* D-Cache Search Parity Error */;
pub const MCSR_DCFP: c_uint = 0x01000000 /* D-Cache Flush Parity Error */;
pub const MCSR_IMPE: c_uint = 0x00800000 /* Imprecise Machine Check Exception */;
pub const PPC47x_MCSR_GPR: c_uint = 0x01000000 /* GPR parity error */;
pub const PPC47x_MCSR_FPR: c_uint = 0x00800000 /* FPR parity error */;
pub const PPC47x_MCSR_IPR: c_uint = 0x00400000 /* Imprecise Machine Check Exception */;

// All e500
pub const MCSR_MCP: c_uint = 0x80000000UL /* Machine Check Input Pin */;
pub const MCSR_ICPERR: c_uint = 0x40000000UL /* I-Cache Parity Error */;
// e500v1/v2
pub const MCSR_DCP_PERR: c_uint = 0x20000000UL /* D-Cache Push Parity Error */;
pub const MCSR_DCPERR: c_uint = 0x10000000UL /* D-Cache Parity Error */;
pub const MCSR_BUS_IAERR: c_uint = 0x00000080UL /* Instruction Address Error */;
pub const MCSR_BUS_RAERR: c_uint = 0x00000040UL /* Read Address Error */;
pub const MCSR_BUS_WAERR: c_uint = 0x00000020UL /* Write Address Error */;
pub const MCSR_BUS_IBERR: c_uint = 0x00000010UL /* Instruction Data Error */;
pub const MCSR_BUS_RBERR: c_uint = 0x00000008UL /* Read Data Bus Error */;
pub const MCSR_BUS_WBERR: c_uint = 0x00000004UL /* Write Data Bus Error */;
pub const MCSR_BUS_IPERR: c_uint = 0x00000002UL /* Instruction parity Error */;
pub const MCSR_BUS_RPERR: c_uint = 0x00000001UL /* Read parity Error */;
// e500mc
pub const MCSR_DCPERR_MC: c_uint = 0x20000000UL /* D-Cache Parity Error */;
pub const MCSR_L2MMU_MHIT: c_uint = 0x08000000UL /* Hit on multiple TLB entries */;
pub const MCSR_NMI: c_uint = 0x00100000UL /* Non-Maskable Interrupt */;
pub const MCSR_MAV: c_uint = 0x00080000UL /* MCAR address valid */;
pub const MCSR_MEA: c_uint = 0x00040000UL /* MCAR is effective address */;
pub const MCSR_IF: c_uint = 0x00010000UL /* Instruction Fetch */;
pub const MCSR_LD: c_uint = 0x00008000UL /* Load */;
pub const MCSR_ST: c_uint = 0x00004000UL /* Store */;
pub const MCSR_LDG: c_uint = 0x00002000UL /* Guarded Load */;
pub const MCSR_TLBSYNC: c_uint = 0x00000002UL /* Multiple tlbsyncs detected */;
pub const MCSR_BSL2_ERR: c_uint = 0x00000001UL /* Backside L2 cache error */;
pub const MSRP_UCLEP: c_uint = 0x04000000 /* Protect MSR[UCLE] */;
pub const MSRP_DEP: c_uint = 0x00000200 /* Protect MSR[DE] */;
pub const MSRP_PMMP: c_uint = 0x00000004 /* Protect MSR[PMM] */;

// Bit definitions for the HID1

// e500v1/v2
pub const HID1_PLL_CFG_MASK: c_uint = 0xfc000000	/* PLL_CFG input pins */;
pub const HID1_RFXE: c_uint = 0x00020000	/* Read fault exception enable */;
pub const HID1_R1DPE: c_uint = 0x00008000	/* R1 data bus parity enable */;
pub const HID1_R2DPE: c_uint = 0x00004000	/* R2 data bus parity enable */;
pub const HID1_ASTME: c_uint = 0x00002000	/* Address bus streaming mode enable */;
pub const HID1_ABE: c_uint = 0x00001000	/* Address broadcast enable */;
pub const HID1_MPXTT: c_uint = 0x00000400	/* MPX re-map transfer type */;
pub const HID1_ATS: c_uint = 0x00000080	/* Atomic status */;
pub const HID1_MID_MASK: c_uint = 0x0000000f	/* MID input pins */;

// Bit definitions for the DBSR.
pub const DBSR_IDE: c_uint = 0x80000000	/* Imprecise Debug Event */;
pub const DBSR_MRR: c_uint = 0x30000000	/* Most Recent Reset */;
pub const DBSR_IC: c_uint = 0x08000000	/* Instruction Completion */;
pub const DBSR_BT: c_uint = 0x04000000	/* Branch Taken */;
pub const DBSR_IRPT: c_uint = 0x02000000	/* Exception Debug Event */;
pub const DBSR_TIE: c_uint = 0x01000000	/* Trap Instruction Event */;
pub const DBSR_IAC1: c_uint = 0x00800000	/* Instr Address Compare 1 Event */;
pub const DBSR_IAC2: c_uint = 0x00400000	/* Instr Address Compare 2 Event */;
pub const DBSR_IAC3: c_uint = 0x00200000	/* Instr Address Compare 3 Event */;
pub const DBSR_IAC4: c_uint = 0x00100000	/* Instr Address Compare 4 Event */;
pub const DBSR_DAC1R: c_uint = 0x00080000	/* Data Addr Compare 1 Read Event */;
pub const DBSR_DAC1W: c_uint = 0x00040000	/* Data Addr Compare 1 Write Event */;
pub const DBSR_DAC2R: c_uint = 0x00020000	/* Data Addr Compare 2 Read Event */;
pub const DBSR_DAC2W: c_uint = 0x00010000	/* Data Addr Compare 2 Write Event */;
pub const DBSR_RET: c_uint = 0x00008000	/* Return Debug Event */;
pub const DBSR_CIRPT: c_uint = 0x00000040	/* Critical Interrupt Taken Event */;
pub const DBSR_CRET: c_uint = 0x00000020	/* Critical Return Debug Event */;
pub const DBSR_IAC12ATS: c_uint = 0x00000002	/* Instr Address Compare 1/2 Toggle */;
pub const DBSR_IAC34ATS: c_uint = 0x00000001	/* Instr Address Compare 3/4 Toggle */;
// Bit definitions related to the ESR.
pub const ESR_MCI: c_uint = 0x80000000	/* Machine Check - Instruction */;
pub const ESR_IMCP: c_uint = 0x80000000	/* Instr. Machine Check - Protection */;
pub const ESR_IMCN: c_uint = 0x40000000	/* Instr. Machine Check - Non-config */;
pub const ESR_IMCB: c_uint = 0x20000000	/* Instr. Machine Check - Bus error */;
pub const ESR_IMCT: c_uint = 0x10000000	/* Instr. Machine Check - Timeout */;
pub const ESR_PIL: c_uint = 0x08000000	/* Program Exception - Illegal */;
pub const ESR_PPR: c_uint = 0x04000000	/* Program Exception - Privileged */;
pub const ESR_PTR: c_uint = 0x02000000	/* Program Exception - Trap */;
pub const ESR_FP: c_uint = 0x01000000	/* Floating Point Operation */;
pub const ESR_DST: c_uint = 0x00800000	/* Storage Exception - Data miss */;
pub const ESR_DIZ: c_uint = 0x00400000	/* Storage Exception - Zone fault */;
pub const ESR_ST: c_uint = 0x00800000	/* Store Operation */;
pub const ESR_DLK: c_uint = 0x00200000	/* Data Cache Locking */;
pub const ESR_ILK: c_uint = 0x00100000	/* Instr. Cache Locking */;
pub const ESR_PUO: c_uint = 0x00040000	/* Unimplemented Operation exception */;
pub const ESR_BO: c_uint = 0x00020000	/* Byte Ordering */;
pub const ESR_SPV: c_uint = 0x00000080	/* Signal Processing operation */;
// Bit definitions related to the DBCR0.
pub const DBCR0_EDM: c_uint = 0x80000000	/* External Debug Mode */;
pub const DBCR0_IDM: c_uint = 0x40000000	/* Internal Debug Mode */;
pub const DBCR0_RST: c_uint = 0x30000000	/* all the bits in the RST field */;
// DBCR0_RST_* is 44x specific and not followed in fsl booke
pub const DBCR0_RST_SYSTEM: c_uint = 0x30000000	/* System Reset */;
pub const DBCR0_RST_CHIP: c_uint = 0x20000000	/* Chip Reset */;
pub const DBCR0_RST_CORE: c_uint = 0x10000000	/* Core Reset */;
pub const DBCR0_RST_NONE: c_uint = 0x00000000	/* No Reset */;
pub const DBCR0_ICMP: c_uint = 0x08000000	/* Instruction Completion */;

pub const DBCR0_BRT: c_uint = 0x04000000	/* Branch Taken */;

pub const DBCR0_IRPT: c_uint = 0x02000000	/* Exception Debug Event */;
pub const DBCR0_TDE: c_uint = 0x01000000	/* TRAP Debug Event */;

pub const DBCR0_IAC1: c_uint = 0x00800000	/* Instr Addr compare 1 enable */;
pub const DBCR0_IAC2: c_uint = 0x00400000	/* Instr Addr compare 2 enable */;
pub const DBCR0_IAC3: c_uint = 0x00200000	/* Instr Addr compare 3 enable */;
pub const DBCR0_IAC4: c_uint = 0x00100000	/* Instr Addr compare 4 enable */;
pub const DBCR0_DAC1R: c_uint = 0x00080000	/* DAC 1 Read enable */;
pub const DBCR0_DAC1W: c_uint = 0x00040000	/* DAC 1 Write enable */;
pub const DBCR0_DAC2R: c_uint = 0x00020000	/* DAC 2 Read enable */;
pub const DBCR0_DAC2W: c_uint = 0x00010000	/* DAC 2 Write enable */;
pub const DBCR0_RET: c_uint = 0x00008000	/* Return Debug Event */;
pub const DBCR0_CIRPT: c_uint = 0x00000040	/* Critical Interrupt Taken Event */;
pub const DBCR0_CRET: c_uint = 0x00000020	/* Critical Return Debug Event */;
pub const DBCR0_FT: c_uint = 0x00000001	/* Freeze Timers on debug event */;

// Bit definitions related to the DBCR1.
pub const DBCR1_IAC1US: c_uint = 0xC0000000	/* Instr Addr Cmp 1 Sup/User   */;
pub const DBCR1_IAC1ER: c_uint = 0x30000000	/* Instr Addr Cmp 1 Eff/Real */;
pub const DBCR1_IAC1ER_01: c_uint = 0x10000000	/* reserved */;
pub const DBCR1_IAC1ER_10: c_uint = 0x20000000	/* Instr Addr Cmp 1 Eff/Real MSR[IS]=0 */;
pub const DBCR1_IAC1ER_11: c_uint = 0x30000000	/* Instr Addr Cmp 1 Eff/Real MSR[IS]=1 */;
pub const DBCR1_IAC2US: c_uint = 0x0C000000	/* Instr Addr Cmp 2 Sup/User   */;
pub const DBCR1_IAC2ER: c_uint = 0x03000000	/* Instr Addr Cmp 2 Eff/Real */;
pub const DBCR1_IAC2ER_01: c_uint = 0x01000000	/* reserved */;
pub const DBCR1_IAC2ER_10: c_uint = 0x02000000	/* Instr Addr Cmp 2 Eff/Real MSR[IS]=0 */;
pub const DBCR1_IAC2ER_11: c_uint = 0x03000000	/* Instr Addr Cmp 2 Eff/Real MSR[IS]=1 */;
pub const DBCR1_IAC12M: c_uint = 0x00800000	/* Instr Addr 1-2 range enable */;
pub const DBCR1_IAC12MX: c_uint = 0x00C00000	/* Instr Addr 1-2 range eXclusive */;
pub const DBCR1_IAC12AT: c_uint = 0x00010000	/* Instr Addr 1-2 range Toggle */;
pub const DBCR1_IAC3US: c_uint = 0x0000C000	/* Instr Addr Cmp 3 Sup/User   */;
pub const DBCR1_IAC3ER: c_uint = 0x00003000	/* Instr Addr Cmp 3 Eff/Real */;
pub const DBCR1_IAC3ER_01: c_uint = 0x00001000	/* reserved */;
pub const DBCR1_IAC3ER_10: c_uint = 0x00002000	/* Instr Addr Cmp 3 Eff/Real MSR[IS]=0 */;
pub const DBCR1_IAC3ER_11: c_uint = 0x00003000	/* Instr Addr Cmp 3 Eff/Real MSR[IS]=1 */;
pub const DBCR1_IAC4US: c_uint = 0x00000C00	/* Instr Addr Cmp 4 Sup/User   */;
pub const DBCR1_IAC4ER: c_uint = 0x00000300	/* Instr Addr Cmp 4 Eff/Real */;
pub const DBCR1_IAC4ER_01: c_uint = 0x00000100	/* Instr Addr Cmp 4 Eff/Real MSR[IS]=0 */;
pub const DBCR1_IAC4ER_10: c_uint = 0x00000200	/* Instr Addr Cmp 4 Eff/Real MSR[IS]=0 */;
pub const DBCR1_IAC4ER_11: c_uint = 0x00000300	/* Instr Addr Cmp 4 Eff/Real MSR[IS]=1 */;
pub const DBCR1_IAC34M: c_uint = 0x00000080	/* Instr Addr 3-4 range enable */;
pub const DBCR1_IAC34MX: c_uint = 0x000000C0	/* Instr Addr 3-4 range eXclusive */;
pub const DBCR1_IAC34AT: c_uint = 0x00000001	/* Instr Addr 3-4 range Toggle */;

// Bit definitions related to the DBCR2.
pub const DBCR2_DAC1US: c_uint = 0xC0000000	/* Data Addr Cmp 1 Sup/User   */;
pub const DBCR2_DAC1ER: c_uint = 0x30000000	/* Data Addr Cmp 1 Eff/Real */;
pub const DBCR2_DAC2US: c_uint = 0x0C000000	/* Data Addr Cmp 2 Sup/User   */;
pub const DBCR2_DAC2ER: c_uint = 0x03000000	/* Data Addr Cmp 2 Eff/Real */;
pub const DBCR2_DAC12M: c_uint = 0x00800000	/* DAC 1-2 range enable */;
pub const DBCR2_DAC12MM: c_uint = 0x00400000	/* DAC 1-2 Mask mode*/;
pub const DBCR2_DAC12MX: c_uint = 0x00C00000	/* DAC 1-2 range eXclusive */;
pub const DBCR2_DAC12MODE: c_uint = 0x00C00000	/* DAC 1-2 Mode Bits */;
pub const DBCR2_DAC12A: c_uint = 0x00200000	/* DAC 1-2 Asynchronous */;
pub const DBCR2_DVC1M: c_uint = 0x000C0000	/* Data Value Comp 1 Mode */;

pub const DBCR2_DVC2M: c_uint = 0x00030000	/* Data Value Comp 2 Mode */;

pub const DBCR2_DVC1BE: c_uint = 0x00000F00	/* Data Value Comp 1 Byte */;

pub const DBCR2_DVC2BE: c_uint = 0x0000000F	/* Data Value Comp 2 Byte */;

//
// Are there any active Debug Events represented in the
// Debug Control Registers?
//

pub const DBCR1_ACTIVE_EVENTS: c_int = 0;

// Bit definitions related to the TCR.

pub const TCR_WIE: c_uint = 0x08000000	/* WDT Interrupt Enable */;
pub const TCR_PIE: c_uint = 0x04000000	/* PIT Interrupt Enable */;

pub const TCR_FIE: c_uint = 0x00800000	/* FIT Interrupt Enable */;
pub const TCR_ARE: c_uint = 0x00400000	/* Auto Reload Enable */;

// Bit definitions for the TSR.
pub const TSR_ENW: c_uint = 0x80000000	/* Enable Next Watchdog */;
pub const TSR_WIS: c_uint = 0x40000000	/* WDT Interrupt Status */;

pub const TSR_PIS: c_uint = 0x08000000	/* PIT Interrupt Status */;

pub const TSR_FIS: c_uint = 0x04000000	/* FIT Interrupt Status */;
// Bit definitions for the DCCR.

// Bit definitions for DCWR.

// Bit definitions for ICCR.

// Bit definitions for L1CSR0.
pub const L1CSR0_CPE: c_uint = 0x00010000	/* Data Cache Parity Enable */;
pub const L1CSR0_CUL: c_uint = 0x00000400	/* Data Cache Unable to Lock */;
pub const L1CSR0_CLFC: c_uint = 0x00000100	/* Cache Lock Bits Flash Clear */;
pub const L1CSR0_DCFI: c_uint = 0x00000002	/* Data Cache Flash Invalidate */;
pub const L1CSR0_CFI: c_uint = 0x00000002	/* Cache Flash Invalidate */;
pub const L1CSR0_DCE: c_uint = 0x00000001	/* Data Cache Enable */;
// Bit definitions for L1CSR1.
pub const L1CSR1_CPE: c_uint = 0x00010000	/* Instruction Cache Parity Enable */;
pub const L1CSR1_ICLFR: c_uint = 0x00000100	/* Instr Cache Lock Bits Flash Reset */;
pub const L1CSR1_ICFI: c_uint = 0x00000002	/* Instr Cache Flash Invalidate */;
pub const L1CSR1_ICE: c_uint = 0x00000001	/* Instr Cache Enable */;
// Bit definitions for L1CSR2.
pub const L1CSR2_DCWS: c_uint = 0x40000000	/* Data Cache write shadow */;
// Bit definitions for BUCSR.
pub const BUCSR_STAC_EN: c_uint = 0x01000000	/* Segment Target Address Cache */;
pub const BUCSR_LS_EN: c_uint = 0x00400000	/* Link Stack */;
pub const BUCSR_BBFI: c_uint = 0x00000200	/* Branch Buffer flash invalidate */;
pub const BUCSR_BPEN: c_uint = 0x00000001	/* Branch prediction enable */;

// Bit definitions for L2CSR0.
pub const L2CSR0_L2E: c_uint = 0x80000000	/* L2 Cache Enable */;
pub const L2CSR0_L2PE: c_uint = 0x40000000	/* L2 Cache Parity/ECC Enable */;
pub const L2CSR0_L2WP: c_uint = 0x1c000000	/* L2 I/D Way Partioning */;
pub const L2CSR0_L2CM: c_uint = 0x03000000	/* L2 Cache Coherency Mode */;
pub const L2CSR0_L2FI: c_uint = 0x00200000	/* L2 Cache Flash Invalidate */;
pub const L2CSR0_L2IO: c_uint = 0x00100000	/* L2 Cache Instruction Only */;
pub const L2CSR0_L2DO: c_uint = 0x00010000	/* L2 Cache Data Only */;
pub const L2CSR0_L2REP: c_uint = 0x00003000	/* L2 Line Replacement Algo */;
pub const L2CSR0_L2FL: c_uint = 0x00000800	/* L2 Cache Flush */;
pub const L2CSR0_L2LFC: c_uint = 0x00000400	/* L2 Cache Lock Flash Clear */;
pub const L2CSR0_L2LOA: c_uint = 0x00000080	/* L2 Cache Lock Overflow Allocate */;
pub const L2CSR0_L2LO: c_uint = 0x00000020	/* L2 Cache Lock Overflow */;
// Bit definitions for SGR.

// Bit definitions for EPCR
pub const SPRN_EPCR_EXTGS: c_uint = 0x80000000	/* External Input interrupt;
// directed to Guest state
pub const SPRN_EPCR_DTLBGS: c_uint = 0x40000000	/* Data TLB Error interrupt;
// directed to guest state
pub const SPRN_EPCR_ITLBGS: c_uint = 0x20000000	/* Instr. TLB error interrupt;
// directed to guest state
pub const SPRN_EPCR_DSIGS: c_uint = 0x10000000	/* Data Storage interrupt;
// directed to guest state
pub const SPRN_EPCR_ISIGS: c_uint = 0x08000000	/* Instr. Storage interrupt;
// directed to guest state
pub const SPRN_EPCR_DUVD: c_uint = 0x04000000	/* Disable Hypervisor Debug */;
pub const SPRN_EPCR_ICM: c_uint = 0x02000000	/* Interrupt computation mode;
// (copied to MSR:CM on intr)
pub const SPRN_EPCR_GICM: c_uint = 0x01000000	/* Guest Interrupt Comp. mode */;
pub const SPRN_EPCR_DGTMI: c_uint = 0x00800000	/* Disable TLB Guest Management;
// instructions
pub const SPRN_EPCR_DMIUH: c_uint = 0x00400000	/* Disable MAS Interrupt updates;
// for hypervisor
// Bit definitions for EPLC/EPSC
pub const EPC_EPR: c_uint = 0x80000000 /* 1 = user, 0 = kernel */;
pub const EPC_EPR_SHIFT: c_int = 31;
pub const EPC_EAS: c_uint = 0x40000000 /* Address Space */;
pub const EPC_EAS_SHIFT: c_int = 30;
pub const EPC_EGS: c_uint = 0x20000000 /* 1 = guest, 0 = hypervisor */;
pub const EPC_EGS_SHIFT: c_int = 29;
pub const EPC_ELPID: c_uint = 0x00ff0000;
pub const EPC_ELPID_SHIFT: c_int = 16;
pub const EPC_EPID: c_uint = 0x00003fff;
pub const EPC_EPID_SHIFT: c_int = 0;
// Some 476 specific registers
pub const SPRN_SSPCR: c_int = 830;
pub const SPRN_USPCR: c_int = 831;
pub const SPRN_ISPCR: c_int = 829;
pub const SPRN_MMUBE0: c_int = 820;
pub const MMUBE0_IBE0_SHIFT: c_int = 24;
pub const MMUBE0_IBE1_SHIFT: c_int = 16;
pub const MMUBE0_IBE2_SHIFT: c_int = 8;
pub const MMUBE0_VBE0: c_uint = 0x00000004;
pub const MMUBE0_VBE1: c_uint = 0x00000002;
pub const MMUBE0_VBE2: c_uint = 0x00000001;
pub const SPRN_MMUBE1: c_int = 821;
pub const MMUBE1_IBE3_SHIFT: c_int = 24;
pub const MMUBE1_IBE4_SHIFT: c_int = 16;
pub const MMUBE1_IBE5_SHIFT: c_int = 8;
pub const MMUBE1_VBE3: c_uint = 0x00000004;
pub const MMUBE1_VBE4: c_uint = 0x00000002;
pub const MMUBE1_VBE5: c_uint = 0x00000001;

pub const TMRN_TMCFG0_NPRIBITS: c_uint = 0x003f0000 /* Bits of thread priority */;
pub const TMRN_TMCFG0_NPRIBITS_SHIFT: c_int = 16;
pub const TMRN_TMCFG0_NATHRD: c_uint = 0x00003f00 /* Number of active threads */;
pub const TMRN_TMCFG0_NATHRD_SHIFT: c_int = 8;
pub const TMRN_TMCFG0_NTHRD: c_uint = 0x0000003f /* Number of threads */;
pub const TMRN_IMSR0: c_uint = 0x120	/* Initial MSR Register 0 (e6500) */;
pub const TMRN_IMSR1: c_uint = 0x121	/* Initial MSR Register 1 (e6500) */;
pub const TMRN_INIA0: c_uint = 0x140	/* Next Instruction Address Register 0 */;
pub const TMRN_INIA1: c_uint = 0x141	/* Next Instruction Address Register 1 */;
pub const SPRN_TENSR: c_uint = 0x1b5	/* Thread Enable Status Register */;
pub const SPRN_TENS: c_uint = 0x1b6	/* Thread Enable Set Register */;
pub const SPRN_TENC: c_uint = 0x1b7	/* Thread Enable Clear Register */;

