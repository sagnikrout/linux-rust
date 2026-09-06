//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/aic94xx/aic94xx_reg_def.h
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
// Aic94xx SAS/SATA driver hardware registers definitions.
//
// Copyright (C) 2004 Adaptec, Inc.  All rights reserved.
// Copyright (C) 2004 David Chaw <david_chaw@adaptec.com>
// Copyright (C) 2005 Luben Tuikov <luben_tuikov@adaptec.com>
//
// Luben Tuikov: Some register value updates to make it work with the window
// agnostic register r/w functions.  Some register corrections, sizes,
// etc.
//
// $Id: //depot/aic94xx/aic94xx_reg_def.h#27 $
//
// Common definitions.
//
pub const CSEQ_MODE_PAGE_SIZE: c_uint = 0x200		/* CSEQ mode page size */;
pub const LmSEQ_MODE_PAGE_SIZE: c_uint = 0x200		/* LmSEQ mode page size */;
pub const LmSEQ_HOST_REG_SIZE: c_uint = 0x4000		/* LmSEQ Host Register size */;
// COM_SAS registers definition
// The base is REG_BASE_ADDR, defined in aic94xx_reg.h.
//
// CHIM Registers, Address Range : (0x00-0xFF)
//

// bits 31:24
pub const L7BLKRST: c_uint = 0x80000000;
pub const L6BLKRST: c_uint = 0x40000000;
pub const L5BLKRST: c_uint = 0x20000000;
pub const L4BLKRST: c_uint = 0x10000000;
pub const L3BLKRST: c_uint = 0x08000000;
pub const L2BLKRST: c_uint = 0x04000000;
pub const L1BLKRST: c_uint = 0x02000000;
pub const L0BLKRST: c_uint = 0x01000000;
pub const LmBLKRST: c_uint = 0xFF000000;

pub const OCMBLKRST: c_uint = 0x00400000;
pub const CTXMEMBLKRST: c_uint = 0x00200000;
pub const CSEQBLKRST: c_uint = 0x00100000;
pub const EXSIBLKRST: c_uint = 0x00040000;
pub const DPIBLKRST: c_uint = 0x00020000;
pub const DFIFBLKRST: c_uint = 0x00010000;
pub const HARDRST: c_uint = 0x00000200;
pub const COMBLKRST: c_uint = 0x00000100;
pub const FRCDFPERR: c_uint = 0x00000080;
pub const FRCCIOPERR: c_uint = 0x00000020;
pub const FRCBISTERR: c_uint = 0x00000010;
pub const COMBISTEN: c_uint = 0x00000004;
pub const COMBISTDONE: c_uint = 0x00000002	/* ro */;
pub const COMBISTFAIL: c_uint = 0x00000001	/* ro */;

pub const REQMBXREAD: c_uint = 0x00000040;
pub const RSPMBXAVAIL: c_uint = 0x00000020;
pub const CSBUFPERR: c_uint = 0x00000008;
pub const OVLYERR: c_uint = 0x00000004;
pub const CSERR: c_uint = 0x00000002;
pub const OVLYDMADONE: c_uint = 0x00000001;

pub const EN_REQMBXREAD: c_uint = 0x00000040;
pub const EN_RSPMBXAVAIL: c_uint = 0x00000020;
pub const EN_CSBUFPERR: c_uint = 0x00000008;
pub const EN_OVLYERR: c_uint = 0x00000004;
pub const EN_CSERR: c_uint = 0x00000002;
pub const EN_OVLYDONE: c_uint = 0x00000001;

pub const SCBCONS_MASK: c_uint = 0xFFFF0000;
pub const SCBPRO_MASK: c_uint = 0x0000FFFF;

pub const EXT_INT0: c_uint = 0x00000800;
pub const EXT_INT1: c_uint = 0x00000400;
pub const PORRSTDET: c_uint = 0x00000200;
pub const HARDRSTDET: c_uint = 0x00000100;
pub const DLAVAILQ: c_uint = 0x00000080	/* ro */;
pub const HOSTERR: c_uint = 0x00000040;
pub const INITERR: c_uint = 0x00000020;
pub const DEVINT: c_uint = 0x00000010;
pub const COMINT: c_uint = 0x00000008;
pub const DEVTIMER2: c_uint = 0x00000004;
pub const DEVTIMER1: c_uint = 0x00000002;
pub const DLAVAIL: c_uint = 0x00000001;

pub const RST_EN_EXT_INT1: c_uint = 0x01000000;
pub const RST_EN_EXT_INT0: c_uint = 0x00800000;
pub const RST_EN_HOSTERR: c_uint = 0x00400000;
pub const RST_EN_INITERR: c_uint = 0x00200000;
pub const RST_EN_DEVINT: c_uint = 0x00100000;
pub const RST_EN_COMINT: c_uint = 0x00080000;
pub const RST_EN_DEVTIMER2: c_uint = 0x00040000;
pub const RST_EN_DEVTIMER1: c_uint = 0x00020000;
pub const RST_EN_DLAVAIL: c_uint = 0x00010000;
pub const SET_EN_EXT_INT1: c_uint = 0x00000100;
pub const SET_EN_EXT_INT0: c_uint = 0x00000080;
pub const SET_EN_HOSTERR: c_uint = 0x00000040;
pub const SET_EN_INITERR: c_uint = 0x00000020;
pub const SET_EN_DEVINT: c_uint = 0x00000010;
pub const SET_EN_COMINT: c_uint = 0x00000008;
pub const SET_EN_DEVTIMER2: c_uint = 0x00000004;
pub const SET_EN_DEVTIMER1: c_uint = 0x00000002;
pub const SET_EN_DLAVAIL: c_uint = 0x00000001;

pub const OVLYADR_MASK: c_uint = 0x07FF0000;
pub const OVLYLSEQ_MASK: c_uint = 0x0000FF00;
pub const OVLYCSEQ: c_uint = 0x00000080;
pub const OVLYHALTERR: c_uint = 0x00000040;
pub const PIOCMODE: c_uint = 0x00000020;
pub const RESETOVLYDMA: c_uint = 0x00000008	/* wo */;
pub const STARTOVLYDMA: c_uint = 0x00000004;
pub const STOPOVLYDMA: c_uint = 0x00000002	/* wo */;
pub const OVLYDMAACT: c_uint = 0x00000001	/* ro */;

pub const OVLYDOMAIN1: c_uint = 0x20000000	/* ro */;
pub const OVLYDOMAIN0: c_uint = 0x10000000;
pub const OVLYBUFADR_MASK: c_uint = 0x007F0000;
pub const OVLYDMACNT_MASK: c_uint = 0x00003FFF;

pub const OVLYERRSTAT_MASK: c_uint = 0x0000FF00	/* ro */;
pub const CSERRSTAT_MASK: c_uint = 0x000000FF	/* ro */;

// 0x38 - 0x3C are reserved

pub const T1DONE: c_uint = 0x00010000	/* ro */;
pub const TIMER64: c_uint = 0x00000400;
pub const T1ENABLE: c_uint = 0x00000200;
pub const T1RELOAD: c_uint = 0x00000100;
pub const T1PRESCALER_MASK: c_uint = 0x00000003;

pub const T2DONE: c_uint = 0x00010000	/* ro */;
pub const T2ENABLE: c_uint = 0x00000200;
pub const T2RELOAD: c_uint = 0x00000100;
pub const T2PRESCALER_MASK: c_uint = 0x00000003;

// 0x58h - 0xFCh are reserved
//
// DCH_SAS Registers, Address Range : (0x800-0xFFF)
//

pub const DEVCTXDOMAIN1: c_uint = 0x00000008	/* ro */;
pub const DEVCTXDOMAIN0: c_uint = 0x00000004;
pub const CMDCTXDOMAIN1: c_uint = 0x00000002	/* ro */;
pub const CMDCTXDOMAIN0: c_uint = 0x00000001;

pub const OCMBISTREPAIR: c_uint = 0x00080000;
pub const OCMBISTEN: c_uint = 0x00040000;
pub const OCMBISTDN: c_uint = 0x00020000	/* ro */;
pub const OCMBISTFAIL: c_uint = 0x00010000	/* ro */;
pub const DDBBISTEN: c_uint = 0x00004000;
pub const DDBBISTDN: c_uint = 0x00002000	/* ro */;
pub const DDBBISTFAIL: c_uint = 0x00001000	/* ro */;
pub const SCBBISTEN: c_uint = 0x00000400;
pub const SCBBISTDN: c_uint = 0x00000200	/* ro */;
pub const SCBBISTFAIL: c_uint = 0x00000100	/* ro */;
pub const MEMSEL_MASK: c_uint = 0x000000E0;
pub const MEMSEL_CCM_LSEQ: c_uint = 0x00000000;
pub const MEMSEL_CCM_IOP: c_uint = 0x00000020;
pub const MEMSEL_CCM_SASCTL: c_uint = 0x00000040;
pub const MEMSEL_DCM_LSEQ: c_uint = 0x00000060;
pub const MEMSEL_DCM_IOP: c_uint = 0x00000080;
pub const MEMSEL_OCM: c_uint = 0x000000A0;
pub const FRCERR: c_uint = 0x00000010;
pub const AUTORLS: c_uint = 0x00000001;

pub const DCHREVISION_MASK: c_uint = 0x000000FF;

pub const EN_CFIFTOERR: c_uint = 0x00020000;
pub const CFIFTOERR: c_uint = 0x00000200;
pub const CSEQINT: c_uint = 0x00000100	/* ro */;
pub const LSEQ7INT: c_uint = 0x00000080	/* ro */;
pub const LSEQ6INT: c_uint = 0x00000040	/* ro */;
pub const LSEQ5INT: c_uint = 0x00000020	/* ro */;
pub const LSEQ4INT: c_uint = 0x00000010	/* ro */;
pub const LSEQ3INT: c_uint = 0x00000008	/* ro */;
pub const LSEQ2INT: c_uint = 0x00000004	/* ro */;
pub const LSEQ1INT: c_uint = 0x00000002	/* ro */;
pub const LSEQ0INT: c_uint = 0x00000001	/* ro */;

pub const ENFAIRMST: c_uint = 0x00FF0000;
pub const DISWRMST9: c_uint = 0x00000200;
pub const DISWRMST8: c_uint = 0x00000100;
pub const DISRDMST: c_uint = 0x000000FF;

// 8 bit wide
pub const AUTOINC: c_uint = 0x80;
pub const ATOMICERR: c_uint = 0x04;
pub const ATOMICWIN: c_uint = 0x02;
pub const ATOMICDONE: c_uint = 0x01;

// 16 bit; bits 8:0 define CIO addr space of CSEQ

// 16 bit wide

// 16 bit wide

// 16 bit

// 16 bit

// 32 bit
// 0x83Ch - 0xFFCh are reserved
//
// ARP2 External Processor Registers, Address Range : (0x00-0x1F)
//
pub const ARP2CTL: c_uint = 0x00;
pub const FRCSCRPERR: c_uint = 0x00040000;
pub const FRCARP2PERR: c_uint = 0x00020000;
pub const FRCARP2ILLOPC: c_uint = 0x00010000;
pub const ENWAITTO: c_uint = 0x00008000;
pub const PERRORDIS: c_uint = 0x00004000;
pub const FAILDIS: c_uint = 0x00002000;
pub const CIOPERRDIS: c_uint = 0x00001000;
pub const BREAKEN3: c_uint = 0x00000800;
pub const BREAKEN2: c_uint = 0x00000400;
pub const BREAKEN1: c_uint = 0x00000200;
pub const BREAKEN0: c_uint = 0x00000100;
pub const EPAUSE: c_uint = 0x00000008;
pub const PAUSED: c_uint = 0x00000004	/* ro */;
pub const STEP: c_uint = 0x00000002;
pub const ARP2RESET: c_uint = 0x00000001	/* wo */;
pub const ARP2INT: c_uint = 0x04;
pub const HALTCODE_MASK: c_uint = 0x00FF0000	/* ro */;
pub const ARP2WAITTO: c_uint = 0x00000100;
pub const ARP2HALTC: c_uint = 0x00000080;
pub const ARP2ILLOPC: c_uint = 0x00000040;
pub const ARP2PERR: c_uint = 0x00000020;
pub const ARP2CIOPERR: c_uint = 0x00000010;
pub const ARP2BREAK3: c_uint = 0x00000008;
pub const ARP2BREAK2: c_uint = 0x00000004;
pub const ARP2BREAK1: c_uint = 0x00000002;
pub const ARP2BREAK0: c_uint = 0x00000001;
pub const ARP2INTEN: c_uint = 0x08;
pub const EN_ARP2WAITTO: c_uint = 0x00000100;
pub const EN_ARP2HALTC: c_uint = 0x00000080;
pub const EN_ARP2ILLOPC: c_uint = 0x00000040;
pub const EN_ARP2PERR: c_uint = 0x00000020;
pub const EN_ARP2CIOPERR: c_uint = 0x00000010;
pub const EN_ARP2BREAK3: c_uint = 0x00000008;
pub const EN_ARP2BREAK2: c_uint = 0x00000004;
pub const EN_ARP2BREAK1: c_uint = 0x00000002;
pub const EN_ARP2BREAK0: c_uint = 0x00000001;
pub const ARP2BREAKADR01: c_uint = 0x0C;
pub const BREAKADR1_MASK: c_uint = 0x0FFF0000;
pub const BREAKADR0_MASK: c_uint = 0x00000FFF;
pub const ARP2BREAKADR23: c_uint = 0x10;
pub const BREAKADR3_MASK: c_uint = 0x0FFF0000;
pub const BREAKADR2_MASK: c_uint = 0x00000FFF;
// 0x14h - 0x1Ch are reserved
//
// ARP2 Registers, Address Range : (0x00-0x1F)
// The definitions have the same address offset for CSEQ and LmSEQ
// CIO Bus Registers.
//
pub const MODEPTR: c_uint = 0x00;
pub const DSTMODE: c_uint = 0xF0;
pub const SRCMODE: c_uint = 0x0F;
pub const ALTMODE: c_uint = 0x01;
pub const ALTDMODE: c_uint = 0xF0;
pub const ALTSMODE: c_uint = 0x0F;
pub const ATOMICXCHG: c_uint = 0x02;
pub const FLAG: c_uint = 0x04;
pub const INTCODE_MASK: c_uint = 0xF0;
pub const ALTMODEV2: c_uint = 0x04;
pub const CARRY_INT: c_uint = 0x02;
pub const CARRY: c_uint = 0x01;
pub const ARP2INTCTL: c_uint = 0x05;
pub const PAUSEDIS: c_uint = 0x80;
pub const RSTINTCTL: c_uint = 0x40;
pub const POPALTMODE: c_uint = 0x08;
pub const ALTMODEV: c_uint = 0x04;
pub const INTMASK: c_uint = 0x02;
pub const IRET: c_uint = 0x01;
pub const STACK: c_uint = 0x06;
pub const FUNCTION1: c_uint = 0x07;
pub const PRGMCNT: c_uint = 0x08;
pub const ACCUM: c_uint = 0x0A;
pub const SINDEX: c_uint = 0x0C;
pub const DINDEX: c_uint = 0x0E;
pub const ALLONES: c_uint = 0x10;
pub const ALLZEROS: c_uint = 0x11;
pub const SINDIR: c_uint = 0x12;
pub const DINDIR: c_uint = 0x13;
pub const JUMLDIR: c_uint = 0x14;
pub const ARP2HALTCODE: c_uint = 0x15;
pub const CURRADDR: c_uint = 0x16;
pub const LASTADDR: c_uint = 0x18;
pub const NXTLADDR: c_uint = 0x1A;
pub const DBGPORTPTR: c_uint = 0x1C;
pub const DBGPORT: c_uint = 0x1D;
//
// CIO Registers.
// The definitions have the same address offset for CSEQ and LmSEQ
// CIO Bus Registers.
//
pub const MnSCBPTR: c_uint = 0x20;
pub const MnDDBPTR: c_uint = 0x22;
pub const SCRATCHPAGE: c_uint = 0x24;
pub const MnSCRATCHPAGE: c_uint = 0x25;
pub const SCRATCHPAGESV: c_uint = 0x26;
pub const MnSCRATCHPAGESV: c_uint = 0x27;
pub const MnDMAERRS: c_uint = 0x46;
pub const MnSGDMAERRS: c_uint = 0x47;
pub const MnSGBUF: c_uint = 0x53;
pub const MnSGDMASTAT: c_uint = 0x5b;
pub const MnDDMACTL: c_uint = 0x5c	/* RAZOR.rspec.fm rev 1.5 is wrong */;
pub const MnDDMASTAT: c_uint = 0x5d	/* RAZOR.rspec.fm rev 1.5 is wrong */;
pub const MnDDMAMODE: c_uint = 0x5e	/* RAZOR.rspec.fm rev 1.5 is wrong */;
pub const MnDMAENG: c_uint = 0x60;
pub const MnPIPECTL: c_uint = 0x61;
pub const MnSGBADR: c_uint = 0x65;
pub const MnSCB_SITE: c_uint = 0x100;
pub const MnDDB_SITE: c_uint = 0x180;
//
// The common definitions below have the same address offset for both
// CSEQ and LmSEQ.
//
pub const BISTCTL0: c_uint = 0x4C;
pub const BISTCTL1: c_uint = 0x50;
pub const MAPPEDSCR: c_uint = 0x800;
//
// CSEQ Host Register, Address Range : (0x000-0xFFC)
//
pub const CSEQ_HOST_REG_BASE_ADR: c_uint = 0xB8001000;

pub const CSEQRAMBISTEN: c_uint = 0x00000040;
pub const CSEQRAMBISTDN: c_uint = 0x00000020	/* ro */;
pub const CSEQRAMBISTFAIL: c_uint = 0x00000010	/* ro */;
pub const CSEQSCRBISTEN: c_uint = 0x00000004;
pub const CSEQSCRBISTDN: c_uint = 0x00000002	/* ro */;
pub const CSEQSCRBISTFAIL: c_uint = 0x00000001	/* ro */;

//
// CSEQ CIO Bus Registers, Address Range : (0x0000-0x1FFC)
// 16 modes, each mode is 512 bytes.
// Unless specified, the register should valid for all modes.
//

// mode 0-7
pub const MnREQMBX: c_uint = 0x30;

// mode 8

// mode 0-7
pub const MnRSPMBX: c_uint = 0x34;

// mode 8

// mode 8

// mode 8

// mode 8

pub const CSHALTERR: c_uint = 0x10;
pub const RESETCSDMA: c_uint = 0x08		/* wo */;
pub const STARTCSDMA: c_uint = 0x04;
pub const STOPCSDMA: c_uint = 0x02		/* wo */;
pub const CSDMAACT: c_uint = 0x01		/* ro */;
// mode 0-7
pub const MnINT: c_uint = 0x38;

pub const CMnREQMBXE: c_uint = 0x02;
pub const CMnRSPMBXF: c_uint = 0x01;
pub const CMnINT_MASK: c_uint = 0x00000003;
// mode 8

// mode 0-7
pub const MnINTEN: c_uint = 0x3C;

pub const EN_CMnRSPMBXF: c_uint = 0x01;
// mode 8

// mode 8

// mode 8

// mode 8

pub const DONELISTEND: c_uint = 0x10;
pub const DONELISTSIZE_MASK: c_uint = 0x0F;
pub const DONELISTSIZE_8ELEM: c_uint = 0x01;
pub const DONELISTSIZE_16ELEM: c_uint = 0x02;
pub const DONELISTSIZE_32ELEM: c_uint = 0x03;
pub const DONELISTSIZE_64ELEM: c_uint = 0x04;
pub const DONELISTSIZE_128ELEM: c_uint = 0x05;
pub const DONELISTSIZE_256ELEM: c_uint = 0x06;
pub const DONELISTSIZE_512ELEM: c_uint = 0x07;
pub const DONELISTSIZE_1024ELEM: c_uint = 0x08;
pub const DONELISTSIZE_2048ELEM: c_uint = 0x09;
pub const DONELISTSIZE_4096ELEM: c_uint = 0x0A;
pub const DONELISTSIZE_8192ELEM: c_uint = 0x0B;
pub const DONELISTSIZE_16384ELEM: c_uint = 0x0C;
// mode 8

// mode 11

// mode 11

// mode 11

// mode 8, 32x32 bits, 128 bytes of mapped buffer

// mode 0-8

//
// CSEQ Mapped Instruction RAM Page, Address Range : (0x0000-0x1FFC)
//
pub const CSEQ_RAM_REG_BASE_ADR: c_uint = 0xB8004000;
//
// The common definitions below have the same address offset for all the Link
// sequencers.
//
pub const MODECTL: c_uint = 0x40;
pub const DBGMODE: c_uint = 0x44;
pub const CONTROL: c_uint = 0x48;
pub const LEDTIMER: c_uint = 0x00010000;
pub const LEDTIMERS_10us: c_uint = 0x00000000;
pub const LEDTIMERS_1ms: c_uint = 0x00000800;
pub const LEDTIMERS_100ms: c_uint = 0x00001000;
pub const LEDMODE_TXRX: c_uint = 0x00000000;
pub const LEDMODE_CONNECTED: c_uint = 0x00000200;
pub const LEDPOL: c_uint = 0x00000100;
pub const LSEQRAM: c_uint = 0x1000;
//
// LmSEQ Host Registers, Address Range : (0x0000-0x3FFC)
//
pub const LSEQ0_HOST_REG_BASE_ADR: c_uint = 0xB8020000;
pub const LSEQ1_HOST_REG_BASE_ADR: c_uint = 0xB8024000;
pub const LSEQ2_HOST_REG_BASE_ADR: c_uint = 0xB8028000;
pub const LSEQ3_HOST_REG_BASE_ADR: c_uint = 0xB802C000;
pub const LSEQ4_HOST_REG_BASE_ADR: c_uint = 0xB8030000;
pub const LSEQ5_HOST_REG_BASE_ADR: c_uint = 0xB8034000;
pub const LSEQ6_HOST_REG_BASE_ADR: c_uint = 0xB8038000;
pub const LSEQ7_HOST_REG_BASE_ADR: c_uint = 0xB803C000;

pub const LmAUTODISCI: c_uint = 0x08000000;
pub const LmDSBLBITLT: c_uint = 0x04000000;
pub const LmDSBLANTT: c_uint = 0x02000000;
pub const LmDSBLCRTT: c_uint = 0x01000000;
pub const LmDSBLCONT: c_uint = 0x00000100;
pub const LmPRIMODE: c_uint = 0x00000080;
pub const LmDSBLHOLD: c_uint = 0x00000040;
pub const LmDISACK: c_uint = 0x00000020;
pub const LmBLIND48: c_uint = 0x00000010;
pub const LmRCVMODE_MASK: c_uint = 0x0000000C;
pub const LmRCVMODE_PLD: c_uint = 0x00000000;
pub const LmRCVMODE_HPC: c_uint = 0x00000004;

pub const LmFRCPERR: c_uint = 0x80000000;
pub const LmMEMSEL_MASK: c_uint = 0x30000000;
pub const LmFRCRBPERR: c_uint = 0x00000000;
pub const LmFRCTBPERR: c_uint = 0x10000000;
pub const LmFRCSGBPERR: c_uint = 0x20000000;
pub const LmFRCARBPERR: c_uint = 0x30000000;
pub const LmRCVIDW: c_uint = 0x00080000;
pub const LmINVDWERR: c_uint = 0x00040000;
pub const LmRCVDISP: c_uint = 0x00004000;
pub const LmDISPERR: c_uint = 0x00002000;
pub const LmDSBLDSCR: c_uint = 0x00000800;
pub const LmDSBLSCR: c_uint = 0x00000400;
pub const LmFRCNAK: c_uint = 0x00000200;
pub const LmFRCROFS: c_uint = 0x00000100;
pub const LmFRCCRC: c_uint = 0x00000080;
pub const LmFRMTYPE_MASK: c_uint = 0x00000070;
pub const LmSG_DATA: c_uint = 0x00000000;
pub const LmSG_COMMAND: c_uint = 0x00000010;
pub const LmSG_TASK: c_uint = 0x00000020;
pub const LmSG_TGTXFER: c_uint = 0x00000030;
pub const LmSG_RESPONSE: c_uint = 0x00000040;
pub const LmSG_IDENADDR: c_uint = 0x00000050;
pub const LmSG_OPENADDR: c_uint = 0x00000060;
pub const LmDISCRCGEN: c_uint = 0x00000008;
pub const LmDISCRCCHK: c_uint = 0x00000004;
pub const LmSSXMTFRM: c_uint = 0x00000002;
pub const LmSSRCVFRM: c_uint = 0x00000001;

pub const LmSTEPXMTFRM: c_uint = 0x00000002;
pub const LmSTEPRCVFRM: c_uint = 0x00000001;

pub const ARBBISTEN: c_uint = 0x40000000;
pub const ARBBISTDN: c_uint = 0x20000000	/* ro */;
pub const ARBBISTFAIL: c_uint = 0x10000000	/* ro */;
pub const TBBISTEN: c_uint = 0x00000400;
pub const TBBISTDN: c_uint = 0x00000200	/* ro */;
pub const TBBISTFAIL: c_uint = 0x00000100	/* ro */;
pub const RBBISTEN: c_uint = 0x00000040;
pub const RBBISTDN: c_uint = 0x00000020	/* ro */;
pub const RBBISTFAIL: c_uint = 0x00000010	/* ro */;
pub const SGBISTEN: c_uint = 0x00000004;
pub const SGBISTDN: c_uint = 0x00000002	/* ro */;
pub const SGBISTFAIL: c_uint = 0x00000001	/* ro */;

pub const LmRAMPAGE1: c_uint = 0x00000200;
pub const LmRAMPAGE0: c_uint = 0x00000100;
pub const LmIMEMBISTEN: c_uint = 0x00000040;
pub const LmIMEMBISTDN: c_uint = 0x00000020	/* ro */;
pub const LmIMEMBISTFAIL: c_uint = 0x00000010	/* ro */;
pub const LmSCRBISTEN: c_uint = 0x00000004;
pub const LmSCRBISTDN: c_uint = 0x00000002	/* ro */;
pub const LmSCRBISTFAIL: c_uint = 0x00000001	/* ro */;

pub const LmRAMPAGE_LSHIFT: c_uint = 0x8;

//
// LmSEQ CIO Bus Register, Address Range : (0x0000-0xFFC)
// 8 modes, each mode is 512 bytes.
// Unless specified, the register should valid for all modes.
//
pub const LmSEQ_CIOBUS_REG_BASE: c_uint = 0x2000;

pub const CTXMEMSIZE: c_uint = 0x80000000	/* ro */;
pub const LmACKREQ: c_uint = 0x08000000;
pub const LmNAKREQ: c_uint = 0x04000000;
pub const LmMnXMTERR: c_uint = 0x02000000;
pub const LmM5OOBSVC: c_uint = 0x01000000;
pub const LmHWTINT: c_uint = 0x00800000;
pub const LmMnCTXDONE: c_uint = 0x00100000;
pub const LmM2REQMBXF: c_uint = 0x00080000;
pub const LmM2RSPMBXE: c_uint = 0x00040000;
pub const LmMnDMAERR: c_uint = 0x00020000;
pub const LmRCVPRIM: c_uint = 0x00010000;
pub const LmRCVERR: c_uint = 0x00008000;
pub const LmADDRRCV: c_uint = 0x00004000;
pub const LmMnHDRMISS: c_uint = 0x00002000;
pub const LmMnWAITSCB: c_uint = 0x00001000;
pub const LmMnRLSSCB: c_uint = 0x00000800;
pub const LmMnSAVECTX: c_uint = 0x00000400;
pub const LmMnFETCHSG: c_uint = 0x00000200;
pub const LmMnLOADCTX: c_uint = 0x00000100;
pub const LmMnCFGICL: c_uint = 0x00000080;
pub const LmMnCFGSATA: c_uint = 0x00000040;
pub const LmMnCFGEXPSATA: c_uint = 0x00000020;
pub const LmMnCFGCMPLT: c_uint = 0x00000010;
pub const LmMnCFGRBUF: c_uint = 0x00000008;
pub const LmMnSAVETTR: c_uint = 0x00000004;
pub const LmMnCFGRDAT: c_uint = 0x00000002;
pub const LmMnCFGHDR: c_uint = 0x00000001;

pub const EN_LmACKREQ: c_uint = 0x08000000;
pub const EN_LmNAKREQ: c_uint = 0x04000000;
pub const EN_LmMnXMTERR: c_uint = 0x02000000;
pub const EN_LmM5OOBSVC: c_uint = 0x01000000;
pub const EN_LmHWTINT: c_uint = 0x00800000;
pub const EN_LmMnCTXDONE: c_uint = 0x00100000;
pub const EN_LmM2REQMBXF: c_uint = 0x00080000;
pub const EN_LmM2RSPMBXE: c_uint = 0x00040000;
pub const EN_LmMnDMAERR: c_uint = 0x00020000;
pub const EN_LmRCVPRIM: c_uint = 0x00010000;
pub const EN_LmRCVERR: c_uint = 0x00008000;
pub const EN_LmADDRRCV: c_uint = 0x00004000;
pub const EN_LmMnHDRMISS: c_uint = 0x00002000;
pub const EN_LmMnWAITSCB: c_uint = 0x00001000;
pub const EN_LmMnRLSSCB: c_uint = 0x00000800;
pub const EN_LmMnSAVECTX: c_uint = 0x00000400;
pub const EN_LmMnFETCHSG: c_uint = 0x00000200;
pub const EN_LmMnLOADCTX: c_uint = 0x00000100;
pub const EN_LmMnCFGICL: c_uint = 0x00000080;
pub const EN_LmMnCFGSATA: c_uint = 0x00000040;
pub const EN_LmMnCFGEXPSATA: c_uint = 0x00000020;
pub const EN_LmMnCFGCMPLT: c_uint = 0x00000010;
pub const EN_LmMnCFGRBUF: c_uint = 0x00000008;
pub const EN_LmMnSAVETTR: c_uint = 0x00000004;
pub const EN_LmMnCFGRDAT: c_uint = 0x00000002;
pub const EN_LmMnCFGHDR: c_uint = 0x00000001;

pub const SAS_ALIGN_DEFAULT: c_uint = 0xFF;

pub const STP_ALIGN_DEFAULT: c_uint = 0x1F;

pub const LmDISALIGN: c_uint = 0x20;
pub const LmROTSTPALIGN: c_uint = 0x10;
pub const LmSTPALIGN: c_uint = 0x08;
pub const LmROTNOTIFY: c_uint = 0x04;
pub const LmDUALALIGN: c_uint = 0x02;
pub const LmROTALIGN: c_uint = 0x01;

pub const LmMnBUFPERR: c_uint = 0x01;
// mode 0-1

pub const LmMnXFRLVL_128: c_uint = 0x05;
pub const LmMnXFRLVL_256: c_uint = 0x04;
pub const LmMnXFRLVL_512: c_uint = 0x03;
pub const LmMnXFRLVL_1024: c_uint = 0x02;
pub const LmMnXFRLVL_1536: c_uint = 0x01;
pub const LmMnXFRLVL_2048: c_uint = 0x00;
// mode 0-1

pub const LmMnRESETSG: c_uint = 0x04;
pub const LmMnSTOPSG: c_uint = 0x02;
pub const LmMnSTARTSG: c_uint = 0x01;
// mode 0-1

// mode 0-1

pub const LmMnFLUSH: c_uint = 0x40		/* wo */;
pub const LmMnRLSRTRY: c_uint = 0x20		/* wo */;
pub const LmMnDISCARD: c_uint = 0x10		/* wo */;
pub const LmMnRESETDAT: c_uint = 0x08		/* wo */;
pub const LmMnSUSDAT: c_uint = 0x04		/* wo */;
pub const LmMnSTOPDAT: c_uint = 0x02		/* wo */;
pub const LmMnSTARTDAT: c_uint = 0x01		/* wo */;
// mode 0-1

pub const LmMnDPEMPTY: c_uint = 0x80;
pub const LmMnFLUSHING: c_uint = 0x40;
pub const LmMnDDMAREQ: c_uint = 0x20;
pub const LmMnHDMAREQ: c_uint = 0x10;
pub const LmMnDATFREE: c_uint = 0x08;
pub const LmMnDATSUS: c_uint = 0x04;
pub const LmMnDATACT: c_uint = 0x02;
pub const LmMnDATEN: c_uint = 0x01;
// mode 0-1

pub const LmMnDMATYPE_NORMAL: c_uint = 0x0000;
pub const LmMnDMATYPE_HOST_ONLY_TX: c_uint = 0x0001;
pub const LmMnDMATYPE_DEVICE_ONLY_TX: c_uint = 0x0002;
pub const LmMnDMATYPE_INVALID: c_uint = 0x0003;
pub const LmMnDMATYPE_MASK: c_uint = 0x0003;
pub const LmMnDMAWRAP: c_uint = 0x0004;
pub const LmMnBITBUCKET: c_uint = 0x0008;
pub const LmMnDISHDR: c_uint = 0x0010;
pub const LmMnSTPCRC: c_uint = 0x0020;
pub const LmXTEST: c_uint = 0x0040;
pub const LmMnDISCRC: c_uint = 0x0080;
pub const LmMnENINTLK: c_uint = 0x0100;
pub const LmMnADDRFRM: c_uint = 0x0400;
pub const LmMnENXMTCRC: c_uint = 0x0800;
// mode 0-1

// mode 0-1

pub const LmMnDPSEL_MASK: c_uint = 0x07;
pub const LmMnEOLPRE: c_uint = 0x40;
pub const LmMnEOSPRE: c_uint = 0x80;
// Registers used in conjunction with LmMnDPSEL and LmMnDPACC registers
// Receive Mode n = 0
pub const LmMnHRADDR: c_uint = 0x00;
pub const LmMnHBYTECNT: c_uint = 0x01;
pub const LmMnHREWIND: c_uint = 0x02;
pub const LmMnDWADDR: c_uint = 0x03;
pub const LmMnDSPACECNT: c_uint = 0x04;
pub const LmMnDFRMSIZE: c_uint = 0x05;
// Registers used in conjunction with LmMnDPSEL and LmMnDPACC registers
// Transmit Mode n = 1
pub const LmMnHWADDR: c_uint = 0x00;
pub const LmMnHSPACECNT: c_uint = 0x01;
// #define LmMnHREWIND			0x02
pub const LmMnDRADDR: c_uint = 0x03;
pub const LmMnDBYTECNT: c_uint = 0x04;
// #define LmMnDFRMSIZE			0x05
// mode 0-1

pub const LmMnDPACC_MASK: c_uint = 0x00FFFFFF;
// mode 0-1

pub const LmPRMSTAT0BYTE0: c_uint = 0x80;
pub const LmPRMSTAT0BYTE1: c_uint = 0x81;
pub const LmPRMSTAT0BYTE2: c_uint = 0x82;
pub const LmPRMSTAT0BYTE3: c_uint = 0x83;
pub const LmFRAMERCVD: c_uint = 0x80000000;
pub const LmXFRRDYRCVD: c_uint = 0x40000000;
pub const LmUNKNOWNP: c_uint = 0x20000000;
pub const LmBREAK: c_uint = 0x10000000;
pub const LmDONE: c_uint = 0x08000000;
pub const LmOPENACPT: c_uint = 0x04000000;
pub const LmOPENRJCT: c_uint = 0x02000000;
pub const LmOPENRTRY: c_uint = 0x01000000;
pub const LmCLOSERV1: c_uint = 0x00800000;
pub const LmCLOSERV0: c_uint = 0x00400000;
pub const LmCLOSENORM: c_uint = 0x00200000;
pub const LmCLOSECLAF: c_uint = 0x00100000;
pub const LmNOTIFYRV2: c_uint = 0x00080000;
pub const LmNOTIFYRV1: c_uint = 0x00040000;
pub const LmNOTIFYRV0: c_uint = 0x00020000;
pub const LmNOTIFYSPIN: c_uint = 0x00010000;
pub const LmBROADRV4: c_uint = 0x00008000;
pub const LmBROADRV3: c_uint = 0x00004000;
pub const LmBROADRV2: c_uint = 0x00002000;
pub const LmBROADRV1: c_uint = 0x00001000;
pub const LmBROADSES: c_uint = 0x00000800;
pub const LmBROADRVCH1: c_uint = 0x00000400;
pub const LmBROADRVCH0: c_uint = 0x00000200;
pub const LmBROADCH: c_uint = 0x00000100;
pub const LmAIPRVWP: c_uint = 0x00000080;
pub const LmAIPWP: c_uint = 0x00000040;
pub const LmAIPWD: c_uint = 0x00000020;
pub const LmAIPWC: c_uint = 0x00000010;
pub const LmAIPRV2: c_uint = 0x00000008;
pub const LmAIPRV1: c_uint = 0x00000004;
pub const LmAIPRV0: c_uint = 0x00000002;
pub const LmAIPNRML: c_uint = 0x00000001;

pub const LmPRMSTAT1BYTE0: c_uint = 0x84;
pub const LmPRMSTAT1BYTE1: c_uint = 0x85;
pub const LmPRMSTAT1BYTE2: c_uint = 0x86;
pub const LmPRMSTAT1BYTE3: c_uint = 0x87;
pub const LmFRMRCVDSTAT: c_uint = 0x80000000;
pub const LmBREAK_DET: c_uint = 0x04000000;
pub const LmCLOSE_DET: c_uint = 0x02000000;
pub const LmDONE_DET: c_uint = 0x01000000;
pub const LmXRDY: c_uint = 0x00040000;
pub const LmSYNCSRST: c_uint = 0x00020000;
pub const LmSYNC: c_uint = 0x00010000;
pub const LmXHOLD: c_uint = 0x00008000;
pub const LmRRDY: c_uint = 0x00004000;
pub const LmHOLD: c_uint = 0x00002000;
pub const LmROK: c_uint = 0x00001000;
pub const LmRIP: c_uint = 0x00000800;
pub const LmCRBLK: c_uint = 0x00000400;
pub const LmACK: c_uint = 0x00000200;
pub const LmNAK: c_uint = 0x00000100;
pub const LmHARDRST: c_uint = 0x00000080;
pub const LmERROR: c_uint = 0x00000040;
pub const LmRERR: c_uint = 0x00000020;
pub const LmPMREQP: c_uint = 0x00000010;
pub const LmPMREQS: c_uint = 0x00000008;
pub const LmPMACK: c_uint = 0x00000004;
pub const LmPMNAK: c_uint = 0x00000002;
pub const LmDMAT: c_uint = 0x00000001;
// mode 1

// mode 0

pub const LmACRCERR: c_uint = 0x00000800;
pub const LmPHYOVRN: c_uint = 0x00000400;
pub const LmOBOVRN: c_uint = 0x00000200;
pub const LmMnZERODATA: c_uint = 0x00000100;
pub const LmSATAINTLK: c_uint = 0x00000080;
pub const LmMnCRCERR: c_uint = 0x00000020;
pub const LmRRDYOVRN: c_uint = 0x00000010;
pub const LmMISSSOAF: c_uint = 0x00000008;
pub const LmMISSSOF: c_uint = 0x00000004;
pub const LmMISSEOAF: c_uint = 0x00000002;
pub const LmMISSEOF: c_uint = 0x00000001;

pub const EN_LmACRCERR: c_uint = 0x00000800;
pub const EN_LmPHYOVRN: c_uint = 0x00000400;
pub const EN_LmOBOVRN: c_uint = 0x00000200;
pub const EN_LmMnZERODATA: c_uint = 0x00000100;
pub const EN_LmSATAINTLK: c_uint = 0x00000080;
pub const EN_LmFRMBAD: c_uint = 0x00000040;
pub const EN_LmMnCRCERR: c_uint = 0x00000020;
pub const EN_LmRRDYOVRN: c_uint = 0x00000010;
pub const EN_LmMISSSOAF: c_uint = 0x00000008;
pub const EN_LmMISSSOF: c_uint = 0x00000004;
pub const EN_LmMISSEOAF: c_uint = 0x00000002;
pub const EN_LmMISSEOF: c_uint = 0x00000001;

pub const EN_LmDONETO: c_uint = 0x80;
pub const EN_LmINVDISP: c_uint = 0x40;
pub const EN_LmINVDW: c_uint = 0x20;
pub const EN_LmDWSEVENT: c_uint = 0x08;
pub const EN_LmCRTTTO: c_uint = 0x04;
pub const EN_LmANTTTO: c_uint = 0x02;
pub const EN_LmBITLTTO: c_uint = 0x01;

pub const LmDONETO: c_uint = 0x80;
pub const LmINVDISP: c_uint = 0x40;
pub const LmINVDW: c_uint = 0x20;
pub const LmDWSEVENT: c_uint = 0x08;
pub const LmCRTTTO: c_uint = 0x04;
pub const LmANTTTO: c_uint = 0x02;
pub const LmBITLTTO: c_uint = 0x01;

pub const LmDATABUFADR_MASK: c_uint = 0x0FFF;

pub const EN_LmUNKNOWNP: c_uint = 0x20000000;
pub const EN_LmBREAK: c_uint = 0x10000000;
pub const EN_LmDONE: c_uint = 0x08000000;
pub const EN_LmOPENACPT: c_uint = 0x04000000;
pub const EN_LmOPENRJCT: c_uint = 0x02000000;
pub const EN_LmOPENRTRY: c_uint = 0x01000000;
pub const EN_LmCLOSERV1: c_uint = 0x00800000;
pub const EN_LmCLOSERV0: c_uint = 0x00400000;
pub const EN_LmCLOSENORM: c_uint = 0x00200000;
pub const EN_LmCLOSECLAF: c_uint = 0x00100000;
pub const EN_LmNOTIFYRV2: c_uint = 0x00080000;
pub const EN_LmNOTIFYRV1: c_uint = 0x00040000;
pub const EN_LmNOTIFYRV0: c_uint = 0x00020000;
pub const EN_LmNOTIFYSPIN: c_uint = 0x00010000;
pub const EN_LmBROADRV4: c_uint = 0x00008000;
pub const EN_LmBROADRV3: c_uint = 0x00004000;
pub const EN_LmBROADRV2: c_uint = 0x00002000;
pub const EN_LmBROADRV1: c_uint = 0x00001000;
pub const EN_LmBROADRV0: c_uint = 0x00000800;
pub const EN_LmBROADRVCH1: c_uint = 0x00000400;
pub const EN_LmBROADRVCH0: c_uint = 0x00000200;
pub const EN_LmBROADCH: c_uint = 0x00000100;
pub const EN_LmAIPRVWP: c_uint = 0x00000080;
pub const EN_LmAIPWP: c_uint = 0x00000040;
pub const EN_LmAIPWD: c_uint = 0x00000020;
pub const EN_LmAIPWC: c_uint = 0x00000010;
pub const EN_LmAIPRV2: c_uint = 0x00000008;
pub const EN_LmAIPRV1: c_uint = 0x00000004;
pub const EN_LmAIPRV0: c_uint = 0x00000002;
pub const EN_LmAIPNRML: c_uint = 0x00000001;

pub const EN_LmXRDY: c_uint = 0x00040000;
pub const EN_LmSYNCSRST: c_uint = 0x00020000;
pub const EN_LmSYNC: c_uint = 0x00010000;
pub const EN_LmXHOLD: c_uint = 0x00008000;
pub const EN_LmRRDY: c_uint = 0x00004000;
pub const EN_LmHOLD: c_uint = 0x00002000;
pub const EN_LmROK: c_uint = 0x00001000;
pub const EN_LmRIP: c_uint = 0x00000800;
pub const EN_LmCRBLK: c_uint = 0x00000400;
pub const EN_LmACK: c_uint = 0x00000200;
pub const EN_LmNAK: c_uint = 0x00000100;
pub const EN_LmHARDRST: c_uint = 0x00000080;
pub const EN_LmERROR: c_uint = 0x00000040;
pub const EN_LmRERR: c_uint = 0x00000020;
pub const EN_LmPMREQP: c_uint = 0x00000010;
pub const EN_LmPMREQS: c_uint = 0x00000008;
pub const EN_LmPMACK: c_uint = 0x00000004;
pub const EN_LmPMNAK: c_uint = 0x00000002;
pub const EN_LmDMAT: c_uint = 0x00000001;

//
// LmSEQ CIO Bus Mode 3 Register.
// Mode 3: Configuration and Setup, IOP Context SCB.
//

//
// LmSEQ CIO Bus Mode 5 Registers.
// Mode 5: Phy/OOB Control and Status.
//

pub const OOB_BFLTR: c_uint = 0x100;
pub const BFLTR_THR_MASK: c_uint = 0xF0;
pub const BFLTR_TC_MASK: c_uint = 0x0F;
pub const OOB_INIT_MIN: c_uint = 0x102;
pub const OOB_INIT_MAX: c_uint = 0x104;
pub const OOB_INIT_NEG: c_uint = 0x106;
pub const OOB_SAS_MIN: c_uint = 0x108;
pub const OOB_SAS_MAX: c_uint = 0x10A;
pub const OOB_SAS_NEG: c_uint = 0x10C;
pub const OOB_WAKE_MIN: c_uint = 0x10E;
pub const OOB_WAKE_MAX: c_uint = 0x110;
pub const OOB_WAKE_NEG: c_uint = 0x112;
pub const OOB_IDLE_MAX: c_uint = 0x114;
pub const OOB_BURST_MAX: c_uint = 0x116;
pub const OOB_DATA_KBITS: c_uint = 0x126;
pub const OOB_ALIGN_0_DATA: c_uint = 0x12C;
pub const OOB_ALIGN_1_DATA: c_uint = 0x130;
pub const D10_2_DATA_k: c_uint = 0x00;
pub const SYNC_DATA_k: c_uint = 0x02;
pub const ALIGN_1_DATA_k: c_uint = 0x04;
pub const ALIGN_0_DATA_k: c_uint = 0x08;
pub const BURST_DATA_k: c_uint = 0x10;
pub const OOB_PHY_RESET_COUNT: c_uint = 0x13C;
pub const OOB_SIG_GEN: c_uint = 0x140;
pub const START_OOB: c_uint = 0x80;
pub const START_DWS: c_uint = 0x40;
pub const ALIGN_CNT3: c_uint = 0x30;
pub const ALIGN_CNT2: c_uint = 0x20;
pub const ALIGN_CNT1: c_uint = 0x10;
pub const ALIGN_CNT4: c_uint = 0x00;
pub const STOP_DWS: c_uint = 0x08;
pub const SEND_COMSAS: c_uint = 0x04;
pub const SEND_COMINIT: c_uint = 0x02;
pub const SEND_COMWAKE: c_uint = 0x01;
pub const OOB_XMIT: c_uint = 0x141;
pub const TX_ENABLE: c_uint = 0x80;
pub const XMIT_OOB_BURST: c_uint = 0x10;
pub const XMIT_D10_2: c_uint = 0x08;
pub const XMIT_SYNC: c_uint = 0x04;
pub const XMIT_ALIGN_1: c_uint = 0x02;
pub const XMIT_ALIGN_0: c_uint = 0x01;
pub const FUNCTION_MASK: c_uint = 0x142;
pub const SAS_MODE_DIS: c_uint = 0x80;
pub const SATA_MODE_DIS: c_uint = 0x40;
pub const SPINUP_HOLD_DIS: c_uint = 0x20;
pub const HOT_PLUG_DIS: c_uint = 0x10;
pub const SATA_PS_DIS: c_uint = 0x08;

pub const OOB_MODE: c_uint = 0x143;
pub const SAS_MODE: c_uint = 0x80;
pub const SATA_MODE: c_uint = 0x40;
pub const SLOW_CLK: c_uint = 0x20;
pub const FORCE_XMIT_15: c_uint = 0x08;
pub const PHY_SPEED_60: c_uint = 0x04;
pub const PHY_SPEED_30: c_uint = 0x02;
pub const PHY_SPEED_15: c_uint = 0x01;
pub const CURRENT_STATUS: c_uint = 0x144;
pub const CURRENT_OOB_DONE: c_uint = 0x80;
pub const CURRENT_LOSS_OF_SIGNAL: c_uint = 0x40;
pub const CURRENT_SPINUP_HOLD: c_uint = 0x20;
pub const CURRENT_HOT_PLUG_CNCT: c_uint = 0x10;
pub const CURRENT_GTO_TIMEOUT: c_uint = 0x08;
pub const CURRENT_OOB_TIMEOUT: c_uint = 0x04;
pub const CURRENT_DEVICE_PRESENT: c_uint = 0x02;
pub const CURRENT_OOB_ERROR: c_uint = 0x01;

pub const SPEED_MASK: c_uint = 0x145;
pub const SATA_SPEED_30_DIS: c_uint = 0x10;
pub const SATA_SPEED_15_DIS: c_uint = 0x08;
pub const SAS_SPEED_60_DIS: c_uint = 0x04;
pub const SAS_SPEED_30_DIS: c_uint = 0x02;
pub const SAS_SPEED_15_DIS: c_uint = 0x01;
pub const SAS_SPEED_MASK_DEFAULT: c_uint = 0x00;
pub const OOB_TIMER_ENABLE: c_uint = 0x14D;
pub const HOT_PLUG_EN: c_uint = 0x80;
pub const RCD_EN: c_uint = 0x40;
pub const COMTIMER_EN: c_uint = 0x20;
pub const SNTT_EN: c_uint = 0x10;
pub const SNLT_EN: c_uint = 0x04;
pub const SNWT_EN: c_uint = 0x02;
pub const ALIGN_EN: c_uint = 0x01;
pub const OOB_STATUS: c_uint = 0x14E;
pub const OOB_DONE: c_uint = 0x80;
pub const LOSS_OF_SIGNAL: c_uint = 0x40		/* ro */;
pub const SPINUP_HOLD: c_uint = 0x20;
pub const HOT_PLUG_CNCT: c_uint = 0x10		/* ro */;
pub const GTO_TIMEOUT: c_uint = 0x08		/* ro */;
pub const OOB_TIMEOUT: c_uint = 0x04		/* ro */;
pub const DEVICE_PRESENT: c_uint = 0x02		/* ro */;
pub const OOB_ERROR: c_uint = 0x01		/* ro */;

pub const OOB_STATUS_CLEAR: c_uint = 0x14F;
pub const OOB_DONE_CLR: c_uint = 0x80;
pub const LOSS_OF_SIGNAL_CLR: c_uint = 0x40;
pub const SPINUP_HOLD_CLR: c_uint = 0x20;
pub const HOT_PLUG_CNCT_CLR: c_uint = 0x10;
pub const GTO_TIMEOUT_CLR: c_uint = 0x08;
pub const OOB_TIMEOUT_CLR: c_uint = 0x04;
pub const OOB_ERROR_CLR: c_uint = 0x01;
pub const HOT_PLUG_DELAY: c_uint = 0x150;
// In 5 ms units. 20 = 100 ms.
pub const HOTPLUG_DELAY_TIMEOUT: c_int = 20;
pub const INT_ENABLE_2: c_uint = 0x15A;
pub const OOB_DONE_EN: c_uint = 0x80;
pub const LOSS_OF_SIGNAL_EN: c_uint = 0x40;
pub const SPINUP_HOLD_EN: c_uint = 0x20;
pub const HOT_PLUG_CNCT_EN: c_uint = 0x10;
pub const GTO_TIMEOUT_EN: c_uint = 0x08;
pub const OOB_TIMEOUT_EN: c_uint = 0x04;
pub const DEVICE_PRESENT_EN: c_uint = 0x02;
pub const OOB_ERROR_EN: c_uint = 0x01;
pub const PHY_CONTROL_0: c_uint = 0x160;
pub const PHY_LOWPWREN_TX: c_uint = 0x80;
pub const PHY_LOWPWREN_RX: c_uint = 0x40;
pub const SPARE_REG_160_B5: c_uint = 0x20;
pub const OFFSET_CANCEL_RX: c_uint = 0x10;
// bits 3:2
pub const PHY_RXCOMCENTER_60V: c_uint = 0x00;
pub const PHY_RXCOMCENTER_70V: c_uint = 0x04;
pub const PHY_RXCOMCENTER_80V: c_uint = 0x08;
pub const PHY_RXCOMCENTER_90V: c_uint = 0x0C;
pub const PHY_RXCOMCENTER_MASK: c_uint = 0x0C;
pub const PHY_RESET: c_uint = 0x02;
pub const SAS_DEFAULT_SEL: c_uint = 0x01;
pub const PHY_CONTROL_1: c_uint = 0x161;
// bits 2:0
pub const SATA_PHY_DETLEVEL_50mv: c_uint = 0x00;
pub const SATA_PHY_DETLEVEL_75mv: c_uint = 0x01;
pub const SATA_PHY_DETLEVEL_100mv: c_uint = 0x02;
pub const SATA_PHY_DETLEVEL_125mv: c_uint = 0x03;
pub const SATA_PHY_DETLEVEL_150mv: c_uint = 0x04;
pub const SATA_PHY_DETLEVEL_175mv: c_uint = 0x05;
pub const SATA_PHY_DETLEVEL_200mv: c_uint = 0x06;
pub const SATA_PHY_DETLEVEL_225mv: c_uint = 0x07;
pub const SATA_PHY_DETLEVEL_MASK: c_uint = 0x07;
// bits 5:3
pub const SAS_PHY_DETLEVEL_50mv: c_uint = 0x00;
pub const SAS_PHY_DETLEVEL_75mv: c_uint = 0x08;
pub const SAS_PHY_DETLEVEL_100mv: c_uint = 0x10;
pub const SAS_PHY_DETLEVEL_125mv: c_uint = 0x11;
pub const SAS_PHY_DETLEVEL_150mv: c_uint = 0x20;
pub const SAS_PHY_DETLEVEL_175mv: c_uint = 0x21;
pub const SAS_PHY_DETLEVEL_200mv: c_uint = 0x30;
pub const SAS_PHY_DETLEVEL_225mv: c_uint = 0x31;
pub const SAS_PHY_DETLEVEL_MASK: c_uint = 0x38;
pub const PHY_CONTROL_2: c_uint = 0x162;
// bits 7:5
pub const SATA_PHY_DRV_400mv: c_uint = 0x00;
pub const SATA_PHY_DRV_450mv: c_uint = 0x20;
pub const SATA_PHY_DRV_500mv: c_uint = 0x40;
pub const SATA_PHY_DRV_550mv: c_uint = 0x60;
pub const SATA_PHY_DRV_600mv: c_uint = 0x80;
pub const SATA_PHY_DRV_650mv: c_uint = 0xA0;
pub const SATA_PHY_DRV_725mv: c_uint = 0xC0;
pub const SATA_PHY_DRV_800mv: c_uint = 0xE0;
pub const SATA_PHY_DRV_MASK: c_uint = 0xE0;
// bits 4:3
pub const SATA_PREEMP_0: c_uint = 0x00;
pub const SATA_PREEMP_1: c_uint = 0x08;
pub const SATA_PREEMP_2: c_uint = 0x10;
pub const SATA_PREEMP_3: c_uint = 0x18;
pub const SATA_PREEMP_MASK: c_uint = 0x18;
pub const SATA_CMSH1P5: c_uint = 0x04;
// bits 1:0
pub const SATA_SLEW_0: c_uint = 0x00;
pub const SATA_SLEW_1: c_uint = 0x01;
pub const SATA_SLEW_2: c_uint = 0x02;
pub const SATA_SLEW_3: c_uint = 0x03;
pub const SATA_SLEW_MASK: c_uint = 0x03;
pub const PHY_CONTROL_3: c_uint = 0x163;
// bits 7:5
pub const SAS_PHY_DRV_400mv: c_uint = 0x00;
pub const SAS_PHY_DRV_450mv: c_uint = 0x20;
pub const SAS_PHY_DRV_500mv: c_uint = 0x40;
pub const SAS_PHY_DRV_550mv: c_uint = 0x60;
pub const SAS_PHY_DRV_600mv: c_uint = 0x80;
pub const SAS_PHY_DRV_650mv: c_uint = 0xA0;
pub const SAS_PHY_DRV_725mv: c_uint = 0xC0;
pub const SAS_PHY_DRV_800mv: c_uint = 0xE0;
pub const SAS_PHY_DRV_MASK: c_uint = 0xE0;
// bits 4:3
pub const SAS_PREEMP_0: c_uint = 0x00;
pub const SAS_PREEMP_1: c_uint = 0x08;
pub const SAS_PREEMP_2: c_uint = 0x10;
pub const SAS_PREEMP_3: c_uint = 0x18;
pub const SAS_PREEMP_MASK: c_uint = 0x18;
pub const SAS_CMSH1P5: c_uint = 0x04;
// bits 1:0
pub const SAS_SLEW_0: c_uint = 0x00;
pub const SAS_SLEW_1: c_uint = 0x01;
pub const SAS_SLEW_2: c_uint = 0x02;
pub const SAS_SLEW_3: c_uint = 0x03;
pub const SAS_SLEW_MASK: c_uint = 0x03;
pub const PHY_CONTROL_4: c_uint = 0x168;
pub const PHY_DONE_CAL_TX: c_uint = 0x80;
pub const PHY_DONE_CAL_RX: c_uint = 0x40;
pub const RX_TERM_LOAD_DIS: c_uint = 0x20;
pub const TX_TERM_LOAD_DIS: c_uint = 0x10;
pub const AUTO_TERM_CAL_DIS: c_uint = 0x08;
pub const PHY_SIGDET_FLTR_EN: c_uint = 0x04;
pub const OSC_FREQ: c_uint = 0x02;
pub const PHY_START_CAL: c_uint = 0x01;
//
// HST_PCIX2 Registers, Address Range: (0x00-0xFC)
//
pub const PCIX_REG_BASE_ADR: c_uint = 0xB8040000;
pub const PCIC_VENDOR_ID: c_uint = 0x00;
pub const PCIC_DEVICE_ID: c_uint = 0x02;
pub const PCIC_COMMAND: c_uint = 0x04;
pub const INT_DIS: c_uint = 0x0400;
pub const FBB_EN: c_uint = 0x0200		/* ro */;
pub const SERR_EN: c_uint = 0x0100;
pub const STEP_EN: c_uint = 0x0080		/* ro */;
pub const PERR_EN: c_uint = 0x0040;
pub const VGA_EN: c_uint = 0x0020		/* ro */;
pub const MWI_EN: c_uint = 0x0010;
pub const SPC_EN: c_uint = 0x0008;
pub const MST_EN: c_uint = 0x0004;
pub const MEM_EN: c_uint = 0x0002;
pub const IO_EN: c_uint = 0x0001;
pub const PCIC_STATUS: c_uint = 0x06;
pub const PERR_DET: c_uint = 0x8000;
pub const SERR_GEN: c_uint = 0x4000;
pub const MABT_DET: c_uint = 0x2000;
pub const TABT_DET: c_uint = 0x1000;
pub const TABT_GEN: c_uint = 0x0800;
pub const DPERR_DET: c_uint = 0x0100;
pub const CAP_LIST: c_uint = 0x0010;
pub const INT_STAT: c_uint = 0x0008;
pub const PCIC_DEVREV_ID: c_uint = 0x08;
pub const PCIC_CLASS_CODE: c_uint = 0x09;
pub const PCIC_CACHELINE_SIZE: c_uint = 0x0C;
pub const PCIC_MBAR0: c_uint = 0x10;
pub const PCIC_MBAR0_OFFSET: c_int = 0;
pub const PCIC_MBAR1: c_uint = 0x18;
pub const PCIC_MBAR1_OFFSET: c_int = 2;
pub const PCIC_IOBAR: c_uint = 0x20;
pub const PCIC_IOBAR_OFFSET: c_int = 4;
pub const PCIC_SUBVENDOR_ID: c_uint = 0x2C;
pub const PCIC_SUBSYTEM_ID: c_uint = 0x2E;
pub const PCIX_STATUS: c_uint = 0x44;
pub const RCV_SCE: c_uint = 0x20000000;
pub const UNEXP_SC: c_uint = 0x00080000;
pub const SC_DISCARD: c_uint = 0x00040000;
pub const ECC_CTRL_STAT: c_uint = 0x48;
pub const UNCOR_ECCERR: c_uint = 0x00000008;
pub const PCIC_PM_CSR: c_uint = 0x5C;
pub const PWR_STATE_D0: c_int = 0;

pub const PWR_STATE_D3: c_int = 3;
pub const PCIC_BASE1: c_uint = 0x6C	/* internal use only */;
pub const BASE1_RSVD: c_uint = 0xFFFFFFF8;
pub const PCIC_BASEA: c_uint = 0x70	/* internal use only */;
pub const BASEA_RSVD: c_uint = 0xFFFFFFC0;
pub const BASEA_START: c_int = 0;
pub const PCIC_BASEB: c_uint = 0x74	/* internal use only */;
pub const BASEB_RSVD: c_uint = 0xFFFFFF80;
pub const BASEB_IOMAP_MASK: c_uint = 0x7F;
pub const BASEB_START: c_uint = 0x80;
pub const PCIC_BASEC: c_uint = 0x78	/* internal use only */;
pub const BASEC_RSVD: c_uint = 0xFFFFFFFC;
pub const BASEC_MASK: c_uint = 0x03;
pub const BASEC_START: c_uint = 0x58;
pub const PCIC_MBAR_KEY: c_uint = 0x7C	/* internal use only */;
pub const MBAR_KEY_MASK: c_uint = 0xFFFFFFFF;
pub const PCIC_HSTPCIX_CNTRL: c_uint = 0xA0;
pub const REWIND_DIS: c_uint = 0x0800;
pub const SC_TMR_DIS: c_uint = 0x04000000;
pub const PCIC_MBAR0_MASK: c_uint = 0xA8;
pub const PCIC_MBAR0_SIZE_MASK: c_uint = 0x1FFFE000;
pub const PCIC_MBAR0_SIZE_SHIFT: c_int = 13;

pub const PCIC_FLASH_MBAR: c_uint = 0xB8;
pub const PCIC_INTRPT_STAT: c_uint = 0xD4;
pub const PCIC_TP_CTRL: c_uint = 0xFC;
//
// EXSI Registers, Address Range: (0x00-0xFC)
//

pub const OCMINITIALIZED: c_uint = 0x80000000;
pub const ASIEN: c_uint = 0x00400000;
pub const HCMODE: c_uint = 0x00200000;
pub const PCIDEF: c_uint = 0x00100000;
pub const COMSTOCK: c_uint = 0x00080000;
pub const SEEPROMEND: c_uint = 0x00040000;
pub const MSTTIMEN: c_uint = 0x00020000;
pub const XREGEX: c_uint = 0x00000200;
pub const NVRAMW: c_uint = 0x00000100;
pub const NVRAMEX: c_uint = 0x00000080;
pub const SRAMW: c_uint = 0x00000040;
pub const SRAMEX: c_uint = 0x00000020;
pub const FLASHW: c_uint = 0x00000010;
pub const FLASHEX: c_uint = 0x00000008;
pub const SEEPROMCFG: c_uint = 0x00000004;
pub const SEEPROMTYP: c_uint = 0x00000002;
pub const SEEPROMEX: c_uint = 0x00000001;

pub const MODINT_EN: c_uint = 0x00000001;

pub const FLASHRST: c_uint = 0x00000002;
pub const FLASHRDY: c_uint = 0x00000001;

pub const FLWEH_MASK: c_uint = 0x30000000;
pub const FLWESU_MASK: c_uint = 0x0C000000;
pub const FLWEPW_MASK: c_uint = 0x03F00000;
pub const FLOEH_MASK: c_uint = 0x000C0000;
pub const FLOESU_MASK: c_uint = 0x00030000;
pub const FLOEPW_MASK: c_uint = 0x0000FC00;
pub const FLCSH_MASK: c_uint = 0x00000300;
pub const FLCSSU_MASK: c_uint = 0x000000C0;
pub const FLCSPW_MASK: c_uint = 0x0000003F;

pub const SRWEH_MASK: c_uint = 0x30000000;
pub const SRWESU_MASK: c_uint = 0x0C000000;
pub const SRWEPW_MASK: c_uint = 0x03F00000;
pub const SROEH_MASK: c_uint = 0x000C0000;
pub const SROESU_MASK: c_uint = 0x00030000;
pub const SROEPW_MASK: c_uint = 0x0000FC00;
pub const SRCSH_MASK: c_uint = 0x00000300;
pub const SRCSSU_MASK: c_uint = 0x000000C0;
pub const SRCSPW_MASK: c_uint = 0x0000003F;

pub const NVWEH_MASK: c_uint = 0x30000000;
pub const NVWESU_MASK: c_uint = 0x0C000000;
pub const NVWEPW_MASK: c_uint = 0x03F00000;
pub const NVOEH_MASK: c_uint = 0x000C0000;
pub const NVOESU_MASK: c_uint = 0x00030000;
pub const NVOEPW_MASK: c_uint = 0x0000FC00;
pub const NVCSH_MASK: c_uint = 0x00000300;
pub const NVCSSU_MASK: c_uint = 0x000000C0;
pub const NVCSPW_MASK: c_uint = 0x0000003F;

pub const XRWEH_MASK: c_uint = 0x30000000;
pub const XRWESU_MASK: c_uint = 0x0C000000;
pub const XRWEPW_MASK: c_uint = 0x03F00000;
pub const XROEH_MASK: c_uint = 0x000C0000;
pub const XROESU_MASK: c_uint = 0x00030000;
pub const XROEPW_MASK: c_uint = 0x0000FC00;
pub const XRCSH_MASK: c_uint = 0x00000300;
pub const XRCSSU_MASK: c_uint = 0x000000C0;
pub const XRCSPW_MASK: c_uint = 0x0000003F;

pub const XRADDRINCEN: c_uint = 0x80000000;
pub const XREGADD_MASK: c_uint = 0x007FFFFF;

pub const XREGDATA_MASK: c_uint = 0x0000FFFF;

pub const GPIO_EXTSRC: c_uint = 0x00000001;

pub const SXFERDONE: c_uint = 0x00000100;
pub const SXFERCNT_MASK: c_uint = 0x000000E0;
pub const SCMDTYP_MASK: c_uint = 0x0000001C;
pub const SXFERSTART: c_uint = 0x00000002;
pub const SXFEREN: c_uint = 0x00000001;

pub const SADDR_MASK: c_uint = 0x0000FFFF;

pub const ASIFMTERR: c_uint = 0x00000400;
pub const ASISEECHKERR: c_uint = 0x00000200;
pub const ASIERR: c_uint = 0x00000100;

pub const CHECKSUM_MASK: c_uint = 0x0000FFFF;

pub const CPI2ASIBYTECNT_MASK: c_uint = 0x00070000;
pub const CPI2ASIBYTEEN_MASK: c_uint = 0x0000F000;
pub const CPI2ASITARGERR_MASK: c_uint = 0x00000F00;
pub const CPI2ASITARGMID_MASK: c_uint = 0x000000F0;
pub const CPI2ASIMSTERR_MASK: c_uint = 0x0000000F;
//
// XSRAM, External SRAM (DWord and any BE pattern accessible)
//
pub const XSRAM_REG_BASE_ADDR: c_uint = 0xB8100000;
pub const XSRAM_SIZE: c_uint = 0x100000;
//
// NVRAM Registers, Address Range: (0x00000 - 0x3FFFF).
//
pub const NVRAM_REG_BASE_ADR: c_uint = 0xBF800000;
pub const NVRAM_MAX_BASE_ADR: c_uint = 0x003FFFFF;
// OCM base address
pub const OCM_BASE_ADDR: c_uint = 0xA0000000;
pub const OCM_MAX_SIZE: c_uint = 0x20000;
//
// Sequencers (Central and Link) Scratch RAM page definitions.
//
// The Central Management Sequencer (CSEQ) Scratch Memory is a 1024
// byte memory.  It is dword accessible and has byte parity
// protection. The CSEQ accesses it in 32 byte windows, either as mode
// dependent or mode independent memory. Each mode has 96 bytes,
// (three 32 byte pages 0-2, not contiguous), leaving 128 bytes of
// Mode Independent memory (four 32 byte pages 3-7). Note that mode
// dependent scratch memory, Mode 8, page 0-3 overlaps mode
// independent scratch memory, pages 0-3.
// - 896 bytes of mode dependent scratch, 96 bytes per Modes 0-7, and
// 128 bytes in mode 8,
// - 259 bytes of mode independent scratch, common to modes 0-15.
//
// Sequencer scratch RAM is 1024 bytes.  This scratch memory is
// divided into mode dependent and mode independent scratch with this
// memory further subdivided into pages of size 32 bytes. There are 5
// pages (160 bytes) of mode independent scratch and 3 pages of
// dependent scratch memory for modes 0-7 (768 bytes). Mode 8 pages
// 0-2 dependent scratch overlap with pages 0-2 of mode independent
// scratch memory.
//
// The host accesses this scratch in a different manner from the
// central sequencer. The sequencer has to use CSEQ registers CSCRPAGE
// and CMnSCRPAGE to access the scratch memory. A flat mapping of the
// scratch memory is available for software convenience and to prevent
// corruption while the sequencer is running. This memory is mapped
// onto addresses 800h - BFFh, total of 400h bytes.
//
// These addresses are mapped as follows:
//
// 800h-83Fh   Mode Dependent Scratch Mode 0 Pages 0-1
// 840h-87Fh   Mode Dependent Scratch Mode 1 Pages 0-1
// 880h-8BFh   Mode Dependent Scratch Mode 2 Pages 0-1
// 8C0h-8FFh   Mode Dependent Scratch Mode 3 Pages 0-1
// 900h-93Fh   Mode Dependent Scratch Mode 4 Pages 0-1
// 940h-97Fh   Mode Dependent Scratch Mode 5 Pages 0-1
// 980h-9BFh   Mode Dependent Scratch Mode 6 Pages 0-1
// 9C0h-9FFh   Mode Dependent Scratch Mode 7 Pages 0-1
// A00h-A5Fh   Mode Dependent Scratch Mode 8 Pages 0-2
// Mode Independent Scratch Pages 0-2
// A60h-A7Fh   Mode Dependent Scratch Mode 8 Page 3
// Mode Independent Scratch Page 3
// A80h-AFFh   Mode Independent Scratch Pages 4-7
// B00h-B1Fh   Mode Dependent Scratch Mode 0 Page 2
// B20h-B3Fh   Mode Dependent Scratch Mode 1 Page 2
// B40h-B5Fh   Mode Dependent Scratch Mode 2 Page 2
// B60h-B7Fh   Mode Dependent Scratch Mode 3 Page 2
// B80h-B9Fh   Mode Dependent Scratch Mode 4 Page 2
// BA0h-BBFh   Mode Dependent Scratch Mode 5 Page 2
// BC0h-BDFh   Mode Dependent Scratch Mode 6 Page 2
// BE0h-BFFh   Mode Dependent Scratch Mode 7 Page 2
//
// General macros

// All macros start with offsets from base + 0x800 (CMAPPEDSCR).
// Mode dependent scratch page 0, mode 0.
// For modes 1-7 you have to do arithmetic.

// Mode dependent scratch page 0 mode 8 macros.

// Mode dependent scratch page 1 mode 8 macros.

// Mode dependent scratch page 2 mode 8 macros

// Mode independent scratch page 4 macros.

// Mode independent scratch page 5 macros.

// Mode independent scratch page 6 macros.

// Mode independent scratch page 7 macros.

//
// Link m Sequencer scratch RAM is 512 bytes.
// This scratch memory is divided into mode dependent and mode
// independent scratch with this memory further subdivided into
// pages of size 32 bytes. There are 4 pages (128 bytes) of
// mode independent scratch and 4 pages of dependent scratch
// memory for modes 0-2 (384 bytes).
//
// The host accesses this scratch in a different manner from the
// link sequencer. The sequencer has to use LSEQ registers
// LmSCRPAGE and LmMnSCRPAGE to access the scratch memory. A flat
// mapping of the scratch memory is available for software
// convenience and to prevent corruption while the sequencer is
// running. This memory is mapped onto addresses 800h - 9FFh.
//
// These addresses are mapped as follows:
//
// 800h-85Fh   Mode Dependent Scratch Mode 0 Pages 0-2
// 860h-87Fh   Mode Dependent Scratch Mode 0 Page 3
// Mode Dependent Scratch Mode 5 Page 0
// 880h-8DFh   Mode Dependent Scratch Mode 1 Pages 0-2
// 8E0h-8FFh   Mode Dependent Scratch Mode 1 Page 3
// Mode Dependent Scratch Mode 5 Page 1
// 900h-95Fh   Mode Dependent Scratch Mode 2 Pages 0-2
// 960h-97Fh   Mode Dependent Scratch Mode 2 Page 3
// Mode Dependent Scratch Mode 5 Page 2
// 980h-9DFh   Mode Independent Scratch Pages 0-3
// 9E0h-9FFh   Mode Independent Scratch Page 3
// Mode Dependent Scratch Mode 5 Page 3
//
// General macros
pub const LSEQ_MODE_SCRATCH_SIZE: c_uint = 0x80 /* Size of scratch RAM per mode */;
pub const LSEQ_PAGE_SIZE: c_uint = 0x20 /* Scratch page size (in bytes) */;
pub const LSEQ_MODE5_PAGE0_OFFSET: c_uint = 0x60;
// Common mode dependent scratch page 0 macros for modes 0,1,2, and 5
// Indexed using LSEQ_MODE_SCRATCH_SIZE * mode, for modes 0,1,2.

// Mode flag macros (byte 0)
pub const SAS_SAVECTX_OCCURRED: c_uint = 0x80;
pub const SAS_OOBSVC_OCCURRED: c_uint = 0x40;
pub const SAS_OOB_DEVICE_PRESENT: c_uint = 0x20;
pub const SAS_CFGHDR_OCCURRED: c_uint = 0x10;
pub const SAS_RCV_INTS_ARE_DISABLED: c_uint = 0x08;
pub const SAS_OOB_HOT_PLUG_CNCT: c_uint = 0x04;
pub const SAS_AWAIT_OPEN_CONNECTION: c_uint = 0x02;
pub const SAS_CFGCMPLT_OCCURRED: c_uint = 0x01;
// Mode flag macros (byte 1)
pub const SAS_RLSSCB_OCCURRED: c_uint = 0x80;
pub const SAS_FORCED_HEADER_MISS: c_uint = 0x40;

// Mode dependent scratch page 0 macros for mode 0 (non-common)
// Absolute offsets

// Mode dependent scratch page 0 macros for mode 1 (non-common)
// Absolute offsets

// Mode dependent scratch page 0 macros for mode 2 (non-common)

// Mode dependent scratch page 0 macros for modes 4/5 (non-common)
// Absolute offsets

// Mode dependent scratch page 1, mode 0 and mode 1

// Mode dependent scratch page 1 macros for mode 2
// Absolute offsets

// Mode dependent scratch page 1 macros for mode 4/5

// Mode dependent scratch page 2 macros for mode 0
// Absolute offsets

// Mode dependent scratch page 2 macros for mode 1
// Absolute offsets
// byte 0 bits 1-0 are domain select.

// Mode dependent scratch page 2 macros for mode 2
// Absolute offsets

// Mode dependent scratch page 2 macros for mode 5

// Mode dependent scratch page 3 macros for modes 0 and 1
// None defined
// Mode dependent scratch page 3 macros for modes 2 and 5
// None defined
// Mode Independent Scratch page 0 macros.

//
// Currently only bit 0, SAS_DWSAQD, is used.
//
pub const SAS_DWSAQD: c_uint = 0x01  /*;
// DWSSTATUS: DWSAQD
// bit las read in ISR.
//

// Connection states (byte 0)
pub const SAS_WE_OPENED_CS: c_uint = 0x01;
pub const SAS_DEVICE_OPENED_CS: c_uint = 0x02;
pub const SAS_WE_SENT_DONE_CS: c_uint = 0x04;
pub const SAS_DEVICE_SENT_DONE_CS: c_uint = 0x08;
pub const SAS_WE_SENT_CLOSE_CS: c_uint = 0x10;
pub const SAS_DEVICE_SENT_CLOSE_CS: c_uint = 0x20;
pub const SAS_WE_SENT_BREAK_CS: c_uint = 0x40;
pub const SAS_DEVICE_SENT_BREAK_CS: c_uint = 0x80;
// Connection states (byte 1)
pub const SAS_OPN_TIMEOUT_OR_OPN_RJCT_CS: c_uint = 0x01;
pub const SAS_AIP_RECEIVED_CS: c_uint = 0x02;
pub const SAS_CREDIT_TIMEOUT_OCCURRED_CS: c_uint = 0x04;
pub const SAS_ACKNAK_TIMEOUT_OCCURRED_CS: c_uint = 0x08;
pub const SAS_SMPRSP_TIMEOUT_OCCURRED_CS: c_uint = 0x10;
pub const SAS_DONE_TIMEOUT_OCCURRED_CS: c_uint = 0x20;
// Connection states (byte 2)
pub const SAS_SMP_RESPONSE_RECEIVED_CS: c_uint = 0x01;
pub const SAS_INTLK_TIMEOUT_OCCURRED_CS: c_uint = 0x02;
pub const SAS_DEVICE_SENT_DMAT_CS: c_uint = 0x04;
pub const SAS_DEVICE_SENT_SYNCSRST_CS: c_uint = 0x08;
pub const SAS_CLEARING_AFFILIATION_CS: c_uint = 0x20;
pub const SAS_RXTASK_ACTIVE_CS: c_uint = 0x40;
pub const SAS_TXTASK_ACTIVE_CS: c_uint = 0x80;
// Connection states (byte 3)
pub const SAS_PHY_LOSS_OF_SIGNAL_CS: c_uint = 0x01;
pub const SAS_DWS_TIMER_EXPIRED_CS: c_uint = 0x02;
pub const SAS_LINK_RESET_NOT_COMPLETE_CS: c_uint = 0x04;
pub const SAS_PHY_DISABLED_CS: c_uint = 0x08;
pub const SAS_LINK_CTL_TASK_ACTIVE_CS: c_uint = 0x10;
pub const SAS_PHY_EVENT_TASK_ACTIVE_CS: c_uint = 0x20;
pub const SAS_DEVICE_SENT_ID_FRAME_CS: c_uint = 0x40;
pub const SAS_DEVICE_SENT_REG_FIS_CS: c_uint = 0x40;
pub const SAS_DEVICE_SENT_HARD_RESET_CS: c_uint = 0x80;

// Mode independent scratch page 1 macros.

// Mode independent scratch page 2 macros.

// Mode independent scratch page 3 macros.

