//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/chipidea/ci.h
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
// ci.h - common structures, functions, and macros of the ChipIdea driver
//
// Copyright (C) 2008 Chipidea - MIPS Technologies, Inc. All rights reserved.
//
// Author: David Lopo
//

//
// DEFINE
//
pub const TD_PAGE_COUNT: c_int = 5;

pub const ENDPT_MAX: c_int = 32;

//
// REGISTERS
//
// Identification Registers
pub const ID_ID: c_uint = 0x0;
pub const ID_HWGENERAL: c_uint = 0x4;
pub const ID_HWHOST: c_uint = 0x8;
pub const ID_HWDEVICE: c_uint = 0xc;
pub const ID_HWTXBUF: c_uint = 0x10;
pub const ID_HWRXBUF: c_uint = 0x14;
pub const ID_SBUSCFG: c_uint = 0x90;
// register indices
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ci_hw_regs {
    CAP_CAPLENGTH,
    CAP_HCCPARAMS,
    CAP_DCCPARAMS,
    CAP_TESTMODE,
    CAP_LAST = CAP_TESTMODE,
    OP_USBCMD,
    OP_USBSTS,
    OP_USBINTR,
    OP_FRINDEX,
    OP_DEVICEADDR,
    OP_ENDPTLISTADDR,
    OP_TTCTRL,
    OP_BURSTSIZE,
    OP_ULPI_VIEWPORT,
    OP_PORTSC,
    OP_DEVLC,
    OP_OTGSC,
    OP_USBMODE,
    OP_ENDPTSETUPSTAT,
    OP_ENDPTPRIME,
    OP_ENDPTFLUSH,
    OP_ENDPTSTAT,
    OP_ENDPTCOMPLETE,
    OP_ENDPTCTRL,
// endptctrl1..15 follow
    OP_LAST = OP_ENDPTCTRL + ENDPT_MAX / 2,
}

//
// STRUCTURES
//
// struct ci_hw_ep - endpoint representation
// @ep: endpoint structure for gadget drivers
// @dir: endpoint direction (TX/RX)
// @num: endpoint number
// @type: endpoint type
// @name: string description of the endpoint
// @qh: queue head for this endpoint
// @wedge: is the endpoint wedged
// @ci: pointer to the controller
// @lock: pointer to controller's spinlock
// @td_pool: pointer to controller's TD pool
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ci_hw_ep {
    pub ep: usb_ep,
    pub dir: u8,
    pub num: u8,
    pub type: u8,
    pub name: [c_char; 16],
    pub queue: list_head,
    pub ptr: *mut ci_hw_qh,
    pub dma: dma_addr_t,
    pub qh: },
    pub wedge: c_int,
// global resources
    pub ci: *mut ci_hdrc,
    pub lock: *mut spinlock_t,
    pub td_pool: *mut dma_pool,
    pub pending_td: *mut td_node,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ci_role {
    CI_ROLE_HOST = 0,
    CI_ROLE_GADGET,
    CI_ROLE_END,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ci_revision {
    CI_REVISION_1X = 10,	/* Revision 1.x */
    CI_REVISION_20 = 20, /* Revision 2.0 */
    CI_REVISION_21, /* Revision 2.1 */
    CI_REVISION_22, /* Revision 2.2 */
    CI_REVISION_23, /* Revision 2.3 */
    CI_REVISION_24, /* Revision 2.4 */
    CI_REVISION_25, /* Revision 2.5 */
    CI_REVISION_25_PLUS, /* Revision above than 2.5 */
    CI_REVISION_UNKNOWN = 99, /* Unknown Revision */
}

//
// struct ci_role_driver - host/gadget role driver
// @start: start this role
// @stop: stop this role
// @suspend: system suspend handler for this role
// @resume: system resume handler for this role
// @irq: irq handler for this role
// @name: role name string (host/gadget)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ci_role_driver {
    pub ): *mut *mut int (start)(struct ci_hdrc,
    pub ): *mut *mut void (stop)(struct ci_hdrc,
    pub ci): *mut *mut void (suspend)(struct ci_hdrc,
    pub power_lost): *mut *mut *mut void (resume)(struct ci_hdrc ci, bool,
    pub ): *mut *mut irqreturn_t (irq)(struct ci_hdrc,
    pub name: *const c_char,
}

//
// struct hw_bank - hardware register mapping representation
// @lpm: set if the device is LPM capable
// @phys: physical address of the controller's registers
// @abs: absolute address of the beginning of register window
// @cap: capability registers
// @op: operational registers
// @size: size of the register window
// @regmap: register lookup table
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hw_bank {
    pub lpm: unsigned,
    pub phys: resource_size_t,
    pub abs: *mut void __iomem,
    pub cap: *mut void __iomem,
    pub op: *mut void __iomem,
    pub size: usize,
    pub 1]: *mut *mut void __iomem regmap[OP_LAST +,
}

//
// struct ci_hdrc - chipidea device representation
// @dev: pointer to parent device
// @lock: access synchronization
// @hw_bank: hardware register mapping
// @irq: IRQ number
// @roles: array of supported roles for this controller
// @role: current role
// @is_otg: if the device is otg-capable
// @fsm: otg finite state machine
// @otg_fsm_hrtimer: hrtimer for otg fsm timers
// @hr_timeouts: time out list for active otg fsm timers
// @enabled_otg_timer_bits: bits of enabled otg timers
// @next_otg_timer: next nearest enabled timer to be expired
// @work: work for role changing
// @power_lost_work: work for power lost handling
// @wq: workqueue thread
// @qh_pool: allocation pool for queue heads
// @td_pool: allocation pool for transfer descriptors
// @gadget: device side representation for peripheral controller
// @driver: gadget driver
// @resume_state: save the state of gadget suspend from
// @hw_ep_max: total number of endpoints supported by hardware
// @ci_hw_ep: array of endpoints
// @ep0_dir: ep0 direction
// @ep0out: pointer to ep0 OUT endpoint
// @ep0in: pointer to ep0 IN endpoint
// @status: ep0 status request
// @setaddr: if we should set the address on status completion
// @address: usb address received from the host
// @remote_wakeup: host-enabled remote wakeup
// @suspended: suspended by host
// @test_mode: the selected test mode
// @platdata: platform specific information supplied by parent device
// @vbus_active: is VBUS active
// @ulpi: pointer to ULPI device, if any
// @ulpi_ops: ULPI read/write ops for this device
// @phy: pointer to PHY, if any
// @usb_phy: pointer to USB PHY, if any and if using the USB PHY framework
// @hcd: pointer to usb_hcd for ehci host driver
// @id_event: indicates there is an id event, and handled at ci_otg_work
// @b_sess_valid_event: indicates there is a vbus event, and handled
// at ci_otg_work
// @imx28_write_fix: Freescale imx28 needs swp instruction for writing
// @supports_runtime_pm: if runtime pm is supported
// @in_lpm: if the core in low power mode
// @wakeup_int: if wakeup interrupt occur
// @rev: The revision number for controller
// @mutex: protect code from concorrent running when doing role switch
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ci_hdrc {
    pub dev: *mut device,
    pub lock: spinlock_t,
    pub hw_bank: hw_bank,
    pub irq: c_int,
    pub roles: [*mut ci_role_driver; CI_ROLE_END],
    pub role: ci_role,
    pub is_otg: bool,
    pub otg: usb_otg,
    pub fsm: otg_fsm,
    pub otg_fsm_hrtimer: hrtimer,
    pub hr_timeouts: [ktime_t; NUM_OTG_FSM_TIMERS],
    pub enabled_otg_timer_bits: unsigned,
    pub next_otg_timer: otg_fsm_timer,
    pub role_switch: *mut usb_role_switch,
    pub work: work_struct,
    pub power_lost_work: work_struct,
    pub wq: *mut workqueue_struct,
    pub qh_pool: *mut dma_pool,
    pub td_pool: *mut dma_pool,
    pub gadget: usb_gadget,
    pub driver: *mut usb_gadget_driver,
    pub resume_state: usb_device_state,
    pub hw_ep_max: unsigned,
    pub ci_hw_ep: [ci_hw_ep; ENDPT_MAX],
    pub ep0_dir: u32,
    pub ep0in: *mut *mut ci_hw_ep ep0out,,
    pub status: *mut usb_request,
    pub setaddr: bool,
    pub address: u8,
    pub remote_wakeup: u8,
    pub suspended: u8,
    pub test_mode: u8,
    pub platdata: *mut ci_hdrc_platform_data,
    pub vbus_active: c_int,
    pub ulpi: *mut ulpi,
    pub ulpi_ops: ulpi_ops,
    pub phy: *mut phy,
// old usb_phy interface
    pub usb_phy: *mut usb_phy,
    pub hcd: *mut usb_hcd,
    pub id_event: bool,
    pub b_sess_valid_event: bool,
    pub imx28_write_fix: bool,
    pub has_portsc_pec_bug: bool,
    pub has_short_pkt_limit: bool,
    pub supports_runtime_pm: bool,
    pub in_lpm: bool,
    pub wakeup_int: bool,
    pub rev: ci_revision,
    pub mutex: mutex,
}

// in device mode but vbus is invalid
//
// hw_read_id_reg: reads from a identification register
// @ci: the controller
// @offset: offset from the beginning of identification registers region
// @mask: bitfield mask
//
// This function returns register contents
//
// hw_write_id_reg: writes to a identification register
// @ci: the controller
// @offset: offset from the beginning of identification registers region
// @mask: bitfield mask
// @data: new value
//
// hw_read: reads from a hw register
// @ci: the controller
// @reg:  register index
// @mask: bitfield mask
//
// This function returns register contents
//

//
// hw_write: writes to a hw register
// @ci: the controller
// @reg:  register index
// @mask: bitfield mask
// @data: new value
//
// hw_test_and_clear: tests & clears a hw register
// @ci: the controller
// @reg:  register index
// @mask: bitfield mask
//
// This function returns register contents
//
// hw_test_and_write: tests & writes a hw register
// @ci: the controller
// @reg:  register index
// @mask: bitfield mask
// @data: new value
//
// This function returns register contents
//
// ci_otg_is_fsm_mode: runtime check if otg controller
// is in otg fsm mode.
//
// @ci: chipidea device
//

extern "C" {
    pub fn ci_ulpi_init(ci: *mut ci_hdrc) -> c_int;
}
extern "C" {
    pub fn ci_ulpi_exit(ci: *mut ci_hdrc);
}
extern "C" {
    pub fn ci_ulpi_resume(ci: *mut ci_hdrc) -> c_int;
}
extern "C" {
    pub fn hw_read_intr_enable(ci: *mut ci_hdrc) -> u32;
}
extern "C" {
    pub fn hw_read_intr_status(ci: *mut ci_hdrc) -> u32;
}
extern "C" {
    pub fn hw_device_reset(ci: *mut ci_hdrc) -> c_int;
}
extern "C" {
    pub fn hw_port_test_set(ci: *mut ci_hdrc, mode: u8) -> c_int;
}
extern "C" {
    pub fn hw_port_test_get(ci: *mut ci_hdrc) -> u8;
}
extern "C" {
    pub fn hw_phymode_configure(ci: *mut ci_hdrc);
}
extern "C" {
    pub fn ci_platform_configure(ci: *mut ci_hdrc);
}
extern "C" {
    pub fn dbg_create_files(ci: *mut ci_hdrc);
}
extern "C" {
    pub fn dbg_remove_files(ci: *mut ci_hdrc);
}
