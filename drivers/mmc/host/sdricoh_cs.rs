//! Automatically rewritten from C to Rust
//! Source: drivers/mmc/host/sdricoh_cs.c
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
// sdricoh_cs.c - driver for Ricoh Secure Digital Card Readers that can be
// found on some Ricoh RL5c476 II cardbus bridge
//
// Copyright (C) 2006 - 2008 Sascha Sommer <saschasommer@freenet.de>
//
// Macro flag: #define DEBUG
// Macro flag: #define VERBOSE_DEBUG
//

    static unsigned int switchlocked;
// i/o region
pub const SDRICOH_PCI_REGION: c_int = 0;
pub const SDRICOH_PCI_REGION_SIZE: c_uint = 0x1000;
// registers
pub const R104_VERSION: c_uint = 0x104;
pub const R200_CMD: c_uint = 0x200;
pub const R204_CMD_ARG: c_uint = 0x204;
pub const R208_DATAIO: c_uint = 0x208;
pub const R20C_RESP: c_uint = 0x20c;
pub const R21C_STATUS: c_uint = 0x21c;
pub const R2E0_INIT: c_uint = 0x2e0;
pub const R2E4_STATUS_RESP: c_uint = 0x2e4;
pub const R2F0_RESET: c_uint = 0x2f0;
pub const R224_MODE: c_uint = 0x224;
pub const R226_BLOCKSIZE: c_uint = 0x226;
pub const R228_POWER: c_uint = 0x228;
pub const R230_DATA: c_uint = 0x230;
// flags for the R21C_STATUS register
pub const STATUS_CMD_FINISHED: c_uint = 0x00000001;
pub const STATUS_TRANSFER_FINISHED: c_uint = 0x00000004;
pub const STATUS_CARD_INSERTED: c_uint = 0x00000020;
pub const STATUS_CARD_LOCKED: c_uint = 0x00000080;
pub const STATUS_CMD_TIMEOUT: c_uint = 0x00400000;
pub const STATUS_READY_TO_READ: c_uint = 0x01000000;
pub const STATUS_READY_TO_WRITE: c_uint = 0x02000000;
pub const STATUS_BUSY: c_uint = 0x40000000;
// timeouts
pub const SDRICOH_CMD_TIMEOUT_US: c_int = 1000000;
pub const SDRICOH_DATA_TIMEOUT_US: c_int = 1000000;
// list of supported pcmcia devices
    static const struct pcmcia_device_id pcmcia_ids[] = {
// vendor and device strings followed by their crc32 hashes
    PCMCIA_DEVICE_PROD_ID12("RICOH", "Bay1Controller", 0xd9f522ed,
    0xc3901202),
    PCMCIA_DEVICE_PROD_ID12("RICOH", "Bay Controller", 0xd9f522ed,
    0xace80909),
    PCMCIA_DEVICE_NULL,
    };
    MODULE_DEVICE_TABLE(pcmcia, pcmcia_ids);
// mmc privdata
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdricoh_host {
    pub dev: *mut device,
    pub /: *mut *mut *mut mmc_host mmc; / MMC structure,
    pub iobase: *mut unsigned char __iomem,
    pub pci_dev: *mut pci_dev,
    pub app_cmd: c_int,
}

// register i/o helper functions
    static inline unsigned int sdricoh_readl(struct sdricoh_host *host,
    unsigned int reg)
    {
    let mut value: c_uint = readl(host.iobase + reg);
    dev_vdbg(host.dev, "rl %x 0x%x\n", reg, value);
    return value;
    }
    static inline void sdricoh_writel(struct sdricoh_host *host, unsigned int reg,
    unsigned int value)
    {
    writel(value, host.iobase + reg);
    dev_vdbg(host.dev, "wl %x 0x%x\n", reg, value);
    }
    static inline void sdricoh_writew(struct sdricoh_host *host, unsigned int reg,
    unsigned short value)
    {
    writew(value, host.iobase + reg);
    dev_vdbg(host.dev, "ww %x 0x%x\n", reg, value);
    }
    static inline unsigned int sdricoh_readb(struct sdricoh_host *host,
    unsigned int reg)
    {
    let mut value: c_uint = readb(host.iobase + reg);
    dev_vdbg(host.dev, "rb %x 0x%x\n", reg, value);
    return value;
    }
    static bool sdricoh_status_ok(struct sdricoh_host *host, unsigned int status,
    unsigned int wanted)
    {
    sdricoh_writel(host, R2E4_STATUS_RESP, status);
    return status & wanted;
    }
#[no_mangle]
unsafe extern "C" fn sdricoh_query_status(host: *mut sdricoh_host, wanted: c_uint) -> c_int {
    static int sdricoh_query_status(struct sdricoh_host *host, unsigned int wanted)
    {
    int ret;
    let mut status: c_uint = 0;
    struct device *dev = host.dev;
    ret = read_poll_timeout(sdricoh_readl, status,
    sdricoh_status_ok(host, status, wanted),
    32, SDRICOH_DATA_TIMEOUT_US, false,
    host, R21C_STATUS);
    if (ret) {
    dev_err(dev, "query_status: timeout waiting for %x\n", wanted);
    return -ETIMEDOUT;
    }
// do not do this check in the loop as some commands fail otherwise
    if (status & 0x7F0000) {
    dev_err(dev, "waiting for status bit %x failed\n", wanted);
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sdricoh_mmc_cmd(host: *mut sdricoh_host, cmd: *mut mmc_command) -> c_int {
    static int sdricoh_mmc_cmd(struct sdricoh_host *host, struct mmc_command *cmd)
    {
    unsigned int status, timeout_us;
    int ret;
    let mut opcode: c_uchar = cmd.opcode;
// reset status reg?
    sdricoh_writel(host, R21C_STATUS, 0x18);
// MMC_APP_CMDs need some special handling
    if (host.app_cmd) {
    opcode |= 64;
    host.app_cmd = 0;
    } else if (opcode == MMC_APP_CMD)
    host.app_cmd = 1;
// fill parameters
    sdricoh_writel(host, R204_CMD_ARG, cmd.arg);
    sdricoh_writel(host, R200_CMD, (0x10000 << 8) | opcode);
// wait for command completion
    if (!opcode)
    return 0;
    timeout_us = cmd.busy_timeout ? cmd.busy_timeout * 1000 :
    SDRICOH_CMD_TIMEOUT_US;
    ret = read_poll_timeout(sdricoh_readl, status,
    sdricoh_status_ok(host, status, STATUS_CMD_FINISHED),
    32, timeout_us, false,
    host, R21C_STATUS);
//
// Don't check for timeout status in the loop, as it's not always reset
// correctly.
//
    if (ret || status & STATUS_CMD_TIMEOUT)
    return -ETIMEDOUT;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sdricoh_reset(host: *mut sdricoh_host) -> c_int {
    static int sdricoh_reset(struct sdricoh_host *host)
    {
    dev_dbg(host.dev, "reset\n");
    sdricoh_writel(host, R2F0_RESET, 0x10001);
    sdricoh_writel(host, R2E0_INIT, 0x10000);
    if (sdricoh_readl(host, R2E0_INIT) != 0x10000)
    return -EIO;
    sdricoh_writel(host, R2E0_INIT, 0x10007);
    sdricoh_writel(host, R224_MODE, 0x2000000);
    sdricoh_writel(host, R228_POWER, 0xe0);
// status register ?
    sdricoh_writel(host, R21C_STATUS, 0x18);
    return 0;
    }
    static int sdricoh_blockio(struct sdricoh_host *host, int read,
    u8 *buf, int len)
    {
    int size;
    let mut data: u32 = 0;
// wait until the data is available
    if (read) {
    if (sdricoh_query_status(host, STATUS_READY_TO_READ))
    return -ETIMEDOUT;
    sdricoh_writel(host, R21C_STATUS, 0x18);
// read data
    while (len) {
    data = sdricoh_readl(host, R230_DATA);
    size = min(len, 4);
    len -= size;
    while (size) {
// buf = data & 0xFF;
    buf++;
    data >>= 8;
    size--;
    }
    }
    } else {
    if (sdricoh_query_status(host, STATUS_READY_TO_WRITE))
    return -ETIMEDOUT;
    sdricoh_writel(host, R21C_STATUS, 0x18);
// write data
    while (len) {
    size = min(len, 4);
    len -= size;
    while (size) {
    data >>= 8;
    data |= (u32)*buf << 24;
    buf++;
    size--;
    }
    sdricoh_writel(host, R230_DATA, data);
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sdricoh_request(mmc: *mut mmc_host, mrq: *mut mmc_request) {
    static void sdricoh_request(struct mmc_host *mmc, struct mmc_request *mrq)
    {
    struct sdricoh_host *host = mmc_priv(mmc);
    struct mmc_command *cmd = mrq.cmd;
    struct mmc_data *data = cmd.data;
    struct device *dev = host.dev;
    int i;
    dev_dbg(dev, "=============================\n");
    dev_dbg(dev, "sdricoh_request opcode=%i\n", cmd.opcode);
    sdricoh_writel(host, R21C_STATUS, 0x18);
// read/write commands seem to require this
    if (data) {
    sdricoh_writew(host, R226_BLOCKSIZE, data.blksz);
    sdricoh_writel(host, R208_DATAIO, 0);
    }
    cmd.error = sdricoh_mmc_cmd(host, cmd);
// read response buffer
    if (cmd.flags & MMC_RSP_PRESENT) {
    if (cmd.flags & MMC_RSP_136) {
// CRC is stripped so we need to do some shifting.
    for (i = 0; i < 4; i++) {
    cmd.resp[i] =
    sdricoh_readl(host,
    R20C_RESP + (3 - i) * 4) << 8;
    if (i != 3)
    cmd.resp[i] |=
    sdricoh_readb(host, R20C_RESP +
    (3 - i) * 4 - 1);
    }
    } else
    cmd.resp[0] = sdricoh_readl(host, R20C_RESP);
    }
// transfer data
    if (data && cmd.error == 0) {
    dev_dbg(dev, "transfer: blksz %i blocks %i sg_len %i "
    "sg length %i\n", data.blksz, data.blocks,
    data.sg_len, data.sg.length);
// enter data reading mode
    sdricoh_writel(host, R21C_STATUS, 0x837f031e);
    for (i = 0; i < data.blocks; i++) {
    let mut len: usize = data.blksz;
    u8 *buf;
    struct page *page;
    int result;
    page = sg_page(data.sg);
    buf = kmap(page) + data.sg.offset + (len * i);
    result =
    sdricoh_blockio(host,
    data.flags & MMC_DATA_READ, buf, len);
    kunmap(page);
    flush_dcache_page(page);
    if (result) {
    dev_err(dev, "sdricoh_request: cmd %i "
    "block transfer failed\n", cmd.opcode);
    cmd.error = result;
    break;
    } else
    data.bytes_xfered += len;
    }
    sdricoh_writel(host, R208_DATAIO, 1);
    if (sdricoh_query_status(host, STATUS_TRANSFER_FINISHED)) {
    dev_err(dev, "sdricoh_request: transfer end error\n");
    cmd.error = -EINVAL;
    }
    }
// FIXME check busy flag
    mmc_request_done(mmc, mrq);
    dev_dbg(dev, "=============================\n");
    }
#[no_mangle]
unsafe extern "C" fn sdricoh_set_ios(mmc: *mut mmc_host, ios: *mut mmc_ios) {
    static void sdricoh_set_ios(struct mmc_host *mmc, struct mmc_ios *ios)
    {
    struct sdricoh_host *host = mmc_priv(mmc);
    dev_dbg(host.dev, "set_ios\n");
    if (ios.power_mode == MMC_POWER_ON) {
    sdricoh_writel(host, R228_POWER, 0xc0e0);
    if (ios.bus_width == MMC_BUS_WIDTH_4) {
    sdricoh_writel(host, R224_MODE, 0x2000300);
    sdricoh_writel(host, R228_POWER, 0x40e0);
    } else {
    sdricoh_writel(host, R224_MODE, 0x2000340);
    }
    } else if (ios.power_mode == MMC_POWER_UP) {
    sdricoh_writel(host, R224_MODE, 0x2000320);
    sdricoh_writel(host, R228_POWER, 0xe0);
    }
    }
#[no_mangle]
unsafe extern "C" fn sdricoh_get_ro(mmc: *mut mmc_host) -> c_int {
    static int sdricoh_get_ro(struct mmc_host *mmc)
    {
    struct sdricoh_host *host = mmc_priv(mmc);
    unsigned int status;
    status = sdricoh_readl(host, R21C_STATUS);
    sdricoh_writel(host, R2E4_STATUS_RESP, status);
// some notebooks seem to have the locked flag switched
    if (switchlocked)
    return !(status & STATUS_CARD_LOCKED);
    return (status & STATUS_CARD_LOCKED);
    }
    static const struct mmc_host_ops sdricoh_ops = {
    .request = sdricoh_request,
    .set_ios = sdricoh_set_ios,
    .get_ro = sdricoh_get_ro,
    };
// initialize the control and register it to the mmc framework
    static int sdricoh_init_mmc(struct pci_dev *pci_dev,
    struct pcmcia_device *pcmcia_dev)
    {
    int result;
    void __iomem *iobase;
    struct mmc_host *mmc;
    struct sdricoh_host *host;
    struct device *dev = &pcmcia_dev.dev;
// map iomem
    if (pci_resource_len(pci_dev, SDRICOH_PCI_REGION) !=
    SDRICOH_PCI_REGION_SIZE) {
    dev_dbg(dev, "unexpected pci resource len\n");
    return -ENODEV;
    }
    iobase =
    pci_iomap(pci_dev, SDRICOH_PCI_REGION, SDRICOH_PCI_REGION_SIZE);
    if (!iobase) {
    dev_err(dev, "unable to map iobase\n");
    return -ENODEV;
    }
// check version?
    if (readl(iobase + R104_VERSION) != 0x4000) {
    dev_dbg(dev, "no supported mmc controller found\n");
    result = -ENODEV;
    goto unmap_io;
    }
// allocate privdata
    mmc = pcmcia_dev.priv =
    devm_mmc_alloc_host(&pcmcia_dev.dev, sizeof(*host));
    if (!mmc) {
    dev_err(dev, "devm_mmc_alloc_host failed\n");
    result = -ENOMEM;
    goto unmap_io;
    }
    host = mmc_priv(mmc);
    host.iobase = iobase;
    host.dev = dev;
    host.pci_dev = pci_dev;
    mmc.ops = &sdricoh_ops;
// FIXME: frequency and voltage handling is done by the controller
//
    mmc.f_min = 450000;
    mmc.f_max = 24000000;
    mmc.ocr_avail = MMC_VDD_32_33 | MMC_VDD_33_34;
    mmc.caps |= MMC_CAP_4_BIT_DATA;
    mmc.max_seg_size = 1024 * 512;
    mmc.max_blk_size = 512;
// reset the controller
    if (sdricoh_reset(host)) {
    dev_dbg(dev, "could not reset\n");
    result = -EIO;
    goto unmap_io;
    }
    result = mmc_add_host(mmc);
    if (!result) {
    dev_dbg(dev, "mmc host registered\n");
    return 0;
    }
    unmap_io:
    pci_iounmap(pci_dev, iobase);
    return result;
    }
// search for supported mmc controllers
#[no_mangle]
unsafe extern "C" fn sdricoh_pcmcia_probe(pcmcia_dev: *mut pcmcia_device) -> c_int {
    static int sdricoh_pcmcia_probe(struct pcmcia_device *pcmcia_dev)
    {
    struct pci_dev *pci_dev = core::ptr::null_mut();
    dev_info(&pcmcia_dev.dev, "Searching MMC controller for pcmcia device"
    " %s %s ...\n", pcmcia_dev.prod_id[0], pcmcia_dev.prod_id[1]);
// search pci cardbus bridge that contains the mmc controller
// the io region is already claimed by yenta_socket...
    while ((pci_dev =
    pci_get_device(PCI_VENDOR_ID_RICOH, PCI_DEVICE_ID_RICOH_RL5C476,
    pci_dev))) {
// try to init the device
    if (!sdricoh_init_mmc(pci_dev, pcmcia_dev)) {
    dev_info(&pcmcia_dev.dev, "MMC controller found\n");
    return 0;
    }
    }
    dev_err(&pcmcia_dev.dev, "No MMC controller was found.\n");
    return -ENODEV;
    }
#[no_mangle]
unsafe extern "C" fn sdricoh_pcmcia_detach(link: *mut pcmcia_device) {
    static void sdricoh_pcmcia_detach(struct pcmcia_device *link)
    {
    struct mmc_host *mmc = link.priv;
    dev_dbg(&link.dev, "detach\n");
// remove mmc host
    if (mmc) {
    struct sdricoh_host *host = mmc_priv(mmc);
    mmc_remove_host(mmc);
    pci_iounmap(host.pci_dev, host.iobase);
    pci_dev_put(host.pci_dev);
    }
    pcmcia_disable_device(link);
    }

#[no_mangle]
unsafe extern "C" fn sdricoh_pcmcia_suspend(link: *mut pcmcia_device) -> c_int {
    static int sdricoh_pcmcia_suspend(struct pcmcia_device *link)
    {
    dev_dbg(&link.dev, "suspend\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sdricoh_pcmcia_resume(link: *mut pcmcia_device) -> c_int {
    static int sdricoh_pcmcia_resume(struct pcmcia_device *link)
    {
    struct mmc_host *mmc = link.priv;
    dev_dbg(&link.dev, "resume\n");
    sdricoh_reset(mmc_priv(mmc));
    return 0;
    }

    static struct pcmcia_driver sdricoh_driver = {
    .name = DRIVER_NAME,
    .probe = sdricoh_pcmcia_probe,
    .remove = sdricoh_pcmcia_detach,
    .id_table = pcmcia_ids,
    .suspend = sdricoh_pcmcia_suspend,
    .resume = sdricoh_pcmcia_resume,
    };
    module_pcmcia_driver(sdricoh_driver);
    module_param(switchlocked, uint, 0444);
    MODULE_AUTHOR("Sascha Sommer <saschasommer@freenet.de>");
    MODULE_DESCRIPTION("Ricoh PCMCIA Secure Digital Interface driver");
    MODULE_LICENSE("GPL");
    MODULE_PARM_DESC(switchlocked, "Switch the cards locked status."
    "Use this when unlocked cards are shown readonly (default 0)");
