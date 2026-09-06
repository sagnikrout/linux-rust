//! Automatically rewritten from C Header to Rust Module
//! Source: arch/loongarch/include/asm/loongarch.h
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
// Copyright (C) 2020-2022 Loongson Technology Corporation Limited
//

// CPUCFG

// LoongArch Registers
pub const REG_ZERO: c_uint = 0x0;
pub const REG_RA: c_uint = 0x1;
pub const REG_TP: c_uint = 0x2;
pub const REG_SP: c_uint = 0x3;
pub const REG_A0: c_uint = 0x4 /* Reused as V0 for return value */;
pub const REG_A1: c_uint = 0x5 /* Reused as V1 for return value */;
pub const REG_A2: c_uint = 0x6;
pub const REG_A3: c_uint = 0x7;
pub const REG_A4: c_uint = 0x8;
pub const REG_A5: c_uint = 0x9;
pub const REG_A6: c_uint = 0xa;
pub const REG_A7: c_uint = 0xb;
pub const REG_T0: c_uint = 0xc;
pub const REG_T1: c_uint = 0xd;
pub const REG_T2: c_uint = 0xe;
pub const REG_T3: c_uint = 0xf;
pub const REG_T4: c_uint = 0x10;
pub const REG_T5: c_uint = 0x11;
pub const REG_T6: c_uint = 0x12;
pub const REG_T7: c_uint = 0x13;
pub const REG_T8: c_uint = 0x14;
pub const REG_U0: c_uint = 0x15 /* Kernel uses it as percpu base */;
pub const REG_FP: c_uint = 0x16;
pub const REG_S0: c_uint = 0x17;
pub const REG_S1: c_uint = 0x18;
pub const REG_S2: c_uint = 0x19;
pub const REG_S3: c_uint = 0x1a;
pub const REG_S4: c_uint = 0x1b;
pub const REG_S5: c_uint = 0x1c;
pub const REG_S6: c_uint = 0x1d;
pub const REG_S7: c_uint = 0x1e;
pub const REG_S8: c_uint = 0x1f;

// Bit fields for CPUCFG registers
pub const LOONGARCH_CPUCFG0: c_uint = 0x0;

pub const LOONGARCH_CPUCFG1: c_uint = 0x1;

pub const LOONGARCH_CPUCFG2: c_uint = 0x2;

pub const LOONGARCH_CPUCFG3: c_uint = 0x3;

pub const LOONGARCH_CPUCFG4: c_uint = 0x4;

pub const LOONGARCH_CPUCFG5: c_uint = 0x5;

pub const LOONGARCH_CPUCFG6: c_uint = 0x6;

pub const CPUCFG6_PMNUM_SHIFT: c_int = 4;

pub const CPUCFG6_PMBITS_SHIFT: c_int = 8;

pub const LOONGARCH_CPUCFG16: c_uint = 0x10;

pub const LOONGARCH_CPUCFG17: c_uint = 0x11;
pub const LOONGARCH_CPUCFG18: c_uint = 0x12;
pub const LOONGARCH_CPUCFG19: c_uint = 0x13;
pub const LOONGARCH_CPUCFG20: c_uint = 0x14;

pub const CPUCFG_CACHE_WAYS: c_int = 0;
pub const CPUCFG_CACHE_SETS: c_int = 16;
pub const CPUCFG_CACHE_LSIZE: c_int = 24;
pub const LOONGARCH_CPUCFG48: c_uint = 0x30;

//
// CPUCFG index area: 0x40000000 -- 0x400000ff
// SW emulation for KVM hypervirsor, see arch/loongarch/include/uapi/asm/kvm_para.h
//
// CSR

// IOCSR

// CSR register number
// Basic CSR registers
pub const LOONGARCH_CSR_CRMD: c_uint = 0x0	/* Current mode info */;
pub const CSR_CRMD_WE_SHIFT: c_int = 9;

pub const CSR_CRMD_DACM_SHIFT: c_int = 7;
pub const CSR_CRMD_DACM_WIDTH: c_int = 2;

pub const CSR_CRMD_DACF_SHIFT: c_int = 5;
pub const CSR_CRMD_DACF_WIDTH: c_int = 2;

pub const CSR_CRMD_PG_SHIFT: c_int = 4;

pub const CSR_CRMD_DA_SHIFT: c_int = 3;

pub const CSR_CRMD_IE_SHIFT: c_int = 2;

pub const CSR_CRMD_PLV_SHIFT: c_int = 0;
pub const CSR_CRMD_PLV_WIDTH: c_int = 2;

pub const PLV_KERN: c_int = 0;
pub const PLV_USER: c_int = 3;
pub const PLV_MASK: c_uint = 0x3;
pub const LOONGARCH_CSR_PRMD: c_uint = 0x1	/* Prev-exception mode info */;
pub const CSR_PRMD_PWE_SHIFT: c_int = 3;

pub const CSR_PRMD_PIE_SHIFT: c_int = 2;

pub const CSR_PRMD_PPLV_SHIFT: c_int = 0;
pub const CSR_PRMD_PPLV_WIDTH: c_int = 2;

pub const LOONGARCH_CSR_EUEN: c_uint = 0x2	/* Extended unit enable */;
pub const CSR_EUEN_LBTEN_SHIFT: c_int = 3;

pub const CSR_EUEN_LASXEN_SHIFT: c_int = 2;

pub const CSR_EUEN_LSXEN_SHIFT: c_int = 1;

pub const CSR_EUEN_FPEN_SHIFT: c_int = 0;

pub const LOONGARCH_CSR_MISC: c_uint = 0x3	/* Misc config */;
pub const LOONGARCH_CSR_ECFG: c_uint = 0x4	/* Exception config */;
pub const CSR_ECFG_VS_SHIFT: c_int = 16;
pub const CSR_ECFG_VS_WIDTH: c_int = 3;

pub const CSR_ECFG_IM_SHIFT: c_int = 0;
pub const CSR_ECFG_IM_WIDTH: c_int = 14;

pub const LOONGARCH_CSR_ESTAT: c_uint = 0x5	/* Exception status */;
pub const CSR_ESTAT_ESUBCODE_SHIFT: c_int = 22;
pub const CSR_ESTAT_ESUBCODE_WIDTH: c_int = 9;

pub const CSR_ESTAT_EXC_SHIFT: c_int = 16;
pub const CSR_ESTAT_EXC_WIDTH: c_int = 6;

pub const CSR_ESTAT_IS_SHIFT: c_int = 0;
pub const CSR_ESTAT_IS_WIDTH: c_int = 15;

pub const LOONGARCH_CSR_ERA: c_uint = 0x6	/* Exception return address */;
pub const LOONGARCH_CSR_BADV: c_uint = 0x7	/* Bad virtual address */;
pub const LOONGARCH_CSR_BADI: c_uint = 0x8	/* Bad instruction */;
pub const LOONGARCH_CSR_EENTRY: c_uint = 0xc	/* Exception entry */;
// TLB related CSR registers
pub const LOONGARCH_CSR_TLBIDX: c_uint = 0x10	/* TLB Index, EHINV, PageSize, NP */;
pub const CSR_TLBIDX_EHINV_SHIFT: c_int = 31;

pub const CSR_TLBIDX_PS_SHIFT: c_int = 24;
pub const CSR_TLBIDX_PS_WIDTH: c_int = 6;

pub const CSR_TLBIDX_IDX_SHIFT: c_int = 0;
pub const CSR_TLBIDX_IDX_WIDTH: c_int = 12;

pub const CSR_TLBIDX_SIZEM: c_uint = 0x3f000000;

pub const CSR_TLBIDX_IDXM: c_uint = 0xfff;

pub const LOONGARCH_CSR_TLBEHI: c_uint = 0x11	/* TLB EntryHi */;
pub const LOONGARCH_CSR_TLBELO0: c_uint = 0x12	/* TLB EntryLo0 */;
pub const CSR_TLBLO0_RPLV_SHIFT: c_int = 63;

pub const CSR_TLBLO0_NX_SHIFT: c_int = 62;

pub const CSR_TLBLO0_NR_SHIFT: c_int = 61;

pub const CSR_TLBLO0_PFN_SHIFT: c_int = 12;
pub const CSR_TLBLO0_PFN_WIDTH: c_int = 36;

pub const CSR_TLBLO0_GLOBAL_SHIFT: c_int = 6;

pub const CSR_TLBLO0_CCA_SHIFT: c_int = 4;
pub const CSR_TLBLO0_CCA_WIDTH: c_int = 2;

pub const CSR_TLBLO0_PLV_SHIFT: c_int = 2;
pub const CSR_TLBLO0_PLV_WIDTH: c_int = 2;

pub const CSR_TLBLO0_WE_SHIFT: c_int = 1;

pub const CSR_TLBLO0_V_SHIFT: c_int = 0;

pub const LOONGARCH_CSR_TLBELO1: c_uint = 0x13	/* TLB EntryLo1 */;
pub const CSR_TLBLO1_RPLV_SHIFT: c_int = 63;

pub const CSR_TLBLO1_NX_SHIFT: c_int = 62;

pub const CSR_TLBLO1_NR_SHIFT: c_int = 61;

pub const CSR_TLBLO1_PFN_SHIFT: c_int = 12;
pub const CSR_TLBLO1_PFN_WIDTH: c_int = 36;

pub const CSR_TLBLO1_GLOBAL_SHIFT: c_int = 6;

pub const CSR_TLBLO1_CCA_SHIFT: c_int = 4;
pub const CSR_TLBLO1_CCA_WIDTH: c_int = 2;

pub const CSR_TLBLO1_PLV_SHIFT: c_int = 2;
pub const CSR_TLBLO1_PLV_WIDTH: c_int = 2;

pub const CSR_TLBLO1_WE_SHIFT: c_int = 1;

pub const CSR_TLBLO1_V_SHIFT: c_int = 0;

pub const LOONGARCH_CSR_GTLBC: c_uint = 0x15	/* Guest TLB control */;
pub const CSR_GTLBC_TGID_SHIFT: c_int = 16;
pub const CSR_GTLBC_TGID_WIDTH: c_int = 8;

pub const CSR_GTLBC_TOTI_SHIFT: c_int = 13;

pub const CSR_GTLBC_USETGID_SHIFT: c_int = 12;

pub const CSR_GTLBC_GMTLBSZ_SHIFT: c_int = 0;
pub const CSR_GTLBC_GMTLBSZ_WIDTH: c_int = 6;

pub const LOONGARCH_CSR_TRGP: c_uint = 0x16	/* TLBR read guest info */;
pub const CSR_TRGP_RID_SHIFT: c_int = 16;
pub const CSR_TRGP_RID_WIDTH: c_int = 8;

pub const CSR_TRGP_GTLB_SHIFT: c_int = 0;

pub const LOONGARCH_CSR_ASID: c_uint = 0x18	/* ASID */;

pub const CSR_ASID_BIT_WIDTH: c_int = 8;

pub const CSR_ASID_ASID_SHIFT: c_int = 0;
pub const CSR_ASID_ASID_WIDTH: c_int = 10;

pub const LOONGARCH_CSR_PGDL: c_uint = 0x19	/* Page table base address when VA[VALEN-1] = 0 */;
pub const LOONGARCH_CSR_PGDH: c_uint = 0x1a	/* Page table base address when VA[VALEN-1] = 1 */;
pub const LOONGARCH_CSR_PGD: c_uint = 0x1b	/* Page table base */;
pub const LOONGARCH_CSR_PWCTL0: c_uint = 0x1c	/* PWCtl0 */;
pub const CSR_PWCTL0_PTEW_SHIFT: c_int = 30;
pub const CSR_PWCTL0_PTEW_WIDTH: c_int = 2;

pub const CSR_PWCTL0_DIR1WIDTH_SHIFT: c_int = 25;
pub const CSR_PWCTL0_DIR1WIDTH_WIDTH: c_int = 5;

pub const CSR_PWCTL0_DIR1BASE_SHIFT: c_int = 20;
pub const CSR_PWCTL0_DIR1BASE_WIDTH: c_int = 5;

pub const CSR_PWCTL0_DIR0WIDTH_SHIFT: c_int = 15;
pub const CSR_PWCTL0_DIR0WIDTH_WIDTH: c_int = 5;

pub const CSR_PWCTL0_DIR0BASE_SHIFT: c_int = 10;
pub const CSR_PWCTL0_DIR0BASE_WIDTH: c_int = 5;

pub const CSR_PWCTL0_PTWIDTH_SHIFT: c_int = 5;
pub const CSR_PWCTL0_PTWIDTH_WIDTH: c_int = 5;

pub const CSR_PWCTL0_PTBASE_SHIFT: c_int = 0;
pub const CSR_PWCTL0_PTBASE_WIDTH: c_int = 5;

pub const LOONGARCH_CSR_PWCTL1: c_uint = 0x1d	/* PWCtl1 */;
pub const CSR_PWCTL1_PTW_SHIFT: c_int = 24;
pub const CSR_PWCTL1_PTW_WIDTH: c_int = 1;

pub const CSR_PWCTL1_DIR3WIDTH_SHIFT: c_int = 18;
pub const CSR_PWCTL1_DIR3WIDTH_WIDTH: c_int = 5;

pub const CSR_PWCTL1_DIR3BASE_SHIFT: c_int = 12;
pub const CSR_PWCTL1_DIR3BASE_WIDTH: c_int = 5;

pub const CSR_PWCTL1_DIR2WIDTH_SHIFT: c_int = 6;
pub const CSR_PWCTL1_DIR2WIDTH_WIDTH: c_int = 5;

pub const CSR_PWCTL1_DIR2BASE_SHIFT: c_int = 0;
pub const CSR_PWCTL1_DIR2BASE_WIDTH: c_int = 5;

pub const LOONGARCH_CSR_STLBPGSIZE: c_uint = 0x1e;
pub const CSR_STLBPGSIZE_PS_WIDTH: c_int = 6;

pub const LOONGARCH_CSR_RVACFG: c_uint = 0x1f;
pub const CSR_RVACFG_RDVA_WIDTH: c_int = 4;

// Config CSR registers
pub const LOONGARCH_CSR_CPUID: c_uint = 0x20	/* CPU core id */;
pub const CSR_CPUID_COREID_WIDTH: c_int = 11;

pub const LOONGARCH_CSR_PRCFG1: c_uint = 0x21	/* Config1 */;
pub const CSR_CONF1_VSMAX_SHIFT: c_int = 12;
pub const CSR_CONF1_VSMAX_WIDTH: c_int = 3;

pub const CSR_CONF1_TMRBITS_SHIFT: c_int = 4;
pub const CSR_CONF1_TMRBITS_WIDTH: c_int = 8;

pub const CSR_CONF1_KSNUM_WIDTH: c_int = 4;

pub const LOONGARCH_CSR_PRCFG2: c_uint = 0x22	/* Config2 */;
pub const CSR_CONF2_PGMASK_SUPP: c_uint = 0x3ffff000;
pub const LOONGARCH_CSR_PRCFG3: c_uint = 0x23	/* Config3 */;
pub const CSR_CONF3_STLBIDX_SHIFT: c_int = 20;
pub const CSR_CONF3_STLBIDX_WIDTH: c_int = 6;

pub const CSR_CONF3_STLBWAYS_SHIFT: c_int = 12;
pub const CSR_CONF3_STLBWAYS_WIDTH: c_int = 8;

pub const CSR_CONF3_MTLBSIZE_SHIFT: c_int = 4;
pub const CSR_CONF3_MTLBSIZE_WIDTH: c_int = 8;

pub const CSR_CONF3_TLBTYPE_SHIFT: c_int = 0;
pub const CSR_CONF3_TLBTYPE_WIDTH: c_int = 4;

// KSave registers
pub const LOONGARCH_CSR_KS0: c_uint = 0x30;
pub const LOONGARCH_CSR_KS1: c_uint = 0x31;
pub const LOONGARCH_CSR_KS2: c_uint = 0x32;
pub const LOONGARCH_CSR_KS3: c_uint = 0x33;
pub const LOONGARCH_CSR_KS4: c_uint = 0x34;
pub const LOONGARCH_CSR_KS5: c_uint = 0x35;
pub const LOONGARCH_CSR_KS6: c_uint = 0x36;
pub const LOONGARCH_CSR_KS7: c_uint = 0x37;
pub const LOONGARCH_CSR_KS8: c_uint = 0x38;
pub const LOONGARCH_CSR_KS9: c_uint = 0x39;
pub const LOONGARCH_CSR_KS10: c_uint = 0x3a;
pub const LOONGARCH_CSR_KS11: c_uint = 0x3b;
pub const LOONGARCH_CSR_KS12: c_uint = 0x3c;
pub const LOONGARCH_CSR_KS13: c_uint = 0x3d;
pub const LOONGARCH_CSR_KS14: c_uint = 0x3e;
pub const LOONGARCH_CSR_KS15: c_uint = 0x3f;
// Exception allocated KS0, KS1 and KS2 statically

// Percpu-data base allocated KS3 statically

// KVM allocated KS4 and KS5 statically

// Timer registers
pub const LOONGARCH_CSR_TMID: c_uint = 0x40	/* Timer ID */;
pub const LOONGARCH_CSR_TCFG: c_uint = 0x41	/* Timer config */;
pub const CSR_TCFG_VAL_SHIFT: c_int = 2;

pub const CSR_TCFG_PERIOD_SHIFT: c_int = 1;

pub const LOONGARCH_CSR_TVAL: c_uint = 0x42	/* Timer value */;
pub const LOONGARCH_CSR_CNTC: c_uint = 0x43	/* Timer offset */;
pub const LOONGARCH_CSR_TINTCLR: c_uint = 0x44	/* Timer interrupt clear */;
pub const CSR_TINTCLR_TI_SHIFT: c_int = 0;

// Guest registers
pub const LOONGARCH_CSR_GSTAT: c_uint = 0x50	/* Guest status */;
pub const CSR_GSTAT_GID_SHIFT: c_int = 16;
pub const CSR_GSTAT_GID_WIDTH: c_int = 8;

pub const CSR_GSTAT_GIDBIT_SHIFT: c_int = 4;
pub const CSR_GSTAT_GIDBIT_WIDTH: c_int = 6;

pub const CSR_GSTAT_PVM_SHIFT: c_int = 1;

pub const CSR_GSTAT_VM_SHIFT: c_int = 0;

pub const LOONGARCH_CSR_GCFG: c_uint = 0x51	/* Guest config */;
pub const CSR_GCFG_GPERF_SHIFT: c_int = 24;
pub const CSR_GCFG_GPERF_WIDTH: c_int = 3;

pub const CSR_GCFG_GPMP_SHIFT: c_int = 23;

pub const CSR_GCFG_GCI_SHIFT: c_int = 20;
pub const CSR_GCFG_GCI_WIDTH: c_int = 2;

pub const CSR_GCFG_GCIP_SHIFT: c_int = 16;

pub const CSR_GCFG_TORU_SHIFT: c_int = 15;

pub const CSR_GCFG_TORUP_SHIFT: c_int = 14;

pub const CSR_GCFG_TOP_SHIFT: c_int = 13;

pub const CSR_GCFG_TOPP_SHIFT: c_int = 12;

pub const CSR_GCFG_TOE_SHIFT: c_int = 11;

pub const CSR_GCFG_TOEP_SHIFT: c_int = 10;

pub const CSR_GCFG_TIT_SHIFT: c_int = 9;

pub const CSR_GCFG_TITP_SHIFT: c_int = 8;

pub const CSR_GCFG_SIT_SHIFT: c_int = 7;

pub const CSR_GCFG_SITP_SHIFT: c_int = 6;

pub const CSR_GCFG_MATC_SHITF: c_int = 4;
pub const CSR_GCFG_MATC_WIDTH: c_int = 2;

pub const CSR_GCFG_MATP_NEST_SHIFT: c_int = 2;

pub const CSR_GCFG_MATP_ROOT_SHIFT: c_int = 1;

pub const CSR_GCFG_MATP_GUEST_SHIFT: c_int = 0;

pub const LOONGARCH_CSR_GINTC: c_uint = 0x52	/* Guest interrupt control */;
pub const CSR_GINTC_HC_SHIFT: c_int = 16;
pub const CSR_GINTC_HC_WIDTH: c_int = 8;

pub const CSR_GINTC_PIP_SHIFT: c_int = 8;
pub const CSR_GINTC_PIP_WIDTH: c_int = 8;

pub const CSR_GINTC_VIP_SHIFT: c_int = 0;
pub const CSR_GINTC_VIP_WIDTH: c_int = 8;

pub const LOONGARCH_CSR_GCNTC: c_uint = 0x53	/* Guest timer offset */;
// LLBCTL register
pub const LOONGARCH_CSR_LLBCTL: c_uint = 0x60	/* LLBit control */;
pub const CSR_LLBCTL_ROLLB_SHIFT: c_int = 0;

pub const CSR_LLBCTL_WCLLB_SHIFT: c_int = 1;

pub const CSR_LLBCTL_KLO_SHIFT: c_int = 2;

// Implement dependent
pub const LOONGARCH_CSR_IMPCTL1: c_uint = 0x80	/* Loongson config1 */;
pub const CSR_LDSTORDER_SHIFT: c_int = 28;
pub const CSR_LDSTORDER_WIDTH: c_int = 3;

pub const CSR_MISPEC_SHIFT: c_int = 20;
pub const CSR_MISPEC_WIDTH: c_int = 8;

pub const CSR_SSEN_SHIFT: c_int = 18;

pub const CSR_SCRAND_SHIFT: c_int = 17;

pub const CSR_LLEXCL_SHIFT: c_int = 16;

pub const CSR_DISVC_SHIFT: c_int = 15;

pub const CSR_VCLRU_SHIFT: c_int = 14;

pub const CSR_DCLRU_SHIFT: c_int = 13;

pub const CSR_FASTLDQ_SHIFT: c_int = 12;

pub const CSR_USERCAC_SHIFT: c_int = 11;

pub const CSR_ANTI_MISPEC_SHIFT: c_int = 10;

pub const CSR_AUTO_FLUSHSFB_SHIFT: c_int = 9;

pub const CSR_STFILL_SHIFT: c_int = 8;

pub const CSR_LIFEP_SHIFT: c_int = 7;

pub const CSR_LLSYNC_SHIFT: c_int = 6;

pub const CSR_BRBTDIS_SHIFT: c_int = 5;

pub const CSR_RASDIS_SHIFT: c_int = 4;

pub const CSR_STPRE_SHIFT: c_int = 2;
pub const CSR_STPRE_WIDTH: c_int = 2;

pub const CSR_INSTPRE_SHIFT: c_int = 1;

pub const CSR_DATAPRE_SHIFT: c_int = 0;

pub const LOONGARCH_CSR_IMPCTL2: c_uint = 0x81	/* Loongson config2 */;
pub const CSR_FLUSH_MTLB_SHIFT: c_int = 0;

pub const CSR_FLUSH_STLB_SHIFT: c_int = 1;

pub const CSR_FLUSH_DTLB_SHIFT: c_int = 2;

pub const CSR_FLUSH_ITLB_SHIFT: c_int = 3;

pub const CSR_FLUSH_BTAC_SHIFT: c_int = 4;

pub const LOONGARCH_CSR_GNMI: c_uint = 0x82;
// TLB Refill registers
pub const LOONGARCH_CSR_TLBRENTRY: c_uint = 0x88	/* TLB refill exception entry */;
pub const LOONGARCH_CSR_TLBRBADV: c_uint = 0x89	/* TLB refill badvaddr */;
pub const LOONGARCH_CSR_TLBRERA: c_uint = 0x8a	/* TLB refill ERA */;
pub const LOONGARCH_CSR_TLBRSAVE: c_uint = 0x8b	/* KSave for TLB refill exception */;
pub const LOONGARCH_CSR_TLBRELO0: c_uint = 0x8c	/* TLB refill entrylo0 */;
pub const LOONGARCH_CSR_TLBRELO1: c_uint = 0x8d	/* TLB refill entrylo1 */;
pub const LOONGARCH_CSR_TLBREHI: c_uint = 0x8e	/* TLB refill entryhi */;
pub const CSR_TLBREHI_PS_SHIFT: c_int = 0;

pub const LOONGARCH_CSR_TLBRPRMD: c_uint = 0x8f	/* TLB refill mode info */;
// Machine Error registers
pub const LOONGARCH_CSR_MERRCTL: c_uint = 0x90	/* MERRCTL */;
pub const LOONGARCH_CSR_MERRINFO1: c_uint = 0x91	/* MError info1 */;
pub const LOONGARCH_CSR_MERRINFO2: c_uint = 0x92	/* MError info2 */;
pub const LOONGARCH_CSR_MERRENTRY: c_uint = 0x93	/* MError exception entry */;
pub const LOONGARCH_CSR_MERRERA: c_uint = 0x94	/* MError exception ERA */;
pub const LOONGARCH_CSR_MERRSAVE: c_uint = 0x95	/* KSave for machine error exception */;
pub const LOONGARCH_CSR_CTAG: c_uint = 0x98	/* TagLo + TagHi */;
pub const LOONGARCH_CSR_ISR0: c_uint = 0xa0;
pub const LOONGARCH_CSR_ISR1: c_uint = 0xa1;
pub const LOONGARCH_CSR_ISR2: c_uint = 0xa2;
pub const LOONGARCH_CSR_ISR3: c_uint = 0xa3;
pub const LOONGARCH_CSR_IRR: c_uint = 0xa4;
pub const LOONGARCH_CSR_IPR: c_uint = 0xa5;
pub const LOONGARCH_CSR_PRID: c_uint = 0xc0;
// Shadow MCSR : 0xc0 ~ 0xff
pub const LOONGARCH_CSR_MCSR0: c_uint = 0xc0	/* CPUCFG0 and CPUCFG1 */;
pub const MCSR0_INT_IMPL_SHIFT: c_int = 58;
pub const MCSR0_INT_IMPL: c_int = 0;
pub const MCSR0_IOCSR_BRD_SHIFT: c_int = 57;

pub const MCSR0_HUGEPG_SHIFT: c_int = 56;

pub const MCSR0_RPLMTLB_SHIFT: c_int = 55;

pub const MCSR0_EP_SHIFT: c_int = 54;

pub const MCSR0_RI_SHIFT: c_int = 53;

pub const MCSR0_UAL_SHIFT: c_int = 52;

pub const MCSR0_VABIT_SHIFT: c_int = 44;
pub const MCSR0_VABIT_WIDTH: c_int = 8;

pub const VABIT_DEFAULT: c_uint = 0x2f;
pub const MCSR0_PABIT_SHIFT: c_int = 36;
pub const MCSR0_PABIT_WIDTH: c_int = 8;

pub const PABIT_DEFAULT: c_uint = 0x2f;
pub const MCSR0_IOCSR_SHIFT: c_int = 35;

pub const MCSR0_PAGING_SHIFT: c_int = 34;

pub const MCSR0_GR64_SHIFT: c_int = 33;

pub const GR64_DEFAULT: c_int = 1;
pub const MCSR0_GR32_SHIFT: c_int = 32;

pub const GR32_DEFAULT: c_int = 0;
pub const MCSR0_PRID_WIDTH: c_int = 32;
pub const MCSR0_PRID: c_uint = 0x14C010;
pub const LOONGARCH_CSR_MCSR1: c_uint = 0xc1	/* CPUCFG2 and CPUCFG3 */;
pub const MCSR1_HPFOLD_SHIFT: c_int = 43;

pub const MCSR1_SPW_LVL_SHIFT: c_int = 40;
pub const MCSR1_SPW_LVL_WIDTH: c_int = 3;

pub const MCSR1_ICACHET_SHIFT: c_int = 39;

pub const MCSR1_ITLBT_SHIFT: c_int = 38;

pub const MCSR1_LLDBAR_SHIFT: c_int = 37;

pub const MCSR1_SCDLY_SHIFT: c_int = 36;

pub const MCSR1_LLEXC_SHIFT: c_int = 35;

pub const MCSR1_UCACC_SHIFT: c_int = 34;

pub const MCSR1_SFB_SHIFT: c_int = 33;

pub const MCSR1_CCDMA_SHIFT: c_int = 32;

pub const MCSR1_LAMO_SHIFT: c_int = 22;

pub const MCSR1_LSPW_SHIFT: c_int = 21;

pub const MCSR1_MIPSBT_SHIFT: c_int = 20;

pub const MCSR1_ARMBT_SHIFT: c_int = 19;

pub const MCSR1_X86BT_SHIFT: c_int = 18;

pub const MCSR1_LLFTPVERS_SHIFT: c_int = 15;
pub const MCSR1_LLFTPVERS_WIDTH: c_int = 3;

pub const MCSR1_LLFTP_SHIFT: c_int = 14;

pub const MCSR1_VZVERS_SHIFT: c_int = 11;
pub const MCSR1_VZVERS_WIDTH: c_int = 3;

pub const MCSR1_VZ_SHIFT: c_int = 10;

pub const MCSR1_CRYPTO_SHIFT: c_int = 9;

pub const MCSR1_COMPLEX_SHIFT: c_int = 8;

pub const MCSR1_LASX_SHIFT: c_int = 7;

pub const MCSR1_LSX_SHIFT: c_int = 6;

pub const MCSR1_FPVERS_SHIFT: c_int = 3;
pub const MCSR1_FPVERS_WIDTH: c_int = 3;

pub const MCSR1_FPDP_SHIFT: c_int = 2;

pub const MCSR1_FPSP_SHIFT: c_int = 1;

pub const MCSR1_FP_SHIFT: c_int = 0;

pub const LOONGARCH_CSR_MCSR2: c_uint = 0xc2	/* CPUCFG4 and CPUCFG5 */;
pub const MCSR2_CCDIV_SHIFT: c_int = 48;
pub const MCSR2_CCDIV_WIDTH: c_int = 16;

pub const MCSR2_CCMUL_SHIFT: c_int = 32;
pub const MCSR2_CCMUL_WIDTH: c_int = 16;

pub const MCSR2_CCFREQ_WIDTH: c_int = 32;

pub const CCFREQ_DEFAULT: c_uint = 0x5f5e100	/* 100MHz */;
pub const LOONGARCH_CSR_MCSR3: c_uint = 0xc3	/* CPUCFG6 */;
pub const MCSR3_UPM_SHIFT: c_int = 14;

pub const MCSR3_PMBITS_SHIFT: c_int = 8;
pub const MCSR3_PMBITS_WIDTH: c_int = 6;

pub const PMBITS_DEFAULT: c_uint = 0x40;
pub const MCSR3_PMNUM_SHIFT: c_int = 4;
pub const MCSR3_PMNUM_WIDTH: c_int = 4;

pub const MCSR3_PAMVER_SHIFT: c_int = 1;
pub const MCSR3_PAMVER_WIDTH: c_int = 3;

pub const MCSR3_PMP_SHIFT: c_int = 0;

pub const LOONGARCH_CSR_MCSR8: c_uint = 0xc8	/* CPUCFG16 and CPUCFG17 */;
pub const MCSR8_L1I_SIZE_SHIFT: c_int = 56;
pub const MCSR8_L1I_SIZE_WIDTH: c_int = 7;

pub const MCSR8_L1I_IDX_SHIFT: c_int = 48;
pub const MCSR8_L1I_IDX_WIDTH: c_int = 8;

pub const MCSR8_L1I_WAY_SHIFT: c_int = 32;
pub const MCSR8_L1I_WAY_WIDTH: c_int = 16;

pub const MCSR8_L3DINCL_SHIFT: c_int = 16;

pub const MCSR8_L3DPRIV_SHIFT: c_int = 15;

pub const MCSR8_L3DPRE_SHIFT: c_int = 14;

pub const MCSR8_L3IUINCL_SHIFT: c_int = 13;

pub const MCSR8_L3IUPRIV_SHIFT: c_int = 12;

pub const MCSR8_L3IUUNIFY_SHIFT: c_int = 11;

pub const MCSR8_L3IUPRE_SHIFT: c_int = 10;

pub const MCSR8_L2DINCL_SHIFT: c_int = 9;

pub const MCSR8_L2DPRIV_SHIFT: c_int = 8;

pub const MCSR8_L2DPRE_SHIFT: c_int = 7;

pub const MCSR8_L2IUINCL_SHIFT: c_int = 6;

pub const MCSR8_L2IUPRIV_SHIFT: c_int = 5;

pub const MCSR8_L2IUUNIFY_SHIFT: c_int = 4;

pub const MCSR8_L2IUPRE_SHIFT: c_int = 3;

pub const MCSR8_L1DPRE_SHIFT: c_int = 2;

pub const MCSR8_L1IUUNIFY_SHIFT: c_int = 1;

pub const MCSR8_L1IUPRE_SHIFT: c_int = 0;

pub const LOONGARCH_CSR_MCSR9: c_uint = 0xc9	/* CPUCFG18 and CPUCFG19 */;
pub const MCSR9_L2U_SIZE_SHIFT: c_int = 56;
pub const MCSR9_L2U_SIZE_WIDTH: c_int = 7;

pub const MCSR9_L2U_IDX_SHIFT: c_int = 48;
pub const MCSR9_L2U_IDX_WIDTH: c_int = 8;

pub const MCSR9_L2U_WAY_SHIFT: c_int = 32;
pub const MCSR9_L2U_WAY_WIDTH: c_int = 16;

pub const MCSR9_L1D_SIZE_SHIFT: c_int = 24;
pub const MCSR9_L1D_SIZE_WIDTH: c_int = 7;

pub const MCSR9_L1D_IDX_SHIFT: c_int = 16;
pub const MCSR9_L1D_IDX_WIDTH: c_int = 8;

pub const MCSR9_L1D_WAY_SHIFT: c_int = 0;
pub const MCSR9_L1D_WAY_WIDTH: c_int = 16;

pub const LOONGARCH_CSR_MCSR10: c_uint = 0xca	/* CPUCFG20 */;
pub const MCSR10_L3U_SIZE_SHIFT: c_int = 24;
pub const MCSR10_L3U_SIZE_WIDTH: c_int = 7;

pub const MCSR10_L3U_IDX_SHIFT: c_int = 16;
pub const MCSR10_L3U_IDX_WIDTH: c_int = 8;

pub const MCSR10_L3U_WAY_SHIFT: c_int = 0;
pub const MCSR10_L3U_WAY_WIDTH: c_int = 16;

pub const LOONGARCH_CSR_MCSR24: c_uint = 0xf0	/* cpucfg48 */;
pub const MCSR24_RAMCG_SHIFT: c_int = 3;

pub const MCSR24_VFPUCG_SHIFT: c_int = 2;

pub const MCSR24_NAPEN_SHIFT: c_int = 1;

pub const MCSR24_MCSRLOCK_SHIFT: c_int = 0;

// Uncached accelerate windows registers
pub const LOONGARCH_CSR_UCAWIN: c_uint = 0x100;
pub const LOONGARCH_CSR_UCAWIN0_LO: c_uint = 0x102;
pub const LOONGARCH_CSR_UCAWIN0_HI: c_uint = 0x103;
pub const LOONGARCH_CSR_UCAWIN1_LO: c_uint = 0x104;
pub const LOONGARCH_CSR_UCAWIN1_HI: c_uint = 0x105;
pub const LOONGARCH_CSR_UCAWIN2_LO: c_uint = 0x106;
pub const LOONGARCH_CSR_UCAWIN2_HI: c_uint = 0x107;
pub const LOONGARCH_CSR_UCAWIN3_LO: c_uint = 0x108;
pub const LOONGARCH_CSR_UCAWIN3_HI: c_uint = 0x109;
// Direct Map windows registers
pub const LOONGARCH_CSR_DMWIN0: c_uint = 0x180	/* 64 direct map win0: MEM & IF */;
pub const LOONGARCH_CSR_DMWIN1: c_uint = 0x181	/* 64 direct map win1: MEM & IF */;
pub const LOONGARCH_CSR_DMWIN2: c_uint = 0x182	/* 64 direct map win2: MEM */;
pub const LOONGARCH_CSR_DMWIN3: c_uint = 0x183	/* 64 direct map win3: MEM */;
// Direct Map window 0/1/2/3

pub const CSR_DMW2_INIT: c_uint = 0x0;
pub const CSR_DMW3_INIT: c_uint = 0x0;

pub const CSR_DMW3_INIT: c_uint = 0x0;

// Performance Counter registers
pub const LOONGARCH_CSR_PERFCTRL0: c_uint = 0x200	/* 32 perf event 0 config */;
pub const LOONGARCH_CSR_PERFCNTR0: c_uint = 0x201	/* 64 perf event 0 count value */;
pub const LOONGARCH_CSR_PERFCTRL1: c_uint = 0x202	/* 32 perf event 1 config */;
pub const LOONGARCH_CSR_PERFCNTR1: c_uint = 0x203	/* 64 perf event 1 count value */;
pub const LOONGARCH_CSR_PERFCTRL2: c_uint = 0x204	/* 32 perf event 2 config */;
pub const LOONGARCH_CSR_PERFCNTR2: c_uint = 0x205	/* 64 perf event 2 count value */;
pub const LOONGARCH_CSR_PERFCTRL3: c_uint = 0x206	/* 32 perf event 3 config */;
pub const LOONGARCH_CSR_PERFCNTR3: c_uint = 0x207	/* 64 perf event 3 count value */;

pub const CSR_PERFCTRL_EVENT: c_uint = 0x3ff;
// Debug registers
pub const LOONGARCH_CSR_MWPC: c_uint = 0x300	/* data breakpoint config */;
pub const LOONGARCH_CSR_MWPS: c_uint = 0x301	/* data breakpoint status */;
pub const LOONGARCH_CSR_DB0ADDR: c_uint = 0x310	/* data breakpoint 0 address */;
pub const LOONGARCH_CSR_DB0MASK: c_uint = 0x311	/* data breakpoint 0 mask */;
pub const LOONGARCH_CSR_DB0CTRL: c_uint = 0x312	/* data breakpoint 0 control */;
pub const LOONGARCH_CSR_DB0ASID: c_uint = 0x313	/* data breakpoint 0 asid */;
pub const LOONGARCH_CSR_DB1ADDR: c_uint = 0x318	/* data breakpoint 1 address */;
pub const LOONGARCH_CSR_DB1MASK: c_uint = 0x319	/* data breakpoint 1 mask */;
pub const LOONGARCH_CSR_DB1CTRL: c_uint = 0x31a	/* data breakpoint 1 control */;
pub const LOONGARCH_CSR_DB1ASID: c_uint = 0x31b	/* data breakpoint 1 asid */;
pub const LOONGARCH_CSR_DB2ADDR: c_uint = 0x320	/* data breakpoint 2 address */;
pub const LOONGARCH_CSR_DB2MASK: c_uint = 0x321	/* data breakpoint 2 mask */;
pub const LOONGARCH_CSR_DB2CTRL: c_uint = 0x322	/* data breakpoint 2 control */;
pub const LOONGARCH_CSR_DB2ASID: c_uint = 0x323	/* data breakpoint 2 asid */;
pub const LOONGARCH_CSR_DB3ADDR: c_uint = 0x328	/* data breakpoint 3 address */;
pub const LOONGARCH_CSR_DB3MASK: c_uint = 0x329	/* data breakpoint 3 mask */;
pub const LOONGARCH_CSR_DB3CTRL: c_uint = 0x32a	/* data breakpoint 3 control */;
pub const LOONGARCH_CSR_DB3ASID: c_uint = 0x32b	/* data breakpoint 3 asid */;
pub const LOONGARCH_CSR_DB4ADDR: c_uint = 0x330	/* data breakpoint 4 address */;
pub const LOONGARCH_CSR_DB4MASK: c_uint = 0x331	/* data breakpoint 4 maks */;
pub const LOONGARCH_CSR_DB4CTRL: c_uint = 0x332	/* data breakpoint 4 control */;
pub const LOONGARCH_CSR_DB4ASID: c_uint = 0x333	/* data breakpoint 4 asid */;
pub const LOONGARCH_CSR_DB5ADDR: c_uint = 0x338	/* data breakpoint 5 address */;
pub const LOONGARCH_CSR_DB5MASK: c_uint = 0x339	/* data breakpoint 5 mask */;
pub const LOONGARCH_CSR_DB5CTRL: c_uint = 0x33a	/* data breakpoint 5 control */;
pub const LOONGARCH_CSR_DB5ASID: c_uint = 0x33b	/* data breakpoint 5 asid */;
pub const LOONGARCH_CSR_DB6ADDR: c_uint = 0x340	/* data breakpoint 6 address */;
pub const LOONGARCH_CSR_DB6MASK: c_uint = 0x341	/* data breakpoint 6 mask */;
pub const LOONGARCH_CSR_DB6CTRL: c_uint = 0x342	/* data breakpoint 6 control */;
pub const LOONGARCH_CSR_DB6ASID: c_uint = 0x343	/* data breakpoint 6 asid */;
pub const LOONGARCH_CSR_DB7ADDR: c_uint = 0x348	/* data breakpoint 7 address */;
pub const LOONGARCH_CSR_DB7MASK: c_uint = 0x349	/* data breakpoint 7 mask */;
pub const LOONGARCH_CSR_DB7CTRL: c_uint = 0x34a	/* data breakpoint 7 control */;
pub const LOONGARCH_CSR_DB7ASID: c_uint = 0x34b	/* data breakpoint 7 asid */;
pub const LOONGARCH_CSR_DB8ADDR: c_uint = 0x350	/* data breakpoint 8 address */;
pub const LOONGARCH_CSR_DB8MASK: c_uint = 0x351	/* data breakpoint 8 mask */;
pub const LOONGARCH_CSR_DB8CTRL: c_uint = 0x352	/* data breakpoint 8 control */;
pub const LOONGARCH_CSR_DB8ASID: c_uint = 0x353	/* data breakpoint 8 asid */;
pub const LOONGARCH_CSR_DB9ADDR: c_uint = 0x358	/* data breakpoint 9 address */;
pub const LOONGARCH_CSR_DB9MASK: c_uint = 0x359	/* data breakpoint 9 mask */;
pub const LOONGARCH_CSR_DB9CTRL: c_uint = 0x35a	/* data breakpoint 9 control */;
pub const LOONGARCH_CSR_DB9ASID: c_uint = 0x35b	/* data breakpoint 9 asid */;
pub const LOONGARCH_CSR_DB10ADDR: c_uint = 0x360	/* data breakpoint 10 address */;
pub const LOONGARCH_CSR_DB10MASK: c_uint = 0x361	/* data breakpoint 10 mask */;
pub const LOONGARCH_CSR_DB10CTRL: c_uint = 0x362	/* data breakpoint 10 control */;
pub const LOONGARCH_CSR_DB10ASID: c_uint = 0x363	/* data breakpoint 10 asid */;
pub const LOONGARCH_CSR_DB11ADDR: c_uint = 0x368	/* data breakpoint 11 address */;
pub const LOONGARCH_CSR_DB11MASK: c_uint = 0x369	/* data breakpoint 11 mask */;
pub const LOONGARCH_CSR_DB11CTRL: c_uint = 0x36a	/* data breakpoint 11 control */;
pub const LOONGARCH_CSR_DB11ASID: c_uint = 0x36b	/* data breakpoint 11 asid */;
pub const LOONGARCH_CSR_DB12ADDR: c_uint = 0x370	/* data breakpoint 12 address */;
pub const LOONGARCH_CSR_DB12MASK: c_uint = 0x371	/* data breakpoint 12 mask */;
pub const LOONGARCH_CSR_DB12CTRL: c_uint = 0x372	/* data breakpoint 12 control */;
pub const LOONGARCH_CSR_DB12ASID: c_uint = 0x373	/* data breakpoint 12 asid */;
pub const LOONGARCH_CSR_DB13ADDR: c_uint = 0x378	/* data breakpoint 13 address */;
pub const LOONGARCH_CSR_DB13MASK: c_uint = 0x379	/* data breakpoint 13 mask */;
pub const LOONGARCH_CSR_DB13CTRL: c_uint = 0x37a	/* data breakpoint 13 control */;
pub const LOONGARCH_CSR_DB13ASID: c_uint = 0x37b	/* data breakpoint 13 asid */;
pub const LOONGARCH_CSR_FWPC: c_uint = 0x380	/* instruction breakpoint config */;
pub const LOONGARCH_CSR_FWPS: c_uint = 0x381	/* instruction breakpoint status */;
pub const LOONGARCH_CSR_IB0ADDR: c_uint = 0x390	/* inst breakpoint 0 address */;
pub const LOONGARCH_CSR_IB0MASK: c_uint = 0x391	/* inst breakpoint 0 mask */;
pub const LOONGARCH_CSR_IB0CTRL: c_uint = 0x392	/* inst breakpoint 0 control */;
pub const LOONGARCH_CSR_IB0ASID: c_uint = 0x393	/* inst breakpoint 0 asid */;
pub const LOONGARCH_CSR_IB1ADDR: c_uint = 0x398	/* inst breakpoint 1 address */;
pub const LOONGARCH_CSR_IB1MASK: c_uint = 0x399	/* inst breakpoint 1 mask */;
pub const LOONGARCH_CSR_IB1CTRL: c_uint = 0x39a	/* inst breakpoint 1 control */;
pub const LOONGARCH_CSR_IB1ASID: c_uint = 0x39b	/* inst breakpoint 1 asid */;
pub const LOONGARCH_CSR_IB2ADDR: c_uint = 0x3a0	/* inst breakpoint 2 address */;
pub const LOONGARCH_CSR_IB2MASK: c_uint = 0x3a1	/* inst breakpoint 2 mask */;
pub const LOONGARCH_CSR_IB2CTRL: c_uint = 0x3a2	/* inst breakpoint 2 control */;
pub const LOONGARCH_CSR_IB2ASID: c_uint = 0x3a3	/* inst breakpoint 2 asid */;
pub const LOONGARCH_CSR_IB3ADDR: c_uint = 0x3a8	/* inst breakpoint 3 address */;
pub const LOONGARCH_CSR_IB3MASK: c_uint = 0x3a9	/* breakpoint 3 mask */;
pub const LOONGARCH_CSR_IB3CTRL: c_uint = 0x3aa	/* inst breakpoint 3 control */;
pub const LOONGARCH_CSR_IB3ASID: c_uint = 0x3ab	/* inst breakpoint 3 asid */;
pub const LOONGARCH_CSR_IB4ADDR: c_uint = 0x3b0	/* inst breakpoint 4 address */;
pub const LOONGARCH_CSR_IB4MASK: c_uint = 0x3b1	/* inst breakpoint 4 mask */;
pub const LOONGARCH_CSR_IB4CTRL: c_uint = 0x3b2	/* inst breakpoint 4 control */;
pub const LOONGARCH_CSR_IB4ASID: c_uint = 0x3b3	/* inst breakpoint 4 asid */;
pub const LOONGARCH_CSR_IB5ADDR: c_uint = 0x3b8	/* inst breakpoint 5 address */;
pub const LOONGARCH_CSR_IB5MASK: c_uint = 0x3b9	/* inst breakpoint 5 mask */;
pub const LOONGARCH_CSR_IB5CTRL: c_uint = 0x3ba	/* inst breakpoint 5 control */;
pub const LOONGARCH_CSR_IB5ASID: c_uint = 0x3bb	/* inst breakpoint 5 asid */;
pub const LOONGARCH_CSR_IB6ADDR: c_uint = 0x3c0	/* inst breakpoint 6 address */;
pub const LOONGARCH_CSR_IB6MASK: c_uint = 0x3c1	/* inst breakpoint 6 mask */;
pub const LOONGARCH_CSR_IB6CTRL: c_uint = 0x3c2	/* inst breakpoint 6 control */;
pub const LOONGARCH_CSR_IB6ASID: c_uint = 0x3c3	/* inst breakpoint 6 asid */;
pub const LOONGARCH_CSR_IB7ADDR: c_uint = 0x3c8	/* inst breakpoint 7 address */;
pub const LOONGARCH_CSR_IB7MASK: c_uint = 0x3c9	/* inst breakpoint 7 mask */;
pub const LOONGARCH_CSR_IB7CTRL: c_uint = 0x3ca	/* inst breakpoint 7 control */;
pub const LOONGARCH_CSR_IB7ASID: c_uint = 0x3cb	/* inst breakpoint 7 asid */;
pub const LOONGARCH_CSR_IB8ADDR: c_uint = 0x3d0	/* inst breakpoint 8 address */;
pub const LOONGARCH_CSR_IB8MASK: c_uint = 0x3d1	/* inst breakpoint 8 mask */;
pub const LOONGARCH_CSR_IB8CTRL: c_uint = 0x3d2	/* inst breakpoint 8 control */;
pub const LOONGARCH_CSR_IB8ASID: c_uint = 0x3d3	/* inst breakpoint 8 asid */;
pub const LOONGARCH_CSR_IB9ADDR: c_uint = 0x3d8	/* inst breakpoint 9 address */;
pub const LOONGARCH_CSR_IB9MASK: c_uint = 0x3d9	/* inst breakpoint 9 mask */;
pub const LOONGARCH_CSR_IB9CTRL: c_uint = 0x3da	/* inst breakpoint 9 control */;
pub const LOONGARCH_CSR_IB9ASID: c_uint = 0x3db	/* inst breakpoint 9 asid */;
pub const LOONGARCH_CSR_IB10ADDR: c_uint = 0x3e0	/* inst breakpoint 10 address */;
pub const LOONGARCH_CSR_IB10MASK: c_uint = 0x3e1	/* inst breakpoint 10 mask */;
pub const LOONGARCH_CSR_IB10CTRL: c_uint = 0x3e2	/* inst breakpoint 10 control */;
pub const LOONGARCH_CSR_IB10ASID: c_uint = 0x3e3	/* inst breakpoint 10 asid */;
pub const LOONGARCH_CSR_IB11ADDR: c_uint = 0x3e8	/* inst breakpoint 11 address */;
pub const LOONGARCH_CSR_IB11MASK: c_uint = 0x3e9	/* inst breakpoint 11 mask */;
pub const LOONGARCH_CSR_IB11CTRL: c_uint = 0x3ea	/* inst breakpoint 11 control */;
pub const LOONGARCH_CSR_IB11ASID: c_uint = 0x3eb	/* inst breakpoint 11 asid */;
pub const LOONGARCH_CSR_IB12ADDR: c_uint = 0x3f0	/* inst breakpoint 12 address */;
pub const LOONGARCH_CSR_IB12MASK: c_uint = 0x3f1	/* inst breakpoint 12 mask */;
pub const LOONGARCH_CSR_IB12CTRL: c_uint = 0x3f2	/* inst breakpoint 12 control */;
pub const LOONGARCH_CSR_IB12ASID: c_uint = 0x3f3	/* inst breakpoint 12 asid */;
pub const LOONGARCH_CSR_IB13ADDR: c_uint = 0x3f8	/* inst breakpoint 13 address */;
pub const LOONGARCH_CSR_IB13MASK: c_uint = 0x3f9	/* inst breakpoint 13 mask */;
pub const LOONGARCH_CSR_IB13CTRL: c_uint = 0x3fa	/* inst breakpoint 13 control */;
pub const LOONGARCH_CSR_IB13ASID: c_uint = 0x3fb	/* inst breakpoint 13 asid */;
pub const LOONGARCH_CSR_DEBUG: c_uint = 0x500	/* debug config */;
pub const LOONGARCH_CSR_DERA: c_uint = 0x501	/* debug era */;
pub const LOONGARCH_CSR_DESAVE: c_uint = 0x502	/* debug save */;
pub const CSR_FWPC_SKIP_SHIFT: c_int = 16;

//
// CSR_ECFG IM
//
pub const ECFG0_IM: c_uint = 0x00005fff;
pub const ECFGB_SIP0: c_int = 0;

pub const ECFGB_SIP1: c_int = 1;

pub const ECFGB_IP0: c_int = 2;

pub const ECFGB_IP1: c_int = 3;

pub const ECFGB_IP2: c_int = 4;

pub const ECFGB_IP3: c_int = 5;

pub const ECFGB_IP4: c_int = 6;

pub const ECFGB_IP5: c_int = 7;

pub const ECFGB_IP6: c_int = 8;

pub const ECFGB_IP7: c_int = 9;

pub const ECFGB_PMC: c_int = 10;

pub const ECFGB_TIMER: c_int = 11;

pub const ECFGB_IPI: c_int = 12;

pub const ESTATF_IP: c_uint = 0x00003fff;
pub const LOONGARCH_IOCSR_FEATURES: c_uint = 0x8;

pub const LOONGARCH_IOCSR_VENDOR: c_uint = 0x10;
pub const LOONGARCH_IOCSR_CPUNAME: c_uint = 0x20;
pub const LOONGARCH_IOCSR_NODECNT: c_uint = 0x408;
pub const LOONGARCH_IOCSR_MISC_FUNC: c_uint = 0x420;

pub const LOONGARCH_IOCSR_CPUTEMP: c_uint = 0x428;
pub const LOONGARCH_IOCSR_SMCMBX: c_uint = 0x51c;
// PerCore CSR, only accessible by local cores
pub const LOONGARCH_IOCSR_IPI_STATUS: c_uint = 0x1000;
pub const LOONGARCH_IOCSR_IPI_EN: c_uint = 0x1004;
pub const LOONGARCH_IOCSR_IPI_SET: c_uint = 0x1008;
pub const LOONGARCH_IOCSR_IPI_CLEAR: c_uint = 0x100c;
pub const LOONGARCH_IOCSR_MBUF0: c_uint = 0x1020;
pub const LOONGARCH_IOCSR_MBUF1: c_uint = 0x1028;
pub const LOONGARCH_IOCSR_MBUF2: c_uint = 0x1030;
pub const LOONGARCH_IOCSR_MBUF3: c_uint = 0x1038;
pub const LOONGARCH_IOCSR_IPI_SEND: c_uint = 0x1040;
pub const IOCSR_IPI_SEND_IP_SHIFT: c_int = 0;
pub const IOCSR_IPI_SEND_CPU_SHIFT: c_int = 16;

pub const LOONGARCH_IOCSR_MBUF_SEND: c_uint = 0x1048;

pub const IOCSR_MBUF_SEND_BOX_SHIFT: c_int = 2;

pub const IOCSR_MBUF_SEND_CPU_SHIFT: c_int = 16;
pub const IOCSR_MBUF_SEND_BUF_SHIFT: c_int = 32;
pub const IOCSR_MBUF_SEND_H32_MASK: c_uint = 0xFFFFFFFF00000000ULL;
pub const LOONGARCH_IOCSR_ANY_SEND: c_uint = 0x1158;

pub const IOCSR_ANY_SEND_CPU_SHIFT: c_int = 16;
pub const IOCSR_ANY_SEND_MASK_SHIFT: c_int = 27;
pub const IOCSR_ANY_SEND_BUF_SHIFT: c_int = 32;
pub const IOCSR_ANY_SEND_H32_MASK: c_uint = 0xFFFFFFFF00000000ULL;
// Register offset and bit definition for CSR access
pub const LOONGARCH_IOCSR_TIMER_CFG: c_uint = 0x1060;
pub const LOONGARCH_IOCSR_TIMER_TICK: c_uint = 0x1070;

pub const IOCSR_TIMER_MASK: c_uint = 0x0ffffffffffffULL;

pub const LOONGARCH_IOCSR_EXTIOI_NODEMAP_BASE: c_uint = 0x14a0;
pub const LOONGARCH_IOCSR_EXTIOI_IPMAP_BASE: c_uint = 0x14c0;
pub const LOONGARCH_IOCSR_EXTIOI_EN_BASE: c_uint = 0x1600;
pub const LOONGARCH_IOCSR_EXTIOI_BOUNCE_BASE: c_uint = 0x1680;
pub const LOONGARCH_IOCSR_EXTIOI_ISR_BASE: c_uint = 0x1800;
pub const LOONGARCH_IOCSR_EXTIOI_ROUTE_BASE: c_uint = 0x1c00;
pub const IOCSR_EXTIOI_VECTOR_NUM: c_int = 256;

extern "C" {
    pub fn csr_read32(_arg: LOONGARCH_CSR_CPUID) -> return;
}

//
// Manipulate bits in a register.
//

// Generic EntryLo bit definitions

pub const ENTRYLO_PLV_SHIFT: c_int = 2;

pub const ENTRYLO_C_SHIFT: c_int = 4;

// Values for PageSize register
pub const PS_4K: c_uint = 0x0000000c;
pub const PS_8K: c_uint = 0x0000000d;
pub const PS_16K: c_uint = 0x0000000e;
pub const PS_32K: c_uint = 0x0000000f;
pub const PS_64K: c_uint = 0x00000010;
pub const PS_128K: c_uint = 0x00000011;
pub const PS_256K: c_uint = 0x00000012;
pub const PS_512K: c_uint = 0x00000013;
pub const PS_1M: c_uint = 0x00000014;
pub const PS_2M: c_uint = 0x00000015;
pub const PS_4M: c_uint = 0x00000016;
pub const PS_8M: c_uint = 0x00000017;
pub const PS_16M: c_uint = 0x00000018;
pub const PS_32M: c_uint = 0x00000019;
pub const PS_64M: c_uint = 0x0000001a;
pub const PS_128M: c_uint = 0x0000001b;
pub const PS_256M: c_uint = 0x0000001c;
pub const PS_512M: c_uint = 0x0000001d;
pub const PS_1G: c_uint = 0x0000001e;
// Default page size for a given kernel configuration

// Default huge tlb size for a given kernel configuration

// ExStatus.ExcCode

// Interrupt numbers

pub const INT_SWI1: c_int = 1;

pub const INT_HWI1: c_int = 3;
pub const INT_HWI2: c_int = 4;
pub const INT_HWI3: c_int = 5;
pub const INT_HWI4: c_int = 6;
pub const INT_HWI5: c_int = 7;
pub const INT_HWI6: c_int = 8;
pub const INT_HWI7: c_int = 9;

pub const INT_IPI: c_int = 12;
pub const INT_NMI: c_int = 13;
pub const INT_AVEC: c_int = 14;
// ExcCodes corresponding to interrupts

pub const EXCCODE_INT_START: c_int = 64;

// FPU Status Register Names

// FPU Status Register Values
pub const FPU_CSR_RSVD: c_uint = 0xe0e0fce0;
//
// X the exception cause indicator
// E the exception enable
// S the sticky/flag bit
//
pub const FPU_CSR_ALL_X: c_uint = 0x1f000000;
pub const FPU_CSR_INV_X: c_uint = 0x10000000;
pub const FPU_CSR_DIV_X: c_uint = 0x08000000;
pub const FPU_CSR_OVF_X: c_uint = 0x04000000;
pub const FPU_CSR_UDF_X: c_uint = 0x02000000;
pub const FPU_CSR_INE_X: c_uint = 0x01000000;
pub const FPU_CSR_ALL_S: c_uint = 0x001f0000;
pub const FPU_CSR_INV_S: c_uint = 0x00100000;
pub const FPU_CSR_DIV_S: c_uint = 0x00080000;
pub const FPU_CSR_OVF_S: c_uint = 0x00040000;
pub const FPU_CSR_UDF_S: c_uint = 0x00020000;
pub const FPU_CSR_INE_S: c_uint = 0x00010000;
pub const FPU_CSR_ALL_E: c_uint = 0x0000001f;
pub const FPU_CSR_INV_E: c_uint = 0x00000010;
pub const FPU_CSR_DIV_E: c_uint = 0x00000008;
pub const FPU_CSR_OVF_E: c_uint = 0x00000004;
pub const FPU_CSR_UDF_E: c_uint = 0x00000002;
pub const FPU_CSR_INE_E: c_uint = 0x00000001;
// Bits 8 and 9 of FPU Status Register specify the rounding mode
pub const FPU_CSR_RM: c_uint = 0x300;
pub const FPU_CSR_RN: c_uint = 0x000	/* nearest */;
pub const FPU_CSR_RZ: c_uint = 0x100	/* towards zero */;
pub const FPU_CSR_RU: c_uint = 0x200	/* towards +Infinity */;
pub const FPU_CSR_RD: c_uint = 0x300	/* towards -Infinity */;
// Bit 6 of FPU Status Register specify the LBT TOP simulation mode
pub const FPU_CSR_TM_SHIFT: c_uint = 0x6;

