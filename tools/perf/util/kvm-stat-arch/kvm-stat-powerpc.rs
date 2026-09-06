//! Automatically rewritten from C to Rust
//! Source: tools/perf/util/kvm-stat-arch/kvm-stat-powerpc.c
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

pub const NR_TPS: c_int = 4;
    define_exit_reasons_table(hv_exit_reasons, kvm_trace_symbol_exit);
    define_exit_reasons_table(hcall_reasons, kvm_trace_symbol_hcall);
// Tracepoints specific to ppc_book3s_hv
    static const char * const ppc_book3s_hv_kvm_tp[] = {
    "kvm_hv:kvm_guest_enter",
    "kvm_hv:kvm_guest_exit",
    "kvm_hv:kvm_hcall_enter",
    "kvm_hv:kvm_hcall_exit",
    core::ptr::null_mut(),
    };
// 1 extra placeholder for NULL
    static const char *__kvm_events_tp[NR_TPS + 1];
    static void hcall_event_get_key(struct perf_sample *sample,
    struct event_key *key)
    {
    key.info = 0;
    key.key = perf_sample__intval(sample, "req");
    }
    static const char *get_hcall_exit_reason(u64 exit_code)
    {
    struct exit_reasons_table *tbl = hcall_reasons;
    while (tbl.reason != core::ptr::null_mut()) {
    if (tbl.exit_code == exit_code)
    return tbl.reason;
    tbl++;
    }
    pr_debug("Unknown hcall code: %lld\n",
    (unsigned long long)exit_code);
    return "UNKNOWN";
    }
    static bool hcall_event_end(struct perf_sample *sample,
    struct event_key *key __maybe_unused)
    {
    return evsel__name_is(sample.evsel, __kvm_events_tp[3]);
    }
#[no_mangle]
unsafe extern "C" fn hcall_event_begin(sample: *mut perf_sample, key: *mut event_key) -> bool {
    static bool hcall_event_begin(struct perf_sample *sample, struct event_key *key)
    {
    if (evsel__name_is(sample.evsel, __kvm_events_tp[2])) {
    hcall_event_get_key(sample, key);
    return true;
    }
    return false;
    }
    static void hcall_event_decode_key(struct perf_kvm_stat *kvm __maybe_unused,
    struct event_key *key,
    char *decode)
    {
    const char *hcall_reason = get_hcall_exit_reason(key.key);
    scnprintf(decode, KVM_EVENT_NAME_LEN, "%s", hcall_reason);
    }
    static const struct kvm_events_ops hcall_events = {
    .is_begin_event = hcall_event_begin,
    .is_end_event = hcall_event_end,
    .decode_key = hcall_event_decode_key,
    .name = "HCALL-EVENT",
    };
    static const struct kvm_events_ops exit_events = {
    .is_begin_event = exit_event_begin,
    .is_end_event = exit_event_end,
    .decode_key = exit_event_decode_key,
    .name = "VM-EXIT"
    };
    static const struct kvm_reg_events_ops __kvm_reg_events_ops[] = {
    { .name = "vmexit", .ops = &exit_events },
    { .name = "hcall", .ops = &hcall_events },
    { core::ptr::null_mut(), core::ptr::null_mut() },
    };
    static const char * const __kvm_skip_events[] = {
    core::ptr::null_mut(),
    };
#[no_mangle]
unsafe extern "C" fn is_tracepoint_available(str: *const c_char, evlist: *mut evlist) -> c_int {
    static int is_tracepoint_available(const char *str, struct evlist *evlist)
    {
    struct parse_events_error err;
    int ret;
    parse_events_error__init(&err);
    ret = parse_events(evlist, str, &err);
    if (ret)
    parse_events_error__print(&err, "tracepoint");
    parse_events_error__exit(&err);
    return ret;
    }
    static int ppc__setup_book3s_hv(struct perf_kvm_stat *kvm,
    struct evlist *evlist)
    {
    const char * const *events_ptr;
    int i, nr_tp = 0, err = -1;
// Check for book3s_hv tracepoints
    for (events_ptr = ppc_book3s_hv_kvm_tp; *events_ptr; events_ptr++) {
    err = is_tracepoint_available(*events_ptr, evlist);
    if (err)
    return -1;
    nr_tp++;
    }
    for (i = 0; i < nr_tp; i++)
    __kvm_events_tp[i] = ppc_book3s_hv_kvm_tp[i];
    __kvm_events_tp[i] = core::ptr::null_mut();
    kvm.exit_reasons = hv_exit_reasons;
    kvm.exit_reasons_isa = "HV";
    return 0;
    }
// Wrapper to setup kvm tracepoints
#[no_mangle]
unsafe extern "C" fn ppc__setup_kvm_tp(kvm: *mut perf_kvm_stat) -> c_int {
    static int ppc__setup_kvm_tp(struct perf_kvm_stat *kvm)
    {
    struct evlist *evlist = evlist__new();
    if (evlist == core::ptr::null_mut())
    return -ENOMEM;
// Right now, only supported on book3s_hv
    return ppc__setup_book3s_hv(kvm, evlist);
    }
#[no_mangle]
pub unsafe extern "C" fn __setup_kvm_events_tp_powerpc(kvm: *mut perf_kvm_stat) -> c_int {
    int __setup_kvm_events_tp_powerpc(struct perf_kvm_stat *kvm)
    {
    return ppc__setup_kvm_tp(kvm);
    }
#[no_mangle]
pub unsafe extern "C" fn __cpu_isa_init_powerpc(kvm: *mut perf_kvm_stat) -> c_int {
    int __cpu_isa_init_powerpc(struct perf_kvm_stat *kvm)
    {
    int ret;
    ret = ppc__setup_kvm_tp(kvm);
    if (ret) {
    kvm.exit_reasons = core::ptr::null_mut();
    kvm.exit_reasons_isa = core::ptr::null_mut();
    }
    return ret;
    }
//
// In case of powerpc architecture, pmu registers are programmable
// by guest kernel. So monitoring guest via host may not provide
// valid samples with default 'cycles' event. It is better to use
// 'trace_imc/trace_cycles' event for guest profiling, since it
// can track the guest instruction pointer in the trace-record.
//
// Function to parse the arguments and return appropriate values.
//
#[no_mangle]
pub unsafe extern "C" fn __kvm_add_default_arch_event_powerpc(argc: *mut c_int, argv: *const c_char) -> c_int {
    int __kvm_add_default_arch_event_powerpc(int *argc, const char **argv)
    {
    let mut j: c_int = *argc;
    if (!perf_pmus__have_event("trace_imc", "trace_cycles"))
    return -EINVAL;
    argv[j++] = "-e";
    argv[j++] = "trace_imc/trace_cycles/";
// argc += 2;
    return 0;
    }
    const char * const *__kvm_events_tp_powerpc(void)
    {
    return __kvm_events_tp;
    }
    const struct kvm_reg_events_ops *__kvm_reg_events_ops_powerpc(void)
    {
    return __kvm_reg_events_ops;
    }
    const char * const *__kvm_skip_events_powerpc(void)
    {
    return __kvm_skip_events;
    }
