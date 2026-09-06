//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/ibm/emac/emac.h
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
// drivers/net/ethernet/ibm/emac/emac.h
//
// Register definitions for PowerPC 4xx on-chip ethernet contoller
//
// Copyright 2007 Benjamin Herrenschmidt, IBM Corp.
// <benh@kernel.crashing.org>
//
// Based on the arch/ppc version of the driver:
//
// Copyright (c) 2004, 2005 Zultys Technologies.
// Eugene Surovegin <eugene.surovegin@zultys.com> or <ebs@ebshome.net>
//
// Based on original work by
// Matt Porter <mporter@kernel.crashing.org>
// Armin Kuster <akuster@mvista.com>
// Copyright 2002-2004 MontaVista Software Inc.
//

// EMAC registers 			Write Access rules
#[repr(C)]
#[derive(Copy, Clone)]
pub struct emac_regs {
// Common registers across all EMAC implementations.
    pub /: *mut *mut u32 mr0; / Special,
    pub /: *mut *mut u32 mr1; / Reset,
    pub /: *mut *mut u32 tmr0; / Special,
    pub /: *mut *mut u32 tmr1; / Special,
    pub /: *mut *mut u32 rmr; / Reset,
    pub /: *mut *mut u32 isr; / Always,
    pub /: *mut *mut u32 iser; / Reset,
    pub /: *mut *mut u32 iahr; / Reset, R, T,
    pub /: *mut *mut u32 ialr; / Reset, R, T,
    pub /: *mut *mut u32 vtpid; / Reset, R, T,
    pub /: *mut *mut u32 vtci; / Reset, R, T,
    pub /: *mut *mut u32 ptr; / Reset, T,
// Registers unique to EMAC4 implementations
    pub /: *mut *mut u32 iaht1; / Reset, R,
    pub /: *mut *mut u32 iaht2; / Reset, R,
    pub /: *mut *mut u32 iaht3; / Reset, R,
    pub /: *mut *mut u32 iaht4; / Reset, R,
    pub /: *mut *mut u32 gaht1; / Reset, R,
    pub /: *mut *mut u32 gaht2; / Reset, R,
    pub /: *mut *mut u32 gaht3; / Reset, R,
    pub /: *mut *mut u32 gaht4; / Reset, R,
    pub emac4: },
// Registers unique to EMAC4SYNC implementations
    pub /: *mut *mut u32 mahr; / Reset, R, T,
    pub /: *mut *mut u32 malr; / Reset, R, T,
    pub /: *mut *mut u32 mmahr; / Reset, R, T,
    pub /: *mut *mut u32 mmalr; / Reset, R, T,
    pub rsvd0: [u32; 4],
    pub emac4sync: },
    pub u0: },
// Common registers across all EMAC implementations.
    pub lsah: u32,
    pub lsal: u32,
    pub /: *mut *mut u32 ipgvr; / Reset, T,
    pub /: *mut *mut u32 stacr; / Special,
    pub /: *mut *mut u32 trtr; / Special,
    pub /: *mut *mut u32 rwmr; / Reset,
    pub octx: u32,
    pub ocrx: u32,
// Registers unique to EMAC4 implementations
    pub ipcr: u32,
    pub emac4: },
// Registers unique to EMAC4SYNC implementations
    pub rsvd1: u32,
    pub revid: u32,
    pub rsvd2: [u32; 2],
    pub /: *mut *mut u32 iaht1; / Reset, R,
    pub /: *mut *mut u32 iaht2; / Reset, R,
    pub /: *mut *mut u32 iaht3; / Reset, R,
    pub /: *mut *mut u32 iaht4; / Reset, R,
    pub /: *mut *mut u32 iaht5; / Reset, R,
    pub /: *mut *mut u32 iaht6; / Reset, R,
    pub /: *mut *mut u32 iaht7; / Reset, R,
    pub /: *mut *mut u32 iaht8; / Reset, R,
    pub /: *mut *mut u32 gaht1; / Reset, R,
    pub /: *mut *mut u32 gaht2; / Reset, R,
    pub /: *mut *mut u32 gaht3; / Reset, R,
    pub /: *mut *mut u32 gaht4; / Reset, R,
    pub /: *mut *mut u32 gaht5; / Reset, R,
    pub /: *mut *mut u32 gaht6; / Reset, R,
    pub /: *mut *mut u32 gaht7; / Reset, R,
    pub /: *mut *mut u32 gaht8; / Reset, R,
    pub /: *mut *mut u32 tpc; / Reset, T,
    pub emac4sync: },
    pub u1: },
}

// EMACx_MR0
pub const EMAC_MR0_RXI: c_uint = 0x80000000;
pub const EMAC_MR0_TXI: c_uint = 0x40000000;
pub const EMAC_MR0_SRST: c_uint = 0x20000000;
pub const EMAC_MR0_TXE: c_uint = 0x10000000;
pub const EMAC_MR0_RXE: c_uint = 0x08000000;
pub const EMAC_MR0_WKE: c_uint = 0x04000000;
// EMACx_MR1
pub const EMAC_MR1_FDE: c_uint = 0x80000000;
pub const EMAC_MR1_ILE: c_uint = 0x40000000;
pub const EMAC_MR1_VLE: c_uint = 0x20000000;
pub const EMAC_MR1_EIFC: c_uint = 0x10000000;
pub const EMAC_MR1_APP: c_uint = 0x08000000;
pub const EMAC_MR1_IST: c_uint = 0x01000000;
pub const EMAC_MR1_MF_MASK: c_uint = 0x00c00000;
pub const EMAC_MR1_MF_10: c_uint = 0x00000000;
pub const EMAC_MR1_MF_100: c_uint = 0x00400000;
pub const EMAC_MR1_MF_1000: c_uint = 0x00800000;
pub const EMAC_MR1_MF_1000GPCS: c_uint = 0x00c00000;

pub const EMAC_MR1_RFS_4K: c_uint = 0x00300000;
pub const EMAC_MR1_RFS_16K: c_uint = 0x00000000;
pub const EMAC_MR1_TFS_2K: c_uint = 0x00080000;
pub const EMAC_MR1_TR0_MULT: c_uint = 0x00008000;
pub const EMAC_MR1_JPSM: c_uint = 0x00000000;
pub const EMAC_MR1_MWSW_001: c_uint = 0x00000000;

pub const EMAC4_MR1_RFS_2K: c_uint = 0x00100000;
pub const EMAC4_MR1_RFS_4K: c_uint = 0x00180000;
pub const EMAC4_MR1_RFS_8K: c_uint = 0x00200000;
pub const EMAC4_MR1_RFS_16K: c_uint = 0x00280000;
pub const EMAC4_MR1_TFS_2K: c_uint = 0x00020000;
pub const EMAC4_MR1_TFS_4K: c_uint = 0x00030000;
pub const EMAC4_MR1_TFS_8K: c_uint = 0x00040000;
pub const EMAC4_MR1_TFS_16K: c_uint = 0x00050000;
pub const EMAC4_MR1_TR: c_uint = 0x00008000;
pub const EMAC4_MR1_MWSW_001: c_uint = 0x00001000;
pub const EMAC4_MR1_JPSM: c_uint = 0x00000800;
pub const EMAC4_MR1_OBCI_MASK: c_uint = 0x00000038;
pub const EMAC4_MR1_OBCI_50: c_uint = 0x00000000;
pub const EMAC4_MR1_OBCI_66: c_uint = 0x00000008;
pub const EMAC4_MR1_OBCI_83: c_uint = 0x00000010;
pub const EMAC4_MR1_OBCI_100: c_uint = 0x00000018;
pub const EMAC4_MR1_OBCI_100P: c_uint = 0x00000020;

// EMACx_TMR0
pub const EMAC_TMR0_GNP: c_uint = 0x80000000;
pub const EMAC_TMR0_DEFAULT: c_uint = 0x00000000;
pub const EMAC4_TMR0_TFAE_2_32: c_uint = 0x00000001;
pub const EMAC4_TMR0_TFAE_4_64: c_uint = 0x00000002;
pub const EMAC4_TMR0_TFAE_8_128: c_uint = 0x00000003;
pub const EMAC4_TMR0_TFAE_16_256: c_uint = 0x00000004;
pub const EMAC4_TMR0_TFAE_32_512: c_uint = 0x00000005;
pub const EMAC4_TMR0_TFAE_64_1024: c_uint = 0x00000006;
pub const EMAC4_TMR0_TFAE_128_2048: c_uint = 0x00000007;

// EMACx_TMR1

// EMACx_RMR
pub const EMAC_RMR_SP: c_uint = 0x80000000;
pub const EMAC_RMR_SFCS: c_uint = 0x40000000;
pub const EMAC_RMR_RRP: c_uint = 0x20000000;
pub const EMAC_RMR_RFP: c_uint = 0x10000000;
pub const EMAC_RMR_ROP: c_uint = 0x08000000;
pub const EMAC_RMR_RPIR: c_uint = 0x04000000;
pub const EMAC_RMR_PPP: c_uint = 0x02000000;
pub const EMAC_RMR_PME: c_uint = 0x01000000;
pub const EMAC_RMR_PMME: c_uint = 0x00800000;
pub const EMAC_RMR_IAE: c_uint = 0x00400000;
pub const EMAC_RMR_MIAE: c_uint = 0x00200000;
pub const EMAC_RMR_BAE: c_uint = 0x00100000;
pub const EMAC_RMR_MAE: c_uint = 0x00080000;
pub const EMAC_RMR_BASE: c_uint = 0x00000000;
pub const EMAC4_RMR_RFAF_2_32: c_uint = 0x00000001;
pub const EMAC4_RMR_RFAF_4_64: c_uint = 0x00000002;
pub const EMAC4_RMR_RFAF_8_128: c_uint = 0x00000003;
pub const EMAC4_RMR_RFAF_16_256: c_uint = 0x00000004;
pub const EMAC4_RMR_RFAF_32_512: c_uint = 0x00000005;
pub const EMAC4_RMR_RFAF_64_1024: c_uint = 0x00000006;
pub const EMAC4_RMR_RFAF_128_2048: c_uint = 0x00000007;

pub const EMAC4_RMR_MJS_MASK: c_uint = 0x0001fff8;

// EMACx_ISR & EMACx_ISER
pub const EMAC4_ISR_TXPE: c_uint = 0x20000000;
pub const EMAC4_ISR_RXPE: c_uint = 0x10000000;
pub const EMAC4_ISR_TXUE: c_uint = 0x08000000;
pub const EMAC4_ISR_RXOE: c_uint = 0x04000000;
pub const EMAC_ISR_OVR: c_uint = 0x02000000;
pub const EMAC_ISR_PP: c_uint = 0x01000000;
pub const EMAC_ISR_BP: c_uint = 0x00800000;
pub const EMAC_ISR_RP: c_uint = 0x00400000;
pub const EMAC_ISR_SE: c_uint = 0x00200000;
pub const EMAC_ISR_ALE: c_uint = 0x00100000;
pub const EMAC_ISR_BFCS: c_uint = 0x00080000;
pub const EMAC_ISR_PTLE: c_uint = 0x00040000;
pub const EMAC_ISR_ORE: c_uint = 0x00020000;
pub const EMAC_ISR_IRE: c_uint = 0x00010000;
pub const EMAC_ISR_SQE: c_uint = 0x00000080;
pub const EMAC_ISR_TE: c_uint = 0x00000040;
pub const EMAC_ISR_MOS: c_uint = 0x00000002;
pub const EMAC_ISR_MOF: c_uint = 0x00000001;
// EMACx_STACR
pub const EMAC_STACR_PHYD_MASK: c_uint = 0xffff;
pub const EMAC_STACR_PHYD_SHIFT: c_int = 16;
pub const EMAC_STACR_OC: c_uint = 0x00008000;
pub const EMAC_STACR_PHYE: c_uint = 0x00004000;
pub const EMAC_STACR_STAC_MASK: c_uint = 0x00003000;
pub const EMAC_STACR_STAC_READ: c_uint = 0x00001000;
pub const EMAC_STACR_STAC_WRITE: c_uint = 0x00002000;
pub const EMAC_STACR_OPBC_MASK: c_uint = 0x00000C00;
pub const EMAC_STACR_OPBC_50: c_uint = 0x00000000;
pub const EMAC_STACR_OPBC_66: c_uint = 0x00000400;
pub const EMAC_STACR_OPBC_83: c_uint = 0x00000800;
pub const EMAC_STACR_OPBC_100: c_uint = 0x00000C00;

pub const EMAC4_STACR_BASE(opb): c_uint = 0x00000000;
pub const EMAC_STACR_PCDA_MASK: c_uint = 0x1f;
pub const EMAC_STACR_PCDA_SHIFT: c_int = 5;
pub const EMAC_STACR_PRA_MASK: c_uint = 0x1f;
pub const EMACX_STACR_STAC_MASK: c_uint = 0x00003800;
pub const EMACX_STACR_STAC_READ: c_uint = 0x00001000;
pub const EMACX_STACR_STAC_WRITE: c_uint = 0x00000800;
pub const EMACX_STACR_STAC_IND_ADDR: c_uint = 0x00002000;
pub const EMACX_STACR_STAC_IND_READ: c_uint = 0x00003800;
pub const EMACX_STACR_STAC_IND_READINC: c_uint = 0x00003000;
pub const EMACX_STACR_STAC_IND_WRITE: c_uint = 0x00002800;
// EMACx_TRTR
pub const EMAC_TRTR_SHIFT_EMAC4: c_int = 24;
pub const EMAC_TRTR_SHIFT: c_int = 27;
// EMAC specific TX descriptor control fields (write access)
pub const EMAC_TX_CTRL_GFCS: c_uint = 0x0200;
pub const EMAC_TX_CTRL_GP: c_uint = 0x0100;
pub const EMAC_TX_CTRL_ISA: c_uint = 0x0080;
pub const EMAC_TX_CTRL_RSA: c_uint = 0x0040;
pub const EMAC_TX_CTRL_IVT: c_uint = 0x0020;
pub const EMAC_TX_CTRL_RVT: c_uint = 0x0010;
pub const EMAC_TX_CTRL_TAH_CSUM: c_uint = 0x000e;
// EMAC specific TX descriptor status fields (read access)
pub const EMAC_TX_ST_BFCS: c_uint = 0x0200;
pub const EMAC_TX_ST_LCS: c_uint = 0x0080;
pub const EMAC_TX_ST_ED: c_uint = 0x0040;
pub const EMAC_TX_ST_EC: c_uint = 0x0020;
pub const EMAC_TX_ST_LC: c_uint = 0x0010;
pub const EMAC_TX_ST_MC: c_uint = 0x0008;
pub const EMAC_TX_ST_SC: c_uint = 0x0004;
pub const EMAC_TX_ST_UR: c_uint = 0x0002;
pub const EMAC_TX_ST_SQE: c_uint = 0x0001;

// EMAC specific RX descriptor status fields (read access)
pub const EMAC_RX_ST_OE: c_uint = 0x0200;
pub const EMAC_RX_ST_PP: c_uint = 0x0100;
pub const EMAC_RX_ST_BP: c_uint = 0x0080;
pub const EMAC_RX_ST_RP: c_uint = 0x0040;
pub const EMAC_RX_ST_SE: c_uint = 0x0020;
pub const EMAC_RX_ST_AE: c_uint = 0x0010;
pub const EMAC_RX_ST_BFCS: c_uint = 0x0008;
pub const EMAC_RX_ST_PTL: c_uint = 0x0004;
pub const EMAC_RX_ST_ORE: c_uint = 0x0002;
pub const EMAC_RX_ST_IRE: c_uint = 0x0001;
pub const EMAC_RX_TAH_BAD_CSUM: c_uint = 0x0003;

