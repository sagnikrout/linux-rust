//! Automatically rewritten from C to Rust
//! Source: drivers/platform/chrome/wilco_ec/debugfs.c
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
// debugfs attributes for Wilco EC
//
// Copyright 2019 Google LLC
//
// See Documentation/ABI/testing/debugfs-wilco-ec for usage.
//

// The raw bytes will take up more space when represented as a hex string

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wilco_ec_debugfs {
    pub ec: *mut wilco_ec_device,
    pub dir: *mut dentry,
    pub response_size: usize,
    pub raw_data: [u8; EC_MAILBOX_DATA_SIZE],
    pub formatted_data: [u8; FORMATTED_BUFFER_SIZE],
}

    static struct wilco_ec_debugfs *debug_info;
//
// parse_hex_sentence() - Convert a ascii hex representation into byte array.
// @in: Input buffer of ascii.
// @isize: Length of input buffer.
// @out: Output buffer.
// @osize: Length of output buffer, e.g. max number of bytes to parse.
//
// An valid input is a series of ascii hexadecimal numbers, separated by spaces.
// An example valid input is
// "   00 f2 0    000076 6 0  ff"
//
// If an individual "word" within the hex sentence is longer than MAX_WORD_SIZE,
// then the sentence is illegal, and parsing will fail.
//
// Return: Number of bytes parsed, or negative error code on failure.
//
#[no_mangle]
unsafe extern "C" fn parse_hex_sentence(in: *const c_char, isize: c_int, out: *mut u8, osize: c_int) -> c_int {
    static int parse_hex_sentence(const char *in, int isize, u8 *out, int osize)
    {
    let mut n_parsed: c_int = 0;
    let mut word_start: c_int = 0;
    int word_end;
    int word_len;
// Temp buffer for holding a "word" of chars that represents one byte
pub const MAX_WORD_SIZE: c_int = 16;
    char tmp[MAX_WORD_SIZE + 1];
    u8 byte;
    while (word_start < isize && n_parsed < osize) {
// Find the start of the next word
    while (word_start < isize && isspace(in[word_start]))
    word_start++;
// reached the end of the input before next word?
    if (word_start >= isize)
    break;
// Find the end of this word
    word_end = word_start;
    while (word_end < isize && !isspace(in[word_end]))
    word_end++;
// Copy to a tmp NULL terminated string
    word_len = word_end - word_start;
    if (word_len > MAX_WORD_SIZE)
    return -EINVAL;
    memcpy(tmp, in + word_start, word_len);
    tmp[word_len] = '\0';
//
// Convert from hex string, place in output. If fails to parse,
// just return -EINVAL because specific error code is only
// relevant for this one word, returning it would be confusing.
//
    if (kstrtou8(tmp, 16, &byte))
    return -EINVAL;
    out[n_parsed++] = byte;
    word_start = word_end;
    }
    return n_parsed;
    }
// The message type takes up two bytes

    static ssize_t raw_write(struct file *file, const char __user *user_buf,
    size_t count, loff_t *ppos)
    {
    char *buf = debug_info.formatted_data;
    struct wilco_ec_message msg;
    u8 request_data[TYPE_AND_DATA_SIZE];
    ssize_t kcount;
    int ret;
    if (count > FORMATTED_BUFFER_SIZE)
    return -EINVAL;
    kcount = simple_write_to_buffer(buf, FORMATTED_BUFFER_SIZE, ppos,
    user_buf, count);
    if (kcount < 0)
    return kcount;
    ret = parse_hex_sentence(buf, kcount, request_data, TYPE_AND_DATA_SIZE);
    if (ret < 0)
    return ret;
// Need at least two bytes for message type and one byte of data
    if (ret < 3)
    return -EINVAL;
    msg.type = request_data[0] << 8 | request_data[1];
    msg.flags = 0;
    msg.request_data = request_data + 2;
    msg.request_size = ret - 2;
    memset(debug_info.raw_data, 0, sizeof(debug_info.raw_data));
    msg.response_data = debug_info.raw_data;
    msg.response_size = EC_MAILBOX_DATA_SIZE;
    ret = wilco_ec_mailbox(debug_info.ec, &msg);
    if (ret < 0)
    return ret;
    debug_info.response_size = ret;
    return count;
    }
    static ssize_t raw_read(struct file *file, char __user *user_buf, size_t count,
    loff_t *ppos)
    {
    let mut fmt_len: c_int = 0;
    if (debug_info.response_size) {
    fmt_len = hex_dump_to_buffer(debug_info.raw_data,
    debug_info.response_size,
    16, 1, debug_info.formatted_data,
    sizeof(debug_info.formatted_data),
    true);
// Only return response the first time it is read
    debug_info.response_size = 0;
    }
    return simple_read_from_buffer(user_buf, count, ppos,
    debug_info.formatted_data, fmt_len);
    }
    static const struct file_operations fops_raw = {
    .owner = THIS_MODULE,
    .read = raw_read,
    .write = raw_write,
    };
pub const CMD_KB_CHROME: c_uint = 0x88;
pub const SUB_CMD_H1_GPIO: c_uint = 0x0A;
pub const SUB_CMD_TEST_EVENT: c_uint = 0x0B;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_request {
    pub /: *mut *mut u8 cmd; / Always CMD_KB_CHROME,
    pub reserved: u8,
    pub sub_cmd: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response {
    pub /: *mut *mut u8 status; / 0 if allowed,
    pub val: u8,
    pub __packed: },
#[no_mangle]
unsafe extern "C" fn send_ec_cmd(ec: *mut wilco_ec_device, sub_cmd: u8, out_val: *mut u8) -> c_int {
    static int send_ec_cmd(struct wilco_ec_device *ec, u8 sub_cmd, u8 *out_val)
    {
    pub rq: ec_request,
    pub rs: ec_response,
    pub msg: wilco_ec_message,
    pub ret: c_int,
    pub sizeof(rq)): memset(&rq, 0,,
    pub CMD_KB_CHROME: rq.cmd =,
    pub sub_cmd: rq.sub_cmd =,
    pub sizeof(msg)): memset(&msg, 0,,
    pub WILCO_EC_MSG_LEGACY: msg.type =,
    pub &rq: msg.request_data =,
    pub sizeof(rq): msg.request_size =,
    pub &rs: msg.response_data =,
    pub sizeof(rs): msg.response_size =,
    pub &msg): ret = wilco_ec_mailbox(ec,,
    if (ret < 0)
    pub ret: return,
    if (rs.status)
    pub -EIO: return,
// out_val = rs.val;
    pub 0: return,
    }
//
// h1_gpio_get() - Gets h1 gpio status.
// @arg: The wilco EC device.
// @val: BIT(0)=ENTRY_TO_FACT_MODE, BIT(1)=SPI_CHROME_SEL
//
#[no_mangle]
unsafe extern "C" fn h1_gpio_get(arg: *mut c_void, val: *mut u64) -> c_int {
    static int h1_gpio_get(void *arg, u64 *val)
    {
    pub ret: c_int,
    pub )val): *mut ret = send_ec_cmd(arg, SUB_CMD_H1_GPIO, (u8,
    if (ret == 0)
// val &= 0xFF;
    pub ret: return,
    }
    pub "0x%02llx\n"): DEFINE_DEBUGFS_ATTRIBUTE(fops_h1_gpio, h1_gpio_get, NULL,,
//
// test_event_set() - Sends command to EC to cause an EC test event.
// @arg: The wilco EC device.
// @val: unused.
//
#[no_mangle]
unsafe extern "C" fn test_event_set(arg: *mut c_void, val: u64) -> c_int {
    static int test_event_set(void *arg, u64 val)
    {
    pub ret: u8,
    pub &ret): return send_ec_cmd(arg, SUB_CMD_TEST_EVENT,,
    }
// Format is unused since it is only required for get method which is NULL
    pub "%llu\n"): DEFINE_DEBUGFS_ATTRIBUTE(fops_test_event, NULL, test_event_set,,
//
// wilco_ec_debugfs_probe() - Create the debugfs node
// @pdev: The platform device, probably created in core.c
//
// Try to create a debugfs node. If it fails, then we don't want to change
// behavior at all, this is for debugging after all. Just fail silently.
//
// Return: 0 always.
//
#[no_mangle]
unsafe extern "C" fn wilco_ec_debugfs_probe(pdev: *mut platform_device) -> c_int {
    static int wilco_ec_debugfs_probe(struct platform_device *pdev)
    {
    pub dev_get_drvdata(pdev->dev.parent): *mut *mut wilco_ec_device ec =,
    pub GFP_KERNEL): *mut *mut debug_info = devm_kzalloc(&pdev->dev, sizeof(debug_info),,
    if (!debug_info)
    pub 0: return,
    pub ec: debug_info->ec =,
    pub NULL): debug_info->dir = debugfs_create_dir("wilco_ec",,
    pub &fops_raw): debugfs_create_file("raw", 0644, debug_info->dir, NULL,,
    debugfs_create_file("h1_gpio", 0444, debug_info.dir, ec,
    debugfs_create_file("test_event", 0200, debug_info.dir, ec,
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn wilco_ec_debugfs_remove(pdev: *mut platform_device) {
    static void wilco_ec_debugfs_remove(struct platform_device *pdev)
    {
    }
    static const struct platform_device_id wilco_ec_debugfs_id[] = {
    { .name = DRV_NAME },
    { }
}

    MODULE_DEVICE_TABLE(platform, wilco_ec_debugfs_id);
    static struct platform_driver wilco_ec_debugfs_driver = {
    .driver = {
    .name = DRV_NAME,
    },
    .probe = wilco_ec_debugfs_probe,
    .remove = wilco_ec_debugfs_remove,
    .id_table = wilco_ec_debugfs_id,
    };
    module_platform_driver(wilco_ec_debugfs_driver);
    MODULE_AUTHOR("Nick Crews <ncrews@chromium.org>");
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("Wilco EC debugfs driver");
