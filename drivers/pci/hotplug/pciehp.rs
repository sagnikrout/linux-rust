//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/pci/hotplug/pciehp.h
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
// PCI Express Hot Plug Controller Driver
//
// Copyright (C) 1995,2001 Compaq Computer Corporation
// Copyright (C) 2001 Greg Kroah-Hartman (greg@kroah.com)
// Copyright (C) 2001 IBM Corp.
// Copyright (C) 2003-2004 Intel Corporation
//
// All rights reserved.
//
// Send feedback to <greg@kroah.com>, <kristen.c.accardi@intel.com>
//

//
// Set CONFIG_DYNAMIC_DEBUG=y and boot with 'dyndbg="file pciehp* +p"' to
// enable debug messages.
//

pub const SLOT_NAME_SIZE: c_int = 10;
//
// struct controller - PCIe hotplug controller
// @pcie: pointer to the controller's PCIe port service device
// @dsn: cached copy of Device Serial Number of Function 0 in the hotplug slot
// (PCIe r6.2 sec 7.9.3); used to determine whether a hotplugged device
// was replaced with a different one during system sleep
// @slot_cap: cached copy of the Slot Capabilities register
// @inband_presence_disabled: In-Band Presence Detect Disable supported by
// controller and disabled per spec recommendation (PCIe r5.0, appendix I
// implementation note)
// @slot_ctrl: cached copy of the Slot Control register
// @ctrl_lock: serializes writes to the Slot Control register
// @cmd_started: jiffies when the Slot Control register was last written;
// the next write is allowed 1 second later, absent a Command Completed
// interrupt (PCIe r4.0, sec 6.7.3.2)
// @cmd_busy: flag set on Slot Control register write, cleared by IRQ handler
// on reception of a Command Completed event
// @queue: wait queue to wake up on reception of a Command Completed event,
// used for synchronous writes to the Slot Control register
// @pending_events: used by the IRQ handler to save events retrieved from the
// Slot Status register for later consumption by the IRQ thread
// @notification_enabled: whether the IRQ was requested successfully
// @power_fault_detected: whether a power fault was detected by the hardware
// that has not yet been cleared by the user
// @poll_thread: thread to poll for slot events if no IRQ is available,
// enabled with pciehp_poll_mode module parameter
// @state: current state machine position
// @state_lock: protects reads and writes of @state;
// protects scheduling, execution and cancellation of @button_work
// @button_work: work item to turn the slot on or off after 5 seconds
// in response to an Attention Button press
// @hotplug_slot: structure registered with the PCI hotplug core
// @reset_lock: prevents access to the Data Link Layer Link Active bit in the
// Link Status register and to the Presence Detect State bit in the Slot
// Status register during a slot reset which may cause them to flap
// @depth: Number of additional hotplug ports in the path to the root bus,
// used as lock subclass for @reset_lock
// @ist_running: flag to keep user request waiting while IRQ thread is running
// @request_result: result of last user request submitted to the IRQ thread
// @requester: wait queue to wake up on completion of user request,
// used for synchronous slot enable/disable request via sysfs
//
// PCIe hotplug has a 1:1 relationship between controller and slot, hence
// unlike other drivers, the two aren't represented by separate structures.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct controller {
    pub pcie: *mut pcie_device,
    pub dsn: u64,
    pub /: *mut *mut u32 slot_cap; / capabilities and quirks,
    pub inband_presence_disabled:1: c_uint,
    pub /: *mut *mut u16 slot_ctrl; / control register access,
    pub ctrl_lock: mutex,
    pub cmd_started: c_ulong,
    pub cmd_busy:1: c_uint,
    pub queue: wait_queue_head_t,
    pub /: *mut *mut atomic_t pending_events; / event handling,
    pub notification_enabled:1: c_uint,
    pub power_fault_detected: c_uint,
    pub poll_thread: *mut task_struct,
    pub /: *mut *mut u8 state; / state machine,
    pub state_lock: mutex,
    pub button_work: delayed_work,
    pub /: *mut *mut hotplug_slot hotplug_slot; / hotplug core interface,
    pub reset_lock: rw_semaphore,
    pub depth: c_uint,
    pub ist_running: c_uint,
    pub request_result: c_int,
    pub requester: wait_queue_head_t,
}

//
// DOC: Slot state
//
// @OFF_STATE: slot is powered off, no subordinate devices are enumerated
// @BLINKINGON_STATE: slot will be powered on after the 5 second delay,
// Power Indicator is blinking
// @BLINKINGOFF_STATE: slot will be powered off after the 5 second delay,
// Power Indicator is blinking
// @POWERON_STATE: slot is currently powering on
// @POWEROFF_STATE: slot is currently powering off
// @ON_STATE: slot is powered on, subordinate devices have been enumerated
//
pub const OFF_STATE: c_int = 0;
pub const BLINKINGON_STATE: c_int = 1;
pub const BLINKINGOFF_STATE: c_int = 2;
pub const POWERON_STATE: c_int = 3;
pub const POWEROFF_STATE: c_int = 4;
pub const ON_STATE: c_int = 5;
//
// DOC: Flags to request an action from the IRQ thread
//
// These are stored together with events read from the Slot Status register,
// hence must be greater than its 16-bit width.
//
// %DISABLE_SLOT: Disable the slot in response to a user request via sysfs or
// an Attention Button press after the 5 second delay
// %RERUN_ISR: Used by the IRQ handler to inform the IRQ thread that the
// hotplug port was inaccessible when the interrupt occurred, requiring
// that the IRQ handler is rerun by the IRQ thread after it has made the
// hotplug port accessible by runtime resuming its parents to D0
//

extern "C" {
    pub fn pciehp_request(ctrl: *mut controller, action: c_int);
}
extern "C" {
    pub fn pciehp_handle_button_press(ctrl: *mut controller);
}
extern "C" {
    pub fn pciehp_handle_disable_request(ctrl: *mut controller);
}
extern "C" {
    pub fn pciehp_handle_presence_or_link_change(ctrl: *mut controller, events: u32);
}
extern "C" {
    pub fn pciehp_configure_device(ctrl: *mut controller) -> c_int;
}
extern "C" {
    pub fn pciehp_unconfigure_device(ctrl: *mut controller, presence: bool);
}
extern "C" {
    pub fn pciehp_queue_pushbutton_work(work: *mut work_struct);
}
extern "C" {
    pub fn pcie_init_notification(ctrl: *mut controller) -> c_int;
}
extern "C" {
    pub fn pcie_shutdown_notification(ctrl: *mut controller);
}
extern "C" {
    pub fn pcie_clear_hotplug_events(ctrl: *mut controller);
}
extern "C" {
    pub fn pcie_enable_interrupt(ctrl: *mut controller);
}
extern "C" {
    pub fn pcie_disable_interrupt(ctrl: *mut controller);
}
extern "C" {
    pub fn pciehp_power_on_slot(ctrl: *mut controller) -> c_int;
}
extern "C" {
    pub fn pciehp_power_off_slot(ctrl: *mut controller);
}
extern "C" {
    pub fn pciehp_get_power_status(ctrl: *mut controller, status: *mut u8);
}

extern "C" {
    pub fn pciehp_set_indicators(ctrl: *mut controller, pwr: c_int, attn: c_int);
}
extern "C" {
    pub fn pciehp_get_latch_status(ctrl: *mut controller, status: *mut u8);
}
extern "C" {
    pub fn pciehp_query_power_fault(ctrl: *mut controller) -> c_int;
}
extern "C" {
    pub fn pciehp_card_present(ctrl: *mut controller) -> c_int;
}
extern "C" {
    pub fn pciehp_card_present_or_link_active(ctrl: *mut controller) -> c_int;
}
extern "C" {
    pub fn pciehp_check_link_status(ctrl: *mut controller) -> c_int;
}
extern "C" {
    pub fn pciehp_check_link_active(ctrl: *mut controller) -> c_int;
}
extern "C" {
    pub fn pciehp_device_replaced(ctrl: *mut controller) -> bool;
}
extern "C" {
    pub fn pciehp_release_ctrl(ctrl: *mut controller);
}
extern "C" {
    pub fn pciehp_sysfs_enable_slot(hotplug_slot: *mut hotplug_slot) -> c_int;
}
extern "C" {
    pub fn pciehp_sysfs_disable_slot(hotplug_slot: *mut hotplug_slot) -> c_int;
}
extern "C" {
    pub fn pciehp_reset_slot(hotplug_slot: *mut hotplug_slot, probe: bool) -> c_int;
}
extern "C" {
    pub fn pciehp_get_attention_status(hotplug_slot: *mut hotplug_slot, status: *mut u8) -> c_int;
}
extern "C" {
    pub fn pciehp_set_raw_indicator_status(h_slot: *mut hotplug_slot, status: u8) -> c_int;
}
extern "C" {
    pub fn pciehp_get_raw_indicator_status(h_slot: *mut hotplug_slot, status: *mut u8) -> c_int;
}
extern "C" {
    pub fn pciehp_slot_reset(dev: *mut pcie_device) -> c_int;
}
extern "C" {
    pub fn hotplug_slot_name(_arg: &ctrl->hotplug_slot) -> return;
}
extern "C" {
    pub fn container_of(_arg: hotplug_slot, controller: struct, _arg: hotplug_slot) -> return;
}
