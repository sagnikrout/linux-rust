//! Automatically rewritten from C to Rust
//! Source: drivers/edac/qcom_edac.c
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
//
// Copyright (c) 2018, The Linux Foundation. All rights reserved.
//

pub const LLCC_ERP_PANIC_ON_UE: c_int = 1;
pub const TRP_SYN_REG_CNT: c_int = 6;
pub const DRP_SYN_REG_CNT: c_int = 8;

pub const LLCC_LB_CNT_SHIFT: c_int = 28;
// Mask and shift macros

pub const SB_ERROR_THRESHOLD: c_uint = 0x1;
pub const SB_ERROR_THRESHOLD_SHIFT: c_int = 24;
pub const SB_DB_TRP_INTERRUPT_ENABLE: c_uint = 0x3;
pub const TRP0_INTERRUPT_ENABLE: c_uint = 0x1;

pub const SB_DB_DRP_INTERRUPT_ENABLE: c_uint = 0x3;
pub const ECC_POLL_MSEC: c_int = 5000;
    enum {
    LLCC_DRAM_CE = 0,
    LLCC_DRAM_UE,
    LLCC_TRAM_CE,
    LLCC_TRAM_UE,
    };
    static const struct llcc_edac_reg_data edac_reg_data[] = {
    [LLCC_DRAM_CE] = {
    .name = "DRAM Single-bit",
    .reg_cnt = DRP_SYN_REG_CNT,
    .count_mask = ECC_SB_ERR_COUNT_MASK,
    .ways_mask = ECC_SB_ERR_WAYS_MASK,
    .count_shift = ECC_SB_ERR_COUNT_SHIFT,
    },
    [LLCC_DRAM_UE] = {
    .name = "DRAM Double-bit",
    .reg_cnt = DRP_SYN_REG_CNT,
    .count_mask = ECC_DB_ERR_COUNT_MASK,
    .ways_mask = ECC_DB_ERR_WAYS_MASK,
    .ways_shift = ECC_DB_ERR_WAYS_SHIFT,
    },
    [LLCC_TRAM_CE] = {
    .name = "TRAM Single-bit",
    .reg_cnt = TRP_SYN_REG_CNT,
    .count_mask = ECC_SB_ERR_COUNT_MASK,
    .ways_mask = ECC_SB_ERR_WAYS_MASK,
    .count_shift = ECC_SB_ERR_COUNT_SHIFT,
    },
    [LLCC_TRAM_UE] = {
    .name = "TRAM Double-bit",
    .reg_cnt = TRP_SYN_REG_CNT,
    .count_mask = ECC_DB_ERR_COUNT_MASK,
    .ways_mask = ECC_DB_ERR_WAYS_MASK,
    .ways_shift = ECC_DB_ERR_WAYS_SHIFT,
    },
    };
#[no_mangle]
unsafe extern "C" fn qcom_llcc_core_setup(drv: *mut llcc_drv_data, llcc_bcast_regmap: *mut regmap) -> c_int {
    static int qcom_llcc_core_setup(struct llcc_drv_data *drv, struct regmap *llcc_bcast_regmap)
    {
    u32 sb_err_threshold;
    int ret;
//
// Configure interrupt enable registers such that Tag, Data RAM related
// interrupts are propagated to interrupt controller for servicing
//
    ret = regmap_update_bits(llcc_bcast_regmap, drv.edac_reg_offset.cmn_interrupt_0_enable,
    TRP0_INTERRUPT_ENABLE,
    TRP0_INTERRUPT_ENABLE);
    if (ret)
    return ret;
    ret = regmap_update_bits(llcc_bcast_regmap, drv.edac_reg_offset.trp_interrupt_0_enable,
    SB_DB_TRP_INTERRUPT_ENABLE,
    SB_DB_TRP_INTERRUPT_ENABLE);
    if (ret)
    return ret;
    sb_err_threshold = (SB_ERROR_THRESHOLD << SB_ERROR_THRESHOLD_SHIFT);
    ret = regmap_write(llcc_bcast_regmap, drv.edac_reg_offset.drp_ecc_error_cfg,
    sb_err_threshold);
    if (ret)
    return ret;
    ret = regmap_update_bits(llcc_bcast_regmap, drv.edac_reg_offset.cmn_interrupt_0_enable,
    DRP0_INTERRUPT_ENABLE,
    DRP0_INTERRUPT_ENABLE);
    if (ret)
    return ret;
    ret = regmap_write(llcc_bcast_regmap, drv.edac_reg_offset.drp_interrupt_enable,
    SB_DB_DRP_INTERRUPT_ENABLE);
    return ret;
    }
// Clear the error interrupt and counter registers
    static int
    qcom_llcc_clear_error_status(int err_type, struct llcc_drv_data *drv)
    {
    int ret;
    switch (err_type) {
    case LLCC_DRAM_CE:
    case LLCC_DRAM_UE:
    ret = regmap_write(drv.bcast_regmap,
    drv.edac_reg_offset.drp_interrupt_clear,
    DRP_TRP_INT_CLEAR);
    if (ret)
    return ret;
    ret = regmap_write(drv.bcast_regmap,
    drv.edac_reg_offset.drp_ecc_error_cntr_clear,
    DRP_TRP_CNT_CLEAR);
    if (ret)
    return ret;
    break;
    case LLCC_TRAM_CE:
    case LLCC_TRAM_UE:
    ret = regmap_write(drv.bcast_regmap,
    drv.edac_reg_offset.trp_interrupt_0_clear,
    DRP_TRP_INT_CLEAR);
    if (ret)
    return ret;
    ret = regmap_write(drv.bcast_regmap,
    drv.edac_reg_offset.trp_ecc_error_cntr_clear,
    DRP_TRP_CNT_CLEAR);
    if (ret)
    return ret;
    break;
    default:
    ret = -EINVAL;
    edac_printk(KERN_CRIT, EDAC_LLCC, "Unexpected error type: %d\n",
    err_type);
    }
    return ret;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_llcc_syn_regs {
    pub synd_reg: u32,
    pub count_status_reg: u32,
    pub ways_status_reg: u32,
}

    static void get_reg_offsets(struct llcc_drv_data *drv, int err_type,
    struct qcom_llcc_syn_regs *syn_regs)
    {
    const struct llcc_edac_reg_offset *edac_reg_offset = drv.edac_reg_offset;
    switch (err_type) {
    case LLCC_DRAM_CE:
    syn_regs.synd_reg = edac_reg_offset.drp_ecc_sb_err_syn0;
    syn_regs.count_status_reg = edac_reg_offset.drp_ecc_error_status1;
    syn_regs.ways_status_reg = edac_reg_offset.drp_ecc_error_status0;
    break;
    case LLCC_DRAM_UE:
    syn_regs.synd_reg = edac_reg_offset.drp_ecc_db_err_syn0;
    syn_regs.count_status_reg = edac_reg_offset.drp_ecc_error_status1;
    syn_regs.ways_status_reg = edac_reg_offset.drp_ecc_error_status0;
    break;
    case LLCC_TRAM_CE:
    syn_regs.synd_reg = edac_reg_offset.trp_ecc_sb_err_syn0;
    syn_regs.count_status_reg = edac_reg_offset.trp_ecc_error_status1;
    syn_regs.ways_status_reg = edac_reg_offset.trp_ecc_error_status0;
    break;
    case LLCC_TRAM_UE:
    syn_regs.synd_reg = edac_reg_offset.trp_ecc_db_err_syn0;
    syn_regs.count_status_reg = edac_reg_offset.trp_ecc_error_status1;
    syn_regs.ways_status_reg = edac_reg_offset.trp_ecc_error_status0;
    break;
    }
    }
// Dump Syndrome registers data for Tag RAM, Data RAM bit errors
    static int
    dump_syn_reg_values(struct llcc_drv_data *drv, u32 bank, int err_type)
    {
    let mut reg_data: llcc_edac_reg_data = edac_reg_data[err_type];
    let mut regs: qcom_llcc_syn_regs = { };
    int err_cnt, err_ways, ret, i;
    u32 synd_reg, synd_val;
    get_reg_offsets(drv, err_type, &regs);
    for (i = 0; i < reg_data.reg_cnt; i++) {
    synd_reg = regs.synd_reg + (i * 4);
    ret = regmap_read(drv.regmaps[bank], synd_reg,
    &synd_val);
    if (ret)
    goto clear;
    edac_printk(KERN_CRIT, EDAC_LLCC, "%s: ECC_SYN%d: 0x%8x\n",
    reg_data.name, i, synd_val);
    }
    ret = regmap_read(drv.regmaps[bank], regs.count_status_reg,
    &err_cnt);
    if (ret)
    goto clear;
    err_cnt &= reg_data.count_mask;
    err_cnt >>= reg_data.count_shift;
    edac_printk(KERN_CRIT, EDAC_LLCC, "%s: Error count: 0x%4x\n",
    reg_data.name, err_cnt);
    ret = regmap_read(drv.regmaps[bank], regs.ways_status_reg,
    &err_ways);
    if (ret)
    goto clear;
    err_ways &= reg_data.ways_mask;
    err_ways >>= reg_data.ways_shift;
    edac_printk(KERN_CRIT, EDAC_LLCC, "%s: Error ways: 0x%4x\n",
    reg_data.name, err_ways);
    clear:
    return qcom_llcc_clear_error_status(err_type, drv);
    }
    static int
    dump_syn_reg(struct edac_device_ctl_info *edev_ctl, int err_type, u32 bank)
    {
    struct llcc_drv_data *drv = edev_ctl.dev.platform_data;
    int ret;
    ret = dump_syn_reg_values(drv, bank, err_type);
    if (ret)
    return ret;
    switch (err_type) {
    case LLCC_DRAM_CE:
    edac_device_handle_ce(edev_ctl, 0, bank,
    "LLCC Data RAM correctable Error");
    break;
    case LLCC_DRAM_UE:
    edac_device_handle_ue(edev_ctl, 0, bank,
    "LLCC Data RAM uncorrectable Error");
    break;
    case LLCC_TRAM_CE:
    edac_device_handle_ce(edev_ctl, 0, bank,
    "LLCC Tag RAM correctable Error");
    break;
    case LLCC_TRAM_UE:
    edac_device_handle_ue(edev_ctl, 0, bank,
    "LLCC Tag RAM uncorrectable Error");
    break;
    default:
    ret = -EINVAL;
    edac_printk(KERN_CRIT, EDAC_LLCC, "Unexpected error type: %d\n",
    err_type);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn llcc_ecc_irq_handler(irq: c_int, edev_ctl: *mut c_void) -> irqreturn_t {
    static irqreturn_t llcc_ecc_irq_handler(int irq, void *edev_ctl)
    {
    struct edac_device_ctl_info *edac_dev_ctl = edev_ctl;
    struct llcc_drv_data *drv = edac_dev_ctl.dev.platform_data;
    let mut irq_rc: irqreturn_t = IRQ_NONE;
    u32 drp_error, trp_error, i;
    int ret;
// Iterate over the banks and look for Tag RAM or Data RAM errors
    for (i = 0; i < drv.num_banks; i++) {
    ret = regmap_read(drv.regmaps[i], drv.edac_reg_offset.drp_interrupt_status,
    &drp_error);
    if (!ret && (drp_error & SB_ECC_ERROR)) {
    edac_printk(KERN_CRIT, EDAC_LLCC,
    "Single Bit Error detected in Data RAM\n");
    ret = dump_syn_reg(edev_ctl, LLCC_DRAM_CE, i);
    } else if (!ret && (drp_error & DB_ECC_ERROR)) {
    edac_printk(KERN_CRIT, EDAC_LLCC,
    "Double Bit Error detected in Data RAM\n");
    ret = dump_syn_reg(edev_ctl, LLCC_DRAM_UE, i);
    }
    if (!ret)
    irq_rc = IRQ_HANDLED;
    ret = regmap_read(drv.regmaps[i], drv.edac_reg_offset.trp_interrupt_0_status,
    &trp_error);
    if (!ret && (trp_error & SB_ECC_ERROR)) {
    edac_printk(KERN_CRIT, EDAC_LLCC,
    "Single Bit Error detected in Tag RAM\n");
    ret = dump_syn_reg(edev_ctl, LLCC_TRAM_CE, i);
    } else if (!ret && (trp_error & DB_ECC_ERROR)) {
    edac_printk(KERN_CRIT, EDAC_LLCC,
    "Double Bit Error detected in Tag RAM\n");
    ret = dump_syn_reg(edev_ctl, LLCC_TRAM_UE, i);
    }
    if (!ret)
    irq_rc = IRQ_HANDLED;
    }
    return irq_rc;
    }
#[no_mangle]
unsafe extern "C" fn llcc_ecc_check(edev_ctl: *mut edac_device_ctl_info) {
    static void llcc_ecc_check(struct edac_device_ctl_info *edev_ctl)
    {
    llcc_ecc_irq_handler(0, edev_ctl);
    }
#[no_mangle]
unsafe extern "C" fn qcom_llcc_edac_probe(pdev: *mut platform_device) -> c_int {
    static int qcom_llcc_edac_probe(struct platform_device *pdev)
    {
    struct llcc_drv_data *llcc_driv_data = pdev.dev.platform_data;
    struct edac_device_ctl_info *edev_ctl;
    struct device *dev = &pdev.dev;
    int ecc_irq;
    int rc;
    if (!llcc_driv_data.ecc_irq_configured) {
    rc = qcom_llcc_core_setup(llcc_driv_data, llcc_driv_data.bcast_regmap);
    if (rc)
    return rc;
    }
// Allocate edac control info
    edev_ctl = edac_device_alloc_ctl_info(0, "qcom-llcc", 1, "bank",
    llcc_driv_data.num_banks, 1,
    edac_device_alloc_index());
    if (!edev_ctl)
    return -ENOMEM;
    edev_ctl.dev = dev;
    edev_ctl.mod_name = dev_name(dev);
    edev_ctl.dev_name = dev_name(dev);
    edev_ctl.ctl_name = "llcc";
    edev_ctl.panic_on_ue = LLCC_ERP_PANIC_ON_UE;
// Check if LLCC driver has passed ECC IRQ
    ecc_irq = llcc_driv_data.ecc_irq;
    if (ecc_irq > 0) {
// Use interrupt mode if IRQ is available
    rc = devm_request_irq(dev, ecc_irq, llcc_ecc_irq_handler,
    IRQF_TRIGGER_HIGH, "llcc_ecc", edev_ctl);
    if (!rc) {
    edac_op_state = EDAC_OPSTATE_INT;
    goto irq_done;
    }
    }
// Fall back to polling mode otherwise
    edev_ctl.poll_msec = ECC_POLL_MSEC;
    edev_ctl.edac_check = llcc_ecc_check;
    edac_op_state = EDAC_OPSTATE_POLL;
    irq_done:
    rc = edac_device_add_device(edev_ctl);
    if (rc) {
    edac_device_free_ctl_info(edev_ctl);
    return rc;
    }
    platform_set_drvdata(pdev, edev_ctl);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn qcom_llcc_edac_remove(pdev: *mut platform_device) {
    static void qcom_llcc_edac_remove(struct platform_device *pdev)
    {
    struct edac_device_ctl_info *edev_ctl = dev_get_drvdata(&pdev.dev);
    edac_device_del_device(edev_ctl.dev);
    edac_device_free_ctl_info(edev_ctl);
    }
    static const struct platform_device_id qcom_llcc_edac_id_table[] = {
    { .name = "qcom_llcc_edac" },
    {}
    };
    MODULE_DEVICE_TABLE(platform, qcom_llcc_edac_id_table);
    static struct platform_driver qcom_llcc_edac_driver = {
    .probe = qcom_llcc_edac_probe,
    .remove = qcom_llcc_edac_remove,
    .driver = {
    .name = "qcom_llcc_edac",
    },
    .id_table = qcom_llcc_edac_id_table,
    };
    module_platform_driver(qcom_llcc_edac_driver);
    MODULE_DESCRIPTION("QCOM EDAC driver");
    MODULE_LICENSE("GPL v2");
