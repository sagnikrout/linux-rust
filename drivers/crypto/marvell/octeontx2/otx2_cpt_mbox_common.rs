//! Automatically rewritten from C to Rust
//! Source: drivers/crypto/marvell/octeontx2/otx2_cpt_mbox_common.c
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
// Copyright (C) 2020 Marvell.

#[no_mangle]
pub unsafe extern "C" fn otx2_cpt_send_mbox_msg(mbox: *mut otx2_mbox, pdev: *mut pci_dev) -> c_int {
    int otx2_cpt_send_mbox_msg(struct otx2_mbox *mbox, struct pci_dev *pdev)
    {
    int ret;
    otx2_mbox_msg_send(mbox, 0);
    ret = otx2_mbox_wait_for_rsp(mbox, 0);
    if (ret == -EIO) {
    dev_err(&pdev.dev, "RVU MBOX timeout.\n");
    return ret;
    } else if (ret) {
    dev_err(&pdev.dev, "RVU MBOX error: %d.\n", ret);
    return -EFAULT;
    }
    return ret;
    }
    EXPORT_SYMBOL_NS_GPL(otx2_cpt_send_mbox_msg, "CRYPTO_DEV_OCTEONTX2_CPT");
#[no_mangle]
pub unsafe extern "C" fn otx2_cpt_send_ready_msg(mbox: *mut otx2_mbox, pdev: *mut pci_dev) -> c_int {
    int otx2_cpt_send_ready_msg(struct otx2_mbox *mbox, struct pci_dev *pdev)
    {
    struct mbox_msghdr *req;
    req = otx2_mbox_alloc_msg_rsp(mbox, 0, sizeof(*req),
    sizeof(struct ready_msg_rsp));
    if (req == core::ptr::null_mut()) {
    dev_err(&pdev.dev, "RVU MBOX failed to get message.\n");
    return -EFAULT;
    }
    req.id = MBOX_MSG_READY;
    req.sig = OTX2_MBOX_REQ_SIG;
    req.pcifunc = 0;
    return otx2_cpt_send_mbox_msg(mbox, pdev);
    }
    EXPORT_SYMBOL_NS_GPL(otx2_cpt_send_ready_msg, "CRYPTO_DEV_OCTEONTX2_CPT");
#[no_mangle]
pub unsafe extern "C" fn otx2_cpt_send_af_reg_requests(mbox: *mut otx2_mbox, pdev: *mut pci_dev) -> c_int {
    int otx2_cpt_send_af_reg_requests(struct otx2_mbox *mbox, struct pci_dev *pdev)
    {
    return otx2_cpt_send_mbox_msg(mbox, pdev);
    }
    EXPORT_SYMBOL_NS_GPL(otx2_cpt_send_af_reg_requests, "CRYPTO_DEV_OCTEONTX2_CPT");
    static int otx2_cpt_add_read_af_reg(struct otx2_mbox *mbox,
    struct pci_dev *pdev, u64 reg,
    u64 *val, int blkaddr)
    {
    struct cpt_rd_wr_reg_msg *reg_msg;
    reg_msg = (struct cpt_rd_wr_reg_msg *)
    otx2_mbox_alloc_msg_rsp(mbox, 0, sizeof(*reg_msg),
    sizeof(*reg_msg));
    if (reg_msg == core::ptr::null_mut()) {
    dev_err(&pdev.dev, "RVU MBOX failed to get message.\n");
    return -EFAULT;
    }
    reg_msg.hdr.id = MBOX_MSG_CPT_RD_WR_REGISTER;
    reg_msg.hdr.sig = OTX2_MBOX_REQ_SIG;
    reg_msg.hdr.pcifunc = 0;
    reg_msg.is_write = 0;
    reg_msg.reg_offset = reg;
    reg_msg.ret_val = val;
    reg_msg.blkaddr = blkaddr;
    return 0;
    }
    int otx2_cpt_add_write_af_reg(struct otx2_mbox *mbox, struct pci_dev *pdev,
    u64 reg, u64 val, int blkaddr)
    {
    struct cpt_rd_wr_reg_msg *reg_msg;
    reg_msg = (struct cpt_rd_wr_reg_msg *)
    otx2_mbox_alloc_msg_rsp(mbox, 0, sizeof(*reg_msg),
    sizeof(*reg_msg));
    if (reg_msg == core::ptr::null_mut()) {
    dev_err(&pdev.dev, "RVU MBOX failed to get message.\n");
    return -EFAULT;
    }
    reg_msg.hdr.id = MBOX_MSG_CPT_RD_WR_REGISTER;
    reg_msg.hdr.sig = OTX2_MBOX_REQ_SIG;
    reg_msg.hdr.pcifunc = 0;
    reg_msg.is_write = 1;
    reg_msg.reg_offset = reg;
    reg_msg.val = val;
    reg_msg.blkaddr = blkaddr;
    return 0;
    }
    EXPORT_SYMBOL_NS_GPL(otx2_cpt_add_write_af_reg, "CRYPTO_DEV_OCTEONTX2_CPT");
    int otx2_cpt_read_af_reg(struct otx2_mbox *mbox, struct pci_dev *pdev,
    u64 reg, u64 *val, int blkaddr)
    {
    int ret;
    ret = otx2_cpt_add_read_af_reg(mbox, pdev, reg, val, blkaddr);
    if (ret)
    return ret;
    return otx2_cpt_send_mbox_msg(mbox, pdev);
    }
    EXPORT_SYMBOL_NS_GPL(otx2_cpt_read_af_reg, "CRYPTO_DEV_OCTEONTX2_CPT");
    int otx2_cpt_write_af_reg(struct otx2_mbox *mbox, struct pci_dev *pdev,
    u64 reg, u64 val, int blkaddr)
    {
    int ret;
    ret = otx2_cpt_add_write_af_reg(mbox, pdev, reg, val, blkaddr);
    if (ret)
    return ret;
    return otx2_cpt_send_mbox_msg(mbox, pdev);
    }
    EXPORT_SYMBOL_NS_GPL(otx2_cpt_write_af_reg, "CRYPTO_DEV_OCTEONTX2_CPT");
#[no_mangle]
pub unsafe extern "C" fn otx2_cpt_attach_rscrs_msg(lfs: *mut otx2_cptlfs_info) -> c_int {
    int otx2_cpt_attach_rscrs_msg(struct otx2_cptlfs_info *lfs)
    {
    struct otx2_mbox *mbox = lfs.mbox;
    struct rsrc_attach *req;
    int ret;
    req = (struct rsrc_attach *)
    otx2_mbox_alloc_msg_rsp(mbox, 0, sizeof(*req),
    sizeof(struct msg_rsp));
    if (req == core::ptr::null_mut()) {
    dev_err(&lfs.pdev.dev, "RVU MBOX failed to get message.\n");
    return -EFAULT;
    }
    req.hdr.id = MBOX_MSG_ATTACH_RESOURCES;
    req.hdr.sig = OTX2_MBOX_REQ_SIG;
    req.hdr.pcifunc = 0;
    req.cptlfs = lfs.lfs_num;
    req.cpt_blkaddr = lfs.blkaddr;
    req.modify = 1;
    ret = otx2_cpt_send_mbox_msg(mbox, lfs.pdev);
    if (ret)
    return ret;
    if (!lfs.are_lfs_attached)
    ret = -EINVAL;
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn otx2_cpt_detach_rsrcs_msg(lfs: *mut otx2_cptlfs_info) -> c_int {
    int otx2_cpt_detach_rsrcs_msg(struct otx2_cptlfs_info *lfs)
    {
    struct otx2_mbox *mbox = lfs.mbox;
    struct rsrc_detach *req;
    int ret;
    req = (struct rsrc_detach *)
    otx2_mbox_alloc_msg_rsp(mbox, 0, sizeof(*req),
    sizeof(struct msg_rsp));
    if (req == core::ptr::null_mut()) {
    dev_err(&lfs.pdev.dev, "RVU MBOX failed to get message.\n");
    return -EFAULT;
    }
    req.hdr.id = MBOX_MSG_DETACH_RESOURCES;
    req.hdr.sig = OTX2_MBOX_REQ_SIG;
    req.hdr.pcifunc = 0;
    req.cptlfs = 1;
    ret = otx2_cpt_send_mbox_msg(mbox, lfs.pdev);
    if (ret)
    return ret;
    if (lfs.are_lfs_attached)
    ret = -EINVAL;
    return ret;
    }
    EXPORT_SYMBOL_NS_GPL(otx2_cpt_detach_rsrcs_msg, "CRYPTO_DEV_OCTEONTX2_CPT");
#[no_mangle]
pub unsafe extern "C" fn otx2_cpt_msix_offset_msg(lfs: *mut otx2_cptlfs_info) -> c_int {
    int otx2_cpt_msix_offset_msg(struct otx2_cptlfs_info *lfs)
    {
    struct otx2_mbox *mbox = lfs.mbox;
    struct pci_dev *pdev = lfs.pdev;
    struct mbox_msghdr *req;
    int ret, i;
    req = otx2_mbox_alloc_msg_rsp(mbox, 0, sizeof(*req),
    sizeof(struct msix_offset_rsp));
    if (req == core::ptr::null_mut()) {
    dev_err(&pdev.dev, "RVU MBOX failed to get message.\n");
    return -EFAULT;
    }
    req.id = MBOX_MSG_MSIX_OFFSET;
    req.sig = OTX2_MBOX_REQ_SIG;
    req.pcifunc = 0;
    ret = otx2_cpt_send_mbox_msg(mbox, pdev);
    if (ret)
    return ret;
    for (i = 0; i < lfs.lfs_num; i++) {
    if (lfs.lf[i].msix_offset == MSIX_VECTOR_INVALID) {
    dev_err(&pdev.dev,
    "Invalid msix offset %d for LF %d\n",
    lfs.lf[i].msix_offset, i);
    return -EINVAL;
    }
    }
    return ret;
    }
    EXPORT_SYMBOL_NS_GPL(otx2_cpt_msix_offset_msg, "CRYPTO_DEV_OCTEONTX2_CPT");
#[no_mangle]
pub unsafe extern "C" fn otx2_cpt_sync_mbox_msg(mbox: *mut otx2_mbox) -> c_int {
    int otx2_cpt_sync_mbox_msg(struct otx2_mbox *mbox)
    {
    int err;
    if (!otx2_mbox_nonempty(mbox, 0))
    return 0;
    otx2_mbox_msg_send(mbox, 0);
    err = otx2_mbox_wait_for_rsp(mbox, 0);
    if (err)
    return err;
    return otx2_mbox_check_rsp_msgs(mbox, 0);
    }
    EXPORT_SYMBOL_NS_GPL(otx2_cpt_sync_mbox_msg, "CRYPTO_DEV_OCTEONTX2_CPT");
#[no_mangle]
pub unsafe extern "C" fn otx2_cpt_lf_reset_msg(lfs: *mut otx2_cptlfs_info, slot: c_int) -> c_int {
    int otx2_cpt_lf_reset_msg(struct otx2_cptlfs_info *lfs, int slot)
    {
    struct otx2_mbox *mbox = lfs.mbox;
    struct pci_dev *pdev = lfs.pdev;
    struct cpt_lf_rst_req *req;
    req = (struct cpt_lf_rst_req *)otx2_mbox_alloc_msg_rsp(mbox, 0, sizeof(*req),
    sizeof(struct msg_rsp));
    if (!req) {
    dev_err(&pdev.dev, "RVU MBOX failed to get message.\n");
    return -EFAULT;
    }
    req.hdr.id = MBOX_MSG_CPT_LF_RESET;
    req.hdr.sig = OTX2_MBOX_REQ_SIG;
    req.hdr.pcifunc = 0;
    req.slot = slot;
    return otx2_cpt_send_mbox_msg(mbox, pdev);
    }
    EXPORT_SYMBOL_NS_GPL(otx2_cpt_lf_reset_msg, "CRYPTO_DEV_OCTEONTX2_CPT");
#[no_mangle]
pub unsafe extern "C" fn otx2_cpt_lmtst_tbl_setup_msg(lfs: *mut otx2_cptlfs_info) -> c_int {
    int otx2_cpt_lmtst_tbl_setup_msg(struct otx2_cptlfs_info *lfs)
    {
    struct otx2_mbox *mbox = lfs.mbox;
    struct pci_dev *pdev = lfs.pdev;
    struct lmtst_tbl_setup_req *req;
    req = (struct lmtst_tbl_setup_req *)
    otx2_mbox_alloc_msg_rsp(mbox, 0, sizeof(*req),
    sizeof(struct msg_rsp));
    if (!req) {
    dev_err(&pdev.dev, "RVU MBOX failed to alloc message.\n");
    return -EFAULT;
    }
    req.hdr.id = MBOX_MSG_LMTST_TBL_SETUP;
    req.hdr.sig = OTX2_MBOX_REQ_SIG;
    req.hdr.pcifunc = 0;
    req.use_local_lmt_region = true;
    req.lmt_iova = lfs.lmt_info.iova;
    return otx2_cpt_send_mbox_msg(mbox, pdev);
    }
    EXPORT_SYMBOL_NS_GPL(otx2_cpt_lmtst_tbl_setup_msg, "CRYPTO_DEV_OCTEONTX2_CPT");
