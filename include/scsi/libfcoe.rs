//! Automatically rewritten from C Header to Rust Module
//! Source: include/scsi/libfcoe.h
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
// Copyright (c) 2008-2009 Cisco Systems, Inc.  All rights reserved.
// Copyright (c) 2007-2008 Intel Corporation.  All rights reserved.
//
// Maintained at www.Open-FCoE.org
//

//
// Max MTU for FCoE: 14 (FCoE header) + 24 (FC header) + 2112 (max FC payload)
// + 4 (FC CRC) + 4 (FCoE trailer) =  2158 bytes
//
pub const FCOE_MTU: c_int = 2158;
//
// FIP tunable parameters.
//

//
// enum fip_state - internal state of FCoE controller.
// @FIP_ST_DISABLED: 	controller has been disabled or not yet enabled.
// @FIP_ST_LINK_WAIT:	the physical link is down or unusable.
// @FIP_ST_AUTO:	determining whether to use FIP or non-FIP mode.
// @FIP_ST_NON_FIP:	non-FIP mode selected.
// @FIP_ST_ENABLED:	FIP mode selected.
// @FIP_ST_VNMP_START:	VN2VN multipath mode start, wait
// @FIP_ST_VNMP_PROBE1:	VN2VN sent first probe, listening
// @FIP_ST_VNMP_PROBE2:	VN2VN sent second probe, listening
// @FIP_ST_VNMP_CLAIM:	VN2VN sent claim, waiting for responses
// @FIP_ST_VNMP_UP:	VN2VN multipath mode operation
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fip_state {
    FIP_ST_DISABLED,
    FIP_ST_LINK_WAIT,
    FIP_ST_AUTO,
    FIP_ST_NON_FIP,
    FIP_ST_ENABLED,
    FIP_ST_VNMP_START,
    FIP_ST_VNMP_PROBE1,
    FIP_ST_VNMP_PROBE2,
    FIP_ST_VNMP_CLAIM,
    FIP_ST_VNMP_UP,
}

//
// Modes:
// The mode is the state that is to be entered after link up.
// It must not change after fcoe_ctlr_init() sets it.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fip_mode {
    FIP_MODE_AUTO,
    FIP_MODE_NON_FIP,
    FIP_MODE_FABRIC,
    FIP_MODE_VN2VN,
}

//
// struct fcoe_ctlr - FCoE Controller and FIP state
// @state:	   internal FIP state for network link and FIP or non-FIP mode.
// @mode:	   LLD-selected mode.
// @lp:		   &fc_lport: libfc local port.
// @sel_fcf:	   currently selected FCF, or NULL.
// @fcfs:	   list of discovered FCFs.
// @cdev:          (Optional) pointer to sysfs fcoe_ctlr_device.
// @fcf_count:	   number of discovered FCF entries.
// @sol_time:	   time when a multicast solicitation was last sent.
// @sel_time:	   time after which to select an FCF.
// @port_ka_time:  time of next port keep-alive.
// @ctlr_ka_time:  time of next controller keep-alive.
// @timer:	   timer struct used for all delayed events.
// @timer_work:	   &work_struct for doing keep-alives and resets.
// @recv_work:	   &work_struct for receiving FIP frames.
// @fip_recv_list: list of received FIP frames.
// @flogi_req:	   clone of FLOGI request sent
// @rnd_state:	   state for pseudo-random number generator.
// @port_id:	   proposed or selected local-port ID.
// @user_mfs:	   configured maximum FC frame size, including FC header.
// @flogi_oxid:    exchange ID of most recent fabric login.
// @flogi_req_send: send of FLOGI requested
// @flogi_count:   number of FLOGI attempts in AUTO mode.
// @map_dest:	   use the FC_MAP mode for destination MAC addresses.
// @fip_resp:	   start FIP VLAN discovery responder
// @spma:	   supports SPMA server-provided MACs mode
// @probe_tries:   number of FC_IDs probed
// @priority:      DCBx FCoE APP priority
// @dest_addr:	   MAC address of the selected FC forwarder.
// @ctl_src_addr:  the native MAC address of our local port.
// @send:	   LLD-supplied function to handle sending FIP Ethernet frames
// @update_mac:    LLD-supplied function to handle changes to MAC addresses.
// @get_src_addr:  LLD-supplied function to supply a source MAC address.
// @ctlr_mutex:	   lock protecting this structure.
// @ctlr_lock:     spinlock covering flogi_req
//
// This structure is used by all FCoE drivers.  It contains information
// needed by all FCoE low-level drivers (LLDs) as well as internal state
// for FIP, and fields shared with the LLDS.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_ctlr {
    pub state: fip_state,
    pub mode: fip_mode,
    pub lp: *mut fc_lport,
    pub sel_fcf: *mut fcoe_fcf,
    pub fcfs: list_head,
    pub cdev: *mut fcoe_ctlr_device,
    pub fcf_count: u16,
    pub sol_time: c_ulong,
    pub sel_time: c_ulong,
    pub port_ka_time: c_ulong,
    pub ctlr_ka_time: c_ulong,
    pub timer: timer_list,
    pub timer_work: work_struct,
    pub recv_work: work_struct,
    pub fip_recv_list: sk_buff_head,
    pub flogi_req: *mut sk_buff,
    pub rnd_state: rnd_state,
    pub port_id: u32,
    pub user_mfs: u16,
    pub flogi_oxid: u16,
    pub flogi_req_send: u8,
    pub flogi_count: u8,
    pub map_dest: bool,
    pub fip_resp: bool,
    pub spma: u8,
    pub probe_tries: u8,
    pub priority: u8,
    pub dest_addr: [u8; ETH_ALEN],
    pub ctl_src_addr: [u8; ETH_ALEN],
    pub ): *mut *mut *mut void (send)(struct fcoe_ctlr , struct sk_buff,
    pub addr): *mut *mut *mut void (update_mac)(struct fc_lport , u8,
    pub ): *mut *mut *mut u8  (get_src_addr)(struct fc_lport,
    pub ctlr_mutex: mutex,
    pub ctlr_lock: spinlock_t,
}

//
// fcoe_ctlr_priv() - Return the private data from a fcoe_ctlr
// @ctlr: The fcoe_ctlr whose private data will be returned
//
// Returns: pointer to the private data
//
// This assumes that the fcoe_ctlr (x) is allocated with the fcoe_ctlr_device.
//

//
// struct fcoe_fcf - Fibre-Channel Forwarder
// @list:	 list linkage
// @event_work:  Work for FC Transport actions queue
// @fip:         The controller that the FCF was discovered on
// @fcf_dev:     The associated fcoe_fcf_device instance
// @time:	 system time (jiffies) when an advertisement was last received
// @switch_name: WWN of switch from advertisement
// @fabric_name: WWN of fabric from advertisement
// @fc_map:	 FC_MAP value from advertisement
// @fcf_mac:	 Ethernet address of the FCF for FIP traffic
// @fcoe_mac:	 Ethernet address of the FCF for FCoE traffic
// @vfid:	 virtual fabric ID
// @pri:	 selection priority, smaller values are better
// @flogi_sent:	 current FLOGI sent to this FCF
// @flags:	 flags received from advertisement
// @fka_period:	 keep-alive period, in jiffies
// @fd_flags:	 no need for FKA from ENode
//
// A Fibre-Channel Forwarder (FCF) is the entity on the Ethernet that
// passes FCoE frames on to an FC fabric.  This structure represents
// one FCF from which advertisements have been received.
//
// When looking up an FCF, @switch_name, @fabric_name, @fc_map, @vfid, and
// @fcf_mac together form the lookup key.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_fcf {
    pub list: list_head,
    pub event_work: work_struct,
    pub fip: *mut fcoe_ctlr,
    pub fcf_dev: *mut fcoe_fcf_device,
    pub time: c_ulong,
    pub switch_name: u64,
    pub fabric_name: u64,
    pub fc_map: u32,
    pub vfid: u16,
    pub fcf_mac: [u8; ETH_ALEN],
    pub fcoe_mac: [u8; ETH_ALEN],
    pub pri: u8,
    pub flogi_sent: u8,
    pub flags: u16,
    pub fka_period: u32,
    pub fd_flags:1: u8,
}

//
// struct fcoe_rport - VN2VN remote port
// @rdata:	libfc remote port private data
// @time:	time of create or last beacon packet received from node
// @fcoe_len:	max FCoE frame size, not including VLAN or Ethernet headers
// @flags:	flags from probe or claim
// @login_count: number of unsuccessful rport logins to this port
// @enode_mac:	E_Node control MAC address
// @vn_mac:	VN_Node assigned MAC address for data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_rport {
    pub rdata: fc_rport_priv,
    pub time: c_ulong,
    pub fcoe_len: u16,
    pub flags: u16,
    pub login_count: u8,
    pub enode_mac: [u8; ETH_ALEN],
    pub vn_mac: [u8; ETH_ALEN],
}

// FIP API functions
extern "C" {
    pub fn fcoe_ctlr_init(: *mut fcoe_ctlr, fip_mode: enum);
}
extern "C" {
    pub fn fcoe_ctlr_destroy(: *mut fcoe_ctlr);
}
extern "C" {
    pub fn fcoe_ctlr_link_up(: *mut fcoe_ctlr);
}
extern "C" {
    pub fn fcoe_ctlr_link_down(: *mut fcoe_ctlr) -> c_int;
}
extern "C" {
    pub fn fcoe_ctlr_els_send(: *mut fcoe_ctlr, : *mut fc_lport, : *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn fcoe_ctlr_recv(: *mut fcoe_ctlr, : *mut sk_buff);
}
// libfcoe funcs
extern "C" {
    pub fn fcoe_fc_crc(fp: *mut fc_frame) -> u32;
}
extern "C" {
    pub fn fcoe_start_io(skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn fcoe_get_wwn(netdev: *mut net_device, wwn: *mut u64, type: c_int) -> c_int;
}
extern "C" {
    pub fn fcoe_wwn_to_str(wwn: u64, buf: *mut c_char, len: c_int);
}
extern "C" {
    pub fn fcoe_validate_vport_create(vport: *mut fc_vport) -> c_int;
}
extern "C" {
    pub fn fcoe_link_speed_update(: *mut fc_lport) -> c_int;
}
extern "C" {
    pub fn fcoe_get_lesb(: *mut fc_lport, : *mut fc_els_lesb);
}
extern "C" {
    pub fn fcoe_ctlr_get_lesb(ctlr_dev: *mut fcoe_ctlr_device);
}
//
// is_fip_mode() - test if FIP mode selected.
// @fip:	FCoE controller.
//
// Returns: %true if FIP mode is selected
//
// helper for FCoE SW HBA drivers, can include subven and subdev if needed. The
// modpost would use pci_device_id table to auto-generate formatted module alias
// into the corresponding .mod.c file, but there may or may not be a pci device
// id table for FCoE drivers so we use the following helper for build the fcoe
// driver module alias.
//

// the name of the default FCoE transport driver fcoe.ko

// struct fcoe_transport - The FCoE transport interface
// @name:	a vendor specific name for their FCoE transport driver
// @attached:	whether this transport is already attached
// @list:	list linkage to all attached transports
// @match:	handler to allow the transport driver to match up a given netdev
// @alloc:      handler to allocate per-instance FCoE structures
// (no discovery or login)
// @create:	handler to sysfs entry of create for FCoE instances
// @destroy:    handler to delete per-instance FCoE structures
// (frees all memory)
// @enable:	handler to sysfs entry of enable for FCoE instances
// @disable:	handler to sysfs entry of disable for FCoE instances
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_transport {
    pub name: [c_char; IFNAMSIZ],
    pub attached: bool,
    pub list: list_head,
    pub device): *mut *mut bool (match) (struct net_device,
    pub device): *mut *mut int (alloc) (struct net_device,
    pub fip_mode): *mut *mut *mut int (create) (struct net_device device, enum fip_mode,
    pub device): *mut *mut int (destroy) (struct net_device,
    pub device): *mut *mut int (enable) (struct net_device,
    pub device): *mut *mut int (disable) (struct net_device,
}

//
// struct fcoe_percpu_s - The context for FCoE receive thread(s)
// @kthread:	    The thread context (used by bnx2fc)
// @work:	    The work item (used by fcoe)
// @fcoe_rx_list:   The queue of pending packets to process
// @crc_eof_page:   The memory page for calculating frame trailer CRCs
// @crc_eof_offset: The offset into the CRC page pointing to available
// memory for a new trailer
// @lock:	    local lock for members of this struct
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_percpu_s {
    pub kthread: *mut task_struct,
    pub work: work_struct,
    pub fcoe_rx_list: sk_buff_head,
    pub crc_eof_page: *mut page,
    pub crc_eof_offset: c_int,
    pub lock: local_lock_t,
}

//
// struct fcoe_port - The FCoE private structure
// @priv:		       The associated fcoe interface. The structure is
// defined by the low level driver
// @lport:		       The associated local port
// @fcoe_pending_queue:	       The pending Rx queue of skbs
// @fcoe_pending_queue_active: Indicates if the pending queue is active
// @max_queue_depth:	       Max queue depth of pending queue
// @min_queue_depth:	       Min queue depth of pending queue
// @timer:		       The queue timer
// @destroy_work:	       Handle for work context
// (to prevent RTNL deadlocks)
// @data_src_addr:	       Source address for data
// @get_netdev:                function that returns a &net_device from @lport
//
// An instance of this structure is to be allocated along with the
// Scsi_Host and libfc fc_lport structures.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_port {
    pub priv: *mut c_void,
    pub lport: *mut fc_lport,
    pub fcoe_pending_queue: sk_buff_head,
    pub fcoe_pending_queue_active: u8,
    pub max_queue_depth: u32,
    pub min_queue_depth: u32,
    pub timer: timer_list,
    pub destroy_work: work_struct,
    pub data_src_addr: [u8; ETH_ALEN],
    pub lport): *const *const *const net_device  (get_netdev)(fc_lport,
}

//
// fcoe_get_netdev() - Return the net device associated with a local port
// @lport: The local port to get the net device from
//
// Returns: the &net_device associated with this @lport
//
extern "C" {
    pub fn fcoe_clean_pending_queue(: *mut fc_lport);
}
extern "C" {
    pub fn fcoe_check_wait_queue(lport: *mut fc_lport, skb: *mut sk_buff);
}
extern "C" {
    pub fn fcoe_queue_timer(t: *mut timer_list);
}
// FCoE Sysfs helpers
extern "C" {
    pub fn fcoe_fcf_get_selected(: *mut fcoe_fcf_device);
}
extern "C" {
    pub fn fcoe_ctlr_set_fip_mode(: *mut fcoe_ctlr_device);
}
//
// struct fcoe_netdev_mapping - A mapping from &net_device to &fcoe_transport
// @list: list linkage of the mappings
// @netdev: the &net_device
// @ft: the fcoe_transport associated with @netdev
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_netdev_mapping {
    pub list: list_head,
    pub netdev: *mut net_device,
    pub ft: *mut fcoe_transport,
}

// fcoe transports registration and deregistration
extern "C" {
    pub fn fcoe_transport_attach(ft: *mut fcoe_transport) -> c_int;
}
extern "C" {
    pub fn fcoe_transport_detach(ft: *mut fcoe_transport) -> c_int;
}
// sysfs store handler for ctrl_control interface
extern "C" {
    pub fn fcoe_ctlr_create_store(buf: *const c_char, count: usize) -> isize;
}
extern "C" {
    pub fn fcoe_ctlr_destroy_store(buf: *const c_char, count: usize) -> isize;
}
