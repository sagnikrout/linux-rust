//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/perf_event.h
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
// Performance event hw details:
//
pub const INTEL_PMC_MAX_GENERIC: c_int = 32;
pub const INTEL_PMC_MAX_FIXED: c_int = 16;
pub const INTEL_PMC_IDX_FIXED: c_int = 32;
pub const X86_PMC_IDX_MAX: c_int = 64;
pub const MSR_ARCH_PERFMON_PERFCTR0: c_uint = 0xc1;
pub const MSR_ARCH_PERFMON_PERFCTR1: c_uint = 0xc2;
pub const MSR_ARCH_PERFMON_EVENTSEL0: c_uint = 0x186;
pub const MSR_ARCH_PERFMON_EVENTSEL1: c_uint = 0x187;
pub const ARCH_PERFMON_EVENTSEL_EVENT: c_uint = 0x000000FFULL;
pub const ARCH_PERFMON_EVENTSEL_UMASK: c_uint = 0x0000FF00ULL;

pub const ARCH_PERFMON_EVENTSEL_CMASK: c_uint = 0xFF000000ULL;

pub const INTEL_FIXED_BITS_STRIDE: c_int = 4;

pub const AMD64_EVENTSEL_INT_CORE_SEL_SHIFT: c_int = 37;

pub const AMD64_L3_SLICE_SHIFT: c_int = 48;

pub const AMD64_L3_THREAD_SHIFT: c_int = 56;

pub const AMD64_L3_COREID_SHIFT: c_int = 42;

pub const AMD64_NUM_COUNTERS: c_int = 4;
pub const AMD64_NUM_COUNTERS_CORE: c_int = 6;
pub const AMD64_NUM_COUNTERS_NB: c_int = 4;
pub const ARCH_PERFMON_UNHALTED_CORE_CYCLES_SEL: c_uint = 0x3c;

pub const ARCH_PERFMON_UNHALTED_CORE_CYCLES_INDEX: c_int = 0;

pub const ARCH_PERFMON_BRANCH_MISSES_RETIRED: c_int = 6;
pub const ARCH_PERFMON_EVENTS_COUNT: c_int = 7;

pub const PEBS_DATACFG_LBR_SHIFT: c_int = 24;
pub const PEBS_DATACFG_CNTR_SHIFT: c_int = 32;

pub const PEBS_DATACFG_FIX_SHIFT: c_int = 48;

// Steal the highest bit of pebs_data_cfg for SW usage

//
// Intel "Architectural Performance Monitoring" CPUID
// detection/enumeration details:
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union cpuid10_eax {
    pub version_id:8: c_uint,
    pub num_counters:8: c_uint,
    pub bit_width:8: c_uint,
    pub mask_length:8: c_uint,
    pub split: },
    pub full: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cpuid10_ebx {
    pub no_unhalted_core_cycles:1: c_uint,
    pub no_instructions_retired:1: c_uint,
    pub no_unhalted_reference_cycles:1: c_uint,
    pub no_llc_reference:1: c_uint,
    pub no_llc_misses:1: c_uint,
    pub no_branch_instruction_retired:1: c_uint,
    pub no_branch_misses_retired:1: c_uint,
    pub split: },
    pub full: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cpuid10_edx {
    pub num_counters_fixed:5: c_uint,
    pub bit_width_fixed:8: c_uint,
    pub reserved1:2: c_uint,
    pub anythread_deprecated:1: c_uint,
    pub reserved2:16: c_uint,
    pub split: },
    pub full: c_uint,
}

//
// Intel "Architectural Performance Monitoring extension" CPUID
// detection/enumeration details:
//
pub const ARCH_PERFMON_EXT_LEAF: c_uint = 0x00000023;
pub const ARCH_PERFMON_NUM_COUNTER_LEAF: c_uint = 0x1;
pub const ARCH_PERFMON_ACR_LEAF: c_uint = 0x2;
pub const ARCH_PERFMON_PEBS_CAP_LEAF: c_uint = 0x4;
pub const ARCH_PERFMON_PEBS_COUNTER_LEAF: c_uint = 0x5;
#[repr(C)]
#[derive(Copy, Clone)]
pub union cpuid35_eax {
    pub leaf0:1: c_uint,
// Counters Sub-Leaf
    pub cntr_subleaf:1: c_uint,
// Auto Counter Reload Sub-Leaf
    pub acr_subleaf:1: c_uint,
// Events Sub-Leaf
    pub events_subleaf:1: c_uint,
// arch-PEBS Sub-Leaves
    pub pebs_caps_subleaf:1: c_uint,
    pub pebs_cnts_subleaf:1: c_uint,
    pub reserved:26: c_uint,
    pub split: },
    pub full: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cpuid35_ebx {
// UnitMask2 Supported
    pub umask2:1: c_uint,
// EQ-bit Supported
    pub eq:1: c_uint,
// rdpmc user disable Supported
    pub rdpmc_user_disable:1: c_uint,
    pub reserved:29: c_uint,
    pub split: },
    pub full: c_uint,
}

//
// Intel Architectural LBR CPUID detection/enumeration details:
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union cpuid28_eax {
// Supported LBR depth values
    pub lbr_depth_mask:8: c_uint,
    pub reserved:22: c_uint,
// Deep C-state Reset
    pub lbr_deep_c_reset:1: c_uint,
// IP values contain LIP
    pub lbr_lip:1: c_uint,
    pub split: },
    pub full: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cpuid28_ebx {
// CPL Filtering Supported
    pub lbr_cpl:1: c_uint,
// Branch Filtering Supported
    pub lbr_filter:1: c_uint,
// Call-stack Mode Supported
    pub lbr_call_stack:1: c_uint,
    pub split: },
    pub full: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cpuid28_ecx {
// Mispredict Bit Supported
    pub lbr_mispred:1: c_uint,
// Timed LBRs Supported
    pub lbr_timed_lbr:1: c_uint,
// Branch Type Field Supported
    pub lbr_br_type:1: c_uint,
    pub reserved:13: c_uint,
// Branch counters (Event Logging) Supported
    pub lbr_counters:4: c_uint,
    pub split: },
    pub full: c_uint,
}

//
// AMD "Extended Performance Monitoring and Debug" CPUID
// detection/enumeration details:
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union cpuid_0x80000022_ebx {
// Number of Core Performance Counters
    pub num_core_pmc:4: c_uint,
// Number of available LBR Stack Entries
    pub lbr_v2_stack_sz:6: c_uint,
// Number of Data Fabric Counters
    pub num_df_pmc:6: c_uint,
// Number of Unified Memory Controller Counters
    pub num_umc_pmc:6: c_uint,
    pub split: },
    pub full: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct x86_pmu_capability {
    pub version: c_int,
    pub num_counters_gp: c_int,
    pub num_counters_fixed: c_int,
    pub bit_width_gp: c_int,
    pub bit_width_fixed: c_int,
    pub events_mask: c_uint,
    pub events_mask_len: c_int,
    pub :1: unsigned int pebs_ept,
    pub :1: unsigned int mediated,
}

//
// Fixed-purpose performance events:
//
// RDPMC offset for Fixed PMCs

//
// All the fixed-mode PMCs are configured via this single MSR:
//
pub const MSR_ARCH_PERFMON_FIXED_CTR_CTRL: c_uint = 0x38d;
//
// There is no event-code assigned to the fixed-mode PMCs.
//
// For a fixed-mode PMC, which has an equivalent event on a general-purpose
// PMC, the event-code of the equivalent event is used for the fixed-mode PMC,
// e.g., Instr_Retired.Any and CPU_CLK_Unhalted.Core.
//
// For a fixed-mode PMC, which doesn't have an equivalent event, a
// pseudo-encoding is used, e.g., CPU_CLK_Unhalted.Ref and TOPDOWN.SLOTS.
// The pseudo event-code for a fixed-mode PMC must be 0x00.
// The pseudo umask-code is 0xX. The X equals the index of the fixed
// counter + 1, e.g., the fixed counter 2 has the pseudo-encoding 0x0300.
//
// The counts are available in separate MSRs:
//
// Instr_Retired.Any:
pub const MSR_ARCH_PERFMON_FIXED_CTR0: c_uint = 0x309;

// CPU_CLK_Unhalted.Core:
pub const MSR_ARCH_PERFMON_FIXED_CTR1: c_uint = 0x30a;

// CPU_CLK_Unhalted.Ref: event=0x00,umask=0x3 (pseudo-encoding)
pub const MSR_ARCH_PERFMON_FIXED_CTR2: c_uint = 0x30b;

// TOPDOWN.SLOTS: event=0x00,umask=0x4 (pseudo-encoding)
pub const MSR_ARCH_PERFMON_FIXED_CTR3: c_uint = 0x30c;

// TOPDOWN_BAD_SPECULATION.ALL: fixed counter 4 (Atom only)
// TOPDOWN_FE_BOUND.ALL: fixed counter 5 (Atom only)
// TOPDOWN_RETIRING.ALL: fixed counter 6 (Atom only)
//
// We model BTS tracing as another fixed-mode PMC.
//
// We choose the value 47 for the fixed index of BTS, since lower
// values are used by actual fixed events and higher values are used
// to indicate other overflow conditions in the PERF_GLOBAL_STATUS msr.
//

//
// The PERF_METRICS MSR is modeled as several magic fixed-mode PMCs, one for
// each TopDown metric event.
//
// Internally the TopDown metric events are mapped to the FxCtr 3 (SLOTS).
//

//
// There is no event-code assigned to the TopDown events.
//
// For the slots event, use the pseudo code of the fixed counter 3.
//
// For the metric events, the pseudo event-code is 0x00.
// The pseudo umask-code starts from the middle of the pseudo event
// space, 0x80.
//
pub const INTEL_TD_SLOTS: c_uint = 0x0400	/* TOPDOWN.SLOTS */;
// Level 1 metrics
pub const INTEL_TD_METRIC_RETIRING: c_uint = 0x8000	/* Retiring metric */;
pub const INTEL_TD_METRIC_BAD_SPEC: c_uint = 0x8100	/* Bad speculation metric */;
pub const INTEL_TD_METRIC_FE_BOUND: c_uint = 0x8200	/* FE bound metric */;
pub const INTEL_TD_METRIC_BE_BOUND: c_uint = 0x8300	/* BE bound metric */;
// Level 2 metrics
pub const INTEL_TD_METRIC_HEAVY_OPS: c_uint = 0x8400  /* Heavy Operations metric */;
pub const INTEL_TD_METRIC_BR_MISPREDICT: c_uint = 0x8500  /* Branch Mispredict metric */;
pub const INTEL_TD_METRIC_FETCH_LAT: c_uint = 0x8600  /* Fetch Latency metric */;
pub const INTEL_TD_METRIC_MEM_BOUND: c_uint = 0x8700  /* Memory bound metric */;

pub const INTEL_TD_METRIC_NUM: c_int = 8;
pub const INTEL_TD_CFG_METRIC_CLEAR_BIT: c_int = 0;

pub const GLOBAL_STATUS_BUFFER_OVF_BIT: c_int = 62;

pub const GLOBAL_STATUS_LBRS_FROZEN_BIT: c_int = 58;

pub const GLOBAL_STATUS_TRACE_TOPAPMI_BIT: c_int = 55;

pub const GLOBAL_STATUS_ARCH_PEBS_THRESHOLD_BIT: c_int = 54;

pub const GLOBAL_STATUS_PERF_METRICS_OVF_BIT: c_int = 48;

//
// We model guest LBR event tracing as another fixed-mode PMC like BTS.
//
// We choose bit 58 because it's used to indicate LBR stack frozen state
// for architectural perfmon v4, also we unconditionally mask that bit in
// the handle_pmi_common(), so it'll never be set in the overflow handling.
//
// With this fake counter assigned, the guest LBR event user (such as KVM),
// can program the LBR registers on its own, and we don't actually do anything
// with then in the host context.
//

//
// Pseudo-encoding the guest LBR event as event=0x00,umask=0x1b,
// since it would claim bit 58 which is effectively Fixed26.
//
pub const INTEL_FIXED_VLBR_EVENT: c_uint = 0x1b00;
//
// Adaptive PEBS v4
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pebs_basic {
    pub ip: u64,
    pub applicable_counters: u64,
    pub tsc: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pebs_meminfo {
    pub address: u64,
    pub aux: u64,
// pre Alder Lake
    pub mem_latency: u64,
// Alder Lake and later
    pub instr_latency:16: u64,
    pub pad2:16: u64,
    pub cache_latency:16: u64,
    pub pad3:16: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pebs_gprs {
    pub di: u64 flags, ip, ax, cx, dx, bx, sp, bp, si,,
    pub r15: u64 r8, r9, r10, r11, r12, r13, r14,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pebs_xmm {
    pub /: *mut *mut *mut u64 xmm[162]; / two entries for each register,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pebs_cntr_header {
    pub cntr: u32,
    pub fixed: u32,
    pub metrics: u32,
    pub reserved: u32,
}

pub const INTEL_CNTR_METRICS: c_uint = 0x3;
//
// Arch PEBS
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union arch_pebs_index {
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arch_pebs_header {
    pub format: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arch_pebs_basic {
    pub ip: u64,
    pub applicable_counters: u64,
    pub tsc: u64,
    pub :47: rsvd,
    pub rsvd2: u64,
    pub rsvd3: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arch_pebs_aux {
    pub address: u64,
    pub rsvd: u64,
    pub rsvd2: u64,
    pub rsvd3: u64,
    pub rsvd4: u64,
    pub aux: u64,
    pub :16: pad3,
    pub tsx_tuning: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arch_pebs_gprs {
    pub di: u64 flags, ip, ax, cx, dx, bx, sp, bp, si,,
    pub ssp: u64 r8, r9, r10, r11, r12, r13, r14, r15,,
    pub rsvd: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arch_pebs_xer_header {
    pub xstate: u64,
    pub rsvd: u64,
}

pub const ARCH_PEBS_LBR_NAN: c_uint = 0x0;
pub const ARCH_PEBS_LBR_NUM_8: c_uint = 0x1;
pub const ARCH_PEBS_LBR_NUM_16: c_uint = 0x2;
pub const ARCH_PEBS_LBR_NUM_VAR: c_uint = 0x3;
pub const ARCH_PEBS_BASE_LBR_ENTRIES: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arch_pebs_lbr_header {
    pub rsvd: u64,
    pub ctl: u64,
    pub depth: u64,
    pub ler_from: u64,
    pub ler_to: u64,
    pub ler_info: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arch_pebs_cntr_header {
    pub cntr: u32,
    pub fixed: u32,
    pub metrics: u32,
    pub reserved: u32,
}

//
// AMD Extended Performance Monitoring and Debug cpuid feature detection
//
pub const EXT_PERFMON_DEBUG_FEATURES: c_uint = 0x80000022;
//
// IBS cpuid feature detection
//
pub const IBS_CPUID_FEATURES: c_uint = 0x8000001b;
//
// Same bit mask as for IBS cpuid feature flags (Fn8000_001B_EAX), but
// bit 0 is used to indicate the existence of IBS.
//

//
// IBS APIC setup
//
pub const IBSCTL: c_uint = 0x1cc;

pub const IBSCTL_LVT_OFFSET_MASK: c_uint = 0x0F;
// IBS fetch bits/masks

pub const IBS_FETCH_CNT: c_uint = 0xFFFF0000ULL;
pub const IBS_FETCH_MAX_CNT: c_uint = 0x0000FFFFULL;

//
// IBS op bits/masks
// The lower 7 bits of the current count are random bits
// preloaded by hardware and ignored in software
//

pub const IBS_OP_MAX_CNT: c_uint = 0x0000FFFFULL;
pub const IBS_OP_MAX_CNT_EXT: c_uint = 0x007FFFFFULL	/* not a register bit mask */;

extern "C" {
    pub fn get_ibs_caps() -> u32;
}
extern "C" {
    pub fn forward_event_to_ibs(event: *mut perf_event) -> c_int;
}

extern "C" {
    pub fn perf_events_lapic_init();
}
//
// Abuse bits {3,5} of the cpu eflags register. These flags are otherwise
// unused and ABI specified to be 0, so nobody should care what we do with
// them.
//
// EXACT - the IP points to the exact instruction that triggered the
// event (HW bugs exempt).
// VM    - original X86_VM_MASK; see set_linear_ip().
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct x86_perf_regs {
    pub regs: pt_regs,
    pub xmm_regs: *mut u64,
}

extern "C" {
    pub fn perf_arch_instruction_pointer(regs: *mut pt_regs) -> c_ulong;
}
extern "C" {
    pub fn perf_arch_misc_flags(regs: *mut pt_regs) -> c_ulong;
}
extern "C" {
    pub fn perf_arch_guest_misc_flags(regs: *mut pt_regs) -> c_ulong;
}

//
// We abuse bit 3 from flags to pass exact information, see
// perf_arch_misc_flags() and the comment with PERF_EFLAGS_EXACT.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_guest_switch_msr {
    pub msr: unsigned,
    pub guest: u64 host,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct x86_pmu_lbr {
    pub nr: c_uint,
    pub from: c_uint,
    pub to: c_uint,
    pub info: c_uint,
    pub has_callstack: bool,
}

extern "C" {
    pub fn perf_get_x86_pmu_capability(cap: *mut x86_pmu_capability);
}
extern "C" {
    pub fn perf_get_hw_event_config(hw_event: c_int) -> u64;
}
extern "C" {
    pub fn perf_check_microcode();
}
extern "C" {
    pub fn perf_clear_dirty_counters();
}
extern "C" {
    pub fn x86_perf_rdpmc_index(event: *mut perf_event) -> c_int;
}

extern "C" {
    pub fn perf_load_guest_lvtpc(guest_lvtpc: u32);
}
extern "C" {
    pub fn perf_put_guest_lvtpc();
}

extern "C" {
    pub fn x86_perf_get_lbr(lbr: *mut x86_pmu_lbr);
}

extern "C" {
    pub fn intel_pt_handle_vmx(on: c_int);
}

extern "C" {
    pub fn amd_pmu_enable_virt();
}
extern "C" {
    pub fn amd_pmu_disable_virt();
}

pub const PERF_NEEDS_LOPWR_CB: c_int = 1;
//
// architectural low power callback impacts
// drivers/acpi/processor_idle.c
// drivers/acpi/acpi_pad.c
//
extern "C" {
    pub fn perf_amd_brs_lopwr_cb(lopwr_in: bool);
}

