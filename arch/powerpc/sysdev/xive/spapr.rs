//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/sysdev/xive/spapr.c
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
// Copyright 2016,2017 IBM Corporation.
//

    static u32 xive_queue_shift;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xive_irq_bitmap {
    pub bitmap: *mut c_ulong,
    pub base: c_uint,
    pub count: c_uint,
    pub lock: spinlock_t,
    pub list: list_head,
}

    static LIST_HEAD(xive_irq_bitmaps);
#[no_mangle]
unsafe extern "C" fn xive_irq_bitmap_add(base: c_int, count: c_int) -> int __init {
    static int __init xive_irq_bitmap_add(int base, int count)
    {
    struct xive_irq_bitmap *xibm;
    xibm = kzalloc_obj(*xibm);
    if (!xibm)
    return -ENOMEM;
    spin_lock_init(&xibm.lock);
    xibm.base = base;
    xibm.count = count;
    xibm.bitmap = bitmap_zalloc(xibm.count, GFP_KERNEL);
    if (!xibm.bitmap) {
    kfree(xibm);
    return -ENOMEM;
    }
    list_add(&xibm.list, &xive_irq_bitmaps);
    pr_info("Using IRQ range [%x-%x]", xibm.base,
    xibm.base + xibm.count - 1);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn xive_irq_bitmap_remove_all() {
    static void xive_irq_bitmap_remove_all(void)
    {
    struct xive_irq_bitmap *xibm, *tmp;
    list_for_each_entry_safe(xibm, tmp, &xive_irq_bitmaps, list) {
    list_del(&xibm.list);
    bitmap_free(xibm.bitmap);
    kfree(xibm);
    }
    }
#[no_mangle]
unsafe extern "C" fn __xive_irq_bitmap_alloc(xibm: *mut xive_irq_bitmap) -> c_int {
    static int __xive_irq_bitmap_alloc(struct xive_irq_bitmap *xibm)
    {
    int irq;
    irq = find_first_zero_bit(xibm.bitmap, xibm.count);
    if (irq != xibm.count) {
    set_bit(irq, xibm.bitmap);
    irq += xibm.base;
    } else {
    irq = -ENOMEM;
    }
    return irq;
    }
#[no_mangle]
unsafe extern "C" fn xive_irq_bitmap_alloc() -> c_int {
    static int xive_irq_bitmap_alloc(void)
    {
    struct xive_irq_bitmap *xibm;
    unsigned long flags;
    let mut irq: c_int = -ENOENT;
    list_for_each_entry(xibm, &xive_irq_bitmaps, list) {
    spin_lock_irqsave(&xibm.lock, flags);
    irq = __xive_irq_bitmap_alloc(xibm);
    spin_unlock_irqrestore(&xibm.lock, flags);
    if (irq >= 0)
    break;
    }
    return irq;
    }
#[no_mangle]
unsafe extern "C" fn xive_irq_bitmap_free(irq: c_int) {
    static void xive_irq_bitmap_free(int irq)
    {
    unsigned long flags;
    struct xive_irq_bitmap *xibm;
    list_for_each_entry(xibm, &xive_irq_bitmaps, list) {
    if ((irq >= xibm.base) && (irq < xibm.base + xibm.count)) {
    spin_lock_irqsave(&xibm.lock, flags);
    clear_bit(irq - xibm.base, xibm.bitmap);
    spin_unlock_irqrestore(&xibm.lock, flags);
    break;
    }
    }
    }
// Based on the similar routines in RTAS
#[no_mangle]
unsafe extern "C" fn plpar_busy_delay_time(rc: c_long) -> c_uint {
    static unsigned int plpar_busy_delay_time(long rc)
    {
    let mut ms: c_uint = 0;
    if (H_IS_LONG_BUSY(rc)) {
    ms = get_longbusy_msecs(rc);
    } else if (rc == H_BUSY) {
    ms = 10; /* seems appropriate for XIVE hcalls */
    }
    return ms;
    }
#[no_mangle]
unsafe extern "C" fn plpar_busy_delay(rc: c_int) -> c_uint {
    static unsigned int plpar_busy_delay(int rc)
    {
    unsigned int ms;
    ms = plpar_busy_delay_time(rc);
    if (ms)
    mdelay(ms);
    return ms;
    }
//
// Note: this call has a partition wide scope and can take a while to
// complete. If it returns H_LONG_BUSY_* it should be retried
// periodically.
//
#[no_mangle]
unsafe extern "C" fn plpar_int_reset(flags: c_ulong) -> c_long {
    static long plpar_int_reset(unsigned long flags)
    {
    long rc;
    do {
    rc = plpar_hcall_norets(H_INT_RESET, flags);
    } while (plpar_busy_delay(rc));
    if (rc)
    pr_err("H_INT_RESET failed %ld\n", rc);
    return rc;
    }
    static long plpar_int_get_source_info(unsigned long flags,
    unsigned long lisn,
    unsigned long *src_flags,
    unsigned long *eoi_page,
    unsigned long *trig_page,
    unsigned long *esb_shift)
    {
    unsigned long retbuf[PLPAR_HCALL_BUFSIZE];
    long rc;
    do {
    rc = plpar_hcall(H_INT_GET_SOURCE_INFO, retbuf, flags, lisn);
    } while (plpar_busy_delay(rc));
    if (rc) {
    pr_err("H_INT_GET_SOURCE_INFO lisn=0x%lx failed %ld\n", lisn, rc);
    return rc;
    }
// src_flags = retbuf[0];
// eoi_page  = retbuf[1];
// trig_page = retbuf[2];
// esb_shift = retbuf[3];
    pr_debug("H_INT_GET_SOURCE_INFO lisn=0x%lx flags=0x%lx eoi=0x%lx trig=0x%lx shift=0x%lx\n",
    lisn, retbuf[0], retbuf[1], retbuf[2], retbuf[3]);
    return 0;
    }

    static long plpar_int_set_source_config(unsigned long flags,
    unsigned long lisn,
    unsigned long target,
    unsigned long prio,
    unsigned long sw_irq)
    {
    long rc;
    pr_debug("H_INT_SET_SOURCE_CONFIG flags=0x%lx lisn=0x%lx target=%ld prio=%ld sw_irq=%ld\n",
    flags, lisn, target, prio, sw_irq);
    do {
    rc = plpar_hcall_norets(H_INT_SET_SOURCE_CONFIG, flags, lisn,
    target, prio, sw_irq);
    } while (plpar_busy_delay(rc));
    if (rc) {
    pr_err("H_INT_SET_SOURCE_CONFIG lisn=0x%lx target=%ld prio=%ld failed %ld\n",
    lisn, target, prio, rc);
    return rc;
    }
    return 0;
    }
    static long plpar_int_get_source_config(unsigned long flags,
    unsigned long lisn,
    unsigned long *target,
    unsigned long *prio,
    unsigned long *sw_irq)
    {
    unsigned long retbuf[PLPAR_HCALL_BUFSIZE];
    long rc;
    pr_debug("H_INT_GET_SOURCE_CONFIG flags=0x%lx lisn=0x%lx\n", flags, lisn);
    do {
    rc = plpar_hcall(H_INT_GET_SOURCE_CONFIG, retbuf, flags, lisn,
    target, prio, sw_irq);
    } while (plpar_busy_delay(rc));
    if (rc) {
    pr_err("H_INT_GET_SOURCE_CONFIG lisn=0x%lx failed %ld\n",
    lisn, rc);
    return rc;
    }
// target = retbuf[0];
// prio   = retbuf[1];
// sw_irq = retbuf[2];
    pr_debug("H_INT_GET_SOURCE_CONFIG target=%ld prio=%ld sw_irq=%ld\n",
    retbuf[0], retbuf[1], retbuf[2]);
    return 0;
    }
    static long plpar_int_get_queue_info(unsigned long flags,
    unsigned long target,
    unsigned long priority,
    unsigned long *esn_page,
    unsigned long *esn_size)
    {
    unsigned long retbuf[PLPAR_HCALL_BUFSIZE];
    long rc;
    do {
    rc = plpar_hcall(H_INT_GET_QUEUE_INFO, retbuf, flags, target,
    priority);
    } while (plpar_busy_delay(rc));
    if (rc) {
    pr_err("H_INT_GET_QUEUE_INFO cpu=%ld prio=%ld failed %ld\n",
    target, priority, rc);
    return rc;
    }
// esn_page = retbuf[0];
// esn_size = retbuf[1];
    pr_debug("H_INT_GET_QUEUE_INFO cpu=%ld prio=%ld page=0x%lx size=0x%lx\n",
    target, priority, retbuf[0], retbuf[1]);
    return 0;
    }

    static long plpar_int_set_queue_config(unsigned long flags,
    unsigned long target,
    unsigned long priority,
    unsigned long qpage,
    unsigned long qsize)
    {
    long rc;
    pr_debug("H_INT_SET_QUEUE_CONFIG flags=0x%lx target=%ld priority=0x%lx qpage=0x%lx qsize=0x%lx\n",
    flags,  target, priority, qpage, qsize);
    do {
    rc = plpar_hcall_norets(H_INT_SET_QUEUE_CONFIG, flags, target,
    priority, qpage, qsize);
    } while (plpar_busy_delay(rc));
    if (rc) {
    pr_err("H_INT_SET_QUEUE_CONFIG cpu=%ld prio=%ld qpage=0x%lx returned %ld\n",
    target, priority, qpage, rc);
    return  rc;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn plpar_int_sync(flags: c_ulong, lisn: c_ulong) -> c_long {
    static long plpar_int_sync(unsigned long flags, unsigned long lisn)
    {
    long rc;
    do {
    rc = plpar_hcall_norets(H_INT_SYNC, flags, lisn);
    } while (plpar_busy_delay(rc));
    if (rc) {
    pr_err("H_INT_SYNC lisn=0x%lx returned %ld\n", lisn, rc);
    return  rc;
    }
    return 0;
    }

    static long plpar_int_esb(unsigned long flags,
    unsigned long lisn,
    unsigned long offset,
    unsigned long in_data,
    unsigned long *out_data)
    {
    unsigned long retbuf[PLPAR_HCALL_BUFSIZE];
    long rc;
    pr_debug("H_INT_ESB flags=0x%lx lisn=0x%lx offset=0x%lx in=0x%lx\n",
    flags,  lisn, offset, in_data);
    do {
    rc = plpar_hcall(H_INT_ESB, retbuf, flags, lisn, offset,
    in_data);
    } while (plpar_busy_delay(rc));
    if (rc) {
    pr_err("H_INT_ESB lisn=0x%lx offset=0x%lx returned %ld\n",
    lisn, offset, rc);
    return  rc;
    }
// out_data = retbuf[0];
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn xive_spapr_esb_rw(lisn: u32, offset: u32, data: u64, write: bool) -> u64 {
    static u64 xive_spapr_esb_rw(u32 lisn, u32 offset, u64 data, bool write)
    {
    unsigned long read_data;
    long rc;
    rc = plpar_int_esb(write ? XIVE_ESB_FLAG_STORE : 0,
    lisn, offset, data, &read_data);
    if (rc)
    return -1;
    return write ? 0 : read_data;
    }

#[no_mangle]
unsafe extern "C" fn xive_spapr_populate_irq_data(hw_irq: u32, data: *mut xive_irq_data) -> c_int {
    static int xive_spapr_populate_irq_data(u32 hw_irq, struct xive_irq_data *data)
    {
    long rc;
    unsigned long flags;
    unsigned long eoi_page;
    unsigned long trig_page;
    unsigned long esb_shift;
    memset(data, 0, sizeof(*data));
    rc = plpar_int_get_source_info(0, hw_irq, &flags, &eoi_page, &trig_page,
    &esb_shift);
    if (rc)
    return  -EINVAL;
    if (flags & XIVE_SRC_H_INT_ESB)
    data.flags  |= XIVE_IRQ_FLAG_H_INT_ESB;
    if (flags & XIVE_SRC_STORE_EOI)
    data.flags  |= XIVE_IRQ_FLAG_STORE_EOI;
    if (flags & XIVE_SRC_LSI)
    data.flags  |= XIVE_IRQ_FLAG_LSI;
    data.eoi_page  = eoi_page;
    data.esb_shift = esb_shift;
    data.trig_page = trig_page;
    data.hw_irq = hw_irq;
//
// No chip-id for the sPAPR backend. This has an impact how we
// pick a target. See xive_pick_irq_target().
//
    data.src_chip = XIVE_INVALID_CHIP_ID;
//
// When the H_INT_ESB flag is set, the H_INT_ESB hcall should
// be used for interrupt management. Skip the remapping of the
// ESB pages which are not available.
//
    if (data.flags & XIVE_IRQ_FLAG_H_INT_ESB)
    return 0;
    data.eoi_mmio = ioremap(data.eoi_page, 1u << data.esb_shift);
    if (!data.eoi_mmio) {
    pr_err("Failed to map EOI page for irq 0x%x\n", hw_irq);
    return -ENOMEM;
    }
// Full function page supports trigger
    if (flags & XIVE_SRC_TRIGGER) {
    data.trig_mmio = data.eoi_mmio;
    return 0;
    }
    data.trig_mmio = ioremap(data.trig_page, 1u << data.esb_shift);
    if (!data.trig_mmio) {
    iounmap(data.eoi_mmio);
    pr_err("Failed to map trigger page for irq 0x%x\n", hw_irq);
    return -ENOMEM;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn xive_spapr_configure_irq(hw_irq: u32, target: u32, prio: u8, sw_irq: u32) -> c_int {
    static int xive_spapr_configure_irq(u32 hw_irq, u32 target, u8 prio, u32 sw_irq)
    {
    long rc;
    rc = plpar_int_set_source_config(XIVE_SRC_SET_EISN, hw_irq, target,
    prio, sw_irq);
    let mut rc: return = = 0 ? 0 : -ENXIO;
    }
    static int xive_spapr_get_irq_config(u32 hw_irq, u32 *target, u8 *prio,
    u32 *sw_irq)
    {
    long rc;
    unsigned long h_target;
    unsigned long h_prio;
    unsigned long h_sw_irq;
    rc = plpar_int_get_source_config(0, hw_irq, &h_target, &h_prio,
    &h_sw_irq);
// target = h_target;
// prio = h_prio;
// sw_irq = h_sw_irq;
    let mut rc: return = = 0 ? 0 : -ENXIO;
    }
// This can be called multiple time to change a queue configuration
    static int xive_spapr_configure_queue(u32 target, struct xive_q *q, u8 prio,
    __be32 *qpage, u32 order)
    {
    let mut rc: i64 = 0;
    unsigned long esn_page;
    unsigned long esn_size;
    u64 flags, qpage_phys;
// If there's an actual queue page, clean it
    if (order) {
    if (WARN_ON(!qpage))
    return -EINVAL;
    qpage_phys = __pa(qpage);
    } else {
    qpage_phys = 0;
    }
// Initialize the rest of the fields
    q.msk = order ? ((1u << (order - 2)) - 1) : 0;
    q.idx = 0;
    q.toggle = 0;
    rc = plpar_int_get_queue_info(0, target, prio, &esn_page, &esn_size);
    if (rc) {
    pr_err("Error %lld getting queue info CPU %d prio %d\n", rc,
    target, prio);
    rc = -EIO;
    goto fail;
    }
// TODO: add support for the notification page
    q.eoi_phys = esn_page;
// Default is to always notify
    flags = XIVE_EQ_ALWAYS_NOTIFY;
// Configure and enable the queue in HW
    rc = plpar_int_set_queue_config(flags, target, prio, qpage_phys, order);
    if (rc) {
    pr_err("Error %lld setting queue for CPU %d prio %d\n", rc,
    target, prio);
    rc = -EIO;
    } else {
    q.qpage = qpage;
    if (is_secure_guest())
    uv_share_page(PHYS_PFN(qpage_phys),
    1 << xive_alloc_order(order));
    }
    fail:
    return rc;
    }
    static int xive_spapr_setup_queue(unsigned int cpu, struct xive_cpu *xc,
    u8 prio)
    {
    struct xive_q *q = &xc.queue[prio];
    __be32 *qpage;
    qpage = xive_queue_page_alloc(cpu, xive_queue_shift);
    if (IS_ERR(qpage))
    return PTR_ERR(qpage);
    return xive_spapr_configure_queue(get_hard_smp_processor_id(cpu),
    q, prio, qpage, xive_queue_shift);
    }
    static void xive_spapr_cleanup_queue(unsigned int cpu, struct xive_cpu *xc,
    u8 prio)
    {
    struct xive_q *q = &xc.queue[prio];
    unsigned int alloc_order;
    long rc;
    let mut hw_cpu: c_int = get_hard_smp_processor_id(cpu);
    rc = plpar_int_set_queue_config(0, hw_cpu, prio, 0, 0);
    if (rc)
    pr_err("Error %ld setting queue for CPU %d prio %d\n", rc,
    hw_cpu, prio);
    alloc_order = xive_alloc_order(xive_queue_shift);
    if (is_secure_guest())
    uv_unshare_page(PHYS_PFN(__pa(q.qpage)), 1 << alloc_order);
    free_pages((unsigned long)q.qpage, alloc_order);
    q.qpage = core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn xive_spapr_match(node: *mut device_node) -> bool {
    static bool xive_spapr_match(struct device_node *node)
    {
// Ignore cascaded controllers for the moment
    return true;
    }

#[no_mangle]
unsafe extern "C" fn xive_spapr_get_ipi(cpu: c_uint, xc: *mut xive_cpu) -> c_int {
    static int xive_spapr_get_ipi(unsigned int cpu, struct xive_cpu *xc)
    {
    let mut irq: c_int = xive_irq_bitmap_alloc();
    if (irq < 0) {
    pr_err("Failed to allocate IPI on CPU %d\n", cpu);
    return -ENXIO;
    }
    xc.hw_ipi = irq;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn xive_spapr_put_ipi(cpu: c_uint, xc: *mut xive_cpu) {
    static void xive_spapr_put_ipi(unsigned int cpu, struct xive_cpu *xc)
    {
    if (xc.hw_ipi == XIVE_BAD_IRQ)
    return;
    xive_irq_bitmap_free(xc.hw_ipi);
    xc.hw_ipi = XIVE_BAD_IRQ;
    }

#[no_mangle]
unsafe extern "C" fn xive_spapr_shutdown() {
    static void xive_spapr_shutdown(void)
    {
    plpar_int_reset(0);
    }
//
// Perform an "ack" cycle on the current thread. Grab the pending
// active priorities and update the CPPR to the most favored one.
//
#[no_mangle]
unsafe extern "C" fn xive_spapr_update_pending(xc: *mut xive_cpu) {
    static void xive_spapr_update_pending(struct xive_cpu *xc)
    {
    u8 nsr, cppr;
    u16 ack;
//
// Perform the "Acknowledge O/S to Register" cycle.
//
// Let's speedup the access to the TIMA using the raw I/O
// accessor as we don't need the synchronisation routine of
// the higher level ones
//
    ack = be16_to_cpu(__raw_readw(xive_tima + TM_SPC_ACK_OS_REG));
// Synchronize subsequent queue accesses
    mb();
//
// Grab the CPPR and the "NSR" field which indicates the source
// of the interrupt (if any)
//
    cppr = ack & 0xff;
    nsr = ack >> 8;
    if (nsr & TM_QW1_NSR_EO) {
    if (cppr == 0xff)
    return;
// Mark the priority pending
    xc.pending_prio |= 1 << cppr;
//
// A new interrupt should never have a CPPR less favored
// than our current one.
//
    if (cppr >= xc.cppr)
    pr_err("CPU %d odd ack CPPR, got %d at %d\n",
    smp_processor_id(), cppr, xc.cppr);
// Update our idea of what the CPPR is
    xc.cppr = cppr;
    }
    }
#[no_mangle]
unsafe extern "C" fn xive_spapr_setup_cpu(cpu: c_uint, xc: *mut xive_cpu) {
    static void xive_spapr_setup_cpu(unsigned int cpu, struct xive_cpu *xc)
    {
// Only some debug on the TIMA settings
    pr_debug("(HW value: %08x %08x %08x)\n",
    in_be32(xive_tima + TM_QW1_OS + TM_WORD0),
    in_be32(xive_tima + TM_QW1_OS + TM_WORD1),
    in_be32(xive_tima + TM_QW1_OS + TM_WORD2));
    }
#[no_mangle]
unsafe extern "C" fn xive_spapr_teardown_cpu(cpu: c_uint, xc: *mut xive_cpu) {
    static void xive_spapr_teardown_cpu(unsigned int cpu, struct xive_cpu *xc)
    {
// Nothing to do */;
    }
#[no_mangle]
unsafe extern "C" fn xive_spapr_sync_source(hw_irq: u32) {
    static void xive_spapr_sync_source(u32 hw_irq)
    {
// Specs are unclear on what this is doing
    plpar_int_sync(0, hw_irq);
    }
#[no_mangle]
unsafe extern "C" fn xive_spapr_debug_show(m: *mut seq_file, private: *mut c_void) -> c_int {
    static int xive_spapr_debug_show(struct seq_file *m, void *private)
    {
    struct xive_irq_bitmap *xibm;
    list_for_each_entry(xibm, &xive_irq_bitmaps, list)
    seq_printf(m, "bitmap #%d: %*pbl\n", xibm.count, xibm.count, xibm.bitmap);
    return 0;
    }
    static const struct xive_ops xive_spapr_ops = {
    .populate_irq_data	= xive_spapr_populate_irq_data,
    .configure_irq		= xive_spapr_configure_irq,
    .get_irq_config		= xive_spapr_get_irq_config,
    .setup_queue		= xive_spapr_setup_queue,
    .cleanup_queue		= xive_spapr_cleanup_queue,
    .match			= xive_spapr_match,
    .shutdown		= xive_spapr_shutdown,
    .update_pending		= xive_spapr_update_pending,
    .setup_cpu		= xive_spapr_setup_cpu,
    .teardown_cpu		= xive_spapr_teardown_cpu,
    .sync_source		= xive_spapr_sync_source,
    .esb_rw			= xive_spapr_esb_rw,

    .get_ipi		= xive_spapr_get_ipi,
    .put_ipi		= xive_spapr_put_ipi,
    .debug_show		= xive_spapr_debug_show,

    .name			= "spapr",
    };
//
// get max priority from "/ibm,plat-res-int-priorities"
//
#[no_mangle]
unsafe extern "C" fn xive_get_max_prio(max_prio: *mut u8) -> bool __init {
    static bool __init xive_get_max_prio(u8 *max_prio)
    {
    struct device_node *rootdn;
    const __be32 *reg;
    u32 len;
    int prio, found;
    rootdn = of_find_node_by_path("/");
    if (!rootdn) {
    pr_err("not root node found !\n");
    return false;
    }
    reg = of_get_property(rootdn, "ibm,plat-res-int-priorities", &len);
    of_node_put(rootdn);
    if (!reg) {
    pr_err("Failed to read 'ibm,plat-res-int-priorities' property\n");
    return false;
    }
    if (len % (2 * sizeof(u32)) != 0) {
    pr_err("invalid 'ibm,plat-res-int-priorities' property\n");
    return false;
    }
// HW supports priorities in the range [0-7] and 0xFF is a
// wildcard priority used to mask. We scan the ranges reserved
// by the hypervisor to find the lowest priority we can use.
//
    found = 0xFF;
    for (prio = 0; prio < 8; prio++) {
    let mut reserved: c_int = 0;
    int i;
    for (i = 0; i < len / (2 * sizeof(u32)); i++) {
    let mut base: c_int = be32_to_cpu(reg[2 * i]);
    let mut range: c_int = be32_to_cpu(reg[2 * i + 1]);
    if (prio >= base && prio < base + range)
    reserved++;
    }
    if (!reserved)
    found = prio;
    }
    if (found == 0xFF) {
    pr_err("no valid priority found in 'ibm,plat-res-int-priorities'\n");
    return false;
    }
// max_prio = found;
    return true;
    }
#[no_mangle]
unsafe extern "C" fn get_vec5_feature(index: c_uint) -> *const u8 __init {
    static const u8 *__init get_vec5_feature(unsigned int index)
    {
    unsigned long root, chosen;
    int size;
    const u8 *vec5;
    root = of_get_flat_dt_root();
    chosen = of_get_flat_dt_subnode_by_name(root, "chosen");
    if (chosen == -FDT_ERR_NOTFOUND)
    return core::ptr::null_mut();
    vec5 = of_get_flat_dt_prop(chosen, "ibm,architecture-vec-5", &size);
    if (!vec5)
    return core::ptr::null_mut();
    if (size <= index)
    return core::ptr::null_mut();
    return vec5 + index;
    }
#[no_mangle]
unsafe extern "C" fn xive_spapr_disabled() -> bool __init {
    static bool __init xive_spapr_disabled(void)
    {
    const u8 *vec5_xive;
    vec5_xive = get_vec5_feature(OV5_INDX(OV5_XIVE_SUPPORT));
    if (vec5_xive) {
    u8 val;
    val = *vec5_xive & OV5_FEAT(OV5_XIVE_SUPPORT);
    switch (val) {
    case OV5_FEAT(OV5_XIVE_EITHER):
    case OV5_FEAT(OV5_XIVE_LEGACY):
    break;
    case OV5_FEAT(OV5_XIVE_EXPLOIT):
// Hypervisor only supports XIVE
    if (xive_cmdline_disabled)
    pr_warn("WARNING: Ignoring cmdline option xive=off\n");
    return false;
    default:
    pr_warn("%s: Unknown xive support option: 0x%x\n",
    __func__, val);
    break;
    }
    }
    return xive_cmdline_disabled;
    }
#[no_mangle]
pub unsafe extern "C" fn xive_spapr_init() -> bool __init {
    bool __init xive_spapr_init(void)
    {
    struct device_node *np;
    struct resource r;
    void __iomem *tima;
    u8 max_prio;
    u32 val;
    u32 len;
    const __be32 *reg;
    int i, err;
    if (xive_spapr_disabled())
    return false;
    pr_devel("%s()\n", __func__);
    np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "ibm,power-ivpe");
    if (!np) {
    pr_devel("not found !\n");
    return false;
    }
    pr_devel("Found %s\n", np.full_name);
// Resource 1 is the OS ring TIMA
    if (of_address_to_resource(np, 1, &r)) {
    pr_err("Failed to get thread mgmnt area resource\n");
    goto err_put;
    }
    tima = ioremap(r.start, resource_size(&r));
    if (!tima) {
    pr_err("Failed to map thread mgmnt area\n");
    goto err_put;
    }
    if (!xive_get_max_prio(&max_prio))
    goto err_unmap;
// Feed the IRQ number allocator with the ranges given in the DT
    reg = of_get_property(np, "ibm,xive-lisn-ranges", &len);
    if (!reg) {
    pr_err("Failed to read 'ibm,xive-lisn-ranges' property\n");
    goto err_unmap;
    }
    if (len % (2 * sizeof(u32)) != 0) {
    pr_err("invalid 'ibm,xive-lisn-ranges' property\n");
    goto err_unmap;
    }
    for (i = 0; i < len / (2 * sizeof(u32)); i++, reg += 2) {
    err = xive_irq_bitmap_add(be32_to_cpu(reg[0]),
    be32_to_cpu(reg[1]));
    if (err < 0)
    goto err_mem_free;
    }
// Iterate the EQ sizes and pick one
    of_property_for_each_u32(np, "ibm,xive-eq-sizes", val) {
    xive_queue_shift = val;
    if (val == PAGE_SHIFT)
    break;
    }
// Initialize XIVE core with our backend
    if (!xive_core_init(np, &xive_spapr_ops, tima, TM_QW1_OS, max_prio))
    goto err_mem_free;
    of_node_put(np);
    pr_info("Using %dkB queues\n", 1 << (xive_queue_shift - 10));
    return true;
    err_mem_free:
    xive_irq_bitmap_remove_all();
    err_unmap:
    iounmap(tima);
    err_put:
    of_node_put(np);
    return false;
    }
    machine_arch_initcall(pseries, xive_core_debug_init);
