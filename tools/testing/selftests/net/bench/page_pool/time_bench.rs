//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/net/bench/page_pool/time_bench.h
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
//
// Benchmarking code execution time inside the kernel
//
// Copyright (C) 2014, Red Hat, Inc., Jesper Dangaard Brouer
// for licensing details see kernel-base/COPYING
//
// Main structure used for recording a benchmark run
#[repr(C)]
#[derive(Copy, Clone)]
pub struct time_bench_record {
    pub version_abi: u32,
    pub /: *mut *mut uint32_t loops; / Requested loop invocations,
    pub /: *mut *mut uint32_t step; / option for e.g. bulk invocations,
    pub /: *mut *mut uint32_t flags; / Measurements types enabled,

    pub /: *mut *mut uint32_t cpu; / Used when embedded in time_bench_cpu,
// Records
    pub /: *mut *mut uint64_t invoked_cnt; / Returned actual invocations,
    pub tsc_start: u64,
    pub tsc_stop: u64,
    pub ts_start: timespec64,
    pub ts_stop: timespec64,
// PMU counters for instruction and cycles
// instructions counter including pipelined instructions
//
    pub pmc_inst_start: u64,
    pub pmc_inst_stop: u64,
// CPU unhalted clock counter
    pub pmc_clk_start: u64,
    pub pmc_clk_stop: u64,
// Result records
    pub tsc_interval: u64,
    pub /: *mut *mut uint64_t time_start, time_stop, time_interval; / in nanosec,
    pub pmc_clk: uint64_t pmc_inst,,
// Derived result records
    pub +decimal?: uint64_t tsc_cycles; //,
    pub ns_per_call_decimal: uint64_t ns_per_call_quotient,,
    pub time_sec: u64,
    pub time_sec_remainder: u32,
    pub /: *mut *mut uint64_t pmc_ipc_quotient, pmc_ipc_decimal; / inst per cycle,
}

// For synchronizing parallel CPUs to run concurrently
#[repr(C)]
#[derive(Copy, Clone)]
pub struct time_bench_sync {
    pub nr_tests_running: core::sync::atomic::AtomicI32,
    pub start_event: completion,
}

// Keep track of CPUs executing our bench function.
//
// Embed a time_bench_record for storing info per cpu
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct time_bench_cpu {
    pub rec: time_bench_record,
    pub /: *mut *mut *mut time_bench_sync sync; / back ptr,
    pub task: *mut task_struct,
// "data" opaque could have been placed in time_bench_sync,
// but to avoid any false sharing, place it per CPU
//
    pub data: *mut c_void,
// Support masking outsome CPUs, mark if it ran
    pub did_bench_run: bool,
// int cpu; // note CPU stored in time_bench_record
    pub data): *mut *mut *mut int (bench_func)(struct time_bench_record record, void,
}

//
// Below TSC assembler code is not compatible with other archs, and
// can also fail on guests if cpu-flags are not correct.
//
// The way TSC reading is used, many iterations, does not require as
// high accuracy as described below (in Intel Doc #324264).
//
// Considering changing to use get_cycles() (#include <asm/timex.h>).
//
// TSC (Time-Stamp Counter) based
// Recommend reading, to understand details of reading TSC accurately:
// Intel Doc #324264, "How to Benchmark Code Execution Times on Intel"
//
// Consider getting exclusive ownership of CPU by using:
// unsigned long flags;
// preempt_disable();
// raw_local_irq_save(flags);
// _your_code_
// raw_local_irq_restore(flags);
// preempt_enable();
//
// Clobbered registers: "%rax", "%rbx", "%rcx", "%rdx"
// RDTSC only change "%rax" and "%rdx" but
// CPUID clears the high 32-bits of all (rax/rbx/rcx/rdx)
//
// See: Intel Doc #324264
// FIXME: on 32bit use clobbered %eax + %edx
// See: Intel Doc #324264
// Wall-clock based
//
// use: getnstimeofday()
// getnstimeofday(&rec->ts_start);
// getnstimeofday(&rec->ts_stop);
//
// API changed see: Documentation/core-api/timekeeping.rst
// https://www.kernel.org/doc/html/latest/core-api/timekeeping.html#c.getnstimeofday
//
// We should instead use: ktime_get_real_ts64() is a direct
// replacement, but consider using monotonic time (ktime_get_ts64())
// and/or a ktime_t based interface (ktime_get()/ktime_get_real()).
//
// PMU (Performance Monitor Unit) based
//
// Needed for calculating: Instructions Per Cycle (IPC)
// - The IPC number tell how efficient the CPU pipelining were
//
// lookup: perf_event_create_kernel_counter()
extern "C" {
    pub fn time_bench_PMU_config(enable: bool) -> bool;
}
// Raw reading via rdpmc() using fixed counters
//
// From: https://github.com/andikleen/simple-pmu
//
extern "C" {
    pub fn volatile("=d"(d): "rdpmc" :, "memory": "=a"(a) : "c"(in) :) -> asm;
}
// These PMU counter needs to be enabled, but I don't have the
// configure code implemented.  My current hack is running:
// sudo perf stat -e cycles:k -e instructions:k insmod lib/ring_queue_test.ko
//
// Reading all pipelined instruction
extern "C" {
    pub fn p_rdpmc(FIXED_INST_RETIRED_ANY: FIXED_SELECT |) -> return;
}
// Reading CPU clock cycles
extern "C" {
    pub fn p_rdpmc(FIXED_CPU_CLK_UNHALTED_CORE: FIXED_SELECT |) -> return;
}
// Raw reading via MSR rdmsr() is likely wrong
// FIXME: How can I know which raw MSR registers are conf for what?
//
pub const MSR_IA32_PCM0: c_uint = 0x400000C1 /* PERFCTR0 */;
pub const MSR_IA32_PCM1: c_uint = 0x400000C2 /* PERFCTR1 */;
pub const MSR_IA32_PCM2: c_uint = 0x400000C3;
extern "C" {
    pub fn rdmsrq_safe(_arg: MSR_IA32_PCM0, _arg: msr_result) -> return;
}
// Generic functions
//
extern "C" {
    pub fn time_bench_calc_stats(rec: *mut time_bench_record) -> bool;
}
// FIXME: use rec->flags to select measurement, should be MACRO
// getnstimeofday(&rec->ts_start);
// getnstimeofday(&rec->ts_stop);
