//! Automatically rewritten from C to Rust
//! Source: drivers/net/mdio/mdio-mux-bcm-iproc.c
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
// Copyright 2016 Broadcom
//

pub const MDIO_RATE_ADJ_EXT_OFFSET: c_uint = 0x000;
pub const MDIO_RATE_ADJ_INT_OFFSET: c_uint = 0x004;
pub const MDIO_RATE_ADJ_DIVIDENT_SHIFT: c_int = 16;
pub const MDIO_SCAN_CTRL_OFFSET: c_uint = 0x008;
pub const MDIO_SCAN_CTRL_OVRIDE_EXT_MSTR: c_int = 28;
pub const MDIO_PARAM_OFFSET: c_uint = 0x23c;
pub const MDIO_PARAM_MIIM_CYCLE: c_int = 29;
pub const MDIO_PARAM_INTERNAL_SEL: c_int = 25;
pub const MDIO_PARAM_BUS_ID: c_int = 22;
pub const MDIO_PARAM_C45_SEL: c_int = 21;
pub const MDIO_PARAM_PHY_ID: c_int = 16;
pub const MDIO_PARAM_PHY_DATA: c_int = 0;
pub const MDIO_READ_OFFSET: c_uint = 0x240;
pub const MDIO_READ_DATA_MASK: c_uint = 0xffff;
pub const MDIO_ADDR_OFFSET: c_uint = 0x244;
pub const MDIO_CTRL_OFFSET: c_uint = 0x248;
pub const MDIO_CTRL_WRITE_OP: c_uint = 0x1;
pub const MDIO_CTRL_READ_OP: c_uint = 0x2;
pub const MDIO_STAT_OFFSET: c_uint = 0x24c;
pub const MDIO_STAT_DONE: c_int = 1;
pub const BUS_MAX_ADDR: c_int = 32;
pub const EXT_BUS_START_ADDR: c_int = 16;
pub const MDIO_REG_ADDR_SPACE_SIZE: c_uint = 0x250;
pub const MDIO_OPERATING_FREQUENCY: c_int = 11000000;
pub const MDIO_RATE_ADJ_DIVIDENT: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iproc_mdiomux_desc {
    pub mux_handle: *mut c_void,
    pub base: *mut void __iomem,
    pub dev: *mut device,
    pub mii_bus: *mut mii_bus,
    pub core_clk: *mut clk,
}

#[no_mangle]
unsafe extern "C" fn mdio_mux_iproc_config(md: *mut iproc_mdiomux_desc) {
    static void mdio_mux_iproc_config(struct iproc_mdiomux_desc *md)
    {
    u32 divisor;
    u32 val;
// Disable external mdio master access
    val = readl(md.base + MDIO_SCAN_CTRL_OFFSET);
    val |= BIT(MDIO_SCAN_CTRL_OVRIDE_EXT_MSTR);
    writel(val, md.base + MDIO_SCAN_CTRL_OFFSET);
    if (md.core_clk) {
// use rate adjust regs to derive the mdio's operating
// frequency from the specified core clock
//
    divisor = clk_get_rate(md.core_clk) / MDIO_OPERATING_FREQUENCY;
    divisor = divisor / (MDIO_RATE_ADJ_DIVIDENT + 1);
    val = divisor;
    val |= MDIO_RATE_ADJ_DIVIDENT << MDIO_RATE_ADJ_DIVIDENT_SHIFT;
    writel(val, md.base + MDIO_RATE_ADJ_EXT_OFFSET);
    writel(val, md.base + MDIO_RATE_ADJ_INT_OFFSET);
    }
    }
#[no_mangle]
unsafe extern "C" fn iproc_mdio_wait_for_idle(base: *mut void __iomem, result: bool) -> c_int {
    static int iproc_mdio_wait_for_idle(void __iomem *base, bool result)
    {
    u32 val;
    return readl_poll_timeout(base + MDIO_STAT_OFFSET, val,
    (val & MDIO_STAT_DONE) == result,
    2000, 1000000);
    }
// start_miim_ops- Program and start MDIO transaction over mdio bus.
// @base: Base address
// @phyid: phyid of the selected bus.
// @reg: register offset to be read/written.
// @val :0 if read op else value to be written in @reg;
// @op: Operation that need to be carried out.
// MDIO_CTRL_READ_OP: Read transaction.
// MDIO_CTRL_WRITE_OP: Write transaction.
//
// Return value: Successful Read operation returns read reg values and write
// operation returns 0. Failure operation returns negative error code.
//
    static int start_miim_ops(void __iomem *base, bool c45,
    u16 phyid, u32 reg, u16 val, u32 op)
    {
    u32 param;
    int ret;
    writel(0, base + MDIO_CTRL_OFFSET);
    ret = iproc_mdio_wait_for_idle(base, 0);
    if (ret)
    goto err;
    param = readl(base + MDIO_PARAM_OFFSET);
    param |= phyid << MDIO_PARAM_PHY_ID;
    param |= val << MDIO_PARAM_PHY_DATA;
    if (c45)
    param |= BIT(MDIO_PARAM_C45_SEL);
    writel(param, base + MDIO_PARAM_OFFSET);
    writel(reg, base + MDIO_ADDR_OFFSET);
    writel(op, base + MDIO_CTRL_OFFSET);
    ret = iproc_mdio_wait_for_idle(base, 1);
    if (ret)
    goto err;
    if (op == MDIO_CTRL_READ_OP)
    ret = readl(base + MDIO_READ_OFFSET) & MDIO_READ_DATA_MASK;
    err:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn iproc_mdiomux_read_c22(bus: *mut mii_bus, phyid: c_int, reg: c_int) -> c_int {
    static int iproc_mdiomux_read_c22(struct mii_bus *bus, int phyid, int reg)
    {
    struct iproc_mdiomux_desc *md = bus.priv;
    int ret;
    ret = start_miim_ops(md.base, false, phyid, reg, 0, MDIO_CTRL_READ_OP);
    if (ret < 0)
    dev_err(&bus.dev, "mdiomux c22 read operation failed!!!");
    return ret;
    }
    static int iproc_mdiomux_read_c45(struct mii_bus *bus, int phyid, int devad,
    int reg)
    {
    struct iproc_mdiomux_desc *md = bus.priv;
    int ret;
    ret = start_miim_ops(md.base, true, phyid, reg | devad << 16, 0,
    MDIO_CTRL_READ_OP);
    if (ret < 0)
    dev_err(&bus.dev, "mdiomux read c45 operation failed!!!");
    return ret;
    }
    static int iproc_mdiomux_write_c22(struct mii_bus *bus,
    int phyid, int reg, u16 val)
    {
    struct iproc_mdiomux_desc *md = bus.priv;
    int ret;
// Write val at reg offset
    ret = start_miim_ops(md.base, false, phyid, reg, val,
    MDIO_CTRL_WRITE_OP);
    if (ret < 0)
    dev_err(&bus.dev, "mdiomux write c22 operation failed!!!");
    return ret;
    }
    static int iproc_mdiomux_write_c45(struct mii_bus *bus,
    int phyid, int devad, int reg, u16 val)
    {
    struct iproc_mdiomux_desc *md = bus.priv;
    int ret;
// Write val at reg offset
    ret = start_miim_ops(md.base, true, phyid, reg | devad << 16, val,
    MDIO_CTRL_WRITE_OP);
    if (ret < 0)
    dev_err(&bus.dev, "mdiomux write c45 operation failed!!!");
    return ret;
    }
    static int mdio_mux_iproc_switch_fn(int current_child, int desired_child,
    void *data)
    {
    struct iproc_mdiomux_desc *md = data;
    u32 param, bus_id;
    bool bus_dir;
// select bus and its properties
    bus_dir = (desired_child < EXT_BUS_START_ADDR);
    bus_id = bus_dir ? desired_child : (desired_child - EXT_BUS_START_ADDR);
    param = (bus_dir ? 1 : 0) << MDIO_PARAM_INTERNAL_SEL;
    param |= (bus_id << MDIO_PARAM_BUS_ID);
    writel(param, md.base + MDIO_PARAM_OFFSET);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mdio_mux_iproc_probe(pdev: *mut platform_device) -> c_int {
    static int mdio_mux_iproc_probe(struct platform_device *pdev)
    {
    struct iproc_mdiomux_desc *md;
    struct mii_bus *bus;
    struct resource *res;
    int rc;
    md = devm_kzalloc(&pdev.dev, sizeof(*md), GFP_KERNEL);
    if (!md)
    return -ENOMEM;
    md.dev = &pdev.dev;
    md.base = devm_platform_get_and_ioremap_resource(pdev, 0, &res);
    if (IS_ERR(md.base))
    return PTR_ERR(md.base);
    if (!IS_ALIGNED(res.start, SZ_4K)) {
// For backward compatibility in case the
// base address is specified with an offset.
//
    dev_info(&pdev.dev, "fix base address in dt-blob\n");
    res.start = ALIGN_DOWN(res.start, SZ_4K);
    res.end = res.start + MDIO_REG_ADDR_SPACE_SIZE - 1;
    }
    md.mii_bus = devm_mdiobus_alloc(&pdev.dev);
    if (!md.mii_bus) {
    dev_err(&pdev.dev, "mdiomux bus alloc failed\n");
    return -ENOMEM;
    }
    md.core_clk = devm_clk_get(&pdev.dev, core::ptr::null_mut());
    if (md.core_clk == ERR_PTR(-ENOENT) ||
    md.core_clk == ERR_PTR(-EINVAL))
    md.core_clk = core::ptr::null_mut();
#[no_mangle]
pub unsafe extern "C" fn if(_arg: IS_ERR(md->core_clk)) -> else {
    else if (IS_ERR(md.core_clk))
    return PTR_ERR(md.core_clk);
    rc = clk_prepare_enable(md.core_clk);
    if (rc) {
    dev_err(&pdev.dev, "failed to enable core clk\n");
    return rc;
    }
    bus = md.mii_bus;
    bus.priv = md;
    bus.name = "iProc MDIO mux bus";
    snprintf(bus.id, MII_BUS_ID_SIZE, "%s-%d", pdev.name, pdev.id);
    bus.parent = &pdev.dev;
    bus.read = iproc_mdiomux_read_c22;
    bus.write = iproc_mdiomux_write_c22;
    bus.read_c45 = iproc_mdiomux_read_c45;
    bus.write_c45 = iproc_mdiomux_write_c45;
    bus.phy_mask = ~0;
    bus.dev.of_node = pdev.dev.of_node;
    rc = mdiobus_register(bus);
    if (rc) {
    dev_err(&pdev.dev, "mdiomux registration failed\n");
    goto out_clk;
    }
    platform_set_drvdata(pdev, md);
    rc = mdio_mux_init(md.dev, md.dev.of_node, mdio_mux_iproc_switch_fn,
    &md.mux_handle, md, md.mii_bus);
    if (rc) {
    dev_info(md.dev, "mdiomux initialization failed\n");
    goto out_register;
    }
    mdio_mux_iproc_config(md);
    dev_info(md.dev, "iProc mdiomux registered\n");
    return 0;
    out_register:
    mdiobus_unregister(bus);
    out_clk:
    clk_disable_unprepare(md.core_clk);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn mdio_mux_iproc_remove(pdev: *mut platform_device) {
    static void mdio_mux_iproc_remove(struct platform_device *pdev)
    {
    struct iproc_mdiomux_desc *md = platform_get_drvdata(pdev);
    mdio_mux_uninit(md.mux_handle);
    mdiobus_unregister(md.mii_bus);
    clk_disable_unprepare(md.core_clk);
    }

#[no_mangle]
unsafe extern "C" fn mdio_mux_iproc_suspend(dev: *mut device) -> c_int {
    static int mdio_mux_iproc_suspend(struct device *dev)
    {
    struct iproc_mdiomux_desc *md = dev_get_drvdata(dev);
    clk_disable_unprepare(md.core_clk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mdio_mux_iproc_resume(dev: *mut device) -> c_int {
    static int mdio_mux_iproc_resume(struct device *dev)
    {
    struct iproc_mdiomux_desc *md = dev_get_drvdata(dev);
    int rc;
    rc = clk_prepare_enable(md.core_clk);
    if (rc) {
    dev_err(md.dev, "failed to enable core clk\n");
    return rc;
    }
    mdio_mux_iproc_config(md);
    return 0;
    }

    static SIMPLE_DEV_PM_OPS(mdio_mux_iproc_pm_ops,
    mdio_mux_iproc_suspend, mdio_mux_iproc_resume);
    static const struct of_device_id mdio_mux_iproc_match[] = {
    {
    .compatible = "brcm,mdio-mux-iproc",
    },
    {},
    };
    MODULE_DEVICE_TABLE(of, mdio_mux_iproc_match);
    static struct platform_driver mdiomux_iproc_driver = {
    .driver = {
    .name		= "mdio-mux-iproc",
    .of_match_table = mdio_mux_iproc_match,
    .pm		= &mdio_mux_iproc_pm_ops,
    },
    .probe		= mdio_mux_iproc_probe,
    .remove		= mdio_mux_iproc_remove,
    };
    module_platform_driver(mdiomux_iproc_driver);
    MODULE_DESCRIPTION("iProc MDIO Mux Bus Driver");
    MODULE_AUTHOR("Pramod Kumar <pramod.kumar@broadcom.com>");
    MODULE_LICENSE("GPL v2");
