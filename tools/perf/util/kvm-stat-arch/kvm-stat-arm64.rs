//! Automatically rewritten from C to Rust
//! Source: tools/perf/util/kvm-stat-arch/kvm-stat-arm64.c
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

    define_exit_reasons_table(arm64_exit_reasons, kvm_arm_exception_type);
    define_exit_reasons_table(arm64_trap_exit_reasons, kvm_arm_exception_class);
    static const char *kvm_trap_exit_reason = "esr_ec";
    static const char * const __kvm_events_tp[] = {
    "kvm:kvm_entry",
    "kvm:kvm_exit",
    core::ptr::null_mut(),
    };
    static void event_get_key(struct perf_sample *sample,
    struct event_key *key)
    {
    key.info = 0;
    key.key = perf_sample__intval(sample, kvm_exit_reason(EM_AARCH64));
    key.exit_reasons = arm64_exit_reasons;
//
// TRAP exceptions carry exception class info in esr_ec field
// and, hence, we need to use a different exit_reasons table to
// properly decode event's est_ec.
//
    if (key.key == ARM_EXCEPTION_TRAP) {
    key.key = perf_sample__intval(sample, kvm_trap_exit_reason);
    key.exit_reasons = arm64_trap_exit_reasons;
    }
    }
    static bool event_begin(struct perf_sample *sample,
    struct event_key *key __maybe_unused)
    {
    return evsel__name_is(sample.evsel, kvm_entry_trace(EM_AARCH64));
    }
    static bool event_end(struct perf_sample *sample,
    struct event_key *key)
    {
    if (evsel__name_is(sample.evsel, kvm_exit_trace(EM_AARCH64))) {
    event_get_key(sample, key);
    return true;
    }
    return false;
    }
    static const struct kvm_events_ops exit_events = {
    .is_begin_event = event_begin,
    .is_end_event	= event_end,
    .decode_key	= exit_event_decode_key,
    .name		= "VM-EXIT"
    };
    static const struct kvm_reg_events_ops __kvm_reg_events_ops[] = {
    {
    .name	= "vmexit",
    .ops	= &exit_events,
    },
    { core::ptr::null_mut(), core::ptr::null_mut() },
    };
    static const char * const __kvm_skip_events[] = {
    core::ptr::null_mut(),
    };
#[no_mangle]
pub unsafe extern "C" fn __cpu_isa_init_arm64(kvm: *mut perf_kvm_stat) -> c_int {
    int __cpu_isa_init_arm64(struct perf_kvm_stat *kvm)
    {
    kvm.exit_reasons_isa = "arm64";
    return 0;
    }
    const char * const *__kvm_events_tp_arm64(void)
    {
    return __kvm_events_tp;
    }
    const struct kvm_reg_events_ops *__kvm_reg_events_ops_arm64(void)
    {
    return __kvm_reg_events_ops;
    }
    const char * const *__kvm_skip_events_arm64(void)
    {
    return __kvm_skip_events;
    }
