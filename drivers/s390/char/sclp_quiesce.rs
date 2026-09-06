//! Automatically rewritten from C to Rust
//! Source: drivers/s390/char/sclp_quiesce.c
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
// signal quiesce handler
//
// Copyright IBM Corp. 1999, 2004
// Author(s): Martin Schwidefsky <schwidefsky@de.ibm.com>
// Peter Oberparleiter <peter.oberparleiter@de.ibm.com>
//

// Shutdown handler. Signal completion of shutdown by loading special PSW.
#[no_mangle]
unsafe extern "C" fn do_machine_quiesce() {
    static void do_machine_quiesce(void)
    {
    psw_t quiesce_psw;
    smp_send_stop();
    quiesce_psw.mask =
    PSW_MASK_BASE | PSW_MASK_EA | PSW_MASK_BA | PSW_MASK_WAIT;
    quiesce_psw.addr = 0xfff;
    __load_psw(quiesce_psw);
    }
// Handler for quiesce event. Start shutdown procedure.
#[no_mangle]
unsafe extern "C" fn sclp_quiesce_handler(evbuf: *mut evbuf_header) {
    static void sclp_quiesce_handler(struct evbuf_header *evbuf)
    {
    _machine_restart = (void *) do_machine_quiesce;
    _machine_halt = do_machine_quiesce;
    _machine_power_off = do_machine_quiesce;
    ctrl_alt_del();
    }
    static struct sclp_register sclp_quiesce_event = {
    .receive_mask = EVTYP_SIGQUIESCE_MASK,
    .receiver_fn = sclp_quiesce_handler,
    };
// Initialize quiesce driver.
#[no_mangle]
unsafe extern "C" fn sclp_quiesce_init() -> int __init {
    static int __init sclp_quiesce_init(void)
    {
    return sclp_register(&sclp_quiesce_event);
    }
    device_initcall(sclp_quiesce_init);
