//! Automatically rewritten from C to Rust
//! Source: drivers/phy/realtek/phy-rtk-usb3.c
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
// phy-rtk-usb3.c RTK usb3.0 phy driver
//
// copyright (c) 2023 realtek semiconductor corporation
//

pub const USB_MDIO_CTRL_PHY_ADDR_SHIFT: c_int = 8;
pub const USB_MDIO_CTRL_PHY_DATA_SHIFT: c_int = 16;
pub const MAX_USB_PHY_DATA_SIZE: c_uint = 0x30;
pub const PHY_ADDR_0X09: c_uint = 0x09;
pub const PHY_ADDR_0X0B: c_uint = 0x0b;
pub const PHY_ADDR_0X0D: c_uint = 0x0d;
pub const PHY_ADDR_0X10: c_uint = 0x10;
pub const PHY_ADDR_0X1F: c_uint = 0x1f;
pub const PHY_ADDR_0X20: c_uint = 0x20;
pub const PHY_ADDR_0X21: c_uint = 0x21;
pub const PHY_ADDR_0X30: c_uint = 0x30;

pub const REG_0X0B_RX_OFFSET_RANGE_MASK: c_uint = 0xc;

pub const REG_0X10_DEBUG_MODE_SETTING: c_uint = 0x3c0;
pub const REG_0X10_DEBUG_MODE_SETTING_MASK: c_uint = 0x3f8;
pub const REG_0X1F_RX_OFFSET_CODE_MASK: c_uint = 0x1e;
pub const USB_U3_TX_LFPS_SWING_TRIM_SHIFT: c_int = 4;
pub const USB_U3_TX_LFPS_SWING_TRIM_MASK: c_uint = 0xf;
pub const AMPLITUDE_CONTROL_COARSE_MASK: c_uint = 0xff;
pub const AMPLITUDE_CONTROL_FINE_MASK: c_uint = 0xffff;
pub const AMPLITUDE_CONTROL_COARSE_DEFAULT: c_uint = 0xff;
pub const AMPLITUDE_CONTROL_FINE_DEFAULT: c_uint = 0xffff;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct phy_reg {
    pub reg_mdio_ctl: *mut void __iomem,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct phy_data {
    pub addr: u8,
    pub data: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct phy_cfg {
    pub param_size: c_int,
    pub param: [phy_data; MAX_USB_PHY_DATA_SIZE],
    pub check_efuse: bool,
    pub do_toggle: bool,
    pub do_toggle_once: bool,
    pub use_default_parameter: bool,
    pub check_rx_front_end_offset: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct phy_parameter {
    pub phy_reg: phy_reg,
// Get from efuse
    pub efuse_usb_u3_tx_lfps_swing_trim: u8,
// Get from dts
    pub amplitude_control_coarse: u32,
    pub amplitude_control_fine: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtk_phy {
    pub dev: *mut device,
    pub phy_cfg: *mut phy_cfg,
    pub num_phy: c_int,
    pub phy_parameter: *mut phy_parameter,
    pub debug_dir: *mut dentry,
}

#[no_mangle]
pub unsafe extern "C" fn utmi_wait_register(reg: *mut void __iomem, mask: u32, result: u32) -> c_int {
    static inline int utmi_wait_register(void __iomem *reg, u32 mask, u32 result)
    {
    int ret;
    unsigned int val;
    ret = read_poll_timeout(readl, val, ((val & mask) == result),
    PHY_IO_DELAY_US, PHY_IO_TIMEOUT_USEC, false, reg);
    if (ret) {
    pr_err("%s can't program USB phy\n", __func__);
    return -ETIMEDOUT;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rtk_phy3_wait_vbusy(phy_reg: *mut phy_reg) -> c_int {
    static int rtk_phy3_wait_vbusy(struct phy_reg *phy_reg)
    {
    return utmi_wait_register(phy_reg.reg_mdio_ctl, USB_MDIO_CTRL_PHY_BUSY, 0);
    }
#[no_mangle]
unsafe extern "C" fn rtk_phy_read(phy_reg: *mut phy_reg, addr: c_char) -> u16 {
    static u16 rtk_phy_read(struct phy_reg *phy_reg, char addr)
    {
    unsigned int tmp;
    u32 value;
    tmp = (addr << USB_MDIO_CTRL_PHY_ADDR_SHIFT);
    writel(tmp, phy_reg.reg_mdio_ctl);
    rtk_phy3_wait_vbusy(phy_reg);
    value = readl(phy_reg.reg_mdio_ctl);
    value = value >> USB_MDIO_CTRL_PHY_DATA_SHIFT;
    return (u16)value;
    }
#[no_mangle]
unsafe extern "C" fn rtk_phy_write(phy_reg: *mut phy_reg, addr: c_char, data: u16) -> c_int {
    static int rtk_phy_write(struct phy_reg *phy_reg, char addr, u16 data)
    {
    unsigned int val;
    val = USB_MDIO_CTRL_PHY_WRITE |
    (addr << USB_MDIO_CTRL_PHY_ADDR_SHIFT) |
    (data << USB_MDIO_CTRL_PHY_DATA_SHIFT);
    writel(val, phy_reg.reg_mdio_ctl);
    rtk_phy3_wait_vbusy(phy_reg);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn do_rtk_usb3_phy_toggle(rtk_phy: *mut rtk_phy, index: c_int, connect: bool) {
    static void do_rtk_usb3_phy_toggle(struct rtk_phy *rtk_phy, int index, bool connect)
    {
    struct phy_cfg *phy_cfg = rtk_phy.phy_cfg;
    struct phy_reg *phy_reg;
    struct phy_parameter *phy_parameter;
    struct phy_data *phy_data;
    u8 addr;
    u16 data;
    int i;
    phy_parameter = &((struct phy_parameter *)rtk_phy.phy_parameter)[index];
    phy_reg = &phy_parameter.phy_reg;
    if (!phy_cfg.do_toggle)
    return;
    i = PHY_ADDR_MAP_ARRAY_INDEX(PHY_ADDR_0X09);
    phy_data = phy_cfg.param + i;
    addr = phy_data.addr;
    data = phy_data.data;
    if (!addr && !data) {
    addr = PHY_ADDR_0X09;
    data = rtk_phy_read(phy_reg, addr);
    phy_data.addr = addr;
    phy_data.data = data;
    }
    rtk_phy_write(phy_reg, addr, data & (~REG_0X09_FORCE_CALIBRATION));
    mdelay(1);
    rtk_phy_write(phy_reg, addr, data | REG_0X09_FORCE_CALIBRATION);
    }
#[no_mangle]
unsafe extern "C" fn do_rtk_phy_init(rtk_phy: *mut rtk_phy, index: c_int) -> c_int {
    static int do_rtk_phy_init(struct rtk_phy *rtk_phy, int index)
    {
    struct phy_cfg *phy_cfg;
    struct phy_reg *phy_reg;
    struct phy_parameter *phy_parameter;
    let mut i: c_int = 0;
    phy_cfg = rtk_phy.phy_cfg;
    phy_parameter = &((struct phy_parameter *)rtk_phy.phy_parameter)[index];
    phy_reg = &phy_parameter.phy_reg;
    if (phy_cfg.use_default_parameter)
    goto do_toggle;
    for (i = 0; i < phy_cfg.param_size; i++) {
    struct phy_data *phy_data = phy_cfg.param + i;
    let mut addr: u8 = phy_data.addr;
    let mut data: u16 = phy_data.data;
    if (!addr && !data)
    continue;
    rtk_phy_write(phy_reg, addr, data);
    }
    do_toggle:
    if (phy_cfg.do_toggle_once)
    phy_cfg.do_toggle = true;
    do_rtk_usb3_phy_toggle(rtk_phy, index, false);
    if (phy_cfg.do_toggle_once) {
    let mut check_value: u16 = 0;
    let mut count: c_int = 10;
    u16 value_0x0d, value_0x10;
// Enable Debug mode by set 0x0D and 0x10
    value_0x0d = rtk_phy_read(phy_reg, PHY_ADDR_0X0D);
    value_0x10 = rtk_phy_read(phy_reg, PHY_ADDR_0X10);
    rtk_phy_write(phy_reg, PHY_ADDR_0X0D,
    value_0x0d | REG_0X0D_RX_DEBUG_TEST_EN);
    rtk_phy_write(phy_reg, PHY_ADDR_0X10,
    (value_0x10 & ~REG_0X10_DEBUG_MODE_SETTING_MASK) |
    REG_0X10_DEBUG_MODE_SETTING);
    check_value = rtk_phy_read(phy_reg, PHY_ADDR_0X30);
    while (!(check_value & BIT(15))) {
    check_value = rtk_phy_read(phy_reg, PHY_ADDR_0X30);
    mdelay(1);
    if (count-- < 0)
    break;
    }
    if (!(check_value & BIT(15)))
    dev_info(rtk_phy.dev, "toggle fail addr=0x%02x, data=0x%04x\n",
    PHY_ADDR_0X30, check_value);
// Disable Debug mode by set 0x0D and 0x10 to default
    rtk_phy_write(phy_reg, PHY_ADDR_0X0D, value_0x0d);
    rtk_phy_write(phy_reg, PHY_ADDR_0X10, value_0x10);
    phy_cfg.do_toggle = false;
    }
    if (phy_cfg.check_rx_front_end_offset) {
    u16 rx_offset_code, rx_offset_range;
    let mut code_mask: u16 = REG_0X1F_RX_OFFSET_CODE_MASK;
    let mut range_mask: u16 = REG_0X0B_RX_OFFSET_RANGE_MASK;
    let mut do_update: bool = false;
    rx_offset_code = rtk_phy_read(phy_reg, PHY_ADDR_0X1F);
    if (((rx_offset_code & code_mask) == 0x0) ||
    ((rx_offset_code & code_mask) == code_mask))
    do_update = true;
    rx_offset_range = rtk_phy_read(phy_reg, PHY_ADDR_0X0B);
    if (((rx_offset_range & range_mask) == range_mask) && do_update) {
    dev_warn(rtk_phy.dev, "Don't update rx_offset_range (rx_offset_code=0x%x, rx_offset_range=0x%x)\n",
    rx_offset_code, rx_offset_range);
    do_update = false;
    }
    if (do_update) {
    u16 tmp1, tmp2;
    tmp1 = rx_offset_range & (~range_mask);
    tmp2 = rx_offset_range & range_mask;
    tmp2 += (1 << 2);
    rx_offset_range = tmp1 | (tmp2 & range_mask);
    rtk_phy_write(phy_reg, PHY_ADDR_0X0B, rx_offset_range);
    goto do_toggle;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rtk_phy_init(phy: *mut phy) -> c_int {
    static int rtk_phy_init(struct phy *phy)
    {
    struct rtk_phy *rtk_phy = phy_get_drvdata(phy);
    let mut ret: c_int = 0;
    int i;
    let mut phy_init_time: c_ulong = jiffies;
    for (i = 0; i < rtk_phy.num_phy; i++)
    ret = do_rtk_phy_init(rtk_phy, i);
    dev_dbg(rtk_phy.dev, "Initialized RTK USB 3.0 PHY (take %dms)\n",
    jiffies_to_msecs(jiffies - phy_init_time));
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn rtk_phy_exit(phy: *mut phy) -> c_int {
    static int rtk_phy_exit(struct phy *phy)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rtk_phy_toggle(rtk_phy: *mut rtk_phy, connect: bool, port: c_int) {
    static void rtk_phy_toggle(struct rtk_phy *rtk_phy, bool connect, int port)
    {
    let mut index: c_int = port;
    if (index > rtk_phy.num_phy) {
    dev_err(rtk_phy.dev, "%s: The port=%d is not in usb phy (num_phy=%d)\n",
    __func__, index, rtk_phy.num_phy);
    return;
    }
    do_rtk_usb3_phy_toggle(rtk_phy, index, connect);
    }
#[no_mangle]
unsafe extern "C" fn rtk_phy_connect(phy: *mut phy, port: c_int) -> c_int {
    static int rtk_phy_connect(struct phy *phy, int port)
    {
    struct rtk_phy *rtk_phy = phy_get_drvdata(phy);
    dev_dbg(rtk_phy.dev, "%s port=%d\n", __func__, port);
    rtk_phy_toggle(rtk_phy, true, port);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rtk_phy_disconnect(phy: *mut phy, port: c_int) -> c_int {
    static int rtk_phy_disconnect(struct phy *phy, int port)
    {
    struct rtk_phy *rtk_phy = phy_get_drvdata(phy);
    dev_dbg(rtk_phy.dev, "%s port=%d\n", __func__, port);
    rtk_phy_toggle(rtk_phy, false, port);
    return 0;
    }
    static const struct phy_ops ops = {
    .init		= rtk_phy_init,
    .exit		= rtk_phy_exit,
    .connect	= rtk_phy_connect,
    .disconnect	= rtk_phy_disconnect,
    .owner		= THIS_MODULE,
    };

    static struct dentry *create_phy_debug_root(void)
    {
    struct dentry *phy_debug_root;
    phy_debug_root = debugfs_lookup("phy", usb_debug_root);
    if (!phy_debug_root)
    phy_debug_root = debugfs_create_dir("phy", usb_debug_root);
    return phy_debug_root;
    }
#[no_mangle]
unsafe extern "C" fn rtk_usb3_parameter_show(s: *mut seq_file, unused: *mut c_void) -> c_int {
    static int rtk_usb3_parameter_show(struct seq_file *s, void *unused)
    {
    struct rtk_phy *rtk_phy = s.private;
    struct phy_cfg *phy_cfg;
    int i, index;
    phy_cfg = rtk_phy.phy_cfg;
    seq_puts(s, "Property:\n");
    seq_printf(s, "  check_efuse: %s\n",
    phy_cfg.check_efuse ? "Enable" : "Disable");
    seq_printf(s, "  do_toggle: %s\n",
    phy_cfg.do_toggle ? "Enable" : "Disable");
    seq_printf(s, "  do_toggle_once: %s\n",
    phy_cfg.do_toggle_once ? "Enable" : "Disable");
    seq_printf(s, "  use_default_parameter: %s\n",
    phy_cfg.use_default_parameter ? "Enable" : "Disable");
    for (index = 0; index < rtk_phy.num_phy; index++) {
    struct phy_reg *phy_reg;
    struct phy_parameter *phy_parameter;
    phy_parameter = &((struct phy_parameter *)rtk_phy.phy_parameter)[index];
    phy_reg = &phy_parameter.phy_reg;
    seq_printf(s, "PHY %d:\n", index);
    for (i = 0; i < phy_cfg.param_size; i++) {
    struct phy_data *phy_data = phy_cfg.param + i;
    let mut addr: u8 = ARRAY_INDEX_MAP_PHY_ADDR(i);
    let mut data: u16 = phy_data.data;
    if (!phy_data.addr && !data)
    seq_printf(s, "  addr = 0x%02x, data = none   ==> read value = 0x%04x\n",
    addr, rtk_phy_read(phy_reg, addr));
    else
    seq_printf(s, "  addr = 0x%02x, data = 0x%04x ==> read value = 0x%04x\n",
    addr, data, rtk_phy_read(phy_reg, addr));
    }
    seq_puts(s, "PHY Property:\n");
    seq_printf(s, "  efuse_usb_u3_tx_lfps_swing_trim: 0x%x\n",
    (int)phy_parameter.efuse_usb_u3_tx_lfps_swing_trim);
    seq_printf(s, "  amplitude_control_coarse: 0x%x\n",
    (int)phy_parameter.amplitude_control_coarse);
    seq_printf(s, "  amplitude_control_fine: 0x%x\n",
    (int)phy_parameter.amplitude_control_fine);
    }
    return 0;
    }
    DEFINE_SHOW_ATTRIBUTE(rtk_usb3_parameter);
#[no_mangle]
pub unsafe extern "C" fn create_debug_files(rtk_phy: *mut rtk_phy) {
    static inline void create_debug_files(struct rtk_phy *rtk_phy)
    {
    struct dentry *phy_debug_root = core::ptr::null_mut();
    phy_debug_root = create_phy_debug_root();
    if (!phy_debug_root)
    return;
    rtk_phy.debug_dir = debugfs_create_dir(dev_name(rtk_phy.dev), phy_debug_root);
    debugfs_create_file("parameter", 0444, rtk_phy.debug_dir, rtk_phy,
    &rtk_usb3_parameter_fops);
    }
#[no_mangle]
pub unsafe extern "C" fn remove_debug_files(rtk_phy: *mut rtk_phy) {
    static inline void remove_debug_files(struct rtk_phy *rtk_phy)
    {
    debugfs_remove_recursive(rtk_phy.debug_dir);
    }

    static inline void create_debug_files(struct rtk_phy *rtk_phy) { }
    static inline void remove_debug_files(struct rtk_phy *rtk_phy) { }

    static int get_phy_data_by_efuse(struct rtk_phy *rtk_phy,
    struct phy_parameter *phy_parameter, int index)
    {
    struct phy_cfg *phy_cfg = rtk_phy.phy_cfg;
    let mut value: u8 = 0;
    struct nvmem_cell *cell;
    if (!phy_cfg.check_efuse)
    goto out;
    cell = nvmem_cell_get(rtk_phy.dev, "usb_u3_tx_lfps_swing_trim");
    if (IS_ERR(cell)) {
    dev_dbg(rtk_phy.dev, "%s no usb_u3_tx_lfps_swing_trim: %ld\n",
    __func__, PTR_ERR(cell));
    } else {
    unsigned char *buf;
    size_t buf_size;
    buf = nvmem_cell_read(cell, &buf_size);
    if (!IS_ERR(buf)) {
    value = buf[0] & USB_U3_TX_LFPS_SWING_TRIM_MASK;
    kfree(buf);
    }
    nvmem_cell_put(cell);
    }
    if (value > 0 && value < 0x8)
    phy_parameter.efuse_usb_u3_tx_lfps_swing_trim = 0x8;
    else
    phy_parameter.efuse_usb_u3_tx_lfps_swing_trim = (u8)value;
    out:
    return 0;
    }
    static void update_amplitude_control_value(struct rtk_phy *rtk_phy,
    struct phy_parameter *phy_parameter)
    {
    struct phy_cfg *phy_cfg;
    struct phy_reg *phy_reg;
    phy_reg = &phy_parameter.phy_reg;
    phy_cfg = rtk_phy.phy_cfg;
    if (phy_parameter.amplitude_control_coarse != AMPLITUDE_CONTROL_COARSE_DEFAULT) {
    let mut val_mask: u16 = AMPLITUDE_CONTROL_COARSE_MASK;
    u16 data;
    if (!phy_cfg.param[PHY_ADDR_0X20].addr && !phy_cfg.param[PHY_ADDR_0X20].data) {
    phy_cfg.param[PHY_ADDR_0X20].addr = PHY_ADDR_0X20;
    data = rtk_phy_read(phy_reg, PHY_ADDR_0X20);
    } else {
    data = phy_cfg.param[PHY_ADDR_0X20].data;
    }
    data &= (~val_mask);
    data |= (phy_parameter.amplitude_control_coarse & val_mask);
    phy_cfg.param[PHY_ADDR_0X20].data = data;
    }
    if (phy_parameter.efuse_usb_u3_tx_lfps_swing_trim) {
    let mut efuse_val: u8 = phy_parameter.efuse_usb_u3_tx_lfps_swing_trim;
    let mut val_mask: u16 = USB_U3_TX_LFPS_SWING_TRIM_MASK;
    let mut val_shift: c_int = USB_U3_TX_LFPS_SWING_TRIM_SHIFT;
    u16 data;
    if (!phy_cfg.param[PHY_ADDR_0X20].addr && !phy_cfg.param[PHY_ADDR_0X20].data) {
    phy_cfg.param[PHY_ADDR_0X20].addr = PHY_ADDR_0X20;
    data = rtk_phy_read(phy_reg, PHY_ADDR_0X20);
    } else {
    data = phy_cfg.param[PHY_ADDR_0X20].data;
    }
    data &= ~(val_mask << val_shift);
    data |= ((efuse_val & val_mask) << val_shift);
    phy_cfg.param[PHY_ADDR_0X20].data = data;
    }
    if (phy_parameter.amplitude_control_fine != AMPLITUDE_CONTROL_FINE_DEFAULT) {
    let mut val_mask: u16 = AMPLITUDE_CONTROL_FINE_MASK;
    if (!phy_cfg.param[PHY_ADDR_0X21].addr && !phy_cfg.param[PHY_ADDR_0X21].data)
    phy_cfg.param[PHY_ADDR_0X21].addr = PHY_ADDR_0X21;
    phy_cfg.param[PHY_ADDR_0X21].data =
    phy_parameter.amplitude_control_fine & val_mask;
    }
    }
#[no_mangle]
unsafe extern "C" fn parse_phy_data(rtk_phy: *mut rtk_phy) -> c_int {
    static int parse_phy_data(struct rtk_phy *rtk_phy)
    {
    struct device *dev = rtk_phy.dev;
    struct phy_parameter *phy_parameter;
    let mut ret: c_int = 0;
    int index;
    rtk_phy.phy_parameter = devm_kzalloc(dev, sizeof(struct phy_parameter) *
    rtk_phy.num_phy, GFP_KERNEL);
    if (!rtk_phy.phy_parameter)
    return -ENOMEM;
    for (index = 0; index < rtk_phy.num_phy; index++) {
    phy_parameter = &((struct phy_parameter *)rtk_phy.phy_parameter)[index];
    phy_parameter.phy_reg.reg_mdio_ctl = of_iomap(dev.of_node, 0) + index;
// Amplitude control address 0x20 bit 0 to bit 7
    if (of_property_read_u32(dev.of_node, "realtek,amplitude-control-coarse-tuning",
    &phy_parameter.amplitude_control_coarse))
    phy_parameter.amplitude_control_coarse = AMPLITUDE_CONTROL_COARSE_DEFAULT;
// Amplitude control address 0x21 bit 0 to bit 16
    if (of_property_read_u32(dev.of_node, "realtek,amplitude-control-fine-tuning",
    &phy_parameter.amplitude_control_fine))
    phy_parameter.amplitude_control_fine = AMPLITUDE_CONTROL_FINE_DEFAULT;
    get_phy_data_by_efuse(rtk_phy, phy_parameter, index);
    update_amplitude_control_value(rtk_phy, phy_parameter);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn rtk_usb3phy_probe(pdev: *mut platform_device) -> c_int {
    static int rtk_usb3phy_probe(struct platform_device *pdev)
    {
    struct rtk_phy *rtk_phy;
    struct device *dev = &pdev.dev;
    struct phy *generic_phy;
    struct phy_provider *phy_provider;
    const struct phy_cfg *phy_cfg;
    int ret;
    phy_cfg = of_device_get_match_data(dev);
    if (!phy_cfg) {
    dev_err(dev, "phy config are not assigned!\n");
    return -EINVAL;
    }
    rtk_phy = devm_kzalloc(dev, sizeof(*rtk_phy), GFP_KERNEL);
    if (!rtk_phy)
    return -ENOMEM;
    rtk_phy.dev			= &pdev.dev;
    rtk_phy.phy_cfg = devm_kzalloc(dev, sizeof(*phy_cfg), GFP_KERNEL);
    if (!rtk_phy.phy_cfg)
    return -ENOMEM;
    memcpy(rtk_phy.phy_cfg, phy_cfg, sizeof(*phy_cfg));
    rtk_phy.num_phy = 1;
    ret = parse_phy_data(rtk_phy);
    if (ret)
    goto err;
    platform_set_drvdata(pdev, rtk_phy);
    generic_phy = devm_phy_create(rtk_phy.dev, core::ptr::null_mut(), &ops);
    if (IS_ERR(generic_phy))
    return PTR_ERR(generic_phy);
    phy_set_drvdata(generic_phy, rtk_phy);
    phy_provider = devm_of_phy_provider_register(rtk_phy.dev, of_phy_simple_xlate);
    if (IS_ERR(phy_provider))
    return PTR_ERR(phy_provider);
    create_debug_files(rtk_phy);
    err:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn rtk_usb3phy_remove(pdev: *mut platform_device) {
    static void rtk_usb3phy_remove(struct platform_device *pdev)
    {
    struct rtk_phy *rtk_phy = platform_get_drvdata(pdev);
    remove_debug_files(rtk_phy);
    }
    static const struct phy_cfg rtd1295_phy_cfg = {
    .param_size = MAX_USB_PHY_DATA_SIZE,
    .param = {  [0] = {0x01, 0x4008},  [1] = {0x01, 0xe046},
    [2] = {0x02, 0x6046},  [3] = {0x03, 0x2779},
    [4] = {0x04, 0x72f5},  [5] = {0x05, 0x2ad3},
    [6] = {0x06, 0x000e},  [7] = {0x07, 0x2e00},
    [8] = {0x08, 0x3591},  [9] = {0x09, 0x525c},
    [10] = {0x0a, 0xa600}, [11] = {0x0b, 0xa904},
    [12] = {0x0c, 0xc000}, [13] = {0x0d, 0xef1c},
    [14] = {0x0e, 0x2000}, [15] = {0x0f, 0x0000},
    [16] = {0x10, 0x000c}, [17] = {0x11, 0x4c00},
    [18] = {0x12, 0xfc00}, [19] = {0x13, 0x0c81},
    [20] = {0x14, 0xde01}, [21] = {0x15, 0x0000},
    [22] = {0x16, 0x0000}, [23] = {0x17, 0x0000},
    [24] = {0x18, 0x0000}, [25] = {0x19, 0x4004},
    [26] = {0x1a, 0x1260}, [27] = {0x1b, 0xff00},
    [28] = {0x1c, 0xcb00}, [29] = {0x1d, 0xa03f},
    [30] = {0x1e, 0xc2e0}, [31] = {0x1f, 0x2807},
    [32] = {0x20, 0x947a}, [33] = {0x21, 0x88aa},
    [34] = {0x22, 0x0057}, [35] = {0x23, 0xab66},
    [36] = {0x24, 0x0800}, [37] = {0x25, 0x0000},
    [38] = {0x26, 0x040a}, [39] = {0x27, 0x01d6},
    [40] = {0x28, 0xf8c2}, [41] = {0x29, 0x3080},
    [42] = {0x2a, 0x3082}, [43] = {0x2b, 0x2078},
    [44] = {0x2c, 0xffff}, [45] = {0x2d, 0xffff},
    [46] = {0x2e, 0x0000}, [47] = {0x2f, 0x0040}, },
    .check_efuse = false,
    .do_toggle = true,
    .do_toggle_once = false,
    .use_default_parameter = false,
    .check_rx_front_end_offset = false,
    };
    static const struct phy_cfg rtd1619_phy_cfg = {
    .param_size = MAX_USB_PHY_DATA_SIZE,
    .param = {  [8] = {0x08, 0x3591},
    [38] = {0x26, 0x840b},
    [40] = {0x28, 0xf842}, },
    .check_efuse = false,
    .do_toggle = true,
    .do_toggle_once = false,
    .use_default_parameter = false,
    .check_rx_front_end_offset = false,
    };
    static const struct phy_cfg rtd1319_phy_cfg = {
    .param_size = MAX_USB_PHY_DATA_SIZE,
    .param = {  [1] = {0x01, 0xac86},
    [6] = {0x06, 0x0003},
    [9] = {0x09, 0x924c},
    [10] = {0x0a, 0xa608},
    [11] = {0x0b, 0xb905},
    [14] = {0x0e, 0x2010},
    [32] = {0x20, 0x705a},
    [33] = {0x21, 0xf645},
    [34] = {0x22, 0x0013},
    [35] = {0x23, 0xcb66},
    [41] = {0x29, 0xff00}, },
    .check_efuse = true,
    .do_toggle = true,
    .do_toggle_once = false,
    .use_default_parameter = false,
    .check_rx_front_end_offset = false,
    };
    static const struct phy_cfg rtd1619b_phy_cfg = {
    .param_size = MAX_USB_PHY_DATA_SIZE,
    .param = {  [1] = {0x01, 0xac8c},
    [6] = {0x06, 0x0017},
    [9] = {0x09, 0x724c},
    [10] = {0x0a, 0xb610},
    [11] = {0x0b, 0xb90d},
    [13] = {0x0d, 0xef2a},
    [15] = {0x0f, 0x9050},
    [16] = {0x10, 0x000c},
    [32] = {0x20, 0x70ff},
    [34] = {0x22, 0x0013},
    [35] = {0x23, 0xdb66},
    [38] = {0x26, 0x8609},
    [41] = {0x29, 0xff13},
    [42] = {0x2a, 0x3070}, },
    .check_efuse = true,
    .do_toggle = false,
    .do_toggle_once = true,
    .use_default_parameter = false,
    .check_rx_front_end_offset = false,
    };
    static const  struct phy_cfg rtd1319d_phy_cfg = {
    .param_size = MAX_USB_PHY_DATA_SIZE,
    .param = {  [1] = {0x01, 0xac89},
    [4] = {0x04, 0xf2f5},
    [6] = {0x06, 0x0017},
    [9] = {0x09, 0x424c},
    [10] = {0x0a, 0x9610},
    [11] = {0x0b, 0x9901},
    [12] = {0x0c, 0xf000},
    [13] = {0x0d, 0xef2a},
    [14] = {0x0e, 0x1000},
    [15] = {0x0f, 0x9050},
    [32] = {0x20, 0x7077},
    [35] = {0x23, 0x0b62},
    [37] = {0x25, 0x10ec},
    [42] = {0x2a, 0x3070}, },
    .check_efuse = true,
    .do_toggle = false,
    .do_toggle_once = true,
    .use_default_parameter = false,
    .check_rx_front_end_offset = true,
    };
    static const struct of_device_id usbphy_rtk_dt_match[] = {
    { .compatible = "realtek,rtd1295-usb3phy", .data = &rtd1295_phy_cfg },
    { .compatible = "realtek,rtd1319-usb3phy", .data = &rtd1319_phy_cfg },
    { .compatible = "realtek,rtd1319d-usb3phy", .data = &rtd1319d_phy_cfg },
    { .compatible = "realtek,rtd1619-usb3phy", .data = &rtd1619_phy_cfg },
    { .compatible = "realtek,rtd1619b-usb3phy", .data = &rtd1619b_phy_cfg },
    {},
    };
    MODULE_DEVICE_TABLE(of, usbphy_rtk_dt_match);
    static struct platform_driver rtk_usb3phy_driver = {
    .probe		= rtk_usb3phy_probe,
    .remove		= rtk_usb3phy_remove,
    .driver		= {
    .name	= "rtk-usb3phy",
    .of_match_table = usbphy_rtk_dt_match,
    },
    };
    module_platform_driver(rtk_usb3phy_driver);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Stanley Chang <stanley_chang@realtek.com>");
    MODULE_DESCRIPTION("Realtek usb 3.0 phy driver");
