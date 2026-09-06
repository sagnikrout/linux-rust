//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/msr-index.h
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

// CPU model specific register (MSR) numbers.
// x86-64 specific MSRs
pub const MSR_EFER: c_uint = 0xc0000080 /* extended feature register */;
pub const MSR_STAR: c_uint = 0xc0000081 /* legacy mode SYSCALL target */;
pub const MSR_LSTAR: c_uint = 0xc0000082 /* long mode SYSCALL target */;
pub const MSR_CSTAR: c_uint = 0xc0000083 /* compat mode SYSCALL target */;
pub const MSR_SYSCALL_MASK: c_uint = 0xc0000084 /* EFLAGS mask for syscall */;
pub const MSR_FS_BASE: c_uint = 0xc0000100 /* 64bit FS base */;
pub const MSR_GS_BASE: c_uint = 0xc0000101 /* 64bit GS base */;
pub const MSR_KERNEL_GS_BASE: c_uint = 0xc0000102 /* SwapGS GS shadow */;
pub const MSR_TSC_AUX: c_uint = 0xc0000103 /* Auxiliary TSC */;
// EFER bits:

//
// Architectural memory types that are common to MTRRs, PAT, VMX MSRs, etc.
// Most MSRs support/allow only a subset of memory types, but the values
// themselves are common across all relevant MSRs.
//

// RESERVED			2
// RESERVED			3

// FRED MSRs
pub const MSR_IA32_FRED_RSP0: c_uint = 0x1cc			/* Level 0 stack pointer */;
pub const MSR_IA32_FRED_RSP1: c_uint = 0x1cd			/* Level 1 stack pointer */;
pub const MSR_IA32_FRED_RSP2: c_uint = 0x1ce			/* Level 2 stack pointer */;
pub const MSR_IA32_FRED_RSP3: c_uint = 0x1cf			/* Level 3 stack pointer */;
pub const MSR_IA32_FRED_STKLVLS: c_uint = 0x1d0			/* Exception stack levels */;

pub const MSR_IA32_FRED_SSP1: c_uint = 0x1d1			/* Level 1 shadow stack pointer */;
pub const MSR_IA32_FRED_SSP2: c_uint = 0x1d2			/* Level 2 shadow stack pointer */;
pub const MSR_IA32_FRED_SSP3: c_uint = 0x1d3			/* Level 3 shadow stack pointer */;
pub const MSR_IA32_FRED_CONFIG: c_uint = 0x1d4			/* Entrypoint and interrupt stack level */;
// Intel MSRs. Some also available on other CPUs
pub const MSR_TEST_CTRL: c_uint = 0x00000033;
pub const MSR_TEST_CTRL_SPLIT_LOCK_DETECT_BIT: c_int = 29;

pub const MSR_IA32_SPEC_CTRL: c_uint = 0x00000048 /* Speculation Control */;

// A mask for bits which the kernel toggles when controlling mitigations

pub const MSR_IA32_PRED_CMD: c_uint = 0x00000049 /* Prediction Command */;

pub const MSR_PPIN_CTL: c_uint = 0x0000004e;
pub const MSR_PPIN: c_uint = 0x0000004f;
pub const MSR_IA32_PERFCTR0: c_uint = 0x000000c1;
pub const MSR_IA32_PERFCTR1: c_uint = 0x000000c2;
pub const MSR_FSB_FREQ: c_uint = 0x000000cd;
pub const MSR_PLATFORM_INFO: c_uint = 0x000000ce;
pub const MSR_PLATFORM_INFO_CPUID_FAULT_BIT: c_int = 31;

pub const MSR_IA32_UMWAIT_CONTROL: c_uint = 0xe1;

//
// The time field is bit[31:2], but representing a 32bit value with
// bit[1:0] zero.
//

// Abbreviated from Intel SDM name IA32_CORE_CAPABILITIES
pub const MSR_IA32_CORE_CAPS: c_uint = 0x000000cf;
pub const MSR_IA32_CORE_CAPS_INTEGRITY_CAPS_BIT: c_int = 2;

pub const MSR_IA32_CORE_CAPS_SPLIT_LOCK_DETECT_BIT: c_int = 5;

pub const MSR_PKG_CST_CONFIG_CONTROL: c_uint = 0x000000e2;

pub const MSR_MTRRcap: c_uint = 0x000000fe;
pub const MSR_IA32_ARCH_CAPABILITIES: c_uint = 0x0000010a;

// Not susceptible to Speculative Store Bypass
// attack, so no Speculative Store Bypass
// control required.
//

// Not susceptible to
// Microarchitectural Data
// Sampling (MDS) vulnerabilities.
//

// The processor is not susceptible to a
// machine check error due to modifying the
// code page size along with either the
// physical address or cache type
// without TLB invalidation.
//

// Not susceptible to
// TSX Async Abort (TAA) vulnerabilities.
//

// Not susceptible to SBDR and SSDP
// variants of Processor MMIO stale data
// vulnerabilities.
//

// Not susceptible to FBSDP variant of
// Processor MMIO stale data
// vulnerabilities.
//

// Not susceptible to PSDP variant of
// Processor MMIO stale data
// vulnerabilities.
//

// Indicates the presence of microcode update
// feature enumeration and status information.
//

// VERW clears CPU fill buffer
// even on MDS_NO CPUs.
//

// MSR_IA32_MCU_OPT_CTRL[FB_CLEAR_DIS]
// bit available to control VERW
// behavior.
//

// Indicates RET may use predictors
// other than the RSB. With eIBRS
// enabled predictions in kernel mode
// are restricted to targets in
// kernel.
//

// CPU is not affected by Branch
// History Injection.
//

// IA32_XAPIC_DISABLE_STATUS MSR
// supported
//

// Not susceptible to Post-Barrier
// Return Stack Buffer Predictions.
//

// CPU is vulnerable to Gather
// Data Sampling (GDS) and
// has controls for mitigation.
//

// CPU is not vulnerable to Gather
// Data Sampling (GDS).
//

// Not susceptible to Register
// File Data Sampling.
//

// VERW clears CPU Register
// File.
//

// Not susceptible to
// Indirect Target Selection.
// This bit is not set by
// HW, but is synthesized by
// VMMs for guests to know
// their affected status.
//
pub const MSR_IA32_FLUSH_CMD: c_uint = 0x0000010b;

// Writeback and invalidate the
// L1 data cache.
//
pub const MSR_IA32_BBL_CR_CTL: c_uint = 0x00000119;
pub const MSR_IA32_BBL_CR_CTL3: c_uint = 0x0000011e;
pub const MSR_IA32_TSX_CTRL: c_uint = 0x00000122;

pub const MSR_IA32_MCU_OPT_CTRL: c_uint = 0x00000123;

pub const MSR_IA32_SYSENTER_CS: c_uint = 0x00000174;
pub const MSR_IA32_SYSENTER_ESP: c_uint = 0x00000175;
pub const MSR_IA32_SYSENTER_EIP: c_uint = 0x00000176;
pub const MSR_IA32_MCG_CAP: c_uint = 0x00000179;
pub const MSR_IA32_MCG_STATUS: c_uint = 0x0000017a;
pub const MSR_IA32_MCG_CTL: c_uint = 0x0000017b;
pub const MSR_ERROR_CONTROL: c_uint = 0x0000017f;
pub const MSR_IA32_MCG_EXT_CTL: c_uint = 0x000004d0;
pub const MSR_OFFCORE_RSP_0: c_uint = 0x000001a6;
pub const MSR_OFFCORE_RSP_1: c_uint = 0x000001a7;
pub const MSR_TURBO_RATIO_LIMIT: c_uint = 0x000001ad;
pub const MSR_TURBO_RATIO_LIMIT1: c_uint = 0x000001ae;
pub const MSR_TURBO_RATIO_LIMIT2: c_uint = 0x000001af;
pub const MSR_SNOOP_RSP_0: c_uint = 0x00001328;
pub const MSR_SNOOP_RSP_1: c_uint = 0x00001329;
pub const MSR_OMR_0: c_uint = 0x000003e0;
pub const MSR_OMR_1: c_uint = 0x000003e1;
pub const MSR_OMR_2: c_uint = 0x000003e2;
pub const MSR_OMR_3: c_uint = 0x000003e3;
pub const MSR_LBR_SELECT: c_uint = 0x000001c8;
pub const MSR_LBR_TOS: c_uint = 0x000001c9;
pub const MSR_IA32_POWER_CTL: c_uint = 0x000001fc;
pub const MSR_IA32_POWER_CTL_BIT_EE: c_int = 19;
// Abbreviated from Intel SDM name IA32_INTEGRITY_CAPABILITIES
pub const MSR_INTEGRITY_CAPS: c_uint = 0x000002d9;
pub const MSR_INTEGRITY_CAPS_ARRAY_BIST_BIT: c_int = 2;

pub const MSR_INTEGRITY_CAPS_PERIODIC_BIST_BIT: c_int = 4;

pub const MSR_INTEGRITY_CAPS_SBAF_BIT: c_int = 8;

pub const MSR_LBR_NHM_FROM: c_uint = 0x00000680;
pub const MSR_LBR_NHM_TO: c_uint = 0x000006c0;
pub const MSR_LBR_CORE_FROM: c_uint = 0x00000040;
pub const MSR_LBR_CORE_TO: c_uint = 0x00000060;
pub const MSR_LBR_INFO_0: c_uint = 0x00000dc0 /* ... 0xddf for _31 */;

pub const LBR_INFO_CYCLES: c_uint = 0xffff;
pub const LBR_INFO_BR_TYPE_OFFSET: c_int = 56;

pub const LBR_INFO_BR_CNTR_OFFSET: c_int = 32;
pub const LBR_INFO_BR_CNTR_NUM: c_int = 4;
pub const LBR_INFO_BR_CNTR_BITS: c_int = 2;

pub const MSR_ARCH_LBR_CTL: c_uint = 0x000014ce;

pub const ARCH_LBR_CTL_CPL_OFFSET: c_int = 1;

pub const ARCH_LBR_CTL_STACK_OFFSET: c_int = 3;

pub const ARCH_LBR_CTL_FILTER_OFFSET: c_int = 16;

pub const MSR_ARCH_LBR_DEPTH: c_uint = 0x000014cf;
pub const MSR_ARCH_LBR_FROM_0: c_uint = 0x00001500;
pub const MSR_ARCH_LBR_TO_0: c_uint = 0x00001600;
pub const MSR_ARCH_LBR_INFO_0: c_uint = 0x00001200;
pub const MSR_IA32_PEBS_ENABLE: c_uint = 0x000003f1;
pub const MSR_PEBS_DATA_CFG: c_uint = 0x000003f2;
pub const MSR_IA32_DS_AREA: c_uint = 0x00000600;
pub const MSR_IA32_PERF_CAPABILITIES: c_uint = 0x00000345;
pub const PERF_CAP_METRICS_IDX: c_int = 15;
pub const PERF_CAP_PT_IDX: c_int = 16;
pub const MSR_PEBS_LD_LAT_THRESHOLD: c_uint = 0x000003f6;
pub const PERF_CAP_LBR_FMT: c_uint = 0x3f;

pub const PERF_CAP_PEBS_FORMAT: c_uint = 0xf00;

// Arch PEBS
pub const MSR_IA32_PEBS_BASE: c_uint = 0x000003f4;
pub const MSR_IA32_PEBS_INDEX: c_uint = 0x000003f5;
pub const ARCH_PEBS_OFFSET_MASK: c_uint = 0x7fffff;
pub const ARCH_PEBS_INDEX_WR_SHIFT: c_int = 4;
pub const ARCH_PEBS_RELOAD: c_uint = 0xffffffff;

pub const ARCH_PEBS_LBR_SHIFT: c_int = 40;

pub const MSR_IA32_RTIT_CTL: c_uint = 0x00000570;

pub const RTIT_CTL_MTC_RANGE_OFFSET: c_int = 14;

pub const RTIT_CTL_CYC_THRESH_OFFSET: c_int = 19;

pub const RTIT_CTL_PSB_FREQ_OFFSET: c_int = 24;

pub const RTIT_CTL_ADDR0_OFFSET: c_int = 32;

pub const RTIT_CTL_ADDR1_OFFSET: c_int = 36;

pub const RTIT_CTL_ADDR2_OFFSET: c_int = 40;

pub const RTIT_CTL_ADDR3_OFFSET: c_int = 44;

pub const MSR_IA32_RTIT_STATUS: c_uint = 0x00000571;

pub const RTIT_STATUS_BYTECNT_OFFSET: c_int = 32;

pub const MSR_IA32_RTIT_ADDR0_A: c_uint = 0x00000580;
pub const MSR_IA32_RTIT_ADDR0_B: c_uint = 0x00000581;
pub const MSR_IA32_RTIT_ADDR1_A: c_uint = 0x00000582;
pub const MSR_IA32_RTIT_ADDR1_B: c_uint = 0x00000583;
pub const MSR_IA32_RTIT_ADDR2_A: c_uint = 0x00000584;
pub const MSR_IA32_RTIT_ADDR2_B: c_uint = 0x00000585;
pub const MSR_IA32_RTIT_ADDR3_A: c_uint = 0x00000586;
pub const MSR_IA32_RTIT_ADDR3_B: c_uint = 0x00000587;
pub const MSR_IA32_RTIT_CR3_MATCH: c_uint = 0x00000572;
pub const MSR_IA32_RTIT_OUTPUT_BASE: c_uint = 0x00000560;
pub const MSR_IA32_RTIT_OUTPUT_MASK: c_uint = 0x00000561;
pub const MSR_MTRRfix64K_00000: c_uint = 0x00000250;
pub const MSR_MTRRfix16K_80000: c_uint = 0x00000258;
pub const MSR_MTRRfix16K_A0000: c_uint = 0x00000259;
pub const MSR_MTRRfix4K_C0000: c_uint = 0x00000268;
pub const MSR_MTRRfix4K_C8000: c_uint = 0x00000269;
pub const MSR_MTRRfix4K_D0000: c_uint = 0x0000026a;
pub const MSR_MTRRfix4K_D8000: c_uint = 0x0000026b;
pub const MSR_MTRRfix4K_E0000: c_uint = 0x0000026c;
pub const MSR_MTRRfix4K_E8000: c_uint = 0x0000026d;
pub const MSR_MTRRfix4K_F0000: c_uint = 0x0000026e;
pub const MSR_MTRRfix4K_F8000: c_uint = 0x0000026f;
pub const MSR_MTRRdefType: c_uint = 0x000002ff;
pub const MSR_IA32_CR_PAT: c_uint = 0x00000277;

pub const MSR_IA32_DEBUGCTLMSR: c_uint = 0x000001d9;
pub const MSR_IA32_LASTBRANCHFROMIP: c_uint = 0x000001db;
pub const MSR_IA32_LASTBRANCHTOIP: c_uint = 0x000001dc;
pub const MSR_IA32_LASTINTFROMIP: c_uint = 0x000001dd;
pub const MSR_IA32_LASTINTTOIP: c_uint = 0x000001de;
pub const MSR_IA32_PASID: c_uint = 0x00000d93;

// DEBUGCTLMSR bits (others vary by model):

pub const DEBUGCTLMSR_BTF_SHIFT: c_int = 1;

pub const DEBUGCTLMSR_FREEZE_IN_SMM_BIT: c_int = 14;

pub const MSR_PEBS_FRONTEND: c_uint = 0x000003f7;
pub const MSR_IA32_MC0_CTL: c_uint = 0x00000400;
pub const MSR_IA32_MC0_STATUS: c_uint = 0x00000401;
pub const MSR_IA32_MC0_ADDR: c_uint = 0x00000402;
pub const MSR_IA32_MC0_MISC: c_uint = 0x00000403;
// C-state Residency Counters
pub const MSR_PKG_C3_RESIDENCY: c_uint = 0x000003f8;
pub const MSR_PKG_C6_RESIDENCY: c_uint = 0x000003f9;
pub const MSR_ATOM_PKG_C6_RESIDENCY: c_uint = 0x000003fa;
pub const MSR_PKG_C7_RESIDENCY: c_uint = 0x000003fa;
pub const MSR_CORE_C3_RESIDENCY: c_uint = 0x000003fc;
pub const MSR_CORE_C6_RESIDENCY: c_uint = 0x000003fd;
pub const MSR_CORE_C7_RESIDENCY: c_uint = 0x000003fe;
pub const MSR_KNL_CORE_C6_RESIDENCY: c_uint = 0x000003ff;
pub const MSR_PKG_C2_RESIDENCY: c_uint = 0x0000060d;
pub const MSR_PKG_C8_RESIDENCY: c_uint = 0x00000630;
pub const MSR_PKG_C9_RESIDENCY: c_uint = 0x00000631;
pub const MSR_PKG_C10_RESIDENCY: c_uint = 0x00000632;
// Interrupt Response Limit
pub const MSR_PKGC3_IRTL: c_uint = 0x0000060a;
pub const MSR_PKGC6_IRTL: c_uint = 0x0000060b;
pub const MSR_PKGC7_IRTL: c_uint = 0x0000060c;
pub const MSR_PKGC8_IRTL: c_uint = 0x00000633;
pub const MSR_PKGC9_IRTL: c_uint = 0x00000634;
pub const MSR_PKGC10_IRTL: c_uint = 0x00000635;
// Run Time Average Power Limiting (RAPL) Interface
pub const MSR_VR_CURRENT_CONFIG: c_uint = 0x00000601;
pub const MSR_RAPL_POWER_UNIT: c_uint = 0x00000606;
pub const MSR_PKG_POWER_LIMIT: c_uint = 0x00000610;
pub const MSR_PKG_ENERGY_STATUS: c_uint = 0x00000611;
pub const MSR_PKG_PERF_STATUS: c_uint = 0x00000613;
pub const MSR_PKG_POWER_INFO: c_uint = 0x00000614;
pub const MSR_DRAM_POWER_LIMIT: c_uint = 0x00000618;
pub const MSR_DRAM_ENERGY_STATUS: c_uint = 0x00000619;
pub const MSR_DRAM_PERF_STATUS: c_uint = 0x0000061b;
pub const MSR_DRAM_POWER_INFO: c_uint = 0x0000061c;
pub const MSR_PP0_POWER_LIMIT: c_uint = 0x00000638;
pub const MSR_PP0_ENERGY_STATUS: c_uint = 0x00000639;
pub const MSR_PP0_POLICY: c_uint = 0x0000063a;
pub const MSR_PP0_PERF_STATUS: c_uint = 0x0000063b;
pub const MSR_PP1_POWER_LIMIT: c_uint = 0x00000640;
pub const MSR_PP1_ENERGY_STATUS: c_uint = 0x00000641;
pub const MSR_PP1_POLICY: c_uint = 0x00000642;
pub const MSR_AMD_RAPL_POWER_UNIT: c_uint = 0xc0010299;
pub const MSR_AMD_CORE_ENERGY_STATUS: c_uint = 0xc001029a;
pub const MSR_AMD_PKG_ENERGY_STATUS: c_uint = 0xc001029b;
// Config TDP MSRs
pub const MSR_CONFIG_TDP_NOMINAL: c_uint = 0x00000648;
pub const MSR_CONFIG_TDP_LEVEL_1: c_uint = 0x00000649;
pub const MSR_CONFIG_TDP_LEVEL_2: c_uint = 0x0000064A;
pub const MSR_CONFIG_TDP_CONTROL: c_uint = 0x0000064B;
pub const MSR_TURBO_ACTIVATION_RATIO: c_uint = 0x0000064C;
pub const MSR_PLATFORM_ENERGY_STATUS: c_uint = 0x0000064D;
pub const MSR_SECONDARY_TURBO_RATIO_LIMIT: c_uint = 0x00000650;
pub const MSR_PKG_WEIGHTED_CORE_C0_RES: c_uint = 0x00000658;
pub const MSR_PKG_ANY_CORE_C0_RES: c_uint = 0x00000659;
pub const MSR_PKG_ANY_GFXE_C0_RES: c_uint = 0x0000065A;
pub const MSR_PKG_BOTH_CORE_GFXE_C0_RES: c_uint = 0x0000065B;
pub const MSR_CORE_C1_RES: c_uint = 0x00000660;
pub const MSR_MODULE_C6_RES_MS: c_uint = 0x00000664;
pub const MSR_CC6_DEMOTION_POLICY_CONFIG: c_uint = 0x00000668;
pub const MSR_MC6_DEMOTION_POLICY_CONFIG: c_uint = 0x00000669;
pub const MSR_ATOM_CORE_RATIOS: c_uint = 0x0000066a;
pub const MSR_ATOM_CORE_VIDS: c_uint = 0x0000066b;
pub const MSR_ATOM_CORE_TURBO_RATIOS: c_uint = 0x0000066c;
pub const MSR_ATOM_CORE_TURBO_VIDS: c_uint = 0x0000066d;
pub const MSR_CORE_PERF_LIMIT_REASONS: c_uint = 0x00000690;
pub const MSR_GFX_PERF_LIMIT_REASONS: c_uint = 0x000006B0;
pub const MSR_RING_PERF_LIMIT_REASONS: c_uint = 0x000006B1;
// Control-flow Enforcement Technology MSRs
pub const MSR_IA32_U_CET: c_uint = 0x000006a0 /* user mode cet */;
pub const MSR_IA32_S_CET: c_uint = 0x000006a2 /* kernel mode cet */;

pub const MSR_IA32_PL0_SSP: c_uint = 0x000006a4 /* ring-0 shadow stack pointer */;
pub const MSR_IA32_PL1_SSP: c_uint = 0x000006a5 /* ring-1 shadow stack pointer */;
pub const MSR_IA32_PL2_SSP: c_uint = 0x000006a6 /* ring-2 shadow stack pointer */;
pub const MSR_IA32_PL3_SSP: c_uint = 0x000006a7 /* ring-3 shadow stack pointer */;
pub const MSR_IA32_INT_SSP_TAB: c_uint = 0x000006a8 /* exception shadow stack table */;
// Hardware P state interface
pub const MSR_PPERF: c_uint = 0x0000064e;
pub const MSR_PERF_LIMIT_REASONS: c_uint = 0x0000064f;
pub const MSR_PM_ENABLE: c_uint = 0x00000770;
pub const MSR_HWP_CAPABILITIES: c_uint = 0x00000771;
pub const MSR_HWP_REQUEST_PKG: c_uint = 0x00000772;
pub const MSR_HWP_INTERRUPT: c_uint = 0x00000773;
pub const MSR_HWP_REQUEST: c_uint = 0x00000774;
pub const MSR_HWP_STATUS: c_uint = 0x00000777;
// CPUID.6.EAX

// IA32_HWP_CAPABILITIES

// IA32_HWP_REQUEST

pub const HWP_EPP_PERFORMANCE: c_uint = 0x00;
pub const HWP_EPP_BALANCE_PERFORMANCE: c_uint = 0x80;
pub const HWP_EPP_BALANCE_POWERSAVE: c_uint = 0xC0;
pub const HWP_EPP_POWERSAVE: c_uint = 0xFF;

// IA32_HWP_STATUS

// IA32_HWP_INTERRUPT

pub const MSR_AMD64_MC0_MASK: c_uint = 0xc0010044;

// These are consecutive and not in the normal 4er MCE bank block
pub const MSR_IA32_MC0_CTL2: c_uint = 0x00000280;

pub const MSR_P6_PERFCTR0: c_uint = 0x000000c1;
pub const MSR_P6_PERFCTR1: c_uint = 0x000000c2;
pub const MSR_P6_EVNTSEL0: c_uint = 0x00000186;
pub const MSR_P6_EVNTSEL1: c_uint = 0x00000187;
pub const MSR_KNC_PERFCTR0: c_uint = 0x00000020;
pub const MSR_KNC_PERFCTR1: c_uint = 0x00000021;
pub const MSR_KNC_EVNTSEL0: c_uint = 0x00000028;
pub const MSR_KNC_EVNTSEL1: c_uint = 0x00000029;
// Alternative perfctr range with full access.
pub const MSR_IA32_PMC0: c_uint = 0x000004c1;
// Auto-reload via MSR instead of DS area
pub const MSR_RELOAD_PMC0: c_uint = 0x000014c1;
pub const MSR_RELOAD_FIXED_CTR0: c_uint = 0x00001309;
// V6 PMON MSR range
pub const MSR_IA32_PMC_V6_GP0_CTR: c_uint = 0x1900;
pub const MSR_IA32_PMC_V6_GP0_CFG_A: c_uint = 0x1901;
pub const MSR_IA32_PMC_V6_GP0_CFG_B: c_uint = 0x1902;
pub const MSR_IA32_PMC_V6_GP0_CFG_C: c_uint = 0x1903;
pub const MSR_IA32_PMC_V6_FX0_CTR: c_uint = 0x1980;
pub const MSR_IA32_PMC_V6_FX0_CFG_B: c_uint = 0x1982;
pub const MSR_IA32_PMC_V6_FX0_CFG_C: c_uint = 0x1983;
pub const MSR_IA32_PMC_V6_STEP: c_int = 4;
// KeyID partitioning between MKTME and TDX
pub const MSR_IA32_MKTME_KEYID_PARTITIONING: c_uint = 0x00000087;
//
// AMD64 MSRs. Not complete. See the architecture manual for a more
// complete list.
//
pub const MSR_AMD64_PATCH_LEVEL: c_uint = 0x0000008b;
pub const MSR_AMD64_TSC_RATIO: c_uint = 0xc0000104;
pub const MSR_AMD64_NB_CFG: c_uint = 0xc001001f;
pub const MSR_AMD64_PATCH_LOADER: c_uint = 0xc0010020;
pub const MSR_AMD_PERF_CTL: c_uint = 0xc0010062;
pub const MSR_AMD_PERF_STATUS: c_uint = 0xc0010063;
pub const MSR_AMD_PSTATE_DEF_BASE: c_uint = 0xc0010064;
pub const MSR_AMD64_GUEST_TSC_FREQ: c_uint = 0xc0010134;
pub const MSR_AMD64_OSVW_ID_LENGTH: c_uint = 0xc0010140;
pub const MSR_AMD64_OSVW_STATUS: c_uint = 0xc0010141;
pub const MSR_AMD_PPIN_CTL: c_uint = 0xc00102f0;
pub const MSR_AMD_PPIN: c_uint = 0xc00102f1;
pub const MSR_AMD64_CPUID_FN_7: c_uint = 0xc0011002;
pub const MSR_AMD64_CPUID_FN_1: c_uint = 0xc0011004;
pub const MSR_AMD64_CPUID_EXT_FEAT: c_uint = 0xc0011005;
pub const MSR_AMD64_CPUID_EXT_FEAT_TOPOEXT_BIT: c_int = 54;

pub const MSR_AMD64_LS_CFG: c_uint = 0xc0011020;
pub const MSR_AMD64_DC_CFG: c_uint = 0xc0011022;
pub const MSR_AMD64_TW_CFG: c_uint = 0xc0011023;
pub const MSR_AMD64_FP_CFG: c_uint = 0xc0011028;
pub const MSR_AMD64_FP_CFG_ZEN1_DENORM_FIX_BIT: c_int = 9;
pub const MSR_AMD64_DE_CFG: c_uint = 0xc0011029;
pub const MSR_AMD64_DE_CFG_LFENCE_SERIALIZE_BIT: c_int = 1;

pub const MSR_AMD64_DE_CFG_ZEN2_FP_BACKUP_FIX_BIT: c_int = 9;
pub const MSR_AMD64_BU_CFG2: c_uint = 0xc001102a;
pub const MSR_AMD64_IBSFETCHCTL: c_uint = 0xc0011030;
pub const MSR_AMD64_IBSFETCHLINAD: c_uint = 0xc0011031;
pub const MSR_AMD64_IBSFETCHPHYSAD: c_uint = 0xc0011032;
pub const MSR_AMD64_IBSFETCH_REG_COUNT: c_int = 3;

pub const MSR_AMD64_IBSOPCTL: c_uint = 0xc0011033;
pub const MSR_AMD64_IBSOPRIP: c_uint = 0xc0011034;
pub const MSR_AMD64_IBSOPDATA: c_uint = 0xc0011035;
pub const MSR_AMD64_IBSOPDATA2: c_uint = 0xc0011036;
pub const MSR_AMD64_IBSOPDATA3: c_uint = 0xc0011037;
pub const MSR_AMD64_IBSDCLINAD: c_uint = 0xc0011038;
pub const MSR_AMD64_IBSDCPHYSAD: c_uint = 0xc0011039;
pub const MSR_AMD64_IBSOP_REG_COUNT: c_int = 7;

pub const MSR_AMD64_IBSCTL: c_uint = 0xc001103a;
pub const MSR_AMD64_IBSBRTARGET: c_uint = 0xc001103b;
pub const MSR_AMD64_ICIBSEXTDCTL: c_uint = 0xc001103c;
pub const MSR_AMD64_IBSOPDATA4: c_uint = 0xc001103d;
pub const MSR_AMD64_IBSOPCTL2: c_uint = 0xc001103e;
pub const MSR_AMD64_IBSFETCHCTL2: c_uint = 0xc001103f;

pub const MSR_AMD64_SVM_AVIC_DOORBELL: c_uint = 0xc001011b;
pub const MSR_AMD64_VM_PAGE_FLUSH: c_uint = 0xc001011e;
pub const MSR_AMD64_VIRT_SPEC_CTRL: c_uint = 0xc001011f;
pub const MSR_AMD64_SEV_ES_GHCB: c_uint = 0xc0010130;
pub const MSR_AMD64_SEV: c_uint = 0xc0010131;
pub const MSR_AMD64_SEV_ENABLED_BIT: c_int = 0;

pub const MSR_AMD64_SEV_ES_ENABLED_BIT: c_int = 1;

pub const MSR_AMD64_SEV_SNP_ENABLED_BIT: c_int = 2;

pub const MSR_AMD64_SNP_VTOM_BIT: c_int = 3;

pub const MSR_AMD64_SNP_REFLECT_VC_BIT: c_int = 4;

pub const MSR_AMD64_SNP_RESTRICTED_INJ_BIT: c_int = 5;

pub const MSR_AMD64_SNP_ALT_INJ_BIT: c_int = 6;

pub const MSR_AMD64_SNP_DEBUG_SWAP_BIT: c_int = 7;

pub const MSR_AMD64_SNP_PREVENT_HOST_IBS_BIT: c_int = 8;

pub const MSR_AMD64_SNP_BTB_ISOLATION_BIT: c_int = 9;

pub const MSR_AMD64_SNP_VMPL_SSS_BIT: c_int = 10;

pub const MSR_AMD64_SNP_SECURE_TSC_BIT: c_int = 11;

pub const MSR_AMD64_SNP_VMGEXIT_PARAM_BIT: c_int = 12;

pub const MSR_AMD64_SNP_IBS_VIRT_BIT: c_int = 14;

pub const MSR_AMD64_SNP_VMSA_REG_PROT_BIT: c_int = 16;

pub const MSR_AMD64_SNP_SMT_PROT_BIT: c_int = 17;

pub const MSR_AMD64_SNP_SECURE_AVIC_BIT: c_int = 18;

pub const MSR_AMD64_SNP_IBPB_ON_ENTRY_BIT: c_int = 23;

pub const MSR_AMD64_SNP_RESV_BIT: c_int = 24;

pub const MSR_AMD64_SAVIC_CONTROL: c_uint = 0xc0010138;
pub const MSR_AMD64_SAVIC_EN_BIT: c_int = 0;

pub const MSR_AMD64_SAVIC_ALLOWEDNMI_BIT: c_int = 1;

pub const MSR_AMD64_RMP_BASE: c_uint = 0xc0010132;
pub const MSR_AMD64_RMP_END: c_uint = 0xc0010133;
pub const MSR_AMD64_RMP_CFG: c_uint = 0xc0010136;
pub const MSR_AMD64_SEG_RMP_ENABLED_BIT: c_int = 0;

pub const MSR_SVSM_CAA: c_uint = 0xc001f000;
// AMD Collaborative Processor Performance Control MSRs
pub const MSR_AMD_CPPC_CAP1: c_uint = 0xc00102b0;
pub const MSR_AMD_CPPC_ENABLE: c_uint = 0xc00102b1;
pub const MSR_AMD_CPPC_CAP2: c_uint = 0xc00102b2;
pub const MSR_AMD_CPPC_REQ: c_uint = 0xc00102b3;
pub const MSR_AMD_CPPC_STATUS: c_uint = 0xc00102b4;
pub const MSR_AMD_CPPC_REQ2: c_uint = 0xc00102b5;
// Masks for use with MSR_AMD_CPPC_CAP1

// Masks for use with MSR_AMD_CPPC_REQ

// Masks for use with MSR_AMD_CPPC_REQ2

// AMD Performance Counter Global Status and Control MSRs
pub const MSR_AMD64_PERF_CNTR_GLOBAL_STATUS: c_uint = 0xc0000300;
pub const MSR_AMD64_PERF_CNTR_GLOBAL_CTL: c_uint = 0xc0000301;
pub const MSR_AMD64_PERF_CNTR_GLOBAL_STATUS_CLR: c_uint = 0xc0000302;
pub const MSR_AMD64_PERF_CNTR_GLOBAL_STATUS_SET: c_uint = 0xc0000303;
// AMD Hardware Feedback Support MSRs
pub const MSR_AMD_WORKLOAD_CLASS_CONFIG: c_uint = 0xc0000500;
pub const MSR_AMD_WORKLOAD_CLASS_ID: c_uint = 0xc0000501;
pub const MSR_AMD_WORKLOAD_HRST: c_uint = 0xc0000502;
// AMD Last Branch Record MSRs
pub const MSR_AMD64_LBR_SELECT: c_uint = 0xc000010e;
// Zen4
pub const MSR_ZEN4_BP_CFG: c_uint = 0xc001102e;
pub const MSR_ZEN4_BP_CFG_BP_SPEC_REDUCE_BIT: c_int = 4;
pub const MSR_ZEN4_BP_CFG_SHARED_BTB_FIX_BIT: c_int = 5;
pub const MSR_ZEN2_BP_CFG_BUG_FIX_BIT: c_int = 33;
// Fam 19h MSRs
pub const MSR_F19H_UMC_PERF_CTL: c_uint = 0xc0010800;
pub const MSR_F19H_UMC_PERF_CTR: c_uint = 0xc0010801;
// Zen 2
pub const MSR_ZEN2_SPECTRAL_CHICKEN: c_uint = 0xc00110e3;
pub const MSR_ZEN2_SPECTRAL_CHICKEN_BIT: c_int = 1;
// Fam 17h MSRs
pub const MSR_F17H_IRPERF: c_uint = 0xc00000e9;
// Fam 16h MSRs
pub const MSR_F16H_L2I_PERF_CTL: c_uint = 0xc0010230;
pub const MSR_F16H_L2I_PERF_CTR: c_uint = 0xc0010231;
pub const MSR_F16H_DR1_ADDR_MASK: c_uint = 0xc0011019;
pub const MSR_F16H_DR2_ADDR_MASK: c_uint = 0xc001101a;
pub const MSR_F16H_DR3_ADDR_MASK: c_uint = 0xc001101b;
pub const MSR_F16H_DR0_ADDR_MASK: c_uint = 0xc0011027;
// Fam 15h MSRs
pub const MSR_F15H_CU_PWR_ACCUMULATOR: c_uint = 0xc001007a;
pub const MSR_F15H_CU_MAX_PWR_ACCUMULATOR: c_uint = 0xc001007b;
pub const MSR_F15H_PERF_CTL: c_uint = 0xc0010200;

pub const MSR_F15H_PERF_CTR: c_uint = 0xc0010201;

pub const MSR_F15H_NB_PERF_CTL: c_uint = 0xc0010240;
pub const MSR_F15H_NB_PERF_CTR: c_uint = 0xc0010241;
pub const MSR_F15H_PTSC: c_uint = 0xc0010280;
pub const MSR_F15H_IC_CFG: c_uint = 0xc0011021;
pub const MSR_F15H_EX_CFG: c_uint = 0xc001102c;
// Fam 10h MSRs
pub const MSR_FAM10H_MMIO_CONF_BASE: c_uint = 0xc0010058;

pub const FAM10H_MMIO_CONF_BUSRANGE_MASK: c_uint = 0xf;
pub const FAM10H_MMIO_CONF_BUSRANGE_SHIFT: c_int = 2;
pub const FAM10H_MMIO_CONF_BASE_MASK: c_uint = 0xfffffffULL;
pub const FAM10H_MMIO_CONF_BASE_SHIFT: c_int = 20;
pub const MSR_FAM10H_NODE_ID: c_uint = 0xc001100c;
// K8 MSRs
pub const MSR_K8_TOP_MEM1: c_uint = 0xc001001a;
pub const MSR_K8_TOP_MEM2: c_uint = 0xc001001d;
pub const MSR_AMD64_SYSCFG: c_uint = 0xc0010010;
pub const MSR_AMD64_SYSCFG_MEM_ENCRYPT_BIT: c_int = 23;

pub const MSR_AMD64_SYSCFG_SNP_EN_BIT: c_int = 24;

pub const MSR_AMD64_SYSCFG_SNP_VMPL_EN_BIT: c_int = 25;

pub const MSR_AMD64_SYSCFG_MFDM_BIT: c_int = 19;

pub const MSR_K8_INT_PENDING_MSG: c_uint = 0xc0010055;
// C1E active bits in int pending message
pub const K8_INTP_C1E_ACTIVE_MASK: c_uint = 0x18000000;
pub const MSR_K8_TSEG_ADDR: c_uint = 0xc0010112;
pub const MSR_K8_TSEG_MASK: c_uint = 0xc0010113;
pub const K8_MTRRFIXRANGE_DRAM_ENABLE: c_uint = 0x00040000 /* MtrrFixDramEn bit    */;
pub const K8_MTRRFIXRANGE_DRAM_MODIFY: c_uint = 0x00080000 /* MtrrFixDramModEn bit */;
pub const K8_MTRR_RDMEM_WRMEM_MASK: c_uint = 0x18181818 /* Mask: RdMem|WrMem    */;
// K7 MSRs
pub const MSR_K7_EVNTSEL0: c_uint = 0xc0010000;
pub const MSR_K7_PERFCTR0: c_uint = 0xc0010004;
pub const MSR_K7_EVNTSEL1: c_uint = 0xc0010001;
pub const MSR_K7_PERFCTR1: c_uint = 0xc0010005;
pub const MSR_K7_EVNTSEL2: c_uint = 0xc0010002;
pub const MSR_K7_PERFCTR2: c_uint = 0xc0010006;
pub const MSR_K7_EVNTSEL3: c_uint = 0xc0010003;
pub const MSR_K7_PERFCTR3: c_uint = 0xc0010007;
pub const MSR_K7_CLK_CTL: c_uint = 0xc001001b;
pub const MSR_K7_HWCR: c_uint = 0xc0010015;
pub const MSR_K7_HWCR_SMMLOCK_BIT: c_int = 0;

pub const MSR_K7_HWCR_IRPERF_EN_BIT: c_int = 30;

pub const MSR_K7_HWCR_CPUID_USER_DIS_BIT: c_int = 35;

pub const MSR_K7_FID_VID_CTL: c_uint = 0xc0010041;
pub const MSR_K7_FID_VID_STATUS: c_uint = 0xc0010042;
pub const MSR_K7_HWCR_CPB_DIS_BIT: c_int = 25;

// K6 MSRs
pub const MSR_K6_WHCR: c_uint = 0xc0000082;
pub const MSR_K6_UWCCR: c_uint = 0xc0000085;
pub const MSR_K6_EPMR: c_uint = 0xc0000086;
pub const MSR_K6_PSOR: c_uint = 0xc0000087;
pub const MSR_K6_PFIR: c_uint = 0xc0000088;
// Centaur-Hauls/IDT defined MSRs.
pub const MSR_IDT_FCR1: c_uint = 0x00000107;
pub const MSR_IDT_FCR2: c_uint = 0x00000108;
pub const MSR_IDT_FCR3: c_uint = 0x00000109;
pub const MSR_IDT_FCR4: c_uint = 0x0000010a;
pub const MSR_IDT_MCR0: c_uint = 0x00000110;
pub const MSR_IDT_MCR1: c_uint = 0x00000111;
pub const MSR_IDT_MCR2: c_uint = 0x00000112;
pub const MSR_IDT_MCR3: c_uint = 0x00000113;
pub const MSR_IDT_MCR4: c_uint = 0x00000114;
pub const MSR_IDT_MCR5: c_uint = 0x00000115;
pub const MSR_IDT_MCR6: c_uint = 0x00000116;
pub const MSR_IDT_MCR7: c_uint = 0x00000117;
pub const MSR_IDT_MCR_CTRL: c_uint = 0x00000120;
// VIA Cyrix defined MSRs
pub const MSR_VIA_FCR: c_uint = 0x00001107;
pub const MSR_VIA_LONGHAUL: c_uint = 0x0000110a;
pub const MSR_VIA_RNG: c_uint = 0x0000110b;
pub const MSR_VIA_BCR2: c_uint = 0x00001147;
// Transmeta defined MSRs
pub const MSR_TMTA_LONGRUN_CTRL: c_uint = 0x80868010;
pub const MSR_TMTA_LONGRUN_FLAGS: c_uint = 0x80868011;
pub const MSR_TMTA_LRTI_READOUT: c_uint = 0x80868018;
pub const MSR_TMTA_LRTI_VOLT_MHZ: c_uint = 0x8086801a;
// Intel defined MSRs.
pub const MSR_IA32_P5_MC_ADDR: c_uint = 0x00000000;
pub const MSR_IA32_P5_MC_TYPE: c_uint = 0x00000001;
pub const MSR_IA32_TSC: c_uint = 0x00000010;
pub const MSR_IA32_PLATFORM_ID: c_uint = 0x00000017;
pub const MSR_IA32_EBL_CR_POWERON: c_uint = 0x0000002a;
pub const MSR_EBC_FREQUENCY_ID: c_uint = 0x0000002c;
pub const MSR_SMI_COUNT: c_uint = 0x00000034;
// Referred to as IA32_FEATURE_CONTROL in Intel's SDM.
pub const MSR_IA32_FEAT_CTL: c_uint = 0x0000003a;

pub const MSR_IA32_TSC_ADJUST: c_uint = 0x0000003b;
pub const MSR_IA32_BNDCFGS: c_uint = 0x00000d90;
pub const MSR_IA32_BNDCFGS_RSVD: c_uint = 0x00000ffc;
pub const MSR_IA32_XFD: c_uint = 0x000001c4;
pub const MSR_IA32_XFD_ERR: c_uint = 0x000001c5;
pub const MSR_IA32_XSS: c_uint = 0x00000da0;
pub const MSR_IA32_APICBASE: c_uint = 0x0000001b;

pub const MSR_IA32_UCODE_WRITE: c_uint = 0x00000079;
pub const MSR_IA32_MCU_ENUMERATION: c_uint = 0x0000007b;

pub const MSR_IA32_UCODE_REV: c_uint = 0x0000008b;
// Intel SGX Launch Enclave Public Key Hash MSRs
pub const MSR_IA32_SGXLEPUBKEYHASH0: c_uint = 0x0000008C;
pub const MSR_IA32_SGXLEPUBKEYHASH1: c_uint = 0x0000008D;
pub const MSR_IA32_SGXLEPUBKEYHASH2: c_uint = 0x0000008E;
pub const MSR_IA32_SGXLEPUBKEYHASH3: c_uint = 0x0000008F;
pub const MSR_IA32_SMM_MONITOR_CTL: c_uint = 0x0000009b;
pub const MSR_IA32_SMBASE: c_uint = 0x0000009e;
pub const MSR_IA32_PERF_STATUS: c_uint = 0x00000198;
pub const MSR_IA32_PERF_CTL: c_uint = 0x00000199;
pub const INTEL_PERF_CTL_MASK: c_uint = 0xffff;
// AMD Branch Sampling configuration
pub const MSR_AMD_DBG_EXTN_CFG: c_uint = 0xc000010f;
pub const MSR_AMD_SAMP_BR_FROM: c_uint = 0xc0010300;

pub const MSR_IA32_MPERF: c_uint = 0x000000e7;
pub const MSR_IA32_APERF: c_uint = 0x000000e8;
pub const MSR_IA32_THERM_CONTROL: c_uint = 0x0000019a;
pub const MSR_IA32_THERM_INTERRUPT: c_uint = 0x0000019b;

pub const MSR_IA32_THERM_STATUS: c_uint = 0x0000019c;

pub const MSR_THERM2_CTL: c_uint = 0x0000019d;

pub const MSR_IA32_MISC_ENABLE: c_uint = 0x000001a0;
pub const MSR_IA32_TEMPERATURE_TARGET: c_uint = 0x000001a2;
pub const MSR_MISC_FEATURE_CONTROL: c_uint = 0x000001a4;
pub const MSR_MISC_PWR_MGMT: c_uint = 0x000001aa;
pub const MSR_IA32_ENERGY_PERF_BIAS: c_uint = 0x000001b0;
pub const ENERGY_PERF_BIAS_PERFORMANCE: c_int = 0;
pub const ENERGY_PERF_BIAS_BALANCE_PERFORMANCE: c_int = 4;
pub const ENERGY_PERF_BIAS_NORMAL: c_int = 6;
pub const ENERGY_PERF_BIAS_NORMAL_POWERSAVE: c_int = 7;
pub const ENERGY_PERF_BIAS_BALANCE_POWERSAVE: c_int = 8;
pub const ENERGY_PERF_BIAS_POWERSAVE: c_int = 15;
pub const MSR_IA32_PACKAGE_THERM_STATUS: c_uint = 0x000001b1;

pub const MSR_IA32_PACKAGE_THERM_INTERRUPT: c_uint = 0x000001b2;

// Thermal Thresholds Support

pub const THERM_SHIFT_THRESHOLD0: c_int = 8;

pub const THERM_SHIFT_THRESHOLD1: c_int = 16;

// MISC_ENABLE bits: architectural
pub const MSR_IA32_MISC_ENABLE_FAST_STRING_BIT: c_int = 0;

pub const MSR_IA32_MISC_ENABLE_TCC_BIT: c_int = 1;

pub const MSR_IA32_MISC_ENABLE_EMON_BIT: c_int = 7;

pub const MSR_IA32_MISC_ENABLE_BTS_UNAVAIL_BIT: c_int = 11;

pub const MSR_IA32_MISC_ENABLE_PEBS_UNAVAIL_BIT: c_int = 12;

pub const MSR_IA32_MISC_ENABLE_ENHANCED_SPEEDSTEP_BIT: c_int = 16;

pub const MSR_IA32_MISC_ENABLE_MWAIT_BIT: c_int = 18;

pub const MSR_IA32_MISC_ENABLE_LIMIT_CPUID_BIT: c_int = 22;

pub const MSR_IA32_MISC_ENABLE_XTPR_DISABLE_BIT: c_int = 23;

pub const MSR_IA32_MISC_ENABLE_XD_DISABLE_BIT: c_int = 34;

// MISC_ENABLE bits: model-specific, meaning may vary from core to core
pub const MSR_IA32_MISC_ENABLE_X87_COMPAT_BIT: c_int = 2;

pub const MSR_IA32_MISC_ENABLE_TM1_BIT: c_int = 3;

pub const MSR_IA32_MISC_ENABLE_SPLIT_LOCK_DISABLE_BIT: c_int = 4;

pub const MSR_IA32_MISC_ENABLE_L3CACHE_DISABLE_BIT: c_int = 6;

pub const MSR_IA32_MISC_ENABLE_SUPPRESS_LOCK_BIT: c_int = 8;

pub const MSR_IA32_MISC_ENABLE_PREFETCH_DISABLE_BIT: c_int = 9;

pub const MSR_IA32_MISC_ENABLE_FERR_BIT: c_int = 10;

pub const MSR_IA32_MISC_ENABLE_FERR_MULTIPLEX_BIT: c_int = 10;

pub const MSR_IA32_MISC_ENABLE_TM2_BIT: c_int = 13;

pub const MSR_IA32_MISC_ENABLE_ADJ_PREF_DISABLE_BIT: c_int = 19;

pub const MSR_IA32_MISC_ENABLE_SPEEDSTEP_LOCK_BIT: c_int = 20;

pub const MSR_IA32_MISC_ENABLE_L1D_CONTEXT_BIT: c_int = 24;

pub const MSR_IA32_MISC_ENABLE_DCU_PREF_DISABLE_BIT: c_int = 37;

pub const MSR_IA32_MISC_ENABLE_TURBO_DISABLE_BIT: c_int = 38;

pub const MSR_IA32_MISC_ENABLE_IP_PREF_DISABLE_BIT: c_int = 39;

// MISC_FEATURES_ENABLES non-architectural features
pub const MSR_MISC_FEATURES_ENABLES: c_uint = 0x00000140;
pub const MSR_MISC_FEATURES_ENABLES_CPUID_FAULT_BIT: c_int = 0;

pub const MSR_MISC_FEATURES_ENABLES_RING3MWAIT_BIT: c_int = 1;
pub const MSR_IA32_TSC_DEADLINE: c_uint = 0x000006E0;
pub const MSR_TSX_FORCE_ABORT: c_uint = 0x0000010F;
pub const MSR_TFA_RTM_FORCE_ABORT_BIT: c_int = 0;

pub const MSR_TFA_TSX_CPUID_CLEAR_BIT: c_int = 1;

pub const MSR_TFA_SDV_ENABLE_RTM_BIT: c_int = 2;

// P4/Xeon+ specific
pub const MSR_IA32_MCG_EAX: c_uint = 0x00000180;
pub const MSR_IA32_MCG_EBX: c_uint = 0x00000181;
pub const MSR_IA32_MCG_ECX: c_uint = 0x00000182;
pub const MSR_IA32_MCG_EDX: c_uint = 0x00000183;
pub const MSR_IA32_MCG_ESI: c_uint = 0x00000184;
pub const MSR_IA32_MCG_EDI: c_uint = 0x00000185;
pub const MSR_IA32_MCG_EBP: c_uint = 0x00000186;
pub const MSR_IA32_MCG_ESP: c_uint = 0x00000187;
pub const MSR_IA32_MCG_EFLAGS: c_uint = 0x00000188;
pub const MSR_IA32_MCG_EIP: c_uint = 0x00000189;
pub const MSR_IA32_MCG_RESERVED: c_uint = 0x0000018a;
// Pentium IV performance counter MSRs
pub const MSR_P4_BPU_PERFCTR0: c_uint = 0x00000300;
pub const MSR_P4_BPU_PERFCTR1: c_uint = 0x00000301;
pub const MSR_P4_BPU_PERFCTR2: c_uint = 0x00000302;
pub const MSR_P4_BPU_PERFCTR3: c_uint = 0x00000303;
pub const MSR_P4_MS_PERFCTR0: c_uint = 0x00000304;
pub const MSR_P4_MS_PERFCTR1: c_uint = 0x00000305;
pub const MSR_P4_MS_PERFCTR2: c_uint = 0x00000306;
pub const MSR_P4_MS_PERFCTR3: c_uint = 0x00000307;
pub const MSR_P4_FLAME_PERFCTR0: c_uint = 0x00000308;
pub const MSR_P4_FLAME_PERFCTR1: c_uint = 0x00000309;
pub const MSR_P4_FLAME_PERFCTR2: c_uint = 0x0000030a;
pub const MSR_P4_FLAME_PERFCTR3: c_uint = 0x0000030b;
pub const MSR_P4_IQ_PERFCTR0: c_uint = 0x0000030c;
pub const MSR_P4_IQ_PERFCTR1: c_uint = 0x0000030d;
pub const MSR_P4_IQ_PERFCTR2: c_uint = 0x0000030e;
pub const MSR_P4_IQ_PERFCTR3: c_uint = 0x0000030f;
pub const MSR_P4_IQ_PERFCTR4: c_uint = 0x00000310;
pub const MSR_P4_IQ_PERFCTR5: c_uint = 0x00000311;
pub const MSR_P4_BPU_CCCR0: c_uint = 0x00000360;
pub const MSR_P4_BPU_CCCR1: c_uint = 0x00000361;
pub const MSR_P4_BPU_CCCR2: c_uint = 0x00000362;
pub const MSR_P4_BPU_CCCR3: c_uint = 0x00000363;
pub const MSR_P4_MS_CCCR0: c_uint = 0x00000364;
pub const MSR_P4_MS_CCCR1: c_uint = 0x00000365;
pub const MSR_P4_MS_CCCR2: c_uint = 0x00000366;
pub const MSR_P4_MS_CCCR3: c_uint = 0x00000367;
pub const MSR_P4_FLAME_CCCR0: c_uint = 0x00000368;
pub const MSR_P4_FLAME_CCCR1: c_uint = 0x00000369;
pub const MSR_P4_FLAME_CCCR2: c_uint = 0x0000036a;
pub const MSR_P4_FLAME_CCCR3: c_uint = 0x0000036b;
pub const MSR_P4_IQ_CCCR0: c_uint = 0x0000036c;
pub const MSR_P4_IQ_CCCR1: c_uint = 0x0000036d;
pub const MSR_P4_IQ_CCCR2: c_uint = 0x0000036e;
pub const MSR_P4_IQ_CCCR3: c_uint = 0x0000036f;
pub const MSR_P4_IQ_CCCR4: c_uint = 0x00000370;
pub const MSR_P4_IQ_CCCR5: c_uint = 0x00000371;
pub const MSR_P4_ALF_ESCR0: c_uint = 0x000003ca;
pub const MSR_P4_ALF_ESCR1: c_uint = 0x000003cb;
pub const MSR_P4_BPU_ESCR0: c_uint = 0x000003b2;
pub const MSR_P4_BPU_ESCR1: c_uint = 0x000003b3;
pub const MSR_P4_BSU_ESCR0: c_uint = 0x000003a0;
pub const MSR_P4_BSU_ESCR1: c_uint = 0x000003a1;
pub const MSR_P4_CRU_ESCR0: c_uint = 0x000003b8;
pub const MSR_P4_CRU_ESCR1: c_uint = 0x000003b9;
pub const MSR_P4_CRU_ESCR2: c_uint = 0x000003cc;
pub const MSR_P4_CRU_ESCR3: c_uint = 0x000003cd;
pub const MSR_P4_CRU_ESCR4: c_uint = 0x000003e0;
pub const MSR_P4_CRU_ESCR5: c_uint = 0x000003e1;
pub const MSR_P4_DAC_ESCR0: c_uint = 0x000003a8;
pub const MSR_P4_DAC_ESCR1: c_uint = 0x000003a9;
pub const MSR_P4_FIRM_ESCR0: c_uint = 0x000003a4;
pub const MSR_P4_FIRM_ESCR1: c_uint = 0x000003a5;
pub const MSR_P4_FLAME_ESCR0: c_uint = 0x000003a6;
pub const MSR_P4_FLAME_ESCR1: c_uint = 0x000003a7;
pub const MSR_P4_FSB_ESCR0: c_uint = 0x000003a2;
pub const MSR_P4_FSB_ESCR1: c_uint = 0x000003a3;
pub const MSR_P4_IQ_ESCR0: c_uint = 0x000003ba;
pub const MSR_P4_IQ_ESCR1: c_uint = 0x000003bb;
pub const MSR_P4_IS_ESCR0: c_uint = 0x000003b4;
pub const MSR_P4_IS_ESCR1: c_uint = 0x000003b5;
pub const MSR_P4_ITLB_ESCR0: c_uint = 0x000003b6;
pub const MSR_P4_ITLB_ESCR1: c_uint = 0x000003b7;
pub const MSR_P4_IX_ESCR0: c_uint = 0x000003c8;
pub const MSR_P4_IX_ESCR1: c_uint = 0x000003c9;
pub const MSR_P4_MOB_ESCR0: c_uint = 0x000003aa;
pub const MSR_P4_MOB_ESCR1: c_uint = 0x000003ab;
pub const MSR_P4_MS_ESCR0: c_uint = 0x000003c0;
pub const MSR_P4_MS_ESCR1: c_uint = 0x000003c1;
pub const MSR_P4_PMH_ESCR0: c_uint = 0x000003ac;
pub const MSR_P4_PMH_ESCR1: c_uint = 0x000003ad;
pub const MSR_P4_RAT_ESCR0: c_uint = 0x000003bc;
pub const MSR_P4_RAT_ESCR1: c_uint = 0x000003bd;
pub const MSR_P4_SAAT_ESCR0: c_uint = 0x000003ae;
pub const MSR_P4_SAAT_ESCR1: c_uint = 0x000003af;
pub const MSR_P4_SSU_ESCR0: c_uint = 0x000003be;
pub const MSR_P4_SSU_ESCR1: c_uint = 0x000003bf /* guess: not in manual */;
pub const MSR_P4_TBPU_ESCR0: c_uint = 0x000003c2;
pub const MSR_P4_TBPU_ESCR1: c_uint = 0x000003c3;
pub const MSR_P4_TC_ESCR0: c_uint = 0x000003c4;
pub const MSR_P4_TC_ESCR1: c_uint = 0x000003c5;
pub const MSR_P4_U2L_ESCR0: c_uint = 0x000003b0;
pub const MSR_P4_U2L_ESCR1: c_uint = 0x000003b1;
pub const MSR_P4_PEBS_MATRIX_VERT: c_uint = 0x000003f2;
// Intel Core-based CPU performance counters
pub const MSR_CORE_PERF_FIXED_CTR0: c_uint = 0x00000309;
pub const MSR_CORE_PERF_FIXED_CTR1: c_uint = 0x0000030a;
pub const MSR_CORE_PERF_FIXED_CTR2: c_uint = 0x0000030b;
pub const MSR_CORE_PERF_FIXED_CTR3: c_uint = 0x0000030c;
pub const MSR_CORE_PERF_FIXED_CTR_CTRL: c_uint = 0x0000038d;
pub const MSR_CORE_PERF_GLOBAL_STATUS: c_uint = 0x0000038e;
pub const MSR_CORE_PERF_GLOBAL_CTRL: c_uint = 0x0000038f;
pub const MSR_CORE_PERF_GLOBAL_OVF_CTRL: c_uint = 0x00000390;
pub const MSR_CORE_PERF_GLOBAL_STATUS_SET: c_uint = 0x00000391;
pub const MSR_PERF_METRICS: c_uint = 0x00000329;
// PERF_GLOBAL_OVF_CTL bits
pub const MSR_CORE_PERF_GLOBAL_OVF_CTRL_TRACE_TOPA_PMI_BIT: c_int = 55;

pub const MSR_CORE_PERF_GLOBAL_OVF_CTRL_OVF_BUF_BIT: c_int = 62;

pub const MSR_CORE_PERF_GLOBAL_OVF_CTRL_COND_CHGD_BIT: c_int = 63;

// Geode defined MSRs
pub const MSR_GEODE_BUSCONT_CONF0: c_uint = 0x00001900;
// Intel VT MSRs
pub const MSR_IA32_VMX_BASIC: c_uint = 0x00000480;
pub const MSR_IA32_VMX_PINBASED_CTLS: c_uint = 0x00000481;
pub const MSR_IA32_VMX_PROCBASED_CTLS: c_uint = 0x00000482;
pub const MSR_IA32_VMX_EXIT_CTLS: c_uint = 0x00000483;
pub const MSR_IA32_VMX_ENTRY_CTLS: c_uint = 0x00000484;
pub const MSR_IA32_VMX_MISC: c_uint = 0x00000485;
pub const MSR_IA32_VMX_CR0_FIXED0: c_uint = 0x00000486;
pub const MSR_IA32_VMX_CR0_FIXED1: c_uint = 0x00000487;
pub const MSR_IA32_VMX_CR4_FIXED0: c_uint = 0x00000488;
pub const MSR_IA32_VMX_CR4_FIXED1: c_uint = 0x00000489;
pub const MSR_IA32_VMX_VMCS_ENUM: c_uint = 0x0000048a;
pub const MSR_IA32_VMX_PROCBASED_CTLS2: c_uint = 0x0000048b;
pub const MSR_IA32_VMX_EPT_VPID_CAP: c_uint = 0x0000048c;
pub const MSR_IA32_VMX_TRUE_PINBASED_CTLS: c_uint = 0x0000048d;
pub const MSR_IA32_VMX_TRUE_PROCBASED_CTLS: c_uint = 0x0000048e;
pub const MSR_IA32_VMX_TRUE_EXIT_CTLS: c_uint = 0x0000048f;
pub const MSR_IA32_VMX_TRUE_ENTRY_CTLS: c_uint = 0x00000490;
pub const MSR_IA32_VMX_VMFUNC: c_uint = 0x00000491;
pub const MSR_IA32_VMX_PROCBASED_CTLS3: c_uint = 0x00000492;
pub const MSR_IA32_MCU_STAGING_MBOX_ADDR: c_uint = 0x000007a5;
// Resctrl MSRs:
// - Intel:
pub const MSR_IA32_L3_QOS_CFG: c_uint = 0xc81;
pub const MSR_IA32_L2_QOS_CFG: c_uint = 0xc82;
pub const MSR_IA32_QM_EVTSEL: c_uint = 0xc8d;
pub const MSR_IA32_QM_CTR: c_uint = 0xc8e;
pub const MSR_IA32_PQR_ASSOC: c_uint = 0xc8f;
pub const MSR_IA32_L3_CBM_BASE: c_uint = 0xc90;
pub const MSR_RMID_SNC_CONFIG: c_uint = 0xca0;
pub const MSR_IA32_L2_CBM_BASE: c_uint = 0xd10;
pub const MSR_IA32_MBA_THRTL_BASE: c_uint = 0xd50;
// - AMD:
pub const MSR_IA32_MBA_BW_BASE: c_uint = 0xc0000200;
pub const MSR_IA32_SMBA_BW_BASE: c_uint = 0xc0000280;
pub const MSR_IA32_L3_QOS_ABMC_CFG: c_uint = 0xc00003fd;
pub const MSR_IA32_L3_QOS_EXT_CFG: c_uint = 0xc00003ff;
pub const MSR_IA32_EVT_CFG_BASE: c_uint = 0xc0000400;
// AMD-V MSRs
pub const MSR_VM_CR: c_uint = 0xc0010114;
pub const MSR_VM_IGNNE: c_uint = 0xc0010115;
pub const MSR_VM_HSAVE_PA: c_uint = 0xc0010117;
pub const SVM_VM_CR_VALID_MASK: c_uint = 0x001fULL;
pub const SVM_VM_CR_SVM_LOCK_MASK: c_uint = 0x0008ULL;
pub const SVM_VM_CR_SVM_DIS_MASK: c_uint = 0x0010ULL;
// Hardware Feedback Interface
pub const MSR_IA32_HW_FEEDBACK_PTR: c_uint = 0x17d0;
pub const MSR_IA32_HW_FEEDBACK_CONFIG: c_uint = 0x17d1;
// x2APIC locked status
pub const MSR_IA32_XAPIC_DISABLE_STATUS: c_uint = 0xBD;

// x2APIC mode is locked and
// disabling x2APIC will cause
// a #GP
//
