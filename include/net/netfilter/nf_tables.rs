//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/netfilter/nf_tables.h
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

pub const NFT_JUMP_STACK_SIZE: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nft_pktinfo {
    pub skb: *mut sk_buff,
    pub state: *const nf_hook_state,
    pub flags: u8,
    pub tprot: u8,
    pub ethertype: __be16,
    pub fragoff: u16,
    pub nhoff: u16,
    pub thoff: u16,
    pub inneroff: u16,
}

//
// struct nft_verdict - nf_tables verdict
//
// @code: nf_tables/netfilter verdict code
// @chain: destination chain for NFT_JUMP/NFT_GOTO
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nft_verdict {
    pub code: u32,
    pub chain: *mut nft_chain,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nft_data {
    pub data: [u32; 4],
    pub verdict: nft_verdict,
}

pub const NFT_REG32_NUM: c_int = 20;
//
// struct nft_regs - nf_tables register set
//
// @data: data registers
// @verdict: verdict register
//
// The first four data registers alias to the verdict register.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nft_regs {
    pub data: [u32; NFT_REG32_NUM],
    pub verdict: nft_verdict,
}

// Store/load an u8, u16 or u64 integer to/from the u32 data register.
//
// Note, when using concatenations, register allocation happens at 32-bit
// level. So for store instruction, pad the rest part with zero to avoid
// garbage values.
//
// dreg = 0;
// (u8 *)dreg = val;
// dreg = 0;
// (u16 *)dreg = val;
extern "C" {
    pub fn get_unaligned()sreg: *mut (u64) -> return;
}
//
// struct nft_ctx - nf_tables rule/set context
//
// @net: net namespace
// @table: the table the chain is contained in
// @chain: the chain the rule is contained in
// @nla: netlink attributes
// @portid: netlink portID of the original message
// @seq: netlink sequence number
// @flags: modifiers to new request
// @family: protocol family
// @level: depth of the chains
// @report: notify via unicast netlink message
// @reg_inited: bitmap of initialised registers
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nft_ctx {
    pub net: *mut net,
    pub table: *mut nft_table,
    pub chain: *mut nft_chain,
    pub nla: *const *const nlattr,
    pub portid: u32,
    pub seq: u32,
    pub flags: u16,
    pub family: u8,
    pub level: u8,
    pub report: bool,
    pub NFT_REG32_NUM): DECLARE_BITMAP(reg_inited,,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nft_data_desc_flags {
    NFT_DATA_DESC_SETELEM	= (1 << 0),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nft_data_desc {
    pub type: nft_data_types,
    pub size: c_uint,
    pub len: c_uint,
    pub flags: c_uint,
}

extern "C" {
    pub fn nft_data_hold(data: *const nft_data, type: nft_data_types);
}
extern "C" {
    pub fn nft_data_release(data: *const nft_data, type: nft_data_types);
}
extern "C" {
    pub fn nft_parse_u32_check(attr: *const nlattr, max: c_int, dest: *mut u32) -> c_int;
}
extern "C" {
    pub fn nft_dump_register(skb: *mut sk_buff, attr: c_uint, reg: c_uint) -> c_int;
}
//
// struct nft_userdata - user defined data associated with an object
//
// @len: length of the data
// @data: content
//
// The presence of user data is indicated in an object specific fashion,
// so a length of zero can't occur and the value "len" indicates data
// of length len + 1.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nft_userdata {
    pub len: u8,
    pub data: [c_uchar; ],
}

// placeholder structure for opaque set element backend representation.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nft_elem_priv {
//
// struct nft_set_elem - generic representation of set elements
//
// @key: element key
// @key_end: closing element key
// @data: element data
// @priv: element private data and extensions
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nft_set_elem {
    pub sizeof(u32)]: u32 buf[NFT_DATA_VALUE_MAXLEN /,
    pub val: nft_data,
    pub key: },
    pub sizeof(u32)]: u32 buf[NFT_DATA_VALUE_MAXLEN /,
    pub val: nft_data,
    pub key_end: },
    pub sizeof(u32)]: u32 buf[NFT_DATA_VALUE_MAXLEN /,
    pub val: nft_data,
    pub data: },
    pub priv: *mut nft_elem_priv,
}

//
// enum nft_iter_type - nftables set iterator type
//
// @NFT_ITER_UNSPEC: unspecified, to catch errors
// @NFT_ITER_READ: read-only iteration over set elements
// @NFT_ITER_UPDATE: iteration under mutex to update set element state
// @NFT_ITER_UPDATE_CLONE: clone set before iteration under mutex to update element
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nft_iter_type {
    NFT_ITER_UNSPEC,
    NFT_ITER_READ,
    NFT_ITER_UPDATE,
    NFT_ITER_UPDATE_CLONE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nft_set_iter {
    pub genmask: u8,
    pub type:8: nft_iter_type,
    pub count: c_uint,
    pub skip: c_uint,
    pub err: c_int,
    pub elem_priv): *mut nft_elem_priv,
}

//
// struct nft_set_desc - description of set elements
//
// @ktype: key type
// @klen: key length
// @dtype: data type
// @dlen: data length
// @objtype: object type
// @size: number of set elements
// @policy: set policy
// @gc_int: garbage collector interval
// @timeout: element timeout
// @field_len: length of each field in concatenation, bytes
// @field_count: number of concatenated fields in element
// @expr: set must support for expressions
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nft_set_desc {
    pub ktype: u32,
    pub klen: c_uint,
    pub dtype: u32,
    pub dlen: c_uint,
    pub objtype: u32,
    pub size: c_uint,
    pub policy: u32,
    pub gc_int: u32,
    pub timeout: u64,
    pub field_len: [u8; NFT_REG32_COUNT],
    pub field_count: u8,
    pub expr: bool,
}

//
// enum nft_set_class - performance class
//
// @NFT_SET_CLASS_O_1: constant, O(1)
// @NFT_SET_CLASS_O_LOG_N: logarithmic, O(log N)
// @NFT_SET_CLASS_O_N: linear, O(N)
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nft_set_class {
    NFT_SET_CLASS_O_1,
    NFT_SET_CLASS_O_LOG_N,
    NFT_SET_CLASS_O_N,
}

//
// struct nft_set_estimate - estimation of memory and performance
// characteristics
//
// @size: required memory
// @lookup: lookup performance class
// @space: memory class
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nft_set_estimate {
    pub size: u64,
    pub lookup: nft_set_class,
    pub space: nft_set_class,
}

pub const NFT_EXPR_MAXATTR: c_int = 16;

//
// struct nft_expr - nf_tables expression
//
// @ops: expression ops
// @data: expression private data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nft_expr {
    pub ops: *const nft_expr_ops,
}

extern "C" {
    pub fn nft_expr_clone(dst: *mut nft_expr, src: *mut nft_expr, gfp: gfp_t) -> c_int;
}
extern "C" {
    pub fn nft_expr_destroy(ctx: *const nft_ctx, expr: *mut nft_expr);
}
//
// struct nft_set_ops - nf_tables set operations
//
// @lookup: look up an element within the set
// @update: update an element if exists, add it if doesn't exist
// @delete: delete an element
// @insert: insert new element into set
// @activate: activate new element in the next generation
// @deactivate: lookup for element and deactivate it in the next generation
// @flush: deactivate element in the next generation
// @remove: remove element from set
// @walk: iterate over all set elements
// @get: get set elements
// @ksize: kernel set size
// @usize: userspace set size
// @adjust_maxsize: delta to adjust maximum set size
// @commit: commit set elements
// @abort: abort set elements
// @privsize: function to return size of set private data
// @estimate: estimate the required memory size and the lookup complexity class
// @init: initialize private data of new set instance
// @destroy: destroy private data of set instance
// @gc_init: initialize garbage collection
// @abort_skip_removal: skip removal of elements from abort path
// @elemsize: element private size
//
// Operations lookup, update and delete have simpler interfaces, are faster
// and currently only used in the packet path. All the rest are slower,
// control plane functions.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nft_set_ops {
    pub key): *const u32,
    pub regs): *mut nft_regs,
    pub key): *const u32,
    pub priv): *mut nft_elem_priv,
    pub elem_priv): *mut nft_elem_priv,
    pub elem): *const nft_set_elem,
    pub priv): *mut nft_elem_priv,
    pub elem_priv): *mut nft_elem_priv,
    pub iter): *mut nft_set_iter,
    pub flags): c_uint,
    pub size): *mut *mut u32 (ksize)(u32,
    pub size): *mut *mut u32 (usize)(u32,
    pub set): *const *const u32 (adjust_maxsize)(struct nft_set,
    pub set): *mut *mut void (commit)(struct nft_set,
    pub set): *const *const void (abort)(struct nft_set,
    pub desc): *const nft_set_desc,
    pub est): *mut nft_set_estimate,
    pub nla[]): *const *const nlattr,
    pub set): *const nft_set,
    pub set): *const *const void (gc_init)(struct nft_set,
    pub abort_skip_removal: bool,
    pub elemsize: c_uint,
}

//
// struct nft_set_type - nf_tables set type
//
// @ops: set ops for this type
// @features: features supported by the implementation
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nft_set_type {
    pub ops: nft_set_ops,
    pub features: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nft_set_elem_expr {
    pub size: u8,
// C attribute field omitted
}

pub const NFT_SET_EXPR_MAX: c_int = 2;
//
// struct nft_set - nf_tables set instance
//
// @list: table set list node
// @bindings: list of set bindings
// @refs: internal refcounting for async set destruction
// @table: table this set belongs to
// @net: netnamespace this set belongs to
// @name: name of the set
// @handle: unique handle of the set
// @ktype: key type (numeric type defined by userspace, not used in the kernel)
// @dtype: data type (verdict or numeric type defined by userspace)
// @objtype: object type (see NFT_OBJECT_* definitions)
// @size: maximum set size
// @field_len: length of each field in concatenation, bytes
// @field_count: number of concatenated fields in element
// @in_update_walk: true during ->walk() in transaction phase
// @use: number of rules references to this set
// @nelems: number of elements
// @ndeact: number of deactivated elements queued for removal
// @timeout: default timeout value in jiffies
// @gc_int: garbage collection interval in msecs
// @policy: set parameterization (see enum nft_set_policies)
// @udlen: user data length
// @udata: user data
// @pending_update: list of pending update set element
// @ops: set ops
// @flags: set flags
// @dead: set will be freed, never cleared
// @genmask: generation mask
// @klen: key length
// @dlen: data length
// @num_exprs: numbers of exprs
// @exprs: stateful expression
// @catchall_list: list of catch-all set element
// @data: private set data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nft_set {
    pub list: list_head,
    pub bindings: list_head,
    pub refs: refcount_t,
    pub table: *mut nft_table,
    pub net: possible_net_t,
    pub name: *mut c_char,
    pub handle: u64,
    pub ktype: u32,
    pub dtype: u32,
    pub objtype: u32,
    pub size: u32,
    pub field_len: [u8; NFT_REG32_COUNT],
    pub field_count: u8,
    pub in_update_walk: bool,
    pub use: u32,
    pub nelems: core::sync::atomic::AtomicI32,
    pub ndeact: u32,
    pub timeout: u64,
    pub gc_int: u32,
    pub policy: u16,
    pub udlen: u16,
    pub udata: *mut c_uchar,
    pub pending_update: list_head,
// runtime data below here
    pub ____cacheline_aligned: *const *const nft_set_ops ops,
    pub klen: u8,
    pub dlen: u8,
    pub num_exprs: u8,
    pub exprs: [*mut nft_expr; NFT_SET_EXPR_MAX],
    pub catchall_list: list_head,
}

//
// struct nft_set_binding - nf_tables set binding
//
// @list: set bindings list node
// @chain: chain containing the rule bound to the set
// @flags: set action flags
//
// A set binding contains all information necessary for validation
// of new elements added to a bound set.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nft_set_binding {
    pub list: list_head,
    pub chain: *const nft_chain,
    pub flags: u32,
}

extern "C" {
    pub fn nf_tables_activate_set(ctx: *const nft_ctx, set: *mut nft_set);
}
extern "C" {
    pub fn nf_tables_destroy_set(ctx: *const nft_ctx, set: *mut nft_set);
}
//
// enum nft_set_extensions - set extension type IDs
//
// @NFT_SET_EXT_KEY: element key
// @NFT_SET_EXT_KEY_END: upper bound element key, for ranges
// @NFT_SET_EXT_DATA: mapping data
// @NFT_SET_EXT_FLAGS: element flags
// @NFT_SET_EXT_TIMEOUT: element timeout
// @NFT_SET_EXT_USERDATA: user data associated with the element
// @NFT_SET_EXT_EXPRESSIONS: expressions associated with the element
// @NFT_SET_EXT_OBJREF: stateful object reference associated with element
// @NFT_SET_EXT_NUM: number of extension types
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nft_set_extensions {
    NFT_SET_EXT_KEY,
    NFT_SET_EXT_KEY_END,
    NFT_SET_EXT_DATA,
    NFT_SET_EXT_FLAGS,
    NFT_SET_EXT_TIMEOUT,
    NFT_SET_EXT_USERDATA,
    NFT_SET_EXT_EXPRESSIONS,
    NFT_SET_EXT_OBJREF,
    NFT_SET_EXT_NUM
}

//
// struct nft_set_ext_type - set extension type
//
// @len: fixed part length of the extension
// @align: alignment requirements of the extension
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nft_set_ext_type {
    pub len: u8,
    pub align: u8,
}

//
// struct nft_set_ext_tmpl - set extension template
//
// @len: length of extension area
// @offset: offsets of individual extension types
// @ext_len: length of the expected extension(used to sanity check)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nft_set_ext_tmpl {
    pub len: u16,
    pub offset: [u8; NFT_SET_EXT_NUM],
    pub ext_len: [u8; NFT_SET_EXT_NUM],
}

//
// struct nft_set_ext - set extensions
//
// @genmask: generation mask, but also flags (see NFT_SET_ELEM_DEAD_BIT)
// @offset: offsets of individual extension types
// @data: beginning of extension data
//
// This structure must be aligned to word size, otherwise atomic bitops
// on genmask field can cause alignment failure on some archs.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nft_set_ext {
    pub genmask: u8,
    pub offset: [u8; NFT_SET_EXT_NUM],
    pub data: [c_char; ],
    pub 8): } __aligned(BITS_PER_LONG /,
    pub sizeof(*tmpl)): *mut memset(tmpl, 0,,
    pub nft_set_ext): tmpl->len = sizeof(struct,
    pub nft_set_ext_types[id].align): tmpl->len = ALIGN(tmpl->len,,
    pub -EINVAL: return,
    pub tmpl->len: tmpl->offset[id] =,
    pub len: tmpl->ext_len[id] = nft_set_ext_types[id].len +,
    pub tmpl->ext_len[id]: tmpl->len +=,
    pub 0: return,
    pub 0): return nft_set_ext_add_length(tmpl, id,,
    pub sizeof(ext->offset)): memcpy(ext->offset, tmpl->offset,,
    pub !!ext->offset[id]: return,
    pub id): return ext && __nft_set_ext_exists(ext,,
    pub ext->offset[id]: *mut *mut return (void )ext +,
    pub NFT_SET_EXT_KEY): return nft_set_ext(ext,,
    pub NFT_SET_EXT_KEY_END): return nft_set_ext(ext,,
    pub NFT_SET_EXT_DATA): return nft_set_ext(ext,,
    pub NFT_SET_EXT_FLAGS): return nft_set_ext(ext,,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nft_timeout {
    pub timeout: u64,
    pub expiration: u64,
}

extern "C" {
    pub fn nft_set_ext(_arg: ext, _arg: NFT_SET_EXT_TIMEOUT) -> return;
}
extern "C" {
    pub fn nft_set_ext(_arg: ext, _arg: NFT_SET_EXT_USERDATA) -> return;
}
extern "C" {
    pub fn nft_set_ext(_arg: ext, _arg: NFT_SET_EXT_EXPRESSIONS) -> return;
}
extern "C" {
    pub fn time_after_eq64(_arg: tstamp, _arg: READ_ONCE(nft_set_ext_timeout(ext)->expiration)) -> return;
}
extern "C" {
    pub fn __nft_set_elem_expired(_arg: ext, _arg: get_jiffies_64()) -> return;
}
extern "C" {
    pub fn nft_set_ext(_arg: ext, _arg: NFT_SET_EXT_OBJREF) -> return;
}
//
// struct nft_expr_type - nf_tables expression type
//
// @select_ops: function to select nft_expr_ops
// @release_ops: release nft_expr_ops
// @ops: default ops, used when no select_ops functions is present
// @inner_ops: inner ops, used for inner packet operation
// @list: used internally
// @name: Identifier
// @owner: module reference
// @policy: netlink attribute policy
// @maxattr: highest netlink attribute number
// @family: address family for AF-specific types
// @flags: expression type flags
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nft_expr_type {
    pub tb[]): *const *const nlattr,
    pub ops): *const *const void (release_ops)(struct nft_expr_ops,
    pub ops: *const nft_expr_ops,
    pub inner_ops: *const nft_expr_ops,
    pub list: list_head,
    pub name: *const c_char,
    pub owner: *mut module,
    pub policy: *const nla_policy,
    pub maxattr: c_uint,
    pub family: u8,
    pub flags: u8,
}

pub const NFT_EXPR_STATEFUL: c_uint = 0x1;
pub const NFT_EXPR_GC: c_uint = 0x2;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nft_trans_phase {
    NFT_TRANS_PREPARE,
    NFT_TRANS_PREPARE_ERROR,
    NFT_TRANS_ABORT,
    NFT_TRANS_COMMIT,
    NFT_TRANS_RELEASE
}

//
// struct nft_expr_ops - nf_tables expression operations
//
// @eval: Expression evaluation function
// @clone: Expression clone function
// @size: full expression size, including private data size
// @init: initialization function
// @activate: activate expression in the next generation
// @deactivate: deactivate expression in next generation
// @destroy: destruction function, called after synchronize_rcu
// @destroy_clone: destruction clone function
// @dump: function to dump parameters
// @validate: validate expression, called during loop detection
// @gc: garbage collection expression
// @offload: hardware offload expression
// @offload_action: function to report true/false to allocate one slot or not in the flow
// offload array
// @offload_stats: function to synchronize hardware stats via updating the counter expression
// @type: expression type
// @data: extra data to attach to this expression operation
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nft_expr_ops {
    pub pkt): *const nft_pktinfo,
    pub gfp): *const *const nft_expr src, gfp_t,
    pub size: c_uint,
    pub tb[]): *const *const nlattr,
    pub expr): *const nft_expr,
    pub phase): nft_trans_phase,
    pub expr): *const nft_expr,
    pub expr): *const nft_expr,
    pub reset): bool,
    pub expr): *const nft_expr,
    pub expr): *const nft_expr,
    pub expr): *const nft_expr,
    pub expr): *const *const bool (offload_action)(struct nft_expr,
    pub stats): *const flow_stats,
    pub type: *const nft_expr_type,
    pub data: *mut c_void,
}

//
// struct nft_rule - nf_tables rule
//
// @list: used internally
// @handle: rule handle
// @genmask: generation mask
// @dlen: length of expression data
// @udata: user data is appended to the rule
// @data: expression data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nft_rule {
    pub list: list_head,
// C attribute field omitted
}

extern "C" {
    pub fn nft_rule_expr_activate(ctx: *const nft_ctx, rule: *mut nft_rule);
}
extern "C" {
    pub fn nf_tables_rule_destroy(ctx: *const nft_ctx, rule: *mut nft_rule);
}
//
// The last pointer isn't really necessary, but the compiler isn't able to
// determine that the result of nft_expr_last() is always the same since it
// can't assume that the dlen value wasn't changed within calls in the loop.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nft_rule_dp {
    pub /: *mut *mut handle:42; / for tracing,
// C attribute field omitted
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nft_rule_dp_last {
    pub /: *mut *mut nft_rule_dp end; / end of nft_rule_blob marker,
    pub /: *mut *mut rcu_head h; / call_rcu head,
    pub /: *mut *mut *mut nft_rule_blob blob; / ptr to free via call_rcu,
    pub /: *const *const *const nft_chain chain; / for nftables tracing,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nft_rule_blob {
    pub size: c_ulong,
// C attribute field omitted
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nft_chain_types {
    NFT_CHAIN_T_DEFAULT = 0,
    NFT_CHAIN_T_ROUTE,
    NFT_CHAIN_T_NAT,
    NFT_CHAIN_T_MAX
}

//
// struct nft_chain_validate_state - validation state
//
// If a chain is encountered again during table validation it is
// possible to avoid revalidation provided the calling context is
// compatible.  This structure stores relevant calling context of
// previous validations.
//
// @hook_mask: the hook numbers and locations the chain is linked to
// @depth: the deepest call chain level the chain is linked to
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nft_chain_validate_state {
    pub hook_mask: [u8; NFT_CHAIN_T_MAX],
    pub depth: u8,
}

//
// struct nft_chain - nf_tables chain
//
// @blob_gen_0: rule blob pointer to the current generation
// @blob_gen_1: rule blob pointer to the future generation
// @rules: list of rules in the chain
// @list: used internally
// @rhlhead: used internally
// @table: table that this chain belongs to
// @handle: chain handle
// @use: number of jump references to this chain
// @flags: bitmask of enum NFTA_CHAIN_FLAGS
// @bound: bind or not
// @genmask: generation mask
// @name: name of the chain
// @udlen: user data length
// @udata: user data in the chain
// @blob_next: rule blob pointer to the next in the chain
// @vstate: validation state
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nft_chain {
    pub blob_gen_0: *mut nft_rule_blob __rcu,
    pub blob_gen_1: *mut nft_rule_blob __rcu,
    pub rules: list_head,
    pub list: list_head,
    pub rhlhead: rhlist_head,
    pub table: *mut nft_table,
    pub handle: u64,
    pub use: u32,
    pub name: *mut c_char,
    pub udlen: u16,
    pub udata: *mut u8,
// Only used during control plane commit phase:
    pub blob_next: *mut nft_rule_blob,
    pub vstate: nft_chain_validate_state,
}

extern "C" {
    pub fn nft_chain_validate(ctx: *const nft_ctx, chain: *mut nft_chain) -> c_int;
}
extern "C" {
    pub fn nft_set_catchall_validate(ctx: *const nft_ctx, set: *mut nft_set) -> c_int;
}
extern "C" {
    pub fn nf_tables_bind_chain(ctx: *const nft_ctx, chain: *mut nft_chain) -> c_int;
}
extern "C" {
    pub fn nf_tables_unbind_chain(ctx: *const nft_ctx, chain: *mut nft_chain);
}
//
// struct nft_chain_type - nf_tables chain type info
//
// @name: name of the type
// @type: numeric identifier
// @family: address family
// @owner: module owner
// @hook_mask: mask of valid hooks
// @hooks: array of hook functions
// @ops_register: base chain register function
// @ops_unregister: base chain unregister function
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nft_chain_type {
    pub name: *const c_char,
    pub type: nft_chain_types,
    pub family: c_int,
    pub owner: *mut module,
    pub hook_mask: c_uint,
    pub hooks: [*mut nf_hookfn; NFT_MAX_HOOKS],
    pub ops): *const *const *const int (ops_register)(struct net net, struct nf_hook_ops,
    pub ops): *const *const *const void (ops_unregister)(struct net net, struct nf_hook_ops,
}

extern "C" {
    pub fn nft_chain_add(table: *mut nft_table, chain: *mut nft_chain) -> c_int;
}
extern "C" {
    pub fn nft_chain_del(chain: *mut nft_chain);
}
extern "C" {
    pub fn nf_tables_chain_destroy(chain: *mut nft_chain);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nft_stats {
    pub bytes: u64,
    pub pkts: u64,
    pub syncp: u64_stats_sync,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nft_hook {
    pub list: list_head,
    pub ops_list: list_head,
    pub rcu: rcu_head,
    pub ifname: [c_char; IFNAMSIZ],
    pub ifnamelen: u8,
    pub flags: u8,
}

//
// struct nft_base_chain - nf_tables base chain
//
// @ops: netfilter hook ops
// @hook_list: list of netfilter hooks (for NFPROTO_NETDEV family)
// @type: chain type
// @policy: default policy
// @flags: indicate the base chain disabled or not
// @stats: per-cpu chain stats
// @chain: the chain
// @flow_block: flow block (for hardware offload)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nft_base_chain {
    pub ops: nf_hook_ops,
    pub hook_list: list_head,
    pub type: *const nft_chain_type,
    pub policy: u8,
    pub flags: u8,
    pub stats: *mut nft_stats __percpu,
    pub chain: nft_chain,
    pub flow_block: flow_block,
}

extern "C" {
    pub fn container_of(_arg: chain, nft_base_chain: struct, _arg: chain) -> return;
}
extern "C" {
    pub fn nft_do_chain(pkt: *mut nft_pktinfo, priv: *mut c_void) -> c_uint;
}
// For error and abort path: restore use counter to previous state.

//
// struct nft_table - nf_tables table
//
// @list: used internally
// @chains_ht: chains in the table
// @chains: same, for stable walks
// @sets: sets in the table
// @objects: stateful objects in the table
// @flowtables: flow tables in the table
// @objname_ht: hashtable for objects lookup by name
// @hgenerator: handle generator state
// @handle: table handle
// @use: number of chain references to this table
// @family:address family
// @flags: table flag (see enum nft_table_flags)
// @genmask: generation mask
// @nlpid: netlink port ID
// @name: name of the table
// @udlen: length of the user data
// @udata: user data
// @validate_state: internal, set when transaction adds jumps
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nft_table {
    pub list: list_head,
    pub chains_ht: rhltable,
    pub chains: list_head,
    pub sets: list_head,
    pub objects: list_head,
    pub flowtables: list_head,
    pub objname_ht: rhltable,
    pub hgenerator: u64,
    pub handle: u64,
    pub use: u32,
    pub nlpid: u32,
    pub name: *mut c_char,
    pub udlen: u16,
    pub udata: *mut u8,
    pub validate_state: u8,
}

extern "C" {
    pub fn nft_register_chain_type(: *const nft_chain_type);
}
extern "C" {
    pub fn nft_unregister_chain_type(: *const nft_chain_type);
}
extern "C" {
    pub fn nft_register_expr(: *mut nft_expr_type) -> c_int;
}
extern "C" {
    pub fn nft_unregister_expr(: *mut nft_expr_type);
}
//
// struct nft_object_hash_key - key to lookup nft_object
//
// @name: name of the stateful object to look up
// @table: table the object belongs to
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nft_object_hash_key {
    pub name: *const c_char,
    pub table: *const nft_table,
}

//
// struct nft_object - nf_tables stateful object
//
// @list: table stateful object list node
// @rhlhead: nft_objname_ht node
// @key: keys that identify this object
// @genmask: generation mask
// @use: number of references to this stateful object
// @handle: unique object handle
// @udlen: length of user data
// @udata: user data
// @ops: object operations
// @data: object data, layout depends on type
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nft_object {
    pub list: list_head,
    pub rhlhead: rhlist_head,
    pub key: nft_object_hash_key,
    pub genmask:2: u32,
    pub use: u32,
    pub handle: u64,
    pub udlen: u16,
    pub udata: *mut u8,
// runtime data below here
    pub ____cacheline_aligned: *const *const nft_object_ops ops,
}

//
// struct nft_object_type - stateful object type
//
// @select_ops: function to select nft_object_ops
// @ops: default ops, used when no select_ops functions is present
// @list: list node in list of object types
// @type: stateful object numeric type
// @owner: module owner
// @maxattr: maximum netlink attribute
// @family: address family for AF-specific object types
// @policy: netlink attribute policy
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nft_object_type {
    pub tb[]): *const *const nlattr,
    pub ops: *const nft_object_ops,
    pub list: list_head,
    pub type: u32,
    pub maxattr: c_uint,
    pub family: u8,
    pub owner: *mut module,
    pub policy: *const nla_policy,
}

//
// struct nft_object_ops - stateful object operations
//
// @eval: stateful object evaluation function
// @size: stateful object size
// @init: initialize object from netlink attributes
// @destroy: release existing stateful object
// @dump: netlink dump stateful object
// @update: update stateful object
// @type: pointer to object type
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nft_object_ops {
    pub pkt): *const nft_pktinfo,
    pub size: c_uint,
    pub obj): *mut nft_object,
    pub obj): *mut nft_object,
    pub reset): bool,
    pub newobj): *mut nft_object,
    pub type: *const nft_object_type,
}

extern "C" {
    pub fn nft_register_obj(obj_type: *mut nft_object_type) -> c_int;
}
extern "C" {
    pub fn nft_unregister_obj(obj_type: *mut nft_object_type);
}
pub const NFT_NETDEVICE_MAX: c_int = 256;
//
// struct nft_flowtable - nf_tables flow table
//
// @list: flow table list node in table list
// @table: the table the flow table is contained in
// @name: name of this flow table
// @hooknum: hook number
// @ops_len: number of hooks in array
// @genmask: generation mask
// @use: number of references to this flow table
// @handle: unique object handle
// @hook_list: hook list for hooks per net_device in flowtables
// @data: rhashtable and garbage collector
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nft_flowtable {
    pub list: list_head,
    pub table: *mut nft_table,
    pub name: *mut c_char,
    pub hooknum: c_int,
    pub ops_len: c_int,
    pub genmask:2: u32,
    pub use: u32,
    pub handle: u64,
// runtime data below here
    pub ____cacheline_aligned: list_head hook_list,
    pub data: nf_flowtable,
}

extern "C" {
    pub fn nft_register_flowtable_type(type: *mut nf_flowtable_type);
}
extern "C" {
    pub fn nft_unregister_flowtable_type(type: *mut nf_flowtable_type);
}
//
// struct nft_traceinfo - nft tracing information and state
//
// @trace: other struct members are initialised
// @nf_trace: copy of skb->nf_trace before rule evaluation
// @type: event type (enum nft_trace_types)
// @skbid: hash of skb to be used as trace id
// @packet_dumped: packet headers sent in a previous traceinfo message
// @basechain: base chain currently processed
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nft_traceinfo {
    pub trace: bool,
    pub nf_trace: bool,
    pub packet_dumped: bool,
    pub type:8: nft_trace_types,
    pub skbid: u32,
    pub basechain: *const nft_base_chain,
}

//
// The gencursor defines two generations, the currently active and the
// next one. Objects contain a bitmask of 2 bits specifying the generations
// they're active in. A set bit means they're inactive in the generation
// represented by that bit.
//
// New objects start out as inactive in the current and active in the
// next generation. When committing the ruleset the bitmask is cleared,
// meaning they're active in all generations. When removing an object,
// it is set inactive in the next generation. After committing the ruleset,
// the objects are removed.
//
// Use READ_ONCE() to prevent refetching the value for atomicity

//
// Generic transaction helpers
//
// Check if this object is currently active.

// Check if this object is active in the next generation.

// This object becomes active in the next generation.

// This object becomes inactive in the next generation.

// After committing the ruleset, clear the stale generation bit.

//
// Set element transaction helpers
//

pub const NFT_SET_ELEM_DEAD_BIT: c_int = 2;

extern "C" {
    pub fn test_bit(_arg: NFT_SET_ELEM_DEAD_BIT, _arg: word) -> return;
}
//
// struct nft_trans - nf_tables object update in transaction
//
// @list: used internally
// @net: struct net
// @table: struct nft_table the object resides in
// @msg_type: message type
// @seq: netlink sequence number
// @flags: modifiers to new request
// @report: notify via unicast netlink message
// @put_net: net needs to be put
//
// This is the information common to all objects in the transaction,
// this must always be the first member of derived sub-types.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nft_trans {
    pub list: list_head,
    pub net: *mut net,
    pub table: *mut nft_table,
    pub msg_type: c_int,
    pub seq: u32,
    pub flags: u16,
    pub report:1: u8,
    pub put_net:1: u8,
}

//
// struct nft_trans_hook - nf_tables hook update in transaction
// @list: used internally
// @hook: struct nft_hook with the device hook
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nft_trans_hook {
    pub list: list_head,
    pub hook: *mut nft_hook,
}

//
// struct nft_trans_binding - nf_tables object with binding support in transaction
// @nft_trans:    base structure, MUST be first member
// @binding_list: list of objects with possible bindings
//
// This is the base type used by objects that can be bound to a chain.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nft_trans_binding {
    pub nft_trans: nft_trans,
    pub binding_list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nft_trans_rule {
    pub nft_trans: nft_trans,
    pub rule: *mut nft_rule,
    pub chain: *mut nft_chain,
    pub flow: *mut nft_flow_rule,
    pub rule_id: u32,
    pub bound: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nft_trans_set {
    pub nft_trans_binding: nft_trans_binding,
    pub list_trans_newset: list_head,
    pub set: *mut nft_set,
    pub set_id: u32,
    pub gc_int: u32,
    pub timeout: u64,
    pub update: bool,
    pub bound: bool,
    pub size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nft_trans_chain {
    pub nft_trans_binding: nft_trans_binding,
    pub chain: *mut nft_chain,
    pub name: *mut c_char,
    pub stats: *mut nft_stats __percpu,
    pub policy: u8,
    pub update: bool,
    pub bound: bool,
    pub chain_id: u32,
    pub basechain: *mut nft_base_chain,
    pub hook_list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nft_trans_table {
    pub nft_trans: nft_trans,
    pub update: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nft_trans_elem_flags {
    NFT_TRANS_UPD_TIMEOUT		= (1 << 0),
    NFT_TRANS_UPD_EXPIRATION	= (1 << 1),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nft_elem_update {
    pub timeout: u64,
    pub expiration: u64,
    pub flags: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nft_trans_one_elem {
    pub priv: *mut nft_elem_priv,
    pub update: *mut nft_elem_update,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nft_trans_elem {
    pub nft_trans: nft_trans,
    pub set: *mut nft_set,
    pub bound: bool,
    pub nelems: c_uint,
    pub __counted_by(nelems): nft_trans_one_elem elems[],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nft_trans_obj {
    pub nft_trans: nft_trans,
    pub obj: *mut nft_object,
    pub newobj: *mut nft_object,
    pub update: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nft_trans_flowtable {
    pub nft_trans: nft_trans,
    pub flowtable: *mut nft_flowtable,
    pub hook_list: list_head,
    pub flags: u32,
    pub update: bool,
}

pub const NFT_TRANS_GC_BATCHCOUNT: c_int = 256;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nft_trans_gc {
    pub list: list_head,
    pub net: *mut net,
    pub set: *mut nft_set,
    pub seq: u32,
    pub count: u16,
    pub priv: [*mut nft_elem_priv; NFT_TRANS_GC_BATCHCOUNT],
    pub rcu: rcu_head,
}

extern "C" {
    pub fn nft_trans_gc_destroy(trans: *mut nft_trans_gc);
}
extern "C" {
    pub fn nft_trans_gc_queue_async_done(gc: *mut nft_trans_gc);
}
extern "C" {
    pub fn nft_trans_gc_queue_sync_done(trans: *mut nft_trans_gc);
}
extern "C" {
    pub fn nft_trans_gc_elem_add(gc: *mut nft_trans_gc, priv: *mut c_void);
}
extern "C" {
    pub fn nft_chain_filter_init() -> int __init;
}
extern "C" {
    pub fn nft_chain_filter_fini();
}
extern "C" {
    pub fn nft_chain_route_init() -> void __init;
}
extern "C" {
    pub fn nft_chain_route_fini();
}
extern "C" {
    pub fn nf_tables_trans_destroy_flush_work(net: *mut net);
}
extern "C" {
    pub fn nf_msecs_to_jiffies64(nla: *const nlattr, result: *mut u64) -> c_int;
}
extern "C" {
    pub fn nf_jiffies64_to_msecs(input: u64) -> __be64;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nftables_pernet {
    pub tables: list_head,
    pub commit_list: list_head,
    pub destroy_list: list_head,
    pub commit_set_list: list_head,
    pub binding_list: list_head,
    pub module_list: list_head,
    pub notify_list: list_head,
    pub set_update_list: list_head,
    pub commit_mutex: mutex,
    pub table_handle: u64,
    pub tstamp: u64,
    pub gc_seq: c_uint,
    pub validate_state: u8,
    pub destroy_work: work_struct,
}

extern "C" {
    pub fn net_generic(_arg: net, _arg: nf_tables_net_id) -> return;
}
