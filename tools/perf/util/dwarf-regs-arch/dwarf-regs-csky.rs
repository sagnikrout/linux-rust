//! Automatically rewritten from C to Rust
//! Source: tools/perf/util/dwarf-regs-arch/dwarf-regs-csky.c
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
// Copyright (C) 2019 Hangzhou C-SKY Microsystems co.,ltd.
// Mapping of DWARF debug register numbers into register names.

// Ensure the V2 perf reg definitions are included.

pub const __CSKYABIV2__: c_int = 1;

pub const CSKY_ABIV2_MAX_REGS: c_int = 73;
    static const char * const csky_dwarf_regs_table_abiv2[CSKY_ABIV2_MAX_REGS] = {
// r0 ~ r8
    "%a0", "%a1", "%a2", "%a3", "%regs0", "%regs1", "%regs2", "%regs3",
// r9 ~ r15
    "%regs4", "%regs5", "%regs6", "%regs7", "%regs8", "%regs9", "%sp",
    "%lr",
// r16 ~ r23
    "%exregs0", "%exregs1", "%exregs2", "%exregs3", "%exregs4",
    "%exregs5", "%exregs6", "%exregs7",
// r24 ~ r31
    "%exregs8", "%exregs9", "%exregs10", "%exregs11", "%exregs12",
    "%exregs13", "%exregs14", "%tls",
    "%pc", core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(), "%hi", "%lo", core::ptr::null_mut(), core::ptr::null_mut(),
    core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(),
    core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(),
    core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(),
    core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(),
    "%epc",
    };
pub const CSKY_ABIV1_MAX_REGS: c_int = 57;
    static const char * const csky_dwarf_regs_table_abiv1[CSKY_ABIV1_MAX_REGS] = {
// r0 ~ r8
    "%sp", "%regs9", "%a0", "%a1", "%a2", "%a3", "%regs0", "%regs1",
// r9 ~ r15
    "%regs2", "%regs3", "%regs4", "%regs5", "%regs6", "%regs7", "%regs8",
    "%lr",
    core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(),
    core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(),
    core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(),
    core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(),
    core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(),
    "%epc",
    };
    const char *__get_csky_regstr(unsigned int n, unsigned int flags)
    {
    if (flags & EF_CSKY_ABIV2)
    return (n < CSKY_ABIV2_MAX_REGS) ? csky_dwarf_regs_table_abiv2[n] : core::ptr::null_mut();
    return (n < CSKY_ABIV1_MAX_REGS) ? csky_dwarf_regs_table_abiv1[n] : core::ptr::null_mut();
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
#[no_mangle]
pub unsafe extern "C" fn __get_csky_regnum(name: *const c_char, flags: c_uint) -> c_int {
    int __get_csky_regnum(const char *name, unsigned int flags)
    {
    if (flags & EF_CSKY_ABIV2)
    return __get_dwarf_regnum(csky_dwarf_regs_table_abiv2, CSKY_ABIV2_MAX_REGS, name);
    return __get_dwarf_regnum(csky_dwarf_regs_table_abiv1, CSKY_ABIV1_MAX_REGS, name);
    }
#[no_mangle]
pub unsafe extern "C" fn __get_dwarf_regnum_for_perf_regnum_csky(perf_regnum: c_int, flags: c_uint) -> c_int {
    int __get_dwarf_regnum_for_perf_regnum_csky(int perf_regnum, unsigned int flags)
    {
    static const int dwarf_csky_regnums[][2] = {
    [PERF_REG_CSKY_TLS] = {-ENOENT, 31},
    [PERF_REG_CSKY_LR] = {15, 15},
    [PERF_REG_CSKY_PC] = {-ENOENT, 32},
// TODO: PERF_REG_CSKY_SR
    [PERF_REG_CSKY_SP] = {0, 14},
// TODO: PERF_REG_CSKY_ORIG_A0
    [PERF_REG_CSKY_A0] = {2, 0},
    [PERF_REG_CSKY_A1] = {3, 1},
    [PERF_REG_CSKY_A2] = {4, 2},
    [PERF_REG_CSKY_A3] = {5, 3},
    [PERF_REG_CSKY_REGS0] = {6, 4},
    [PERF_REG_CSKY_REGS1] = {7, 5},
    [PERF_REG_CSKY_REGS2] = {8, 6},
    [PERF_REG_CSKY_REGS3] = {9, 7},
    [PERF_REG_CSKY_REGS4] = {10, 8},
    [PERF_REG_CSKY_REGS5] = {11, 9},
    [PERF_REG_CSKY_REGS6] = {12, 10},
    [PERF_REG_CSKY_REGS7] = {13, 11},
    [PERF_REG_CSKY_REGS8] = {14, 12},
    [PERF_REG_CSKY_REGS9] = {1, 13},
    [PERF_REG_CSKY_EXREGS0] = {-ENOENT, 16},
    [PERF_REG_CSKY_EXREGS1] = {-ENOENT, 17},
    [PERF_REG_CSKY_EXREGS2] = {-ENOENT, 18},
    [PERF_REG_CSKY_EXREGS3] = {-ENOENT, 19},
    [PERF_REG_CSKY_EXREGS4] = {-ENOENT, 20},
    [PERF_REG_CSKY_EXREGS5] = {-ENOENT, 21},
    [PERF_REG_CSKY_EXREGS6] = {-ENOENT, 22},
    [PERF_REG_CSKY_EXREGS7] = {-ENOENT, 23},
    [PERF_REG_CSKY_EXREGS8] = {-ENOENT, 24},
    [PERF_REG_CSKY_EXREGS9] = {-ENOENT, 25},
    [PERF_REG_CSKY_EXREGS10] = {-ENOENT, 26},
    [PERF_REG_CSKY_EXREGS11] = {-ENOENT, 27},
    [PERF_REG_CSKY_EXREGS12] = {-ENOENT, 28},
    [PERF_REG_CSKY_EXREGS13] = {-ENOENT, 29},
    [PERF_REG_CSKY_EXREGS14] = {-ENOENT, 30},
// TODO: PERF_REG_CSKY_HI
// TODO: PERF_REG_CSKY_LO
// TODO: PERF_REG_CSKY_DCSR
    };
    let mut idx: c_int = 0;
    if (flags & EF_CSKY_ABIV2)
    idx++;
    if (perf_regnum <  0 || perf_regnum > (int)ARRAY_SIZE(dwarf_csky_regnums) ||
    dwarf_csky_regnums[perf_regnum][idx] == 0)
    return -ENOENT;
    return dwarf_csky_regnums[perf_regnum][idx];
    }
