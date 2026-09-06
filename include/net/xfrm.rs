//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/xfrm.h
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

pub const XFRM_PROTO_ESP: c_int = 50;
pub const XFRM_PROTO_AH: c_int = 51;
pub const XFRM_PROTO_COMP: c_int = 108;
pub const XFRM_PROTO_IPIP: c_int = 4;
pub const XFRM_PROTO_IPV6: c_int = 41;

// Organization of SPD aka "XFRM rules"
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_state_walk {
    pub all: list_head,
    pub state: u8,
    pub dying: u8,
    pub proto: u8,
    pub seq: u32,
    pub filter: *mut xfrm_address_filter,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_dev_offload {
// The device for this offload.
// Device drivers should not use this directly, as that will prevent
// them from working with bonding device. Instead, the device passed
// to the add/delete callbacks should be used.
//
    pub dev: *mut net_device,
    pub dev_tracker: netdevice_tracker,
// This is a private pointer used by the bonding driver (and eventually
// should be moved there). Device drivers should not use it.
// Protected by xfrm_state.lock AND bond.ipsec_lock in most cases,
// except in the .xdo_dev_state_del() flow, where only xfrm_state.lock
// is held.
//
    pub real_dev: *mut net_device,
    pub offload_handle: c_ulong,
// Snapshot the attached device index for dump paths.
    pub ifindex: c_int,
    pub 2: u8 dir :,
    pub 2: u8 type :,
    pub 2: u8 flags :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_mode {
    pub encap: u8,
    pub family: u8,
    pub flags: u8,
}

// Flags for xfrm_mode.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xfrm_replay_mode {
    XFRM_REPLAY_MODE_LEGACY,
    XFRM_REPLAY_MODE_BMP,
    XFRM_REPLAY_MODE_ESN,
}

// Full description of state of transformer.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_state {
    pub xs_net: possible_net_t,
    pub gclist: hlist_node,
    pub bydst: hlist_node,
}

// Key manager bits
// Parameters of this state.
// Data for transformer
// mapping change rate limiting
// Data for encapsulator
// NAT keepalive
// Data for care-of address
// IPComp needs an IPIP tunnel for handling uncompressed packets
// If a tunnel, number of users + 1
// State for replay detection
// Replay detection state at the time we sent the last notification
// replay detection mode
// internal flag that only holds state for delayed aevent at the
// moment
//
// Replay detection notification settings
// Replay detection notification timer
// Statistics
// used to fix curlft->add_time when changing date
// Last used time
// Reference to data common to all the instances of this
// transformer.
// Security context
// Private data of this transformer, format is opaque,
// interpreted by xfrm_type methods.
extern "C" {
    pub fn read_pnet(_arg: &x->xs_net) -> return;
}
// xflags - make enum if more show up
pub const XFRM_TIME_DEFER: c_int = 1;
pub const XFRM_SOFT_EXPIRE: c_int = 2;
// callback structure passed from either netlink or pfkey
#[repr(C)]
#[derive(Copy, Clone)]
pub struct km_event {
    pub hard: u32,
    pub proto: u32,
    pub byid: u32,
    pub aevent: u32,
    pub type: u32,
    pub data: },
    pub seq: u32,
    pub portid: u32,
    pub event: u32,
    pub net: *mut net,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_if_decode_session_result {
    pub net: *mut net,
    pub if_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_if_cb {
    pub res): *mut xfrm_if_decode_session_result,
}

extern "C" {
    pub fn xfrm_if_register_cb(ifcb: *const xfrm_if_cb);
}
extern "C" {
    pub fn xfrm_if_unregister_cb();
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_dst_lookup_params {
    pub net: *mut net,
    pub dscp: dscp_t,
    pub oif: c_int,
    pub saddr: *mut xfrm_address_t,
    pub daddr: *mut xfrm_address_t,
    pub mark: u32,
    pub ipproto: __u8,
    pub uli: flowi_uli,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_policy_afinfo {
    pub dst_ops: *mut dst_ops,
    pub params): *const *const *const dst_entry (dst_lookup)(xfrm_dst_lookup_params,
    pub params): *const xfrm_dst_lookup_params,
    pub fl): *const flowi,
    pub orig): *mut *mut *mut *mut dst_entry (blackhole_route)(net net, dst_entry,
}

extern "C" {
    pub fn xfrm_policy_register_afinfo(afinfo: *const xfrm_policy_afinfo, family: c_int) -> c_int;
}
extern "C" {
    pub fn xfrm_policy_unregister_afinfo(afinfo: *const xfrm_policy_afinfo);
}
extern "C" {
    pub fn km_state_notify(x: *mut xfrm_state, c: *const km_event);
}
extern "C" {
    pub fn km_state_expired(x: *mut xfrm_state, hard: c_int, portid: u32);
}
extern "C" {
    pub fn __xfrm_state_delete(x: *mut xfrm_state) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_state_afinfo {
    pub family: u8,
    pub proto: u8,
    pub type_offload_esp: *const xfrm_type_offload,
    pub type_esp: *const xfrm_type,
    pub type_ipip: *const xfrm_type,
    pub type_ipip6: *const xfrm_type,
    pub type_comp: *const xfrm_type,
    pub type_ah: *const xfrm_type,
    pub type_routing: *const xfrm_type,
    pub type_dstopts: *const xfrm_type,
    pub skb): *mut *mut *mut *mut int (output)(struct net net, struct sock sk, struct sk_buff,
    pub async): c_int,
    pub mtu): *mut *mut *mut void (local_error)(struct sk_buff skb, u32,
}

extern "C" {
    pub fn xfrm_state_register_afinfo(afinfo: *mut xfrm_state_afinfo) -> c_int;
}
extern "C" {
    pub fn xfrm_state_unregister_afinfo(afinfo: *mut xfrm_state_afinfo) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_input_afinfo {
    pub family: u8,
    pub is_ipip: bool,
    pub err): c_int,
}

extern "C" {
    pub fn xfrm_input_register_afinfo(afinfo: *const xfrm_input_afinfo) -> c_int;
}
extern "C" {
    pub fn xfrm_input_unregister_afinfo(afinfo: *const xfrm_input_afinfo) -> c_int;
}
extern "C" {
    pub fn xfrm_flush_gc();
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_type {
    pub owner: *mut module,
    pub proto: u8,
    pub flags: u8,
pub const XFRM_TYPE_NON_FRAGMENT: c_int = 1;
pub const XFRM_TYPE_REPLAY_PROT: c_int = 2;
pub const XFRM_TYPE_LOCAL_COADDR: c_int = 4;
pub const XFRM_TYPE_REMOTE_COADDR: c_int = 8;
    pub extack): *mut netlink_ext_ack,
    pub ): *mut *mut void (destructor)(struct xfrm_state,
    pub skb): *mut *mut *mut int (input)(struct xfrm_state , struct sk_buff,
    pub pskb): *mut *mut *mut int (output)(struct xfrm_state , struct sk_buff,
    pub ): *const flowi,
}

extern "C" {
    pub fn xfrm_register_type(type: *const xfrm_type, family: c_ushort) -> c_int;
}
extern "C" {
    pub fn xfrm_unregister_type(type: *const xfrm_type, family: c_ushort);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_type_offload {
    pub owner: *mut module,
    pub proto: u8,
    pub pskb): *mut *mut *mut void (encap)(struct xfrm_state , struct sk_buff,
    pub skb): *mut *mut *mut int (input_tail)(struct xfrm_state x, struct sk_buff,
    pub features): *mut *mut *mut *mut int (xmit)(struct xfrm_state , struct sk_buff pskb, netdev_features_t,
}

extern "C" {
    pub fn xfrm_register_type_offload(type: *const xfrm_type_offload, family: c_ushort) -> c_int;
}
extern "C" {
    pub fn xfrm_unregister_type_offload(type: *const xfrm_type_offload, family: c_ushort);
}
extern "C" {
    pub fn xfrm_set_type_offload(x: *mut xfrm_state, try_load: bool);
}
//
// struct xfrm_mode_cbs - XFRM mode callbacks
// @owner: module owner or NULL
// @init_state: Add/init mode specific state in `xfrm_state *x`
// @clone_state: Copy mode specific values from `orig` to new state `x`
// @destroy_state: Cleanup mode specific state from `xfrm_state *x`
// @user_init: Process mode specific netlink attributes from user
// @copy_to_user: Add netlink attributes to `attrs` based on state in `x`
// @sa_len: Return space required to store mode specific netlink attributes
// @get_inner_mtu: Return avail payload space after removing encap overhead
// @input: Process received packet from SA using mode
// @output: Output given packet using mode
// @prepare_output: Add mode specific encapsulation to packet in skb. On return
// `transport_header` should point at ESP header, `network_header` should
// point at outer IP header and `mac_header` should opint at the
// protocol/nexthdr field of the outer IP.
//
// One should examine and understand the specific uses of these callbacks in
// xfrm for further detail on how and when these functions are called. RTSL.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_mode_cbs {
    pub owner: *mut module,
    pub x): *mut *mut int (init_state)(struct xfrm_state,
    pub orig): *mut *mut *mut int (clone_state)(struct xfrm_state x, struct xfrm_state,
    pub x): *mut *mut void (destroy_state)(struct xfrm_state,
    pub extack): *mut netlink_ext_ack,
    pub skb): *mut *mut *mut int (copy_to_user)(struct xfrm_state x, struct sk_buff,
    pub x): *const *const unsigned int (sa_len)(struct xfrm_state,
    pub outer_mtu): *mut *mut *mut u32 (get_inner_mtu)(struct xfrm_state x, int,
    pub skb): *mut *mut *mut int (input)(struct xfrm_state x, struct sk_buff,
    pub skb): *mut *mut *mut *mut int (output)(struct net net, struct sock sk, struct sk_buff,
    pub skb): *mut *mut *mut int (prepare_output)(struct xfrm_state x, struct sk_buff,
}

extern "C" {
    pub fn xfrm_register_mode_cbs(mode: u8, mode_cbs: *const xfrm_mode_cbs) -> c_int;
}
extern "C" {
    pub fn xfrm_unregister_mode_cbs(mode: u8);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_tmpl {
// id in template is interpreted as:
// daddr - destination of tunnel, may be zero for transport mode.
// spi   - zero to acquire spi. Not zero if spi is static, then
// daddr must be fixed too.
// proto - AH/ESP/IPCOMP
//
    pub id: xfrm_id,
// Source address of tunnel. Ignored, if it is not a tunnel.
    pub saddr: xfrm_address_t,
    pub encap_family: c_ushort,
    pub reqid: u32,
// Mode: transport, tunnel etc.
    pub mode: u8,
// Sharing mode: unique, this session only, this user only etc.
    pub share: u8,
// May skip this transfomration if no SA is found
    pub optional: u8,
// Skip aalgos/ealgos/calgos checks.
    pub allalgs: u8,
// Bit mask of algos allowed for acquisition
    pub aalgos: u32,
    pub ealgos: u32,
    pub calgos: u32,
}

pub const XFRM_MAX_DEPTH: c_int = 6;
pub const XFRM_MAX_OFFLOAD_DEPTH: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_policy_walk_entry {
    pub all: list_head,
    pub dead: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_policy_walk {
    pub walk: xfrm_policy_walk_entry,
    pub type: u8,
    pub seq: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_policy_queue {
    pub hold_queue: sk_buff_head,
    pub hold_timer: timer_list,
    pub timeout: c_ulong,
}

//
// struct xfrm_policy - xfrm policy
// @xp_net: network namespace the policy lives in
// @bydst: hlist node for SPD hash table or rbtree list
// @byidx: hlist node for index hash table
// @state_cache_list: hlist head for policy cached xfrm states
// @lock: serialize changes to policy structure members
// @refcnt: reference count, freed once it reaches 0
// @pos: kernel internal tie-breaker to determine age of policy
// @timer: timer
// @genid: generation, used to invalidate old policies
// @priority: priority, set by userspace
// @index:  policy index (autogenerated)
// @if_id: virtual xfrm interface id
// @mark: packet mark
// @selector: selector
// @lft: liftime configuration data
// @curlft: liftime state
// @walk: list head on pernet policy list
// @polq: queue to hold packets while aqcuire operaion in progress
// @bydst_reinsert: policy tree node needs to be merged
// @type: XFRM_POLICY_TYPE_MAIN or _SUB
// @action: XFRM_POLICY_ALLOW or _BLOCK
// @flags: XFRM_POLICY_LOCALOK, XFRM_POLICY_ICMP
// @xfrm_nr: number of used templates in @xfrm_vec
// @family: protocol family
// @security: SELinux security label
// @xfrm_vec: array of templates to resolve state
// @rcu: rcu head, used to defer memory release
// @xdo: hardware offload state
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_policy {
    pub xp_net: possible_net_t,
    pub bydst: hlist_node,
    pub byidx: hlist_node,
    pub state_cache_list: hlist_head,
// This lock only affects elements except for entry.
    pub lock: rwlock_t,
    pub refcnt: refcount_t,
    pub pos: u32,
    pub timer: timer_list,
    pub genid: core::sync::atomic::AtomicI32,
    pub priority: u32,
    pub index: u32,
    pub if_id: u32,
    pub mark: xfrm_mark,
    pub selector: xfrm_selector,
    pub lft: xfrm_lifetime_cfg,
    pub curlft: xfrm_lifetime_cur,
    pub walk: xfrm_policy_walk_entry,
    pub polq: xfrm_policy_queue,
    pub bydst_reinsert: bool,
    pub type: u8,
    pub action: u8,
    pub flags: u8,
    pub xfrm_nr: u8,
    pub family: u16,
    pub security: *mut xfrm_sec_ctx,
    pub xfrm_vec: [xfrm_tmpl; XFRM_MAX_DEPTH],
    pub rcu: rcu_head,
    pub xdo: xfrm_dev_offload,
}

extern "C" {
    pub fn read_pnet(_arg: &xp->xp_net) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_kmaddress {
    pub local: xfrm_address_t,
    pub remote: xfrm_address_t,
    pub reserved: u32,
    pub family: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_migrate {
    pub old_daddr: xfrm_address_t,
    pub old_saddr: xfrm_address_t,
    pub new_daddr: xfrm_address_t,
    pub new_saddr: xfrm_address_t,
    pub encap: *mut xfrm_encap_tmpl,
    pub xuo: *mut xfrm_user_offload,
    pub old_mark: xfrm_mark,
    pub new_mark: *const xfrm_mark,
    pub smark: xfrm_mark,
    pub proto: u8,
    pub mode: u8,
    pub /: *mut *mut u16 msg_type; / XFRM_MSG_MIGRATE or XFRM_MSG_MIGRATE_STATE,
    pub flags: u32,
    pub old_reqid: u32,
    pub new_reqid: u32,
    pub nat_keepalive_interval: u32,
    pub mapping_maxage: u32,
    pub old_family: u16,
    pub new_family: u16,
    pub new_sel: *const xfrm_selector,
}

pub const XFRM_KM_TIMEOUT: c_int = 30;
// what happened

// default aevent timeout in units of 100ms
pub const XFRM_AE_ETIME: c_int = 10;
// Async Event timer multiplier
pub const XFRM_AE_ETH_M: c_int = 10;
// default seq threshold size
pub const XFRM_AE_SEQT_SIZE: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_mgr {
    pub list: list_head,
    pub c): *const *const *const int (notify)(struct xfrm_state x, struct km_event,
    pub xp): *mut *mut *mut *mut int (acquire)(struct xfrm_state x, struct xfrm_tmpl , struct xfrm_policy,
    pub dir): *mut *mut *mut *mut *mut xfrm_policy (compile_policy)(sock sk, int opt, u8 data, int len, int,
    pub sport): *mut *mut *mut *mut int (new_mapping)(struct xfrm_state x, xfrm_address_t ipaddr, __be16,
    pub c): *const *const *const int (notify_policy)(struct xfrm_policy x, int dir, struct km_event,
    pub addr): *mut *mut *mut *mut int (report)(struct net net, u8 proto, struct xfrm_selector sel, xfrm_address_t,
    pub encap): *const xfrm_encap_tmpl,
    pub c): *const *const bool (is_alive)(struct km_event,
}

extern "C" {
    pub fn xfrm_register_km(km: *mut xfrm_mgr);
}
extern "C" {
    pub fn xfrm_unregister_km(km: *mut xfrm_mgr);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_tunnel_skb_cb {
    pub h4: inet_skb_parm,
    pub h6: inet6_skb_parm,
    pub header: },
    pub ip4: *mut ip_tunnel,
    pub ip6: *mut ip6_tnl,
    pub tunnel: },
}

//
// This structure is used for the duration where packets are being
// transformed by IPsec.  As soon as the packet leaves IPsec the
// area beyond the generic IP part may be overwritten.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_skb_cb {
    pub header: xfrm_tunnel_skb_cb,
// Sequence number for replay protection.
    pub low: __u32,
    pub hi: __u32,
    pub output: },
    pub low: __be32,
    pub hi: __be32,
    pub input: },
    pub seq: },
}

//
// This structure is used by the afinfo prepare_input/prepare_output functions
// to transmit header information to the mode input/output functions.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_mode_skb_cb {
    pub header: xfrm_tunnel_skb_cb,
// Copied from header for IPv4, always set to zero and DF for IPv6.
    pub id: __be16,
    pub frag_off: __be16,
// IP header length (excluding options or extension headers).
    pub ihl: u8,
// TOS for IPv4, class for IPv6.
    pub tos: u8,
// TTL for IPv4, hop limitfor IPv6.
    pub ttl: u8,
// Protocol for IPv4, NH for IPv6.
    pub protocol: u8,
// Option length for IPv4, zero for IPv6.
    pub optlen: u8,
// Used by IPv6 only, zero for IPv4.
    pub flow_lbl: [u8; 3],
}

//
// This structure is used by the input processing to locate the SPI and
// related information.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_spi_skb_cb {
    pub header: xfrm_tunnel_skb_cb,
    pub daddroff: c_uint,
    pub family: c_uint,
    pub seq: __be32,
}

extern "C" {
    pub fn xfrm_audit_policy_add(xp: *mut xfrm_policy, result: c_int, task_valid: bool);
}
extern "C" {
    pub fn xfrm_audit_state_add(x: *mut xfrm_state, result: c_int, task_valid: bool);
}
extern "C" {
    pub fn xfrm_audit_state_delete(x: *mut xfrm_state, result: c_int, task_valid: bool);
}
extern "C" {
    pub fn xfrm_audit_state_notfound_simple(skb: *mut sk_buff, family: u16);
}

extern "C" {
    pub fn xfrm_policy_destroy(policy: *mut xfrm_policy);
}
extern "C" {
    pub fn __xfrm_state_destroy(: *mut xfrm_state);
}
// C99 6.5.7 (3): u32 << 32 is undefined behaviour

// If neither has a context --> match
// Otherwise, both must have a context and the sids, doi, alg must match
//

// A struct encoding bundle of transformations to apply to some set of flow.
//
// xdst->child points to the next element of bundle.
// dst->xfrm  points to an instanse of transformer.
//
// Due to unfortunate limitations of current routing cache, which we
// have no time to fix, it mirrors struct rtable and bound to the same
// routing key, including saddr,daddr. However, we can have many of
// bundles differing by session id. All the bundles grow from a parent
// policy rule.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_dst {
    pub dst: dst_entry,
    pub rt: rtable,
    pub rt6: rt6_info,
    pub u: },
    pub route: *mut dst_entry,
    pub child: *mut dst_entry,
    pub path: *mut dst_entry,
    pub pols: [*mut xfrm_policy; XFRM_POLICY_TYPE_MAX],
    pub num_xfrms: int num_pols,,
    pub xfrm_genid: u32,
    pub policy_genid: u32,
    pub route_mtu_cached: u32,
    pub child_mtu_cached: u32,
    pub route_cookie: u32,
    pub path_cookie: u32,
}

extern "C" {
    pub fn xfrm_dst_ifdown(dst: *mut dst_entry, dev: *mut net_device);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_if_parms {
    pub /: *mut *mut int link; / ifindex of underlying L2 interface,
    pub /: *mut *mut u32 if_id; / interface identifier,
    pub collect_md: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_if {
    pub /: *mut *mut *mut xfrm_if __rcu next; / next interface in list,
    pub /: *mut *mut *mut net_device dev; / virtual device associated with interface,
    pub /: *mut *mut *mut net net; / netns for packet i/o,
    pub /: *mut *mut xfrm_if_parms p; / interface parms,
    pub gro_cells: gro_cells,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_offload {
// Output sequence number for replay protection on offloading.
    pub low: __u32,
    pub hi: __u32,
    pub seq: },
    pub flags: __u32,
pub const SA_DELETE_REQ: c_int = 1;
pub const CRYPTO_DONE: c_int = 2;
pub const CRYPTO_NEXT_DONE: c_int = 4;
pub const CRYPTO_FALLBACK: c_int = 8;
pub const XFRM_GSO_SEGMENT: c_int = 16;
pub const XFRM_GRO: c_int = 32;
// 64 is free
pub const XFRM_DEV_RESUME: c_int = 128;
pub const XFRM_XMIT: c_int = 256;
    pub status: __u32,
pub const CRYPTO_SUCCESS: c_int = 1;
pub const CRYPTO_GENERIC_ERROR: c_int = 2;
pub const CRYPTO_TRANSPORT_AH_AUTH_FAILED: c_int = 4;
pub const CRYPTO_TRANSPORT_ESP_AUTH_FAILED: c_int = 8;
pub const CRYPTO_TUNNEL_AH_AUTH_FAILED: c_int = 16;
pub const CRYPTO_TUNNEL_ESP_AUTH_FAILED: c_int = 32;
pub const CRYPTO_INVALID_PACKET_SYNTAX: c_int = 64;
pub const CRYPTO_INVALID_PROTOCOL: c_int = 128;
// Used to keep whole l2 header for transport mode GRO
    pub orig_mac_len: __u16,
    pub proto: __u8,
    pub inner_ipproto: __u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sec_path {
    pub xvec: [*mut xfrm_state; XFRM_MAX_DEPTH],
    pub ovec: [xfrm_offload; XFRM_MAX_OFFLOAD_DEPTH],
    pub len: u8,
    pub olen: u8,
    pub verified_cnt: u8,
}

extern "C" {
    pub fn ipv6_addr_any(_arg: &addr->in6) -> return;
}
extern "C" {
    pub fn __xfrm4_state_addr_cmp(_arg: tmpl, _arg: x) -> return;
}
extern "C" {
    pub fn __xfrm6_state_addr_cmp(_arg: tmpl, _arg: x) -> return;
}

// same dst may be used for traffic originating from
// devices with different policy settings.
//
extern "C" {
    pub fn skb_dst(DST_NOPOLICY: skb) && (skb_dst(skb)->flags &) -> return;
}
extern "C" {
    pub fn __xfrm_policy_check(_arg: sk, _arg: ndir, _arg: skb, _arg: family) -> return;
}
// The packets here are plain ones and secpath was
// needed to indicate that hardware already handled
// them and there is no need to do nothing in addition.
//
// Consume secpath which was set by drivers.
//
extern "C" {
    pub fn __xfrm_policy_check2(_arg: sk, _arg: dir, _arg: skb, _arg: family, _arg: 0) -> return;
}
extern "C" {
    pub fn xfrm_policy_check(_arg: sk, _arg: dir, _arg: skb, _arg: AF_INET) -> return;
}
extern "C" {
    pub fn xfrm_policy_check(_arg: sk, _arg: dir, _arg: skb, _arg: AF_INET6) -> return;
}
extern "C" {
    pub fn __xfrm_policy_check2(_arg: sk, _arg: dir, _arg: skb, _arg: AF_INET, _arg: 1) -> return;
}
extern "C" {
    pub fn __xfrm_policy_check2(_arg: sk, _arg: dir, _arg: skb, _arg: AF_INET6, _arg: 1) -> return;
}
extern "C" {
    pub fn __xfrm_decode_session(_arg: net, _arg: skb, _arg: fl, _arg: family, _arg: 0) -> return;
}
extern "C" {
    pub fn __xfrm_decode_session(_arg: net, _arg: skb, _arg: fl, _arg: family, _arg: 1) -> return;
}
extern "C" {
    pub fn __xfrm_route_forward(skb: *mut sk_buff, family: c_ushort) -> c_int;
}
extern "C" {
    pub fn xfrm_route_forward(_arg: skb, _arg: AF_INET) -> return;
}
extern "C" {
    pub fn xfrm_route_forward(_arg: skb, _arg: AF_INET6) -> return;
}
extern "C" {
    pub fn __xfrm_sk_clone_policy(sk: *mut sock, osk: *const sock) -> c_int;
}
extern "C" {
    pub fn __xfrm_sk_clone_policy(_arg: sk, _arg: osk) -> return;
}
extern "C" {
    pub fn xfrm_policy_delete(pol: *mut xfrm_policy, dir: c_int) -> c_int;
}

extern "C" {
    pub fn __xfrm4_state_addr_check(_arg: x, _arg: daddr, _arg: saddr) -> return;
}
extern "C" {
    pub fn __xfrm6_state_addr_check(_arg: x, _arg: daddr, _arg: saddr) -> return;
}
extern "C" {
    pub fn atomic_read(_arg: &x->tunnel_users) -> return;
}

// IPSEC_PROTO_ANY only matches 3 IPsec protocols, 0 could match all.
//
// xfrm algorithm information
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_algo_aead_info {
    pub geniv: *mut c_char,
    pub icv_truncbits: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_algo_auth_info {
    pub icv_truncbits: u16,
    pub icv_fullbits: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_algo_encr_info {
    pub geniv: *mut c_char,
    pub blockbits: u16,
    pub defkeybits: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_algo_comp_info {
    pub threshold: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_algo_desc {
    pub name: *mut c_char,
    pub compat: *mut c_char,
    pub available:1: u8,
    pub pfkey_supported:1: u8,
    pub aead: xfrm_algo_aead_info,
    pub auth: xfrm_algo_auth_info,
    pub encr: xfrm_algo_encr_info,
    pub comp: xfrm_algo_comp_info,
    pub uinfo: },
    pub desc: sadb_alg,
}

// XFRM protocol handlers.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm4_protocol {
    pub skb): *mut *mut int (handler)(struct sk_buff,
    pub encap_type): c_int,
    pub err): *mut *mut *mut int (cb_handler)(struct sk_buff skb, int,
    pub info): *mut *mut *mut int (err_handler)(struct sk_buff skb, u32,
    pub next: *mut xfrm4_protocol __rcu,
    pub priority: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm6_protocol {
    pub skb): *mut *mut int (handler)(struct sk_buff,
    pub encap_type): c_int,
    pub err): *mut *mut *mut int (cb_handler)(struct sk_buff skb, int,
    pub info): u8 type, u8 code, int offset, __be32,
    pub next: *mut xfrm6_protocol __rcu,
    pub priority: c_int,
}

// XFRM tunnel handlers.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_tunnel {
    pub skb): *mut *mut int (handler)(struct sk_buff,
    pub err): *mut *mut *mut int (cb_handler)(struct sk_buff skb, int,
    pub info): *mut *mut *mut int (err_handler)(struct sk_buff skb, u32,
    pub next: *mut xfrm_tunnel __rcu,
    pub priority: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm6_tunnel {
    pub skb): *mut *mut int (handler)(struct sk_buff,
    pub err): *mut *mut *mut int (cb_handler)(struct sk_buff skb, int,
    pub info): u8 type, u8 code, int offset, __be32,
    pub next: *mut xfrm6_tunnel __rcu,
    pub priority: c_int,
}

extern "C" {
    pub fn xfrm_init();
}
extern "C" {
    pub fn xfrm4_init();
}
extern "C" {
    pub fn xfrm_state_init(net: *mut net) -> c_int;
}
extern "C" {
    pub fn xfrm_state_fini(net: *mut net);
}
extern "C" {
    pub fn xfrm4_state_init();
}
extern "C" {
    pub fn xfrm4_protocol_init();
}

extern "C" {
    pub fn xfrm6_init() -> c_int;
}
extern "C" {
    pub fn xfrm6_fini();
}
extern "C" {
    pub fn xfrm6_state_init() -> c_int;
}
extern "C" {
    pub fn xfrm6_state_fini();
}
extern "C" {
    pub fn xfrm6_protocol_init() -> c_int;
}
extern "C" {
    pub fn xfrm6_protocol_fini();
}

extern "C" {
    pub fn xfrm_proc_init(net: *mut net) -> c_int;
}
extern "C" {
    pub fn xfrm_proc_fini(net: *mut net);
}

extern "C" {
    pub fn xfrm_sysctl_init(net: *mut net) -> c_int;
}

extern "C" {
    pub fn xfrm_sysctl_fini(net: *mut net);
}

extern "C" {
    pub fn xfrm_state_walk_done(walk: *mut xfrm_state_walk, net: *mut net);
}
extern "C" {
    pub fn xfrm_state_free(x: *mut xfrm_state);
}
extern "C" {
    pub fn xfrm_state_check_expire(x: *mut xfrm_state) -> c_int;
}
extern "C" {
    pub fn xfrm_state_update_stats(net: *mut net);
}

extern "C" {
    pub fn xfrm_state_insert(x: *mut xfrm_state);
}
extern "C" {
    pub fn xfrm_state_add(x: *mut xfrm_state) -> c_int;
}
extern "C" {
    pub fn xfrm_state_update(x: *mut xfrm_state) -> c_int;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrmk_sadinfo {
    pub /: *mut *mut u32 sadhcnt; / current hash bkts,
    pub /: *mut *mut u32 sadhmcnt; / max allowed hash bkts,
    pub /: *mut *mut u32 sadcnt; / current running count,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrmk_spdinfo {
    pub incnt: u32,
    pub outcnt: u32,
    pub fwdcnt: u32,
    pub inscnt: u32,
    pub outscnt: u32,
    pub fwdscnt: u32,
    pub spdhcnt: u32,
    pub spdhmcnt: u32,
}

extern "C" {
    pub fn xfrm_state_delete(x: *mut xfrm_state) -> c_int;
}
extern "C" {
    pub fn xfrm_state_flush(net: *mut net, proto: u8, task_valid: bool) -> c_int;
}
extern "C" {
    pub fn xfrm_dev_state_flush(net: *mut net, dev: *mut net_device, task_valid: bool) -> c_int;
}
extern "C" {
    pub fn xfrm_sad_getinfo(net: *mut net, si: *mut xfrmk_sadinfo);
}
extern "C" {
    pub fn xfrm_spd_getinfo(net: *mut net, si: *mut xfrmk_spdinfo);
}
extern "C" {
    pub fn xfrm_replay_seqhi(x: *mut xfrm_state, net_seq: __be32) -> u32;
}
extern "C" {
    pub fn xfrm_init_replay(x: *mut xfrm_state, extack: *mut netlink_ext_ack) -> c_int;
}
extern "C" {
    pub fn xfrm_state_mtu(x: *mut xfrm_state, mtu: c_int) -> u32;
}
extern "C" {
    pub fn __xfrm_init_state(x: *mut xfrm_state, extack: *mut netlink_ext_ack) -> c_int;
}
extern "C" {
    pub fn xfrm_init_state(x: *mut xfrm_state, extack: *mut netlink_ext_ack) -> c_int;
}
extern "C" {
    pub fn xfrm_input(skb: *mut sk_buff, nexthdr: c_int, spi: __be32, encap_type: c_int) -> c_int;
}
extern "C" {
    pub fn xfrm_input_resume(skb: *mut sk_buff, nexthdr: c_int) -> c_int;
}
extern "C" {
    pub fn xfrm_output_resume(sk: *mut sock, skb: *mut sk_buff, err: c_int) -> c_int;
}
extern "C" {
    pub fn xfrm_output(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn xfrm4_tunnel_check_size(skb: *mut sk_buff) -> c_int;
}

extern "C" {
    pub fn xfrm6_tunnel_check_size(skb: *mut sk_buff) -> c_int;
}

extern "C" {
    pub fn pktgen_xfrm_outer_mode_output(x: *mut xfrm_state, skb: *mut sk_buff) -> c_int;
}

extern "C" {
    pub fn xfrm_local_error(skb: *mut sk_buff, mtu: c_int);
}
extern "C" {
    pub fn xfrm4_transport_finish(skb: *mut sk_buff, async: c_int) -> c_int;
}
extern "C" {
    pub fn xfrm4_rcv(skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn xfrm_input(_arg: skb, _arg: nexthdr, _arg: spi, _arg: 0) -> return;
}
extern "C" {
    pub fn xfrm4_output(net: *mut net, sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn xfrm4_protocol_register(handler: *mut xfrm4_protocol, protocol: c_uchar) -> c_int;
}
extern "C" {
    pub fn xfrm4_protocol_deregister(handler: *mut xfrm4_protocol, protocol: c_uchar) -> c_int;
}
extern "C" {
    pub fn xfrm4_tunnel_register(handler: *mut xfrm_tunnel, family: c_ushort) -> c_int;
}
extern "C" {
    pub fn xfrm4_tunnel_deregister(handler: *mut xfrm_tunnel, family: c_ushort) -> c_int;
}
extern "C" {
    pub fn xfrm4_local_error(skb: *mut sk_buff, mtu: u32);
}
extern "C" {
    pub fn xfrm6_transport_finish(skb: *mut sk_buff, async: c_int) -> c_int;
}
extern "C" {
    pub fn xfrm6_rcv_tnl(skb: *mut sk_buff, t: *mut ip6_tnl) -> c_int;
}
extern "C" {
    pub fn xfrm6_rcv(skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn xfrm6_local_error(skb: *mut sk_buff, mtu: u32);
}
extern "C" {
    pub fn xfrm6_protocol_register(handler: *mut xfrm6_protocol, protocol: c_uchar) -> c_int;
}
extern "C" {
    pub fn xfrm6_protocol_deregister(handler: *mut xfrm6_protocol, protocol: c_uchar) -> c_int;
}
extern "C" {
    pub fn xfrm6_tunnel_register(handler: *mut xfrm6_tunnel, family: c_ushort) -> c_int;
}
extern "C" {
    pub fn xfrm6_tunnel_deregister(handler: *mut xfrm6_tunnel, family: c_ushort) -> c_int;
}
extern "C" {
    pub fn xfrm6_tunnel_alloc_spi(net: *mut net, saddr: *mut xfrm_address_t) -> __be32;
}
extern "C" {
    pub fn xfrm6_tunnel_spi_lookup(net: *mut net, saddr: *const xfrm_address_t) -> __be32;
}
extern "C" {
    pub fn xfrm6_output(net: *mut net, sk: *mut sock, skb: *mut sk_buff) -> c_int;
}

extern "C" {
    pub fn xfrm6_local_rxpmtu(skb: *mut sk_buff, mtu: u32);
}
extern "C" {
    pub fn xfrm4_udp_encap_rcv(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn xfrm6_udp_encap_rcv(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}

extern "C" {
    pub fn xfrm_policy_walk_init(walk: *mut xfrm_policy_walk, type: u8);
}
extern "C" {
    pub fn xfrm_policy_walk_done(walk: *mut xfrm_policy_walk, net: *mut net);
}
extern "C" {
    pub fn xfrm_policy_insert(dir: c_int, policy: *mut xfrm_policy, excl: c_int) -> c_int;
}
extern "C" {
    pub fn xfrm_policy_flush(net: *mut net, type: u8, task_valid: bool) -> c_int;
}
extern "C" {
    pub fn xfrm_policy_hash_rebuild(net: *mut net);
}
extern "C" {
    pub fn xfrm_get_acqseq() -> u32;
}
extern "C" {
    pub fn verify_spi_info(proto: u8, min: u32, max: u32, extack: *mut netlink_ext_ack) -> c_int;
}
extern "C" {
    pub fn xfrm_sk_policy_insert(sk: *mut sock, dir: c_int, pol: *mut xfrm_policy) -> c_int;
}

extern "C" {
    pub fn km_new_mapping(x: *mut xfrm_state, ipaddr: *mut xfrm_address_t, sport: __be16) -> c_int;
}
extern "C" {
    pub fn km_policy_expired(pol: *mut xfrm_policy, dir: c_int, hard: c_int, portid: u32);
}
extern "C" {
    pub fn xfrm_input_init();
}
extern "C" {
    pub fn xfrm_parse_spi(skb: *mut sk_buff, nexthdr: u8, spi: *mut __be32, seq: *mut __be32) -> c_int;
}
extern "C" {
    pub fn xfrm_probe_algs();
}
extern "C" {
    pub fn xfrm_count_pfkey_auth_supported() -> c_int;
}
extern "C" {
    pub fn xfrm_count_pfkey_enc_supported() -> c_int;
}
extern "C" {
    pub fn xfrm6_addr_equal(_arg: a, _arg: b) -> return;
}

extern "C" {
    pub fn xfrm_replay_advance(x: *mut xfrm_state, net_seq: __be32);
}
extern "C" {
    pub fn xfrm_replay_check(x: *mut xfrm_state, skb: *mut sk_buff, net_seq: __be32) -> c_int;
}
extern "C" {
    pub fn xfrm_replay_notify(x: *mut xfrm_state, event: c_int);
}
extern "C" {
    pub fn xfrm_replay_overflow(x: *mut xfrm_state, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn xfrm_replay_recheck(x: *mut xfrm_state, skb: *mut sk_buff, net_seq: __be32) -> c_int;
}

extern "C" {
    pub fn sizeof(8: *mut *mut alg) + ((alg->alg_key_len + 7) /) -> return;
}
extern "C" {
    pub fn sizeof(8: *mut *mut alg) + ((alg->alg_key_len + 7) /) -> return;
}
extern "C" {
    pub fn sizeof(8: *mut *mut alg) + ((alg->alg_key_len + 7) /) -> return;
}
extern "C" {
    pub fn sizeof(sizeof(__u32: *mut *mut *mut replay_esn) + replay_esn->bmp_len) -> return;
}

// Counters synced later in xfrm_replay_sync()
// called under lock so no race conditions or mallocs allowed
extern "C" {
    pub fn kmemdup(_arg: orig, _arg: aead_len(orig), _arg: GFP_KERNEL) -> return;
}
extern "C" {
    pub fn kmemdup(_arg: orig, _arg: xfrm_alg_len(orig), _arg: GFP_KERNEL) -> return;
}
extern "C" {
    pub fn kmemdup(_arg: orig, _arg: xfrm_alg_auth_len(orig), _arg: GFP_KERNEL) -> return;
}

extern "C" {
    pub fn xfrm_dev_init() -> void __init;
}

extern "C" {
    pub fn xfrm_dev_resume(skb: *mut sk_buff);
}
extern "C" {
    pub fn xfrm_dev_backlog(sd: *mut softnet_data);
}
extern "C" {
    pub fn xfrm_dev_offload_ok(skb: *mut sk_buff, x: *mut xfrm_state) -> bool;
}
extern "C" {
    pub fn xfrm_dev_state_delete(x: *mut xfrm_state);
}
extern "C" {
    pub fn xfrm_dev_state_free(x: *mut xfrm_state);
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_translator {
// Allocate frag_list and put compat translation there
    pub src): *const *const *const int (alloc_compat)(struct sk_buff skb, struct nlmsghdr,
// Allocate nlmsg with 64-bit translaton of received 32-bit message
    pub extack): *mut netlink_ext_ack,
// Translate 32-bit user_policy from sockptr
    pub optlen): *mut *mut *mut *mut int (xlate_user_policy_sockptr)(u8 pdata32, int,
    pub owner: *mut module,
}

extern "C" {
    pub fn xfrm_register_translator(xtr: *mut xfrm_translator) -> c_int;
}
extern "C" {
    pub fn xfrm_unregister_translator(xtr: *mut xfrm_translator) -> c_int;
}
extern "C" {
    pub fn xfrm_put_translator(xtr: *mut xfrm_translator);
}

extern "C" {
    pub fn inet6_test_bit(_arg: DONTFRAG, _arg: sk) -> return;
}

extern "C" {
    pub fn register_xfrm_interface_bpf() -> c_int;
}

extern "C" {
    pub fn register_xfrm_state_bpf() -> c_int;
}

extern "C" {
    pub fn xfrm_nat_keepalive_init(family: c_ushort) -> c_int;
}
extern "C" {
    pub fn xfrm_nat_keepalive_fini(family: c_ushort);
}
extern "C" {
    pub fn xfrm_nat_keepalive_net_init(net: *mut net) -> c_int;
}
extern "C" {
    pub fn xfrm_nat_keepalive_net_fini(net: *mut net) -> c_int;
}
extern "C" {
    pub fn xfrm_nat_keepalive_state_updated(x: *mut xfrm_state);
}
