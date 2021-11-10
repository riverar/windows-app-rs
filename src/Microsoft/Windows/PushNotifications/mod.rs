#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[repr(transparent)]
#[doc(hidden)]
pub struct IPushNotificationActivationInfo(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPushNotificationActivationInfo {
    type Vtable = IPushNotificationActivationInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4111143462,
        36530,
        22457,
        [137, 31, 156, 235, 57, 72, 197, 195],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IPushNotificationActivationInfo_abi(
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
        result__: *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut PushNotificationRegistrationActivators,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result_size__: *mut u32,
        result__: *mut *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        conditions_array_size: u32,
        conditions: *const ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPushNotificationActivationInfoFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPushNotificationActivationInfoFactory {
    type Vtable = IPushNotificationActivationInfoFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        893070399,
        48228,
        23442,
        [190, 29, 167, 106, 44, 222, 43, 248],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IPushNotificationActivationInfoFactory_abi(
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
        activators: PushNotificationRegistrationActivators,
        taskclsid: ::windows::runtime::GUID,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        activators: PushNotificationRegistrationActivators,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPushNotificationChannel(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPushNotificationChannel {
    type Vtable = IPushNotificationChannel_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3444503200,
        14725,
        22600,
        [132, 38, 129, 61, 230, 54, 248, 37],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IPushNotificationChannel_abi(
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
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::Foundation::DateTime,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
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
#[repr(transparent)]
#[doc(hidden)]
pub struct IPushNotificationChannelFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPushNotificationChannelFactory {
    type Vtable = IPushNotificationChannelFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4033202924,
        16337,
        20679,
        [148, 52, 14, 41, 255, 231, 186, 94],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IPushNotificationChannelFactory_abi(
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
        channel: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPushNotificationCreateChannelResult(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPushNotificationCreateChannelResult {
    type Vtable = IPushNotificationCreateChannelResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1307799935,
        23859,
        22249,
        [179, 129, 27, 53, 12, 149, 114, 46],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IPushNotificationCreateChannelResult_abi(
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
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::HRESULT,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut PushNotificationChannelStatus,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPushNotificationCreateChannelResultFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPushNotificationCreateChannelResultFactory {
    type Vtable = IPushNotificationCreateChannelResultFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1633855816,
        3546,
        21422,
        [171, 205, 91, 71, 243, 54, 214, 18],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IPushNotificationCreateChannelResultFactory_abi(
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
        channel: ::windows::runtime::RawPtr,
        extendederror: ::windows::runtime::HRESULT,
        status: PushNotificationChannelStatus,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPushNotificationManagerStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPushNotificationManagerStatics {
    type Vtable = IPushNotificationManagerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        558635444,
        12723,
        24082,
        [158, 254, 190, 188, 2, 121, 191, 140],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IPushNotificationManagerStatics_abi(
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
        details: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        activators: PushNotificationRegistrationActivators,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        remoteid: ::windows::runtime::GUID,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        activators: PushNotificationRegistrationActivators,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPushNotificationReceivedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPushNotificationReceivedEventArgs {
    type Vtable = IPushNotificationReceivedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2835251644,
        53977,
        21199,
        [171, 27, 111, 222, 113, 24, 98, 234],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IPushNotificationReceivedEventArgs_abi(
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
        result_size__: *mut u32,
        result__: *mut *mut u8,
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
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Windows_PushNotifications`*"]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct PushNotificationActivationInfo(pub ::windows::runtime::IInspectable);
impl PushNotificationActivationInfo {
    #[doc = "*Required features: `Windows_PushNotifications`*"]
    pub fn TaskClsid(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    #[doc = "*Required features: `Windows_PushNotifications`*"]
    pub fn Activators(&self) -> ::windows::runtime::Result<PushNotificationRegistrationActivators> {
        let this = self;
        unsafe {
            let mut result__: PushNotificationRegistrationActivators = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<PushNotificationRegistrationActivators>(result__)
        }
    }
    #[doc = "*Required features: `Windows_PushNotifications`*"]
    pub fn GetConditions(
        &self,
    ) -> ::windows::runtime::Result<
        ::windows::runtime::Array<::windows::ApplicationModel::Background::IBackgroundCondition>,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::Array<
                ::windows::ApplicationModel::Background::IBackgroundCondition,
            > = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                ::windows::runtime::Array::<
                    ::windows::ApplicationModel::Background::IBackgroundCondition,
                >::set_abi_len(&mut result__),
                &mut result__ as *mut _ as _,
            )
            .and_then(|| result__)
        }
    }
    #[doc = "*Required features: `Windows_PushNotifications`*"]
    pub fn SetConditions(
        &self,
        conditions : & [ < ::windows::ApplicationModel::Background:: IBackgroundCondition as :: windows :: runtime :: DefaultType > :: DefaultType ],
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                conditions.len() as u32,
                ::std::mem::transmute(conditions.as_ptr()),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `Windows_PushNotifications`*"]
    pub fn CreateInstance<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
    >(
        activators: PushNotificationRegistrationActivators,
        taskclsid: Param1,
    ) -> ::windows::runtime::Result<PushNotificationActivationInfo> {
        Self::IPushNotificationActivationInfoFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                activators,
                taskclsid.into_param().abi(),
                &mut result__,
            )
            .from_abi::<PushNotificationActivationInfo>(result__)
        })
    }
    #[doc = "*Required features: `Windows_PushNotifications`*"]
    pub fn CreateInstance2(
        activators: PushNotificationRegistrationActivators,
    ) -> ::windows::runtime::Result<PushNotificationActivationInfo> {
        Self::IPushNotificationActivationInfoFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                activators,
                &mut result__,
            )
            .from_abi::<PushNotificationActivationInfo>(result__)
        })
    }
    pub fn IPushNotificationActivationInfoFactory<
        R,
        F: FnOnce(&IPushNotificationActivationInfoFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            PushNotificationActivationInfo,
            IPushNotificationActivationInfoFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PushNotificationActivationInfo {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.Windows.PushNotifications.PushNotificationActivationInfo;{f50b1226-8eb2-57b9-891f-9ceb3948c5c3})" ) ;
}
unsafe impl ::windows::runtime::Interface for PushNotificationActivationInfo {
    type Vtable = IPushNotificationActivationInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4111143462,
        36530,
        22457,
        [137, 31, 156, 235, 57, 72, 197, 195],
    );
}
impl ::windows::runtime::RuntimeName for PushNotificationActivationInfo {
    const NAME: &'static str = "Microsoft.Windows.PushNotifications.PushNotificationActivationInfo";
}
impl ::std::convert::From<PushNotificationActivationInfo> for ::windows::runtime::IUnknown {
    fn from(value: PushNotificationActivationInfo) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&PushNotificationActivationInfo> for ::windows::runtime::IUnknown {
    fn from(value: &PushNotificationActivationInfo) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for PushNotificationActivationInfo
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &'a PushNotificationActivationInfo
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<PushNotificationActivationInfo> for ::windows::runtime::IInspectable {
    fn from(value: PushNotificationActivationInfo) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PushNotificationActivationInfo> for ::windows::runtime::IInspectable {
    fn from(value: &PushNotificationActivationInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for PushNotificationActivationInfo
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a PushNotificationActivationInfo
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for PushNotificationActivationInfo {}
unsafe impl ::std::marker::Sync for PushNotificationActivationInfo {}
#[doc = "*Required features: `Windows_PushNotifications`*"]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct PushNotificationChannel(pub ::windows::runtime::IInspectable);
impl PushNotificationChannel {
    #[doc = "*Required features: `Windows_PushNotifications`*"]
    pub fn Uri(&self) -> ::windows::runtime::Result<::windows::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `Windows_PushNotifications`*"]
    pub fn ExpirationTime(&self) -> ::windows::runtime::Result<::windows::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::DateTime = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `Windows_PushNotifications`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this)).ok()
        }
    }
    #[doc = "*Required features: `Windows_PushNotifications`*"]
    pub fn PushReceived<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                PushNotificationChannel,
                PushNotificationReceivedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `Windows_PushNotifications`*"]
    pub fn RemovePushReceived<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `Windows_PushNotifications`*"]
    pub fn CreateInstance<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            ::windows::Networking::PushNotifications::PushNotificationChannel,
        >,
    >(
        channel: Param0,
    ) -> ::windows::runtime::Result<PushNotificationChannel> {
        Self::IPushNotificationChannelFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                channel.into_param().abi(),
                &mut result__,
            )
            .from_abi::<PushNotificationChannel>(result__)
        })
    }
    pub fn IPushNotificationChannelFactory<
        R,
        F: FnOnce(&IPushNotificationChannelFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            PushNotificationChannel,
            IPushNotificationChannelFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PushNotificationChannel {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.Windows.PushNotifications.PushNotificationChannel;{cd4ef2a0-3985-5848-8426-813de636f825})" ) ;
}
unsafe impl ::windows::runtime::Interface for PushNotificationChannel {
    type Vtable = IPushNotificationChannel_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3444503200,
        14725,
        22600,
        [132, 38, 129, 61, 230, 54, 248, 37],
    );
}
impl ::windows::runtime::RuntimeName for PushNotificationChannel {
    const NAME: &'static str = "Microsoft.Windows.PushNotifications.PushNotificationChannel";
}
impl ::std::convert::From<PushNotificationChannel> for ::windows::runtime::IUnknown {
    fn from(value: PushNotificationChannel) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&PushNotificationChannel> for ::windows::runtime::IUnknown {
    fn from(value: &PushNotificationChannel) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for PushNotificationChannel
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &'a PushNotificationChannel
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<PushNotificationChannel> for ::windows::runtime::IInspectable {
    fn from(value: PushNotificationChannel) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PushNotificationChannel> for ::windows::runtime::IInspectable {
    fn from(value: &PushNotificationChannel) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for PushNotificationChannel
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a PushNotificationChannel
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for PushNotificationChannel {}
unsafe impl ::std::marker::Sync for PushNotificationChannel {}
#[doc = "*Required features: `Windows_PushNotifications`*"]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct PushNotificationChannelStatus(pub i32);
impl PushNotificationChannelStatus {
    pub const InProgress: PushNotificationChannelStatus = PushNotificationChannelStatus(0i32);
    pub const InProgressRetry: PushNotificationChannelStatus = PushNotificationChannelStatus(1i32);
    pub const CompletedSuccess: PushNotificationChannelStatus = PushNotificationChannelStatus(2i32);
    pub const CompletedFailure: PushNotificationChannelStatus = PushNotificationChannelStatus(3i32);
}
impl ::std::convert::From<i32> for PushNotificationChannelStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PushNotificationChannelStatus {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PushNotificationChannelStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Microsoft.Windows.PushNotifications.PushNotificationChannelStatus;i4)",
    );
}
impl ::windows::runtime::DefaultType for PushNotificationChannelStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `Windows_PushNotifications`*"]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct PushNotificationCreateChannelResult(pub ::windows::runtime::IInspectable);
impl PushNotificationCreateChannelResult {
    #[doc = "*Required features: `Windows_PushNotifications`*"]
    pub fn Channel(&self) -> ::windows::runtime::Result<PushNotificationChannel> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<PushNotificationChannel>(result__)
        }
    }
    #[doc = "*Required features: `Windows_PushNotifications`*"]
    pub fn ExtendedError(&self) -> ::windows::runtime::Result<::windows::runtime::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::HRESULT = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HRESULT>(result__)
        }
    }
    #[doc = "*Required features: `Windows_PushNotifications`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<PushNotificationChannelStatus> {
        let this = self;
        unsafe {
            let mut result__: PushNotificationChannelStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<PushNotificationChannelStatus>(result__)
        }
    }
    #[doc = "*Required features: `Windows_PushNotifications`*"]
    pub fn CreateInstance<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, PushNotificationChannel>,
    >(
        channel: Param0,
        extendederror: ::windows::runtime::HRESULT,
        status: PushNotificationChannelStatus,
    ) -> ::windows::runtime::Result<PushNotificationCreateChannelResult> {
        Self::IPushNotificationCreateChannelResultFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                channel.into_param().abi(),
                extendederror,
                status,
                &mut result__,
            )
            .from_abi::<PushNotificationCreateChannelResult>(result__)
        })
    }
    pub fn IPushNotificationCreateChannelResultFactory<
        R,
        F: FnOnce(&IPushNotificationCreateChannelResultFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            PushNotificationCreateChannelResult,
            IPushNotificationCreateChannelResultFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PushNotificationCreateChannelResult {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.Windows.PushNotifications.PushNotificationCreateChannelResult;{4df3717f-5d33-56e9-b381-1b350c95722e})" ) ;
}
unsafe impl ::windows::runtime::Interface for PushNotificationCreateChannelResult {
    type Vtable = IPushNotificationCreateChannelResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1307799935,
        23859,
        22249,
        [179, 129, 27, 53, 12, 149, 114, 46],
    );
}
impl ::windows::runtime::RuntimeName for PushNotificationCreateChannelResult {
    const NAME: &'static str =
        "Microsoft.Windows.PushNotifications.PushNotificationCreateChannelResult";
}
impl ::std::convert::From<PushNotificationCreateChannelResult> for ::windows::runtime::IUnknown {
    fn from(value: PushNotificationCreateChannelResult) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&PushNotificationCreateChannelResult> for ::windows::runtime::IUnknown {
    fn from(value: &PushNotificationCreateChannelResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for PushNotificationCreateChannelResult
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &'a PushNotificationCreateChannelResult
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<PushNotificationCreateChannelResult>
    for ::windows::runtime::IInspectable
{
    fn from(value: PushNotificationCreateChannelResult) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PushNotificationCreateChannelResult>
    for ::windows::runtime::IInspectable
{
    fn from(value: &PushNotificationCreateChannelResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for PushNotificationCreateChannelResult
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a PushNotificationCreateChannelResult
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for PushNotificationCreateChannelResult {}
unsafe impl ::std::marker::Sync for PushNotificationCreateChannelResult {}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Windows_PushNotifications`*"]
pub struct PushNotificationCreateChannelStatus {
    pub status: PushNotificationChannelStatus,
    pub extendedError: ::windows::runtime::HRESULT,
    pub retryCount: u32,
}
impl PushNotificationCreateChannelStatus {}
impl ::std::default::Default for PushNotificationCreateChannelStatus {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PushNotificationCreateChannelStatus {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PushNotificationCreateChannelStatus")
            .field("status", &self.status)
            .field("extendedError", &self.extendedError)
            .field("retryCount", &self.retryCount)
            .finish()
    }
}
impl ::std::cmp::PartialEq for PushNotificationCreateChannelStatus {
    fn eq(&self, other: &Self) -> bool {
        self.status == other.status
            && self.extendedError == other.extendedError
            && self.retryCount == other.retryCount
    }
}
impl ::std::cmp::Eq for PushNotificationCreateChannelStatus {}
unsafe impl ::windows::runtime::Abi for PushNotificationCreateChannelStatus {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PushNotificationCreateChannelStatus {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"struct(Microsoft.Windows.PushNotifications.PushNotificationCreateChannelStatus;enum(Microsoft.Windows.PushNotifications.PushNotificationChannelStatus;i4);struct(Windows.Foundation.HResult;i4);u4)" ) ;
}
impl ::windows::runtime::DefaultType for PushNotificationCreateChannelStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `Windows_PushNotifications`*"]
pub struct PushNotificationManager {}
impl PushNotificationManager {
    #[doc = "*Required features: `Windows_PushNotifications`*"]
    pub fn RegisterActivator<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, PushNotificationActivationInfo>,
    >(
        details: Param0,
    ) -> ::windows::runtime::Result<()> {
        Self::IPushNotificationManagerStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                details.into_param().abi(),
            )
            .ok()
        })
    }
    #[doc = "*Required features: `Windows_PushNotifications`*"]
    pub fn UnregisterActivator(
        activators: PushNotificationRegistrationActivators,
    ) -> ::windows::runtime::Result<()> {
        Self::IPushNotificationManagerStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                activators,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `Windows_PushNotifications`*"]
    pub fn UnregisterAllActivators() -> ::windows::runtime::Result<()> {
        Self::IPushNotificationManagerStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this)).ok()
        })
    }
    #[doc = "*Required features: `Windows_PushNotifications`*"]
    pub fn CreateChannelAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>,
    >(
        remoteid: Param0,
    ) -> ::windows::runtime::Result<
        ::windows::Foundation::IAsyncOperationWithProgress<
            PushNotificationCreateChannelResult,
            PushNotificationCreateChannelStatus,
        >,
    > {
        Self::IPushNotificationManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                remoteid.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::IAsyncOperationWithProgress<
                PushNotificationCreateChannelResult,
                PushNotificationCreateChannelStatus,
            >>(result__)
        })
    }
    #[doc = "*Required features: `Windows_PushNotifications`*"]
    pub fn IsActivatorSupported(
        activators: PushNotificationRegistrationActivators,
    ) -> ::windows::runtime::Result<bool> {
        Self::IPushNotificationManagerStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                activators,
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    pub fn IPushNotificationManagerStatics<
        R,
        F: FnOnce(&IPushNotificationManagerStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            PushNotificationManager,
            IPushNotificationManagerStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for PushNotificationManager {
    const NAME: &'static str = "Microsoft.Windows.PushNotifications.PushNotificationManager";
}
#[doc = "*Required features: `Windows_PushNotifications`*"]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct PushNotificationReceivedEventArgs(pub ::windows::runtime::IInspectable);
impl PushNotificationReceivedEventArgs {
    #[doc = "*Required features: `Windows_PushNotifications`*"]
    pub fn Payload(&self) -> ::windows::runtime::Result<::windows::runtime::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::Array<u8> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                ::windows::runtime::Array::<u8>::set_abi_len(&mut result__),
                &mut result__ as *mut _ as _,
            )
            .and_then(|| result__)
        }
    }
    #[doc = "*Required features: `Windows_PushNotifications`*"]
    pub fn GetDeferral(
        &self,
    ) -> ::windows::runtime::Result<::windows::ApplicationModel::Background::BackgroundTaskDeferral>
    {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::ApplicationModel::Background::BackgroundTaskDeferral>(result__)
        }
    }
    #[doc = "*Required features: `Windows_PushNotifications`*"]
    pub fn Canceled<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            ::windows::ApplicationModel::Background::BackgroundTaskCanceledEventHandler,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `Windows_PushNotifications`*"]
    pub fn RemoveCanceled<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `Windows_PushNotifications`*"]
    pub fn Handled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Windows_PushNotifications`*"]
    pub fn SetHandled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PushNotificationReceivedEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.Windows.PushNotifications.PushNotificationReceivedEventArgs;{a8fe81bc-d2d9-52cf-ab1b-6fde711862ea})" ) ;
}
unsafe impl ::windows::runtime::Interface for PushNotificationReceivedEventArgs {
    type Vtable = IPushNotificationReceivedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2835251644,
        53977,
        21199,
        [171, 27, 111, 222, 113, 24, 98, 234],
    );
}
impl ::windows::runtime::RuntimeName for PushNotificationReceivedEventArgs {
    const NAME: &'static str =
        "Microsoft.Windows.PushNotifications.PushNotificationReceivedEventArgs";
}
impl ::std::convert::From<PushNotificationReceivedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: PushNotificationReceivedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&PushNotificationReceivedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &PushNotificationReceivedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for PushNotificationReceivedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &'a PushNotificationReceivedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<PushNotificationReceivedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: PushNotificationReceivedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PushNotificationReceivedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &PushNotificationReceivedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for PushNotificationReceivedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a PushNotificationReceivedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for PushNotificationReceivedEventArgs {}
unsafe impl ::std::marker::Sync for PushNotificationReceivedEventArgs {}
#[doc = "*Required features: `Windows_PushNotifications`*"]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct PushNotificationRegistrationActivators(pub u32);
impl PushNotificationRegistrationActivators {
    pub const Undefined: PushNotificationRegistrationActivators =
        PushNotificationRegistrationActivators(0u32);
    pub const PushTrigger: PushNotificationRegistrationActivators =
        PushNotificationRegistrationActivators(1u32);
    pub const ComActivator: PushNotificationRegistrationActivators =
        PushNotificationRegistrationActivators(2u32);
    pub const ProtocolActivator: PushNotificationRegistrationActivators =
        PushNotificationRegistrationActivators(4u32);
}
impl ::std::convert::From<u32> for PushNotificationRegistrationActivators {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PushNotificationRegistrationActivators {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PushNotificationRegistrationActivators {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Microsoft.Windows.PushNotifications.PushNotificationRegistrationActivators;u4)",
    );
}
impl ::windows::runtime::DefaultType for PushNotificationRegistrationActivators {
    type DefaultType = Self;
}
impl ::std::ops::BitOr for PushNotificationRegistrationActivators {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for PushNotificationRegistrationActivators {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for PushNotificationRegistrationActivators {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for PushNotificationRegistrationActivators {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for PushNotificationRegistrationActivators {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
