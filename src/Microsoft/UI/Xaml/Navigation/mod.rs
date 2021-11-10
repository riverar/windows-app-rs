#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[doc = "*Required features: `UI_Xaml_Navigation`*"]
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct FrameNavigationOptions(pub ::windows::runtime::IInspectable);
impl FrameNavigationOptions {
    #[doc = "*Required features: `UI_Xaml_Navigation`*"]
    pub fn IsNavigationStackEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Navigation`*"]
    pub fn SetIsNavigationStackEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Xaml_Media_Animation")]
    #[doc = "*Required features: `UI_Xaml_Navigation`, `UI_Xaml_Media_Animation`*"]
    pub fn TransitionInfoOverride(
        &self,
    ) -> ::windows::runtime::Result<super::Media::Animation::NavigationTransitionInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::Animation::NavigationTransitionInfo>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media_Animation")]
    #[doc = "*Required features: `UI_Xaml_Navigation`, `UI_Xaml_Media_Animation`*"]
    pub fn SetTransitionInfoOverride<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Media::Animation::NavigationTransitionInfo>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Navigation`*"]
    pub fn new() -> ::windows::runtime::Result<FrameNavigationOptions> {
        Self::IFrameNavigationOptionsFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                ::core::ptr::null_mut(),
                &mut ::core::option::Option::<::windows::runtime::IInspectable>::None as *mut _
                    as _,
                &mut result__,
            )
            .from_abi::<FrameNavigationOptions>(result__)
        })
    }
    pub fn IFrameNavigationOptionsFactory<
        R,
        F: FnOnce(&IFrameNavigationOptionsFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            FrameNavigationOptions,
            IFrameNavigationOptionsFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for FrameNavigationOptions {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Navigation.FrameNavigationOptions;{390de593-14cf-5312-af99-6cd8d59ec5d5})" ) ;
}
unsafe impl ::windows::runtime::Interface for FrameNavigationOptions {
    type Vtable = IFrameNavigationOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        957212051,
        5327,
        21266,
        [175, 153, 108, 216, 213, 158, 197, 213],
    );
}
impl ::windows::runtime::RuntimeName for FrameNavigationOptions {
    const NAME: &'static str = "Microsoft.UI.Xaml.Navigation.FrameNavigationOptions";
}
impl ::core::convert::From<FrameNavigationOptions> for ::windows::runtime::IUnknown {
    fn from(value: FrameNavigationOptions) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&FrameNavigationOptions> for ::windows::runtime::IUnknown {
    fn from(value: &FrameNavigationOptions) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for FrameNavigationOptions
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &'a FrameNavigationOptions
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<FrameNavigationOptions> for ::windows::runtime::IInspectable {
    fn from(value: FrameNavigationOptions) -> Self {
        value.0
    }
}
impl ::core::convert::From<&FrameNavigationOptions> for ::windows::runtime::IInspectable {
    fn from(value: &FrameNavigationOptions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for FrameNavigationOptions
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a FrameNavigationOptions
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for FrameNavigationOptions {}
unsafe impl ::core::marker::Sync for FrameNavigationOptions {}
#[repr(transparent)]
#[doc(hidden)]
pub struct IFrameNavigationOptions(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IFrameNavigationOptions {
    type Vtable = IFrameNavigationOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        957212051,
        5327,
        21266,
        [175, 153, 108, 216, 213, 158, 197, 213],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameNavigationOptions_abi(
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
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "UI_Xaml_Media_Animation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media_Animation"))] usize,
    #[cfg(feature = "UI_Xaml_Media_Animation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media_Animation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IFrameNavigationOptionsFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IFrameNavigationOptionsFactory {
    type Vtable = IFrameNavigationOptionsFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3723753288,
        28967,
        23790,
        [159, 121, 172, 40, 26, 35, 70, 50],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameNavigationOptionsFactory_abi(
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
        baseinterface: ::windows::runtime::RawPtr,
        innerinterface: *mut ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct INavigatingCancelEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for INavigatingCancelEventArgs {
    type Vtable = INavigatingCancelEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        389013010,
        57455,
        24054,
        [147, 14, 95, 172, 247, 179, 251, 231],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct INavigatingCancelEventArgs_abi(
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
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut NavigationMode,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::core::mem::ManuallyDrop<::windows::UI::Xaml::Interop::TypeName>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "UI_Xaml_Media_Animation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media_Animation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct INavigationEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for INavigationEventArgs {
    type Vtable = INavigationEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2271965364,
        10531,
        22405,
        [156, 234, 46, 68, 170, 7, 97, 189],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct INavigationEventArgs_abi(
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
    #[cfg(feature = "UI_Xaml_Media_Animation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media_Animation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::core::mem::ManuallyDrop<::windows::UI::Xaml::Interop::TypeName>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut NavigationMode,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct INavigationFailedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for INavigationFailedEventArgs {
    type Vtable = INavigationFailedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4161337760,
        4876,
        22900,
        [135, 248, 68, 51, 39, 26, 53, 169],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct INavigationFailedEventArgs_abi(
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
        result__: *mut ::windows::runtime::HRESULT,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::core::mem::ManuallyDrop<::windows::UI::Xaml::Interop::TypeName>,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPageStackEntry(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPageStackEntry {
    type Vtable = IPageStackEntry_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3583112558,
        16994,
        23697,
        [157, 121, 41, 22, 92, 216, 33, 0],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IPageStackEntry_abi(
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
        result__: *mut ::core::mem::ManuallyDrop<::windows::UI::Xaml::Interop::TypeName>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "UI_Xaml_Media_Animation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media_Animation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPageStackEntryFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPageStackEntryFactory {
    type Vtable = IPageStackEntryFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2119865449,
        24840,
        24210,
        [164, 153, 94, 233, 240, 101, 166, 138],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IPageStackEntryFactory_abi(
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
    #[cfg(feature = "UI_Xaml_Media_Animation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        sourcepagetype: ::core::mem::ManuallyDrop<::windows::UI::Xaml::Interop::TypeName>,
        parameter: ::windows::runtime::RawPtr,
        navigationtransitioninfo: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media_Animation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPageStackEntryStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPageStackEntryStatics {
    type Vtable = IPageStackEntryStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        790449335,
        37435,
        22971,
        [191, 196, 117, 9, 51, 242, 131, 133],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IPageStackEntryStatics_abi(
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
);
#[doc = "*Required features: `UI_Xaml_Navigation`*"]
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct NavigatedEventHandler(::windows::runtime::IUnknown);
impl NavigatedEventHandler {
    pub fn new<
        F: FnMut(
                &::core::option::Option<::windows::runtime::IInspectable>,
                &::core::option::Option<NavigationEventArgs>,
            ) -> ::windows::runtime::Result<()>
            + 'static,
    >(
        invoke: F,
    ) -> Self {
        let com = NavigatedEventHandler_box::<F> {
            vtable: &NavigatedEventHandler_box::<F>::VTABLE,
            count: ::windows::runtime::RefCount::new(1),
            invoke,
        };
        unsafe { core::mem::transmute(::windows::runtime::alloc::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `UI_Xaml_Navigation`*"]
    pub fn Invoke<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>,
        Param1: ::windows::runtime::IntoParam<'a, NavigationEventArgs>,
    >(
        &self,
        sender: Param0,
        e: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).3)(
                ::core::mem::transmute_copy(this),
                sender.into_param().abi(),
                e.into_param().abi(),
            )
            .ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for NavigatedEventHandler {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"delegate({8631b517-6d8e-58ee-82fe-d4034d1bd7c1})",
    );
}
unsafe impl ::windows::runtime::Interface for NavigatedEventHandler {
    type Vtable = NavigatedEventHandler_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2251404567,
        28046,
        22766,
        [130, 254, 212, 3, 77, 27, 215, 193],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct NavigatedEventHandler_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        sender: ::windows::runtime::RawPtr,
        e: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(C)]
struct NavigatedEventHandler_box<
    F: FnMut(
            &::core::option::Option<::windows::runtime::IInspectable>,
            &::core::option::Option<NavigationEventArgs>,
        ) -> ::windows::runtime::Result<()>
        + 'static,
> {
    vtable: *const NavigatedEventHandler_abi,
    invoke: F,
    count: ::windows::runtime::RefCount,
}
impl<
        F: FnMut(
                &::core::option::Option<::windows::runtime::IInspectable>,
                &::core::option::Option<NavigationEventArgs>,
            ) -> ::windows::runtime::Result<()>
            + 'static,
    > NavigatedEventHandler_box<F>
{
    const VTABLE: NavigatedEventHandler_abi = NavigatedEventHandler_abi(
        Self::QueryInterface,
        Self::AddRef,
        Self::Release,
        Self::Invoke,
    );
    unsafe extern "system" fn QueryInterface(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        *interface = if iid == &<NavigatedEventHandler as ::windows::runtime::Interface>::IID
            || iid == &<::windows::runtime::IUnknown as ::windows::runtime::Interface>::IID
            || iid == &<::windows::runtime::IAgileObject as ::windows::runtime::Interface>::IID
        {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::core::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::runtime::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::runtime::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows::runtime::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(
        this: ::windows::runtime::RawPtr,
        sender: ::windows::runtime::RawPtr,
        e: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        ( ( * this ) . invoke ) ( & * ( & sender as * const < :: windows :: runtime :: IInspectable as :: windows :: runtime :: Abi > :: Abi as * const < :: windows :: runtime :: IInspectable as :: windows :: runtime :: DefaultType > :: DefaultType ) , & * ( & e as * const < NavigationEventArgs as :: windows :: runtime :: Abi > :: Abi as * const < NavigationEventArgs as :: windows :: runtime :: DefaultType > :: DefaultType ) , ) . into ( )
    }
}
#[doc = "*Required features: `UI_Xaml_Navigation`*"]
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct NavigatingCancelEventArgs(pub ::windows::runtime::IInspectable);
impl NavigatingCancelEventArgs {
    #[doc = "*Required features: `UI_Xaml_Navigation`*"]
    pub fn Cancel(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Navigation`*"]
    pub fn SetCancel(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Navigation`*"]
    pub fn NavigationMode(&self) -> ::windows::runtime::Result<NavigationMode> {
        let this = self;
        unsafe {
            let mut result__: NavigationMode = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<NavigationMode>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Navigation`*"]
    pub fn SourcePageType(
        &self,
    ) -> ::windows::runtime::Result<::windows::UI::Xaml::Interop::TypeName> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::UI::Xaml::Interop::TypeName> =
                ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Xaml::Interop::TypeName>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Navigation`*"]
    pub fn Parameter(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media_Animation")]
    #[doc = "*Required features: `UI_Xaml_Navigation`, `UI_Xaml_Media_Animation`*"]
    pub fn NavigationTransitionInfo(
        &self,
    ) -> ::windows::runtime::Result<super::Media::Animation::NavigationTransitionInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::Animation::NavigationTransitionInfo>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for NavigatingCancelEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Navigation.NavigatingCancelEventArgs;{172fde12-e06f-5df6-930e-5facf7b3fbe7})" ) ;
}
unsafe impl ::windows::runtime::Interface for NavigatingCancelEventArgs {
    type Vtable = INavigatingCancelEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        389013010,
        57455,
        24054,
        [147, 14, 95, 172, 247, 179, 251, 231],
    );
}
impl ::windows::runtime::RuntimeName for NavigatingCancelEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Navigation.NavigatingCancelEventArgs";
}
impl ::core::convert::From<NavigatingCancelEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: NavigatingCancelEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&NavigatingCancelEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &NavigatingCancelEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for NavigatingCancelEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &'a NavigatingCancelEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<NavigatingCancelEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: NavigatingCancelEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&NavigatingCancelEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &NavigatingCancelEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for NavigatingCancelEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a NavigatingCancelEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for NavigatingCancelEventArgs {}
unsafe impl ::core::marker::Sync for NavigatingCancelEventArgs {}
#[doc = "*Required features: `UI_Xaml_Navigation`*"]
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct NavigatingCancelEventHandler(::windows::runtime::IUnknown);
impl NavigatingCancelEventHandler {
    pub fn new<
        F: FnMut(
                &::core::option::Option<::windows::runtime::IInspectable>,
                &::core::option::Option<NavigatingCancelEventArgs>,
            ) -> ::windows::runtime::Result<()>
            + 'static,
    >(
        invoke: F,
    ) -> Self {
        let com = NavigatingCancelEventHandler_box::<F> {
            vtable: &NavigatingCancelEventHandler_box::<F>::VTABLE,
            count: ::windows::runtime::RefCount::new(1),
            invoke,
        };
        unsafe { core::mem::transmute(::windows::runtime::alloc::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `UI_Xaml_Navigation`*"]
    pub fn Invoke<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>,
        Param1: ::windows::runtime::IntoParam<'a, NavigatingCancelEventArgs>,
    >(
        &self,
        sender: Param0,
        e: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).3)(
                ::core::mem::transmute_copy(this),
                sender.into_param().abi(),
                e.into_param().abi(),
            )
            .ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for NavigatingCancelEventHandler {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"delegate({fcae1401-ec94-565f-9f48-7c4b6272b3b1})",
    );
}
unsafe impl ::windows::runtime::Interface for NavigatingCancelEventHandler {
    type Vtable = NavigatingCancelEventHandler_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4239266817,
        60564,
        22111,
        [159, 72, 124, 75, 98, 114, 179, 177],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct NavigatingCancelEventHandler_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        sender: ::windows::runtime::RawPtr,
        e: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(C)]
struct NavigatingCancelEventHandler_box<
    F: FnMut(
            &::core::option::Option<::windows::runtime::IInspectable>,
            &::core::option::Option<NavigatingCancelEventArgs>,
        ) -> ::windows::runtime::Result<()>
        + 'static,
> {
    vtable: *const NavigatingCancelEventHandler_abi,
    invoke: F,
    count: ::windows::runtime::RefCount,
}
impl<
        F: FnMut(
                &::core::option::Option<::windows::runtime::IInspectable>,
                &::core::option::Option<NavigatingCancelEventArgs>,
            ) -> ::windows::runtime::Result<()>
            + 'static,
    > NavigatingCancelEventHandler_box<F>
{
    const VTABLE: NavigatingCancelEventHandler_abi = NavigatingCancelEventHandler_abi(
        Self::QueryInterface,
        Self::AddRef,
        Self::Release,
        Self::Invoke,
    );
    unsafe extern "system" fn QueryInterface(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        *interface = if iid == &<NavigatingCancelEventHandler as ::windows::runtime::Interface>::IID
            || iid == &<::windows::runtime::IUnknown as ::windows::runtime::Interface>::IID
            || iid == &<::windows::runtime::IAgileObject as ::windows::runtime::Interface>::IID
        {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::core::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::runtime::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::runtime::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows::runtime::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(
        this: ::windows::runtime::RawPtr,
        sender: ::windows::runtime::RawPtr,
        e: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        ( ( * this ) . invoke ) ( & * ( & sender as * const < :: windows :: runtime :: IInspectable as :: windows :: runtime :: Abi > :: Abi as * const < :: windows :: runtime :: IInspectable as :: windows :: runtime :: DefaultType > :: DefaultType ) , & * ( & e as * const < NavigatingCancelEventArgs as :: windows :: runtime :: Abi > :: Abi as * const < NavigatingCancelEventArgs as :: windows :: runtime :: DefaultType > :: DefaultType ) , ) . into ( )
    }
}
#[doc = "*Required features: `UI_Xaml_Navigation`*"]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: marker :: Copy,
    :: core :: clone :: Clone,
    :: core :: default :: Default,
    :: core :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct NavigationCacheMode(pub i32);
impl NavigationCacheMode {
    pub const Disabled: NavigationCacheMode = NavigationCacheMode(0i32);
    pub const Required: NavigationCacheMode = NavigationCacheMode(1i32);
    pub const Enabled: NavigationCacheMode = NavigationCacheMode(2i32);
}
impl ::core::convert::From<i32> for NavigationCacheMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NavigationCacheMode {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for NavigationCacheMode {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Navigation.NavigationCacheMode;i4)",
    );
}
impl ::windows::runtime::DefaultType for NavigationCacheMode {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Xaml_Navigation`*"]
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct NavigationEventArgs(pub ::windows::runtime::IInspectable);
impl NavigationEventArgs {
    #[doc = "*Required features: `UI_Xaml_Navigation`*"]
    pub fn Content(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Navigation`*"]
    pub fn Parameter(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
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
    #[cfg(feature = "UI_Xaml_Media_Animation")]
    #[doc = "*Required features: `UI_Xaml_Navigation`, `UI_Xaml_Media_Animation`*"]
    pub fn NavigationTransitionInfo(
        &self,
    ) -> ::windows::runtime::Result<super::Media::Animation::NavigationTransitionInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::Animation::NavigationTransitionInfo>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Navigation`*"]
    pub fn SourcePageType(
        &self,
    ) -> ::windows::runtime::Result<::windows::UI::Xaml::Interop::TypeName> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::UI::Xaml::Interop::TypeName> =
                ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Xaml::Interop::TypeName>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Navigation`*"]
    pub fn NavigationMode(&self) -> ::windows::runtime::Result<NavigationMode> {
        let this = self;
        unsafe {
            let mut result__: NavigationMode = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<NavigationMode>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Navigation`*"]
    pub fn Uri(&self) -> ::windows::runtime::Result<::windows::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Navigation`*"]
    pub fn SetUri<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::Uri>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for NavigationEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Navigation.NavigationEventArgs;{876b70b4-2923-5785-9cea-2e44aa0761bd})" ) ;
}
unsafe impl ::windows::runtime::Interface for NavigationEventArgs {
    type Vtable = INavigationEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2271965364,
        10531,
        22405,
        [156, 234, 46, 68, 170, 7, 97, 189],
    );
}
impl ::windows::runtime::RuntimeName for NavigationEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Navigation.NavigationEventArgs";
}
impl ::core::convert::From<NavigationEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: NavigationEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&NavigationEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &NavigationEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for NavigationEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &'a NavigationEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<NavigationEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: NavigationEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&NavigationEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &NavigationEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for NavigationEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a NavigationEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for NavigationEventArgs {}
unsafe impl ::core::marker::Sync for NavigationEventArgs {}
#[doc = "*Required features: `UI_Xaml_Navigation`*"]
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct NavigationFailedEventArgs(pub ::windows::runtime::IInspectable);
impl NavigationFailedEventArgs {
    #[doc = "*Required features: `UI_Xaml_Navigation`*"]
    pub fn Exception(&self) -> ::windows::runtime::Result<::windows::runtime::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::HRESULT = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HRESULT>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Navigation`*"]
    pub fn Handled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Navigation`*"]
    pub fn SetHandled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Navigation`*"]
    pub fn SourcePageType(
        &self,
    ) -> ::windows::runtime::Result<::windows::UI::Xaml::Interop::TypeName> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::UI::Xaml::Interop::TypeName> =
                ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Xaml::Interop::TypeName>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for NavigationFailedEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Navigation.NavigationFailedEventArgs;{f808f9a0-130c-5974-87f8-4433271a35a9})" ) ;
}
unsafe impl ::windows::runtime::Interface for NavigationFailedEventArgs {
    type Vtable = INavigationFailedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4161337760,
        4876,
        22900,
        [135, 248, 68, 51, 39, 26, 53, 169],
    );
}
impl ::windows::runtime::RuntimeName for NavigationFailedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Navigation.NavigationFailedEventArgs";
}
impl ::core::convert::From<NavigationFailedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: NavigationFailedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&NavigationFailedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &NavigationFailedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for NavigationFailedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &'a NavigationFailedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<NavigationFailedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: NavigationFailedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&NavigationFailedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &NavigationFailedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for NavigationFailedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a NavigationFailedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for NavigationFailedEventArgs {}
unsafe impl ::core::marker::Sync for NavigationFailedEventArgs {}
#[doc = "*Required features: `UI_Xaml_Navigation`*"]
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct NavigationFailedEventHandler(::windows::runtime::IUnknown);
impl NavigationFailedEventHandler {
    pub fn new<
        F: FnMut(
                &::core::option::Option<::windows::runtime::IInspectable>,
                &::core::option::Option<NavigationFailedEventArgs>,
            ) -> ::windows::runtime::Result<()>
            + 'static,
    >(
        invoke: F,
    ) -> Self {
        let com = NavigationFailedEventHandler_box::<F> {
            vtable: &NavigationFailedEventHandler_box::<F>::VTABLE,
            count: ::windows::runtime::RefCount::new(1),
            invoke,
        };
        unsafe { core::mem::transmute(::windows::runtime::alloc::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `UI_Xaml_Navigation`*"]
    pub fn Invoke<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>,
        Param1: ::windows::runtime::IntoParam<'a, NavigationFailedEventArgs>,
    >(
        &self,
        sender: Param0,
        e: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).3)(
                ::core::mem::transmute_copy(this),
                sender.into_param().abi(),
                e.into_param().abi(),
            )
            .ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for NavigationFailedEventHandler {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"delegate({97ca2b56-d6eb-5fd2-a675-a339640eedba})",
    );
}
unsafe impl ::windows::runtime::Interface for NavigationFailedEventHandler {
    type Vtable = NavigationFailedEventHandler_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2546608982,
        55019,
        24530,
        [166, 117, 163, 57, 100, 14, 237, 186],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct NavigationFailedEventHandler_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        sender: ::windows::runtime::RawPtr,
        e: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(C)]
struct NavigationFailedEventHandler_box<
    F: FnMut(
            &::core::option::Option<::windows::runtime::IInspectable>,
            &::core::option::Option<NavigationFailedEventArgs>,
        ) -> ::windows::runtime::Result<()>
        + 'static,
> {
    vtable: *const NavigationFailedEventHandler_abi,
    invoke: F,
    count: ::windows::runtime::RefCount,
}
impl<
        F: FnMut(
                &::core::option::Option<::windows::runtime::IInspectable>,
                &::core::option::Option<NavigationFailedEventArgs>,
            ) -> ::windows::runtime::Result<()>
            + 'static,
    > NavigationFailedEventHandler_box<F>
{
    const VTABLE: NavigationFailedEventHandler_abi = NavigationFailedEventHandler_abi(
        Self::QueryInterface,
        Self::AddRef,
        Self::Release,
        Self::Invoke,
    );
    unsafe extern "system" fn QueryInterface(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        *interface = if iid == &<NavigationFailedEventHandler as ::windows::runtime::Interface>::IID
            || iid == &<::windows::runtime::IUnknown as ::windows::runtime::Interface>::IID
            || iid == &<::windows::runtime::IAgileObject as ::windows::runtime::Interface>::IID
        {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::core::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::runtime::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::runtime::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows::runtime::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(
        this: ::windows::runtime::RawPtr,
        sender: ::windows::runtime::RawPtr,
        e: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        ( ( * this ) . invoke ) ( & * ( & sender as * const < :: windows :: runtime :: IInspectable as :: windows :: runtime :: Abi > :: Abi as * const < :: windows :: runtime :: IInspectable as :: windows :: runtime :: DefaultType > :: DefaultType ) , & * ( & e as * const < NavigationFailedEventArgs as :: windows :: runtime :: Abi > :: Abi as * const < NavigationFailedEventArgs as :: windows :: runtime :: DefaultType > :: DefaultType ) , ) . into ( )
    }
}
#[doc = "*Required features: `UI_Xaml_Navigation`*"]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: marker :: Copy,
    :: core :: clone :: Clone,
    :: core :: default :: Default,
    :: core :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct NavigationMode(pub i32);
impl NavigationMode {
    pub const New: NavigationMode = NavigationMode(0i32);
    pub const Back: NavigationMode = NavigationMode(1i32);
    pub const Forward: NavigationMode = NavigationMode(2i32);
    pub const Refresh: NavigationMode = NavigationMode(3i32);
}
impl ::core::convert::From<i32> for NavigationMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NavigationMode {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for NavigationMode {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Navigation.NavigationMode;i4)",
    );
}
impl ::windows::runtime::DefaultType for NavigationMode {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Xaml_Navigation`*"]
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct NavigationStoppedEventHandler(::windows::runtime::IUnknown);
impl NavigationStoppedEventHandler {
    pub fn new<
        F: FnMut(
                &::core::option::Option<::windows::runtime::IInspectable>,
                &::core::option::Option<NavigationEventArgs>,
            ) -> ::windows::runtime::Result<()>
            + 'static,
    >(
        invoke: F,
    ) -> Self {
        let com = NavigationStoppedEventHandler_box::<F> {
            vtable: &NavigationStoppedEventHandler_box::<F>::VTABLE,
            count: ::windows::runtime::RefCount::new(1),
            invoke,
        };
        unsafe { core::mem::transmute(::windows::runtime::alloc::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `UI_Xaml_Navigation`*"]
    pub fn Invoke<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>,
        Param1: ::windows::runtime::IntoParam<'a, NavigationEventArgs>,
    >(
        &self,
        sender: Param0,
        e: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).3)(
                ::core::mem::transmute_copy(this),
                sender.into_param().abi(),
                e.into_param().abi(),
            )
            .ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for NavigationStoppedEventHandler {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"delegate({b9e796a6-7ffe-5a63-aef4-cbc331663b66})",
    );
}
unsafe impl ::windows::runtime::Interface for NavigationStoppedEventHandler {
    type Vtable = NavigationStoppedEventHandler_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3118962342,
        32766,
        23139,
        [174, 244, 203, 195, 49, 102, 59, 102],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct NavigationStoppedEventHandler_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        sender: ::windows::runtime::RawPtr,
        e: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(C)]
struct NavigationStoppedEventHandler_box<
    F: FnMut(
            &::core::option::Option<::windows::runtime::IInspectable>,
            &::core::option::Option<NavigationEventArgs>,
        ) -> ::windows::runtime::Result<()>
        + 'static,
> {
    vtable: *const NavigationStoppedEventHandler_abi,
    invoke: F,
    count: ::windows::runtime::RefCount,
}
impl<
        F: FnMut(
                &::core::option::Option<::windows::runtime::IInspectable>,
                &::core::option::Option<NavigationEventArgs>,
            ) -> ::windows::runtime::Result<()>
            + 'static,
    > NavigationStoppedEventHandler_box<F>
{
    const VTABLE: NavigationStoppedEventHandler_abi = NavigationStoppedEventHandler_abi(
        Self::QueryInterface,
        Self::AddRef,
        Self::Release,
        Self::Invoke,
    );
    unsafe extern "system" fn QueryInterface(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        *interface = if iid
            == &<NavigationStoppedEventHandler as ::windows::runtime::Interface>::IID
            || iid == &<::windows::runtime::IUnknown as ::windows::runtime::Interface>::IID
            || iid == &<::windows::runtime::IAgileObject as ::windows::runtime::Interface>::IID
        {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::core::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::runtime::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::runtime::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows::runtime::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(
        this: ::windows::runtime::RawPtr,
        sender: ::windows::runtime::RawPtr,
        e: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        ( ( * this ) . invoke ) ( & * ( & sender as * const < :: windows :: runtime :: IInspectable as :: windows :: runtime :: Abi > :: Abi as * const < :: windows :: runtime :: IInspectable as :: windows :: runtime :: DefaultType > :: DefaultType ) , & * ( & e as * const < NavigationEventArgs as :: windows :: runtime :: Abi > :: Abi as * const < NavigationEventArgs as :: windows :: runtime :: DefaultType > :: DefaultType ) , ) . into ( )
    }
}
#[doc = "*Required features: `UI_Xaml_Navigation`*"]
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct PageStackEntry(pub ::windows::runtime::IInspectable);
impl PageStackEntry {
    #[doc = "*Required features: `UI_Xaml_Navigation`*"]
    pub fn SourcePageType(
        &self,
    ) -> ::windows::runtime::Result<::windows::UI::Xaml::Interop::TypeName> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::UI::Xaml::Interop::TypeName> =
                ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Xaml::Interop::TypeName>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Navigation`*"]
    pub fn Parameter(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
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
    #[cfg(feature = "UI_Xaml_Media_Animation")]
    #[doc = "*Required features: `UI_Xaml_Navigation`, `UI_Xaml_Media_Animation`*"]
    pub fn NavigationTransitionInfo(
        &self,
    ) -> ::windows::runtime::Result<super::Media::Animation::NavigationTransitionInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Media::Animation::NavigationTransitionInfo>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media_Animation")]
    #[doc = "*Required features: `UI_Xaml_Navigation`, `UI_Xaml_Media_Animation`*"]
    pub fn CreateInstance<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::UI::Xaml::Interop::TypeName>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>,
        Param2: ::windows::runtime::IntoParam<'a, super::Media::Animation::NavigationTransitionInfo>,
    >(
        sourcepagetype: Param0,
        parameter: Param1,
        navigationtransitioninfo: Param2,
    ) -> ::windows::runtime::Result<PageStackEntry> {
        Self::IPageStackEntryFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                sourcepagetype.into_param().abi(),
                parameter.into_param().abi(),
                navigationtransitioninfo.into_param().abi(),
                &mut result__,
            )
            .from_abi::<PageStackEntry>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Navigation`*"]
    pub fn SourcePageTypeProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::IPageStackEntryStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Navigation`*"]
    pub fn GetValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Navigation`*"]
    pub fn SetValue<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>,
    >(
        &self,
        dp: Param0,
        value: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Navigation`*"]
    pub fn ClearValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Navigation`*"]
    pub fn ReadLocalValue<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Navigation`*"]
    pub fn GetAnimationBaseValue<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Navigation`*"]
    pub fn RegisterPropertyChangedCallback<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::runtime::IntoParam<'a, super::DependencyPropertyChangedCallback>,
    >(
        &self,
        dp: Param0,
        callback: Param1,
    ) -> ::windows::runtime::Result<i64> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                callback.into_param().abi(),
                &mut result__,
            )
            .from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Navigation`*"]
    pub fn UnregisterPropertyChangedCallback<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
        token: i64,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                token,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Navigation`*"]
    pub fn Dispatcher(&self) -> ::windows::runtime::Result<::windows::UI::Core::CoreDispatcher> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Core::CoreDispatcher>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    #[doc = "*Required features: `UI_Xaml_Navigation`, `UI_Dispatching`*"]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::runtime::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    pub fn IPageStackEntryFactory<
        R,
        F: FnOnce(&IPageStackEntryFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            PageStackEntry,
            IPageStackEntryFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IPageStackEntryStatics<
        R,
        F: FnOnce(&IPageStackEntryStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            PageStackEntry,
            IPageStackEntryStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PageStackEntry {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Navigation.PageStackEntry;{d591f56e-4262-5c91-9d79-29165cd82100})",
    );
}
unsafe impl ::windows::runtime::Interface for PageStackEntry {
    type Vtable = IPageStackEntry_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3583112558,
        16994,
        23697,
        [157, 121, 41, 22, 92, 216, 33, 0],
    );
}
impl ::windows::runtime::RuntimeName for PageStackEntry {
    const NAME: &'static str = "Microsoft.UI.Xaml.Navigation.PageStackEntry";
}
impl ::core::convert::From<PageStackEntry> for ::windows::runtime::IUnknown {
    fn from(value: PageStackEntry) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PageStackEntry> for ::windows::runtime::IUnknown {
    fn from(value: &PageStackEntry) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PageStackEntry {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a PageStackEntry {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PageStackEntry> for ::windows::runtime::IInspectable {
    fn from(value: PageStackEntry) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PageStackEntry> for ::windows::runtime::IInspectable {
    fn from(value: &PageStackEntry) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PageStackEntry {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a PageStackEntry
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<PageStackEntry> for super::DependencyObject {
    fn from(value: PageStackEntry) -> Self {
        ::core::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&PageStackEntry> for super::DependencyObject {
    fn from(value: &PageStackEntry) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::DependencyObject> for PageStackEntry {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::DependencyObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(
            self,
        ))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::DependencyObject> for &PageStackEntry {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::DependencyObject> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::core::marker::Send for PageStackEntry {}
unsafe impl ::core::marker::Sync for PageStackEntry {}
