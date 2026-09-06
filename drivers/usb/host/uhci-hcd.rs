//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/host/uhci-hcd.h
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

pub const PIPE_DEVEP_MASK: c_uint = 0x0007ff00;
//
// Universal Host Controller Interface data structures and defines
//
// Command register
pub const USBCMD: c_int = 0;
pub const USBCMD_RS: c_uint = 0x0001	/* Run/Stop */;
pub const USBCMD_HCRESET: c_uint = 0x0002	/* Host reset */;
pub const USBCMD_GRESET: c_uint = 0x0004	/* Global reset */;
pub const USBCMD_EGSM: c_uint = 0x0008	/* Global Suspend Mode */;
pub const USBCMD_FGR: c_uint = 0x0010	/* Force Global Resume */;
pub const USBCMD_SWDBG: c_uint = 0x0020	/* SW Debug mode */;
pub const USBCMD_CF: c_uint = 0x0040	/* Config Flag (sw only) */;
pub const USBCMD_MAXP: c_uint = 0x0080	/* Max Packet (0 = 32, 1 = 64) */;
// Status register
pub const USBSTS: c_int = 2;
pub const USBSTS_USBINT: c_uint = 0x0001	/* Interrupt due to IOC */;
pub const USBSTS_ERROR: c_uint = 0x0002	/* Interrupt due to error */;
pub const USBSTS_RD: c_uint = 0x0004	/* Resume Detect */;
pub const USBSTS_HSE: c_uint = 0x0008	/* Host System Error: PCI problems */;
pub const USBSTS_HCPE: c_uint = 0x0010	/* Host Controller Process Error:;
// the schedule is buggy
pub const USBSTS_HCH: c_uint = 0x0020	/* HC Halted */;
// Interrupt enable register
pub const USBINTR: c_int = 4;
pub const USBINTR_TIMEOUT: c_uint = 0x0001	/* Timeout/CRC error enable */;
pub const USBINTR_RESUME: c_uint = 0x0002	/* Resume interrupt enable */;
pub const USBINTR_IOC: c_uint = 0x0004	/* Interrupt On Complete enable */;
pub const USBINTR_SP: c_uint = 0x0008	/* Short packet interrupt enable */;
pub const USBFRNUM: c_int = 6;
pub const USBFLBASEADD: c_int = 8;
pub const USBSOF: c_int = 12;

// USB port status and control registers
pub const USBPORTSC1: c_int = 16;
pub const USBPORTSC2: c_int = 18;
pub const USBPORTSC3: c_int = 20;
pub const USBPORTSC4: c_int = 22;
pub const USBPORTSC_CCS: c_uint = 0x0001	/* Current Connect Status;
// ("device present")
pub const USBPORTSC_CSC: c_uint = 0x0002	/* Connect Status Change */;
pub const USBPORTSC_PE: c_uint = 0x0004	/* Port Enable */;
pub const USBPORTSC_PEC: c_uint = 0x0008	/* Port Enable Change */;
pub const USBPORTSC_DPLUS: c_uint = 0x0010	/* D+ high (line status) */;
pub const USBPORTSC_DMINUS: c_uint = 0x0020	/* D- high (line status) */;
pub const USBPORTSC_RD: c_uint = 0x0040	/* Resume Detect */;
pub const USBPORTSC_RES1: c_uint = 0x0080	/* reserved, always 1 */;
pub const USBPORTSC_LSDA: c_uint = 0x0100	/* Low Speed Device Attached */;
pub const USBPORTSC_PR: c_uint = 0x0200	/* Port Reset */;
// OC and OCC from Intel 430TX and later (not UHCI 1.1d spec)
pub const USBPORTSC_OC: c_uint = 0x0400	/* Over Current condition */;
pub const USBPORTSC_OCC: c_uint = 0x0800	/* Over Current Change R/WC */;
pub const USBPORTSC_SUSP: c_uint = 0x1000	/* Suspend */;
pub const USBPORTSC_RES2: c_uint = 0x2000	/* reserved, write zeroes */;
pub const USBPORTSC_RES3: c_uint = 0x4000	/* reserved, write zeroes */;
pub const USBPORTSC_RES4: c_uint = 0x8000	/* reserved, write zeroes */;
// PCI legacy support register
pub const USBLEGSUP: c_uint = 0xc0;
pub const USBLEGSUP_DEFAULT: c_uint = 0x2000	/* only PIRQ enable set */;
pub const USBLEGSUP_RWC: c_uint = 0x8f00	/* the R/WC bits */;
pub const USBLEGSUP_RO: c_uint = 0x5040	/* R/O and reserved bits */;
// PCI Intel-specific resume-enable register
pub const USBRES_INTEL: c_uint = 0xc4;
pub const USBPORT1EN: c_uint = 0x01;
pub const USBPORT2EN: c_uint = 0x02;

// can be scheduled

// When no queues need Full-Speed Bandwidth Reclamation,
// delay this long before turning FSBR off

// If a queue hasn't advanced after this much time, assume it is stuck

//
// __hc32 and __hc16 are "Host Controller" types, they may be equivalent to
// __leXX (normally) or __beXX (given UHCI_BIG_ENDIAN_DESC), depending on
// the host controller implementation.
//
// To facilitate the strongest possible byte-order checking from "sparse"
// and so on, we use __leXX unless that's not practical.
//

pub type __hc32 = __u32 ;
pub type __hc16 = __u16 ;

//
// Queue Headers
//
// One role of a QH is to hold a queue of TDs for some endpoint.  One QH goes
// with each endpoint, and qh->element (updated by the HC) is either:
// - the next unprocessed TD in the endpoint's queue, or
// - UHCI_PTR_TERM (when there's no more traffic for this endpoint).
//
// The other role of a QH is to serve as a "skeleton" framelist entry, so we
// can easily splice a QH for some endpoint into the schedule at the right
// place.  Then qh->element is UHCI_PTR_TERM.
//
// In the schedule, qh->link maintains a list of QHs seen by the HC:
// skel1 --> ep1-qh --> ep2-qh --> ... --> skel2 --> ...
//
// qh->node is the software equivalent of qh->link.  The differences
// are that the software list is doubly-linked and QHs in the UNLINKING
// state are on the software list but not the hardware schedule.
//
// For bookkeeping purposes we maintain QHs even for Isochronous endpoints,
// but they never get added to the hardware schedule.
//

// schedule but the hardware may
// still be using it

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uhci_qh {
// Hardware fields
    pub /: *mut *mut __hc32 link; / Next QH in the schedule,
    pub /: *mut *mut __hc32 element; / Queue element (TD) pointer,
// Software fields
    pub dma_handle: dma_addr_t,
    pub /: *mut *mut list_head node; / Node in the list of QHs,
    pub /: *mut *mut *mut usb_host_endpoint hep; / Endpoint information,
    pub udev: *mut usb_device,
    pub /: *mut *mut list_head queue; / Queue of urbps for this QH,
    pub /: *mut *mut *mut uhci_td dummy_td; / Dummy TD to end the queue,
    pub /: *mut *mut *mut uhci_td post_td; / Last TD completed,
    pub iso_packet_desc: *mut usb_iso_packet_descriptor,
// Next urb->iso_frame_desc entry
    pub /: *mut *mut unsigned long advance_jiffies; / Time of last queue advance,
    pub /: *mut *mut unsigned int unlink_frame; / When the QH was unlinked,
    pub /: *mut *mut unsigned int period; / For Interrupt and Isochronous QHs,
    pub /: *mut *mut short phase; / Between 0 and period-1,
    pub /: *mut *mut short load; / Periodic time requirement, in us,
    pub /: *mut *mut unsigned int iso_frame; / Frame # for iso_packet_desc,
    pub /: *mut *mut int state; / QH_STATE_xxx; see above,
    pub /: *mut *mut int type; / Queue type (control, bulk, etc),
    pub /: *mut *mut int skel; / Skeleton queue number,
    pub /: *mut *mut unsigned int initial_toggle:1; / Endpoint's current toggle value,
    pub /: *mut *mut unsigned int needs_fixup:1; / Must fix the TD toggle values,
    pub /: *mut *mut unsigned int is_stopped:1; / Queue was stopped by error/unlink,
    pub /: *mut *mut unsigned int wait_expired:1; / QH_WAIT_TIMEOUT has expired,
    pub has: *mut *mut unsigned int bandwidth_reserved:1; / Periodic bandwidth,
// been allocated
    pub __attribute__((aligned(16))): },
//
// We need a special accessor for the element pointer because it is
// subject to asynchronous updates by the controller.
//

//
// Transfer Descriptors
//
// for TD <status>:
//

pub const TD_CTRL_C_ERR_SHIFT: c_int = 27;

pub const TD_CTRL_ACTLEN_MASK: c_uint = 0x7FF	/* actual length, encoded as n - 1 */;

//
// for TD <info>: (a.k.a. Token)
//

pub const TD_TOKEN_DEVADDR_SHIFT: c_int = 8;
pub const TD_TOKEN_TOGGLE_SHIFT: c_int = 19;

pub const TD_TOKEN_EXPLEN_SHIFT: c_int = 21;
pub const TD_TOKEN_EXPLEN_MASK: c_uint = 0x7FF	/* expected length, encoded as n-1 */;
pub const TD_TOKEN_PID_MASK: c_uint = 0xFF;

//
// The documentation says "4 words for hardware, 4 words for software".
//
// That's silly, the hardware doesn't care. The hardware only cares that
// the hardware words are 16-byte aligned, and we can have any amount of
// sw space after the TD entry.
//
// td->link points to either another TD (not necessarily for the same urb or
// even the same endpoint), or nothing (PTR_TERM), or a QH.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uhci_td {
// Hardware fields
    pub link: __hc32,
    pub status: __hc32,
    pub token: __hc32,
    pub buffer: __hc32,
// Software fields
    pub dma_handle: dma_addr_t,
    pub list: list_head,
    pub /: *mut *mut int frame; / for iso: what frame?,
    pub fl_list: list_head,
    pub __attribute__((aligned(16))): },
//
// We need a special accessor for the control/status word because it is
// subject to asynchronous updates by the controller.
//

//
// Skeleton Queue Headers
//
// The UHCI driver uses QHs with Interrupt, Control and Bulk URBs for
// automatic queuing. To make it easy to insert entries into the schedule,
// we have a skeleton of QHs for each predefined Interrupt latency.
// Asynchronous QHs (low-speed control, full-speed control, and bulk)
// go onto the period-1 interrupt list, since they all get accessed on
// every frame.
//
// When we want to add a new QH, we add it to the list starting from the
// appropriate skeleton QH.  For instance, the schedule can look like this:
//
// skel int128 QH
// dev 1 interrupt QH
// dev 5 interrupt QH
// skel int64 QH
// skel int32 QH
// ...
// skel int1 + async QH
// dev 5 low-speed control QH
// dev 1 bulk QH
// dev 2 bulk QH
//
// There is a special terminating QH used to keep full-speed bandwidth
// reclamation active when no full-speed control or bulk QHs are linked
// into the schedule.  It has an inactive TD (to work around a PIIX bug,
// see the Intel errata) and it points back to itself.
//
// There's a special skeleton QH for Isochronous QHs which never appears
// on the schedule.  Isochronous TDs go on the schedule before the
// skeleton QHs.  The hardware accesses them directly rather than
// through their QH, which is used only for bookkeeping purposes.
// While the UHCI spec doesn't forbid the use of QHs for Isochronous,
// it doesn't use them either.  And the spec says that queues never
// advance on an error completion status, which makes them totally
// unsuitable for Isochronous transfers.
//
// There's also a special skeleton QH used for QHs which are in the process
// of unlinking and so may still be in use by the hardware.  It too never
// appears on the schedule.
//
pub const UHCI_NUM_SKELQH: c_int = 11;
pub const SKEL_UNLINK: c_int = 0;

pub const SKEL_ISO: c_int = 1;

// int128, int64, ..., int1 = 2, 3, ..., 9

pub const SKEL_ASYNC: c_int = 9;

pub const SKEL_TERM: c_int = 10;

// The following entries refer to sublists of skel_async_qh
pub const SKEL_LS_CONTROL: c_int = 20;
pub const SKEL_FS_CONTROL: c_int = 21;

pub const SKEL_BULK: c_int = 22;
//
// The UHCI controller and root hub
//
// States for the root hub:
//
// To prevent "bouncing" in the presence of electrical noise,
// when there are no devices attached we delay for 1 second in the
// RUNNING_NODEVS state before switching to the AUTO_STOPPED state.
//
// (Note that the AUTO_STOPPED state won't be necessary once the hub
// driver learns to autosuspend.)
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum uhci_rh_state {
// In the following states the HC must be halted.
// These two must come first.
    UHCI_RH_RESET,
    UHCI_RH_SUSPENDED,

    UHCI_RH_AUTO_STOPPED,
    UHCI_RH_RESUMING,

// In this state the HC changes from running to halted,
// so it can legally appear either way.
    UHCI_RH_SUSPENDING,

// In the following states it's an error if the HC is halted.
// These two must come last.
    UHCI_RH_RUNNING,		/* The normal state */
    UHCI_RH_RUNNING_NODEVS,		/* Running with no devices attached */
}

//
// The full UHCI controller information:
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uhci_hcd {
// Grabbed from PCI
    pub io_addr: c_ulong,
// Used when registers are memory mapped
    pub regs: *mut void __iomem,
    pub qh_pool: *mut dma_pool,
    pub td_pool: *mut dma_pool,
    pub /: *mut *mut *mut uhci_td term_td; / Terminating TD, see UHCI bug,
    pub /: *mut *mut *mut uhci_qh skelqh[UHCI_NUM_SKELQH]; / Skeleton QHs,
    pub /: *mut *mut *mut uhci_qh next_qh; / Next QH to scan,
    pub lock: spinlock_t,
    pub /: *mut *mut dma_addr_t frame_dma_handle; / Hardware frame list,
    pub frame: *mut __hc32,
    pub /: *mut *mut *mut *mut void frame_cpu; / CPU's frame list,
    pub rh_state: uhci_rh_state,
    pub /: *mut *mut unsigned long auto_stop_time; / When to AUTO_STOP,
    pub /: *mut *mut unsigned int frame_number; / As of last check,
    pub is_stopped: c_uint,

    pub /: *mut *mut unsigned int last_iso_frame; / Frame of last scan,
    pub /: *mut *mut unsigned int cur_iso_frame; / Frame for current scan,
    pub /: *mut *mut unsigned int scan_in_progress:1; / Schedule scan is running,
    pub /: *mut *mut unsigned int need_rescan:1; / Redo the schedule scan,
    pub /: *mut *mut unsigned int dead:1; / Controller has died,
    pub with: *mut *mut unsigned int RD_enable:1; / Suspended root hub,
    pub /: *mut *mut unsigned int is_initialized:1; / Data structure is usable,
    pub /: *mut *mut unsigned int fsbr_is_on:1; / FSBR is turned on,
    pub /: *mut *mut unsigned int fsbr_is_wanted:1; / Does any URB want FSBR?,
    pub /: *mut *mut unsigned int fsbr_expiring:1; / FSBR is timing out,
    pub /: *mut *mut timer_list fsbr_timer; / For turning off FBSR,
// Silicon quirks
    pub /: *mut *mut unsigned int oc_low:1; / OverCurrent bit active low,
    pub /: *mut *mut unsigned int wait_for_hp:1; / Wait for HP port reset,
    pub /: *mut *mut unsigned int big_endian_mmio:1; / Big endian registers,
    pub /: *mut *mut unsigned int big_endian_desc:1; / Big endian descriptors,
    pub /: *mut *mut unsigned int is_aspeed:1; / Aspeed impl. workarounds,
// Support for port suspend/resume/reset
    pub /: *mut *mut unsigned long port_c_suspend; / Bit-arrays of ports,
    pub resuming_ports: c_ulong,
    pub /: *mut *mut unsigned long ports_timeout; / Time to stop signalling,
    pub /: *mut *mut list_head idle_qh_list; / Where the idle QHs live,
    pub /: *mut *mut int rh_numports; / Number of root-hub ports,
    pub /: *mut *mut wait_queue_head_t waitqh; / endpoint_disable waiters,
    pub /: *mut *mut int num_waiting; / Number of waiters,
    pub /: *mut *mut int total_load; / Sum of array values,
    pub /: *mut *mut short load[MAX_PHASE]; / Periodic allocations,
    pub /: *mut *mut *mut clk clk; / (optional) clock source,
    pub /: *mut *mut *mut reset_control rsts; / (optional) clock reset,
// Reset host controller
    pub uhci): *mut *mut void (reset_hc) (struct uhci_hcd,
    pub uhci): *mut *mut int (check_and_reset_hc) (struct uhci_hcd,
// configure_hc should perform arch specific settings, if needed
    pub uhci): *mut *mut void (configure_hc) (struct uhci_hcd,
// Check for broken resume detect interrupts
    pub uhci): *mut *mut int (resume_detect_interrupts_are_broken) (struct uhci_hcd,
// Check for broken global suspend
    pub uhci): *mut *mut int (global_suspend_mode_is_broken) (struct uhci_hcd,
}

// Convert between a usb_hcd pointer and the corresponding uhci_hcd
extern "C" {
    pub fn container_of(uhci: *mut *mut (void ), usb_hcd: struct, _arg: hcd_priv) -> return;
}

// Utility macro for comparing frame numbers

//
// Private per-URB data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct urb_priv {
    pub /: *mut *mut list_head node; / Node in the QH's urbp list,
    pub urb: *mut urb,
    pub /: *mut *mut *mut uhci_qh qh; / QH for this URB,
    pub td_list: list_head,
    pub /: *mut *mut unsigned fsbr:1; / URB wants FSBR,
}

// Some special IDs
pub const PCI_VENDOR_ID_GENESYS: c_uint = 0x17a0;
pub const PCI_DEVICE_ID_GL880S_UHCI: c_uint = 0x8083;
// Aspeed SoC needs some quirks
//
// Functions used to access controller registers. The UCHI spec says that host
// controller I/O registers are mapped into PCI I/O space. For non-PCI hosts
// we use memory mapped registers.
//

pub const UHCI_IN(x): c_int = 0;

// Support PCI only
extern "C" {
    pub fn inl(reg: uhci->io_addr +) -> return;
}
extern "C" {
    pub fn inw(reg: uhci->io_addr +) -> return;
}
extern "C" {
    pub fn inb(reg: uhci->io_addr +) -> return;
}

// Support non-PCI host controllers

// Support PCI and non-PCI host controllers

// Support non-PCI host controllers only
pub const uhci_has_pci_registers(u): c_int = 0;

// Support (non-PCI) big endian host controllers

pub const uhci_big_endian_mmio(u): c_int = 0;

// Return an unimplemented register
extern "C" {
    pub fn UHCI_IN(reg): inl(uhci->io_addr +) -> return;
}
extern "C" {
    pub fn readl(uhci_aspeed_reg(reg): uhci->regs +) -> return;
}

extern "C" {
    pub fn readl_be(reg: uhci->regs +) -> return;
}

extern "C" {
    pub fn readl(reg: uhci->regs +) -> return;
}

extern "C" {
    pub fn UHCI_IN(reg): inw(uhci->io_addr +) -> return;
}
extern "C" {
    pub fn readl(uhci_aspeed_reg(reg): uhci->regs +) -> return;
}

extern "C" {
    pub fn readw_be(reg: uhci->regs +) -> return;
}

extern "C" {
    pub fn readw(reg: uhci->regs +) -> return;
}

extern "C" {
    pub fn UHCI_IN(reg): inb(uhci->io_addr +) -> return;
}
extern "C" {
    pub fn readl(uhci_aspeed_reg(reg): uhci->regs +) -> return;
}

extern "C" {
    pub fn readb_be(reg: uhci->regs +) -> return;
}

extern "C" {
    pub fn readb(reg: uhci->regs +) -> return;
}

//
// The GRLIB GRUSBHC controller can use big endian format for its descriptors.
//
// UHCI controllers accessed through PCI work normally (little-endian
// everywhere), so we don't bother supporting a BE-only mode.
//

// cpu to uhci
// uhci to cpu

// cpu to uhci
extern "C" {
    pub fn cpu_to_le32(_arg: x) -> return;
}
// uhci to cpu
extern "C" {
    pub fn le32_to_cpu(_arg: x) -> return;
}

