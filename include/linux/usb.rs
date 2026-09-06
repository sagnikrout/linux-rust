//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/usb.h
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

pub const USB_MAJOR: c_int = 180;
pub const USB_DEVICE_MAJOR: c_int = 189;

// -------------------------------------------------------------------------
//
// Host-side wrappers for standard USB descriptors ... these are parsed
// from the data provided by devices.  Parsing turns them from a flat
// sequence of descriptors into a hierarchy:
//
// - devices have one (usually) or more configs;
// - configs have one (often) or more interfaces;
// - interfaces have one (usually) or more settings;
// - each interface setting has zero or (usually) more endpoints.
// - a SuperSpeed endpoint has a companion descriptor
//
// And there might be other descriptors mixed in with those.
//
// Devices may also have class-specific or vendor-specific descriptors.
//
// struct usb_host_endpoint - host-side endpoint descriptor and queue
// @desc: descriptor for this endpoint, wMaxPacketSize in native byteorder
// @ss_ep_comp: SuperSpeed companion descriptor for this endpoint
// @ssp_isoc_ep_comp: SuperSpeedPlus isoc companion descriptor for this endpoint
// @eusb2_isoc_ep_comp: eUSB2 isoc companion descriptor for this endpoint
// @urb_list: urbs queued to this endpoint; maintained by usbcore
// @hcpriv: for use by HCD; typically holds hardware dma queue head (QH)
// with one or more transfer descriptors (TDs) per urb; must be preserved
// by core while BW is allocated for the endpoint
// @ep_dev: ep_device for sysfs info
// @extra: descriptors following this endpoint in the configuration
// @extralen: how many bytes of "extra" are valid
// @enabled: URBs may be submitted to this endpoint
// @streams: number of USB-3 streams allocated on the endpoint
//
// USB requests are always queued to a given endpoint, identified by a
// descriptor within an active interface in a given USB configuration.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_host_endpoint {
    pub desc: usb_endpoint_descriptor,
    pub ss_ep_comp: usb_ss_ep_comp_descriptor,
    pub ssp_isoc_ep_comp: usb_ssp_isoc_ep_comp_descriptor,
    pub eusb2_isoc_ep_comp: usb_eusb2_isoc_ep_comp_descriptor,
    pub urb_list: list_head,
    pub hcpriv: *mut c_void,
    pub /: *mut *mut *mut ep_device ep_dev; / For sysfs info,
    pub /: *mut *mut *mut unsigned char extra; / Extra descriptors,
    pub extralen: c_int,
    pub enabled: c_int,
    pub streams: c_int,
}

// host-side wrapper for one interface setting's parsed descriptors
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_host_interface {
    pub desc: usb_interface_descriptor,
    pub extralen: c_int,
    pub /: *mut *mut *mut unsigned char extra; / Extra descriptors,
// array of desc.bNumEndpoints endpoints associated with this
// interface setting.  these will be in no particular order.
//
    pub endpoint: *mut usb_host_endpoint,
    pub /: *mut *mut *mut char string; / iInterface string, if present,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum usb_interface_condition {
    USB_INTERFACE_UNBOUND = 0,
    USB_INTERFACE_BINDING,
    USB_INTERFACE_BOUND,
    USB_INTERFACE_UNBINDING,
}

extern "C" {
    pub fn usb_find_common_endpoints(_arg: alt, _arg: bulk_in, _arg: NULL, _arg: NULL, _arg: NULL) -> return;
}
extern "C" {
    pub fn usb_find_common_endpoints(_arg: alt, _arg: NULL, _arg: bulk_out, _arg: NULL, _arg: NULL) -> return;
}
extern "C" {
    pub fn usb_find_common_endpoints(_arg: alt, _arg: NULL, _arg: NULL, _arg: int_in, _arg: NULL) -> return;
}
extern "C" {
    pub fn usb_find_common_endpoints(_arg: alt, _arg: NULL, _arg: NULL, _arg: NULL, _arg: int_out) -> return;
}
extern "C" {
    pub fn usb_find_common_endpoints_reverse(_arg: alt, _arg: bulk_in, _arg: NULL, _arg: NULL, _arg: NULL) -> return;
}
extern "C" {
    pub fn usb_find_common_endpoints_reverse(_arg: alt, _arg: NULL, _arg: bulk_out, _arg: NULL, _arg: NULL) -> return;
}
extern "C" {
    pub fn usb_find_common_endpoints_reverse(_arg: alt, _arg: NULL, _arg: NULL, _arg: int_in, _arg: NULL) -> return;
}
extern "C" {
    pub fn usb_find_common_endpoints_reverse(_arg: alt, _arg: NULL, _arg: NULL, _arg: NULL, _arg: int_out) -> return;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum usb_wireless_status {
    USB_WIRELESS_STATUS_NA = 0,
    USB_WIRELESS_STATUS_DISCONNECTED,
    USB_WIRELESS_STATUS_CONNECTED,
}

//
// struct usb_interface - what usb device drivers talk to
// @altsetting: array of interface structures, one for each alternate
// setting that may be selected.  Each one includes a set of
// endpoint configurations.  They will be in no particular order.
// @cur_altsetting: the current altsetting.
// @num_altsetting: number of altsettings defined.
// @intf_assoc: interface association descriptor
// @minor: the minor number assigned to this interface, if this
// interface is bound to a driver that uses the USB major number.
// If this interface does not use the USB major, this field should
// be unused.  The driver should set this value in the probe()
// function of the driver, after it has been assigned a minor
// number from the USB core by calling usb_register_dev().
// @condition: binding state of the interface: not bound, binding
// (in probe()), bound to a driver, or unbinding (in disconnect())
// @sysfs_files_created: sysfs attributes exist
// @ep_devs_created: endpoint child pseudo-devices exist
// @unregistering: flag set when the interface is being unregistered
// @needs_remote_wakeup: flag set when the driver requires remote-wakeup
// capability during autosuspend.
// @needs_altsetting0: flag set when a set-interface request for altsetting 0
// has been deferred.
// @needs_binding: flag set when the driver should be re-probed or unbound
// following a reset or suspend operation it doesn't support.
// @authorized: This allows to (de)authorize individual interfaces instead
// a whole device in contrast to the device authorization.
// @wireless_status: if the USB device uses a receiver/emitter combo, whether
// the emitter is connected.
// @wireless_status_work: Used for scheduling wireless status changes
// from atomic context.
// @dev: driver model's view of this device
// @usb_dev: if an interface is bound to the USB major, this will point
// to the sysfs representation for that device.
// @reset_ws: Used for scheduling resets from atomic context.
// @resetting_device: USB core reset the device, so use alt setting 0 as
// current; needs bandwidth alloc after reset.
//
// USB device drivers attach to interfaces on a physical device.  Each
// interface encapsulates a single high level function, such as feeding
// an audio stream to a speaker or reporting a change in a volume control.
// Many USB devices only have one interface.  The protocol used to talk to
// an interface's endpoints can be defined in a usb "class" specification,
// or by a product's vendor.  The (default) control endpoint is part of
// every interface, but is never listed among the interface's descriptors.
//
// The driver that is bound to the interface can use standard driver model
// calls such as dev_get_drvdata() on the dev member of this structure.
//
// Each interface may have alternate settings.  The initial configuration
// of a device sets altsetting 0, but the device driver can change
// that setting using usb_set_interface().  Alternate settings are often
// used to control the use of periodic endpoints, such as by having
// different endpoints use different amounts of reserved USB bandwidth.
// All standards-conformant USB devices that use isochronous endpoints
// will use them in non-default settings.
//
// The USB specification says that alternate setting numbers must run from
// 0 to one less than the total number of alternate settings.  But some
// devices manage to mess this up, and the structures aren't necessarily
// stored in numerical order anyhow.  Use usb_altnum_to_altsetting() to
// look up an alternate setting in the altsetting array based on its number.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_interface {
// array of alternate settings for this interface,
// stored in no particular order
    pub altsetting: *mut usb_host_interface,
    pub currently: *mut *mut *mut usb_host_interface cur_altsetting; / the,
// active alternate setting
    pub /: *mut *mut unsigned num_altsetting; / number of alternate settings,
// If there is an interface association descriptor then it will list
// the associated interfaces
    pub intf_assoc: *mut usb_interface_assoc_descriptor,
    pub is: *mut *mut int minor; / minor number this interface,
// bound to
    pub /: *mut *mut usb_interface_condition condition; / state of binding,
    pub /: *mut *mut unsigned sysfs_files_created:1; / the sysfs attributes exist,
    pub /: *mut *mut unsigned ep_devs_created:1; / endpoint "devices" exist,
    pub /: *mut *mut unsigned unregistering:1; / unregistration is in progress,
    pub /: *mut *mut unsigned needs_remote_wakeup:1; / driver requires remote wakeup,
    pub /: *mut *mut unsigned needs_altsetting0:1; / switch to altsetting 0 is pending,
    pub /: *mut *mut unsigned needs_binding:1; / needs delayed unbind/rebind,
    pub /: *mut *mut unsigned resetting_device:1; / true: bandwidth alloc after reset,
    pub /: *mut *mut unsigned authorized:1; / used for interface authorization,
    pub wireless_status: usb_wireless_status,
    pub wireless_status_work: work_struct,
    pub /: *mut *mut device dev; / interface specific device info,
    pub usb_dev: *mut device,
    pub /: *mut *mut work_reset_ws; / for resets in atomic context,
}

extern "C" {
    pub fn dev_get_drvdata(_arg: &intf->dev) -> return;
}
//
// usb_set_intfdata() - associate driver-specific data with an interface
// @intf: USB interface
// @data: driver data
//
// Drivers can use this function in their probe() callbacks to associate
// driver-specific data with an interface.
//
// Note that there is generally no need to clear the driver-data pointer even
// if some drivers do so for historical or implementation-specific reasons.
//
extern "C" {
    pub fn usb_put_intf(intf: *mut usb_interface);
}
// Hard limit
pub const USB_MAXENDPOINTS: c_int = 30;
// this maximum is arbitrary
pub const USB_MAXINTERFACES: c_int = 32;

//
// USB Resume Timer: Every Host controller driver should drive the resume
// signalling on the bus for the amount of time defined by this macro.
//
// That way we will have a 'stable' behavior among all HCDs supported by Linux.
//
// Note that the USB Specification states we should drive resume for *at least
// 20 ms, but it doesn't give an upper bound. This creates two possible
// situations which we want to avoid:
//
// (a) sometimes an msleep(20) might expire slightly before 20 ms, which causes
// us to fail USB Electrical Tests, thus failing Certification
//
// (b) Some (many) devices actually need more than 20 ms of resume signalling,
// and while we can argue that's against the USB Specification, we don't have
// control over which devices a certification laboratory will be using for
// certification. If CertLab uses a device which was tested against Windows and
// that happens to have relaxed resume signalling rules, we might fall into
// situations where we fail interoperability and electrical tests.
//
// In order to avoid both conditions, we're using a 40 ms resume timeout, which
// should cope with both LPJ calibration errors and devices not following every
// detail of the USB Specification.
//

//
// struct usb_interface_cache - long-term representation of a device interface
// @num_altsetting: number of altsettings defined.
// @ref: reference counter.
// @altsetting: variable-length array of interface structures, one for
// each alternate setting that may be selected.  Each one includes a
// set of endpoint configurations.  They will be in no particular order.
//
// These structures persist for the lifetime of a usb_device, unlike
// struct usb_interface (which persists only as long as its configuration
// is installed).  The altsetting arrays can be accessed through these
// structures at any time, permitting comparison of configurations and
// providing support for the /sys/kernel/debug/usb/devices pseudo-file.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_interface_cache {
    pub /: *mut *mut unsigned num_altsetting; / number of alternate settings,
    pub /: *mut *mut kref ref; / reference counter,
// variable-length array of alternate settings for this interface,
// stored in no particular order
    pub altsetting: [usb_host_interface; ],
}

//
// struct usb_host_config - representation of a device's configuration
// @desc: the device's configuration descriptor.
// @string: pointer to the cached version of the iConfiguration string, if
// present for this configuration.
// @intf_assoc: list of any interface association descriptors in this config
// @interface: array of pointers to usb_interface structures, one for each
// interface in the configuration.  The number of interfaces is stored
// in desc.bNumInterfaces.  These pointers are valid only while the
// configuration is active.
// @intf_cache: array of pointers to usb_interface_cache structures, one
// for each interface in the configuration.  These structures exist
// for the entire life of the device.
// @extra: pointer to buffer containing all extra descriptors associated
// with this configuration (those preceding the first interface
// descriptor).
// @extralen: length of the extra descriptors buffer.
//
// USB devices may have multiple configurations, but only one can be active
// at any time.  Each encapsulates a different operational environment;
// for example, a dual-speed device would have separate configurations for
// full-speed and high-speed operation.  The number of configurations
// available is stored in the device descriptor as bNumConfigurations.
//
// A configuration can contain multiple interfaces.  Each corresponds to
// a different function of the USB device, and all are available whenever
// the configuration is active.  The USB standard says that interfaces
// are supposed to be numbered from 0 to desc.bNumInterfaces-1, but a lot
// of devices get this wrong.  In addition, the interface array is not
// guaranteed to be sorted in numerical order.  Use usb_ifnum_to_if() to
// look up an interface entry based on its number.
//
// Device drivers should not attempt to activate configurations.  The choice
// of which configuration to install is a policy decision based on such
// considerations as available power, functionality provided, and the user's
// desires (expressed through userspace tools).  However, drivers can call
// usb_reset_configuration() to reinitialize the current configuration and
// all its interfaces.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_host_config {
    pub desc: usb_config_descriptor,
    pub /: *mut *mut *mut char string; / iConfiguration string, if present,
// List of any Interface Association Descriptors in this
// configuration.
    pub intf_assoc: [*mut usb_interface_assoc_descriptor; USB_MAXIADS],
// the interfaces associated with this configuration,
// stored in no particular order
    pub interface: [*mut usb_interface; USB_MAXINTERFACES],
// Interface information available even when this is not the
// active configuration
    pub intf_cache: [*mut usb_interface_cache; USB_MAXINTERFACES],
    pub /: *mut *mut *mut unsigned char extra; / Extra descriptors,
    pub extralen: c_int,
}

// USB2.0 and USB3.0 device BOS descriptor set
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_host_bos {
    pub desc: *mut usb_bos_descriptor,
    pub ext_cap: *mut usb_ext_cap_descriptor,
    pub ss_cap: *mut usb_ss_cap_descriptor,
    pub ssp_cap: *mut usb_ssp_cap_descriptor,
    pub ss_id: *mut usb_ss_container_id_descriptor,
    pub ptm_cap: *mut usb_ptm_cap_descriptor,
}

// -----------------------------------------------------------------------
//
// Allocated per bus (tree of devices) we have:
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_bus {
    pub /: *mut *mut *mut device controller; / host side hardware,
    pub /: *mut *mut *mut device sysdev; / as seen from firmware or bus,
    pub /: *mut *mut int busnum; / Bus number (in order of reg),
    pub /: *const *const *const char bus_name; / stable id (PCI slot_name etc),
    pub /*: *mut u8 uses_pio_for_control;,
// Does the host controller use PIO
// for control transfers?
//
    pub /: *mut *mut u8 otg_port; / 0, or number of OTG/HNP port,
    pub /: *mut *mut unsigned is_b_host:1; / true during some HNP roleswitches,
    pub /: *mut *mut unsigned b_hnp_enable:1; / OTG: did A-Host enable HNP?,
    pub /*: *mut unsigned no_stop_on_short:1;,
// Quirk: some controllers don't stop
// the ep queue on a short transfer
// with the URB_SHORT_NOT_OK flag set.
//
    pub /: *mut *mut unsigned no_sg_constraint:1; / no sg constraint,
    pub /: *mut *mut unsigned sg_tablesize; / 0 or largest number of sg list entries,
    pub in: *mut *mut int devnum_next; / Next open device number,
// round-robin allocation
    pub /: *mut *mut mutex devnum_next_mutex; / devnum_next mutex,
    pub /: *mut *mut DECLARE_BITMAP(devmap, 128); / USB device number allocation bitmap,
    pub /: *mut *mut *mut usb_device root_hub; / Root hub,
    pub /: *mut *mut *mut usb_bus hs_companion; / Companion EHCI bus, if any,
    pub time: *mut *mut int bandwidth_allocated; / on this bus: how much of the,
// reserved for periodic (intr/iso)
// requests is used, on average?
// Units: microseconds/frame.
// Limits: Full/low speed reserve 90%,
// while high speed reserves 80%.
//
    pub /: *mut *mut int bandwidth_int_reqs; / number of Interrupt requests,
    pub /: *mut *mut int bandwidth_isoc_reqs; / number of Isoc. requests,
    pub /: *mut *mut unsigned resuming_ports; / bit array: resuming root-hub ports,

    pub /: *mut *mut *mut mon_bus mon_bus; / non-null when associated,
    pub /: *mut *mut int monitored; / non-zero when monitored,

}

// -----------------------------------------------------------------------
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum usb_link_tunnel_mode {
    USB_LINK_UNKNOWN = 0,
    USB_LINK_NATIVE,
    USB_LINK_TUNNELED,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum usb_port_connect_type {
    USB_PORT_CONNECT_TYPE_UNKNOWN = 0,
    USB_PORT_CONNECT_TYPE_HOT_PLUG,
    USB_PORT_CONNECT_TYPE_HARD_WIRED,
    USB_PORT_NOT_USED,
}

//
// USB port quirks.
//
// For the given port, prefer the old (faster) enumeration scheme.

// Decrease TRSTRCY to 10ms during device enumeration.

//
// USB 2.0 Link Power Management (LPM) parameters.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb2_lpm_parameters {
// Best effort service latency indicate how long the host will drive
// resume on an exit from L1.
//
    pub besl: c_uint,
// Timeout value in microseconds for the L1 inactivity (LPM) timer.
// When the timer counts to zero, the parent hub will initiate a LPM
// transition to L1.
//
    pub timeout: c_int,
}

//
// USB 3.0 Link Power Management (LPM) parameters.
//
// PEL and SEL are USB 3.0 Link PM latencies for device-initiated LPM exit.
// MEL is the USB 3.0 Link PM latency for host-initiated LPM exit.
// All three are stored in nanoseconds.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb3_lpm_parameters {
//
// Maximum exit latency (MEL) for the host to send a packet to the
// device (either a Ping for isoc endpoints, or a data packet for
// interrupt endpoints), the hubs to decode the packet, and for all hubs
// in the path to transition the links to U0.
//
    pub mel: c_uint,
//
// Maximum exit latency for a device-initiated LPM transition to bring
// all links into U0.  Abbreviated as "PEL" in section 9.4.12 of the USB
// 3.0 spec, with no explanation of what "P" stands for.  "Path"?
//
    pub pel: c_uint,
//
// The System Exit Latency (SEL) includes PEL, and three other
// latencies.  After a device initiates a U0 transition, it will take
// some time from when the device sends the ERDY to when it will finally
// receive the data packet.  Basically, SEL should be the worse-case
// latency from when a device starts initiating a U0 transition to when
// it will get data.
//
    pub sel: c_uint,
//
// The idle timeout value that is currently programmed into the parent
// hub for this device.  When the timer counts to zero, the parent hub
// will initiate an LPM transition to either U1 or U2.
//
    pub timeout: c_int,
}

//
// struct usb_device - kernel's representation of a USB device
// @devnum: device number; address on a USB bus
// @devpath: device ID string for use in messages (e.g., /port/...)
// @route: tree topology hex string for use with xHCI
// @state: device state: configured, not attached, etc.
// @speed: device speed: high/full/low (or error)
// @rx_lanes: number of rx lanes in use, USB 3.2 adds dual-lane support
// @tx_lanes: number of tx lanes in use, USB 3.2 adds dual-lane support
// @ssp_rate: SuperSpeed Plus phy signaling rate and lane count
// @tt: Transaction Translator info; used with low/full speed dev, highspeed hub
// @ttport: device port on that tt hub
// @toggle: one bit for each endpoint, with ([0] = IN, [1] = OUT) endpoints
// @parent: our hub, unless we're the root
// @bus: bus we're part of
// @ep0: endpoint 0 data (default control pipe)
// @dev: generic device interface
// @descriptor: USB device descriptor
// @bos: USB device BOS descriptor set
// @config: all of the device's configs
// @actconfig: the active configuration
// @ep_in: array of IN endpoints
// @ep_out: array of OUT endpoints
// @rawdescriptors: raw descriptors for each config
// @bus_mA: Current available from the bus
// @portnum: parent port number (origin 1)
// @level: number of USB hub ancestors
// @devaddr: device address, XHCI: assigned by HW, others: same as devnum
// @can_submit: URBs may be submitted
// @persist_enabled:  USB_PERSIST enabled for this device
// @reset_in_progress: the device is being reset
// @have_langid: whether string_langid is valid
// @authorized: policy has said we can use it;
// (user space) policy determines if we authorize this device to be
// used or not. By default, wired USB devices are authorized.
// WUSB devices are not, until we authorize them from user space.
// FIXME -- complete doc
// @authenticated: Crypto authentication passed
// @tunnel_mode: Connection native or tunneled over USB4
// @usb4_link: device link to the USB4 host interface
// @lpm_capable: device supports LPM
// @lpm_devinit_allow: Allow USB3 device initiated LPM, exit latency is in range
// @usb2_hw_lpm_capable: device can perform USB2 hardware LPM
// @usb2_hw_lpm_besl_capable: device can perform USB2 hardware BESL LPM
// @usb2_hw_lpm_enabled: USB2 hardware LPM is enabled
// @usb2_hw_lpm_allowed: Userspace allows USB 2.0 LPM to be enabled
// @usb3_lpm_u1_enabled: USB3 hardware U1 LPM enabled
// @usb3_lpm_u2_enabled: USB3 hardware U2 LPM enabled
// @string_langid: language ID for strings
// @product: iProduct string, if present (static)
// @manufacturer: iManufacturer string, if present (static)
// @serial: iSerialNumber string, if present (static)
// @filelist: usbfs files that are open to this device
// @maxchild: number of ports if hub
// @quirks: quirks of the whole device
// @urbnum: number of URBs submitted for the whole device
// @active_duration: total time device is not suspended
// @connect_time: time device was first connected
// @do_remote_wakeup:  remote wakeup should be enabled
// @reset_resume: needs reset instead of resume
// @port_is_suspended: the upstream port is suspended (L2 or U3)
// @offload_pm_locked: prevents offload_usage changes during PM transitions.
// @offload_usage: number of offload activities happening on this usb device.
// @offload_lock: protects offload_usage and offload_pm_locked
// @slot_id: Slot ID assigned by xHCI
// @l1_params: best effor service latency for USB2 L1 LPM state, and L1 timeout.
// @u1_params: exit latencies for USB3 U1 LPM state, and hub-initiated timeout.
// @u2_params: exit latencies for USB3 U2 LPM state, and hub-initiated timeout.
// @lpm_disable_count: Ref count used by usb_disable_lpm() and usb_enable_lpm()
// to keep track of the number of functions that require USB 3.0 Link Power
// Management to be disabled for this usb_device.  This count should only
// be manipulated by those functions, with the bandwidth_mutex is held.
// @hub_delay: cached value consisting of:
// parent->hub_delay + wHubDelay + tTPTransmissionDelay (40ns)
// Will be used as wValue for SetIsochDelay requests.
// @use_generic_driver: ask driver core to reprobe using the generic driver.
//
// Notes:
// Usbcore drivers should not set usbdev->state directly.  Instead use
// usb_set_device_state().
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_device {
    pub devnum: c_int,
    pub devpath: [c_char; 16],
    pub route: u32,
    pub state: usb_device_state,
    pub speed: usb_device_speed,
    pub rx_lanes: c_uint,
    pub tx_lanes: c_uint,
    pub ssp_rate: usb_ssp_rate,
    pub tt: *mut usb_tt,
    pub ttport: c_int,
    pub toggle: [c_uint; 2],
    pub parent: *mut usb_device,
    pub bus: *mut usb_bus,
    pub ep0: usb_host_endpoint,
    pub dev: device,
    pub descriptor: usb_device_descriptor,
    pub bos: *mut usb_host_bos,
    pub config: *mut usb_host_config,
    pub actconfig: *mut usb_host_config,
    pub ep_in: [*mut usb_host_endpoint; 16],
    pub ep_out: [*mut usb_host_endpoint; 16],
    pub rawdescriptors: *mut c_char,
    pub bus_mA: c_ushort,
    pub portnum: u8,
    pub level: u8,
    pub devaddr: u8,
    pub can_submit:1: unsigned,
    pub persist_enabled:1: unsigned,
    pub reset_in_progress:1: unsigned,
    pub have_langid:1: unsigned,
    pub authorized:1: unsigned,
    pub authenticated:1: unsigned,
    pub lpm_capable:1: unsigned,
    pub lpm_devinit_allow:1: unsigned,
    pub usb2_hw_lpm_capable:1: unsigned,
    pub usb2_hw_lpm_besl_capable:1: unsigned,
    pub usb2_hw_lpm_enabled:1: unsigned,
    pub usb2_hw_lpm_allowed:1: unsigned,
    pub usb3_lpm_u1_enabled:1: unsigned,
    pub usb3_lpm_u2_enabled:1: unsigned,
    pub string_langid: c_int,
// static strings from the device
    pub product: *mut c_char,
    pub manufacturer: *mut c_char,
    pub serial: *mut c_char,
    pub filelist: list_head,
    pub maxchild: c_int,
    pub quirks: u32,
    pub urbnum: core::sync::atomic::AtomicI32,
    pub active_duration: c_ulong,
    pub connect_time: c_ulong,
    pub do_remote_wakeup:1: unsigned,
    pub reset_resume:1: unsigned,
    pub port_is_suspended:1: unsigned,
    pub offload_pm_locked:1: unsigned,
    pub offload_usage: c_int,
    pub offload_lock: spinlock_t,
    pub tunnel_mode: usb_link_tunnel_mode,
    pub usb4_link: *mut device_link,
    pub slot_id: c_int,
    pub l1_params: usb2_lpm_parameters,
    pub u1_params: usb3_lpm_parameters,
    pub u2_params: usb3_lpm_parameters,
    pub lpm_disable_count: unsigned,
    pub hub_delay: u16,
    pub use_generic_driver:1: unsigned,
}

extern "C" {
    pub fn to_usb_device(_arg: intf->dev.parent) -> return;
}
extern "C" {
    pub fn to_usb_device()intf->dev.parent: *const (struct device) -> return;
}

extern "C" {
    pub fn usb_put_dev(dev: *mut usb_device);
}
//
// usb_hub_for_each_child - iterate over all child devices on the hub
// @hdev:  USB device belonging to the usb hub
// @port1: portnum associated with child device
// @child: child device pointer
//

// USB device locking

// USB port reset for device reinitialization
extern "C" {
    pub fn usb_reset_device(dev: *mut usb_device) -> c_int;
}
extern "C" {
    pub fn usb_queue_reset_device(dev: *mut usb_interface);
}

extern "C" {
    pub fn usb_acpi_power_manageable(hdev: *mut usb_device, index: c_int) -> bool;
}
extern "C" {
    pub fn usb_acpi_port_lpm_incapable(hdev: *mut usb_device, index: c_int) -> c_int;
}

// USB autosuspend and autoresume

extern "C" {
    pub fn usb_enable_autosuspend(udev: *mut usb_device);
}
extern "C" {
    pub fn usb_disable_autosuspend(udev: *mut usb_device);
}
extern "C" {
    pub fn usb_autopm_get_interface(intf: *mut usb_interface) -> c_int;
}
extern "C" {
    pub fn usb_autopm_put_interface(intf: *mut usb_interface);
}
extern "C" {
    pub fn usb_autopm_get_interface_async(intf: *mut usb_interface) -> c_int;
}
extern "C" {
    pub fn usb_autopm_put_interface_async(intf: *mut usb_interface);
}
extern "C" {
    pub fn usb_autopm_get_interface_no_resume(intf: *mut usb_interface);
}
extern "C" {
    pub fn usb_autopm_put_interface_no_suspend(intf: *mut usb_interface);
}

extern "C" {
    pub fn usb_offload_get(udev: *mut usb_device) -> c_int;
}
extern "C" {
    pub fn usb_offload_put(udev: *mut usb_device) -> c_int;
}
extern "C" {
    pub fn usb_offload_check(udev: *mut usb_device) -> bool;
}
extern "C" {
    pub fn usb_offload_set_pm_locked(udev: *mut usb_device, locked: bool);
}

extern "C" {
    pub fn usb_disable_lpm(udev: *mut usb_device) -> c_int;
}
extern "C" {
    pub fn usb_enable_lpm(udev: *mut usb_device);
}
// Same as above, but these functions lock/unlock the bandwidth_mutex.
extern "C" {
    pub fn usb_unlocked_disable_lpm(udev: *mut usb_device) -> c_int;
}
extern "C" {
    pub fn usb_unlocked_enable_lpm(udev: *mut usb_device);
}
extern "C" {
    pub fn usb_disable_ltm(udev: *mut usb_device) -> c_int;
}
extern "C" {
    pub fn usb_enable_ltm(udev: *mut usb_device);
}
// -------------------------------------------------------------------------
// for drivers using iso endpoints
extern "C" {
    pub fn usb_get_current_frame_number(usb_dev: *mut usb_device) -> c_int;
}
// Sets up a group of bulk endpoints to support multiple stream IDs.
// Reverts a group of bulk endpoints back to not using stream IDs.
// used these for multi-interface device registration
//
// usb_interface_claimed - returns true iff an interface is claimed
// @iface: the interface being checked
//
// Return: %true (nonzero) iff the interface is claimed, else %false
// (zero).
//
// Note:
// Callers must own the driver model's usb bus readlock.  So driver
// probe() entries don't need extra locking, but other call contexts
// may need to explicitly claim that lock.
//
extern "C" {
    pub fn usb_for_each_dev(data: *mut c_void, : *mut *mut int (fn)(struct usb_device, ): *mut c_void) -> c_int;
}
// port claiming functions
//
// usb_make_path - returns stable device path in the usb tree
// @dev: the device whose path is being constructed
// @buf: where to put the string
// @size: how big is "buf"?
//
// Return: Length of the string (> 0) or negative if size was too small.
//
// Note:
// This identifier is intended to be "stable", reflecting physical paths in
// hardware such as physical bus addresses for host controllers or ports on
// USB hubs.  That makes it stay the same until systems are physically
// reconfigured, by re-cabling a tree of USB devices or by moving USB host
// controllers.  Adding and removing devices, including virtual root hubs
// in host controller driver modules, does not change these path identifiers;
// neither does rebooting or re-enumerating.  These are more useful identifiers
// than changeable ("unstable") ones like bus numbers or device addresses.
//
// With a partial exception for devices connected to USB 2.0 root hubs, these
// identifiers are also predictable.  So long as the device tree isn't changed,
// plugging any USB device into a given hub port always gives it the same path.
// Because of the use of "companion" controllers, devices connected to ports on
// USB 2.0 root hubs (EHCI host controllers) will get one path ID if they are
// high speed, and a different one if they are full or low speed.
//
// -------------------------------------------------------------------------

//
// USB_DEVICE - macro used to describe a specific usb device
// @vend: the 16 bit USB Vendor ID
// @prod: the 16 bit USB Product ID
//
// This macro is used to create a struct usb_device_id that matches a
// specific device.
//

//
// USB_DEVICE_VER - describe a specific usb device with a version range
// @vend: the 16 bit USB Vendor ID
// @prod: the 16 bit USB Product ID
// @lo: the bcdDevice_lo value
// @hi: the bcdDevice_hi value
//
// This macro is used to create a struct usb_device_id that matches a
// specific device, with a version range.
//

//
// USB_DEVICE_INTERFACE_CLASS - describe a usb device with a specific interface class
// @vend: the 16 bit USB Vendor ID
// @prod: the 16 bit USB Product ID
// @cl: bInterfaceClass value
//
// This macro is used to create a struct usb_device_id that matches a
// specific interface class of devices.
//

//
// USB_DEVICE_INTERFACE_PROTOCOL - describe a usb device with a specific interface protocol
// @vend: the 16 bit USB Vendor ID
// @prod: the 16 bit USB Product ID
// @pr: bInterfaceProtocol value
//
// This macro is used to create a struct usb_device_id that matches a
// specific interface protocol of devices.
//

//
// USB_DEVICE_INTERFACE_NUMBER - describe a usb device with a specific interface number
// @vend: the 16 bit USB Vendor ID
// @prod: the 16 bit USB Product ID
// @num: bInterfaceNumber value
//
// This macro is used to create a struct usb_device_id that matches a
// specific interface number of devices.
//

//
// USB_DEVICE_INFO - macro used to describe a class of usb devices
// @cl: bDeviceClass value
// @sc: bDeviceSubClass value
// @pr: bDeviceProtocol value
//
// This macro is used to create a struct usb_device_id that matches a
// specific class of devices.
//

//
// USB_INTERFACE_INFO - macro used to describe a class of usb interfaces
// @cl: bInterfaceClass value
// @sc: bInterfaceSubClass value
// @pr: bInterfaceProtocol value
//
// This macro is used to create a struct usb_device_id that matches a
// specific class of interfaces.
//

//
// USB_DEVICE_AND_INTERFACE_INFO - describe a specific usb device with a class of usb interfaces
// @vend: the 16 bit USB Vendor ID
// @prod: the 16 bit USB Product ID
// @cl: bInterfaceClass value
// @sc: bInterfaceSubClass value
// @pr: bInterfaceProtocol value
//
// This macro is used to create a struct usb_device_id that matches a
// specific device with a specific class of interfaces.
//
// This is especially useful when explicitly matching devices that have
// vendor specific bDeviceClass values, but standards-compliant interfaces.
//

//
// USB_VENDOR_AND_INTERFACE_INFO - describe a specific usb vendor with a class of usb interfaces
// @vend: the 16 bit USB Vendor ID
// @cl: bInterfaceClass value
// @sc: bInterfaceSubClass value
// @pr: bInterfaceProtocol value
//
// This macro is used to create a struct usb_device_id that matches a
// specific vendor with a specific class of interfaces.
//
// This is especially useful when explicitly matching devices that have
// vendor specific bDeviceClass values, but standards-compliant interfaces.
//

// -----------------------------------------------------------------------
// Stuff for dynamic usb ids
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_dynids {
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_dynid {
    pub node: list_head,
    pub id: usb_device_id,
}

extern "C" {
    pub fn usb_show_dynids(dynids: *mut usb_dynids, buf: *mut c_char) -> isize;
}
//
// struct usb_driver - identifies USB interface driver to usbcore
// @name: The driver name should be unique among USB drivers,
// and should normally be the same as the module name.
// @probe: Called to see if the driver is willing to manage a particular
// interface on a device.  If it is, probe returns zero and uses
// usb_set_intfdata() to associate driver-specific data with the
// interface.  It may also use usb_set_interface() to specify the
// appropriate altsetting.  If unwilling to manage the interface,
// return -ENODEV, if genuine IO errors occurred, an appropriate
// negative errno value.  The usb_device_id parameter is only valid during
// probe.
// @disconnect: Called when the interface is no longer accessible, usually
// because its device has been (or is being) disconnected or the
// driver module is being unloaded.
// @unlocked_ioctl: Used for drivers that want to talk to userspace through
// the "usbfs" filesystem.  This lets devices provide ways to
// expose information to user space regardless of where they
// do (or don't) show up otherwise in the filesystem.
// @suspend: Called when the device is going to be suspended by the
// system either from system sleep or runtime suspend context. The
// return value will be ignored in system sleep context, so do NOT
// try to continue using the device if suspend fails in this case.
// Instead, let the resume or reset-resume routine recover from
// the failure.
// @resume: Called when the device is being resumed by the system.
// @reset_resume: Called when the suspended device has been reset instead
// of being resumed.
// @pre_reset: Called by usb_reset_device() when the device is about to be
// reset.  This routine must not return until the driver has no active
// URBs for the device, and no more URBs may be submitted until the
// post_reset method is called.
// @post_reset: Called by usb_reset_device() after the device
// has been reset
// @shutdown: Called at shut-down time to quiesce the device.
// @id_table: USB drivers use ID table to support hotplugging.
// Export this with MODULE_DEVICE_TABLE(usb,...).  This must be set
// or your driver's probe function will never get called.
// @dev_groups: Attributes attached to the device that will be created once it
// is bound to the driver.
// @dynids: used internally to hold the list of dynamically added device
// ids for this driver.
// @driver: The driver-model core driver structure.
// @no_dynamic_id: if set to 1, the USB core will not allow dynamic ids to be
// added to this driver by preventing the sysfs file from being created.
// @supports_autosuspend: if set to 0, the USB core will not allow autosuspend
// for interfaces bound to this driver.
// @soft_unbind: if set to 1, the USB core will not kill URBs and disable
// endpoints before calling the driver's disconnect method.
// @disable_hub_initiated_lpm: if set to 1, the USB core will not allow hubs
// to initiate lower power link state transitions when an idle timeout
// occurs.  Device-initiated USB 3.0 link PM will still be allowed.
//
// USB interface drivers must provide a name, probe() and disconnect()
// methods, and an id_table.  Other driver fields are optional.
//
// The id_table is used in hotplugging.  It holds a set of descriptors,
// and specialized data may be associated with each entry.  That table
// is used by both user and kernel mode hotplugging support.
//
// The probe() and disconnect() methods are called in a context where
// they can sleep, but they should avoid abusing the privilege.  Most
// work to connect to a device should be done when the device is opened,
// and undone at the last close.  The disconnect code needs to address
// concurrency issues with respect to open() and close() methods, as
// well as forcing all pending I/O requests to complete (by unlinking
// them as necessary, and blocking until the unlinks complete).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_driver {
    pub name: *const c_char,
    pub id): *const usb_device_id,
    pub intf): *mut *mut void (disconnect) (struct usb_interface,
    pub buf): *mut c_void,
    pub message): *mut *mut *mut int (suspend) (struct usb_interface intf, pm_message_t,
    pub intf): *mut *mut int (resume) (struct usb_interface,
    pub intf): *mut *mut int (reset_resume)(struct usb_interface,
    pub intf): *mut *mut int (pre_reset)(struct usb_interface,
    pub intf): *mut *mut int (post_reset)(struct usb_interface,
    pub intf): *mut *mut void (shutdown)(struct usb_interface,
    pub id_table: *const usb_device_id,
    pub dev_groups: *const attribute_group,
    pub dynids: usb_dynids,
    pub driver: device_driver,
    pub no_dynamic_id:1: c_uint,
    pub supports_autosuspend:1: c_uint,
    pub disable_hub_initiated_lpm:1: c_uint,
    pub soft_unbind:1: c_uint,
}

//
// struct usb_device_driver - identifies USB device driver to usbcore
// @name: The driver name should be unique among USB drivers,
// and should normally be the same as the module name.
// @match: If set, used for better device/driver matching.
// @probe: Called to see if the driver is willing to manage a particular
// device.  If it is, probe returns zero and uses dev_set_drvdata()
// to associate driver-specific data with the device.  If unwilling
// to manage the device, return a negative errno value.
// @disconnect: Called when the device is no longer accessible, usually
// because it has been (or is being) disconnected or the driver's
// module is being unloaded.
// @suspend: Called when the device is going to be suspended by the system.
// @resume: Called when the device is being resumed by the system.
// @choose_configuration: If non-NULL, called instead of the default
// usb_choose_configuration(). If this returns an error then we'll go
// on to call the normal usb_choose_configuration().
// @dev_groups: Attributes attached to the device that will be created once it
// is bound to the driver.
// @driver: The driver-model core driver structure.
// @id_table: used with @match() to select better matching driver at
// probe() time.
// @supports_autosuspend: if set to 0, the USB core will not allow autosuspend
// for devices bound to this driver.
// @generic_subclass: if set to 1, the generic USB driver's probe, disconnect,
// resume and suspend functions will be called in addition to the driver's
// own, so this part of the setup does not need to be replicated.
//
// USB device drivers must provide a name, other driver fields are optional.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_device_driver {
    pub name: *const c_char,
    pub udev): *mut *mut bool (match) (struct usb_device,
    pub udev): *mut *mut int (probe) (struct usb_device,
    pub udev): *mut *mut void (disconnect) (struct usb_device,
    pub message): *mut *mut *mut int (suspend) (struct usb_device udev, pm_message_t,
    pub message): *mut *mut *mut int (resume) (struct usb_device udev, pm_message_t,
    pub udev): *mut *mut int (choose_configuration) (struct usb_device,
    pub dev_groups: *const attribute_group,
    pub driver: device_driver,
    pub id_table: *const usb_device_id,
    pub supports_autosuspend:1: c_uint,
    pub generic_subclass:1: c_uint,
}

//
// struct usb_class_driver - identifies a USB driver that wants to use the USB major number
// @name: the usb class device name for this driver.  Will show up in sysfs.
// @devnode: Callback to provide a naming hint for a possible
// device node to create.
// @fops: pointer to the struct file_operations of this driver.
// @minor_base: the start of the minor range for this driver.
//
// This structure is used for the usb_register_dev() and
// usb_deregister_dev() functions, to consolidate a number of the
// parameters used for them.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_class_driver {
    pub name: *mut c_char,
    pub mode): *const *const *const *const char (devnode)(struct device dev, umode_t,
    pub fops: *const file_operations,
    pub minor_base: c_int,
}

//
// use these in module_init()/module_exit()
// and don't forget MODULE_DEVICE_TABLE(usb, ...)
//
// use a define to avoid include chaining to get THIS_MODULE & friends

extern "C" {
    pub fn usb_deregister(: *mut usb_driver);
}
//
// module_usb_driver() - Helper macro for registering a USB driver
// @__usb_driver: usb_driver struct
//
// Helper macro for USB drivers which do not do anything special in module
// init/exit. This eliminates a lot of boilerplate. Each module may only
// use this macro once, and calling it replaces module_init() and module_exit()
//

extern "C" {
    pub fn usb_deregister_device_driver(: *mut usb_device_driver);
}
extern "C" {
    pub fn usb_disabled() -> c_int;
}
// -----------------------------------------------------------------------
//
// URB support, for asynchronous request completions
//
// urb->transfer_flags:
//
// Note: URB_DIR_IN/OUT is automatically set in usb_submit_urb().
//
pub const URB_SHORT_NOT_OK: c_uint = 0x0001	/* report short reads as errors */;
pub const URB_ISO_ASAP: c_uint = 0x0002	/* iso-only; use the first unexpired;
// slot in the schedule
pub const URB_NO_TRANSFER_DMA_MAP: c_uint = 0x0004	/* urb->transfer_dma valid on submit */;
pub const URB_ZERO_PACKET: c_uint = 0x0040	/* Finish bulk OUT with short packet */;
pub const URB_NO_INTERRUPT: c_uint = 0x0080	/* HINT: no non-error interrupt;
// needed
pub const URB_FREE_BUFFER: c_uint = 0x0100	/* Free transfer buffer with the URB */;
// The following flags are used internally by usbcore and HCDs
pub const URB_DIR_IN: c_uint = 0x0200	/* Transfer from device to host */;
pub const URB_DIR_OUT: c_int = 0;

pub const URB_DMA_MAP_SINGLE: c_uint = 0x00010000	/* Non-scatter-gather mapping */;
pub const URB_DMA_MAP_PAGE: c_uint = 0x00020000	/* HCD-unsupported S-G */;
pub const URB_DMA_MAP_SG: c_uint = 0x00040000	/* HCD-supported S-G */;
pub const URB_MAP_LOCAL: c_uint = 0x00080000	/* HCD-local-memory mapping */;
pub const URB_SETUP_MAP_SINGLE: c_uint = 0x00100000	/* Setup packet DMA mapped */;
pub const URB_SETUP_MAP_LOCAL: c_uint = 0x00200000	/* HCD-local setup packet */;
pub const URB_DMA_SG_COMBINED: c_uint = 0x00400000	/* S-G entries were combined */;
pub const URB_ALIGNED_TEMP_BUFFER: c_uint = 0x00800000	/* Temp buffer was alloc'd */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_iso_packet_descriptor {
    pub offset: c_uint,
    pub /: *mut *mut unsigned int length; / expected length,
    pub actual_length: c_uint,
    pub status: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_anchor {
    pub urb_list: list_head,
    pub wait: wait_queue_head_t,
    pub lock: spinlock_t,
    pub suspend_wakeups: core::sync::atomic::AtomicI32,
    pub poisoned:1: c_uint,
}

extern "C" {
    pub fn void(: *mut *mut usb_complete_t)(struct urb) -> typedef;
}
//
// struct urb - USB Request Block
// @urb_list: For use by current owner of the URB.
// @anchor_list: membership in the list of an anchor
// @anchor: to anchor URBs to a common mooring
// @ep: Points to the endpoint's data structure.  Will eventually
// replace @pipe.
// @pipe: Holds endpoint number, direction, type, and more.
// Create these values with the eight macros available;
// usb_{snd,rcv}TYPEpipe(dev,endpoint), where the TYPE is "ctrl"
// (control), "bulk", "int" (interrupt), or "iso" (isochronous).
// For example usb_sndbulkpipe() or usb_rcvintpipe().  Endpoint
// numbers range from zero to fifteen.  Note that "in" endpoint two
// is a different endpoint (and pipe) from "out" endpoint two.
// The current configuration controls the existence, type, and
// maximum packet size of any given endpoint.
// @stream_id: the endpoint's stream ID for bulk streams
// @dev: Identifies the USB device to perform the request.
// @status: This is read in non-iso completion functions to get the
// status of the particular request.  ISO requests only use it
// to tell whether the URB was unlinked; detailed status for
// each frame is in the fields of the iso_frame-desc.
// @transfer_flags: A variety of flags may be used to affect how URB
// submission, unlinking, or operation are handled.  Different
// kinds of URB can use different flags.
// @transfer_buffer:  This identifies the buffer to (or from) which the I/O
// request will be performed unless URB_NO_TRANSFER_DMA_MAP is set
// (however, do not leave garbage in transfer_buffer even then).
// This buffer must be suitable for DMA; allocate it with
// kmalloc() or equivalent.  For transfers to "in" endpoints, contents
// of this buffer will be modified.  This buffer is used for the data
// stage of control transfers.
// @transfer_dma: When transfer_flags includes URB_NO_TRANSFER_DMA_MAP,
// the device driver is saying that it provided this DMA address,
// which the host controller driver should use in preference to the
// transfer_buffer.
// @sg: scatter gather buffer list, the buffer size of each element in
// the list (except the last) must be divisible by the endpoint's
// max packet size if no_sg_constraint isn't set in 'struct usb_bus'
// @sgt: used to hold a scatter gather table returned by usb_alloc_noncoherent(),
// which describes the allocated non-coherent and possibly non-contiguous
// memory and is guaranteed to have 1 single DMA mapped segment. The
// allocated memory needs to be freed by usb_free_noncoherent().
// @num_mapped_sgs: (internal) number of mapped sg entries
// @num_sgs: number of entries in the sg list
// @transfer_buffer_length: How big is transfer_buffer.  The transfer may
// be broken up into chunks according to the current maximum packet
// size for the endpoint, which is a function of the configuration
// and is encoded in the pipe.  When the length is zero, neither
// transfer_buffer nor transfer_dma is used.
// @actual_length: This is read in non-iso completion functions, and
// it tells how many bytes (out of transfer_buffer_length) were
// transferred.  It will normally be the same as requested, unless
// either an error was reported or a short read was performed.
// The URB_SHORT_NOT_OK transfer flag may be used to make such
// short reads be reported as errors.
// @setup_packet: Only used for control transfers, this points to eight bytes
// of setup data.  Control transfers always start by sending this data
// to the device.  Then transfer_buffer is read or written, if needed.
// @setup_dma: DMA pointer for the setup packet.  The caller must not use
// this field; setup_packet must point to a valid buffer.
// @start_frame: Returns the initial frame for isochronous transfers.
// @number_of_packets: Lists the number of ISO transfer buffers.
// @interval: Specifies the polling interval for interrupt or isochronous
// transfers.  The units are frames (milliseconds) for full and low
// speed devices, and microframes (1/8 millisecond) for highspeed
// and SuperSpeed devices.
// @error_count: Returns the number of ISO transfers that reported errors.
// @context: For use in completion functions.  This normally points to
// request-specific driver context.
// @complete: Completion handler. This URB is passed as the parameter to the
// completion function.  The completion function may then do what
// it likes with the URB, including resubmitting or freeing it.
// @iso_frame_desc: Used to provide arrays of ISO transfer buffers and to
// collect the transfer status for each buffer.
//
// This structure identifies USB transfer requests.  URBs must be allocated by
// calling usb_alloc_urb() and freed with a call to usb_free_urb().
// Initialization may be done using various usb_fill_*_urb() functions.  URBs
// are submitted using usb_submit_urb(), and pending requests may be canceled
// using usb_unlink_urb() or usb_kill_urb().
//
// Data Transfer Buffers:
//
// Normally drivers provide I/O buffers allocated with kmalloc() or otherwise
// taken from the general page pool.  That is provided by transfer_buffer
// (control requests also use setup_packet), and host controller drivers
// perform a dma mapping (and unmapping) for each buffer transferred.  Those
// mapping operations can be expensive on some platforms (perhaps using a dma
// bounce buffer or talking to an IOMMU),
// although they're cheap on commodity x86 and ppc hardware.
//
// Alternatively, drivers may pass the URB_NO_TRANSFER_DMA_MAP transfer flag,
// which tells the host controller driver that no such mapping is needed for
// the transfer_buffer since
// the device driver is DMA-aware.  For example, a device driver might
// allocate a DMA buffer with usb_alloc_coherent() or call usb_buffer_map().
// When this transfer flag is provided, host controller drivers will
// attempt to use the dma address found in the transfer_dma
// field rather than determining a dma address themselves.
//
// Note that transfer_buffer must still be set if the controller
// does not support DMA (as indicated by hcd_uses_dma()) and when talking
// to root hub. If you have to transfer between highmem zone and the device
// on such controller, create a bounce buffer or bail out with an error.
// If transfer_buffer cannot be set (is in highmem) and the controller is DMA
// capable, assign NULL to it, so that usbmon knows not to use the value.
// The setup_packet must always be set, so it cannot be located in highmem.
//
// Initialization:
//
// All URBs submitted must initialize the dev, pipe, transfer_flags (may be
// zero), and complete fields.  All URBs must also initialize
// transfer_buffer and transfer_buffer_length.  They may provide the
// URB_SHORT_NOT_OK transfer flag, indicating that short reads are
// to be treated as errors; that flag is invalid for write requests.
//
// Bulk URBs may
// use the URB_ZERO_PACKET transfer flag, indicating that bulk OUT transfers
// should always terminate with a short packet, even if it means adding an
// extra zero length packet.
//
// Control URBs must provide a valid pointer in the setup_packet field.
// Unlike the transfer_buffer, the setup_packet may not be mapped for DMA
// beforehand.
//
// Interrupt URBs must provide an interval, saying how often (in milliseconds
// or, for highspeed devices, 125 microsecond units)
// to poll for transfers.  After the URB has been submitted, the interval
// field reflects how the transfer was actually scheduled.
// The polling interval may be more frequent than requested.
// For example, some controllers have a maximum interval of 32 milliseconds,
// while others support intervals of up to 1024 milliseconds.
// Isochronous URBs also have transfer intervals.  (Note that for isochronous
// endpoints, as well as high speed interrupt endpoints, the encoding of
// the transfer interval in the endpoint descriptor is logarithmic.
// Device drivers must convert that value to linear units themselves.)
//
// If an isochronous endpoint queue isn't already running, the host
// controller will schedule a new URB to start as soon as bandwidth
// utilization allows.  If the queue is running then a new URB will be
// scheduled to start in the first transfer slot following the end of the
// preceding URB, if that slot has not already expired.  If the slot has
// expired (which can happen when IRQ delivery is delayed for a long time),
// the scheduling behavior depends on the URB_ISO_ASAP flag.  If the flag
// is clear then the URB will be scheduled to start in the expired slot,
// implying that some of its packets will not be transferred; if the flag
// is set then the URB will be scheduled in the first unexpired slot,
// breaking the queue's synchronization.  Upon URB completion, the
// start_frame field will be set to the (micro)frame number in which the
// transfer was scheduled.  Ranges for frame counter values are HC-specific
// and can go from as low as 256 to as high as 65536 frames.
//
// Isochronous URBs have a different data transfer model, in part because
// the quality of service is only "best effort".  Callers provide specially
// allocated URBs, with number_of_packets worth of iso_frame_desc structures
// at the end.  Each such packet is an individual ISO transfer.  Isochronous
// URBs are normally queued, submitted by drivers to arrange that
// transfers are at least double buffered, and then explicitly resubmitted
// in completion handlers, so
// that data (such as audio or video) streams at as constant a rate as the
// host controller scheduler can support.
//
// Completion Callbacks:
//
// The completion callback is made in_interrupt(), and one of the first
// things that a completion handler should do is check the status field.
// The status field is provided for all URBs.  It is used to report
// unlinked URBs, and status for all non-ISO transfers.  It should not
// be examined before the URB is returned to the completion handler.
//
// The context field is normally used to link URBs back to the relevant
// driver or request state.
//
// When the completion callback is invoked for non-isochronous URBs, the
// actual_length field tells how many bytes were transferred.  This field
// is updated even when the URB terminated with an error or was unlinked.
//
// ISO transfer status is reported in the status and actual_length fields
// of the iso_frame_desc array, and the number of errors is reported in
// error_count.  Completion callbacks for ISO transfers will normally
// (re)submit URBs to ensure a constant transfer rate.
//
// Note that even fields marked "public" should not be touched by the driver
// when the urb is owned by the hcd, that is, since the call to
// usb_submit_urb() till the entry into the completion routine.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct urb {
// private: usb core and host controller only fields in the urb
    pub /: *mut *mut kref kref; / reference count of the URB,
    pub /: *mut *mut int unlinked; / unlink error code,
    pub /: *mut *mut *mut void hcpriv; / private data for host controller,
    pub /: *mut *mut atomic_t use_count; / concurrent submissions counter,
    pub /: *mut *mut atomic_t reject; / submissions will fail,
// public: documented fields in the urb that can be used by drivers
    pub urb's: *mut *mut list_head urb_list; / list head for use by the,
// current owner
    pub /: *mut *mut list_head anchor_list; / the URB may be anchored,
    pub anchor: *mut usb_anchor,
    pub /: *mut *mut *mut usb_device dev; / (in) pointer to associated device,
    pub /: *mut *mut *mut usb_host_endpoint ep; / (internal) pointer to endpoint,
    pub /: *mut *mut unsigned int pipe; / (in) pipe information,
    pub /: *mut *mut unsigned int stream_id; / (in) stream ID,
    pub /: *mut *mut int status; / (return) non-ISO status,
    pub ...*/: *mut *mut unsigned int transfer_flags; / (in) URB_SHORT_NOT_OK |,
    pub /: *mut *mut *mut void transfer_buffer; / (in) associated data buffer,
    pub /: *mut *mut dma_addr_t transfer_dma; / (in) dma addr for transfer_buffer,
    pub /: *mut *mut *mut scatterlist sg; / (in) scatter gather buffer list,
    pub /: *mut *mut *mut sg_table sgt; / (in) scatter gather table for noncoherent buffer,
    pub /: *mut *mut int num_mapped_sgs; / (internal) mapped sg entries,
    pub /: *mut *mut int num_sgs; / (in) number of entries in the sg list,
    pub /: *mut *mut u32 transfer_buffer_length; / (in) data buffer length,
    pub /: *mut *mut u32 actual_length; / (return) actual transfer length,
    pub /: *mut *mut *mut unsigned char setup_packet; / (in) setup packet (control only),
    pub /: *mut *mut dma_addr_t setup_dma; / (in) dma addr for setup_packet,
    pub /: *mut *mut int start_frame; / (modify) start frame (ISO),
    pub /: *mut *mut int number_of_packets; / (in) number of ISO packets,
    pub interval: *mut *mut int interval; / (modify) transfer,
// (INT/ISO)
    pub /: *mut *mut int error_count; / (return) number of ISO errors,
    pub /: *mut *mut *mut void context; / (in) context for completion,
    pub /: *mut *mut usb_complete_t complete; / (in) completion routine,
    pub iso_frame_desc: [usb_iso_packet_descriptor; ],
// (in) ISO ONLY
}

// -----------------------------------------------------------------------
//
// usb_fill_control_urb - initializes a control urb
// @urb: pointer to the urb to initialize.
// @dev: pointer to the struct usb_device for this urb.
// @pipe: the endpoint pipe
// @setup_packet: pointer to the setup_packet buffer. The buffer must be
// suitable for DMA.
// @transfer_buffer: pointer to the transfer buffer. The buffer must be
// suitable for DMA.
// @buffer_length: length of the transfer buffer
// @complete_fn: pointer to the usb_complete_t function
// @context: what to set the urb context to.
//
// Initializes a control urb with the proper information needed to submit
// it to a device.
//
// The transfer buffer and the setup_packet buffer will most likely be filled
// or read via DMA. The simplest way to get a buffer that can be DMAed to is
// allocating it via kmalloc() or equivalent, even for very small buffers.
// If the buffers are embedded in a bigger structure, there is a risk that
// the buffer itself, the previous fields and/or the next fields are corrupted
// due to cache incoherencies; or slowed down if they are evicted from the
// cache. For more information, check &struct urb.
//
// usb_fill_bulk_urb - macro to help initialize a bulk urb
// @urb: pointer to the urb to initialize.
// @dev: pointer to the struct usb_device for this urb.
// @pipe: the endpoint pipe
// @transfer_buffer: pointer to the transfer buffer. The buffer must be
// suitable for DMA.
// @buffer_length: length of the transfer buffer
// @complete_fn: pointer to the usb_complete_t function
// @context: what to set the urb context to.
//
// Initializes a bulk urb with the proper information needed to submit it
// to a device.
//
// Refer to usb_fill_control_urb() for a description of the requirements for
// transfer_buffer.
//
// usb_fill_int_urb - macro to help initialize a interrupt urb
// @urb: pointer to the urb to initialize.
// @dev: pointer to the struct usb_device for this urb.
// @pipe: the endpoint pipe
// @transfer_buffer: pointer to the transfer buffer. The buffer must be
// suitable for DMA.
// @buffer_length: length of the transfer buffer
// @complete_fn: pointer to the usb_complete_t function
// @context: what to set the urb context to.
// @interval: what to set the urb interval to, encoded like
// the endpoint descriptor's bInterval value.
//
// Initializes a interrupt urb with the proper information needed to submit
// it to a device.
//
// Refer to usb_fill_control_urb() for a description of the requirements for
// transfer_buffer.
//
// Note that High Speed and SuperSpeed(+) interrupt endpoints use a logarithmic
// encoding of the endpoint interval, and express polling intervals in
// microframes (eight per millisecond) rather than in frames (one per
// millisecond).
//
// make sure interval is within allowed range
extern "C" {
    pub fn usb_init_urb(urb: *mut urb);
}
extern "C" {
    pub fn usb_free_urb(urb: *mut urb);
}

extern "C" {
    pub fn usb_submit_urb(urb: *mut urb, mem_flags: gfp_t) -> c_int;
}
extern "C" {
    pub fn usb_unlink_urb(urb: *mut urb) -> c_int;
}
extern "C" {
    pub fn usb_kill_urb(urb: *mut urb);
}
extern "C" {
    pub fn usb_poison_urb(urb: *mut urb);
}
extern "C" {
    pub fn usb_unpoison_urb(urb: *mut urb);
}
extern "C" {
    pub fn usb_block_urb(urb: *mut urb);
}
extern "C" {
    pub fn usb_kill_anchored_urbs(anchor: *mut usb_anchor);
}
extern "C" {
    pub fn usb_poison_anchored_urbs(anchor: *mut usb_anchor);
}
extern "C" {
    pub fn usb_unpoison_anchored_urbs(anchor: *mut usb_anchor);
}
extern "C" {
    pub fn usb_anchor_suspend_wakeups(anchor: *mut usb_anchor);
}
extern "C" {
    pub fn usb_anchor_resume_wakeups(anchor: *mut usb_anchor);
}
extern "C" {
    pub fn usb_anchor_urb(urb: *mut urb, anchor: *mut usb_anchor);
}
extern "C" {
    pub fn usb_unanchor_urb(urb: *mut urb);
}
extern "C" {
    pub fn usb_scuttle_anchored_urbs(anchor: *mut usb_anchor);
}
extern "C" {
    pub fn usb_anchor_empty(anchor: *mut usb_anchor) -> c_int;
}

//
// usb_urb_dir_in - check if an URB describes an IN transfer
// @urb: URB to be checked
//
// Return: 1 if @urb describes an IN transfer (device-to-host),
// otherwise 0.
//
// usb_urb_dir_out - check if an URB describes an OUT transfer
// @urb: URB to be checked
//
// Return: 1 if @urb describes an OUT transfer (host-to-device),
// otherwise 0.
//
extern "C" {
    pub fn usb_pipe_type_check(dev: *mut usb_device, pipe: c_uint) -> c_int;
}
extern "C" {
    pub fn usb_urb_ep_type_check(urb: *const urb) -> c_int;
}
// -------------------------------------------------------------------
// SYNCHRONOUS CALL SUPPORT
// -------------------------------------------------------------------
// Maximum value allowed for timeout in synchronous routines below

// wrappers around usb_control_msg() for the most common standard requests
// wrappers that also update important state inside usbcore
extern "C" {
    pub fn usb_clear_halt(dev: *mut usb_device, pipe: c_int) -> c_int;
}
extern "C" {
    pub fn usb_reset_configuration(dev: *mut usb_device) -> c_int;
}
extern "C" {
    pub fn usb_set_interface(dev: *mut usb_device, ifnum: c_int, alternate: c_int) -> c_int;
}
extern "C" {
    pub fn usb_reset_endpoint(dev: *mut usb_device, epaddr: c_uint);
}
// this request isn't really synchronous, but it belongs with the others
extern "C" {
    pub fn usb_driver_set_configuration(udev: *mut usb_device, config: c_int) -> c_int;
}
// choose and set configuration for device
extern "C" {
    pub fn usb_choose_configuration(udev: *mut usb_device) -> c_int;
}
extern "C" {
    pub fn usb_set_configuration(dev: *mut usb_device, configuration: c_int) -> c_int;
}
//
// timeouts, in milliseconds, used for sending/receiving control messages
// they typically complete within a few frames (msec) after they're issued
// USB identifies 5 second timeouts, maybe more in a few cases, and a few
// slow devices (like some MGE Ellipse UPSes) actually push that limit.
//
pub const USB_CTRL_GET_TIMEOUT: c_int = 5000;
pub const USB_CTRL_SET_TIMEOUT: c_int = 5000;
//
// struct usb_sg_request - support for scatter/gather I/O
// @status: zero indicates success, else negative errno
// @bytes: counts bytes transferred.
//
// These requests are initialized using usb_sg_init(), and then are used
// as request handles passed to usb_sg_wait() or usb_sg_cancel().  Most
// members of the request object aren't for driver access.
//
// The status and bytecount values are valid only after usb_sg_wait()
// returns.  If the status is zero, then the bytecount matches the total
// from the request.
//
// After an error completion, drivers may need to clear a halt condition
// on the endpoint.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_sg_request {
    pub status: c_int,
    pub bytes: usize,
// private:
// members below are private to usbcore,
// and are not provided for driver access!
//
    pub lock: spinlock_t,
    pub dev: *mut usb_device,
    pub pipe: c_int,
    pub entries: c_int,
    pub urbs: *mut urb,
    pub count: c_int,
    pub complete: completion,
}

extern "C" {
    pub fn usb_sg_cancel(io: *mut usb_sg_request);
}
extern "C" {
    pub fn usb_sg_wait(io: *mut usb_sg_request);
}
// -----------------------------------------------------------------------
//
// For various legacy reasons, Linux has a small cookie that's paired with
// a struct usb_device to identify an endpoint queue.  Queue characteristics
// are defined by the endpoint's descriptor.  This cookie is called a "pipe",
// an unsigned int encoded as:
//
// - direction:	bit 7		(0 = Host-to-Device [Out],
// 1 = Device-to-Host [In] ...
// like endpoint bEndpointAddress)
// - device address:	bits 8-14       ... bit positions known to uhci-hcd
// - endpoint:		bits 15-18      ... bit positions known to uhci-hcd
// - pipe type:	bits 30-31	(00 = isochronous, 01 = interrupt,
// 10 = control, 11 = bulk)
//
// Given the device address and endpoint descriptor, pipes are redundant.
//
// NOTE:  these are not the standard USB_ENDPOINT_XFER_* values!!
// (yet ... they're the values used by usbfs)
pub const PIPE_ISOCHRONOUS: c_int = 0;
pub const PIPE_INTERRUPT: c_int = 1;
pub const PIPE_CONTROL: c_int = 2;
pub const PIPE_BULK: c_int = 3;

// Create various pipes...

// NOTE:  only 0x07ff bits are for packet size...
extern "C" {
    pub fn usb_endpoint_maxp(_arg: &ep->desc) -> return;
}
// translate USB error codes to codes user space understands
// Events from the usb core
pub const USB_DEVICE_ADD: c_uint = 0x0001;
pub const USB_DEVICE_REMOVE: c_uint = 0x0002;
pub const USB_BUS_ADD: c_uint = 0x0003;
pub const USB_BUS_REMOVE: c_uint = 0x0004;
extern "C" {
    pub fn usb_register_notify(nb: *mut notifier_block);
}
extern "C" {
    pub fn usb_unregister_notify(nb: *mut notifier_block);
}
// debugfs stuff
// LED triggers
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum usb_led_event {
    USB_LED_EVENT_HOST = 0,
    USB_LED_EVENT_GADGET = 1,
}

extern "C" {
    pub fn usb_led_activity(ev: usb_led_event);
}

