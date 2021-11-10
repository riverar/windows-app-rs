#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[doc = "*Required features: `Windows_System_Power`*"]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct BatteryStatus(pub i32);
impl BatteryStatus {
    pub const NotPresent: BatteryStatus = BatteryStatus(0i32);
    pub const Discharging: BatteryStatus = BatteryStatus(1i32);
    pub const Idle: BatteryStatus = BatteryStatus(2i32);
    pub const Charging: BatteryStatus = BatteryStatus(3i32);
}
impl ::std::convert::From<i32> for BatteryStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for BatteryStatus {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for BatteryStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Microsoft.Windows.System.Power.BatteryStatus;i4)",
    );
}
impl ::windows::runtime::DefaultType for BatteryStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `Windows_System_Power`*"]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DisplayStatus(pub i32);
impl DisplayStatus {
    pub const Off: DisplayStatus = DisplayStatus(0i32);
    pub const On: DisplayStatus = DisplayStatus(1i32);
    pub const Dimmed: DisplayStatus = DisplayStatus(2i32);
}
impl ::std::convert::From<i32> for DisplayStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DisplayStatus {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for DisplayStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Microsoft.Windows.System.Power.DisplayStatus;i4)",
    );
}
impl ::windows::runtime::DefaultType for DisplayStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `Windows_System_Power`*"]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct EffectivePowerMode(pub i32);
impl EffectivePowerMode {
    pub const BatterySaver: EffectivePowerMode = EffectivePowerMode(0i32);
    pub const BetterBattery: EffectivePowerMode = EffectivePowerMode(1i32);
    pub const Balanced: EffectivePowerMode = EffectivePowerMode(2i32);
    pub const HighPerformance: EffectivePowerMode = EffectivePowerMode(3i32);
    pub const MaxPerformance: EffectivePowerMode = EffectivePowerMode(4i32);
    pub const GameMode: EffectivePowerMode = EffectivePowerMode(5i32);
    pub const MixedReality: EffectivePowerMode = EffectivePowerMode(6i32);
}
impl ::std::convert::From<i32> for EffectivePowerMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for EffectivePowerMode {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for EffectivePowerMode {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Microsoft.Windows.System.Power.EffectivePowerMode;i4)",
    );
}
impl ::windows::runtime::DefaultType for EffectivePowerMode {
    type DefaultType = Self;
}
#[doc = "*Required features: `Windows_System_Power`*"]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct EnergySaverStatus(pub i32);
impl EnergySaverStatus {
    pub const Uninitialized: EnergySaverStatus = EnergySaverStatus(0i32);
    pub const Disabled: EnergySaverStatus = EnergySaverStatus(1i32);
    pub const Off: EnergySaverStatus = EnergySaverStatus(2i32);
    pub const On: EnergySaverStatus = EnergySaverStatus(3i32);
}
impl ::std::convert::From<i32> for EnergySaverStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for EnergySaverStatus {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for EnergySaverStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Microsoft.Windows.System.Power.EnergySaverStatus;i4)",
    );
}
impl ::windows::runtime::DefaultType for EnergySaverStatus {
    type DefaultType = Self;
}
#[repr(transparent)]
#[doc(hidden)]
pub struct IPowerManagerStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPowerManagerStatics {
    type Vtable = IPowerManagerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4197799116,
        48668,
        21324,
        [191, 248, 114, 223, 120, 233, 244, 164],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IPowerManagerStatics_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut EnergySaverStatus,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut BatteryStatus,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut PowerSupplyStatus,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::Foundation::TimeSpan,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut PowerSourceKind,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut DisplayStatus,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut UserPresenceStatus,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut SystemSuspendStatus,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Windows_System_Power`*"]
pub struct PowerManager {}
impl PowerManager {
    #[doc = "*Required features: `Windows_System_Power`*"]
    pub fn EnergySaverStatus() -> ::windows::runtime::Result<EnergySaverStatus> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__: EnergySaverStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<EnergySaverStatus>(result__)
        })
    }
    #[doc = "*Required features: `Windows_System_Power`*"]
    pub fn EnergySaverStatusChanged<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            ::windows::Foundation::EventHandler<::windows::runtime::IInspectable>,
        >,
    >(
        handler: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::EventRegistrationToken> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `Windows_System_Power`*"]
    pub fn RemoveEnergySaverStatusChanged<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        Self::IPowerManagerStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        })
    }
    #[doc = "*Required features: `Windows_System_Power`*"]
    pub fn BatteryStatus() -> ::windows::runtime::Result<BatteryStatus> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__: BatteryStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<BatteryStatus>(result__)
        })
    }
    #[doc = "*Required features: `Windows_System_Power`*"]
    pub fn BatteryStatusChanged<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            ::windows::Foundation::EventHandler<::windows::runtime::IInspectable>,
        >,
    >(
        handler: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::EventRegistrationToken> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `Windows_System_Power`*"]
    pub fn RemoveBatteryStatusChanged<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        Self::IPowerManagerStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        })
    }
    #[doc = "*Required features: `Windows_System_Power`*"]
    pub fn PowerSupplyStatus() -> ::windows::runtime::Result<PowerSupplyStatus> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__: PowerSupplyStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<PowerSupplyStatus>(result__)
        })
    }
    #[doc = "*Required features: `Windows_System_Power`*"]
    pub fn PowerSupplyStatusChanged<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            ::windows::Foundation::EventHandler<::windows::runtime::IInspectable>,
        >,
    >(
        handler: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::EventRegistrationToken> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `Windows_System_Power`*"]
    pub fn RemovePowerSupplyStatusChanged<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        Self::IPowerManagerStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        })
    }
    #[doc = "*Required features: `Windows_System_Power`*"]
    pub fn RemainingChargePercent() -> ::windows::runtime::Result<i32> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        })
    }
    #[doc = "*Required features: `Windows_System_Power`*"]
    pub fn RemainingChargePercentChanged<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            ::windows::Foundation::EventHandler<::windows::runtime::IInspectable>,
        >,
    >(
        handler: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::EventRegistrationToken> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `Windows_System_Power`*"]
    pub fn RemoveRemainingChargePercentChanged<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        Self::IPowerManagerStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).17)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        })
    }
    #[doc = "*Required features: `Windows_System_Power`*"]
    pub fn RemainingDischargeTime() -> ::windows::runtime::Result<::windows::Foundation::TimeSpan> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__: ::windows::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::TimeSpan>(result__)
        })
    }
    #[doc = "*Required features: `Windows_System_Power`*"]
    pub fn RemainingDischargeTimeChanged<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            ::windows::Foundation::EventHandler<::windows::runtime::IInspectable>,
        >,
    >(
        handler: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::EventRegistrationToken> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `Windows_System_Power`*"]
    pub fn RemoveRemainingDischargeTimeChanged<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        Self::IPowerManagerStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).20)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        })
    }
    #[doc = "*Required features: `Windows_System_Power`*"]
    pub fn PowerSourceKind() -> ::windows::runtime::Result<PowerSourceKind> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__: PowerSourceKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<PowerSourceKind>(result__)
        })
    }
    #[doc = "*Required features: `Windows_System_Power`*"]
    pub fn PowerSourceKindChanged<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            ::windows::Foundation::EventHandler<::windows::runtime::IInspectable>,
        >,
    >(
        handler: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::EventRegistrationToken> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `Windows_System_Power`*"]
    pub fn RemovePowerSourceKindChanged<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        Self::IPowerManagerStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).23)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        })
    }
    #[doc = "*Required features: `Windows_System_Power`*"]
    pub fn DisplayStatus() -> ::windows::runtime::Result<DisplayStatus> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__: DisplayStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<DisplayStatus>(result__)
        })
    }
    #[doc = "*Required features: `Windows_System_Power`*"]
    pub fn DisplayStatusChanged<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            ::windows::Foundation::EventHandler<::windows::runtime::IInspectable>,
        >,
    >(
        handler: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::EventRegistrationToken> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).25)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `Windows_System_Power`*"]
    pub fn RemoveDisplayStatusChanged<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        Self::IPowerManagerStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).26)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        })
    }
    #[doc = "*Required features: `Windows_System_Power`*"]
    pub fn SystemIdleStatusChanged<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            ::windows::Foundation::EventHandler<::windows::runtime::IInspectable>,
        >,
    >(
        handler: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::EventRegistrationToken> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).27)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `Windows_System_Power`*"]
    pub fn RemoveSystemIdleStatusChanged<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        Self::IPowerManagerStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).28)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        })
    }
    #[doc = "*Required features: `Windows_System_Power`*"]
    pub fn EffectivePowerMode(
    ) -> ::windows::runtime::Result<::windows::Foundation::IAsyncOperation<EffectivePowerMode>>
    {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).29)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::IAsyncOperation<EffectivePowerMode>>(result__)
        })
    }
    #[doc = "*Required features: `Windows_System_Power`*"]
    pub fn EffectivePowerModeChanged<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            ::windows::Foundation::EventHandler<::windows::runtime::IInspectable>,
        >,
    >(
        handler: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::EventRegistrationToken> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).30)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `Windows_System_Power`*"]
    pub fn RemoveEffectivePowerModeChanged<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        Self::IPowerManagerStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).31)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        })
    }
    #[doc = "*Required features: `Windows_System_Power`*"]
    pub fn UserPresenceStatus() -> ::windows::runtime::Result<UserPresenceStatus> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__: UserPresenceStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).32)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<UserPresenceStatus>(result__)
        })
    }
    #[doc = "*Required features: `Windows_System_Power`*"]
    pub fn UserPresenceStatusChanged<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            ::windows::Foundation::EventHandler<::windows::runtime::IInspectable>,
        >,
    >(
        handler: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::EventRegistrationToken> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).33)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `Windows_System_Power`*"]
    pub fn RemoveUserPresenceStatusChanged<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        Self::IPowerManagerStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).34)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        })
    }
    #[doc = "*Required features: `Windows_System_Power`*"]
    pub fn SystemSuspendStatus() -> ::windows::runtime::Result<SystemSuspendStatus> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__: SystemSuspendStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).35)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<SystemSuspendStatus>(result__)
        })
    }
    #[doc = "*Required features: `Windows_System_Power`*"]
    pub fn SystemSuspendStatusChanged<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            ::windows::Foundation::EventHandler<::windows::runtime::IInspectable>,
        >,
    >(
        handler: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::EventRegistrationToken> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).36)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `Windows_System_Power`*"]
    pub fn RemoveSystemSuspendStatusChanged<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        Self::IPowerManagerStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).37)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        })
    }
    pub fn IPowerManagerStatics<
        R,
        F: FnOnce(&IPowerManagerStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PowerManager, IPowerManagerStatics> =
            ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for PowerManager {
    const NAME: &'static str = "Microsoft.Windows.System.Power.PowerManager";
}
#[doc = "*Required features: `Windows_System_Power`*"]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct PowerSourceKind(pub i32);
impl PowerSourceKind {
    pub const AC: PowerSourceKind = PowerSourceKind(0i32);
    pub const DC: PowerSourceKind = PowerSourceKind(1i32);
}
impl ::std::convert::From<i32> for PowerSourceKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PowerSourceKind {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PowerSourceKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Microsoft.Windows.System.Power.PowerSourceKind;i4)",
    );
}
impl ::windows::runtime::DefaultType for PowerSourceKind {
    type DefaultType = Self;
}
#[doc = "*Required features: `Windows_System_Power`*"]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct PowerSupplyStatus(pub i32);
impl PowerSupplyStatus {
    pub const NotPresent: PowerSupplyStatus = PowerSupplyStatus(0i32);
    pub const Inadequate: PowerSupplyStatus = PowerSupplyStatus(1i32);
    pub const Adequate: PowerSupplyStatus = PowerSupplyStatus(2i32);
}
impl ::std::convert::From<i32> for PowerSupplyStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PowerSupplyStatus {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PowerSupplyStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Microsoft.Windows.System.Power.PowerSupplyStatus;i4)",
    );
}
impl ::windows::runtime::DefaultType for PowerSupplyStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `Windows_System_Power`*"]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct SystemSuspendStatus(pub i32);
impl SystemSuspendStatus {
    pub const Uninitialized: SystemSuspendStatus = SystemSuspendStatus(0i32);
    pub const Entering: SystemSuspendStatus = SystemSuspendStatus(1i32);
    pub const AutoResume: SystemSuspendStatus = SystemSuspendStatus(2i32);
    pub const ManualResume: SystemSuspendStatus = SystemSuspendStatus(3i32);
}
impl ::std::convert::From<i32> for SystemSuspendStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SystemSuspendStatus {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SystemSuspendStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Microsoft.Windows.System.Power.SystemSuspendStatus;i4)",
    );
}
impl ::windows::runtime::DefaultType for SystemSuspendStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `Windows_System_Power`*"]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct UserPresenceStatus(pub i32);
impl UserPresenceStatus {
    pub const Present: UserPresenceStatus = UserPresenceStatus(0i32);
    pub const Absent: UserPresenceStatus = UserPresenceStatus(1i32);
}
impl ::std::convert::From<i32> for UserPresenceStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for UserPresenceStatus {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for UserPresenceStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Microsoft.Windows.System.Power.UserPresenceStatus;i4)",
    );
}
impl ::windows::runtime::DefaultType for UserPresenceStatus {
    type DefaultType = Self;
}
