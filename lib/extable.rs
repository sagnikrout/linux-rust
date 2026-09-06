//! Automatically rewritten from C to Rust
//! Source: lib/extable.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Derived from arch/ppc/mm/extable.c and arch/i386/mm/extable.c.
//
// Copyright (C) 2004 Paul Mackerras, IBM Corp.
//

#[no_mangle]
pub unsafe extern "C" fn ex_to_insn(x: *const exception_table_entry) -> c_ulong {
    static inline unsigned long ex_to_insn(const struct exception_table_entry *x)
    {
    return (unsigned long)&x.insn + x.insn;
    }

#[no_mangle]
unsafe extern "C" fn swap_ex(a: *mut c_void, b: *mut c_void, size: c_int) {
    static void swap_ex(void *a, void *b, int size)
    {
    struct exception_table_entry *x = a, *y = b, tmp;
    let mut delta: c_int = b - a;
    tmp = *x;
    x.insn = y.insn + delta;
    y.insn = tmp.insn - delta;

    swap_ex_entry_fixup(x, y, tmp, delta);

    x.fixup = y.fixup + delta;
    y.fixup = tmp.fixup - delta;

    }

//
// The exception table needs to be sorted so that the binary
// search that we use to find entries in it works properly.
// This is used both for the kernel exception table and for
// the exception tables of modules that get loaded.
//
#[no_mangle]
unsafe extern "C" fn cmp_ex_sort(a: *const c_void, b: *const c_void) -> c_int {
    static int cmp_ex_sort(const void *a, const void *b)
    {
    const struct exception_table_entry *x = a, *y = b;
// avoid overflow
    if (ex_to_insn(x) > ex_to_insn(y))
    return 1;
    if (ex_to_insn(x) < ex_to_insn(y))
    return -1;
    return 0;
    }
    void sort_extable(struct exception_table_entry *start,
    struct exception_table_entry *finish)
    {
    sort(start, finish - start, sizeof(struct exception_table_entry),
    cmp_ex_sort, swap_ex);
    }

//
// If the exception table is sorted, any referring to the module init
// will be at the beginning or the end.
//
#[no_mangle]
pub unsafe extern "C" fn trim_init_extable(m: *mut module) {
    void trim_init_extable(struct module *m)
    {
// trim the beginning
    while (m.num_exentries &&
    within_module_init(ex_to_insn(&m.extable[0]), m)) {
    m.extable++;
    m.num_exentries--;
    }
// trim the end
    while (m.num_exentries &&
    within_module_init(ex_to_insn(&m.extable[m.num_exentries - 1]),
    m))
    m.num_exentries--;
    }

#[no_mangle]
unsafe extern "C" fn cmp_ex_search(key: *const c_void, elt: *const c_void) -> c_int {
    static int cmp_ex_search(const void *key, const void *elt)
    {
    const struct exception_table_entry *_elt = elt;
    let mut _key: c_ulong = *(unsigned long *)key;
// avoid overflow
    if (_key > ex_to_insn(_elt))
    return 1;
    if (_key < ex_to_insn(_elt))
    return -1;
    return 0;
    }
//
// Search one exception table for an entry corresponding to the
// given instruction address, and return the address of the entry,
// or NULL if none is found.
// We use a binary search, and thus we assume that the table is
// already sorted.
//
    const struct exception_table_entry *
    search_extable(const struct exception_table_entry *base,
    const size_t num,
    unsigned long value)
    {
    return bsearch(&value, base, num,
    sizeof(struct exception_table_entry), cmp_ex_search);
    }
