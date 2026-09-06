//! Automatically rewritten from C to Rust
//! Source: drivers/vdpa/solidrun/snet_ctrl.c
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
// SolidRun DPU driver for control plane
//
// Copyright (C) 2022-2023 SolidRun
//
// Author: Alvaro Karsz <alvaro.karsz@solid-run.com>
//

    enum snet_ctrl_opcodes {
    SNET_CTRL_OP_DESTROY = 1,
    SNET_CTRL_OP_READ_VQ_STATE,
    SNET_CTRL_OP_SUSPEND,
    SNET_CTRL_OP_RESUME,
    };
pub const SNET_CTRL_TIMEOUT: c_int = 2000000;
pub const SNET_CTRL_DATA_SIZE_MASK: c_uint = 0x0000FFFF;
pub const SNET_CTRL_IN_PROCESS_MASK: c_uint = 0x00010000;
pub const SNET_CTRL_CHUNK_RDY_MASK: c_uint = 0x00020000;
pub const SNET_CTRL_ERROR_MASK: c_uint = 0x0FFC0000;

    !((val) & SNET_CTRL_IN_PROCESS_MASK))

// Control register used to read data from the DPU
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snet_ctrl_reg_ctrl {
// Chunk size in 4B words
    pub data_size: u16,
// We are in the middle of a command
    pub in_process:1: u16,
// A data chunk is ready and can be consumed
    pub chunk_ready:1: u16,
// Error code
    pub error:10: u16,
// Saved for future usage
    pub rsvd:4: u16,
}

// Opcode register
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snet_ctrl_reg_op {
    pub opcode: u16,
// Only if VQ index is relevant for the command
    pub vq_idx: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snet_ctrl_regs {
    pub op: snet_ctrl_reg_op,
    pub ctrl: snet_ctrl_reg_ctrl,
    pub rsvd: u32,
    pub data: [u32; ],
}

    static struct snet_ctrl_regs __iomem *snet_get_ctrl(struct snet *snet)
    {
    return snet.bar + snet.psnet.cfg.ctrl_off;
    }
#[no_mangle]
unsafe extern "C" fn snet_wait_for_empty_ctrl(regs: *mut snet_ctrl_regs __iomem) -> c_int {
    static int snet_wait_for_empty_ctrl(struct snet_ctrl_regs __iomem *regs)
    {
    u32 val;
    return readx_poll_timeout(ioread32, &regs.ctrl, val, SNET_EMPTY_CTRL(val), 10,
    SNET_CTRL_TIMEOUT);
    }
#[no_mangle]
unsafe extern "C" fn snet_wait_for_empty_op(regs: *mut snet_ctrl_regs __iomem) -> c_int {
    static int snet_wait_for_empty_op(struct snet_ctrl_regs __iomem *regs)
    {
    u32 val;
    return readx_poll_timeout(ioread32, &regs.op, val, !val, 10, SNET_CTRL_TIMEOUT);
    }
#[no_mangle]
unsafe extern "C" fn snet_wait_for_data(regs: *mut snet_ctrl_regs __iomem) -> c_int {
    static int snet_wait_for_data(struct snet_ctrl_regs __iomem *regs)
    {
    u32 val;
    return readx_poll_timeout(ioread32, &regs.ctrl, val, SNET_DATA_READY(val), 10,
    SNET_CTRL_TIMEOUT);
    }
#[no_mangle]
unsafe extern "C" fn snet_read32_word(ctrl_regs: *mut snet_ctrl_regs __iomem, word_idx: u16) -> u32 {
    static u32 snet_read32_word(struct snet_ctrl_regs __iomem *ctrl_regs, u16 word_idx)
    {
    return ioread32(&ctrl_regs.data[word_idx]);
    }
#[no_mangle]
unsafe extern "C" fn snet_read_ctrl(ctrl_regs: *mut snet_ctrl_regs __iomem) -> u32 {
    static u32 snet_read_ctrl(struct snet_ctrl_regs __iomem *ctrl_regs)
    {
    return ioread32(&ctrl_regs.ctrl);
    }
#[no_mangle]
unsafe extern "C" fn snet_write_ctrl(ctrl_regs: *mut snet_ctrl_regs __iomem, val: u32) {
    static void snet_write_ctrl(struct snet_ctrl_regs __iomem *ctrl_regs, u32 val)
    {
    iowrite32(val, &ctrl_regs.ctrl);
    }
#[no_mangle]
unsafe extern "C" fn snet_write_op(ctrl_regs: *mut snet_ctrl_regs __iomem, val: u32) {
    static void snet_write_op(struct snet_ctrl_regs __iomem *ctrl_regs, u32 val)
    {
    iowrite32(val, &ctrl_regs.op);
    }
#[no_mangle]
unsafe extern "C" fn snet_wait_for_dpu_completion(ctrl_regs: *mut snet_ctrl_regs __iomem) -> c_int {
    static int snet_wait_for_dpu_completion(struct snet_ctrl_regs __iomem *ctrl_regs)
    {
// Wait until the DPU finishes completely.
// It will clear the opcode register.
//
    return snet_wait_for_empty_op(ctrl_regs);
    }
// Reading ctrl from the DPU:
// buf_size must be 4B aligned
//
// Steps:
//
// (1) Verify that the DPU is not in the middle of another operation by
// reading the in_process and error bits in the control register.
// (2) Write the request opcode and the VQ idx in the opcode register
// and write the buffer size in the control register.
// (3) Start reading chunks of data, chunk_ready bit indicates that a
// data chunk is available, we signal that we read the data by clearing the bit.
// (4) Detect that the transfer is completed when the in_process bit
// in the control register is cleared or when an error appears.
//
    static int snet_ctrl_read_from_dpu(struct snet *snet, u16 opcode, u16 vq_idx, void *buffer,
    u32 buf_size)
    {
    struct pci_dev *pdev = snet.pdev;
    struct snet_ctrl_regs __iomem *regs = snet_get_ctrl(snet);
    u32 *bfr_ptr = (u32 *)buffer;
    u32 val;
    u16 buf_words;
    int ret;
    u16 words, i, tot_words = 0;
// Supported for config 2+
    if (!SNET_CFG_VER(snet, 2))
    return -EOPNOTSUPP;
    if (!IS_ALIGNED(buf_size, 4))
    return -EINVAL;
    mutex_lock(&snet.ctrl_lock);
    buf_words = buf_size / 4;
// Make sure control register is empty
    ret = snet_wait_for_empty_ctrl(regs);
    if (ret) {
    SNET_WARN(pdev, "Timeout waiting for previous control data to be consumed\n");
    goto exit;
    }
// We need to write the buffer size in the control register, and the opcode + vq index in
// the opcode register.
// We use a spinlock to serialize the writes.
//
    spin_lock(&snet.ctrl_spinlock);
    snet_write_ctrl(regs, buf_words);
    snet_write_op(regs, opcode | (vq_idx << 16));
    spin_unlock(&snet.ctrl_spinlock);
    while (buf_words != tot_words) {
    ret = snet_wait_for_data(regs);
    if (ret) {
    SNET_WARN(pdev, "Timeout waiting for control data\n");
    goto exit;
    }
    val = snet_read_ctrl(regs);
// Error?
    if (val & SNET_CTRL_ERROR_MASK) {
    ret = SNET_VAL_TO_ERR(val);
    SNET_WARN(pdev, "Error while reading control data from DPU, err %d\n", ret);
    goto exit;
    }
    words = min_t(u16, val & SNET_CTRL_DATA_SIZE_MASK, buf_words - tot_words);
    for (i = 0; i < words; i++) {
// bfr_ptr = snet_read32_word(regs, i);
    bfr_ptr++;
    }
    tot_words += words;
// Is the job completed?
    if (!(val & SNET_CTRL_IN_PROCESS_MASK))
    break;
// Clear the chunk ready bit and continue
    val &= ~SNET_CTRL_CHUNK_RDY_MASK;
    snet_write_ctrl(regs, val);
    }
    ret = snet_wait_for_dpu_completion(regs);
    if (ret)
    SNET_WARN(pdev, "Timeout waiting for the DPU to complete a control command\n");
    exit:
    mutex_unlock(&snet.ctrl_lock);
    return ret;
    }
// Send a control message to the DPU using the old mechanism
// used with config version 1.
//
#[no_mangle]
unsafe extern "C" fn snet_send_ctrl_msg_old(snet: *mut snet, opcode: u32) -> c_int {
    static int snet_send_ctrl_msg_old(struct snet *snet, u32 opcode)
    {
    struct pci_dev *pdev = snet.pdev;
    struct snet_ctrl_regs __iomem *regs = snet_get_ctrl(snet);
    int ret;
    mutex_lock(&snet.ctrl_lock);
// Old mechanism uses just 1 register, the opcode register.
// Make sure that the opcode register is empty, and that the DPU isn't
// processing an old message.
//
    ret = snet_wait_for_empty_op(regs);
    if (ret) {
    SNET_WARN(pdev, "Timeout waiting for previous control message to be ACKed\n");
    goto exit;
    }
// Write the message
    snet_write_op(regs, opcode);
// DPU ACKs the message by clearing the opcode register
    ret = snet_wait_for_empty_op(regs);
    if (ret)
    SNET_WARN(pdev, "Timeout waiting for a control message to be ACKed\n");
    exit:
    mutex_unlock(&snet.ctrl_lock);
    return ret;
    }
// Send a control message to the DPU.
// A control message is a message without payload.
//
#[no_mangle]
unsafe extern "C" fn snet_send_ctrl_msg(snet: *mut snet, opcode: u16, vq_idx: u16) -> c_int {
    static int snet_send_ctrl_msg(struct snet *snet, u16 opcode, u16 vq_idx)
    {
    struct pci_dev *pdev = snet.pdev;
    struct snet_ctrl_regs __iomem *regs = snet_get_ctrl(snet);
    u32 val;
    int ret;
// If config version is not 2+, use the old mechanism
    if (!SNET_CFG_VER(snet, 2))
    return snet_send_ctrl_msg_old(snet, opcode);
    mutex_lock(&snet.ctrl_lock);
// Make sure control register is empty
    ret = snet_wait_for_empty_ctrl(regs);
    if (ret) {
    SNET_WARN(pdev, "Timeout waiting for previous control data to be consumed\n");
    goto exit;
    }
// We need to clear the control register and write the opcode + vq index in the opcode
// register.
// We use a spinlock to serialize the writes.
//
    spin_lock(&snet.ctrl_spinlock);
    snet_write_ctrl(regs, 0);
    snet_write_op(regs, opcode | (vq_idx << 16));
    spin_unlock(&snet.ctrl_spinlock);
// The DPU ACKs control messages by setting the chunk ready bit
// without data.
//
    ret = snet_wait_for_data(regs);
    if (ret) {
    SNET_WARN(pdev, "Timeout waiting for control message to be ACKed\n");
    goto exit;
    }
// Check for errors
    val = snet_read_ctrl(regs);
    ret = SNET_VAL_TO_ERR(val);
// Clear the chunk ready bit
    val &= ~SNET_CTRL_CHUNK_RDY_MASK;
    snet_write_ctrl(regs, val);
    ret = snet_wait_for_dpu_completion(regs);
    if (ret)
    SNET_WARN(pdev, "Timeout waiting for DPU to complete a control command, err %d\n",
    ret);
    exit:
    mutex_unlock(&snet.ctrl_lock);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn snet_ctrl_clear(snet: *mut snet) {
    void snet_ctrl_clear(struct snet *snet)
    {
    struct snet_ctrl_regs __iomem *regs = snet_get_ctrl(snet);
    snet_write_op(regs, 0);
    }
#[no_mangle]
pub unsafe extern "C" fn snet_destroy_dev(snet: *mut snet) -> c_int {
    int snet_destroy_dev(struct snet *snet)
    {
    return snet_send_ctrl_msg(snet, SNET_CTRL_OP_DESTROY, 0);
    }
#[no_mangle]
pub unsafe extern "C" fn snet_read_vq_state(snet: *mut snet, idx: u16, state: *mut vdpa_vq_state) -> c_int {
    int snet_read_vq_state(struct snet *snet, u16 idx, struct vdpa_vq_state *state)
    {
    return snet_ctrl_read_from_dpu(snet, SNET_CTRL_OP_READ_VQ_STATE, idx, state,
    sizeof(*state));
    }
#[no_mangle]
pub unsafe extern "C" fn snet_suspend_dev(snet: *mut snet) -> c_int {
    int snet_suspend_dev(struct snet *snet)
    {
    return snet_send_ctrl_msg(snet, SNET_CTRL_OP_SUSPEND, 0);
    }
#[no_mangle]
pub unsafe extern "C" fn snet_resume_dev(snet: *mut snet) -> c_int {
    int snet_resume_dev(struct snet *snet)
    {
    return snet_send_ctrl_msg(snet, SNET_CTRL_OP_RESUME, 0);
    }
