//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/renesas/ravb.h
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
// Renesas Ethernet AVB device driver
//
// Copyright (C) 2014-2015 Renesas Electronics Corporation
// Copyright (C) 2015 Renesas Solutions Corp.
// Copyright (C) 2015-2016 Cogent Embedded, Inc. <source@cogentembedded.com>
//
// Based on the SuperH Ethernet driver
//

pub const BE_TX_RING_MIN: c_int = 64;
pub const BE_RX_RING_MIN: c_int = 64;
pub const BE_TX_RING_MAX: c_int = 1024;
pub const BE_RX_RING_MAX: c_int = 2048;
pub const PKT_BUF_SZ: c_int = 1538;
// Driver's parameters
pub const RAVB_ALIGN: c_int = 128;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ravb_reg {
// AVB-DMAC registers
    CCC	= 0x0000,
    DBAT	= 0x0004,
    DLR	= 0x0008,
    CSR	= 0x000C,
    CDAR0	= 0x0010,
    CDAR1	= 0x0014,
    CDAR2	= 0x0018,
    CDAR3	= 0x001C,
    CDAR4	= 0x0020,
    CDAR5	= 0x0024,
    CDAR6	= 0x0028,
    CDAR7	= 0x002C,
    CDAR8	= 0x0030,
    CDAR9	= 0x0034,
    CDAR10	= 0x0038,
    CDAR11	= 0x003C,
    CDAR12	= 0x0040,
    CDAR13	= 0x0044,
    CDAR14	= 0x0048,
    CDAR15	= 0x004C,
    CDAR16	= 0x0050,
    CDAR17	= 0x0054,
    CDAR18	= 0x0058,
    CDAR19	= 0x005C,
    CDAR20	= 0x0060,
    CDAR21	= 0x0064,
    ESR	= 0x0088,
    APSR	= 0x008C,	/* R-Car Gen3 only */
    RCR	= 0x0090,
    RQC0	= 0x0094,
    RQC1	= 0x0098,
    RQC2	= 0x009C,
    RQC3	= 0x00A0,
    RQC4	= 0x00A4,
    RPC	= 0x00B0,
    RTC	= 0x00B4,	/* R-Car Gen3 and RZ/G2L only */
    UFCW	= 0x00BC,
    UFCS	= 0x00C0,
    UFCV0	= 0x00C4,
    UFCV1	= 0x00C8,
    UFCV2	= 0x00CC,
    UFCV3	= 0x00D0,
    UFCV4	= 0x00D4,
    UFCD0	= 0x00E0,
    UFCD1	= 0x00E4,
    UFCD2	= 0x00E8,
    UFCD3	= 0x00EC,
    UFCD4	= 0x00F0,
    SFO	= 0x00FC,
    SFP0	= 0x0100,
    SFP1	= 0x0104,
    SFP2	= 0x0108,
    SFP3	= 0x010C,
    SFP4	= 0x0110,
    SFP5	= 0x0114,
    SFP6	= 0x0118,
    SFP7	= 0x011C,
    SFP8	= 0x0120,
    SFP9	= 0x0124,
    SFP10	= 0x0128,
    SFP11	= 0x012C,
    SFP12	= 0x0130,
    SFP13	= 0x0134,
    SFP14	= 0x0138,
    SFP15	= 0x013C,
    SFP16	= 0x0140,
    SFP17	= 0x0144,
    SFP18	= 0x0148,
    SFP19	= 0x014C,
    SFP20	= 0x0150,
    SFP21	= 0x0154,
    SFP22	= 0x0158,
    SFP23	= 0x015C,
    SFP24	= 0x0160,
    SFP25	= 0x0164,
    SFP26	= 0x0168,
    SFP27	= 0x016C,
    SFP28	= 0x0170,
    SFP29	= 0x0174,
    SFP30	= 0x0178,
    SFP31	= 0x017C,
    SFM0	= 0x01C0,
    SFM1	= 0x01C4,
    TGC	= 0x0300,
    TCCR	= 0x0304,
    TSR	= 0x0308,
    TFA0	= 0x0310,
    TFA1	= 0x0314,
    TFA2	= 0x0318,
    CIVR0	= 0x0320,
    CIVR1	= 0x0324,
    CDVR0	= 0x0328,
    CDVR1	= 0x032C,
    CUL0	= 0x0330,
    CUL1	= 0x0334,
    CLL0	= 0x0338,
    CLL1	= 0x033C,
    DIC	= 0x0350,
    DIS	= 0x0354,
    EIC	= 0x0358,
    EIS	= 0x035C,
    RIC0	= 0x0360,
    RIS0	= 0x0364,
    RIC1	= 0x0368,
    RIS1	= 0x036C,
    RIC2	= 0x0370,
    RIS2	= 0x0374,
    TIC	= 0x0378,
    TIS	= 0x037C,
    ISS	= 0x0380,
    CIE	= 0x0384,	/* R-Car Gen3 only */
    GCCR	= 0x0390,
    GMTT	= 0x0394,
    GPTC	= 0x0398,
    GTI	= 0x039C,
    GTO0	= 0x03A0,
    GTO1	= 0x03A4,
    GTO2	= 0x03A8,
    GIC	= 0x03AC,
    GIS	= 0x03B0,
    GCPT	= 0x03B4,	/* Documented for R-Car Gen3 only */
    GCT0	= 0x03B8,
    GCT1	= 0x03BC,
    GCT2	= 0x03C0,
    GIE	= 0x03CC,	/* R-Car Gen3 only */
    GID	= 0x03D0,	/* R-Car Gen3 only */
    DIL	= 0x0440,	/* R-Car Gen3 only */
    RIE0	= 0x0460,	/* R-Car Gen3 only */
    RID0	= 0x0464,	/* R-Car Gen3 only */
    RIE2	= 0x0470,	/* R-Car Gen3 only */
    RID2	= 0x0474,	/* R-Car Gen3 only */
    TIE	= 0x0478,	/* R-Car Gen3 only */
    TID	= 0x047c,	/* R-Car Gen3 only */

// E-MAC registers
    ECMR	= 0x0500,
    RFLR	= 0x0508,
    ECSR	= 0x0510,
    ECSIPR	= 0x0518,
    PIR	= 0x0520,
    PSR	= 0x0528,
    PIPR	= 0x052c,
    CXR31	= 0x0530,	/* RZ/G2L only */
    CXR35	= 0x0540,	/* RZ/G2L only */
    MPR	= 0x0558,
    PFTCR	= 0x055c,
    PFRCR	= 0x0560,
    GECMR	= 0x05b0,
    MAHR	= 0x05c0,
    MALR	= 0x05c8,
    TROCR	= 0x0700,	/* R-Car Gen3 and RZ/G2L only */
    CXR41	= 0x0708,	/* RZ/G2L only */
    CXR42	= 0x0710,	/* RZ/G2L only */
    CEFCR	= 0x0740,
    FRECR	= 0x0748,
    TSFRCR	= 0x0750,
    TLFRCR	= 0x0758,
    RFCR	= 0x0760,
    MAFCR	= 0x0778,

// TOE registers (RZ/G2L only)
    CSR0    = 0x0800,
    CSR1    = 0x0804,
    CSR2    = 0x0808,
}

// Register bits of the Ethernet AVB
// CCC
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum CCC_BIT {
    CCC_OPC		= 0x00000003,
    CCC_OPC_RESET	= 0x00000000,
    CCC_OPC_CONFIG	= 0x00000001,
    CCC_OPC_OPERATION = 0x00000002,
    CCC_GAC		= 0x00000080,
    CCC_DTSR	= 0x00000100,
    CCC_CSEL	= 0x00030000,
    CCC_CSEL_HPB	= 0x00010000,
    CCC_CSEL_ETH_TX	= 0x00020000,
    CCC_CSEL_GMII_REF = 0x00030000,
    CCC_LBME	= 0x01000000,
}

// CSR
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum CSR_BIT {
    CSR_OPS		= 0x0000000F,
    CSR_OPS_RESET	= 0x00000001,
    CSR_OPS_CONFIG	= 0x00000002,
    CSR_OPS_OPERATION = 0x00000004,
    CSR_OPS_STANDBY	= 0x00000008,	/* Documented for R-Car Gen3 only */
    CSR_DTS		= 0x00000100,
    CSR_TPO0	= 0x00010000,
    CSR_TPO1	= 0x00020000,
    CSR_TPO2	= 0x00040000,
    CSR_TPO3	= 0x00080000,
    CSR_RPO		= 0x00100000,
}

// ESR
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ESR_BIT {
    ESR_EQN		= 0x0000001F,
    ESR_ET		= 0x00000F00,
    ESR_EIL		= 0x00001000,
}

// APSR (R-Car Gen3 only)
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum APSR_BIT {
    APSR_MEMS	= 0x00000002,	/* Undocumented */
    APSR_CMSW	= 0x00000010,
    APSR_RDM	= 0x00002000,
    APSR_TDM	= 0x00004000,
    APSR_MIISELECT	= 0x01000000,	/* R-Car V4M only */
}

// RCR
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum RCR_BIT {
    RCR_EFFS	= 0x00000001,
    RCR_ENCF	= 0x00000002,
    RCR_ESF		= 0x0000000C,
    RCR_ETS0	= 0x00000010,
    RCR_ETS2	= 0x00000020,
    RCR_RFCL	= 0x1FFF0000,
}

// RQC0/1/2/3/4
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum RQC_BIT {
    RQC_RSM0	= 0x00000003,
    RQC_UFCC0	= 0x00000030,
    RQC_RSM1	= 0x00000300,
    RQC_UFCC1	= 0x00003000,
    RQC_RSM2	= 0x00030000,
    RQC_UFCC2	= 0x00300000,
    RQC_RSM3	= 0x03000000,
    RQC_UFCC3	= 0x30000000,
}

// RPC
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum RPC_BIT {
    RPC_PCNT	= 0x00000700,
    RPC_DCNT	= 0x00FF0000,
}

// UFCW
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum UFCW_BIT {
    UFCW_WL0	= 0x0000003F,
    UFCW_WL1	= 0x00003F00,
    UFCW_WL2	= 0x003F0000,
    UFCW_WL3	= 0x3F000000,
}

// UFCS
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum UFCS_BIT {
    UFCS_SL0	= 0x0000003F,
    UFCS_SL1	= 0x00003F00,
    UFCS_SL2	= 0x003F0000,
    UFCS_SL3	= 0x3F000000,
}

// UFCV0/1/2/3/4
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum UFCV_BIT {
    UFCV_CV0	= 0x0000003F,
    UFCV_CV1	= 0x00003F00,
    UFCV_CV2	= 0x003F0000,
    UFCV_CV3	= 0x3F000000,
}

// UFCD0/1/2/3/4
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum UFCD_BIT {
    UFCD_DV0	= 0x0000003F,
    UFCD_DV1	= 0x00003F00,
    UFCD_DV2	= 0x003F0000,
    UFCD_DV3	= 0x3F000000,
}

// SFO
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum SFO_BIT {
    SFO_FBP		= 0x0000003F,
}

// RTC
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum RTC_BIT {
    RTC_MFL0	= 0x00000FFF,
    RTC_MFL1	= 0x0FFF0000,
}

// TGC
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum TGC_BIT {
    TGC_TSM0	= 0x00000001,
    TGC_TSM1	= 0x00000002,
    TGC_TSM2	= 0x00000004,
    TGC_TSM3	= 0x00000008,
    TGC_TQP		= 0x00000030,
    TGC_TQP_NONAVB	= 0x00000000,
    TGC_TQP_AVBMODE1 = 0x00000010,
    TGC_TQP_AVBMODE2 = 0x00000030,
    TGC_TBD0	= 0x00000300,
    TGC_TBD1	= 0x00003000,
    TGC_TBD2	= 0x00030000,
    TGC_TBD3	= 0x00300000,
}

// TCCR
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum TCCR_BIT {
    TCCR_TSRQ0	= 0x00000001,
    TCCR_TSRQ1	= 0x00000002,
    TCCR_TSRQ2	= 0x00000004,
    TCCR_TSRQ3	= 0x00000008,
    TCCR_TFEN	= 0x00000100,
    TCCR_TFR	= 0x00000200,
}

// TSR
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum TSR_BIT {
    TSR_CCS0	= 0x00000003,
    TSR_CCS1	= 0x0000000C,
    TSR_TFFL	= 0x00000700,
}

// TFA2
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum TFA2_BIT {
    TFA2_TSV	= 0x0000FFFF,
    TFA2_TST	= 0x03FF0000,
}

// DIC
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum DIC_BIT {
    DIC_DPE1	= 0x00000002,
    DIC_DPE2	= 0x00000004,
    DIC_DPE3	= 0x00000008,
    DIC_DPE4	= 0x00000010,
    DIC_DPE5	= 0x00000020,
    DIC_DPE6	= 0x00000040,
    DIC_DPE7	= 0x00000080,
    DIC_DPE8	= 0x00000100,
    DIC_DPE9	= 0x00000200,
    DIC_DPE10	= 0x00000400,
    DIC_DPE11	= 0x00000800,
    DIC_DPE12	= 0x00001000,
    DIC_DPE13	= 0x00002000,
    DIC_DPE14	= 0x00004000,
    DIC_DPE15	= 0x00008000,
}

// DIS
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum DIS_BIT {
    DIS_DPF1	= 0x00000002,
    DIS_DPF2	= 0x00000004,
    DIS_DPF3	= 0x00000008,
    DIS_DPF4	= 0x00000010,
    DIS_DPF5	= 0x00000020,
    DIS_DPF6	= 0x00000040,
    DIS_DPF7	= 0x00000080,
    DIS_DPF8	= 0x00000100,
    DIS_DPF9	= 0x00000200,
    DIS_DPF10	= 0x00000400,
    DIS_DPF11	= 0x00000800,
    DIS_DPF12	= 0x00001000,
    DIS_DPF13	= 0x00002000,
    DIS_DPF14	= 0x00004000,
    DIS_DPF15	= 0x00008000,
}

// EIC
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum EIC_BIT {
    EIC_MREE	= 0x00000001,
    EIC_MTEE	= 0x00000002,
    EIC_QEE		= 0x00000004,
    EIC_SEE		= 0x00000008,
    EIC_CLLE0	= 0x00000010,
    EIC_CLLE1	= 0x00000020,
    EIC_CULE0	= 0x00000040,
    EIC_CULE1	= 0x00000080,
    EIC_TFFE	= 0x00000100,
}

// EIS
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum EIS_BIT {
    EIS_MREF	= 0x00000001,
    EIS_MTEF	= 0x00000002,
    EIS_QEF		= 0x00000004,
    EIS_SEF		= 0x00000008,
    EIS_CLLF0	= 0x00000010,
    EIS_CLLF1	= 0x00000020,
    EIS_CULF0	= 0x00000040,
    EIS_CULF1	= 0x00000080,
    EIS_TFFF	= 0x00000100,
    EIS_QFS		= 0x00010000,
    EIS_RESERVED	= (GENMASK(31, 17) | GENMASK(15, 11)),
}

// RIC0
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum RIC0_BIT {
    RIC0_FRE0	= 0x00000001,
    RIC0_FRE1	= 0x00000002,
    RIC0_FRE2	= 0x00000004,
    RIC0_FRE3	= 0x00000008,
    RIC0_FRE4	= 0x00000010,
    RIC0_FRE5	= 0x00000020,
    RIC0_FRE6	= 0x00000040,
    RIC0_FRE7	= 0x00000080,
    RIC0_FRE8	= 0x00000100,
    RIC0_FRE9	= 0x00000200,
    RIC0_FRE10	= 0x00000400,
    RIC0_FRE11	= 0x00000800,
    RIC0_FRE12	= 0x00001000,
    RIC0_FRE13	= 0x00002000,
    RIC0_FRE14	= 0x00004000,
    RIC0_FRE15	= 0x00008000,
    RIC0_FRE16	= 0x00010000,
    RIC0_FRE17	= 0x00020000,
}

// RIC0
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum RIS0_BIT {
    RIS0_FRF0	= 0x00000001,
    RIS0_FRF1	= 0x00000002,
    RIS0_FRF2	= 0x00000004,
    RIS0_FRF3	= 0x00000008,
    RIS0_FRF4	= 0x00000010,
    RIS0_FRF5	= 0x00000020,
    RIS0_FRF6	= 0x00000040,
    RIS0_FRF7	= 0x00000080,
    RIS0_FRF8	= 0x00000100,
    RIS0_FRF9	= 0x00000200,
    RIS0_FRF10	= 0x00000400,
    RIS0_FRF11	= 0x00000800,
    RIS0_FRF12	= 0x00001000,
    RIS0_FRF13	= 0x00002000,
    RIS0_FRF14	= 0x00004000,
    RIS0_FRF15	= 0x00008000,
    RIS0_FRF16	= 0x00010000,
    RIS0_FRF17	= 0x00020000,
    RIS0_RESERVED	= GENMASK(31, 18),
}

// RIC1
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum RIC1_BIT {
    RIC1_RFWE	= 0x80000000,
}

// RIS1
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum RIS1_BIT {
    RIS1_RFWF	= 0x80000000,
}

// RIC2
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum RIC2_BIT {
    RIC2_QFE0	= 0x00000001,
    RIC2_QFE1	= 0x00000002,
    RIC2_QFE2	= 0x00000004,
    RIC2_QFE3	= 0x00000008,
    RIC2_QFE4	= 0x00000010,
    RIC2_QFE5	= 0x00000020,
    RIC2_QFE6	= 0x00000040,
    RIC2_QFE7	= 0x00000080,
    RIC2_QFE8	= 0x00000100,
    RIC2_QFE9	= 0x00000200,
    RIC2_QFE10	= 0x00000400,
    RIC2_QFE11	= 0x00000800,
    RIC2_QFE12	= 0x00001000,
    RIC2_QFE13	= 0x00002000,
    RIC2_QFE14	= 0x00004000,
    RIC2_QFE15	= 0x00008000,
    RIC2_QFE16	= 0x00010000,
    RIC2_QFE17	= 0x00020000,
    RIC2_RFFE	= 0x80000000,
}

// RIS2
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum RIS2_BIT {
    RIS2_QFF0	= 0x00000001,
    RIS2_QFF1	= 0x00000002,
    RIS2_QFF2	= 0x00000004,
    RIS2_QFF3	= 0x00000008,
    RIS2_QFF4	= 0x00000010,
    RIS2_QFF5	= 0x00000020,
    RIS2_QFF6	= 0x00000040,
    RIS2_QFF7	= 0x00000080,
    RIS2_QFF8	= 0x00000100,
    RIS2_QFF9	= 0x00000200,
    RIS2_QFF10	= 0x00000400,
    RIS2_QFF11	= 0x00000800,
    RIS2_QFF12	= 0x00001000,
    RIS2_QFF13	= 0x00002000,
    RIS2_QFF14	= 0x00004000,
    RIS2_QFF15	= 0x00008000,
    RIS2_QFF16	= 0x00010000,
    RIS2_QFF17	= 0x00020000,
    RIS2_RFFF	= 0x80000000,
    RIS2_RESERVED	= GENMASK(30, 18),
}

// TIC
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum TIC_BIT {
    TIC_FTE0	= 0x00000001,	/* Documented for R-Car Gen3 only */
    TIC_FTE1	= 0x00000002,	/* Documented for R-Car Gen3 only */
    TIC_TFUE	= 0x00000100,
    TIC_TFWE	= 0x00000200,
}

// TIS
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum TIS_BIT {
    TIS_FTF0	= 0x00000001,	/* Documented for R-Car Gen3 only */
    TIS_FTF1	= 0x00000002,	/* Documented for R-Car Gen3 only */
    TIS_TFUF	= 0x00000100,
    TIS_TFWF	= 0x00000200,
    TIS_RESERVED	= (GENMASK(31, 20) | GENMASK(15, 12) | GENMASK(7, 4))
}

// ISS
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ISS_BIT {
    ISS_FRS		= 0x00000001,	/* Documented for R-Car Gen3 only */
    ISS_FTS		= 0x00000004,	/* Documented for R-Car Gen3 only */
    ISS_ES		= 0x00000040,
    ISS_MS		= 0x00000080,
    ISS_TFUS	= 0x00000100,
    ISS_TFWS	= 0x00000200,
    ISS_RFWS	= 0x00001000,
    ISS_CGIS	= 0x00002000,
    ISS_DPS1	= 0x00020000,
    ISS_DPS2	= 0x00040000,
    ISS_DPS3	= 0x00080000,
    ISS_DPS4	= 0x00100000,
    ISS_DPS5	= 0x00200000,
    ISS_DPS6	= 0x00400000,
    ISS_DPS7	= 0x00800000,
    ISS_DPS8	= 0x01000000,
    ISS_DPS9	= 0x02000000,
    ISS_DPS10	= 0x04000000,
    ISS_DPS11	= 0x08000000,
    ISS_DPS12	= 0x10000000,
    ISS_DPS13	= 0x20000000,
    ISS_DPS14	= 0x40000000,
    ISS_DPS15	= 0x80000000,
}

// CIE (R-Car Gen3 only)
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum CIE_BIT {
    CIE_CRIE	= 0x00000001,
    CIE_CTIE	= 0x00000100,
    CIE_RQFM	= 0x00010000,
    CIE_CL0M	= 0x00020000,
    CIE_RFWL	= 0x00040000,
    CIE_RFFL	= 0x00080000,
}

// GCCR
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum GCCR_BIT {
    GCCR_TCR	= 0x00000003,
    GCCR_TCR_NOREQ	= 0x00000000, /* No request */
    GCCR_TCR_RESET	= 0x00000001, /* gPTP/AVTP presentation timer reset */
    GCCR_TCR_CAPTURE = 0x00000003, /* Capture value set in GCCR.TCSS */
    GCCR_LTO	= 0x00000004,
    GCCR_LTI	= 0x00000008,
    GCCR_LPTC	= 0x00000010,
    GCCR_LMTT	= 0x00000020,
    GCCR_TCSS	= 0x00000300,
    GCCR_TCSS_GPTP	= 0x00000000,	/* gPTP timer value */
    GCCR_TCSS_ADJGPTP = 0x00000100, /* Adjusted gPTP timer value */
    GCCR_TCSS_AVTP	= 0x00000200,	/* AVTP presentation time value */
}

// GTI
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum GTI_BIT {
    GTI_TIV		= 0x0FFFFFFF,
}

pub const GTI_TIV_MIN: c_uint = 0x20;
// GIC
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum GIC_BIT {
    GIC_PTCE	= 0x00000001,	/* Documented for R-Car Gen3 only */
    GIC_PTME	= 0x00000004,
}

// GIS
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum GIS_BIT {
    GIS_PTCF	= 0x00000001,	/* Documented for R-Car Gen3 only */
    GIS_PTMF	= 0x00000004,
    GIS_RESERVED	= GENMASK(15, 10),
}

// GIE (R-Car Gen3 only)
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum GIE_BIT {
    GIE_PTCS	= 0x00000001,
    GIE_PTOS	= 0x00000002,
    GIE_PTMS0	= 0x00000004,
    GIE_PTMS1	= 0x00000008,
    GIE_PTMS2	= 0x00000010,
    GIE_PTMS3	= 0x00000020,
    GIE_PTMS4	= 0x00000040,
    GIE_PTMS5	= 0x00000080,
    GIE_PTMS6	= 0x00000100,
    GIE_PTMS7	= 0x00000200,
    GIE_ATCS0	= 0x00010000,
    GIE_ATCS1	= 0x00020000,
    GIE_ATCS2	= 0x00040000,
    GIE_ATCS3	= 0x00080000,
    GIE_ATCS4	= 0x00100000,
    GIE_ATCS5	= 0x00200000,
    GIE_ATCS6	= 0x00400000,
    GIE_ATCS7	= 0x00800000,
    GIE_ATCS8	= 0x01000000,
    GIE_ATCS9	= 0x02000000,
    GIE_ATCS10	= 0x04000000,
    GIE_ATCS11	= 0x08000000,
    GIE_ATCS12	= 0x10000000,
    GIE_ATCS13	= 0x20000000,
    GIE_ATCS14	= 0x40000000,
    GIE_ATCS15	= 0x80000000,
}

// GID (R-Car Gen3 only)
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum GID_BIT {
    GID_PTCD	= 0x00000001,
    GID_PTOD	= 0x00000002,
    GID_PTMD0	= 0x00000004,
    GID_PTMD1	= 0x00000008,
    GID_PTMD2	= 0x00000010,
    GID_PTMD3	= 0x00000020,
    GID_PTMD4	= 0x00000040,
    GID_PTMD5	= 0x00000080,
    GID_PTMD6	= 0x00000100,
    GID_PTMD7	= 0x00000200,
    GID_ATCD0	= 0x00010000,
    GID_ATCD1	= 0x00020000,
    GID_ATCD2	= 0x00040000,
    GID_ATCD3	= 0x00080000,
    GID_ATCD4	= 0x00100000,
    GID_ATCD5	= 0x00200000,
    GID_ATCD6	= 0x00400000,
    GID_ATCD7	= 0x00800000,
    GID_ATCD8	= 0x01000000,
    GID_ATCD9	= 0x02000000,
    GID_ATCD10	= 0x04000000,
    GID_ATCD11	= 0x08000000,
    GID_ATCD12	= 0x10000000,
    GID_ATCD13	= 0x20000000,
    GID_ATCD14	= 0x40000000,
    GID_ATCD15	= 0x80000000,
}

// RIE0 (R-Car Gen3 only)
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum RIE0_BIT {
    RIE0_FRS0	= 0x00000001,
    RIE0_FRS1	= 0x00000002,
    RIE0_FRS2	= 0x00000004,
    RIE0_FRS3	= 0x00000008,
    RIE0_FRS4	= 0x00000010,
    RIE0_FRS5	= 0x00000020,
    RIE0_FRS6	= 0x00000040,
    RIE0_FRS7	= 0x00000080,
    RIE0_FRS8	= 0x00000100,
    RIE0_FRS9	= 0x00000200,
    RIE0_FRS10	= 0x00000400,
    RIE0_FRS11	= 0x00000800,
    RIE0_FRS12	= 0x00001000,
    RIE0_FRS13	= 0x00002000,
    RIE0_FRS14	= 0x00004000,
    RIE0_FRS15	= 0x00008000,
    RIE0_FRS16	= 0x00010000,
    RIE0_FRS17	= 0x00020000,
}

// RID0 (R-Car Gen3 only)
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum RID0_BIT {
    RID0_FRD0	= 0x00000001,
    RID0_FRD1	= 0x00000002,
    RID0_FRD2	= 0x00000004,
    RID0_FRD3	= 0x00000008,
    RID0_FRD4	= 0x00000010,
    RID0_FRD5	= 0x00000020,
    RID0_FRD6	= 0x00000040,
    RID0_FRD7	= 0x00000080,
    RID0_FRD8	= 0x00000100,
    RID0_FRD9	= 0x00000200,
    RID0_FRD10	= 0x00000400,
    RID0_FRD11	= 0x00000800,
    RID0_FRD12	= 0x00001000,
    RID0_FRD13	= 0x00002000,
    RID0_FRD14	= 0x00004000,
    RID0_FRD15	= 0x00008000,
    RID0_FRD16	= 0x00010000,
    RID0_FRD17	= 0x00020000,
}

// RIE2 (R-Car Gen3 only)
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum RIE2_BIT {
    RIE2_QFS0	= 0x00000001,
    RIE2_QFS1	= 0x00000002,
    RIE2_QFS2	= 0x00000004,
    RIE2_QFS3	= 0x00000008,
    RIE2_QFS4	= 0x00000010,
    RIE2_QFS5	= 0x00000020,
    RIE2_QFS6	= 0x00000040,
    RIE2_QFS7	= 0x00000080,
    RIE2_QFS8	= 0x00000100,
    RIE2_QFS9	= 0x00000200,
    RIE2_QFS10	= 0x00000400,
    RIE2_QFS11	= 0x00000800,
    RIE2_QFS12	= 0x00001000,
    RIE2_QFS13	= 0x00002000,
    RIE2_QFS14	= 0x00004000,
    RIE2_QFS15	= 0x00008000,
    RIE2_QFS16	= 0x00010000,
    RIE2_QFS17	= 0x00020000,
    RIE2_RFFS	= 0x80000000,
}

// RID2 (R-Car Gen3 only)
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum RID2_BIT {
    RID2_QFD0	= 0x00000001,
    RID2_QFD1	= 0x00000002,
    RID2_QFD2	= 0x00000004,
    RID2_QFD3	= 0x00000008,
    RID2_QFD4	= 0x00000010,
    RID2_QFD5	= 0x00000020,
    RID2_QFD6	= 0x00000040,
    RID2_QFD7	= 0x00000080,
    RID2_QFD8	= 0x00000100,
    RID2_QFD9	= 0x00000200,
    RID2_QFD10	= 0x00000400,
    RID2_QFD11	= 0x00000800,
    RID2_QFD12	= 0x00001000,
    RID2_QFD13	= 0x00002000,
    RID2_QFD14	= 0x00004000,
    RID2_QFD15	= 0x00008000,
    RID2_QFD16	= 0x00010000,
    RID2_QFD17	= 0x00020000,
    RID2_RFFD	= 0x80000000,
}

// TIE (R-Car Gen3 only)
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum TIE_BIT {
    TIE_FTS0	= 0x00000001,
    TIE_FTS1	= 0x00000002,
    TIE_FTS2	= 0x00000004,
    TIE_FTS3	= 0x00000008,
    TIE_TFUS	= 0x00000100,
    TIE_TFWS	= 0x00000200,
    TIE_MFUS	= 0x00000400,
    TIE_MFWS	= 0x00000800,
    TIE_TDPS0	= 0x00010000,
    TIE_TDPS1	= 0x00020000,
    TIE_TDPS2	= 0x00040000,
    TIE_TDPS3	= 0x00080000,
}

// TID (R-Car Gen3 only)
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum TID_BIT {
    TID_FTD0	= 0x00000001,
    TID_FTD1	= 0x00000002,
    TID_FTD2	= 0x00000004,
    TID_FTD3	= 0x00000008,
    TID_TFUD	= 0x00000100,
    TID_TFWD	= 0x00000200,
    TID_MFUD	= 0x00000400,
    TID_MFWD	= 0x00000800,
    TID_TDPD0	= 0x00010000,
    TID_TDPD1	= 0x00020000,
    TID_TDPD2	= 0x00040000,
    TID_TDPD3	= 0x00080000,
}

// ECMR
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ECMR_BIT {
    ECMR_PRM	= 0x00000001,
    ECMR_DM		= 0x00000002,
    ECMR_TE		= 0x00000020,
    ECMR_RE		= 0x00000040,
    ECMR_MPDE	= 0x00000200,
    ECMR_TXF	= 0x00010000,	/* Documented for R-Car Gen3 only */
    ECMR_RXF	= 0x00020000,
    ECMR_PFR	= 0x00040000,
    ECMR_ZPF	= 0x00080000,	/* Documented for R-Car Gen3 and RZ/G2L */
    ECMR_RZPF	= 0x00100000,
    ECMR_DPAD	= 0x00200000,
    ECMR_RCSC	= 0x00800000,
    ECMR_RCPT	= 0x02000000,	/* Documented for RZ/G2L only */
    ECMR_TRCCM	= 0x04000000,
}

// ECSR
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ECSR_BIT {
    ECSR_ICD	= 0x00000001,
    ECSR_MPD	= 0x00000002,
    ECSR_LCHNG	= 0x00000004,
    ECSR_PHYI	= 0x00000008,
    ECSR_PFRI	= 0x00000010,	/* Documented for R-Car Gen3 and RZ/G2L */
}

// ECSIPR
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ECSIPR_BIT {
    ECSIPR_ICDIP	= 0x00000001,
    ECSIPR_MPDIP	= 0x00000002,
    ECSIPR_LCHNGIP	= 0x00000004,
}

// PIR
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum PIR_BIT {
    PIR_MDC		= 0x00000001,
    PIR_MMD		= 0x00000002,
    PIR_MDO		= 0x00000004,
    PIR_MDI		= 0x00000008,
}

// PSR
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum PSR_BIT {
    PSR_LMON	= 0x00000001,
}

// PIPR
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum PIPR_BIT {
    PIPR_PHYIP	= 0x00000001,
}

// MPR
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MPR_BIT {
    MPR_MP		= 0x0000ffff,
}

// GECMR
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum GECMR_BIT {
    GECMR_SPEED		= 0x00000001,
    GECMR_SPEED_100		= 0x00000000,
    GECMR_SPEED_1000	= 0x00000001,
    GBETH_GECMR_SPEED	= 0x00000030,
    GBETH_GECMR_SPEED_10	= 0x00000000,
    GBETH_GECMR_SPEED_100	= 0x00000010,
    GBETH_GECMR_SPEED_1000	= 0x00000020,
}

// The Ethernet AVB descriptor definitions.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ravb_desc {
    pub /: *mut *mut __le16 ds; / Descriptor size,
    pub /: *mut *mut u8 cc; / Content control MSBs (reserved),
    pub /: *mut *mut u8 die_dt; / Descriptor interrupt enable and type,
    pub /: *mut *mut __le32 dptr; / Descriptor pointer,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum DIE_DT {
// Frame data
    DT_FMID		= 0x40,
    DT_FSTART	= 0x50,
    DT_FEND		= 0x60,
    DT_FSINGLE	= 0x70,
// Chain control
    DT_LINK		= 0x80,
    DT_LINKFIX	= 0x90,
    DT_EOS		= 0xa0,
// HW/SW arbitration
    DT_FEMPTY	= 0xc0,
    DT_FEMPTY_IS	= 0xd0,
    DT_FEMPTY_IC	= 0xe0,
    DT_FEMPTY_ND	= 0xf0,
    DT_LEMPTY	= 0x20,
    DT_EEMPTY	= 0x30,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ravb_rx_desc {
    pub /: *mut *mut __le16 ds_cc; / Descriptor size and content control LSBs,
    pub /: *mut *mut u8 msc; / MAC status code,
    pub /: *mut *mut u8 die_dt; / Descriptor interrupt enable and type,
    pub /: *mut *mut __le32 dptr; / Descpriptor pointer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ravb_ex_rx_desc {
    pub /: *mut *mut __le16 ds_cc; / Descriptor size and content control lower bits,
    pub /: *mut *mut u8 msc; / MAC status code,
    pub /: *mut *mut u8 die_dt; / Descriptor interrupt enable and type,
    pub /: *mut *mut __le32 dptr; / Descpriptor pointer,
    pub /: *mut *mut __le32 ts_n; / Timestampe nsec,
    pub /: *mut *mut __le32 ts_sl; / Timestamp low,
    pub /: *mut *mut __le16 ts_sh; / Timestamp high,
    pub /: *mut *mut __le16 res; / Reserved bits,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum RX_DS_CC_BIT {
    RX_DS		= 0x0fff, /* Data size */
    RX_TR		= 0x1000, /* Truncation indication */
    RX_EI		= 0x2000, /* Error indication */
    RX_PS		= 0xc000, /* Padding selection */
}

// E-MAC status code
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MSC_BIT {
    MSC_CRC		= 0x01, /* Frame CRC error */
    MSC_RFE		= 0x02, /* Frame reception error (flagged by PHY) */
    MSC_RTSF	= 0x04, /* Frame length error (frame too short) */
    MSC_RTLF	= 0x08, /* Frame length error (frame too long) */
    MSC_FRE		= 0x10, /* Fraction error (not a multiple of 8 bits) */
    MSC_CRL		= 0x20, /* Carrier lost */
    MSC_CEEF	= 0x40, /* Carrier extension error */
    MSC_MC		= 0x80, /* Multicast frame reception */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ravb_tx_desc {
    pub /: *mut *mut __le16 ds_tagl; / Descriptor size and frame tag LSBs,
    pub /: *mut *mut u8 tagh_tsr; / Frame tag MSBs and timestamp storage request bit,
    pub /: *mut *mut u8 die_dt; / Descriptor interrupt enable and type,
    pub /: *mut *mut __le32 dptr; / Descpriptor pointer,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum TX_DS_TAGL_BIT {
    TX_DS		= 0x0fff, /* Data size */
    TX_TAGL		= 0xf000, /* Frame tag LSBs */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum TX_TAGH_TSR_BIT {
    TX_TAGH		= 0x3f, /* Frame tag MSBs */
    TX_TSR		= 0x40, /* Timestamp storage request */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum RAVB_QUEUE {
    RAVB_BE = 0,	/* Best Effort Queue */
    RAVB_NC,	/* Network Control Queue */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum CXR31_BIT {
    CXR31_SEL_LINK0	= 0x00000001,
    CXR31_SEL_LINK1	= 0x00000008,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum CXR35_BIT {
    CXR35_SEL_XMII		= 0x00000003,
    CXR35_SEL_XMII_RGMII	= 0x00000000,
    CXR35_SEL_XMII_MII	= 0x00000002,
    CXR35_HALFCYC_CLKSW	= 0xffff0000,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum CSR0_BIT {
    CSR0_TPE	= 0x00000010,
    CSR0_RPE	= 0x00000020,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum CSR1_BIT {
    CSR1_TIP4	= 0x00000001,
    CSR1_TTCP4	= 0x00000010,
    CSR1_TUDP4	= 0x00000020,
    CSR1_TICMP4	= 0x00000040,
    CSR1_TTCP6	= 0x00100000,
    CSR1_TUDP6	= 0x00200000,
    CSR1_TICMP6	= 0x00400000,
    CSR1_THOP	= 0x01000000,
    CSR1_TROUT	= 0x02000000,
    CSR1_TAHD	= 0x04000000,
    CSR1_TDHD	= 0x08000000,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum CSR2_BIT {
    CSR2_RIP4	= 0x00000001,
    CSR2_RTCP4	= 0x00000010,
    CSR2_RUDP4	= 0x00000020,
    CSR2_RICMP4	= 0x00000040,
    CSR2_RTCP6	= 0x00100000,
    CSR2_RUDP6	= 0x00200000,
    CSR2_RICMP6	= 0x00400000,
    CSR2_RHOP	= 0x01000000,
    CSR2_RROUT	= 0x02000000,
    CSR2_RAHD	= 0x04000000,
    CSR2_RDHD	= 0x08000000,
}

pub const RX_QUEUE_OFFSET: c_int = 4;
pub const NUM_RX_QUEUE: c_int = 2;
pub const NUM_TX_QUEUE: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ravb_tstamp_skb {
    pub list: list_head,
    pub skb: *mut sk_buff,
    pub tag: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ravb_ptp_perout {
    pub target: u32,
    pub period: u32,
}

pub const N_EXT_TS: c_int = 1;
pub const N_PER_OUT: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ravb_ptp {
    pub clock: *mut ptp_clock,
    pub info: ptp_clock_info,
    pub phc_index: c_int,
    pub default_addend: u32,
    pub current_addend: u32,
    pub extts: [c_int; N_EXT_TS],
    pub perout: [ravb_ptp_perout; N_PER_OUT],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ravb_hw_info {
    pub q): *mut *mut *mut int (receive)(struct net_device ndev, int budget, int,
    pub ndev): *mut *mut void (set_rate)(struct net_device,
    pub features): *mut *mut *mut int (set_feature)(struct net_device ndev, netdev_features_t,
    pub ndev): *mut *mut int (dmac_init)(struct net_device,
    pub ndev): *mut *mut void (emac_init)(struct net_device,
    pub (*gstrings_stats)[ETH_GSTRING_LEN]: *const c_char,
    pub gstrings_size: usize,
    pub net_hw_features: netdev_features_t,
    pub net_features: netdev_features_t,
    pub vlan_features: netdev_features_t,
    pub stats_len: c_int,
    pub tccr_mask: u32,
    pub tx_max_frame_size: u32,
    pub rx_max_frame_size: u32,
    pub rx_buffer_size: u32,
    pub rx_desc_size: u32,
    pub dbat_entry_num: u32,
    pub 1: unsigned aligned_tx:,
    pub /: *mut *mut unsigned coalesce_irqs:1; / Needs software IRQ coalescing,
// hardware features
    pub /: *mut *mut unsigned internal_delay:1; / AVB-DMAC has internal delays,
    pub /: *mut *mut unsigned tx_counters:1; / E-MAC has TX counters,
    pub /: *mut *mut unsigned carrier_counters:1; / E-MAC has carrier counters,
    pub /: *mut *mut unsigned multi_irqs:1; / AVB-DMAC and E-MAC has multiple irqs,
    pub /: *mut *mut unsigned irq_en_dis:1; / Has separate irq enable and disable regs,
    pub /: *mut *mut unsigned err_mgmt_irqs:1; / Line1 (Err) and Line2 (Mgmt) irqs are separate,
    pub /: *mut *mut unsigned gptp:1; / AVB-DMAC has gPTP support,
    pub /: *mut *mut unsigned ccc_gac:1; / AVB-DMAC has gPTP support active in config mode,
    pub /: *mut *mut unsigned gptp_ref_clk:1; / gPTP has separate reference clock,
    pub /: *mut *mut unsigned nc_queues:1; / AVB-DMAC has RX and TX NC queues,
    pub /: *mut *mut unsigned magic_pkt:1; / E-MAC supports magic packet detection,
    pub /: *mut *mut unsigned half_duplex:1; / E-MAC supports half duplex mode,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ravb_rx_buffer {
    pub page: *mut page,
    pub offset: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ravb_private {
    pub ndev: *mut net_device,
    pub pdev: *mut platform_device,
    pub addr: *mut void __iomem,
    pub clk: *mut clk,
    pub refclk: *mut clk,
    pub gptp_clk: *mut clk,
    pub mdiobb: mdiobb_ctrl,
    pub num_rx_ring: [u32; NUM_RX_QUEUE],
    pub num_tx_ring: [u32; NUM_TX_QUEUE],
    pub desc_bat_size: u32,
    pub desc_bat_dma: dma_addr_t,
    pub desc_bat: *mut ravb_desc,
    pub rx_desc_dma: [dma_addr_t; NUM_RX_QUEUE],
    pub tx_desc_dma: [dma_addr_t; NUM_TX_QUEUE],
    pub desc: *mut ravb_rx_desc,
    pub ex_desc: *mut ravb_ex_rx_desc,
    pub raw: *mut c_void,
    pub rx_ring: [}; NUM_RX_QUEUE],
    pub tx_ring: [*mut ravb_tx_desc; NUM_TX_QUEUE],
    pub tx_align: [*mut c_void; NUM_TX_QUEUE],
    pub rx_1st_skb: *mut sk_buff,
    pub rx_pool: [*mut page_pool; NUM_RX_QUEUE],
    pub rx_buffers: [*mut ravb_rx_buffer; NUM_RX_QUEUE],
    pub tx_skb: [*mut sk_buff; NUM_TX_QUEUE],
    pub rx_over_errors: u32,
    pub rx_fifo_errors: u32,
    pub stats: [net_device_stats; NUM_RX_QUEUE],
    pub tstamp_tx_ctrl: hwtstamp_tx_types,
    pub tstamp_rx_ctrl: hwtstamp_rx_filters,
    pub ts_skb_list: list_head,
    pub ts_skb_tag: u32,
    pub ptp: ravb_ptp,
    pub /: *mut *mut spinlock_t lock; / Register access lock,
    pub /: *mut *mut u32 cur_rx[NUM_RX_QUEUE]; / Consumer ring indices,
    pub /: *mut *mut u32 dirty_rx[NUM_RX_QUEUE]; / Producer ring indices,
    pub cur_tx: [u32; NUM_TX_QUEUE],
    pub dirty_tx: [u32; NUM_TX_QUEUE],
    pub napi: [napi_struct; NUM_RX_QUEUE],
    pub work: work_struct,
// MII transceiver section.
    pub /: *mut *mut *mut mii_bus mii_bus; / MDIO bus control,
    pub link: c_int,
    pub phy_interface: phy_interface_t,
    pub msg_enable: c_int,
    pub speed: c_int,
    pub emac_irq: c_int,
    pub err_irq: c_int,
    pub mgmt_irq: c_int,
    pub no_avb_link:1: unsigned,
    pub avb_link_active_low:1: unsigned,
    pub wol_enabled:1: unsigned,
    pub /: *mut *mut unsigned rxcidm:1; / RX Clock Internal Delay Mode,
    pub /: *mut *mut unsigned txcidm:1; / TX Clock Internal Delay Mode,
    pub /: *mut *mut *mut unsigned rgmii_override:1; / Deprecated rgmii-id behavior,
    pub /: *mut *mut unsigned int num_tx_desc; / TX descriptors per packet,
    pub duplex: c_int,
    pub info: *const ravb_hw_info,
    pub rstc: *mut reset_control,
    pub gti_tiv: u32,
}

extern "C" {
    pub fn ioread32(reg: priv->addr +) -> return;
}
extern "C" {
    pub fn ravb_wait(ndev: *mut net_device, reg: ravb_reg, mask: u32, value: u32) -> c_int;
}
extern "C" {
    pub fn ravb_ptp_interrupt(ndev: *mut net_device);
}
extern "C" {
    pub fn ravb_ptp_init(ndev: *mut net_device, pdev: *mut platform_device);
}
extern "C" {
    pub fn ravb_ptp_stop(ndev: *mut net_device);
}
