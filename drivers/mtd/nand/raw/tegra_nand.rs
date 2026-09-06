//! Automatically rewritten from C to Rust
//! Source: drivers/mtd/nand/raw/tegra_nand.c
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
// Copyright (C) 2018 Stefan Agner <stefan@agner.ch>
// Copyright (C) 2014-2015 Lucas Stach <dev@lynxeye.de>
// Copyright (C) 2012 Avionic Design GmbH
//

pub const COMMAND: c_uint = 0x00;

pub const STATUS: c_uint = 0x04;
pub const ISR: c_uint = 0x08;

pub const IER: c_uint = 0x0c;

pub const CONFIG: c_uint = 0x10;

pub const TIMING_1: c_uint = 0x14;

pub const RESP: c_uint = 0x18;
pub const TIMING_2: c_uint = 0x1c;

pub const CMD_REG1: c_uint = 0x20;
pub const CMD_REG2: c_uint = 0x24;
pub const ADDR_REG1: c_uint = 0x28;
pub const ADDR_REG2: c_uint = 0x2c;
pub const DMA_MST_CTRL: c_uint = 0x30;

pub const DMA_CFG_A: c_uint = 0x34;
pub const DMA_CFG_B: c_uint = 0x38;
pub const FIFO_CTRL: c_uint = 0x3c;

pub const DATA_PTR: c_uint = 0x40;
pub const TAG_PTR: c_uint = 0x44;
pub const ECC_PTR: c_uint = 0x48;
pub const DEC_STATUS: c_uint = 0x4c;

pub const DEC_STATUS_ERR_COUNT_MASK: c_uint = 0x00ff0000;
pub const DEC_STATUS_ERR_COUNT_SHIFT: c_int = 16;
pub const HWSTATUS_CMD: c_uint = 0x50;
pub const HWSTATUS_MASK: c_uint = 0x54;

pub const BCH_CONFIG: c_uint = 0xcc;

pub const DEC_STAT_RESULT: c_uint = 0xd0;
pub const DEC_STAT_BUF: c_uint = 0xd4;
pub const DEC_STAT_BUF_FAIL_SEC_FLAG_MASK: c_uint = 0xff000000;
pub const DEC_STAT_BUF_FAIL_SEC_FLAG_SHIFT: c_int = 24;
pub const DEC_STAT_BUF_CORR_SEC_FLAG_MASK: c_uint = 0x00ff0000;
pub const DEC_STAT_BUF_CORR_SEC_FLAG_SHIFT: c_int = 16;
pub const DEC_STAT_BUF_MAX_CORR_CNT_MASK: c_uint = 0x00001f00;
pub const DEC_STAT_BUF_MAX_CORR_CNT_SHIFT: c_int = 8;

pub const SKIP_SPARE_BYTES: c_int = 4;
pub const BITS_PER_STEP_RS: c_int = 18;
pub const BITS_PER_STEP_BCH: c_int = 13;

    HWSTATUS_RDSTATUS_VALUE(0) | \
    HWSTATUS_RBSY_MASK(NAND_STATUS_READY) | \
    HWSTATUS_RBSY_VALUE(NAND_STATUS_READY))
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_nand_controller {
    pub controller: nand_controller,
    pub dev: *mut device,
    pub regs: *mut void __iomem,
    pub irq: c_int,
    pub clk: *mut clk,
    pub command_complete: completion,
    pub dma_complete: completion,
    pub last_read_error: bool,
    pub cur_cs: c_int,
    pub chip: *mut nand_chip,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_nand_chip {
    pub chip: nand_chip,
    pub wp_gpio: *mut gpio_desc,
    pub ecc: mtd_oob_region,
    pub config: u32,
    pub config_ecc: u32,
    pub bch_config: u32,
    pub cs: [c_int; 1],
}

    static inline struct tegra_nand_controller *
    to_tegra_ctrl(struct nand_controller *hw_ctrl)
    {
    return container_of(hw_ctrl, struct tegra_nand_controller, controller);
    }
    static inline struct tegra_nand_chip *to_tegra_chip(struct nand_chip *chip)
    {
    return container_of(chip, struct tegra_nand_chip, chip);
    }
    static int tegra_nand_ooblayout_rs_ecc(struct mtd_info *mtd, int section,
    struct mtd_oob_region *oobregion)
    {
    struct nand_chip *chip = mtd_to_nand(mtd);
    int bytes_per_step = DIV_ROUND_UP(BITS_PER_STEP_RS * chip.ecc.strength,
    BITS_PER_BYTE);
    if (section > 0)
    return -ERANGE;
    oobregion.offset = SKIP_SPARE_BYTES;
    oobregion.length = round_up(bytes_per_step * chip.ecc.steps, 4);
    return 0;
    }
    static int tegra_nand_ooblayout_no_free(struct mtd_info *mtd, int section,
    struct mtd_oob_region *oobregion)
    {
    return -ERANGE;
    }
    static const struct mtd_ooblayout_ops tegra_nand_oob_rs_ops = {
    .ecc = tegra_nand_ooblayout_rs_ecc,
    .free = tegra_nand_ooblayout_no_free,
    };
    static int tegra_nand_ooblayout_bch_ecc(struct mtd_info *mtd, int section,
    struct mtd_oob_region *oobregion)
    {
    struct nand_chip *chip = mtd_to_nand(mtd);
    int bytes_per_step = DIV_ROUND_UP(BITS_PER_STEP_BCH * chip.ecc.strength,
    BITS_PER_BYTE);
    if (section > 0)
    return -ERANGE;
    oobregion.offset = SKIP_SPARE_BYTES;
    oobregion.length = round_up(bytes_per_step * chip.ecc.steps, 4);
    return 0;
    }
    static const struct mtd_ooblayout_ops tegra_nand_oob_bch_ops = {
    .ecc = tegra_nand_ooblayout_bch_ecc,
    .free = tegra_nand_ooblayout_no_free,
    };
#[no_mangle]
unsafe extern "C" fn tegra_nand_irq(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t tegra_nand_irq(int irq, void *data)
    {
    struct tegra_nand_controller *ctrl = data;
    u32 isr, dma;
    isr = readl_relaxed(ctrl.regs + ISR);
    dma = readl_relaxed(ctrl.regs + DMA_MST_CTRL);
    dev_dbg(ctrl.dev, "isr %08x\n", isr);
    if (!isr && !(dma & DMA_MST_CTRL_IS_DONE))
    return IRQ_NONE;
//
// The bit name is somewhat missleading: This is also set when
// HW ECC was successful. The data sheet states:
// Correctable OR Un-correctable errors occurred in the DMA transfer...
//
    if (isr & ISR_CORRFAIL_ERR)
    ctrl.last_read_error = true;
    if (isr & ISR_CMD_DONE)
    complete(&ctrl.command_complete);
    if (isr & ISR_UND)
    dev_err(ctrl.dev, "FIFO underrun\n");
    if (isr & ISR_OVR)
    dev_err(ctrl.dev, "FIFO overrun\n");
// handle DMA interrupts
    if (dma & DMA_MST_CTRL_IS_DONE) {
    writel_relaxed(dma, ctrl.regs + DMA_MST_CTRL);
    complete(&ctrl.dma_complete);
    }
// clear interrupts
    writel_relaxed(isr, ctrl.regs + ISR);
    return IRQ_HANDLED;
    }
    static const char * const tegra_nand_reg_names[] = {
    "COMMAND",
    "STATUS",
    "ISR",
    "IER",
    "CONFIG",
    "TIMING",
    core::ptr::null_mut(),
    "TIMING2",
    "CMD_REG1",
    "CMD_REG2",
    "ADDR_REG1",
    "ADDR_REG2",
    "DMA_MST_CTRL",
    "DMA_CFG_A",
    "DMA_CFG_B",
    "FIFO_CTRL",
    };
#[no_mangle]
unsafe extern "C" fn tegra_nand_dump_reg(ctrl: *mut tegra_nand_controller) {
    static void tegra_nand_dump_reg(struct tegra_nand_controller *ctrl)
    {
    u32 reg;
    int i;
    dev_err(ctrl.dev, "Tegra NAND controller register dump\n");
    for (i = 0; i < ARRAY_SIZE(tegra_nand_reg_names); i++) {
    const char *reg_name = tegra_nand_reg_names[i];
    if (!reg_name)
    continue;
    reg = readl_relaxed(ctrl.regs + (i * 4));
    dev_err(ctrl.dev, "%s: 0x%08x\n", reg_name, reg);
    }
    }
#[no_mangle]
unsafe extern "C" fn tegra_nand_controller_abort(ctrl: *mut tegra_nand_controller) {
    static void tegra_nand_controller_abort(struct tegra_nand_controller *ctrl)
    {
    u32 isr, dma;
    disable_irq(ctrl.irq);
// Abort current command/DMA operation
    writel_relaxed(0, ctrl.regs + DMA_MST_CTRL);
    writel_relaxed(0, ctrl.regs + COMMAND);
// clear interrupts
    isr = readl_relaxed(ctrl.regs + ISR);
    writel_relaxed(isr, ctrl.regs + ISR);
    dma = readl_relaxed(ctrl.regs + DMA_MST_CTRL);
    writel_relaxed(dma, ctrl.regs + DMA_MST_CTRL);
    reinit_completion(&ctrl.command_complete);
    reinit_completion(&ctrl.dma_complete);
    enable_irq(ctrl.irq);
    }
    static int tegra_nand_cmd(struct nand_chip *chip,
    const struct nand_subop *subop)
    {
    const struct nand_op_instr *instr;
    const struct nand_op_instr *instr_data_in = core::ptr::null_mut();
    struct tegra_nand_controller *ctrl = to_tegra_ctrl(chip.controller);
    unsigned int op_id, size = 0, offset = 0;
    let mut first_cmd: bool = true;
    u32 reg, cmd = 0;
    int ret;
    for (op_id = 0; op_id < subop.ninstrs; op_id++) {
    unsigned int naddrs, i;
    const u8 *addrs;
    let mut addr1: u32 = 0, addr2 = 0;
    instr = &subop.instrs[op_id];
    switch (instr.type) {
    case NAND_OP_CMD_INSTR:
    if (first_cmd) {
    cmd |= COMMAND_CLE;
    writel_relaxed(instr.ctx.cmd.opcode,
    ctrl.regs + CMD_REG1);
    } else {
    cmd |= COMMAND_SEC_CMD;
    writel_relaxed(instr.ctx.cmd.opcode,
    ctrl.regs + CMD_REG2);
    }
    first_cmd = false;
    break;
    case NAND_OP_ADDR_INSTR:
    offset = nand_subop_get_addr_start_off(subop, op_id);
    naddrs = nand_subop_get_num_addr_cyc(subop, op_id);
    addrs = &instr.ctx.addr.addrs[offset];
    cmd |= COMMAND_ALE | COMMAND_ALE_SIZE(naddrs);
    for (i = 0; i < min_t(unsigned int, 4, naddrs); i++)
    addr1 |= *addrs++ << (BITS_PER_BYTE * i);
    naddrs -= i;
    for (i = 0; i < min_t(unsigned int, 4, naddrs); i++)
    addr2 |= *addrs++ << (BITS_PER_BYTE * i);
    writel_relaxed(addr1, ctrl.regs + ADDR_REG1);
    writel_relaxed(addr2, ctrl.regs + ADDR_REG2);
    break;
    case NAND_OP_DATA_IN_INSTR:
    size = nand_subop_get_data_len(subop, op_id);
    offset = nand_subop_get_data_start_off(subop, op_id);
    cmd |= COMMAND_TRANS_SIZE(size) | COMMAND_PIO |
    COMMAND_RX | COMMAND_A_VALID;
    instr_data_in = instr;
    break;
    case NAND_OP_DATA_OUT_INSTR:
    size = nand_subop_get_data_len(subop, op_id);
    offset = nand_subop_get_data_start_off(subop, op_id);
    cmd |= COMMAND_TRANS_SIZE(size) | COMMAND_PIO |
    COMMAND_TX | COMMAND_A_VALID;
    memcpy(&reg, instr.ctx.data.buf.out + offset, size);
    writel_relaxed(reg, ctrl.regs + RESP);
    break;
    case NAND_OP_WAITRDY_INSTR:
    cmd |= COMMAND_RBSY_CHK;
    break;
    }
    }
    cmd |= COMMAND_GO | COMMAND_CE(ctrl.cur_cs);
    writel_relaxed(cmd, ctrl.regs + COMMAND);
    ret = wait_for_completion_timeout(&ctrl.command_complete,
    msecs_to_jiffies(500));
    if (!ret) {
    dev_err(ctrl.dev, "COMMAND timeout\n");
    tegra_nand_dump_reg(ctrl);
    tegra_nand_controller_abort(ctrl);
    return -ETIMEDOUT;
    }
    if (instr_data_in) {
    reg = readl_relaxed(ctrl.regs + RESP);
    memcpy(instr_data_in.ctx.data.buf.in + offset, &reg, size);
    }
    return 0;
    }
    static const struct nand_op_parser tegra_nand_op_parser = NAND_OP_PARSER(
    NAND_OP_PARSER_PATTERN(tegra_nand_cmd,
    NAND_OP_PARSER_PAT_CMD_ELEM(true),
    NAND_OP_PARSER_PAT_ADDR_ELEM(true, 8),
    NAND_OP_PARSER_PAT_CMD_ELEM(true),
    NAND_OP_PARSER_PAT_WAITRDY_ELEM(true)),
    NAND_OP_PARSER_PATTERN(tegra_nand_cmd,
    NAND_OP_PARSER_PAT_DATA_OUT_ELEM(false, 4)),
    NAND_OP_PARSER_PATTERN(tegra_nand_cmd,
    NAND_OP_PARSER_PAT_CMD_ELEM(true),
    NAND_OP_PARSER_PAT_ADDR_ELEM(true, 8),
    NAND_OP_PARSER_PAT_CMD_ELEM(true),
    NAND_OP_PARSER_PAT_WAITRDY_ELEM(true),
    NAND_OP_PARSER_PAT_DATA_IN_ELEM(true, 4)),
    );
    static void tegra_nand_select_target(struct nand_chip *chip,
    unsigned int die_nr)
    {
    struct tegra_nand_chip *nand = to_tegra_chip(chip);
    struct tegra_nand_controller *ctrl = to_tegra_ctrl(chip.controller);
    ctrl.cur_cs = nand.cs[die_nr];
    }
    static int tegra_nand_exec_op(struct nand_chip *chip,
    const struct nand_operation *op,
    bool check_only)
    {
    if (!check_only)
    tegra_nand_select_target(chip, op.cs);
    return nand_op_parser_exec_op(chip, &tegra_nand_op_parser, op,
    check_only);
    }
    static void tegra_nand_hw_ecc(struct tegra_nand_controller *ctrl,
    struct nand_chip *chip, bool enable)
    {
    struct tegra_nand_chip *nand = to_tegra_chip(chip);
    if (chip.ecc.algo == NAND_ECC_ALGO_BCH && enable)
    writel_relaxed(nand.bch_config, ctrl.regs + BCH_CONFIG);
    else
    writel_relaxed(0, ctrl.regs + BCH_CONFIG);
    if (enable)
    writel_relaxed(nand.config_ecc, ctrl.regs + CONFIG);
    else
    writel_relaxed(nand.config, ctrl.regs + CONFIG);
    }
    static int tegra_nand_page_xfer(struct mtd_info *mtd, struct nand_chip *chip,
    void *buf, void *oob_buf, int oob_len, int page,
    bool read)
    {
    struct tegra_nand_controller *ctrl = to_tegra_ctrl(chip.controller);
    let mut dir: enum dma_data_direction = read ? DMA_FROM_DEVICE : DMA_TO_DEVICE;
    let mut dma_addr: dma_addr_t = 0, dma_addr_oob = 0;
    u32 addr1, cmd, dma_ctrl;
    int ret;
    tegra_nand_select_target(chip, chip.cur_cs);
    if (read) {
    writel_relaxed(NAND_CMD_READ0, ctrl.regs + CMD_REG1);
    writel_relaxed(NAND_CMD_READSTART, ctrl.regs + CMD_REG2);
    } else {
    writel_relaxed(NAND_CMD_SEQIN, ctrl.regs + CMD_REG1);
    writel_relaxed(NAND_CMD_PAGEPROG, ctrl.regs + CMD_REG2);
    }
    cmd = COMMAND_CLE | COMMAND_SEC_CMD;
// Lower 16-bits are column, by default 0
    addr1 = page << 16;
    if (!buf)
    addr1 |= mtd.writesize;
    writel_relaxed(addr1, ctrl.regs + ADDR_REG1);
    if (chip.options & NAND_ROW_ADDR_3) {
    writel_relaxed(page >> 16, ctrl.regs + ADDR_REG2);
    cmd |= COMMAND_ALE | COMMAND_ALE_SIZE(5);
    } else {
    cmd |= COMMAND_ALE | COMMAND_ALE_SIZE(4);
    }
    if (buf) {
    dma_addr = dma_map_single(ctrl.dev, buf, mtd.writesize, dir);
    ret = dma_mapping_error(ctrl.dev, dma_addr);
    if (ret) {
    dev_err(ctrl.dev, "dma mapping error\n");
    return -EINVAL;
    }
    writel_relaxed(mtd.writesize - 1, ctrl.regs + DMA_CFG_A);
    writel_relaxed(dma_addr, ctrl.regs + DATA_PTR);
    }
    if (oob_buf) {
    dma_addr_oob = dma_map_single(ctrl.dev, oob_buf, mtd.oobsize,
    dir);
    ret = dma_mapping_error(ctrl.dev, dma_addr_oob);
    if (ret) {
    dev_err(ctrl.dev, "dma mapping error\n");
    ret = -EINVAL;
    goto err_unmap_dma_page;
    }
    writel_relaxed(oob_len - 1, ctrl.regs + DMA_CFG_B);
    writel_relaxed(dma_addr_oob, ctrl.regs + TAG_PTR);
    }
    dma_ctrl = DMA_MST_CTRL_GO | DMA_MST_CTRL_PERF_EN |
    DMA_MST_CTRL_IE_DONE | DMA_MST_CTRL_IS_DONE |
    DMA_MST_CTRL_BURST_16;
    if (buf)
    dma_ctrl |= DMA_MST_CTRL_EN_A;
    if (oob_buf)
    dma_ctrl |= DMA_MST_CTRL_EN_B;
    if (read)
    dma_ctrl |= DMA_MST_CTRL_IN | DMA_MST_CTRL_REUSE;
    else
    dma_ctrl |= DMA_MST_CTRL_OUT;
    writel_relaxed(dma_ctrl, ctrl.regs + DMA_MST_CTRL);
    cmd |= COMMAND_GO | COMMAND_RBSY_CHK | COMMAND_TRANS_SIZE(9) |
    COMMAND_CE(ctrl.cur_cs);
    if (buf)
    cmd |= COMMAND_A_VALID;
    if (oob_buf)
    cmd |= COMMAND_B_VALID;
    if (read)
    cmd |= COMMAND_RX;
    else
    cmd |= COMMAND_TX | COMMAND_AFT_DAT;
    writel_relaxed(cmd, ctrl.regs + COMMAND);
    ret = wait_for_completion_timeout(&ctrl.command_complete,
    msecs_to_jiffies(500));
    if (!ret) {
    dev_err(ctrl.dev, "COMMAND timeout\n");
    tegra_nand_dump_reg(ctrl);
    tegra_nand_controller_abort(ctrl);
    ret = -ETIMEDOUT;
    goto err_unmap_dma;
    }
    ret = wait_for_completion_timeout(&ctrl.dma_complete,
    msecs_to_jiffies(500));
    if (!ret) {
    dev_err(ctrl.dev, "DMA timeout\n");
    tegra_nand_dump_reg(ctrl);
    tegra_nand_controller_abort(ctrl);
    ret = -ETIMEDOUT;
    goto err_unmap_dma;
    }
    ret = 0;
    err_unmap_dma:
    if (oob_buf)
    dma_unmap_single(ctrl.dev, dma_addr_oob, mtd.oobsize, dir);
    err_unmap_dma_page:
    if (buf)
    dma_unmap_single(ctrl.dev, dma_addr, mtd.writesize, dir);
    return ret;
    }
    static int tegra_nand_read_page_raw(struct nand_chip *chip, u8 *buf,
    int oob_required, int page)
    {
    struct mtd_info *mtd = nand_to_mtd(chip);
    void *oob_buf = oob_required ? chip.oob_poi : core::ptr::null_mut();
    return tegra_nand_page_xfer(mtd, chip, buf, oob_buf,
    mtd.oobsize, page, true);
    }
    static int tegra_nand_write_page_raw(struct nand_chip *chip, const u8 *buf,
    int oob_required, int page)
    {
    struct mtd_info *mtd = nand_to_mtd(chip);
    void *oob_buf = oob_required ? chip.oob_poi : core::ptr::null_mut();
    return tegra_nand_page_xfer(mtd, chip, (void *)buf, oob_buf,
    mtd.oobsize, page, false);
    }
#[no_mangle]
unsafe extern "C" fn tegra_nand_read_oob(chip: *mut nand_chip, page: c_int) -> c_int {
    static int tegra_nand_read_oob(struct nand_chip *chip, int page)
    {
    struct mtd_info *mtd = nand_to_mtd(chip);
    return tegra_nand_page_xfer(mtd, chip, core::ptr::null_mut(), chip.oob_poi,
    mtd.oobsize, page, true);
    }
#[no_mangle]
unsafe extern "C" fn tegra_nand_write_oob(chip: *mut nand_chip, page: c_int) -> c_int {
    static int tegra_nand_write_oob(struct nand_chip *chip, int page)
    {
    struct mtd_info *mtd = nand_to_mtd(chip);
    return tegra_nand_page_xfer(mtd, chip, core::ptr::null_mut(), chip.oob_poi,
    mtd.oobsize, page, false);
    }
    static int tegra_nand_read_page_hwecc(struct nand_chip *chip, u8 *buf,
    int oob_required, int page)
    {
    struct mtd_info *mtd = nand_to_mtd(chip);
    struct tegra_nand_controller *ctrl = to_tegra_ctrl(chip.controller);
    struct tegra_nand_chip *nand = to_tegra_chip(chip);
    void *oob_buf = oob_required ? chip.oob_poi : core::ptr::null_mut();
    u32 dec_stat, max_corr_cnt;
    unsigned long fail_sec_flag;
    int ret;
    tegra_nand_hw_ecc(ctrl, chip, true);
    ret = tegra_nand_page_xfer(mtd, chip, buf, oob_buf, 0, page, true);
    tegra_nand_hw_ecc(ctrl, chip, false);
    if (ret)
    return ret;
// No correctable or un-correctable errors, page must have 0 bitflips
    if (!ctrl.last_read_error)
    return 0;
//
// Correctable or un-correctable errors occurred. Use DEC_STAT_BUF
// which contains information for all ECC selections.
//
// Note that since we do not use Command Queues DEC_RESULT does not
// state the number of pages we can read from the DEC_STAT_BUF. But
// since CORRFAIL_ERR did occur during page read we do have a valid
// result in DEC_STAT_BUF.
//
    ctrl.last_read_error = false;
    dec_stat = readl_relaxed(ctrl.regs + DEC_STAT_BUF);
    fail_sec_flag = (dec_stat & DEC_STAT_BUF_FAIL_SEC_FLAG_MASK) >>
    DEC_STAT_BUF_FAIL_SEC_FLAG_SHIFT;
    max_corr_cnt = (dec_stat & DEC_STAT_BUF_MAX_CORR_CNT_MASK) >>
    DEC_STAT_BUF_MAX_CORR_CNT_SHIFT;
    if (fail_sec_flag) {
    int bit, max_bitflips = 0;
//
// Since we do not support subpage writes, a complete page
// is either written or not. We can take a shortcut here by
// checking wheather any of the sector has been successful
// read. If at least one sectors has been read successfully,
// the page must have been a written previously. It cannot
// be an erased page.
//
// E.g. controller might return fail_sec_flag with 0x4, which
// would mean only the third sector failed to correct. The
// page must have been written and the third sector is really
// not correctable anymore.
//
    if (fail_sec_flag ^ GENMASK(chip.ecc.steps - 1, 0)) {
    mtd.ecc_stats.failed += hweight8(fail_sec_flag);
    return max_corr_cnt;
    }
//
// All sectors failed to correct, but the ECC isn't smart
// enough to figure out if a page is really just erased.
// Read OOB data and check whether data/OOB is completely
// erased or if error correction just failed for all sub-
// pages.
//
    ret = tegra_nand_read_oob(chip, page);
    if (ret < 0)
    return ret;
    for_each_set_bit(bit, &fail_sec_flag, chip.ecc.steps) {
    u8 *data = buf + (chip.ecc.size * bit);
    u8 *oob = chip.oob_poi + nand.ecc.offset +
    (chip.ecc.bytes * bit);
    ret = nand_check_erased_ecc_chunk(data, chip.ecc.size,
    oob, chip.ecc.bytes,
    core::ptr::null_mut(), 0,
    chip.ecc.strength);
    if (ret < 0) {
    mtd.ecc_stats.failed++;
    } else {
    mtd.ecc_stats.corrected += ret;
    max_bitflips = max(ret, max_bitflips);
    }
    }
    return max_t(unsigned int, max_corr_cnt, max_bitflips);
    } else {
    int corr_sec_flag;
    corr_sec_flag = (dec_stat & DEC_STAT_BUF_CORR_SEC_FLAG_MASK) >>
    DEC_STAT_BUF_CORR_SEC_FLAG_SHIFT;
//
// The value returned in the register is the maximum of
// bitflips encountered in any of the ECC regions. As there is
// no way to get the number of bitflips in a specific regions
// we are not able to deliver correct stats but instead
// overestimate the number of corrected bitflips by assuming
// that all regions where errors have been corrected
// encountered the maximum number of bitflips.
//
    mtd.ecc_stats.corrected += max_corr_cnt * hweight8(corr_sec_flag);
    return max_corr_cnt;
    }
    }
    static int tegra_nand_write_page_hwecc(struct nand_chip *chip, const u8 *buf,
    int oob_required, int page)
    {
    struct mtd_info *mtd = nand_to_mtd(chip);
    struct tegra_nand_controller *ctrl = to_tegra_ctrl(chip.controller);
    void *oob_buf = oob_required ? chip.oob_poi : core::ptr::null_mut();
    int ret;
    tegra_nand_hw_ecc(ctrl, chip, true);
    ret = tegra_nand_page_xfer(mtd, chip, (void *)buf, oob_buf,
    0, page, false);
    tegra_nand_hw_ecc(ctrl, chip, false);
    return ret;
    }
    static void tegra_nand_setup_timing(struct tegra_nand_controller *ctrl,
    const struct nand_sdr_timings *timings)
    {
//
// The period (and all other timings in this function) is in ps,
// so need to take care here to avoid integer overflows.
//
    let mut rate: c_uint = clk_get_rate(ctrl.clk) / 1000000;
    let mut period: c_uint = DIV_ROUND_UP(1000000, rate);
    u32 val, reg = 0;
    val = DIV_ROUND_UP(max3(timings.tAR_min, timings.tRR_min,
    timings.tRC_min), period);
    reg |= TIMING_TCR_TAR_TRR(OFFSET(val, 3));
    val = DIV_ROUND_UP(max(max(timings.tCS_min, timings.tCH_min),
    max(timings.tALS_min, timings.tALH_min)),
    period);
    reg |= TIMING_TCS(OFFSET(val, 2));
    val = DIV_ROUND_UP(max(timings.tRP_min, timings.tREA_max) + 6000,
    period);
    reg |= TIMING_TRP(OFFSET(val, 1)) | TIMING_TRP_RESP(OFFSET(val, 1));
    reg |= TIMING_TWB(OFFSET(DIV_ROUND_UP(timings.tWB_max, period), 1));
    reg |= TIMING_TWHR(OFFSET(DIV_ROUND_UP(timings.tWHR_min, period), 1));
    reg |= TIMING_TWH(OFFSET(DIV_ROUND_UP(timings.tWH_min, period), 1));
    reg |= TIMING_TWP(OFFSET(DIV_ROUND_UP(timings.tWP_min, period), 1));
    reg |= TIMING_TRH(OFFSET(DIV_ROUND_UP(timings.tREH_min, period), 1));
    writel_relaxed(reg, ctrl.regs + TIMING_1);
    val = DIV_ROUND_UP(timings.tADL_min, period);
    reg = TIMING_TADL(OFFSET(val, 3));
    writel_relaxed(reg, ctrl.regs + TIMING_2);
    }
    static int tegra_nand_setup_interface(struct nand_chip *chip, int csline,
    const struct nand_interface_config *conf)
    {
    struct tegra_nand_controller *ctrl = to_tegra_ctrl(chip.controller);
    const struct nand_sdr_timings *timings;
    timings = nand_get_sdr_timings(conf);
    if (IS_ERR(timings))
    return PTR_ERR(timings);
    if (csline == NAND_DATA_IFACE_CHECK_ONLY)
    return 0;
    tegra_nand_setup_timing(ctrl, timings);
    return 0;
    }
    static const int rs_strength_bootable[] = { 4 };
    static const int rs_strength[] = { 4, 6, 8 };
    static const int bch_strength_bootable[] = { 8, 16 };
    static const int bch_strength[] = { 4, 8, 14, 16 };
    static int tegra_nand_get_strength(struct nand_chip *chip, const int *strength,
    int strength_len, int bits_per_step,
    int oobsize)
    {
    struct nand_device *base = mtd_to_nanddev(nand_to_mtd(chip));
    const struct nand_ecc_props *requirements =
    nanddev_get_ecc_requirements(base);
    let mut maximize: bool = base.ecc.user_conf.flags & NAND_ECC_MAXIMIZE_STRENGTH;
    int i;
//
// Loop through available strengths. Backwards in case we try to
// maximize the BCH strength.
//
    for (i = 0; i < strength_len; i++) {
    int strength_sel, bytes_per_step, bytes_per_page;
    if (maximize) {
    strength_sel = strength[strength_len - i - 1];
    } else {
    strength_sel = strength[i];
    if (strength_sel < requirements.strength)
    continue;
    }
    bytes_per_step = DIV_ROUND_UP(bits_per_step * strength_sel,
    BITS_PER_BYTE);
    bytes_per_page = round_up(bytes_per_step * chip.ecc.steps, 4);
// Check whether strength fits OOB
    if (bytes_per_page < (oobsize - SKIP_SPARE_BYTES))
    return strength_sel;
    }
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn tegra_nand_select_strength(chip: *mut nand_chip, oobsize: c_int) -> c_int {
    static int tegra_nand_select_strength(struct nand_chip *chip, int oobsize)
    {
    const int *strength;
    int strength_len, bits_per_step;
    switch (chip.ecc.algo) {
    case NAND_ECC_ALGO_RS:
    bits_per_step = BITS_PER_STEP_RS;
    if (chip.options & NAND_IS_BOOT_MEDIUM) {
    strength = rs_strength_bootable;
    strength_len = ARRAY_SIZE(rs_strength_bootable);
    } else {
    strength = rs_strength;
    strength_len = ARRAY_SIZE(rs_strength);
    }
    break;
    case NAND_ECC_ALGO_BCH:
    bits_per_step = BITS_PER_STEP_BCH;
    if (chip.options & NAND_IS_BOOT_MEDIUM) {
    strength = bch_strength_bootable;
    strength_len = ARRAY_SIZE(bch_strength_bootable);
    } else {
    strength = bch_strength;
    strength_len = ARRAY_SIZE(bch_strength);
    }
    break;
    default:
    return -EINVAL;
    }
    return tegra_nand_get_strength(chip, strength, strength_len,
    bits_per_step, oobsize);
    }
#[no_mangle]
unsafe extern "C" fn tegra_nand_attach_chip(chip: *mut nand_chip) -> c_int {
    static int tegra_nand_attach_chip(struct nand_chip *chip)
    {
    struct tegra_nand_controller *ctrl = to_tegra_ctrl(chip.controller);
    const struct nand_ecc_props *requirements =
    nanddev_get_ecc_requirements(&chip.base);
    struct tegra_nand_chip *nand = to_tegra_chip(chip);
    struct mtd_info *mtd = nand_to_mtd(chip);
    int bits_per_step;
    int ret;
    if (chip.bbt_options & NAND_BBT_USE_FLASH)
    chip.bbt_options |= NAND_BBT_NO_OOB;
    chip.ecc.engine_type = NAND_ECC_ENGINE_TYPE_ON_HOST;
    chip.ecc.size = 512;
    chip.ecc.steps = mtd.writesize / chip.ecc.size;
    if (requirements.step_size != 512) {
    dev_err(ctrl.dev, "Unsupported step size %d\n",
    requirements.step_size);
    return -EINVAL;
    }
    chip.ecc.read_page = tegra_nand_read_page_hwecc;
    chip.ecc.write_page = tegra_nand_write_page_hwecc;
    chip.ecc.read_page_raw = tegra_nand_read_page_raw;
    chip.ecc.write_page_raw = tegra_nand_write_page_raw;
    chip.ecc.read_oob = tegra_nand_read_oob;
    chip.ecc.write_oob = tegra_nand_write_oob;
    if (chip.options & NAND_BUSWIDTH_16)
    nand.config |= CONFIG_BUS_WIDTH_16;
    if (chip.ecc.algo == NAND_ECC_ALGO_UNKNOWN) {
    if (mtd.writesize < 2048)
    chip.ecc.algo = NAND_ECC_ALGO_RS;
    else
    chip.ecc.algo = NAND_ECC_ALGO_BCH;
    }
    if (chip.ecc.algo == NAND_ECC_ALGO_BCH && mtd.writesize < 2048) {
    dev_err(ctrl.dev, "BCH supports 2K or 4K page size only\n");
    return -EINVAL;
    }
    if (!chip.ecc.strength) {
    ret = tegra_nand_select_strength(chip, mtd.oobsize);
    if (ret < 0) {
    dev_err(ctrl.dev,
    "No valid strength found, minimum %d\n",
    requirements.strength);
    return ret;
    }
    chip.ecc.strength = ret;
    }
    nand.config_ecc = CONFIG_PIPE_EN | CONFIG_SKIP_SPARE |
    CONFIG_SKIP_SPARE_SIZE_4;
    switch (chip.ecc.algo) {
    case NAND_ECC_ALGO_RS:
    bits_per_step = BITS_PER_STEP_RS * chip.ecc.strength;
    mtd_set_ooblayout(mtd, &tegra_nand_oob_rs_ops);
    nand.config_ecc |= CONFIG_HW_ECC | CONFIG_ECC_SEL |
    CONFIG_ERR_COR;
    switch (chip.ecc.strength) {
    case 4:
    nand.config_ecc |= CONFIG_TVAL_4;
    break;
    case 6:
    nand.config_ecc |= CONFIG_TVAL_6;
    break;
    case 8:
    nand.config_ecc |= CONFIG_TVAL_8;
    break;
    default:
    dev_err(ctrl.dev, "ECC strength %d not supported\n",
    chip.ecc.strength);
    return -EINVAL;
    }
    break;
    case NAND_ECC_ALGO_BCH:
    bits_per_step = BITS_PER_STEP_BCH * chip.ecc.strength;
    mtd_set_ooblayout(mtd, &tegra_nand_oob_bch_ops);
    nand.bch_config = BCH_ENABLE;
    switch (chip.ecc.strength) {
    case 4:
    nand.bch_config |= BCH_TVAL_4;
    break;
    case 8:
    nand.bch_config |= BCH_TVAL_8;
    break;
    case 14:
    nand.bch_config |= BCH_TVAL_14;
    break;
    case 16:
    nand.bch_config |= BCH_TVAL_16;
    break;
    default:
    dev_err(ctrl.dev, "ECC strength %d not supported\n",
    chip.ecc.strength);
    return -EINVAL;
    }
    break;
    default:
    dev_err(ctrl.dev, "ECC algorithm not supported\n");
    return -EINVAL;
    }
    dev_info(ctrl.dev, "Using %s with strength %d per 512 byte step\n",
    chip.ecc.algo == NAND_ECC_ALGO_BCH ? "BCH" : "RS",
    chip.ecc.strength);
    chip.ecc.bytes = DIV_ROUND_UP(bits_per_step, BITS_PER_BYTE);
    switch (mtd.writesize) {
    case 256:
    nand.config |= CONFIG_PS_256;
    break;
    case 512:
    nand.config |= CONFIG_PS_512;
    break;
    case 1024:
    nand.config |= CONFIG_PS_1024;
    break;
    case 2048:
    nand.config |= CONFIG_PS_2048;
    break;
    case 4096:
    nand.config |= CONFIG_PS_4096;
    break;
    default:
    dev_err(ctrl.dev, "Unsupported writesize %d\n",
    mtd.writesize);
    return -ENODEV;
    }
// Store complete configuration for HW ECC in config_ecc
    nand.config_ecc |= nand.config;
// Non-HW ECC read/writes complete OOB
    nand.config |= CONFIG_TAG_BYTE_SIZE(mtd.oobsize - 1);
    writel_relaxed(nand.config, ctrl.regs + CONFIG);
    return 0;
    }
    static const struct nand_controller_ops tegra_nand_controller_ops = {
    .attach_chip = &tegra_nand_attach_chip,
    .exec_op = tegra_nand_exec_op,
    .setup_interface = tegra_nand_setup_interface,
    };
    static int tegra_nand_chips_init(struct device *dev,
    struct tegra_nand_controller *ctrl)
    {
    struct device_node *np = dev.of_node;
    struct device_node *np_nand;
    int nsels, nchips = of_get_child_count(np);
    struct tegra_nand_chip *nand;
    struct mtd_info *mtd;
    struct nand_chip *chip;
    int ret;
    u32 cs;
    if (nchips != 1) {
    dev_err(dev, "Currently only one NAND chip supported\n");
    return -EINVAL;
    }
    np_nand = of_get_next_child(np, core::ptr::null_mut());
    nsels = of_property_count_elems_of_size(np_nand, "reg", sizeof(u32));
    if (nsels != 1) {
    dev_err(dev, "Missing/invalid reg property\n");
    return -EINVAL;
    }
// Retrieve CS id, currently only single die NAND supported
    ret = of_property_read_u32(np_nand, "reg", &cs);
    if (ret) {
    dev_err(dev, "could not retrieve reg property: %d\n", ret);
    return ret;
    }
    nand = devm_kzalloc(dev, sizeof(*nand), GFP_KERNEL);
    if (!nand)
    return -ENOMEM;
    nand.cs[0] = cs;
    nand.wp_gpio = devm_gpiod_get_optional(dev, "wp", GPIOD_OUT_LOW);
    if (IS_ERR(nand.wp_gpio)) {
    ret = PTR_ERR(nand.wp_gpio);
    dev_err(dev, "Failed to request WP GPIO: %d\n", ret);
    return ret;
    }
    chip = &nand.chip;
    chip.controller = &ctrl.controller;
    mtd = nand_to_mtd(chip);
    mtd.dev.parent = dev;
    mtd.owner = THIS_MODULE;
    nand_set_flash_node(chip, np_nand);
    if (!mtd.name)
    mtd.name = "tegra_nand";
    chip.options = NAND_NO_SUBPAGE_WRITE | NAND_USES_DMA;
    ret = nand_scan(chip, 1);
    if (ret)
    return ret;
    mtd_ooblayout_ecc(mtd, 0, &nand.ecc);
    ret = mtd_device_register(mtd, core::ptr::null_mut(), 0);
    if (ret) {
    dev_err(dev, "Failed to register mtd device: %d\n", ret);
    nand_cleanup(chip);
    return ret;
    }
    ctrl.chip = chip;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tegra_nand_probe(pdev: *mut platform_device) -> c_int {
    static int tegra_nand_probe(struct platform_device *pdev)
    {
    struct reset_control *rst;
    struct tegra_nand_controller *ctrl;
    let mut err: c_int = 0;
    ctrl = devm_kzalloc(&pdev.dev, sizeof(*ctrl), GFP_KERNEL);
    if (!ctrl)
    return -ENOMEM;
    ctrl.dev = &pdev.dev;
    platform_set_drvdata(pdev, ctrl);
    nand_controller_init(&ctrl.controller);
    ctrl.controller.ops = &tegra_nand_controller_ops;
    ctrl.regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(ctrl.regs))
    return PTR_ERR(ctrl.regs);
    rst = devm_reset_control_get(&pdev.dev, "nand");
    if (IS_ERR(rst))
    return PTR_ERR(rst);
    ctrl.clk = devm_clk_get(&pdev.dev, "nand");
    if (IS_ERR(ctrl.clk))
    return PTR_ERR(ctrl.clk);
    err = devm_tegra_core_dev_init_opp_table_common(&pdev.dev);
    if (err)
    return err;
//
// This driver doesn't support active power management yet,
// so we will simply keep device resumed.
//
    pm_runtime_enable(&pdev.dev);
    err = pm_runtime_resume_and_get(&pdev.dev);
    if (err)
    goto err_dis_pm;
    err = reset_control_reset(rst);
    if (err) {
    dev_err(ctrl.dev, "Failed to reset HW: %d\n", err);
    goto err_put_pm;
    }
    writel_relaxed(HWSTATUS_CMD_DEFAULT, ctrl.regs + HWSTATUS_CMD);
    writel_relaxed(HWSTATUS_MASK_DEFAULT, ctrl.regs + HWSTATUS_MASK);
    writel_relaxed(INT_MASK, ctrl.regs + IER);
    init_completion(&ctrl.command_complete);
    init_completion(&ctrl.dma_complete);
    ctrl.irq = platform_get_irq(pdev, 0);
    if (ctrl.irq < 0) {
    err = ctrl.irq;
    goto err_put_pm;
    }
    err = devm_request_irq(&pdev.dev, ctrl.irq, tegra_nand_irq, 0,
    dev_name(&pdev.dev), ctrl);
    if (err) {
    dev_err(ctrl.dev, "Failed to get IRQ: %d\n", err);
    goto err_put_pm;
    }
    writel_relaxed(DMA_MST_CTRL_IS_DONE, ctrl.regs + DMA_MST_CTRL);
    err = tegra_nand_chips_init(ctrl.dev, ctrl);
    if (err)
    goto err_put_pm;
    return 0;
    err_put_pm:
    pm_runtime_put_sync_suspend(ctrl.dev);
    pm_runtime_force_suspend(ctrl.dev);
    err_dis_pm:
    pm_runtime_disable(&pdev.dev);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn tegra_nand_remove(pdev: *mut platform_device) {
    static void tegra_nand_remove(struct platform_device *pdev)
    {
    struct tegra_nand_controller *ctrl = platform_get_drvdata(pdev);
    struct nand_chip *chip = ctrl.chip;
    struct mtd_info *mtd = nand_to_mtd(chip);
    WARN_ON(mtd_device_unregister(mtd));
    nand_cleanup(chip);
    pm_runtime_put_sync_suspend(ctrl.dev);
    pm_runtime_force_suspend(ctrl.dev);
    }
#[no_mangle]
unsafe extern "C" fn tegra_nand_runtime_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused tegra_nand_runtime_resume(struct device *dev)
    {
    struct tegra_nand_controller *ctrl = dev_get_drvdata(dev);
    int err;
    err = clk_prepare_enable(ctrl.clk);
    if (err) {
    dev_err(dev, "Failed to enable clock: %d\n", err);
    return err;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tegra_nand_runtime_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused tegra_nand_runtime_suspend(struct device *dev)
    {
    struct tegra_nand_controller *ctrl = dev_get_drvdata(dev);
    clk_disable_unprepare(ctrl.clk);
    return 0;
    }
    static const struct dev_pm_ops tegra_nand_pm = {
    SET_RUNTIME_PM_OPS(tegra_nand_runtime_suspend, tegra_nand_runtime_resume,
    core::ptr::null_mut())
    };
    static const struct of_device_id tegra_nand_of_match[] = {
    { .compatible = "nvidia,tegra20-nand" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, tegra_nand_of_match);
    static struct platform_driver tegra_nand_driver = {
    .driver = {
    .name = "tegra-nand",
    .of_match_table = tegra_nand_of_match,
    .pm = &tegra_nand_pm,
    },
    .probe = tegra_nand_probe,
    .remove = tegra_nand_remove,
    };
    module_platform_driver(tegra_nand_driver);
    MODULE_DESCRIPTION("NVIDIA Tegra NAND driver");
    MODULE_AUTHOR("Thierry Reding <thierry.reding@nvidia.com>");
    MODULE_AUTHOR("Lucas Stach <dev@lynxeye.de>");
    MODULE_AUTHOR("Stefan Agner <stefan@agner.ch>");
    MODULE_LICENSE("GPL v2");
