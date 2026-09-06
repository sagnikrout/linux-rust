//! Automatically rewritten from C to Rust
//! Source: arch/s390/kernel/alternative.c
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

    unsigned long __bootdata_preserved(machine_features[1]);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct alt_debug {
    pub BITS_PER_LONG]: unsigned long facilities[MAX_FACILITY_BIT /,
    pub BITS_PER_LONG]: unsigned long mfeatures[MAX_MFEATURE_BIT /,
    pub spec: c_int,
}

    static struct alt_debug __bootdata_preserved(alt_debug);
#[no_mangle]
unsafe extern "C" fn alternative_dump(old: *mut u8, new: *mut u8, len: c_uint, type: c_uint, data: c_uint) {
    static void alternative_dump(u8 *old, u8 *new, unsigned int len, unsigned int type, unsigned int data)
    {
    char oinsn[33], ninsn[33];
    unsigned long kptr;
    unsigned int pos;
    for (pos = 0; pos < len && 2 * pos < sizeof(oinsn) - 3; pos++)
    hex_byte_pack(&oinsn[2 * pos], old[pos]);
    oinsn[2 * pos] = 0;
    for (pos = 0; pos < len && 2 * pos < sizeof(ninsn) - 3; pos++)
    hex_byte_pack(&ninsn[2 * pos], new[pos]);
    ninsn[2 * pos] = 0;
    kptr = (unsigned long)__kernel_va(old);
    a_debug("[%d/%3d] %016lx: %s . %s\n", type, data, kptr, oinsn, ninsn);
    }
#[no_mangle]
pub unsafe extern "C" fn __apply_alternatives(start: *mut alt_instr, end: *mut alt_instr, ctx: c_uint) {
    void __apply_alternatives(struct alt_instr *start, struct alt_instr *end, unsigned int ctx)
    {
    struct alt_debug *d;
    struct alt_instr *a;
    bool debug, replace;
    u8 *old, *new;
//
// The scan order should be from start to end. A later scanned
// alternative code can overwrite previously scanned alternative code.
//
    d = &alt_debug;
    for (a = start; a < end; a++) {
    if (!(a.ctx & ctx))
    continue;
    switch (a.type) {
    case ALT_TYPE_FACILITY:
    replace = test_facility(a.data);
    debug = __test_facility(a.data, d.facilities);
    break;
    case ALT_TYPE_FEATURE:
    replace = test_machine_feature(a.data);
    debug = __test_machine_feature(a.data, d.mfeatures);
    break;
    case ALT_TYPE_SPEC:
    replace = nobp_enabled();
    debug = d.spec;
    break;
    default:
    replace = false;
    debug = false;
    }
    if (!replace)
    continue;
    old = (u8 *)&a.instr_offset + a.instr_offset;
    new = (u8 *)&a.repl_offset + a.repl_offset;
    if (debug)
    alternative_dump(old, new, a.instrlen, a.type, a.data);
    s390_kernel_write(old, new, a.instrlen);
    }
    }
