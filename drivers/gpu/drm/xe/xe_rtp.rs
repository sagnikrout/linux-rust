//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_rtp.h
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


// SPDX-License-Identifier: MIT
//
// Copyright © 2022 Intel Corporation
//

//
// Register table poke infrastructure
//
// Macros to encode rules to match against platform, IP version, stepping, etc.
// Shouldn't be used directly - see XE_RTP_RULES()
//

//
// XE_RTP_RULE_PLATFORM - Create rule matching platform
// @plat_: platform to match
//
// Refer to XE_RTP_RULES() for expected usage.
//

//
// XE_RTP_RULE_SUBPLATFORM - Create rule matching platform and sub-platform
// @plat_: platform to match
// @sub_: sub-platform to match
//
// Refer to XE_RTP_RULES() for expected usage.
//

//
// XE_RTP_RULE_PLATFORM_STEP - Create rule matching platform-level stepping
// @start_: First stepping matching the rule
// @end_: First stepping that does not match the rule
//
// Note that the range matching this rule is [ @start_, @end_ ), i.e. inclusive
// on the left, exclusive on the right.
//
// You need to make sure that proper support for reading platform-level stepping
// information is present for the target platform before using this rule.
//
// Refer to XE_RTP_RULES() for expected usage.
//

//
// XE_RTP_RULE_GRAPHICS_STEP - Create rule matching graphics stepping
// @start_: First stepping matching the rule
// @end_: First stepping that does not match the rule
//
// Note that the range matching this rule is [ @start_, @end_ ), i.e. inclusive
// on the left, exclusive on the right.
//
// Refer to XE_RTP_RULES() for expected usage.
//

//
// XE_RTP_RULE_MEDIA_STEP - Create rule matching media stepping
// @start_: First stepping matching the rule
// @end_: First stepping that does not match the rule
//
// Note that the range matching this rule is [ @start_, @end_ ), i.e. inclusive
// on the left, exclusive on the right.
//
// Refer to XE_RTP_RULES() for expected usage.
//

//
// XE_RTP_RULE_ENGINE_CLASS - Create rule matching an engine class
// @cls_: Engine class to match
//
// Refer to XE_RTP_RULES() for expected usage.
//

//
// XE_RTP_RULE_FUNC - Create rule using callback function for match
// @func__: Function to call to decide if rule matches
//
// This allows more complex checks to be performed. The ``XE_RTP``
// infrastructure will simply call the function @func_ passed to decide if this
// rule matches the device.
//
// Refer to XE_RTP_RULES() for expected usage.
//

//
// XE_RTP_RULE_GRAPHICS_VERSION - Create rule matching graphics version
// @ver__: Graphics IP version to match
//
// Refer to XE_RTP_RULES() for expected usage.
//

//
// XE_RTP_RULE_GRAPHICS_VERSION_RANGE - Create rule matching a range of graphics version
// @ver_start__: First graphics IP version to match
// @ver_end__: Last graphics IP version to match
//
// Note that the range matching this rule is [ @ver_start__, @ver_end__ ], i.e.
// inclusive on both sides
//
// Refer to XE_RTP_RULES() for expected usage.
//

//
// XE_RTP_RULE_GRAPHICS_VERSION_ANY_GT - Create rule matching graphics version on any GT
// @ver__: Graphics IP version to match
//
// Like XE_RTP_RULE_GRAPHICS_VERSION, but it matches even if the current GT
// being checked is not of the graphics type. It allows to add RTP entries to
// another GT when the device contains a Graphics IP with that version.
//
// Refer to XE_RTP_RULES() for expected usage.
//

//
// XE_RTP_RULE_MEDIA_VERSION - Create rule matching media version
// @ver__: Media IP version to match
//
// Refer to XE_RTP_RULES() for expected usage.
//

//
// XE_RTP_RULE_MEDIA_VERSION_RANGE - Create rule matching a range of media version
// @ver_start__: First media IP version to match
// @ver_end__: Last media IP version to match
//
// Note that the range matching this rule is [ @ver_start__, @ver_end__ ], i.e.
// inclusive on both sides
//
// Refer to XE_RTP_RULES() for expected usage.
//

//
// XE_RTP_RULE_MEDIA_VERSION_ANY_GT - Create rule matching media version on any GT
// @ver__: Media IP version to match
//
// Like XE_RTP_RULE_MEDIA_VERSION, but it matches even if the current GT being
// checked is not of the media type. It allows to add RTP entries to another
// GT when the device contains a Media IP with that version.
//
// Refer to XE_RTP_RULES() for expected usage.
//

//
// XE_RTP_RULE_IS_INTEGRATED - Create a rule matching integrated graphics devices
//
// Refer to XE_RTP_RULES() for expected usage.
//

//
// XE_RTP_RULE_IS_DISCRETE - Create a rule matching discrete graphics devices
//
// Refer to XE_RTP_RULES() for expected usage.
//

//
// XE_RTP_RULE_OR - Create an OR condition for rtp rules
//
// RTP rules are AND'ed when evaluated and all of them need to match.
// XE_RTP_RULE_OR allows to create set of rules where any of them matching is
// sufficient for the action to trigger. Example:
//
// .. code-block:: c
//
// const struct xe_rtp_entry_sr entries[] = {
// ...
// { XE_RTP_NAME("test-entry"),
// XE_RTP_RULES(PLATFORM(DG2), OR, PLATFORM(TIGERLAKE)),
// ...
// },
// ...
// };
//

//
// XE_RTP_ACTION_WR - Helper to write a value to the register, overriding all
// the bits
// @reg_: Register
// @val_: Value to set
// @...: Additional fields to override in the struct xe_rtp_action entry
//
// The correspondent notation in bspec is:
//
// REGNAME = VALUE
//

//
// XE_RTP_ACTION_SET - Set bits from @val_ in the register.
// @reg_: Register
// @val_: Bits to set in the register
// @...: Additional fields to override in the struct xe_rtp_action entry
//
// For masked registers this translates to a single write, while for other
// registers it's a RMW. The correspondent bspec notation is (example for bits 2
// and 5, but could be any):
//
// REGNAME[2] = 1
// REGNAME[5] = 1
//

//
// XE_RTP_ACTION_CLR: Clear bits from @val_ in the register.
// @reg_: Register
// @val_: Bits to clear in the register
// @...: Additional fields to override in the struct xe_rtp_action entry
//
// For masked registers this translates to a single write, while for other
// registers it's a RMW. The correspondent bspec notation is (example for bits 2
// and 5, but could be any):
//
// REGNAME[2] = 0
// REGNAME[5] = 0
//

//
// XE_RTP_ACTION_FIELD_SET: Set a bit range
// @reg_: Register
// @mask_bits_: Mask of bits to be changed in the register, forming a field
// @val_: Value to set in the field denoted by @mask_bits_
// @...: Additional fields to override in the struct xe_rtp_action entry
//
// For masked registers this translates to a single write, while for other
// registers it's a RMW. The correspondent bspec notation is:
//
// REGNAME[<end>:<start>] = VALUE
//

//
// XE_RTP_ACTION_FIELD_SET_FUNC: Set a bit range to the value returned by a function
// @reg_: Register
// @mask_bits_: Mask of bits to be changed in the register, forming a field
// @func_: Function that returns value to set in the field denoted by @mask_bits_
// @...: Additional fields to override in the struct xe_rtp_action entry
//
// This macro works like XE_RTP_ACTION_FIELD_SET(), except that the
// field value is evaluated at the time the RTP table is processed.
//
// @func_ will only be called a single time, when the RTP table is being
// processed.  After processing, the value in the reg_sr entry is fixed and
// will not be re-evaluated.
//

//
// XE_RTP_ACTION_WHITELIST - Add register to userspace whitelist
// @reg_: Register
// @val_: Whitelist-specific flags to set
// @...: Additional fields to override in the struct xe_rtp_action entry
//
// Add a register to the whitelist, allowing userspace to modify the ster with
// regular user privileges.
//

// TODO fail build if ((flags) & ~(RING_FORCE_TO_NONPRIV_MASK_VALID)) */\

//
// XE_RTP_NAME - Helper to set the name in xe_rtp_entry
// @s_: Name describing this rule, often a HW-specific number
//
// TODO: maybe move this behind a debug config?
//

//
// XE_RTP_ENTRY_FLAG - Helper to add multiple flags to a struct xe_rtp_entry_sr
// @...: Entry flags, without the ``XE_RTP_ENTRY_FLAG_`` prefix
//
// Helper to automatically add a ``XE_RTP_ENTRY_FLAG_`` prefix to the flags
// when defining struct xe_rtp_entry entries. Example:
//
// .. code-block:: c
//
// const struct xe_rtp_entry_sr wa_entries[] = {
// ...
// { XE_RTP_NAME("test-entry"),
// ...
// XE_RTP_ENTRY_FLAG(FOREACH_ENGINE),
// ...
// },
// ...
// };
//

//
// XE_RTP_ACTION_FLAG - Helper to add multiple flags to a struct xe_rtp_action
// @...: Action flags, without the ``XE_RTP_ACTION_FLAG_`` prefix
//
// Helper to automatically add a ``XE_RTP_ACTION_FLAG_`` prefix to the flags
// when defining struct xe_rtp_action entries. Example:
//
// .. code-block:: c
//
// const struct xe_rtp_entry_sr wa_entries[] = {
// ...
// { XE_RTP_NAME("test-entry"),
// ...
// XE_RTP_ACTION_SET(..., XE_RTP_ACTION_FLAG(FOREACH_ENGINE)),
// ...
// },
// ...
// };
//

//
// XE_RTP_RULES - Helper to set multiple rules to a struct xe_rtp_entry_sr entry
// @...: Rules
//
// When an RTP table is being processed, the rules of each entry are evaluated
// to check if they match the target entity (platform, gt or hwe, depending on
// the specific RTP table).
//
// The sequence of arguments of this macro must follow the following eBNF
// grammar::
//
// rules = disjunction;
// disjunction = conjunction, { "OR", conjunction };
// conjunction = single_rule, { single_rule };
// (* the AND operator is implicit *)
// single_rule = ? GRAPHICS_VERSION(...), MEDIA_VERSION(...),
// FUNC(...), etc ?
//
// Examples:
//
// .. code-block:: c
//
// const struct xe_rtp_entry_sr wa_entries[] = {
// ...
// { XE_RTP_NAME("entry-a"),
// // Match DG2-G10 with graphics steppings A0 up-to B0
// // (exclusive).
// XE_RTP_RULES(SUBPLATFORM(DG2, G10), GRAPHICS_STEP(A0, B0)),
// ...
// },
// { XE_RTP_NAME("entry-b"),
// // Match graphics version 20 (all steppings) or graphics
// // version 30 steppings A0 up-to B0 (exclusive).
// XE_RTP_RULES(GRAPHICS_VERSION(2000), OR,
// GRAPHICS_VERSION(3000), GRAPHICS_STEP(A0, B0))
// ...
// },
// ...
// };
//

//
// XE_RTP_ACTIONS - Helper to set multiple actions to a struct xe_rtp_entry_sr
// @...: Actions to be taken
//
// At least one action is needed and up to 12 are supported. See XE_RTP_ACTION_
// for the possible actions. Example:
//
// .. code-block:: c
//
// const struct xe_rtp_entry_sr wa_entries[] = {
// ...
// { XE_RTP_NAME("test-entry"),
// XE_RTP_RULES(...),
// XE_RTP_ACTIONS(SET(..), SET(...), CLR(...)),
// ...
// },
// ...
// };
//

//
// Note: ARRAY_SIZE() cannot be used here because it expands through
// __must_be_array() -> __BUILD_BUG_ON_ZERO_MSG() -> _Static_assert inside
// sizeof(struct{}), which clang < 21 rejects when the compound literal
// contains non-compile-time-constant initializers.
//

// Match functions to be used with XE_RTP_MATCH_FUNC
//
// xe_rtp_match_always - Match RTP entry unconditionally
// @xe: Device structure
// @gt: GT structure
// @hwe: Engine instance
//
// Returns: true, regardless of inputs
//
// xe_rtp_match_even_instance - Match if engine instance is even
// @xe: Device structure
// @gt: GT structure
// @hwe: Engine instance
//
// Returns: true if engine instance is even, false otherwise
//
// xe_rtp_match_first_render_or_compute - Match if it's first render or compute
// engine in the GT
//
// @xe: Device structure
// @gt: GT structure
// @hwe: Engine instance
//
// Registers on the render reset domain need to have their values re-applied
// when any of those engines are reset. Since the engines reset together, a
// programming can be set to just one of them. For simplicity the first engine
// of either render or compute class can be chosen.
//
// Returns: true if engine id is the first to match the render reset domain,
// false otherwise.
//
// xe_rtp_match_not_sriov_vf - Match when not on SR-IOV VF device
//
// @xe: Device structure
// @gt: GT structure
// @hwe: Engine instance
//
// Returns: true if device is not VF, false otherwise.
//
// xe_rtp_match_has_flat_ccs - Match when platform has FlatCCS compression
// @xe: Device structure
// @gt: GT structure
// @hwe: Engine instance
//
// Returns: true if platform has FlatCCS compression, false otherwise
//
// xe_rtp_match_has_msix - Match when platform has MSI-X
// @xe: Device structure
// @gt: GT structure
// @hwe: Engine instance
//
// Returns: true if platform has MSI-X interrupt support
//
