//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/ibmaem.c
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
// A hwmon driver for the IBM System Director Active Energy Manager (AEM)
// temperature/power/energy sensors and capping functionality.
// Copyright (C) 2008 IBM
//
// Author: Darrick J. Wong <darrick.wong@oracle.com>
//

pub const AEM_NETFN: c_uint = 0x2E;
pub const AEM_FIND_FW_CMD: c_uint = 0x80;
pub const AEM_ELEMENT_CMD: c_uint = 0x81;
pub const AEM_FW_INSTANCE_CMD: c_uint = 0x82;
pub const AEM_READ_ELEMENT_CFG: c_uint = 0x80;
pub const AEM_READ_BUFFER: c_uint = 0x81;
pub const AEM_READ_REGISTER: c_uint = 0x82;
pub const AEM_WRITE_REGISTER: c_uint = 0x83;
pub const AEM_SET_REG_MASK: c_uint = 0x84;
pub const AEM_CLEAR_REG_MASK: c_uint = 0x85;
pub const AEM_READ_ELEMENT_CFG2: c_uint = 0x86;
pub const AEM_CONTROL_ELEMENT: c_int = 0;
pub const AEM_ENERGY_ELEMENT: c_int = 1;
pub const AEM_CLOCK_ELEMENT: c_int = 4;
pub const AEM_POWER_CAP_ELEMENT: c_int = 7;
pub const AEM_EXHAUST_ELEMENT: c_int = 9;
pub const AEM_POWER_ELEMENT: c_int = 10;
pub const AEM_MODULE_TYPE_ID: c_uint = 0x0001;
pub const AEM2_NUM_ENERGY_REGS: c_int = 2;
pub const AEM2_NUM_PCAP_REGS: c_int = 6;
pub const AEM2_NUM_TEMP_REGS: c_int = 2;
pub const AEM2_NUM_SENSORS: c_int = 14;
pub const AEM1_NUM_ENERGY_REGS: c_int = 1;
pub const AEM1_NUM_SENSORS: c_int = 3;
// AEM 2.x has more energy registers

// AEM 2.x needs more sensor files

pub const POWER_CAP: c_int = 0;
pub const POWER_CAP_MAX_HOTPLUG: c_int = 1;
pub const POWER_CAP_MAX: c_int = 2;
pub const POWER_CAP_MIN_WARNING: c_int = 3;
pub const POWER_CAP_MIN: c_int = 4;
pub const POWER_AUX: c_int = 5;
pub const AEM_DEFAULT_POWER_INTERVAL: c_int = 1000;
pub const AEM_MIN_POWER_INTERVAL: c_int = 200;

    static DEFINE_IDA(aem_ida);
    static struct platform_driver aem_driver = {
    .driver = {
    .name = DRVNAME,
    .bus = &platform_bus_type,
    }
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aem_ipmi_data {
    pub read_complete: completion,
    pub address: ipmi_addr,
    pub user: *mut ipmi_user,
    pub interface: c_int,
    pub tx_message: kernel_ipmi_msg,
    pub tx_msgid: c_long,
    pub rx_msg_data: *mut c_void,
    pub rx_msg_len: c_ushort,
    pub rx_result: c_uchar,
    pub rx_recv_type: c_int,
    pub bmc_device: *mut device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aem_ro_sensor_template {
    pub label: *mut c_char,
    ssize_t (*show)(struct device *dev,
    struct device_attribute *devattr,
    pub buf): *mut c_char,
    pub index: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aem_rw_sensor_template {
    pub label: *mut c_char,
    ssize_t (*show)(struct device *dev,
    struct device_attribute *devattr,
    pub buf): *mut c_char,
    ssize_t (*set)(struct device *dev,
    struct device_attribute *devattr,
    pub count): *const *const char buf, size_t,
    pub index: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aem_data {
    pub list: list_head,
    pub hwmon_dev: *mut device,
    pub pdev: *mut platform_device,
    pub lock: mutex,
    pub valid: bool,
    pub /: *mut *mut unsigned long last_updated; / In jiffies,
    pub ver_major: u8,
    pub ver_minor: u8,
    pub module_handle: u8,
    pub id: c_int,
    pub ipmi: aem_ipmi_data,
// Function and buffer to update sensors
    pub data): *mut *mut void (update)(struct aem_data,
    pub rs_resp: *mut aem_read_sensor_resp,
//
// AEM 1.x sensors:
// Available sensors:
// Energy meter
// Power meter
//
// AEM 2.x sensors:
// Two energy meters
// Two power meters
// Two temperature sensors
// Six power cap registers
//
// sysfs attrs
    pub sensors: [sensor_device_attribute; AEM_NUM_SENSORS],
// energy use in mJ
    pub energy: [u64; AEM_NUM_ENERGY_REGS],
// power sampling interval in ms
    pub power_period: [c_ulong; AEM_NUM_ENERGY_REGS],
// Everything past here is for AEM2 only
// power caps in dW
    pub pcap: [u16; AEM2_NUM_PCAP_REGS],
// exhaust temperature in C
    pub temp: [u8; AEM2_NUM_TEMP_REGS],
}

// Data structures returned by the AEM firmware
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aem_iana_id {
    pub bytes: [u8; 3],
}

    static struct aem_iana_id system_x_id = {
    .bytes = {0x4D, 0x4F, 0x00}
    };
// These are used to find AEM1 instances
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aem_find_firmware_req {
    pub id: aem_iana_id,
    pub rsvd: u8,
    pub index: __be16,
    pub module_type_id: __be16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aem_find_firmware_resp {
    pub id: aem_iana_id,
    pub num_instances: u8,
    pub __packed: },
// These are used to find AEM2 instances
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aem_find_instance_req {
    pub id: aem_iana_id,
    pub instance_number: u8,
    pub module_type_id: __be16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aem_find_instance_resp {
    pub id: aem_iana_id,
    pub num_instances: u8,
    pub major: u8,
    pub minor: u8,
    pub module_handle: u8,
    pub record_id: u16,
    pub __packed: },
// These are used to query sensors
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aem_read_sensor_req {
    pub id: aem_iana_id,
    pub module_handle: u8,
    pub element: u8,
    pub subcommand: u8,
    pub reg: u8,
    pub rx_buf_size: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aem_read_sensor_resp {
    pub id: aem_iana_id,
    pub bytes: [u8; ],
    pub __packed: },
// Data structures to talk to the IPMI layer
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aem_driver_data {
    pub aem_devices: list_head,
    pub bmc_events: ipmi_smi_watcher,
    pub ipmi_hndlrs: ipmi_user_hndl,
}

    static void aem_register_bmc(int iface, struct device *dev);
    static void aem_bmc_gone(int iface);
    static void aem_msg_handler(struct ipmi_recv_msg *msg, void *user_msg_data);
    static void aem_remove_sensors(struct aem_data *data);
    static int aem1_find_sensors(struct aem_data *data);
    static int aem2_find_sensors(struct aem_data *data);
    static void update_aem1_sensors(struct aem_data *data);
    static void update_aem2_sensors(struct aem_data *data);
    static struct aem_driver_data driver_data = {
    .aem_devices = LIST_HEAD_INIT(driver_data.aem_devices),
    .bmc_events = {
    .owner = THIS_MODULE,
    .new_smi = aem_register_bmc,
    .smi_gone = aem_bmc_gone,
    },
    .ipmi_hndlrs = {
    .ipmi_recv_hndl = aem_msg_handler,
    },
    };
// Functions to talk to the IPMI layer
// Initialize IPMI address, message buffers and user data
    static int aem_init_ipmi_data(struct aem_ipmi_data *data, int iface,
    struct device *bmc)
    {
    int err;
    init_completion(&data.read_complete);
    data.bmc_device = bmc;
// Initialize IPMI address
    data.address.addr_type = IPMI_SYSTEM_INTERFACE_ADDR_TYPE;
    data.address.channel = IPMI_BMC_CHANNEL;
    data.address.data[0] = 0;
    data.interface = iface;
// Initialize message buffers
    data.tx_msgid = 0;
    data.tx_message.netfn = AEM_NETFN;
// Create IPMI messaging interface user
    err = ipmi_create_user(data.interface, &driver_data.ipmi_hndlrs,
    data, &data.user);
    if (err < 0) {
    dev_err(bmc,
    "Unable to register user with IPMI interface %d\n",
    data.interface);
    return err;
    }
    return 0;
    }
// Send an IPMI command
#[no_mangle]
unsafe extern "C" fn aem_send_message(data: *mut aem_ipmi_data) -> c_int {
    static int aem_send_message(struct aem_ipmi_data *data)
    {
    int err;
    err = ipmi_validate_addr(&data.address, sizeof(data.address));
    if (err)
    goto out;
    data.tx_msgid++;
    err = ipmi_request_settime(data.user, &data.address, data.tx_msgid,
    &data.tx_message, data, 0, 0, 0);
    if (err)
    goto out1;
    return 0;
    out1:
    dev_err(data.bmc_device, "request_settime=%x\n", err);
    return err;
    out:
    dev_err(data.bmc_device, "validate_addr=%x\n", err);
    return err;
    }
// Dispatch IPMI messages to callers
#[no_mangle]
unsafe extern "C" fn aem_msg_handler(msg: *mut ipmi_recv_msg, user_msg_data: *mut c_void) {
    static void aem_msg_handler(struct ipmi_recv_msg *msg, void *user_msg_data)
    {
    unsigned short rx_len;
    struct aem_ipmi_data *data = user_msg_data;
    if (msg.msgid != data.tx_msgid) {
    dev_err(data.bmc_device,
    "Mismatch between received msgid (%02x) and transmitted msgid (%02x)!\n",
    (int)msg.msgid,
    (int)data.tx_msgid);
    ipmi_free_recv_msg(msg);
    return;
    }
    data.rx_recv_type = msg.recv_type;
    if (msg.msg.data_len > 0)
    data.rx_result = msg.msg.data[0];
    else
    data.rx_result = IPMI_UNKNOWN_ERR_COMPLETION_CODE;
    if (msg.msg.data_len > 1) {
    rx_len = msg.msg.data_len - 1;
    if (data.rx_msg_len < rx_len)
    rx_len = data.rx_msg_len;
    data.rx_msg_len = rx_len;
    memcpy(data.rx_msg_data, msg.msg.data + 1, data.rx_msg_len);
    } else
    data.rx_msg_len = 0;
    ipmi_free_recv_msg(msg);
    complete(&data.read_complete);
    }
// Sensor support functions
// Read a sensor value; must be called with data->lock held
    static int aem_read_sensor(struct aem_data *data, u8 elt, u8 reg,
    void *buf, size_t size)
    {
    int rs_size;
    struct aem_read_sensor_req rs_req;
// Use preallocated rx buffer
    struct aem_read_sensor_resp *rs_resp = data.rs_resp;
    struct aem_ipmi_data *ipmi = &data.ipmi;
// AEM registers are 1, 2, 4 or 8 bytes
    switch (size) {
    case 1:
    case 2:
    case 4:
    case 8:
    break;
    default:
    return -EINVAL;
    }
    rs_req.id = system_x_id;
    rs_req.module_handle = data.module_handle;
    rs_req.element = elt;
    rs_req.subcommand = AEM_READ_REGISTER;
    rs_req.reg = reg;
    rs_req.rx_buf_size = size;
    ipmi.tx_message.cmd = AEM_ELEMENT_CMD;
    ipmi.tx_message.data = (char *)&rs_req;
    ipmi.tx_message.data_len = sizeof(rs_req);
    rs_size = sizeof(*rs_resp) + size;
    ipmi.rx_msg_data = rs_resp;
    ipmi.rx_msg_len = rs_size;
    aem_send_message(ipmi);
    if (!wait_for_completion_timeout(&ipmi.read_complete, IPMI_TIMEOUT))
    return -ETIMEDOUT;
    if (ipmi.rx_result || ipmi.rx_msg_len != rs_size ||
    memcmp(&rs_resp.id, &system_x_id, sizeof(system_x_id)))
    return -ENOENT;
    switch (size) {
    case 1: {
    u8 *x = buf;
// x = rs_resp->bytes[0];
    break;
    }
    case 2: {
    u16 *x = buf;
// x = be16_to_cpup((__be16 *)rs_resp->bytes);
    break;
    }
    case 4: {
    u32 *x = buf;
// x = be32_to_cpup((__be32 *)rs_resp->bytes);
    break;
    }
    case 8: {
    u64 *x = buf;
// x = be64_to_cpup((__be64 *)rs_resp->bytes);
    break;
    }
    }
    return 0;
    }
// Update AEM energy registers
#[no_mangle]
unsafe extern "C" fn update_aem_energy_one(data: *mut aem_data, which: c_int) {
    static void update_aem_energy_one(struct aem_data *data, int which)
    {
    aem_read_sensor(data, AEM_ENERGY_ELEMENT, which,
    &data.energy[which], 8);
    }
#[no_mangle]
unsafe extern "C" fn update_aem_energy(data: *mut aem_data) {
    static void update_aem_energy(struct aem_data *data)
    {
    update_aem_energy_one(data, 0);
    if (data.ver_major < 2)
    return;
    update_aem_energy_one(data, 1);
    }
// Update all AEM1 sensors
#[no_mangle]
unsafe extern "C" fn update_aem1_sensors(data: *mut aem_data) {
    static void update_aem1_sensors(struct aem_data *data)
    {
    mutex_lock(&data.lock);
    if (time_before(jiffies, data.last_updated + REFRESH_INTERVAL) &&
    data.valid)
    goto out;
    update_aem_energy(data);
    out:
    mutex_unlock(&data.lock);
    }
// Update all AEM2 sensors
#[no_mangle]
unsafe extern "C" fn update_aem2_sensors(data: *mut aem_data) {
    static void update_aem2_sensors(struct aem_data *data)
    {
    int i;
    mutex_lock(&data.lock);
    if (time_before(jiffies, data.last_updated + REFRESH_INTERVAL) &&
    data.valid)
    goto out;
    update_aem_energy(data);
    aem_read_sensor(data, AEM_EXHAUST_ELEMENT, 0, &data.temp[0], 1);
    aem_read_sensor(data, AEM_EXHAUST_ELEMENT, 1, &data.temp[1], 1);
    for (i = POWER_CAP; i <= POWER_AUX; i++)
    aem_read_sensor(data, AEM_POWER_CAP_ELEMENT, i,
    &data.pcap[i], 2);
    out:
    mutex_unlock(&data.lock);
    }
// Delete an AEM instance
#[no_mangle]
unsafe extern "C" fn aem_delete(data: *mut aem_data) {
    static void aem_delete(struct aem_data *data)
    {
    list_del(&data.list);
    aem_remove_sensors(data);
    kfree(data.rs_resp);
    hwmon_device_unregister(data.hwmon_dev);
    ipmi_destroy_user(data.ipmi.user);
    platform_set_drvdata(data.pdev, core::ptr::null_mut());
    platform_device_unregister(data.pdev);
    ida_free(&aem_ida, data.id);
    kfree(data);
    }
// Probe functions for AEM1 devices
// Retrieve version and module handle for an AEM1 instance
#[no_mangle]
unsafe extern "C" fn aem_find_aem1_count(data: *mut aem_ipmi_data) -> c_int {
    static int aem_find_aem1_count(struct aem_ipmi_data *data)
    {
    struct aem_find_firmware_req	ff_req;
    struct aem_find_firmware_resp	ff_resp;
    ff_req.id = system_x_id;
    ff_req.index = 0;
    ff_req.module_type_id = cpu_to_be16(AEM_MODULE_TYPE_ID);
    data.tx_message.cmd = AEM_FIND_FW_CMD;
    data.tx_message.data = (char *)&ff_req;
    data.tx_message.data_len = sizeof(ff_req);
    data.rx_msg_data = &ff_resp;
    data.rx_msg_len = sizeof(ff_resp);
    aem_send_message(data);
    if (!wait_for_completion_timeout(&data.read_complete, IPMI_TIMEOUT))
    return -ETIMEDOUT;
    if (data.rx_result || data.rx_msg_len != sizeof(ff_resp) ||
    memcmp(&ff_resp.id, &system_x_id, sizeof(system_x_id)))
    return -ENOENT;
    return ff_resp.num_instances;
    }
// Find and initialize one AEM1 instance
#[no_mangle]
unsafe extern "C" fn aem_init_aem1_inst(probe: *mut aem_ipmi_data, module_handle: u8) -> c_int {
    static int aem_init_aem1_inst(struct aem_ipmi_data *probe, u8 module_handle)
    {
    struct aem_data *data;
    int i;
    let mut res: c_int = -ENOMEM;
    data = kzalloc_obj(*data);
    if (!data)
    return res;
    mutex_init(&data.lock);
// Copy instance data
    data.ver_major = 1;
    data.ver_minor = 0;
    data.module_handle = module_handle;
    for (i = 0; i < AEM1_NUM_ENERGY_REGS; i++)
    data.power_period[i] = AEM_DEFAULT_POWER_INTERVAL;
// Create sub-device for this fw instance
    data.id = ida_alloc(&aem_ida, GFP_KERNEL);
    if (data.id < 0)
    goto id_err;
    data.pdev = platform_device_alloc(DRVNAME, data.id);
    if (!data.pdev)
    goto dev_err;
    data.pdev.dev.driver = &aem_driver.driver;
    res = platform_device_add(data.pdev);
    if (res)
    goto dev_add_err;
    platform_set_drvdata(data.pdev, data);
// Set up IPMI interface
    res = aem_init_ipmi_data(&data.ipmi, probe.interface,
    probe.bmc_device);
    if (res)
    goto ipmi_err;
// Register with hwmon
    data.hwmon_dev = hwmon_device_register(&data.pdev.dev);
    if (IS_ERR(data.hwmon_dev)) {
    dev_err(&data.pdev.dev,
    "Unable to register hwmon device for IPMI interface %d\n",
    probe.interface);
    res = PTR_ERR(data.hwmon_dev);
    goto hwmon_reg_err;
    }
    data.update = update_aem1_sensors;
    data.rs_resp = kzalloc(sizeof(*(data.rs_resp)) + 8, GFP_KERNEL);
    if (!data.rs_resp) {
    res = -ENOMEM;
    goto alloc_resp_err;
    }
// Find sensors
    res = aem1_find_sensors(data);
    if (res)
    goto sensor_err;
// Add to our list of AEM devices
    list_add_tail(&data.list, &driver_data.aem_devices);
    dev_info(data.ipmi.bmc_device, "Found AEM v%d.%d at 0x%X\n",
    data.ver_major, data.ver_minor,
    data.module_handle);
    return 0;
    sensor_err:
    kfree(data.rs_resp);
    alloc_resp_err:
    hwmon_device_unregister(data.hwmon_dev);
    hwmon_reg_err:
    ipmi_destroy_user(data.ipmi.user);
    ipmi_err:
    platform_set_drvdata(data.pdev, core::ptr::null_mut());
    platform_device_del(data.pdev);
    dev_add_err:
    platform_device_put(data.pdev);
    dev_err:
    ida_free(&aem_ida, data.id);
    id_err:
    kfree(data);
    return res;
    }
// Find and initialize all AEM1 instances
#[no_mangle]
unsafe extern "C" fn aem_init_aem1(probe: *mut aem_ipmi_data) {
    static void aem_init_aem1(struct aem_ipmi_data *probe)
    {
    int num, i, err;
    num = aem_find_aem1_count(probe);
    for (i = 0; i < num; i++) {
    err = aem_init_aem1_inst(probe, i);
    if (err) {
    dev_err(probe.bmc_device,
    "Error %d initializing AEM1 0x%X\n",
    err, i);
    }
    }
    }
// Probe functions for AEM2 devices
// Retrieve version and module handle for an AEM2 instance
    static int aem_find_aem2(struct aem_ipmi_data *data,
    struct aem_find_instance_resp *fi_resp,
    int instance_num)
    {
    struct aem_find_instance_req fi_req;
    fi_req.id = system_x_id;
    fi_req.instance_number = instance_num;
    fi_req.module_type_id = cpu_to_be16(AEM_MODULE_TYPE_ID);
    data.tx_message.cmd = AEM_FW_INSTANCE_CMD;
    data.tx_message.data = (char *)&fi_req;
    data.tx_message.data_len = sizeof(fi_req);
    data.rx_msg_data = fi_resp;
    data.rx_msg_len = sizeof(*fi_resp);
    aem_send_message(data);
    if (!wait_for_completion_timeout(&data.read_complete, IPMI_TIMEOUT))
    return -ETIMEDOUT;
    if (data.rx_result || data.rx_msg_len != sizeof(*fi_resp) ||
    memcmp(&fi_resp.id, &system_x_id, sizeof(system_x_id)) ||
    fi_resp.num_instances <= instance_num)
    return -ENOENT;
    return 0;
    }
// Find and initialize one AEM2 instance
    static int aem_init_aem2_inst(struct aem_ipmi_data *probe,
    struct aem_find_instance_resp *fi_resp)
    {
    struct aem_data *data;
    int i;
    let mut res: c_int = -ENOMEM;
    data = kzalloc_obj(*data);
    if (!data)
    return res;
    mutex_init(&data.lock);
// Copy instance data
    data.ver_major = fi_resp.major;
    data.ver_minor = fi_resp.minor;
    data.module_handle = fi_resp.module_handle;
    for (i = 0; i < AEM2_NUM_ENERGY_REGS; i++)
    data.power_period[i] = AEM_DEFAULT_POWER_INTERVAL;
// Create sub-device for this fw instance
    data.id = ida_alloc(&aem_ida, GFP_KERNEL);
    if (data.id < 0)
    goto id_err;
    data.pdev = platform_device_alloc(DRVNAME, data.id);
    if (!data.pdev)
    goto dev_err;
    data.pdev.dev.driver = &aem_driver.driver;
    res = platform_device_add(data.pdev);
    if (res)
    goto dev_add_err;
    platform_set_drvdata(data.pdev, data);
// Set up IPMI interface
    res = aem_init_ipmi_data(&data.ipmi, probe.interface,
    probe.bmc_device);
    if (res)
    goto ipmi_err;
// Register with hwmon
    data.hwmon_dev = hwmon_device_register(&data.pdev.dev);
    if (IS_ERR(data.hwmon_dev)) {
    dev_err(&data.pdev.dev,
    "Unable to register hwmon device for IPMI interface %d\n",
    probe.interface);
    res = PTR_ERR(data.hwmon_dev);
    goto hwmon_reg_err;
    }
    data.update = update_aem2_sensors;
    data.rs_resp = kzalloc(sizeof(*(data.rs_resp)) + 8, GFP_KERNEL);
    if (!data.rs_resp) {
    res = -ENOMEM;
    goto alloc_resp_err;
    }
// Find sensors
    res = aem2_find_sensors(data);
    if (res)
    goto sensor_err;
// Add to our list of AEM devices
    list_add_tail(&data.list, &driver_data.aem_devices);
    dev_info(data.ipmi.bmc_device, "Found AEM v%d.%d at 0x%X\n",
    data.ver_major, data.ver_minor,
    data.module_handle);
    return 0;
    sensor_err:
    kfree(data.rs_resp);
    alloc_resp_err:
    hwmon_device_unregister(data.hwmon_dev);
    hwmon_reg_err:
    ipmi_destroy_user(data.ipmi.user);
    ipmi_err:
    platform_set_drvdata(data.pdev, core::ptr::null_mut());
    platform_device_del(data.pdev);
    dev_add_err:
    platform_device_put(data.pdev);
    dev_err:
    ida_free(&aem_ida, data.id);
    id_err:
    kfree(data);
    return res;
    }
// Find and initialize all AEM2 instances
#[no_mangle]
unsafe extern "C" fn aem_init_aem2(probe: *mut aem_ipmi_data) {
    static void aem_init_aem2(struct aem_ipmi_data *probe)
    {
    struct aem_find_instance_resp fi_resp;
    int err;
    let mut i: c_int = 0;
    while (!aem_find_aem2(probe, &fi_resp, i)) {
    if (fi_resp.major != 2) {
    dev_err(probe.bmc_device,
    "Unknown AEM v%d; please report this to the maintainer.\n",
    fi_resp.major);
    i++;
    continue;
    }
    err = aem_init_aem2_inst(probe, &fi_resp);
    if (err) {
    dev_err(probe.bmc_device,
    "Error %d initializing AEM2 0x%X\n",
    err, fi_resp.module_handle);
    }
    i++;
    }
    }
// Probe a BMC for AEM firmware instances
#[no_mangle]
unsafe extern "C" fn aem_register_bmc(iface: c_int, dev: *mut device) {
    static void aem_register_bmc(int iface, struct device *dev)
    {
    struct aem_ipmi_data probe;
    if (aem_init_ipmi_data(&probe, iface, dev))
    return;
// Ignore probe errors; they won't cause problems
    aem_init_aem1(&probe);
    aem_init_aem2(&probe);
    ipmi_destroy_user(probe.user);
    }
// Handle BMC deletion
#[no_mangle]
unsafe extern "C" fn aem_bmc_gone(iface: c_int) {
    static void aem_bmc_gone(int iface)
    {
    struct aem_data *p1, *next1;
    list_for_each_entry_safe(p1, next1, &driver_data.aem_devices, list)
    if (p1.ipmi.interface == iface)
    aem_delete(p1);
    }
// sysfs support functions
// AEM device name
    static ssize_t name_show(struct device *dev, struct device_attribute *devattr,
    char *buf)
    {
    struct aem_data *data = dev_get_drvdata(dev);
    return sprintf(buf, "%s%d\n", DRVNAME, data.ver_major);
    }
    static SENSOR_DEVICE_ATTR_RO(name, name, 0);
// AEM device version
    static ssize_t version_show(struct device *dev,
    struct device_attribute *devattr, char *buf)
    {
    struct aem_data *data = dev_get_drvdata(dev);
    return sprintf(buf, "%d.%d\n", data.ver_major, data.ver_minor);
    }
    static SENSOR_DEVICE_ATTR_RO(version, version, 0);
// Display power use
    static ssize_t aem_show_power(struct device *dev,
    struct device_attribute *devattr,
    char *buf)
    {
    struct sensor_device_attribute *attr = to_sensor_dev_attr(devattr);
    struct aem_data *data = dev_get_drvdata(dev);
    u64 before, after, delta, time;
    signed long leftover;
    mutex_lock(&data.lock);
    update_aem_energy_one(data, attr.index);
    time = ktime_get_ns();
    before = data.energy[attr.index];
    leftover = schedule_timeout_interruptible(
    msecs_to_jiffies(data.power_period[attr.index])
    );
    if (leftover) {
    mutex_unlock(&data.lock);
    return 0;
    }
    update_aem_energy_one(data, attr.index);
    time = ktime_get_ns() - time;
    after = data.energy[attr.index];
    mutex_unlock(&data.lock);
    delta = (after - before) * UJ_PER_MJ;
    return sprintf(buf, "%llu\n",
    (unsigned long long)div64_u64(delta * NSEC_PER_SEC, time));
    }
// Display energy use
    static ssize_t aem_show_energy(struct device *dev,
    struct device_attribute *devattr,
    char *buf)
    {
    struct sensor_device_attribute *attr = to_sensor_dev_attr(devattr);
    struct aem_data *a = dev_get_drvdata(dev);
    mutex_lock(&a.lock);
    update_aem_energy_one(a, attr.index);
    mutex_unlock(&a.lock);
    return sprintf(buf, "%llu\n",
    (unsigned long long)a.energy[attr.index] * 1000);
    }
// Display power interval registers
    static ssize_t aem_show_power_period(struct device *dev,
    struct device_attribute *devattr,
    char *buf)
    {
    struct sensor_device_attribute *attr = to_sensor_dev_attr(devattr);
    struct aem_data *a = dev_get_drvdata(dev);
    a.update(a);
    return sprintf(buf, "%lu\n", a.power_period[attr.index]);
    }
// Set power interval registers
    static ssize_t aem_set_power_period(struct device *dev,
    struct device_attribute *devattr,
    const char *buf, size_t count)
    {
    struct sensor_device_attribute *attr = to_sensor_dev_attr(devattr);
    struct aem_data *a = dev_get_drvdata(dev);
    unsigned long temp;
    int res;
    res = kstrtoul(buf, 10, &temp);
    if (res)
    return res;
    if (temp < AEM_MIN_POWER_INTERVAL)
    return -EINVAL;
    mutex_lock(&a.lock);
    a.power_period[attr.index] = temp;
    mutex_unlock(&a.lock);
    return count;
    }
// Discover sensors on an AEM device
    static int aem_register_sensors(struct aem_data *data,
    const struct aem_ro_sensor_template *ro,
    const struct aem_rw_sensor_template *rw)
    {
    struct device *dev = &data.pdev.dev;
    struct sensor_device_attribute *sensors = data.sensors;
    int err;
// Set up read-only sensors
    while (ro.label) {
    sysfs_attr_init(&sensors.dev_attr.attr);
    sensors.dev_attr.attr.name = ro.label;
    sensors.dev_attr.attr.mode = 0444;
    sensors.dev_attr.show = ro.show;
    sensors.index = ro.index;
    err = device_create_file(dev, &sensors.dev_attr);
    if (err) {
    sensors.dev_attr.attr.name = core::ptr::null_mut();
    goto error;
    }
    sensors++;
    ro++;
    }
// Set up read-write sensors
    while (rw.label) {
    sysfs_attr_init(&sensors.dev_attr.attr);
    sensors.dev_attr.attr.name = rw.label;
    sensors.dev_attr.attr.mode = 0644;
    sensors.dev_attr.show = rw.show;
    sensors.dev_attr.store = rw.set;
    sensors.index = rw.index;
    err = device_create_file(dev, &sensors.dev_attr);
    if (err) {
    sensors.dev_attr.attr.name = core::ptr::null_mut();
    goto error;
    }
    sensors++;
    rw++;
    }
    err = device_create_file(dev, &sensor_dev_attr_name.dev_attr);
    if (err)
    goto error;
    err = device_create_file(dev, &sensor_dev_attr_version.dev_attr);
    return err;
    error:
    aem_remove_sensors(data);
    return err;
    }
// sysfs support functions for AEM2 sensors
// Display temperature use
    static ssize_t aem2_show_temp(struct device *dev,
    struct device_attribute *devattr,
    char *buf)
    {
    struct sensor_device_attribute *attr = to_sensor_dev_attr(devattr);
    struct aem_data *a = dev_get_drvdata(dev);
    a.update(a);
    return sprintf(buf, "%u\n", a.temp[attr.index] * 1000);
    }
// Display power-capping registers
    static ssize_t aem2_show_pcap_value(struct device *dev,
    struct device_attribute *devattr,
    char *buf)
    {
    struct sensor_device_attribute *attr = to_sensor_dev_attr(devattr);
    struct aem_data *a = dev_get_drvdata(dev);
    a.update(a);
    return sprintf(buf, "%u\n", a.pcap[attr.index] * 100000);
    }
// Remove sensors attached to an AEM device
#[no_mangle]
unsafe extern "C" fn aem_remove_sensors(data: *mut aem_data) {
    static void aem_remove_sensors(struct aem_data *data)
    {
    int i;
    for (i = 0; i < AEM_NUM_SENSORS; i++) {
    if (!data.sensors[i].dev_attr.attr.name)
    continue;
    device_remove_file(&data.pdev.dev,
    &data.sensors[i].dev_attr);
    }
    device_remove_file(&data.pdev.dev,
    &sensor_dev_attr_name.dev_attr);
    device_remove_file(&data.pdev.dev,
    &sensor_dev_attr_version.dev_attr);
    }
// Sensor probe functions
// Description of AEM1 sensors
    static const struct aem_ro_sensor_template aem1_ro_sensors[] = {
    {"energy1_input",  aem_show_energy, 0},
    {"power1_average", aem_show_power,  0},
    {core::ptr::null_mut(),		   core::ptr::null_mut(),	    0},
    };
    static const struct aem_rw_sensor_template aem1_rw_sensors[] = {
    {"power1_average_interval", aem_show_power_period, aem_set_power_period, 0},
    {core::ptr::null_mut(),			    core::ptr::null_mut(),                  core::ptr::null_mut(),                 0},
    };
// Description of AEM2 sensors
    static const struct aem_ro_sensor_template aem2_ro_sensors[] = {
    {"energy1_input",	  aem_show_energy,	0},
    {"energy2_input",	  aem_show_energy,	1},
    {"power1_average",	  aem_show_power,	0},
    {"power2_average",	  aem_show_power,	1},
    {"temp1_input",		  aem2_show_temp,	0},
    {"temp2_input",		  aem2_show_temp,	1},
    {"power4_average",	  aem2_show_pcap_value,	POWER_CAP_MAX_HOTPLUG},
    {"power5_average",	  aem2_show_pcap_value,	POWER_CAP_MAX},
    {"power6_average",	  aem2_show_pcap_value,	POWER_CAP_MIN_WARNING},
    {"power7_average",	  aem2_show_pcap_value,	POWER_CAP_MIN},
    {"power3_average",	  aem2_show_pcap_value,	POWER_AUX},
    {"power_cap",		  aem2_show_pcap_value,	POWER_CAP},
    {core::ptr::null_mut(),                    core::ptr::null_mut(),                 0},
    };
    static const struct aem_rw_sensor_template aem2_rw_sensors[] = {
    {"power1_average_interval", aem_show_power_period, aem_set_power_period, 0},
    {"power2_average_interval", aem_show_power_period, aem_set_power_period, 1},
    {core::ptr::null_mut(),			    core::ptr::null_mut(),                  core::ptr::null_mut(),                 0},
    };
// Set up AEM1 sensor attrs
#[no_mangle]
unsafe extern "C" fn aem1_find_sensors(data: *mut aem_data) -> c_int {
    static int aem1_find_sensors(struct aem_data *data)
    {
    return aem_register_sensors(data, aem1_ro_sensors, aem1_rw_sensors);
    }
// Set up AEM2 sensor attrs
#[no_mangle]
unsafe extern "C" fn aem2_find_sensors(data: *mut aem_data) -> c_int {
    static int aem2_find_sensors(struct aem_data *data)
    {
    return aem_register_sensors(data, aem2_ro_sensors, aem2_rw_sensors);
    }
// Module init/exit routines
#[no_mangle]
unsafe extern "C" fn aem_init() -> int __init {
    static int __init aem_init(void)
    {
    int res;
    res = driver_register(&aem_driver.driver);
    if (res) {
    pr_err("Can't register aem driver\n");
    return res;
    }
    res = ipmi_smi_watcher_register(&driver_data.bmc_events);
    if (res)
    goto ipmi_reg_err;
    return 0;
    ipmi_reg_err:
    driver_unregister(&aem_driver.driver);
    return res;
    }
#[no_mangle]
unsafe extern "C" fn aem_exit() -> void __exit {
    static void __exit aem_exit(void)
    {
    struct aem_data *p1, *next1;
    ipmi_smi_watcher_unregister(&driver_data.bmc_events);
    driver_unregister(&aem_driver.driver);
    list_for_each_entry_safe(p1, next1, &driver_data.aem_devices, list)
    aem_delete(p1);
    }
    MODULE_AUTHOR("Darrick J. Wong <darrick.wong@oracle.com>");
    MODULE_DESCRIPTION("IBM AEM power/temp/energy sensor driver");
    MODULE_LICENSE("GPL");
    module_init(aem_init);
    module_exit(aem_exit);
    MODULE_ALIAS("dmi:bvnIBM:*:pnIBMSystemx3350-*");
    MODULE_ALIAS("dmi:bvnIBM:*:pnIBMSystemx3550-*");
    MODULE_ALIAS("dmi:bvnIBM:*:pnIBMSystemx3650-*");
    MODULE_ALIAS("dmi:bvnIBM:*:pnIBMSystemx3655-*");
    MODULE_ALIAS("dmi:bvnIBM:*:pnIBMSystemx3755-*");
    MODULE_ALIAS("dmi:bvnIBM:*:pnIBM3850M2/x3950M2-*");
    MODULE_ALIAS("dmi:bvnIBM:*:pnIBMBladeHC10-*");
