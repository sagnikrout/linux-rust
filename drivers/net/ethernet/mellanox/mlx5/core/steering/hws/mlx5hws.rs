//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlx5/core/steering/hws/mlx5hws.h
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


// SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB
// Copyright (c) 2024 NVIDIA Corporation & Affiliates
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5hws_table_type {
    MLX5HWS_TABLE_TYPE_FDB,
    MLX5HWS_TABLE_TYPE_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5hws_matcher_resource_mode {
// Allocate resources based on number of rules with minimal failure probability
    MLX5HWS_MATCHER_RESOURCE_MODE_RULE,
// Allocate fixed size hash table based on given column and rows
    MLX5HWS_MATCHER_RESOURCE_MODE_HTABLE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5hws_action_type {
    MLX5HWS_ACTION_TYP_LAST,
    MLX5HWS_ACTION_TYP_REFORMAT_TNL_L2_TO_L2,
    MLX5HWS_ACTION_TYP_REFORMAT_L2_TO_TNL_L2,
    MLX5HWS_ACTION_TYP_REFORMAT_TNL_L3_TO_L2,
    MLX5HWS_ACTION_TYP_REFORMAT_L2_TO_TNL_L3,
    MLX5HWS_ACTION_TYP_DROP,
    MLX5HWS_ACTION_TYP_MISS,
    MLX5HWS_ACTION_TYP_TBL,
    MLX5HWS_ACTION_TYP_CTR,
    MLX5HWS_ACTION_TYP_TAG,
    MLX5HWS_ACTION_TYP_MODIFY_HDR,
    MLX5HWS_ACTION_TYP_VPORT,
    MLX5HWS_ACTION_TYP_POP_VLAN,
    MLX5HWS_ACTION_TYP_PUSH_VLAN,
    MLX5HWS_ACTION_TYP_ASO_METER,
    MLX5HWS_ACTION_TYP_INSERT_HEADER,
    MLX5HWS_ACTION_TYP_REMOVE_HEADER,
    MLX5HWS_ACTION_TYP_RANGE,
    MLX5HWS_ACTION_TYP_SAMPLER,
    MLX5HWS_ACTION_TYP_DEST_ARRAY,
    MLX5HWS_ACTION_TYP_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5hws_action_flags {
    MLX5HWS_ACTION_FLAG_HWS_FDB = 1 << 0,
// Shared action can be used over a few threads, since the
// data is written only once at the creation of the action.
//
    MLX5HWS_ACTION_FLAG_SHARED = 1 << 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5hws_action_aso_meter_color {
    MLX5HWS_ACTION_ASO_METER_COLOR_RED = 0x0,
    MLX5HWS_ACTION_ASO_METER_COLOR_YELLOW = 0x1,
    MLX5HWS_ACTION_ASO_METER_COLOR_GREEN = 0x2,
    MLX5HWS_ACTION_ASO_METER_COLOR_UNDEFINED = 0x3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5hws_send_queue_actions {
// Start executing all pending queued rules
    MLX5HWS_SEND_QUEUE_ACTION_DRAIN_ASYNC = 1 << 0,
// Start executing all pending queued rules wait till completion
    MLX5HWS_SEND_QUEUE_ACTION_DRAIN_SYNC = 1 << 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5hws_context_attr {
    pub queues: u16,
    pub queue_size: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5hws_table_attr {
    pub type: mlx5hws_table_type,
    pub level: u32,
    pub uid: u16,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5hws_matcher_flow_src {
    MLX5HWS_MATCHER_FLOW_SRC_ANY = 0x0,
    MLX5HWS_MATCHER_FLOW_SRC_WIRE = 0x1,
    MLX5HWS_MATCHER_FLOW_SRC_VPORT = 0x2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5hws_matcher_insert_mode {
    MLX5HWS_MATCHER_INSERT_BY_HASH = 0x0,
    MLX5HWS_MATCHER_INSERT_BY_INDEX = 0x1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5hws_matcher_distribute_mode {
    MLX5HWS_MATCHER_DISTRIBUTE_BY_HASH = 0x0,
    MLX5HWS_MATCHER_DISTRIBUTE_BY_LINEAR = 0x1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5hws_matcher_size_type {
    MLX5HWS_MATCHER_SIZE_TYPE_RX,
    MLX5HWS_MATCHER_SIZE_TYPE_TX,
    MLX5HWS_MATCHER_SIZE_TYPE_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union mlx5hws_matcher_size {
    pub sz_row_log: u8,
    pub sz_col_log: u8,
    pub table: },
    pub num_log: u8,
    pub rule: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5hws_matcher_attr {
// Processing priority inside table
    pub priority: u32,
// Provide all rules with unique rule_idx in num_log range to reduce locking
    pub optimize_using_rule_idx: bool,
// Resource mode and corresponding size
    pub mode: mlx5hws_matcher_resource_mode,
// Optimize insertion in case packet origin is the same for all rules
    pub optimize_flow_src: mlx5hws_matcher_flow_src,
// Define the insertion and distribution modes for this matcher
    pub insert_mode: mlx5hws_matcher_insert_mode,
    pub distribute_mode: mlx5hws_matcher_distribute_mode,
// Define whether the created matcher supports resizing into a bigger matcher
    pub resizable: bool,
    pub size: [mlx5hws_matcher_size; MLX5HWS_MATCHER_SIZE_TYPE_MAX],
// Optional AT attach configuration - Max number of additional AT
    pub max_num_of_at_attach: u8,
// Optional end FT (miss FT ID) for match RTC (for isolated matcher)
    pub isolated_matcher_end_ft_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5hws_rule_attr {
    pub user_data: *mut c_void,
// Valid if matcher optimize_using_rule_idx is set or
// if matcher is configured to insert rules by index.
//
    pub rule_idx: u32,
    pub flow_source: u32,
    pub queue_id: u16,
    pub burst:1: u32,
}

// In actions that take offset, the offset is unique, pointing to a single
// resource and the user should not reuse the same index because data changing
// is not atomic.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5hws_rule_action {
    pub action: *mut mlx5hws_action,
    pub value: u32,
    pub tag: },
    pub offset: u32,
    pub counter: },
    pub offset: u32,
    pub data: *mut u8,
    pub modify_header: },
    pub offset: u32,
    pub hdr_idx: u8,
    pub data: *mut u8,
    pub reformat: },
    pub vlan_hdr: __be32,
    pub push_vlan: },
    pub offset: u32,
    pub init_color: mlx5hws_action_aso_meter_color,
    pub aso_meter: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5hws_action_reformat_header {
    pub sz: usize,
    pub data: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5hws_action_insert_header {
    pub hdr: mlx5hws_action_reformat_header,
// PRM start anchor to which header will be inserted
    pub anchor: u8,
// Header insertion offset in bytes, from the start
// anchor to the location where new header will be inserted.
//
    pub offset: u8,
// Indicates this header insertion adds encapsulation header to the packet,
// requiring device to update offloaded fields (for example IPv4 total length).
//
    pub encap: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5hws_action_remove_header_attr {
// PRM start anchor from which header will be removed
    pub anchor: u8,
// Header remove offset in bytes, from the start
// anchor to the location where remove header starts.
//
    pub offset: u8,
// Indicates the removed header size in bytes
    pub size: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5hws_action_mh_pattern {
// Byte size of modify actions provided by "data"
    pub sz: usize,
// PRM format modify actions pattern
    pub data: *mut __be64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5hws_action_dest_attr {
// Required destination action to forward the packet
    pub dest: *mut mlx5hws_action,
// Optional reformat action
    pub reformat: *mut mlx5hws_action,
    pub is_wire_ft: bool,
}

//
// mlx5hws_is_supported - Check whether HWS is supported
//
// @mdev: The device to check.
//
// Return: true if supported, false otherwise.
//
// mlx5hws_context_open - Open a context used for direct rule insertion
// using hardware steering.
//
// @mdev: The device to be used for HWS.
// @attr: Attributes used for context open.
//
// Return: pointer to mlx5hws_context on success NULL otherwise.
//
// mlx5hws_context_close - Close a context used for direct hardware steering.
//
// @ctx: mlx5hws context to close.
//
// Return: zero on success non zero otherwise.
//
extern "C" {
    pub fn mlx5hws_context_close(ctx: *mut mlx5hws_context) -> c_int;
}
//
// mlx5hws_context_set_peer - Set a peer context.
// Each context can have multiple contexts as peers.
//
// @ctx: The context in which the peer_ctx will be peered to it.
// @peer_ctx: The peer context.
// @peer_vhca_id: The peer context vhca id.
//
// mlx5hws_table_create - Create a new direct rule table.
// Each table can contain multiple matchers.
//
// @ctx: The context in which the new table will be opened.
// @attr: Attributes used for table creation.
//
// Return: pointer to mlx5hws_table on success NULL otherwise.
//
// mlx5hws_table_destroy - Destroy direct rule table.
//
// @tbl: Table to destroy.
//
// Return: zero on success non zero otherwise.
//
extern "C" {
    pub fn mlx5hws_table_destroy(tbl: *mut mlx5hws_table) -> c_int;
}
//
// mlx5hws_table_get_id() - Get ID of the flow table.
//
// @tbl:Table to get ID of.
//
// Return: ID of the table.
//
extern "C" {
    pub fn mlx5hws_table_get_id(tbl: *mut mlx5hws_table) -> u32;
}
//
// mlx5hws_table_set_default_miss - Set default miss table for mlx5hws_table
// by using another mlx5hws_table.
// Traffic which all table matchers miss will be forwarded to miss table.
//
// @tbl: Source table
// @miss_tbl: Target (miss) table, or NULL to remove current miss table
//
// Return: zero on success non zero otherwise.
//
// mlx5hws_match_template_create - Create a new match template based on items mask.
// The match template will be used for matcher creation.
//
// @ctx: The context in which the new template will be created.
// @match_param: Describe the mask based on PRM match parameters.
// @match_param_sz: Size of match param buffer.
// @match_criteria_enable: Bitmap for each sub-set in match_criteria buffer.
//
// Return: Pointer to mlx5hws_match_template on success, NULL otherwise.
//
// mlx5hws_match_template_destroy - Destroy a match template.
//
// @mt: Match template to destroy.
//
// Return: Zero on success, non-zero otherwise.
//
extern "C" {
    pub fn mlx5hws_match_template_destroy(mt: *mut mlx5hws_match_template) -> c_int;
}
//
// mlx5hws_action_template_create - Create a new action template based on an action_type array.
//
// @action_type: An array of actions based on the order of actions which will be provided
// with rule_actions to mlx5hws_rule_create. The last action is marked
// using MLX5HWS_ACTION_TYP_LAST.
//
// Return: Pointer to mlx5hws_action_template on success, NULL otherwise.
//
// mlx5hws_action_template_destroy - Destroy action template.
//
// @at: Action template to destroy.
//
// Return: zero on success non zero otherwise.
//
extern "C" {
    pub fn mlx5hws_action_template_destroy(at: *mut mlx5hws_action_template) -> c_int;
}
//
// mlx5hws_matcher_create - Create a new direct rule matcher.
//
// Each matcher can contain multiple rules. Matchers on the table will be
// processed by priority. Matching fields and mask are described by the
// match template. In some cases, multiple match templates can be used on
// the same matcher.
//
// @table: The table in which the new matcher will be opened.
// @mt: Array of match templates to be used on matcher.
// @num_of_mt: Number of match templates in mt array.
// @at: Array of action templates to be used on matcher.
// @num_of_at: Number of action templates in at array.
// @attr: Attributes used for matcher creation.
//
// Return: Pointer to mlx5hws_matcher on success, NULL otherwise.
//
// mlx5hws_matcher_destroy - Destroy a direct rule matcher.
//
// @matcher: Matcher to destroy.
//
// Return: Zero on success, non-zero otherwise.
//
extern "C" {
    pub fn mlx5hws_matcher_destroy(matcher: *mut mlx5hws_matcher) -> c_int;
}
//
// mlx5hws_matcher_attach_at - Attach a new action template to a direct rule matcher.
//
// @matcher: Matcher to attach the action template to.
// @at: Action template to be attached to the matcher.
//
// Return: Zero on success, non-zero otherwise.
//
// mlx5hws_matcher_resize_set_target - Link two matchers and enable moving rules.
//
// Both matchers must be in the same table type, must be created with the
// 'resizable' property, and should have the same characteristics (e.g., same
// match templates and action templates). It is the user's responsibility to
// ensure that the destination matcher is allocated with the appropriate size.
//
// Once the function is completed, the user is:
// - Allowed to move rules from the source into the destination matcher.
// - No longer allowed to insert rules into the source matcher.
//
// The user is always allowed to insert rules into the destination matcher and
// to delete rules from any matcher.
//
// @src_matcher: Source matcher for moving rules from.
// @dst_matcher: Destination matcher for moving rules to.
//
// Return: Zero on successful move, non-zero otherwise.
//
// mlx5hws_matcher_resize_rule_move - Enqueue moving rule operation.
//
// This function enqueues the operation of moving a rule from the source
// matcher to the destination matcher.
//
// @src_matcher: Matcher that the rule belongs to.
// @rule: The rule to move.
// @attr: Rule attributes.
//
// Return: Zero on success, non-zero otherwise.
//
// mlx5hws_rule_create - Enqueue create rule operation.
//
// @matcher: The matcher in which the new rule will be created.
// @mt_idx: Match template index to create the match with.
// @match_param: The match parameter PRM buffer used for value matching.
// @at_idx: Action template index to apply the actions with.
// @rule_actions: Rule actions to be executed on match.
// @attr: Rule creation attributes.
// @rule_handle: A valid rule handle. The handle doesn't require any initialization.
//
// Return: Zero on successful enqueue, non-zero otherwise.
//
// mlx5hws_rule_destroy - Enqueue destroy rule operation.
//
// @rule: The rule destruction to enqueue.
// @attr: Rule destruction attributes.
//
// Return: Zero on successful enqueue, non-zero otherwise.
//
// mlx5hws_rule_action_update - Enqueue update actions on an existing rule.
//
// @rule: A valid rule handle to update.
// @at_idx: Action template index to update the actions with.
// @rule_actions: Rule actions to be executed on match.
// @attr: Rule update attributes.
//
// Return: Zero on successful enqueue, non-zero otherwise.
//
// mlx5hws_action_get_type - Get action type.
//
// @action: The action to get the type of.
//
// Return: action type.
//
// mlx5hws_action_get_dev - Get mlx5 core device.
//
// @action: The action to get the device from.
//
// Return: mlx5 core device.
//
// mlx5hws_action_create_dest_drop - Create a direct rule drop action.
//
// @ctx: The context in which the new action will be created.
// @flags: Action creation flags (enum mlx5hws_action_flags).
//
// Return: Pointer to mlx5hws_action on success, NULL otherwise.
//
// mlx5hws_action_create_default_miss - Create a direct rule default miss action.
// Defaults are RX: Drop, TX: Wire.
//
// @ctx: The context in which the new action will be created.
// @flags: Action creation flags (enum mlx5hws_action_flags).
//
// Return: Pointer to mlx5hws_action on success, NULL otherwise.
//
// mlx5hws_action_create_dest_table - Create direct rule goto table action.
//
// @ctx: The context in which the new action will be created.
// @tbl: Destination table.
// @flags: Action creation flags (enum mlx5hws_action_flags).
//
// Return: pointer to mlx5hws_action on success NULL otherwise.
//
// mlx5hws_action_create_dest_table_num - Create direct rule goto table number action.
//
// @ctx: The context in which the new action will be created.
// @tbl_num: Destination table number.
// @flags: Action creation flags (enum mlx5hws_action_flags).
//
// Return: pointer to mlx5hws_action on success NULL otherwise.
//
// mlx5hws_action_create_dest_match_range - Create direct rule range match action.
//
// @ctx: The context in which the new action will be created.
// @field: Field to comapare the value.
// @hit_ft: Flow table to go to on hit.
// @miss_ft: Flow table to go to on miss.
// @min: Minimal value of the field to be considered as hit.
// @max: Maximal value of the field to be considered as hit.
// @flags: Action creation flags (enum mlx5hws_action_flags).
//
// Return: pointer to mlx5hws_action on success NULL otherwise.
//
// mlx5hws_action_create_flow_sampler - Create direct rule flow sampler action.
//
// @ctx: The context in which the new action will be created.
// @sampler_id: Flow sampler object ID.
// @flags: Action creation flags (enum mlx5hws_action_flags).
//
// Return: pointer to mlx5hws_action on success NULL otherwise.
//
// mlx5hws_action_create_dest_vport - Create direct rule goto vport action.
//
// @ctx: The context in which the new action will be created.
// @vport_num: Destination vport number.
// @vhca_id_valid: Tells if the vhca_id parameter is valid.
// @vhca_id: VHCA ID of the destination vport.
// @flags: Action creation flags (enum mlx5hws_action_flags).
//
// Return: pointer to mlx5hws_action on success NULL otherwise.
//
// mlx5hws_action_create_tag - Create direct rule TAG action.
//
// @ctx: The context in which the new action will be created.
// @flags: Action creation flags (enum mlx5hws_action_flags).
//
// Return: pointer to mlx5hws_action on success NULL otherwise.
//
// mlx5hws_action_create_counter - Create direct rule counter action.
//
// @ctx: The context in which the new action will be created.
// @obj_id: Direct rule counter object ID.
// @flags: Action creation flags (enum mlx5hws_action_flags).
//
// Return: pointer to mlx5hws_action on success NULL otherwise.
//
// mlx5hws_action_create_reformat - Create direct rule reformat action.
//
// @ctx: The context in which the new action will be created.
// @reformat_type: Type of reformat prefixed with MLX5HWS_ACTION_TYP_REFORMAT.
// @num_of_hdrs: Number of provided headers in "hdrs" array.
// @hdrs: Headers array containing header information.
// @log_bulk_size: Number of unique values used with this reformat.
// @flags: Action creation flags (enum mlx5hws_action_flags).
//
// Return: pointer to mlx5hws_action on success NULL otherwise.
//
// mlx5hws_action_create_modify_header - Create direct rule modify header action.
//
// @ctx: The context in which the new action will be created.
// @num_of_patterns: Number of provided patterns in "patterns" array.
// @patterns: Patterns array containing pattern information.
// @log_bulk_size: Number of unique values used with this pattern.
// @flags: Action creation flags (enum mlx5hws_action_flags).
//
// Return: pointer to mlx5hws_action on success NULL otherwise.
//
// mlx5hws_action_create_aso_meter - Create direct rule ASO flow meter action.
//
// @ctx: The context in which the new action will be created.
// @obj_id: ASO object ID.
// @return_reg_c: Copy the ASO object value into this reg_c,
// after a packet hits a rule with this ASO object.
// @flags: Action creation flags (enum mlx5hws_action_flags).
//
// Return: pointer to mlx5hws_action on success NULL otherwise.
//
// mlx5hws_action_create_pop_vlan - Create direct rule pop vlan action.
//
// @ctx: The context in which the new action will be created.
// @flags: Action creation flags (enum mlx5hws_action_flags).
//
// Return: pointer to mlx5hws_action on success NULL otherwise.
//
// mlx5hws_action_create_push_vlan - Create direct rule push vlan action.
//
// @ctx: The context in which the new action will be created.
// @flags: Action creation flags (enum mlx5hws_action_flags).
//
// Return: pointer to mlx5hws_action on success NULL otherwise.
//
// mlx5hws_action_create_dest_array - Create a dest array action, this action can
// duplicate packets and forward to multiple destinations in the destination list.
//
// @ctx: The context in which the new action will be created.
// @num_dest: The number of dests attributes.
// @dests: The destination array. Each contains a destination action and can
// have additional actions.
// @flags: Action creation flags (enum mlx5hws_action_flags).
//
// Return: pointer to mlx5hws_action on success NULL otherwise.
//
// mlx5hws_action_create_insert_header - Create insert header action.
//
// @ctx: The context in which the new action will be created.
// @num_of_hdrs: Number of provided headers in "hdrs" array.
// @hdrs: Headers array containing header information.
// @log_bulk_size: Number of unique values used with this insert header.
// @flags: Action creation flags. (enum mlx5hws_action_flags)
//
// Return: pointer to mlx5hws_action on success NULL otherwise.
//
// mlx5hws_action_create_remove_header - Create remove header action.
//
// @ctx: The context in which the new action will be created.
// @attr: attributes that specifie the remove header type, PRM start anchor and
// the PRM end anchor or the PRM start anchor and remove size in bytes.
// @flags: Action creation flags. (enum mlx5hws_action_flags)
//
// Return: pointer to mlx5hws_action on success NULL otherwise.
//
// mlx5hws_action_create_last - Create direct rule LAST action.
//
// @ctx: The context in which the new action will be created.
// @flags: Action creation flags. (enum mlx5hws_action_flags)
//
// Return: pointer to mlx5hws_action on success NULL otherwise.
//
// mlx5hws_action_destroy - Destroy direct rule action.
//
// @action: The action to destroy.
//
// Return: zero on success non zero otherwise.
//
extern "C" {
    pub fn mlx5hws_action_destroy(action: *mut mlx5hws_action) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5hws_flow_op_status {
    MLX5HWS_FLOW_OP_SUCCESS,
    MLX5HWS_FLOW_OP_ERROR,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5hws_flow_op_result {
    pub status: mlx5hws_flow_op_status,
    pub user_data: *mut c_void,
}

//
// mlx5hws_send_queue_poll - Poll queue for rule creation and deletions completions.
//
// @ctx: The context to which the queue belong to.
// @queue_id: The id of the queue to poll.
// @res: Completion array.
// @res_nb: Maximum number of results to return.
//
// Return: negative number on failure, the number of completions otherwise.
//
// mlx5hws_send_queue_action - Perform an action on the queue
//
// @ctx: The context to which the queue belong to.
// @queue_id: The id of the queue to perform the action on.
// @actions: Actions to perform on the queue (enum mlx5hws_send_queue_actions)
//
// Return: zero on success non zero otherwise.
//
// mlx5hws_debug_dump - Dump HWS info
//
// @ctx: The context which to dump the info from.
//
// Return: zero on success non zero otherwise.
//
extern "C" {
    pub fn mlx5hws_debug_dump(ctx: *mut mlx5hws_context) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5hws_match_parameters {
    pub match_sz: usize,
    pub /: *mut *mut *mut u32 match_buf; / Device spec format,
}

//
// mlx5hws_bwc_matcher_create - Create a new BWC direct rule matcher.
//
// This function does the following:
// - creates match template based on flow items
// - creates an empty action template
// - creates a usual mlx5hws_matcher with these mt and at, setting
// its size to minimal
// Notes:
// - table->ctx must have BWC support
// - complex rules are not supported
//
// @table: The table in which the new matcher will be opened
// @priority: Priority for this BWC matcher
// @match_criteria_enable: Bitmask that defines matching criteria
// @mask: Match parameters
//
// Return: pointer to mlx5hws_bwc_matcher on success or NULL otherwise.
//
// mlx5hws_bwc_matcher_destroy - Destroy BWC direct rule matcher.
//
// @bwc_matcher: Matcher to destroy
//
// Return: zero on success, non zero otherwise
//
extern "C" {
    pub fn mlx5hws_bwc_matcher_destroy(bwc_matcher: *mut mlx5hws_bwc_matcher) -> c_int;
}
//
// mlx5hws_bwc_rule_create - Create a new BWC rule.
//
// Unlike the usual rule creation function, this one is blocking: when the
// function returns, the rule is written to its place (no need to poll).
// This function does the following:
// - finds matching action template based on the provided rule_actions, or
// creates new action template if matching action template doesn't exist
// - updates corresponding BWC matcher stats
// - if needed, the function performs rehash:
// - creates a new matcher based on mt, at, new_sz
// - moves all the existing matcher rules to the new matcher
// - removes the old matcher
// - inserts new rule
// - polls till completion is received
// Notes:
// - matcher->tbl->ctx must have BWC support
// - separate BWC ctx queues are used
//
// @bwc_matcher: The BWC matcher in which the new rule will be created.
// @params: Match perameters
// @flow_source: Flow source for this rule
// @rule_actions: Rule action to be executed on match
//
// Return: valid BWC rule handle on success, NULL otherwise
//
// mlx5hws_bwc_rule_destroy - Destroy BWC direct rule.
//
// @bwc_rule: Rule to destroy.
//
// Return: zero on success, non zero otherwise.
//
extern "C" {
    pub fn mlx5hws_bwc_rule_destroy(bwc_rule: *mut mlx5hws_bwc_rule) -> c_int;
}
//
// mlx5hws_bwc_rule_action_update - Update actions on an existing BWC rule.
//
// @bwc_rule: Rule to update
// @rule_actions: Rule action to update with
//
// Return: zero on successful update, non zero otherwise.
//
