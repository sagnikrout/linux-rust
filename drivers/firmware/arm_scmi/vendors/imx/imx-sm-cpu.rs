//! Automatically rewritten from C to Rust
//! Source: drivers/firmware/arm_scmi/vendors/imx/imx-sm-cpu.c
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
// System control and Management Interface (SCMI) NXP CPU Protocol
//
// Copyright 2025 NXP
//

pub const SCMI_PROTOCOL_SUPPORTED_VERSION: c_uint = 0x10000;
    enum scmi_imx_cpu_protocol_cmd {
    SCMI_IMX_CPU_ATTRIBUTES	= 0x3,
    SCMI_IMX_CPU_START = 0x4,
    SCMI_IMX_CPU_STOP = 0x5,
    SCMI_IMX_CPU_RESET_VECTOR_SET = 0x6,
    SCMI_IMX_CPU_INFO_GET = 0xC,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_imx_cpu_info {
    pub nr_cpu: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_msg_imx_cpu_protocol_attributes {
    pub attributes: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_msg_imx_cpu_attributes_out {
    pub attributes: __le32,
pub const CPU_MAX_NAME: c_int = 16;
    pub name: [u8; CPU_MAX_NAME],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_imx_cpu_reset_vector_set_in {
    pub cpuid: __le32,

    pub flags: __le32,
    pub resetvectorlow: __le32,
    pub resetvectorhigh: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_imx_cpu_info_get_out {
pub const CPU_RUN_MODE_START: c_int = 0;
pub const CPU_RUN_MODE_HOLD: c_int = 1;
pub const CPU_RUN_MODE_STOP: c_int = 2;
pub const CPU_RUN_MODE_SLEEP: c_int = 3;
    pub runmode: __le32,
    pub sleepmode: __le32,
    pub resetvectorlow: __le32,
    pub resetvectorhigh: __le32,
}

    static int scmi_imx_cpu_validate_cpuid(const struct scmi_protocol_handle *ph,
    u32 cpuid)
    {
    struct scmi_imx_cpu_info *info = ph.get_priv(ph);
    if (cpuid >= info.nr_cpu)
    return -EINVAL;
    return 0;
    }
    static int scmi_imx_cpu_start(const struct scmi_protocol_handle *ph,
    u32 cpuid, bool start)
    {
    struct scmi_xfer *t;
    u8 msg_id;
    int ret;
    ret = scmi_imx_cpu_validate_cpuid(ph, cpuid);
    if (ret)
    return ret;
    if (start)
    msg_id = SCMI_IMX_CPU_START;
    else
    msg_id = SCMI_IMX_CPU_STOP;
    ret = ph.xops.xfer_get_init(ph, msg_id, sizeof(u32), 0, &t);
    if (ret)
    return ret;
    put_unaligned_le32(cpuid, t.tx.buf);
    ret = ph.xops.do_xfer(ph, t);
    ph.xops.xfer_put(ph, t);
    return ret;
    }
    static int scmi_imx_cpu_reset_vector_set(const struct scmi_protocol_handle *ph,
    u32 cpuid, u64 vector, bool start,
    bool boot, bool resume)
    {
    struct scmi_imx_cpu_reset_vector_set_in *in;
    struct scmi_xfer *t;
    int ret;
    ret = scmi_imx_cpu_validate_cpuid(ph, cpuid);
    if (ret)
    return ret;
    ret = ph.xops.xfer_get_init(ph, SCMI_IMX_CPU_RESET_VECTOR_SET, sizeof(*in),
    0, &t);
    if (ret)
    return ret;
    in = t.tx.buf;
    in.cpuid = cpu_to_le32(cpuid);
    in.flags = cpu_to_le32(0);
    if (start)
    in.flags |= le32_encode_bits(1, CPU_VEC_FLAGS_START);
    if (boot)
    in.flags |= le32_encode_bits(1, CPU_VEC_FLAGS_BOOT);
    if (resume)
    in.flags |= le32_encode_bits(1, CPU_VEC_FLAGS_RESUME);
    in.resetvectorlow = cpu_to_le32(lower_32_bits(vector));
    in.resetvectorhigh = cpu_to_le32(upper_32_bits(vector));
    ret = ph.xops.do_xfer(ph, t);
    ph.xops.xfer_put(ph, t);
    return ret;
    }
    static int scmi_imx_cpu_started(const struct scmi_protocol_handle *ph, u32 cpuid,
    bool *started)
    {
    struct scmi_imx_cpu_info_get_out *out;
    struct scmi_xfer *t;
    u32 mode;
    int ret;
    if (!started)
    return -EINVAL;
// started = false;
    ret = scmi_imx_cpu_validate_cpuid(ph, cpuid);
    if (ret)
    return ret;
    ret = ph.xops.xfer_get_init(ph, SCMI_IMX_CPU_INFO_GET, sizeof(u32),
    0, &t);
    if (ret)
    return ret;
    put_unaligned_le32(cpuid, t.tx.buf);
    ret = ph.xops.do_xfer(ph, t);
    if (!ret) {
    out = t.rx.buf;
    mode = le32_to_cpu(out.runmode);
    if (mode == CPU_RUN_MODE_START || mode == CPU_RUN_MODE_SLEEP)
// started = true;
    }
    ph.xops.xfer_put(ph, t);
    return ret;
    }
    static const struct scmi_imx_cpu_proto_ops scmi_imx_cpu_proto_ops = {
    .cpu_reset_vector_set = scmi_imx_cpu_reset_vector_set,
    .cpu_start = scmi_imx_cpu_start,
    .cpu_started = scmi_imx_cpu_started,
    };
    static int scmi_imx_cpu_protocol_attributes_get(const struct scmi_protocol_handle *ph,
    struct scmi_imx_cpu_info *info)
    {
    struct scmi_msg_imx_cpu_protocol_attributes *attr;
    struct scmi_xfer *t;
    int ret;
    ret = ph.xops.xfer_get_init(ph, PROTOCOL_ATTRIBUTES, 0,
    sizeof(*attr), &t);
    if (ret)
    return ret;
    attr = t.rx.buf;
    ret = ph.xops.do_xfer(ph, t);
    if (!ret) {
    info.nr_cpu = le32_get_bits(attr.attributes, SCMI_IMX_CPU_NR_CPU_MASK);
    dev_info(ph.dev, "i.MX SM CPU: %d cpus\n",
    info.nr_cpu);
    }
    ph.xops.xfer_put(ph, t);
    return ret;
    }
    static int scmi_imx_cpu_attributes_get(const struct scmi_protocol_handle *ph,
    u32 cpuid)
    {
    struct scmi_msg_imx_cpu_attributes_out *out;
    char name[SCMI_SHORT_NAME_MAX_SIZE] = {'\0'};
    struct scmi_xfer *t;
    int ret;
    ret = ph.xops.xfer_get_init(ph, SCMI_IMX_CPU_ATTRIBUTES, sizeof(u32), 0, &t);
    if (ret)
    return ret;
    put_unaligned_le32(cpuid, t.tx.buf);
    ret = ph.xops.do_xfer(ph, t);
    if (!ret) {
    out = t.rx.buf;
    strscpy(name, out.name, SCMI_SHORT_NAME_MAX_SIZE);
    dev_info(ph.dev, "i.MX CPU: name: %s\n", name);
    } else {
    dev_err(ph.dev, "i.MX cpu: Failed to get info of cpu(%u)\n", cpuid);
    }
    ph.xops.xfer_put(ph, t);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn scmi_imx_cpu_protocol_init(ph: *const scmi_protocol_handle) -> c_int {
    static int scmi_imx_cpu_protocol_init(const struct scmi_protocol_handle *ph)
    {
    struct scmi_imx_cpu_info *info;
    int ret, i;
    dev_info(ph.dev, "NXP SM CPU Protocol Version %d.%d\n",
    PROTOCOL_REV_MAJOR(ph.version), PROTOCOL_REV_MINOR(ph.version));
    info = devm_kzalloc(ph.dev, sizeof(*info), GFP_KERNEL);
    if (!info)
    return -ENOMEM;
    ret = scmi_imx_cpu_protocol_attributes_get(ph, info);
    if (ret)
    return ret;
    for (i = 0; i < info.nr_cpu; i++) {
    ret = scmi_imx_cpu_attributes_get(ph, i);
    if (ret)
    return ret;
    }
    return ph.set_priv(ph, info);
    }
    static const struct scmi_protocol scmi_imx_cpu = {
    .id = SCMI_PROTOCOL_IMX_CPU,
    .owner = THIS_MODULE,
    .instance_init = &scmi_imx_cpu_protocol_init,
    .ops = &scmi_imx_cpu_proto_ops,
    .supported_version = SCMI_PROTOCOL_SUPPORTED_VERSION,
    .vendor_id = SCMI_IMX_VENDOR,
    .sub_vendor_id = SCMI_IMX_SUBVENDOR,
    };
    module_scmi_protocol(scmi_imx_cpu);
    MODULE_ALIAS("scmi-protocol-" __stringify(SCMI_PROTOCOL_IMX_CPU) "-" SCMI_IMX_VENDOR);
    MODULE_DESCRIPTION("i.MX SCMI CPU driver");
    MODULE_LICENSE("GPL");
