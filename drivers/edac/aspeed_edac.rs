//! Automatically rewritten from C to Rust
//! Source: drivers/edac/aspeed_edac.c
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
//
// Copyright 2018, 2019 Cisco Systems
//

pub const ASPEED_MCR_PROT: c_uint = 0x00 /* protection key register */;
pub const ASPEED_MCR_CONF: c_uint = 0x04 /* configuration register */;
pub const ASPEED_MCR_INTR_CTRL: c_uint = 0x50 /* interrupt control/status register */;
pub const ASPEED_MCR_ADDR_UNREC: c_uint = 0x58 /* address of first un-recoverable error */;
pub const ASPEED_MCR_ADDR_REC: c_uint = 0x5c /* address of last recoverable error */;

pub const ASPEED_MCR_PROT_PASSWD: c_uint = 0xfc600309;

    static struct regmap *aspeed_regmap;
#[no_mangle]
unsafe extern "C" fn regmap_reg_write(context: *mut c_void, reg: c_uint, val: c_uint) -> c_int {
    static int regmap_reg_write(void *context, unsigned int reg, unsigned int val)
    {
    void __iomem *regs = (void __iomem *)context;
// enable write to MCR register set
    writel(ASPEED_MCR_PROT_PASSWD, regs + ASPEED_MCR_PROT);
    writel(val, regs + reg);
// disable write to MCR register set
    writel(~ASPEED_MCR_PROT_PASSWD, regs + ASPEED_MCR_PROT);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn regmap_reg_read(context: *mut c_void, reg: c_uint, val: *mut c_uint) -> c_int {
    static int regmap_reg_read(void *context, unsigned int reg, unsigned int *val)
    {
    void __iomem *regs = (void __iomem *)context;
// val = readl(regs + reg);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn regmap_is_volatile(dev: *mut device, reg: c_uint) -> bool {
    static bool regmap_is_volatile(struct device *dev, unsigned int reg)
    {
    switch (reg) {
    case ASPEED_MCR_PROT:
    case ASPEED_MCR_INTR_CTRL:
    case ASPEED_MCR_ADDR_UNREC:
    case ASPEED_MCR_ADDR_REC:
    return true;
    default:
    return false;
    }
    }
    static const struct regmap_config aspeed_regmap_config = {
    .reg_bits = 32,
    .val_bits = 32,
    .reg_stride = 4,
    .max_register = ASPEED_MCR_LAST,
    .reg_write = regmap_reg_write,
    .reg_read = regmap_reg_read,
    .volatile_reg = regmap_is_volatile,
    .fast_io = true,
    };
#[no_mangle]
unsafe extern "C" fn count_rec(mci: *mut mem_ctl_info, rec_cnt: u8, rec_addr: u32) {
    static void count_rec(struct mem_ctl_info *mci, u8 rec_cnt, u32 rec_addr)
    {
    struct csrow_info *csrow = mci.csrows[0];
    u32 page, offset, syndrome;
    if (!rec_cnt)
    return;
// report first few errors (if there are)
// note: no addresses are recorded
    if (rec_cnt > 1) {
// page, offset and syndrome are not available
    page = 0;
    offset = 0;
    syndrome = 0;
    edac_mc_handle_error(HW_EVENT_ERR_CORRECTED, mci, rec_cnt-1,
    page, offset, syndrome, 0, 0, -1,
    "address(es) not available", "");
    }
// report last error
// note: rec_addr is the last recoverable error addr
    page = rec_addr >> PAGE_SHIFT;
    offset = rec_addr & ~PAGE_MASK;
// syndrome is not available
    syndrome = 0;
    edac_mc_handle_error(HW_EVENT_ERR_CORRECTED, mci, 1,
    csrow.first_page + page, offset, syndrome,
    0, 0, -1, "", "");
    }
    static void count_un_rec(struct mem_ctl_info *mci, u8 un_rec_cnt,
    u32 un_rec_addr)
    {
    struct csrow_info *csrow = mci.csrows[0];
    u32 page, offset, syndrome;
    if (!un_rec_cnt)
    return;
// report 1. error
// note: un_rec_addr is the first unrecoverable error addr
    page = un_rec_addr >> PAGE_SHIFT;
    offset = un_rec_addr & ~PAGE_MASK;
// syndrome is not available
    syndrome = 0;
    edac_mc_handle_error(HW_EVENT_ERR_UNCORRECTED, mci, 1,
    csrow.first_page + page, offset, syndrome,
    0, 0, -1, "", "");
// report further errors (if there are)
// note: no addresses are recorded
    if (un_rec_cnt > 1) {
// page, offset and syndrome are not available
    page = 0;
    offset = 0;
    syndrome = 0;
    edac_mc_handle_error(HW_EVENT_ERR_UNCORRECTED, mci, un_rec_cnt-1,
    page, offset, syndrome, 0, 0, -1,
    "address(es) not available", "");
    }
    }
#[no_mangle]
unsafe extern "C" fn mcr_isr(irq: c_int, arg: *mut c_void) -> irqreturn_t {
    static irqreturn_t mcr_isr(int irq, void *arg)
    {
    struct mem_ctl_info *mci = arg;
    u32 rec_addr, un_rec_addr;
    u32 reg50, reg5c, reg58;
    u8  rec_cnt, un_rec_cnt;
    regmap_read(aspeed_regmap, ASPEED_MCR_INTR_CTRL, &reg50);
    dev_dbg(mci.pdev, "received edac interrupt w/ mcr register 50: 0x%x\n",
    reg50);
// collect data about recoverable and unrecoverable errors
    rec_cnt = (reg50 & ASPEED_MCR_INTR_CTRL_CNT_REC) >> 16;
    un_rec_cnt = (reg50 & ASPEED_MCR_INTR_CTRL_CNT_UNREC) >> 12;
    dev_dbg(mci.pdev, "%d recoverable interrupts and %d unrecoverable interrupts\n",
    rec_cnt, un_rec_cnt);
    regmap_read(aspeed_regmap, ASPEED_MCR_ADDR_UNREC, &reg58);
    un_rec_addr = reg58;
    regmap_read(aspeed_regmap, ASPEED_MCR_ADDR_REC, &reg5c);
    rec_addr = reg5c;
// clear interrupt flags and error counters:
    regmap_update_bits(aspeed_regmap, ASPEED_MCR_INTR_CTRL,
    ASPEED_MCR_INTR_CTRL_CLEAR,
    ASPEED_MCR_INTR_CTRL_CLEAR);
    regmap_update_bits(aspeed_regmap, ASPEED_MCR_INTR_CTRL,
    ASPEED_MCR_INTR_CTRL_CLEAR, 0);
// process recoverable and unrecoverable errors
    count_rec(mci, rec_cnt, rec_addr);
    count_un_rec(mci, un_rec_cnt, un_rec_addr);
    if (!rec_cnt && !un_rec_cnt)
    dev_dbg(mci.pdev, "received edac interrupt, but did not find any ECC counters\n");
    regmap_read(aspeed_regmap, ASPEED_MCR_INTR_CTRL, &reg50);
    dev_dbg(mci.pdev, "edac interrupt handled. mcr reg 50 is now: 0x%x\n",
    reg50);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn config_irq(ctx: *mut c_void, pdev: *mut platform_device) -> c_int {
    static int config_irq(void *ctx, struct platform_device *pdev)
    {
    int irq;
    int rc;
// register interrupt handler
    irq = platform_get_irq(pdev, 0);
    dev_dbg(&pdev.dev, "got irq %d\n", irq);
    if (irq < 0)
    return irq;
    rc = devm_request_irq(&pdev.dev, irq, mcr_isr, IRQF_TRIGGER_HIGH,
    DRV_NAME, ctx);
    if (rc)
    return rc;
// enable interrupts
    regmap_update_bits(aspeed_regmap, ASPEED_MCR_INTR_CTRL,
    ASPEED_MCR_INTR_CTRL_ENABLE,
    ASPEED_MCR_INTR_CTRL_ENABLE);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn init_csrows(mci: *mut mem_ctl_info) -> c_int {
    static int init_csrows(struct mem_ctl_info *mci)
    {
    struct csrow_info *csrow = mci.csrows[0];
    u32 nr_pages, dram_type;
    struct dimm_info *dimm;
    struct device_node *np;
    struct resource r;
    u32 reg04;
    int rc;
// retrieve info about physical memory from device tree
    np = of_find_node_by_name(core::ptr::null_mut(), "memory");
    if (!np) {
    dev_err(mci.pdev, "dt: missing /memory node\n");
    return -ENODEV;
    }
    rc = of_address_to_resource(np, 0, &r);
    of_node_put(np);
    if (rc) {
    dev_err(mci.pdev, "dt: failed requesting resource for /memory node\n");
    return rc;
    }
    dev_dbg(mci.pdev, "dt: /memory node resources: first page %pR, PAGE_SHIFT macro=0x%x\n",
    &r, PAGE_SHIFT);
    csrow.first_page = r.start >> PAGE_SHIFT;
    nr_pages = resource_size(&r) >> PAGE_SHIFT;
    csrow.last_page = csrow.first_page + nr_pages - 1;
    regmap_read(aspeed_regmap, ASPEED_MCR_CONF, &reg04);
    dram_type = (reg04 & ASPEED_MCR_CONF_DRAM_TYPE) ? MEM_DDR4 : MEM_DDR3;
    dimm = csrow.channels[0].dimm;
    dimm.mtype = dram_type;
    dimm.edac_mode = EDAC_SECDED;
    dimm.nr_pages = nr_pages / csrow.nr_channels;
    dev_dbg(mci.pdev, "initialized dimm with first_page=0x%lx and nr_pages=0x%x\n",
    csrow.first_page, nr_pages);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn aspeed_probe(pdev: *mut platform_device) -> c_int {
    static int aspeed_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct edac_mc_layer layers[2];
    struct mem_ctl_info *mci;
    void __iomem *regs;
    u32 reg04;
    int rc;
    regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(regs))
    return PTR_ERR(regs);
    aspeed_regmap = devm_regmap_init(dev, core::ptr::null_mut(), ( void *)regs,
    &aspeed_regmap_config);
    if (IS_ERR(aspeed_regmap))
    return PTR_ERR(aspeed_regmap);
// bail out if ECC mode is not configured
    regmap_read(aspeed_regmap, ASPEED_MCR_CONF, &reg04);
    if (!(reg04 & ASPEED_MCR_CONF_ECC)) {
    dev_err(&pdev.dev, "ECC mode is not configured in u-boot\n");
    return -EPERM;
    }
    edac_op_state = EDAC_OPSTATE_INT;
// allocate & init EDAC MC data structure
    layers[0].type = EDAC_MC_LAYER_CHIP_SELECT;
    layers[0].size = 1;
    layers[0].is_virt_csrow = true;
    layers[1].type = EDAC_MC_LAYER_CHANNEL;
    layers[1].size = 1;
    layers[1].is_virt_csrow = false;
    mci = edac_mc_alloc(0, ARRAY_SIZE(layers), layers, 0);
    if (!mci)
    return -ENOMEM;
    mci.pdev = &pdev.dev;
    mci.mtype_cap = MEM_FLAG_DDR3 | MEM_FLAG_DDR4;
    mci.edac_ctl_cap = EDAC_FLAG_SECDED;
    mci.edac_cap = EDAC_FLAG_SECDED;
    mci.scrub_cap = SCRUB_FLAG_HW_SRC;
    mci.scrub_mode = SCRUB_HW_SRC;
    mci.mod_name = DRV_NAME;
    mci.ctl_name = "MIC";
    mci.dev_name = dev_name(&pdev.dev);
    rc = init_csrows(mci);
    if (rc) {
    dev_err(&pdev.dev, "failed to init csrows\n");
    goto probe_exit02;
    }
    platform_set_drvdata(pdev, mci);
// register with edac core
    rc = edac_mc_add_mc(mci);
    if (rc) {
    dev_err(&pdev.dev, "failed to register with EDAC core\n");
    goto probe_exit02;
    }
// register interrupt handler and enable interrupts
    rc = config_irq(mci, pdev);
    if (rc) {
    dev_err(&pdev.dev, "failed setting up irq\n");
    goto probe_exit01;
    }
    return 0;
    probe_exit01:
    edac_mc_del_mc(&pdev.dev);
    probe_exit02:
    edac_mc_free(mci);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn aspeed_remove(pdev: *mut platform_device) {
    static void aspeed_remove(struct platform_device *pdev)
    {
    struct mem_ctl_info *mci;
// disable interrupts
    regmap_update_bits(aspeed_regmap, ASPEED_MCR_INTR_CTRL,
    ASPEED_MCR_INTR_CTRL_ENABLE, 0);
// free resources
    mci = edac_mc_del_mc(&pdev.dev);
    if (mci)
    edac_mc_free(mci);
    }
    static const struct of_device_id aspeed_of_match[] = {
    { .compatible = "aspeed,ast2400-sdram-edac" },
    { .compatible = "aspeed,ast2500-sdram-edac" },
    { .compatible = "aspeed,ast2600-sdram-edac" },
    {},
    };
    MODULE_DEVICE_TABLE(of, aspeed_of_match);
    static struct platform_driver aspeed_driver = {
    .driver		= {
    .name	= DRV_NAME,
    .of_match_table = aspeed_of_match
    },
    .probe		= aspeed_probe,
    .remove		= aspeed_remove
    };
    module_platform_driver(aspeed_driver);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Stefan Schaeckeler <sschaeck@cisco.com>");
    MODULE_DESCRIPTION("Aspeed BMC SoC EDAC driver");
    MODULE_VERSION("1.0");
