//! Automatically rewritten from C to Rust
//! Source: tools/perf/arch/powerpc/util/skip-callchain-idx.c
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
// Use DWARF Debug information to skip unnecessary callchain entries.
//
// Copyright (C) 2014 Sukadev Bhattiprolu, IBM Corporation.
// Copyright (C) 2014 Ulrich Weigand, IBM Corporation.
//

//
// When saving the callchain on Power, the kernel conservatively saves
// excess entries in the callchain. A few of these entries are needed
// in some cases but not others. If the unnecessary entries are not
// ignored, we end up with duplicate arcs in the call-graphs. Use
// DWARF debug information to skip over any unnecessary callchain
// entries.
//
// See function header for arch_adjust_callchain() below for more details.
//
// The libdwfl code in this file is based on code from elfutils
// (libdwfl/argp-std.c, libdwfl/tests/addrcfi.c, etc).
//
// Use the DWARF expression for the Call-frame-address and determine
// if return address is in LR and if a new frame was allocated.
//
#[no_mangle]
unsafe extern "C" fn check_return_reg(ra_regno: c_int, frame: *mut Dwarf_Frame) -> c_int {
    static int check_return_reg(int ra_regno, Dwarf_Frame *frame)
    {
    Dwarf_Op ops_mem[3];
    Dwarf_Op dummy;
    Dwarf_Op *ops = &dummy;
    size_t nops;
    int result;
    result = dwarf_frame_register(frame, ra_regno, ops_mem, &ops, &nops);
    if (result < 0) {
    pr_debug("dwarf_frame_register() %s\n", dwarf_errmsg(-1));
    return -1;
    }
//
// Check if return address is on the stack. If return address
// is in a register (typically R0), it is yet to be saved on
// the stack.
//
    if ((nops != 0 || ops != core::ptr::null_mut()) &&
    !(nops == 1 && ops[0].atom == DW_OP_regx &&
    ops[0].number2 == 0 && ops[0].offset == 0))
    return 0;
//
// Return address is in LR. Check if a frame was allocated
// but not-yet used.
//
    result = dwarf_frame_cfa(frame, &ops, &nops);
    if (result < 0) {
    pr_debug("dwarf_frame_cfa() returns %d, %s\n", result,
    dwarf_errmsg(-1));
    return -1;
    }
//
// If call frame address is in r1, no new frame was allocated.
//
    if (nops == 1 && ops[0].atom == DW_OP_bregx && ops[0].number == 1 &&
    ops[0].number2 == 0)
    return 1;
//
// A new frame was allocated but has not yet been used.
//
    return 2;
    }
//
// Get the DWARF frame from the .eh_frame section.
//
    static Dwarf_Frame *get_eh_frame(Dwfl_Module *mod, Dwarf_Addr pc)
    {
    int		result;
    Dwarf_Addr	bias;
    Dwarf_CFI	*cfi;
    Dwarf_Frame	*frame;
    cfi = dwfl_module_eh_cfi(mod, &bias);
    if (!cfi) {
    pr_debug("%s(): no CFI - %s\n", __func__, dwfl_errmsg(-1));
    return core::ptr::null_mut();
    }
    result = dwarf_cfi_addrframe(cfi, pc-bias, &frame);
    if (result) {
    pr_debug("%s(): %s\n", __func__, dwfl_errmsg(-1));
    return core::ptr::null_mut();
    }
    return frame;
    }
//
// Get the DWARF frame from the .debug_frame section.
//
    static Dwarf_Frame *get_dwarf_frame(Dwfl_Module *mod, Dwarf_Addr pc)
    {
    Dwarf_CFI       *cfi;
    Dwarf_Addr      bias;
    Dwarf_Frame     *frame;
    int             result;
    cfi = dwfl_module_dwarf_cfi(mod, &bias);
    if (!cfi) {
    pr_debug("%s(): no CFI - %s\n", __func__, dwfl_errmsg(-1));
    return core::ptr::null_mut();
    }
    result = dwarf_cfi_addrframe(cfi, pc-bias, &frame);
    if (result) {
    pr_debug("%s(): %s\n", __func__, dwfl_errmsg(-1));
    return core::ptr::null_mut();
    }
    return frame;
    }
//
// Return:
// 0 if return address for the program counter @pc is on stack
// 1 if return address is in LR and no new stack frame was allocated
// 2 if return address is in LR and a new frame was allocated (but not
// yet used)
// -1 in case of errors
//
#[no_mangle]
unsafe extern "C" fn check_return_addr(dso: *mut dso, mapped_pc: Dwarf_Addr) -> c_int {
    static int check_return_addr(struct dso *dso, Dwarf_Addr mapped_pc)
    {
    let mut rc: c_int = -1;
    Dwfl		*dwfl;
    Dwfl_Module	*mod;
    Dwarf_Frame	*frame;
    int		ra_regno;
    let mut start: Dwarf_Addr = mapped_pc;
    let mut end: Dwarf_Addr = mapped_pc;
    bool		signalp;
    dwfl = dso__libdw_dwfl(dso);
    if (!dwfl)
    return -1;
    mod = dwfl_addrmodule(dwfl, mapped_pc);
    if (!mod) {
    pr_debug("dwfl_addrmodule() failed, %s\n", dwarf_errmsg(-1));
    goto out;
    }
//
// To work with split debug info files (eg: glibc), check both
// .eh_frame and .debug_frame sections of the ELF header.
//
    frame = get_eh_frame(mod, mapped_pc);
    if (!frame) {
    frame = get_dwarf_frame(mod, mapped_pc);
    if (!frame)
    goto out;
    }
    ra_regno = dwarf_frame_info(frame, &start, &end, &signalp);
    if (ra_regno < 0) {
    pr_debug("Return address register unavailable: %s\n",
    dwarf_errmsg(-1));
    goto out;
    }
    rc = check_return_reg(ra_regno, frame);
    out:
    return rc;
    }
//
// The callchain saved by the kernel always includes the link register (LR).
//
// 0:	PERF_CONTEXT_USER
// 1:	Program counter (Next instruction pointer)
// 2:	LR value
// 3:	Caller's caller
// 4:	...
//
// The value in LR is only needed when it holds a return address. If the
// return address is on the stack, we should ignore the LR value.
//
// Further, when the return address is in the LR, if a new frame was just
// allocated but the LR was not saved into it, then the LR contains the
// caller, slot 4: contains the caller's caller and the contents of slot 3:
// (chain->ips[3]) is undefined and must be ignored.
//
// Use DWARF debug information to determine if any entries need to be skipped.
//
// Return:
// index:	of callchain entry that needs to be ignored (if any)
// -1	if no entry needs to be ignored or in case of errors
//
#[no_mangle]
pub unsafe extern "C" fn arch_skip_callchain_idx(thread: *mut thread, chain: *mut ip_callchain) -> c_int {
    int arch_skip_callchain_idx(struct thread *thread, struct ip_callchain *chain)
    {
    struct addr_location al;
    struct dso *dso = core::ptr::null_mut();
    int rc;
    u64 ip;
    let mut skip_slot: u64 = -1;
    if (!chain || chain.nr < 3)
    return skip_slot;
    addr_location__init(&al);
    ip = chain.ips[1];
    thread__find_symbol(thread, PERF_RECORD_MISC_USER, ip, &al);
    if (al.map)
    dso = map__dso(al.map);
    if (!dso) {
    pr_debug("%" PRIx64 " dso is core::ptr::null_mut()\n", ip);
    addr_location__exit(&al);
    return skip_slot;
    }
    rc = check_return_addr(dso, map__map_ip(al.map, ip));
    pr_debug("[DSO %s, sym %s, ip 0x%" PRIx64 "] rc %d\n",
    dso__long_name(dso), al.sym.name, ip, rc);
    if (rc == 0) {
//
// Return address on stack. Ignore LR value in callchain
//
    skip_slot = 2;
    } else if (rc == 2) {
//
// New frame allocated but return address still in LR.
// Ignore the caller's caller entry in callchain.
//
    skip_slot = 3;
    }
    addr_location__exit(&al);
    return skip_slot;
    }
