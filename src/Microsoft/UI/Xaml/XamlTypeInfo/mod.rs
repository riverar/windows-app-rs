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
pub struct IXamlControlsXamlMetaDataProvider(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IXamlControlsXamlMetaDataProvider {
    type Vtable = IXamlControlsXamlMetaDataProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        402276184,
        13426,
        23202,
        [160, 248, 26, 184, 165, 25, 87, 61],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlControlsXamlMetaDataProvider_abi(
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
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IXamlControlsXamlMetaDataProviderStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IXamlControlsXamlMetaDataProviderStatics {
    type Vtable = IXamlControlsXamlMetaDataProviderStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        763278333,
        60635,
        20612,
        [183, 224, 18, 249, 89, 131, 129, 239],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlControlsXamlMetaDataProviderStatics_abi(
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
);
#[cfg(feature = "UI_Xaml_Markup")]
#[doc = "*Required features: `UI_Xaml_XamlTypeInfo`, `UI_Xaml_Markup`*"]
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct XamlControlsXamlMetaDataProvider(pub ::windows::runtime::IInspectable);
#[cfg(feature = "UI_Xaml_Markup")]
impl XamlControlsXamlMetaDataProvider {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            XamlControlsXamlMetaDataProvider,
            ::windows::runtime::IActivationFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `UI_Xaml_XamlTypeInfo`*"]
    pub fn Initialize() -> ::windows::runtime::Result<()> {
        Self::IXamlControlsXamlMetaDataProviderStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok()
        })
    }
    #[cfg(feature = "UI_Xaml_Markup")]
    #[doc = "*Required features: `UI_Xaml_XamlTypeInfo`, `UI_Xaml_Markup`*"]
    pub fn GetXamlType<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::UI::Xaml::Interop::TypeName>,
    >(
        &self,
        r#type: Param0,
    ) -> ::windows::runtime::Result<super::Markup::IXamlType> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                r#type.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::Markup::IXamlType>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Markup")]
    #[doc = "*Required features: `UI_Xaml_XamlTypeInfo`, `UI_Xaml_Markup`*"]
    pub fn GetXamlTypeByFullName<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        fullname: Param0,
    ) -> ::windows::runtime::Result<super::Markup::IXamlType> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                fullname.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::Markup::IXamlType>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Markup")]
    #[doc = "*Required features: `UI_Xaml_XamlTypeInfo`, `UI_Xaml_Markup`*"]
    pub fn GetXmlnsDefinitions(
        &self,
    ) -> ::windows::runtime::Result<::windows::runtime::Array<super::Markup::XmlnsDefinition>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::Array<super::Markup::XmlnsDefinition> =
                ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                ::windows::runtime::Array::<super::Markup::XmlnsDefinition>::set_abi_len(
                    &mut result__,
                ),
                &mut result__ as *mut _ as _,
            )
            .and_then(|| result__)
        }
    }
    pub fn IXamlControlsXamlMetaDataProviderStatics<
        R,
        F: FnOnce(&IXamlControlsXamlMetaDataProviderStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            XamlControlsXamlMetaDataProvider,
            IXamlControlsXamlMetaDataProviderStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
#[cfg(feature = "UI_Xaml_Markup")]
unsafe impl ::windows::runtime::RuntimeType for XamlControlsXamlMetaDataProvider {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.XamlTypeInfo.XamlControlsXamlMetaDataProvider;{a96251f0-2214-5d53-8746-ce99a2593cd7})" ) ;
}
#[cfg(feature = "UI_Xaml_Markup")]
unsafe impl ::windows::runtime::Interface for XamlControlsXamlMetaDataProvider {
    type Vtable = super::Markup::IXamlMetadataProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2841793008,
        8724,
        23891,
        [135, 70, 206, 153, 162, 89, 60, 215],
    );
}
#[cfg(feature = "UI_Xaml_Markup")]
impl ::windows::runtime::RuntimeName for XamlControlsXamlMetaDataProvider {
    const NAME: &'static str = "Microsoft.UI.Xaml.XamlTypeInfo.XamlControlsXamlMetaDataProvider";
}
#[cfg(feature = "UI_Xaml_Markup")]
impl ::core::convert::From<XamlControlsXamlMetaDataProvider> for ::windows::runtime::IUnknown {
    fn from(value: XamlControlsXamlMetaDataProvider) -> Self {
        value.0 .0
    }
}
#[cfg(feature = "UI_Xaml_Markup")]
impl ::core::convert::From<&XamlControlsXamlMetaDataProvider> for ::windows::runtime::IUnknown {
    fn from(value: &XamlControlsXamlMetaDataProvider) -> Self {
        value.0 .0.clone()
    }
}
#[cfg(feature = "UI_Xaml_Markup")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for XamlControlsXamlMetaDataProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
#[cfg(feature = "UI_Xaml_Markup")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &'a XamlControlsXamlMetaDataProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
#[cfg(feature = "UI_Xaml_Markup")]
impl ::core::convert::From<XamlControlsXamlMetaDataProvider> for ::windows::runtime::IInspectable {
    fn from(value: XamlControlsXamlMetaDataProvider) -> Self {
        value.0
    }
}
#[cfg(feature = "UI_Xaml_Markup")]
impl ::core::convert::From<&XamlControlsXamlMetaDataProvider> for ::windows::runtime::IInspectable {
    fn from(value: &XamlControlsXamlMetaDataProvider) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "UI_Xaml_Markup")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for XamlControlsXamlMetaDataProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
#[cfg(feature = "UI_Xaml_Markup")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a XamlControlsXamlMetaDataProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "UI_Xaml_Markup")]
impl ::core::convert::From<XamlControlsXamlMetaDataProvider>
    for super::Markup::IXamlMetadataProvider
{
    fn from(value: XamlControlsXamlMetaDataProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "UI_Xaml_Markup")]
impl ::core::convert::From<&XamlControlsXamlMetaDataProvider>
    for super::Markup::IXamlMetadataProvider
{
    fn from(value: &XamlControlsXamlMetaDataProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "UI_Xaml_Markup")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Markup::IXamlMetadataProvider>
    for XamlControlsXamlMetaDataProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Markup::IXamlMetadataProvider> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "UI_Xaml_Markup")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Markup::IXamlMetadataProvider>
    for &XamlControlsXamlMetaDataProvider
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Markup::IXamlMetadataProvider> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "UI_Xaml_Markup")]
unsafe impl ::core::marker::Send for XamlControlsXamlMetaDataProvider {}
#[cfg(feature = "UI_Xaml_Markup")]
unsafe impl ::core::marker::Sync for XamlControlsXamlMetaDataProvider {}
