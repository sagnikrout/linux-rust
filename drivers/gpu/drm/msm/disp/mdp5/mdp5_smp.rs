//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/msm/disp/mdp5/mdp5_smp.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (c) 2014, The Linux Foundation. All rights reserved.
// Copyright (C) 2013 Red Hat
// Author: Rob Clark <robdclark@gmail.com>
//

//
// SMP - Shared Memory Pool:
//
// SMP blocks are shared between all the clients, where each plane in
// a scanout buffer is a SMP client.  Ie. scanout of 3 plane I420 on
// pipe VIG0 => 3 clients: VIG0_Y, VIG0_CB, VIG0_CR.
//
// Based on the size of the attached scanout buffer, a certain # of
// blocks must be allocated to that client out of the shared pool.
//
// In some hw, some blocks are statically allocated for certain pipes
// and CANNOT be re-allocated (eg: MMB0 and MMB1 both tied to RGB0).
//
// Atomic SMP State:
//
// On atomic updates that modify SMP configuration, the state is cloned
// (copied) and modified.  For test-only, or in cases where atomic
// update fails (or if we hit ww_mutex deadlock/backoff condition) the
// new state is simply thrown away.
//
// Because the SMP registers are not double buffered, updates are a
// two step process:
//
// 1) in _prepare_commit() we configure things (via read-modify-write)
// for the newly assigned pipes, so we don't take away blocks
// assigned to pipes that are still scanning out
// 2) in _complete_commit(), after vblank/etc, we clear things for the
// released clients, since at that point old pipes are no longer
// scanning out.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdp5_smp_state {
// global state of what blocks are in use:
    pub state: mdp5_smp_state_t,
// per client state of what blocks they are using:
    pub client_state: [mdp5_smp_state_t; MAX_CLIENTS],
// assigned pipes (hw updated at _prepare_commit()):
    pub assigned: c_ulong,
// released pipes (hw updated at _complete_commit()):
    pub released: c_ulong,
}

//
// SMP module prototypes:
// mdp5_smp_init() returns a SMP @handler,
// which is then used to call the other mdp5_smp_*(handler, ...) functions.
//
extern "C" {
    pub fn mdp5_smp_prepare_commit(smp: *mut mdp5_smp, state: *mut mdp5_smp_state);
}
extern "C" {
    pub fn mdp5_smp_complete_commit(smp: *mut mdp5_smp, state: *mut mdp5_smp_state);
}
