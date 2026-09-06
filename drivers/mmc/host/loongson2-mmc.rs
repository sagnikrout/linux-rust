//! Automatically rewritten from C to Rust
//! Source: drivers/mmc/host/loongson2-mmc.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Loongson-2K MMC/SDIO controller driver
//
// Copyright (C) 2018-2025 Loongson Technology Corporation Limited.
//

pub const LOONGSON2_MMC_REG_CTL: c_uint = 0x00 /* Control Register */;
pub const LOONGSON2_MMC_REG_PRE: c_uint = 0x04 /* Prescaler Register */;
pub const LOONGSON2_MMC_REG_CARG: c_uint = 0x08 /* Command Register */;
pub const LOONGSON2_MMC_REG_CCTL: c_uint = 0x0c /* Command Control Register */;
pub const LOONGSON2_MMC_REG_CSTS: c_uint = 0x10 /* Command Status Register */;
pub const LOONGSON2_MMC_REG_RSP0: c_uint = 0x14 /* Command Response Register 0 */;
pub const LOONGSON2_MMC_REG_RSP1: c_uint = 0x18 /* Command Response Register 1 */;
pub const LOONGSON2_MMC_REG_RSP2: c_uint = 0x1c /* Command Response Register 2 */;
pub const LOONGSON2_MMC_REG_RSP3: c_uint = 0x20 /* Command Response Register 3 */;
pub const LOONGSON2_MMC_REG_TIMER: c_uint = 0x24 /* Data Timeout Register */;
pub const LOONGSON2_MMC_REG_BSIZE: c_uint = 0x28 /* Block Size Register */;
pub const LOONGSON2_MMC_REG_DCTL: c_uint = 0x2c /* Data Control Register */;
pub const LOONGSON2_MMC_REG_DCNT: c_uint = 0x30 /* Data Counter Register */;
pub const LOONGSON2_MMC_REG_DSTS: c_uint = 0x34 /* Data Status Register */;
pub const LOONGSON2_MMC_REG_FSTS: c_uint = 0x38 /* FIFO Status Register */;
pub const LOONGSON2_MMC_REG_INT: c_uint = 0x3c /* Interrupt Register */;
pub const LOONGSON2_MMC_REG_DATA: c_uint = 0x40 /* Data Register */;
pub const LOONGSON2_MMC_REG_IEN: c_uint = 0x64 /* Interrupt Enable Register */;
// EMMC DLL Mode Registers
pub const LOONGSON2_MMC_REG_DLLVAL: c_uint = 0xf0 /* DLL Master Lock-value Register */;
pub const LOONGSON2_MMC_REG_DLLCTL: c_uint = 0xf4 /* DLL Control Register */;
pub const LOONGSON2_MMC_REG_DELAY: c_uint = 0xf8 /* DLL Delayed Parameter Register */;
pub const LOONGSON2_MMC_REG_SEL: c_uint = 0xfc /* Bus Mode Selection Register */;
// Exclusive DMA R/W Registers
pub const LOONGSON2_MMC_REG_WDMA_LO: c_uint = 0x400;
pub const LOONGSON2_MMC_REG_WDMA_HI: c_uint = 0x404;
pub const LOONGSON2_MMC_REG_RDMA_LO: c_uint = 0x800;
pub const LOONGSON2_MMC_REG_RDMA_HI: c_uint = 0x804;
// Bitfields of control register

// Bitfields of prescaler register

// Bitfields of command control register

// Bitfields of command status register

// Bitfields of data timeout register

// Bitfields of block size register

// Bitfields of data control register

// Bitfields of sata counter register

// Bitfields of command status register

// Bitfields of FIFO Status Register

// Bitfields of interrupt register

// Bitfields of interrupt enable register

// Bitfields of DLL master lock-value register

// Bitfields of DLL control register

// Internal dma controller registers
// Bitfields of Global Configuration Register

// Bitfields of ndesc_addr field of HW descriptor

// Bitfields of cmd field of HW descriptor

pub const LOONGSON2_MMC_DLLVAL_TIMEOUT_US: c_int = 4000;
pub const LOONGSON2_MMC_TXFULL_TIMEOUT_US: c_int = 500;
//
// Due to a hardware design flaw, the Loongson-2K0300 may fail to recognize the
// CMD48 (SD_READ_EXTR_SINGLE) interrupt.
//

// Loongson-2K1000 SDIO2 DMA routing register

pub const LS2K1000_DMA0_CONF: c_uint = 0x0;
pub const LS2K1000_DMA1_CONF: c_uint = 0x1;
pub const LS2K1000_DMA2_CONF: c_uint = 0x2;
pub const LS2K1000_DMA3_CONF: c_uint = 0x3;
pub const LS2K1000_DMA4_CONF: c_uint = 0x4;
// Loongson-2K0500 SDIO2 DMA routing register

pub const LS2K0500_DMA0_CONF: c_uint = 0x1;
pub const LS2K0500_DMA1_CONF: c_uint = 0x2;
pub const LS2K0500_DMA2_CONF: c_uint = 0x3;
    enum loongson2_mmc_state {
    STATE_NONE,
    STATE_FINALIZE,
    STATE_CMDSENT,
    STATE_RSPFIN,
    STATE_XFERFINISH,
    STATE_XFERFINISH_RSPFIN,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct loongson2_dma_desc {
    pub ndesc_addr: u32,
    pub mem_addr: u32,
    pub apb_addr: u32,
    pub len: u32,
    pub step_len: u32,
    pub step_times: u32,
    pub cmd: u32,
    pub stats: u32,
    pub high_ndesc_addr: u32,
    pub high_mem_addr: u32,
    pub reserved: [u32; 2],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct loongson2_mmc_host {
    pub dev: *mut device,
    pub mrq: *mut mmc_request,
    pub regmap: *mut regmap,
    pub res: *mut resource,
    pub clk: *mut clk,
    pub current_clk: u32,
    pub sg_cpu: *mut c_void,
    pub sg_dma: dma_addr_t,
    pub dma_complete: c_int,
    pub chan: *mut dma_chan,
    pub cmd_is_stop: c_int,
    pub bus_width: c_int,
    pub /: *mut *mut spinlock_t lock; / Prevent races with irq handler,
    pub state: enum loongson2_mmc_state,
    pub pdata: *const loongson2_mmc_pdata,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct loongson2_mmc_pdata {
    pub flags: u32,
    pub regmap_config: *const regmap_config,
    pub cmd): *mut *mut *mut void (reorder_cmd_data)(struct loongson2_mmc_host host, struct mmc_command,
    pub cmd): *mut *mut *mut void (fix_data_timeout)(struct loongson2_mmc_host host, struct mmc_command,
    pub pdev): *mut *mut *mut int (setting_dma)(struct loongson2_mmc_host host, struct platform_device,
    pub data): *mut *mut *mut int (prepare_dma)(struct loongson2_mmc_host host, struct mmc_data,
    pub dev): *mut *mut *mut void (release_dma)(struct loongson2_mmc_host host, struct device,
}

    static void loongson2_mmc_send_command(struct loongson2_mmc_host *host,
    struct mmc_command *cmd)
    {
    u32 cctrl;
    if (cmd.data)
    host.state = STATE_XFERFINISH_RSPFIN;
#[no_mangle]
pub unsafe extern "C" fn if(MMC_RSP_PRESENT: cmd->flags &) -> else {
    else if (cmd.flags & MMC_RSP_PRESENT)
    host.state = STATE_RSPFIN;
    else
    host.state = STATE_CMDSENT;
    regmap_write(host.regmap, LOONGSON2_MMC_REG_CARG, cmd.arg);
    cctrl = FIELD_PREP(LOONGSON2_MMC_CCTL_INDEX, cmd.opcode);
    cctrl |= LOONGSON2_MMC_CCTL_HOST | LOONGSON2_MMC_CCTL_START;
    if (cmd.opcode == SD_SWITCH && cmd.data)
    cctrl |= LOONGSON2_MMC_CCTL_CMD6;
    if (cmd.flags & MMC_RSP_PRESENT)
    cctrl |= LOONGSON2_MMC_CCTL_WAIT_RSP;
    if (cmd.flags & MMC_RSP_136)
    cctrl |= LOONGSON2_MMC_CCTL_LONG_RSP;
    regmap_write(host.regmap, LOONGSON2_MMC_REG_CCTL, cctrl);
    }
    static int loongson2_mmc_setup_data(struct loongson2_mmc_host *host,
    struct mmc_data *data)
    {
    u32 dctrl;
    if ((data.blksz & 3) != 0)
    return -EINVAL;
    dctrl = FIELD_PREP(LOONGSON2_MMC_DCTL_BNUM, data.blocks);
    dctrl |= LOONGSON2_MMC_DCTL_START | LOONGSON2_MMC_DCTL_ENDMA;
    if (host.bus_width == MMC_BUS_WIDTH_4)
    dctrl |= LOONGSON2_MMC_DCTL_WIDE;
#[no_mangle]
pub unsafe extern "C" fn if(MMC_BUS_WIDTH_8: host->bus_width ==) -> else {
    else if (host.bus_width == MMC_BUS_WIDTH_8)
    dctrl |= LOONGSON2_MMC_DCTL_8BIT_BUS;
    regmap_write(host.regmap, LOONGSON2_MMC_REG_DCTL, dctrl);
    regmap_write(host.regmap, LOONGSON2_MMC_REG_BSIZE, data.blksz);
    regmap_write(host.regmap, LOONGSON2_MMC_REG_TIMER, U32_MAX);
    return 0;
    }
    static int loongson2_mmc_prepare_dma(struct loongson2_mmc_host *host,
    struct mmc_data *data)
    {
    int ret;
    if (!data)
    return 0;
    ret = loongson2_mmc_setup_data(host, data);
    if (ret)
    return ret;
    host.dma_complete = 0;
    return host.pdata.prepare_dma(host, data);
    }
#[no_mangle]
unsafe extern "C" fn loongson2_mmc_send_request(mmc: *mut mmc_host) {
    static void loongson2_mmc_send_request(struct mmc_host *mmc)
    {
    int ret;
    struct loongson2_mmc_host *host = mmc_priv(mmc);
    struct mmc_request *mrq = host.mrq;
    struct mmc_command *cmd = host.cmd_is_stop ? mrq.stop : mrq.cmd;
    ret = loongson2_mmc_prepare_dma(host, cmd.data);
    if (ret) {
    dev_err(host.dev, "DMA data prepared failed with %d\n", ret);
    cmd.error = ret;
    cmd.data.error = ret;
    mmc_request_done(mmc, mrq);
    return;
    }
    if (host.pdata.fix_data_timeout)
    host.pdata.fix_data_timeout(host, cmd);
    loongson2_mmc_send_command(host, cmd);
// Fix deselect card
    if (cmd.opcode == MMC_SELECT_CARD && cmd.arg == 0) {
    cmd.error = 0;
    mmc_request_done(mmc, mrq);
    }
    }
#[no_mangle]
unsafe extern "C" fn loongson2_mmc_irq_worker(irq: c_int, devid: *mut c_void) -> irqreturn_t {
    static irqreturn_t loongson2_mmc_irq_worker(int irq, void *devid)
    {
    struct loongson2_mmc_host *host = (struct loongson2_mmc_host *)devid;
    struct mmc_host *mmc = mmc_from_priv(host);
    struct mmc_request *mrq = host.mrq;
    struct mmc_command *cmd = host.cmd_is_stop ? mrq.stop : mrq.cmd;
    if (cmd.data)
    dma_unmap_sg(mmc_dev(mmc), cmd.data.sg, cmd.data.sg_len,
    mmc_get_dma_dir(cmd.data));
    if (cmd.data && !cmd.error &&
    !cmd.data.error && !host.dma_complete)
    return IRQ_HANDLED;
// Read response from controller.
    regmap_read(host.regmap, LOONGSON2_MMC_REG_RSP0, &cmd.resp[0]);
    regmap_read(host.regmap, LOONGSON2_MMC_REG_RSP1, &cmd.resp[1]);
    regmap_read(host.regmap, LOONGSON2_MMC_REG_RSP2, &cmd.resp[2]);
    regmap_read(host.regmap, LOONGSON2_MMC_REG_RSP3, &cmd.resp[3]);
// Cleanup controller
    regmap_write(host.regmap, LOONGSON2_MMC_REG_CARG, 0);
    regmap_write(host.regmap, LOONGSON2_MMC_REG_CCTL, 0);
    if (cmd.data && cmd.error)
    cmd.data.error = cmd.error;
    if (cmd.data && cmd.data.stop && !host.cmd_is_stop) {
    host.cmd_is_stop = 1;
    loongson2_mmc_send_request(mmc);
    return IRQ_HANDLED;
    }
// If we have no data transfer we are finished here
    if (!mrq.data)
    goto request_done;
// Calculate the amount of bytes transfer if there was no error
    if (mrq.data.error == 0) {
    mrq.data.bytes_xfered =
    (mrq.data.blocks * mrq.data.blksz);
    } else {
    mrq.data.bytes_xfered = 0;
    }
    request_done:
    host.state = STATE_NONE;
    host.mrq = core::ptr::null_mut();
    mmc_request_done(mmc, mrq);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn loongson2_mmc_irq(irq: c_int, devid: *mut c_void) -> irqreturn_t {
    static irqreturn_t loongson2_mmc_irq(int irq, void *devid)
    {
    struct loongson2_mmc_host *host = (struct loongson2_mmc_host *)devid;
    struct mmc_host *mmc = mmc_from_priv(host);
    struct mmc_command *cmd;
    unsigned long iflags;
    u32 dsts, imsk;
    regmap_read(host.regmap, LOONGSON2_MMC_REG_INT, &imsk);
    regmap_read(host.regmap, LOONGSON2_MMC_REG_DSTS, &dsts);
    if ((dsts & LOONGSON2_MMC_DSTS_IRQ) &&
    (imsk & LOONGSON2_MMC_INT_SDIOIRQ)) {
    regmap_update_bits(host.regmap, LOONGSON2_MMC_REG_INT,
    LOONGSON2_MMC_INT_SDIOIRQ, LOONGSON2_MMC_INT_SDIOIRQ);
    sdio_signal_irq(mmc);
    return IRQ_HANDLED;
    }
    spin_lock_irqsave(&host.lock, iflags);
    if (host.state == STATE_NONE || host.state == STATE_FINALIZE || !host.mrq)
    goto irq_out;
    cmd = host.cmd_is_stop ? host.mrq.stop : host.mrq.cmd;
    if (!cmd)
    goto irq_out;
    cmd.error = 0;
    if (imsk & LOONGSON2_MMC_INT_CTIMEOUT) {
    cmd.error = -ETIMEDOUT;
    goto close_transfer;
    }
    if (imsk & LOONGSON2_MMC_INT_CSENT) {
    if (host.state == STATE_RSPFIN || host.state == STATE_CMDSENT)
    goto close_transfer;
    if (host.state == STATE_XFERFINISH_RSPFIN)
    host.state = STATE_XFERFINISH;
    }
    if (!cmd.data)
    goto irq_out;
    if (imsk & (LOONGSON2_MMC_INT_RXCRC | LOONGSON2_MMC_INT_TXCRC)) {
    cmd.data.error = -EILSEQ;
    goto close_transfer;
    }
    if (imsk & LOONGSON2_MMC_INT_DTIMEOUT) {
    cmd.data.error = -ETIMEDOUT;
    goto close_transfer;
    }
    if (imsk & LOONGSON2_MMC_INT_DFIN) {
    if (host.state == STATE_XFERFINISH) {
    host.dma_complete = 1;
    goto close_transfer;
    }
    if (host.state == STATE_XFERFINISH_RSPFIN)
    host.state = STATE_RSPFIN;
    }
    irq_out:
    regmap_write(host.regmap, LOONGSON2_MMC_REG_INT, imsk);
    spin_unlock_irqrestore(&host.lock, iflags);
    return IRQ_HANDLED;
    close_transfer:
    host.state = STATE_FINALIZE;
    host.pdata.reorder_cmd_data(host, cmd);
    regmap_write(host.regmap, LOONGSON2_MMC_REG_INT, imsk);
    spin_unlock_irqrestore(&host.lock, iflags);
    return IRQ_WAKE_THREAD;
    }
#[no_mangle]
unsafe extern "C" fn loongson2_mmc_dll_mode_init(host: *mut loongson2_mmc_host) {
    static void loongson2_mmc_dll_mode_init(struct loongson2_mmc_host *host)
    {
    u32 val, pad_delay, delay;
    int ret;
    regmap_update_bits(host.regmap, LOONGSON2_MMC_REG_SEL,
    LOONGSON2_MMC_SEL_DATA, LOONGSON2_MMC_SEL_DATA);
    val = FIELD_PREP(LOONGSON2_MMC_DLLCTL_TIME, 0xc8)
    | FIELD_PREP(LOONGSON2_MMC_DLLCTL_INCRE, 0x1)
    | FIELD_PREP(LOONGSON2_MMC_DLLCTL_START, 0x1)
    | FIELD_PREP(LOONGSON2_MMC_DLLCTL_CLK_MODE, 0x1)
    | FIELD_PREP(LOONGSON2_MMC_DLLCTL_START_BIT, 0x1)
    | FIELD_PREP(LOONGSON2_MMC_DLLCTL_TIME_BPASS, 0xf);
    regmap_write(host.regmap, LOONGSON2_MMC_REG_DLLCTL, val);
    ret = regmap_read_poll_timeout(host.regmap, LOONGSON2_MMC_REG_DLLVAL, val,
    (val & LOONGSON2_MMC_DLLVAL_DONE), 0,
    LOONGSON2_MMC_DLLVAL_TIMEOUT_US);
    if (ret < 0)
    return;
    regmap_read(host.regmap, LOONGSON2_MMC_REG_DLLVAL, &val);
    pad_delay = FIELD_GET(GENMASK(7, 1), val);
    delay = FIELD_PREP(LOONGSON2_MMC_DELAY_PAD, pad_delay)
    | FIELD_PREP(LOONGSON2_MMC_DELAY_RD, pad_delay + 1);
    regmap_write(host.regmap, LOONGSON2_MMC_REG_DELAY, delay);
    }
#[no_mangle]
unsafe extern "C" fn loongson2_mmc_set_clk(host: *mut loongson2_mmc_host, ios: *mut mmc_ios) {
    static void loongson2_mmc_set_clk(struct loongson2_mmc_host *host, struct mmc_ios *ios)
    {
    u32 pre;
    pre = DIV_ROUND_UP(host.current_clk, ios.clock);
    if (pre > 255)
    pre = 255;
    regmap_write(host.regmap, LOONGSON2_MMC_REG_PRE, pre | LOONGSON2_MMC_PRE_EN);
    regmap_update_bits(host.regmap, LOONGSON2_MMC_REG_CTL,
    LOONGSON2_MMC_CTL_ENCLK, LOONGSON2_MMC_CTL_ENCLK);
// EMMC DLL mode setting
    if (ios.timing == MMC_TIMING_UHS_DDR50 || ios.timing == MMC_TIMING_MMC_DDR52)
    loongson2_mmc_dll_mode_init(host);
    }
#[no_mangle]
unsafe extern "C" fn loongson2_mmc_set_ios(mmc: *mut mmc_host, ios: *mut mmc_ios) {
    static void loongson2_mmc_set_ios(struct mmc_host *mmc, struct mmc_ios *ios)
    {
    struct loongson2_mmc_host *host = mmc_priv(mmc);
    int ret;
    if (ios.power_mode == MMC_POWER_UP) {
    if (!IS_ERR(mmc.supply.vmmc)) {
    ret = mmc_regulator_set_ocr(mmc, mmc.supply.vmmc, ios.vdd);
    if (ret) {
    dev_err(host.dev, "failed to enable vmmc regulator\n");
    return; /* return, if failed turn on vmmc */
    }
    }
    regmap_write(host.regmap, LOONGSON2_MMC_REG_CTL, LOONGSON2_MMC_CTL_RESET);
    mdelay(10);
    regmap_write(host.regmap, LOONGSON2_MMC_REG_CTL, LOONGSON2_MMC_CTL_EXTCLK);
    regmap_write(host.regmap, LOONGSON2_MMC_REG_INT, LOONGSON2_MMC_IEN_ALL);
    regmap_write(host.regmap, LOONGSON2_MMC_REG_IEN, LOONGSON2_MMC_INT_CLEAR);
    } else if (ios.power_mode == MMC_POWER_OFF) {
    regmap_update_bits(host.regmap, LOONGSON2_MMC_REG_CTL,
    LOONGSON2_MMC_CTL_RESET, LOONGSON2_MMC_CTL_RESET);
    if (!IS_ERR(mmc.supply.vmmc))
    mmc_regulator_set_ocr(mmc, mmc.supply.vmmc, 0);
    return;
    }
    loongson2_mmc_set_clk(host, ios);
    host.bus_width = ios.bus_width;
    }
#[no_mangle]
unsafe extern "C" fn loongson2_mmc_request(mmc: *mut mmc_host, mrq: *mut mmc_request) {
    static void loongson2_mmc_request(struct mmc_host *mmc, struct mmc_request *mrq)
    {
    struct loongson2_mmc_host *host = mmc_priv(mmc);
    if ((host.pdata.flags & LOONGSON2_MMC_CMD48_QUIRK) &&
    mrq.cmd.opcode == SD_READ_EXTR_SINGLE) {
    mmc_request_done(mmc, mrq);
    return;
    }
    host.cmd_is_stop = 0;
    host.mrq = mrq;
    loongson2_mmc_send_request(mmc);
    }
#[no_mangle]
unsafe extern "C" fn loongson2_mmc_enable_sdio_irq(mmc: *mut mmc_host, enable: c_int) {
    static void loongson2_mmc_enable_sdio_irq(struct mmc_host *mmc, int enable)
    {
    struct loongson2_mmc_host *host = mmc_priv(mmc);
    regmap_update_bits(host.regmap, LOONGSON2_MMC_REG_IEN, LOONGSON2_MMC_INT_SDIOIRQ, enable);
    }
#[no_mangle]
unsafe extern "C" fn loongson2_mmc_ack_sdio_irq(mmc: *mut mmc_host) {
    static void loongson2_mmc_ack_sdio_irq(struct mmc_host *mmc)
    {
    loongson2_mmc_enable_sdio_irq(mmc, 1);
    }
    static struct mmc_host_ops loongson2_mmc_ops = {
    .request	= loongson2_mmc_request,
    .set_ios	= loongson2_mmc_set_ios,
    .get_ro		= mmc_gpio_get_ro,
    .get_cd		= mmc_gpio_get_cd,
    .enable_sdio_irq = loongson2_mmc_enable_sdio_irq,
    .ack_sdio_irq	= loongson2_mmc_ack_sdio_irq,
    };
    static const struct regmap_config ls2k0500_mmc_regmap_config = {
    .reg_bits = 32,
    .val_bits = 32,
    .reg_stride = 4,
    .max_register = LOONGSON2_MMC_REG_IEN,
    };
    static int loongson2_reorder_cmd_list[] = { SD_APP_SEND_SCR, SD_APP_SEND_NUM_WR_BLKS,
    SD_APP_SD_STATUS, MMC_SEND_WRITE_PROT, SD_SWITCH };
//
// According to SD spec, ACMD13, ACMD22, ACMD51 and CMD30
// response datas has different byte order with usual data packets.
// However sdio controller will send these datas in usual data format,
// so we need to adjust these datas to a protocol consistent byte order.
//
    static void ls2k0500_mmc_reorder_cmd_data(struct loongson2_mmc_host *host,
    struct mmc_command *cmd)
    {
    struct scatterlist *sg;
    u32 *data;
    int i, j;
    if (mmc_cmd_type(cmd) != MMC_CMD_ADTC)
    return;
    for (i = 0; i < ARRAY_SIZE(loongson2_reorder_cmd_list); i++)
    if (cmd.opcode == loongson2_reorder_cmd_list[i])
    break;
    if (i == ARRAY_SIZE(loongson2_reorder_cmd_list))
    return;
    for_each_sg(cmd.data.sg, sg, cmd.data.sg_len, i) {
    data = sg_virt(sg);
    for (j = 0; j < (sg_dma_len(sg) / 4); j++)
    if (cmd.opcode == SD_SWITCH)
    data[j] = bitrev8x4(data[j]);
    else
    data[j] = ( u32)cpu_to_be32(data[j]);
    }
    }
    static int loongson2_mmc_prepare_external_dma(struct loongson2_mmc_host *host,
    struct mmc_data *data)
    {
    struct mmc_host *mmc = mmc_from_priv(host);
    let mut dma_conf: dma_slave_config = { };
    struct dma_async_tx_descriptor *desc;
    int ret;
    ret = dma_map_sg(mmc_dev(mmc), data.sg, data.sg_len,
    mmc_get_dma_dir(data));
    if (!ret)
    return -ENOMEM;
    dma_conf.src_addr = host.res.start + LOONGSON2_MMC_REG_DATA,
    dma_conf.dst_addr = host.res.start + LOONGSON2_MMC_REG_DATA,
    dma_conf.src_addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES,
    dma_conf.dst_addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES,
    dma_conf.direction = !(data.flags & MMC_DATA_WRITE) ? DMA_DEV_TO_MEM : DMA_MEM_TO_DEV;
    dmaengine_slave_config(host.chan, &dma_conf);
    desc = dmaengine_prep_slave_sg(host.chan, data.sg, data.sg_len,
    dma_conf.direction,
    DMA_CTRL_ACK | DMA_PREP_INTERRUPT);
    if (!desc)
    goto unmap_exit;
    dmaengine_submit(desc);
    dma_async_issue_pending(host.chan);
    return 0;
    unmap_exit:
    dma_unmap_sg(mmc_dev(mmc), data.sg, data.sg_len, mmc_get_dma_dir(data));
    return -ENOMEM;
    }
    static void loongson2_mmc_release_external_dma(struct loongson2_mmc_host *host,
    struct device *dev)
    {
    dma_release_channel(host.chan);
    }
    static int ls2k0500_mmc_set_external_dma(struct loongson2_mmc_host *host,
    struct platform_device *pdev)
    {
    int ret, val;
    void __iomem *regs;
    regs = devm_platform_ioremap_resource(pdev, 1);
    if (IS_ERR(regs))
    return PTR_ERR(regs);
    val = readl(regs);
    val |= FIELD_PREP(LS2K0500_SDIO_DMA_MASK, LS2K0500_DMA2_CONF);
    writel(val, regs);
    host.chan = dma_request_chan(&pdev.dev, "rx-tx");
    ret = PTR_ERR_OR_ZERO(host.chan);
    if (ret) {
    dev_err(&pdev.dev, "Cannot get DMA channel.\n");
    return ret;
    }
    return 0;
    }
    static int ls2k1000_mmc_set_external_dma(struct loongson2_mmc_host *host,
    struct platform_device *pdev)
    {
    int ret, val;
    void __iomem *regs;
    regs = devm_platform_ioremap_resource(pdev, 1);
    if (IS_ERR(regs))
    return PTR_ERR(regs);
    val = readl(regs);
    val |= FIELD_PREP(LS2K1000_SDIO_DMA_MASK, LS2K1000_DMA1_CONF);
    writel(val, regs);
    host.chan = dma_request_chan(&pdev.dev, "rx-tx");
    ret = PTR_ERR_OR_ZERO(host.chan);
    if (ret) {
    dev_err(&pdev.dev, "Cannot get DMA channel.\n");
    return ret;
    }
    return 0;
    }
    static const struct regmap_config ls2k2000_mmc_regmap_config = {
    .reg_bits = 32,
    .val_bits = 32,
    .reg_stride = 4,
    .max_register = LOONGSON2_MMC_REG_RDMA_HI,
    };
    static void ls2k2000_mmc_reorder_cmd_data(struct loongson2_mmc_host *host,
    struct mmc_command *cmd)
    {
    struct scatterlist *sg;
    u32 *data;
    int i, j;
    if (cmd.opcode != SD_SWITCH || mmc_cmd_type(cmd) != MMC_CMD_ADTC)
    return;
    for_each_sg(cmd.data.sg, sg, cmd.data.sg_len, i) {
    data = sg_virt(sg);
    for (j = 0; j < (sg_dma_len(sg) / 4); j++)
    data[j] = bitrev8x4(data[j]);
    }
    }
//
// This is a controller hardware defect. Single/multiple block write commands
// must be sent after the TX FULL flag is set, otherwise a data timeout interrupt
// will occur.
//
    static void ls2k2000_mmc_fix_data_timeout(struct loongson2_mmc_host *host,
    struct mmc_command *cmd)
    {
    int val;
    if (cmd.opcode != MMC_WRITE_BLOCK && cmd.opcode != MMC_WRITE_MULTIPLE_BLOCK)
    return;
    regmap_read_poll_timeout(host.regmap, LOONGSON2_MMC_REG_FSTS, val,
    (val & LOONGSON2_MMC_FSTS_TXFULL), 0,
    LOONGSON2_MMC_TXFULL_TIMEOUT_US);
    }
    static int loongson2_mmc_prepare_internal_dma(struct loongson2_mmc_host *host,
    struct mmc_data *data)
    {
    struct loongson2_dma_desc *pdes = (struct loongson2_dma_desc *)host.sg_cpu;
    struct mmc_host *mmc = mmc_from_priv(host);
    let mut next_desc: dma_addr_t = host.sg_dma;
    struct scatterlist *sg;
    int reg_lo, reg_hi;
    u64 dma_order;
    int i, ret;
    ret = dma_map_sg(mmc_dev(mmc), data.sg, data.sg_len,
    mmc_get_dma_dir(data));
    if (!ret)
    return -ENOMEM;
    for_each_sg(data.sg, sg, data.sg_len, i) {
    pdes[i].len = sg_dma_len(&sg[i]) / 4;
    pdes[i].step_len = 0;
    pdes[i].step_times = 1;
    pdes[i].mem_addr = lower_32_bits(sg_dma_address(&sg[i]));
    pdes[i].high_mem_addr = upper_32_bits(sg_dma_address(&sg[i]));
    pdes[i].apb_addr = host.res.start + LOONGSON2_MMC_REG_DATA;
    pdes[i].cmd = LOONGSON2_MMC_DMA_INT;
    if (data.flags & MMC_DATA_READ) {
    reg_lo = LOONGSON2_MMC_REG_RDMA_LO;
    reg_hi = LOONGSON2_MMC_REG_RDMA_HI;
    } else {
    pdes[i].cmd |= LOONGSON2_MMC_DMA_DATA_DIR;
    reg_lo = LOONGSON2_MMC_REG_WDMA_LO;
    reg_hi = LOONGSON2_MMC_REG_WDMA_HI;
    }
    next_desc += sizeof(struct loongson2_dma_desc);
    pdes[i].ndesc_addr = lower_32_bits(next_desc) |
    LOONGSON2_MMC_DMA_DESC_EN;
    pdes[i].high_ndesc_addr = upper_32_bits(next_desc);
    }
// Setting the last descriptor enable bit
    pdes[i - 1].ndesc_addr &= ~LOONGSON2_MMC_DMA_DESC_EN;
    dma_order = (host.sg_dma & ~LOONGSON2_MMC_DMA_CONFIG_MASK) |
    LOONGSON2_MMC_DMA_64BIT_EN |
    LOONGSON2_MMC_DMA_START;
    regmap_write(host.regmap, reg_hi, upper_32_bits(dma_order));
    regmap_write(host.regmap, reg_lo, lower_32_bits(dma_order));
    return 0;
    }
    static int ls2k2000_mmc_set_internal_dma(struct loongson2_mmc_host *host,
    struct platform_device *pdev)
    {
    host.sg_cpu = dma_alloc_coherent(&pdev.dev, PAGE_SIZE,
    &host.sg_dma, GFP_KERNEL);
    if (!host.sg_cpu)
    return -ENOMEM;
    return 0;
    }
    static void loongson2_mmc_release_internal_dma(struct loongson2_mmc_host *host,
    struct device *dev)
    {
    dma_free_coherent(dev, PAGE_SIZE, host.sg_cpu, host.sg_dma);
    }
    static struct loongson2_mmc_pdata ls2k0300_mmc_pdata = {
    .flags			= LOONGSON2_MMC_CMD48_QUIRK,
    .regmap_config		= &ls2k2000_mmc_regmap_config,
    .reorder_cmd_data	= ls2k2000_mmc_reorder_cmd_data,
    .fix_data_timeout	= ls2k2000_mmc_fix_data_timeout,
    .setting_dma		= ls2k2000_mmc_set_internal_dma,
    .prepare_dma		= loongson2_mmc_prepare_internal_dma,
    .release_dma		= loongson2_mmc_release_internal_dma,
    };
    static struct loongson2_mmc_pdata ls2k0500_mmc_pdata = {
    .flags			= 0,
    .regmap_config		= &ls2k0500_mmc_regmap_config,
    .reorder_cmd_data	= ls2k0500_mmc_reorder_cmd_data,
    .setting_dma		= ls2k0500_mmc_set_external_dma,
    .prepare_dma		= loongson2_mmc_prepare_external_dma,
    .release_dma		= loongson2_mmc_release_external_dma,
    };
    static struct loongson2_mmc_pdata ls2k1000_mmc_pdata = {
    .flags			= 0,
    .regmap_config		= &ls2k0500_mmc_regmap_config,
    .reorder_cmd_data	= ls2k0500_mmc_reorder_cmd_data,
    .setting_dma		= ls2k1000_mmc_set_external_dma,
    .prepare_dma		= loongson2_mmc_prepare_external_dma,
    .release_dma		= loongson2_mmc_release_external_dma,
    };
    static struct loongson2_mmc_pdata ls2k2000_mmc_pdata = {
    .flags			= 0,
    .regmap_config		= &ls2k2000_mmc_regmap_config,
    .reorder_cmd_data	= ls2k2000_mmc_reorder_cmd_data,
    .fix_data_timeout	= ls2k2000_mmc_fix_data_timeout,
    .setting_dma		= ls2k2000_mmc_set_internal_dma,
    .prepare_dma		= loongson2_mmc_prepare_internal_dma,
    .release_dma		= loongson2_mmc_release_internal_dma,
    };
    static int loongson2_mmc_resource_request(struct platform_device *pdev,
    struct loongson2_mmc_host *host)
    {
    struct device *dev = &pdev.dev;
    void __iomem *base;
    int ret, irq;
    base = devm_platform_get_and_ioremap_resource(pdev, 0, &host.res);
    if (IS_ERR(base))
    return PTR_ERR(base);
    host.regmap = devm_regmap_init_mmio(dev, base, host.pdata.regmap_config);
    if (IS_ERR(host.regmap))
    return PTR_ERR(host.regmap);
    host.clk = devm_clk_get_optional_enabled(dev, core::ptr::null_mut());
    if (IS_ERR(host.clk))
    return PTR_ERR(host.clk);
    if (host.clk) {
    ret = devm_clk_rate_exclusive_get(dev, host.clk);
    if (ret)
    return ret;
    host.current_clk = clk_get_rate(host.clk);
    } else {
// For ACPI, the clock is accessed via the clock-frequency attribute.
    device_property_read_u32(dev, "clock-frequency", &host.current_clk);
    }
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    ret = devm_request_threaded_irq(dev, irq, loongson2_mmc_irq,
    loongson2_mmc_irq_worker,
    IRQF_ONESHOT, "loongson2-mmc", host);
    if (ret)
    return ret;
    ret = dma_set_mask_and_coherent(dev, DMA_BIT_MASK(64));
    if (ret)
    return ret;
    return host.pdata.setting_dma(host, pdev);
    }
#[no_mangle]
unsafe extern "C" fn loongson2_mmc_probe(pdev: *mut platform_device) -> c_int {
    static int loongson2_mmc_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct loongson2_mmc_host *host;
    struct mmc_host	*mmc;
    int ret;
    mmc = devm_mmc_alloc_host(dev, sizeof(*host));
    if (!mmc)
    return -ENOMEM;
    platform_set_drvdata(pdev, mmc);
    host = mmc_priv(mmc);
    host.state = STATE_NONE;
    spin_lock_init(&host.lock);
    host.pdata = device_get_match_data(dev);
    if (!host.pdata)
    return dev_err_probe(dev, -EINVAL, "Failed to get match data\n");
    ret = loongson2_mmc_resource_request(pdev, host);
    if (ret)
    return dev_err_probe(dev, ret, "Failed to request resource\n");
    mmc.ops = &loongson2_mmc_ops;
    mmc.f_min = DIV_ROUND_UP(host.current_clk, 256);
    mmc.f_max = host.current_clk;
    mmc.max_blk_count = 4095;
    mmc.max_blk_size = 4095;
    mmc.max_req_size = mmc.max_blk_count * mmc.max_blk_size;
    mmc.max_segs = 1;
    mmc.max_seg_size = mmc.max_req_size;
// Process SDIO IRQs through the sdio_irq_work.
    if (mmc.caps & MMC_CAP_SDIO_IRQ)
    mmc.caps2 |= MMC_CAP2_SDIO_IRQ_NOTHREAD;
    ret = mmc_regulator_get_supply(mmc);
    if (ret || mmc.ocr_avail == 0) {
    dev_warn(dev, "Can't get voltage, defaulting to 3.3V\n");
    mmc.ocr_avail = MMC_VDD_32_33 | MMC_VDD_33_34;
    }
    ret = mmc_of_parse(mmc);
    if (ret) {
    dev_err(dev, "Failed to parse device node\n");
    goto free_dma;
    }
    ret = mmc_add_host(mmc);
    if (ret) {
    dev_err(dev, "Failed to add mmc host\n");
    goto free_dma;
    }
    return 0;
    free_dma:
    host.pdata.release_dma(host, dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn loongson2_mmc_remove(pdev: *mut platform_device) {
    static void loongson2_mmc_remove(struct platform_device *pdev)
    {
    struct mmc_host *mmc  = platform_get_drvdata(pdev);
    struct loongson2_mmc_host *host = mmc_priv(mmc);
    mmc_remove_host(mmc);
    host.pdata.release_dma(host, &pdev.dev);
    }
    static const struct of_device_id loongson2_mmc_of_ids[] = {
    { .compatible = "loongson,ls2k0300-mmc", .data = &ls2k0300_mmc_pdata },
    { .compatible = "loongson,ls2k0500-mmc", .data = &ls2k0500_mmc_pdata },
    { .compatible = "loongson,ls2k1000-mmc", .data = &ls2k1000_mmc_pdata },
    { .compatible = "loongson,ls2k2000-mmc", .data = &ls2k2000_mmc_pdata },
    { },
    };
    MODULE_DEVICE_TABLE(of, loongson2_mmc_of_ids);
#[no_mangle]
unsafe extern "C" fn loongson2_mmc_suspend(dev: *mut device) -> c_int {
    static int loongson2_mmc_suspend(struct device *dev)
    {
    struct mmc_host *mmc = dev_get_drvdata(dev);
    struct loongson2_mmc_host *host = mmc_priv(mmc);
    clk_disable_unprepare(host.clk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn loongson2_mmc_resume(dev: *mut device) -> c_int {
    static int loongson2_mmc_resume(struct device *dev)
    {
    struct mmc_host *mmc = dev_get_drvdata(dev);
    struct loongson2_mmc_host *host = mmc_priv(mmc);
    return clk_prepare_enable(host.clk);
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(loongson2_mmc_pm_ops, loongson2_mmc_suspend, loongson2_mmc_resume);
    static struct platform_driver loongson2_mmc_driver = {
    .driver	= {
    .name = "loongson2-mmc",
    .of_match_table = loongson2_mmc_of_ids,
    .pm = pm_ptr(&loongson2_mmc_pm_ops),
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    },
    .probe = loongson2_mmc_probe,
    .remove = loongson2_mmc_remove,
    };
    module_platform_driver(loongson2_mmc_driver);
    MODULE_DESCRIPTION("Loongson-2K SD/SDIO/eMMC Interface driver");
    MODULE_AUTHOR("Loongson Technology Corporation Limited");
    MODULE_LICENSE("GPL");
