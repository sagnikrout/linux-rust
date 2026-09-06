//! Automatically rewritten from C to Rust
//! Source: drivers/crypto/hisilicon/sec2/sec_main.c
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
// Copyright (c) 2019 HiSilicon Limited.

pub const CAP_FILE_PERMISSION: c_int = 0444;
pub const SEC_VF_NUM: c_int = 63;
pub const SEC_QUEUE_NUM_V1: c_int = 4096;
pub const PCI_DEVICE_ID_HUAWEI_SEC_PF: c_uint = 0xa255;
pub const SEC_BD_ERR_CHK_EN0: c_uint = 0xEFFFFFFF;
pub const SEC_BD_ERR_CHK_EN1: c_uint = 0x7ffff7fd;
pub const SEC_BD_ERR_CHK_EN3: c_uint = 0xffffbfff;
pub const SEC_SQE_SIZE: c_int = 128;
pub const SEC_PF_DEF_Q_NUM: c_int = 256;
pub const SEC_PF_DEF_Q_BASE: c_int = 0;
pub const SEC_CTX_Q_NUM_DEF: c_int = 2;
pub const SEC_CTX_Q_NUM_MAX: c_int = 32;
pub const SEC_CTRL_CNT_CLR_CE: c_uint = 0x301120;

pub const SEC_CORE_INT_SOURCE: c_uint = 0x301010;
pub const SEC_CORE_INT_MASK: c_uint = 0x301000;
pub const SEC_CORE_INT_STATUS: c_uint = 0x301008;
pub const SEC_CORE_SRAM_ECC_ERR_INFO: c_uint = 0x301C14;
pub const SEC_ECC_NUM: c_int = 16;
pub const SEC_ECC_MASH: c_uint = 0xFF;
pub const SEC_CORE_INT_DISABLE: c_uint = 0x0;
pub const SEC_RAS_CE_REG: c_uint = 0x301050;
pub const SEC_RAS_FE_REG: c_uint = 0x301054;
pub const SEC_RAS_NFE_REG: c_uint = 0x301058;
pub const SEC_RAS_FE_ENB_MSK: c_uint = 0x0;
pub const SEC_OOO_SHUTDOWN_SEL: c_uint = 0x301014;
pub const SEC_RAS_DISABLE: c_uint = 0x0;

pub const SEC_MEM_START_INIT_REG: c_uint = 0x301100;
pub const SEC_MEM_INIT_DONE_REG: c_uint = 0x301104;
// clock gating
pub const SEC_CONTROL_REG: c_uint = 0x301200;
pub const SEC_DYNAMIC_GATE_REG: c_uint = 0x30121c;
pub const SEC_CORE_AUTO_GATE: c_uint = 0x30212c;
pub const SEC_DYNAMIC_GATE_EN: c_uint = 0x7fff;

pub const SEC_TRNG_EN_SHIFT: c_int = 8;

pub const SEC_AXI_SHUTDOWN_DISABLE: c_uint = 0xFFFFEFFF;
pub const SEC_INTERFACE_USER_CTRL0_REG: c_uint = 0x301220;
pub const SEC_INTERFACE_USER_CTRL1_REG: c_uint = 0x301224;
pub const SEC_SAA_EN_REG: c_uint = 0x301270;
pub const SEC_BD_ERR_CHK_EN_REG0: c_uint = 0x301380;
pub const SEC_BD_ERR_CHK_EN_REG1: c_uint = 0x301384;
pub const SEC_BD_ERR_CHK_EN_REG3: c_uint = 0x30138c;

    SEC_USER1_ENABLE_DATA_SSV | \
    SEC_USER1_WB_CONTEXT_SSV |  \
    SEC_USER1_WB_DATA_SSV)

pub const SEC_INTERFACE_USER_CTRL0_REG_V3: c_uint = 0x302220;
pub const SEC_INTERFACE_USER_CTRL1_REG_V3: c_uint = 0x302224;

pub const SEC_USER1_SMMU_MASK_V3: c_uint = 0xFF79E79E;

pub const SEC_PREFETCH_CFG: c_uint = 0x301130;
pub const SEC_SVA_TRANS: c_uint = 0x301EC4;

pub const SEC_SVA_PREFETCH_INFO: c_uint = 0x301ED4;

pub const SEC_WAIT_SVA_READY: c_int = 500000;
pub const SEC_READ_SVA_STATUS_TIMES: c_int = 3;
pub const SEC_WAIT_US_MIN: c_int = 10;
pub const SEC_WAIT_US_MAX: c_int = 20;
pub const SEC_WAIT_QP_US_MIN: c_int = 1000;
pub const SEC_WAIT_QP_US_MAX: c_int = 2000;
pub const SEC_MAX_WAIT_TIMES: c_int = 2000;
pub const SEC_DELAY_10_US: c_int = 10;
pub const SEC_POLL_TIMEOUT_US: c_int = 1000;
pub const SEC_DBGFS_VAL_MAX_LEN: c_int = 20;
pub const SEC_SINGLE_PORT_MAX_TRANS: c_uint = 0x2060;
pub const SEC_SQE_MASK_OFFSET: c_int = 16;
pub const SEC_SQE_MASK_LEN: c_int = 108;
pub const SEC_SHAPER_TYPE_RATE: c_int = 400;
pub const SEC_DFX_BASE: c_uint = 0x301000;
pub const SEC_DFX_CORE: c_uint = 0x302100;
pub const SEC_DFX_COMMON1: c_uint = 0x301600;
pub const SEC_DFX_COMMON2: c_uint = 0x301C00;
pub const SEC_DFX_BASE_LEN: c_uint = 0x9D;
pub const SEC_DFX_CORE_LEN: c_uint = 0x32B;
pub const SEC_DFX_COMMON1_LEN: c_uint = 0x45;
pub const SEC_DFX_COMMON2_LEN: c_uint = 0xBA;
pub const SEC_ALG_BITMAP_SHIFT: c_int = 32;

    GENMASK(24, 21))

    GENMASK_ULL(42, 25))

    GENMASK_ULL(45, 43))
pub const SEC_MAX_CHANNEL_NUM: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sec_hw_error {
    pub int_msk: u32,
    pub msg: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sec_dfx_item {
    pub name: *const c_char,
    pub offset: u32,
}

    static const char sec_name[] = "hisi_sec2";
    static struct dentry *sec_debugfs_root;
    static struct hisi_qm_list sec_devices = {
    .register_to_crypto	= sec_register_to_crypto,
    .unregister_from_crypto	= sec_unregister_from_crypto,
    };
    static const struct hisi_qm_cap_info sec_basic_info[] = {
    {SEC_QM_NFE_MASK_CAP,   0x3124, 0, GENMASK(31, 0), 0x0, 0x1C77, 0x7C77},
    {SEC_QM_RESET_MASK_CAP, 0x3128, 0, GENMASK(31, 0), 0x0, 0xC77, 0x6C77},
    {SEC_QM_OOO_SHUTDOWN_MASK_CAP, 0x3128, 0, GENMASK(31, 0), 0x0, 0x4, 0x6C77},
    {SEC_QM_CE_MASK_CAP,    0x312C, 0, GENMASK(31, 0), 0x0, 0x8, 0x8},
    {SEC_NFE_MASK_CAP,      0x3130, 0, GENMASK(31, 0), 0x0, 0x177, 0x60177},
    {SEC_RESET_MASK_CAP,    0x3134, 0, GENMASK(31, 0), 0x0, 0x177, 0x177},
    {SEC_OOO_SHUTDOWN_MASK_CAP, 0x3134, 0, GENMASK(31, 0), 0x0, 0x4, 0x177},
    {SEC_CE_MASK_CAP,       0x3138, 0, GENMASK(31, 0), 0x0, 0x88, 0xC088},
    {SEC_CLUSTER_NUM_CAP, 0x313c, 20, GENMASK(3, 0), 0x1, 0x1, 0x1},
    {SEC_CORE_TYPE_NUM_CAP, 0x313c, 16, GENMASK(3, 0), 0x1, 0x1, 0x1},
    {SEC_CORE_NUM_CAP, 0x313c, 8, GENMASK(7, 0), 0x4, 0x4, 0x4},
    {SEC_CORES_PER_CLUSTER_NUM_CAP, 0x313c, 0, GENMASK(7, 0), 0x4, 0x4, 0x4},
    {SEC_CORE_ENABLE_BITMAP, 0x3140, 0, GENMASK(31, 0), 0x17F, 0x17F, 0xF},
    {SEC_DRV_ALG_BITMAP_LOW, 0x3144, 0, GENMASK(31, 0), 0x18050CB, 0x18050CB, 0x18670CF},
    {SEC_DRV_ALG_BITMAP_HIGH, 0x3148, 0, GENMASK(31, 0), 0x395C, 0x395C, 0x395C},
    {SEC_DEV_ALG_BITMAP_LOW, 0x314c, 0, GENMASK(31, 0), 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF},
    {SEC_DEV_ALG_BITMAP_HIGH, 0x3150, 0, GENMASK(31, 0), 0x3FFF, 0x3FFF, 0x3FFF},
    {SEC_CORE1_ALG_BITMAP_LOW, 0x3154, 0, GENMASK(31, 0), 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF},
    {SEC_CORE1_ALG_BITMAP_HIGH, 0x3158, 0, GENMASK(31, 0), 0x3FFF, 0x3FFF, 0x3FFF},
    {SEC_CORE2_ALG_BITMAP_LOW, 0x315c, 0, GENMASK(31, 0), 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF},
    {SEC_CORE2_ALG_BITMAP_HIGH, 0x3160, 0, GENMASK(31, 0), 0x3FFF, 0x3FFF, 0x3FFF},
    {SEC_CORE3_ALG_BITMAP_LOW, 0x3164, 0, GENMASK(31, 0), 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF},
    {SEC_CORE3_ALG_BITMAP_HIGH, 0x3168, 0, GENMASK(31, 0), 0x3FFF, 0x3FFF, 0x3FFF},
    {SEC_CORE4_ALG_BITMAP_LOW, 0x316c, 0, GENMASK(31, 0), 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF},
    {SEC_CORE4_ALG_BITMAP_HIGH, 0x3170, 0, GENMASK(31, 0), 0x3FFF, 0x3FFF, 0x3FFF},
    };
    static const struct hisi_qm_cap_query_info sec_cap_query_info[] = {
    {QM_RAS_NFE_TYPE, "QM_RAS_NFE_TYPE             ", 0x3124, 0x0, 0x1C77, 0x7C77},
    {QM_RAS_NFE_RESET, "QM_RAS_NFE_RESET            ", 0x3128, 0x0, 0xC77, 0x6C77},
    {QM_RAS_CE_TYPE, "QM_RAS_CE_TYPE              ", 0x312C, 0x0, 0x8, 0x8},
    {SEC_RAS_NFE_TYPE, "SEC_RAS_NFE_TYPE            ", 0x3130, 0x0, 0x177, 0x60177},
    {SEC_RAS_NFE_RESET, "SEC_RAS_NFE_RESET           ", 0x3134, 0x0, 0x177, 0x177},
    {SEC_RAS_CE_TYPE, "SEC_RAS_CE_TYPE             ", 0x3138, 0x0, 0x88, 0xC088},
    {SEC_CORE_INFO, "SEC_CORE_INFO               ", 0x313c, 0x110404, 0x110404, 0x110404},
    {SEC_CORE_EN, "SEC_CORE_EN                 ", 0x3140, 0x17F, 0x17F, 0xF},
    {SEC_DRV_ALG_BITMAP_LOW_TB, "SEC_DRV_ALG_BITMAP_LOW      ",
    0x3144, 0x18050CB, 0x18050CB, 0x18670CF},
    {SEC_DRV_ALG_BITMAP_HIGH_TB, "SEC_DRV_ALG_BITMAP_HIGH     ",
    0x3148, 0x395C, 0x395C, 0x395C},
    {SEC_ALG_BITMAP_LOW, "SEC_ALG_BITMAP_LOW          ",
    0x314c, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF},
    {SEC_ALG_BITMAP_HIGH, "SEC_ALG_BITMAP_HIGH         ", 0x3150, 0x3FFF, 0x3FFF, 0x3FFF},
    {SEC_CORE1_BITMAP_LOW, "SEC_CORE1_BITMAP_LOW        ",
    0x3154, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF},
    {SEC_CORE1_BITMAP_HIGH, "SEC_CORE1_BITMAP_HIGH       ", 0x3158, 0x3FFF, 0x3FFF, 0x3FFF},
    {SEC_CORE2_BITMAP_LOW, "SEC_CORE2_BITMAP_LOW        ",
    0x315c, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF},
    {SEC_CORE2_BITMAP_HIGH, "SEC_CORE2_BITMAP_HIGH       ", 0x3160, 0x3FFF, 0x3FFF, 0x3FFF},
    {SEC_CORE3_BITMAP_LOW, "SEC_CORE3_BITMAP_LOW        ",
    0x3164, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF},
    {SEC_CORE3_BITMAP_HIGH, "SEC_CORE3_BITMAP_HIGH       ", 0x3168, 0x3FFF, 0x3FFF, 0x3FFF},
    {SEC_CORE4_BITMAP_LOW, "SEC_CORE4_BITMAP_LOW        ",
    0x316c, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF},
    {SEC_CORE4_BITMAP_HIGH, "SEC_CORE4_BITMAP_HIGH       ", 0x3170, 0x3FFF, 0x3FFF, 0x3FFF},
    };
    static const struct qm_dev_alg sec_dev_algs[] = { {
    .alg_msk = SEC_CIPHER_BITMAP,
    .alg = "cipher\n",
    }, {
    .alg_msk = SEC_DIGEST_BITMAP,
    .alg = "digest\n",
    }, {
    .alg_msk = SEC_AEAD_BITMAP,
    .alg = "aead\n",
    },
    };
    static const struct sec_hw_error sec_hw_errors[] = {
    {
    .int_msk = BIT(0),
    .msg = "sec_axi_rresp_err_rint"
    },
    {
    .int_msk = BIT(1),
    .msg = "sec_axi_bresp_err_rint"
    },
    {
    .int_msk = BIT(2),
    .msg = "sec_ecc_2bit_err_rint"
    },
    {
    .int_msk = BIT(3),
    .msg = "sec_ecc_1bit_err_rint"
    },
    {
    .int_msk = BIT(4),
    .msg = "sec_req_trng_timeout_rint"
    },
    {
    .int_msk = BIT(5),
    .msg = "sec_fsm_hbeat_rint"
    },
    {
    .int_msk = BIT(6),
    .msg = "sec_channel_req_rng_timeout_rint"
    },
    {
    .int_msk = BIT(7),
    .msg = "sec_bd_err_rint"
    },
    {
    .int_msk = BIT(8),
    .msg = "sec_chain_buff_err_rint"
    },
    {
    .int_msk = BIT(14),
    .msg = "sec_no_secure_access"
    },
    {
    .int_msk = BIT(15),
    .msg = "sec_wrapping_key_auth_err"
    },
    {
    .int_msk = BIT(16),
    .msg = "sec_km_key_crc_fail"
    },
    {
    .int_msk = BIT(17),
    .msg = "sec_axi_poison_err"
    },
    {
    .int_msk = BIT(18),
    .msg = "sec_sva_err"
    },
    {}
    };
    static const char * const sec_dbg_file_name[] = {
    [SEC_CLEAR_ENABLE] = "clear_enable",
    };
    static struct sec_dfx_item sec_dfx_labels[] = {
    {"send_cnt", offsetof(struct sec_dfx, send_cnt)},
    {"recv_cnt", offsetof(struct sec_dfx, recv_cnt)},
    {"send_busy_cnt", offsetof(struct sec_dfx, send_busy_cnt)},
    {"recv_busy_cnt", offsetof(struct sec_dfx, recv_busy_cnt)},
    {"err_bd_cnt", offsetof(struct sec_dfx, err_bd_cnt)},
    {"invalid_req_cnt", offsetof(struct sec_dfx, invalid_req_cnt)},
    {"done_flag_cnt", offsetof(struct sec_dfx, done_flag_cnt)},
    };
    static const struct debugfs_reg32 sec_dfx_regs[] = {
    {"SEC_PF_ABNORMAL_INT_SOURCE    ",  0x301010},
    {"SEC_SAA_EN                    ",  0x301270},
    {"SEC_BD_LATENCY_MIN            ",  0x301600},
    {"SEC_BD_LATENCY_MAX            ",  0x301608},
    {"SEC_BD_LATENCY_AVG            ",  0x30160C},
    {"SEC_BD_NUM_IN_SAA0            ",  0x301670},
    {"SEC_BD_NUM_IN_SAA1            ",  0x301674},
    {"SEC_BD_NUM_IN_SEC             ",  0x301680},
    {"SEC_ECC_1BIT_CNT              ",  0x301C00},
    {"SEC_ECC_1BIT_INFO             ",  0x301C04},
    {"SEC_ECC_2BIT_CNT              ",  0x301C10},
    {"SEC_ECC_2BIT_INFO             ",  0x301C14},
    {"SEC_BD_SAA0                   ",  0x301C20},
    {"SEC_BD_SAA1                   ",  0x301C24},
    {"SEC_BD_SAA2                   ",  0x301C28},
    {"SEC_BD_SAA3                   ",  0x301C2C},
    {"SEC_BD_SAA4                   ",  0x301C30},
    {"SEC_BD_SAA5                   ",  0x301C34},
    {"SEC_BD_SAA6                   ",  0x301C38},
    {"SEC_BD_SAA7                   ",  0x301C3C},
    {"SEC_BD_SAA8                   ",  0x301C40},
    {"SEC_RAS_CE_ENABLE             ",  0x301050},
    {"SEC_RAS_FE_ENABLE             ",  0x301054},
    {"SEC_RAS_NFE_ENABLE            ",  0x301058},
    {"SEC_REQ_TRNG_TIME_TH          ",  0x30112C},
    {"SEC_CHANNEL_RNG_REQ_THLD      ",  0x302110},
    };
// define the SEC's dfx regs region and region length
    static struct dfx_diff_registers sec_diff_regs[] = {
    {
    .reg_offset = SEC_DFX_BASE,
    .reg_len = SEC_DFX_BASE_LEN,
    }, {
    .reg_offset = SEC_DFX_COMMON1,
    .reg_len = SEC_DFX_COMMON1_LEN,
    }, {
    .reg_offset = SEC_DFX_COMMON2,
    .reg_len = SEC_DFX_COMMON2_LEN,
    }, {
    .reg_offset = SEC_DFX_CORE,
    .reg_len = SEC_DFX_CORE_LEN,
    },
    };
#[no_mangle]
unsafe extern "C" fn sec_diff_regs_show(s: *mut seq_file, unused: *mut c_void) -> c_int {
    static int sec_diff_regs_show(struct seq_file *s, void *unused)
    {
    struct hisi_qm *qm = s.private;
    hisi_qm_acc_diff_regs_dump(qm, s, qm.debug.acc_diff_regs,
    ARRAY_SIZE(sec_diff_regs));
    return 0;
    }
    DEFINE_SHOW_ATTRIBUTE(sec_diff_regs);
    static bool pf_q_num_flag;
#[no_mangle]
unsafe extern "C" fn sec_pf_q_num_set(val: *const c_char, kp: *const kernel_param) -> c_int {
    static int sec_pf_q_num_set(const char *val, const struct kernel_param *kp)
    {
    pf_q_num_flag = true;
    return hisi_qm_q_num_set(val, kp, PCI_DEVICE_ID_HUAWEI_SEC_PF);
    }
    static const struct kernel_param_ops sec_pf_q_num_ops = {
    .set = sec_pf_q_num_set,
    .get = param_get_int,
    };
    let mut pf_q_num: static u32 = SEC_PF_DEF_Q_NUM;
    module_param_cb(pf_q_num, &sec_pf_q_num_ops, &pf_q_num, 0444);
    MODULE_PARM_DESC(pf_q_num, "Number of queues in PF(v1 2-4096, v2 2-1024)");
#[no_mangle]
unsafe extern "C" fn sec_ctx_q_num_set(val: *const c_char, kp: *const kernel_param) -> c_int {
    static int sec_ctx_q_num_set(const char *val, const struct kernel_param *kp)
    {
    u32 ctx_q_num;
    int ret;
    if (!val)
    return -EINVAL;
    ret = kstrtou32(val, 10, &ctx_q_num);
    if (ret)
    return -EINVAL;
    if (!ctx_q_num || ctx_q_num > SEC_CTX_Q_NUM_MAX || ctx_q_num & 0x1) {
    pr_err("ctx queue num[%u] is invalid!\n", ctx_q_num);
    return -EINVAL;
    }
    return param_set_int(val, kp);
    }
    static const struct kernel_param_ops sec_ctx_q_num_ops = {
    .set = sec_ctx_q_num_set,
    .get = param_get_int,
    };
    let mut ctx_q_num: static u32 = SEC_CTX_Q_NUM_DEF;
    module_param_cb(ctx_q_num, &sec_ctx_q_num_ops, &ctx_q_num, 0444);
    MODULE_PARM_DESC(ctx_q_num, "Queue num in ctx (2 default, 2, 4, ..., 32)");
    static const struct kernel_param_ops vfs_num_ops = {
    .set = vfs_num_set,
    .get = param_get_int,
    };
    static u32 vfs_num;
    module_param_cb(vfs_num, &vfs_num_ops, &vfs_num, 0444);
    MODULE_PARM_DESC(vfs_num, "Number of VFs to enable(1-63), 0(default)");
#[no_mangle]
pub unsafe extern "C" fn sec_destroy_qps(qps: *mut hisi_qp, qp_num: c_int) {
    void sec_destroy_qps(struct hisi_qp **qps, int qp_num)
    {
    hisi_qm_free_qps(qps, qp_num);
    kfree(qps);
    }
    struct hisi_qp **sec_create_qps(void)
    {
    let mut node: c_int = cpu_to_node(raw_smp_processor_id());
    let mut ctx_num: u32 = ctx_q_num;
    struct hisi_qp **qps;
    u8 *type;
    int ret;
    qps = kzalloc_objs(struct hisi_qp *, ctx_num);
    if (!qps)
    return core::ptr::null_mut();
// The type of SEC is all 0, so just allocated by kcalloc
    type = kcalloc(ctx_num, sizeof(u8), GFP_KERNEL);
    if (!type) {
    kfree(qps);
    return core::ptr::null_mut();
    }
    ret = hisi_qm_alloc_qps_node(&sec_devices, ctx_num, type, node, qps);
    if (ret) {
    kfree(type);
    kfree(qps);
    return core::ptr::null_mut();
    }
    kfree(type);
    return qps;
    }
#[no_mangle]
pub unsafe extern "C" fn sec_get_alg_bitmap(qm: *mut hisi_qm, high: u32, low: u32) -> u64 {
    u64 sec_get_alg_bitmap(struct hisi_qm *qm, u32 high, u32 low)
    {
    u32 cap_val_h, cap_val_l;
    cap_val_h = qm.cap_tables.dev_cap_table[high].cap_val;
    cap_val_l = qm.cap_tables.dev_cap_table[low].cap_val;
    return ((u64)cap_val_h << SEC_ALG_BITMAP_SHIFT) | (u64)cap_val_l;
    }
    static const struct kernel_param_ops sec_uacce_mode_ops = {
    .set = uacce_mode_set,
    .get = param_get_int,
    };
//
// uacce_mode = 0 means sec only register to crypto,
// uacce_mode = 1 means sec both register to crypto and uacce.
//
    let mut uacce_mode: static u32 = UACCE_MODE_NOUACCE;
    module_param_cb(uacce_mode, &sec_uacce_mode_ops, &uacce_mode, 0444);
    MODULE_PARM_DESC(uacce_mode, UACCE_MODE_DESC);
    static const struct pci_device_id sec_dev_ids[] = {
    { PCI_DEVICE(PCI_VENDOR_ID_HUAWEI, PCI_DEVICE_ID_HUAWEI_SEC_PF) },
    { PCI_DEVICE(PCI_VENDOR_ID_HUAWEI, PCI_DEVICE_ID_HUAWEI_SEC_VF) },
    { 0, }
    };
    MODULE_DEVICE_TABLE(pci, sec_dev_ids);
#[no_mangle]
unsafe extern "C" fn sec_set_endian(qm: *mut hisi_qm) {
    static void sec_set_endian(struct hisi_qm *qm)
    {
    u32 reg;
    reg = readl_relaxed(qm.io_base + SEC_CONTROL_REG);
    reg &= ~(BIT(1) | BIT(0));
    if (!IS_ENABLED(CONFIG_64BIT))
    reg |= BIT(1);
    if (!IS_ENABLED(CONFIG_CPU_LITTLE_ENDIAN))
    reg |= BIT(0);
    writel_relaxed(reg, qm.io_base + SEC_CONTROL_REG);
    }
#[no_mangle]
unsafe extern "C" fn sec_wait_sva_ready(qm: *mut hisi_qm, offset: __u32, mask: __u32) -> c_int {
    static int sec_wait_sva_ready(struct hisi_qm *qm, __u32 offset, __u32 mask)
    {
    u32 val, try_times = 0;
    let mut count: u8 = 0;
//
// Read the register value every 10-20us. If the value is 0 for three
// consecutive times, the SVA module is ready.
//
    do {
    val = readl(qm.io_base + offset);
    if (val & mask)
    count = 0;
#[no_mangle]
pub unsafe extern "C" fn if(SEC_READ_SVA_STATUS_TIMES: ++count ==) -> else {
    else if (++count == SEC_READ_SVA_STATUS_TIMES)
    break;
    usleep_range(SEC_WAIT_US_MIN, SEC_WAIT_US_MAX);
    } while (++try_times < SEC_WAIT_SVA_READY);
    if (try_times == SEC_WAIT_SVA_READY) {
    pci_err(qm.pdev, "failed to wait sva prefetch ready\n");
    return -ETIMEDOUT;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sec_close_sva_prefetch(qm: *mut hisi_qm) {
    static void sec_close_sva_prefetch(struct hisi_qm *qm)
    {
    u32 val;
    int ret;
    if (!test_bit(QM_SUPPORT_SVA_PREFETCH, &qm.caps))
    return;
    val = readl_relaxed(qm.io_base + SEC_PREFETCH_CFG);
    val |= SEC_PREFETCH_DISABLE;
    writel(val, qm.io_base + SEC_PREFETCH_CFG);
    ret = readl_relaxed_poll_timeout(qm.io_base + SEC_SVA_TRANS,
    val, !(val & SEC_SVA_DISABLE_READY),
    SEC_DELAY_10_US, SEC_POLL_TIMEOUT_US);
    if (ret)
    pci_err(qm.pdev, "failed to close sva prefetch\n");
    (void)sec_wait_sva_ready(qm, SEC_SVA_PREFETCH_INFO, SEC_SVA_STALL_NUM);
    }
#[no_mangle]
unsafe extern "C" fn sec_open_sva_prefetch(qm: *mut hisi_qm) {
    static void sec_open_sva_prefetch(struct hisi_qm *qm)
    {
    u32 val;
    int ret;
    if (!test_bit(QM_SUPPORT_SVA_PREFETCH, &qm.caps))
    return;
// Enable prefetch
    val = readl_relaxed(qm.io_base + SEC_PREFETCH_CFG);
    val &= SEC_PREFETCH_ENABLE;
    writel(val, qm.io_base + SEC_PREFETCH_CFG);
    ret = readl_relaxed_poll_timeout(qm.io_base + SEC_PREFETCH_CFG,
    val, !(val & SEC_PREFETCH_DISABLE),
    SEC_DELAY_10_US, SEC_POLL_TIMEOUT_US);
    if (ret) {
    pci_err(qm.pdev, "failed to open sva prefetch\n");
    sec_close_sva_prefetch(qm);
    return;
    }
    ret = sec_wait_sva_ready(qm, SEC_SVA_TRANS, SEC_SVA_PREFETCH_NUM);
    if (ret)
    sec_close_sva_prefetch(qm);
    }
#[no_mangle]
unsafe extern "C" fn sec_engine_sva_config(qm: *mut hisi_qm) {
    static void sec_engine_sva_config(struct hisi_qm *qm)
    {
    u32 reg;
    if (qm.ver > QM_HW_V2) {
    reg = readl_relaxed(qm.io_base +
    SEC_INTERFACE_USER_CTRL0_REG_V3);
    reg |= SEC_USER0_SMMU_NORMAL;
    writel_relaxed(reg, qm.io_base +
    SEC_INTERFACE_USER_CTRL0_REG_V3);
    reg = readl_relaxed(qm.io_base +
    SEC_INTERFACE_USER_CTRL1_REG_V3);
    reg &= SEC_USER1_SMMU_MASK_V3;
    reg |= SEC_USER1_SMMU_NORMAL_V3;
    writel_relaxed(reg, qm.io_base +
    SEC_INTERFACE_USER_CTRL1_REG_V3);
    } else {
    reg = readl_relaxed(qm.io_base +
    SEC_INTERFACE_USER_CTRL0_REG);
    reg |= SEC_USER0_SMMU_NORMAL;
    writel_relaxed(reg, qm.io_base +
    SEC_INTERFACE_USER_CTRL0_REG);
    reg = readl_relaxed(qm.io_base +
    SEC_INTERFACE_USER_CTRL1_REG);
    reg &= SEC_USER1_SMMU_MASK;
    if (qm.use_sva)
    reg |= SEC_USER1_SMMU_SVA;
    else
    reg |= SEC_USER1_SMMU_NORMAL;
    writel_relaxed(reg, qm.io_base +
    SEC_INTERFACE_USER_CTRL1_REG);
    }
    sec_open_sva_prefetch(qm);
    }
#[no_mangle]
unsafe extern "C" fn sec_enable_clock_gate(qm: *mut hisi_qm) {
    static void sec_enable_clock_gate(struct hisi_qm *qm)
    {
    u32 val;
    if (qm.ver < QM_HW_V3)
    return;
    val = readl_relaxed(qm.io_base + SEC_CONTROL_REG);
    val |= SEC_CLK_GATE_ENABLE;
    writel_relaxed(val, qm.io_base + SEC_CONTROL_REG);
    val = readl(qm.io_base + SEC_DYNAMIC_GATE_REG);
    val |= SEC_DYNAMIC_GATE_EN;
    writel(val, qm.io_base + SEC_DYNAMIC_GATE_REG);
    val = readl(qm.io_base + SEC_CORE_AUTO_GATE);
    val |= SEC_CORE_AUTO_GATE_EN;
    writel(val, qm.io_base + SEC_CORE_AUTO_GATE);
    }
#[no_mangle]
unsafe extern "C" fn sec_disable_clock_gate(qm: *mut hisi_qm) {
    static void sec_disable_clock_gate(struct hisi_qm *qm)
    {
    u32 val;
// Kunpeng920 needs to close clock gating
    val = readl_relaxed(qm.io_base + SEC_CONTROL_REG);
    val &= SEC_CLK_GATE_DISABLE;
    writel_relaxed(val, qm.io_base + SEC_CONTROL_REG);
    }
#[no_mangle]
unsafe extern "C" fn sec_engine_init(qm: *mut hisi_qm) -> c_int {
    static int sec_engine_init(struct hisi_qm *qm)
    {
    int ret;
    u32 reg;
// disable clock gate control before mem init
    sec_disable_clock_gate(qm);
    writel_relaxed(0x1, qm.io_base + SEC_MEM_START_INIT_REG);
    ret = readl_relaxed_poll_timeout(qm.io_base + SEC_MEM_INIT_DONE_REG,
    reg, reg & 0x1, SEC_DELAY_10_US,
    SEC_POLL_TIMEOUT_US);
    if (ret) {
    pci_err(qm.pdev, "fail to init sec mem\n");
    return ret;
    }
    reg = readl_relaxed(qm.io_base + SEC_CONTROL_REG);
    reg |= (0x1 << SEC_TRNG_EN_SHIFT);
    writel_relaxed(reg, qm.io_base + SEC_CONTROL_REG);
    sec_engine_sva_config(qm);
    writel(SEC_SINGLE_PORT_MAX_TRANS,
    qm.io_base + AM_CFG_SINGLE_PORT_MAX_TRANS);
    reg = hisi_qm_get_hw_info(qm, sec_basic_info, SEC_CORE_ENABLE_BITMAP, qm.cap_ver);
    writel(reg, qm.io_base + SEC_SAA_EN_REG);
    if (qm.ver < QM_HW_V3) {
// HW V2 enable sm4 extra mode, as ctr/ecb
    writel_relaxed(SEC_BD_ERR_CHK_EN0,
    qm.io_base + SEC_BD_ERR_CHK_EN_REG0);
// HW V2 enable sm4 xts mode multiple iv
    writel_relaxed(SEC_BD_ERR_CHK_EN1,
    qm.io_base + SEC_BD_ERR_CHK_EN_REG1);
    writel_relaxed(SEC_BD_ERR_CHK_EN3,
    qm.io_base + SEC_BD_ERR_CHK_EN_REG3);
    }
// config endian
    sec_set_endian(qm);
    sec_enable_clock_gate(qm);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sec_set_user_domain_and_cache(qm: *mut hisi_qm) -> c_int {
    static int sec_set_user_domain_and_cache(struct hisi_qm *qm)
    {
// qm user domain
    writel(AXUSER_BASE, qm.io_base + QM_ARUSER_M_CFG_1);
    writel(ARUSER_M_CFG_ENABLE, qm.io_base + QM_ARUSER_M_CFG_ENABLE);
    writel(AXUSER_BASE, qm.io_base + QM_AWUSER_M_CFG_1);
    writel(AWUSER_M_CFG_ENABLE, qm.io_base + QM_AWUSER_M_CFG_ENABLE);
    writel(WUSER_M_CFG_ENABLE, qm.io_base + QM_WUSER_M_CFG_ENABLE);
// qm cache
    writel(AXI_M_CFG, qm.io_base + QM_AXI_M_CFG);
    writel(AXI_M_CFG_ENABLE, qm.io_base + QM_AXI_M_CFG_ENABLE);
// disable FLR triggered by BME(bus master enable)
    writel(PEH_AXUSER_CFG, qm.io_base + QM_PEH_AXUSER_CFG);
    writel(PEH_AXUSER_CFG_ENABLE, qm.io_base + QM_PEH_AXUSER_CFG_ENABLE);
// enable sqc,cqc writeback
    writel(SQC_CACHE_ENABLE | CQC_CACHE_ENABLE | SQC_CACHE_WB_ENABLE |
    CQC_CACHE_WB_ENABLE | FIELD_PREP(SQC_CACHE_WB_THRD, 1) |
    FIELD_PREP(CQC_CACHE_WB_THRD, 1), qm.io_base + QM_CACHE_CTL);
    return sec_engine_init(qm);
    }
// sec_debug_regs_clear() - clear the sec debug regs
#[no_mangle]
unsafe extern "C" fn sec_debug_regs_clear(qm: *mut hisi_qm) {
    static void sec_debug_regs_clear(struct hisi_qm *qm)
    {
    int i;
// clear sec dfx regs
    writel(0x1, qm.io_base + SEC_CTRL_CNT_CLR_CE);
    for (i = 0; i < ARRAY_SIZE(sec_dfx_regs); i++)
    readl(qm.io_base + sec_dfx_regs[i].offset);
// clear rdclr_en
    writel(0x0, qm.io_base + SEC_CTRL_CNT_CLR_CE);
    hisi_qm_debug_regs_clear(qm);
    }
#[no_mangle]
unsafe extern "C" fn sec_master_ooo_ctrl(qm: *mut hisi_qm, enable: bool) {
    static void sec_master_ooo_ctrl(struct hisi_qm *qm, bool enable)
    {
    u32 val1, val2;
    val1 = readl(qm.io_base + SEC_CONTROL_REG);
    if (enable) {
    val1 |= SEC_AXI_SHUTDOWN_ENABLE;
    val2 = qm.err_info.dev_err.shutdown_mask;
    } else {
    val1 &= SEC_AXI_SHUTDOWN_DISABLE;
    val2 = 0x0;
    }
    if (qm.ver > QM_HW_V2)
    writel(val2, qm.io_base + SEC_OOO_SHUTDOWN_SEL);
    writel(val1, qm.io_base + SEC_CONTROL_REG);
    }
#[no_mangle]
unsafe extern "C" fn sec_hw_error_enable(qm: *mut hisi_qm) {
    static void sec_hw_error_enable(struct hisi_qm *qm)
    {
    struct hisi_qm_err_mask *dev_err = &qm.err_info.dev_err;
    let mut err_mask: u32 = dev_err.ce | dev_err.nfe | dev_err.fe;
    if (qm.ver == QM_HW_V1) {
    writel(SEC_CORE_INT_DISABLE, qm.io_base + SEC_CORE_INT_MASK);
    pci_info(qm.pdev, "V1 not support hw error handle\n");
    return;
    }
// clear SEC hw error source if having
    writel(SEC_RAS_CLEAR_ALL, qm.io_base + SEC_CORE_INT_SOURCE);
// enable RAS int
    writel(dev_err.ce, qm.io_base + SEC_RAS_CE_REG);
    writel(dev_err.fe, qm.io_base + SEC_RAS_FE_REG);
    writel(dev_err.nfe, qm.io_base + SEC_RAS_NFE_REG);
// enable SEC block master OOO when nfe occurs on Kunpeng930
    sec_master_ooo_ctrl(qm, true);
// enable SEC hw error interrupts
    writel(err_mask, qm.io_base + SEC_CORE_INT_MASK);
    }
#[no_mangle]
unsafe extern "C" fn sec_hw_error_disable(qm: *mut hisi_qm) {
    static void sec_hw_error_disable(struct hisi_qm *qm)
    {
// disable SEC hw error interrupts
    writel(SEC_CORE_INT_DISABLE, qm.io_base + SEC_CORE_INT_MASK);
// disable SEC block master OOO when nfe occurs on Kunpeng930
    sec_master_ooo_ctrl(qm, false);
// disable RAS int
    writel(SEC_RAS_DISABLE, qm.io_base + SEC_RAS_CE_REG);
    writel(SEC_RAS_DISABLE, qm.io_base + SEC_RAS_FE_REG);
    writel(SEC_RAS_DISABLE, qm.io_base + SEC_RAS_NFE_REG);
    }
#[no_mangle]
unsafe extern "C" fn sec_clear_enable_read(qm: *mut hisi_qm) -> u32 {
    static u32 sec_clear_enable_read(struct hisi_qm *qm)
    {
    return readl(qm.io_base + SEC_CTRL_CNT_CLR_CE) &
    SEC_CTRL_CNT_CLR_CE_BIT;
    }
#[no_mangle]
unsafe extern "C" fn sec_clear_enable_write(qm: *mut hisi_qm, val: u32) -> c_int {
    static int sec_clear_enable_write(struct hisi_qm *qm, u32 val)
    {
    u32 tmp;
    if (val != 1 && val)
    return -EINVAL;
    tmp = (readl(qm.io_base + SEC_CTRL_CNT_CLR_CE) &
    ~SEC_CTRL_CNT_CLR_CE_BIT) | val;
    writel(tmp, qm.io_base + SEC_CTRL_CNT_CLR_CE);
    return 0;
    }
    static ssize_t sec_debug_read(struct file *filp, char __user *buf,
    size_t count, loff_t *pos)
    {
    struct sec_debug_file *file = filp.private_data;
    char tbuf[SEC_DBGFS_VAL_MAX_LEN];
    struct hisi_qm *qm = file.qm;
    u32 val;
    int ret;
    ret = hisi_qm_get_dfx_access(qm);
    if (ret)
    return ret;
    spin_lock_irq(&file.lock);
    switch (file.index) {
    case SEC_CLEAR_ENABLE:
    val = sec_clear_enable_read(qm);
    break;
    default:
    goto err_input;
    }
    spin_unlock_irq(&file.lock);
    hisi_qm_put_dfx_access(qm);
    ret = snprintf(tbuf, SEC_DBGFS_VAL_MAX_LEN, "%u\n", val);
    return simple_read_from_buffer(buf, count, pos, tbuf, ret);
    err_input:
    spin_unlock_irq(&file.lock);
    hisi_qm_put_dfx_access(qm);
    return -EINVAL;
    }
    static ssize_t sec_debug_write(struct file *filp, const char __user *buf,
    size_t count, loff_t *pos)
    {
    struct sec_debug_file *file = filp.private_data;
    char tbuf[SEC_DBGFS_VAL_MAX_LEN];
    struct hisi_qm *qm = file.qm;
    unsigned long val;
    int len, ret;
    if (*pos != 0)
    return 0;
    if (count >= SEC_DBGFS_VAL_MAX_LEN)
    return -ENOSPC;
    len = simple_write_to_buffer(tbuf, SEC_DBGFS_VAL_MAX_LEN - 1,
    pos, buf, count);
    if (len < 0)
    return len;
    tbuf[len] = '\0';
    if (kstrtoul(tbuf, 0, &val))
    return -EFAULT;
    ret = hisi_qm_get_dfx_access(qm);
    if (ret)
    return ret;
    spin_lock_irq(&file.lock);
    switch (file.index) {
    case SEC_CLEAR_ENABLE:
    ret = sec_clear_enable_write(qm, val);
    if (ret)
    goto err_input;
    break;
    default:
    ret = -EINVAL;
    goto err_input;
    }
    ret = count;
    err_input:
    spin_unlock_irq(&file.lock);
    hisi_qm_put_dfx_access(qm);
    return ret;
    }
    static const struct file_operations sec_dbg_fops = {
    .owner = THIS_MODULE,
    .open = simple_open,
    .read = sec_debug_read,
    .write = sec_debug_write,
    };
#[no_mangle]
unsafe extern "C" fn sec_debugfs_atomic64_get(data: *mut c_void, val: *mut u64) -> c_int {
    static int sec_debugfs_atomic64_get(void *data, u64 *val)
    {
// val = atomic64_read((atomic64_t *)data);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sec_debugfs_atomic64_set(data: *mut c_void, val: u64) -> c_int {
    static int sec_debugfs_atomic64_set(void *data, u64 val)
    {
    if (val)
    return -EINVAL;
    atomic64_set((atomic64_t *)data, 0);
    return 0;
    }
    DEFINE_DEBUGFS_ATTRIBUTE(sec_atomic64_ops, sec_debugfs_atomic64_get,
    sec_debugfs_atomic64_set, "%llu\n");
#[no_mangle]
unsafe extern "C" fn sec_regs_show(s: *mut seq_file, unused: *mut c_void) -> c_int {
    static int sec_regs_show(struct seq_file *s, void *unused)
    {
    hisi_qm_regs_dump(s, s.private);
    return 0;
    }
    DEFINE_SHOW_ATTRIBUTE(sec_regs);
#[no_mangle]
unsafe extern "C" fn sec_cap_regs_show(s: *mut seq_file, unused: *mut c_void) -> c_int {
    static int sec_cap_regs_show(struct seq_file *s, void *unused)
    {
    struct hisi_qm *qm = s.private;
    u32 i, size;
    size = qm.cap_tables.qm_cap_size;
    for (i = 0; i < size; i++)
    seq_printf(s, "%s= 0x%08x\n", qm.cap_tables.qm_cap_table[i].name,
    qm.cap_tables.qm_cap_table[i].cap_val);
    size = qm.cap_tables.dev_cap_size;
    for (i = 0; i < size; i++)
    seq_printf(s, "%s= 0x%08x\n", qm.cap_tables.dev_cap_table[i].name,
    qm.cap_tables.dev_cap_table[i].cap_val);
    return 0;
    }
    DEFINE_SHOW_ATTRIBUTE(sec_cap_regs);
#[no_mangle]
unsafe extern "C" fn sec_core_debug_init(qm: *mut hisi_qm) -> c_int {
    static int sec_core_debug_init(struct hisi_qm *qm)
    {
    struct dfx_diff_registers *sec_regs = qm.debug.acc_diff_regs;
    struct sec_dev *sec = container_of(qm, struct sec_dev, qm);
    struct device *dev = &qm.pdev.dev;
    struct sec_dfx *dfx = &sec.debug.dfx;
    struct debugfs_regset32 *regset;
    struct dentry *tmp_d;
    int i;
    tmp_d = debugfs_create_dir("sec_dfx", qm.debug.debug_root);
    regset = devm_kzalloc(dev, sizeof(*regset), GFP_KERNEL);
    if (!regset)
    return -ENOMEM;
    regset.regs = sec_dfx_regs;
    regset.nregs = ARRAY_SIZE(sec_dfx_regs);
    regset.base = qm.io_base;
    regset.dev = dev;
    if (qm.pdev.device == PCI_DEVICE_ID_HUAWEI_SEC_PF)
    debugfs_create_file("regs", 0444, tmp_d, regset, &sec_regs_fops);
    if (qm.fun_type == QM_HW_PF && sec_regs)
    debugfs_create_file("diff_regs", 0444, tmp_d,
    qm, &sec_diff_regs_fops);
    for (i = 0; i < ARRAY_SIZE(sec_dfx_labels); i++) {
    atomic64_t *data = (atomic64_t *)((uintptr_t)dfx +
    sec_dfx_labels[i].offset);
    debugfs_create_file(sec_dfx_labels[i].name, 0644,
    tmp_d, data, &sec_atomic64_ops);
    }
    debugfs_create_file("cap_regs", CAP_FILE_PERMISSION,
    qm.debug.debug_root, qm, &sec_cap_regs_fops);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sec_debug_init(qm: *mut hisi_qm) -> c_int {
    static int sec_debug_init(struct hisi_qm *qm)
    {
    struct sec_dev *sec = container_of(qm, struct sec_dev, qm);
    int i;
    if (qm.pdev.device == PCI_DEVICE_ID_HUAWEI_SEC_PF) {
    for (i = SEC_CLEAR_ENABLE; i < SEC_DEBUG_FILE_NUM; i++) {
    spin_lock_init(&sec.debug.files[i].lock);
    sec.debug.files[i].index = i;
    sec.debug.files[i].qm = qm;
    debugfs_create_file(sec_dbg_file_name[i], 0600,
    qm.debug.debug_root,
    sec.debug.files + i,
    &sec_dbg_fops);
    }
    }
    return sec_core_debug_init(qm);
    }
#[no_mangle]
unsafe extern "C" fn sec_debugfs_init(qm: *mut hisi_qm) -> c_int {
    static int sec_debugfs_init(struct hisi_qm *qm)
    {
    struct device *dev = &qm.pdev.dev;
    int ret;
    ret = hisi_qm_regs_debugfs_init(qm, sec_diff_regs, ARRAY_SIZE(sec_diff_regs));
    if (ret) {
    dev_warn(dev, "Failed to init SEC diff regs!\n");
    return ret;
    }
    qm.debug.debug_root = debugfs_create_dir(dev_name(dev),
    sec_debugfs_root);
    qm.debug.sqe_mask_offset = SEC_SQE_MASK_OFFSET;
    qm.debug.sqe_mask_len = SEC_SQE_MASK_LEN;
    hisi_qm_debug_init(qm);
    ret = sec_debug_init(qm);
    if (ret)
    goto debugfs_remove;
    return 0;
    debugfs_remove:
    debugfs_remove_recursive(qm.debug.debug_root);
    hisi_qm_regs_debugfs_uninit(qm, ARRAY_SIZE(sec_diff_regs));
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn sec_debugfs_exit(qm: *mut hisi_qm) {
    static void sec_debugfs_exit(struct hisi_qm *qm)
    {
    debugfs_remove_recursive(qm.debug.debug_root);
    hisi_qm_regs_debugfs_uninit(qm, ARRAY_SIZE(sec_diff_regs));
    }
#[no_mangle]
unsafe extern "C" fn sec_show_last_regs_init(qm: *mut hisi_qm) -> c_int {
    static int sec_show_last_regs_init(struct hisi_qm *qm)
    {
    struct qm_debug *debug = &qm.debug;
    int i;
    debug.last_words = kcalloc(ARRAY_SIZE(sec_dfx_regs),
    sizeof(unsigned int), GFP_KERNEL);
    if (!debug.last_words)
    return -ENOMEM;
    for (i = 0; i < ARRAY_SIZE(sec_dfx_regs); i++)
    debug.last_words[i] = readl_relaxed(qm.io_base +
    sec_dfx_regs[i].offset);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sec_show_last_regs_uninit(qm: *mut hisi_qm) {
    static void sec_show_last_regs_uninit(struct hisi_qm *qm)
    {
    struct qm_debug *debug = &qm.debug;
    if (qm.fun_type == QM_HW_VF || !debug.last_words)
    return;
    kfree(debug.last_words);
    debug.last_words = core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn sec_show_last_dfx_regs(qm: *mut hisi_qm) {
    static void sec_show_last_dfx_regs(struct hisi_qm *qm)
    {
    struct qm_debug *debug = &qm.debug;
    struct pci_dev *pdev = qm.pdev;
    u32 val;
    int i;
    if (qm.fun_type == QM_HW_VF || !debug.last_words)
    return;
// dumps last word of the debugging registers during controller reset
    for (i = 0; i < ARRAY_SIZE(sec_dfx_regs); i++) {
    val = readl_relaxed(qm.io_base + sec_dfx_regs[i].offset);
    if (val != debug.last_words[i])
    pci_info(pdev, "%s \t= 0x%08x => 0x%08x\n",
    sec_dfx_regs[i].name, debug.last_words[i], val);
    }
    }
#[no_mangle]
unsafe extern "C" fn sec_log_hw_error(qm: *mut hisi_qm, err_sts: u32) {
    static void sec_log_hw_error(struct hisi_qm *qm, u32 err_sts)
    {
    const struct sec_hw_error *errs = sec_hw_errors;
    struct device *dev = &qm.pdev.dev;
    u32 err_val;
    while (errs.msg) {
    if (errs.int_msk & err_sts) {
    dev_err(dev, "%s [error status=0x%x] found\n",
    errs.msg, errs.int_msk);
    if (SEC_CORE_INT_STATUS_M_ECC & errs.int_msk) {
    err_val = readl(qm.io_base +
    SEC_CORE_SRAM_ECC_ERR_INFO);
    dev_err(dev, "multi ecc sram num=0x%x\n",
    ((err_val) >> SEC_ECC_NUM) &
    SEC_ECC_MASH);
    }
    }
    errs++;
    }
    }
#[no_mangle]
unsafe extern "C" fn sec_get_hw_err_status(qm: *mut hisi_qm) -> u32 {
    static u32 sec_get_hw_err_status(struct hisi_qm *qm)
    {
    return readl(qm.io_base + SEC_CORE_INT_STATUS);
    }
#[no_mangle]
unsafe extern "C" fn sec_clear_hw_err_status(qm: *mut hisi_qm, err_sts: u32) {
    static void sec_clear_hw_err_status(struct hisi_qm *qm, u32 err_sts)
    {
    writel(err_sts, qm.io_base + SEC_CORE_INT_SOURCE);
    }
#[no_mangle]
unsafe extern "C" fn sec_disable_error_report(qm: *mut hisi_qm, err_type: u32) {
    static void sec_disable_error_report(struct hisi_qm *qm, u32 err_type)
    {
    let mut nfe_mask: u32 = qm.err_info.dev_err.nfe;
    writel(nfe_mask & (~err_type), qm.io_base + SEC_RAS_NFE_REG);
    }
#[no_mangle]
unsafe extern "C" fn sec_enable_error_report(qm: *mut hisi_qm) {
    static void sec_enable_error_report(struct hisi_qm *qm)
    {
    let mut nfe_mask: u32 = qm.err_info.dev_err.nfe;
    let mut ce_mask: u32 = qm.err_info.dev_err.ce;
    writel(nfe_mask, qm.io_base + SEC_RAS_NFE_REG);
    writel(ce_mask, qm.io_base + SEC_RAS_CE_REG);
    }
#[no_mangle]
unsafe extern "C" fn sec_open_axi_master_ooo(qm: *mut hisi_qm) {
    static void sec_open_axi_master_ooo(struct hisi_qm *qm)
    {
    u32 val;
    val = readl(qm.io_base + SEC_CONTROL_REG);
    writel(val & SEC_AXI_SHUTDOWN_DISABLE, qm.io_base + SEC_CONTROL_REG);
    writel(val | SEC_AXI_SHUTDOWN_ENABLE, qm.io_base + SEC_CONTROL_REG);
    }
#[no_mangle]
unsafe extern "C" fn sec_get_err_result(qm: *mut hisi_qm) -> enum acc_err_result {
    static enum acc_err_result sec_get_err_result(struct hisi_qm *qm)
    {
    u32 err_status;
    err_status = sec_get_hw_err_status(qm);
    if (err_status) {
    if (err_status & qm.err_info.dev_err.ecc_2bits_mask)
    qm.err_status.is_dev_ecc_mbit = true;
    sec_log_hw_error(qm, err_status);
    if (err_status & qm.err_info.dev_err.reset_mask) {
// Disable the same error reporting until device is recovered.
    sec_disable_error_report(qm, err_status);
    return ACC_ERR_NEED_RESET;
    }
    sec_clear_hw_err_status(qm, err_status);
// Avoid firmware disable error report, re-enable.
    sec_enable_error_report(qm);
    }
    return ACC_ERR_RECOVERED;
    }
#[no_mangle]
unsafe extern "C" fn sec_dev_is_abnormal(qm: *mut hisi_qm) -> bool {
    static bool sec_dev_is_abnormal(struct hisi_qm *qm)
    {
    u32 err_status;
    err_status = sec_get_hw_err_status(qm);
    if (err_status & qm.err_info.dev_err.shutdown_mask)
    return true;
    return false;
    }
#[no_mangle]
unsafe extern "C" fn sec_disable_axi_error(qm: *mut hisi_qm) {
    static void sec_disable_axi_error(struct hisi_qm *qm)
    {
    struct hisi_qm_err_mask *dev_err = &qm.err_info.dev_err;
    let mut err_mask: u32 = dev_err.ce | dev_err.nfe | dev_err.fe;
    writel(err_mask & ~SEC_AXI_ERROR_MASK, qm.io_base + SEC_CORE_INT_MASK);
    if (qm.ver > QM_HW_V2)
    writel(dev_err.shutdown_mask & (~SEC_AXI_ERROR_MASK),
    qm.io_base + SEC_OOO_SHUTDOWN_SEL);
    }
#[no_mangle]
unsafe extern "C" fn sec_enable_axi_error(qm: *mut hisi_qm) {
    static void sec_enable_axi_error(struct hisi_qm *qm)
    {
    struct hisi_qm_err_mask *dev_err = &qm.err_info.dev_err;
    let mut err_mask: u32 = dev_err.ce | dev_err.nfe | dev_err.fe;
// clear axi error source
    writel(SEC_AXI_ERROR_MASK, qm.io_base + SEC_CORE_INT_SOURCE);
    writel(err_mask, qm.io_base + SEC_CORE_INT_MASK);
    if (qm.ver > QM_HW_V2)
    writel(dev_err.shutdown_mask, qm.io_base + SEC_OOO_SHUTDOWN_SEL);
    }
#[no_mangle]
unsafe extern "C" fn sec_err_info_init(qm: *mut hisi_qm) {
    static void sec_err_info_init(struct hisi_qm *qm)
    {
    struct hisi_qm_err_info *err_info = &qm.err_info;
    struct hisi_qm_err_mask *qm_err = &err_info.qm_err;
    struct hisi_qm_err_mask *dev_err = &err_info.dev_err;
    qm_err.fe = SEC_RAS_FE_ENB_MSK;
    qm_err.ce = hisi_qm_get_hw_info(qm, sec_basic_info, SEC_QM_CE_MASK_CAP, qm.cap_ver);
    qm_err.nfe = hisi_qm_get_hw_info(qm, sec_basic_info, SEC_QM_NFE_MASK_CAP, qm.cap_ver);
    qm_err.shutdown_mask = hisi_qm_get_hw_info(qm, sec_basic_info,
    SEC_QM_OOO_SHUTDOWN_MASK_CAP, qm.cap_ver);
    qm_err.reset_mask = hisi_qm_get_hw_info(qm, sec_basic_info,
    SEC_QM_RESET_MASK_CAP, qm.cap_ver);
    qm_err.ecc_2bits_mask = QM_ECC_MBIT;
    dev_err.fe = SEC_RAS_FE_ENB_MSK;
    dev_err.ce = hisi_qm_get_hw_info(qm, sec_basic_info, SEC_CE_MASK_CAP, qm.cap_ver);
    dev_err.nfe = hisi_qm_get_hw_info(qm, sec_basic_info, SEC_NFE_MASK_CAP, qm.cap_ver);
    dev_err.shutdown_mask = hisi_qm_get_hw_info(qm, sec_basic_info,
    SEC_OOO_SHUTDOWN_MASK_CAP, qm.cap_ver);
    dev_err.reset_mask = hisi_qm_get_hw_info(qm, sec_basic_info,
    SEC_RESET_MASK_CAP, qm.cap_ver);
    dev_err.ecc_2bits_mask = SEC_CORE_INT_STATUS_M_ECC;
    err_info.msi_wr_port = BIT(0);
    err_info.acpi_rst = "SRST";
    }
    static const struct hisi_qm_err_ini sec_err_ini = {
    .hw_init		= sec_set_user_domain_and_cache,
    .hw_err_enable		= sec_hw_error_enable,
    .hw_err_disable		= sec_hw_error_disable,
    .get_dev_hw_err_status	= sec_get_hw_err_status,
    .clear_dev_hw_err_status = sec_clear_hw_err_status,
    .open_axi_master_ooo	= sec_open_axi_master_ooo,
    .open_sva_prefetch	= sec_open_sva_prefetch,
    .close_sva_prefetch	= sec_close_sva_prefetch,
    .show_last_dfx_regs	= sec_show_last_dfx_regs,
    .err_info_init		= sec_err_info_init,
    .get_err_result		= sec_get_err_result,
    .dev_is_abnormal        = sec_dev_is_abnormal,
    .disable_axi_error	= sec_disable_axi_error,
    .enable_axi_error	= sec_enable_axi_error,
    };
#[no_mangle]
unsafe extern "C" fn sec_pf_probe_init(sec: *mut sec_dev) -> c_int {
    static int sec_pf_probe_init(struct sec_dev *sec)
    {
    struct hisi_qm *qm = &sec.qm;
    int ret;
    ret = sec_set_user_domain_and_cache(qm);
    if (ret)
    return ret;
    hisi_qm_dev_err_init(qm);
    sec_debug_regs_clear(qm);
    ret = sec_show_last_regs_init(qm);
    if (ret)
    pci_err(qm.pdev, "Failed to init last word regs!\n");
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn sec_pre_store_cap_reg(qm: *mut hisi_qm) -> c_int {
    static int sec_pre_store_cap_reg(struct hisi_qm *qm)
    {
    struct hisi_qm_cap_record *sec_cap;
    struct pci_dev *pdev = qm.pdev;
    size_t i, size;
    size = ARRAY_SIZE(sec_cap_query_info);
    sec_cap = devm_kcalloc(&pdev.dev, size, sizeof(*sec_cap), GFP_KERNEL);
    if (!sec_cap)
    return -ENOMEM;
    for (i = 0; i < size; i++) {
    sec_cap[i].type = sec_cap_query_info[i].type;
    sec_cap[i].name = sec_cap_query_info[i].name;
    sec_cap[i].cap_val = hisi_qm_get_cap_value(qm, sec_cap_query_info,
    i, qm.cap_ver);
    }
    qm.cap_tables.dev_cap_table = sec_cap;
    qm.cap_tables.dev_cap_size = size;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sec_set_channels(qm: *mut hisi_qm) {
    static void sec_set_channels(struct hisi_qm *qm)
    {
    struct qm_channel *channel_data = &qm.channel_data;
    channel_data.channel_num = SEC_MAX_CHANNEL_NUM;
    channel_data.channel_name[0] = "SEC";
    }
#[no_mangle]
unsafe extern "C" fn sec_qm_init(qm: *mut hisi_qm, pdev: *mut pci_dev) -> c_int {
    static int sec_qm_init(struct hisi_qm *qm, struct pci_dev *pdev)
    {
    u64 alg_msk;
    int ret;
    qm.pdev = pdev;
    qm.mode = uacce_mode;
    qm.sqe_size = SEC_SQE_SIZE;
    qm.dev_name = sec_name;
    qm.fun_type = (pdev.device == PCI_DEVICE_ID_HUAWEI_SEC_PF) ?
    QM_HW_PF : QM_HW_VF;
    if (qm.fun_type == QM_HW_PF) {
    qm.qp_base = SEC_PF_DEF_Q_BASE;
    qm.qp_num = pf_q_num;
    qm.debug.curr_qm_qp_num = pf_q_num;
    qm.qm_list = &sec_devices;
    qm.err_ini = &sec_err_ini;
    if (pf_q_num_flag)
    set_bit(QM_MODULE_PARAM, &qm.misc_ctl);
    } else if (qm.fun_type == QM_HW_VF && qm.ver == QM_HW_V1) {
//
// have no way to get qm configure in VM in v1 hardware,
// so currently force PF to uses SEC_PF_DEF_Q_NUM, and force
// to trigger only one VF in v1 hardware.
// v2 hardware has no such problem.
//
    qm.qp_base = SEC_PF_DEF_Q_NUM;
    qm.qp_num = SEC_QUEUE_NUM_V1 - SEC_PF_DEF_Q_NUM;
    }
    ret = hisi_qm_init(qm);
    if (ret) {
    pci_err(qm.pdev, "Failed to init sec qm configures!\n");
    return ret;
    }
    sec_set_channels(qm);
// Fetch and save the value of capability registers
    ret = sec_pre_store_cap_reg(qm);
    if (ret) {
    pci_err(qm.pdev, "Failed to pre-store capability registers!\n");
    hisi_qm_uninit(qm);
    return ret;
    }
    alg_msk = sec_get_alg_bitmap(qm, SEC_ALG_BITMAP_HIGH, SEC_ALG_BITMAP_LOW);
    ret = hisi_qm_set_algs(qm, alg_msk, sec_dev_algs, ARRAY_SIZE(sec_dev_algs));
    if (ret) {
    pci_err(qm.pdev, "Failed to set sec algs!\n");
    hisi_qm_uninit(qm);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn sec_qm_uninit(qm: *mut hisi_qm) {
    static void sec_qm_uninit(struct hisi_qm *qm)
    {
    hisi_qm_uninit(qm);
    }
#[no_mangle]
unsafe extern "C" fn sec_probe_init(sec: *mut sec_dev) -> c_int {
    static int sec_probe_init(struct sec_dev *sec)
    {
    let mut type_rate: u32 = SEC_SHAPER_TYPE_RATE;
    struct hisi_qm *qm = &sec.qm;
    int ret;
    if (qm.fun_type == QM_HW_PF) {
    ret = sec_pf_probe_init(sec);
    if (ret)
    return ret;
// enable shaper type 0
    if (qm.ver >= QM_HW_V3) {
    type_rate |= QM_SHAPER_ENABLE;
    qm.type_rate = type_rate;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sec_probe_uninit(qm: *mut hisi_qm) {
    static void sec_probe_uninit(struct hisi_qm *qm)
    {
    if (qm.fun_type == QM_HW_VF)
    return;
    sec_debug_regs_clear(qm);
    sec_show_last_regs_uninit(qm);
    sec_close_sva_prefetch(qm);
    hisi_qm_dev_err_uninit(qm);
    }
#[no_mangle]
unsafe extern "C" fn sec_iommu_used_check(sec: *mut sec_dev) {
    static void sec_iommu_used_check(struct sec_dev *sec)
    {
    struct iommu_domain *domain;
    struct device *dev = &sec.qm.pdev.dev;
    domain = iommu_get_domain_for_dev(dev);
// Check if iommu is used
    sec.iommu_used = false;
    if (domain) {
    if (domain.type & __IOMMU_DOMAIN_PAGING)
    sec.iommu_used = true;
    dev_info(dev, "SMMU Opened, the iommu type = %u\n",
    domain.type);
    }
    }
#[no_mangle]
unsafe extern "C" fn sec_probe(pdev: *mut pci_dev, id: *const pci_device_id) -> c_int {
    static int sec_probe(struct pci_dev *pdev, const struct pci_device_id *id)
    {
    struct sec_dev *sec;
    struct hisi_qm *qm;
    int ret;
    sec = devm_kzalloc(&pdev.dev, sizeof(*sec), GFP_KERNEL);
    if (!sec)
    return -ENOMEM;
    qm = &sec.qm;
    ret = sec_qm_init(qm, pdev);
    if (ret) {
    pci_err(pdev, "Failed to init SEC QM (%d)!\n", ret);
    return ret;
    }
    sec.ctx_q_num = ctx_q_num;
    sec_iommu_used_check(sec);
    ret = sec_probe_init(sec);
    if (ret) {
    pci_err(pdev, "Failed to probe!\n");
    goto err_qm_uninit;
    }
    ret = hisi_qm_start(qm);
    if (ret) {
    pci_err(pdev, "Failed to start sec qm!\n");
    goto err_probe_uninit;
    }
    ret = sec_debugfs_init(qm);
    if (ret)
    pci_warn(pdev, "Failed to init debugfs!\n");
    hisi_qm_add_list(qm, &sec_devices);
    ret = hisi_qm_alg_register(qm, &sec_devices, ctx_q_num);
    if (ret < 0) {
    pr_err("Failed to register driver to crypto.\n");
    goto err_qm_del_list;
    }
    ret = hisi_qm_register_uacce(qm);
    if (ret) {
    pci_err(pdev, "failed to register uacce (%d)!\n", ret);
    goto err_alg_unregister;
    }
    if (qm.fun_type == QM_HW_PF && vfs_num) {
    ret = hisi_qm_sriov_enable(pdev, vfs_num);
    if (ret < 0)
    goto err_alg_unregister;
    }
    hisi_qm_pm_init(qm);
    return 0;
    err_alg_unregister:
    hisi_qm_alg_unregister(qm, &sec_devices, ctx_q_num);
    err_qm_del_list:
    hisi_qm_del_list(qm, &sec_devices);
    sec_debugfs_exit(qm);
    hisi_qm_stop(qm, QM_NORMAL);
    err_probe_uninit:
    sec_probe_uninit(qm);
    err_qm_uninit:
    sec_qm_uninit(qm);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn sec_remove(pdev: *mut pci_dev) {
    static void sec_remove(struct pci_dev *pdev)
    {
    struct hisi_qm *qm = pci_get_drvdata(pdev);
    hisi_qm_pm_uninit(qm);
    hisi_qm_wait_task_finish(qm, &sec_devices);
    hisi_qm_alg_unregister(qm, &sec_devices, ctx_q_num);
    hisi_qm_del_list(qm, &sec_devices);
    if (qm.fun_type == QM_HW_PF && qm.vfs_num)
    hisi_qm_sriov_disable(pdev, true);
    sec_debugfs_exit(qm);
    (void)hisi_qm_stop(qm, QM_NORMAL);
    sec_probe_uninit(qm);
    sec_qm_uninit(qm);
    }
    static const struct dev_pm_ops sec_pm_ops = {
    SET_RUNTIME_PM_OPS(hisi_qm_suspend, hisi_qm_resume, core::ptr::null_mut())
    };
    static const struct pci_error_handlers sec_err_handler = {
    .error_detected = hisi_qm_dev_err_detected,
    .slot_reset	= hisi_qm_dev_slot_reset,
    .reset_prepare	= hisi_qm_reset_prepare,
    .reset_done	= hisi_qm_reset_done,
    };
    static struct pci_driver sec_pci_driver = {
    .name = "hisi_sec2",
    .id_table = sec_dev_ids,
    .probe = sec_probe,
    .remove = sec_remove,
    .err_handler = &sec_err_handler,
    .sriov_configure = IS_ENABLED(CONFIG_PCI_IOV) ?
    hisi_qm_sriov_configure : core::ptr::null_mut(),
    .shutdown = hisi_qm_dev_shutdown,
    .driver.pm = &sec_pm_ops,
    };
    struct pci_driver *hisi_sec_get_pf_driver(void)
    {
    return &sec_pci_driver;
    }
    EXPORT_SYMBOL_GPL(hisi_sec_get_pf_driver);
#[no_mangle]
unsafe extern "C" fn sec_register_debugfs() {
    static void sec_register_debugfs(void)
    {
    if (!debugfs_initialized())
    return;
    sec_debugfs_root = debugfs_create_dir("hisi_sec2", core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn sec_unregister_debugfs() {
    static void sec_unregister_debugfs(void)
    {
    debugfs_remove_recursive(sec_debugfs_root);
    }
#[no_mangle]
unsafe extern "C" fn sec_init() -> int __init {
    static int __init sec_init(void)
    {
    int ret;
    hisi_qm_init_list(&sec_devices);
    sec_register_debugfs();
    ret = pci_register_driver(&sec_pci_driver);
    if (ret < 0) {
    sec_unregister_debugfs();
    pr_err("Failed to register pci driver.\n");
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sec_exit() -> void __exit {
    static void __exit sec_exit(void)
    {
    pci_unregister_driver(&sec_pci_driver);
    sec_unregister_debugfs();
    }
    module_init(sec_init);
    module_exit(sec_exit);
    MODULE_LICENSE("GPL v2");
    MODULE_AUTHOR("Zaibo Xu <xuzaibo@huawei.com>");
    MODULE_AUTHOR("Longfang Liu <liulongfang@huawei.com>");
    MODULE_AUTHOR("Kai Ye <yekai13@huawei.com>");
    MODULE_AUTHOR("Wei Zhang <zhangwei375@huawei.com>");
    MODULE_DESCRIPTION("Driver for HiSilicon SEC accelerator");
