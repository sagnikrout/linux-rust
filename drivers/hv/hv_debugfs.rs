//! Automatically rewritten from C to Rust
//! Source: drivers/hv/hv_debugfs.c
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
// Authors:
// Branden Bonaby <brandonbonaby94@gmail.com>
//

    static struct dentry *hv_debug_root;
#[no_mangle]
unsafe extern "C" fn hv_debugfs_delay_get(data: *mut c_void, val: *mut u64) -> c_int {
    static int hv_debugfs_delay_get(void *data, u64 *val)
    {
// val = *(u32 *)data;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hv_debugfs_delay_set(data: *mut c_void, val: u64) -> c_int {
    static int hv_debugfs_delay_set(void *data, u64 val)
    {
    if (val > 1000)
    return -EINVAL;
// (u32 *)data = val;
    return 0;
    }
    DEFINE_DEBUGFS_ATTRIBUTE(hv_debugfs_delay_fops, hv_debugfs_delay_get,
    hv_debugfs_delay_set, "%llu\n");
#[no_mangle]
unsafe extern "C" fn hv_debugfs_state_get(data: *mut c_void, val: *mut u64) -> c_int {
    static int hv_debugfs_state_get(void *data, u64 *val)
    {
// val = *(bool *)data;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hv_debugfs_state_set(data: *mut c_void, val: u64) -> c_int {
    static int hv_debugfs_state_set(void *data, u64 val)
    {
    if (val == 1)
// (bool *)data = true;
#[no_mangle]
pub unsafe extern "C" fn if(0: val ==) -> else {
    else if (val == 0)
// (bool *)data = false;
    else
    return -EINVAL;
    return 0;
    }
    DEFINE_DEBUGFS_ATTRIBUTE(hv_debugfs_state_fops, hv_debugfs_state_get,
    hv_debugfs_state_set, "%llu\n");
// Setup delay files to store test values
#[no_mangle]
unsafe extern "C" fn hv_debug_delay_files(dev: *mut hv_device, root: *mut dentry) -> c_int {
    static int hv_debug_delay_files(struct hv_device *dev, struct dentry *root)
    {
    struct vmbus_channel *channel = dev.channel;
    char *buffer = "fuzz_test_buffer_interrupt_delay";
    char *message = "fuzz_test_message_delay";
    int *buffer_val = &channel.fuzz_testing_interrupt_delay;
    int *message_val = &channel.fuzz_testing_message_delay;
    struct dentry *buffer_file, *message_file;
    buffer_file = debugfs_create_file(buffer, 0644, root,
    buffer_val,
    &hv_debugfs_delay_fops);
    if (IS_ERR(buffer_file)) {
    pr_debug("debugfs_hyperv: file %s not created\n", buffer);
    return PTR_ERR(buffer_file);
    }
    message_file = debugfs_create_file(message, 0644, root,
    message_val,
    &hv_debugfs_delay_fops);
    if (IS_ERR(message_file)) {
    pr_debug("debugfs_hyperv: file %s not created\n", message);
    return PTR_ERR(message_file);
    }
    return 0;
    }
// Setup test state value for vmbus device
#[no_mangle]
unsafe extern "C" fn hv_debug_set_test_state(dev: *mut hv_device, root: *mut dentry) -> c_int {
    static int hv_debug_set_test_state(struct hv_device *dev, struct dentry *root)
    {
    struct vmbus_channel *channel = dev.channel;
    bool *state = &channel.fuzz_testing_state;
    char *status = "fuzz_test_state";
    struct dentry *test_state;
    test_state = debugfs_create_file(status, 0644, root,
    state,
    &hv_debugfs_state_fops);
    if (IS_ERR(test_state)) {
    pr_debug("debugfs_hyperv: file %s not created\n", status);
    return PTR_ERR(test_state);
    }
    return 0;
    }
// Bind hv device to a dentry for debugfs
#[no_mangle]
unsafe extern "C" fn hv_debug_set_dir_dentry(dev: *mut hv_device, root: *mut dentry) {
    static void hv_debug_set_dir_dentry(struct hv_device *dev, struct dentry *root)
    {
    if (hv_debug_root)
    dev.debug_dir = root;
    }
// Create all test dentry's and names for fuzz testing
#[no_mangle]
pub unsafe extern "C" fn hv_debug_add_dev_dir(dev: *mut hv_device) -> c_int {
    int hv_debug_add_dev_dir(struct hv_device *dev)
    {
    const char *device = dev_name(&dev.device);
    char *delay_name = "delay";
    struct dentry *delay, *dev_root;
    int ret;
    if (!IS_ERR(hv_debug_root)) {
    dev_root = debugfs_create_dir(device, hv_debug_root);
    if (IS_ERR(dev_root)) {
    pr_debug("debugfs_hyperv: hyperv/%s/ not created\n",
    device);
    return PTR_ERR(dev_root);
    }
    hv_debug_set_test_state(dev, dev_root);
    hv_debug_set_dir_dentry(dev, dev_root);
    delay = debugfs_create_dir(delay_name, dev_root);
    if (IS_ERR(delay)) {
    pr_debug("debugfs_hyperv: hyperv/%s/%s/ not created\n",
    device, delay_name);
    return PTR_ERR(delay);
    }
    ret = hv_debug_delay_files(dev, delay);
    return ret;
    }
    pr_debug("debugfs_hyperv: hyperv/ not in root debugfs path\n");
    return PTR_ERR(hv_debug_root);
    }
// Remove dentry associated with released hv device
#[no_mangle]
pub unsafe extern "C" fn hv_debug_rm_dev_dir(dev: *mut hv_device) {
    void hv_debug_rm_dev_dir(struct hv_device *dev)
    {
    if (!IS_ERR(hv_debug_root))
    debugfs_remove_recursive(dev.debug_dir);
    }
// Remove all dentrys associated with vmbus testing
#[no_mangle]
pub unsafe extern "C" fn hv_debug_rm_all_dir() {
    void hv_debug_rm_all_dir(void)
    {
    debugfs_remove_recursive(hv_debug_root);
    }
// Delay buffer/message reads on a vmbus channel
#[no_mangle]
pub unsafe extern "C" fn hv_debug_delay_test(channel: *mut vmbus_channel, delay_type: enum delay) {
    void hv_debug_delay_test(struct vmbus_channel *channel, enum delay delay_type)
    {
    struct vmbus_channel *test_channel =    channel.primary_channel ?
    channel.primary_channel :
    channel;
    let mut state: bool = test_channel.fuzz_testing_state;
    if (state) {
    if (delay_type == 0)
    udelay(test_channel.fuzz_testing_interrupt_delay);
    else
    udelay(test_channel.fuzz_testing_message_delay);
    }
    }
// Initialize top dentry for vmbus testing
#[no_mangle]
pub unsafe extern "C" fn hv_debug_init() -> c_int {
    int hv_debug_init(void)
    {
    hv_debug_root = debugfs_create_dir("hyperv", core::ptr::null_mut());
    if (IS_ERR(hv_debug_root)) {
    pr_debug("debugfs_hyperv: hyperv/ not created\n");
    return PTR_ERR(hv_debug_root);
    }
    return 0;
    }
