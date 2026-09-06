//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/qlogic/qed/qed_dev_api.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
// QLogic qed NIC Driver
// Copyright (c) 2015-2017  QLogic Corporation
// Copyright (c) 2019-2020 Marvell International Ltd.
//

//
// qed_init_dp(): Initialize the debug level.
//
// @cdev: Qed dev pointer.
// @dp_module: Module debug parameter.
// @dp_level: Module debug level.
//
// Return: Void.
//
// qed_init_struct(): Initialize the device structure to
// its defaults.
//
// @cdev: Qed dev pointer.
//
// Return: Void.
//
extern "C" {
    pub fn qed_init_struct(cdev: *mut qed_dev);
}
//
// qed_resc_free: Free device resources.
//
// @cdev: Qed dev pointer.
//
// Return: Void.
//
extern "C" {
    pub fn qed_resc_free(cdev: *mut qed_dev);
}
//
// qed_resc_alloc(): Alloc device resources.
//
// @cdev: Qed dev pointer.
//
// Return: Int.
//
extern "C" {
    pub fn qed_resc_alloc(cdev: *mut qed_dev) -> c_int;
}
//
// qed_resc_setup(): Setup device resources.
//
// @cdev: Qed dev pointer.
//
// Return: Void.
//
extern "C" {
    pub fn qed_resc_setup(cdev: *mut qed_dev);
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qed_override_force_load {
    QED_OVERRIDE_FORCE_LOAD_NONE,
    QED_OVERRIDE_FORCE_LOAD_ALWAYS,
    QED_OVERRIDE_FORCE_LOAD_NEVER,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_drv_load_params {
// Indicates whether the driver is running over a crash kernel.
// As part of the load request, this will be used for providing the
// driver role to the MFW.
// In case of a crash kernel over PDA - this should be set to false.
//
    pub is_crash_kernel: bool,
// The timeout value that the MFW should use when locking the engine for
// the driver load process.
// A value of '0' means the default value, and '255' means no timeout.
//
    pub mfw_timeout_val: u8,
pub const QED_LOAD_REQ_LOCK_TO_DEFAULT: c_int = 0;
pub const QED_LOAD_REQ_LOCK_TO_NONE: c_int = 255;
// Avoid engine reset when first PF loads on it
    pub avoid_eng_reset: bool,
// Allow overriding the default force load behavior
    pub override_force_load: qed_override_force_load,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_hw_init_params {
// Tunneling parameters
    pub p_tunn: *mut qed_tunnel_info,
    pub b_hw_start: bool,
// Interrupt mode [msix, inta, etc.] to use
    pub int_mode: qed_int_mode,
// NPAR tx switching to be used for vports for tx-switching
    pub allow_npar_tx_switch: bool,
// Binary fw data pointer in binary fw file
    pub bin_fw_data: *const u8,
// Driver load parameters
    pub p_drv_load_params: *mut qed_drv_load_params,
}

//
// qed_hw_init(): Init Qed hardware.
//
// @cdev: Qed dev pointer.
// @p_params: Pointers to params.
//
// Return: Int.
//
extern "C" {
    pub fn qed_hw_init(cdev: *mut qed_dev, p_params: *mut qed_hw_init_params) -> c_int;
}
//
// qed_hw_timers_stop_all(): Stop the timers HW block.
//
// @cdev: Qed dev pointer.
//
// Return: void.
//
extern "C" {
    pub fn qed_hw_timers_stop_all(cdev: *mut qed_dev);
}
//
// qed_hw_stop(): Stop Qed hardware.
//
// @cdev: Qed dev pointer.
//
// Return: int.
//
extern "C" {
    pub fn qed_hw_stop(cdev: *mut qed_dev) -> c_int;
}
//
// qed_hw_stop_fastpath(): Should be called incase
// slowpath is still required for the device,
// but fastpath is not.
//
// @cdev: Qed dev pointer.
//
// Return: Int.
//
extern "C" {
    pub fn qed_hw_stop_fastpath(cdev: *mut qed_dev) -> c_int;
}
//
// qed_hw_start_fastpath(): Restart fastpath traffic,
// only if hw_stop_fastpath was called.
//
// @p_hwfn: HW device data.
//
// Return: Int.
//
extern "C" {
    pub fn qed_hw_start_fastpath(p_hwfn: *mut qed_hwfn) -> c_int;
}
//
// qed_hw_prepare(): Prepare Qed hardware.
//
// @cdev: Qed dev pointer.
// @personality: Personality to initialize.
//
// Return: Int.
//
// qed_hw_remove(): Remove Qed hardware.
//
// @cdev: Qed dev pointer.
//
// Return: Void.
//
extern "C" {
    pub fn qed_hw_remove(cdev: *mut qed_dev);
}
//
// qed_ptt_acquire(): Allocate a PTT window.
//
// @p_hwfn: HW device data.
//
// Return: struct qed_ptt.
//
// Should be called at the entry point to the driver (at the beginning of an
// exported function).
//
// qed_ptt_acquire_context(): Allocate a PTT window honoring the context
// atomicy.
//
// @p_hwfn: HW device data.
// @is_atomic: Hint from the caller - if the func can sleep or not.
//
// Context: The function should not sleep in case is_atomic == true.
// Return: struct qed_ptt.
//
// Should be called at the entry point to the driver
// (at the beginning of an exported function).
//
// qed_ptt_release(): Release PTT Window.
//
// @p_hwfn: HW device data.
// @p_ptt: P_ptt.
//
// Return: Void.
//
// Should be called at the end of a flow - at the end of the function that
// acquired the PTT.
//
extern "C" {
    pub fn qed_reset_vport_stats(cdev: *mut qed_dev);
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qed_dmae_address_type_t {
    QED_DMAE_ADDRESS_HOST_VIRT,
    QED_DMAE_ADDRESS_HOST_PHYS,
    QED_DMAE_ADDRESS_GRC
}

//
// qed_dmae_host2grc(): Copy data from source addr to
// dmae registers using the given ptt.
//
// @p_hwfn: HW device data.
// @p_ptt: P_ptt.
// @source_addr: Source address.
// @grc_addr: GRC address (dmae_data_offset).
// @size_in_dwords: Size.
// @p_params: (default parameters will be used in case of NULL).
//
// Return: Int.
//
// qed_dmae_grc2host(): Read data from dmae data offset
// to source address using the given ptt.
//
// @p_ptt: P_ptt.
// @grc_addr: GRC address (dmae_data_offset).
// @dest_addr: Destination Address.
// @size_in_dwords: Size.
// @p_params: (default parameters will be used in case of NULL).
//
// Return: Int.
//
// qed_dmae_host2host(): Copy data from to source address
// to a destination adrress (for SRIOV) using the given
// ptt.
//
// @p_hwfn: HW device data.
// @p_ptt: P_ptt.
// @source_addr: Source address.
// @dest_addr: Destination address.
// @size_in_dwords: size.
// @p_params: (default parameters will be used in case of NULL).
//
// Return: Int.
//
extern "C" {
    pub fn qed_chain_free(cdev: *mut qed_dev, chain: *mut qed_chain);
}
//
// qed_fw_l2_queue(): Get absolute L2 queue ID.
//
// @p_hwfn: HW device data.
// @src_id: Relative to p_hwfn.
// @dst_id: Absolute per engine.
//
// Return: Int.
//
// qed_fw_vport(): Get absolute vport ID.
//
// @p_hwfn: HW device data.
// @src_id: Relative to p_hwfn.
// @dst_id: Absolute per engine.
//
// Return: Int.
//
// qed_fw_rss_eng(): Get absolute RSS engine ID.
//
// @p_hwfn: HW device data.
// @src_id: Relative to p_hwfn.
// @dst_id: Absolute per engine.
//
// Return: Int.
//
// qed_llh_get_num_ppfid(): Return the allocated number of LLH filter
// banks that are allocated to the PF.
//
// @cdev: Qed dev pointer.
//
// Return: u8 Number of LLH filter banks.
//
extern "C" {
    pub fn qed_llh_get_num_ppfid(cdev: *mut qed_dev) -> u8;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qed_eng {
    QED_ENG0,
    QED_ENG1,
    QED_BOTH_ENG,
}

//
// qed_llh_set_ppfid_affinity(): Set the engine affinity for the given
// LLH filter bank.
//
// @cdev: Qed dev pointer.
// @ppfid: Relative within the allocated ppfids ('0' is the default one).
// @eng: Engine.
//
// Return: Int.
//
// qed_llh_set_roce_affinity(): Set the RoCE engine affinity.
//
// @cdev: Qed dev pointer.
// @eng: Engine.
//
// Return: Int.
//
extern "C" {
    pub fn qed_llh_set_roce_affinity(cdev: *mut qed_dev, eng: qed_eng) -> c_int;
}
//
// qed_llh_add_mac_filter(): Add a LLH MAC filter into the given filter
// bank.
//
// @cdev: Qed dev pointer.
// @ppfid: Relative within the allocated ppfids ('0' is the default one).
// @mac_addr: MAC to add.
//
// Return: Int.
//
// qed_llh_remove_mac_filter(): Remove a LLH MAC filter from the given
// filter bank.
//
// @cdev: Qed dev pointer.
// @ppfid: Ppfid.
// @mac_addr: MAC to remove
//
// Return: Void.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qed_llh_prot_filter_type_t {
    QED_LLH_FILTER_ETHERTYPE,
    QED_LLH_FILTER_TCP_SRC_PORT,
    QED_LLH_FILTER_TCP_DEST_PORT,
    QED_LLH_FILTER_TCP_SRC_AND_DEST_PORT,
    QED_LLH_FILTER_UDP_SRC_PORT,
    QED_LLH_FILTER_UDP_DEST_PORT,
    QED_LLH_FILTER_UDP_SRC_AND_DEST_PORT
}

//
// qed_llh_add_protocol_filter(): Add a LLH protocol filter into the
// given filter bank.
//
// @cdev: Qed dev pointer.
// @ppfid: Relative within the allocated ppfids ('0' is the default one).
// @type: Type of filters and comparing.
// @source_port_or_eth_type: Source port or ethertype to add.
// @dest_port: Destination port to add.
//
// Return: Int.
//
// qed_llh_remove_protocol_filter(): Remove a LLH protocol filter from
// the given filter bank.
//
// @cdev: Qed dev pointer.
// @ppfid: Relative within the allocated ppfids ('0' is the default one).
// @type: Type of filters and comparing.
// @source_port_or_eth_type: Source port or ethertype to add.
// @dest_port: Destination port to add.
//
// qed_final_cleanup(): Cleanup of previous driver remains prior to load.
//
// @p_hwfn: HW device data.
// @p_ptt: P_ptt.
// @id: For PF, engine-relative. For VF, PF-relative.
// @is_vf: True iff cleanup is made for a VF.
//
// Return: Int.
//
// qed_get_queue_coalesce(): Retrieve coalesce value for a given queue.
//
// @p_hwfn: HW device data.
// @coal: Store coalesce value read from the hardware.
// @handle: P_handle.
//
// Return: Int.
//
extern "C" {
    pub fn qed_get_queue_coalesce(p_hwfn: *mut qed_hwfn, coal: *mut u16, handle: *mut c_void) -> c_int;
}
//
// qed_set_queue_coalesce(): Configure coalesce parameters for Rx and
// Tx queue. The fact that we can configure coalescing to up to 511, but on
// varying accuracy [the bigger the value the less accurate] up to a mistake
// of 3usec for the highest values.
// While the API allows setting coalescing per-qid, all queues sharing a SB
// should be in same range [i.e., either 0-0x7f, 0x80-0xff or 0x100-0x1ff]
// otherwise configuration would break.
//
// @rx_coal: Rx Coalesce value in micro seconds.
// @tx_coal: TX Coalesce value in micro seconds.
// @p_handle: P_handle.
//
// Return: Int.
//
// qed_pglueb_set_pfid_enable(): Enable or disable PCI BUS MASTER.
//
// @p_hwfn: HW device data.
// @p_ptt: P_ptt.
// @b_enable: True/False.
//
// Return: Int.
//
// qed_db_recovery_add(): add doorbell information to the doorbell
// recovery mechanism.
//
// @cdev: Qed dev pointer.
// @db_addr: Doorbell address.
// @db_data: Address of where db_data is stored.
// @db_width: Doorbell is 32b pr 64b.
// @db_space: Doorbell recovery addresses are user or kernel space.
//
// Return: Int.
//
// qed_db_recovery_del() - remove doorbell information from the doorbell
// recovery mechanism. db_data serves as key (db_addr is not unique).
//
// @cdev: Qed dev pointer.
// @db_addr: doorbell address.
// @db_data: address where db_data is stored. Serves as key for the
// entry to delete.
//
// Return: Int.
//
