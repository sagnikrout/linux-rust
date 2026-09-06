//! Automatically rewritten from C to Rust
//! Source: drivers/mmc/host/owl-mmc.c
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
// Actions Semi Owl SoCs SD/MMC driver
//
// Copyright (c) 2014 Actions Semi Inc.
// Copyright (c) 2019 Manivannan Sadhasivam <manivannan.sadhasivam@linaro.org>
//
// TODO: SDIO support
//

//
// SDC registers
//
pub const OWL_REG_SD_EN: c_uint = 0x0000;
pub const OWL_REG_SD_CTL: c_uint = 0x0004;
pub const OWL_REG_SD_STATE: c_uint = 0x0008;
pub const OWL_REG_SD_CMD: c_uint = 0x000c;
pub const OWL_REG_SD_ARG: c_uint = 0x0010;
pub const OWL_REG_SD_RSPBUF0: c_uint = 0x0014;
pub const OWL_REG_SD_RSPBUF1: c_uint = 0x0018;
pub const OWL_REG_SD_RSPBUF2: c_uint = 0x001c;
pub const OWL_REG_SD_RSPBUF3: c_uint = 0x0020;
pub const OWL_REG_SD_RSPBUF4: c_uint = 0x0024;
pub const OWL_REG_SD_DAT: c_uint = 0x0028;
pub const OWL_REG_SD_BLK_SIZE: c_uint = 0x002c;
pub const OWL_REG_SD_BLK_NUM: c_uint = 0x0030;
pub const OWL_REG_SD_BUF_SIZE: c_uint = 0x0034;
// SD_EN Bits

// SD_CTL Bits

pub const OWL_SD_DELAY_LOW_CLK: c_uint = 0x0f;
pub const OWL_SD_DELAY_MID_CLK: c_uint = 0x0a;
pub const OWL_SD_DELAY_HIGH_CLK: c_uint = 0x09;
pub const OWL_SD_RDELAY_DDR50: c_uint = 0x0a;
pub const OWL_SD_WDELAY_DDR50: c_uint = 0x08;
// SD_STATE Bits

pub const OWL_CMD_TIMEOUT_MS: c_int = 30000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct owl_mmc_host {
    pub dev: *mut device,
    pub reset: *mut reset_control,
    pub base: *mut void __iomem,
    pub clk: *mut clk,
    pub sdc_complete: completion,
    pub lock: spinlock_t,
    pub irq: c_int,
    pub clock: u32,
    pub ddr_50: bool,
    pub dma_dir: enum dma_data_direction,
    pub dma: *mut dma_chan,
    pub desc: *mut dma_async_tx_descriptor,
    pub dma_cfg: dma_slave_config,
    pub dma_complete: completion,
    pub mmc: *mut mmc_host,
    pub mrq: *mut mmc_request,
    pub cmd: *mut mmc_command,
    pub data: *mut mmc_data,
}

#[no_mangle]
unsafe extern "C" fn owl_mmc_update_reg(reg: *mut void __iomem, val: c_uint, state: bool) {
    static void owl_mmc_update_reg(void __iomem *reg, unsigned int val, bool state)
    {
    unsigned int regval;
    regval = readl(reg);
    if (state)
    regval |= val;
    else
    regval &= ~val;
    writel(regval, reg);
    }
#[no_mangle]
unsafe extern "C" fn owl_irq_handler(irq: c_int, devid: *mut c_void) -> irqreturn_t {
    static irqreturn_t owl_irq_handler(int irq, void *devid)
    {
    struct owl_mmc_host *owl_host = devid;
    u32 state;
    spin_lock(&owl_host.lock);
    state = readl(owl_host.base + OWL_REG_SD_STATE);
    if (state & OWL_SD_STATE_TEI) {
    state = readl(owl_host.base + OWL_REG_SD_STATE);
    state |= OWL_SD_STATE_TEI;
    writel(state, owl_host.base + OWL_REG_SD_STATE);
    complete(&owl_host.sdc_complete);
    }
    spin_unlock(&owl_host.lock);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn owl_mmc_finish_request(owl_host: *mut owl_mmc_host) {
    static void owl_mmc_finish_request(struct owl_mmc_host *owl_host)
    {
    struct mmc_request *mrq = owl_host.mrq;
    struct mmc_data *data = mrq.data;
// Should never be NULL
    WARN_ON(!mrq);
    owl_host.mrq = core::ptr::null_mut();
    if (data)
    dma_unmap_sg(owl_host.dma.device.dev, data.sg, data.sg_len,
    owl_host.dma_dir);
// Finally finish request
    mmc_request_done(owl_host.mmc, mrq);
    }
    static void owl_mmc_send_cmd(struct owl_mmc_host *owl_host,
    struct mmc_command *cmd,
    struct mmc_data *data)
    {
    unsigned long timeout;
    u32 mode, state, resp[2];
    let mut cmd_rsp_mask: u32 = 0;
    init_completion(&owl_host.sdc_complete);
    switch (mmc_resp_type(cmd)) {
    case MMC_RSP_NONE:
    mode = OWL_SD_CTL_TM(0);
    break;
    case MMC_RSP_R1:
    if (data) {
    if (data.flags & MMC_DATA_READ)
    mode = OWL_SD_CTL_TM(4);
    else
    mode = OWL_SD_CTL_TM(5);
    } else {
    mode = OWL_SD_CTL_TM(1);
    }
    cmd_rsp_mask = OWL_SD_STATE_CLNR | OWL_SD_STATE_CRC7ER;
    break;
    case MMC_RSP_R1B:
    mode = OWL_SD_CTL_TM(3);
    cmd_rsp_mask = OWL_SD_STATE_CLNR | OWL_SD_STATE_CRC7ER;
    break;
    case MMC_RSP_R2:
    mode = OWL_SD_CTL_TM(2);
    cmd_rsp_mask = OWL_SD_STATE_CLNR | OWL_SD_STATE_CRC7ER;
    break;
    case MMC_RSP_R3:
    mode = OWL_SD_CTL_TM(1);
    cmd_rsp_mask = OWL_SD_STATE_CLNR;
    break;
    default:
    dev_warn(owl_host.dev, "Unknown MMC command\n");
    cmd.error = -EINVAL;
    return;
    }
// Keep current WDELAY and RDELAY
    mode |= (readl(owl_host.base + OWL_REG_SD_CTL) & (0xff << 16));
// Start to send corresponding command type
    writel(cmd.arg, owl_host.base + OWL_REG_SD_ARG);
    writel(cmd.opcode, owl_host.base + OWL_REG_SD_CMD);
// Set LBE to send clk at the end of last read block
    if (data) {
    mode |= (OWL_SD_CTL_TS | OWL_SD_CTL_LBE | 0x64000000);
    } else {
    mode &= ~(OWL_SD_CTL_TOUTEN | OWL_SD_CTL_LBE);
    mode |= OWL_SD_CTL_TS;
    }
    owl_host.cmd = cmd;
// Start transfer
    writel(mode, owl_host.base + OWL_REG_SD_CTL);
    if (data)
    return;
    timeout = msecs_to_jiffies(cmd.busy_timeout ? cmd.busy_timeout :
    OWL_CMD_TIMEOUT_MS);
    if (!wait_for_completion_timeout(&owl_host.sdc_complete, timeout)) {
    dev_err(owl_host.dev, "CMD interrupt timeout\n");
    cmd.error = -ETIMEDOUT;
    return;
    }
    state = readl(owl_host.base + OWL_REG_SD_STATE);
    if (mmc_resp_type(cmd) & MMC_RSP_PRESENT) {
    if (cmd_rsp_mask & state) {
    if (state & OWL_SD_STATE_CLNR) {
    dev_err(owl_host.dev, "Error CMD_NO_RSP\n");
    cmd.error = -EILSEQ;
    return;
    }
    if (state & OWL_SD_STATE_CRC7ER) {
    dev_err(owl_host.dev, "Error CMD_RSP_CRC\n");
    cmd.error = -EILSEQ;
    return;
    }
    }
    if (mmc_resp_type(cmd) & MMC_RSP_136) {
    cmd.resp[3] = readl(owl_host.base + OWL_REG_SD_RSPBUF0);
    cmd.resp[2] = readl(owl_host.base + OWL_REG_SD_RSPBUF1);
    cmd.resp[1] = readl(owl_host.base + OWL_REG_SD_RSPBUF2);
    cmd.resp[0] = readl(owl_host.base + OWL_REG_SD_RSPBUF3);
    } else {
    resp[0] = readl(owl_host.base + OWL_REG_SD_RSPBUF0);
    resp[1] = readl(owl_host.base + OWL_REG_SD_RSPBUF1);
    cmd.resp[0] = resp[1] << 24 | resp[0] >> 8;
    cmd.resp[1] = resp[1] >> 8;
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn owl_mmc_dma_complete(param: *mut c_void) {
    static void owl_mmc_dma_complete(void *param)
    {
    struct owl_mmc_host *owl_host = param;
    struct mmc_data *data = owl_host.data;
    if (data)
    complete(&owl_host.dma_complete);
    }
    static int owl_mmc_prepare_data(struct owl_mmc_host *owl_host,
    struct mmc_data *data)
    {
    u32 total;
    owl_mmc_update_reg(owl_host.base + OWL_REG_SD_EN, OWL_SD_EN_BSEL,
    true);
    writel(data.blocks, owl_host.base + OWL_REG_SD_BLK_NUM);
    writel(data.blksz, owl_host.base + OWL_REG_SD_BLK_SIZE);
    total = data.blksz * data.blocks;
    if (total < 512)
    writel(total, owl_host.base + OWL_REG_SD_BUF_SIZE);
    else
    writel(512, owl_host.base + OWL_REG_SD_BUF_SIZE);
    if (data.flags & MMC_DATA_WRITE) {
    owl_host.dma_dir = DMA_TO_DEVICE;
    owl_host.dma_cfg.direction = DMA_MEM_TO_DEV;
    } else {
    owl_host.dma_dir = DMA_FROM_DEVICE;
    owl_host.dma_cfg.direction = DMA_DEV_TO_MEM;
    }
    dma_map_sg(owl_host.dma.device.dev, data.sg,
    data.sg_len, owl_host.dma_dir);
    dmaengine_slave_config(owl_host.dma, &owl_host.dma_cfg);
    owl_host.desc = dmaengine_prep_slave_sg(owl_host.dma, data.sg,
    data.sg_len,
    owl_host.dma_cfg.direction,
    DMA_PREP_INTERRUPT |
    DMA_CTRL_ACK);
    if (!owl_host.desc) {
    dev_err(owl_host.dev, "Can't prepare slave sg\n");
    return -EBUSY;
    }
    owl_host.data = data;
    owl_host.desc.callback = owl_mmc_dma_complete;
    owl_host.desc.callback_param = (void *)owl_host;
    data.error = 0;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn owl_mmc_request(mmc: *mut mmc_host, mrq: *mut mmc_request) {
    static void owl_mmc_request(struct mmc_host *mmc, struct mmc_request *mrq)
    {
    struct owl_mmc_host *owl_host = mmc_priv(mmc);
    struct mmc_data *data = mrq.data;
    int ret;
    owl_host.mrq = mrq;
    if (mrq.data) {
    ret = owl_mmc_prepare_data(owl_host, data);
    if (ret < 0) {
    data.error = ret;
    goto err_out;
    }
    init_completion(&owl_host.dma_complete);
    dmaengine_submit(owl_host.desc);
    dma_async_issue_pending(owl_host.dma);
    }
    owl_mmc_send_cmd(owl_host, mrq.cmd, data);
    if (data) {
    if (!wait_for_completion_timeout(&owl_host.sdc_complete,
    10 * HZ)) {
    dev_err(owl_host.dev, "CMD interrupt timeout\n");
    mrq.cmd.error = -ETIMEDOUT;
    dmaengine_terminate_all(owl_host.dma);
    goto err_out;
    }
    if (!wait_for_completion_timeout(&owl_host.dma_complete,
    5 * HZ)) {
    dev_err(owl_host.dev, "DMA interrupt timeout\n");
    mrq.cmd.error = -ETIMEDOUT;
    dmaengine_terminate_all(owl_host.dma);
    goto err_out;
    }
    if (data.stop)
    owl_mmc_send_cmd(owl_host, data.stop, core::ptr::null_mut());
    data.bytes_xfered = data.blocks * data.blksz;
    }
    err_out:
    owl_mmc_finish_request(owl_host);
    }
    static int owl_mmc_set_clk_rate(struct owl_mmc_host *owl_host,
    unsigned int rate)
    {
    unsigned long clk_rate;
    int ret;
    u32 reg;
    reg = readl(owl_host.base + OWL_REG_SD_CTL);
    reg &= ~OWL_SD_CTL_DELAY_MSK;
// Set RDELAY and WDELAY based on the clock
    if (rate <= 1000000) {
    writel(reg | OWL_SD_CTL_RDELAY(OWL_SD_DELAY_LOW_CLK) |
    OWL_SD_CTL_WDELAY(OWL_SD_DELAY_LOW_CLK),
    owl_host.base + OWL_REG_SD_CTL);
    } else if ((rate > 1000000) && (rate <= 26000000)) {
    writel(reg | OWL_SD_CTL_RDELAY(OWL_SD_DELAY_MID_CLK) |
    OWL_SD_CTL_WDELAY(OWL_SD_DELAY_MID_CLK),
    owl_host.base + OWL_REG_SD_CTL);
    } else if ((rate > 26000000) && (rate <= 52000000) && !owl_host.ddr_50) {
    writel(reg | OWL_SD_CTL_RDELAY(OWL_SD_DELAY_HIGH_CLK) |
    OWL_SD_CTL_WDELAY(OWL_SD_DELAY_HIGH_CLK),
    owl_host.base + OWL_REG_SD_CTL);
// DDR50 mode has special delay chain
    } else if ((rate > 26000000) && (rate <= 52000000) && owl_host.ddr_50) {
    writel(reg | OWL_SD_CTL_RDELAY(OWL_SD_RDELAY_DDR50) |
    OWL_SD_CTL_WDELAY(OWL_SD_WDELAY_DDR50),
    owl_host.base + OWL_REG_SD_CTL);
    } else {
    dev_err(owl_host.dev, "SD clock rate not supported\n");
    return -EINVAL;
    }
    clk_rate = clk_round_rate(owl_host.clk, rate << 1);
    ret = clk_set_rate(owl_host.clk, clk_rate);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn owl_mmc_set_clk(owl_host: *mut owl_mmc_host, ios: *mut mmc_ios) {
    static void owl_mmc_set_clk(struct owl_mmc_host *owl_host, struct mmc_ios *ios)
    {
    if (!ios.clock)
    return;
    owl_host.clock = ios.clock;
    owl_mmc_set_clk_rate(owl_host, ios.clock);
    }
    static void owl_mmc_set_bus_width(struct owl_mmc_host *owl_host,
    struct mmc_ios *ios)
    {
    u32 reg;
    reg = readl(owl_host.base + OWL_REG_SD_EN);
    reg &= ~0x03;
    switch (ios.bus_width) {
    case MMC_BUS_WIDTH_1:
    break;
    case MMC_BUS_WIDTH_4:
    reg |= OWL_SD_EN_DATAWID(1);
    break;
    case MMC_BUS_WIDTH_8:
    reg |= OWL_SD_EN_DATAWID(2);
    break;
    }
    writel(reg, owl_host.base + OWL_REG_SD_EN);
    }
#[no_mangle]
unsafe extern "C" fn owl_mmc_ctr_reset(owl_host: *mut owl_mmc_host) {
    static void owl_mmc_ctr_reset(struct owl_mmc_host *owl_host)
    {
    reset_control_assert(owl_host.reset);
    udelay(20);
    reset_control_deassert(owl_host.reset);
    }
#[no_mangle]
unsafe extern "C" fn owl_mmc_power_on(owl_host: *mut owl_mmc_host) {
    static void owl_mmc_power_on(struct owl_mmc_host *owl_host)
    {
    u32 mode;
    init_completion(&owl_host.sdc_complete);
// Enable transfer end IRQ
    owl_mmc_update_reg(owl_host.base + OWL_REG_SD_STATE,
    OWL_SD_STATE_TEIE, true);
// Send init clk
    mode = (readl(owl_host.base + OWL_REG_SD_CTL) & (0xff << 16));
    mode |= OWL_SD_CTL_TS | OWL_SD_CTL_TCN(5) | OWL_SD_CTL_TM(8);
    writel(mode, owl_host.base + OWL_REG_SD_CTL);
    if (!wait_for_completion_timeout(&owl_host.sdc_complete, HZ)) {
    dev_err(owl_host.dev, "CMD interrupt timeout\n");
    return;
    }
    }
#[no_mangle]
unsafe extern "C" fn owl_mmc_set_ios(mmc: *mut mmc_host, ios: *mut mmc_ios) {
    static void owl_mmc_set_ios(struct mmc_host *mmc, struct mmc_ios *ios)
    {
    struct owl_mmc_host *owl_host = mmc_priv(mmc);
    switch (ios.power_mode) {
    case MMC_POWER_UP:
    dev_dbg(owl_host.dev, "Powering card up\n");
// Reset the SDC controller to clear all previous states
    owl_mmc_ctr_reset(owl_host);
    clk_prepare_enable(owl_host.clk);
    writel(OWL_SD_ENABLE | OWL_SD_EN_RESE,
    owl_host.base + OWL_REG_SD_EN);
    break;
    case MMC_POWER_ON:
    dev_dbg(owl_host.dev, "Powering card on\n");
    owl_mmc_power_on(owl_host);
    break;
    case MMC_POWER_OFF:
    dev_dbg(owl_host.dev, "Powering card off\n");
    clk_disable_unprepare(owl_host.clk);
    return;
    default:
    dev_dbg(owl_host.dev, "Ignoring unknown card power state\n");
    break;
    }
    if (ios.clock != owl_host.clock)
    owl_mmc_set_clk(owl_host, ios);
    owl_mmc_set_bus_width(owl_host, ios);
// Enable DDR mode if requested
    if (ios.timing == MMC_TIMING_UHS_DDR50) {
    owl_host.ddr_50 = true;
    owl_mmc_update_reg(owl_host.base + OWL_REG_SD_EN,
    OWL_SD_EN_DDREN, true);
    } else {
    owl_host.ddr_50 = false;
    }
    }
    static int owl_mmc_start_signal_voltage_switch(struct mmc_host *mmc,
    struct mmc_ios *ios)
    {
    struct owl_mmc_host *owl_host = mmc_priv(mmc);
// It is enough to change the pad ctrl bit for voltage switch
    switch (ios.signal_voltage) {
    case MMC_SIGNAL_VOLTAGE_330:
    owl_mmc_update_reg(owl_host.base + OWL_REG_SD_EN,
    OWL_SD_EN_S18EN, false);
    break;
    case MMC_SIGNAL_VOLTAGE_180:
    owl_mmc_update_reg(owl_host.base + OWL_REG_SD_EN,
    OWL_SD_EN_S18EN, true);
    break;
    default:
    return -ENOTSUPP;
    }
    return 0;
    }
    static const struct mmc_host_ops owl_mmc_ops = {
    .request	= owl_mmc_request,
    .set_ios	= owl_mmc_set_ios,
    .get_ro		= mmc_gpio_get_ro,
    .get_cd		= mmc_gpio_get_cd,
    .start_signal_voltage_switch = owl_mmc_start_signal_voltage_switch,
    };
#[no_mangle]
unsafe extern "C" fn owl_mmc_probe(pdev: *mut platform_device) -> c_int {
    static int owl_mmc_probe(struct platform_device *pdev)
    {
    struct owl_mmc_host *owl_host;
    struct mmc_host *mmc;
    struct resource *res;
    int ret;
    mmc = devm_mmc_alloc_host(&pdev.dev, sizeof(*owl_host));
    if (!mmc) {
    dev_err(&pdev.dev, "mmc alloc host failed\n");
    return -ENOMEM;
    }
    platform_set_drvdata(pdev, mmc);
    owl_host = mmc_priv(mmc);
    owl_host.dev = &pdev.dev;
    owl_host.mmc = mmc;
    spin_lock_init(&owl_host.lock);
    owl_host.base = devm_platform_get_and_ioremap_resource(pdev, 0, &res);
    if (IS_ERR(owl_host.base))
    return PTR_ERR(owl_host.base);
    owl_host.clk = devm_clk_get(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(owl_host.clk))
    return dev_err_probe(&pdev.dev, PTR_ERR(owl_host.clk),
    "No clock defined\n");
    owl_host.reset = devm_reset_control_get_exclusive(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(owl_host.reset))
    return dev_err_probe(&pdev.dev, PTR_ERR(owl_host.reset),
    "Could not get reset control\n");
    mmc.ops		= &owl_mmc_ops;
    mmc.max_blk_count	= 512;
    mmc.max_blk_size	= 512;
    mmc.max_segs		= 256;
    mmc.max_seg_size	= 262144;
    mmc.max_req_size	= 262144;
// 100kHz ~ 52MHz
    mmc.f_min		= 100000;
    mmc.f_max		= 52000000;
    mmc.caps	       |= MMC_CAP_MMC_HIGHSPEED | MMC_CAP_SD_HIGHSPEED |
    MMC_CAP_4_BIT_DATA;
    mmc.caps2		= (MMC_CAP2_BOOTPART_NOACC | MMC_CAP2_NO_SDIO);
    mmc.ocr_avail		= MMC_VDD_32_33 | MMC_VDD_33_34 |
    MMC_VDD_165_195;
    ret = mmc_of_parse(mmc);
    if (ret)
    return ret;
    pdev.dev.coherent_dma_mask = DMA_BIT_MASK(32);
    pdev.dev.dma_mask = &pdev.dev.coherent_dma_mask;
    owl_host.dma = dma_request_chan(&pdev.dev, "mmc");
    if (IS_ERR(owl_host.dma))
    return dev_err_probe(&pdev.dev, PTR_ERR(owl_host.dma),
    "Failed to get external DMA channel.\n");
    dev_info(&pdev.dev, "Using %s for DMA transfers\n",
    dma_chan_name(owl_host.dma));
    owl_host.dma_cfg.src_addr = res.start + OWL_REG_SD_DAT;
    owl_host.dma_cfg.dst_addr = res.start + OWL_REG_SD_DAT;
    owl_host.dma_cfg.src_addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES;
    owl_host.dma_cfg.dst_addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES;
    owl_host.dma_cfg.device_fc = false;
    owl_host.irq = platform_get_irq(pdev, 0);
    if (owl_host.irq < 0) {
    ret = owl_host.irq;
    goto err_release_channel;
    }
    ret = devm_request_irq(&pdev.dev, owl_host.irq, owl_irq_handler,
    0, dev_name(&pdev.dev), owl_host);
    if (ret)
    goto err_release_channel;
    ret = mmc_add_host(mmc);
    if (ret) {
    dev_err(&pdev.dev, "Failed to add host\n");
    goto err_release_channel;
    }
    dev_dbg(&pdev.dev, "Owl MMC Controller Initialized\n");
    return 0;
    err_release_channel:
    dma_release_channel(owl_host.dma);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn owl_mmc_remove(pdev: *mut platform_device) {
    static void owl_mmc_remove(struct platform_device *pdev)
    {
    struct mmc_host	*mmc = platform_get_drvdata(pdev);
    struct owl_mmc_host *owl_host = mmc_priv(mmc);
    mmc_remove_host(mmc);
    disable_irq(owl_host.irq);
    dma_release_channel(owl_host.dma);
    }
    static const struct of_device_id owl_mmc_of_match[] = {
    {.compatible = "actions,owl-mmc",},
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, owl_mmc_of_match);
    static struct platform_driver owl_mmc_driver = {
    .driver = {
    .name	= "owl_mmc",
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    .of_match_table = owl_mmc_of_match,
    },
    .probe		= owl_mmc_probe,
    .remove		= owl_mmc_remove,
    };
    module_platform_driver(owl_mmc_driver);
    MODULE_DESCRIPTION("Actions Semi Owl SoCs SD/MMC Driver");
    MODULE_AUTHOR("Actions Semi");
    MODULE_AUTHOR("Manivannan Sadhasivam <manivannan.sadhasivam@linaro.org>");
    MODULE_LICENSE("GPL");
