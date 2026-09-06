//! Automatically rewritten from C to Rust
//! Source: drivers/platform/x86/intel/vsec_tpmi.c
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
// Driver to enumerate TPMI features and create devices
//
// Copyright (c) 2023, Intel Corporation.
// All Rights Reserved.
//
// The TPMI (Topology Aware Register and PM Capsule Interface) provides a
// flexible, extendable and PCIe enumerable MMIO interface for PM features.
//
// For example Intel RAPL (Running Average Power Limit) provides a MMIO
// interface using TPMI. This has advantage over traditional MSR
// (Model Specific Register) interface, where a thread needs to be scheduled
// on the target CPU to read or write. Also the RAPL features vary between
// CPU models, and hence lot of model specific code. Here TPMI provides an
// architectural interface by providing hierarchical tables and fields,
// which will not need any model specific implementation.
//
// The TPMI interface uses a PCI VSEC structure to expose the location of
// MMIO region.
//
// This VSEC structure is present in the PCI configuration space of the
// Intel Out-of-Band (OOB) device, which  is handled by the Intel VSEC
// driver. The Intel VSEC driver parses VSEC structures present in the PCI
// configuration space of the given device and creates an auxiliary device
// object for each of them. In particular, it creates an auxiliary device
// object representing TPMI that can be bound by an auxiliary driver.
//
// This TPMI driver will bind to the TPMI auxiliary device object created
// by the Intel VSEC driver.
//
// The TPMI specification defines a PFS (PM Feature Structure) table.
// This table is present in the TPMI MMIO region. The starting address
// of PFS is derived from the tBIR (Bar Indicator Register) and "Address"
// field from the VSEC header.
//
// Each TPMI PM feature has one entry in the PFS with a unique TPMI
// ID and its access details. The TPMI driver creates device nodes
// for the supported PM features.
//
// The names of the devices created by the TPMI driver start with the
// "intel_vsec.tpmi-" prefix which is followed by a specific name of the
// given PM feature (for example, "intel_vsec.tpmi-rapl.0").
//
// The device nodes are create by using interface "intel_vsec_add_aux()"
// provided by the Intel VSEC driver.
//

//
// struct intel_tpmi_pfs_entry - TPMI PM Feature Structure (PFS) entry
// @tpmi_id:	TPMI feature identifier (what the feature is and its data format).
// @num_entries: Number of feature interface instances present in the PFS.
// This represents the maximum number of Power domains in the SoC.
// @entry_size:	Interface instance entry size in 32-bit words.
// @cap_offset:	Offset from the PM_Features base address to the base of the PM VSEC
// register bank in KB.
// @attribute:	Feature attribute: 0=BIOS. 1=OS. 2-3=Reserved.
// @reserved:	Bits for use in the future.
//
// Represents one TPMI feature entry data in the PFS retrieved as is
// from the hardware.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_tpmi_pfs_entry {
    pub tpmi_id:8: u64,
    pub num_entries:8: u64,
    pub entry_size:16: u64,
    pub cap_offset:16: u64,
    pub attribute:2: u64,
    pub reserved:14: u64,
    pub __packed: },
//
// struct intel_tpmi_pm_feature - TPMI PM Feature information for a TPMI ID
// @pfs_header:	PFS header retireved from the hardware.
// @vsec_offset: Starting MMIO address for this feature in bytes. Essentially
// this offset = "Address" from VSEC header + PFS Capability
// offset for this feature entry.
// @vsec_dev:	Pointer to intel_vsec_device structure for this TPMI device
//
// Represents TPMI instance information for one TPMI ID.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_tpmi_pm_feature {
    pub pfs_header: intel_tpmi_pfs_entry,
    pub vsec_offset: u64,
    pub vsec_dev: *mut intel_vsec_device,
}

//
// struct intel_tpmi_info - TPMI information for all IDs in an instance
// @tpmi_features:	Pointer to a list of TPMI feature instances
// @vsec_dev:		Pointer to intel_vsec_device structure for this TPMI device
// @feature_count:	Number of TPMI of TPMI instances pointed by tpmi_features
// @pfs_start:		Start of PFS offset for the TPMI instances in this device
// @plat_info:		Stores platform info which can be used by the client drivers
// @tpmi_control_mem:	Memory mapped IO for getting control information
// @dbgfs_dir:		debugfs entry pointer
//
// Stores the information for all TPMI devices enumerated from a single PCI device.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_tpmi_info {
    pub tpmi_features: *mut intel_tpmi_pm_feature,
    pub vsec_dev: *mut intel_vsec_device,
    pub feature_count: c_int,
    pub pfs_start: u64,
    pub plat_info: oobmsm_plat_info,
    pub tpmi_control_mem: *mut void __iomem,
    pub dbgfs_dir: *mut dentry,
}

//
// struct tpmi_info_header - CPU package ID to PCI device mapping information
// @fn:		PCI function number
// @dev:	PCI device number
// @bus:	PCI bus number
// @pkg:	CPU Package id
// @segment:	PCI segment id
// @partition:	Package Partition id
// @cdie_mask:	Bitmap of compute dies in the current partition
// @reserved:	Reserved for future use
// @lock:	When set to 1 the register is locked and becomes read-only
// until next reset. Not for use by the OS driver.
//
// The structure to read hardware provided mapping information.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tpmi_info_header {
    pub fn:3: u64,
    pub dev:5: u64,
    pub bus:8: u64,
    pub pkg:8: u64,
    pub segment:8: u64,
    pub partition:2: u64,
    pub cdie_mask:16: u64,
    pub reserved:13: u64,
    pub lock:1: u64,
    pub __packed: },
//
// struct tpmi_feature_state - Structure to read hardware state of a feature
// @enabled:	Enable state of a feature, 1: enabled, 0: disabled
// @reserved_1:	Reserved for future use
// @write_blocked: Writes are blocked means all write operations are ignored
// @read_blocked: Reads are blocked means will read 0xFFs
// @pcs_select:	Interface used by out of band software, not used in OS
// @reserved_2:	Reserved for future use
// @id:		TPMI ID of the feature
// @reserved_3:	Reserved for future use
// @locked:	When set to 1, OS can't change this register.
//
// The structure is used to read hardware state of a TPMI feature. This
// information is used for debug and restricting operations for this feature.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tpmi_feature_state {
    pub enabled:1: u32,
    pub reserved_1:3: u32,
    pub write_blocked:1: u32,
    pub read_blocked:1: u32,
    pub pcs_select:1: u32,
    pub reserved_2:1: u32,
    pub id:8: u32,
    pub reserved_3:15: u32,
    pub locked:1: u32,
    pub __packed: },
//
// The size from hardware is in u32 units. This size is from a trusted hardware,
// but better to verify for pre silicon platforms. Set size to 0, when invalid.
//

    ({											\
    pub \: pfs->pfs_header.entry_size > SZ_1K ? 0 : pfs->pfs_header.entry_size << 2;,
    })
// Used during auxbus device creation
    pub DEFINE_IDA(intel_vsec_tpmi_ida): static,
    pub BLOCKING_NOTIFIER_HEAD(tpmi_notify_list): static,
#[no_mangle]
pub unsafe extern "C" fn tpmi_register_notifier(nb: *mut notifier_block) -> c_int {
    int tpmi_register_notifier(struct notifier_block *nb)
    {
    pub nb): return blocking_notifier_chain_register(&tpmi_notify_list,,
    }
    pub "INTEL_TPMI"): EXPORT_SYMBOL_NS_GPL(tpmi_register_notifier,,
#[no_mangle]
pub unsafe extern "C" fn tpmi_unregister_notifier(nb: *mut notifier_block) -> c_int {
    int tpmi_unregister_notifier(struct notifier_block *nb)
    {
    pub nb): return blocking_notifier_chain_unregister(&tpmi_notify_list,,
    }
    pub "INTEL_TPMI"): EXPORT_SYMBOL_NS_GPL(tpmi_unregister_notifier,,
    struct oobmsm_plat_info *tpmi_get_platform_data(struct auxiliary_device *auxdev)
    {
    pub auxdev_to_ivdev(auxdev): *mut *mut intel_vsec_device vsec_dev =,
    pub vsec_dev->priv_data: return,
    }
    pub "INTEL_TPMI"): EXPORT_SYMBOL_NS_GPL(tpmi_get_platform_data,,
#[no_mangle]
pub unsafe extern "C" fn tpmi_get_resource_count(auxdev: *mut auxiliary_device) -> c_int {
    int tpmi_get_resource_count(struct auxiliary_device *auxdev)
    {
    pub auxdev_to_ivdev(auxdev): *mut *mut intel_vsec_device vsec_dev =,
    if (vsec_dev)
    pub vsec_dev->num_resources: return,
    pub 0: return,
    }
    pub "INTEL_TPMI"): EXPORT_SYMBOL_NS_GPL(tpmi_get_resource_count,,
    struct resource *tpmi_get_resource_at_index(struct auxiliary_device *auxdev, int index)
    {
    pub auxdev_to_ivdev(auxdev): *mut *mut intel_vsec_device vsec_dev =,
    if (vsec_dev && index < vsec_dev.num_resources)
    pub &vsec_dev->resource[index]: return,
    pub NULL: return,
    }
    pub "INTEL_TPMI"): EXPORT_SYMBOL_NS_GPL(tpmi_get_resource_at_index,,
// TPMI Control Interface
pub const TPMI_CONTROL_STATUS_OFFSET: c_uint = 0x00;
pub const TPMI_COMMAND_OFFSET: c_uint = 0x08;
pub const TMPI_CONTROL_DATA_VAL_OFFSET: c_uint = 0x0c;
//
// Spec is calling for max 1 seconds to get ownership at the worst
// case. Read at 10 ms timeouts and repeat up to 1 second.
//

// TPMI Control status register defines

pub const TPMI_OWNER_NONE: c_int = 0;
pub const TPMI_OWNER_IN_BAND: c_int = 1;

pub const TPMI_CMD_PKT_LEN: c_int = 2;
pub const TPMI_CMD_STATUS_SUCCESS: c_uint = 0x40;
// TPMI command data registers

// Command to send via control interface
pub const TPMI_CONTROL_GET_STATE_CMD: c_uint = 0x10;

// Mutex to complete get feature status without interruption
    pub DEFINE_MUTEX(tpmi_dev_lock): static,
#[no_mangle]
unsafe extern "C" fn tpmi_wait_for_owner(tpmi_info: *mut intel_tpmi_info, owner: u8) -> c_int {
    static int tpmi_wait_for_owner(struct intel_tpmi_info *tpmi_info, u8 owner)
    {
    pub control: u64,
    return readq_poll_timeout(tpmi_info.tpmi_control_mem + TPMI_CONTROL_STATUS_OFFSET,
    control, owner == FIELD_GET(TPMI_CONTROL_STATUS_OWNER, control),
    pub TPMI_CONTROL_TIMEOUT_MAX_US): TPMI_CONTROL_TIMEOUT_US,,
    }
    static int tpmi_read_feature_status(struct intel_tpmi_info *tpmi_info, int feature_id,
    struct tpmi_feature_state *feature_state)
    {
    pub data: u64 control,,
    pub ret: c_int,
    if (!tpmi_info.tpmi_control_mem)
    pub -EFAULT: return,
// Wait for owner bit set to 0 (none)
    pub TPMI_OWNER_NONE): ret = tpmi_wait_for_owner(tpmi_info,,
    if (ret)
    pub err_unlock: goto,
// set command id to 0x10 for TPMI_GET_STATE
    pub TPMI_CONTROL_GET_STATE_CMD): data = FIELD_PREP(TMPI_CONTROL_DATA_CMD,,
// 32 bits for DATA offset and +8 for feature_id field
    pub feature_id): data |= FIELD_PREP(TPMI_CONTROL_DATA_VAL_FEATURE,,
// Write at command offset for qword access
    pub TPMI_COMMAND_OFFSET): writeq(data, tpmi_info->tpmi_control_mem +,
// Wait for owner bit set to in-band
    pub TPMI_OWNER_IN_BAND): ret = tpmi_wait_for_owner(tpmi_info,,
    if (ret)
    pub err_unlock: goto,
// Set Run Busy and packet length of 2 dwords
    pub TPMI_CONTROL_STATUS_RB: control =,
    pub TPMI_CMD_PKT_LEN): control |= FIELD_PREP(TPMI_CONTROL_STATUS_LEN,,
// Write at status offset for qword access
    pub TPMI_CONTROL_STATUS_OFFSET): writeq(control, tpmi_info->tpmi_control_mem +,
// Wait for Run Busy clear
    ret = readq_poll_timeout(tpmi_info.tpmi_control_mem + TPMI_CONTROL_STATUS_OFFSET,
    control, !(control & TPMI_CONTROL_STATUS_RB),
    pub TPMI_RB_TIMEOUT_MAX_US): TPMI_RB_TIMEOUT_US,,
    if (ret)
    pub done_proc: goto,
    pub control): control = FIELD_GET(TPMI_CONTROL_STATUS_RESULT,,
    if (control != TPMI_CMD_STATUS_SUCCESS) {
    pub -EBUSY: ret =,
    pub done_proc: goto,
    }
// Response is ready
    memcpy_fromio(feature_state, tpmi_info.tpmi_control_mem + TMPI_CONTROL_DATA_VAL_OFFSET,
    pub 0: ret =,
    done_proc:
// Set CPL "completion" bit
    pub TPMI_CONTROL_STATUS_OFFSET): writeq(TPMI_CONTROL_STATUS_CPL, tpmi_info->tpmi_control_mem +,
    err_unlock:
    pub ret: return,
    }
    int tpmi_get_feature_status(struct auxiliary_device *auxdev,
    int feature_id, bool *read_blocked, bool *write_blocked)
    {
    pub dev_to_ivdev(auxdev->dev.parent): *mut *mut intel_vsec_device intel_vsec_dev =,
    pub auxiliary_get_drvdata(&intel_vsec_dev->auxdev): *mut *mut intel_tpmi_info tpmi_info =,
    pub feature_state: tpmi_feature_state,
    pub ret: c_int,
    pub &feature_state): ret = tpmi_read_feature_status(tpmi_info, feature_id,,
    if (ret)
    pub ret: return,
// read_blocked = feature_state.read_blocked;
// write_blocked = feature_state.write_blocked;
    pub 0: return,
    }
    pub "INTEL_TPMI"): EXPORT_SYMBOL_NS_GPL(tpmi_get_feature_status,,
    struct dentry *tpmi_get_debugfs_dir(struct auxiliary_device *auxdev)
    {
    pub dev_to_ivdev(auxdev->dev.parent): *mut *mut intel_vsec_device intel_vsec_dev =,
    pub auxiliary_get_drvdata(&intel_vsec_dev->auxdev): *mut *mut intel_tpmi_info tpmi_info =,
    pub tpmi_info->dbgfs_dir: return,
    }
    pub "INTEL_TPMI"): EXPORT_SYMBOL_NS_GPL(tpmi_get_debugfs_dir,,
#[no_mangle]
unsafe extern "C" fn tpmi_pfs_dbg_show(s: *mut seq_file, unused: *mut c_void) -> c_int {
    static int tpmi_pfs_dbg_show(struct seq_file *s, void *unused)
    {
    pub s->private: *mut *mut intel_tpmi_info tpmi_info =,
    pub write_blocked: int locked, disabled, read_blocked,,
    pub feature_state: tpmi_feature_state,
    pub pfs: *mut intel_tpmi_pm_feature,
    pub i: int ret,,
    pub tpmi_info->pfs_start): seq_printf(s, "tpmi PFS start offset 0x:%llx\n",,
    pub "tpmi_id\t\tentries\t\tsize\t\tcap_offset\tattribute\tvsec_offset\tlocked\tdisabled\tread_blocked\twrite_blocked\n"): seq_puts(s,,
    pub {: for (i = 0; i < tpmi_info->feature_count; ++i),
    pub &tpmi_info->tpmi_features[i]: pfs =,
    pub &feature_state): ret = tpmi_read_feature_status(tpmi_info, pfs->pfs_header.tpmi_id,,
    if (ret) {
    pub 'U': locked =,
    pub 'U': disabled =,
    pub 'U': read_blocked =,
    pub 'U': write_blocked =,
    } else {
    pub 'Y': disabled = feature_state.enabled ? 'N' :,
    pub 'N': locked = feature_state.locked ? 'Y' :,
    pub 'N': read_blocked = feature_state.read_blocked ? 'Y' :,
    pub 'N': write_blocked = feature_state.write_blocked ? 'Y' :,
    }
    seq_printf(s, "0x%02x\t\t0x%02x\t\t0x%04x\t\t0x%04x\t\t0x%02x\t\t0x%016llx\t%c\t%c\t\t%c\t\t%c\n",
    pfs.pfs_header.tpmi_id, pfs.pfs_header.num_entries,
    pfs.pfs_header.entry_size, pfs.pfs_header.cap_offset,
    pfs.pfs_header.attribute, pfs.vsec_offset, locked, disabled,
    pub write_blocked): read_blocked,,
    }
    pub 0: return,
    }
pub const MEM_DUMP_COLUMN_COUNT: c_int = 8;
#[no_mangle]
unsafe extern "C" fn tpmi_mem_dump_show(s: *mut seq_file, unused: *mut c_void) -> c_int {
    static int tpmi_mem_dump_show(struct seq_file *s, void *unused)
    {
    pub sizeof(u32): *mut *mut size_t row_size = MEM_DUMP_COLUMN_COUNT,
    pub s->private: *mut *mut intel_tpmi_pm_feature pfs =,
    pub 0: int count, ret =,
    pub mem: *mut void __iomem,
    pub size: u32,
    pub off: u64,
    pub buffer: *mut u8,
    pub TPMI_GET_SINGLE_ENTRY_SIZE(pfs): size =,
    if (!size)
    pub -EIO: return,
    pub GFP_KERNEL): buffer = kmalloc(size,,
    if (!buffer)
    pub -ENOMEM: return,
    pub pfs->vsec_offset: off =,
    pub {: for (count = 0; count < pfs->pfs_header.num_entries; ++count),
    pub off): seq_printf(s, "TPMI Instance:%d offset:0x%llx\n", count,,
    pub size): mem = ioremap(off,,
    if (!mem) {
    pub -ENOMEM: ret =,
    }
    pub size): memcpy_fromio(buffer, mem,,
    seq_hex_dump(s, " ", DUMP_PREFIX_OFFSET, row_size, sizeof(u32), buffer, size,
    pub size: off +=,
    }
    pub ret: return,
    }
#[no_mangle]
unsafe extern "C" fn mem_write(file: *mut file, userbuf: *const char __user, len: usize, ppos: *mut loff_t) -> isize {
    static ssize_t mem_write(struct file *file, const char __user *userbuf, size_t len, loff_t *ppos)
    {
    pub file->private_data: *mut *mut seq_file m =,
    pub m->private: *mut *mut intel_tpmi_pm_feature pfs =,
    pub size: u32 addr, value, punit,,
    pub num_elems: u32,
    pub mem: *mut void __iomem,
    pub ret: c_int,
    pub TPMI_GET_SINGLE_ENTRY_SIZE(pfs): size =,
    if (!size)
    pub -EIO: return,
    pub NULL: *mut *mut u32 array __free(kfree) =,
    pub )&array): *mut ret = parse_int_array_user(userbuf, len, (int,
    if (ret < 0)
    pub ret: return,
    pub array: *mut num_elems =,
    if (num_elems != 3)
    pub -EINVAL: return,
    pub array: [punit =; 1],
    pub array: [addr =; 2],
    pub array: [value =; 3],
    if (!IS_ALIGNED(addr, sizeof(u32)))
    pub -EINVAL: return,
    if (punit >= pfs.pfs_header.num_entries)
    pub -EINVAL: return,
    if (addr >= size)
    pub -EINVAL: return,
    pub size): *mut *mut mem = ioremap(pfs->vsec_offset + punit  size,,
    if (!mem)
    pub -ENOMEM: return,
    pub addr): writel(value, mem +,
    pub len: return,
    }
#[no_mangle]
unsafe extern "C" fn mem_write_show(s: *mut seq_file, unused: *mut c_void) -> c_int {
    static int mem_write_show(struct seq_file *s, void *unused)
    {
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn mem_write_open(inode: *mut inode, file: *mut file) -> c_int {
    static int mem_write_open(struct inode *inode, struct file *file)
    {
    pub inode->i_private): return single_open(file, mem_write_show,,
    }
    static const struct file_operations mem_write_ops = {
    .open           = mem_write_open,
    .read           = seq_read,
    .write          = mem_write,
    .llseek         = seq_lseek,
    .release        = single_release,
}

#[no_mangle]
unsafe extern "C" fn tpmi_dbgfs_register(tpmi_info: *mut intel_tpmi_info) {
    static void tpmi_dbgfs_register(struct intel_tpmi_info *tpmi_info)
    {
    char name[64];
    int i;
    snprintf(name, sizeof(name), "tpmi-%s", dev_name(tpmi_to_dev(tpmi_info)));
    tpmi_info.dbgfs_dir = debugfs_create_dir(name, core::ptr::null_mut());
    debugfs_create_file("pfs_dump", 0444, tpmi_info.dbgfs_dir, tpmi_info, &tpmi_pfs_dbg_fops);
    for (i = 0; i < tpmi_info.feature_count; ++i) {
    struct intel_tpmi_pm_feature *pfs;
    struct dentry *dir;
    pfs = &tpmi_info.tpmi_features[i];
    snprintf(name, sizeof(name), "tpmi-id-%02x", pfs.pfs_header.tpmi_id);
    dir = debugfs_create_dir(name, tpmi_info.dbgfs_dir);
    debugfs_create_file("mem_dump", 0444, dir, pfs, &tpmi_mem_dump_fops);
    debugfs_create_file("mem_write", 0644, dir, pfs, &mem_write_ops);
    }
    }
    static void tpmi_set_control_base(struct auxiliary_device *auxdev,
    struct intel_tpmi_info *tpmi_info,
    struct intel_tpmi_pm_feature *pfs)
    {
    void __iomem *mem;
    u32 size;
    size = TPMI_GET_SINGLE_ENTRY_SIZE(pfs);
    if (!size)
    return;
    mem = devm_ioremap(&auxdev.dev, pfs.vsec_offset, size);
    if (!mem)
    return;
// mem is pointing to TPMI CONTROL base
    tpmi_info.tpmi_control_mem = mem;
    }
    static const char *intel_tpmi_name(enum intel_tpmi_id id)
    {
    switch (id) {
    case TPMI_ID_RAPL:
    return "rapl";
    case TPMI_ID_PEM:
    return "pem";
    case TPMI_ID_UNCORE:
    return "uncore";
    case TPMI_ID_SST:
    return "sst";
    case TPMI_ID_PLR:
    return "plr";
    default:
    return core::ptr::null_mut();
    }
    }
// String Length for tpmi-"feature_name(upto 8 bytes)"
pub const TPMI_FEATURE_NAME_LEN: c_int = 14;
    static int tpmi_create_device(struct intel_tpmi_info *tpmi_info,
    struct intel_tpmi_pm_feature *pfs,
    u64 pfs_start)
    {
    struct intel_vsec_device *vsec_dev = tpmi_info.vsec_dev;
    char feature_id_name[TPMI_FEATURE_NAME_LEN];
    struct intel_vsec_device *feature_vsec_dev;
    struct tpmi_feature_state feature_state;
    struct resource *res, *tmp;
    const char *name;
    int i, ret;
    ret = tpmi_read_feature_status(tpmi_info, pfs.pfs_header.tpmi_id, &feature_state);
    if (ret)
    return ret;
//
// If not enabled, continue to look at other features in the PFS, so return -EOPNOTSUPP.
// This will not cause failure of loading of this driver.
//
    if (!feature_state.enabled)
    return -EOPNOTSUPP;
    name = intel_tpmi_name(pfs.pfs_header.tpmi_id);
    if (!name)
    return -EOPNOTSUPP;
    feature_vsec_dev = kzalloc_flex(*feature_vsec_dev, resource, pfs.pfs_header.num_entries);
    if (!feature_vsec_dev)
    return -ENOMEM;
    feature_vsec_dev.num_resources = pfs.pfs_header.num_entries;
    res = feature_vsec_dev.resource;
    snprintf(feature_id_name, sizeof(feature_id_name), "tpmi-%s", name);
    for (i = 0, tmp = res; i < pfs.pfs_header.num_entries; i++, tmp++) {
    let mut entry_size_bytes: u64 = pfs.pfs_header.entry_size * sizeof(u32);
    tmp.start = pfs.vsec_offset + entry_size_bytes * i;
    tmp.end = tmp.start + entry_size_bytes - 1;
    tmp.flags = IORESOURCE_MEM;
    }
    feature_vsec_dev.dev = vsec_dev.dev;
    feature_vsec_dev.priv_data = &tpmi_info.plat_info;
    feature_vsec_dev.priv_data_size = sizeof(tpmi_info.plat_info);
    feature_vsec_dev.ida = &intel_vsec_tpmi_ida;
//
// intel_vsec_add_aux() is resource managed, no explicit
// delete is required on error or on module unload.
// feature_vsec_dev and res memory are also freed as part of
// device deletion.
//
    return intel_vsec_add_aux(&vsec_dev.auxdev.dev,
    feature_vsec_dev, feature_id_name);
    }
#[no_mangle]
unsafe extern "C" fn tpmi_create_devices(tpmi_info: *mut intel_tpmi_info) -> c_int {
    static int tpmi_create_devices(struct intel_tpmi_info *tpmi_info)
    {
    struct intel_vsec_device *vsec_dev = tpmi_info.vsec_dev;
    int ret, i;
    for (i = 0; i < vsec_dev.num_resources; i++) {
    ret = tpmi_create_device(tpmi_info, &tpmi_info.tpmi_features[i],
    tpmi_info.pfs_start);
//
// Fail, if the supported features fails to create device,
// otherwise, continue. Even if one device failed to create,
// fail the loading of driver. Since intel_vsec_add_aux()
// is resource managed, no clean up is required for the
// successfully created devices.
//
    if (ret && ret != -EOPNOTSUPP)
    return ret;
    }
    return 0;
    }
pub const TPMI_INFO_BUS_INFO_OFFSET: c_uint = 0x08;
pub const TPMI_INFO_MAJOR_VERSION: c_uint = 0x00;
pub const TPMI_INFO_MINOR_VERSION: c_uint = 0x02;
    static int tpmi_process_info(struct intel_tpmi_info *tpmi_info,
    struct intel_tpmi_pm_feature *pfs)
    {
    struct tpmi_info_header header;
    void __iomem *info_mem;
    u64 feature_header;
    let mut ret: c_int = 0;
    info_mem = ioremap(pfs.vsec_offset, pfs.pfs_header.entry_size * sizeof(u32));
    if (!info_mem)
    return -ENOMEM;
    feature_header = readq(info_mem);
    if (TPMI_MAJOR_VERSION(feature_header) != TPMI_INFO_MAJOR_VERSION) {
    ret = -ENODEV;
    goto error_info_header;
    }
    memcpy_fromio(&header, info_mem + TPMI_INFO_BUS_INFO_OFFSET, sizeof(header));
    tpmi_info.plat_info.package_id = header.pkg;
    tpmi_info.plat_info.bus_number = header.bus;
    tpmi_info.plat_info.device_number = header.dev;
    tpmi_info.plat_info.function_number = header.fn;
    if (TPMI_MINOR_VERSION(feature_header) >= TPMI_INFO_MINOR_VERSION) {
    tpmi_info.plat_info.cdie_mask = header.cdie_mask;
    tpmi_info.plat_info.partition = header.partition;
    tpmi_info.plat_info.segment = header.segment;
    }
    error_info_header:
    iounmap(info_mem);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn tpmi_fetch_pfs_header(pfs: *mut intel_tpmi_pm_feature, start: u64, size: c_int) -> c_int {
    static int tpmi_fetch_pfs_header(struct intel_tpmi_pm_feature *pfs, u64 start, int size)
    {
    void __iomem *pfs_mem;
    pfs_mem = ioremap(start, size);
    if (!pfs_mem)
    return -ENOMEM;
    memcpy_fromio(&pfs.pfs_header, pfs_mem, sizeof(pfs.pfs_header));
    iounmap(pfs_mem);
    return 0;
    }
pub const TPMI_CAP_OFFSET_UNIT: c_int = 1024;
#[no_mangle]
unsafe extern "C" fn intel_vsec_tpmi_init(auxdev: *mut auxiliary_device) -> c_int {
    static int intel_vsec_tpmi_init(struct auxiliary_device *auxdev)
    {
    struct intel_vsec_device *vsec_dev = auxdev_to_ivdev(auxdev);
    struct pci_dev *pci_dev = to_pci_dev(vsec_dev.dev);
    struct intel_tpmi_info *tpmi_info;
    let mut pfs_start: u64 = 0;
    int ret, i;
    tpmi_info = devm_kzalloc(&auxdev.dev, sizeof(*tpmi_info), GFP_KERNEL);
    if (!tpmi_info)
    return -ENOMEM;
    tpmi_info.vsec_dev = vsec_dev;
    tpmi_info.feature_count = vsec_dev.num_resources;
    tpmi_info.plat_info.bus_number = pci_dev.bus.number;
    tpmi_info.tpmi_features = devm_kcalloc(&auxdev.dev, vsec_dev.num_resources,
    sizeof(*tpmi_info.tpmi_features),
    GFP_KERNEL);
    if (!tpmi_info.tpmi_features)
    return -ENOMEM;
    for (i = 0; i < vsec_dev.num_resources; i++) {
    struct intel_tpmi_pm_feature *pfs;
    struct resource *res;
    u64 res_start;
    int size, ret;
    pfs = &tpmi_info.tpmi_features[i];
    pfs.vsec_dev = vsec_dev;
    res = &vsec_dev.resource[i];
    if (!res)
    continue;
    res_start = res.start;
    size = resource_size(res);
    if (size < 0)
    continue;
    ret = tpmi_fetch_pfs_header(pfs, res_start, size);
    if (ret)
    continue;
    if (!pfs_start)
    pfs_start = res_start;
    pfs.vsec_offset = pfs_start + pfs.pfs_header.cap_offset * TPMI_CAP_OFFSET_UNIT;
//
// Process TPMI_INFO to get PCI device to CPU package ID.
// Device nodes for TPMI features are not created in this
// for loop. So, the mapping information will be available
// when actual device nodes created outside this
// loop via tpmi_create_devices().
//
    if (pfs.pfs_header.tpmi_id == TPMI_INFO_ID) {
    ret = tpmi_process_info(tpmi_info, pfs);
    if (ret)
    return ret;
    ret = intel_vsec_set_mapping(&tpmi_info.plat_info, vsec_dev);
    if (ret)
    return ret;
    }
    if (pfs.pfs_header.tpmi_id == TPMI_CONTROL_ID)
    tpmi_set_control_base(auxdev, tpmi_info, pfs);
    }
    tpmi_info.pfs_start = pfs_start;
    auxiliary_set_drvdata(auxdev, tpmi_info);
//
// Allow debugfs when security policy allows. Everything this debugfs
// interface provides, can also be done via /dev/mem access. If
// /dev/mem interface is locked, don't allow debugfs to present any
// information. Also check for CAP_SYS_RAWIO as /dev/mem interface.
//
    if (!security_locked_down(LOCKDOWN_DEV_MEM) && capable(CAP_SYS_RAWIO))
    tpmi_dbgfs_register(tpmi_info);
    ret = tpmi_create_devices(tpmi_info);
    if (ret) {
    debugfs_remove_recursive(tpmi_info.dbgfs_dir);
    return ret;
    }
    blocking_notifier_call_chain(&tpmi_notify_list, TPMI_CORE_INIT, auxdev);
    return 0;
    }
    static int tpmi_probe(struct auxiliary_device *auxdev,
    const struct auxiliary_device_id *id)
    {
    return intel_vsec_tpmi_init(auxdev);
    }
#[no_mangle]
unsafe extern "C" fn tpmi_remove(auxdev: *mut auxiliary_device) {
    static void tpmi_remove(struct auxiliary_device *auxdev)
    {
    struct intel_tpmi_info *tpmi_info = auxiliary_get_drvdata(auxdev);
    blocking_notifier_call_chain(&tpmi_notify_list, TPMI_CORE_EXIT, auxdev);
    debugfs_remove_recursive(tpmi_info.dbgfs_dir);
    }
    static const struct auxiliary_device_id tpmi_id_table[] = {
    { .name = "intel_vsec.tpmi" },
    {}
    };
    MODULE_DEVICE_TABLE(auxiliary, tpmi_id_table);
    static struct auxiliary_driver tpmi_aux_driver = {
    .id_table	= tpmi_id_table,
    .probe		= tpmi_probe,
    .remove         = tpmi_remove,
    };
    module_auxiliary_driver(tpmi_aux_driver);
    MODULE_IMPORT_NS("INTEL_VSEC");
    MODULE_DESCRIPTION("Intel TPMI enumeration module");
    MODULE_LICENSE("GPL");
