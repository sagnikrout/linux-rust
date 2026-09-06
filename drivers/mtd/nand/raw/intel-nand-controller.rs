//! Automatically rewritten from C to Rust
//! Source: drivers/mtd/nand/raw/intel-nand-controller.c
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


// SPDX-License-Identifier: GPL-2.0+
// Copyright (c) 2020 Intel Corporation.

pub const EBU_CLC: c_uint = 0x000;
pub const EBU_CLC_RST: c_uint = 0x00000000u;

// 5 bits 26:22 included for comparison in the ADDR_SELx

pub const EBU_ADDR_SEL_REGEN: c_uint = 0x1;

pub const EBU_BUSCON_CMULT_V4: c_uint = 0x1;

pub const EBU_BUSCON_BCGEN_CS: c_uint = 0x0;

pub const EBU_BUSCON_ALEC: c_uint = 0xC000;
pub const EBU_CON: c_uint = 0x0B0;

pub const EBU_CON_NANDM_DIS: c_uint = 0x0;

pub const EBU_WAIT: c_uint = 0x0B4;

pub const HSNAND_CTL1: c_uint = 0x110;
pub const HSNAND_CTL1_ADDR_SHIFT: c_int = 24;
pub const HSNAND_CTL2: c_uint = 0x114;
pub const HSNAND_CTL2_ADDR_SHIFT: c_int = 8;

pub const HSNAND_INT_MSK_CTL: c_uint = 0x124;

pub const HSNAND_INT_STA: c_uint = 0x128;

pub const HSNAND_CTL: c_uint = 0x130;

pub const HSNAND_CTL_RW_READ: c_uint = 0x0;

pub const HSNAND_CTL_CKFF_EN: c_uint = 0x0;

pub const HSNAND_PARA0: c_uint = 0x13c;
pub const HSNAND_PARA0_PAGE_V8192: c_uint = 0x3;

pub const HSNAND_PARA0_BYP_EN_NP: c_uint = 0x0;
pub const HSNAND_PARA0_BYP_DEC_NP: c_uint = 0x0;

pub const HSNAND_CMSG_0: c_uint = 0x150;
pub const HSNAND_CMSG_1: c_uint = 0x154;

pub const HSNAND_ECC_OFFSET: c_uint = 0x008;
pub const MAX_CS: c_int = 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ebu_nand_cs {
    pub chipaddr: *mut void __iomem,
    pub addr_sel: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ebu_nand_controller {
    pub controller: nand_controller,
    pub chip: nand_chip,
    pub dev: *mut device,
    pub ebu: *mut void __iomem,
    pub hsnand: *mut void __iomem,
    pub dma_tx: *mut dma_chan,
    pub dma_rx: *mut dma_chan,
    pub dma_access_complete: completion,
    pub clk: *mut clk,
    pub nd_para0: u32,
    pub cs_num: u8,
    pub cs: [ebu_nand_cs; MAX_CS],
}

    static inline struct ebu_nand_controller *nand_to_ebu(struct nand_chip *chip)
    {
    return container_of(chip, struct ebu_nand_controller, chip);
    }
#[no_mangle]
unsafe extern "C" fn ebu_nand_waitrdy(chip: *mut nand_chip, timeout_ms: c_int) -> c_int {
    static int ebu_nand_waitrdy(struct nand_chip *chip, int timeout_ms)
    {
    struct ebu_nand_controller *ctrl = nand_to_ebu(chip);
    u32 status;
    return readl_poll_timeout(ctrl.ebu + EBU_WAIT, status,
    (status & EBU_WAIT_RDBY) ||
    (status & EBU_WAIT_WR_C), 20, timeout_ms);
    }
#[no_mangle]
unsafe extern "C" fn ebu_nand_readb(chip: *mut nand_chip) -> u8 {
    static u8 ebu_nand_readb(struct nand_chip *chip)
    {
    struct ebu_nand_controller *ebu_host = nand_get_controller_data(chip);
    let mut cs_num: u8 = ebu_host.cs_num;
    u8 val;
    val = readb(ebu_host.cs[cs_num].chipaddr + HSNAND_CS_OFFS);
    ebu_nand_waitrdy(chip, 1000);
    return val;
    }
#[no_mangle]
unsafe extern "C" fn ebu_nand_writeb(chip: *mut nand_chip, offset: u32, value: u8) {
    static void ebu_nand_writeb(struct nand_chip *chip, u32 offset, u8 value)
    {
    struct ebu_nand_controller *ebu_host = nand_get_controller_data(chip);
    let mut cs_num: u8 = ebu_host.cs_num;
    writeb(value, ebu_host.cs[cs_num].chipaddr + offset);
    ebu_nand_waitrdy(chip, 1000);
    }
#[no_mangle]
unsafe extern "C" fn ebu_read_buf(chip: *mut nand_chip, buf: *mut u_char, len: c_uint) {
    static void ebu_read_buf(struct nand_chip *chip, u_char *buf, unsigned int len)
    {
    int i;
    for (i = 0; i < len; i++)
    buf[i] = ebu_nand_readb(chip);
    }
#[no_mangle]
unsafe extern "C" fn ebu_write_buf(chip: *mut nand_chip, buf: *const u_char, len: c_int) {
    static void ebu_write_buf(struct nand_chip *chip, const u_char *buf, int len)
    {
    int i;
    for (i = 0; i < len; i++)
    ebu_nand_writeb(chip, HSNAND_CS_OFFS, buf[i]);
    }
#[no_mangle]
unsafe extern "C" fn ebu_nand_disable(chip: *mut nand_chip) {
    static void ebu_nand_disable(struct nand_chip *chip)
    {
    struct ebu_nand_controller *ebu_host = nand_get_controller_data(chip);
    writel(0, ebu_host.ebu + EBU_CON);
    }
#[no_mangle]
unsafe extern "C" fn ebu_select_chip(chip: *mut nand_chip) {
    static void ebu_select_chip(struct nand_chip *chip)
    {
    struct ebu_nand_controller *ebu_host = nand_get_controller_data(chip);
    void __iomem *nand_con = ebu_host.ebu + EBU_CON;
    let mut cs: u32 = ebu_host.cs_num;
    writel(EBU_CON_NANDM_EN | EBU_CON_CSMUX_E_EN | EBU_CON_CS_P_LOW |
    EBU_CON_SE_P_LOW | EBU_CON_WP_P_LOW | EBU_CON_PRE_P_LOW |
    EBU_CON_IN_CS_S(cs) | EBU_CON_OUT_CS_S(cs) |
    EBU_CON_LAT_EN_CS_P, nand_con);
    }
    static int ebu_nand_set_timings(struct nand_chip *chip, int csline,
    const struct nand_interface_config *conf)
    {
    struct ebu_nand_controller *ctrl = nand_to_ebu(chip);
    let mut rate: c_uint = clk_get_rate(ctrl.clk) / HZ_PER_MHZ;
    let mut period: c_uint = DIV_ROUND_UP(USEC_PER_SEC, rate);
    const struct nand_sdr_timings *timings;
    u32 trecov, thold, twrwait, trdwait;
    let mut reg: u32 = 0;
    timings = nand_get_sdr_timings(conf);
    if (IS_ERR(timings))
    return PTR_ERR(timings);
    if (csline == NAND_DATA_IFACE_CHECK_ONLY)
    return 0;
    trecov = DIV_ROUND_UP(max(timings.tREA_max, timings.tREH_min),
    period);
    reg |= EBU_BUSCON_RECOVC(trecov);
    thold = DIV_ROUND_UP(max(timings.tDH_min, timings.tDS_min), period);
    reg |= EBU_BUSCON_HOLDC(thold);
    trdwait = DIV_ROUND_UP(max(timings.tRC_min, timings.tREH_min),
    period);
    reg |= EBU_BUSCON_WAITRDC(trdwait);
    twrwait = DIV_ROUND_UP(max(timings.tWC_min, timings.tWH_min), period);
    reg |= EBU_BUSCON_WAITWRC(twrwait);
    reg |= EBU_BUSCON_CMULT_V4 | EBU_BUSCON_BCGEN_CS | EBU_BUSCON_ALEC |
    EBU_BUSCON_SETUP_EN;
    writel(reg, ctrl.ebu + EBU_BUSCON(ctrl.cs_num));
    return 0;
    }
    static int ebu_nand_ooblayout_ecc(struct mtd_info *mtd, int section,
    struct mtd_oob_region *oobregion)
    {
    struct nand_chip *chip = mtd_to_nand(mtd);
    if (section)
    return -ERANGE;
    oobregion.offset = HSNAND_ECC_OFFSET;
    oobregion.length = chip.ecc.total;
    return 0;
    }
    static int ebu_nand_ooblayout_free(struct mtd_info *mtd, int section,
    struct mtd_oob_region *oobregion)
    {
    struct nand_chip *chip = mtd_to_nand(mtd);
    if (section)
    return -ERANGE;
    oobregion.offset = chip.ecc.total + HSNAND_ECC_OFFSET;
    oobregion.length = mtd.oobsize - oobregion.offset;
    return 0;
    }
    static const struct mtd_ooblayout_ops ebu_nand_ooblayout_ops = {
    .ecc = ebu_nand_ooblayout_ecc,
    .free = ebu_nand_ooblayout_free,
    };
#[no_mangle]
unsafe extern "C" fn ebu_dma_rx_callback(cookie: *mut c_void) {
    static void ebu_dma_rx_callback(void *cookie)
    {
    struct ebu_nand_controller *ebu_host = cookie;
    dmaengine_terminate_async(ebu_host.dma_rx);
    complete(&ebu_host.dma_access_complete);
    }
#[no_mangle]
unsafe extern "C" fn ebu_dma_tx_callback(cookie: *mut c_void) {
    static void ebu_dma_tx_callback(void *cookie)
    {
    struct ebu_nand_controller *ebu_host = cookie;
    dmaengine_terminate_async(ebu_host.dma_tx);
    complete(&ebu_host.dma_access_complete);
    }
    static int ebu_dma_start(struct ebu_nand_controller *ebu_host, u32 dir,
    const u8 *buf, u32 len)
    {
    struct dma_async_tx_descriptor *tx;
    struct completion *dma_completion;
    dma_async_tx_callback callback;
    struct dma_chan *chan;
    dma_cookie_t cookie;
    let mut flags: c_ulong = DMA_CTRL_ACK | DMA_PREP_INTERRUPT;
    dma_addr_t buf_dma;
    int ret;
    unsigned long time_left;
    if (dir == DMA_DEV_TO_MEM) {
    chan = ebu_host.dma_rx;
    dma_completion = &ebu_host.dma_access_complete;
    callback = ebu_dma_rx_callback;
    } else {
    chan = ebu_host.dma_tx;
    dma_completion = &ebu_host.dma_access_complete;
    callback = ebu_dma_tx_callback;
    }
    buf_dma = dma_map_single(chan.device.dev, (void *)buf, len, dir);
    if (dma_mapping_error(chan.device.dev, buf_dma)) {
    dev_err(ebu_host.dev, "Failed to map DMA buffer\n");
    ret = -EIO;
    goto err_unmap;
    }
    tx = dmaengine_prep_slave_single(chan, buf_dma, len, dir, flags);
    if (!tx) {
    ret = -ENXIO;
    goto err_unmap;
    }
    tx.callback = callback;
    tx.callback_param = ebu_host;
    cookie = tx.tx_submit(tx);
    ret = dma_submit_error(cookie);
    if (ret) {
    dev_err(ebu_host.dev, "dma_submit_error %d\n", cookie);
    ret = -EIO;
    goto err_unmap;
    }
    init_completion(dma_completion);
    dma_async_issue_pending(chan);
// Wait DMA to finish the data transfer.
    time_left = wait_for_completion_timeout(dma_completion, msecs_to_jiffies(1000));
    if (!time_left) {
    dev_err(ebu_host.dev, "I/O Error in DMA RX (status %d)\n",
    dmaengine_tx_status(chan, cookie, core::ptr::null_mut()));
    dmaengine_terminate_sync(chan);
    ret = -ETIMEDOUT;
    goto err_unmap;
    }
    return 0;
    err_unmap:
    dma_unmap_single(ebu_host.dev, buf_dma, len, dir);
    return ret;
    }
    static void ebu_nand_trigger(struct ebu_nand_controller *ebu_host,
    int page, u32 cmd)
    {
    unsigned int val;
    val = cmd | (page & 0xFF) << HSNAND_CTL1_ADDR_SHIFT;
    writel(val, ebu_host.hsnand + HSNAND_CTL1);
    val = (page & 0xFFFF00) >> 8 | HSNAND_CTL2_CYC_N_V5;
    writel(val, ebu_host.hsnand + HSNAND_CTL2);
    writel(ebu_host.nd_para0, ebu_host.hsnand + HSNAND_PARA0);
// clear first, will update later
    writel(0xFFFFFFFF, ebu_host.hsnand + HSNAND_CMSG_0);
    writel(0xFFFFFFFF, ebu_host.hsnand + HSNAND_CMSG_1);
    writel(HSNAND_INT_MSK_CTL_WR_C,
    ebu_host.hsnand + HSNAND_INT_MSK_CTL);
    if (!cmd)
    val = HSNAND_CTL_RW_READ;
    else
    val = HSNAND_CTL_RW_WRITE;
    writel(HSNAND_CTL_MSG_EN | HSNAND_CTL_CKFF_EN |
    HSNAND_CTL_ECC_OFF_V8TH | HSNAND_CTL_CE_SEL_CS(ebu_host.cs_num) |
    HSNAND_CTL_ENABLE_ECC | HSNAND_CTL_GO | val,
    ebu_host.hsnand + HSNAND_CTL);
    }
    static int ebu_nand_read_page_hwecc(struct nand_chip *chip, u8 *buf,
    int oob_required, int page)
    {
    struct mtd_info *mtd = nand_to_mtd(chip);
    struct ebu_nand_controller *ebu_host = nand_get_controller_data(chip);
    int ret, reg_data;
    ebu_nand_trigger(ebu_host, page, NAND_CMD_READ0);
    ret = ebu_dma_start(ebu_host, DMA_DEV_TO_MEM, buf, mtd.writesize);
    if (ret)
    return ret;
    if (oob_required)
    chip.ecc.read_oob(chip, page);
    reg_data = readl(ebu_host.hsnand + HSNAND_CTL);
    reg_data &= ~HSNAND_CTL_GO;
    writel(reg_data, ebu_host.hsnand + HSNAND_CTL);
    return 0;
    }
    static int ebu_nand_write_page_hwecc(struct nand_chip *chip, const u8 *buf,
    int oob_required, int page)
    {
    struct mtd_info *mtd = nand_to_mtd(chip);
    struct ebu_nand_controller *ebu_host = nand_get_controller_data(chip);
    void __iomem *int_sta = ebu_host.hsnand + HSNAND_INT_STA;
    int reg_data, ret, val;
    u32 reg;
    ebu_nand_trigger(ebu_host, page, NAND_CMD_SEQIN);
    ret = ebu_dma_start(ebu_host, DMA_MEM_TO_DEV, buf, mtd.writesize);
    if (ret)
    return ret;
    if (oob_required) {
    reg = get_unaligned_le32(chip.oob_poi);
    writel(reg, ebu_host.hsnand + HSNAND_CMSG_0);
    reg = get_unaligned_le32(chip.oob_poi + 4);
    writel(reg, ebu_host.hsnand + HSNAND_CMSG_1);
    }
    ret = readl_poll_timeout_atomic(int_sta, val, !(val & HSNAND_INT_STA_WR_C),
    10, 1000);
    if (ret)
    return ret;
    reg_data = readl(ebu_host.hsnand + HSNAND_CTL);
    reg_data &= ~HSNAND_CTL_GO;
    writel(reg_data, ebu_host.hsnand + HSNAND_CTL);
    return 0;
    }
    static const u8 ecc_strength[] = { 1, 1, 4, 8, 24, 32, 40, 60, };
#[no_mangle]
unsafe extern "C" fn ebu_nand_attach_chip(chip: *mut nand_chip) -> c_int {
    static int ebu_nand_attach_chip(struct nand_chip *chip)
    {
    struct mtd_info *mtd = nand_to_mtd(chip);
    struct ebu_nand_controller *ebu_host = nand_get_controller_data(chip);
    u32 ecc_steps, ecc_bytes, ecc_total, pagesize, pg_per_blk;
    let mut ecc_strength_ds: u32 = chip.ecc.strength;
    let mut ecc_size: u32 = chip.ecc.size;
    let mut writesize: u32 = mtd.writesize;
    let mut blocksize: u32 = mtd.erasesize;
    int bch_algo, start, val;
// Default to an ECC size of 512
    if (!chip.ecc.size)
    chip.ecc.size = 512;
    switch (ecc_size) {
    case 512:
    start = 1;
    if (!ecc_strength_ds)
    ecc_strength_ds = 4;
    break;
    case 1024:
    start = 4;
    if (!ecc_strength_ds)
    ecc_strength_ds = 32;
    break;
    default:
    return -EINVAL;
    }
// BCH ECC algorithm Settings for number of bits per 512B/1024B
    bch_algo = round_up(start + 1, 4);
    for (val = start; val < bch_algo; val++) {
    if (ecc_strength_ds == ecc_strength[val])
    break;
    }
    if (val == bch_algo)
    return -EINVAL;
    if (ecc_strength_ds == 8)
    ecc_bytes = 14;
    else
    ecc_bytes = DIV_ROUND_UP(ecc_strength_ds * fls(8 * ecc_size), 8);
    ecc_steps = writesize / ecc_size;
    ecc_total = ecc_steps * ecc_bytes;
    if ((ecc_total + 8) > mtd.oobsize)
    return -ERANGE;
    chip.ecc.total = ecc_total;
    pagesize = fls(writesize >> 11);
    if (pagesize > HSNAND_PARA0_PAGE_V8192)
    return -ERANGE;
    pg_per_blk = fls((blocksize / writesize) >> 6) / 8;
    if (pg_per_blk > HSNAND_PARA0_PIB_V256)
    return -ERANGE;
    ebu_host.nd_para0 = pagesize | pg_per_blk | HSNAND_PARA0_BYP_EN_NP |
    HSNAND_PARA0_BYP_DEC_NP | HSNAND_PARA0_ADEP_EN |
    HSNAND_PARA0_TYPE_ONFI | (val << 29);
    mtd_set_ooblayout(mtd, &ebu_nand_ooblayout_ops);
    chip.ecc.read_page = ebu_nand_read_page_hwecc;
    chip.ecc.write_page = ebu_nand_write_page_hwecc;
    return 0;
    }
    static int ebu_nand_exec_op(struct nand_chip *chip,
    const struct nand_operation *op, bool check_only)
    {
    const struct nand_op_instr *instr = core::ptr::null_mut();
    unsigned int op_id;
    int i, timeout_ms, ret = 0;
    if (check_only)
    return 0;
    ebu_select_chip(chip);
    for (op_id = 0; op_id < op.ninstrs; op_id++) {
    instr = &op.instrs[op_id];
    switch (instr.type) {
    case NAND_OP_CMD_INSTR:
    ebu_nand_writeb(chip, HSNAND_CLE_OFFS | HSNAND_CS_OFFS,
    instr.ctx.cmd.opcode);
    break;
    case NAND_OP_ADDR_INSTR:
    for (i = 0; i < instr.ctx.addr.naddrs; i++)
    ebu_nand_writeb(chip,
    HSNAND_ALE_OFFS | HSNAND_CS_OFFS,
    instr.ctx.addr.addrs[i]);
    break;
    case NAND_OP_DATA_IN_INSTR:
    ebu_read_buf(chip, instr.ctx.data.buf.in,
    instr.ctx.data.len);
    break;
    case NAND_OP_DATA_OUT_INSTR:
    ebu_write_buf(chip, instr.ctx.data.buf.out,
    instr.ctx.data.len);
    break;
    case NAND_OP_WAITRDY_INSTR:
    timeout_ms = instr.ctx.waitrdy.timeout_ms * 1000;
    ret = ebu_nand_waitrdy(chip, timeout_ms);
    break;
    }
    }
    return ret;
    }
    static const struct nand_controller_ops ebu_nand_controller_ops = {
    .attach_chip = ebu_nand_attach_chip,
    .setup_interface = ebu_nand_set_timings,
    .exec_op = ebu_nand_exec_op,
    };
#[no_mangle]
unsafe extern "C" fn ebu_dma_cleanup(ebu_host: *mut ebu_nand_controller) {
    static void ebu_dma_cleanup(struct ebu_nand_controller *ebu_host)
    {
    if (ebu_host.dma_rx)
    dma_release_channel(ebu_host.dma_rx);
    if (ebu_host.dma_tx)
    dma_release_channel(ebu_host.dma_tx);
    }
#[no_mangle]
unsafe extern "C" fn ebu_nand_probe(pdev: *mut platform_device) -> c_int {
    static int ebu_nand_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct ebu_nand_controller *ebu_host;
    struct device_node *chip_np;
    struct nand_chip *nand;
    struct mtd_info *mtd;
    struct resource *res;
    char *resname;
    int ret;
    u32 cs;
    ebu_host = devm_kzalloc(dev, sizeof(*ebu_host), GFP_KERNEL);
    if (!ebu_host)
    return -ENOMEM;
    ebu_host.dev = dev;
    nand_controller_init(&ebu_host.controller);
    ebu_host.ebu = devm_platform_ioremap_resource_byname(pdev, "ebunand");
    if (IS_ERR(ebu_host.ebu))
    return PTR_ERR(ebu_host.ebu);
    ebu_host.hsnand = devm_platform_ioremap_resource_byname(pdev, "hsnand");
    if (IS_ERR(ebu_host.hsnand))
    return PTR_ERR(ebu_host.hsnand);
    chip_np = of_get_next_child(dev.of_node, core::ptr::null_mut());
    if (!chip_np)
    return dev_err_probe(dev, -EINVAL,
    "Could not find child node for the NAND chip\n");
    ret = of_property_read_u32(chip_np, "reg", &cs);
    if (ret) {
    dev_err(dev, "failed to get chip select: %d\n", ret);
    goto err_of_node_put;
    }
    if (cs >= MAX_CS) {
    dev_err(dev, "got invalid chip select: %d\n", cs);
    ret = -EINVAL;
    goto err_of_node_put;
    }
    ebu_host.cs_num = cs;
    resname = devm_kasprintf(dev, GFP_KERNEL, "nand_cs%d", cs);
    if (!resname) {
    ret = -ENOMEM;
    goto err_of_node_put;
    }
    ebu_host.cs[cs].chipaddr = devm_platform_ioremap_resource_byname(pdev,
    resname);
    if (IS_ERR(ebu_host.cs[cs].chipaddr)) {
    ret = PTR_ERR(ebu_host.cs[cs].chipaddr);
    goto err_of_node_put;
    }
    ebu_host.clk = devm_clk_get_enabled(dev, core::ptr::null_mut());
    if (IS_ERR(ebu_host.clk)) {
    ret = dev_err_probe(dev, PTR_ERR(ebu_host.clk),
    "failed to get and enable clock\n");
    goto err_of_node_put;
    }
    ebu_host.dma_tx = dma_request_chan(dev, "tx");
    if (IS_ERR(ebu_host.dma_tx)) {
    ret = dev_err_probe(dev, PTR_ERR(ebu_host.dma_tx),
    "failed to request DMA tx chan!.\n");
    goto err_of_node_put;
    }
    ebu_host.dma_rx = dma_request_chan(dev, "rx");
    if (IS_ERR(ebu_host.dma_rx)) {
    ret = dev_err_probe(dev, PTR_ERR(ebu_host.dma_rx),
    "failed to request DMA rx chan!.\n");
    ebu_host.dma_rx = core::ptr::null_mut();
    goto err_cleanup_dma;
    }
    resname = devm_kasprintf(dev, GFP_KERNEL, "addr_sel%d", cs);
    if (!resname) {
    ret = -ENOMEM;
    goto err_cleanup_dma;
    }
    res = platform_get_resource_byname(pdev, IORESOURCE_MEM, resname);
    if (!res) {
    ret = -EINVAL;
    goto err_cleanup_dma;
    }
    ebu_host.cs[cs].addr_sel = res.start;
    writel(ebu_host.cs[cs].addr_sel | EBU_ADDR_MASK(5) | EBU_ADDR_SEL_REGEN,
    ebu_host.ebu + EBU_ADDR_SEL(cs));
    nand_set_flash_node(&ebu_host.chip, chip_np);
    mtd = nand_to_mtd(&ebu_host.chip);
    if (!mtd.name) {
    dev_err(ebu_host.dev, "NAND label property is mandatory\n");
    ret = -EINVAL;
    goto err_cleanup_dma;
    }
    mtd.dev.parent = dev;
    ebu_host.dev = dev;
    platform_set_drvdata(pdev, ebu_host);
    nand_set_controller_data(&ebu_host.chip, ebu_host);
    nand = &ebu_host.chip;
    nand.controller = &ebu_host.controller;
    nand.controller.ops = &ebu_nand_controller_ops;
// Scan to find existence of the device
    ret = nand_scan(&ebu_host.chip, 1);
    if (ret)
    goto err_cleanup_dma;
    ret = mtd_device_register(mtd, core::ptr::null_mut(), 0);
    if (ret)
    goto err_clean_nand;
    return 0;
    err_clean_nand:
    nand_cleanup(&ebu_host.chip);
    err_cleanup_dma:
    ebu_dma_cleanup(ebu_host);
    err_of_node_put:
    of_node_put(chip_np);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ebu_nand_remove(pdev: *mut platform_device) {
    static void ebu_nand_remove(struct platform_device *pdev)
    {
    struct ebu_nand_controller *ebu_host = platform_get_drvdata(pdev);
    int ret;
    ret = mtd_device_unregister(nand_to_mtd(&ebu_host.chip));
    WARN_ON(ret);
    nand_cleanup(&ebu_host.chip);
    ebu_nand_disable(&ebu_host.chip);
    ebu_dma_cleanup(ebu_host);
    }
    static const struct of_device_id ebu_nand_match[] = {
    { .compatible = "intel,lgm-ebunand" },
    {}
    };
    MODULE_DEVICE_TABLE(of, ebu_nand_match);
    static struct platform_driver ebu_nand_driver = {
    .probe = ebu_nand_probe,
    .remove = ebu_nand_remove,
    .driver = {
    .name = "intel-nand-controller",
    .of_match_table = ebu_nand_match,
    },
    };
    module_platform_driver(ebu_nand_driver);
    MODULE_LICENSE("GPL v2");
    MODULE_AUTHOR("Vadivel Murugan R <vadivel.muruganx.ramuthevar@intel.com>");
    MODULE_DESCRIPTION("Intel's LGM External Bus NAND Controller driver");
