//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/host/ohci.h
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


// SPDX-License-Identifier: GPL-1.0+
//
// OHCI HCD (Host Controller Driver) for USB.
//
// (C) Copyright 1999 Roman Weissgaerber <weissg@vienna.at>
// (C) Copyright 2000-2002 David Brownell <dbrownell@users.sourceforge.net>
//
// This file is licenced under the GPL.
//
// __hc32 and __hc16 are "Host Controller" types, they may be equivalent to
// __leXX (normally) or __beXX (given OHCI_BIG_ENDIAN), depending on the
// host controller implementation.
//
pub type __hc32 = __u32 ;
pub type __hc16 = __u16 ;
//
// OHCI Endpoint Descriptor (ED) ... holds TD queue
// See OHCI spec, section 4.2
//
// This is a "Queue Head" for those transfers, which is why
// both EHCI and UHCI call similar structures a "QH".
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ed {
// first fields are hardware-specified
    pub /: *mut *mut __hc32 hwINFO; / endpoint config bitmap,
// info bits defined by hcd

// info bits defined by the hardware

    pub /: *mut *mut __hc32 hwTailP; / tail of TD list,
    pub /: *mut *mut __hc32 hwHeadP; / head of TD list (hc r/w),

    pub /: *mut *mut __hc32 hwNextED; / next ED in list,
// rest are purely for the driver's use
    pub /: *mut *mut dma_addr_t dma; / addr of ED,
    pub /: *mut *mut *mut td dummy; / next TD to activate,
// host's view of schedule
    pub /: *mut *mut *mut ed ed_next; / on schedule or rm_list,
    pub /: *mut *mut *mut ed ed_prev; / for non-interrupt EDs,
    pub /: *mut *mut list_head td_list; / "shadow list" of our TDs,
    pub in_use_list: list_head,
// create --> IDLE --> OPER --> ... --> IDLE --> destroy
// usually:  OPER --> UNLINK --> (IDLE | OPER) --> ...
//
    pub /: *mut *mut u8 state; / ED_{IDLE,UNLINK,OPER},
pub const ED_IDLE: c_uint = 0x00		/* NOT linked to HC */;
pub const ED_UNLINK: c_uint = 0x01		/* being unlinked from hc */;
pub const ED_OPER: c_uint = 0x02		/* IS linked to hc */;
    pub /: *mut *mut u8 type; / PIPE_{BULK,...},
// periodic scheduling params (for intr and iso)
    pub branch: u8,
    pub interval: u16,
    pub load: u16,
    pub /: *mut *mut u16 last_iso; / iso only,
// HC may see EDs on rm_list until next frame (frame_no == tick)
    pub tick: u16,
// Detect TDs not added to the done queue
    pub takeback_wdh_cnt: unsigned,
    pub pending_td: *mut td,
// C attribute field omitted

//
// OHCI Transfer Descriptor (TD) ... one per transfer segment
// See OHCI spec, sections 4.3.1 (general = control/bulk/interrupt)
// and 4.3.2 (iso)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct td {
// first fields are hardware-specified
    pub /: *mut *mut __hc32 hwINFO; / transfer info bitmask,
// hwINFO bits for both general and iso tds:
pub const TD_CC: c_uint = 0xf0000000			/* condition code */;

// #define TD_CC_SET(td_p, cc) (td_p) = ((td_p) & 0x0fffffff) | (((cc) & 0x0f) << 28)
pub const TD_DI: c_uint = 0x00E00000			/* frames before interrupt */;

// these two bits are available for definition/use by HCDs in both
// general and iso tds ... others are available for only one type
//
pub const TD_DONE: c_uint = 0x00020000			/* retired to donelist */;
pub const TD_ISO: c_uint = 0x00010000			/* copy of ED_ISO */;
// hwINFO bits for general tds:
pub const TD_EC: c_uint = 0x0C000000			/* error count */;
pub const TD_T: c_uint = 0x03000000			/* data toggle state */;
pub const TD_T_DATA0: c_uint = 0x02000000				/* DATA0 */;
pub const TD_T_DATA1: c_uint = 0x03000000				/* DATA1 */;
pub const TD_T_TOGGLE: c_uint = 0x00000000				/* uses ED_C */;
pub const TD_DP: c_uint = 0x00180000			/* direction/pid */;
pub const TD_DP_SETUP: c_uint = 0x00000000			/* SETUP pid */;
pub const TD_DP_IN: c_uint = 0x00100000				/* IN pid */;
pub const TD_DP_OUT: c_uint = 0x00080000				/* OUT pid */;
// 0x00180000 rsvd
pub const TD_R: c_uint = 0x00040000			/* round: short packets OK? */;
// (no hwINFO #defines yet for iso tds)
    pub /: *mut *mut __hc32 hwCBP; / Current Buffer Pointer (or 0),
    pub /: *mut *mut __hc32 hwNextTD; / Next TD Pointer,
    pub /: *mut *mut __hc32 hwBE; / Memory Buffer End Pointer,
// PSW is only for ISO.  Only 1 PSW entry is used, but on
// big-endian PPC hardware that's the second entry.
//
pub const MAXPSW: c_int = 2;
    pub [MAXPSW]: __hc16 hwPSW,
// rest are purely for the driver's use
    pub index: __u8,
    pub ed: *mut ed,
    pub /: *mut *mut *mut td td_hash; / dma-->td hashtable,
    pub next_dl_td: *mut td,
    pub urb: *mut urb,
    pub /: *mut *mut dma_addr_t td_dma; / addr of this TD,
    pub /: *mut *mut dma_addr_t data_dma; / addr of data it points to,
    pub /: *mut *mut list_head td_list; / "shadow list", TDs on same ED,
    pub /: *mut *mut } __attribute__ ((aligned(32))); / c/b/i need 16; only iso needs 32,

//
// Hardware transfer status codes -- CC from td->hwINFO or td->hwPSW
//
pub const TD_CC_NOERROR: c_uint = 0x00;
pub const TD_CC_CRC: c_uint = 0x01;
pub const TD_CC_BITSTUFFING: c_uint = 0x02;
pub const TD_CC_DATATOGGLEM: c_uint = 0x03;
pub const TD_CC_STALL: c_uint = 0x04;
pub const TD_DEVNOTRESP: c_uint = 0x05;
pub const TD_PIDCHECKFAIL: c_uint = 0x06;
pub const TD_UNEXPECTEDPID: c_uint = 0x07;
pub const TD_DATAOVERRUN: c_uint = 0x08;
pub const TD_DATAUNDERRUN: c_uint = 0x09;
// 0x0A, 0x0B reserved for hardware
pub const TD_BUFFEROVERRUN: c_uint = 0x0C;
pub const TD_BUFFERUNDERRUN: c_uint = 0x0D;
// 0x0E, 0x0F reserved for HCD
pub const TD_NOTACCESSED: c_uint = 0x0F;
// map OHCI TD status codes (CC) to errno values
// No  Error  */               0,
// CRC Error  */               -EILSEQ,
// Bit Stuff  */               -EPROTO,
// Data Togg  */               -EILSEQ,
// Stall      */               -EPIPE,
// DevNotResp */               -ETIME,
// PIDCheck   */               -EPROTO,
// UnExpPID   */               -EPROTO,
// DataOver   */               -EOVERFLOW,
// DataUnder  */               -EREMOTEIO,
// (for hw)   */               -EIO,
// BufferOver */               -ECOMM,
// BuffUnder  */               -ENOSR,
// (for HCD)  */               -EALREADY,
// (for HCD)  */               -EALREADY
}

//
// The HCCA (Host Controller Communications Area) is a 256 byte
// structure defined section 4.4.1 of the OHCI spec. The HC is
// told the base address of it.  It must be 256-byte aligned.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ohci_hcca {
pub const NUM_INTS: c_int = 32;
    pub /: *mut *mut __hc32 int_table [NUM_INTS]; / periodic schedule,
//
// OHCI defines u16 frame_no, followed by u16 zero pad.
// Since some processors can't do 16 bit bus accesses,
// portable access must be a 32 bits wide.
//
    pub /: *mut *mut __hc32 frame_no; / current frame number,
    pub /: *mut *mut __hc32 done_head; / info returned for an interrupt,
    pub [116]: u8 reserved_for_hc,
    pub /: *mut *mut u8 what [4]; / spec only identifies 252 bytes :),
// C attribute field omitted
//
// This is the structure of the OHCI controller's memory mapped I/O region.
// You must use readl() and writel() (in <asm/io.h>) to access these fields!!
// Layout is in section 7 (and appendix B) of the spec.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ohci_regs {
// control and status registers (section 7.1)
    pub revision: __hc32,
    pub control: __hc32,
    pub cmdstatus: __hc32,
    pub intrstatus: __hc32,
    pub intrenable: __hc32,
    pub intrdisable: __hc32,
// memory pointers (section 7.2)
    pub hcca: __hc32,
    pub ed_periodcurrent: __hc32,
    pub ed_controlhead: __hc32,
    pub ed_controlcurrent: __hc32,
    pub ed_bulkhead: __hc32,
    pub ed_bulkcurrent: __hc32,
    pub donehead: __hc32,
// frame counters (section 7.3)
    pub fminterval: __hc32,
    pub fmremaining: __hc32,
    pub fmnumber: __hc32,
    pub periodicstart: __hc32,
    pub lsthresh: __hc32,
// Root hub ports (section 7.4)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ohci_roothub_regs {
    pub a: __hc32,
    pub b: __hc32,
    pub status: __hc32,

    pub [MAX_ROOT_PORTS]: __hc32 portstatus,
    pub roothub: },
// and optional "legacy support" registers (appendix B) at 0x0100
// C attribute field omitted
// OHCI CONTROL AND STATUS REGISTER MASKS
//
// HcControl (control) register masks
//

// pre-shifted values for HCFS

//
// HcCommandStatus (cmdstatus) register masks
//

//
// masks used with interrupt registers:
// HcInterruptStatus (intrstatus)
// HcInterruptEnable (intrenable)
// HcInterruptDisable (intrdisable)
//

// OHCI ROOT HUB REGISTER MASKS
// roothub.portstatus [i] bits
pub const RH_PS_CCS: c_uint = 0x00000001		/* current connect status */;
pub const RH_PS_PES: c_uint = 0x00000002		/* port enable status*/;
pub const RH_PS_PSS: c_uint = 0x00000004		/* port suspend status */;
pub const RH_PS_POCI: c_uint = 0x00000008		/* port over current indicator */;
pub const RH_PS_PRS: c_uint = 0x00000010		/* port reset status */;
pub const RH_PS_PPS: c_uint = 0x00000100		/* port power status */;
pub const RH_PS_LSDA: c_uint = 0x00000200		/* low speed device attached */;
pub const RH_PS_CSC: c_uint = 0x00010000		/* connect status change */;
pub const RH_PS_PESC: c_uint = 0x00020000		/* port enable status change */;
pub const RH_PS_PSSC: c_uint = 0x00040000		/* port suspend status change */;
pub const RH_PS_OCIC: c_uint = 0x00080000		/* over current indicator change */;
pub const RH_PS_PRSC: c_uint = 0x00100000		/* port reset status change */;
// roothub.status bits
pub const RH_HS_LPS: c_uint = 0x00000001		/* local power status */;
pub const RH_HS_OCI: c_uint = 0x00000002		/* over current indicator */;
pub const RH_HS_DRWE: c_uint = 0x00008000		/* device remote wakeup enable */;
pub const RH_HS_LPSC: c_uint = 0x00010000		/* local power status change */;
pub const RH_HS_OCIC: c_uint = 0x00020000		/* over current indicator change */;
pub const RH_HS_CRWE: c_uint = 0x80000000		/* clear remote wakeup enable */;
// roothub.b masks
pub const RH_B_DR: c_uint = 0x0000ffff		/* device removable flags */;
pub const RH_B_PPCM: c_uint = 0xffff0000		/* port power control mask */;
// roothub.a masks

// hcd-private per-urb state
    pub ed: *mut ed,
    pub request: u16 length; // # tds in this,
    pub serviced: u16 td_cnt; // tds already,
    pub pending: list_head,
    pub request: *mut *mut td td[] __counted_by(length); // all TDs in this,
    pub urb_priv_t: },

// sizeof (struct td) ~= 64 == 2^6 ...

//
// This is the full ohci controller description
//
// Note how the "proper" USB information is just
// a subset of what the full implementation needs. (Linus)
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ohci_rh_state {
    OHCI_RH_HALTED,
    OHCI_RH_SUSPENDED,
    OHCI_RH_RUNNING
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ohci_hcd {
    pub lock: spinlock_t,
//
// I/O memory used to communicate with the HC (dma-consistent)
//
    pub regs: *mut ohci_regs __iomem,
//
// main memory used to communicate with the HC (dma-consistent).
// hcd adds to schedule for a live hc any time, but removals finish
// only at the start of the next frame.
//
    pub hcca: *mut ohci_hcca,
    pub hcca_dma: dma_addr_t,
    pub /: *mut *mut *mut ed ed_rm_list; / to be removed,
    pub /: *mut *mut *mut ed ed_bulktail; / last in bulk list,
    pub /: *mut *mut *mut ed ed_controltail; / last in ctrl list,
    pub /: *mut *mut *mut ed periodic [NUM_INTS]; / shadow int_table,
    pub ohci): *mut *mut void (start_hnp)(struct ohci_hcd,
//
// memory management for queue data structures
//
// @td_cache and @ed_cache are %NULL if &usb_hcd.localmem_pool is used.
//
    pub td_cache: *mut dma_pool,
    pub ed_cache: *mut dma_pool,
    pub [TD_HASH_SIZE]: *mut *mut td td_hash,
    pub /: *mut *mut *mut *mut td dl_start, dl_end; / the done list,
    pub pending: list_head,
    pub /: *mut *mut list_head eds_in_use; / all EDs with at least 1 TD,
//
// driver state
//
    pub rh_state: ohci_rh_state,
    pub num_ports: c_int,
    pub [NUM_INTS]: int load,
    pub /: *mut *mut u32 hc_control; / copy of hc control reg,
    pub /: *mut *mut unsigned long next_statechange; / suspend/resume,
    pub /: *mut *mut u32 fminterval; / saved register,
    pub /: *mut *mut unsigned autostop:1; / rh auto stopping/stopped,
    pub working:1: unsigned,
    pub restart_work:1: unsigned,
    pub /: *mut *mut unsigned long flags; / for HC bugs,
pub const OHCI_QUIRK_AMD756: c_uint = 0x01			/* erratum #4 */;
pub const OHCI_QUIRK_SUPERIO: c_uint = 0x02			/* natsemi */;
pub const OHCI_QUIRK_INITRESET: c_uint = 0x04			/* SiS, OPTi, ... */;
pub const OHCI_QUIRK_BE_DESC: c_uint = 0x08			/* BE descriptors */;
pub const OHCI_QUIRK_BE_MMIO: c_uint = 0x10			/* BE registers */;
pub const OHCI_QUIRK_ZFMICRO: c_uint = 0x20			/* Compaq ZFMicro chipset*/;
pub const OHCI_QUIRK_NEC: c_uint = 0x40			/* lost interrupts */;
pub const OHCI_QUIRK_FRAME_NO: c_uint = 0x80			/* no big endian frame_no shift */;
pub const OHCI_QUIRK_HUB_POWER: c_uint = 0x100			/* distrust firmware power/oc setup */;
pub const OHCI_QUIRK_AMD_PLL: c_uint = 0x200			/* AMD PLL quirk*/;
pub const OHCI_QUIRK_AMD_PREFETCH: c_uint = 0x400			/* pre-fetch for ISO transfer */;
pub const OHCI_QUIRK_GLOBAL_SUSPEND: c_uint = 0x800		/* must suspend ports */;
pub const OHCI_QUIRK_QEMU: c_uint = 0x1000			/* relax timing expectations */;
// there are also chip quirks/bugs in init logic
    pub prev_frame_no: unsigned,
    pub prev_wdh_cnt: unsigned wdh_cnt,,
    pub prev_donehead: u32,
    pub io_watchdog: timer_list,
    pub /: *mut *mut work_nec_work; / Worker for NEC quirk,
    pub debug_dir: *mut dentry,
// platform-specific data -- must come last
    pub __aligned(sizeof(s64)): unsigned long priv[],
}

// convert between an hcd pointer and the corresponding ohci_hcd
extern "C" {
    pub fn container_of(ohci: *mut *mut (void ), usb_hcd: struct, _arg: hcd_priv) -> return;
}
// -------------------------------------------------------------------------

// -------------------------------------------------------------------------
//
// While most USB host controllers implement their registers and
// in-memory communication descriptors in little-endian format,
// a minority (notably the IBM STB04XXX and the Motorola MPC5200
// processors) implement them in big endian format.
//
// In addition some more exotic implementations like the Toshiba
// Spider (aka SCC) cell southbridge are "mixed" endian, that is,
// they have a different endianness for registers vs. in-memory
// descriptors.
//
// This attempts to support either format at compile time without a
// runtime penalty, or both formats with the additional overhead
// of checking a flag bit.
//
// That leads to some tricky Kconfig rules howevber. There are
// different defaults based on some arch/ppc platforms, though
// the basic rules are:
//
// Controller type              Kconfig options needed
// ---------------              ----------------------
// little endian                CONFIG_USB_OHCI_LITTLE_ENDIAN
//
// fully big endian             CONFIG_USB_OHCI_BIG_ENDIAN_DESC _and_
// CONFIG_USB_OHCI_BIG_ENDIAN_MMIO
//
// mixed endian                 CONFIG_USB_OHCI_LITTLE_ENDIAN _and_
// CONFIG_USB_OHCI_BIG_ENDIAN_{MMIO,DESC}
//
// (If you have a mixed endian controller, you -must- also define
// CONFIG_USB_OHCI_LITTLE_ENDIAN or things will not work when building
// both your mixed endian and a fully big endian controller support in
// the same kernel image).
//

//
// Big-endian read/write functions are arch-specific.
// Other arches can be added if/when they're needed.
//

extern "C" {
    pub fn readl(_arg: regs) -> return;
}

// -------------------------------------------------------------------------
// cpu to ohci
// ohci to cpu
// -------------------------------------------------------------------------
//
// The HCCA frame number is 16 bits, but is accessed as 32 bits since not all
// hardware handles 16 bit reads.  Depending on the SoC implementation, the
// frame number can wind up in either bits [31:16] (default) or
// [15:0] (OHCI_QUIRK_FRAME_NO) on big endian hosts.
//
// Somewhat similarly, the 16-bit PSW fields in a transfer descriptor are
// reordered on BE.
//
extern "C" {
    pub fn hc16_to_cpup(_arg: ohci, _arg: ohci_hwPSWp(ohci, _arg: td, _arg: index)) -> return;
}
// -------------------------------------------------------------------------
pub const FI: c_uint = 0x2edf		/* 12000 bits per frame (-1) */;

pub const LSTHRESH: c_uint = 0x628		/* lowspeed bit threshold */;
// AMD-756 (D2 rev) reports corrupt register contents in some cases.
// The erratum (#4) description is incorrect.  AMD's workaround waits
// till some bits (mostly reserved) are clear; ok for all revs.
//

// Declarations of things exported for use by ohci platform drivers
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ohci_driver_overrides {
    pub product_desc: *const c_char,
    pub extra_priv_size: usize,
    pub hcd): *mut *mut int (reset)(struct usb_hcd,
}

extern "C" {
    pub fn ohci_restart(ohci: *mut ohci_hcd) -> c_int;
}
extern "C" {
    pub fn ohci_setup(hcd: *mut usb_hcd) -> c_int;
}
extern "C" {
    pub fn ohci_suspend(hcd: *mut usb_hcd, do_wakeup: bool) -> c_int;
}
extern "C" {
    pub fn ohci_resume(hcd: *mut usb_hcd, hibernated: bool) -> c_int;
}
extern "C" {
    pub fn ohci_hub_status_data(hcd: *mut usb_hcd, buf: *mut c_char) -> c_int;
}
