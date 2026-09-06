//! Automatically rewritten from C to Rust
//! Source: drivers/mtd/nand/raw/arasan-nand-controller.c
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
// Arasan NAND Flash Controller Driver
//
// Copyright (C) 2014 - 2020 Xilinx, Inc.
// Author:
// Miquel Raynal <miquel.raynal@bootlin.com>
// Original work (fully rewritten):
// Punnaiah Choudary Kalluri <punnaia@xilinx.com>
// Naga Sureshkumar Relli <nagasure@xilinx.com>
//

pub const PKT_REG: c_uint = 0x00;

pub const MEM_ADDR1_REG: c_uint = 0x04;
pub const MEM_ADDR2_REG: c_uint = 0x08;

pub const CMD_REG: c_uint = 0x0C;

pub const PROG_REG: c_uint = 0x10;

pub const INTR_STS_EN_REG: c_uint = 0x14;
pub const INTR_SIG_EN_REG: c_uint = 0x18;
pub const INTR_STS_REG: c_uint = 0x1C;

pub const READY_STS_REG: c_uint = 0x20;
pub const DMA_ADDR0_REG: c_uint = 0x50;
pub const DMA_ADDR1_REG: c_uint = 0x24;
pub const FLASH_STS_REG: c_uint = 0x28;
pub const TIMING_REG: c_uint = 0x2C;
pub const TCCS_TIME_500NS: c_int = 0;
pub const TCCS_TIME_300NS: c_int = 3;
pub const TCCS_TIME_200NS: c_int = 2;
pub const TCCS_TIME_100NS: c_int = 1;

pub const DATA_PORT_REG: c_uint = 0x30;
pub const ECC_CONF_REG: c_uint = 0x34;

pub const ECC_ERR_CNT_REG: c_uint = 0x38;

pub const ECC_SP_REG: c_uint = 0x3C;

pub const ECC_1ERR_CNT_REG: c_uint = 0x40;
pub const ECC_2ERR_CNT_REG: c_uint = 0x44;
pub const DATA_INTERFACE_REG: c_uint = 0x6C;

pub const DIFACE_SDR: c_int = 0;

pub const ANFC_MAX_CS: c_int = 2;
pub const ANFC_DFLT_TIMEOUT_US: c_int = 1000000;

pub const ANFC_RSVD_ECC_BYTES: c_int = 21;
pub const ANFC_XLNX_SDR_DFLT_CORE_CLK: c_int = 100000000;
pub const ANFC_XLNX_SDR_HS_CORE_CLK: c_int = 80000000;
    static struct gpio_desc *anfc_default_cs_array[2] = {core::ptr::null_mut(), core::ptr::null_mut()};
//
// struct anfc_op - Defines how to execute an operation
// @pkt_reg: Packet register
// @addr1_reg: Memory address 1 register
// @addr2_reg: Memory address 2 register
// @cmd_reg: Command register
// @prog_reg: Program register
// @steps: Number of "packets" to read/write
// @rdy_timeout_ms: Timeout for waits on Ready/Busy pin
// @len: Data transfer length
// @read: Data transfer direction from the controller point of view
// @buf: Data buffer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct anfc_op {
    pub pkt_reg: u32,
    pub addr1_reg: u32,
    pub addr2_reg: u32,
    pub cmd_reg: u32,
    pub prog_reg: u32,
    pub steps: c_int,
    pub rdy_timeout_ms: c_uint,
    pub len: c_uint,
    pub read: bool,
    pub buf: *mut u8,
}

//
// struct anand - Defines the NAND chip related information
// @node:		Used to store NAND chips into a list
// @chip:		NAND chip information structure
// @rb:			Ready-busy line
// @page_sz:		Register value of the page_sz field to use
// @clk:		Expected clock frequency to use
// @data_iface:		Data interface timing mode to use
// @timings:		NV-DDR specific timings to use
// @ecc_conf:		Hardware ECC configuration value
// @strength:		Register value of the ECC strength
// @raddr_cycles:	Row address cycle information
// @caddr_cycles:	Column address cycle information
// @ecc_bits:		Exact number of ECC bits per syndrome
// @ecc_total:		Total number of ECC bytes
// @errloc:		Array of errors located with soft BCH
// @hw_ecc:		Buffer to store syndromes computed by hardware
// @bch:		BCH structure
// @cs_idx:		Array of chip-select for this device, values are indexes
// of the controller structure @gpio_cs array
// @ncs_idx:		Size of the @cs_idx array
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct anand {
    pub node: list_head,
    pub chip: nand_chip,
    pub rb: c_uint,
    pub page_sz: c_uint,
    pub clk: c_ulong,
    pub data_iface: u32,
    pub timings: u32,
    pub ecc_conf: u32,
    pub strength: u32,
    pub raddr_cycles: u16,
    pub caddr_cycles: u16,
    pub ecc_bits: c_uint,
    pub ecc_total: c_uint,
    pub errloc: *mut c_uint,
    pub hw_ecc: *mut u8,
    pub bch: *mut bch_control,
    pub cs_idx: *mut c_int,
    pub ncs_idx: c_int,
}

//
// struct arasan_nfc - Defines the Arasan NAND flash controller driver instance
// @dev:		Pointer to the device structure
// @base:		Remapped register area
// @controller_clk:		Pointer to the system clock
// @bus_clk:		Pointer to the flash clock
// @controller:		Base controller structure
// @chips:		List of all NAND chips attached to the controller
// @cur_clk:		Current clock rate
// @cs_array:		CS array. Native CS are left empty, the other cells are
// populated with their corresponding GPIO descriptor.
// @ncs:		Size of @cs_array
// @cur_cs:		Index in @cs_array of the currently in use CS
// @native_cs:		Currently selected native CS
// @spare_cs:		Native CS that is not wired (may be selected when a GPIO
// CS is in use)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arasan_nfc {
    pub dev: *mut device,
    pub base: *mut void __iomem,
    pub controller_clk: *mut clk,
    pub bus_clk: *mut clk,
    pub controller: nand_controller,
    pub chips: list_head,
    pub cur_clk: c_uint,
    pub cs_array: *mut gpio_desc,
    pub ncs: c_uint,
    pub cur_cs: c_int,
    pub native_cs: c_uint,
    pub spare_cs: c_uint,
}

    static struct anand *to_anand(struct nand_chip *nand)
    {
    return container_of(nand, struct anand, chip);
    }
    static struct arasan_nfc *to_anfc(struct nand_controller *ctrl)
    {
    return container_of(ctrl, struct arasan_nfc, controller);
    }
#[no_mangle]
unsafe extern "C" fn anfc_wait_for_event(nfc: *mut arasan_nfc, event: c_uint) -> c_int {
    static int anfc_wait_for_event(struct arasan_nfc *nfc, unsigned int event)
    {
    u32 val;
    int ret;
    ret = readl_relaxed_poll_timeout(nfc.base + INTR_STS_REG, val,
    val & event, 0,
    ANFC_DFLT_TIMEOUT_US);
    if (ret) {
    dev_err(nfc.dev, "Timeout waiting for event 0x%x\n", event);
    return -ETIMEDOUT;
    }
    writel_relaxed(event, nfc.base + INTR_STS_REG);
    return 0;
    }
    static int anfc_wait_for_rb(struct arasan_nfc *nfc, struct nand_chip *chip,
    unsigned int timeout_ms)
    {
    struct anand *anand = to_anand(chip);
    u32 val;
    int ret;
// There is no R/B interrupt, we must poll a register
    ret = readl_relaxed_poll_timeout(nfc.base + READY_STS_REG, val,
    val & BIT(anand.rb),
    1, timeout_ms * 1000);
    if (ret) {
    dev_err(nfc.dev, "Timeout waiting for R/B 0x%x\n",
    readl_relaxed(nfc.base + READY_STS_REG));
    return -ETIMEDOUT;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn anfc_trigger_op(nfc: *mut arasan_nfc, nfc_op: *mut anfc_op) {
    static void anfc_trigger_op(struct arasan_nfc *nfc, struct anfc_op *nfc_op)
    {
    writel_relaxed(nfc_op.pkt_reg, nfc.base + PKT_REG);
    writel_relaxed(nfc_op.addr1_reg, nfc.base + MEM_ADDR1_REG);
    writel_relaxed(nfc_op.addr2_reg, nfc.base + MEM_ADDR2_REG);
    writel_relaxed(nfc_op.cmd_reg, nfc.base + CMD_REG);
    writel_relaxed(nfc_op.prog_reg, nfc.base + PROG_REG);
    }
    static int anfc_pkt_len_config(unsigned int len, unsigned int *steps,
    unsigned int *pktsize)
    {
    unsigned int nb, sz;
    for (nb = 1; nb < ANFC_MAX_STEPS; nb *= 2) {
    sz = len / nb;
    if (sz <= ANFC_MAX_PKT_SIZE)
    break;
    }
    if (sz * nb != len)
    return -ENOTSUPP;
    if (steps)
// steps = nb;
    if (pktsize)
// pktsize = sz;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn anfc_is_gpio_cs(nfc: *mut arasan_nfc, nfc_cs: c_int) -> bool {
    static bool anfc_is_gpio_cs(struct arasan_nfc *nfc, int nfc_cs)
    {
    return nfc_cs >= 0 && nfc.cs_array[nfc_cs];
    }
#[no_mangle]
unsafe extern "C" fn anfc_relative_to_absolute_cs(anand: *mut anand, num: c_int) -> c_int {
    static int anfc_relative_to_absolute_cs(struct anand *anand, int num)
    {
    return anand.cs_idx[num];
    }
#[no_mangle]
unsafe extern "C" fn anfc_assert_cs(nfc: *mut arasan_nfc, nfc_cs_idx: c_uint) {
    static void anfc_assert_cs(struct arasan_nfc *nfc, unsigned int nfc_cs_idx)
    {
// CS did not change: do nothing
    if (nfc.cur_cs == nfc_cs_idx)
    return;
// Deassert the previous CS if it was a GPIO
    if (anfc_is_gpio_cs(nfc, nfc.cur_cs))
    gpiod_set_value_cansleep(nfc.cs_array[nfc.cur_cs], 1);
// Assert the new one
    if (anfc_is_gpio_cs(nfc, nfc_cs_idx)) {
    nfc.native_cs = nfc.spare_cs;
    gpiod_set_value_cansleep(nfc.cs_array[nfc_cs_idx], 0);
    } else {
    nfc.native_cs = nfc_cs_idx;
    }
    nfc.cur_cs = nfc_cs_idx;
    }
#[no_mangle]
unsafe extern "C" fn anfc_select_target(chip: *mut nand_chip, target: c_int) -> c_int {
    static int anfc_select_target(struct nand_chip *chip, int target)
    {
    struct anand *anand = to_anand(chip);
    struct arasan_nfc *nfc = to_anfc(chip.controller);
    let mut nfc_cs_idx: c_uint = anfc_relative_to_absolute_cs(anand, target);
    int ret;
    anfc_assert_cs(nfc, nfc_cs_idx);
// Update the controller timings and the potential ECC configuration
    writel_relaxed(anand.data_iface, nfc.base + DATA_INTERFACE_REG);
    writel_relaxed(anand.timings, nfc.base + TIMING_REG);
// Update clock frequency
    if (nfc.cur_clk != anand.clk) {
    clk_disable_unprepare(nfc.bus_clk);
    ret = clk_set_rate(nfc.bus_clk, anand.clk);
    if (ret) {
    dev_err(nfc.dev, "Failed to change clock rate\n");
    return ret;
    }
    ret = clk_prepare_enable(nfc.bus_clk);
    if (ret) {
    dev_err(nfc.dev,
    "Failed to re-enable the bus clock\n");
    return ret;
    }
    nfc.cur_clk = anand.clk;
    }
    return 0;
    }
//
// When using the embedded hardware ECC engine, the controller is in charge of
// feeding the engine with, first, the ECC residue present in the data array.
// A typical read operation is:
// 1/ Assert the read operation by sending the relevant command/address cycles
// but targeting the column of the first ECC bytes in the OOB area instead of
// the main data directly.
// 2/ After having read the relevant number of ECC bytes, the controller uses
// the RNDOUT/RNDSTART commands which are set into the "ECC Spare Command
// Register" to move the pointer back at the beginning of the main data.
// 3/ It will read the content of the main area for a given size (pktsize) and
// will feed the ECC engine with this buffer again.
// 4/ The ECC engine derives the ECC bytes for the given data and compare them
// with the ones already received. It eventually trigger status flags and
// then set the "Buffer Read Ready" flag.
// 5/ The corrected data is then available for reading from the data port
// register.
//
// The hardware BCH ECC engine is known to be inconstent in BCH mode and never
// reports uncorrectable errors. Because of this bug, we have to use the
// software BCH implementation in the read path.
//
    static int anfc_read_page_hw_ecc(struct nand_chip *chip, u8 *buf,
    int oob_required, int page)
    {
    struct arasan_nfc *nfc = to_anfc(chip.controller);
    struct mtd_info *mtd = nand_to_mtd(chip);
    struct anand *anand = to_anand(chip);
    let mut len: c_uint = mtd.writesize + (oob_required ? mtd.oobsize : 0);
    let mut max_bitflips: c_uint = 0;
    dma_addr_t dma_addr;
    int step, ret;
    struct anfc_op nfc_op = {
    .pkt_reg =
    PKT_SIZE(chip.ecc.size) |
    PKT_STEPS(chip.ecc.steps),
    .addr1_reg =
    (page & 0xFF) << (8 * (anand.caddr_cycles)) |
    (((page >> 8) & 0xFF) << (8 * (1 + anand.caddr_cycles))),
    .addr2_reg =
    ((page >> 16) & 0xFF) |
    ADDR2_STRENGTH(anand.strength) |
    ADDR2_CS(nfc.native_cs),
    .cmd_reg =
    CMD_1(NAND_CMD_READ0) |
    CMD_2(NAND_CMD_READSTART) |
    CMD_PAGE_SIZE(anand.page_sz) |
    CMD_DMA_ENABLE |
    CMD_NADDRS(anand.caddr_cycles +
    anand.raddr_cycles),
    .prog_reg = PROG_PGRD,
    };
    dma_addr = dma_map_single(nfc.dev, (void *)buf, len, DMA_FROM_DEVICE);
    if (dma_mapping_error(nfc.dev, dma_addr)) {
    dev_err(nfc.dev, "Buffer mapping error");
    return -EIO;
    }
    writel_relaxed(lower_32_bits(dma_addr), nfc.base + DMA_ADDR0_REG);
    writel_relaxed(upper_32_bits(dma_addr), nfc.base + DMA_ADDR1_REG);
    anfc_trigger_op(nfc, &nfc_op);
    ret = anfc_wait_for_event(nfc, XFER_COMPLETE);
    dma_unmap_single(nfc.dev, dma_addr, len, DMA_FROM_DEVICE);
    if (ret) {
    dev_err(nfc.dev, "Error reading page %d\n", page);
    return ret;
    }
// Store the raw OOB bytes as well
    ret = nand_change_read_column_op(chip, mtd.writesize, chip.oob_poi,
    mtd.oobsize, 0);
    if (ret)
    return ret;
//
// For each step, compute by softare the BCH syndrome over the raw data.
// Compare the theoretical amount of errors and compare with the
// hardware engine feedback.
//
    for (step = 0; step < chip.ecc.steps; step++) {
    u8 *raw_buf = &buf[step * chip.ecc.size];
    unsigned int bit, byte;
    int bf, i;
// Extract the syndrome, it is not necessarily aligned
    memset(anand.hw_ecc, 0, chip.ecc.bytes);
    nand_extract_bits(anand.hw_ecc, 0,
    &chip.oob_poi[mtd.oobsize - anand.ecc_total],
    anand.ecc_bits * step, anand.ecc_bits);
    bf = bch_decode(anand.bch, raw_buf, chip.ecc.size,
    anand.hw_ecc, core::ptr::null_mut(), core::ptr::null_mut(), anand.errloc);
    if (!bf) {
    continue;
    } else if (bf > 0) {
    for (i = 0; i < bf; i++) {
// Only correct the data, not the syndrome
    if (anand.errloc[i] < (chip.ecc.size * 8)) {
    bit = BIT(anand.errloc[i] & 7);
    byte = anand.errloc[i] >> 3;
    raw_buf[byte] ^= bit;
    }
    }
    mtd.ecc_stats.corrected += bf;
    max_bitflips = max_t(unsigned int, max_bitflips, bf);
    continue;
    }
    bf = nand_check_erased_ecc_chunk(raw_buf, chip.ecc.size,
    anand.hw_ecc, chip.ecc.bytes, core::ptr::null_mut(), 0,
    chip.ecc.strength);
    if (bf > 0) {
    mtd.ecc_stats.corrected += bf;
    max_bitflips = max_t(unsigned int, max_bitflips, bf);
    memset(raw_buf, 0xFF, chip.ecc.size);
    } else if (bf < 0) {
    mtd.ecc_stats.failed++;
    }
    }
    return 0;
    }
    static int anfc_sel_read_page_hw_ecc(struct nand_chip *chip, u8 *buf,
    int oob_required, int page)
    {
    int ret;
    ret = anfc_select_target(chip, chip.cur_cs);
    if (ret)
    return ret;
    return anfc_read_page_hw_ecc(chip, buf, oob_required, page);
    };
    static int anfc_write_page_hw_ecc(struct nand_chip *chip, const u8 *buf,
    int oob_required, int page)
    {
    struct anand *anand = to_anand(chip);
    struct arasan_nfc *nfc = to_anfc(chip.controller);
    struct mtd_info *mtd = nand_to_mtd(chip);
    let mut len: c_uint = mtd.writesize + (oob_required ? mtd.oobsize : 0);
    dma_addr_t dma_addr;
    u8 status;
    int ret;
    struct anfc_op nfc_op = {
    .pkt_reg =
    PKT_SIZE(chip.ecc.size) |
    PKT_STEPS(chip.ecc.steps),
    .addr1_reg =
    (page & 0xFF) << (8 * (anand.caddr_cycles)) |
    (((page >> 8) & 0xFF) << (8 * (1 + anand.caddr_cycles))),
    .addr2_reg =
    ((page >> 16) & 0xFF) |
    ADDR2_STRENGTH(anand.strength) |
    ADDR2_CS(nfc.native_cs),
    .cmd_reg =
    CMD_1(NAND_CMD_SEQIN) |
    CMD_2(NAND_CMD_PAGEPROG) |
    CMD_PAGE_SIZE(anand.page_sz) |
    CMD_DMA_ENABLE |
    CMD_NADDRS(anand.caddr_cycles +
    anand.raddr_cycles) |
    CMD_ECC_ENABLE,
    .prog_reg = PROG_PGPROG,
    };
    writel_relaxed(anand.ecc_conf, nfc.base + ECC_CONF_REG);
    writel_relaxed(ECC_SP_CMD1(NAND_CMD_RNDIN) |
    ECC_SP_ADDRS(anand.caddr_cycles),
    nfc.base + ECC_SP_REG);
    dma_addr = dma_map_single(nfc.dev, (void *)buf, len, DMA_TO_DEVICE);
    if (dma_mapping_error(nfc.dev, dma_addr)) {
    dev_err(nfc.dev, "Buffer mapping error");
    return -EIO;
    }
    writel_relaxed(lower_32_bits(dma_addr), nfc.base + DMA_ADDR0_REG);
    writel_relaxed(upper_32_bits(dma_addr), nfc.base + DMA_ADDR1_REG);
    anfc_trigger_op(nfc, &nfc_op);
    ret = anfc_wait_for_event(nfc, XFER_COMPLETE);
    dma_unmap_single(nfc.dev, dma_addr, len, DMA_TO_DEVICE);
    if (ret) {
    dev_err(nfc.dev, "Error writing page %d\n", page);
    return ret;
    }
// Spare data is not protected
    if (oob_required) {
    ret = nand_write_oob_std(chip, page);
    if (ret)
    return ret;
    }
// Check write status on the chip side
    ret = nand_status_op(chip, &status);
    if (ret)
    return ret;
    if (status & NAND_STATUS_FAIL)
    return -EIO;
    return 0;
    }
    static int anfc_sel_write_page_hw_ecc(struct nand_chip *chip, const u8 *buf,
    int oob_required, int page)
    {
    int ret;
    ret = anfc_select_target(chip, chip.cur_cs);
    if (ret)
    return ret;
    return anfc_write_page_hw_ecc(chip, buf, oob_required, page);
    };
// NAND framework ->exec_op() hooks and related helpers
    static int anfc_parse_instructions(struct nand_chip *chip,
    const struct nand_subop *subop,
    struct anfc_op *nfc_op)
    {
    struct arasan_nfc *nfc = to_anfc(chip.controller);
    struct anand *anand = to_anand(chip);
    const struct nand_op_instr *instr = core::ptr::null_mut();
    let mut first_cmd: bool = true;
    unsigned int op_id;
    int ret, i;
    memset(nfc_op, 0, sizeof(*nfc_op));
    nfc_op.addr2_reg = ADDR2_CS(nfc.native_cs);
    nfc_op.cmd_reg = CMD_PAGE_SIZE(anand.page_sz);
    for (op_id = 0; op_id < subop.ninstrs; op_id++) {
    unsigned int offset, naddrs, pktsize;
    const u8 *addrs;
    u8 *buf;
    instr = &subop.instrs[op_id];
    switch (instr.type) {
    case NAND_OP_CMD_INSTR:
    if (first_cmd)
    nfc_op.cmd_reg |= CMD_1(instr.ctx.cmd.opcode);
    else
    nfc_op.cmd_reg |= CMD_2(instr.ctx.cmd.opcode);
    first_cmd = false;
    break;
    case NAND_OP_ADDR_INSTR:
    offset = nand_subop_get_addr_start_off(subop, op_id);
    naddrs = nand_subop_get_num_addr_cyc(subop, op_id);
    addrs = &instr.ctx.addr.addrs[offset];
    nfc_op.cmd_reg |= CMD_NADDRS(naddrs);
    for (i = 0; i < min(ANFC_MAX_ADDR_CYC, naddrs); i++) {
    if (i < 4)
    nfc_op.addr1_reg |= (u32)addrs[i] << i * 8;
    else
    nfc_op.addr2_reg |= addrs[i];
    }
    break;
    case NAND_OP_DATA_IN_INSTR:
    nfc_op.read = true;
    fallthrough;
    case NAND_OP_DATA_OUT_INSTR:
    offset = nand_subop_get_data_start_off(subop, op_id);
    buf = instr.ctx.data.buf.in;
    nfc_op.buf = &buf[offset];
    nfc_op.len = nand_subop_get_data_len(subop, op_id);
    ret = anfc_pkt_len_config(nfc_op.len, &nfc_op.steps,
    &pktsize);
    if (ret)
    return ret;
//
// Number of DATA cycles must be aligned on 4, this
// means the controller might read/write more than
// requested. This is harmless most of the time as extra
// DATA are discarded in the write path and read pointer
// adjusted in the read path.
//
// FIXME: The core should mark operations where
// reading/writing more is allowed so the exec_op()
// implementation can take the right decision when the
// alignment constraint is not met: adjust the number of
// DATA cycles when it's allowed, reject the operation
// otherwise.
//
    nfc_op.pkt_reg |= PKT_SIZE(round_up(pktsize, 4)) |
    PKT_STEPS(nfc_op.steps);
    break;
    case NAND_OP_WAITRDY_INSTR:
    nfc_op.rdy_timeout_ms = instr.ctx.waitrdy.timeout_ms;
    break;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn anfc_rw_pio_op(nfc: *mut arasan_nfc, nfc_op: *mut anfc_op) -> c_int {
    static int anfc_rw_pio_op(struct arasan_nfc *nfc, struct anfc_op *nfc_op)
    {
    let mut dwords: c_uint = (nfc_op.len / 4) / nfc_op.steps;
    let mut last_len: c_uint = nfc_op.len % 4;
    unsigned int offset, dir;
    u8 *buf = nfc_op.buf;
    int ret, i;
    for (i = 0; i < nfc_op.steps; i++) {
    dir = nfc_op.read ? READ_READY : WRITE_READY;
    ret = anfc_wait_for_event(nfc, dir);
    if (ret) {
    dev_err(nfc.dev, "PIO %s ready signal not received\n",
    nfc_op.read ? "Read" : "Write");
    return ret;
    }
    offset = i * (dwords * 4);
    if (nfc_op.read)
    ioread32_rep(nfc.base + DATA_PORT_REG, &buf[offset],
    dwords);
    else
    iowrite32_rep(nfc.base + DATA_PORT_REG, &buf[offset],
    dwords);
    }
    if (last_len) {
    u32 remainder;
    offset = nfc_op.len - last_len;
    if (nfc_op.read) {
    remainder = readl_relaxed(nfc.base + DATA_PORT_REG);
    memcpy(&buf[offset], &remainder, last_len);
    } else {
    memcpy(&remainder, &buf[offset], last_len);
    writel_relaxed(remainder, nfc.base + DATA_PORT_REG);
    }
    }
    return anfc_wait_for_event(nfc, XFER_COMPLETE);
    }
    static int anfc_misc_data_type_exec(struct nand_chip *chip,
    const struct nand_subop *subop,
    u32 prog_reg)
    {
    struct arasan_nfc *nfc = to_anfc(chip.controller);
    let mut nfc_op: anfc_op = {};
    int ret;
    ret = anfc_parse_instructions(chip, subop, &nfc_op);
    if (ret)
    return ret;
    nfc_op.prog_reg = prog_reg;
    anfc_trigger_op(nfc, &nfc_op);
    if (nfc_op.rdy_timeout_ms) {
    ret = anfc_wait_for_rb(nfc, chip, nfc_op.rdy_timeout_ms);
    if (ret)
    return ret;
    }
    return anfc_rw_pio_op(nfc, &nfc_op);
    }
    static int anfc_param_read_type_exec(struct nand_chip *chip,
    const struct nand_subop *subop)
    {
    return anfc_misc_data_type_exec(chip, subop, PROG_RDPARAM);
    }
    static int anfc_data_read_type_exec(struct nand_chip *chip,
    const struct nand_subop *subop)
    {
    let mut prog_reg: u32 = PROG_PGRD;
//
// Experience shows that while in SDR mode sending a CHANGE READ COLUMN
// command through the READ PAGE "type" always works fine, when in
// NV-DDR mode the same command simply fails. However, it was also
// spotted that any CHANGE READ COLUMN command sent through the CHANGE
// READ COLUMN ENHANCED "type" would correctly work in both cases (SDR
// and NV-DDR). So, for simplicity, let's program the controller with
// the CHANGE READ COLUMN ENHANCED "type" whenever we are requested to
// perform a CHANGE READ COLUMN operation.
//
    if (subop.instrs[0].ctx.cmd.opcode == NAND_CMD_RNDOUT &&
    subop.instrs[2].ctx.cmd.opcode == NAND_CMD_RNDOUTSTART)
    prog_reg = PROG_CHG_RD_COL_ENH;
    return anfc_misc_data_type_exec(chip, subop, prog_reg);
    }
    static int anfc_param_write_type_exec(struct nand_chip *chip,
    const struct nand_subop *subop)
    {
    return anfc_misc_data_type_exec(chip, subop, PROG_SET_FEATURE);
    }
    static int anfc_data_write_type_exec(struct nand_chip *chip,
    const struct nand_subop *subop)
    {
    return anfc_misc_data_type_exec(chip, subop, PROG_PGPROG);
    }
    static int anfc_misc_zerolen_type_exec(struct nand_chip *chip,
    const struct nand_subop *subop,
    u32 prog_reg)
    {
    struct arasan_nfc *nfc = to_anfc(chip.controller);
    let mut nfc_op: anfc_op = {};
    int ret;
    ret = anfc_parse_instructions(chip, subop, &nfc_op);
    if (ret)
    return ret;
    nfc_op.prog_reg = prog_reg;
    anfc_trigger_op(nfc, &nfc_op);
    ret = anfc_wait_for_event(nfc, XFER_COMPLETE);
    if (ret)
    return ret;
    if (nfc_op.rdy_timeout_ms)
    ret = anfc_wait_for_rb(nfc, chip, nfc_op.rdy_timeout_ms);
    return ret;
    }
    static int anfc_status_type_exec(struct nand_chip *chip,
    const struct nand_subop *subop)
    {
    struct arasan_nfc *nfc = to_anfc(chip.controller);
    u32 tmp;
    int ret;
// See anfc_check_op() for details about this constraint
    if (subop.instrs[0].ctx.cmd.opcode != NAND_CMD_STATUS)
    return -ENOTSUPP;
    ret = anfc_misc_zerolen_type_exec(chip, subop, PROG_STATUS);
    if (ret)
    return ret;
    tmp = readl_relaxed(nfc.base + FLASH_STS_REG);
    memcpy(subop.instrs[1].ctx.data.buf.in, &tmp, 1);
    return 0;
    }
    static int anfc_reset_type_exec(struct nand_chip *chip,
    const struct nand_subop *subop)
    {
    return anfc_misc_zerolen_type_exec(chip, subop, PROG_RST);
    }
    static int anfc_erase_type_exec(struct nand_chip *chip,
    const struct nand_subop *subop)
    {
    return anfc_misc_zerolen_type_exec(chip, subop, PROG_ERASE);
    }
    static int anfc_wait_type_exec(struct nand_chip *chip,
    const struct nand_subop *subop)
    {
    struct arasan_nfc *nfc = to_anfc(chip.controller);
    let mut nfc_op: anfc_op = {};
    int ret;
    ret = anfc_parse_instructions(chip, subop, &nfc_op);
    if (ret)
    return ret;
    return anfc_wait_for_rb(nfc, chip, nfc_op.rdy_timeout_ms);
    }
    static const struct nand_op_parser anfc_op_parser = NAND_OP_PARSER(
    NAND_OP_PARSER_PATTERN(
    anfc_param_read_type_exec,
    NAND_OP_PARSER_PAT_CMD_ELEM(false),
    NAND_OP_PARSER_PAT_ADDR_ELEM(false, ANFC_MAX_ADDR_CYC),
    NAND_OP_PARSER_PAT_WAITRDY_ELEM(true),
    NAND_OP_PARSER_PAT_DATA_IN_ELEM(false, ANFC_MAX_CHUNK_SIZE)),
    NAND_OP_PARSER_PATTERN(
    anfc_param_write_type_exec,
    NAND_OP_PARSER_PAT_CMD_ELEM(false),
    NAND_OP_PARSER_PAT_ADDR_ELEM(false, ANFC_MAX_ADDR_CYC),
    NAND_OP_PARSER_PAT_DATA_OUT_ELEM(false, ANFC_MAX_PARAM_SIZE)),
    NAND_OP_PARSER_PATTERN(
    anfc_data_read_type_exec,
    NAND_OP_PARSER_PAT_CMD_ELEM(false),
    NAND_OP_PARSER_PAT_ADDR_ELEM(false, ANFC_MAX_ADDR_CYC),
    NAND_OP_PARSER_PAT_CMD_ELEM(false),
    NAND_OP_PARSER_PAT_WAITRDY_ELEM(true),
    NAND_OP_PARSER_PAT_DATA_IN_ELEM(true, ANFC_MAX_CHUNK_SIZE)),
    NAND_OP_PARSER_PATTERN(
    anfc_data_write_type_exec,
    NAND_OP_PARSER_PAT_CMD_ELEM(false),
    NAND_OP_PARSER_PAT_ADDR_ELEM(false, ANFC_MAX_ADDR_CYC),
    NAND_OP_PARSER_PAT_DATA_OUT_ELEM(false, ANFC_MAX_CHUNK_SIZE),
    NAND_OP_PARSER_PAT_CMD_ELEM(false)),
    NAND_OP_PARSER_PATTERN(
    anfc_reset_type_exec,
    NAND_OP_PARSER_PAT_CMD_ELEM(false),
    NAND_OP_PARSER_PAT_WAITRDY_ELEM(false)),
    NAND_OP_PARSER_PATTERN(
    anfc_erase_type_exec,
    NAND_OP_PARSER_PAT_CMD_ELEM(false),
    NAND_OP_PARSER_PAT_ADDR_ELEM(false, ANFC_MAX_ADDR_CYC),
    NAND_OP_PARSER_PAT_CMD_ELEM(false),
    NAND_OP_PARSER_PAT_WAITRDY_ELEM(false)),
    NAND_OP_PARSER_PATTERN(
    anfc_status_type_exec,
    NAND_OP_PARSER_PAT_CMD_ELEM(false),
    NAND_OP_PARSER_PAT_DATA_IN_ELEM(false, ANFC_MAX_CHUNK_SIZE)),
    NAND_OP_PARSER_PATTERN(
    anfc_wait_type_exec,
    NAND_OP_PARSER_PAT_WAITRDY_ELEM(false)),
    );
    static int anfc_check_op(struct nand_chip *chip,
    const struct nand_operation *op)
    {
    const struct nand_op_instr *instr;
    int op_id;
//
// The controller abstracts all the NAND operations and do not support
// data only operations.
//
// TODO: The nand_op_parser framework should be extended to
// support custom checks on DATA instructions.
//
    for (op_id = 0; op_id < op.ninstrs; op_id++) {
    instr = &op.instrs[op_id];
    switch (instr.type) {
    case NAND_OP_ADDR_INSTR:
    if (instr.ctx.addr.naddrs > ANFC_MAX_ADDR_CYC)
    return -ENOTSUPP;
    break;
    case NAND_OP_DATA_IN_INSTR:
    case NAND_OP_DATA_OUT_INSTR:
    if (instr.ctx.data.len > ANFC_MAX_CHUNK_SIZE)
    return -ENOTSUPP;
    if (anfc_pkt_len_config(instr.ctx.data.len, core::ptr::null_mut(), core::ptr::null_mut()))
    return -ENOTSUPP;
    break;
    default:
    break;
    }
    }
//
// The controller does not allow to proceed with a CMD+DATA_IN cycle
// manually on the bus by reading data from the data register. Instead,
// the controller abstract a status read operation with its own status
// register after ordering a read status operation. Hence, we cannot
// support any CMD+DATA_IN operation other than a READ STATUS.
//
// TODO: The nand_op_parser() framework should be extended to describe
// fixed patterns instead of open-coding this check here.
//
    if (op.ninstrs == 2 &&
    op.instrs[0].type == NAND_OP_CMD_INSTR &&
    op.instrs[0].ctx.cmd.opcode != NAND_CMD_STATUS &&
    op.instrs[1].type == NAND_OP_DATA_IN_INSTR)
    return -ENOTSUPP;
    return nand_op_parser_exec_op(chip, &anfc_op_parser, op, true);
    }
    static int anfc_exec_op(struct nand_chip *chip,
    const struct nand_operation *op,
    bool check_only)
    {
    int ret;
    if (check_only)
    return anfc_check_op(chip, op);
    ret = anfc_select_target(chip, op.cs);
    if (ret)
    return ret;
    return nand_op_parser_exec_op(chip, &anfc_op_parser, op, check_only);
    }
    static int anfc_setup_interface(struct nand_chip *chip, int target,
    const struct nand_interface_config *conf)
    {
    struct anand *anand = to_anand(chip);
    struct arasan_nfc *nfc = to_anfc(chip.controller);
    struct device_node *np = nfc.dev.of_node;
    const struct nand_sdr_timings *sdr;
    const struct nand_nvddr_timings *nvddr;
    unsigned int tccs_min, dqs_mode, fast_tcad;
    if (nand_interface_is_nvddr(conf)) {
    nvddr = nand_get_nvddr_timings(conf);
    if (IS_ERR(nvddr))
    return PTR_ERR(nvddr);
    } else {
    sdr = nand_get_sdr_timings(conf);
    if (IS_ERR(sdr))
    return PTR_ERR(sdr);
    }
    if (target < 0)
    return 0;
    if (nand_interface_is_sdr(conf)) {
    anand.data_iface = DIFACE_SDR |
    DIFACE_SDR_MODE(conf.timings.mode);
    anand.timings = 0;
    } else {
    anand.data_iface = DIFACE_NVDDR |
    DIFACE_DDR_MODE(conf.timings.mode);
    if (conf.timings.nvddr.tCCS_min <= 100000)
    tccs_min = TCCS_TIME_100NS;
#[no_mangle]
pub unsafe extern "C" fn if(200000: conf->timings.nvddr.tCCS_min <=) -> else {
    else if (conf.timings.nvddr.tCCS_min <= 200000)
    tccs_min = TCCS_TIME_200NS;
#[no_mangle]
pub unsafe extern "C" fn if(300000: conf->timings.nvddr.tCCS_min <=) -> else {
    else if (conf.timings.nvddr.tCCS_min <= 300000)
    tccs_min = TCCS_TIME_300NS;
    else
    tccs_min = TCCS_TIME_500NS;
    fast_tcad = 0;
    if (conf.timings.nvddr.tCAD_min < 45000)
    fast_tcad = FAST_TCAD;
    switch (conf.timings.mode) {
    case 5:
    case 4:
    dqs_mode = 2;
    break;
    case 3:
    dqs_mode = 3;
    break;
    case 2:
    dqs_mode = 4;
    break;
    case 1:
    dqs_mode = 5;
    break;
    case 0:
    default:
    dqs_mode = 6;
    break;
    }
    anand.timings = tccs_min | fast_tcad |
    DQS_BUFF_SEL_IN(dqs_mode) |
    DQS_BUFF_SEL_OUT(dqs_mode);
    }
    if (nand_interface_is_sdr(conf)) {
    anand.clk = ANFC_XLNX_SDR_DFLT_CORE_CLK;
    } else {
// ONFI timings are defined in picoseconds
    anand.clk = div_u64((u64)NSEC_PER_SEC * 1000,
    conf.timings.nvddr.tCK_min);
    }
//
// Due to a hardware bug in the ZynqMP SoC, SDR timing modes 0-1 work
// with f > 90MHz (default clock is 100MHz) but signals are unstable
// with higher modes. Hence we decrease a little bit the clock rate to
// 80MHz when using SDR modes 2-5 with this SoC.
//
    if (of_device_is_compatible(np, "xlnx,zynqmp-nand-controller") &&
    nand_interface_is_sdr(conf) && conf.timings.mode >= 2)
    anand.clk = ANFC_XLNX_SDR_HS_CORE_CLK;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn anfc_calc_hw_ecc_bytes(step_size: c_int, strength: c_int) -> c_int {
    static int anfc_calc_hw_ecc_bytes(int step_size, int strength)
    {
    unsigned int bch_gf_mag, ecc_bits;
    switch (step_size) {
    case SZ_512:
    bch_gf_mag = 13;
    break;
    case SZ_1K:
    bch_gf_mag = 14;
    break;
    default:
    return -EINVAL;
    }
    ecc_bits = bch_gf_mag * strength;
    return DIV_ROUND_UP(ecc_bits, 8);
    }
    static const int anfc_hw_ecc_512_strengths[] = {4, 8, 12};
    static const int anfc_hw_ecc_1024_strengths[] = {24};
    static const struct nand_ecc_step_info anfc_hw_ecc_step_infos[] = {
    {
    .stepsize = SZ_512,
    .strengths = anfc_hw_ecc_512_strengths,
    .nstrengths = ARRAY_SIZE(anfc_hw_ecc_512_strengths),
    },
    {
    .stepsize = SZ_1K,
    .strengths = anfc_hw_ecc_1024_strengths,
    .nstrengths = ARRAY_SIZE(anfc_hw_ecc_1024_strengths),
    },
    };
    static const struct nand_ecc_caps anfc_hw_ecc_caps = {
    .stepinfos = anfc_hw_ecc_step_infos,
    .nstepinfos = ARRAY_SIZE(anfc_hw_ecc_step_infos),
    .calc_ecc_bytes = anfc_calc_hw_ecc_bytes,
    };
    static int anfc_init_hw_ecc_controller(struct arasan_nfc *nfc,
    struct nand_chip *chip)
    {
    struct anand *anand = to_anand(chip);
    struct mtd_info *mtd = nand_to_mtd(chip);
    struct nand_ecc_ctrl *ecc = &chip.ecc;
    let mut bch_prim_poly: c_uint = 0, bch_gf_mag = 0, ecc_offset;
    int ret;
    switch (mtd.writesize) {
    case SZ_512:
    case SZ_2K:
    case SZ_4K:
    case SZ_8K:
    case SZ_16K:
    break;
    default:
    dev_err(nfc.dev, "Unsupported page size %d\n", mtd.writesize);
    return -EINVAL;
    }
    ret = nand_ecc_choose_conf(chip, &anfc_hw_ecc_caps, mtd.oobsize);
    if (ret)
    return ret;
    switch (ecc.strength) {
    case 12:
    anand.strength = 0x1;
    break;
    case 8:
    anand.strength = 0x2;
    break;
    case 4:
    anand.strength = 0x3;
    break;
    case 24:
    anand.strength = 0x4;
    break;
    default:
    dev_err(nfc.dev, "Unsupported strength %d\n", ecc.strength);
    return -EINVAL;
    }
    switch (ecc.size) {
    case SZ_512:
    bch_gf_mag = 13;
    bch_prim_poly = 0x201b;
    break;
    case SZ_1K:
    bch_gf_mag = 14;
    bch_prim_poly = 0x4443;
    break;
    default:
    dev_err(nfc.dev, "Unsupported step size %d\n", ecc.strength);
    return -EINVAL;
    }
    mtd_set_ooblayout(mtd, nand_get_large_page_ooblayout());
    ecc.steps = mtd.writesize / ecc.size;
    ecc.algo = NAND_ECC_ALGO_BCH;
    anand.ecc_bits = bch_gf_mag * ecc.strength;
    ecc.bytes = DIV_ROUND_UP(anand.ecc_bits, 8);
    anand.ecc_total = DIV_ROUND_UP(anand.ecc_bits * ecc.steps, 8);
    ecc_offset = mtd.writesize + mtd.oobsize - anand.ecc_total;
    anand.ecc_conf = ECC_CONF_COL(ecc_offset) |
    ECC_CONF_LEN(anand.ecc_total) |
    ECC_CONF_BCH_EN;
    anand.errloc = devm_kmalloc_array(nfc.dev, ecc.strength,
    sizeof(*anand.errloc), GFP_KERNEL);
    if (!anand.errloc)
    return -ENOMEM;
    anand.hw_ecc = devm_kmalloc(nfc.dev, ecc.bytes, GFP_KERNEL);
    if (!anand.hw_ecc)
    return -ENOMEM;
// Enforce bit swapping to fit the hardware
    anand.bch = bch_init(bch_gf_mag, ecc.strength, bch_prim_poly, true);
    if (!anand.bch)
    return -EINVAL;
    ecc.read_page = anfc_sel_read_page_hw_ecc;
    ecc.write_page = anfc_sel_write_page_hw_ecc;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn anfc_attach_chip(chip: *mut nand_chip) -> c_int {
    static int anfc_attach_chip(struct nand_chip *chip)
    {
    struct anand *anand = to_anand(chip);
    struct arasan_nfc *nfc = to_anfc(chip.controller);
    struct mtd_info *mtd = nand_to_mtd(chip);
    let mut ret: c_int = 0;
    if (mtd.writesize <= SZ_512)
    anand.caddr_cycles = 1;
    else
    anand.caddr_cycles = 2;
    if (chip.options & NAND_ROW_ADDR_3)
    anand.raddr_cycles = 3;
    else
    anand.raddr_cycles = 2;
    switch (mtd.writesize) {
    case 512:
    anand.page_sz = 0;
    break;
    case 1024:
    anand.page_sz = 5;
    break;
    case 2048:
    anand.page_sz = 1;
    break;
    case 4096:
    anand.page_sz = 2;
    break;
    case 8192:
    anand.page_sz = 3;
    break;
    case 16384:
    anand.page_sz = 4;
    break;
    default:
    return -EINVAL;
    }
// These hooks are valid for all ECC providers
    chip.ecc.read_page_raw = nand_monolithic_read_page_raw;
    chip.ecc.write_page_raw = nand_monolithic_write_page_raw;
    switch (chip.ecc.engine_type) {
    case NAND_ECC_ENGINE_TYPE_NONE:
    case NAND_ECC_ENGINE_TYPE_SOFT:
    case NAND_ECC_ENGINE_TYPE_ON_DIE:
    break;
    case NAND_ECC_ENGINE_TYPE_ON_HOST:
    ret = anfc_init_hw_ecc_controller(nfc, chip);
    break;
    default:
    dev_err(nfc.dev, "Unsupported ECC mode: %d\n",
    chip.ecc.engine_type);
    return -EINVAL;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn anfc_detach_chip(chip: *mut nand_chip) {
    static void anfc_detach_chip(struct nand_chip *chip)
    {
    struct anand *anand = to_anand(chip);
    if (anand.bch)
    bch_free(anand.bch);
    }
    static const struct nand_controller_ops anfc_ops = {
    .exec_op = anfc_exec_op,
    .setup_interface = anfc_setup_interface,
    .attach_chip = anfc_attach_chip,
    .detach_chip = anfc_detach_chip,
    };
#[no_mangle]
unsafe extern "C" fn anfc_chip_init(nfc: *mut arasan_nfc, np: *mut device_node) -> c_int {
    static int anfc_chip_init(struct arasan_nfc *nfc, struct device_node *np)
    {
    struct anand *anand;
    struct nand_chip *chip;
    struct mtd_info *mtd;
    int rb, ret, i;
    anand = devm_kzalloc(nfc.dev, sizeof(*anand), GFP_KERNEL);
    if (!anand)
    return -ENOMEM;
// Chip-select init
    anand.ncs_idx = of_property_count_elems_of_size(np, "reg", sizeof(u32));
    if (anand.ncs_idx <= 0 || anand.ncs_idx > nfc.ncs) {
    dev_err(nfc.dev, "Invalid reg property\n");
    return -EINVAL;
    }
    anand.cs_idx = devm_kcalloc(nfc.dev, anand.ncs_idx,
    sizeof(*anand.cs_idx), GFP_KERNEL);
    if (!anand.cs_idx)
    return -ENOMEM;
    for (i = 0; i < anand.ncs_idx; i++) {
    ret = of_property_read_u32_index(np, "reg", i,
    &anand.cs_idx[i]);
    if (ret) {
    dev_err(nfc.dev, "invalid CS property: %d\n", ret);
    return ret;
    }
    }
// Ready-busy init
    ret = of_property_read_u32(np, "nand-rb", &rb);
    if (ret)
    return ret;
    if (rb >= ANFC_MAX_CS) {
    dev_err(nfc.dev, "Wrong RB %d\n", rb);
    return -EINVAL;
    }
    anand.rb = rb;
    chip = &anand.chip;
    mtd = nand_to_mtd(chip);
    mtd.dev.parent = nfc.dev;
    chip.controller = &nfc.controller;
    chip.options = NAND_BUSWIDTH_AUTO | NAND_NO_SUBPAGE_WRITE |
    NAND_USES_DMA;
    nand_set_flash_node(chip, np);
    if (!mtd.name) {
    dev_err(nfc.dev, "NAND label property is mandatory\n");
    return -EINVAL;
    }
    ret = nand_scan(chip, anand.ncs_idx);
    if (ret) {
    dev_err(nfc.dev, "Scan operation failed\n");
    return ret;
    }
    ret = mtd_device_register(mtd, core::ptr::null_mut(), 0);
    if (ret) {
    nand_cleanup(chip);
    return ret;
    }
    list_add_tail(&anand.node, &nfc.chips);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn anfc_chips_cleanup(nfc: *mut arasan_nfc) {
    static void anfc_chips_cleanup(struct arasan_nfc *nfc)
    {
    struct anand *anand, *tmp;
    struct nand_chip *chip;
    int ret;
    list_for_each_entry_safe(anand, tmp, &nfc.chips, node) {
    chip = &anand.chip;
    ret = mtd_device_unregister(nand_to_mtd(chip));
    WARN_ON(ret);
    nand_cleanup(chip);
    list_del(&anand.node);
    }
    }
#[no_mangle]
unsafe extern "C" fn anfc_chips_init(nfc: *mut arasan_nfc) -> c_int {
    static int anfc_chips_init(struct arasan_nfc *nfc)
    {
    struct device_node *np = nfc.dev.of_node;
    let mut nchips: c_int = of_get_child_count(np);
    int ret;
    if (!nchips) {
    dev_err(nfc.dev, "Incorrect number of NAND chips (%d)\n",
    nchips);
    return -EINVAL;
    }
    for_each_child_of_node_scoped(np, nand_np) {
    ret = anfc_chip_init(nfc, nand_np);
    if (ret) {
    anfc_chips_cleanup(nfc);
    break;
    }
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn anfc_reset(nfc: *mut arasan_nfc) {
    static void anfc_reset(struct arasan_nfc *nfc)
    {
// Disable interrupt signals
    writel_relaxed(0, nfc.base + INTR_SIG_EN_REG);
// Enable interrupt status
    writel_relaxed(EVENT_MASK, nfc.base + INTR_STS_EN_REG);
    nfc.cur_cs = -1;
    }
#[no_mangle]
unsafe extern "C" fn anfc_parse_cs(nfc: *mut arasan_nfc) -> c_int {
    static int anfc_parse_cs(struct arasan_nfc *nfc)
    {
    int ret;
// Check the gpio-cs property
    ret = rawnand_dt_parse_gpio_cs(nfc.dev, &nfc.cs_array, &nfc.ncs);
    if (ret)
    return ret;
//
// The controller native CS cannot be both disabled at the same time.
// Hence, only one native CS can be used if GPIO CS are needed, so that
// the other is selected when a non-native CS must be asserted (not
// wired physically or configured as GPIO instead of NAND CS). In this
// case, the "not" chosen CS is assigned to nfc->spare_cs and selected
// whenever a GPIO CS must be asserted.
//
    if (nfc.cs_array) {
    if (nfc.ncs > 2 && !nfc.cs_array[0] && !nfc.cs_array[1]) {
    dev_err(nfc.dev,
    "Assign a single native CS when using GPIOs\n");
    return -EINVAL;
    }
    if (nfc.cs_array[0])
    nfc.spare_cs = 0;
    else
    nfc.spare_cs = 1;
    }
    if (!nfc.cs_array) {
    nfc.cs_array = anfc_default_cs_array;
    nfc.ncs = ANFC_MAX_CS;
    return 0;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn anfc_probe(pdev: *mut platform_device) -> c_int {
    static int anfc_probe(struct platform_device *pdev)
    {
    struct arasan_nfc *nfc;
    int ret;
    nfc = devm_kzalloc(&pdev.dev, sizeof(*nfc), GFP_KERNEL);
    if (!nfc)
    return -ENOMEM;
    nfc.dev = &pdev.dev;
    nand_controller_init(&nfc.controller);
    nfc.controller.ops = &anfc_ops;
    INIT_LIST_HEAD(&nfc.chips);
    nfc.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(nfc.base))
    return PTR_ERR(nfc.base);
    anfc_reset(nfc);
    nfc.controller_clk = devm_clk_get_enabled(&pdev.dev, "controller");
    if (IS_ERR(nfc.controller_clk))
    return PTR_ERR(nfc.controller_clk);
    nfc.bus_clk = devm_clk_get_enabled(&pdev.dev, "bus");
    if (IS_ERR(nfc.bus_clk))
    return PTR_ERR(nfc.bus_clk);
    ret = dma_set_mask(&pdev.dev, DMA_BIT_MASK(64));
    if (ret)
    return ret;
    ret = anfc_parse_cs(nfc);
    if (ret)
    return ret;
    ret = anfc_chips_init(nfc);
    if (ret)
    return ret;
    platform_set_drvdata(pdev, nfc);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn anfc_remove(pdev: *mut platform_device) {
    static void anfc_remove(struct platform_device *pdev)
    {
    int i;
    struct arasan_nfc *nfc = platform_get_drvdata(pdev);
    for (i = 0; i < nfc.ncs; i++) {
    if (nfc.cs_array[i]) {
    gpiod_put(nfc.cs_array[i]);
    }
    }
    anfc_chips_cleanup(nfc);
    }
    static const struct of_device_id anfc_ids[] = {
    {
    .compatible = "xlnx,zynqmp-nand-controller",
    },
    {
    .compatible = "arasan,nfc-v3p10",
    },
    {}
    };
    MODULE_DEVICE_TABLE(of, anfc_ids);
    static struct platform_driver anfc_driver = {
    .driver = {
    .name = "arasan-nand-controller",
    .of_match_table = anfc_ids,
    },
    .probe = anfc_probe,
    .remove = anfc_remove,
    };
    module_platform_driver(anfc_driver);
    MODULE_LICENSE("GPL v2");
    MODULE_AUTHOR("Punnaiah Choudary Kalluri <punnaia@xilinx.com>");
    MODULE_AUTHOR("Naga Sureshkumar Relli <nagasure@xilinx.com>");
    MODULE_AUTHOR("Miquel Raynal <miquel.raynal@bootlin.com>");
    MODULE_DESCRIPTION("Arasan NAND Flash Controller Driver");
