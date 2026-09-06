//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/host/ehci.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// Copyright (c) 2001-2002 by David Brownell
//
// definitions used for the EHCI driver
//
// __hc32 and __hc16 are "Host Controller" types, they may be equivalent to
// __leXX (normally) or __beXX (given EHCI_BIG_ENDIAN_DESC), depending on
// the host controller implementation.
//
// To facilitate the strongest possible byte-order checking from "sparse"
// and so on, we use __leXX unless that's not practical.
//

pub type __hc32 = __u32 ;
pub type __hc16 = __u16 ;

// statistics can be kept for tuning/monitoring

// Macro flag: #define EHCI_STATS

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ehci_stats {
// irq usage
    pub normal: c_ulong,
    pub error: c_ulong,
    pub iaa: c_ulong,
    pub lost_iaa: c_ulong,
// termination of urbs from core
    pub complete: c_ulong,
    pub unlink: c_ulong,
}

//
// Scheduling and budgeting information for periodic transfers, for both
// high-speed devices and full/low-speed devices lying behind a TT.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ehci_per_sched {
    pub /: *mut *mut *mut usb_device udev; / access to the TT,
    pub ep: *mut usb_host_endpoint,
    pub /: *mut *mut list_head ps_list; / node on ehci_tt's ps_list,
    pub /: *mut *mut u16 tt_usecs; / time on the FS/LS bus,
    pub /: *mut *mut u16 cs_mask; / C-mask and S-mask bytes,
    pub /: *mut *mut u16 period; / actual period in frames,
    pub /: *mut *mut u16 phase; / actual phase, frame part,
    pub bandwidth: *mut *mut u8 bw_phase; / same, for,
    pub /: *mut *mut u8 phase_uf; / uframe part of the phase,
    pub /: *mut *mut u8 usecs, c_usecs; / times on the HS bus,
    pub for: *mut *mut u8 bw_uperiod; / period in microframes,,
    pub /: *mut *mut u8 bw_period; / same, in frames,
}

// ehci_hcd->lock guards shared data against other CPUs:
// ehci_hcd:	async, unlink, periodic (and shadow), ...
// usb_host_endpoint: hcpriv
// ehci_qh:	qh_next, qtd_list
// ehci_qtd:	qtd_list
//
// Also, hold this lock when talking to HC registers or
// when updating hw_* fields in shared qh/qtd/... structures.
//

//
// ehci_rh_state values of EHCI_RH_RUNNING or above mean that the
// controller may be doing DMA.  Lower values mean there's no DMA.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ehci_rh_state {
    EHCI_RH_HALTED,
    EHCI_RH_SUSPENDED,
    EHCI_RH_RUNNING,
    EHCI_RH_STOPPING
}

//
// Timer events, ordered by increasing delay length.
// Always update event_delays_ns[] and event_handlers[] (defined in
// ehci-timer.c) in parallel with this list.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ehci_hrtimer_event {
    EHCI_HRTIMER_POLL_ASS,		/* Poll for async schedule off */
    EHCI_HRTIMER_POLL_PSS,		/* Poll for periodic schedule off */
    EHCI_HRTIMER_POLL_DEAD,		/* Wait for dead controller to stop */
    EHCI_HRTIMER_UNLINK_INTR,	/* Wait for interrupt QH unlink */
    EHCI_HRTIMER_FREE_ITDS,		/* Wait for unused iTDs and siTDs */
    EHCI_HRTIMER_ACTIVE_UNLINK,	/* Wait while unlinking an active QH */
    EHCI_HRTIMER_START_UNLINK_INTR, /* Unlink empty interrupt QHs */
    EHCI_HRTIMER_ASYNC_UNLINKS,	/* Unlink empty async QHs */
    EHCI_HRTIMER_IAA_WATCHDOG,	/* Handle lost IAA interrupts */
    EHCI_HRTIMER_DISABLE_PERIODIC,	/* Wait to disable periodic sched */
    EHCI_HRTIMER_DISABLE_ASYNC,	/* Wait to disable async sched */
    EHCI_HRTIMER_IO_WATCHDOG,	/* Check for missing IRQs */
    EHCI_HRTIMER_NUM_EVENTS		/* Must come last */
}

pub const EHCI_HRTIMER_NO_EVENT: c_int = 99;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ehci_hcd {
// timing support
    pub next_hrtimer_event: ehci_hrtimer_event,
    pub enabled_hrtimer_events: unsigned,
    pub hr_timeouts: [ktime_t; EHCI_HRTIMER_NUM_EVENTS],
    pub hrtimer: hrtimer,
    pub PSS_poll_count: c_int,
    pub ASS_poll_count: c_int,
    pub died_poll_count: c_int,
// glue to PCI and HCD framework
    pub caps: *mut ehci_caps __iomem,
    pub regs: *mut ehci_regs __iomem,
    pub debug: *mut ehci_dbg_port __iomem,
    pub /: *mut *mut __u32 hcs_params; / cached register copy,
    pub lock: spinlock_t,
    pub rh_state: ehci_rh_state,
// general schedule support
    pub scanning:1: bool,
    pub need_rescan:1: bool,
    pub intr_unlinking:1: bool,
    pub iaa_in_progress:1: bool,
    pub async_unlinking:1: bool,
    pub shutdown:1: bool,
    pub qh_scan_next: *mut ehci_qh,
// async schedule support
    pub async: *mut ehci_qh,
    pub /: *mut *mut *mut ehci_qh dummy; / For AMD quirk use,
    pub async_unlink: list_head,
    pub async_idle: list_head,
    pub async_unlink_cycle: unsigned,
    pub /: *mut *mut unsigned async_count; / async activity count,
    pub /: *mut *mut __hc32 old_current; / Test for QH becoming,
    pub /: *mut *mut __hc32 old_token; / inactive during unlink,
// periodic schedule support

    pub periodic_size: unsigned,
    pub /: *mut *mut *mut __hc32 periodic; / hw periodic table,
    pub periodic_dma: dma_addr_t,
    pub intr_qh_list: list_head,
    pub /: *mut *mut unsigned i_thresh; / uframes HC might cache,
    pub /: *mut *mut *mut ehci_shadow pshadow; / mirror hw periodic table,
    pub intr_unlink_wait: list_head,
    pub intr_unlink: list_head,
    pub intr_unlink_wait_cycle: unsigned,
    pub intr_unlink_cycle: unsigned,
    pub /: *mut *mut unsigned now_frame; / frame from HC hardware,
    pub /: *mut *mut unsigned last_iso_frame; / last frame scanned for iso,
    pub /: *mut *mut unsigned intr_count; / intr activity count,
    pub /: *mut *mut unsigned isoc_count; / isoc activity count,
    pub /: *mut *mut unsigned periodic_count; / periodic activity count,
    pub /: *mut *mut unsigned uframe_periodic_max; / max periodic time per uframe,
// list of itds & sitds completed while now_frame was still active
    pub cached_itd_list: list_head,
    pub last_itd_to_free: *mut ehci_itd,
    pub cached_sitd_list: list_head,
    pub last_sitd_to_free: *mut ehci_sitd,
// per root hub port
    pub reset_done: [c_ulong; EHCI_MAX_ROOT_PORTS],
// bit vectors (one bit per port)
    pub were: *mut *mut unsigned long bus_suspended; / which ports,
    pub are: *mut *mut unsigned long companion_ports; / which ports,
    pub are: *mut *mut unsigned long owned_ports; / which ports,
    pub have: *mut *mut unsigned long port_c_suspend; / which ports,
    pub are: *mut *mut unsigned long suspended_ports; / which ports,
    pub have: *mut *mut unsigned long resuming_ports; / which ports,
// per-HC memory pools (could be per-bus, but ...)
    pub /: *mut *mut *mut dma_pool qh_pool; / qh per active urb,
    pub /: *mut *mut *mut dma_pool qtd_pool; / one or more per qh,
    pub /: *mut *mut *mut dma_pool itd_pool; / itd per iso urb,
    pub /: *mut *mut *mut dma_pool sitd_pool; / sitd per split iso urb,
    pub random_frame: unsigned,
    pub next_statechange: c_ulong,
    pub last_periodic_enable: ktime_t,
    pub command: u32,
// SILICON QUIRKS
    pub no_selective_suspend:1: unsigned,
    pub /: *mut *mut unsigned has_fsl_port_bug:1; / FreeScale,
    pub /: *mut *mut unsigned has_fsl_hs_errata:1; / Freescale HS quirk,
    pub /: *mut *mut unsigned has_fsl_susp_errata:1; / NXP SUSP quirk,
    pub /: *mut *mut unsigned has_ci_pec_bug:1; / ChipIdea PEC bug,
    pub big_endian_mmio:1: unsigned,
    pub big_endian_desc:1: unsigned,
    pub big_endian_capbase:1: unsigned,
    pub has_amcc_usb23:1: unsigned,
    pub need_io_watchdog:1: unsigned,
    pub amd_pll_fix:1: unsigned,
    pub quirk*/: *mut *mut unsigned use_dummy_qh:1; / AMD Frame List table,
    pub /: *mut *mut unsigned has_synopsys_hc_bug:1; / Synopsys HC,
    pub /: *mut *mut unsigned frame_index_bug:1; / MosChip (AKA NetMos),
    pub /: *mut *mut unsigned need_oc_pp_cycle:1; / MPC834X port power,
    pub /: *mut *mut unsigned imx28_write_fix:1; / For Freescale i.MX28,
    pub spurious_oc:1: unsigned,
    pub is_aspeed:1: unsigned,
    pub zx_wakeup_clear_needed:1: unsigned,
// required for usb32 quirk

pub const OHCI_HCCTRL_OFFSET: c_uint = 0x4;
pub const OHCI_HCCTRL_LEN: c_uint = 0x4;
    pub ohci_hcctrl_reg: *mut __hc32,
    pub has_hostpc:1: unsigned,
    pub has_tdi_phy_lpm:1: unsigned,
    pub /: *mut *mut unsigned has_ppcd:1; / support per-port change bits,
    pub /: *mut *mut u8 sbrn; / packed release number,
// irq statistics

    pub stats: ehci_stats,

// debug files

    pub debug_dir: *mut dentry,

// bandwidth usage
pub const EHCI_BANDWIDTH_SIZE: c_int = 64;
    pub bandwidth: [u8; EHCI_BANDWIDTH_SIZE],
// us allocated per uframe
    pub tt_budget: [u8; EHCI_BANDWIDTH_SIZE],
// us budgeted per uframe
    pub tt_list: list_head,
// platform-specific data -- must come last
    pub __aligned(sizeof(s64)): unsigned long priv[],
}

// convert between an HCD pointer and the corresponding EHCI_HCD
extern "C" {
    pub fn container_of(ehci: *mut *mut (void ), usb_hcd: struct, _arg: hcd_priv) -> return;
}
// -------------------------------------------------------------------------

// -------------------------------------------------------------------------

//
// EHCI Specification 0.95 Section 3.5
// QTD: describe data transfer components (buffer, direction, ...)
// See Fig 3-6 "Queue Element Transfer Descriptor Block Diagram".
//
// These are associated only with "QH" (Queue Head) structures,
// used with control, bulk, and interrupt transfers.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ehci_qtd {
// first part defined by EHCI spec
    pub /: *mut *mut __hc32 hw_next; / see EHCI 3.5.1,
    pub /: *mut *mut __hc32 hw_alt_next; / see EHCI 3.5.2,
    pub /: *mut *mut __hc32 hw_token; / see EHCI 3.5.3,

    pub /: *mut *mut __hc32 hw_buf[5]; / see EHCI 3.5.4,
    pub /: *mut *mut __hc32 hw_buf_hi[5]; / Appendix B,
// the rest is HCD-private
    pub /: *mut *mut dma_addr_t qtd_dma; / qtd address,
    pub /: *mut *mut list_head qtd_list; / sw qtd list,
    pub /: *mut *mut *mut urb urb; / qtd's urb,
    pub /: *mut *mut size_t length; / length of buffer,
    pub __aligned(32): },
// PID Codes that are used here, from EHCI specification, Table 3-16.
pub const PID_CODE_OUT: c_int = 0;
pub const PID_CODE_IN: c_int = 1;
pub const PID_CODE_SETUP: c_int = 2;
// mask NakCnt+T in qh->hw_alt_next

// -------------------------------------------------------------------------
// type tag from {qh,itd,sitd,fstn}->hw_next

//
// Now the following defines are not converted using the
// cpu_to_le32() macro anymore, since we have to support
// "dynamic" switching between be and le support, so that the driver
// can be used on one system with SoC EHCI controller using big-endian
// descriptors as well as a normal little-endian PCI EHCI controller.
//
// values for that type tag

// next async queue entry, or pointer to interrupt/periodic QH

// for periodic/async schedules and qtd lists, mark end of list

//
// Entries in periodic shadow table are pointers to one of four kinds
// of data structure.  That's dictated by the hardware; a type tag is
// encoded in the low bits of the hardware's periodic schedule.  Use
// Q_NEXT_TYPE to get the tag.
//
// For entries in the async schedule, the type tag always says "qh".
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union ehci_shadow {
    pub /: *mut *mut *mut ehci_qh qh; / Q_TYPE_QH,
    pub /: *mut *mut *mut ehci_itd itd; / Q_TYPE_ITD,
    pub /: *mut *mut *mut ehci_sitd sitd; / Q_TYPE_SITD,
    pub /: *mut *mut *mut ehci_fstn fstn; / Q_TYPE_FSTN,
    pub /: *mut *mut *mut __hc32 hw_next; / (all types),
    pub ptr: *mut c_void,
}

// -------------------------------------------------------------------------
//
// EHCI Specification 0.95 Section 3.6
// QH: describes control/bulk/interrupt endpoints
// See Fig 3-7 "Queue Head Structure Layout".
//
// These appear in both the async and (for interrupt) periodic schedules.
//
// first part defined by EHCI spec
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ehci_qh_hw {
    pub /: *mut *mut __hc32 hw_next; / see EHCI 3.6.1,
    pub /: *mut *mut __hc32 hw_info1; / see EHCI 3.6.2,

    pub /: *mut *mut __hc32 hw_info2; / see EHCI 3.6.2,
pub const QH_SMASK: c_uint = 0x000000ff;
pub const QH_CMASK: c_uint = 0x0000ff00;
pub const QH_HUBADDR: c_uint = 0x007f0000;
pub const QH_HUBPORT: c_uint = 0x3f800000;
pub const QH_MULT: c_uint = 0xc0000000;
    pub /: *mut *mut __hc32 hw_current; / qtd list - see EHCI 3.6.4,
// qtd overlay (hardware parts of a struct ehci_qtd)
    pub hw_qtd_next: __hc32,
    pub hw_alt_next: __hc32,
    pub hw_token: __hc32,
    pub hw_buf: [__hc32; 5],
    pub hw_buf_hi: [__hc32; 5],
    pub __aligned(32): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ehci_qh {
    pub /: *mut *mut *mut ehci_qh_hw hw; / Must come first,
// the rest is HCD-private
    pub /: *mut *mut dma_addr_t qh_dma; / address of qh,
    pub /: *mut *mut ehci_shadow qh_next; / ptr to qh; or periodic,
    pub /: *mut *mut list_head qtd_list; / sw qtd list,
    pub /: *mut *mut list_head intr_node; / list of intr QHs,
    pub dummy: *mut ehci_qtd,
    pub unlink_node: list_head,
    pub /: *mut *mut ehci_per_sched ps; / scheduling info,
    pub unlink_cycle: unsigned,
    pub qh_state: u8,

    pub /: *mut *mut u8 xacterrs; / XactErr retry counter,

    pub unlink_reason: u8,
pub const QH_UNLINK_HALTED: c_uint = 0x01		/* Halt flag is set */;
pub const QH_UNLINK_SHORT_READ: c_uint = 0x02		/* Recover from a short read */;
pub const QH_UNLINK_DUMMY_OVERLAY: c_uint = 0x04		/* QH overlayed the dummy TD */;
pub const QH_UNLINK_SHUTDOWN: c_uint = 0x08		/* The HC isn't running */;
pub const QH_UNLINK_QUEUE_EMPTY: c_uint = 0x10		/* Reached end of the queue */;
pub const QH_UNLINK_REQUESTED: c_uint = 0x20		/* Disable, reset, or dequeue */;
    pub /: *mut *mut u8 gap_uf; / uframes split/csplit gap,
    pub /: *mut *mut unsigned is_out:1; / bulk or intr OUT,
    pub /: *mut *mut unsigned clearing_tt:1; / Clear-TT-Buf in progress,
    pub dequeue_during_giveback:1: unsigned,
    pub should_be_inactive:1: unsigned,
}

// -------------------------------------------------------------------------
// description of one iso transaction (up to 3 KB data if highspeed)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ehci_iso_packet {
// These will be copied to iTD when scheduling
    pub /: *mut *mut u64 bufp; / itd->hw_bufp{,_hi}[pg] |=,
    pub /: *mut *mut __hc32 transaction; / itd->hw_transaction[i] |=,
    pub /: *mut *mut u8 cross; / buf crosses pages,
// for full speed OUT splits
    pub buf1: u32,
}

// temporary schedule data for packets from iso urbs (both speeds)
// each packet is one logical usb transaction to the device (not TT),
// beginning at stream->next_uframe
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ehci_iso_sched {
    pub td_list: list_head,
    pub span: unsigned,
    pub first_packet: unsigned,
    pub packet: [ehci_iso_packet; ],
}

//
// ehci_iso_stream - groups all (s)itds for this endpoint.
// acts like a qh would, if EHCI had them for ISO.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ehci_iso_stream {
// first field matches ehci_qh, but is NULL
    pub hw: *mut ehci_qh_hw,
    pub bEndpointAddress: u8,
    pub highspeed: u8,
    pub /: *mut *mut list_head td_list; / queued itds/sitds,
    pub /: *mut *mut list_head free_list; / list of unused itds/sitds,
// output of (re)scheduling
    pub /: *mut *mut ehci_per_sched ps; / scheduling info,
    pub next_uframe: unsigned,
    pub splits: __hc32,
// the rest is derived from the endpoint descriptor,
// including the extra info for hw_bufp[0..2]
//
    pub /: *mut *mut u16 uperiod; / period in uframes,
    pub maxp: u16,
    pub bandwidth: unsigned,
// This is used to initialize iTD's hw_bufp fields
    pub buf0: __hc32,
    pub buf1: __hc32,
    pub buf2: __hc32,
// this is used to initialize sITD's tt info
    pub address: __hc32,
}

// -------------------------------------------------------------------------
//
// EHCI Specification 0.95 Section 3.3
// Fig 3-4 "Isochronous Transaction Descriptor (iTD)"
//
// Schedule records for high speed iso xfers
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ehci_itd {
// first part defined by EHCI spec
    pub /: *mut *mut __hc32 hw_next; / see EHCI 3.3.1,
    pub /: *mut *mut __hc32 hw_transaction[8]; / see EHCI 3.3.2,

    pub /: *mut *mut __hc32 hw_bufp[7]; / see EHCI 3.3.3,
    pub /: *mut *mut __hc32 hw_bufp_hi[7]; / Appendix B,
// the rest is HCD-private
    pub /: *mut *mut dma_addr_t itd_dma; / for this itd,
    pub /: *mut *mut ehci_shadow itd_next; / ptr to periodic q entry,
    pub urb: *mut urb,
    pub /: *mut *mut *mut ehci_iso_stream stream; / endpoint's queue,
    pub /: *mut *mut list_head itd_list; / list of stream's itds,
// any/all hw_transactions here may be used by that urb
    pub /: *mut *mut unsigned frame; / where scheduled,
    pub pg: unsigned,
    pub /: *mut *mut unsigned index[8]; / in urb->iso_frame_desc,
    pub __aligned(32): },
// -------------------------------------------------------------------------
//
// EHCI Specification 0.95 Section 3.4
// siTD, aka split-transaction isochronous Transfer Descriptor
// ... describe full speed iso xfers through TT in hubs
// see Figure 3-5 "Split-transaction Isochronous Transaction Descriptor (siTD)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ehci_sitd {
// first part defined by EHCI spec
    pub hw_next: __hc32,
// uses bit field macros above - see EHCI 0.95 Table 3-8
    pub /: *mut *mut __hc32 hw_fullspeed_ep; / EHCI table 3-9,
    pub /: *mut *mut __hc32 hw_uframe; / EHCI table 3-10,
    pub /: *mut *mut __hc32 hw_results; / EHCI table 3-11,

    pub /: *mut *mut __hc32 hw_buf[2]; / EHCI table 3-12,
    pub /: *mut *mut __hc32 hw_backpointer; / EHCI table 3-13,
    pub /: *mut *mut __hc32 hw_buf_hi[2]; / Appendix B,
// the rest is HCD-private
    pub sitd_dma: dma_addr_t,
    pub /: *mut *mut ehci_shadow sitd_next; / ptr to periodic q entry,
    pub urb: *mut urb,
    pub /: *mut *mut *mut ehci_iso_stream stream; / endpoint's queue,
    pub /: *mut *mut list_head sitd_list; / list of stream's sitds,
    pub frame: unsigned,
    pub index: unsigned,
    pub __aligned(32): },
// -------------------------------------------------------------------------
//
// EHCI Specification 0.96 Section 3.7
// Periodic Frame Span Traversal Node (FSTN)
//
// Manages split interrupt transactions (using TT) that span frame boundaries
// into uframes 0/1; see 4.12.2.2.  In those uframes, a "save place" FSTN
// makes the HC jump (back) to a QH to scan for fs/ls QH completions until
// it hits a "restore" FSTN; then it returns to finish other uframe 0/1 work.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ehci_fstn {
    pub /: *mut *mut __hc32 hw_next; / any periodic q entry,
    pub /: *mut *mut __hc32 hw_prev; / qh or EHCI_LIST_END,
// the rest is HCD-private
    pub fstn_dma: dma_addr_t,
    pub /: *mut *mut ehci_shadow fstn_next; / ptr to periodic q entry,
    pub __aligned(32): },
// -------------------------------------------------------------------------
//
// USB-2.0 Specification Sections 11.14 and 11.18
// Scheduling and budgeting split transactions using TTs
//
// A hub can have a single TT for all its ports, or multiple TTs (one for each
// port).  The bandwidth and budgeting information for the full/low-speed bus
// below each TT is self-contained and independent of the other TTs or the
// high-speed bus.
//
// "Bandwidth" refers to the number of microseconds on the FS/LS bus allocated
// to an interrupt or isochronous endpoint for each frame.  "Budget" refers to
// the best-case estimate of the number of full-speed bytes allocated to an
// endpoint for each microframe within an allocated frame.
//
// Removal of an endpoint invalidates a TT's budget.  Instead of trying to
// keep an up-to-date record, we recompute the budget when it is needed.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ehci_tt {
    pub bandwidth: [u16; EHCI_BANDWIDTH_FRAMES],
    pub /: *mut *mut list_head tt_list; / List of all ehci_tt's,
    pub /: *mut *mut list_head ps_list; / Items using this TT,
    pub usb_tt: *mut usb_tt,
    pub /: *mut *mut int tt_port; / TT port number,
}

// -------------------------------------------------------------------------
// Prepare the PORTSC wakeup flags during controller suspend/resume

// -------------------------------------------------------------------------

//
// Some EHCI controllers have a Transaction Translator built into the
// root hub. This is a non-standard feature.  Each controller will need
// to add code to the following inline functions, and call them as
// needed (mostly in root hub code).
//

// Returns the speed of a device attached to a port on the root hub.

// -------------------------------------------------------------------------

// Some Freescale processors have an erratum in which the TT
// port number in the queue head was 0..N-1 instead of 1..N.
//

// Some Freescale processors have an erratum (USB A-005275) in which
// incoming packets get corrupted in HS mode
//

//
// Some Freescale/NXP processors have an erratum (USB A-005697)
// in which we need to wait for 10ms for bus to enter suspend mode
// after setting SUSP bit.
//

//
// Some Freescale/NXP processors using ChipIdea IP have a bug in which
// disabling the port (PE is cleared) does not cause PEC to be asserted
// when frame babble is detected.
//

//
// While most USB host controllers implement their registers in
// little-endian format, a minority (celleb companion chip) implement
// them in big endian format.
//
// This attempts to support either format at compile time without a
// runtime penalty, or both formats with the additional overhead
// of checking a flag bit.
//
// ehci_big_endian_capbase is a special quirk for controllers that
// implement the HC capability registers as separate registers and not
// as fields of a 32-bit register.
//

pub const ehci_big_endian_mmio(e): c_int = 0;
pub const ehci_big_endian_capbase(e): c_int = 0;

//
// Big-endian read/write functions are arch-specific.
// Other arches can be added if/when they're needed.
//

extern "C" {
    pub fn readl(_arg: regs) -> return;
}

//
// On certain ppc-44x SoC there is a HW issue, that could only worked around with
// explicit suspend/operate of OHCI. This function hereby makes sense only on that arch.
// Other common bits are dependent on has_amcc_usb23 quirk flag.
//

// -------------------------------------------------------------------------
//
// The AMCC 440EPx not only implements its EHCI registers in big-endian
// format, but also its DMA data structures (descriptors).
//
// EHCI controllers accessed through PCI work normally (little-endian
// everywhere), so we won't bother supporting a BE-only mode for now.
//

// cpu to ehci
// ehci to cpu

// cpu to ehci
extern "C" {
    pub fn cpu_to_le32(_arg: x) -> return;
}
// ehci to cpu
extern "C" {
    pub fn le32_to_cpu(_arg: x) -> return;
}
extern "C" {
    pub fn le32_to_cpup(_arg: x) -> return;
}

// -------------------------------------------------------------------------

// -------------------------------------------------------------------------
// Declarations of things exported for use by ehci platform drivers
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ehci_driver_overrides {
    pub extra_priv_size: usize,
    pub hcd): *mut *mut int (reset)(struct usb_hcd,
    pub enable): int portnum, bool,
}

extern "C" {
    pub fn ehci_setup(hcd: *mut usb_hcd) -> c_int;
}
extern "C" {
    pub fn ehci_reset(ehci: *mut ehci_hcd) -> c_int;
}
extern "C" {
    pub fn ehci_suspend(hcd: *mut usb_hcd, do_wakeup: bool) -> c_int;
}
extern "C" {
    pub fn ehci_resume(hcd: *mut usb_hcd, force_reset: bool) -> c_int;
}
