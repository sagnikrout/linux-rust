//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/sfc/nic.h
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
// Driver for Solarflare network controllers and boards
// Copyright 2005-2006 Fen Systems Ltd.
// Copyright 2006-2013 Solarflare Communications Inc.
//

// Maximum number of TX PIO buffers we may allocate to a function.
// This matches the total number of buffers on each SFC9100-family
// controller.
//
pub const EF10_TX_PIOBUF_COUNT: c_int = 16;
//
// struct efx_ef10_nic_data - EF10 architecture NIC state
// @mcdi_buf: DMA buffer for MCDI
// @warm_boot_count: Last seen MC warm boot count
// @vi_base: Absolute index of first VI in this function
// @n_allocated_vis: Number of VIs allocated to this function
// @n_piobufs: Number of PIO buffers allocated to this function
// @wc_membase: Base address of write-combining mapping of the memory BAR
// @pio_write_base: Base address for writing PIO buffers
// @pio_write_vi_base: Relative VI number for @pio_write_base
// @piobuf_handle: Handle of each PIO buffer allocated
// @piobuf_size: size of a single PIO buffer
// @must_restore_piobufs: Flag: PIO buffers have yet to be restored after MC
// reboot
// @mc_stats: Scratch buffer for converting statistics to the kernel's format
// @stats: Hardware statistics
// @workaround_35388: Flag: firmware supports workaround for bug 35388
// @workaround_26807: Flag: firmware supports workaround for bug 26807
// @workaround_61265: Flag: firmware supports workaround for bug 61265
// @must_check_datapath_caps: Flag: @datapath_caps needs to be revalidated
// after MC reboot
// @datapath_caps: Capabilities of datapath firmware (FLAGS1 field of
// %MC_CMD_GET_CAPABILITIES response)
// @datapath_caps2: Further Capabilities of datapath firmware (FLAGS2 field of
// %MC_CMD_GET_CAPABILITIES response)
// @datapath_caps3: Further Capabilities of datapath firmware (FLAGS3 field of
// %MC_CMD_GET_CAPABILITIES response)
// @rx_dpcpu_fw_id: Firmware ID of the RxDPCPU
// @tx_dpcpu_fw_id: Firmware ID of the TxDPCPU
// @must_probe_vswitching: Flag: vswitching has yet to be setup after MC reboot
// @pf_index: The number for this PF, or the parent PF if this is a VF
// @port_id: Ethernet address of owning PF, used for phys_port_id
// @vf_index: The number for this VF, or 0xFFFF if this is a VF
// @vf: for a PF, array of VF data structures indexed by VF's @vf_index
// @vport_mac: The MAC address on the vport, only for PFs; VFs will be zero
// @vlan_list: List of VLANs added over the interface. Serialised by vlan_lock.
// @vlan_lock: Lock to serialize access to vlan_list.
// @udp_tunnels: UDP tunnel port numbers and types.
// @udp_tunnels_dirty: flag indicating a reboot occurred while pushing
// @udp_tunnels to hardware and thus the push must be re-done.
// @udp_tunnels_lock: Serialises writes to @udp_tunnels and @udp_tunnels_dirty.
// @licensed_features: Flags for licensed firmware features.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efx_ef10_nic_data {
    pub mcdi_buf: efx_buffer,
    pub warm_boot_count: u16,
    pub vi_base: c_uint,
    pub n_allocated_vis: c_uint,
    pub n_piobufs: c_uint,
    pub pio_write_base: *mut *mut void __iomem wc_membase,,
    pub pio_write_vi_base: c_uint,
    pub piobuf_handle: [c_uint; EF10_TX_PIOBUF_COUNT],
    pub piobuf_size: u16,
    pub must_restore_piobufs: bool,
    pub mc_stats: *mut __le64,
    pub stats: [u64; EF10_STAT_COUNT],
    pub workaround_35388: bool,
    pub workaround_26807: bool,
    pub workaround_61265: bool,
    pub must_check_datapath_caps: bool,
    pub datapath_caps: u32,
    pub datapath_caps2: u32,
    pub datapath_caps3: u32,
    pub rx_dpcpu_fw_id: c_uint,
    pub tx_dpcpu_fw_id: c_uint,
    pub must_probe_vswitching: bool,
    pub pf_index: c_uint,
    pub port_id: [u8; ETH_ALEN],
    pub vf_index: c_uint,
    pub vf: *mut ef10_vf,
    pub vport_mac: [u8; ETH_ALEN],
    pub vlan_list: list_head,
    pub vlan_lock: mutex,
    pub udp_tunnels: [efx_udp_tunnel; 16],
    pub udp_tunnels_dirty: bool,
    pub udp_tunnels_lock: mutex,
    pub licensed_features: u64,
}

// TSOv2
