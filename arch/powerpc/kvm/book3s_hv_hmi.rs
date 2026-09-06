//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/kvm/book3s_hv_hmi.c
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
// Hypervisor Maintenance Interrupt (HMI) handling.
//
// Copyright 2015 IBM Corporation
// Author: Mahesh Salgaonkar <mahesh@linux.vnet.ibm.com>
//

#[no_mangle]
pub unsafe extern "C" fn wait_for_subcore_guest_exit() {
    void wait_for_subcore_guest_exit(void)
    {
    int i;
//
// NULL bitmap pointer indicates that KVM module hasn't
// been loaded yet and hence no guests are running, or running
// on POWER9 or newer CPU.
//
// If no KVM is in use, no need to co-ordinate among threads
// as all of them will always be in host and no one is going
// to modify TB other than the opal hmi handler.
//
// POWER9 and newer don't need this synchronisation.
//
// Hence, just return from here.
//
    if (!local_paca.sibling_subcore_state)
    return;
    for (i = 0; i < MAX_SUBCORE_PER_CORE; i++)
    while (local_paca.sibling_subcore_state.in_guest[i])
    cpu_relax();
    }
#[no_mangle]
pub unsafe extern "C" fn wait_for_tb_resync() {
    void wait_for_tb_resync(void)
    {
    if (!local_paca.sibling_subcore_state)
    return;
    while (test_bit(CORE_TB_RESYNC_REQ_BIT,
    &local_paca.sibling_subcore_state.flags))
    cpu_relax();
    }
