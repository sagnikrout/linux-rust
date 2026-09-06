//! Automatically rewritten from C Header to Rust Module
//! Source: include/soc/fsl/qman.h
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

// Hardware constants
pub const QM_CHANNEL_SWPORTAL0: c_int = 0;
pub const QMAN_CHANNEL_POOL1: c_uint = 0x21;
pub const QMAN_CHANNEL_CAAM: c_uint = 0x80;
pub const QMAN_CHANNEL_POOL1_REV3: c_uint = 0x401;
pub const QMAN_CHANNEL_CAAM_REV3: c_uint = 0x840;
// Portal processing (interrupt) sources
pub const QM_PIRQ_CSCI: c_uint = 0x00100000	/* Congestion State Change */;
pub const QM_PIRQ_EQCI: c_uint = 0x00080000	/* Enqueue Command Committed */;
pub const QM_PIRQ_EQRI: c_uint = 0x00040000	/* EQCR Ring (below threshold) */;
pub const QM_PIRQ_DQRI: c_uint = 0x00020000	/* DQRR Ring (non-empty) */;
pub const QM_PIRQ_MRI: c_uint = 0x00010000	/* MR Ring (non-empty) */;
//
// This mask contains all the interrupt sources that need handling except DQRI,
// ie. that if present should trigger slow-path processing.
//

// For qman_static_dequeue_*** APIs
pub const QM_SDQCR_CHANNELS_POOL_MASK: c_uint = 0x00007fff;
// for n in [1,15]

// for conversion from n of qm_channel
extern "C" {
    pub fn QM_SDQCR_CHANNELS_POOL(qm_channel_pool1: channel + 1 -) -> return;
}
// --- QMan data structures (and associated constants) ---
// "Frame Descriptor (FD)"
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qm_fd {
    pub cfg8b_w1: u8,
    pub /: *mut *mut u8 bpid; / Buffer Pool ID,
    pub cfg8b_w3: u8,
    pub /: *mut *mut u8 addr_hi; / high 8-bits of 40-bit address,
    pub /: *mut *mut __be32 addr_lo; / low 32-bits of 40-bit address,
    pub __packed: },
    pub data: __be64,
}

pub const QM_FD_OFF_SHIFT: c_int = 20;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qm_fd_format {
//
// 'contig' implies a contiguous buffer, whereas 'sg' implies a
// scatter-gather table. 'big' implies a 29-bit length with no offset
// field, otherwise length is 20-bit and offset is 9-bit. 'compound'
// implies a s/g-like table, where each entry itself represents a frame
// (contiguous or scatter-gather) and the 29-bit "length" is
// interpreted purely for congestion calculations, ie. a "congestion
// weight".
//
    qm_fd_contig = 0,
    qm_fd_contig_big = QM_FD_FORMAT_LONG,
    qm_fd_sg = QM_FD_FORMAT_SG,
    qm_fd_sg_big = QM_FD_FORMAT_SG | QM_FD_FORMAT_LONG,
    qm_fd_compound = QM_FD_FORMAT_COMPOUND
}

//
// The 'format' field indicates the interpretation of the remaining
// 29 bits of the 32-bit word.
// If 'format' is _contig or _sg, 20b length and 9b offset.
// If 'format' is _contig_big or _sg_big, 29b length.
// If 'format' is _compound, 29b "congestion weight".
//

// Scatter/Gather table entry
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qm_sg_entry {
    pub __reserved1: [u8; 3],
    pub /: *mut *mut u8 addr_hi; / high 8-bits of 40-bit address,
    pub /: *mut *mut __be32 addr_lo; / low 32-bits of 40-bit address,
}

// "Frame Dequeue Response"
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qm_dqrr_entry {
    pub verb: u8,
    pub stat: u8,
    pub /: *mut *mut __be16 seqnum; / 15-bit,
    pub tok: u8,
    pub __reserved2: [u8; 3],
    pub /: *mut *mut __be32 fqid; / 24-bit,
    pub context_b: __be32,
    pub fd: qm_fd,
    pub __reserved4: [u8; 32],
    pub __aligned(64): } __packed,
pub const QM_DQRR_VERB_VBIT: c_uint = 0x80;
pub const QM_DQRR_VERB_MASK: c_uint = 0x7f	/* where the verb contains; */;
pub const QM_DQRR_VERB_FRAME_DEQUEUE: c_uint = 0x60	/* "this format" */;
pub const QM_DQRR_STAT_FQ_EMPTY: c_uint = 0x80	/* FQ empty */;
pub const QM_DQRR_STAT_FQ_HELDACTIVE: c_uint = 0x40	/* FQ held active */;
pub const QM_DQRR_STAT_FQ_FORCEELIGIBLE: c_uint = 0x20	/* FQ was force-eligible'd */;
pub const QM_DQRR_STAT_FD_VALID: c_uint = 0x10	/* has a non-NULL FD */;
pub const QM_DQRR_STAT_UNSCHEDULED: c_uint = 0x02	/* Unscheduled dequeue */;
pub const QM_DQRR_STAT_DQCR_EXPIRED: c_uint = 0x01	/* VDQCR or PDQCR expired*/;
// 'fqid' is a 24-bit field in every h/w descriptor

// "ERN Message Response"
// "FQ State Change Notification"
#[repr(C)]
#[derive(Copy, Clone)]
pub union qm_mr_entry {
    pub verb: u8,
    pub __reserved: [u8; 63],
}

pub const QM_MR_VERB_VBIT: c_uint = 0x80;
//
// ERNs originating from direct-connect portals ("dcern") use 0x20 as a verb
// which would be invalid as a s/w enqueue verb. A s/w ERN can be distinguished
// from the other MR types by noting if the 0x20 bit is unset.
//
pub const QM_MR_VERB_TYPE_MASK: c_uint = 0x27;
pub const QM_MR_VERB_DC_ERN: c_uint = 0x20;
pub const QM_MR_VERB_FQRN: c_uint = 0x21;
pub const QM_MR_VERB_FQRNI: c_uint = 0x22;
pub const QM_MR_VERB_FQRL: c_uint = 0x23;
pub const QM_MR_VERB_FQPN: c_uint = 0x24;
pub const QM_MR_RC_MASK: c_uint = 0xf0	/* contains one of; */;
pub const QM_MR_RC_CGR_TAILDROP: c_uint = 0x00;
pub const QM_MR_RC_WRED: c_uint = 0x10;
pub const QM_MR_RC_ERROR: c_uint = 0x20;
pub const QM_MR_RC_ORPWINDOW_EARLY: c_uint = 0x30;
pub const QM_MR_RC_ORPWINDOW_LATE: c_uint = 0x40;
pub const QM_MR_RC_FQ_TAILDROP: c_uint = 0x50;
pub const QM_MR_RC_ORPWINDOW_RETIRED: c_uint = 0x60;
pub const QM_MR_RC_ORP_ZERO: c_uint = 0x70;
pub const QM_MR_FQS_ORLPRESENT: c_uint = 0x02	/* ORL fragments to come */;
pub const QM_MR_FQS_NOTEMPTY: c_uint = 0x01	/* FQ has enqueued frames */;
//
// An identical structure of FQD fields is present in the "Init FQ" command and
// the "Query FQ" result, it's suctioned out into the "struct qm_fqd" type.
// Within that, the 'stashing' and 'taildrop' pieces are also factored out, the
// latter has two inlines to assist with converting to/from the mant+exp
// representation.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qm_fqd_stashing {
// See QM_STASHING_EXCL_<...>
    pub exclusive: u8,
// Numbers of cachelines
    pub /: *mut *mut u8 cl; / _res[6-7], as[4-5], ds[2-3], cs[0-1],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qm_fqd_oac {
// "Overhead Accounting Control", see QM_OAC_<...>
    pub /: *mut *mut u8 oac; / oac[6-7], _res[0-5],
// Two's-complement value (-128 to +127)
    pub /: *mut *mut s8 oal; / "Overhead Accounting Length",
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qm_fqd {
// _res[6-7], orprws[3-5], oa[2], olws[0-1]
    pub orpc: u8,
    pub cgid: u8,
    pub /: *mut *mut __be16 fq_ctrl; / See QM_FQCTRL_<...>,
    pub /: *mut *mut __be16 dest_wq; / channel[3-15], wq[0-2],
    pub /: *mut *mut __be16 ics_cred; / 15-bit,
//
// For "Initialize Frame Queue" commands, the write-enable mask
// determines whether 'td' or 'oac_init' is observed. For query
// commands, this field is always 'td', and 'oac_query' (below) reflects
// the Overhead ACcounting values.
//
    pub /: *mut *mut __be16 td; / "Taildrop": _res[13-15], mant[5-12], exp[0-4],
    pub oac_init: qm_fqd_oac,
}

// Treat it as 64-bit opaque
// Treat it as s/w portal stashing config
// see "FQD Context_A field used for [...]"
//
// 48-bit address of FQ context to
// stash, must be cacheline-aligned
//
pub const QM_FQD_CHAN_OFF: c_int = 3;

pub const QM_FQD_TD_MANT_OFF: c_int = 5;

pub const QM_FQD_TD_MAX: c_uint = 0xe0000000;
pub const QM_FQD_TD_MANT_MAX: c_uint = 0xff;
pub const QM_FQD_OAC_OFF: c_int = 6;
pub const QM_FQD_AS_OFF: c_int = 4;
pub const QM_FQD_DS_OFF: c_int = 2;
pub const QM_FQD_XS_MASK: c_uint = 0x3;
// 64-bit converters for context_hi/lo
extern "C" {
    pub fn qm_fqd_stashing_get64(_arg: fqd) -> return;
}
// convert a threshold value into mant+exp representation
// and the other direction
// See "Frame Queue Descriptor (FQD)"
// Frame Queue Descriptor (FQD) field 'fq_ctrl' uses these constants
pub const QM_FQCTRL_MASK: c_uint = 0x07ff	/* 'fq_ctrl' flags; */;
pub const QM_FQCTRL_CGE: c_uint = 0x0400	/* Congestion Group Enable */;
pub const QM_FQCTRL_TDE: c_uint = 0x0200	/* Tail-Drop Enable */;
pub const QM_FQCTRL_CTXASTASHING: c_uint = 0x0080	/* Context-A stashing */;
pub const QM_FQCTRL_CPCSTASH: c_uint = 0x0040	/* CPC Stash Enable */;
pub const QM_FQCTRL_FORCESFDR: c_uint = 0x0008	/* High-priority SFDRs */;
pub const QM_FQCTRL_AVOIDBLOCK: c_uint = 0x0004	/* Don't block active */;
pub const QM_FQCTRL_HOLDACTIVE: c_uint = 0x0002	/* Hold active in portal */;
pub const QM_FQCTRL_PREFERINCACHE: c_uint = 0x0001	/* Aggressively cache FQD */;

// See "FQD Context_A field used for [...]
// Frame Queue Descriptor (FQD) field 'CONTEXT_A' uses these constants
pub const QM_STASHING_EXCL_ANNOTATION: c_uint = 0x04;
pub const QM_STASHING_EXCL_DATA: c_uint = 0x02;
pub const QM_STASHING_EXCL_CTX: c_uint = 0x01;
// See "Intra Class Scheduling"
// FQD field 'OAC' (Overhead ACcounting) uses these constants
pub const QM_OAC_ICS: c_uint = 0x2 /* Accounting for Intra-Class Scheduling */;
pub const QM_OAC_CG: c_uint = 0x1 /* Accounting for Congestion Groups */;
//
// This struct represents the 32-bit "WR_PARM_[GYR]" parameters in CGR fields
// and associated commands/responses. The WRED parameters are calculated from
// these fields as follows;
// MaxTH = MA * (2 ^ Mn)
// Slope = SA / (2 ^ Sn)
// MaxP = 4 * (Pn + 1)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qm_cgr_wr_parm {
// MA[24-31], Mn[19-23], SA[12-18], Sn[6-11], Pn[0-5]
    pub word: __be32,
}

//
// This struct represents the 13-bit "CS_THRES" CGR field. In the corresponding
// management commands, this is padded to a 16-bit structure field, so that's
// how we represent it here. The congestion state threshold is calculated from
// these fields as follows;
// CS threshold = TA * (2 ^ Tn)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qm_cgr_cs_thres {
// _res[13-15], TA[5-12], Tn[0-4]
    pub word: __be16,
}

//
// This identical structure of CGR fields is present in the "Init/Modify CGR"
// commands and the "Query CGR" result. It's suctioned out here into its own
// struct.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __qm_mc_cgr {
    pub wr_parm_g: qm_cgr_wr_parm,
    pub wr_parm_y: qm_cgr_wr_parm,
    pub wr_parm_r: qm_cgr_wr_parm,
    pub /: *mut *mut u8 wr_en_g; / boolean, use QM_CGR_EN,
    pub /: *mut *mut u8 wr_en_y; / boolean, use QM_CGR_EN,
    pub /: *mut *mut u8 wr_en_r; / boolean, use QM_CGR_EN,
    pub /: *mut *mut u8 cscn_en; / boolean, use QM_CGR_EN,
    pub /: *mut *mut *mut __be16 cscn_targ_upd_ctrl; / use QM_CGR_TARG_UDP_,
    pub cscn_targ_dcp_low: __be16,
}

pub const QM_CGR_EN: c_uint = 0x01 /* For wr_en_*, cscn_en, cstd_en */;
pub const QM_CGR_TARG_UDP_CTRL_WRITE_BIT: c_uint = 0x8000 /* value written to portal bit*/;
pub const QM_CGR_TARG_UDP_CTRL_DCP: c_uint = 0x4000 /* 0: SWP, 1: DCP */;

pub const QM_CGR_TARG_FMAN0: c_uint = 0x00200000 /* direct-connect portal: fman0 */;
pub const QM_CGR_TARG_FMAN1: c_uint = 0x00100000 /*			   : fman1 */;
// Convert CGR thresholds to/from "cs_thres" format
// "Initialize FQ"
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qm_mcc_initfq {
    pub __reserved1: [u8; 2],
    pub /: *mut *mut __be16 we_mask; / Write Enable Mask,
    pub /: *mut *mut __be32 fqid; / 24-bit,
    pub /: *mut *mut __be16 count; / Initialises 'count+1' FQDs,
    pub /: *mut *mut qm_fqd fqd; / the FQD fields go here,
    pub __reserved2: [u8; 30],
    pub __packed: },
// "Initialize/Modify CGR"
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qm_mcc_initcgr {
    pub __reserve1: [u8; 2],
    pub /: *mut *mut __be16 we_mask; / Write Enable Mask,
    pub /: *mut *mut __qm_mc_cgr cgr; / CGR fields,
    pub __reserved2: [u8; 2],
    pub cgid: u8,
    pub __reserved3: [u8; 32],
    pub __packed: },
// INITFQ-specific flags
pub const QM_INITFQ_WE_MASK: c_uint = 0x01ff	/* 'Write Enable' flags; */;
pub const QM_INITFQ_WE_OAC: c_uint = 0x0100;
pub const QM_INITFQ_WE_ORPC: c_uint = 0x0080;
pub const QM_INITFQ_WE_CGID: c_uint = 0x0040;
pub const QM_INITFQ_WE_FQCTRL: c_uint = 0x0020;
pub const QM_INITFQ_WE_DESTWQ: c_uint = 0x0010;
pub const QM_INITFQ_WE_ICSCRED: c_uint = 0x0008;
pub const QM_INITFQ_WE_TDTHRESH: c_uint = 0x0004;
pub const QM_INITFQ_WE_CONTEXTB: c_uint = 0x0002;
pub const QM_INITFQ_WE_CONTEXTA: c_uint = 0x0001;
// INITCGR/MODIFYCGR-specific flags
pub const QM_CGR_WE_MASK: c_uint = 0x07ff	/* 'Write Enable Mask'; */;
pub const QM_CGR_WE_WR_PARM_G: c_uint = 0x0400;
pub const QM_CGR_WE_WR_PARM_Y: c_uint = 0x0200;
pub const QM_CGR_WE_WR_PARM_R: c_uint = 0x0100;
pub const QM_CGR_WE_WR_EN_G: c_uint = 0x0080;
pub const QM_CGR_WE_WR_EN_Y: c_uint = 0x0040;
pub const QM_CGR_WE_WR_EN_R: c_uint = 0x0020;
pub const QM_CGR_WE_CSCN_EN: c_uint = 0x0010;
pub const QM_CGR_WE_CSCN_TARG: c_uint = 0x0008;
pub const QM_CGR_WE_CSTD_EN: c_uint = 0x0004;
pub const QM_CGR_WE_CS_THRES: c_uint = 0x0002;
pub const QM_CGR_WE_MODE: c_uint = 0x0001;
pub const QMAN_CGR_FLAG_USE_INIT: c_uint = 0x00000001;
pub const QMAN_CGR_MODE_FRAME: c_uint = 0x00000001;
// Portal and Frame Queues
// Represents a managed portal
    pub qman_portal: struct,
//
// This object type represents QMan frame queue descriptors (FQD), it is
// cacheline-aligned, and initialised by qman_create_fq(). The structure is
// defined further down.
//
    pub qman_fq: struct,
//
// This object type represents a QMan congestion group, it is defined further
// down.
//
    pub qman_cgr: struct,
//
// This enum, and the callback type that returns it, are used when handling
// dequeued frames via DQRR. Note that for "null" callbacks registered with the
// portal object (for handling dequeues that do not demux because context_b is
// NULL), the return value *MUST* be qman_cb_dqrr_consume.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qman_cb_dqrr_result {
// DQRR entry can be consumed
    qman_cb_dqrr_consume,
// Like _consume, but requests parking - FQ must be held-active
    qman_cb_dqrr_park,
// Does not consume, for DCA mode only.
    qman_cb_dqrr_defer,
//
// Stop processing without consuming this ring entry. Exits the current
// qman_p_poll_dqrr() or interrupt-handling, as appropriate. If within
// an interrupt handler, the callback would typically call
// qman_irqsource_remove(QM_PIRQ_DQRI) before returning this value,
// otherwise the interrupt will reassert immediately.
//
    qman_cb_dqrr_stop,
// Like qman_cb_dqrr_stop, but consumes the current entry.
    qman_cb_dqrr_consume_stop
}

    pub sched_napi): bool,
//
// This callback type is used when handling ERNs, FQRNs and FQRLs via MR. They
// are always consumed after the callback returns.
//
    pub msg): *const qm_mr_entry,
//
// s/w-visible states. Ie. tentatively scheduled + truly scheduled + active +
// held-active + held-suspended are just "sched". Things like "retired" will not
// be assumed until it is complete (ie. QMAN_FQ_STATE_CHANGING is set until
// then, to indicate it's completing and to gate attempts to retry the retire
// command). Note, park commands do not set QMAN_FQ_STATE_CHANGING because it's
// technically impossible in the case of enqueue DCAs (which refer to DQRR ring
// index rather than the FQ that ring entry corresponds to), so repeated park
// commands are allowed (if you're silly enough to try) but won't change FQ
// state, and the resulting park notifications move FQs from "sched" to
// "parked".
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qman_fq_state {
    qman_fq_state_oos,
    qman_fq_state_parked,
    qman_fq_state_sched,
    qman_fq_state_retired
}

pub const QMAN_FQ_STATE_CHANGING: c_uint = 0x80000000 /* 'state' is changing */;
pub const QMAN_FQ_STATE_NE: c_uint = 0x40000000 /* retired FQ isn't empty */;
pub const QMAN_FQ_STATE_ORL: c_uint = 0x20000000 /* retired FQ has ORL */;
pub const QMAN_FQ_STATE_BLOCKOOS: c_uint = 0xe0000000 /* if any are set, no OOS */;
pub const QMAN_FQ_STATE_CGR_EN: c_uint = 0x10000000 /* CGR enabled */;
pub const QMAN_FQ_STATE_VDQCR: c_uint = 0x08000000 /* being volatile dequeued */;
//
// Frame queue objects (struct qman_fq) are stored within memory passed to
// qman_create_fq(), as this allows stashing of caller-provided demux callback
// pointers at no extra cost to stashing of (driver-internal) FQ state. If the
// caller wishes to add per-FQ state and have it benefit from dequeue-stashing,
// they should;
//
// (a) extend the qman_fq structure with their state; eg.
//
// // myfq is allocated and driver_fq callbacks filled in;
// struct my_fq {
// struct qman_fq base;
// int an_extra_field;
// [ ... add other fields to be associated with each FQ ...]
// } *myfq = some_my_fq_allocator();
// struct qman_fq *fq = qman_create_fq(fqid, flags, &myfq->base);
//
// // in a dequeue callback, access extra fields from 'fq' via a cast;
// struct my_fq *myfq = (struct my_fq *)fq;
// do_something_with(myfq->an_extra_field);
// [...]
//
// (b) when and if configuring the FQ for context stashing, specify how ever
// many cachelines are required to stash 'struct my_fq', to accelerate not
// only the QMan driver but the callback as well.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qman_fq_cb {
    pub /: *mut *mut qman_cb_dqrr dqrr; / for dequeued frames,
    pub /: *mut *mut qman_cb_mr ern; / for s/w ERNs,
    pub changes*/: *mut *mut qman_cb_mr fqs; / frame-queue state,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qman_fq {
// Caller of qman_create_fq() provides these demux callbacks
    pub cb: qman_fq_cb,
//
// These are internal to the driver, don't touch. In particular, they
// may change, be removed, or extended (so you shouldn't rely on
// sizeof(qman_fq) being a constant).
//
    pub idx: u32 fqid,,
    pub flags: c_ulong,
    pub state: qman_fq_state,
    pub cgr_groupid: c_int,
}

//
// This callback type is used when handling congestion group entry/exit.
// 'congested' is non-zero on congestion-entry, and zero on congestion-exit.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qman_cgr {
// Set these prior to qman_create_cgr()
    pub etc.*/: *mut *mut u32 cgrid; / 0..255, but u32 to allow specials like -1, 256,,
    pub cb: qman_cb_cgr,
// These are private to the driver
    pub /: *mut *mut u16 chan; / portal channel this object is created on,
    pub node: list_head,
}

// Flags to qman_create_fq()
pub const QMAN_FQ_FLAG_NO_ENQUEUE: c_uint = 0x00000001 /* can't enqueue */;
pub const QMAN_FQ_FLAG_NO_MODIFY: c_uint = 0x00000002 /* can only enqueue */;
pub const QMAN_FQ_FLAG_TO_DCPORTAL: c_uint = 0x00000004 /* consumed by CAAM/PME/Fman */;
pub const QMAN_FQ_FLAG_DYNAMIC_FQID: c_uint = 0x00000020 /* (de)allocate fqid */;
// Flags to qman_init_fq()
pub const QMAN_INITFQ_FLAG_SCHED: c_uint = 0x00000001 /* schedule rather than park */;
pub const QMAN_INITFQ_FLAG_LOCAL: c_uint = 0x00000004 /* set dest portal */;
//
// For qman_volatile_dequeue(); Choose one PRECEDENCE. EXACT is optional. Use
// NUMFRAMES(n) (6-bit) or NUMFRAMES_TILLEMPTY to fill in the frame-count. Use
// FQID(n) to fill in the frame queue ID.
//
pub const QM_VDQCR_PRECEDENCE_VDQCR: c_uint = 0x0;
pub const QM_VDQCR_PRECEDENCE_SDQCR: c_uint = 0x80000000;
pub const QM_VDQCR_EXACT: c_uint = 0x40000000;
pub const QM_VDQCR_NUMFRAMES_MASK: c_uint = 0x3f000000;

pub const QMAN_VOLATILE_FLAG_WAIT: c_uint = 0x00000001 /* wait if VDQCR is in use */;
pub const QMAN_VOLATILE_FLAG_WAIT_INT: c_uint = 0x00000002 /* if wait, interruptible? */;
pub const QMAN_VOLATILE_FLAG_FINISH: c_uint = 0x00000004 /* wait till VDQCR completes */;
// "Query FQ Non-Programmable Fields"
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qm_mcr_queryfq_np {
    pub verb: u8,
    pub result: u8,
    pub __reserved1: u8,
    pub /: *mut *mut *mut *mut *mut u8 state; / QM_MCR_NP_STATE_,
    pub /: *mut *mut u32 fqd_link; / 24-bit, _res2[24-31],
    pub /: *mut *mut u16 odp_seq; / 14-bit, _res3[14-15],
    pub /: *mut *mut u16 orp_nesn; / 14-bit, _res4[14-15],
    pub /: *mut *mut u16 orp_ea_hseq; / 15-bit, _res5[15],
    pub /: *mut *mut u16 orp_ea_tseq; / 15-bit, _res6[15],
    pub /: *mut *mut u32 orp_ea_hptr; / 24-bit, _res7[24-31],
    pub /: *mut *mut u32 orp_ea_tptr; / 24-bit, _res8[24-31],
    pub /: *mut *mut u32 pfdr_hptr; / 24-bit, _res9[24-31],
    pub /: *mut *mut u32 pfdr_tptr; / 24-bit, _res10[24-31],
    pub __reserved2: [u8; 5],
    pub /: *mut *mut u8 is; / 1-bit, _res12[1-7],
    pub ics_surp: u16,
    pub byte_cnt: u32,
    pub /: *mut *mut u32 frm_cnt; / 24-bit, _res13[24-31],
    pub __reserved3: u32,
    pub /: *mut *mut *mut *mut *mut u16 ra1_sfdr; / QM_MCR_NP_RA1_,
    pub /: *mut *mut *mut *mut *mut u16 ra2_sfdr; / QM_MCR_NP_RA2_,
    pub __reserved4: u16,
    pub /: *mut *mut *mut *mut *mut u16 od1_sfdr; / QM_MCR_NP_OD1_,
    pub /: *mut *mut *mut *mut *mut u16 od2_sfdr; / QM_MCR_NP_OD2_,
    pub /: *mut *mut *mut *mut *mut u16 od3_sfdr; / QM_MCR_NP_OD3_,
    pub __packed: },
pub const QM_MCR_NP_STATE_FE: c_uint = 0x10;
pub const QM_MCR_NP_STATE_R: c_uint = 0x08;
pub const QM_MCR_NP_STATE_MASK: c_uint = 0x07	/* Reads FQD::STATE; */;
pub const QM_MCR_NP_STATE_OOS: c_uint = 0x00;
pub const QM_MCR_NP_STATE_RETIRED: c_uint = 0x01;
pub const QM_MCR_NP_STATE_TEN_SCHED: c_uint = 0x02;
pub const QM_MCR_NP_STATE_TRU_SCHED: c_uint = 0x03;
pub const QM_MCR_NP_STATE_PARKED: c_uint = 0x04;
pub const QM_MCR_NP_STATE_ACTIVE: c_uint = 0x05;
pub const QM_MCR_NP_PTR_MASK: c_uint = 0x07ff	/* for RA[12] & OD[123] */;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qm_mcr_queryfq_np_masks {
    qm_mcr_fqd_link_mask = BIT(24) - 1,
    qm_mcr_odp_seq_mask = BIT(14) - 1,
    qm_mcr_orp_nesn_mask = BIT(14) - 1,
    qm_mcr_orp_ea_hseq_mask = BIT(15) - 1,
    qm_mcr_orp_ea_tseq_mask = BIT(15) - 1,
    qm_mcr_orp_ea_hptr_mask = BIT(24) - 1,
    qm_mcr_orp_ea_tptr_mask = BIT(24) - 1,
    qm_mcr_pfdr_hptr_mask = BIT(24) - 1,
    qm_mcr_pfdr_tptr_mask = BIT(24) - 1,
    qm_mcr_is_mask = BIT(1) - 1,
    qm_mcr_frm_cnt_mask = BIT(24) - 1,
}

// Portal Management
//
// qman_p_irqsource_add - add processing sources to be interrupt-driven
// @bits: bitmask of QM_PIRQ_**I processing sources
//
// Adds processing sources that should be interrupt-driven (rather than
// processed via qman_poll_***() functions).
//
    pub bits): *mut *mut void qman_p_irqsource_add(struct qman_portal p, u32,
//
// qman_p_irqsource_remove - remove processing sources from being int-driven
// @bits: bitmask of QM_PIRQ_**I processing sources
//
// Removes processing sources from being interrupt-driven, so that they will
// instead be processed via qman_poll_***() functions.
//
    pub bits): *mut *mut void qman_p_irqsource_remove(struct qman_portal p, u32,
//
// qman_affine_cpus - return a mask of cpus that have affine portals
//
    pub qman_affine_cpus(void): *const cpumask_t,
//
// qman_affine_channel - return the channel ID of an portal
// @cpu: the cpu whose affine portal is the subject of the query
//
// If @cpu is -1, the affine portal for the current CPU will be used. It is a
// bug to call this function for any value of @cpu (other than -1) that is not a
// member of the mask returned from qman_affine_cpus().
//
    pub cpu): u16 qman_affine_channel(int,
//
// qman_get_affine_portal - return the portal pointer affine to cpu
// @cpu: the cpu whose affine portal is the subject of the query
//
    pub cpu): *mut *mut qman_portal qman_get_affine_portal(int,
//
// qman_start_using_portal - register a device link for the portal user
// @p: the portal that will be in use
// @dev: the device that will use the portal
//
// Makes sure that the devices that use the portal are unbound when the
// portal is unbound
//
    pub dev): *mut *mut int qman_start_using_portal(struct qman_portal p, struct device,
//
// qman_p_poll_dqrr - process DQRR (fast-path) entries
// @limit: the maximum number of DQRR entries to process
//
// Use of this function requires that DQRR processing not be interrupt-driven.
// The return value represents the number of DQRR entries processed.
//
    pub limit): *mut *mut int qman_p_poll_dqrr(struct qman_portal p, unsigned int,
//
// qman_p_static_dequeue_add - Add pool channels to the portal SDQCR
// @pools: bit-mask of pool channels, using QM_SDQCR_CHANNELS_POOL(n)
//
// Adds a set of pool channels to the portal's static dequeue command register
// (SDQCR). The requested pools are limited to those the portal has dequeue
// access to.
//
    pub pools): *mut *mut void qman_p_static_dequeue_add(struct qman_portal p, u32,
// FQ management
//
// qman_create_fq - Allocates a FQ
// @fqid: the index of the FQD to encapsulate, must be "Out of Service"
// @flags: bit-mask of QMAN_FQ_FLAG_*** options
// @fq: memory for storing the 'fq', with callbacks filled in
//
// Creates a frame queue object for the given @fqid, unless the
// QMAN_FQ_FLAG_DYNAMIC_FQID flag is set in @flags, in which case a FQID is
// dynamically allocated (or the function fails if none are available). Once
// created, the caller should not touch the memory at 'fq' except as extended to
// adjacent memory for user-defined fields (see the definition of "struct
// qman_fq" for more info). NO_MODIFY is only intended for enqueuing to
// pre-existing frame-queues that aren't to be otherwise interfered with, it
// prevents all other modifications to the frame queue. The TO_DCPORTAL flag
// causes the driver to honour any context_b modifications requested in the
// qm_init_fq() API, as this indicates the frame queue will be consumed by a
// direct-connect portal (PME, CAAM, or Fman). When frame queues are consumed by
// software portals, the context_b field is controlled by the driver and can't
// be modified by the caller.
//
    pub fq): *mut int qman_create_fq(u32 fqid, u32 flags, struct qman_fq,
//
// qman_destroy_fq - Deallocates a FQ
// @fq: the frame queue object to release
//
// The memory for this frame queue object ('fq' provided in qman_create_fq()) is
// not deallocated but the caller regains ownership, to do with as desired. The
// FQ must be in the 'out-of-service' or in the 'parked' state.
//
    pub fq): *mut void qman_destroy_fq(struct qman_fq,
//
// qman_fq_fqid - Queries the frame queue ID of a FQ object
// @fq: the frame queue object to query
//
    pub fq): *mut u32 qman_fq_fqid(struct qman_fq,
//
// qman_init_fq - Initialises FQ fields, leaves the FQ "parked" or "scheduled"
// @fq: the frame queue object to modify, must be 'parked' or new.
// @flags: bit-mask of QMAN_INITFQ_FLAG_*** options
// @opts: the FQ-modification settings, as defined in the low-level API
//
// The @opts parameter comes from the low-level portal API. Select
// QMAN_INITFQ_FLAG_SCHED in @flags to cause the frame queue to be scheduled
// rather than parked. NB, @opts can be NULL.
//
// Note that some fields and options within @opts may be ignored or overwritten
// by the driver;
// 1. the 'count' and 'fqid' fields are always ignored (this operation only
// affects one frame queue: @fq).
// 2. the QM_INITFQ_WE_CONTEXTB option of the 'we_mask' field and the associated
// 'fqd' structure's 'context_b' field are sometimes overwritten;
// - if @fq was not created with QMAN_FQ_FLAG_TO_DCPORTAL, then context_b is
// initialised to a value used by the driver for demux.
// - if context_b is initialised for demux, so is context_a in case stashing
// is requested (see item 4).
// (So caller control of context_b is only possible for TO_DCPORTAL frame queue
// objects.)
// 3. if @flags contains QMAN_INITFQ_FLAG_LOCAL, the 'fqd' structure's
// 'dest::channel' field will be overwritten to match the portal used to issue
// the command. If the WE_DESTWQ write-enable bit had already been set by the
// caller, the channel workqueue will be left as-is, otherwise the write-enable
// bit is set and the workqueue is set to a default of 4. If the "LOCAL" flag
// isn't set, the destination channel/workqueue fields and the write-enable bit
// are left as-is.
// 4. if the driver overwrites context_a/b for demux, then if
// QM_INITFQ_WE_CONTEXTA is set, the driver will only overwrite
// context_a.address fields and will leave the stashing fields provided by the
// user alone, otherwise it will zero out the context_a.stashing fields.
//
    pub opts): *mut *mut int qman_init_fq(struct qman_fq fq, u32 flags, struct qm_mcc_initfq,
//
// qman_schedule_fq - Schedules a FQ
// @fq: the frame queue object to schedule, must be 'parked'
//
// Schedules the frame queue, which must be Parked, which takes it to
// Tentatively-Scheduled or Truly-Scheduled depending on its fill-level.
//
    pub fq): *mut int qman_schedule_fq(struct qman_fq,
//
// qman_retire_fq - Retires a FQ
// @fq: the frame queue object to retire
// @flags: FQ flags (QMAN_FQ_STATE*) if retirement completes immediately
//
// Retires the frame queue. This returns zero if it succeeds immediately, +1 if
// the retirement was started asynchronously, otherwise it returns negative for
// failure. When this function returns zero, @flags is set to indicate whether
// the retired FQ is empty and/or whether it has any ORL fragments (to show up
// as ERNs). Otherwise the corresponding flags will be known when a subsequent
// FQRN message shows up on the portal's message ring.
//
// NB, if the retirement is asynchronous (the FQ was in the Truly Scheduled or
// Active state), the completion will be via the message ring as a FQRN - but
// the corresponding callback may occur before this function returns!! Ie. the
// caller should be prepared to accept the callback as the function is called,
// not only once it has returned.
//
    pub flags): *mut *mut int qman_retire_fq(struct qman_fq fq, u32,
//
// qman_oos_fq - Puts a FQ "out of service"
// @fq: the frame queue object to be put out-of-service, must be 'retired'
//
// The frame queue must be retired and empty, and if any order restoration list
// was released as ERNs at the time of retirement, they must all be consumed.
//
    pub fq): *mut int qman_oos_fq(struct qman_fq,
//
// qman_volatile_dequeue - Issue a volatile dequeue command
// @fq: the frame queue object to dequeue from
// @flags: a bit-mask of QMAN_VOLATILE_FLAG_*** options
// @vdqcr: bit mask of QM_VDQCR_*** options, as per qm_dqrr_vdqcr_set()
//
// Attempts to lock access to the portal's VDQCR volatile dequeue functionality.
// The function will block and sleep if QMAN_VOLATILE_FLAG_WAIT is specified and
// the VDQCR is already in use, otherwise returns non-zero for failure. If
// QMAN_VOLATILE_FLAG_FINISH is specified, the function will only return once
// the VDQCR command has finished executing (ie. once the callback for the last
// DQRR entry resulting from the VDQCR command has been called). If not using
// the FINISH flag, completion can be determined either by detecting the
// presence of the QM_DQRR_STAT_UNSCHEDULED and QM_DQRR_STAT_DQCR_EXPIRED bits
// in the "stat" parameter passed to the FQ's dequeue callback, or by waiting
// for the QMAN_FQ_STATE_VDQCR bit to disappear.
//
    pub vdqcr): *mut *mut int qman_volatile_dequeue(struct qman_fq fq, u32 flags, u32,
//
// qman_enqueue - Enqueue a frame to a frame queue
// @fq: the frame queue object to enqueue to
// @fd: a descriptor of the frame to be enqueued
//
// Fills an entry in the EQCR of portal @qm to enqueue the frame described by
// @fd. The descriptor details are copied from @fd to the EQCR entry, the 'pid'
// field is ignored. The return value is non-zero on error, such as ring full.
//
    pub fd): *const *const int qman_enqueue(struct qman_fq fq, struct qm_fd,
//
// qman_alloc_fqid_range - Allocate a contiguous range of FQIDs
// @result: is set by the API to the base FQID of the allocated range
// @count: the number of FQIDs required
//
// Returns 0 on success, or a negative error code.
//
    pub count): *mut *mut int qman_alloc_fqid_range(u32 result, u32,

//
// qman_release_fqid - Release the specified frame queue ID
// @fqid: the FQID to be released back to the resource pool
//
// This function can also be used to seed the allocator with
// FQID ranges that it can subsequently allocate from.
// Returns 0 on success, or a negative error code.
//
    pub fqid): int qman_release_fqid(u32,
//
// qman_query_fq_np - Queries non-programmable FQD fields
// @fq: the frame queue object to be queried
// @np: storage for the queried FQD fields
//
    pub np): *mut *mut int qman_query_fq_np(struct qman_fq fq, struct qm_mcr_queryfq_np,
// Pool-channel management
//
// qman_alloc_pool_range - Allocate a contiguous range of pool-channel IDs
// @result: is set by the API to the base pool-channel ID of the allocated range
// @count: the number of pool-channel IDs required
//
// Returns 0 on success, or a negative error code.
//
    pub count): *mut *mut int qman_alloc_pool_range(u32 result, u32,

//
// qman_release_pool - Release the specified pool-channel ID
// @id: the pool-chan ID to be released back to the resource pool
//
// This function can also be used to seed the allocator with
// pool-channel ID ranges that it can subsequently allocate from.
// Returns 0 on success, or a negative error code.
//
    pub id): int qman_release_pool(u32,
// CGR management
//
// qman_create_cgr - Register a congestion group object
// @cgr: the 'cgr' object, with fields filled in
// @flags: QMAN_CGR_FLAG_* values
// @opts: optional state of CGR settings
//
// Registers this object to receiving congestion entry/exit callbacks on the
// portal affine to the cpu portal on which this API is executed. If opts is
// NULL then only the callback (cgr->cb) function is registered. If @flags
// contains QMAN_CGR_FLAG_USE_INIT, then an init hw command (which will reset
// any unspecified parameters) will be used rather than a modify hw hardware
// (which only modifies the specified parameters).
//
    pub opts): *mut qm_mcc_initcgr,
//
// qman_delete_cgr - Deregisters a congestion group object
// @cgr: the 'cgr' object to deregister
//
// "Unplugs" this CGR object from the portal affine to the cpu on which this API
// is executed. This must be excuted on the same affine portal on which it was
// created.
//
    pub cgr): *mut int qman_delete_cgr(struct qman_cgr,
//
// qman_delete_cgr_safe - Deregisters a congestion group object from any CPU
// @cgr: the 'cgr' object to deregister
//
// This will select the proper CPU and run there qman_delete_cgr().
//
    pub cgr): *mut void qman_delete_cgr_safe(struct qman_cgr,
//
// qman_update_cgr_safe - Modifies a congestion group object from any CPU
// @cgr: the 'cgr' object to modify
// @opts: state of the CGR settings
//
// This will select the proper CPU and modify the CGR settings.
//
    pub opts): *mut *mut int qman_update_cgr_safe(struct qman_cgr cgr, struct qm_mcc_initcgr,
//
// qman_query_cgr_congested - Queries CGR's congestion status
// @cgr: the 'cgr' object to query
// @result: returns 'cgr's congestion status, 1 (true) if congested
//
    pub result): *mut *mut int qman_query_cgr_congested(struct qman_cgr cgr, bool,
//
// qman_alloc_cgrid_range - Allocate a contiguous range of CGR IDs
// @result: is set by the API to the base CGR ID of the allocated range
// @count: the number of CGR IDs required
//
// Returns 0 on success, or a negative error code.
//
    pub count): *mut *mut int qman_alloc_cgrid_range(u32 result, u32,

//
// qman_release_cgrid - Release the specified CGR ID
// @id: the CGR ID to be released back to the resource pool
//
// This function can also be used to seed the allocator with
// CGR ID ranges that it can subsequently allocate from.
// Returns 0 on success, or a negative error code.
//
    pub id): int qman_release_cgrid(u32,
//
// qman_is_probed - Check if qman is probed
//
// Returns 1 if the qman driver successfully probed, -1 if the qman driver
// failed to probe or 0 if the qman driver did not probed yet.
//
    pub qman_is_probed(void): c_int,
//
// qman_portals_probed - Check if all cpu bound qman portals are probed
//
// Returns 1 if all the required cpu bound qman portals successfully probed,
// -1 if probe errors appeared or 0 if the qman portals did not yet finished
// probing.
//
    pub qman_portals_probed(void): c_int,
//
// qman_dqrr_get_ithresh - Get coalesce interrupt threshold
// @portal: portal to get the value for
// @ithresh: threshold pointer
//
    pub ithresh): *mut *mut void qman_dqrr_get_ithresh(struct qman_portal portal, u8,
//
// qman_dqrr_set_ithresh - Set coalesce interrupt threshold
// @portal: portal to set the new value on
// @ithresh: new threshold value
//
// Returns 0 on success, or a negative error code.
//
    pub ithresh): *mut *mut int qman_dqrr_set_ithresh(struct qman_portal portal, u8,
//
// qman_dqrr_get_iperiod - Get coalesce interrupt period
// @portal: portal to get the value for
// @iperiod: period pointer
//
    pub iperiod): *mut *mut void qman_portal_get_iperiod(struct qman_portal portal, u32,
//
// qman_dqrr_set_iperiod - Set coalesce interrupt period
// @portal: portal to set the new value on
// @ithresh: new period value
//
// Returns 0 on success, or a negative error code.
//
    pub iperiod): *mut *mut int qman_portal_set_iperiod(struct qman_portal portal, u32,
