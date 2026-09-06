//! Automatically rewritten from C to Rust
//! Source: drivers/rapidio/switches/idt_gen2.c
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
// IDT CPS Gen.2 Serial RapidIO switch family support
//
// Copyright 2010 Integrated Device Technology, Inc.
// Alexandre Bounine <alexandre.bounine@idt.com>
//

pub const LOCAL_RTE_CONF_DESTID_SEL: c_uint = 0x010070;
pub const LOCAL_RTE_CONF_DESTID_SEL_PSEL: c_uint = 0x0000001f;
pub const IDT_LT_ERR_REPORT_EN: c_uint = 0x03100c;

pub const IDT_PORT_ERR_REPORT_EN_BC: c_uint = 0x03ff04;

pub const IDT_PORT_ISERR_REPORT_EN_BC: c_uint = 0x03ff0c;
pub const IDT_PORT_INIT_TX_ACQUIRED: c_uint = 0x00000020;

pub const IDT_LANE_ERR_REPORT_EN_BC: c_uint = 0x03ff10;
pub const IDT_DEV_CTRL_1: c_uint = 0xf2000c;
pub const IDT_DEV_CTRL_1_GENPW: c_uint = 0x02000000;
pub const IDT_DEV_CTRL_1_PRSTBEH: c_uint = 0x00000001;
pub const IDT_CFGBLK_ERR_CAPTURE_EN: c_uint = 0x020008;
pub const IDT_CFGBLK_ERR_REPORT: c_uint = 0xf20014;
pub const IDT_CFGBLK_ERR_REPORT_GENPW: c_uint = 0x00000002;
pub const IDT_AUX_PORT_ERR_CAP_EN: c_uint = 0x020000;
pub const IDT_AUX_ERR_REPORT_EN: c_uint = 0xf20018;
pub const IDT_AUX_PORT_ERR_LOG_I2C: c_uint = 0x00000002;
pub const IDT_AUX_PORT_ERR_LOG_JTAG: c_uint = 0x00000001;
pub const IDT_ISLTL_ADDRESS_CAP: c_uint = 0x021014;
pub const IDT_RIO_DOMAIN: c_uint = 0xf20020;
pub const IDT_RIO_DOMAIN_MASK: c_uint = 0x000000ff;
pub const IDT_PW_INFO_CSR: c_uint = 0xf20024;
pub const IDT_SOFT_RESET: c_uint = 0xf20040;
pub const IDT_SOFT_RESET_REQ: c_uint = 0x00030097;
pub const IDT_I2C_MCTRL: c_uint = 0xf20050;
pub const IDT_I2C_MCTRL_GENPW: c_uint = 0x04000000;
pub const IDT_JTAG_CTRL: c_uint = 0xf2005c;
pub const IDT_JTAG_CTRL_GENPW: c_uint = 0x00000002;

pub const IDT_LANE_CTRL_BC: c_uint = 0xffff00;
pub const IDT_LANE_CTRL_GENPW: c_uint = 0x00200000;
pub const IDT_LANE_DFE_1_BC: c_uint = 0xffff18;
pub const IDT_LANE_DFE_2_BC: c_uint = 0xffff1c;

pub const IDT_PORT_OPS_GENPW: c_uint = 0x08000000;
pub const IDT_PORT_OPS_PL_ELOG: c_uint = 0x00000040;
pub const IDT_PORT_OPS_LL_ELOG: c_uint = 0x00000020;
pub const IDT_PORT_OPS_LT_ELOG: c_uint = 0x00000010;
pub const IDT_PORT_OPS_BC: c_uint = 0xf4ff04;

pub const IDT_ERR_CAP: c_uint = 0xfd0000;
pub const IDT_ERR_CAP_LOG_OVERWR: c_uint = 0x00000004;
pub const IDT_ERR_RD: c_uint = 0xfd0004;
pub const IDT_DEFAULT_ROUTE: c_uint = 0xde;
pub const IDT_NO_ROUTE: c_uint = 0xdf;
    static int
    idtg2_route_add_entry(struct rio_mport *mport, u16 destid, u8 hopcount,
    u16 table, u16 route_destid, u8 route_port)
    {
//
// Select routing table to update
//
    if (table == RIO_GLOBAL_TABLE)
    table = 0;
    else
    table++;
    if (route_port == RIO_INVALID_ROUTE)
    route_port = IDT_DEFAULT_ROUTE;
    rio_mport_write_config_32(mport, destid, hopcount,
    LOCAL_RTE_CONF_DESTID_SEL, table);
//
// Program destination port for the specified destID
//
    rio_mport_write_config_32(mport, destid, hopcount,
    RIO_STD_RTE_CONF_DESTID_SEL_CSR,
    (u32)route_destid);
    rio_mport_write_config_32(mport, destid, hopcount,
    RIO_STD_RTE_CONF_PORT_SEL_CSR,
    (u32)route_port);
    udelay(10);
    return 0;
    }
    static int
    idtg2_route_get_entry(struct rio_mport *mport, u16 destid, u8 hopcount,
    u16 table, u16 route_destid, u8 *route_port)
    {
    u32 result;
//
// Select routing table to read
//
    if (table == RIO_GLOBAL_TABLE)
    table = 0;
    else
    table++;
    rio_mport_write_config_32(mport, destid, hopcount,
    LOCAL_RTE_CONF_DESTID_SEL, table);
    rio_mport_write_config_32(mport, destid, hopcount,
    RIO_STD_RTE_CONF_DESTID_SEL_CSR,
    route_destid);
    rio_mport_read_config_32(mport, destid, hopcount,
    RIO_STD_RTE_CONF_PORT_SEL_CSR, &result);
    if (IDT_DEFAULT_ROUTE == (u8)result || IDT_NO_ROUTE == (u8)result)
// route_port = RIO_INVALID_ROUTE;
    else
// route_port = (u8)result;
    return 0;
    }
    static int
    idtg2_route_clr_table(struct rio_mport *mport, u16 destid, u8 hopcount,
    u16 table)
    {
    u32 i;
//
// Select routing table to read
//
    if (table == RIO_GLOBAL_TABLE)
    table = 0;
    else
    table++;
    rio_mport_write_config_32(mport, destid, hopcount,
    LOCAL_RTE_CONF_DESTID_SEL, table);
    for (i = RIO_STD_RTE_CONF_EXTCFGEN;
    i <= (RIO_STD_RTE_CONF_EXTCFGEN | 0xff);) {
    rio_mport_write_config_32(mport, destid, hopcount,
    RIO_STD_RTE_CONF_DESTID_SEL_CSR, i);
    rio_mport_write_config_32(mport, destid, hopcount,
    RIO_STD_RTE_CONF_PORT_SEL_CSR,
    (IDT_DEFAULT_ROUTE << 24) | (IDT_DEFAULT_ROUTE << 16) |
    (IDT_DEFAULT_ROUTE << 8) | IDT_DEFAULT_ROUTE);
    i += 4;
    }
    return 0;
    }
    static int
    idtg2_set_domain(struct rio_mport *mport, u16 destid, u8 hopcount,
    u8 sw_domain)
    {
//
// Switch domain configuration operates only at global level
//
    rio_mport_write_config_32(mport, destid, hopcount,
    IDT_RIO_DOMAIN, (u32)sw_domain);
    return 0;
    }
    static int
    idtg2_get_domain(struct rio_mport *mport, u16 destid, u8 hopcount,
    u8 *sw_domain)
    {
    u32 regval;
//
// Switch domain configuration operates only at global level
//
    rio_mport_read_config_32(mport, destid, hopcount,
    IDT_RIO_DOMAIN, &regval);
// sw_domain = (u8)(regval & 0xff);
    return 0;
    }
    static int
    idtg2_em_init(struct rio_dev *rdev)
    {
    u32 regval;
    int i, tmp;
//
// This routine performs device-specific initialization only.
// All standard EM configuration should be performed at upper level.
//
    pr_debug("RIO: %s [%d:%d]\n", __func__, rdev.destid, rdev.hopcount);
// Set Port-Write info CSR: PRIO=3 and CRF=1
    rio_write_config_32(rdev, IDT_PW_INFO_CSR, 0x0000e000);
//
// Configure LT LAYER error reporting.
//
// Enable standard (RIO.p8) error reporting
    rio_write_config_32(rdev, IDT_LT_ERR_REPORT_EN,
    REM_LTL_ERR_ILLTRAN | REM_LTL_ERR_UNSOLR |
    REM_LTL_ERR_UNSUPTR);
// Use Port-Writes for LT layer error reporting.
// Enable per-port reset
//
    rio_read_config_32(rdev, IDT_DEV_CTRL_1, &regval);
    rio_write_config_32(rdev, IDT_DEV_CTRL_1,
    regval | IDT_DEV_CTRL_1_GENPW | IDT_DEV_CTRL_1_PRSTBEH);
//
// Configure PORT error reporting.
//
// Report all RIO.p8 errors supported by device
    rio_write_config_32(rdev, IDT_PORT_ERR_REPORT_EN_BC, 0x807e8037);
// Configure reporting of implementation specific errors/events
    rio_write_config_32(rdev, IDT_PORT_ISERR_REPORT_EN_BC,
    IDT_PORT_INIT_TX_ACQUIRED);
// Use Port-Writes for port error reporting and enable error logging
    tmp = RIO_GET_TOTAL_PORTS(rdev.swpinfo);
    for (i = 0; i < tmp; i++) {
    rio_read_config_32(rdev, IDT_PORT_OPS(i), &regval);
    rio_write_config_32(rdev,
    IDT_PORT_OPS(i), regval | IDT_PORT_OPS_GENPW |
    IDT_PORT_OPS_PL_ELOG |
    IDT_PORT_OPS_LL_ELOG |
    IDT_PORT_OPS_LT_ELOG);
    }
// Overwrite error log if full
    rio_write_config_32(rdev, IDT_ERR_CAP, IDT_ERR_CAP_LOG_OVERWR);
//
// Configure LANE error reporting.
//
// Disable line error reporting
    rio_write_config_32(rdev, IDT_LANE_ERR_REPORT_EN_BC, 0);
// Use Port-Writes for lane error reporting (when enabled)
// (do per-lane update because lanes may have different configuration)
//
    tmp = (rdev.did == RIO_DID_IDTCPS1848) ? 48 : 16;
    for (i = 0; i < tmp; i++) {
    rio_read_config_32(rdev, IDT_LANE_CTRL(i), &regval);
    rio_write_config_32(rdev, IDT_LANE_CTRL(i),
    regval | IDT_LANE_CTRL_GENPW);
    }
//
// Configure AUX error reporting.
//
// Disable JTAG and I2C Error capture
    rio_write_config_32(rdev, IDT_AUX_PORT_ERR_CAP_EN, 0);
// Disable JTAG and I2C Error reporting/logging
    rio_write_config_32(rdev, IDT_AUX_ERR_REPORT_EN, 0);
// Disable Port-Write notification from JTAG
    rio_write_config_32(rdev, IDT_JTAG_CTRL, 0);
// Disable Port-Write notification from I2C
    rio_read_config_32(rdev, IDT_I2C_MCTRL, &regval);
    rio_write_config_32(rdev, IDT_I2C_MCTRL, regval & ~IDT_I2C_MCTRL_GENPW);
//
// Configure CFG_BLK error reporting.
//
// Disable Configuration Block error capture
    rio_write_config_32(rdev, IDT_CFGBLK_ERR_CAPTURE_EN, 0);
// Disable Port-Writes for Configuration Block error reporting
    rio_read_config_32(rdev, IDT_CFGBLK_ERR_REPORT, &regval);
    rio_write_config_32(rdev, IDT_CFGBLK_ERR_REPORT,
    regval & ~IDT_CFGBLK_ERR_REPORT_GENPW);
// set TVAL = ~50us
    rio_write_config_32(rdev,
    rdev.phys_efptr + RIO_PORT_LINKTO_CTL_CSR, 0x8e << 8);
    return 0;
    }
    static int
    idtg2_em_handler(struct rio_dev *rdev, u8 portnum)
    {
    u32 regval, em_perrdet, em_ltlerrdet;
    rio_read_config_32(rdev,
    rdev.em_efptr + RIO_EM_LTL_ERR_DETECT, &em_ltlerrdet);
    if (em_ltlerrdet) {
// Service Logical/Transport Layer Error(s)
    if (em_ltlerrdet & REM_LTL_ERR_IMPSPEC) {
// Implementation specific error reported
    rio_read_config_32(rdev,
    IDT_ISLTL_ADDRESS_CAP, &regval);
    pr_debug("RIO: %s Implementation Specific LTL errors" \
    " 0x%x @(0x%x)\n",
    rio_name(rdev), em_ltlerrdet, regval);
// Clear implementation specific address capture CSR
    rio_write_config_32(rdev, IDT_ISLTL_ADDRESS_CAP, 0);
    }
    }
    rio_read_config_32(rdev,
    rdev.em_efptr + RIO_EM_PN_ERR_DETECT(portnum), &em_perrdet);
    if (em_perrdet) {
// Service Port-Level Error(s)
    if (em_perrdet & REM_PED_IMPL_SPEC) {
// Implementation Specific port error reported
// Get IS errors reported
    rio_read_config_32(rdev,
    IDT_PORT_ISERR_DET(portnum), &regval);
    pr_debug("RIO: %s Implementation Specific Port" \
    " errors 0x%x\n", rio_name(rdev), regval);
// Clear all implementation specific events
    rio_write_config_32(rdev,
    IDT_PORT_ISERR_DET(portnum), 0);
    }
    }
    return 0;
    }
    static ssize_t
    idtg2_show_errlog(struct device *dev, struct device_attribute *attr, char *buf)
    {
    struct rio_dev *rdev = to_rio_dev(dev);
    let mut len: isize = 0;
    u32 regval;
    while (!rio_read_config_32(rdev, IDT_ERR_RD, &regval)) {
    if (!regval)    /* 0 = end of log */
    break;
    len += snprintf(buf + len, PAGE_SIZE - len,
    "%08x\n", regval);
    if (len >= (PAGE_SIZE - 10))
    break;
    }
    return len;
    }
    static DEVICE_ATTR(errlog, S_IRUGO, idtg2_show_errlog, core::ptr::null_mut());
#[no_mangle]
unsafe extern "C" fn idtg2_sysfs(rdev: *mut rio_dev, create: bool) -> c_int {
    static int idtg2_sysfs(struct rio_dev *rdev, bool create)
    {
    struct device *dev = &rdev.dev;
    let mut err: c_int = 0;
    if (create) {
// Initialize sysfs entries
    err = device_create_file(dev, &dev_attr_errlog);
    if (err)
    dev_err(dev, "Unable create sysfs errlog file\n");
    } else
    device_remove_file(dev, &dev_attr_errlog);
    return err;
    }
    static struct rio_switch_ops idtg2_switch_ops = {
    .owner = THIS_MODULE,
    .add_entry = idtg2_route_add_entry,
    .get_entry = idtg2_route_get_entry,
    .clr_table = idtg2_route_clr_table,
    .set_domain = idtg2_set_domain,
    .get_domain = idtg2_get_domain,
    .em_init = idtg2_em_init,
    .em_handle = idtg2_em_handler,
    };
#[no_mangle]
unsafe extern "C" fn idtg2_probe(rdev: *mut rio_dev, id: *const rio_device_id) -> c_int {
    static int idtg2_probe(struct rio_dev *rdev, const struct rio_device_id *id)
    {
    pr_debug("RIO: %s for %s\n", __func__, rio_name(rdev));
    spin_lock(&rdev.rswitch.lock);
    if (rdev.rswitch.ops) {
    spin_unlock(&rdev.rswitch.lock);
    return -EINVAL;
    }
    rdev.rswitch.ops = &idtg2_switch_ops;
    if (rdev.do_enum) {
// Ensure that default routing is disabled on startup
    rio_write_config_32(rdev,
    RIO_STD_RTE_DEFAULT_PORT, IDT_NO_ROUTE);
    }
    spin_unlock(&rdev.rswitch.lock);
// Create device-specific sysfs attributes
    idtg2_sysfs(rdev, true);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn idtg2_remove(rdev: *mut rio_dev) {
    static void idtg2_remove(struct rio_dev *rdev)
    {
    pr_debug("RIO: %s for %s\n", __func__, rio_name(rdev));
    spin_lock(&rdev.rswitch.lock);
    if (rdev.rswitch.ops != &idtg2_switch_ops) {
    spin_unlock(&rdev.rswitch.lock);
    return;
    }
    rdev.rswitch.ops = core::ptr::null_mut();
    spin_unlock(&rdev.rswitch.lock);
// Remove device-specific sysfs attributes
    idtg2_sysfs(rdev, false);
    }
    static const struct rio_device_id idtg2_id_table[] = {
    {RIO_DEVICE(RIO_DID_IDTCPS1848, RIO_VID_IDT)},
    {RIO_DEVICE(RIO_DID_IDTCPS1616, RIO_VID_IDT)},
    {RIO_DEVICE(RIO_DID_IDTVPS1616, RIO_VID_IDT)},
    {RIO_DEVICE(RIO_DID_IDTSPS1616, RIO_VID_IDT)},
    {RIO_DEVICE(RIO_DID_IDTCPS1432, RIO_VID_IDT)},
    { 0, }	/* terminate list */
    };
    static struct rio_driver idtg2_driver = {
    .name = "idt_gen2",
    .id_table = idtg2_id_table,
    .probe = idtg2_probe,
    .remove = idtg2_remove,
    };
#[no_mangle]
unsafe extern "C" fn idtg2_init() -> int __init {
    static int __init idtg2_init(void)
    {
    return rio_register_driver(&idtg2_driver);
    }
#[no_mangle]
unsafe extern "C" fn idtg2_exit() -> void __exit {
    static void __exit idtg2_exit(void)
    {
    pr_debug("RIO: %s\n", __func__);
    rio_unregister_driver(&idtg2_driver);
    pr_debug("RIO: %s done\n", __func__);
    }
    device_initcall(idtg2_init);
    module_exit(idtg2_exit);
    MODULE_DESCRIPTION("IDT CPS Gen.2 Serial RapidIO switch family driver");
    MODULE_AUTHOR("Integrated Device Technology, Inc.");
    MODULE_LICENSE("GPL");
