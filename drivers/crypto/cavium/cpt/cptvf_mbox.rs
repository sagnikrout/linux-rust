//! Automatically rewritten from C to Rust
//! Source: drivers/crypto/cavium/cpt/cptvf_mbox.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (C) 2016 Cavium, Inc.
//

#[no_mangle]
unsafe extern "C" fn cptvf_send_msg_to_pf(cptvf: *mut cpt_vf, mbx: *mut cpt_mbox) {
    static void cptvf_send_msg_to_pf(struct cpt_vf *cptvf, struct cpt_mbox *mbx)
    {
// Writing mbox(1) causes interrupt
    cpt_write_csr64(cptvf.reg_base, CPTX_VFX_PF_MBOXX(0, 0, 0),
    mbx.msg);
    cpt_write_csr64(cptvf.reg_base, CPTX_VFX_PF_MBOXX(0, 0, 1),
    mbx.data);
    }
// Interrupt handler to handle mailbox messages from VFs
#[no_mangle]
pub unsafe extern "C" fn cptvf_handle_mbox_intr(cptvf: *mut cpt_vf) {
    void cptvf_handle_mbox_intr(struct cpt_vf *cptvf)
    {
    let mut mbx: cpt_mbox = {};
//
// MBOX[0] contains msg
// MBOX[1] contains data
//
    mbx.msg  = cpt_read_csr64(cptvf.reg_base, CPTX_VFX_PF_MBOXX(0, 0, 0));
    mbx.data = cpt_read_csr64(cptvf.reg_base, CPTX_VFX_PF_MBOXX(0, 0, 1));
    dev_dbg(&cptvf.pdev.dev, "%s: Mailbox msg 0x%llx from PF\n",
    __func__, mbx.msg);
    switch (mbx.msg) {
    case CPT_MSG_READY:
    {
    cptvf.pf_acked = true;
    cptvf.vfid = mbx.data;
    dev_dbg(&cptvf.pdev.dev, "Received VFID %d\n", cptvf.vfid);
    break;
    }
    case CPT_MSG_QBIND_GRP:
    cptvf.pf_acked = true;
    cptvf.vftype = mbx.data;
    dev_dbg(&cptvf.pdev.dev, "VF %d type %s group %d\n",
    cptvf.vfid, ((mbx.data == SE_TYPES) ? "SE" : "AE"),
    cptvf.vfgrp);
    break;
    case CPT_MBOX_MSG_TYPE_ACK:
    cptvf.pf_acked = true;
    break;
    case CPT_MBOX_MSG_TYPE_NACK:
    cptvf.pf_nacked = true;
    break;
    default:
    dev_err(&cptvf.pdev.dev, "Invalid msg from PF, msg 0x%llx\n",
    mbx.msg);
    break;
    }
    }
    static int cptvf_send_msg_to_pf_timeout(struct cpt_vf *cptvf,
    struct cpt_mbox *mbx)
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
    dev_err(&cptvf.pdev.dev, "PF didn't ack to mbox msg %llx from VF%u\n",
    (mbx.msg & 0xFF), cptvf.vfid);
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
pub unsafe extern "C" fn cptvf_check_pf_ready(cptvf: *mut cpt_vf) -> c_int {
    int cptvf_check_pf_ready(struct cpt_vf *cptvf)
    {
    struct pci_dev *pdev = cptvf.pdev;
    let mut mbx: cpt_mbox = {};
    mbx.msg = CPT_MSG_READY;
    if (cptvf_send_msg_to_pf_timeout(cptvf, &mbx)) {
    dev_err(&pdev.dev, "PF didn't respond to READY msg\n");
    return -EBUSY;
    }
    return 0;
    }
//
// Communicate VQs size to PF to program CPT(0)_PF_Q(0-15)_CTL of the VF.
// Must be ACKed.
//
#[no_mangle]
pub unsafe extern "C" fn cptvf_send_vq_size_msg(cptvf: *mut cpt_vf) -> c_int {
    int cptvf_send_vq_size_msg(struct cpt_vf *cptvf)
    {
    struct pci_dev *pdev = cptvf.pdev;
    let mut mbx: cpt_mbox = {};
    mbx.msg = CPT_MSG_QLEN;
    mbx.data = cptvf.qsize;
    if (cptvf_send_msg_to_pf_timeout(cptvf, &mbx)) {
    dev_err(&pdev.dev, "PF didn't respond to vq_size msg\n");
    return -EBUSY;
    }
    return 0;
    }
//
// Communicate VF group required to PF and get the VQ binded to that group
//
#[no_mangle]
pub unsafe extern "C" fn cptvf_send_vf_to_grp_msg(cptvf: *mut cpt_vf) -> c_int {
    int cptvf_send_vf_to_grp_msg(struct cpt_vf *cptvf)
    {
    struct pci_dev *pdev = cptvf.pdev;
    let mut mbx: cpt_mbox = {};
    mbx.msg = CPT_MSG_QBIND_GRP;
// Convey group of the VF
    mbx.data = cptvf.vfgrp;
    if (cptvf_send_msg_to_pf_timeout(cptvf, &mbx)) {
    dev_err(&pdev.dev, "PF didn't respond to vf_type msg\n");
    return -EBUSY;
    }
    return 0;
    }
//
// Communicate VF group required to PF and get the VQ binded to that group
//
#[no_mangle]
pub unsafe extern "C" fn cptvf_send_vf_priority_msg(cptvf: *mut cpt_vf) -> c_int {
    int cptvf_send_vf_priority_msg(struct cpt_vf *cptvf)
    {
    struct pci_dev *pdev = cptvf.pdev;
    let mut mbx: cpt_mbox = {};
    mbx.msg = CPT_MSG_VQ_PRIORITY;
// Convey group of the VF
    mbx.data = cptvf.priority;
    if (cptvf_send_msg_to_pf_timeout(cptvf, &mbx)) {
    dev_err(&pdev.dev, "PF didn't respond to vf_type msg\n");
    return -EBUSY;
    }
    return 0;
    }
//
// Communicate to PF that VF is UP and running
//
#[no_mangle]
pub unsafe extern "C" fn cptvf_send_vf_up(cptvf: *mut cpt_vf) -> c_int {
    int cptvf_send_vf_up(struct cpt_vf *cptvf)
    {
    struct pci_dev *pdev = cptvf.pdev;
    let mut mbx: cpt_mbox = {};
    mbx.msg = CPT_MSG_VF_UP;
    if (cptvf_send_msg_to_pf_timeout(cptvf, &mbx)) {
    dev_err(&pdev.dev, "PF didn't respond to UP msg\n");
    return -EBUSY;
    }
    return 0;
    }
//
// Communicate to PF that VF is DOWN and running
//
#[no_mangle]
pub unsafe extern "C" fn cptvf_send_vf_down(cptvf: *mut cpt_vf) -> c_int {
    int cptvf_send_vf_down(struct cpt_vf *cptvf)
    {
    struct pci_dev *pdev = cptvf.pdev;
    let mut mbx: cpt_mbox = {};
    mbx.msg = CPT_MSG_VF_DOWN;
    if (cptvf_send_msg_to_pf_timeout(cptvf, &mbx)) {
    dev_err(&pdev.dev, "PF didn't respond to DOWN msg\n");
    return -EBUSY;
    }
    return 0;
    }
