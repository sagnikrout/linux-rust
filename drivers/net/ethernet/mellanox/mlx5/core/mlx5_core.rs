//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlx5/core/mlx5_core.h
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


//
// Copyright (c) 2013-2015, Mellanox Technologies, Ltd.  All rights reserved.
//
// This software is available to you under a choice of one of two
// licenses.  You may choose to be licensed under the terms of the GNU
// General Public License (GPL) Version 2, available from the file
// COPYING in the main directory of this source tree, or the
// OpenIB.org BSD license below:
//
// Redistribution and use in source and binary forms, with or
// without modification, are permitted provided that the following
// conditions are met:
//
// - Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer.
//
// - Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following
// disclaimer in the documentation and/or other materials
// provided with the distribution.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
// BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
// ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//

pub const ACCESS_KEY_LEN: c_int = 32;
pub const FT_ID_FT_TYPE_OFFSET: c_int = 24;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_cmd_allow_other_vhca_access_attr {
    pub obj_type: u16,
    pub obj_id: u32,
    pub access_key: [u8; ACCESS_KEY_LEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_cmd_alias_obj_create_attr {
    pub obj_id: u32,
    pub vhca_id: u16,
    pub vhca_id_type: u8,
    pub obj_type: u16,
    pub access_key: [u8; ACCESS_KEY_LEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_port_eth_proto {
    pub cap: u32,
    pub admin: u32,
    pub oper: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_module_eeprom_query_params {
    pub size: u16,
    pub offset: u16,
    pub i2c_address: u16,
    pub page: u32,
    pub bank: u32,
    pub module_number: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_link_info {
    pub speed: u32,
    pub lanes: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_semaphore_space_address {
    MLX5_SEMAPHORE_SPACE_DOMAIN     = 0xA,
    MLX5_SEMAPHORE_SW_RESET         = 0x20,
}

pub const MLX5_DEFAULT_PROF: c_int = 2;
pub const MLX5_SF_PROF: c_int = 3;
pub const MLX5_NUM_FW_CMD_THREADS: c_int = 8;

extern "C" {
    pub fn mlx5_core_get_caps(dev: *mut mlx5_core_dev, cap_type: mlx5_cap_type) -> c_int;
}
extern "C" {
    pub fn mlx5_query_hca_caps(dev: *mut mlx5_core_dev) -> c_int;
}
extern "C" {
    pub fn mlx5_query_board_id(dev: *mut mlx5_core_dev) -> c_int;
}
extern "C" {
    pub fn mlx5_query_module_num(dev: *mut mlx5_core_dev, module_num: *mut c_int) -> c_int;
}
extern "C" {
    pub fn mlx5_cmd_init(dev: *mut mlx5_core_dev) -> c_int;
}
extern "C" {
    pub fn mlx5_cmd_cleanup(dev: *mut mlx5_core_dev);
}
extern "C" {
    pub fn mlx5_cmd_enable(dev: *mut mlx5_core_dev) -> c_int;
}
extern "C" {
    pub fn mlx5_cmd_disable(dev: *mut mlx5_core_dev);
}
extern "C" {
    pub fn mlx5_cmd_init_hca(dev: *mut mlx5_core_dev, sw_owner_id: *mut u32) -> c_int;
}
extern "C" {
    pub fn mlx5_cmd_teardown_hca(dev: *mut mlx5_core_dev) -> c_int;
}
extern "C" {
    pub fn mlx5_cmd_force_teardown_hca(dev: *mut mlx5_core_dev) -> c_int;
}
extern "C" {
    pub fn mlx5_cmd_fast_teardown_hca(dev: *mut mlx5_core_dev) -> c_int;
}
extern "C" {
    pub fn mlx5_enter_error_state(dev: *mut mlx5_core_dev, force: bool);
}
extern "C" {
    pub fn mlx5_error_sw_reset(dev: *mut mlx5_core_dev);
}
extern "C" {
    pub fn mlx5_health_check_fatal_sensors(dev: *mut mlx5_core_dev) -> u32;
}
extern "C" {
    pub fn mlx5_health_wait_pci_up(dev: *mut mlx5_core_dev) -> c_int;
}
extern "C" {
    pub fn mlx5_disable_device(dev: *mut mlx5_core_dev);
}
extern "C" {
    pub fn mlx5_recover_device(dev: *mut mlx5_core_dev) -> c_int;
}
extern "C" {
    pub fn mlx5_sriov_init(dev: *mut mlx5_core_dev) -> c_int;
}
extern "C" {
    pub fn mlx5_sriov_cleanup(dev: *mut mlx5_core_dev);
}
extern "C" {
    pub fn mlx5_sriov_attach(dev: *mut mlx5_core_dev) -> c_int;
}
extern "C" {
    pub fn mlx5_sriov_detach(dev: *mut mlx5_core_dev);
}
extern "C" {
    pub fn mlx5_core_sriov_configure(dev: *mut pci_dev, num_vfs: c_int) -> c_int;
}
extern "C" {
    pub fn mlx5_sriov_disable(pdev: *mut pci_dev, num_vf_change: bool);
}
extern "C" {
    pub fn mlx5_core_sriov_set_msix_vec_count(vf: *mut pci_dev, msix_vec_count: c_int) -> c_int;
}
extern "C" {
    pub fn mlx5_core_enable_hca(dev: *mut mlx5_core_dev, func_id: u16) -> c_int;
}
extern "C" {
    pub fn mlx5_core_disable_hca(dev: *mut mlx5_core_dev, func_id: u16) -> c_int;
}
extern "C" {
    pub fn mlx5_qos_element_type_supported(dev: *mut mlx5_core_dev, type: c_int, hierarchy: u8) -> bool;
}
extern "C" {
    pub fn mlx5_qos_tsar_type_supported(dev: *mut mlx5_core_dev, type: c_int, hierarchy: u8) -> bool;
}
extern "C" {
    pub fn mlx5_wait_for_pages(dev: *mut mlx5_core_dev, pages: *mut c_int) -> c_int;
}
extern "C" {
    pub fn mlx5_cmd_flush(dev: *mut mlx5_core_dev);
}
extern "C" {
    pub fn mlx5_cq_debugfs_init(dev: *mut mlx5_core_dev);
}
extern "C" {
    pub fn mlx5_cq_debugfs_cleanup(dev: *mut mlx5_core_dev);
}
extern "C" {
    pub fn mlx5_vhca_debugfs_init(dev: *mut mlx5_core_dev);
}
extern "C" {
    pub fn mlx5_query_mpir_reg(dev: *mut mlx5_core_dev, mpir: *mut u32) -> c_int;
}
extern "C" {
    pub fn mlx5_lag_add_netdev(dev: *mut mlx5_core_dev, netdev: *mut net_device);
}
extern "C" {
    pub fn mlx5_lag_remove_netdev(dev: *mut mlx5_core_dev, netdev: *mut net_device);
}
extern "C" {
    pub fn mlx5_lag_add_mdev(dev: *mut mlx5_core_dev);
}
extern "C" {
    pub fn mlx5_lag_remove_mdev(dev: *mut mlx5_core_dev);
}
extern "C" {
    pub fn mlx5_lag_disable_change(dev: *mut mlx5_core_dev);
}
extern "C" {
    pub fn mlx5_lag_enable_change(dev: *mut mlx5_core_dev);
}
extern "C" {
    pub fn mlx5_events_init(dev: *mut mlx5_core_dev) -> c_int;
}
extern "C" {
    pub fn mlx5_events_cleanup(dev: *mut mlx5_core_dev);
}
extern "C" {
    pub fn mlx5_events_start(dev: *mut mlx5_core_dev);
}
extern "C" {
    pub fn mlx5_events_stop(dev: *mut mlx5_core_dev);
}
extern "C" {
    pub fn mlx5_adev_idx_alloc() -> c_int;
}
extern "C" {
    pub fn mlx5_adev_idx_free(idx: c_int);
}
extern "C" {
    pub fn mlx5_adev_cleanup(dev: *mut mlx5_core_dev);
}
extern "C" {
    pub fn mlx5_adev_init(dev: *mut mlx5_core_dev) -> c_int;
}
extern "C" {
    pub fn mlx5_attach_device(dev: *mut mlx5_core_dev) -> c_int;
}
extern "C" {
    pub fn mlx5_detach_device(dev: *mut mlx5_core_dev, suspend: bool);
}
extern "C" {
    pub fn mlx5_register_device(dev: *mut mlx5_core_dev) -> c_int;
}
extern "C" {
    pub fn mlx5_unregister_device(dev: *mut mlx5_core_dev);
}
extern "C" {
    pub fn mlx5_dev_set_lightweight(dev: *mut mlx5_core_dev);
}
extern "C" {
    pub fn mlx5_dev_is_lightweight(dev: *mut mlx5_core_dev) -> bool;
}
extern "C" {
    pub fn mlx5_core_reps_aux_devs_remove(dev: *mut mlx5_core_dev);
}
extern "C" {
    pub fn mlx5_fw_reporters_create(dev: *mut mlx5_core_dev);
}
extern "C" {
    pub fn mlx5_query_mtpps(dev: *mut mlx5_core_dev, mtpps: *mut u32, mtpps_size: u32) -> c_int;
}
extern "C" {
    pub fn mlx5_set_mtpps(mdev: *mut mlx5_core_dev, mtpps: *mut u32, mtpps_size: u32) -> c_int;
}
extern "C" {
    pub fn mlx5_set_mtppse(mdev: *mut mlx5_core_dev, pin: u8, arm: u8, mode: u8) -> c_int;
}
extern "C" {
    pub fn mlx5_dm_cleanup(dev: *mut mlx5_core_dev);
}

extern "C" {
    pub fn mlx5_st_destroy(dev: *mut mlx5_core_dev);
}

extern "C" {
    pub fn mlx5_toggle_port_link(dev: *mut mlx5_core_dev);
}
extern "C" {
    pub fn mlx5_set_port_beacon(dev: *mut mlx5_core_dev, beacon_duration: u16) -> c_int;
}
extern "C" {
    pub fn mlx5_set_port_mtu(dev: *mut mlx5_core_dev, mtu: u16, port: u8) -> c_int;
}
extern "C" {
    pub fn mlx5_set_port_pause(dev: *mut mlx5_core_dev, rx_pause: u32, tx_pause: u32) -> c_int;
}
extern "C" {
    pub fn mlx5_set_port_pfc(dev: *mut mlx5_core_dev, pfc_en_tx: u8, pfc_en_rx: u8) -> c_int;
}
extern "C" {
    pub fn mlx5_max_tc(mdev: *mut mlx5_core_dev) -> c_int;
}
extern "C" {
    pub fn mlx5_set_port_prio_tc(mdev: *mut mlx5_core_dev, prio_tc: *mut u8) -> c_int;
}
extern "C" {
    pub fn mlx5_set_port_tc_group(mdev: *mut mlx5_core_dev, tc_group: *mut u8) -> c_int;
}
extern "C" {
    pub fn mlx5_set_port_tc_bw_alloc(mdev: *mut mlx5_core_dev, tc_bw: *mut u8) -> c_int;
}
extern "C" {
    pub fn mlx5_set_port_wol(mdev: *mut mlx5_core_dev, wol_mode: u8) -> c_int;
}
extern "C" {
    pub fn mlx5_query_port_wol(mdev: *mut mlx5_core_dev, wol_mode: *mut u8) -> c_int;
}
extern "C" {
    pub fn mlx5_query_ports_check(mdev: *mut mlx5_core_dev, out: *mut u32, outlen: c_int) -> c_int;
}
extern "C" {
    pub fn mlx5_set_ports_check(mdev: *mut mlx5_core_dev, in: *mut u32, inlen: c_int) -> c_int;
}
extern "C" {
    pub fn mlx5_set_port_fcs(mdev: *mut mlx5_core_dev, enable: u8) -> c_int;
}
extern "C" {
    pub fn mlx5_query_port_dcbx_param(mdev: *mut mlx5_core_dev, out: *mut u32) -> c_int;
}
extern "C" {
    pub fn mlx5_set_port_dcbx_param(mdev: *mut mlx5_core_dev, in: *mut u32) -> c_int;
}
extern "C" {
    pub fn mlx5_set_trust_state(mdev: *mut mlx5_core_dev, trust_state: u8) -> c_int;
}
extern "C" {
    pub fn mlx5_query_trust_state(mdev: *mut mlx5_core_dev, trust_state: *mut u8) -> c_int;
}
extern "C" {
    pub fn mlx5_set_dscp2prio(mdev: *mut mlx5_core_dev, dscp: u8, prio: u8) -> c_int;
}
extern "C" {
    pub fn mlx5_query_dscp2prio(mdev: *mut mlx5_core_dev, dscp2prio: *mut u8) -> c_int;
}
extern "C" {
    pub fn mlx5_ptys_ext_supported(mdev: *mut mlx5_core_dev) -> bool;
}
extern "C" {
    pub fn mlx5_port_oper_linkspeed(mdev: *mut mlx5_core_dev, speed: *mut u32) -> c_int;
}
extern "C" {
    pub fn mlx5_port_max_linkspeed(mdev: *mut mlx5_core_dev, speed: *mut u32) -> c_int;
}

extern "C" {
    pub fn mlx5e_init() -> c_int;
}
extern "C" {
    pub fn mlx5e_cleanup();
}

extern "C" {
    pub fn mlx5_rescan_drivers_locked(dev: *mut mlx5_core_dev) -> c_int;
}
extern "C" {
    pub fn mlx5_get_nic_state(dev: *mut mlx5_core_dev) -> u8;
}
extern "C" {
    pub fn mlx5_set_nic_state(dev: *mut mlx5_core_dev, state: u8);
}
extern "C" {
    pub fn container_of(_arg: mdev->device, auxiliary_device: struct, _arg: dev) -> return;
}
extern "C" {
    pub fn mlx5_mdev_init(dev: *mut mlx5_core_dev, profile_idx: c_int) -> c_int;
}
extern "C" {
    pub fn mlx5_mdev_uninit(dev: *mut mlx5_core_dev);
}
extern "C" {
    pub fn mlx5_frag_buf_pools_init(dev: *mut mlx5_core_dev) -> c_int;
}
extern "C" {
    pub fn mlx5_frag_buf_pools_cleanup(dev: *mut mlx5_core_dev);
}
extern "C" {
    pub fn mlx5_db_pools_init(dev: *mut mlx5_core_dev) -> c_int;
}
extern "C" {
    pub fn mlx5_db_pools_cleanup(dev: *mut mlx5_core_dev);
}
extern "C" {
    pub fn mlx5_init_one(dev: *mut mlx5_core_dev) -> c_int;
}
extern "C" {
    pub fn mlx5_init_one_devl_locked(dev: *mut mlx5_core_dev) -> c_int;
}
extern "C" {
    pub fn mlx5_uninit_one(dev: *mut mlx5_core_dev);
}
extern "C" {
    pub fn mlx5_unload_one(dev: *mut mlx5_core_dev, suspend: bool);
}
extern "C" {
    pub fn mlx5_unload_one_devl_locked(dev: *mut mlx5_core_dev, suspend: bool);
}
extern "C" {
    pub fn mlx5_load_one(dev: *mut mlx5_core_dev, recovery: bool) -> c_int;
}
extern "C" {
    pub fn mlx5_load_one_devl_locked(dev: *mut mlx5_core_dev, recovery: bool) -> c_int;
}
extern "C" {
    pub fn mlx5_init_one_light(dev: *mut mlx5_core_dev) -> c_int;
}
extern "C" {
    pub fn mlx5_uninit_one_light(dev: *mut mlx5_core_dev);
}
extern "C" {
    pub fn mlx5_unload_one_light(dev: *mut mlx5_core_dev);
}

extern "C" {
    pub fn MLX5_CAP_GEN_MAX(_arg: dev, _arg: num_total_dynamic_vf_msix) -> return;
}
extern "C" {
    pub fn mlx5_eth_supported(dev: *mut mlx5_core_dev) -> bool;
}
extern "C" {
    pub fn mlx5_rdma_supported(dev: *mut mlx5_core_dev) -> bool;
}
extern "C" {
    pub fn mlx5_vnet_supported(dev: *mut mlx5_core_dev) -> bool;
}
extern "C" {
    pub fn mlx5_same_hw_devs(dev: *mut mlx5_core_dev, peer_dev: *mut mlx5_core_dev) -> bool;
}
extern "C" {
    pub fn mlx5_cmd_alias_obj_destroy(dev: *mut mlx5_core_dev, obj_id: u32, obj_type: u16) -> c_int;
}
extern "C" {
    pub fn MLX5_CAP_GEN_2(_arg: dev, _arg: ec_vf_vport_base) -> return;
}
extern "C" {
    pub fn mlx5_core_is_ecpf(mlx5_core_ec_vf_vport_base(dev: dev) &&) -> return;
}
extern "C" {
    pub fn MLX5_CAP_GEN_2(_arg: dev, _arg: max_num_eqs_24b) -> return;
}
extern "C" {
    pub fn MLX5_CAP_GEN(_arg: dev, _arg: max_num_eqs) -> return;
}
