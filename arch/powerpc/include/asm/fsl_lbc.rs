//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/fsl_lbc.h
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
// Freescale Local Bus Controller
//
// Copyright © 2006-2007, 2010 Freescale Semiconductor
//
// Authors: Nick Spence <nick.spence@freescale.com>,
// Scott Wood <scottwood@freescale.com>
// Jack Lan <jack.lan@freescale.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_lbc_bank {
    pub /: *mut *mut *mut __be32 br; /< Base Register,
pub const BR_BA: c_uint = 0xFFFF8000;
pub const BR_BA_SHIFT: c_int = 15;
pub const BR_PS: c_uint = 0x00001800;
pub const BR_PS_SHIFT: c_int = 11;
pub const BR_PS_8: c_uint = 0x00000800  /* Port Size 8 bit */;
pub const BR_PS_16: c_uint = 0x00001000  /* Port Size 16 bit */;
pub const BR_PS_32: c_uint = 0x00001800  /* Port Size 32 bit */;
pub const BR_DECC: c_uint = 0x00000600;
pub const BR_DECC_SHIFT: c_int = 9;
pub const BR_DECC_OFF: c_uint = 0x00000000  /* HW ECC checking and generation off */;
pub const BR_DECC_CHK: c_uint = 0x00000200  /* HW ECC checking on, generation off */;
pub const BR_DECC_CHK_GEN: c_uint = 0x00000400  /* HW ECC checking and generation on */;
pub const BR_WP: c_uint = 0x00000100;
pub const BR_WP_SHIFT: c_int = 8;
pub const BR_MSEL: c_uint = 0x000000E0;
pub const BR_MSEL_SHIFT: c_int = 5;
pub const BR_MS_GPCM: c_uint = 0x00000000  /* GPCM */;
pub const BR_MS_FCM: c_uint = 0x00000020  /* FCM */;
pub const BR_MS_SDRAM: c_uint = 0x00000060  /* SDRAM */;
pub const BR_MS_UPMA: c_uint = 0x00000080  /* UPMA */;
pub const BR_MS_UPMB: c_uint = 0x000000A0  /* UPMB */;
pub const BR_MS_UPMC: c_uint = 0x000000C0  /* UPMC */;
pub const BR_V: c_uint = 0x00000001;
pub const BR_V_SHIFT: c_int = 0;

    pub /: *mut *mut *mut __be32 or; /< Base Register,
pub const OR0: c_uint = 0x5004;
pub const OR1: c_uint = 0x500C;
pub const OR2: c_uint = 0x5014;
pub const OR3: c_uint = 0x501C;
pub const OR4: c_uint = 0x5024;
pub const OR5: c_uint = 0x502C;
pub const OR6: c_uint = 0x5034;
pub const OR7: c_uint = 0x503C;
pub const OR_FCM_AM: c_uint = 0xFFFF8000;
pub const OR_FCM_AM_SHIFT: c_int = 15;
pub const OR_FCM_BCTLD: c_uint = 0x00001000;
pub const OR_FCM_BCTLD_SHIFT: c_int = 12;
pub const OR_FCM_PGS: c_uint = 0x00000400;
pub const OR_FCM_PGS_SHIFT: c_int = 10;
pub const OR_FCM_CSCT: c_uint = 0x00000200;
pub const OR_FCM_CSCT_SHIFT: c_int = 9;
pub const OR_FCM_CST: c_uint = 0x00000100;
pub const OR_FCM_CST_SHIFT: c_int = 8;
pub const OR_FCM_CHT: c_uint = 0x00000080;
pub const OR_FCM_CHT_SHIFT: c_int = 7;
pub const OR_FCM_SCY: c_uint = 0x00000070;
pub const OR_FCM_SCY_SHIFT: c_int = 4;
pub const OR_FCM_SCY_1: c_uint = 0x00000010;
pub const OR_FCM_SCY_2: c_uint = 0x00000020;
pub const OR_FCM_SCY_3: c_uint = 0x00000030;
pub const OR_FCM_SCY_4: c_uint = 0x00000040;
pub const OR_FCM_SCY_5: c_uint = 0x00000050;
pub const OR_FCM_SCY_6: c_uint = 0x00000060;
pub const OR_FCM_SCY_7: c_uint = 0x00000070;
pub const OR_FCM_RST: c_uint = 0x00000008;
pub const OR_FCM_RST_SHIFT: c_int = 3;
pub const OR_FCM_TRLX: c_uint = 0x00000004;
pub const OR_FCM_TRLX_SHIFT: c_int = 2;
pub const OR_FCM_EHTR: c_uint = 0x00000002;
pub const OR_FCM_EHTR_SHIFT: c_int = 1;
pub const OR_GPCM_AM: c_uint = 0xFFFF8000;
pub const OR_GPCM_AM_SHIFT: c_int = 15;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_lbc_regs {
    pub bank: [fsl_lbc_bank; 12],
    pub res0: [u8; 0x8],
    pub /: *mut *mut *mut __be32 mar; /< UPM Address Register,
    pub res1: [u8; 0x4],
    pub /: *mut *mut *mut __be32 mamr; /< UPMA Mode Register,

pub const MxMR_MAD: c_uint = 0x3f      /**< machine address */;
    pub /: *mut *mut *mut __be32 mbmr; /< UPMB Mode Register,
    pub /: *mut *mut *mut __be32 mcmr; /< UPMC Mode Register,
    pub res2: [u8; 0x8],
    pub /: *mut *mut *mut __be32 mrtpr; /< Memory Refresh Timer Prescaler Register,
    pub /: *mut *mut *mut __be32 mdr; /< UPM Data Register,
    pub res3: [u8; 0x4],
    pub /: *mut *mut *mut __be32 lsor; /< Special Operation Initiation Register,
    pub /: *mut *mut *mut __be32 lsdmr; /< SDRAM Mode Register,
    pub res4: [u8; 0x8],
    pub /: *mut *mut *mut __be32 lurt; /< UPM Refresh Timer,
    pub /: *mut *mut *mut __be32 lsrt; /< SDRAM Refresh Timer,
    pub res5: [u8; 0x8],
    pub /: *mut *mut *mut __be32 ltesr; /< Transfer Error Status Register,
pub const LTESR_BM: c_uint = 0x80000000;
pub const LTESR_FCT: c_uint = 0x40000000;
pub const LTESR_PAR: c_uint = 0x20000000;
pub const LTESR_WP: c_uint = 0x04000000;
pub const LTESR_ATMW: c_uint = 0x00800000;
pub const LTESR_ATMR: c_uint = 0x00400000;
pub const LTESR_CS: c_uint = 0x00080000;
pub const LTESR_UPM: c_uint = 0x00000002;
pub const LTESR_CC: c_uint = 0x00000001;

pub const LTESR_CLEAR: c_uint = 0xFFFFFFFF;
pub const LTECCR_CLEAR: c_uint = 0xFFFFFFFF;

pub const LTEDR_ENABLE: c_uint = 0x00000000;
    pub /: *mut *mut *mut __be32 ltedr; /< Transfer Error Disable Register,
    pub /: *mut *mut *mut __be32 lteir; /< Transfer Error Interrupt Register,
    pub /: *mut *mut *mut __be32 lteatr; /< Transfer Error Attributes Register,
    pub /: *mut *mut *mut __be32 ltear; /< Transfer Error Address Register,
    pub /: *mut *mut *mut __be32 lteccr; /< Transfer Error ECC Register,
    pub res6: [u8; 0x8],
    pub /: *mut *mut *mut __be32 lbcr; /< Configuration Register,
pub const LBCR_LDIS: c_uint = 0x80000000;
pub const LBCR_LDIS_SHIFT: c_int = 31;
pub const LBCR_BCTLC: c_uint = 0x00C00000;
pub const LBCR_BCTLC_SHIFT: c_int = 22;
pub const LBCR_AHD: c_uint = 0x00200000;
pub const LBCR_LPBSE: c_uint = 0x00020000;
pub const LBCR_LPBSE_SHIFT: c_int = 17;
pub const LBCR_EPAR: c_uint = 0x00010000;
pub const LBCR_EPAR_SHIFT: c_int = 16;
pub const LBCR_BMT: c_uint = 0x0000FF00;
pub const LBCR_BMT_SHIFT: c_int = 8;
pub const LBCR_BMTPS: c_uint = 0x0000000F;
pub const LBCR_BMTPS_SHIFT: c_int = 0;
pub const LBCR_INIT: c_uint = 0x00040000;
    pub /: *mut *mut *mut __be32 lcrr; /< Clock Ratio Register,
pub const LCRR_DBYP: c_uint = 0x80000000;
pub const LCRR_DBYP_SHIFT: c_int = 31;
pub const LCRR_BUFCMDC: c_uint = 0x30000000;
pub const LCRR_BUFCMDC_SHIFT: c_int = 28;
pub const LCRR_ECL: c_uint = 0x03000000;
pub const LCRR_ECL_SHIFT: c_int = 24;
pub const LCRR_EADC: c_uint = 0x00030000;
pub const LCRR_EADC_SHIFT: c_int = 16;
pub const LCRR_CLKDIV: c_uint = 0x0000000F;
pub const LCRR_CLKDIV_SHIFT: c_int = 0;
    pub res7: [u8; 0x8],
    pub /: *mut *mut *mut __be32 fmr; /< Flash Mode Register,
pub const FMR_CWTO: c_uint = 0x0000F000;
pub const FMR_CWTO_SHIFT: c_int = 12;
pub const FMR_BOOT: c_uint = 0x00000800;
pub const FMR_ECCM: c_uint = 0x00000100;
pub const FMR_AL: c_uint = 0x00000030;
pub const FMR_AL_SHIFT: c_int = 4;
pub const FMR_OP: c_uint = 0x00000003;
pub const FMR_OP_SHIFT: c_int = 0;
    pub /: *mut *mut *mut __be32 fir; /< Flash Instruction Register,
pub const FIR_OP0: c_uint = 0xF0000000;
pub const FIR_OP0_SHIFT: c_int = 28;
pub const FIR_OP1: c_uint = 0x0F000000;
pub const FIR_OP1_SHIFT: c_int = 24;
pub const FIR_OP2: c_uint = 0x00F00000;
pub const FIR_OP2_SHIFT: c_int = 20;
pub const FIR_OP3: c_uint = 0x000F0000;
pub const FIR_OP3_SHIFT: c_int = 16;
pub const FIR_OP4: c_uint = 0x0000F000;
pub const FIR_OP4_SHIFT: c_int = 12;
pub const FIR_OP5: c_uint = 0x00000F00;
pub const FIR_OP5_SHIFT: c_int = 8;
pub const FIR_OP6: c_uint = 0x000000F0;
pub const FIR_OP6_SHIFT: c_int = 4;
pub const FIR_OP7: c_uint = 0x0000000F;
pub const FIR_OP7_SHIFT: c_int = 0;
pub const FIR_OP_NOP: c_uint = 0x0	/* No operation and end of sequence */;
pub const FIR_OP_CA: c_uint = 0x1        /* Issue current column address */;
pub const FIR_OP_PA: c_uint = 0x2        /* Issue current block+page address */;
pub const FIR_OP_UA: c_uint = 0x3        /* Issue user defined address */;
pub const FIR_OP_CM0: c_uint = 0x4        /* Issue command from FCR[CMD0] */;
pub const FIR_OP_CM1: c_uint = 0x5        /* Issue command from FCR[CMD1] */;
pub const FIR_OP_CM2: c_uint = 0x6        /* Issue command from FCR[CMD2] */;
pub const FIR_OP_CM3: c_uint = 0x7        /* Issue command from FCR[CMD3] */;
pub const FIR_OP_WB: c_uint = 0x8        /* Write FBCR bytes from FCM buffer */;
pub const FIR_OP_WS: c_uint = 0x9        /* Write 1 or 2 bytes from MDR[AS] */;
pub const FIR_OP_RB: c_uint = 0xA        /* Read FBCR bytes to FCM buffer */;
pub const FIR_OP_RS: c_uint = 0xB        /* Read 1 or 2 bytes to MDR[AS] */;
pub const FIR_OP_CW0: c_uint = 0xC        /* Wait then issue FCR[CMD0] */;
pub const FIR_OP_CW1: c_uint = 0xD        /* Wait then issue FCR[CMD1] */;
pub const FIR_OP_RBW: c_uint = 0xE        /* Wait then read FBCR bytes */;
pub const FIR_OP_RSW: c_uint = 0xE        /* Wait then read 1 or 2 bytes */;
    pub /: *mut *mut *mut __be32 fcr; /< Flash Command Register,
pub const FCR_CMD0: c_uint = 0xFF000000;
pub const FCR_CMD0_SHIFT: c_int = 24;
pub const FCR_CMD1: c_uint = 0x00FF0000;
pub const FCR_CMD1_SHIFT: c_int = 16;
pub const FCR_CMD2: c_uint = 0x0000FF00;
pub const FCR_CMD2_SHIFT: c_int = 8;
pub const FCR_CMD3: c_uint = 0x000000FF;
pub const FCR_CMD3_SHIFT: c_int = 0;
    pub /: *mut *mut *mut __be32 fbar; /< Flash Block Address Register,
pub const FBAR_BLK: c_uint = 0x00FFFFFF;
    pub /: *mut *mut *mut __be32 fpar; /< Flash Page Address Register,
pub const FPAR_SP_PI: c_uint = 0x00007C00;
pub const FPAR_SP_PI_SHIFT: c_int = 10;
pub const FPAR_SP_MS: c_uint = 0x00000200;
pub const FPAR_SP_CI: c_uint = 0x000001FF;
pub const FPAR_SP_CI_SHIFT: c_int = 0;
pub const FPAR_LP_PI: c_uint = 0x0003F000;
pub const FPAR_LP_PI_SHIFT: c_int = 12;
pub const FPAR_LP_MS: c_uint = 0x00000800;
pub const FPAR_LP_CI: c_uint = 0x000007FF;
pub const FPAR_LP_CI_SHIFT: c_int = 0;
    pub /: *mut *mut *mut __be32 fbcr; /< Flash Byte Count Register,
pub const FBCR_BC: c_uint = 0x00000FFF;
}

//
// FSL UPM routines
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_upm {
    pub mxmr: *mut __be32 __iomem,
    pub width: c_int,
}

extern "C" {
    pub fn fsl_lbc_addr(addr_base: phys_addr_t) -> u32;
}
extern "C" {
    pub fn fsl_lbc_find(addr_base: phys_addr_t) -> c_int;
}
extern "C" {
    pub fn fsl_upm_find(addr_base: phys_addr_t, upm: *mut fsl_upm) -> c_int;
}
//
// fsl_upm_start_pattern - start UPM patterns execution
// @upm:	pointer to the fsl_upm structure obtained via fsl_upm_find
// @pat_offset:	UPM pattern offset for the command to be executed
//
// This routine programmes UPM so the next memory access that hits an UPM
// will trigger pattern execution, starting at pat_offset.
//
// fsl_upm_end_pattern - end UPM patterns execution
// @upm:	pointer to the fsl_upm structure obtained via fsl_upm_find
//
// This routine reverts UPM to normal operation mode.
//
// overview of the fsl lbc controller
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_lbc_ctrl {
// device info
    pub dev: *mut device,
    pub regs: *mut fsl_lbc_regs __iomem,
    pub irq: [c_int; 2],
    pub irq_wait: wait_queue_head_t,
    pub lock: spinlock_t,
    pub nand: *mut c_void,
// status read from LTESR by irq handler
    pub irq_status: c_uint,

// save regs when system go to deep-sleep
    pub saved_regs: *mut fsl_lbc_regs,

}
