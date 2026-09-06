//! Automatically rewritten from C to Rust
//! Source: drivers/infiniband/hw/mthca/mthca_eq.c
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


//
// Copyright (c) 2004, 2005 Topspin Communications.  All rights reserved.
// Copyright (c) 2005 Mellanox Technologies. All rights reserved.
//
// This software is available to you under a choice of one of two
// licenses.  You may choose to be licensed under the terms of the GNU
// General Public License (GPL) Version 2, available from the file
// COPYING in the main directory of this source tree, or the
// OpenIB.org BSD license below:
//
// Redistribution and use in source and binary forms, with or
// without modification, are permitted provided that the following
// conditions are met:
//
// - Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer.
//
// - Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following
// disclaimer in the documentation and/or other materials
// provided with the distribution.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
// BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
// ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//

    enum {
    MTHCA_NUM_ASYNC_EQE = 0x80,
    MTHCA_NUM_CMD_EQE   = 0x80,
    MTHCA_NUM_SPARE_EQE = 0x80,
    MTHCA_EQ_ENTRY_SIZE = 0x20
    };
//
// Must be packed because start is 64 bits but only aligned to 32 bits.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mthca_eq_context {
    pub flags: __be32,
    pub start: __be64,
    pub logsize_usrpage: __be32,
    pub /: *mut *mut __be32 tavor_pd; / reserved for Arbel,
    pub reserved1: [u8; 3],
    pub intr: u8,
    pub /: *mut *mut __be32 arbel_pd; / lost_count for Tavor,
    pub lkey: __be32,
    pub reserved2: [u32; 2],
    pub consumer_index: __be32,
    pub producer_index: __be32,
    pub reserved3: [u32; 4],
    pub __packed: },

    enum {
    MTHCA_EVENT_TYPE_COMP       	    = 0x00,
    MTHCA_EVENT_TYPE_PATH_MIG   	    = 0x01,
    MTHCA_EVENT_TYPE_COMM_EST   	    = 0x02,
    MTHCA_EVENT_TYPE_SQ_DRAINED 	    = 0x03,
    MTHCA_EVENT_TYPE_SRQ_QP_LAST_WQE    = 0x13,
    MTHCA_EVENT_TYPE_SRQ_LIMIT	    = 0x14,
    MTHCA_EVENT_TYPE_CQ_ERROR   	    = 0x04,
    MTHCA_EVENT_TYPE_WQ_CATAS_ERROR     = 0x05,
    MTHCA_EVENT_TYPE_EEC_CATAS_ERROR    = 0x06,
    MTHCA_EVENT_TYPE_PATH_MIG_FAILED    = 0x07,
    MTHCA_EVENT_TYPE_WQ_INVAL_REQ_ERROR = 0x10,
    MTHCA_EVENT_TYPE_WQ_ACCESS_ERROR    = 0x11,
    MTHCA_EVENT_TYPE_SRQ_CATAS_ERROR    = 0x12,
    MTHCA_EVENT_TYPE_LOCAL_CATAS_ERROR  = 0x08,
    MTHCA_EVENT_TYPE_PORT_CHANGE        = 0x09,
    MTHCA_EVENT_TYPE_EQ_OVERFLOW        = 0x0f,
    MTHCA_EVENT_TYPE_ECC_DETECT         = 0x0e,
    MTHCA_EVENT_TYPE_CMD                = 0x0a
}

    (1ULL << MTHCA_EVENT_TYPE_COMM_EST)           | \
    (1ULL << MTHCA_EVENT_TYPE_SQ_DRAINED)         | \
    (1ULL << MTHCA_EVENT_TYPE_CQ_ERROR)           | \
    (1ULL << MTHCA_EVENT_TYPE_WQ_CATAS_ERROR)     | \
    (1ULL << MTHCA_EVENT_TYPE_EEC_CATAS_ERROR)    | \
    (1ULL << MTHCA_EVENT_TYPE_PATH_MIG_FAILED)    | \
    (1ULL << MTHCA_EVENT_TYPE_WQ_INVAL_REQ_ERROR) | \
    (1ULL << MTHCA_EVENT_TYPE_WQ_ACCESS_ERROR)    | \
    (1ULL << MTHCA_EVENT_TYPE_LOCAL_CATAS_ERROR)  | \
    (1ULL << MTHCA_EVENT_TYPE_PORT_CHANGE)        | \
    (1ULL << MTHCA_EVENT_TYPE_ECC_DETECT))

    (1ULL << MTHCA_EVENT_TYPE_SRQ_QP_LAST_WQE)    | \
    (1ULL << MTHCA_EVENT_TYPE_SRQ_LIMIT))

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mthca_eqe {
    pub reserved1: u8,
    pub type: u8,
    pub reserved2: u8,
    pub subtype: u8,
    union {
    pub raw: [u32; 6],
    struct {
    pub cqn: __be32,
    pub comp: } __packed,
    struct {
    pub reserved1: u16,
    pub token: __be16,
    pub reserved2: u32,
    pub reserved3: [u8; 3],
    pub status: u8,
    pub out_param: __be64,
    pub cmd: } __packed,
    struct {
    pub qpn: __be32,
    pub qp: } __packed,
    struct {
    pub srqn: __be32,
    pub srq: } __packed,
    struct {
    pub cqn: __be32,
    pub reserved1: u32,
    pub reserved2: [u8; 3],
    pub syndrome: u8,
    pub cq_err: } __packed,
    struct {
    pub reserved1: [u32; 2],
    pub port: __be32,
    pub port_change: } __packed,
    pub event: },
    pub reserved3: [u8; 3],
    pub owner: u8,
    pub __packed: },

#[no_mangle]
pub unsafe extern "C" fn async_mask(dev: *mut mthca_dev) -> u64 {
    static inline u64 async_mask(struct mthca_dev *dev)
    {
    return dev.mthca_flags & MTHCA_FLAG_SRQ ?
    MTHCA_ASYNC_EVENT_MASK | MTHCA_SRQ_EVENT_MASK :
    }
#[no_mangle]
pub unsafe extern "C" fn tavor_set_eq_ci(dev: *mut mthca_dev, eq: *mut mthca_eq, ci: u32) {
    static inline void tavor_set_eq_ci(struct mthca_dev *dev, struct mthca_eq *eq, u32 ci)
    {
//
// This barrier makes sure that all updates to ownership bits
// done by set_eqe_hw() hit memory before the consumer index
// is updated.  set_eq_ci() allows the HCA to possibly write
// more EQ entries, and we want to avoid the exceedingly
// unlikely possibility of the HCA writing an entry and then
// having set_eqe_hw() overwrite the owner field.
//
    mthca_write64(MTHCA_EQ_DB_SET_CI | eq.eqn, ci & (eq.nent - 1),
    dev.kar + MTHCA_EQ_DOORBELL,
    }
#[no_mangle]
pub unsafe extern "C" fn arbel_set_eq_ci(dev: *mut mthca_dev, eq: *mut mthca_eq, ci: u32) {
    static inline void arbel_set_eq_ci(struct mthca_dev *dev, struct mthca_eq *eq, u32 ci)
    {
// See comment in tavor_set_eq_ci() above.
    __raw_writel(( u32) cpu_to_be32(ci),
    pub 8): *mut *mut dev->eq_regs.arbel.eq_set_ci_base + eq->eqn,
// We still want ordering, just not swabbing, so add a barrier
    }
#[no_mangle]
pub unsafe extern "C" fn set_eq_ci(dev: *mut mthca_dev, eq: *mut mthca_eq, ci: u32) {
    static inline void set_eq_ci(struct mthca_dev *dev, struct mthca_eq *eq, u32 ci)
    {
    if (mthca_is_memfree(dev))
    pub ci): arbel_set_eq_ci(dev, eq,,
    else
    pub ci): tavor_set_eq_ci(dev, eq,,
    }
#[no_mangle]
pub unsafe extern "C" fn tavor_eq_req_not(dev: *mut mthca_dev, eqn: c_int) {
    static inline void tavor_eq_req_not(struct mthca_dev *dev, int eqn)
    {
    mthca_write64(MTHCA_EQ_DB_REQ_NOT | eqn, 0,
    dev.kar + MTHCA_EQ_DOORBELL,
    }
#[no_mangle]
pub unsafe extern "C" fn arbel_eq_req_not(dev: *mut mthca_dev, eqn_mask: u32) {
    static inline void arbel_eq_req_not(struct mthca_dev *dev, u32 eqn_mask)
    {
    pub dev->eq_regs.arbel.eq_arm): writel(eqn_mask,,
    }
#[no_mangle]
pub unsafe extern "C" fn disarm_cq(dev: *mut mthca_dev, eqn: c_int, cqn: c_int) {
    static inline void disarm_cq(struct mthca_dev *dev, int eqn, int cqn)
    {
    if (!mthca_is_memfree(dev)) {
    mthca_write64(MTHCA_EQ_DB_DISARM_CQ | eqn, cqn,
    dev.kar + MTHCA_EQ_DOORBELL,
    }
    }
    static inline struct mthca_eqe *get_eqe(struct mthca_eq *eq, u32 entry)
    {
    pub MTHCA_EQ_ENTRY_SIZE: *mut *mut unsigned long off = (entry & (eq->nent - 1)),
    pub PAGE_SIZE: return eq->page_list[off / PAGE_SIZE].buf + off %,
    }
    static inline struct mthca_eqe *next_eqe_sw(struct mthca_eq *eq)
    {
    pub eqe: *mut mthca_eqe,
    pub eq->cons_index): eqe = get_eqe(eq,,
    pub eqe: return (MTHCA_EQ_ENTRY_OWNER_HW & eqe->owner) ? NULL :,
    }
#[no_mangle]
pub unsafe extern "C" fn set_eqe_hw(eqe: *mut mthca_eqe) {
    static inline void set_eqe_hw(struct mthca_eqe *eqe)
    {
    pub MTHCA_EQ_ENTRY_OWNER_HW: eqe->owner =,
    }
#[no_mangle]
unsafe extern "C" fn port_change(dev: *mut mthca_dev, port: c_int, active: c_int) {
    static void port_change(struct mthca_dev *dev, int port, int active)
    {
    pub record: ib_event,
    mthca_dbg(dev, "Port change to %s for port %d\n",
    pub port): active ? "active" : "down",,
    pub &dev->ib_dev: record.device =,
    pub IB_EVENT_PORT_ERR: record.event = active ? IB_EVENT_PORT_ACTIVE :,
    pub port: record.element.port_num =,
    }
#[no_mangle]
unsafe extern "C" fn mthca_eq_int(dev: *mut mthca_dev, eq: *mut mthca_eq) -> c_int {
    static int mthca_eq_int(struct mthca_dev *dev, struct mthca_eq *eq)
    {
    pub eqe: *mut mthca_eqe,
    pub disarm_cqn: c_int,
    pub 0: int eqes_found =,
    pub 0: int set_ci =,
    while ((eqe = next_eqe_sw(eq))) {
//
// Make sure we read EQ entry contents after we've
// checked the ownership bit.
//
    switch (eqe.type) {
    case MTHCA_EVENT_TYPE_COMP:
    pub 0xffffff: disarm_cqn = be32_to_cpu(eqe->event.comp.cqn) &,
    pub disarm_cqn): disarm_cq(dev, eq->eqn,,
    pub disarm_cqn): mthca_cq_completion(dev,,
    case MTHCA_EVENT_TYPE_PATH_MIG:
    mthca_qp_event(dev, be32_to_cpu(eqe.event.qp.qpn) & 0xffffff,
    case MTHCA_EVENT_TYPE_COMM_EST:
    mthca_qp_event(dev, be32_to_cpu(eqe.event.qp.qpn) & 0xffffff,
    case MTHCA_EVENT_TYPE_SQ_DRAINED:
    mthca_qp_event(dev, be32_to_cpu(eqe.event.qp.qpn) & 0xffffff,
    case MTHCA_EVENT_TYPE_SRQ_QP_LAST_WQE:
    mthca_qp_event(dev, be32_to_cpu(eqe.event.qp.qpn) & 0xffffff,
    case MTHCA_EVENT_TYPE_SRQ_LIMIT:
    mthca_srq_event(dev, be32_to_cpu(eqe.event.srq.srqn) & 0xffffff,
    case MTHCA_EVENT_TYPE_WQ_CATAS_ERROR:
    mthca_qp_event(dev, be32_to_cpu(eqe.event.qp.qpn) & 0xffffff,
    case MTHCA_EVENT_TYPE_PATH_MIG_FAILED:
    mthca_qp_event(dev, be32_to_cpu(eqe.event.qp.qpn) & 0xffffff,
    case MTHCA_EVENT_TYPE_WQ_INVAL_REQ_ERROR:
    mthca_qp_event(dev, be32_to_cpu(eqe.event.qp.qpn) & 0xffffff,
    case MTHCA_EVENT_TYPE_WQ_ACCESS_ERROR:
    mthca_qp_event(dev, be32_to_cpu(eqe.event.qp.qpn) & 0xffffff,
    case MTHCA_EVENT_TYPE_CMD:
    mthca_cmd_event(dev,
    be16_to_cpu(eqe.event.cmd.token),
    eqe.event.cmd.status,
    case MTHCA_EVENT_TYPE_PORT_CHANGE:
    port_change(dev,
    (be32_to_cpu(eqe.event.port_change.port) >> 28) & 3,
    pub 0x4): eqe->subtype ==,
    case MTHCA_EVENT_TYPE_CQ_ERROR:
    mthca_warn(dev, "CQ %s on CQN %06x\n",
    eqe.event.cq_err.syndrome == 1 ?
    "overrun" : "access violation",
    pub 0xffffff): be32_to_cpu(eqe->event.cq_err.cqn) &,
    mthca_cq_event(dev, be32_to_cpu(eqe.event.cq_err.cqn),
    case MTHCA_EVENT_TYPE_EQ_OVERFLOW:
    pub eq->eqn): mthca_warn(dev, "EQ overrun on EQN %d\n",,
    case MTHCA_EVENT_TYPE_EEC_CATAS_ERROR:
    case MTHCA_EVENT_TYPE_SRQ_CATAS_ERROR:
    case MTHCA_EVENT_TYPE_LOCAL_CATAS_ERROR:
    case MTHCA_EVENT_TYPE_ECC_DETECT:
    default:
    mthca_warn(dev, "Unhandled event %02x(%02x) on EQ %d\n",
    pub eq->eqn): eqe->type, eqe->subtype,,
    }
    pub 1: eqes_found =,
//
// The HCA will think the queue has overflowed if we
// don't tell it we've been processing events.  We
// create our EQs with MTHCA_NUM_SPARE_EQE extra
// entries, so we must update our consumer index at
// least that often.
//
    if (unlikely(set_ci >= MTHCA_NUM_SPARE_EQE)) {
//
// Conditional on hca_type is OK here because
// this is a rare case, not the fast path.
//
    pub eq->cons_index): set_eq_ci(dev, eq,,
    pub 0: set_ci =,
    }
    }
//
// Rely on caller to set consumer index so that we don't have
// to test hca_type in our interrupt handling fast path.
//
    pub eqes_found: return,
    }
#[no_mangle]
unsafe extern "C" fn mthca_tavor_interrupt(irq: c_int, dev_ptr: *mut c_void) -> irqreturn_t {
    static irqreturn_t mthca_tavor_interrupt(int irq, void *dev_ptr)
    {
    pub dev_ptr: *mut *mut mthca_dev dev =,
    pub ecr: u32,
    pub i: c_int,
    if (dev.eq_table.clr_mask)
    pub dev->eq_table.clr_int): writel(dev->eq_table.clr_mask,,
    pub 4): ecr = readl(dev->eq_regs.tavor.ecr_base +,
    if (!ecr)
    pub IRQ_NONE: return,
    writel(ecr, dev.eq_regs.tavor.ecr_base +
    pub 4): MTHCA_ECR_CLR_BASE - MTHCA_ECR_BASE +,
    pub ++i): for (i = 0; i < MTHCA_NUM_EQ;,
    if (ecr & dev.eq_table.eq[i].eqn_mask) {
    if (mthca_eq_int(dev, &dev.eq_table.eq[i]))
    tavor_set_eq_ci(dev, &dev.eq_table.eq[i],
    pub dev->eq_table.eq[i].eqn): tavor_eq_req_not(dev,,
    }
    pub IRQ_HANDLED: return,
    }
#[no_mangle]
unsafe extern "C" fn mthca_tavor_msi_x_interrupt(irq: c_int, eq_ptr: *mut c_void) -> irqreturn_t {
    static irqreturn_t mthca_tavor_msi_x_interrupt(int irq, void *eq_ptr)
    {
    pub eq_ptr: *mut *mut mthca_eq eq =,
    pub eq->dev: *mut *mut mthca_dev dev =,
    pub eq): mthca_eq_int(dev,,
    pub eq->cons_index): tavor_set_eq_ci(dev, eq,,
    pub eq->eqn): tavor_eq_req_not(dev,,
// MSI-X vectors always belong to us
    pub IRQ_HANDLED: return,
    }
#[no_mangle]
unsafe extern "C" fn mthca_arbel_interrupt(irq: c_int, dev_ptr: *mut c_void) -> irqreturn_t {
    static irqreturn_t mthca_arbel_interrupt(int irq, void *dev_ptr)
    {
    pub dev_ptr: *mut *mut mthca_dev dev =,
    pub 0: int work =,
    pub i: c_int,
    if (dev.eq_table.clr_mask)
    pub dev->eq_table.clr_int): writel(dev->eq_table.clr_mask,,
    pub ++i): for (i = 0; i < MTHCA_NUM_EQ;,
    if (mthca_eq_int(dev, &dev.eq_table.eq[i])) {
    pub 1: work =,
    arbel_set_eq_ci(dev, &dev.eq_table.eq[i],
    }
    pub dev->eq_table.arm_mask): arbel_eq_req_not(dev,,
    pub IRQ_RETVAL(work): return,
    }
#[no_mangle]
unsafe extern "C" fn mthca_arbel_msi_x_interrupt(irq: c_int, eq_ptr: *mut c_void) -> irqreturn_t {
    static irqreturn_t mthca_arbel_msi_x_interrupt(int irq, void *eq_ptr)
    {
    pub eq_ptr: *mut *mut mthca_eq eq =,
    pub eq->dev: *mut *mut mthca_dev dev =,
    pub eq): mthca_eq_int(dev,,
    pub eq->cons_index): arbel_set_eq_ci(dev, eq,,
    pub eq->eqn_mask): arbel_eq_req_not(dev,,
// MSI-X vectors always belong to us
    pub IRQ_HANDLED: return,
    }
    static int mthca_create_eq(struct mthca_dev *dev,
    int nent,
    u8 intr,
    struct mthca_eq *eq)
    {
    pub npages: c_int,
    pub NULL: *mut *mut u64 dma_list =,
    pub t: dma_addr_t,
    pub mailbox: *mut mthca_mailbox,
    pub eq_context: *mut mthca_eq_context,
    pub -ENOMEM: int err =,
    pub i: c_int,
    pub dev: eq->dev =,
    pub 2)): eq->nent = roundup_pow_of_two(max(nent,,
    pub PAGE_SIZE: *mut *mut npages = ALIGN(eq->nent  MTHCA_EQ_ENTRY_SIZE, PAGE_SIZE) /,
    pub npages): *mut *mut eq->page_list = kmalloc_objs(eq->page_list,,
    if (!eq.page_list)
    pub err_out: goto,
    pub ++i): for (i = 0; i < npages;,
    pub NULL: eq->page_list[i].buf =,
    pub GFP_KERNEL): *mut *mut dma_list = kmalloc_array(npages, sizeof(dma_list),,
    if (!dma_list)
    pub err_out_free: goto,
    pub GFP_KERNEL): mailbox = mthca_alloc_mailbox(dev,,
    if (IS_ERR(mailbox))
    pub err_out_free: goto,
    pub mailbox->buf: eq_context =,
    pub {: for (i = 0; i < npages; ++i),
    eq.page_list[i].buf = dma_alloc_coherent(&dev.pdev.dev,
    pub GFP_KERNEL): PAGE_SIZE, &t,,
    if (!eq.page_list[i].buf)
    pub err_out_free_pages: goto,
    pub t: dma_list[i] =,
    pub t): dma_unmap_addr_set(&eq->page_list[i], mapping,,
    }
    pub ++i): for (i = 0; i < eq->nent;,
    pub i)): set_eqe_hw(get_eqe(eq,,
    pub mthca_alloc(&dev->eq_table.alloc): eq->eqn =,
    if (eq.eqn == -1)
    pub err_out_free_pages: goto,
    err = mthca_mr_alloc_phys(dev, dev.driver_pd.pd_num,
    dma_list, PAGE_SHIFT, npages,
    0, npages * PAGE_SIZE,
    MTHCA_MPT_FLAG_LOCAL_WRITE |
    MTHCA_MPT_FLAG_LOCAL_READ,
    if (err)
    pub err_out_free_eq: goto,
    pub eq_context): *mut memset(eq_context, 0, sizeof,
    eq_context.flags           = cpu_to_be32(MTHCA_EQ_STATUS_OK   |
    MTHCA_EQ_OWNER_HW    |
    MTHCA_EQ_STATE_ARMED |
    if (mthca_is_memfree(dev))
    pub cpu_to_be32(MTHCA_EQ_STATE_ARBEL): eq_context->flags |=,
    pub 24): eq_context->logsize_usrpage = cpu_to_be32((ffs(eq->nent) - 1) <<,
    if (mthca_is_memfree(dev)) {
    pub cpu_to_be32(dev->driver_pd.pd_num): eq_context->arbel_pd =,
    } else {
    pub cpu_to_be32(dev->driver_uar.index): eq_context->logsize_usrpage |=,
    pub cpu_to_be32(dev->driver_pd.pd_num): eq_context->tavor_pd =,
    }
    pub intr: eq_context->intr =,
    pub cpu_to_be32(eq->mr.ibmr.lkey): eq_context->lkey =,
    pub eq->eqn): err = mthca_SW2HW_EQ(dev, mailbox,,
    if (err) {
    pub err): mthca_warn(dev, "SW2HW_EQ returned %d\n",,
    pub err_out_free_mr: goto,
    }
    pub mailbox): mthca_free_mailbox(dev,,
    pub eq->eqn): eq->eqn_mask = swab32(1 <<,
    pub 0: eq->cons_index =,
    pub eq->eqn_mask: dev->eq_table.arm_mask |=,
    mthca_dbg(dev, "Allocated EQ %d with %d entries\n",
    pub eq->nent): eq->eqn,,
    pub err: return,
    err_out_free_mr:
    pub &eq->mr): mthca_free_mr(dev,,
    err_out_free_eq:
    pub eq->eqn): mthca_free(&dev->eq_table.alloc,,
    err_out_free_pages:
    pub ++i): for (i = 0; i < npages;,
    if (eq.page_list[i].buf)
    dma_free_coherent(&dev.pdev.dev, PAGE_SIZE,
    eq.page_list[i].buf,
    dma_unmap_addr(&eq.page_list[i],
    pub mailbox): mthca_free_mailbox(dev,,
    err_out_free:
    err_out:
    pub err: return,
    }
    static void mthca_free_eq(struct mthca_dev *dev,
    struct mthca_eq *eq)
    {
    pub mailbox: *mut mthca_mailbox,
    pub err: c_int,
    int npages = (eq.nent * MTHCA_EQ_ENTRY_SIZE + PAGE_SIZE - 1) /
    pub i: c_int,
    pub GFP_KERNEL): mailbox = mthca_alloc_mailbox(dev,,
    if (IS_ERR(mailbox))
    pub eq->eqn): err = mthca_HW2SW_EQ(dev, mailbox,,
    if (err)
    pub err): mthca_warn(dev, "HW2SW_EQ returned %d\n",,
    pub ~eq->eqn_mask: dev->eq_table.arm_mask &=,
    if (0) {
    pub eq->eqn): mthca_dbg(dev, "Dumping EQ context %02x:\n",,
    pub {: for (i = 0; i < sizeof (struct mthca_eq_context) / 4; ++i),
    if (i % 4 == 0)
    pub 4): *mut *mut printk("[%02x] ", i,
    pub 4)): *mut *mut printk(" %08x", be32_to_cpup(mailbox->buf + i,
    if ((i + 1) % 4 == 0)
    }
    }
    pub &eq->mr): mthca_free_mr(dev,,
    pub ++i): for (i = 0; i < npages;,
    dma_free_coherent(&dev.pdev.dev, PAGE_SIZE,
    eq.page_list[i].buf,
    pub mapping)): dma_unmap_addr(&eq->page_list[i],,
    pub mailbox): mthca_free_mailbox(dev,,
    }
#[no_mangle]
unsafe extern "C" fn mthca_free_irqs(dev: *mut mthca_dev) {
    static void mthca_free_irqs(struct mthca_dev *dev)
    {
    pub i: c_int,
    if (dev.eq_table.have_irq)
    pub dev): free_irq(dev->pdev->irq,,
    pub ++i): for (i = 0; i < MTHCA_NUM_EQ;,
    if (dev.eq_table.eq[i].have_irq) {
    free_irq(dev.eq_table.eq[i].msi_x_vector,
    pub i): dev->eq_table.eq +,
    pub 0: dev->eq_table.eq[i].have_irq =,
    }
    }
    static int mthca_map_reg(struct mthca_dev *dev,
    unsigned long offset, unsigned long size,
    void __iomem **map)
    {
    pub 0): phys_addr_t base = pci_resource_start(dev->pdev,,
// map = ioremap(base + offset, size);
    if (!*map)
    pub -ENOMEM: return,
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn mthca_map_eq_regs(dev: *mut mthca_dev) -> c_int {
    static int mthca_map_eq_regs(struct mthca_dev *dev)
    {
    if (mthca_is_memfree(dev)) {
//
// We assume that the EQ arm and EQ set CI registers
// fall within the first BAR.  We can't trust the
// values firmware gives us, since those addresses are
// valid on the HCA's side of the PCI bus but not
// necessarily the host side.
//
    if (mthca_map_reg(dev, (pci_resource_len(dev.pdev, 0) - 1) &
    dev.fw.arbel.clr_int_base, MTHCA_CLR_INT_SIZE,
    &dev.clr_base)) {
    mthca_err(dev, "Couldn't map interrupt clear register, "
    pub -ENOMEM: return,
    }
//
// Add 4 because we limit ourselves to EQs 0 ... 31,
// so we only need the low word of the register.
//
    if (mthca_map_reg(dev, ((pci_resource_len(dev.pdev, 0) - 1) &
    dev.fw.arbel.eq_arm_base) + 4, 4,
    &dev.eq_regs.arbel.eq_arm)) {
    pub aborting.\n"): mthca_err(dev, "Couldn't map EQ arm register,,
    pub -ENOMEM: return,
    }
    if (mthca_map_reg(dev, (pci_resource_len(dev.pdev, 0) - 1) &
    dev.fw.arbel.eq_set_ci_base,
    MTHCA_EQ_SET_CI_SIZE,
    &dev.eq_regs.arbel.eq_set_ci_base)) {
    pub aborting.\n"): mthca_err(dev, "Couldn't map EQ CI register,,
    pub -ENOMEM: return,
    }
    } else {
    if (mthca_map_reg(dev, MTHCA_CLR_INT_BASE, MTHCA_CLR_INT_SIZE,
    &dev.clr_base)) {
    mthca_err(dev, "Couldn't map interrupt clear register, "
    pub -ENOMEM: return,
    }
    if (mthca_map_reg(dev, MTHCA_ECR_BASE,
    MTHCA_ECR_SIZE + MTHCA_ECR_CLR_SIZE,
    &dev.eq_regs.tavor.ecr_base)) {
    mthca_err(dev, "Couldn't map ecr register, "
    pub -ENOMEM: return,
    }
    }
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn mthca_unmap_eq_regs(dev: *mut mthca_dev) {
    static void mthca_unmap_eq_regs(struct mthca_dev *dev)
    {
    if (mthca_is_memfree(dev)) {
    } else {
    }
    }
#[no_mangle]
pub unsafe extern "C" fn mthca_map_eq_icm(dev: *mut mthca_dev, icm_virt: u64) -> c_int {
    int mthca_map_eq_icm(struct mthca_dev *dev, u64 icm_virt)
    {
    pub ret: c_int,
//
// We assume that mapping one page is enough for the whole EQ
// context table.  This is fine with all current HCAs, because
// we only use 32 EQs and each EQ uses 32 bytes of context
// memory, or 1 KB total.
//
    pub icm_virt: dev->eq_table.icm_virt =,
    pub alloc_page(GFP_HIGHUSER): dev->eq_table.icm_page =,
    if (!dev.eq_table.icm_page)
    pub -ENOMEM: return,
    dev.eq_table.icm_dma =
    dma_map_page(&dev.pdev.dev, dev.eq_table.icm_page, 0,
    pub DMA_BIDIRECTIONAL): PAGE_SIZE,,
    if (dma_mapping_error(&dev.pdev.dev, dev.eq_table.icm_dma)) {
    pub -ENOMEM: return,
    }
    pub icm_virt): ret = mthca_MAP_ICM_page(dev, dev->eq_table.icm_dma,,
    if (ret) {
    dma_unmap_page(&dev.pdev.dev, dev.eq_table.icm_dma,
    pub DMA_BIDIRECTIONAL): PAGE_SIZE,,
    }
    pub ret: return,
    }
#[no_mangle]
pub unsafe extern "C" fn mthca_unmap_eq_icm(dev: *mut mthca_dev) {
    void mthca_unmap_eq_icm(struct mthca_dev *dev)
    {
    pub 1): mthca_UNMAP_ICM(dev, dev->eq_table.icm_virt,,
    dma_unmap_page(&dev.pdev.dev, dev.eq_table.icm_dma, PAGE_SIZE,
    }
#[no_mangle]
pub unsafe extern "C" fn mthca_init_eq_table(dev: *mut mthca_dev) -> c_int {
    int mthca_init_eq_table(struct mthca_dev *dev)
    {
    pub err: c_int,
    pub intr: u8,
    pub i: c_int,
    err = mthca_alloc_init(&dev.eq_table.alloc,
    dev.limits.num_eqs,
    dev.limits.num_eqs - 1,
    if (err)
    pub err: return,
    pub mthca_map_eq_regs(dev): err =,
    if (err)
    pub err_out_free: goto,
    if (dev.mthca_flags & MTHCA_FLAG_MSI_X) {
    pub 0: dev->eq_table.clr_mask =,
    } else {
    dev.eq_table.clr_mask =
    pub 31)): swab32(1 << (dev->eq_table.inta_pin &,
    dev.eq_table.clr_int  = dev.clr_base +
    pub 0): (dev->eq_table.inta_pin < 32 ? 4 :,
    }
    pub 0: dev->eq_table.arm_mask =,
    pub dev->eq_table.inta_pin: intr =,
    err = mthca_create_eq(dev, dev.limits.num_cqs + MTHCA_NUM_SPARE_EQE,
    (dev.mthca_flags & MTHCA_FLAG_MSI_X) ? 128 : intr,
    if (err)
    pub err_out_unmap: goto,
    err = mthca_create_eq(dev, MTHCA_NUM_ASYNC_EQE + MTHCA_NUM_SPARE_EQE,
    (dev.mthca_flags & MTHCA_FLAG_MSI_X) ? 129 : intr,
    if (err)
    pub err_out_comp: goto,
    err = mthca_create_eq(dev, MTHCA_NUM_CMD_EQE + MTHCA_NUM_SPARE_EQE,
    (dev.mthca_flags & MTHCA_FLAG_MSI_X) ? 130 : intr,
    if (err)
    pub err_out_async: goto,
    if (dev.mthca_flags & MTHCA_FLAG_MSI_X) {
    static const char *eq_name[] = {
    [MTHCA_EQ_COMP]  = DRV_NAME "-comp",
    [MTHCA_EQ_ASYNC] = DRV_NAME "-async",
    [MTHCA_EQ_CMD]   = DRV_NAME "-cmd"
}

    for (i = 0; i < MTHCA_NUM_EQ; ++i) {
    snprintf(dev.eq_table.eq[i].irq_name,
    IB_DEVICE_NAME_MAX,
    "%s@pci:%s", eq_name[i],
    pci_name(dev.pdev));
    err = request_irq(dev.eq_table.eq[i].msi_x_vector,
    mthca_is_memfree(dev) ?
    mthca_arbel_msi_x_interrupt :
    mthca_tavor_msi_x_interrupt,
    0, dev.eq_table.eq[i].irq_name,
    dev.eq_table.eq + i);
    if (err)
    goto err_out_cmd;
    dev.eq_table.eq[i].have_irq = 1;
    }
    } else {
    snprintf(dev.eq_table.eq[0].irq_name, IB_DEVICE_NAME_MAX,
    DRV_NAME "@pci:%s", pci_name(dev.pdev));
    err = request_irq(dev.pdev.irq,
    mthca_is_memfree(dev) ?
    mthca_arbel_interrupt :
    mthca_tavor_interrupt,
    IRQF_SHARED, dev.eq_table.eq[0].irq_name, dev);
    if (err)
    goto err_out_cmd;
    dev.eq_table.have_irq = 1;
    }
    err = mthca_MAP_EQ(dev, async_mask(dev),
    0, dev.eq_table.eq[MTHCA_EQ_ASYNC].eqn);
    if (err)
    mthca_warn(dev, "MAP_EQ for async EQ %d failed (%d)\n",
    dev.eq_table.eq[MTHCA_EQ_ASYNC].eqn, err);
    err = mthca_MAP_EQ(dev, MTHCA_CMD_EVENT_MASK,
    0, dev.eq_table.eq[MTHCA_EQ_CMD].eqn);
    if (err)
    mthca_warn(dev, "MAP_EQ for cmd EQ %d failed (%d)\n",
    dev.eq_table.eq[MTHCA_EQ_CMD].eqn, err);
    for (i = 0; i < MTHCA_NUM_EQ; ++i)
    if (mthca_is_memfree(dev))
    arbel_eq_req_not(dev, dev.eq_table.eq[i].eqn_mask);
    else
    tavor_eq_req_not(dev, dev.eq_table.eq[i].eqn);
    return 0;
    err_out_cmd:
    mthca_free_irqs(dev);
    mthca_free_eq(dev, &dev.eq_table.eq[MTHCA_EQ_CMD]);
    err_out_async:
    mthca_free_eq(dev, &dev.eq_table.eq[MTHCA_EQ_ASYNC]);
    err_out_comp:
    mthca_free_eq(dev, &dev.eq_table.eq[MTHCA_EQ_COMP]);
    err_out_unmap:
    mthca_unmap_eq_regs(dev);
    err_out_free:
    mthca_alloc_cleanup(&dev.eq_table.alloc);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn mthca_cleanup_eq_table(dev: *mut mthca_dev) {
    void mthca_cleanup_eq_table(struct mthca_dev *dev)
    {
    int i;
    mthca_free_irqs(dev);
    mthca_MAP_EQ(dev, async_mask(dev),
    1, dev.eq_table.eq[MTHCA_EQ_ASYNC].eqn);
    mthca_MAP_EQ(dev, MTHCA_CMD_EVENT_MASK,
    1, dev.eq_table.eq[MTHCA_EQ_CMD].eqn);
    for (i = 0; i < MTHCA_NUM_EQ; ++i)
    mthca_free_eq(dev, &dev.eq_table.eq[i]);
    mthca_unmap_eq_regs(dev);
    mthca_alloc_cleanup(&dev.eq_table.alloc);
    }
