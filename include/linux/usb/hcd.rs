//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/usb/hcd.h
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

pub const MAX_TOPO_LEVEL: c_int = 6;
// This file contains declarations of usbcore internals that are mostly
// used or exposed by Host Controller Drivers.
//
// USB Packet IDs (PIDs)
//
pub const USB_PID_EXT: c_uint = 0xf0	/* USB 2.0 LPM ECN */;
pub const USB_PID_OUT: c_uint = 0xe1;
pub const USB_PID_ACK: c_uint = 0xd2;
pub const USB_PID_DATA0: c_uint = 0xc3;
pub const USB_PID_PING: c_uint = 0xb4	/* USB 2.0 */;
pub const USB_PID_SOF: c_uint = 0xa5;
pub const USB_PID_NYET: c_uint = 0x96	/* USB 2.0 */;
pub const USB_PID_DATA2: c_uint = 0x87	/* USB 2.0 */;
pub const USB_PID_SPLIT: c_uint = 0x78	/* USB 2.0 */;
pub const USB_PID_IN: c_uint = 0x69;
pub const USB_PID_NAK: c_uint = 0x5a;
pub const USB_PID_DATA1: c_uint = 0x4b;
pub const USB_PID_PREAMBLE: c_uint = 0x3c	/* Token mode */;
pub const USB_PID_ERR: c_uint = 0x3c	/* USB 2.0: handshake mode */;
pub const USB_PID_SETUP: c_uint = 0x2d;
pub const USB_PID_STALL: c_uint = 0x1e;
pub const USB_PID_MDATA: c_uint = 0x0f	/* USB 2.0 */;
// -------------------------------------------------------------------------
//
// USB Host Controller Driver (usb_hcd) framework
//
// Since "struct usb_bus" is so thin, you can't share much code in it.
// This framework is a layer over that, and should be more shareable.
//
// -------------------------------------------------------------------------
#[repr(C)]
#[derive(Copy, Clone)]
pub struct giveback_urb_bh {
    pub running: bool,
    pub high_prio: bool,
    pub lock: spinlock_t,
    pub head: list_head,
    pub bh: work_struct,
    pub completing_ep: *mut usb_host_endpoint,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum usb_dev_authorize_policy {
    USB_DEVICE_AUTHORIZE_NONE	= 0,
    USB_DEVICE_AUTHORIZE_ALL	= 1,
    USB_DEVICE_AUTHORIZE_INTERNAL	= 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_hcd {
//
// housekeeping
//
    pub /: *mut *mut usb_bus self; / hcd is-a bus,
    pub /: *mut *mut kref kref; / reference counter,
    pub /: *const *const *const char product_desc; / product/vendor string,
    pub roothub.: *mut *mut int speed; / Speed for this,
// May be different from
// hcd->driver->flags & HCD_MASK
//
    pub /: *mut *mut char irq_descr[24]; / driver + bus #,
    pub /: *mut *mut timer_list rh_timer; / drives root-hub polling,
    pub /: *mut *mut *mut urb status_urb; / the current status urb,

    pub /: *mut *mut work_wakeup_work; / for remote wakeup,

    pub /: *mut *mut work_died_work; / for when the device dies,
//
// hardware info/state
//
    pub /: *const *const *const hc_driver driver; / hw-specific hooks,
//
// OTG and some Host controllers need software interaction with phys;
// other external phys should be software-transparent
//
    pub usb_phy: *mut usb_phy,
    pub phy_roothub: *mut usb_phy_roothub,
// Flags that need to be manipulated atomically because they can
// change while the host controller is running.  Always use
// set_bit() or clear_bit() to change their values.
//
    pub flags: c_ulong,

// The flags can be tested using these macros; they are likely to
// be slightly faster than test_bit().
//

//
// Specifies if interfaces are authorized by default
// or they require explicit user space authorization; this bit is
// settable through /sys/class/usb_host/X/interface_authorized_default
//

//
// Specifies if devices are authorized by default
// or they require explicit user space authorization; this bit is
// settable through /sys/class/usb_host/X/authorized_default
//
    pub dev_policy: usb_dev_authorize_policy,
// Flags that get set only during HCD registration or removal.
    pub /: *mut *mut unsigned rh_registered:1;/ is root hub registered?,
    pub /: *mut *mut unsigned rh_pollable:1; / may we poll the root hub?,
    pub /: *mut *mut unsigned msix_enabled:1; / driver has MSI-X enabled?,
    pub /: *mut *mut unsigned msi_enabled:1; / driver has MSI enabled?,
//
// do not manage the PHY state in the HCD core, instead let the driver
// handle this (for example if the PHY can only be turned on after a
// specific event)
//
    pub skip_phy_initialization:1: unsigned,
// The next flag is a stopgap, to be removed when all the HCDs
// support the new root-hub polling mechanism.
    pub uses_new_polling:1: unsigned,
    pub /: *mut *mut unsigned has_tt:1; / Integrated TT in root hub,
    pub /: *mut *mut unsigned amd_resume_bug:1; / AMD remote wakeup quirk,
    pub /: *mut *mut unsigned can_do_streams:1; / HC supports streams,
    pub /: *mut *mut unsigned tpl_support:1; / OTG & EH TPL support,
    pub cant_recv_wakeups:1: unsigned,
// wakeup requests from downstream aren't received
    pub /: *mut *mut unsigned int irq; / irq allocated,
    pub /: *mut *mut *mut void __iomem regs; / device memory/io,
    pub /: *mut *mut resource_size_t rsrc_start; / memory/io resource start,
    pub /: *mut *mut resource_size_t rsrc_len; / memory/io resource length,
    pub /: *mut *mut unsigned power_budget; / in mA, 0 = no limit,
    pub high_prio_bh: giveback_urb_bh,
    pub low_prio_bh: giveback_urb_bh,
// bandwidth_mutex should be taken before adding or removing
// any new bus bandwidth constraints:
// 1. Before adding a configuration for a new device.
// 2. Before removing the configuration to put the device into
// the addressed state.
// 3. Before selecting a different configuration.
// 4. Before selecting an alternate interface setting.
//
// bandwidth_mutex should be dropped after a successful control message
// to the device, or resetting the bandwidth after a failed attempt.
//
    pub address0_mutex: *mut mutex,
    pub bandwidth_mutex: *mut mutex,
    pub shared_hcd: *mut usb_hcd,
    pub primary_hcd: *mut usb_hcd,
pub const HCD_BUFFER_POOLS: c_int = 4;
    pub pool: [*mut dma_pool; HCD_BUFFER_POOLS],
    pub state: c_int,

// memory pool for HCs having local memory, or %NULL
    pub localmem_pool: *mut gen_pool,
// more shared queuing code would be good; it should support
// smarter scheduling, handle transaction translators, etc;
// input size of periodic table to an interrupt scheduler.
// (ohci 32, uhci 1024, ehci 256/512/1024).
//
// The HC driver's private data is stored at the end of
// this structure.
//
// C attribute field omitted
}

// 2.4 does this a bit differently ...
extern "C" {
    pub fn container_of(_arg: bus, usb_hcd: struct, _arg: self) -> return;
}
// -------------------------------------------------------------------------
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hc_driver {
    pub /: *const *const *const char description; / "ehci-hcd" etc,
    pub /: *const *const *const char product_desc; / product/vendor string,
    pub /: *mut *mut size_t hcd_priv_size; / size of private data,
// irq handler
    pub hcd): *mut *mut irqreturn_t (irq) (struct usb_hcd,
    pub flags: c_int,
pub const HCD_MEMORY: c_uint = 0x0001		/* HC regs use memory (else I/O) */;
pub const HCD_DMA: c_uint = 0x0002		/* HC uses DMA */;
pub const HCD_SHARED: c_uint = 0x0004		/* Two (or more) usb_hcds share HW */;
pub const HCD_USB11: c_uint = 0x0010		/* USB 1.1 */;
pub const HCD_USB2: c_uint = 0x0020		/* USB 2.0 */;
pub const HCD_USB3: c_uint = 0x0040		/* USB 3.0 */;
pub const HCD_USB31: c_uint = 0x0050		/* USB 3.1 */;
pub const HCD_USB32: c_uint = 0x0060		/* USB 3.2 */;
pub const HCD_MASK: c_uint = 0x0070;
pub const HCD_BH: c_uint = 0x0100		/* URB complete in BH context */;
// called to init HCD and root hub
    pub hcd): *mut *mut int (reset) (struct usb_hcd,
    pub hcd): *mut *mut int (start) (struct usb_hcd,
// NOTE:  these suspend/resume calls relate to the HC as
// a whole, not just the root hub; they're for PCI bus glue.
//
// called after suspending the hub, before entering D3 etc
    pub do_wakeup): *mut *mut *mut int (pci_suspend)(struct usb_hcd hcd, bool,
// called after entering D0 (etc), before resuming the hub
    pub state): *mut *mut *mut int (pci_resume)(struct usb_hcd hcd, pm_message_t,
// called just before hibernate final D3 state, allows host to poweroff parts
    pub do_wakeup): *mut *mut *mut int (pci_poweroff_late)(struct usb_hcd hcd, bool,
// cleanly make HCD stop writing memory and doing I/O
    pub hcd): *mut *mut void (stop) (struct usb_hcd,
// shutdown HCD
    pub hcd): *mut *mut void (shutdown) (struct usb_hcd,
// return current frame number
    pub hcd): *mut *mut int (get_frame_number) (struct usb_hcd,
// manage i/o requests, device state
    pub mem_flags): *mut *mut urb urb, gfp_t,
    pub status): *mut *mut urb urb, int,
//
// (optional) these hooks allow an HCD to override the default DMA
// mapping and unmapping routines.  In general, they shouldn't be
// necessary unless the host controller has special DMA requirements,
// such as alignment constraints.  If these are not specified, the
// general usb_hcd_(un)?map_urb_for_dma functions will be used instead
// (and it may be a good idea to call these functions in your HCD
// implementation)
//
    pub mem_flags): gfp_t,
    pub urb): *mut *mut *mut void (unmap_urb_for_dma)(struct usb_hcd hcd, struct urb,
// hw synch, freeing endpoint resources that urb_dequeue can't
    pub ep): *mut usb_host_endpoint,
// (optional) reset any endpoint state such as sequence number
    pub ep): *mut usb_host_endpoint,
// root hub support
    pub buf): *mut *mut *mut int (hub_status_data) (struct usb_hcd hcd, char,
    pub wLength): *mut *mut char buf, u16,
    pub ): *mut *mut int (bus_suspend)(struct usb_hcd,
    pub ): *mut *mut int (bus_resume)(struct usb_hcd,
    pub port_num): *mut *mut *mut int (start_port_reset)(struct usb_hcd , unsigned,
    pub ): *mut *mut unsigned long (get_resuming_ports)(struct usb_hcd,
// force handover of high-speed port to full-speed companion
    pub int): *mut *mut *mut void (relinquish_port)(struct usb_hcd ,,
// has a port been handed over to a companion?
    pub int): *mut *mut *mut int (port_handed_over)(struct usb_hcd ,,
// CLEAR_TT_BUFFER completion callback
    pub ): *mut usb_host_endpoint,
// xHCI specific functions
// Called by usb_alloc_dev to alloc HC device structures
    pub ): *mut *mut *mut int (alloc_dev)(struct usb_hcd , struct usb_device,
// Called by usb_disconnect to free HC device structures
    pub ): *mut *mut *mut void (free_dev)(struct usb_hcd , struct usb_device,
// Change a group of bulk endpoints to support multiple stream IDs
    pub mem_flags): unsigned int num_streams, gfp_t,
// Reverts a group of bulk endpoints back to not using stream IDs.
// Can fail if we run out of memory.
//
    pub mem_flags): gfp_t,
// Bandwidth computation functions
// Note that add_endpoint() can only be called once per endpoint before
// check_bandwidth() or reset_bandwidth() must be called.
// drop_endpoint() can only be called once per endpoint also.
// A call to xhci_drop_endpoint() followed by a call to
// xhci_add_endpoint() will add the endpoint to the schedule with
// possibly new parameters denoted by a different endpoint descriptor
// in usb_host_endpoint.  A call to xhci_add_endpoint() followed by a
// call to xhci_drop_endpoint() is not allowed.
//
// Allocate endpoint resources and add them to a new schedule
    pub ): *mut usb_host_endpoint,
// Drop an endpoint from a new schedule
    pub ): *mut usb_host_endpoint,
// Check that a new hardware configuration, set using
// endpoint_enable and endpoint_disable, does not exceed bus
// bandwidth.  This must be called before any set configuration
// or set interface requests are sent to the device.
//
    pub ): *mut *mut *mut int (check_bandwidth)(struct usb_hcd , struct usb_device,
// Reset the device schedule to the last known good schedule,
// which was set from a previous successful call to
// check_bandwidth().  This reverts any add_endpoint() and
// drop_endpoint() calls since that last successful call.
// Used for when a check_bandwidth() call fails due to resource
// or bandwidth constraints.
//
    pub ): *mut *mut *mut void (reset_bandwidth)(struct usb_hcd , struct usb_device,
// Set the hardware-chosen device address
    pub timeout_ms): c_uint,
// prepares the hardware to send commands to the device
    pub udev): *mut *mut *mut int (enable_device)(struct usb_hcd , struct usb_device,
// Notifies the HCD after a hub descriptor is fetched.
// Will block.
//
    pub mem_flags): *mut *mut usb_tt tt, gfp_t,
    pub ): *mut *mut *mut int (reset_device)(struct usb_hcd , struct usb_device,
// Notifies the HCD after a device is connected and its
// address is set
//
    pub ): *mut *mut *mut int (update_device)(struct usb_hcd , struct usb_device,
    pub int): *mut *mut *mut *mut int (set_usb2_hw_lpm)(struct usb_hcd , struct usb_device ,,
// USB 3.0 Link Power Management
// Returns the USB3 hub-encoded value for the U1/U2 timeout.
    pub state): *mut *mut usb_device , enum usb3_link_state,
// The xHCI host controller can still fail the command to
// disable the LPM timeouts, so this can return an error code.
//
    pub state): *mut *mut usb_device , enum usb3_link_state,
    pub int): *mut *mut *mut int (find_raw_port_number)(struct usb_hcd ,,
// Call for power on/off the port if necessary
    pub enable): *mut *mut *mut int (port_power)(struct usb_hcd hcd, int portnum, bool,
// Call for SINGLE_STEP_SET_FEATURE Test for USB2 EH certification
pub const EHSET_TEST_SINGLE_STEP_SET_FEATURE: c_uint = 0x06;
    pub int): *mut *mut urb ,,
}

extern "C" {
    pub fn IS_ENABLED(HCD_DMA: CONFIG_HAS_DMA) && (hcd->driver->flags &) -> return;
}
extern "C" {
    pub fn usb_hcd_link_urb_to_ep(hcd: *mut usb_hcd, urb: *mut urb) -> c_int;
}
extern "C" {
    pub fn usb_hcd_unlink_urb_from_ep(hcd: *mut usb_hcd, urb: *mut urb);
}
extern "C" {
    pub fn usb_hcd_submit_urb(urb: *mut urb, mem_flags: gfp_t) -> c_int;
}
extern "C" {
    pub fn usb_hcd_unlink_urb(urb: *mut urb, status: c_int) -> c_int;
}
extern "C" {
    pub fn usb_hcd_unmap_urb_setup_for_dma(: *mut usb_hcd, : *mut urb);
}
extern "C" {
    pub fn usb_hcd_unmap_urb_for_dma(: *mut usb_hcd, : *mut urb);
}
extern "C" {
    pub fn usb_hcd_synchronize_unlinks(udev: *mut usb_device);
}
extern "C" {
    pub fn usb_hcd_get_frame_number(udev: *mut usb_device) -> c_int;
}
extern "C" {
    pub fn usb_put_hcd(hcd: *mut usb_hcd);
}
extern "C" {
    pub fn usb_hcd_is_primary_hcd(hcd: *mut usb_hcd) -> c_int;
}
extern "C" {
    pub fn usb_remove_hcd(hcd: *mut usb_hcd);
}
extern "C" {
    pub fn usb_hcd_find_raw_port_number(hcd: *mut usb_hcd, port1: c_int) -> c_int;
}
extern "C" {
    pub fn usb_hcd_platform_shutdown(dev: *mut platform_device);
}

extern "C" {
    pub fn ehset_single_step_set_feature(hcd: *mut usb_hcd, port: c_int) -> c_int;
}

extern "C" {
    pub fn usb_hcd_pci_remove(dev: *mut pci_dev);
}
extern "C" {
    pub fn usb_hcd_pci_shutdown(dev: *mut pci_dev);
}

extern "C" {
    pub fn usb_hcd_amd_remote_wakeup_quirk(dev: *mut pci_dev) -> c_int;
}

// pci-ish (pdev null is ok) buffer alloc/mapping support
extern "C" {
    pub fn usb_init_pool_max();
}
extern "C" {
    pub fn hcd_buffer_create(hcd: *mut usb_hcd) -> c_int;
}
extern "C" {
    pub fn hcd_buffer_destroy(hcd: *mut usb_hcd);
}
// generic bus glue, needed for host controllers that don't use PCI
extern "C" {
    pub fn usb_hcd_irq(irq: c_int, __hcd: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn usb_hc_died(hcd: *mut usb_hcd);
}
extern "C" {
    pub fn usb_hcd_poll_rh_status(hcd: *mut usb_hcd);
}
extern "C" {
    pub fn usb_hcd_start_port_resume(bus: *mut usb_bus, portnum: c_int);
}
extern "C" {
    pub fn usb_hcd_end_port_resume(bus: *mut usb_bus, portnum: c_int);
}
// The D0/D1 toggle bits ... USE WITH CAUTION (they're almost hcd-internal)

// --------------------------------------------------------------------------
// Enumeration is only for the hub driver, or HCD virtual root hubs
extern "C" {
    pub fn usb_new_device(dev: *mut usb_device) -> c_int;
}
extern "C" {
    pub fn usb_disconnect(: *mut usb_device);
}
extern "C" {
    pub fn usb_get_configuration(dev: *mut usb_device) -> c_int;
}
extern "C" {
    pub fn usb_destroy_configuration(dev: *mut usb_device);
}
// -------------------------------------------------------------------------
//
// HCD Root Hub support
//

//
// As of USB 2.0, full/low speed devices are segregated into trees.
// One type grows from USB 1.1 host controllers (OHCI, UHCI etc).
// The other type grows from high speed hubs when they connect to
// full/low speed devices using "Transaction Translators" (TTs).
//
// TTs should only be known to the hub driver, and high speed bus
// drivers (only EHCI for now).  They affect periodic scheduling and
// sometimes control/bulk error recovery.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_tt {
    pub /: *mut *mut *mut usb_device hub; / upstream highspeed hub,
    pub /: *mut *mut int multi; / true means one TT per port,
    pub /: *mut *mut unsigned think_time; / think time in ns,
    pub /: *mut *mut *mut void hcpriv; / HCD private data,
// for control/bulk error recovery (CLEAR_TT_BUFFER)
    pub lock: spinlock_t,
    pub /: *mut *mut list_head clear_list; / of usb_tt_clear,
    pub clear_work: work_struct,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_tt_clear {
    pub clear_list: list_head,
    pub tt: unsigned,
    pub devinfo: u16,
    pub hcd: *mut usb_hcd,
    pub ep: *mut usb_host_endpoint,
}

extern "C" {
    pub fn usb_hub_clear_tt_buffer(urb: *mut urb) -> c_int;
}
extern "C" {
    pub fn usb_ep0_reinit(: *mut usb_device);
}
// (shifted) direction/type/recipient from the USB 2.0 spec, table 9.2

// class requests from the USB 2.0 hub spec, table 11-15

// GetBusState and SetHubDescriptor are optional, omitted

// -------------------------------------------------------------------------
// class requests from USB 3.1 hub spec, table 10-7

//
// Generic bandwidth allocation constants/support
//

// Trying not to use worst-case bit-stuffing
// of (7/6 * 8 * bytecount) = 9.33 * bytecount
// bytecount = data payload byte count

// convert nanoseconds to microseconds, rounding up
//
// Full/low speed bandwidth allocation constants/support.
//

// 4 full-speed bit times (est.)

//
// Ceiling [nano/micro]seconds (typical) for that many bytes at high speed
// ISO is a bit less, no ACK ... from USB 2.0 spec, 5.11.3 (and needed
// to preallocate bandwidth)
//

// -------------------------------------------------------------------------
// exported only within usbcore

extern "C" {
    pub fn usb_wakeup_enabled_descendants(udev: *mut usb_device) -> unsigned;
}
extern "C" {
    pub fn usb_root_hub_lost_power(rhdev: *mut usb_device);
}
extern "C" {
    pub fn hcd_bus_suspend(rhdev: *mut usb_device, msg: pm_message_t) -> c_int;
}
extern "C" {
    pub fn hcd_bus_resume(rhdev: *mut usb_device, msg: pm_message_t) -> c_int;
}
extern "C" {
    pub fn usb_hcd_resume_root_hub(hcd: *mut usb_hcd);
}

// -------------------------------------------------------------------------

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_mon_operations {
    pub urb): *mut *mut *mut void (urb_submit)(struct usb_bus bus, struct urb,
    pub err): *mut *mut *mut *mut void (urb_submit_error)(struct usb_bus bus, struct urb urb, int,
    pub status): *mut *mut *mut *mut void (urb_complete)(struct usb_bus bus, struct urb urb, int,
// void (*urb_unlink)(struct usb_bus *bus, struct urb *urb);
}

extern "C" {
    pub fn usb_mon_register(ops: *const usb_mon_operations) -> c_int;
}
extern "C" {
    pub fn usb_mon_deregister();
}

// -------------------------------------------------------------------------
// random stuff
// This rwsem is for use only by the hub driver and ehci-hcd.
// Nobody else should touch it.
//

