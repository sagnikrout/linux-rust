//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/usb/phy.h
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
// USB PHY defines
//
// These APIs may be used between USB controllers.  USB device drivers
// (for either host or peripheral roles) don't use these calls; they
// continue to use just usb_device and usb_gadget.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum usb_phy_interface {
    USBPHY_INTERFACE_MODE_UNKNOWN,
    USBPHY_INTERFACE_MODE_UTMI,
    USBPHY_INTERFACE_MODE_UTMIW,
    USBPHY_INTERFACE_MODE_ULPI,
    USBPHY_INTERFACE_MODE_SERIAL,
    USBPHY_INTERFACE_MODE_HSIC,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum usb_phy_events {
    USB_EVENT_NONE,         /* no events or cable disconnected */
    USB_EVENT_VBUS,         /* vbus valid event */
    USB_EVENT_ID,           /* id was grounded */
    USB_EVENT_CHARGER,      /* usb dedicated charger */
    USB_EVENT_ENUMERATED,   /* gadget driver enumerated */
}

// associate a type with PHY
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum usb_phy_type {
    USB_PHY_TYPE_UNDEFINED,
    USB_PHY_TYPE_USB2,
    USB_PHY_TYPE_USB3,
}

// OTG defines lots of enumeration states before device reset
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum usb_otg_state {
    OTG_STATE_UNDEFINED = 0,

// single-role peripheral, and dual-role default-b
    OTG_STATE_B_IDLE,
    OTG_STATE_B_SRP_INIT,
    OTG_STATE_B_PERIPHERAL,

// extra dual-role default-b states
    OTG_STATE_B_WAIT_ACON,
    OTG_STATE_B_HOST,

// dual-role default-a
    OTG_STATE_A_IDLE,
    OTG_STATE_A_WAIT_VRISE,
    OTG_STATE_A_WAIT_BCON,
    OTG_STATE_A_HOST,
    OTG_STATE_A_SUSPEND,
    OTG_STATE_A_PERIPHERAL,
    OTG_STATE_A_WAIT_VFALL,
    OTG_STATE_A_VBUS_ERR,
}

// for phys connected thru an ULPI interface, the user must
// provide access ops
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_phy_io_ops {
    pub reg): *mut *mut *mut int (read)(struct usb_phy x, u32,
    pub reg): *mut *mut *mut int (write)(struct usb_phy x, u32 val, u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_charger_current {
    pub sdp_min: c_uint,
    pub sdp_max: c_uint,
    pub dcp_min: c_uint,
    pub dcp_max: c_uint,
    pub cdp_min: c_uint,
    pub cdp_max: c_uint,
    pub aca_min: c_uint,
    pub aca_max: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_phy {
    pub dev: *mut device,
    pub label: *const c_char,
    pub flags: c_uint,
    pub type: usb_phy_type,
    pub last_event: usb_phy_events,
    pub otg: *mut usb_otg,
    pub io_dev: *mut device,
    pub io_ops: *mut usb_phy_io_ops,
    pub io_priv: *mut void __iomem,
// to support extcon device
    pub edev: *mut extcon_dev,
    pub id_edev: *mut extcon_dev,
    pub vbus_nb: notifier_block,
    pub id_nb: notifier_block,
    pub type_nb: notifier_block,
// Support USB charger
    pub chg_type: usb_charger_type,
    pub chg_state: usb_charger_state,
    pub chg_cur: usb_charger_current,
    pub chg_work: work_struct,
// for notification of usb_phy_events
    pub notifier: atomic_notifier_head,
// to pass extra port status to the root hub
    pub port_status: u16,
    pub port_change: u16,
// to support controllers that have multiple phys
    pub head: list_head,
// initialize/shutdown the phy
    pub x): *mut *mut int (init)(struct usb_phy,
    pub x): *mut *mut void (shutdown)(struct usb_phy,
// enable/disable VBUS
    pub on): *mut *mut *mut int (set_vbus)(struct usb_phy x, int,
// effective for B devices, ignored for A-peripheral
    pub mA): unsigned,
// Set phy into suspend mode
    pub suspend): c_int,
//
// Set wakeup enable for PHY, in that case, the PHY can be
// woken up from suspend status due to external events,
// like vbus change, dp/dm change and id.
//
    pub enabled): *mut *mut *mut int (set_wakeup)(struct usb_phy x, bool,
// notify phy connect status change
    pub speed): usb_device_speed,
    pub speed): usb_device_speed,
//
// Charger detection method can be implemented if you need to
// manually detect the charger type.
//
    pub x): *mut *mut usb_charger_type (charger_detect)(struct usb_phy,
}

// for board-specific init logic
extern "C" {
    pub fn usb_add_phy(: *mut usb_phy, type: usb_phy_type) -> c_int;
}
extern "C" {
    pub fn usb_add_phy_dev(: *mut usb_phy) -> c_int;
}
extern "C" {
    pub fn usb_remove_phy(: *mut usb_phy);
}
// helpers for direct access thru low-level io interface
// for usb host and peripheral controller drivers

extern "C" {
    pub fn usb_put_phy(: *mut usb_phy);
}
extern "C" {
    pub fn usb_phy_set_event(x: *mut usb_phy, event: c_ulong);
}

extern "C" {
    pub fn ERR_PTR(_arg: -ENXIO) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -ENXIO) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -ENXIO) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -ENXIO) -> return;
}

// Context: can sleep
// notifiers
extern "C" {
    pub fn atomic_notifier_chain_register(_arg: &x->notifier, _arg: nb) -> return;
}
