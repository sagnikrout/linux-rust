//! Automatically rewritten from C Header to Rust Module
//! Source: tools/include/uapi/linux/if_link.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note

// This struct should be in sync with struct rtnl_link_stats64
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtnl_link_stats {
    pub rx_packets: __u32,
    pub tx_packets: __u32,
    pub rx_bytes: __u32,
    pub tx_bytes: __u32,
    pub rx_errors: __u32,
    pub tx_errors: __u32,
    pub rx_dropped: __u32,
    pub tx_dropped: __u32,
    pub multicast: __u32,
    pub collisions: __u32,
// detailed rx_errors:
    pub rx_length_errors: __u32,
    pub rx_over_errors: __u32,
    pub rx_crc_errors: __u32,
    pub rx_frame_errors: __u32,
    pub rx_fifo_errors: __u32,
    pub rx_missed_errors: __u32,
// detailed tx_errors
    pub tx_aborted_errors: __u32,
    pub tx_carrier_errors: __u32,
    pub tx_fifo_errors: __u32,
    pub tx_heartbeat_errors: __u32,
    pub tx_window_errors: __u32,
// for cslip etc
    pub rx_compressed: __u32,
    pub tx_compressed: __u32,
    pub rx_nohandler: __u32,
}

//
// struct rtnl_link_stats64 - The main device statistics structure.
//
// @rx_packets: Number of good packets received by the interface.
// For hardware interfaces counts all good packets received from the device
// by the host, including packets which host had to drop at various stages
// of processing (even in the driver).
//
// @tx_packets: Number of packets successfully transmitted.
// For hardware interfaces counts packets which host was able to successfully
// hand over to the device, which does not necessarily mean that packets
// had been successfully transmitted out of the device, only that device
// acknowledged it copied them out of host memory.
//
// @rx_bytes: Number of good received bytes, corresponding to @rx_packets.
//
// For IEEE 802.3 devices should count the length of Ethernet Frames
// excluding the FCS.
//
// @tx_bytes: Number of good transmitted bytes, corresponding to @tx_packets.
//
// For IEEE 802.3 devices should count the length of Ethernet Frames
// excluding the FCS.
//
// @rx_errors: Total number of bad packets received on this network device.
// This counter must include events counted by @rx_length_errors,
// @rx_crc_errors, @rx_frame_errors and other errors not otherwise
// counted.
//
// @tx_errors: Total number of transmit problems.
// This counter must include events counter by @tx_aborted_errors,
// @tx_carrier_errors, @tx_fifo_errors, @tx_heartbeat_errors,
// @tx_window_errors and other errors not otherwise counted.
//
// @rx_dropped: Number of packets received but not processed,
// e.g. due to lack of resources or unsupported protocol.
// For hardware interfaces this counter may include packets discarded
// due to L2 address filtering but should not include packets dropped
// by the device due to buffer exhaustion which are counted separately in
// @rx_missed_errors (since procfs folds those two counters together).
//
// @tx_dropped: Number of packets dropped on their way to transmission,
// e.g. due to lack of resources.
//
// @multicast: Multicast packets received.
// For hardware interfaces this statistic is commonly calculated
// at the device level (unlike @rx_packets) and therefore may include
// packets which did not reach the host.
//
// For IEEE 802.3 devices this counter may be equivalent to:
//
// - 30.3.1.1.21 aMulticastFramesReceivedOK
//
// @collisions: Number of collisions during packet transmissions.
//
// @rx_length_errors: Number of packets dropped due to invalid length.
// Part of aggregate "frame" errors in `/proc/net/dev`.
//
// For IEEE 802.3 devices this counter should be equivalent to a sum
// of the following attributes:
//
// - 30.3.1.1.23 aInRangeLengthErrors
// - 30.3.1.1.24 aOutOfRangeLengthField
// - 30.3.1.1.25 aFrameTooLongErrors
//
// @rx_over_errors: Receiver FIFO overflow event counter.
//
// Historically the count of overflow events. Such events may be
// reported in the receive descriptors or via interrupts, and may
// not correspond one-to-one with dropped packets.
//
// The recommended interpretation for high speed interfaces is -
// number of packets dropped because they did not fit into buffers
// provided by the host, e.g. packets larger than MTU or next buffer
// in the ring was not available for a scatter transfer.
//
// Part of aggregate "frame" errors in `/proc/net/dev`.
//
// This statistics was historically used interchangeably with
// @rx_fifo_errors.
//
// This statistic corresponds to hardware events and is not commonly used
// on software devices.
//
// @rx_crc_errors: Number of packets received with a CRC error.
// Part of aggregate "frame" errors in `/proc/net/dev`.
//
// For IEEE 802.3 devices this counter must be equivalent to:
//
// - 30.3.1.1.6 aFrameCheckSequenceErrors
//
// @rx_frame_errors: Receiver frame alignment errors.
// Part of aggregate "frame" errors in `/proc/net/dev`.
//
// For IEEE 802.3 devices this counter should be equivalent to:
//
// - 30.3.1.1.7 aAlignmentErrors
//
// @rx_fifo_errors: Receiver FIFO error counter.
//
// Historically the count of overflow events. Those events may be
// reported in the receive descriptors or via interrupts, and may
// not correspond one-to-one with dropped packets.
//
// This statistics was used interchangeably with @rx_over_errors.
// Not recommended for use in drivers for high speed interfaces.
//
// This statistic is used on software devices, e.g. to count software
// packet queue overflow (can) or sequencing errors (GRE).
//
// @rx_missed_errors: Count of packets missed by the host.
// Folded into the "drop" counter in `/proc/net/dev`.
//
// Counts number of packets dropped by the device due to lack
// of buffer space. This usually indicates that the host interface
// is slower than the network interface, or host is not keeping up
// with the receive packet rate.
//
// This statistic corresponds to hardware events and is not used
// on software devices.
//
// @tx_aborted_errors:
// Part of aggregate "carrier" errors in `/proc/net/dev`.
// For IEEE 802.3 devices capable of half-duplex operation this counter
// must be equivalent to:
//
// - 30.3.1.1.11 aFramesAbortedDueToXSColls
//
// High speed interfaces may use this counter as a general device
// discard counter.
//
// @tx_carrier_errors: Number of frame transmission errors due to loss
// of carrier during transmission.
// Part of aggregate "carrier" errors in `/proc/net/dev`.
//
// For IEEE 802.3 devices this counter must be equivalent to:
//
// - 30.3.1.1.13 aCarrierSenseErrors
//
// @tx_fifo_errors: Number of frame transmission errors due to device
// FIFO underrun / underflow. This condition occurs when the device
// begins transmission of a frame but is unable to deliver the
// entire frame to the transmitter in time for transmission.
// Part of aggregate "carrier" errors in `/proc/net/dev`.
//
// @tx_heartbeat_errors: Number of Heartbeat / SQE Test errors for
// old half-duplex Ethernet.
// Part of aggregate "carrier" errors in `/proc/net/dev`.
//
// For IEEE 802.3 devices possibly equivalent to:
//
// - 30.3.2.1.4 aSQETestErrors
//
// @tx_window_errors: Number of frame transmission errors due
// to late collisions (for Ethernet - after the first 64B of transmission).
// Part of aggregate "carrier" errors in `/proc/net/dev`.
//
// For IEEE 802.3 devices this counter must be equivalent to:
//
// - 30.3.1.1.10 aLateCollisions
//
// @rx_compressed: Number of correctly received compressed packets.
// This counters is only meaningful for interfaces which support
// packet compression (e.g. CSLIP, PPP).
//
// @tx_compressed: Number of transmitted compressed packets.
// This counters is only meaningful for interfaces which support
// packet compression (e.g. CSLIP, PPP).
//
// @rx_nohandler: Number of packets received on the interface
// but dropped by the networking stack because the device is
// not designated to receive packets (e.g. backup link in a bond).
//
// @rx_otherhost_dropped: Number of packets dropped due to mismatch
// in destination MAC address.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtnl_link_stats64 {
    pub rx_packets: __u64,
    pub tx_packets: __u64,
    pub rx_bytes: __u64,
    pub tx_bytes: __u64,
    pub rx_errors: __u64,
    pub tx_errors: __u64,
    pub rx_dropped: __u64,
    pub tx_dropped: __u64,
    pub multicast: __u64,
    pub collisions: __u64,
// detailed rx_errors:
    pub rx_length_errors: __u64,
    pub rx_over_errors: __u64,
    pub rx_crc_errors: __u64,
    pub rx_frame_errors: __u64,
    pub rx_fifo_errors: __u64,
    pub rx_missed_errors: __u64,
// detailed tx_errors
    pub tx_aborted_errors: __u64,
    pub tx_carrier_errors: __u64,
    pub tx_fifo_errors: __u64,
    pub tx_heartbeat_errors: __u64,
    pub tx_window_errors: __u64,
// for cslip etc
    pub rx_compressed: __u64,
    pub tx_compressed: __u64,
    pub rx_nohandler: __u64,
    pub rx_otherhost_dropped: __u64,
}

// Subset of link stats useful for in-HW collection. Meaning of the fields is as
// for struct rtnl_link_stats64.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtnl_hw_stats64 {
    pub rx_packets: __u64,
    pub tx_packets: __u64,
    pub rx_bytes: __u64,
    pub tx_bytes: __u64,
    pub rx_errors: __u64,
    pub tx_errors: __u64,
    pub rx_dropped: __u64,
    pub tx_dropped: __u64,
    pub multicast: __u64,
}

// The struct should be in sync with struct ifmap
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtnl_link_ifmap {
    pub mem_start: __u64,
    pub mem_end: __u64,
    pub base_addr: __u64,
    pub irq: __u16,
    pub dma: __u8,
    pub port: __u8,
}

//
// IFLA_AF_SPEC
// Contains nested attributes for address family specific attributes.
// Each address family may create a attribute with the address family
// number as type and create its own attribute structure in it.
//
// Example:
// [IFLA_AF_SPEC] = {
// [AF_INET] = {
// [IFLA_INET_CONF] = ...,
// },
// [AF_INET6] = {
// [IFLA_INET6_FLAGS] = ...,
// [IFLA_INET6_CONF] = ...,
// }
//

// device (sysfs) name as parent, used instead
// of IFLA_LINK where there's no parent netdev
//

// backwards compatibility for userspace

// ifi_flags.
//
// IFLA_LINK.
//
// Subtype attributes for IFLA_PROTINFO

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum in6_addr_gen_mode {
    IN6_ADDR_GEN_MODE_EUI64,
    IN6_ADDR_GEN_MODE_NONE,
    IN6_ADDR_GEN_MODE_STABLE_PRIVACY,
    IN6_ADDR_GEN_MODE_RANDOM,
}

// Bridge section
//
// DOC: Bridge enum definition
//
// Please *note* that the timer values in the following section are expected
// in clock_t format, which is seconds multiplied by USER_HZ (generally
// defined as 100).
//
// @IFLA_BR_FORWARD_DELAY
// The bridge forwarding delay is the time spent in LISTENING state
// (before moving to LEARNING) and in LEARNING state (before moving
// to FORWARDING). Only relevant if STP is enabled.
//
// The valid values are between (2 * USER_HZ) and (30 * USER_HZ).
// The default value is (15 * USER_HZ).
//
// @IFLA_BR_HELLO_TIME
// The time between hello packets sent by the bridge, when it is a root
// bridge or a designated bridge. Only relevant if STP is enabled.
//
// The valid values are between (1 * USER_HZ) and (10 * USER_HZ).
// The default value is (2 * USER_HZ).
//
// @IFLA_BR_MAX_AGE
// The hello packet timeout is the time until another bridge in the
// spanning tree is assumed to be dead, after reception of its last hello
// message. Only relevant if STP is enabled.
//
// The valid values are between (6 * USER_HZ) and (40 * USER_HZ).
// The default value is (20 * USER_HZ).
//
// @IFLA_BR_AGEING_TIME
// Configure the bridge's FDB entries aging time. It is the time a MAC
// address will be kept in the FDB after a packet has been received from
// that address. After this time has passed, entries are cleaned up.
// Allow values outside the 802.1 standard specification for special cases:
//
// * 0 - entry never ages (all permanent)
// * 1 - entry disappears (no persistence)
//
// The default value is (300 * USER_HZ).
//
// @IFLA_BR_STP_STATE
// Turn spanning tree protocol on (*IFLA_BR_STP_STATE* > 0) or off
// (*IFLA_BR_STP_STATE* == 0) for this bridge.
//
// The default value is 0 (disabled).
//
// @IFLA_BR_PRIORITY
// Set this bridge's spanning tree priority, used during STP root bridge
// election.
//
// The valid values are between 0 and 65535.
//
// @IFLA_BR_VLAN_FILTERING
// Turn VLAN filtering on (*IFLA_BR_VLAN_FILTERING* > 0) or off
// (*IFLA_BR_VLAN_FILTERING* == 0). When disabled, the bridge will not
// consider the VLAN tag when handling packets.
//
// The default value is 0 (disabled).
//
// @IFLA_BR_VLAN_PROTOCOL
// Set the protocol used for VLAN filtering.
//
// The valid values are 0x8100(802.1Q) or 0x88A8(802.1AD). The default value
// is 0x8100(802.1Q).
//
// @IFLA_BR_GROUP_FWD_MASK
// The group forwarding mask. This is the bitmask that is applied to
// decide whether to forward incoming frames destined to link-local
// addresses (of the form 01:80:C2:00:00:0X).
//
// The default value is 0, which means the bridge does not forward any
// link-local frames coming on this port.
//
// @IFLA_BR_ROOT_ID
// The bridge root id, read only.
//
// @IFLA_BR_BRIDGE_ID
// The bridge id, read only.
//
// @IFLA_BR_ROOT_PORT
// The bridge root port, read only.
//
// @IFLA_BR_ROOT_PATH_COST
// The bridge root path cost, read only.
//
// @IFLA_BR_TOPOLOGY_CHANGE
// The bridge topology change, read only.
//
// @IFLA_BR_TOPOLOGY_CHANGE_DETECTED
// The bridge topology change detected, read only.
//
// @IFLA_BR_HELLO_TIMER
// The bridge hello timer, read only.
//
// @IFLA_BR_TCN_TIMER
// The bridge tcn timer, read only.
//
// @IFLA_BR_TOPOLOGY_CHANGE_TIMER
// The bridge topology change timer, read only.
//
// @IFLA_BR_GC_TIMER
// The bridge gc timer, read only.
//
// @IFLA_BR_GROUP_ADDR
// Set the MAC address of the multicast group this bridge uses for STP.
// The address must be a link-local address in standard Ethernet MAC address
// format. It is an address of the form 01:80:C2:00:00:0X, with X in [0, 4..f].
//
// The default value is 0.
//
// @IFLA_BR_FDB_FLUSH
// Flush bridge's fdb dynamic entries.
//
// @IFLA_BR_MCAST_ROUTER
// Set bridge's multicast router if IGMP snooping is enabled.
// The valid values are:
//
// * 0 - disabled.
// * 1 - automatic (queried).
// * 2 - permanently enabled.
//
// The default value is 1.
//
// @IFLA_BR_MCAST_SNOOPING
// Turn multicast snooping on (*IFLA_BR_MCAST_SNOOPING* > 0) or off
// (*IFLA_BR_MCAST_SNOOPING* == 0).
//
// The default value is 1.
//
// @IFLA_BR_MCAST_QUERY_USE_IFADDR
// If enabled use the bridge's own IP address as source address for IGMP
// queries (*IFLA_BR_MCAST_QUERY_USE_IFADDR* > 0) or the default of 0.0.0.0
// (*IFLA_BR_MCAST_QUERY_USE_IFADDR* == 0).
//
// The default value is 0 (disabled).
//
// @IFLA_BR_MCAST_QUERIER
// Enable (*IFLA_BR_MULTICAST_QUERIER* > 0) or disable
// (*IFLA_BR_MULTICAST_QUERIER* == 0) IGMP querier, ie sending of multicast
// queries by the bridge.
//
// The default value is 0 (disabled).
//
// @IFLA_BR_MCAST_HASH_ELASTICITY
// Set multicast database hash elasticity, It is the maximum chain length in
// the multicast hash table. This attribute is *deprecated* and the value
// is always 16.
//
// @IFLA_BR_MCAST_HASH_MAX
// Set maximum size of the multicast hash table
//
// The default value is 4096, the value must be a power of 2.
//
// @IFLA_BR_MCAST_LAST_MEMBER_CNT
// The Last Member Query Count is the number of Group-Specific Queries
// sent before the router assumes there are no local members. The Last
// Member Query Count is also the number of Group-and-Source-Specific
// Queries sent before the router assumes there are no listeners for a
// particular source.
//
// The default value is 2.
//
// @IFLA_BR_MCAST_STARTUP_QUERY_CNT
// The Startup Query Count is the number of Queries sent out on startup,
// separated by the Startup Query Interval.
//
// The default value is 2.
//
// @IFLA_BR_MCAST_LAST_MEMBER_INTVL
// The Last Member Query Interval is the Max Response Time inserted into
// Group-Specific Queries sent in response to Leave Group messages, and
// is also the amount of time between Group-Specific Query messages.
//
// The default value is (1 * USER_HZ).
//
// @IFLA_BR_MCAST_MEMBERSHIP_INTVL
// The interval after which the bridge will leave a group, if no membership
// reports for this group are received.
//
// The default value is (260 * USER_HZ).
//
// @IFLA_BR_MCAST_QUERIER_INTVL
// The interval between queries sent by other routers. if no queries are
// seen after this delay has passed, the bridge will start to send its own
// queries (as if *IFLA_BR_MCAST_QUERIER_INTVL* was enabled).
//
// The default value is (255 * USER_HZ).
//
// @IFLA_BR_MCAST_QUERY_INTVL
// The Query Interval is the interval between General Queries sent by
// the Querier.
//
// The default value is (125 * USER_HZ). The minimum value is (1 * USER_HZ).
//
// @IFLA_BR_MCAST_QUERY_RESPONSE_INTVL
// The Max Response Time used to calculate the Max Resp Code inserted
// into the periodic General Queries.
//
// The default value is (10 * USER_HZ).
//
// @IFLA_BR_MCAST_STARTUP_QUERY_INTVL
// The interval between queries in the startup phase.
//
// The default value is (125 * USER_HZ) / 4. The minimum value is (1 * USER_HZ).
//
// @IFLA_BR_NF_CALL_IPTABLES
// Enable (*NF_CALL_IPTABLES* > 0) or disable (*NF_CALL_IPTABLES* == 0)
// iptables hooks on the bridge.
//
// The default value is 0 (disabled).
//
// @IFLA_BR_NF_CALL_IP6TABLES
// Enable (*NF_CALL_IP6TABLES* > 0) or disable (*NF_CALL_IP6TABLES* == 0)
// ip6tables hooks on the bridge.
//
// The default value is 0 (disabled).
//
// @IFLA_BR_NF_CALL_ARPTABLES
// Enable (*NF_CALL_ARPTABLES* > 0) or disable (*NF_CALL_ARPTABLES* == 0)
// arptables hooks on the bridge.
//
// The default value is 0 (disabled).
//
// @IFLA_BR_VLAN_DEFAULT_PVID
// VLAN ID applied to untagged and priority-tagged incoming packets.
//
// The default value is 1. Setting to the special value 0 makes all ports of
// this bridge not have a PVID by default, which means that they will
// not accept VLAN-untagged traffic.
//
// @IFLA_BR_PAD
// Bridge attribute padding type for netlink message.
//
// @IFLA_BR_VLAN_STATS_ENABLED
// Enable (*IFLA_BR_VLAN_STATS_ENABLED* == 1) or disable
// (*IFLA_BR_VLAN_STATS_ENABLED* == 0) per-VLAN stats accounting.
//
// The default value is 0 (disabled).
//
// @IFLA_BR_MCAST_STATS_ENABLED
// Enable (*IFLA_BR_MCAST_STATS_ENABLED* > 0) or disable
// (*IFLA_BR_MCAST_STATS_ENABLED* == 0) multicast (IGMP/MLD) stats
// accounting.
//
// The default value is 0 (disabled).
//
// @IFLA_BR_MCAST_IGMP_VERSION
// Set the IGMP version.
//
// The valid values are 2 and 3. The default value is 2.
//
// @IFLA_BR_MCAST_MLD_VERSION
// Set the MLD version.
//
// The valid values are 1 and 2. The default value is 1.
//
// @IFLA_BR_VLAN_STATS_PER_PORT
// Enable (*IFLA_BR_VLAN_STATS_PER_PORT* == 1) or disable
// (*IFLA_BR_VLAN_STATS_PER_PORT* == 0) per-VLAN per-port stats accounting.
// Can be changed only when there are no port VLANs configured.
//
// The default value is 0 (disabled).
//
// @IFLA_BR_MULTI_BOOLOPT
// The multi_boolopt is used to control new boolean options to avoid adding
// new netlink attributes. You can look at ``enum br_boolopt_id`` for those
// options.
//
// @IFLA_BR_MCAST_QUERIER_STATE
// Bridge mcast querier states, read only.
//
// @IFLA_BR_FDB_N_LEARNED
// The number of dynamically learned FDB entries for the current bridge,
// read only.
//
// @IFLA_BR_FDB_MAX_LEARNED
// Set the number of max dynamically learned FDB entries for the current
// bridge.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ifla_bridge_id {
    pub prio: [__u8; 2],
    pub /: *mut *mut __u8 addr[6]; / ETH_ALEN,
}

//
// DOC: Bridge mode enum definition
//
// @BRIDGE_MODE_HAIRPIN
// Controls whether traffic may be sent back out of the port on which it
// was received. This option is also called reflective relay mode, and is
// used to support basic VEPA (Virtual Ethernet Port Aggregator)
// capabilities. By default, this flag is turned off and the bridge will
// not forward traffic back out of the receiving port.
//
// DOC: Bridge port enum definition
//
// @IFLA_BRPORT_STATE
// The operation state of the port. Here are the valid values.
//
// * 0 - port is in STP *DISABLED* state. Make this port completely
// inactive for STP. This is also called BPDU filter and could be used
// to disable STP on an untrusted port, like a leaf virtual device.
// The traffic forwarding is also stopped on this port.
// * 1 - port is in STP *LISTENING* state. Only valid if STP is enabled
// on the bridge. In this state the port listens for STP BPDUs and
// drops all other traffic frames.
// * 2 - port is in STP *LEARNING* state. Only valid if STP is enabled on
// the bridge. In this state the port will accept traffic only for the
// purpose of updating MAC address tables.
// * 3 - port is in STP *FORWARDING* state. Port is fully active.
// * 4 - port is in STP *BLOCKING* state. Only valid if STP is enabled on
// the bridge. This state is used during the STP election process.
// In this state, port will only process STP BPDUs.
//
// @IFLA_BRPORT_PRIORITY
// The STP port priority. The valid values are between 0 and 255.
//
// @IFLA_BRPORT_COST
// The STP path cost of the port. The valid values are between 1 and 65535.
//
// @IFLA_BRPORT_MODE
// Set the bridge port mode. See *BRIDGE_MODE_HAIRPIN* for more details.
//
// @IFLA_BRPORT_GUARD
// Controls whether STP BPDUs will be processed by the bridge port. By
// default, the flag is turned off to allow BPDU processing. Turning this
// flag on will disable the bridge port if a STP BPDU packet is received.
//
// If the bridge has Spanning Tree enabled, hostile devices on the network
// may send BPDU on a port and cause network failure. Setting *guard on
// will detect and stop this by disabling the port. The port will be
// restarted if the link is brought down, or removed and reattached.
//
// @IFLA_BRPORT_PROTECT
// Controls whether a given port is allowed to become a root port or not.
// Only used when STP is enabled on the bridge. By default the flag is off.
//
// This feature is also called root port guard. If BPDU is received from a
// leaf (edge) port, it should not be elected as root port. This could
// be used if using STP on a bridge and the downstream bridges are not fully
// trusted; this prevents a hostile guest from rerouting traffic.
//
// @IFLA_BRPORT_FAST_LEAVE
// This flag allows the bridge to immediately stop multicast traffic
// forwarding on a port that receives an IGMP Leave message. It is only used
// when IGMP snooping is enabled on the bridge. By default the flag is off.
//
// @IFLA_BRPORT_LEARNING
// Controls whether a given port will learn *source* MAC addresses from
// received traffic or not. Also controls whether dynamic FDB entries
// (which can also be added by software) will be refreshed by incoming
// traffic. By default this flag is on.
//
// @IFLA_BRPORT_UNICAST_FLOOD
// Controls whether unicast traffic for which there is no FDB entry will
// be flooded towards this port. By default this flag is on.
//
// @IFLA_BRPORT_PROXYARP
// Enable proxy ARP on this port.
//
// @IFLA_BRPORT_LEARNING_SYNC
// Controls whether a given port will sync MAC addresses learned on device
// port to bridge FDB.
//
// @IFLA_BRPORT_PROXYARP_WIFI
// Enable proxy ARP on this port which meets extended requirements by
// IEEE 802.11 and Hotspot 2.0 specifications.
//
// @IFLA_BRPORT_ROOT_ID
//
// @IFLA_BRPORT_BRIDGE_ID
//
// @IFLA_BRPORT_DESIGNATED_PORT
//
// @IFLA_BRPORT_DESIGNATED_COST
//
// @IFLA_BRPORT_ID
//
// @IFLA_BRPORT_NO
//
// @IFLA_BRPORT_TOPOLOGY_CHANGE_ACK
//
// @IFLA_BRPORT_CONFIG_PENDING
//
// @IFLA_BRPORT_MESSAGE_AGE_TIMER
//
// @IFLA_BRPORT_FORWARD_DELAY_TIMER
//
// @IFLA_BRPORT_HOLD_TIMER
//
// @IFLA_BRPORT_FLUSH
// Flush bridge ports' fdb dynamic entries.
//
// @IFLA_BRPORT_MULTICAST_ROUTER
// Configure the port's multicast router presence. A port with
// a multicast router will receive all multicast traffic.
// The valid values are:
//
// * 0 disable multicast routers on this port
// * 1 let the system detect the presence of routers (default)
// * 2 permanently enable multicast traffic forwarding on this port
// * 3 enable multicast routers temporarily on this port, not depending
// on incoming queries.
//
// @IFLA_BRPORT_PAD
//
// @IFLA_BRPORT_MCAST_FLOOD
// Controls whether a given port will flood multicast traffic for which
// there is no MDB entry. By default this flag is on.
//
// @IFLA_BRPORT_MCAST_TO_UCAST
// Controls whether a given port will replicate packets using unicast
// instead of multicast. By default this flag is off.
//
// This is done by copying the packet per host and changing the multicast
// destination MAC to a unicast one accordingly.
//
// *mcast_to_unicast* works on top of the multicast snooping feature of the
// bridge. Which means unicast copies are only delivered to hosts which
// are interested in unicast and signaled this via IGMP/MLD reports previously.
//
// This feature is intended for interface types which have a more reliable
// and/or efficient way to deliver unicast packets than broadcast ones
// (e.g. WiFi).
//
// However, it should only be enabled on interfaces where no IGMPv2/MLDv1
// report suppression takes place. IGMP/MLD report suppression issue is
// usually overcome by the network daemon (supplicant) enabling AP isolation
// and by that separating all STAs.
//
// Delivery of STA-to-STA IP multicast is made possible again by enabling
// and utilizing the bridge hairpin mode, which considers the incoming port
// as a potential outgoing port, too (see *BRIDGE_MODE_HAIRPIN* option).
// Hairpin mode is performed after multicast snooping, therefore leading
// to only deliver reports to STAs running a multicast router.
//
// @IFLA_BRPORT_VLAN_TUNNEL
// Controls whether vlan to tunnel mapping is enabled on the port.
// By default this flag is off.
//
// @IFLA_BRPORT_BCAST_FLOOD
// Controls flooding of broadcast traffic on the given port. By default
// this flag is on.
//
// @IFLA_BRPORT_GROUP_FWD_MASK
// Set the group forward mask. This is a bitmask that is applied to
// decide whether to forward incoming frames destined to link-local
// addresses. The addresses of the form are 01:80:C2:00:00:0X (defaults
// to 0, which means the bridge does not forward any link-local frames
// coming on this port).
//
// @IFLA_BRPORT_NEIGH_SUPPRESS
// Controls whether neighbor discovery (arp and nd) proxy and suppression
// is enabled on the port. By default this flag is off.
//
// @IFLA_BRPORT_ISOLATED
// Controls whether a given port will be isolated, which means it will be
// able to communicate with non-isolated ports only. By default this
// flag is off.
//
// @IFLA_BRPORT_BACKUP_PORT
// Set a backup port. If the port loses carrier all traffic will be
// redirected to the configured backup port. Set the value to 0 to disable
// it.
//
// @IFLA_BRPORT_MRP_RING_OPEN
//
// @IFLA_BRPORT_MRP_IN_OPEN
//
// @IFLA_BRPORT_MCAST_EHT_HOSTS_LIMIT
// The number of per-port EHT hosts limit. The default value is 512.
// Setting to 0 is not allowed.
//
// @IFLA_BRPORT_MCAST_EHT_HOSTS_CNT
// The current number of tracked hosts, read only.
//
// @IFLA_BRPORT_LOCKED
// Controls whether a port will be locked, meaning that hosts behind the
// port will not be able to communicate through the port unless an FDB
// entry with the unit's MAC address is in the FDB. The common use case is
// that hosts are allowed access through authentication with the IEEE 802.1X
// protocol or based on whitelists. By default this flag is off.
//
// Please note that secure 802.1X deployments should always use the
// *BR_BOOLOPT_NO_LL_LEARN* flag, to not permit the bridge to populate its
// FDB based on link-local (EAPOL) traffic received on the port.
//
// @IFLA_BRPORT_MAB
// Controls whether a port will use MAC Authentication Bypass (MAB), a
// technique through which select MAC addresses may be allowed on a locked
// port, without using 802.1X authentication. Packets with an unknown source
// MAC address generates a "locked" FDB entry on the incoming bridge port.
// The common use case is for user space to react to these bridge FDB
// notifications and optionally replace the locked FDB entry with a normal
// one, allowing traffic to pass for whitelisted MAC addresses.
//
// Setting this flag also requires *IFLA_BRPORT_LOCKED* and
// *IFLA_BRPORT_LEARNING*. *IFLA_BRPORT_LOCKED* ensures that unauthorized
// data packets are dropped, and *IFLA_BRPORT_LEARNING* allows the dynamic
// FDB entries installed by user space (as replacements for the locked FDB
// entries) to be refreshed and/or aged out.
//
// @IFLA_BRPORT_MCAST_N_GROUPS
//
// @IFLA_BRPORT_MCAST_MAX_GROUPS
// Sets the maximum number of MDB entries that can be registered for a
// given port. Attempts to register more MDB entries at the port than this
// limit allows will be rejected, whether they are done through netlink
// (e.g. the bridge tool), or IGMP or MLD membership reports. Setting a
// limit of 0 disables the limit. The default value is 0.
//
// @IFLA_BRPORT_NEIGH_VLAN_SUPPRESS
// Controls whether neighbor discovery (arp and nd) proxy and suppression is
// enabled for a given port. By default this flag is off.
//
// Note that this option only takes effect when *IFLA_BRPORT_NEIGH_SUPPRESS
// is enabled for a given port.
//
// @IFLA_BRPORT_BACKUP_NHID
// The FDB nexthop object ID to attach to packets being redirected to a
// backup port that has VLAN tunnel mapping enabled (via the
// *IFLA_BRPORT_VLAN_TUNNEL* option). Setting a value of 0 (default) has
// the effect of not attaching any ID.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ifla_cacheinfo {
    pub max_reasm_len: __u32,
    pub /: *mut *mut __u32 tstamp; / ipv6InterfaceTable updated timestamp,
    pub reachable_time: __u32,
    pub retrans_time: __u32,
}

// VLAN section

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ifla_vlan_flags {
    pub flags: __u32,
    pub mask: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ifla_vlan_qos_mapping {
    pub from: __u32,
    pub to: __u32,
}

// MACVLAN section

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum macvlan_mode {
    MACVLAN_MODE_PRIVATE = 1, /* don't talk to other macvlans */
    MACVLAN_MODE_VEPA    = 2, /* talk to other ports through ext bridge */
    MACVLAN_MODE_BRIDGE  = 4, /* talk to bridge ports directly */
    MACVLAN_MODE_PASSTHRU = 8,/* take over the underlying device */
    MACVLAN_MODE_SOURCE  = 16,/* use source MAC address list to assign */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum macvlan_macaddr_mode {
    MACVLAN_MACADDR_ADD,
    MACVLAN_MACADDR_DEL,
    MACVLAN_MACADDR_FLUSH,
    MACVLAN_MACADDR_SET,
}

pub const MACVLAN_FLAG_NOPROMISC: c_int = 1;

// VRF section

// MACSEC section

// XFRM section

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum macsec_validation_type {
    MACSEC_VALIDATE_DISABLED = 0,
    MACSEC_VALIDATE_CHECK = 1,
    MACSEC_VALIDATE_STRICT = 2,
    __MACSEC_VALIDATE_END,
    MACSEC_VALIDATE_MAX = __MACSEC_VALIDATE_END - 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum macsec_offload {
    MACSEC_OFFLOAD_OFF = 0,
    MACSEC_OFFLOAD_PHY = 1,
    MACSEC_OFFLOAD_MAC = 2,
    __MACSEC_OFFLOAD_END,
    MACSEC_OFFLOAD_MAX = __MACSEC_OFFLOAD_END - 1,
}

// IPVLAN section

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ipvlan_mode {
    IPVLAN_MODE_L2 = 0,
    IPVLAN_MODE_L3,
    IPVLAN_MODE_L3S,
    IPVLAN_MODE_MAX
}

pub const IPVLAN_F_PRIVATE: c_uint = 0x01;
pub const IPVLAN_F_VEPA: c_uint = 0x02;
// Tunnel RTM header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tunnel_msg {
    pub family: __u8,
    pub flags: __u8,
    pub reserved2: __u16,
    pub ifindex: __u32,
}

// netkit section
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum netkit_action {
    NETKIT_NEXT	= -1,
    NETKIT_PASS	= 0,
    NETKIT_DROP	= 2,
    NETKIT_REDIRECT	= 7,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum netkit_mode {
    NETKIT_L2,
    NETKIT_L3,
}

// NETKIT_SCRUB_NONE leaves clearing skb->{mark,priority} up to
// the BPF program if attached. This also means the latter can
// consume the two fields if they were populated earlier.
//
// NETKIT_SCRUB_DEFAULT zeroes skb->{mark,priority} fields before
// invoking the attached BPF program when the peer device resides
// in a different network namespace. This is the default behavior.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum netkit_scrub {
    NETKIT_SCRUB_NONE,
    NETKIT_SCRUB_DEFAULT,
}

// VXLAN section
// include statistics in the dump
pub const TUNNEL_MSG_FLAG_STATS: c_uint = 0x01;

// Embedded inside VXLAN_VNIFILTER_ENTRY_STATS

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ifla_vxlan_port_range {
    pub low: __be16,
    pub high: __be16,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ifla_vxlan_df {
    VXLAN_DF_UNSET = 0,
    VXLAN_DF_SET,
    VXLAN_DF_INHERIT,
    __VXLAN_DF_END,
    VXLAN_DF_MAX = __VXLAN_DF_END - 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ifla_vxlan_label_policy {
    VXLAN_LABEL_FIXED = 0,
    VXLAN_LABEL_INHERIT = 1,
    __VXLAN_LABEL_END,
    VXLAN_LABEL_MAX = __VXLAN_LABEL_END - 1,
}

// GENEVE section

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ifla_geneve_df {
    GENEVE_DF_UNSET = 0,
    GENEVE_DF_SET,
    GENEVE_DF_INHERIT,
    __GENEVE_DF_END,
    GENEVE_DF_MAX = __GENEVE_DF_END - 1,
}

// Bareudp section

// PPP section

// GTP section
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ifla_gtp_role {
    GTP_ROLE_GGSN = 0,
    GTP_ROLE_SGSN,
}

// Bonding section

// SR-IOV virtual function management section

// on/off switch
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ifla_vf_mac {
    pub vf: __u32,
    pub /: *mut *mut __u8 mac[32]; / MAX_ADDR_LEN,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ifla_vf_broadcast {
    pub broadcast: [__u8; 32],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ifla_vf_vlan {
    pub vf: __u32,
    pub /: *mut *mut __u32 vlan; / 0 - 4095, 0 disables VLAN filter,
    pub qos: __u32,
}

pub const MAX_VLAN_LIST_LEN: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ifla_vf_vlan_info {
    pub vf: __u32,
    pub /: *mut *mut __u32 vlan; / 0 - 4095, 0 disables VLAN filter,
    pub qos: __u32,
    pub /: *mut *mut __be16 vlan_proto; / VLAN protocol either 802.1Q or 802.1ad,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ifla_vf_tx_rate {
    pub vf: __u32,
    pub /: *mut *mut __u32 rate; / Max TX bandwidth in Mbps, 0 disables throttling,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ifla_vf_rate {
    pub vf: __u32,
    pub /: *mut *mut __u32 min_tx_rate; / Min Bandwidth in Mbps,
    pub /: *mut *mut __u32 max_tx_rate; / Max Bandwidth in Mbps,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ifla_vf_spoofchk {
    pub vf: __u32,
    pub setting: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ifla_vf_guid {
    pub vf: __u32,
    pub guid: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ifla_vf_link_state {
    pub vf: __u32,
    pub link_state: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ifla_vf_rss_query_en {
    pub vf: __u32,
    pub setting: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ifla_vf_trust {
    pub vf: __u32,
    pub setting: __u32,
}

// VF ports management section
//
// Nested layout of set/get msg is:
//
// [IFLA_NUM_VF]
// [IFLA_VF_PORTS]
// [IFLA_VF_PORT]
// [IFLA_PORT_*], ...
// [IFLA_VF_PORT]
// [IFLA_PORT_*], ...
// ...
// [IFLA_PORT_SELF]
// [IFLA_PORT_*], ...
//

pub const PORT_PROFILE_MAX: c_int = 40;
pub const PORT_UUID_MAX: c_int = 16;

// 0x08-0xFF reserved for future VDP use
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ifla_port_vsi {
    pub vsi_mgr_id: __u8,
    pub vsi_type_id: [__u8; 3],
    pub vsi_type_version: __u8,
    pub pad: [__u8; 3],
}

// IPoIB section

// HSR/PRP section, both uses same interface
// Different redundancy protocols for hsr device
// HSR. For example PRP.
//

// STATS section
#[repr(C)]
#[derive(Copy, Clone)]
pub struct if_stats_msg {
    pub family: __u8,
    pub pad1: __u8,
    pub pad2: __u16,
    pub ifindex: __u32,
    pub filter_mask: __u32,
}

// A stats attribute can be netdev specific or a global stat.
// For netdev stats, lets use the prefix IFLA_STATS_LINK_
//

// a filter mask for the corresponding group.
//

// These are embedded into IFLA_STATS_LINK_XSTATS:
// [IFLA_STATS_LINK_XSTATS]
// -> [LINK_XSTATS_TYPE_xxx]
// -> [rtnl link type specific attributes]
//

// These are stats embedded into IFLA_STATS_LINK_OFFLOAD_XSTATS

// XDP section

// These are stored into IFLA_XDP_ATTACHED on dump.

// tun section

// rmnet section

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ifla_rmnet_flags {
    pub flags: __u32,
    pub mask: __u32,
}

// MCTP section

// DSA section
// Deprecated, use IFLA_DSA_CONDUIT instead

