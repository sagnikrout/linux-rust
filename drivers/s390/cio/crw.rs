//! Automatically rewritten from C to Rust
//! Source: drivers/s390/cio/crw.c
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
// Channel report handling code
//
// Copyright IBM Corp. 2000, 2009
// Author(s): Ingo Adlung <adlung@de.ibm.com>,
// Martin Schwidefsky <schwidefsky@de.ibm.com>,
// Cornelia Huck <cornelia.huck@de.ibm.com>,
//

    static DEFINE_MUTEX(crw_handler_mutex);
    static crw_handler_t crw_handlers[NR_RSCS];
    let mut crw_nr_req: static atomic_t = ATOMIC_INIT(0);
    static DECLARE_WAIT_QUEUE_HEAD(crw_handler_wait_q);
//
// crw_register_handler() - register a channel report word handler
// @rsc: reporting source code to handle
// @handler: handler to be registered
//
// Returns %0 on success and a negative error value otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn crw_register_handler(rsc: c_int, handler: crw_handler_t) -> c_int {
    int crw_register_handler(int rsc, crw_handler_t handler)
    {
    let mut rc: c_int = 0;
    if ((rsc < 0) || (rsc >= NR_RSCS))
    return -EINVAL;
    mutex_lock(&crw_handler_mutex);
    if (crw_handlers[rsc])
    rc = -EBUSY;
    else
    crw_handlers[rsc] = handler;
    mutex_unlock(&crw_handler_mutex);
    return rc;
    }
//
// crw_unregister_handler() - unregister a channel report word handler
// @rsc: reporting source code to handle
//
#[no_mangle]
pub unsafe extern "C" fn crw_unregister_handler(rsc: c_int) {
    void crw_unregister_handler(int rsc)
    {
    if ((rsc < 0) || (rsc >= NR_RSCS))
    return;
    mutex_lock(&crw_handler_mutex);
    crw_handlers[rsc] = core::ptr::null_mut();
    mutex_unlock(&crw_handler_mutex);
    }
//
// Retrieve CRWs and call function to handle event.
//
#[no_mangle]
unsafe extern "C" fn crw_collect_info(unused: *mut c_void) -> c_int {
    static int crw_collect_info(void *unused)
    {
    struct crw crw[2];
    int ccode, signal;
    unsigned int chain;
    repeat:
    signal = wait_event_interruptible(crw_handler_wait_q,
    atomic_read(&crw_nr_req) > 0);
    if (unlikely(signal))
    atomic_inc(&crw_nr_req);
    chain = 0;
    while (1) {
    crw_handler_t handler;
    if (unlikely(chain > 1)) {
    struct crw tmp_crw;
    printk(KERN_WARNING "%s: Code does not support more than two chained crws\n",
    __func__);
    ccode = stcrw(&tmp_crw);
    printk(KERN_WARNING"%s: crw reports slct=%d, oflw=%d, "
    "chn=%d, rsc=%X, anc=%d, erc=%X, rsid=%X\n",
    __func__, tmp_crw.slct, tmp_crw.oflw,
    tmp_crw.chn, tmp_crw.rsc, tmp_crw.anc,
    tmp_crw.erc, tmp_crw.rsid);
    printk(KERN_WARNING"%s: This was crw number %x in the "
    "chain\n", __func__, chain);
    if (ccode != 0)
    break;
    chain = tmp_crw.chn ? chain + 1 : 0;
    continue;
    }
    ccode = stcrw(&crw[chain]);
    if (ccode != 0)
    break;
    printk(KERN_DEBUG "crw_info : CRW reports slct=%d, oflw=%d, "
    "chn=%d, rsc=%X, anc=%d, erc=%X, rsid=%X\n",
    crw[chain].slct, crw[chain].oflw, crw[chain].chn,
    crw[chain].rsc, crw[chain].anc, crw[chain].erc,
    crw[chain].rsid);
// Check for overflows.
    if (crw[chain].oflw) {
    int i;
    pr_debug("%s: crw overflow detected!\n", __func__);
    mutex_lock(&crw_handler_mutex);
    for (i = 0; i < NR_RSCS; i++) {
    if (crw_handlers[i])
    crw_handlers[i](core::ptr::null_mut(), core::ptr::null_mut(), 1);
    }
    mutex_unlock(&crw_handler_mutex);
    chain = 0;
    continue;
    }
    if (crw[0].chn && !chain) {
    chain++;
    continue;
    }
    mutex_lock(&crw_handler_mutex);
    handler = crw_handlers[crw[chain].rsc];
    if (handler)
    handler(&crw[0], chain ? &crw[1] : core::ptr::null_mut(), 0);
    mutex_unlock(&crw_handler_mutex);
// chain is always 0 or 1 here.
    chain = crw[chain].chn ? chain + 1 : 0;
    }
    if (atomic_dec_and_test(&crw_nr_req))
    wake_up(&crw_handler_wait_q);
    goto repeat;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn crw_handle_channel_report() {
    void crw_handle_channel_report(void)
    {
    atomic_inc(&crw_nr_req);
    wake_up(&crw_handler_wait_q);
    }
#[no_mangle]
pub unsafe extern "C" fn crw_wait_for_channel_report() {
    void crw_wait_for_channel_report(void)
    {
    crw_handle_channel_report();
    wait_event(crw_handler_wait_q, atomic_read(&crw_nr_req) == 0);
    }
//
// Machine checks for the channel subsystem must be enabled
// after the channel subsystem is initialized
//
#[no_mangle]
unsafe extern "C" fn crw_machine_check_init() -> int __init {
    static int __init crw_machine_check_init(void)
    {
    struct task_struct *task;
    task = kthread_run(crw_collect_info, core::ptr::null_mut(), "kmcheck");
    if (IS_ERR(task))
    return PTR_ERR(task);
    system_ctl_set_bit(14, CR14_CHANNEL_REPORT_SUBMASK_BIT);
    return 0;
    }
    device_initcall(crw_machine_check_init);
