//! Automatically rewritten from C to Rust
//! Source: drivers/fpga/socfpga-a10.c
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
// FPGA Manager Driver for Altera Arria10 SoCFPGA
//
// Copyright (C) 2015-2016 Altera Corporation
//

pub const A10_FPGAMGR_DCLKCNT_OFST: c_uint = 0x08;
pub const A10_FPGAMGR_DCLKSTAT_OFST: c_uint = 0x0c;
pub const A10_FPGAMGR_IMGCFG_CTL_00_OFST: c_uint = 0x70;
pub const A10_FPGAMGR_IMGCFG_CTL_01_OFST: c_uint = 0x74;
pub const A10_FPGAMGR_IMGCFG_CTL_02_OFST: c_uint = 0x78;
pub const A10_FPGAMGR_IMGCFG_STAT_OFST: c_uint = 0x80;

pub const A10_FPGAMGR_IMGCFG_CTL_02_CDRATIO_SHIFT: c_int = 16;

pub const A10_FPGAMGR_IMGCFG_CTL_02_CFGWIDTH_SHIFT: c_int = 24;

pub const A10_FPGAMGR_IMGCFG_STAT_F2S_MSEL_SHIFT: c_int = 16;
// FPGA CD Ratio Value
pub const CDRATIO_x1: c_uint = 0x0;
pub const CDRATIO_x2: c_uint = 0x1;
pub const CDRATIO_x4: c_uint = 0x2;
pub const CDRATIO_x8: c_uint = 0x3;
// Configuration width 16/32 bit
pub const CFGWDTH_32: c_int = 1;
pub const CFGWDTH_16: c_int = 0;
//
// struct a10_fpga_priv - private data for fpga manager
// @regmap: regmap for register access
// @fpga_data_addr: iomap for single address data register to FPGA
// @clk: clock
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct a10_fpga_priv {
    pub regmap: *mut regmap,
    pub fpga_data_addr: *mut void __iomem,
    pub clk: *mut clk,
}

#[no_mangle]
unsafe extern "C" fn socfpga_a10_fpga_writeable_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool socfpga_a10_fpga_writeable_reg(struct device *dev, unsigned int reg)
    {
    switch (reg) {
    case A10_FPGAMGR_DCLKCNT_OFST:
    case A10_FPGAMGR_DCLKSTAT_OFST:
    case A10_FPGAMGR_IMGCFG_CTL_00_OFST:
    case A10_FPGAMGR_IMGCFG_CTL_01_OFST:
    case A10_FPGAMGR_IMGCFG_CTL_02_OFST:
    return true;
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn socfpga_a10_fpga_readable_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool socfpga_a10_fpga_readable_reg(struct device *dev, unsigned int reg)
    {
    switch (reg) {
    case A10_FPGAMGR_DCLKCNT_OFST:
    case A10_FPGAMGR_DCLKSTAT_OFST:
    case A10_FPGAMGR_IMGCFG_CTL_00_OFST:
    case A10_FPGAMGR_IMGCFG_CTL_01_OFST:
    case A10_FPGAMGR_IMGCFG_CTL_02_OFST:
    case A10_FPGAMGR_IMGCFG_STAT_OFST:
    return true;
    }
    return false;
    }
    static const struct regmap_config socfpga_a10_fpga_regmap_config = {
    .reg_bits = 32,
    .reg_stride = 4,
    .val_bits = 32,
    .writeable_reg = socfpga_a10_fpga_writeable_reg,
    .readable_reg = socfpga_a10_fpga_readable_reg,
    .max_register = A10_FPGAMGR_IMGCFG_STAT_OFST,
    .cache_type = REGCACHE_NONE,
    };
//
// from the register map description of cdratio in imgcfg_ctrl_02:
// Normal Configuration    : 32bit Passive Parallel
// Partial Reconfiguration : 16bit Passive Parallel
//
    static void socfpga_a10_fpga_set_cfg_width(struct a10_fpga_priv *priv,
    int width)
    {
    width <<= A10_FPGAMGR_IMGCFG_CTL_02_CFGWIDTH_SHIFT;
    regmap_update_bits(priv.regmap, A10_FPGAMGR_IMGCFG_CTL_02_OFST,
    A10_FPGAMGR_IMGCFG_CTL_02_CFGWIDTH, width);
    }
    static void socfpga_a10_fpga_generate_dclks(struct a10_fpga_priv *priv,
    u32 count)
    {
    u32 val;
// Clear any existing DONE status.
    regmap_write(priv.regmap, A10_FPGAMGR_DCLKSTAT_OFST,
    A10_FPGAMGR_DCLKSTAT_DCLKDONE);
// Issue the DCLK regmap.
    regmap_write(priv.regmap, A10_FPGAMGR_DCLKCNT_OFST, count);
// wait till the dclkcnt done
    regmap_read_poll_timeout(priv.regmap, A10_FPGAMGR_DCLKSTAT_OFST, val,
    val, 1, 100);
// Clear DONE status.
    regmap_write(priv.regmap, A10_FPGAMGR_DCLKSTAT_OFST,
    A10_FPGAMGR_DCLKSTAT_DCLKDONE);
    }
pub const RBF_ENCRYPTION_MODE_OFFSET: c_int = 69;
pub const RBF_DECOMPRESS_OFFSET: c_int = 229;
#[no_mangle]
unsafe extern "C" fn socfpga_a10_fpga_encrypted(buf32: *mut u32, buf32_size: usize) -> c_int {
    static int socfpga_a10_fpga_encrypted(u32 *buf32, size_t buf32_size)
    {
    if (buf32_size < RBF_ENCRYPTION_MODE_OFFSET + 1)
    return -EINVAL;
// Is the bitstream encrypted?
    return ((buf32[RBF_ENCRYPTION_MODE_OFFSET] >> 2) & 3) != 0;
    }
#[no_mangle]
unsafe extern "C" fn socfpga_a10_fpga_compressed(buf32: *mut u32, buf32_size: usize) -> c_int {
    static int socfpga_a10_fpga_compressed(u32 *buf32, size_t buf32_size)
    {
    if (buf32_size < RBF_DECOMPRESS_OFFSET + 1)
    return -EINVAL;
// Is the bitstream compressed?
    return !((buf32[RBF_DECOMPRESS_OFFSET] >> 1) & 1);
    }
    static unsigned int socfpga_a10_fpga_get_cd_ratio(unsigned int cfg_width,
    bool encrypt, bool compress)
    {
    unsigned int cd_ratio;
//
// cd ratio is dependent on cfg width and whether the bitstream
// is encrypted and/or compressed.
//
// | width | encr. | compr. | cd ratio |
// |  16   |   0   |   0    |     1    |
// |  16   |   0   |   1    |     4    |
// |  16   |   1   |   0    |     2    |
// |  16   |   1   |   1    |     4    |
// |  32   |   0   |   0    |     1    |
// |  32   |   0   |   1    |     8    |
// |  32   |   1   |   0    |     4    |
// |  32   |   1   |   1    |     8    |
//
    if (!compress && !encrypt)
    return CDRATIO_x1;
    if (compress)
    cd_ratio = CDRATIO_x4;
    else
    cd_ratio = CDRATIO_x2;
// If 32 bit, double the cd ratio by incrementing the field
    if (cfg_width == CFGWDTH_32)
    cd_ratio += 1;
    return cd_ratio;
    }
    static int socfpga_a10_fpga_set_cdratio(struct fpga_manager *mgr,
    unsigned int cfg_width,
    const char *buf, size_t count)
    {
    struct a10_fpga_priv *priv = mgr.priv;
    unsigned int cd_ratio;
    int encrypt, compress;
    encrypt = socfpga_a10_fpga_encrypted((u32 *)buf, count / 4);
    if (encrypt < 0)
    return -EINVAL;
    compress = socfpga_a10_fpga_compressed((u32 *)buf, count / 4);
    if (compress < 0)
    return -EINVAL;
    cd_ratio = socfpga_a10_fpga_get_cd_ratio(cfg_width, encrypt, compress);
    regmap_update_bits(priv.regmap, A10_FPGAMGR_IMGCFG_CTL_02_OFST,
    A10_FPGAMGR_IMGCFG_CTL_02_CDRATIO_MASK,
    cd_ratio << A10_FPGAMGR_IMGCFG_CTL_02_CDRATIO_SHIFT);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn socfpga_a10_fpga_read_stat(priv: *mut a10_fpga_priv) -> u32 {
    static u32 socfpga_a10_fpga_read_stat(struct a10_fpga_priv *priv)
    {
    u32 val;
    regmap_read(priv.regmap, A10_FPGAMGR_IMGCFG_STAT_OFST, &val);
    return val;
    }
#[no_mangle]
unsafe extern "C" fn socfpga_a10_fpga_wait_for_pr_ready(priv: *mut a10_fpga_priv) -> c_int {
    static int socfpga_a10_fpga_wait_for_pr_ready(struct a10_fpga_priv *priv)
    {
    u32 reg, i;
    for (i = 0; i < 10 ; i++) {
    reg = socfpga_a10_fpga_read_stat(priv);
    if (reg & A10_FPGAMGR_IMGCFG_STAT_F2S_PR_ERROR)
    return -EINVAL;
    if (reg & A10_FPGAMGR_IMGCFG_STAT_F2S_PR_READY)
    return 0;
    }
    return -ETIMEDOUT;
    }
#[no_mangle]
unsafe extern "C" fn socfpga_a10_fpga_wait_for_pr_done(priv: *mut a10_fpga_priv) -> c_int {
    static int socfpga_a10_fpga_wait_for_pr_done(struct a10_fpga_priv *priv)
    {
    u32 reg, i;
    for (i = 0; i < 10 ; i++) {
    reg = socfpga_a10_fpga_read_stat(priv);
    if (reg & A10_FPGAMGR_IMGCFG_STAT_F2S_PR_ERROR)
    return -EINVAL;
    if (reg & A10_FPGAMGR_IMGCFG_STAT_F2S_PR_DONE)
    return 0;
    }
    return -ETIMEDOUT;
    }
// Start the FPGA programming by initialize the FPGA Manager
    static int socfpga_a10_fpga_write_init(struct fpga_manager *mgr,
    struct fpga_image_info *info,
    const char *buf, size_t count)
    {
    struct a10_fpga_priv *priv = mgr.priv;
    unsigned int cfg_width;
    u32 msel, stat, mask;
    int ret;
    if (info.flags & FPGA_MGR_PARTIAL_RECONFIG)
    cfg_width = CFGWDTH_16;
    else
    return -EINVAL;
// Check for passive parallel (msel == 000 or 001)
    msel = socfpga_a10_fpga_read_stat(priv);
    msel &= A10_FPGAMGR_IMGCFG_STAT_F2S_MSEL_MASK;
    msel >>= A10_FPGAMGR_IMGCFG_STAT_F2S_MSEL_SHIFT;
    if ((msel != 0) && (msel != 1)) {
    dev_dbg(&mgr.dev, "Fail: invalid msel=%d\n", msel);
    return -EINVAL;
    }
// Make sure no external devices are interfering
    stat = socfpga_a10_fpga_read_stat(priv);
    mask = A10_FPGAMGR_IMGCFG_STAT_F2S_NCONFIG_PIN |
    A10_FPGAMGR_IMGCFG_STAT_F2S_NSTATUS_PIN;
    if ((stat & mask) != mask)
    return -EINVAL;
// Set cfg width
    socfpga_a10_fpga_set_cfg_width(priv, cfg_width);
// Determine cd ratio from bitstream header and set cd ratio
    ret = socfpga_a10_fpga_set_cdratio(mgr, cfg_width, buf, count);
    if (ret)
    return ret;
//
// Clear s2f_nce to enable chip select.  Leave pr_request
// unasserted and override disabled.
//
    regmap_write(priv.regmap, A10_FPGAMGR_IMGCFG_CTL_01_OFST,
    A10_FPGAMGR_IMGCFG_CTL_01_S2F_NENABLE_CONFIG);
// Set cfg_ctrl to enable s2f dclk and data
    regmap_update_bits(priv.regmap, A10_FPGAMGR_IMGCFG_CTL_02_OFST,
    A10_FPGAMGR_IMGCFG_CTL_02_EN_CFG_CTRL,
    A10_FPGAMGR_IMGCFG_CTL_02_EN_CFG_CTRL);
//
// Disable overrides not needed for pr.
// s2f_config==1 leaves reset deasseted.
//
    regmap_write(priv.regmap, A10_FPGAMGR_IMGCFG_CTL_00_OFST,
    A10_FPGAMGR_IMGCFG_CTL_00_S2F_NENABLE_NCONFIG |
    A10_FPGAMGR_IMGCFG_CTL_00_S2F_NENABLE_NSTATUS |
    A10_FPGAMGR_IMGCFG_CTL_00_S2F_NENABLE_CONDONE |
    A10_FPGAMGR_IMGCFG_CTL_00_S2F_NCONFIG);
// Enable override for data, dclk, nce, and pr_request to CSS
    regmap_update_bits(priv.regmap, A10_FPGAMGR_IMGCFG_CTL_01_OFST,
    A10_FPGAMGR_IMGCFG_CTL_01_S2F_NENABLE_CONFIG, 0);
// Send some clocks to clear out any errors
    socfpga_a10_fpga_generate_dclks(priv, 256);
// Assert pr_request
    regmap_update_bits(priv.regmap, A10_FPGAMGR_IMGCFG_CTL_01_OFST,
    A10_FPGAMGR_IMGCFG_CTL_01_S2F_PR_REQUEST,
    A10_FPGAMGR_IMGCFG_CTL_01_S2F_PR_REQUEST);
// Provide 2048 DCLKs before starting the config data streaming.
    socfpga_a10_fpga_generate_dclks(priv, 0x7ff);
// Wait for pr_ready
    return socfpga_a10_fpga_wait_for_pr_ready(priv);
    }
//
// write data to the FPGA data register
//
    static int socfpga_a10_fpga_write(struct fpga_manager *mgr, const char *buf,
    size_t count)
    {
    struct a10_fpga_priv *priv = mgr.priv;
    u32 *buffer_32 = (u32 *)buf;
    let mut i: usize = 0;
    if (count <= 0)
    return -EINVAL;
// Write out the complete 32-bit chunks
    while (count >= sizeof(u32)) {
    writel(buffer_32[i++], priv.fpga_data_addr);
    count -= sizeof(u32);
    }
// Write out remaining non 32-bit chunks
    switch (count) {
    case 3:
    writel(buffer_32[i++] & 0x00ffffff, priv.fpga_data_addr);
    break;
    case 2:
    writel(buffer_32[i++] & 0x0000ffff, priv.fpga_data_addr);
    break;
    case 1:
    writel(buffer_32[i++] & 0x000000ff, priv.fpga_data_addr);
    break;
    case 0:
    break;
    default:
// This will never happen
    return -EFAULT;
    }
    return 0;
    }
    static int socfpga_a10_fpga_write_complete(struct fpga_manager *mgr,
    struct fpga_image_info *info)
    {
    struct a10_fpga_priv *priv = mgr.priv;
    u32 reg;
    int ret;
// Wait for pr_done
    ret = socfpga_a10_fpga_wait_for_pr_done(priv);
// Clear pr_request
    regmap_update_bits(priv.regmap, A10_FPGAMGR_IMGCFG_CTL_01_OFST,
    A10_FPGAMGR_IMGCFG_CTL_01_S2F_PR_REQUEST, 0);
// Send some clocks to clear out any errors
    socfpga_a10_fpga_generate_dclks(priv, 256);
// Disable s2f dclk and data
    regmap_update_bits(priv.regmap, A10_FPGAMGR_IMGCFG_CTL_02_OFST,
    A10_FPGAMGR_IMGCFG_CTL_02_EN_CFG_CTRL, 0);
// Deassert chip select
    regmap_update_bits(priv.regmap, A10_FPGAMGR_IMGCFG_CTL_01_OFST,
    A10_FPGAMGR_IMGCFG_CTL_01_S2F_NCE,
    A10_FPGAMGR_IMGCFG_CTL_01_S2F_NCE);
// Disable data, dclk, nce, and pr_request override to CSS
    regmap_update_bits(priv.regmap, A10_FPGAMGR_IMGCFG_CTL_01_OFST,
    A10_FPGAMGR_IMGCFG_CTL_01_S2F_NENABLE_CONFIG,
    A10_FPGAMGR_IMGCFG_CTL_01_S2F_NENABLE_CONFIG);
// Return any errors regarding pr_done or pr_error
    if (ret)
    return ret;
// Final check
    reg = socfpga_a10_fpga_read_stat(priv);
    if (((reg & A10_FPGAMGR_IMGCFG_STAT_F2S_USERMODE) == 0) ||
    ((reg & A10_FPGAMGR_IMGCFG_STAT_F2S_CONDONE_PIN) == 0) ||
    ((reg & A10_FPGAMGR_IMGCFG_STAT_F2S_NSTATUS_PIN) == 0)) {
    dev_dbg(&mgr.dev,
    "Timeout in final check. Status=%08xf\n", reg);
    return -ETIMEDOUT;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn socfpga_a10_fpga_state(mgr: *mut fpga_manager) -> enum fpga_mgr_states {
    static enum fpga_mgr_states socfpga_a10_fpga_state(struct fpga_manager *mgr)
    {
    struct a10_fpga_priv *priv = mgr.priv;
    let mut reg: u32 = socfpga_a10_fpga_read_stat(priv);
    if (reg & A10_FPGAMGR_IMGCFG_STAT_F2S_USERMODE)
    return FPGA_MGR_STATE_OPERATING;
    if (reg & A10_FPGAMGR_IMGCFG_STAT_F2S_PR_READY)
    return FPGA_MGR_STATE_WRITE;
    if (reg & A10_FPGAMGR_IMGCFG_STAT_F2S_CRC_ERROR)
    return FPGA_MGR_STATE_WRITE_COMPLETE_ERR;
    if ((reg & A10_FPGAMGR_IMGCFG_STAT_F2S_NSTATUS_PIN) == 0)
    return FPGA_MGR_STATE_RESET;
    return FPGA_MGR_STATE_UNKNOWN;
    }
    static const struct fpga_manager_ops socfpga_a10_fpga_mgr_ops = {
    .initial_header_size = (RBF_DECOMPRESS_OFFSET + 1) * 4,
    .state = socfpga_a10_fpga_state,
    .write_init = socfpga_a10_fpga_write_init,
    .write = socfpga_a10_fpga_write,
    .write_complete = socfpga_a10_fpga_write_complete,
    };
#[no_mangle]
unsafe extern "C" fn socfpga_a10_fpga_probe(pdev: *mut platform_device) -> c_int {
    static int socfpga_a10_fpga_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct a10_fpga_priv *priv;
    void __iomem *reg_base;
    struct fpga_manager *mgr;
    int ret;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
// First mmio base is for register access
    reg_base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(reg_base))
    return PTR_ERR(reg_base);
// Second mmio base is for writing FPGA image data
    priv.fpga_data_addr = devm_platform_ioremap_resource(pdev, 1);
    if (IS_ERR(priv.fpga_data_addr))
    return PTR_ERR(priv.fpga_data_addr);
// regmap for register access
    priv.regmap = devm_regmap_init_mmio(dev, reg_base,
    &socfpga_a10_fpga_regmap_config);
    if (IS_ERR(priv.regmap))
    return -ENODEV;
    priv.clk = devm_clk_get(dev, core::ptr::null_mut());
    if (IS_ERR(priv.clk)) {
    dev_err(dev, "no clock specified\n");
    return PTR_ERR(priv.clk);
    }
    ret = clk_prepare_enable(priv.clk);
    if (ret) {
    dev_err(dev, "could not enable clock\n");
    return -EBUSY;
    }
    mgr = fpga_mgr_register(dev, "SoCFPGA Arria10 FPGA Manager",
    &socfpga_a10_fpga_mgr_ops, priv);
    if (IS_ERR(mgr)) {
    clk_disable_unprepare(priv.clk);
    return PTR_ERR(mgr);
    }
    platform_set_drvdata(pdev, mgr);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn socfpga_a10_fpga_remove(pdev: *mut platform_device) {
    static void socfpga_a10_fpga_remove(struct platform_device *pdev)
    {
    struct fpga_manager *mgr = platform_get_drvdata(pdev);
    struct a10_fpga_priv *priv = mgr.priv;
    fpga_mgr_unregister(mgr);
    clk_disable_unprepare(priv.clk);
    }
    static const struct of_device_id socfpga_a10_fpga_of_match[] = {
    { .compatible = "altr,socfpga-a10-fpga-mgr", },
    {},
    };
    MODULE_DEVICE_TABLE(of, socfpga_a10_fpga_of_match);
    static struct platform_driver socfpga_a10_fpga_driver = {
    .probe = socfpga_a10_fpga_probe,
    .remove = socfpga_a10_fpga_remove,
    .driver = {
    .name	= "socfpga_a10_fpga_manager",
    .of_match_table = socfpga_a10_fpga_of_match,
    },
    };
    module_platform_driver(socfpga_a10_fpga_driver);
    MODULE_AUTHOR("Alan Tull <atull@opensource.altera.com>");
    MODULE_DESCRIPTION("SoCFPGA Arria10 FPGA Manager");
    MODULE_LICENSE("GPL v2");
