//! Automatically rewritten from C to Rust
//! Source: drivers/perf/arm_v7_pmu.c
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
// ARMv7 Cortex-A8 and Cortex-A9 Performance Events handling code.
//
// ARMv7 support: Jean Pihet <jpihet@mvista.com>
// 2010 (c) MontaVista Software, LLC.
//
// Copied from ARMv6 code, with the low level code inspired
// by the ARMv7 Oprofile code.
//
// Cortex-A8 has up to 4 configurable performance counters and
// a single cycle counter.
// Cortex-A9 has up to 31 configurable performance counters and
// a single cycle counter.
//
// All counters can be enabled/disabled and IRQ masked separately. The cycle
// counter and all 4 performance counters together can be reset separately.
//

//
// Common ARMv7 event types
//
// Note: An implementation may not be able to count all of these events
// but the encodings are considered to be `reserved' in the case that
// they are not available.
//
pub const ARMV7_PERFCTR_PMNC_SW_INCR: c_uint = 0x00;
pub const ARMV7_PERFCTR_L1_ICACHE_REFILL: c_uint = 0x01;
pub const ARMV7_PERFCTR_ITLB_REFILL: c_uint = 0x02;
pub const ARMV7_PERFCTR_L1_DCACHE_REFILL: c_uint = 0x03;
pub const ARMV7_PERFCTR_L1_DCACHE_ACCESS: c_uint = 0x04;
pub const ARMV7_PERFCTR_DTLB_REFILL: c_uint = 0x05;
pub const ARMV7_PERFCTR_MEM_READ: c_uint = 0x06;
pub const ARMV7_PERFCTR_MEM_WRITE: c_uint = 0x07;
pub const ARMV7_PERFCTR_INSTR_EXECUTED: c_uint = 0x08;
pub const ARMV7_PERFCTR_EXC_TAKEN: c_uint = 0x09;
pub const ARMV7_PERFCTR_EXC_EXECUTED: c_uint = 0x0A;
pub const ARMV7_PERFCTR_CID_WRITE: c_uint = 0x0B;
//
// ARMV7_PERFCTR_PC_WRITE is equivalent to HW_BRANCH_INSTRUCTIONS.
// It counts:
// - all (taken) branch instructions,
// - instructions that explicitly write the PC,
// - exception generating instructions.
//
pub const ARMV7_PERFCTR_PC_WRITE: c_uint = 0x0C;
pub const ARMV7_PERFCTR_PC_IMM_BRANCH: c_uint = 0x0D;
pub const ARMV7_PERFCTR_PC_PROC_RETURN: c_uint = 0x0E;
pub const ARMV7_PERFCTR_MEM_UNALIGNED_ACCESS: c_uint = 0x0F;
pub const ARMV7_PERFCTR_PC_BRANCH_MIS_PRED: c_uint = 0x10;
pub const ARMV7_PERFCTR_CLOCK_CYCLES: c_uint = 0x11;
pub const ARMV7_PERFCTR_PC_BRANCH_PRED: c_uint = 0x12;
// These events are defined by the PMUv2 supplement (ARM DDI 0457A).
pub const ARMV7_PERFCTR_MEM_ACCESS: c_uint = 0x13;
pub const ARMV7_PERFCTR_L1_ICACHE_ACCESS: c_uint = 0x14;
pub const ARMV7_PERFCTR_L1_DCACHE_WB: c_uint = 0x15;
pub const ARMV7_PERFCTR_L2_CACHE_ACCESS: c_uint = 0x16;
pub const ARMV7_PERFCTR_L2_CACHE_REFILL: c_uint = 0x17;
pub const ARMV7_PERFCTR_L2_CACHE_WB: c_uint = 0x18;
pub const ARMV7_PERFCTR_BUS_ACCESS: c_uint = 0x19;
pub const ARMV7_PERFCTR_MEM_ERROR: c_uint = 0x1A;
pub const ARMV7_PERFCTR_INSTR_SPEC: c_uint = 0x1B;
pub const ARMV7_PERFCTR_TTBR_WRITE: c_uint = 0x1C;
pub const ARMV7_PERFCTR_BUS_CYCLES: c_uint = 0x1D;
pub const ARMV7_PERFCTR_CPU_CYCLES: c_uint = 0xFF;
// ARMv7 Cortex-A8 specific event types
pub const ARMV7_A8_PERFCTR_L2_CACHE_ACCESS: c_uint = 0x43;
pub const ARMV7_A8_PERFCTR_L2_CACHE_REFILL: c_uint = 0x44;
pub const ARMV7_A8_PERFCTR_L1_ICACHE_ACCESS: c_uint = 0x50;
pub const ARMV7_A8_PERFCTR_STALL_ISIDE: c_uint = 0x56;
// ARMv7 Cortex-A9 specific event types
pub const ARMV7_A9_PERFCTR_INSTR_CORE_RENAME: c_uint = 0x68;
pub const ARMV7_A9_PERFCTR_STALL_ICACHE: c_uint = 0x60;
pub const ARMV7_A9_PERFCTR_STALL_DISPATCH: c_uint = 0x66;
// ARMv7 Cortex-A5 specific event types
pub const ARMV7_A5_PERFCTR_PREFETCH_LINEFILL: c_uint = 0xc2;
pub const ARMV7_A5_PERFCTR_PREFETCH_LINEFILL_DROP: c_uint = 0xc3;
// ARMv7 Cortex-A15 specific event types
pub const ARMV7_A15_PERFCTR_L1_DCACHE_ACCESS_READ: c_uint = 0x40;
pub const ARMV7_A15_PERFCTR_L1_DCACHE_ACCESS_WRITE: c_uint = 0x41;
pub const ARMV7_A15_PERFCTR_L1_DCACHE_REFILL_READ: c_uint = 0x42;
pub const ARMV7_A15_PERFCTR_L1_DCACHE_REFILL_WRITE: c_uint = 0x43;
pub const ARMV7_A15_PERFCTR_DTLB_REFILL_L1_READ: c_uint = 0x4C;
pub const ARMV7_A15_PERFCTR_DTLB_REFILL_L1_WRITE: c_uint = 0x4D;
pub const ARMV7_A15_PERFCTR_L2_CACHE_ACCESS_READ: c_uint = 0x50;
pub const ARMV7_A15_PERFCTR_L2_CACHE_ACCESS_WRITE: c_uint = 0x51;
pub const ARMV7_A15_PERFCTR_L2_CACHE_REFILL_READ: c_uint = 0x52;
pub const ARMV7_A15_PERFCTR_L2_CACHE_REFILL_WRITE: c_uint = 0x53;
pub const ARMV7_A15_PERFCTR_PC_WRITE_SPEC: c_uint = 0x76;
// ARMv7 Cortex-A12 specific event types
pub const ARMV7_A12_PERFCTR_L1_DCACHE_ACCESS_READ: c_uint = 0x40;
pub const ARMV7_A12_PERFCTR_L1_DCACHE_ACCESS_WRITE: c_uint = 0x41;
pub const ARMV7_A12_PERFCTR_L2_CACHE_ACCESS_READ: c_uint = 0x50;
pub const ARMV7_A12_PERFCTR_L2_CACHE_ACCESS_WRITE: c_uint = 0x51;
pub const ARMV7_A12_PERFCTR_PC_WRITE_SPEC: c_uint = 0x76;
pub const ARMV7_A12_PERFCTR_PF_TLB_REFILL: c_uint = 0xe7;
// ARMv7 Krait specific event types
pub const KRAIT_PMRESR0_GROUP0: c_uint = 0xcc;
pub const KRAIT_PMRESR1_GROUP0: c_uint = 0xd0;
pub const KRAIT_PMRESR2_GROUP0: c_uint = 0xd4;
pub const KRAIT_VPMRESR0_GROUP0: c_uint = 0xd8;
pub const KRAIT_PERFCTR_L1_ICACHE_ACCESS: c_uint = 0x10011;
pub const KRAIT_PERFCTR_L1_ICACHE_MISS: c_uint = 0x10010;
pub const KRAIT_PERFCTR_L1_ITLB_ACCESS: c_uint = 0x12222;
pub const KRAIT_PERFCTR_L1_DTLB_ACCESS: c_uint = 0x12210;
// ARMv7 Scorpion specific event types
pub const SCORPION_LPM0_GROUP0: c_uint = 0x4c;
pub const SCORPION_LPM1_GROUP0: c_uint = 0x50;
pub const SCORPION_LPM2_GROUP0: c_uint = 0x54;
pub const SCORPION_L2LPM_GROUP0: c_uint = 0x58;
pub const SCORPION_VLPM_GROUP0: c_uint = 0x5c;
pub const SCORPION_ICACHE_ACCESS: c_uint = 0x10053;
pub const SCORPION_ICACHE_MISS: c_uint = 0x10052;
pub const SCORPION_DTLB_ACCESS: c_uint = 0x12013;
pub const SCORPION_DTLB_MISS: c_uint = 0x12012;
pub const SCORPION_ITLB_MISS: c_uint = 0x12021;
//
// Cortex-A8 HW events mapping
//
// The hardware events that we support. We do support cache operations but
// we have harvard caches and no way to combine instruction and data
// accesses/misses in hardware.
//
    static const unsigned armv7_a8_perf_map[PERF_COUNT_HW_MAX] = {
    PERF_MAP_ALL_UNSUPPORTED,
    [PERF_COUNT_HW_CPU_CYCLES]		= ARMV7_PERFCTR_CPU_CYCLES,
    [PERF_COUNT_HW_INSTRUCTIONS]		= ARMV7_PERFCTR_INSTR_EXECUTED,
    [PERF_COUNT_HW_CACHE_REFERENCES]	= ARMV7_PERFCTR_L1_DCACHE_ACCESS,
    [PERF_COUNT_HW_CACHE_MISSES]		= ARMV7_PERFCTR_L1_DCACHE_REFILL,
    [PERF_COUNT_HW_BRANCH_INSTRUCTIONS]	= ARMV7_PERFCTR_PC_WRITE,
    [PERF_COUNT_HW_BRANCH_MISSES]		= ARMV7_PERFCTR_PC_BRANCH_MIS_PRED,
    [PERF_COUNT_HW_STALLED_CYCLES_FRONTEND]	= ARMV7_A8_PERFCTR_STALL_ISIDE,
    };
    static const unsigned armv7_a8_perf_cache_map[PERF_COUNT_HW_CACHE_MAX]
    [PERF_COUNT_HW_CACHE_OP_MAX]
    [PERF_COUNT_HW_CACHE_RESULT_MAX] = {
    PERF_CACHE_MAP_ALL_UNSUPPORTED,
//
// The performance counters don't differentiate between read and write
// accesses/misses so this isn't strictly correct, but it's the best we
// can do. Writes and reads get combined.
//
    [C(L1D)][C(OP_READ)][C(RESULT_ACCESS)]	= ARMV7_PERFCTR_L1_DCACHE_ACCESS,
    [C(L1D)][C(OP_READ)][C(RESULT_MISS)]	= ARMV7_PERFCTR_L1_DCACHE_REFILL,
    [C(L1D)][C(OP_WRITE)][C(RESULT_ACCESS)]	= ARMV7_PERFCTR_L1_DCACHE_ACCESS,
    [C(L1D)][C(OP_WRITE)][C(RESULT_MISS)]	= ARMV7_PERFCTR_L1_DCACHE_REFILL,
    [C(L1I)][C(OP_READ)][C(RESULT_ACCESS)]	= ARMV7_A8_PERFCTR_L1_ICACHE_ACCESS,
    [C(L1I)][C(OP_READ)][C(RESULT_MISS)]	= ARMV7_PERFCTR_L1_ICACHE_REFILL,
    [C(LL)][C(OP_READ)][C(RESULT_ACCESS)]	= ARMV7_A8_PERFCTR_L2_CACHE_ACCESS,
    [C(LL)][C(OP_READ)][C(RESULT_MISS)]	= ARMV7_A8_PERFCTR_L2_CACHE_REFILL,
    [C(LL)][C(OP_WRITE)][C(RESULT_ACCESS)]	= ARMV7_A8_PERFCTR_L2_CACHE_ACCESS,
    [C(LL)][C(OP_WRITE)][C(RESULT_MISS)]	= ARMV7_A8_PERFCTR_L2_CACHE_REFILL,
    [C(DTLB)][C(OP_READ)][C(RESULT_MISS)]	= ARMV7_PERFCTR_DTLB_REFILL,
    [C(DTLB)][C(OP_WRITE)][C(RESULT_MISS)]	= ARMV7_PERFCTR_DTLB_REFILL,
    [C(ITLB)][C(OP_READ)][C(RESULT_MISS)]	= ARMV7_PERFCTR_ITLB_REFILL,
    [C(ITLB)][C(OP_WRITE)][C(RESULT_MISS)]	= ARMV7_PERFCTR_ITLB_REFILL,
    [C(BPU)][C(OP_READ)][C(RESULT_ACCESS)]	= ARMV7_PERFCTR_PC_BRANCH_PRED,
    [C(BPU)][C(OP_READ)][C(RESULT_MISS)]	= ARMV7_PERFCTR_PC_BRANCH_MIS_PRED,
    [C(BPU)][C(OP_WRITE)][C(RESULT_ACCESS)]	= ARMV7_PERFCTR_PC_BRANCH_PRED,
    [C(BPU)][C(OP_WRITE)][C(RESULT_MISS)]	= ARMV7_PERFCTR_PC_BRANCH_MIS_PRED,
    };
//
// Cortex-A9 HW events mapping
//
    static const unsigned armv7_a9_perf_map[PERF_COUNT_HW_MAX] = {
    PERF_MAP_ALL_UNSUPPORTED,
    [PERF_COUNT_HW_CPU_CYCLES]		= ARMV7_PERFCTR_CPU_CYCLES,
    [PERF_COUNT_HW_INSTRUCTIONS]		= ARMV7_A9_PERFCTR_INSTR_CORE_RENAME,
    [PERF_COUNT_HW_CACHE_REFERENCES]	= ARMV7_PERFCTR_L1_DCACHE_ACCESS,
    [PERF_COUNT_HW_CACHE_MISSES]		= ARMV7_PERFCTR_L1_DCACHE_REFILL,
    [PERF_COUNT_HW_BRANCH_INSTRUCTIONS]	= ARMV7_PERFCTR_PC_WRITE,
    [PERF_COUNT_HW_BRANCH_MISSES]		= ARMV7_PERFCTR_PC_BRANCH_MIS_PRED,
    [PERF_COUNT_HW_STALLED_CYCLES_FRONTEND]	= ARMV7_A9_PERFCTR_STALL_ICACHE,
    [PERF_COUNT_HW_STALLED_CYCLES_BACKEND]	= ARMV7_A9_PERFCTR_STALL_DISPATCH,
    };
    static const unsigned armv7_a9_perf_cache_map[PERF_COUNT_HW_CACHE_MAX]
    [PERF_COUNT_HW_CACHE_OP_MAX]
    [PERF_COUNT_HW_CACHE_RESULT_MAX] = {
    PERF_CACHE_MAP_ALL_UNSUPPORTED,
//
// The performance counters don't differentiate between read and write
// accesses/misses so this isn't strictly correct, but it's the best we
// can do. Writes and reads get combined.
//
    [C(L1D)][C(OP_READ)][C(RESULT_ACCESS)]	= ARMV7_PERFCTR_L1_DCACHE_ACCESS,
    [C(L1D)][C(OP_READ)][C(RESULT_MISS)]	= ARMV7_PERFCTR_L1_DCACHE_REFILL,
    [C(L1D)][C(OP_WRITE)][C(RESULT_ACCESS)]	= ARMV7_PERFCTR_L1_DCACHE_ACCESS,
    [C(L1D)][C(OP_WRITE)][C(RESULT_MISS)]	= ARMV7_PERFCTR_L1_DCACHE_REFILL,
    [C(L1I)][C(OP_READ)][C(RESULT_MISS)]	= ARMV7_PERFCTR_L1_ICACHE_REFILL,
    [C(DTLB)][C(OP_READ)][C(RESULT_MISS)]	= ARMV7_PERFCTR_DTLB_REFILL,
    [C(DTLB)][C(OP_WRITE)][C(RESULT_MISS)]	= ARMV7_PERFCTR_DTLB_REFILL,
    [C(ITLB)][C(OP_READ)][C(RESULT_MISS)]	= ARMV7_PERFCTR_ITLB_REFILL,
    [C(ITLB)][C(OP_WRITE)][C(RESULT_MISS)]	= ARMV7_PERFCTR_ITLB_REFILL,
    [C(BPU)][C(OP_READ)][C(RESULT_ACCESS)]	= ARMV7_PERFCTR_PC_BRANCH_PRED,
    [C(BPU)][C(OP_READ)][C(RESULT_MISS)]	= ARMV7_PERFCTR_PC_BRANCH_MIS_PRED,
    [C(BPU)][C(OP_WRITE)][C(RESULT_ACCESS)]	= ARMV7_PERFCTR_PC_BRANCH_PRED,
    [C(BPU)][C(OP_WRITE)][C(RESULT_MISS)]	= ARMV7_PERFCTR_PC_BRANCH_MIS_PRED,
    };
//
// Cortex-A5 HW events mapping
//
    static const unsigned armv7_a5_perf_map[PERF_COUNT_HW_MAX] = {
    PERF_MAP_ALL_UNSUPPORTED,
    [PERF_COUNT_HW_CPU_CYCLES]		= ARMV7_PERFCTR_CPU_CYCLES,
    [PERF_COUNT_HW_INSTRUCTIONS]		= ARMV7_PERFCTR_INSTR_EXECUTED,
    [PERF_COUNT_HW_CACHE_REFERENCES]	= ARMV7_PERFCTR_L1_DCACHE_ACCESS,
    [PERF_COUNT_HW_CACHE_MISSES]		= ARMV7_PERFCTR_L1_DCACHE_REFILL,
    [PERF_COUNT_HW_BRANCH_INSTRUCTIONS]	= ARMV7_PERFCTR_PC_WRITE,
    [PERF_COUNT_HW_BRANCH_MISSES]		= ARMV7_PERFCTR_PC_BRANCH_MIS_PRED,
    };
    static const unsigned armv7_a5_perf_cache_map[PERF_COUNT_HW_CACHE_MAX]
    [PERF_COUNT_HW_CACHE_OP_MAX]
    [PERF_COUNT_HW_CACHE_RESULT_MAX] = {
    PERF_CACHE_MAP_ALL_UNSUPPORTED,
    [C(L1D)][C(OP_READ)][C(RESULT_ACCESS)]	= ARMV7_PERFCTR_L1_DCACHE_ACCESS,
    [C(L1D)][C(OP_READ)][C(RESULT_MISS)]	= ARMV7_PERFCTR_L1_DCACHE_REFILL,
    [C(L1D)][C(OP_WRITE)][C(RESULT_ACCESS)]	= ARMV7_PERFCTR_L1_DCACHE_ACCESS,
    [C(L1D)][C(OP_WRITE)][C(RESULT_MISS)]	= ARMV7_PERFCTR_L1_DCACHE_REFILL,
    [C(L1D)][C(OP_PREFETCH)][C(RESULT_ACCESS)]	= ARMV7_A5_PERFCTR_PREFETCH_LINEFILL,
    [C(L1D)][C(OP_PREFETCH)][C(RESULT_MISS)]	= ARMV7_A5_PERFCTR_PREFETCH_LINEFILL_DROP,
    [C(L1I)][C(OP_READ)][C(RESULT_ACCESS)]	= ARMV7_PERFCTR_L1_ICACHE_ACCESS,
    [C(L1I)][C(OP_READ)][C(RESULT_MISS)]	= ARMV7_PERFCTR_L1_ICACHE_REFILL,
//
// The prefetch counters don't differentiate between the I side and the
// D side.
//
    [C(L1I)][C(OP_PREFETCH)][C(RESULT_ACCESS)]	= ARMV7_A5_PERFCTR_PREFETCH_LINEFILL,
    [C(L1I)][C(OP_PREFETCH)][C(RESULT_MISS)]	= ARMV7_A5_PERFCTR_PREFETCH_LINEFILL_DROP,
    [C(DTLB)][C(OP_READ)][C(RESULT_MISS)]	= ARMV7_PERFCTR_DTLB_REFILL,
    [C(DTLB)][C(OP_WRITE)][C(RESULT_MISS)]	= ARMV7_PERFCTR_DTLB_REFILL,
    [C(ITLB)][C(OP_READ)][C(RESULT_MISS)]	= ARMV7_PERFCTR_ITLB_REFILL,
    [C(ITLB)][C(OP_WRITE)][C(RESULT_MISS)]	= ARMV7_PERFCTR_ITLB_REFILL,
    [C(BPU)][C(OP_READ)][C(RESULT_ACCESS)]	= ARMV7_PERFCTR_PC_BRANCH_PRED,
    [C(BPU)][C(OP_READ)][C(RESULT_MISS)]	= ARMV7_PERFCTR_PC_BRANCH_MIS_PRED,
    [C(BPU)][C(OP_WRITE)][C(RESULT_ACCESS)]	= ARMV7_PERFCTR_PC_BRANCH_PRED,
    [C(BPU)][C(OP_WRITE)][C(RESULT_MISS)]	= ARMV7_PERFCTR_PC_BRANCH_MIS_PRED,
    };
//
// Cortex-A15 HW events mapping
//
    static const unsigned armv7_a15_perf_map[PERF_COUNT_HW_MAX] = {
    PERF_MAP_ALL_UNSUPPORTED,
    [PERF_COUNT_HW_CPU_CYCLES]		= ARMV7_PERFCTR_CPU_CYCLES,
    [PERF_COUNT_HW_INSTRUCTIONS]		= ARMV7_PERFCTR_INSTR_EXECUTED,
    [PERF_COUNT_HW_CACHE_REFERENCES]	= ARMV7_PERFCTR_L1_DCACHE_ACCESS,
    [PERF_COUNT_HW_CACHE_MISSES]		= ARMV7_PERFCTR_L1_DCACHE_REFILL,
    [PERF_COUNT_HW_BRANCH_INSTRUCTIONS]	= ARMV7_A15_PERFCTR_PC_WRITE_SPEC,
    [PERF_COUNT_HW_BRANCH_MISSES]		= ARMV7_PERFCTR_PC_BRANCH_MIS_PRED,
    [PERF_COUNT_HW_BUS_CYCLES]		= ARMV7_PERFCTR_BUS_CYCLES,
    };
    static const unsigned armv7_a15_perf_cache_map[PERF_COUNT_HW_CACHE_MAX]
    [PERF_COUNT_HW_CACHE_OP_MAX]
    [PERF_COUNT_HW_CACHE_RESULT_MAX] = {
    PERF_CACHE_MAP_ALL_UNSUPPORTED,
    [C(L1D)][C(OP_READ)][C(RESULT_ACCESS)]	= ARMV7_A15_PERFCTR_L1_DCACHE_ACCESS_READ,
    [C(L1D)][C(OP_READ)][C(RESULT_MISS)]	= ARMV7_A15_PERFCTR_L1_DCACHE_REFILL_READ,
    [C(L1D)][C(OP_WRITE)][C(RESULT_ACCESS)]	= ARMV7_A15_PERFCTR_L1_DCACHE_ACCESS_WRITE,
    [C(L1D)][C(OP_WRITE)][C(RESULT_MISS)]	= ARMV7_A15_PERFCTR_L1_DCACHE_REFILL_WRITE,
//
// Not all performance counters differentiate between read and write
// accesses/misses so we're not always strictly correct, but it's the
// best we can do. Writes and reads get combined in these cases.
//
    [C(L1I)][C(OP_READ)][C(RESULT_ACCESS)]	= ARMV7_PERFCTR_L1_ICACHE_ACCESS,
    [C(L1I)][C(OP_READ)][C(RESULT_MISS)]	= ARMV7_PERFCTR_L1_ICACHE_REFILL,
    [C(LL)][C(OP_READ)][C(RESULT_ACCESS)]	= ARMV7_A15_PERFCTR_L2_CACHE_ACCESS_READ,
    [C(LL)][C(OP_READ)][C(RESULT_MISS)]	= ARMV7_A15_PERFCTR_L2_CACHE_REFILL_READ,
    [C(LL)][C(OP_WRITE)][C(RESULT_ACCESS)]	= ARMV7_A15_PERFCTR_L2_CACHE_ACCESS_WRITE,
    [C(LL)][C(OP_WRITE)][C(RESULT_MISS)]	= ARMV7_A15_PERFCTR_L2_CACHE_REFILL_WRITE,
    [C(DTLB)][C(OP_READ)][C(RESULT_MISS)]	= ARMV7_A15_PERFCTR_DTLB_REFILL_L1_READ,
    [C(DTLB)][C(OP_WRITE)][C(RESULT_MISS)]	= ARMV7_A15_PERFCTR_DTLB_REFILL_L1_WRITE,
    [C(ITLB)][C(OP_READ)][C(RESULT_MISS)]	= ARMV7_PERFCTR_ITLB_REFILL,
    [C(ITLB)][C(OP_WRITE)][C(RESULT_MISS)]	= ARMV7_PERFCTR_ITLB_REFILL,
    [C(BPU)][C(OP_READ)][C(RESULT_ACCESS)]	= ARMV7_PERFCTR_PC_BRANCH_PRED,
    [C(BPU)][C(OP_READ)][C(RESULT_MISS)]	= ARMV7_PERFCTR_PC_BRANCH_MIS_PRED,
    [C(BPU)][C(OP_WRITE)][C(RESULT_ACCESS)]	= ARMV7_PERFCTR_PC_BRANCH_PRED,
    [C(BPU)][C(OP_WRITE)][C(RESULT_MISS)]	= ARMV7_PERFCTR_PC_BRANCH_MIS_PRED,
    };
//
// Cortex-A7 HW events mapping
//
    static const unsigned armv7_a7_perf_map[PERF_COUNT_HW_MAX] = {
    PERF_MAP_ALL_UNSUPPORTED,
    [PERF_COUNT_HW_CPU_CYCLES]		= ARMV7_PERFCTR_CPU_CYCLES,
    [PERF_COUNT_HW_INSTRUCTIONS]		= ARMV7_PERFCTR_INSTR_EXECUTED,
    [PERF_COUNT_HW_CACHE_REFERENCES]	= ARMV7_PERFCTR_L1_DCACHE_ACCESS,
    [PERF_COUNT_HW_CACHE_MISSES]		= ARMV7_PERFCTR_L1_DCACHE_REFILL,
    [PERF_COUNT_HW_BRANCH_INSTRUCTIONS]	= ARMV7_PERFCTR_PC_WRITE,
    [PERF_COUNT_HW_BRANCH_MISSES]		= ARMV7_PERFCTR_PC_BRANCH_MIS_PRED,
    [PERF_COUNT_HW_BUS_CYCLES]		= ARMV7_PERFCTR_BUS_CYCLES,
    };
    static const unsigned armv7_a7_perf_cache_map[PERF_COUNT_HW_CACHE_MAX]
    [PERF_COUNT_HW_CACHE_OP_MAX]
    [PERF_COUNT_HW_CACHE_RESULT_MAX] = {
    PERF_CACHE_MAP_ALL_UNSUPPORTED,
//
// The performance counters don't differentiate between read and write
// accesses/misses so this isn't strictly correct, but it's the best we
// can do. Writes and reads get combined.
//
    [C(L1D)][C(OP_READ)][C(RESULT_ACCESS)]	= ARMV7_PERFCTR_L1_DCACHE_ACCESS,
    [C(L1D)][C(OP_READ)][C(RESULT_MISS)]	= ARMV7_PERFCTR_L1_DCACHE_REFILL,
    [C(L1D)][C(OP_WRITE)][C(RESULT_ACCESS)]	= ARMV7_PERFCTR_L1_DCACHE_ACCESS,
    [C(L1D)][C(OP_WRITE)][C(RESULT_MISS)]	= ARMV7_PERFCTR_L1_DCACHE_REFILL,
    [C(L1I)][C(OP_READ)][C(RESULT_ACCESS)]	= ARMV7_PERFCTR_L1_ICACHE_ACCESS,
    [C(L1I)][C(OP_READ)][C(RESULT_MISS)]	= ARMV7_PERFCTR_L1_ICACHE_REFILL,
    [C(LL)][C(OP_READ)][C(RESULT_ACCESS)]	= ARMV7_PERFCTR_L2_CACHE_ACCESS,
    [C(LL)][C(OP_READ)][C(RESULT_MISS)]	= ARMV7_PERFCTR_L2_CACHE_REFILL,
    [C(LL)][C(OP_WRITE)][C(RESULT_ACCESS)]	= ARMV7_PERFCTR_L2_CACHE_ACCESS,
    [C(LL)][C(OP_WRITE)][C(RESULT_MISS)]	= ARMV7_PERFCTR_L2_CACHE_REFILL,
    [C(DTLB)][C(OP_READ)][C(RESULT_MISS)]	= ARMV7_PERFCTR_DTLB_REFILL,
    [C(DTLB)][C(OP_WRITE)][C(RESULT_MISS)]	= ARMV7_PERFCTR_DTLB_REFILL,
    [C(ITLB)][C(OP_READ)][C(RESULT_MISS)]	= ARMV7_PERFCTR_ITLB_REFILL,
    [C(ITLB)][C(OP_WRITE)][C(RESULT_MISS)]	= ARMV7_PERFCTR_ITLB_REFILL,
    [C(BPU)][C(OP_READ)][C(RESULT_ACCESS)]	= ARMV7_PERFCTR_PC_BRANCH_PRED,
    [C(BPU)][C(OP_READ)][C(RESULT_MISS)]	= ARMV7_PERFCTR_PC_BRANCH_MIS_PRED,
    [C(BPU)][C(OP_WRITE)][C(RESULT_ACCESS)]	= ARMV7_PERFCTR_PC_BRANCH_PRED,
    [C(BPU)][C(OP_WRITE)][C(RESULT_MISS)]	= ARMV7_PERFCTR_PC_BRANCH_MIS_PRED,
    };
//
// Cortex-A12 HW events mapping
//
    static const unsigned armv7_a12_perf_map[PERF_COUNT_HW_MAX] = {
    PERF_MAP_ALL_UNSUPPORTED,
    [PERF_COUNT_HW_CPU_CYCLES]		= ARMV7_PERFCTR_CPU_CYCLES,
    [PERF_COUNT_HW_INSTRUCTIONS]		= ARMV7_PERFCTR_INSTR_EXECUTED,
    [PERF_COUNT_HW_CACHE_REFERENCES]	= ARMV7_PERFCTR_L1_DCACHE_ACCESS,
    [PERF_COUNT_HW_CACHE_MISSES]		= ARMV7_PERFCTR_L1_DCACHE_REFILL,
    [PERF_COUNT_HW_BRANCH_INSTRUCTIONS]	= ARMV7_A12_PERFCTR_PC_WRITE_SPEC,
    [PERF_COUNT_HW_BRANCH_MISSES]		= ARMV7_PERFCTR_PC_BRANCH_MIS_PRED,
    [PERF_COUNT_HW_BUS_CYCLES]		= ARMV7_PERFCTR_BUS_CYCLES,
    };
    static const unsigned armv7_a12_perf_cache_map[PERF_COUNT_HW_CACHE_MAX]
    [PERF_COUNT_HW_CACHE_OP_MAX]
    [PERF_COUNT_HW_CACHE_RESULT_MAX] = {
    PERF_CACHE_MAP_ALL_UNSUPPORTED,
    [C(L1D)][C(OP_READ)][C(RESULT_ACCESS)]	= ARMV7_A12_PERFCTR_L1_DCACHE_ACCESS_READ,
    [C(L1D)][C(OP_READ)][C(RESULT_MISS)]	= ARMV7_PERFCTR_L1_DCACHE_REFILL,
    [C(L1D)][C(OP_WRITE)][C(RESULT_ACCESS)]	= ARMV7_A12_PERFCTR_L1_DCACHE_ACCESS_WRITE,
    [C(L1D)][C(OP_WRITE)][C(RESULT_MISS)]	= ARMV7_PERFCTR_L1_DCACHE_REFILL,
//
// Not all performance counters differentiate between read and write
// accesses/misses so we're not always strictly correct, but it's the
// best we can do. Writes and reads get combined in these cases.
//
    [C(L1I)][C(OP_READ)][C(RESULT_ACCESS)]	= ARMV7_PERFCTR_L1_ICACHE_ACCESS,
    [C(L1I)][C(OP_READ)][C(RESULT_MISS)]	= ARMV7_PERFCTR_L1_ICACHE_REFILL,
    [C(LL)][C(OP_READ)][C(RESULT_ACCESS)]	= ARMV7_A12_PERFCTR_L2_CACHE_ACCESS_READ,
    [C(LL)][C(OP_READ)][C(RESULT_MISS)]	= ARMV7_PERFCTR_L2_CACHE_REFILL,
    [C(LL)][C(OP_WRITE)][C(RESULT_ACCESS)]	= ARMV7_A12_PERFCTR_L2_CACHE_ACCESS_WRITE,
    [C(LL)][C(OP_WRITE)][C(RESULT_MISS)]	= ARMV7_PERFCTR_L2_CACHE_REFILL,
    [C(DTLB)][C(OP_READ)][C(RESULT_MISS)]	= ARMV7_PERFCTR_DTLB_REFILL,
    [C(DTLB)][C(OP_WRITE)][C(RESULT_MISS)]	= ARMV7_PERFCTR_DTLB_REFILL,
    [C(DTLB)][C(OP_PREFETCH)][C(RESULT_MISS)]	= ARMV7_A12_PERFCTR_PF_TLB_REFILL,
    [C(ITLB)][C(OP_READ)][C(RESULT_MISS)]	= ARMV7_PERFCTR_ITLB_REFILL,
    [C(ITLB)][C(OP_WRITE)][C(RESULT_MISS)]	= ARMV7_PERFCTR_ITLB_REFILL,
    [C(BPU)][C(OP_READ)][C(RESULT_ACCESS)]	= ARMV7_PERFCTR_PC_BRANCH_PRED,
    [C(BPU)][C(OP_READ)][C(RESULT_MISS)]	= ARMV7_PERFCTR_PC_BRANCH_MIS_PRED,
    [C(BPU)][C(OP_WRITE)][C(RESULT_ACCESS)]	= ARMV7_PERFCTR_PC_BRANCH_PRED,
    [C(BPU)][C(OP_WRITE)][C(RESULT_MISS)]	= ARMV7_PERFCTR_PC_BRANCH_MIS_PRED,
    };
//
// Krait HW events mapping
//
    static const unsigned krait_perf_map[PERF_COUNT_HW_MAX] = {
    PERF_MAP_ALL_UNSUPPORTED,
    [PERF_COUNT_HW_CPU_CYCLES]	    = ARMV7_PERFCTR_CPU_CYCLES,
    [PERF_COUNT_HW_INSTRUCTIONS]	    = ARMV7_PERFCTR_INSTR_EXECUTED,
    [PERF_COUNT_HW_BRANCH_INSTRUCTIONS] = ARMV7_PERFCTR_PC_WRITE,
    [PERF_COUNT_HW_BRANCH_MISSES]	    = ARMV7_PERFCTR_PC_BRANCH_MIS_PRED,
    [PERF_COUNT_HW_BUS_CYCLES]	    = ARMV7_PERFCTR_CLOCK_CYCLES,
    };
    static const unsigned krait_perf_map_no_branch[PERF_COUNT_HW_MAX] = {
    PERF_MAP_ALL_UNSUPPORTED,
    [PERF_COUNT_HW_CPU_CYCLES]	    = ARMV7_PERFCTR_CPU_CYCLES,
    [PERF_COUNT_HW_INSTRUCTIONS]	    = ARMV7_PERFCTR_INSTR_EXECUTED,
    [PERF_COUNT_HW_BRANCH_MISSES]	    = ARMV7_PERFCTR_PC_BRANCH_MIS_PRED,
    [PERF_COUNT_HW_BUS_CYCLES]	    = ARMV7_PERFCTR_CLOCK_CYCLES,
    };
    static const unsigned krait_perf_cache_map[PERF_COUNT_HW_CACHE_MAX]
    [PERF_COUNT_HW_CACHE_OP_MAX]
    [PERF_COUNT_HW_CACHE_RESULT_MAX] = {
    PERF_CACHE_MAP_ALL_UNSUPPORTED,
//
// The performance counters don't differentiate between read and write
// accesses/misses so this isn't strictly correct, but it's the best we
// can do. Writes and reads get combined.
//
    [C(L1D)][C(OP_READ)][C(RESULT_ACCESS)]	= ARMV7_PERFCTR_L1_DCACHE_ACCESS,
    [C(L1D)][C(OP_READ)][C(RESULT_MISS)]	= ARMV7_PERFCTR_L1_DCACHE_REFILL,
    [C(L1D)][C(OP_WRITE)][C(RESULT_ACCESS)]	= ARMV7_PERFCTR_L1_DCACHE_ACCESS,
    [C(L1D)][C(OP_WRITE)][C(RESULT_MISS)]	= ARMV7_PERFCTR_L1_DCACHE_REFILL,
    [C(L1I)][C(OP_READ)][C(RESULT_ACCESS)]	= KRAIT_PERFCTR_L1_ICACHE_ACCESS,
    [C(L1I)][C(OP_READ)][C(RESULT_MISS)]	= KRAIT_PERFCTR_L1_ICACHE_MISS,
    [C(DTLB)][C(OP_READ)][C(RESULT_ACCESS)]	= KRAIT_PERFCTR_L1_DTLB_ACCESS,
    [C(DTLB)][C(OP_WRITE)][C(RESULT_ACCESS)]	= KRAIT_PERFCTR_L1_DTLB_ACCESS,
    [C(ITLB)][C(OP_READ)][C(RESULT_ACCESS)]	= KRAIT_PERFCTR_L1_ITLB_ACCESS,
    [C(ITLB)][C(OP_WRITE)][C(RESULT_ACCESS)]	= KRAIT_PERFCTR_L1_ITLB_ACCESS,
    [C(BPU)][C(OP_READ)][C(RESULT_ACCESS)]	= ARMV7_PERFCTR_PC_BRANCH_PRED,
    [C(BPU)][C(OP_READ)][C(RESULT_MISS)]	= ARMV7_PERFCTR_PC_BRANCH_MIS_PRED,
    [C(BPU)][C(OP_WRITE)][C(RESULT_ACCESS)]	= ARMV7_PERFCTR_PC_BRANCH_PRED,
    [C(BPU)][C(OP_WRITE)][C(RESULT_MISS)]	= ARMV7_PERFCTR_PC_BRANCH_MIS_PRED,
    };
//
// Scorpion HW events mapping
//
    static const unsigned scorpion_perf_map[PERF_COUNT_HW_MAX] = {
    PERF_MAP_ALL_UNSUPPORTED,
    [PERF_COUNT_HW_CPU_CYCLES]	    = ARMV7_PERFCTR_CPU_CYCLES,
    [PERF_COUNT_HW_INSTRUCTIONS]	    = ARMV7_PERFCTR_INSTR_EXECUTED,
    [PERF_COUNT_HW_BRANCH_INSTRUCTIONS] = ARMV7_PERFCTR_PC_WRITE,
    [PERF_COUNT_HW_BRANCH_MISSES]	    = ARMV7_PERFCTR_PC_BRANCH_MIS_PRED,
    [PERF_COUNT_HW_BUS_CYCLES]	    = ARMV7_PERFCTR_CLOCK_CYCLES,
    };
    static const unsigned scorpion_perf_cache_map[PERF_COUNT_HW_CACHE_MAX]
    [PERF_COUNT_HW_CACHE_OP_MAX]
    [PERF_COUNT_HW_CACHE_RESULT_MAX] = {
    PERF_CACHE_MAP_ALL_UNSUPPORTED,
//
// The performance counters don't differentiate between read and write
// accesses/misses so this isn't strictly correct, but it's the best we
// can do. Writes and reads get combined.
//
    [C(L1D)][C(OP_READ)][C(RESULT_ACCESS)] = ARMV7_PERFCTR_L1_DCACHE_ACCESS,
    [C(L1D)][C(OP_READ)][C(RESULT_MISS)] = ARMV7_PERFCTR_L1_DCACHE_REFILL,
    [C(L1D)][C(OP_WRITE)][C(RESULT_ACCESS)] = ARMV7_PERFCTR_L1_DCACHE_ACCESS,
    [C(L1D)][C(OP_WRITE)][C(RESULT_MISS)] = ARMV7_PERFCTR_L1_DCACHE_REFILL,
    [C(L1I)][C(OP_READ)][C(RESULT_ACCESS)] = SCORPION_ICACHE_ACCESS,
    [C(L1I)][C(OP_READ)][C(RESULT_MISS)] = SCORPION_ICACHE_MISS,
//
// Only ITLB misses and DTLB refills are supported.  If users want the
// DTLB refills misses a raw counter must be used.
//
    [C(DTLB)][C(OP_READ)][C(RESULT_ACCESS)] = SCORPION_DTLB_ACCESS,
    [C(DTLB)][C(OP_READ)][C(RESULT_MISS)] = SCORPION_DTLB_MISS,
    [C(DTLB)][C(OP_WRITE)][C(RESULT_ACCESS)] = SCORPION_DTLB_ACCESS,
    [C(DTLB)][C(OP_WRITE)][C(RESULT_MISS)] = SCORPION_DTLB_MISS,
    [C(ITLB)][C(OP_READ)][C(RESULT_MISS)] = SCORPION_ITLB_MISS,
    [C(ITLB)][C(OP_WRITE)][C(RESULT_MISS)] = SCORPION_ITLB_MISS,
    [C(BPU)][C(OP_READ)][C(RESULT_ACCESS)] = ARMV7_PERFCTR_PC_BRANCH_PRED,
    [C(BPU)][C(OP_READ)][C(RESULT_MISS)] = ARMV7_PERFCTR_PC_BRANCH_MIS_PRED,
    [C(BPU)][C(OP_WRITE)][C(RESULT_ACCESS)] = ARMV7_PERFCTR_PC_BRANCH_PRED,
    [C(BPU)][C(OP_WRITE)][C(RESULT_MISS)] = ARMV7_PERFCTR_PC_BRANCH_MIS_PRED,
    };
    PMU_FORMAT_ATTR(event, "config:0-7");
    static struct attribute *armv7_pmu_format_attrs[] = {
    &format_attr_event.attr,
    core::ptr::null_mut(),
    };
    static struct attribute_group armv7_pmu_format_attr_group = {
    .name = "format",
    .attrs = armv7_pmu_format_attrs,
    };

    PMU_EVENT_ATTR_STRING(name, armv7_event_attr_##name, \
    "event=" ARMV7_EVENT_ATTR_RESOLVE(config))
    ARMV7_EVENT_ATTR(sw_incr, ARMV7_PERFCTR_PMNC_SW_INCR);
    ARMV7_EVENT_ATTR(l1i_cache_refill, ARMV7_PERFCTR_L1_ICACHE_REFILL);
    ARMV7_EVENT_ATTR(l1i_tlb_refill, ARMV7_PERFCTR_ITLB_REFILL);
    ARMV7_EVENT_ATTR(l1d_cache_refill, ARMV7_PERFCTR_L1_DCACHE_REFILL);
    ARMV7_EVENT_ATTR(l1d_cache, ARMV7_PERFCTR_L1_DCACHE_ACCESS);
    ARMV7_EVENT_ATTR(l1d_tlb_refill, ARMV7_PERFCTR_DTLB_REFILL);
    ARMV7_EVENT_ATTR(ld_retired, ARMV7_PERFCTR_MEM_READ);
    ARMV7_EVENT_ATTR(st_retired, ARMV7_PERFCTR_MEM_WRITE);
    ARMV7_EVENT_ATTR(inst_retired, ARMV7_PERFCTR_INSTR_EXECUTED);
    ARMV7_EVENT_ATTR(exc_taken, ARMV7_PERFCTR_EXC_TAKEN);
    ARMV7_EVENT_ATTR(exc_return, ARMV7_PERFCTR_EXC_EXECUTED);
    ARMV7_EVENT_ATTR(cid_write_retired, ARMV7_PERFCTR_CID_WRITE);
    ARMV7_EVENT_ATTR(pc_write_retired, ARMV7_PERFCTR_PC_WRITE);
    ARMV7_EVENT_ATTR(br_immed_retired, ARMV7_PERFCTR_PC_IMM_BRANCH);
    ARMV7_EVENT_ATTR(br_return_retired, ARMV7_PERFCTR_PC_PROC_RETURN);
    ARMV7_EVENT_ATTR(unaligned_ldst_retired, ARMV7_PERFCTR_MEM_UNALIGNED_ACCESS);
    ARMV7_EVENT_ATTR(br_mis_pred, ARMV7_PERFCTR_PC_BRANCH_MIS_PRED);
    ARMV7_EVENT_ATTR(cpu_cycles, ARMV7_PERFCTR_CLOCK_CYCLES);
    ARMV7_EVENT_ATTR(br_pred, ARMV7_PERFCTR_PC_BRANCH_PRED);
    static struct attribute *armv7_pmuv1_event_attrs[] = {
    &armv7_event_attr_sw_incr.attr.attr,
    &armv7_event_attr_l1i_cache_refill.attr.attr,
    &armv7_event_attr_l1i_tlb_refill.attr.attr,
    &armv7_event_attr_l1d_cache_refill.attr.attr,
    &armv7_event_attr_l1d_cache.attr.attr,
    &armv7_event_attr_l1d_tlb_refill.attr.attr,
    &armv7_event_attr_ld_retired.attr.attr,
    &armv7_event_attr_st_retired.attr.attr,
    &armv7_event_attr_inst_retired.attr.attr,
    &armv7_event_attr_exc_taken.attr.attr,
    &armv7_event_attr_exc_return.attr.attr,
    &armv7_event_attr_cid_write_retired.attr.attr,
    &armv7_event_attr_pc_write_retired.attr.attr,
    &armv7_event_attr_br_immed_retired.attr.attr,
    &armv7_event_attr_br_return_retired.attr.attr,
    &armv7_event_attr_unaligned_ldst_retired.attr.attr,
    &armv7_event_attr_br_mis_pred.attr.attr,
    &armv7_event_attr_cpu_cycles.attr.attr,
    &armv7_event_attr_br_pred.attr.attr,
    core::ptr::null_mut(),
    };
    static struct attribute_group armv7_pmuv1_events_attr_group = {
    .name = "events",
    .attrs = armv7_pmuv1_event_attrs,
    };
    ARMV7_EVENT_ATTR(mem_access, ARMV7_PERFCTR_MEM_ACCESS);
    ARMV7_EVENT_ATTR(l1i_cache, ARMV7_PERFCTR_L1_ICACHE_ACCESS);
    ARMV7_EVENT_ATTR(l1d_cache_wb, ARMV7_PERFCTR_L1_DCACHE_WB);
    ARMV7_EVENT_ATTR(l2d_cache, ARMV7_PERFCTR_L2_CACHE_ACCESS);
    ARMV7_EVENT_ATTR(l2d_cache_refill, ARMV7_PERFCTR_L2_CACHE_REFILL);
    ARMV7_EVENT_ATTR(l2d_cache_wb, ARMV7_PERFCTR_L2_CACHE_WB);
    ARMV7_EVENT_ATTR(bus_access, ARMV7_PERFCTR_BUS_ACCESS);
    ARMV7_EVENT_ATTR(memory_error, ARMV7_PERFCTR_MEM_ERROR);
    ARMV7_EVENT_ATTR(inst_spec, ARMV7_PERFCTR_INSTR_SPEC);
    ARMV7_EVENT_ATTR(ttbr_write_retired, ARMV7_PERFCTR_TTBR_WRITE);
    ARMV7_EVENT_ATTR(bus_cycles, ARMV7_PERFCTR_BUS_CYCLES);
    static struct attribute *armv7_pmuv2_event_attrs[] = {
    &armv7_event_attr_sw_incr.attr.attr,
    &armv7_event_attr_l1i_cache_refill.attr.attr,
    &armv7_event_attr_l1i_tlb_refill.attr.attr,
    &armv7_event_attr_l1d_cache_refill.attr.attr,
    &armv7_event_attr_l1d_cache.attr.attr,
    &armv7_event_attr_l1d_tlb_refill.attr.attr,
    &armv7_event_attr_ld_retired.attr.attr,
    &armv7_event_attr_st_retired.attr.attr,
    &armv7_event_attr_inst_retired.attr.attr,
    &armv7_event_attr_exc_taken.attr.attr,
    &armv7_event_attr_exc_return.attr.attr,
    &armv7_event_attr_cid_write_retired.attr.attr,
    &armv7_event_attr_pc_write_retired.attr.attr,
    &armv7_event_attr_br_immed_retired.attr.attr,
    &armv7_event_attr_br_return_retired.attr.attr,
    &armv7_event_attr_unaligned_ldst_retired.attr.attr,
    &armv7_event_attr_br_mis_pred.attr.attr,
    &armv7_event_attr_cpu_cycles.attr.attr,
    &armv7_event_attr_br_pred.attr.attr,
    &armv7_event_attr_mem_access.attr.attr,
    &armv7_event_attr_l1i_cache.attr.attr,
    &armv7_event_attr_l1d_cache_wb.attr.attr,
    &armv7_event_attr_l2d_cache.attr.attr,
    &armv7_event_attr_l2d_cache_refill.attr.attr,
    &armv7_event_attr_l2d_cache_wb.attr.attr,
    &armv7_event_attr_bus_access.attr.attr,
    &armv7_event_attr_memory_error.attr.attr,
    &armv7_event_attr_inst_spec.attr.attr,
    &armv7_event_attr_ttbr_write_retired.attr.attr,
    &armv7_event_attr_bus_cycles.attr.attr,
    core::ptr::null_mut(),
    };
    static struct attribute_group armv7_pmuv2_events_attr_group = {
    .name = "events",
    .attrs = armv7_pmuv2_event_attrs,
    };
//
// Perf Events' indices
//
pub const ARMV7_IDX_CYCLE_COUNTER: c_int = 31;
pub const ARMV7_IDX_COUNTER_MAX: c_int = 31;
//
// ARMv7 low level PMNC access
//
// Per-CPU PMNC: config reg
//

pub const ARMV7_PMNC_N_MASK: c_uint = 0x1f;
pub const ARMV7_PMNC_MASK: c_uint = 0x3f	 /* Mask for writable bits */;
//
// FLAG: counters overflow flag status reg
//
pub const ARMV7_FLAG_MASK: c_uint = 0xffffffff	/* Mask for writable bits */;

//
// PMXEVTYPER: Event selection reg
//
pub const ARMV7_EVTYPE_MASK: c_uint = 0xc80000ff	/* Mask for writable bits */;
pub const ARMV7_EVTYPE_EVENT: c_uint = 0xff		/* Mask for EVENT bits */;
//
// Event filters for PMUv2
//

//
// Secure debug enable reg
//

#[no_mangle]
pub unsafe extern "C" fn armv7_pmnc_read() -> u32 {
    static inline u32 armv7_pmnc_read(void)
    {
    u32 val;
    asm volatile("mrc p15, 0, %0, c9, c12, 0" : "=r"(val));
    return val;
    }
#[no_mangle]
pub unsafe extern "C" fn armv7_pmnc_write(val: u32) {
    static inline void armv7_pmnc_write(u32 val)
    {
    val &= ARMV7_PMNC_MASK;
    isb();
    asm volatile("mcr p15, 0, %0, c9, c12, 0" : : "r"(val));
    }
#[no_mangle]
pub unsafe extern "C" fn armv7_pmnc_has_overflowed(pmnc: u32) -> c_int {
    static inline int armv7_pmnc_has_overflowed(u32 pmnc)
    {
    return pmnc & ARMV7_OVERFLOWED_MASK;
    }
#[no_mangle]
pub unsafe extern "C" fn armv7_pmnc_counter_valid(cpu_pmu: *mut arm_pmu, idx: c_int) -> c_int {
    static inline int armv7_pmnc_counter_valid(struct arm_pmu *cpu_pmu, int idx)
    {
    return test_bit(idx, cpu_pmu.cntr_mask);
    }
#[no_mangle]
pub unsafe extern "C" fn armv7_pmnc_counter_has_overflowed(pmnc: u32, idx: c_int) -> c_int {
    static inline int armv7_pmnc_counter_has_overflowed(u32 pmnc, int idx)
    {
    return pmnc & BIT(idx);
    }
#[no_mangle]
pub unsafe extern "C" fn armv7_pmnc_select_counter(idx: c_int) {
    static inline void armv7_pmnc_select_counter(int idx)
    {
    asm volatile("mcr p15, 0, %0, c9, c12, 5" : : "r" (idx));
    isb();
    }
#[no_mangle]
pub unsafe extern "C" fn armv7pmu_read_counter(event: *mut perf_event) -> u64 {
    static inline u64 armv7pmu_read_counter(struct perf_event *event)
    {
    struct arm_pmu *cpu_pmu = to_arm_pmu(event.pmu);
    struct hw_perf_event *hwc = &event.hw;
    let mut idx: c_int = hwc.idx;
    let mut value: u32 = 0;
    if (!armv7_pmnc_counter_valid(cpu_pmu, idx)) {
    pr_err("CPU%u reading wrong counter %d\n",
    smp_processor_id(), idx);
    } else if (idx == ARMV7_IDX_CYCLE_COUNTER) {
    asm volatile("mrc p15, 0, %0, c9, c13, 0" : "=r" (value));
    } else {
    armv7_pmnc_select_counter(idx);
    asm volatile("mrc p15, 0, %0, c9, c13, 2" : "=r" (value));
    }
    return value;
    }
#[no_mangle]
pub unsafe extern "C" fn armv7pmu_write_counter(event: *mut perf_event, value: u64) {
    static inline void armv7pmu_write_counter(struct perf_event *event, u64 value)
    {
    struct arm_pmu *cpu_pmu = to_arm_pmu(event.pmu);
    struct hw_perf_event *hwc = &event.hw;
    let mut idx: c_int = hwc.idx;
    if (!armv7_pmnc_counter_valid(cpu_pmu, idx)) {
    pr_err("CPU%u writing wrong counter %d\n",
    smp_processor_id(), idx);
    } else if (idx == ARMV7_IDX_CYCLE_COUNTER) {
    asm volatile("mcr p15, 0, %0, c9, c13, 0" : : "r" ((u32)value));
    } else {
    armv7_pmnc_select_counter(idx);
    asm volatile("mcr p15, 0, %0, c9, c13, 2" : : "r" ((u32)value));
    }
    }
#[no_mangle]
pub unsafe extern "C" fn armv7_pmnc_write_evtsel(idx: c_int, val: u32) {
    static inline void armv7_pmnc_write_evtsel(int idx, u32 val)
    {
    armv7_pmnc_select_counter(idx);
    val &= ARMV7_EVTYPE_MASK;
    asm volatile("mcr p15, 0, %0, c9, c13, 1" : : "r" (val));
    }
#[no_mangle]
pub unsafe extern "C" fn armv7_pmnc_enable_counter(idx: c_int) {
    static inline void armv7_pmnc_enable_counter(int idx)
    {
    asm volatile("mcr p15, 0, %0, c9, c12, 1" : : "r" (BIT(idx)));
    }
#[no_mangle]
pub unsafe extern "C" fn armv7_pmnc_disable_counter(idx: c_int) {
    static inline void armv7_pmnc_disable_counter(int idx)
    {
    asm volatile("mcr p15, 0, %0, c9, c12, 2" : : "r" (BIT(idx)));
    }
#[no_mangle]
pub unsafe extern "C" fn armv7_pmnc_enable_intens(idx: c_int) {
    static inline void armv7_pmnc_enable_intens(int idx)
    {
    asm volatile("mcr p15, 0, %0, c9, c14, 1" : : "r" (BIT(idx)));
    }
#[no_mangle]
pub unsafe extern "C" fn armv7_pmnc_disable_intens(idx: c_int) {
    static inline void armv7_pmnc_disable_intens(int idx)
    {
    asm volatile("mcr p15, 0, %0, c9, c14, 2" : : "r" (BIT(idx)));
    isb();
// Clear the overflow flag in case an interrupt is pending.
    asm volatile("mcr p15, 0, %0, c9, c12, 3" : : "r" (BIT(idx)));
    isb();
    }
#[no_mangle]
pub unsafe extern "C" fn armv7_pmnc_getreset_flags() -> u32 {
    static inline u32 armv7_pmnc_getreset_flags(void)
    {
    u32 val;
// Read
    asm volatile("mrc p15, 0, %0, c9, c12, 3" : "=r" (val));
// Write to clear flags
    val &= ARMV7_FLAG_MASK;
    asm volatile("mcr p15, 0, %0, c9, c12, 3" : : "r" (val));
    return val;
    }

#[no_mangle]
unsafe extern "C" fn armv7_pmnc_dump_regs(cpu_pmu: *mut arm_pmu) {
    static void armv7_pmnc_dump_regs(struct arm_pmu *cpu_pmu)
    {
    u32 val;
    unsigned int cnt;
    pr_info("PMNC registers dump:\n");
    asm volatile("mrc p15, 0, %0, c9, c12, 0" : "=r" (val));
    pr_info("PMNC  =0x%08x\n", val);
    asm volatile("mrc p15, 0, %0, c9, c12, 1" : "=r" (val));
    pr_info("CNTENS=0x%08x\n", val);
    asm volatile("mrc p15, 0, %0, c9, c14, 1" : "=r" (val));
    pr_info("INTENS=0x%08x\n", val);
    asm volatile("mrc p15, 0, %0, c9, c12, 3" : "=r" (val));
    pr_info("FLAGS =0x%08x\n", val);
    asm volatile("mrc p15, 0, %0, c9, c12, 5" : "=r" (val));
    pr_info("SELECT=0x%08x\n", val);
    asm volatile("mrc p15, 0, %0, c9, c13, 0" : "=r" (val));
    pr_info("CCNT  =0x%08x\n", val);
    for_each_set_bit(cnt, cpu_pmu.cntr_mask, ARMV7_IDX_COUNTER_MAX) {
    armv7_pmnc_select_counter(cnt);
    asm volatile("mrc p15, 0, %0, c9, c13, 2" : "=r" (val));
    pr_info("CNT[%d] count =0x%08x\n", cnt, val);
    asm volatile("mrc p15, 0, %0, c9, c13, 1" : "=r" (val));
    pr_info("CNT[%d] evtsel=0x%08x\n", cnt, val);
    }
    }

#[no_mangle]
unsafe extern "C" fn armv7pmu_enable_event(event: *mut perf_event) {
    static void armv7pmu_enable_event(struct perf_event *event)
    {
    struct hw_perf_event *hwc = &event.hw;
    struct arm_pmu *cpu_pmu = to_arm_pmu(event.pmu);
    let mut idx: c_int = hwc.idx;
    if (!armv7_pmnc_counter_valid(cpu_pmu, idx)) {
    pr_err("CPU%u enabling wrong PMNC counter IRQ enable %d\n",
    smp_processor_id(), idx);
    return;
    }
//
// Set event (if destined for PMNx counters)
// We only need to set the event for the cycle counter if we
// have the ability to perform event filtering.
//
    if (cpu_pmu.set_event_filter || idx != ARMV7_IDX_CYCLE_COUNTER)
    armv7_pmnc_write_evtsel(idx, hwc.config_base);
    armv7_pmnc_enable_intens(idx);
    armv7_pmnc_enable_counter(idx);
    }
#[no_mangle]
unsafe extern "C" fn armv7pmu_disable_event(event: *mut perf_event) {
    static void armv7pmu_disable_event(struct perf_event *event)
    {
    struct hw_perf_event *hwc = &event.hw;
    struct arm_pmu *cpu_pmu = to_arm_pmu(event.pmu);
    let mut idx: c_int = hwc.idx;
    if (!armv7_pmnc_counter_valid(cpu_pmu, idx)) {
    pr_err("CPU%u disabling wrong PMNC counter IRQ enable %d\n",
    smp_processor_id(), idx);
    return;
    }
    armv7_pmnc_disable_counter(idx);
    armv7_pmnc_disable_intens(idx);
    }
#[no_mangle]
unsafe extern "C" fn armv7pmu_handle_irq(cpu_pmu: *mut arm_pmu) -> irqreturn_t {
    static irqreturn_t armv7pmu_handle_irq(struct arm_pmu *cpu_pmu)
    {
    u32 pmnc;
    struct perf_sample_data data;
    struct pmu_hw_events *cpuc = this_cpu_ptr(cpu_pmu.hw_events);
    struct pt_regs *regs;
    int idx;
//
// Get and reset the IRQ flags
//
    pmnc = armv7_pmnc_getreset_flags();
//
// Did an overflow occur?
//
    if (!armv7_pmnc_has_overflowed(pmnc))
    return IRQ_NONE;
//
// Handle the counter(s) overflow(s)
//
    regs = get_irq_regs();
    for_each_set_bit(idx, cpu_pmu.cntr_mask, ARMPMU_MAX_HWEVENTS) {
    struct perf_event *event = cpuc.events[idx];
    struct hw_perf_event *hwc;
// Ignore if we don't have an event.
    if (!event)
    continue;
//
// We have a single interrupt for all counters. Check that
// each counter has overflowed before we process it.
//
    if (!armv7_pmnc_counter_has_overflowed(pmnc, idx))
    continue;
    hwc = &event.hw;
    armpmu_event_update(event);
    perf_sample_data_init(&data, 0, hwc.last_period);
    if (!armpmu_event_set_period(event))
    continue;
    perf_event_overflow(event, &data, regs);
    }
//
// Handle the pending perf events.
//
// Note: this call *must* be run with interrupts disabled. For
// platforms that can have the PMU interrupts raised as an NMI, this
// will not work.
//
    irq_work_run();
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn armv7pmu_start(cpu_pmu: *mut arm_pmu) {
    static void armv7pmu_start(struct arm_pmu *cpu_pmu)
    {
// Enable all counters
    armv7_pmnc_write(armv7_pmnc_read() | ARMV7_PMNC_E);
    }
#[no_mangle]
unsafe extern "C" fn armv7pmu_stop(cpu_pmu: *mut arm_pmu) {
    static void armv7pmu_stop(struct arm_pmu *cpu_pmu)
    {
// Disable all counters
    armv7_pmnc_write(armv7_pmnc_read() & ~ARMV7_PMNC_E);
    }
    static int armv7pmu_get_event_idx(struct pmu_hw_events *cpuc,
    struct perf_event *event)
    {
    int idx;
    struct arm_pmu *cpu_pmu = to_arm_pmu(event.pmu);
    struct hw_perf_event *hwc = &event.hw;
    let mut evtype: c_ulong = hwc.config_base & ARMV7_EVTYPE_EVENT;
// Always place a cycle counter into the cycle counter.
    if (evtype == ARMV7_PERFCTR_CPU_CYCLES) {
    if (test_and_set_bit(ARMV7_IDX_CYCLE_COUNTER, cpuc.used_mask))
    return -EAGAIN;
    return ARMV7_IDX_CYCLE_COUNTER;
    }
//
// For anything other than a cycle counter, try and use
// the events counters
//
    for_each_set_bit(idx, cpu_pmu.cntr_mask, ARMV7_IDX_COUNTER_MAX) {
    if (!test_and_set_bit(idx, cpuc.used_mask))
    return idx;
    }
// The counters are all in use.
    return -EAGAIN;
    }
    static void armv7pmu_clear_event_idx(struct pmu_hw_events *cpuc,
    struct perf_event *event)
    {
    clear_bit(event.hw.idx, cpuc.used_mask);
    }
//
// Add an event filter to a given event. This will only work for PMUv2 PMUs.
//
    static int armv7pmu_set_event_filter(struct hw_perf_event *event,
    struct perf_event_attr *attr)
    {
    let mut config_base: c_ulong = 0;
    if (attr.exclude_idle) {
    pr_debug("ARM performance counters do not support mode exclusion\n");
    return -EOPNOTSUPP;
    }
    if (attr.exclude_user)
    config_base |= ARMV7_EXCLUDE_USER;
    if (attr.exclude_kernel)
    config_base |= ARMV7_EXCLUDE_PL1;
    if (!attr.exclude_hv)
    config_base |= ARMV7_INCLUDE_HYP;
//
// Install the filter into config_base as this is used to
// construct the event type.
//
    event.config_base = config_base;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn armv7pmu_reset(info: *mut c_void) {
    static void armv7pmu_reset(void *info)
    {
    struct arm_pmu *cpu_pmu = (struct arm_pmu *)info;
    u32 idx, val;
    if (cpu_pmu.secure_access) {
    asm volatile("mrc p15, 0, %0, c1, c1, 1" : "=r" (val));
    val |= ARMV7_SDER_SUNIDEN;
    asm volatile("mcr p15, 0, %0, c1, c1, 1" : : "r" (val));
    }
// The counter and interrupt enable registers are unknown at reset.
    for_each_set_bit(idx, cpu_pmu.cntr_mask, ARMPMU_MAX_HWEVENTS) {
    armv7_pmnc_disable_counter(idx);
    armv7_pmnc_disable_intens(idx);
    }
// Initialize & Reset PMNC: C and P bits
    armv7_pmnc_write(ARMV7_PMNC_P | ARMV7_PMNC_C);
    }
#[no_mangle]
unsafe extern "C" fn armv7_a8_map_event(event: *mut perf_event) -> c_int {
    static int armv7_a8_map_event(struct perf_event *event)
    {
    return armpmu_map_event(event, &armv7_a8_perf_map,
    &armv7_a8_perf_cache_map, 0xFF);
    }
#[no_mangle]
unsafe extern "C" fn armv7_a9_map_event(event: *mut perf_event) -> c_int {
    static int armv7_a9_map_event(struct perf_event *event)
    {
    return armpmu_map_event(event, &armv7_a9_perf_map,
    &armv7_a9_perf_cache_map, 0xFF);
    }
#[no_mangle]
unsafe extern "C" fn armv7_a5_map_event(event: *mut perf_event) -> c_int {
    static int armv7_a5_map_event(struct perf_event *event)
    {
    return armpmu_map_event(event, &armv7_a5_perf_map,
    &armv7_a5_perf_cache_map, 0xFF);
    }
#[no_mangle]
unsafe extern "C" fn armv7_a15_map_event(event: *mut perf_event) -> c_int {
    static int armv7_a15_map_event(struct perf_event *event)
    {
    return armpmu_map_event(event, &armv7_a15_perf_map,
    &armv7_a15_perf_cache_map, 0xFF);
    }
#[no_mangle]
unsafe extern "C" fn armv7_a7_map_event(event: *mut perf_event) -> c_int {
    static int armv7_a7_map_event(struct perf_event *event)
    {
    return armpmu_map_event(event, &armv7_a7_perf_map,
    &armv7_a7_perf_cache_map, 0xFF);
    }
#[no_mangle]
unsafe extern "C" fn armv7_a12_map_event(event: *mut perf_event) -> c_int {
    static int armv7_a12_map_event(struct perf_event *event)
    {
    return armpmu_map_event(event, &armv7_a12_perf_map,
    &armv7_a12_perf_cache_map, 0xFF);
    }
#[no_mangle]
unsafe extern "C" fn krait_map_event(event: *mut perf_event) -> c_int {
    static int krait_map_event(struct perf_event *event)
    {
    return armpmu_map_event(event, &krait_perf_map,
    &krait_perf_cache_map, 0xFFFFF);
    }
#[no_mangle]
unsafe extern "C" fn krait_map_event_no_branch(event: *mut perf_event) -> c_int {
    static int krait_map_event_no_branch(struct perf_event *event)
    {
    return armpmu_map_event(event, &krait_perf_map_no_branch,
    &krait_perf_cache_map, 0xFFFFF);
    }
#[no_mangle]
unsafe extern "C" fn scorpion_map_event(event: *mut perf_event) -> c_int {
    static int scorpion_map_event(struct perf_event *event)
    {
    return armpmu_map_event(event, &scorpion_perf_map,
    &scorpion_perf_cache_map, 0xFFFFF);
    }
#[no_mangle]
unsafe extern "C" fn armv7pmu_init(cpu_pmu: *mut arm_pmu) {
    static void armv7pmu_init(struct arm_pmu *cpu_pmu)
    {
    cpu_pmu.handle_irq	= armv7pmu_handle_irq;
    cpu_pmu.enable		= armv7pmu_enable_event;
    cpu_pmu.disable	= armv7pmu_disable_event;
    cpu_pmu.read_counter	= armv7pmu_read_counter;
    cpu_pmu.write_counter	= armv7pmu_write_counter;
    cpu_pmu.get_event_idx	= armv7pmu_get_event_idx;
    cpu_pmu.clear_event_idx = armv7pmu_clear_event_idx;
    cpu_pmu.start		= armv7pmu_start;
    cpu_pmu.stop		= armv7pmu_stop;
    cpu_pmu.reset		= armv7pmu_reset;
    };
#[no_mangle]
unsafe extern "C" fn armv7_read_num_pmnc_events(info: *mut c_void) {
    static void armv7_read_num_pmnc_events(void *info)
    {
    int nb_cnt;
    struct arm_pmu *cpu_pmu = info;
// Read the nb of CNTx counters supported from PMNC
    nb_cnt = (armv7_pmnc_read() >> ARMV7_PMNC_N_SHIFT) & ARMV7_PMNC_N_MASK;
    bitmap_set(cpu_pmu.cntr_mask, 0, nb_cnt);
// Add the CPU cycles counter
    set_bit(ARMV7_IDX_CYCLE_COUNTER, cpu_pmu.cntr_mask);
    }
#[no_mangle]
unsafe extern "C" fn armv7_probe_num_events(arm_pmu: *mut arm_pmu) -> c_int {
    static int armv7_probe_num_events(struct arm_pmu *arm_pmu)
    {
    return smp_call_function_any(&arm_pmu.supported_cpus,
    armv7_read_num_pmnc_events,
    arm_pmu, 1);
    }
#[no_mangle]
unsafe extern "C" fn armv7_a8_pmu_init(cpu_pmu: *mut arm_pmu) -> c_int {
    static int armv7_a8_pmu_init(struct arm_pmu *cpu_pmu)
    {
    armv7pmu_init(cpu_pmu);
    cpu_pmu.name		= "armv7_cortex_a8";
    cpu_pmu.map_event	= armv7_a8_map_event;
    cpu_pmu.attr_groups[ARMPMU_ATTR_GROUP_EVENTS] =
    &armv7_pmuv1_events_attr_group;
    cpu_pmu.attr_groups[ARMPMU_ATTR_GROUP_FORMATS] =
    &armv7_pmu_format_attr_group;
    return armv7_probe_num_events(cpu_pmu);
    }
#[no_mangle]
unsafe extern "C" fn armv7_a9_pmu_init(cpu_pmu: *mut arm_pmu) -> c_int {
    static int armv7_a9_pmu_init(struct arm_pmu *cpu_pmu)
    {
    armv7pmu_init(cpu_pmu);
    cpu_pmu.name		= "armv7_cortex_a9";
    cpu_pmu.map_event	= armv7_a9_map_event;
    cpu_pmu.attr_groups[ARMPMU_ATTR_GROUP_EVENTS] =
    &armv7_pmuv1_events_attr_group;
    cpu_pmu.attr_groups[ARMPMU_ATTR_GROUP_FORMATS] =
    &armv7_pmu_format_attr_group;
    return armv7_probe_num_events(cpu_pmu);
    }
#[no_mangle]
unsafe extern "C" fn armv7_a5_pmu_init(cpu_pmu: *mut arm_pmu) -> c_int {
    static int armv7_a5_pmu_init(struct arm_pmu *cpu_pmu)
    {
    armv7pmu_init(cpu_pmu);
    cpu_pmu.name		= "armv7_cortex_a5";
    cpu_pmu.map_event	= armv7_a5_map_event;
    cpu_pmu.attr_groups[ARMPMU_ATTR_GROUP_EVENTS] =
    &armv7_pmuv1_events_attr_group;
    cpu_pmu.attr_groups[ARMPMU_ATTR_GROUP_FORMATS] =
    &armv7_pmu_format_attr_group;
    return armv7_probe_num_events(cpu_pmu);
    }
#[no_mangle]
unsafe extern "C" fn armv7_a15_pmu_init(cpu_pmu: *mut arm_pmu) -> c_int {
    static int armv7_a15_pmu_init(struct arm_pmu *cpu_pmu)
    {
    armv7pmu_init(cpu_pmu);
    cpu_pmu.name		= "armv7_cortex_a15";
    cpu_pmu.map_event	= armv7_a15_map_event;
    cpu_pmu.set_event_filter = armv7pmu_set_event_filter;
    cpu_pmu.attr_groups[ARMPMU_ATTR_GROUP_EVENTS] =
    &armv7_pmuv2_events_attr_group;
    cpu_pmu.attr_groups[ARMPMU_ATTR_GROUP_FORMATS] =
    &armv7_pmu_format_attr_group;
    return armv7_probe_num_events(cpu_pmu);
    }
#[no_mangle]
unsafe extern "C" fn armv7_a7_pmu_init(cpu_pmu: *mut arm_pmu) -> c_int {
    static int armv7_a7_pmu_init(struct arm_pmu *cpu_pmu)
    {
    armv7pmu_init(cpu_pmu);
    cpu_pmu.name		= "armv7_cortex_a7";
    cpu_pmu.map_event	= armv7_a7_map_event;
    cpu_pmu.set_event_filter = armv7pmu_set_event_filter;
    cpu_pmu.attr_groups[ARMPMU_ATTR_GROUP_EVENTS] =
    &armv7_pmuv2_events_attr_group;
    cpu_pmu.attr_groups[ARMPMU_ATTR_GROUP_FORMATS] =
    &armv7_pmu_format_attr_group;
    return armv7_probe_num_events(cpu_pmu);
    }
#[no_mangle]
unsafe extern "C" fn armv7_a12_pmu_init(cpu_pmu: *mut arm_pmu) -> c_int {
    static int armv7_a12_pmu_init(struct arm_pmu *cpu_pmu)
    {
    armv7pmu_init(cpu_pmu);
    cpu_pmu.name		= "armv7_cortex_a12";
    cpu_pmu.map_event	= armv7_a12_map_event;
    cpu_pmu.set_event_filter = armv7pmu_set_event_filter;
    cpu_pmu.attr_groups[ARMPMU_ATTR_GROUP_EVENTS] =
    &armv7_pmuv2_events_attr_group;
    cpu_pmu.attr_groups[ARMPMU_ATTR_GROUP_FORMATS] =
    &armv7_pmu_format_attr_group;
    return armv7_probe_num_events(cpu_pmu);
    }
#[no_mangle]
unsafe extern "C" fn armv7_a17_pmu_init(cpu_pmu: *mut arm_pmu) -> c_int {
    static int armv7_a17_pmu_init(struct arm_pmu *cpu_pmu)
    {
    let mut ret: c_int = armv7_a12_pmu_init(cpu_pmu);
    cpu_pmu.name = "armv7_cortex_a17";
    cpu_pmu.attr_groups[ARMPMU_ATTR_GROUP_EVENTS] =
    &armv7_pmuv2_events_attr_group;
    cpu_pmu.attr_groups[ARMPMU_ATTR_GROUP_FORMATS] =
    &armv7_pmu_format_attr_group;
    return ret;
    }
//
// Krait Performance Monitor Region Event Selection Register (PMRESRn)
//
// 31   30     24     16     8      0
// +--------------------------------+
// PMRESR0   | EN |  CC  |  CC  |  CC  |  CC  |   N = 1, R = 0
// +--------------------------------+
// PMRESR1   | EN |  CC  |  CC  |  CC  |  CC  |   N = 1, R = 1
// +--------------------------------+
// PMRESR2   | EN |  CC  |  CC  |  CC  |  CC  |   N = 1, R = 2
// +--------------------------------+
// VPMRESR0  | EN |  CC  |  CC  |  CC  |  CC  |   N = 2, R = ?
// +--------------------------------+
// EN | G=3  | G=2  | G=1  | G=0
//
// Event Encoding:
//
// hwc->config_base = 0xNRCCG
//
// N  = prefix, 1 for Krait CPU (PMRESRn), 2 for Venum VFP (VPMRESR)
// R  = region register
// CC = class of events the group G is choosing from
// G  = group or particular event
//
// Example: 0x12021 is a Krait CPU event in PMRESR2's group 1 with code 2
//
// A region (R) corresponds to a piece of the CPU (execution unit, instruction
// unit, etc.) while the event code (CC) corresponds to a particular class of
// events (interrupts for example). An event code is broken down into
// groups (G) that can be mapped into the PMU (irq, fiqs, and irq+fiqs for
// example).
//

#[no_mangle]
unsafe extern "C" fn krait_read_pmresrn(n: c_int) -> u32 {
    static u32 krait_read_pmresrn(int n)
    {
    u32 val;
    switch (n) {
    case 0:
    asm volatile("mrc p15, 1, %0, c9, c15, 0" : "=r" (val));
    break;
    case 1:
    asm volatile("mrc p15, 1, %0, c9, c15, 1" : "=r" (val));
    break;
    case 2:
    asm volatile("mrc p15, 1, %0, c9, c15, 2" : "=r" (val));
    break;
    default:
    BUG(); /* Should be validated in krait_pmu_get_event_idx() */
    }
    return val;
    }
#[no_mangle]
unsafe extern "C" fn krait_write_pmresrn(n: c_int, val: u32) {
    static void krait_write_pmresrn(int n, u32 val)
    {
    switch (n) {
    case 0:
    asm volatile("mcr p15, 1, %0, c9, c15, 0" : : "r" (val));
    break;
    case 1:
    asm volatile("mcr p15, 1, %0, c9, c15, 1" : : "r" (val));
    break;
    case 2:
    asm volatile("mcr p15, 1, %0, c9, c15, 2" : : "r" (val));
    break;
    default:
    BUG(); /* Should be validated in krait_pmu_get_event_idx() */
    }
    }
#[no_mangle]
unsafe extern "C" fn venum_read_pmresr() -> u32 {
    static u32 venum_read_pmresr(void)
    {
    u32 val;
    asm volatile("mrc p10, 7, %0, c11, c0, 0" : "=r" (val));
    return val;
    }
#[no_mangle]
unsafe extern "C" fn venum_write_pmresr(val: u32) {
    static void venum_write_pmresr(u32 val)
    {
    asm volatile("mcr p10, 7, %0, c11, c0, 0" : : "r" (val));
    }
#[no_mangle]
unsafe extern "C" fn venum_pre_pmresr(venum_orig_val: *mut u32, fp_orig_val: *mut u32) {
    static void venum_pre_pmresr(u32 *venum_orig_val, u32 *fp_orig_val)
    {
    u32 venum_new_val;
    u32 fp_new_val;
    BUG_ON(preemptible());
// CPACR Enable CP10 and CP11 access
// venum_orig_val = get_copro_access();
    venum_new_val = *venum_orig_val | CPACC_SVC(10) | CPACC_SVC(11);
    set_copro_access(venum_new_val);
// Enable FPEXC
// fp_orig_val = fmrx(FPEXC);
    fp_new_val = *fp_orig_val | FPEXC_EN;
    fmxr(FPEXC, fp_new_val);
    }
#[no_mangle]
unsafe extern "C" fn venum_post_pmresr(venum_orig_val: u32, fp_orig_val: u32) {
    static void venum_post_pmresr(u32 venum_orig_val, u32 fp_orig_val)
    {
    BUG_ON(preemptible());
// Restore FPEXC
    fmxr(FPEXC, fp_orig_val);
    isb();
// Restore CPACR
    set_copro_access(venum_orig_val);
    }
#[no_mangle]
unsafe extern "C" fn krait_get_pmresrn_event(region: c_uint) -> u32 {
    static u32 krait_get_pmresrn_event(unsigned int region)
    {
    static const u32 pmresrn_table[] = { KRAIT_PMRESR0_GROUP0,
    KRAIT_PMRESR1_GROUP0,
    KRAIT_PMRESR2_GROUP0 };
    return pmresrn_table[region];
    }
#[no_mangle]
unsafe extern "C" fn krait_evt_setup(idx: c_int, config_base: u32) {
    static void krait_evt_setup(int idx, u32 config_base)
    {
    u32 val;
    u32 mask;
    u32 vval, fval;
    let mut region: c_uint = EVENT_REGION(config_base);
    let mut group: c_uint = EVENT_GROUP(config_base);
    let mut code: c_uint = EVENT_CODE(config_base);
    unsigned int group_shift;
    let mut venum_event: bool = EVENT_VENUM(config_base);
    group_shift = group * 8;
    mask = 0xff << group_shift;
// Configure evtsel for the region and group
    if (venum_event)
    val = KRAIT_VPMRESR0_GROUP0;
    else
    val = krait_get_pmresrn_event(region);
    val += group;
// Mix in mode-exclusion bits
    val |= config_base & (ARMV7_EXCLUDE_USER | ARMV7_EXCLUDE_PL1);
    armv7_pmnc_write_evtsel(idx, val);
    if (venum_event) {
    venum_pre_pmresr(&vval, &fval);
    val = venum_read_pmresr();
    val &= ~mask;
    val |= code << group_shift;
    val |= PMRESRn_EN;
    venum_write_pmresr(val);
    venum_post_pmresr(vval, fval);
    } else {
    val = krait_read_pmresrn(region);
    val &= ~mask;
    val |= code << group_shift;
    val |= PMRESRn_EN;
    krait_write_pmresrn(region, val);
    }
    }
#[no_mangle]
unsafe extern "C" fn clear_pmresrn_group(val: u32, group: c_int) -> u32 {
    static u32 clear_pmresrn_group(u32 val, int group)
    {
    u32 mask;
    int group_shift;
    group_shift = group * 8;
    mask = 0xff << group_shift;
    val &= ~mask;
// Don't clear enable bit if entire region isn't disabled
    if (val & ~PMRESRn_EN)
    return val |= PMRESRn_EN;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn krait_clearpmu(config_base: u32) {
    static void krait_clearpmu(u32 config_base)
    {
    u32 val;
    u32 vval, fval;
    let mut region: c_uint = EVENT_REGION(config_base);
    let mut group: c_uint = EVENT_GROUP(config_base);
    let mut venum_event: bool = EVENT_VENUM(config_base);
    if (venum_event) {
    venum_pre_pmresr(&vval, &fval);
    val = venum_read_pmresr();
    val = clear_pmresrn_group(val, group);
    venum_write_pmresr(val);
    venum_post_pmresr(vval, fval);
    } else {
    val = krait_read_pmresrn(region);
    val = clear_pmresrn_group(val, group);
    krait_write_pmresrn(region, val);
    }
    }
#[no_mangle]
unsafe extern "C" fn krait_pmu_disable_event(event: *mut perf_event) {
    static void krait_pmu_disable_event(struct perf_event *event)
    {
    struct hw_perf_event *hwc = &event.hw;
    let mut idx: c_int = hwc.idx;
// Disable counter and interrupt
// Disable counter
    armv7_pmnc_disable_counter(idx);
//
// Clear pmresr code (if destined for PMNx counters)
//
    if (hwc.config_base & KRAIT_EVENT_MASK)
    krait_clearpmu(hwc.config_base);
// Disable interrupt for this counter
    armv7_pmnc_disable_intens(idx);
    }
#[no_mangle]
unsafe extern "C" fn krait_pmu_enable_event(event: *mut perf_event) {
    static void krait_pmu_enable_event(struct perf_event *event)
    {
    struct hw_perf_event *hwc = &event.hw;
    let mut idx: c_int = hwc.idx;
//
// Set event (if destined for PMNx counters)
// We set the event for the cycle counter because we
// have the ability to perform event filtering.
//
    if (hwc.config_base & KRAIT_EVENT_MASK)
    krait_evt_setup(idx, hwc.config_base);
    else
    armv7_pmnc_write_evtsel(idx, hwc.config_base);
    armv7_pmnc_enable_intens(idx);
    armv7_pmnc_enable_counter(idx);
    }
#[no_mangle]
unsafe extern "C" fn krait_pmu_reset(info: *mut c_void) {
    static void krait_pmu_reset(void *info)
    {
    u32 vval, fval;
    struct arm_pmu *cpu_pmu = info;
    u32 idx;
    armv7pmu_reset(info);
// Clear all pmresrs
    krait_write_pmresrn(0, 0);
    krait_write_pmresrn(1, 0);
    krait_write_pmresrn(2, 0);
    venum_pre_pmresr(&vval, &fval);
    venum_write_pmresr(0);
    venum_post_pmresr(vval, fval);
// Reset PMxEVNCTCR to sane default
    for_each_set_bit(idx, cpu_pmu.cntr_mask, ARMV7_IDX_COUNTER_MAX) {
    armv7_pmnc_select_counter(idx);
    asm volatile("mcr p15, 0, %0, c9, c15, 0" : : "r" (0));
    }
    }
    static int krait_event_to_bit(struct perf_event *event, unsigned int region,
    unsigned int group)
    {
    int bit;
    struct hw_perf_event *hwc = &event.hw;
    struct arm_pmu *cpu_pmu = to_arm_pmu(event.pmu);
    if (hwc.config_base & VENUM_EVENT)
    bit = KRAIT_VPMRESR0_GROUP0;
    else
    bit = krait_get_pmresrn_event(region);
    bit -= krait_get_pmresrn_event(0);
    bit += group;
//
// Lower bits are reserved for use by the counters (see
// armv7pmu_get_event_idx() for more info)
//
    bit += bitmap_weight(cpu_pmu.cntr_mask, ARMV7_IDX_COUNTER_MAX);
    return bit;
    }
//
// We check for column exclusion constraints here.
// Two events cant use the same group within a pmresr register.
//
    static int krait_pmu_get_event_idx(struct pmu_hw_events *cpuc,
    struct perf_event *event)
    {
    int idx;
    let mut bit: c_int = -1;
    struct hw_perf_event *hwc = &event.hw;
    let mut region: c_uint = EVENT_REGION(hwc.config_base);
    let mut code: c_uint = EVENT_CODE(hwc.config_base);
    let mut group: c_uint = EVENT_GROUP(hwc.config_base);
    let mut venum_event: bool = EVENT_VENUM(hwc.config_base);
    let mut krait_event: bool = EVENT_CPU(hwc.config_base);
    if (venum_event || krait_event) {
// Ignore invalid events
    if (group > 3 || region > 2)
    return -EINVAL;
    if (venum_event && (code & 0xe0))
    return -EINVAL;
    bit = krait_event_to_bit(event, region, group);
    if (test_and_set_bit(bit, cpuc.used_mask))
    return -EAGAIN;
    }
    idx = armv7pmu_get_event_idx(cpuc, event);
    if (idx < 0 && bit >= 0)
    clear_bit(bit, cpuc.used_mask);
    return idx;
    }
    static void krait_pmu_clear_event_idx(struct pmu_hw_events *cpuc,
    struct perf_event *event)
    {
    int bit;
    struct hw_perf_event *hwc = &event.hw;
    let mut region: c_uint = EVENT_REGION(hwc.config_base);
    let mut group: c_uint = EVENT_GROUP(hwc.config_base);
    let mut venum_event: bool = EVENT_VENUM(hwc.config_base);
    let mut krait_event: bool = EVENT_CPU(hwc.config_base);
    armv7pmu_clear_event_idx(cpuc, event);
    if (venum_event || krait_event) {
    bit = krait_event_to_bit(event, region, group);
    clear_bit(bit, cpuc.used_mask);
    }
    }
#[no_mangle]
unsafe extern "C" fn krait_pmu_init(cpu_pmu: *mut arm_pmu) -> c_int {
    static int krait_pmu_init(struct arm_pmu *cpu_pmu)
    {
    armv7pmu_init(cpu_pmu);
    cpu_pmu.name		= "armv7_krait";
// Some early versions of Krait don't support PC write events
    if (of_property_read_bool(cpu_pmu.plat_device.dev.of_node,
    "qcom,no-pc-write"))
    cpu_pmu.map_event = krait_map_event_no_branch;
    else
    cpu_pmu.map_event = krait_map_event;
    cpu_pmu.set_event_filter = armv7pmu_set_event_filter;
    cpu_pmu.reset		= krait_pmu_reset;
    cpu_pmu.enable		= krait_pmu_enable_event;
    cpu_pmu.disable	= krait_pmu_disable_event;
    cpu_pmu.get_event_idx	= krait_pmu_get_event_idx;
    cpu_pmu.clear_event_idx = krait_pmu_clear_event_idx;
    return armv7_probe_num_events(cpu_pmu);
    }
//
// Scorpion Local Performance Monitor Register (LPMn)
//
// 31   30     24     16     8      0
// +--------------------------------+
// LPM0      | EN |  CC  |  CC  |  CC  |  CC  |   N = 1, R = 0
// +--------------------------------+
// LPM1      | EN |  CC  |  CC  |  CC  |  CC  |   N = 1, R = 1
// +--------------------------------+
// LPM2      | EN |  CC  |  CC  |  CC  |  CC  |   N = 1, R = 2
// +--------------------------------+
// L2LPM     | EN |  CC  |  CC  |  CC  |  CC  |   N = 1, R = 3
// +--------------------------------+
// VLPM      | EN |  CC  |  CC  |  CC  |  CC  |   N = 2, R = ?
// +--------------------------------+
// EN | G=3  | G=2  | G=1  | G=0
//
// Event Encoding:
//
// hwc->config_base = 0xNRCCG
//
// N  = prefix, 1 for Scorpion CPU (LPMn/L2LPM), 2 for Venum VFP (VLPM)
// R  = region register
// CC = class of events the group G is choosing from
// G  = group or particular event
//
// Example: 0x12021 is a Scorpion CPU event in LPM2's group 1 with code 2
//
// A region (R) corresponds to a piece of the CPU (execution unit, instruction
// unit, etc.) while the event code (CC) corresponds to a particular class of
// events (interrupts for example). An event code is broken down into
// groups (G) that can be mapped into the PMU (irq, fiqs, and irq+fiqs for
// example).
//
#[no_mangle]
unsafe extern "C" fn scorpion_read_pmresrn(n: c_int) -> u32 {
    static u32 scorpion_read_pmresrn(int n)
    {
    u32 val;
    switch (n) {
    case 0:
    asm volatile("mrc p15, 0, %0, c15, c0, 0" : "=r" (val));
    break;
    case 1:
    asm volatile("mrc p15, 1, %0, c15, c0, 0" : "=r" (val));
    break;
    case 2:
    asm volatile("mrc p15, 2, %0, c15, c0, 0" : "=r" (val));
    break;
    case 3:
    asm volatile("mrc p15, 3, %0, c15, c2, 0" : "=r" (val));
    break;
    default:
    BUG(); /* Should be validated in scorpion_pmu_get_event_idx() */
    }
    return val;
    }
#[no_mangle]
unsafe extern "C" fn scorpion_write_pmresrn(n: c_int, val: u32) {
    static void scorpion_write_pmresrn(int n, u32 val)
    {
    switch (n) {
    case 0:
    asm volatile("mcr p15, 0, %0, c15, c0, 0" : : "r" (val));
    break;
    case 1:
    asm volatile("mcr p15, 1, %0, c15, c0, 0" : : "r" (val));
    break;
    case 2:
    asm volatile("mcr p15, 2, %0, c15, c0, 0" : : "r" (val));
    break;
    case 3:
    asm volatile("mcr p15, 3, %0, c15, c2, 0" : : "r" (val));
    break;
    default:
    BUG(); /* Should be validated in scorpion_pmu_get_event_idx() */
    }
    }
#[no_mangle]
unsafe extern "C" fn scorpion_get_pmresrn_event(region: c_uint) -> u32 {
    static u32 scorpion_get_pmresrn_event(unsigned int region)
    {
    static const u32 pmresrn_table[] = { SCORPION_LPM0_GROUP0,
    SCORPION_LPM1_GROUP0,
    SCORPION_LPM2_GROUP0,
    SCORPION_L2LPM_GROUP0 };
    return pmresrn_table[region];
    }
#[no_mangle]
unsafe extern "C" fn scorpion_evt_setup(idx: c_int, config_base: u32) {
    static void scorpion_evt_setup(int idx, u32 config_base)
    {
    u32 val;
    u32 mask;
    u32 vval, fval;
    let mut region: c_uint = EVENT_REGION(config_base);
    let mut group: c_uint = EVENT_GROUP(config_base);
    let mut code: c_uint = EVENT_CODE(config_base);
    unsigned int group_shift;
    let mut venum_event: bool = EVENT_VENUM(config_base);
    group_shift = group * 8;
    mask = 0xff << group_shift;
// Configure evtsel for the region and group
    if (venum_event)
    val = SCORPION_VLPM_GROUP0;
    else
    val = scorpion_get_pmresrn_event(region);
    val += group;
// Mix in mode-exclusion bits
    val |= config_base & (ARMV7_EXCLUDE_USER | ARMV7_EXCLUDE_PL1);
    armv7_pmnc_write_evtsel(idx, val);
    asm volatile("mcr p15, 0, %0, c9, c15, 0" : : "r" (0));
    if (venum_event) {
    venum_pre_pmresr(&vval, &fval);
    val = venum_read_pmresr();
    val &= ~mask;
    val |= code << group_shift;
    val |= PMRESRn_EN;
    venum_write_pmresr(val);
    venum_post_pmresr(vval, fval);
    } else {
    val = scorpion_read_pmresrn(region);
    val &= ~mask;
    val |= code << group_shift;
    val |= PMRESRn_EN;
    scorpion_write_pmresrn(region, val);
    }
    }
#[no_mangle]
unsafe extern "C" fn scorpion_clearpmu(config_base: u32) {
    static void scorpion_clearpmu(u32 config_base)
    {
    u32 val;
    u32 vval, fval;
    let mut region: c_uint = EVENT_REGION(config_base);
    let mut group: c_uint = EVENT_GROUP(config_base);
    let mut venum_event: bool = EVENT_VENUM(config_base);
    if (venum_event) {
    venum_pre_pmresr(&vval, &fval);
    val = venum_read_pmresr();
    val = clear_pmresrn_group(val, group);
    venum_write_pmresr(val);
    venum_post_pmresr(vval, fval);
    } else {
    val = scorpion_read_pmresrn(region);
    val = clear_pmresrn_group(val, group);
    scorpion_write_pmresrn(region, val);
    }
    }
#[no_mangle]
unsafe extern "C" fn scorpion_pmu_disable_event(event: *mut perf_event) {
    static void scorpion_pmu_disable_event(struct perf_event *event)
    {
    struct hw_perf_event *hwc = &event.hw;
    let mut idx: c_int = hwc.idx;
// Disable counter and interrupt
// Disable counter
    armv7_pmnc_disable_counter(idx);
//
// Clear pmresr code (if destined for PMNx counters)
//
    if (hwc.config_base & KRAIT_EVENT_MASK)
    scorpion_clearpmu(hwc.config_base);
// Disable interrupt for this counter
    armv7_pmnc_disable_intens(idx);
    }
#[no_mangle]
unsafe extern "C" fn scorpion_pmu_enable_event(event: *mut perf_event) {
    static void scorpion_pmu_enable_event(struct perf_event *event)
    {
    struct hw_perf_event *hwc = &event.hw;
    let mut idx: c_int = hwc.idx;
//
// Set event (if destined for PMNx counters)
// We don't set the event for the cycle counter because we
// don't have the ability to perform event filtering.
//
    if (hwc.config_base & KRAIT_EVENT_MASK)
    scorpion_evt_setup(idx, hwc.config_base);
#[no_mangle]
pub unsafe extern "C" fn if(ARMV7_IDX_CYCLE_COUNTER: idx !=) -> else {
    else if (idx != ARMV7_IDX_CYCLE_COUNTER)
    armv7_pmnc_write_evtsel(idx, hwc.config_base);
    armv7_pmnc_enable_intens(idx);
    armv7_pmnc_enable_counter(idx);
    }
#[no_mangle]
unsafe extern "C" fn scorpion_pmu_reset(info: *mut c_void) {
    static void scorpion_pmu_reset(void *info)
    {
    u32 vval, fval;
    struct arm_pmu *cpu_pmu = info;
    u32 idx;
    armv7pmu_reset(info);
// Clear all pmresrs
    scorpion_write_pmresrn(0, 0);
    scorpion_write_pmresrn(1, 0);
    scorpion_write_pmresrn(2, 0);
    scorpion_write_pmresrn(3, 0);
    venum_pre_pmresr(&vval, &fval);
    venum_write_pmresr(0);
    venum_post_pmresr(vval, fval);
// Reset PMxEVNCTCR to sane default
    for_each_set_bit(idx, cpu_pmu.cntr_mask, ARMV7_IDX_COUNTER_MAX) {
    armv7_pmnc_select_counter(idx);
    asm volatile("mcr p15, 0, %0, c9, c15, 0" : : "r" (0));
    }
    }
    static int scorpion_event_to_bit(struct perf_event *event, unsigned int region,
    unsigned int group)
    {
    int bit;
    struct hw_perf_event *hwc = &event.hw;
    struct arm_pmu *cpu_pmu = to_arm_pmu(event.pmu);
    if (hwc.config_base & VENUM_EVENT)
    bit = SCORPION_VLPM_GROUP0;
    else
    bit = scorpion_get_pmresrn_event(region);
    bit -= scorpion_get_pmresrn_event(0);
    bit += group;
//
// Lower bits are reserved for use by the counters (see
// armv7pmu_get_event_idx() for more info)
//
    bit += bitmap_weight(cpu_pmu.cntr_mask, ARMV7_IDX_COUNTER_MAX);
    return bit;
    }
//
// We check for column exclusion constraints here.
// Two events cant use the same group within a pmresr register.
//
    static int scorpion_pmu_get_event_idx(struct pmu_hw_events *cpuc,
    struct perf_event *event)
    {
    int idx;
    let mut bit: c_int = -1;
    struct hw_perf_event *hwc = &event.hw;
    let mut region: c_uint = EVENT_REGION(hwc.config_base);
    let mut group: c_uint = EVENT_GROUP(hwc.config_base);
    let mut venum_event: bool = EVENT_VENUM(hwc.config_base);
    let mut scorpion_event: bool = EVENT_CPU(hwc.config_base);
    if (venum_event || scorpion_event) {
// Ignore invalid events
    if (group > 3 || region > 3)
    return -EINVAL;
    bit = scorpion_event_to_bit(event, region, group);
    if (test_and_set_bit(bit, cpuc.used_mask))
    return -EAGAIN;
    }
    idx = armv7pmu_get_event_idx(cpuc, event);
    if (idx < 0 && bit >= 0)
    clear_bit(bit, cpuc.used_mask);
    return idx;
    }
    static void scorpion_pmu_clear_event_idx(struct pmu_hw_events *cpuc,
    struct perf_event *event)
    {
    int bit;
    struct hw_perf_event *hwc = &event.hw;
    let mut region: c_uint = EVENT_REGION(hwc.config_base);
    let mut group: c_uint = EVENT_GROUP(hwc.config_base);
    let mut venum_event: bool = EVENT_VENUM(hwc.config_base);
    let mut scorpion_event: bool = EVENT_CPU(hwc.config_base);
    armv7pmu_clear_event_idx(cpuc, event);
    if (venum_event || scorpion_event) {
    bit = scorpion_event_to_bit(event, region, group);
    clear_bit(bit, cpuc.used_mask);
    }
    }
#[no_mangle]
unsafe extern "C" fn scorpion_pmu_init(cpu_pmu: *mut arm_pmu) -> c_int {
    static int scorpion_pmu_init(struct arm_pmu *cpu_pmu)
    {
    armv7pmu_init(cpu_pmu);
    cpu_pmu.name		= "armv7_scorpion";
    cpu_pmu.map_event	= scorpion_map_event;
    cpu_pmu.reset		= scorpion_pmu_reset;
    cpu_pmu.enable		= scorpion_pmu_enable_event;
    cpu_pmu.disable	= scorpion_pmu_disable_event;
    cpu_pmu.get_event_idx	= scorpion_pmu_get_event_idx;
    cpu_pmu.clear_event_idx = scorpion_pmu_clear_event_idx;
    return armv7_probe_num_events(cpu_pmu);
    }
#[no_mangle]
unsafe extern "C" fn scorpion_mp_pmu_init(cpu_pmu: *mut arm_pmu) -> c_int {
    static int scorpion_mp_pmu_init(struct arm_pmu *cpu_pmu)
    {
    armv7pmu_init(cpu_pmu);
    cpu_pmu.name		= "armv7_scorpion_mp";
    cpu_pmu.map_event	= scorpion_map_event;
    cpu_pmu.reset		= scorpion_pmu_reset;
    cpu_pmu.enable		= scorpion_pmu_enable_event;
    cpu_pmu.disable	= scorpion_pmu_disable_event;
    cpu_pmu.get_event_idx	= scorpion_pmu_get_event_idx;
    cpu_pmu.clear_event_idx = scorpion_pmu_clear_event_idx;
    return armv7_probe_num_events(cpu_pmu);
    }
    static const struct of_device_id armv7_pmu_of_device_ids[] = {
    {.compatible = "arm,cortex-a17-pmu",	.data = armv7_a17_pmu_init},
    {.compatible = "arm,cortex-a15-pmu",	.data = armv7_a15_pmu_init},
    {.compatible = "arm,cortex-a12-pmu",	.data = armv7_a12_pmu_init},
    {.compatible = "arm,cortex-a9-pmu",	.data = armv7_a9_pmu_init},
    {.compatible = "arm,cortex-a8-pmu",	.data = armv7_a8_pmu_init},
    {.compatible = "arm,cortex-a7-pmu",	.data = armv7_a7_pmu_init},
    {.compatible = "arm,cortex-a5-pmu",	.data = armv7_a5_pmu_init},
    {.compatible = "qcom,krait-pmu",	.data = krait_pmu_init},
    {.compatible = "qcom,scorpion-pmu",	.data = scorpion_pmu_init},
    {.compatible = "qcom,scorpion-mp-pmu",	.data = scorpion_mp_pmu_init},
    {},
    };
#[no_mangle]
unsafe extern "C" fn armv7_pmu_device_probe(pdev: *mut platform_device) -> c_int {
    static int armv7_pmu_device_probe(struct platform_device *pdev)
    {
    return arm_pmu_device_probe(pdev, armv7_pmu_of_device_ids, core::ptr::null_mut());
    }
    static struct platform_driver armv7_pmu_driver = {
    .driver		= {
    .name	= "armv7-pmu",
    .of_match_table = armv7_pmu_of_device_ids,
    .suppress_bind_attrs = true,
    },
    .probe		= armv7_pmu_device_probe,
    };
    builtin_platform_driver(armv7_pmu_driver);
