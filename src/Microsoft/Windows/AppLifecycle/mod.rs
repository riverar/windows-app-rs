#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[doc = "*Required features: `Windows_AppLifecycle`*"]
pub struct ActivationRegistrationManager {}
impl ActivationRegistrationManager {
    #[doc = "*Required features: `Windows_AppLifecycle`*"]
    pub fn RegisterForFileTypeActivation<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param4: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        supportedfiletypes : & [ < :: windows :: runtime :: HSTRING as :: windows :: runtime :: DefaultType > :: DefaultType ],
        logo: Param1,
        displayname: Param2,
        supportedverbs : & [ < :: windows :: runtime :: HSTRING as :: windows :: runtime :: DefaultType > :: DefaultType ],
        exepath: Param4,
    ) -> ::windows::runtime::Result<()> {
        Self::IActivationRegistrationManagerStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                supportedfiletypes.len() as u32,
                ::core::mem::transmute(supportedfiletypes.as_ptr()),
                logo.into_param().abi(),
                displayname.into_param().abi(),
                supportedverbs.len() as u32,
                ::core::mem::transmute(supportedverbs.as_ptr()),
                exepath.into_param().abi(),
            )
            .ok()
        })
    }
    #[doc = "*Required features: `Windows_AppLifecycle`*"]
    pub fn RegisterForProtocolActivation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        scheme: Param0,
        logo: Param1,
        displayname: Param2,
        exepath: Param3,
    ) -> ::windows::runtime::Result<()> {
        Self::IActivationRegistrationManagerStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                scheme.into_param().abi(),
                logo.into_param().abi(),
                displayname.into_param().abi(),
                exepath.into_param().abi(),
            )
            .ok()
        })
    }
    #[doc = "*Required features: `Windows_AppLifecycle`*"]
    pub fn RegisterForStartupActivation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        taskid: Param0,
        exepath: Param1,
    ) -> ::windows::runtime::Result<()> {
        Self::IActivationRegistrationManagerStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                taskid.into_param().abi(),
                exepath.into_param().abi(),
            )
            .ok()
        })
    }
    #[doc = "*Required features: `Windows_AppLifecycle`*"]
    pub fn UnregisterForFileTypeActivation<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        filetypes : & [ < :: windows :: runtime :: HSTRING as :: windows :: runtime :: DefaultType > :: DefaultType ],
        exepath: Param1,
    ) -> ::windows::runtime::Result<()> {
        Self::IActivationRegistrationManagerStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                filetypes.len() as u32,
                ::core::mem::transmute(filetypes.as_ptr()),
                exepath.into_param().abi(),
            )
            .ok()
        })
    }
    #[doc = "*Required features: `Windows_AppLifecycle`*"]
    pub fn UnregisterForProtocolActivation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        scheme: Param0,
        exepath: Param1,
    ) -> ::windows::runtime::Result<()> {
        Self::IActivationRegistrationManagerStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                scheme.into_param().abi(),
                exepath.into_param().abi(),
            )
            .ok()
        })
    }
    #[doc = "*Required features: `Windows_AppLifecycle`*"]
    pub fn UnregisterForStartupActivation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        taskid: Param0,
    ) -> ::windows::runtime::Result<()> {
        Self::IActivationRegistrationManagerStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                taskid.into_param().abi(),
            )
            .ok()
        })
    }
    pub fn IActivationRegistrationManagerStatics<
        R,
        F: FnOnce(&IActivationRegistrationManagerStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            ActivationRegistrationManager,
            IActivationRegistrationManagerStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for ActivationRegistrationManager {
    const NAME: &'static str = "Microsoft.Windows.AppLifecycle.ActivationRegistrationManager";
}
#[doc = "*Required features: `Windows_AppLifecycle`*"]
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct AppActivationArguments(pub ::windows::runtime::IInspectable);
impl AppActivationArguments {
    #[doc = "*Required features: `Windows_AppLifecycle`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ExtendedActivationKind> {
        let this = self;
        unsafe {
            let mut result__: ExtendedActivationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ExtendedActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `Windows_AppLifecycle`*"]
    pub fn Data(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppActivationArguments {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.Windows.AppLifecycle.AppActivationArguments;{14f99eaf-1580-5062-bdc8-d5d1c31138fb})" ) ;
}
unsafe impl ::windows::runtime::Interface for AppActivationArguments {
    type Vtable = IAppActivationArguments_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        351903407,
        5504,
        20578,
        [189, 200, 213, 209, 195, 17, 56, 251],
    );
}
impl ::windows::runtime::RuntimeName for AppActivationArguments {
    const NAME: &'static str = "Microsoft.Windows.AppLifecycle.AppActivationArguments";
}
impl ::core::convert::From<AppActivationArguments> for ::windows::runtime::IUnknown {
    fn from(value: AppActivationArguments) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppActivationArguments> for ::windows::runtime::IUnknown {
    fn from(value: &AppActivationArguments) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for AppActivationArguments
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &'a AppActivationArguments
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppActivationArguments> for ::windows::runtime::IInspectable {
    fn from(value: AppActivationArguments) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppActivationArguments> for ::windows::runtime::IInspectable {
    fn from(value: &AppActivationArguments) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for AppActivationArguments
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a AppActivationArguments
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AppActivationArguments {}
unsafe impl ::core::marker::Sync for AppActivationArguments {}
#[doc = "*Required features: `Windows_AppLifecycle`*"]
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct AppInstance(pub ::windows::runtime::IInspectable);
impl AppInstance {
    #[doc = "*Required features: `Windows_AppLifecycle`*"]
    pub fn UnregisterKey(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok()
        }
    }
    #[doc = "*Required features: `Windows_AppLifecycle`*"]
    pub fn RedirectActivationToAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, AppActivationArguments>,
    >(
        &self,
        args: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                args.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `Windows_AppLifecycle`*"]
    pub fn GetActivatedEventArgs(&self) -> ::windows::runtime::Result<AppActivationArguments> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AppActivationArguments>(result__)
        }
    }
    #[doc = "*Required features: `Windows_AppLifecycle`*"]
    pub fn Activated<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            ::windows::Foundation::EventHandler<AppActivationArguments>,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `Windows_AppLifecycle`*"]
    pub fn RemoveActivated<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `Windows_AppLifecycle`*"]
    pub fn Key(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Windows_AppLifecycle`*"]
    pub fn IsCurrent(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Windows_AppLifecycle`*"]
    pub fn ProcessId(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Windows_AppLifecycle`*"]
    pub fn GetCurrent() -> ::windows::runtime::Result<AppInstance> {
        Self::IAppInstanceStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AppInstance>(result__)
        })
    }
    #[doc = "*Required features: `Windows_AppLifecycle`*"]
    pub fn GetInstances(
    ) -> ::windows::runtime::Result<::windows::Foundation::Collections::IVector<AppInstance>> {
        Self::IAppInstanceStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Collections::IVector<AppInstance>>(result__)
        })
    }
    #[doc = "*Required features: `Windows_AppLifecycle`*"]
    pub fn FindOrRegisterForKey<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        key: Param0,
    ) -> ::windows::runtime::Result<AppInstance> {
        Self::IAppInstanceStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                key.into_param().abi(),
                &mut result__,
            )
            .from_abi::<AppInstance>(result__)
        })
    }
    pub fn IAppInstanceStatics<
        R,
        F: FnOnce(&IAppInstanceStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<AppInstance, IAppInstanceStatics> =
            ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppInstance {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Microsoft.Windows.AppLifecycle.AppInstance;{75766ae4-0239-5a26-b9da-d5bfc75a4866})",
    );
}
unsafe impl ::windows::runtime::Interface for AppInstance {
    type Vtable = IAppInstance_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1970694884,
        569,
        23078,
        [185, 218, 213, 191, 199, 90, 72, 102],
    );
}
impl ::windows::runtime::RuntimeName for AppInstance {
    const NAME: &'static str = "Microsoft.Windows.AppLifecycle.AppInstance";
}
impl ::core::convert::From<AppInstance> for ::windows::runtime::IUnknown {
    fn from(value: AppInstance) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppInstance> for ::windows::runtime::IUnknown {
    fn from(value: &AppInstance) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AppInstance {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a AppInstance {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppInstance> for ::windows::runtime::IInspectable {
    fn from(value: AppInstance) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppInstance> for ::windows::runtime::IInspectable {
    fn from(value: &AppInstance) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for AppInstance {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a AppInstance {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AppInstance {}
unsafe impl ::core::marker::Sync for AppInstance {}
#[doc = "*Required features: `Windows_AppLifecycle`*"]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: marker :: Copy,
    :: core :: clone :: Clone,
    :: core :: default :: Default,
    :: core :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct ExtendedActivationKind(pub i32);
impl ExtendedActivationKind {
    pub const Launch: ExtendedActivationKind = ExtendedActivationKind(0i32);
    pub const Search: ExtendedActivationKind = ExtendedActivationKind(1i32);
    pub const ShareTarget: ExtendedActivationKind = ExtendedActivationKind(2i32);
    pub const File: ExtendedActivationKind = ExtendedActivationKind(3i32);
    pub const Protocol: ExtendedActivationKind = ExtendedActivationKind(4i32);
    pub const FileOpenPicker: ExtendedActivationKind = ExtendedActivationKind(5i32);
    pub const FileSavePicker: ExtendedActivationKind = ExtendedActivationKind(6i32);
    pub const CachedFileUpdater: ExtendedActivationKind = ExtendedActivationKind(7i32);
    pub const ContactPicker: ExtendedActivationKind = ExtendedActivationKind(8i32);
    pub const Device: ExtendedActivationKind = ExtendedActivationKind(9i32);
    pub const PrintTaskSettings: ExtendedActivationKind = ExtendedActivationKind(10i32);
    pub const CameraSettings: ExtendedActivationKind = ExtendedActivationKind(11i32);
    pub const RestrictedLaunch: ExtendedActivationKind = ExtendedActivationKind(12i32);
    pub const AppointmentsProvider: ExtendedActivationKind = ExtendedActivationKind(13i32);
    pub const Contact: ExtendedActivationKind = ExtendedActivationKind(14i32);
    pub const LockScreenCall: ExtendedActivationKind = ExtendedActivationKind(15i32);
    pub const VoiceCommand: ExtendedActivationKind = ExtendedActivationKind(16i32);
    pub const LockScreen: ExtendedActivationKind = ExtendedActivationKind(17i32);
    pub const PickerReturned: ExtendedActivationKind = ExtendedActivationKind(1000i32);
    pub const WalletAction: ExtendedActivationKind = ExtendedActivationKind(1001i32);
    pub const PickFileContinuation: ExtendedActivationKind = ExtendedActivationKind(1002i32);
    pub const PickSaveFileContinuation: ExtendedActivationKind = ExtendedActivationKind(1003i32);
    pub const PickFolderContinuation: ExtendedActivationKind = ExtendedActivationKind(1004i32);
    pub const WebAuthenticationBrokerContinuation: ExtendedActivationKind =
        ExtendedActivationKind(1005i32);
    pub const WebAccountProvider: ExtendedActivationKind = ExtendedActivationKind(1006i32);
    pub const ComponentUI: ExtendedActivationKind = ExtendedActivationKind(1007i32);
    pub const ProtocolForResults: ExtendedActivationKind = ExtendedActivationKind(1009i32);
    pub const ToastNotification: ExtendedActivationKind = ExtendedActivationKind(1010i32);
    pub const Print3DWorkflow: ExtendedActivationKind = ExtendedActivationKind(1011i32);
    pub const DialReceiver: ExtendedActivationKind = ExtendedActivationKind(1012i32);
    pub const DevicePairing: ExtendedActivationKind = ExtendedActivationKind(1013i32);
    pub const UserDataAccountsProvider: ExtendedActivationKind = ExtendedActivationKind(1014i32);
    pub const FilePickerExperience: ExtendedActivationKind = ExtendedActivationKind(1015i32);
    pub const LockScreenComponent: ExtendedActivationKind = ExtendedActivationKind(1016i32);
    pub const ContactPanel: ExtendedActivationKind = ExtendedActivationKind(1017i32);
    pub const PrintWorkflowForegroundTask: ExtendedActivationKind = ExtendedActivationKind(1018i32);
    pub const GameUIProvider: ExtendedActivationKind = ExtendedActivationKind(1019i32);
    pub const StartupTask: ExtendedActivationKind = ExtendedActivationKind(1020i32);
    pub const CommandLineLaunch: ExtendedActivationKind = ExtendedActivationKind(1021i32);
    pub const BarcodeScannerProvider: ExtendedActivationKind = ExtendedActivationKind(1022i32);
    pub const PrintSupportJobUI: ExtendedActivationKind = ExtendedActivationKind(1023i32);
    pub const PrintSupportSettingsUI: ExtendedActivationKind = ExtendedActivationKind(1024i32);
    pub const PhoneCallActivation: ExtendedActivationKind = ExtendedActivationKind(1025i32);
    pub const VpnForeground: ExtendedActivationKind = ExtendedActivationKind(1026i32);
    pub const Push: ExtendedActivationKind = ExtendedActivationKind(5000i32);
}
impl ::core::convert::From<i32> for ExtendedActivationKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ExtendedActivationKind {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ExtendedActivationKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Microsoft.Windows.AppLifecycle.ExtendedActivationKind;i4)",
    );
}
impl ::windows::runtime::DefaultType for ExtendedActivationKind {
    type DefaultType = Self;
}
#[repr(transparent)]
#[doc(hidden)]
pub struct IActivationRegistrationManagerStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IActivationRegistrationManagerStatics {
    type Vtable = IActivationRegistrationManagerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1522854190,
        379,
        23912,
        [129, 152, 246, 134, 54, 171, 153, 211],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivationRegistrationManagerStatics_abi(
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
        supportedFileTypes_array_size: u32,
        supportedfiletypes: *const ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        logo: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        displayname: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        supportedVerbs_array_size: u32,
        supportedverbs: *const ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        exepath: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        scheme: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        logo: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        displayname: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        exepath: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        taskid: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        exepath: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        fileTypes_array_size: u32,
        filetypes: *const ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        exepath: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        scheme: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        exepath: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        taskid: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppActivationArguments(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppActivationArguments {
    type Vtable = IAppActivationArguments_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        351903407,
        5504,
        20578,
        [189, 200, 213, 209, 195, 17, 56, 251],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppActivationArguments_abi(
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
        result__: *mut ExtendedActivationKind,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppInstance(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppInstance {
    type Vtable = IAppInstance_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1970694884,
        569,
        23078,
        [185, 218, 213, 191, 199, 90, 72, 102],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstance_abi(
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        args: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
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
        result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut u32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppInstanceStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppInstanceStatics {
    type Vtable = IAppInstanceStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1329679141,
        33584,
        23195,
        [187, 193, 130, 41, 212, 121, 100, 157],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstanceStatics_abi(
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
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        key: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
