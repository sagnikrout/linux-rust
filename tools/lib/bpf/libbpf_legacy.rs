//! Automatically rewritten from C Header to Rust Module
//! Source: tools/lib/bpf/libbpf_legacy.h
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


// SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause)
//
// Libbpf legacy APIs (either discouraged or deprecated, as mentioned in [0])
//
// [0] https://docs.google.com/document/d/1UyjTZuPFWiPFyKk1tV5an11_iaRuec6U-ZESZ54nNTY
//
// Copyright (C) 2021 Facebook
//

// As of libbpf 1.0 libbpf_set_strict_mode() and enum libbpf_struct_mode have
// no effect. But they are left in libbpf_legacy.h so that applications that
// prepared for libbpf 1.0 before final release by using
// libbpf_set_strict_mode() still work with libbpf 1.0+ without any changes.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum libbpf_strict_mode {
// Turn on all supported strict features of libbpf to simulate libbpf
// v1.0 behavior.
// This will be the default behavior in libbpf v1.0.
//
    LIBBPF_STRICT_ALL = 0xffffffff,

//
// Disable any libbpf 1.0 behaviors. This is the default before libbpf
// v1.0. It won't be supported anymore in v1.0, please update your
// code so that it handles LIBBPF_STRICT_ALL mode before libbpf v1.0.
//
    LIBBPF_STRICT_NONE = 0x00,
//
// Return NULL pointers on error, not ERR_PTR(err).
// Additionally, libbpf also always sets errno to corresponding Exx
// (positive) error code.
//
    LIBBPF_STRICT_CLEAN_PTRS = 0x01,
//
// Return actual error codes from low-level APIs directly, not just -1.
// Additionally, libbpf also always sets errno to corresponding Exx
// (positive) error code.
//
    LIBBPF_STRICT_DIRECT_ERRS = 0x02,
//
// Enforce strict BPF program section (SEC()) names.
// E.g., while prefiously SEC("xdp_whatever") or SEC("perf_event_blah") were
// allowed, with LIBBPF_STRICT_SEC_PREFIX this will become
// unrecognized by libbpf and would have to be just SEC("xdp") and
// SEC("xdp") and SEC("perf_event").
//
// Note, in this mode the program pin path will be based on the
// function name instead of section name.
//
// Additionally, routines in the .text section are always considered
// sub-programs. Legacy behavior allows for a single routine in .text
// to be a program.
//
    LIBBPF_STRICT_SEC_NAME = 0x04,
//
// Disable the global 'bpf_objects_list'. Maintaining this list adds
// a race condition to bpf_object__open() and bpf_object__close().
// Clients can maintain it on their own if it is valuable for them.
//
    LIBBPF_STRICT_NO_OBJECT_LIST = 0x08,
//
// Automatically bump RLIMIT_MEMLOCK using setrlimit() before the
// first BPF program or map creation operation. This is done only if
// kernel is too old to support memcg-based memory accounting for BPF
// subsystem. By default, RLIMIT_MEMLOCK limit is set to RLIM_INFINITY,
// but it can be overridden with libbpf_set_memlock_rlim() API.
// Note that libbpf_set_memlock_rlim() needs to be called before
// the very first bpf_prog_load(), bpf_map_create() or bpf_object__load()
// operation.
//
    LIBBPF_STRICT_AUTO_RLIMIT_MEMLOCK = 0x10,
//
// Error out on any SEC("maps") map definition, which are deprecated
// in favor of BTF-defined map definitions in SEC(".maps").
//
    LIBBPF_STRICT_MAP_DEFINITIONS = 0x20,

    __LIBBPF_STRICT_LAST,
}

extern "C" {
    pub fn libbpf_set_strict_mode(mode: libbpf_strict_mode) -> LIBBPF_API int;
}
//
// @brief **libbpf_get_error()** extracts the error code from the passed
// pointer
// @param ptr pointer returned from libbpf API function
// @return error code; or 0 if no error occurred
//
// Note, as of libbpf 1.0 this function is not necessary and not recommended
// to be used. Libbpf doesn't return error code embedded into the pointer
// itself. Instead, NULL is returned on error and error code is passed through
// thread-local errno variable. **libbpf_get_error()** is just returning -errno
// value if it receives NULL, which is correct only if errno hasn't been
// modified between libbpf API call and corresponding **libbpf_get_error()
// call. Prefer to check return for NULL and use errno directly.
//
// This API is left in libbpf 1.0 to allow applications that were 1.0-ready
// before final libbpf 1.0 without needing to change them.
//
extern "C" {
    pub fn libbpf_get_error(ptr: *const c_void) -> LIBBPF_API long;
}

// "Discouraged" APIs which don't follow consistent libbpf naming patterns.
// They are normally a trivial aliases or wrappers for proper APIs and are
// left to minimize unnecessary disruption for users of libbpf. But they
// shouldn't be used going forward.
//
extern "C" {
    pub fn bpf_program__get_type(prog: *const bpf_program) -> LIBBPF_API enum bpf_prog_type;
}
extern "C" {
    pub fn bpf_program__get_expected_attach_type(prog: *const bpf_program) -> LIBBPF_API enum bpf_attach_type;
}

