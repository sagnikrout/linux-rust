//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/aquantia/atlantic/hw_atl/hw_atl_llh.h
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
// Atlantic Network Driver
//
// Copyright (C) 2014-2019 aQuantia Corporation
// Copyright (C) 2019-2020 Marvell International Ltd.
//
// File hw_atl_llh.h: Declarations of bitfield and register access functions for
// Atlantic registers.
//

// set temperature sense reset
extern "C" {
    pub fn hw_atl_ts_reset_set(aq_hw: *mut aq_hw_s, val: u32);
}
// set temperature sense power down
extern "C" {
    pub fn hw_atl_ts_power_down_set(aq_hw: *mut aq_hw_s, val: u32);
}
// get temperature sense power down
extern "C" {
    pub fn hw_atl_ts_power_down_get(aq_hw: *mut aq_hw_s) -> u32;
}
// get temperature sense ready
extern "C" {
    pub fn hw_atl_ts_ready_get(aq_hw: *mut aq_hw_s) -> u32;
}
// get temperature sense ready latch high
extern "C" {
    pub fn hw_atl_ts_ready_latch_high_get(aq_hw: *mut aq_hw_s) -> u32;
}
// get temperature sense data
extern "C" {
    pub fn hw_atl_ts_data_get(aq_hw: *mut aq_hw_s) -> u32;
}
// SMBUS0 bus busy
extern "C" {
    pub fn hw_atl_smb0_bus_busy_get(aq_hw: *mut aq_hw_s) -> u32;
}
// SMBUS0 byte transfer complete
extern "C" {
    pub fn hw_atl_smb0_byte_transfer_complete_get(aq_hw: *mut aq_hw_s) -> u32;
}
// SMBUS0 receive acknowledged
extern "C" {
    pub fn hw_atl_smb0_receive_acknowledged_get(aq_hw: *mut aq_hw_s) -> u32;
}
// SMBUS0 set transmitted data (only leftmost byte of data valid)
extern "C" {
    pub fn hw_atl_smb0_tx_data_set(aq_hw: *mut aq_hw_s, data: u32);
}
// SMBUS0 provisioning2 command register
extern "C" {
    pub fn hw_atl_smb0_provisioning2_set(aq_hw: *mut aq_hw_s, data: u32);
}
// SMBUS0 repeated start detect
extern "C" {
    pub fn hw_atl_smb0_repeated_start_detect_get(aq_hw: *mut aq_hw_s) -> u32;
}
// SMBUS0 received data register
extern "C" {
    pub fn hw_atl_smb0_rx_data_get(aq_hw: *mut aq_hw_s) -> u32;
}
// global
// set global microprocessor semaphore
// get global microprocessor semaphore
extern "C" {
    pub fn hw_atl_reg_glb_cpu_sem_get(aq_hw: *mut aq_hw_s, semaphore: u32) -> u32;
}
// set global register reset disable
extern "C" {
    pub fn hw_atl_glb_glb_reg_res_dis_set(aq_hw: *mut aq_hw_s, glb_reg_res_dis: u32);
}
// set soft reset
extern "C" {
    pub fn hw_atl_glb_soft_res_set(aq_hw: *mut aq_hw_s, soft_res: u32);
}
// get soft reset
extern "C" {
    pub fn hw_atl_glb_soft_res_get(aq_hw: *mut aq_hw_s) -> u32;
}
// stats
extern "C" {
    pub fn hw_atl_rpb_rx_dma_drop_pkt_cnt_get(aq_hw: *mut aq_hw_s) -> u32;
}
// get rx dma good octet counter
extern "C" {
    pub fn hw_atl_stats_rx_dma_good_octet_counter_get(aq_hw: *mut aq_hw_s) -> u64;
}
// get rx dma good packet counter
extern "C" {
    pub fn hw_atl_stats_rx_dma_good_pkt_counter_get(aq_hw: *mut aq_hw_s) -> u64;
}
// get tx dma good octet counter
extern "C" {
    pub fn hw_atl_stats_tx_dma_good_octet_counter_get(aq_hw: *mut aq_hw_s) -> u64;
}
// get tx dma good packet counter
extern "C" {
    pub fn hw_atl_stats_tx_dma_good_pkt_counter_get(aq_hw: *mut aq_hw_s) -> u64;
}
// get msm rx errors counter register
extern "C" {
    pub fn hw_atl_reg_mac_msm_rx_errs_cnt_get(aq_hw: *mut aq_hw_s) -> u32;
}
// get msm rx unicast frames counter register
extern "C" {
    pub fn hw_atl_reg_mac_msm_rx_ucst_frm_cnt_get(aq_hw: *mut aq_hw_s) -> u32;
}
// get msm rx multicast frames counter register
extern "C" {
    pub fn hw_atl_reg_mac_msm_rx_mcst_frm_cnt_get(aq_hw: *mut aq_hw_s) -> u32;
}
// get msm rx broadcast frames counter register
extern "C" {
    pub fn hw_atl_reg_mac_msm_rx_bcst_frm_cnt_get(aq_hw: *mut aq_hw_s) -> u32;
}
// get msm rx broadcast octets counter register 1
extern "C" {
    pub fn hw_atl_reg_mac_msm_rx_bcst_octets_counter1get(aq_hw: *mut aq_hw_s) -> u32;
}
// get msm rx unicast octets counter register 0
extern "C" {
    pub fn hw_atl_reg_mac_msm_rx_ucst_octets_counter0get(aq_hw: *mut aq_hw_s) -> u32;
}
// get msm tx errors counter register
extern "C" {
    pub fn hw_atl_reg_mac_msm_tx_errs_cnt_get(aq_hw: *mut aq_hw_s) -> u32;
}
// get msm tx unicast frames counter register
extern "C" {
    pub fn hw_atl_reg_mac_msm_tx_ucst_frm_cnt_get(aq_hw: *mut aq_hw_s) -> u32;
}
// get msm tx multicast frames counter register
extern "C" {
    pub fn hw_atl_reg_mac_msm_tx_mcst_frm_cnt_get(aq_hw: *mut aq_hw_s) -> u32;
}
// get msm tx broadcast frames counter register
extern "C" {
    pub fn hw_atl_reg_mac_msm_tx_bcst_frm_cnt_get(aq_hw: *mut aq_hw_s) -> u32;
}
// get msm tx multicast octets counter register 1
extern "C" {
    pub fn hw_atl_reg_mac_msm_tx_mcst_octets_counter1get(aq_hw: *mut aq_hw_s) -> u32;
}
// get msm tx broadcast octets counter register 1
extern "C" {
    pub fn hw_atl_reg_mac_msm_tx_bcst_octets_counter1get(aq_hw: *mut aq_hw_s) -> u32;
}
// get msm tx unicast octets counter register 0
extern "C" {
    pub fn hw_atl_reg_mac_msm_tx_ucst_octets_counter0get(aq_hw: *mut aq_hw_s) -> u32;
}
// get global mif identification
extern "C" {
    pub fn hw_atl_reg_glb_mif_id_get(aq_hw: *mut aq_hw_s) -> u32;
}
// interrupt
// set interrupt auto mask lsw
// set interrupt mapping enable rx
// set interrupt mapping enable tx
// set interrupt mapping rx
extern "C" {
    pub fn hw_atl_itr_irq_map_rx_set(aq_hw: *mut aq_hw_s, irq_map_rx: u32, rx: u32);
}
// set interrupt mapping tx
extern "C" {
    pub fn hw_atl_itr_irq_map_tx_set(aq_hw: *mut aq_hw_s, irq_map_tx: u32, tx: u32);
}
// set interrupt mask clear lsw
// set interrupt mask set lsw
extern "C" {
    pub fn hw_atl_itr_irq_msk_setlsw_set(aq_hw: *mut aq_hw_s, irq_msk_setlsw: u32);
}
// set interrupt register reset disable
extern "C" {
    pub fn hw_atl_itr_irq_reg_res_dis_set(aq_hw: *mut aq_hw_s, irq_reg_res_dis: u32);
}
// set interrupt status clear lsw
// get interrupt status lsw
extern "C" {
    pub fn hw_atl_itr_irq_statuslsw_get(aq_hw: *mut aq_hw_s) -> u32;
}
// get reset interrupt
extern "C" {
    pub fn hw_atl_itr_res_irq_get(aq_hw: *mut aq_hw_s) -> u32;
}
// set reset interrupt
extern "C" {
    pub fn hw_atl_itr_res_irq_set(aq_hw: *mut aq_hw_s, res_irq: u32);
}
// set RSC interrupt
extern "C" {
    pub fn hw_atl_itr_rsc_en_set(aq_hw: *mut aq_hw_s, enable: u32);
}
// set RSC delay
extern "C" {
    pub fn hw_atl_itr_rsc_delay_set(aq_hw: *mut aq_hw_s, delay: u32);
}
// rdm
// set cpu id
extern "C" {
    pub fn hw_atl_rdm_cpu_id_set(aq_hw: *mut aq_hw_s, cpuid: u32, dca: u32);
}
// set rx dca enable
extern "C" {
    pub fn hw_atl_rdm_rx_dca_en_set(aq_hw: *mut aq_hw_s, rx_dca_en: u32);
}
// set rx dca mode
extern "C" {
    pub fn hw_atl_rdm_rx_dca_mode_set(aq_hw: *mut aq_hw_s, rx_dca_mode: u32);
}
// set rx descriptor data buffer size
// set rx descriptor dca enable
// set rx descriptor enable
// set rx descriptor header splitting
// get rx descriptor head pointer
extern "C" {
    pub fn hw_atl_rdm_rx_desc_head_ptr_get(aq_hw: *mut aq_hw_s, descriptor: u32) -> u32;
}
// set rx descriptor length
// set rx descriptor write-back interrupt enable
// set rx header dca enable
// set rx payload dca enable
// set rx descriptor header buffer size
// set rx descriptor reset
// Set RDM Interrupt Moderation Enable
// reg
// set general interrupt mapping register
// get general interrupt status register
extern "C" {
    pub fn hw_atl_reg_gen_irq_status_get(aq_hw: *mut aq_hw_s) -> u32;
}
// set interrupt global control register
extern "C" {
    pub fn hw_atl_reg_irq_glb_ctl_set(aq_hw: *mut aq_hw_s, intr_glb_ctl: u32);
}
// set interrupt throttle register
extern "C" {
    pub fn hw_atl_reg_irq_thr_set(aq_hw: *mut aq_hw_s, intr_thr: u32, throttle: u32);
}
// set rx dma descriptor base address lsw
// set rx dma descriptor base address msw
// get rx dma descriptor status register
extern "C" {
    pub fn hw_atl_reg_rx_dma_desc_status_get(aq_hw: *mut aq_hw_s, descriptor: u32) -> u32;
}
// set rx dma descriptor tail pointer register
// set rx filter multicast filter mask register
// set rx filter multicast filter register
// set rx filter rss control register 1
// Set RX Filter Control Register 2
extern "C" {
    pub fn hw_atl_reg_rx_flr_control2_set(aq_hw: *mut aq_hw_s, rx_flr_control2: u32);
}
// Set RX Interrupt Moderation Control Register
// set tx dma debug control
// set tx dma descriptor base address lsw
// set tx dma descriptor base address msw
// set tx dma descriptor tail pointer register
// Set TX Interrupt Moderation Control Register
// set global microprocessor scratch pad
// rpb
// set dma system loopback
extern "C" {
    pub fn hw_atl_rpb_dma_sys_lbk_set(aq_hw: *mut aq_hw_s, dma_sys_lbk: u32);
}
// set dma network loopback
extern "C" {
    pub fn hw_atl_rpb_dma_net_lbk_set(aq_hw: *mut aq_hw_s, dma_net_lbk: u32);
}
// set rx traffic class mode
// get rx traffic class mode
extern "C" {
    pub fn hw_atl_rpb_rpf_rx_traf_class_mode_get(aq_hw: *mut aq_hw_s) -> u32;
}
// set rx buffer enable
extern "C" {
    pub fn hw_atl_rpb_rx_buff_en_set(aq_hw: *mut aq_hw_s, rx_buff_en: u32);
}
// set rx buffer high threshold (per tc)
// set rx buffer low threshold (per tc)
// set rx flow control mode
// set rx packet buffer size (per tc)
// toggle rdm rx dma descriptor cache init
extern "C" {
    pub fn hw_atl_rdm_rx_dma_desc_cache_init_tgl(aq_hw: *mut aq_hw_s);
}
// get rdm rx dma descriptor cache init done
extern "C" {
    pub fn hw_atl_rdm_rx_dma_desc_cache_init_done_get(aq_hw: *mut aq_hw_s) -> u32;
}
// set rx xoff enable (per tc)
// rpf
// set l2 broadcast count threshold
// set l2 broadcast enable
extern "C" {
    pub fn hw_atl_rpfl2broadcast_en_set(aq_hw: *mut aq_hw_s, l2broadcast_en: u32);
}
// set l2 broadcast filter action
// set l2 multicast filter enable
// get l2 promiscuous mode enable
extern "C" {
    pub fn hw_atl_rpfl2promiscuous_mode_en_get(aq_hw: *mut aq_hw_s) -> u32;
}
// set l2 promiscuous mode enable
// set l2 unicast filter action
// set l2 unicast filter enable
// set l2 unicast destination address lsw
// set l2 unicast destination address msw
// Set L2 Accept all Multicast packets
// set user-priority tc mapping
// set rss key address
extern "C" {
    pub fn hw_atl_rpf_rss_key_addr_set(aq_hw: *mut aq_hw_s, rss_key_addr: u32);
}
// set rss key write data
extern "C" {
    pub fn hw_atl_rpf_rss_key_wr_data_set(aq_hw: *mut aq_hw_s, rss_key_wr_data: u32);
}
// get rss key write enable
extern "C" {
    pub fn hw_atl_rpf_rss_key_wr_en_get(aq_hw: *mut aq_hw_s) -> u32;
}
// set rss key write enable
extern "C" {
    pub fn hw_atl_rpf_rss_key_wr_en_set(aq_hw: *mut aq_hw_s, rss_key_wr_en: u32);
}
// set rss redirection table address
// set rss redirection table write data
// get rss redirection write enable
extern "C" {
    pub fn hw_atl_rpf_rss_redir_wr_en_get(aq_hw: *mut aq_hw_s) -> u32;
}
// set rss redirection write enable
extern "C" {
    pub fn hw_atl_rpf_rss_redir_wr_en_set(aq_hw: *mut aq_hw_s, rss_redir_wr_en: u32);
}
// set tpo to rpf system loopback
// set vlan inner ethertype
extern "C" {
    pub fn hw_atl_rpf_vlan_inner_etht_set(aq_hw: *mut aq_hw_s, vlan_inner_etht: u32);
}
// set vlan outer ethertype
extern "C" {
    pub fn hw_atl_rpf_vlan_outer_etht_set(aq_hw: *mut aq_hw_s, vlan_outer_etht: u32);
}
// set vlan promiscuous mode enable
// Get VLAN promiscuous mode enable
extern "C" {
    pub fn hw_atl_rpf_vlan_prom_mode_en_get(aq_hw: *mut aq_hw_s) -> u32;
}
// Set VLAN untagged action
// Set VLAN accept untagged packets
// Set VLAN filter enable
// Set VLAN Filter Action
// Set VLAN ID Filter
// Set VLAN RX queue assignment enable
// Set VLAN RX queue
// set ethertype filter enable
// set  ethertype user-priority enable
// set  ethertype rx queue enable
// set ethertype rx queue
// set ethertype user-priority
// set ethertype management queue
// set ethertype filter action
// set ethertype filter
extern "C" {
    pub fn hw_atl_rpf_etht_flr_set(aq_hw: *mut aq_hw_s, etht_flr: u32, filter: u32);
}
// set L4 source port
extern "C" {
    pub fn hw_atl_rpf_l4_spd_set(aq_hw: *mut aq_hw_s, val: u32, filter: u32);
}
// set L4 destination port
extern "C" {
    pub fn hw_atl_rpf_l4_dpd_set(aq_hw: *mut aq_hw_s, val: u32, filter: u32);
}
// rpo
// set ipv4 header checksum offload enable
// set rx descriptor vlan stripping
extern "C" {
    pub fn hw_atl_rpo_outer_vlan_tag_mode_get(context: *mut c_void) -> u32;
}
// set tcp/udp checksum offload enable
// Set LRO Patch Optimization Enable.
// Set Large Receive Offload Enable
extern "C" {
    pub fn hw_atl_rpo_lro_en_set(aq_hw: *mut aq_hw_s, lro_en: u32);
}
// Set LRO Q Sessions Limit
// Set LRO Total Descriptor Limit
// Set LRO Min Payload of First Packet
// Set LRO Packet Limit
extern "C" {
    pub fn hw_atl_rpo_lro_pkt_lim_set(aq_hw: *mut aq_hw_s, lro_packet_lim: u32);
}
// Set LRO Max Number of Descriptors
// Set LRO Time Base Divider
// Set LRO Inactive Interval
// Set LRO Max Coalescing Interval
// rx
// set rx register reset disable
extern "C" {
    pub fn hw_atl_rx_rx_reg_res_dis_set(aq_hw: *mut aq_hw_s, rx_reg_res_dis: u32);
}
// tdm
// set cpu id
extern "C" {
    pub fn hw_atl_tdm_cpu_id_set(aq_hw: *mut aq_hw_s, cpuid: u32, dca: u32);
}
// set large send offload enable
// set tx descriptor enable
// set tx dca enable
extern "C" {
    pub fn hw_atl_tdm_tx_dca_en_set(aq_hw: *mut aq_hw_s, tx_dca_en: u32);
}
// set tx dca mode
extern "C" {
    pub fn hw_atl_tdm_tx_dca_mode_set(aq_hw: *mut aq_hw_s, tx_dca_mode: u32);
}
// set tx descriptor dca enable
// get tx descriptor head pointer
extern "C" {
    pub fn hw_atl_tdm_tx_desc_head_ptr_get(aq_hw: *mut aq_hw_s, descriptor: u32) -> u32;
}
// set tx descriptor length
// set tx descriptor write-back interrupt enable
// set tx descriptor write-back threshold
// Set TDM Interrupt Moderation Enable
// thm
// set lso tcp flag of first packet
// set lso tcp flag of last packet
// set lso tcp flag of middle packet
// tpb
// set TX Traffic Class Mode
// get TX Traffic Class Mode
extern "C" {
    pub fn hw_atl_tpb_tps_tx_tc_mode_get(aq_hw: *mut aq_hw_s) -> u32;
}
// set tx buffer enable
extern "C" {
    pub fn hw_atl_tpb_tx_buff_en_set(aq_hw: *mut aq_hw_s, tx_buff_en: u32);
}
// set tx buffer high threshold (per tc)
// set tx buffer low threshold (per tc)
// set tx dma system loopback enable
extern "C" {
    pub fn hw_atl_tpb_tx_dma_sys_lbk_en_set(aq_hw: *mut aq_hw_s, tx_dma_sys_lbk_en: u32);
}
// set tx dma network loopback enable
// set tx clock gating enable
// set tx packet buffer size (per tc)
// set tx path pad insert enable
extern "C" {
    pub fn hw_atl_tpb_tx_path_scp_ins_en_set(aq_hw: *mut aq_hw_s, tx_path_scp_ins_en: u32);
}
// tpo
// set ipv4 header checksum offload enable
// set tcp/udp checksum offload enable
// set tx pkt system loopback enable
// tps
// set tx packet scheduler data arbitration mode
// set tx packet scheduler descriptor rate current time reset
// set tx packet scheduler descriptor rate limit
// set tx packet scheduler descriptor tc arbitration mode
// set tx packet scheduler descriptor tc max credit
// set tx packet scheduler descriptor tc weight
// set tx packet scheduler descriptor vm arbitration mode
// set tx packet scheduler tc data max credit
// set tx packet scheduler tc data weight
// set tx descriptor rate mode
// set tx packet scheduler descriptor rate enable
// set tx packet scheduler descriptor rate integral value
// set tx packet scheduler descriptor rate fractional value
// tx
// set tx register reset disable
extern "C" {
    pub fn hw_atl_tx_tx_reg_res_dis_set(aq_hw: *mut aq_hw_s, tx_reg_res_dis: u32);
}
// msm
// get register access status
extern "C" {
    pub fn hw_atl_msm_reg_access_status_get(aq_hw: *mut aq_hw_s) -> u32;
}
// set  register address for indirect address
// set register read strobe
extern "C" {
    pub fn hw_atl_msm_reg_rd_strobe_set(aq_hw: *mut aq_hw_s, reg_rd_strobe: u32);
}
// get  register read data
extern "C" {
    pub fn hw_atl_msm_reg_rd_data_get(aq_hw: *mut aq_hw_s) -> u32;
}
// set  register write data
extern "C" {
    pub fn hw_atl_msm_reg_wr_data_set(aq_hw: *mut aq_hw_s, reg_wr_data: u32);
}
// set register write strobe
extern "C" {
    pub fn hw_atl_msm_reg_wr_strobe_set(aq_hw: *mut aq_hw_s, reg_wr_strobe: u32);
}
// pci
// set pci register reset disable
extern "C" {
    pub fn hw_atl_pci_pci_reg_res_dis_set(aq_hw: *mut aq_hw_s, pci_reg_res_dis: u32);
}
// pcs
extern "C" {
    pub fn hw_atl_pcs_ptp_clock_get(aq_hw: *mut aq_hw_s, index: u32) -> u32;
}
// set uP Force Interrupt
extern "C" {
    pub fn hw_atl_mcp_up_force_intr_set(aq_hw: *mut aq_hw_s, up_force_intr: u32);
}
// clear ipv4 filter destination address
extern "C" {
    pub fn hw_atl_rpfl3l4_ipv4_dest_addr_clear(aq_hw: *mut aq_hw_s, location: u8);
}
// clear ipv4 filter source address
extern "C" {
    pub fn hw_atl_rpfl3l4_ipv4_src_addr_clear(aq_hw: *mut aq_hw_s, location: u8);
}
// clear command for filter l3-l4
extern "C" {
    pub fn hw_atl_rpfl3l4_cmd_clear(aq_hw: *mut aq_hw_s, location: u8);
}
// clear ipv6 filter destination address
extern "C" {
    pub fn hw_atl_rpfl3l4_ipv6_dest_addr_clear(aq_hw: *mut aq_hw_s, location: u8);
}
// clear ipv6 filter source address
extern "C" {
    pub fn hw_atl_rpfl3l4_ipv6_src_addr_clear(aq_hw: *mut aq_hw_s, location: u8);
}
// set ipv4 filter destination address
// set ipv4 filter source address
// set command for filter l3-l4
extern "C" {
    pub fn hw_atl_rpfl3l4_cmd_set(aq_hw: *mut aq_hw_s, location: u8, cmd: u32);
}
// set ipv6 filter source address
// set ipv6 filter destination address
// set Global MDIO Interface 1
extern "C" {
    pub fn hw_atl_glb_mdio_iface1_set(hw: *mut aq_hw_s, value: u32);
}
// get Global MDIO Interface 1
extern "C" {
    pub fn hw_atl_glb_mdio_iface1_get(hw: *mut aq_hw_s) -> u32;
}
// set Global MDIO Interface 2
extern "C" {
    pub fn hw_atl_glb_mdio_iface2_set(hw: *mut aq_hw_s, value: u32);
}
// get Global MDIO Interface 2
extern "C" {
    pub fn hw_atl_glb_mdio_iface2_get(hw: *mut aq_hw_s) -> u32;
}
// set Global MDIO Interface 3
extern "C" {
    pub fn hw_atl_glb_mdio_iface3_set(hw: *mut aq_hw_s, value: u32);
}
// get Global MDIO Interface 3
extern "C" {
    pub fn hw_atl_glb_mdio_iface3_get(hw: *mut aq_hw_s) -> u32;
}
// set Global MDIO Interface 4
extern "C" {
    pub fn hw_atl_glb_mdio_iface4_set(hw: *mut aq_hw_s, value: u32);
}
// get Global MDIO Interface 4
extern "C" {
    pub fn hw_atl_glb_mdio_iface4_get(hw: *mut aq_hw_s) -> u32;
}
// set Global MDIO Interface 5
extern "C" {
    pub fn hw_atl_glb_mdio_iface5_set(hw: *mut aq_hw_s, value: u32);
}
// get Global MDIO Interface 5
extern "C" {
    pub fn hw_atl_glb_mdio_iface5_get(hw: *mut aq_hw_s) -> u32;
}
extern "C" {
    pub fn hw_atl_mdio_busy_get(aq_hw: *mut aq_hw_s) -> u32;
}
// get global microprocessor ram semaphore
extern "C" {
    pub fn hw_atl_sem_ram_get(self: *mut aq_hw_s) -> u32;
}
// get global microprocessor mdio semaphore
extern "C" {
    pub fn hw_atl_sem_mdio_get(self: *mut aq_hw_s) -> u32;
}
extern "C" {
    pub fn hw_atl_sem_reset1_get(self: *mut aq_hw_s) -> u32;
}
extern "C" {
    pub fn hw_atl_sem_reset2_get(self: *mut aq_hw_s) -> u32;
}
// get global microprocessor scratch pad register
extern "C" {
    pub fn hw_atl_scrpad_get(aq_hw: *mut aq_hw_s, scratch_scp: u32) -> u32;
}
// get global microprocessor scratch pad 12 register
extern "C" {
    pub fn hw_atl_scrpad12_get(self: *mut aq_hw_s) -> u32;
}
// get global microprocessor scratch pad 25 register
extern "C" {
    pub fn hw_atl_scrpad25_get(self: *mut aq_hw_s) -> u32;
}
