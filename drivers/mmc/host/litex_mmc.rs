//! Automatically rewritten from C to Rust
//! Source: drivers/mmc/host/litex_mmc.c
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
// LiteX LiteSDCard driver
//
// Copyright (C) 2019-2020 Antmicro <contact@antmicro.com>
// Copyright (C) 2019-2020 Kamil Rakoczy <krakoczy@antmicro.com>
// Copyright (C) 2019-2020 Maciej Dudek <mdudek@internships.antmicro.com>
// Copyright (C) 2020 Paul Mackerras <paulus@ozlabs.org>
// Copyright (C) 2020-2022 Gabriel Somlo <gsomlo@gmail.com>
//

pub const LITEX_PHY_CARDDETECT: c_uint = 0x00;
pub const LITEX_PHY_CLOCKERDIV: c_uint = 0x04;
pub const LITEX_PHY_INITIALIZE: c_uint = 0x08;
pub const LITEX_PHY_WRITESTATUS: c_uint = 0x0C;
pub const LITEX_PHY_SETTINGS: c_uint = 0x18;
pub const LITEX_CORE_CMDARG: c_uint = 0x00;
pub const LITEX_CORE_CMDCMD: c_uint = 0x04;
pub const LITEX_CORE_CMDSND: c_uint = 0x08;
pub const LITEX_CORE_CMDRSP: c_uint = 0x0C;
pub const LITEX_CORE_CMDEVT: c_uint = 0x1C;
pub const LITEX_CORE_DATEVT: c_uint = 0x20;
pub const LITEX_CORE_BLKLEN: c_uint = 0x24;
pub const LITEX_CORE_BLKCNT: c_uint = 0x28;
pub const LITEX_BLK2MEM_BASE: c_uint = 0x00;
pub const LITEX_BLK2MEM_LEN: c_uint = 0x08;
pub const LITEX_BLK2MEM_ENA: c_uint = 0x0C;
pub const LITEX_BLK2MEM_DONE: c_uint = 0x10;
pub const LITEX_BLK2MEM_LOOP: c_uint = 0x14;
pub const LITEX_MEM2BLK_BASE: c_uint = 0x00;
pub const LITEX_MEM2BLK_LEN: c_uint = 0x08;
pub const LITEX_MEM2BLK_ENA: c_uint = 0x0C;
pub const LITEX_MEM2BLK_DONE: c_uint = 0x10;
pub const LITEX_MEM2BLK_LOOP: c_uint = 0x14;
pub const LITEX_MEM2BLK: c_uint = 0x18;
pub const LITEX_IRQ_STATUS: c_uint = 0x00;
pub const LITEX_IRQ_PENDING: c_uint = 0x04;
pub const LITEX_IRQ_ENABLE: c_uint = 0x08;
pub const SD_CTL_DATA_XFER_NONE: c_int = 0;
pub const SD_CTL_DATA_XFER_READ: c_int = 1;
pub const SD_CTL_DATA_XFER_WRITE: c_int = 2;
pub const SD_CTL_RESP_NONE: c_int = 0;
pub const SD_CTL_RESP_SHORT: c_int = 1;
pub const SD_CTL_RESP_LONG: c_int = 2;
pub const SD_CTL_RESP_SHORT_BUSY: c_int = 3;

pub const SD_SLEEP_US: c_int = 5;
pub const SD_TIMEOUT_US: c_int = 20000;
pub const SD_INIT_DELAY_US: c_int = 1000;
pub const SD_INIT_CLK_HZ: c_int = 400000;
pub const SD_PHY_SPEED_1X: c_int = 0;
pub const SD_PHY_SPEED_4X: c_int = 1;
pub const SD_PHY_SPEED_8X: c_int = 2;
pub const SDIRQ_CARD_DETECT: c_int = 1;
pub const SDIRQ_SD_TO_MEM_DONE: c_int = 2;
pub const SDIRQ_MEM_TO_SD_DONE: c_int = 4;
pub const SDIRQ_CMD_DONE: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct litex_mmc_host {
    pub mmc: *mut mmc_host,
    pub sdphy: *mut void __iomem,
    pub sdcore: *mut void __iomem,
    pub sdreader: *mut void __iomem,
    pub sdwriter: *mut void __iomem,
    pub sdirq: *mut void __iomem,
    pub buffer: *mut c_void,
    pub buf_size: usize,
    pub dma: dma_addr_t,
    pub cmd_done: completion,
    pub irq: c_int,
    pub ref_clk: c_uint,
    pub sd_clk: c_uint,
    pub width: u8,
    pub resp: [u32; 4],
}

#[no_mangle]
unsafe extern "C" fn litex_mmc_sdcard_wait_done(reg: *mut void __iomem, dev: *mut device) -> c_int {
    static int litex_mmc_sdcard_wait_done(void __iomem *reg, struct device *dev)
    {
    u8 evt;
    int ret;
    ret = readx_poll_timeout(litex_read8, reg, evt, evt & SD_BIT_DONE,
    SD_SLEEP_US, SD_TIMEOUT_US);
    if (ret)
    return ret;
    if (evt == SD_BIT_DONE)
    return 0;
    if (evt & SD_BIT_WR_ERR)
    return -EIO;
    if (evt & SD_BIT_TIMEOUT)
    return -ETIMEDOUT;
    if (evt & SD_BIT_CRC_ERR)
    return -EILSEQ;
    dev_err(dev, "%s: unknown error (evt=%x)\n", __func__, evt);
    return -EINVAL;
    }
    static int litex_mmc_send_cmd(struct litex_mmc_host *host,
    u8 cmd, u32 arg, u8 response_len, u8 transfer)
    {
    struct device *dev = mmc_dev(host.mmc);
    void __iomem *reg;
    int ret;
    u8 evt;
    litex_write32(host.sdcore + LITEX_CORE_CMDARG, arg);
    litex_write32(host.sdcore + LITEX_CORE_CMDCMD,
    cmd << 8 | transfer << 5 | response_len);
    litex_write8(host.sdcore + LITEX_CORE_CMDSND, 1);
//
// Wait for an interrupt if we have an interrupt and either there is
// data to be transferred, or if the card can report busy via DAT0.
//
    if (host.irq > 0 &&
    (transfer != SD_CTL_DATA_XFER_NONE ||
    response_len == SD_CTL_RESP_SHORT_BUSY)) {
    reinit_completion(&host.cmd_done);
    litex_write32(host.sdirq + LITEX_IRQ_ENABLE,
    SDIRQ_CMD_DONE | SDIRQ_CARD_DETECT);
    wait_for_completion(&host.cmd_done);
    }
    ret = litex_mmc_sdcard_wait_done(host.sdcore + LITEX_CORE_CMDEVT, dev);
    if (ret) {
    dev_err(dev, "Command (cmd %d) error, status %d\n", cmd, ret);
    return ret;
    }
    if (response_len != SD_CTL_RESP_NONE) {
//
// NOTE: this matches the semantics of litex_read32()
// regardless of underlying arch endianness!
//
    memcpy_fromio(host.resp,
    host.sdcore + LITEX_CORE_CMDRSP, 0x10);
    }
    if (transfer == SD_CTL_DATA_XFER_NONE)
    return ret; /* OK from prior litex_mmc_sdcard_wait_done() */
    ret = litex_mmc_sdcard_wait_done(host.sdcore + LITEX_CORE_DATEVT, dev);
    if (ret) {
    dev_err(dev, "Data xfer (cmd %d) error, status %d\n", cmd, ret);
    return ret;
    }
// Wait for completion of (read or write) DMA transfer
    reg = (transfer == SD_CTL_DATA_XFER_READ) ?
    host.sdreader + LITEX_BLK2MEM_DONE :
    host.sdwriter + LITEX_MEM2BLK_DONE;
    ret = readx_poll_timeout(litex_read8, reg, evt, evt & SD_BIT_DONE,
    SD_SLEEP_US, SD_TIMEOUT_US);
    if (ret)
    dev_err(dev, "DMA timeout (cmd %d)\n", cmd);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn litex_mmc_get_cd(mmc: *mut mmc_host) -> c_int {
    static int litex_mmc_get_cd(struct mmc_host *mmc)
    {
    struct litex_mmc_host *host = mmc_priv(mmc);
    int ret;
    if (!mmc_card_is_removable(mmc))
    return 1;
    ret = !litex_read8(host.sdphy + LITEX_PHY_CARDDETECT);
    if (ret)
    return ret;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn litex_mmc_interrupt(irq: c_int, arg: *mut c_void) -> irqreturn_t {
    static irqreturn_t litex_mmc_interrupt(int irq, void *arg)
    {
    struct mmc_host *mmc = arg;
    struct litex_mmc_host *host = mmc_priv(mmc);
    let mut pending: u32 = litex_read32(host.sdirq + LITEX_IRQ_PENDING);
    let mut ret: irqreturn_t = IRQ_NONE;
// Check for card change interrupt
    if (pending & SDIRQ_CARD_DETECT) {
    litex_write32(host.sdirq + LITEX_IRQ_PENDING,
    SDIRQ_CARD_DETECT);
    mmc_detect_change(mmc, msecs_to_jiffies(10));
    ret = IRQ_HANDLED;
    }
// Check for command completed
    if (pending & SDIRQ_CMD_DONE) {
// Disable it so it doesn't keep interrupting
    litex_write32(host.sdirq + LITEX_IRQ_ENABLE,
    SDIRQ_CARD_DETECT);
    complete(&host.cmd_done);
    ret = IRQ_HANDLED;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn litex_mmc_response_len(cmd: *mut mmc_command) -> u32 {
    static u32 litex_mmc_response_len(struct mmc_command *cmd)
    {
    if (cmd.flags & MMC_RSP_136)
    return SD_CTL_RESP_LONG;
    if (!(cmd.flags & MMC_RSP_PRESENT))
    return SD_CTL_RESP_NONE;
    if (cmd.flags & MMC_RSP_BUSY)
    return SD_CTL_RESP_SHORT_BUSY;
    return SD_CTL_RESP_SHORT;
    }
    static void litex_mmc_do_dma(struct litex_mmc_host *host, struct mmc_data *data,
    unsigned int *len, bool *direct, u8 *transfer)
    {
    struct device *dev = mmc_dev(host.mmc);
    dma_addr_t dma;
    int sg_count;
//
// Try to DMA directly to/from the data buffer.
// We can do that if the buffer can be mapped for DMA
// in one contiguous chunk.
//
    dma = host.dma;
// len = data->blksz * data->blocks;
    sg_count = dma_map_sg(dev, data.sg, data.sg_len,
    mmc_get_dma_dir(data));
    if (sg_count == 1) {
    dma = sg_dma_address(data.sg);
// len = sg_dma_len(data->sg);
// direct = true;
    } else if (*len > host.buf_size)
// len = host->buf_size;
    if (data.flags & MMC_DATA_READ) {
    litex_write8(host.sdreader + LITEX_BLK2MEM_ENA, 0);
    litex_write64(host.sdreader + LITEX_BLK2MEM_BASE, dma);
    litex_write32(host.sdreader + LITEX_BLK2MEM_LEN, *len);
    litex_write8(host.sdreader + LITEX_BLK2MEM_ENA, 1);
// transfer = SD_CTL_DATA_XFER_READ;
    } else if (data.flags & MMC_DATA_WRITE) {
    if (!*direct)
    sg_copy_to_buffer(data.sg, data.sg_len,
    host.buffer, *len);
    litex_write8(host.sdwriter + LITEX_MEM2BLK_ENA, 0);
    litex_write64(host.sdwriter + LITEX_MEM2BLK_BASE, dma);
    litex_write32(host.sdwriter + LITEX_MEM2BLK_LEN, *len);
    litex_write8(host.sdwriter + LITEX_MEM2BLK_ENA, 1);
// transfer = SD_CTL_DATA_XFER_WRITE;
    } else {
    dev_warn(dev, "Data present w/o read or write flag.\n");
// Continue: set cmd status, mark req done
    }
    litex_write16(host.sdcore + LITEX_CORE_BLKLEN, data.blksz);
    litex_write32(host.sdcore + LITEX_CORE_BLKCNT, data.blocks);
    }
#[no_mangle]
unsafe extern "C" fn litex_mmc_request(mmc: *mut mmc_host, mrq: *mut mmc_request) {
    static void litex_mmc_request(struct mmc_host *mmc, struct mmc_request *mrq)
    {
    struct litex_mmc_host *host = mmc_priv(mmc);
    struct device *dev = mmc_dev(mmc);
    struct mmc_command *cmd = mrq.cmd;
    struct mmc_command *sbc = mrq.sbc;
    struct mmc_data *data = mrq.data;
    struct mmc_command *stop = mrq.stop;
    let mut retries: c_uint = cmd.retries;
    let mut len: c_uint = 0;
    let mut direct: bool = false;
    let mut response_len: u32 = litex_mmc_response_len(cmd);
    let mut transfer: u8 = SD_CTL_DATA_XFER_NONE;
// First check that the card is still there
    if (!litex_mmc_get_cd(mmc)) {
    cmd.error = -ENOMEDIUM;
    mmc_request_done(mmc, mrq);
    return;
    }
// Send set-block-count command if needed
    if (sbc) {
    sbc.error = litex_mmc_send_cmd(host, sbc.opcode, sbc.arg,
    litex_mmc_response_len(sbc),
    SD_CTL_DATA_XFER_NONE);
    if (sbc.error) {
    mmc_request_done(mmc, mrq);
    return;
    }
    }
    if (data)
    litex_mmc_do_dma(host, data, &len, &direct, &transfer);
    do {
    cmd.error = litex_mmc_send_cmd(host, cmd.opcode, cmd.arg,
    response_len, transfer);
    } while (cmd.error && retries-- > 0);
    if (response_len == SD_CTL_RESP_SHORT) {
// Pull short response fields from appropriate host registers
    cmd.resp[0] = host.resp[3];
    cmd.resp[1] = host.resp[2] & 0xFF;
    } else if (response_len == SD_CTL_RESP_LONG) {
    cmd.resp[0] = host.resp[0];
    cmd.resp[1] = host.resp[1];
    cmd.resp[2] = host.resp[2];
    cmd.resp[3] = host.resp[3];
    }
// Send stop-transmission command if required
    if (stop && (cmd.error || !sbc))
    stop.error = litex_mmc_send_cmd(host, stop.opcode, stop.arg,
    litex_mmc_response_len(stop),
    SD_CTL_DATA_XFER_NONE);
    if (data) {
    dma_unmap_sg(dev, data.sg, data.sg_len,
    mmc_get_dma_dir(data));
    }
    if (!cmd.error && transfer != SD_CTL_DATA_XFER_NONE) {
    data.bytes_xfered = min(len, mmc.max_req_size);
    if (transfer == SD_CTL_DATA_XFER_READ && !direct) {
    sg_copy_from_buffer(data.sg, sg_nents(data.sg),
    host.buffer, data.bytes_xfered);
    }
    }
    mmc_request_done(mmc, mrq);
    }
#[no_mangle]
unsafe extern "C" fn litex_mmc_setclk(host: *mut litex_mmc_host, freq: c_uint) {
    static void litex_mmc_setclk(struct litex_mmc_host *host, unsigned int freq)
    {
    struct device *dev = mmc_dev(host.mmc);
    u32 div;
    div = freq ? DIV_ROUND_UP(host.ref_clk, freq) : 256U;
    div = clamp(div, 2U, 256U);
    dev_dbg(dev, "sd_clk_freq=%d: set to %d via div=%d\n",
    freq, host.ref_clk / ((div + 1) & ~1U), div);
    litex_write16(host.sdphy + LITEX_PHY_CLOCKERDIV, div);
    host.sd_clk = freq;
    }
#[no_mangle]
unsafe extern "C" fn litex_mmc_set_ios(mmc: *mut mmc_host, ios: *mut mmc_ios) {
    static void litex_mmc_set_ios(struct mmc_host *mmc, struct mmc_ios *ios)
    {
    struct litex_mmc_host *host = mmc_priv(mmc);
    let mut bus_width: c_uint = SD_PHY_SPEED_1X;
    switch (ios.bus_width) {
    case MMC_BUS_WIDTH_1:
    bus_width = SD_PHY_SPEED_1X;
    break;
    case MMC_BUS_WIDTH_4:
    bus_width = SD_PHY_SPEED_4X;
    break;
    case MMC_BUS_WIDTH_8:
    bus_width = SD_PHY_SPEED_8X;
    break;
    }
    if (host.width != ios.bus_width) {
    litex_write8(host.sdphy + LITEX_PHY_SETTINGS, bus_width);
    host.width = ios.bus_width;
    }
//
// The SD specification requires at least 74 idle clocks before CMD0.
// These dummy cycles is generated by writing LITEX_PHY_INITIALIZE.
//
    if (ios.chip_select == MMC_CS_HIGH) {
    litex_mmc_setclk(host, SD_INIT_CLK_HZ);
    litex_write8(host.sdphy + LITEX_PHY_INITIALIZE, 1);
    fsleep(SD_INIT_DELAY_US);
    return;
    }
// Update sd_clk
    if (ios.clock != host.sd_clk)
    litex_mmc_setclk(host, ios.clock);
    }
    static const struct mmc_host_ops litex_mmc_ops = {
    .get_cd = litex_mmc_get_cd,
    .request = litex_mmc_request,
    .set_ios = litex_mmc_set_ios,
    };
    static int litex_mmc_irq_init(struct platform_device *pdev,
    struct litex_mmc_host *host)
    {
    struct device *dev = mmc_dev(host.mmc);
    int ret;
    ret = platform_get_irq_optional(pdev, 0);
    if (ret < 0 && ret != -ENXIO)
    return ret;
    if (ret > 0)
    host.irq = ret;
    else {
    dev_warn(dev, "Failed to get IRQ, using polling\n");
    goto use_polling;
    }
    host.sdirq = devm_platform_ioremap_resource_byname(pdev, "irq");
    if (IS_ERR(host.sdirq))
    return PTR_ERR(host.sdirq);
    ret = devm_request_irq(dev, host.irq, litex_mmc_interrupt, 0,
    "litex-mmc", host.mmc);
    if (ret < 0) {
    dev_warn(dev, "IRQ request error %d, using polling\n", ret);
    goto use_polling;
    }
// Clear & enable card-change interrupts
    litex_write32(host.sdirq + LITEX_IRQ_PENDING, SDIRQ_CARD_DETECT);
    litex_write32(host.sdirq + LITEX_IRQ_ENABLE, SDIRQ_CARD_DETECT);
    return 0;
    use_polling:
    host.mmc.caps |= MMC_CAP_NEEDS_POLL;
    host.irq = 0;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn litex_mmc_probe(pdev: *mut platform_device) -> c_int {
    static int litex_mmc_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct litex_mmc_host *host;
    struct mmc_host *mmc;
    struct clk *clk;
    int ret;
//
// NOTE: defaults to max_[req,seg]_size=PAGE_SIZE, max_blk_size=512,
// and max_blk_count accordingly set to 8;
// If for some reason we need to modify max_blk_count, we must also
// re-calculate `max_[req,seg]_size = max_blk_size * max_blk_count;`
//
    mmc = devm_mmc_alloc_host(dev, sizeof(*host));
    if (!mmc)
    return -ENOMEM;
    host = mmc_priv(mmc);
    host.mmc = mmc;
// Initialize clock source
    clk = devm_clk_get(dev, core::ptr::null_mut());
    if (IS_ERR(clk))
    return dev_err_probe(dev, PTR_ERR(clk), "can't get clock\n");
    host.ref_clk = clk_get_rate(clk);
    host.sd_clk = 0;
    host.width = MMC_BUS_WIDTH_1;
// LiteSDCard can support 64-bit DMA addressing
    ret = dma_set_mask_and_coherent(dev, DMA_BIT_MASK(64));
    if (ret)
    return ret;
    host.buf_size = mmc.max_req_size * 2;
    host.buffer = dmam_alloc_coherent(dev, host.buf_size,
    &host.dma, GFP_KERNEL);
    if (host.buffer == core::ptr::null_mut())
    return -ENOMEM;
    host.sdphy = devm_platform_ioremap_resource_byname(pdev, "phy");
    if (IS_ERR(host.sdphy))
    return PTR_ERR(host.sdphy);
    host.sdcore = devm_platform_ioremap_resource_byname(pdev, "core");
    if (IS_ERR(host.sdcore))
    return PTR_ERR(host.sdcore);
    host.sdreader = devm_platform_ioremap_resource_byname(pdev, "reader");
    if (IS_ERR(host.sdreader))
    return PTR_ERR(host.sdreader);
    host.sdwriter = devm_platform_ioremap_resource_byname(pdev, "writer");
    if (IS_ERR(host.sdwriter))
    return PTR_ERR(host.sdwriter);
// Ensure DMA bus masters are disabled
    litex_write8(host.sdreader + LITEX_BLK2MEM_ENA, 0);
    litex_write8(host.sdwriter + LITEX_MEM2BLK_ENA, 0);
// Ensure the litex is at bus width x1
    litex_write8(host.sdphy + LITEX_PHY_SETTINGS, SD_PHY_SPEED_1X);
    init_completion(&host.cmd_done);
    ret = litex_mmc_irq_init(pdev, host);
    if (ret)
    return ret;
    mmc.ops = &litex_mmc_ops;
    ret = mmc_regulator_get_supply(mmc);
    if (ret || mmc.ocr_avail == 0) {
    dev_warn(dev, "can't get voltage, defaulting to 3.3V\n");
    mmc.ocr_avail = MMC_VDD_32_33 | MMC_VDD_33_34;
    }
//
// Set default sd_clk frequency range based on empirical observations
// of LiteSDCard gateware behavior on typical SDCard media
//
    mmc.f_min = 12.5e6;
    mmc.f_max = 50e6;
    ret = mmc_of_parse(mmc);
    if (ret)
    return ret;
// Only drop 8-bit bus_width support
    mmc.caps &= ~MMC_CAP_8_BIT_DATA;
// Set default capabilities
    mmc.caps |= MMC_CAP_WAIT_WHILE_BUSY |
    MMC_CAP_DRIVER_TYPE_D |
    MMC_CAP_CMD23;
    mmc.caps2 |= MMC_CAP2_NO_WRITE_PROTECT |
    MMC_CAP2_NO_SDIO |
    MMC_CAP2_NO_MMC;
    platform_set_drvdata(pdev, host);
    ret = mmc_add_host(mmc);
    if (ret)
    return ret;
    dev_info(dev, "LiteX MMC controller initialized.\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn litex_mmc_remove(pdev: *mut platform_device) {
    static void litex_mmc_remove(struct platform_device *pdev)
    {
    struct litex_mmc_host *host = platform_get_drvdata(pdev);
    mmc_remove_host(host.mmc);
    }
    static const struct of_device_id litex_match[] = {
    { .compatible = "litex,mmc" },
    { }
    };
    MODULE_DEVICE_TABLE(of, litex_match);
    static struct platform_driver litex_mmc_driver = {
    .probe = litex_mmc_probe,
    .remove = litex_mmc_remove,
    .driver = {
    .name = "litex-mmc",
    .of_match_table = litex_match,
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    },
    };
    module_platform_driver(litex_mmc_driver);
    MODULE_DESCRIPTION("LiteX SDCard driver");
    MODULE_AUTHOR("Antmicro <contact@antmicro.com>");
    MODULE_AUTHOR("Kamil Rakoczy <krakoczy@antmicro.com>");
    MODULE_AUTHOR("Maciej Dudek <mdudek@internships.antmicro.com>");
    MODULE_AUTHOR("Paul Mackerras <paulus@ozlabs.org>");
    MODULE_AUTHOR("Gabriel Somlo <gsomlo@gmail.com>");
    MODULE_LICENSE("GPL v2");
