//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/usb/ch9.h
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
// This file holds USB constants and structures that are needed for
// USB device APIs.  These are used by the USB device model, which is
// defined in chapter 9 of the USB 2.0 specification and in the
// Wireless USB 1.0 spec (now defunct).  Linux has several APIs in C that
// need these:
//
// - the master/host side Linux-USB kernel driver API;
// - the "usbfs" user space API; and
// - the Linux "gadget" slave/device/peripheral side driver API.
//
// USB 2.0 adds an additional "On The Go" (OTG) mode, which lets systems
// act either as a USB master/host or as a USB slave/device.  That means
// the master and slave side APIs benefit from working well together.
//
// Note all descriptors are declared '__attribute__((packed))' so that:
//
// [a] they never get padded, either internally (USB spec writers
// probably handled that) or externally;
//
// [b] so that accessing bigger-than-a-bytes fields will never
// generate bus errors on any platform, even when the location of
// its descriptor inside a bundle isn't "naturally aligned", and
//
// [c] for consistency, removing all doubt even when it appears to
// someone that the two other points are non-issues for that
// particular descriptor type.
//

// -------------------------------------------------------------------------
// CONTROL REQUEST SUPPORT
//
// USB directions
//
// This bit flag is used in endpoint descriptors' bEndpointAddress field.
// It's also one of three fields in control requests bRequestType.
//

pub const USB_DIR_IN: c_uint = 0x80		/* to host */;
//
// USB types, the second of three bRequestType fields
//

//
// USB recipients, the third of three bRequestType fields
//
pub const USB_RECIP_MASK: c_uint = 0x1f;
pub const USB_RECIP_DEVICE: c_uint = 0x00;
pub const USB_RECIP_INTERFACE: c_uint = 0x01;
pub const USB_RECIP_ENDPOINT: c_uint = 0x02;
pub const USB_RECIP_OTHER: c_uint = 0x03;
// From Wireless USB 1.0
pub const USB_RECIP_PORT: c_uint = 0x04;
pub const USB_RECIP_RPIPE: c_uint = 0x05;
//
// Standard requests, for the bRequest field of a SETUP packet.
//
// These are qualified by the bRequestType field, so that for example
// TYPE_CLASS or TYPE_VENDOR specific feature flags could be retrieved
// by a GET_STATUS request.
//
pub const USB_REQ_GET_STATUS: c_uint = 0x00;
pub const USB_REQ_CLEAR_FEATURE: c_uint = 0x01;
pub const USB_REQ_SET_FEATURE: c_uint = 0x03;
pub const USB_REQ_SET_ADDRESS: c_uint = 0x05;
pub const USB_REQ_GET_DESCRIPTOR: c_uint = 0x06;
pub const USB_REQ_SET_DESCRIPTOR: c_uint = 0x07;
pub const USB_REQ_GET_CONFIGURATION: c_uint = 0x08;
pub const USB_REQ_SET_CONFIGURATION: c_uint = 0x09;
pub const USB_REQ_GET_INTERFACE: c_uint = 0x0A;
pub const USB_REQ_SET_INTERFACE: c_uint = 0x0B;
pub const USB_REQ_SYNCH_FRAME: c_uint = 0x0C;
pub const USB_REQ_SET_SEL: c_uint = 0x30;
pub const USB_REQ_SET_ISOCH_DELAY: c_uint = 0x31;
pub const USB_REQ_SET_ENCRYPTION: c_uint = 0x0D	/* Wireless USB */;
pub const USB_REQ_GET_ENCRYPTION: c_uint = 0x0E;
pub const USB_REQ_RPIPE_ABORT: c_uint = 0x0E;
pub const USB_REQ_SET_HANDSHAKE: c_uint = 0x0F;
pub const USB_REQ_RPIPE_RESET: c_uint = 0x0F;
pub const USB_REQ_GET_HANDSHAKE: c_uint = 0x10;
pub const USB_REQ_SET_CONNECTION: c_uint = 0x11;
pub const USB_REQ_SET_SECURITY_DATA: c_uint = 0x12;
pub const USB_REQ_GET_SECURITY_DATA: c_uint = 0x13;
pub const USB_REQ_SET_WUSB_DATA: c_uint = 0x14;
pub const USB_REQ_LOOPBACK_DATA_WRITE: c_uint = 0x15;
pub const USB_REQ_LOOPBACK_DATA_READ: c_uint = 0x16;
pub const USB_REQ_SET_INTERFACE_DS: c_uint = 0x17;
pub const USB_REQ_AUTH_IN: c_uint = 0x18;
pub const USB_REQ_AUTH_OUT: c_uint = 0x19;
// specific requests for USB Power Delivery
pub const USB_REQ_GET_PARTNER_PDO: c_int = 20;
pub const USB_REQ_GET_BATTERY_STATUS: c_int = 21;
pub const USB_REQ_SET_PDO: c_int = 22;
pub const USB_REQ_GET_VDM: c_int = 23;
pub const USB_REQ_SEND_VDM: c_int = 24;
// The Link Power Management (LPM) ECN defines USB_REQ_TEST_AND_SET command,
// used by hubs to put ports into a new L1 suspend state, except that it
// forgot to define its number ...
//
// USB feature flags are written using USB_REQ_{CLEAR,SET}_FEATURE, and
// are read as a bit array returned by USB_REQ_GET_STATUS.  (So there
// are at most sixteen features of each type.)  Hubs may also support a
// new USB_REQ_TEST_AND_SET_FEATURE to put ports into L1 suspend.
//

//
// Test Mode Selectors
// See USB 2.0 spec Table 9-7
//
pub const USB_TEST_J: c_int = 1;
pub const USB_TEST_K: c_int = 2;
pub const USB_TEST_SE0_NAK: c_int = 3;
pub const USB_TEST_PACKET: c_int = 4;
pub const USB_TEST_FORCE_ENABLE: c_int = 5;
// Status Type
pub const USB_STATUS_TYPE_STANDARD: c_int = 0;
pub const USB_STATUS_TYPE_PTM: c_int = 1;
//
// New Feature Selectors as added by USB 3.0
// See USB 3.0 spec Table 9-7
//

pub const USB_INTR_FUNC_SUSPEND_OPT_MASK: c_uint = 0xFF00;
//
// Suspend Options, Table 9-8 USB 3.0 spec
//

//
// Interface status, Figure 9-5 USB 3.0 spec
//
pub const USB_INTRF_STAT_FUNC_RW_CAP: c_int = 1;
pub const USB_INTRF_STAT_FUNC_RW: c_int = 2;

// Bit array elements as returned by the USB_REQ_GET_STATUS request.

//
// Feature selectors from Table 9-8 USB Power Delivery spec
//
pub const USB_DEVICE_BATTERY_WAKE_MASK: c_int = 40;
pub const USB_DEVICE_OS_IS_PD_AWARE: c_int = 41;
pub const USB_DEVICE_POLICY_MODE: c_int = 42;
pub const USB_PORT_PR_SWAP: c_int = 43;
pub const USB_PORT_GOTO_MIN: c_int = 44;
pub const USB_PORT_RETURN_POWER: c_int = 45;
pub const USB_PORT_ACCEPT_PD_REQUEST: c_int = 46;
pub const USB_PORT_REJECT_PD_REQUEST: c_int = 47;
pub const USB_PORT_PORT_PD_RESET: c_int = 48;
pub const USB_PORT_C_PORT_PD_CHANGE: c_int = 49;
pub const USB_PORT_CABLE_PD_RESET: c_int = 50;
pub const USB_DEVICE_CHARGING_POLICY: c_int = 54;
//
// struct usb_ctrlrequest - SETUP data for a USB device control request
// @bRequestType: matches the USB bmRequestType field
// @bRequest: matches the USB bRequest field
// @wValue: matches the USB wValue field (le16 byte order)
// @wIndex: matches the USB wIndex field (le16 byte order)
// @wLength: matches the USB wLength field (le16 byte order)
//
// This structure is used to send control requests to a USB device.  It matches
// the different fields of the USB 2.0 Spec section 9.3, table 9-2.  See the
// USB spec for a fuller description of the different fields, and what they are
// used for.
//
// Note that the driver for any interface can issue control requests.
// For most devices, interfaces don't coordinate with each other, so
// such requests may be made at any time.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_ctrlrequest {
    pub bRequestType: __u8,
    pub bRequest: __u8,
    pub wValue: __le16,
    pub wIndex: __le16,
    pub wLength: __le16,
// C attribute field omitted
// -------------------------------------------------------------------------
//
// STANDARD DESCRIPTORS ... as returned by GET_DESCRIPTOR, or
// (rarely) accepted by SET_DESCRIPTOR.
//
// Note that all multi-byte values here are encoded in little endian
// byte order "on the wire".  Within the kernel and when exposed
// through the Linux-USB APIs, they are not converted to cpu byte
// order; it is the responsibility of the client code to do this.
// The single exception is when device and configuration descriptors (but
// not other descriptors) are read from character devices
// (i.e. /dev/bus/usb/BBB/DDD);
// in this case the fields are converted to host endianness by the kernel.
//
// Descriptor types ... USB 2.0 spec table 9.5
//
pub const USB_DT_DEVICE: c_uint = 0x01;
pub const USB_DT_CONFIG: c_uint = 0x02;
pub const USB_DT_STRING: c_uint = 0x03;
pub const USB_DT_INTERFACE: c_uint = 0x04;
pub const USB_DT_ENDPOINT: c_uint = 0x05;
pub const USB_DT_DEVICE_QUALIFIER: c_uint = 0x06;
pub const USB_DT_OTHER_SPEED_CONFIG: c_uint = 0x07;
pub const USB_DT_INTERFACE_POWER: c_uint = 0x08;
// these are from a minor usb 2.0 revision (ECN)
pub const USB_DT_OTG: c_uint = 0x09;
pub const USB_DT_DEBUG: c_uint = 0x0a;
pub const USB_DT_INTERFACE_ASSOCIATION: c_uint = 0x0b;
// these are from the Wireless USB spec
pub const USB_DT_SECURITY: c_uint = 0x0c;
pub const USB_DT_KEY: c_uint = 0x0d;
pub const USB_DT_ENCRYPTION_TYPE: c_uint = 0x0e;
pub const USB_DT_BOS: c_uint = 0x0f;
pub const USB_DT_DEVICE_CAPABILITY: c_uint = 0x10;
pub const USB_DT_WIRELESS_ENDPOINT_COMP: c_uint = 0x11;
// From the eUSB2 spec
pub const USB_DT_EUSB2_ISOC_ENDPOINT_COMP: c_uint = 0x12;
// From Wireless USB spec
pub const USB_DT_WIRE_ADAPTER: c_uint = 0x21;
// From USB Device Firmware Upgrade Specification, Revision 1.1
pub const USB_DT_DFU_FUNCTIONAL: c_uint = 0x21;
// these are from the Wireless USB spec
pub const USB_DT_RPIPE: c_uint = 0x22;
pub const USB_DT_CS_RADIO_CONTROL: c_uint = 0x23;
// From the T10 UAS specification
pub const USB_DT_PIPE_USAGE: c_uint = 0x24;
// From the USB 3.0 spec
pub const USB_DT_SS_ENDPOINT_COMP: c_uint = 0x30;
// From the USB 3.1 spec
pub const USB_DT_SSP_ISOC_ENDPOINT_COMP: c_uint = 0x31;
// Conventional codes for class-specific descriptors.  The convention is
// defined in the USB "Common Class" Spec (3.11).  Individual class specs
// are authoritative for their usage, not the "common class" writeup.
//

// All standard descriptors have these 2 fields at the beginning
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_descriptor_header {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
// C attribute field omitted
// -------------------------------------------------------------------------
// USB_DT_DEVICE: Device descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_device_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bcdUSB: __le16,
    pub bDeviceClass: __u8,
    pub bDeviceSubClass: __u8,
    pub bDeviceProtocol: __u8,
    pub bMaxPacketSize0: __u8,
    pub idVendor: __le16,
    pub idProduct: __le16,
    pub bcdDevice: __le16,
    pub iManufacturer: __u8,
    pub iProduct: __u8,
    pub iSerialNumber: __u8,
    pub bNumConfigurations: __u8,
// C attribute field omitted
pub const USB_DT_DEVICE_SIZE: c_int = 18;
//
// Device and/or Interface Class codes
// as found in bDeviceClass or bInterfaceClass
// and defined by www.usb.org documents
//

pub const USB_CLASS_AUDIO: c_int = 1;
pub const USB_CLASS_COMM: c_int = 2;
pub const USB_CLASS_HID: c_int = 3;
pub const USB_CLASS_PHYSICAL: c_int = 5;
pub const USB_CLASS_STILL_IMAGE: c_int = 6;
pub const USB_CLASS_PRINTER: c_int = 7;
pub const USB_CLASS_MASS_STORAGE: c_int = 8;
pub const USB_CLASS_HUB: c_int = 9;
pub const USB_CLASS_CDC_DATA: c_uint = 0x0a;
pub const USB_CLASS_CSCID: c_uint = 0x0b	/* chip+ smart card */;
pub const USB_CLASS_CONTENT_SEC: c_uint = 0x0d	/* content security */;
pub const USB_CLASS_VIDEO: c_uint = 0x0e;
pub const USB_CLASS_WIRELESS_CONTROLLER: c_uint = 0xe0;
pub const USB_CLASS_PERSONAL_HEALTHCARE: c_uint = 0x0f;
pub const USB_CLASS_AUDIO_VIDEO: c_uint = 0x10;
pub const USB_CLASS_BILLBOARD: c_uint = 0x11;
pub const USB_CLASS_USB_TYPE_C_BRIDGE: c_uint = 0x12;
pub const USB_CLASS_MCTP: c_uint = 0x14;
pub const USB_CLASS_MISC: c_uint = 0xef;
pub const USB_CLASS_APP_SPEC: c_uint = 0xfe;
pub const USB_SUBCLASS_DFU: c_uint = 0x01;
pub const USB_CLASS_VENDOR_SPEC: c_uint = 0xff;
pub const USB_SUBCLASS_VENDOR_SPEC: c_uint = 0xff;
// -------------------------------------------------------------------------
// USB_DT_CONFIG: Configuration descriptor information.
//
// USB_DT_OTHER_SPEED_CONFIG is the same descriptor, except that the
// descriptor type is different.  Highspeed-capable devices can look
// different depending on what speed they're currently running.  Only
// devices with a USB_DT_DEVICE_QUALIFIER have any OTHER_SPEED_CONFIG
// descriptors.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_config_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub wTotalLength: __le16,
    pub bNumInterfaces: __u8,
    pub bConfigurationValue: __u8,
    pub iConfiguration: __u8,
    pub bmAttributes: __u8,
    pub bMaxPower: __u8,
// C attribute field omitted
pub const USB_DT_CONFIG_SIZE: c_int = 9;
// from config descriptor bmAttributes

// -------------------------------------------------------------------------
// USB String descriptors can contain at most 126 characters.
pub const USB_MAX_STRING_LEN: c_int = 126;
// USB_DT_STRING: String descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_string_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub legacy_padding: __le16,
    pub /: *mut *mut __DECLARE_FLEX_ARRAY(__le16, wData); / UTF-16LE encoded,
}

// note that "string" zero is special, it holds language codes that
// the device supports, not Unicode characters.
//
// -------------------------------------------------------------------------
// USB_DT_INTERFACE: Interface descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_interface_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bInterfaceNumber: __u8,
    pub bAlternateSetting: __u8,
    pub bNumEndpoints: __u8,
    pub bInterfaceClass: __u8,
    pub bInterfaceSubClass: __u8,
    pub bInterfaceProtocol: __u8,
    pub iInterface: __u8,
// C attribute field omitted
pub const USB_DT_INTERFACE_SIZE: c_int = 9;
// -------------------------------------------------------------------------
// USB_DT_ENDPOINT: Endpoint descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_endpoint_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bEndpointAddress: __u8,
    pub bmAttributes: __u8,
    pub wMaxPacketSize: __le16,
    pub bInterval: __u8,
// NOTE:  these two are _only_ in audio endpoints.
// use USB_DT_ENDPOINT*_SIZE in bLength, not sizeof.
    pub bRefresh: __u8,
    pub bSynchAddress: __u8,
// C attribute field omitted
pub const USB_DT_ENDPOINT_SIZE: c_int = 7;

//
// Endpoints
//
pub const USB_ENDPOINT_NUMBER_MASK: c_uint = 0x0f	/* in bEndpointAddress */;
pub const USB_ENDPOINT_DIR_MASK: c_uint = 0x80;
pub const USB_ENDPOINT_XFERTYPE_MASK: c_uint = 0x03	/* in bmAttributes */;
pub const USB_ENDPOINT_XFER_CONTROL: c_int = 0;
pub const USB_ENDPOINT_XFER_ISOC: c_int = 1;
pub const USB_ENDPOINT_XFER_BULK: c_int = 2;
pub const USB_ENDPOINT_XFER_INT: c_int = 3;
pub const USB_ENDPOINT_MAX_ADJUSTABLE: c_uint = 0x80;
pub const USB_ENDPOINT_MAXP_MASK: c_uint = 0x07ff;
pub const USB_EP_MAXP_MULT_SHIFT: c_int = 11;

// The USB 3.0 spec redefines bits 5:4 of bmAttributes as interrupt ep type.
pub const USB_ENDPOINT_INTRTYPE: c_uint = 0x30;

pub const USB_ENDPOINT_SYNCTYPE: c_uint = 0x0c;

pub const USB_ENDPOINT_USAGE_MASK: c_uint = 0x30;
pub const USB_ENDPOINT_USAGE_DATA: c_uint = 0x00;
pub const USB_ENDPOINT_USAGE_FEEDBACK: c_uint = 0x10;
pub const USB_ENDPOINT_USAGE_IMPLICIT_FB: c_uint = 0x20	/* Implicit feedback Data endpoint */;
// -------------------------------------------------------------------------
//
// usb_endpoint_num - get the endpoint's number
// @epd: endpoint to be checked
//
// Returns @epd's number: 0 to 15.
//
    pub USB_ENDPOINT_NUMBER_MASK: return epd->bEndpointAddress &,
//
// usb_endpoint_type - get the endpoint's transfer type
// @epd: endpoint to be checked
//
// Returns one of USB_ENDPOINT_XFER_{CONTROL, ISOC, BULK, INT} according
// to @epd's transfer type.
//
    pub USB_ENDPOINT_XFERTYPE_MASK: return epd->bmAttributes &,
//
// usb_endpoint_dir_in - check if the endpoint has IN direction
// @epd: endpoint to be checked
//
// Returns true if the endpoint is of type IN, otherwise it returns false.
//
    pub USB_DIR_IN): return ((epd->bEndpointAddress & USB_ENDPOINT_DIR_MASK) ==,
//
// usb_endpoint_dir_out - check if the endpoint has OUT direction
// @epd: endpoint to be checked
//
// Returns true if the endpoint is of type OUT, otherwise it returns false.
//
    pub USB_DIR_OUT): return ((epd->bEndpointAddress & USB_ENDPOINT_DIR_MASK) ==,
//
// usb_endpoint_xfer_bulk - check if the endpoint has bulk transfer type
// @epd: endpoint to be checked
//
// Returns true if the endpoint is of type bulk, otherwise it returns false.
//
// usb_endpoint_xfer_control - check if the endpoint has control transfer type
// @epd: endpoint to be checked
//
// Returns true if the endpoint is of type control, otherwise it returns false.
//
// usb_endpoint_xfer_int - check if the endpoint has interrupt transfer type
// @epd: endpoint to be checked
//
// Returns true if the endpoint is of type interrupt, otherwise it returns
// false.
//
// usb_endpoint_xfer_isoc - check if the endpoint has isochronous transfer type
// @epd: endpoint to be checked
//
// Returns true if the endpoint is of type isochronous, otherwise it returns
// false.
//
// usb_endpoint_is_bulk_in - check if the endpoint is bulk IN
// @epd: endpoint to be checked
//
// Returns true if the endpoint has bulk transfer type and IN direction,
// otherwise it returns false.
//
    pub usb_endpoint_dir_in(epd): return usb_endpoint_xfer_bulk(epd) &&,
//
// usb_endpoint_is_bulk_out - check if the endpoint is bulk OUT
// @epd: endpoint to be checked
//
// Returns true if the endpoint has bulk transfer type and OUT direction,
// otherwise it returns false.
//
    pub usb_endpoint_dir_out(epd): return usb_endpoint_xfer_bulk(epd) &&,
//
// usb_endpoint_is_int_in - check if the endpoint is interrupt IN
// @epd: endpoint to be checked
//
// Returns true if the endpoint has interrupt transfer type and IN direction,
// otherwise it returns false.
//
    pub usb_endpoint_dir_in(epd): return usb_endpoint_xfer_int(epd) &&,
//
// usb_endpoint_is_int_out - check if the endpoint is interrupt OUT
// @epd: endpoint to be checked
//
// Returns true if the endpoint has interrupt transfer type and OUT direction,
// otherwise it returns false.
//
    pub usb_endpoint_dir_out(epd): return usb_endpoint_xfer_int(epd) &&,
//
// usb_endpoint_is_isoc_in - check if the endpoint is isochronous IN
// @epd: endpoint to be checked
//
// Returns true if the endpoint has isochronous transfer type and IN direction,
// otherwise it returns false.
//
    pub usb_endpoint_dir_in(epd): return usb_endpoint_xfer_isoc(epd) &&,
//
// usb_endpoint_is_isoc_out - check if the endpoint is isochronous OUT
// @epd: endpoint to be checked
//
// Returns true if the endpoint has isochronous transfer type and OUT direction,
// otherwise it returns false.
//
    pub usb_endpoint_dir_out(epd): return usb_endpoint_xfer_isoc(epd) &&,
//
// usb_endpoint_maxp - get endpoint's max packet size
// @epd: endpoint to be checked
//
// Returns @epd's max packet bits [10:0]
//
    pub USB_ENDPOINT_MAXP_MASK: return __le16_to_cpu(epd->wMaxPacketSize) &,
//
// usb_endpoint_maxp_mult - get endpoint's transactional opportunities
// @epd: endpoint to be checked
//
// Return @epd's wMaxPacketSize[12:11] + 1
//
    pub __le16_to_cpu(epd->wMaxPacketSize): int maxp =,
    pub 1: return USB_EP_MAXP_MULT(maxp) +,
    pub USB_ENDPOINT_INTRTYPE: return epd->bmAttributes &,
// -------------------------------------------------------------------------
// USB_DT_EUSB2_ISOC_ENDPOINT_COMP: eUSB2 Isoch Endpoint Companion descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_eusb2_isoc_ep_comp_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub wMaxPacketSize: __le16,
    pub dwBytesPerInterval: __le32,
// C attribute field omitted
pub const USB_DT_EUSB2_ISOC_EP_COMP_SIZE: c_int = 8;
// -------------------------------------------------------------------------
// USB_DT_SSP_ISOC_ENDPOINT_COMP: SuperSpeedPlus Isochronous Endpoint Companion
// descriptor
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_ssp_isoc_ep_comp_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub wReseved: __le16,
    pub dwBytesPerInterval: __le32,
// C attribute field omitted
pub const USB_DT_SSP_ISOC_EP_COMP_SIZE: c_int = 8;
// -------------------------------------------------------------------------
// USB_DT_SS_ENDPOINT_COMP: SuperSpeed Endpoint Companion descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_ss_ep_comp_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bMaxBurst: __u8,
    pub bmAttributes: __u8,
    pub wBytesPerInterval: __le16,
// C attribute field omitted
pub const USB_DT_SS_EP_COMP_SIZE: c_int = 6;
// Bits 4:0 of bmAttributes if this is a bulk endpoint
    pub max_streams: c_int,
    pub 0: return,
    pub 0x1f: max_streams = comp->bmAttributes &,
    pub 0: return,
    pub max_streams: max_streams = 1 <<,
    pub max_streams: return,
// Bits 1:0 of bmAttributes if this is an isoc endpoint

// Bit 7 of bmAttributes if a SSP isoc endpoint companion descriptor exists

// -------------------------------------------------------------------------
// USB_DT_DEVICE_QUALIFIER: Device Qualifier descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_qualifier_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bcdUSB: __le16,
    pub bDeviceClass: __u8,
    pub bDeviceSubClass: __u8,
    pub bDeviceProtocol: __u8,
    pub bMaxPacketSize0: __u8,
    pub bNumConfigurations: __u8,
    pub bRESERVED: __u8,
// C attribute field omitted
// -------------------------------------------------------------------------
// USB_DT_OTG (from OTG 1.0a supplement)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_otg_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub /: *mut *mut __u8 bmAttributes; / support for HNP, SRP, etc,
// C attribute field omitted
// USB_DT_OTG (from OTG 2.0 supplement)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_otg20_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub /: *mut *mut __u8 bmAttributes; / support for HNP, SRP and ADP, etc,
    pub number: *mut *mut __le16 bcdOTG; / OTG and EH supplement release,
// in binary-coded decimal(i.e. 2.0 is 0200H)
//
// C attribute field omitted
// from usb_otg_descriptor.bmAttributes

// OTG 3.0

pub const OTG_STS_SELECTOR: c_uint = 0xF000		/* OTG status selector */;
// -------------------------------------------------------------------------
// USB_DT_DEBUG:  for special highspeed devices, replacing serial console
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_debug_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
// bulk endpoints with 8 byte maxpacket
    pub bDebugInEndpoint: __u8,
    pub bDebugOutEndpoint: __u8,
    pub __attribute__((packed)): },
// -------------------------------------------------------------------------
// USB_DT_INTERFACE_ASSOCIATION: groups interfaces
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_interface_assoc_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bFirstInterface: __u8,
    pub bInterfaceCount: __u8,
    pub bFunctionClass: __u8,
    pub bFunctionSubClass: __u8,
    pub bFunctionProtocol: __u8,
    pub iFunction: __u8,
// C attribute field omitted
pub const USB_DT_INTERFACE_ASSOCIATION_SIZE: c_int = 8;
// -------------------------------------------------------------------------
// USB_DT_SECURITY:  group of wireless security descriptors, including
// encryption types available for setting up a CC/association.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_security_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub wTotalLength: __le16,
    pub bNumEncryptionTypes: __u8,
    pub __attribute__((packed)): },
// -------------------------------------------------------------------------
// USB_DT_KEY:  used with {GET,SET}_SECURITY_DATA; only public keys
// may be retrieved.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_key_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub tTKID: [__u8; 3],
    pub bReserved: __u8,
    pub bKeyData: [__u8; ],
    pub __attribute__((packed)): },
// -------------------------------------------------------------------------
// USB_DT_ENCRYPTION_TYPE:  bundled in DT_SECURITY groups
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_encryption_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bEncryptionType: __u8,
pub const USB_ENC_TYPE_UNSECURE: c_int = 0;

    pub /: *mut *mut __u8 bEncryptionValue; / use in SET_ENCRYPTION,
    pub bAuthKeyIndex: __u8,
    pub __attribute__((packed)): },
// -------------------------------------------------------------------------
// USB_DT_BOS:  group of device-level capabilities
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_bos_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub wTotalLength: __le16,
    pub bNumDeviceCaps: __u8,
    pub __attribute__((packed)): },
pub const USB_DT_BOS_SIZE: c_int = 5;
// -------------------------------------------------------------------------
// USB_DT_DEVICE_CAPABILITY:  grouped with BOS
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_dev_cap_header {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bDevCapabilityType: __u8,
    pub __attribute__((packed)): },
pub const USB_CAP_TYPE_WIRELESS_USB: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_wireless_cap_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bDevCapabilityType: __u8,
    pub bmAttributes: __u8,

    pub /: *mut *mut __le16 wPHYRates; / bit rates, Mbps,

    pub /: *mut *mut __u8 bmTFITXPowerInfo; / TFI power levels,
    pub /: *mut *mut __u8 bmFFITXPowerInfo; / FFI power levels,
    pub bmBandGroup: __le16,
    pub bReserved: __u8,
    pub __attribute__((packed)): },
pub const USB_DT_USB_WIRELESS_CAP_SIZE: c_int = 11;
// USB 2.0 Extension descriptor
pub const USB_CAP_TYPE_EXT: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_ext_cap_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bDevCapabilityType: __u8,
    pub bmAttributes: __le32,

    pub __attribute__((packed)): },
pub const USB_DT_USB_EXT_CAP_SIZE: c_int = 7;
//
// SuperSpeed USB Capability descriptor: Defines the set of SuperSpeed USB
// specific device level capabilities
//
pub const USB_SS_CAP_TYPE: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_ss_cap_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bDevCapabilityType: __u8,
    pub bmAttributes: __u8,

    pub wSpeedSupported: __le16,

    pub bFunctionalitySupport: __u8,
    pub bU1devExitLat: __u8,
    pub bU2DevExitLat: __le16,
    pub __attribute__((packed)): },
pub const USB_DT_USB_SS_CAP_SIZE: c_int = 10;
//
// Container ID Capability descriptor: Defines the instance unique ID used to
// identify the instance across all operating modes
//
pub const CONTAINER_ID_TYPE: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_ss_container_id_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bDevCapabilityType: __u8,
    pub bReserved: __u8,
    pub /: *mut *mut __u8 ContainerID[16]; / 128-bit number,
    pub __attribute__((packed)): },
pub const USB_DT_USB_SS_CONTN_ID_SIZE: c_int = 20;
//
// Platform Device Capability descriptor: Defines platform specific device
// capabilities
//
pub const USB_PLAT_DEV_CAP_TYPE: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_plat_dev_cap_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bDevCapabilityType: __u8,
    pub bReserved: __u8,
    pub UUID: [__u8; 16],
    pub CapabilityData: [__u8; ],
    pub __attribute__((packed)): },

//
// SuperSpeed Plus USB Capability descriptor: Defines the set of
// SuperSpeed Plus USB specific device level capabilities
//
pub const USB_SSP_CAP_TYPE: c_uint = 0xa;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_ssp_cap_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bDevCapabilityType: __u8,
    pub bReserved: __u8,
    pub bmAttributes: __le32,

    pub wFunctionalitySupport: __le16,

    pub wReserved: __le16,
    pub legacy_padding: __le32,
// list of sublink speed attrib entries
    pub bmSublinkSpeedAttr): __DECLARE_FLEX_ARRAY(__le32,,
}

pub const USB_SSP_SUBLINK_SPEED_LSE_BPS: c_int = 0;
pub const USB_SSP_SUBLINK_SPEED_LSE_KBPS: c_int = 1;
pub const USB_SSP_SUBLINK_SPEED_LSE_MBPS: c_int = 2;
pub const USB_SSP_SUBLINK_SPEED_LSE_GBPS: c_int = 3;

pub const USB_SSP_SUBLINK_SPEED_ST_SYM_RX: c_int = 0;
pub const USB_SSP_SUBLINK_SPEED_ST_ASYM_RX: c_int = 1;
pub const USB_SSP_SUBLINK_SPEED_ST_SYM_TX: c_int = 2;
pub const USB_SSP_SUBLINK_SPEED_ST_ASYM_TX: c_int = 3;

pub const USB_SSP_SUBLINK_SPEED_LP_SS: c_int = 0;
pub const USB_SSP_SUBLINK_SPEED_LP_SSP: c_int = 1;

//
// USB Power Delivery Capability Descriptor:
// Defines capabilities for PD
//
// Defines the various PD Capabilities of this device
pub const USB_PD_POWER_DELIVERY_CAPABILITY: c_uint = 0x06;
// Provides information on each battery supported by the device
pub const USB_PD_BATTERY_INFO_CAPABILITY: c_uint = 0x07;
// The Consumer characteristics of a Port on the device
pub const USB_PD_PD_CONSUMER_PORT_CAPABILITY: c_uint = 0x08;
// The provider characteristics of a Port on the device
pub const USB_PD_PD_PROVIDER_PORT_CAPABILITY: c_uint = 0x09;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_pd_cap_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub /: *mut *mut __u8 bDevCapabilityType; / set to USB_PD_POWER_DELIVERY_CAPABILITY,
    pub bReserved: __u8,
    pub bmAttributes: __le32,

    pub /: *mut *mut __le16 bmProviderPorts; / Bit zero refers to the UFP of the device,
    pub bmConsumerPorts: __le16,
    pub bcdBCVersion: __le16,
    pub bcdPDVersion: __le16,
    pub bcdUSBTypeCVersion: __le16,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_pd_cap_battery_info_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bDevCapabilityType: __u8,
// Index of string descriptor shall contain the user friendly name for this battery
    pub iBattery: __u8,
// Index of string descriptor shall contain the Serial Number String for this battery
    pub iSerial: __u8,
    pub iManufacturer: __u8,
    pub /: *mut *mut __u8 bBatteryId; / uniquely identifies this battery in status Messages,
    pub bReserved: __u8,
//
// Shall contain the Battery Charge value above which this
// battery is considered to be fully charged but not necessarily
// “topped off.”
//
    pub /: *mut *mut __le32 dwChargedThreshold; / in mWh,
//
// Shall contain the minimum charge level of this battery such
// that above this threshold, a device can be assured of being
// able to power up successfully (see Battery Charging 1.2).
//
    pub /: *mut *mut __le32 dwWeakThreshold; / in mWh,
    pub /: *mut *mut __le32 dwBatteryDesignCapacity; / in mWh,
    pub /: *mut *mut __le32 dwBatteryLastFullchargeCapacity; / in mWh,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_pd_cap_consumer_port_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bDevCapabilityType: __u8,
    pub bReserved: __u8,
    pub bmCapabilities: __u8,
// port will oerate under:

    pub /: *mut *mut __le16 wMinVoltage; / in 50mV units,
    pub /: *mut *mut __le16 wMaxVoltage; / in 50mV units,
    pub wReserved: __u16,
    pub /: *mut *mut __le32 dwMaxOperatingPower; / in 10 mW - operating at steady state,
    pub /: *mut *mut __le32 dwMaxPeakPower; / in 10mW units - operating at peak power,
    pub /: *mut *mut __le32 dwMaxPeakPowerTime; / in 100ms units - duration of peak,
pub const USB_PD_CAP_CONSUMER_UNKNOWN_PEAK_POWER_TIME: c_uint = 0xffff;
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_pd_cap_provider_port_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bDevCapabilityType: __u8,
    pub bReserved1: __u8,
    pub bmCapabilities: __u8,
// port will oerate under:

    pub bNumOfPDObjects: __u8,
    pub bReserved2: __u8,
    pub wPowerDataObject: [__le32; ],
    pub __attribute__((packed)): },
//
// Precision time measurement capability descriptor: advertised by devices and
// hubs that support PTM
//
pub const USB_PTM_CAP_TYPE: c_uint = 0xb;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_ptm_cap_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bDevCapabilityType: __u8,
    pub __attribute__((packed)): },
pub const USB_DT_USB_PTM_ID_SIZE: c_int = 3;
//
// The size of the descriptor for the Sublink Speed Attribute Count
// (SSAC) specified in bmAttributes[4:0]. SSAC is zero-based
//

// -------------------------------------------------------------------------
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_authentication_capability_descriptor {
    pub bLength: __u8,
    pub /: *mut *mut __u8 bDescriptorType; / set to USB_DT_DEVICE_CAPABILITY,
    pub bmAttributes: __u8,
    pub bcdProtocolVersion: __u8,
    pub bcdCapability: __u8,
    pub __attribute__((packed)): },
// -------------------------------------------------------------------------
// USB_DT_WIRELESS_ENDPOINT_COMP:  companion descriptor associated with
// each endpoint descriptor for a wireless device
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_wireless_ep_comp_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bMaxBurst: __u8,
    pub bMaxSequence: __u8,
    pub wMaxStreamDelay: __le16,
    pub wOverTheAirPacketSize: __le16,
    pub bOverTheAirInterval: __u8,
    pub bmCompAttributes: __u8,
pub const USB_ENDPOINT_SWITCH_MASK: c_uint = 0x03	/* in bmCompAttributes */;
pub const USB_ENDPOINT_SWITCH_NO: c_int = 0;
pub const USB_ENDPOINT_SWITCH_SWITCH: c_int = 1;
pub const USB_ENDPOINT_SWITCH_SCALE: c_int = 2;
    pub __attribute__((packed)): },
// -------------------------------------------------------------------------
// USB_REQ_SET_HANDSHAKE is a four-way handshake used between a wireless
// host and a device for connection set up, mutual authentication, and
// exchanging short lived session keys.  The handshake depends on a CC.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_handshake {
    pub bMessageNumber: __u8,
    pub bStatus: __u8,
    pub tTKID: [__u8; 3],
    pub bReserved: __u8,
    pub CDID: [__u8; 16],
    pub nonce: [__u8; 16],
    pub MIC: [__u8; 8],
    pub __attribute__((packed)): },
// -------------------------------------------------------------------------
// USB_REQ_SET_CONNECTION modifies or revokes a connection context (CC).
// A CC may also be set up using non-wireless secure channels (including
// wired USB!), and some devices may support CCs with multiple hosts.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_connection_context {
    pub /: *mut *mut __u8 CHID[16]; / persistent host id,
    pub /: *mut *mut __u8 CDID[16]; / device id (unique w/in host context),
    pub /: *mut *mut __u8 CK[16]; / connection key,
    pub __attribute__((packed)): },
// -------------------------------------------------------------------------
// USB 2.0 defines three speeds, here's how Linux identifies them
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum usb_device_speed {
    USB_SPEED_UNKNOWN = 0,			/* enumerating */
    USB_SPEED_LOW, USB_SPEED_FULL,		/* usb 1.1 */
    USB_SPEED_HIGH,				/* usb 2.0 */
    USB_SPEED_WIRELESS,			/* wireless (usb 2.5) */
    USB_SPEED_SUPER,			/* usb 3.0 */
    USB_SPEED_SUPER_PLUS,			/* usb 3.1 */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum usb_device_state {
// NOTATTACHED isn't in the USB spec, and this state acts
// the same as ATTACHED ... but it's clearer this way.
//
    USB_STATE_NOTATTACHED = 0,

// chapter 9 and authentication (wireless) device states
    USB_STATE_ATTACHED,
    USB_STATE_POWERED,			/* wired */
    USB_STATE_RECONNECTING,			/* auth */
    USB_STATE_UNAUTHENTICATED,		/* auth */
    USB_STATE_DEFAULT,			/* limited function */
    USB_STATE_ADDRESS,
    USB_STATE_CONFIGURED,			/* most functions */

    USB_STATE_SUSPENDED

// NOTE:  there are actually four different SUSPENDED
// states, returning to POWERED, DEFAULT, ADDRESS, or
// CONFIGURED respectively when SOF tokens flow again.
// At this level there's no difference between L1 and L2
// suspend states.  (L2 being original USB 1.1 suspend.)
//
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum usb3_link_state {
    USB3_LPM_U0 = 0,
    USB3_LPM_U1,
    USB3_LPM_U2,
    USB3_LPM_U3
}

//
// A U1 timeout of 0x0 means the parent hub will reject any transitions to U1.
// 0xff means the parent hub will accept transitions to U1, but will not
// initiate a transition.
//
// A U1 timeout of 0x1 to 0x7F also causes the hub to initiate a transition to
// U1 after that many microseconds.  Timeouts of 0x80 to 0xFE are reserved
// values.
//
// A U2 timeout of 0x0 means the parent hub will reject any transitions to U2.
// 0xff means the parent hub will accept transitions to U2, but will not
// initiate a transition.
//
// A U2 timeout of 0x1 to 0xFE also causes the hub to initiate a transition to
// U2 after N*256 microseconds.  Therefore a U2 timeout value of 0x1 means a U2
// idle timer of 256 microseconds, 0x2 means 512 microseconds, 0xFE means
// 65.024ms.
//
pub const USB3_LPM_DISABLED: c_uint = 0x0;
pub const USB3_LPM_U1_MAX_TIMEOUT: c_uint = 0x7F;
pub const USB3_LPM_U2_MAX_TIMEOUT: c_uint = 0xFE;
pub const USB3_LPM_DEVICE_INITIATED: c_uint = 0xFF;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_set_sel_req {
    pub u1_sel: __u8,
    pub u1_pel: __u8,
    pub u2_sel: __le16,
    pub u2_pel: __le16,
// C attribute field omitted
//
// The Set System Exit Latency control transfer provides one byte each for
// U1 SEL and U1 PEL, so the max exit latency is 0xFF.  U2 SEL and U2 PEL each
// are two bytes long.
//
pub const USB3_LPM_MAX_U1_SEL_PEL: c_uint = 0xFF;
pub const USB3_LPM_MAX_U2_SEL_PEL: c_uint = 0xFFFF;
// -------------------------------------------------------------------------
//
// As per USB compliance update, a device that is actively drawing
// more than 100mA from USB must report itself as bus-powered in
// the GetStatus(DEVICE) call.
// https://compliance.usb.org/index.asp?UpdateFile=Electrical&Format=Standard#34
//
pub const USB_SELF_POWER_VBUS_MAX_DRAW: c_int = 100;
