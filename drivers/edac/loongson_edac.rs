//! Automatically rewritten from C to Rust
//! Source: drivers/edac/loongson_edac.c
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
// Copyright (C) 2024 Loongson Technology Corporation Limited.
//

pub const ECC_CS_COUNT_REG: c_uint = 0x18;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct loongson_edac_pvt {
    pub ecc_base: *mut void __iomem,
//
// The ECC register in this controller records the number of errors
// encountered since reset and cannot be zeroed so in order to be able
// to report the error count at each check, this records the previous
// register state.
//
    pub last_ce_count: c_int,
}

#[no_mangle]
unsafe extern "C" fn read_ecc(mci: *mut mem_ctl_info) -> c_int {
    static int read_ecc(struct mem_ctl_info *mci)
    {
    struct loongson_edac_pvt *pvt = mci.pvt_info;
    u64 ecc;
    int cs;
    ecc = readq(pvt.ecc_base + ECC_CS_COUNT_REG);
// cs0 -- cs3
    cs = ecc & 0xff;
    cs += (ecc >> 8) & 0xff;
    cs += (ecc >> 16) & 0xff;
    cs += (ecc >> 24) & 0xff;
    return cs;
    }
#[no_mangle]
unsafe extern "C" fn edac_check(mci: *mut mem_ctl_info) {
    static void edac_check(struct mem_ctl_info *mci)
    {
    struct loongson_edac_pvt *pvt = mci.pvt_info;
    int new, add;
    new = read_ecc(mci);
    add = new - pvt.last_ce_count;
    pvt.last_ce_count = new;
    if (add <= 0)
    return;
    edac_mc_handle_error(HW_EVENT_ERR_CORRECTED, mci, add,
    0, 0, 0, 0, 0, -1, "error", "");
    }
#[no_mangle]
unsafe extern "C" fn dimm_config_init(mci: *mut mem_ctl_info) {
    static void dimm_config_init(struct mem_ctl_info *mci)
    {
    struct dimm_info *dimm;
    u32 size, npages;
// size not used
    size = -1;
    npages = MiB_TO_PAGES(size);
    dimm = edac_get_dimm(mci, 0, 0, 0);
    dimm.nr_pages = npages;
    snprintf(dimm.label, sizeof(dimm.label),
    "MC#%uChannel#%u_DIMM#%u", mci.mc_idx, 0, 0);
    dimm.grain = 8;
    }
#[no_mangle]
unsafe extern "C" fn pvt_init(mci: *mut mem_ctl_info, vbase: *mut void __iomem) {
    static void pvt_init(struct mem_ctl_info *mci, void __iomem *vbase)
    {
    struct loongson_edac_pvt *pvt = mci.pvt_info;
    pvt.ecc_base = vbase;
    pvt.last_ce_count = read_ecc(mci);
    }
#[no_mangle]
unsafe extern "C" fn edac_probe(pdev: *mut platform_device) -> c_int {
    static int edac_probe(struct platform_device *pdev)
    {
    struct edac_mc_layer layers[2];
    struct mem_ctl_info *mci;
    void __iomem *vbase;
    int ret;
    vbase = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(vbase))
    return PTR_ERR(vbase);
    layers[0].type = EDAC_MC_LAYER_CHANNEL;
    layers[0].size = 1;
    layers[0].is_virt_csrow = false;
    layers[1].type = EDAC_MC_LAYER_SLOT;
    layers[1].size = 1;
    layers[1].is_virt_csrow = true;
    mci = edac_mc_alloc(0, ARRAY_SIZE(layers), layers,
    sizeof(struct loongson_edac_pvt));
    if (mci == core::ptr::null_mut())
    return -ENOMEM;
    mci.mc_idx = edac_device_alloc_index();
    mci.mtype_cap = MEM_FLAG_RDDR4;
    mci.edac_ctl_cap = EDAC_FLAG_NONE;
    mci.edac_cap = EDAC_FLAG_NONE;
    mci.mod_name = "loongson_edac.c";
    mci.ctl_name = "loongson_edac_ctl";
    mci.dev_name = "loongson_edac_dev";
    mci.ctl_page_to_phys = core::ptr::null_mut();
    mci.pdev = &pdev.dev;
    mci.error_desc.grain = 8;
    mci.edac_check = edac_check;
    pvt_init(mci, vbase);
    dimm_config_init(mci);
    ret = edac_mc_add_mc(mci);
    if (ret) {
    edac_dbg(0, "MC: failed edac_mc_add_mc()\n");
    edac_mc_free(mci);
    return ret;
    }
    edac_op_state = EDAC_OPSTATE_POLL;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn edac_remove(pdev: *mut platform_device) {
    static void edac_remove(struct platform_device *pdev)
    {
    struct mem_ctl_info *mci = edac_mc_del_mc(&pdev.dev);
    if (mci)
    edac_mc_free(mci);
    }
    static const struct acpi_device_id loongson_edac_acpi_match[] = {
    {"LOON0010", 0},
    {}
    };
    MODULE_DEVICE_TABLE(acpi, loongson_edac_acpi_match);
    static struct platform_driver loongson_edac_driver = {
    .probe		= edac_probe,
    .remove		= edac_remove,
    .driver		= {
    .name	= "loongson-mc-edac",
    .acpi_match_table = loongson_edac_acpi_match,
    },
    };
    module_platform_driver(loongson_edac_driver);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Zhao Qunqin <zhaoqunqin@loongson.cn>");
    MODULE_DESCRIPTION("EDAC driver for loongson memory controller");
