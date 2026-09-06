//! Automatically rewritten from C to Rust
//! Source: tools/perf/util/symbol_fprintf.c
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

#[no_mangle]
pub unsafe extern "C" fn symbol__fprintf(sym: *mut symbol, fp: *mut FILE) -> usize {
    size_t symbol__fprintf(struct symbol *sym, FILE *fp)
    {
    return fprintf(fp, " %" PRIx64 "-%" PRIx64 " %c %s\n",
    sym.start, sym.end,
    symbol__binding(sym) == STB_GLOBAL ? 'g' :
    symbol__binding(sym) == STB_LOCAL  ? 'l' : 'w',
    sym.name);
    }
    size_t __symbol__fprintf_symname_offs(const struct symbol *sym,
    const struct addr_location *al,
    bool unknown_as_addr,
    bool print_offsets, FILE *fp)
    {
    unsigned long offset;
    size_t length;
    if (sym) {
    length = fprintf(fp, "%s", sym.name);
    if (al && print_offsets) {
    if (al.addr < sym.end)
    offset = al.addr - sym.start;
    else
    offset = al.addr - map__start(al.map) - sym.start;
    length += fprintf(fp, "+0x%lx", offset);
    }
    return length;
    } else if (al && unknown_as_addr)
    return fprintf(fp, "[%#" PRIx64 "]", al.addr);
    else
    return fprintf(fp, "[unknown]");
    }
    size_t symbol__fprintf_symname_offs(const struct symbol *sym,
    const struct addr_location *al,
    FILE *fp)
    {
    return __symbol__fprintf_symname_offs(sym, al, false, true, fp);
    }
    size_t __symbol__fprintf_symname(const struct symbol *sym,
    const struct addr_location *al,
    bool unknown_as_addr, FILE *fp)
    {
    return __symbol__fprintf_symname_offs(sym, al, unknown_as_addr, false, fp);
    }
#[no_mangle]
pub unsafe extern "C" fn symbol__fprintf_symname(sym: *const symbol, fp: *mut FILE) -> usize {
    size_t symbol__fprintf_symname(const struct symbol *sym, FILE *fp)
    {
    return __symbol__fprintf_symname_offs(sym, core::ptr::null_mut(), false, false, fp);
    }
    size_t dso__fprintf_symbols_by_name(struct dso *dso,
    FILE *fp)
    {
    let mut ret: usize = 0;
    for (size_t i = 0; i < dso__symbol_names_len(dso); i++) {
    struct symbol *pos = dso__symbol_names(dso)[i];
    ret += fprintf(fp, "%s\n", pos.name);
    }
    return ret;
    }
