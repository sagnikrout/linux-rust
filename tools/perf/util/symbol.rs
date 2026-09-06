//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/symbol.h
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
pub const __PERF_SYMBOL: c_int = 1;

//
// Ignore kernel mapping symbols, matching kernel is_mapping_symbol() logic.
// This checks for '$' prefix (used by ARM, AArch64, RISC-V) and
// x86 local symbol prefixes (.L* and L0*).
// Only use this for kernel symbols (kallsyms, ksymbol events, kernel ELF DSOs).
//
// Livepatch symbols (.klp.sym.*) are relocation placeholders whose resolved
// addresses alias existing kernel symbols.  They carry a [module] tag which
// confuses module boundary tracking and symbol table lookups.
//
extern "C" {
    pub fn strstarts(_arg: str, _arg: KLP_SYM_PREFIX) -> return;
}
//
// libelf 0.8.x and earlier do not support ELF_C_READ_MMAP;
// for newer versions we can use mmap to reduce memory usage:
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum symbol_idle_kind {
    SYMBOL_IDLE__UNKNOWN = 0,
    SYMBOL_IDLE__NOT_IDLE = 1,
    SYMBOL_IDLE__IDLE = 2,
}

pub const SYMBOL_FLAG_TYPE_SHIFT: c_int = 0;

pub const SYMBOL_FLAG_BINDING_SHIFT: c_int = 4;

pub const SYMBOL_FLAG_IDLE_SHIFT: c_int = 8;

//
// A symtab entry. When allocated this may be preceded by an annotation (see
// symbol__annotation) and/or a browser_index (see symbol__browser_index).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct symbol {
    pub rb_node: rb_node,
// Range of symbol [start, end).
    pub start: u64,
    pub end: u64,
// Length of the string name.
    pub namelen: u16,
    pub flags: _Atomic uint16_t,
// Architecture specific. Unused except on PPC where it holds st_other.
    pub arch_sym: u8,
// The name of length namelen associated with the symbol.
    pub name: [c_char; ],
}

extern "C" {
    pub fn symbol__delete(sym: *mut symbol);
}
extern "C" {
    pub fn symbols__delete(symbols: *mut rb_root_cached);
}
extern "C" {
    pub fn symbol__is_idle(sym: *mut symbol, dso: *const dso, env: *mut perf_env) -> bool;
}
extern "C" {
    pub fn symbol__set_ignore(sym: *mut symbol, ignore: bool);
}
extern "C" {
    pub fn symbol__set_annotate2(sym: *mut symbol, annotate2: bool);
}
extern "C" {
    pub fn symbol__set_inlined(sym: *mut symbol, inlined: bool);
}
extern "C" {
    pub fn symbol__set_ifunc_alias(sym: *mut symbol, ifunc_alias: bool);
}
// symbols__for_each_entry - iterate over symbols (rb_root)
//
// @symbols: the rb_root of symbols
// @pos: the 'struct symbol *' to use as a loop cursor
// @nd: the 'struct rb_node *' to use as a temporary storage
//

extern "C" {
    pub fn path__join(_arg: bf, _arg: size, _arg: symbol_conf.symfs, _arg: perf_basename(path)) -> return;
}
extern "C" {
    pub fn path__join(_arg: bf, _arg: size, _arg: symbol_conf.symfs, _arg: path) -> return;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ref_reloc_sym {
    pub name: *const c_char,
    pub addr: u64,
    pub unrelocated_addr: u64,
}

extern "C" {
    pub fn dso__load(dso: *mut dso, map: *mut map) -> c_int;
}
extern "C" {
    pub fn dso__load_vmlinux_path(dso: *mut dso, map: *mut map) -> c_int;
}
extern "C" {
    pub fn dso__load_kallsyms(dso: *mut dso, filename: *const c_char, map: *mut map) -> c_int;
}
extern "C" {
    pub fn dso__type_fd(fd: c_int) -> dso_type;
}
extern "C" {
    pub fn filename__read_build_id(filename: *const c_char, id: *mut build_id) -> c_int;
}
extern "C" {
    pub fn sysfs__read_build_id(filename: *const c_char, bid: *mut build_id) -> c_int;
}
extern "C" {
    pub fn filename__has_section(filename: *const c_char, sec: *const c_char) -> bool;
}
extern "C" {
    pub fn symbol__init(env: *mut perf_env) -> c_int;
}
extern "C" {
    pub fn symbol__exit();
}
extern "C" {
    pub fn symbol__elf_init();
}
extern "C" {
    pub fn symbol__annotation_init() -> c_int;
}
extern "C" {
    pub fn symbol__fprintf_symname(sym: *const symbol, fp: *mut FILE) -> usize;
}
extern "C" {
    pub fn symbol__fprintf(sym: *mut symbol, fp: *mut FILE) -> usize;
}

extern "C" {
    pub fn dso__load_bfd_symbols(dso: *mut dso, debugfile: *const c_char) -> c_int;
}

extern "C" {
    pub fn dso__synthesize_plt_symbols(dso: *mut dso, ss: *mut symsrc) -> c_int;
}
extern "C" {
    pub fn __symbols__insert(symbols: *mut rb_root_cached, sym: *mut symbol);
}
extern "C" {
    pub fn symbols__insert(symbols: *mut rb_root_cached, sym: *mut symbol);
}
extern "C" {
    pub fn symbols__fixup_duplicate(symbols: *mut rb_root_cached);
}
extern "C" {
    pub fn symbols__fixup_end(symbols: *mut rb_root_cached, is_kallsyms: bool);
}
extern "C" {
    pub fn int(start: *mut *mut mapfn_t)(u64, len: u64, pgoff: u64, data: *mut c_void) -> typedef;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kcore_extract {
    pub kcore_filename: *mut c_char,
    pub addr: u64,
    pub offs: u64,
    pub len: u64,
    pub extract_filename: [c_char; sizeof(PERF_KCORE_EXTRACT)],
    pub fd: c_int,
}

extern "C" {
    pub fn kcore_extract__create(kce: *mut kcore_extract) -> c_int;
}
extern "C" {
    pub fn kcore_extract__delete(kce: *mut kcore_extract);
}
extern "C" {
    pub fn kcore_copy(from_dir: *const c_char, to_dir: *const c_char) -> c_int;
}
extern "C" {
    pub fn compare_proc_modules(from: *const c_char, to: *const c_char) -> c_int;
}

extern "C" {
    pub fn arch__sym_update(s: *mut symbol, sym: *mut GElf_Sym);
}

pub const SYMBOL_A: c_int = 0;
pub const SYMBOL_B: c_int = 1;
extern "C" {
    pub fn arch__compare_symbol_names(namea: *const c_char, nameb: *const c_char) -> c_int;
}
extern "C" {
    pub fn arch__choose_best_symbol(syma: *mut symbol, symb: *mut symbol) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum symbol_tag_include {
    SYMBOL_TAG_INCLUDE__NONE = 0,
    SYMBOL_TAG_INCLUDE__DEFAULT_ONLY
}

// structure containing an SDT note's info
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdt_note {
    pub note*/: *mut *mut *mut char name; / name of the,
    pub /: *mut *mut *mut char provider; / provider name,
    pub args: *mut c_char,
    pub /: *mut *mut bool bit32; / whether the location is 32 bits?,
    pub a64: [Elf64_Addr; 3],
    pub a32: [Elf32_Addr; 3],
    pub addr: },
    pub /: *mut *mut list_head note_list; / SDT notes' list,
}

extern "C" {
    pub fn get_sdt_note_list(head: *mut list_head, target: *const c_char) -> c_int;
}
extern "C" {
    pub fn cleanup_sdt_note_list(sdt_notes: *mut list_head) -> c_int;
}
extern "C" {
    pub fn sdt_notes__get_count(start: *mut list_head) -> c_int;
}

pub const SDT_NOTE_TYPE: c_int = 3;

pub const NR_ADDR: c_int = 3;
extern "C" {
    pub fn symbol__validate_sym_arguments() -> c_int;
}
