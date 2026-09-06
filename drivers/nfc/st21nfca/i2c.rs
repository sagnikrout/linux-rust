//! Automatically rewritten from C to Rust
//! Source: drivers/nfc/st21nfca/i2c.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// I2C Link Layer for ST21NFCA HCI based Driver
// Copyright (C) 2014  STMicroelectronics SAS. All rights reserved.
//

//
// Every frame starts with ST21NFCA_SOF_EOF and ends with ST21NFCA_SOF_EOF.
// Because ST21NFCA_SOF_EOF is a possible data value, there is a mecanism
// called byte stuffing has been introduced.
//
// if byte == ST21NFCA_SOF_EOF or ST21NFCA_ESCAPE_BYTE_STUFFING
// - insert ST21NFCA_ESCAPE_BYTE_STUFFING (escape byte)
// - xor byte with ST21NFCA_BYTE_STUFFING_MASK
//
pub const ST21NFCA_SOF_EOF: c_uint = 0x7e;
pub const ST21NFCA_BYTE_STUFFING_MASK: c_uint = 0x20;
pub const ST21NFCA_ESCAPE_BYTE_STUFFING: c_uint = 0x7d;
// SOF + 00
pub const ST21NFCA_FRAME_HEADROOM: c_int = 2;
// 2 bytes crc + EOF
pub const ST21NFCA_FRAME_TAILROOM: c_int = 3;

    buf[1] == 0)

#[repr(C)]
#[derive(Copy, Clone)]
pub struct st21nfca_i2c_phy {
    pub i2c_dev: *mut i2c_client,
    pub hdev: *mut nfc_hci_dev,
    pub gpiod_ena: *mut gpio_desc,
    pub se_status: st21nfca_se_status,
    pub pending_skb: *mut sk_buff,
    pub current_read_len: c_int,
//
// crc might have fail because i2c macro
// is disable due to other interface activity
//
    pub crc_trials: c_int,
    pub powered: c_int,
    pub run_mode: c_int,
//
// < 0 if hardware error occured (e.g. i2c err)
// and prevents normal operation.
//
    pub hard_fault: c_int,
    pub phy_lock: mutex,
}

    static const u8 len_seq[] = { 16, 24, 12, 29 };
    static const u16 wait_tab[] = { 2, 3, 5, 15, 20, 40};

    do {								\
    pr_debug("%s:\n", info);				\
    print_hex_dump(KERN_DEBUG, "i2c: ", DUMP_PREFIX_OFFSET,	\
    16, 1, (skb).data, (skb).len, 0);	\
    } while (0)
//
// In order to get the CLF in a known state we generate an internal reboot
// using a proprietary command.
// Once the reboot is completed, we expect to receive a ST21NFCA_SOF_EOF
// fill buffer.
//
#[no_mangle]
unsafe extern "C" fn st21nfca_hci_platform_init(phy: *mut st21nfca_i2c_phy) -> c_int {
    static int st21nfca_hci_platform_init(struct st21nfca_i2c_phy *phy)
    {
    u16 wait_reboot[] = { 50, 300, 1000 };
    char reboot_cmd[] = { 0x7E, 0x66, 0x48, 0xF6, 0x7E };
    u8 tmp[ST21NFCA_HCI_LLC_MAX_SIZE];
    int i, r = -1;
    for (i = 0; i < ARRAY_SIZE(wait_reboot) && r < 0; i++) {
    r = i2c_master_send(phy.i2c_dev, reboot_cmd,
    sizeof(reboot_cmd));
    if (r < 0)
    msleep(wait_reboot[i]);
    }
    if (r < 0)
    return r;
// CLF is spending about 20ms to do an internal reboot
    msleep(20);
    r = -1;
    for (i = 0; i < ARRAY_SIZE(wait_reboot) && r < 0; i++) {
    r = i2c_master_recv(phy.i2c_dev, tmp,
    ST21NFCA_HCI_LLC_MAX_SIZE);
    if (r < 0)
    msleep(wait_reboot[i]);
    }
    if (r < 0)
    return r;
    for (i = 0; i < ST21NFCA_HCI_LLC_MAX_SIZE &&
    tmp[i] == ST21NFCA_SOF_EOF; i++)
    ;
    if (r != ST21NFCA_HCI_LLC_MAX_SIZE)
    return -ENODEV;
    usleep_range(1000, 1500);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn st21nfca_hci_i2c_enable(phy_id: *mut c_void) -> c_int {
    static int st21nfca_hci_i2c_enable(void *phy_id)
    {
    struct st21nfca_i2c_phy *phy = phy_id;
    gpiod_set_value(phy.gpiod_ena, 1);
    phy.powered = 1;
    phy.run_mode = ST21NFCA_HCI_MODE;
    usleep_range(10000, 15000);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn st21nfca_hci_i2c_disable(phy_id: *mut c_void) {
    static void st21nfca_hci_i2c_disable(void *phy_id)
    {
    struct st21nfca_i2c_phy *phy = phy_id;
    gpiod_set_value(phy.gpiod_ena, 0);
    phy.powered = 0;
    }
#[no_mangle]
unsafe extern "C" fn st21nfca_hci_add_len_crc(skb: *mut sk_buff) {
    static void st21nfca_hci_add_len_crc(struct sk_buff *skb)
    {
    u16 crc;
    u8 tmp;
// (u8 *)skb_push(skb, 1) = 0;
    crc = crc_ccitt(0xffff, skb.data, skb.len);
    crc = ~crc;
    tmp = crc & 0x00ff;
    skb_put_u8(skb, tmp);
    tmp = (crc >> 8) & 0x00ff;
    skb_put_u8(skb, tmp);
    }
#[no_mangle]
unsafe extern "C" fn st21nfca_hci_remove_len_crc(skb: *mut sk_buff) {
    static void st21nfca_hci_remove_len_crc(struct sk_buff *skb)
    {
    skb_pull(skb, ST21NFCA_FRAME_HEADROOM);
    skb_trim(skb, skb.len - ST21NFCA_FRAME_TAILROOM);
    }
//
// Writing a frame must not return the number of written bytes.
// It must return either zero for success, or <0 for error.
// In addition, it must not alter the skb
//
#[no_mangle]
unsafe extern "C" fn st21nfca_hci_i2c_write(phy_id: *mut c_void, skb: *mut sk_buff) -> c_int {
    static int st21nfca_hci_i2c_write(void *phy_id, struct sk_buff *skb)
    {
    let mut r: c_int = -1, i, j;
    struct st21nfca_i2c_phy *phy = phy_id;
    struct i2c_client *client = phy.i2c_dev;
    u8 tmp[ST21NFCA_HCI_LLC_MAX_SIZE * 2];
    I2C_DUMP_SKB("st21nfca_hci_i2c_write", skb);
    if (phy.hard_fault != 0)
    return phy.hard_fault;
//
// Compute CRC before byte stuffing computation on frame
// Note st21nfca_hci_add_len_crc is doing a byte stuffing
// on its own value
//
    st21nfca_hci_add_len_crc(skb);
// add ST21NFCA_SOF_EOF on tail
    skb_put_u8(skb, ST21NFCA_SOF_EOF);
// add ST21NFCA_SOF_EOF on head
// (u8 *)skb_push(skb, 1) = ST21NFCA_SOF_EOF;
//
// Compute byte stuffing
// if byte == ST21NFCA_SOF_EOF or ST21NFCA_ESCAPE_BYTE_STUFFING
// insert ST21NFCA_ESCAPE_BYTE_STUFFING (escape byte)
// xor byte with ST21NFCA_BYTE_STUFFING_MASK
//
    tmp[0] = skb.data[0];
    for (i = 1, j = 1; i < skb.len - 1; i++, j++) {
    if (skb.data[i] == ST21NFCA_SOF_EOF
    || skb.data[i] == ST21NFCA_ESCAPE_BYTE_STUFFING) {
    tmp[j] = ST21NFCA_ESCAPE_BYTE_STUFFING;
    j++;
    tmp[j] = skb.data[i] ^ ST21NFCA_BYTE_STUFFING_MASK;
    } else {
    tmp[j] = skb.data[i];
    }
    }
    tmp[j] = skb.data[i];
    j++;
//
// Manage sleep mode
// Try 3 times to send data with delay between each
//
    mutex_lock(&phy.phy_lock);
    for (i = 0; i < ARRAY_SIZE(wait_tab) && r < 0; i++) {
    r = i2c_master_send(client, tmp, j);
    if (r < 0)
    msleep(wait_tab[i]);
    }
    mutex_unlock(&phy.phy_lock);
    if (r >= 0) {
    if (r != j)
    r = -EREMOTEIO;
    else
    r = 0;
    }
    st21nfca_hci_remove_len_crc(skb);
    return r;
    }
#[no_mangle]
unsafe extern "C" fn get_frame_size(buf: *mut u8, buflen: c_int) -> c_int {
    static int get_frame_size(u8 *buf, int buflen)
    {
    let mut len: c_int = 0;
    if (buf[len + 1] == ST21NFCA_SOF_EOF)
    return 0;
    for (len = 1; len < buflen && buf[len] != ST21NFCA_SOF_EOF; len++)
    ;
    return len;
    }
#[no_mangle]
unsafe extern "C" fn check_crc(buf: *mut u8, buflen: c_int) -> c_int {
    static int check_crc(u8 *buf, int buflen)
    {
    u16 crc;
    crc = crc_ccitt(0xffff, buf, buflen - 2);
    crc = ~crc;
    if (buf[buflen - 2] != (crc & 0xff) || buf[buflen - 1] != (crc >> 8)) {
    pr_err(ST21NFCA_HCI_DRIVER_NAME
    ": CRC error 0x%x != 0x%x 0x%x\n", crc, buf[buflen - 1],
    buf[buflen - 2]);
    pr_info(DRIVER_DESC ": %s : BAD CRC\n", __func__);
    print_hex_dump(KERN_DEBUG, "crc: ", DUMP_PREFIX_NONE,
    16, 2, buf, buflen, false);
    return -EPERM;
    }
    return 0;
    }
//
// Prepare received data for upper layer.
// Received data include byte stuffing, crc and sof/eof
// which is not usable by hci part.
// returns:
// frame size without sof/eof, header and byte stuffing
// -EBADMSG : frame was incorrect and discarded
//
#[no_mangle]
unsafe extern "C" fn st21nfca_hci_i2c_repack(skb: *mut sk_buff) -> c_int {
    static int st21nfca_hci_i2c_repack(struct sk_buff *skb)
    {
    int i, j, r, size;
    if (skb.len < 1 || (skb.len > 1 && skb.data[1] != 0))
    return -EBADMSG;
    size = get_frame_size(skb.data, skb.len);
    if (size > 0) {
    skb_trim(skb, size);
// remove ST21NFCA byte stuffing for upper layer
    for (i = 1, j = 0; i < skb.len; i++) {
    if (skb.data[i + j] ==
    (u8) ST21NFCA_ESCAPE_BYTE_STUFFING) {
    skb.data[i] = skb.data[i + j + 1]
    | ST21NFCA_BYTE_STUFFING_MASK;
    i++;
    j++;
    }
    skb.data[i] = skb.data[i + j];
    }
// remove byte stuffing useless byte
    skb_trim(skb, i - j);
// remove ST21NFCA_SOF_EOF from head
    skb_pull(skb, 1);
    r = check_crc(skb.data, skb.len);
    if (r != 0)
    return -EBADMSG;
// remove headbyte
    skb_pull(skb, 1);
// remove crc. Byte Stuffing is already removed here
    skb_trim(skb, skb.len - 2);
    return skb.len;
    }
    return 0;
    }
//
// Reads an shdlc frame and returns it in a newly allocated sk_buff. Guarantees
// that i2c bus will be flushed and that next read will start on a new frame.
// returned skb contains only LLC header and payload.
// returns:
// frame size : if received frame is complete (find ST21NFCA_SOF_EOF at
// end of read)
// -EAGAIN : if received frame is incomplete (not find ST21NFCA_SOF_EOF
// at end of read)
// -EREMOTEIO : i2c read error (fatal)
// -EBADMSG : frame was incorrect and discarded
// (value returned from st21nfca_hci_i2c_repack)
// -EIO : if no ST21NFCA_SOF_EOF is found after reaching
// the read length end sequence
//
    static int st21nfca_hci_i2c_read(struct st21nfca_i2c_phy *phy,
    struct sk_buff *skb)
    {
    int r, i;
    u8 len;
    u8 buf[ST21NFCA_HCI_LLC_MAX_PAYLOAD];
    struct i2c_client *client = phy.i2c_dev;
    if (phy.current_read_len < ARRAY_SIZE(len_seq)) {
    len = len_seq[phy.current_read_len];
//
// Add retry mecanism
// Operation on I2C interface may fail in case of operation on
// RF or SWP interface
//
    r = 0;
    mutex_lock(&phy.phy_lock);
    for (i = 0; i < ARRAY_SIZE(wait_tab) && r <= 0; i++) {
    r = i2c_master_recv(client, buf, len);
    if (r < 0)
    msleep(wait_tab[i]);
    }
    mutex_unlock(&phy.phy_lock);
    if (r != len) {
    phy.current_read_len = 0;
    return -EREMOTEIO;
    }
//
// The first read sequence does not start with SOF.
// Data is corrupeted so we drop it.
//
    if (!phy.current_read_len && !IS_START_OF_FRAME(buf)) {
    skb_trim(skb, 0);
    phy.current_read_len = 0;
    return -EIO;
    } else if (phy.current_read_len && IS_START_OF_FRAME(buf)) {
//
// Previous frame transmission was interrupted and
// the frame got repeated.
// Received frame start with ST21NFCA_SOF_EOF + 00.
//
    skb_trim(skb, 0);
    phy.current_read_len = 0;
    }
    skb_put_data(skb, buf, len);
    if (skb.data[skb.len - 1] == ST21NFCA_SOF_EOF) {
    phy.current_read_len = 0;
    return st21nfca_hci_i2c_repack(skb);
    }
    phy.current_read_len++;
    return -EAGAIN;
    }
    return -EIO;
    }
//
// Reads an shdlc frame from the chip. This is not as straightforward as it
// seems. The frame format is data-crc, and corruption can occur anywhere
// while transiting on i2c bus, such that we could read an invalid data.
// The tricky case is when we read a corrupted data or crc. We must detect
// this here in order to determine that data can be transmitted to the hci
// core. This is the reason why we check the crc here.
// The CLF will repeat a frame until we send a RR on that frame.
//
// On ST21NFCA, IRQ goes in idle when read starts. As no size information are
// available in the incoming data, other IRQ might come. Every IRQ will trigger
// a read sequence with different length and will fill the current frame.
// The reception is complete once we reach a ST21NFCA_SOF_EOF.
//
#[no_mangle]
unsafe extern "C" fn st21nfca_hci_irq_thread_fn(irq: c_int, phy_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t st21nfca_hci_irq_thread_fn(int irq, void *phy_id)
    {
    struct st21nfca_i2c_phy *phy = phy_id;
    int r;
    if (!phy || irq != phy.i2c_dev.irq) {
    WARN_ON_ONCE(1);
    return IRQ_NONE;
    }
    if (phy.hard_fault != 0)
    return IRQ_HANDLED;
    r = st21nfca_hci_i2c_read(phy, phy.pending_skb);
    if (r == -EREMOTEIO) {
    phy.hard_fault = r;
    nfc_hci_recv_frame(phy.hdev, core::ptr::null_mut());
    return IRQ_HANDLED;
    } else if (r == -EAGAIN || r == -EIO) {
    return IRQ_HANDLED;
    } else if (r == -EBADMSG && phy.crc_trials < ARRAY_SIZE(wait_tab)) {
//
// With ST21NFCA, only one interface (I2C, RF or SWP)
// may be active at a time.
// Having incorrect crc is usually due to i2c macrocell
// deactivation in the middle of a transmission.
// It may generate corrupted data on i2c.
// We give sometime to get i2c back.
// The complete frame will be repeated.
//
    msleep(wait_tab[phy.crc_trials]);
    phy.crc_trials++;
    phy.current_read_len = 0;
    kfree_skb(phy.pending_skb);
    } else if (r > 0) {
//
// We succeeded to read data from the CLF and
// data is valid.
// Reset counter.
//
    nfc_hci_recv_frame(phy.hdev, phy.pending_skb);
    phy.crc_trials = 0;
    } else {
    kfree_skb(phy.pending_skb);
    }
    phy.pending_skb = alloc_skb(ST21NFCA_HCI_LLC_MAX_SIZE * 2, GFP_KERNEL);
    if (phy.pending_skb == core::ptr::null_mut()) {
    phy.hard_fault = -ENOMEM;
    nfc_hci_recv_frame(phy.hdev, core::ptr::null_mut());
    }
    return IRQ_HANDLED;
    }
    static const struct nfc_phy_ops i2c_phy_ops = {
    .write = st21nfca_hci_i2c_write,
    .enable = st21nfca_hci_i2c_enable,
    .disable = st21nfca_hci_i2c_disable,
    };
    let mut enable_gpios: static struct acpi_gpio_params = { 1, 0, false };
    static const struct acpi_gpio_mapping acpi_st21nfca_gpios[] = {
    { "enable-gpios", &enable_gpios, 1 },
    {},
    };
#[no_mangle]
unsafe extern "C" fn st21nfca_hci_i2c_probe(client: *mut i2c_client) -> c_int {
    static int st21nfca_hci_i2c_probe(struct i2c_client *client)
    {
    struct device *dev = &client.dev;
    struct st21nfca_i2c_phy *phy;
    int r;
    if (!i2c_check_functionality(client.adapter, I2C_FUNC_I2C)) {
    nfc_err(&client.dev, "Need I2C_FUNC_I2C\n");
    return -ENODEV;
    }
    phy = devm_kzalloc(&client.dev, sizeof(struct st21nfca_i2c_phy),
    GFP_KERNEL);
    if (!phy)
    return -ENOMEM;
    phy.i2c_dev = client;
    phy.pending_skb = alloc_skb(ST21NFCA_HCI_LLC_MAX_SIZE * 2, GFP_KERNEL);
    if (phy.pending_skb == core::ptr::null_mut())
    return -ENOMEM;
    phy.current_read_len = 0;
    phy.crc_trials = 0;
    mutex_init(&phy.phy_lock);
    i2c_set_clientdata(client, phy);
    r = devm_acpi_dev_add_driver_gpios(dev, acpi_st21nfca_gpios);
    if (r)
    dev_dbg(dev, "Unable to add GPIO mapping table\n");
// Get EN GPIO from resource provider
    phy.gpiod_ena = devm_gpiod_get(dev, "enable", GPIOD_OUT_LOW);
    if (IS_ERR(phy.gpiod_ena)) {
    nfc_err(dev, "Unable to get ENABLE GPIO\n");
    r = PTR_ERR(phy.gpiod_ena);
    goto out_free;
    }
    phy.se_status.is_ese_present =
    device_property_read_bool(&client.dev, "ese-present");
    phy.se_status.is_uicc_present =
    device_property_read_bool(&client.dev, "uicc-present");
    r = st21nfca_hci_platform_init(phy);
    if (r < 0) {
    nfc_err(&client.dev, "Unable to reboot st21nfca\n");
    goto out_free;
    }
    r = devm_request_threaded_irq(&client.dev, client.irq, core::ptr::null_mut(),
    st21nfca_hci_irq_thread_fn,
    IRQF_ONESHOT,
    ST21NFCA_HCI_DRIVER_NAME, phy);
    if (r < 0) {
    nfc_err(&client.dev, "Unable to register IRQ handler\n");
    goto out_free;
    }
    r = st21nfca_hci_probe(phy, &i2c_phy_ops, LLC_SHDLC_NAME,
    ST21NFCA_FRAME_HEADROOM,
    ST21NFCA_FRAME_TAILROOM,
    ST21NFCA_HCI_LLC_MAX_PAYLOAD,
    &phy.hdev,
    &phy.se_status);
    if (r)
    goto out_free;
    return 0;
    out_free:
    kfree_skb(phy.pending_skb);
    return r;
    }
#[no_mangle]
unsafe extern "C" fn st21nfca_hci_i2c_remove(client: *mut i2c_client) {
    static void st21nfca_hci_i2c_remove(struct i2c_client *client)
    {
    struct st21nfca_i2c_phy *phy = i2c_get_clientdata(client);
    st21nfca_hci_remove(phy.hdev);
    if (phy.powered)
    st21nfca_hci_i2c_disable(phy);
    kfree_skb(phy.pending_skb);
    }
    static const struct i2c_device_id st21nfca_hci_i2c_id_table[] = {
    { .name = ST21NFCA_HCI_DRIVER_NAME },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, st21nfca_hci_i2c_id_table);
    static const struct acpi_device_id st21nfca_hci_i2c_acpi_match[] = {
    { .id = "SMO2100" },
    { }
    };
    MODULE_DEVICE_TABLE(acpi, st21nfca_hci_i2c_acpi_match);
    static const struct of_device_id of_st21nfca_i2c_match[] = {
    { .compatible = "st,st21nfca-i2c" },
    { .compatible = "st,st21nfca_i2c" },
    { }
    };
    MODULE_DEVICE_TABLE(of, of_st21nfca_i2c_match);
    static struct i2c_driver st21nfca_hci_i2c_driver = {
    .driver = {
    .name = ST21NFCA_HCI_I2C_DRIVER_NAME,
    .of_match_table = of_match_ptr(of_st21nfca_i2c_match),
    .acpi_match_table = ACPI_PTR(st21nfca_hci_i2c_acpi_match),
    },
    .probe = st21nfca_hci_i2c_probe,
    .id_table = st21nfca_hci_i2c_id_table,
    .remove = st21nfca_hci_i2c_remove,
    };
    module_i2c_driver(st21nfca_hci_i2c_driver);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION(DRIVER_DESC);
