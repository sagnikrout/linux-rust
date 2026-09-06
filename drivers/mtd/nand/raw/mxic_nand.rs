//! Automatically rewritten from C to Rust
//! Source: drivers/mtd/nand/raw/mxic_nand.c
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
// Copyright (C) 2019 Macronix International Co., Ltd.
//
// Author:
// Mason Yang <masonccyang@mxic.com.tw>
//

pub const HC_CFG: c_uint = 0x0;

pub const HC_CFG_TYPE_SPI_NOR: c_int = 0;
pub const HC_CFG_TYPE_SPI_NAND: c_int = 1;
pub const HC_CFG_TYPE_SPI_RAM: c_int = 2;
pub const HC_CFG_TYPE_RAW_NAND: c_int = 3;

pub const INT_STS: c_uint = 0x4;
pub const INT_STS_EN: c_uint = 0x8;
pub const INT_SIG_EN: c_uint = 0xc;

pub const HC_EN: c_uint = 0x10;

pub const RXD: c_uint = 0x24;

pub const LRD_CFG: c_uint = 0x44;
pub const LWR_CFG: c_uint = 0x80;
pub const RWW_CFG: c_uint = 0x70;

pub const OP_BUSW_1: c_int = 0;
pub const OP_BUSW_2: c_int = 1;
pub const OP_BUSW_4: c_int = 2;
pub const OP_BUSW_8: c_int = 3;
pub const OCTA_CRC: c_uint = 0x38;

pub const LRD_CTRL: c_uint = 0x48;
pub const RWW_CTRL: c_uint = 0x74;
pub const LWR_CTRL: c_uint = 0x84;

pub const LRD_ADDR: c_uint = 0x4c;
pub const LWR_ADDR: c_uint = 0x88;
pub const LRD_RANGE: c_uint = 0x50;
pub const LWR_RANGE: c_uint = 0x8c;
pub const AXI_SLV_ADDR: c_uint = 0x54;
pub const DMAC_RD_CFG: c_uint = 0x58;
pub const DMAC_WR_CFG: c_uint = 0x94;

pub const DMAC_RD_CNT: c_uint = 0x5c;
pub const DMAC_WR_CNT: c_uint = 0x98;
pub const SDMA_ADDR: c_uint = 0x60;
pub const DMAM_CFG: c_uint = 0x64;

pub const DMAM_CNT: c_uint = 0x68;
pub const LNR_TIMER_TH: c_uint = 0x6c;
pub const RDM_CFG0: c_uint = 0x78;

pub const RDM_CFG1: c_uint = 0x7c;

pub const LWR_SUSP_CTRL: c_uint = 0x90;

pub const DMAS_CTRL: c_uint = 0x9c;

pub const DATA_STROB: c_uint = 0xa0;

pub const GPIO: c_uint = 0xc4;

pub const HC_VER: c_uint = 0xd0;

pub const MXIC_NFC_MAX_CLK_HZ: c_int = 50000000;
pub const IRQ_TIMEOUT: c_int = 1000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mxic_nand_ctlr {
    pub ps_clk: *mut clk,
    pub send_clk: *mut clk,
    pub send_dly_clk: *mut clk,
    pub complete: completion,
    pub regs: *mut void __iomem,
    pub controller: nand_controller,
    pub dev: *mut device,
    pub chip: nand_chip,
}

#[no_mangle]
unsafe extern "C" fn mxic_nfc_clk_enable(nfc: *mut mxic_nand_ctlr) -> c_int {
    static int mxic_nfc_clk_enable(struct mxic_nand_ctlr *nfc)
    {
    int ret;
    ret = clk_prepare_enable(nfc.ps_clk);
    if (ret)
    return ret;
    ret = clk_prepare_enable(nfc.send_clk);
    if (ret)
    goto err_ps_clk;
    ret = clk_prepare_enable(nfc.send_dly_clk);
    if (ret)
    goto err_send_dly_clk;
    return ret;
    err_send_dly_clk:
    clk_disable_unprepare(nfc.send_clk);
    err_ps_clk:
    clk_disable_unprepare(nfc.ps_clk);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mxic_nfc_clk_disable(nfc: *mut mxic_nand_ctlr) {
    static void mxic_nfc_clk_disable(struct mxic_nand_ctlr *nfc)
    {
    clk_disable_unprepare(nfc.send_clk);
    clk_disable_unprepare(nfc.send_dly_clk);
    clk_disable_unprepare(nfc.ps_clk);
    }
#[no_mangle]
unsafe extern "C" fn mxic_nfc_set_input_delay(nfc: *mut mxic_nand_ctlr, idly_code: u8) {
    static void mxic_nfc_set_input_delay(struct mxic_nand_ctlr *nfc, u8 idly_code)
    {
    writel(IDLY_CODE_VAL(0, idly_code) |
    IDLY_CODE_VAL(1, idly_code) |
    IDLY_CODE_VAL(2, idly_code) |
    IDLY_CODE_VAL(3, idly_code),
    nfc.regs + IDLY_CODE(0));
    writel(IDLY_CODE_VAL(4, idly_code) |
    IDLY_CODE_VAL(5, idly_code) |
    IDLY_CODE_VAL(6, idly_code) |
    IDLY_CODE_VAL(7, idly_code),
    nfc.regs + IDLY_CODE(1));
    }
#[no_mangle]
unsafe extern "C" fn mxic_nfc_clk_setup(nfc: *mut mxic_nand_ctlr, freq: c_ulong) -> c_int {
    static int mxic_nfc_clk_setup(struct mxic_nand_ctlr *nfc, unsigned long freq)
    {
    int ret;
    ret = clk_set_rate(nfc.send_clk, freq);
    if (ret)
    return ret;
    ret = clk_set_rate(nfc.send_dly_clk, freq);
    if (ret)
    return ret;
//
// A constant delay range from 0x0 ~ 0x1F for input delay,
// the unit is 78 ps, the max input delay is 2.418 ns.
//
    mxic_nfc_set_input_delay(nfc, 0xf);
//
// Phase degree = 360 * freq * output-delay
// where output-delay is a constant value 1 ns in FPGA.
//
// Get Phase degree = 360 * freq * 1 ns
// = 360 * freq * 1 sec / 1000000000
// = 9 * freq / 25000000
//
    ret = clk_set_phase(nfc.send_dly_clk, 9 * freq / 25000000);
    if (ret)
    return ret;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mxic_nfc_set_freq(nfc: *mut mxic_nand_ctlr, freq: c_ulong) -> c_int {
    static int mxic_nfc_set_freq(struct mxic_nand_ctlr *nfc, unsigned long freq)
    {
    int ret;
    if (freq > MXIC_NFC_MAX_CLK_HZ)
    freq = MXIC_NFC_MAX_CLK_HZ;
    mxic_nfc_clk_disable(nfc);
    ret = mxic_nfc_clk_setup(nfc, freq);
    if (ret)
    return ret;
    ret = mxic_nfc_clk_enable(nfc);
    if (ret)
    return ret;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mxic_nfc_isr(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t mxic_nfc_isr(int irq, void *dev_id)
    {
    struct mxic_nand_ctlr *nfc = dev_id;
    u32 sts;
    sts = readl(nfc.regs + INT_STS);
    if (sts & INT_RDY_PIN)
    complete(&nfc.complete);
    else
    return IRQ_NONE;
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn mxic_nfc_hw_init(nfc: *mut mxic_nand_ctlr) {
    static void mxic_nfc_hw_init(struct mxic_nand_ctlr *nfc)
    {
    writel(HC_CFG_NIO(8) | HC_CFG_TYPE(1, HC_CFG_TYPE_RAW_NAND) |
    HC_CFG_SLV_ACT(0) | HC_CFG_MAN_CS_EN |
    HC_CFG_IDLE_SIO_LVL(1), nfc.regs + HC_CFG);
    writel(INT_STS_ALL, nfc.regs + INT_STS_EN);
    writel(INT_RDY_PIN, nfc.regs + INT_SIG_EN);
    writel(0x0, nfc.regs + ONFI_DIN_CNT(0));
    writel(0, nfc.regs + LRD_CFG);
    writel(0, nfc.regs + LRD_CTRL);
    writel(0x0, nfc.regs + HC_EN);
    }
#[no_mangle]
unsafe extern "C" fn mxic_nfc_cs_enable(nfc: *mut mxic_nand_ctlr) {
    static void mxic_nfc_cs_enable(struct mxic_nand_ctlr *nfc)
    {
    writel(readl(nfc.regs + HC_CFG) | HC_CFG_MAN_CS_EN,
    nfc.regs + HC_CFG);
    writel(HC_CFG_MAN_CS_ASSERT | readl(nfc.regs + HC_CFG),
    nfc.regs + HC_CFG);
    }
#[no_mangle]
unsafe extern "C" fn mxic_nfc_cs_disable(nfc: *mut mxic_nand_ctlr) {
    static void mxic_nfc_cs_disable(struct mxic_nand_ctlr *nfc)
    {
    writel(~HC_CFG_MAN_CS_ASSERT & readl(nfc.regs + HC_CFG),
    nfc.regs + HC_CFG);
    }
#[no_mangle]
unsafe extern "C" fn mxic_nfc_wait_ready(chip: *mut nand_chip) -> c_int {
    static int  mxic_nfc_wait_ready(struct nand_chip *chip)
    {
    struct mxic_nand_ctlr *nfc = nand_get_controller_data(chip);
    int ret;
    ret = wait_for_completion_timeout(&nfc.complete,
    msecs_to_jiffies(IRQ_TIMEOUT));
    if (!ret) {
    dev_err(nfc.dev, "nand device timeout\n");
    return -ETIMEDOUT;
    }
    return 0;
    }
    static int mxic_nfc_data_xfer(struct mxic_nand_ctlr *nfc, const void *txbuf,
    void *rxbuf, unsigned int len)
    {
    let mut pos: c_uint = 0;
    while (pos < len) {
    let mut nbytes: c_uint = len - pos;
    let mut data: u32 = 0xffffffff;
    u32 sts;
    int ret;
    if (nbytes > 4)
    nbytes = 4;
    if (txbuf)
    memcpy(&data, txbuf + pos, nbytes);
    ret = readl_poll_timeout(nfc.regs + INT_STS, sts,
    sts & INT_TX_EMPTY, 0, USEC_PER_SEC);
    if (ret)
    return ret;
    writel(data, nfc.regs + TXD(nbytes % 4));
    ret = readl_poll_timeout(nfc.regs + INT_STS, sts,
    sts & INT_TX_EMPTY, 0, USEC_PER_SEC);
    if (ret)
    return ret;
    ret = readl_poll_timeout(nfc.regs + INT_STS, sts,
    sts & INT_RX_NOT_EMPTY, 0,
    USEC_PER_SEC);
    if (ret)
    return ret;
    data = readl(nfc.regs + RXD);
    if (rxbuf) {
    data >>= (8 * (4 - nbytes));
    memcpy(rxbuf + pos, &data, nbytes);
    }
    if (readl(nfc.regs + INT_STS) & INT_RX_NOT_EMPTY)
    dev_warn(nfc.dev, "RX FIFO not empty\n");
    pos += nbytes;
    }
    return 0;
    }
    static int mxic_nfc_exec_op(struct nand_chip *chip,
    const struct nand_operation *op, bool check_only)
    {
    struct mxic_nand_ctlr *nfc = nand_get_controller_data(chip);
    const struct nand_op_instr *instr = core::ptr::null_mut();
    let mut ret: c_int = 0;
    unsigned int op_id;
    if (check_only)
    return 0;
    mxic_nfc_cs_enable(nfc);
    init_completion(&nfc.complete);
    for (op_id = 0; op_id < op.ninstrs; op_id++) {
    instr = &op.instrs[op_id];
    switch (instr.type) {
    case NAND_OP_CMD_INSTR:
    writel(0, nfc.regs + HC_EN);
    writel(HC_EN_BIT, nfc.regs + HC_EN);
    writel(OP_CMD_BUSW(OP_BUSW_8) |  OP_DUMMY_CYC(0x3F) |
    OP_CMD_BYTES(0), nfc.regs + SS_CTRL(0));
    ret = mxic_nfc_data_xfer(nfc,
    &instr.ctx.cmd.opcode,
    core::ptr::null_mut(), 1);
    break;
    case NAND_OP_ADDR_INSTR:
    writel(OP_ADDR_BUSW(OP_BUSW_8) | OP_DUMMY_CYC(0x3F) |
    OP_ADDR_BYTES(instr.ctx.addr.naddrs),
    nfc.regs + SS_CTRL(0));
    ret = mxic_nfc_data_xfer(nfc,
    instr.ctx.addr.addrs, core::ptr::null_mut(),
    instr.ctx.addr.naddrs);
    break;
    case NAND_OP_DATA_IN_INSTR:
    writel(0x0, nfc.regs + ONFI_DIN_CNT(0));
    writel(OP_DATA_BUSW(OP_BUSW_8) | OP_DUMMY_CYC(0x3F) |
    OP_READ, nfc.regs + SS_CTRL(0));
    ret = mxic_nfc_data_xfer(nfc, core::ptr::null_mut(),
    instr.ctx.data.buf.in,
    instr.ctx.data.len);
    break;
    case NAND_OP_DATA_OUT_INSTR:
    writel(instr.ctx.data.len,
    nfc.regs + ONFI_DIN_CNT(0));
    writel(OP_DATA_BUSW(OP_BUSW_8) | OP_DUMMY_CYC(0x3F),
    nfc.regs + SS_CTRL(0));
    ret = mxic_nfc_data_xfer(nfc,
    instr.ctx.data.buf.out, core::ptr::null_mut(),
    instr.ctx.data.len);
    break;
    case NAND_OP_WAITRDY_INSTR:
    ret = mxic_nfc_wait_ready(chip);
    break;
    }
    }
    mxic_nfc_cs_disable(nfc);
    return ret;
    }
    static int mxic_nfc_setup_interface(struct nand_chip *chip, int chipnr,
    const struct nand_interface_config *conf)
    {
    struct mxic_nand_ctlr *nfc = nand_get_controller_data(chip);
    const struct nand_sdr_timings *sdr;
    unsigned long freq;
    int ret;
    sdr = nand_get_sdr_timings(conf);
    if (IS_ERR(sdr))
    return PTR_ERR(sdr);
    if (chipnr == NAND_DATA_IFACE_CHECK_ONLY)
    return 0;
    freq = NSEC_PER_SEC / (sdr.tRC_min / 1000);
    ret =  mxic_nfc_set_freq(nfc, freq);
    if (ret)
    dev_err(nfc.dev, "set freq:%ld failed\n", freq);
    if (sdr.tRC_min < 30000)
    writel(DATA_STROB_EDO_EN, nfc.regs + DATA_STROB);
    return 0;
    }
    static const struct nand_controller_ops mxic_nand_controller_ops = {
    .exec_op = mxic_nfc_exec_op,
    .setup_interface = mxic_nfc_setup_interface,
    };
#[no_mangle]
unsafe extern "C" fn mxic_nfc_probe(pdev: *mut platform_device) -> c_int {
    static int mxic_nfc_probe(struct platform_device *pdev)
    {
    struct device_node *nand_np, *np = pdev.dev.of_node;
    struct mtd_info *mtd;
    struct mxic_nand_ctlr *nfc;
    struct nand_chip *nand_chip;
    int err;
    int irq;
    nfc = devm_kzalloc(&pdev.dev, sizeof(struct mxic_nand_ctlr),
    GFP_KERNEL);
    if (!nfc)
    return -ENOMEM;
    nfc.ps_clk = devm_clk_get(&pdev.dev, "ps");
    if (IS_ERR(nfc.ps_clk))
    return PTR_ERR(nfc.ps_clk);
    nfc.send_clk = devm_clk_get(&pdev.dev, "send");
    if (IS_ERR(nfc.send_clk))
    return PTR_ERR(nfc.send_clk);
    nfc.send_dly_clk = devm_clk_get(&pdev.dev, "send_dly");
    if (IS_ERR(nfc.send_dly_clk))
    return PTR_ERR(nfc.send_dly_clk);
    nfc.regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(nfc.regs))
    return PTR_ERR(nfc.regs);
    nand_chip = &nfc.chip;
    mtd = nand_to_mtd(nand_chip);
    mtd.dev.parent = &pdev.dev;
    for_each_child_of_node(np, nand_np)
    nand_set_flash_node(nand_chip, nand_np);
    nand_chip.priv = nfc;
    nfc.dev = &pdev.dev;
    nfc.controller.ops = &mxic_nand_controller_ops;
    nand_controller_init(&nfc.controller);
    nand_chip.controller = &nfc.controller;
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    mxic_nfc_hw_init(nfc);
    err = devm_request_irq(&pdev.dev, irq, mxic_nfc_isr,
    0, "mxic-nfc", nfc);
    if (err)
    goto fail;
    err = nand_scan(nand_chip, 1);
    if (err)
    goto fail;
    err = mtd_device_register(mtd, core::ptr::null_mut(), 0);
    if (err)
    goto fail;
    platform_set_drvdata(pdev, nfc);
    return 0;
    fail:
    mxic_nfc_clk_disable(nfc);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn mxic_nfc_remove(pdev: *mut platform_device) {
    static void mxic_nfc_remove(struct platform_device *pdev)
    {
    struct mxic_nand_ctlr *nfc = platform_get_drvdata(pdev);
    struct nand_chip *chip = &nfc.chip;
    int ret;
    ret = mtd_device_unregister(nand_to_mtd(chip));
    WARN_ON(ret);
    nand_cleanup(chip);
    mxic_nfc_clk_disable(nfc);
    }
    static const struct of_device_id mxic_nfc_of_ids[] = {
    { .compatible = "mxic,multi-itfc-v009-nand-controller", },
    {},
    };
    MODULE_DEVICE_TABLE(of, mxic_nfc_of_ids);
    static struct platform_driver mxic_nfc_driver = {
    .probe = mxic_nfc_probe,
    .remove = mxic_nfc_remove,
    .driver = {
    .name = "mxic-nfc",
    .of_match_table = mxic_nfc_of_ids,
    },
    };
    module_platform_driver(mxic_nfc_driver);
    MODULE_AUTHOR("Mason Yang <masonccyang@mxic.com.tw>");
    MODULE_DESCRIPTION("Macronix raw NAND controller driver");
    MODULE_LICENSE("GPL v2");
