#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[doc = "*Required features: `Windows_ApplicationModel_DynamicDependency`*"]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct AddPackageDependencyOptions(pub ::windows::runtime::IInspectable);
impl AddPackageDependencyOptions {
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
            AddPackageDependencyOptions,
            ::windows::runtime::IActivationFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Windows_ApplicationModel_DynamicDependency`*"]
    pub fn Rank(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Windows_ApplicationModel_DynamicDependency`*"]
    pub fn SetRank(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[doc = "*Required features: `Windows_ApplicationModel_DynamicDependency`*"]
    pub fn PrependIfRankCollision(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Windows_ApplicationModel_DynamicDependency`*"]
    pub fn SetPrependIfRankCollision(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AddPackageDependencyOptions {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.Windows.ApplicationModel.DynamicDependency.AddPackageDependencyOptions;{01b801fd-24e3-5e6b-9f1c-805ab410b604})" ) ;
}
unsafe impl ::windows::runtime::Interface for AddPackageDependencyOptions {
    type Vtable = IAddPackageDependencyOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        28836349,
        9443,
        24171,
        [159, 28, 128, 90, 180, 16, 182, 4],
    );
}
impl ::windows::runtime::RuntimeName for AddPackageDependencyOptions {
    const NAME: &'static str =
        "Microsoft.Windows.ApplicationModel.DynamicDependency.AddPackageDependencyOptions";
}
impl ::std::convert::From<AddPackageDependencyOptions> for ::windows::runtime::IUnknown {
    fn from(value: AddPackageDependencyOptions) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&AddPackageDependencyOptions> for ::windows::runtime::IUnknown {
    fn from(value: &AddPackageDependencyOptions) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for AddPackageDependencyOptions
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &'a AddPackageDependencyOptions
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<AddPackageDependencyOptions> for ::windows::runtime::IInspectable {
    fn from(value: AddPackageDependencyOptions) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AddPackageDependencyOptions> for ::windows::runtime::IInspectable {
    fn from(value: &AddPackageDependencyOptions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for AddPackageDependencyOptions
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a AddPackageDependencyOptions
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for AddPackageDependencyOptions {}
unsafe impl ::std::marker::Sync for AddPackageDependencyOptions {}
#[doc = "*Required features: `Windows_ApplicationModel_DynamicDependency`*"]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct CreatePackageDependencyOptions(pub ::windows::runtime::IInspectable);
impl CreatePackageDependencyOptions {
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
            CreatePackageDependencyOptions,
            ::windows::runtime::IActivationFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Windows_ApplicationModel_DynamicDependency`*"]
    pub fn Architectures(
        &self,
    ) -> ::windows::runtime::Result<PackageDependencyProcessorArchitectures> {
        let this = self;
        unsafe {
            let mut result__: PackageDependencyProcessorArchitectures = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<PackageDependencyProcessorArchitectures>(result__)
        }
    }
    #[doc = "*Required features: `Windows_ApplicationModel_DynamicDependency`*"]
    pub fn SetArchitectures(
        &self,
        value: PackageDependencyProcessorArchitectures,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[doc = "*Required features: `Windows_ApplicationModel_DynamicDependency`*"]
    pub fn VerifyDependencyResolution(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Windows_ApplicationModel_DynamicDependency`*"]
    pub fn SetVerifyDependencyResolution(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[doc = "*Required features: `Windows_ApplicationModel_DynamicDependency`*"]
    pub fn LifetimeArtifactKind(
        &self,
    ) -> ::windows::runtime::Result<PackageDependencyLifetimeArtifactKind> {
        let this = self;
        unsafe {
            let mut result__: PackageDependencyLifetimeArtifactKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<PackageDependencyLifetimeArtifactKind>(result__)
        }
    }
    #[doc = "*Required features: `Windows_ApplicationModel_DynamicDependency`*"]
    pub fn SetLifetimeArtifactKind(
        &self,
        value: PackageDependencyLifetimeArtifactKind,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `Windows_ApplicationModel_DynamicDependency`*"]
    pub fn LifetimeArtifact(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Windows_ApplicationModel_DynamicDependency`*"]
    pub fn SetLifetimeArtifact<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CreatePackageDependencyOptions {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.Windows.ApplicationModel.DynamicDependency.CreatePackageDependencyOptions;{cdbb820f-3c69-55dc-a017-b4132574c5d6})" ) ;
}
unsafe impl ::windows::runtime::Interface for CreatePackageDependencyOptions {
    type Vtable = ICreatePackageDependencyOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3451617807,
        15465,
        21980,
        [160, 23, 180, 19, 37, 116, 197, 214],
    );
}
impl ::windows::runtime::RuntimeName for CreatePackageDependencyOptions {
    const NAME: &'static str =
        "Microsoft.Windows.ApplicationModel.DynamicDependency.CreatePackageDependencyOptions";
}
impl ::std::convert::From<CreatePackageDependencyOptions> for ::windows::runtime::IUnknown {
    fn from(value: CreatePackageDependencyOptions) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&CreatePackageDependencyOptions> for ::windows::runtime::IUnknown {
    fn from(value: &CreatePackageDependencyOptions) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for CreatePackageDependencyOptions
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &'a CreatePackageDependencyOptions
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<CreatePackageDependencyOptions> for ::windows::runtime::IInspectable {
    fn from(value: CreatePackageDependencyOptions) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CreatePackageDependencyOptions> for ::windows::runtime::IInspectable {
    fn from(value: &CreatePackageDependencyOptions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for CreatePackageDependencyOptions
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a CreatePackageDependencyOptions
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for CreatePackageDependencyOptions {}
unsafe impl ::std::marker::Sync for CreatePackageDependencyOptions {}
#[repr(transparent)]
#[doc(hidden)]
pub struct IAddPackageDependencyOptions(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAddPackageDependencyOptions {
    type Vtable = IAddPackageDependencyOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        28836349,
        9443,
        24171,
        [159, 28, 128, 90, 180, 16, 182, 4],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IAddPackageDependencyOptions_abi(
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
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: i32,
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
#[repr(transparent)]
#[doc(hidden)]
pub struct ICreatePackageDependencyOptions(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICreatePackageDependencyOptions {
    type Vtable = ICreatePackageDependencyOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3451617807,
        15465,
        21980,
        [160, 23, 180, 19, 37, 116, 197, 214],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreatePackageDependencyOptions_abi(
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
        result__: *mut PackageDependencyProcessorArchitectures,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: PackageDependencyProcessorArchitectures,
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
        result__: *mut PackageDependencyLifetimeArtifactKind,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: PackageDependencyLifetimeArtifactKind,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPackageDependency(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPackageDependency {
    type Vtable = IPackageDependency_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        850295701,
        58200,
        23112,
        [150, 105, 201, 125, 133, 173, 101, 86],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageDependency_abi(
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
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        options: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPackageDependencyContext(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPackageDependencyContext {
    type Vtable = IPackageDependencyContext_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2567095130,
        41973,
        22085,
        [175, 15, 205, 249, 252, 160, 13, 94],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageDependencyContext_abi(
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
        result__: *mut PackageDependencyContextId,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPackageDependencyContextFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPackageDependencyContextFactory {
    type Vtable = IPackageDependencyContextFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2568286799,
        48831,
        20843,
        [173, 171, 92, 62, 139, 243, 35, 248],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageDependencyContextFactory_abi(
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
        contextid: PackageDependencyContextId,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPackageDependencyRankStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPackageDependencyRankStatics {
    type Vtable = IPackageDependencyRankStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        637895613,
        42155,
        21501,
        [161, 144, 196, 70, 191, 219, 83, 132],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageDependencyRankStatics_abi(
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
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPackageDependencyStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPackageDependencyStatics {
    type Vtable = IPackageDependencyStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        397825761,
        6744,
        24380,
        [132, 168, 68, 48, 246, 231, 73, 194],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageDependencyStatics_abi(
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
        id: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        id: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        packagefamilyname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        minversion: ::windows::ApplicationModel::PackageVersion,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        packagefamilyname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        minversion: ::windows::ApplicationModel::PackageVersion,
        options: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        packagefamilyname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        minversion: ::windows::ApplicationModel::PackageVersion,
        options: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut u32,
    ) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Windows_ApplicationModel_DynamicDependency`*"]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct PackageDependency(pub ::windows::runtime::IInspectable);
impl PackageDependency {
    #[doc = "*Required features: `Windows_ApplicationModel_DynamicDependency`*"]
    pub fn Id(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Windows_ApplicationModel_DynamicDependency`*"]
    pub fn Delete(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this)).ok()
        }
    }
    #[doc = "*Required features: `Windows_ApplicationModel_DynamicDependency`*"]
    pub fn Add(&self) -> ::windows::runtime::Result<PackageDependencyContext> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<PackageDependencyContext>(result__)
        }
    }
    #[doc = "*Required features: `Windows_ApplicationModel_DynamicDependency`*"]
    pub fn Add2<'a, Param0: ::windows::runtime::IntoParam<'a, AddPackageDependencyOptions>>(
        &self,
        options: Param0,
    ) -> ::windows::runtime::Result<PackageDependencyContext> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                options.into_param().abi(),
                &mut result__,
            )
            .from_abi::<PackageDependencyContext>(result__)
        }
    }
    #[doc = "*Required features: `Windows_ApplicationModel_DynamicDependency`*"]
    pub fn GetFromId<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        id: Param0,
    ) -> ::windows::runtime::Result<PackageDependency> {
        Self::IPackageDependencyStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                id.into_param().abi(),
                &mut result__,
            )
            .from_abi::<PackageDependency>(result__)
        })
    }
    #[doc = "*Required features: `Windows_ApplicationModel_DynamicDependency`*"]
    pub fn GetFromIdForSystem<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        id: Param0,
    ) -> ::windows::runtime::Result<PackageDependency> {
        Self::IPackageDependencyStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                id.into_param().abi(),
                &mut result__,
            )
            .from_abi::<PackageDependency>(result__)
        })
    }
    #[doc = "*Required features: `Windows_ApplicationModel_DynamicDependency`*"]
    pub fn Create<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::ApplicationModel::PackageVersion>,
    >(
        packagefamilyname: Param0,
        minversion: Param1,
    ) -> ::windows::runtime::Result<PackageDependency> {
        Self::IPackageDependencyStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                packagefamilyname.into_param().abi(),
                minversion.into_param().abi(),
                &mut result__,
            )
            .from_abi::<PackageDependency>(result__)
        })
    }
    #[doc = "*Required features: `Windows_ApplicationModel_DynamicDependency`*"]
    pub fn Create2<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::ApplicationModel::PackageVersion>,
        Param2: ::windows::runtime::IntoParam<'a, CreatePackageDependencyOptions>,
    >(
        packagefamilyname: Param0,
        minversion: Param1,
        options: Param2,
    ) -> ::windows::runtime::Result<PackageDependency> {
        Self::IPackageDependencyStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                packagefamilyname.into_param().abi(),
                minversion.into_param().abi(),
                options.into_param().abi(),
                &mut result__,
            )
            .from_abi::<PackageDependency>(result__)
        })
    }
    #[doc = "*Required features: `Windows_ApplicationModel_DynamicDependency`*"]
    pub fn CreateForSystem<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::ApplicationModel::PackageVersion>,
        Param2: ::windows::runtime::IntoParam<'a, CreatePackageDependencyOptions>,
    >(
        packagefamilyname: Param0,
        minversion: Param1,
        options: Param2,
    ) -> ::windows::runtime::Result<PackageDependency> {
        Self::IPackageDependencyStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                packagefamilyname.into_param().abi(),
                minversion.into_param().abi(),
                options.into_param().abi(),
                &mut result__,
            )
            .from_abi::<PackageDependency>(result__)
        })
    }
    #[doc = "*Required features: `Windows_ApplicationModel_DynamicDependency`*"]
    pub fn GenerationId() -> ::windows::runtime::Result<u32> {
        Self::IPackageDependencyStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        })
    }
    pub fn IPackageDependencyStatics<
        R,
        F: FnOnce(&IPackageDependencyStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            PackageDependency,
            IPackageDependencyStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PackageDependency {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.Windows.ApplicationModel.DynamicDependency.PackageDependency;{32ae7b95-e358-5a48-9669-c97d85ad6556})" ) ;
}
unsafe impl ::windows::runtime::Interface for PackageDependency {
    type Vtable = IPackageDependency_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        850295701,
        58200,
        23112,
        [150, 105, 201, 125, 133, 173, 101, 86],
    );
}
impl ::windows::runtime::RuntimeName for PackageDependency {
    const NAME: &'static str =
        "Microsoft.Windows.ApplicationModel.DynamicDependency.PackageDependency";
}
impl ::std::convert::From<PackageDependency> for ::windows::runtime::IUnknown {
    fn from(value: PackageDependency) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&PackageDependency> for ::windows::runtime::IUnknown {
    fn from(value: &PackageDependency) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PackageDependency {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a PackageDependency {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<PackageDependency> for ::windows::runtime::IInspectable {
    fn from(value: PackageDependency) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PackageDependency> for ::windows::runtime::IInspectable {
    fn from(value: &PackageDependency) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PackageDependency {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a PackageDependency
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for PackageDependency {}
unsafe impl ::std::marker::Sync for PackageDependency {}
#[doc = "*Required features: `Windows_ApplicationModel_DynamicDependency`*"]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct PackageDependencyContext(pub ::windows::runtime::IInspectable);
impl PackageDependencyContext {
    #[doc = "*Required features: `Windows_ApplicationModel_DynamicDependency`*"]
    pub fn ContextId(&self) -> ::windows::runtime::Result<PackageDependencyContextId> {
        let this = self;
        unsafe {
            let mut result__: PackageDependencyContextId = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<PackageDependencyContextId>(result__)
        }
    }
    #[doc = "*Required features: `Windows_ApplicationModel_DynamicDependency`*"]
    pub fn PackageDependencyId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Windows_ApplicationModel_DynamicDependency`*"]
    pub fn PackageFullName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Windows_ApplicationModel_DynamicDependency`*"]
    pub fn Remove(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this)).ok()
        }
    }
    #[doc = "*Required features: `Windows_ApplicationModel_DynamicDependency`*"]
    pub fn CreateInstance<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, PackageDependencyContextId>,
    >(
        contextid: Param0,
    ) -> ::windows::runtime::Result<PackageDependencyContext> {
        Self::IPackageDependencyContextFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                contextid.into_param().abi(),
                &mut result__,
            )
            .from_abi::<PackageDependencyContext>(result__)
        })
    }
    pub fn IPackageDependencyContextFactory<
        R,
        F: FnOnce(&IPackageDependencyContextFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            PackageDependencyContext,
            IPackageDependencyContextFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PackageDependencyContext {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.Windows.ApplicationModel.DynamicDependency.PackageDependencyContext;{9902c35a-a3f5-5645-af0f-cdf9fca00d5e})" ) ;
}
unsafe impl ::windows::runtime::Interface for PackageDependencyContext {
    type Vtable = IPackageDependencyContext_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2567095130,
        41973,
        22085,
        [175, 15, 205, 249, 252, 160, 13, 94],
    );
}
impl ::windows::runtime::RuntimeName for PackageDependencyContext {
    const NAME: &'static str =
        "Microsoft.Windows.ApplicationModel.DynamicDependency.PackageDependencyContext";
}
impl ::std::convert::From<PackageDependencyContext> for ::windows::runtime::IUnknown {
    fn from(value: PackageDependencyContext) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&PackageDependencyContext> for ::windows::runtime::IUnknown {
    fn from(value: &PackageDependencyContext) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for PackageDependencyContext
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &'a PackageDependencyContext
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<PackageDependencyContext> for ::windows::runtime::IInspectable {
    fn from(value: PackageDependencyContext) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PackageDependencyContext> for ::windows::runtime::IInspectable {
    fn from(value: &PackageDependencyContext) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for PackageDependencyContext
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a PackageDependencyContext
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for PackageDependencyContext {}
unsafe impl ::std::marker::Sync for PackageDependencyContext {}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Windows_ApplicationModel_DynamicDependency`*"]
pub struct PackageDependencyContextId {
    pub Id: u64,
}
impl PackageDependencyContextId {}
impl ::std::default::Default for PackageDependencyContextId {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PackageDependencyContextId {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PackageDependencyContextId")
            .field("Id", &self.Id)
            .finish()
    }
}
impl ::std::cmp::PartialEq for PackageDependencyContextId {
    fn eq(&self, other: &Self) -> bool {
        self.Id == other.Id
    }
}
impl ::std::cmp::Eq for PackageDependencyContextId {}
unsafe impl ::windows::runtime::Abi for PackageDependencyContextId {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PackageDependencyContextId {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"struct(Microsoft.Windows.ApplicationModel.DynamicDependency.PackageDependencyContextId;u8)" ) ;
}
impl ::windows::runtime::DefaultType for PackageDependencyContextId {
    type DefaultType = Self;
}
#[doc = "*Required features: `Windows_ApplicationModel_DynamicDependency`*"]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct PackageDependencyLifetimeArtifactKind(pub i32);
impl PackageDependencyLifetimeArtifactKind {
    pub const Process: PackageDependencyLifetimeArtifactKind =
        PackageDependencyLifetimeArtifactKind(0i32);
    pub const FilePath: PackageDependencyLifetimeArtifactKind =
        PackageDependencyLifetimeArtifactKind(1i32);
    pub const RegistryKey: PackageDependencyLifetimeArtifactKind =
        PackageDependencyLifetimeArtifactKind(2i32);
}
impl ::std::convert::From<i32> for PackageDependencyLifetimeArtifactKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PackageDependencyLifetimeArtifactKind {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PackageDependencyLifetimeArtifactKind {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"enum(Microsoft.Windows.ApplicationModel.DynamicDependency.PackageDependencyLifetimeArtifactKind;i4)" ) ;
}
impl ::windows::runtime::DefaultType for PackageDependencyLifetimeArtifactKind {
    type DefaultType = Self;
}
#[doc = "*Required features: `Windows_ApplicationModel_DynamicDependency`*"]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct PackageDependencyProcessorArchitectures(pub u32);
impl PackageDependencyProcessorArchitectures {
    pub const None: PackageDependencyProcessorArchitectures =
        PackageDependencyProcessorArchitectures(0u32);
    pub const Neutral: PackageDependencyProcessorArchitectures =
        PackageDependencyProcessorArchitectures(1u32);
    pub const X86: PackageDependencyProcessorArchitectures =
        PackageDependencyProcessorArchitectures(2u32);
    pub const X64: PackageDependencyProcessorArchitectures =
        PackageDependencyProcessorArchitectures(4u32);
    pub const Arm: PackageDependencyProcessorArchitectures =
        PackageDependencyProcessorArchitectures(8u32);
    pub const Arm64: PackageDependencyProcessorArchitectures =
        PackageDependencyProcessorArchitectures(16u32);
    pub const X86OnArm64: PackageDependencyProcessorArchitectures =
        PackageDependencyProcessorArchitectures(32u32);
}
impl ::std::convert::From<u32> for PackageDependencyProcessorArchitectures {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PackageDependencyProcessorArchitectures {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PackageDependencyProcessorArchitectures {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"enum(Microsoft.Windows.ApplicationModel.DynamicDependency.PackageDependencyProcessorArchitectures;u4)" ) ;
}
impl ::windows::runtime::DefaultType for PackageDependencyProcessorArchitectures {
    type DefaultType = Self;
}
impl ::std::ops::BitOr for PackageDependencyProcessorArchitectures {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for PackageDependencyProcessorArchitectures {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for PackageDependencyProcessorArchitectures {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for PackageDependencyProcessorArchitectures {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for PackageDependencyProcessorArchitectures {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Windows_ApplicationModel_DynamicDependency`*"]
pub struct PackageDependencyRank {}
impl PackageDependencyRank {
    #[doc = "*Required features: `Windows_ApplicationModel_DynamicDependency`*"]
    pub fn Default() -> ::windows::runtime::Result<i32> {
        Self::IPackageDependencyRankStatics(|this| unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        })
    }
    pub fn IPackageDependencyRankStatics<
        R,
        F: FnOnce(&IPackageDependencyRankStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            PackageDependencyRank,
            IPackageDependencyRankStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for PackageDependencyRank {
    const NAME: &'static str =
        "Microsoft.Windows.ApplicationModel.DynamicDependency.PackageDependencyRank";
}
