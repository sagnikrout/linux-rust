//! Automatically rewritten from C to Rust
//! Source: drivers/fsi/fsi-occ.c
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

pub const OCC_SRAM_BYTES: c_int = 8192;
pub const OCC_CMD_DATA_BYTES: c_int = 8186;
pub const OCC_RESP_DATA_BYTES: c_int = 8185;
pub const OCC_P9_SRAM_CMD_ADDR: c_uint = 0xFFFBE000;
pub const OCC_P9_SRAM_RSP_ADDR: c_uint = 0xFFFBF000;
pub const OCC_P10_SRAM_CMD_ADDR: c_uint = 0xFFFFD000;
pub const OCC_P10_SRAM_RSP_ADDR: c_uint = 0xFFFFE000;
pub const OCC_P10_SRAM_MODE: c_uint = 0x58	/* Normal mode, OCB channel 2 */;
pub const OCC_TIMEOUT_MS: c_int = 1000;
pub const OCC_CMD_IN_PRG_WAIT_MS: c_int = 50;
    enum versions { occ_p9, occ_p10 };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct occ {
    pub dev: *mut device,
    pub sbefifo: *mut device,
    pub name: [c_char; 32],
    pub idx: c_int,
    pub platform_hwmon: bool,
    pub sequence_number: u8,
    pub buffer: *mut c_void,
    pub client_buffer: *mut c_void,
    pub client_buffer_size: usize,
    pub client_response_size: usize,
    pub version: enum versions,
    pub mdev: miscdevice,
    pub occ_lock: mutex,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct occ_response {
    pub seq_no: u8,
    pub cmd_type: u8,
    pub return_status: u8,
    pub data_length: __be16,
    pub /: *mut *mut u8 data[OCC_RESP_DATA_BYTES + 2]; / two bytes checksum,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct occ_client {
    pub occ: *mut occ,
    pub lock: mutex,
    pub data_size: usize,
    pub read_offset: usize,
    pub buffer: *mut u8,
}

    static DEFINE_IDA(occ_ida);
#[no_mangle]
unsafe extern "C" fn occ_open(inode: *mut inode, file: *mut file) -> c_int {
    static int occ_open(struct inode *inode, struct file *file)
    {
    struct occ_client *client = kzalloc_obj(*client);
    struct miscdevice *mdev = file.private_data;
    struct occ *occ = to_occ(mdev);
    if (!client)
    return -ENOMEM;
    client.buffer = kvmalloc(OCC_SRAM_BYTES, GFP_KERNEL);
    if (!client.buffer) {
    kfree(client);
    return -ENOMEM;
    }
    client.occ = occ;
    mutex_init(&client.lock);
    file.private_data = client;
    get_device(occ.dev);
    return 0;
    }
    static ssize_t occ_read(struct file *file, char __user *buf, size_t len,
    loff_t *offset)
    {
    struct occ_client *client = file.private_data;
    let mut rc: isize = 0;
    if (!client)
    return -ENODEV;
    if (len > OCC_SRAM_BYTES)
    return -EINVAL;
    mutex_lock(&client.lock);
// This should not be possible ...
    if (WARN_ON_ONCE(client.read_offset > client.data_size)) {
    rc = -EIO;
    goto done;
    }
// Grab how much data we have to read
    rc = min(len, client.data_size - client.read_offset);
    if (copy_to_user(buf, client.buffer + client.read_offset, rc))
    rc = -EFAULT;
    else
    client.read_offset += rc;
    done:
    mutex_unlock(&client.lock);
    return rc;
    }
    static ssize_t occ_write(struct file *file, const char __user *buf,
    size_t len, loff_t *offset)
    {
    struct occ_client *client = file.private_data;
    size_t rlen, data_length;
    ssize_t rc;
    u8 *cmd;
    if (!client)
    return -ENODEV;
    if (len > (OCC_CMD_DATA_BYTES + 3) || len < 3)
    return -EINVAL;
    mutex_lock(&client.lock);
// Construct the command
    cmd = client.buffer;
//
// Copy the user command (assume user data follows the occ command
// format)
// byte 0: command type
// bytes 1-2: data length (msb first)
// bytes 3-n: data
//
    if (copy_from_user(&cmd[1], buf, len)) {
    rc = -EFAULT;
    goto done;
    }
// Extract data length
    data_length = (cmd[2] << 8) + cmd[3];
    if (data_length > OCC_CMD_DATA_BYTES) {
    rc = -EINVAL;
    goto done;
    }
// Submit command; 4 bytes before the data and 2 bytes after
    rlen = OCC_SRAM_BYTES;
    rc = fsi_occ_submit(client.occ.dev, cmd, data_length + 6, cmd,
    &rlen);
    if (rc)
    goto done;
// Set read tracking data
    client.data_size = rlen;
    client.read_offset = 0;
// Done
    rc = len;
    done:
    mutex_unlock(&client.lock);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn occ_release(inode: *mut inode, file: *mut file) -> c_int {
    static int occ_release(struct inode *inode, struct file *file)
    {
    struct occ_client *client = file.private_data;
    put_device(client.occ.dev);
    kvfree(client.buffer);
    kfree(client);
    return 0;
    }
    static const struct file_operations occ_fops = {
    .owner = THIS_MODULE,
    .open = occ_open,
    .read = occ_read,
    .write = occ_write,
    .release = occ_release,
    };
    static void occ_save_ffdc(struct occ *occ, __be32 *resp, size_t parsed_len,
    size_t resp_len)
    {
    if (resp_len > parsed_len) {
    let mut dh: usize = resp_len - parsed_len;
    size_t ffdc_len = (dh - 1) * 4; /* SBE words are four bytes */
    __be32 *ffdc = &resp[parsed_len];
    if (ffdc_len > occ.client_buffer_size)
    ffdc_len = occ.client_buffer_size;
    memcpy(occ.client_buffer, ffdc, ffdc_len);
    occ.client_response_size = ffdc_len;
    }
    }
    static int occ_verify_checksum(struct occ *occ, struct occ_response *resp,
    u16 data_length)
    {
// Fetch the two bytes after the data for the checksum.
    let mut checksum_resp: u16 = get_unaligned_be16(&resp.data[data_length]);
    u16 checksum;
    u16 i;
    checksum = resp.seq_no;
    checksum += resp.cmd_type;
    checksum += resp.return_status;
    checksum += (data_length >> 8) + (data_length & 0xFF);
    for (i = 0; i < data_length; ++i)
    checksum += resp.data[i];
    if (checksum != checksum_resp) {
    dev_err(occ.dev, "Bad checksum: %04x!=%04x\n", checksum,
    checksum_resp);
    return -EBADE;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn occ_getsram(occ: *mut occ, offset: u32, data: *mut c_void, len: isize) -> c_int {
    static int occ_getsram(struct occ *occ, u32 offset, void *data, ssize_t len)
    {
    u32 data_len = ((len + 7) / 8) * 8;	/* must be multiples of 8 B */
    size_t cmd_len, parsed_len, resp_data_len;
    let mut resp_len: usize = OCC_MAX_RESP_WORDS;
    __be32 *resp = occ.buffer;
    __be32 cmd[6];
    let mut idx: c_int = 0, rc;
//
// Magic sequence to do SBE getsram command. SBE will fetch data from
// specified SRAM address.
//
    switch (occ.version) {
    default:
    case occ_p9:
    cmd_len = 5;
    cmd[2] = cpu_to_be32(1);	/* Normal mode */
    cmd[3] = cpu_to_be32(OCC_P9_SRAM_RSP_ADDR + offset);
    break;
    case occ_p10:
    idx = 1;
    cmd_len = 6;
    cmd[2] = cpu_to_be32(OCC_P10_SRAM_MODE);
    cmd[3] = 0;
    cmd[4] = cpu_to_be32(OCC_P10_SRAM_RSP_ADDR + offset);
    break;
    }
    cmd[0] = cpu_to_be32(cmd_len);
    cmd[1] = cpu_to_be32(SBEFIFO_CMD_GET_OCC_SRAM);
    cmd[4 + idx] = cpu_to_be32(data_len);
    rc = sbefifo_submit(occ.sbefifo, cmd, cmd_len, resp, &resp_len);
    if (rc)
    return rc;
    rc = sbefifo_parse_status(occ.sbefifo, SBEFIFO_CMD_GET_OCC_SRAM,
    resp, resp_len, &parsed_len);
    if (rc > 0) {
    dev_err(occ.dev, "SRAM read returned failure status: %08x\n",
    rc);
    occ_save_ffdc(occ, resp, parsed_len, resp_len);
    return -ECOMM;
    } else if (rc) {
    return rc;
    }
    resp_data_len = be32_to_cpu(resp[parsed_len - 1]);
    if (resp_data_len != data_len) {
    dev_err(occ.dev, "SRAM read expected %d bytes got %zd\n",
    data_len, resp_data_len);
    rc = -EBADMSG;
    } else {
    memcpy(data, resp, len);
    }
    return rc;
    }
    static int occ_putsram(struct occ *occ, const void *data, ssize_t len,
    u8 seq_no, u16 checksum)
    {
    u32 data_len = ((len + 7) / 8) * 8;	/* must be multiples of 8 B */
    size_t cmd_len, parsed_len, resp_data_len;
    let mut resp_len: usize = OCC_MAX_RESP_WORDS;
    __be32 *buf = occ.buffer;
    u8 *byte_buf;
    let mut idx: c_int = 0, rc;
    cmd_len = (occ.version == occ_p10) ? 6 : 5;
    cmd_len += data_len >> 2;
//
// Magic sequence to do SBE putsram command. SBE will transfer
// data to specified SRAM address.
//
    buf[0] = cpu_to_be32(cmd_len);
    buf[1] = cpu_to_be32(SBEFIFO_CMD_PUT_OCC_SRAM);
    switch (occ.version) {
    default:
    case occ_p9:
    buf[2] = cpu_to_be32(1);	/* Normal mode */
    buf[3] = cpu_to_be32(OCC_P9_SRAM_CMD_ADDR);
    break;
    case occ_p10:
    idx = 1;
    buf[2] = cpu_to_be32(OCC_P10_SRAM_MODE);
    buf[3] = 0;
    buf[4] = cpu_to_be32(OCC_P10_SRAM_CMD_ADDR);
    break;
    }
    buf[4 + idx] = cpu_to_be32(data_len);
    memcpy(&buf[5 + idx], data, len);
    byte_buf = (u8 *)&buf[5 + idx];
//
// Overwrite the first byte with our sequence number and the last two
// bytes with the checksum.
//
    byte_buf[0] = seq_no;
    byte_buf[len - 2] = checksum >> 8;
    byte_buf[len - 1] = checksum & 0xff;
    rc = sbefifo_submit(occ.sbefifo, buf, cmd_len, buf, &resp_len);
    if (rc)
    return rc;
    rc = sbefifo_parse_status(occ.sbefifo, SBEFIFO_CMD_PUT_OCC_SRAM,
    buf, resp_len, &parsed_len);
    if (rc > 0) {
    dev_err(occ.dev, "SRAM write returned failure status: %08x\n",
    rc);
    occ_save_ffdc(occ, buf, parsed_len, resp_len);
    return -ECOMM;
    } else if (rc) {
    return rc;
    }
    if (parsed_len != 1) {
    dev_err(occ.dev, "SRAM write response length invalid: %zd\n",
    parsed_len);
    rc = -EBADMSG;
    } else {
    resp_data_len = be32_to_cpu(buf[0]);
    if (resp_data_len != data_len) {
    dev_err(occ.dev,
    "SRAM write expected %d bytes got %zd\n",
    data_len, resp_data_len);
    rc = -EBADMSG;
    }
    }
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn occ_trigger_attn(occ: *mut occ) -> c_int {
    static int occ_trigger_attn(struct occ *occ)
    {
    __be32 *buf = occ.buffer;
    size_t cmd_len, parsed_len, resp_data_len;
    let mut resp_len: usize = OCC_MAX_RESP_WORDS;
    let mut idx: c_int = 0, rc;
    switch (occ.version) {
    default:
    case occ_p9:
    cmd_len = 7;
    buf[2] = cpu_to_be32(3); /* Circular mode */
    buf[3] = 0;
    break;
    case occ_p10:
    idx = 1;
    cmd_len = 8;
    buf[2] = cpu_to_be32(0xd0); /* Circular mode, OCB Channel 1 */
    buf[3] = 0;
    buf[4] = 0;
    break;
    }
    buf[0] = cpu_to_be32(cmd_len);		/* Chip-op length in words */
    buf[1] = cpu_to_be32(SBEFIFO_CMD_PUT_OCC_SRAM);
    buf[4 + idx] = cpu_to_be32(8);		/* Data length in bytes */
    buf[5 + idx] = cpu_to_be32(0x20010000);	/* Trigger OCC attention */
    buf[6 + idx] = 0;
    rc = sbefifo_submit(occ.sbefifo, buf, cmd_len, buf, &resp_len);
    if (rc)
    return rc;
    rc = sbefifo_parse_status(occ.sbefifo, SBEFIFO_CMD_PUT_OCC_SRAM,
    buf, resp_len, &parsed_len);
    if (rc > 0) {
    dev_err(occ.dev, "SRAM attn returned failure status: %08x\n",
    rc);
    occ_save_ffdc(occ, buf, parsed_len, resp_len);
    return -ECOMM;
    } else if (rc) {
    return rc;
    }
    if (parsed_len != 1) {
    dev_err(occ.dev, "SRAM attn response length invalid: %zd\n",
    parsed_len);
    rc = -EBADMSG;
    } else {
    resp_data_len = be32_to_cpu(buf[0]);
    if (resp_data_len != 8) {
    dev_err(occ.dev,
    "SRAM attn expected 8 bytes got %zd\n",
    resp_data_len);
    rc = -EBADMSG;
    }
    }
    return rc;
    }
    static bool fsi_occ_response_not_ready(struct occ_response *resp, u8 seq_no,
    u8 cmd_type)
    {
    return resp.return_status == OCC_RESP_CMD_IN_PRG ||
    resp.return_status == OCC_RESP_CRIT_INIT ||
    resp.seq_no != seq_no || resp.cmd_type != cmd_type;
    }
    int fsi_occ_submit(struct device *dev, const void *request, size_t req_len,
    void *response, size_t *resp_len)
    {
    let mut timeout: c_ulong = msecs_to_jiffies(OCC_TIMEOUT_MS);
    const unsigned long wait_time =
    msecs_to_jiffies(OCC_CMD_IN_PRG_WAIT_MS);
    struct occ *occ = dev_get_drvdata(dev);
    struct occ_response *resp = response;
    let mut user_resp_len: usize = *resp_len;
    u8 seq_no;
    u8 cmd_type;
    let mut checksum: u16 = 0;
    u16 resp_data_length;
    const u8 *byte_request = (const u8 *)request;
    unsigned long end;
    int rc;
    size_t i;
// resp_len = 0;
    if (!occ)
    return -ENODEV;
    if (user_resp_len < 7) {
    dev_dbg(dev, "Bad resplen %zd\n", user_resp_len);
    return -EINVAL;
    }
    cmd_type = byte_request[1];
// Checksum the request, ignoring first byte (sequence number).
    for (i = 1; i < req_len - 2; ++i)
    checksum += byte_request[i];
    rc = mutex_lock_interruptible(&occ.occ_lock);
    if (rc)
    return rc;
    occ.client_buffer = response;
    occ.client_buffer_size = user_resp_len;
    occ.client_response_size = 0;
    if (!occ.buffer) {
    rc = -ENOENT;
    goto done;
    }
//
// Get a sequence number and update the counter. Avoid a sequence
// number of 0 which would pass the response check below even if the
// OCC response is uninitialized. Any sequence number the user is
// trying to send is overwritten since this function is the only common
// interface to the OCC and therefore the only place we can guarantee
// unique sequence numbers.
//
    seq_no = occ.sequence_number++;
    if (!occ.sequence_number)
    occ.sequence_number = 1;
    checksum += seq_no;
    rc = occ_putsram(occ, request, req_len, seq_no, checksum);
    if (rc)
    goto done;
    rc = occ_trigger_attn(occ);
    if (rc)
    goto done;
    end = jiffies + timeout;
    while (true) {
// Read occ response header
    rc = occ_getsram(occ, 0, resp, 8);
    if (rc)
    goto done;
    if (fsi_occ_response_not_ready(resp, seq_no, cmd_type)) {
    if (time_after(jiffies, end)) {
    dev_err(occ.dev,
    "resp timeout status=%02x seq=%d cmd=%d, our seq=%d cmd=%d\n",
    resp.return_status, resp.seq_no,
    resp.cmd_type, seq_no, cmd_type);
    rc = -ETIMEDOUT;
    goto done;
    }
    set_current_state(TASK_UNINTERRUPTIBLE);
    schedule_timeout(wait_time);
    } else {
// Extract size of response data
    resp_data_length =
    get_unaligned_be16(&resp.data_length);
//
// Message size is data length + 5 bytes header + 2
// bytes checksum
//
    if ((resp_data_length + 7) > user_resp_len) {
    rc = -EMSGSIZE;
    goto done;
    }
//
// Get the entire response including the header again,
// in case it changed
//
    if (resp_data_length > 1) {
    rc = occ_getsram(occ, 0, resp,
    resp_data_length + 7);
    if (rc)
    goto done;
    if (!fsi_occ_response_not_ready(resp, seq_no,
    cmd_type))
    break;
    } else {
    break;
    }
    }
    }
    dev_dbg(dev, "resp_status=%02x resp_data_len=%d\n",
    resp.return_status, resp_data_length);
    rc = occ_verify_checksum(occ, resp, resp_data_length);
    if (rc)
    goto done;
    occ.client_response_size = resp_data_length + 7;
    done:
// resp_len = occ->client_response_size;
    mutex_unlock(&occ.occ_lock);
    return rc;
    }
    EXPORT_SYMBOL_GPL(fsi_occ_submit);
#[no_mangle]
unsafe extern "C" fn occ_unregister_platform_child(dev: *mut device, data: *mut c_void) -> c_int {
    static int occ_unregister_platform_child(struct device *dev, void *data)
    {
    struct platform_device *hwmon_dev = to_platform_device(dev);
    platform_device_unregister(hwmon_dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn occ_unregister_of_child(dev: *mut device, data: *mut c_void) -> c_int {
    static int occ_unregister_of_child(struct device *dev, void *data)
    {
    struct platform_device *hwmon_dev = to_platform_device(dev);
    of_device_unregister(hwmon_dev);
    if (dev.of_node)
    of_node_clear_flag(dev.of_node, OF_POPULATED);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn occ_probe(pdev: *mut platform_device) -> c_int {
    static int occ_probe(struct platform_device *pdev)
    {
    int rc;
    u32 reg;
    char child_name[32];
    struct occ *occ;
    struct platform_device *hwmon_dev = core::ptr::null_mut();
    struct device_node *hwmon_node;
    struct device *dev = &pdev.dev;
    struct platform_device_info hwmon_dev_info = {
    .parent = dev,
    .name = "occ-hwmon",
    };
    occ = devm_kzalloc(dev, sizeof(*occ), GFP_KERNEL);
    if (!occ)
    return -ENOMEM;
// SBE words are always four bytes
    occ.buffer = kvmalloc(OCC_MAX_RESP_WORDS * 4, GFP_KERNEL);
    if (!occ.buffer)
    return -ENOMEM;
    occ.version = (uintptr_t)of_device_get_match_data(dev);
    occ.dev = dev;
    occ.sbefifo = dev.parent;
//
// Quickly derive a pseudo-random number from jiffies so that
// re-probing the driver doesn't accidentally overlap sequence numbers.
//
    occ.sequence_number = (u8)((jiffies % 0xff) + 1);
    mutex_init(&occ.occ_lock);
    if (dev.of_node) {
    rc = of_property_read_u32(dev.of_node, "reg", &reg);
    if (!rc) {
// make sure we don't have a duplicate from dts
    occ.idx = ida_alloc_range(&occ_ida, reg, reg,
    GFP_KERNEL);
    if (occ.idx < 0)
    occ.idx = ida_alloc_min(&occ_ida, 1,
    GFP_KERNEL);
    } else {
    occ.idx = ida_alloc_min(&occ_ida, 1, GFP_KERNEL);
    }
    } else {
    occ.idx = ida_alloc_min(&occ_ida, 1, GFP_KERNEL);
    }
    platform_set_drvdata(pdev, occ);
    snprintf(occ.name, sizeof(occ.name), "occ%d", occ.idx);
    occ.mdev.fops = &occ_fops;
    occ.mdev.minor = MISC_DYNAMIC_MINOR;
    occ.mdev.name = occ.name;
    occ.mdev.parent = dev;
    rc = misc_register(&occ.mdev);
    if (rc) {
    dev_err(dev, "failed to register miscdevice: %d\n", rc);
    ida_free(&occ_ida, occ.idx);
    kvfree(occ.buffer);
    return rc;
    }
    hwmon_node = of_get_child_by_name(dev.of_node, hwmon_dev_info.name);
    if (hwmon_node) {
    snprintf(child_name, sizeof(child_name), "%s.%d", hwmon_dev_info.name, occ.idx);
    hwmon_dev = of_platform_device_create(hwmon_node, child_name, dev);
    of_node_put(hwmon_node);
    }
    if (!hwmon_dev) {
    occ.platform_hwmon = true;
    hwmon_dev_info.id = occ.idx;
    hwmon_dev = platform_device_register_full(&hwmon_dev_info);
    if (IS_ERR(hwmon_dev))
    dev_warn(dev, "failed to create hwmon device\n");
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn occ_remove(pdev: *mut platform_device) {
    static void occ_remove(struct platform_device *pdev)
    {
    struct occ *occ = platform_get_drvdata(pdev);
    misc_deregister(&occ.mdev);
    mutex_lock(&occ.occ_lock);
    kvfree(occ.buffer);
    occ.buffer = core::ptr::null_mut();
    mutex_unlock(&occ.occ_lock);
    if (occ.platform_hwmon)
    device_for_each_child(&pdev.dev, core::ptr::null_mut(), occ_unregister_platform_child);
    else
    device_for_each_child(&pdev.dev, core::ptr::null_mut(), occ_unregister_of_child);
    ida_free(&occ_ida, occ.idx);
    }
    static const struct of_device_id occ_match[] = {
    {
    .compatible = "ibm,p9-occ",
    .data = (void *)occ_p9
    },
    {
    .compatible = "ibm,p10-occ",
    .data = (void *)occ_p10
    },
    { },
    };
    MODULE_DEVICE_TABLE(of, occ_match);
    static struct platform_driver occ_driver = {
    .driver = {
    .name = "occ",
    .of_match_table	= occ_match,
    },
    .probe	= occ_probe,
    .remove = occ_remove,
    };
#[no_mangle]
unsafe extern "C" fn occ_init() -> c_int {
    static int occ_init(void)
    {
    return platform_driver_register(&occ_driver);
    }
#[no_mangle]
unsafe extern "C" fn occ_exit() {
    static void occ_exit(void)
    {
    platform_driver_unregister(&occ_driver);
    ida_destroy(&occ_ida);
    }
    module_init(occ_init);
    module_exit(occ_exit);
    MODULE_AUTHOR("Eddie James <eajames@linux.ibm.com>");
    MODULE_DESCRIPTION("BMC P9 OCC driver");
    MODULE_LICENSE("GPL");
