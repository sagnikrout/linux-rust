//! Automatically rewritten from C to Rust
//! Source: tools/perf/trace/beauty/mmap.c
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


// SPDX-License-Identifier: LGPL-2.1

    static DEFINE_STRARRAY(mmap_prot, "PROT_");
#[no_mangle]
unsafe extern "C" fn mmap__scnprintf_prot(prot: c_ulong, bf: *mut c_char, size: usize, show_prefix: bool) -> usize {
    static size_t mmap__scnprintf_prot(unsigned long prot, char *bf, size_t size, bool show_prefix)
    {
    return strarray__scnprintf_flags(&strarray__mmap_prot, bf, size, show_prefix, prot);
    }
#[no_mangle]
pub unsafe extern "C" fn syscall_arg__scnprintf_mmap_prot(bf: *mut c_char, size: usize, arg: *mut syscall_arg) -> usize {
    size_t syscall_arg__scnprintf_mmap_prot(char *bf, size_t size, struct syscall_arg *arg)
    {
    let mut prot: c_ulong = arg.val;
    if (prot == 0)
    return scnprintf(bf, size, "%sNONE", arg.show_string_prefix ? strarray__mmap_prot.prefix : "");
    return mmap__scnprintf_prot(prot, bf, size, arg.show_string_prefix);
    }

    DEFINE_STRARRAY(mmap_flags, "MAP_");
#[no_mangle]
unsafe extern "C" fn mmap__scnprintf_flags(flags: c_ulong, bf: *mut c_char, size: usize, show_prefix: bool) -> usize {
    static size_t mmap__scnprintf_flags(unsigned long flags, char *bf, size_t size, bool show_prefix)
    {
    return strarray__scnprintf_flags(&strarray__mmap_flags, bf, size, show_prefix, flags);
    }
    size_t syscall_arg__scnprintf_mmap_flags(char *bf, size_t size,
    struct syscall_arg *arg)
    {
    let mut flags: c_ulong = arg.val;
    if (flags & MAP_ANONYMOUS)
    arg.mask |= (1 << 4) | (1 << 5); /* Mask 4th ('fd') and 5th ('offset') args, ignored */
    return mmap__scnprintf_flags(flags, bf, size, arg.show_string_prefix);
    }

    static DEFINE_STRARRAY(mremap_flags, "MREMAP_");
#[no_mangle]
unsafe extern "C" fn mremap__scnprintf_flags(flags: c_ulong, bf: *mut c_char, size: usize, show_prefix: bool) -> usize {
    static size_t mremap__scnprintf_flags(unsigned long flags, char *bf, size_t size, bool show_prefix)
    {
    return strarray__scnprintf_flags(&strarray__mremap_flags, bf, size, show_prefix, flags);
    }
#[no_mangle]
pub unsafe extern "C" fn syscall_arg__scnprintf_mremap_flags(bf: *mut c_char, size: usize, arg: *mut syscall_arg) -> usize {
    size_t syscall_arg__scnprintf_mremap_flags(char *bf, size_t size, struct syscall_arg *arg)
    {
    let mut flags: c_ulong = arg.val;
    if (!(flags & MREMAP_FIXED))
    arg.mask |=  (1 << 5); /* Mask 5th ('new_address') args, ignored */
    return mremap__scnprintf_flags(flags, bf, size, arg.show_string_prefix);
    }
#[no_mangle]
unsafe extern "C" fn madvise__scnprintf_behavior(behavior: c_int, bf: *mut c_char, size: usize) -> usize {
    static size_t madvise__scnprintf_behavior(int behavior, char *bf, size_t size)
    {

    static DEFINE_STRARRAY(madvise_advices, "MADV_");
    if (behavior < strarray__madvise_advices.nr_entries && strarray__madvise_advices.entries[behavior] != core::ptr::null_mut())
    return scnprintf(bf, size, "MADV_%s", strarray__madvise_advices.entries[behavior]);
    return scnprintf(bf, size, "%#", behavior);
    }
    size_t syscall_arg__scnprintf_madvise_behavior(char *bf, size_t size,
    struct syscall_arg *arg)
    {
    return madvise__scnprintf_behavior(arg.val, bf, size);
    }
