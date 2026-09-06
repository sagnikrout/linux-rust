//! Automatically rewritten from C to Rust
//! Source: tools/perf/arch/powerpc/util/sym-handling.c
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
// Copyright (C) 2015 Naveen N. Rao, IBM Corporation
//

    int arch__choose_best_symbol(struct symbol *syma,
    struct symbol *symb __maybe_unused)
    {
    char *sym = syma.name;

// Skip over any initial dot
    if (*sym == '.')
    sym++;

// Avoid "SyS" kernel syscall aliases
    if (strlen(sym) >= 3 && !strncmp(sym, "SyS", 3))
    return SYMBOL_B;
    if (strlen(sym) >= 10 && !strncmp(sym, "compat_SyS", 10))
    return SYMBOL_B;
    return SYMBOL_A;
    }

// Allow matching against dot variants
#[no_mangle]
pub unsafe extern "C" fn arch__compare_symbol_names(namea: *const c_char, nameb: *const c_char) -> c_int {
    int arch__compare_symbol_names(const char *namea, const char *nameb)
    {
// Skip over initial dot
    if (*namea == '.')
    namea++;
    if (*nameb == '.')
    nameb++;
    return strcmp(namea, nameb);
    }
    int arch__compare_symbol_names_n(const char *namea, const char *nameb,
    unsigned int n)
    {
// Skip over initial dot
    if (*namea == '.')
    namea++;
    if (*nameb == '.')
    nameb++;
    return strncmp(namea, nameb, n);
    }
    const char *arch__normalize_symbol_name(const char *name)
    {
// Skip over initial dot
    if (name && *name == '.')
    name++;
    return name;
    }

#[no_mangle]
pub unsafe extern "C" fn arch__sym_update(s: *mut symbol, sym: *mut GElf_Sym) {
    void arch__sym_update(struct symbol *s, GElf_Sym *sym)
    {
    s.arch_sym = sym.st_other;
    }

pub const PPC64LE_LEP_OFFSET: c_int = 8;
    void arch__fix_tev_from_maps(struct perf_probe_event *pev,
    struct probe_trace_event *tev, struct map *map,
    struct symbol *sym)
    {
    int lep_offset;
//
// When probing at a function entry point, we normally always want the
// LEP since that catches calls to the function through both the GEP and
// the LEP. Hence, we would like to probe at an offset of 8 bytes if
// the user only specified the function entry.
//
// However, if the user specifies an offset, we fall back to using the
// GEP since all userspace applications (objdump/readelf) show function
// disassembly with offsets from the GEP.
//
    if (pev.point.offset || !map || !sym)
    return;
// For kretprobes, add an offset only if the kernel supports it
    if (!pev.uprobes && pev.point.retprobe) {

    if (!kretprobe_offset_is_supported())

    return;
    }
    lep_offset = PPC64_LOCAL_ENTRY_OFFSET(sym.arch_sym);
    if (map__dso(map).symtab_type == DSO_BINARY_TYPE__KALLSYMS)
    tev.point.offset += PPC64LE_LEP_OFFSET;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: lep_offset) -> else {
    if (pev.uprobes)
    tev.point.address += lep_offset;
    else
    tev.point.offset += lep_offset;
    }
    }

    void arch__post_process_probe_trace_events(struct perf_probe_event *pev,
    int ntevs)
    {
    struct probe_trace_event *tev;
    struct map *map;
    struct symbol *sym = core::ptr::null_mut();
    struct rb_node *tmp;
    let mut i: c_int = 0;
    map = get_target_map(pev.target, pev.nsi, pev.uprobes);
    if (!map || map__load(map) < 0)
    return;
    for (i = 0; i < ntevs; i++) {
    tev = &pev.tevs[i];
    map__for_each_symbol(map, sym, tmp) {
    if (map__unmap_ip(map, sym.start) == tev.point.address) {
    arch__fix_tev_from_maps(pev, tev, map, sym);
    break;
    }
    }
    }
    }

