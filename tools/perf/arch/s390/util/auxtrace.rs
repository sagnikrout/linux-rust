//! Automatically rewritten from C to Rust
//! Source: tools/perf/arch/s390/util/auxtrace.c
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


pub const PERF_EVENT_CPUM_SF: c_uint = 0xB0000 /* Event: Basic-sampling */;
pub const PERF_EVENT_CPUM_SF_DIAG: c_uint = 0xBD000 /* Event: Combined-sampling */;
pub const DEFAULT_AUX_PAGES: c_int = 128;
pub const DEFAULT_FREQ: c_int = 4000;
#[no_mangle]
unsafe extern "C" fn cpumsf_free(itr: *mut auxtrace_record) {
    static void cpumsf_free(struct auxtrace_record *itr)
    {
    free(itr);
    }
    static size_t cpumsf_info_priv_size(struct auxtrace_record *itr __maybe_unused,
    struct evlist *evlist __maybe_unused)
    {
    return 0;
    }
    static int
    cpumsf_info_fill(struct auxtrace_record *itr __maybe_unused,
    struct perf_session *session __maybe_unused,
    struct perf_record_auxtrace_info *auxtrace_info __maybe_unused,
    size_t priv_size __maybe_unused)
    {
    auxtrace_info.type = PERF_AUXTRACE_S390_CPUMSF;
    return 0;
    }
    static unsigned long
    cpumsf_reference(struct auxtrace_record *itr __maybe_unused)
    {
    return 0;
    }
    static int
    cpumsf_recording_options(struct auxtrace_record *ar __maybe_unused,
    struct evlist *evlist __maybe_unused,
    struct record_opts *opts)
    {
    let mut factor: c_uint = 1;
    unsigned int pages;
    opts.full_auxtrace = true;
//
// The AUX buffer size should be set properly to avoid
// overflow of samples if it is not set explicitly.
// DEFAULT_AUX_PAGES is an proper size when sampling frequency
// is DEFAULT_FREQ. It is expected to hold about 1/2 second
// of sampling data. The size used for AUX buffer will scale
// according to the specified frequency and DEFAULT_FREQ.
//
    if (!opts.auxtrace_mmap_pages) {
    if (opts.user_freq != UINT_MAX)
    factor = (opts.user_freq + DEFAULT_FREQ
    - 1) / DEFAULT_FREQ;
    pages = DEFAULT_AUX_PAGES * factor;
    opts.auxtrace_mmap_pages = roundup_pow_of_two(pages);
    }
    return 0;
    }
    static int
    cpumsf_parse_snapshot_options(struct auxtrace_record *itr __maybe_unused,
    struct record_opts *opts __maybe_unused,
    const char *str __maybe_unused)
    {
    return 0;
    }
//
// auxtrace_record__init is called when perf record
// check if the event really need auxtrace
//
    struct auxtrace_record *auxtrace_record__init(struct evlist *evlist,
    int *err)
    {
    struct auxtrace_record *aux;
    struct evsel *pos;
    let mut diagnose: c_int = 0;
// err = 0;
    if (evlist.core.nr_entries == 0)
    return core::ptr::null_mut();
    evlist__for_each_entry(evlist, pos) {
    if (pos.core.attr.config == PERF_EVENT_CPUM_SF_DIAG) {
    diagnose = 1;
    pos.needs_auxtrace_mmap = true;
    break;
    }
    }
    if (!diagnose)
    return core::ptr::null_mut();
// sampling in diagnose mode. alloc aux buffer
    aux = zalloc(sizeof(*aux));
    if (aux == core::ptr::null_mut()) {
// err = -ENOMEM;
    return core::ptr::null_mut();
    }
    aux.parse_snapshot_options = cpumsf_parse_snapshot_options;
    aux.recording_options = cpumsf_recording_options;
    aux.info_priv_size = cpumsf_info_priv_size;
    aux.info_fill = cpumsf_info_fill;
    aux.free = cpumsf_free;
    aux.reference = cpumsf_reference;
    return aux;
    }
