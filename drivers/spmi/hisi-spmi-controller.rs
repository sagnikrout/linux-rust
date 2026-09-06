//! Automatically rewritten from C to Rust
//! Source: drivers/spmi/hisi-spmi-controller.c
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
// SPMI register addr
//
pub const SPMI_CHANNEL_OFFSET: c_uint = 0x0300;
pub const SPMI_SLAVE_OFFSET: c_uint = 0x20;
pub const SPMI_APB_SPMI_CMD_BASE_ADDR: c_uint = 0x0100;
pub const SPMI_APB_SPMI_WDATA0_BASE_ADDR: c_uint = 0x0104;
pub const SPMI_APB_SPMI_WDATA1_BASE_ADDR: c_uint = 0x0108;
pub const SPMI_APB_SPMI_WDATA2_BASE_ADDR: c_uint = 0x010c;
pub const SPMI_APB_SPMI_WDATA3_BASE_ADDR: c_uint = 0x0110;
pub const SPMI_APB_SPMI_STATUS_BASE_ADDR: c_uint = 0x0200;
pub const SPMI_APB_SPMI_RDATA0_BASE_ADDR: c_uint = 0x0204;
pub const SPMI_APB_SPMI_RDATA1_BASE_ADDR: c_uint = 0x0208;
pub const SPMI_APB_SPMI_RDATA2_BASE_ADDR: c_uint = 0x020c;
pub const SPMI_APB_SPMI_RDATA3_BASE_ADDR: c_uint = 0x0210;
pub const SPMI_PER_DATAREG_BYTE: c_int = 4;
//
// SPMI cmd register
//

pub const SPMI_APB_SPMI_CMD_TYPE_OFFSET: c_int = 24;
pub const SPMI_APB_SPMI_CMD_LENGTH_OFFSET: c_int = 20;
pub const SPMI_APB_SPMI_CMD_SLAVEID_OFFSET: c_int = 16;
pub const SPMI_APB_SPMI_CMD_ADDR_OFFSET: c_int = 0;
// Command Opcodes
    enum spmi_controller_cmd_op_code {
    SPMI_CMD_REG_ZERO_WRITE = 0,
    SPMI_CMD_REG_WRITE = 1,
    SPMI_CMD_REG_READ = 2,
    SPMI_CMD_EXT_REG_WRITE = 3,
    SPMI_CMD_EXT_REG_READ = 4,
    SPMI_CMD_EXT_REG_WRITE_L = 5,
    SPMI_CMD_EXT_REG_READ_L = 6,
    SPMI_CMD_REG_RESET = 7,
    SPMI_CMD_REG_SLEEP = 8,
    SPMI_CMD_REG_SHUTDOWN = 9,
    SPMI_CMD_REG_WAKEUP = 10,
    };
//
// SPMI status register
//

// Command register fields
pub const SPMI_CONTROLLER_CMD_MAX_BYTE_COUNT: c_int = 16;
// Maximum number of support PMIC peripherals
pub const SPMI_CONTROLLER_TIMEOUT_US: c_int = 1000;
pub const SPMI_CONTROLLER_MAX_TRANS_BYTES: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spmi_controller_dev {
    pub controller: *mut spmi_controller,
    pub dev: *mut device,
    pub base: *mut void __iomem,
    pub lock: spinlock_t,
    pub channel: u32,
}

    static int spmi_controller_wait_for_done(struct device *dev,
    struct spmi_controller_dev *ctrl_dev,
    void __iomem *base, u8 sid, u16 addr)
    {
    let mut timeout: u32 = SPMI_CONTROLLER_TIMEOUT_US;
    u32 status, offset;
    offset  = SPMI_APB_SPMI_STATUS_BASE_ADDR;
    offset += SPMI_CHANNEL_OFFSET * ctrl_dev.channel + SPMI_SLAVE_OFFSET * sid;
    do {
    status = readl(base + offset);
    if (status & SPMI_APB_TRANS_DONE) {
    if (status & SPMI_APB_TRANS_FAIL) {
    dev_err(dev, "%s: transaction failed (0x%x)\n",
    __func__, status);
    return -EIO;
    }
    dev_dbg(dev, "%s: status 0x%x\n", __func__, status);
    return 0;
    }
    udelay(1);
    } while (timeout--);
    dev_err(dev, "%s: timeout, status 0x%x\n", __func__, status);
    return -ETIMEDOUT;
    }
    static int spmi_read_cmd(struct spmi_controller *ctrl,
    u8 opc, u8 slave_id, u16 slave_addr, u8 *__buf, size_t bc)
    {
    struct spmi_controller_dev *spmi_controller = dev_get_drvdata(&ctrl.dev);
    let mut chnl_ofst: u32 = SPMI_CHANNEL_OFFSET * spmi_controller.channel;
    unsigned long flags;
    u8 *buf = __buf;
    u32 cmd, data;
    int rc;
    u8 op_code, i;
    if (bc > SPMI_CONTROLLER_MAX_TRANS_BYTES) {
    dev_err(&ctrl.dev,
    "spmi_controller supports 1..%d bytes per trans, but:%zu requested\n",
    SPMI_CONTROLLER_MAX_TRANS_BYTES, bc);
    return  -EINVAL;
    }
    switch (opc) {
    case SPMI_CMD_READ:
    op_code = SPMI_CMD_REG_READ;
    break;
    case SPMI_CMD_EXT_READ:
    op_code = SPMI_CMD_EXT_REG_READ;
    break;
    case SPMI_CMD_EXT_READL:
    op_code = SPMI_CMD_EXT_REG_READ_L;
    break;
    default:
    dev_err(&ctrl.dev, "invalid read cmd 0x%x\n", opc);
    return -EINVAL;
    }
    cmd = SPMI_APB_SPMI_CMD_EN |
    (op_code << SPMI_APB_SPMI_CMD_TYPE_OFFSET) |
    ((bc - 1) << SPMI_APB_SPMI_CMD_LENGTH_OFFSET) |
    ((slave_id & 0xf) << SPMI_APB_SPMI_CMD_SLAVEID_OFFSET) |  /* slvid */
    ((slave_addr & 0xffff)  << SPMI_APB_SPMI_CMD_ADDR_OFFSET); /* slave_addr */
    spin_lock_irqsave(&spmi_controller.lock, flags);
    writel(cmd, spmi_controller.base + chnl_ofst + SPMI_APB_SPMI_CMD_BASE_ADDR);
    rc = spmi_controller_wait_for_done(&ctrl.dev, spmi_controller,
    spmi_controller.base, slave_id, slave_addr);
    if (rc)
    goto done;
    for (i = 0; bc > i * SPMI_PER_DATAREG_BYTE; i++) {
    data = readl(spmi_controller.base + chnl_ofst +
    SPMI_SLAVE_OFFSET * slave_id +
    SPMI_APB_SPMI_RDATA0_BASE_ADDR +
    i * SPMI_PER_DATAREG_BYTE);
    data = be32_to_cpu((__be32 )data);
    if ((bc - i * SPMI_PER_DATAREG_BYTE) >> 2) {
    memcpy(buf, &data, sizeof(data));
    buf += sizeof(data);
    } else {
    memcpy(buf, &data, bc % SPMI_PER_DATAREG_BYTE);
    buf += (bc % SPMI_PER_DATAREG_BYTE);
    }
    }
    done:
    spin_unlock_irqrestore(&spmi_controller.lock, flags);
    if (rc)
    dev_err(&ctrl.dev,
    "spmi read wait timeout op:0x%x slave_id:%d slave_addr:0x%x bc:%zu\n",
    opc, slave_id, slave_addr, bc + 1);
    else
    dev_dbg(&ctrl.dev, "%s: id:%d slave_addr:0x%x, read value: %*ph\n",
    __func__, slave_id, slave_addr, (int)bc, __buf);
    return rc;
    }
    static int spmi_write_cmd(struct spmi_controller *ctrl,
    u8 opc, u8 slave_id, u16 slave_addr, const u8 *__buf, size_t bc)
    {
    struct spmi_controller_dev *spmi_controller = dev_get_drvdata(&ctrl.dev);
    let mut chnl_ofst: u32 = SPMI_CHANNEL_OFFSET * spmi_controller.channel;
    const u8 *buf = __buf;
    unsigned long flags;
    u32 cmd, data;
    int rc;
    u8 op_code, i;
    if (bc > SPMI_CONTROLLER_MAX_TRANS_BYTES) {
    dev_err(&ctrl.dev,
    "spmi_controller supports 1..%d bytes per trans, but:%zu requested\n",
    SPMI_CONTROLLER_MAX_TRANS_BYTES, bc);
    return  -EINVAL;
    }
    switch (opc) {
    case SPMI_CMD_WRITE:
    op_code = SPMI_CMD_REG_WRITE;
    break;
    case SPMI_CMD_EXT_WRITE:
    op_code = SPMI_CMD_EXT_REG_WRITE;
    break;
    case SPMI_CMD_EXT_WRITEL:
    op_code = SPMI_CMD_EXT_REG_WRITE_L;
    break;
    default:
    dev_err(&ctrl.dev, "invalid write cmd 0x%x\n", opc);
    return -EINVAL;
    }
    cmd = SPMI_APB_SPMI_CMD_EN |
    (op_code << SPMI_APB_SPMI_CMD_TYPE_OFFSET) |
    ((bc - 1) << SPMI_APB_SPMI_CMD_LENGTH_OFFSET) |
    ((slave_id & 0xf) << SPMI_APB_SPMI_CMD_SLAVEID_OFFSET) |
    ((slave_addr & 0xffff)  << SPMI_APB_SPMI_CMD_ADDR_OFFSET);
// Write data to FIFOs
    spin_lock_irqsave(&spmi_controller.lock, flags);
    for (i = 0; bc > i * SPMI_PER_DATAREG_BYTE; i++) {
    data = 0;
    if ((bc - i * SPMI_PER_DATAREG_BYTE) >> 2) {
    memcpy(&data, buf, sizeof(data));
    buf += sizeof(data);
    } else {
    memcpy(&data, buf, bc % SPMI_PER_DATAREG_BYTE);
    buf += (bc % SPMI_PER_DATAREG_BYTE);
    }
    writel((u32 )cpu_to_be32(data),
    spmi_controller.base + chnl_ofst +
    SPMI_APB_SPMI_WDATA0_BASE_ADDR +
    SPMI_PER_DATAREG_BYTE * i);
    }
// Start the transaction
    writel(cmd, spmi_controller.base + chnl_ofst + SPMI_APB_SPMI_CMD_BASE_ADDR);
    rc = spmi_controller_wait_for_done(&ctrl.dev, spmi_controller,
    spmi_controller.base, slave_id,
    slave_addr);
    spin_unlock_irqrestore(&spmi_controller.lock, flags);
    if (rc)
    dev_err(&ctrl.dev, "spmi write wait timeout op:0x%x slave_id:%d slave_addr:0x%x bc:%zu\n",
    opc, slave_id, slave_addr, bc);
    else
    dev_dbg(&ctrl.dev, "%s: id:%d slave_addr:0x%x, wrote value: %*ph\n",
    __func__, slave_id, slave_addr, (int)bc, __buf);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn spmi_controller_probe(pdev: *mut platform_device) -> c_int {
    static int spmi_controller_probe(struct platform_device *pdev)
    {
    struct spmi_controller_dev *spmi_controller;
    struct spmi_controller *ctrl;
    struct resource *iores;
    int ret;
    ctrl = devm_spmi_controller_alloc(&pdev.dev, sizeof(*spmi_controller));
    if (IS_ERR(ctrl)) {
    dev_err(&pdev.dev, "can not allocate spmi_controller data\n");
    return PTR_ERR(ctrl);
    }
    spmi_controller = spmi_controller_get_drvdata(ctrl);
    spmi_controller.controller = ctrl;
    iores = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    if (!iores) {
    dev_err(&pdev.dev, "can not get resource!\n");
    return -EINVAL;
    }
    spmi_controller.base = devm_ioremap(&pdev.dev, iores.start,
    resource_size(iores));
    if (!spmi_controller.base) {
    dev_err(&pdev.dev, "can not remap base addr!\n");
    return -EADDRNOTAVAIL;
    }
    ret = of_property_read_u32(pdev.dev.of_node, "hisilicon,spmi-channel",
    &spmi_controller.channel);
    if (ret) {
    dev_err(&pdev.dev, "can not get channel\n");
    return -ENODEV;
    }
    platform_set_drvdata(pdev, spmi_controller);
    dev_set_drvdata(&ctrl.dev, spmi_controller);
    spin_lock_init(&spmi_controller.lock);
// Callbacks
    ctrl.read_cmd = spmi_read_cmd;
    ctrl.write_cmd = spmi_write_cmd;
    ret = devm_spmi_controller_add(&pdev.dev, ctrl);
    if (ret) {
    dev_err(&pdev.dev, "spmi_controller_add failed with error %d!\n", ret);
    return ret;
    }
    return 0;
    }
    static const struct of_device_id spmi_controller_match_table[] = {
    {
    .compatible = "hisilicon,kirin970-spmi-controller",
    },
    {}
    };
    MODULE_DEVICE_TABLE(of, spmi_controller_match_table);
    static struct platform_driver spmi_controller_driver = {
    .probe		= spmi_controller_probe,
    .driver		= {
    .name	= "hisi_spmi_controller",
    .of_match_table = spmi_controller_match_table,
    },
    };
#[no_mangle]
unsafe extern "C" fn spmi_controller_init() -> int __init {
    static int __init spmi_controller_init(void)
    {
    return platform_driver_register(&spmi_controller_driver);
    }
    postcore_initcall(spmi_controller_init);
#[no_mangle]
unsafe extern "C" fn spmi_controller_exit() -> void __exit {
    static void __exit spmi_controller_exit(void)
    {
    platform_driver_unregister(&spmi_controller_driver);
    }
    module_exit(spmi_controller_exit);
    MODULE_DESCRIPTION("Hisilicon 3670 SPMI Controller driver");
    MODULE_LICENSE("GPL v2");
    MODULE_VERSION("1.0");
    MODULE_ALIAS("platform:spmi_controller");
