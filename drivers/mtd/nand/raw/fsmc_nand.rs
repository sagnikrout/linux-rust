//! Automatically rewritten from C to Rust
//! Source: drivers/mtd/nand/raw/fsmc_nand.c
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
// ST Microelectronics
// Flexible Static Memory Controller (FSMC)
// Driver for NAND portions
//
// Copyright © 2010 ST Microelectronics
// Vipin Kumar <vipin.kumar@st.com>
// Ashish Priyadarshi
//
// Based on drivers/mtd/nand/nomadik_nand.c (removed in v3.8)
// Copyright © 2007 STMicroelectronics Pvt. Ltd.
// Copyright © 2009 Alessandro Rubini
//

// fsmc controller registers for NOR flash
pub const CTRL: c_uint = 0x0;
// ctrl register definitions

pub const CTRL_TIM: c_uint = 0x4;
// ctrl_tim register definitions
pub const FSMC_NOR_BANK_SZ: c_uint = 0x8;
pub const FSMC_NOR_REG_SIZE: c_uint = 0x40;

    (FSMC_NOR_BANK_SZ * (bank)) +	\
    (reg))
// fsmc controller registers for NAND flash
pub const FSMC_PC: c_uint = 0x00;
// pc register definitions

pub const STS: c_uint = 0x04;
// sts register definitions

pub const COMM: c_uint = 0x08;
// comm register definitions
pub const FSMC_TSET_SHIFT: c_int = 0;
pub const FSMC_TSET_MASK: c_uint = 0xFF;
pub const FSMC_TWAIT_SHIFT: c_int = 8;
pub const FSMC_TWAIT_MASK: c_uint = 0xFF;
pub const FSMC_THOLD_SHIFT: c_int = 16;
pub const FSMC_THOLD_MASK: c_uint = 0xFF;
pub const FSMC_THIZ_SHIFT: c_int = 24;
pub const FSMC_THIZ_MASK: c_uint = 0xFF;
pub const ATTRIB: c_uint = 0x0C;
pub const IOATA: c_uint = 0x10;
pub const ECC1: c_uint = 0x14;
pub const ECC2: c_uint = 0x18;
pub const ECC3: c_uint = 0x1C;
pub const FSMC_NAND_BANK_SZ: c_uint = 0x20;

//
// According to SPEAr300 Reference Manual (RM0082)
// TOUDEL = 7ns (Output delay from the flip-flops to the board)
// TINDEL = 5ns (Input delay from the board to the flipflop)
//
pub const TOUTDEL: c_int = 7000;
pub const TINDEL: c_int = 5000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsmc_nand_timings {
    pub tclr: u8,
    pub tar: u8,
    pub thiz: u8,
    pub thold: u8,
    pub twait: u8,
    pub tset: u8,
}

    enum access_mode {
    USE_DMA_ACCESS = 1,
    USE_WORD_ACCESS,
    };
//
// struct fsmc_nand_data - structure for FSMC NAND device state
//
// @base:		Inherit from the nand_controller struct
// @pid:		Part ID on the AMBA PrimeCell format
// @nand:		Chip related info for a NAND flash.
//
// @bank:		Bank number for probed device.
// @dev:		Parent device
// @mode:		Access mode
// @clk:		Clock structure for FSMC.
//
// @read_dma_chan:	DMA channel for read access
// @write_dma_chan:	DMA channel for write access to NAND
// @dma_access_complete: Completion structure
//
// @dev_timings:	NAND timings
//
// @data_pa:		NAND Physical port for Data.
// @data_va:		NAND port for Data.
// @cmd_va:		NAND port for Command.
// @addr_va:		NAND port for Address.
// @regs_va:		Registers base address for a given bank.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsmc_nand_data {
    pub base: nand_controller,
    pub pid: u32,
    pub nand: nand_chip,
    pub bank: c_uint,
    pub dev: *mut device,
    pub mode: enum access_mode,
    pub clk: *mut clk,
// DMA related objects
    pub read_dma_chan: *mut dma_chan,
    pub write_dma_chan: *mut dma_chan,
    pub dma_access_complete: completion,
    pub dev_timings: *mut fsmc_nand_timings,
    pub data_pa: dma_addr_t,
    pub data_va: *mut void __iomem,
    pub cmd_va: *mut void __iomem,
    pub addr_va: *mut void __iomem,
    pub regs_va: *mut void __iomem,
}

    static int fsmc_ecc1_ooblayout_ecc(struct mtd_info *mtd, int section,
    struct mtd_oob_region *oobregion)
    {
    struct nand_chip *chip = mtd_to_nand(mtd);
    if (section >= chip.ecc.steps)
    return -ERANGE;
    oobregion.offset = (section * 16) + 2;
    oobregion.length = 3;
    return 0;
    }
    static int fsmc_ecc1_ooblayout_free(struct mtd_info *mtd, int section,
    struct mtd_oob_region *oobregion)
    {
    struct nand_chip *chip = mtd_to_nand(mtd);
    if (section >= chip.ecc.steps)
    return -ERANGE;
    oobregion.offset = (section * 16) + 8;
    if (section < chip.ecc.steps - 1)
    oobregion.length = 8;
    else
    oobregion.length = mtd.oobsize - oobregion.offset;
    return 0;
    }
    static const struct mtd_ooblayout_ops fsmc_ecc1_ooblayout_ops = {
    .ecc = fsmc_ecc1_ooblayout_ecc,
    .free = fsmc_ecc1_ooblayout_free,
    };
//
// ECC placement definitions in oobfree type format.
// There are 13 bytes of ecc for every 512 byte block and it has to be read
// consecutively and immediately after the 512 byte data block for hardware to
// generate the error bit offsets in 512 byte data.
//
    static int fsmc_ecc4_ooblayout_ecc(struct mtd_info *mtd, int section,
    struct mtd_oob_region *oobregion)
    {
    struct nand_chip *chip = mtd_to_nand(mtd);
    if (section >= chip.ecc.steps)
    return -ERANGE;
    oobregion.length = chip.ecc.bytes;
    if (!section && mtd.writesize <= 512)
    oobregion.offset = 0;
    else
    oobregion.offset = (section * 16) + 2;
    return 0;
    }
    static int fsmc_ecc4_ooblayout_free(struct mtd_info *mtd, int section,
    struct mtd_oob_region *oobregion)
    {
    struct nand_chip *chip = mtd_to_nand(mtd);
    if (section >= chip.ecc.steps)
    return -ERANGE;
    oobregion.offset = (section * 16) + 15;
    if (section < chip.ecc.steps - 1)
    oobregion.length = 3;
    else
    oobregion.length = mtd.oobsize - oobregion.offset;
    return 0;
    }
    static const struct mtd_ooblayout_ops fsmc_ecc4_ooblayout_ops = {
    .ecc = fsmc_ecc4_ooblayout_ecc,
    .free = fsmc_ecc4_ooblayout_free,
    };
    static inline struct fsmc_nand_data *nand_to_fsmc(struct nand_chip *chip)
    {
    return container_of(chip, struct fsmc_nand_data, nand);
    }
//
// fsmc_nand_setup - FSMC (Flexible Static Memory Controller) init routine
//
// This routine initializes timing parameters related to NAND memory access in
// FSMC registers
//
    static void fsmc_nand_setup(struct fsmc_nand_data *host,
    struct fsmc_nand_timings *tims)
    {
    let mut value: u32 = FSMC_DEVTYPE_NAND | FSMC_ENABLE | FSMC_WAITON;
    u32 tclr, tar, thiz, thold, twait, tset;
    tclr = (tims.tclr & FSMC_TCLR_MASK) << FSMC_TCLR_SHIFT;
    tar = (tims.tar & FSMC_TAR_MASK) << FSMC_TAR_SHIFT;
    thiz = (tims.thiz & FSMC_THIZ_MASK) << FSMC_THIZ_SHIFT;
    thold = (tims.thold & FSMC_THOLD_MASK) << FSMC_THOLD_SHIFT;
    twait = (tims.twait & FSMC_TWAIT_MASK) << FSMC_TWAIT_SHIFT;
    tset = (tims.tset & FSMC_TSET_MASK) << FSMC_TSET_SHIFT;
    if (host.nand.options & NAND_BUSWIDTH_16)
    value |= FSMC_DEVWID_16;
    writel_relaxed(value | tclr | tar, host.regs_va + FSMC_PC);
    writel_relaxed(thiz | thold | twait | tset, host.regs_va + COMM);
    writel_relaxed(thiz | thold | twait | tset, host.regs_va + ATTRIB);
    }
    static int fsmc_calc_timings(struct fsmc_nand_data *host,
    const struct nand_sdr_timings *sdrt,
    struct fsmc_nand_timings *tims)
    {
    let mut hclk: c_ulong = clk_get_rate(host.clk);
    let mut hclkn: c_ulong = NSEC_PER_SEC / hclk;
    u32 thiz, thold, twait, tset, twait_min;
    if (sdrt.tRC_min < 30000)
    return -EOPNOTSUPP;
    tims.tar = DIV_ROUND_UP(sdrt.tAR_min / 1000, hclkn) - 1;
    if (tims.tar > FSMC_TAR_MASK)
    tims.tar = FSMC_TAR_MASK;
    tims.tclr = DIV_ROUND_UP(sdrt.tCLR_min / 1000, hclkn) - 1;
    if (tims.tclr > FSMC_TCLR_MASK)
    tims.tclr = FSMC_TCLR_MASK;
    thiz = sdrt.tCS_min - sdrt.tWP_min;
    tims.thiz = DIV_ROUND_UP(thiz / 1000, hclkn);
    thold = sdrt.tDH_min;
    if (thold < sdrt.tCH_min)
    thold = sdrt.tCH_min;
    if (thold < sdrt.tCLH_min)
    thold = sdrt.tCLH_min;
    if (thold < sdrt.tWH_min)
    thold = sdrt.tWH_min;
    if (thold < sdrt.tALH_min)
    thold = sdrt.tALH_min;
    if (thold < sdrt.tREH_min)
    thold = sdrt.tREH_min;
    tims.thold = DIV_ROUND_UP(thold / 1000, hclkn);
    if (tims.thold == 0)
    tims.thold = 1;
#[no_mangle]
pub unsafe extern "C" fn if(FSMC_THOLD_MASK: tims->thold >) -> else {
    else if (tims.thold > FSMC_THOLD_MASK)
    tims.thold = FSMC_THOLD_MASK;
    tset = max(sdrt.tCS_min - sdrt.tWP_min,
    sdrt.tCEA_max - sdrt.tREA_max);
    tims.tset = DIV_ROUND_UP(tset / 1000, hclkn) - 1;
    if (tims.tset == 0)
    tims.tset = 1;
#[no_mangle]
pub unsafe extern "C" fn if(FSMC_TSET_MASK: tims->tset >) -> else {
    else if (tims.tset > FSMC_TSET_MASK)
    tims.tset = FSMC_TSET_MASK;
//
// According to SPEAr300 Reference Manual (RM0082) which gives more
// information related to FSMSC timings than the SPEAr600 one (RM0305),
// twait >= tCEA - (tset * TCLK) + TOUTDEL + TINDEL
//
    twait_min = sdrt.tCEA_max - ((tims.tset + 1) * hclkn * 1000)
    + TOUTDEL + TINDEL;
    twait = max3(sdrt.tRP_min, sdrt.tWP_min, twait_min);
    tims.twait = DIV_ROUND_UP(twait / 1000, hclkn) - 1;
    if (tims.twait == 0)
    tims.twait = 1;
#[no_mangle]
pub unsafe extern "C" fn if(FSMC_TWAIT_MASK: tims->twait >) -> else {
    else if (tims.twait > FSMC_TWAIT_MASK)
    tims.twait = FSMC_TWAIT_MASK;
    return 0;
    }
    static int fsmc_setup_interface(struct nand_chip *nand, int csline,
    const struct nand_interface_config *conf)
    {
    struct fsmc_nand_data *host = nand_to_fsmc(nand);
    struct fsmc_nand_timings tims;
    const struct nand_sdr_timings *sdrt;
    int ret;
    sdrt = nand_get_sdr_timings(conf);
    if (IS_ERR(sdrt))
    return PTR_ERR(sdrt);
    ret = fsmc_calc_timings(host, sdrt, &tims);
    if (ret)
    return ret;
    if (csline == NAND_DATA_IFACE_CHECK_ONLY)
    return 0;
    fsmc_nand_setup(host, &tims);
    return 0;
    }
//
// fsmc_enable_hwecc - Enables Hardware ECC through FSMC registers
//
#[no_mangle]
unsafe extern "C" fn fsmc_enable_hwecc(chip: *mut nand_chip, mode: c_int) {
    static void fsmc_enable_hwecc(struct nand_chip *chip, int mode)
    {
    struct fsmc_nand_data *host = nand_to_fsmc(chip);
    writel_relaxed(readl(host.regs_va + FSMC_PC) & ~FSMC_ECCPLEN_256,
    host.regs_va + FSMC_PC);
    writel_relaxed(readl(host.regs_va + FSMC_PC) & ~FSMC_ECCEN,
    host.regs_va + FSMC_PC);
    writel_relaxed(readl(host.regs_va + FSMC_PC) | FSMC_ECCEN,
    host.regs_va + FSMC_PC);
    }
//
// fsmc_read_hwecc_ecc4 - Hardware ECC calculator for ecc4 option supported by
// FSMC. ECC is 13 bytes for 512 bytes of data (supports error correction up to
// max of 8-bits)
//
    static int fsmc_read_hwecc_ecc4(struct nand_chip *chip, const u8 *data,
    u8 *ecc)
    {
    struct fsmc_nand_data *host = nand_to_fsmc(chip);
    u32 ecc_tmp;
    let mut deadline: c_ulong = jiffies + FSMC_BUSY_WAIT_TIMEOUT;
    do {
    if (readl_relaxed(host.regs_va + STS) & FSMC_CODE_RDY)
    break;
    cond_resched();
    } while (!time_after_eq(jiffies, deadline));
    if (time_after_eq(jiffies, deadline)) {
    dev_err(host.dev, "calculate ecc timed out\n");
    return -ETIMEDOUT;
    }
    ecc_tmp = readl_relaxed(host.regs_va + ECC1);
    ecc[0] = ecc_tmp;
    ecc[1] = ecc_tmp >> 8;
    ecc[2] = ecc_tmp >> 16;
    ecc[3] = ecc_tmp >> 24;
    ecc_tmp = readl_relaxed(host.regs_va + ECC2);
    ecc[4] = ecc_tmp;
    ecc[5] = ecc_tmp >> 8;
    ecc[6] = ecc_tmp >> 16;
    ecc[7] = ecc_tmp >> 24;
    ecc_tmp = readl_relaxed(host.regs_va + ECC3);
    ecc[8] = ecc_tmp;
    ecc[9] = ecc_tmp >> 8;
    ecc[10] = ecc_tmp >> 16;
    ecc[11] = ecc_tmp >> 24;
    ecc_tmp = readl_relaxed(host.regs_va + STS);
    ecc[12] = ecc_tmp >> 16;
    return 0;
    }
//
// fsmc_read_hwecc_ecc1 - Hardware ECC calculator for ecc1 option supported by
// FSMC. ECC is 3 bytes for 512 bytes of data (supports error correction up to
// max of 1-bit)
//
    static int fsmc_read_hwecc_ecc1(struct nand_chip *chip, const u8 *data,
    u8 *ecc)
    {
    struct fsmc_nand_data *host = nand_to_fsmc(chip);
    u32 ecc_tmp;
    ecc_tmp = readl_relaxed(host.regs_va + ECC1);
    ecc[0] = ecc_tmp;
    ecc[1] = ecc_tmp >> 8;
    ecc[2] = ecc_tmp >> 16;
    return 0;
    }
    static int fsmc_correct_ecc1(struct nand_chip *chip,
    unsigned char *buf,
    unsigned char *read_ecc,
    unsigned char *calc_ecc)
    {
    let mut sm_order: bool = chip.ecc.options & NAND_ECC_SOFT_HAMMING_SM_ORDER;
    return ecc_sw_hamming_correct(buf, read_ecc, calc_ecc,
    chip.ecc.size, sm_order);
    }
// Count the number of 0's in buff upto a max of max_bits
#[no_mangle]
unsafe extern "C" fn count_written_bits(buff: *mut u8, size: c_int, max_bits: c_int) -> c_int {
    static int count_written_bits(u8 *buff, int size, int max_bits)
    {
    int k, written_bits = 0;
    for (k = 0; k < size; k++) {
    written_bits += hweight8(~buff[k]);
    if (written_bits > max_bits)
    break;
    }
    return written_bits;
    }
#[no_mangle]
unsafe extern "C" fn dma_complete(param: *mut c_void) {
    static void dma_complete(void *param)
    {
    struct fsmc_nand_data *host = param;
    complete(&host.dma_access_complete);
    }
    static int dma_xfer(struct fsmc_nand_data *host, void *buffer, int len,
    enum dma_data_direction direction)
    {
    struct dma_chan *chan;
    struct dma_device *dma_dev;
    struct dma_async_tx_descriptor *tx;
    dma_addr_t dma_dst, dma_src, dma_addr;
    dma_cookie_t cookie;
    let mut flags: c_ulong = DMA_CTRL_ACK | DMA_PREP_INTERRUPT;
    int ret;
    unsigned long time_left;
    if (direction == DMA_TO_DEVICE)
    chan = host.write_dma_chan;
#[no_mangle]
pub unsafe extern "C" fn if(DMA_FROM_DEVICE: direction ==) -> else {
    else if (direction == DMA_FROM_DEVICE)
    chan = host.read_dma_chan;
    else
    return -EINVAL;
    dma_dev = chan.device;
    dma_addr = dma_map_single(dma_dev.dev, buffer, len, direction);
    if (dma_mapping_error(dma_dev.dev, dma_addr))
    return -EINVAL;
    if (direction == DMA_TO_DEVICE) {
    dma_src = dma_addr;
    dma_dst = host.data_pa;
    } else {
    dma_src = host.data_pa;
    dma_dst = dma_addr;
    }
    tx = dma_dev.device_prep_dma_memcpy(chan, dma_dst, dma_src,
    len, flags);
    if (!tx) {
    dev_err(host.dev, "device_prep_dma_memcpy error\n");
    ret = -EIO;
    goto unmap_dma;
    }
    tx.callback = dma_complete;
    tx.callback_param = host;
    cookie = tx.tx_submit(tx);
    ret = dma_submit_error(cookie);
    if (ret) {
    dev_err(host.dev, "dma_submit_error %d\n", cookie);
    goto unmap_dma;
    }
    dma_async_issue_pending(chan);
    time_left =
    wait_for_completion_timeout(&host.dma_access_complete,
    msecs_to_jiffies(3000));
    if (time_left == 0) {
    dmaengine_terminate_all(chan);
    dev_err(host.dev, "wait_for_completion_timeout\n");
    ret = -ETIMEDOUT;
    goto unmap_dma;
    }
    ret = 0;
    unmap_dma:
    dma_unmap_single(dma_dev.dev, dma_addr, len, direction);
    return ret;
    }
//
// fsmc_write_buf - write buffer to chip
// @host:	FSMC NAND controller
// @buf:	data buffer
// @len:	number of bytes to write
//
    static void fsmc_write_buf(struct fsmc_nand_data *host, const u8 *buf,
    int len)
    {
    int i;
    if (IS_ALIGNED((uintptr_t)buf, sizeof(u32)) &&
    IS_ALIGNED(len, sizeof(u32))) {
    u32 *p = (u32 *)buf;
    len = len >> 2;
    for (i = 0; i < len; i++)
    writel_relaxed(p[i], host.data_va);
    } else {
    for (i = 0; i < len; i++)
    writeb_relaxed(buf[i], host.data_va);
    }
    }
//
// fsmc_read_buf - read chip data into buffer
// @host:	FSMC NAND controller
// @buf:	buffer to store date
// @len:	number of bytes to read
//
#[no_mangle]
unsafe extern "C" fn fsmc_read_buf(host: *mut fsmc_nand_data, buf: *mut u8, len: c_int) {
    static void fsmc_read_buf(struct fsmc_nand_data *host, u8 *buf, int len)
    {
    int i;
    if (IS_ALIGNED((uintptr_t)buf, sizeof(u32)) &&
    IS_ALIGNED(len, sizeof(u32))) {
    u32 *p = (u32 *)buf;
    len = len >> 2;
    for (i = 0; i < len; i++)
    p[i] = readl_relaxed(host.data_va);
    } else {
    for (i = 0; i < len; i++)
    buf[i] = readb_relaxed(host.data_va);
    }
    }
//
// fsmc_read_buf_dma - read chip data into buffer
// @host:	FSMC NAND controller
// @buf:	buffer to store date
// @len:	number of bytes to read
//
    static void fsmc_read_buf_dma(struct fsmc_nand_data *host, u8 *buf,
    int len)
    {
    dma_xfer(host, buf, len, DMA_FROM_DEVICE);
    }
//
// fsmc_write_buf_dma - write buffer to chip
// @host:	FSMC NAND controller
// @buf:	data buffer
// @len:	number of bytes to write
//
    static void fsmc_write_buf_dma(struct fsmc_nand_data *host, const u8 *buf,
    int len)
    {
    dma_xfer(host, (void *)buf, len, DMA_TO_DEVICE);
    }
//
// fsmc_exec_op - hook called by the core to execute NAND operations
//
// This controller is simple enough and thus does not need to use the parser
// provided by the core, instead, handle every situation here.
//
    static int fsmc_exec_op(struct nand_chip *chip, const struct nand_operation *op,
    bool check_only)
    {
    struct fsmc_nand_data *host = nand_to_fsmc(chip);
    const struct nand_op_instr *instr = core::ptr::null_mut();
    let mut ret: c_int = 0;
    unsigned int op_id;
    int i;
    if (check_only)
    return 0;
    pr_debug("Executing operation [%d instructions]:\n", op.ninstrs);
    for (op_id = 0; op_id < op.ninstrs; op_id++) {
    instr = &op.instrs[op_id];
    nand_op_trace("  ", instr);
    switch (instr.type) {
    case NAND_OP_CMD_INSTR:
    writeb_relaxed(instr.ctx.cmd.opcode, host.cmd_va);
    break;
    case NAND_OP_ADDR_INSTR:
    for (i = 0; i < instr.ctx.addr.naddrs; i++)
    writeb_relaxed(instr.ctx.addr.addrs[i],
    host.addr_va);
    break;
    case NAND_OP_DATA_IN_INSTR:
    if (host.mode == USE_DMA_ACCESS)
    fsmc_read_buf_dma(host, instr.ctx.data.buf.in,
    instr.ctx.data.len);
    else
    fsmc_read_buf(host, instr.ctx.data.buf.in,
    instr.ctx.data.len);
    break;
    case NAND_OP_DATA_OUT_INSTR:
    if (host.mode == USE_DMA_ACCESS)
    fsmc_write_buf_dma(host,
    instr.ctx.data.buf.out,
    instr.ctx.data.len);
    else
    fsmc_write_buf(host, instr.ctx.data.buf.out,
    instr.ctx.data.len);
    break;
    case NAND_OP_WAITRDY_INSTR:
    ret = nand_soft_waitrdy(chip,
    instr.ctx.waitrdy.timeout_ms);
    break;
    }
    if (instr.delay_ns)
    ndelay(instr.delay_ns);
    }
    return ret;
    }
//
// fsmc_read_page_hwecc
// @chip:	nand chip info structure
// @buf:	buffer to store read data
// @oob_required:	caller expects OOB data read to chip->oob_poi
// @page:	page number to read
//
// This routine is needed for fsmc version 8 as reading from NAND chip has to be
// performed in a strict sequence as follows:
// data(512 byte) -> ecc(13 byte)
// After this read, fsmc hardware generates and reports error data bits(up to a
// max of 8 bits)
//
    static int fsmc_read_page_hwecc(struct nand_chip *chip, u8 *buf,
    int oob_required, int page)
    {
    struct mtd_info *mtd = nand_to_mtd(chip);
    int i, j, s, stat, eccsize = chip.ecc.size;
    let mut eccbytes: c_int = chip.ecc.bytes;
    let mut eccsteps: c_int = chip.ecc.steps;
    u8 *p = buf;
    u8 *ecc_calc = chip.ecc.calc_buf;
    u8 *ecc_code = chip.ecc.code_buf;
    int off, len, ret, group = 0;
//
// ecc_oob is intentionally taken as u16. In 16bit devices, we
// end up reading 14 bytes (7 words) from oob. The local array is
// to maintain word alignment
//
    u16 ecc_oob[7];
    u8 *oob = (u8 *)&ecc_oob[0];
    let mut max_bitflips: c_uint = 0;
    for (i = 0, s = 0; s < eccsteps; s++, i += eccbytes, p += eccsize) {
    nand_read_page_op(chip, page, s * eccsize, core::ptr::null_mut(), 0);
    chip.ecc.hwctl(chip, NAND_ECC_READ);
    ret = nand_read_data_op(chip, p, eccsize, false, false);
    if (ret)
    return ret;
    for (j = 0; j < eccbytes;) {
    struct mtd_oob_region oobregion;
    ret = mtd_ooblayout_ecc(mtd, group++, &oobregion);
    if (ret)
    return ret;
    off = oobregion.offset;
    len = oobregion.length;
//
// length is intentionally kept a higher multiple of 2
// to read at least 13 bytes even in case of 16 bit NAND
// devices
//
    if (chip.options & NAND_BUSWIDTH_16)
    len = roundup(len, 2);
    nand_read_oob_op(chip, page, off, oob + j, len);
    j += len;
    }
    memcpy(&ecc_code[i], oob, chip.ecc.bytes);
    chip.ecc.calculate(chip, p, &ecc_calc[i]);
    stat = chip.ecc.correct(chip, p, &ecc_code[i], &ecc_calc[i]);
    if (stat < 0) {
    mtd.ecc_stats.failed++;
    } else {
    mtd.ecc_stats.corrected += stat;
    max_bitflips = max_t(unsigned int, max_bitflips, stat);
    }
    }
    return max_bitflips;
    }
//
// fsmc_bch8_correct_data
// @mtd:	mtd info structure
// @dat:	buffer of read data
// @read_ecc:	ecc read from device spare area
// @calc_ecc:	ecc calculated from read data
//
// calc_ecc is a 104 bit information containing maximum of 8 error
// offset information of 13 bits each in 512 bytes of read data.
//
    static int fsmc_bch8_correct_data(struct nand_chip *chip, u8 *dat,
    u8 *read_ecc, u8 *calc_ecc)
    {
    struct fsmc_nand_data *host = nand_to_fsmc(chip);
    u32 err_idx[8];
    u32 num_err, i;
    u32 ecc1, ecc2, ecc3, ecc4;
    num_err = (readl_relaxed(host.regs_va + STS) >> 10) & 0xF;
// no bit flipping
    if (likely(num_err == 0))
    return 0;
// too many errors
    if (unlikely(num_err > 8)) {
//
// This is a temporary erase check. A newly erased page read
// would result in an ecc error because the oob data is also
// erased to FF and the calculated ecc for an FF data is not
// FF..FF.
// This is a workaround to skip performing correction in case
// data is FF..FF
//
// Logic:
// For every page, each bit written as 0 is counted until these
// number of bits are greater than 8 (the maximum correction
// capability of FSMC for each 512 + 13 bytes)
//
    let mut bits_ecc: c_int = count_written_bits(read_ecc, chip.ecc.bytes, 8);
    let mut bits_data: c_int = count_written_bits(dat, chip.ecc.size, 8);
    if ((bits_ecc + bits_data) <= 8) {
    if (bits_data)
    memset(dat, 0xff, chip.ecc.size);
    return bits_data;
    }
    return -EBADMSG;
    }
//
// ------------------- calc_ecc[] bit wise -----------|--13 bits--|
// |---idx[7]--|--.....-----|---idx[2]--||---idx[1]--||---idx[0]--|
//
// calc_ecc is a 104 bit information containing maximum of 8 error
// offset information of 13 bits each. calc_ecc is copied into a
// u64 array and error offset indexes are populated in err_idx
// array
//
    ecc1 = readl_relaxed(host.regs_va + ECC1);
    ecc2 = readl_relaxed(host.regs_va + ECC2);
    ecc3 = readl_relaxed(host.regs_va + ECC3);
    ecc4 = readl_relaxed(host.regs_va + STS);
    err_idx[0] = (ecc1 >> 0) & 0x1FFF;
    err_idx[1] = (ecc1 >> 13) & 0x1FFF;
    err_idx[2] = (((ecc2 >> 0) & 0x7F) << 6) | ((ecc1 >> 26) & 0x3F);
    err_idx[3] = (ecc2 >> 7) & 0x1FFF;
    err_idx[4] = (((ecc3 >> 0) & 0x1) << 12) | ((ecc2 >> 20) & 0xFFF);
    err_idx[5] = (ecc3 >> 1) & 0x1FFF;
    err_idx[6] = (ecc3 >> 14) & 0x1FFF;
    err_idx[7] = (((ecc4 >> 16) & 0xFF) << 5) | ((ecc3 >> 27) & 0x1F);
    i = 0;
    while (num_err--) {
    err_idx[i] ^= 3;
    if (err_idx[i] < chip.ecc.size * 8) {
    let mut err: c_int = err_idx[i];
    dat[err >> 3] ^= BIT(err & 7);
    i++;
    }
    }
    return i;
    }
#[no_mangle]
unsafe extern "C" fn filter(chan: *mut dma_chan, slave: *mut c_void) -> bool {
    static bool filter(struct dma_chan *chan, void *slave)
    {
    chan.private = slave;
    return true;
    }
    static int fsmc_nand_probe_config_dt(struct platform_device *pdev,
    struct fsmc_nand_data *host,
    struct nand_chip *nand)
    {
    struct device_node *np = pdev.dev.of_node;
    u32 val;
    int ret;
    nand.options = 0;
    if (!of_property_read_u32(np, "bank-width", &val)) {
    if (val == 2) {
    nand.options |= NAND_BUSWIDTH_16;
    } else if (val == 1) {
    nand.options |= NAND_BUSWIDTH_AUTO;
    } else {
    dev_err(&pdev.dev, "invalid bank-width %u\n", val);
    return -EINVAL;
    }
    } else {
    nand.options |= NAND_BUSWIDTH_AUTO;
    }
    if (of_property_read_bool(np, "nand-skip-bbtscan"))
    nand.options |= NAND_SKIP_BBTSCAN;
    host.dev_timings = devm_kzalloc(&pdev.dev,
    sizeof(*host.dev_timings),
    GFP_KERNEL);
    if (!host.dev_timings)
    return -ENOMEM;
    ret = of_property_read_u8_array(np, "timings", (u8 *)host.dev_timings,
    sizeof(*host.dev_timings));
    if (ret)
    host.dev_timings = core::ptr::null_mut();
// Set default NAND bank to 0
    host.bank = 0;
    if (!of_property_read_u32(np, "bank", &val)) {
    if (val > 3) {
    dev_err(&pdev.dev, "invalid bank %u\n", val);
    return -EINVAL;
    }
    host.bank = val;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn fsmc_nand_attach_chip(nand: *mut nand_chip) -> c_int {
    static int fsmc_nand_attach_chip(struct nand_chip *nand)
    {
    struct mtd_info *mtd = nand_to_mtd(nand);
    struct fsmc_nand_data *host = nand_to_fsmc(nand);
    if (nand.ecc.engine_type == NAND_ECC_ENGINE_TYPE_INVALID)
    nand.ecc.engine_type = NAND_ECC_ENGINE_TYPE_ON_HOST;
    if (!nand.ecc.size)
    nand.ecc.size = 512;
    if (AMBA_REV_BITS(host.pid) >= 8) {
    nand.ecc.read_page = fsmc_read_page_hwecc;
    nand.ecc.calculate = fsmc_read_hwecc_ecc4;
    nand.ecc.correct = fsmc_bch8_correct_data;
    nand.ecc.bytes = 13;
    nand.ecc.strength = 8;
    }
    if (AMBA_REV_BITS(host.pid) >= 8) {
    switch (mtd.oobsize) {
    case 16:
    case 64:
    case 128:
    case 224:
    case 256:
    break;
    default:
    dev_warn(host.dev,
    "No oob scheme defined for oobsize %d\n",
    mtd.oobsize);
    return -EINVAL;
    }
    mtd_set_ooblayout(mtd, &fsmc_ecc4_ooblayout_ops);
    return 0;
    }
    switch (nand.ecc.engine_type) {
    case NAND_ECC_ENGINE_TYPE_ON_HOST:
    dev_info(host.dev, "Using 1-bit HW ECC scheme\n");
    nand.ecc.calculate = fsmc_read_hwecc_ecc1;
    nand.ecc.correct = fsmc_correct_ecc1;
    nand.ecc.hwctl = fsmc_enable_hwecc;
    nand.ecc.bytes = 3;
    nand.ecc.strength = 1;
    nand.ecc.options |= NAND_ECC_SOFT_HAMMING_SM_ORDER;
    break;
    case NAND_ECC_ENGINE_TYPE_SOFT:
    if (nand.ecc.algo == NAND_ECC_ALGO_BCH) {
    dev_info(host.dev,
    "Using 4-bit SW BCH ECC scheme\n");
    break;
    }
    break;
    case NAND_ECC_ENGINE_TYPE_ON_DIE:
    break;
    default:
    dev_err(host.dev, "Unsupported ECC mode!\n");
    return -ENOTSUPP;
    }
//
// Don't set layout for BCH4 SW ECC. This will be
// generated later during BCH initialization.
//
    if (nand.ecc.engine_type == NAND_ECC_ENGINE_TYPE_ON_HOST) {
    switch (mtd.oobsize) {
    case 16:
    case 64:
    case 128:
    mtd_set_ooblayout(mtd,
    &fsmc_ecc1_ooblayout_ops);
    break;
    default:
    dev_warn(host.dev,
    "No oob scheme defined for oobsize %d\n",
    mtd.oobsize);
    return -EINVAL;
    }
    }
    return 0;
    }
    static const struct nand_controller_ops fsmc_nand_controller_ops = {
    .attach_chip = fsmc_nand_attach_chip,
    .exec_op = fsmc_exec_op,
    .setup_interface = fsmc_setup_interface,
    };
//
// fsmc_nand_disable() - Disables the NAND bank
// @host: The instance to disable
//
#[no_mangle]
unsafe extern "C" fn fsmc_nand_disable(host: *mut fsmc_nand_data) {
    static void fsmc_nand_disable(struct fsmc_nand_data *host)
    {
    u32 val;
    val = readl(host.regs_va + FSMC_PC);
    val &= ~FSMC_ENABLE;
    writel(val, host.regs_va + FSMC_PC);
    }
//
// fsmc_nand_probe - Probe function
// @pdev:       platform device structure
//
#[no_mangle]
unsafe extern "C" fn fsmc_nand_probe(pdev: *mut platform_device) -> int __init {
    static int __init fsmc_nand_probe(struct platform_device *pdev)
    {
    struct fsmc_nand_data *host;
    struct mtd_info *mtd;
    struct nand_chip *nand;
    struct resource *res;
    void __iomem *base;
    dma_cap_mask_t mask;
    let mut ret: c_int = 0;
    u32 pid;
    int i;
// Allocate memory for the device structure (and zero it)
    host = devm_kzalloc(&pdev.dev, sizeof(*host), GFP_KERNEL);
    if (!host)
    return -ENOMEM;
    nand = &host.nand;
    ret = fsmc_nand_probe_config_dt(pdev, host, nand);
    if (ret)
    return ret;
    res = platform_get_resource_byname(pdev, IORESOURCE_MEM, "nand_data");
    host.data_va = devm_ioremap_resource(&pdev.dev, res);
    if (IS_ERR(host.data_va))
    return PTR_ERR(host.data_va);
    host.data_pa = (dma_addr_t)res.start;
    res = platform_get_resource_byname(pdev, IORESOURCE_MEM, "nand_addr");
    host.addr_va = devm_ioremap_resource(&pdev.dev, res);
    if (IS_ERR(host.addr_va))
    return PTR_ERR(host.addr_va);
    res = platform_get_resource_byname(pdev, IORESOURCE_MEM, "nand_cmd");
    host.cmd_va = devm_ioremap_resource(&pdev.dev, res);
    if (IS_ERR(host.cmd_va))
    return PTR_ERR(host.cmd_va);
    res = platform_get_resource_byname(pdev, IORESOURCE_MEM, "fsmc_regs");
    base = devm_ioremap_resource(&pdev.dev, res);
    if (IS_ERR(base))
    return PTR_ERR(base);
    host.regs_va = base + FSMC_NOR_REG_SIZE +
    (host.bank * FSMC_NAND_BANK_SZ);
    host.clk = devm_clk_get_enabled(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(host.clk)) {
    dev_err(&pdev.dev, "failed to fetch block clock\n");
    return PTR_ERR(host.clk);
    }
//
// This device ID is actually a common AMBA ID as used on the
// AMBA PrimeCell bus. However it is not a PrimeCell.
//
    for (pid = 0, i = 0; i < 4; i++)
    pid |= (readl(base + resource_size(res) - 0x20 + 4 * i) &
    255) << (i * 8);
    host.pid = pid;
    dev_info(&pdev.dev,
    "FSMC device partno %03x, manufacturer %02x, revision %02x, config %02x\n",
    AMBA_PART_BITS(pid), AMBA_MANF_BITS(pid),
    AMBA_REV_BITS(pid), AMBA_CONFIG_BITS(pid));
    host.dev = &pdev.dev;
    if (host.mode == USE_DMA_ACCESS)
    init_completion(&host.dma_access_complete);
// Link all private pointers
    mtd = nand_to_mtd(&host.nand);
    nand_set_flash_node(nand, pdev.dev.of_node);
    mtd.dev.parent = &pdev.dev;
    nand.badblockbits = 7;
    if (host.mode == USE_DMA_ACCESS) {
    dma_cap_zero(mask);
    dma_cap_set(DMA_MEMCPY, mask);
    host.read_dma_chan = dma_request_channel(mask, filter, core::ptr::null_mut());
    if (!host.read_dma_chan) {
    dev_err(&pdev.dev, "Unable to get read dma channel\n");
    ret = -ENODEV;
    goto disable_fsmc;
    }
    host.write_dma_chan = dma_request_channel(mask, filter, core::ptr::null_mut());
    if (!host.write_dma_chan) {
    dev_err(&pdev.dev, "Unable to get write dma channel\n");
    ret = -ENODEV;
    goto release_dma_read_chan;
    }
    }
    if (host.dev_timings) {
    fsmc_nand_setup(host, host.dev_timings);
    nand.options |= NAND_KEEP_TIMINGS;
    }
    nand_controller_init(&host.base);
    host.base.ops = &fsmc_nand_controller_ops;
    nand.controller = &host.base;
//
// Scan to find existence of the device
//
    ret = nand_scan(nand, 1);
    if (ret)
    goto release_dma_write_chan;
    mtd.name = "nand";
    ret = mtd_device_register(mtd, core::ptr::null_mut(), 0);
    if (ret)
    goto cleanup_nand;
    platform_set_drvdata(pdev, host);
    dev_info(&pdev.dev, "FSMC NAND driver registration successful\n");
    return 0;
    cleanup_nand:
    nand_cleanup(nand);
    release_dma_write_chan:
    if (host.mode == USE_DMA_ACCESS)
    dma_release_channel(host.write_dma_chan);
    release_dma_read_chan:
    if (host.mode == USE_DMA_ACCESS)
    dma_release_channel(host.read_dma_chan);
    disable_fsmc:
    fsmc_nand_disable(host);
    return ret;
    }
//
// Clean up routine
//
#[no_mangle]
unsafe extern "C" fn fsmc_nand_remove(pdev: *mut platform_device) {
    static void fsmc_nand_remove(struct platform_device *pdev)
    {
    struct fsmc_nand_data *host = platform_get_drvdata(pdev);
    if (host) {
    struct nand_chip *chip = &host.nand;
    int ret;
    ret = mtd_device_unregister(nand_to_mtd(chip));
    WARN_ON(ret);
    nand_cleanup(chip);
    fsmc_nand_disable(host);
    if (host.mode == USE_DMA_ACCESS) {
    dma_release_channel(host.write_dma_chan);
    dma_release_channel(host.read_dma_chan);
    }
    }
    }

#[no_mangle]
unsafe extern "C" fn fsmc_nand_suspend(dev: *mut device) -> c_int {
    static int fsmc_nand_suspend(struct device *dev)
    {
    struct fsmc_nand_data *host = dev_get_drvdata(dev);
    if (host)
    clk_disable_unprepare(host.clk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn fsmc_nand_resume(dev: *mut device) -> c_int {
    static int fsmc_nand_resume(struct device *dev)
    {
    struct fsmc_nand_data *host = dev_get_drvdata(dev);
    int ret;
    if (host) {
    ret = clk_prepare_enable(host.clk);
    if (ret) {
    dev_err(dev, "failed to enable clk\n");
    return ret;
    }
    if (host.dev_timings)
    fsmc_nand_setup(host, host.dev_timings);
    nand_reset(&host.nand, 0);
    }
    return 0;
    }

    static SIMPLE_DEV_PM_OPS(fsmc_nand_pm_ops, fsmc_nand_suspend, fsmc_nand_resume);
    static const struct of_device_id fsmc_nand_id_table[] = {
    { .compatible = "st,spear600-fsmc-nand" },
    { .compatible = "stericsson,fsmc-nand" },
    {}
    };
    MODULE_DEVICE_TABLE(of, fsmc_nand_id_table);
    static struct platform_driver fsmc_nand_driver = {
    .remove = fsmc_nand_remove,
    .driver = {
    .name = "fsmc-nand",
    .of_match_table = fsmc_nand_id_table,
    .pm = &fsmc_nand_pm_ops,
    },
    };
    module_platform_driver_probe(fsmc_nand_driver, fsmc_nand_probe);
    MODULE_LICENSE("GPL v2");
    MODULE_AUTHOR("Vipin Kumar <vipin.kumar@st.com>, Ashish Priyadarshi");
    MODULE_DESCRIPTION("NAND driver for SPEAr Platforms");
