//! Automatically rewritten from C to Rust
//! Source: drivers/edac/octeon_edac-lmc.c
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


//
// This file is subject to the terms and conditions of the GNU General Public
// License.  See the file "COPYING" in the main directory of this archive
// for more details.
//
// Copyright (C) 2009 Wind River Systems,
// written by Ralf Baechle <ralf@linux-mips.org>
//
// Copyright (c) 2013 by Cisco Systems, Inc.
// All rights reserved.
//

pub const OCTEON_MAX_MC: c_int = 4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct octeon_lmc_pvt {
    pub inject: c_ulong,
    pub error_type: c_ulong,
    pub dimm: c_ulong,
    pub rank: c_ulong,
    pub bank: c_ulong,
    pub row: c_ulong,
    pub col: c_ulong,
}

#[no_mangle]
unsafe extern "C" fn octeon_lmc_edac_poll(mci: *mut mem_ctl_info) {
    static void octeon_lmc_edac_poll(struct mem_ctl_info *mci)
    {
    union cvmx_lmcx_mem_cfg0 cfg0;
    let mut do_clear: bool = false;
    char msg[64];
    cfg0.u64 = cvmx_read_csr(CVMX_LMCX_MEM_CFG0(mci.mc_idx));
    if (cfg0.s.sec_err || cfg0.s.ded_err) {
    union cvmx_lmcx_fadr fadr;
    fadr.u64 = cvmx_read_csr(CVMX_LMCX_FADR(mci.mc_idx));
    snprintf(msg, sizeof(msg),
    "DIMM %d rank %d bank %d row %d col %d",
    fadr.cn30xx.fdimm, fadr.cn30xx.fbunk,
    fadr.cn30xx.fbank, fadr.cn30xx.frow, fadr.cn30xx.fcol);
    }
    if (cfg0.s.sec_err) {
    edac_mc_handle_error(HW_EVENT_ERR_CORRECTED, mci, 1, 0, 0, 0,
    -1, -1, -1, msg, "");
    cfg0.s.sec_err = -1;	/* Done, re-arm */
    do_clear = true;
    }
    if (cfg0.s.ded_err) {
    edac_mc_handle_error(HW_EVENT_ERR_UNCORRECTED, mci, 1, 0, 0, 0,
    -1, -1, -1, msg, "");
    cfg0.s.ded_err = -1;	/* Done, re-arm */
    do_clear = true;
    }
    if (do_clear)
    cvmx_write_csr(CVMX_LMCX_MEM_CFG0(mci.mc_idx), cfg0.u64);
    }
#[no_mangle]
unsafe extern "C" fn octeon_lmc_edac_poll_o2(mci: *mut mem_ctl_info) {
    static void octeon_lmc_edac_poll_o2(struct mem_ctl_info *mci)
    {
    struct octeon_lmc_pvt *pvt = mci.pvt_info;
    union cvmx_lmcx_int int_reg;
    let mut do_clear: bool = false;
    char msg[64];
    if (!pvt.inject)
    int_reg.u64 = cvmx_read_csr(CVMX_LMCX_INT(mci.mc_idx));
    else {
    int_reg.u64 = 0;
    if (pvt.error_type == 1)
    int_reg.s.sec_err = 1;
    if (pvt.error_type == 2)
    int_reg.s.ded_err = 1;
    }
    if (int_reg.s.sec_err || int_reg.s.ded_err) {
    union cvmx_lmcx_fadr fadr;
    if (likely(!pvt.inject))
    fadr.u64 = cvmx_read_csr(CVMX_LMCX_FADR(mci.mc_idx));
    else {
    fadr.cn61xx.fdimm = pvt.dimm;
    fadr.cn61xx.fbunk = pvt.rank;
    fadr.cn61xx.fbank = pvt.bank;
    fadr.cn61xx.frow = pvt.row;
    fadr.cn61xx.fcol = pvt.col;
    }
    snprintf(msg, sizeof(msg),
    "DIMM %d rank %d bank %d row %d col %d",
    fadr.cn61xx.fdimm, fadr.cn61xx.fbunk,
    fadr.cn61xx.fbank, fadr.cn61xx.frow, fadr.cn61xx.fcol);
    }
    if (int_reg.s.sec_err) {
    edac_mc_handle_error(HW_EVENT_ERR_CORRECTED, mci, 1, 0, 0, 0,
    -1, -1, -1, msg, "");
    int_reg.s.sec_err = -1;	/* Done, re-arm */
    do_clear = true;
    }
    if (int_reg.s.ded_err) {
    edac_mc_handle_error(HW_EVENT_ERR_UNCORRECTED, mci, 1, 0, 0, 0,
    -1, -1, -1, msg, "");
    int_reg.s.ded_err = -1;	/* Done, re-arm */
    do_clear = true;
    }
    if (do_clear) {
    if (likely(!pvt.inject))
    cvmx_write_csr(CVMX_LMCX_INT(mci.mc_idx), int_reg.u64);
    else
    pvt.inject = 0;
    }
    }
// MC SYSFS parts
// Only a couple naming differences per template, so very similar

    static ssize_t octeon_mc_inject_##reg##_show(struct device *dev,	\
    struct device_attribute *attr,		\
    char *data)				\
    {									\
    struct mem_ctl_info *mci = to_mci(dev);				\
    struct octeon_lmc_pvt *pvt = mci.pvt_info;			\
    return sprintf(data, "%016llu\n", (u64)pvt.reg);		\
    }

    static ssize_t octeon_mc_inject_##reg##_store(struct device *dev,	\
    struct device_attribute *attr,		\
    const char *data, size_t count)		\
    {									\
    struct mem_ctl_info *mci = to_mci(dev);				\
    struct octeon_lmc_pvt *pvt = mci.pvt_info;			\
    if (isdigit(*data)) {						\
    if (!kstrtoul(data, 0, &pvt.reg))			\
    return count;					\
    }								\
    return 0;							\
    }
    TEMPLATE_SHOW(inject);
    TEMPLATE_STORE(inject);
    TEMPLATE_SHOW(dimm);
    TEMPLATE_STORE(dimm);
    TEMPLATE_SHOW(bank);
    TEMPLATE_STORE(bank);
    TEMPLATE_SHOW(rank);
    TEMPLATE_STORE(rank);
    TEMPLATE_SHOW(row);
    TEMPLATE_STORE(row);
    TEMPLATE_SHOW(col);
    TEMPLATE_STORE(col);
    static ssize_t octeon_mc_inject_error_type_store(struct device *dev,
    struct device_attribute *attr,
    const char *data,
    size_t count)
    {
    struct mem_ctl_info *mci = to_mci(dev);
    struct octeon_lmc_pvt *pvt = mci.pvt_info;
    if (!strncmp(data, "single", 6))
    pvt.error_type = 1;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strncmp(data, _arg: "double", _arg: 6)) -> else {
    else if (!strncmp(data, "double", 6))
    pvt.error_type = 2;
    return count;
    }
    static ssize_t octeon_mc_inject_error_type_show(struct device *dev,
    struct device_attribute *attr,
    char *data)
    {
    struct mem_ctl_info *mci = to_mci(dev);
    struct octeon_lmc_pvt *pvt = mci.pvt_info;
    if (pvt.error_type == 1)
    return sprintf(data, "single");
#[no_mangle]
pub unsafe extern "C" fn if(2: pvt->error_type ==) -> else {
    else if (pvt.error_type == 2)
    return sprintf(data, "double");
    return 0;
    }
    static DEVICE_ATTR(inject, S_IRUGO | S_IWUSR,
    octeon_mc_inject_inject_show, octeon_mc_inject_inject_store);
    static DEVICE_ATTR(error_type, S_IRUGO | S_IWUSR,
    octeon_mc_inject_error_type_show, octeon_mc_inject_error_type_store);
    static DEVICE_ATTR(dimm, S_IRUGO | S_IWUSR,
    octeon_mc_inject_dimm_show, octeon_mc_inject_dimm_store);
    static DEVICE_ATTR(rank, S_IRUGO | S_IWUSR,
    octeon_mc_inject_rank_show, octeon_mc_inject_rank_store);
    static DEVICE_ATTR(bank, S_IRUGO | S_IWUSR,
    octeon_mc_inject_bank_show, octeon_mc_inject_bank_store);
    static DEVICE_ATTR(row, S_IRUGO | S_IWUSR,
    octeon_mc_inject_row_show, octeon_mc_inject_row_store);
    static DEVICE_ATTR(col, S_IRUGO | S_IWUSR,
    octeon_mc_inject_col_show, octeon_mc_inject_col_store);
    static struct attribute *octeon_dev_attrs[] = {
    &dev_attr_inject.attr,
    &dev_attr_error_type.attr,
    &dev_attr_dimm.attr,
    &dev_attr_rank.attr,
    &dev_attr_bank.attr,
    &dev_attr_row.attr,
    &dev_attr_col.attr,
    core::ptr::null_mut()
    };
    ATTRIBUTE_GROUPS(octeon_dev);
#[no_mangle]
unsafe extern "C" fn octeon_lmc_edac_probe(pdev: *mut platform_device) -> c_int {
    static int octeon_lmc_edac_probe(struct platform_device *pdev)
    {
    struct mem_ctl_info *mci;
    struct edac_mc_layer layers[1];
    let mut mc: c_int = pdev.id;
    opstate_init();
    layers[0].type = EDAC_MC_LAYER_CHANNEL;
    layers[0].size = 1;
    layers[0].is_virt_csrow = false;
    if (OCTEON_IS_OCTEON1PLUS()) {
    union cvmx_lmcx_mem_cfg0 cfg0;
    cfg0.u64 = cvmx_read_csr(CVMX_LMCX_MEM_CFG0(0));
    if (!cfg0.s.ecc_ena) {
    dev_info(&pdev.dev, "Disabled (ECC not enabled)\n");
    return 0;
    }
    mci = edac_mc_alloc(mc, ARRAY_SIZE(layers), layers, sizeof(struct octeon_lmc_pvt));
    if (!mci)
    return -ENXIO;
    mci.pdev = &pdev.dev;
    mci.dev_name = dev_name(&pdev.dev);
    mci.mod_name = "octeon-lmc";
    mci.ctl_name = "octeon-lmc-err";
    mci.edac_check = octeon_lmc_edac_poll;
    if (edac_mc_add_mc_with_groups(mci, octeon_dev_groups)) {
    dev_err(&pdev.dev, "edac_mc_add_mc() failed\n");
    edac_mc_free(mci);
    return -ENXIO;
    }
    cfg0.u64 = cvmx_read_csr(CVMX_LMCX_MEM_CFG0(mc));
    cfg0.s.intr_ded_ena = 0;	/* We poll */
    cfg0.s.intr_sec_ena = 0;
    cvmx_write_csr(CVMX_LMCX_MEM_CFG0(mc), cfg0.u64);
    } else {
// OCTEON II
    union cvmx_lmcx_int_en en;
    union cvmx_lmcx_config config;
    config.u64 = cvmx_read_csr(CVMX_LMCX_CONFIG(0));
    if (!config.s.ecc_ena) {
    dev_info(&pdev.dev, "Disabled (ECC not enabled)\n");
    return 0;
    }
    mci = edac_mc_alloc(mc, ARRAY_SIZE(layers), layers, sizeof(struct octeon_lmc_pvt));
    if (!mci)
    return -ENXIO;
    mci.pdev = &pdev.dev;
    mci.dev_name = dev_name(&pdev.dev);
    mci.mod_name = "octeon-lmc";
    mci.ctl_name = "co_lmc_err";
    mci.edac_check = octeon_lmc_edac_poll_o2;
    if (edac_mc_add_mc_with_groups(mci, octeon_dev_groups)) {
    dev_err(&pdev.dev, "edac_mc_add_mc() failed\n");
    edac_mc_free(mci);
    return -ENXIO;
    }
    en.u64 = cvmx_read_csr(CVMX_LMCX_MEM_CFG0(mc));
    en.s.intr_ded_ena = 0;	/* We poll */
    en.s.intr_sec_ena = 0;
    cvmx_write_csr(CVMX_LMCX_MEM_CFG0(mc), en.u64);
    }
    platform_set_drvdata(pdev, mci);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn octeon_lmc_edac_remove(pdev: *mut platform_device) {
    static void octeon_lmc_edac_remove(struct platform_device *pdev)
    {
    struct mem_ctl_info *mci = platform_get_drvdata(pdev);
    edac_mc_del_mc(&pdev.dev);
    edac_mc_free(mci);
    }
    static struct platform_driver octeon_lmc_edac_driver = {
    .probe = octeon_lmc_edac_probe,
    .remove = octeon_lmc_edac_remove,
    .driver = {
    .name = "octeon_lmc_edac",
    }
    };
    module_platform_driver(octeon_lmc_edac_driver);
    MODULE_DESCRIPTION("Cavium Octeon DRAM Memory Controller (LMC) EDAC driver");
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Ralf Baechle <ralf@linux-mips.org>");
