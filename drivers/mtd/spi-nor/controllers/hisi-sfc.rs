//! Automatically rewritten from C to Rust
//! Source: drivers/mtd/spi-nor/controllers/hisi-sfc.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// HiSilicon FMC SPI NOR flash controller driver
//
// Copyright (c) 2015-2016 HiSilicon Technologies Co., Ltd.
//

// Hardware register offsets and field definitions
pub const FMC_CFG: c_uint = 0x00;

pub const FMC_CFG_OP_MODE_BOOT: c_int = 0;
pub const FMC_CFG_OP_MODE_NORMAL: c_int = 1;

pub const FMC_CFG_FLASH_SEL_MASK: c_uint = 0x6;

pub const FMC_GLOBAL_CFG: c_uint = 0x04;

pub const FMC_SPI_TIMING_CFG: c_uint = 0x08;

pub const CS_HOLD_TIME: c_uint = 0x6;
pub const CS_SETUP_TIME: c_uint = 0x6;
pub const CS_DESELECT_TIME: c_uint = 0xf;
pub const FMC_INT: c_uint = 0x18;

pub const FMC_INT_CLR: c_uint = 0x20;
pub const FMC_CMD: c_uint = 0x24;

pub const FMC_ADDRL: c_uint = 0x2c;
pub const FMC_OP_CFG: c_uint = 0x30;

pub const FMC_DATA_NUM: c_uint = 0x38;

pub const FMC_OP: c_uint = 0x3c;

pub const FMC_DMA_LEN: c_uint = 0x40;

pub const FMC_DMA_SADDR_D0: c_uint = 0x4c;

pub const FMC_OP_DMA: c_uint = 0x68;

pub const FMC_OP_READ: c_uint = 0x0;
pub const FMC_OP_WRITE: c_uint = 0x1;
pub const FMC_WAIT_TIMEOUT: c_int = 1000000;
    enum hifmc_iftype {
    IF_TYPE_STD,
    IF_TYPE_DUAL,
    IF_TYPE_DIO,
    IF_TYPE_QUAD,
    IF_TYPE_QIO,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hifmc_priv {
    pub chipselect: u32,
    pub clkrate: u32,
    pub host: *mut hifmc_host,
}

pub const HIFMC_MAX_CHIP_NUM: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hifmc_host {
    pub dev: *mut device,
    pub lock: mutex,
    pub regbase: *mut void __iomem,
    pub iobase: *mut void __iomem,
    pub clk: *mut clk,
    pub buffer: *mut c_void,
    pub dma_buffer: dma_addr_t,
    pub nor: [*mut spi_nor; HIFMC_MAX_CHIP_NUM],
    pub num_chip: u32,
}

#[no_mangle]
pub unsafe extern "C" fn hisi_spi_nor_wait_op_finish(host: *mut hifmc_host) -> c_int {
    static inline int hisi_spi_nor_wait_op_finish(struct hifmc_host *host)
    {
    u32 reg;
    return readl_poll_timeout(host.regbase + FMC_INT, reg,
    (reg & FMC_INT_OP_DONE), 0, FMC_WAIT_TIMEOUT);
    }
#[no_mangle]
unsafe extern "C" fn hisi_spi_nor_get_if_type(proto: enum spi_nor_protocol) -> c_int {
    static int hisi_spi_nor_get_if_type(enum spi_nor_protocol proto)
    {
    enum hifmc_iftype if_type;
    switch (proto) {
    case SNOR_PROTO_1_1_2:
    if_type = IF_TYPE_DUAL;
    break;
    case SNOR_PROTO_1_2_2:
    if_type = IF_TYPE_DIO;
    break;
    case SNOR_PROTO_1_1_4:
    if_type = IF_TYPE_QUAD;
    break;
    case SNOR_PROTO_1_4_4:
    if_type = IF_TYPE_QIO;
    break;
    case SNOR_PROTO_1_1_1:
    default:
    if_type = IF_TYPE_STD;
    break;
    }
    return if_type;
    }
#[no_mangle]
unsafe extern "C" fn hisi_spi_nor_init(host: *mut hifmc_host) {
    static void hisi_spi_nor_init(struct hifmc_host *host)
    {
    u32 reg;
    reg = TIMING_CFG_TCSH(CS_HOLD_TIME)
    | TIMING_CFG_TCSS(CS_SETUP_TIME)
    | TIMING_CFG_TSHSL(CS_DESELECT_TIME);
    writel(reg, host.regbase + FMC_SPI_TIMING_CFG);
    }
#[no_mangle]
unsafe extern "C" fn hisi_spi_nor_prep(nor: *mut spi_nor) -> c_int {
    static int hisi_spi_nor_prep(struct spi_nor *nor)
    {
    struct hifmc_priv *priv = nor.priv;
    struct hifmc_host *host = priv.host;
    int ret;
    mutex_lock(&host.lock);
    ret = clk_set_rate(host.clk, priv.clkrate);
    if (ret)
    goto out;
    ret = clk_prepare_enable(host.clk);
    if (ret)
    goto out;
    return 0;
    out:
    mutex_unlock(&host.lock);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn hisi_spi_nor_unprep(nor: *mut spi_nor) {
    static void hisi_spi_nor_unprep(struct spi_nor *nor)
    {
    struct hifmc_priv *priv = nor.priv;
    struct hifmc_host *host = priv.host;
    clk_disable_unprepare(host.clk);
    mutex_unlock(&host.lock);
    }
    static int hisi_spi_nor_op_reg(struct spi_nor *nor,
    u8 opcode, size_t len, u8 optype)
    {
    struct hifmc_priv *priv = nor.priv;
    struct hifmc_host *host = priv.host;
    u32 reg;
    reg = FMC_CMD_CMD1(opcode);
    writel(reg, host.regbase + FMC_CMD);
    reg = FMC_DATA_NUM_CNT(len);
    writel(reg, host.regbase + FMC_DATA_NUM);
    reg = OP_CFG_FM_CS(priv.chipselect);
    writel(reg, host.regbase + FMC_OP_CFG);
    writel(0xff, host.regbase + FMC_INT_CLR);
    reg = FMC_OP_CMD1_EN | FMC_OP_REG_OP_START | optype;
    writel(reg, host.regbase + FMC_OP);
    return hisi_spi_nor_wait_op_finish(host);
    }
    static int hisi_spi_nor_read_reg(struct spi_nor *nor, u8 opcode, u8 *buf,
    size_t len)
    {
    struct hifmc_priv *priv = nor.priv;
    struct hifmc_host *host = priv.host;
    int ret;
    ret = hisi_spi_nor_op_reg(nor, opcode, len, FMC_OP_READ_DATA_EN);
    if (ret)
    return ret;
    memcpy_fromio(buf, host.iobase, len);
    return 0;
    }
    static int hisi_spi_nor_write_reg(struct spi_nor *nor, u8 opcode,
    const u8 *buf, size_t len)
    {
    struct hifmc_priv *priv = nor.priv;
    struct hifmc_host *host = priv.host;
    if (len)
    memcpy_toio(host.iobase, buf, len);
    return hisi_spi_nor_op_reg(nor, opcode, len, FMC_OP_WRITE_DATA_EN);
    }
    static int hisi_spi_nor_dma_transfer(struct spi_nor *nor, loff_t start_off,
    dma_addr_t dma_buf, size_t len, u8 op_type)
    {
    struct hifmc_priv *priv = nor.priv;
    struct hifmc_host *host = priv.host;
    let mut if_type: u8 = 0;
    u32 reg;
    reg = readl(host.regbase + FMC_CFG);
    reg &= ~(FMC_CFG_OP_MODE_MASK | SPI_NOR_ADDR_MODE_MASK);
    reg |= FMC_CFG_OP_MODE_NORMAL;
    reg |= (nor.addr_nbytes == 4) ? SPI_NOR_ADDR_MODE_4BYTES
    : SPI_NOR_ADDR_MODE_3BYTES;
    writel(reg, host.regbase + FMC_CFG);
    writel(start_off, host.regbase + FMC_ADDRL);
    writel(dma_buf, host.regbase + FMC_DMA_SADDR_D0);
    writel(FMC_DMA_LEN_SET(len), host.regbase + FMC_DMA_LEN);
    reg = OP_CFG_FM_CS(priv.chipselect);
    if (op_type == FMC_OP_READ)
    if_type = hisi_spi_nor_get_if_type(nor.read_proto);
    else
    if_type = hisi_spi_nor_get_if_type(nor.write_proto);
    reg |= OP_CFG_MEM_IF_TYPE(if_type);
    if (op_type == FMC_OP_READ)
    reg |= OP_CFG_DUMMY_NUM(nor.read_dummy >> 3);
    writel(reg, host.regbase + FMC_OP_CFG);
    writel(0xff, host.regbase + FMC_INT_CLR);
    reg = OP_CTRL_RW_OP(op_type) | OP_CTRL_DMA_OP_READY;
    reg |= (op_type == FMC_OP_READ)
    ? OP_CTRL_RD_OPCODE(nor.read_opcode)
    : OP_CTRL_WR_OPCODE(nor.program_opcode);
    writel(reg, host.regbase + FMC_OP_DMA);
    return hisi_spi_nor_wait_op_finish(host);
    }
    static ssize_t hisi_spi_nor_read(struct spi_nor *nor, loff_t from, size_t len,
    u_char *read_buf)
    {
    struct hifmc_priv *priv = nor.priv;
    struct hifmc_host *host = priv.host;
    size_t offset;
    int ret;
    for (offset = 0; offset < len; offset += HIFMC_DMA_MAX_LEN) {
    let mut trans: usize = min_t(size_t, HIFMC_DMA_MAX_LEN, len - offset);
    ret = hisi_spi_nor_dma_transfer(nor,
    from + offset, host.dma_buffer, trans, FMC_OP_READ);
    if (ret) {
    dev_warn(nor.dev, "DMA read timeout\n");
    return ret;
    }
    memcpy(read_buf + offset, host.buffer, trans);
    }
    return len;
    }
    static ssize_t hisi_spi_nor_write(struct spi_nor *nor, loff_t to,
    size_t len, const u_char *write_buf)
    {
    struct hifmc_priv *priv = nor.priv;
    struct hifmc_host *host = priv.host;
    size_t offset;
    int ret;
    for (offset = 0; offset < len; offset += HIFMC_DMA_MAX_LEN) {
    let mut trans: usize = min_t(size_t, HIFMC_DMA_MAX_LEN, len - offset);
    memcpy(host.buffer, write_buf + offset, trans);
    ret = hisi_spi_nor_dma_transfer(nor,
    to + offset, host.dma_buffer, trans, FMC_OP_WRITE);
    if (ret) {
    dev_warn(nor.dev, "DMA write timeout\n");
    return ret;
    }
    }
    return len;
    }
    static const struct spi_nor_controller_ops hisi_controller_ops = {
    .prepare = hisi_spi_nor_prep,
    .unprepare = hisi_spi_nor_unprep,
    .read_reg = hisi_spi_nor_read_reg,
    .write_reg = hisi_spi_nor_write_reg,
    .read = hisi_spi_nor_read,
    .write = hisi_spi_nor_write,
    };
//
// Get spi flash device information and register it as a mtd device.
//
    static int hisi_spi_nor_register(struct device_node *np,
    struct hifmc_host *host)
    {
    const struct spi_nor_hwcaps hwcaps = {
    .mask = SNOR_HWCAPS_READ |
    SNOR_HWCAPS_READ_FAST |
    SNOR_HWCAPS_READ_1_1_2 |
    SNOR_HWCAPS_READ_1_1_4 |
    SNOR_HWCAPS_PP,
    };
    struct device *dev = host.dev;
    struct spi_nor *nor;
    struct hifmc_priv *priv;
    struct mtd_info *mtd;
    int ret;
    nor = devm_kzalloc(dev, sizeof(*nor), GFP_KERNEL);
    if (!nor)
    return -ENOMEM;
    nor.dev = dev;
    spi_nor_set_flash_node(nor, np);
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    ret = of_property_read_u32(np, "reg", &priv.chipselect);
    if (ret) {
    dev_err(dev, "There's no reg property for %pOF\n",
    np);
    return ret;
    }
    ret = of_property_read_u32(np, "spi-max-frequency",
    &priv.clkrate);
    if (ret) {
    dev_err(dev, "There's no spi-max-frequency property for %pOF\n",
    np);
    return ret;
    }
    priv.host = host;
    nor.priv = priv;
    nor.controller_ops = &hisi_controller_ops;
    ret = spi_nor_scan(nor, core::ptr::null_mut(), &hwcaps);
    if (ret)
    return ret;
    mtd = &nor.mtd;
    mtd.name = np.name;
    ret = mtd_device_register(mtd, core::ptr::null_mut(), 0);
    if (ret)
    return ret;
    host.nor[host.num_chip] = nor;
    host.num_chip++;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hisi_spi_nor_unregister_all(host: *mut hifmc_host) {
    static void hisi_spi_nor_unregister_all(struct hifmc_host *host)
    {
    int i;
    for (i = 0; i < host.num_chip; i++)
    mtd_device_unregister(&host.nor[i].mtd);
    }
#[no_mangle]
unsafe extern "C" fn hisi_spi_nor_register_all(host: *mut hifmc_host) -> c_int {
    static int hisi_spi_nor_register_all(struct hifmc_host *host)
    {
    struct device *dev = host.dev;
    int ret;
    for_each_available_child_of_node_scoped(dev.of_node, np) {
    ret = hisi_spi_nor_register(np, host);
    if (ret)
    goto fail;
    if (host.num_chip == HIFMC_MAX_CHIP_NUM) {
    dev_warn(dev, "Flash device number exceeds the maximum chipselect number\n");
    break;
    }
    }
    return 0;
    fail:
    hisi_spi_nor_unregister_all(host);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn hisi_spi_nor_probe(pdev: *mut platform_device) -> c_int {
    static int hisi_spi_nor_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct hifmc_host *host;
    int ret;
    host = devm_kzalloc(dev, sizeof(*host), GFP_KERNEL);
    if (!host)
    return -ENOMEM;
    platform_set_drvdata(pdev, host);
    host.dev = dev;
    host.regbase = devm_platform_ioremap_resource_byname(pdev, "control");
    if (IS_ERR(host.regbase))
    return PTR_ERR(host.regbase);
    host.iobase = devm_platform_ioremap_resource_byname(pdev, "memory");
    if (IS_ERR(host.iobase))
    return PTR_ERR(host.iobase);
    host.clk = devm_clk_get(dev, core::ptr::null_mut());
    if (IS_ERR(host.clk))
    return PTR_ERR(host.clk);
    ret = dma_set_mask_and_coherent(dev, DMA_BIT_MASK(32));
    if (ret) {
    dev_warn(dev, "Unable to set dma mask\n");
    return ret;
    }
    host.buffer = dmam_alloc_coherent(dev, HIFMC_DMA_MAX_LEN,
    &host.dma_buffer, GFP_KERNEL);
    if (!host.buffer)
    return -ENOMEM;
    ret = clk_prepare_enable(host.clk);
    if (ret)
    return ret;
    mutex_init(&host.lock);
    hisi_spi_nor_init(host);
    ret = hisi_spi_nor_register_all(host);
    if (ret)
    mutex_destroy(&host.lock);
    clk_disable_unprepare(host.clk);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn hisi_spi_nor_remove(pdev: *mut platform_device) {
    static void hisi_spi_nor_remove(struct platform_device *pdev)
    {
    struct hifmc_host *host = platform_get_drvdata(pdev);
    hisi_spi_nor_unregister_all(host);
    mutex_destroy(&host.lock);
    }
    static const struct of_device_id hisi_spi_nor_dt_ids[] = {
    { .compatible = "hisilicon,fmc-spi-nor"},
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, hisi_spi_nor_dt_ids);
    static struct platform_driver hisi_spi_nor_driver = {
    .driver = {
    .name	= "hisi-sfc",
    .of_match_table = hisi_spi_nor_dt_ids,
    },
    .probe	= hisi_spi_nor_probe,
    .remove = hisi_spi_nor_remove,
    };
    module_platform_driver(hisi_spi_nor_driver);
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("HiSilicon SPI Nor Flash Controller Driver");
