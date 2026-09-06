//! Automatically rewritten from C to Rust
//! Source: drivers/s390/cio/qdio_thinint.c
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
// Copyright IBM Corp. 2000, 2009
// Author(s): Utz Bacher <utz.bacher@de.ibm.com>
// Cornelia Huck <cornelia.huck@de.ibm.com>
// Jan Glauber <jang@linux.vnet.ibm.com>
//

//
// Restriction: only 63 iqdio subchannels would have its own indicator,
// after that, subsequent subchannels share one indicator
//
pub const TIQDIO_NR_NONSHARED_IND: c_int = 63;

pub const TIQDIO_SHARED_IND: c_int = 63;
// device state change indicators
#[repr(C)]
#[derive(Copy, Clone)]
pub struct indicator_t {
    pub /: *mut *mut u32 ind; / u32 because of compare-and-swap performance,
    pub /: *mut *mut atomic_t count; / use count, 0 or 1 for non-shared indicators,
}

// list of thin interrupt input queues
    static LIST_HEAD(tiq_list);
    static DEFINE_MUTEX(tiq_list_lock);
    static struct indicator_t *q_indicators;
    u64 last_ai_time;
// returns addr for the device state change indicator
    static u32 *get_indicator(void)
    {
    int i;
    for (i = 0; i < TIQDIO_NR_NONSHARED_IND; i++)
    if (!atomic_cmpxchg(&q_indicators[i].count, 0, 1))
    return &q_indicators[i].ind;
// use the shared indicator
    atomic_inc(&q_indicators[TIQDIO_SHARED_IND].count);
    return &q_indicators[TIQDIO_SHARED_IND].ind;
    }
#[no_mangle]
unsafe extern "C" fn put_indicator(addr: *mut u32) {
    static void put_indicator(u32 *addr)
    {
    struct indicator_t *ind = container_of(addr, struct indicator_t, ind);
    if (!addr)
    return;
    atomic_dec(&ind.count);
    }
#[no_mangle]
pub unsafe extern "C" fn references_shared_dsci(irq_ptr: *mut qdio_irq) -> c_int {
    static inline int references_shared_dsci(struct qdio_irq *irq_ptr)
    {
    return irq_ptr.dsci == &q_indicators[TIQDIO_SHARED_IND].ind;
    }
#[no_mangle]
pub unsafe extern "C" fn test_nonshared_ind(irq_ptr: *mut qdio_irq) -> c_int {
    int test_nonshared_ind(struct qdio_irq *irq_ptr)
    {
    if (!is_thinint_irq(irq_ptr))
    return 0;
    if (references_shared_dsci(irq_ptr))
    return 0;
    if (*irq_ptr.dsci)
    return 1;
    else
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn clear_shared_ind() -> u32 {
    static inline u32 clear_shared_ind(void)
    {
    if (!atomic_read(&q_indicators[TIQDIO_SHARED_IND].count))
    return 0;
    return xchg(&q_indicators[TIQDIO_SHARED_IND].ind, 0);
    }
//
// tiqdio_thinint_handler - thin interrupt handler for qdio
// @airq: pointer to adapter interrupt descriptor
// @tpi_info: interrupt information (e.g. floating vs directed -- unused)
//
    static void tiqdio_thinint_handler(struct airq_struct *airq,
    struct tpi_info *tpi_info)
    {
    let mut irq_time: u64 = get_lowcore().int_clock.tod;
    let mut si_used: u32 = clear_shared_ind();
    struct qdio_irq *irq;
    last_ai_time = irq_time;
    inc_irq_stat(IRQIO_QAI);
// protect tiq_list entries, only changed in activate or shutdown
    rcu_read_lock();
    list_for_each_entry_rcu(irq, &tiq_list, entry) {
// only process queues from changed sets
    if (unlikely(references_shared_dsci(irq))) {
    if (!si_used)
    continue;
    } else {
    if (!*irq.dsci)
    continue;
    xchg(irq.dsci, 0);
    }
    qdio_deliver_irq(irq);
    irq.last_data_irq_time = irq_time;
    QDIO_PERF_STAT_INC(irq, adapter_int);
    }
    rcu_read_unlock();
    }
    static struct airq_struct tiqdio_airq = {
    .handler = tiqdio_thinint_handler,
    .isc = QDIO_AIRQ_ISC,
    };
#[no_mangle]
unsafe extern "C" fn set_subchannel_ind(irq_ptr: *mut qdio_irq, reset: c_int) -> c_int {
    static int set_subchannel_ind(struct qdio_irq *irq_ptr, int reset)
    {
    struct chsc_scssc_area *scssc = (void *)irq_ptr.chsc_page;
    dma64_t summary_indicator_addr, subchannel_indicator_addr;
    int rc;
    if (reset) {
    summary_indicator_addr = 0;
    subchannel_indicator_addr = 0;
    } else {
    summary_indicator_addr = virt_to_dma64(tiqdio_airq.lsi_ptr);
    subchannel_indicator_addr = virt_to_dma64(irq_ptr.dsci);
    }
    rc = chsc_sadc(irq_ptr.schid, scssc, summary_indicator_addr,
    subchannel_indicator_addr, tiqdio_airq.isc);
    if (rc) {
    DBF_ERROR("%4x SSI r:%4x", irq_ptr.schid.sch_no,
    scssc.response.code);
    goto out;
    }
    DBF_EVENT("setscind");
    DBF_HEX(&summary_indicator_addr, sizeof(summary_indicator_addr));
    DBF_HEX(&subchannel_indicator_addr, sizeof(subchannel_indicator_addr));
    out:
    return rc;
    }
#[no_mangle]
pub unsafe extern "C" fn qdio_establish_thinint(irq_ptr: *mut qdio_irq) -> c_int {
    int qdio_establish_thinint(struct qdio_irq *irq_ptr)
    {
    int rc;
    if (!is_thinint_irq(irq_ptr))
    return 0;
    irq_ptr.dsci = get_indicator();
    DBF_HEX(&irq_ptr.dsci, sizeof(void *));
    rc = set_subchannel_ind(irq_ptr, 0);
    if (rc) {
    put_indicator(irq_ptr.dsci);
    return rc;
    }
    mutex_lock(&tiq_list_lock);
    list_add_rcu(&irq_ptr.entry, &tiq_list);
    mutex_unlock(&tiq_list_lock);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn qdio_shutdown_thinint(irq_ptr: *mut qdio_irq) {
    void qdio_shutdown_thinint(struct qdio_irq *irq_ptr)
    {
    if (!is_thinint_irq(irq_ptr))
    return;
    mutex_lock(&tiq_list_lock);
    list_del_rcu(&irq_ptr.entry);
    mutex_unlock(&tiq_list_lock);
    synchronize_rcu();
// reset adapter interrupt indicators
    set_subchannel_ind(irq_ptr, 1);
    put_indicator(irq_ptr.dsci);
    }
#[no_mangle]
pub unsafe extern "C" fn qdio_thinint_init() -> int __init {
    int __init qdio_thinint_init(void)
    {
    int rc;
    q_indicators = kzalloc_objs(struct indicator_t, TIQDIO_NR_INDICATORS);
    if (!q_indicators)
    return -ENOMEM;
    rc = register_adapter_interrupt(&tiqdio_airq);
    if (rc) {
    DBF_EVENT("RTI:%x", rc);
    kfree(q_indicators);
    return rc;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn qdio_thinint_exit() -> void __exit {
    void __exit qdio_thinint_exit(void)
    {
    WARN_ON(!list_empty(&tiq_list));
    unregister_adapter_interrupt(&tiqdio_airq);
    kfree(q_indicators);
    }
