//! Automatically rewritten from C to Rust
//! Source: drivers/crypto/marvell/octeontx/otx_cptvf_mbox.c
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
// Marvell OcteonTX CPT driver
//
// Copyright (C) 2019 Marvell International Ltd.
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License version 2 as
// published by the Free Software Foundation.
//

pub const CPT_MBOX_MSG_TIMEOUT: c_int = 2000;
    static char *get_mbox_opcode_str(int msg_opcode)
    {
    char *str = "Unknown";
    switch (msg_opcode) {
    case OTX_CPT_MSG_VF_UP:
    str = "UP";
    break;
    case OTX_CPT_MSG_VF_DOWN:
    str = "DOWN";
    break;
    case OTX_CPT_MSG_READY:
    str = "READY";
    break;
    case OTX_CPT_MSG_QLEN:
    str = "QLEN";
    break;
    case OTX_CPT_MSG_QBIND_GRP:
    str = "QBIND_GRP";
    break;
    case OTX_CPT_MSG_VQ_PRIORITY:
    str = "VQ_PRIORITY";
    break;
    case OTX_CPT_MSG_PF_TYPE:
    str = "PF_TYPE";
    break;
    case OTX_CPT_MSG_ACK:
    str = "ACK";
    break;
    case OTX_CPT_MSG_NACK:
    str = "NACK";
    break;
    }
    return str;
    }
#[no_mangle]
unsafe extern "C" fn dump_mbox_msg(mbox_msg: *mut otx_cpt_mbox, vf_id: c_int) {
    static void dump_mbox_msg(struct otx_cpt_mbox *mbox_msg, int vf_id)
    {
    char raw_data_str[OTX_CPT_MAX_MBOX_DATA_STR_SIZE];
    hex_dump_to_buffer(mbox_msg, sizeof(struct otx_cpt_mbox), 16, 8,
    raw_data_str, OTX_CPT_MAX_MBOX_DATA_STR_SIZE, false);
    if (vf_id >= 0)
    pr_debug("MBOX msg %s received from VF%d raw_data %s",
    get_mbox_opcode_str(mbox_msg.msg), vf_id,
    raw_data_str);
    else
    pr_debug("MBOX msg %s received from PF raw_data %s",
    get_mbox_opcode_str(mbox_msg.msg), raw_data_str);
    }
    static void cptvf_send_msg_to_pf(struct otx_cptvf *cptvf,
    struct otx_cpt_mbox *mbx)
    {
// Writing mbox(1) causes interrupt
    writeq(mbx.msg, cptvf.reg_base + OTX_CPT_VFX_PF_MBOXX(0, 0));
    writeq(mbx.data, cptvf.reg_base + OTX_CPT_VFX_PF_MBOXX(0, 1));
    }
// Interrupt handler to handle mailbox messages from VFs
#[no_mangle]
pub unsafe extern "C" fn otx_cptvf_handle_mbox_intr(cptvf: *mut otx_cptvf) {
    void otx_cptvf_handle_mbox_intr(struct otx_cptvf *cptvf)
    {
    let mut mbx: otx_cpt_mbox = {};
//
// MBOX[0] contains msg
// MBOX[1] contains data
//
    mbx.msg  = readq(cptvf.reg_base + OTX_CPT_VFX_PF_MBOXX(0, 0));
    mbx.data = readq(cptvf.reg_base + OTX_CPT_VFX_PF_MBOXX(0, 1));
    dump_mbox_msg(&mbx, -1);
    switch (mbx.msg) {
    case OTX_CPT_MSG_VF_UP:
    cptvf.pf_acked = true;
    cptvf.num_vfs = mbx.data;
    break;
    case OTX_CPT_MSG_READY:
    cptvf.pf_acked = true;
    cptvf.vfid = mbx.data;
    dev_dbg(&cptvf.pdev.dev, "Received VFID %d\n", cptvf.vfid);
    break;
    case OTX_CPT_MSG_QBIND_GRP:
    cptvf.pf_acked = true;
    cptvf.vftype = mbx.data;
    dev_dbg(&cptvf.pdev.dev, "VF %d type %s group %d\n",
    cptvf.vfid,
    ((mbx.data == OTX_CPT_SE_TYPES) ? "SE" : "AE"),
    cptvf.vfgrp);
    break;
    case OTX_CPT_MSG_ACK:
    cptvf.pf_acked = true;
    break;
    case OTX_CPT_MSG_NACK:
    cptvf.pf_nacked = true;
    break;
    default:
    dev_err(&cptvf.pdev.dev, "Invalid msg from PF, msg 0x%llx\n",
    mbx.msg);
    break;
    }
    }
    static int cptvf_send_msg_to_pf_timeout(struct otx_cptvf *cptvf,
    struct otx_cpt_mbox *mbx)
    {
    let mut timeout: c_int = CPT_MBOX_MSG_TIMEOUT;
    let mut sleep: c_int = 10;
    cptvf.pf_acked = false;
    cptvf.pf_nacked = false;
    cptvf_send_msg_to_pf(cptvf, mbx);
// Wait for previous message to be acked, timeout 2sec
    while (!cptvf.pf_acked) {
    if (cptvf.pf_nacked)
    return -EINVAL;
    msleep(sleep);
    if (cptvf.pf_acked)
    break;
    timeout -= sleep;
    if (!timeout) {
    dev_err(&cptvf.pdev.dev,
    "PF didn't ack to mbox msg %llx from VF%u\n",
    mbx.msg, cptvf.vfid);
    return -EBUSY;
    }
    }
    return 0;
    }
//
// Checks if VF is able to comminicate with PF
// and also gets the CPT number this VF is associated to.
//
#[no_mangle]
pub unsafe extern "C" fn otx_cptvf_check_pf_ready(cptvf: *mut otx_cptvf) -> c_int {
    int otx_cptvf_check_pf_ready(struct otx_cptvf *cptvf)
    {
    let mut mbx: otx_cpt_mbox = {};
    mbx.msg = OTX_CPT_MSG_READY;
    return cptvf_send_msg_to_pf_timeout(cptvf, &mbx);
    }
//
// Communicate VQs size to PF to program CPT(0)_PF_Q(0-15)_CTL of the VF.
// Must be ACKed.
//
#[no_mangle]
pub unsafe extern "C" fn otx_cptvf_send_vq_size_msg(cptvf: *mut otx_cptvf) -> c_int {
    int otx_cptvf_send_vq_size_msg(struct otx_cptvf *cptvf)
    {
    let mut mbx: otx_cpt_mbox = {};
    mbx.msg = OTX_CPT_MSG_QLEN;
    mbx.data = cptvf.qsize;
    return cptvf_send_msg_to_pf_timeout(cptvf, &mbx);
    }
//
// Communicate VF group required to PF and get the VQ binded to that group
//
#[no_mangle]
pub unsafe extern "C" fn otx_cptvf_send_vf_to_grp_msg(cptvf: *mut otx_cptvf, group: c_int) -> c_int {
    int otx_cptvf_send_vf_to_grp_msg(struct otx_cptvf *cptvf, int group)
    {
    let mut mbx: otx_cpt_mbox = {};
    int ret;
    mbx.msg = OTX_CPT_MSG_QBIND_GRP;
// Convey group of the VF
    mbx.data = group;
    ret = cptvf_send_msg_to_pf_timeout(cptvf, &mbx);
    if (ret)
    return ret;
    cptvf.vfgrp = group;
    return 0;
    }
//
// Communicate VF group required to PF and get the VQ binded to that group
//
#[no_mangle]
pub unsafe extern "C" fn otx_cptvf_send_vf_priority_msg(cptvf: *mut otx_cptvf) -> c_int {
    int otx_cptvf_send_vf_priority_msg(struct otx_cptvf *cptvf)
    {
    let mut mbx: otx_cpt_mbox = {};
    mbx.msg = OTX_CPT_MSG_VQ_PRIORITY;
// Convey group of the VF
    mbx.data = cptvf.priority;
    return cptvf_send_msg_to_pf_timeout(cptvf, &mbx);
    }
//
// Communicate to PF that VF is UP and running
//
#[no_mangle]
pub unsafe extern "C" fn otx_cptvf_send_vf_up(cptvf: *mut otx_cptvf) -> c_int {
    int otx_cptvf_send_vf_up(struct otx_cptvf *cptvf)
    {
    let mut mbx: otx_cpt_mbox = {};
    mbx.msg = OTX_CPT_MSG_VF_UP;
    return cptvf_send_msg_to_pf_timeout(cptvf, &mbx);
    }
//
// Communicate to PF that VF is DOWN and running
//
#[no_mangle]
pub unsafe extern "C" fn otx_cptvf_send_vf_down(cptvf: *mut otx_cptvf) -> c_int {
    int otx_cptvf_send_vf_down(struct otx_cptvf *cptvf)
    {
    let mut mbx: otx_cpt_mbox = {};
    mbx.msg = OTX_CPT_MSG_VF_DOWN;
    return cptvf_send_msg_to_pf_timeout(cptvf, &mbx);
    }
