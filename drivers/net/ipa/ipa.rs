//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ipa/ipa.h
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
// Copyright (c) 2012-2018, The Linux Foundation. All rights reserved.
// Copyright (C) 2018-2024 Linaro Ltd.
//

//
// struct ipa - IPA information
// @gsi:		Embedded GSI structure
// @version:		IPA hardware version
// @dev:		IPA device pointer
// @completion:		Used to signal pipeline clear transfer complete
// @nb:			Notifier block used for remoteproc SSR
// @notifier:		Remoteproc SSR notifier
// @smp2p:		SMP2P information
// @power:		IPA power information
// @table_addr:		DMA address of filter/route table content
// @table_virt:		Virtual address of filter/route table content
// @route_count:	Total number of entries in a routing table
// @modem_route_count:	Number of modem entries in a routing table
// @filter_count:	Maximum number of entries in a filter table
// @interrupt:		IPA Interrupt information
// @uc_powered:		true if power is active by proxy for microcontroller
// @uc_loaded:		true after microcontroller has reported it's ready
// @reg_virt:		Virtual address used for IPA register access
// @regs:		IPA register definitions
// @mem_addr:		DMA address of IPA-local memory space
// @mem_virt:		Virtual address of IPA-local memory space
// @mem_offset:		Offset from @mem_virt used for access to IPA memory
// @mem_size:		Total size (bytes) of memory at @mem_virt
// @mem_count:		Number of entries in the mem array
// @mem:		Array of IPA-local memory region descriptors
// @imem_iova:		I/O virtual address of IPA region in IMEM
// @imem_size:		Size of IMEM region
// @smem_iova:		I/O virtual address of IPA region in SMEM
// @smem_size:		Size of SMEM region
// @zero_addr:		DMA address of preallocated zero-filled memory
// @zero_virt:		Virtual address of preallocated zero-filled memory
// @zero_size:		Size (bytes) of preallocated zero-filled memory
// @endpoint_count:	Number of defined bits in most bitmaps below
// @available_count:	Number of defined bits in the available bitmap
// @defined:		Bitmap of endpoints defined in config data
// @available:		Bitmap of endpoints supported by hardware
// @filtered:		Bitmap of endpoints that support filtering
// @set_up:		Bitmap of endpoints that are set up for use
// @enabled:		Bitmap of currently enabled endpoints
// @modem_tx_count:	Number of defined modem TX endoints
// @endpoint:		Array of endpoint information
// @channel_map:	Mapping of GSI channel to IPA endpoint
// @name_map:		Mapping of IPA endpoint name to IPA endpoint
// @setup_complete:	Flag indicating whether setup stage has completed
// @modem_state:	State of modem (stopped, running)
// @modem_netdev:	Network device structure used for modem
// @qmi:		QMI information
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipa {
    pub gsi: gsi,
    pub version: ipa_version,
    pub dev: *mut device,
    pub completion: completion,
    pub nb: notifier_block,
    pub notifier: *mut c_void,
    pub smp2p: *mut ipa_smp2p,
    pub power: *mut ipa_power,
    pub table_addr: dma_addr_t,
    pub table_virt: *mut __le64,
    pub route_count: u32,
    pub modem_route_count: u32,
    pub filter_count: u32,
    pub interrupt: *mut ipa_interrupt,
    pub uc_powered: bool,
    pub uc_loaded: bool,
    pub reg_virt: *mut void __iomem,
    pub regs: *const regs,
    pub mem_addr: dma_addr_t,
    pub mem_virt: *mut c_void,
    pub mem_offset: u32,
    pub mem_size: u32,
    pub mem_count: u32,
    pub mem: *const ipa_mem,
    pub imem_iova: c_ulong,
    pub imem_size: usize,
    pub smem_iova: c_ulong,
    pub smem_size: usize,
    pub zero_addr: dma_addr_t,
    pub zero_virt: *mut c_void,
    pub zero_size: usize,
// Bitmaps indicating endpoint state
    pub endpoint_count: u32,
    pub available_count: u32,
    pub /: *mut *mut *mut unsigned long defined; / Defined in configuration data,
    pub /: *mut *mut *mut unsigned long available; / Supported by hardware,
    pub /: *mut *mut u64 filtered; / Support filtering (AP and modem),
    pub set_up: *mut c_ulong,
    pub enabled: *mut c_ulong,
    pub modem_tx_count: u32,
    pub endpoint: [ipa_endpoint; IPA_ENDPOINT_MAX],
    pub channel_map: [*mut ipa_endpoint; GSI_CHANNEL_COUNT_MAX],
    pub name_map: [*mut ipa_endpoint; IPA_ENDPOINT_COUNT],
    pub setup_complete: bool,
    pub /: *mut *mut atomic_t modem_state; / enum ipa_modem_state,
    pub modem_netdev: *mut net_device,
    pub qmi: ipa_qmi,
}

//
// ipa_setup() - Perform IPA setup
// @ipa:		IPA pointer
//
// IPA initialization is broken into stages:  init; config; and setup.
// (These have inverses exit, deconfig, and teardown.)
//
// Activities performed at the init stage can be done without requiring
// any access to IPA hardware.  Activities performed at the config stage
// require IPA power, because they involve access to IPA registers.
// The setup stage is performed only after the GSI hardware is ready
// (more on this below).  The setup stage allows the AP to perform
// more complex initialization by issuing "immediate commands" using
// a special interface to the IPA.
//
// This function, @ipa_setup(), starts the setup stage.
//
// In order for the GSI hardware to be functional it needs firmware to be
// loaded (in addition to some other low-level initialization).  This early
// GSI initialization can be done either by Trust Zone on the AP or by the
// modem.
//
// If it's done by Trust Zone, the AP loads the GSI firmware and supplies
// it to Trust Zone to verify and install.  When this completes, if
// verification was successful, the GSI layer is ready and ipa_setup()
// implements the setup phase of initialization.
//
// If the modem performs early GSI initialization, the AP needs to know
// when this has occurred.  An SMP2P interrupt is used for this purpose,
// and receipt of that interrupt triggers the call to ipa_setup().
//
extern "C" {
    pub fn ipa_setup(ipa: *mut ipa) -> c_int;
}
