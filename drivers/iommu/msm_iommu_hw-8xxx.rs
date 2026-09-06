//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/iommu/msm_iommu_hw-8xxx.h
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
// Copyright (c) 2010-2011, Code Aurora Forum. All rights reserved.
//
pub const CTX_SHIFT: c_int = 12;

// Wrappers for numbered registers

// Field wrappers

pub const NUM_FL_PTE: c_int = 4096;
pub const NUM_SL_PTE: c_int = 256;
pub const NUM_TEX_CLASS: c_int = 8;
// First-level page table bits
pub const FL_BASE_MASK: c_uint = 0xFFFFFC00;

// Second-level page table bits
pub const SL_BASE_MASK_LARGE: c_uint = 0xFFFF0000;
pub const SL_BASE_MASK_SMALL: c_uint = 0xFFFFF000;

// Memory type and cache policy attributes
pub const MT_SO: c_int = 0;
pub const MT_DEV: c_int = 1;
pub const MT_NORMAL: c_int = 2;
pub const CP_NONCACHED: c_int = 0;
pub const CP_WB_WA: c_int = 1;
pub const CP_WT: c_int = 2;
pub const CP_WB_NWA: c_int = 3;
// Global register setters / getters

// Context register setters/getters

// Global field setters / getters
// Global Field Setters:
// CBACR_N

// M2VCBR_N

// CR

// ESR

// ESYNR0

// ESYNR1

// TESTBUSCR

// TLBIVMID

// TLBRSW

// TLBTR0

// TLBTR1

// TLBTR2

// Global Field Getters
// CBACR_N

// M2VCBR_N

// CR

// ESR

// ESYNR0

// ESYNR1

// IDR

// REV

// TESTBUSCR

// TLBIVMID

// TLBTR0

// TLBTR1

// TLBTR2

// Context Register setters / getters
// Context Register setters
// ACTLR

// BFBCR

// CONTEXTIDR

// FSR

// FSYNR0

// FSYNR1

// NMRR

// PAR

// PRRR

// RESUME

// SCTLR

// TLBLKCR

// TTBCR

// TTBR0

// TTBR1

// V2PSR

// Context Register getters
// ACTLR

// BFBCR

// CONTEXTIDR

// FSR

// FSYNR0

// FSYNR1

// NMRR

// PAR

// PRRR

// RESUME

// SCTLR

// TLBLKCR

// TTBCR

// TTBR0

// TTBR1

// V2PSR

// Global Registers

// Context Bank Registers

// Global Register Fields
// CBACRn

// CR

// ESR

// ESYNR0

// ESYNR1

// IDR

// M2VCBRn

// REV

// TESTBUSCR

// TLBIVMID

// TLBRSW

// TLBTR0

// TLBTR1

// TLBTR2

// Context Register Fields
// ACTLR

// BFBCR

// CONTEXTIDR

// FSR

// FSYNR0

// FSYNR1

// NMRR

// PAR

// If a fault is present, these are the

// If NO fault is present, the following fields are in effect
// (FAULT remains as before)

// PRRR

// RESUME

// SCTLR

// TLBIASID

// TLBIVA

// TLBIVAA

// TLBLCKR

// TTBCR

// TTBR0

// TTBR1

// V2PSR

// V2Pxx

// Global Register Masks
// CBACRn
pub const RWVMID_MASK: c_uint = 0x1F;
pub const RWE_MASK: c_uint = 0x01;
pub const RWGE_MASK: c_uint = 0x01;
pub const CBVMID_MASK: c_uint = 0x1F;
pub const IRPTNDX_MASK: c_uint = 0xFF;
// CR
pub const RPUE_MASK: c_uint = 0x01;
pub const RPUERE_MASK: c_uint = 0x01;
pub const RPUEIE_MASK: c_uint = 0x01;
pub const DCDEE_MASK: c_uint = 0x01;
pub const CLIENTPD_MASK: c_uint = 0x01;
pub const STALLD_MASK: c_uint = 0x01;
pub const TLBLKCRWE_MASK: c_uint = 0x01;
pub const CR_TLBIALLCFG_MASK: c_uint = 0x01;
pub const TLBIVMIDCFG_MASK: c_uint = 0x01;
pub const CR_HUME_MASK: c_uint = 0x01;
// ESR
pub const CFG_MASK: c_uint = 0x01;
pub const BYPASS_MASK: c_uint = 0x01;
pub const ESR_MULTI_MASK: c_uint = 0x01;
// ESYNR0
pub const ESYNR0_AMID_MASK: c_uint = 0xFF;
pub const ESYNR0_APID_MASK: c_uint = 0x1F;
pub const ESYNR0_ABID_MASK: c_uint = 0x07;
pub const ESYNR0_AVMID_MASK: c_uint = 0x1F;
pub const ESYNR0_ATID_MASK: c_uint = 0xFF;
// ESYNR1
pub const ESYNR1_AMEMTYPE_MASK: c_uint = 0x07;
pub const ESYNR1_ASHARED_MASK: c_uint = 0x01;
pub const ESYNR1_AINNERSHARED_MASK: c_uint = 0x01;
pub const ESYNR1_APRIV_MASK: c_uint = 0x01;
pub const ESYNR1_APROTNS_MASK: c_uint = 0x01;
pub const ESYNR1_AINST_MASK: c_uint = 0x01;
pub const ESYNR1_AWRITE_MASK: c_uint = 0x01;
pub const ESYNR1_ABURST_MASK: c_uint = 0x01;
pub const ESYNR1_ALEN_MASK: c_uint = 0x0F;
pub const ESYNR1_ASIZE_MASK: c_uint = 0x01;
pub const ESYNR1_ALOCK_MASK: c_uint = 0x03;
pub const ESYNR1_AOOO_MASK: c_uint = 0x01;
pub const ESYNR1_AFULL_MASK: c_uint = 0x01;
pub const ESYNR1_AC_MASK: c_uint = 0x01;
pub const ESYNR1_DCD_MASK: c_uint = 0x01;
// IDR
pub const NM2VCBMT_MASK: c_uint = 0x1FF;
pub const HTW_MASK: c_uint = 0x01;
pub const HUM_MASK: c_uint = 0x01;
pub const TLBSIZE_MASK: c_uint = 0x0F;
pub const NCB_MASK: c_uint = 0xFF;
pub const NIRPT_MASK: c_uint = 0xFF;
// M2VCBRn
pub const VMID_MASK: c_uint = 0x1F;
pub const CBNDX_MASK: c_uint = 0xFF;
pub const BYPASSD_MASK: c_uint = 0x01;
pub const BPRCOSH_MASK: c_uint = 0x01;
pub const BPRCISH_MASK: c_uint = 0x01;
pub const BPRCNSH_MASK: c_uint = 0x01;
pub const BPSHCFG_MASK: c_uint = 0x03;
pub const NSCFG_MASK: c_uint = 0x03;
pub const BPMTCFG_MASK: c_uint = 0x01;
pub const BPMEMTYPE_MASK: c_uint = 0x07;
// REV
pub const MINOR_MASK: c_uint = 0x0F;
pub const MAJOR_MASK: c_uint = 0x0F;
// TESTBUSCR
pub const TBE_MASK: c_uint = 0x01;
pub const SPDMBE_MASK: c_uint = 0x01;
pub const WGSEL_MASK: c_uint = 0x03;
pub const TBLSEL_MASK: c_uint = 0x03;
pub const TBHSEL_MASK: c_uint = 0x03;
pub const SPDM0SEL_MASK: c_uint = 0x0F;
pub const SPDM1SEL_MASK: c_uint = 0x0F;
pub const SPDM2SEL_MASK: c_uint = 0x0F;
pub const SPDM3SEL_MASK: c_uint = 0x0F;
// TLBIMID
pub const TLBIVMID_VMID_MASK: c_uint = 0x1F;
// TLBRSW
pub const TLBRSW_INDEX_MASK: c_uint = 0xFF;
pub const TLBBFBS_MASK: c_uint = 0x03;
// TLBTR0
pub const PR_MASK: c_uint = 0x01;
pub const PW_MASK: c_uint = 0x01;
pub const UR_MASK: c_uint = 0x01;
pub const UW_MASK: c_uint = 0x01;
pub const XN_MASK: c_uint = 0x01;
pub const NSDESC_MASK: c_uint = 0x01;
pub const ISH_MASK: c_uint = 0x01;
pub const SH_MASK: c_uint = 0x01;
pub const MT_MASK: c_uint = 0x07;
pub const DPSIZR_MASK: c_uint = 0x07;
pub const DPSIZC_MASK: c_uint = 0x07;
// TLBTR1
pub const TLBTR1_VMID_MASK: c_uint = 0x1F;
pub const TLBTR1_PA_MASK: c_uint = 0x000FFFFF;
// TLBTR2
pub const TLBTR2_ASID_MASK: c_uint = 0xFF;
pub const TLBTR2_V_MASK: c_uint = 0x01;
pub const TLBTR2_NSTID_MASK: c_uint = 0x01;
pub const TLBTR2_NV_MASK: c_uint = 0x01;
pub const TLBTR2_VA_MASK: c_uint = 0x000FFFFF;
// Global Register Shifts
// CBACRn
pub const RWVMID_SHIFT: c_int = 0;
pub const RWE_SHIFT: c_int = 8;
pub const RWGE_SHIFT: c_int = 9;
pub const CBVMID_SHIFT: c_int = 16;
pub const IRPTNDX_SHIFT: c_int = 24;
// CR
pub const RPUE_SHIFT: c_int = 0;
pub const RPUERE_SHIFT: c_int = 1;
pub const RPUEIE_SHIFT: c_int = 2;
pub const DCDEE_SHIFT: c_int = 3;
pub const CLIENTPD_SHIFT: c_int = 4;
pub const STALLD_SHIFT: c_int = 5;
pub const TLBLKCRWE_SHIFT: c_int = 6;
pub const CR_TLBIALLCFG_SHIFT: c_int = 7;
pub const TLBIVMIDCFG_SHIFT: c_int = 8;
pub const CR_HUME_SHIFT: c_int = 9;
// ESR
pub const CFG_SHIFT: c_int = 0;
pub const BYPASS_SHIFT: c_int = 1;
pub const ESR_MULTI_SHIFT: c_int = 31;
// ESYNR0
pub const ESYNR0_AMID_SHIFT: c_int = 0;
pub const ESYNR0_APID_SHIFT: c_int = 8;
pub const ESYNR0_ABID_SHIFT: c_int = 13;
pub const ESYNR0_AVMID_SHIFT: c_int = 16;
pub const ESYNR0_ATID_SHIFT: c_int = 24;
// ESYNR1
pub const ESYNR1_AMEMTYPE_SHIFT: c_int = 0;
pub const ESYNR1_ASHARED_SHIFT: c_int = 3;
pub const ESYNR1_AINNERSHARED_SHIFT: c_int = 4;
pub const ESYNR1_APRIV_SHIFT: c_int = 5;
pub const ESYNR1_APROTNS_SHIFT: c_int = 6;
pub const ESYNR1_AINST_SHIFT: c_int = 7;
pub const ESYNR1_AWRITE_SHIFT: c_int = 8;
pub const ESYNR1_ABURST_SHIFT: c_int = 10;
pub const ESYNR1_ALEN_SHIFT: c_int = 12;
pub const ESYNR1_ASIZE_SHIFT: c_int = 16;
pub const ESYNR1_ALOCK_SHIFT: c_int = 20;
pub const ESYNR1_AOOO_SHIFT: c_int = 22;
pub const ESYNR1_AFULL_SHIFT: c_int = 24;
pub const ESYNR1_AC_SHIFT: c_int = 30;
pub const ESYNR1_DCD_SHIFT: c_int = 31;
// IDR
pub const NM2VCBMT_SHIFT: c_int = 0;
pub const HTW_SHIFT: c_int = 9;
pub const HUM_SHIFT: c_int = 10;
pub const TLBSIZE_SHIFT: c_int = 12;
pub const NCB_SHIFT: c_int = 16;
pub const NIRPT_SHIFT: c_int = 24;
// M2VCBRn
pub const VMID_SHIFT: c_int = 0;
pub const CBNDX_SHIFT: c_int = 8;
pub const BYPASSD_SHIFT: c_int = 16;
pub const BPRCOSH_SHIFT: c_int = 17;
pub const BPRCISH_SHIFT: c_int = 18;
pub const BPRCNSH_SHIFT: c_int = 19;
pub const BPSHCFG_SHIFT: c_int = 20;
pub const NSCFG_SHIFT: c_int = 22;
pub const BPMTCFG_SHIFT: c_int = 24;
pub const BPMEMTYPE_SHIFT: c_int = 25;
// REV
pub const MINOR_SHIFT: c_int = 0;
pub const MAJOR_SHIFT: c_int = 4;
// TESTBUSCR
pub const TBE_SHIFT: c_int = 0;
pub const SPDMBE_SHIFT: c_int = 1;
pub const WGSEL_SHIFT: c_int = 8;
pub const TBLSEL_SHIFT: c_int = 12;
pub const TBHSEL_SHIFT: c_int = 14;
pub const SPDM0SEL_SHIFT: c_int = 16;
pub const SPDM1SEL_SHIFT: c_int = 20;
pub const SPDM2SEL_SHIFT: c_int = 24;
pub const SPDM3SEL_SHIFT: c_int = 28;
// TLBIMID
pub const TLBIVMID_VMID_SHIFT: c_int = 0;
// TLBRSW
pub const TLBRSW_INDEX_SHIFT: c_int = 0;
pub const TLBBFBS_SHIFT: c_int = 8;
// TLBTR0
pub const PR_SHIFT: c_int = 0;
pub const PW_SHIFT: c_int = 1;
pub const UR_SHIFT: c_int = 2;
pub const UW_SHIFT: c_int = 3;
pub const XN_SHIFT: c_int = 4;
pub const NSDESC_SHIFT: c_int = 6;
pub const ISH_SHIFT: c_int = 7;
pub const SH_SHIFT: c_int = 8;
pub const MT_SHIFT: c_int = 9;
pub const DPSIZR_SHIFT: c_int = 16;
pub const DPSIZC_SHIFT: c_int = 20;
// TLBTR1
pub const TLBTR1_VMID_SHIFT: c_int = 0;
pub const TLBTR1_PA_SHIFT: c_int = 12;
// TLBTR2
pub const TLBTR2_ASID_SHIFT: c_int = 0;
pub const TLBTR2_V_SHIFT: c_int = 8;
pub const TLBTR2_NSTID_SHIFT: c_int = 9;
pub const TLBTR2_NV_SHIFT: c_int = 10;
pub const TLBTR2_VA_SHIFT: c_int = 12;
// Context Register Masks
// ACTLR
pub const CFERE_MASK: c_uint = 0x01;
pub const CFEIE_MASK: c_uint = 0x01;
pub const PTSHCFG_MASK: c_uint = 0x03;
pub const RCOSH_MASK: c_uint = 0x01;
pub const RCISH_MASK: c_uint = 0x01;
pub const RCNSH_MASK: c_uint = 0x01;
pub const PRIVCFG_MASK: c_uint = 0x03;
pub const DNA_MASK: c_uint = 0x01;
pub const DNLV2PA_MASK: c_uint = 0x01;
pub const TLBMCFG_MASK: c_uint = 0x03;
pub const CFCFG_MASK: c_uint = 0x01;
pub const TIPCF_MASK: c_uint = 0x01;
pub const V2PCFG_MASK: c_uint = 0x03;
pub const HUME_MASK: c_uint = 0x01;
pub const PTMTCFG_MASK: c_uint = 0x01;
pub const PTMEMTYPE_MASK: c_uint = 0x07;
// BFBCR
pub const BFBDFE_MASK: c_uint = 0x01;
pub const BFBSFE_MASK: c_uint = 0x01;
pub const SFVS_MASK: c_uint = 0x01;
pub const FLVIC_MASK: c_uint = 0x0F;
pub const SLVIC_MASK: c_uint = 0x0F;
// CONTEXTIDR
pub const CONTEXTIDR_ASID_MASK: c_uint = 0xFF;
pub const PROCID_MASK: c_uint = 0x00FFFFFF;
// FSR
pub const TF_MASK: c_uint = 0x01;
pub const AFF_MASK: c_uint = 0x01;
pub const APF_MASK: c_uint = 0x01;
pub const TLBMF_MASK: c_uint = 0x01;
pub const HTWDEEF_MASK: c_uint = 0x01;
pub const HTWSEEF_MASK: c_uint = 0x01;
pub const MHF_MASK: c_uint = 0x01;
pub const SL_MASK: c_uint = 0x01;
pub const SS_MASK: c_uint = 0x01;
pub const MULTI_MASK: c_uint = 0x01;
// FSYNR0
pub const AMID_MASK: c_uint = 0xFF;
pub const APID_MASK: c_uint = 0x1F;
pub const ABID_MASK: c_uint = 0x07;
pub const ATID_MASK: c_uint = 0xFF;
// FSYNR1
pub const AMEMTYPE_MASK: c_uint = 0x07;
pub const ASHARED_MASK: c_uint = 0x01;
pub const AINNERSHARED_MASK: c_uint = 0x01;
pub const APRIV_MASK: c_uint = 0x01;
pub const APROTNS_MASK: c_uint = 0x01;
pub const AINST_MASK: c_uint = 0x01;
pub const AWRITE_MASK: c_uint = 0x01;
pub const ABURST_MASK: c_uint = 0x01;
pub const ALEN_MASK: c_uint = 0x0F;
pub const FSYNR1_ASIZE_MASK: c_uint = 0x07;
pub const ALOCK_MASK: c_uint = 0x03;
pub const AFULL_MASK: c_uint = 0x01;
// NMRR
pub const ICPC0_MASK: c_uint = 0x03;
pub const ICPC1_MASK: c_uint = 0x03;
pub const ICPC2_MASK: c_uint = 0x03;
pub const ICPC3_MASK: c_uint = 0x03;
pub const ICPC4_MASK: c_uint = 0x03;
pub const ICPC5_MASK: c_uint = 0x03;
pub const ICPC6_MASK: c_uint = 0x03;
pub const ICPC7_MASK: c_uint = 0x03;
pub const OCPC0_MASK: c_uint = 0x03;
pub const OCPC1_MASK: c_uint = 0x03;
pub const OCPC2_MASK: c_uint = 0x03;
pub const OCPC3_MASK: c_uint = 0x03;
pub const OCPC4_MASK: c_uint = 0x03;
pub const OCPC5_MASK: c_uint = 0x03;
pub const OCPC6_MASK: c_uint = 0x03;
pub const OCPC7_MASK: c_uint = 0x03;
// PAR
pub const FAULT_MASK: c_uint = 0x01;
// If a fault is present, these are the
pub const FAULT_TF_MASK: c_uint = 0x01;
pub const FAULT_AFF_MASK: c_uint = 0x01;
pub const FAULT_APF_MASK: c_uint = 0x01;
pub const FAULT_TLBMF_MASK: c_uint = 0x01;
pub const FAULT_HTWDEEF_MASK: c_uint = 0x01;
pub const FAULT_HTWSEEF_MASK: c_uint = 0x01;
pub const FAULT_MHF_MASK: c_uint = 0x01;
pub const FAULT_SL_MASK: c_uint = 0x01;
pub const FAULT_SS_MASK: c_uint = 0x01;
// If NO fault is present, the following
// fields are in effect
// (FAULT remains as before)
pub const PAR_NOFAULT_SS_MASK: c_uint = 0x01;
pub const PAR_NOFAULT_MT_MASK: c_uint = 0x07;
pub const PAR_NOFAULT_SH_MASK: c_uint = 0x01;
pub const PAR_NOFAULT_NS_MASK: c_uint = 0x01;
pub const PAR_NOFAULT_NOS_MASK: c_uint = 0x01;
pub const PAR_NPFAULT_PA_MASK: c_uint = 0x000FFFFF;
// PRRR
pub const MTC0_MASK: c_uint = 0x03;
pub const MTC1_MASK: c_uint = 0x03;
pub const MTC2_MASK: c_uint = 0x03;
pub const MTC3_MASK: c_uint = 0x03;
pub const MTC4_MASK: c_uint = 0x03;
pub const MTC5_MASK: c_uint = 0x03;
pub const MTC6_MASK: c_uint = 0x03;
pub const MTC7_MASK: c_uint = 0x03;
pub const SHDSH0_MASK: c_uint = 0x01;
pub const SHDSH1_MASK: c_uint = 0x01;
pub const SHNMSH0_MASK: c_uint = 0x01;
pub const SHNMSH1_MASK: c_uint = 0x01;
pub const NOS0_MASK: c_uint = 0x01;
pub const NOS1_MASK: c_uint = 0x01;
pub const NOS2_MASK: c_uint = 0x01;
pub const NOS3_MASK: c_uint = 0x01;
pub const NOS4_MASK: c_uint = 0x01;
pub const NOS5_MASK: c_uint = 0x01;
pub const NOS6_MASK: c_uint = 0x01;
pub const NOS7_MASK: c_uint = 0x01;
// RESUME
pub const TNR_MASK: c_uint = 0x01;
// SCTLR
pub const M_MASK: c_uint = 0x01;
pub const TRE_MASK: c_uint = 0x01;
pub const AFE_MASK: c_uint = 0x01;
pub const HAF_MASK: c_uint = 0x01;
pub const BE_MASK: c_uint = 0x01;
pub const AFFD_MASK: c_uint = 0x01;
// TLBIASID
pub const TLBIASID_ASID_MASK: c_uint = 0xFF;
// TLBIVA
pub const TLBIVA_ASID_MASK: c_uint = 0xFF;
pub const TLBIVA_VA_MASK: c_uint = 0x000FFFFF;
// TLBIVAA
pub const TLBIVAA_VA_MASK: c_uint = 0x000FFFFF;
// TLBLCKR
pub const LKE_MASK: c_uint = 0x01;
pub const TLBLCKR_TLBIALLCFG_MASK: c_uint = 0x01;
pub const TLBIASIDCFG_MASK: c_uint = 0x01;
pub const TLBIVAACFG_MASK: c_uint = 0x01;
pub const FLOOR_MASK: c_uint = 0xFF;
pub const VICTIM_MASK: c_uint = 0xFF;
// TTBCR
pub const N_MASK: c_uint = 0x07;
pub const PD0_MASK: c_uint = 0x01;
pub const PD1_MASK: c_uint = 0x01;
// TTBR0
pub const TTBR0_IRGNH_MASK: c_uint = 0x01;
pub const TTBR0_SH_MASK: c_uint = 0x01;
pub const TTBR0_ORGN_MASK: c_uint = 0x03;
pub const TTBR0_NOS_MASK: c_uint = 0x01;
pub const TTBR0_IRGNL_MASK: c_uint = 0x01;
pub const TTBR0_PA_MASK: c_uint = 0x0003FFFF;
// TTBR1
pub const TTBR1_IRGNH_MASK: c_uint = 0x01;
pub const TTBR1_SH_MASK: c_uint = 0x01;
pub const TTBR1_ORGN_MASK: c_uint = 0x03;
pub const TTBR1_NOS_MASK: c_uint = 0x01;
pub const TTBR1_IRGNL_MASK: c_uint = 0x01;
pub const TTBR1_PA_MASK: c_uint = 0x0003FFFF;
// V2PSR
pub const HIT_MASK: c_uint = 0x01;
pub const INDEX_MASK: c_uint = 0xFF;
// V2Pxx
pub const V2Pxx_INDEX_MASK: c_uint = 0xFF;
pub const V2Pxx_VA_MASK: c_uint = 0x000FFFFF;
// Context Register Shifts
// ACTLR
pub const CFERE_SHIFT: c_int = 0;
pub const CFEIE_SHIFT: c_int = 1;
pub const PTSHCFG_SHIFT: c_int = 2;
pub const RCOSH_SHIFT: c_int = 4;
pub const RCISH_SHIFT: c_int = 5;
pub const RCNSH_SHIFT: c_int = 6;
pub const PRIVCFG_SHIFT: c_int = 8;
pub const DNA_SHIFT: c_int = 10;
pub const DNLV2PA_SHIFT: c_int = 11;
pub const TLBMCFG_SHIFT: c_int = 12;
pub const CFCFG_SHIFT: c_int = 14;
pub const TIPCF_SHIFT: c_int = 15;
pub const V2PCFG_SHIFT: c_int = 16;
pub const HUME_SHIFT: c_int = 18;
pub const PTMTCFG_SHIFT: c_int = 20;
pub const PTMEMTYPE_SHIFT: c_int = 21;
// BFBCR
pub const BFBDFE_SHIFT: c_int = 0;
pub const BFBSFE_SHIFT: c_int = 1;
pub const SFVS_SHIFT: c_int = 2;
pub const FLVIC_SHIFT: c_int = 4;
pub const SLVIC_SHIFT: c_int = 8;
// CONTEXTIDR
pub const CONTEXTIDR_ASID_SHIFT: c_int = 0;
pub const PROCID_SHIFT: c_int = 8;
// FSR
pub const TF_SHIFT: c_int = 1;
pub const AFF_SHIFT: c_int = 2;
pub const APF_SHIFT: c_int = 3;
pub const TLBMF_SHIFT: c_int = 4;
pub const HTWDEEF_SHIFT: c_int = 5;
pub const HTWSEEF_SHIFT: c_int = 6;
pub const MHF_SHIFT: c_int = 7;
pub const SL_SHIFT: c_int = 16;
pub const SS_SHIFT: c_int = 30;
pub const MULTI_SHIFT: c_int = 31;
// FSYNR0
pub const AMID_SHIFT: c_int = 0;
pub const APID_SHIFT: c_int = 8;
pub const ABID_SHIFT: c_int = 13;
pub const ATID_SHIFT: c_int = 24;
// FSYNR1
pub const AMEMTYPE_SHIFT: c_int = 0;
pub const ASHARED_SHIFT: c_int = 3;
pub const AINNERSHARED_SHIFT: c_int = 4;
pub const APRIV_SHIFT: c_int = 5;
pub const APROTNS_SHIFT: c_int = 6;
pub const AINST_SHIFT: c_int = 7;
pub const AWRITE_SHIFT: c_int = 8;
pub const ABURST_SHIFT: c_int = 10;
pub const ALEN_SHIFT: c_int = 12;
pub const FSYNR1_ASIZE_SHIFT: c_int = 16;
pub const ALOCK_SHIFT: c_int = 20;
pub const AFULL_SHIFT: c_int = 24;
// NMRR
pub const ICPC0_SHIFT: c_int = 0;
pub const ICPC1_SHIFT: c_int = 2;
pub const ICPC2_SHIFT: c_int = 4;
pub const ICPC3_SHIFT: c_int = 6;
pub const ICPC4_SHIFT: c_int = 8;
pub const ICPC5_SHIFT: c_int = 10;
pub const ICPC6_SHIFT: c_int = 12;
pub const ICPC7_SHIFT: c_int = 14;
pub const OCPC0_SHIFT: c_int = 16;
pub const OCPC1_SHIFT: c_int = 18;
pub const OCPC2_SHIFT: c_int = 20;
pub const OCPC3_SHIFT: c_int = 22;
pub const OCPC4_SHIFT: c_int = 24;
pub const OCPC5_SHIFT: c_int = 26;
pub const OCPC6_SHIFT: c_int = 28;
pub const OCPC7_SHIFT: c_int = 30;
// PAR
pub const FAULT_SHIFT: c_int = 0;
// If a fault is present, these are the
pub const FAULT_TF_SHIFT: c_int = 1;
pub const FAULT_AFF_SHIFT: c_int = 2;
pub const FAULT_APF_SHIFT: c_int = 3;
pub const FAULT_TLBMF_SHIFT: c_int = 4;
pub const FAULT_HTWDEEF_SHIFT: c_int = 5;
pub const FAULT_HTWSEEF_SHIFT: c_int = 6;
pub const FAULT_MHF_SHIFT: c_int = 7;
pub const FAULT_SL_SHIFT: c_int = 16;
pub const FAULT_SS_SHIFT: c_int = 30;
// If NO fault is present, the following
// fields are in effect
// (FAULT remains as before)
pub const PAR_NOFAULT_SS_SHIFT: c_int = 1;
pub const PAR_NOFAULT_MT_SHIFT: c_int = 4;
pub const PAR_NOFAULT_SH_SHIFT: c_int = 7;
pub const PAR_NOFAULT_NS_SHIFT: c_int = 9;
pub const PAR_NOFAULT_NOS_SHIFT: c_int = 10;
pub const PAR_NPFAULT_PA_SHIFT: c_int = 12;
// PRRR
pub const MTC0_SHIFT: c_int = 0;
pub const MTC1_SHIFT: c_int = 2;
pub const MTC2_SHIFT: c_int = 4;
pub const MTC3_SHIFT: c_int = 6;
pub const MTC4_SHIFT: c_int = 8;
pub const MTC5_SHIFT: c_int = 10;
pub const MTC6_SHIFT: c_int = 12;
pub const MTC7_SHIFT: c_int = 14;
pub const SHDSH0_SHIFT: c_int = 16;
pub const SHDSH1_SHIFT: c_int = 17;
pub const SHNMSH0_SHIFT: c_int = 18;
pub const SHNMSH1_SHIFT: c_int = 19;
pub const NOS0_SHIFT: c_int = 24;
pub const NOS1_SHIFT: c_int = 25;
pub const NOS2_SHIFT: c_int = 26;
pub const NOS3_SHIFT: c_int = 27;
pub const NOS4_SHIFT: c_int = 28;
pub const NOS5_SHIFT: c_int = 29;
pub const NOS6_SHIFT: c_int = 30;
pub const NOS7_SHIFT: c_int = 31;
// RESUME
pub const TNR_SHIFT: c_int = 0;
// SCTLR
pub const M_SHIFT: c_int = 0;
pub const TRE_SHIFT: c_int = 1;
pub const AFE_SHIFT: c_int = 2;
pub const HAF_SHIFT: c_int = 3;
pub const BE_SHIFT: c_int = 4;
pub const AFFD_SHIFT: c_int = 5;
// TLBIASID
pub const TLBIASID_ASID_SHIFT: c_int = 0;
// TLBIVA
pub const TLBIVA_ASID_SHIFT: c_int = 0;
pub const TLBIVA_VA_SHIFT: c_int = 12;
// TLBIVAA
pub const TLBIVAA_VA_SHIFT: c_int = 12;
// TLBLCKR
pub const LKE_SHIFT: c_int = 0;
pub const TLBLCKR_TLBIALLCFG_SHIFT: c_int = 1;
pub const TLBIASIDCFG_SHIFT: c_int = 2;
pub const TLBIVAACFG_SHIFT: c_int = 3;
pub const FLOOR_SHIFT: c_int = 8;
pub const VICTIM_SHIFT: c_int = 8;
// TTBCR
pub const N_SHIFT: c_int = 3;
pub const PD0_SHIFT: c_int = 4;
pub const PD1_SHIFT: c_int = 5;
// TTBR0
pub const TTBR0_IRGNH_SHIFT: c_int = 0;
pub const TTBR0_SH_SHIFT: c_int = 1;
pub const TTBR0_ORGN_SHIFT: c_int = 3;
pub const TTBR0_NOS_SHIFT: c_int = 5;
pub const TTBR0_IRGNL_SHIFT: c_int = 6;
pub const TTBR0_PA_SHIFT: c_int = 14;
// TTBR1
pub const TTBR1_IRGNH_SHIFT: c_int = 0;
pub const TTBR1_SH_SHIFT: c_int = 1;
pub const TTBR1_ORGN_SHIFT: c_int = 3;
pub const TTBR1_NOS_SHIFT: c_int = 5;
pub const TTBR1_IRGNL_SHIFT: c_int = 6;
pub const TTBR1_PA_SHIFT: c_int = 14;
// V2PSR
pub const HIT_SHIFT: c_int = 0;
pub const INDEX_SHIFT: c_int = 8;
// V2Pxx
pub const V2Pxx_INDEX_SHIFT: c_int = 0;
pub const V2Pxx_VA_SHIFT: c_int = 12;
