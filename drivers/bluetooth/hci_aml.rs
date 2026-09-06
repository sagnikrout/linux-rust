//! Automatically rewritten from C to Rust
//! Source: drivers/bluetooth/hci_aml.c
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


// SPDX-License-Identifier: (GPL-2.0-only OR MIT)
//
// Copyright (C) 2024 Amlogic, Inc. All rights reserved
//

pub const AML_EVT_HEAD_SIZE: c_int = 4;

// TCI command
pub const AML_TCI_CMD_READ: c_uint = 0xFEF0;
pub const AML_TCI_CMD_WRITE: c_uint = 0xFEF1;
pub const AML_TCI_CMD_UPDATE_BAUDRATE: c_uint = 0xFEF2;
pub const AML_TCI_CMD_HARDWARE_RESET: c_uint = 0xFEF2;
pub const AML_TCI_CMD_DOWNLOAD_BT_FW: c_uint = 0xFEF3;
// Vendor command
pub const AML_BT_HCI_VENDOR_CMD: c_uint = 0xFC1A;
// TCI operation parameter in controller chip
pub const AML_OP_UART_MODE: c_uint = 0x00A30128;
pub const AML_OP_EVT_ENABLE: c_uint = 0x00A70014;
pub const AML_OP_MEM_HARD_TRANS_EN: c_uint = 0x00A7000C;
pub const AML_OP_RF_CFG: c_uint = 0x00F03040;
pub const AML_OP_RAM_POWER_CTR: c_uint = 0x00F03050;
pub const AML_OP_HARDWARE_RST: c_uint = 0x00F03058;
pub const AML_OP_ICCM_RAM_BASE: c_uint = 0x00000000;
pub const AML_OP_DCCM_RAM_BASE: c_uint = 0x00D00000;
// UART configuration

pub const AML_UART_CLK_SOURCE: c_int = 40000000;
// Controller event

// RAM power control

// RF configuration

// Memory transaction

// Controller reset

    enum {
    FW_ICCM,
    FW_DCCM
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aml_fw_len {
    pub iccm_len: u32,
    pub dccm_len: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aml_tci_rsp {
    pub num_cmd_packet: u8,
    pub opcode: u16,
    pub status: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aml_device_data {
    pub iccm_offset: c_int,
    pub dccm_offset: c_int,
    pub is_coex: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aml_serdev {
    pub serdev_hu: hci_uart,
    pub dev: *mut device,
    pub bt_en_gpio: *mut gpio_desc,
    pub bt_supply: *mut regulator,
    pub lpo_clk: *mut clk,
    pub aml_dev_data: *const aml_device_data,
    pub firmware_name: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aml_data {
    pub rx_skb: *mut sk_buff,
    pub txq: sk_buff_head,
}

    static const struct h4_recv_pkt aml_recv_pkts[] = {
    { H4_RECV_ACL, .recv = hci_recv_frame },
    { H4_RECV_SCO, .recv = hci_recv_frame },
    { H4_RECV_EVENT, .recv = hci_recv_frame },
    { H4_RECV_ISO, .recv = hci_recv_frame },
    };
// The TCI command is a private command, which is for setting baud rate,
// downloading firmware, initiating RAM.
//
// op_code |      op_len           | op_addr | parameter   |
// --------|-----------------------|---------|-------------|
// 2B    | 1B len(addr+param)    |    4B   |  len(param) |
//
    static int aml_send_tci_cmd(struct hci_dev *hdev, u16 op_code, u32 op_addr,
    u32 *param, u32 param_len)
    {
    struct aml_tci_rsp *rsp = core::ptr::null_mut();
    struct sk_buff *skb = core::ptr::null_mut();
    let mut buf_len: usize = 0;
    u8 *buf = core::ptr::null_mut();
    let mut err: c_int = 0;
    buf_len = sizeof(op_addr) + param_len;
    buf = kmalloc(buf_len, GFP_KERNEL);
    if (!buf)
    return -ENOMEM;
    memcpy(buf, &op_addr, sizeof(op_addr));
    if (param && param_len > 0)
    memcpy(buf + sizeof(op_addr), param, param_len);
    skb = __hci_cmd_sync_ev(hdev, op_code, buf_len, buf,
    HCI_EV_CMD_COMPLETE, HCI_INIT_TIMEOUT);
    if (IS_ERR(skb)) {
    err = PTR_ERR(skb);
    bt_dev_err(hdev, "Failed to send TCI cmd (error: %d)", err);
    goto exit;
    }
    rsp = skb_pull_data(skb, sizeof(struct aml_tci_rsp));
    if (!rsp)
    goto skb_free;
    if (rsp.opcode != op_code || rsp.status != 0x00) {
    bt_dev_err(hdev, "send TCI cmd (0x%04X), response (0x%04X):(%d)",
    op_code, rsp.opcode, rsp.status);
    err = -EINVAL;
    goto skb_free;
    }
    skb_free:
    kfree_skb(skb);
    exit:
    kfree(buf);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn aml_update_chip_baudrate(hdev: *mut hci_dev, baud: u32) -> c_int {
    static int aml_update_chip_baudrate(struct hci_dev *hdev, u32 baud)
    {
    u32 value;
    value = ((AML_UART_CLK_SOURCE / baud) - 1) & 0x0FFF;
    value |= AML_UART_XMIT_EN | AML_UART_RECV_EN | AML_UART_TIMEOUT_INT_EN;
    return aml_send_tci_cmd(hdev, AML_TCI_CMD_UPDATE_BAUDRATE,
    AML_OP_UART_MODE, &value, sizeof(value));
    }
#[no_mangle]
unsafe extern "C" fn aml_start_chip(hdev: *mut hci_dev) -> c_int {
    static int aml_start_chip(struct hci_dev *hdev)
    {
    let mut value: u32 = 0;
    int ret;
    value = AML_MM_CTR_HARD_TRAS_EN;
    ret = aml_send_tci_cmd(hdev, AML_TCI_CMD_WRITE,
    AML_OP_MEM_HARD_TRANS_EN,
    &value, sizeof(value));
    if (ret)
    return ret;
// controller hardware reset
    value = AML_CTR_CPU_RESET | AML_CTR_MAC_RESET | AML_CTR_PHY_RESET;
    ret = aml_send_tci_cmd(hdev, AML_TCI_CMD_HARDWARE_RESET,
    AML_OP_HARDWARE_RST,
    &value, sizeof(value));
    return ret;
    }
    static int aml_send_firmware_segment(struct hci_dev *hdev,
    u8 fw_type,
    u8 *seg,
    u32 seg_size,
    u32 offset)
    {
    let mut op_addr: u32 = 0;
    if (fw_type == FW_ICCM)
    op_addr = AML_OP_ICCM_RAM_BASE  + offset;
#[no_mangle]
pub unsafe extern "C" fn if(FW_DCCM: fw_type ==) -> else {
    else if (fw_type == FW_DCCM)
    op_addr = AML_OP_DCCM_RAM_BASE + offset;
    return aml_send_tci_cmd(hdev, AML_TCI_CMD_DOWNLOAD_BT_FW,
    op_addr, (u32 *)seg, seg_size);
    }
    static int aml_send_firmware(struct hci_dev *hdev, u8 fw_type,
    u8 *fw, u32 fw_size, u32 offset)
    {
    let mut seg_size: u32 = 0;
    let mut seg_off: u32 = 0;
    if (fw_size > AML_FIRMWARE_MAX_SIZE) {
    bt_dev_err(hdev,
    "Firmware size %d kB is larger than the maximum of 512 kB. Aborting.",
    fw_size);
    return -EINVAL;
    }
    while (fw_size > 0) {
    seg_size = (fw_size > AML_FIRMWARE_OPERATION_SIZE) ?
    AML_FIRMWARE_OPERATION_SIZE : fw_size;
    if (aml_send_firmware_segment(hdev, fw_type, (fw + seg_off),
    seg_size, offset)) {
    bt_dev_err(hdev, "Failed send firmware, type: %d, offset: 0x%x",
    fw_type, offset);
    return -EINVAL;
    }
    seg_off += seg_size;
    fw_size -= seg_size;
    offset += seg_size;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn aml_download_firmware(hdev: *mut hci_dev, fw_name: *const c_char) -> c_int {
    static int aml_download_firmware(struct hci_dev *hdev, const char *fw_name)
    {
    struct hci_uart *hu = hci_get_drvdata(hdev);
    struct aml_serdev *amldev = serdev_device_get_drvdata(hu.serdev);
    const struct firmware *firmware = core::ptr::null_mut();
    const struct aml_fw_len *fw_len = core::ptr::null_mut();
    u8 *iccm_start = core::ptr::null_mut(), *dccm_start = core::ptr::null_mut();
    u32 iccm_len, dccm_len;
    let mut value: u32 = 0;
    let mut ret: c_int = 0;
// Enable firmware download event
    value = AML_EVT_EN;
    ret = aml_send_tci_cmd(hdev, AML_TCI_CMD_WRITE,
    AML_OP_EVT_ENABLE,
    &value, sizeof(value));
    if (ret)
    goto exit;
// RAM power on
    value = AML_RAM_POWER_ON;
    ret = aml_send_tci_cmd(hdev, AML_TCI_CMD_WRITE,
    AML_OP_RAM_POWER_CTR,
    &value, sizeof(value));
    if (ret)
    goto exit;
// Check RAM power status
    ret = aml_send_tci_cmd(hdev, AML_TCI_CMD_READ,
    AML_OP_RAM_POWER_CTR, core::ptr::null_mut(), 0);
    if (ret)
    goto exit;
    ret = request_firmware(&firmware, fw_name, &hdev.dev);
    if (ret < 0) {
    bt_dev_err(hdev, "Failed to load <%s>:(%d)", fw_name, ret);
    goto exit;
    }
    if (firmware.size < sizeof(*fw_len)) {
    bt_dev_err(hdev, "Firmware is too small for its header");
    ret = -EINVAL;
    goto exit;
    }
    fw_len = (const struct aml_fw_len *)firmware.data;
    if (fw_len.iccm_len < amldev.aml_dev_data.iccm_offset ||
    fw_len.iccm_len > firmware.size - sizeof(*fw_len) ||
    fw_len.dccm_len > firmware.size - sizeof(*fw_len) -
    fw_len.iccm_len) {
    bt_dev_err(hdev, "Invalid firmware segment lengths");
    ret = -EINVAL;
    goto exit;
    }
// Download ICCM
    iccm_start = (u8 *)(firmware.data) + sizeof(struct aml_fw_len)
    + amldev.aml_dev_data.iccm_offset;
    iccm_len = fw_len.iccm_len - amldev.aml_dev_data.iccm_offset;
    ret = aml_send_firmware(hdev, FW_ICCM, iccm_start, iccm_len,
    amldev.aml_dev_data.iccm_offset);
    if (ret) {
    bt_dev_err(hdev, "Failed to send FW_ICCM (%d)", ret);
    goto exit;
    }
// Download DCCM
    dccm_start = (u8 *)(firmware.data) + sizeof(struct aml_fw_len) + fw_len.iccm_len;
    dccm_len = fw_len.dccm_len;
    ret = aml_send_firmware(hdev, FW_DCCM, dccm_start, dccm_len,
    amldev.aml_dev_data.dccm_offset);
    if (ret) {
    bt_dev_err(hdev, "Failed to send FW_DCCM (%d)", ret);
    goto exit;
    }
// Disable firmware download event
    value = 0;
    ret = aml_send_tci_cmd(hdev, AML_TCI_CMD_WRITE,
    AML_OP_EVT_ENABLE,
    &value, sizeof(value));
    if (ret)
    goto exit;
    exit:
    release_firmware(firmware);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn aml_send_reset(hdev: *mut hci_dev) -> c_int {
    static int aml_send_reset(struct hci_dev *hdev)
    {
    struct sk_buff *skb;
    int err;
    skb = __hci_cmd_sync_ev(hdev, HCI_OP_RESET, 0, core::ptr::null_mut(),
    HCI_EV_CMD_COMPLETE, HCI_INIT_TIMEOUT);
    if (IS_ERR(skb)) {
    err = PTR_ERR(skb);
    bt_dev_err(hdev, "Failed to send hci reset cmd (%d)", err);
    return err;
    }
    kfree_skb(skb);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn aml_dump_fw_version(hdev: *mut hci_dev) -> c_int {
    static int aml_dump_fw_version(struct hci_dev *hdev)
    {
    struct aml_tci_rsp *rsp = core::ptr::null_mut();
    struct sk_buff *skb;
    u8 value[6] = {0};
    u8 *fw_ver = core::ptr::null_mut();
    let mut err: c_int = 0;
    skb = __hci_cmd_sync_ev(hdev, AML_BT_HCI_VENDOR_CMD, sizeof(value), value,
    HCI_EV_CMD_COMPLETE, HCI_INIT_TIMEOUT);
    if (IS_ERR(skb)) {
    err = PTR_ERR(skb);
    bt_dev_err(hdev, "Failed to get fw version (error: %d)", err);
    return err;
    }
    rsp = skb_pull_data(skb, sizeof(struct aml_tci_rsp));
    if (!rsp)
    goto exit;
    if (rsp.opcode != AML_BT_HCI_VENDOR_CMD || rsp.status != 0x00) {
    bt_dev_err(hdev, "dump version, error response (0x%04X):(%d)",
    rsp.opcode, rsp.status);
    err = -EINVAL;
    goto exit;
    }
    fw_ver = (u8 *)rsp + AML_EVT_HEAD_SIZE;
    bt_dev_info(hdev, "fw_version: date = %02x.%02x, number = 0x%02x%02x",
// (fw_ver + 1), *fw_ver, *(fw_ver + 3), *(fw_ver + 2));
    exit:
    kfree_skb(skb);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn aml_set_bdaddr(hdev: *mut hci_dev, bdaddr: *const bdaddr_t) -> c_int {
    static int aml_set_bdaddr(struct hci_dev *hdev, const bdaddr_t *bdaddr)
    {
    struct aml_tci_rsp *rsp = core::ptr::null_mut();
    struct sk_buff *skb;
    let mut err: c_int = 0;
    bt_dev_info(hdev, "set bdaddr (%pM)", bdaddr);
    skb = __hci_cmd_sync_ev(hdev, AML_BT_HCI_VENDOR_CMD,
    sizeof(bdaddr_t), bdaddr,
    HCI_EV_CMD_COMPLETE, HCI_INIT_TIMEOUT);
    if (IS_ERR(skb)) {
    err = PTR_ERR(skb);
    bt_dev_err(hdev, "Failed to set bdaddr (error: %d)", err);
    return err;
    }
    rsp = skb_pull_data(skb, sizeof(struct aml_tci_rsp));
    if (!rsp)
    goto exit;
    if (rsp.opcode != AML_BT_HCI_VENDOR_CMD || rsp.status != 0x00) {
    bt_dev_err(hdev, "error response (0x%x):(%d)", rsp.opcode, rsp.status);
    err = -EINVAL;
    goto exit;
    }
    exit:
    kfree_skb(skb);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn aml_check_bdaddr(hdev: *mut hci_dev) -> c_int {
    static int aml_check_bdaddr(struct hci_dev *hdev)
    {
    struct hci_rp_read_bd_addr *paddr;
    struct sk_buff *skb;
    int err;
    if (bacmp(&hdev.public_addr, BDADDR_ANY))
    return 0;
    skb = __hci_cmd_sync(hdev, HCI_OP_READ_BD_ADDR, 0, core::ptr::null_mut(),
    HCI_INIT_TIMEOUT);
    if (IS_ERR(skb)) {
    err = PTR_ERR(skb);
    bt_dev_err(hdev, "Failed to read bdaddr (error: %d)", err);
    return err;
    }
    paddr = skb_pull_data(skb, sizeof(struct hci_rp_read_bd_addr));
    if (!paddr)
    goto exit;
    if (!bacmp(&paddr.bdaddr, AML_BDADDR_DEFAULT)) {
    bt_dev_info(hdev, "amlbt using default bdaddr (%pM)", &paddr.bdaddr);
    hci_set_quirk(hdev, HCI_QUIRK_INVALID_BDADDR);
    }
    exit:
    kfree_skb(skb);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn aml_config_rf(hdev: *mut hci_dev, is_coex: bool) -> c_int {
    static int aml_config_rf(struct hci_dev *hdev, bool is_coex)
    {
    let mut value: u32 = AML_RF_ANT_DOUBLE;
// Use a single antenna when co-existing with wifi
    if (is_coex)
    value = AML_RF_ANT_SINGLE;
    return aml_send_tci_cmd(hdev, AML_TCI_CMD_WRITE,
    AML_OP_RF_CFG,
    &value, sizeof(value));
    }
#[no_mangle]
unsafe extern "C" fn aml_parse_dt(amldev: *mut aml_serdev) -> c_int {
    static int aml_parse_dt(struct aml_serdev *amldev)
    {
    struct device *pdev = amldev.dev;
    amldev.bt_en_gpio = devm_gpiod_get(pdev, "enable",
    GPIOD_OUT_LOW);
    if (IS_ERR(amldev.bt_en_gpio)) {
    dev_err(pdev, "Failed to acquire enable gpios");
    return PTR_ERR(amldev.bt_en_gpio);
    }
    if (device_property_read_string(pdev, "firmware-name",
    &amldev.firmware_name)) {
    dev_err(pdev, "Failed to acquire firmware path");
    return -ENODEV;
    }
    amldev.bt_supply = devm_regulator_get(pdev, "vddio");
    if (IS_ERR(amldev.bt_supply)) {
    dev_err(pdev, "Failed to acquire regulator");
    return PTR_ERR(amldev.bt_supply);
    }
    amldev.lpo_clk = devm_clk_get(pdev, core::ptr::null_mut());
    if (IS_ERR(amldev.lpo_clk)) {
    dev_err(pdev, "Failed to acquire clock source");
    return PTR_ERR(amldev.lpo_clk);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn aml_power_on(amldev: *mut aml_serdev) -> c_int {
    static int aml_power_on(struct aml_serdev *amldev)
    {
    int err;
    err = regulator_enable(amldev.bt_supply);
    if (err) {
    dev_err(amldev.dev, "Failed to enable regulator: (%d)", err);
    return err;
    }
    err = clk_prepare_enable(amldev.lpo_clk);
    if (err) {
    dev_err(amldev.dev, "Failed to enable lpo clock: (%d)", err);
    return err;
    }
    gpiod_set_value_cansleep(amldev.bt_en_gpio, 1);
// Wait 20ms for bluetooth controller power on
    msleep(20);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn aml_power_off(amldev: *mut aml_serdev) -> c_int {
    static int aml_power_off(struct aml_serdev *amldev)
    {
    gpiod_set_value_cansleep(amldev.bt_en_gpio, 0);
    clk_disable_unprepare(amldev.lpo_clk);
    regulator_disable(amldev.bt_supply);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn aml_set_baudrate(hu: *mut hci_uart, speed: c_uint) -> c_int {
    static int aml_set_baudrate(struct hci_uart *hu, unsigned int speed)
    {
// update controller baudrate
    if (aml_update_chip_baudrate(hu.hdev, speed) != 0) {
    bt_dev_err(hu.hdev, "Failed to update baud rate");
    return -EINVAL;
    }
// update local baudrate
    serdev_device_set_baudrate(hu.serdev, speed);
    return 0;
    }
// Initialize protocol
#[no_mangle]
unsafe extern "C" fn aml_open(hu: *mut hci_uart) -> c_int {
    static int aml_open(struct hci_uart *hu)
    {
    struct aml_serdev *amldev = serdev_device_get_drvdata(hu.serdev);
    struct aml_data *aml_data;
    int err;
    err = aml_parse_dt(amldev);
    if (err)
    return err;
    if (!hci_uart_has_flow_control(hu)) {
    bt_dev_err(hu.hdev, "no flow control");
    return -EOPNOTSUPP;
    }
    aml_data = kzalloc_obj(*aml_data);
    if (!aml_data)
    return -ENOMEM;
    skb_queue_head_init(&aml_data.txq);
    hu.priv = aml_data;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn aml_close(hu: *mut hci_uart) -> c_int {
    static int aml_close(struct hci_uart *hu)
    {
    struct aml_serdev *amldev = serdev_device_get_drvdata(hu.serdev);
    struct aml_data *aml_data = hu.priv;
    skb_queue_purge(&aml_data.txq);
    kfree_skb(aml_data.rx_skb);
    kfree(aml_data);
    hu.priv = core::ptr::null_mut();
    return aml_power_off(amldev);
    }
#[no_mangle]
unsafe extern "C" fn aml_flush(hu: *mut hci_uart) -> c_int {
    static int aml_flush(struct hci_uart *hu)
    {
    struct aml_data *aml_data = hu.priv;
    skb_queue_purge(&aml_data.txq);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn aml_setup(hu: *mut hci_uart) -> c_int {
    static int aml_setup(struct hci_uart *hu)
    {
    struct aml_serdev *amldev = serdev_device_get_drvdata(hu.serdev);
    struct hci_dev *hdev = amldev.serdev_hu.hdev;
    int err;
// Setup bdaddr
    hdev.set_bdaddr = aml_set_bdaddr;
    err = aml_power_on(amldev);
    if (err)
    return err;
    err = aml_set_baudrate(hu, amldev.serdev_hu.proto.oper_speed);
    if (err)
    return err;
    err = aml_download_firmware(hdev, amldev.firmware_name);
    if (err)
    return err;
    err = aml_config_rf(hdev, amldev.aml_dev_data.is_coex);
    if (err)
    return err;
    err = aml_start_chip(hdev);
    if (err)
    return err;
// Wait 60ms for controller startup
    msleep(60);
    err = aml_dump_fw_version(hdev);
    if (err)
    return err;
    err = aml_send_reset(hdev);
    if (err)
    return err;
    err = aml_check_bdaddr(hdev);
    if (err)
    return err;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn aml_enqueue(hu: *mut hci_uart, skb: *mut sk_buff) -> c_int {
    static int aml_enqueue(struct hci_uart *hu, struct sk_buff *skb)
    {
    struct aml_data *aml_data = hu.priv;
    skb_queue_tail(&aml_data.txq, skb);
    return 0;
    }
    static struct sk_buff *aml_dequeue(struct hci_uart *hu)
    {
    struct aml_data *aml_data = hu.priv;
    struct sk_buff *skb;
    skb = skb_dequeue(&aml_data.txq);
// Prepend skb with frame type
    if (skb)
    memcpy(skb_push(skb, 1), &bt_cb(skb).pkt_type, 1);
    return skb;
    }
#[no_mangle]
unsafe extern "C" fn aml_recv(hu: *mut hci_uart, data: *const c_void, count: c_int) -> c_int {
    static int aml_recv(struct hci_uart *hu, const void *data, int count)
    {
    struct aml_data *aml_data = hu.priv;
    int err;
    aml_data.rx_skb = h4_recv_buf(hu, aml_data.rx_skb, data, count,
    aml_recv_pkts,
    ARRAY_SIZE(aml_recv_pkts));
    if (IS_ERR(aml_data.rx_skb)) {
    err = PTR_ERR(aml_data.rx_skb);
    bt_dev_err(hu.hdev, "Frame reassembly failed (%d)", err);
    aml_data.rx_skb = core::ptr::null_mut();
    return err;
    }
    return count;
    }
    static const struct hci_uart_proto aml_hci_proto = {
    .id		= HCI_UART_AML,
    .name		= "AML",
    .init_speed	= 115200,
    .oper_speed	= 4000000,
    .open		= aml_open,
    .close		= aml_close,
    .setup		= aml_setup,
    .flush		= aml_flush,
    .recv		= aml_recv,
    .enqueue	= aml_enqueue,
    .dequeue	= aml_dequeue,
    };
#[no_mangle]
unsafe extern "C" fn aml_serdev_probe(serdev: *mut serdev_device) -> c_int {
    static int aml_serdev_probe(struct serdev_device *serdev)
    {
    struct aml_serdev *amldev;
    int err;
    amldev = devm_kzalloc(&serdev.dev, sizeof(*amldev), GFP_KERNEL);
    if (!amldev)
    return -ENOMEM;
    amldev.serdev_hu.serdev = serdev;
    amldev.dev = &serdev.dev;
    serdev_device_set_drvdata(serdev, amldev);
    err = hci_uart_register_device(&amldev.serdev_hu, &aml_hci_proto);
    if (err)
    return dev_err_probe(amldev.dev, err,
    "Failed to register hci uart device");
    amldev.aml_dev_data = device_get_match_data(&serdev.dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn aml_serdev_remove(serdev: *mut serdev_device) {
    static void aml_serdev_remove(struct serdev_device *serdev)
    {
    struct aml_serdev *amldev = serdev_device_get_drvdata(serdev);
    hci_uart_unregister_device(&amldev.serdev_hu);
    }
#[no_mangle]
unsafe extern "C" fn aml_serdev_shutdown(serdev: *mut serdev_device) {
    static void aml_serdev_shutdown(struct serdev_device *serdev)
    {
    struct aml_serdev *amldev = serdev_device_get_drvdata(serdev);
    aml_power_off(amldev);
    }
    static const struct aml_device_data data_w155s2 = {
    .iccm_offset = 256 * 1024,
    };
    static const struct aml_device_data data_w265s2 = {
    .iccm_offset = 384 * 1024,
    };
    static const struct of_device_id aml_bluetooth_of_match[] = {
    { .compatible = "amlogic,w155s2-bt", .data = &data_w155s2 },
    { .compatible = "amlogic,w265s2-bt", .data = &data_w265s2 },
    { /* sentinel */ },
    };
    MODULE_DEVICE_TABLE(of, aml_bluetooth_of_match);
    static struct serdev_device_driver aml_serdev_driver = {
    .probe = aml_serdev_probe,
    .remove = aml_serdev_remove,
    .shutdown = aml_serdev_shutdown,
    .driver = {
    .name = "hci_uart_aml",
    .of_match_table = aml_bluetooth_of_match,
    },
    };
#[no_mangle]
pub unsafe extern "C" fn aml_init() -> int __init {
    int __init aml_init(void)
    {
    serdev_device_driver_register(&aml_serdev_driver);
    return hci_uart_register_proto(&aml_hci_proto);
    }
#[no_mangle]
pub unsafe extern "C" fn aml_deinit() -> int __exit {
    int __exit aml_deinit(void)
    {
    serdev_device_driver_unregister(&aml_serdev_driver);
    return hci_uart_unregister_proto(&aml_hci_proto);
    }
