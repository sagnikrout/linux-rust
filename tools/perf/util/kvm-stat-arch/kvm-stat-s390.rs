//! Automatically rewritten from C to Rust
//! Source: tools/perf/util/kvm-stat-arch/kvm-stat-s390.c
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
// Arch specific functions for perf kvm stat.
//
// Copyright 2014 IBM Corp.
// Author(s): Alexander Yarygin <yarygin@linux.vnet.ibm.com>
//

    define_exit_reasons_table(sie_exit_reasons, sie_intercept_code);
    define_exit_reasons_table(sie_icpt_insn_codes, icpt_insn_codes);
    define_exit_reasons_table(sie_sigp_order_codes, sigp_order_codes);
    define_exit_reasons_table(sie_diagnose_codes, diagnose_codes);
    define_exit_reasons_table(sie_icpt_prog_codes, icpt_prog_codes);
    static void event_icpt_insn_get_key(struct perf_sample *sample,
    struct event_key *key)
    {
    u64 insn;
    insn = perf_sample__intval(sample, "instruction");
    key.key = icpt_insn_decoder(insn);
    key.exit_reasons = sie_icpt_insn_codes;
    }
    static void event_sigp_get_key(struct perf_sample *sample,
    struct event_key *key)
    {
    key.key = perf_sample__intval(sample, "order_code");
    key.exit_reasons = sie_sigp_order_codes;
    }
    static void event_diag_get_key(struct perf_sample *sample,
    struct event_key *key)
    {
    key.key = perf_sample__intval(sample, "code");
    key.exit_reasons = sie_diagnose_codes;
    }
    static void event_icpt_prog_get_key(struct perf_sample *sample,
    struct event_key *key)
    {
    key.key = perf_sample__intval(sample, "code");
    key.exit_reasons = sie_icpt_prog_codes;
    }
    static const struct child_event_ops child_events[] = {
    { .name = "kvm:kvm_s390_intercept_instruction",
    .get_key = event_icpt_insn_get_key },
    { .name = "kvm:kvm_s390_handle_sigp",
    .get_key = event_sigp_get_key },
    { .name = "kvm:kvm_s390_handle_diag",
    .get_key = event_diag_get_key },
    { .name = "kvm:kvm_s390_intercept_prog",
    .get_key = event_icpt_prog_get_key },
    { core::ptr::null_mut(), core::ptr::null_mut() },
    };
    static const struct kvm_events_ops exit_events = {
    .is_begin_event = exit_event_begin,
    .is_end_event = exit_event_end,
    .child_ops = child_events,
    .decode_key = exit_event_decode_key,
    .name = "VM-EXIT"
    };
    static const char * const __kvm_events_tp[] = {
    "kvm:kvm_s390_sie_enter",
    "kvm:kvm_s390_sie_exit",
    "kvm:kvm_s390_intercept_instruction",
    "kvm:kvm_s390_handle_sigp",
    "kvm:kvm_s390_handle_diag",
    "kvm:kvm_s390_intercept_prog",
    core::ptr::null_mut(),
    };
    static const struct kvm_reg_events_ops __kvm_reg_events_ops[] = {
    { .name = "vmexit", .ops = &exit_events },
    { core::ptr::null_mut(), core::ptr::null_mut() },
    };
    static const char * const __kvm_skip_events[] = {
    "Wait state",
    core::ptr::null_mut(),
    };
#[no_mangle]
pub unsafe extern "C" fn __cpu_isa_init_s390(kvm: *mut perf_kvm_stat, cpuid: *const c_char) -> c_int {
    int __cpu_isa_init_s390(struct perf_kvm_stat *kvm, const char *cpuid)
    {
    if (strstr(cpuid, "IBM")) {
    kvm.exit_reasons = sie_exit_reasons;
    kvm.exit_reasons_isa = "SIE";
    } else
    return -ENOTSUP;
    return 0;
    }
    const char * const *__kvm_events_tp_s390(void)
    {
    return __kvm_events_tp;
    }
    const struct kvm_reg_events_ops *__kvm_reg_events_ops_s390(void)
    {
    return __kvm_reg_events_ops;
    }
    const char * const *__kvm_skip_events_s390(void)
    {
    return __kvm_skip_events;
    }
