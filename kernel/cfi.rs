//! Automatically rewritten from C to Rust
//! Source: kernel/cfi.c
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
// Clang Control Flow Integrity (CFI) error handling.
//
// Copyright (C) 2022 Google LLC
//

    let mut __ro_after_init: bool cfi_warn = IS_ENABLED(CONFIG_CFI_PERMISSIVE);
    enum bug_trap_type report_cfi_failure(struct pt_regs *regs, unsigned long addr,
    unsigned long *target, u32 type)
    {
    if (target)
    pr_err("CFI failure at %pS (target: %pS; expected type: 0x%08x)\n",
    (void *)addr, (void *)*target, type);
    else
    pr_err("CFI failure at %pS (no target information)\n",
    (void *)addr);
    if (cfi_warn) {
    __warn(core::ptr::null_mut(), 0, (void *)addr, 0, regs, core::ptr::null_mut());
    return BUG_TRAP_TYPE_WARN;
    }
    return BUG_TRAP_TYPE_BUG;
    }
//
// Declare two non-existent functions with types that match bpf_func_t and
// bpf_callback_t pointers, and use DEFINE_CFI_TYPE to define type hash
// variables for each function type. The cfi_bpf_* variables are used by
// arch-specific BPF JIT implementations to ensure indirectly callable JIT
// code has matching CFI type hashes.
//
    extern typeof(*(bpf_func_t)0) __bpf_prog_runX;
    DEFINE_CFI_TYPE(cfi_bpf_hash, __bpf_prog_runX);
    extern typeof(*(bpf_callback_t)0) __bpf_callback_fn;
    DEFINE_CFI_TYPE(cfi_bpf_subprog_hash, __bpf_callback_fn);

#[no_mangle]
pub unsafe extern "C" fn trap_address(p: *mut i32) -> c_ulong {
    static inline unsigned long trap_address(s32 *p)
    {
    return (unsigned long)((long)p + (long)*p);
    }
#[no_mangle]
unsafe extern "C" fn is_trap(addr: c_ulong, start: *mut i32, end: *mut i32) -> bool {
    static bool is_trap(unsigned long addr, s32 *start, s32 *end)
    {
    s32 *p;
    for (p = start; p < end; ++p) {
    if (trap_address(p) == addr)
    return true;
    }
    return false;
    }

// Populates `kcfi_trap(_end)?` fields in `struct module`.
    void module_cfi_finalize(const Elf_Ehdr *hdr, const Elf_Shdr *sechdrs,
    struct module *mod)
    {
    char *secstrings;
    unsigned int i;
    mod.kcfi_traps = core::ptr::null_mut();
    mod.kcfi_traps_end = core::ptr::null_mut();
    secstrings = (char *)hdr + sechdrs[hdr.e_shstrndx].sh_offset;
    for (i = 1; i < hdr.e_shnum; i++) {
    if (strcmp(secstrings + sechdrs[i].sh_name, "__kcfi_traps"))
    continue;
    mod.kcfi_traps = (s32 *)sechdrs[i].sh_addr;
    mod.kcfi_traps_end = (s32 *)(sechdrs[i].sh_addr + sechdrs[i].sh_size);
    break;
    }
    }
#[no_mangle]
unsafe extern "C" fn is_module_cfi_trap(addr: c_ulong) -> bool {
    static bool is_module_cfi_trap(unsigned long addr)
    {
    struct module *mod;
    let mut found: bool = false;
    guard(rcu)();
    mod = __module_address(addr);
    if (mod)
    found = is_trap(addr, mod.kcfi_traps, mod.kcfi_traps_end);
    return found;
    }

#[no_mangle]
pub unsafe extern "C" fn is_module_cfi_trap(addr: c_ulong) -> bool {
    static inline bool is_module_cfi_trap(unsigned long addr)
    {
    return false;
    }

    extern s32 __start___kcfi_traps[];
    extern s32 __stop___kcfi_traps[];
#[no_mangle]
pub unsafe extern "C" fn is_cfi_trap(addr: c_ulong) -> bool {
    bool is_cfi_trap(unsigned long addr)
    {
    if (is_trap(addr, __start___kcfi_traps, __stop___kcfi_traps))
    return true;
    return is_module_cfi_trap(addr);
    }
