//! Automatically rewritten from C to Rust
//! Source: drivers/crypto/cavium/cpt/cptpf_mbox.c
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

    static void cpt_send_msg_to_vf(struct cpt_device *cpt, int vf,
    struct cpt_mbox *mbx)
    {
// Writing mbox(0) causes interrupt
    cpt_write_csr64(cpt.reg_base, CPTX_PF_VFX_MBOXX(0, vf, 1),
    mbx.data);
    cpt_write_csr64(cpt.reg_base, CPTX_PF_VFX_MBOXX(0, vf, 0), mbx.msg);
    }
// ACKs VF's mailbox message
// @vf: VF to which ACK to be sent
//
    static void cpt_mbox_send_ack(struct cpt_device *cpt, int vf,
    struct cpt_mbox *mbx)
    {
    mbx.data = 0ull;
    mbx.msg = CPT_MBOX_MSG_TYPE_ACK;
    cpt_send_msg_to_vf(cpt, vf, mbx);
    }
#[no_mangle]
unsafe extern "C" fn cpt_clear_mbox_intr(cpt: *mut cpt_device, vf: u32) {
    static void cpt_clear_mbox_intr(struct cpt_device *cpt, u32 vf)
    {
// W1C for the VF
    cpt_write_csr64(cpt.reg_base, CPTX_PF_MBOX_INTX(0, 0), (1 << vf));
    }
//
// Configure QLEN/Chunk sizes for VF
//
#[no_mangle]
unsafe extern "C" fn cpt_cfg_qlen_for_vf(cpt: *mut cpt_device, vf: c_int, size: u32) {
    static void cpt_cfg_qlen_for_vf(struct cpt_device *cpt, int vf, u32 size)
    {
    union cptx_pf_qx_ctl pf_qx_ctl;
    pf_qx_ctl.u = cpt_read_csr64(cpt.reg_base, CPTX_PF_QX_CTL(0, vf));
    pf_qx_ctl.s.size = size;
    pf_qx_ctl.s.cont_err = true;
    cpt_write_csr64(cpt.reg_base, CPTX_PF_QX_CTL(0, vf), pf_qx_ctl.u);
    }
//
// Configure VQ priority
//
#[no_mangle]
unsafe extern "C" fn cpt_cfg_vq_priority(cpt: *mut cpt_device, vf: c_int, pri: u32) {
    static void cpt_cfg_vq_priority(struct cpt_device *cpt, int vf, u32 pri)
    {
    union cptx_pf_qx_ctl pf_qx_ctl;
    pf_qx_ctl.u = cpt_read_csr64(cpt.reg_base, CPTX_PF_QX_CTL(0, vf));
    pf_qx_ctl.s.pri = pri;
    cpt_write_csr64(cpt.reg_base, CPTX_PF_QX_CTL(0, vf), pf_qx_ctl.u);
    }
#[no_mangle]
unsafe extern "C" fn cpt_bind_vq_to_grp(cpt: *mut cpt_device, q: u8, grp: u8) -> c_int {
    static int cpt_bind_vq_to_grp(struct cpt_device *cpt, u8 q, u8 grp)
    {
    struct microcode *mcode = cpt.mcode;
    union cptx_pf_qx_ctl pf_qx_ctl;
    struct device *dev = &cpt.pdev.dev;
    if (q >= CPT_MAX_VF_NUM) {
    dev_err(dev, "Queues are more than cores in the group");
    return -EINVAL;
    }
    if (grp >= CPT_MAX_CORE_GROUPS) {
    dev_err(dev, "Request group is more than possible groups");
    return -EINVAL;
    }
    if (grp >= cpt.next_mc_idx) {
    dev_err(dev, "Request group is higher than available functional groups");
    return -EINVAL;
    }
    pf_qx_ctl.u = cpt_read_csr64(cpt.reg_base, CPTX_PF_QX_CTL(0, q));
    pf_qx_ctl.s.grp = mcode[grp].group;
    cpt_write_csr64(cpt.reg_base, CPTX_PF_QX_CTL(0, q), pf_qx_ctl.u);
    dev_dbg(dev, "VF %d TYPE %s", q, (mcode[grp].is_ae ? "AE" : "SE"));
    return mcode[grp].is_ae ? AE_TYPES : SE_TYPES;
    }
// Interrupt handler to handle mailbox messages from VFs
#[no_mangle]
unsafe extern "C" fn cpt_handle_mbox_intr(cpt: *mut cpt_device, vf: c_int) {
    static void cpt_handle_mbox_intr(struct cpt_device *cpt, int vf)
    {
    struct cpt_vf_info *vfx = &cpt.vfinfo[vf];
    let mut mbx: cpt_mbox = {};
    int vftype;
    struct device *dev = &cpt.pdev.dev;
//
// MBOX[0] contains msg
// MBOX[1] contains data
//
    mbx.msg  = cpt_read_csr64(cpt.reg_base, CPTX_PF_VFX_MBOXX(0, vf, 0));
    mbx.data = cpt_read_csr64(cpt.reg_base, CPTX_PF_VFX_MBOXX(0, vf, 1));
    dev_dbg(dev, "%s: Mailbox msg 0x%llx from VF%d", __func__, mbx.msg, vf);
    switch (mbx.msg) {
    case CPT_MSG_VF_UP:
    vfx.state = VF_STATE_UP;
    try_module_get(THIS_MODULE);
    cpt_mbox_send_ack(cpt, vf, &mbx);
    break;
    case CPT_MSG_READY:
    mbx.msg  = CPT_MSG_READY;
    mbx.data = vf;
    cpt_send_msg_to_vf(cpt, vf, &mbx);
    break;
    case CPT_MSG_VF_DOWN:
// First msg in VF teardown sequence
    vfx.state = VF_STATE_DOWN;
    module_put(THIS_MODULE);
    cpt_mbox_send_ack(cpt, vf, &mbx);
    break;
    case CPT_MSG_QLEN:
    vfx.qlen = mbx.data;
    cpt_cfg_qlen_for_vf(cpt, vf, vfx.qlen);
    cpt_mbox_send_ack(cpt, vf, &mbx);
    break;
    case CPT_MSG_QBIND_GRP:
    vftype = cpt_bind_vq_to_grp(cpt, vf, (u8)mbx.data);
    if ((vftype != AE_TYPES) && (vftype != SE_TYPES))
    dev_err(dev, "Queue %d binding to group %llu failed",
    vf, mbx.data);
    else {
    dev_dbg(dev, "Queue %d binding to group %llu successful",
    vf, mbx.data);
    mbx.msg = CPT_MSG_QBIND_GRP;
    mbx.data = vftype;
    cpt_send_msg_to_vf(cpt, vf, &mbx);
    }
    break;
    case CPT_MSG_VQ_PRIORITY:
    vfx.priority = mbx.data;
    cpt_cfg_vq_priority(cpt, vf, vfx.priority);
    cpt_mbox_send_ack(cpt, vf, &mbx);
    break;
    default:
    dev_err(&cpt.pdev.dev, "Invalid msg from VF%d, msg 0x%llx\n",
    vf, mbx.msg);
    break;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn cpt_mbox_intr_handler(cpt: *mut cpt_device, mbx: c_int) {
    void cpt_mbox_intr_handler (struct cpt_device *cpt, int mbx)
    {
    u64 intr;
    u8  vf;
    intr = cpt_read_csr64(cpt.reg_base, CPTX_PF_MBOX_INTX(0, 0));
    dev_dbg(&cpt.pdev.dev, "PF interrupt Mbox%d 0x%llx\n", mbx, intr);
    for (vf = 0; vf < CPT_MAX_VF_NUM; vf++) {
    if (intr & (1ULL << vf)) {
    dev_dbg(&cpt.pdev.dev, "Intr from VF %d\n", vf);
    cpt_handle_mbox_intr(cpt, vf);
    cpt_clear_mbox_intr(cpt, vf);
    }
    }
    }
