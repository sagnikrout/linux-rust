//! Automatically rewritten from C to Rust
//! Source: drivers/rapidio/rio-scan.c
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
// RapidIO enumeration and discovery support
//
// Copyright 2005 MontaVista Software, Inc.
// Matt Porter <mporter@kernel.crashing.org>
//
// Copyright 2009 Integrated Device Technology, Inc.
// Alex Bounine <alexandre.bounine@idt.com>
// - Added Port-Write/Error Management initialization and handling
//
// Copyright 2009 Sysgo AG
// Thomas Moll <thomas.moll@sysgo.com>
// - Added Input- Output- enable functionality, to allow full communication
//

    static void rio_init_em(struct rio_dev *rdev);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rio_id_table {
    pub /: *mut *mut u16 start; / logical minimal id,
    pub /: *mut *mut u32 max; / max number of IDs in table,
    pub lock: spinlock_t,
    pub table: [c_ulong; ],
}

    let mut next_destid: static int = 0;
    let mut next_comptag: static int = 1;
//
// rio_destid_alloc - Allocate next available destID for given network
// @net: RIO network
//
// Returns next available device destination ID for the specified RIO network.
// Marks allocated ID as one in use.
// Returns RIO_INVALID_DESTID if new destID is not available.
//
#[no_mangle]
unsafe extern "C" fn rio_destid_alloc(net: *mut rio_net) -> u16 {
    static u16 rio_destid_alloc(struct rio_net *net)
    {
    int destid;
    struct rio_id_table *idtab = (struct rio_id_table *)net.enum_data;
    spin_lock(&idtab.lock);
    destid = find_first_zero_bit(idtab.table, idtab.max);
    if (destid < idtab.max) {
    set_bit(destid, idtab.table);
    destid += idtab.start;
    } else
    destid = RIO_INVALID_DESTID;
    spin_unlock(&idtab.lock);
    return (u16)destid;
    }
//
// rio_destid_reserve - Reserve the specified destID
// @net: RIO network
// @destid: destID to reserve
//
// Tries to reserve the specified destID.
// Returns 0 if successful.
//
#[no_mangle]
unsafe extern "C" fn rio_destid_reserve(net: *mut rio_net, destid: u16) -> c_int {
    static int rio_destid_reserve(struct rio_net *net, u16 destid)
    {
    int oldbit;
    struct rio_id_table *idtab = (struct rio_id_table *)net.enum_data;
    destid -= idtab.start;
    spin_lock(&idtab.lock);
    oldbit = test_and_set_bit(destid, idtab.table);
    spin_unlock(&idtab.lock);
    return oldbit;
    }
//
// rio_destid_free - free a previously allocated destID
// @net: RIO network
// @destid: destID to free
//
// Makes the specified destID available for use.
//
#[no_mangle]
unsafe extern "C" fn rio_destid_free(net: *mut rio_net, destid: u16) {
    static void rio_destid_free(struct rio_net *net, u16 destid)
    {
    struct rio_id_table *idtab = (struct rio_id_table *)net.enum_data;
    destid -= idtab.start;
    spin_lock(&idtab.lock);
    clear_bit(destid, idtab.table);
    spin_unlock(&idtab.lock);
    }
//
// rio_destid_first - return first destID in use
// @net: RIO network
//
#[no_mangle]
unsafe extern "C" fn rio_destid_first(net: *mut rio_net) -> u16 {
    static u16 rio_destid_first(struct rio_net *net)
    {
    int destid;
    struct rio_id_table *idtab = (struct rio_id_table *)net.enum_data;
    spin_lock(&idtab.lock);
    destid = find_first_bit(idtab.table, idtab.max);
    if (destid >= idtab.max)
    destid = RIO_INVALID_DESTID;
    else
    destid += idtab.start;
    spin_unlock(&idtab.lock);
    return (u16)destid;
    }
//
// rio_destid_next - return next destID in use
// @net: RIO network
// @from: destination ID from which search shall continue
//
#[no_mangle]
unsafe extern "C" fn rio_destid_next(net: *mut rio_net, from: u16) -> u16 {
    static u16 rio_destid_next(struct rio_net *net, u16 from)
    {
    int destid;
    struct rio_id_table *idtab = (struct rio_id_table *)net.enum_data;
    spin_lock(&idtab.lock);
    destid = find_next_bit(idtab.table, idtab.max, from);
    if (destid >= idtab.max)
    destid = RIO_INVALID_DESTID;
    else
    destid += idtab.start;
    spin_unlock(&idtab.lock);
    return (u16)destid;
    }
//
// rio_get_device_id - Get the base/extended device id for a device
// @port: RIO master port
// @destid: Destination ID of device
// @hopcount: Hopcount to device
//
// Reads the base/extended device id from a device. Returns the
// 8/16-bit device ID.
//
#[no_mangle]
unsafe extern "C" fn rio_get_device_id(port: *mut rio_mport, destid: u16, hopcount: u8) -> u16 {
    static u16 rio_get_device_id(struct rio_mport *port, u16 destid, u8 hopcount)
    {
    u32 result;
    rio_mport_read_config_32(port, destid, hopcount, RIO_DID_CSR, &result);
    return RIO_GET_DID(port.sys_size, result);
    }
//
// rio_set_device_id - Set the base/extended device id for a device
// @port: RIO master port
// @destid: Destination ID of device
// @hopcount: Hopcount to device
// @did: Device ID value to be written
//
// Writes the base/extended device id from a device.
//
#[no_mangle]
unsafe extern "C" fn rio_set_device_id(port: *mut rio_mport, destid: u16, hopcount: u8, did: u16) {
    static void rio_set_device_id(struct rio_mport *port, u16 destid, u8 hopcount, u16 did)
    {
    rio_mport_write_config_32(port, destid, hopcount, RIO_DID_CSR,
    RIO_SET_DID(port.sys_size, did));
    }
//
// rio_clear_locks- Release all host locks and signal enumeration complete
// @net: RIO network to run on
//
// Marks the component tag CSR on each device with the enumeration
// complete flag. When complete, it then release the host locks on
// each device. Returns 0 on success or %-EINVAL on failure.
//
#[no_mangle]
unsafe extern "C" fn rio_clear_locks(net: *mut rio_net) -> c_int {
    static int rio_clear_locks(struct rio_net *net)
    {
    struct rio_mport *port = net.hport;
    struct rio_dev *rdev;
    u32 result;
    let mut ret: c_int = 0;
// Release host device id locks
    rio_local_write_config_32(port, RIO_HOST_DID_LOCK_CSR,
    port.host_deviceid);
    rio_local_read_config_32(port, RIO_HOST_DID_LOCK_CSR, &result);
    if ((result & 0xffff) != 0xffff) {
    printk(KERN_INFO
    "RIO: badness when releasing host lock on master port, result %8.8x\n",
    result);
    ret = -EINVAL;
    }
    list_for_each_entry(rdev, &net.devices, net_list) {
    rio_write_config_32(rdev, RIO_HOST_DID_LOCK_CSR,
    port.host_deviceid);
    rio_read_config_32(rdev, RIO_HOST_DID_LOCK_CSR, &result);
    if ((result & 0xffff) != 0xffff) {
    printk(KERN_INFO
    "RIO: badness when releasing host lock on vid %4.4x did %4.4x\n",
    rdev.vid, rdev.did);
    ret = -EINVAL;
    }
// Mark device as discovered and enable master
    rio_read_config_32(rdev,
    rdev.phys_efptr + RIO_PORT_GEN_CTL_CSR,
    &result);
    result |= RIO_PORT_GEN_DISCOVERED | RIO_PORT_GEN_MASTER;
    rio_write_config_32(rdev,
    rdev.phys_efptr + RIO_PORT_GEN_CTL_CSR,
    result);
    }
    return ret;
    }
//
// rio_enum_host- Set host lock and initialize host destination ID
// @port: Master port to issue transaction
//
// Sets the local host master port lock and destination ID register
// with the host device ID value. The host device ID value is provided
// by the platform. Returns %0 on success or %-1 on failure.
//
#[no_mangle]
unsafe extern "C" fn rio_enum_host(port: *mut rio_mport) -> c_int {
    static int rio_enum_host(struct rio_mport *port)
    {
    u32 result;
// Set master port host device id lock
    rio_local_write_config_32(port, RIO_HOST_DID_LOCK_CSR,
    port.host_deviceid);
    rio_local_read_config_32(port, RIO_HOST_DID_LOCK_CSR, &result);
    if ((result & 0xffff) != port.host_deviceid)
    return -1;
// Set master port destid and init destid ctr
    rio_local_set_device_id(port, port.host_deviceid);
    return 0;
    }
//
// rio_device_has_destid- Test if a device contains a destination ID register
// @port: Master port to issue transaction
// @src_ops: RIO device source operations
// @dst_ops: RIO device destination operations
//
// Checks the provided @src_ops and @dst_ops for the necessary transaction
// capabilities that indicate whether or not a device will implement a
// destination ID register. Returns 1 if true or 0 if false.
//
    static int rio_device_has_destid(struct rio_mport *port, int src_ops,
    int dst_ops)
    {
    let mut mask: u32 = RIO_OPS_READ | RIO_OPS_WRITE | RIO_OPS_ATOMIC_TST_SWP | RIO_OPS_ATOMIC_INC | RIO_OPS_ATOMIC_DEC | RIO_OPS_ATOMIC_SET | RIO_OPS_ATOMIC_CLR;
    return !!((src_ops | dst_ops) & mask);
    }
//
// rio_release_dev- Frees a RIO device struct
// @dev: LDM device associated with a RIO device struct
//
// Gets the RIO device struct associated a RIO device struct.
// The RIO device struct is freed.
//
#[no_mangle]
unsafe extern "C" fn rio_release_dev(dev: *mut device) {
    static void rio_release_dev(struct device *dev)
    {
    struct rio_dev *rdev;
    rdev = to_rio_dev(dev);
    kfree(rdev);
    }
//
// rio_is_switch- Tests if a RIO device has switch capabilities
// @rdev: RIO device
//
// Gets the RIO device Processing Element Features register
// contents and tests for switch capabilities. Returns 1 if
// the device is a switch or 0 if it is not a switch.
// The RIO device struct is freed.
//
#[no_mangle]
unsafe extern "C" fn rio_is_switch(rdev: *mut rio_dev) -> c_int {
    static int rio_is_switch(struct rio_dev *rdev)
    {
    if (rdev.pef & RIO_PEF_SWITCH)
    return 1;
    return 0;
    }
//
// rio_setup_device- Allocates and sets up a RIO device
// @net: RIO network
// @port: Master port to send transactions
// @destid: Current destination ID
// @hopcount: Current hopcount
// @do_enum: Enumeration/Discovery mode flag
//
// Allocates a RIO device and configures fields based on configuration
// space contents. If device has a destination ID register, a destination
// ID is either assigned in enumeration mode or read from configuration
// space in discovery mode.  If the device has switch capabilities, then
// a switch is allocated and configured appropriately. Returns a pointer
// to a RIO device on success or NULL on failure.
//
    static struct rio_dev *rio_setup_device(struct rio_net *net,
    struct rio_mport *port, u16 destid,
    u8 hopcount, int do_enum)
    {
    let mut ret: c_int = 0;
    struct rio_dev *rdev;
    struct rio_switch *rswitch = core::ptr::null_mut();
    int result, rdid;
    size_t size;
    let mut swpinfo: u32 = 0;
    size = sizeof(*rdev);
    if (rio_mport_read_config_32(port, destid, hopcount,
    RIO_PEF_CAR, &result))
    return core::ptr::null_mut();
    if (result & (RIO_PEF_SWITCH | RIO_PEF_MULTIPORT)) {
    rio_mport_read_config_32(port, destid, hopcount,
    RIO_SWP_INFO_CAR, &swpinfo);
    if (result & RIO_PEF_SWITCH)
    size += struct_size(rswitch, nextdev, RIO_GET_TOTAL_PORTS(swpinfo));
    }
    rdev = kzalloc(size, GFP_KERNEL);
    if (!rdev)
    return core::ptr::null_mut();
    rdev.net = net;
    rdev.pef = result;
    rdev.swpinfo = swpinfo;
    rio_mport_read_config_32(port, destid, hopcount, RIO_DEV_ID_CAR,
    &result);
    rdev.did = result >> 16;
    rdev.vid = result & 0xffff;
    rio_mport_read_config_32(port, destid, hopcount, RIO_DEV_INFO_CAR,
    &rdev.device_rev);
    rio_mport_read_config_32(port, destid, hopcount, RIO_ASM_ID_CAR,
    &result);
    rdev.asm_did = result >> 16;
    rdev.asm_vid = result & 0xffff;
    rio_mport_read_config_32(port, destid, hopcount, RIO_ASM_INFO_CAR,
    &result);
    rdev.asm_rev = result >> 16;
    if (rdev.pef & RIO_PEF_EXT_FEATURES) {
    rdev.efptr = result & 0xffff;
    rdev.phys_efptr = rio_mport_get_physefb(port, 0, destid,
    hopcount, &rdev.phys_rmap);
    pr_debug("RIO: %s Register Map %d device\n",
    __func__, rdev.phys_rmap);
    rdev.em_efptr = rio_mport_get_feature(port, 0, destid,
    hopcount, RIO_EFB_ERR_MGMNT);
    if (!rdev.em_efptr)
    rdev.em_efptr = rio_mport_get_feature(port, 0, destid,
    hopcount, RIO_EFB_ERR_MGMNT_HS);
    }
    rio_mport_read_config_32(port, destid, hopcount, RIO_SRC_OPS_CAR,
    &rdev.src_ops);
    rio_mport_read_config_32(port, destid, hopcount, RIO_DST_OPS_CAR,
    &rdev.dst_ops);
    if (do_enum) {
// Assign component tag to device
    if (next_comptag >= 0x10000) {
    pr_err("RIO: Component Tag Counter Overflow\n");
    goto cleanup;
    }
    rio_mport_write_config_32(port, destid, hopcount,
    RIO_COMPONENT_TAG_CSR, next_comptag);
    rdev.comp_tag = next_comptag++;
    rdev.do_enum = true;
    }  else {
    rio_mport_read_config_32(port, destid, hopcount,
    RIO_COMPONENT_TAG_CSR,
    &rdev.comp_tag);
    }
    if (rio_device_has_destid(port, rdev.src_ops, rdev.dst_ops)) {
    if (do_enum) {
    rio_set_device_id(port, destid, hopcount, next_destid);
    rdev.destid = next_destid;
    next_destid = rio_destid_alloc(net);
    } else
    rdev.destid = rio_get_device_id(port, destid, hopcount);
    rdev.hopcount = 0xff;
    } else {
// Switch device has an associated destID which
// will be adjusted later
//
    rdev.destid = destid;
    rdev.hopcount = hopcount;
    }
// If a PE has both switch and other functions, show it as a switch
    if (rio_is_switch(rdev)) {
    rswitch = rdev.rswitch;
    rswitch.port_ok = 0;
    spin_lock_init(&rswitch.lock);
    rswitch.route_table =
    kzalloc(RIO_MAX_ROUTE_ENTRIES(port.sys_size),
    GFP_KERNEL);
    if (!rswitch.route_table)
    goto cleanup;
// Initialize switch route table
    for (rdid = 0; rdid < RIO_MAX_ROUTE_ENTRIES(port.sys_size);
    rdid++)
    rswitch.route_table[rdid] = RIO_INVALID_ROUTE;
    dev_set_name(&rdev.dev, "%02x:s:%04x", rdev.net.id,
    rdev.comp_tag & RIO_CTAG_UDEVID);
    if (do_enum)
    rio_route_clr_table(rdev, RIO_GLOBAL_TABLE, 0);
    } else {
    if (do_enum)
// Enable Input Output Port (transmitter receiver)
    rio_enable_rx_tx_port(port, 0, destid, hopcount, 0);
    dev_set_name(&rdev.dev, "%02x:e:%04x", rdev.net.id,
    rdev.comp_tag & RIO_CTAG_UDEVID);
    }
    rdev.dev.parent = &net.dev;
    rio_attach_device(rdev);
    rdev.dev.release = rio_release_dev;
    rdev.dma_mask = DMA_BIT_MASK(32);
    rdev.dev.dma_mask = &rdev.dma_mask;
    rdev.dev.coherent_dma_mask = DMA_BIT_MASK(32);
    if (rdev.dst_ops & RIO_DST_OPS_DOORBELL)
    rio_init_dbell_res(&rdev.riores[RIO_DOORBELL_RESOURCE],
    0, 0xffff);
    ret = rio_add_device(rdev);
    if (ret) {
    if (rswitch)
    kfree(rswitch.route_table);
    put_device(&rdev.dev);
    return core::ptr::null_mut();
    }
    rio_dev_get(rdev);
    return rdev;
    cleanup:
    if (rswitch)
    kfree(rswitch.route_table);
    kfree(rdev);
    return core::ptr::null_mut();
    }
//
// rio_sport_is_active- Tests if a switch port has an active connection.
// @rdev: RapidIO device object
// @sp: Switch port number
//
// Reads the port error status CSR for a particular switch port to
// determine if the port has an active link.  Returns
// %RIO_PORT_N_ERR_STS_PORT_OK if the port is active or %0 if it is
// inactive.
//
    static int
    rio_sport_is_active(struct rio_dev *rdev, int sp)
    {
    let mut result: u32 = 0;
    rio_read_config_32(rdev, RIO_DEV_PORT_N_ERR_STS_CSR(rdev, sp),
    &result);
    return result & RIO_PORT_N_ERR_STS_PORT_OK;
    }
//
// rio_get_host_deviceid_lock- Reads the Host Device ID Lock CSR on a device
// @port: Master port to send transaction
// @hopcount: Number of hops to the device
//
// Used during enumeration to read the Host Device ID Lock CSR on a
// RIO device. Returns the value of the lock register.
//
#[no_mangle]
unsafe extern "C" fn rio_get_host_deviceid_lock(port: *mut rio_mport, hopcount: u8) -> u16 {
    static u16 rio_get_host_deviceid_lock(struct rio_mport *port, u8 hopcount)
    {
    u32 result;
    rio_mport_read_config_32(port, RIO_ANY_DESTID(port.sys_size), hopcount,
    RIO_HOST_DID_LOCK_CSR, &result);
    return (u16) (result & 0xffff);
    }
//
// rio_enum_peer- Recursively enumerate a RIO network through a master port
// @net: RIO network being enumerated
// @port: Master port to send transactions
// @hopcount: Number of hops into the network
// @prev: Previous RIO device connected to the enumerated one
// @prev_port: Port on previous RIO device
//
// Recursively enumerates a RIO network.  Transactions are sent via the
// master port passed in @port.
//
    static int rio_enum_peer(struct rio_net *net, struct rio_mport *port,
    u8 hopcount, struct rio_dev *prev, int prev_port)
    {
    struct rio_dev *rdev;
    u32 regval;
    int tmp;
    if (rio_mport_chk_dev_access(port,
    RIO_ANY_DESTID(port.sys_size), hopcount)) {
    pr_debug("RIO: device access check failed\n");
    return -1;
    }
    if (rio_get_host_deviceid_lock(port, hopcount) == port.host_deviceid) {
    pr_debug("RIO: PE already discovered by this host\n");
//
// Already discovered by this host. Add it as another
// link to the existing device.
//
    rio_mport_read_config_32(port, RIO_ANY_DESTID(port.sys_size),
    hopcount, RIO_COMPONENT_TAG_CSR, &regval);
    if (regval) {
    rdev = rio_get_comptag((regval & 0xffff), core::ptr::null_mut());
    if (rdev && prev && rio_is_switch(prev)) {
    pr_debug("RIO: redundant path to %s\n",
    rio_name(rdev));
    prev.rswitch.nextdev[prev_port] = rdev;
    }
    }
    return 0;
    }
// Attempt to acquire device lock
    rio_mport_write_config_32(port, RIO_ANY_DESTID(port.sys_size),
    hopcount,
    RIO_HOST_DID_LOCK_CSR, port.host_deviceid);
    while ((tmp = rio_get_host_deviceid_lock(port, hopcount))
    < port.host_deviceid) {
// Delay a bit
    mdelay(1);
// Attempt to acquire device lock again
    rio_mport_write_config_32(port, RIO_ANY_DESTID(port.sys_size),
    hopcount,
    RIO_HOST_DID_LOCK_CSR,
    port.host_deviceid);
    }
    if (rio_get_host_deviceid_lock(port, hopcount) > port.host_deviceid) {
    pr_debug(
    "RIO: PE locked by a higher priority host...retreating\n");
    return -1;
    }
// Setup new RIO device
    rdev = rio_setup_device(net, port, RIO_ANY_DESTID(port.sys_size),
    hopcount, 1);
    if (rdev) {
    rdev.prev = prev;
    if (prev && rio_is_switch(prev))
    prev.rswitch.nextdev[prev_port] = rdev;
    } else
    return -1;
    if (rio_is_switch(rdev)) {
    int sw_destid;
    int cur_destid;
    int sw_inport;
    u16 destid;
    int port_num;
    sw_inport = RIO_GET_PORT_NUM(rdev.swpinfo);
    rio_route_add_entry(rdev, RIO_GLOBAL_TABLE,
    port.host_deviceid, sw_inport, 0);
    rdev.rswitch.route_table[port.host_deviceid] = sw_inport;
    destid = rio_destid_first(net);
    while (destid != RIO_INVALID_DESTID && destid < next_destid) {
    if (destid != port.host_deviceid) {
    rio_route_add_entry(rdev, RIO_GLOBAL_TABLE,
    destid, sw_inport, 0);
    rdev.rswitch.route_table[destid] = sw_inport;
    }
    destid = rio_destid_next(net, destid + 1);
    }
    pr_debug(
    "RIO: found %s (vid %4.4x did %4.4x) with %d ports\n",
    rio_name(rdev), rdev.vid, rdev.did,
    RIO_GET_TOTAL_PORTS(rdev.swpinfo));
    sw_destid = next_destid;
    for (port_num = 0;
    port_num < RIO_GET_TOTAL_PORTS(rdev.swpinfo);
    port_num++) {
    if (sw_inport == port_num) {
    rio_enable_rx_tx_port(port, 0,
    RIO_ANY_DESTID(port.sys_size),
    hopcount, port_num);
    rdev.rswitch.port_ok |= (1 << port_num);
    continue;
    }
    cur_destid = next_destid;
    if (rio_sport_is_active(rdev, port_num)) {
    pr_debug(
    "RIO: scanning device on port %d\n",
    port_num);
    rio_enable_rx_tx_port(port, 0,
    RIO_ANY_DESTID(port.sys_size),
    hopcount, port_num);
    rdev.rswitch.port_ok |= (1 << port_num);
    rio_route_add_entry(rdev, RIO_GLOBAL_TABLE,
    RIO_ANY_DESTID(port.sys_size),
    port_num, 0);
    if (rio_enum_peer(net, port, hopcount + 1,
    rdev, port_num) < 0)
    return -1;
// Update routing tables
    destid = rio_destid_next(net, cur_destid + 1);
    if (destid != RIO_INVALID_DESTID) {
    for (destid = cur_destid;
    destid < next_destid;) {
    if (destid != port.host_deviceid) {
    rio_route_add_entry(rdev,
    RIO_GLOBAL_TABLE,
    destid,
    port_num,
    0);
    rdev.rswitch.
    route_table[destid] =
    port_num;
    }
    destid = rio_destid_next(net,
    destid + 1);
    }
    }
    } else {
// If switch supports Error Management,
// set PORT_LOCKOUT bit for unused port
//
    if (rdev.em_efptr)
    rio_set_port_lockout(rdev, port_num, 1);
    rdev.rswitch.port_ok &= ~(1 << port_num);
    }
    }
// Direct Port-write messages to the enumeratiing host
    if ((rdev.src_ops & RIO_SRC_OPS_PORT_WRITE) &&
    (rdev.em_efptr)) {
    rio_write_config_32(rdev,
    rdev.em_efptr + RIO_EM_PW_TGT_DEVID,
    (port.host_deviceid << 16) |
    (port.sys_size << 15));
    }
    rio_init_em(rdev);
// Check for empty switch
    if (next_destid == sw_destid)
    next_destid = rio_destid_alloc(net);
    rdev.destid = sw_destid;
    } else
    pr_debug("RIO: found %s (vid %4.4x did %4.4x)\n",
    rio_name(rdev), rdev.vid, rdev.did);
    return 0;
    }
//
// rio_enum_complete- Tests if enumeration of a network is complete
// @port: Master port to send transaction
//
// Tests the PGCCSR discovered bit for non-zero value (enumeration
// complete flag). Return %1 if enumeration is complete or %0 if
// enumeration is incomplete.
//
#[no_mangle]
unsafe extern "C" fn rio_enum_complete(port: *mut rio_mport) -> c_int {
    static int rio_enum_complete(struct rio_mport *port)
    {
    u32 regval;
    rio_local_read_config_32(port, port.phys_efptr + RIO_PORT_GEN_CTL_CSR,
    &regval);
    return (regval & RIO_PORT_GEN_DISCOVERED) ? 1 : 0;
    }
//
// rio_disc_peer- Recursively discovers a RIO network through a master port
// @net: RIO network being discovered
// @port: Master port to send transactions
// @destid: Current destination ID in network
// @hopcount: Number of hops into the network
// @prev: previous rio_dev
// @prev_port: previous port number
//
// Recursively discovers a RIO network.  Transactions are sent via the
// master port passed in @port.
//
    static int
    rio_disc_peer(struct rio_net *net, struct rio_mport *port, u16 destid,
    u8 hopcount, struct rio_dev *prev, int prev_port)
    {
    u8 port_num, route_port;
    struct rio_dev *rdev;
    u16 ndestid;
// Setup new RIO device
    if ((rdev = rio_setup_device(net, port, destid, hopcount, 0))) {
    rdev.prev = prev;
    if (prev && rio_is_switch(prev))
    prev.rswitch.nextdev[prev_port] = rdev;
    } else
    return -1;
    if (rio_is_switch(rdev)) {
// Associated destid is how we accessed this switch
    rdev.destid = destid;
    pr_debug(
    "RIO: found %s (vid %4.4x did %4.4x) with %d ports\n",
    rio_name(rdev), rdev.vid, rdev.did,
    RIO_GET_TOTAL_PORTS(rdev.swpinfo));
    for (port_num = 0;
    port_num < RIO_GET_TOTAL_PORTS(rdev.swpinfo);
    port_num++) {
    if (RIO_GET_PORT_NUM(rdev.swpinfo) == port_num)
    continue;
    if (rio_sport_is_active(rdev, port_num)) {
    pr_debug(
    "RIO: scanning device on port %d\n",
    port_num);
    rio_lock_device(port, destid, hopcount, 1000);
    for (ndestid = 0;
    ndestid < RIO_ANY_DESTID(port.sys_size);
    ndestid++) {
    rio_route_get_entry(rdev,
    RIO_GLOBAL_TABLE,
    ndestid,
    &route_port, 0);
    if (route_port == port_num)
    break;
    }
    if (ndestid == RIO_ANY_DESTID(port.sys_size))
    continue;
    rio_unlock_device(port, destid, hopcount);
    if (rio_disc_peer(net, port, ndestid,
    hopcount + 1, rdev, port_num) < 0)
    return -1;
    }
    }
    } else
    pr_debug("RIO: found %s (vid %4.4x did %4.4x)\n",
    rio_name(rdev), rdev.vid, rdev.did);
    return 0;
    }
//
// rio_mport_is_active- Tests if master port link is active
// @port: Master port to test
//
// Reads the port error status CSR for the master port to
// determine if the port has an active link.  Returns
// %RIO_PORT_N_ERR_STS_PORT_OK if the  master port is active
// or %0 if it is inactive.
//
#[no_mangle]
unsafe extern "C" fn rio_mport_is_active(port: *mut rio_mport) -> c_int {
    static int rio_mport_is_active(struct rio_mport *port)
    {
    let mut result: u32 = 0;
    rio_local_read_config_32(port,
    port.phys_efptr +
    RIO_PORT_N_ERR_STS_CSR(port.index, port.phys_rmap),
    &result);
    return result & RIO_PORT_N_ERR_STS_PORT_OK;
    }
#[no_mangle]
unsafe extern "C" fn rio_scan_release_net(net: *mut rio_net) {
    static void rio_scan_release_net(struct rio_net *net)
    {
    pr_debug("RIO-SCAN: %s: net_%d\n", __func__, net.id);
    kfree(net.enum_data);
    }
#[no_mangle]
unsafe extern "C" fn rio_scan_release_dev(dev: *mut device) {
    static void rio_scan_release_dev(struct device *dev)
    {
    struct rio_net *net;
    net = to_rio_net(dev);
    pr_debug("RIO-SCAN: %s: net_%d\n", __func__, net.id);
    kfree(net);
    }
//
// rio_scan_alloc_net - Allocate and configure a new RIO network
// @mport: Master port associated with the RIO network
// @do_enum: Enumeration/Discovery mode flag
// @start: logical minimal start id for new net
//
// Allocates a new RIO network structure and initializes enumerator-specific
// part of it (if required).
// Returns a RIO network pointer on success or %NULL on failure.
//
    static struct rio_net *rio_scan_alloc_net(struct rio_mport *mport,
    int do_enum, u16 start)
    {
    struct rio_net *net;
    net = rio_alloc_net(mport);
    if (net && do_enum) {
    struct rio_id_table *idtab;
    size_t size;
    size = sizeof(struct rio_id_table) +
    BITS_TO_LONGS(
    RIO_MAX_ROUTE_ENTRIES(mport.sys_size)
    ) * sizeof(long);
    idtab = kzalloc(size, GFP_KERNEL);
    if (idtab == core::ptr::null_mut()) {
    pr_err("RIO: failed to allocate destID table\n");
    kfree(net);
    mport.net = core::ptr::null_mut();
    net = core::ptr::null_mut();
    } else {
    net.enum_data = idtab;
    net.release = rio_scan_release_net;
    idtab.start = start;
    idtab.max = RIO_MAX_ROUTE_ENTRIES(mport.sys_size);
    spin_lock_init(&idtab.lock);
    }
    }
    if (net) {
    net.id = mport.id;
    net.hport = mport;
    dev_set_name(&net.dev, "rnet_%d", net.id);
    net.dev.parent = &mport.dev;
    net.dev.release = rio_scan_release_dev;
    if (rio_add_net(net)) {
    put_device(&net.dev);
    mport.net = core::ptr::null_mut();
    net = core::ptr::null_mut();
    }
    }
    return net;
    }
//
// rio_update_route_tables- Updates route tables in switches
// @net: RIO network to run update on
//
// For each enumerated device, ensure that each switch in a system
// has correct routing entries. Add routes for devices that where
// unknown during the first enumeration pass through the switch.
//
#[no_mangle]
unsafe extern "C" fn rio_update_route_tables(net: *mut rio_net) {
    static void rio_update_route_tables(struct rio_net *net)
    {
    struct rio_dev *rdev, *swrdev;
    struct rio_switch *rswitch;
    u8 sport;
    u16 destid;
    list_for_each_entry(rdev, &net.devices, net_list) {
    destid = rdev.destid;
    list_for_each_entry(rswitch, &net.switches, node) {
    if (rio_is_switch(rdev)	&& (rdev.rswitch == rswitch))
    continue;
    if (RIO_INVALID_ROUTE == rswitch.route_table[destid]) {
    swrdev = sw_to_rio_dev(rswitch);
// Skip if destid ends in empty switch
    if (swrdev.destid == destid)
    continue;
    sport = RIO_GET_PORT_NUM(swrdev.swpinfo);
    rio_route_add_entry(swrdev, RIO_GLOBAL_TABLE,
    destid, sport, 0);
    rswitch.route_table[destid] = sport;
    }
    }
    }
    }
//
// rio_init_em - Initializes RIO Error Management (for switches)
// @rdev: RIO device
//
// For each enumerated switch, call device-specific error management
// initialization routine (if supplied by the switch driver).
//
#[no_mangle]
unsafe extern "C" fn rio_init_em(rdev: *mut rio_dev) {
    static void rio_init_em(struct rio_dev *rdev)
    {
    if (rio_is_switch(rdev) && (rdev.em_efptr) &&
    rdev.rswitch.ops && rdev.rswitch.ops.em_init) {
    rdev.rswitch.ops.em_init(rdev);
    }
    }
//
// rio_enum_mport- Start enumeration through a master port
// @mport: Master port to send transactions
// @flags: Enumeration control flags
//
// Starts the enumeration process. If somebody has enumerated our
// master port device, then give up. If not and we have an active
// link, then start recursive peer enumeration. Returns %0 if
// enumeration succeeds or %-EBUSY if enumeration fails.
//
#[no_mangle]
unsafe extern "C" fn rio_enum_mport(mport: *mut rio_mport, flags: u32) -> c_int {
    static int rio_enum_mport(struct rio_mport *mport, u32 flags)
    {
    struct rio_net *net = core::ptr::null_mut();
    let mut rc: c_int = 0;
    printk(KERN_INFO "RIO: enumerate master port %d, %s\n", mport.id,
    mport.name);
//
// To avoid multiple start requests (repeat enumeration is not supported
// by this method) check if enumeration/discovery was performed for this
// mport: if mport was added into the list of mports for a net exit
// with error.
//
    if (mport.nnode.next || mport.nnode.prev)
    return -EBUSY;
// If somebody else enumerated our master port device, bail.
    if (rio_enum_host(mport) < 0) {
    printk(KERN_INFO
    "RIO: master port %d device has been enumerated by a remote host\n",
    mport.id);
    rc = -EBUSY;
    goto out;
    }
// If master port has an active link, allocate net and enum peers
    if (rio_mport_is_active(mport)) {
    net = rio_scan_alloc_net(mport, 1, 0);
    if (!net) {
    printk(KERN_ERR "RIO: failed to allocate new net\n");
    rc = -ENOMEM;
    goto out;
    }
// reserve mport destID in new net
    rio_destid_reserve(net, mport.host_deviceid);
// Enable Input Output Port (transmitter receiver)
    rio_enable_rx_tx_port(mport, 1, 0, 0, 0);
// Set component tag for host
    rio_local_write_config_32(mport, RIO_COMPONENT_TAG_CSR,
    next_comptag++);
    next_destid = rio_destid_alloc(net);
    if (rio_enum_peer(net, mport, 0, core::ptr::null_mut(), 0) < 0) {
// A higher priority host won enumeration, bail.
    printk(KERN_INFO
    "RIO: master port %d device has lost enumeration to a remote host\n",
    mport.id);
    rio_clear_locks(net);
    rc = -EBUSY;
    goto out;
    }
// free the last allocated destID (unused)
    rio_destid_free(net, next_destid);
    rio_update_route_tables(net);
    rio_clear_locks(net);
    rio_pw_enable(mport, 1);
    } else {
    printk(KERN_INFO "RIO: master port %d link inactive\n",
    mport.id);
    rc = -EINVAL;
    }
    out:
    return rc;
    }
//
// rio_build_route_tables- Generate route tables from switch route entries
// @net: RIO network to run route tables scan on
//
// For each switch device, generate a route table by copying existing
// route entries from the switch.
//
#[no_mangle]
unsafe extern "C" fn rio_build_route_tables(net: *mut rio_net) {
    static void rio_build_route_tables(struct rio_net *net)
    {
    struct rio_switch *rswitch;
    struct rio_dev *rdev;
    int i;
    u8 sport;
    list_for_each_entry(rswitch, &net.switches, node) {
    rdev = sw_to_rio_dev(rswitch);
    rio_lock_device(net.hport, rdev.destid,
    rdev.hopcount, 1000);
    for (i = 0;
    i < RIO_MAX_ROUTE_ENTRIES(net.hport.sys_size);
    i++) {
    if (rio_route_get_entry(rdev, RIO_GLOBAL_TABLE,
    i, &sport, 0) < 0)
    continue;
    rswitch.route_table[i] = sport;
    }
    rio_unlock_device(net.hport, rdev.destid, rdev.hopcount);
    }
    }
//
// rio_disc_mport- Start discovery through a master port
// @mport: Master port to send transactions
// @flags: discovery control flags
//
// Starts the discovery process. If we have an active link,
// then wait for the signal that enumeration is complete (if wait
// is allowed).
// When enumeration completion is signaled, start recursive
// peer discovery. Returns %0 if discovery succeeds or %-EBUSY
// on failure.
//
#[no_mangle]
unsafe extern "C" fn rio_disc_mport(mport: *mut rio_mport, flags: u32) -> c_int {
    static int rio_disc_mport(struct rio_mport *mport, u32 flags)
    {
    struct rio_net *net = core::ptr::null_mut();
    unsigned long to_end;
    printk(KERN_INFO "RIO: discover master port %d, %s\n", mport.id,
    mport.name);
// If master port has an active link, allocate net and discover peers
    if (rio_mport_is_active(mport)) {
    if (rio_enum_complete(mport))
    goto enum_done;
#[no_mangle]
pub unsafe extern "C" fn if(RIO_SCAN_ENUM_NO_WAIT: flags &) -> else {
    else if (flags & RIO_SCAN_ENUM_NO_WAIT)
    return -EAGAIN;
    pr_debug("RIO: wait for enumeration to complete...\n");
    to_end = jiffies + CONFIG_RAPIDIO_DISC_TIMEOUT * HZ;
    while (time_before(jiffies, to_end)) {
    if (rio_enum_complete(mport))
    goto enum_done;
    msleep(10);
    }
    pr_debug("RIO: discovery timeout on mport %d %s\n",
    mport.id, mport.name);
    goto bail;
    enum_done:
    pr_debug("RIO: ... enumeration done\n");
    net = rio_scan_alloc_net(mport, 0, 0);
    if (!net) {
    printk(KERN_ERR "RIO: Failed to allocate new net\n");
    goto bail;
    }
// Read DestID assigned by enumerator
    rio_local_read_config_32(mport, RIO_DID_CSR,
    &mport.host_deviceid);
    mport.host_deviceid = RIO_GET_DID(mport.sys_size,
    mport.host_deviceid);
    if (rio_disc_peer(net, mport, RIO_ANY_DESTID(mport.sys_size),
    0, core::ptr::null_mut(), 0) < 0) {
    printk(KERN_INFO
    "RIO: master port %d device has failed discovery\n",
    mport.id);
    goto bail;
    }
    rio_build_route_tables(net);
    }
    return 0;
    bail:
    return -EBUSY;
    }
    static struct rio_scan rio_scan_ops = {
    .owner = THIS_MODULE,
    .enumerate = rio_enum_mport,
    .discover = rio_disc_mport,
    };
    static bool scan;
    module_param(scan, bool, 0);
    MODULE_PARM_DESC(scan, "Start RapidIO network enumeration/discovery "
    "(default = 0)");
//
// rio_basic_attach:
//
// When this enumeration/discovery method is loaded as a module this function
// registers its specific enumeration and discover routines for all available
// RapidIO mport devices. The "scan" command line parameter controls ability of
// the module to start RapidIO enumeration/discovery automatically.
//
// Returns 0 for success or -EIO if unable to register itself.
//
// This enumeration/discovery method cannot be unloaded and therefore does not
// provide a matching cleanup_module routine.
//
#[no_mangle]
unsafe extern "C" fn rio_basic_attach() -> int __init {
    static int __init rio_basic_attach(void)
    {
    if (rio_register_scan(RIO_MPORT_ANY, &rio_scan_ops))
    return -EIO;
    if (scan)
    rio_init_mports();
    return 0;
    }
    late_initcall(rio_basic_attach);
    MODULE_DESCRIPTION("Basic RapidIO enumeration/discovery");
    MODULE_LICENSE("GPL");
