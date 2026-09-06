//! Automatically rewritten from C to Rust
//! Source: drivers/soc/fsl/qbman/bman.c
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


// Copyright 2008 - 2016 Freescale Semiconductor, Inc.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are met:
// * Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
// * Redistributions in binary form must reproduce the above copyright
// notice, this list of conditions and the following disclaimer in the
// documentation and/or other materials provided with the distribution.
// * Neither the name of Freescale Semiconductor nor the
// names of its contributors may be used to endorse or promote products
// derived from this software without specific prior written permission.
//
// ALTERNATIVELY, this software may be distributed under the terms of the
// GNU General Public License ("GPL") as published by the Free Software
// Foundation, either version 2 of that License or (at your option) any
// later version.
//
// THIS SOFTWARE IS PROVIDED BY Freescale Semiconductor ``AS IS'' AND ANY
// EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
// WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
// DISCLAIMED. IN NO EVENT SHALL Freescale Semiconductor BE LIABLE FOR ANY
// DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
// (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
// LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
// ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
// SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//

// Portal register assists

// Cache-inhibited register offsets
pub const BM_REG_RCR_PI_CINH: c_uint = 0x3000;
pub const BM_REG_RCR_CI_CINH: c_uint = 0x3100;
pub const BM_REG_RCR_ITR: c_uint = 0x3200;
pub const BM_REG_CFG: c_uint = 0x3300;

pub const BM_REG_ISR: c_uint = 0x3e00;
pub const BM_REG_IER: c_uint = 0x3e40;
pub const BM_REG_ISDR: c_uint = 0x3e80;
pub const BM_REG_IIR: c_uint = 0x3ec0;
// Cache-enabled register offsets
pub const BM_CL_CR: c_uint = 0x0000;
pub const BM_CL_RR0: c_uint = 0x0100;
pub const BM_CL_RR1: c_uint = 0x0140;
pub const BM_CL_RCR: c_uint = 0x1000;
pub const BM_CL_RCR_PI_CENA: c_uint = 0x3000;
pub const BM_CL_RCR_CI_CENA: c_uint = 0x3100;

// Cache-inhibited register offsets
pub const BM_REG_RCR_PI_CINH: c_uint = 0x0000;
pub const BM_REG_RCR_CI_CINH: c_uint = 0x0004;
pub const BM_REG_RCR_ITR: c_uint = 0x0008;
pub const BM_REG_CFG: c_uint = 0x0100;

pub const BM_REG_ISR: c_uint = 0x0e00;
pub const BM_REG_IER: c_uint = 0x0e04;
pub const BM_REG_ISDR: c_uint = 0x0e08;
pub const BM_REG_IIR: c_uint = 0x0e0c;
// Cache-enabled register offsets
pub const BM_CL_CR: c_uint = 0x0000;
pub const BM_CL_RR0: c_uint = 0x0100;
pub const BM_CL_RR1: c_uint = 0x0140;
pub const BM_CL_RCR: c_uint = 0x1000;
pub const BM_CL_RCR_PI_CENA: c_uint = 0x3000;
pub const BM_CL_RCR_CI_CENA: c_uint = 0x3100;

//
// Portal modes.
// Enum types;
// pmode == production mode
// cmode == consumption mode,
// Enum values use 3 letter codes. First letter matches the portal mode,
// remaining two letters indicate;
// ci == cache-inhibited portal register
// ce == cache-enabled portal register
// vb == in-band valid-bit (cache-enabled)
//
    enum bm_rcr_pmode {		/* matches BCSP_CFG::RPM */
    bm_rcr_pci = 0,		/* PI index, cache-inhibited */
    bm_rcr_pce = 1,		/* PI index, cache-enabled */
    bm_rcr_pvb = 2		/* valid-bit */
    };
    enum bm_rcr_cmode {		/* s/w-only */
    bm_rcr_cci,		/* CI index, cache-inhibited */
    bm_rcr_cce		/* CI index, cache-enabled */
    };
// --- Portal structures ---
pub const BM_RCR_SIZE: c_int = 8;
// Release Command
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bm_rcr_entry {
    union {
    struct {
    pub /: *mut *mut u8 _ncw_verb; / writes to this are non-coherent,
    pub /: *mut *mut u8 bpid; / used with BM_RCR_VERB_CMD_BPID_SINGLE,
    pub __reserved1: [u8; 62],
}

    struct bm_buffer bufs[8];
    };
    };
pub const BM_RCR_VERB_VBIT: c_uint = 0x80;
pub const BM_RCR_VERB_CMD_MASK: c_uint = 0x70	/* one of two values; */;
pub const BM_RCR_VERB_CMD_BPID_SINGLE: c_uint = 0x20;
pub const BM_RCR_VERB_CMD_BPID_MULTI: c_uint = 0x30;
pub const BM_RCR_VERB_BUFCOUNT_MASK: c_uint = 0x0f	/* values 1..8 */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bm_rcr {
    pub cursor: *mut *mut bm_rcr_entry ring,,
    pub vbit: u8 ci, available, ithresh,,

    pub busy: u32,
    pub pmode: enum bm_rcr_pmode,
    pub cmode: enum bm_rcr_cmode,

}

// MC (Management Command) command
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bm_mc_command {
    pub /: *mut *mut u8 _ncw_verb; / writes to this are non-coherent,
    pub /: *mut *mut u8 bpid; / used by acquire command,
    pub __reserved: [u8; 62],
}

pub const BM_MCC_VERB_VBIT: c_uint = 0x80;
pub const BM_MCC_VERB_CMD_MASK: c_uint = 0x70	/* where the verb contains; */;
pub const BM_MCC_VERB_CMD_ACQUIRE: c_uint = 0x10;
pub const BM_MCC_VERB_CMD_QUERY: c_uint = 0x40;
pub const BM_MCC_VERB_ACQUIRE_BUFCOUNT: c_uint = 0x0f	/* values 1..8 go here */;
// MC result, Acquire and Query Response
    union bm_mc_result {
    struct {
    u8 verb;
    u8 bpid;
    u8 __reserved[62];
    };
    struct bm_buffer bufs[8];
    };
pub const BM_MCR_VERB_VBIT: c_uint = 0x80;

pub const BM_MCR_VERB_CMD_ERR_INVALID: c_uint = 0x60;
pub const BM_MCR_VERB_CMD_ERR_ECC: c_uint = 0x70;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bm_mc {
    pub cr: *mut bm_mc_command,
    pub rr: *mut union bm_mc_result,
    pub vbit: u8 rridx,,

    enum {
// Can only be _mc_start()ed
    mc_idle,
// Can only be _mc_commit()ed or _mc_abort()ed
    mc_user,
// Can only be _mc_retry()ed
    mc_hw
    pub state: },

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bm_addr {
    pub /: *mut *mut *mut void ce; / cache-enabled,
    pub /: *mut *mut *mut __be32 ce_be; / Same as above but for direct access,
    pub /: *mut *mut *mut void __iomem ci; / cache-inhibited,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bm_portal {
    pub addr: bm_addr,
    pub rcr: bm_rcr,
    pub mc: bm_mc,
    pub ____cacheline_aligned: },
// Cache-inhibited register access.
#[no_mangle]
pub unsafe extern "C" fn bm_in(p: *mut bm_portal, offset: u32) -> u32 {
    static inline u32 bm_in(struct bm_portal *p, u32 offset)
    {
    pub offset): return ioread32be(p->addr.ci +,
    }
#[no_mangle]
pub unsafe extern "C" fn bm_out(p: *mut bm_portal, offset: u32, val: u32) {
    static inline void bm_out(struct bm_portal *p, u32 offset, u32 val)
    {
    pub offset): iowrite32be(val, p->addr.ci +,
    }
// Cache Enabled Portal Access
#[no_mangle]
pub unsafe extern "C" fn bm_cl_invalidate(p: *mut bm_portal, offset: u32) {
    static inline void bm_cl_invalidate(struct bm_portal *p, u32 offset)
    {
    pub offset): dpaa_invalidate(p->addr.ce +,
    }
#[no_mangle]
pub unsafe extern "C" fn bm_cl_touch_ro(p: *mut bm_portal, offset: u32) {
    static inline void bm_cl_touch_ro(struct bm_portal *p, u32 offset)
    {
    pub offset): dpaa_touch_ro(p->addr.ce +,
    }
#[no_mangle]
pub unsafe extern "C" fn bm_ce_in(p: *mut bm_portal, offset: u32) -> u32 {
    static inline u32 bm_ce_in(struct bm_portal *p, u32 offset)
    {
    pub (offset/4))): *mut *mut return be32_to_cpu((p->addr.ce_be +,
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bman_portal {
    pub p: bm_portal,
// interrupt sources processed by portal_isr(), configurable
    pub irq_sources: c_ulong,
// probing time config params for cpu-affine portals
    pub config: *const bm_portal_config,
    pub irqname: [c_char; MAX_IRQNAME],
}

    static cpumask_t affine_mask;
    static DEFINE_SPINLOCK(affine_mask_lock);
    static DEFINE_PER_CPU(struct bman_portal, bman_affine_portal);
    static inline struct bman_portal *get_affine_portal(void)
    {
    return &get_cpu_var(bman_affine_portal);
    }
#[no_mangle]
pub unsafe extern "C" fn put_affine_portal() {
    static inline void put_affine_portal(void)
    {
    put_cpu_var(bman_affine_portal);
    }
//
// This object type refers to a pool, it isn't *the* pool. There may be
// more than one such object per BMan buffer pool, eg. if different users of the
// pool are operating via different portals.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bman_pool {
// index of the buffer pool to encapsulate (0-63)
    pub bpid: u32,
// Used for hash-table admin when using depletion notifications.
    pub portal: *mut bman_portal,
    pub next: *mut bman_pool,
}

    static u32 poll_portal_slow(struct bman_portal *p, u32 is);
#[no_mangle]
unsafe extern "C" fn portal_isr(irq: c_int, ptr: *mut c_void) -> irqreturn_t {
    static irqreturn_t portal_isr(int irq, void *ptr)
    {
    struct bman_portal *p = ptr;
    struct bm_portal *portal = &p.p;
    let mut clear: u32 = p.irq_sources;
    let mut is: u32 = bm_in(portal, BM_REG_ISR) & p.irq_sources;
    if (unlikely(!is))
    return IRQ_NONE;
    clear |= poll_portal_slow(p, is);
    bm_out(portal, BM_REG_ISR, clear);
    return IRQ_HANDLED;
    }
// --- RCR API ---

// Bit-wise logic to wrap a ring pointer by clearing the "carry bit"
    static struct bm_rcr_entry *rcr_carryclear(struct bm_rcr_entry *p)
    {
    let mut addr: uintptr_t = (uintptr_t)p;
    addr &= ~RCR_CARRY;
    return (struct bm_rcr_entry *)addr;
    }

// Bit-wise logic to convert a ring pointer to a ring index
#[no_mangle]
unsafe extern "C" fn rcr_ptr2idx(e: *mut bm_rcr_entry) -> c_int {
    static int rcr_ptr2idx(struct bm_rcr_entry *e)
    {
    return ((uintptr_t)e >> RCR_SHIFT) & (BM_RCR_SIZE - 1);
    }

// Increment the 'cursor' ring pointer, taking 'vbit' into account
#[no_mangle]
pub unsafe extern "C" fn rcr_inc(rcr: *mut bm_rcr) {
    static inline void rcr_inc(struct bm_rcr *rcr)
    {
// increment to the next RCR pointer and handle overflow and 'vbit'
    struct bm_rcr_entry *partial = rcr.cursor + 1;
    rcr.cursor = rcr_carryclear(partial);
    if (partial != rcr.cursor)
    rcr.vbit ^= BM_RCR_VERB_VBIT;
    }
#[no_mangle]
unsafe extern "C" fn bm_rcr_get_avail(portal: *mut bm_portal) -> c_int {
    static int bm_rcr_get_avail(struct bm_portal *portal)
    {
    struct bm_rcr *rcr = &portal.rcr;
    return rcr.available;
    }
#[no_mangle]
unsafe extern "C" fn bm_rcr_get_fill(portal: *mut bm_portal) -> c_int {
    static int bm_rcr_get_fill(struct bm_portal *portal)
    {
    struct bm_rcr *rcr = &portal.rcr;
    return BM_RCR_SIZE - 1 - rcr.available;
    }
#[no_mangle]
unsafe extern "C" fn bm_rcr_set_ithresh(portal: *mut bm_portal, ithresh: u8) {
    static void bm_rcr_set_ithresh(struct bm_portal *portal, u8 ithresh)
    {
    struct bm_rcr *rcr = &portal.rcr;
    rcr.ithresh = ithresh;
    bm_out(portal, BM_REG_RCR_ITR, ithresh);
    }
#[no_mangle]
unsafe extern "C" fn bm_rcr_cce_prefetch(portal: *mut bm_portal) {
    static void bm_rcr_cce_prefetch(struct bm_portal *portal)
    {
    __maybe_unused struct bm_rcr *rcr = &portal.rcr;
    DPAA_ASSERT(rcr.cmode == bm_rcr_cce);
    bm_cl_touch_ro(portal, BM_CL_RCR_CI_CENA);
    }
#[no_mangle]
unsafe extern "C" fn bm_rcr_cce_update(portal: *mut bm_portal) -> u8 {
    static u8 bm_rcr_cce_update(struct bm_portal *portal)
    {
    struct bm_rcr *rcr = &portal.rcr;
    u8 diff, old_ci = rcr.ci;
    DPAA_ASSERT(rcr.cmode == bm_rcr_cce);
    rcr.ci = bm_ce_in(portal, BM_CL_RCR_CI_CENA) & (BM_RCR_SIZE - 1);
    bm_cl_invalidate(portal, BM_CL_RCR_CI_CENA);
    diff = dpaa_cyc_diff(BM_RCR_SIZE, old_ci, rcr.ci);
    rcr.available += diff;
    return diff;
    }
    static inline struct bm_rcr_entry *bm_rcr_start(struct bm_portal *portal)
    {
    struct bm_rcr *rcr = &portal.rcr;
    DPAA_ASSERT(!rcr.busy);
    if (!rcr.available)
    return core::ptr::null_mut();

    rcr.busy = 1;

    dpaa_zero(rcr.cursor);
    return rcr.cursor;
    }
#[no_mangle]
pub unsafe extern "C" fn bm_rcr_pvb_commit(portal: *mut bm_portal, myverb: u8) {
    static inline void bm_rcr_pvb_commit(struct bm_portal *portal, u8 myverb)
    {
    struct bm_rcr *rcr = &portal.rcr;
    struct bm_rcr_entry *rcursor;
    DPAA_ASSERT(rcr.busy);
    DPAA_ASSERT(rcr.pmode == bm_rcr_pvb);
    DPAA_ASSERT(rcr.available >= 1);
    dma_wmb();
    rcursor = rcr.cursor;
    rcursor._ncw_verb = myverb | rcr.vbit;
    dpaa_flush(rcursor);
    rcr_inc(rcr);
    rcr.available--;

    rcr.busy = 0;

    }
    static int bm_rcr_init(struct bm_portal *portal, enum bm_rcr_pmode pmode,
    enum bm_rcr_cmode cmode)
    {
    struct bm_rcr *rcr = &portal.rcr;
    u32 cfg;
    u8 pi;
    rcr.ring = portal.addr.ce + BM_CL_RCR;
    rcr.ci = bm_in(portal, BM_REG_RCR_CI_CINH) & (BM_RCR_SIZE - 1);
    pi = bm_in(portal, BM_REG_RCR_PI_CINH) & (BM_RCR_SIZE - 1);
    rcr.cursor = rcr.ring + pi;
    rcr.vbit = (bm_in(portal, BM_REG_RCR_PI_CINH) & BM_RCR_SIZE) ?
    BM_RCR_VERB_VBIT : 0;
    rcr.available = BM_RCR_SIZE - 1
    - dpaa_cyc_diff(BM_RCR_SIZE, rcr.ci, pi);
    rcr.ithresh = bm_in(portal, BM_REG_RCR_ITR);

    rcr.busy = 0;
    rcr.pmode = pmode;
    rcr.cmode = cmode;

    cfg = (bm_in(portal, BM_REG_CFG) & 0xffffffe0)
    | (pmode & 0x3); /* BCSP_CFG::RPM */
    bm_out(portal, BM_REG_CFG, cfg);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bm_rcr_finish(portal: *mut bm_portal) {
    static void bm_rcr_finish(struct bm_portal *portal)
    {

    struct bm_rcr *rcr = &portal.rcr;
    int i;
    DPAA_ASSERT(!rcr.busy);
    i = bm_in(portal, BM_REG_RCR_PI_CINH) & (BM_RCR_SIZE - 1);
    if (i != rcr_ptr2idx(rcr.cursor))
    pr_crit("losing uncommitted RCR entries\n");
    i = bm_in(portal, BM_REG_RCR_CI_CINH) & (BM_RCR_SIZE - 1);
    if (i != rcr.ci)
    pr_crit("missing existing RCR completions\n");
    if (rcr.ci != rcr_ptr2idx(rcr.cursor))
    pr_crit("RCR destroyed unquiesced\n");

    }
// --- Management command API ---
#[no_mangle]
unsafe extern "C" fn bm_mc_init(portal: *mut bm_portal) -> c_int {
    static int bm_mc_init(struct bm_portal *portal)
    {
    struct bm_mc *mc = &portal.mc;
    mc.cr = portal.addr.ce + BM_CL_CR;
    mc.rr = portal.addr.ce + BM_CL_RR0;
    mc.rridx = (mc.cr._ncw_verb & BM_MCC_VERB_VBIT) ?
    0 : 1;
    mc.vbit = mc.rridx ? BM_MCC_VERB_VBIT : 0;

    mc.state = mc_idle;

    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bm_mc_finish(portal: *mut bm_portal) {
    static void bm_mc_finish(struct bm_portal *portal)
    {

    struct bm_mc *mc = &portal.mc;
    DPAA_ASSERT(mc.state == mc_idle);
    if (mc.state != mc_idle)
    pr_crit("Losing incomplete MC command\n");

    }
    static inline struct bm_mc_command *bm_mc_start(struct bm_portal *portal)
    {
    struct bm_mc *mc = &portal.mc;
    DPAA_ASSERT(mc.state == mc_idle);

    mc.state = mc_user;

    dpaa_zero(mc.cr);
    return mc.cr;
    }
#[no_mangle]
pub unsafe extern "C" fn bm_mc_commit(portal: *mut bm_portal, myverb: u8) {
    static inline void bm_mc_commit(struct bm_portal *portal, u8 myverb)
    {
    struct bm_mc *mc = &portal.mc;
    union bm_mc_result *rr = mc.rr + mc.rridx;
    DPAA_ASSERT(mc.state == mc_user);
    dma_wmb();
    mc.cr._ncw_verb = myverb | mc.vbit;
    dpaa_flush(mc.cr);
    dpaa_invalidate_touch_ro(rr);

    mc.state = mc_hw;

    }
    static inline union bm_mc_result *bm_mc_result(struct bm_portal *portal)
    {
    struct bm_mc *mc = &portal.mc;
    union bm_mc_result *rr = mc.rr + mc.rridx;
    DPAA_ASSERT(mc.state == mc_hw);
//
// The inactive response register's verb byte always returns zero until
// its command is submitted and completed. This includes the valid-bit,
// in case you were wondering...
//
    if (!rr.verb) {
    dpaa_invalidate_touch_ro(rr);
    return core::ptr::null_mut();
    }
    mc.rridx ^= 1;
    mc.vbit ^= BM_MCC_VERB_VBIT;

    mc.state = mc_idle;

    return rr;
    }
    static inline int bm_mc_result_timeout(struct bm_portal *portal,
    union bm_mc_result **mcr)
    {
    let mut timeout: c_int = BM_MCR_TIMEOUT;
    do {
// mcr = bm_mc_result(portal);
    if (*mcr)
    break;
    udelay(1);
    } while (--timeout);
    return timeout;
    }
// Disable all BSCN interrupts for the portal
#[no_mangle]
unsafe extern "C" fn bm_isr_bscn_disable(portal: *mut bm_portal) {
    static void bm_isr_bscn_disable(struct bm_portal *portal)
    {
    bm_out(portal, BM_REG_SCN(0), 0);
    bm_out(portal, BM_REG_SCN(1), 0);
    }
    static int bman_create_portal(struct bman_portal *portal,
    const struct bm_portal_config *c)
    {
    struct bm_portal *p;
    int ret;
    p = &portal.p;
//
// prep the low-level portal struct with the mapped addresses from the
// config, everything that follows depends on it and "config" is more
// for (de)reference...
//
    p.addr.ce = c.addr_virt_ce;
    p.addr.ce_be = c.addr_virt_ce;
    p.addr.ci = c.addr_virt_ci;
    if (bm_rcr_init(p, bm_rcr_pvb, bm_rcr_cce)) {
    dev_err(c.dev, "RCR initialisation failed\n");
    goto fail_rcr;
    }
    if (bm_mc_init(p)) {
    dev_err(c.dev, "MC initialisation failed\n");
    goto fail_mc;
    }
//
// Default to all BPIDs disabled, we enable as required at
// run-time.
//
    bm_isr_bscn_disable(p);
// Write-to-clear any stale interrupt status bits
    bm_out(p, BM_REG_ISDR, 0xffffffff);
    portal.irq_sources = 0;
    bm_out(p, BM_REG_IER, 0);
    bm_out(p, BM_REG_ISR, 0xffffffff);
    snprintf(portal.irqname, MAX_IRQNAME, IRQNAME, c.cpu);
    if (request_irq(c.irq, portal_isr, 0, portal.irqname,	portal)) {
    dev_err(c.dev, "request_irq() failed\n");
    goto fail_irq;
    }
    if (dpaa_set_portal_irq_affinity(c.dev, c.irq, c.cpu))
    goto fail_affinity;
// Need RCR to be empty before continuing
    ret = bm_rcr_get_fill(p);
    if (ret) {
    dev_err(c.dev, "RCR unclean\n");
    goto fail_rcr_empty;
    }
// Success
    portal.config = c;
    bm_out(p, BM_REG_ISDR, 0);
    bm_out(p, BM_REG_IIR, 0);
    return 0;
    fail_rcr_empty:
    fail_affinity:
    free_irq(c.irq, portal);
    fail_irq:
    bm_mc_finish(p);
    fail_mc:
    bm_rcr_finish(p);
    fail_rcr:
    return -EIO;
    }
    struct bman_portal *bman_create_affine_portal(const struct bm_portal_config *c)
    {
    struct bman_portal *portal;
    int err;
    portal = &per_cpu(bman_affine_portal, c.cpu);
    err = bman_create_portal(portal, c);
    if (err)
    return core::ptr::null_mut();
    spin_lock(&affine_mask_lock);
    cpumask_set_cpu(c.cpu, &affine_mask);
    spin_unlock(&affine_mask_lock);
    return portal;
    }
#[no_mangle]
unsafe extern "C" fn poll_portal_slow(p: *mut bman_portal, is: u32) -> u32 {
    static u32 poll_portal_slow(struct bman_portal *p, u32 is)
    {
    let mut ret: u32 = is;
    if (is & BM_PIRQ_RCRI) {
    bm_rcr_cce_update(&p.p);
    bm_rcr_set_ithresh(&p.p, 0);
    bm_out(&p.p, BM_REG_ISR, BM_PIRQ_RCRI);
    is &= ~BM_PIRQ_RCRI;
    }
// There should be no status register bits left undefined
    DPAA_ASSERT(!is);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn bman_p_irqsource_add(p: *mut bman_portal, bits: u32) -> c_int {
    int bman_p_irqsource_add(struct bman_portal *p, u32 bits)
    {
    unsigned long irqflags;
    local_irq_save(irqflags);
    p.irq_sources |= bits & BM_PIRQ_VISIBLE;
    bm_out(&p.p, BM_REG_IER, p.irq_sources);
    local_irq_restore(irqflags);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn bm_shutdown_pool(bpid: u32) -> c_int {
    int bm_shutdown_pool(u32 bpid)
    {
    let mut err: c_int = 0;
    struct bm_mc_command *bm_cmd;
    union bm_mc_result *bm_res;
    struct bman_portal *p = get_affine_portal();
    while (1) {
// Acquire buffers until empty
    bm_cmd = bm_mc_start(&p.p);
    bm_cmd.bpid = bpid;
    bm_mc_commit(&p.p, BM_MCC_VERB_CMD_ACQUIRE | 1);
    if (!bm_mc_result_timeout(&p.p, &bm_res)) {
    pr_crit("BMan Acquire Command timedout\n");
    err = -ETIMEDOUT;
    goto done;
    }
    if (!(bm_res.verb & BM_MCR_VERB_ACQUIRE_BUFCOUNT)) {
// Pool is empty
    goto done;
    }
    }
    done:
    put_affine_portal();
    return err;
    }
    struct gen_pool *bm_bpalloc;
#[no_mangle]
unsafe extern "C" fn bm_alloc_bpid_range(result: *mut u32, count: u32) -> c_int {
    static int bm_alloc_bpid_range(u32 *result, u32 count)
    {
    unsigned long addr;
    addr = gen_pool_alloc(bm_bpalloc, count);
    if (!addr)
    return -ENOMEM;
// result = addr & ~DPAA_GENALLOC_OFF;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bm_release_bpid(bpid: u32) -> c_int {
    static int bm_release_bpid(u32 bpid)
    {
    int ret;
    ret = bm_shutdown_pool(bpid);
    if (ret) {
    pr_debug("BPID %d leaked\n", bpid);
    return ret;
    }
    gen_pool_free(bm_bpalloc, bpid | DPAA_GENALLOC_OFF, 1);
    return 0;
    }
    struct bman_pool *bman_new_pool(void)
    {
    struct bman_pool *pool = core::ptr::null_mut();
    u32 bpid;
    if (bm_alloc_bpid_range(&bpid, 1))
    return core::ptr::null_mut();
    pool = kmalloc_obj(*pool);
    if (!pool)
    goto err;
    pool.bpid = bpid;
    return pool;
    err:
    bm_release_bpid(bpid);
    return core::ptr::null_mut();
    }
    EXPORT_SYMBOL(bman_new_pool);
#[no_mangle]
pub unsafe extern "C" fn bman_free_pool(pool: *mut bman_pool) {
    void bman_free_pool(struct bman_pool *pool)
    {
    bm_release_bpid(pool.bpid);
    kfree(pool);
    }
    EXPORT_SYMBOL(bman_free_pool);
#[no_mangle]
pub unsafe extern "C" fn bman_get_bpid(pool: *const bman_pool) -> c_int {
    int bman_get_bpid(const struct bman_pool *pool)
    {
    return pool.bpid;
    }
    EXPORT_SYMBOL(bman_get_bpid);
#[no_mangle]
unsafe extern "C" fn update_rcr_ci(p: *mut bman_portal, avail: c_int) {
    static void update_rcr_ci(struct bman_portal *p, int avail)
    {
    if (avail)
    bm_rcr_cce_prefetch(&p.p);
    else
    bm_rcr_cce_update(&p.p);
    }
#[no_mangle]
pub unsafe extern "C" fn bman_release(pool: *mut bman_pool, bufs: *const bm_buffer, num: u8) -> c_int {
    int bman_release(struct bman_pool *pool, const struct bm_buffer *bufs, u8 num)
    {
    struct bman_portal *p;
    struct bm_rcr_entry *r;
    unsigned long irqflags;
    int avail, timeout = 1000; /* 1ms */
    let mut i: c_int = num - 1;
    DPAA_ASSERT(num > 0 && num <= 8);
    do {
    p = get_affine_portal();
    local_irq_save(irqflags);
    avail = bm_rcr_get_avail(&p.p);
    if (avail < 2)
    update_rcr_ci(p, avail);
    r = bm_rcr_start(&p.p);
    local_irq_restore(irqflags);
    put_affine_portal();
    if (likely(r))
    break;
    udelay(1);
    } while (--timeout);
    if (unlikely(!timeout))
    return -ETIMEDOUT;
    p = get_affine_portal();
    local_irq_save(irqflags);
//
// we can copy all but the first entry, as this can trigger badness
// with the valid-bit
//
    bm_buffer_set64(r.bufs, bm_buffer_get64(bufs));
    bm_buffer_set_bpid(r.bufs, pool.bpid);
    if (i)
    memcpy(&r.bufs[1], &bufs[1], i * sizeof(bufs[0]));
    bm_rcr_pvb_commit(&p.p, BM_RCR_VERB_CMD_BPID_SINGLE |
    (num & BM_RCR_VERB_BUFCOUNT_MASK));
    local_irq_restore(irqflags);
    put_affine_portal();
    return 0;
    }
    EXPORT_SYMBOL(bman_release);
#[no_mangle]
pub unsafe extern "C" fn bman_acquire(pool: *mut bman_pool, bufs: *mut bm_buffer, num: u8) -> c_int {
    int bman_acquire(struct bman_pool *pool, struct bm_buffer *bufs, u8 num)
    {
    struct bman_portal *p = get_affine_portal();
    struct bm_mc_command *mcc;
    union bm_mc_result *mcr;
    int ret;
    DPAA_ASSERT(num > 0 && num <= 8);
    mcc = bm_mc_start(&p.p);
    mcc.bpid = pool.bpid;
    bm_mc_commit(&p.p, BM_MCC_VERB_CMD_ACQUIRE |
    (num & BM_MCC_VERB_ACQUIRE_BUFCOUNT));
    if (!bm_mc_result_timeout(&p.p, &mcr)) {
    put_affine_portal();
    pr_crit("BMan Acquire Timeout\n");
    return -ETIMEDOUT;
    }
    ret = mcr.verb & BM_MCR_VERB_ACQUIRE_BUFCOUNT;
    if (bufs)
    memcpy(&bufs[0], &mcr.bufs[0], num * sizeof(bufs[0]));
    put_affine_portal();
    if (ret != num)
    ret = -ENOMEM;
    return ret;
    }
    EXPORT_SYMBOL(bman_acquire);
    const struct bm_portal_config *
    bman_get_bm_portal_config(const struct bman_portal *portal)
    {
    return portal.config;
    }
