/*
Copyright (c) 2016 Saurav Sachidanand

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.
*/

//! Virtual Machine Control Structure (VMCS) field IDs

use libc::*;

pub const VMCS_VPID                        : uint32_t = 0x00000000;
pub const VMCS_CTRL_POSTED_INT_N_VECTOR    : uint32_t = 0x00000002;
pub const VMCS_CTRL_EPTP_INDEX             : uint32_t = 0x00000004;
pub const VMCS_GUEST_ES                    : uint32_t = 0x00000800;
pub const VMCS_GUEST_CS                    : uint32_t = 0x00000802;
pub const VMCS_GUEST_SS                    : uint32_t = 0x00000804;
pub const VMCS_GUEST_DS                    : uint32_t = 0x00000806;
pub const VMCS_GUEST_FS                    : uint32_t = 0x00000808;
pub const VMCS_GUEST_GS                    : uint32_t = 0x0000080a;
pub const VMCS_GUEST_LDTR                  : uint32_t = 0x0000080c;
pub const VMCS_GUEST_TR                    : uint32_t = 0x0000080e;
pub const VMCS_GUEST_INT_STATUS            : uint32_t = 0x00000810;
pub const VMCS_HOST_ES                     : uint32_t = 0x00000c00;
pub const VMCS_HOST_CS                     : uint32_t = 0x00000c02;
pub const VMCS_HOST_SS                     : uint32_t = 0x00000c04;
pub const VMCS_HOST_DS                     : uint32_t = 0x00000c06;
pub const VMCS_HOST_FS                     : uint32_t = 0x00000c08;
pub const VMCS_HOST_GS                     : uint32_t = 0x00000c0a;
pub const VMCS_HOST_TR                     : uint32_t = 0x00000c0c;
pub const VMCS_CTRL_IO_BITMAP_A            : uint32_t = 0x00002000;
pub const VMCS_CTRL_IO_BITMAP_B            : uint32_t = 0x00002002;
pub const VMCS_CTRL_MSR_BITMAPS            : uint32_t = 0x00002004;
pub const VMCS_CTRL_VMEXIT_MSR_STORE_ADDR  : uint32_t = 0x00002006;
pub const VMCS_CTRL_VMEXIT_MSR_LOAD_ADDR   : uint32_t = 0x00002008;
pub const VMCS_CTRL_VMENTRY_MSR_LOAD_ADDR  : uint32_t = 0x0000200a;
pub const VMCS_CTRL_EXECUTIVE_VMCS_PTR     : uint32_t = 0x0000200c;
pub const VMCS_CTRL_TSC_OFFSET             : uint32_t = 0x00002010;
pub const VMCS_CTRL_VIRTUAL_APIC           : uint32_t = 0x00002012;
pub const VMCS_CTRL_APIC_ACCESS            : uint32_t = 0x00002014;
pub const VMCS_CTRL_POSTED_INT_DESC_ADDR   : uint32_t = 0x00002016;
pub const VMCS_CTRL_VMFUNC_CTRL            : uint32_t = 0x00002018;
pub const VMCS_CTRL_EPTP                   : uint32_t = 0x0000201a;
pub const VMCS_CTRL_EOI_EXIT_BITMAP_0      : uint32_t = 0x0000201c;
pub const VMCS_CTRL_EOI_EXIT_BITMAP_1      : uint32_t = 0x0000201e;
pub const VMCS_CTRL_EOI_EXIT_BITMAP_2      : uint32_t = 0x00002020;
pub const VMCS_CTRL_EOI_EXIT_BITMAP_3      : uint32_t = 0x00002022;
pub const VMCS_CTRL_EPTP_LIST_ADDR         : uint32_t = 0x00002024;
pub const VMCS_CTRL_VMREAD_BITMAP_ADDR     : uint32_t = 0x00002026;
pub const VMCS_CTRL_VMWRITE_BITMAP_ADDR    : uint32_t = 0x00002028;
pub const VMCS_CTRL_VIRT_EXC_INFO_ADDR     : uint32_t = 0x0000202a;
pub const VMCS_CTRL_XSS_EXITING_BITMAP     : uint32_t = 0x0000202c;
pub const VMCS_GUEST_PHYSICAL_ADDRESS      : uint32_t = 0x00002400;
pub const VMCS_GUEST_LINK_POINTER          : uint32_t = 0x00002800;
pub const VMCS_GUEST_IA32_DEBUGCTL         : uint32_t = 0x00002802;
pub const VMCS_GUEST_IA32_PAT              : uint32_t = 0x00002804;
pub const VMCS_GUEST_IA32_EFER             : uint32_t = 0x00002806;
pub const VMCS_GUEST_IA32_PERF_GLOBAL_CTRL : uint32_t = 0x00002808;
pub const VMCS_GUEST_PDPTE0                : uint32_t = 0x0000280a;
pub const VMCS_GUEST_PDPTE1                : uint32_t = 0x0000280c;
pub const VMCS_GUEST_PDPTE2                : uint32_t = 0x0000280e;
pub const VMCS_GUEST_PDPTE3                : uint32_t = 0x00002810;
pub const VMCS_HOST_IA32_PAT               : uint32_t = 0x00002c00;
pub const VMCS_HOST_IA32_EFER              : uint32_t = 0x00002c02;
pub const VMCS_HOST_IA32_PERF_GLOBAL_CTRL  : uint32_t = 0x00002c04;
pub const VMCS_CTRL_PIN_BASED              : uint32_t = 0x00004000;
pub const VMCS_CTRL_CPU_BASED              : uint32_t = 0x00004002;
pub const VMCS_CTRL_EXC_BITMAP             : uint32_t = 0x00004004;
pub const VMCS_CTRL_PF_ERROR_MASK          : uint32_t = 0x00004006;
pub const VMCS_CTRL_PF_ERROR_MATCH         : uint32_t = 0x00004008;
pub const VMCS_CTRL_CR3_COUNT              : uint32_t = 0x0000400a;
pub const VMCS_CTRL_VMEXIT_CONTROLS        : uint32_t = 0x0000400c;
pub const VMCS_CTRL_VMEXIT_MSR_STORE_COUNT : uint32_t = 0x0000400e;
pub const VMCS_CTRL_VMEXIT_MSR_LOAD_COUNT  : uint32_t = 0x00004010;
pub const VMCS_CTRL_VMENTRY_CONTROLS       : uint32_t = 0x00004012;
pub const VMCS_CTRL_VMENTRY_MSR_LOAD_COUNT : uint32_t = 0x00004014;
pub const VMCS_CTRL_VMENTRY_IRQ_INFO       : uint32_t = 0x00004016;
pub const VMCS_CTRL_VMENTRY_EXC_ERROR      : uint32_t = 0x00004018;
pub const VMCS_CTRL_VMENTRY_INSTR_LEN      : uint32_t = 0x0000401a;
pub const VMCS_CTRL_TPR_THRESHOLD          : uint32_t = 0x0000401c;
pub const VMCS_CTRL_CPU_BASED2             : uint32_t = 0x0000401e;
pub const VMCS_CTRL_PLE_GAP                : uint32_t = 0x00004020;
pub const VMCS_CTRL_PLE_WINDOW             : uint32_t = 0x00004022;
pub const VMCS_RO_INSTR_ERROR              : uint32_t = 0x00004400;
pub const VMCS_RO_EXIT_REASON              : uint32_t = 0x00004402;
pub const VMCS_RO_VMEXIT_IRQ_INFO          : uint32_t = 0x00004404;
pub const VMCS_RO_VMEXIT_IRQ_ERROR         : uint32_t = 0x00004406;
pub const VMCS_RO_IDT_VECTOR_INFO          : uint32_t = 0x00004408;
pub const VMCS_RO_IDT_VECTOR_ERROR         : uint32_t = 0x0000440a;
pub const VMCS_RO_VMEXIT_INSTR_LEN         : uint32_t = 0x0000440c;
pub const VMCS_RO_VMX_INSTR_INFO           : uint32_t = 0x0000440e;
pub const VMCS_GUEST_ES_LIMIT              : uint32_t = 0x00004800;
pub const VMCS_GUEST_CS_LIMIT              : uint32_t = 0x00004802;
pub const VMCS_GUEST_SS_LIMIT              : uint32_t = 0x00004804;
pub const VMCS_GUEST_DS_LIMIT              : uint32_t = 0x00004806;
pub const VMCS_GUEST_FS_LIMIT              : uint32_t = 0x00004808;
pub const VMCS_GUEST_GS_LIMIT              : uint32_t = 0x0000480a;
pub const VMCS_GUEST_LDTR_LIMIT            : uint32_t = 0x0000480c;
pub const VMCS_GUEST_TR_LIMIT              : uint32_t = 0x0000480e;
pub const VMCS_GUEST_GDTR_LIMIT            : uint32_t = 0x00004810;
pub const VMCS_GUEST_IDTR_LIMIT            : uint32_t = 0x00004812;
pub const VMCS_GUEST_ES_AR                 : uint32_t = 0x00004814;
pub const VMCS_GUEST_CS_AR                 : uint32_t = 0x00004816;
pub const VMCS_GUEST_SS_AR                 : uint32_t = 0x00004818;
pub const VMCS_GUEST_DS_AR                 : uint32_t = 0x0000481a;
pub const VMCS_GUEST_FS_AR                 : uint32_t = 0x0000481c;
pub const VMCS_GUEST_GS_AR                 : uint32_t = 0x0000481e;
pub const VMCS_GUEST_LDTR_AR               : uint32_t = 0x00004820;
pub const VMCS_GUEST_TR_AR                 : uint32_t = 0x00004822;
pub const VMCS_GUEST_IGNORE_IRQ            : uint32_t = 0x00004824;
pub const VMCS_GUEST_ACTIVITY_STATE        : uint32_t = 0x00004826;
pub const VMCS_GUEST_SMBASE                : uint32_t = 0x00004828;
pub const VMCS_GUEST_IA32_SYSENTER_CS      : uint32_t = 0x0000482a;
pub const VMCS_GUEST_VMX_TIMER_VALUE       : uint32_t = 0x0000482e;
pub const VMCS_HOST_IA32_SYSENTER_CS       : uint32_t = 0x00004c00;
pub const VMCS_CTRL_CR0_MASK               : uint32_t = 0x00006000;
pub const VMCS_CTRL_CR4_MASK               : uint32_t = 0x00006002;
pub const VMCS_CTRL_CR0_SHADOW             : uint32_t = 0x00006004;
pub const VMCS_CTRL_CR4_SHADOW             : uint32_t = 0x00006006;
pub const VMCS_CTRL_CR3_VALUE0             : uint32_t = 0x00006008;
pub const VMCS_CTRL_CR3_VALUE1             : uint32_t = 0x0000600a;
pub const VMCS_CTRL_CR3_VALUE2             : uint32_t = 0x0000600c;
pub const VMCS_CTRL_CR3_VALUE3             : uint32_t = 0x0000600e;
pub const VMCS_RO_EXIT_QUALIFIC            : uint32_t = 0x00006400;
pub const VMCS_RO_IO_RCX                   : uint32_t = 0x00006402;
pub const VMCS_RO_IO_RSI                   : uint32_t = 0x00006404;
pub const VMCS_RO_IO_RDI                   : uint32_t = 0x00006406;
pub const VMCS_RO_IO_RIP                   : uint32_t = 0x00006408;
pub const VMCS_RO_GUEST_LIN_ADDR           : uint32_t = 0x0000640a;
pub const VMCS_GUEST_CR0                   : uint32_t = 0x00006800;
pub const VMCS_GUEST_CR3                   : uint32_t = 0x00006802;
pub const VMCS_GUEST_CR4                   : uint32_t = 0x00006804;
pub const VMCS_GUEST_ES_BASE               : uint32_t = 0x00006806;
pub const VMCS_GUEST_CS_BASE               : uint32_t = 0x00006808;
pub const VMCS_GUEST_SS_BASE               : uint32_t = 0x0000680a;
pub const VMCS_GUEST_DS_BASE               : uint32_t = 0x0000680c;
pub const VMCS_GUEST_FS_BASE               : uint32_t = 0x0000680e;
pub const VMCS_GUEST_GS_BASE               : uint32_t = 0x00006810;
pub const VMCS_GUEST_LDTR_BASE             : uint32_t = 0x00006812;
pub const VMCS_GUEST_TR_BASE               : uint32_t = 0x00006814;
pub const VMCS_GUEST_GDTR_BASE             : uint32_t = 0x00006816;
pub const VMCS_GUEST_IDTR_BASE             : uint32_t = 0x00006818;
pub const VMCS_GUEST_DR7                   : uint32_t = 0x0000681a;
pub const VMCS_GUEST_RSP                   : uint32_t = 0x0000681c;
pub const VMCS_GUEST_RIP                   : uint32_t = 0x0000681e;
pub const VMCS_GUEST_RFLAGS                : uint32_t = 0x00006820;
pub const VMCS_GUEST_DEBUG_EXC             : uint32_t = 0x00006822;
pub const VMCS_GUEST_SYSENTER_ESP          : uint32_t = 0x00006824;
pub const VMCS_GUEST_SYSENTER_EIP          : uint32_t = 0x00006826;
pub const VMCS_HOST_CR0                    : uint32_t = 0x00006c00;
pub const VMCS_HOST_CR3                    : uint32_t = 0x00006c02;
pub const VMCS_HOST_CR4                    : uint32_t = 0x00006c04;
pub const VMCS_HOST_FS_BASE                : uint32_t = 0x00006c06;
pub const VMCS_HOST_GS_BASE                : uint32_t = 0x00006c08;
pub const VMCS_HOST_TR_BASE                : uint32_t = 0x00006c0a;
pub const VMCS_HOST_GDTR_BASE              : uint32_t = 0x00006c0c;
pub const VMCS_HOST_IDTR_BASE              : uint32_t = 0x00006c0e;
pub const VMCS_HOST_IA32_SYSENTER_ESP      : uint32_t = 0x00006c10;
pub const VMCS_HOST_IA32_SYSENTER_EIP      : uint32_t = 0x00006c12;
pub const VMCS_HOST_RSP                    : uint32_t = 0x00006c14;
pub const VMCS_HOST_RIP                    : uint32_t = 0x00006c16;
pub const VMCS_MAX                         : uint32_t = 0x00006c18;
