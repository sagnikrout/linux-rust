//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/netdevice.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// INET		An implementation of the TCP/IP protocol suite for the LINUX
// operating system.  INET is implemented using the  BSD Socket
// interface as the means of communication with the user level.
//
// Definitions for the Interfaces handler.
//
// Version:	@(#)dev.h	1.0.10	08/12/93
//
// Authors:	Ross Biro
// Fred N. van Kempen, <waltje@uWalt.NL.Mugnet.ORG>
// Corey Minyard <wf-rch!minyard@relay.EU.net>
// Donald J. Becker, <becker@cesdis.gsfc.nasa.gov>
// Alan Cox, <alan@lxorguk.ukuu.org.uk>
// Bjorn Ekwall. <bj0rn@blox.se>
// Pekka Riikonen <priikone@poseidon.pspt.fi>
//
// Moved to /usr/include/linux for NET3
//

// 802.11 specific
// 802.15.4 specific
// UDP Tunnel offloads
pub type xdp_features_t = u32;
extern "C" {
    pub fn synchronize_net();
}
extern "C" {
    pub fn netdev_sw_irq_coalesce_default_on(dev: *mut net_device);
}
// Backlog congestion levels

pub const MAX_NEST_DEV: c_int = 8;
//
// Transmit return codes: transmit return codes originate from three different
// namespaces:
//
// - qdisc return codes
// - driver transmit return codes
// - errno values
//
// Drivers are allowed to return any one of those in their hard_start_xmit()
// function. Real network devices commonly used with qdiscs should only return
// the driver transmit return codes though - when qdiscs are used, the actual
// transmission happens asynchronously, so the value is not propagated to
// higher layers. Virtual network devices transmit synchronously; in this case
// the driver transmit return codes are consumed by dev_queue_xmit(), and all
// others are propagated to higher layers.
//
// qdisc ->enqueue() return codes.
pub const NET_XMIT_SUCCESS: c_uint = 0x00;
pub const NET_XMIT_DROP: c_uint = 0x01	/* skb dropped			*/;
pub const NET_XMIT_CN: c_uint = 0x02	/* congestion notification	*/;
pub const NET_XMIT_MASK: c_uint = 0x0f	/* qdisc flags in net/sch_generic.h */;
// NET_XMIT_CN is special. It does not guarantee that this packet is lost. It
// indicates that the device will soon be dropping packets, or already drops
// some packets of the same priority; prompting us to send less aggressively.

// Driver transmit return codes
pub const NETDEV_TX_MASK: c_uint = 0xf0;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum netdev_tx {
    __NETDEV_TX_MIN	 = INT_MIN,	/* make sure enum is signed */
    NETDEV_TX_OK	 = 0x00,	/* driver took care of packet */
    NETDEV_TX_BUSY	 = 0x10,	/* driver tx path was busy*/
}

pub type netdev_tx_t = netdev_tx;
//
// Current order: NETDEV_TX_MASK > NET_XMIT_MASK >= 0 is significant;
// hard_start_xmit() return < NET_XMIT_MASK means skb was consumed.
//
// Positive cases with an skb consumed by a driver:
// - successful transmission (rc == NETDEV_TX_OK)
// - error while transmitting (rc < 0)
// - error while queueing to a different device (rc & NET_XMIT_MASK)
//
// Compute the worst-case header length according to the protocols
// used.
//

//
// Old network device statistics. Fields are native words
// (unsigned long) so they can be read and written atomically.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct net_device_stats {
}

// per-cpu stats, allocated on demand.
// Try to fit them in a single cache line, for dev_get_stats() sake.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct net_device_core_stats {
    pub rx_dropped: c_ulong,
    pub tx_dropped: c_ulong,
    pub rx_nohandler: c_ulong,
    pub rx_otherhost_dropped: c_ulong,
    pub long)): *mut *mut } __aligned(4  sizeof(unsigned,

    pub neighbour: struct,
    pub neigh_parms: struct,
    pub sk_buff: struct,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct netdev_hw_addr {
    pub list: list_head,
    pub node: rb_node,
    pub addr: [c_uchar; MAX_ADDR_LEN],
    pub type: c_uchar,
pub const NETDEV_HW_ADDR_T_LAN: c_int = 1;
pub const NETDEV_HW_ADDR_T_SAN: c_int = 2;
pub const NETDEV_HW_ADDR_T_UNICAST: c_int = 3;
pub const NETDEV_HW_ADDR_T_MULTICAST: c_int = 4;
    pub global_use: bool,
    pub sync_cnt: c_int,
    pub refcount: c_int,
    pub synced: c_int,
    pub rcu_head: rcu_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct netdev_hw_addr_list {
    pub list: list_head,
    pub count: c_int,
// Auxiliary tree for faster lookup on addition and deletion
    pub tree: rb_root,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hh_cache {
    pub hh_len: c_uint,
    pub hh_lock: seqlock_t,
// cached hardware header; allow for machine alignment needs.
pub const HH_DATA_MOD: c_int = 16;

    pub sizeof(long)]: unsigned long hh_data[HH_DATA_ALIGN(LL_MAX_HEADER) /,
}

// Reserve HH_DATA_MOD byte-aligned hard_header_len, but at least that much.
// Alternative is:
// dev->hard_header_len ? (dev->hard_header_len +
// (HH_DATA_MOD - 1)) & ~(HH_DATA_MOD - 1) : 0
//
// We could use other alignment values, but we must maintain the
// relationship HH alignment <= LL alignment.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct header_ops {
    pub len): *const *const void saddr, unsigned int,
    pub haddr): *mut c_uchar,
    pub type): *const *const *const *const int (cache)(struct neighbour neigh, struct hh_cache hh, __be16,
    pub haddr): *const c_uchar,
    pub len): *const *const *const bool (validate)(char ll_header, unsigned int,
    pub skb): *const *const __be16 (parse_protocol)(struct sk_buff,
}

// These flag bits are private to the generic network queueing
// layer; they may not be explicitly referenced by any other
// code.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum netdev_state_t {
    __LINK_STATE_START,
    __LINK_STATE_PRESENT,
    __LINK_STATE_NOCARRIER,
    __LINK_STATE_LINKWATCH_PENDING,
    __LINK_STATE_DORMANT,
    __LINK_STATE_TESTING,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gro_list {
    pub list: list_head,
    pub count: c_int,
}

//
// size of gro hash buckets, must be <= the number of bits in
// gro_node::bitmask
//
pub const GRO_HASH_BUCKETS: c_int = 8;
//
// struct gro_node - structure to support Generic Receive Offload
// @bitmask: bitmask to indicate used buckets in @hash
// @hash: hashtable of pending aggregated skbs, separated by flows
// @rx_list: list of pending ``GRO_NORMAL`` skbs
// @rx_count: cached current length of @rx_list
// @cached_napi_id: napi_struct::napi_id cached for hotpath, 0 for standalone
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gro_node {
    pub bitmask: c_ulong,
    pub hash: [gro_list; GRO_HASH_BUCKETS],
    pub rx_list: list_head,
    pub rx_count: u32,
    pub cached_napi_id: u32,
}

//
// Structure for per-NAPI config
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct napi_config {
    pub gro_flush_timeout: u64,
    pub irq_suspend_timeout: u64,
    pub defer_hard_irqs: u32,
    pub affinity_mask: cpumask_t,
    pub threaded: u8,
    pub napi_id: c_uint,
}

//
// Structure for NAPI scheduling similar to tasklet but with weighting
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct napi_struct {
// This field should be first or softnet_data.backlog needs tweaks.
    pub state: c_ulong,
// The poll_list must only be managed by the entity which
// changes the state of the NAPI_STATE_SCHED bit.  This means
// whoever atomically sets that bit can add this napi_struct
// to the per-CPU poll_list, and whoever clears that bit
// can remove from the list right before clearing the bit.
//
    pub poll_list: list_head,
    pub weight: c_int,
    pub defer_hard_irqs_count: u32,
    pub int): *mut *mut *mut int (poll)(struct napi_struct ,,

// CPU actively polling if netpoll is configured
    pub poll_owner: c_int,

// CPU on which NAPI has been scheduled for processing
    pub list_owner: c_int,
    pub dev: *mut net_device,
    pub skb: *mut sk_buff,
    pub gro: gro_node,
    pub timer: hrtimer,
// all fields past this point are write-protected by netdev_lock
    pub thread: *mut task_struct,
    pub gro_flush_timeout: c_ulong,
    pub irq_suspend_timeout: c_ulong,
    pub defer_hard_irqs: u32,
// control-path-only fields follow
    pub napi_id: u32,
    pub dev_list: list_head,
    pub napi_hash_node: hlist_node,
    pub irq: c_int,
    pub notify: irq_affinity_notify,
    pub napi_rmap_idx: c_int,
    pub index: c_int,
    pub config: *mut napi_config,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gro_result {
    GRO_MERGED,
    GRO_MERGED_FREE,
    GRO_HELD,
    GRO_NORMAL,
    GRO_CONSUMED,
}

pub type gro_result_t = gro_result;
//
// enum rx_handler_result - Possible return values for rx_handlers.
// @RX_HANDLER_CONSUMED: skb was consumed by rx_handler, do not process it
// further.
// @RX_HANDLER_ANOTHER: Do another round in receive path. This is indicated in
// case skb->dev was changed by rx_handler.
// @RX_HANDLER_EXACT: Force exact delivery, no wildcard.
// @RX_HANDLER_PASS: Do nothing, pass the skb as if no rx_handler was called.
//
// rx_handlers are functions called from inside __netif_receive_skb(), to do
// special processing of the skb, prior to delivery to protocol handlers.
//
// Currently, a net_device can only have a single rx_handler registered. Trying
// to register a second rx_handler will return -EBUSY.
//
// To register a rx_handler on a net_device, use netdev_rx_handler_register().
// To unregister a rx_handler on a net_device, use
// netdev_rx_handler_unregister().
//
// Upon return, rx_handler is expected to tell __netif_receive_skb() what to
// do with the skb.
//
// If the rx_handler consumed the skb in some way, it should return
// RX_HANDLER_CONSUMED. This is appropriate when the rx_handler arranged for
// the skb to be delivered in some other way.
//
// If the rx_handler changed skb->dev, to divert the skb to another
// net_device, it should return RX_HANDLER_ANOTHER. The rx_handler for the
// new device will be called if it exists.
//
// If the rx_handler decides the skb should be ignored, it should return
// RX_HANDLER_EXACT. The skb will only be delivered to protocol handlers that
// are registered on exact device (ptype->dev == skb->dev).
//
// If the rx_handler didn't change skb->dev, but wants the skb to be normally
// delivered, it should return RX_HANDLER_PASS.
//
// A device without a registered rx_handler will behave as if rx_handler
// returned RX_HANDLER_PASS.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rx_handler_result {
    RX_HANDLER_CONSUMED,
    RX_HANDLER_ANOTHER,
    RX_HANDLER_EXACT,
    RX_HANDLER_PASS,
}

pub type rx_handler_result_t = rx_handler_result;
extern "C" {
    pub fn rx_handler_func_t(pskb: *mut sk_buff) -> typedef rx_handler_result_t;
}
extern "C" {
    pub fn __napi_schedule(n: *mut napi_struct);
}
extern "C" {
    pub fn __napi_schedule_irqoff(n: *mut napi_struct);
}
extern "C" {
    pub fn test_bit(_arg: NAPI_STATE_DISABLE, _arg: &n->state) -> return;
}
extern "C" {
    pub fn test_bit(_arg: NAPI_STATE_PREFER_BUSY_POLL, _arg: &n->state) -> return;
}
//
// napi_is_scheduled - test if NAPI is scheduled
// @n: NAPI context
//
// This check is "best-effort". With no locking implemented,
// a NAPI can be scheduled or terminate right after this check
// and produce not precise results.
//
// NAPI_STATE_SCHED is an internal state, napi_is_scheduled
// should not be used normally and napi_schedule should be
// used instead.
//
// Use only if the driver really needs to check if a NAPI
// is scheduled for example in the context of delayed timer
// that can be skipped if a NAPI is already scheduled.
//
// Return: True if NAPI is scheduled, False otherwise.
//
extern "C" {
    pub fn test_bit(_arg: NAPI_STATE_SCHED, _arg: &n->state) -> return;
}
extern "C" {
    pub fn napi_schedule_prep(n: *mut napi_struct) -> bool;
}
//
// napi_schedule - schedule NAPI poll
// @n: NAPI context
//
// Schedule NAPI poll routine to be called if it is not already
// running.
// Return: true if we schedule a NAPI or false if not.
// Refer to napi_schedule_prep() for additional reason on why
// a NAPI might not be scheduled.
//
// napi_schedule_irqoff - schedule NAPI poll
// @n: NAPI context
//
// Variant of napi_schedule(), assuming hard irqs are masked.
//
// napi_complete_done - NAPI processing complete
// @n: NAPI context
// @work_done: number of packets processed
//
// Mark NAPI processing as complete. Should only be called if poll budget
// has not been completely consumed.
// Prefer over napi_complete().
// Return: false if device should avoid rearming interrupts.
//
extern "C" {
    pub fn napi_complete_done(n: *mut napi_struct, work_done: c_int) -> bool;
}
extern "C" {
    pub fn napi_complete_done(_arg: n, _arg: 0) -> return;
}
extern "C" {
    pub fn netif_threaded_enable(dev: *mut net_device);
}
extern "C" {
    pub fn napi_disable(n: *mut napi_struct);
}
extern "C" {
    pub fn napi_disable_locked(n: *mut napi_struct);
}
extern "C" {
    pub fn napi_enable(n: *mut napi_struct);
}
extern "C" {
    pub fn napi_enable_locked(n: *mut napi_struct);
}
//
// napi_synchronize - wait until NAPI is not running
// @n: NAPI context
//
// Wait until NAPI is done being scheduled on this context.
// Waits till any outstanding processing completes but
// does not disable future activations.
//
// napi_if_scheduled_mark_missed - if napi is running, set the
// NAPIF_STATE_MISSED
// @n: NAPI context
//
// If napi is running, set the NAPIF_STATE_MISSED, and return true if
// NAPI is scheduled.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum netdev_queue_state_t {
    __QUEUE_STATE_DRV_XOFF,
    __QUEUE_STATE_STACK_XOFF,
    __QUEUE_STATE_FROZEN,
}

//
// __QUEUE_STATE_DRV_XOFF is used by drivers to stop the transmit queue.  The
// netif_tx_* functions below are used to manipulate this flag.  The
// __QUEUE_STATE_STACK_XOFF flag is used by the stack to stop the transmit
// queue independently.  The netif_xmit_*stopped functions below are called
// to check if the queue has been stopped by the driver or stack (either
// of the XOFF bits are set in the state).  Drivers should not need to call
// netif_xmit*stopped functions, they should only be using netif_tx_*.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct netdev_queue {
//
// read-mostly part
//
    pub dev: *mut net_device,
    pub dev_tracker: netdevice_tracker,
    pub qdisc: *mut Qdisc __rcu,
    pub qdisc_sleeping: *mut Qdisc __rcu,

    pub kobj: kobject,
    pub groups: *const attribute_group,

    pub tx_maxrate: c_ulong,
//
// Number of TX timeouts for this queue
// (/sys/class/net/DEV/Q/trans_timeout)
//
    pub trans_timeout: atomic_long_t,
// Subordinate device that the queue has been assigned to
    pub sb_dev: *mut net_device,

// "ops protected", see comment about net_device::lock
    pub pool: *mut xsk_buff_pool,

//
// write-mostly part
//

    pub dql: dql,

    pub ____cacheline_aligned_in_smp: spinlock_t _xmit_lock,
    pub xmit_lock_owner: c_int,
//
// Time (in jiffies) of last Tx
//
    pub trans_start: c_ulong,
    pub state: c_ulong,
//
// slow- / control-path part
//
// NAPI instance for the queue
// "ops protected", see comment about net_device::lock
//
    pub napi: *mut napi_struct,

    pub numa_node: c_int,

    pub ____cacheline_aligned_in_smp: },
    pub sysctl_fb_tunnels_only_for_init_net: extern int,
    pub sysctl_devconf_inherit_init_net: extern int,
//
// sysctl_fb_tunnels_only_for_init_net == 0 : For all netns
// == 1 : For initns only
// == 2 : For none.
//

    pub READ_ONCE(sysctl_fb_tunnels_only_for_init_net): int fb_tunnels_only_for_init_net =,
    pub 1): (net_eq(net, &init_net) && fb_tunnels_only_for_init_net ==,

    pub true: return,

    pub READ_ONCE(sysctl_devconf_inherit_init_net): return,

    pub 0: return,

    pub q->numa_node: return,

    pub NUMA_NO_NODE: return,

    pub node: q->numa_node =,

    pub filter_id): u16,

// XPS map type and offset of the xps map within net_device->xps_maps[].
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xps_map_type {
    XPS_CPUS = 0,
    XPS_RXQS,
    XPS_MAPS_MAX,
}

//
// This structure holds an XPS map which can be of variable length.  The
// map is an array of queues.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xps_map {
    pub len: c_uint,
    pub alloc_len: c_uint,
    pub rcu: rcu_head,
    pub queues: [u16; ],
}

//
// This structure holds all XPS maps for device.  Maps are indexed by CPU.
//
// We keep track of the number of cpus/rxqs used when the struct is allocated,
// in nr_ids. This will help not accessing out-of-bound memory.
//
// We keep track of the number of traffic classes used when the struct is
// allocated, in num_tc. This will be used to navigate the maps, to ensure we're
// not crossing its upper bound, as the original dev->num_tc can be updated in
// the meantime.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xps_dev_maps {
    pub rcu: rcu_head,
    pub nr_ids: c_uint,
    pub num_tc: i16,
    pub /: *mut *mut *mut xps_map __rcu attr_map[]; / Either CPUs map or RXQs map,
}

pub const TC_MAX_QUEUE: c_int = 16;
pub const TC_BITMASK: c_int = 15;
// HW offloaded queuing disciplines txq count and offset maps
#[repr(C)]
#[derive(Copy, Clone)]
pub struct netdev_tc_txq {
    pub count: u16,
    pub offset: u16,
}

//
// This structure is to hold information about the device
// configured to run FCoE protocol stack.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct netdev_fcoe_hbainfo {
    pub manufacturer: [c_char; 64],
    pub serial_number: [c_char; 64],
    pub hardware_version: [c_char; 64],
    pub driver_version: [c_char; 64],
    pub optionrom_version: [c_char; 64],
    pub firmware_version: [c_char; 64],
    pub model: [c_char; 256],
    pub model_description: [c_char; 256],
}

pub const MAX_PHYS_ITEM_ID_LEN: c_int = 32;
// This structure holds a unique identifier to identify some
// physical item (port for example) used by a netdevice.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct netdev_phys_item_id {
    pub id: [c_uchar; MAX_PHYS_ITEM_ID_LEN],
    pub id_len: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum net_device_path_type {
    DEV_PATH_ETHERNET = 0,
    DEV_PATH_VLAN,
    DEV_PATH_BRIDGE,
    DEV_PATH_PPPOE,
    DEV_PATH_DSA,
    DEV_PATH_MTK_WDMA,
    DEV_PATH_TUN,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct net_device_path {
    pub type: net_device_path_type,
    pub dev: *const net_device,
    pub id: u16,
    pub proto: __be16,
    pub h_dest: [u8; ETH_ALEN],
    pub encap: },
    pub dst: *mut dst_entry,
    pub src_v4: in_addr,
    pub src_v6: in6_addr,
}

pub const NET_DEVICE_PATH_STACK_MAX: c_int = 5;
pub const NET_DEVICE_PATH_VLAN_MAX: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct net_device_path_stack {
    pub num_paths: c_int,
    pub path: [net_device_path; NET_DEVICE_PATH_STACK_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct net_device_path_ctx {
    pub dev: *const net_device,
    pub daddr: [u8; ETH_ALEN],
    pub ether_type: __be16,
    pub num_vlans: c_int,
    pub id: u16,
    pub proto: __be16,
    pub vlan: [}; NET_DEVICE_PATH_VLAN_MAX],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tc_setup_type {
    TC_QUERY_CAPS,
    TC_SETUP_QDISC_MQPRIO,
    TC_SETUP_CLSU32,
    TC_SETUP_CLSFLOWER,
    TC_SETUP_CLSMATCHALL,
    TC_SETUP_CLSBPF,
    TC_SETUP_BLOCK,
    TC_SETUP_QDISC_CBS,
    TC_SETUP_QDISC_RED,
    TC_SETUP_QDISC_PRIO,
    TC_SETUP_QDISC_MQ,
    TC_SETUP_QDISC_ETF,
    TC_SETUP_ROOT_QDISC,
    TC_SETUP_QDISC_GRED,
    TC_SETUP_QDISC_TAPRIO,
    TC_SETUP_FT,
    TC_SETUP_QDISC_ETS,
    TC_SETUP_QDISC_TBF,
    TC_SETUP_QDISC_FIFO,
    TC_SETUP_QDISC_HTB,
    TC_SETUP_ACT,
}

// These structures hold the attributes of bpf state that are being passed
// to the netdevice through the bpf op.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bpf_netdev_command {
// Set or clear a bpf program used in the earliest stages of packet
// rx. The prog will have been loaded as BPF_PROG_TYPE_XDP. The callee
// is responsible for calling bpf_prog_put on any old progs that are
// stored. In case of error, the callee need not release the new prog
// reference, but on success it takes ownership and must bpf_prog_put
// when it is no longer used.
//
    XDP_SETUP_PROG,
    XDP_SETUP_PROG_HW,
// BPF program for offload callbacks, invoked at program load time.
    BPF_OFFLOAD_MAP_ALLOC,
    BPF_OFFLOAD_MAP_FREE,
    XDP_SETUP_XSK_POOL,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bpf_xdp_mode {
    XDP_MODE_SKB = 0,
    XDP_MODE_DRV = 1,
    XDP_MODE_HW = 2,
    __MAX_XDP_MODE
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_xdp_entity {
    pub prog: *mut bpf_prog,
    pub link: *mut bpf_xdp_link,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct netdev_bpf {
    pub command: bpf_netdev_command,
// XDP_SETUP_PROG
    pub flags: u32,
    pub prog: *mut bpf_prog,
    pub extack: *mut netlink_ext_ack,
}

// BPF_OFFLOAD_MAP_ALLOC, BPF_OFFLOAD_MAP_FREE
// XDP_SETUP_XSK_POOL
// Flags for ndo_xsk_wakeup.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrmdev_ops {
    pub extack): *mut netlink_ext_ack,
    pub x): *mut xfrm_state,
    pub x): *mut xfrm_state,
    pub x): *mut xfrm_state,
    pub x): *mut *mut void (xdo_dev_state_advance_esn) (struct xfrm_state,
    pub x): *mut *mut void (xdo_dev_state_update_stats) (struct xfrm_state,
    pub extack): *mut *mut *mut int (xdo_dev_policy_add) (struct xfrm_policy x, struct netlink_ext_ack,
    pub x): *mut *mut void (xdo_dev_policy_delete) (struct xfrm_policy,
    pub x): *mut *mut void (xdo_dev_policy_free) (struct xfrm_policy,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dev_ifalias {
    pub rcuhead: rcu_head,
    pub ifalias: [c_char; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct netdev_net_notifier {
    pub list: list_head,
    pub nb: *mut notifier_block,
}

//
// This structure defines the management hooks for network devices.
// The following hooks can be defined; unless noted otherwise, they are
// optional and can be filled with a null pointer.
//
// int (*ndo_init)(struct net_device *dev);
// This function is called once when a network device is registered.
// The network device can use this for any late stage initialization
// or semantic validation. It can fail with an error code which will
// be propagated back to register_netdev.
//
// void (*ndo_uninit)(struct net_device *dev);
// This function is called when device is unregistered or when registration
// fails. It is not called if init fails.
//
// int (*ndo_open)(struct net_device *dev);
// This function is called when a network device transitions to the up
// state.
//
// int (*ndo_stop)(struct net_device *dev);
// This function is called when a network device transitions to the down
// state.
//
// netdev_tx_t (*ndo_start_xmit)(struct sk_buff *skb,
// struct net_device *dev);
// Called when a packet needs to be transmitted.
// Returns NETDEV_TX_OK.  Can return NETDEV_TX_BUSY, but you should stop
// the queue before that can happen; it's for obsolete devices and weird
// corner cases, but the stack really does a non-trivial amount
// of useless work if you return NETDEV_TX_BUSY.
// Required; cannot be NULL.
//
// netdev_features_t (*ndo_features_check)(struct sk_buff *skb,
// struct net_device *dev
// netdev_features_t features);
// Called by core transmit path to determine if device is capable of
// performing offload operations on a given packet. This is to give
// the device an opportunity to implement any restrictions that cannot
// be otherwise expressed by feature flags. The check is called with
// the set of features that the stack has calculated and it returns
// those the driver believes to be appropriate.
//
// u16 (*ndo_select_queue)(struct net_device *dev, struct sk_buff *skb,
// struct net_device *sb_dev);
// Called to decide which queue to use when device supports multiple
// transmit queues.
//
// void (*ndo_change_rx_flags)(struct net_device *dev, int flags);
// This function is called to allow device receiver to make
// changes to configuration when multicast or promiscuous is enabled.
//
// void (*ndo_set_rx_mode)(struct net_device *dev);
// This function is called device changes address list filtering.
// If driver handles unicast address filtering, it should set
// IFF_UNICAST_FLT in its priv_flags.
// Cannot sleep, called with netif_addr_lock_bh held.
// Deprecated in favor of ndo_set_rx_mode_async.
//
// int (*ndo_set_rx_mode_async)(struct net_device *dev,
// struct netdev_hw_addr_list *uc,
// struct netdev_hw_addr_list *mc);
// Async version of ndo_set_rx_mode which runs in process context
// with rtnl_lock and netdev_lock_ops(dev) held. The uc/mc parameters
// are snapshots of the address lists - iterate with
// netdev_hw_addr_list_for_each(ha, uc). Return 0 on success or a
// negative errno to request a retry via the core backoff.
//
// void (*ndo_work)(struct net_device *dev, unsigned long events);
// Run deferred work scheduled with netdev_work_sched(@events).
//
// int (*ndo_set_mac_address)(struct net_device *dev, void *addr);
// This function  is called when the Media Access Control address
// needs to be changed. If this interface is not defined, the
// MAC address can not be changed.
//
// int (*ndo_validate_addr)(struct net_device *dev);
// Test if Media Access Control address is valid for the device.
//
// int (*ndo_do_ioctl)(struct net_device *dev, struct ifreq *ifr, int cmd);
// Old-style ioctl entry point. This is used internally by the
// ieee802154 subsystem but is no longer called by the device
// ioctl handler.
//
// int (*ndo_siocbond)(struct net_device *dev, struct ifreq *ifr, int cmd);
// Used by the bonding driver for its device specific ioctls:
// SIOCBONDENSLAVE, SIOCBONDRELEASE, SIOCBONDSETHWADDR, SIOCBONDCHANGEACTIVE,
// SIOCBONDSLAVEINFOQUERY, and SIOCBONDINFOQUERY
//
// * int (*ndo_eth_ioctl)(struct net_device *dev, struct ifreq *ifr, int cmd);
// Called for ethernet specific ioctls: SIOCGMIIPHY, SIOCGMIIREG and
// SIOCSMIIREG.
//
// int (*ndo_set_config)(struct net_device *dev, struct ifmap *map);
// Used to set network devices bus interface parameters. This interface
// is retained for legacy reasons; new devices should use the bus
// interface (PCI) for low level management.
//
// int (*ndo_change_mtu)(struct net_device *dev, int new_mtu);
// Called when a user wants to change the Maximum Transfer Unit
// of a device.
//
// void (*ndo_tx_timeout)(struct net_device *dev, unsigned int txqueue);
// Callback used when the transmitter has not made any progress
// for dev->watchdog ticks.
//
// void (*ndo_get_stats64)(struct net_device *dev,
// struct rtnl_link_stats64 *storage);
// struct net_device_stats* (*ndo_get_stats)(struct net_device *dev);
// Called when a user wants to get the network device usage
// statistics. Drivers must do one of the following:
// 1. Define @ndo_get_stats64 to fill in a zero-initialised
// rtnl_link_stats64 structure passed by the caller.
// 2. Define @ndo_get_stats to update a net_device_stats structure
// (which should normally be dev->stats) and return a pointer to
// it. The structure may be changed asynchronously only if each
// field is written atomically.
// 3. Update dev->stats asynchronously and atomically, and define
// neither operation.
//
// bool (*ndo_has_offload_stats)(const struct net_device *dev, int attr_id)
// Return true if this device supports offload stats of this attr_id.
//
// int (*ndo_get_offload_stats)(int attr_id, const struct net_device *dev,
// void *attr_data)
// Get statistics for offload operations by attr_id. Write it into the
// attr_data pointer.
//
// int (*ndo_vlan_rx_add_vid)(struct net_device *dev, __be16 proto, u16 vid);
// If device supports VLAN filtering this function is called when a
// VLAN id is registered.
//
// int (*ndo_vlan_rx_kill_vid)(struct net_device *dev, __be16 proto, u16 vid);
// If device supports VLAN filtering this function is called when a
// VLAN id is unregistered.
//
// void (*ndo_poll_controller)(struct net_device *dev);
//
// SR-IOV management functions.
// int (*ndo_set_vf_mac)(struct net_device *dev, int vf, u8* mac);
// int (*ndo_set_vf_vlan)(struct net_device *dev, int vf, u16 vlan,
// u8 qos, __be16 proto);
// int (*ndo_set_vf_rate)(struct net_device *dev, int vf, int min_tx_rate,
// int max_tx_rate);
// int (*ndo_set_vf_spoofchk)(struct net_device *dev, int vf, bool setting);
// int (*ndo_set_vf_trust)(struct net_device *dev, int vf, bool setting);
// int (*ndo_get_vf_config)(struct net_device *dev,
// int vf, struct ifla_vf_info *ivf);
// int (*ndo_set_vf_link_state)(struct net_device *dev, int vf, int link_state);
// int (*ndo_set_vf_port)(struct net_device *dev, int vf,
// struct nlattr *port[]);
//
// Enable or disable the VF ability to query its RSS Redirection Table and
// Hash Key. This is needed since on some devices VF share this information
// with PF and querying it may introduce a theoretical security risk.
// int (*ndo_set_vf_rss_query_en)(struct net_device *dev, int vf, bool setting);
// int (*ndo_get_vf_port)(struct net_device *dev, int vf, struct sk_buff *skb);
// int (*ndo_setup_tc)(struct net_device *dev, enum tc_setup_type type,
// void *type_data);
// Called to setup any 'tc' scheduler, classifier or action on @dev.
// This is always called from the stack with the rtnl lock held and netif
// tx queues stopped. This allows the netdevice to perform queue
// management safely.
//
// NB: Returning -EOPNOTSUPP for whatever commands means "this qdisc
// is not offloaded (anymore, offloading may have silently stopped)",
// and the offloading flag is cleared. Notably, this is also true for
// dump queries (e.g. TC_*_STATS commands). If the underlying device does
// not report any statistics but is still offloading, return 0 instead.
//
// Fiber Channel over Ethernet (FCoE) offload functions.
// int (*ndo_fcoe_enable)(struct net_device *dev);
// Called when the FCoE protocol stack wants to start using LLD for FCoE
// so the underlying device can perform whatever needed configuration or
// initialization to support acceleration of FCoE traffic.
//
// int (*ndo_fcoe_disable)(struct net_device *dev);
// Called when the FCoE protocol stack wants to stop using LLD for FCoE
// so the underlying device can perform whatever needed clean-ups to
// stop supporting acceleration of FCoE traffic.
//
// int (*ndo_fcoe_ddp_setup)(struct net_device *dev, u16 xid,
// struct scatterlist *sgl, unsigned int sgc);
// Called when the FCoE Initiator wants to initialize an I/O that
// is a possible candidate for Direct Data Placement (DDP). The LLD can
// perform necessary setup and returns 1 to indicate the device is set up
// successfully to perform DDP on this I/O, otherwise this returns 0.
//
// int (*ndo_fcoe_ddp_done)(struct net_device *dev,  u16 xid);
// Called when the FCoE Initiator/Target is done with the DDPed I/O as
// indicated by the FC exchange id 'xid', so the underlying device can
// clean up and reuse resources for later DDP requests.
//
// int (*ndo_fcoe_ddp_target)(struct net_device *dev, u16 xid,
// struct scatterlist *sgl, unsigned int sgc);
// Called when the FCoE Target wants to initialize an I/O that
// is a possible candidate for Direct Data Placement (DDP). The LLD can
// perform necessary setup and returns 1 to indicate the device is set up
// successfully to perform DDP on this I/O, otherwise this returns 0.
//
// int (*ndo_fcoe_get_hbainfo)(struct net_device *dev,
// struct netdev_fcoe_hbainfo *hbainfo);
// Called when the FCoE Protocol stack wants information on the underlying
// device. This information is utilized by the FCoE protocol stack to
// register attributes with Fiber Channel management service as per the
// FC-GS Fabric Device Management Information(FDMI) specification.
//
// int (*ndo_fcoe_get_wwn)(struct net_device *dev, u64 *wwn, int type);
// Called when the underlying device wants to override default World Wide
// Name (WWN) generation mechanism in FCoE protocol stack to pass its own
// World Wide Port Name (WWPN) or World Wide Node Name (WWNN) to the FCoE
// protocol stack to use.
//
// RFS acceleration.
// int (*ndo_rx_flow_steer)(struct net_device *dev, const struct sk_buff *skb,
// u16 rxq_index, u32 flow_id);
// Set hardware filter for RFS.  rxq_index is the target queue index;
// flow_id is a flow ID to be passed to rps_may_expire_flow() later.
// Return the filter ID on success, or a negative error code.
//
// Slave management functions (for bridge, bonding, etc).
// int (*ndo_add_slave)(struct net_device *dev, struct net_device *slave_dev);
// Called to make another netdev an underling.
//
// int (*ndo_del_slave)(struct net_device *dev, struct net_device *slave_dev);
// Called to release previously enslaved netdev.
//
// struct net_device *(*ndo_get_xmit_slave)(struct net_device *dev,
// struct sk_buff *skb,
// bool all_slaves);
// Get the xmit slave of master device. If all_slaves is true, function
// assume all the slaves can transmit.
//
// Feature/offload setting functions.
// netdev_features_t (*ndo_fix_features)(struct net_device *dev,
// netdev_features_t features);
// Adjusts the requested feature flags according to device-specific
// constraints, and returns the resulting flags. Must not modify
// the device state.
//
// int (*ndo_set_features)(struct net_device *dev, netdev_features_t features);
// Called to update device configuration to new features. Passed
// feature set might be less than what was returned by ndo_fix_features()).
// Must return >0 or -errno if it changed dev->features itself.
//
// int (*ndo_fdb_add)(struct ndmsg *ndm, struct nlattr *tb[],
// struct net_device *dev,
// const unsigned char *addr, u16 vid, u16 flags,
// bool *notified, struct netlink_ext_ack *extack);
// Adds an FDB entry to dev for addr.
// Callee shall set *notified to true if it sent any appropriate
// notification(s). Otherwise core will send a generic one.
// int (*ndo_fdb_del)(struct ndmsg *ndm, struct nlattr *tb[],
// struct net_device *dev,
// const unsigned char *addr, u16 vid
// bool *notified, struct netlink_ext_ack *extack);
// Deletes the FDB entry from dev corresponding to addr.
// Callee shall set *notified to true if it sent any appropriate
// notification(s). Otherwise core will send a generic one.
// int (*ndo_fdb_del_bulk)(struct nlmsghdr *nlh, struct net_device *dev,
// struct netlink_ext_ack *extack);
// int (*ndo_fdb_dump)(struct sk_buff *skb, struct netlink_callback *cb,
// struct net_device *dev, struct net_device *filter_dev,
// int *idx)
// Used to add FDB entries to dump requests. Implementers should add
// entries to skb and update idx with the number of entries.
//
// int (*ndo_mdb_add)(struct net_device *dev, struct nlattr *tb[],
// u16 nlmsg_flags, struct netlink_ext_ack *extack);
// Adds an MDB entry to dev.
// int (*ndo_mdb_del)(struct net_device *dev, struct nlattr *tb[],
// struct netlink_ext_ack *extack);
// Deletes the MDB entry from dev.
// int (*ndo_mdb_del_bulk)(struct net_device *dev, struct nlattr *tb[],
// struct netlink_ext_ack *extack);
// Bulk deletes MDB entries from dev.
// int (*ndo_mdb_dump)(struct net_device *dev, struct sk_buff *skb,
// struct netlink_callback *cb);
// Dumps MDB entries from dev. The first argument (marker) in the netlink
// callback is used by core rtnetlink code.
//
// int (*ndo_bridge_setlink)(struct net_device *dev, struct nlmsghdr *nlh,
// u16 flags, struct netlink_ext_ack *extack)
// int (*ndo_bridge_getlink)(struct sk_buff *skb, u32 pid, u32 seq,
// struct net_device *dev, u32 filter_mask,
// int nlflags)
// int (*ndo_bridge_dellink)(struct net_device *dev, struct nlmsghdr *nlh,
// u16 flags);
//
// int (*ndo_change_carrier)(struct net_device *dev, bool new_carrier);
// Called to change device carrier. Soft-devices (like dummy, team, etc)
// which do not represent real hardware may define this to allow their
// userspace components to manage their virtual carrier state. Devices
// that determine carrier state from physical hardware properties (eg
// network cables) or protocol-dependent mechanisms (eg
// USB_CDC_NOTIFY_NETWORK_CONNECTION) should NOT implement this function.
//
// int (*ndo_get_phys_port_id)(struct net_device *dev,
// struct netdev_phys_item_id *ppid);
// Called to get ID of physical port of this device. If driver does
// not implement this, it is assumed that the hw is not able to have
// multiple net devices on single physical port.
//
// int (*ndo_get_port_parent_id)(struct net_device *dev,
// struct netdev_phys_item_id *ppid)
// Called to get the parent ID of the physical port of this device.
//
// void* (*ndo_dfwd_add_station)(struct net_device *pdev,
// struct net_device *dev)
// Called by upper layer devices to accelerate switching or other
// station functionality into hardware. 'pdev is the lowerdev
// to use for the offload and 'dev' is the net device that will
// back the offload. Returns a pointer to the private structure
// the upper layer will maintain.
// void (*ndo_dfwd_del_station)(struct net_device *pdev, void *priv)
// Called by upper layer device to delete the station created
// by 'ndo_dfwd_add_station'. 'pdev' is the net device backing
// the station and priv is the structure returned by the add
// operation.
// int (*ndo_set_tx_maxrate)(struct net_device *dev,
// int queue_index, u32 maxrate);
// Called when a user wants to set a max-rate limitation of specific
// TX queue.
// int (*ndo_get_iflink)(const struct net_device *dev);
// Called to get the iflink value of this device.
// int (*ndo_fill_metadata_dst)(struct net_device *dev, struct sk_buff *skb);
// This function is used to get egress tunnel information for given skb.
// This is useful for retrieving outer tunnel header parameters while
// sampling packet.
// void (*ndo_set_rx_headroom)(struct net_device *dev, int needed_headroom);
// This function is used to specify the headroom that the skb must
// consider when allocation skb during packet reception. Setting
// appropriate rx headroom value allows avoiding skb head copy on
// forward. Setting a negative value resets the rx headroom to the
// default value.
// int (*ndo_bpf)(struct net_device *dev, struct netdev_bpf *bpf);
// This function is used to set or query state related to XDP on the
// netdevice and manage BPF offload. See definition of
// enum bpf_netdev_command for details.
// int (*ndo_xdp_xmit)(struct net_device *dev, int n, struct xdp_frame **xdp,
// u32 flags);
// This function is used to submit @n XDP packets for transmit on a
// netdevice. Returns number of frames successfully transmitted, frames
// that got dropped are freed/returned via xdp_return_frame().
// Returns negative number, means general error invoking ndo, meaning
// no frames were xmit'ed and core-caller will free all frames.
// struct net_device *(*ndo_xdp_get_xmit_slave)(struct net_device *dev,
// struct xdp_buff *xdp);
// Get the xmit slave of master device based on the xdp_buff.
// int (*ndo_xsk_wakeup)(struct net_device *dev, u32 queue_id, u32 flags);
// This function is used to wake up the softirq, ksoftirqd or kthread
// responsible for sending and/or receiving packets on a specific
// queue id bound to an AF_XDP socket. The flags field specifies if
// only RX, only Tx, or both should be woken up using the flags
// XDP_WAKEUP_RX and XDP_WAKEUP_TX.
// int (*ndo_tunnel_ctl)(struct net_device *dev, struct ip_tunnel_parm_kern *p,
// int cmd);
// Add, change, delete or get information on an IPv4 tunnel.
// struct net_device *(*ndo_get_peer_dev)(struct net_device *dev);
// If a device is paired with a peer device, return the peer instance.
// The caller must be under RCU read context.
// int (*ndo_fill_forward_path)(struct net_device_path_ctx *ctx, struct net_device_path *path);
// Get the forwarding path to reach the real device from the HW destination address
// ktime_t (*ndo_get_tstamp)(struct net_device *dev,
// const struct skb_shared_hwtstamps *hwtstamps,
// bool cycles);
// Get hardware timestamp based on normal/adjustable time or free running
// cycle counter. This function is required if physical clock supports a
// free running cycle counter.
//
// int (*ndo_hwtstamp_get)(struct net_device *dev,
// struct kernel_hwtstamp_config *kernel_config);
// Get the currently configured hardware timestamping parameters for the
// NIC device.
//
// int (*ndo_hwtstamp_set)(struct net_device *dev,
// struct kernel_hwtstamp_config *kernel_config,
// struct netlink_ext_ack *extack);
// Change the hardware timestamping parameters for NIC device.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct net_device_ops {
    pub dev): *mut *mut int (ndo_init)(struct net_device,
    pub dev): *mut *mut void (ndo_uninit)(struct net_device,
    pub dev): *mut *mut int (ndo_open)(struct net_device,
    pub dev): *mut *mut int (ndo_stop)(struct net_device,
    pub dev): *mut net_device,
    pub features): netdev_features_t,
    pub sb_dev): *mut net_device,
    pub flags): c_int,
    pub dev): *mut *mut void (ndo_set_rx_mode)(struct net_device,
    pub mc): *mut netdev_hw_addr_list,
    pub events): c_ulong,
    pub addr): *mut c_void,
    pub dev): *mut *mut int (ndo_validate_addr)(struct net_device,
    pub cmd): *mut *mut ifreq ifr, int,
    pub cmd): *mut *mut ifreq ifr, int,
    pub cmd): *mut *mut ifreq ifr, int,
    pub ifs): *mut if_settings,
    pub cmd): *mut *mut void __user data, int,
    pub map): *mut ifmap,
    pub new_mtu): c_int,
    pub ): *mut neigh_parms,
    pub txqueue): c_uint,
    pub storage): *mut rtnl_link_stats64,
    pub attr_id): *const *const *const bool (ndo_has_offload_stats)(struct net_device dev, int,
    pub attr_data): *mut c_void,
    pub dev): *mut *mut *mut net_device_stats (ndo_get_stats)(net_device,
    pub vid): __be16 proto, u16,
    pub vid): __be16 proto, u16,

    pub dev): *mut *mut void (ndo_poll_controller)(struct net_device,
    pub dev): *mut *mut int (ndo_netpoll_setup)(struct net_device,
    pub dev): *mut *mut void (ndo_netpoll_cleanup)(struct net_device,

    pub mac): *mut int queue, u8,
    pub proto): u8 qos, __be16,
    pub max_tx_rate): c_int,
    pub setting): int vf, bool,
    pub setting): int vf, bool,
    pub ivf): *mut ifla_vf_info,
    pub link_state): int vf, int,
// vf_stats);
    pub port[]): *mut nlattr,
    pub skb): *mut int vf, struct sk_buff,
    pub port_guid): *mut ifla_vf_guid,
    pub guid_type): c_int,
    pub setting): int vf, bool,
    pub type_data): *mut c_void,

    pub dev): *mut *mut int (ndo_fcoe_enable)(struct net_device,
    pub dev): *mut *mut int (ndo_fcoe_disable)(struct net_device,
    pub sgc): c_uint,
    pub xid): u16,
    pub sgc): c_uint,
    pub hbainfo): *mut netdev_fcoe_hbainfo,

pub const NETDEV_FCOE_WWNN: c_int = 0;
pub const NETDEV_FCOE_WWPN: c_int = 1;
    pub type): *mut *mut u64 wwn, int,

    pub flow_id): u32,

    pub extack): *mut netlink_ext_ack,
    pub slave_dev): *mut net_device,
    pub all_slaves): bool,
    pub sk): *mut sock,
    pub features): netdev_features_t,
    pub features): netdev_features_t,
    pub n): *mut neighbour,
    pub n): *mut neighbour,
    pub extack): *mut netlink_ext_ack,
    pub extack): *mut netlink_ext_ack,
    pub extack): *mut netlink_ext_ack,
    pub idx): *mut c_int,
    pub extack): *mut netlink_ext_ack,
    pub extack): *mut netlink_ext_ack,
    pub extack): *mut netlink_ext_ack,
    pub extack): *mut netlink_ext_ack,
    pub cb): *mut netlink_callback,
    pub extack): *mut netlink_ext_ack,
    pub extack): *mut netlink_ext_ack,
    pub nlflags): c_int,
    pub flags): u16,
    pub new_carrier): bool,
    pub ppid): *mut netdev_phys_item_id,
    pub ppid): *mut netdev_phys_item_id,
    pub len): *mut *mut char name, size_t,
    pub dev): *mut net_device,
    pub priv): *mut c_void,
    pub maxrate): u32,
    pub dev): *const *const int (ndo_get_iflink)(struct net_device,
    pub skb): *mut sk_buff,
    pub needed_headroom): c_int,
    pub bpf): *mut netdev_bpf,
    pub flags): u32,
    pub xdp): *mut xdp_buff,
    pub flags): u32 queue_id, u32,
    pub cmd): c_int,
    pub dev): *mut *mut *mut net_device  (ndo_get_peer_dev)(net_device,
    pub path): *mut net_device_path,
    pub cycles): bool,
    pub kernel_config): *mut kernel_hwtstamp_config,
    pub extack): *mut netlink_ext_ack,

//
// @net_shaper_ops: Device shaping offload operations
// see include/net/net_shapers.h
//
    pub net_shaper_ops: *const net_shaper_ops,

}

//
// enum netdev_priv_flags - &struct net_device priv_flags
//
// These are the &struct net_device, they are only set internally
// by drivers and used in the kernel. These flags are invisible to
// userspace; this means that the order of these flags can change
// during any kernel release.
//
// You should add bitfield booleans after either net_device::priv_flags
// (hotpath) or ::threaded (slowpath) instead of extending these flags.
//
// @IFF_802_1Q_VLAN: 802.1Q VLAN device
// @IFF_EBRIDGE: Ethernet bridging device
// @IFF_BONDING: bonding master or slave
// @IFF_ISATAP: ISATAP interface (RFC4214)
// @IFF_WAN_HDLC: WAN HDLC device
// @IFF_XMIT_DST_RELEASE: dev_hard_start_xmit() is allowed to
// release skb->dst
// @IFF_DONT_BRIDGE: disallow bridging this ether dev
// @IFF_DISABLE_NETPOLL: disable netpoll at run-time
// @IFF_MACVLAN_PORT: device used as macvlan port
// @IFF_BRIDGE_PORT: device used as bridge port
// @IFF_OVS_DATAPATH: device used as Open vSwitch datapath port
// @IFF_TX_SKB_SHARING: The interface supports sharing skbs on transmit
// @IFF_UNICAST_FLT: Supports unicast filtering
// @IFF_TEAM_PORT: device used as team port
// @IFF_SUPP_NOFCS: device supports sending custom FCS
// @IFF_LIVE_ADDR_CHANGE: device supports hardware address
// change when it's running
// @IFF_MACVLAN: Macvlan device
// @IFF_XMIT_DST_RELEASE_PERM: IFF_XMIT_DST_RELEASE not taking into account
// underlying stacked devices
// @IFF_L3MDEV_MASTER: device is an L3 master device
// @IFF_NO_QUEUE: device can run without qdisc attached
// @IFF_OPENVSWITCH: device is a Open vSwitch master
// @IFF_L3MDEV_SLAVE: device is enslaved to an L3 master device
// @IFF_TEAM: device is a team device
// @IFF_PHONY_HEADROOM: the headroom value is controlled by an external
// entity (i.e. the master device for bridged veth)
// @IFF_MACSEC: device is a MACsec device
// @IFF_NO_RX_HANDLER: device doesn't support the rx_handler hook
// @IFF_FAILOVER: device is a failover master device
// @IFF_FAILOVER_SLAVE: device is lower dev of a failover master device
// @IFF_L3MDEV_RX_HANDLER: only invoke the rx handler of L3 master device
// @IFF_NO_ADDRCONF: prevent ipv6 addrconf
// @IFF_TX_SKB_NO_LINEAR: device/driver is capable of xmitting frames with
// skb_headlen(skb) == 0 (data starts from frag0)
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum netdev_priv_flags {
    IFF_802_1Q_VLAN			= 1<<0,
    IFF_EBRIDGE			= 1<<1,
    IFF_BONDING			= 1<<2,
    IFF_ISATAP			= 1<<3,
    IFF_WAN_HDLC			= 1<<4,
    IFF_XMIT_DST_RELEASE		= 1<<5,
    IFF_DONT_BRIDGE			= 1<<6,
    IFF_DISABLE_NETPOLL		= 1<<7,
    IFF_MACVLAN_PORT		= 1<<8,
    IFF_BRIDGE_PORT			= 1<<9,
    IFF_OVS_DATAPATH		= 1<<10,
    IFF_TX_SKB_SHARING		= 1<<11,
    IFF_UNICAST_FLT			= 1<<12,
    IFF_TEAM_PORT			= 1<<13,
    IFF_SUPP_NOFCS			= 1<<14,
    IFF_LIVE_ADDR_CHANGE		= 1<<15,
    IFF_MACVLAN			= 1<<16,
    IFF_XMIT_DST_RELEASE_PERM	= 1<<17,
    IFF_L3MDEV_MASTER		= 1<<18,
    IFF_NO_QUEUE			= 1<<19,
    IFF_OPENVSWITCH			= 1<<20,
    IFF_L3MDEV_SLAVE		= 1<<21,
    IFF_TEAM			= 1<<22,
    IFF_PHONY_HEADROOM		= 1<<24,
    IFF_MACSEC			= 1<<25,
    IFF_NO_RX_HANDLER		= 1<<26,
    IFF_FAILOVER			= 1<<27,
    IFF_FAILOVER_SLAVE		= 1<<28,
    IFF_L3MDEV_RX_HANDLER		= 1<<29,
    IFF_NO_ADDRCONF			= BIT_ULL(30),
    IFF_TX_SKB_NO_LINEAR		= BIT_ULL(31),
}

// Specifies the type of the struct net_device::ml_priv pointer
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum netdev_ml_priv_type {
    ML_PRIV_NONE,
    ML_PRIV_CAN,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum netdev_stat_type {
    NETDEV_PCPU_STAT_NONE,
    NETDEV_PCPU_STAT_LSTATS, /* struct pcpu_lstats */
    NETDEV_PCPU_STAT_TSTATS, /* struct pcpu_sw_netstats */
    NETDEV_PCPU_STAT_DSTATS, /* struct pcpu_dstats */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum netmem_tx_mode {
    NETMEM_TX_NONE,		/* no netmem TX support */
    NETMEM_TX_DMA,		/* DMA-capable netmem TX (real HW) */
    NETMEM_TX_NO_DMA,	/* no DMA, e.g. passthrough for virtual devs */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum netdev_reg_state {
    NETREG_UNINITIALIZED = 0,
    NETREG_REGISTERED,	/* completed register_netdevice */
    NETREG_UNREGISTERING,	/* called unregister_netdevice */
    NETREG_UNREGISTERED,	/* completed unregister todo */
    NETREG_RELEASED,	/* called free_netdev */
    NETREG_DUMMY,		/* dummy device for NAPI poll */
}

//
// struct net_device - The DEVICE structure.
//
// Actually, this whole structure is a big mistake.  It mixes I/O
// data with strictly "high-level" data, and it has to know about
// almost every data structure used in the INET module.
//
// @priv_flags:	flags invisible to userspace defined as bits, see
// enum netdev_priv_flags for the definitions
// @lltx:		device supports lockless Tx. Deprecated for real HW
// drivers. Mainly used by logical interfaces, such as
// bonding and tunnels
// @netmem_tx:	device netmem TX mode
//
// @name:	This is the first field of the "visible" part of this structure
// (i.e. as seen by users in the "Space.c" file).  It is the name
// of the interface.
//
// @name_node:	Name hashlist node
// @ifalias:	SNMP alias
// @mem_end:	Shared memory end
// @mem_start:	Shared memory start
// @base_addr:	Device I/O address
// @irq:		Device IRQ number
//
// @state:		Generic network queuing layer state, see netdev_state_t
// @dev_list:	The global list of network devices
// @napi_list:	List entry used for polling NAPI devices
// @unreg_list:	List entry  when we are unregistering the
// device; see the function unregister_netdev
// @unreg_list_net:List entry when we are unregistering the cross-netns
// device; see the function unregister_netdevice_queue_net()
// @close_list:	List entry used when we are closing the device
// @ptype_all:     Device-specific packet handlers for all protocols
// @ptype_specific: Device-specific, protocol-specific packet handlers
//
// @adj_list:	Directly linked devices, like slaves for bonding
// @features:	Currently active device features
// @hw_features:	User-changeable features
//
// @wanted_features:	User-requested features
// @vlan_features:		Mask of features inheritable by VLAN devices
//
// @hw_enc_features:	Mask of features inherited by encapsulating devices
// This field indicates what encapsulation
// offloads the hardware is capable of doing,
// and drivers will need to set them appropriately.
//
// @mpls_features:	Mask of features inheritable by MPLS
// @gso_partial_features: value(s) from NETIF_F_GSO\
// @mangleid_features:	Mask of features requiring MANGLEID, will be
// disabled together with the latter.
//
// @ifindex:	interface index
// @group:		The group the device belongs to
//
// @stats:		Statistics struct, which was left as a legacy, use
// rtnl_link_stats64 instead
//
// @core_stats:	core networking counters,
// do not use this in drivers
// @carrier_up_count:	Number of times the carrier has been up
// @carrier_down_count:	Number of times the carrier has been down
//
// @wireless_handlers:	List of functions to handle Wireless Extensions,
// instead of ioctl,
// see <net/iw_handler.h> for details.
//
// @netdev_ops:	Includes several pointers to callbacks,
// if one wants to override the ndo_*() functions
// @xdp_metadata_ops:	Includes pointers to XDP metadata callbacks.
// @xsk_tx_metadata_ops:	Includes pointers to AF_XDP TX metadata callbacks.
// @ethtool_ops:	Management operations
// @l3mdev_ops:	Layer 3 master device operations
// @ndisc_ops:	Includes callbacks for different IPv6 neighbour
// discovery handling. Necessary for e.g. 6LoWPAN.
// @xfrmdev_ops:	Transformation offload operations
// @tlsdev_ops:	Transport Layer Security offload operations
// @header_ops:	Includes callbacks for creating,parsing,caching,etc
// of Layer 2 headers.
//
// @flags:		Interface flags (a la BSD)
// @xdp_features:	XDP capability supported by the device
// @gflags:	Global flags ( kept as legacy )
// @priv_len:	Size of the ->priv flexible array
// @priv:		Flexible array containing private data
// @operstate:	RFC2863 operstate
// @link_mode:	Mapping policy to operstate
// @if_port:	Selectable AUI, TP, ...
// @dma:		DMA channel
// @mtu:		Interface MTU value
// @min_mtu:	Interface Minimum MTU value
// @max_mtu:	Interface Maximum MTU value
// @type:		Interface hardware type
// @hard_header_len: Maximum hardware header length.
// @min_header_len:  Minimum hardware header length
//
// @needed_headroom: Extra headroom the hardware may need, but not in all
// cases can this be guaranteed
// @needed_tailroom: Extra tailroom the hardware may need, but not in all
// cases can this be guaranteed. Some cases also use
// LL_MAX_HEADER instead to allocate the skb
//
// interface address info:
//
// @perm_addr:		Permanent hw address
// @addr_assign_type:	Hw address assignment type
// @addr_len:		Hardware address length
// @upper_level:		Maximum depth level of upper devices.
// @lower_level:		Maximum depth level of lower devices.
// @threaded:		napi threaded state.
// @neigh_priv_len:	Used in neigh_alloc()
// @dev_id:		Used to differentiate devices that share
// the same link layer address
// @dev_port:		Used to differentiate devices that share
// the same function
// @addr_list_lock:	XXX: need comments on this one
// @name_assign_type:	network interface name assignment type
// @uc_promisc:		Counter that indicates promiscuous mode
// has been enabled due to the need to listen to
// additional unicast addresses in a device that
// does not implement ndo_set_rx_mode()
// @work_node:		List entry for async netdev_work processing
// @work_tracker:		Refcount tracker for async netdev_work
// @work_pending:		Driver-defined pending netdev_work, passed to
// ndo_work() (see netdev_work_sched())
// @work_core_pending:	Core-defined pending netdev_work (NETDEV_WORK_*)
// @rx_mode_addr_cache:	Recycled snapshot entries for rx_mode work
// @rx_mode_retry_timer:	Timer that re-queues rx_mode work after failure
// @rx_mode_retry_count:	Number of consecutive retries already scheduled
// @uc:			unicast mac addresses
// @mc:			multicast mac addresses
// @dev_addrs:		list of device hw addresses
// @queues_kset:		Group of all Kobjects in the Tx and RX queues
// @promiscuity:		Number of times the NIC is told to work in
// promiscuous mode; if it becomes 0 the NIC will
// exit promiscuous mode
// @allmulti:		Counter, enables or disables allmulticast mode
//
// @vlan_info:	VLAN info
// @dsa_ptr:	dsa specific data
// @tipc_ptr:	TIPC specific data
// @ip_ptr:	IPv4 specific data
// @ip6_ptr:	IPv6 specific data
// @ieee80211_ptr:	IEEE 802.11 specific data, assign before registering
// @ieee802154_ptr: IEEE 802.15.4 low-rate Wireless Personal Area Network
// device struct
// @mpls_ptr:	mpls_dev struct pointer
// @mctp_ptr:	MCTP specific data
// @psp_dev:	PSP crypto device registered for this netdev
//
// @dev_addr:	Hw address (before bcast,
// because most packets are unicast)
//
// @_rx:			Array of RX queues
// @num_rx_queues:		Number of RX queues
// allocated at register_netdev() time
// @real_num_rx_queues: 	Number of RX queues currently active in device
// @xdp_prog:		XDP sockets filter program pointer
//
// @rx_handler:		handler for received packets
// @rx_handler_data: 	XXX: need comments on this one
// @tcx_ingress:		BPF & clsact qdisc specific data for ingress processing
// @ingress_queue:		XXX: need comments on this one
// @nf_hooks_ingress:	netfilter hooks executed for ingress packets
// @broadcast:		hw bcast address
//
// @rx_cpu_rmap:	CPU reverse-mapping for RX completion interrupts,
// indexed by RX queue number. Assigned by driver.
// This must only be set if the ndo_rx_flow_steer
// operation is defined
// @index_hlist:		Device index hash chain
//
// @_tx:			Array of TX queues
// @num_tx_queues:		Number of TX queues allocated at alloc_netdev_mq() time
// @real_num_tx_queues: 	Number of TX queues currently active in device
// @qdisc:			Root qdisc from userspace point of view
// @tx_queue_len:		Max frames per queue allowed
// @tx_global_lock: 	XXX: need comments on this one
// @xdp_bulkq:		XDP device bulk queue
// @xps_maps:		all CPUs/RXQs maps for XPS device
//
// @xps_maps:	XXX: need comments on this one
// @tcx_egress:		BPF & clsact qdisc specific data for egress processing
// @nf_hooks_egress:	netfilter hooks executed for egress packets
// @qdisc_hash:		qdisc hash table
// @watchdog_timeo:	Represents the timeout that is used by
// the watchdog (see dev_watchdog())
// @watchdog_lock:		protect watchdog_ref_held
// @watchdog_ref_held:	True if the watchdog device ref is taken.
// @watchdog_timer:	List of timers
//
// @proto_down_reason:	reason a netdev interface is held down
// @pcpu_refcnt:		Number of references to this device
// @dev_refcnt:		Number of references to this device
// @refcnt_tracker:	Tracker directory for tracked references to this device
// @todo_list:		Delayed register/unregister
// @link_watch_list:	XXX: need comments on this one
//
// @reg_state:		Register/unregister state machine
// @dismantle:		Device is going to be freed
// @needs_free_netdev:	Should unregister perform free_netdev?
// @priv_destructor:	Called from unregister
// @npinfo:		XXX: need comments on this one
// @nd_net:		Network namespace this network device is inside
// protected by @lock
//
// @ml_priv:	Mid-layer private
// @ml_priv_type:  Mid-layer private type
//
// @pcpu_stat_type:	Type of device statistics which the core should
// allocate/free: none, lstats, tstats, dstats. none
// means the driver is handling statistics allocation
// freeing internally.
// @lstats:		Loopback statistics: packets, bytes
// @tstats:		Tunnel statistics: RX/TX packets, RX/TX bytes
// @dstats:		Dummy statistics: RX/TX/drop packets, RX/TX bytes
//
// @garp_port:	GARP
// @mrp_port:	MRP
//
// @dm_private:	Drop monitor private
//
// @dev:		Class/net/name entry
// @sysfs_groups:	Space for optional device, statistics and wireless
// sysfs groups
//
// @sysfs_rx_queue_group:	Space for optional per-rx queue attributes
// @rtnl_link_ops:	Rtnl_link_ops
// @stat_ops:	Optional ops for queue-aware statistics
// @queue_mgmt_ops:	Optional ops for queue management
//
// @gso_max_size:	Maximum size of generic segmentation offload
// @tso_max_size:	Device (as in HW) limit on the max TSO request size
// @gso_max_segs:	Maximum number of segments that can be passed to the
// NIC for GSO
// @tso_max_segs:	Device (as in HW) limit on the max TSO segment count
// @gso_ipv4_max_size:	Maximum size of generic segmentation offload,
// for IPv4.
//
// @dcbnl_ops:	Data Center Bridging netlink ops
// @num_tc:	Number of traffic classes in the net device
// @tc_to_txq:	XXX: need comments on this one
// @prio_tc_map:	XXX: need comments on this one
//
// @fcoe_ddp_xid:	Max exchange id for FCoE LRO by ddp
//
// @priomap:	XXX: need comments on this one
// @link_topo:	Physical link topology tracking attached PHYs
// @phydev:	Physical device may attach itself
// for hardware timestamping
// @sfp_bus:	attached &struct sfp_bus structure.
//
// @qdisc_tx_busylock: lockdep class annotating Qdisc->busylock spinlock
//
// @proto_down:	protocol port state information can be sent to the
// switch driver and used to set the phys state of the
// switch port.
//
// @irq_affinity_auto: driver wants the core to store and re-assign the IRQ
// affinity. Set by netif_enable_irq_affinity(), then
// the driver must create a persistent napi by
// netif_napi_add_config() and finally bind the napi to
// IRQ (via netif_napi_set_irq()).
//
// @rx_cpu_rmap_auto: driver wants the core to manage the ARFS rmap.
// Set by calling netif_enable_cpu_rmap().
//
// @see_all_hwtstamp_requests: device wants to see calls to
// ndo_hwtstamp_set() for all timestamp requests
// regardless of source, even if those aren't
// HWTSTAMP_SOURCE_NETDEV
// @change_proto_down: device supports setting carrier via IFLA_PROTO_DOWN
// @netns_immutable: interface can't change network namespaces
// @fcoe_mtu:	device supports maximum FCoE MTU, 2158 bytes
//
// @net_notifier_list:	List of per-net netdev notifier block
// that follow this device when it is moved
// to another network namespace.
//
// @macsec_ops:    MACsec offloading ops
//
// @udp_tunnel_nic_info:	static structure describing the UDP tunnel
// offload capabilities of the device
// @udp_tunnel_nic:	UDP tunnel offload state
// @ethtool:	ethtool related state
// @xdp_state:		stores info on attached XDP BPF programs
//
// @nested_level:	Used as a parameter of spin_lock_nested() of
// dev->addr_list_lock.
// @unlink_list:	As netif_addr_lock() can be called recursively,
// keep a list of interfaces to be deleted.
// @gro_max_size:	Maximum size of aggregated packet in generic
// receive offload (GRO)
// @gro_ipv4_max_size:	Maximum size of aggregated packet in generic
// receive offload (GRO), for IPv4.
// @xdp_zc_max_segs:	Maximum number of segments supported by AF_XDP
// zero copy driver
//
// @dev_addr_shadow:	Copy of @dev_addr to catch direct writes.
// @linkwatch_dev_tracker:	refcount tracker used by linkwatch.
// @watchdog_dev_tracker:	refcount tracker used by watchdog.
// @dev_registered_tracker:	tracker for reference held while
// registered
// @offload_xstats_l3:	L3 HW stats for this netdevice.
//
// @devlink_port:	Pointer to related devlink port structure.
// Assigned by a driver before netdev registration using
// SET_NETDEV_DEVLINK_PORT macro. This pointer is static
// during the time netdevice is registered.
//
// @dpll_pin: Pointer to the SyncE source pin of a DPLL subsystem,
// where the clock is recovered.
//
// @max_pacing_offload_horizon: max EDT offload horizon in nsec.
// @napi_config: An array of napi_config structures containing per-NAPI
// settings.
// @num_napi_configs:	number of allocated NAPI config structs,
// always >= max(num_rx_queues, num_tx_queues).
// @gro_flush_timeout:	timeout for GRO layer in NAPI
// @napi_defer_hard_irqs:	If not zero, provides a counter that would
// allow to avoid NIC hard IRQ, on busy queues.
//
// @neighbours:	List heads pointing to this device's neighbours'
// dev_list, one per address-family.
// @hwprov: Tracks which PTP performs hardware packet time stamping.
//
// FIXME: cleanup struct net_device such that network protocol info
// moves out.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct net_device {
// Cacheline organization can be found documented in
// Documentation/networking/net_cachelines/net_device.rst.
// Please update the document when adding new fields.
//
// TX read-mostly hotpath
    pub priv_flags:32: c_ulong,
    pub lltx:1: c_ulong,
    pub netmem_tx:2: c_ulong,
    pub netdev_ops: *const net_device_ops,
    pub header_ops: *const header_ops,
    pub _tx: *mut netdev_queue,
    pub gso_partial_features: netdev_features_t,
    pub real_num_tx_queues: c_uint,
    pub gso_max_size: c_uint,
    pub gso_ipv4_max_size: c_uint,
    pub gso_max_segs: u16,
    pub num_tc: i16,
// Note : dev->mtu is often read without holding a lock.
// Writers usually hold RTNL.
// It is recommended to use READ_ONCE() to annotate the reads,
// and to use WRITE_ONCE() to annotate the writes.
//
    pub mtu: c_uint,
    pub needed_headroom: c_ushort,
    pub tc_to_txq: [netdev_tc_txq; TC_MAX_QUEUE],    pub xps_maps: [*mut xps_dev_maps __rcu; XPS_MAPS_MAX],
    pub nf_hooks_egress: *mut nf_hook_entries __rcu,

    pub tcx_egress: *mut bpf_mprog_entry __rcu,

// TXRX read-mostly hotpath
    pub lstats: *mut pcpu_lstats __percpu,
    pub tstats: *mut pcpu_sw_netstats __percpu,
    pub dstats: *mut pcpu_dstats __percpu,
}

// RX read-mostly hotpath

//
// I/O specific fields
// FIXME: Merge these and struct ifmap into one
//
// Some hardware also needs these fields (state,dev_list,
// napi_list,unreg_list,close_list) but they are not
// part of the usual set specified in Space.c.
//

// Read-mostly cache-line for fast-path access
// Stats to monitor link on/off, flapping

// Interface address info.

// Protocol-specific pointers
// @fib_nh_head: nexthops associated with this netdev

//
// Cache lines mostly used on receive path (including eth_type_trans())
//
// Interface address info used in eth_type_trans()

// TCP minimal MSS is 8 (TCP_MIN_GSO_SIZE),
// and shinfo->gso_segs is a 16bit field.
//

//
// Cache lines mostly used on transmit path
//

// These may be needed for future network-power-down code.

// @moving_ns: device is changing netns, protected by @lock
// @rtnl_link_initializing: Device being created, suppress events
// mid-layer private

// for setting kernel sock attribute on TCP connection setup

// TCP minimal MSS is 8 (TCP_MIN_GSO_SIZE),
// and shinfo->gso_segs is a 16bit field.
//

pub const TSO_LEGACY_MAX_SIZE: c_int = 65536;

// priv_flags_slow, ungrouped to save space

// MACsec management functions

// @cfg: net_device queue-related configuration
//
// @cfg_pending: same as @cfg but when device is being actively
// reconfigured includes any changes to the configuration
// requested by the user, but which may or may not be rejected.
//
// protected by rtnl_lock

// @page_pools: page pools created for this netdevice

// @irq_moder: dim parameters used if IS_ENABLED(CONFIG_DIMLIB).
//
// @up: copy of @state's IFF_UP, but safe to read with just @lock.
// May report false negatives while the device is being opened
// or closed (@lock does not protect .ndo_open, or .ndo_close).
//
// @request_ops_lock: request the core to run all @netdev_ops and
// @ethtool_ops under the @lock.
//
// @lock: netdev-scope lock, protects a small selection of fields.
// Should always be taken using netdev_lock() / netdev_unlock() helpers.
// Drivers are free to use it for other protection.
//
// For the drivers that implement shaper or queue API, the scope
// of this lock is expanded to cover most ndo/queue/ethtool/sysfs
// operations. Drivers may opt-in to this behavior by setting
// @request_ops_lock.
//
// @lock protection mixes with rtnl_lock in multiple ways, fields are
// either:
//
// - simply protected by the instance @lock;
//
// - double protected - writers hold both locks, readers hold either;
//
// - ops protected - protected by the lock held around the NDOs
// and other callbacks, that is the instance lock on devices for
// which netdev_need_ops_lock() returns true, otherwise by rtnl_lock;
//
// - double ops protected - always protected by rtnl_lock but for
// devices for which netdev_need_ops_lock() returns true - also
// the instance lock.
//
// Simply protects:
// @gro_flush_timeout, @napi_defer_hard_irqs, @napi_list,
// @net_shaper_hierarchy, @reg_state, @threaded
//
// Double protects:
// @up, @moving_ns, @nd_net, @xdp_features
//
// Ops protects:
// @cfg, @cfg_pending, @ethtool, @hwprov
//
// Double ops protects:
// @real_num_rx_queues, @real_num_tx_queues
//
// Also protects some fields in:
// struct napi_struct, struct netdev_queue, struct netdev_rx_queue
//
// Ordering:
//
// - take after rtnl_lock
//
// - for the case of netdev queue leasing, the netdev-scope lock is
// taken for both the virtual and the physical device; to prevent
// deadlocks, the virtual device's lock must always be acquired
// before the physical device's (see netdev_nl_queue_create_doit)
//

//
// @net_shaper_hierarchy: data tracking the current shaper status
// see include/net/net_shapers.h
//

//
// Driver should use this to assign devlink port instance to a netdevice
// before it registers the netdevice. Therefore devlink_port is static
// during the netdev lifetime after it is registered.
//

pub const NETDEV_ALIGN: c_int = 32;
extern "C" {
    pub fn READ_ONCE(TC_BITMASK]: dev->prio_tc_map[prio &) -> return;
}
extern "C" {
    pub fn netdev_txq_to_tc(dev: *mut net_device, txq: c_uint) -> c_int;
}
extern "C" {
    pub fn netdev_reset_tc(dev: *mut net_device);
}
extern "C" {
    pub fn netdev_set_tc_queue(dev: *mut net_device, tc: u8, count: u16, offset: u16) -> c_int;
}
extern "C" {
    pub fn netdev_set_num_tc(dev: *mut net_device, num_tc: u8) -> c_int;
}
extern "C" {
    pub fn READ_ONCE(_arg: dev->num_tc) -> return;
}

extern "C" {
    pub fn netdev_set_sb_channel(dev: *mut net_device, channel: u16) -> c_int;
}
extern "C" {
    pub fn max_t(_arg: c_int, _arg: -READ_ONCE(dev->num_tc), _arg: 0) -> return;
}
extern "C" {
    pub fn netdev_get_tx_queue(_arg: dev, _arg: skb_get_queue_mapping(skb)) -> return;
}
// returns the headroom that the master device needs to take in account
// when forwarding to this dev
//
// set the device rx headroom to the dev's default
//
// Net namespace inlines
//
extern "C" {
    pub fn read_pnet(_arg: &dev->nd_net) -> return;
}
extern "C" {
    pub fn read_pnet_rcu(_arg: &dev->nd_net) -> return;
}
//
// netdev_priv - access network device private data
// @dev: network device
//
// Get network device private data
//
// netdev_from_priv() - get network device from priv
// @priv: network device private data
//
// Returns: net_device to which @priv belongs
//
extern "C" {
    pub fn container_of(_arg: priv, net_device: struct, _arg: priv) -> return;
}
// Set the sysfs physical device reference for the network logical device
// if set prior to registration will cause a symlink during initialization.
//

// Set the sysfs device type for the network logical device to allow
// fine-grained identification of different network device types. For
// example Ethernet, Wireless LAN, Bluetooth, WiMAX etc.
//

// Additional netdev_lock()-related helpers are in net/netdev_lock.h
extern "C" {
    pub fn netif_napi_set_irq_locked(napi: *mut napi_struct, irq: c_int);
}
// Default NAPI poll() weight
// Device drivers are strongly advised to not use bigger value
//
pub const NAPI_POLL_WEIGHT: c_int = 64;
//
// netif_napi_add() - initialize a NAPI context
// @dev:  network device
// @napi: NAPI context
// @poll: polling function
//
// netif_napi_add() must be used to initialize a NAPI context prior to calling
// *any* of the other NAPI-related functions.
//
// netif_napi_add_config - initialize a NAPI context with persistent config
// @dev: network device
// @napi: NAPI context
// @poll: polling function
// @index: the NAPI index
//
// netif_napi_add_tx() - initialize a NAPI context to be used for Tx only
// @dev:  network device
// @napi: NAPI context
// @poll: polling function
//
// This variant of netif_napi_add() should be used from drivers using NAPI
// to exclusively poll a TX queue.
// This will avoid we add it into napi_hash[], thus polluting this hash table.
//
extern "C" {
    pub fn __netif_napi_del_locked(napi: *mut napi_struct);
}
//
// __netif_napi_del - remove a NAPI context
// @napi: NAPI context
//
// Warning: caller must observe RCU grace period before freeing memory
// containing @napi. Drivers might want to call this helper to combine
// all the needed RCU grace periods into a single one.
//
// netif_napi_del - remove a NAPI context
// @napi: NAPI context
//
// netif_napi_del() removes a NAPI context from the network device NAPI list
//
extern "C" {
    pub fn netif_enable_cpu_rmap(dev: *mut net_device, num_irqs: c_uint) -> c_int;
}
extern "C" {
    pub fn netif_set_affinity_auto(dev: *mut net_device);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct packet_type {
    pub /: *mut *mut __be16 type; / This is really htons(ether_type).,
    pub ignore_outgoing: bool,
    pub /: *mut *mut *mut net_device dev; / NULL is wildcarded here,
    pub dev_tracker: netdevice_tracker,
    pub ): *mut net_device,
    pub ): *mut net_device,
    pub sk): *mut sock,
    pub af_packet_net: *mut net,
    pub af_packet_priv: *mut c_void,
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct offload_callbacks {
    pub features): netdev_features_t,
    pub skb): *mut sk_buff,
    pub nhoff): *mut *mut *mut int (gro_complete)(struct sk_buff skb, int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct packet_offload {
    pub /: *mut *mut __be16 type; / This is really htons(ether_type).,
    pub priority: u16,
    pub callbacks: offload_callbacks,
    pub list: list_head,
}

// often modified stats are per-CPU, other are shared (netdev->stats)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcpu_sw_netstats {
    pub rx_packets: u64_stats_t,
    pub rx_bytes: u64_stats_t,
    pub tx_packets: u64_stats_t,
    pub tx_bytes: u64_stats_t,
    pub syncp: u64_stats_sync,
    pub sizeof(u64)): *mut *mut } __aligned(4,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcpu_dstats {
    pub rx_packets: u64_stats_t,
    pub rx_bytes: u64_stats_t,
    pub tx_packets: u64_stats_t,
    pub tx_bytes: u64_stats_t,
    pub rx_drops: u64_stats_t,
    pub tx_drops: u64_stats_t,
    pub syncp: u64_stats_sync,
    pub sizeof(u64)): *mut *mut } __aligned(8,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcpu_lstats {
    pub packets: u64_stats_t,
    pub bytes: u64_stats_t,
    pub syncp: u64_stats_sync,
    pub sizeof(u64)): *mut *mut } __aligned(2,
    pub bytes): *mut *mut *mut void dev_lstats_read(struct net_device dev, u64 packets, u64,
    pub this_cpu_ptr(dev->tstats): *mut *mut pcpu_sw_netstats tstats =,
    pub len): u64_stats_add(&tstats->rx_bytes,,
    pub this_cpu_ptr(dev->tstats): *mut *mut pcpu_sw_netstats tstats =,
    pub len): u64_stats_add(&tstats->tx_bytes,,
    pub packets): u64_stats_add(&tstats->tx_packets,,
    pub this_cpu_ptr(dev->lstats): *mut *mut pcpu_lstats lstats =,
    pub len): u64_stats_add(&lstats->bytes,,
    pub this_cpu_ptr(dev->dstats): *mut *mut pcpu_dstats dstats =,
    pub len): u64_stats_add(&dstats->rx_bytes,,
    pub this_cpu_ptr(dev->dstats): *mut *mut pcpu_dstats dstats =,
    pub this_cpu_ptr(dev->dstats): *mut *mut pcpu_dstats dstats =,
    pub packets): u64_stats_add(&dstats->rx_drops,,
    pub this_cpu_ptr(dev->dstats): *mut *mut pcpu_dstats dstats =,
    pub len): u64_stats_add(&dstats->tx_bytes,,
    pub this_cpu_ptr(dev->dstats): *mut *mut pcpu_dstats dstats =,

    pub gfp);\: *mut *mut typeof(type) __percpu pcpu_stats = alloc_percpu_gfp(type,,
    pub \: int __cpu;,
    pub \: *mut *mut typeof(type) stat;,
    pub \: stat = per_cpu_ptr(pcpu_stats, __cpu);,
    pub \: u64_stats_init(&stat->syncp);,
    pub \: pcpu_stats;,

    pub type);\: *mut *mut typeof(type) __percpu pcpu_stats = devm_alloc_percpu(dev,,
    pub \: int __cpu;,
    pub \: *mut *mut typeof(type) stat;,
    pub \: stat = per_cpu_ptr(pcpu_stats, __cpu);,
    pub \: u64_stats_init(&stat->syncp);,
    pub \: pcpu_stats;,
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum netdev_lag_tx_type {
    NETDEV_LAG_TX_TYPE_UNKNOWN,
    NETDEV_LAG_TX_TYPE_RANDOM,
    NETDEV_LAG_TX_TYPE_BROADCAST,
    NETDEV_LAG_TX_TYPE_ROUNDROBIN,
    NETDEV_LAG_TX_TYPE_ACTIVEBACKUP,
    NETDEV_LAG_TX_TYPE_HASH,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum netdev_lag_hash {
    NETDEV_LAG_HASH_NONE,
    NETDEV_LAG_HASH_L2,
    NETDEV_LAG_HASH_L34,
    NETDEV_LAG_HASH_L23,
    NETDEV_LAG_HASH_E23,
    NETDEV_LAG_HASH_E34,
    NETDEV_LAG_HASH_VLAN_SRCMAC,
    NETDEV_LAG_HASH_UNKNOWN,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct netdev_lag_upper_info {
    pub tx_type: netdev_lag_tx_type,
    pub hash_type: netdev_lag_hash,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct netdev_lag_lower_state_info {
    pub 1: tx_enabled :,
}

// netdevice notifier chain. Please remember to update netdev_cmd_to_name()
// and the rtnetlink notification exclusion list in rtnetlink_event() when
// adding new types.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum netdev_cmd {
    NETDEV_UP	= 1,	/* For now you can't veto a device up/down */
    NETDEV_DOWN,
    NETDEV_REBOOT,		/* Tell a protocol stack a network interface
    detected a hardware crash and restarted
    - we can use this eg to kick tcp sessions
    once done */
    NETDEV_CHANGE,		/* Notify device state change */
    NETDEV_REGISTER,
    NETDEV_UNREGISTER,
    NETDEV_CHANGEMTU,	/* notify after mtu change happened */
    NETDEV_CHANGEADDR,	/* notify after the address change */
    NETDEV_PRE_CHANGEADDR,	/* notify before the address change */
    NETDEV_GOING_DOWN,
    NETDEV_CHANGENAME,
    NETDEV_FEAT_CHANGE,
    NETDEV_BONDING_FAILOVER,
    NETDEV_PRE_UP,
    NETDEV_PRE_TYPE_CHANGE,
    NETDEV_POST_TYPE_CHANGE,
    NETDEV_POST_INIT,
    NETDEV_PRE_UNINIT,
    NETDEV_RELEASE,
    NETDEV_NOTIFY_PEERS,
    NETDEV_JOIN,
    NETDEV_CHANGEUPPER,
    NETDEV_RESEND_IGMP,
    NETDEV_PRECHANGEMTU,	/* notify before mtu change happened */
    NETDEV_CHANGEINFODATA,
    NETDEV_BONDING_INFO,
    NETDEV_PRECHANGEUPPER,
    NETDEV_CHANGELOWERSTATE,
    NETDEV_UDP_TUNNEL_PUSH_INFO,
    NETDEV_UDP_TUNNEL_DROP_INFO,
    NETDEV_CHANGE_TX_QUEUE_LEN,
    NETDEV_CVLAN_FILTER_PUSH_INFO,
    NETDEV_CVLAN_FILTER_DROP_INFO,
    NETDEV_SVLAN_FILTER_PUSH_INFO,
    NETDEV_SVLAN_FILTER_DROP_INFO,
    NETDEV_OFFLOAD_XSTATS_ENABLE,
    NETDEV_OFFLOAD_XSTATS_DISABLE,
    NETDEV_OFFLOAD_XSTATS_REPORT_USED,
    NETDEV_OFFLOAD_XSTATS_REPORT_DELTA,
    NETDEV_XDP_FEAT_CHANGE,
}

extern "C" {
    pub fn register_netdevice_notifier(nb: *mut notifier_block) -> c_int;
}
extern "C" {
    pub fn unregister_netdevice_notifier(nb: *mut notifier_block) -> c_int;
}
extern "C" {
    pub fn register_netdevice_notifier_net(net: *mut net, nb: *mut notifier_block) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct netdev_notifier_info {
    pub dev: *mut net_device,
    pub extack: *mut netlink_ext_ack,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct netdev_notifier_info_ext {
    pub /: *mut *mut netdev_notifier_info info; / must be first,
    pub mtu: u32,
    pub ext: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct netdev_notifier_change_info {
    pub /: *mut *mut netdev_notifier_info info; / must be first,
    pub flags_changed: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct netdev_notifier_changeupper_info {
    pub /: *mut *mut netdev_notifier_info info; / must be first,
    pub /: *mut *mut *mut net_device upper_dev; / new upper dev,
    pub /: *mut *mut bool master; / is upper dev master,
    pub /: *mut *mut bool linking; / is the notification for link or unlink,
    pub /: *mut *mut *mut void upper_info; / upper dev info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct netdev_notifier_changelowerstate_info {
    pub /: *mut *mut netdev_notifier_info info; / must be first,
    pub /: *mut *mut *mut void lower_state_info; / is lower dev state,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct netdev_notifier_pre_changeaddr_info {
    pub /: *mut *mut netdev_notifier_info info; / must be first,
    pub dev_addr: *const c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum netdev_offload_xstats_type {
    NETDEV_OFFLOAD_XSTATS_TYPE_L3 = 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct netdev_notifier_offload_xstats_info {
    pub /: *mut *mut netdev_notifier_info info; / must be first,
    pub type: netdev_offload_xstats_type,
// NETDEV_OFFLOAD_XSTATS_REPORT_DELTA
    pub report_delta: *mut netdev_notifier_offload_xstats_rd,
// NETDEV_OFFLOAD_XSTATS_REPORT_USED
    pub report_used: *mut netdev_notifier_offload_xstats_ru,
}

extern "C" {
    pub fn call_netdevice_notifiers(val: c_ulong, dev: *mut net_device) -> c_int;
}

extern "C" {
    pub fn dev_add_pack(pt: *mut packet_type);
}
extern "C" {
    pub fn dev_remove_pack(pt: *mut packet_type);
}
extern "C" {
    pub fn __dev_remove_pack(pt: *mut packet_type);
}
extern "C" {
    pub fn dev_add_offload(po: *mut packet_offload);
}
extern "C" {
    pub fn dev_remove_offload(po: *mut packet_offload);
}
extern "C" {
    pub fn dev_get_iflink(dev: *const net_device) -> c_int;
}
extern "C" {
    pub fn dev_fill_metadata_dst(dev: *mut net_device, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn dev_fill_forward_path_release(stack: *mut net_device_path_stack);
}
extern "C" {
    pub fn netdev_name_in_use(net: *mut net, name: *const c_char) -> bool;
}
extern "C" {
    pub fn dev_alloc_name(dev: *mut net_device, name: *const c_char) -> c_int;
}
extern "C" {
    pub fn netif_open(dev: *mut net_device, extack: *mut netlink_ext_ack) -> c_int;
}
extern "C" {
    pub fn dev_open(dev: *mut net_device, extack: *mut netlink_ext_ack) -> c_int;
}
extern "C" {
    pub fn netif_close(dev: *mut net_device);
}
extern "C" {
    pub fn dev_close(dev: *mut net_device);
}
extern "C" {
    pub fn netif_close_many(head: *mut list_head, unlink: bool);
}
extern "C" {
    pub fn netif_disable_lro(dev: *mut net_device);
}
extern "C" {
    pub fn dev_disable_lro(dev: *mut net_device);
}
extern "C" {
    pub fn dev_loopback_xmit(net: *mut net, sk: *mut sock, newskb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn __dev_queue_xmit(skb: *mut sk_buff, sb_dev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn __dev_direct_xmit(skb: *mut sk_buff, queue_id: u16) -> c_int;
}
extern "C" {
    pub fn __dev_queue_xmit(_arg: skb, _arg: NULL) -> return;
}
extern "C" {
    pub fn __dev_queue_xmit(_arg: skb, _arg: sb_dev) -> return;
}
extern "C" {
    pub fn register_netdevice(dev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn unregister_netdevice_queue(dev: *mut net_device, head: *mut list_head);
}
extern "C" {
    pub fn unregister_netdevice_many(head: *mut list_head);
}
extern "C" {
    pub fn unregister_netdevice_queued(dev: *const net_device) -> bool;
}

extern "C" {
    pub fn unregister_netdevice_many_net(net: *mut net);
}
extern "C" {
    pub fn unregister_netdevice_queue_many_net(net: *mut net, head: *mut list_head);
}

extern "C" {
    pub fn netdev_refcnt_read(dev: *const net_device) -> c_int;
}
extern "C" {
    pub fn free_netdev(dev: *mut net_device);
}
extern "C" {
    pub fn netdev_copy_name(dev: *mut net_device, name: *mut c_char);
}
// ll_header must have at least hard_header_len allocated
#[repr(C)]
#[derive(Copy, Clone)]
pub struct numa_drop_counters {
    pub ____cacheline_aligned_in_smp: atomic_t drops0,
    pub ____cacheline_aligned_in_smp: atomic_t drops1,
}

extern "C" {
    pub fn atomic_read(atomic_read(&ndc->drops1: &ndc->drops0) +) -> return;
}
//
// Incoming packets are placed on per-CPU queues
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct softnet_data {
    pub poll_list: list_head,
    pub process_queue: sk_buff_head,
    pub process_queue_bh_lock: local_lock_t,
// stats
    pub processed: c_uint,
    pub time_squeeze: c_uint,

    pub rps_ipi_list: *mut softnet_data,

    pub received_rps: c_uint,
    pub in_net_rx_action: bool,
    pub in_napi_threaded_poll: bool,

    pub flow_limit: *mut sd_flow_limit __rcu,

    pub output_queue: *mut Qdisc,
    pub output_queue_tailp: *mut Qdisc,
    pub completion_queue: *mut sk_buff,

    pub xfrm_backlog: sk_buff_head,

// written and read only by owning cpu:
    pub xmit: netdev_xmit,

// input_queue_head should be written by cpu owning this struct,
// and only read by other cpus. Worth using a cache line.
//
    pub ____cacheline_aligned_in_smp: unsigned int input_queue_head,
// Elements below can be accessed between CPUs for RPS/RFS
    pub ____cacheline_aligned_in_smp: call_single_data_t csd,
    pub rps_ipi_next: *mut softnet_data,
    pub cpu: c_uint,
// We force a cacheline alignment from here, to hold together
// input_queue_tail, input_pkt_queue and backlog.state.
// We add holes so that backlog.state is the last field
// of this cache line.
//
    pub ____cacheline_aligned_in_smp: long pad[3],
    pub input_queue_tail: c_uint,

    pub input_pkt_queue: sk_buff_head,
    pub backlog: napi_struct,
    pub drop_counters: numa_drop_counters,
    pub ____cacheline_aligned_in_smp: int defer_ipi_scheduled,
    pub defer_csd: call_single_data_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct page_pool_bh {
    pub pool: *mut page_pool,
    pub bh_lock: local_lock_t,
}

pub const XMIT_RECURSION_LIMIT: c_int = 8;

extern "C" {
    pub fn this_cpu_read(_arg: softnet_data.xmit.recursion) -> return;
}

extern "C" {
    pub fn unlikely(XMIT_RECURSION_LIMIT: current->net_xmit.recursion >) -> return;
}

extern "C" {
    pub fn __netif_schedule(q: *mut Qdisc);
}
extern "C" {
    pub fn netif_schedule_queue(txq: *mut netdev_queue);
}
//
// netif_start_queue - allow transmit
// @dev: network device
//
// Allow upper layers to call the device hard_start_xmit routine.
//
extern "C" {
    pub fn netif_tx_wake_queue(dev_queue: *mut netdev_queue);
}
//
// netif_wake_queue - restart transmit
// @dev: network device
//
// Allow upper layers to call the device hard_start_xmit routine.
// Used for flow control when transmit resources are available.
//
// Paired with READ_ONCE() from dev_watchdog()
// This barrier is paired with smp_mb() from dev_watchdog()
// Must be an atomic op see netif_txq_try_stop()
//
// netif_stop_queue - stop transmitted packets
// @dev: network device
//
// Stop upper layers calling the device hard_start_xmit routine.
// Used for flow control when transmit resources are unavailable.
//
extern "C" {
    pub fn netif_tx_stop_all_queues(dev: *mut net_device);
}
extern "C" {
    pub fn test_bit(_arg: __QUEUE_STATE_DRV_XOFF, _arg: &dev_queue->state) -> return;
}
//
// netif_queue_stopped - test if transmit queue is flowblocked
// @dev: network device
//
// Test if transmit queue on device is currently unable to send.
//
extern "C" {
    pub fn netif_tx_queue_stopped(_arg: netdev_get_tx_queue(dev, _arg: 0)) -> return;
}
//
// netdev_queue_set_dql_min_limit - set dql minimum limit
// @dev_queue: pointer to transmit queue
// @min_limit: dql minimum limit
//
// Forces xmit_more() to return true until the minimum threshold
// defined by @min_limit is reached (or until the tx queue is
// empty). Warning: to be use with care, misuse will impact the
// latency.
//

// Non-BQL migrated drivers will return 0, too.
extern "C" {
    pub fn dql_avail(_arg: &txq->dql) -> return;
}

//
// netdev_txq_bql_enqueue_prefetchw - prefetch bql data for write
// @dev_queue: pointer to transmit queue
//
// BQL enabled drivers might use this helper in their ndo_start_xmit(),
// to give appropriate hint to the CPU.
//

//
// netdev_txq_bql_complete_prefetchw - prefetch bql data for write
// @dev_queue: pointer to transmit queue
//
// BQL enabled drivers might use this helper in their TX completion path,
// to give appropriate hint to the CPU.
//

//
// netdev_tx_sent_queue - report the number of bytes queued to a given tx queue
// @dev_queue: network device queue
// @bytes: number of bytes queued to the device queue
//
// Report the number of bytes queued for sending/completion to the network
// device hardware queue. @bytes should be a good approximation and should
// exactly match netdev_completed_queue() @bytes.
// This is typically called once per packet, from ndo_start_xmit().
//

// Paired with READ_ONCE() from dev_watchdog()
// This barrier is paired with smp_mb() from dev_watchdog()
//
// The XOFF flag must be set before checking the dql_avail below,
// because in netdev_tx_completed_queue we update the dql_completed
// before checking the XOFF flag.
//
// check again in case another CPU has just made room avail

// Variant of netdev_tx_sent_queue() for drivers that are aware
// that they should not test BQL status themselves.
// We do want to change __QUEUE_STATE_STACK_XOFF only for the last
// skb of a batch.
// Returns true if the doorbell must be used to kick the NIC.
//

extern "C" {
    pub fn netif_tx_queue_stopped(_arg: dev_queue) -> return;
}
//
// netdev_sent_queue - report the number of bytes queued to hardware
// @dev: network device
// @bytes: number of bytes queued to the hardware device queue
//
// Report the number of bytes queued for sending/completion to the network
// device hardware queue#0. @bytes should be a good approximation and should
// exactly match netdev_completed_queue() @bytes.
// This is typically called once per packet, from ndo_start_xmit().
//
// netdev_tx_completed_queue - report number of packets/bytes at TX completion.
// @dev_queue: network device queue
// @pkts: number of packets (currently ignored)
// @bytes: number of bytes dequeued from the device queue
//
// Must be called at most once per TX completion round (and not per
// individual packet), so that BQL can adjust its limits appropriately.
//

//
// Without the memory barrier there is a small possibility that
// netdev_tx_sent_queue will miss the update and cause the queue to
// be stopped forever
//

//
// netdev_completed_queue - report bytes and packets completed by device
// @dev: network device
// @pkts: actual number of packets sent over the medium
// @bytes: actual number of bytes sent over the medium
//
// Report the number of bytes and packets transmitted by the network device
// hardware queue over the physical medium, @bytes must exactly match the
// @bytes amount passed to netdev_sent_queue()
//

//
// netdev_tx_reset_subqueue - reset the BQL stats and state of a netdev queue
// @dev: network device
// @qid: stack index of the queue to reset
//
// netdev_reset_queue - reset the packets and bytes count of a network device
// @dev_queue: network device
//
// Reset the bytes and packet count of a network device and clear the
// software flow control OFF bit for this network device
//
// netdev_cap_txqueue - check if selected tx queue exceeds device queues
// @dev: network device
// @queue_index: given tx queue index
//
// Returns 0 if given tx queue index >= number of device tx queues,
// otherwise returns the originally passed tx queue index.
//
// netif_running - test if up
// @dev: network device
//
// Test if the device has been brought up.
//
extern "C" {
    pub fn test_bit(_arg: __LINK_STATE_START, _arg: &dev->state) -> return;
}
//
// Routines to manage the subqueues on a device.  We only need start,
// stop, and a check if it's stopped.  All other device management is
// done at the overall netdevice level.
// Also test the device if we're multiqueue.
//
// netif_start_subqueue - allow sending packets on subqueue
// @dev: network device
// @queue_index: sub queue index
//
// Start individual transmit queue of a device with multiple transmit queues.
//
// netif_stop_subqueue - stop sending packets on subqueue
// @dev: network device
// @queue_index: sub queue index
//
// Stop individual transmit queue of a device with multiple transmit queues.
//
// __netif_subqueue_stopped - test status of subqueue
// @dev: network device
// @queue_index: sub queue index
//
// Check individual transmit queue of a device with multiple transmit queues.
//
extern "C" {
    pub fn netif_tx_queue_stopped(_arg: txq) -> return;
}
//
// netif_subqueue_stopped - test status of subqueue
// @dev: network device
// @skb: sub queue buffer pointer
//
// Check individual transmit queue of a device with multiple transmit queues.
//
extern "C" {
    pub fn __netif_subqueue_stopped(_arg: dev, _arg: skb_get_queue_mapping(skb)) -> return;
}
//
// netif_wake_subqueue - allow sending packets on subqueue
// @dev: network device
// @queue_index: sub queue index
//
// Resume individual transmit queue of a device with multiple transmit queues.
//

//
// netif_attr_test_mask - Test a CPU or Rx queue set in a mask
// @j: CPU/Rx queue index
// @mask: bitmask of all cpus/rx queues
// @nr_bits: number of bits in the bitmask
//
// Test if a CPU or Rx queue index is set in a mask of all CPU/Rx queues.
//
extern "C" {
    pub fn test_bit(_arg: j, _arg: mask) -> return;
}
//
// netif_attr_test_online - Test for online CPU/Rx queue
// @j: CPU/Rx queue index
// @online_mask: bitmask for CPUs/Rx queues that are online
// @nr_bits: number of bits in the bitmask
//
// Returns: true if a CPU/Rx queue is online.
//
extern "C" {
    pub fn test_bit(_arg: j, _arg: online_mask) -> return;
}
//
// netif_attrmask_next - get the next CPU/Rx queue in a cpu/Rx queues mask
// @n: CPU/Rx queue index
// @srcp: the cpumask/Rx queue mask pointer
// @nr_bits: number of bits in the bitmask
//
// Returns: next (after n) CPU/Rx queue index in the mask;
// >= nr_bits if no further CPUs/Rx queues set.
//
// -1 is a legal arg here.
extern "C" {
    pub fn find_next_bit(_arg: srcp, _arg: nr_bits, 1: n +) -> return;
}
//
// netif_attrmask_next_and - get the next CPU/Rx queue in \*src1p & \*src2p
// @n: CPU/Rx queue index
// @src1p: the first CPUs/Rx queues mask pointer
// @src2p: the second CPUs/Rx queues mask pointer
// @nr_bits: number of bits in the bitmask
//
// Returns: next (after n) CPU/Rx queue index set in both masks;
// >= nr_bits if no further CPUs/Rx queues set in both.
//
// -1 is a legal arg here.
extern "C" {
    pub fn find_next_and_bit(_arg: src1p, _arg: src2p, _arg: nr_bits, 1: n +) -> return;
}
extern "C" {
    pub fn find_next_bit(_arg: src1p, _arg: nr_bits, 1: n +) -> return;
}
extern "C" {
    pub fn find_next_bit(_arg: src2p, _arg: nr_bits, 1: n +) -> return;
}

//
// netif_is_multiqueue - test if device has multiple transmit queues
// @dev: network device
//
// Check if device has multiple transmit queues
//
extern "C" {
    pub fn netif_set_real_num_tx_queues(dev: *mut net_device, txq: c_uint) -> c_int;
}
extern "C" {
    pub fn netif_set_real_num_rx_queues(dev: *mut net_device, rxq: c_uint) -> c_int;
}
extern "C" {
    pub fn netif_get_num_default_rss_queues() -> c_int;
}
extern "C" {
    pub fn dev_kfree_skb_irq_reason(skb: *mut sk_buff, reason: skb_drop_reason);
}
extern "C" {
    pub fn dev_kfree_skb_any_reason(skb: *mut sk_buff, reason: skb_drop_reason);
}
//
// It is not allowed to call kfree_skb() or consume_skb() from hardware
// interrupt context or with hardware interrupts being disabled.
// (in_hardirq() || irqs_disabled())
//
// We provide four helpers that can be used in following contexts :
//
// dev_kfree_skb_irq(skb) when caller drops a packet from irq context,
// replacing kfree_skb(skb)
//
// dev_consume_skb_irq(skb) when caller consumes a packet from irq context.
// Typically used in place of consume_skb(skb) in TX completion path
//
// dev_kfree_skb_any(skb) when caller doesn't know its current irq context,
// replacing kfree_skb(skb)
//
// dev_consume_skb_any(skb) when caller doesn't know its current irq context,
// and consumed a packet. Used in place of consume_skb(skb)
//
extern "C" {
    pub fn generic_xdp_tx(skb: *mut sk_buff, xdp_prog: *const bpf_prog);
}
extern "C" {
    pub fn do_xdp_generic(xdp_prog: *const bpf_prog, pskb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn netif_rx(skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn __netif_rx(skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn netif_receive_skb(skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn netif_receive_skb_core(skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn netif_receive_skb_list_internal(head: *mut list_head);
}
extern "C" {
    pub fn netif_receive_skb_list(head: *mut list_head);
}
extern "C" {
    pub fn gro_receive_skb(gro: *mut gro_node, skb: *mut sk_buff) -> gro_result_t;
}
extern "C" {
    pub fn gro_receive_skb(_arg: &napi->gro, _arg: skb) -> return;
}
extern "C" {
    pub fn napi_gro_frags(napi: *mut napi_struct) -> gro_result_t;
}
extern "C" {
    pub fn netdev_is_rx_handler_busy(dev: *mut net_device) -> bool;
}
extern "C" {
    pub fn netdev_rx_handler_unregister(dev: *mut net_device);
}
extern "C" {
    pub fn dev_valid_name(name: *const c_char) -> bool;
}
extern "C" {
    pub fn get_user_ifreq(ifr: *mut ifreq, ifrdata: *mut void __user, arg: *mut void __user) -> c_int;
}
extern "C" {
    pub fn put_user_ifreq(ifr: *mut ifreq, arg: *mut void __user) -> c_int;
}
extern "C" {
    pub fn dev_ifconf(net: *mut net, ifc: *mut ifconf __user) -> c_int;
}
extern "C" {
    pub fn dev_ethtool(net: *mut net, ifr: *mut ifreq, userdata: *mut void __user) -> c_int;
}
extern "C" {
    pub fn netif_get_flags(dev: *const net_device) -> c_uint;
}
extern "C" {
    pub fn netif_set_alias(dev: *mut net_device, alias: *const c_char, len: usize) -> c_int;
}
extern "C" {
    pub fn dev_set_alias(: *mut net_device, : *const c_char, _arg: usize) -> c_int;
}
extern "C" {
    pub fn dev_get_alias(: *const net_device, : *mut c_char, _arg: usize) -> c_int;
}
extern "C" {
    pub fn __netif_set_mtu(dev: *mut net_device, new_mtu: c_int) -> c_int;
}
extern "C" {
    pub fn netif_set_mtu(dev: *mut net_device, new_mtu: c_int) -> c_int;
}
extern "C" {
    pub fn dev_set_mtu(: *mut net_device, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn netif_get_mac_address(sa: *mut sockaddr, net: *mut net, dev_name: *mut c_char) -> c_int;
}
extern "C" {
    pub fn netdev_port_same_parent_id(a: *mut net_device, b: *mut net_device) -> bool;
}
extern "C" {
    pub fn bpf_xdp_link_attach(attr: *const bpf_attr, prog: *mut bpf_prog) -> c_int;
}
extern "C" {
    pub fn dev_xdp_prog_count(dev: *mut net_device) -> u8;
}
extern "C" {
    pub fn netif_xdp_propagate(dev: *mut net_device, bpf: *mut netdev_bpf) -> c_int;
}
extern "C" {
    pub fn dev_xdp_propagate(dev: *mut net_device, bpf: *mut netdev_bpf) -> c_int;
}
extern "C" {
    pub fn dev_xdp_sb_prog_count(dev: *mut net_device) -> u8;
}
extern "C" {
    pub fn dev_xdp_prog_id(dev: *mut net_device, mode: bpf_xdp_mode) -> u32;
}
extern "C" {
    pub fn dev_get_min_mp_channel_count(dev: *const net_device) -> u32;
}
extern "C" {
    pub fn __dev_forward_skb(dev: *mut net_device, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn dev_forward_skb(dev: *mut net_device, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn dev_forward_skb_nomtu(dev: *mut net_device, skb: *mut sk_buff) -> c_int;
}
// if TSO is enabled, we don't care about the length as the packet
// could be forwarded without being segmented before
//
extern "C" {
    pub fn netdev_core_stats_inc(dev: *mut net_device, offset: u32);
}

extern "C" {
    pub fn dev_nit_active_rcu(dev: *const net_device) -> bool;
}
extern "C" {
    pub fn dev_queue_xmit_nit(skb: *mut sk_buff, dev: *mut net_device);
}

// netdev_tracker_alloc() can upgrade a prior untracked reference
// taken by dev_get_by_name()/dev_get_by_index() to a tracked one.
//

//
// dev_hold - get reference to device
// @dev: network device
//
// Hold reference to device to keep it from being freed.
// Try using netdev_hold() instead.
//
// dev_put - release reference to device
// @dev: network device
//
// Release reference to device to allow it to be freed.
// Try using netdev_put() instead.
//
// Carrier loss detection, dial on demand. The functions netif_carrier_on
// and _off may be called from IRQ context, but it is caller
// who is responsible for serialization of these calls.
//
// The name carrier is inappropriate, these functions should really be
// called netif_lowerlayer_*() because they represent the state of any
// kind of lower layer not just hardware media.
//
extern "C" {
    pub fn linkwatch_fire_event(dev: *mut net_device);
}
//
// linkwatch_sync_dev - sync linkwatch for the given device
// @dev: network device to sync linkwatch for
//
// Sync linkwatch for the given device, removing it from the
// pending work list (if queued).
//
extern "C" {
    pub fn linkwatch_sync_dev(dev: *mut net_device);
}
extern "C" {
    pub fn __linkwatch_sync_dev(dev: *mut net_device);
}
//
// netif_carrier_ok - test if carrier present
// @dev: network device
//
// Check if carrier is present on device
//
extern "C" {
    pub fn dev_trans_start(dev: *mut net_device) -> c_ulong;
}
extern "C" {
    pub fn netdev_watchdog_up(dev: *mut net_device);
}
extern "C" {
    pub fn netif_carrier_on(dev: *mut net_device);
}
extern "C" {
    pub fn netif_carrier_off(dev: *mut net_device);
}
extern "C" {
    pub fn netif_carrier_event(dev: *mut net_device);
}
//
// netif_dormant_on - mark device as dormant.
// @dev: network device
//
// Mark device as dormant (as per RFC2863).
//
// The dormant state indicates that the relevant interface is not
// actually in a condition to pass packets (i.e., it is not 'up') but is
// in a "pending" state, waiting for some external event.  For "on-
// demand" interfaces, this new state identifies the situation where the
// interface is waiting for events to place it in the up state.
//
// netif_dormant_off - set device as not dormant.
// @dev: network device
//
// Device is not in dormant state.
//
// netif_dormant - test if device is dormant
// @dev: network device
//
// Check if device is dormant.
//
extern "C" {
    pub fn test_bit(_arg: __LINK_STATE_DORMANT, _arg: &dev->state) -> return;
}
//
// netif_testing_on - mark device as under test.
// @dev: network device
//
// Mark device as under test (as per RFC2863).
//
// The testing state indicates that some test(s) must be performed on
// the interface. After completion, of the test, the interface state
// will change to up, dormant, or down, as appropriate.
//
// netif_testing_off - set device as not under test.
// @dev: network device
//
// Device is not in testing state.
//
// netif_testing - test if device is under test
// @dev: network device
//
// Check if device is under test
//
extern "C" {
    pub fn test_bit(_arg: __LINK_STATE_TESTING, _arg: &dev->state) -> return;
}
//
// netif_oper_up - test if device is operational
// @dev: network device
//
// Check if carrier is operational
//
// netif_device_present - is device available or removed
// @dev: network device
//
// Check if device has not been removed from system.
//
extern "C" {
    pub fn test_bit(_arg: __LINK_STATE_PRESENT, _arg: &dev->state) -> return;
}
extern "C" {
    pub fn netif_device_detach(dev: *mut net_device);
}
extern "C" {
    pub fn netif_device_attach(dev: *mut net_device);
}
//
// Network interface message level settings
//
// When you add a new bit above, update netif_msg_class_names array
// in net/ethtool/common.c
//
// Both ethtool_ops interface and internal driver implementation use u32

// use default
// set low N bits
// Pairs with READ_ONCE() in netif_tx_owned()
//
// txq->trans_start can be read locklessly from dev_watchdog()
//
// legacy drivers only, netdev_start_xmit() sets txq->trans_start
//
// netif_tx_lock - grab network device transmit lock
// @dev: network device
//
// Get network device transmit lock
//
extern "C" {
    pub fn netif_tx_lock(dev: *mut net_device);
}
extern "C" {
    pub fn netif_tx_unlock(dev: *mut net_device);
}

// Other cpus might concurrently change txq->xmit_lock_owner
// to -1 or to their cpu id, but not to our id.
//

//
// dev_addrs walker. Should be used only for read access. Call with
// rcu_read_lock held.
//

// These functions live elsewhere (drivers/net/net_init.c, but related)
extern "C" {
    pub fn ether_setup(dev: *mut net_device);
}
// Allocate dummy net_device
// Support for loadable net-drivers

extern "C" {
    pub fn register_netdev(dev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn unregister_netdev(dev: *mut net_device);
}
extern "C" {
    pub fn devm_register_netdev(dev: *mut device, ndev: *mut net_device) -> c_int;
}
// General hardware address lists handling functions
extern "C" {
    pub fn __hw_addr_init(list: *mut netdev_hw_addr_list);
}
extern "C" {
    pub fn __hw_addr_flush(list: *mut netdev_hw_addr_list);
}
// Functions used for device addresses handling
// Functions used for unicast addresses handling
extern "C" {
    pub fn dev_uc_add(dev: *mut net_device, addr: *const c_uchar) -> c_int;
}
extern "C" {
    pub fn dev_uc_add_excl(dev: *mut net_device, addr: *const c_uchar) -> c_int;
}
extern "C" {
    pub fn dev_uc_del(dev: *mut net_device, addr: *const c_uchar) -> c_int;
}
extern "C" {
    pub fn dev_uc_sync(to: *mut net_device, from: *mut net_device) -> c_int;
}
extern "C" {
    pub fn dev_uc_sync_multiple(to: *mut net_device, from: *mut net_device) -> c_int;
}
extern "C" {
    pub fn dev_uc_unsync(to: *mut net_device, from: *mut net_device);
}
extern "C" {
    pub fn dev_uc_flush(dev: *mut net_device);
}
extern "C" {
    pub fn dev_uc_init(dev: *mut net_device);
}
//
// __dev_uc_sync - Synchronize device's unicast list
// @dev:  device to sync
// @sync: function to call if address should be added
// @unsync: function to call if address should be removed
//
// Add newly added addresses to the interface, and release
// addresses that have been deleted.
//
extern "C" {
    pub fn __hw_addr_sync_dev(_arg: &dev->uc, _arg: dev, _arg: sync, _arg: unsync) -> return;
}
//
// __dev_uc_unsync - Remove synchronized addresses from device
// @dev:  device to sync
// @unsync: function to call if address should be removed
//
// Remove all addresses that were added to the device by dev_uc_sync().
//
// Functions used for multicast addresses handling
extern "C" {
    pub fn dev_mc_add(dev: *mut net_device, addr: *const c_uchar) -> c_int;
}
extern "C" {
    pub fn dev_mc_add_global(dev: *mut net_device, addr: *const c_uchar) -> c_int;
}
extern "C" {
    pub fn dev_mc_add_excl(dev: *mut net_device, addr: *const c_uchar) -> c_int;
}
extern "C" {
    pub fn dev_mc_del(dev: *mut net_device, addr: *const c_uchar) -> c_int;
}
extern "C" {
    pub fn dev_mc_del_global(dev: *mut net_device, addr: *const c_uchar) -> c_int;
}
extern "C" {
    pub fn dev_mc_sync(to: *mut net_device, from: *mut net_device) -> c_int;
}
extern "C" {
    pub fn dev_mc_sync_multiple(to: *mut net_device, from: *mut net_device) -> c_int;
}
extern "C" {
    pub fn dev_mc_unsync(to: *mut net_device, from: *mut net_device);
}
extern "C" {
    pub fn dev_mc_flush(dev: *mut net_device);
}
extern "C" {
    pub fn dev_mc_init(dev: *mut net_device);
}
//
// __dev_mc_sync - Synchronize device's multicast list
// @dev:  device to sync
// @sync: function to call if address should be added
// @unsync: function to call if address should be removed
//
// Add newly added addresses to the interface, and release
// addresses that have been deleted.
//
extern "C" {
    pub fn __hw_addr_sync_dev(_arg: &dev->mc, _arg: dev, _arg: sync, _arg: unsync) -> return;
}
//
// __dev_mc_unsync - Remove synchronized addresses from device
// @dev:  device to sync
// @unsync: function to call if address should be removed
//
// Remove all addresses that were added to the device by dev_mc_sync().
//
// Functions used for secondary unicast and multicast support
extern "C" {
    pub fn dev_set_rx_mode(dev: *mut net_device);
}
extern "C" {
    pub fn netif_rx_mode_schedule_retry(dev: *mut net_device);
}
extern "C" {
    pub fn netif_set_promiscuity(dev: *mut net_device, inc: c_int) -> c_int;
}
extern "C" {
    pub fn dev_set_promiscuity(dev: *mut net_device, inc: c_int) -> c_int;
}
extern "C" {
    pub fn netif_set_allmulti(dev: *mut net_device, inc: c_int, notify: bool) -> c_int;
}
extern "C" {
    pub fn dev_set_allmulti(dev: *mut net_device, inc: c_int) -> c_int;
}
extern "C" {
    pub fn netif_state_change(dev: *mut net_device);
}
extern "C" {
    pub fn netdev_state_change(dev: *mut net_device);
}
extern "C" {
    pub fn __netdev_notify_peers(dev: *mut net_device);
}
extern "C" {
    pub fn netdev_notify_peers(dev: *mut net_device);
}
extern "C" {
    pub fn netdev_features_change(dev: *mut net_device);
}
// Load a device via the kmod
extern "C" {
    pub fn dev_load(net: *mut net, name: *const c_char);
}
extern "C" {
    pub fn dev_get_tstats64(dev: *mut net_device, s: *mut rtnl_link_stats64);
}
extern "C" {
    pub fn netdev_work_sched(dev: *mut net_device, events: c_ulong);
}
extern "C" {
    pub fn netdev_work_cancel(dev: *mut net_device, mask: c_ulong) -> c_ulong;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct netdev_nested_priv {
    pub flags: c_uchar,
    pub data: *mut c_void,
}

extern "C" {
    pub fn netdev_has_upper_dev(dev: *mut net_device, upper_dev: *mut net_device) -> bool;
}
// iterate through upper list, must be called under RCU read lock

extern "C" {
    pub fn netdev_has_any_upper_dev(dev: *mut net_device) -> bool;
}

extern "C" {
    pub fn netdev_adjacent_rename_links(dev: *mut net_device, oldname: *mut c_char);
}
pub const NETDEV_RSS_KEY_LEN: c_int = 256;
extern "C" {
    pub fn netdev_rss_key_fill(buffer: *mut c_void, len: usize);
}
extern "C" {
    pub fn skb_checksum_help(skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn skb_crc32c_csum_help(skb: *mut sk_buff) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct netdev_bonding_info {
    pub slave: ifslave,
    pub master: ifbond,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct netdev_notifier_bonding_info {
    pub /: *mut *mut netdev_notifier_info info; / must be first,
    pub bonding_info: netdev_bonding_info,
}

extern "C" {
    pub fn ethtool_notify(dev: *mut net_device, cmd: c_uint);
}

extern "C" {
    pub fn skb_network_protocol(skb: *mut sk_buff, depth: *mut c_int) -> __be16;
}
// Assume this is an IP checksum (not SCTP CRC)
// Can checksum everything

extern "C" {
    pub fn netdev_rx_csum_fault(dev: *mut net_device, skb: *mut sk_buff);
}

// rx skb timestamps
extern "C" {
    pub fn net_enable_timestamp();
}
extern "C" {
    pub fn net_disable_timestamp();
}

extern "C" {
    pub fn __this_cpu_read(_arg: softnet_data.xmit.more) -> return;
}

// Allow TSO being used on stacked device :
// Performing the GSO segmentation before last device
// is a performance improvement.
//
extern "C" {
    pub fn __netdev_update_features(dev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn netdev_update_features(dev: *mut net_device);
}
extern "C" {
    pub fn netdev_change_features(dev: *mut net_device);
}
extern "C" {
    pub fn netdev_compute_master_upper_features(dev: *mut net_device, update_header: bool);
}
extern "C" {
    pub fn netif_skb_features(skb: *mut sk_buff) -> netdev_features_t;
}
extern "C" {
    pub fn skb_warn_bad_offload(skb: *const sk_buff);
}
// check flags correspondence
extern "C" {
    pub fn netif_set_tso_max_size(dev: *mut net_device, size: c_uint);
}
extern "C" {
    pub fn netif_set_tso_max_segs(dev: *mut net_device, segs: c_uint);
}
// pairs with WRITE_ONCE() in netif_set_gro(_ipv4)_max_size()
// pairs with WRITE_ONCE() in netif_set_gso(_ipv4)_max_size()

extern "C" {
    pub fn netif_is_bridge_master(netif_is_ovs_master(dev: dev) ||) -> return;
}
extern "C" {
    pub fn netif_is_bridge_port(netif_is_ovs_port(dev: dev) ||) -> return;
}
extern "C" {
    pub fn netif_is_bond_master(netif_is_team_master(dev: dev) ||) -> return;
}
extern "C" {
    pub fn netif_is_bond_slave(netif_is_team_port(dev: dev) ||) -> return;
}
extern "C" {
    pub fn netif_is_rxfh_configured(dev: *const net_device) -> bool;
}
// This device needs to keep skb dst for qdisc enqueue or ndo_start_xmit()
// return true if dev can't cope with mtu frames that need vlan tag insertion
// TODO: reserve and use an additional IFF bit, if we get more users
extern "C" {
    pub fn netif_is_macsec(_arg: dev) -> return;
}
// Logging, debugging and troubleshooting/diagnostic helpers.
// netdev_printk helpers, similar to dev_printk

//
// netdev_WARN() acts like dev_printk(), but with the key difference
// of using a WARN/WARN_ON to get the message out, including the
// file/line information and a backtrace.
//

//
// The list of packet types we will receive (as opposed to discard)
// and the routines to invoke.
//
// Why 16. Because with 16 the only overlap we get on a hash of the
// low nibble of the protocol value is RARP/SNAP/X.25.
//
// 0800	IP
// 0001	802.3
// 0002	AX.25
// 0004	802.2
// 8035	RARP
// 0005	SNAP
// 0805	X.25
// 0806	ARP
// 8137	IPX
// 0009	Localtalk
// 86DD	IPv6
//

// Note: Avoid these macros in fast path, prefer per-cpu or per-queue counters.

