//! Automatically rewritten from C to Rust
//! Source: tools/perf/arch/powerpc/util/auxtrace.c
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
// VPA support
//

    static int
    powerpc_vpadtl_recording_options(struct auxtrace_record *ar __maybe_unused,
    struct evlist *evlist __maybe_unused,
    struct record_opts *opts)
    {
    opts.full_auxtrace = true;
//
// Set auxtrace_mmap_pages to minimum
// two pages
//
    if (!opts.auxtrace_mmap_pages) {
    opts.auxtrace_mmap_pages = KiB(128) / page_size;
    if (opts.mmap_pages == UINT_MAX)
    opts.mmap_pages = KiB(256) / page_size;
    }
    return 0;
    }
    static size_t powerpc_vpadtl_info_priv_size(struct auxtrace_record *itr __maybe_unused,
    struct evlist *evlist __maybe_unused)
    {
    return VPADTL_AUXTRACE_PRIV_SIZE;
    }
    static int
    powerpc_vpadtl_info_fill(struct auxtrace_record *itr __maybe_unused,
    struct perf_session *session __maybe_unused,
    struct perf_record_auxtrace_info *auxtrace_info,
    size_t priv_size __maybe_unused)
    {
    auxtrace_info.type = PERF_AUXTRACE_VPA_DTL;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn powerpc_vpadtl_free(itr: *mut auxtrace_record) {
    static void powerpc_vpadtl_free(struct auxtrace_record *itr)
    {
    free(itr);
    }
#[no_mangle]
unsafe extern "C" fn powerpc_vpadtl_reference(__maybe_unused: *mut *mut auxtrace_record itr) -> u64 {
    static u64 powerpc_vpadtl_reference(struct auxtrace_record *itr __maybe_unused)
    {
    return 0;
    }
    struct auxtrace_record *auxtrace_record__init(struct evlist *evlist,
    int *err)
    {
    struct auxtrace_record *aux;
    struct evsel *pos;
    let mut found: c_int = 0;
//
// Set err value to zero here. Any fail later
// will set appropriate return code to err.
//
// err = 0;
    evlist__for_each_entry(evlist, pos) {
    if (strstarts(pos.name, "vpa_dtl")) {
    found = 1;
    pos.needs_auxtrace_mmap = true;
    break;
    }
    }
    if (!found)
    return core::ptr::null_mut();
//
// To obtain the auxtrace buffer file descriptor, the auxtrace event
// must come first.
//
    evlist__to_front(pos.evlist, pos);
    aux = zalloc(sizeof(*aux));
    if (aux == core::ptr::null_mut()) {
    pr_debug("aux record is core::ptr::null_mut()\n");
// err = -ENOMEM;
    return core::ptr::null_mut();
    }
    aux.recording_options = powerpc_vpadtl_recording_options;
    aux.info_priv_size = powerpc_vpadtl_info_priv_size;
    aux.info_fill = powerpc_vpadtl_info_fill;
    aux.free = powerpc_vpadtl_free;
    aux.reference = powerpc_vpadtl_reference;
    return aux;
    }
