//! Automatically rewritten from C to Rust
//! Source: drivers/platform/x86/intel/punit_ipc.c
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
// Driver for the Intel P-Unit Mailbox IPC mechanism
//
// (C) Copyright 2015 Intel Corporation
//
// The heart of the P-Unit is the Foxton microcontroller and its firmware,
// which provide mailbox interface for power management usage.
//

// IPC Mailbox registers
pub const OFFSET_DATA_LOW: c_uint = 0x0;
pub const OFFSET_DATA_HIGH: c_uint = 0x4;
// bit field of interface register

pub const CMD_PARA1_SHIFT: c_int = 8;
pub const CMD_PARA2_SHIFT: c_int = 16;
pub const CMD_TIMEOUT_SECONDS: c_int = 1;
    enum {
    BASE_DATA = 0,
    BASE_IFACE,
    BASE_MAX,
    };
    typedef struct {
    struct device *dev;
    struct mutex lock;
    int irq;
    struct completion cmd_complete;
// base of interface and data registers
    void __iomem *base[RESERVED_IPC][BASE_MAX];
    IPC_TYPE type;
    } IPC_DEV;
    static IPC_DEV *punit_ipcdev;
#[no_mangle]
pub unsafe extern "C" fn ipc_read_status(ipcdev: *mut IPC_DEV, type: IPC_TYPE) -> u32 {
    static inline u32 ipc_read_status(IPC_DEV *ipcdev, IPC_TYPE type)
    {
    return readl(ipcdev.base[type][BASE_IFACE]);
    }
#[no_mangle]
pub unsafe extern "C" fn ipc_write_cmd(ipcdev: *mut IPC_DEV, type: IPC_TYPE, cmd: u32) {
    static inline void ipc_write_cmd(IPC_DEV *ipcdev, IPC_TYPE type, u32 cmd)
    {
    writel(cmd, ipcdev.base[type][BASE_IFACE]);
    }
#[no_mangle]
pub unsafe extern "C" fn ipc_read_data_low(ipcdev: *mut IPC_DEV, type: IPC_TYPE) -> u32 {
    static inline u32 ipc_read_data_low(IPC_DEV *ipcdev, IPC_TYPE type)
    {
    return readl(ipcdev.base[type][BASE_DATA] + OFFSET_DATA_LOW);
    }
#[no_mangle]
pub unsafe extern "C" fn ipc_read_data_high(ipcdev: *mut IPC_DEV, type: IPC_TYPE) -> u32 {
    static inline u32 ipc_read_data_high(IPC_DEV *ipcdev, IPC_TYPE type)
    {
    return readl(ipcdev.base[type][BASE_DATA] + OFFSET_DATA_HIGH);
    }
#[no_mangle]
pub unsafe extern "C" fn ipc_write_data_low(ipcdev: *mut IPC_DEV, type: IPC_TYPE, data: u32) {
    static inline void ipc_write_data_low(IPC_DEV *ipcdev, IPC_TYPE type, u32 data)
    {
    writel(data, ipcdev.base[type][BASE_DATA] + OFFSET_DATA_LOW);
    }
#[no_mangle]
pub unsafe extern "C" fn ipc_write_data_high(ipcdev: *mut IPC_DEV, type: IPC_TYPE, data: u32) {
    static inline void ipc_write_data_high(IPC_DEV *ipcdev, IPC_TYPE type, u32 data)
    {
    writel(data, ipcdev.base[type][BASE_DATA] + OFFSET_DATA_HIGH);
    }
    static const char *ipc_err_string(int error)
    {
    if (error == IPC_PUNIT_ERR_SUCCESS)
    return "no error";
#[no_mangle]
pub unsafe extern "C" fn if(IPC_PUNIT_ERR_INVALID_CMD: error ==) -> else {
    else if (error == IPC_PUNIT_ERR_INVALID_CMD)
    return "invalid command";
#[no_mangle]
pub unsafe extern "C" fn if(IPC_PUNIT_ERR_INVALID_PARAMETER: error ==) -> else {
    else if (error == IPC_PUNIT_ERR_INVALID_PARAMETER)
    return "invalid parameter";
#[no_mangle]
pub unsafe extern "C" fn if(IPC_PUNIT_ERR_CMD_TIMEOUT: error ==) -> else {
    else if (error == IPC_PUNIT_ERR_CMD_TIMEOUT)
    return "command timeout";
#[no_mangle]
pub unsafe extern "C" fn if(IPC_PUNIT_ERR_CMD_LOCKED: error ==) -> else {
    else if (error == IPC_PUNIT_ERR_CMD_LOCKED)
    return "command locked";
#[no_mangle]
pub unsafe extern "C" fn if(IPC_PUNIT_ERR_INVALID_VR_ID: error ==) -> else {
    else if (error == IPC_PUNIT_ERR_INVALID_VR_ID)
    return "invalid vr id";
#[no_mangle]
pub unsafe extern "C" fn if(IPC_PUNIT_ERR_VR_ERR: error ==) -> else {
    else if (error == IPC_PUNIT_ERR_VR_ERR)
    return "vr error";
    else
    return "unknown error";
    }
#[no_mangle]
unsafe extern "C" fn intel_punit_ipc_check_status(ipcdev: *mut IPC_DEV, type: IPC_TYPE) -> c_int {
    static int intel_punit_ipc_check_status(IPC_DEV *ipcdev, IPC_TYPE type)
    {
    let mut loops: c_int = CMD_TIMEOUT_SECONDS * USEC_PER_SEC;
    int errcode;
    int status;
    if (ipcdev.irq) {
    if (!wait_for_completion_timeout(&ipcdev.cmd_complete,
    CMD_TIMEOUT_SECONDS * HZ)) {
    dev_err(ipcdev.dev, "IPC timed out\n");
    return -ETIMEDOUT;
    }
    } else {
    while ((ipc_read_status(ipcdev, type) & CMD_RUN) && --loops)
    udelay(1);
    if (!loops) {
    dev_err(ipcdev.dev, "IPC timed out\n");
    return -ETIMEDOUT;
    }
    }
    status = ipc_read_status(ipcdev, type);
    errcode = status & CMD_ERRCODE_MASK;
    if (errcode) {
    dev_err(ipcdev.dev, "IPC failed: %s, IPC_STS=0x%x\n",
    ipc_err_string(errcode), status);
    return -EIO;
    }
    return 0;
    }
//
// intel_punit_ipc_command() - IPC command with data and pointers
// @cmd:	IPC command code.
// @para1:	First 8bit parameter, set 0 if not used.
// @para2:	Second 8bit parameter, set 0 if not used.
// @in:		Input data, 32bit for BIOS cmd, two 32bit for GTD and ISPD.
// @out:	Output data.
//
// Send a IPC command to P-Unit with data transaction
//
// Return:	IPC error code or 0 on success.
//
#[no_mangle]
pub unsafe extern "C" fn intel_punit_ipc_command(cmd: u32, para1: u32, para2: u32, in: *mut u32, out: *mut u32) -> c_int {
    int intel_punit_ipc_command(u32 cmd, u32 para1, u32 para2, u32 *in, u32 *out)
    {
    IPC_DEV *ipcdev = punit_ipcdev;
    IPC_TYPE type;
    u32 val;
    int ret;
    mutex_lock(&ipcdev.lock);
    reinit_completion(&ipcdev.cmd_complete);
    type = (cmd & IPC_PUNIT_CMD_TYPE_MASK) >> IPC_TYPE_OFFSET;
    if (in) {
    ipc_write_data_low(ipcdev, type, *in);
    if (type == GTDRIVER_IPC || type == ISPDRIVER_IPC)
    ipc_write_data_high(ipcdev, type, *++in);
    }
    val = cmd & ~IPC_PUNIT_CMD_TYPE_MASK;
    val |= CMD_RUN | para2 << CMD_PARA2_SHIFT | para1 << CMD_PARA1_SHIFT;
    ipc_write_cmd(ipcdev, type, val);
    ret = intel_punit_ipc_check_status(ipcdev, type);
    if (ret)
    goto out;
    if (out) {
// out = ipc_read_data_low(ipcdev, type);
    if (type == GTDRIVER_IPC || type == ISPDRIVER_IPC)
// ++out = ipc_read_data_high(ipcdev, type);
    }
    out:
    mutex_unlock(&ipcdev.lock);
    return ret;
    }
    EXPORT_SYMBOL_GPL(intel_punit_ipc_command);
#[no_mangle]
unsafe extern "C" fn intel_punit_ioc(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t intel_punit_ioc(int irq, void *dev_id)
    {
    IPC_DEV *ipcdev = dev_id;
    complete(&ipcdev.cmd_complete);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn intel_punit_get_bars(pdev: *mut platform_device) -> c_int {
    static int intel_punit_get_bars(struct platform_device *pdev)
    {
    void __iomem *addr;
//
// The following resources are required
// - BIOS_IPC BASE_DATA
// - BIOS_IPC BASE_IFACE
//
    addr = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(addr))
    return PTR_ERR(addr);
    punit_ipcdev.base[BIOS_IPC][BASE_DATA] = addr;
    addr = devm_platform_ioremap_resource(pdev, 1);
    if (IS_ERR(addr))
    return PTR_ERR(addr);
    punit_ipcdev.base[BIOS_IPC][BASE_IFACE] = addr;
//
// The following resources are optional
// - ISPDRIVER_IPC BASE_DATA
// - ISPDRIVER_IPC BASE_IFACE
// - GTDRIVER_IPC BASE_DATA
// - GTDRIVER_IPC BASE_IFACE
//
    addr = devm_platform_ioremap_resource(pdev, 2);
    if (!IS_ERR(addr))
    punit_ipcdev.base[ISPDRIVER_IPC][BASE_DATA] = addr;
    addr = devm_platform_ioremap_resource(pdev, 3);
    if (!IS_ERR(addr))
    punit_ipcdev.base[ISPDRIVER_IPC][BASE_IFACE] = addr;
    addr = devm_platform_ioremap_resource(pdev, 4);
    if (!IS_ERR(addr))
    punit_ipcdev.base[GTDRIVER_IPC][BASE_DATA] = addr;
    addr = devm_platform_ioremap_resource(pdev, 5);
    if (!IS_ERR(addr))
    punit_ipcdev.base[GTDRIVER_IPC][BASE_IFACE] = addr;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn intel_punit_ipc_probe(pdev: *mut platform_device) -> c_int {
    static int intel_punit_ipc_probe(struct platform_device *pdev)
    {
    int irq, ret;
    punit_ipcdev = devm_kzalloc(&pdev.dev,
    sizeof(*punit_ipcdev), GFP_KERNEL);
    if (!punit_ipcdev)
    return -ENOMEM;
    platform_set_drvdata(pdev, punit_ipcdev);
    irq = platform_get_irq_optional(pdev, 0);
    if (irq < 0) {
    dev_warn(&pdev.dev, "Invalid IRQ, using polling mode\n");
    } else {
    ret = devm_request_irq(&pdev.dev, irq, intel_punit_ioc,
    IRQF_NO_SUSPEND, "intel_punit_ipc",
    punit_ipcdev);
    if (ret) {
    dev_err(&pdev.dev, "Failed to request irq: %d\n", irq);
    return ret;
    }
    punit_ipcdev.irq = irq;
    }
    ret = intel_punit_get_bars(pdev);
    if (ret)
    return ret;
    punit_ipcdev.dev = &pdev.dev;
    mutex_init(&punit_ipcdev.lock);
    init_completion(&punit_ipcdev.cmd_complete);
    return 0;
    }
    static const struct acpi_device_id punit_ipc_acpi_ids[] = {
    { "INT34D4", 0 },
    { }
    };
    MODULE_DEVICE_TABLE(acpi, punit_ipc_acpi_ids);
    static struct platform_driver intel_punit_ipc_driver = {
    .probe = intel_punit_ipc_probe,
    .driver = {
    .name = "intel_punit_ipc",
    .acpi_match_table = punit_ipc_acpi_ids,
    },
    };
#[no_mangle]
unsafe extern "C" fn intel_punit_ipc_init() -> int __init {
    static int __init intel_punit_ipc_init(void)
    {
    return platform_driver_register(&intel_punit_ipc_driver);
    }
#[no_mangle]
unsafe extern "C" fn intel_punit_ipc_exit() -> void __exit {
    static void __exit intel_punit_ipc_exit(void)
    {
    platform_driver_unregister(&intel_punit_ipc_driver);
    }
    MODULE_AUTHOR("Zha Qipeng <qipeng.zha@intel.com>");
    MODULE_DESCRIPTION("Intel P-Unit IPC driver");
    MODULE_LICENSE("GPL v2");
// Some modules are dependent on this, so init earlier
    fs_initcall(intel_punit_ipc_init);
    module_exit(intel_punit_ipc_exit);
