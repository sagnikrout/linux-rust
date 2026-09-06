//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/powernv/opal-memory-errors.c
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
// OPAL asynchronus Memory error handling support in PowerNV.
//
// Copyright 2013 IBM Corporation
// Author: Mahesh Salgaonkar <mahesh@linux.vnet.ibm.com>
//

    static int opal_mem_err_nb_init;
    static LIST_HEAD(opal_memory_err_list);
    static DEFINE_SPINLOCK(opal_mem_err_lock);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct OpalMsgNode {
    pub list: list_head,
    pub msg: opal_msg,
}

#[no_mangle]
unsafe extern "C" fn handle_memory_error_event(merr_evt: *mut OpalMemoryErrorData) {
    static void handle_memory_error_event(struct OpalMemoryErrorData *merr_evt)
    {
    uint64_t paddr_start, paddr_end;
    pr_debug("%s: Retrieved memory error event, type: 0x%x\n",
    __func__, merr_evt.type);
    switch (merr_evt.type) {
    case OPAL_MEM_ERR_TYPE_RESILIENCE:
    paddr_start = be64_to_cpu(merr_evt.u.resilience.physical_address_start);
    paddr_end = be64_to_cpu(merr_evt.u.resilience.physical_address_end);
    break;
    case OPAL_MEM_ERR_TYPE_DYN_DALLOC:
    paddr_start = be64_to_cpu(merr_evt.u.dyn_dealloc.physical_address_start);
    paddr_end = be64_to_cpu(merr_evt.u.dyn_dealloc.physical_address_end);
    break;
    default:
    return;
    }
    for (; paddr_start < paddr_end; paddr_start += PAGE_SIZE) {
    memory_failure(paddr_start >> PAGE_SHIFT, 0);
    }
    }
#[no_mangle]
unsafe extern "C" fn handle_memory_error() {
    static void handle_memory_error(void)
    {
    unsigned long flags;
    struct OpalMemoryErrorData *merr_evt;
    struct OpalMsgNode *msg_node;
    spin_lock_irqsave(&opal_mem_err_lock, flags);
    while (!list_empty(&opal_memory_err_list)) {
    msg_node = list_entry(opal_memory_err_list.next,
    struct OpalMsgNode, list);
    list_del(&msg_node.list);
    spin_unlock_irqrestore(&opal_mem_err_lock, flags);
    merr_evt = (struct OpalMemoryErrorData *)
    &msg_node.msg.params[0];
    handle_memory_error_event(merr_evt);
    kfree(msg_node);
    spin_lock_irqsave(&opal_mem_err_lock, flags);
    }
    spin_unlock_irqrestore(&opal_mem_err_lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn mem_error_handler(work: *mut work_struct) {
    static void mem_error_handler(struct work_struct *work)
    {
    handle_memory_error();
    }
    static DECLARE_WORK(mem_error_work, mem_error_handler);
//
// opal_memory_err_event - notifier handler that queues up the opal message
// to be processed later.
//
    static int opal_memory_err_event(struct notifier_block *nb,
    unsigned long msg_type, void *msg)
    {
    unsigned long flags;
    struct OpalMsgNode *msg_node;
    if (msg_type != OPAL_MSG_MEM_ERR)
    return 0;
    msg_node = kzalloc_obj(*msg_node, GFP_ATOMIC);
    if (!msg_node) {
    pr_err("MEMORY_ERROR: out of memory, Opal message event not"
    "handled\n");
    return -ENOMEM;
    }
    memcpy(&msg_node.msg, msg, sizeof(msg_node.msg));
    spin_lock_irqsave(&opal_mem_err_lock, flags);
    list_add(&msg_node.list, &opal_memory_err_list);
    spin_unlock_irqrestore(&opal_mem_err_lock, flags);
    schedule_work(&mem_error_work);
    return 0;
    }
    static struct notifier_block opal_mem_err_nb = {
    .notifier_call	= opal_memory_err_event,
    .next		= core::ptr::null_mut(),
    .priority	= 0,
    };
#[no_mangle]
unsafe extern "C" fn opal_mem_err_init() -> int __init {
    static int __init opal_mem_err_init(void)
    {
    int ret;
    if (!opal_mem_err_nb_init) {
    ret = opal_message_notifier_register(
    OPAL_MSG_MEM_ERR, &opal_mem_err_nb);
    if (ret) {
    pr_err("%s: Can't register OPAL event notifier (%d)\n",
    __func__, ret);
    return ret;
    }
    opal_mem_err_nb_init = 1;
    }
    return 0;
    }
    machine_device_initcall(powernv, opal_mem_err_init);
