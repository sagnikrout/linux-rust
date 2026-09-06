//! Automatically rewritten from C Header to Rust Module
//! Source: tools/include/perf/arm_pmuv3.h
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
// Copyright (C) 2012 ARM Ltd.
//

pub const ARMV8_PMU_MAX_COUNTERS: c_int = 32;

//
// Common architectural and microarchitectural event numbers.
//
pub const ARMV8_PMUV3_PERFCTR_SW_INCR: c_uint = 0x0000;
pub const ARMV8_PMUV3_PERFCTR_L1I_CACHE_REFILL: c_uint = 0x0001;
pub const ARMV8_PMUV3_PERFCTR_L1I_TLB_REFILL: c_uint = 0x0002;
pub const ARMV8_PMUV3_PERFCTR_L1D_CACHE_REFILL: c_uint = 0x0003;
pub const ARMV8_PMUV3_PERFCTR_L1D_CACHE: c_uint = 0x0004;
pub const ARMV8_PMUV3_PERFCTR_L1D_TLB_REFILL: c_uint = 0x0005;
pub const ARMV8_PMUV3_PERFCTR_LD_RETIRED: c_uint = 0x0006;
pub const ARMV8_PMUV3_PERFCTR_ST_RETIRED: c_uint = 0x0007;
pub const ARMV8_PMUV3_PERFCTR_INST_RETIRED: c_uint = 0x0008;
pub const ARMV8_PMUV3_PERFCTR_EXC_TAKEN: c_uint = 0x0009;
pub const ARMV8_PMUV3_PERFCTR_EXC_RETURN: c_uint = 0x000A;
pub const ARMV8_PMUV3_PERFCTR_CID_WRITE_RETIRED: c_uint = 0x000B;
pub const ARMV8_PMUV3_PERFCTR_PC_WRITE_RETIRED: c_uint = 0x000C;
pub const ARMV8_PMUV3_PERFCTR_BR_IMMED_RETIRED: c_uint = 0x000D;
pub const ARMV8_PMUV3_PERFCTR_BR_RETURN_RETIRED: c_uint = 0x000E;
pub const ARMV8_PMUV3_PERFCTR_UNALIGNED_LDST_RETIRED: c_uint = 0x000F;
pub const ARMV8_PMUV3_PERFCTR_BR_MIS_PRED: c_uint = 0x0010;
pub const ARMV8_PMUV3_PERFCTR_CPU_CYCLES: c_uint = 0x0011;
pub const ARMV8_PMUV3_PERFCTR_BR_PRED: c_uint = 0x0012;
pub const ARMV8_PMUV3_PERFCTR_MEM_ACCESS: c_uint = 0x0013;
pub const ARMV8_PMUV3_PERFCTR_L1I_CACHE: c_uint = 0x0014;
pub const ARMV8_PMUV3_PERFCTR_L1D_CACHE_WB: c_uint = 0x0015;
pub const ARMV8_PMUV3_PERFCTR_L2D_CACHE: c_uint = 0x0016;
pub const ARMV8_PMUV3_PERFCTR_L2D_CACHE_REFILL: c_uint = 0x0017;
pub const ARMV8_PMUV3_PERFCTR_L2D_CACHE_WB: c_uint = 0x0018;
pub const ARMV8_PMUV3_PERFCTR_BUS_ACCESS: c_uint = 0x0019;
pub const ARMV8_PMUV3_PERFCTR_MEMORY_ERROR: c_uint = 0x001A;
pub const ARMV8_PMUV3_PERFCTR_INST_SPEC: c_uint = 0x001B;
pub const ARMV8_PMUV3_PERFCTR_TTBR_WRITE_RETIRED: c_uint = 0x001C;
pub const ARMV8_PMUV3_PERFCTR_BUS_CYCLES: c_uint = 0x001D;
pub const ARMV8_PMUV3_PERFCTR_CHAIN: c_uint = 0x001E;
pub const ARMV8_PMUV3_PERFCTR_L1D_CACHE_ALLOCATE: c_uint = 0x001F;
pub const ARMV8_PMUV3_PERFCTR_L2D_CACHE_ALLOCATE: c_uint = 0x0020;
pub const ARMV8_PMUV3_PERFCTR_BR_RETIRED: c_uint = 0x0021;
pub const ARMV8_PMUV3_PERFCTR_BR_MIS_PRED_RETIRED: c_uint = 0x0022;
pub const ARMV8_PMUV3_PERFCTR_STALL_FRONTEND: c_uint = 0x0023;
pub const ARMV8_PMUV3_PERFCTR_STALL_BACKEND: c_uint = 0x0024;
pub const ARMV8_PMUV3_PERFCTR_L1D_TLB: c_uint = 0x0025;
pub const ARMV8_PMUV3_PERFCTR_L1I_TLB: c_uint = 0x0026;
pub const ARMV8_PMUV3_PERFCTR_L2I_CACHE: c_uint = 0x0027;
pub const ARMV8_PMUV3_PERFCTR_L2I_CACHE_REFILL: c_uint = 0x0028;
pub const ARMV8_PMUV3_PERFCTR_L3D_CACHE_ALLOCATE: c_uint = 0x0029;
pub const ARMV8_PMUV3_PERFCTR_L3D_CACHE_REFILL: c_uint = 0x002A;
pub const ARMV8_PMUV3_PERFCTR_L3D_CACHE: c_uint = 0x002B;
pub const ARMV8_PMUV3_PERFCTR_L3D_CACHE_WB: c_uint = 0x002C;
pub const ARMV8_PMUV3_PERFCTR_L2D_TLB_REFILL: c_uint = 0x002D;
pub const ARMV8_PMUV3_PERFCTR_L2I_TLB_REFILL: c_uint = 0x002E;
pub const ARMV8_PMUV3_PERFCTR_L2D_TLB: c_uint = 0x002F;
pub const ARMV8_PMUV3_PERFCTR_L2I_TLB: c_uint = 0x0030;
pub const ARMV8_PMUV3_PERFCTR_REMOTE_ACCESS: c_uint = 0x0031;
pub const ARMV8_PMUV3_PERFCTR_LL_CACHE: c_uint = 0x0032;
pub const ARMV8_PMUV3_PERFCTR_LL_CACHE_MISS: c_uint = 0x0033;
pub const ARMV8_PMUV3_PERFCTR_DTLB_WALK: c_uint = 0x0034;
pub const ARMV8_PMUV3_PERFCTR_ITLB_WALK: c_uint = 0x0035;
pub const ARMV8_PMUV3_PERFCTR_LL_CACHE_RD: c_uint = 0x0036;
pub const ARMV8_PMUV3_PERFCTR_LL_CACHE_MISS_RD: c_uint = 0x0037;
pub const ARMV8_PMUV3_PERFCTR_REMOTE_ACCESS_RD: c_uint = 0x0038;
pub const ARMV8_PMUV3_PERFCTR_L1D_CACHE_LMISS_RD: c_uint = 0x0039;
pub const ARMV8_PMUV3_PERFCTR_OP_RETIRED: c_uint = 0x003A;
pub const ARMV8_PMUV3_PERFCTR_OP_SPEC: c_uint = 0x003B;
pub const ARMV8_PMUV3_PERFCTR_STALL: c_uint = 0x003C;
pub const ARMV8_PMUV3_PERFCTR_STALL_SLOT_BACKEND: c_uint = 0x003D;
pub const ARMV8_PMUV3_PERFCTR_STALL_SLOT_FRONTEND: c_uint = 0x003E;
pub const ARMV8_PMUV3_PERFCTR_STALL_SLOT: c_uint = 0x003F;
// Statistical profiling extension microarchitectural events
pub const ARMV8_SPE_PERFCTR_SAMPLE_POP: c_uint = 0x4000;
pub const ARMV8_SPE_PERFCTR_SAMPLE_FEED: c_uint = 0x4001;
pub const ARMV8_SPE_PERFCTR_SAMPLE_FILTRATE: c_uint = 0x4002;
pub const ARMV8_SPE_PERFCTR_SAMPLE_COLLISION: c_uint = 0x4003;
// AMUv1 architecture events
pub const ARMV8_AMU_PERFCTR_CNT_CYCLES: c_uint = 0x4004;
pub const ARMV8_AMU_PERFCTR_STALL_BACKEND_MEM: c_uint = 0x4005;
// long-latency read miss events
pub const ARMV8_PMUV3_PERFCTR_L1I_CACHE_LMISS: c_uint = 0x4006;
pub const ARMV8_PMUV3_PERFCTR_L2D_CACHE_LMISS_RD: c_uint = 0x4009;
pub const ARMV8_PMUV3_PERFCTR_L2I_CACHE_LMISS: c_uint = 0x400A;
pub const ARMV8_PMUV3_PERFCTR_L3D_CACHE_LMISS_RD: c_uint = 0x400B;
// Trace buffer events
pub const ARMV8_PMUV3_PERFCTR_TRB_WRAP: c_uint = 0x400C;
pub const ARMV8_PMUV3_PERFCTR_TRB_TRIG: c_uint = 0x400E;
// Trace unit events
pub const ARMV8_PMUV3_PERFCTR_TRCEXTOUT0: c_uint = 0x4010;
pub const ARMV8_PMUV3_PERFCTR_TRCEXTOUT1: c_uint = 0x4011;
pub const ARMV8_PMUV3_PERFCTR_TRCEXTOUT2: c_uint = 0x4012;
pub const ARMV8_PMUV3_PERFCTR_TRCEXTOUT3: c_uint = 0x4013;
pub const ARMV8_PMUV3_PERFCTR_CTI_TRIGOUT4: c_uint = 0x4018;
pub const ARMV8_PMUV3_PERFCTR_CTI_TRIGOUT5: c_uint = 0x4019;
pub const ARMV8_PMUV3_PERFCTR_CTI_TRIGOUT6: c_uint = 0x401A;
pub const ARMV8_PMUV3_PERFCTR_CTI_TRIGOUT7: c_uint = 0x401B;
// additional latency from alignment events
pub const ARMV8_PMUV3_PERFCTR_LDST_ALIGN_LAT: c_uint = 0x4020;
pub const ARMV8_PMUV3_PERFCTR_LD_ALIGN_LAT: c_uint = 0x4021;
pub const ARMV8_PMUV3_PERFCTR_ST_ALIGN_LAT: c_uint = 0x4022;
// Armv8.5 Memory Tagging Extension events
pub const ARMV8_MTE_PERFCTR_MEM_ACCESS_CHECKED: c_uint = 0x4024;
pub const ARMV8_MTE_PERFCTR_MEM_ACCESS_CHECKED_RD: c_uint = 0x4025;
pub const ARMV8_MTE_PERFCTR_MEM_ACCESS_CHECKED_WR: c_uint = 0x4026;
// ARMv8 recommended implementation defined event types
pub const ARMV8_IMPDEF_PERFCTR_L1D_CACHE_RD: c_uint = 0x0040;
pub const ARMV8_IMPDEF_PERFCTR_L1D_CACHE_WR: c_uint = 0x0041;
pub const ARMV8_IMPDEF_PERFCTR_L1D_CACHE_REFILL_RD: c_uint = 0x0042;
pub const ARMV8_IMPDEF_PERFCTR_L1D_CACHE_REFILL_WR: c_uint = 0x0043;
pub const ARMV8_IMPDEF_PERFCTR_L1D_CACHE_REFILL_INNER: c_uint = 0x0044;
pub const ARMV8_IMPDEF_PERFCTR_L1D_CACHE_REFILL_OUTER: c_uint = 0x0045;
pub const ARMV8_IMPDEF_PERFCTR_L1D_CACHE_WB_VICTIM: c_uint = 0x0046;
pub const ARMV8_IMPDEF_PERFCTR_L1D_CACHE_WB_CLEAN: c_uint = 0x0047;
pub const ARMV8_IMPDEF_PERFCTR_L1D_CACHE_INVAL: c_uint = 0x0048;
pub const ARMV8_IMPDEF_PERFCTR_L1D_TLB_REFILL_RD: c_uint = 0x004C;
pub const ARMV8_IMPDEF_PERFCTR_L1D_TLB_REFILL_WR: c_uint = 0x004D;
pub const ARMV8_IMPDEF_PERFCTR_L1D_TLB_RD: c_uint = 0x004E;
pub const ARMV8_IMPDEF_PERFCTR_L1D_TLB_WR: c_uint = 0x004F;
pub const ARMV8_IMPDEF_PERFCTR_L2D_CACHE_RD: c_uint = 0x0050;
pub const ARMV8_IMPDEF_PERFCTR_L2D_CACHE_WR: c_uint = 0x0051;
pub const ARMV8_IMPDEF_PERFCTR_L2D_CACHE_REFILL_RD: c_uint = 0x0052;
pub const ARMV8_IMPDEF_PERFCTR_L2D_CACHE_REFILL_WR: c_uint = 0x0053;
pub const ARMV8_IMPDEF_PERFCTR_L2D_CACHE_WB_VICTIM: c_uint = 0x0056;
pub const ARMV8_IMPDEF_PERFCTR_L2D_CACHE_WB_CLEAN: c_uint = 0x0057;
pub const ARMV8_IMPDEF_PERFCTR_L2D_CACHE_INVAL: c_uint = 0x0058;
pub const ARMV8_IMPDEF_PERFCTR_L2D_TLB_REFILL_RD: c_uint = 0x005C;
pub const ARMV8_IMPDEF_PERFCTR_L2D_TLB_REFILL_WR: c_uint = 0x005D;
pub const ARMV8_IMPDEF_PERFCTR_L2D_TLB_RD: c_uint = 0x005E;
pub const ARMV8_IMPDEF_PERFCTR_L2D_TLB_WR: c_uint = 0x005F;
pub const ARMV8_IMPDEF_PERFCTR_BUS_ACCESS_RD: c_uint = 0x0060;
pub const ARMV8_IMPDEF_PERFCTR_BUS_ACCESS_WR: c_uint = 0x0061;
pub const ARMV8_IMPDEF_PERFCTR_BUS_ACCESS_SHARED: c_uint = 0x0062;
pub const ARMV8_IMPDEF_PERFCTR_BUS_ACCESS_NOT_SHARED: c_uint = 0x0063;
pub const ARMV8_IMPDEF_PERFCTR_BUS_ACCESS_NORMAL: c_uint = 0x0064;
pub const ARMV8_IMPDEF_PERFCTR_BUS_ACCESS_PERIPH: c_uint = 0x0065;
pub const ARMV8_IMPDEF_PERFCTR_MEM_ACCESS_RD: c_uint = 0x0066;
pub const ARMV8_IMPDEF_PERFCTR_MEM_ACCESS_WR: c_uint = 0x0067;
pub const ARMV8_IMPDEF_PERFCTR_UNALIGNED_LD_SPEC: c_uint = 0x0068;
pub const ARMV8_IMPDEF_PERFCTR_UNALIGNED_ST_SPEC: c_uint = 0x0069;
pub const ARMV8_IMPDEF_PERFCTR_UNALIGNED_LDST_SPEC: c_uint = 0x006A;
pub const ARMV8_IMPDEF_PERFCTR_LDREX_SPEC: c_uint = 0x006C;
pub const ARMV8_IMPDEF_PERFCTR_STREX_PASS_SPEC: c_uint = 0x006D;
pub const ARMV8_IMPDEF_PERFCTR_STREX_FAIL_SPEC: c_uint = 0x006E;
pub const ARMV8_IMPDEF_PERFCTR_STREX_SPEC: c_uint = 0x006F;
pub const ARMV8_IMPDEF_PERFCTR_LD_SPEC: c_uint = 0x0070;
pub const ARMV8_IMPDEF_PERFCTR_ST_SPEC: c_uint = 0x0071;
pub const ARMV8_IMPDEF_PERFCTR_LDST_SPEC: c_uint = 0x0072;
pub const ARMV8_IMPDEF_PERFCTR_DP_SPEC: c_uint = 0x0073;
pub const ARMV8_IMPDEF_PERFCTR_ASE_SPEC: c_uint = 0x0074;
pub const ARMV8_IMPDEF_PERFCTR_VFP_SPEC: c_uint = 0x0075;
pub const ARMV8_IMPDEF_PERFCTR_PC_WRITE_SPEC: c_uint = 0x0076;
pub const ARMV8_IMPDEF_PERFCTR_CRYPTO_SPEC: c_uint = 0x0077;
pub const ARMV8_IMPDEF_PERFCTR_BR_IMMED_SPEC: c_uint = 0x0078;
pub const ARMV8_IMPDEF_PERFCTR_BR_RETURN_SPEC: c_uint = 0x0079;
pub const ARMV8_IMPDEF_PERFCTR_BR_INDIRECT_SPEC: c_uint = 0x007A;
pub const ARMV8_IMPDEF_PERFCTR_ISB_SPEC: c_uint = 0x007C;
pub const ARMV8_IMPDEF_PERFCTR_DSB_SPEC: c_uint = 0x007D;
pub const ARMV8_IMPDEF_PERFCTR_DMB_SPEC: c_uint = 0x007E;
pub const ARMV8_IMPDEF_PERFCTR_EXC_UNDEF: c_uint = 0x0081;
pub const ARMV8_IMPDEF_PERFCTR_EXC_SVC: c_uint = 0x0082;
pub const ARMV8_IMPDEF_PERFCTR_EXC_PABORT: c_uint = 0x0083;
pub const ARMV8_IMPDEF_PERFCTR_EXC_DABORT: c_uint = 0x0084;
pub const ARMV8_IMPDEF_PERFCTR_EXC_IRQ: c_uint = 0x0086;
pub const ARMV8_IMPDEF_PERFCTR_EXC_FIQ: c_uint = 0x0087;
pub const ARMV8_IMPDEF_PERFCTR_EXC_SMC: c_uint = 0x0088;
pub const ARMV8_IMPDEF_PERFCTR_EXC_HVC: c_uint = 0x008A;
pub const ARMV8_IMPDEF_PERFCTR_EXC_TRAP_PABORT: c_uint = 0x008B;
pub const ARMV8_IMPDEF_PERFCTR_EXC_TRAP_DABORT: c_uint = 0x008C;
pub const ARMV8_IMPDEF_PERFCTR_EXC_TRAP_OTHER: c_uint = 0x008D;
pub const ARMV8_IMPDEF_PERFCTR_EXC_TRAP_IRQ: c_uint = 0x008E;
pub const ARMV8_IMPDEF_PERFCTR_EXC_TRAP_FIQ: c_uint = 0x008F;
pub const ARMV8_IMPDEF_PERFCTR_RC_LD_SPEC: c_uint = 0x0090;
pub const ARMV8_IMPDEF_PERFCTR_RC_ST_SPEC: c_uint = 0x0091;
pub const ARMV8_IMPDEF_PERFCTR_L3D_CACHE_RD: c_uint = 0x00A0;
pub const ARMV8_IMPDEF_PERFCTR_L3D_CACHE_WR: c_uint = 0x00A1;
pub const ARMV8_IMPDEF_PERFCTR_L3D_CACHE_REFILL_RD: c_uint = 0x00A2;
pub const ARMV8_IMPDEF_PERFCTR_L3D_CACHE_REFILL_WR: c_uint = 0x00A3;
pub const ARMV8_IMPDEF_PERFCTR_L3D_CACHE_WB_VICTIM: c_uint = 0x00A6;
pub const ARMV8_IMPDEF_PERFCTR_L3D_CACHE_WB_CLEAN: c_uint = 0x00A7;
pub const ARMV8_IMPDEF_PERFCTR_L3D_CACHE_INVAL: c_uint = 0x00A8;
//
// Per-CPU PMCR: config reg
//

// Mask for writable bits

//
// PMOVSR: counters overflow flag status reg
//

// Mask for writable bits is both P and C fields

//
// PMXEVTYPER: Event selection reg
//

//
// Event filters for PMUv3
//

//
// PMUSERENR: user enable reg
//

// Mask for writable bits

// PMMIR_EL1.SLOTS mask

//
// This code is really good
//

