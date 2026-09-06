//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/broadcom/bnx2x/bnx2x_cmn.h
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


// bnx2x_cmn.h: QLogic Everest network driver.
//
// Copyright (c) 2007-2013 Broadcom Corporation
// Copyright (c) 2014 QLogic Corporation
// All rights reserved
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation.
//
// Maintained by: Ariel Elior <ariel.elior@qlogic.com>
// Written by: Eliezer Tamir
// Based on code from Michael Chan's bnx2 driver
// UDP CSUM errata workaround by Arik Gendelman
// Slowpath and fastpath rework by Vladislav Zolotarov
// Statistics and Link management by Yitchak Gertner
//

// This is used as a replacement for an MCP if it's not present
// Macros

// Interfaces
// Functions that need to be implemented by each driver version
//
// Init
//
// bnx2x_send_unload_req - request unload mode from the MCP.
//
// @bp:			driver handle
// @unload_mode:	requested function's unload mode
//
// Return unload mode returned by the MCP: COMMON, PORT or FUNC.
//
extern "C" {
    pub fn bnx2x_send_unload_req(bp: *mut bnx2x, unload_mode: c_int) -> u32;
}
//
// bnx2x_send_unload_done - send UNLOAD_DONE command to the MCP.
//
// @bp:		driver handle
// @keep_link:		true iff link should be kept up
//
extern "C" {
    pub fn bnx2x_send_unload_done(bp: *mut bnx2x, keep_link: bool);
}
//
// bnx2x_config_rss_pf - configure RSS parameters in a PF.
//
// @bp:			driver handle
// @rss_obj:		RSS object to use
// @ind_table:		indirection table to configure
// @config_hash:	re-configure RSS hash keys configuration
// @enable:		enabled or disabled configuration
//
// bnx2x__init_func_obj - init function object
//
// @bp:			driver handle
//
// Initializes the Function Object with the appropriate
// parameters which include a function slow path driver
// interface.
//
extern "C" {
    pub fn bnx2x__init_func_obj(bp: *mut bnx2x);
}
//
// bnx2x_setup_queue - setup eth queue.
//
// @bp:		driver handle
// @fp:		pointer to the fastpath structure
// @leading:	boolean
//
// bnx2x_setup_leading - bring up a leading eth queue.
//
// @bp:		driver handle
//
extern "C" {
    pub fn bnx2x_setup_leading(bp: *mut bnx2x) -> c_int;
}
//
// bnx2x_fw_command - send the MCP a request
//
// @bp:		driver handle
// @command:	request
// @param:	request's parameter
//
// block until there is a reply
//
extern "C" {
    pub fn bnx2x_fw_command(bp: *mut bnx2x, command: u32, param: u32) -> u32;
}
//
// bnx2x_initial_phy_init - initialize link parameters structure variables.
//
// @bp:		driver handle
// @load_mode:	current mode
//
extern "C" {
    pub fn bnx2x_initial_phy_init(bp: *mut bnx2x, load_mode: c_int) -> c_int;
}
//
// bnx2x_link_set - configure hw according to link parameters structure.
//
// @bp:		driver handle
//
extern "C" {
    pub fn bnx2x_link_set(bp: *mut bnx2x);
}
//
// bnx2x_force_link_reset - Forces link reset, and put the PHY
// in reset as well.
//
// @bp:		driver handle
//
extern "C" {
    pub fn bnx2x_force_link_reset(bp: *mut bnx2x);
}
//
// bnx2x_link_test - query link status.
//
// @bp:		driver handle
// @is_serdes:	bool
//
// Returns 0 if link is UP.
//
extern "C" {
    pub fn bnx2x_link_test(bp: *mut bnx2x, is_serdes: u8) -> u8;
}
//
// bnx2x_drv_pulse - write driver pulse to shmem
//
// @bp:		driver handle
//
// writes the value in bp->fw_drv_pulse_wr_seq to drv_pulse mbox
// in the shmem.
//
extern "C" {
    pub fn bnx2x_drv_pulse(bp: *mut bnx2x);
}
//
// bnx2x_igu_ack_sb - update IGU with current SB value
//
// @bp:		driver handle
// @igu_sb_id:	SB id
// @segment:	SB segment
// @index:	SB index
// @op:		SB operation
// @update:	is HW update required
//
// Disable transactions from chip to host
extern "C" {
    pub fn bnx2x_pf_disable(bp: *mut bnx2x);
}
extern "C" {
    pub fn bnx2x_pretend_func(bp: *mut bnx2x, pretend_func_val: u16) -> c_int;
}
//
// bnx2x__link_status_update - handles link status change.
//
// @bp:		driver handle
//
extern "C" {
    pub fn bnx2x__link_status_update(bp: *mut bnx2x);
}
//
// bnx2x_link_report - report link status to upper layer.
//
// @bp:		driver handle
//
extern "C" {
    pub fn bnx2x_link_report(bp: *mut bnx2x);
}
// None-atomic version of bnx2x_link_report()
extern "C" {
    pub fn __bnx2x_link_report(bp: *mut bnx2x);
}
//
// bnx2x_get_mf_speed - calculate MF speed.
//
// @bp:		driver handle
//
// Takes into account current linespeed and MF configuration.
//
extern "C" {
    pub fn bnx2x_get_mf_speed(bp: *mut bnx2x) -> u16;
}
//
// bnx2x_msix_sp_int - MSI-X slowpath interrupt handler
//
// @irq:		irq number
// @dev_instance:	private instance
//
extern "C" {
    pub fn bnx2x_msix_sp_int(irq: c_int, dev_instance: *mut c_void) -> irqreturn_t;
}
//
// bnx2x_interrupt - non MSI-X interrupt handler
//
// @irq:		irq number
// @dev_instance:	private instance
//
extern "C" {
    pub fn bnx2x_interrupt(irq: c_int, dev_instance: *mut c_void) -> irqreturn_t;
}
//
// bnx2x_cnic_notify - send command to cnic driver
//
// @bp:		driver handle
// @cmd:	command
//
extern "C" {
    pub fn bnx2x_cnic_notify(bp: *mut bnx2x, cmd: c_int) -> c_int;
}
//
// bnx2x_setup_cnic_irq_info - provides cnic with IRQ information
//
// @bp:		driver handle
//
extern "C" {
    pub fn bnx2x_setup_cnic_irq_info(bp: *mut bnx2x);
}
//
// bnx2x_setup_cnic_info - provides cnic with updated info
//
// @bp:		driver handle
//
extern "C" {
    pub fn bnx2x_setup_cnic_info(bp: *mut bnx2x);
}
//
// bnx2x_int_enable - enable HW interrupts.
//
// @bp:		driver handle
//
extern "C" {
    pub fn bnx2x_int_enable(bp: *mut bnx2x);
}
//
// bnx2x_int_disable_sync - disable interrupts.
//
// @bp:		driver handle
// @disable_hw:	true, disable HW interrupts.
//
// This function ensures that there are no
// ISRs or SP DPCs (sp_task) are running after it returns.
//
extern "C" {
    pub fn bnx2x_int_disable_sync(bp: *mut bnx2x, disable_hw: c_int);
}
//
// bnx2x_nic_init_cnic - init driver internals for cnic.
//
// @bp:		driver handle
// @load_code:	COMMON, PORT or FUNCTION
//
// Initializes:
// - rings
// - status blocks
// - etc.
//
extern "C" {
    pub fn bnx2x_nic_init_cnic(bp: *mut bnx2x);
}
//
// bnx2x_preirq_nic_init - init driver internals.
//
// @bp:		driver handle
//
// Initializes:
// - fastpath object
// - fastpath rings
// etc.
//
extern "C" {
    pub fn bnx2x_pre_irq_nic_init(bp: *mut bnx2x);
}
//
// bnx2x_postirq_nic_init - init driver internals.
//
// @bp:		driver handle
// @load_code:	COMMON, PORT or FUNCTION
//
// Initializes:
// - status blocks
// - slowpath rings
// - etc.
//
extern "C" {
    pub fn bnx2x_post_irq_nic_init(bp: *mut bnx2x, load_code: u32);
}
//
// bnx2x_alloc_mem_cnic - allocate driver's memory for cnic.
//
// @bp:		driver handle
//
extern "C" {
    pub fn bnx2x_alloc_mem_cnic(bp: *mut bnx2x) -> c_int;
}
//
// bnx2x_alloc_mem - allocate driver's memory.
//
// @bp:		driver handle
//
extern "C" {
    pub fn bnx2x_alloc_mem(bp: *mut bnx2x) -> c_int;
}
//
// bnx2x_free_mem_cnic - release driver's memory for cnic.
//
// @bp:		driver handle
//
extern "C" {
    pub fn bnx2x_free_mem_cnic(bp: *mut bnx2x);
}
//
// bnx2x_free_mem - release driver's memory.
//
// @bp:		driver handle
//
extern "C" {
    pub fn bnx2x_free_mem(bp: *mut bnx2x);
}
//
// bnx2x_set_num_queues - set number of queues according to mode.
//
// @bp:		driver handle
//
extern "C" {
    pub fn bnx2x_set_num_queues(bp: *mut bnx2x);
}
//
// bnx2x_chip_cleanup - cleanup chip internals.
//
// @bp:			driver handle
// @unload_mode:	COMMON, PORT, FUNCTION
// @keep_link:		true iff link should be kept up.
//
// - Cleanup MAC configuration.
// - Closes clients.
// - etc.
//
extern "C" {
    pub fn bnx2x_chip_cleanup(bp: *mut bnx2x, unload_mode: c_int, keep_link: bool);
}
//
// bnx2x_acquire_hw_lock - acquire HW lock.
//
// @bp:		driver handle
// @resource:	resource bit which was locked
//
extern "C" {
    pub fn bnx2x_acquire_hw_lock(bp: *mut bnx2x, resource: u32) -> c_int;
}
//
// bnx2x_release_hw_lock - release HW lock.
//
// @bp:		driver handle
// @resource:	resource bit which was locked
//
extern "C" {
    pub fn bnx2x_release_hw_lock(bp: *mut bnx2x, resource: u32) -> c_int;
}
//
// bnx2x_release_leader_lock - release recovery leader lock
//
// @bp:		driver handle
//
extern "C" {
    pub fn bnx2x_release_leader_lock(bp: *mut bnx2x) -> c_int;
}
//
// bnx2x_set_eth_mac - configure eth MAC address in the HW
//
// @bp:		driver handle
// @set:	set or clear
//
// Configures according to the value in netdev->dev_addr.
//
extern "C" {
    pub fn bnx2x_set_eth_mac(bp: *mut bnx2x, set: bool) -> c_int;
}
//
// bnx2x_set_rx_mode - set MAC filtering configurations.
//
// @dev:	netdevice
//
// called with netif_tx_lock from dev_mcast.c
// If bp->state is OPEN, should be called with
// netif_addr_lock_bh()
//
extern "C" {
    pub fn bnx2x_set_rx_mode_inner(bp: *mut bnx2x);
}
// Parity errors related
extern "C" {
    pub fn bnx2x_set_pf_load(bp: *mut bnx2x);
}
extern "C" {
    pub fn bnx2x_clear_pf_load(bp: *mut bnx2x) -> bool;
}
extern "C" {
    pub fn bnx2x_chk_parity_attn(bp: *mut bnx2x, global: *mut bool, print: bool) -> bool;
}
extern "C" {
    pub fn bnx2x_reset_is_done(bp: *mut bnx2x, engine: c_int) -> bool;
}
extern "C" {
    pub fn bnx2x_set_reset_in_progress(bp: *mut bnx2x);
}
extern "C" {
    pub fn bnx2x_set_reset_global(bp: *mut bnx2x);
}
extern "C" {
    pub fn bnx2x_disable_close_the_gate(bp: *mut bnx2x);
}
extern "C" {
    pub fn bnx2x_init_hw_func_cnic(bp: *mut bnx2x) -> c_int;
}
extern "C" {
    pub fn bnx2x_clear_vlan_info(bp: *mut bnx2x);
}
//
// bnx2x_sp_event - handle ramrods completion.
//
// @fp:		fastpath handle for the event
// @rr_cqe:	eth_rx_cqe
//
extern "C" {
    pub fn bnx2x_sp_event(fp: *mut bnx2x_fastpath, rr_cqe: *mut eth_rx_cqe);
}
//
// bnx2x_ilt_set_info - prepare ILT configurations.
//
// @bp:		driver handle
//
extern "C" {
    pub fn bnx2x_ilt_set_info(bp: *mut bnx2x);
}
//
// bnx2x_ilt_set_cnic_info - prepare ILT configurations for SRC
// and TM.
//
// @bp:		driver handle
//
extern "C" {
    pub fn bnx2x_ilt_set_info_cnic(bp: *mut bnx2x);
}
//
// bnx2x_dcbx_init - initialize dcbx protocol.
//
// @bp:		driver handle
//
extern "C" {
    pub fn bnx2x_dcbx_init(bp: *mut bnx2x, update_shmem: bool);
}
//
// bnx2x_set_power_state - set power state to the requested value.
//
// @bp:		driver handle
// @state:	required state D0 or D3hot
//
// Currently only D0 and D3hot are supported.
//
extern "C" {
    pub fn bnx2x_set_power_state(bp: *mut bnx2x, state: pci_power_t) -> c_int;
}
//
// bnx2x_update_max_mf_config - update MAX part of MF configuration in HW.
//
// @bp:		driver handle
// @value:	new value
//
extern "C" {
    pub fn bnx2x_update_max_mf_config(bp: *mut bnx2x, value: u32);
}
// Error handling
extern "C" {
    pub fn bnx2x_fw_dump_lvl(bp: *mut bnx2x, lvl: *const c_char);
}
// dev_close main block
extern "C" {
    pub fn bnx2x_nic_unload(bp: *mut bnx2x, unload_mode: c_int, keep_link: bool) -> c_int;
}
// dev_open main block
extern "C" {
    pub fn bnx2x_nic_load(bp: *mut bnx2x, load_mode: c_int) -> c_int;
}
// hard_xmit callback
extern "C" {
    pub fn bnx2x_start_xmit(skb: *mut sk_buff, dev: *mut net_device) -> netdev_tx_t;
}
// setup_tc callback
extern "C" {
    pub fn bnx2x_setup_tc(dev: *mut net_device, num_tc: u8) -> c_int;
}
extern "C" {
    pub fn bnx2x_set_vf_mac(dev: *mut net_device, queue: c_int, mac: *mut u8) -> c_int;
}
extern "C" {
    pub fn bnx2x_set_vf_spoofchk(dev: *mut net_device, idx: c_int, val: bool) -> c_int;
}
// select_queue callback
// Update producers
// Make sure that the BD and SGE data is updated before updating the
// producers since FW might read the BD/SGE right after the producer
// is updated.
// This is only applicable for weak-ordered memory model archs such
// as IA-64. The following barrier is also mandatory since FW will
// assumes BDs must have buffers.
//
// reload helper
extern "C" {
    pub fn bnx2x_reload_if_running(dev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn bnx2x_change_mac_addr(dev: *mut net_device, p: *mut c_void) -> c_int;
}
// NAPI poll Tx part
extern "C" {
    pub fn bnx2x_tx_int(bp: *mut bnx2x, txdata: *mut bnx2x_fp_txdata) -> c_int;
}
// Release IRQ vectors
extern "C" {
    pub fn bnx2x_free_irq(bp: *mut bnx2x);
}
extern "C" {
    pub fn bnx2x_free_fp_mem(bp: *mut bnx2x);
}
extern "C" {
    pub fn bnx2x_init_rx_rings(bp: *mut bnx2x);
}
extern "C" {
    pub fn bnx2x_init_rx_rings_cnic(bp: *mut bnx2x);
}
extern "C" {
    pub fn bnx2x_free_skbs(bp: *mut bnx2x);
}
extern "C" {
    pub fn bnx2x_netif_stop(bp: *mut bnx2x, disable_hw: c_int);
}
extern "C" {
    pub fn bnx2x_netif_start(bp: *mut bnx2x);
}
extern "C" {
    pub fn bnx2x_load_cnic(bp: *mut bnx2x) -> c_int;
}
//
// bnx2x_enable_msix - set msix configuration.
//
// @bp:		driver handle
//
// fills msix_table, requests vectors, updates num_queues
// according to number of available vectors.
//
extern "C" {
    pub fn bnx2x_enable_msix(bp: *mut bnx2x) -> c_int;
}
//
// bnx2x_enable_msi - request msi mode from OS, updated internals accordingly
//
// @bp:		driver handle
//
extern "C" {
    pub fn bnx2x_enable_msi(bp: *mut bnx2x) -> c_int;
}
//
// bnx2x_alloc_mem_bp - allocate memories outsize main driver structure
//
// @bp:		driver handle
//
extern "C" {
    pub fn bnx2x_alloc_mem_bp(bp: *mut bnx2x) -> c_int;
}
//
// bnx2x_free_mem_bp - release memories outsize main driver structure
//
// @bp:		driver handle
//
extern "C" {
    pub fn bnx2x_free_mem_bp(bp: *mut bnx2x);
}
//
// bnx2x_change_mtu - change mtu netdev callback
//
// @dev:	net device
// @new_mtu:	requested mtu
//
extern "C" {
    pub fn bnx2x_change_mtu(dev: *mut net_device, new_mtu: c_int) -> c_int;
}

//
// bnx2x_fcoe_get_wwn - return the requested WWN value for this port
//
// @dev:	net_device
// @wwn:	output buffer
// @type:	WWN type: NETDEV_FCOE_WWNN (node) or NETDEV_FCOE_WWPN (port)
//
extern "C" {
    pub fn bnx2x_fcoe_get_wwn(dev: *mut net_device, wwn: *mut u64, type: c_int) -> c_int;
}

extern "C" {
    pub fn bnx2x_set_features(dev: *mut net_device, features: netdev_features_t) -> c_int;
}
//
// bnx2x_tx_timeout - tx timeout netdev callback
//
// @dev:	net device
//
extern "C" {
    pub fn bnx2x_tx_timeout(dev: *mut net_device, txqueue: c_uint);
}
// bnx2x_get_c2s_mapping - read inner-to-outer vlan configuration
// c2s_map should have BNX2X_MAX_PRIORITY entries.
// @bp:			driver handle
// @c2s_map:		should have BNX2X_MAX_PRIORITY entries for mapping
// @c2s_default:	entry for non-tagged configuration
//
extern "C" {
    pub fn bnx2x_get_c2s_mapping(bp: *mut bnx2x, c2s_map: *mut u8, c2s_default: *mut u8);
}
// Inlines
// Fast path
// Make sure that ACK is written
extern "C" {
    pub fn bnx2x_hc_ack_int(_arg: bp) -> return;
}
extern "C" {
    pub fn bnx2x_igu_ack_int(_arg: bp) -> return;
}
// Tell compiler that consumer and producer can change

// Tell compiler that status block fields can change

extern "C" {
    pub fn BNX2X_IS_CQE_COMPLETED(_arg: cqe_fp) -> return;
}
//
// bnx2x_tx_disable - disables tx from stack point of view
//
// @bp:		driver handle
//
// Skip "next page" elements
// Since many fragments can share the same page, make sure to
// only unmap and free the page once.
//
extern "C" {
    pub fn bnx2x_set_int_mode(bp: *mut bnx2x) -> c_int;
}
// Set the mask to all 1-s: it's faster to compare to 0 than to 0xf-s
// Clear the two last indices in the page to 1:
// note that we are not allocating a new buffer,
// we are just moving one from cons to prod
// we are not creating a new mapping,
// so there is no need to check for dma_mapping_error().
//
// prod_bd = *cons_bd;
// Init
// returns func by VN for current port
extern "C" {
    pub fn bnx2x_rss(_arg: bp, _arg: &bp->rss_conf_obj, _arg: config_hash, _arg: true) -> return;
}
//
// bnx2x_func_start - init function
//
// @bp:		driver handle
//
// Must be called before sending CLIENT_SETUP for the first client.
//
// Prepare parameters for function state transitions
// Function parameters
// Configure Ethertype for BD mode
extern "C" {
    pub fn bnx2x_func_state_change(_arg: bp, _arg: &func_params) -> return;
}
//
// bnx2x_set_fw_mac_addr - fill in a MAC address in FW format
//
// @fw_hi:	pointer to upper part
// @fw_mid:	pointer to middle part
// @fw_lo:	pointer to lower part
// @mac:	pointer to MAC address
//
// Statistics ID are global per chip/path, while Client IDs for E1x are per
// port.
//
// there are special statistics counters for FCoE 136..140
// Configure classification DBs
//
// bnx2x_get_path_func_num - get number of active functions
//
// @bp:		driver handle
//
// Calculates the number of active (not hidden) functions on the
// current path.
//
// 57710 has only one function per-port
// Calculate a number of functions enabled on the current
// PATH/PORT.
//
// RX_MODE controlling object
// multicast configuration controlling object
// Setup CAM credit pools
// RSS configuration object
// the 'first' id is allocated for the cnic

extern "C" {
    pub fn bnx2x_get_link_cfg_idx(bp: *mut bnx2x) -> c_int;
}
//
// bnx2x_wait_sp_comp - wait for the outstanding SP commands.
//
// @bp:		driver handle
// @mask:	bits that need to be cleared
//
// bnx2x_set_ctx_validation - set CDU context validation values
//
// @bp:		driver handle
// @cxt:	context of the connection on the host memory
// @cid:	SW CID of the connection to be configured
//
extern "C" {
    pub fn bnx2x_acquire_phy_lock(bp: *mut bnx2x);
}
extern "C" {
    pub fn bnx2x_release_phy_lock(bp: *mut bnx2x);
}
//
// bnx2x_extract_max_cfg - extract MAX BW part from MF configuration.
//
// @bp:		driver handle
// @mf_cfg:	MF configuration
//
// checks if HW supports GRO for given MTU
// gro frags per page
//
// 1. Number of frags should not grow above MAX_SKB_FRAGS
// 2. Frag must fit the page
//
// bnx2x_get_iscsi_info - update iSCSI params according to licensing info.
//
// @bp:		driver handle
//
extern "C" {
    pub fn bnx2x_get_iscsi_info(bp: *mut bnx2x);
}
//
// bnx2x_link_sync_notify - send notification to other functions.
//
// @bp:		driver handle
//
// Set the attention towards other drivers on the same port
//
// bnx2x_update_drv_flags - update flags in shmem
//
// @bp:		driver handle
// @flags:	flags to update
// @set:	set or clear
//
// bnx2x_fill_fw_str - Fill buffer with FW version string
//
// @bp:        driver handle
// @buf:       character buffer to fill with the fw name
// @buf_len:   length of the above buffer
//
extern "C" {
    pub fn bnx2x_fill_fw_str(bp: *mut bnx2x, buf: *mut c_char, buf_len: usize);
}
extern "C" {
    pub fn bnx2x_drain_tx_queues(bp: *mut bnx2x) -> c_int;
}
extern "C" {
    pub fn bnx2x_squeeze_objects(bp: *mut bnx2x);
}
//
// bnx2x_set_os_driver_state - write driver state for management FW usage
//
// @bp:		driver handle
// @state:	OS_DRIVER_STATE_* value reflecting current driver state
//
extern "C" {
    pub fn bnx2x_set_os_driver_state(bp: *mut bnx2x, state: u32);
}
//
// bnx2x_nvram_read - reads data from nvram [might sleep]
//
// @bp:		driver handle
// @offset:	byte offset in nvram
// @ret_buf:	pointer to buffer where data is to be stored
// @buf_size:   Length of 'ret_buf' in bytes
//
