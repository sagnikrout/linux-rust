//! Automatically rewritten from C to Rust
//! Source: drivers/bus/fsl-mc/dpbp.c
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


// SPDX-License-Identifier: (GPL-2.0+ OR BSD-3-Clause)
//
// Copyright 2013-2016 Freescale Semiconductor Inc.
//

//
// dpbp_open() - Open a control session for the specified object.
// @mc_io:	Pointer to MC portal's I/O object
// @cmd_flags:	Command flags; one or more of 'MC_CMD_FLAG_'
// @dpbp_id:	DPBP unique ID
// @token:	Returned token; use in subsequent API calls
//
// This function can be used to open a control session for an
// already created object; an object may have been declared in
// the DPL or by calling the dpbp_create function.
// This function returns a unique authentication token,
// associated with the specific object ID and the specific MC
// portal; this token must be used in all subsequent commands for
// this specific object
//
// Return:	'0' on Success; Error code otherwise.
//
    int dpbp_open(struct fsl_mc_io *mc_io,
    u32 cmd_flags,
    int dpbp_id,
    u16 *token)
    {
    let mut cmd: fsl_mc_command = { 0 };
    struct dpbp_cmd_open *cmd_params;
    int err;
// prepare command
    cmd.header = mc_encode_cmd_header(DPBP_CMDID_OPEN,
    cmd_flags, 0);
    cmd_params = (struct dpbp_cmd_open *)cmd.params;
    cmd_params.dpbp_id = cpu_to_le32(dpbp_id);
// send command to mc
    err = mc_send_command(mc_io, &cmd);
    if (err)
    return err;
// retrieve response parameters
// token = mc_cmd_hdr_read_token(&cmd);
    return err;
    }
    EXPORT_SYMBOL_GPL(dpbp_open);
//
// dpbp_close() - Close the control session of the object
// @mc_io:	Pointer to MC portal's I/O object
// @cmd_flags:	Command flags; one or more of 'MC_CMD_FLAG_'
// @token:	Token of DPBP object
//
// After this function is called, no further operations are
// allowed on the object without opening a new control session.
//
// Return:	'0' on Success; Error code otherwise.
//
    int dpbp_close(struct fsl_mc_io *mc_io,
    u32 cmd_flags,
    u16 token)
    {
    let mut cmd: fsl_mc_command = { 0 };
// prepare command
    cmd.header = mc_encode_cmd_header(DPBP_CMDID_CLOSE, cmd_flags,
    token);
// send command to mc
    return mc_send_command(mc_io, &cmd);
    }
    EXPORT_SYMBOL_GPL(dpbp_close);
//
// dpbp_enable() - Enable the DPBP.
// @mc_io:	Pointer to MC portal's I/O object
// @cmd_flags:	Command flags; one or more of 'MC_CMD_FLAG_'
// @token:	Token of DPBP object
//
// Return:	'0' on Success; Error code otherwise.
//
    int dpbp_enable(struct fsl_mc_io *mc_io,
    u32 cmd_flags,
    u16 token)
    {
    let mut cmd: fsl_mc_command = { 0 };
// prepare command
    cmd.header = mc_encode_cmd_header(DPBP_CMDID_ENABLE, cmd_flags,
    token);
// send command to mc
    return mc_send_command(mc_io, &cmd);
    }
    EXPORT_SYMBOL_GPL(dpbp_enable);
//
// dpbp_disable() - Disable the DPBP.
// @mc_io:	Pointer to MC portal's I/O object
// @cmd_flags:	Command flags; one or more of 'MC_CMD_FLAG_'
// @token:	Token of DPBP object
//
// Return:	'0' on Success; Error code otherwise.
//
    int dpbp_disable(struct fsl_mc_io *mc_io,
    u32 cmd_flags,
    u16 token)
    {
    let mut cmd: fsl_mc_command = { 0 };
// prepare command
    cmd.header = mc_encode_cmd_header(DPBP_CMDID_DISABLE,
    cmd_flags, token);
// send command to mc
    return mc_send_command(mc_io, &cmd);
    }
    EXPORT_SYMBOL_GPL(dpbp_disable);
//
// dpbp_reset() - Reset the DPBP, returns the object to initial state.
// @mc_io:	Pointer to MC portal's I/O object
// @cmd_flags:	Command flags; one or more of 'MC_CMD_FLAG_'
// @token:	Token of DPBP object
//
// Return:	'0' on Success; Error code otherwise.
//
    int dpbp_reset(struct fsl_mc_io *mc_io,
    u32 cmd_flags,
    u16 token)
    {
    let mut cmd: fsl_mc_command = { 0 };
// prepare command
    cmd.header = mc_encode_cmd_header(DPBP_CMDID_RESET,
    cmd_flags, token);
// send command to mc
    return mc_send_command(mc_io, &cmd);
    }
    EXPORT_SYMBOL_GPL(dpbp_reset);
//
// dpbp_get_attributes - Retrieve DPBP attributes.
//
// @mc_io:	Pointer to MC portal's I/O object
// @cmd_flags:	Command flags; one or more of 'MC_CMD_FLAG_'
// @token:	Token of DPBP object
// @attr:	Returned object's attributes
//
// Return:	'0' on Success; Error code otherwise.
//
    int dpbp_get_attributes(struct fsl_mc_io *mc_io,
    u32 cmd_flags,
    u16 token,
    struct dpbp_attr *attr)
    {
    let mut cmd: fsl_mc_command = { 0 };
    struct dpbp_rsp_get_attributes *rsp_params;
    int err;
// prepare command
    cmd.header = mc_encode_cmd_header(DPBP_CMDID_GET_ATTR,
    cmd_flags, token);
// send command to mc
    err = mc_send_command(mc_io, &cmd);
    if (err)
    return err;
// retrieve response parameters
    rsp_params = (struct dpbp_rsp_get_attributes *)cmd.params;
    attr.bpid = le16_to_cpu(rsp_params.bpid);
    attr.id = le32_to_cpu(rsp_params.id);
    return 0;
    }
    EXPORT_SYMBOL_GPL(dpbp_get_attributes);
