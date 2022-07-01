/// Undocumented API
pub mod private {
    use crate::prelude_::*;

    nvstruct! {
        pub struct NV_GPU_CLIENT_VOLT_RAILS_STATUS_V1 {
            pub version: NvVersion,
            pub flags: u32,
            pub zero: [u32; 8],
            pub value_uV: u32,
            pub unknown: [u32; 8],
        }
    }

    nvversion! { NV_GPU_CLIENT_VOLT_RAILS_STATUS_VER_1(NV_GPU_CLIENT_VOLT_RAILS_STATUS_V1 = 76, 1) }
    nvversion! { NV_GPU_CLIENT_VOLT_RAILS_STATUS_VER = NV_GPU_CLIENT_VOLT_RAILS_STATUS_VER_1 }

    pub type NV_GPU_CLIENT_VOLT_RAILS_STATUS = NV_GPU_CLIENT_VOLT_RAILS_STATUS_V1;

    nvapi! {
        /// Pascal and later
        pub unsafe fn NvAPI_GPU_ClientVoltRailsGetStatus(hPhysicalGPU: NvPhysicalGpuHandle, pVoltageStatus: *mut NV_GPU_CLIENT_VOLT_RAILS_STATUS) -> NvAPI_Status;
    }

    nvstruct! {
        pub struct NV_GPU_CLIENT_VOLT_RAILS_CONTROL_V1 {
            pub version: NvVersion,
            /// uiDelta
            pub percent: u32, // apparently actually i32?
            pub unknown: [u32; 8],
        }
    }

    nvversion! { NV_GPU_CLIENT_VOLT_RAILS_CONTROL_VER_1(NV_GPU_CLIENT_VOLT_RAILS_CONTROL_V1 = 4 * (2 + 8), 1) }
    nvversion! { NV_GPU_CLIENT_VOLT_RAILS_CONTROL_VER = NV_GPU_CLIENT_VOLT_RAILS_CONTROL_VER_1 }

    pub type NV_GPU_CLIENT_VOLT_RAILS_CONTROL = NV_GPU_CLIENT_VOLT_RAILS_CONTROL_V1;

    nvapi! {
        /// Pascal and later
        pub unsafe fn NvAPI_GPU_ClientVoltRailsGetControl(hPhysicalGPU: NvPhysicalGpuHandle, pVoltboostPercent: *mut NV_GPU_CLIENT_VOLT_RAILS_CONTROL) -> NvAPI_Status;
    }

    nvapi! {
        /// Pascal and later
        pub unsafe fn NvAPI_GPU_ClientVoltRailsSetControl(hPhysicalGPU: NvPhysicalGpuHandle, pVoltboostPercent: *const NV_GPU_CLIENT_VOLT_RAILS_CONTROL) -> NvAPI_Status;
    }

    nvstruct! {
        pub struct NV_GPU_CLOCK_CLIENT_CLK_VF_POINT {
            pub freq_kHz: u32,
            pub voltage_uV: u32,
        }
    }

    nvstruct! {
        pub struct NV_GPU_CLOCK_CLIENT_CLK_VF_POINT_STATUS_V1 {
            pub clock_type: u32, // 0, 1 for idle mem values?
            pub point: NV_GPU_CLOCK_CLIENT_CLK_VF_POINT,
            pub unknown: Padding<[u32; 4]>,
        }
    }

    nvstruct! {
        pub struct NV_GPU_CLOCK_CLIENT_CLK_VF_POINT_STATUS_V3 {
            pub clock_type: u32, // 0, 1?
            pub point: NV_GPU_CLOCK_CLIENT_CLK_VF_POINT,
            pub point_default: NV_GPU_CLOCK_CLIENT_CLK_VF_POINT,
            pub unknown0: Padding<[u32; 8]>,
            /// overclockedFrequencyKhz and millivoltage
            pub point_overclocked: NV_GPU_CLOCK_CLIENT_CLK_VF_POINT,
            pub unknown: Padding<[u32; 348/4 - (7 + 8)]>,
        }
    }

    nvstruct! {
        pub struct NV_GPU_CLOCK_CLIENT_CLK_VF_POINTS_STATUS_V1 {
            pub version: NvVersion,
            pub mask: ClockMask,
            pub unknown: Padding<[u32; 8]>,
            pub entries: [NV_GPU_CLOCK_CLIENT_CLK_VF_POINT_STATUS_V1; 255],
        }
    }

    nvstruct! {
        pub struct NV_GPU_CLOCK_CLIENT_CLK_VF_POINTS_STATUS_V3 {
            pub version: NvVersion,
            pub mask: ClockMask,
            pub unknown: Padding<[u8; 0x44]>,
            pub entries: [NV_GPU_CLOCK_CLIENT_CLK_VF_POINT_STATUS_V3; 255],
        }
    }

    nvversion! { NV_GPU_CLOCK_CLIENT_CLK_VF_POINTS_STATUS_VER_1(NV_GPU_CLOCK_CLIENT_CLK_VF_POINTS_STATUS_V1 = 0x1c28, 1) }
    nvversion! { NV_GPU_CLOCK_CLIENT_CLK_VF_POINTS_STATUS_VER_2(NV_GPU_CLOCK_CLIENT_CLK_VF_POINTS_STATUS_V2 = 0x1c28, 2) }
    nvversion! { NV_GPU_CLOCK_CLIENT_CLK_VF_POINTS_STATUS_VER_3(NV_GPU_CLOCK_CLIENT_CLK_VF_POINTS_STATUS_V3 = 0x15b0c, 3) } // TODO: 0x5b0c or 0x15b0c?
    nvversion! { NV_GPU_CLOCK_CLIENT_CLK_VF_POINTS_STATUS_VER = NV_GPU_CLOCK_CLIENT_CLK_VF_POINTS_STATUS_VER_3 }

    pub type NV_GPU_CLOCK_CLIENT_CLK_VF_POINTS_STATUS_V2 = NV_GPU_CLOCK_CLIENT_CLK_VF_POINTS_STATUS_V1;
    pub type NV_GPU_CLOCK_CLIENT_CLK_VF_POINTS_STATUS = NV_GPU_CLOCK_CLIENT_CLK_VF_POINTS_STATUS_V3;

    nvapi! {
        /// Pascal and later
        pub unsafe fn NvAPI_GPU_ClockClientClkVfPointsGetStatus(hPhysicalGPU: NvPhysicalGpuHandle, pVfpCurve: *mut NV_GPU_CLOCK_CLIENT_CLK_VF_POINTS_STATUS) -> NvAPI_Status;
    }

    nvenum! {
        pub enum NV_GPU_CLIENT_POWER_POLICIES_POLICY_ID / PowerPolicyId {
            NV_GPU_CLIENT_POWER_POLICIES_POLICY_ID_DEFAULT / Default = 0,
        }
    }

    nvenum_display! {
        PowerPolicyId => {
            Default = "Board Power Limit",
        }
    }

    nvstruct! {
        pub struct NV_GPU_CLIENT_POWER_POLICIES_INFO_ENTRY_V1 {
            pub policy_id: NV_GPU_CLIENT_POWER_POLICIES_POLICY_ID,
            pub b: u32,
            pub c: u32,
            pub min_power: u32,
            pub e: u32,
            pub f: u32,
            pub def_power: u32,
            pub h: u32,
            pub i: u32,
            pub max_power: u32,
            pub k: u32, // 0
        }
    }

    nvstruct! {
        pub struct NV_GPU_CLIENT_POWER_POLICIES_INFO_V1 {
            pub version: NvVersion,
            pub valid: u8,
            pub count: u8,
            pub padding: [u8; 2],
            pub entries: [NV_GPU_CLIENT_POWER_POLICIES_INFO_ENTRY_V1; 4],
        }
    }

    nvstruct! {
        pub struct NV_GPU_CLIENT_POWER_POLICIES_INFO_ENTRY_V2 {
            pub policy_id: NV_GPU_CLIENT_POWER_POLICIES_POLICY_ID,
            pub unknown0: Padding<[u32; 3]>,
            pub min_power: u32,
            pub unknown1: Padding<[u32; 2]>,
            pub def_power: u32,
            pub unknown2: Padding<[u32; 2]>,
            pub max_power: u32,
            pub padding: Padding<[u32; 560/4 - 11]>,
        }
    }

    nvstruct! {
        pub struct NV_GPU_CLIENT_POWER_POLICIES_INFO_V2 {
            pub version: NvVersion,
            pub valid: u8,
            pub count: u8,
            pub padding: [u8; 2],
            pub entries: [NV_GPU_CLIENT_POWER_POLICIES_INFO_ENTRY_V2; 4],
        }
    }

    impl NV_GPU_CLIENT_POWER_POLICIES_INFO_V2 {
        pub fn entries(&self) -> &[NV_GPU_CLIENT_POWER_POLICIES_INFO_ENTRY_V2] {
            &self.entries[..self.count as usize]
        }
    }

    pub type NV_GPU_CLIENT_POWER_POLICIES_INFO = NV_GPU_CLIENT_POWER_POLICIES_INFO_V2;

    nvversion! { NV_GPU_CLIENT_POWER_POLICIES_INFO_VER_1(NV_GPU_CLIENT_POWER_POLICIES_INFO_V1 = 4 * 2 + 4 * (4 * 11), 1) }
    nvversion! { NV_GPU_CLIENT_POWER_POLICIES_INFO_VER_2(NV_GPU_CLIENT_POWER_POLICIES_INFO_V2 = 2248, 2) }
    nvversion! { NV_GPU_CLIENT_POWER_POLICIES_INFO_VER = NV_GPU_CLIENT_POWER_POLICIES_INFO_VER_2 }

    nvapi! {
        pub unsafe fn NvAPI_GPU_ClientPowerPoliciesGetInfo(hPhysicalGPU: NvPhysicalGpuHandle, pPowerInfo: *mut NV_GPU_CLIENT_POWER_POLICIES_INFO) -> NvAPI_Status;
    }

    nvstruct! {
        pub struct NV_GPU_CLIENT_POWER_POLICIES_STATUS_ENTRY_V1 {
            pub policy_id: NV_GPU_CLIENT_POWER_POLICIES_POLICY_ID,
            pub b: u32,
            pub power_target: u32,
            pub d: u32,
        }
    }

    nvstruct! {
        pub struct NV_GPU_CLIENT_POWER_POLICIES_STATUS_V1 {
            pub version: NvVersion,
            pub count: u32,
            pub entries: [NV_GPU_CLIENT_POWER_POLICIES_STATUS_ENTRY_V1; 4],
        }
    }

    nvstruct! {
        pub struct NV_GPU_CLIENT_POWER_POLICIES_STATUS_ENTRY_V2 {
            pub policy_id: NV_GPU_CLIENT_POWER_POLICIES_POLICY_ID,
            pub unknown: Padding<[u32; 1]>,
            pub flags: u32,
            pub power_target: u32,
            pub padding: Padding<[u32; 340/4 - 4]>,
        }
    }

    impl NV_GPU_CLIENT_POWER_POLICIES_STATUS_ENTRY_V2 {
        /// Unsure what this is but flag should be cleared for SetStatus, maybe?
        pub fn set_flag(&mut self, value: bool) {
            self.flags = self.flags & 0xfffffffe | if value { 1 } else { 0 }
        }
    }

    nvstruct! {
        pub struct NV_GPU_CLIENT_POWER_POLICIES_STATUS_V2 {
            pub version: NvVersion,
            pub count: u32,
            pub entries: [NV_GPU_CLIENT_POWER_POLICIES_STATUS_ENTRY_V2; 4],
        }
    }

    pub type NV_GPU_CLIENT_POWER_POLICIES_STATUS = NV_GPU_CLIENT_POWER_POLICIES_STATUS_V2;

    nvversion! { NV_GPU_CLIENT_POWER_POLICIES_STATUS_VER_1(NV_GPU_CLIENT_POWER_POLICIES_STATUS_V1 = 4 * 2 + 4 * (4 * 4), 1) }
    nvversion! { NV_GPU_CLIENT_POWER_POLICIES_STATUS_VER_2(NV_GPU_CLIENT_POWER_POLICIES_STATUS_V2 = 1368, 2) }
    nvversion! { NV_GPU_CLIENT_POWER_POLICIES_STATUS_VER = NV_GPU_CLIENT_POWER_POLICIES_STATUS_VER_2 }

    nvapi! {
        pub unsafe fn NvAPI_GPU_ClientPowerPoliciesGetStatus(hPhysicalGPU: NvPhysicalGpuHandle, pPowerStatus: *mut NV_GPU_CLIENT_POWER_POLICIES_STATUS) -> NvAPI_Status;
    }

    nvapi! {
        pub unsafe fn NvAPI_GPU_ClientPowerPoliciesSetStatus(hPhysicalGPU: NvPhysicalGpuHandle, pPowerStatus: *const NV_GPU_CLIENT_POWER_POLICIES_STATUS) -> NvAPI_Status;
    }

    nvstruct! {
        pub struct NV_GPU_CLIENT_POWER_TOPOLOGY_INFO_V1 {
            pub version: NvVersion,
            pub valid: u8,
            pub count: u8,
            pub padding: Padding<[u8; 2]>,
            pub channels: [NV_GPU_CLIENT_POWER_TOPOLOGY_CHANNEL_ID; 4],
        }
    }

    impl NV_GPU_CLIENT_POWER_TOPOLOGY_INFO_V1 {
        pub fn channels(&self) -> &[NV_GPU_CLIENT_POWER_TOPOLOGY_CHANNEL_ID] {
            &self.channels[..self.count as usize]
        }
    }

    pub type NV_GPU_CLIENT_POWER_TOPOLOGY_INFO = NV_GPU_CLIENT_POWER_TOPOLOGY_INFO_V1;

    nvversion! { NV_GPU_CLIENT_POWER_TOPOLOGY_INFO_VER_1(NV_GPU_CLIENT_POWER_TOPOLOGY_INFO_V1 = 24, 1) }
    nvversion! { NV_GPU_CLIENT_POWER_TOPOLOGY_INFO_VER = NV_GPU_CLIENT_POWER_TOPOLOGY_INFO_VER_1 }

    nvapi! {
        pub unsafe fn NvAPI_GPU_ClientPowerTopologyGetInfo(hPhysicalGPU: NvPhysicalGpuHandle, pPowerTopo: *mut NV_GPU_CLIENT_POWER_TOPOLOGY_INFO) -> NvAPI_Status;
    }

    nvenum! {
        pub enum NV_GPU_CLIENT_POWER_TOPOLOGY_CHANNEL_ID / PowerTopologyChannelId {
            NV_GPU_CLIENT_POWER_TOPOLOGY_CHANNEL_ID_TOTAL_GPU_POWER / TotalGpuPower = 0,
            NV_GPU_CLIENT_POWER_TOPOLOGY_CHANNEL_ID_NORMALIZED_TOTAL_POWER / NormalizedTotalPower = 1,
        }
    }

    nvenum_display! {
        PowerTopologyChannelId => {
            TotalGpuPower = "Total Power",
            NormalizedTotalPower = "Normalized Power",
        }
    }

    nvstruct! {
        pub struct NV_GPU_CLIENT_POWER_TOPOLOGY_STATUS_ENTRY {
            pub channel: NV_GPU_CLIENT_POWER_TOPOLOGY_CHANNEL_ID,
            pub unknown0: u32,
            pub power: u32,
            pub unknown1: u32,
        }
    }

    nvstruct! {
        pub struct NV_GPU_CLIENT_POWER_TOPOLOGY_STATUS_V1 {
            pub version: NvVersion,
            pub count: u32,
            pub entries: [NV_GPU_CLIENT_POWER_TOPOLOGY_STATUS_ENTRY; 4],
        }
    }

    impl NV_GPU_CLIENT_POWER_TOPOLOGY_STATUS_V1 {
        pub fn entries(&self) -> &[NV_GPU_CLIENT_POWER_TOPOLOGY_STATUS_ENTRY] {
            &self.entries[..self.count as usize]
        }
    }

    pub type NV_GPU_CLIENT_POWER_TOPOLOGY_STATUS = NV_GPU_CLIENT_POWER_TOPOLOGY_STATUS_V1;

    nvversion! { NV_GPU_CLIENT_POWER_TOPOLOGY_STATUS_VER_1(NV_GPU_CLIENT_POWER_TOPOLOGY_STATUS_V1 = 72, 1) }
    nvversion! { NV_GPU_CLIENT_POWER_TOPOLOGY_STATUS_VER = NV_GPU_CLIENT_POWER_TOPOLOGY_STATUS_VER_1 }

    nvapi! {
        pub unsafe fn NvAPI_GPU_ClientPowerTopologyGetStatus(hPhysicalGPU: NvPhysicalGpuHandle, pPowerTopo: *mut NV_GPU_CLIENT_POWER_TOPOLOGY_STATUS) -> NvAPI_Status;
    }

    nvbits! {
        pub enum NV_GPU_PERF_FLAGS / PerfFlags {
            NV_GPU_PERF_FLAGS_POWER_LIMIT / POWER_LIMIT = 1,
            NV_GPU_PERF_FLAGS_THERMAL_LIMIT / THERMAL_LIMIT = 2,
            /// Reliability voltage
            NV_GPU_PERF_FLAGS_VOLTAGE_REL_LIMIT / VOLTAGE_REL_LIMIT = 4,
            /// Operating voltage
            NV_GPU_PERF_FLAGS_VOLTAGE_OP_LIMIT / VOLTAGE_OP_LIMIT = 8,
            /// GPU utilization
            NV_GPU_PERF_FLAGS_NO_LOAD_LIMIT / NO_LOAD_LIMIT = 16,
            /// Never seen this
            NV_GPU_PERF_FLAGS_UNKNOWN_32 / UNKNOWN_32 = 32,
        }
    }

    nvenum_display! {
        PerfFlags => {
            POWER_LIMIT = "Power",
            THERMAL_LIMIT = "Temperature",
            VOLTAGE_REL_LIMIT = "Reliability Voltage",
            VOLTAGE_OP_LIMIT = "Operating Voltage",
            NO_LOAD_LIMIT = "No Load",
            UNKNOWN_32 = "Unknown32",
            _ = _,
        }
    }

    nvstruct! {
        pub struct NV_GPU_PERF_POLICIES_INFO_PARAMS_V1 {
            pub version: NvVersion,
            pub maxUnknown: u32,
            pub limitSupport: NV_GPU_PERF_FLAGS,
            pub padding: [u32; 16],
        }
    }

    pub type NV_GPU_PERF_POLICIES_INFO_PARAMS = NV_GPU_PERF_POLICIES_INFO_PARAMS_V1;

    nvversion! { NV_GPU_PERF_POLICIES_INFO_PARAMS_VER_1(NV_GPU_PERF_POLICIES_INFO_PARAMS_V1 = 76, 1) }
    nvversion! { NV_GPU_PERF_POLICIES_INFO_PARAMS_VER = NV_GPU_PERF_POLICIES_INFO_PARAMS_VER_1 }

    nvapi! {
        pub unsafe fn NvAPI_GPU_PerfPoliciesGetInfo(hPhysicalGPU: NvPhysicalGpuHandle, pPerfInfo: *mut NV_GPU_PERF_POLICIES_INFO_PARAMS) -> NvAPI_Status;
    }

    nvstruct! {
        pub struct NV_GPU_PERF_POLICIES_STATUS_PARAMS_V1 {
            pub version: NvVersion,
            pub flags: u32,
            /// nanoseconds
            pub timer: u64,
            /// - 1 = power limit
            /// - 2 = temp limit
            /// - 4 = voltage limit
            /// - 8 = only got with 15 in driver crash
            /// - 16 = no-load limit
            pub limits: NV_GPU_PERF_FLAGS,
            pub zero0: u32,
            /// - 1 on load
            /// - 3 in low clocks
            /// - 7 in idle
            pub unknown: u32,
            pub zero1: u32,
            /// nanoseconds
            pub timers: [u64; 3],
            pub padding: [u32; 326],
        }
    }

    pub type NV_GPU_PERF_POLICIES_STATUS_PARAMS = NV_GPU_PERF_POLICIES_STATUS_PARAMS_V1;

    nvversion! { NV_GPU_PERF_POLICIES_STATUS_PARAMS_VER_1(NV_GPU_PERF_POLICIES_STATUS_PARAMS_V1 = 0x550, 1) }
    nvversion! { NV_GPU_PERF_POLICIES_STATUS_PARAMS_VER = NV_GPU_PERF_POLICIES_STATUS_PARAMS_VER_1 }

    nvapi! {
        pub unsafe fn NvAPI_GPU_PerfPoliciesGetStatus(hPhysicalGPU: NvPhysicalGpuHandle, pPerfStatus: *mut NV_GPU_PERF_POLICIES_STATUS_PARAMS) -> NvAPI_Status;
    }

    nvstruct! {
        pub struct NV_VOLT_STATUS_V1 {
            pub version: NvVersion,
            pub flags: u32,
            /// unsure
            pub count: u32,
            pub unknown: u32,
            pub value_uV: u32,
            pub buf1: [u32; 30],
        }
    }

    pub type NV_VOLT_STATUS = NV_VOLT_STATUS_V1;

    nvversion! { NV_VOLT_STATUS_VER_1(NV_VOLT_STATUS_V1 = 140, 1) }
    nvversion! { NV_VOLT_STATUS_VER = NV_VOLT_STATUS_VER_1 }

    nvapi! {
        /// Maxwell only
        pub unsafe fn NvAPI_GPU_GetVoltageDomainsStatus(hPhysicalGPU: NvPhysicalGpuHandle, pVoltStatus: *mut NV_VOLT_STATUS) -> NvAPI_Status;
    }

    nvapi! {
        /// Maxwell only
        pub unsafe fn NvAPI_GPU_GetVoltageStep(hPhysicalGPU: NvPhysicalGpuHandle, pVoltStep: *mut NV_VOLT_STATUS) -> NvAPI_Status;
    }

    nvstruct! {
        pub struct NV_VOLT_TABLE_ENTRY {
            pub voltage_uV: u32,
            pub unknown: u32,
        }
    }

    nvstruct! {
        pub struct NV_VOLT_TABLE_V1 {
            pub version: NvVersion,
            pub flags: u32,
            /// 1
            pub filled: u32,
            pub entries: [NV_VOLT_TABLE_ENTRY; 128],
            /// empty tables?
            pub buf1: [u32; 3888],
        }
    }

    pub type NV_VOLT_TABLE = NV_VOLT_TABLE_V1;

    nvversion! { NV_VOLT_TABLE_VER_1(NV_VOLT_TABLE_V1 = 0x40cc, 1) }
    nvversion! { NV_VOLT_TABLE_VER = NV_VOLT_TABLE_VER_1 }

    nvapi! {
        /// Maxwell only
        pub unsafe fn NvAPI_GPU_GetVoltages(hPhysicalGPU: NvPhysicalGpuHandle, pVolts: *mut NV_VOLT_TABLE) -> NvAPI_Status;
    }
}
