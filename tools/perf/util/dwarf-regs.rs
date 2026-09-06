//! Automatically rewritten from C to Rust
//! Source: tools/perf/util/dwarf-regs.c
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
// dwarf-regs.c : Mapping of DWARF debug register numbers into register names.
//
// Written by: Masami Hiramatsu <mhiramat@kernel.org>
//

// Define const char * {arch}_register_tbl[]
// Macro flag: #define DEFINE_DWARF_REGSTR_TABLE

// Return architecture dependent register string (for kprobe-tracer)
    const char *get_dwarf_regstr(unsigned int n, unsigned int machine, unsigned int flags)
    {

    if (machine == EM_NONE) {
// Generic arch - use host arch
    machine = EM_HOST;
    }
    switch (machine) {
    case EM_386:
    return __get_dwarf_regstr(x86_32_regstr_tbl, n);
    case EM_X86_64:
    return __get_dwarf_regstr(x86_64_regstr_tbl, n);
    case EM_ARM:
    return __get_dwarf_regstr(arm_regstr_tbl, n);
    case EM_AARCH64:
    return __get_dwarf_regstr(aarch64_regstr_tbl, n);
    case EM_CSKY:
    return __get_csky_regstr(n, flags);
    case EM_SH:
    return __get_dwarf_regstr(sh_regstr_tbl, n);
    case EM_S390:
    return __get_dwarf_regstr(s390_regstr_tbl, n);
    case EM_PPC:
    case EM_PPC64:
    return __get_dwarf_regstr(powerpc_regstr_tbl, n);
    case EM_RISCV:
    return __get_dwarf_regstr(riscv_regstr_tbl, n);
    case EM_SPARC:
    case EM_SPARCV9:
    return __get_dwarf_regstr(sparc_regstr_tbl, n);
    case EM_XTENSA:
    return __get_dwarf_regstr(xtensa_regstr_tbl, n);
    case EM_MIPS:
    return __get_dwarf_regstr(mips_regstr_tbl, n);
    case EM_LOONGARCH:
    return __get_dwarf_regstr(loongarch_regstr_tbl, n);
    default:
    pr_err("ELF MACHINE %x is not supported.\n", machine);
    }
    return core::ptr::null_mut();

    }
#[no_mangle]
unsafe extern "C" fn __get_dwarf_regnum(regstr: *const *const c_char, num_regstr: usize, name: *const c_char) -> c_int {
    static int __get_dwarf_regnum(const char *const *regstr, size_t num_regstr, const char *name)
    {
    for (size_t i = 0; i < num_regstr; i++) {
    if (regstr[i] && !strcmp(regstr[i], name))
    return i;
    }
    return -ENOENT;
    }
// Return DWARF register number from architecture register name
#[no_mangle]
pub unsafe extern "C" fn get_dwarf_regnum(name: *const c_char, machine: c_uint, flags: c_uint) -> c_int {
    int get_dwarf_regnum(const char *name, unsigned int machine, unsigned int flags)
    {
    char *regname = strdup(name);
    let mut reg: c_int = -1;
    char *p;

    if (regname == core::ptr::null_mut())
    return -EINVAL;
// For convenience, remove trailing characters
    p = strpbrk(regname, " ,)");
    if (p)
// p = '\0';
    if (machine == EM_NONE) {
// Generic arch - use host arch
    machine = EM_HOST;
    }
    switch (machine) {
    case EM_X86_64:
    reg = __get_dwarf_regnum_x86_64(name);
    break;
    case EM_386:
    reg = __get_dwarf_regnum_i386(name);
    break;
    case EM_ARM:
    reg = _get_dwarf_regnum(arm_regstr_tbl, name);
    break;
    case EM_AARCH64:
    reg = _get_dwarf_regnum(aarch64_regstr_tbl, name);
    break;
    case EM_CSKY:
    reg = __get_csky_regnum(name, flags);
    break;
    case EM_SH:
    reg = _get_dwarf_regnum(sh_regstr_tbl, name);
    break;
    case EM_S390:
    reg = _get_dwarf_regnum(s390_regstr_tbl, name);
    break;
    case EM_PPC:
    case EM_PPC64:
    reg = _get_dwarf_regnum(powerpc_regstr_tbl, name);
    break;
    case EM_RISCV:
    reg = _get_dwarf_regnum(riscv_regstr_tbl, name);
    break;
    case EM_SPARC:
    case EM_SPARCV9:
    reg = _get_dwarf_regnum(sparc_regstr_tbl, name);
    break;
    case EM_XTENSA:
    reg = _get_dwarf_regnum(xtensa_regstr_tbl, name);
    break;
    case EM_MIPS:
    reg = _get_dwarf_regnum(mips_regstr_tbl, name);
    break;
    case EM_LOONGARCH:
    reg = _get_dwarf_regnum(loongarch_regstr_tbl, name);
    break;
    default:
    pr_err("ELF MACHINE %x is not supported.\n", machine);
    }
    free(regname);
    return reg;

    }
#[no_mangle]
unsafe extern "C" fn get_libdw_frame_nregs(machine: c_uint, __maybe_unused: unsigned int flags) -> c_int {
    static int get_libdw_frame_nregs(unsigned int machine, unsigned int flags __maybe_unused)
    {
    switch (machine) {
    case EM_X86_64:
    return 17;
    case EM_386:
    return 9;
    case EM_ARM:
    return 16;
    case EM_AARCH64:
    return 97;
    case EM_CSKY:
    return 38;
    case EM_S390:
    return 32;
    case EM_PPC:
    case EM_PPC64:
    return 145;
    case EM_RISCV:
    return 66;
    case EM_SPARC:
    case EM_SPARCV9:
    return 103;
    case EM_LOONGARCH:
    return 74;
    case EM_MIPS:
    return 71;
    default:
    return 0;
    }
    }
    int get_dwarf_regnum_for_perf_regnum(int perf_regnum, unsigned int machine,
    unsigned int flags, bool only_libdw_supported)
    {
    int reg;
    switch (machine) {
    case EM_X86_64:
    reg = __get_dwarf_regnum_for_perf_regnum_x86_64(perf_regnum);
    break;
    case EM_386:
    reg = __get_dwarf_regnum_for_perf_regnum_i386(perf_regnum);
    break;
    case EM_ARM:
    reg = __get_dwarf_regnum_for_perf_regnum_arm(perf_regnum);
    break;
    case EM_AARCH64:
    reg = __get_dwarf_regnum_for_perf_regnum_arm64(perf_regnum);
    break;
    case EM_CSKY:
    reg = __get_dwarf_regnum_for_perf_regnum_csky(perf_regnum, flags);
    break;
    case EM_PPC:
    case EM_PPC64:
    reg = __get_dwarf_regnum_for_perf_regnum_powerpc(perf_regnum);
    break;
    case EM_RISCV:
    reg = __get_dwarf_regnum_for_perf_regnum_riscv(perf_regnum);
    break;
    case EM_S390:
    reg = __get_dwarf_regnum_for_perf_regnum_s390(perf_regnum);
    break;
    case EM_LOONGARCH:
    reg = __get_dwarf_regnum_for_perf_regnum_loongarch(perf_regnum);
    break;
    case EM_MIPS:
    reg = __get_dwarf_regnum_for_perf_regnum_mips(perf_regnum);
    break;
    default:
    pr_err("ELF MACHINE %x is not supported.\n", machine);
    return -ENOENT;
    }
    if (reg >= 0 && only_libdw_supported) {
    let mut nregs: c_int = get_libdw_frame_nregs(machine, flags);
    if (reg >= nregs)
    reg = -ENOENT;
    }
    return reg;
    }
