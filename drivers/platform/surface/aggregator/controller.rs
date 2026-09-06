//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/platform/surface/aggregator/controller.h
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
// Main SSAM/SSH controller structure and functionality.
//
// Copyright (C) 2019-2022 Maximilian Luz <luzmaximilian@gmail.com>
//

// -- Safe counters. --------------------------------------------------------
//
// struct ssh_seq_counter - Safe counter for SSH sequence IDs.
// @value: The current counter value.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ssh_seq_counter {
    pub value: u8,
}

//
// struct ssh_rqid_counter - Safe counter for SSH request IDs.
// @value: The current counter value.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ssh_rqid_counter {
    pub value: u16,
}

// -- Event/notification system. --------------------------------------------
//
// struct ssam_nf_head - Notifier head for SSAM events.
// @srcu: The SRCU struct for synchronization.
// @head: List-head for notifier blocks registered under this head.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ssam_nf_head {
    pub srcu: srcu_struct,
    pub head: list_head,
}

//
// struct ssam_nf - Notifier callback- and activation-registry for SSAM events.
// @lock:     Lock guarding (de-)registration of notifier blocks. Note: This
// lock does not need to be held for notifier calls, only
// registration and deregistration.
// @refcount: The root of the RB-tree used for reference-counting enabled
// events/notifications.
// @head:     The list of notifier heads for event/notification callbacks.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ssam_nf {
    pub lock: mutex,
    pub refcount: rb_root,
    pub head: [ssam_nf_head; SSH_NUM_EVENTS],
}

// -- Event/async request completion system. --------------------------------
//
// struct ssam_event_item - Struct for event queuing and completion.
// @node:     The node in the queue.
// @rqid:     The request ID of the event.
// @ops:      Instance specific functions.
// @ops.free: Callback for freeing this event item.
// @event:    Actual event data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ssam_event_item {
    pub node: list_head,
    pub rqid: u16,
    pub event): *mut *mut void (free)(struct ssam_event_item,
    pub ops: },
    pub /: *mut *mut ssam_event event; / must be last,
}

//
// struct ssam_event_queue - Queue for completing received events.
// @cplt: Reference to the completion system on which this queue is active.
// @lock: The lock for any operation on the queue.
// @head: The list-head of the queue.
// @work: The &struct work_struct performing completion work for this queue.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ssam_event_queue {
    pub cplt: *mut ssam_cplt,
    pub lock: spinlock_t,
    pub head: list_head,
    pub work: work_struct,
}

//
// struct ssam_event_target - Set of queues for a single SSH target ID.
// @queue: The array of queues, one queue per event ID.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ssam_event_target {
    pub queue: [ssam_event_queue; SSH_NUM_EVENTS],
}

//
// struct ssam_cplt - SSAM event/async request completion system.
// @dev:          The device with which this system is associated. Only used
// for logging.
// @wq:           The &struct workqueue_struct on which all completion work
// items are queued.
// @event:        Event completion management.
// @event.target: Array of &struct ssam_event_target, one for each target.
// @event.notif:  Notifier callbacks and event activation reference counting.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ssam_cplt {
    pub dev: *mut device,
    pub wq: *mut workqueue_struct,
    pub target: [ssam_event_target; SSH_NUM_TARGETS],
    pub notif: ssam_nf,
    pub event: },
}

// -- Main SSAM device structures. ------------------------------------------
//
// enum ssam_controller_state - State values for &struct ssam_controller.
// @SSAM_CONTROLLER_UNINITIALIZED:
// The controller has not been initialized yet or has been deinitialized.
// @SSAM_CONTROLLER_INITIALIZED:
// The controller is initialized, but has not been started yet.
// @SSAM_CONTROLLER_STARTED:
// The controller has been started and is ready to use.
// @SSAM_CONTROLLER_STOPPED:
// The controller has been stopped.
// @SSAM_CONTROLLER_SUSPENDED:
// The controller has been suspended.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ssam_controller_state {
    SSAM_CONTROLLER_UNINITIALIZED,
    SSAM_CONTROLLER_INITIALIZED,
    SSAM_CONTROLLER_STARTED,
    SSAM_CONTROLLER_STOPPED,
    SSAM_CONTROLLER_SUSPENDED,
}

//
// struct ssam_controller_caps - Controller device capabilities.
// @ssh_power_profile:             SSH power profile.
// @ssh_buffer_size:               SSH driver UART buffer size.
// @screen_on_sleep_idle_timeout:  SAM UART screen-on sleep idle timeout.
// @screen_off_sleep_idle_timeout: SAM UART screen-off sleep idle timeout.
// @d3_closes_handle:              SAM closes UART handle in D3.
//
// Controller and SSH device capabilities found in ACPI.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ssam_controller_caps {
    pub ssh_power_profile: u32,
    pub ssh_buffer_size: u32,
    pub screen_on_sleep_idle_timeout: u32,
    pub screen_off_sleep_idle_timeout: u32,
    pub d3_closes_handle:1: u32,
}

//
// struct ssam_controller - SSAM controller device.
// @kref:  Reference count of the controller.
// @lock:  Main lock for the controller, used to guard state changes.
// @state: Controller state.
// @rtl:   Request transport layer for SSH I/O.
// @cplt:  Completion system for SSH/SSAM events and asynchronous requests.
// @counter:      Safe SSH message ID counters.
// @counter.seq:  Sequence ID counter.
// @counter.rqid: Request ID counter.
// @irq:          Wakeup IRQ resources.
// @irq.num:      The wakeup IRQ number.
// @irq.wakeup_enabled: Whether wakeup by IRQ is enabled during suspend.
// @caps: The controller device capabilities.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ssam_controller {
    pub kref: kref,
    pub lock: rw_semaphore,
    pub state: ssam_controller_state,
    pub rtl: ssh_rtl,
    pub cplt: ssam_cplt,
    pub seq: ssh_seq_counter,
    pub rqid: ssh_rqid_counter,
    pub counter: },
    pub num: c_int,
    pub wakeup_enabled: bool,
    pub irq: },
    pub caps: ssam_controller_caps,
}

//
// ssam_controller_receive_buf() - Provide input-data to the controller.
// @ctrl: The controller.
// @buf:  The input buffer.
// @n:    The number of bytes in the input buffer.
//
// Provide input data to be evaluated by the controller, which has been
// received via the lower-level transport.
//
// Return: Returns the number of bytes consumed, or, if the packet transport
// layer of the controller has been shut down, %-ESHUTDOWN.
//
extern "C" {
    pub fn ssh_ptl_rx_rcvbuf(_arg: &ctrl->rtl.ptl, _arg: buf, _arg: n) -> return;
}
//
// ssam_controller_write_wakeup() - Notify the controller that the underlying
// device has space available for data to be written.
// @ctrl: The controller.
//
extern "C" {
    pub fn ssam_controller_init(ctrl: *mut ssam_controller, s: *mut serdev_device) -> c_int;
}
extern "C" {
    pub fn ssam_controller_start(ctrl: *mut ssam_controller) -> c_int;
}
extern "C" {
    pub fn ssam_controller_shutdown(ctrl: *mut ssam_controller);
}
extern "C" {
    pub fn ssam_controller_destroy(ctrl: *mut ssam_controller);
}
extern "C" {
    pub fn ssam_notifier_disable_registered(ctrl: *mut ssam_controller) -> c_int;
}
extern "C" {
    pub fn ssam_notifier_restore_registered(ctrl: *mut ssam_controller);
}
extern "C" {
    pub fn ssam_irq_setup(ctrl: *mut ssam_controller) -> c_int;
}
extern "C" {
    pub fn ssam_irq_free(ctrl: *mut ssam_controller);
}
extern "C" {
    pub fn ssam_irq_arm_for_wakeup(ctrl: *mut ssam_controller) -> c_int;
}
extern "C" {
    pub fn ssam_irq_disarm_wakeup(ctrl: *mut ssam_controller);
}
extern "C" {
    pub fn ssam_controller_lock(c: *mut ssam_controller);
}
extern "C" {
    pub fn ssam_controller_unlock(c: *mut ssam_controller);
}
extern "C" {
    pub fn ssam_get_firmware_version(ctrl: *mut ssam_controller, version: *mut u32) -> c_int;
}
extern "C" {
    pub fn ssam_ctrl_notif_display_off(ctrl: *mut ssam_controller) -> c_int;
}
extern "C" {
    pub fn ssam_ctrl_notif_display_on(ctrl: *mut ssam_controller) -> c_int;
}
extern "C" {
    pub fn ssam_ctrl_notif_d0_exit(ctrl: *mut ssam_controller) -> c_int;
}
extern "C" {
    pub fn ssam_ctrl_notif_d0_entry(ctrl: *mut ssam_controller) -> c_int;
}
extern "C" {
    pub fn ssam_controller_suspend(ctrl: *mut ssam_controller) -> c_int;
}
extern "C" {
    pub fn ssam_controller_resume(ctrl: *mut ssam_controller) -> c_int;
}
extern "C" {
    pub fn ssam_event_item_cache_init() -> c_int;
}
extern "C" {
    pub fn ssam_event_item_cache_destroy();
}
