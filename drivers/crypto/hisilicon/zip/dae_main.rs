//! Automatically rewritten from C to Rust
//! Source: drivers/crypto/hisilicon/zip/dae_main.c
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
// Copyright (c) 2024 HiSilicon Limited.

// memory
pub const DAE_MEM_START_OFFSET: c_uint = 0x331040;
pub const DAE_MEM_DONE_OFFSET: c_uint = 0x331044;
pub const DAE_MEM_START_MASK: c_uint = 0x1;
pub const DAE_MEM_DONE_MASK: c_uint = 0x1;
pub const DAE_REG_RD_INTVRL_US: c_int = 10;

// error
pub const DAE_AXI_CFG_OFFSET: c_uint = 0x331000;

pub const DAE_ERR_SOURCE_OFFSET: c_uint = 0x331C84;
pub const DAE_ERR_STATUS_OFFSET: c_uint = 0x331C88;
pub const DAE_ERR_CE_OFFSET: c_uint = 0x331CA0;

pub const DAE_ERR_NFE_OFFSET: c_uint = 0x331CA4;
pub const DAE_ERR_NFE_MASK: c_uint = 0x17;
pub const DAE_ERR_FE_OFFSET: c_uint = 0x331CA8;
pub const DAE_ERR_FE_MASK: c_int = 0;

pub const DAE_ECC_INFO_OFFSET: c_uint = 0x33400C;
pub const DAE_ERR_SHUTDOWN_OFFSET: c_uint = 0x331CAC;
pub const DAE_ERR_SHUTDOWN_MASK: c_uint = 0x17;
pub const DAE_ERR_ENABLE_OFFSET: c_uint = 0x331C80;

pub const DAE_AM_CTRL_GLOBAL_OFFSET: c_uint = 0x330000;
pub const DAE_AM_RETURN_OFFSET: c_uint = 0x330150;
pub const DAE_AM_RETURN_MASK: c_uint = 0x3;
pub const DAE_AXI_CFG_OFFSET: c_uint = 0x331000;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hisi_dae_hw_error {
    pub int_msk: u32,
    pub msg: *const c_char,
}

    static const struct hisi_dae_hw_error dae_hw_error[] = {
    { .int_msk = BIT(0), .msg = "dae_axi_bus_err" },
    { .int_msk = BIT(1), .msg = "dae_axi_poison_err" },
    { .int_msk = BIT(2), .msg = "dae_ecc_2bit_err" },
    { .int_msk = BIT(3), .msg = "dae_ecc_1bit_err" },
    { .int_msk = BIT(4), .msg = "dae_fsm_hbeat_err" },
    };
#[no_mangle]
pub unsafe extern "C" fn dae_is_support(qm: *mut hisi_qm) -> bool {
    static inline bool dae_is_support(struct hisi_qm *qm)
    {
    if (test_bit(QM_SUPPORT_DAE, &qm.caps))
    return true;
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn hisi_dae_set_user_domain(qm: *mut hisi_qm) -> c_int {
    int hisi_dae_set_user_domain(struct hisi_qm *qm)
    {
    u32 val;
    int ret;
    if (!dae_is_support(qm))
    return 0;
    val = readl(qm.io_base + DAE_MEM_START_OFFSET);
    val |= DAE_MEM_START_MASK;
    writel(val, qm.io_base + DAE_MEM_START_OFFSET);
    ret = readl_relaxed_poll_timeout(qm.io_base + DAE_MEM_DONE_OFFSET, val,
    val & DAE_MEM_DONE_MASK,
    DAE_REG_RD_INTVRL_US, DAE_REG_RD_TMOUT_US);
    if (ret)
    pci_err(qm.pdev, "failed to init dae memory!\n");
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn hisi_dae_set_alg(qm: *mut hisi_qm) -> c_int {
    int hisi_dae_set_alg(struct hisi_qm *qm)
    {
    const char *alg_name;
    size_t len;
    if (!dae_is_support(qm))
    return 0;
    if (!qm.uacce)
    return 0;
    if (qm.ver >= QM_HW_V5)
    alg_name = DAE_V5_ALG_NAME;
    else
    alg_name = DAE_ALG_NAME;
    len = strlen(qm.uacce.algs);
// A line break may be required
    if (len + strlen(alg_name) + 1 >= QM_DEV_ALG_MAX_LEN) {
    pci_err(qm.pdev, "algorithm name is too long!\n");
    return -EINVAL;
    }
    if (len)
    strcat((char *)qm.uacce.algs, "\n");
    strcat((char *)qm.uacce.algs, alg_name);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hisi_dae_master_ooo_ctrl(qm: *mut hisi_qm, enable: bool) {
    static void hisi_dae_master_ooo_ctrl(struct hisi_qm *qm, bool enable)
    {
    u32 axi_val, err_val;
    axi_val = readl(qm.io_base + DAE_AXI_CFG_OFFSET);
    if (enable) {
    axi_val |= DAE_AXI_SHUTDOWN_MASK;
    err_val = DAE_ERR_SHUTDOWN_MASK;
    } else {
    axi_val &= ~DAE_AXI_SHUTDOWN_MASK;
    err_val = 0;
    }
    writel(axi_val, qm.io_base + DAE_AXI_CFG_OFFSET);
    writel(err_val, qm.io_base + DAE_ERR_SHUTDOWN_OFFSET);
    }
#[no_mangle]
pub unsafe extern "C" fn hisi_dae_hw_error_enable(qm: *mut hisi_qm) {
    void hisi_dae_hw_error_enable(struct hisi_qm *qm)
    {
    if (!dae_is_support(qm))
    return;
// clear dae hw error source if having
    writel(DAE_ERR_ENABLE_MASK, qm.io_base + DAE_ERR_SOURCE_OFFSET);
// configure error type
    writel(DAE_ERR_CE_MASK, qm.io_base + DAE_ERR_CE_OFFSET);
    writel(DAE_ERR_NFE_MASK, qm.io_base + DAE_ERR_NFE_OFFSET);
    writel(DAE_ERR_FE_MASK, qm.io_base + DAE_ERR_FE_OFFSET);
    hisi_dae_master_ooo_ctrl(qm, true);
// enable dae hw error interrupts
    writel(DAE_ERR_ENABLE_MASK, qm.io_base + DAE_ERR_ENABLE_OFFSET);
    }
#[no_mangle]
pub unsafe extern "C" fn hisi_dae_hw_error_disable(qm: *mut hisi_qm) {
    void hisi_dae_hw_error_disable(struct hisi_qm *qm)
    {
    if (!dae_is_support(qm))
    return;
    writel(0, qm.io_base + DAE_ERR_ENABLE_OFFSET);
    hisi_dae_master_ooo_ctrl(qm, false);
    }
#[no_mangle]
unsafe extern "C" fn hisi_dae_get_hw_err_status(qm: *mut hisi_qm) -> u32 {
    static u32 hisi_dae_get_hw_err_status(struct hisi_qm *qm)
    {
    return readl(qm.io_base + DAE_ERR_STATUS_OFFSET);
    }
#[no_mangle]
unsafe extern "C" fn hisi_dae_clear_hw_err_status(qm: *mut hisi_qm, err_sts: u32) {
    static void hisi_dae_clear_hw_err_status(struct hisi_qm *qm, u32 err_sts)
    {
    if (!dae_is_support(qm))
    return;
    writel(err_sts, qm.io_base + DAE_ERR_SOURCE_OFFSET);
    }
#[no_mangle]
unsafe extern "C" fn hisi_dae_disable_error_report(qm: *mut hisi_qm, err_type: u32) {
    static void hisi_dae_disable_error_report(struct hisi_qm *qm, u32 err_type)
    {
    writel(DAE_ERR_NFE_MASK & (~err_type), qm.io_base + DAE_ERR_NFE_OFFSET);
    }
#[no_mangle]
unsafe extern "C" fn hisi_dae_enable_error_report(qm: *mut hisi_qm) {
    static void hisi_dae_enable_error_report(struct hisi_qm *qm)
    {
    writel(DAE_ERR_CE_MASK, qm.io_base + DAE_ERR_CE_OFFSET);
    writel(DAE_ERR_NFE_MASK, qm.io_base + DAE_ERR_NFE_OFFSET);
    }
#[no_mangle]
unsafe extern "C" fn hisi_dae_log_hw_error(qm: *mut hisi_qm, err_type: u32) {
    static void hisi_dae_log_hw_error(struct hisi_qm *qm, u32 err_type)
    {
    const struct hisi_dae_hw_error *err = dae_hw_error;
    struct device *dev = &qm.pdev.dev;
    u32 ecc_info;
    size_t i;
    for (i = 0; i < ARRAY_SIZE(dae_hw_error); i++) {
    err = &dae_hw_error[i];
    if (!(err.int_msk & err_type))
    continue;
    dev_err(dev, "%s [error status=0x%x] found\n",
    err.msg, err.int_msk);
    if (err.int_msk & DAE_ECC_MBIT_MASK) {
    ecc_info = readl(qm.io_base + DAE_ECC_INFO_OFFSET);
    dev_err(dev, "dae multi ecc sram info 0x%x\n", ecc_info);
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn hisi_dae_get_err_result(qm: *mut hisi_qm) -> enum acc_err_result {
    enum acc_err_result hisi_dae_get_err_result(struct hisi_qm *qm)
    {
    u32 err_status;
    if (!dae_is_support(qm))
    return ACC_ERR_NONE;
    err_status = hisi_dae_get_hw_err_status(qm);
    if (!err_status)
    return ACC_ERR_NONE;
    hisi_dae_log_hw_error(qm, err_status);
    if (err_status & DAE_ERR_NFE_MASK) {
// Disable the same error reporting until device is recovered.
    hisi_dae_disable_error_report(qm, err_status);
    return ACC_ERR_NEED_RESET;
    }
    hisi_dae_clear_hw_err_status(qm, err_status);
// Avoid firmware disable error report, re-enable.
    hisi_dae_enable_error_report(qm);
    return ACC_ERR_RECOVERED;
    }
#[no_mangle]
pub unsafe extern "C" fn hisi_dae_dev_is_abnormal(qm: *mut hisi_qm) -> bool {
    bool hisi_dae_dev_is_abnormal(struct hisi_qm *qm)
    {
    u32 err_status;
    if (!dae_is_support(qm))
    return false;
    err_status = hisi_dae_get_hw_err_status(qm);
    if (err_status & DAE_ERR_NFE_MASK)
    return true;
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn hisi_dae_close_axi_master_ooo(qm: *mut hisi_qm) -> c_int {
    int hisi_dae_close_axi_master_ooo(struct hisi_qm *qm)
    {
    u32 val;
    int ret;
    if (!dae_is_support(qm))
    return 0;
    val = readl(qm.io_base + DAE_AM_CTRL_GLOBAL_OFFSET);
    val |= BIT(0);
    writel(val, qm.io_base + DAE_AM_CTRL_GLOBAL_OFFSET);
    ret = readl_relaxed_poll_timeout(qm.io_base + DAE_AM_RETURN_OFFSET,
    val, (val == DAE_AM_RETURN_MASK),
    DAE_REG_RD_INTVRL_US, DAE_REG_RD_TMOUT_US);
    if (ret)
    dev_err(&qm.pdev.dev, "failed to close dae axi ooo!\n");
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn hisi_dae_open_axi_master_ooo(qm: *mut hisi_qm) {
    void hisi_dae_open_axi_master_ooo(struct hisi_qm *qm)
    {
    u32 val;
    if (!dae_is_support(qm))
    return;
    val = readl(qm.io_base + DAE_AXI_CFG_OFFSET);
    writel(val & ~DAE_AXI_SHUTDOWN_EN_MASK, qm.io_base + DAE_AXI_CFG_OFFSET);
    writel(val | DAE_AXI_SHUTDOWN_EN_MASK, qm.io_base + DAE_AXI_CFG_OFFSET);
    }
