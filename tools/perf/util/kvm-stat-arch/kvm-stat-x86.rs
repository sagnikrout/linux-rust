//! Automatically rewritten from C to Rust
//! Source: tools/perf/util/kvm-stat-arch/kvm-stat-x86.c
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

    define_exit_reasons_table(vmx_exit_reasons, VMX_EXIT_REASONS);
    define_exit_reasons_table(svm_exit_reasons, SVM_EXIT_REASONS);
    static const struct kvm_events_ops exit_events = {
    .is_begin_event = exit_event_begin,
    .is_end_event = exit_event_end,
    .decode_key = exit_event_decode_key,
    .name = "VM-EXIT"
    };
//
// For the mmio events, we treat:
// the time of MMIO write: kvm_mmio(KVM_TRACE_MMIO_WRITE...) -> kvm_entry
// the time of MMIO read: kvm_exit -> kvm_mmio(KVM_TRACE_MMIO_READ...).
//
    static void mmio_event_get_key(struct perf_sample *sample,
    struct event_key *key)
    {
    key.key  = perf_sample__intval(sample, "gpa");
    key.info = perf_sample__intval(sample, "type");
    }
pub const KVM_TRACE_MMIO_READ_UNSATISFIED: c_int = 0;
pub const KVM_TRACE_MMIO_READ: c_int = 1;
pub const KVM_TRACE_MMIO_WRITE: c_int = 2;
#[no_mangle]
unsafe extern "C" fn mmio_event_begin(sample: *mut perf_sample, key: *mut event_key) -> bool {
    static bool mmio_event_begin(struct perf_sample *sample, struct event_key *key)
    {
// MMIO read begin event in kernel.
    if (kvm_exit_event(sample.evsel))
    return true;
// MMIO write begin event in kernel.
    if (evsel__name_is(sample.evsel, "kvm:kvm_mmio") &&
    perf_sample__intval(sample, "type") == KVM_TRACE_MMIO_WRITE) {
    mmio_event_get_key(sample, key);
    return true;
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn mmio_event_end(sample: *mut perf_sample, key: *mut event_key) -> bool {
    static bool mmio_event_end(struct perf_sample *sample, struct event_key *key)
    {
// MMIO write end event in kernel.
    if (kvm_entry_event(sample.evsel))
    return true;
// MMIO read end event in kernel.
    if (evsel__name_is(sample.evsel, "kvm:kvm_mmio") &&
    perf_sample__intval(sample, "type") == KVM_TRACE_MMIO_READ) {
    mmio_event_get_key(sample, key);
    return true;
    }
    return false;
    }
    static void mmio_event_decode_key(struct perf_kvm_stat *kvm __maybe_unused,
    struct event_key *key,
    char *decode)
    {
    scnprintf(decode, KVM_EVENT_NAME_LEN, "%#lx:%s",
    (unsigned long)key.key,
    key.info == KVM_TRACE_MMIO_WRITE ? "W" : "R");
    }
    static const struct kvm_events_ops mmio_events = {
    .is_begin_event = mmio_event_begin,
    .is_end_event = mmio_event_end,
    .decode_key = mmio_event_decode_key,
    .name = "MMIO Access"
    };
// The time of emulation pio access is from kvm_pio to kvm_entry.
    static void ioport_event_get_key(struct perf_sample *sample,
    struct event_key *key)
    {
    key.key  = perf_sample__intval(sample, "port");
    key.info = perf_sample__intval(sample, "rw");
    }
    static bool ioport_event_begin(struct perf_sample *sample,
    struct event_key *key)
    {
    if (evsel__name_is(sample.evsel, "kvm:kvm_pio")) {
    ioport_event_get_key(sample, key);
    return true;
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn ioport_event_end(sample: *mut perf_sample, __maybe_unused: *mut *mut event_key key) -> bool {
    static bool ioport_event_end(struct perf_sample *sample, struct event_key *key __maybe_unused)
    {
    return kvm_entry_event(sample.evsel);
    }
    static void ioport_event_decode_key(struct perf_kvm_stat *kvm __maybe_unused,
    struct event_key *key,
    char *decode)
    {
    scnprintf(decode, KVM_EVENT_NAME_LEN, "%#llx:%s",
    (unsigned long long)key.key,
    key.info ? "POUT" : "PIN");
    }
    static const struct kvm_events_ops ioport_events = {
    .is_begin_event = ioport_event_begin,
    .is_end_event = ioport_event_end,
    .decode_key = ioport_event_decode_key,
    .name = "IO Port Access"
    };
// The time of emulation msr is from kvm_msr to kvm_entry.
#[no_mangle]
unsafe extern "C" fn msr_event_get_key(sample: *mut perf_sample, key: *mut event_key) {
    static void msr_event_get_key(struct perf_sample *sample, struct event_key *key)
    {
    key.key  = perf_sample__intval(sample, "ecx");
    key.info = perf_sample__intval(sample, "write");
    }
#[no_mangle]
unsafe extern "C" fn msr_event_begin(sample: *mut perf_sample, key: *mut event_key) -> bool {
    static bool msr_event_begin(struct perf_sample *sample, struct event_key *key)
    {
    if (evsel__name_is(sample.evsel, "kvm:kvm_msr")) {
    msr_event_get_key(sample, key);
    return true;
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn msr_event_end(sample: *mut perf_sample, __maybe_unused: *mut *mut event_key key) -> bool {
    static bool msr_event_end(struct perf_sample *sample, struct event_key *key __maybe_unused)
    {
    return kvm_entry_event(sample.evsel);
    }
    static void msr_event_decode_key(struct perf_kvm_stat *kvm __maybe_unused,
    struct event_key *key,
    char *decode)
    {
    scnprintf(decode, KVM_EVENT_NAME_LEN, "%#llx:%s",
    (unsigned long long)key.key,
    key.info ? "W" : "R");
    }
    static const struct kvm_events_ops msr_events = {
    .is_begin_event = msr_event_begin,
    .is_end_event = msr_event_end,
    .decode_key = msr_event_decode_key,
    .name = "MSR Access"
    };
    static const char * const __kvm_events_tp[] = {
    "kvm:kvm_entry",
    "kvm:kvm_exit",
    "kvm:kvm_mmio",
    "kvm:kvm_pio",
    "kvm:kvm_msr",
    core::ptr::null_mut(),
    };
    static const struct kvm_reg_events_ops __kvm_reg_events_ops[] = {
    { .name = "vmexit", .ops = &exit_events },
    { .name = "mmio", .ops = &mmio_events },
    { .name = "ioport", .ops = &ioport_events },
    { .name = "msr", .ops = &msr_events },
    { core::ptr::null_mut(), core::ptr::null_mut() },
    };
    static const char * const __kvm_skip_events[] = {
    "HLT",
    core::ptr::null_mut(),
    };
#[no_mangle]
pub unsafe extern "C" fn __cpu_isa_init_x86(kvm: *mut perf_kvm_stat, cpuid: *const c_char) -> c_int {
    int __cpu_isa_init_x86(struct perf_kvm_stat *kvm, const char *cpuid)
    {
    if (strstr(cpuid, "Intel")) {
    kvm.exit_reasons = vmx_exit_reasons;
    kvm.exit_reasons_isa = "VMX";
    } else if (strstr(cpuid, "AMD") || strstr(cpuid, "Hygon")) {
    kvm.exit_reasons = svm_exit_reasons;
    kvm.exit_reasons_isa = "SVM";
    } else
    return -ENOTSUP;
    return 0;
    }
//
// After KVM supports PEBS for guest on Intel platforms
// (https://lore.kernel.org/all/20220411101946.20262-1-likexu@tencent.com/),
// host loses the capability to sample guest with PEBS since all PEBS related
// MSRs are switched to guest value after vm-entry, like IA32_DS_AREA MSR is
// switched to guest GVA at vm-entry. This would lead to "perf kvm record"
// fails to sample guest on Intel platforms since "cycles:P" event is used to
// sample guest by default.
//
// So, to avoid this issue explicitly use "cycles" instead of "cycles:P" event
// by default to sample guest on Intel platforms.
//
#[no_mangle]
pub unsafe extern "C" fn __kvm_add_default_arch_event_x86(argc: *mut c_int, argv: *const c_char) -> c_int {
    int __kvm_add_default_arch_event_x86(int *argc, const char **argv)
    {
    let mut j: c_int = *argc;
    argv[j++] = "-e";
    argv[j++] = "cycles";
// argc += 2;
    return 0;
    }
    const char * const *__kvm_events_tp_x86(void)
    {
    return __kvm_events_tp;
    }
    const struct kvm_reg_events_ops *__kvm_reg_events_ops_x86(void)
    {
    return __kvm_reg_events_ops;
    }
    const char * const *__kvm_skip_events_x86(void)
    {
    return __kvm_skip_events;
    }
