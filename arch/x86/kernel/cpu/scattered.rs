//! Automatically rewritten from C to Rust
//! Source: arch/x86/kernel/cpu/scattered.c
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


//
// Routines to identify additional cpu features that are scattered in
// cpuid space.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpuid_bit {
    pub feature: u16,
    pub reg: u8,
    pub bit: u8,
    pub level: u32,
    pub sub_leaf: u32,
}

//
// Please keep the leaf sorted by cpuid_bit.level for faster search.
// X86_FEATURE_MBA is supported by both Intel and AMD. But the CPUID
// levels are different and there is a separate entry for each.
//
    static const struct cpuid_bit cpuid_bits[] = {
    { X86_FEATURE_APERFMPERF,		CPUID_ECX,  0, 0x00000006, 0 },
    { X86_FEATURE_EPB,			CPUID_ECX,  3, 0x00000006, 0 },
    { X86_FEATURE_INTEL_PPIN,		CPUID_EBX,  0, 0x00000007, 1 },
    { X86_FEATURE_MSR_IMM,			CPUID_ECX,  5, 0x00000007, 1 },
    { X86_FEATURE_APX,			CPUID_EDX, 21, 0x00000007, 1 },
    { X86_FEATURE_RRSBA_CTRL,		CPUID_EDX,  2, 0x00000007, 2 },
    { X86_FEATURE_BHI_CTRL,			CPUID_EDX,  4, 0x00000007, 2 },
    { X86_FEATURE_CQM_LLC,			CPUID_EDX,  1, 0x0000000f, 0 },
    { X86_FEATURE_CQM_OCCUP_LLC,		CPUID_EDX,  0, 0x0000000f, 1 },
    { X86_FEATURE_CQM_MBM_TOTAL,		CPUID_EDX,  1, 0x0000000f, 1 },
    { X86_FEATURE_CQM_MBM_LOCAL,		CPUID_EDX,  2, 0x0000000f, 1 },
    { X86_FEATURE_CAT_L3,			CPUID_EBX,  1, 0x00000010, 0 },
    { X86_FEATURE_CAT_L2,			CPUID_EBX,  2, 0x00000010, 0 },
    { X86_FEATURE_MBA,			CPUID_EBX,  3, 0x00000010, 0 },
    { X86_FEATURE_CDP_L3,			CPUID_ECX,  2, 0x00000010, 1 },
    { X86_FEATURE_CDP_L2,			CPUID_ECX,  2, 0x00000010, 2 },
    { X86_FEATURE_PER_THREAD_MBA,		CPUID_ECX,  0, 0x00000010, 3 },
    { X86_FEATURE_SGX1,			CPUID_EAX,  0, 0x00000012, 0 },
    { X86_FEATURE_SGX2,			CPUID_EAX,  1, 0x00000012, 0 },
    { X86_FEATURE_SGX_EUPDATESVN,		CPUID_EAX, 10, 0x00000012, 0 },
    { X86_FEATURE_SGX_EDECCSSA,		CPUID_EAX, 11, 0x00000012, 0 },
    { X86_FEATURE_OVERFLOW_RECOV,		CPUID_EBX,  0, 0x80000007, 0 },
    { X86_FEATURE_SUCCOR,			CPUID_EBX,  1, 0x80000007, 0 },
    { X86_FEATURE_SMCA,			CPUID_EBX,  3, 0x80000007, 0 },
    { X86_FEATURE_HW_PSTATE,		CPUID_EDX,  7, 0x80000007, 0 },
    { X86_FEATURE_CPB,			CPUID_EDX,  9, 0x80000007, 0 },
    { X86_FEATURE_PROC_FEEDBACK,		CPUID_EDX, 11, 0x80000007, 0 },
    { X86_FEATURE_AMD_FAST_CPPC,		CPUID_EDX, 15, 0x80000007, 0 },
    { X86_FEATURE_CPPC_PERF_PRIO,		CPUID_EDX, 16, 0x80000007, 0 },
    { X86_FEATURE_MBA,			CPUID_EBX,  6, 0x80000008, 0 },
    { X86_FEATURE_X2AVIC_EXT,		CPUID_ECX,  6, 0x8000000a, 0 },
    { X86_FEATURE_COHERENCY_SFW_NO,		CPUID_EBX, 31, 0x8000001f, 0 },
    { X86_FEATURE_SMBA,			CPUID_EBX,  2, 0x80000020, 0 },
    { X86_FEATURE_BMEC,			CPUID_EBX,  3, 0x80000020, 0 },
    { X86_FEATURE_ABMC,			CPUID_EBX,  5, 0x80000020, 0 },
    { X86_FEATURE_SDCIAE,			CPUID_EBX,  6, 0x80000020, 0 },
    { X86_FEATURE_AMD_WORKLOAD_CLASS,	CPUID_EAX, 22, 0x80000021, 0 },
    { X86_FEATURE_TSA_SQ_NO,		CPUID_ECX,  1, 0x80000021, 0 },
    { X86_FEATURE_TSA_L1_NO,		CPUID_ECX,  2, 0x80000021, 0 },
    { X86_FEATURE_PERFMON_V2,		CPUID_EAX,  0, 0x80000022, 0 },
    { X86_FEATURE_AMD_LBR_V2,		CPUID_EAX,  1, 0x80000022, 0 },
    { X86_FEATURE_AMD_LBR_PMC_FREEZE,	CPUID_EAX,  2, 0x80000022, 0 },
    { X86_FEATURE_AMD_HTR_CORES,		CPUID_EAX, 30, 0x80000026, 0 },
    { 0, 0, 0, 0, 0 }
    };
#[no_mangle]
pub unsafe extern "C" fn init_scattered_cpuid_features(c: *mut cpuinfo_x86) {
    void init_scattered_cpuid_features(struct cpuinfo_x86 *c)
    {
    u32 max_level;
    u32 regs[4];
    const struct cpuid_bit *cb;
    for (cb = cpuid_bits; cb.feature; cb++) {
// Verify that the level is valid
    max_level = cpuid_eax(cb.level & 0xffff0000);
    if (max_level < cb.level ||
    max_level > (cb.level | 0xffff))
    continue;
    cpuid_count(cb.level, cb.sub_leaf, &regs[CPUID_EAX],
    &regs[CPUID_EBX], &regs[CPUID_ECX],
    &regs[CPUID_EDX]);
    if (regs[cb.reg] & (1 << cb.bit))
    set_cpu_cap(c, cb.feature);
    }
    }
