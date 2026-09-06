//! Automatically rewritten from C to Rust
//! Source: drivers/platform/x86/amd/hsmp/plat.c
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
// AMD HSMP Platform Driver
// Copyright (c) 2024, AMD.
// All Rights Reserved.
//
// This file provides platform device implementations.
//

//
// To access specific HSMP mailbox register, s/w writes the SMN address of HSMP mailbox
// register into the SMN_INDEX register, and reads/writes the SMN_DATA reg.
// Below are required SMN address for HSMP Mailbox register offsets in SMU address space
//
pub const SMN_HSMP_BASE: c_uint = 0x3B00000;
pub const SMN_HSMP_MSG_ID: c_uint = 0x0010534;
pub const SMN_HSMP_MSG_ID_F1A_M0H: c_uint = 0x0010934;
pub const SMN_HSMP_MSG_RESP: c_uint = 0x0010980;
pub const SMN_HSMP_MSG_DATA: c_uint = 0x00109E0;
    static struct hsmp_plat_device *hsmp_pdev;
    static int amd_hsmp_pci_rdwr(struct hsmp_socket *sock, u32 offset,
    u32 *value, bool write)
    {
    return amd_smn_hsmp_rdwr(sock.sock_ind, sock.mbinfo.base_addr + offset, value, write);
    }
    static ssize_t hsmp_metric_tbl_plat_read(struct file *filp, struct kobject *kobj,
    const struct bin_attribute *bin_attr, char *buf,
    loff_t off, size_t count)
    {
    struct hsmp_socket *sock;
    u16 sock_ind;
    sock_ind = (uintptr_t)bin_attr.private;
    if (sock_ind >= hsmp_pdev.num_sockets)
    return -EINVAL;
    sock = &hsmp_pdev.sock[sock_ind];
    return hsmp_metric_tbl_read(sock, buf, count);
    }
    static umode_t hsmp_is_sock_attr_visible(struct kobject *kobj,
    const struct bin_attribute *battr, int id)
    {
    u16 sock_ind;
    sock_ind = (uintptr_t)battr.private;
    if (id == 0 && sock_ind >= hsmp_pdev.num_sockets)
    return SYSFS_GROUP_INVISIBLE;
    if (hsmp_pdev.proto_ver == HSMP_PROTO_VER6)
    return battr.attr.mode;
    return 0;
    }
//
// AMD supports maximum of 8 sockets in a system.
// Static array of 8 + 1(for NULL) elements is created below
// to create sysfs groups for sockets.
// is_bin_visible function is used to show / hide the necessary groups.
//
// Validate the maximum number against MAX_AMD_NUM_NODES. If this changes,
// then the attributes and groups below must be adjusted.
//
    static_assert(MAX_AMD_NUM_NODES == 8);

    static const struct bin_attribute attr##index = {			\
    .attr = { .name = HSMP_METRICS_TABLE_NAME, .mode = 0444},	\
    .private = (void *)index,					\
    .read = hsmp_metric_tbl_plat_read,				\
    .size = sizeof(struct hsmp_metric_table),			\
    };									\
    static const struct bin_attribute _list[] = {				\
    &attr##index,							\
    core::ptr::null_mut()								\
    }
    HSMP_BIN_ATTR(0, *sock0_attr_list);
    HSMP_BIN_ATTR(1, *sock1_attr_list);
    HSMP_BIN_ATTR(2, *sock2_attr_list);
    HSMP_BIN_ATTR(3, *sock3_attr_list);
    HSMP_BIN_ATTR(4, *sock4_attr_list);
    HSMP_BIN_ATTR(5, *sock5_attr_list);
    HSMP_BIN_ATTR(6, *sock6_attr_list);
    HSMP_BIN_ATTR(7, *sock7_attr_list);

    static const struct attribute_group sock##index##_attr_grp = {	\
    .bin_attrs = _list,					\
    .is_bin_visible = hsmp_is_sock_attr_visible,		\
    .name = #_name,						\
    }
    HSMP_BIN_ATTR_GRP(0, sock0_attr_list, socket0);
    HSMP_BIN_ATTR_GRP(1, sock1_attr_list, socket1);
    HSMP_BIN_ATTR_GRP(2, sock2_attr_list, socket2);
    HSMP_BIN_ATTR_GRP(3, sock3_attr_list, socket3);
    HSMP_BIN_ATTR_GRP(4, sock4_attr_list, socket4);
    HSMP_BIN_ATTR_GRP(5, sock5_attr_list, socket5);
    HSMP_BIN_ATTR_GRP(6, sock6_attr_list, socket6);
    HSMP_BIN_ATTR_GRP(7, sock7_attr_list, socket7);
    static const struct attribute_group *hsmp_groups[] = {
    &sock0_attr_grp,
    &sock1_attr_grp,
    &sock2_attr_grp,
    &sock3_attr_grp,
    &sock4_attr_grp,
    &sock5_attr_grp,
    &sock6_attr_grp,
    &sock7_attr_grp,
    core::ptr::null_mut()
    };
#[no_mangle]
pub unsafe extern "C" fn is_f1a_m0h() -> bool {
    static inline bool is_f1a_m0h(void)
    {
    if (boot_cpu_data.x86 == 0x1A && boot_cpu_data.x86_model <= 0x0F)
    return true;
    return false;
    }
#[no_mangle]
unsafe extern "C" fn init_platform_device(dev: *mut device) -> c_int {
    static int init_platform_device(struct device *dev)
    {
    struct hsmp_socket *sock;
    int ret, i;
    for (i = 0; i < hsmp_pdev.num_sockets; i++) {
    sock = &hsmp_pdev.sock[i];
    sock.sock_ind			= i;
    sock.dev			= dev;
    sock.mbinfo.base_addr		= SMN_HSMP_BASE;
    sock.amd_hsmp_rdwr		= amd_hsmp_pci_rdwr;
//
// This is a transitional change from non-ACPI to ACPI, only
// family 0x1A, model 0x00 platform is supported for both ACPI and non-ACPI.
//
    if (is_f1a_m0h())
    sock.mbinfo.msg_id_off	= SMN_HSMP_MSG_ID_F1A_M0H;
    else
    sock.mbinfo.msg_id_off	= SMN_HSMP_MSG_ID;
    sock.mbinfo.msg_resp_off	= SMN_HSMP_MSG_RESP;
    sock.mbinfo.msg_arg_off	= SMN_HSMP_MSG_DATA;
    sema_init(&sock.hsmp_sem, 1);
// Test the hsmp interface on each socket
    ret = hsmp_test(i, 0xDEADBEEF);
    if (ret) {
    dev_err(dev, "HSMP test message failed on Fam:%x model:%x\n",
    boot_cpu_data.x86, boot_cpu_data.x86_model);
    dev_err(dev, "Is HSMP disabled in BIOS ?\n");
    return ret;
    }
    ret = hsmp_cache_proto_ver(i);
    if (ret) {
    dev_err(dev, "Failed to read HSMP protocol version\n");
    return ret;
    }
    if (hsmp_pdev.proto_ver == HSMP_PROTO_VER6) {
    ret = hsmp_get_tbl_dram_base(i);
    if (ret)
    dev_info(dev, "Failed to init metric table\n");
    }
// Register with hwmon interface for reporting power
    ret = hsmp_create_sensor(dev, i);
    if (ret)
    dev_info(dev, "Failed to register HSMP sensors with hwmon\n");
    }
    return 0;
    }
//
// The socket array is devm-managed and freed by the driver core, but the
// metric-table DRAM regions are mapped with plain ioremap() during probe and
// the per-socket mutexes need an explicit mutex_destroy(), neither of which
// devres covers.
//
// Take the data-plane rwsem for write to drain any in-flight
// hsmp_send_message(), unmap the metric tables, destroy the mutexes and drop
// the global socket pointer, all before devres frees the array. Registered as
// a devres action so it runs on both remove and probe failure.
//
#[no_mangle]
unsafe extern "C" fn hsmp_pltdrv_release(data: *mut c_void) {
    static void hsmp_pltdrv_release(void *data)
    {
    guard(rwsem_write)(&hsmp_sock_rwsem);
    hsmp_unmap_metric_tbls(hsmp_pdev);
    hsmp_destroy_metric_read_locks(hsmp_pdev);
    hsmp_pdev.sock = core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn hsmp_pltdrv_probe(pdev: *mut platform_device) -> c_int {
    static int hsmp_pltdrv_probe(struct platform_device *pdev)
    {
    int ret;
    hsmp_pdev.sock = devm_kcalloc(&pdev.dev, hsmp_pdev.num_sockets,
    sizeof(*hsmp_pdev.sock),
    GFP_KERNEL);
    if (!hsmp_pdev.sock)
    return -ENOMEM;
    hsmp_init_metric_read_locks(hsmp_pdev);
    ret = devm_add_action_or_reset(&pdev.dev, hsmp_pltdrv_release, core::ptr::null_mut());
    if (ret)
    return ret;
//
// init_platform_device() runs the mailbox handshake via the probe-only
// senders, which issue messages through hsmp_send_message_locked() and
// so require hsmp_sock_rwsem held. Hold it for write, matching probe's
// role as a socket bring-up path. The lock is not held across
// devm_add_action_or_reset() above so the release action, which also
// takes it for write, does not deadlock if that registration fails.
//
    scoped_guard(rwsem_write, &hsmp_sock_rwsem)
    ret = init_platform_device(&pdev.dev);
    if (ret) {
    dev_err(&pdev.dev, "Failed to init HSMP mailbox\n");
    return ret;
    }
    ret = hsmp_misc_register(&pdev.dev);
    if (ret) {
    dev_err(&pdev.dev, "Failed to register misc device\n");
    return ret;
    }
    dev_dbg(&pdev.dev, "AMD HSMP is probed successfully\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hsmp_pltdrv_remove(pdev: *mut platform_device) {
    static void hsmp_pltdrv_remove(struct platform_device *pdev)
    {
    hsmp_misc_deregister();
    }
    static struct platform_driver amd_hsmp_driver = {
    .probe		= hsmp_pltdrv_probe,
    .remove		= hsmp_pltdrv_remove,
    .driver		= {
    .name	= DRIVER_NAME,
    .dev_groups = hsmp_groups,
    },
    };
    static struct platform_device *amd_hsmp_platdev;
#[no_mangle]
unsafe extern "C" fn hsmp_plat_dev_register() -> c_int {
    static int hsmp_plat_dev_register(void)
    {
    int ret;
    amd_hsmp_platdev = platform_device_alloc(DRIVER_NAME, PLATFORM_DEVID_NONE);
    if (!amd_hsmp_platdev)
    return -ENOMEM;
    ret = platform_device_add(amd_hsmp_platdev);
    if (ret)
    platform_device_put(amd_hsmp_platdev);
    return ret;
    }
//
// This check is only needed for backward compatibility of previous platforms.
// All new platforms are expected to support ACPI based probing.
//
#[no_mangle]
unsafe extern "C" fn legacy_hsmp_support() -> bool {
    static bool legacy_hsmp_support(void)
    {
    if (boot_cpu_data.x86_vendor != X86_VENDOR_AMD)
    return false;
    switch (boot_cpu_data.x86) {
    case 0x19:
    switch (boot_cpu_data.x86_model) {
    case 0x00 ... 0x1F:
    case 0x30 ... 0x3F:
    case 0x90 ... 0x9F:
    case 0xA0 ... 0xAF:
    return true;
    default:
    return false;
    }
    case 0x1A:
    switch (boot_cpu_data.x86_model) {
    case 0x00 ... 0x0F:
    return true;
    default:
    return false;
    }
    default:
    return false;
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn hsmp_plt_init() -> int __init {
    static int __init hsmp_plt_init(void)
    {
    let mut ret: c_int = -ENODEV;
    if (acpi_dev_present(ACPI_HSMP_DEVICE_HID, core::ptr::null_mut(), -1)) {
    if (IS_ENABLED(CONFIG_AMD_HSMP_ACPI))
    pr_debug("HSMP is supported through ACPI on this platform, please use hsmp_acpi.ko\n");
    else
    pr_info("HSMP is supported through ACPI on this platform, please enable AMD_HSMP_ACPI config\n");
    return -ENODEV;
    }
    if (!legacy_hsmp_support()) {
    pr_info("HSMP interface is either disabled or not supported on family:%x model:%x\n",
    boot_cpu_data.x86, boot_cpu_data.x86_model);
    return ret;
    }
    hsmp_pdev = get_hsmp_pdev();
    if (!hsmp_pdev)
    return -ENOMEM;
//
// amd_num_nodes() returns number of SMN/DF interfaces present in the system
// if we have N SMN/DF interfaces that ideally means N sockets
//
    hsmp_pdev.num_sockets = amd_num_nodes();
    if (hsmp_pdev.num_sockets == 0 || hsmp_pdev.num_sockets > MAX_AMD_NUM_NODES) {
    pr_err("Wrong number of sockets\n");
    return ret;
    }
    ret = platform_driver_register(&amd_hsmp_driver);
    if (ret)
    return ret;
    ret = hsmp_plat_dev_register();
    if (ret)
    platform_driver_unregister(&amd_hsmp_driver);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn hsmp_plt_exit() -> void __exit {
    static void __exit hsmp_plt_exit(void)
    {
    platform_device_unregister(amd_hsmp_platdev);
    platform_driver_unregister(&amd_hsmp_driver);
    }
    device_initcall(hsmp_plt_init);
    module_exit(hsmp_plt_exit);
    MODULE_IMPORT_NS("AMD_HSMP");
    MODULE_DESCRIPTION("AMD HSMP Platform Interface Driver");
    MODULE_VERSION(DRIVER_VERSION);
    MODULE_LICENSE("GPL");
