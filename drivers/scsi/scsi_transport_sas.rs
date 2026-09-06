//! Automatically rewritten from C to Rust
//! Source: drivers/scsi/scsi_transport_sas.c
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
// Copyright (C) 2005-2006 Dell Inc.
//
// Serial Attached SCSI (SAS) transport class.
//
// The SAS transport class contains common code to deal with SAS HBAs,
// an aproximated representation of SAS topologies in the driver model,
// and various sysfs attributes to expose these topologies and management
// interfaces to userspace.
//
// In addition to the basic SCSI core objects this transport class
// introduces two additional intermediate objects:  The SAS PHY
// as represented by struct sas_phy defines an "outgoing" PHY on
// a SAS HBA or Expander, and the SAS remote PHY represented by
// struct sas_rphy defines an "incoming" PHY on a SAS Expander or
// end device.  Note that this is purely a software concept, the
// underlying hardware for a PHY and a remote PHY is the exactly
// the same.
//
// There is no concept of a SAS port in this code, users can see
// what PHYs form a wide port based on the port_identifier attribute,
// which is the same for all PHYs in a port.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sas_host_attrs {
    pub rphy_list: list_head,
    pub lock: mutex,
    pub q: *mut request_queue,
    pub next_target_id: u32,
    pub next_expander_id: u32,
    pub next_port_id: c_int,
}

//
// Hack to allow attributes of the same name in different objects.
//

    struct device_attribute dev_attr_##_prefix##_##_name = \
    __ATTR(_name,_mode,_show,_store)
//
// Pretty printing helpers
//

    static ssize_t							\
    get_sas_##title##_names(u32 table_key, char *buf)		\
    {								\
    char *prefix = "";					\
    ssize_t len = 0;					\
    int i;							\
    \
    for (i = 0; i < ARRAY_SIZE(table); i++) {		\
    if (table[i].value & table_key) {		\
    len += sprintf(buf + len, "%s%s",	\
    prefix, table[i].name);		\
    prefix = ", ";				\
    }						\
    }							\
    len += sprintf(buf + len, "\n");			\
    return len;						\
    }

    static ssize_t							\
    set_sas_##title##_names(u32 *table_key, const char *buf)	\
    {								\
    ssize_t len = 0;					\
    int i;							\
    \
    for (i = 0; i < ARRAY_SIZE(table); i++) {		\
    len = strlen(table[i].name);			\
    if (strncmp(buf, table[i].name, len) == 0 &&	\
    (buf[len] == '\n' || buf[len] == '\0')) {	\
// table_key = table[i].value;		\
    return 0;				\
    }						\
    }							\
    return -EINVAL;						\
    }

    static ssize_t							\
    get_sas_##title##_names(u32 table_key, char *buf)		\
    {								\
    ssize_t len = 0;					\
    int i;							\
    \
    for (i = 0; i < ARRAY_SIZE(table); i++) {		\
    if (table[i].value == table_key) {		\
    len += sprintf(buf + len, "%s",		\
    table[i].name);			\
    break;					\
    }						\
    }							\
    len += sprintf(buf + len, "\n");			\
    return len;						\
    }
    static struct {
    u32		value;
    char		*name;
    } sas_device_type_names[] = {
    { SAS_PHY_UNUSED,		"unused" },
    { SAS_END_DEVICE,		"end device" },
    { SAS_EDGE_EXPANDER_DEVICE,	"edge expander" },
    { SAS_FANOUT_EXPANDER_DEVICE,	"fanout expander" },
    };
    sas_bitfield_name_search(device_type, sas_device_type_names)
    static struct {
    u32		value;
    char		*name;
    } sas_protocol_names[] = {
    { SAS_PROTOCOL_SATA,		"sata" },
    { SAS_PROTOCOL_SMP,		"smp" },
    { SAS_PROTOCOL_STP,		"stp" },
    { SAS_PROTOCOL_SSP,		"ssp" },
    };
    sas_bitfield_name_match(protocol, sas_protocol_names)
    static struct {
    u32		value;
    char		*name;
    } sas_linkspeed_names[] = {
    { SAS_LINK_RATE_UNKNOWN,	"Unknown" },
    { SAS_PHY_DISABLED,		"Phy disabled" },
    { SAS_LINK_RATE_FAILED,		"Link Rate failed" },
    { SAS_SATA_SPINUP_HOLD,		"Spin-up hold" },
    { SAS_LINK_RATE_1_5_GBPS,	"1.5 Gbit" },
    { SAS_LINK_RATE_3_0_GBPS,	"3.0 Gbit" },
    { SAS_LINK_RATE_6_0_GBPS,	"6.0 Gbit" },
    { SAS_LINK_RATE_12_0_GBPS,	"12.0 Gbit" },
    { SAS_LINK_RATE_22_5_GBPS,	"22.5 Gbit" },
    };
    sas_bitfield_name_search(linkspeed, sas_linkspeed_names)
    sas_bitfield_name_set(linkspeed, sas_linkspeed_names)
    static struct sas_end_device *sas_sdev_to_rdev(struct scsi_device *sdev)
    {
    struct sas_rphy *rphy = target_to_rphy(sdev.sdev_target);
    struct sas_end_device *rdev;
    BUG_ON(rphy.identify.device_type != SAS_END_DEVICE);
    rdev = rphy_to_end_device(rphy);
    return rdev;
    }
#[no_mangle]
unsafe extern "C" fn sas_smp_dispatch(job: *mut bsg_job) -> c_int {
    static int sas_smp_dispatch(struct bsg_job *job)
    {
    struct Scsi_Host *shost = dev_to_shost(job.dev);
    struct sas_rphy *rphy = core::ptr::null_mut();
    if (!scsi_is_host_device(job.dev))
    rphy = dev_to_rphy(job.dev);
    if (!job.reply_payload.payload_len) {
    dev_warn(job.dev, "space for a smp response is missing\n");
    bsg_job_done(job, -EINVAL, 0);
    return 0;
    }
    to_sas_internal(shost.transportt).f.smp_handler(job, shost, rphy);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sas_bsg_initialize(shost: *mut Scsi_Host, rphy: *mut sas_rphy) -> c_int {
    static int sas_bsg_initialize(struct Scsi_Host *shost, struct sas_rphy *rphy)
    {
    struct request_queue *q;
    if (!to_sas_internal(shost.transportt).f.smp_handler) {
    printk("%s can't handle SMP requests\n", shost.hostt.name);
    return 0;
    }
    if (rphy) {
    q = bsg_setup_queue(&rphy.dev, dev_name(&rphy.dev), core::ptr::null_mut(),
    sas_smp_dispatch, core::ptr::null_mut(), 0);
    if (IS_ERR(q))
    return PTR_ERR(q);
    rphy.q = q;
    } else {
    char name[20];
    snprintf(name, sizeof(name), "sas_host%d", shost.host_no);
    q = bsg_setup_queue(&shost.shost_gendev, name, core::ptr::null_mut(),
    sas_smp_dispatch, core::ptr::null_mut(), 0);
    if (IS_ERR(q))
    return PTR_ERR(q);
    to_sas_host_attrs(shost).q = q;
    }
    return 0;
    }
//
// SAS host attributes
//
// Set shost->opt_sectors from the DMA optimal mapping size, but only
// when dma_opt_mapping_size() is strictly less than dma_max_mapping_size(),
// indicating a genuine optimization hint from an IOMMU or DMA backend.
// When the two are equal (e.g. IOMMU disabled / passthrough), no real
// hint exists, so leave opt_sectors at 0 to avoid bogus optimal_io_size
// values that break filesystem geometry (e.g. mkfs.xfs stripe alignment).
//
#[no_mangle]
unsafe extern "C" fn sas_dma_setup_opt_sectors(shost: *mut Scsi_Host) {
    static void sas_dma_setup_opt_sectors(struct Scsi_Host *shost)
    {
    struct device *dma_dev = shost.dma_dev;
    let mut opt: usize = dma_opt_mapping_size(dma_dev);
    let mut max: usize = dma_max_mapping_size(dma_dev);
    unsigned int opt_sectors;
// opt >= max means no real hint was provided by the DMA layer
    if (opt >= max)
    return;
// Clamp to max_sectors to avoid overflow in sector arithmetic
    opt_sectors = min_t(unsigned int, opt >> SECTOR_SHIFT,
    shost.max_sectors);
// Guard against zero before rounddown_pow_of_two()
    if (!opt_sectors)
    return;
//
// Round down to power-of-two so filesystem geometry calculations
// (e.g. XFS stripe width/unit) always produce clean divisors.
//
    shost.opt_sectors = rounddown_pow_of_two(opt_sectors);
    }
    static int sas_host_setup(struct transport_container *tc, struct device *dev,
    struct device *cdev)
    {
    struct Scsi_Host *shost = dev_to_shost(dev);
    struct sas_host_attrs *sas_host = to_sas_host_attrs(shost);
    INIT_LIST_HEAD(&sas_host.rphy_list);
    mutex_init(&sas_host.lock);
    sas_host.next_target_id = 0;
    sas_host.next_expander_id = 0;
    sas_host.next_port_id = 0;
    if (sas_bsg_initialize(shost, core::ptr::null_mut()))
    dev_printk(KERN_ERR, dev, "fail to a bsg device %d\n",
    shost.host_no);
    sas_dma_setup_opt_sectors(shost);
    return 0;
    }
    static int sas_host_remove(struct transport_container *tc, struct device *dev,
    struct device *cdev)
    {
    struct Scsi_Host *shost = dev_to_shost(dev);
    struct request_queue *q = to_sas_host_attrs(shost).q;
    bsg_remove_queue(q);
    return 0;
    }
    static DECLARE_TRANSPORT_CLASS(sas_host_class,
    "sas_host", sas_host_setup, sas_host_remove, core::ptr::null_mut());
    static int sas_host_match(struct attribute_container *cont,
    struct device *dev)
    {
    struct Scsi_Host *shost;
    struct sas_internal *i;
    if (!scsi_is_host_device(dev))
    return 0;
    shost = dev_to_shost(dev);
    if (!shost.transportt)
    return 0;
    if (shost.transportt.host_attrs.ac.class !=
    &sas_host_class.class)
    return 0;
    i = to_sas_internal(shost.transportt);
    return &i.t.host_attrs.ac == cont;
    }
#[no_mangle]
unsafe extern "C" fn do_sas_phy_delete(dev: *mut device, data: *mut c_void) -> c_int {
    static int do_sas_phy_delete(struct device *dev, void *data)
    {
    let mut pass: c_int = (int)(unsigned long)data;
    if (pass == 0 && scsi_is_sas_port(dev))
    sas_port_delete(dev_to_sas_port(dev));
#[no_mangle]
pub unsafe extern "C" fn if(scsi_is_sas_phy(dev): pass == 1 &&) -> else {
    else if (pass == 1 && scsi_is_sas_phy(dev))
    sas_phy_delete(dev_to_phy(dev));
    return 0;
    }
//
// sas_remove_children  -  tear down a devices SAS data structures
// @dev:	device belonging to the sas object
//
// Removes all SAS PHYs and remote PHYs for a given object
//
#[no_mangle]
pub unsafe extern "C" fn sas_remove_children(dev: *mut device) {
    void sas_remove_children(struct device *dev)
    {
    device_for_each_child(dev, (void *)0, do_sas_phy_delete);
    device_for_each_child(dev, (void *)1, do_sas_phy_delete);
    }
    EXPORT_SYMBOL(sas_remove_children);
//
// sas_remove_host  -  tear down a Scsi_Host's SAS data structures
// @shost:	Scsi Host that is torn down
//
// Removes all SAS PHYs and remote PHYs for a given Scsi_Host and remove the
// Scsi_Host as well.
//
// Note: Do not call scsi_remove_host() on the Scsi_Host any more, as it is
// already removed.
//
#[no_mangle]
pub unsafe extern "C" fn sas_remove_host(shost: *mut Scsi_Host) {
    void sas_remove_host(struct Scsi_Host *shost)
    {
    sas_remove_children(&shost.shost_gendev);
    scsi_remove_host(shost);
    }
    EXPORT_SYMBOL(sas_remove_host);
//
// sas_get_address - return the SAS address of the device
// @sdev: scsi device
//
// Returns the SAS address of the scsi device
//
#[no_mangle]
pub unsafe extern "C" fn sas_get_address(sdev: *mut scsi_device) -> u64 {
    u64 sas_get_address(struct scsi_device *sdev)
    {
    struct sas_end_device *rdev = sas_sdev_to_rdev(sdev);
    return rdev.rphy.identify.sas_address;
    }
    EXPORT_SYMBOL(sas_get_address);
//
// sas_tlr_supported - checking TLR bit in vpd 0x90
// @sdev: scsi device struct
//
// Check Transport Layer Retries are supported or not.
// If vpd page 0x90 is present, TRL is supported.
//
    unsigned int
    sas_tlr_supported(struct scsi_device *sdev)
    {
    let mut vpd_len: c_int = 32;
    struct sas_end_device *rdev = sas_sdev_to_rdev(sdev);
    char *buffer = kzalloc(vpd_len, GFP_KERNEL);
    let mut ret: c_int = 0;
    if (!buffer)
    goto out;
    if (scsi_get_vpd_page(sdev, 0x90, buffer, vpd_len))
    goto out;
//
// Magic numbers: the VPD Protocol page (0x90)
// has a 4 byte header and then one entry per device port
// the TLR bit is at offset 8 on each port entry
// if we take the first port, that's at total offset 12
//
    ret = buffer[12] & 0x01;
    out:
    kfree(buffer);
    rdev.tlr_supported = ret;
    return ret;
    }
    EXPORT_SYMBOL_GPL(sas_tlr_supported);
//
// sas_disable_tlr - setting TLR flags
// @sdev: scsi device struct
//
// Seting tlr_enabled flag to 0.
//
    void
    sas_disable_tlr(struct scsi_device *sdev)
    {
    struct sas_end_device *rdev = sas_sdev_to_rdev(sdev);
    rdev.tlr_enabled = 0;
    }
    EXPORT_SYMBOL_GPL(sas_disable_tlr);
//
// sas_enable_tlr - setting TLR flags
// @sdev: scsi device struct
//
// Seting tlr_enabled flag 1.
//
#[no_mangle]
pub unsafe extern "C" fn sas_enable_tlr(sdev: *mut scsi_device) {
    void sas_enable_tlr(struct scsi_device *sdev)
    {
    let mut tlr_supported: c_uint = 0;
    tlr_supported  = sas_tlr_supported(sdev);
    if (tlr_supported) {
    struct sas_end_device *rdev = sas_sdev_to_rdev(sdev);
    rdev.tlr_enabled = 1;
    }
    return;
    }
    EXPORT_SYMBOL_GPL(sas_enable_tlr);
#[no_mangle]
pub unsafe extern "C" fn sas_is_tlr_enabled(sdev: *mut scsi_device) -> c_uint {
    unsigned int sas_is_tlr_enabled(struct scsi_device *sdev)
    {
    struct sas_end_device *rdev = sas_sdev_to_rdev(sdev);
    return rdev.tlr_enabled;
    }
    EXPORT_SYMBOL_GPL(sas_is_tlr_enabled);
//
// sas_ata_ncq_prio_supported - Check for ATA NCQ command priority support
// @sdev: SCSI device
//
// Check if an ATA device supports NCQ priority using VPD page 89h (ATA
// Information). Since this VPD page is implemented only for ATA devices,
// this function always returns false for SCSI devices.
//
#[no_mangle]
pub unsafe extern "C" fn sas_ata_ncq_prio_supported(sdev: *mut scsi_device) -> bool {
    bool sas_ata_ncq_prio_supported(struct scsi_device *sdev)
    {
    struct scsi_vpd *vpd;
    let mut ncq_prio_supported: bool = false;
    rcu_read_lock();
    vpd = rcu_dereference(sdev.vpd_pg89);
    if (vpd && vpd.len >= 214)
    ncq_prio_supported = (vpd.data[213] >> 4) & 1;
    rcu_read_unlock();
    return ncq_prio_supported;
    }
    EXPORT_SYMBOL_GPL(sas_ata_ncq_prio_supported);
//
// SAS Phy attributes
//

    static ssize_t								\
    show_sas_phy_##name(struct device *dev, 				\
    struct device_attribute *attr, char *buf)		\
    {									\
    struct sas_phy *phy = transport_class_to_phy(dev);		\
    \
    return snprintf(buf, 20, format_string, cast phy.field);	\
    }

    sas_phy_show_simple(field, name, format_string, (type))	\
#[no_mangle]
pub unsafe extern "C" fn DEVICE_ATTR(_arg: name, _arg: S_IRUGO, _arg: show_sas_phy_##name, _arg: NULL) -> static {
    static DEVICE_ATTR(name, S_IRUGO, show_sas_phy_##name, core::ptr::null_mut())

    static ssize_t								\
    show_sas_phy_##name(struct device *dev, 				\
    struct device_attribute *attr, char *buf)		\
    {									\
    struct sas_phy *phy = transport_class_to_phy(dev);		\
    \
    if (!phy.field)						\
    return snprintf(buf, 20, "none\n");			\
    return get_sas_protocol_names(phy.field, buf);		\
    }

    sas_phy_show_protocol(field, name)				\
#[no_mangle]
pub unsafe extern "C" fn DEVICE_ATTR(_arg: name, _arg: S_IRUGO, _arg: show_sas_phy_##name, _arg: NULL) -> static {
    static DEVICE_ATTR(name, S_IRUGO, show_sas_phy_##name, core::ptr::null_mut())

    static ssize_t								\
    show_sas_phy_##field(struct device *dev, 				\
    struct device_attribute *attr, char *buf)		\
    {									\
    struct sas_phy *phy = transport_class_to_phy(dev);		\
    \
    return get_sas_linkspeed_names(phy.field, buf);		\
    }
// Fudge to tell if we're minimum or maximum

    static ssize_t								\
    store_sas_phy_##field(struct device *dev, 				\
    struct device_attribute *attr, 			\
    const char *buf,	size_t count)			\
    {									\
    struct sas_phy *phy = transport_class_to_phy(dev);		\
    struct Scsi_Host *shost = dev_to_shost(phy.dev.parent);	\
    struct sas_internal *i = to_sas_internal(shost.transportt);	\
    u32 value;							\
    struct sas_phy_linkrates rates = {0};				\
    int error;							\
    \
    error = set_sas_linkspeed_names(&value, buf);			\
    if (error)							\
    return error;						\
    rates.field = value;						\
    error = i.f.set_phy_speed(phy, &rates);			\
    \
    return error ? error : count;					\
    }

    sas_phy_show_linkspeed(field)					\
    sas_phy_store_linkspeed(field)					\
    static DEVICE_ATTR(field, S_IRUGO, show_sas_phy_##field,		\
    store_sas_phy_##field)

    sas_phy_show_linkspeed(field)					\
#[no_mangle]
pub unsafe extern "C" fn DEVICE_ATTR(_arg: field, _arg: S_IRUGO, _arg: show_sas_phy_##field, _arg: NULL) -> static {
    static DEVICE_ATTR(field, S_IRUGO, show_sas_phy_##field, core::ptr::null_mut())

    static ssize_t								\
    show_sas_phy_##field(struct device *dev, 				\
    struct device_attribute *attr, char *buf)		\
    {									\
    struct sas_phy *phy = transport_class_to_phy(dev);		\
    struct Scsi_Host *shost = dev_to_shost(phy.dev.parent);	\
    struct sas_internal *i = to_sas_internal(shost.transportt);	\
    int error;							\
    \
    error = i.f.get_linkerrors ? i.f.get_linkerrors(phy) : 0;	\
    if (error)							\
    return error;						\
    return snprintf(buf, 20, "%u\n", phy.field);			\
    }

    sas_phy_show_linkerror(field)					\
#[no_mangle]
pub unsafe extern "C" fn DEVICE_ATTR(_arg: field, _arg: S_IRUGO, _arg: show_sas_phy_##field, _arg: NULL) -> static {
    static DEVICE_ATTR(field, S_IRUGO, show_sas_phy_##field, core::ptr::null_mut())
    static ssize_t
    show_sas_device_type(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct sas_phy *phy = transport_class_to_phy(dev);
    if (!phy.identify.device_type)
    return snprintf(buf, 20, "none\n");
    return get_sas_device_type_names(phy.identify.device_type, buf);
    }
    static DEVICE_ATTR(device_type, S_IRUGO, show_sas_device_type, core::ptr::null_mut());
    static ssize_t do_sas_phy_enable(struct device *dev,
    size_t count, int enable)
    {
    struct sas_phy *phy = transport_class_to_phy(dev);
    struct Scsi_Host *shost = dev_to_shost(phy.dev.parent);
    struct sas_internal *i = to_sas_internal(shost.transportt);
    int error;
    error = i.f.phy_enable(phy, enable);
    if (error)
    return error;
    phy.enabled = enable;
    return count;
    };
    static ssize_t
    store_sas_phy_enable(struct device *dev, struct device_attribute *attr,
    const char *buf, size_t count)
    {
    if (count < 1)
    return -EINVAL;
    switch (buf[0]) {
    case '0':
    do_sas_phy_enable(dev, count, 0);
    break;
    case '1':
    do_sas_phy_enable(dev, count, 1);
    break;
    default:
    return -EINVAL;
    }
    return count;
    }
    static ssize_t
    show_sas_phy_enable(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    struct sas_phy *phy = transport_class_to_phy(dev);
    return snprintf(buf, 20, "%d\n", phy.enabled);
    }
    static DEVICE_ATTR(enable, S_IRUGO | S_IWUSR, show_sas_phy_enable,
    store_sas_phy_enable);
    static ssize_t
    do_sas_phy_reset(struct device *dev, size_t count, int hard_reset)
    {
    struct sas_phy *phy = transport_class_to_phy(dev);
    struct Scsi_Host *shost = dev_to_shost(phy.dev.parent);
    struct sas_internal *i = to_sas_internal(shost.transportt);
    int error;
    error = i.f.phy_reset(phy, hard_reset);
    if (error)
    return error;
    phy.enabled = 1;
    return count;
    };
    static ssize_t
    store_sas_link_reset(struct device *dev, struct device_attribute *attr,
    const char *buf, size_t count)
    {
    return do_sas_phy_reset(dev, count, 0);
    }
    static DEVICE_ATTR(link_reset, S_IWUSR, core::ptr::null_mut(), store_sas_link_reset);
    static ssize_t
    store_sas_hard_reset(struct device *dev, struct device_attribute *attr,
    const char *buf, size_t count)
    {
    return do_sas_phy_reset(dev, count, 1);
    }
    static DEVICE_ATTR(hard_reset, S_IWUSR, core::ptr::null_mut(), store_sas_hard_reset);
    sas_phy_protocol_attr(identify.initiator_port_protocols,
    initiator_port_protocols);
    sas_phy_protocol_attr(identify.target_port_protocols,
    target_port_protocols);
    sas_phy_simple_attr(identify.sas_address, sas_address, "0x%016llx\n",
    unsigned long long);
    sas_phy_simple_attr(identify.phy_identifier, phy_identifier, "%d\n", u8);
    sas_phy_linkspeed_attr(negotiated_linkrate);
    sas_phy_linkspeed_attr(minimum_linkrate_hw);
    sas_phy_linkspeed_rw_attr(minimum_linkrate);
    sas_phy_linkspeed_attr(maximum_linkrate_hw);
    sas_phy_linkspeed_rw_attr(maximum_linkrate);
    sas_phy_linkerror_attr(invalid_dword_count);
    sas_phy_linkerror_attr(running_disparity_error_count);
    sas_phy_linkerror_attr(loss_of_dword_sync_count);
    sas_phy_linkerror_attr(phy_reset_problem_count);
    static int sas_phy_setup(struct transport_container *tc, struct device *dev,
    struct device *cdev)
    {
    struct sas_phy *phy = dev_to_phy(dev);
    struct Scsi_Host *shost = dev_to_shost(phy.dev.parent);
    struct sas_internal *i = to_sas_internal(shost.transportt);
    if (i.f.phy_setup)
    i.f.phy_setup(phy);
    return 0;
    }
    static DECLARE_TRANSPORT_CLASS(sas_phy_class,
    "sas_phy", sas_phy_setup, core::ptr::null_mut(), core::ptr::null_mut());
#[no_mangle]
unsafe extern "C" fn sas_phy_match(cont: *mut attribute_container, dev: *mut device) -> c_int {
    static int sas_phy_match(struct attribute_container *cont, struct device *dev)
    {
    struct Scsi_Host *shost;
    struct sas_internal *i;
    if (!scsi_is_sas_phy(dev))
    return 0;
    shost = dev_to_shost(dev.parent);
    if (!shost.transportt)
    return 0;
    if (shost.transportt.host_attrs.ac.class !=
    &sas_host_class.class)
    return 0;
    i = to_sas_internal(shost.transportt);
    return &i.phy_attr_cont.ac == cont;
    }
#[no_mangle]
unsafe extern "C" fn sas_phy_release(dev: *mut device) {
    static void sas_phy_release(struct device *dev)
    {
    struct sas_phy *phy = dev_to_phy(dev);
    struct Scsi_Host *shost = dev_to_shost(phy.dev.parent);
    struct sas_internal *i = to_sas_internal(shost.transportt);
    if (i.f.phy_release)
    i.f.phy_release(phy);
    put_device(dev.parent);
    kfree(phy);
    }
//
// sas_phy_alloc  -  allocates and initialize a SAS PHY structure
// @parent:	Parent device
// @number:	Phy index
//
// Allocates an SAS PHY structure.  It will be added in the device tree
// below the device specified by @parent, which has to be either a Scsi_Host
// or sas_rphy.
//
// Returns:
// SAS PHY allocated or %NULL if the allocation failed.
//
    struct sas_phy *sas_phy_alloc(struct device *parent, int number)
    {
    struct Scsi_Host *shost = dev_to_shost(parent);
    struct sas_phy *phy;
    phy = kzalloc_obj(*phy);
    if (!phy)
    return core::ptr::null_mut();
    phy.number = number;
    phy.enabled = 1;
    device_initialize(&phy.dev);
    phy.dev.parent = get_device(parent);
    phy.dev.release = sas_phy_release;
    INIT_LIST_HEAD(&phy.port_siblings);
    if (scsi_is_sas_expander_device(parent)) {
    struct sas_rphy *rphy = dev_to_rphy(parent);
    dev_set_name(&phy.dev, "phy-%d:%d:%d", shost.host_no,
    rphy.scsi_target_id, number);
    } else
    dev_set_name(&phy.dev, "phy-%d:%d", shost.host_no, number);
    transport_setup_device(&phy.dev);
    return phy;
    }
    EXPORT_SYMBOL(sas_phy_alloc);
//
// sas_phy_add  -  add a SAS PHY to the device hierarchy
// @phy:	The PHY to be added
//
// Publishes a SAS PHY to the rest of the system.
//
#[no_mangle]
pub unsafe extern "C" fn sas_phy_add(phy: *mut sas_phy) -> c_int {
    int sas_phy_add(struct sas_phy *phy)
    {
    int error;
    error = device_add(&phy.dev);
    if (error)
    return error;
    error = transport_add_device(&phy.dev);
    if (error) {
    device_del(&phy.dev);
    return error;
    }
    transport_configure_device(&phy.dev);
    return 0;
    }
    EXPORT_SYMBOL(sas_phy_add);
//
// sas_phy_free  -  free a SAS PHY
// @phy:	SAS PHY to free
//
// Frees the specified SAS PHY.
//
// Note:
// This function must only be called on a PHY that has not
// successfully been added using sas_phy_add().
//
#[no_mangle]
pub unsafe extern "C" fn sas_phy_free(phy: *mut sas_phy) {
    void sas_phy_free(struct sas_phy *phy)
    {
    transport_destroy_device(&phy.dev);
    put_device(&phy.dev);
    }
    EXPORT_SYMBOL(sas_phy_free);
//
// sas_phy_delete  -  remove SAS PHY
// @phy:	SAS PHY to remove
//
// Removes the specified SAS PHY.  If the SAS PHY has an
// associated remote PHY it is removed before.
//
    void
    sas_phy_delete(struct sas_phy *phy)
    {
    struct device *dev = &phy.dev;
// this happens if the phy is still part of a port when deleted
    BUG_ON(!list_empty(&phy.port_siblings));
    transport_remove_device(dev);
    device_del(dev);
    transport_destroy_device(dev);
    put_device(dev);
    }
    EXPORT_SYMBOL(sas_phy_delete);
//
// scsi_is_sas_phy  -  check if a struct device represents a SAS PHY
// @dev:	device to check
//
// Returns:
// %1 if the device represents a SAS PHY, %0 else
//
#[no_mangle]
pub unsafe extern "C" fn scsi_is_sas_phy(dev: *const device) -> c_int {
    int scsi_is_sas_phy(const struct device *dev)
    {
    return dev.release == sas_phy_release;
    }
    EXPORT_SYMBOL(scsi_is_sas_phy);
//
// SAS Port attributes
//

    static ssize_t								\
    show_sas_port_##name(struct device *dev, 				\
    struct device_attribute *attr, char *buf)		\
    {									\
    struct sas_port *port = transport_class_to_sas_port(dev);	\
    \
    return snprintf(buf, 20, format_string, cast port.field);	\
    }

    sas_port_show_simple(field, name, format_string, (type))	\
#[no_mangle]
pub unsafe extern "C" fn DEVICE_ATTR(_arg: name, _arg: S_IRUGO, _arg: show_sas_port_##name, _arg: NULL) -> static {
    static DEVICE_ATTR(name, S_IRUGO, show_sas_port_##name, core::ptr::null_mut())
    sas_port_simple_attr(num_phys, num_phys, "%d\n", int);
    static DECLARE_TRANSPORT_CLASS(sas_port_class,
    "sas_port", core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut());
#[no_mangle]
unsafe extern "C" fn sas_port_match(cont: *mut attribute_container, dev: *mut device) -> c_int {
    static int sas_port_match(struct attribute_container *cont, struct device *dev)
    {
    struct Scsi_Host *shost;
    struct sas_internal *i;
    if (!scsi_is_sas_port(dev))
    return 0;
    shost = dev_to_shost(dev.parent);
    if (!shost.transportt)
    return 0;
    if (shost.transportt.host_attrs.ac.class !=
    &sas_host_class.class)
    return 0;
    i = to_sas_internal(shost.transportt);
    return &i.port_attr_cont.ac == cont;
    }
#[no_mangle]
unsafe extern "C" fn sas_port_release(dev: *mut device) {
    static void sas_port_release(struct device *dev)
    {
    struct sas_port *port = dev_to_sas_port(dev);
    BUG_ON(!list_empty(&port.phy_list));
    put_device(dev.parent);
    kfree(port);
    }
    static void sas_port_create_link(struct sas_port *port,
    struct sas_phy *phy)
    {
    int res;
    res = sysfs_create_link(&port.dev.kobj, &phy.dev.kobj,
    dev_name(&phy.dev));
    if (res)
    goto err;
    res = sysfs_create_link(&phy.dev.kobj, &port.dev.kobj, "port");
    if (res)
    goto err;
    return;
    err:
    printk(KERN_ERR "%s: Cannot create port links, err=%d\n",
    __func__, res);
    }
    static void sas_port_delete_link(struct sas_port *port,
    struct sas_phy *phy)
    {
    sysfs_remove_link(&port.dev.kobj, dev_name(&phy.dev));
    sysfs_remove_link(&phy.dev.kobj, "port");
    }
//
// sas_port_alloc - allocate and initialize a SAS port structure
//
// @parent:	parent device
// @port_id:	port number
//
// Allocates a SAS port structure.  It will be added to the device tree
// below the device specified by @parent which must be either a Scsi_Host
// or a sas_expander_device.
//
// Returns: %NULL on error
//
    struct sas_port *sas_port_alloc(struct device *parent, int port_id)
    {
    struct Scsi_Host *shost = dev_to_shost(parent);
    struct sas_port *port;
    port = kzalloc_obj(*port);
    if (!port)
    return core::ptr::null_mut();
    port.port_identifier = port_id;
    device_initialize(&port.dev);
    port.dev.parent = get_device(parent);
    port.dev.release = sas_port_release;
    mutex_init(&port.phy_list_mutex);
    INIT_LIST_HEAD(&port.phy_list);
    if (scsi_is_sas_expander_device(parent)) {
    struct sas_rphy *rphy = dev_to_rphy(parent);
    dev_set_name(&port.dev, "port-%d:%d:%d", shost.host_no,
    rphy.scsi_target_id, port.port_identifier);
    } else
    dev_set_name(&port.dev, "port-%d:%d", shost.host_no,
    port.port_identifier);
    transport_setup_device(&port.dev);
    return port;
    }
    EXPORT_SYMBOL(sas_port_alloc);
//
// sas_port_alloc_num - allocate and initialize a SAS port structure
//
// @parent:	parent device
//
// Allocates a SAS port structure and a number to go with it.  This
// interface is really for adapters where the port number has no
// meansing, so the sas class should manage them.  It will be added to
// the device tree below the device specified by @parent which must be
// either a Scsi_Host or a sas_expander_device.
//
// Returns: %NULL on error
//
    struct sas_port *sas_port_alloc_num(struct device *parent)
    {
    int index;
    struct Scsi_Host *shost = dev_to_shost(parent);
    struct sas_host_attrs *sas_host = to_sas_host_attrs(shost);
// FIXME: use idr for this eventually
    mutex_lock(&sas_host.lock);
    if (scsi_is_sas_expander_device(parent)) {
    struct sas_rphy *rphy = dev_to_rphy(parent);
    struct sas_expander_device *exp = rphy_to_expander_device(rphy);
    index = exp.next_port_id++;
    } else
    index = sas_host.next_port_id++;
    mutex_unlock(&sas_host.lock);
    return sas_port_alloc(parent, index);
    }
    EXPORT_SYMBOL(sas_port_alloc_num);
//
// sas_port_add - add a SAS port to the device hierarchy
// @port:	port to be added
//
// publishes a port to the rest of the system
//
#[no_mangle]
pub unsafe extern "C" fn sas_port_add(port: *mut sas_port) -> c_int {
    int sas_port_add(struct sas_port *port)
    {
    int error;
// No phys should be added until this is made visible
    BUG_ON(!list_empty(&port.phy_list));
    error = device_add(&port.dev);
    if (error)
    return error;
    transport_add_device(&port.dev);
    transport_configure_device(&port.dev);
    return 0;
    }
    EXPORT_SYMBOL(sas_port_add);
//
// sas_port_free  -  free a SAS PORT
// @port:	SAS PORT to free
//
// Frees the specified SAS PORT.
//
// Note:
// This function must only be called on a PORT that has not
// successfully been added using sas_port_add().
//
#[no_mangle]
pub unsafe extern "C" fn sas_port_free(port: *mut sas_port) {
    void sas_port_free(struct sas_port *port)
    {
    transport_destroy_device(&port.dev);
    put_device(&port.dev);
    }
    EXPORT_SYMBOL(sas_port_free);
//
// sas_port_delete  -  remove SAS PORT
// @port:	SAS PORT to remove
//
// Removes the specified SAS PORT.  If the SAS PORT has an
// associated phys, unlink them from the port as well.
//
#[no_mangle]
pub unsafe extern "C" fn sas_port_delete(port: *mut sas_port) {
    void sas_port_delete(struct sas_port *port)
    {
    struct device *dev = &port.dev;
    struct sas_phy *phy, *tmp_phy;
    if (port.rphy) {
    sas_rphy_delete(port.rphy);
    port.rphy = core::ptr::null_mut();
    }
    mutex_lock(&port.phy_list_mutex);
    list_for_each_entry_safe(phy, tmp_phy, &port.phy_list,
    port_siblings) {
    sas_port_delete_link(port, phy);
    list_del_init(&phy.port_siblings);
    }
    mutex_unlock(&port.phy_list_mutex);
    if (port.is_backlink) {
    struct device *parent = port.dev.parent;
    sysfs_remove_link(&port.dev.kobj, dev_name(parent));
    port.is_backlink = 0;
    }
    transport_remove_device(dev);
    device_del(dev);
    transport_destroy_device(dev);
    put_device(dev);
    }
    EXPORT_SYMBOL(sas_port_delete);
//
// scsi_is_sas_port -  check if a struct device represents a SAS port
// @dev:	device to check
//
// Returns:
// %1 if the device represents a SAS Port, %0 else
//
#[no_mangle]
pub unsafe extern "C" fn scsi_is_sas_port(dev: *const device) -> c_int {
    int scsi_is_sas_port(const struct device *dev)
    {
    return dev.release == sas_port_release;
    }
    EXPORT_SYMBOL(scsi_is_sas_port);
//
// sas_port_get_phy - try to take a reference on a port member
// @port: port to check
//
    struct sas_phy *sas_port_get_phy(struct sas_port *port)
    {
    struct sas_phy *phy;
    mutex_lock(&port.phy_list_mutex);
    if (list_empty(&port.phy_list))
    phy = core::ptr::null_mut();
    else {
    struct list_head *ent = port.phy_list.next;
    phy = list_entry(ent, typeof(*phy), port_siblings);
    get_device(&phy.dev);
    }
    mutex_unlock(&port.phy_list_mutex);
    return phy;
    }
    EXPORT_SYMBOL(sas_port_get_phy);
//
// sas_port_add_phy - add another phy to a port to form a wide port
// @port:	port to add the phy to
// @phy:	phy to add
//
// When a port is initially created, it is empty (has no phys).  All
// ports must have at least one phy to operated, and all wide ports
// must have at least two.  The current code makes no difference
// between ports and wide ports, but the only object that can be
// connected to a remote device is a port, so ports must be formed on
// all devices with phys if they're connected to anything.
//
#[no_mangle]
pub unsafe extern "C" fn sas_port_add_phy(port: *mut sas_port, phy: *mut sas_phy) {
    void sas_port_add_phy(struct sas_port *port, struct sas_phy *phy)
    {
    mutex_lock(&port.phy_list_mutex);
    if (unlikely(!list_empty(&phy.port_siblings))) {
// make sure we're already on this port
    struct sas_phy *tmp;
    list_for_each_entry(tmp, &port.phy_list, port_siblings)
    if (tmp == phy)
    break;
// If this trips, you added a phy that was already
// part of a different port
    if (unlikely(tmp != phy)) {
    dev_printk(KERN_ERR, &port.dev, "trying to add phy %s fails: it's already part of another port\n",
    dev_name(&phy.dev));
    BUG();
    }
    } else {
    sas_port_create_link(port, phy);
    list_add_tail(&phy.port_siblings, &port.phy_list);
    port.num_phys++;
    }
    mutex_unlock(&port.phy_list_mutex);
    }
    EXPORT_SYMBOL(sas_port_add_phy);
//
// sas_port_delete_phy - remove a phy from a port or wide port
// @port:	port to remove the phy from
// @phy:	phy to remove
//
// This operation is used for tearing down ports again.  It must be
// done to every port or wide port before calling sas_port_delete.
//
#[no_mangle]
pub unsafe extern "C" fn sas_port_delete_phy(port: *mut sas_port, phy: *mut sas_phy) {
    void sas_port_delete_phy(struct sas_port *port, struct sas_phy *phy)
    {
    mutex_lock(&port.phy_list_mutex);
    sas_port_delete_link(port, phy);
    list_del_init(&phy.port_siblings);
    port.num_phys--;
    mutex_unlock(&port.phy_list_mutex);
    }
    EXPORT_SYMBOL(sas_port_delete_phy);
#[no_mangle]
pub unsafe extern "C" fn sas_port_mark_backlink(port: *mut sas_port) {
    void sas_port_mark_backlink(struct sas_port *port)
    {
    int res;
    struct device *parent = port.dev.parent.parent.parent;
    if (port.is_backlink)
    return;
    port.is_backlink = 1;
    res = sysfs_create_link(&port.dev.kobj, &parent.kobj,
    dev_name(parent));
    if (res)
    goto err;
    return;
    err:
    printk(KERN_ERR "%s: Cannot create port backlink, err=%d\n",
    __func__, res);
    }
    EXPORT_SYMBOL(sas_port_mark_backlink);
//
// SAS remote PHY attributes.
//

    static ssize_t								\
    show_sas_rphy_##name(struct device *dev, 				\
    struct device_attribute *attr, char *buf)		\
    {									\
    struct sas_rphy *rphy = transport_class_to_rphy(dev);		\
    \
    return snprintf(buf, 20, format_string, cast rphy.field);	\
    }

    sas_rphy_show_simple(field, name, format_string, (type))	\
    static SAS_DEVICE_ATTR(rphy, name, S_IRUGO, 			\
    show_sas_rphy_##name, core::ptr::null_mut())

    static ssize_t								\
    show_sas_rphy_##name(struct device *dev, 				\
    struct device_attribute *attr, char *buf)		\
    {									\
    struct sas_rphy *rphy = transport_class_to_rphy(dev);		\
    \
    if (!rphy.field)					\
    return snprintf(buf, 20, "none\n");			\
    return get_sas_protocol_names(rphy.field, buf);	\
    }

    sas_rphy_show_protocol(field, name)				\
    static SAS_DEVICE_ATTR(rphy, name, S_IRUGO,			\
    show_sas_rphy_##name, core::ptr::null_mut())
    static ssize_t
    show_sas_rphy_device_type(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct sas_rphy *rphy = transport_class_to_rphy(dev);
    if (!rphy.identify.device_type)
    return snprintf(buf, 20, "none\n");
    return get_sas_device_type_names(
    rphy.identify.device_type, buf);
    }
    static SAS_DEVICE_ATTR(rphy, device_type, S_IRUGO,
    show_sas_rphy_device_type, core::ptr::null_mut());
    static ssize_t
    show_sas_rphy_enclosure_identifier(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct sas_rphy *rphy = transport_class_to_rphy(dev);
    struct sas_phy *phy = dev_to_phy(rphy.dev.parent);
    struct Scsi_Host *shost = dev_to_shost(phy.dev.parent);
    struct sas_internal *i = to_sas_internal(shost.transportt);
    u64 identifier;
    int error;
    error = i.f.get_enclosure_identifier(rphy, &identifier);
    if (error)
    return error;
    return sprintf(buf, "0x%llx\n", (unsigned long long)identifier);
    }
    static SAS_DEVICE_ATTR(rphy, enclosure_identifier, S_IRUGO,
    show_sas_rphy_enclosure_identifier, core::ptr::null_mut());
    static ssize_t
    show_sas_rphy_bay_identifier(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct sas_rphy *rphy = transport_class_to_rphy(dev);
    struct sas_phy *phy = dev_to_phy(rphy.dev.parent);
    struct Scsi_Host *shost = dev_to_shost(phy.dev.parent);
    struct sas_internal *i = to_sas_internal(shost.transportt);
    int val;
    val = i.f.get_bay_identifier(rphy);
    if (val < 0)
    return val;
    return sprintf(buf, "%d\n", val);
    }
    static SAS_DEVICE_ATTR(rphy, bay_identifier, S_IRUGO,
    show_sas_rphy_bay_identifier, core::ptr::null_mut());
    sas_rphy_protocol_attr(identify.initiator_port_protocols,
    initiator_port_protocols);
    sas_rphy_protocol_attr(identify.target_port_protocols, target_port_protocols);
    sas_rphy_simple_attr(identify.sas_address, sas_address, "0x%016llx\n",
    unsigned long long);
    sas_rphy_simple_attr(identify.phy_identifier, phy_identifier, "%d\n", u8);
    sas_rphy_simple_attr(scsi_target_id, scsi_target_id, "%d\n", u32);
// only need 8 bytes of data plus header (4 or 8)
pub const BUF_SIZE: c_int = 64;
#[no_mangle]
pub unsafe extern "C" fn sas_read_port_mode_page(sdev: *mut scsi_device) -> c_int {
    int sas_read_port_mode_page(struct scsi_device *sdev)
    {
    char *buffer = kzalloc(BUF_SIZE, GFP_KERNEL), *msdata;
    struct sas_end_device *rdev = sas_sdev_to_rdev(sdev);
    struct scsi_mode_data mode_data;
    int error;
    if (!buffer)
    return -ENOMEM;
    error = scsi_mode_sense(sdev, 1, 0x19, 0, buffer, BUF_SIZE, 30*HZ, 3,
    &mode_data, core::ptr::null_mut());
    if (error)
    goto out;
    msdata = buffer +  mode_data.header_length +
    mode_data.block_descriptor_length;
    if (msdata - buffer > BUF_SIZE - 8)
    goto out;
    error = 0;
    rdev.ready_led_meaning = msdata[2] & 0x10 ? 1 : 0;
    rdev.I_T_nexus_loss_timeout = (msdata[4] << 8) + msdata[5];
    rdev.initiator_response_timeout = (msdata[6] << 8) + msdata[7];
    out:
    kfree(buffer);
    return error;
    }
    EXPORT_SYMBOL(sas_read_port_mode_page);
    static DECLARE_TRANSPORT_CLASS(sas_end_dev_class,
    "sas_end_device", core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut());

    static ssize_t								\
    show_sas_end_dev_##name(struct device *dev, 				\
    struct device_attribute *attr, char *buf)	\
    {									\
    struct sas_rphy *rphy = transport_class_to_rphy(dev);		\
    struct sas_end_device *rdev = rphy_to_end_device(rphy);		\
    \
    return snprintf(buf, 20, format_string, cast rdev.field);	\
    }

    sas_end_dev_show_simple(field, name, format_string, (type))	\
    static SAS_DEVICE_ATTR(end_dev, name, S_IRUGO, 			\
    show_sas_end_dev_##name, core::ptr::null_mut())
    sas_end_dev_simple_attr(ready_led_meaning, ready_led_meaning, "%d\n", int);
    sas_end_dev_simple_attr(I_T_nexus_loss_timeout, I_T_nexus_loss_timeout,
    "%d\n", int);
    sas_end_dev_simple_attr(initiator_response_timeout, initiator_response_timeout,
    "%d\n", int);
    sas_end_dev_simple_attr(tlr_supported, tlr_supported,
    "%d\n", int);
    sas_end_dev_simple_attr(tlr_enabled, tlr_enabled,
    "%d\n", int);
    static DECLARE_TRANSPORT_CLASS(sas_expander_class,
    "sas_expander", core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut());

    static ssize_t								\
    show_sas_expander_##name(struct device *dev, 				\
    struct device_attribute *attr, char *buf)	\
    {									\
    struct sas_rphy *rphy = transport_class_to_rphy(dev);		\
    struct sas_expander_device *edev = rphy_to_expander_device(rphy); \
    \
    return snprintf(buf, 20, format_string, cast edev.field);	\
    }

    sas_expander_show_simple(field, name, format_string, (type))	\
    static SAS_DEVICE_ATTR(expander, name, S_IRUGO, 			\
    show_sas_expander_##name, core::ptr::null_mut())
    sas_expander_simple_attr(vendor_id, vendor_id, "%s\n", char *);
    sas_expander_simple_attr(product_id, product_id, "%s\n", char *);
    sas_expander_simple_attr(product_rev, product_rev, "%s\n", char *);
    sas_expander_simple_attr(component_vendor_id, component_vendor_id,
    "%s\n", char *);
    sas_expander_simple_attr(component_id, component_id, "%u\n", unsigned int);
    sas_expander_simple_attr(component_revision_id, component_revision_id, "%u\n",
    unsigned int);
    sas_expander_simple_attr(level, level, "%d\n", int);
    static DECLARE_TRANSPORT_CLASS(sas_rphy_class,
    "sas_device", core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut());
#[no_mangle]
unsafe extern "C" fn sas_rphy_match(cont: *mut attribute_container, dev: *mut device) -> c_int {
    static int sas_rphy_match(struct attribute_container *cont, struct device *dev)
    {
    struct Scsi_Host *shost;
    struct sas_internal *i;
    if (!scsi_is_sas_rphy(dev))
    return 0;
    shost = dev_to_shost(dev.parent.parent);
    if (!shost.transportt)
    return 0;
    if (shost.transportt.host_attrs.ac.class !=
    &sas_host_class.class)
    return 0;
    i = to_sas_internal(shost.transportt);
    return &i.rphy_attr_cont.ac == cont;
    }
    static int sas_end_dev_match(struct attribute_container *cont,
    struct device *dev)
    {
    struct Scsi_Host *shost;
    struct sas_internal *i;
    struct sas_rphy *rphy;
    if (!scsi_is_sas_rphy(dev))
    return 0;
    shost = dev_to_shost(dev.parent.parent);
    rphy = dev_to_rphy(dev);
    if (!shost.transportt)
    return 0;
    if (shost.transportt.host_attrs.ac.class !=
    &sas_host_class.class)
    return 0;
    i = to_sas_internal(shost.transportt);
    return &i.end_dev_attr_cont.ac == cont &&
    rphy.identify.device_type == SAS_END_DEVICE;
    }
    static int sas_expander_match(struct attribute_container *cont,
    struct device *dev)
    {
    struct Scsi_Host *shost;
    struct sas_internal *i;
    struct sas_rphy *rphy;
    if (!scsi_is_sas_rphy(dev))
    return 0;
    shost = dev_to_shost(dev.parent.parent);
    rphy = dev_to_rphy(dev);
    if (!shost.transportt)
    return 0;
    if (shost.transportt.host_attrs.ac.class !=
    &sas_host_class.class)
    return 0;
    i = to_sas_internal(shost.transportt);
    return &i.expander_attr_cont.ac == cont &&
    (rphy.identify.device_type == SAS_EDGE_EXPANDER_DEVICE ||
    rphy.identify.device_type == SAS_FANOUT_EXPANDER_DEVICE);
    }
#[no_mangle]
unsafe extern "C" fn sas_expander_release(dev: *mut device) {
    static void sas_expander_release(struct device *dev)
    {
    struct sas_rphy *rphy = dev_to_rphy(dev);
    struct sas_expander_device *edev = rphy_to_expander_device(rphy);
    put_device(dev.parent);
    kfree(edev);
    }
#[no_mangle]
unsafe extern "C" fn sas_end_device_release(dev: *mut device) {
    static void sas_end_device_release(struct device *dev)
    {
    struct sas_rphy *rphy = dev_to_rphy(dev);
    struct sas_end_device *edev = rphy_to_end_device(rphy);
    put_device(dev.parent);
    kfree(edev);
    }
//
// sas_rphy_initialize - common rphy initialization
// @rphy:	rphy to initialise
//
// Used by both sas_end_device_alloc() and sas_expander_alloc() to
// initialise the common rphy component of each.
//
#[no_mangle]
unsafe extern "C" fn sas_rphy_initialize(rphy: *mut sas_rphy) {
    static void sas_rphy_initialize(struct sas_rphy *rphy)
    {
    INIT_LIST_HEAD(&rphy.list);
    }
//
// sas_end_device_alloc - allocate an rphy for an end device
// @parent: which port
//
// Allocates an SAS remote PHY structure, connected to @parent.
//
// Returns:
// SAS PHY allocated or %NULL if the allocation failed.
//
    struct sas_rphy *sas_end_device_alloc(struct sas_port *parent)
    {
    struct Scsi_Host *shost = dev_to_shost(&parent.dev);
    struct sas_end_device *rdev;
    rdev = kzalloc_obj(*rdev);
    if (!rdev) {
    return core::ptr::null_mut();
    }
    device_initialize(&rdev.rphy.dev);
    rdev.rphy.dev.parent = get_device(&parent.dev);
    rdev.rphy.dev.release = sas_end_device_release;
    if (scsi_is_sas_expander_device(parent.dev.parent)) {
    struct sas_rphy *rphy = dev_to_rphy(parent.dev.parent);
    dev_set_name(&rdev.rphy.dev, "end_device-%d:%d:%d",
    shost.host_no, rphy.scsi_target_id,
    parent.port_identifier);
    } else
    dev_set_name(&rdev.rphy.dev, "end_device-%d:%d",
    shost.host_no, parent.port_identifier);
    rdev.rphy.identify.device_type = SAS_END_DEVICE;
    sas_rphy_initialize(&rdev.rphy);
    transport_setup_device(&rdev.rphy.dev);
    return &rdev.rphy;
    }
    EXPORT_SYMBOL(sas_end_device_alloc);
//
// sas_expander_alloc - allocate an rphy for an end device
// @parent: which port
// @type: SAS_EDGE_EXPANDER_DEVICE or SAS_FANOUT_EXPANDER_DEVICE
//
// Allocates an SAS remote PHY structure, connected to @parent.
//
// Returns:
// SAS PHY allocated or %NULL if the allocation failed.
//
    struct sas_rphy *sas_expander_alloc(struct sas_port *parent,
    enum sas_device_type type)
    {
    struct Scsi_Host *shost = dev_to_shost(&parent.dev);
    struct sas_expander_device *rdev;
    struct sas_host_attrs *sas_host = to_sas_host_attrs(shost);
    BUG_ON(type != SAS_EDGE_EXPANDER_DEVICE &&
    type != SAS_FANOUT_EXPANDER_DEVICE);
    rdev = kzalloc_obj(*rdev);
    if (!rdev) {
    return core::ptr::null_mut();
    }
    device_initialize(&rdev.rphy.dev);
    rdev.rphy.dev.parent = get_device(&parent.dev);
    rdev.rphy.dev.release = sas_expander_release;
    mutex_lock(&sas_host.lock);
    rdev.rphy.scsi_target_id = sas_host.next_expander_id++;
    mutex_unlock(&sas_host.lock);
    dev_set_name(&rdev.rphy.dev, "expander-%d:%d",
    shost.host_no, rdev.rphy.scsi_target_id);
    rdev.rphy.identify.device_type = type;
    sas_rphy_initialize(&rdev.rphy);
    transport_setup_device(&rdev.rphy.dev);
    return &rdev.rphy;
    }
    EXPORT_SYMBOL(sas_expander_alloc);
//
// sas_rphy_add  -  add a SAS remote PHY to the device hierarchy
// @rphy:	The remote PHY to be added
//
// Publishes a SAS remote PHY to the rest of the system.
//
#[no_mangle]
pub unsafe extern "C" fn sas_rphy_add(rphy: *mut sas_rphy) -> c_int {
    int sas_rphy_add(struct sas_rphy *rphy)
    {
    struct sas_port *parent = dev_to_sas_port(rphy.dev.parent);
    struct Scsi_Host *shost = dev_to_shost(parent.dev.parent);
    struct sas_host_attrs *sas_host = to_sas_host_attrs(shost);
    struct sas_identify *identify = &rphy.identify;
    int error;
    if (parent.rphy)
    return -ENXIO;
    parent.rphy = rphy;
    error = device_add(&rphy.dev);
    if (error)
    return error;
    transport_add_device(&rphy.dev);
    transport_configure_device(&rphy.dev);
    if (sas_bsg_initialize(shost, rphy))
    printk("fail to a bsg device %s\n", dev_name(&rphy.dev));
    mutex_lock(&sas_host.lock);
    list_add_tail(&rphy.list, &sas_host.rphy_list);
    if (identify.device_type == SAS_END_DEVICE &&
    (identify.target_port_protocols &
    (SAS_PROTOCOL_SSP | SAS_PROTOCOL_STP | SAS_PROTOCOL_SATA)))
    rphy.scsi_target_id = sas_host.next_target_id++;
#[no_mangle]
pub unsafe extern "C" fn if(SAS_END_DEVICE: identify->device_type ==) -> else {
    else if (identify.device_type == SAS_END_DEVICE)
    rphy.scsi_target_id = -1;
    mutex_unlock(&sas_host.lock);
    if (identify.device_type == SAS_END_DEVICE &&
    rphy.scsi_target_id != -1) {
    int lun;
    if (identify.target_port_protocols & SAS_PROTOCOL_SSP)
    lun = SCAN_WILD_CARD;
    else
    lun = 0;
    scsi_scan_target(&rphy.dev, 0, rphy.scsi_target_id, lun,
    SCSI_SCAN_INITIAL);
    }
    return 0;
    }
    EXPORT_SYMBOL(sas_rphy_add);
//
// sas_rphy_free  -  free a SAS remote PHY
// @rphy: SAS remote PHY to free
//
// Frees the specified SAS remote PHY.
//
// Note:
// This function must only be called on a remote
// PHY that has not successfully been added using
// sas_rphy_add() (or has been sas_rphy_remove()'d)
//
#[no_mangle]
pub unsafe extern "C" fn sas_rphy_free(rphy: *mut sas_rphy) {
    void sas_rphy_free(struct sas_rphy *rphy)
    {
    struct device *dev = &rphy.dev;
    struct Scsi_Host *shost = dev_to_shost(rphy.dev.parent.parent);
    struct sas_host_attrs *sas_host = to_sas_host_attrs(shost);
    mutex_lock(&sas_host.lock);
    list_del(&rphy.list);
    mutex_unlock(&sas_host.lock);
    transport_destroy_device(dev);
    put_device(dev);
    }
    EXPORT_SYMBOL(sas_rphy_free);
//
// sas_rphy_delete  -  remove and free SAS remote PHY
// @rphy:	SAS remote PHY to remove and free
//
// Removes the specified SAS remote PHY and frees it.
//
    void
    sas_rphy_delete(struct sas_rphy *rphy)
    {
    sas_rphy_remove(rphy);
    sas_rphy_free(rphy);
    }
    EXPORT_SYMBOL(sas_rphy_delete);
//
// sas_rphy_unlink  -  unlink SAS remote PHY
// @rphy:	SAS remote phy to unlink from its parent port
//
// Removes port reference to an rphy
//
#[no_mangle]
pub unsafe extern "C" fn sas_rphy_unlink(rphy: *mut sas_rphy) {
    void sas_rphy_unlink(struct sas_rphy *rphy)
    {
    struct sas_port *parent = dev_to_sas_port(rphy.dev.parent);
    parent.rphy = core::ptr::null_mut();
    }
    EXPORT_SYMBOL(sas_rphy_unlink);
//
// sas_rphy_remove  -  remove SAS remote PHY
// @rphy:	SAS remote phy to remove
//
// Removes the specified SAS remote PHY.
//
    void
    sas_rphy_remove(struct sas_rphy *rphy)
    {
    struct device *dev = &rphy.dev;
    switch (rphy.identify.device_type) {
    case SAS_END_DEVICE:
    scsi_remove_target(dev);
    break;
    case SAS_EDGE_EXPANDER_DEVICE:
    case SAS_FANOUT_EXPANDER_DEVICE:
    sas_remove_children(dev);
    break;
    default:
    break;
    }
    sas_rphy_unlink(rphy);
    bsg_remove_queue(rphy.q);
    transport_remove_device(dev);
    device_del(dev);
    }
    EXPORT_SYMBOL(sas_rphy_remove);
//
// scsi_is_sas_rphy  -  check if a struct device represents a SAS remote PHY
// @dev:	device to check
//
// Returns:
// %1 if the device represents a SAS remote PHY, %0 else
//
#[no_mangle]
pub unsafe extern "C" fn scsi_is_sas_rphy(dev: *const device) -> c_int {
    int scsi_is_sas_rphy(const struct device *dev)
    {
    return dev.release == sas_end_device_release ||
    dev.release == sas_expander_release;
    }
    EXPORT_SYMBOL(scsi_is_sas_rphy);
//
// SCSI scan helper
//
    static int sas_user_scan(struct Scsi_Host *shost, uint channel,
    uint id, u64 lun)
    {
    struct sas_host_attrs *sas_host = to_sas_host_attrs(shost);
    struct sas_rphy *rphy;
    mutex_lock(&sas_host.lock);
    list_for_each_entry(rphy, &sas_host.rphy_list, list) {
    if (rphy.identify.device_type != SAS_END_DEVICE ||
    rphy.scsi_target_id == -1)
    continue;
    if ((channel == SCAN_WILD_CARD || channel == 0) &&
    (id == SCAN_WILD_CARD || id == rphy.scsi_target_id)) {
    scsi_scan_target(&rphy.dev, 0, rphy.scsi_target_id,
    lun, SCSI_SCAN_MANUAL);
    }
    }
    mutex_unlock(&sas_host.lock);
    return 0;
    }
//
// Setup / Teardown code
//

    i.private_##attrb[count] = dev_attr_##field;		\
    i.private_##attrb[count].attr.mode = perm;			\
    i.attrb[count] = &i.private_##attrb[count];			\
    if (test)							\
    count++

    i.private_##attrb[count] = dev_attr_##field;		\
    i.private_##attrb[count].attr.mode = perm;			\
    if (ro_test) {							\
    i.private_##attrb[count].attr.mode = ro_perm;		\
    i.private_##attrb[count].store = core::ptr::null_mut();			\
    }								\
    i.attrb[count] = &i.private_##attrb[count];			\
    if (test)							\
    count++

    SETUP_TEMPLATE(rphy_attrs, field, S_IRUGO, 1)

    SETUP_TEMPLATE(rphy_attrs, field, S_IRUGO, i.f.func)

    SETUP_TEMPLATE(phy_attrs, field, S_IRUGO, 1)

    SETUP_TEMPLATE_RW(phy_attrs, field, S_IRUGO | S_IWUSR, 1,	\
    !i.f.set_phy_speed, S_IRUGO)

    SETUP_TEMPLATE_RW(phy_attrs, field, S_IRUGO | S_IWUSR, 1,	\
    !i.f.func, S_IRUGO)

    SETUP_TEMPLATE(port_attrs, field, S_IRUGO, 1)

    SETUP_TEMPLATE(phy_attrs, field, S_IRUGO, i.f.func)

    SETUP_TEMPLATE(phy_attrs, field, S_IWUSR, 1)

    SETUP_TEMPLATE(phy_attrs, field, S_IWUSR, i.f.func)

    SETUP_TEMPLATE(end_dev_attrs, field, S_IRUGO, 1)

    SETUP_TEMPLATE(expander_attrs, expander_##field, S_IRUGO, 1)
//
// sas_attach_transport  -  instantiate SAS transport template
// @ft:		SAS transport class function template
//
    struct scsi_transport_template *
    sas_attach_transport(struct sas_function_template *ft)
    {
    struct sas_internal *i;
    int count;
    i = kzalloc_obj(struct sas_internal);
    if (!i)
    return core::ptr::null_mut();
    i.t.user_scan = sas_user_scan;
    i.t.host_attrs.ac.attrs = &i.host_attrs[0];
    i.t.host_attrs.ac.class = &sas_host_class.class;
    i.t.host_attrs.ac.match = sas_host_match;
    transport_container_register(&i.t.host_attrs);
    i.t.host_size = sizeof(struct sas_host_attrs);
    i.phy_attr_cont.ac.class = &sas_phy_class.class;
    i.phy_attr_cont.ac.attrs = &i.phy_attrs[0];
    i.phy_attr_cont.ac.match = sas_phy_match;
    transport_container_register(&i.phy_attr_cont);
    i.port_attr_cont.ac.class = &sas_port_class.class;
    i.port_attr_cont.ac.attrs = &i.port_attrs[0];
    i.port_attr_cont.ac.match = sas_port_match;
    transport_container_register(&i.port_attr_cont);
    i.rphy_attr_cont.ac.class = &sas_rphy_class.class;
    i.rphy_attr_cont.ac.attrs = &i.rphy_attrs[0];
    i.rphy_attr_cont.ac.match = sas_rphy_match;
    transport_container_register(&i.rphy_attr_cont);
    i.end_dev_attr_cont.ac.class = &sas_end_dev_class.class;
    i.end_dev_attr_cont.ac.attrs = &i.end_dev_attrs[0];
    i.end_dev_attr_cont.ac.match = sas_end_dev_match;
    transport_container_register(&i.end_dev_attr_cont);
    i.expander_attr_cont.ac.class = &sas_expander_class.class;
    i.expander_attr_cont.ac.attrs = &i.expander_attrs[0];
    i.expander_attr_cont.ac.match = sas_expander_match;
    transport_container_register(&i.expander_attr_cont);
    i.f = ft;
    count = 0;
    SETUP_PHY_ATTRIBUTE(initiator_port_protocols);
    SETUP_PHY_ATTRIBUTE(target_port_protocols);
    SETUP_PHY_ATTRIBUTE(device_type);
    SETUP_PHY_ATTRIBUTE(sas_address);
    SETUP_PHY_ATTRIBUTE(phy_identifier);
    SETUP_PHY_ATTRIBUTE(negotiated_linkrate);
    SETUP_PHY_ATTRIBUTE(minimum_linkrate_hw);
    SETUP_PHY_ATTRIBUTE_RW(minimum_linkrate);
    SETUP_PHY_ATTRIBUTE(maximum_linkrate_hw);
    SETUP_PHY_ATTRIBUTE_RW(maximum_linkrate);
    SETUP_PHY_ATTRIBUTE(invalid_dword_count);
    SETUP_PHY_ATTRIBUTE(running_disparity_error_count);
    SETUP_PHY_ATTRIBUTE(loss_of_dword_sync_count);
    SETUP_PHY_ATTRIBUTE(phy_reset_problem_count);
    SETUP_OPTIONAL_PHY_ATTRIBUTE_WRONLY(link_reset, phy_reset);
    SETUP_OPTIONAL_PHY_ATTRIBUTE_WRONLY(hard_reset, phy_reset);
    SETUP_OPTIONAL_PHY_ATTRIBUTE_RW(enable, phy_enable);
    i.phy_attrs[count] = core::ptr::null_mut();
    count = 0;
    SETUP_PORT_ATTRIBUTE(num_phys);
    i.port_attrs[count] = core::ptr::null_mut();
    count = 0;
    SETUP_RPORT_ATTRIBUTE(rphy_initiator_port_protocols);
    SETUP_RPORT_ATTRIBUTE(rphy_target_port_protocols);
    SETUP_RPORT_ATTRIBUTE(rphy_device_type);
    SETUP_RPORT_ATTRIBUTE(rphy_sas_address);
    SETUP_RPORT_ATTRIBUTE(rphy_phy_identifier);
    SETUP_RPORT_ATTRIBUTE(rphy_scsi_target_id);
    SETUP_OPTIONAL_RPORT_ATTRIBUTE(rphy_enclosure_identifier,
    get_enclosure_identifier);
    SETUP_OPTIONAL_RPORT_ATTRIBUTE(rphy_bay_identifier,
    get_bay_identifier);
    i.rphy_attrs[count] = core::ptr::null_mut();
    count = 0;
    SETUP_END_DEV_ATTRIBUTE(end_dev_ready_led_meaning);
    SETUP_END_DEV_ATTRIBUTE(end_dev_I_T_nexus_loss_timeout);
    SETUP_END_DEV_ATTRIBUTE(end_dev_initiator_response_timeout);
    SETUP_END_DEV_ATTRIBUTE(end_dev_tlr_supported);
    SETUP_END_DEV_ATTRIBUTE(end_dev_tlr_enabled);
    i.end_dev_attrs[count] = core::ptr::null_mut();
    count = 0;
    SETUP_EXPANDER_ATTRIBUTE(vendor_id);
    SETUP_EXPANDER_ATTRIBUTE(product_id);
    SETUP_EXPANDER_ATTRIBUTE(product_rev);
    SETUP_EXPANDER_ATTRIBUTE(component_vendor_id);
    SETUP_EXPANDER_ATTRIBUTE(component_id);
    SETUP_EXPANDER_ATTRIBUTE(component_revision_id);
    SETUP_EXPANDER_ATTRIBUTE(level);
    i.expander_attrs[count] = core::ptr::null_mut();
    return &i.t;
    }
    EXPORT_SYMBOL(sas_attach_transport);
//
// sas_release_transport  -  release SAS transport template instance
// @t:		transport template instance
//
#[no_mangle]
pub unsafe extern "C" fn sas_release_transport(t: *mut scsi_transport_template) {
    void sas_release_transport(struct scsi_transport_template *t)
    {
    struct sas_internal *i = to_sas_internal(t);
    transport_container_unregister(&i.t.host_attrs);
    transport_container_unregister(&i.phy_attr_cont);
    transport_container_unregister(&i.port_attr_cont);
    transport_container_unregister(&i.rphy_attr_cont);
    transport_container_unregister(&i.end_dev_attr_cont);
    transport_container_unregister(&i.expander_attr_cont);
    kfree(i);
    }
    EXPORT_SYMBOL(sas_release_transport);
#[no_mangle]
unsafe extern "C" fn sas_transport_init() -> __init int {
    static __init int sas_transport_init(void)
    {
    int error;
    error = transport_class_register(&sas_host_class);
    if (error)
    goto out;
    error = transport_class_register(&sas_phy_class);
    if (error)
    goto out_unregister_transport;
    error = transport_class_register(&sas_port_class);
    if (error)
    goto out_unregister_phy;
    error = transport_class_register(&sas_rphy_class);
    if (error)
    goto out_unregister_port;
    error = transport_class_register(&sas_end_dev_class);
    if (error)
    goto out_unregister_rphy;
    error = transport_class_register(&sas_expander_class);
    if (error)
    goto out_unregister_end_dev;
    return 0;
    out_unregister_end_dev:
    transport_class_unregister(&sas_end_dev_class);
    out_unregister_rphy:
    transport_class_unregister(&sas_rphy_class);
    out_unregister_port:
    transport_class_unregister(&sas_port_class);
    out_unregister_phy:
    transport_class_unregister(&sas_phy_class);
    out_unregister_transport:
    transport_class_unregister(&sas_host_class);
    out:
    return error;
    }
#[no_mangle]
unsafe extern "C" fn sas_transport_exit() -> void __exit {
    static void __exit sas_transport_exit(void)
    {
    transport_class_unregister(&sas_host_class);
    transport_class_unregister(&sas_phy_class);
    transport_class_unregister(&sas_port_class);
    transport_class_unregister(&sas_rphy_class);
    transport_class_unregister(&sas_end_dev_class);
    transport_class_unregister(&sas_expander_class);
    }
    MODULE_AUTHOR("Christoph Hellwig");
    MODULE_DESCRIPTION("SAS Transport Attributes");
    MODULE_LICENSE("GPL");
    module_init(sas_transport_init);
    module_exit(sas_transport_exit);
