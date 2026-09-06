//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/usb/gadget.h
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
// <linux/usb/gadget.h>
//
// We call the USB code inside a Linux-based peripheral device a "gadget"
// driver, except for the hardware-specific bus glue.  One USB host can
// talk to many USB gadgets, but the gadgets are only able to communicate
// to one host.
//
// (C) Copyright 2002-2004 by David Brownell
// All Rights Reserved.
//

pub const UDC_TRACE_STR_MAX: c_int = 512;
//
// struct usb_request - describes one i/o request
// @ep: The associated endpoint set by usb_ep_alloc_request().
// @buf: Buffer used for data.  Always provide this; some controllers
// only use PIO, or don't use DMA for some endpoints.
// @dma: DMA address corresponding to 'buf'.  If you don't set this
// field, and the usb controller needs one, it is responsible
// for mapping and unmapping the buffer.
// @sg: a scatterlist for SG-capable controllers.
// @num_sgs: number of SG entries
// @num_mapped_sgs: number of SG entries mapped to DMA (internal)
// @length: Length of that data
// @stream_id: The stream id, when USB3.0 bulk streams are being used
// @is_last: Indicates if this is the last request of a stream_id before
// switching to a different stream (required for DWC3 controllers).
// @no_interrupt: If true, hints that no completion irq is needed.
// Helpful sometimes with deep request queues that are handled
// directly by DMA controllers.
// @zero: If true, when writing data, makes the last packet be "short"
// by adding a zero length packet as needed;
// @short_not_ok: When reading data, makes short packets be
// treated as errors (queue stops advancing till cleanup).
// @dma_mapped: Indicates if request has been mapped to DMA (internal)
// @sg_was_mapped: Set if the scatterlist has been mapped before the request
// @complete: Function called when request completes, so this request and
// its buffer may be re-used.  The function will always be called with
// interrupts disabled, and it must not sleep.
// Reads terminate with a short packet, or when the buffer fills,
// whichever comes first.  When writes terminate, some data bytes
// will usually still be in flight (often in a hardware fifo).
// Errors (for reads or writes) stop the queue from advancing
// until the completion function returns, so that any transfers
// invalidated by the error may first be dequeued.
// @context: For use by the completion callback
// @list: For use by the gadget driver.
// @frame_number: Reports the interval number in (micro)frame in which the
// isochronous transfer was transmitted or received.
// @status: Reports completion code, zero or a negative errno.
// Normally, faults block the transfer queue from advancing until
// the completion callback returns.
// Code "-ESHUTDOWN" indicates completion caused by device disconnect,
// or when the driver disabled the endpoint.
// @actual: Reports bytes transferred to/from the buffer.  For reads (OUT
// transfers) this may be less than the requested length.  If the
// short_not_ok flag is set, short reads are treated as errors
// even when status otherwise indicates successful completion.
// Note that for writes (IN transfers) some data bytes may still
// reside in a device-side FIFO when the request is reported as
// complete.
//
// These are allocated/freed through the endpoint they're used with.  The
// hardware's driver can add extra per-request data to the memory it returns,
// which often avoids separate memory allocations (potential failures),
// later when the request is queued.
//
// Request flags affect request handling, such as whether a zero length
// packet is written (the "zero" flag), whether a short read should be
// treated as an error (blocking request queue advance, the "short_not_ok"
// flag), or hinting that an interrupt is not required (the "no_interrupt"
// flag, for use with deep request queues).
//
// Bulk endpoints can use any size buffers, and can also be used for interrupt
// transfers. interrupt-only endpoints can be much less functional.
//
// NOTE:  this is analogous to 'struct urb' on the host side, except that
// it's thinner and promotes more pre-allocation.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_request {
    pub ep: *mut usb_ep,
    pub buf: *mut c_void,
    pub length: unsigned,
    pub dma: dma_addr_t,
    pub sg: *mut scatterlist,
    pub num_sgs: unsigned,
    pub num_mapped_sgs: unsigned,
    pub stream_id:16: unsigned,
    pub is_last:1: unsigned,
    pub no_interrupt:1: unsigned,
    pub zero:1: unsigned,
    pub short_not_ok:1: unsigned,
    pub dma_mapped:1: unsigned,
    pub sg_was_mapped:1: unsigned,
    pub req): *mut usb_request,
    pub context: *mut c_void,
    pub list: list_head,
    pub /: *mut *mut unsigned frame_number; / ISO ONLY,
    pub status: c_int,
    pub actual: unsigned,
}

// -------------------------------------------------------------------------
// endpoint-specific parts of the api to the usb controller hardware.
// unlike the urb model, (de)multiplexing layers are not required.
// (so this api could slash overhead if used on the host side...)
//
// note that device side usb controllers commonly differ in how many
// endpoints they support, as well as their capabilities.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_ep_ops {
    pub desc): *const usb_endpoint_descriptor,
    pub ep): *mut *mut int (disable) (struct usb_ep,
    pub ep): *mut *mut void (dispose) (struct usb_ep,
    pub gfp_flags): gfp_t,
    pub req): *mut *mut *mut void (free_request) (struct usb_ep ep, struct usb_request,
    pub gfp_flags): gfp_t,
    pub req): *mut *mut *mut int (dequeue) (struct usb_ep ep, struct usb_request,
    pub value): *mut *mut *mut int (set_halt) (struct usb_ep ep, int,
    pub ep): *mut *mut int (set_wedge) (struct usb_ep,
    pub ep): *mut *mut int (fifo_status) (struct usb_ep,
    pub ep): *mut *mut void (fifo_flush) (struct usb_ep,
}

//
// struct usb_ep_caps - endpoint capabilities description
// @type_control:Endpoint supports control type (reserved for ep0).
// @type_iso:Endpoint supports isochronous transfers.
// @type_bulk:Endpoint supports bulk transfers.
// @type_int:Endpoint supports interrupt transfers.
// @dir_in:Endpoint supports IN direction.
// @dir_out:Endpoint supports OUT direction.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_ep_caps {
    pub type_control:1: unsigned,
    pub type_iso:1: unsigned,
    pub type_bulk:1: unsigned,
    pub type_int:1: unsigned,
    pub dir_in:1: unsigned,
    pub dir_out:1: unsigned,
}

pub const USB_EP_CAPS_TYPE_CONTROL: c_uint = 0x01;
pub const USB_EP_CAPS_TYPE_ISO: c_uint = 0x02;
pub const USB_EP_CAPS_TYPE_BULK: c_uint = 0x04;
pub const USB_EP_CAPS_TYPE_INT: c_uint = 0x08;

pub const USB_EP_CAPS_DIR_IN: c_uint = 0x01;
pub const USB_EP_CAPS_DIR_OUT: c_uint = 0x02;

//
// struct usb_ep - device side representation of USB endpoint
// @name:identifier for the endpoint, such as "ep-a" or "ep9in-bulk"
// @ops: Function pointers used to access hardware-specific operations.
// @ep_list:the gadget's ep_list holds all of its endpoints
// @caps:The structure describing types and directions supported by endpoint.
// @enabled: The current endpoint enabled/disabled state.
// @claimed: True if this endpoint is claimed by a function.
// @maxpacket:The maximum packet size used on this endpoint.  The initial
// value can sometimes be reduced (hardware allowing), according to
// the endpoint descriptor used to configure the endpoint.
// @maxpacket_limit:The maximum packet size value which can be handled by this
// endpoint. It's set once by UDC driver when endpoint is initialized, and
// should not be changed. Should not be confused with maxpacket.
// @max_streams: The maximum number of streams supported
// by this EP (0 - 16, actual number is 2^n)
// @mult: multiplier, 'mult' value for SS Isoc EPs
// @maxburst: the maximum number of bursts supported by this EP (for usb3)
// @driver_data:for use by the gadget driver.
// @address: used to identify the endpoint when finding descriptor that
// matches connection speed
// @desc: endpoint descriptor.  This pointer is set before the endpoint is
// enabled and remains valid until the endpoint is disabled.
// @comp_desc: In case of SuperSpeed support, this is the endpoint companion
// descriptor that is used to configure the endpoint
//
// the bus controller driver lists all the general purpose endpoints in
// gadget->ep_list.  the control endpoint (gadget->ep0) is not in that list,
// and is accessed only in response to a driver setup() callback.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_ep {
    pub driver_data: *mut c_void,
    pub name: *const c_char,
    pub ops: *const usb_ep_ops,
    pub desc: *const usb_endpoint_descriptor,
    pub comp_desc: *const usb_ss_ep_comp_descriptor,
    pub ep_list: list_head,
    pub caps: usb_ep_caps,
    pub claimed: bool,
    pub enabled: bool,
    pub mult:2: unsigned,
    pub maxburst:5: unsigned,
    pub address: u8,
    pub maxpacket: u16,
    pub maxpacket_limit: u16,
    pub max_streams: u16,
}

// -------------------------------------------------------------------------

extern "C" {
    pub fn usb_ep_set_maxpacket_limit(ep: *mut usb_ep, maxpacket_limit: unsigned);
}
extern "C" {
    pub fn usb_ep_enable(ep: *mut usb_ep) -> c_int;
}
extern "C" {
    pub fn usb_ep_disable(ep: *mut usb_ep) -> c_int;
}
extern "C" {
    pub fn usb_ep_free_request(ep: *mut usb_ep, req: *mut usb_request);
}
extern "C" {
    pub fn usb_ep_queue(ep: *mut usb_ep, req: *mut usb_request, gfp_flags: gfp_t) -> c_int;
}
extern "C" {
    pub fn usb_ep_dequeue(ep: *mut usb_ep, req: *mut usb_request) -> c_int;
}
extern "C" {
    pub fn usb_ep_set_halt(ep: *mut usb_ep) -> c_int;
}
extern "C" {
    pub fn usb_ep_clear_halt(ep: *mut usb_ep) -> c_int;
}
extern "C" {
    pub fn usb_ep_set_wedge(ep: *mut usb_ep) -> c_int;
}
extern "C" {
    pub fn usb_ep_fifo_status(ep: *mut usb_ep) -> c_int;
}
extern "C" {
    pub fn usb_ep_fifo_flush(ep: *mut usb_ep);
}

// -------------------------------------------------------------------------
//
// free_usb_request - frees a usb_request object and its buffer
// @req: the request being freed
//
// This helper function frees both the request's buffer and the request object
// itself by calling usb_ep_free_request(). Its signature is designed to be used
// with DEFINE_FREE() to enable automatic, scope-based cleanup for usb_request
// pointers.
//
// -------------------------------------------------------------------------
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_dcd_config_params {
    pub /: *mut *mut __u8 bU1devExitLat; / U1 Device exit Latency,
pub const USB_DEFAULT_U1_DEV_EXIT_LAT: c_uint = 0x01	/* Less then 1 microsec */;
    pub /: *mut *mut __le16 bU2DevExitLat; / U2 Device exit Latency,
pub const USB_DEFAULT_U2_DEV_EXIT_LAT: c_uint = 0x1F4	/* Less then 500 microsec */;
    pub /: *mut *mut __u8 besl_baseline; / Recommended baseline BESL (0-15),
    pub /: *mut *mut __u8 besl_deep; / Recommended deep BESL (0-15),
pub const USB_DEFAULT_BESL_UNSPECIFIED: c_uint = 0xFF	/* No recommended value */;
}

// the rest of the api to the controller hardware: device operations,
// which don't involve endpoints (or i/o).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_gadget_ops {
    pub ): *mut *mut int (get_frame)(struct usb_gadget,
    pub ): *mut *mut int (wakeup)(struct usb_gadget,
    pub intf_id): *mut *mut *mut int (func_wakeup)(struct usb_gadget gadget, int,
    pub set): *mut *mut *mut int (set_remote_wakeup)(struct usb_gadget , int,
    pub is_selfpowered): *mut *mut *mut int (set_selfpowered) (struct usb_gadget , int,
    pub is_active): *mut *mut *mut int (vbus_session) (struct usb_gadget , int,
    pub mA): *mut *mut *mut int (vbus_draw) (struct usb_gadget , unsigned,
    pub is_on): *mut *mut *mut int (pullup) (struct usb_gadget , int,
    pub param): unsigned code, unsigned long,
    pub ): *mut usb_dcd_config_params,
    pub ): *mut usb_gadget_driver,
    pub ): *mut *mut int (udc_stop)(struct usb_gadget,
    pub usb_device_speed): *mut *mut *mut void (udc_set_speed)(struct usb_gadget , enum,
    pub rate): usb_ssp_rate,
    pub enable): *mut *mut *mut void (udc_async_callbacks)(struct usb_gadget gadget, bool,
    pub ): *mut usb_ss_ep_comp_descriptor,
    pub gadget): *mut *mut int (check_config)(struct usb_gadget,
}

//
// struct usb_gadget - represents a usb device
// @work: (internal use) Workqueue to be used for sysfs_notify()
// @udc: struct usb_udc pointer for this gadget
// @ops: Function pointers used to access hardware-specific operations.
// @ep0: Endpoint zero, used when reading or writing responses to
// driver setup() requests
// @ep_list: List of other endpoints supported by the device.
// @speed: Speed of current connection to USB host.
// @max_speed: Maximal speed the UDC can handle.  UDC must support this
// and all slower speeds.
// @ssp_rate: Current connected SuperSpeed Plus signaling rate and lane count.
// @max_ssp_rate: Maximum SuperSpeed Plus signaling rate and lane count the UDC
// can handle. The UDC must support this and all slower speeds and lower
// number of lanes.
// @state: the state we are now (attached, suspended, configured, etc)
// @state_lock: Spinlock protecting the `state` and `teardown` members.
// @teardown: True if the device is undergoing teardown, used to prevent
// new work from being scheduled during cleanup.
// @name: Identifies the controller hardware type.  Used in diagnostics
// and sometimes configuration.
// @dev: Driver model state for this abstract device.
// @isoch_delay: value from Set Isoch Delay request. Only valid on SS/SSP
// @out_epnum: last used out ep number
// @in_epnum: last used in ep number
// @mA: last set mA value
// @otg_caps: OTG capabilities of this gadget.
// @sg_supported: true if we can handle scatter-gather
// @is_otg: True if the USB device port uses a Mini-AB jack, so that the
// gadget driver must provide a USB OTG descriptor.
// @is_a_peripheral: False unless is_otg, the "A" end of a USB cable
// is in the Mini-AB jack, and HNP has been used to switch roles
// so that the "A" device currently acts as A-Peripheral, not A-Host.
// @a_hnp_support: OTG device feature flag, indicating that the A-Host
// supports HNP at this port.
// @a_alt_hnp_support: OTG device feature flag, indicating that the A-Host
// only supports HNP on a different root port.
// @b_hnp_enable: OTG device feature flag, indicating that the A-Host
// enabled HNP support.
// @hnp_polling_support: OTG device feature flag, indicating if the OTG device
// in peripheral mode can support HNP polling.
// @host_request_flag: OTG device feature flag, indicating if A-Peripheral
// or B-Peripheral wants to take host role.
// @quirk_ep_out_aligned_size: epout requires buffer size to be aligned to
// MaxPacketSize.
// @quirk_altset_not_supp: UDC controller doesn't support alt settings.
// @quirk_stall_not_supp: UDC controller doesn't support stalling.
// @quirk_zlp_not_supp: UDC controller doesn't support ZLP.
// @quirk_avoids_skb_reserve: udc/platform wants to avoid skb_reserve() in
// u_ether.c to improve performance.
// @is_selfpowered: if the gadget is self-powered.
// @deactivated: True if gadget is deactivated - in deactivated state it cannot
// be connected.
// @connected: True if gadget is connected.
// @lpm_capable: If the gadget max_speed is FULL or HIGH, this flag
// indicates that it supports LPM as per the LPM ECN & errata.
// @wakeup_capable: True if gadget is capable of sending remote wakeup.
// @wakeup_armed: True if gadget is armed by the host for remote wakeup.
// @irq: the interrupt number for device controller.
// @id_number: a unique ID number for ensuring that gadget names are distinct
//
// Gadgets have a mostly-portable "gadget driver" implementing device
// functions, handling all usb configurations and interfaces.  Gadget
// drivers talk to hardware-specific code indirectly, through ops vectors.
// That insulates the gadget driver from hardware details, and packages
// the hardware endpoints through generic i/o queues.  The "usb_gadget"
// and "usb_ep" interfaces provide that insulation from the hardware.
//
// Except for the driver data, all fields in this structure are
// read-only to the gadget driver.  That driver data is part of the
// "driver model" infrastructure in 2.6 (and later) kernels, and for
// earlier systems is grouped in a similar structure that's not known
// to the rest of the kernel.
//
// Values of the three OTG device feature flags are updated before the
// setup() call corresponding to USB_REQ_SET_CONFIGURATION, and before
// driver suspend() calls.  They are valid only when is_otg, and when the
// device is acting as a B-Peripheral (so is_a_peripheral is false).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_gadget {
    pub work: work_struct,
    pub udc: *mut usb_udc,
// readonly to gadget driver
    pub ops: *const usb_gadget_ops,
    pub ep0: *mut usb_ep,
    pub /: *mut *mut list_head ep_list; / of usb_ep,
    pub speed: usb_device_speed,
    pub max_speed: usb_device_speed,
// USB SuperSpeed Plus only
    pub ssp_rate: usb_ssp_rate,
    pub max_ssp_rate: usb_ssp_rate,
    pub state: usb_device_state,
    pub state_lock: spinlock_t,
    pub teardown: bool,
    pub name: *const c_char,
    pub dev: device,
    pub isoch_delay: unsigned,
    pub out_epnum: unsigned,
    pub in_epnum: unsigned,
    pub mA: unsigned,
    pub otg_caps: *mut usb_otg_caps,
    pub sg_supported:1: unsigned,
    pub is_otg:1: unsigned,
    pub is_a_peripheral:1: unsigned,
    pub b_hnp_enable:1: unsigned,
    pub a_hnp_support:1: unsigned,
    pub a_alt_hnp_support:1: unsigned,
    pub hnp_polling_support:1: unsigned,
    pub host_request_flag:1: unsigned,
    pub quirk_ep_out_aligned_size:1: unsigned,
    pub quirk_altset_not_supp:1: unsigned,
    pub quirk_stall_not_supp:1: unsigned,
    pub quirk_zlp_not_supp:1: unsigned,
    pub quirk_avoids_skb_reserve:1: unsigned,
    pub is_selfpowered:1: unsigned,
    pub deactivated:1: unsigned,
    pub connected:1: unsigned,
    pub lpm_capable:1: unsigned,
    pub wakeup_capable:1: unsigned,
    pub wakeup_armed:1: unsigned,
    pub irq: c_int,
    pub id_number: c_int,
}

// Interface to the device model
extern "C" {
    pub fn container_of(_arg: dev, usb_gadget: struct, _arg: dev) -> return;
}
extern "C" {
    pub fn usb_add_gadget(gadget: *mut usb_gadget) -> c_int;
}
extern "C" {
    pub fn usb_del_gadget(gadget: *mut usb_gadget);
}
// Legacy device-model interface
extern "C" {
    pub fn usb_add_gadget_udc(parent: *mut device, gadget: *mut usb_gadget) -> c_int;
}
extern "C" {
    pub fn usb_del_gadget_udc(gadget: *mut usb_gadget);
}
// iterates the non-control endpoints; 'tmp' is a struct usb_ep pointer

//
// usb_ep_align - returns @len aligned to ep's maxpacketsize.
// @ep: the endpoint whose maxpacketsize is used to align @len
// @len: buffer size's length to align to @ep's maxpacketsize
//
// This helper is used to align buffer's size to an ep's maxpacketsize.
//
extern "C" {
    pub fn round_up(_arg: len, _arg: max_packet_size) -> return;
}
//
// usb_ep_align_maybe - returns @len aligned to ep's maxpacketsize if gadget
// requires quirk_ep_out_aligned_size, otherwise returns len.
// @g: controller to check for quirk
// @ep: the endpoint whose maxpacketsize is used to align @len
// @len: buffer size's length to align to @ep's maxpacketsize
//
// This helper is used in case it's required for any reason to check and maybe
// align buffer's size to an ep's maxpacketsize.
//
// gadget_is_altset_supported - return true iff the hardware supports
// altsettings
// @g: controller to check for quirk
//
// gadget_is_stall_supported - return true iff the hardware supports stalling
// @g: controller to check for quirk
//
// gadget_is_zlp_supported - return true iff the hardware supports zlp
// @g: controller to check for quirk
//
// gadget_avoids_skb_reserve - return true iff the hardware would like to avoid
// skb_reserve to improve performance.
// @g: controller to check for quirk
//
// gadget_is_dualspeed - return true iff the hardware handles high speed
// @g: controller that might support both high and full speeds
//
// gadget_is_superspeed() - return true if the hardware handles superspeed
// @g: controller that might support superspeed
//
// gadget_is_superspeed_plus() - return true if the hardware handles
// superspeed plus
// @g: controller that might support superspeed plus
//
// gadget_is_otg - return true iff the hardware is OTG-ready
// @g: controller that might have a Mini-AB connector
//
// This is a runtime test, since kernels with a USB-OTG stack sometimes
// run on boards which only have a Mini-B (or Mini-A) connector.
//

// -------------------------------------------------------------------------

extern "C" {
    pub fn usb_gadget_frame_number(gadget: *mut usb_gadget) -> c_int;
}
extern "C" {
    pub fn usb_gadget_wakeup(gadget: *mut usb_gadget) -> c_int;
}
extern "C" {
    pub fn usb_gadget_set_remote_wakeup(gadget: *mut usb_gadget, set: c_int) -> c_int;
}
extern "C" {
    pub fn usb_gadget_set_selfpowered(gadget: *mut usb_gadget) -> c_int;
}
extern "C" {
    pub fn usb_gadget_clear_selfpowered(gadget: *mut usb_gadget) -> c_int;
}
extern "C" {
    pub fn usb_gadget_vbus_connect(gadget: *mut usb_gadget) -> c_int;
}
extern "C" {
    pub fn usb_gadget_vbus_draw(gadget: *mut usb_gadget, mA: unsigned) -> c_int;
}
extern "C" {
    pub fn usb_gadget_vbus_disconnect(gadget: *mut usb_gadget) -> c_int;
}
extern "C" {
    pub fn usb_gadget_connect(gadget: *mut usb_gadget) -> c_int;
}
extern "C" {
    pub fn usb_gadget_disconnect(gadget: *mut usb_gadget) -> c_int;
}
extern "C" {
    pub fn usb_gadget_deactivate(gadget: *mut usb_gadget) -> c_int;
}
extern "C" {
    pub fn usb_gadget_activate(gadget: *mut usb_gadget) -> c_int;
}
extern "C" {
    pub fn usb_gadget_check_config(gadget: *mut usb_gadget) -> c_int;
}

// -------------------------------------------------------------------------
//
// struct usb_gadget_driver - driver for usb gadget devices
// @function: String describing the gadget's function
// @max_speed: Highest speed the driver handles.
// @setup: Invoked for ep0 control requests that aren't handled by
// the hardware level driver. Most calls must be handled by
// the gadget driver, including descriptor and configuration
// management.  The 16 bit members of the setup data are in
// USB byte order. Called in_interrupt; this may not sleep.  Driver
// queues a response to ep0, or returns negative to stall.
// @disconnect: Invoked after all transfers have been stopped,
// when the host is disconnected.  May be called in_interrupt; this
// may not sleep.  Some devices can't detect disconnect, so this might
// not be called except as part of controller shutdown.
// @bind: the driver's bind callback
// @unbind: Invoked when the driver is unbound from a gadget,
// usually from rmmod (after a disconnect is reported).
// Called in a context that permits sleeping.
// @suspend: Invoked on USB suspend.  May be called in_interrupt.
// @resume: Invoked on USB resume.  May be called in_interrupt.
// @reset: Invoked on USB bus reset. It is mandatory for all gadget drivers
// and should be called in_interrupt.
// @driver: Driver model state for this driver.
// @udc_name: A name of UDC this driver should be bound to. If udc_name is NULL,
// this driver will be bound to any available UDC.
// @match_existing_only: If udc is not found, return an error and fail
// the driver registration
// @is_bound: Allow a driver to be bound to only one gadget
//
// Devices are disabled till a gadget driver successfully bind()s, which
// means the driver will handle setup() requests needed to enumerate (and
// meet "chapter 9" requirements) then do some useful work.
//
// If gadget->is_otg is true, the gadget driver must provide an OTG
// descriptor during enumeration, or else fail the bind() call.  In such
// cases, no USB traffic may flow until both bind() returns without
// having called usb_gadget_disconnect(), and the USB host stack has
// initialized.
//
// Drivers use hardware-specific knowledge to configure the usb hardware.
// endpoint addressing is only one of several hardware characteristics that
// are in descriptors the ep0 implementation returns from setup() calls.
//
// Except for ep0 implementation, most driver code shouldn't need change to
// run on top of different usb controllers.  It'll use endpoints set up by
// that ep0 implementation.
//
// The usb controller driver handles a few standard usb requests.  Those
// include set_address, and feature flags for devices, interfaces, and
// endpoints (the get_status, set_feature, and clear_feature requests).
//
// Accordingly, the driver's setup() callback must always implement all
// get_descriptor requests, returning at least a device descriptor and
// a configuration descriptor.  Drivers must make sure the endpoint
// descriptors match any hardware constraints. Some hardware also constrains
// other descriptors. (The pxa250 allows only configurations 1, 2, or 3).
//
// The driver's setup() callback must also implement set_configuration,
// and should also implement set_interface, get_configuration, and
// get_interface.  Setting a configuration (or interface) is where
// endpoints should be activated or (config 0) shut down.
//
// The gadget driver's setup() callback does not have to queue a response to
// ep0 within the setup() call, the driver can do it after setup() returns.
// The UDC driver must wait until such a response is queued before proceeding
// with the data/status stages of the control transfer.
//
// NOTE: Currently, a number of UDC drivers rely on USB_GADGET_DELAYED_STATUS
// being returned from the setup() callback, which is a bug. See the comment
// next to USB_GADGET_DELAYED_STATUS for details.
//
// (Note that only the default control endpoint is supported.  Neither
// hosts nor devices generally support control traffic except to ep0.)
//
// Most devices will ignore USB suspend/resume operations, and so will
// not provide those callbacks.  However, some may need to change modes
// when the host is not longer directing those activities.  For example,
// local controls (buttons, dials, etc) may need to be re-enabled since
// the (remote) host can't do that any longer; or an error state might
// be cleared, to make the device behave identically whether or not
// power is maintained.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_gadget_driver {
    pub function: *mut c_char,
    pub max_speed: usb_device_speed,
    pub driver): *mut usb_gadget_driver,
    pub ): *mut *mut void (unbind)(struct usb_gadget,
    pub ): *const usb_ctrlrequest,
    pub ): *mut *mut void (disconnect)(struct usb_gadget,
    pub ): *mut *mut void (suspend)(struct usb_gadget,
    pub ): *mut *mut void (resume)(struct usb_gadget,
    pub ): *mut *mut void (reset)(struct usb_gadget,
// FIXME support safe rmmod
    pub driver: device_driver,
    pub udc_name: *mut c_char,
    pub match_existing_only:1: unsigned,
    pub is_bound:1: bool,
}

// -------------------------------------------------------------------------
// driver modules register and unregister, as usual.
// these calls must be made in a context that can sleep.
//
// A gadget driver can be bound to only one gadget at a time.
//
// usb_gadget_register_driver_owner - register a gadget driver
// @driver: the driver being registered
// @owner: the driver module
// @mod_name: the driver module's build name
// Context: can sleep
//
// Call this in your gadget driver's module initialization function,
// to tell the underlying UDC controller driver about your driver.
// The @bind() function will be called to bind it to a gadget before this
// registration call returns.  It's expected that the @bind() function will
// be in init sections.
//
// Use the macro defined below instead of calling this directly.
//
// use a define to avoid include chaining to get THIS_MODULE & friends

//
// usb_gadget_unregister_driver - unregister a gadget driver
// @driver:the driver being unregistered
// Context: can sleep
//
// Call this in your gadget driver's module cleanup function,
// to tell the underlying usb controller that your driver is
// going away.  If the controller is connected to a USB host,
// it will first disconnect().  The driver is also requested
// to unbind() and clean up any device state, before this procedure
// finally returns.  It's expected that the unbind() functions
// will be in exit sections, so may not be linked in some kernels.
//
extern "C" {
    pub fn usb_gadget_unregister_driver(driver: *mut usb_gadget_driver) -> c_int;
}
// -------------------------------------------------------------------------
// utility to simplify dealing with string descriptors
//
// struct usb_string - wraps a C string and its USB id
// @id:the (nonzero) ID for this string
// @s:the string, in UTF-8 encoding
//
// If you're using usb_gadget_get_string(), use this to wrap a string
// together with its ID.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_string {
    pub id: u8,
    pub s: *const c_char,
}

//
// struct usb_gadget_strings - a set of USB strings in a given language
// @language:identifies the strings' language (0x0409 for en-us)
// @strings:array of strings with their ids
//
// If you're using usb_gadget_get_string(), use this to wrap all the
// strings for a given language.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_gadget_strings {
    pub /: *mut *mut u16 language; / 0x0409 for en-us,
    pub strings: *mut usb_string,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_gadget_string_container {
    pub list: list_head,
    pub stash: [*mut u8; ],
}

// put descriptor for string with that id into buf (buflen >= 256)
extern "C" {
    pub fn usb_gadget_get_string(table: *const usb_gadget_strings, id: c_int, buf: *mut u8) -> c_int;
}
// check if the given language identifier is valid
extern "C" {
    pub fn usb_validate_langid(langid: u16) -> bool;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gadget_string {
    pub item: config_item,
    pub list: list_head,
    pub string: [c_char; USB_MAX_STRING_LEN],
    pub usb_string: usb_string,
}

// Macro flag: #define to_gadget_string(str_item)\
// -------------------------------------------------------------------------
// utility to simplify managing config descriptors
// write vector of descriptors into buffer
// copy a NULL-terminated vector of descriptors
//
// usb_free_descriptors - free descriptors returned by usb_copy_descriptors()
// @v: vector of descriptors
//
extern "C" {
    pub fn usb_free_all_descriptors(f: *mut usb_function);
}
// -------------------------------------------------------------------------
// utility to simplify map/unmap of usb_requests to/from DMA

// -------------------------------------------------------------------------
// utility to set gadget state properly
// -------------------------------------------------------------------------
// utility to tell udc core that the bus reset occurs
// -------------------------------------------------------------------------
// utility to give requests back to the gadget layer
// -------------------------------------------------------------------------
// utility to find endpoint by name
// -------------------------------------------------------------------------
// utility to check if endpoint caps match descriptor needs
// -------------------------------------------------------------------------
// utility to update vbus status for udc core, it may be scheduled
extern "C" {
    pub fn usb_udc_vbus_handler(gadget: *mut usb_gadget, status: bool);
}
// -------------------------------------------------------------------------
// utility wrapping a simple endpoint selection policy
extern "C" {
    pub fn usb_ep_autoconfig_release(: *mut usb_ep);
}
extern "C" {
    pub fn usb_ep_autoconfig_reset(: *mut usb_gadget);
}
