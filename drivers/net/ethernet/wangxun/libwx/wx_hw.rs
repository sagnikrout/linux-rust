//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/wangxun/libwx/wx_hw.h
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
// Copyright (c) 2015 - 2022 Beijing WangXun Technology Co., Ltd.

extern "C" {
    pub fn wx_phy_read_reg_mdi_c22(bus: *mut mii_bus, phy_addr: c_int, regnum: c_int) -> c_int;
}
extern "C" {
    pub fn wx_phy_write_reg_mdi_c22(bus: *mut mii_bus, phy_addr: c_int, regnum: c_int, value: u16) -> c_int;
}
extern "C" {
    pub fn wx_phy_read_reg_mdi_c45(bus: *mut mii_bus, phy_addr: c_int, devnum: c_int, regnum: c_int) -> c_int;
}
extern "C" {
    pub fn wx_intr_enable(wx: *mut wx, qmask: u64);
}
extern "C" {
    pub fn wx_irq_disable(wx: *mut wx);
}
extern "C" {
    pub fn wx_check_flash_load(wx: *mut wx, check_bit: u32) -> c_int;
}
extern "C" {
    pub fn wx_control_hw(wx: *mut wx, drv: bool);
}
extern "C" {
    pub fn wx_mng_present(wx: *mut wx) -> c_int;
}
extern "C" {
    pub fn wx_set_pps(wx: *mut wx, enable: bool, nsec: u64, cycles: u64) -> c_int;
}
extern "C" {
    pub fn wx_read_ee_hostif(wx: *mut wx, offset: u16, data: *mut u16) -> c_int;
}
extern "C" {
    pub fn wx_init_eeprom_params(wx: *mut wx);
}
extern "C" {
    pub fn wx_get_mac_addr(wx: *mut wx, mac_addr: *mut u8);
}
extern "C" {
    pub fn wx_init_rx_addrs(wx: *mut wx);
}
extern "C" {
    pub fn wx_mac_set_default_filter(wx: *mut wx, addr: *mut u8);
}
extern "C" {
    pub fn wx_add_mac_filter(wx: *mut wx, addr: *mut u8, pool: u16) -> c_int;
}
extern "C" {
    pub fn wx_del_mac_filter(wx: *mut wx, addr: *mut u8, pool: u16) -> c_int;
}
extern "C" {
    pub fn wx_flush_sw_mac_table(wx: *mut wx);
}
extern "C" {
    pub fn wx_mta_vector(wx: *mut wx, mc_addr: *mut u8) -> u32;
}
extern "C" {
    pub fn wx_set_mac(netdev: *mut net_device, p: *mut c_void) -> c_int;
}
extern "C" {
    pub fn wx_disable_rx(wx: *mut wx);
}
extern "C" {
    pub fn wx_set_vf_spoofchk(netdev: *mut net_device, vf: c_int, setting: bool) -> c_int;
}
extern "C" {
    pub fn wx_disable_sec_rx_path(wx: *mut wx) -> c_int;
}
extern "C" {
    pub fn wx_enable_sec_rx_path(wx: *mut wx);
}
extern "C" {
    pub fn wx_set_rx_mode(netdev: *mut net_device);
}
extern "C" {
    pub fn wx_change_mtu(netdev: *mut net_device, new_mtu: c_int) -> c_int;
}
extern "C" {
    pub fn wx_disable_rx_queue(wx: *mut wx, ring: *mut wx_ring);
}
extern "C" {
    pub fn wx_enable_rx_queue(wx: *mut wx, ring: *mut wx_ring);
}
extern "C" {
    pub fn wx_rss_indir_tbl_entries(wx: *mut wx) -> u32;
}
extern "C" {
    pub fn wx_store_reta(wx: *mut wx);
}
extern "C" {
    pub fn wx_store_rsskey(wx: *mut wx);
}
extern "C" {
    pub fn wx_config_rss_field(wx: *mut wx);
}
extern "C" {
    pub fn wx_enable_rss(wx: *mut wx, enable: bool);
}
extern "C" {
    pub fn wx_configure_rx(wx: *mut wx);
}
extern "C" {
    pub fn wx_configure(wx: *mut wx);
}
extern "C" {
    pub fn wx_start_hw(wx: *mut wx);
}
extern "C" {
    pub fn wx_disable_pcie_master(wx: *mut wx) -> c_int;
}
extern "C" {
    pub fn wx_stop_adapter(wx: *mut wx) -> c_int;
}
extern "C" {
    pub fn wx_reset_mac(wx: *mut wx);
}
extern "C" {
    pub fn wx_reset_misc(wx: *mut wx);
}
extern "C" {
    pub fn wx_get_pcie_msix_counts(wx: *mut wx, msix_count: *mut u16, max_msix_count: u16) -> c_int;
}
extern "C" {
    pub fn wx_sw_init(wx: *mut wx) -> c_int;
}
extern "C" {
    pub fn wx_set_vfta(wx: *mut wx, vlan: u32, vind: u32, vlan_on: bool) -> c_int;
}
extern "C" {
    pub fn wx_vlan_rx_add_vid(netdev: *mut net_device, proto: __be16, vid: u16) -> c_int;
}
extern "C" {
    pub fn wx_vlan_rx_kill_vid(netdev: *mut net_device, proto: __be16, vid: u16) -> c_int;
}
extern "C" {
    pub fn wx_fc_enable(wx: *mut wx, tx_pause: bool, rx_pause: bool) -> c_int;
}
extern "C" {
    pub fn wx_update_stats(wx: *mut wx);
}
extern "C" {
    pub fn wx_clear_hw_cntrs(wx: *mut wx);
}
