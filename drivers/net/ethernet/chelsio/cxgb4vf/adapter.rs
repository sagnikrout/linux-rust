//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/chelsio/cxgb4vf/adapter.h
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
// This file is part of the Chelsio T4 PCI-E SR-IOV Virtual Function Ethernet
// driver for Linux.
//
// Copyright (c) 2009-2010 Chelsio Communications, Inc. All rights reserved.
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
// This file should not be included directly.  Include t4vf_common.h instead.
//

//
// Constants of the implementation.
//
// MSI-X interrupt index usage.
//
// The maximum number of Ingress and Egress Queues is determined by
// the maximum number of "Queue Sets" which we support plus any
// ancillary queues.  Each "Queue Set" requires one Ingress Queue
// for RX Packet Ingress Event notifications and two Egress Queues for
// a Free List and an Ethernet TX list.
//
// forwarded interrupts
//
// Forward structure definition references.
//
// Per-"port" information.  This is really per-Virtual Interface information
// but the use of the "port" nomanclature makes it easier to go back and forth
// between the PF and VF drivers ...
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct port_info {
    pub /: *mut *mut *mut adapter adapter; / our adapter,
    pub /: *mut *mut u32 vlan_id; / vlan id for VST,
    pub /: *mut *mut u16 viid; / virtual interface ID,
    pub /: *mut *mut int xact_addr_filt; / index of our MAC address filter,
    pub /: *mut *mut u16 rss_size; / size of VI's RSS table slice,
    pub /: *mut *mut u8 pidx; / index into adapter port[],
    pub mdio_addr: i8,
    pub /: *mut *mut u8 port_type; / firmware port type,
    pub /: *mut *mut u8 mod_type; / firmware module type,
    pub /: *mut *mut u8 port_id; / physical port ID,
    pub /: *mut *mut u8 nqsets; / # of "Queue Sets",
    pub /: *mut *mut u8 first_qset; / index of first "Queue Set",
    pub /: *mut *mut link_config link_cfg; / physical port configuration,
}

//
// Scatter Gather Engine resources for the "adapter".  Our ingress and egress
// queues are organized into "Queue Sets" with one ingress and one egress
// queue per Queue Set.  These Queue Sets are aportionable between the "ports"
// (Virtual Interfaces).  One extra ingress queue is used to receive
// asynchronous messages from the firmware.  Note that the "Queue IDs" that we
// use here are really "Relative Queue IDs" which are returned as part of the
// firmware command to allocate queues.  These queue IDs are relative to the
// absolute Queue ID base of the section of the Queue ID space allocated to
// the PF/VF.
//
// SGE free-list queue state.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sge_fl {
    pub /: *mut *mut unsigned int avail; / # of available RX buffers,
    pub /: *mut *mut unsigned int pend_cred; / new buffers since last FL DB ring,
    pub /: *mut *mut unsigned int cidx; / consumer index,
    pub /: *mut *mut unsigned int pidx; / producer index,
    pub /: *mut *mut unsigned long alloc_failed; / # of buffer allocation failures,
    pub large_alloc_failed: c_ulong,
    pub /: *mut *mut unsigned long starving; / # of times FL was found starving,
//
// Write-once/infrequently fields.
// -------------------------------
//
    pub /: *mut *mut unsigned int cntxt_id; / SGE relative QID for the free list,
    pub /: *mut *mut unsigned int abs_id; / SGE absolute QID for the free list,
    pub /: *mut *mut unsigned int size; / capacity of free list,
    pub /: *mut *mut *mut rx_sw_desc sdesc; / address of SW RX descriptor ring,
    pub /: *mut *mut *mut __be64 desc; / address of HW RX descriptor ring,
    pub /: *mut *mut dma_addr_t addr; / PCI bus address of hardware ring,
    pub /: *mut *mut *mut void __iomem bar2_addr; / address of BAR2 Queue registers,
    pub /: *mut *mut unsigned int bar2_qid; / Queue ID for BAR2 Queue registers,
}

//
// An ingress packet gather list.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pkt_gl {
    pub frags: [page_frag; MAX_SKB_FRAGS],
    pub /: *mut *mut *mut void va; / virtual address of first byte,
    pub /: *mut *mut unsigned int nfrags; / # of fragments,
    pub /: *mut *mut unsigned int tot_len; / total length of fragments,
}

//
// State for an SGE Response Queue.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sge_rspq {
    pub /: *mut *mut napi_napi; / NAPI scheduling control,
    pub /: *const *const *const __be64 cur_desc; / current descriptor in queue,
    pub /: *mut *mut unsigned int cidx; / consumer index,
    pub /: *mut *mut u8 gen; / current generation bit,
    pub /: *mut *mut u8 next_intr_params; / holdoff params for next interrupt,
    pub /: *mut *mut int offset; / offset into current FL buffer,
    pub /: *mut *mut unsigned int unhandled_irqs; / bogus interrupts,
//
// Write-once/infrequently fields.
// -------------------------------
//
    pub /: *mut *mut u8 intr_params; / interrupt holdoff parameters,
    pub /: *mut *mut u8 pktcnt_idx; / interrupt packet threshold,
    pub /: *mut *mut u8 idx; / queue index within its group,
    pub /: *mut *mut u16 cntxt_id; / SGE rel QID for the response Q,
    pub /: *mut *mut u16 abs_id; / SGE abs QID for the response Q,
    pub /: *mut *mut *mut __be64 desc; / address of hardware response ring,
    pub /: *mut *mut dma_addr_t phys_addr; / PCI bus address of ring,
    pub /: *mut *mut *mut void __iomem bar2_addr; / address of BAR2 Queue registers,
    pub /: *mut *mut unsigned int bar2_qid; / Queue ID for BAR2 Queue registers,
    pub /: *mut *mut unsigned int iqe_len; / entry size,
    pub /: *mut *mut unsigned int size; / capcity of response Q,
    pub /: *mut *mut *mut adapter adapter; / our adapter,
    pub /: *mut *mut *mut net_device netdev; / associated net device,
    pub /: *mut *mut rspq_handler_t handler; / the handler for this response Q,
}

//
// Ethernet queue statistics
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sge_eth_stats {
    pub /: *mut *mut unsigned long pkts; / # of ethernet packets,
    pub /: *mut *mut unsigned long lro_pkts; / # of LRO super packets,
    pub /: *mut *mut unsigned long lro_merged; / # of wire packets merged by LRO,
    pub /: *mut *mut unsigned long rx_cso; / # of Rx checksum offloads,
    pub /: *mut *mut unsigned long vlan_ex; / # of Rx VLAN extractions,
    pub /: *mut *mut unsigned long rx_drops; / # of packets dropped due to no mem,
}

//
// State for an Ethernet Receive Queue.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sge_eth_rxq {
    pub /: *mut *mut sge_rspq rspq; / Response Queue,
    pub /: *mut *mut sge_fl fl; / Free List,
    pub /: *mut *mut sge_eth_stats stats; / receive statistics,
}

//
// SGE Transmit Queue state.  This contains all of the resources associated
// with the hardware status of a TX Queue which is a circular ring of hardware
// TX Descriptors.  For convenience, it also contains a pointer to a parallel
// "Software Descriptor" array but we don't know anything about it here other
// than its type name.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tx_desc {
//
// Egress Queues are measured in units of SGE_EQ_IDXSIZE by the
// hardware: Sizes, Producer and Consumer indices, etc.
//
    pub flit: [__be64; SGE_EQ_IDXSIZE/sizeof(__be64)],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sge_txq {
    pub /: *mut *mut unsigned int in_use; / # of in-use TX descriptors,
    pub /: *mut *mut unsigned int size; / # of descriptors,
    pub /: *mut *mut unsigned int cidx; / SW consumer index,
    pub /: *mut *mut unsigned int pidx; / producer index,
    pub /: *mut *mut unsigned long stops; / # of times queue has been stopped,
    pub /: *mut *mut unsigned long restarts; / # of queue restarts,
//
// Write-once/infrequently fields.
// -------------------------------
//
    pub /: *mut *mut unsigned int cntxt_id; / SGE relative QID for the TX Q,
    pub /: *mut *mut unsigned int abs_id; / SGE absolute QID for the TX Q,
    pub /: *mut *mut *mut tx_desc desc; / address of HW TX descriptor ring,
    pub /: *mut *mut *mut tx_sw_desc sdesc; / address of SW TX descriptor ring,
    pub /: *mut *mut *mut sge_qstat stat; / queue status entry,
    pub /: *mut *mut dma_addr_t phys_addr; / PCI bus address of hardware ring,
    pub /: *mut *mut *mut void __iomem bar2_addr; / address of BAR2 Queue registers,
    pub /: *mut *mut unsigned int bar2_qid; / Queue ID for BAR2 Queue registers,
}

//
// State for an Ethernet Transmit Queue.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sge_eth_txq {
    pub /: *mut *mut sge_txq q; / SGE TX Queue,
    pub /: *mut *mut *mut netdev_queue txq; / associated netdev TX queue,
    pub /: *mut *mut unsigned long tso; / # of TSO requests,
    pub /: *mut *mut unsigned long tx_cso; / # of TX checksum offloads,
    pub /: *mut *mut unsigned long vlan_ins; / # of TX VLAN insertions,
    pub /: *mut *mut unsigned long mapping_err; / # of I/O MMU packet mapping errors,
}

//
// The complete set of Scatter/Gather Engine resources.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sge {
//
// Our "Queue Sets" ...
//
    pub ethtxq: [sge_eth_txq; MAX_ETH_QSETS],
    pub ethrxq: [sge_eth_rxq; MAX_ETH_QSETS],
//
// Extra ingress queues for asynchronous firmware events and
// forwarded interrupts (when in MSI mode).
//
    pub ____cacheline_aligned_in_smp: sge_rspq fw_evtq,
    pub ____cacheline_aligned_in_smp: sge_rspq intrq,
    pub intrq_lock: spinlock_t,
//
// State for managing "starving Free Lists" -- Free Lists which have
// fallen below a certain threshold of buffers available to the
// hardware and attempts to refill them up to that threshold have
// failed.  We have a regular "slow tick" timer process which will
// make periodic attempts to refill these starving Free Lists ...
//
    pub MAX_EGRQ): DECLARE_BITMAP(starving_fl,,
    pub rx_timer: timer_list,
//
// State for cleaning up completed TX descriptors.
//
    pub tx_timer: timer_list,
//
// Write-once/infrequently fields.
// -------------------------------
//
    pub /: *mut *mut u16 max_ethqsets; / # of available Ethernet queue sets,
    pub /: *mut *mut u16 ethqsets; / # of active Ethernet queue sets,
    pub /: *mut *mut u16 ethtxq_rover; / Tx queue to clean up next,
    pub /: *mut *mut u16 timer_val[SGE_NTIMERS]; / interrupt holdoff timer array,
    pub /: *mut *mut u8 counter_val[SGE_NCOUNTERS]; / interrupt RX threshold array,
// Decoded Adapter Parameters.
//
    pub /: *mut *mut u32 fl_pg_order; / large page allocation size,
    pub /: *mut *mut u32 stat_len; / length of status page at ring end,
    pub /: *mut *mut u32 pktshift; / padding between CPL & packet data,
    pub /: *mut *mut u32 fl_align; / response queue message alignment,
    pub /: *mut *mut u32 fl_starve_thres; / Free List starvation threshold,
//
// Reverse maps from Absolute Queue IDs to associated queue pointers.
// The absolute Queue IDs are in a compact range which start at a
// [potentially large] Base Queue ID.  We perform the reverse map by
// first converting the Absolute Queue ID into a Relative Queue ID by
// subtracting off the Base Queue ID and then use a Relative Queue ID
// indexed table to get the pointer to the corresponding software
// queue structure.
//
    pub egr_base: c_uint,
    pub ingr_base: c_uint,
    pub egr_map: [*mut c_void; MAX_EGRQ],
    pub ingr_map: [*mut sge_rspq; MAX_INGQ],
}

//
// Utility macros to convert Absolute- to Relative-Queue indices and Egress-
// and Ingress-Queues.  The EQ_MAP() and IQ_MAP() macros which provide
// pointers to Ingress- and Egress-Queues can be used as both L- and R-values
//

//
// Macro to iterate across Queue Sets ("rxq" is a historic misnomer).
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hash_mac_addr {
    pub list: list_head,
    pub addr: [u8; ETH_ALEN],
    pub iface_mac: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mbox_list {
    pub list: list_head,
}

//
// Per-"adapter" (Virtual Function) information.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct adapter {
// PCI resources
    pub regs: *mut void __iomem,
    pub bar2: *mut void __iomem,
    pub pdev: *mut pci_dev,
    pub pdev_dev: *mut device,
// "adapter" resources
    pub registered_device_map: c_ulong,
    pub open_device_map: c_ulong,
    pub flags: c_ulong,
    pub params: adapter_params,
// queue and interrupt resources
    pub vec: c_ushort,
    pub desc: [c_char; 22],
    pub msix_info: [}; MSIX_ENTRIES],
    pub sge: sge,
// Linux network device resources
    pub port: [*mut net_device; MAX_NPORTS],
    pub name: *const c_char,
    pub msg_enable: c_uint,
// debugfs resources
    pub debugfs_root: *mut dentry,
// various locks
    pub stats_lock: spinlock_t,
// lock for mailbox cmd list
    pub mbox_lock: spinlock_t,
    pub mlist: mbox_list,
// support for mailbox command/reply logging
pub const T4VF_OS_LOG_MBOX_CMDS: c_int = 256;
    pub mbox_log: *mut mbox_cmd_log,
// list of MAC addresses in MPS Hash
    pub mac_hlist: list_head,
}

//
// The following register read/write routine definitions are required by
// the common code.
//
// t4_read_reg - read a HW register
// @adapter: the adapter
// @reg_addr: the register address
//
// Returns the 32-bit value of the given HW register.
//
extern "C" {
    pub fn readl(reg_addr: adapter->regs +) -> return;
}
//
// t4_write_reg - write a HW register
// @adapter: the adapter
// @reg_addr: the register address
// @val: the value to write
//
// Write a 32-bit value into the given HW register.
//

extern "C" {
    pub fn readl(32: addr) + ((u64)readl(addr + 4) <<) -> return;
}

//
// t4_read_reg64 - read a 64-bit HW register
// @adapter: the adapter
// @reg_addr: the register address
//
// Returns the 64-bit value of the given HW register.
//
extern "C" {
    pub fn readq(reg_addr: adapter->regs +) -> return;
}
//
// t4_write_reg64 - write a 64-bit HW register
// @adapter: the adapter
// @reg_addr: the register address
// @val: the value to write
//
// Write a 64-bit value into the given HW register.
//
// port_name - return the string name of a port
// @adapter: the adapter
// @pidx: the port index
//
// Return the string name of the selected port.
//
// t4_os_set_hw_addr - store a port's MAC address in SW
// @adapter: the adapter
// @pidx: the port index
// @hw_addr: the Ethernet address
//
// Store the Ethernet address of the given port in SW.  Called by the common
// code when it retrieves a port's Ethernet address from EEPROM.
//
// netdev2pinfo - return the port_info structure associated with a net_device
// @dev: the netdev
//
// Return the struct port_info associated with a net_device
//
extern "C" {
    pub fn netdev_priv(_arg: dev) -> return;
}
//
// adap2pinfo - return the port_info of a port
// @adap: the adapter
// @pidx: the port index
//
// Return the port_info structure for the adapter.
//
extern "C" {
    pub fn netdev_priv(_arg: adapter->port[pidx]) -> return;
}
//
// netdev2adap - return the adapter structure associated with a net_device
// @dev: the netdev
//
// Return the struct adapter associated with a net_device
//
// OS "Callback" function declarations.  These are functions that the OS code
// is "contracted" to provide for the common code.
//
extern "C" {
    pub fn t4vf_os_link_changed(: *mut adapter, _arg: c_int, _arg: c_int);
}
extern "C" {
    pub fn t4vf_os_portmod_changed(: *mut adapter, _arg: c_int);
}
//
// SGE function prototype declarations.
//
extern "C" {
    pub fn t4vf_free_sge_resources(: *mut adapter);
}
extern "C" {
    pub fn t4vf_eth_xmit(: *mut sk_buff, : *mut net_device) -> netdev_tx_t;
}
extern "C" {
    pub fn t4vf_intr_handler(: *mut adapter) -> irq_handler_t;
}
extern "C" {
    pub fn t4vf_sge_intr_msix(_arg: c_int, : *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn t4vf_sge_init(: *mut adapter) -> c_int;
}
extern "C" {
    pub fn t4vf_sge_start(: *mut adapter);
}
extern "C" {
    pub fn t4vf_sge_stop(: *mut adapter);
}
