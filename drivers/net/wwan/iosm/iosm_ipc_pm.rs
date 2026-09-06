//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wwan/iosm/iosm_ipc_pm.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (C) 2020-21 Intel Corporation.
//
// Trigger the doorbell interrupt on cp to change the PM sleep/active status

// Trigger the doorbell interrupt on CP to do hpda update

//
// union ipc_pm_cond - Conditions for D3 and the sleep message to CP.
// @raw:	raw/combined value for faster check
// @irq:	IRQ towards CP
// @hs:		Host Sleep
// @link:	Device link state.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union ipc_pm_cond {
    pub raw: c_uint,
}

//
// enum ipc_mem_host_pm_state - Possible states of the HOST SLEEP finite state
// machine.
// @IPC_MEM_HOST_PM_ACTIVE:		   Host is active
// @IPC_MEM_HOST_PM_ACTIVE_WAIT:	   Intermediate state before going to
// active
// @IPC_MEM_HOST_PM_SLEEP_WAIT_IDLE:	   Intermediate state to wait for idle
// before going into sleep
// @IPC_MEM_HOST_PM_SLEEP_WAIT_D3:	   Intermediate state to wait for D3
// before going to sleep
// @IPC_MEM_HOST_PM_SLEEP:		   after this state the interface is not
// accessible host is in suspend to RAM
// @IPC_MEM_HOST_PM_SLEEP_WAIT_EXIT_SLEEP: Intermediate state before exiting
// sleep
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ipc_mem_host_pm_state {
    IPC_MEM_HOST_PM_ACTIVE,
    IPC_MEM_HOST_PM_ACTIVE_WAIT,
    IPC_MEM_HOST_PM_SLEEP_WAIT_IDLE,
    IPC_MEM_HOST_PM_SLEEP_WAIT_D3,
    IPC_MEM_HOST_PM_SLEEP,
    IPC_MEM_HOST_PM_SLEEP_WAIT_EXIT_SLEEP,
}

//
// enum ipc_mem_dev_pm_state - Possible states of the DEVICE SLEEP finite state
// machine.
// @IPC_MEM_DEV_PM_ACTIVE:		IPC_MEM_DEV_PM_ACTIVE is the initial
// power management state.
// IRQ(struct ipc_mem_device_info:
// device_sleep_notification)
// and DOORBELL-IRQ-HPDA(data) values.
// @IPC_MEM_DEV_PM_SLEEP:		IPC_MEM_DEV_PM_SLEEP is PM state for
// sleep.
// @IPC_MEM_DEV_PM_WAKEUP:		DOORBELL-IRQ-DEVICE_WAKE(data).
// @IPC_MEM_DEV_PM_HOST_SLEEP:		DOORBELL-IRQ-HOST_SLEEP(data).
// @IPC_MEM_DEV_PM_ACTIVE_WAIT:		Local intermediate states.
// @IPC_MEM_DEV_PM_FORCE_SLEEP:		DOORBELL-IRQ-FORCE_SLEEP.
// @IPC_MEM_DEV_PM_FORCE_ACTIVE:	DOORBELL-IRQ-FORCE_ACTIVE.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ipc_mem_dev_pm_state {
    IPC_MEM_DEV_PM_ACTIVE,
    IPC_MEM_DEV_PM_SLEEP,
    IPC_MEM_DEV_PM_WAKEUP,
    IPC_MEM_DEV_PM_HOST_SLEEP,
    IPC_MEM_DEV_PM_ACTIVE_WAIT,
    IPC_MEM_DEV_PM_FORCE_SLEEP = 7,
    IPC_MEM_DEV_PM_FORCE_ACTIVE,
}

//
// struct iosm_pm - Power management instance
// @pcie:			Pointer to iosm_pcie structure
// @dev:			Pointer to device structure
// @host_pm_state:		PM states for host
// @host_sleep_pend:		Variable to indicate Host Sleep Pending
// @host_sleep_complete:	Generic wait-for-completion used in
// case of Host Sleep
// @pm_cond:			Conditions for power management
// @ap_state:			Current power management state, the
// initial state is IPC_MEM_DEV_PM_ACTIVE eq. 0.
// @cp_state:			PM State of CP
// @device_sleep_notification:	last handled device_sleep_notfication
// @pending_hpda_update:	is a HPDA update pending?
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iosm_pm {
    pub pcie: *mut iosm_pcie,
    pub dev: *mut device,
    pub host_pm_state: ipc_mem_host_pm_state,
    pub host_sleep_pend: c_ulong,
    pub host_sleep_complete: completion,
    pub pm_cond: ipc_pm_cond,
    pub ap_state: ipc_mem_dev_pm_state,
    pub cp_state: ipc_mem_dev_pm_state,
    pub device_sleep_notification: u32,
    pub pending_hpda_update:1: u8,
}

//
// enum ipc_pm_unit - Power management units.
// @IPC_PM_UNIT_IRQ:	IRQ towards CP
// @IPC_PM_UNIT_HS:	Host Sleep for converged protocol
// @IPC_PM_UNIT_LINK:	Link state controlled by CP.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ipc_pm_unit {
    IPC_PM_UNIT_IRQ,
    IPC_PM_UNIT_HS,
    IPC_PM_UNIT_LINK,
}

//
// ipc_pm_init - Allocate power management component
// @ipc_protocol:	Pointer to iosm_protocol structure
//
extern "C" {
    pub fn ipc_pm_init(ipc_protocol: *mut iosm_protocol);
}
//
// ipc_pm_deinit - Free power management component, invalidating its pointer.
// @ipc_protocol:	Pointer to iosm_protocol structure
//
extern "C" {
    pub fn ipc_pm_deinit(ipc_protocol: *mut iosm_protocol);
}
//
// ipc_pm_dev_slp_notification - Handle a sleep notification message from the
// device. This can be called from interrupt state
// This function handles Host Sleep requests too
// if the Host Sleep protocol is register based.
// @ipc_pm:			Pointer to power management component
// @sleep_notification:		Actual notification from device
//
// Returns: true if dev sleep state has to be checked, false otherwise.
//
// ipc_pm_set_s2idle_sleep - Set PM variables to sleep/active
// @ipc_pm:	Pointer to power management component
// @sleep:	true to enter sleep/false to exit sleep
//
extern "C" {
    pub fn ipc_pm_set_s2idle_sleep(ipc_pm: *mut iosm_pm, sleep: bool);
}
//
// ipc_pm_prepare_host_sleep - Prepare the PM for sleep by entering
// IPC_MEM_HOST_PM_SLEEP_WAIT_D3 state.
// @ipc_pm:	Pointer to power management component
//
// Returns: true on success, false if the host was not active.
//
extern "C" {
    pub fn ipc_pm_prepare_host_sleep(ipc_pm: *mut iosm_pm) -> bool;
}
//
// ipc_pm_prepare_host_active - Prepare the PM for wakeup by entering
// IPC_MEM_HOST_PM_ACTIVE_WAIT state.
// @ipc_pm:	Pointer to power management component
//
// Returns: true on success, false if the host was not sleeping.
//
extern "C" {
    pub fn ipc_pm_prepare_host_active(ipc_pm: *mut iosm_pm) -> bool;
}
//
// ipc_pm_wait_for_device_active - Wait up to IPC_PM_ACTIVE_TIMEOUT_MS ms
// for the device to reach active state
// @ipc_pm:	Pointer to power management component
//
// Returns: true if device is active, false on timeout
//
extern "C" {
    pub fn ipc_pm_wait_for_device_active(ipc_pm: *mut iosm_pm) -> bool;
}
//
// ipc_pm_signal_hpda_doorbell - Wake up the device if it is in low power mode
// and trigger a head pointer update interrupt.
// @ipc_pm:		Pointer to power management component
// @identifier:		specifies what component triggered hpda update irq
// @host_slp_check:	if set to true then Host Sleep state machine check will
// be performed. If Host Sleep state machine allows HP
// update then only doorbell is triggered otherwise pending
// flag will be set. If set to false then Host Sleep check
// will not be performed. This is helpful for Host Sleep
// negotiation through message ring.
//
// ipc_pm_trigger - Update power manager and wake up the link if needed
// @ipc_pm:	Pointer to power management component
// @unit:	Power management units
// @active:	Device link state
//
// Returns: true if link is unchanged or active, false otherwise
//
extern "C" {
    pub fn ipc_pm_trigger(ipc_pm: *mut iosm_pm, unit: ipc_pm_unit, active: bool) -> bool;
}
