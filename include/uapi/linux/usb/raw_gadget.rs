//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/usb/raw_gadget.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
//
// USB Raw Gadget driver.
//
// See Documentation/usb/raw-gadget.rst for more details.
//

// Maximum length of driver_name/device_name in the usb_raw_init struct.
pub const UDC_NAME_LENGTH_MAX: c_int = 128;
//
// struct usb_raw_init - argument for USB_RAW_IOCTL_INIT ioctl.
// @speed: The speed of the emulated USB device, takes the same values as
// the usb_device_speed enum: USB_SPEED_FULL, USB_SPEED_HIGH, etc.
// @driver_name: The name of the UDC driver.
// @device_name: The name of a UDC instance.
//
// The last two fields identify a UDC the gadget driver should bind to.
// For example, Dummy UDC has "dummy_udc" as its driver_name and "dummy_udc.N"
// as its device_name, where N in the index of the Dummy UDC instance.
// At the same time the dwc2 driver that is used on Raspberry Pi Zero, has
// "20980000.usb" as both driver_name and device_name.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_raw_init {
    pub driver_name: [__u8; UDC_NAME_LENGTH_MAX],
    pub device_name: [__u8; UDC_NAME_LENGTH_MAX],
    pub speed: __u8,
}

// The type of event fetched with the USB_RAW_IOCTL_EVENT_FETCH ioctl.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum usb_raw_event_type {
    USB_RAW_EVENT_INVALID = 0,

// This event is queued when the driver has bound to a UDC.
    USB_RAW_EVENT_CONNECT = 1,

// This event is queued when a new control request arrived to ep0.
    USB_RAW_EVENT_CONTROL = 2,

//
// These events are queued when the gadget driver is suspended,
// resumed, reset, or disconnected. Note that some UDCs (e.g. dwc2)
// report a disconnect event instead of a reset.
//
    USB_RAW_EVENT_SUSPEND = 3,
    USB_RAW_EVENT_RESUME = 4,
    USB_RAW_EVENT_RESET = 5,
    USB_RAW_EVENT_DISCONNECT = 6,

// The list might grow in the future.
}

//
// struct usb_raw_event - argument for USB_RAW_IOCTL_EVENT_FETCH ioctl.
// @type: The type of the fetched event.
// @length: Length of the data buffer. Updated by the driver and set to the
// actual length of the fetched event data.
// @data: A buffer to store the fetched event data.
//
// The fetched event data buffer contains struct usb_ctrlrequest for
// USB_RAW_EVENT_CONTROL and is empty for other events.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_raw_event {
    pub type: __u32,
    pub length: __u32,
    pub data: [__u8; ],
}

pub const USB_RAW_IO_FLAGS_ZERO: c_uint = 0x0001;
pub const USB_RAW_IO_FLAGS_MASK: c_uint = 0x0001;
//
// struct usb_raw_ep_io - argument for USB_RAW_IOCTL_EP0/EP_WRITE/READ ioctls.
// @ep: Endpoint handle as returned by USB_RAW_IOCTL_EP_ENABLE for
// USB_RAW_IOCTL_EP_WRITE/READ. Ignored for USB_RAW_IOCTL_EP0_WRITE/READ.
// @flags: When USB_RAW_IO_FLAGS_ZERO is specified, the zero flag is set on
// the submitted USB request, see include/linux/usb/gadget.h for details.
// @length: Length of data.
// @data: Data to send for USB_RAW_IOCTL_EP0/EP_WRITE. Buffer to store received
// data for USB_RAW_IOCTL_EP0/EP_READ.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_raw_ep_io {
    pub ep: __u16,
    pub flags: __u16,
    pub length: __u32,
    pub data: [__u8; ],
}

// Maximum number of non-control endpoints in struct usb_raw_eps_info.
pub const USB_RAW_EPS_NUM_MAX: c_int = 30;
// Maximum length of UDC endpoint name in struct usb_raw_ep_info.
pub const USB_RAW_EP_NAME_MAX: c_int = 16;
// Used as addr in struct usb_raw_ep_info if endpoint accepts any address.
pub const USB_RAW_EP_ADDR_ANY: c_uint = 0xff;
//
// struct usb_raw_ep_caps - exposes endpoint capabilities from struct usb_ep
// (technically from its member struct usb_ep_caps).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_raw_ep_caps {
    pub 1: __u32 type_control :,
    pub 1: __u32 type_iso :,
    pub 1: __u32 type_bulk :,
    pub 1: __u32 type_int :,
    pub 1: __u32 dir_in :,
    pub 1: __u32 dir_out :,
}

//
// struct usb_raw_ep_limits - exposes endpoint limits from struct usb_ep.
// @maxpacket_limit: Maximum packet size value supported by this endpoint.
// @max_streams: maximum number of streams supported by this endpoint
// (actual number is 2^n).
// @reserved: Empty, reserved for potential future extensions.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_raw_ep_limits {
    pub maxpacket_limit: __u16,
    pub max_streams: __u16,
    pub reserved: __u32,
}

//
// struct usb_raw_ep_info - stores information about a gadget endpoint.
// @name: Name of the endpoint as it is defined in the UDC driver.
// @addr: Address of the endpoint that must be specified in the endpoint
// descriptor passed to USB_RAW_IOCTL_EP_ENABLE ioctl.
// @caps: Endpoint capabilities.
// @limits: Endpoint limits.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_raw_ep_info {
    pub name: [__u8; USB_RAW_EP_NAME_MAX],
    pub addr: __u32,
    pub caps: usb_raw_ep_caps,
    pub limits: usb_raw_ep_limits,
}

//
// struct usb_raw_eps_info - argument for USB_RAW_IOCTL_EPS_INFO ioctl.
// eps: Structures that store information about non-control endpoints.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_raw_eps_info {
    pub eps: [usb_raw_ep_info; USB_RAW_EPS_NUM_MAX],
}

//
// Initializes a Raw Gadget instance.
// Accepts a pointer to the usb_raw_init struct as an argument.
// Returns 0 on success or negative error code on failure.
//

//
// Instructs Raw Gadget to bind to a UDC and start emulating a USB device.
// Returns 0 on success or negative error code on failure.
//

//
// A blocking ioctl that waits for an event and returns fetched event data to
// the user.
// Accepts a pointer to the usb_raw_event struct.
// Returns 0 on success or negative error code on failure.
//

//
// Queues an IN (OUT for READ) request as a response to the last setup request
// received on endpoint 0 (provided that was an IN (OUT for READ) request), and
// waits until the request is completed. Copies received data to user for READ.
// Accepts a pointer to the usb_raw_ep_io struct as an argument.
// Returns length of transferred data on success or negative error code on
// failure.
//

//
// Finds an endpoint that satisfies the parameters specified in the provided
// descriptors (address, transfer type, etc.) and enables it.
// Accepts a pointer to the usb_raw_ep_descs struct as an argument.
// Returns enabled endpoint handle on success or negative error code on failure.
//

//
// Disables specified endpoint.
// Accepts endpoint handle as an argument.
// Returns 0 on success or negative error code on failure.
//

//
// Queues an IN (OUT for READ) request as a response to the last setup request
// received on endpoint usb_raw_ep_io.ep (provided that was an IN (OUT for READ)
// request), and waits until the request is completed. Copies received data to
// user for READ.
// Accepts a pointer to the usb_raw_ep_io struct as an argument.
// Returns length of transferred data on success or negative error code on
// failure.
//

//
// Switches the gadget into the configured state.
// Returns 0 on success or negative error code on failure.
//

//
// Constrains UDC VBUS power usage.
// Accepts current limit in 2 mA units as an argument.
// Returns 0 on success or negative error code on failure.
//

//
// Fills in the usb_raw_eps_info structure with information about non-control
// endpoints available for the currently connected UDC.
// Returns the number of available endpoints on success or negative error code
// on failure.
//

//
// Stalls a pending control request on endpoint 0.
// Returns 0 on success or negative error code on failure.
//

//
// Sets or clears halt or wedge status of the endpoint.
// Accepts endpoint handle as an argument.
// Returns 0 on success or negative error code on failure.
//

