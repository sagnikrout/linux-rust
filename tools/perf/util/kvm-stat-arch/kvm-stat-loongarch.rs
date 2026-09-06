//! Automatically rewritten from C to Rust
//! Source: tools/perf/util/kvm-stat-arch/kvm-stat-loongarch.c
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

pub const LOONGARCH_EXCEPTION_INT: c_int = 0;
pub const LOONGARCH_EXCEPTION_PIL: c_int = 1;
pub const LOONGARCH_EXCEPTION_PIS: c_int = 2;
pub const LOONGARCH_EXCEPTION_PIF: c_int = 3;
pub const LOONGARCH_EXCEPTION_PME: c_int = 4;
pub const LOONGARCH_EXCEPTION_FPD: c_int = 15;
pub const LOONGARCH_EXCEPTION_SXD: c_int = 16;
pub const LOONGARCH_EXCEPTION_ASXD: c_int = 17;
pub const LOONGARCH_EXCEPTION_GSPR: c_int = 22;
pub const LOONGARCH_EXCEPTION_CPUCFG: c_int = 100;
pub const LOONGARCH_EXCEPTION_CSR: c_int = 101;
pub const LOONGARCH_EXCEPTION_IOCSR: c_int = 102;
pub const LOONGARCH_EXCEPTION_IDLE: c_int = 103;
pub const LOONGARCH_EXCEPTION_OTHERS: c_int = 104;
pub const LOONGARCH_EXCEPTION_HVC: c_int = 23;

    {LOONGARCH_EXCEPTION_INT,  "Interrupt" },		\
    {LOONGARCH_EXCEPTION_PIL,  "Mem Read" },		\
    {LOONGARCH_EXCEPTION_PIS,  "Mem Store" },		\
    {LOONGARCH_EXCEPTION_PIF,  "Inst Fetch" },		\
    {LOONGARCH_EXCEPTION_PME,  "Mem Modify" },		\
    {LOONGARCH_EXCEPTION_FPD,  "FPU" },			\
    {LOONGARCH_EXCEPTION_SXD,  "LSX" },			\
    {LOONGARCH_EXCEPTION_ASXD, "LASX" },			\
    {LOONGARCH_EXCEPTION_GSPR, "Privilege Error" },		\
    {LOONGARCH_EXCEPTION_HVC,  "Hypercall" },		\
    {LOONGARCH_EXCEPTION_CPUCFG, "CPUCFG" },		\
    {LOONGARCH_EXCEPTION_CSR,    "CSR" },			\
    {LOONGARCH_EXCEPTION_IOCSR,  "IOCSR" },			\
    {LOONGARCH_EXCEPTION_IDLE,   "Idle" },			\
    {LOONGARCH_EXCEPTION_OTHERS, "Others" }
    define_exit_reasons_table(loongarch_exit_reasons, loongarch_exception_type);
    static const char *kvm_reenter_trace = "kvm:kvm_reenter";
    static const char * const __kvm_events_tp[] = {
    "kvm:kvm_enter",
    "kvm:kvm_reenter",
    "kvm:kvm_exit",
    "kvm:kvm_exit_gspr",
    core::ptr::null_mut(),
    };
#[no_mangle]
unsafe extern "C" fn event_begin(sample: *mut perf_sample, key: *mut event_key) -> bool {
    static bool event_begin(struct perf_sample *sample, struct event_key *key)
    {
    return exit_event_begin(sample, key);
    }
    static bool event_end(struct perf_sample *sample,
    struct event_key *key __maybe_unused)
    {
//
// LoongArch kvm is different with other architectures
//
// There is kvm:kvm_reenter or kvm:kvm_enter event adjacent with
// kvm:kvm_exit event.
// kvm:kvm_enter   means returning to vmm and then to guest
// kvm:kvm_reenter means returning to guest immediately
//
    return evsel__name_is(sample.evsel, kvm_entry_trace(EM_LOONGARCH)) ||
    evsel__name_is(sample.evsel, kvm_reenter_trace);
    }
#[no_mangle]
unsafe extern "C" fn event_gspr_get_key(sample: *mut perf_sample, key: *mut event_key) {
    static void event_gspr_get_key(struct perf_sample *sample, struct event_key *key)
    {
    unsigned int insn;
    key.key = LOONGARCH_EXCEPTION_OTHERS;
    insn = perf_sample__intval(sample, "inst_word");
    switch (insn >> 24) {
    case 0:
// CPUCFG inst trap
    if ((insn >> 10) == 0x1b)
    key.key = LOONGARCH_EXCEPTION_CPUCFG;
    break;
    case 4:
// CSR inst trap
    key.key = LOONGARCH_EXCEPTION_CSR;
    break;
    case 6:
// IOCSR inst trap
    if ((insn >> 15) == 0xc90)
    key.key = LOONGARCH_EXCEPTION_IOCSR;
#[no_mangle]
pub unsafe extern "C" fn if(0xc91: (insn >> 15) ==) -> else {
    else if ((insn >> 15) == 0xc91)
// Idle inst trap
    key.key = LOONGARCH_EXCEPTION_IDLE;
    break;
    default:
    key.key = LOONGARCH_EXCEPTION_OTHERS;
    break;
    }
    }
    static const struct child_event_ops child_events[] = {
    { .name = "kvm:kvm_exit_gspr", .get_key = event_gspr_get_key },
    { core::ptr::null_mut(), core::ptr::null_mut() },
    };
    static const struct kvm_events_ops exit_events = {
    .is_begin_event = event_begin,
    .is_end_event = event_end,
    .child_ops = child_events,
    .decode_key = exit_event_decode_key,
    .name = "VM-EXIT"
    };
    static const struct kvm_reg_events_ops __kvm_reg_events_ops[] = {
    { .name	= "vmexit", .ops = &exit_events, },
    { core::ptr::null_mut(), core::ptr::null_mut() },
    };
    static const char * const __kvm_skip_events[] = {
    core::ptr::null_mut(),
    };
#[no_mangle]
pub unsafe extern "C" fn __cpu_isa_init_loongarch(kvm: *mut perf_kvm_stat) -> c_int {
    int __cpu_isa_init_loongarch(struct perf_kvm_stat *kvm)
    {
    kvm.exit_reasons_isa = "loongarch64";
    kvm.exit_reasons = loongarch_exit_reasons;
    return 0;
    }
    const char * const *__kvm_events_tp_loongarch(void)
    {
    return __kvm_events_tp;
    }
    const struct kvm_reg_events_ops *__kvm_reg_events_ops_loongarch(void)
    {
    return __kvm_reg_events_ops;
    }
    const char * const *__kvm_skip_events_loongarch(void)
    {
    return __kvm_skip_events;
    }
