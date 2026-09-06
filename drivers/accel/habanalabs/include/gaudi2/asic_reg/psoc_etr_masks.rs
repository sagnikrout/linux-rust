//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/gaudi2/asic_reg/psoc_etr_masks.h
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
// Copyright 2016-2020 HabanaLabs, Ltd.
// All Rights Reserved.
//
// This is an auto-generated file
// DO NOT EDIT BELOW
//
// PSOC_ETR
// (Prototype: ETR)
//
// PSOC_ETR_RSZ
pub const PSOC_ETR_RSZ_RSZ_ETR_SHIFT: c_int = 0;
pub const PSOC_ETR_RSZ_RSZ_ETR_MASK: c_uint = 0x7FFFFFFF;
// PSOC_ETR_STS
pub const PSOC_ETR_STS_FULL_SHIFT: c_int = 0;
pub const PSOC_ETR_STS_FULL_MASK: c_uint = 0x1;
pub const PSOC_ETR_STS_TRIGGERED_SHIFT: c_int = 1;
pub const PSOC_ETR_STS_TRIGGERED_MASK: c_uint = 0x2;
pub const PSOC_ETR_STS_TMCREADY_SHIFT: c_int = 2;
pub const PSOC_ETR_STS_TMCREADY_MASK: c_uint = 0x4;
pub const PSOC_ETR_STS_FTEMPTY_SHIFT: c_int = 3;
pub const PSOC_ETR_STS_FTEMPTY_MASK: c_uint = 0x8;
pub const PSOC_ETR_STS_EMPTY_SHIFT: c_int = 4;
pub const PSOC_ETR_STS_EMPTY_MASK: c_uint = 0x10;
pub const PSOC_ETR_STS_MEMERR_SHIFT: c_int = 5;
pub const PSOC_ETR_STS_MEMERR_MASK: c_uint = 0x20;
// PSOC_ETR_RRD
pub const PSOC_ETR_RRD_RRD_SHIFT: c_int = 0;
pub const PSOC_ETR_RRD_RRD_MASK: c_uint = 0xFFFFFFFF;
// PSOC_ETR_RRP
pub const PSOC_ETR_RRP_RRP_SHIFT: c_int = 0;
pub const PSOC_ETR_RRP_RRP_MASK: c_uint = 0xFFFFFFFF;
// PSOC_ETR_RWP
pub const PSOC_ETR_RWP_RWP_SHIFT: c_int = 0;
pub const PSOC_ETR_RWP_RWP_MASK: c_uint = 0xFFFFFFFF;
// PSOC_ETR_TRG
pub const PSOC_ETR_TRG_TRG_SHIFT: c_int = 0;
pub const PSOC_ETR_TRG_TRG_MASK: c_uint = 0xFFFFFFFF;
// PSOC_ETR_CTL
pub const PSOC_ETR_CTL_TRACECAPTEN_SHIFT: c_int = 0;
pub const PSOC_ETR_CTL_TRACECAPTEN_MASK: c_uint = 0x1;
// PSOC_ETR_RWD
pub const PSOC_ETR_RWD_RWD_SHIFT: c_int = 0;
pub const PSOC_ETR_RWD_RWD_MASK: c_uint = 0xFFFFFFFF;
// PSOC_ETR_MODE
pub const PSOC_ETR_MODE_MODE_SHIFT: c_int = 0;
pub const PSOC_ETR_MODE_MODE_MASK: c_uint = 0x3;
// PSOC_ETR_LBUFLEVEL
pub const PSOC_ETR_LBUFLEVEL_LBUFLEVEL_SHIFT: c_int = 0;
pub const PSOC_ETR_LBUFLEVEL_LBUFLEVEL_MASK: c_uint = 0x7FFFFFFF;
// PSOC_ETR_CBUFLEVEL
pub const PSOC_ETR_CBUFLEVEL_CBUFLEVEL_SHIFT: c_int = 0;
pub const PSOC_ETR_CBUFLEVEL_CBUFLEVEL_MASK: c_uint = 0x7FFFFFFF;
// PSOC_ETR_BUFWM
pub const PSOC_ETR_BUFWM_BUFWM_SHIFT: c_int = 0;
pub const PSOC_ETR_BUFWM_BUFWM_MASK: c_uint = 0x3FFFFFFF;
// PSOC_ETR_RRPHI
pub const PSOC_ETR_RRPHI_RRPHI_SHIFT: c_int = 0;
pub const PSOC_ETR_RRPHI_RRPHI_MASK: c_uint = 0xFF;
// PSOC_ETR_RWPHI
pub const PSOC_ETR_RWPHI_RWPHI_SHIFT: c_int = 0;
pub const PSOC_ETR_RWPHI_RWPHI_MASK: c_uint = 0xFF;
// PSOC_ETR_AXICTL
pub const PSOC_ETR_AXICTL_PROTCTRLBIT0_SHIFT: c_int = 0;
pub const PSOC_ETR_AXICTL_PROTCTRLBIT0_MASK: c_uint = 0x1;
pub const PSOC_ETR_AXICTL_PROTCTRLBIT1_SHIFT: c_int = 1;
pub const PSOC_ETR_AXICTL_PROTCTRLBIT1_MASK: c_uint = 0x2;
pub const PSOC_ETR_AXICTL_CACHECTRLBIT0_SHIFT: c_int = 2;
pub const PSOC_ETR_AXICTL_CACHECTRLBIT0_MASK: c_uint = 0x4;
pub const PSOC_ETR_AXICTL_CACHECTRLBIT1_SHIFT: c_int = 3;
pub const PSOC_ETR_AXICTL_CACHECTRLBIT1_MASK: c_uint = 0x8;
pub const PSOC_ETR_AXICTL_CACHECTRLBIT2_SHIFT: c_int = 4;
pub const PSOC_ETR_AXICTL_CACHECTRLBIT2_MASK: c_uint = 0x10;
pub const PSOC_ETR_AXICTL_CACHECTRLBIT3_SHIFT: c_int = 5;
pub const PSOC_ETR_AXICTL_CACHECTRLBIT3_MASK: c_uint = 0x20;
pub const PSOC_ETR_AXICTL_SCATTERGATHERMODE_SHIFT: c_int = 7;
pub const PSOC_ETR_AXICTL_SCATTERGATHERMODE_MASK: c_uint = 0x80;
pub const PSOC_ETR_AXICTL_WRBURSTLEN_SHIFT: c_int = 8;
pub const PSOC_ETR_AXICTL_WRBURSTLEN_MASK: c_uint = 0xF00;
// PSOC_ETR_DBALO
pub const PSOC_ETR_DBALO_BUFADDRLO_SHIFT: c_int = 0;
pub const PSOC_ETR_DBALO_BUFADDRLO_MASK: c_uint = 0xFFFFFFFF;
// PSOC_ETR_DBAHI
pub const PSOC_ETR_DBAHI_BUFADDRHI_SHIFT: c_int = 0;
pub const PSOC_ETR_DBAHI_BUFADDRHI_MASK: c_uint = 0xFF;
// PSOC_ETR_FFSR
pub const PSOC_ETR_FFSR_FLINPROG_SHIFT: c_int = 0;
pub const PSOC_ETR_FFSR_FLINPROG_MASK: c_uint = 0x1;
pub const PSOC_ETR_FFSR_FTSTOPPED_SHIFT: c_int = 1;
pub const PSOC_ETR_FFSR_FTSTOPPED_MASK: c_uint = 0x2;
// PSOC_ETR_FFCR
pub const PSOC_ETR_FFCR_ENFT_SHIFT: c_int = 0;
pub const PSOC_ETR_FFCR_ENFT_MASK: c_uint = 0x1;
pub const PSOC_ETR_FFCR_ENTI_SHIFT: c_int = 1;
pub const PSOC_ETR_FFCR_ENTI_MASK: c_uint = 0x2;
pub const PSOC_ETR_FFCR_FONFLIN_SHIFT: c_int = 4;
pub const PSOC_ETR_FFCR_FONFLIN_MASK: c_uint = 0x10;
pub const PSOC_ETR_FFCR_FONTRIGEVT_SHIFT: c_int = 5;
pub const PSOC_ETR_FFCR_FONTRIGEVT_MASK: c_uint = 0x20;
pub const PSOC_ETR_FFCR_FLUSHMAN_SHIFT: c_int = 6;
pub const PSOC_ETR_FFCR_FLUSHMAN_MASK: c_uint = 0x40;
pub const PSOC_ETR_FFCR_TRIGONTRIGIN_SHIFT: c_int = 8;
pub const PSOC_ETR_FFCR_TRIGONTRIGIN_MASK: c_uint = 0x100;
pub const PSOC_ETR_FFCR_TRIGONTRIGEVT_SHIFT: c_int = 9;
pub const PSOC_ETR_FFCR_TRIGONTRIGEVT_MASK: c_uint = 0x200;
pub const PSOC_ETR_FFCR_TRIGONFL_SHIFT: c_int = 10;
pub const PSOC_ETR_FFCR_TRIGONFL_MASK: c_uint = 0x400;
pub const PSOC_ETR_FFCR_STOPONFL_SHIFT: c_int = 12;
pub const PSOC_ETR_FFCR_STOPONFL_MASK: c_uint = 0x1000;
pub const PSOC_ETR_FFCR_STOPONTRIGEVT_SHIFT: c_int = 13;
pub const PSOC_ETR_FFCR_STOPONTRIGEVT_MASK: c_uint = 0x2000;
// PSOC_ETR_PSCR
pub const PSOC_ETR_PSCR_PSCOUNT_SHIFT: c_int = 0;
pub const PSOC_ETR_PSCR_PSCOUNT_MASK: c_uint = 0x1F;
// PSOC_ETR_ITMISCOP0
pub const PSOC_ETR_ITMISCOP0_ACQCOMP_SHIFT: c_int = 0;
pub const PSOC_ETR_ITMISCOP0_ACQCOMP_MASK: c_uint = 0x1;
pub const PSOC_ETR_ITMISCOP0_FULL_SHIFT: c_int = 1;
pub const PSOC_ETR_ITMISCOP0_FULL_MASK: c_uint = 0x2;
// PSOC_ETR_ITTRFLIN
pub const PSOC_ETR_ITTRFLIN_TRIGIN_SHIFT: c_int = 0;
pub const PSOC_ETR_ITTRFLIN_TRIGIN_MASK: c_uint = 0x1;
pub const PSOC_ETR_ITTRFLIN_FLUSHIN_SHIFT: c_int = 1;
pub const PSOC_ETR_ITTRFLIN_FLUSHIN_MASK: c_uint = 0x2;
// PSOC_ETR_ITATBDATA0
pub const PSOC_ETR_ITATBDATA0_ATDATASBIT0_SHIFT: c_int = 0;
pub const PSOC_ETR_ITATBDATA0_ATDATASBIT0_MASK: c_uint = 0x1;
pub const PSOC_ETR_ITATBDATA0_ATDATASBIT7_SHIFT: c_int = 1;
pub const PSOC_ETR_ITATBDATA0_ATDATASBIT7_MASK: c_uint = 0x2;
pub const PSOC_ETR_ITATBDATA0_ATDATASBIT15_SHIFT: c_int = 2;
pub const PSOC_ETR_ITATBDATA0_ATDATASBIT15_MASK: c_uint = 0x4;
pub const PSOC_ETR_ITATBDATA0_ATDATASBIT23_SHIFT: c_int = 3;
pub const PSOC_ETR_ITATBDATA0_ATDATASBIT23_MASK: c_uint = 0x8;
pub const PSOC_ETR_ITATBDATA0_ATDATASBIT31_SHIFT: c_int = 4;
pub const PSOC_ETR_ITATBDATA0_ATDATASBIT31_MASK: c_uint = 0x10;
pub const PSOC_ETR_ITATBDATA0_ATDATASBIT39_SHIFT: c_int = 5;
pub const PSOC_ETR_ITATBDATA0_ATDATASBIT39_MASK: c_uint = 0x20;
pub const PSOC_ETR_ITATBDATA0_ATDATASBIT47_SHIFT: c_int = 6;
pub const PSOC_ETR_ITATBDATA0_ATDATASBIT47_MASK: c_uint = 0x40;
pub const PSOC_ETR_ITATBDATA0_ATDATASBIT55_SHIFT: c_int = 7;
pub const PSOC_ETR_ITATBDATA0_ATDATASBIT55_MASK: c_uint = 0x80;
pub const PSOC_ETR_ITATBDATA0_ATDATASBIT63_SHIFT: c_int = 8;
pub const PSOC_ETR_ITATBDATA0_ATDATASBIT63_MASK: c_uint = 0x100;
// PSOC_ETR_ITATBCTR2
pub const PSOC_ETR_ITATBCTR2_ATREADYS_SHIFT: c_int = 0;
pub const PSOC_ETR_ITATBCTR2_ATREADYS_MASK: c_uint = 0x1;
pub const PSOC_ETR_ITATBCTR2_AFVALIDS_SHIFT: c_int = 1;
pub const PSOC_ETR_ITATBCTR2_AFVALIDS_MASK: c_uint = 0x2;
pub const PSOC_ETR_ITATBCTR2_SYNCREQS_SHIFT: c_int = 2;
pub const PSOC_ETR_ITATBCTR2_SYNCREQS_MASK: c_uint = 0x4;
// PSOC_ETR_ITATBCTR1
pub const PSOC_ETR_ITATBCTR1_ATIDS_SHIFT: c_int = 0;
pub const PSOC_ETR_ITATBCTR1_ATIDS_MASK: c_uint = 0x7F;
// PSOC_ETR_ITATBCTR0
pub const PSOC_ETR_ITATBCTR0_ATVALIDS_SHIFT: c_int = 0;
pub const PSOC_ETR_ITATBCTR0_ATVALIDS_MASK: c_uint = 0x1;
pub const PSOC_ETR_ITATBCTR0_AFREADYS_SHIFT: c_int = 1;
pub const PSOC_ETR_ITATBCTR0_AFREADYS_MASK: c_uint = 0x2;
pub const PSOC_ETR_ITATBCTR0_ATBYTESS_SHIFT: c_int = 8;
pub const PSOC_ETR_ITATBCTR0_ATBYTESS_MASK: c_uint = 0x700;
// PSOC_ETR_ITCTRL
pub const PSOC_ETR_ITCTRL_INTEGRATION_MODE_SHIFT: c_int = 0;
pub const PSOC_ETR_ITCTRL_INTEGRATION_MODE_MASK: c_uint = 0x1;
// PSOC_ETR_CLAIMSET
pub const PSOC_ETR_CLAIMSET_CLAIMSET_SHIFT: c_int = 0;
pub const PSOC_ETR_CLAIMSET_CLAIMSET_MASK: c_uint = 0xF;
// PSOC_ETR_CLAIMCLR
pub const PSOC_ETR_CLAIMCLR_CLAIMCLR_SHIFT: c_int = 0;
pub const PSOC_ETR_CLAIMCLR_CLAIMCLR_MASK: c_uint = 0xF;
// PSOC_ETR_LAR
pub const PSOC_ETR_LAR_ACCESS_W_SHIFT: c_int = 0;
pub const PSOC_ETR_LAR_ACCESS_W_MASK: c_uint = 0xFFFFFFFF;
// PSOC_ETR_LSR
pub const PSOC_ETR_LSR_LOCKEXIST_SHIFT: c_int = 0;
pub const PSOC_ETR_LSR_LOCKEXIST_MASK: c_uint = 0x1;
pub const PSOC_ETR_LSR_LOCKGRANT_SHIFT: c_int = 1;
pub const PSOC_ETR_LSR_LOCKGRANT_MASK: c_uint = 0x2;
pub const PSOC_ETR_LSR_LOCKTYPE_SHIFT: c_int = 2;
pub const PSOC_ETR_LSR_LOCKTYPE_MASK: c_uint = 0x4;
// PSOC_ETR_AUTHSTATUS
pub const PSOC_ETR_AUTHSTATUS_NSID_SHIFT: c_int = 0;
pub const PSOC_ETR_AUTHSTATUS_NSID_MASK: c_uint = 0x3;
pub const PSOC_ETR_AUTHSTATUS_NSNID_SHIFT: c_int = 2;
pub const PSOC_ETR_AUTHSTATUS_NSNID_MASK: c_uint = 0xC;
pub const PSOC_ETR_AUTHSTATUS_SID_SHIFT: c_int = 4;
pub const PSOC_ETR_AUTHSTATUS_SID_MASK: c_uint = 0x30;
pub const PSOC_ETR_AUTHSTATUS_SNID_SHIFT: c_int = 6;
pub const PSOC_ETR_AUTHSTATUS_SNID_MASK: c_uint = 0xC0;
// PSOC_ETR_DEVID
pub const PSOC_ETR_DEVID_ATBINPORTCOUNT_SHIFT: c_int = 0;
pub const PSOC_ETR_DEVID_ATBINPORTCOUNT_MASK: c_uint = 0x1F;
pub const PSOC_ETR_DEVID_CLKSCHEME_SHIFT: c_int = 5;
pub const PSOC_ETR_DEVID_CLKSCHEME_MASK: c_uint = 0x20;
pub const PSOC_ETR_DEVID_CONFIGTYPE_SHIFT: c_int = 6;
pub const PSOC_ETR_DEVID_CONFIGTYPE_MASK: c_uint = 0xC0;
pub const PSOC_ETR_DEVID_MEMWIDTH_SHIFT: c_int = 8;
pub const PSOC_ETR_DEVID_MEMWIDTH_MASK: c_uint = 0x700;
pub const PSOC_ETR_DEVID_WBUF_DEPTH_SHIFT: c_int = 11;
pub const PSOC_ETR_DEVID_WBUF_DEPTH_MASK: c_uint = 0x3800;
// PSOC_ETR_DEVTYPE
pub const PSOC_ETR_DEVTYPE_MAJOR_TYPE_SHIFT: c_int = 0;
pub const PSOC_ETR_DEVTYPE_MAJOR_TYPE_MASK: c_uint = 0xF;
pub const PSOC_ETR_DEVTYPE_SUB_TYPE_SHIFT: c_int = 4;
pub const PSOC_ETR_DEVTYPE_SUB_TYPE_MASK: c_uint = 0xF0;
// PSOC_ETR_PERIPHID4
pub const PSOC_ETR_PERIPHID4_JEP106_CONT_SHIFT: c_int = 0;
pub const PSOC_ETR_PERIPHID4_JEP106_CONT_MASK: c_uint = 0xF;
pub const PSOC_ETR_PERIPHID4_FOURKB_COUNT_SHIFT: c_int = 4;
pub const PSOC_ETR_PERIPHID4_FOURKB_COUNT_MASK: c_uint = 0xF0;
// PSOC_ETR_PERIPHID5
pub const PSOC_ETR_PERIPHID5_PERIPHID5_SHIFT: c_int = 0;
pub const PSOC_ETR_PERIPHID5_PERIPHID5_MASK: c_uint = 0xFFFFFFFF;
// PSOC_ETR_PERIPHID6
pub const PSOC_ETR_PERIPHID6_PERIPHID6_SHIFT: c_int = 0;
pub const PSOC_ETR_PERIPHID6_PERIPHID6_MASK: c_uint = 0xFFFFFFFF;
// PSOC_ETR_PERIPHID7
pub const PSOC_ETR_PERIPHID7_PERIPHID7_SHIFT: c_int = 0;
pub const PSOC_ETR_PERIPHID7_PERIPHID7_MASK: c_uint = 0xFFFFFFFF;
// PSOC_ETR_PERIPHID0
pub const PSOC_ETR_PERIPHID0_PART_NUMBER_BITS7TO0_SHIFT: c_int = 0;
pub const PSOC_ETR_PERIPHID0_PART_NUMBER_BITS7TO0_MASK: c_uint = 0xFF;
// PSOC_ETR_PERIPHID1
pub const PSOC_ETR_PERIPHID1_PART_NUMBER_BITS11TO8_SHIFT: c_int = 0;
pub const PSOC_ETR_PERIPHID1_PART_NUMBER_BITS11TO8_MASK: c_uint = 0xF;
pub const PSOC_ETR_PERIPHID1_JEP106_BITS3TO0_SHIFT: c_int = 4;
pub const PSOC_ETR_PERIPHID1_JEP106_BITS3TO0_MASK: c_uint = 0xF0;
// PSOC_ETR_PERIPHID2
pub const PSOC_ETR_PERIPHID2_JEP106_BITS6TO4_SHIFT: c_int = 0;
pub const PSOC_ETR_PERIPHID2_JEP106_BITS6TO4_MASK: c_uint = 0x7;
pub const PSOC_ETR_PERIPHID2_JEDEC_SHIFT: c_int = 3;
pub const PSOC_ETR_PERIPHID2_JEDEC_MASK: c_uint = 0x8;
pub const PSOC_ETR_PERIPHID2_REVISION_SHIFT: c_int = 4;
pub const PSOC_ETR_PERIPHID2_REVISION_MASK: c_uint = 0xF0;
// PSOC_ETR_PERIPHID3
pub const PSOC_ETR_PERIPHID3_CUSTOMER_MODIFIED_SHIFT: c_int = 0;
pub const PSOC_ETR_PERIPHID3_CUSTOMER_MODIFIED_MASK: c_uint = 0xF;
pub const PSOC_ETR_PERIPHID3_REVAND_SHIFT: c_int = 4;
pub const PSOC_ETR_PERIPHID3_REVAND_MASK: c_uint = 0xF0;
// PSOC_ETR_COMPID0
pub const PSOC_ETR_COMPID0_PREAMBLE_SHIFT: c_int = 0;
pub const PSOC_ETR_COMPID0_PREAMBLE_MASK: c_uint = 0xFF;
// PSOC_ETR_COMPID1
pub const PSOC_ETR_COMPID1_PREAMBLE_SHIFT: c_int = 0;
pub const PSOC_ETR_COMPID1_PREAMBLE_MASK: c_uint = 0xF;
pub const PSOC_ETR_COMPID1_F_CLASS_SHIFT: c_int = 4;
pub const PSOC_ETR_COMPID1_F_CLASS_MASK: c_uint = 0xF0;
// PSOC_ETR_COMPID2
pub const PSOC_ETR_COMPID2_PREAMBLE_SHIFT: c_int = 0;
pub const PSOC_ETR_COMPID2_PREAMBLE_MASK: c_uint = 0xFF;
// PSOC_ETR_COMPID3
pub const PSOC_ETR_COMPID3_PREAMBLE_SHIFT: c_int = 0;
pub const PSOC_ETR_COMPID3_PREAMBLE_MASK: c_uint = 0xFF;
