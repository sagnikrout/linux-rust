//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/kernel/static_call.c
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
pub unsafe extern "C" fn arch_static_call_transform(site: *mut c_void, tramp: *mut c_void, func: *mut c_void, tail: bool) {
    void arch_static_call_transform(void *site, void *tramp, void *func, bool tail)
    {
    int err;
    let mut is_ret0: bool = (func == __static_call_return0);
    let mut _tramp: c_ulong = (unsigned long)tramp;
    let mut _func: c_ulong = (unsigned long)func;
    let mut _ret0: c_ulong = _tramp + PPC_SCT_RET0;
    let mut is_short: bool = is_offset_in_branch_range((long)func - (long)(site ? : tramp));
    mutex_lock(&text_mutex);
    if (site && tail) {
    if (!func)
    err = patch_instruction(site, ppc_inst(PPC_RAW_BLR()));
#[no_mangle]
pub unsafe extern "C" fn if(_arg: is_ret0) -> else {
    else if (is_ret0)
    err = patch_branch(site, _ret0, 0);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: is_short) -> else {
    else if (is_short)
    err = patch_branch(site, _func, 0);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: tramp) -> else {
    else if (tramp)
    err = patch_branch(site, _tramp, 0);
    else
    err = 0;
    } else if (site) {
    if (!func)
    err = patch_instruction(site, ppc_inst(PPC_RAW_NOP()));
#[no_mangle]
pub unsafe extern "C" fn if(_arg: is_ret0) -> else {
    else if (is_ret0)
    err = patch_instruction(site, ppc_inst(PPC_RAW_LI(_R3, 0)));
#[no_mangle]
pub unsafe extern "C" fn if(_arg: is_short) -> else {
    else if (is_short)
    err = patch_branch(site, _func, BRANCH_SET_LINK);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: tramp) -> else {
    else if (tramp)
    err = patch_branch(site, _tramp, BRANCH_SET_LINK);
    else
    err = 0;
    } else if (tramp) {
    if (func && !is_short) {
    err = patch_ulong(tramp + PPC_SCT_DATA, _func);
    if (err)
    goto out;
    }
    if (!func)
    err = patch_instruction(tramp, ppc_inst(PPC_RAW_BLR()));
#[no_mangle]
pub unsafe extern "C" fn if(_arg: is_ret0) -> else {
    else if (is_ret0)
    err = patch_branch(tramp, _ret0, 0);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: is_short) -> else {
    else if (is_short)
    err = patch_branch(tramp, _func, 0);
    else
    err = patch_instruction(tramp, ppc_inst(PPC_RAW_NOP()));
    } else {
    err = 0;
    }
    out:
    mutex_unlock(&text_mutex);
    if (err)
    panic("%s: patching failed %pS at %pS\n", __func__, func, tramp);
    }
    EXPORT_SYMBOL_GPL(arch_static_call_transform);
