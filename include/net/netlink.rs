//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/netlink.h
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

// ========================================================================
// Netlink Messages and Attributes Interface (As Seen On TV)
// ------------------------------------------------------------------------
// Messages Interface
// ------------------------------------------------------------------------
//
// Message Format:
// <--- nlmsg_total_size(payload)  --->
// <-- nlmsg_msg_size(payload) ->
// +----------+- - -+-------------+- - -+-------- - -
// | nlmsghdr | Pad |   Payload   | Pad | nlmsghdr
// +----------+- - -+-------------+- - -+-------- - -
// nlmsg_data(nlh)---^                   ^
// nlmsg_next(nlh)-----------------------+
//
// Payload Format:
// <---------------------- nlmsg_len(nlh) --------------------->
// <------ hdrlen ------>       <- nlmsg_attrlen(nlh, hdrlen) ->
// +----------------------+- - -+--------------------------------+
// |     Family Header    | Pad |           Attributes           |
// +----------------------+- - -+--------------------------------+
// nlmsg_attrdata(nlh, hdrlen)---^
//
// Data Structures:
// struct nlmsghdr			netlink message header
//
// Message Construction:
// nlmsg_new()			create a new netlink message
// nlmsg_put()			add a netlink message to an skb
// nlmsg_put_answer()			callback based nlmsg_put()
// nlmsg_end()			finalize netlink message
// nlmsg_get_pos()			return current position in message
// nlmsg_trim()			trim part of message
// nlmsg_cancel()			cancel message construction
// nlmsg_consume()			free a netlink message (expected)
// nlmsg_free()			free a netlink message (drop)
//
// Message Sending:
// nlmsg_multicast()			multicast message to several groups
// nlmsg_unicast()			unicast a message to a single socket
// nlmsg_notify()			send notification message
//
// Message Length Calculations:
// nlmsg_msg_size(payload)		length of message w/o padding
// nlmsg_total_size(payload)		length of message w/ padding
// nlmsg_padlen(payload)		length of padding at tail
//
// Message Payload Access:
// nlmsg_data(nlh)			head of message payload
// nlmsg_len(nlh)			length of message payload
// nlmsg_attrdata(nlh, hdrlen)	head of attributes data
// nlmsg_attrlen(nlh, hdrlen)		length of attributes data
//
// Message Parsing:
// nlmsg_ok(nlh, remaining)		does nlh fit into remaining bytes?
// nlmsg_next(nlh, remaining)		get next netlink message
// nlmsg_parse()			parse attributes of a message
// nlmsg_find_attr()			find an attribute in a message
// nlmsg_for_each_msg()		loop over all messages
// nlmsg_validate()			validate netlink message incl. attrs
// nlmsg_for_each_attr()		loop over all attributes
// nlmsg_for_each_attr_type()		loop over all attributes with the
// given type
//
// Misc:
// nlmsg_report()			report back to application?
//
// ------------------------------------------------------------------------
// Attributes Interface
// ------------------------------------------------------------------------
//
// Attribute Format:
// <------- nla_total_size(payload) ------->
// <---- nla_attr_size(payload) ----->
// +----------+- - -+- - - - - - - - - +- - -+-------- - -
// |  Header  | Pad |     Payload      | Pad |  Header
// +----------+- - -+- - - - - - - - - +- - -+-------- - -
// <- nla_len(nla) ->      ^
// nla_data(nla)----^                        |
// nla_next(nla)-----------------------------'
//
// Data Structures:
// struct nlattr			netlink attribute header
//
// Attribute Construction:
// nla_reserve(skb, type, len)	reserve room for an attribute
// nla_reserve_nohdr(skb, len)	reserve room for an attribute w/o hdr
// nla_put(skb, type, len, data)	add attribute to skb
// nla_put_nohdr(skb, len, data)	add attribute w/o hdr
// nla_append(skb, len, data)		append data to skb
//
// Attribute Construction for Basic Types:
// nla_put_u8(skb, type, value)	add u8 attribute to skb
// nla_put_u16(skb, type, value)	add u16 attribute to skb
// nla_put_u32(skb, type, value)	add u32 attribute to skb
// nla_put_u64_64bit(skb, type,
// value, padattr)	add u64 attribute to skb
// nla_put_s8(skb, type, value)	add s8 attribute to skb
// nla_put_s16(skb, type, value)	add s16 attribute to skb
// nla_put_s32(skb, type, value)	add s32 attribute to skb
// nla_put_s64(skb, type, value,
// padattr)		add s64 attribute to skb
// nla_put_string(skb, type, str)	add string attribute to skb
// nla_put_flag(skb, type)		add flag attribute to skb
// nla_put_msecs(skb, type, jiffies,
// padattr)		add msecs attribute to skb
// nla_put_in_addr(skb, type, addr)	add IPv4 address attribute to skb
// nla_put_in6_addr(skb, type, addr)	add IPv6 address attribute to skb
//
// Nested Attributes Construction:
// nla_nest_start(skb, type)		start a nested attribute
// nla_nest_end(skb, nla)		finalize a nested attribute
// nla_nest_cancel(skb, nla)		cancel nested attribute construction
// nla_put_empty_nest(skb, type)	create an empty nest
//
// Attribute Length Calculations:
// nla_attr_size(payload)		length of attribute w/o padding
// nla_total_size(payload)		length of attribute w/ padding
// nla_padlen(payload)		length of padding
//
// Attribute Payload Access:
// nla_data(nla)			head of attribute payload
// nla_len(nla)			length of attribute payload
//
// Attribute Payload Access for Basic Types:
// nla_get_uint(nla)			get payload for a uint attribute
// nla_get_sint(nla)			get payload for a sint attribute
// nla_get_u8(nla)			get payload for a u8 attribute
// nla_get_u16(nla)			get payload for a u16 attribute
// nla_get_u32(nla)			get payload for a u32 attribute
// nla_get_u64(nla)			get payload for a u64 attribute
// nla_get_s8(nla)			get payload for a s8 attribute
// nla_get_s16(nla)			get payload for a s16 attribute
// nla_get_s32(nla)			get payload for a s32 attribute
// nla_get_s64(nla)			get payload for a s64 attribute
// nla_get_flag(nla)			return 1 if flag is true
// nla_get_msecs(nla)			get payload for a msecs attribute
//
// The same functions also exist with _default().
//
// Attribute Misc:
// nla_memcpy(dest, nla, count)	copy attribute into memory
// nla_memcmp(nla, data, size)	compare attribute with memory area
// nla_strscpy(dst, nla, size)	copy attribute to a sized string
// nla_strcmp(nla, str)		compare attribute with string
//
// Attribute Parsing:
// nla_ok(nla, remaining)		does nla fit into remaining bytes?
// nla_next(nla, remaining)		get next netlink attribute
// nla_validate()			validate a stream of attributes
// nla_validate_nested()		validate a stream of nested attributes
// nla_find()				find attribute in stream of attributes
// nla_find_nested()			find attribute in nested attributes
// nla_parse()			parse and validate stream of attrs
// nla_parse_nested()			parse nested attributes
// nla_for_each_attr()		loop over all attributes
// nla_for_each_attr_type()		loop over all attributes with the
// given type
// nla_for_each_nested()		loop over the nested attributes
// nla_for_each_nested_type()		loop over the nested attributes with
// the given type
// =========================================================================
//
// Standard attribute types to specify validation policy
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct netlink_range_validation {
    pub max: u64 min,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct netlink_range_validation_signed {
    pub max: s64 min,,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nla_policy_validation {
    NLA_VALIDATE_NONE,
    NLA_VALIDATE_RANGE,
    NLA_VALIDATE_RANGE_WARN_TOO_LONG,
    NLA_VALIDATE_MIN,
    NLA_VALIDATE_MAX,
    NLA_VALIDATE_MASK,
    NLA_VALIDATE_RANGE_PTR,
    NLA_VALIDATE_FUNCTION,
}

//
// struct nla_policy - attribute validation policy
// @type: Type of attribute or NLA_UNSPEC
// @validation_type: type of attribute validation done in addition to
// type-specific validation (e.g. range, function call), see
// &enum nla_policy_validation
// @len: Type specific length of payload
//
// Policies are defined as arrays of this struct, the array must be
// accessible by attribute type up to the highest identifier to be expected.
//
// Meaning of `len' field:
// NLA_STRING           Maximum length of string
// NLA_NUL_STRING       Maximum length of string (excluding NUL)
// NLA_FLAG             Unused
// NLA_BINARY           Maximum length of attribute payload
// (but see also below with the validation type)
// NLA_NESTED,
// NLA_NESTED_ARRAY     Length verification is done by checking len of
// nested header (or empty); len field is used if
// nested_policy is also used, for the max attr
// number in the nested policy.
// NLA_SINT, NLA_UINT,
// NLA_U8, NLA_U16,
// NLA_U32, NLA_U64,
// NLA_S8, NLA_S16,
// NLA_S32, NLA_S64,
// NLA_BE16, NLA_BE32,
// NLA_MSECS            Leaving the length field zero will verify the
// given type fits, using it verifies minimum length
// just like "All other"
// NLA_BITFIELD32       Unused
// NLA_REJECT           Unused
// All other            Minimum length of attribute payload
//
// Meaning of validation union:
// NLA_BITFIELD32       This is a 32-bit bitmap/bitselector attribute and
// `bitfield32_valid' is the u32 value of valid flags
// NLA_REJECT           This attribute is always rejected and `reject_message'
// may point to a string to report as the error instead
// of the generic one in extended ACK.
// NLA_NESTED           `nested_policy' to a nested policy to validate, must
// also set `len' to the max attribute number. Use the
// provided NLA_POLICY_NESTED() macro.
// Note that nla_parse() will validate, but of course not
// parse, the nested sub-policies.
// NLA_NESTED_ARRAY     `nested_policy' points to a nested policy to validate,
// must also set `len' to the max attribute number. Use
// the provided NLA_POLICY_NESTED_ARRAY() macro.
// The difference to NLA_NESTED is the structure:
// NLA_NESTED has the nested attributes directly inside
// while an array has the nested attributes at another
// level down and the attribute types directly in the
// nesting don't matter.
// NLA_UINT,
// NLA_U8,
// NLA_U16,
// NLA_U32,
// NLA_U64,
// NLA_BE16,
// NLA_BE32,
// NLA_SINT,
// NLA_S8,
// NLA_S16,
// NLA_S32,
// NLA_S64              The `min' and `max' fields are used depending on the
// validation_type field, if that is min/max/range then
// the min, max or both are used (respectively) to check
// the value of the integer attribute.
// Note that in the interest of code simplicity and
// struct size both limits are s16, so you cannot
// enforce a range that doesn't fall within the range
// of s16 - do that using the NLA_POLICY_FULL_RANGE()
// or NLA_POLICY_FULL_RANGE_SIGNED() macros instead.
// Use the NLA_POLICY_MIN(), NLA_POLICY_MAX() and
// NLA_POLICY_RANGE() macros.
// NLA_UINT,
// NLA_U8,
// NLA_U16,
// NLA_U32,
// NLA_U64              If the validation_type field instead is set to
// NLA_VALIDATE_RANGE_PTR, `range' must be a pointer
// to a struct netlink_range_validation that indicates
// the min/max values.
// Use NLA_POLICY_FULL_RANGE().
// NLA_SINT,
// NLA_S8,
// NLA_S16,
// NLA_S32,
// NLA_S64              If the validation_type field instead is set to
// NLA_VALIDATE_RANGE_PTR, `range_signed' must be a
// pointer to a struct netlink_range_validation_signed
// that indicates the min/max values.
// Use NLA_POLICY_FULL_RANGE_SIGNED().
//
// NLA_BINARY           If the validation type is like the ones for integers
// above, then the min/max length (not value like for
// integers) of the attribute is enforced.
//
// All other            Unused - but note that it's a union
//
// Meaning of `validate' field, use via NLA_POLICY_VALIDATE_FN:
// NLA_U8, NLA_U16,
// NLA_U32, NLA_U64,
// NLA_S8, NLA_S16,
// NLA_S32, NLA_S64,
// NLA_MSECS,
// NLA_BINARY           Validation function called for the attribute.
//
// All other            Unused - but note that it's a union
//
// Example:
//
// static const u32 myvalidflags = 0xff231023;
//
// static const struct nla_policy my_policy[ATTR_MAX+1] = {
// [ATTR_FOO] = { .type = NLA_U16 },
// [ATTR_BAR] = { .type = NLA_STRING, .len = BARSIZ },
// [ATTR_BAZ] = NLA_POLICY_EXACT_LEN(sizeof(struct mystruct)),
// [ATTR_GOO] = NLA_POLICY_BITFIELD32(myvalidflags),
// };
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nla_policy {
    pub type: u8,
    pub validation_type: u8,
    pub len: u16,
//
// @strict_start_type: first attribute to validate strictly
//
// This entry is special, and used for the attribute at index 0
// only, and specifies special data about the policy, namely it
// specifies the "boundary type" where strict length validation
// starts for any attribute types >= this value, also, strict
// nesting validation starts here.
//
// Additionally, it means that NLA_UNSPEC is actually NLA_REJECT
// for any types >= this, so need to use NLA_POLICY_MIN_LEN() to
// get the previous pure { .len = xyz } behaviour. The advantage
// of this is that types not specified in the policy will be
// rejected.
//
// For completely new families it should be set to 1 so that the
// validation is enforced for all attributes. For existing ones
// it should be set at least when new attributes are added to
// the enum used by the policy, and be set to the new value that
// was added to enforce strict validation from thereon.
//
    pub strict_start_type: u16,
// private: use NLA_POLICY_*() to set
    pub bitfield32_valid: u32,
    pub mask: u32,
    pub reject_message: *const c_char,
    pub nested_policy: *const nla_policy,
    pub range: *const netlink_range_validation,
    pub range_signed: *const netlink_range_validation_signed,
    pub max: s16 min,,
}

//
// struct nl_info - netlink source information
// @nlh: Netlink message header of original request
// @nl_net: Network namespace
// @portid: Netlink PORTID of requesting application
// @skip_notify: Skip netlink notifications to user space
// @skip_notify_kernel: Skip selected in-kernel notifications
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nl_info {
    pub nlh: *mut nlmsghdr,
    pub nl_net: *mut net,
    pub portid: u32,
}

//
// enum netlink_validation - netlink message/attribute validation levels
// @NL_VALIDATE_LIBERAL: Old-style "be liberal" validation, not caring about
// extra data at the end of the message, attributes being longer than
// they should be, or unknown attributes being present.
// @NL_VALIDATE_TRAILING: Reject junk data encountered after attribute parsing.
// @NL_VALIDATE_MAXTYPE: Reject attributes > max type; Together with _TRAILING
// this is equivalent to the old nla_parse_strict()/nlmsg_parse_strict().
// @NL_VALIDATE_UNSPEC: Reject attributes with NLA_UNSPEC in the policy.
// This can safely be set by the kernel when the given policy has no
// NLA_UNSPEC anymore, and can thus be used to ensure policy entries
// are enforced going forward.
// @NL_VALIDATE_STRICT_ATTRS: strict attribute policy parsing (e.g.
// U8, U16, U32 must have exact size, etc.)
// @NL_VALIDATE_NESTED: Check that NLA_F_NESTED is set for NLA_NESTED(_ARRAY)
// and unset for other policies.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum netlink_validation {
    NL_VALIDATE_LIBERAL = 0,
    NL_VALIDATE_TRAILING = BIT(0),
    NL_VALIDATE_MAXTYPE = BIT(1),
    NL_VALIDATE_UNSPEC = BIT(2),
    NL_VALIDATE_STRICT_ATTRS = BIT(3),
    NL_VALIDATE_NESTED = BIT(4),
}

extern "C" {
    pub fn nla_policy_len(: *const nla_policy, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn nla_strscpy(dst: *mut c_char, nla: *const nlattr, dstsize: usize) -> isize;
}
extern "C" {
    pub fn nla_memcpy(dest: *mut c_void, src: *const nlattr, count: c_int) -> c_int;
}
extern "C" {
    pub fn nla_memcmp(nla: *const nlattr, data: *const c_void, size: usize) -> c_int;
}
extern "C" {
    pub fn nla_strcmp(nla: *const nlattr, str: *const c_char) -> c_int;
}
extern "C" {
    pub fn __nla_put_nohdr(skb: *mut sk_buff, attrlen: c_int, data: *const c_void);
}
extern "C" {
    pub fn nla_put(skb: *mut sk_buff, attrtype: c_int, attrlen: c_int, data: *const c_void) -> c_int;
}
extern "C" {
    pub fn nla_put_nohdr(skb: *mut sk_buff, attrlen: c_int, data: *const c_void) -> c_int;
}
extern "C" {
    pub fn nla_append(skb: *mut sk_buff, attrlen: c_int, data: *const c_void) -> c_int;
}
//
// Netlink Messages
//
// nlmsg_msg_size - length of netlink message not including padding
// @payload: length of message payload
//
// nlmsg_total_size - length of netlink message including padding
// @payload: length of message payload
//
extern "C" {
    pub fn NLMSG_ALIGN(_arg: nlmsg_msg_size(payload)) -> return;
}
//
// nlmsg_padlen - length of padding at the message's tail
// @payload: length of message payload
//
extern "C" {
    pub fn nlmsg_total_size(nlmsg_msg_size(payload: payload) -) -> return;
}
//
// nlmsg_data - head of message payload
// @nlh: netlink message header
//
// nlmsg_len - length of message payload
// @nlh: netlink message header
//
// nlmsg_payload - message payload if the data fits in the len
// @nlh: netlink message header
// @len: struct length
//
// Returns: The netlink message payload/data if the length is sufficient,
// otherwise NULL.
//
extern "C" {
    pub fn nlmsg_data(_arg: nlh) -> return;
}
//
// nlmsg_attrdata - head of attributes data
// @nlh: netlink message header
// @hdrlen: length of family specific header
//
// nlmsg_attrlen - length of attributes data
// @nlh: netlink message header
// @hdrlen: length of family specific header
//
extern "C" {
    pub fn nlmsg_len(NLMSG_ALIGN(hdrlen: nlh) -) -> return;
}
//
// nlmsg_ok - check if the netlink message fits into the remaining bytes
// @nlh: netlink message header
// @remaining: number of bytes remaining in message stream
//
// nlmsg_next - next netlink message in message stream
// @nlh: netlink message header
// @remaining: number of bytes remaining in message stream
//
// Returns: the next netlink message in the message stream and
// decrements remaining by the size of the current message.
//
// remaining -= totlen;
//
// nla_parse - Parse a stream of attributes into a tb buffer
// @tb: destination array with maxtype+1 elements
// @maxtype: maximum attribute type to be expected
// @head: head of attribute stream
// @len: length of attribute stream
// @policy: validation policy
// @extack: extended ACK pointer
//
// Parses a stream of attributes and stores a pointer to each attribute in
// the tb array accessible via the attribute type. Attributes with a type
// exceeding maxtype will be rejected, policy must be specified, attributes
// will be validated in the strictest way possible.
//
// Returns: 0 on success or a negative error code.
//
// nla_parse_deprecated - Parse a stream of attributes into a tb buffer
// @tb: destination array with maxtype+1 elements
// @maxtype: maximum attribute type to be expected
// @head: head of attribute stream
// @len: length of attribute stream
// @policy: validation policy
// @extack: extended ACK pointer
//
// Parses a stream of attributes and stores a pointer to each attribute in
// the tb array accessible via the attribute type. Attributes with a type
// exceeding maxtype will be ignored and attributes from the policy are not
// always strictly validated (only for new attributes).
//
// Returns: 0 on success or a negative error code.
//
// nla_parse_deprecated_strict - Parse a stream of attributes into a tb buffer
// @tb: destination array with maxtype+1 elements
// @maxtype: maximum attribute type to be expected
// @head: head of attribute stream
// @len: length of attribute stream
// @policy: validation policy
// @extack: extended ACK pointer
//
// Parses a stream of attributes and stores a pointer to each attribute in
// the tb array accessible via the attribute type. Attributes with a type
// exceeding maxtype will be rejected as well as trailing data, but the
// policy is not completely strictly validated (only for new attributes).
//
// Returns: 0 on success or a negative error code.
//
// __nlmsg_parse - parse attributes of a netlink message
// @nlh: netlink message header
// @hdrlen: length of family specific header
// @tb: destination array with maxtype+1 elements
// @maxtype: maximum attribute type to be expected
// @policy: validation policy
// @validate: validation strictness
// @extack: extended ACK report struct
//
// See nla_parse()
//
// nlmsg_parse - parse attributes of a netlink message
// @nlh: netlink message header
// @hdrlen: length of family specific header
// @tb: destination array with maxtype+1 elements
// @maxtype: maximum attribute type to be expected
// @policy: validation policy
// @extack: extended ACK report struct
//
// See nla_parse()
//
// nlmsg_parse_deprecated - parse attributes of a netlink message
// @nlh: netlink message header
// @hdrlen: length of family specific header
// @tb: destination array with maxtype+1 elements
// @maxtype: maximum attribute type to be expected
// @policy: validation policy
// @extack: extended ACK report struct
//
// See nla_parse_deprecated()
//
// nlmsg_parse_deprecated_strict - parse attributes of a netlink message
// @nlh: netlink message header
// @hdrlen: length of family specific header
// @tb: destination array with maxtype+1 elements
// @maxtype: maximum attribute type to be expected
// @policy: validation policy
// @extack: extended ACK report struct
//
// See nla_parse_deprecated_strict()
//
// nlmsg_find_attr - find a specific attribute in a netlink message
// @nlh: netlink message header
// @hdrlen: length of family specific header
// @attrtype: type of attribute to look for
//
// Returns: the first attribute which matches the specified type.
//
// nla_validate_deprecated - Validate a stream of attributes
// @head: head of attribute stream
// @len: length of attribute stream
// @maxtype: maximum attribute type to be expected
// @policy: validation policy
// @extack: extended ACK report struct
//
// Validates all attributes in the specified attribute stream against the
// specified policy. Validation is done in liberal mode.
// See documentation of struct nla_policy for more details.
//
// Returns: 0 on success or a negative error code.
//
// nla_validate - Validate a stream of attributes
// @head: head of attribute stream
// @len: length of attribute stream
// @maxtype: maximum attribute type to be expected
// @policy: validation policy
// @extack: extended ACK report struct
//
// Validates all attributes in the specified attribute stream against the
// specified policy. Validation is done in strict mode.
// See documentation of struct nla_policy for more details.
//
// Returns: 0 on success or a negative error code.
//
// nlmsg_validate_deprecated - validate a netlink message including attributes
// @nlh: netlinket message header
// @hdrlen: length of family specific header
// @maxtype: maximum attribute type to be expected
// @policy: validation policy
// @extack: extended ACK report struct
//
// nlmsg_report - need to report back to application?
// @nlh: netlink message header
//
// Returns: 1 if a report back to the application is requested.
//
// nlmsg_seq - return the seq number of netlink message
// @nlh: netlink message header
//
// Returns: 0 if netlink message is NULL
//
// nlmsg_for_each_attr - iterate over a stream of attributes
// @pos: loop counter, set to current attribute
// @nlh: netlink message header
// @hdrlen: length of family specific header
// @rem: initialized to len, holds bytes currently remaining in stream
//

//
// nlmsg_for_each_attr_type - iterate over a stream of attributes
// @pos: loop counter, set to the current attribute
// @type: required attribute type for @pos
// @nlh: netlink message header
// @hdrlen: length of the family specific header
// @rem: initialized to len, holds bytes currently remaining in stream
//

//
// nlmsg_put - Add a new netlink message to an skb
// @skb: socket buffer to store message in
// @portid: netlink PORTID of requesting application
// @seq: sequence number of message
// @type: message type
// @payload: length of message payload
// @flags: message flags
//
// Returns: NULL if the tailroom of the skb is insufficient to store
// the message header and payload.
//
extern "C" {
    pub fn __nlmsg_put(_arg: skb, _arg: portid, _arg: seq, _arg: type, _arg: payload, _arg: flags) -> return;
}
//
// nlmsg_append - Add more data to a nlmsg in a skb
// @skb: socket buffer to store message in
// @size: length of message payload
//
// Append data to an existing nlmsg, used when constructing a message
// with multiple fixed-format headers (which is rare).
// Returns: NULL if the tailroom of the skb is insufficient to store
// the extra payload.
//
extern "C" {
    pub fn __skb_put(_arg: skb, _arg: NLMSG_ALIGN(size)) -> return;
}
//
// nlmsg_put_answer - Add a new callback based netlink message to an skb
// @skb: socket buffer to store message in
// @cb: netlink callback
// @type: message type
// @payload: length of message payload
// @flags: message flags
//
// Returns: NULL if the tailroom of the skb is insufficient to store
// the message header and payload.
//
// nlmsg_new - Allocate a new netlink message
// @payload: size of the message payload
// @flags: the type of memory to allocate.
//
// Use NLMSG_DEFAULT_SIZE if the size of the payload isn't known
// and a good default is needed.
//
extern "C" {
    pub fn alloc_skb(_arg: nlmsg_total_size(payload), _arg: flags) -> return;
}
//
// nlmsg_new_large - Allocate a new netlink message with non-contiguous
// physical memory
// @payload: size of the message payload
//
// The allocated skb is unable to have frag page for shinfo->frags*,
// as the NULL setting for skb->head in netlink_skb_destructor() will
// bypass most of the handling in skb_release_data()
//
extern "C" {
    pub fn netlink_alloc_large_skb(_arg: nlmsg_total_size(payload), _arg: 0) -> return;
}
//
// nlmsg_end - Finalize a netlink message
// @skb: socket buffer the message is stored in
// @nlh: netlink message header
//
// Corrects the netlink message header to include the appended
// attributes. Only necessary if attributes have been added to
// the message.
//
// nlmsg_get_pos - return current position in netlink message
// @skb: socket buffer the message is stored in
//
// Returns: a pointer to the current tail of the message.
//
extern "C" {
    pub fn skb_tail_pointer(_arg: skb) -> return;
}
//
// nlmsg_trim - Trim message to a mark
// @skb: socket buffer the message is stored in
// @mark: mark to trim to
//
// Trims the message to the provided mark.
//
// nlmsg_cancel - Cancel construction of a netlink message
// @skb: socket buffer the message is stored in
// @nlh: netlink message header
//
// Removes the complete netlink message including all
// attributes from the socket buffer again.
//
// nlmsg_free - drop a netlink message
// @skb: socket buffer of netlink message
//
// nlmsg_consume - free a netlink message
// @skb: socket buffer of netlink message
//
// nlmsg_multicast_filtered - multicast a netlink message with filter function
// @sk: netlink socket to spread messages to
// @skb: netlink message as socket buffer
// @portid: own netlink portid to avoid sending to yourself
// @group: multicast group id
// @flags: allocation flags
// @filter: filter function
// @filter_data: filter function private data
//
// Return: 0 on success, negative error code for failure.
//
// nlmsg_multicast - multicast a netlink message
// @sk: netlink socket to spread messages to
// @skb: netlink message as socket buffer
// @portid: own netlink portid to avoid sending to yourself
// @group: multicast group id
// @flags: allocation flags
//
// nlmsg_unicast - unicast a netlink message
// @sk: netlink socket to spread message to
// @skb: netlink message as socket buffer
// @portid: netlink portid of the destination socket
//
// nlmsg_for_each_msg - iterate over a stream of messages
// @pos: loop counter, set to current message
// @head: head of message stream
// @len: length of message stream
// @rem: initialized to len, holds bytes currently remaining in stream
//

//
// nl_dump_check_consistent - check if sequence is consistent and advertise if not
// @cb: netlink callback structure that stores the sequence number
// @nlh: netlink message header to write the flag to
//
// This function checks if the sequence (generation) number changed during dump
// and if it did, advertises it in the netlink message header.
//
// The correct way to use it is to set cb->seq to the generation counter when
// all locks for dumping have been acquired, and then call this function for
// each message that is generated.
//
// Note that due to initialisation concerns, 0 is an invalid sequence number
// and must not be used by code that uses this functionality.
//
// Netlink Attributes
//
// nla_attr_size - length of attribute not including padding
// @payload: length of payload
//
// nla_total_size - total length of attribute including padding
// @payload: length of payload
//
extern "C" {
    pub fn NLA_ALIGN(_arg: nla_attr_size(payload)) -> return;
}
//
// nla_padlen - length of padding at the tail of attribute
// @payload: length of payload
//
extern "C" {
    pub fn nla_total_size(nla_attr_size(payload: payload) -) -> return;
}
//
// nla_type - attribute type
// @nla: netlink attribute
//
// nla_data - head of payload
// @nla: netlink attribute
//
// nla_len - length of payload
// @nla: netlink attribute
//
// nla_ok - check if the netlink attribute fits into the remaining bytes
// @nla: netlink attribute
// @remaining: number of bytes remaining in attribute stream
//
// nla_next - next netlink attribute in attribute stream
// @nla: netlink attribute
// @remaining: number of bytes remaining in attribute stream
//
// Returns: the next netlink attribute in the attribute stream and
// decrements remaining by the size of the current attribute.
//
// remaining -= totlen;
//
// nla_find_nested - find attribute in a set of nested attributes
// @nla: attribute containing the nested attributes
// @attrtype: type of attribute to look for
//
// Returns: the first attribute which matches the specified type.
//
extern "C" {
    pub fn nla_find(_arg: nla_data(nla), _arg: nla_len(nla), _arg: attrtype) -> return;
}
//
// nla_parse_nested - parse nested attributes
// @tb: destination array with maxtype+1 elements
// @maxtype: maximum attribute type to be expected
// @nla: attribute containing the nested attributes
// @policy: validation policy
// @extack: extended ACK report struct
//
// See nla_parse()
//
// nla_parse_nested_deprecated - parse nested attributes
// @tb: destination array with maxtype+1 elements
// @maxtype: maximum attribute type to be expected
// @nla: attribute containing the nested attributes
// @policy: validation policy
// @extack: extended ACK report struct
//
// See nla_parse_deprecated()
//
// nla_put_u8 - Add a u8 netlink attribute to a socket buffer
// @skb: socket buffer to add attribute to
// @attrtype: attribute type
// @value: numeric value
//
// temporary variables to work around GCC PR81715 with asan-stack=1
extern "C" {
    pub fn nla_put(_arg: skb, _arg: attrtype, _arg: sizeof(u8), _arg: &tmp) -> return;
}
//
// nla_put_u16 - Add a u16 netlink attribute to a socket buffer
// @skb: socket buffer to add attribute to
// @attrtype: attribute type
// @value: numeric value
//
extern "C" {
    pub fn nla_put(_arg: skb, _arg: attrtype, _arg: sizeof(u16), _arg: &tmp) -> return;
}
//
// nla_put_be16 - Add a __be16 netlink attribute to a socket buffer
// @skb: socket buffer to add attribute to
// @attrtype: attribute type
// @value: numeric value
//
extern "C" {
    pub fn nla_put(_arg: skb, _arg: attrtype, _arg: sizeof(__be16), _arg: &tmp) -> return;
}
//
// nla_put_net16 - Add 16-bit network byte order netlink attribute to a socket buffer
// @skb: socket buffer to add attribute to
// @attrtype: attribute type
// @value: numeric value
//
extern "C" {
    pub fn nla_put_be16(_arg: skb, NLA_F_NET_BYTEORDER: attrtype |, _arg: tmp) -> return;
}
//
// nla_put_le16 - Add a __le16 netlink attribute to a socket buffer
// @skb: socket buffer to add attribute to
// @attrtype: attribute type
// @value: numeric value
//
extern "C" {
    pub fn nla_put(_arg: skb, _arg: attrtype, _arg: sizeof(__le16), _arg: &tmp) -> return;
}
//
// nla_put_u32 - Add a u32 netlink attribute to a socket buffer
// @skb: socket buffer to add attribute to
// @attrtype: attribute type
// @value: numeric value
//
extern "C" {
    pub fn nla_put(_arg: skb, _arg: attrtype, _arg: sizeof(u32), _arg: &tmp) -> return;
}
//
// nla_put_uint - Add a variable-size unsigned int to a socket buffer
// @skb: socket buffer to add attribute to
// @attrtype: attribute type
// @value: numeric value
//
extern "C" {
    pub fn nla_put_u32(_arg: skb, _arg: attrtype, _arg: tmp32) -> return;
}
extern "C" {
    pub fn nla_put(_arg: skb, _arg: attrtype, _arg: sizeof(u64), _arg: &tmp64) -> return;
}
//
// nla_put_be32 - Add a __be32 netlink attribute to a socket buffer
// @skb: socket buffer to add attribute to
// @attrtype: attribute type
// @value: numeric value
//
extern "C" {
    pub fn nla_put(_arg: skb, _arg: attrtype, _arg: sizeof(__be32), _arg: &tmp) -> return;
}
//
// nla_put_net32 - Add 32-bit network byte order netlink attribute to a socket buffer
// @skb: socket buffer to add attribute to
// @attrtype: attribute type
// @value: numeric value
//
extern "C" {
    pub fn nla_put_be32(_arg: skb, NLA_F_NET_BYTEORDER: attrtype |, _arg: tmp) -> return;
}
//
// nla_put_le32 - Add a __le32 netlink attribute to a socket buffer
// @skb: socket buffer to add attribute to
// @attrtype: attribute type
// @value: numeric value
//
extern "C" {
    pub fn nla_put(_arg: skb, _arg: attrtype, _arg: sizeof(__le32), _arg: &tmp) -> return;
}
//
// nla_put_u64_64bit - Add a u64 netlink attribute to a skb and align it
// @skb: socket buffer to add attribute to
// @attrtype: attribute type
// @value: numeric value
// @padattr: attribute type for the padding
//
extern "C" {
    pub fn nla_put_64bit(_arg: skb, _arg: attrtype, _arg: sizeof(u64), _arg: &tmp, _arg: padattr) -> return;
}
//
// nla_put_be64 - Add a __be64 netlink attribute to a socket buffer and align it
// @skb: socket buffer to add attribute to
// @attrtype: attribute type
// @value: numeric value
// @padattr: attribute type for the padding
//
extern "C" {
    pub fn nla_put_64bit(_arg: skb, _arg: attrtype, _arg: sizeof(__be64), _arg: &tmp, _arg: padattr) -> return;
}
//
// nla_put_net64 - Add 64-bit network byte order nlattr to a skb and align it
// @skb: socket buffer to add attribute to
// @attrtype: attribute type
// @value: numeric value
// @padattr: attribute type for the padding
//
// nla_put_le64 - Add a __le64 netlink attribute to a socket buffer and align it
// @skb: socket buffer to add attribute to
// @attrtype: attribute type
// @value: numeric value
// @padattr: attribute type for the padding
//
extern "C" {
    pub fn nla_put_64bit(_arg: skb, _arg: attrtype, _arg: sizeof(__le64), _arg: &tmp, _arg: padattr) -> return;
}
//
// nla_put_s8 - Add a s8 netlink attribute to a socket buffer
// @skb: socket buffer to add attribute to
// @attrtype: attribute type
// @value: numeric value
//
extern "C" {
    pub fn nla_put(_arg: skb, _arg: attrtype, _arg: sizeof(s8), _arg: &tmp) -> return;
}
//
// nla_put_s16 - Add a s16 netlink attribute to a socket buffer
// @skb: socket buffer to add attribute to
// @attrtype: attribute type
// @value: numeric value
//
extern "C" {
    pub fn nla_put(_arg: skb, _arg: attrtype, _arg: sizeof(s16), _arg: &tmp) -> return;
}
//
// nla_put_s32 - Add a s32 netlink attribute to a socket buffer
// @skb: socket buffer to add attribute to
// @attrtype: attribute type
// @value: numeric value
//
extern "C" {
    pub fn nla_put(_arg: skb, _arg: attrtype, _arg: sizeof(s32), _arg: &tmp) -> return;
}
//
// nla_put_s64 - Add a s64 netlink attribute to a socket buffer and align it
// @skb: socket buffer to add attribute to
// @attrtype: attribute type
// @value: numeric value
// @padattr: attribute type for the padding
//
extern "C" {
    pub fn nla_put_64bit(_arg: skb, _arg: attrtype, _arg: sizeof(s64), _arg: &tmp, _arg: padattr) -> return;
}
//
// nla_put_sint - Add a variable-size signed int to a socket buffer
// @skb: socket buffer to add attribute to
// @attrtype: attribute type
// @value: numeric value
//
extern "C" {
    pub fn nla_put_s32(_arg: skb, _arg: attrtype, _arg: tmp32) -> return;
}
extern "C" {
    pub fn nla_put(_arg: skb, _arg: attrtype, _arg: sizeof(s64), _arg: &tmp64) -> return;
}
//
// nla_put_string - Add a string netlink attribute to a socket buffer
// @skb: socket buffer to add attribute to
// @attrtype: attribute type
// @str: NUL terminated string
//
extern "C" {
    pub fn nla_put(_arg: skb, _arg: attrtype, 1: strlen(str) +, _arg: str) -> return;
}
//
// nla_put_flag - Add a flag netlink attribute to a socket buffer
// @skb: socket buffer to add attribute to
// @attrtype: attribute type
//
extern "C" {
    pub fn nla_put(_arg: skb, _arg: attrtype, _arg: 0, _arg: NULL) -> return;
}
//
// nla_put_msecs - Add a msecs netlink attribute to a skb and align it
// @skb: socket buffer to add attribute to
// @attrtype: attribute type
// @njiffies: number of jiffies to convert to msecs
// @padattr: attribute type for the padding
//
extern "C" {
    pub fn nla_put_64bit(_arg: skb, _arg: attrtype, _arg: sizeof(u64), _arg: &tmp, _arg: padattr) -> return;
}
//
// nla_put_in_addr - Add an IPv4 address netlink attribute to a socket
// buffer
// @skb: socket buffer to add attribute to
// @attrtype: attribute type
// @addr: IPv4 address
//
extern "C" {
    pub fn nla_put_be32(_arg: skb, _arg: attrtype, _arg: tmp) -> return;
}
//
// nla_put_in6_addr - Add an IPv6 address netlink attribute to a socket
// buffer
// @skb: socket buffer to add attribute to
// @attrtype: attribute type
// @addr: IPv6 address
//
extern "C" {
    pub fn nla_put(_arg: skb, _arg: attrtype, _arg: *mut sizeof(addr), _arg: addr) -> return;
}
//
// nla_put_bitfield32 - Add a bitfield32 netlink attribute to a socket buffer
// @skb: socket buffer to add attribute to
// @attrtype: attribute type
// @value: value carrying bits
// @selector: selector of valid bits
//
extern "C" {
    pub fn nla_put(_arg: skb, _arg: attrtype, _arg: sizeof(tmp), _arg: &tmp) -> return;
}
//
// nla_get_u32 - return payload of u32 attribute
// @nla: u32 netlink attribute
//
// nla_get_u32_default - return payload of u32 attribute or default
// @nla: u32 netlink attribute, may be %NULL
// @defvalue: default value to use if @nla is %NULL
//
// Return: the value of the attribute, or the default value if not present
//
extern "C" {
    pub fn nla_get_u32(_arg: nla) -> return;
}
//
// nla_get_be32 - return payload of __be32 attribute
// @nla: __be32 netlink attribute
//
// nla_get_be32_default - return payload of be32 attribute or default
// @nla: __be32 netlink attribute, may be %NULL
// @defvalue: default value to use if @nla is %NULL
//
// Return: the value of the attribute, or the default value if not present
//
extern "C" {
    pub fn nla_get_be32(_arg: nla) -> return;
}
//
// nla_get_le32 - return payload of __le32 attribute
// @nla: __le32 netlink attribute
//
// nla_get_le32_default - return payload of le32 attribute or default
// @nla: __le32 netlink attribute, may be %NULL
// @defvalue: default value to use if @nla is %NULL
//
// Return: the value of the attribute, or the default value if not present
//
extern "C" {
    pub fn nla_get_le32(_arg: nla) -> return;
}
//
// nla_get_u16 - return payload of u16 attribute
// @nla: u16 netlink attribute
//
// nla_get_u16_default - return payload of u16 attribute or default
// @nla: u16 netlink attribute, may be %NULL
// @defvalue: default value to use if @nla is %NULL
//
// Return: the value of the attribute, or the default value if not present
//
extern "C" {
    pub fn nla_get_u16(_arg: nla) -> return;
}
//
// nla_get_be16 - return payload of __be16 attribute
// @nla: __be16 netlink attribute
//
// nla_get_be16_default - return payload of be16 attribute or default
// @nla: __be16 netlink attribute, may be %NULL
// @defvalue: default value to use if @nla is %NULL
//
// Return: the value of the attribute, or the default value if not present
//
extern "C" {
    pub fn nla_get_be16(_arg: nla) -> return;
}
//
// nla_get_le16 - return payload of __le16 attribute
// @nla: __le16 netlink attribute
//
// nla_get_le16_default - return payload of le16 attribute or default
// @nla: __le16 netlink attribute, may be %NULL
// @defvalue: default value to use if @nla is %NULL
//
// Return: the value of the attribute, or the default value if not present
//
extern "C" {
    pub fn nla_get_le16(_arg: nla) -> return;
}
//
// nla_get_u8 - return payload of u8 attribute
// @nla: u8 netlink attribute
//
// nla_get_u8_default - return payload of u8 attribute or default
// @nla: u8 netlink attribute, may be %NULL
// @defvalue: default value to use if @nla is %NULL
//
// Return: the value of the attribute, or the default value if not present
//
extern "C" {
    pub fn nla_get_u8(_arg: nla) -> return;
}
//
// nla_get_u64 - return payload of u64 attribute
// @nla: u64 netlink attribute
//
// nla_get_u64_default - return payload of u64 attribute or default
// @nla: u64 netlink attribute, may be %NULL
// @defvalue: default value to use if @nla is %NULL
//
// Return: the value of the attribute, or the default value if not present
//
extern "C" {
    pub fn nla_get_u64(_arg: nla) -> return;
}
//
// nla_get_uint - return payload of uint attribute
// @nla: uint netlink attribute
//
extern "C" {
    pub fn nla_get_u32(_arg: nla) -> return;
}
extern "C" {
    pub fn nla_get_u64(_arg: nla) -> return;
}
//
// nla_get_uint_default - return payload of uint attribute or default
// @nla: uint netlink attribute, may be %NULL
// @defvalue: default value to use if @nla is %NULL
//
// Return: the value of the attribute, or the default value if not present
//
extern "C" {
    pub fn nla_get_uint(_arg: nla) -> return;
}
//
// nla_get_be64 - return payload of __be64 attribute
// @nla: __be64 netlink attribute
//
// nla_get_be64_default - return payload of be64 attribute or default
// @nla: __be64 netlink attribute, may be %NULL
// @defvalue: default value to use if @nla is %NULL
//
// Return: the value of the attribute, or the default value if not present
//
extern "C" {
    pub fn nla_get_be64(_arg: nla) -> return;
}
//
// nla_get_le64 - return payload of __le64 attribute
// @nla: __le64 netlink attribute
//
// nla_get_le64_default - return payload of le64 attribute or default
// @nla: __le64 netlink attribute, may be %NULL
// @defvalue: default value to use if @nla is %NULL
//
// Return: the value of the attribute, or the default value if not present
//
extern "C" {
    pub fn nla_get_le64(_arg: nla) -> return;
}
//
// nla_get_s32 - return payload of s32 attribute
// @nla: s32 netlink attribute
//
// nla_get_s32_default - return payload of s32 attribute or default
// @nla: s32 netlink attribute, may be %NULL
// @defvalue: default value to use if @nla is %NULL
//
// Return: the value of the attribute, or the default value if not present
//
extern "C" {
    pub fn nla_get_s32(_arg: nla) -> return;
}
//
// nla_get_s16 - return payload of s16 attribute
// @nla: s16 netlink attribute
//
// nla_get_s16_default - return payload of s16 attribute or default
// @nla: s16 netlink attribute, may be %NULL
// @defvalue: default value to use if @nla is %NULL
//
// Return: the value of the attribute, or the default value if not present
//
extern "C" {
    pub fn nla_get_s16(_arg: nla) -> return;
}
//
// nla_get_s8 - return payload of s8 attribute
// @nla: s8 netlink attribute
//
// nla_get_s8_default - return payload of s8 attribute or default
// @nla: s8 netlink attribute, may be %NULL
// @defvalue: default value to use if @nla is %NULL
//
// Return: the value of the attribute, or the default value if not present
//
extern "C" {
    pub fn nla_get_s8(_arg: nla) -> return;
}
//
// nla_get_s64 - return payload of s64 attribute
// @nla: s64 netlink attribute
//
// nla_get_s64_default - return payload of s64 attribute or default
// @nla: s64 netlink attribute, may be %NULL
// @defvalue: default value to use if @nla is %NULL
//
// Return: the value of the attribute, or the default value if not present
//
extern "C" {
    pub fn nla_get_s64(_arg: nla) -> return;
}
//
// nla_get_sint - return payload of uint attribute
// @nla: uint netlink attribute
//
extern "C" {
    pub fn nla_get_s32(_arg: nla) -> return;
}
extern "C" {
    pub fn nla_get_s64(_arg: nla) -> return;
}
//
// nla_get_sint_default - return payload of sint attribute or default
// @nla: sint netlink attribute, may be %NULL
// @defvalue: default value to use if @nla is %NULL
//
// Return: the value of the attribute, or the default value if not present
//
extern "C" {
    pub fn nla_get_sint(_arg: nla) -> return;
}
//
// nla_get_flag - return payload of flag attribute
// @nla: flag netlink attribute
//
// nla_get_msecs - return payload of msecs attribute
// @nla: msecs netlink attribute
//
// Returns: the number of milliseconds in jiffies.
//
extern "C" {
    pub fn msecs_to_jiffies(msecs: (unsigned long)) -> return;
}
//
// nla_get_msecs_default - return payload of msecs attribute or default
// @nla: msecs netlink attribute, may be %NULL
// @defvalue: default value to use if @nla is %NULL
//
// Return: the value of the attribute, or the default value if not present
//
extern "C" {
    pub fn nla_get_msecs(_arg: nla) -> return;
}
//
// nla_get_in_addr - return payload of IPv4 address attribute
// @nla: IPv4 address netlink attribute
//
// nla_get_in_addr_default - return payload of be32 attribute or default
// @nla: IPv4 address netlink attribute, may be %NULL
// @defvalue: default value to use if @nla is %NULL
//
// Return: the value of the attribute, or the default value if not present
//
extern "C" {
    pub fn nla_get_in_addr(_arg: nla) -> return;
}
//
// nla_get_in6_addr - return payload of IPv6 address attribute
// @nla: IPv6 address netlink attribute
//
// nla_get_bitfield32 - return payload of 32 bitfield attribute
// @nla: nla_bitfield32 attribute
//
// nla_memdup - duplicate attribute memory (kmemdup)
// @src: netlink attribute to duplicate from
// @gfp: GFP mask
//
extern "C" {
    pub fn kmemdup_noprof(_arg: nla_data(src), _arg: nla_len(src), _arg: gfp) -> return;
}

//
// nla_nest_start_noflag - Start a new level of nested attributes
// @skb: socket buffer to add attributes to
// @attrtype: attribute type of container
//
// This function exists for backward compatibility to use in APIs which never
// marked their nest attributes with NLA_F_NESTED flag. New APIs should use
// nla_nest_start() which sets the flag.
//
// Returns: the container attribute or NULL on error
//
// nla_nest_start - Start a new level of nested attributes, with NLA_F_NESTED
// @skb: socket buffer to add attributes to
// @attrtype: attribute type of container
//
// Unlike nla_nest_start_noflag(), mark the nest attribute with NLA_F_NESTED
// flag. This is the preferred function to use in new code.
//
// Returns: the container attribute or NULL on error
//
extern "C" {
    pub fn nla_nest_start_noflag(_arg: skb, NLA_F_NESTED: attrtype |) -> return;
}
//
// nla_nest_end - Finalize nesting of attributes
// @skb: socket buffer the attributes are stored in
// @start: container attribute
//
// Corrects the container attribute header to include the all
// appended attributes.
//
// Returns: the total data length of the skb.
//
// nla_nest_end_safe - Validate and finalize nesting of attributes
// @skb: socket buffer the attributes are stored in
// @start: container attribute
//
// Corrects the container attribute header to include all appended
// attributes.
//
// Returns: the total data length of the skb, or -EMSGSIZE if the
// nested attribute length exceeds U16_MAX.
//
extern "C" {
    pub fn nla_nest_end(_arg: skb, _arg: start) -> return;
}
//
// nla_nest_cancel - Cancel nesting of attributes
// @skb: socket buffer the message is stored in
// @start: container attribute
//
// Removes the container attribute and including all nested
// attributes. Returns -EMSGSIZE
//
// nla_put_empty_nest - Create an empty nest
// @skb: socket buffer the message is stored in
// @attrtype: attribute type of the container
//
// This function is a helper for creating empty nests.
//
// Returns: 0 when successful or -EMSGSIZE on failure.
//
// __nla_validate_nested - Validate a stream of nested attributes
// @start: container attribute
// @maxtype: maximum attribute type to be expected
// @policy: validation policy
// @validate: validation strictness
// @extack: extended ACK report struct
//
// Validates all attributes in the nested attribute stream against the
// specified policy. Attributes with a type exceeding maxtype will be
// ignored. See documentation of struct nla_policy for more details.
//
// Returns: 0 on success or a negative error code.
//
// nla_need_padding_for_64bit - test 64-bit alignment of the next attribute
// @skb: socket buffer the message is stored in
//
// Return: true if padding is needed to align the next attribute (nla_data()) to
// a 64-bit aligned area.
//

// The nlattr header is 4 bytes in size, that's why we test
// if the skb->data _is_ aligned.  A NOP attribute, plus
// nlattr header for next attribute, will make nla_data()
// 8-byte aligned.
//

//
// nla_align_64bit - 64-bit align the nla_data() of next attribute
// @skb: socket buffer the message is stored in
// @padattr: attribute type for the padding
//
// Conditionally emit a padding netlink attribute in order to make
// the next attribute we emit have a 64-bit aligned nla_data() area.
// This will only be done in architectures which do not have
// CONFIG_HAVE_EFFICIENT_UNALIGNED_ACCESS defined.
//
// Returns: zero on success or a negative error code.
//
// nla_total_size_64bit - total length of attribute including padding
// @payload: length of payload
//

//
// nla_for_each_attr - iterate over a stream of attributes
// @pos: loop counter, set to current attribute
// @head: head of attribute stream
// @len: length of attribute stream
// @rem: initialized to len, holds bytes currently remaining in stream
//

//
// nla_for_each_attr_type - iterate over a stream of attributes
// @pos: loop counter, set to current attribute
// @type: required attribute type for @pos
// @head: head of attribute stream
// @len: length of attribute stream
// @rem: initialized to len, holds bytes currently remaining in stream
//

//
// nla_for_each_nested - iterate over nested attributes
// @pos: loop counter, set to current attribute
// @nla: attribute containing the nested attributes
// @rem: initialized to len, holds bytes currently remaining in stream
//

//
// nla_for_each_nested_type - iterate over nested attributes
// @pos: loop counter, set to current attribute
// @type: required attribute type for @pos
// @nla: attribute containing the nested attributes
// @rem: initialized to len, holds bytes currently remaining in stream
//

//
// nla_is_last - Test if attribute is last in stream
// @nla: attribute to test
// @rem: bytes remaining in stream
//
extern "C" {
    pub fn netlink_policy_dump_loop(state: *mut netlink_policy_dump_state) -> bool;
}
extern "C" {
    pub fn netlink_policy_dump_attr_size_estimate(pt: *const nla_policy) -> c_int;
}
extern "C" {
    pub fn netlink_policy_dump_free(state: *mut netlink_policy_dump_state);
}
