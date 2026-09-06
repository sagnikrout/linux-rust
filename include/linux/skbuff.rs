//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/skbuff.h
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
// Definitions for the 'struct sk_buff' memory handlers.
//
// Authors:
// Alan Cox, <gw4pts@gw4pts.ampr.org>
// Florian La Roche, <rzsfl@rz.uni-sb.de>
//

//
// DOC: skb checksums
//
// The interface for checksum offload between the stack and networking drivers
// is as follows...
//
// IP checksum related features
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//
// Drivers advertise checksum offload capabilities in the features of a device.
// From the stack's point of view these are capabilities offered by the driver.
// A driver typically only advertises features that it is capable of offloading
// to its device.
//
// .. flat-table:: Checksum related device features
// :widths: 1 10
//
// * - %NETIF_F_HW_CSUM
// - The driver (or its device) is able to compute one
// IP (one's complement) checksum for any combination
// of protocols or protocol layering. The checksum is
// computed and set in a packet per the CHECKSUM_PARTIAL
// interface (see below).
//
// * - %NETIF_F_IP_CSUM
// - Driver (device) is only able to checksum plain
// TCP or UDP packets over IPv4. These are specifically
// unencapsulated packets of the form IPv4|TCP or
// IPv4|UDP where the Protocol field in the IPv4 header
// is TCP or UDP. The IPv4 header may contain IP options.
// This feature cannot be set in features for a device
// with NETIF_F_HW_CSUM also set. This feature is being
// DEPRECATED (see below).
//
// * - %NETIF_F_IPV6_CSUM
// - Driver (device) is only able to checksum plain
// TCP or UDP packets over IPv6. These are specifically
// unencapsulated packets of the form IPv6|TCP or
// IPv6|UDP where the Next Header field in the IPv6
// header is either TCP or UDP. IPv6 extension headers
// are not supported with this feature. This feature
// cannot be set in features for a device with
// NETIF_F_HW_CSUM also set. This feature is being
// DEPRECATED (see below).
//
// * - %NETIF_F_RXCSUM
// - Driver (device) performs receive checksum offload.
// This flag is only used to disable the RX checksum
// feature for a device. The stack will accept receive
// checksum indication in packets received on a device
// regardless of whether NETIF_F_RXCSUM is set.
//
// Checksumming of received packets by device
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//
// Indication of checksum verification is set in &sk_buff.ip_summed.
// Possible values are:
//
// - %CHECKSUM_NONE
//
// Device did not checksum this packet e.g. due to lack of capabilities.
// The packet contains full (though not verified) checksum in packet but
// not in skb->csum. Thus, skb->csum is undefined in this case.
//
// - %CHECKSUM_UNNECESSARY
//
// The hardware you're dealing with doesn't calculate the full checksum
// (as in %CHECKSUM_COMPLETE), but it does parse headers and verify checksums
// for specific protocols. For such packets it will set %CHECKSUM_UNNECESSARY
// if their checksums are okay. &sk_buff.csum is still undefined in this case
// though. A driver or device must never modify the checksum field in the
// packet even if checksum is verified.
//
// %CHECKSUM_UNNECESSARY is applicable to following protocols:
//
// - TCP: IPv6 and IPv4.
// - UDP: IPv4 and IPv6. A device may apply CHECKSUM_UNNECESSARY to a
// zero UDP checksum for either IPv4 or IPv6, the networking stack
// may perform further validation in this case.
// - GRE: only if the checksum is present in the header.
// - SCTP: indicates the CRC in SCTP header has been validated.
// - FCOE: indicates the CRC in FC frame has been validated.
//
// &sk_buff.csum_level indicates the number of consecutive checksums found in
// the packet minus one that have been verified as %CHECKSUM_UNNECESSARY.
// For instance if a device receives an IPv6->UDP->GRE->IPv4->TCP packet
// and a device is able to verify the checksums for UDP (possibly zero),
// GRE (checksum flag is set) and TCP, &sk_buff.csum_level would be set to
// two. If the device were only able to verify the UDP checksum and not
// GRE, either because it doesn't support GRE checksum or because GRE
// checksum is bad, skb->csum_level would be set to zero (TCP checksum is
// not considered in this case).
//
// - %CHECKSUM_COMPLETE
//
// This is the most generic way. The device supplied checksum of the _whole_
// packet as seen by netif_rx() and fills in &sk_buff.csum. This means the
// hardware doesn't need to parse L3/L4 headers to implement this.
//
// Notes:
//
// - Even if device supports only some protocols, but is able to produce
// skb->csum, it MUST use CHECKSUM_COMPLETE, not CHECKSUM_UNNECESSARY.
// - CHECKSUM_COMPLETE is not applicable to SCTP and FCoE protocols.
//
// - %CHECKSUM_PARTIAL
//
// A checksum is set up to be offloaded to a device as described in the
// output description for CHECKSUM_PARTIAL. This may occur on a packet
// received directly from another Linux OS, e.g., a virtualized Linux kernel
// on the same host, or it may be set in the input path in GRO or remote
// checksum offload. For the purposes of checksum verification, the checksum
// referred to by skb->csum_start + skb->csum_offset and any preceding
// checksums in the packet are considered verified. Any checksums in the
// packet that are after the checksum being offloaded are not considered to
// be verified.
//
// Checksumming on transmit for non-GSO
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//
// The stack requests checksum offload in the &sk_buff.ip_summed for a packet.
// Values are:
//
// - %CHECKSUM_PARTIAL
//
// The driver is required to checksum the packet as seen by hard_start_xmit()
// from &sk_buff.csum_start up to the end, and to record/write the checksum at
// offset &sk_buff.csum_start + &sk_buff.csum_offset.
// A driver may verify that the
// csum_start and csum_offset values are valid values given the length and
// offset of the packet, but it should not attempt to validate that the
// checksum refers to a legitimate transport layer checksum -- it is the
// purview of the stack to validate that csum_start and csum_offset are set
// correctly.
//
// When the stack requests checksum offload for a packet, the driver MUST
// ensure that the checksum is set correctly. A driver can either offload the
// checksum calculation to the device, or call skb_checksum_help (in the case
// that the device does not support offload for a particular checksum).
//
// %NETIF_F_IP_CSUM and %NETIF_F_IPV6_CSUM are being deprecated in favor of
// %NETIF_F_HW_CSUM. New devices should use %NETIF_F_HW_CSUM to indicate
// checksum offload capability.
// skb_csum_hwoffload_help() can be called to resolve %CHECKSUM_PARTIAL based
// on network device checksumming capabilities: if a packet does not match
// them, skb_checksum_help() or skb_crc32c_help() (depending on the value of
// &sk_buff.csum_not_inet, see :ref:`crc`)
// is called to resolve the checksum.
//
// - %CHECKSUM_NONE
//
// The skb was already checksummed by the protocol, or a checksum is not
// required.
//
// - %CHECKSUM_UNNECESSARY
//
// This has the same meaning as CHECKSUM_NONE for checksum offload on
// output.
//
// - %CHECKSUM_COMPLETE
//
// Not used in checksum output. If a driver observes a packet with this value
// set in skbuff, it should treat the packet as if %CHECKSUM_NONE were set.
//
// .. _crc:
//
// Non-IP checksum (CRC) offloads
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//
// .. flat-table::
// :widths: 1 10
//
// * - %NETIF_F_SCTP_CRC
// - This feature indicates that a device is capable of
// offloading the SCTP CRC in a packet. To perform this offload the stack
// will set csum_start and csum_offset accordingly, set ip_summed to
// %CHECKSUM_PARTIAL and set csum_not_inet to 1, to provide an indication
// in the skbuff that the %CHECKSUM_PARTIAL refers to CRC32c.
// A driver that supports both IP checksum offload and SCTP CRC32c offload
// must verify which offload is configured for a packet by testing the
// value of &sk_buff.csum_not_inet; skb_crc32c_csum_help() is provided to
// resolve %CHECKSUM_PARTIAL on skbs where csum_not_inet is set to 1.
//
// * - %NETIF_F_FCOE_CRC
// - This feature indicates that a device is capable of offloading the FCOE
// CRC in a packet. To perform this offload the stack will set ip_summed
// to %CHECKSUM_PARTIAL and set csum_start and csum_offset
// accordingly. Note that there is no indication in the skbuff that the
// %CHECKSUM_PARTIAL refers to an FCOE checksum, so a driver that supports
// both IP checksum offload and FCOE CRC offload must verify which offload
// is configured for a packet, presumably by inspecting packet headers.
//
// Checksumming on output with GSO
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//
// In the case of a GSO packet (skb_is_gso() is true), checksum offload
// is implied by the SKB_GSO_* flags in gso_type. Most obviously, if the
// gso_type is %SKB_GSO_TCPV4 or %SKB_GSO_TCPV6, TCP checksum offload as
// part of the GSO operation is implied. If a checksum is being offloaded
// with GSO then ip_summed is %CHECKSUM_PARTIAL, and both csum_start and
// csum_offset are set to refer to the outermost checksum being offloaded
// (two offloaded checksums are possible with UDP encapsulation).
//
// Don't change this without changing skb_csum_unnecessary!
pub const CHECKSUM_NONE: c_int = 0;
pub const CHECKSUM_UNNECESSARY: c_int = 1;
pub const CHECKSUM_COMPLETE: c_int = 2;
pub const CHECKSUM_PARTIAL: c_int = 3;
// Maximum value in skb->csum_level
pub const SKB_MAX_CSUM_LEVEL: c_int = 3;

// For X bytes available in skb->head, what is the minimal
// allocation needed, knowing struct skb_shared_info needs
// to be aligned.
//

// return minimum truesize of one skb containing X bytes of data

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_bridge_info {
    pub orig_proto:8: },
    pub pkt_otherhost:1: u8,
    pub in_prerouting:1: u8,
    pub bridged_dnat:1: u8,
    pub sabotage_in_done:1: u8,
    pub frag_max_size: __u16,
    pub physinif: c_int,
// always valid & non-NULL from FORWARD on, for physdev match
    pub physoutdev: *mut net_device,
// prerouting: detect dnat in orig/reply direction
    pub ipv4_daddr: __be32,
    pub ipv6_daddr: in6_addr,
// after prerouting + nat detected: store original source
// mac since neigh resolution overwrites it, only used while
// skb is out in neigh layer.
//
    pub neigh_header: [c_char; 8],
}

// Chain in tc_skb_ext will be used to share the tc chain with
// ovs recirc_id. It will be set to the current chain by tc
// and read by ovs to recirc_id.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_skb_ext {
    pub act_miss_cookie: u64,
    pub chain: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sk_buff_head {
// These two members must be first to match sk_buff.
    pub next: *mut sk_buff,
    pub prev: *mut sk_buff,
    pub qlen: __u32,
    pub lock: spinlock_t,
}

// Set skb_shinfo(skb)->gso_size to this in case you want skb_segment to
// segment using its current segmentation instead.
//
pub const GSO_BY_FRAGS: c_uint = 0xFFFF;
//
// skb_frag_size() - Returns the size of a skb fragment
// @frag: skb fragment
//
// skb_frag_size_set() - Sets the size of a skb fragment
// @frag: skb fragment
// @size: size of fragment
//
// skb_frag_size_add() - Increments the size of a skb fragment by @delta
// @frag: skb fragment
// @delta: value to add
//
// skb_frag_size_sub() - Decrements the size of a skb fragment by @delta
// @frag: skb fragment
// @delta: value to subtract
//
// skb_frag_must_loop - Test if %p is a high memory page
// @p: fragment's page
//

//
// skb_frag_foreach_page - loop over pages in a fragment
//
// @f:		skb frag to operate on
// @f_off:		offset from start of f->netmem
// @f_len:		length from f_off to loop over
// @p:		(temp var) current page
// @p_off:		(temp var) offset from start of current page,
// non-zero only on first page.
// @p_len:		(temp var) length in current page,
// < PAGE_SIZE only on first and last page.
// @copied:	(temp var) length so far, excluding current p_len.
//
// A fragment can hold a compound page, in which case per-page
// operations, notably kmap_atomic, must be called for each
// regular page.
//

//
// struct skb_shared_hwtstamps - hardware time stamps
// @hwtstamp:		hardware time stamp transformed into duration
// since arbitrary point in time
// @netdev_data:	address/cookie of network device driver used as
// reference to actual hardware time stamp
//
// Software time stamps generated by ktime_get_real() are stored in
// skb->tstamp.
//
// hwtstamps can only be compared against other hwtstamps from
// the same device.
//
// This structure is attached to packets as part of the
// &skb_shared_info. Use skb_hwtstamps() to get a pointer.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct skb_shared_hwtstamps {
    pub hwtstamp: ktime_t,
    pub netdev_data: *mut c_void,
}

// Definitions for tx_flags in struct skb_shared_info
// generate hardware time stamp
// generate software time stamp when queueing packet to NIC
// device driver is going to provide hardware time stamp
// generate software time stamp on packet tx completion
// determine hardware time stamp based on time or cycles
// generate software time stamp when entering packet scheduling
// used for bpf extension when a bpf program is loaded

// Definitions for flags in struct skb_shared_info
// use zcopy routines
// This indicates at least one fragment might be overwritten
// (as in vmsplice(), sendfile() ...)
// If we need to compute a TX checksum, we'll need to copy
// all frags to avoid possible bad checksum
//
// segment contains only zerocopy data and should not be
// charged to the kernel memory.
//
// page references are managed by the ubuf_info, so it's safe to
// use frags only up until ubuf_info is released
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ubuf_info_ops {
    pub zerocopy_success): bool,
// has to be compatible with skb_zcopy_set()
    pub uarg): *mut *mut *mut int (link_skb)(struct sk_buff skb, struct ubuf_info,
}

//
// The callback notifies userspace to release buffers when skb DMA is done in
// lower device, the skb last reference should be 0 when calling this.
// The zerocopy_success argument is true if zero copy transmit occurred,
// false on data copy or out of memory error caused by data copy attempt.
// The ctx field is used to track device context.
// The desc field is used to track userspace buffer index.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ubuf_info {
    pub ops: *const ubuf_info_ops,
    pub refcnt: refcount_t,
    pub flags: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ubuf_info_msgzc {
    pub ubuf: ubuf_info,
    pub desc: c_ulong,
    pub ctx: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmpin {
    pub user: *mut user_struct,
    pub num_pg: c_uint,
    pub mmp: },
}

extern "C" {
    pub fn mm_account_pinned_pages(mmp: *mut mmpin, size: usize) -> c_int;
}
extern "C" {
    pub fn mm_unaccount_pinned_pages(mmp: *mut mmpin);
}
// Preserve some data across TX submission and completion.
//
// Note, this state is stored in the driver. Extending the layout
// might need some special care.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xsk_tx_metadata_compl {
    pub tx_timestamp: *mut __u64,
}

// This data is invariant across clones and lives at
// the end of the header data, ie. at skb->end.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct skb_shared_info {
    pub flags: __u8,
    pub meta_len: __u8,
    pub nr_frags: __u8,
    pub tx_flags: __u8,
    pub gso_size: c_ushort,
// Warning: this field is not always filled in (UFO)!
    pub gso_segs: c_ushort,
    pub frag_list: *mut sk_buff,
    pub hwtstamps: skb_shared_hwtstamps,
    pub xsk_meta: xsk_tx_metadata_compl,
}

//
// Warning : all fields before dataref are cleared in __alloc_skb()
//
// Intermediate layers must ensure that destructor_arg
// remains valid until skb destructor.
//
// must be last field, see pskb_expand_head()
//
// DOC: dataref and headerless skbs
//
// Transport layers send out clones of payload skbs they hold for
// retransmissions. To allow lower layers of the stack to prepend their headers
// we split &skb_shared_info.dataref into two halves.
// The lower 16 bits count the overall number of references.
// The higher 16 bits indicate how many of the references are payload-only.
// skb_header_cloned() checks if skb is allowed to add / write the headers.
//
// The creator of the skb (e.g. TCP) marks its skb as &sk_buff.nohdr
// (via __skb_header_release()). Any clone created from marked skb will get
// &sk_buff.hdr_len populated with the available headroom.
// If there's the only clone in existence it's able to modify the headroom
// at will. The sequence of calls inside the transport layer is::
//
// <alloc skb>
// skb_reserve()
// __skb_header_release()
// skb_clone()
// // send the clone down the stack
//
// This is not a very generic construct and it depends on the transport layers
// doing the right thing. In practice there's usually only one payload-only skb.
// Having multiple payload-only skbs with different lengths of hdr_len is not
// possible. The payload-only skbs should never leave their owner.
//
pub const SKB_DATAREF_SHIFT: c_int = 16;

// This indicates the skb is from an untrusted source.
// This indicates the tcp segment has CWR set.
// These indirectly map onto the same netdev feature.
// If NETIF_F_TSO_MANGLEID is set it may mangle both inner and outer IDs.
//

pub const NET_SKBUFF_DATA_USES_OFFSET: c_int = 1;

pub type sk_buff_data_t = c_uint;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum skb_tstamp_type {
    SKB_CLOCK_REALTIME,
    SKB_CLOCK_MONOTONIC,
    SKB_CLOCK_TAI,
    __SKB_CLOCK_MAX = SKB_CLOCK_TAI,
}

//
// DOC: Basic sk_buff geometry
//
// struct sk_buff itself is a metadata structure and does not hold any packet
// data. All the data is held in associated buffers.
//
// &sk_buff.head points to the main "head" buffer. The head buffer is divided
// into two parts:
//
// - data buffer, containing headers and sometimes payload;
// this is the part of the skb operated on by the common helpers
// such as skb_put() or skb_pull();
// - shared info (struct skb_shared_info) which holds an array of pointers
// to read-only data in the (page, offset, length) format.
//
// Optionally &skb_shared_info.frag_list may point to another skb.
//
// Basic diagram may look like this::
//
// ---------------
// | sk_buff       |
// ---------------
// ,---------------------------  + head
// /          ,-----------------  + data
// /          /      ,-----------  + tail
// |          |      |            , + end
// |          |      |           |
// v          v      v           v
// -----------------------------------------------
// | headroom | data |  tailroom | skb_shared_info |
// -----------------------------------------------
// + [page frag]
// + [page frag]       ---------
// + frag_list    --> | sk_buff |
// ---------
//
// struct sk_buff - socket buffer
// @next: Next buffer in list
// @prev: Previous buffer in list
// @tstamp: Time we arrived/left
// @skb_mstamp_ns: (aka @tstamp) earliest departure time; start point
// for retransmit timer
// @rbnode: RB tree node, alternative to next/prev for netem/tcp
// @list: queue head
// @ll_node: anchor in an llist (eg socket defer_list)
// @sk: Socket we are owned by
// @dev: Device we arrived on/are leaving by
// @dev_scratch: (aka @dev) alternate use of @dev when @dev would be %NULL
// @cb: Control buffer. Free for use by every layer. Put private vars here
// @_skb_refdst: destination entry (with norefcount bit)
// @len: Length of actual data
// @data_len: Data length
// @mac_len: Length of link layer header
// @hdr_len: writable header length of cloned skb
// @csum: Checksum (must include start/offset pair)
// @csum_start: Offset from skb->head where checksumming should start
// @csum_offset: Offset from csum_start where checksum should be stored
// @priority: Packet queueing priority
// @ignore_df: allow local fragmentation
// @cloned: Head may be cloned (check refcnt to be sure)
// @ip_summed: Driver fed us an IP checksum
// @nohdr: Payload reference only, must not modify header
// @pkt_type: Packet class
// @fclone: skbuff clone status
// @ipvs_property: skbuff is owned by ipvs
// @inner_protocol_type: whether the inner protocol is
// ENCAP_TYPE_ETHER or ENCAP_TYPE_IPPROTO
// @remcsum_offload: remote checksum offload is enabled
// @offload_fwd_mark: Packet was L2-forwarded in hardware
// @offload_l3_fwd_mark: Packet was L3-forwarded in hardware
// @tc_skip_classify: do not classify packet. set by IFB device
// @tc_at_ingress: used within tc_classify to distinguish in/egress
// @redirected: packet was redirected by packet classifier
// @from_ingress: packet was redirected from the ingress path
// @nf_skip_egress: packet shall skip nf egress - see netfilter_netdev.h
// @peeked: this packet has been seen already, so stats have been
// done for it, don't do them again
// @nf_trace: netfilter packet trace flag
// @protocol: Packet protocol from driver
// @destructor: Destruct function
// @tcp_tsorted_anchor: list structure for TCP (tp->tsorted_sent_queue)
// @_sk_redir: socket redirection information for skmsg
// @_nfct: Associated connection, if any (with nfctinfo bits)
// @skb_iif: ifindex of device we arrived on
// @tc_depth: counter for packet duplication
// @tc_index: Traffic control index
// @hash: the packet hash
// @queue_mapping: Queue mapping for multiqueue devices
// @head_frag: skb was allocated from page fragments,
// not allocated by kmalloc() or vmalloc().
// @pfmemalloc: skbuff was allocated from PFMEMALLOC reserves
// @pp_recycle: mark the packet for recycling instead of freeing (implies
// page_pool support on driver)
// @active_extensions: active extensions (skb_ext_id types)
// @ndisc_nodetype: router type (from link layer)
// @ooo_okay: allow the mapping of a socket to a queue to be changed
// @l4_hash: indicate hash is a canonical 4-tuple hash over transport
// ports.
// @sw_hash: indicates hash was computed in software stack
// @wifi_acked_valid: wifi_acked was set
// @wifi_acked: whether frame was acked on wifi or not
// @no_fcs:  Request NIC to treat last 4 bytes as Ethernet FCS
// @encapsulation: indicates the inner headers in the skbuff are valid
// @encap_hdr_csum: software checksum is needed
// @csum_valid: checksum is already valid
// @csum_not_inet: use CRC32c to resolve CHECKSUM_PARTIAL
// @csum_complete_sw: checksum was completed by software
// @csum_level: indicates the number of consecutive checksums found in
// the packet minus one that have been verified as
// CHECKSUM_UNNECESSARY (max 3)
// @unreadable: indicates that at least 1 of the fragments in this skb is
// unreadable.
// @dst_pending_confirm: need to confirm neighbour
// @decrypted: Decrypted SKB
// @slow_gro: state present at GRO time, slower prepare step required
// @tstamp_type: When set, skb->tstamp has the
// delivery_time clock base of skb->tstamp.
// @napi_id: id of the NAPI struct this skb came from
// @sender_cpu: (aka @napi_id) source CPU in XPS
// @alloc_cpu: CPU which did the skb allocation.
// @secmark: security marking
// @mark: Generic packet mark
// @reserved_tailroom: (aka @mark) number of bytes of free space available
// at the tail of an sk_buff
// @vlan_all: vlan fields (proto & tci)
// @vlan_proto: vlan encapsulation protocol
// @vlan_tci: vlan tag control information
// @inner_protocol: Protocol (encapsulation)
// @inner_ipproto: (aka @inner_protocol) stores ipproto when
// skb->inner_protocol_type == ENCAP_TYPE_IPPROTO;
// @inner_transport_header: Inner transport layer header (encapsulation)
// @inner_network_header: Network layer header (encapsulation)
// @inner_mac_header: Link layer header (encapsulation)
// @transport_header: Transport layer header
// @network_header: Network layer header
// @mac_header: Link layer header
// @kcov_handle: KCOV remote handle for remote coverage collection
// @tail: Tail pointer
// @end: End pointer
// @head: Head of buffer
// @data: Data head pointer
// @truesize: Buffer size
// @users: User count - see {datagram,tcp}.c
// @extensions: allocated extensions, valid if active_extensions is nonzero
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sk_buff {
// These two members must be first to match sk_buff_head.
    pub next: *mut sk_buff,
    pub prev: *mut sk_buff,
    pub dev: *mut net_device,
// Some protocols might use this space to store information,
// while device pointer would be NULL.
// UDP receive path is one user.
//
    pub dev_scratch: c_ulong,
}

//
// This is the control buffer. It is free to use for every
// layer. Please put your private variables there. If you
// want to keep them across layers you have to do a skb_clone()
// first. This is owned by whoever has the skb queued ATM.
//

// Following fields are _not_ copied in __copy_skb_header()
// Note that queue_mapping is here mostly to fill a hole.
//
// if you move cloned around you also must adapt those constants

pub const CLONED_MASK: c_int = 1;

// private:
// public:

// Fields enclosed in headers group are copied
// using a single memcpy() in __copy_skb_header()
//
// private:
// public:
// private:
// public:

// Indicates the inner headers are valid in the skbuff.

// These elements must be at the end, see alloc_skb() for details.
// data;

// only usable after checking ->active_extensions != 0

// if you move pkt_type around you also must adapt those constants

pub const PKT_TYPE_MAX: c_int = 7;

// if you move tc_at_ingress or tstamp_type
// around, you also must adapt these constants.
//

//
// Handling routines are only of interest to the kernel
//
pub const SKB_ALLOC_FCLONE: c_uint = 0x01;
pub const SKB_ALLOC_RX: c_uint = 0x02;
pub const SKB_ALLOC_NAPI: c_uint = 0x04;
//
// skb_pfmemalloc - Test if the skb was allocated from PFMEMALLOC reserves
// @skb: buffer
//
extern "C" {
    pub fn unlikely(_arg: skb->pfmemalloc) -> return;
}
//
// skb might have a dst pointer attached, refcounted or not.
// _skb_refdst low order bit is set if refcount was _not_ taken
//

//
// skb_dst - returns skb dst_entry
// @skb: buffer
//
// Returns: skb dst_entry, regardless of reference taken or not.
//
// If refdst was not refcounted, check we still are in a
// rcu_read_lock section
//
// skb_dstref_steal() - return current dst_entry value and clear it
// @skb: buffer
//
// Resets skb dst_entry without adjusting its reference count. Useful in
// cases where dst_entry needs to be temporarily reset and restored.
// Note that the returned value cannot be used directly because it
// might contain SKB_DST_NOREF bit.
//
// When in doubt, prefer skb_dst_drop() over skb_dstref_steal() to correctly
// handle dst_entry reference counting.
//
// Returns: original skb dst_entry.
//
// skb_dstref_restore() - restore skb dst_entry removed via skb_dstref_steal()
// @skb: buffer
// @refdst: dst entry from a call to skb_dstref_steal()
//
// skb_dst_set - sets skb dst
// @skb: buffer
// @dst: dst entry
//
// Sets skb dst, assuming a reference was taken on dst and should
// be released by skb_dst_drop()
//
// skb_dst_set_noref - sets skb dst, hopefully, without taking reference
// @skb: buffer
// @dst: dst entry
//
// Sets skb dst, assuming a reference was not taken on dst.
// If dst entry is cached, we do not take reference and dst_release
// will be avoided by refdst_drop. If dst entry is not cached, we take
// reference, so that last dst_release can destroy the dst immediately.
//
// skb_dst_is_noref - Test if skb dst isn't refcounted
// @skb: buffer
//
// For mangling skb->pkt_type from user space side from applications
// such as nft, tc, etc, we only allow a conservative subset of
// possible pkt_types to be set.
//
// skb_napi_id - Returns the skb's NAPI id
// @skb: buffer
//

//
// skb_unref - decrement the skb's reference count
// @skb: buffer
//
// Returns: true if we can free the skb.
//
// kfree_skb - free an sk_buff with 'NOT_SPECIFIED' reason
// @skb: buffer to free
//
extern "C" {
    pub fn skb_release_head_state(skb: *mut sk_buff);
}
extern "C" {
    pub fn skb_dump(level: *const c_char, skb: *const sk_buff, full_pkt: bool);
}
extern "C" {
    pub fn skb_tx_error(skb: *mut sk_buff);
}

extern "C" {
    pub fn consume_skb(skb: *mut sk_buff);
}

extern "C" {
    pub fn kfree_skb(_arg: skb) -> return;
}

extern "C" {
    pub fn __consume_stateless_skb(skb: *mut sk_buff);
}
extern "C" {
    pub fn __kfree_skb(skb: *mut sk_buff);
}
extern "C" {
    pub fn kfree_skb_partial(skb: *mut sk_buff, head_stolen: bool);
}
extern "C" {
    pub fn skb_attempt_defer_free(skb: *mut sk_buff);
}
extern "C" {
    pub fn napi_skb_cache_get_bulk(skbs: *mut c_void, n: u32) -> u32;
}
//
// alloc_skb - allocate a network buffer
// @size: size to allocate
// @priority: allocation mask
//
// This function is a convenient wrapper around __alloc_skb().
//
extern "C" {
    pub fn __alloc_skb(_arg: size, _arg: priority, _arg: 0, _arg: NUMA_NO_NODE) -> return;
}
// Layout of fast clones : [skb1][skb2][fclone_ref]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sk_buff_fclones {
    pub skb1: sk_buff,
    pub skb2: sk_buff,
    pub fclone_ref: refcount_t,
}

//
// skb_fclone_busy - check if fclone is busy
// @sk: socket
// @skb: buffer
//
// Returns: true if skb is a fast clone, and its clone is not freed.
// Some drivers call skb_orphan() in their ndo_start_xmit(),
// so we also check that didn't happen.
//
// alloc_skb_fclone - allocate a network buffer from fclone cache
// @size: size to allocate
// @priority: allocation mask
//
// This function is a convenient wrapper around __alloc_skb().
//
extern "C" {
    pub fn __alloc_skb(_arg: size, _arg: priority, _arg: SKB_ALLOC_FCLONE, _arg: NUMA_NO_NODE) -> return;
}
extern "C" {
    pub fn skb_headers_offset_update(skb: *mut sk_buff, off: c_int);
}
extern "C" {
    pub fn skb_copy_ubufs(skb: *mut sk_buff, gfp_mask: gfp_t) -> c_int;
}
extern "C" {
    pub fn skb_copy_header(new: *mut sk_buff, old: *const sk_buff);
}
extern "C" {
    pub fn __pskb_copy_fclone(_arg: skb, _arg: headroom, _arg: gfp_mask, _arg: false) -> return;
}
extern "C" {
    pub fn pskb_expand_head(skb: *mut sk_buff, nhead: c_int, ntail: c_int, gfp_mask: gfp_t) -> c_int;
}
extern "C" {
    pub fn skb_cow_data(skb: *mut sk_buff, tailbits: c_int, trailer: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn __skb_pad(skb: *mut sk_buff, pad: c_int, free_on_error: bool) -> c_int;
}
//
// skb_pad			-	zero pad the tail of an skb
// @skb: buffer to pad
// @pad: space to pad
//
// Ensure that a buffer is followed by a padding area that is zero
// filled. Used by network drivers which may DMA or transfer data
// beyond the buffer end onto the wire.
//
// May return error in out of memory cases. The skb is freed on error.
//
extern "C" {
    pub fn __skb_pad(_arg: skb, _arg: pad, _arg: true) -> return;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct skb_seq_state {
    pub lower_offset: __u32,
    pub upper_offset: __u32,
    pub frag_idx: __u32,
    pub stepped_offset: __u32,
    pub root_skb: *mut sk_buff,
    pub cur_skb: *mut sk_buff,
    pub frag_data: *mut __u8,
    pub frag_off: __u32,
}

extern "C" {
    pub fn skb_abort_seq_read(st: *mut skb_seq_state);
}
extern "C" {
    pub fn skb_copy_seq_read(st: *mut skb_seq_state, offset: c_int, to: *mut c_void, len: c_int) -> c_int;
}
//
// Packet hash types specify the type of hash in skb_set_hash.
//
// Hash types refer to the protocol layer addresses which are used to
// construct a packet's hash. The hashes are used to differentiate or identify
// flows of the protocol layer for the hash type. Hash types are either
// layer-2 (L2), layer-3 (L3), or layer-4 (L4).
//
// Properties of hashes:
//
// 1) Two packets in different flows have different hash values
// 2) Two packets in the same flow should have the same hash value
//
// A hash at a higher layer is considered to be more specific. A driver should
// set the most specific hash possible.
//
// A driver cannot indicate a more specific hash than the layer at which a hash
// was computed. For instance an L3 hash cannot be set as an L4 hash.
//
// A driver may indicate a hash level which is less specific than the
// actual layer the hash was computed on. For instance, a hash computed
// at L4 may be considered an L3 hash. This should only be done if the
// driver can't unambiguously determine that the HW computed the hash at
// the higher layer. Note that the "should" in the second property above
// permits this.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pkt_hash_types {
    PKT_HASH_TYPE_NONE,	/* Undefined type */
    PKT_HASH_TYPE_L2,	/* Input: src_MAC, dest_MAC */
    PKT_HASH_TYPE_L3,	/* Input: src_IP, dst_IP */
    PKT_HASH_TYPE_L4,	/* Input: src_IP, dst_IP, src_port, dst_port */
}

// Used by drivers to set hash from HW
extern "C" {
    pub fn __skb_get_hash_symmetric_net(net: *const net, skb: *const sk_buff) -> u32;
}
extern "C" {
    pub fn __skb_get_hash_symmetric_net(_arg: NULL, _arg: skb) -> return;
}
extern "C" {
    pub fn __skb_get_hash_net(net: *const net, skb: *mut sk_buff);
}
extern "C" {
    pub fn skb_get_poff(skb: *const sk_buff) -> u32;
}
// Gets a skb connection tracking info, ctinfo map should be a
// map of mapsize to translate enum ip_conntrack_info states
// to user states.
//

extern "C" {
    pub fn msg_zerocopy_put_abort(uarg: *mut ubuf_info, have_uref: bool);
}
// Internal

extern "C" {
    pub fn skb_zcopy_pure(skb_zcopy_pure(skb2: skb1) ==) -> return;
}
// have_ref = false;
// Release a reference on a zerocopy structure
extern "C" {
    pub fn __skb_zcopy_downgrade_managed(skb: *mut sk_buff);
}
// Return true if frags in this skb are readable by the host.

// Iterate through singly-linked GSO fragments of an skb.

//
// skb_queue_empty - check if a queue is empty
// @list: queue head
//
// Returns true if the queue is empty, false otherwise.
//
// skb_queue_empty_lockless - check if a queue is empty
// @list: queue head
//
// Returns true if the queue is empty, false otherwise.
// This variant can be used in lockless contexts.
//
// skb_queue_is_last - check if skb is the last entry in the queue
// @list: queue head
// @skb: buffer
//
// Returns true if @skb is the last buffer on the list.
//
// skb_queue_is_first - check if skb is the first entry in the queue
// @list: queue head
// @skb: buffer
//
// Returns true if @skb is the first buffer on the list.
//
// skb_queue_next - return the next packet in the queue
// @list: queue head
// @skb: current buffer
//
// Return the next packet in @list after @skb.  It is only valid to
// call this if skb_queue_is_last() evaluates to false.
//
// This BUG_ON may seem severe, but if we just return then we
// are going to dereference garbage.
//
// skb_queue_prev - return the prev packet in the queue
// @list: queue head
// @skb: current buffer
//
// Return the prev packet in @list before @skb.  It is only valid to
// call this if skb_queue_is_first() evaluates to false.
//
// This BUG_ON may seem severe, but if we just return then we
// are going to dereference garbage.
//
// skb_get - reference buffer
// @skb: buffer to reference
//
// Makes another reference to a socket buffer and returns a pointer
// to the buffer.
//
// If users == 1, we are the only owner and can avoid redundant atomic changes.
//
// skb_cloned - is the buffer a clone
// @skb: buffer to check
//
// Returns true if the buffer was generated with skb_clone() and is
// one of multiple shared copies of the buffer. Cloned buffers are
// shared data so must not be written to under normal circumstances.
//
extern "C" {
    pub fn pskb_expand_head(_arg: skb, _arg: 0, _arg: 0, _arg: pri) -> return;
}
// This variant of skb_unclone() makes sure skb->truesize
// and skb_end_offset() are not changed, whenever a new skb->head is needed.
//
// Indeed there is no guarantee that ksize(kmalloc(X)) == ksize(kmalloc(X))
// when various debugging features are in place.
//
extern "C" {
    pub fn __skb_unclone_keeptruesize(skb: *mut sk_buff, pri: gfp_t) -> c_int;
}
extern "C" {
    pub fn __skb_unclone_keeptruesize(_arg: skb, _arg: pri) -> return;
}
//
// skb_header_cloned - is the header a clone
// @skb: buffer to check
//
// Returns true if modifying the header part of the buffer requires
// the data to be copied.
//
extern "C" {
    pub fn pskb_expand_head(_arg: skb, _arg: 0, _arg: 0, _arg: pri) -> return;
}
//
// __skb_header_release() - allow clones to use the headroom
// @skb: buffer to operate on
//
// See "DOC: dataref and headerless skbs".
//
// skb_shared - is the buffer shared
// @skb: buffer to check
//
// Returns true if more than one person has a reference to this
// buffer.
//
// skb_share_check - check if buffer is shared and if so clone it
// @skb: buffer to check
// @pri: priority for memory allocation
//
// If the buffer is shared the buffer is cloned and the old copy
// drops a reference. A new clone with a single reference is returned.
// If the buffer is not shared the original buffer is returned. When
// being called from interrupt status or with spinlocks held pri must
// be GFP_ATOMIC.
//
// NULL is returned on a memory allocation failure.
//
// Copy shared buffers into a new sk_buff. We effectively do COW on
// packets to handle cases where we have a local reader and forward
// and a couple of other messy ones. The normal one is tcpdumping
// a packet that's being forwarded.
//
// skb_unshare - make a copy of a shared buffer
// @skb: buffer to check
// @pri: priority for memory allocation
//
// If the socket buffer is a clone then this function creates a new
// copy of the data, drops a reference count on the old copy and returns
// the new copy with the reference count at 1. If the buffer is not a clone
// the original buffer is returned. When called with a spinlock held or
// from interrupt state @pri must be %GFP_ATOMIC
//
// %NULL is returned on a memory allocation failure.
//
// Free our shared copy
//
// skb_peek - peek at the head of an &sk_buff_head
// @list_: list to peek at
//
// Peek an &sk_buff. Unlike most other operations you _MUST_
// be careful with this one. A peek leaves the buffer on the
// list and someone else may run off with it. You must hold
// the appropriate locks or have a private queue to do this.
//
// Returns %NULL for an empty list or a pointer to the head element.
// The reference count is not incremented and the reference is therefore
// volatile. Use with caution.
//
// __skb_peek - peek at the head of a non-empty &sk_buff_head
// @list_: list to peek at
//
// Like skb_peek(), but the caller knows that the list is not empty.
//
// skb_peek_next - peek skb following the given one from a queue
// @skb: skb to start from
// @list_: list to peek at
//
// Returns %NULL when the end of the list is met or a pointer to the
// next element. The reference count is not incremented and the
// reference is therefore volatile. Use with caution.
//
// skb_peek_tail - peek at the tail of an &sk_buff_head
// @list_: list to peek at
//
// Peek an &sk_buff. Unlike most other operations you _MUST_
// be careful with this one. A peek leaves the buffer on the
// list and someone else may run off with it. You must hold
// the appropriate locks or have a private queue to do this.
//
// Returns %NULL for an empty list or a pointer to the tail element.
// The reference count is not incremented and the reference is therefore
// volatile. Use with caution.
//
// skb_queue_len	- get queue length
// @list_: list to measure
//
// Return the length of an &sk_buff queue.
//
// skb_queue_len_lockless	- get queue length
// @list_: list to measure
//
// Return the length of an &sk_buff queue.
// This variant can be used in lockless contexts.
//
extern "C" {
    pub fn READ_ONCE(_arg: list_->qlen) -> return;
}
//
// __skb_queue_head_init - initialize non-spinlock portions of sk_buff_head
// @list: queue to initialize
//
// This initializes only the list and queue length aspects of
// an sk_buff_head object.  This allows to initialize the list
// aspects of an sk_buff_head without reinitializing things like
// the spinlock.  It can also be used for on-stack sk_buff_head
// objects where the spinlock is known to not be used.
//
// This function creates a split out lock class for each invocation;
// this is needed for now since a whole lot of users of the skb-queue
// infrastructure in drivers have different locking usage (in hardirq)
// than the networking core (in softirq only). In the long run either the
// network layer or drivers should need annotation to consolidate the
// main types of usage into 3 classes.
//
// Insert an sk_buff on a list.
//
// The "__skb_xxxx()" functions are the non-atomic ones that
// can only be called with interrupts disabled.
//
// See skb_queue_empty_lockless() and skb_peek_tail()
// for the opposite READ_ONCE()
//
// skb_queue_splice - join two skb lists, this is designed for stacks
// @list: the new list to add
// @head: the place to add it in the first list
//
// skb_queue_splice_init - join two skb lists and reinitialise the emptied list
// @list: the new list to add
// @head: the place to add it in the first list
//
// The list at @list is reinitialised
//
// skb_queue_splice_tail - join two skb lists, each list being a queue
// @list: the new list to add
// @head: the place to add it in the first list
//
// skb_queue_splice_tail_init - join two skb lists and reinitialise the emptied list
// @list: the new list to add
// @head: the place to add it in the first list
//
// Each of the lists is a queue.
// The list at @list is reinitialised
//
// __skb_queue_after - queue a buffer at the list head
// @list: list to use
// @prev: place after this buffer
// @newsk: buffer to queue
//
// Queue a buffer int the middle of a list. This function takes no locks
// and you must therefore hold required locks before calling it.
//
// A buffer cannot be placed on two lists at the same time.
//
// __skb_queue_head - queue a buffer at the list head
// @list: list to use
// @newsk: buffer to queue
//
// Queue a buffer at the start of a list. This function takes no locks
// and you must therefore hold required locks before calling it.
//
// A buffer cannot be placed on two lists at the same time.
//
extern "C" {
    pub fn skb_queue_head(list: *mut sk_buff_head, newsk: *mut sk_buff);
}
//
// __skb_queue_tail - queue a buffer at the list tail
// @list: list to use
// @newsk: buffer to queue
//
// Queue a buffer at the end of a list. This function takes no locks
// and you must therefore hold required locks before calling it.
//
// A buffer cannot be placed on two lists at the same time.
//
extern "C" {
    pub fn skb_queue_tail(list: *mut sk_buff_head, newsk: *mut sk_buff);
}
//
// remove sk_buff from list. _Must_ be called atomically, and with
// the list known..
//
extern "C" {
    pub fn skb_unlink(skb: *mut sk_buff, list: *mut sk_buff_head);
}
//
// __skb_dequeue - remove from the head of the queue
// @list: list to dequeue from
//
// Remove the head of the list. This function does not take any locks
// so must be used with appropriate locks held only. The head item is
// returned or %NULL if the list is empty.
//
// __skb_dequeue_tail - remove from the tail of the queue
// @list: list to dequeue from
//
// Remove the tail of the list. This function does not take any locks
// so must be used with appropriate locks held only. The tail item is
// returned or %NULL if the list is empty.
//
extern "C" {
    pub fn skb_headlen(__skb_pagelen(skb: skb) +) -> return;
}
//
// skb_len_add - adds a number to len fields of skb
// @skb: buffer to add len to
// @delta: number of bytes to add
//
// __skb_fill_netmem_desc - initialise a fragment in an skb
// @skb: buffer containing fragment to be initialised
// @i: fragment index to initialise
// @netmem: the netmem to use for this fragment
// @off: the offset to the data with @page
// @size: the length of the data
//
// Initialises the @i'th fragment of @skb to point to &size bytes at
// offset @off within @page.
//
// Does not take any additional reference on the fragment.
//
// Propagate page pfmemalloc to the skb if we can. The problem is
// that not all callers have unique ownership of the page but rely
// on page_is_pfmemalloc doing the right thing(tm).
//
// skb_fill_page_desc - initialise a paged fragment in an skb
// @skb: buffer containing fragment to be initialised
// @i: paged fragment index to initialise
// @page: the page to use for this fragment
// @off: the offset to the data with @page
// @size: the length of the data
//
// As per __skb_fill_page_desc() -- initialises the @i'th fragment of
// @skb to point to @size bytes at offset @off within @page. In
// addition updates @skb such that @i is the last fragment.
//
// Does not take any additional reference on the fragment.
//
// skb_fill_page_desc_noacc - initialise a paged fragment in an skb
// @skb: buffer containing fragment to be initialised
// @i: paged fragment index to initialise
// @page: the page to use for this fragment
// @off: the offset to the data with @page
// @size: the length of the data
//
// Variant of skb_fill_page_desc() which does not deal with
// pfmemalloc, if page is not owned by us.
//

extern "C" {
    pub fn skb_might_realloc(skb: *mut sk_buff);
}

//
// Add data to an sk_buff
//
// (u8 *)__skb_put(skb, 1) = val;
// (u8 *)skb_put(skb, 1) = val;

extern "C" {
    pub fn unlikely(__skb_pull(skb: len > skb->len) ? NULL :, _arg: len) -> return;
}
extern "C" {
    pub fn skb_condense(skb: *mut sk_buff);
}
//
// skb_headroom - bytes at buffer head
// @skb: buffer to check
//
// Return the number of bytes of free space at the head of an &sk_buff.
//
// skb_tailroom - bytes at buffer end
// @skb: buffer to check
//
// Return the number of bytes of free space at the tail of an sk_buff
//
// skb_availroom - bytes at buffer end
// @skb: buffer to check
//
// Return the number of bytes of free space at the tail of an sk_buff
// allocated by sk_stream_alloc()
//
// skb_reserve - adjust headroom
// @skb: buffer to alter
// @len: bytes to move
//
// Increase the headroom of an empty &sk_buff by reducing the tail
// room. This is only allowed for an empty buffer.
//
// skb_tailroom_reserve - adjust reserved_tailroom
// @skb: buffer to alter
// @mtu: maximum amount of headlen permitted
// @needed_tailroom: minimum amount of reserved_tailroom
//
// Set reserved_tailroom so that headlen can be as large as possible but
// not larger than mtu and tailroom cannot be smaller than
// needed_tailroom.
// The required headroom should already have been reserved before using
// this function.
//
// use at most mtu
// use up to all available space
pub const ENCAP_TYPE_ETHER: c_int = 0;
pub const ENCAP_TYPE_IPPROTO: c_int = 1;
// skb)
//
// skb_reset_transport_header_careful - conditionally reset transport header
// @skb: buffer to alter
//
// Hardened version of skb_reset_transport_header().
//
// Returns: true if the operation was a success.
//
// skb_set_transport_header_careful - conditionally set transport header
// @skb: buffer to alter
// @offset: offset to add to skb->data
//
// Hardened version of skb_set_transport_header().
//
// Returns: true if the operation was a success.
//
// Move the full mac header up to current network_header.
// Leaves skb->data pointing at offset skb->mac_len into the mac_header.
// Must be provided the complete mac header length.
//
extern "C" {
    pub fn pskb_may_pull_reason(_arg: skb, len: skb_network_offset(skb) +) -> return;
}
//
// CPUs often take a performance hit when accessing unaligned memory
// locations. The actual performance hit varies, it can be small if the
// hardware handles it or large if we have to take an exception and fix it
// in software.
//
// Since an ethernet header is 14 bytes network drivers often end up with
// the IP header at an unaligned offset. The IP header can be aligned by
// shifting the start of the packet by 2 bytes. Drivers should do this
// with:
//
// skb_reserve(skb, NET_IP_ALIGN);
//
// The downside to this alignment of the IP header is that the DMA is now
// unaligned. On some architectures the cost of an unaligned DMA is high
// and this cost outweighs the gains made by aligning the IP header.
//
// Since this trade off varies between architectures, we allow NET_IP_ALIGN
// to be overridden.
//

pub const NET_IP_ALIGN: c_int = 2;

//
// The networking layer reserves some headroom in skb data (via
// dev_alloc_skb). This is used to avoid having to reallocate skb data when
// the header has to grow. In the default case, if the header has to grow
// 32 bytes or less we avoid the reallocation.
//
// Unfortunately this headroom changes the DMA alignment of the resulting
// network packet. As for NET_IP_ALIGN, this unaligned DMA is expensive
// on some architectures. An architecture can override this value,
// perhaps setting it to a cacheline in size (since that will maintain
// cacheline alignment of the DMA). It must be a power of 2.
//
// Various parts of the networking layer expect at least 32 bytes of
// headroom, you should not reduce this.
//
// Using max(32, L1_CACHE_BYTES) makes sense (especially with RPS)
// to reduce average number of cache lines per packet.
// get_rps_cpu() for example only access one 64 bytes aligned block :
// NET_IP_ALIGN(2) + ethernet_header(14) + IP_header(20/40) + ports(8)
//

extern "C" {
    pub fn ___pskb_trim(skb: *mut sk_buff, len: c_uint) -> c_int;
}
extern "C" {
    pub fn skb_trim(skb: *mut sk_buff, len: c_uint);
}
extern "C" {
    pub fn ___pskb_trim(_arg: skb, _arg: len) -> return;
}
//
// pskb_trim_unique - remove end from a paged unique (not cloned) buffer
// @skb: buffer to alter
// @len: new length
//
// This is identical to pskb_trim except that the caller knows that
// the skb is not cloned so we should never get an error due to out-
// of-memory.
//
// skb_orphan - orphan a buffer
// @skb: buffer to orphan
//
// If a buffer currently has an owner then we call the owner's
// destructor function and make the @skb unowned. The buffer continues
// to exist but is no longer charged to its former owner.
//
// skb_orphan_frags - orphan the frags contained in a buffer
// @skb: buffer to orphan frags from
// @gfp_mask: allocation mask for replacement pages
//
// For each frag in the SKB which needs a destructor (i.e. has an
// owner) create a copy of that frag and release the original
// page by calling the destructor.
//
extern "C" {
    pub fn skb_copy_ubufs(_arg: skb, _arg: gfp_mask) -> return;
}
// Frags must be orphaned, even if refcounted, if skb might loop to rx path
extern "C" {
    pub fn skb_copy_ubufs(_arg: skb, _arg: gfp_mask) -> return;
}
//
// __skb_queue_purge_reason - empty a list
// @list: list to empty
// @reason: drop reason
//
// Delete all buffers on an &sk_buff list. Each buffer is removed from
// the list and one reference dropped. This function does not take the
// list lock and the caller must hold the relevant locks to use it.
//
extern "C" {
    pub fn skb_rbtree_purge(root: *mut rb_root) -> c_uint;
}
extern "C" {
    pub fn skb_errqueue_purge(list: *mut sk_buff_head);
}
//
// netdev_alloc_frag - allocate a page fragment
// @fragsz: fragment size
//
// Allocates a frag from a page for receive buffer.
// Uses GFP_ATOMIC allocations.
//
extern "C" {
    pub fn __netdev_alloc_frag_align(_arg: fragsz, _arg: ~0u) -> return;
}
extern "C" {
    pub fn __netdev_alloc_frag_align(_arg: fragsz, _arg: -align) -> return;
}
//
// netdev_alloc_skb - allocate an skbuff for rx on a specific device
// @dev: network device to receive on
// @length: length to allocate
//
// Allocate a new &sk_buff and assign it a usage count of one. The
// buffer has unspecified headroom built in. Users should allocate
// the headroom they think they need without accounting for the
// built in space. The built in space is used for optimisations.
//
// %NULL is returned if there is no free memory. Although this function
// allocates memory it can be called from an interrupt.
//
extern "C" {
    pub fn __netdev_alloc_skb(_arg: dev, _arg: length, _arg: GFP_ATOMIC) -> return;
}
// legacy helper around __netdev_alloc_skb()
extern "C" {
    pub fn __netdev_alloc_skb(_arg: NULL, _arg: length, _arg: gfp_mask) -> return;
}
// legacy helper around netdev_alloc_skb()
extern "C" {
    pub fn netdev_alloc_skb(_arg: NULL, _arg: length) -> return;
}
extern "C" {
    pub fn __netdev_alloc_skb_ip_align(_arg: dev, _arg: length, _arg: GFP_ATOMIC) -> return;
}
extern "C" {
    pub fn __napi_alloc_frag_align(_arg: fragsz, _arg: ~0u) -> return;
}
extern "C" {
    pub fn __napi_alloc_frag_align(_arg: fragsz, _arg: -align) -> return;
}
extern "C" {
    pub fn napi_consume_skb(skb: *mut sk_buff, budget: c_int);
}
extern "C" {
    pub fn napi_skb_free_stolen_head(skb: *mut sk_buff);
}
extern "C" {
    pub fn __napi_kfree_skb(skb: *mut sk_buff, reason: skb_drop_reason);
}
//
// __dev_alloc_pages - allocate page for network Rx
// @gfp_mask: allocation priority. Set __GFP_NOMEMALLOC if not for network Rx
// @order: size of the allocation
//
// Allocate a new page.
//
// %NULL is returned if there is no free memory.
//
// This piece of code contains several assumptions.
// 1.  This is for device Rx, therefore a cold page is preferred.
// 2.  The expectation is the user wants a compound page.
// 3.  If requesting a order 0 page it will not be compound
// due to the check to see if order has a value in prep_new_page
// 4.  __GFP_MEMALLOC is ignored if __GFP_NOMEMALLOC is set due to
// code in alloc_flags_slowpath() that should be enforcing this.
//
extern "C" {
    pub fn alloc_pages_node_noprof(_arg: NUMA_NO_NODE, _arg: gfp_mask, _arg: order) -> return;
}

//
// This specialized allocator has to be a macro for its allocations to be
// accounted separately (to have a separate alloc_tag).
//

//
// __dev_alloc_page - allocate a page for network Rx
// @gfp_mask: allocation priority. Set __GFP_NOMEMALLOC if not for network Rx
//
// Allocate a new page.
//
// %NULL is returned if there is no free memory.
//
extern "C" {
    pub fn __dev_alloc_pages_noprof(_arg: gfp_mask, _arg: 0) -> return;
}

//
// This specialized allocator has to be a macro for its allocations to be
// accounted separately (to have a separate alloc_tag).
//

//
// dev_page_is_reusable - check whether a page can be reused for network Rx
// @page: the page to test
//
// A page shouldn't be considered for reusing/recycling if it was allocated
// under memory pressure or at a distant memory node.
//
// Returns: false if this page should be returned to page allocator, true
// otherwise.
//
// skb_propagate_pfmemalloc - Propagate pfmemalloc if skb is allocated after RX page
// @page: The page that was allocated from skb_alloc_page
// @skb: The skb that may need pfmemalloc set
//
// skb_frag_off() - Returns the offset of a skb fragment
// @frag: the paged fragment
//
// skb_frag_off_add() - Increments the offset of a skb fragment by @delta
// @frag: skb fragment
// @delta: value to add
//
// skb_frag_off_set() - Sets the offset of a skb fragment
// @frag: skb fragment
// @offset: offset of fragment
//
// skb_frag_off_copy() - Sets the offset of a skb fragment from another fragment
// @fragto: skb fragment where offset is set
// @fragfrom: skb fragment offset is copied from
//
// Return: true if the skb_frag contains a net_iov.
extern "C" {
    pub fn netmem_is_net_iov(_arg: frag->netmem) -> return;
}
//
// skb_frag_net_iov - retrieve the net_iov referred to by fragment
// @frag: the fragment
//
// Return: the &struct net_iov associated with @frag. Returns NULL if this
// frag has no associated net_iov.
//
extern "C" {
    pub fn netmem_to_net_iov(_arg: frag->netmem) -> return;
}
//
// skb_frag_page - retrieve the page referred to by a paged fragment
// @frag: the paged fragment
//
// Return: the &struct page associated with @frag. Returns NULL if this frag
// has no associated page.
//
extern "C" {
    pub fn netmem_to_page(_arg: frag->netmem) -> return;
}
//
// skb_frag_netmem - retrieve the netmem referred to by a fragment
// @frag: the fragment
//
// Return: the &netmem_ref associated with @frag.
//
// skb_frag_address - gets the address of the data contained in a paged fragment
// @frag: the paged fragment buffer
//
// Returns: the address of the data within @frag. The page must already
// be mapped.
//
extern "C" {
    pub fn page_address(skb_frag_off(frag: skb_frag_page(frag)) +) -> return;
}
//
// skb_frag_address_safe - gets the address of the data contained in a paged fragment
// @frag: the paged fragment buffer
//
// Returns: the address of the data within @frag. Checks that the page
// is mapped and returns %NULL otherwise.
//
// skb_frag_phys - gets the physical address of the data in a paged fragment
// @frag: the paged fragment buffer
//
// Returns: the physical address of the data within @frag.
//
extern "C" {
    pub fn page_to_phys(skb_frag_off(frag: skb_frag_page(frag)) +) -> return;
}
//
// skb_frag_page_copy() - sets the page in a fragment from another fragment
// @fragto: skb fragment where page is set
// @fragfrom: skb fragment page is copied from
//
extern "C" {
    pub fn skb_page_frag_refill(sz: c_uint, pfrag: *mut page_frag, prio: gfp_t) -> bool;
}
//
// __skb_frag_dma_map - maps a paged fragment via the DMA API
// @dev: the device to map the fragment to
// @frag: the paged fragment to map
// @offset: the offset within the fragment (starting at the
// fragment's own offset)
// @size: the number of bytes to map
// @dir: the direction of the mapping (``PCI_DMA_*``)
//
// Maps the page associated with @frag to @device.
//

extern "C" {
    pub fn __pskb_copy(_arg: skb, _arg: skb_headroom(skb), _arg: gfp_mask) -> return;
}
extern "C" {
    pub fn __pskb_copy_fclone(_arg: skb, _arg: skb_headroom(skb), _arg: gfp_mask, _arg: true) -> return;
}
//
// skb_clone_writable - is the header of a clone writable
// @skb: buffer to check
// @len: length up to which to write
//
// Returns true if modifying the header part of the cloned buffer
// does not requires the data to be copied.
//
// skb_cow - copy header of skb when it is required
// @skb: buffer to cow
// @headroom: needed headroom
//
// If the skb passed lacks sufficient headroom or its data part
// is shared, data is reallocated. If reallocation fails, an error
// is returned and original skb is not changed.
//
// The result is skb with writable area skb->head...skb->tail
// and at least @headroom of space at head.
//
extern "C" {
    pub fn __skb_cow(_arg: skb, _arg: headroom, _arg: skb_cloned(skb)) -> return;
}
//
// skb_cow_head - skb_cow but only making the head writable
// @skb: buffer to cow
// @headroom: needed headroom
//
// This function is identical to skb_cow except that we replace the
// skb_cloned check by skb_header_cloned.  It should be used when
// you only need to push on some header and do not need to modify
// the data.
//
extern "C" {
    pub fn __skb_cow(_arg: skb, _arg: headroom, _arg: skb_header_cloned(skb)) -> return;
}
//
// skb_padto	- pad an skbuff up to a minimal size
// @skb: buffer to pad
// @len: minimal length
//
// Pads up a buffer to ensure the trailing bytes exist and are
// blanked. If the buffer already contains sufficient data it
// is untouched. Otherwise it is extended. Returns zero on
// success. The skb is freed on error.
//
extern "C" {
    pub fn skb_pad(_arg: skb, size: len -) -> return;
}
//
// __skb_put_padto - increase size and pad an skbuff up to a minimal size
// @skb: buffer to pad
// @len: minimal length
// @free_on_error: free buffer on error
//
// Pads up a buffer to ensure the trailing bytes exist and are
// blanked. If the buffer already contains sufficient data it
// is untouched. Otherwise it is extended. Returns zero on
// success. The skb is freed on error if @free_on_error is true.
//
// skb_put_padto - increase size and pad an skbuff up to a minimal size
// @skb: buffer to pad
// @len: minimal length
//
// Pads up a buffer to ensure the trailing bytes exist and are
// blanked. If the buffer already contains sufficient data it
// is untouched. Otherwise it is extended. Returns zero on
// success. The skb is freed on error.
//
extern "C" {
    pub fn __skb_put_padto(_arg: skb, _arg: len, _arg: true) -> return;
}
extern "C" {
    pub fn skb_can_coalesce_netmem(_arg: skb, _arg: i, _arg: page_to_netmem(page), _arg: off) -> return;
}
//
// skb_linearize - convert paged skb to linear one
// @skb: buffer to linarize
//
// If there is no free memory -ENOMEM is returned, otherwise zero
// is returned and the old skb data released.
//
// skb_has_shared_frag - can any frag be overwritten
// @skb: buffer to test
//
// Return: true if the skb has at least one frag that might be modified
// by an external entity (as in vmsplice()/sendfile())
//
// skb_linearize_cow - make sure skb is linear and writable
// @skb: buffer to process
//
// If there is no free memory -ENOMEM is returned, otherwise zero
// is returned and the old skb data released.
//
// skb_postpull_rcsum - update checksum for received skb after pull
// @skb: buffer to update
// @start: start of data before pull
// @len: length of data pulled
//
// After doing a pull on a received packet, you need to call this to
// update the CHECKSUM_COMPLETE checksum, or set ip_summed to
// CHECKSUM_NONE so that it can be recomputed from scratch.
//
// skb_postpush_rcsum - update checksum for received skb after push
// @skb: buffer to update
// @start: start of data after push
// @len: length of data pushed
//
// After doing a push on a received packet, you need to call this to
// update the CHECKSUM_COMPLETE checksum.
//
// skb_push_rcsum - push skb and update receive checksum
// @skb: buffer to update
// @len: length of data pulled
//
// This function performs an skb_push on the packet and updates
// the CHECKSUM_COMPLETE checksum.  It should be used on
// receive path processing instead of skb_push unless you know
// that the checksum difference is zero (e.g., a valid IP header)
// or you are setting ip_summed to CHECKSUM_NONE.
//
extern "C" {
    pub fn pskb_trim_rcsum_slow(skb: *mut sk_buff, len: c_uint) -> c_int;
}
//
// pskb_trim_rcsum - trim received skb and update checksum
// @skb: buffer to trim
// @len: new length
//
// This is exactly the same as pskb_trim except that it ensures the
// checksum of received packets are still valid after the operation.
// It can change skb pointers.
//
extern "C" {
    pub fn pskb_trim_rcsum_slow(_arg: skb, _arg: len) -> return;
}
extern "C" {
    pub fn __skb_grow(_arg: skb, _arg: len) -> return;
}

extern "C" {
    pub fn skb_copy_datagram_iter(_arg: from, _arg: offset, _arg: &msg->msg_iter, _arg: size) -> return;
}
extern "C" {
    pub fn zerocopy_sg_from_iter(skb: *mut sk_buff, frm: *mut iov_iter) -> c_int;
}
extern "C" {
    pub fn skb_free_datagram(sk: *mut sock, skb: *mut sk_buff);
}
extern "C" {
    pub fn skb_kill_datagram(sk: *mut sock, skb: *mut sk_buff, flags: c_uint) -> c_int;
}
extern "C" {
    pub fn skb_copy_bits(skb: *const sk_buff, offset: c_int, to: *mut c_void, len: c_int) -> c_int;
}
extern "C" {
    pub fn skb_store_bits(skb: *mut sk_buff, offset: c_int, from: *const c_void, len: c_int) -> c_int;
}
extern "C" {
    pub fn skb_send_sock(sk: *mut sock, skb: *mut sk_buff, offset: c_int, len: c_int) -> c_int;
}
extern "C" {
    pub fn skb_copy_and_csum_dev(skb: *const sk_buff, to: *mut u8);
}
extern "C" {
    pub fn skb_zerocopy_headlen(from: *const sk_buff) -> c_uint;
}
extern "C" {
    pub fn skb_split(skb: *mut sk_buff, skb1: *mut sk_buff, len: u32);
}
extern "C" {
    pub fn skb_shift(tgt: *mut sk_buff, skb: *mut sk_buff, shiftlen: c_int) -> c_int;
}
extern "C" {
    pub fn skb_scrub_packet(skb: *mut sk_buff, xnet: bool);
}
extern "C" {
    pub fn skb_ensure_writable(skb: *mut sk_buff, write_len: c_uint) -> c_int;
}
extern "C" {
    pub fn skb_ensure_writable_head_tail(skb: *mut sk_buff, dev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn __skb_vlan_pop(skb: *mut sk_buff, vlan_tci: *mut u16) -> c_int;
}
extern "C" {
    pub fn skb_vlan_pop(skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn skb_vlan_push(skb: *mut sk_buff, vlan_proto: __be16, vlan_tci: u16) -> c_int;
}
extern "C" {
    pub fn skb_eth_pop(skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn skb_mpls_update_lse(skb: *mut sk_buff, mpls_lse: __be32) -> c_int;
}
extern "C" {
    pub fn skb_mpls_dec_ttl(skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn skb_crc32c(skb: *const sk_buff, offset: c_int, len: c_int, crc: u32) -> u32;
}
// Variant of skb_header_pointer() where @offset is user-controlled
// and potentially negative.
//
extern "C" {
    pub fn skb_header_pointer(_arg: skb, _arg: offset, _arg: len, _arg: buffer) -> return;
}
//
// skb_needs_linearize - check if we need to linearize a given skb
// depending on the given device features.
// @skb: socket buffer to check
// @features: net device features
//
// Returns true if either:
// 1. skb has frag_list and the device doesn't support FRAGLIST, or
// 2. skb is fragmented and the device does not support SG.
//
extern "C" {
    pub fn skb_init();
}
//
// skb_get_timestamp - get timestamp from a skb
// @skb: skb to get stamp from
// @stamp: pointer to struct __kernel_old_timeval to store stamp in
//
// Timestamps are stored in the skb as offsets to a base timestamp.
// This function converts the offset back to a struct timeval and stores
// it in stamp.
//
// stamp = ns_to_kernel_old_timeval(skb->tstamp);
extern "C" {
    pub fn ktime_sub(_arg: ktime_get_real(), _arg: t) -> return;
}
// It is used in the ingress path to clear the delivery_time.
// If needed, set the skb->tstamp to the (rcv) timestamp.
//
extern "C" {
    pub fn ktime_get_real() -> return;
}
extern "C" {
    pub fn skb_mac_header(_arg: skb) -> return;
}
// Using more efficient variant than plain call to memcmp().

extern "C" {
    pub fn memcmp(meta_len: a -, meta_len: b -, _arg: meta_len) -> return;
}
//
// skb_data_move - Move packet data and metadata after skb_push() or skb_pull().
// @skb: packet to operate on
// @len: number of bytes pushed or pulled from &sk_buff->data
// @n: number of bytes to memmove() from pre-push/pull &sk_buff->data
//
// Moves @n bytes of packet data, can be zero, and all bytes of skb metadata.
//
// Assumes metadata is located immediately before &sk_buff->data prior to the
// push/pull, and that sufficient headroom exists to hold it after an
// skb_push(). Otherwise, metadata is cleared and a one-time warning is issued.
//
// Prefer skb_postpull_data_move() or skb_postpush_data_move() to calling this
// helper directly.
//
// skb_postpull_data_move - Move packet data and metadata after skb_pull().
// @skb: packet to operate on
// @len: number of bytes pulled from &sk_buff->data
// @n: number of bytes to memmove() from pre-pull &sk_buff->data
//
// See skb_data_move() for details.
//
// skb_postpush_data_move - Move packet data and metadata after skb_push().
// @skb: packet to operate on
// @len: number of bytes pushed onto &sk_buff->data
// @n: number of bytes to memmove() from pre-push &sk_buff->data
//
// See skb_data_move() for details.
//

extern "C" {
    pub fn skb_clone_tx_timestamp(skb: *mut sk_buff);
}
extern "C" {
    pub fn skb_defer_rx_timestamp(skb: *mut sk_buff) -> bool;
}

//
// skb_complete_tx_timestamp() - deliver cloned skb with tx timestamps
//
// PHY drivers may accept clones of transmitted packets for
// timestamping via their phy_driver.txtstamp method. These drivers
// must call this function to return the skb back to the stack with a
// timestamp.
//
// @skb: clone of the original outgoing packet
// @hwtstamps: hardware time stamps
//
// skb_tstamp_tx - queue clone of skb with send time stamps
// @orig_skb:	the original outgoing packet
// @hwtstamps:	hardware time stamps, may be NULL if not available
//
// If the skb has a socket associated, then this function clones the
// skb (thus sharing the actual data and optional structures), stores
// the optional hardware time stamping information (if non NULL) or
// generates a software time stamp (otherwise), then queues the clone
// to the error queue of the socket.  Errors are silently ignored.
//
// skb_tx_timestamp() - Driver hook for transmit timestamping
//
// Ethernet MAC Drivers should call this function in their hard_xmit()
// function immediately before giving the sk_buff to the MAC hardware.
//
// Specifically, one should make absolutely sure that this function is
// called before TX completion of this packet can trigger.  Otherwise
// the packet could potentially already be freed.
//
// @skb: A socket buffer.
//
// skb_complete_wifi_ack - deliver skb with wifi status
//
// @skb: the original outgoing packet
// @acked: ack status
//
extern "C" {
    pub fn skb_complete_wifi_ack(skb: *mut sk_buff, acked: bool);
}
extern "C" {
    pub fn __skb_checksum_complete_head(skb: *mut sk_buff, len: c_int) -> __sum16;
}
extern "C" {
    pub fn __skb_checksum_complete(skb: *mut sk_buff) -> __sum16;
}
//
// skb_checksum_complete - Calculate checksum of an entire packet
// @skb: packet to process
//
// This function calculates the checksum over the entire packet plus
// the value of skb->csum.  The latter can be used to supply the
// checksum of a pseudo header as used by TCP/UDP.  It returns the
// checksum.
//
// For protocols that contain complete checksums such as ICMP/TCP/UDP,
// this function can be used to verify that checksum on received
// packets.  In that case the function should return zero if the
// checksum is correct.  In particular, this function will return zero
// if skb->ip_summed is CHECKSUM_UNNECESSARY which indicates that the
// hardware has already verified the correctness of the checksum.
//
// Check if we need to perform checksum complete validation.
//
// Returns: true if checksum complete is needed, false otherwise
// (either checksum is unnecessary or zero checksum is allowed).
//
// For small packets <= CHECKSUM_BREAK perform checksum complete directly
// in checksum_init.
//
pub const CHECKSUM_BREAK: c_int = 76;
// Unset checksum-complete
//
// Unset checksum complete can be done when packet is being modified
// (uncompressed for instance) and checksum-complete value is
// invalidated.
//
// Validate (init) checksum based on checksum complete.
//
// Return values:
// 0: checksum is validated or try to in skb_checksum_complete. In the latter
// case the ip_summed will not be CHECKSUM_UNNECESSARY and the pseudo
// checksum is stored in skb->csum for use in __skb_checksum_complete
// non-zero: value of invalid checksum
//
// Perform checksum validate (init). Note that this is a macro since we only
// want to calculate the pseudo header which is an input function if necessary.
// First we try to validate without any computation (checksum unnecessary) and
// then calculate based on checksum complete calling the function to compute
// pseudo header.
//
// Return values:
// 0: checksum is validated or try to in skb_checksum_complete
// non-zero: value of invalid checksum
//

// Update skbuf and packet to reflect the remote checksum offload operation.
// When called, ptr indicates the starting point for skb->csum when
// ip_summed is CHECKSUM_COMPLETE. If we need create checksum complete
// here, skb_postpull_rcsum is done so skb->csum start is ptr.
//
// Adjust skb->csum since we changed the packet

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum skb_ext_id {

    SKB_EXT_BRIDGE_NF,

    SKB_EXT_SEC_PATH,

    TC_SKB_EXT,

    SKB_EXT_MPTCP,

    SKB_EXT_MCTP,

    SKB_EXT_PSP,

    SKB_EXT_CAN,

    SKB_EXT_NUM, /* must be last */
}

//
// struct skb_ext - sk_buff extensions
// @refcnt: 1 on allocation, deallocated on 0
// @offset: offset to add to @data to obtain extension address
// @chunks: size currently allocated, stored in SKB_EXT_ALIGN_SHIFT units
// @data: start of extension data, variable sized
//
// Note: offsets/lengths are stored in chunks of 8 bytes, this allows
// to use 'u8' types while allowing up to 2kb worth of extension data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct skb_ext {
    pub refcnt: refcount_t,
    pub /: *mut *mut u8 offset[SKB_EXT_NUM]; / in chunks of 8 bytes,
    pub /: *mut *mut u8 chunks; / same,
    pub __aligned(8): char data[],
}

extern "C" {
    pub fn __skb_ext_del(skb: *mut sk_buff, id: skb_ext_id);
}
extern "C" {
    pub fn __skb_ext_put(ext: *mut skb_ext);
}
extern "C" {
    pub fn unlikely(_arg: skb->active_extensions) -> return;
}

// Note: This doesn't put any conntrack info in dst.

extern "C" {
    pub fn skb_ext_exist(_arg: skb, _arg: SKB_EXT_SEC_PATH) -> return;
}

extern "C" {
    pub fn skb_ext_find(_arg: skb, _arg: SKB_EXT_SEC_PATH) -> return;
}

// Note: Should be called only if skb_is_gso(skb) is true
extern "C" {
    pub fn skb_shinfo(SKB_GSO_TCPV6: skb)->gso_type & (SKB_GSO_TCPV4 |) -> return;
}
extern "C" {
    pub fn __skb_warn_lro_forwarding(skb: *const sk_buff);
}
// LRO sets gso_size but not gso_type, whereas if GSO is really
// wanted then gso_type will be set.
// Unfortunately we don't support this one.  Any brave souls?
//
// skb_checksum_none_assert - make sure skb ip_summed is CHECKSUM_NONE
// @skb: skb to check
//
// fresh skbs have their ip_summed set to CHECKSUM_NONE.
// Instead of forcing ip_summed to CHECKSUM_NONE, we can
// use this helper, to document places where we make this assertion.
//
extern "C" {
    pub fn skb_partial_csum_set(skb: *mut sk_buff, start: u16, off: u16) -> bool;
}
extern "C" {
    pub fn skb_checksum_setup(skb: *mut sk_buff, recalculate: bool) -> c_int;
}
//
// skb_head_is_locked - Determine if the skb->head is locked down
// @skb: skb to check
//
// The head on skbs build around a head frag can be removed if they are
// not cloned.  This function returns true if the skb head is locked down
// due to either being allocated via kmalloc, or by being a clone with
// multiple references to the head.
//
// Local Checksum Offload.
// Compute outer checksum based on the assumption that the
// inner checksum will be offloaded later.
// See Documentation/networking/checksum-offloads.rst for
// explanation of how this works.
// Fill in outer checksum adjustment (e.g. with sum of outer
// pseudo-header) before calling.
// Also ensure that inner checksum is in linear data area.
//
// Start with complement of inner checksum adjustment
// Add in checksum of our headers (incl. outer checksum
// adjustment filled in by caller) and return result.
//
extern "C" {
    pub fn csum_partial(_arg: l4_hdr, l4_hdr: csum_start -, _arg: partial) -> return;
}

