//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/intel/iwlwifi/iwl-drv.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
//
// Copyright (C) 2005-2014, 2020-2021, 2023, 2025 Intel Corporation
// Copyright (C) 2013-2014 Intel Mobile Communications GmbH
//

// Macro flag: #define __iwl_drv_h__

// for all modules

// radio config bits (actual values from NVM definition)

//
// DOC: Driver system flows - drv component
//
// This component implements the system flows such as bus enumeration, bus
// removal. Bus dependent parts of system flows (such as iwl_pci_probe) are in
// bus specific files (transport files). This is the code that is common among
// different buses.
//
// This component is also in charge of managing the several implementations of
// the wifi flows: it will allow to have several fw API implementation. These
// different implementations will differ in the way they implement mac80211's
// handlers too.
// The init flow wrt to the drv component looks like this:
// 1) The bus specific component is called from module_init
// 2) The bus specific component registers the bus driver
// 3) The bus driver calls the probe function
// 4) The bus specific component configures the bus
// 5) The bus specific component calls to the drv bus agnostic part
// (iwl_drv_start)
// 6) iwl_drv_start fetches the fw ASYNC, iwl_req_fw_callback
// 7) iwl_req_fw_callback parses the fw file
// 8) iwl_req_fw_callback starts the wifi implementation to matches the fw
//
// iwl_drv_start - start the drv
//
// @trans: the transport
//
// starts the driver: fetches the firmware. This should be called by bus
// specific system flows implementations. For example, the bus specific probe
// function should do bus related operations only, and then call to this
// function.
// Return: the driver object or %NULL if an error occurred.
//
// iwl_drv_stop - stop the drv
//
// @drv:
//
// Stop the driver. This should be called by bus specific system flows
// implementations. For example, the bus specific remove function should first
// call this function and then do the bus related operations only.
//
extern "C" {
    pub fn iwl_drv_stop(drv: *mut iwl_drv);
}
//
// iwl_drv_is_wifi7_supported - returns if wifi7 is supported
// If yes, iwlmld needs to be used to drive the device.
//
extern "C" {
    pub fn iwl_drv_is_wifi7_supported(trans: *mut iwl_trans) -> bool;
}
//
// exported symbol management
//
// The driver can be split into multiple modules, in which case some symbols
// must be exported for the sub-modules. However, if it's not split and
// everything is built-in, then we can avoid that.
//

// Macro flag: #define IWL_EXPORT_SYMBOL(sym)

// Macro flag: #define VISIBLE_IF_IWLWIFI_KUNIT

// Macro flag: #define EXPORT_SYMBOL_IF_IWLWIFI_KUNIT(sym)

// max retry for init flow
pub const IWL_MAX_INIT_RETRY: c_int = 2;
pub const FW_NAME_PRE_BUFSIZE: c_int = 64;
