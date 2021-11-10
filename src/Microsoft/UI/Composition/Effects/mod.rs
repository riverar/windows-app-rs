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
pub struct ISceneLightingEffect(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISceneLightingEffect {
    type Vtable = ISceneLightingEffect_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3944641302,
        4428,
        22864,
        [132, 128, 32, 162, 154, 59, 177, 238],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneLightingEffect_abi(
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
        result__: *mut f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: f32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISceneLightingEffect2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISceneLightingEffect2 {
    type Vtable = ISceneLightingEffect2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1801754290,
        18061,
        20689,
        [187, 233, 89, 59, 130, 99, 173, 128],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneLightingEffect2_abi(
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
        result__: *mut SceneLightingEffectReflectanceModel,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: SceneLightingEffectReflectanceModel,
    ) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `UI_Composition_Effects`*"]
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct SceneLightingEffect(pub ::windows::runtime::IInspectable);
impl SceneLightingEffect {
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
            SceneLightingEffect,
            ::windows::runtime::IActivationFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `UI_Composition_Effects`*"]
    pub fn AmbientAmount(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Effects`*"]
    pub fn SetAmbientAmount(&self, value: f32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Effects`*"]
    pub fn DiffuseAmount(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Effects`*"]
    pub fn SetDiffuseAmount(&self, value: f32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Effects`*"]
    pub fn NormalMapSource(
        &self,
    ) -> ::windows::runtime::Result<::windows::Graphics::Effects::IGraphicsEffectSource> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Graphics::Effects::IGraphicsEffectSource>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Effects`*"]
    pub fn SetNormalMapSource<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Graphics::Effects::IGraphicsEffectSource>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Effects`*"]
    pub fn SpecularAmount(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Effects`*"]
    pub fn SetSpecularAmount(&self, value: f32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Effects`*"]
    pub fn SpecularShine(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Effects`*"]
    pub fn SetSpecularShine(&self, value: f32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).15)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Effects`*"]
    pub fn ReflectanceModel(
        &self,
    ) -> ::windows::runtime::Result<SceneLightingEffectReflectanceModel> {
        let this = &::windows::runtime::Interface::cast::<ISceneLightingEffect2>(self)?;
        unsafe {
            let mut result__: SceneLightingEffectReflectanceModel = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<SceneLightingEffectReflectanceModel>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Effects`*"]
    pub fn SetReflectanceModel(
        &self,
        value: SceneLightingEffectReflectanceModel,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISceneLightingEffect2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Effects`*"]
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<
            ::windows::Graphics::Effects::IGraphicsEffect,
        >(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Effects`*"]
    pub fn SetName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        &self,
        name: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<
            ::windows::Graphics::Effects::IGraphicsEffect,
        >(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                name.into_param().abi(),
            )
            .ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SceneLightingEffect {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Composition.Effects.SceneLightingEffect;{eb1e7316-114c-5950-8480-20a29a3bb1ee})" ) ;
}
unsafe impl ::windows::runtime::Interface for SceneLightingEffect {
    type Vtable = ISceneLightingEffect_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3944641302,
        4428,
        22864,
        [132, 128, 32, 162, 154, 59, 177, 238],
    );
}
impl ::windows::runtime::RuntimeName for SceneLightingEffect {
    const NAME: &'static str = "Microsoft.UI.Composition.Effects.SceneLightingEffect";
}
impl ::core::convert::From<SceneLightingEffect> for ::windows::runtime::IUnknown {
    fn from(value: SceneLightingEffect) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SceneLightingEffect> for ::windows::runtime::IUnknown {
    fn from(value: &SceneLightingEffect) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SceneLightingEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &'a SceneLightingEffect
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SceneLightingEffect> for ::windows::runtime::IInspectable {
    fn from(value: SceneLightingEffect) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SceneLightingEffect> for ::windows::runtime::IInspectable {
    fn from(value: &SceneLightingEffect) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for SceneLightingEffect
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a SceneLightingEffect
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<SceneLightingEffect>
    for ::windows::Graphics::Effects::IGraphicsEffect
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: SceneLightingEffect) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SceneLightingEffect>
    for ::windows::Graphics::Effects::IGraphicsEffect
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SceneLightingEffect) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::Graphics::Effects::IGraphicsEffect>
    for SceneLightingEffect
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, ::windows::Graphics::Effects::IGraphicsEffect> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::Graphics::Effects::IGraphicsEffect>
    for &SceneLightingEffect
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, ::windows::Graphics::Effects::IGraphicsEffect> {
        ::core::convert::TryInto::<::windows::Graphics::Effects::IGraphicsEffect>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<SceneLightingEffect>
    for ::windows::Graphics::Effects::IGraphicsEffectSource
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: SceneLightingEffect) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SceneLightingEffect>
    for ::windows::Graphics::Effects::IGraphicsEffectSource
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SceneLightingEffect) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::Graphics::Effects::IGraphicsEffectSource>
    for SceneLightingEffect
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, ::windows::Graphics::Effects::IGraphicsEffectSource> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::Graphics::Effects::IGraphicsEffectSource>
    for &SceneLightingEffect
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, ::windows::Graphics::Effects::IGraphicsEffectSource> {
        ::core::convert::TryInto::<::windows::Graphics::Effects::IGraphicsEffectSource>::try_into(
            self,
        )
        .map(::windows::runtime::Param::Owned)
        .unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::core::marker::Send for SceneLightingEffect {}
unsafe impl ::core::marker::Sync for SceneLightingEffect {}
#[doc = "*Required features: `UI_Composition_Effects`*"]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: marker :: Copy,
    :: core :: clone :: Clone,
    :: core :: default :: Default,
    :: core :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct SceneLightingEffectReflectanceModel(pub i32);
impl SceneLightingEffectReflectanceModel {
    pub const BlinnPhong: SceneLightingEffectReflectanceModel =
        SceneLightingEffectReflectanceModel(0i32);
    pub const PhysicallyBasedBlinnPhong: SceneLightingEffectReflectanceModel =
        SceneLightingEffectReflectanceModel(1i32);
}
impl ::core::convert::From<i32> for SceneLightingEffectReflectanceModel {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SceneLightingEffectReflectanceModel {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SceneLightingEffectReflectanceModel {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Composition.Effects.SceneLightingEffectReflectanceModel;i4)",
    );
}
impl ::windows::runtime::DefaultType for SceneLightingEffectReflectanceModel {
    type DefaultType = Self;
}
