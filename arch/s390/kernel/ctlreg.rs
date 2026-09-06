//! Automatically rewritten from C to Rust
//! Source: arch/s390/kernel/ctlreg.c
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
// Copyright IBM Corp. 1999, 2023
//

//
// ctl_lock guards access to global control register contents which
// are kept in the control register save area within absolute lowcore
// at physical address zero.
//
    static DEFINE_SPINLOCK(system_ctl_lock);
#[no_mangle]
pub unsafe extern "C" fn system_ctlreg_lock() {
    void system_ctlreg_lock(void)
    __acquires(&system_ctl_lock)
    {
    spin_lock(&system_ctl_lock);
    }
#[no_mangle]
pub unsafe extern "C" fn system_ctlreg_unlock() {
    void system_ctlreg_unlock(void)
    __releases(&system_ctl_lock)
    {
    spin_unlock(&system_ctl_lock);
    }
    static bool system_ctlreg_area_init __ro_after_init;
#[no_mangle]
pub unsafe extern "C" fn system_ctlreg_init_save_area(lc: *mut lowcore) -> void __init {
    void __init system_ctlreg_init_save_area(struct lowcore *lc)
    {
    struct lowcore *abs_lc;
    abs_lc = get_abs_lowcore();
    __local_ctl_store(0, 15, lc.cregs_save_area);
    __local_ctl_store(0, 15, abs_lc.cregs_save_area);
    put_abs_lowcore(abs_lc);
    system_ctlreg_area_init = true;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctlreg_parms {
    pub andval: c_ulong,
    pub orval: c_ulong,
    pub val: c_ulong,
    pub request: c_int,
    pub cr: c_int,
}

#[no_mangle]
unsafe extern "C" fn ctlreg_callback(info: *mut c_void) {
    static void ctlreg_callback(void *info)
    {
    struct ctlreg_parms *pp = info;
    struct ctlreg regs[16];
    __local_ctl_store(0, 15, regs);
    if (pp.request == CTLREG_LOAD) {
    regs[pp.cr].val = pp.val;
    } else {
    regs[pp.cr].val &= pp.andval;
    regs[pp.cr].val |= pp.orval;
    }
    __local_ctl_load(0, 15, regs);
    }
#[no_mangle]
unsafe extern "C" fn system_ctlreg_update(info: *mut c_void) {
    static void system_ctlreg_update(void *info)
    {
    unsigned long flags;
    if (system_state == SYSTEM_BOOTING) {
//
// For very early calls do not call on_each_cpu()
// since not everything might be setup.
//
    local_irq_save(flags);
    ctlreg_callback(info);
    local_irq_restore(flags);
    } else {
    on_each_cpu(ctlreg_callback, info, 1);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn system_ctlreg_modify(cr: c_uint, data: c_ulong, request: c_int) {
    void system_ctlreg_modify(unsigned int cr, unsigned long data, int request)
    {
    let mut pp: ctlreg_parms = { .cr = cr, .request = request, };
    struct lowcore *abs_lc;
    switch (request) {
    case CTLREG_SET_BIT:
    pp.orval  = 1UL << data;
    pp.andval = -1UL;
    break;
    case CTLREG_CLEAR_BIT:
    pp.orval  = 0;
    pp.andval = ~(1UL << data);
    break;
    case CTLREG_LOAD:
    pp.val = data;
    break;
    }
    if (system_ctlreg_area_init) {
    system_ctlreg_lock();
    abs_lc = get_abs_lowcore();
    if (request == CTLREG_LOAD) {
    abs_lc.cregs_save_area[cr].val = pp.val;
    } else {
    abs_lc.cregs_save_area[cr].val &= pp.andval;
    abs_lc.cregs_save_area[cr].val |= pp.orval;
    }
    put_abs_lowcore(abs_lc);
    system_ctlreg_update(&pp);
    system_ctlreg_unlock();
    } else {
    system_ctlreg_update(&pp);
    }
    }
    EXPORT_SYMBOL(system_ctlreg_modify);
