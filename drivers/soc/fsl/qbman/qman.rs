//! Automatically rewritten from C to Rust
//! Source: drivers/soc/fsl/qbman/qman.c
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

pub const DQRR_MAXFILL: c_int = 15;

pub const QMAN_POLL_LIMIT: c_int = 32;
pub const QMAN_PIRQ_DQRR_ITHRESH: c_int = 12;
pub const QMAN_DQRR_IT_MAX: c_int = 15;
pub const QMAN_ITP_MAX: c_uint = 0xFFF;
pub const QMAN_PIRQ_MR_ITHRESH: c_int = 4;
pub const QMAN_PIRQ_IPERIOD: c_int = 100;
// Portal register assists

// Cache-inhibited register offsets
pub const QM_REG_EQCR_PI_CINH: c_uint = 0x3000;
pub const QM_REG_EQCR_CI_CINH: c_uint = 0x3040;
pub const QM_REG_EQCR_ITR: c_uint = 0x3080;
pub const QM_REG_DQRR_PI_CINH: c_uint = 0x3100;
pub const QM_REG_DQRR_CI_CINH: c_uint = 0x3140;
pub const QM_REG_DQRR_ITR: c_uint = 0x3180;
pub const QM_REG_DQRR_DCAP: c_uint = 0x31C0;
pub const QM_REG_DQRR_SDQCR: c_uint = 0x3200;
pub const QM_REG_DQRR_VDQCR: c_uint = 0x3240;
pub const QM_REG_DQRR_PDQCR: c_uint = 0x3280;
pub const QM_REG_MR_PI_CINH: c_uint = 0x3300;
pub const QM_REG_MR_CI_CINH: c_uint = 0x3340;
pub const QM_REG_MR_ITR: c_uint = 0x3380;
pub const QM_REG_CFG: c_uint = 0x3500;
pub const QM_REG_ISR: c_uint = 0x3600;
pub const QM_REG_IER: c_uint = 0x3640;
pub const QM_REG_ISDR: c_uint = 0x3680;
pub const QM_REG_IIR: c_uint = 0x36C0;
pub const QM_REG_ITPR: c_uint = 0x3740;
// Cache-enabled register offsets
pub const QM_CL_EQCR: c_uint = 0x0000;
pub const QM_CL_DQRR: c_uint = 0x1000;
pub const QM_CL_MR: c_uint = 0x2000;
pub const QM_CL_EQCR_PI_CENA: c_uint = 0x3000;
pub const QM_CL_EQCR_CI_CENA: c_uint = 0x3040;
pub const QM_CL_DQRR_PI_CENA: c_uint = 0x3100;
pub const QM_CL_DQRR_CI_CENA: c_uint = 0x3140;
pub const QM_CL_MR_PI_CENA: c_uint = 0x3300;
pub const QM_CL_MR_CI_CENA: c_uint = 0x3340;
pub const QM_CL_CR: c_uint = 0x3800;
pub const QM_CL_RR0: c_uint = 0x3900;
pub const QM_CL_RR1: c_uint = 0x3940;

// Cache-inhibited register offsets
pub const QM_REG_EQCR_PI_CINH: c_uint = 0x0000;
pub const QM_REG_EQCR_CI_CINH: c_uint = 0x0004;
pub const QM_REG_EQCR_ITR: c_uint = 0x0008;
pub const QM_REG_DQRR_PI_CINH: c_uint = 0x0040;
pub const QM_REG_DQRR_CI_CINH: c_uint = 0x0044;
pub const QM_REG_DQRR_ITR: c_uint = 0x0048;
pub const QM_REG_DQRR_DCAP: c_uint = 0x0050;
pub const QM_REG_DQRR_SDQCR: c_uint = 0x0054;
pub const QM_REG_DQRR_VDQCR: c_uint = 0x0058;
pub const QM_REG_DQRR_PDQCR: c_uint = 0x005c;
pub const QM_REG_MR_PI_CINH: c_uint = 0x0080;
pub const QM_REG_MR_CI_CINH: c_uint = 0x0084;
pub const QM_REG_MR_ITR: c_uint = 0x0088;
pub const QM_REG_CFG: c_uint = 0x0100;
pub const QM_REG_ISR: c_uint = 0x0e00;
pub const QM_REG_IER: c_uint = 0x0e04;
pub const QM_REG_ISDR: c_uint = 0x0e08;
pub const QM_REG_IIR: c_uint = 0x0e0c;
pub const QM_REG_ITPR: c_uint = 0x0e14;
// Cache-enabled register offsets
pub const QM_CL_EQCR: c_uint = 0x0000;
pub const QM_CL_DQRR: c_uint = 0x1000;
pub const QM_CL_MR: c_uint = 0x2000;
pub const QM_CL_EQCR_PI_CENA: c_uint = 0x3000;
pub const QM_CL_EQCR_CI_CENA: c_uint = 0x3100;
pub const QM_CL_DQRR_PI_CENA: c_uint = 0x3200;
pub const QM_CL_DQRR_CI_CENA: c_uint = 0x3300;
pub const QM_CL_MR_PI_CENA: c_uint = 0x3400;
pub const QM_CL_MR_CI_CENA: c_uint = 0x3500;
pub const QM_CL_CR: c_uint = 0x3800;
pub const QM_CL_RR0: c_uint = 0x3900;
pub const QM_CL_RR1: c_uint = 0x3940;

//
// BTW, the drivers (and h/w programming model) already obtain the required
// synchronisation for portal accesses and data-dependencies. Use of barrier()s
// or other order-preserving primitives simply degrade performance. Hence the
// use of the __raw_*() interfaces, which simply ensure that the compiler treats
// the portal registers as volatile
//
// Cache-enabled ring access

//
// Portal modes.
// Enum types;
// pmode == production mode
// cmode == consumption mode,
// dmode == h/w dequeue mode.
// Enum values use 3 letter codes. First letter matches the portal mode,
// remaining two letters indicate;
// ci == cache-inhibited portal register
// ce == cache-enabled portal register
// vb == in-band valid-bit (cache-enabled)
// dc == DCA (Discrete Consumption Acknowledgment), DQRR-only
// As for "enum qm_dqrr_dmode", it should be self-explanatory.
//
    enum qm_eqcr_pmode {		/* matches QCSP_CFG::EPM */
    qm_eqcr_pci = 0,	/* PI index, cache-inhibited */
    qm_eqcr_pce = 1,	/* PI index, cache-enabled */
    qm_eqcr_pvb = 2		/* valid-bit */
    };
    enum qm_dqrr_dmode {		/* matches QCSP_CFG::DP */
    qm_dqrr_dpush = 0,	/* SDQCR  + VDQCR */
    qm_dqrr_dpull = 1	/* PDQCR */
    };
    enum qm_dqrr_pmode {		/* s/w-only */
    qm_dqrr_pci,		/* reads DQRR_PI_CINH */
    qm_dqrr_pce,		/* reads DQRR_PI_CENA */
    qm_dqrr_pvb		/* reads valid-bit */
    };
    enum qm_dqrr_cmode {		/* matches QCSP_CFG::DCM */
    qm_dqrr_cci = 0,	/* CI index, cache-inhibited */
    qm_dqrr_cce = 1,	/* CI index, cache-enabled */
    qm_dqrr_cdc = 2		/* Discrete Consumption Acknowledgment */
    };
    enum qm_mr_pmode {		/* s/w-only */
    qm_mr_pci,		/* reads MR_PI_CINH */
    qm_mr_pce,		/* reads MR_PI_CENA */
    qm_mr_pvb		/* reads valid-bit */
    };
    enum qm_mr_cmode {		/* matches QCSP_CFG::MM */
    qm_mr_cci = 0,		/* CI index, cache-inhibited */
    qm_mr_cce = 1		/* CI index, cache-enabled */
    };
// --- Portal structures ---
pub const QM_EQCR_SIZE: c_int = 8;
pub const QM_DQRR_SIZE: c_int = 16;
pub const QM_MR_SIZE: c_int = 8;
// "Enqueue Command"
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qm_eqcr_entry {
    pub /: *mut *mut u8 _ncw_verb; / writes to this are non-coherent,
    pub dca: u8,
    pub seqnum: __be16,
    pub __reserved: [u8; 4],
    pub /: *mut *mut __be32 fqid; / 24-bit,
    pub tag: __be32,
    pub fd: qm_fd,
    pub __reserved3: [u8; 32],
    pub __aligned(8): } __packed,
pub const QM_EQCR_VERB_VBIT: c_uint = 0x80;
pub const QM_EQCR_VERB_CMD_MASK: c_uint = 0x61	/* but only one value; */;
pub const QM_EQCR_VERB_CMD_ENQUEUE: c_uint = 0x01;
pub const QM_EQCR_SEQNUM_NESN: c_uint = 0x8000	/* Advance NESN */;
pub const QM_EQCR_SEQNUM_NLIS: c_uint = 0x4000	/* More fragments to come */;
pub const QM_EQCR_SEQNUM_SEQMASK: c_uint = 0x3fff	/* sequence number goes here */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qm_eqcr {
    pub cursor: *mut *mut qm_eqcr_entry ring,,
    pub vbit: u8 ci, available, ithresh,,

    pub busy: u32,
    pub pmode: enum qm_eqcr_pmode,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qm_dqrr {
    pub cursor: *const *const qm_dqrr_entry ring,,
    pub vbit: u8 pi, ci, fill, ithresh,,

    pub dmode: enum qm_dqrr_dmode,
    pub pmode: enum qm_dqrr_pmode,
    pub cmode: enum qm_dqrr_cmode,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qm_mr {
    pub cursor: *mut *mut union qm_mr_entry ring,,
    pub vbit: u8 pi, ci, fill, ithresh,,

    pub pmode: enum qm_mr_pmode,
    pub cmode: enum qm_mr_cmode,

}

// MC (Management Command) command
// "FQ" command layout
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qm_mcc_fq {
    pub _ncw_verb: u8,
    pub __reserved1: [u8; 3],
    pub /: *mut *mut __be32 fqid; / 24-bit,
    pub __reserved2: [u8; 56],
    pub __packed: },
// "CGR" command layout
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qm_mcc_cgr {
    pub _ncw_verb: u8,
    pub __reserved1: [u8; 30],
    pub cgid: u8,
    pub __reserved2: [u8; 32],
}

pub const QM_MCC_VERB_VBIT: c_uint = 0x80;
pub const QM_MCC_VERB_MASK: c_uint = 0x7f	/* where the verb contains; */;
pub const QM_MCC_VERB_INITFQ_PARKED: c_uint = 0x40;
pub const QM_MCC_VERB_INITFQ_SCHED: c_uint = 0x41;
pub const QM_MCC_VERB_QUERYFQ: c_uint = 0x44;
pub const QM_MCC_VERB_QUERYFQ_NP: c_uint = 0x45	/* "non-programmable" fields */;
pub const QM_MCC_VERB_QUERYWQ: c_uint = 0x46;
pub const QM_MCC_VERB_QUERYWQ_DEDICATED: c_uint = 0x47;
pub const QM_MCC_VERB_ALTER_SCHED: c_uint = 0x48	/* Schedule FQ */;
pub const QM_MCC_VERB_ALTER_FE: c_uint = 0x49	/* Force Eligible FQ */;
pub const QM_MCC_VERB_ALTER_RETIRE: c_uint = 0x4a	/* Retire FQ */;
pub const QM_MCC_VERB_ALTER_OOS: c_uint = 0x4b	/* Take FQ out of service */;
pub const QM_MCC_VERB_ALTER_FQXON: c_uint = 0x4d	/* FQ XON */;
pub const QM_MCC_VERB_ALTER_FQXOFF: c_uint = 0x4e	/* FQ XOFF */;
pub const QM_MCC_VERB_INITCGR: c_uint = 0x50;
pub const QM_MCC_VERB_MODIFYCGR: c_uint = 0x51;
pub const QM_MCC_VERB_CGRTESTWRITE: c_uint = 0x52;
pub const QM_MCC_VERB_QUERYCGR: c_uint = 0x58;
pub const QM_MCC_VERB_QUERYCONGESTION: c_uint = 0x59;
    union qm_mc_command {
    struct {
    u8 _ncw_verb; /* writes to this are non-coherent */
    u8 __reserved[63];
    };
    struct qm_mcc_initfq initfq;
    struct qm_mcc_initcgr initcgr;
    struct qm_mcc_fq fq;
    struct qm_mcc_cgr cgr;
    };
// MC (Management Command) result
// "Query FQ"
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qm_mcr_queryfq {
    pub verb: u8,
    pub result: u8,
    pub __reserved1: [u8; 8],
    pub /: *mut *mut qm_fqd fqd; / the FQD fields are here,
    pub __reserved2: [u8; 30],
    pub __packed: },
// "Alter FQ State Commands"
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qm_mcr_alterfq {
    pub verb: u8,
    pub result: u8,
    pub /: *mut *mut u8 fqs; / Frame Queue Status,
    pub __reserved1: [u8; 61],
}

pub const QM_MCR_VERB_RRID: c_uint = 0x80;

pub const QM_MCR_RESULT_NULL: c_uint = 0x00;
pub const QM_MCR_RESULT_OK: c_uint = 0xf0;
pub const QM_MCR_RESULT_ERR_FQID: c_uint = 0xf1;
pub const QM_MCR_RESULT_ERR_FQSTATE: c_uint = 0xf2;
pub const QM_MCR_RESULT_ERR_NOTEMPTY: c_uint = 0xf3	/* OOS fails if FQ is !empty */;
pub const QM_MCR_RESULT_ERR_BADCHANNEL: c_uint = 0xf4;
pub const QM_MCR_RESULT_PENDING: c_uint = 0xf8;
pub const QM_MCR_RESULT_ERR_BADCOMMAND: c_uint = 0xff;
pub const QM_MCR_FQS_ORLPRESENT: c_uint = 0x02	/* ORL fragments to come */;
pub const QM_MCR_FQS_NOTEMPTY: c_uint = 0x01	/* FQ has enqueued frames */;

    union qm_mc_result {
    struct {
    u8 verb;
    u8 result;
    u8 __reserved1[62];
    };
    struct qm_mcr_queryfq queryfq;
    struct qm_mcr_alterfq alterfq;
    struct qm_mcr_querycgr querycgr;
    struct qm_mcr_querycongestion querycongestion;
    struct qm_mcr_querywq querywq;
    struct qm_mcr_queryfq_np queryfq_np;
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qm_mc {
    pub cr: *mut union qm_mc_command,
    pub rr: *mut union qm_mc_result,
    pub vbit: u8 rridx,,

    enum {
// Can be _mc_start()ed
    qman_mc_idle,
// Can be _mc_commit()ed or _mc_abort()ed
    qman_mc_user,
// Can only be _mc_retry()ed
    qman_mc_hw
    pub state: },

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qm_addr {
    pub /: *mut *mut *mut void ce; / cache-enabled,
    pub /: *mut *mut *mut __be32 ce_be; / same value as above but for direct access,
    pub /: *mut *mut *mut void __iomem ci; / cache-inhibited,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qm_portal {
//
// In the non-CONFIG_FSL_DPAA_CHECKING case, the following stuff up to
// and including 'mc' fits within a cacheline (yay!). The 'config' part
// is setup-only, so isn't a cause for a concern. In other words, don't
// rearrange this structure on a whim, there be dragons ...
//
    pub addr: qm_addr,
    pub eqcr: qm_eqcr,
    pub dqrr: qm_dqrr,
    pub mr: qm_mr,
    pub mc: qm_mc,
    pub ____cacheline_aligned: },
// Cache-inhibited register access.
#[no_mangle]
pub unsafe extern "C" fn qm_in(p: *mut qm_portal, offset: u32) -> u32 {
    static inline u32 qm_in(struct qm_portal *p, u32 offset)
    {
    pub offset): return ioread32be(p->addr.ci +,
    }
#[no_mangle]
pub unsafe extern "C" fn qm_out(p: *mut qm_portal, offset: u32, val: u32) {
    static inline void qm_out(struct qm_portal *p, u32 offset, u32 val)
    {
    pub offset): iowrite32be(val, p->addr.ci +,
    }
// Cache Enabled Portal Access
#[no_mangle]
pub unsafe extern "C" fn qm_cl_invalidate(p: *mut qm_portal, offset: u32) {
    static inline void qm_cl_invalidate(struct qm_portal *p, u32 offset)
    {
    pub offset): dpaa_invalidate(p->addr.ce +,
    }
#[no_mangle]
pub unsafe extern "C" fn qm_cl_touch_ro(p: *mut qm_portal, offset: u32) {
    static inline void qm_cl_touch_ro(struct qm_portal *p, u32 offset)
    {
    pub offset): dpaa_touch_ro(p->addr.ce +,
    }
#[no_mangle]
pub unsafe extern "C" fn qm_ce_in(p: *mut qm_portal, offset: u32) -> u32 {
    static inline u32 qm_ce_in(struct qm_portal *p, u32 offset)
    {
    pub (offset/4))): *mut *mut return be32_to_cpu((p->addr.ce_be +,
    }
// --- EQCR API ---

// Bit-wise logic to wrap a ring pointer by clearing the "carry bit"
    static struct qm_eqcr_entry *eqcr_carryclear(struct qm_eqcr_entry *p)
    {
    pub (uintptr_t)p: uintptr_t addr =,
    pub ~EQCR_CARRY: addr &=,
    pub )addr: *mut return (struct qm_eqcr_entry,
    }
// Bit-wise logic to convert a ring pointer to a ring index
#[no_mangle]
unsafe extern "C" fn eqcr_ptr2idx(e: *mut qm_eqcr_entry) -> c_int {
    static int eqcr_ptr2idx(struct qm_eqcr_entry *e)
    {
    pub 1): return ((uintptr_t)e >> EQCR_SHIFT) & (QM_EQCR_SIZE -,
    }
// Increment the 'cursor' ring pointer, taking 'vbit' into account
#[no_mangle]
pub unsafe extern "C" fn eqcr_inc(eqcr: *mut qm_eqcr) {
    static inline void eqcr_inc(struct qm_eqcr *eqcr)
    {
// increment to the next EQCR pointer and handle overflow and 'vbit'
    pub 1: *mut *mut qm_eqcr_entry partial = eqcr->cursor +,
    pub eqcr_carryclear(partial): eqcr->cursor =,
    if (partial != eqcr.cursor)
    pub QM_EQCR_VERB_VBIT: eqcr->vbit ^=,
    }
    static inline int qm_eqcr_init(struct qm_portal *portal,
    enum qm_eqcr_pmode pmode,
    unsigned int eq_stash_thresh,
    int eq_stash_prio)
    {
    pub &portal->eqcr: *mut *mut qm_eqcr eqcr =,
    pub cfg: u32,
    pub pi: u8,
    pub QM_CL_EQCR: eqcr->ring = portal->addr.ce +,
    pub 1): eqcr->ci = qm_in(portal, QM_REG_EQCR_CI_CINH) & (QM_EQCR_SIZE -,
    pub QM_CL_EQCR_CI_CENA): qm_cl_invalidate(portal,,
    pub 1): pi = qm_in(portal, QM_REG_EQCR_PI_CINH) & (QM_EQCR_SIZE -,
    pub pi: eqcr->cursor = eqcr->ring +,
    eqcr.vbit = (qm_in(portal, QM_REG_EQCR_PI_CINH) & QM_EQCR_SIZE) ?
    pub 0: QM_EQCR_VERB_VBIT :,
    eqcr.available = QM_EQCR_SIZE - 1 -
    pub pi): dpaa_cyc_diff(QM_EQCR_SIZE, eqcr->ci,,
    pub QM_REG_EQCR_ITR): eqcr->ithresh = qm_in(portal,,

    pub 0: eqcr->busy =,
    pub pmode: eqcr->pmode =,

    cfg = (qm_in(portal, QM_REG_CFG) & 0x00ffffff) |
    (eq_stash_thresh << 28) | /* QCSP_CFG: EST */
    (eq_stash_prio << 26) | /* QCSP_CFG: EP */
    pub /: *mut *mut ((pmode & 0x3) << 24); / QCSP_CFG::EPM,
    pub cfg): qm_out(portal, QM_REG_CFG,,
    pub 0: return,
    }
#[no_mangle]
pub unsafe extern "C" fn qm_eqcr_finish(portal: *mut qm_portal) {
    static inline void qm_eqcr_finish(struct qm_portal *portal)
    {
    pub &portal->eqcr: *mut *mut qm_eqcr eqcr =,
    pub 1): u8 pi = qm_in(portal, QM_REG_EQCR_PI_CINH) & (QM_EQCR_SIZE -,
    pub 1): u8 ci = qm_in(portal, QM_REG_EQCR_CI_CINH) & (QM_EQCR_SIZE -,
    if (pi != eqcr_ptr2idx(eqcr.cursor))
    pub entries\n"): pr_crit("losing uncommitted EQCR,
    if (ci != eqcr.ci)
    pub completions\n"): pr_crit("missing existing EQCR,
    if (eqcr.ci != eqcr_ptr2idx(eqcr.cursor))
    pub unquiesced\n"): pr_crit("EQCR destroyed,
    }
    static inline struct qm_eqcr_entry *qm_eqcr_start_no_stash(struct qm_portal
// portal)
    {
    pub &portal->eqcr: *mut *mut qm_eqcr eqcr =,
    if (!eqcr.available)
    pub NULL: return,

    pub 1: eqcr->busy =,

    pub eqcr->cursor: return,
    }
    static inline struct qm_eqcr_entry *qm_eqcr_start_stash(struct qm_portal
// portal)
    {
    pub &portal->eqcr: *mut *mut qm_eqcr eqcr =,
    pub old_ci: u8 diff,,
    if (!eqcr.available) {
    pub eqcr->ci: old_ci =,
    eqcr.ci = qm_ce_in(portal, QM_CL_EQCR_CI_CENA) &
    pub 1): (QM_EQCR_SIZE -,
    pub eqcr->ci): diff = dpaa_cyc_diff(QM_EQCR_SIZE, old_ci,,
    pub diff: eqcr->available +=,
    if (!diff)
    pub NULL: return,
    }

    pub 1: eqcr->busy =,

    pub eqcr->cursor: return,
    }
#[no_mangle]
pub unsafe extern "C" fn eqcr_commit_checks(eqcr: *mut qm_eqcr) {
    static inline void eqcr_commit_checks(struct qm_eqcr *eqcr)
    {
    pub ~QM_FQID_MASK)): DPAA_ASSERT(!(be32_to_cpu(eqcr->cursor->fqid) &,
    pub 1): DPAA_ASSERT(eqcr->available >=,
    }
#[no_mangle]
pub unsafe extern "C" fn qm_eqcr_pvb_commit(portal: *mut qm_portal, myverb: u8) {
    static inline void qm_eqcr_pvb_commit(struct qm_portal *portal, u8 myverb)
    {
    pub &portal->eqcr: *mut *mut qm_eqcr eqcr =,
    pub eqcursor: *mut qm_eqcr_entry,
    pub qm_eqcr_pvb): DPAA_ASSERT(eqcr->pmode ==,
    pub eqcr->cursor: eqcursor =,
    pub eqcr->vbit: eqcursor->_ncw_verb = myverb |,

    pub 0: eqcr->busy =,

    }
#[no_mangle]
pub unsafe extern "C" fn qm_eqcr_cce_prefetch(portal: *mut qm_portal) {
    static inline void qm_eqcr_cce_prefetch(struct qm_portal *portal)
    {
    pub QM_CL_EQCR_CI_CENA): qm_cl_touch_ro(portal,,
    }
#[no_mangle]
pub unsafe extern "C" fn qm_eqcr_cce_update(portal: *mut qm_portal) -> u8 {
    static inline u8 qm_eqcr_cce_update(struct qm_portal *portal)
    {
    pub &portal->eqcr: *mut *mut qm_eqcr eqcr =,
    pub eqcr->ci: u8 diff, old_ci =,
    pub 1): eqcr->ci = qm_ce_in(portal, QM_CL_EQCR_CI_CENA) & (QM_EQCR_SIZE -,
    pub QM_CL_EQCR_CI_CENA): qm_cl_invalidate(portal,,
    pub eqcr->ci): diff = dpaa_cyc_diff(QM_EQCR_SIZE, old_ci,,
    pub diff: eqcr->available +=,
    pub diff: return,
    }
#[no_mangle]
pub unsafe extern "C" fn qm_eqcr_set_ithresh(portal: *mut qm_portal, ithresh: u8) {
    static inline void qm_eqcr_set_ithresh(struct qm_portal *portal, u8 ithresh)
    {
    pub &portal->eqcr: *mut *mut qm_eqcr eqcr =,
    pub ithresh: eqcr->ithresh =,
    pub ithresh): qm_out(portal, QM_REG_EQCR_ITR,,
    }
#[no_mangle]
pub unsafe extern "C" fn qm_eqcr_get_avail(portal: *mut qm_portal) -> u8 {
    static inline u8 qm_eqcr_get_avail(struct qm_portal *portal)
    {
    pub &portal->eqcr: *mut *mut qm_eqcr eqcr =,
    pub eqcr->available: return,
    }
#[no_mangle]
pub unsafe extern "C" fn qm_eqcr_get_fill(portal: *mut qm_portal) -> u8 {
    static inline u8 qm_eqcr_get_fill(struct qm_portal *portal)
    {
    pub &portal->eqcr: *mut *mut qm_eqcr eqcr =,
    pub eqcr->available: return QM_EQCR_SIZE - 1 -,
    }
// --- DQRR API ---

    static const struct qm_dqrr_entry *dqrr_carryclear(
    const struct qm_dqrr_entry *p)
    {
    pub (uintptr_t)p: uintptr_t addr =,
    pub ~DQRR_CARRY: addr &=,
    pub )addr: *const return (struct qm_dqrr_entry,
    }
#[no_mangle]
pub unsafe extern "C" fn dqrr_ptr2idx(e: *const qm_dqrr_entry) -> c_int {
    static inline int dqrr_ptr2idx(const struct qm_dqrr_entry *e)
    {
    pub 1): return ((uintptr_t)e >> DQRR_SHIFT) & (QM_DQRR_SIZE -,
    }
    static const struct qm_dqrr_entry *dqrr_inc(const struct qm_dqrr_entry *e)
    {
    pub 1): return dqrr_carryclear(e +,
    }
#[no_mangle]
pub unsafe extern "C" fn qm_dqrr_set_maxfill(portal: *mut qm_portal, mf: u8) {
    static inline void qm_dqrr_set_maxfill(struct qm_portal *portal, u8 mf)
    {
    qm_out(portal, QM_REG_CFG, (qm_in(portal, QM_REG_CFG) & 0xff0fffff) |
    pub 20)): ((mf & (QM_DQRR_SIZE - 1)) <<,
    }
    static inline int qm_dqrr_init(struct qm_portal *portal,
    const struct qm_portal_config *config,
    enum qm_dqrr_dmode dmode,
    enum qm_dqrr_pmode pmode,
    enum qm_dqrr_cmode cmode, u8 max_fill)
    {
    pub &portal->dqrr: *mut *mut qm_dqrr dqrr =,
    pub cfg: u32,
// Make sure the DQRR will be idle when we enable
    pub 0): qm_out(portal, QM_REG_DQRR_SDQCR,,
    pub 0): qm_out(portal, QM_REG_DQRR_VDQCR,,
    pub 0): qm_out(portal, QM_REG_DQRR_PDQCR,,
    pub QM_CL_DQRR: dqrr->ring = portal->addr.ce +,
    pub 1): dqrr->pi = qm_in(portal, QM_REG_DQRR_PI_CINH) & (QM_DQRR_SIZE -,
    pub 1): dqrr->ci = qm_in(portal, QM_REG_DQRR_CI_CINH) & (QM_DQRR_SIZE -,
    pub dqrr->ci: dqrr->cursor = dqrr->ring +,
    pub dqrr->pi): dqrr->fill = dpaa_cyc_diff(QM_DQRR_SIZE, dqrr->ci,,
    dqrr.vbit = (qm_in(portal, QM_REG_DQRR_PI_CINH) & QM_DQRR_SIZE) ?
    pub 0: QM_DQRR_VERB_VBIT :,
    pub QM_REG_DQRR_ITR): dqrr->ithresh = qm_in(portal,,

    pub dmode: dqrr->dmode =,
    pub pmode: dqrr->pmode =,
    pub cmode: dqrr->cmode =,

// Invalidate every ring entry before beginning
    pub cfg++): for (cfg = 0; cfg < QM_DQRR_SIZE;,
    pub cfg)): dpaa_invalidate(qm_cl(dqrr->ring,,
    cfg = (qm_in(portal, QM_REG_CFG) & 0xff000f00) |
    ((max_fill & (QM_DQRR_SIZE - 1)) << 20) | /* DQRR_MF */
    ((dmode & 1) << 18) |			/* DP */
    ((cmode & 3) << 16) |			/* DCM */
    0xa0 |					/* RE+SE */
    (0 ? 0x40 : 0) |			/* Ignore RP */
    pub /: *mut *mut (0 ? 0x10 : 0); / Ignore SP,
    pub cfg): qm_out(portal, QM_REG_CFG,,
    pub max_fill): qm_dqrr_set_maxfill(portal,,
    pub 0: return,
    }
#[no_mangle]
pub unsafe extern "C" fn qm_dqrr_finish(portal: *mut qm_portal) {
    static inline void qm_dqrr_finish(struct qm_portal *portal)
    {

    pub &portal->dqrr: *mut *mut qm_dqrr dqrr =,
    if (dqrr.cmode != qm_dqrr_cdc &&
    dqrr.ci != dqrr_ptr2idx(dqrr.cursor))
    pub entries\n"): pr_crit("Ignoring completed DQRR,

    }
    static inline const struct qm_dqrr_entry *qm_dqrr_current(
    struct qm_portal *portal)
    {
    pub &portal->dqrr: *mut *mut qm_dqrr dqrr =,
    if (!dqrr.fill)
    pub NULL: return,
    pub dqrr->cursor: return,
    }
#[no_mangle]
pub unsafe extern "C" fn qm_dqrr_next(portal: *mut qm_portal) -> u8 {
    static inline u8 qm_dqrr_next(struct qm_portal *portal)
    {
    pub &portal->dqrr: *mut *mut qm_dqrr dqrr =,
    pub dqrr_inc(dqrr->cursor): dqrr->cursor =,
    pub --dqrr->fill: return,
    }
#[no_mangle]
pub unsafe extern "C" fn qm_dqrr_pvb_update(portal: *mut qm_portal) {
    static inline void qm_dqrr_pvb_update(struct qm_portal *portal)
    {
    pub &portal->dqrr: *mut *mut qm_dqrr dqrr =,
    pub dqrr->pi): *mut *mut qm_dqrr_entry res = qm_cl(dqrr->ring,,
    pub qm_dqrr_pvb): DPAA_ASSERT(dqrr->pmode ==,

//
// If PAMU is not available we need to invalidate the cache.
// When PAMU is available the cache is updated by stash
//

    if ((res.verb & QM_DQRR_VERB_VBIT) == dqrr.vbit) {
    pub 1): dqrr->pi = (dqrr->pi + 1) & (QM_DQRR_SIZE -,
    if (!dqrr.pi)
    pub QM_DQRR_VERB_VBIT: dqrr->vbit ^=,
    }
    }
    static inline void qm_dqrr_cdc_consume_1ptr(struct qm_portal *portal,
    const struct qm_dqrr_entry *dq,
    int park)
    {
    pub &portal->dqrr: *mut *mut __maybe_unused struct qm_dqrr dqrr =,
    pub dqrr_ptr2idx(dq): int idx =,
    pub qm_dqrr_cdc): DPAA_ASSERT(dqrr->cmode ==,
    pub dq): DPAA_ASSERT((dqrr->ring + idx) ==,
    pub QM_DQRR_SIZE): DPAA_ASSERT(idx <,
    qm_out(portal, QM_REG_DQRR_DCAP, (0 << 8) | /* DQRR_DCAP::S */
    ((park ? 1 : 0) << 6) |		    /* DQRR_DCAP::PK */
    pub /: *mut *mut idx); / DQRR_DCAP::DCAP_CI,
    }
#[no_mangle]
pub unsafe extern "C" fn qm_dqrr_cdc_consume_n(portal: *mut qm_portal, bitmask: u32) {
    static inline void qm_dqrr_cdc_consume_n(struct qm_portal *portal, u32 bitmask)
    {
    pub &portal->dqrr: *mut *mut __maybe_unused struct qm_dqrr dqrr =,
    pub qm_dqrr_cdc): DPAA_ASSERT(dqrr->cmode ==,
    qm_out(portal, QM_REG_DQRR_DCAP, (1 << 8) | /* DQRR_DCAP::S */
    pub /: *mut *mut (bitmask << 16)); / DQRR_DCAP::DCAP_CI,
    }
#[no_mangle]
pub unsafe extern "C" fn qm_dqrr_sdqcr_set(portal: *mut qm_portal, sdqcr: u32) {
    static inline void qm_dqrr_sdqcr_set(struct qm_portal *portal, u32 sdqcr)
    {
    pub sdqcr): qm_out(portal, QM_REG_DQRR_SDQCR,,
    }
#[no_mangle]
pub unsafe extern "C" fn qm_dqrr_vdqcr_set(portal: *mut qm_portal, vdqcr: u32) {
    static inline void qm_dqrr_vdqcr_set(struct qm_portal *portal, u32 vdqcr)
    {
    pub vdqcr): qm_out(portal, QM_REG_DQRR_VDQCR,,
    }
#[no_mangle]
pub unsafe extern "C" fn qm_dqrr_set_ithresh(portal: *mut qm_portal, ithresh: u8) -> c_int {
    static inline int qm_dqrr_set_ithresh(struct qm_portal *portal, u8 ithresh)
    {
    if (ithresh > QMAN_DQRR_IT_MAX)
    pub -EINVAL: return,
    pub ithresh): qm_out(portal, QM_REG_DQRR_ITR,,
    pub 0: return,
    }
// --- MR API ---

    static union qm_mr_entry *mr_carryclear(union qm_mr_entry *p)
    {
    pub (uintptr_t)p: uintptr_t addr =,
    pub ~MR_CARRY: addr &=,
    pub )addr: *mut return (union qm_mr_entry,
    }
#[no_mangle]
pub unsafe extern "C" fn mr_ptr2idx(e: *const union qm_mr_entry) -> c_int {
    static inline int mr_ptr2idx(const union qm_mr_entry *e)
    {
    pub 1): return ((uintptr_t)e >> MR_SHIFT) & (QM_MR_SIZE -,
    }
    static inline union qm_mr_entry *mr_inc(union qm_mr_entry *e)
    {
    pub 1): return mr_carryclear(e +,
    }
    static inline int qm_mr_init(struct qm_portal *portal, enum qm_mr_pmode pmode,
    enum qm_mr_cmode cmode)
    {
    pub &portal->mr: *mut *mut qm_mr mr =,
    pub cfg: u32,
    pub QM_CL_MR: mr->ring = portal->addr.ce +,
    pub 1): mr->pi = qm_in(portal, QM_REG_MR_PI_CINH) & (QM_MR_SIZE -,
    pub 1): mr->ci = qm_in(portal, QM_REG_MR_CI_CINH) & (QM_MR_SIZE -,
    pub mr->ci: mr->cursor = mr->ring +,
    pub mr->pi): mr->fill = dpaa_cyc_diff(QM_MR_SIZE, mr->ci,,
    mr.vbit = (qm_in(portal, QM_REG_MR_PI_CINH) & QM_MR_SIZE)
    pub 0: ? QM_MR_VERB_VBIT :,
    pub QM_REG_MR_ITR): mr->ithresh = qm_in(portal,,

    pub pmode: mr->pmode =,
    pub cmode: mr->cmode =,

    cfg = (qm_in(portal, QM_REG_CFG) & 0xfffff0ff) |
    pub /: *mut *mut ((cmode & 1) << 8); / QCSP_CFG:MM,
    pub cfg): qm_out(portal, QM_REG_CFG,,
    pub 0: return,
    }
#[no_mangle]
pub unsafe extern "C" fn qm_mr_finish(portal: *mut qm_portal) {
    static inline void qm_mr_finish(struct qm_portal *portal)
    {
    pub &portal->mr: *mut *mut qm_mr mr =,
    if (mr.ci != mr_ptr2idx(mr.cursor))
    pub entries\n"): pr_crit("Ignoring completed MR,
    }
    static inline const union qm_mr_entry *qm_mr_current(struct qm_portal *portal)
    {
    pub &portal->mr: *mut *mut qm_mr mr =,
    if (!mr.fill)
    pub NULL: return,
    pub mr->cursor: return,
    }
#[no_mangle]
pub unsafe extern "C" fn qm_mr_next(portal: *mut qm_portal) -> c_int {
    static inline int qm_mr_next(struct qm_portal *portal)
    {
    pub &portal->mr: *mut *mut qm_mr mr =,
    pub mr_inc(mr->cursor): mr->cursor =,
    pub --mr->fill: return,
    }
#[no_mangle]
pub unsafe extern "C" fn qm_mr_pvb_update(portal: *mut qm_portal) {
    static inline void qm_mr_pvb_update(struct qm_portal *portal)
    {
    pub &portal->mr: *mut *mut qm_mr mr =,
    pub mr->pi): *mut *mut union qm_mr_entry res = qm_cl(mr->ring,,
    pub qm_mr_pvb): DPAA_ASSERT(mr->pmode ==,
    if ((res.verb & QM_MR_VERB_VBIT) == mr.vbit) {
    pub 1): mr->pi = (mr->pi + 1) & (QM_MR_SIZE -,
    if (!mr.pi)
    pub QM_MR_VERB_VBIT: mr->vbit ^=,
    pub mr_inc(res): res =,
    }
    }
#[no_mangle]
pub unsafe extern "C" fn qm_mr_cci_consume(portal: *mut qm_portal, num: u8) {
    static inline void qm_mr_cci_consume(struct qm_portal *portal, u8 num)
    {
    pub &portal->mr: *mut *mut qm_mr mr =,
    pub qm_mr_cci): DPAA_ASSERT(mr->cmode ==,
    pub 1): mr->ci = (mr->ci + num) & (QM_MR_SIZE -,
    pub mr->ci): qm_out(portal, QM_REG_MR_CI_CINH,,
    }
#[no_mangle]
pub unsafe extern "C" fn qm_mr_cci_consume_to_current(portal: *mut qm_portal) {
    static inline void qm_mr_cci_consume_to_current(struct qm_portal *portal)
    {
    pub &portal->mr: *mut *mut qm_mr mr =,
    pub qm_mr_cci): DPAA_ASSERT(mr->cmode ==,
    pub mr_ptr2idx(mr->cursor): mr->ci =,
    pub mr->ci): qm_out(portal, QM_REG_MR_CI_CINH,,
    }
#[no_mangle]
pub unsafe extern "C" fn qm_mr_set_ithresh(portal: *mut qm_portal, ithresh: u8) {
    static inline void qm_mr_set_ithresh(struct qm_portal *portal, u8 ithresh)
    {
    pub ithresh): qm_out(portal, QM_REG_MR_ITR,,
    }
// --- Management command API ---
#[no_mangle]
pub unsafe extern "C" fn qm_mc_init(portal: *mut qm_portal) -> c_int {
    static inline int qm_mc_init(struct qm_portal *portal)
    {
    pub rr1: u8 rr0,,
    pub &portal->mc: *mut *mut qm_mc mc =,
    pub QM_CL_CR: mc->cr = portal->addr.ce +,
    pub QM_CL_RR0: mc->rr = portal->addr.ce +,
//
// The expected valid bit polarity for the next CR command is 0
// if RR1 contains a valid response, and is 1 if RR0 contains a
// valid response. If both RR contain all 0, this indicates either
// that no command has been executed since reset (in which case the
// expected valid bit polarity is 1)
//
    pub mc->rr->verb: rr0 =,
    pub (mc->rr+1)->verb: rr1 =,
    if ((rr0 == 0 && rr1 == 0) || rr0 != 0)
    pub 1: mc->rridx =,
    else
    pub 0: mc->rridx =,
    pub 0: mc->vbit = mc->rridx ? QM_MCC_VERB_VBIT :,

    pub qman_mc_idle: mc->state =,

    pub 0: return,
    }
#[no_mangle]
pub unsafe extern "C" fn qm_mc_finish(portal: *mut qm_portal) {
    static inline void qm_mc_finish(struct qm_portal *portal)
    {

    pub &portal->mc: *mut *mut qm_mc mc =,
    pub qman_mc_idle): DPAA_ASSERT(mc->state ==,
    if (mc.state != qman_mc_idle)
    pub command\n"): pr_crit("Losing incomplete MC,

    }
    static inline union qm_mc_command *qm_mc_start(struct qm_portal *portal)
    {
    pub &portal->mc: *mut *mut qm_mc mc =,
    pub qman_mc_idle): DPAA_ASSERT(mc->state ==,

    pub qman_mc_user: mc->state =,

    pub mc->cr: return,
    }
#[no_mangle]
pub unsafe extern "C" fn qm_mc_commit(portal: *mut qm_portal, myverb: u8) {
    static inline void qm_mc_commit(struct qm_portal *portal, u8 myverb)
    {
    pub &portal->mc: *mut *mut qm_mc mc =,
    pub mc->rridx: *mut *mut union qm_mc_result rr = mc->rr +,
    pub qman_mc_user): DPAA_ASSERT(mc->state ==,
    pub mc->vbit: mc->cr->_ncw_verb = myverb |,

    pub qman_mc_hw: mc->state =,

    }
    static inline union qm_mc_result *qm_mc_result(struct qm_portal *portal)
    {
    pub &portal->mc: *mut *mut qm_mc mc =,
    pub mc->rridx: *mut *mut union qm_mc_result rr = mc->rr +,
    pub qman_mc_hw): DPAA_ASSERT(mc->state ==,
//
// The inactive response register's verb byte always returns zero until
// its command is submitted and completed. This includes the valid-bit,
// in case you were wondering...
//
    if (!rr.verb) {
    pub NULL: return,
    }
    pub 1: mc->rridx ^=,
    pub QM_MCC_VERB_VBIT: mc->vbit ^=,

    pub qman_mc_idle: mc->state =,

    pub rr: return,
    }
    static inline int qm_mc_result_timeout(struct qm_portal *portal,
    union qm_mc_result **mcr)
    {
    pub QM_MCR_TIMEOUT: int timeout =,
    do {
// mcr = qm_mc_result(portal);
    if (*mcr)
    pub (--timeout): } while,
    pub timeout: return,
    }
#[no_mangle]
pub unsafe extern "C" fn fq_set(fq: *mut qman_fq, mask: u32) {
    static inline void fq_set(struct qman_fq *fq, u32 mask)
    {
    pub mask: fq->flags |=,
    }
#[no_mangle]
pub unsafe extern "C" fn fq_clear(fq: *mut qman_fq, mask: u32) {
    static inline void fq_clear(struct qman_fq *fq, u32 mask)
    {
    pub ~mask: fq->flags &=,
    }
#[no_mangle]
pub unsafe extern "C" fn fq_isset(fq: *mut qman_fq, mask: u32) -> c_int {
    static inline int fq_isset(struct qman_fq *fq, u32 mask)
    {
    pub mask: return fq->flags &,
    }
#[no_mangle]
pub unsafe extern "C" fn fq_isclear(fq: *mut qman_fq, mask: u32) -> c_int {
    static inline int fq_isclear(struct qman_fq *fq, u32 mask)
    {
    pub mask): return !(fq->flags &,
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qman_portal {
    pub p: qm_portal,
// PORTAL_BITS_*** - dynamic, strictly internal
    pub bits: c_ulong,
// interrupt sources processed by portal_isr(), configurable
    pub irq_sources: c_ulong,
    pub use_eqcr_ci_stashing: u32,
// only 1 volatile dequeue at a time
    pub vdqcr_owned: *mut qman_fq,
    pub sdqcr: u32,
// probing time config params for cpu-affine portals
    pub config: *const qm_portal_config,
// 2-element array. cgrs[0] is mask, cgrs[1] is snapshot.
    pub cgrs: *mut qman_cgrs,
// linked-list of CSCN handlers.
    pub cgr_cbs: list_head,
// list lock
    pub cgr_lock: raw_spinlock_t,
    pub congestion_work: work_struct,
    pub mr_work: work_struct,
    pub irqname: [c_char; MAX_IRQNAME],
}

    static cpumask_t affine_mask;
    static DEFINE_SPINLOCK(affine_mask_lock);
    static u16 affine_channels[NR_CPUS];
    static DEFINE_PER_CPU(struct qman_portal, qman_affine_portal);
    struct qman_portal *affine_portals[NR_CPUS];
    static inline struct qman_portal *get_affine_portal(void)
    {
    return &get_cpu_var(qman_affine_portal);
    }
#[no_mangle]
pub unsafe extern "C" fn put_affine_portal() {
    static inline void put_affine_portal(void)
    {
    put_cpu_var(qman_affine_portal);
    }
    static inline struct qman_portal *get_portal_for_channel(u16 channel)
    {
    int i;
    for (i = 0; i < num_possible_cpus(); i++) {
    if (affine_portals[i] &&
    affine_portals[i].config.channel == channel)
    return affine_portals[i];
    }
    return core::ptr::null_mut();
    }
    static struct workqueue_struct *qm_portal_wq;
#[no_mangle]
pub unsafe extern "C" fn qman_dqrr_set_ithresh(portal: *mut qman_portal, ithresh: u8) -> c_int {
    int qman_dqrr_set_ithresh(struct qman_portal *portal, u8 ithresh)
    {
    int res;
    if (!portal)
    return -EINVAL;
    res = qm_dqrr_set_ithresh(&portal.p, ithresh);
    if (res)
    return res;
    portal.p.dqrr.ithresh = ithresh;
    return 0;
    }
    EXPORT_SYMBOL(qman_dqrr_set_ithresh);
#[no_mangle]
pub unsafe extern "C" fn qman_dqrr_get_ithresh(portal: *mut qman_portal, ithresh: *mut u8) {
    void qman_dqrr_get_ithresh(struct qman_portal *portal, u8 *ithresh)
    {
    if (portal && ithresh)
// ithresh = qm_in(&portal->p, QM_REG_DQRR_ITR);
    }
    EXPORT_SYMBOL(qman_dqrr_get_ithresh);
#[no_mangle]
pub unsafe extern "C" fn qman_portal_get_iperiod(portal: *mut qman_portal, iperiod: *mut u32) {
    void qman_portal_get_iperiod(struct qman_portal *portal, u32 *iperiod)
    {
    if (portal && iperiod)
// iperiod = qm_in(&portal->p, QM_REG_ITPR);
    }
    EXPORT_SYMBOL(qman_portal_get_iperiod);
#[no_mangle]
pub unsafe extern "C" fn qman_portal_set_iperiod(portal: *mut qman_portal, iperiod: u32) -> c_int {
    int qman_portal_set_iperiod(struct qman_portal *portal, u32 iperiod)
    {
    if (!portal || iperiod > QMAN_ITP_MAX)
    return -EINVAL;
    qm_out(&portal.p, QM_REG_ITPR, iperiod);
    return 0;
    }
    EXPORT_SYMBOL(qman_portal_set_iperiod);
#[no_mangle]
pub unsafe extern "C" fn qman_wq_alloc() -> c_int {
    int qman_wq_alloc(void)
    {
    qm_portal_wq = alloc_workqueue("qman_portal_wq", WQ_PERCPU, 1);
    if (!qm_portal_wq)
    return -ENOMEM;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn qman_enable_irqs() {
    void qman_enable_irqs(void)
    {
    int i;
    for (i = 0; i < num_possible_cpus(); i++) {
    if (affine_portals[i]) {
    qm_out(&affine_portals[i].p, QM_REG_ISR, 0xffffffff);
    qm_out(&affine_portals[i].p, QM_REG_IIR, 0);
    }
    }
    }
//
// This is what everything can wait on, even if it migrates to a different cpu
// to the one whose affine portal it is waiting on.
//
    static DECLARE_WAIT_QUEUE_HEAD(affine_queue);
    static struct qman_fq **fq_table;
    static u32 num_fqids;
#[no_mangle]
pub unsafe extern "C" fn qman_alloc_fq_table(_num_fqids: u32) -> c_int {
    int qman_alloc_fq_table(u32 _num_fqids)
    {
    num_fqids = _num_fqids;
    fq_table = vzalloc(array3_size(sizeof(struct qman_fq *),
    num_fqids, 2));
    if (!fq_table)
    return -ENOMEM;
    pr_debug("Allocated fq lookup table at %p, entry count %u\n",
    fq_table, num_fqids * 2);
    return 0;
    }
    static struct qman_fq *idx_to_fq(u32 idx)
    {
    struct qman_fq *fq;

    if (WARN_ON(idx >= num_fqids * 2))
    return core::ptr::null_mut();

    fq = fq_table[idx];
    DPAA_ASSERT(!fq || idx == fq.idx);
    return fq;
    }
//
// Only returns full-service fq objects, not enqueue-only
// references (QMAN_FQ_FLAG_NO_MODIFY).
//
    static struct qman_fq *fqid_to_fq(u32 fqid)
    {
    return idx_to_fq(fqid * 2);
    }
    static struct qman_fq *tag_to_fq(u32 tag)
    {

    return idx_to_fq(tag);

    return (struct qman_fq *)tag;

    }
#[no_mangle]
unsafe extern "C" fn fq_to_tag(fq: *mut qman_fq) -> u32 {
    static u32 fq_to_tag(struct qman_fq *fq)
    {

    return fq.idx;

    return (u32)fq;

    }
    static u32 __poll_portal_slow(struct qman_portal *p, u32 is);
    static inline unsigned int __poll_portal_fast(struct qman_portal *p,
    unsigned int poll_limit, bool sched_napi);
    static void qm_congestion_task(struct work_struct *work);
    static void qm_mr_process_task(struct work_struct *work);
#[no_mangle]
unsafe extern "C" fn portal_isr(irq: c_int, ptr: *mut c_void) -> irqreturn_t {
    static irqreturn_t portal_isr(int irq, void *ptr)
    {
    struct qman_portal *p = ptr;
    let mut is: u32 = qm_in(&p.p, QM_REG_ISR) & p.irq_sources;
    let mut clear: u32 = 0;
    if (unlikely(!is))
    return IRQ_NONE;
// DQRR-handling if it's interrupt-driven
    if (is & QM_PIRQ_DQRI) {
    __poll_portal_fast(p, QMAN_POLL_LIMIT, true);
    clear = QM_DQAVAIL_MASK | QM_PIRQ_DQRI;
    }
// Handling of anything else that's interrupt-driven
    clear |= __poll_portal_slow(p, is) & QM_PIRQ_SLOW;
    qm_out(&p.p, QM_REG_ISR, clear);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn drain_mr_fqrni(p: *mut qm_portal) -> c_int {
    static int drain_mr_fqrni(struct qm_portal *p)
    {
    const union qm_mr_entry *msg;
    loop:
    qm_mr_pvb_update(p);
    msg = qm_mr_current(p);
    if (!msg) {
//
// if MR was full and h/w had other FQRNI entries to produce, we
// need to allow it time to produce those entries once the
// existing entries are consumed. A worst-case situation
// (fully-loaded system) means h/w sequencers may have to do 3-4
// other things before servicing the portal's MR pump, each of
// which (if slow) may take ~50 qman cycles (which is ~200
// processor cycles). So rounding up and then multiplying this
// worst-case estimate by a factor of 10, just to be
// ultra-paranoid, goes as high as 10,000 cycles. NB, we consume
// one entry at a time, so h/w has an opportunity to produce new
// entries well before the ring has been fully consumed, so
// we're being *really* paranoid here.
//
    mdelay(1);
    qm_mr_pvb_update(p);
    msg = qm_mr_current(p);
    if (!msg)
    return 0;
    }
    if ((msg.verb & QM_MR_VERB_TYPE_MASK) != QM_MR_VERB_FQRNI) {
// We aren't draining anything but FQRNIs
    pr_err("Found verb 0x%x in MR\n", msg.verb);
    return -1;
    }
    qm_mr_next(p);
    qm_mr_cci_consume(p, 1);
    goto loop;
    }
    static int qman_create_portal(struct qman_portal *portal,
    const struct qm_portal_config *c,
    const struct qman_cgrs *cgrs)
    {
    struct qm_portal *p;
    int ret;
    u32 isdr;
    p = &portal.p;

// PAMU is required for stashing
    portal.use_eqcr_ci_stashing = ((qman_ip_rev >= QMAN_REV30) ? 1 : 0);

    portal.use_eqcr_ci_stashing = 0;

//
// prep the low-level portal struct with the mapped addresses from the
// config, everything that follows depends on it and "config" is more
// for (de)reference
//
    p.addr.ce = c.addr_virt_ce;
    p.addr.ce_be = c.addr_virt_ce;
    p.addr.ci = c.addr_virt_ci;
//
// If CI-stashing is used, the current defaults use a threshold of 3,
// and stash with high-than-DQRR priority.
//
    if (qm_eqcr_init(p, qm_eqcr_pvb,
    portal.use_eqcr_ci_stashing ? 3 : 0, 1)) {
    dev_err(c.dev, "EQCR initialisation failed\n");
    goto fail_eqcr;
    }
    if (qm_dqrr_init(p, c, qm_dqrr_dpush, qm_dqrr_pvb,
    qm_dqrr_cdc, DQRR_MAXFILL)) {
    dev_err(c.dev, "DQRR initialisation failed\n");
    goto fail_dqrr;
    }
    if (qm_mr_init(p, qm_mr_pvb, qm_mr_cci)) {
    dev_err(c.dev, "MR initialisation failed\n");
    goto fail_mr;
    }
    if (qm_mc_init(p)) {
    dev_err(c.dev, "MC initialisation failed\n");
    goto fail_mc;
    }
// static interrupt-gating controls
    qm_dqrr_set_ithresh(p, QMAN_PIRQ_DQRR_ITHRESH);
    qm_mr_set_ithresh(p, QMAN_PIRQ_MR_ITHRESH);
    qm_out(p, QM_REG_ITPR, QMAN_PIRQ_IPERIOD);
    portal.cgrs = kmalloc_objs(*portal.cgrs, 2);
    if (!portal.cgrs)
    goto fail_cgrs;
// initial snapshot is no-depletion
    qman_cgrs_init(&portal.cgrs[1]);
    if (cgrs)
    portal.cgrs[0] = *cgrs;
    else
// if the given mask is NULL, assume all CGRs can be seen
    qman_cgrs_fill(&portal.cgrs[0]);
    INIT_LIST_HEAD(&portal.cgr_cbs);
    raw_spin_lock_init(&portal.cgr_lock);
    INIT_WORK(&portal.congestion_work, qm_congestion_task);
    INIT_WORK(&portal.mr_work, qm_mr_process_task);
    portal.bits = 0;
    portal.sdqcr = QM_SDQCR_SOURCE_CHANNELS | QM_SDQCR_COUNT_UPTO3 |
    QM_SDQCR_DEDICATED_PRECEDENCE | QM_SDQCR_TYPE_PRIO_QOS |
    QM_SDQCR_TOKEN_SET(0xab) | QM_SDQCR_CHANNELS_DEDICATED;
    isdr = 0xffffffff;
    qm_out(p, QM_REG_ISDR, isdr);
    portal.irq_sources = 0;
    qm_out(p, QM_REG_IER, 0);
    snprintf(portal.irqname, MAX_IRQNAME, IRQNAME, c.cpu);
    qm_out(p, QM_REG_IIR, 1);
    if (request_irq(c.irq, portal_isr, 0, portal.irqname,	portal)) {
    dev_err(c.dev, "request_irq() failed\n");
    goto fail_irq;
    }
    if (dpaa_set_portal_irq_affinity(c.dev, c.irq, c.cpu))
    goto fail_affinity;
// Need EQCR to be empty before continuing
    isdr &= ~QM_PIRQ_EQCI;
    qm_out(p, QM_REG_ISDR, isdr);
    ret = qm_eqcr_get_fill(p);
    if (ret) {
    dev_err(c.dev, "EQCR unclean\n");
    goto fail_eqcr_empty;
    }
    isdr &= ~(QM_PIRQ_DQRI | QM_PIRQ_MRI);
    qm_out(p, QM_REG_ISDR, isdr);
    if (qm_dqrr_current(p)) {
    dev_dbg(c.dev, "DQRR unclean\n");
    qm_dqrr_cdc_consume_n(p, 0xffff);
    }
    if (qm_mr_current(p) && drain_mr_fqrni(p)) {
// special handling, drain just in case it's a few FQRNIs
    const union qm_mr_entry *e = qm_mr_current(p);
    dev_err(c.dev, "MR dirty, VB 0x%x, rc 0x%x, addr 0x%llx\n",
    e.verb, e.ern.rc, qm_fd_addr_get64(&e.ern.fd));
    goto fail_dqrr_mr_empty;
    }
// Success
    portal.config = c;
    qm_out(p, QM_REG_ISR, 0xffffffff);
    qm_out(p, QM_REG_ISDR, 0);
    if (!qman_requires_cleanup())
    qm_out(p, QM_REG_IIR, 0);
// Write a sane SDQCR
    qm_dqrr_sdqcr_set(p, portal.sdqcr);
    return 0;
    fail_dqrr_mr_empty:
    fail_eqcr_empty:
    fail_affinity:
    free_irq(c.irq, portal);
    fail_irq:
    kfree(portal.cgrs);
    fail_cgrs:
    qm_mc_finish(p);
    fail_mc:
    qm_mr_finish(p);
    fail_mr:
    qm_dqrr_finish(p);
    fail_dqrr:
    qm_eqcr_finish(p);
    fail_eqcr:
    return -EIO;
    }
    struct qman_portal *qman_create_affine_portal(const struct qm_portal_config *c,
    const struct qman_cgrs *cgrs)
    {
    struct qman_portal *portal;
    int err;
    portal = &per_cpu(qman_affine_portal, c.cpu);
    err = qman_create_portal(portal, c, cgrs);
    if (err)
    return core::ptr::null_mut();
    spin_lock(&affine_mask_lock);
    cpumask_set_cpu(c.cpu, &affine_mask);
    affine_channels[c.cpu] = c.channel;
    affine_portals[c.cpu] = portal;
    spin_unlock(&affine_mask_lock);
    return portal;
    }
#[no_mangle]
unsafe extern "C" fn qman_destroy_portal(qm: *mut qman_portal) {
    static void qman_destroy_portal(struct qman_portal *qm)
    {
    const struct qm_portal_config *pcfg;
// Stop dequeues on the portal
    qm_dqrr_sdqcr_set(&qm.p, 0);
//
// NB we do this to "quiesce" EQCR. If we add enqueue-completions or
// something related to QM_PIRQ_EQCI, this may need fixing.
// Also, due to the prefetching model used for CI updates in the enqueue
// path, this update will only invalidate the CI cacheline *after
// working on it, so we need to call this twice to ensure a full update
// irrespective of where the enqueue processing was at when the teardown
// began.
//
    qm_eqcr_cce_update(&qm.p);
    qm_eqcr_cce_update(&qm.p);
    pcfg = qm.config;
    free_irq(pcfg.irq, qm);
    kfree(qm.cgrs);
    qm_mc_finish(&qm.p);
    qm_mr_finish(&qm.p);
    qm_dqrr_finish(&qm.p);
    qm_eqcr_finish(&qm.p);
    qm.config = core::ptr::null_mut();
    }
    const struct qm_portal_config *qman_destroy_affine_portal(void)
    {
    struct qman_portal *qm = get_affine_portal();
    const struct qm_portal_config *pcfg;
    int cpu;
    pcfg = qm.config;
    cpu = pcfg.cpu;
    qman_destroy_portal(qm);
    spin_lock(&affine_mask_lock);
    cpumask_clear_cpu(cpu, &affine_mask);
    spin_unlock(&affine_mask_lock);
    put_affine_portal();
    return pcfg;
    }
// Inline helper to reduce nesting in __poll_portal_slow()
    static inline void fq_state_change(struct qman_portal *p, struct qman_fq *fq,
    const union qm_mr_entry *msg, u8 verb)
    {
    switch (verb) {
    case QM_MR_VERB_FQRL:
    DPAA_ASSERT(fq_isset(fq, QMAN_FQ_STATE_ORL));
    fq_clear(fq, QMAN_FQ_STATE_ORL);
    break;
    case QM_MR_VERB_FQRN:
    DPAA_ASSERT(fq.state == qman_fq_state_parked ||
    fq.state == qman_fq_state_sched);
    DPAA_ASSERT(fq_isset(fq, QMAN_FQ_STATE_CHANGING));
    fq_clear(fq, QMAN_FQ_STATE_CHANGING);
    if (msg.fq.fqs & QM_MR_FQS_NOTEMPTY)
    fq_set(fq, QMAN_FQ_STATE_NE);
    if (msg.fq.fqs & QM_MR_FQS_ORLPRESENT)
    fq_set(fq, QMAN_FQ_STATE_ORL);
    fq.state = qman_fq_state_retired;
    break;
    case QM_MR_VERB_FQPN:
    DPAA_ASSERT(fq.state == qman_fq_state_sched);
    DPAA_ASSERT(fq_isclear(fq, QMAN_FQ_STATE_CHANGING));
    fq.state = qman_fq_state_parked;
    }
    }
#[no_mangle]
unsafe extern "C" fn qm_congestion_task(work: *mut work_struct) {
    static void qm_congestion_task(struct work_struct *work)
    {
    struct qman_portal *p = container_of(work, struct qman_portal,
    congestion_work);
    struct qman_cgrs rr, c;
    union qm_mc_result *mcr;
    struct qman_cgr *cgr;
//
// FIXME: QM_MCR_TIMEOUT is 10ms, which is too long for a raw spinlock!
//
    raw_spin_lock_irq(&p.cgr_lock);
    qm_mc_start(&p.p);
    qm_mc_commit(&p.p, QM_MCC_VERB_QUERYCONGESTION);
    if (!qm_mc_result_timeout(&p.p, &mcr)) {
    raw_spin_unlock_irq(&p.cgr_lock);
    dev_crit(p.config.dev, "QUERYCONGESTION timeout\n");
    qman_p_irqsource_add(p, QM_PIRQ_CSCI);
    return;
    }
// mask out the ones I'm not interested in
    qman_cgrs_and(&rr, (struct qman_cgrs *)&mcr.querycongestion.state,
    &p.cgrs[0]);
// check previous snapshot for delta, enter/exit congestion
    qman_cgrs_xor(&c, &rr, &p.cgrs[1]);
// update snapshot
    qman_cgrs_cp(&p.cgrs[1], &rr);
// Invoke callback
    list_for_each_entry(cgr, &p.cgr_cbs, node)
    if (cgr.cb && qman_cgrs_get(&c, cgr.cgrid))
    cgr.cb(p, cgr, qman_cgrs_get(&rr, cgr.cgrid));
    raw_spin_unlock_irq(&p.cgr_lock);
    qman_p_irqsource_add(p, QM_PIRQ_CSCI);
    }
#[no_mangle]
unsafe extern "C" fn qm_mr_process_task(work: *mut work_struct) {
    static void qm_mr_process_task(struct work_struct *work)
    {
    struct qman_portal *p = container_of(work, struct qman_portal,
    mr_work);
    const union qm_mr_entry *msg;
    struct qman_fq *fq;
    u8 verb, num = 0;
    preempt_disable();
    while (1) {
    qm_mr_pvb_update(&p.p);
    msg = qm_mr_current(&p.p);
    if (!msg)
    break;
    verb = msg.verb & QM_MR_VERB_TYPE_MASK;
// The message is a software ERN iff the 0x20 bit is clear
    if (verb & 0x20) {
    switch (verb) {
    case QM_MR_VERB_FQRNI:
// nada, we drop FQRNIs on the floor
    break;
    case QM_MR_VERB_FQRN:
    case QM_MR_VERB_FQRL:
// Lookup in the retirement table
    fq = fqid_to_fq(qm_fqid_get(&msg.fq));
    if (WARN_ON(!fq))
    break;
    fq_state_change(p, fq, msg, verb);
    if (fq.cb.fqs)
    fq.cb.fqs(p, fq, msg);
    break;
    case QM_MR_VERB_FQPN:
// Parked
    fq = tag_to_fq(be32_to_cpu(msg.fq.context_b));
    fq_state_change(p, fq, msg, verb);
    if (fq.cb.fqs)
    fq.cb.fqs(p, fq, msg);
    break;
    case QM_MR_VERB_DC_ERN:
// DCP ERN
    pr_crit_once("Leaking DCP ERNs!\n");
    break;
    default:
    pr_crit("Invalid MR verb 0x%02x\n", verb);
    }
    } else {
// Its a software ERN
    fq = tag_to_fq(be32_to_cpu(msg.ern.tag));
    fq.cb.ern(p, fq, msg);
    }
    num++;
    qm_mr_next(&p.p);
    }
    qm_mr_cci_consume(&p.p, num);
    qman_p_irqsource_add(p, QM_PIRQ_MRI);
    preempt_enable();
    }
#[no_mangle]
unsafe extern "C" fn __poll_portal_slow(p: *mut qman_portal, is: u32) -> u32 {
    static u32 __poll_portal_slow(struct qman_portal *p, u32 is)
    {
    if (is & QM_PIRQ_CSCI) {
    qman_p_irqsource_remove(p, QM_PIRQ_CSCI);
    queue_work_on(smp_processor_id(), qm_portal_wq,
    &p.congestion_work);
    }
    if (is & QM_PIRQ_EQRI) {
    qm_eqcr_cce_update(&p.p);
    qm_eqcr_set_ithresh(&p.p, 0);
    wake_up(&affine_queue);
    }
    if (is & QM_PIRQ_MRI) {
    qman_p_irqsource_remove(p, QM_PIRQ_MRI);
    queue_work_on(smp_processor_id(), qm_portal_wq,
    &p.mr_work);
    }
    return is;
    }
//
// remove some slowish-path stuff from the "fast path" and make sure it isn't
// inlined.
//
#[no_mangle]
unsafe extern "C" fn clear_vdqcr(p: *mut qman_portal, fq: *mut qman_fq) -> noinline void {
    static noinline void clear_vdqcr(struct qman_portal *p, struct qman_fq *fq)
    {
    p.vdqcr_owned = core::ptr::null_mut();
    fq_clear(fq, QMAN_FQ_STATE_VDQCR);
    wake_up(&affine_queue);
    }
//
// The only states that would conflict with other things if they ran at the
// same time on the same cpu are:
//
// (i) setting/clearing vdqcr_owned, and
// (ii) clearing the NE (Not Empty) flag.
//
// Both are safe. Because;
//
// (i) this clearing can only occur after qman_volatile_dequeue() has set the
// vdqcr_owned field (which it does before setting VDQCR), and
// qman_volatile_dequeue() blocks interrupts and preemption while this is
// done so that we can't interfere.
// (ii) the NE flag is only cleared after qman_retire_fq() has set it, and as
// with (i) that API prevents us from interfering until it's safe.
//
// The good thing is that qman_volatile_dequeue() and qman_retire_fq() run far
// less frequently (ie. per-FQ) than __poll_portal_fast() does, so the nett
// advantage comes from this function not having to "lock" anything at all.
//
// Note also that the callbacks are invoked at points which are safe against the
// above potential conflicts, but that this function itself is not re-entrant
// (this is because the function tracks one end of each FIFO in the portal and
// we do *not* want to lock that). So the consequence is that it is safe for
// user callbacks to call into any QMan API.
//
    static inline unsigned int __poll_portal_fast(struct qman_portal *p,
    unsigned int poll_limit, bool sched_napi)
    {
    const struct qm_dqrr_entry *dq;
    struct qman_fq *fq;
    enum qman_cb_dqrr_result res;
    let mut limit: c_uint = 0;
    do {
    qm_dqrr_pvb_update(&p.p);
    dq = qm_dqrr_current(&p.p);
    if (!dq)
    break;
    if (dq.stat & QM_DQRR_STAT_UNSCHEDULED) {
//
// VDQCR: don't trust context_b as the FQ may have
// been configured for h/w consumption and we're
// draining it post-retirement.
//
    fq = p.vdqcr_owned;
//
// We only set QMAN_FQ_STATE_NE when retiring, so we
// only need to check for clearing it when doing
// volatile dequeues.  It's one less thing to check
// in the critical path (SDQCR).
//
    if (dq.stat & QM_DQRR_STAT_FQ_EMPTY)
    fq_clear(fq, QMAN_FQ_STATE_NE);
//
// This is duplicated from the SDQCR code, but we
// have stuff to do before *and* after this callback,
// and we don't want multiple if()s in the critical
// path (SDQCR).
//
    res = fq.cb.dqrr(p, fq, dq, sched_napi);
    if (res == qman_cb_dqrr_stop)
    break;
// Check for VDQCR completion
    if (dq.stat & QM_DQRR_STAT_DQCR_EXPIRED)
    clear_vdqcr(p, fq);
    } else {
// SDQCR: context_b points to the FQ
    fq = tag_to_fq(be32_to_cpu(dq.context_b));
// Now let the callback do its stuff
    res = fq.cb.dqrr(p, fq, dq, sched_napi);
//
// The callback can request that we exit without
// consuming this entry nor advancing;
//
    if (res == qman_cb_dqrr_stop)
    break;
    }
// Interpret 'dq' from a driver perspective.
//
// Parking isn't possible unless HELDACTIVE was set. NB,
// FORCEELIGIBLE implies HELDACTIVE, so we only need to
// check for HELDACTIVE to cover both.
//
    DPAA_ASSERT((dq.stat & QM_DQRR_STAT_FQ_HELDACTIVE) ||
    (res != qman_cb_dqrr_park));
// just means "skip it, I'll consume it myself later on"
    if (res != qman_cb_dqrr_defer)
    qm_dqrr_cdc_consume_1ptr(&p.p, dq,
    res == qman_cb_dqrr_park);
// Move forward
    qm_dqrr_next(&p.p);
//
// Entry processed and consumed, increment our counter.  The
// callback can request that we exit after consuming the
// entry, and we also exit if we reach our processing limit,
// so loop back only if neither of these conditions is met.
//
    } while (++limit < poll_limit && res != qman_cb_dqrr_consume_stop);
    return limit;
    }
#[no_mangle]
pub unsafe extern "C" fn qman_p_irqsource_add(p: *mut qman_portal, bits: u32) {
    void qman_p_irqsource_add(struct qman_portal *p, u32 bits)
    {
    unsigned long irqflags;
    local_irq_save(irqflags);
    p.irq_sources |= bits & QM_PIRQ_VISIBLE;
    qm_out(&p.p, QM_REG_IER, p.irq_sources);
    local_irq_restore(irqflags);
    }
    EXPORT_SYMBOL(qman_p_irqsource_add);
#[no_mangle]
pub unsafe extern "C" fn qman_p_irqsource_remove(p: *mut qman_portal, bits: u32) {
    void qman_p_irqsource_remove(struct qman_portal *p, u32 bits)
    {
    unsigned long irqflags;
    u32 ier;
//
// Our interrupt handler only processes+clears status register bits that
// are in p->irq_sources. As we're trimming that mask, if one of them
// were to assert in the status register just before we remove it from
// the enable register, there would be an interrupt-storm when we
// release the IRQ lock. So we wait for the enable register update to
// take effect in h/w (by reading it back) and then clear all other bits
// in the status register. Ie. we clear them from ISR once it's certain
// IER won't allow them to reassert.
//
    local_irq_save(irqflags);
    bits &= QM_PIRQ_VISIBLE;
    p.irq_sources &= ~bits;
    qm_out(&p.p, QM_REG_IER, p.irq_sources);
    ier = qm_in(&p.p, QM_REG_IER);
//
// Using "~ier" (rather than "bits" or "~p->irq_sources") creates a
// data-dependency, ie. to protect against re-ordering.
//
    qm_out(&p.p, QM_REG_ISR, ~ier);
    local_irq_restore(irqflags);
    }
    EXPORT_SYMBOL(qman_p_irqsource_remove);
    const cpumask_t *qman_affine_cpus(void)
    {
    return &affine_mask;
    }
    EXPORT_SYMBOL(qman_affine_cpus);
#[no_mangle]
pub unsafe extern "C" fn qman_affine_channel(cpu: c_int) -> u16 {
    u16 qman_affine_channel(int cpu)
    {
    if (cpu < 0) {
    struct qman_portal *portal = get_affine_portal();
    cpu = portal.config.cpu;
    put_affine_portal();
    }
    WARN_ON(!cpumask_test_cpu(cpu, &affine_mask));
    return affine_channels[cpu];
    }
    EXPORT_SYMBOL(qman_affine_channel);
    struct qman_portal *qman_get_affine_portal(int cpu)
    {
    return affine_portals[cpu];
    }
    EXPORT_SYMBOL(qman_get_affine_portal);
#[no_mangle]
pub unsafe extern "C" fn qman_start_using_portal(p: *mut qman_portal, dev: *mut device) -> c_int {
    int qman_start_using_portal(struct qman_portal *p, struct device *dev)
    {
    return (!device_link_add(dev, p.config.dev,
    DL_FLAG_AUTOREMOVE_CONSUMER)) ? -EINVAL : 0;
    }
    EXPORT_SYMBOL(qman_start_using_portal);
#[no_mangle]
pub unsafe extern "C" fn qman_p_poll_dqrr(p: *mut qman_portal, limit: c_uint) -> c_int {
    int qman_p_poll_dqrr(struct qman_portal *p, unsigned int limit)
    {
    return __poll_portal_fast(p, limit, false);
    }
    EXPORT_SYMBOL(qman_p_poll_dqrr);
#[no_mangle]
pub unsafe extern "C" fn qman_p_static_dequeue_add(p: *mut qman_portal, pools: u32) {
    void qman_p_static_dequeue_add(struct qman_portal *p, u32 pools)
    {
    unsigned long irqflags;
    local_irq_save(irqflags);
    pools &= p.config.pools;
    p.sdqcr |= pools;
    qm_dqrr_sdqcr_set(&p.p, p.sdqcr);
    local_irq_restore(irqflags);
    }
    EXPORT_SYMBOL(qman_p_static_dequeue_add);
// Frame queue API
    static const char *mcr_result_str(u8 result)
    {
    switch (result) {
    case QM_MCR_RESULT_NULL:
    return "QM_MCR_RESULT_NULL";
    case QM_MCR_RESULT_OK:
    return "QM_MCR_RESULT_OK";
    case QM_MCR_RESULT_ERR_FQID:
    return "QM_MCR_RESULT_ERR_FQID";
    case QM_MCR_RESULT_ERR_FQSTATE:
    return "QM_MCR_RESULT_ERR_FQSTATE";
    case QM_MCR_RESULT_ERR_NOTEMPTY:
    return "QM_MCR_RESULT_ERR_NOTEMPTY";
    case QM_MCR_RESULT_PENDING:
    return "QM_MCR_RESULT_PENDING";
    case QM_MCR_RESULT_ERR_BADCOMMAND:
    return "QM_MCR_RESULT_ERR_BADCOMMAND";
    }
    return "<unknown MCR result>";
    }
#[no_mangle]
pub unsafe extern "C" fn qman_create_fq(fqid: u32, flags: u32, fq: *mut qman_fq) -> c_int {
    int qman_create_fq(u32 fqid, u32 flags, struct qman_fq *fq)
    {
    if (flags & QMAN_FQ_FLAG_DYNAMIC_FQID) {
    let mut ret: c_int = qman_alloc_fqid(&fqid);
    if (ret)
    return ret;
    }
    fq.fqid = fqid;
    fq.flags = flags;
    fq.state = qman_fq_state_oos;
    fq.cgr_groupid = 0;
// A context_b of 0 is allegedly special, so don't use that fqid
    if (fqid == 0 || fqid >= num_fqids) {
    WARN(1, "bad fqid %d\n", fqid);
    return -EINVAL;
    }
    fq.idx = fqid * 2;
    if (flags & QMAN_FQ_FLAG_NO_MODIFY)
    fq.idx++;
    WARN_ON(fq_table[fq.idx]);
    fq_table[fq.idx] = fq;
    return 0;
    }
    EXPORT_SYMBOL(qman_create_fq);
#[no_mangle]
pub unsafe extern "C" fn qman_destroy_fq(fq: *mut qman_fq) {
    void qman_destroy_fq(struct qman_fq *fq)
    {
    int leaked;
//
// We don't need to lock the FQ as it is a pre-condition that the FQ be
// quiesced. Instead, run some checks.
//
    switch (fq.state) {
    case qman_fq_state_parked:
    case qman_fq_state_oos:
//
// There's a race condition here on releasing the fqid,
// setting the fq_table to NULL, and freeing the fqid.
// To prevent it, this order should be respected:
//
    if (fq_isset(fq, QMAN_FQ_FLAG_DYNAMIC_FQID)) {
    leaked = qman_shutdown_fq(fq.fqid);
    if (leaked)
    pr_debug("FQID %d leaked\n", fq.fqid);
    }
    DPAA_ASSERT(fq_table[fq.idx]);
    fq_table[fq.idx] = core::ptr::null_mut();
    if (fq_isset(fq, QMAN_FQ_FLAG_DYNAMIC_FQID) && !leaked) {
//
// fq_table[fq->idx] should be set to null before
// freeing fq->fqid otherwise it could by allocated by
// qman_alloc_fqid() while still being !NULL
//
    smp_wmb();
    gen_pool_free(qm_fqalloc, fq.fqid | DPAA_GENALLOC_OFF, 1);
    }
    return;
    default:
    break;
    }
    DPAA_ASSERT(core::ptr::null_mut() == "qman_free_fq() on unquiesced FQ!");
    }
    EXPORT_SYMBOL(qman_destroy_fq);
#[no_mangle]
pub unsafe extern "C" fn qman_fq_fqid(fq: *mut qman_fq) -> u32 {
    u32 qman_fq_fqid(struct qman_fq *fq)
    {
    return fq.fqid;
    }
    EXPORT_SYMBOL(qman_fq_fqid);
#[no_mangle]
pub unsafe extern "C" fn qman_init_fq(fq: *mut qman_fq, flags: u32, opts: *mut qm_mcc_initfq) -> c_int {
    int qman_init_fq(struct qman_fq *fq, u32 flags, struct qm_mcc_initfq *opts)
    {
    union qm_mc_command *mcc;
    union qm_mc_result *mcr;
    struct qman_portal *p;
    u8 res, myverb;
    let mut ret: c_int = 0;
    myverb = (flags & QMAN_INITFQ_FLAG_SCHED)
    ? QM_MCC_VERB_INITFQ_SCHED : QM_MCC_VERB_INITFQ_PARKED;
    if (fq.state != qman_fq_state_oos &&
    fq.state != qman_fq_state_parked)
    return -EINVAL;

    if (fq_isset(fq, QMAN_FQ_FLAG_NO_MODIFY))
    return -EINVAL;

    if (opts && (be16_to_cpu(opts.we_mask) & QM_INITFQ_WE_OAC)) {
// And can't be set at the same time as TDTHRESH
    if (be16_to_cpu(opts.we_mask) & QM_INITFQ_WE_TDTHRESH)
    return -EINVAL;
    }
// Issue an INITFQ_[PARKED|SCHED] management command
    p = get_affine_portal();
    if (fq_isset(fq, QMAN_FQ_STATE_CHANGING) ||
    (fq.state != qman_fq_state_oos &&
    fq.state != qman_fq_state_parked)) {
    ret = -EBUSY;
    goto out;
    }
    mcc = qm_mc_start(&p.p);
    if (opts)
    mcc.initfq = *opts;
    qm_fqid_set(&mcc.fq, fq.fqid);
    mcc.initfq.count = 0;
//
// If the FQ does *not* have the TO_DCPORTAL flag, context_b is set as a
// demux pointer. Otherwise, the caller-provided value is allowed to
// stand, don't overwrite it.
//
    if (fq_isclear(fq, QMAN_FQ_FLAG_TO_DCPORTAL)) {
    dma_addr_t phys_fq;
    mcc.initfq.we_mask |= cpu_to_be16(QM_INITFQ_WE_CONTEXTB);
    mcc.initfq.fqd.context_b = cpu_to_be32(fq_to_tag(fq));
//
// and the physical address - NB, if the user wasn't trying to
// set CONTEXTA, clear the stashing settings.
//
    if (!(be16_to_cpu(mcc.initfq.we_mask) &
    QM_INITFQ_WE_CONTEXTA)) {
    mcc.initfq.we_mask |=
    cpu_to_be16(QM_INITFQ_WE_CONTEXTA);
    memset(&mcc.initfq.fqd.context_a, 0,
    sizeof(mcc.initfq.fqd.context_a));
    } else {
    struct qman_portal *p = qman_dma_portal;
    phys_fq = dma_map_single(p.config.dev, fq,
    sizeof(*fq), DMA_TO_DEVICE);
    if (dma_mapping_error(p.config.dev, phys_fq)) {
    dev_err(p.config.dev, "dma_mapping failed\n");
    ret = -EIO;
    goto out;
    }
    qm_fqd_stashing_set64(&mcc.initfq.fqd, phys_fq);
    }
    }
    if (flags & QMAN_INITFQ_FLAG_LOCAL) {
    let mut wq: c_int = 0;
    if (!(be16_to_cpu(mcc.initfq.we_mask) &
    QM_INITFQ_WE_DESTWQ)) {
    mcc.initfq.we_mask |=
    cpu_to_be16(QM_INITFQ_WE_DESTWQ);
    wq = 4;
    }
    qm_fqd_set_destwq(&mcc.initfq.fqd, p.config.channel, wq);
    }
    qm_mc_commit(&p.p, myverb);
    if (!qm_mc_result_timeout(&p.p, &mcr)) {
    dev_err(p.config.dev, "MCR timeout\n");
    ret = -ETIMEDOUT;
    goto out;
    }
    DPAA_ASSERT((mcr.verb & QM_MCR_VERB_MASK) == myverb);
    res = mcr.result;
    if (res != QM_MCR_RESULT_OK) {
    ret = -EIO;
    goto out;
    }
    if (opts) {
    if (be16_to_cpu(opts.we_mask) & QM_INITFQ_WE_FQCTRL) {
    if (be16_to_cpu(opts.fqd.fq_ctrl) & QM_FQCTRL_CGE)
    fq_set(fq, QMAN_FQ_STATE_CGR_EN);
    else
    fq_clear(fq, QMAN_FQ_STATE_CGR_EN);
    }
    if (be16_to_cpu(opts.we_mask) & QM_INITFQ_WE_CGID)
    fq.cgr_groupid = opts.fqd.cgid;
    }
    fq.state = (flags & QMAN_INITFQ_FLAG_SCHED) ?
    qman_fq_state_sched : qman_fq_state_parked;
    out:
    put_affine_portal();
    return ret;
    }
    EXPORT_SYMBOL(qman_init_fq);
#[no_mangle]
pub unsafe extern "C" fn qman_schedule_fq(fq: *mut qman_fq) -> c_int {
    int qman_schedule_fq(struct qman_fq *fq)
    {
    union qm_mc_command *mcc;
    union qm_mc_result *mcr;
    struct qman_portal *p;
    let mut ret: c_int = 0;
    if (fq.state != qman_fq_state_parked)
    return -EINVAL;

    if (fq_isset(fq, QMAN_FQ_FLAG_NO_MODIFY))
    return -EINVAL;

// Issue a ALTERFQ_SCHED management command
    p = get_affine_portal();
    if (fq_isset(fq, QMAN_FQ_STATE_CHANGING) ||
    fq.state != qman_fq_state_parked) {
    ret = -EBUSY;
    goto out;
    }
    mcc = qm_mc_start(&p.p);
    qm_fqid_set(&mcc.fq, fq.fqid);
    qm_mc_commit(&p.p, QM_MCC_VERB_ALTER_SCHED);
    if (!qm_mc_result_timeout(&p.p, &mcr)) {
    dev_err(p.config.dev, "ALTER_SCHED timeout\n");
    ret = -ETIMEDOUT;
    goto out;
    }
    DPAA_ASSERT((mcr.verb & QM_MCR_VERB_MASK) == QM_MCR_VERB_ALTER_SCHED);
    if (mcr.result != QM_MCR_RESULT_OK) {
    ret = -EIO;
    goto out;
    }
    fq.state = qman_fq_state_sched;
    out:
    put_affine_portal();
    return ret;
    }
    EXPORT_SYMBOL(qman_schedule_fq);
#[no_mangle]
pub unsafe extern "C" fn qman_retire_fq(fq: *mut qman_fq, flags: *mut u32) -> c_int {
    int qman_retire_fq(struct qman_fq *fq, u32 *flags)
    {
    union qm_mc_command *mcc;
    union qm_mc_result *mcr;
    struct qman_portal *p;
    int ret;
    u8 res;
    if (fq.state != qman_fq_state_parked &&
    fq.state != qman_fq_state_sched)
    return -EINVAL;

    if (fq_isset(fq, QMAN_FQ_FLAG_NO_MODIFY))
    return -EINVAL;

    p = get_affine_portal();
    if (fq_isset(fq, QMAN_FQ_STATE_CHANGING) ||
    fq.state == qman_fq_state_retired ||
    fq.state == qman_fq_state_oos) {
    ret = -EBUSY;
    goto out;
    }
    mcc = qm_mc_start(&p.p);
    qm_fqid_set(&mcc.fq, fq.fqid);
    qm_mc_commit(&p.p, QM_MCC_VERB_ALTER_RETIRE);
    if (!qm_mc_result_timeout(&p.p, &mcr)) {
    dev_crit(p.config.dev, "ALTER_RETIRE timeout\n");
    ret = -ETIMEDOUT;
    goto out;
    }
    DPAA_ASSERT((mcr.verb & QM_MCR_VERB_MASK) == QM_MCR_VERB_ALTER_RETIRE);
    res = mcr.result;
//
// "Elegant" would be to treat OK/PENDING the same way; set CHANGING,
// and defer the flags until FQRNI or FQRN (respectively) show up. But
// "Friendly" is to process OK immediately, and not set CHANGING. We do
// friendly, otherwise the caller doesn't necessarily have a fully
// "retired" FQ on return even if the retirement was immediate. However
// this does mean some code duplication between here and
// fq_state_change().
//
    if (res == QM_MCR_RESULT_OK) {
    ret = 0;
// Process 'fq' right away, we'll ignore FQRNI
    if (mcr.alterfq.fqs & QM_MCR_FQS_NOTEMPTY)
    fq_set(fq, QMAN_FQ_STATE_NE);
    if (mcr.alterfq.fqs & QM_MCR_FQS_ORLPRESENT)
    fq_set(fq, QMAN_FQ_STATE_ORL);
    if (flags)
// flags = fq->flags;
    fq.state = qman_fq_state_retired;
    if (fq.cb.fqs) {
//
// Another issue with supporting "immediate" retirement
// is that we're forced to drop FQRNIs, because by the
// time they're seen it may already be "too late" (the
// fq may have been OOS'd and free()'d already). But if
// the upper layer wants a callback whether it's
// immediate or not, we have to fake a "MR" entry to
// look like an FQRNI...
//
    union qm_mr_entry msg;
    msg.verb = QM_MR_VERB_FQRNI;
    msg.fq.fqs = mcr.alterfq.fqs;
    qm_fqid_set(&msg.fq, fq.fqid);
    msg.fq.context_b = cpu_to_be32(fq_to_tag(fq));
    fq.cb.fqs(p, fq, &msg);
    }
    } else if (res == QM_MCR_RESULT_PENDING) {
    ret = 1;
    fq_set(fq, QMAN_FQ_STATE_CHANGING);
    } else {
    ret = -EIO;
    }
    out:
    put_affine_portal();
    return ret;
    }
    EXPORT_SYMBOL(qman_retire_fq);
#[no_mangle]
pub unsafe extern "C" fn qman_oos_fq(fq: *mut qman_fq) -> c_int {
    int qman_oos_fq(struct qman_fq *fq)
    {
    union qm_mc_command *mcc;
    union qm_mc_result *mcr;
    struct qman_portal *p;
    let mut ret: c_int = 0;
    if (fq.state != qman_fq_state_retired)
    return -EINVAL;

    if (fq_isset(fq, QMAN_FQ_FLAG_NO_MODIFY))
    return -EINVAL;

    p = get_affine_portal();
    if (fq_isset(fq, QMAN_FQ_STATE_BLOCKOOS) ||
    fq.state != qman_fq_state_retired) {
    ret = -EBUSY;
    goto out;
    }
    mcc = qm_mc_start(&p.p);
    qm_fqid_set(&mcc.fq, fq.fqid);
    qm_mc_commit(&p.p, QM_MCC_VERB_ALTER_OOS);
    if (!qm_mc_result_timeout(&p.p, &mcr)) {
    ret = -ETIMEDOUT;
    goto out;
    }
    DPAA_ASSERT((mcr.verb & QM_MCR_VERB_MASK) == QM_MCR_VERB_ALTER_OOS);
    if (mcr.result != QM_MCR_RESULT_OK) {
    ret = -EIO;
    goto out;
    }
    fq.state = qman_fq_state_oos;
    out:
    put_affine_portal();
    return ret;
    }
    EXPORT_SYMBOL(qman_oos_fq);
#[no_mangle]
pub unsafe extern "C" fn qman_query_fq(fq: *mut qman_fq, fqd: *mut qm_fqd) -> c_int {
    int qman_query_fq(struct qman_fq *fq, struct qm_fqd *fqd)
    {
    union qm_mc_command *mcc;
    union qm_mc_result *mcr;
    struct qman_portal *p = get_affine_portal();
    let mut ret: c_int = 0;
    mcc = qm_mc_start(&p.p);
    qm_fqid_set(&mcc.fq, fq.fqid);
    qm_mc_commit(&p.p, QM_MCC_VERB_QUERYFQ);
    if (!qm_mc_result_timeout(&p.p, &mcr)) {
    ret = -ETIMEDOUT;
    goto out;
    }
    DPAA_ASSERT((mcr.verb & QM_MCR_VERB_MASK) == QM_MCR_VERB_QUERYFQ);
    if (mcr.result == QM_MCR_RESULT_OK)
// fqd = mcr->queryfq.fqd;
    else
    ret = -EIO;
    out:
    put_affine_portal();
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn qman_query_fq_np(fq: *mut qman_fq, np: *mut qm_mcr_queryfq_np) -> c_int {
    int qman_query_fq_np(struct qman_fq *fq, struct qm_mcr_queryfq_np *np)
    {
    union qm_mc_command *mcc;
    union qm_mc_result *mcr;
    struct qman_portal *p = get_affine_portal();
    let mut ret: c_int = 0;
    mcc = qm_mc_start(&p.p);
    qm_fqid_set(&mcc.fq, fq.fqid);
    qm_mc_commit(&p.p, QM_MCC_VERB_QUERYFQ_NP);
    if (!qm_mc_result_timeout(&p.p, &mcr)) {
    ret = -ETIMEDOUT;
    goto out;
    }
    DPAA_ASSERT((mcr.verb & QM_MCR_VERB_MASK) == QM_MCR_VERB_QUERYFQ_NP);
    if (mcr.result == QM_MCR_RESULT_OK)
// np = mcr->queryfq_np;
#[no_mangle]
pub unsafe extern "C" fn if(QM_MCR_RESULT_ERR_FQID: mcr->result ==) -> else {
    else if (mcr.result == QM_MCR_RESULT_ERR_FQID)
    ret = -ERANGE;
    else
    ret = -EIO;
    out:
    put_affine_portal();
    return ret;
    }
    EXPORT_SYMBOL(qman_query_fq_np);
    static int qman_query_cgr(struct qman_cgr *cgr,
    struct qm_mcr_querycgr *cgrd)
    {
    union qm_mc_command *mcc;
    union qm_mc_result *mcr;
    struct qman_portal *p = get_affine_portal();
    let mut ret: c_int = 0;
    mcc = qm_mc_start(&p.p);
    mcc.cgr.cgid = cgr.cgrid;
    qm_mc_commit(&p.p, QM_MCC_VERB_QUERYCGR);
    if (!qm_mc_result_timeout(&p.p, &mcr)) {
    ret = -ETIMEDOUT;
    goto out;
    }
    DPAA_ASSERT((mcr.verb & QM_MCR_VERB_MASK) == QM_MCC_VERB_QUERYCGR);
    if (mcr.result == QM_MCR_RESULT_OK)
// cgrd = mcr->querycgr;
    else {
    dev_err(p.config.dev, "QUERY_CGR failed: %s\n",
    mcr_result_str(mcr.result));
    ret = -EIO;
    }
    out:
    put_affine_portal();
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn qman_query_cgr_congested(cgr: *mut qman_cgr, result: *mut bool) -> c_int {
    int qman_query_cgr_congested(struct qman_cgr *cgr, bool *result)
    {
    struct qm_mcr_querycgr query_cgr;
    int err;
    err = qman_query_cgr(cgr, &query_cgr);
    if (err)
    return err;
// result = !!query_cgr.cgr.cs;
    return 0;
    }
    EXPORT_SYMBOL(qman_query_cgr_congested);
// internal function used as a wait_event() expression
#[no_mangle]
unsafe extern "C" fn set_p_vdqcr(p: *mut qman_portal, fq: *mut qman_fq, vdqcr: u32) -> c_int {
    static int set_p_vdqcr(struct qman_portal *p, struct qman_fq *fq, u32 vdqcr)
    {
    unsigned long irqflags;
    let mut ret: c_int = -EBUSY;
    local_irq_save(irqflags);
    if (p.vdqcr_owned)
    goto out;
    if (fq_isset(fq, QMAN_FQ_STATE_VDQCR))
    goto out;
    fq_set(fq, QMAN_FQ_STATE_VDQCR);
    p.vdqcr_owned = fq;
    qm_dqrr_vdqcr_set(&p.p, vdqcr);
    ret = 0;
    out:
    local_irq_restore(irqflags);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn set_vdqcr(p: *mut qman_portal, fq: *mut qman_fq, vdqcr: u32) -> c_int {
    static int set_vdqcr(struct qman_portal **p, struct qman_fq *fq, u32 vdqcr)
    {
    int ret;
// p = get_affine_portal();
    ret = set_p_vdqcr(*p, fq, vdqcr);
    put_affine_portal();
    return ret;
    }
    static int wait_vdqcr_start(struct qman_portal **p, struct qman_fq *fq,
    u32 vdqcr, u32 flags)
    {
    let mut ret: c_int = 0;
    if (flags & QMAN_VOLATILE_FLAG_WAIT_INT)
    ret = wait_event_interruptible(affine_queue,
    !set_vdqcr(p, fq, vdqcr));
    else
    wait_event(affine_queue, !set_vdqcr(p, fq, vdqcr));
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn qman_volatile_dequeue(fq: *mut qman_fq, flags: u32, vdqcr: u32) -> c_int {
    int qman_volatile_dequeue(struct qman_fq *fq, u32 flags, u32 vdqcr)
    {
    struct qman_portal *p;
    int ret;
    if (fq.state != qman_fq_state_parked &&
    fq.state != qman_fq_state_retired)
    return -EINVAL;
    if (vdqcr & QM_VDQCR_FQID_MASK)
    return -EINVAL;
    if (fq_isset(fq, QMAN_FQ_STATE_VDQCR))
    return -EBUSY;
    vdqcr = (vdqcr & ~QM_VDQCR_FQID_MASK) | fq.fqid;
    if (flags & QMAN_VOLATILE_FLAG_WAIT)
    ret = wait_vdqcr_start(&p, fq, vdqcr, flags);
    else
    ret = set_vdqcr(&p, fq, vdqcr);
    if (ret)
    return ret;
// VDQCR is set
    if (flags & QMAN_VOLATILE_FLAG_FINISH) {
    if (flags & QMAN_VOLATILE_FLAG_WAIT_INT)
//
// NB: don't propagate any error - the caller wouldn't
// know whether the VDQCR was issued or not. A signal
// could arrive after returning anyway, so the caller
// can check signal_pending() if that's an issue.
//
    wait_event_interruptible(affine_queue,
    !fq_isset(fq, QMAN_FQ_STATE_VDQCR));
    else
    wait_event(affine_queue,
    !fq_isset(fq, QMAN_FQ_STATE_VDQCR));
    }
    return 0;
    }
    EXPORT_SYMBOL(qman_volatile_dequeue);
#[no_mangle]
unsafe extern "C" fn update_eqcr_ci(p: *mut qman_portal, avail: u8) {
    static void update_eqcr_ci(struct qman_portal *p, u8 avail)
    {
    if (avail)
    qm_eqcr_cce_prefetch(&p.p);
    else
    qm_eqcr_cce_update(&p.p);
    }
#[no_mangle]
pub unsafe extern "C" fn qman_enqueue(fq: *mut qman_fq, fd: *const qm_fd) -> c_int {
    int qman_enqueue(struct qman_fq *fq, const struct qm_fd *fd)
    {
    struct qman_portal *p;
    struct qm_eqcr_entry *eq;
    unsigned long irqflags;
    u8 avail;
    p = get_affine_portal();
    local_irq_save(irqflags);
    if (p.use_eqcr_ci_stashing) {
//
// The stashing case is easy, only update if we need to in
// order to try and liberate ring entries.
//
    eq = qm_eqcr_start_stash(&p.p);
    } else {
//
// The non-stashing case is harder, need to prefetch ahead of
// time.
//
    avail = qm_eqcr_get_avail(&p.p);
    if (avail < 2)
    update_eqcr_ci(p, avail);
    eq = qm_eqcr_start_no_stash(&p.p);
    }
    if (unlikely(!eq))
    goto out;
    qm_fqid_set(eq, fq.fqid);
    eq.tag = cpu_to_be32(fq_to_tag(fq));
    eq.fd = *fd;
    qm_eqcr_pvb_commit(&p.p, QM_EQCR_VERB_CMD_ENQUEUE);
    out:
    local_irq_restore(irqflags);
    put_affine_portal();
    return 0;
    }
    EXPORT_SYMBOL(qman_enqueue);
    static int qm_modify_cgr(struct qman_cgr *cgr, u32 flags,
    struct qm_mcc_initcgr *opts)
    {
    union qm_mc_command *mcc;
    union qm_mc_result *mcr;
    struct qman_portal *p = get_affine_portal();
    let mut verb: u8 = QM_MCC_VERB_MODIFYCGR;
    let mut ret: c_int = 0;
    mcc = qm_mc_start(&p.p);
    if (opts)
    mcc.initcgr = *opts;
    mcc.initcgr.cgid = cgr.cgrid;
    if (flags & QMAN_CGR_FLAG_USE_INIT)
    verb = QM_MCC_VERB_INITCGR;
    qm_mc_commit(&p.p, verb);
    if (!qm_mc_result_timeout(&p.p, &mcr)) {
    ret = -ETIMEDOUT;
    goto out;
    }
    DPAA_ASSERT((mcr.verb & QM_MCR_VERB_MASK) == verb);
    if (mcr.result != QM_MCR_RESULT_OK)
    ret = -EIO;
    out:
    put_affine_portal();
    return ret;
    }

// congestion state change notification target update control
#[no_mangle]
unsafe extern "C" fn qm_cgr_cscn_targ_set(cgr: *mut __qm_mc_cgr, pi: c_int, val: u32) {
    static void qm_cgr_cscn_targ_set(struct __qm_mc_cgr *cgr, int pi, u32 val)
    {
    if (qman_ip_rev >= QMAN_REV30)
    cgr.cscn_targ_upd_ctrl = cpu_to_be16(pi |
    QM_CGR_TARG_UDP_CTRL_WRITE_BIT);
    else
    cgr.cscn_targ = cpu_to_be32(val | QM_CGR_TARG_PORTAL(pi));
    }
#[no_mangle]
unsafe extern "C" fn qm_cgr_cscn_targ_clear(cgr: *mut __qm_mc_cgr, pi: c_int, val: u32) {
    static void qm_cgr_cscn_targ_clear(struct __qm_mc_cgr *cgr, int pi, u32 val)
    {
    if (qman_ip_rev >= QMAN_REV30)
    cgr.cscn_targ_upd_ctrl = cpu_to_be16(pi);
    else
    cgr.cscn_targ = cpu_to_be32(val & ~QM_CGR_TARG_PORTAL(pi));
    }
    static u8 qman_cgr_cpus[CGR_NUM];
#[no_mangle]
pub unsafe extern "C" fn qman_init_cgr_all() {
    void qman_init_cgr_all(void)
    {
    struct qman_cgr cgr;
    let mut err_cnt: c_int = 0;
    for (cgr.cgrid = 0; cgr.cgrid < CGR_NUM; cgr.cgrid++) {
    if (qm_modify_cgr(&cgr, QMAN_CGR_FLAG_USE_INIT, core::ptr::null_mut()))
    err_cnt++;
    }
    if (err_cnt)
    pr_err("Warning: %d error%s while initialising CGR h/w\n",
    err_cnt, (err_cnt > 1) ? "s" : "");
    }
    int qman_create_cgr(struct qman_cgr *cgr, u32 flags,
    struct qm_mcc_initcgr *opts)
    {
    struct qm_mcr_querycgr cgr_state;
    int ret;
    struct qman_portal *p;
//
// We have to check that the provided CGRID is within the limits of the
// data-structures, for obvious reasons. However we'll let h/w take
// care of determining whether it's within the limits of what exists on
// the SoC.
//
    if (cgr.cgrid >= CGR_NUM)
    return -EINVAL;
    preempt_disable();
    p = get_affine_portal();
    qman_cgr_cpus[cgr.cgrid] = smp_processor_id();
    preempt_enable();
    cgr.chan = p.config.channel;
    raw_spin_lock_irq(&p.cgr_lock);
    if (opts) {
    let mut local_opts: qm_mcc_initcgr = *opts;
    ret = qman_query_cgr(cgr, &cgr_state);
    if (ret)
    goto out;
    qm_cgr_cscn_targ_set(&local_opts.cgr, PORTAL_IDX(p),
    be32_to_cpu(cgr_state.cgr.cscn_targ));
    local_opts.we_mask |= cpu_to_be16(QM_CGR_WE_CSCN_TARG);
// send init if flags indicate so
    if (flags & QMAN_CGR_FLAG_USE_INIT)
    ret = qm_modify_cgr(cgr, QMAN_CGR_FLAG_USE_INIT,
    &local_opts);
    else
    ret = qm_modify_cgr(cgr, 0, &local_opts);
    if (ret)
    goto out;
    }
    list_add(&cgr.node, &p.cgr_cbs);
// Determine if newly added object requires its callback to be called
    ret = qman_query_cgr(cgr, &cgr_state);
    if (ret) {
// we can't go back, so proceed and return success
    dev_err(p.config.dev, "CGR HW state partially modified\n");
    ret = 0;
    goto out;
    }
    if (cgr.cb && cgr_state.cgr.cscn_en &&
    qman_cgrs_get(&p.cgrs[1], cgr.cgrid))
    cgr.cb(p, cgr, 1);
    out:
    raw_spin_unlock_irq(&p.cgr_lock);
    put_affine_portal();
    return ret;
    }
    EXPORT_SYMBOL(qman_create_cgr);
    static struct qman_portal *qman_cgr_get_affine_portal(struct qman_cgr *cgr)
    {
    struct qman_portal *p = get_affine_portal();
    if (cgr.chan != p.config.channel) {
// attempt to delete from other portal than creator
    dev_err(p.config.dev, "CGR not owned by current portal");
    dev_dbg(p.config.dev, " create 0x%x, delete 0x%x\n",
    cgr.chan, p.config.channel);
    put_affine_portal();
    return core::ptr::null_mut();
    }
    return p;
    }
#[no_mangle]
pub unsafe extern "C" fn qman_delete_cgr(cgr: *mut qman_cgr) -> c_int {
    int qman_delete_cgr(struct qman_cgr *cgr)
    {
    unsigned long irqflags;
    struct qm_mcr_querycgr cgr_state;
    struct qm_mcc_initcgr local_opts;
    let mut ret: c_int = 0;
    struct qman_cgr *i;
    struct qman_portal *p = qman_cgr_get_affine_portal(cgr);
    if (!p)
    return -EINVAL;
    memset(&local_opts, 0, sizeof(struct qm_mcc_initcgr));
    raw_spin_lock_irqsave(&p.cgr_lock, irqflags);
    list_del(&cgr.node);
//
// If there are no other CGR objects for this CGRID in the list,
// update CSCN_TARG accordingly
//
    list_for_each_entry(i, &p.cgr_cbs, node)
    if (i.cgrid == cgr.cgrid && i.cb)
    goto release_lock;
    ret = qman_query_cgr(cgr, &cgr_state);
    if (ret)  {
// add back to the list
    list_add(&cgr.node, &p.cgr_cbs);
    goto release_lock;
    }
    local_opts.we_mask = cpu_to_be16(QM_CGR_WE_CSCN_TARG);
    qm_cgr_cscn_targ_clear(&local_opts.cgr, PORTAL_IDX(p),
    be32_to_cpu(cgr_state.cgr.cscn_targ));
    ret = qm_modify_cgr(cgr, 0, &local_opts);
    if (ret)
// add back to the list
    list_add(&cgr.node, &p.cgr_cbs);
    release_lock:
    raw_spin_unlock_irqrestore(&p.cgr_lock, irqflags);
    put_affine_portal();
    return ret;
    }
    EXPORT_SYMBOL(qman_delete_cgr);
#[no_mangle]
unsafe extern "C" fn qman_delete_cgr_smp_call(p: *mut c_void) {
    static void qman_delete_cgr_smp_call(void *p)
    {
    qman_delete_cgr((struct qman_cgr *)p);
    }
#[no_mangle]
pub unsafe extern "C" fn qman_delete_cgr_safe(cgr: *mut qman_cgr) {
    void qman_delete_cgr_safe(struct qman_cgr *cgr)
    {
    preempt_disable();
    if (qman_cgr_cpus[cgr.cgrid] != smp_processor_id()) {
    smp_call_function_single(qman_cgr_cpus[cgr.cgrid],
    qman_delete_cgr_smp_call, cgr, true);
    preempt_enable();
    return;
    }
    qman_delete_cgr(cgr);
    preempt_enable();
    }
    EXPORT_SYMBOL(qman_delete_cgr_safe);
#[no_mangle]
unsafe extern "C" fn qman_update_cgr(cgr: *mut qman_cgr, opts: *mut qm_mcc_initcgr) -> c_int {
    static int qman_update_cgr(struct qman_cgr *cgr, struct qm_mcc_initcgr *opts)
    {
    int ret;
    unsigned long irqflags;
    struct qman_portal *p = qman_cgr_get_affine_portal(cgr);
    if (!p)
    return -EINVAL;
    raw_spin_lock_irqsave(&p.cgr_lock, irqflags);
    ret = qm_modify_cgr(cgr, 0, opts);
    raw_spin_unlock_irqrestore(&p.cgr_lock, irqflags);
    put_affine_portal();
    return ret;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct update_cgr_params {
    pub cgr: *mut qman_cgr,
    pub opts: *mut qm_mcc_initcgr,
    pub ret: c_int,
}

#[no_mangle]
unsafe extern "C" fn qman_update_cgr_smp_call(p: *mut c_void) {
    static void qman_update_cgr_smp_call(void *p)
    {
    struct update_cgr_params *params = p;
    params.ret = qman_update_cgr(params.cgr, params.opts);
    }
#[no_mangle]
pub unsafe extern "C" fn qman_update_cgr_safe(cgr: *mut qman_cgr, opts: *mut qm_mcc_initcgr) -> c_int {
    int qman_update_cgr_safe(struct qman_cgr *cgr, struct qm_mcc_initcgr *opts)
    {
    struct update_cgr_params params = {
    .cgr = cgr,
    .opts = opts,
    };
    preempt_disable();
    if (qman_cgr_cpus[cgr.cgrid] != smp_processor_id())
    smp_call_function_single(qman_cgr_cpus[cgr.cgrid],
    qman_update_cgr_smp_call, &params,
    true);
    else
    params.ret = qman_update_cgr(cgr, opts);
    preempt_enable();
    return params.ret;
    }
    EXPORT_SYMBOL(qman_update_cgr_safe);
// Cleanup FQs
#[no_mangle]
unsafe extern "C" fn _qm_mr_consume_and_match_verb(p: *mut qm_portal, v: c_int) -> c_int {
    static int _qm_mr_consume_and_match_verb(struct qm_portal *p, int v)
    {
    const union qm_mr_entry *msg;
    let mut found: c_int = 0;
    qm_mr_pvb_update(p);
    msg = qm_mr_current(p);
    while (msg) {
    if ((msg.verb & QM_MR_VERB_TYPE_MASK) == v)
    found = 1;
    qm_mr_next(p);
    qm_mr_cci_consume_to_current(p);
    qm_mr_pvb_update(p);
    msg = qm_mr_current(p);
    }
    return found;
    }
    static int _qm_dqrr_consume_and_match(struct qm_portal *p, u32 fqid, int s,
    bool wait)
    {
    const struct qm_dqrr_entry *dqrr;
    let mut found: c_int = 0;
    do {
    qm_dqrr_pvb_update(p);
    dqrr = qm_dqrr_current(p);
    if (!dqrr)
    cpu_relax();
    } while (wait && !dqrr);
    while (dqrr) {
    if (qm_fqid_get(dqrr) == fqid && (dqrr.stat & s))
    found = 1;
    qm_dqrr_cdc_consume_1ptr(p, dqrr, 0);
    qm_dqrr_pvb_update(p);
    qm_dqrr_next(p);
    dqrr = qm_dqrr_current(p);
    }
    return found;
    }

    _qm_mr_consume_and_match_verb(p, QM_MR_VERB_##V)

    _qm_dqrr_consume_and_match(p, f, QM_DQRR_STAT_##S, false)

    _qm_dqrr_consume_and_match(p, f, QM_DQRR_STAT_##S, true)

    _qm_dqrr_consume_and_match(p, 0, 0, false)
#[no_mangle]
pub unsafe extern "C" fn qman_shutdown_fq(fqid: u32) -> c_int {
    int qman_shutdown_fq(u32 fqid)
    {
    struct qman_portal *p, *channel_portal;
    struct device *dev;
    union qm_mc_command *mcc;
    union qm_mc_result *mcr;
    int orl_empty, drain = 0, ret = 0;
    u32 channel, res;
    u8 state;
    p = get_affine_portal();
    dev = p.config.dev;
// Determine the state of the FQID
    mcc = qm_mc_start(&p.p);
    qm_fqid_set(&mcc.fq, fqid);
    qm_mc_commit(&p.p, QM_MCC_VERB_QUERYFQ_NP);
    if (!qm_mc_result_timeout(&p.p, &mcr)) {
    dev_err(dev, "QUERYFQ_NP timeout\n");
    ret = -ETIMEDOUT;
    goto out;
    }
    DPAA_ASSERT((mcr.verb & QM_MCR_VERB_MASK) == QM_MCR_VERB_QUERYFQ_NP);
    state = mcr.queryfq_np.state & QM_MCR_NP_STATE_MASK;
    if (state == QM_MCR_NP_STATE_OOS)
    goto out; /* Already OOS, no need to do anymore checks */
// Query which channel the FQ is using
    mcc = qm_mc_start(&p.p);
    qm_fqid_set(&mcc.fq, fqid);
    qm_mc_commit(&p.p, QM_MCC_VERB_QUERYFQ);
    if (!qm_mc_result_timeout(&p.p, &mcr)) {
    dev_err(dev, "QUERYFQ timeout\n");
    ret = -ETIMEDOUT;
    goto out;
    }
    DPAA_ASSERT((mcr.verb & QM_MCR_VERB_MASK) == QM_MCR_VERB_QUERYFQ);
// Need to store these since the MCR gets reused
    channel = qm_fqd_get_chan(&mcr.queryfq.fqd);
    qm_fqd_get_wq(&mcr.queryfq.fqd);
    if (channel < qm_channel_pool1) {
    channel_portal = get_portal_for_channel(channel);
    if (channel_portal == core::ptr::null_mut()) {
    dev_err(dev, "Can't find portal for dedicated channel 0x%x\n",
    channel);
    ret = -EIO;
    goto out;
    }
    } else
    channel_portal = p;
    switch (state) {
    case QM_MCR_NP_STATE_TEN_SCHED:
    case QM_MCR_NP_STATE_TRU_SCHED:
    case QM_MCR_NP_STATE_ACTIVE:
    case QM_MCR_NP_STATE_PARKED:
    orl_empty = 0;
    mcc = qm_mc_start(&channel_portal.p);
    qm_fqid_set(&mcc.fq, fqid);
    qm_mc_commit(&channel_portal.p, QM_MCC_VERB_ALTER_RETIRE);
    if (!qm_mc_result_timeout(&channel_portal.p, &mcr)) {
    dev_err(dev, "ALTER_RETIRE timeout\n");
    ret = -ETIMEDOUT;
    goto out;
    }
    DPAA_ASSERT((mcr.verb & QM_MCR_VERB_MASK) ==
    QM_MCR_VERB_ALTER_RETIRE);
    res = mcr.result; /* Make a copy as we reuse MCR below */
    if (res == QM_MCR_RESULT_OK)
    drain_mr_fqrni(&channel_portal.p);
    if (res == QM_MCR_RESULT_PENDING) {
//
// Need to wait for the FQRN in the message ring, which
// will only occur once the FQ has been drained.  In
// order for the FQ to drain the portal needs to be set
// to dequeue from the channel the FQ is scheduled on
//
    let mut found_fqrn: c_int = 0;
// Flag that we need to drain FQ
    drain = 1;
    if (channel >= qm_channel_pool1 &&
    channel < qm_channel_pool1 + 15) {
// Pool channel, enable the bit in the portal
    } else if (channel < qm_channel_pool1) {
// Dedicated channel
    } else {
    dev_err(dev, "Can't recover FQ 0x%x, ch: 0x%x",
    fqid, channel);
    ret = -EBUSY;
    goto out;
    }
// Set the sdqcr to drain this channel
    if (channel < qm_channel_pool1)
    qm_dqrr_sdqcr_set(&channel_portal.p,
    QM_SDQCR_TYPE_ACTIVE |
    QM_SDQCR_CHANNELS_DEDICATED);
    else
    qm_dqrr_sdqcr_set(&channel_portal.p,
    QM_SDQCR_TYPE_ACTIVE |
    QM_SDQCR_CHANNELS_POOL_CONV
    (channel));
    do {
// Keep draining DQRR while checking the MR
    qm_dqrr_drain_nomatch(&channel_portal.p);
// Process message ring too
    found_fqrn = qm_mr_drain(&channel_portal.p,
    FQRN);
    cpu_relax();
    } while (!found_fqrn);
// Restore SDQCR
    qm_dqrr_sdqcr_set(&channel_portal.p,
    channel_portal.sdqcr);
    }
    if (res != QM_MCR_RESULT_OK &&
    res != QM_MCR_RESULT_PENDING) {
    dev_err(dev, "retire_fq failed: FQ 0x%x, res=0x%x\n",
    fqid, res);
    ret = -EIO;
    goto out;
    }
    if (!(mcr.alterfq.fqs & QM_MCR_FQS_ORLPRESENT)) {
//
// ORL had no entries, no need to wait until the
// ERNs come in
//
    orl_empty = 1;
    }
//
// Retirement succeeded, check to see if FQ needs
// to be drained
//
    if (drain || mcr.alterfq.fqs & QM_MCR_FQS_NOTEMPTY) {
// FQ is Not Empty, drain using volatile DQ commands
    do {
    let mut vdqcr: u32 = fqid | QM_VDQCR_NUMFRAMES_SET(3);
    qm_dqrr_vdqcr_set(&p.p, vdqcr);
//
// Wait for a dequeue and process the dequeues,
// making sure to empty the ring completely
//
    } while (!qm_dqrr_drain_wait(&p.p, fqid, FQ_EMPTY));
    }
    while (!orl_empty) {
// Wait for the ORL to have been completely drained
    orl_empty = qm_mr_drain(&p.p, FQRL);
    cpu_relax();
    }
    mcc = qm_mc_start(&p.p);
    qm_fqid_set(&mcc.fq, fqid);
    qm_mc_commit(&p.p, QM_MCC_VERB_ALTER_OOS);
    if (!qm_mc_result_timeout(&p.p, &mcr)) {
    ret = -ETIMEDOUT;
    goto out;
    }
    DPAA_ASSERT((mcr.verb & QM_MCR_VERB_MASK) ==
    QM_MCR_VERB_ALTER_OOS);
    if (mcr.result != QM_MCR_RESULT_OK) {
    dev_err(dev, "OOS after drain fail: FQ 0x%x (0x%x)\n",
    fqid, mcr.result);
    ret = -EIO;
    goto out;
    }
    break;
    case QM_MCR_NP_STATE_RETIRED:
// Send OOS Command
    mcc = qm_mc_start(&p.p);
    qm_fqid_set(&mcc.fq, fqid);
    qm_mc_commit(&p.p, QM_MCC_VERB_ALTER_OOS);
    if (!qm_mc_result_timeout(&p.p, &mcr)) {
    ret = -ETIMEDOUT;
    goto out;
    }
    DPAA_ASSERT((mcr.verb & QM_MCR_VERB_MASK) ==
    QM_MCR_VERB_ALTER_OOS);
    if (mcr.result != QM_MCR_RESULT_OK) {
    dev_err(dev, "OOS fail: FQ 0x%x (0x%x)\n",
    fqid, mcr.result);
    ret = -EIO;
    goto out;
    }
    break;
    case QM_MCR_NP_STATE_OOS:
// Done
    break;
    default:
    ret = -EIO;
    }
    out:
    put_affine_portal();
    return ret;
    }
    const struct qm_portal_config *qman_get_qm_portal_config(
    struct qman_portal *portal)
    {
    return portal.config;
    }
    EXPORT_SYMBOL(qman_get_qm_portal_config);
    struct gen_pool *qm_fqalloc; /* FQID allocator */
    struct gen_pool *qm_qpalloc; /* pool-channel allocator */
    struct gen_pool *qm_cgralloc; /* CGR ID allocator */
#[no_mangle]
unsafe extern "C" fn qman_alloc_range(p: *mut gen_pool, result: *mut u32, cnt: u32) -> c_int {
    static int qman_alloc_range(struct gen_pool *p, u32 *result, u32 cnt)
    {
    unsigned long addr;
    if (!p)
    return -ENODEV;
    addr = gen_pool_alloc(p, cnt);
    if (!addr)
    return -ENOMEM;
// result = addr & ~DPAA_GENALLOC_OFF;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn qman_alloc_fqid_range(result: *mut u32, count: u32) -> c_int {
    int qman_alloc_fqid_range(u32 *result, u32 count)
    {
    return qman_alloc_range(qm_fqalloc, result, count);
    }
    EXPORT_SYMBOL(qman_alloc_fqid_range);
#[no_mangle]
pub unsafe extern "C" fn qman_alloc_pool_range(result: *mut u32, count: u32) -> c_int {
    int qman_alloc_pool_range(u32 *result, u32 count)
    {
    return qman_alloc_range(qm_qpalloc, result, count);
    }
    EXPORT_SYMBOL(qman_alloc_pool_range);
#[no_mangle]
pub unsafe extern "C" fn qman_alloc_cgrid_range(result: *mut u32, count: u32) -> c_int {
    int qman_alloc_cgrid_range(u32 *result, u32 count)
    {
    return qman_alloc_range(qm_cgralloc, result, count);
    }
    EXPORT_SYMBOL(qman_alloc_cgrid_range);
#[no_mangle]
pub unsafe extern "C" fn qman_release_fqid(fqid: u32) -> c_int {
    int qman_release_fqid(u32 fqid)
    {
    let mut ret: c_int = qman_shutdown_fq(fqid);
    if (ret) {
    pr_debug("FQID %d leaked\n", fqid);
    return ret;
    }
    gen_pool_free(qm_fqalloc, fqid | DPAA_GENALLOC_OFF, 1);
    return 0;
    }
    EXPORT_SYMBOL(qman_release_fqid);
#[no_mangle]
unsafe extern "C" fn qpool_cleanup(qp: u32) -> c_int {
    static int qpool_cleanup(u32 qp)
    {
//
// We query all FQDs starting from
// FQID 1 until we get an "invalid FQID" error, looking for non-OOS FQDs
// whose destination channel is the pool-channel being released.
// When a non-OOS FQD is found we attempt to clean it up
//
    struct qman_fq fq = {
    .fqid = QM_FQID_RANGE_START
    };
    int err;
    do {
    struct qm_mcr_queryfq_np np;
    err = qman_query_fq_np(&fq, &np);
    if (err == -ERANGE)
// FQID range exceeded, found no problems
    return 0;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: WARN_ON(err)) -> else {
    else if (WARN_ON(err))
    return err;
    if ((np.state & QM_MCR_NP_STATE_MASK) != QM_MCR_NP_STATE_OOS) {
    struct qm_fqd fqd;
    err = qman_query_fq(&fq, &fqd);
    if (WARN_ON(err))
    return err;
    if (qm_fqd_get_chan(&fqd) == qp) {
// The channel is the FQ's target, clean it
    err = qman_shutdown_fq(fq.fqid);
    if (err)
//
// Couldn't shut down the FQ
// so the pool must be leaked
//
    return err;
    }
    }
// Move to the next FQID
    fq.fqid++;
    } while (1);
    }
#[no_mangle]
pub unsafe extern "C" fn qman_release_pool(qp: u32) -> c_int {
    int qman_release_pool(u32 qp)
    {
    int ret;
    ret = qpool_cleanup(qp);
    if (ret) {
    pr_debug("CHID %d leaked\n", qp);
    return ret;
    }
    gen_pool_free(qm_qpalloc, qp | DPAA_GENALLOC_OFF, 1);
    return 0;
    }
    EXPORT_SYMBOL(qman_release_pool);
#[no_mangle]
unsafe extern "C" fn cgr_cleanup(cgrid: u32) -> c_int {
    static int cgr_cleanup(u32 cgrid)
    {
//
// query all FQDs starting from FQID 1 until we get an "invalid FQID"
// error, looking for non-OOS FQDs whose CGR is the CGR being released
//
    struct qman_fq fq = {
    .fqid = QM_FQID_RANGE_START
    };
    int err;
    do {
    struct qm_mcr_queryfq_np np;
    err = qman_query_fq_np(&fq, &np);
    if (err == -ERANGE)
// FQID range exceeded, found no problems
    return 0;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: WARN_ON(err)) -> else {
    else if (WARN_ON(err))
    return err;
    if ((np.state & QM_MCR_NP_STATE_MASK) != QM_MCR_NP_STATE_OOS) {
    struct qm_fqd fqd;
    err = qman_query_fq(&fq, &fqd);
    if (WARN_ON(err))
    return err;
    if (be16_to_cpu(fqd.fq_ctrl) & QM_FQCTRL_CGE &&
    fqd.cgid == cgrid) {
    pr_err("CRGID 0x%x is being used by FQID 0x%x, CGR will be leaked\n",
    cgrid, fq.fqid);
    return -EIO;
    }
    }
// Move to the next FQID
    fq.fqid++;
    } while (1);
    }
#[no_mangle]
pub unsafe extern "C" fn qman_release_cgrid(cgrid: u32) -> c_int {
    int qman_release_cgrid(u32 cgrid)
    {
    int ret;
    ret = cgr_cleanup(cgrid);
    if (ret) {
    pr_debug("CGRID %d leaked\n", cgrid);
    return ret;
    }
    gen_pool_free(qm_cgralloc, cgrid | DPAA_GENALLOC_OFF, 1);
    return 0;
    }
    EXPORT_SYMBOL(qman_release_cgrid);
