//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/usb/composite.h
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
// composite.h -- framework for usb gadgets which are composite devices
//
// Copyright (C) 2006-2008 David Brownell
//
// This framework is an optional layer on top of the USB Gadget interface,
// making it easier to build (a) Composite devices, supporting multiple
// functions within any single configuration, and (b) Multi-configuration
// devices, also supporting multiple functions but without necessarily
// having more than one function per configuration.
//
// Example:  a device with a single configuration supporting both network
// link and mass storage functions is a composite device.  Those functions
// might alternatively be packaged in individual configurations, but in
// the composite model the host can use both functions at the same time.
//

//
// USB function drivers should return USB_GADGET_DELAYED_STATUS if they
// wish to delay the data/status stages of the control transfer till they
// are ready. The control transfer will then be kept from completing till
// all the function drivers that requested for USB_GADGET_DELAYED_STAUS
// invoke usb_composite_setup_continue().
//
// NOTE: USB_GADGET_DELAYED_STATUS must not be used in UDC drivers: they
// must delay completing the status stage for 0-length control transfers
// regardless of the whether USB_GADGET_DELAYED_STATUS is returned from
// the gadget driver's setup() callback.
// Currently, a number of UDC drivers rely on USB_GADGET_DELAYED_STATUS,
// which is a bug. These drivers must be fixed and USB_GADGET_DELAYED_STATUS
// must be contained within the composite framework.
//
pub const USB_GADGET_DELAYED_STATUS: c_uint = 0x7fff	/* Impossibly large value */;
// big enough to hold our biggest descriptor
pub const USB_COMP_EP0_BUFSIZ: c_int = 4096;
// OS feature descriptor length <= 4kB
pub const USB_COMP_EP0_OS_DESC_BUFSIZ: c_int = 4096;

//
// struct usb_os_desc_ext_prop - describes one "Extended Property"
// @entry: used to keep a list of extended properties
// @type: Extended Property type
// @name_len: Extended Property unicode name length, including terminating '\0'
// @name: Extended Property name
// @data_len: Length of Extended Property blob (for unicode store double len)
// @data: Extended Property blob
// @item: Represents this Extended Property in configfs
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_os_desc_ext_prop {
    pub entry: list_head,
    pub type: u8,
    pub name_len: c_int,
    pub name: *mut c_char,
    pub data_len: c_int,
    pub data: *mut c_char,
    pub item: config_item,
}

//
// struct usb_os_desc - describes OS descriptors associated with one interface
// @ext_compat_id: 16 bytes of "Compatible ID" and "Subcompatible ID"
// @ext_prop: Extended Properties list
// @ext_prop_len: Total length of Extended Properties blobs
// @ext_prop_count: Number of Extended Properties
// @opts_mutex: Optional mutex protecting config data of a usb_function_instance
// @group: Represents OS descriptors associated with an interface in configfs
// @owner: Module associated with this OS descriptor
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_os_desc {
    pub ext_compat_id: *mut c_char,
    pub ext_prop: list_head,
    pub ext_prop_len: c_int,
    pub ext_prop_count: c_int,
    pub opts_mutex: *mut mutex,
    pub group: config_group,
    pub owner: *mut module,
}

//
// struct usb_os_desc_table - describes OS descriptors associated with one
// interface of a usb_function
// @if_id: Interface id
// @os_desc: "Extended Compatibility ID" and "Extended Properties" of the
// interface
//
// Each interface can have at most one "Extended Compatibility ID" and a
// number of "Extended Properties".
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_os_desc_table {
    pub if_id: c_int,
    pub os_desc: *mut usb_os_desc,
}

//
// struct usb_function - describes one function of a configuration
// @name: For diagnostics, identifies the function.
// @strings: tables of strings, keyed by identifiers assigned during bind()
// and by language IDs provided in control requests
// @fs_descriptors: Table of full (or low) speed descriptors, using interface and
// string identifiers assigned during @bind().  If this pointer is null,
// the function will not be available at full speed (or at low speed).
// @hs_descriptors: Table of high speed descriptors, using interface and
// string identifiers assigned during @bind().  If this pointer is null,
// the function will not be available at high speed.
// @ss_descriptors: Table of super speed descriptors, using interface and
// string identifiers assigned during @bind(). If this
// pointer is null after initiation, the function will not
// be available at super speed.
// @ssp_descriptors: Table of super speed plus descriptors, using
// interface and string identifiers assigned during @bind(). If
// this pointer is null after initiation, the function will not
// be available at super speed plus.
// @config: assigned when @usb_add_function() is called; this is the
// configuration with which this function is associated.
// @os_desc_table: Table of (interface id, os descriptors) pairs. The function
// can expose more than one interface. If an interface is a member of
// an IAD, only the first interface of IAD has its entry in the table.
// @os_desc_n: Number of entries in os_desc_table
// @bind: Before the gadget can register, all of its functions bind() to the
// available resources including string and interface identifiers used
// in interface or class descriptors; endpoints; I/O buffers; and so on.
// @unbind: Reverses @bind; called as a side effect of unregistering the
// driver which added this function.
// @free_func: free the struct usb_function.
// @mod: (internal) points to the module that created this structure.
// @set_alt: (REQUIRED) Reconfigures altsettings; function drivers may
// initialize usb_ep.driver data at this time (when it is used).
// Note that setting an interface to its current altsetting resets
// interface state, and that all interfaces have a disabled state.
// @get_alt: Returns the active altsetting.  If this is not provided,
// then only altsetting zero is supported.
// @disable: (REQUIRED) Indicates the function should be disabled.  Reasons
// include host resetting or reconfiguring the gadget, and disconnection.
// @setup: Used for interface-specific control requests.
// @req_match: Tests if a given class request can be handled by this function.
// @suspend: Notifies functions when the host stops sending USB traffic.
// @resume: Notifies functions when the host restarts USB traffic.
// @get_status: Returns function status as a reply to
// GetStatus() request when the recipient is Interface.
// @func_suspend: callback to be called when
// SetFeature(FUNCTION_SUSPEND) is reseived
// @func_suspended: Indicates whether the function is in function suspend state.
// @func_wakeup_armed: Indicates whether the function is armed by the host for
// wakeup signaling.
//
// A single USB function uses one or more interfaces, and should in most
// cases support operation at both full and high speeds.  Each function is
// associated by @usb_add_function() with a one configuration; that function
// causes @bind() to be called so resources can be allocated as part of
// setting up a gadget driver.  Those resources include endpoints, which
// should be allocated using @usb_ep_autoconfig().
//
// To support dual speed operation, a function driver provides descriptors
// for both high and full speed operation.  Except in rare cases that don't
// involve bulk endpoints, each speed needs different endpoint descriptors.
//
// Function drivers choose their own strategies for managing instance data.
// The simplest strategy just declares it "static', which means the function
// can only be activated once.  If the function needs to be exposed in more
// than one configuration at a given speed, it needs to support multiple
// usb_function structures (one for each configuration).
//
// A more complex strategy might encapsulate a @usb_function structure inside
// a driver-specific instance structure to allows multiple activations.  An
// example of multiple activations might be a CDC ACM function that supports
// two or more distinct instances within the same configuration, providing
// several independent logical data links to a USB host.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_function {
    pub name: *const c_char,
    pub strings: *mut usb_gadget_strings,
    pub fs_descriptors: *mut usb_descriptor_header,
    pub hs_descriptors: *mut usb_descriptor_header,
    pub ss_descriptors: *mut usb_descriptor_header,
    pub ssp_descriptors: *mut usb_descriptor_header,
    pub config: *mut usb_configuration,
    pub os_desc_table: *mut usb_os_desc_table,
    pub os_desc_n: unsigned,
// REVISIT:  bind() functions can be marked __init, which
// makes trouble for section mismatch analysis.  See if
// we can't restructure things to avoid mismatching.
// Related:  unbind() may kfree() but bind() won't...
//
// configuration management:  bind/unbind
    pub ): *mut usb_function,
    pub ): *mut usb_function,
    pub f): *mut *mut void (free_func)(struct usb_function,
    pub mod: *mut module,
// runtime state management
    pub alt): unsigned interface, unsigned,
    pub interface): unsigned,
    pub ): *mut *mut void (disable)(struct usb_function,
    pub ): *const usb_ctrlrequest,
    pub config0): bool,
    pub ): *mut *mut void (suspend)(struct usb_function,
    pub ): *mut *mut void (resume)(struct usb_function,
// USB 3.0 additions
    pub ): *mut *mut int (get_status)(struct usb_function,
    pub suspend_opt): u8,
    pub func_suspended: bool,
    pub func_wakeup_armed: bool,
// private:
// internals
    pub list: list_head,
    pub 32): DECLARE_BITMAP(endpoints,,
    pub fi: *mut usb_function_instance,
    pub bind_deactivated:1: c_uint,
}

extern "C" {
    pub fn usb_add_function(: *mut usb_configuration, : *mut usb_function) -> c_int;
}
extern "C" {
    pub fn usb_function_deactivate(: *mut usb_function) -> c_int;
}
extern "C" {
    pub fn usb_function_activate(: *mut usb_function) -> c_int;
}
extern "C" {
    pub fn usb_interface_id(: *mut usb_configuration, : *mut usb_function) -> c_int;
}
extern "C" {
    pub fn usb_func_wakeup(func: *mut usb_function) -> c_int;
}
pub const MAX_CONFIG_INTERFACES: c_int = 32;
//
// struct usb_configuration - represents one gadget configuration
// @label: For diagnostics, describes the configuration.
// @strings: Tables of strings, keyed by identifiers assigned during @bind()
// and by language IDs provided in control requests.
// @descriptors: Table of descriptors preceding all function descriptors.
// Examples include OTG and vendor-specific descriptors.
// @unbind: Reverses @bind; called as a side effect of unregistering the
// driver which added this configuration.
// @setup: Used to delegate control requests that aren't handled by standard
// device infrastructure or directed at a specific interface.
// @bConfigurationValue: Copied into configuration descriptor.
// @iConfiguration: Copied into configuration descriptor.
// @bmAttributes: Copied into configuration descriptor.
// @MaxPower: Power consumption in mA. Used to compute bMaxPower in the
// configuration descriptor after considering the bus speed.
// @cdev: assigned by @usb_add_config() before calling @bind(); this is
// the device associated with this configuration.
//
// Configurations are building blocks for gadget drivers structured around
// function drivers.  Simple USB gadgets require only one function and one
// configuration, and handle dual-speed hardware by always providing the same
// functionality.  Slightly more complex gadgets may have more than one
// single-function configuration at a given speed; or have configurations
// that only work at one speed.
//
// Composite devices are, by definition, ones with configurations which
// include more than one function.
//
// The lifecycle of a usb_configuration includes allocation, initialization
// of the fields described above, and calling @usb_add_config() to set up
// internal data and bind it to a specific device.  The configuration's
// @bind() method is then used to initialize all the functions and then
// call @usb_add_function() for them.
//
// Those functions would normally be independent of each other, but that's
// not mandatory.  CDC WMC devices are an example where functions often
// depend on other functions, with some functions subsidiary to others.
// Such interdependency may be managed in any way, so long as all of the
// descriptors complete by the time the composite driver returns from
// its bind() routine.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_configuration {
    pub label: *const c_char,
    pub strings: *mut usb_gadget_strings,
    pub descriptors: *const usb_descriptor_header,
// REVISIT:  bind() functions can be marked __init, which
// makes trouble for section mismatch analysis.  See if
// we can't restructure things to avoid mismatching...
//
// configuration management: unbind/setup
    pub ): *mut *mut void (unbind)(struct usb_configuration,
    pub ): *const usb_ctrlrequest,
// fields in the config descriptor
    pub bConfigurationValue: u8,
    pub iConfiguration: u8,
    pub bmAttributes: u8,
    pub MaxPower: u16,
    pub cdev: *mut usb_composite_dev,
// private:
// internals
    pub list: list_head,
    pub functions: list_head,
    pub next_interface_id: u8,
    pub superspeed:1: unsigned,
    pub highspeed:1: unsigned,
    pub fullspeed:1: unsigned,
    pub superspeed_plus:1: unsigned,
    pub interface: [*mut usb_function; MAX_CONFIG_INTERFACES],
}

// predefined index for usb_composite_driver
//
// struct usb_composite_driver - groups configurations into a gadget
// @name: For diagnostics, identifies the driver.
// @dev: Template descriptor for the device, including default device
// identifiers.
// @strings: tables of strings, keyed by identifiers assigned during @bind
// and language IDs provided in control requests. Note: The first entries
// are predefined. The first entry that may be used is
// USB_GADGET_FIRST_AVAIL_IDX
// @max_speed: Highest speed the driver supports.
// @needs_serial: set to 1 if the gadget needs userspace to provide
// a serial number.  If one is not provided, warning will be printed.
// @bind: (REQUIRED) Used to allocate resources that are shared across the
// whole device, such as string IDs, and add its configurations using
// @usb_add_config(). This may fail by returning a negative errno
// value; it should return zero on successful initialization.
// @unbind: Reverses @bind; called as a side effect of unregistering
// this driver.
// @disconnect: optional driver disconnect method
// @suspend: Notifies when the host stops sending USB traffic,
// after function notifications
// @resume: Notifies configuration when the host restarts USB traffic,
// before function notifications
// @gadget_driver: Gadget driver controlling this driver
//
// Devices default to reporting self powered operation.  Devices which rely
// on bus powered operation should report this in their @bind method.
//
// Before returning from @bind, various fields in the template descriptor
// may be overridden.  These include the idVendor/idProduct/bcdDevice values
// normally to bind the appropriate host side driver, and the three strings
// (iManufacturer, iProduct, iSerialNumber) normally used to provide user
// meaningful device identifiers.  (The strings will not be defined unless
// they are defined in @dev and @strings.)  The correct ep0 maxpacket size
// is also reported, as defined by the underlying controller driver.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_composite_driver {
    pub name: *const c_char,
    pub dev: *const usb_device_descriptor,
    pub strings: *mut usb_gadget_strings,
    pub max_speed: usb_device_speed,
    pub needs_serial:1: unsigned,
    pub cdev): *mut *mut int (bind)(struct usb_composite_dev,
    pub ): *mut *mut int (unbind)(struct usb_composite_dev,
    pub ): *mut *mut void (disconnect)(struct usb_composite_dev,
// global suspend hooks
    pub ): *mut *mut void (suspend)(struct usb_composite_dev,
    pub ): *mut *mut void (resume)(struct usb_composite_dev,
    pub gadget_driver: usb_gadget_driver,
}

extern "C" {
    pub fn usb_composite_probe(driver: *mut usb_composite_driver) -> c_int;
}
extern "C" {
    pub fn usb_composite_unregister(driver: *mut usb_composite_driver);
}
//
// module_usb_composite_driver() - Helper macro for registering a USB gadget
// composite driver
// @__usb_composite_driver: usb_composite_driver struct
//
// Helper macro for USB gadget composite drivers which do not do anything
// special in module init/exit. This eliminates a lot of boilerplate. Each
// module may only use this macro once, and calling it replaces module_init()
// and module_exit()
//

extern "C" {
    pub fn usb_composite_setup_continue(cdev: *mut usb_composite_dev);
}
extern "C" {
    pub fn composite_dev_cleanup(cdev: *mut usb_composite_dev);
}
extern "C" {
    pub fn container_of(_arg: gdrv, usb_composite_driver: struct, _arg: gadget_driver) -> return;
}
pub const OS_STRING_QW_SIGN_LEN: c_int = 14;
pub const OS_STRING_IDX: c_uint = 0xEE;
//
// struct usb_composite_dev - represents one composite usb gadget
// @gadget: read-only, abstracts the gadget's usb peripheral controller
// @req: used for control responses; buffer is pre-allocated
// @os_desc_req: used for OS descriptors responses; buffer is pre-allocated
// @config: the currently active configuration
// @qw_sign: qwSignature part of the OS string
// @b_vendor_code: bMS_VendorCode part of the OS string
// @use_os_string: false by default, interested gadgets set it
// @bcd_webusb_version: 0x0100 by default, WebUSB specification version
// @b_webusb_vendor_code: 0x0 by default, vendor code for WebUSB
// @landing_page: empty by default, landing page to announce in WebUSB
// @use_webusb: false by default, interested gadgets set it
// @os_desc_config: the configuration to be used with OS descriptors
// @setup_pending: true when setup request is queued but not completed
// @os_desc_pending: true when os_desc request is queued but not completed
//
// One of these devices is allocated and initialized before the
// associated device driver's bind() is called.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_composite_dev {
    pub gadget: *mut usb_gadget,
    pub req: *mut usb_request,
    pub os_desc_req: *mut usb_request,
    pub config: *mut usb_configuration,
// OS String is a custom (yet popular) extension to the USB standard.
    pub qw_sign: [u8; OS_STRING_QW_SIGN_LEN],
    pub b_vendor_code: u8,
    pub os_desc_config: *mut usb_configuration,
    pub use_os_string:1: c_uint,
// WebUSB
    pub bcd_webusb_version: u16,
    pub b_webusb_vendor_code: u8,
    pub landing_page: [c_char; WEBUSB_URL_RAW_MAX_LENGTH],
    pub use_webusb:1: c_uint,
// private:
// internals
    pub suspended:1: c_uint,
    pub desc: usb_device_descriptor,
    pub configs: list_head,
    pub gstrings: list_head,
    pub driver: *mut usb_composite_driver,
    pub next_string_id: u8,
    pub def_manufacturer: *mut c_char,
    pub usb_strings: *mut usb_string,
// the gadget driver won't enable the data pullup
// while the deactivation count is nonzero.
//
    pub deactivations: unsigned,
// the composite driver won't complete the control transfer's
// data/status stages till delayed_status is zero.
//
    pub delayed_status: c_int,
// protects deactivations and delayed_status counts
    pub lock: spinlock_t,
// public:
    pub setup_pending:1: c_uint,
    pub os_desc_pending:1: c_uint,
}

extern "C" {
    pub fn usb_string_id(c: *mut usb_composite_dev) -> c_int;
}
extern "C" {
    pub fn usb_string_ids_n(c: *mut usb_composite_dev, n: unsigned) -> c_int;
}
extern "C" {
    pub fn composite_disconnect(gadget: *mut usb_gadget);
}
extern "C" {
    pub fn composite_reset(gadget: *mut usb_gadget);
}
extern "C" {
    pub fn composite_suspend(gadget: *mut usb_gadget);
}
extern "C" {
    pub fn composite_resume(gadget: *mut usb_gadget);
}
//
// Some systems will need runtime overrides for the  product identifiers
// published in the device descriptor, either numbers or strings or both.
// String parameters are in UTF-8 (superset of ASCII's 7 bit characters).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_composite_overwrite {
    pub idVendor: u16,
    pub idProduct: u16,
    pub bcdDevice: u16,
    pub serial_number: *mut c_char,
    pub manufacturer: *mut c_char,
    pub product: *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_function_driver {
    pub name: *const c_char,
    pub mod: *mut module,
    pub list: list_head,
    pub (*alloc_inst)(void): *mut usb_function_instance,
    pub inst): *mut *mut *mut usb_function (alloc_func)(usb_function_instance,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_function_instance {
    pub group: config_group,
    pub cfs_list: list_head,
    pub fd: *mut usb_function_driver,
    pub name): *const c_char,
    pub inst): *mut *mut void (free_func_inst)(struct usb_function_instance,
}

extern "C" {
    pub fn usb_function_unregister(f: *mut usb_function_driver);
}
extern "C" {
    pub fn usb_function_register(newf: *mut usb_function_driver) -> c_int;
}
extern "C" {
    pub fn usb_put_function_instance(fi: *mut usb_function_instance);
}
extern "C" {
    pub fn usb_put_function(f: *mut usb_function);
}
extern "C" {
    pub fn usb_remove_function(c: *mut usb_configuration, f: *mut usb_function);
}

// messaging utils

