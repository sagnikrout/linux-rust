//! Automatically rewritten from C to Rust
//! Source: arch/s390/kernel/fpu.c
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
// In-kernel vector facility support functions
//
// Copyright IBM Corp. 2015
// Author(s): Hendrik Brueckner <brueckner@linux.vnet.ibm.com>
//

#[no_mangle]
pub unsafe extern "C" fn __kernel_fpu_begin(state: *mut kernel_fpu, flags: c_int) {
    void __kernel_fpu_begin(struct kernel_fpu *state, int flags)
    {
    __vector128 *vxrs = state.vxrs;
    int mask;
//
// Limit the save to the FPU/vector registers already
// in use by the previous context.
//
    flags &= state.hdr.mask;
    if (flags & KERNEL_FPC)
    fpu_stfpc(&state.hdr.fpc);
    if (!cpu_has_vx()) {
    if (flags & KERNEL_VXR_LOW)
    save_fp_regs_vx(vxrs);
    return;
    }
    mask = flags & KERNEL_VXR;
    if (mask == KERNEL_VXR) {
    vxrs += fpu_vstm(0, 15, vxrs);
    vxrs += fpu_vstm(16, 31, vxrs);
    return;
    }
    if (mask == KERNEL_VXR_MID) {
    vxrs += fpu_vstm(8, 23, vxrs);
    return;
    }
    mask = flags & KERNEL_VXR_LOW;
    if (mask) {
    if (mask == KERNEL_VXR_LOW)
    vxrs += fpu_vstm(0, 15, vxrs);
#[no_mangle]
pub unsafe extern "C" fn if(KERNEL_VXR_V0V7: mask ==) -> else {
    else if (mask == KERNEL_VXR_V0V7)
    vxrs += fpu_vstm(0, 7, vxrs);
    else
    vxrs += fpu_vstm(8, 15, vxrs);
    }
    mask = flags & KERNEL_VXR_HIGH;
    if (mask) {
    if (mask == KERNEL_VXR_HIGH)
    vxrs += fpu_vstm(16, 31, vxrs);
#[no_mangle]
pub unsafe extern "C" fn if(KERNEL_VXR_V16V23: mask ==) -> else {
    else if (mask == KERNEL_VXR_V16V23)
    vxrs += fpu_vstm(16, 23, vxrs);
    else
    vxrs += fpu_vstm(24, 31, vxrs);
    }
    }
    EXPORT_SYMBOL(__kernel_fpu_begin);
#[no_mangle]
pub unsafe extern "C" fn __kernel_fpu_end(state: *mut kernel_fpu, flags: c_int) {
    void __kernel_fpu_end(struct kernel_fpu *state, int flags)
    {
    __vector128 *vxrs = state.vxrs;
    int mask;
//
// Limit the restore to the FPU/vector registers of the
// previous context that have been overwritten by the
// current context.
//
    flags &= state.hdr.mask;
    if (flags & KERNEL_FPC)
    fpu_lfpc(&state.hdr.fpc);
    if (!cpu_has_vx()) {
    if (flags & KERNEL_VXR_LOW)
    load_fp_regs_vx(vxrs);
    return;
    }
    mask = flags & KERNEL_VXR;
    if (mask == KERNEL_VXR) {
    vxrs += fpu_vlm(0, 15, vxrs);
    vxrs += fpu_vlm(16, 31, vxrs);
    return;
    }
    if (mask == KERNEL_VXR_MID) {
    vxrs += fpu_vlm(8, 23, vxrs);
    return;
    }
    mask = flags & KERNEL_VXR_LOW;
    if (mask) {
    if (mask == KERNEL_VXR_LOW)
    vxrs += fpu_vlm(0, 15, vxrs);
#[no_mangle]
pub unsafe extern "C" fn if(KERNEL_VXR_V0V7: mask ==) -> else {
    else if (mask == KERNEL_VXR_V0V7)
    vxrs += fpu_vlm(0, 7, vxrs);
    else
    vxrs += fpu_vlm(8, 15, vxrs);
    }
    mask = flags & KERNEL_VXR_HIGH;
    if (mask) {
    if (mask == KERNEL_VXR_HIGH)
    vxrs += fpu_vlm(16, 31, vxrs);
#[no_mangle]
pub unsafe extern "C" fn if(KERNEL_VXR_V16V23: mask ==) -> else {
    else if (mask == KERNEL_VXR_V16V23)
    vxrs += fpu_vlm(16, 23, vxrs);
    else
    vxrs += fpu_vlm(24, 31, vxrs);
    }
    }
    EXPORT_SYMBOL(__kernel_fpu_end);
#[no_mangle]
pub unsafe extern "C" fn load_fpu_state(state: *mut fpu, flags: c_int) {
    void load_fpu_state(struct fpu *state, int flags)
    {
    __vector128 *vxrs = &state.vxrs[0];
    int mask;
    if (flags & KERNEL_FPC)
    fpu_lfpc_safe(&state.fpc);
    if (!cpu_has_vx()) {
    if (flags & KERNEL_VXR_V0V7)
    load_fp_regs_vx(state.vxrs);
    return;
    }
    mask = flags & KERNEL_VXR;
    if (mask == KERNEL_VXR) {
    fpu_vlm(0, 15, &vxrs[0]);
    fpu_vlm(16, 31, &vxrs[16]);
    return;
    }
    if (mask == KERNEL_VXR_MID) {
    fpu_vlm(8, 23, &vxrs[8]);
    return;
    }
    mask = flags & KERNEL_VXR_LOW;
    if (mask) {
    if (mask == KERNEL_VXR_LOW)
    fpu_vlm(0, 15, &vxrs[0]);
#[no_mangle]
pub unsafe extern "C" fn if(KERNEL_VXR_V0V7: mask ==) -> else {
    else if (mask == KERNEL_VXR_V0V7)
    fpu_vlm(0, 7, &vxrs[0]);
    else
    fpu_vlm(8, 15, &vxrs[8]);
    }
    mask = flags & KERNEL_VXR_HIGH;
    if (mask) {
    if (mask == KERNEL_VXR_HIGH)
    fpu_vlm(16, 31, &vxrs[16]);
#[no_mangle]
pub unsafe extern "C" fn if(KERNEL_VXR_V16V23: mask ==) -> else {
    else if (mask == KERNEL_VXR_V16V23)
    fpu_vlm(16, 23, &vxrs[16]);
    else
    fpu_vlm(24, 31, &vxrs[24]);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn save_fpu_state(state: *mut fpu, flags: c_int) {
    void save_fpu_state(struct fpu *state, int flags)
    {
    __vector128 *vxrs = &state.vxrs[0];
    int mask;
    if (flags & KERNEL_FPC)
    fpu_stfpc(&state.fpc);
    if (!cpu_has_vx()) {
    if (flags & KERNEL_VXR_LOW)
    save_fp_regs_vx(state.vxrs);
    return;
    }
    mask = flags & KERNEL_VXR;
    if (mask == KERNEL_VXR) {
    fpu_vstm(0, 15, &vxrs[0]);
    fpu_vstm(16, 31, &vxrs[16]);
    return;
    }
    if (mask == KERNEL_VXR_MID) {
    fpu_vstm(8, 23, &vxrs[8]);
    return;
    }
    mask = flags & KERNEL_VXR_LOW;
    if (mask) {
    if (mask == KERNEL_VXR_LOW)
    fpu_vstm(0, 15, &vxrs[0]);
#[no_mangle]
pub unsafe extern "C" fn if(KERNEL_VXR_V0V7: mask ==) -> else {
    else if (mask == KERNEL_VXR_V0V7)
    fpu_vstm(0, 7, &vxrs[0]);
    else
    fpu_vstm(8, 15, &vxrs[8]);
    }
    mask = flags & KERNEL_VXR_HIGH;
    if (mask) {
    if (mask == KERNEL_VXR_HIGH)
    fpu_vstm(16, 31, &vxrs[16]);
#[no_mangle]
pub unsafe extern "C" fn if(KERNEL_VXR_V16V23: mask ==) -> else {
    else if (mask == KERNEL_VXR_V16V23)
    fpu_vstm(16, 23, &vxrs[16]);
    else
    fpu_vstm(24, 31, &vxrs[24]);
    }
    }
    EXPORT_SYMBOL(save_fpu_state);
