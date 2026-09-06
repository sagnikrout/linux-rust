//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/usb/typec.h
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

// USB Type-C Specification releases
pub const USB_TYPEC_REV_1_0: c_uint = 0x100 /* 1.0 */;
pub const USB_TYPEC_REV_1_1: c_uint = 0x110 /* 1.1 */;
pub const USB_TYPEC_REV_1_2: c_uint = 0x120 /* 1.2 */;
pub const USB_TYPEC_REV_1_3: c_uint = 0x130 /* 1.3 */;
pub const USB_TYPEC_REV_1_4: c_uint = 0x140 /* 1.4 */;
pub const USB_TYPEC_REV_2_0: c_uint = 0x200 /* 2.0 */;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum typec_port_type {
    TYPEC_PORT_SRC,
    TYPEC_PORT_SNK,
    TYPEC_PORT_DRP,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum typec_port_data {
    TYPEC_PORT_DFP,
    TYPEC_PORT_UFP,
    TYPEC_PORT_DRD,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum typec_plug_type {
    USB_PLUG_NONE,
    USB_PLUG_TYPE_A,
    USB_PLUG_TYPE_B,
    USB_PLUG_TYPE_C,
    USB_PLUG_CAPTIVE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum typec_data_role {
    TYPEC_DEVICE,
    TYPEC_HOST,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum typec_role {
    TYPEC_SINK,
    TYPEC_SOURCE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum typec_pwr_opmode {
    TYPEC_PWR_MODE_USB,
    TYPEC_PWR_MODE_1_5A,
    TYPEC_PWR_MODE_3_0A,
    TYPEC_PWR_MODE_PD,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum typec_accessory {
    TYPEC_ACCESSORY_NONE,
    TYPEC_ACCESSORY_AUDIO,
    TYPEC_ACCESSORY_DEBUG,
}

pub const TYPEC_MAX_ACCESSORY: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum typec_orientation {
    TYPEC_ORIENTATION_NONE,
    TYPEC_ORIENTATION_NORMAL,
    TYPEC_ORIENTATION_REVERSE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum usb_mode {
    USB_MODE_NONE,
    USB_MODE_USB2,
    USB_MODE_USB3,
    USB_MODE_USB4
}

//
// struct enter_usb_data - Enter_USB Message details
// @eudo: Enter_USB Data Object
// @active_link_training: Active Cable Plug Link Training
//
// @active_link_training is a flag that should be set with uni-directional SBRX
// communication, and left 0 with passive cables and with bi-directional SBRX
// communication.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct enter_usb_data {
    pub eudo: u32,
    pub active_link_training:1: c_uchar,
}

//
// struct usb_pd_identity - USB Power Delivery identity data
// @id_header: ID Header VDO
// @cert_stat: Cert Stat VDO
// @product: Product VDO
// @vdo: Product Type Specific VDOs
//
// USB power delivery Discover Identity command response data.
//
// REVISIT: This is USB Power Delivery specific information, so this structure
// probable belongs to USB Power Delivery header file once we have them.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_pd_identity {
    pub id_header: u32,
    pub cert_stat: u32,
    pub product: u32,
    pub vdo: [u32; 3],
}

extern "C" {
    pub fn typec_partner_set_identity(partner: *mut typec_partner) -> c_int;
}
extern "C" {
    pub fn typec_cable_set_identity(cable: *mut typec_cable) -> c_int;
}
//
// struct typec_altmode_desc - USB Type-C Alternate Mode Descriptor
// @svid: Standard or Vendor ID
// @mode: Index of the Mode
// @vdo: VDO returned by Discover Modes USB PD command
// @roles: Only for ports. DRP if the mode is available in both roles
// @inactive: Only for ports. Make this port inactive (default is active).
//
// Description of an Alternate Mode which a connector, cable plug or partner
// supports.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct typec_altmode_desc {
    pub svid: u16,
    pub mode: u8,
    pub vdo: u32,
// Only used with ports
    pub roles: typec_port_data,
    pub inactive: bool,
    pub mode_selection: bool,
}

extern "C" {
    pub fn typec_partner_set_pd_revision(partner: *mut typec_partner, pd_revision: u16);
}
extern "C" {
    pub fn typec_partner_set_num_altmodes(partner: *mut typec_partner, num_altmodes: c_int) -> c_int;
}
// typec_partner_register_altmode(struct typec_partner *partner,
extern "C" {
    pub fn typec_plug_set_num_altmodes(plug: *mut typec_plug, num_altmodes: c_int) -> c_int;
}
// typec_plug_register_altmode(struct typec_plug *plug,
// typec_port_register_altmode(struct typec_port *port,
extern "C" {
    pub fn typec_unregister_altmode(altmode: *mut typec_altmode);
}
extern "C" {
    pub fn typec_altmode_update_active(alt: *mut typec_altmode, active: bool);
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum typec_plug_index {
    TYPEC_PLUG_SOP_P,
    TYPEC_PLUG_SOP_PP,
}

//
// struct typec_plug_desc - USB Type-C Cable Plug Descriptor
// @index: SOP Prime for the plug connected to DFP and SOP Double Prime for the
// plug connected to UFP
//
// Represents USB Type-C Cable Plug.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct typec_plug_desc {
    pub index: typec_plug_index,
}

//
// struct typec_cable_desc - USB Type-C Cable Descriptor
// @type: The plug type from USB PD Cable VDO
// @active: Is the cable active or passive
// @identity: Result of Discover Identity command
// @pd_revision: USB Power Delivery Specification revision if supported
//
// Represents USB Type-C Cable attached to USB Type-C port.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct typec_cable_desc {
    pub type: typec_plug_type,
    pub active:1: c_uint,
    pub identity: *mut usb_pd_identity,
    pub /: *mut *mut u16 pd_revision; / 0300H = "3.0",
}

//
// struct typec_partner_desc - USB Type-C Partner Descriptor
// @usb_pd: USB Power Delivery support
// @accessory: Audio, Debug or none.
// @identity: Discover Identity command data
// @pd_revision: USB Power Delivery Specification Revision if supported
// @usb_capability: Supported USB Modes
// @attach: Notification about attached USB device
// @deattach: Notification about removed USB device
//
// Details about a partner that is attached to USB Type-C port. If @identity
// member exists when partner is registered, a directory named "identity" is
// created to sysfs for the partner device.
//
// @pd_revision is based on the setting of the "Specification Revision" field
// in the message header on the initial "Source Capabilities" message received
// from the partner, or a "Request" message received from the partner, depending
// on whether our port is a Sink or a Source.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct typec_partner_desc {
    pub usb_pd:1: c_uint,
    pub accessory: typec_accessory,
    pub identity: *mut usb_pd_identity,
    pub /: *mut *mut u16 pd_revision; / 0300H = "3.0",
    pub usb_capability: u8,
    pub dev): *mut *mut *mut void (attach)(struct typec_partner partner, struct device,
    pub dev): *mut *mut *mut void (deattach)(struct typec_partner partner, struct device,
}

//
// struct typec_operations - USB Type-C Port Operations
// @try_role: Set data role preference for DRP port
// @dr_set: Set Data Role
// @pr_set: Set Power Role
// @vconn_set: Source VCONN
// @port_type_set: Set port type
// @pd_get: Get available USB Power Delivery Capabilities.
// @pd_set: Set USB Power Delivery Capabilities.
// @default_usb_mode_set: USB Mode to be used by default with Enter_USB Message
// @enter_usb_mode: Change the active USB Mode
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct typec_operations {
    pub role): *mut *mut *mut int (try_role)(struct typec_port port, int,
    pub role): *mut *mut *mut int (dr_set)(struct typec_port port, enum typec_data_role,
    pub role): *mut *mut *mut int (pr_set)(struct typec_port port, enum typec_role,
    pub role): *mut *mut *mut int (vconn_set)(struct typec_port port, enum typec_role,
    pub type): typec_port_type,
    pub port): *mut *mut *mut *mut usb_power_delivery (pd_get)(typec_port,
    pub pd): *mut *mut *mut int (pd_set)(struct typec_port port, struct usb_power_delivery,
    pub mode): *mut *mut *mut int (default_usb_mode_set)(struct typec_port port, enum usb_mode,
    pub mode): *mut *mut *mut int (enter_usb_mode)(struct typec_port port, enum usb_mode,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum usb_pd_svdm_ver {
    SVDM_VER_1_0 = 0,
    SVDM_VER_2_0 = 1,
    SVDM_VER_MAX = SVDM_VER_2_0,
}

//
// struct typec_capability - USB Type-C Port Capabilities
// @type: Supported power role of the port
// @data: Supported data role of the port
// @revision: USB Type-C Specification release. Binary coded decimal
// @pd_revision: USB Power Delivery Specification revision if supported
// @svdm_version: USB PD Structured VDM version if supported
// @prefer_role: Initial role preference (DRP ports).
// @accessory: Supported Accessory Modes
// @usb_capability: Supported USB Modes
// @no_mode_control: Ability to manage Alternate Modes
// @fwnode: Optional fwnode of the port
// @driver_data: Private pointer for driver specific info
// @pd: Optional USB Power Delivery Support
// @ops: Port operations vector
//
// Static capabilities of a single USB Type-C port.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct typec_capability {
    pub type: typec_port_type,
    pub data: typec_port_data,
    pub /: *mut *mut u16 revision; / 0120H = "1.2",
    pub /: *mut *mut u16 pd_revision; / 0300H = "3.0",
    pub svdm_version: usb_pd_svdm_ver,
    pub prefer_role: c_int,
    pub accessory: [typec_accessory; TYPEC_MAX_ACCESSORY],
    pub orientation_aware:1: c_uint,
    pub usb_capability: u8,
    pub no_mode_control: bool,
    pub fwnode: *mut fwnode_handle,
    pub driver_data: *mut c_void,
    pub pd: *mut usb_power_delivery,
    pub ops: *const typec_operations,
}

// Specific to try_role(). Indicates the user want's to clear the preference.

extern "C" {
    pub fn typec_unregister_port(port: *mut typec_port);
}
extern "C" {
    pub fn typec_unregister_partner(partner: *mut typec_partner);
}
extern "C" {
    pub fn typec_unregister_cable(cable: *mut typec_cable);
}
extern "C" {
    pub fn typec_cable_put(cable: *mut typec_cable);
}
extern "C" {
    pub fn typec_cable_is_active(cable: *mut typec_cable) -> c_int;
}
extern "C" {
    pub fn typec_cable_altmode_unsupported(alt: *mut typec_altmode) -> bool;
}
extern "C" {
    pub fn typec_unregister_plug(plug: *mut typec_plug);
}
extern "C" {
    pub fn typec_set_data_role(port: *mut typec_port, role: typec_data_role);
}
extern "C" {
    pub fn typec_get_data_role(port: *mut typec_port) -> typec_data_role;
}
extern "C" {
    pub fn typec_set_pwr_role(port: *mut typec_port, role: typec_role);
}
extern "C" {
    pub fn typec_set_vconn_role(port: *mut typec_port, role: typec_role);
}
extern "C" {
    pub fn typec_set_pwr_opmode(port: *mut typec_port, mode: typec_pwr_opmode);
}
extern "C" {
    pub fn typec_get_orientation(port: *mut typec_port) -> typec_orientation;
}
extern "C" {
    pub fn typec_set_mode(port: *mut typec_port, mode: c_int) -> c_int;
}
extern "C" {
    pub fn typec_find_pwr_opmode(name: *const c_char) -> c_int;
}
extern "C" {
    pub fn typec_find_orientation(name: *const c_char) -> c_int;
}
extern "C" {
    pub fn typec_find_port_power_role(name: *const c_char) -> c_int;
}
extern "C" {
    pub fn typec_find_power_role(name: *const c_char) -> c_int;
}
extern "C" {
    pub fn typec_find_port_data_role(name: *const c_char) -> c_int;
}
extern "C" {
    pub fn typec_get_negotiated_svdm_version(port: *mut typec_port) -> c_int;
}
extern "C" {
    pub fn typec_get_cable_svdm_version(port: *mut typec_port) -> c_int;
}
extern "C" {
    pub fn typec_cable_set_svdm_version(cable: *mut typec_cable, svdm_version: usb_pd_svdm_ver);
}
extern "C" {
    pub fn typec_port_set_usb_power_delivery(port: *mut typec_port, pd: *mut usb_power_delivery) -> c_int;
}
extern "C" {
    pub fn typec_partner_set_usb_mode(partner: *mut typec_partner, usb_mode: usb_mode);
}
extern "C" {
    pub fn typec_port_set_usb_mode(port: *mut typec_port, mode: usb_mode);
}
//
// struct typec_connector - Representation of Type-C port for external drivers
// @attach: notification about device removal
// @deattach: notification about device removal
//
// Drivers that control the USB and other ports (DisplayPorts, etc.), that are
// connected to the Type-C connectors, can use these callbacks to inform the
// Type-C connector class about connections and disconnections. That information
// can then be used by the typec-port drivers to power on or off parts that are
// needed or not needed - as an example, in USB mode if USB2 device is
// enumerated, USB3 components (retimers, phys, and what have you) do not need
// to be powered on.
//
// The attached (enumerated) devices will be liked with the typec-partner device.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct typec_connector {
    pub dev): *mut *mut *mut void (attach)(struct typec_connector con, struct device,
    pub dev): *mut *mut *mut void (deattach)(struct typec_connector con, struct device,
}
