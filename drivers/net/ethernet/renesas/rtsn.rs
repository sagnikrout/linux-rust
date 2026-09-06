//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/renesas/rtsn.h
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
// Renesas Ethernet-TSN device driver
//
// Copyright (C) 2022 Renesas Electronics Corporation
// Copyright (C) 2023 Niklas Söderlund <niklas.soderlund@ragnatech.se>
//

pub const AXIBMI: c_uint = 0x0000;
pub const TSNMHD: c_uint = 0x1000;
pub const RMSO: c_uint = 0x2000;
pub const RMRO: c_uint = 0x3800;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtsn_reg {
    AXIWC		= AXIBMI + 0x0000,
    AXIRC		= AXIBMI + 0x0004,
    TDPC0		= AXIBMI + 0x0010,
    TFT		= AXIBMI + 0x0090,
    TATLS0		= AXIBMI + 0x00a0,
    TATLS1		= AXIBMI + 0x00a4,
    TATLR		= AXIBMI + 0x00a8,
    RATLS0		= AXIBMI + 0x00b0,
    RATLS1		= AXIBMI + 0x00b4,
    RATLR		= AXIBMI + 0x00b8,
    TSA0		= AXIBMI + 0x00c0,
    TSS0		= AXIBMI + 0x00c4,
    TRCR0		= AXIBMI + 0x0140,
    RIDAUAS0	= AXIBMI + 0x0180,
    RR		= AXIBMI + 0x0200,
    TATS		= AXIBMI + 0x0210,
    TATSR0		= AXIBMI + 0x0214,
    TATSR1		= AXIBMI + 0x0218,
    TATSR2		= AXIBMI + 0x021c,
    RATS		= AXIBMI + 0x0220,
    RATSR0		= AXIBMI + 0x0224,
    RATSR1		= AXIBMI + 0x0228,
    RATSR2		= AXIBMI + 0x022c,
    RIDASM0		= AXIBMI + 0x0240,
    RIDASAM0	= AXIBMI + 0x0244,
    RIDACAM0	= AXIBMI + 0x0248,
    EIS0		= AXIBMI + 0x0300,
    EIE0		= AXIBMI + 0x0304,
    EID0		= AXIBMI + 0x0308,
    EIS1		= AXIBMI + 0x0310,
    EIE1		= AXIBMI + 0x0314,
    EID1		= AXIBMI + 0x0318,
    TCEIS0		= AXIBMI + 0x0340,
    TCEIE0		= AXIBMI + 0x0344,
    TCEID0		= AXIBMI + 0x0348,
    RFSEIS0		= AXIBMI + 0x04c0,
    RFSEIE0		= AXIBMI + 0x04c4,
    RFSEID0		= AXIBMI + 0x04c8,
    RFEIS0		= AXIBMI + 0x0540,
    RFEIE0		= AXIBMI + 0x0544,
    RFEID0		= AXIBMI + 0x0548,
    RCEIS0		= AXIBMI + 0x05c0,
    RCEIE0		= AXIBMI + 0x05c4,
    RCEID0		= AXIBMI + 0x05c8,
    RIDAOIS		= AXIBMI + 0x0640,
    RIDAOIE		= AXIBMI + 0x0644,
    RIDAOID		= AXIBMI + 0x0648,
    TSFEIS		= AXIBMI + 0x06c0,
    TSFEIE		= AXIBMI + 0x06c4,
    TSFEID		= AXIBMI + 0x06c8,
    TSCEIS		= AXIBMI + 0x06d0,
    TSCEIE		= AXIBMI + 0x06d4,
    TSCEID		= AXIBMI + 0x06d8,
    DIS		= AXIBMI + 0x0b00,
    DIE		= AXIBMI + 0x0b04,
    DID		= AXIBMI + 0x0b08,
    TDIS0		= AXIBMI + 0x0b10,
    TDIE0		= AXIBMI + 0x0b14,
    TDID0		= AXIBMI + 0x0b18,
    RDIS0		= AXIBMI + 0x0b90,
    RDIE0		= AXIBMI + 0x0b94,
    RDID0		= AXIBMI + 0x0b98,
    TSDIS		= AXIBMI + 0x0c10,
    TSDIE		= AXIBMI + 0x0c14,
    TSDID		= AXIBMI + 0x0c18,
    GPOUT		= AXIBMI + 0x6000,

    OCR		= TSNMHD + 0x0000,
    OSR		= TSNMHD + 0x0004,
    SWR		= TSNMHD + 0x0008,
    SIS		= TSNMHD + 0x000c,
    GIS		= TSNMHD + 0x0010,
    GIE		= TSNMHD + 0x0014,
    GID		= TSNMHD + 0x0018,
    TIS1		= TSNMHD + 0x0020,
    TIE1		= TSNMHD + 0x0024,
    TID1		= TSNMHD + 0x0028,
    TIS2		= TSNMHD + 0x0030,
    TIE2		= TSNMHD + 0x0034,
    TID2		= TSNMHD + 0x0038,
    RIS		= TSNMHD + 0x0040,
    RIE		= TSNMHD + 0x0044,
    RID		= TSNMHD + 0x0048,
    TGC1		= TSNMHD + 0x0050,
    TGC2		= TSNMHD + 0x0054,
    TFS0		= TSNMHD + 0x0060,
    TCF0		= TSNMHD + 0x0070,
    TCR1		= TSNMHD + 0x0080,
    TCR2		= TSNMHD + 0x0084,
    TCR3		= TSNMHD + 0x0088,
    TCR4		= TSNMHD + 0x008c,
    TMS0		= TSNMHD + 0x0090,
    TSR1		= TSNMHD + 0x00b0,
    TSR2		= TSNMHD + 0x00b4,
    TSR3		= TSNMHD + 0x00b8,
    TSR4		= TSNMHD + 0x00bc,
    TSR5		= TSNMHD + 0x00c0,
    RGC		= TSNMHD + 0x00d0,
    RDFCR		= TSNMHD + 0x00d4,
    RCFCR		= TSNMHD + 0x00d8,
    REFCNCR		= TSNMHD + 0x00dc,
    RSR1		= TSNMHD + 0x00e0,
    RSR2		= TSNMHD + 0x00e4,
    RSR3		= TSNMHD + 0x00e8,
    TCIS		= TSNMHD + 0x01e0,
    TCIE		= TSNMHD + 0x01e4,
    TCID		= TSNMHD + 0x01e8,
    TPTPC		= TSNMHD + 0x01f0,
    TTML		= TSNMHD + 0x01f4,
    TTJ		= TSNMHD + 0x01f8,
    TCC		= TSNMHD + 0x0200,
    TCS		= TSNMHD + 0x0204,
    TGS		= TSNMHD + 0x020c,
    TACST0		= TSNMHD + 0x0210,
    TACST1		= TSNMHD + 0x0214,
    TACST2		= TSNMHD + 0x0218,
    TALIT0		= TSNMHD + 0x0220,
    TALIT1		= TSNMHD + 0x0224,
    TALIT2		= TSNMHD + 0x0228,
    TAEN0		= TSNMHD + 0x0230,
    TAEN1		= TSNMHD + 0x0234,
    TASFE		= TSNMHD + 0x0240,
    TACLL0		= TSNMHD + 0x0250,
    TACLL1		= TSNMHD + 0x0254,
    TACLL2		= TSNMHD + 0x0258,
    CACC		= TSNMHD + 0x0260,
    CCS		= TSNMHD + 0x0264,
    CAIV0		= TSNMHD + 0x0270,
    CAUL0		= TSNMHD + 0x0290,
    TOCST0		= TSNMHD + 0x0300,
    TOCST1		= TSNMHD + 0x0304,
    TOCST2		= TSNMHD + 0x0308,
    TOLIT0		= TSNMHD + 0x0310,
    TOLIT1		= TSNMHD + 0x0314,
    TOLIT2		= TSNMHD + 0x0318,
    TOEN0		= TSNMHD + 0x0320,
    TOEN1		= TSNMHD + 0x0324,
    TOSFE		= TSNMHD + 0x0330,
    TCLR0		= TSNMHD + 0x0340,
    TCLR1		= TSNMHD + 0x0344,
    TCLR2		= TSNMHD + 0x0348,
    TSMS		= TSNMHD + 0x0350,
    COCC		= TSNMHD + 0x0360,
    COIV0		= TSNMHD + 0x03b0,
    COUL0		= TSNMHD + 0x03d0,
    QSTMACU0	= TSNMHD + 0x0400,
    QSTMACD0	= TSNMHD + 0x0404,
    QSTMAMU0	= TSNMHD + 0x0408,
    QSTMAMD0	= TSNMHD + 0x040c,
    QSFTVL0		= TSNMHD + 0x0410,
    QSFTVLM0	= TSNMHD + 0x0414,
    QSFTMSD0	= TSNMHD + 0x0418,
    QSFTGMI0	= TSNMHD + 0x041c,
    QSFTLS		= TSNMHD + 0x0600,
    QSFTLIS		= TSNMHD + 0x0604,
    QSFTLIE		= TSNMHD + 0x0608,
    QSFTLID		= TSNMHD + 0x060c,
    QSMSMC		= TSNMHD + 0x0610,
    QSGTMC		= TSNMHD + 0x0614,
    QSEIS		= TSNMHD + 0x0618,
    QSEIE		= TSNMHD + 0x061c,
    QSEID		= TSNMHD + 0x0620,
    QGACST0		= TSNMHD + 0x0630,
    QGACST1		= TSNMHD + 0x0634,
    QGACST2		= TSNMHD + 0x0638,
    QGALIT1		= TSNMHD + 0x0640,
    QGALIT2		= TSNMHD + 0x0644,
    QGAEN0		= TSNMHD + 0x0648,
    QGAEN1		= TSNMHD + 0x074c,
    QGIGS		= TSNMHD + 0x0650,
    QGGC		= TSNMHD + 0x0654,
    QGATL0		= TSNMHD + 0x0664,
    QGATL1		= TSNMHD + 0x0668,
    QGATL2		= TSNMHD + 0x066c,
    QGOCST0		= TSNMHD + 0x0670,
    QGOCST1		= TSNMHD + 0x0674,
    QGOCST2		= TSNMHD + 0x0678,
    QGOLIT0		= TSNMHD + 0x067c,
    QGOLIT1		= TSNMHD + 0x0680,
    QGOLIT2		= TSNMHD + 0x0684,
    QGOEN0		= TSNMHD + 0x0688,
    QGOEN1		= TSNMHD + 0x068c,
    QGTRO		= TSNMHD + 0x0690,
    QGTR1		= TSNMHD + 0x0694,
    QGTR2		= TSNMHD + 0x0698,
    QGFSMS		= TSNMHD + 0x069c,
    QTMIS		= TSNMHD + 0x06e0,
    QTMIE		= TSNMHD + 0x06e4,
    QTMID		= TSNMHD + 0x06e8,
    QMEC		= TSNMHD + 0x0700,
    QMMC		= TSNMHD + 0x0704,
    QRFDC		= TSNMHD + 0x0708,
    QYFDC		= TSNMHD + 0x070c,
    QVTCMC0		= TSNMHD + 0x0710,
    QMCBSC0		= TSNMHD + 0x0750,
    QMCIRC0		= TSNMHD + 0x0790,
    QMEBSC0		= TSNMHD + 0x07d0,
    QMEIRC0		= TSNMHD + 0x0710,
    QMCFC		= TSNMHD + 0x0850,
    QMEIS		= TSNMHD + 0x0860,
    QMEIE		= TSNMHD + 0x0864,
    QMEID		= TSNMHD + 0x086c,
    QSMFC0		= TSNMHD + 0x0870,
    QMSPPC0		= TSNMHD + 0x08b0,
    QMSRPC0		= TSNMHD + 0x08f0,
    QGPPC0		= TSNMHD + 0x0930,
    QGRPC0		= TSNMHD + 0x0950,
    QMDPC0		= TSNMHD + 0x0970,
    QMGPC0		= TSNMHD + 0x09b0,
    QMYPC0		= TSNMHD + 0x09f0,
    QMRPC0		= TSNMHD + 0x0a30,
    MQSTMACU	= TSNMHD + 0x0a70,
    MQSTMACD	= TSNMHD + 0x0a74,
    MQSTMAMU	= TSNMHD + 0x0a78,
    MQSTMAMD	= TSNMHD + 0x0a7c,
    MQSFTVL		= TSNMHD + 0x0a80,
    MQSFTVLM	= TSNMHD + 0x0a84,
    MQSFTMSD	= TSNMHD + 0x0a88,
    MQSFTGMI	= TSNMHD + 0x0a8c,

    CFCR0		= RMSO + 0x0800,
    FMSCR		= RMSO + 0x0c10,

    MMC		= RMRO + 0x0000,
    MPSM		= RMRO + 0x0010,
    MPIC		= RMRO + 0x0014,
    MTFFC		= RMRO + 0x0020,
    MTPFC		= RMRO + 0x0024,
    MTATC0		= RMRO + 0x0040,
    MRGC		= RMRO + 0x0080,
    MRMAC0		= RMRO + 0x0084,
    MRMAC1		= RMRO + 0x0088,
    MRAFC		= RMRO + 0x008c,
    MRSCE		= RMRO + 0x0090,
    MRSCP		= RMRO + 0x0094,
    MRSCC		= RMRO + 0x0098,
    MRFSCE		= RMRO + 0x009c,
    MRFSCP		= RMRO + 0x00a0,
    MTRC		= RMRO + 0x00a4,
    MPFC		= RMRO + 0x0100,
    MLVC		= RMRO + 0x0340,
    MEEEC		= RMRO + 0x0350,
    MLBC		= RMRO + 0x0360,
    MGMR		= RMRO + 0x0400,
    MMPFTCT		= RMRO + 0x0410,
    MAPFTCT		= RMRO + 0x0414,
    MPFRCT		= RMRO + 0x0418,
    MFCICT		= RMRO + 0x041c,
    MEEECT		= RMRO + 0x0420,
    MEIS		= RMRO + 0x0500,
    MEIE		= RMRO + 0x0504,
    MEID		= RMRO + 0x0508,
    MMIS0		= RMRO + 0x0510,
    MMIE0		= RMRO + 0x0514,
    MMID0		= RMRO + 0x0518,
    MMIS1		= RMRO + 0x0520,
    MMIE1		= RMRO + 0x0524,
    MMID1		= RMRO + 0x0528,
    MMIS2		= RMRO + 0x0530,
    MMIE2		= RMRO + 0x0534,
    MMID2		= RMRO + 0x0538,
    MXMS		= RMRO + 0x0600,

}

// AXIBMI

pub const RR_RST_COMPLETE: c_uint = 0x03;
pub const AXIWC_DEFAULT: c_uint = 0xffff;
pub const AXIRC_DEFAULT: c_uint = 0xffff;

pub const TATLS0_TATEN_SHIFT: c_int = 24;

pub const RATLS0_RATEN_SHIFT: c_int = 24;

// MHD
pub const OSR_OPS: c_uint = 0x07;

pub const TGC1_TQTM_SFM: c_uint = 0xff00;
pub const TGC1_STTV_DEFAULT: c_uint = 0x03;
pub const TMS_MFS_MAX: c_uint = 0x2800;
// RMAC System

// RMAC

pub const MPIC_PIS_MII: c_int = 0;
pub const MPIC_PIS_RMII: c_uint = 0x01;
pub const MPIC_PIS_GMII: c_uint = 0x02;
pub const MPIC_PIS_RGMII: c_uint = 0x03;
pub const MPIC_LSC_SHIFT: c_int = 2;

pub const MPIC_PSMCS_SHIFT: c_int = 16;

pub const MPIC_PSMHT_SHIFT: c_int = 24;

pub const MPSM_PDA_SHIFT: c_int = 3;

pub const MPSM_PRA_SHIFT: c_int = 8;

pub const MPSM_PRD_SHIFT: c_int = 16;

// RTSN
pub const RTSN_INTERVAL_US: c_int = 1000;
pub const RTSN_TIMEOUT_US: c_int = 1000000;
pub const TX_NUM_CHAINS: c_int = 1;
pub const RX_NUM_CHAINS: c_int = 1;
pub const TX_CHAIN_SIZE: c_int = 1024;
pub const RX_CHAIN_SIZE: c_int = 1024;
pub const TX_CHAIN_IDX: c_int = 0;
pub const RX_CHAIN_IDX: c_int = 0;

pub const PKT_BUF_SZ: c_int = 1584;
pub const RTSN_ALIGN: c_int = 128;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtsn_mode {
    OCR_OPC_DISABLE,
    OCR_OPC_CONFIG,
    OCR_OPC_OPERATION,
}

// Descriptors
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum RX_DS_CC_BIT {
    RX_DS	= 0x0fff, /* Data size */
    RX_TR	= 0x1000, /* Truncation indication */
    RX_EI	= 0x2000, /* Error indication */
    RX_PS	= 0xc000, /* Padding selection */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum TX_FS_TAGL_BIT {
    TX_DS	= 0x0fff, /* Data size */
    TX_TAGL	= 0xf000, /* Frame tag LSBs */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum DIE_DT {
// HW/SW arbitration
    DT_FEMPTY_IS	= 0x10,
    DT_FEMPTY_IC	= 0x20,
    DT_FEMPTY_ND	= 0x30,
    DT_FEMPTY	= 0x40,
    DT_FEMPTY_START	= 0x50,
    DT_FEMPTY_MID	= 0x60,
    DT_FEMPTY_END	= 0x70,

// Frame data
    DT_FSINGLE	= 0x80,
    DT_FSTART	= 0x90,
    DT_FMID		= 0xa0,
    DT_FEND		= 0xb0,

// Chain control
    DT_LEMPTY	= 0xc0,
    DT_EEMPTY	= 0xd0,
    DT_LINK		= 0xe0,
    DT_EOS		= 0xf0,

    DT_MASK		= 0xf0,
    D_DIE		= 0x08,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtsn_desc {
    pub info_ds: __le16,
    pub info: __u8,
    pub die_dt: u8,
    pub dptr: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtsn_ts_desc {
    pub info_ds: __le16,
    pub info: __u8,
    pub die_dt: u8,
    pub dptr: __le32,
    pub ts_nsec: __le32,
    pub ts_sec: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtsn_ext_desc {
    pub info_ds: __le16,
    pub info: __u8,
    pub die_dt: u8,
    pub dptr: __le32,
    pub info1: __le64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtsn_ext_ts_desc {
    pub info_ds: __le16,
    pub info: __u8,
    pub die_dt: u8,
    pub dptr: __le32,
    pub info1: __le64,
    pub ts_nsec: __le32,
    pub ts_sec: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum EXT_INFO_DS_BIT {
    TXC = 0x4000,
}
