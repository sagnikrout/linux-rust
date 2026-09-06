//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/fotg210/fotg210-hcd.h
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

// definitions used for the EHCI driver
//
// __hc32 and __hc16 are "Host Controller" types, they may be equivalent to
// __leXX (normally) or __beXX (given FOTG210_BIG_ENDIAN_DESC), depending on
// the host controller implementation.
//
// To facilitate the strongest possible byte-order checking from "sparse"
// and so on, we use __leXX unless that's not practical.
//

// statistics can be kept for tuning/monitoring
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fotg210_stats {
// irq usage
    pub normal: c_ulong,
    pub error: c_ulong,
    pub iaa: c_ulong,
    pub lost_iaa: c_ulong,
// termination of urbs from core
    pub complete: c_ulong,
    pub unlink: c_ulong,
}

// fotg210_hcd->lock guards shared data against other CPUs:
// fotg210_hcd:	async, unlink, periodic (and shadow), ...
// usb_host_endpoint: hcpriv
// fotg210_qh:	qh_next, qtd_list
// fotg210_qtd:	qtd_list
//
// Also, hold this lock when talking to HC registers or
// when updating hw_* fields in shared qh/qtd/... structures.
//

//
// fotg210_rh_state values of FOTG210_RH_RUNNING or above mean that the
// controller may be doing DMA.  Lower values mean there's no DMA.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fotg210_rh_state {
    FOTG210_RH_HALTED,
    FOTG210_RH_SUSPENDED,
    FOTG210_RH_RUNNING,
    FOTG210_RH_STOPPING
}

//
// Timer events, ordered by increasing delay length.
// Always update event_delays_ns[] and event_handlers[] (defined in
// ehci-timer.c) in parallel with this list.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fotg210_hrtimer_event {
    FOTG210_HRTIMER_POLL_ASS,	/* Poll for async schedule off */
    FOTG210_HRTIMER_POLL_PSS,	/* Poll for periodic schedule off */
    FOTG210_HRTIMER_POLL_DEAD,	/* Wait for dead controller to stop */
    FOTG210_HRTIMER_UNLINK_INTR,	/* Wait for interrupt QH unlink */
    FOTG210_HRTIMER_FREE_ITDS,	/* Wait for unused iTDs and siTDs */
    FOTG210_HRTIMER_ASYNC_UNLINKS,	/* Unlink empty async QHs */
    FOTG210_HRTIMER_IAA_WATCHDOG,	/* Handle lost IAA interrupts */
    FOTG210_HRTIMER_DISABLE_PERIODIC, /* Wait to disable periodic sched */
    FOTG210_HRTIMER_DISABLE_ASYNC,	/* Wait to disable async sched */
    FOTG210_HRTIMER_IO_WATCHDOG,	/* Check for missing IRQs */
    FOTG210_HRTIMER_NUM_EVENTS	/* Must come last */
}

pub const FOTG210_HRTIMER_NO_EVENT: c_int = 99;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fotg210_hcd {
// timing support
    pub next_hrtimer_event: fotg210_hrtimer_event,
    pub enabled_hrtimer_events: unsigned,
    pub hr_timeouts: [ktime_t; FOTG210_HRTIMER_NUM_EVENTS],
    pub hrtimer: hrtimer,
    pub PSS_poll_count: c_int,
    pub ASS_poll_count: c_int,
    pub died_poll_count: c_int,
// glue to PCI and HCD framework
    pub caps: *mut fotg210_caps __iomem,
    pub regs: *mut fotg210_regs __iomem,
    pub debug: *mut ehci_dbg_port __iomem,
    pub /: *mut *mut __u32 hcs_params; / cached register copy,
    pub lock: spinlock_t,
    pub rh_state: fotg210_rh_state,
// general schedule support
    pub scanning:1: bool,
    pub need_rescan:1: bool,
    pub intr_unlinking:1: bool,
    pub async_unlinking:1: bool,
    pub shutdown:1: bool,
    pub qh_scan_next: *mut fotg210_qh,
// async schedule support
    pub async: *mut fotg210_qh,
    pub /: *mut *mut *mut fotg210_qh dummy; / For AMD quirk use,
    pub async_unlink: *mut fotg210_qh,
    pub async_unlink_last: *mut fotg210_qh,
    pub async_iaa: *mut fotg210_qh,
    pub async_unlink_cycle: unsigned,
    pub /: *mut *mut unsigned async_count; / async activity count,
// periodic schedule support

    pub periodic_size: unsigned,
    pub /: *mut *mut *mut __hc32 periodic; / hw periodic table,
    pub periodic_dma: dma_addr_t,
    pub intr_qh_list: list_head,
    pub /: *mut *mut unsigned i_thresh; / uframes HC might cache,
    pub /: *mut *mut *mut fotg210_shadow pshadow; / mirror hw periodic table,
    pub intr_unlink: *mut fotg210_qh,
    pub intr_unlink_last: *mut fotg210_qh,
    pub intr_unlink_cycle: unsigned,
    pub /: *mut *mut unsigned now_frame; / frame from HC hardware,
    pub /: *mut *mut unsigned next_frame; / scan periodic, start here,
    pub /: *mut *mut unsigned intr_count; / intr activity count,
    pub /: *mut *mut unsigned isoc_count; / isoc activity count,
    pub /: *mut *mut unsigned periodic_count; / periodic activity count,
// max periodic time per uframe
    pub uframe_periodic_max: unsigned,
// list of itds completed while now_frame was still active
    pub cached_itd_list: list_head,
    pub last_itd_to_free: *mut fotg210_itd,
// per root hub port
    pub reset_done: [c_ulong; FOTG210_MAX_ROOT_PORTS],
// bit vectors (one bit per port)
// which ports were already suspended at the start of a bus suspend
//
    pub bus_suspended: c_ulong,
// which ports are edicated to the companion controller
    pub companion_ports: c_ulong,
// which ports are owned by the companion during a bus suspend
    pub owned_ports: c_ulong,
// which ports have the change-suspend feature turned on
    pub port_c_suspend: c_ulong,
// which ports are suspended
    pub suspended_ports: c_ulong,
// which ports have started to resume
    pub resuming_ports: c_ulong,
// per-HC memory pools (could be per-bus, but ...)
    pub /: *mut *mut *mut dma_pool qh_pool; / qh per active urb,
    pub /: *mut *mut *mut dma_pool qtd_pool; / one or more per qh,
    pub /: *mut *mut *mut dma_pool itd_pool; / itd per iso urb,
    pub random_frame: unsigned,
    pub next_statechange: c_ulong,
    pub last_periodic_enable: ktime_t,
    pub command: u32,
// SILICON QUIRKS
    pub need_io_watchdog:1: unsigned,
    pub /: *mut *mut unsigned fs_i_thresh:1; / Intel iso scheduling,
    pub /: *mut *mut u8 sbrn; / packed release number,
// irq statistics

    pub stats: fotg210_stats,

    pub /: *mut *mut *mut fotg210 fotg; / Overarching FOTG210 device,
// silicon clock
    pub pclk: *mut clk,
}

// convert between an HCD pointer and the corresponding FOTG210_HCD
extern "C" {
    pub fn container_of(fotg210: *mut *mut (void ), usb_hcd: struct, _arg: hcd_priv) -> return;
}
// -------------------------------------------------------------------------
// EHCI register interface, corresponds to EHCI Revision 0.95 specification
// Section 2.2 Host Controller Capability Registers
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fotg210_caps {
// these fields are specified as 8 and 16 bit registers,
// but some hosts can't perform 8 or 16 bit PCI accesses.
// some hosts treat caplength and hciversion as parts of a 32-bit
// register, others treat them as two separate registers, this
// affects the memory map for big endian controllers.
//
    pub hc_capbase: u32,

    pub /: *mut *mut u32 hcs_params; / HCSPARAMS - offset 0x4,

    pub /: *mut *mut u32 hcc_params; / HCCPARAMS - offset 0x8,

    pub /: *mut *mut u8 portroute[8]; / nibbles for routing - offset 0xC,
}

// Section 2.3 Host Controller Operational Registers
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fotg210_regs {
// USBCMD: offset 0x00
    pub command: u32,
// EHCI 1.1 addendum
// 23:16 is r/w intr rate, in microframes; default "8" == 1/msec

// 3:2 is periodic frame list size

// USBSTS: offset 0x04
    pub status: u32,

// some bits reserved
// these STS_* flags are also intr_enable bits (USBINTR)

// USBINTR: offset 0x08
    pub intr_enable: u32,
// FRINDEX: offset 0x0C
    pub /: *mut *mut u32 frame_index; / current microframe number,
// CTRLDSSEGMENT: offset 0x10
    pub /: *mut *mut u32 segment; / address bits 63:32 if needed,
// PERIODICLISTBASE: offset 0x14
    pub /: *mut *mut u32 frame_list; / points to periodic list,
// ASYNCLISTADDR: offset 0x18
    pub /: *mut *mut u32 async_next; / address of next async queue head,
    pub reserved1: u32,
// PORTSC: offset 0x20
    pub port_status: u32,
// 31:23 reserved
    pub reserved2: [u32; 19],
// OTGCSR: offet 0x70
    pub otgcsr: u32,

// OTGISR: offset 0x74
    pub otgisr: u32,
    pub reserved3: [u32; 15],
// GMIR: offset 0xB4
    pub gmir: u32,

}

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
pub struct fotg210_qtd {
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
// mask NakCnt+T in qh->hw_alt_next

// -------------------------------------------------------------------------
// type tag from {qh,itd,fstn}->hw_next

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
pub union fotg210_shadow {
    pub /: *mut *mut *mut fotg210_qh qh; / Q_TYPE_QH,
    pub /: *mut *mut *mut fotg210_itd itd; / Q_TYPE_ITD,
    pub /: *mut *mut *mut fotg210_fstn fstn; / Q_TYPE_FSTN,
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
pub struct fotg210_qh_hw {
    pub /: *mut *mut __hc32 hw_next; / see EHCI 3.6.1,
    pub /: *mut *mut __hc32 hw_info1; / see EHCI 3.6.2,

    pub /: *mut *mut __hc32 hw_info2; / see EHCI 3.6.2,
pub const QH_SMASK: c_uint = 0x000000ff;
pub const QH_CMASK: c_uint = 0x0000ff00;
pub const QH_HUBADDR: c_uint = 0x007f0000;
pub const QH_HUBPORT: c_uint = 0x3f800000;
pub const QH_MULT: c_uint = 0xc0000000;
    pub /: *mut *mut __hc32 hw_current; / qtd list - see EHCI 3.6.4,
// qtd overlay (hardware parts of a struct fotg210_qtd)
    pub hw_qtd_next: __hc32,
    pub hw_alt_next: __hc32,
    pub hw_token: __hc32,
    pub hw_buf: [__hc32; 5],
    pub hw_buf_hi: [__hc32; 5],
    pub __aligned(32): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fotg210_qh {
    pub /: *mut *mut *mut fotg210_qh_hw hw; / Must come first,
// the rest is HCD-private
    pub /: *mut *mut dma_addr_t qh_dma; / address of qh,
    pub /: *mut *mut fotg210_shadow qh_next; / ptr to qh; or periodic,
    pub /: *mut *mut list_head qtd_list; / sw qtd list,
    pub /: *mut *mut list_head intr_node; / list of intr QHs,
    pub dummy: *mut fotg210_qtd,
    pub /: *mut *mut *mut fotg210_qh unlink_next; / next on unlink list,
    pub unlink_cycle: unsigned,
    pub /: *mut *mut u8 needs_rescan; / Dequeue during giveback,
    pub qh_state: u8,

    pub /: *mut *mut u8 xacterrs; / XactErr retry counter,

// periodic schedule info
    pub /: *mut *mut u8 usecs; / intr bandwidth,
    pub /: *mut *mut u8 gap_uf; / uframes split/csplit gap,
    pub /: *mut *mut u8 c_usecs; / ... split completion bw,
    pub /: *mut *mut u16 tt_usecs; / tt downstream bandwidth,
    pub /: *mut *mut unsigned short period; / polling interval,
    pub /: *mut *mut unsigned short start; / where polling starts,

    pub /: *mut *mut *mut usb_device dev; / access to TT,
    pub /: *mut *mut unsigned is_out:1; / bulk or intr OUT,
    pub /: *mut *mut unsigned clearing_tt:1; / Clear-TT-Buf in progress,
}

// -------------------------------------------------------------------------
// description of one iso transaction (up to 3 KB data if highspeed)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fotg210_iso_packet {
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
pub struct fotg210_iso_sched {
    pub td_list: list_head,
    pub span: unsigned,
    pub packet: [fotg210_iso_packet; ],
}

//
// fotg210_iso_stream - groups all (s)itds for this endpoint.
// acts like a qh would, if EHCI had them for ISO.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fotg210_iso_stream {
// first field matches fotg210_hq, but is NULL
    pub hw: *mut fotg210_qh_hw,
    pub bEndpointAddress: u8,
    pub highspeed: u8,
    pub /: *mut *mut list_head td_list; / queued itds,
    pub /: *mut *mut list_head free_list; / list of unused itds,
    pub udev: *mut usb_device,
    pub ep: *mut usb_host_endpoint,
// output of (re)scheduling
    pub next_uframe: c_int,
    pub splits: __hc32,
// the rest is derived from the endpoint descriptor,
// trusting urb->interval == f(epdesc->bInterval) and
// including the extra info for hw_bufp[0..2]
//
    pub c_usecs: u8 usecs,,
    pub interval: u16,
    pub tt_usecs: u16,
    pub maxp: u16,
    pub raw_mask: u16,
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
pub struct fotg210_itd {
// first part defined by EHCI spec
    pub /: *mut *mut __hc32 hw_next; / see EHCI 3.3.1,
    pub /: *mut *mut __hc32 hw_transaction[8]; / see EHCI 3.3.2,

    pub /: *mut *mut __hc32 hw_bufp[7]; / see EHCI 3.3.3,
    pub /: *mut *mut __hc32 hw_bufp_hi[7]; / Appendix B,
// the rest is HCD-private
    pub /: *mut *mut dma_addr_t itd_dma; / for this itd,
    pub /: *mut *mut fotg210_shadow itd_next; / ptr to periodic q entry,
    pub urb: *mut urb,
    pub /: *mut *mut *mut fotg210_iso_stream stream; / endpoint's queue,
    pub /: *mut *mut list_head itd_list; / list of stream's itds,
// any/all hw_transactions here may be used by that urb
    pub /: *mut *mut unsigned frame; / where scheduled,
    pub pg: unsigned,
    pub /: *mut *mut unsigned index[8]; / in urb->iso_frame_desc,
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
pub struct fotg210_fstn {
    pub /: *mut *mut __hc32 hw_next; / any periodic q entry,
    pub /: *mut *mut __hc32 hw_prev; / qh or FOTG210_LIST_END,
// the rest is HCD-private
    pub fstn_dma: dma_addr_t,
    pub /: *mut *mut fotg210_shadow fstn_next; / ptr to periodic q entry,
    pub __aligned(32): },
// -------------------------------------------------------------------------
// Prepare the PORTSC wakeup flags during controller suspend/resume

// -------------------------------------------------------------------------
//
// Some EHCI controllers have a Transaction Translator built into the
// root hub. This is a non-standard feature.  Each controller will need
// to add code to the following inline functions, and call them as
// needed (mostly in root hub code).
//
    pub 22: & OTGCSR_HOST_SPD_TYP) >>,
// Returns the speed of a device attached to a port on the root hub.
    pub 0: return,
    pub USB_PORT_STAT_LOW_SPEED: return,
    pub USB_PORT_STAT_HIGH_SPEED: return,
// -------------------------------------------------------------------------

//
// While most USB host controllers implement their registers in
// little-endian format, a minority (celleb companion chip) implement
// them in big endian format.
//
// This attempts to support either format at compile time without a
// runtime penalty, or both formats with the additional overhead
// of checking a flag bit.
//
pub const fotg210_big_endian_mmio(e): c_int = 0;
pub const fotg210_big_endian_capbase(e): c_int = 0;
    pub readl(regs): return,
    pub regs): writel(val,,
// cpu to fotg210
    pub cpu_to_le32(x): return,
// fotg210 to cpu
    pub le32_to_cpu(x): return,
    pub le32_to_cpup(x): return,
// -------------------------------------------------------------------------
    pub &fotg210->regs->frame_index): return fotg210_readl(fotg210,,
// -------------------------------------------------------------------------
