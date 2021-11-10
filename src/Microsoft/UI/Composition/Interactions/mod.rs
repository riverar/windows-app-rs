#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[doc = "*Required features: `UI_Composition_Interactions`*"]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct CompositionConditionalValue(pub ::windows::runtime::IInspectable);
impl CompositionConditionalValue {
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Condition(&self) -> ::windows::runtime::Result<super::ExpressionAnimation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ExpressionAnimation>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn SetCondition<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ExpressionAnimation>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Value(&self) -> ::windows::runtime::Result<super::ExpressionAnimation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ExpressionAnimation>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn SetValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::ExpressionAnimation>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, super::Compositor>>(
        compositor: Param0,
    ) -> ::windows::runtime::Result<CompositionConditionalValue> {
        Self::ICompositionConditionalValueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                compositor.into_param().abi(),
                &mut result__,
            )
            .from_abi::<CompositionConditionalValue>(result__)
        })
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn PopulatePropertyInfo<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, super::AnimationPropertyInfo>,
    >(
        &self,
        propertyname: Param0,
        propertyinfo: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                propertyinfo.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Compositor(&self) -> ::windows::runtime::Result<super::Compositor> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Compositor>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<super::CompositionPropertySet> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn StartAnimation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, super::CompositionAnimation>,
    >(
        &self,
        propertyname: Param0,
        animation: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                animation.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn StopAnimation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        propertyname: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Comment(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
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
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn SetComment<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn ImplicitAnimations(
        &self,
    ) -> ::windows::runtime::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn SetImplicitAnimations<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ImplicitAnimationCollection>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn StartAnimationGroup<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn StopAnimationGroup<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    #[doc = "*Required features: `UI_Composition_Interactions`, `UI_Dispatching`*"]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::runtime::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn TryGetAnimationController<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        propertyname: Param0,
    ) -> ::windows::runtime::Result<super::AnimationController> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::AnimationController>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn ICompositionConditionalValueStatics<
        R,
        F: FnOnce(&ICompositionConditionalValueStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            CompositionConditionalValue,
            ICompositionConditionalValueStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CompositionConditionalValue {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Composition.Interactions.CompositionConditionalValue;{3743dda0-fbe2-5ecf-9e80-4638a011f707})" ) ;
}
unsafe impl ::windows::runtime::Interface for CompositionConditionalValue {
    type Vtable = ICompositionConditionalValue_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        927194528,
        64482,
        24271,
        [158, 128, 70, 56, 160, 17, 247, 7],
    );
}
impl ::windows::runtime::RuntimeName for CompositionConditionalValue {
    const NAME: &'static str = "Microsoft.UI.Composition.Interactions.CompositionConditionalValue";
}
impl ::std::convert::From<CompositionConditionalValue> for ::windows::runtime::IUnknown {
    fn from(value: CompositionConditionalValue) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&CompositionConditionalValue> for ::windows::runtime::IUnknown {
    fn from(value: &CompositionConditionalValue) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for CompositionConditionalValue
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &'a CompositionConditionalValue
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<CompositionConditionalValue> for ::windows::runtime::IInspectable {
    fn from(value: CompositionConditionalValue) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CompositionConditionalValue> for ::windows::runtime::IInspectable {
    fn from(value: &CompositionConditionalValue) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for CompositionConditionalValue
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a CompositionConditionalValue
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<CompositionConditionalValue> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: CompositionConditionalValue) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&CompositionConditionalValue> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &CompositionConditionalValue) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject>
    for CompositionConditionalValue
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject>
    for &CompositionConditionalValue
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::std::convert::TryInto::<super::IAnimationObject>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<CompositionConditionalValue> for ::windows::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: CompositionConditionalValue) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&CompositionConditionalValue> for ::windows::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &CompositionConditionalValue) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::Foundation::IClosable>
    for CompositionConditionalValue
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::Foundation::IClosable>
    for &CompositionConditionalValue
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::Foundation::IClosable> {
        ::std::convert::TryInto::<::windows::Foundation::IClosable>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::From<CompositionConditionalValue> for super::CompositionObject {
    fn from(value: CompositionConditionalValue) -> Self {
        ::std::convert::Into::<super::CompositionObject>::into(&value)
    }
}
impl ::std::convert::From<&CompositionConditionalValue> for super::CompositionObject {
    fn from(value: &CompositionConditionalValue) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject>
    for CompositionConditionalValue
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::CompositionObject>::into(
            self,
        ))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject>
    for &CompositionConditionalValue
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::CompositionObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::std::marker::Send for CompositionConditionalValue {}
unsafe impl ::std::marker::Sync for CompositionConditionalValue {}
#[doc = "*Required features: `UI_Composition_Interactions`*"]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct CompositionInteractionSourceCollection(pub ::windows::runtime::IInspectable);
impl CompositionInteractionSourceCollection {
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Count(&self) -> ::windows::runtime::Result<i32> {
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
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Add<'a, Param0: ::windows::runtime::IntoParam<'a, ICompositionInteractionSource>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Remove<'a, Param0: ::windows::runtime::IntoParam<'a, ICompositionInteractionSource>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn RemoveAll(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this)).ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn First(
        &self,
    ) -> ::windows::runtime::Result<
        ::windows::Foundation::Collections::IIterator<ICompositionInteractionSource>,
    > {
        let this = &::windows::runtime::Interface::cast::<
            ::windows::Foundation::Collections::IIterable<ICompositionInteractionSource>,
        >(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Collections::IIterator<
                ICompositionInteractionSource,
            >>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn PopulatePropertyInfo<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, super::AnimationPropertyInfo>,
    >(
        &self,
        propertyname: Param0,
        propertyinfo: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                propertyinfo.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Compositor(&self) -> ::windows::runtime::Result<super::Compositor> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Compositor>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<super::CompositionPropertySet> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn StartAnimation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, super::CompositionAnimation>,
    >(
        &self,
        propertyname: Param0,
        animation: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                animation.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn StopAnimation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        propertyname: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Comment(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
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
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn SetComment<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn ImplicitAnimations(
        &self,
    ) -> ::windows::runtime::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn SetImplicitAnimations<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ImplicitAnimationCollection>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn StartAnimationGroup<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn StopAnimationGroup<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    #[doc = "*Required features: `UI_Composition_Interactions`, `UI_Dispatching`*"]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::runtime::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn TryGetAnimationController<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        propertyname: Param0,
    ) -> ::windows::runtime::Result<super::AnimationController> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::AnimationController>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CompositionInteractionSourceCollection {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Composition.Interactions.CompositionInteractionSourceCollection;{9aa1b86b-b002-5e2e-bb2b-0e2c547445e1})" ) ;
}
unsafe impl ::windows::runtime::Interface for CompositionInteractionSourceCollection {
    type Vtable = ICompositionInteractionSourceCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2594289771,
        45058,
        24110,
        [187, 43, 14, 44, 84, 116, 69, 225],
    );
}
impl ::windows::runtime::RuntimeName for CompositionInteractionSourceCollection {
    const NAME: &'static str =
        "Microsoft.UI.Composition.Interactions.CompositionInteractionSourceCollection";
}
impl ::std::convert::From<CompositionInteractionSourceCollection> for ::windows::runtime::IUnknown {
    fn from(value: CompositionInteractionSourceCollection) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&CompositionInteractionSourceCollection>
    for ::windows::runtime::IUnknown
{
    fn from(value: &CompositionInteractionSourceCollection) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for CompositionInteractionSourceCollection
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &'a CompositionInteractionSourceCollection
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<CompositionInteractionSourceCollection>
    for ::windows::runtime::IInspectable
{
    fn from(value: CompositionInteractionSourceCollection) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CompositionInteractionSourceCollection>
    for ::windows::runtime::IInspectable
{
    fn from(value: &CompositionInteractionSourceCollection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for CompositionInteractionSourceCollection
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a CompositionInteractionSourceCollection
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<CompositionInteractionSourceCollection>
    for ::windows::Foundation::Collections::IIterable<ICompositionInteractionSource>
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: CompositionInteractionSourceCollection) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&CompositionInteractionSourceCollection>
    for ::windows::Foundation::Collections::IIterable<ICompositionInteractionSource>
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: &CompositionInteractionSourceCollection,
    ) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        ::windows::Foundation::Collections::IIterable<ICompositionInteractionSource>,
    > for CompositionInteractionSourceCollection
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        ::windows::Foundation::Collections::IIterable<ICompositionInteractionSource>,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        ::windows::Foundation::Collections::IIterable<ICompositionInteractionSource>,
    > for &CompositionInteractionSourceCollection
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        ::windows::Foundation::Collections::IIterable<ICompositionInteractionSource>,
    > {
        ::std::convert::TryInto::<
            ::windows::Foundation::Collections::IIterable<ICompositionInteractionSource>,
        >::try_into(self)
        .map(::windows::runtime::Param::Owned)
        .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<CompositionInteractionSourceCollection> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: CompositionInteractionSourceCollection) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&CompositionInteractionSourceCollection> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: &CompositionInteractionSourceCollection,
    ) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject>
    for CompositionInteractionSourceCollection
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject>
    for &CompositionInteractionSourceCollection
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::std::convert::TryInto::<super::IAnimationObject>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<CompositionInteractionSourceCollection>
    for ::windows::Foundation::IClosable
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: CompositionInteractionSourceCollection) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&CompositionInteractionSourceCollection>
    for ::windows::Foundation::IClosable
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: &CompositionInteractionSourceCollection,
    ) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::Foundation::IClosable>
    for CompositionInteractionSourceCollection
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::Foundation::IClosable>
    for &CompositionInteractionSourceCollection
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::Foundation::IClosable> {
        ::std::convert::TryInto::<::windows::Foundation::IClosable>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::From<CompositionInteractionSourceCollection> for super::CompositionObject {
    fn from(value: CompositionInteractionSourceCollection) -> Self {
        ::std::convert::Into::<super::CompositionObject>::into(&value)
    }
}
impl ::std::convert::From<&CompositionInteractionSourceCollection> for super::CompositionObject {
    fn from(value: &CompositionInteractionSourceCollection) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject>
    for CompositionInteractionSourceCollection
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::CompositionObject>::into(
            self,
        ))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject>
    for &CompositionInteractionSourceCollection
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::CompositionObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::std::marker::Send for CompositionInteractionSourceCollection {}
unsafe impl ::std::marker::Sync for CompositionInteractionSourceCollection {}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::std::iter::IntoIterator for CompositionInteractionSourceCollection {
    type Item = ICompositionInteractionSource;
    type IntoIter = ::windows::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::std::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::std::iter::IntoIterator for &CompositionInteractionSourceCollection {
    type Item = ICompositionInteractionSource;
    type IntoIter = ::windows::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[repr(transparent)]
#[doc(hidden)]
pub struct ICompositionConditionalValue(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICompositionConditionalValue {
    type Vtable = ICompositionConditionalValue_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        927194528,
        64482,
        24271,
        [158, 128, 70, 56, 160, 17, 247, 7],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionConditionalValue_abi(
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
        value: ::windows::runtime::RawPtr,
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
pub struct ICompositionConditionalValueStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICompositionConditionalValueStatics {
    type Vtable = ICompositionConditionalValueStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3742579743,
        41349,
        21356,
        [181, 75, 143, 54, 146, 18, 165, 129],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionConditionalValueStatics_abi(
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
        compositor: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc = "*Required features: `UI_Composition_Interactions`*"]
pub struct ICompositionInteractionSource(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICompositionInteractionSource {
    type Vtable = ICompositionInteractionSource_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1897689792,
        50182,
        18962,
        [133, 155, 180, 79, 101, 26, 240, 70],
    );
}
impl ICompositionInteractionSource {}
unsafe impl ::windows::runtime::RuntimeType for ICompositionInteractionSource {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"{711c72c0-c406-4a12-859b-b44f651af046}");
}
impl ::std::convert::From<ICompositionInteractionSource> for ::windows::runtime::IUnknown {
    fn from(value: ICompositionInteractionSource) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&ICompositionInteractionSource> for ::windows::runtime::IUnknown {
    fn from(value: &ICompositionInteractionSource) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for ICompositionInteractionSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &'a ICompositionInteractionSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<ICompositionInteractionSource> for ::windows::runtime::IInspectable {
    fn from(value: ICompositionInteractionSource) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ICompositionInteractionSource> for ::windows::runtime::IInspectable {
    fn from(value: &ICompositionInteractionSource) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for ICompositionInteractionSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a ICompositionInteractionSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionInteractionSource_abi(
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
pub struct ICompositionInteractionSourceCollection(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICompositionInteractionSourceCollection {
    type Vtable = ICompositionInteractionSourceCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2594289771,
        45058,
        24110,
        [187, 43, 14, 44, 84, 116, 69, 225],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionInteractionSourceCollection_abi(
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
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInteractionSourceConfiguration(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInteractionSourceConfiguration {
    type Vtable = IInteractionSourceConfiguration_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        161349924,
        56031,
        23494,
        [168, 149, 144, 56, 118, 87, 85, 15],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionSourceConfiguration_abi(
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
        result__: *mut InteractionSourceRedirectionMode,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: InteractionSourceRedirectionMode,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut InteractionSourceRedirectionMode,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: InteractionSourceRedirectionMode,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut InteractionSourceRedirectionMode,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: InteractionSourceRedirectionMode,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInteractionTracker(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInteractionTracker {
    type Vtable = IInteractionTracker_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        47770655,
        36612,
        20574,
        [189, 30, 71, 178, 162, 4, 222, 81],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTracker_abi(
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
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::Foundation::Numerics::Vector3,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::Foundation::Numerics::Vector3,
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
        result__: *mut ::windows::Foundation::Numerics::Vector3,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::Foundation::Numerics::Vector3,
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
        result__: *mut ::windows::Foundation::Numerics::Vector3,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::Foundation::Numerics::Vector3,
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
        result__: *mut ::windows::Foundation::Numerics::Vector3,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f32,
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
        adjustment: f32,
        positionthreshold: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        adjustment: f32,
        positionthreshold: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        modifiers: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        modifiers: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        modifiers: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::Foundation::Numerics::Vector3,
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        amount: ::windows::Foundation::Numerics::Vector3,
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        animation: ::windows::runtime::RawPtr,
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        velocityinpixelspersecond: ::windows::Foundation::Numerics::Vector3,
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: f32,
        centerpoint: ::windows::Foundation::Numerics::Vector3,
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        animation: ::windows::runtime::RawPtr,
        centerpoint: ::windows::Foundation::Numerics::Vector3,
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        velocityinpercentpersecond: f32,
        centerpoint: ::windows::Foundation::Numerics::Vector3,
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInteractionTracker2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInteractionTracker2 {
    type Vtable = IInteractionTracker2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        963477425,
        12205,
        21768,
        [133, 145, 79, 240, 220, 90, 116, 132],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTracker2_abi(
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
        conditionalvalues: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        conditionalvalues: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInteractionTracker3(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInteractionTracker3 {
    type Vtable = IInteractionTracker3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        597119695,
        9836,
        23243,
        [172, 195, 179, 227, 236, 175, 77, 63],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTracker3_abi(
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
        modifiers: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInteractionTracker4(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInteractionTracker4 {
    type Vtable = IInteractionTracker4_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2846477826,
        21449,
        22160,
        [165, 117, 243, 64, 183, 194, 253, 242],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTracker4_abi(
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
        value: ::windows::Foundation::Numerics::Vector3,
        option: InteractionTrackerClampingOption,
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        amount: ::windows::Foundation::Numerics::Vector3,
        option: InteractionTrackerClampingOption,
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInteractionTracker5(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInteractionTracker5 {
    type Vtable = IInteractionTracker5_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3690779443,
        50111,
        20567,
        [164, 94, 37, 237, 240, 110, 189, 143],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTracker5_abi(
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
        value: ::windows::Foundation::Numerics::Vector3,
        option: InteractionTrackerClampingOption,
        posupdateoption: InteractionTrackerPositionUpdateOption,
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInteractionTrackerCustomAnimationStateEnteredArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInteractionTrackerCustomAnimationStateEnteredArgs {
    type Vtable = IInteractionTrackerCustomAnimationStateEnteredArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1952711516,
        53198,
        22234,
        [148, 114, 66, 15, 39, 107, 208, 165],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerCustomAnimationStateEnteredArgs_abi(
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
pub struct IInteractionTrackerCustomAnimationStateEnteredArgs2(
    pub ::windows::runtime::IInspectable,
);
unsafe impl ::windows::runtime::Interface for IInteractionTrackerCustomAnimationStateEnteredArgs2 {
    type Vtable = IInteractionTrackerCustomAnimationStateEnteredArgs2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        112828348,
        54952,
        23267,
        [136, 184, 233, 22, 33, 190, 203, 214],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerCustomAnimationStateEnteredArgs2_abi(
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
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInteractionTrackerIdleStateEnteredArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInteractionTrackerIdleStateEnteredArgs {
    type Vtable = IInteractionTrackerIdleStateEnteredArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        428905643,
        5629,
        21404,
        [151, 184, 150, 74, 129, 150, 247, 119],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerIdleStateEnteredArgs_abi(
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
pub struct IInteractionTrackerIdleStateEnteredArgs2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInteractionTrackerIdleStateEnteredArgs2 {
    type Vtable = IInteractionTrackerIdleStateEnteredArgs2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1320293312,
        37660,
        20836,
        [137, 101, 17, 192, 24, 109, 51, 144],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerIdleStateEnteredArgs2_abi(
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
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaModifier(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInteractionTrackerInertiaModifier {
    type Vtable = IInteractionTrackerInertiaModifier_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1295649899,
        50440,
        20521,
        [164, 122, 203, 246, 70, 54, 240, 16],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaModifier_abi(
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
pub struct IInteractionTrackerInertiaModifierFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInteractionTrackerInertiaModifierFactory {
    type Vtable = IInteractionTrackerInertiaModifierFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1844337459,
        2906,
        22449,
        [133, 55, 147, 212, 253, 3, 143, 159],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaModifierFactory_abi(
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
pub struct IInteractionTrackerInertiaMotion(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInteractionTrackerInertiaMotion {
    type Vtable = IInteractionTrackerInertiaMotion_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2448843456,
        12609,
        23390,
        [134, 47, 207, 198, 11, 238, 140, 214],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaMotion_abi(
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
        value: ::windows::runtime::RawPtr,
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
pub struct IInteractionTrackerInertiaMotionStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInteractionTrackerInertiaMotionStatics {
    type Vtable = IInteractionTrackerInertiaMotionStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2954385999,
        89,
        21190,
        [166, 96, 154, 237, 12, 68, 255, 125],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaMotionStatics_abi(
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
        compositor: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaNaturalMotion(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInteractionTrackerInertiaNaturalMotion {
    type Vtable = IInteractionTrackerInertiaNaturalMotion_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2356445920,
        6237,
        22193,
        [182, 127, 252, 164, 252, 209, 60, 210],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaNaturalMotion_abi(
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
        value: ::windows::runtime::RawPtr,
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
pub struct IInteractionTrackerInertiaNaturalMotionStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInteractionTrackerInertiaNaturalMotionStatics {
    type Vtable = IInteractionTrackerInertiaNaturalMotionStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2249113923,
        61797,
        21144,
        [171, 242, 71, 54, 157, 208, 127, 16],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaNaturalMotionStatics_abi(
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
        compositor: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaRestingValue(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInteractionTrackerInertiaRestingValue {
    type Vtable = IInteractionTrackerInertiaRestingValue_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        439034061,
        13169,
        21503,
        [165, 96, 244, 132, 123, 70, 125, 115],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaRestingValue_abi(
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
        value: ::windows::runtime::RawPtr,
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
pub struct IInteractionTrackerInertiaRestingValueStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInteractionTrackerInertiaRestingValueStatics {
    type Vtable = IInteractionTrackerInertiaRestingValueStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3473867796,
        32735,
        21124,
        [174, 239, 40, 183, 27, 98, 170, 79],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaRestingValueStatics_abi(
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
        compositor: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaStateEnteredArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInteractionTrackerInertiaStateEnteredArgs {
    type Vtable = IInteractionTrackerInertiaStateEnteredArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1534511433,
        42192,
        23709,
        [146, 146, 112, 19, 174, 150, 86, 199],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaStateEnteredArgs_abi(
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
        result__: *mut ::windows::Foundation::Numerics::Vector3,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::Foundation::Numerics::Vector3,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaStateEnteredArgs2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInteractionTrackerInertiaStateEnteredArgs2 {
    type Vtable = IInteractionTrackerInertiaStateEnteredArgs2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3291315855,
        29081,
        22441,
        [138, 236, 135, 39, 85, 43, 19, 230],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaStateEnteredArgs2_abi(
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
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaStateEnteredArgs3(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInteractionTrackerInertiaStateEnteredArgs3 {
    type Vtable = IInteractionTrackerInertiaStateEnteredArgs3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3463605408,
        7172,
        21275,
        [153, 81, 74, 236, 153, 105, 82, 228],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaStateEnteredArgs3_abi(
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
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInteractionTrackerInteractingStateEnteredArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInteractionTrackerInteractingStateEnteredArgs {
    type Vtable = IInteractionTrackerInteractingStateEnteredArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1892850564,
        2353,
        24343,
        [168, 161, 130, 248, 248, 120, 37, 50],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInteractingStateEnteredArgs_abi(
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
pub struct IInteractionTrackerInteractingStateEnteredArgs2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInteractionTrackerInteractingStateEnteredArgs2 {
    type Vtable = IInteractionTrackerInteractingStateEnteredArgs2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        790623117,
        12113,
        23787,
        [141, 9, 189, 161, 81, 159, 147, 66],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInteractingStateEnteredArgs2_abi(
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
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc = "*Required features: `UI_Composition_Interactions`*"]
pub struct IInteractionTrackerOwner(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInteractionTrackerOwner {
    type Vtable = IInteractionTrackerOwner_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2288613277,
        7466,
        22550,
        [131, 106, 104, 169, 16, 80, 125, 135],
    );
}
impl IInteractionTrackerOwner {
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn CustomAnimationStateEntered<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, InteractionTracker>,
        Param1: ::windows::runtime::IntoParam<'a, InteractionTrackerCustomAnimationStateEnteredArgs>,
    >(
        &self,
        sender: Param0,
        args: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                sender.into_param().abi(),
                args.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn IdleStateEntered<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, InteractionTracker>,
        Param1: ::windows::runtime::IntoParam<'a, InteractionTrackerIdleStateEnteredArgs>,
    >(
        &self,
        sender: Param0,
        args: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                sender.into_param().abi(),
                args.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn InertiaStateEntered<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, InteractionTracker>,
        Param1: ::windows::runtime::IntoParam<'a, InteractionTrackerInertiaStateEnteredArgs>,
    >(
        &self,
        sender: Param0,
        args: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                sender.into_param().abi(),
                args.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn InteractingStateEntered<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, InteractionTracker>,
        Param1: ::windows::runtime::IntoParam<'a, InteractionTrackerInteractingStateEnteredArgs>,
    >(
        &self,
        sender: Param0,
        args: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                sender.into_param().abi(),
                args.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn RequestIgnored<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, InteractionTracker>,
        Param1: ::windows::runtime::IntoParam<'a, InteractionTrackerRequestIgnoredArgs>,
    >(
        &self,
        sender: Param0,
        args: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                sender.into_param().abi(),
                args.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn ValuesChanged<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, InteractionTracker>,
        Param1: ::windows::runtime::IntoParam<'a, InteractionTrackerValuesChangedArgs>,
    >(
        &self,
        sender: Param0,
        args: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                sender.into_param().abi(),
                args.into_param().abi(),
            )
            .ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IInteractionTrackerOwner {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"{8869779d-1d2a-5816-836a-68a910507d87}");
}
impl ::std::convert::From<IInteractionTrackerOwner> for ::windows::runtime::IUnknown {
    fn from(value: IInteractionTrackerOwner) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&IInteractionTrackerOwner> for ::windows::runtime::IUnknown {
    fn from(value: &IInteractionTrackerOwner) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IInteractionTrackerOwner
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &'a IInteractionTrackerOwner
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<IInteractionTrackerOwner> for ::windows::runtime::IInspectable {
    fn from(value: IInteractionTrackerOwner) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IInteractionTrackerOwner> for ::windows::runtime::IInspectable {
    fn from(value: &IInteractionTrackerOwner) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for IInteractionTrackerOwner
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a IInteractionTrackerOwner
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerOwner_abi(
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
        sender: ::windows::runtime::RawPtr,
        args: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        sender: ::windows::runtime::RawPtr,
        args: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        sender: ::windows::runtime::RawPtr,
        args: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        sender: ::windows::runtime::RawPtr,
        args: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        sender: ::windows::runtime::RawPtr,
        args: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        sender: ::windows::runtime::RawPtr,
        args: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInteractionTrackerRequestIgnoredArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInteractionTrackerRequestIgnoredArgs {
    type Vtable = IInteractionTrackerRequestIgnoredArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3262521438,
        63397,
        23458,
        [173, 69, 209, 44, 60, 51, 145, 73],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerRequestIgnoredArgs_abi(
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
pub struct IInteractionTrackerStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInteractionTrackerStatics {
    type Vtable = IInteractionTrackerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2060027514,
        57710,
        22255,
        [152, 9, 246, 228, 4, 36, 15, 80],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerStatics_abi(
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
        compositor: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        compositor: ::windows::runtime::RawPtr,
        owner: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInteractionTrackerStatics2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInteractionTrackerStatics2 {
    type Vtable = IInteractionTrackerStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        627412556,
        47519,
        20744,
        [170, 183, 28, 196, 79, 17, 80, 139],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerStatics2_abi(
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
        boundtracker1: ::windows::runtime::RawPtr,
        boundtracker2: ::windows::runtime::RawPtr,
        axismode: InteractionBindingAxisModes,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        boundtracker1: ::windows::runtime::RawPtr,
        boundtracker2: ::windows::runtime::RawPtr,
        result__: *mut InteractionBindingAxisModes,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInteractionTrackerValuesChangedArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInteractionTrackerValuesChangedArgs {
    type Vtable = IInteractionTrackerValuesChangedArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2605276141,
        7415,
        21953,
        [130, 185, 128, 34, 203, 243, 199, 102],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerValuesChangedArgs_abi(
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
        result__: *mut ::windows::Foundation::Numerics::Vector3,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInteractionTrackerVector2InertiaModifier(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInteractionTrackerVector2InertiaModifier {
    type Vtable = IInteractionTrackerVector2InertiaModifier_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1267651344,
        52065,
        24330,
        [185, 154, 148, 12, 221, 44, 66, 177],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerVector2InertiaModifier_abi(
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
pub struct IInteractionTrackerVector2InertiaModifierFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInteractionTrackerVector2InertiaModifierFactory {
    type Vtable = IInteractionTrackerVector2InertiaModifierFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        457167424,
        47718,
        21142,
        [184, 1, 98, 162, 163, 96, 102, 19],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerVector2InertiaModifierFactory_abi(
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
pub struct IInteractionTrackerVector2InertiaNaturalMotion(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInteractionTrackerVector2InertiaNaturalMotion {
    type Vtable = IInteractionTrackerVector2InertiaNaturalMotion_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        159097254,
        57463,
        21201,
        [134, 211, 56, 227, 246, 97, 157, 223],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerVector2InertiaNaturalMotion_abi(
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
        value: ::windows::runtime::RawPtr,
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
pub struct IInteractionTrackerVector2InertiaNaturalMotionStatics(
    pub ::windows::runtime::IInspectable,
);
unsafe impl ::windows::runtime::Interface
    for IInteractionTrackerVector2InertiaNaturalMotionStatics
{
    type Vtable = IInteractionTrackerVector2InertiaNaturalMotionStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3424955271,
        37169,
        21126,
        [179, 206, 30, 249, 126, 9, 116, 230],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerVector2InertiaNaturalMotionStatics_abi(
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
        compositor: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVisualInteractionSource(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IVisualInteractionSource {
    type Vtable = IVisualInteractionSource_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3931724949,
        47563,
        23764,
        [187, 156, 73, 52, 255, 50, 144, 99],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualInteractionSource_abi(
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
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut VisualInteractionSourceRedirectionMode,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: VisualInteractionSourceRedirectionMode,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut InteractionChainingMode,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: InteractionChainingMode,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut InteractionSourceMode,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: InteractionSourceMode,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut InteractionChainingMode,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: InteractionChainingMode,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut InteractionSourceMode,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: InteractionSourceMode,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut InteractionChainingMode,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: InteractionChainingMode,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut InteractionSourceMode,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: InteractionSourceMode,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "UI_Input")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pointerpoint: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Input"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVisualInteractionSource2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IVisualInteractionSource2 {
    type Vtable = IVisualInteractionSource2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4279317178,
        56333,
        20894,
        [190, 73, 190, 48, 30, 82, 48, 106],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualInteractionSource2_abi(
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
        result__: *mut ::windows::Foundation::Numerics::Vector3,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::Foundation::Numerics::Vector3,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::Foundation::Numerics::Vector3,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        conditionalvalues: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        conditionalvalues: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        conditionalvalues: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        conditionalvalues: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        conditionalvalues: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVisualInteractionSource3(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IVisualInteractionSource3 {
    type Vtable = IVisualInteractionSource3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3575889254,
        41053,
        21527,
        [142, 7, 132, 174, 60, 175, 151, 82],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualInteractionSource3_abi(
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
#[repr(transparent)]
#[doc(hidden)]
pub struct IVisualInteractionSourceObjectFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IVisualInteractionSourceObjectFactory {
    type Vtable = IVisualInteractionSourceObjectFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4273418498,
        9100,
        21162,
        [142, 3, 182, 141, 94, 204, 68, 179],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualInteractionSourceObjectFactory_abi(
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
pub struct IVisualInteractionSourceStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IVisualInteractionSourceStatics {
    type Vtable = IVisualInteractionSourceStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1607059299,
        58085,
        21262,
        [135, 205, 185, 49, 24, 173, 232, 163],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualInteractionSourceStatics_abi(
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
        source: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVisualInteractionSourceStatics2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IVisualInteractionSourceStatics2 {
    type Vtable = IVisualInteractionSourceStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2796852478,
        4769,
        23155,
        [184, 126, 76, 78, 245, 142, 172, 108],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualInteractionSourceStatics2_abi(
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
        source: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `UI_Composition_Interactions`*"]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct InteractionBindingAxisModes(pub u32);
impl InteractionBindingAxisModes {
    pub const None: InteractionBindingAxisModes = InteractionBindingAxisModes(0u32);
    pub const PositionX: InteractionBindingAxisModes = InteractionBindingAxisModes(1u32);
    pub const PositionY: InteractionBindingAxisModes = InteractionBindingAxisModes(2u32);
    pub const Scale: InteractionBindingAxisModes = InteractionBindingAxisModes(4u32);
}
impl ::std::convert::From<u32> for InteractionBindingAxisModes {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for InteractionBindingAxisModes {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for InteractionBindingAxisModes {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Composition.Interactions.InteractionBindingAxisModes;u4)",
    );
}
impl ::windows::runtime::DefaultType for InteractionBindingAxisModes {
    type DefaultType = Self;
}
impl ::std::ops::BitOr for InteractionBindingAxisModes {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for InteractionBindingAxisModes {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for InteractionBindingAxisModes {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for InteractionBindingAxisModes {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for InteractionBindingAxisModes {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `UI_Composition_Interactions`*"]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct InteractionChainingMode(pub i32);
impl InteractionChainingMode {
    pub const Auto: InteractionChainingMode = InteractionChainingMode(0i32);
    pub const Always: InteractionChainingMode = InteractionChainingMode(1i32);
    pub const Never: InteractionChainingMode = InteractionChainingMode(2i32);
}
impl ::std::convert::From<i32> for InteractionChainingMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for InteractionChainingMode {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for InteractionChainingMode {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Composition.Interactions.InteractionChainingMode;i4)",
    );
}
impl ::windows::runtime::DefaultType for InteractionChainingMode {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Composition_Interactions`*"]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct InteractionSourceConfiguration(pub ::windows::runtime::IInspectable);
impl InteractionSourceConfiguration {
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn PositionXSourceMode(
        &self,
    ) -> ::windows::runtime::Result<InteractionSourceRedirectionMode> {
        let this = self;
        unsafe {
            let mut result__: InteractionSourceRedirectionMode = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<InteractionSourceRedirectionMode>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn SetPositionXSourceMode(
        &self,
        value: InteractionSourceRedirectionMode,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn PositionYSourceMode(
        &self,
    ) -> ::windows::runtime::Result<InteractionSourceRedirectionMode> {
        let this = self;
        unsafe {
            let mut result__: InteractionSourceRedirectionMode = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<InteractionSourceRedirectionMode>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn SetPositionYSourceMode(
        &self,
        value: InteractionSourceRedirectionMode,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn ScaleSourceMode(&self) -> ::windows::runtime::Result<InteractionSourceRedirectionMode> {
        let this = self;
        unsafe {
            let mut result__: InteractionSourceRedirectionMode = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<InteractionSourceRedirectionMode>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn SetScaleSourceMode(
        &self,
        value: InteractionSourceRedirectionMode,
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
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn PopulatePropertyInfo<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, super::AnimationPropertyInfo>,
    >(
        &self,
        propertyname: Param0,
        propertyinfo: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                propertyinfo.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Compositor(&self) -> ::windows::runtime::Result<super::Compositor> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Compositor>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<super::CompositionPropertySet> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn StartAnimation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, super::CompositionAnimation>,
    >(
        &self,
        propertyname: Param0,
        animation: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                animation.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn StopAnimation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        propertyname: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Comment(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
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
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn SetComment<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn ImplicitAnimations(
        &self,
    ) -> ::windows::runtime::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn SetImplicitAnimations<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ImplicitAnimationCollection>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn StartAnimationGroup<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn StopAnimationGroup<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    #[doc = "*Required features: `UI_Composition_Interactions`, `UI_Dispatching`*"]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::runtime::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn TryGetAnimationController<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        propertyname: Param0,
    ) -> ::windows::runtime::Result<super::AnimationController> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::AnimationController>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for InteractionSourceConfiguration {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Composition.Interactions.InteractionSourceConfiguration;{099e0124-dadf-5bc6-a895-90387657550f})" ) ;
}
unsafe impl ::windows::runtime::Interface for InteractionSourceConfiguration {
    type Vtable = IInteractionSourceConfiguration_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        161349924,
        56031,
        23494,
        [168, 149, 144, 56, 118, 87, 85, 15],
    );
}
impl ::windows::runtime::RuntimeName for InteractionSourceConfiguration {
    const NAME: &'static str =
        "Microsoft.UI.Composition.Interactions.InteractionSourceConfiguration";
}
impl ::std::convert::From<InteractionSourceConfiguration> for ::windows::runtime::IUnknown {
    fn from(value: InteractionSourceConfiguration) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&InteractionSourceConfiguration> for ::windows::runtime::IUnknown {
    fn from(value: &InteractionSourceConfiguration) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for InteractionSourceConfiguration
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &'a InteractionSourceConfiguration
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<InteractionSourceConfiguration> for ::windows::runtime::IInspectable {
    fn from(value: InteractionSourceConfiguration) -> Self {
        value.0
    }
}
impl ::std::convert::From<&InteractionSourceConfiguration> for ::windows::runtime::IInspectable {
    fn from(value: &InteractionSourceConfiguration) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for InteractionSourceConfiguration
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a InteractionSourceConfiguration
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<InteractionSourceConfiguration> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: InteractionSourceConfiguration) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&InteractionSourceConfiguration> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &InteractionSourceConfiguration) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject>
    for InteractionSourceConfiguration
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject>
    for &InteractionSourceConfiguration
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::std::convert::TryInto::<super::IAnimationObject>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<InteractionSourceConfiguration> for ::windows::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: InteractionSourceConfiguration) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&InteractionSourceConfiguration> for ::windows::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &InteractionSourceConfiguration) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::Foundation::IClosable>
    for InteractionSourceConfiguration
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::Foundation::IClosable>
    for &InteractionSourceConfiguration
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::Foundation::IClosable> {
        ::std::convert::TryInto::<::windows::Foundation::IClosable>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::From<InteractionSourceConfiguration> for super::CompositionObject {
    fn from(value: InteractionSourceConfiguration) -> Self {
        ::std::convert::Into::<super::CompositionObject>::into(&value)
    }
}
impl ::std::convert::From<&InteractionSourceConfiguration> for super::CompositionObject {
    fn from(value: &InteractionSourceConfiguration) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject>
    for InteractionSourceConfiguration
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::CompositionObject>::into(
            self,
        ))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject>
    for &InteractionSourceConfiguration
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::CompositionObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::std::marker::Send for InteractionSourceConfiguration {}
unsafe impl ::std::marker::Sync for InteractionSourceConfiguration {}
#[doc = "*Required features: `UI_Composition_Interactions`*"]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct InteractionSourceMode(pub i32);
impl InteractionSourceMode {
    pub const Disabled: InteractionSourceMode = InteractionSourceMode(0i32);
    pub const EnabledWithInertia: InteractionSourceMode = InteractionSourceMode(1i32);
    pub const EnabledWithoutInertia: InteractionSourceMode = InteractionSourceMode(2i32);
}
impl ::std::convert::From<i32> for InteractionSourceMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for InteractionSourceMode {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for InteractionSourceMode {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Composition.Interactions.InteractionSourceMode;i4)",
    );
}
impl ::windows::runtime::DefaultType for InteractionSourceMode {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Composition_Interactions`*"]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct InteractionSourceRedirectionMode(pub i32);
impl InteractionSourceRedirectionMode {
    pub const Disabled: InteractionSourceRedirectionMode = InteractionSourceRedirectionMode(0i32);
    pub const Enabled: InteractionSourceRedirectionMode = InteractionSourceRedirectionMode(1i32);
}
impl ::std::convert::From<i32> for InteractionSourceRedirectionMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for InteractionSourceRedirectionMode {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for InteractionSourceRedirectionMode {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Composition.Interactions.InteractionSourceRedirectionMode;i4)",
    );
}
impl ::windows::runtime::DefaultType for InteractionSourceRedirectionMode {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Composition_Interactions`*"]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct InteractionTracker(pub ::windows::runtime::IInspectable);
impl InteractionTracker {
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn InteractionSources(
        &self,
    ) -> ::windows::runtime::Result<CompositionInteractionSourceCollection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CompositionInteractionSourceCollection>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn IsPositionRoundingSuggested(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn MaxPosition(
        &self,
    ) -> ::windows::runtime::Result<::windows::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::Numerics::Vector3 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn SetMaxPosition<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::Numerics::Vector3>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn MaxScale(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn SetMaxScale(&self, value: f32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn MinPosition(
        &self,
    ) -> ::windows::runtime::Result<::windows::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::Numerics::Vector3 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn SetMinPosition<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::Numerics::Vector3>,
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
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn MinScale(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn SetMinScale(&self, value: f32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).15)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn NaturalRestingPosition(
        &self,
    ) -> ::windows::runtime::Result<::windows::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::Numerics::Vector3 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn NaturalRestingScale(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Owner(&self) -> ::windows::runtime::Result<IInteractionTrackerOwner> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<IInteractionTrackerOwner>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Position(&self) -> ::windows::runtime::Result<::windows::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::Numerics::Vector3 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn PositionInertiaDecayRate(
        &self,
    ) -> ::windows::runtime::Result<
        ::windows::Foundation::IReference<::windows::Foundation::Numerics::Vector3>,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            ( :: windows :: runtime :: Interface :: vtable ( this ) .20 ) ( :: std :: mem :: transmute_copy ( this ) , & mut result__ ) . from_abi :: < ::windows::Foundation:: IReference :: < ::windows::Foundation::Numerics:: Vector3 > > ( result__ )
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn SetPositionInertiaDecayRate<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            ::windows::Foundation::IReference<::windows::Foundation::Numerics::Vector3>,
        >,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).21)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn PositionVelocityInPixelsPerSecond(
        &self,
    ) -> ::windows::runtime::Result<::windows::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::Numerics::Vector3 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Scale(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn ScaleInertiaDecayRate(
        &self,
    ) -> ::windows::runtime::Result<::windows::Foundation::IReference<f32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::IReference<f32>>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn SetScaleInertiaDecayRate<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::IReference<f32>>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).25)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn ScaleVelocityInPercentPerSecond(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).26)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn AdjustPositionXIfGreaterThanThreshold(
        &self,
        adjustment: f32,
        positionthreshold: f32,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).27)(
                ::std::mem::transmute_copy(this),
                adjustment,
                positionthreshold,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn AdjustPositionYIfGreaterThanThreshold(
        &self,
        adjustment: f32,
        positionthreshold: f32,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).28)(
                ::std::mem::transmute_copy(this),
                adjustment,
                positionthreshold,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn ConfigurePositionXInertiaModifiers<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            ::windows::Foundation::Collections::IIterable<InteractionTrackerInertiaModifier>,
        >,
    >(
        &self,
        modifiers: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).29)(
                ::std::mem::transmute_copy(this),
                modifiers.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn ConfigurePositionYInertiaModifiers<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            ::windows::Foundation::Collections::IIterable<InteractionTrackerInertiaModifier>,
        >,
    >(
        &self,
        modifiers: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).30)(
                ::std::mem::transmute_copy(this),
                modifiers.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn ConfigureScaleInertiaModifiers<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            ::windows::Foundation::Collections::IIterable<InteractionTrackerInertiaModifier>,
        >,
    >(
        &self,
        modifiers: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).31)(
                ::std::mem::transmute_copy(this),
                modifiers.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn TryUpdatePosition<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::Numerics::Vector3>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).32)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn TryUpdatePositionBy<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::Numerics::Vector3>,
    >(
        &self,
        amount: Param0,
    ) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).33)(
                ::std::mem::transmute_copy(this),
                amount.into_param().abi(),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn TryUpdatePositionWithAnimation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::CompositionAnimation>,
    >(
        &self,
        animation: Param0,
    ) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).34)(
                ::std::mem::transmute_copy(this),
                animation.into_param().abi(),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn TryUpdatePositionWithAdditionalVelocity<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::Numerics::Vector3>,
    >(
        &self,
        velocityinpixelspersecond: Param0,
    ) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).35)(
                ::std::mem::transmute_copy(this),
                velocityinpixelspersecond.into_param().abi(),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn TryUpdateScale<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::Foundation::Numerics::Vector3>,
    >(
        &self,
        value: f32,
        centerpoint: Param1,
    ) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).36)(
                ::std::mem::transmute_copy(this),
                value,
                centerpoint.into_param().abi(),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn TryUpdateScaleWithAnimation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::CompositionAnimation>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::Foundation::Numerics::Vector3>,
    >(
        &self,
        animation: Param0,
        centerpoint: Param1,
    ) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).37)(
                ::std::mem::transmute_copy(this),
                animation.into_param().abi(),
                centerpoint.into_param().abi(),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn TryUpdateScaleWithAdditionalVelocity<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::Foundation::Numerics::Vector3>,
    >(
        &self,
        velocityinpercentpersecond: f32,
        centerpoint: Param1,
    ) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).38)(
                ::std::mem::transmute_copy(this),
                velocityinpercentpersecond,
                centerpoint.into_param().abi(),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn ConfigureCenterPointXInertiaModifiers<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            ::windows::Foundation::Collections::IIterable<CompositionConditionalValue>,
        >,
    >(
        &self,
        conditionalvalues: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IInteractionTracker2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                conditionalvalues.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn ConfigureCenterPointYInertiaModifiers<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            ::windows::Foundation::Collections::IIterable<CompositionConditionalValue>,
        >,
    >(
        &self,
        conditionalvalues: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IInteractionTracker2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                conditionalvalues.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn ConfigureVector2PositionInertiaModifiers<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            ::windows::Foundation::Collections::IIterable<InteractionTrackerVector2InertiaModifier>,
        >,
    >(
        &self,
        modifiers: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IInteractionTracker3>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                modifiers.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn TryUpdatePositionWithOption<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::Numerics::Vector3>,
    >(
        &self,
        value: Param0,
        option: InteractionTrackerClampingOption,
    ) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<IInteractionTracker4>(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
                option,
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn TryUpdatePositionByWithOption<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::Numerics::Vector3>,
    >(
        &self,
        amount: Param0,
        option: InteractionTrackerClampingOption,
    ) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<IInteractionTracker4>(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                amount.into_param().abi(),
                option,
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn IsInertiaFromImpulse(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IInteractionTracker4>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn TryUpdatePositionWithOption2<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::Numerics::Vector3>,
    >(
        &self,
        value: Param0,
        option: InteractionTrackerClampingOption,
        posupdateoption: InteractionTrackerPositionUpdateOption,
    ) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<IInteractionTracker5>(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
                option,
                posupdateoption,
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, super::Compositor>>(
        compositor: Param0,
    ) -> ::windows::runtime::Result<InteractionTracker> {
        Self::IInteractionTrackerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                compositor.into_param().abi(),
                &mut result__,
            )
            .from_abi::<InteractionTracker>(result__)
        })
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn CreateWithOwner<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Compositor>,
        Param1: ::windows::runtime::IntoParam<'a, IInteractionTrackerOwner>,
    >(
        compositor: Param0,
        owner: Param1,
    ) -> ::windows::runtime::Result<InteractionTracker> {
        Self::IInteractionTrackerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                compositor.into_param().abi(),
                owner.into_param().abi(),
                &mut result__,
            )
            .from_abi::<InteractionTracker>(result__)
        })
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn SetBindingMode<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, InteractionTracker>,
        Param1: ::windows::runtime::IntoParam<'a, InteractionTracker>,
    >(
        boundtracker1: Param0,
        boundtracker2: Param1,
        axismode: InteractionBindingAxisModes,
    ) -> ::windows::runtime::Result<()> {
        Self::IInteractionTrackerStatics2(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                boundtracker1.into_param().abi(),
                boundtracker2.into_param().abi(),
                axismode,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn GetBindingMode<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, InteractionTracker>,
        Param1: ::windows::runtime::IntoParam<'a, InteractionTracker>,
    >(
        boundtracker1: Param0,
        boundtracker2: Param1,
    ) -> ::windows::runtime::Result<InteractionBindingAxisModes> {
        Self::IInteractionTrackerStatics2(|this| unsafe {
            let mut result__: InteractionBindingAxisModes = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                boundtracker1.into_param().abi(),
                boundtracker2.into_param().abi(),
                &mut result__,
            )
            .from_abi::<InteractionBindingAxisModes>(result__)
        })
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn PopulatePropertyInfo<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, super::AnimationPropertyInfo>,
    >(
        &self,
        propertyname: Param0,
        propertyinfo: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                propertyinfo.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Compositor(&self) -> ::windows::runtime::Result<super::Compositor> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Compositor>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<super::CompositionPropertySet> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn StartAnimation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, super::CompositionAnimation>,
    >(
        &self,
        propertyname: Param0,
        animation: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                animation.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn StopAnimation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        propertyname: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Comment(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
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
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn SetComment<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn ImplicitAnimations(
        &self,
    ) -> ::windows::runtime::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn SetImplicitAnimations<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ImplicitAnimationCollection>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn StartAnimationGroup<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn StopAnimationGroup<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    #[doc = "*Required features: `UI_Composition_Interactions`, `UI_Dispatching`*"]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::runtime::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn TryGetAnimationController<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        propertyname: Param0,
    ) -> ::windows::runtime::Result<super::AnimationController> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::AnimationController>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn IInteractionTrackerStatics<
        R,
        F: FnOnce(&IInteractionTrackerStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            InteractionTracker,
            IInteractionTrackerStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IInteractionTrackerStatics2<
        R,
        F: FnOnce(&IInteractionTrackerStatics2) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            InteractionTracker,
            IInteractionTrackerStatics2,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for InteractionTracker {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Composition.Interactions.InteractionTracker;{02d8ec1f-8f04-505e-bd1e-47b2a204de51})" ) ;
}
unsafe impl ::windows::runtime::Interface for InteractionTracker {
    type Vtable = IInteractionTracker_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        47770655,
        36612,
        20574,
        [189, 30, 71, 178, 162, 4, 222, 81],
    );
}
impl ::windows::runtime::RuntimeName for InteractionTracker {
    const NAME: &'static str = "Microsoft.UI.Composition.Interactions.InteractionTracker";
}
impl ::std::convert::From<InteractionTracker> for ::windows::runtime::IUnknown {
    fn from(value: InteractionTracker) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&InteractionTracker> for ::windows::runtime::IUnknown {
    fn from(value: &InteractionTracker) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for InteractionTracker {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &'a InteractionTracker
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<InteractionTracker> for ::windows::runtime::IInspectable {
    fn from(value: InteractionTracker) -> Self {
        value.0
    }
}
impl ::std::convert::From<&InteractionTracker> for ::windows::runtime::IInspectable {
    fn from(value: &InteractionTracker) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for InteractionTracker
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a InteractionTracker
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<InteractionTracker> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: InteractionTracker) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&InteractionTracker> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &InteractionTracker) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject> for InteractionTracker {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject> for &InteractionTracker {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::std::convert::TryInto::<super::IAnimationObject>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<InteractionTracker> for ::windows::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: InteractionTracker) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&InteractionTracker> for ::windows::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &InteractionTracker) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::Foundation::IClosable>
    for InteractionTracker
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::Foundation::IClosable>
    for &InteractionTracker
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::Foundation::IClosable> {
        ::std::convert::TryInto::<::windows::Foundation::IClosable>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::From<InteractionTracker> for super::CompositionObject {
    fn from(value: InteractionTracker) -> Self {
        ::std::convert::Into::<super::CompositionObject>::into(&value)
    }
}
impl ::std::convert::From<&InteractionTracker> for super::CompositionObject {
    fn from(value: &InteractionTracker) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject> for InteractionTracker {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::CompositionObject>::into(
            self,
        ))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject> for &InteractionTracker {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::CompositionObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::std::marker::Send for InteractionTracker {}
unsafe impl ::std::marker::Sync for InteractionTracker {}
#[doc = "*Required features: `UI_Composition_Interactions`*"]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct InteractionTrackerClampingOption(pub i32);
impl InteractionTrackerClampingOption {
    pub const Auto: InteractionTrackerClampingOption = InteractionTrackerClampingOption(0i32);
    pub const Disabled: InteractionTrackerClampingOption = InteractionTrackerClampingOption(1i32);
}
impl ::std::convert::From<i32> for InteractionTrackerClampingOption {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for InteractionTrackerClampingOption {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for InteractionTrackerClampingOption {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Composition.Interactions.InteractionTrackerClampingOption;i4)",
    );
}
impl ::windows::runtime::DefaultType for InteractionTrackerClampingOption {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Composition_Interactions`*"]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct InteractionTrackerCustomAnimationStateEnteredArgs(pub ::windows::runtime::IInspectable);
impl InteractionTrackerCustomAnimationStateEnteredArgs {
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn RequestId(&self) -> ::windows::runtime::Result<i32> {
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
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn IsFromBinding(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<
            IInteractionTrackerCustomAnimationStateEnteredArgs2,
        >(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for InteractionTrackerCustomAnimationStateEnteredArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Composition.Interactions.InteractionTrackerCustomAnimationStateEnteredArgs;{7464035c-cfce-56da-9472-420f276bd0a5})" ) ;
}
unsafe impl ::windows::runtime::Interface for InteractionTrackerCustomAnimationStateEnteredArgs {
    type Vtable = IInteractionTrackerCustomAnimationStateEnteredArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1952711516,
        53198,
        22234,
        [148, 114, 66, 15, 39, 107, 208, 165],
    );
}
impl ::windows::runtime::RuntimeName for InteractionTrackerCustomAnimationStateEnteredArgs {
    const NAME: &'static str =
        "Microsoft.UI.Composition.Interactions.InteractionTrackerCustomAnimationStateEnteredArgs";
}
impl ::std::convert::From<InteractionTrackerCustomAnimationStateEnteredArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: InteractionTrackerCustomAnimationStateEnteredArgs) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&InteractionTrackerCustomAnimationStateEnteredArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: &InteractionTrackerCustomAnimationStateEnteredArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for InteractionTrackerCustomAnimationStateEnteredArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &'a InteractionTrackerCustomAnimationStateEnteredArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<InteractionTrackerCustomAnimationStateEnteredArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: InteractionTrackerCustomAnimationStateEnteredArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&InteractionTrackerCustomAnimationStateEnteredArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: &InteractionTrackerCustomAnimationStateEnteredArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for InteractionTrackerCustomAnimationStateEnteredArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a InteractionTrackerCustomAnimationStateEnteredArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for InteractionTrackerCustomAnimationStateEnteredArgs {}
unsafe impl ::std::marker::Sync for InteractionTrackerCustomAnimationStateEnteredArgs {}
#[doc = "*Required features: `UI_Composition_Interactions`*"]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct InteractionTrackerIdleStateEnteredArgs(pub ::windows::runtime::IInspectable);
impl InteractionTrackerIdleStateEnteredArgs {
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn RequestId(&self) -> ::windows::runtime::Result<i32> {
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
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn IsFromBinding(&self) -> ::windows::runtime::Result<bool> {
        let this =
            &::windows::runtime::Interface::cast::<IInteractionTrackerIdleStateEnteredArgs2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for InteractionTrackerIdleStateEnteredArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Composition.Interactions.InteractionTrackerIdleStateEnteredArgs;{199094ab-15fd-539c-97b8-964a8196f777})" ) ;
}
unsafe impl ::windows::runtime::Interface for InteractionTrackerIdleStateEnteredArgs {
    type Vtable = IInteractionTrackerIdleStateEnteredArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        428905643,
        5629,
        21404,
        [151, 184, 150, 74, 129, 150, 247, 119],
    );
}
impl ::windows::runtime::RuntimeName for InteractionTrackerIdleStateEnteredArgs {
    const NAME: &'static str =
        "Microsoft.UI.Composition.Interactions.InteractionTrackerIdleStateEnteredArgs";
}
impl ::std::convert::From<InteractionTrackerIdleStateEnteredArgs> for ::windows::runtime::IUnknown {
    fn from(value: InteractionTrackerIdleStateEnteredArgs) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&InteractionTrackerIdleStateEnteredArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: &InteractionTrackerIdleStateEnteredArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for InteractionTrackerIdleStateEnteredArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &'a InteractionTrackerIdleStateEnteredArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<InteractionTrackerIdleStateEnteredArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: InteractionTrackerIdleStateEnteredArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&InteractionTrackerIdleStateEnteredArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: &InteractionTrackerIdleStateEnteredArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for InteractionTrackerIdleStateEnteredArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a InteractionTrackerIdleStateEnteredArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for InteractionTrackerIdleStateEnteredArgs {}
unsafe impl ::std::marker::Sync for InteractionTrackerIdleStateEnteredArgs {}
#[doc = "*Required features: `UI_Composition_Interactions`*"]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct InteractionTrackerInertiaModifier(pub ::windows::runtime::IInspectable);
impl InteractionTrackerInertiaModifier {
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn PopulatePropertyInfo<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, super::AnimationPropertyInfo>,
    >(
        &self,
        propertyname: Param0,
        propertyinfo: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                propertyinfo.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Compositor(&self) -> ::windows::runtime::Result<super::Compositor> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Compositor>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<super::CompositionPropertySet> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn StartAnimation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, super::CompositionAnimation>,
    >(
        &self,
        propertyname: Param0,
        animation: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                animation.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn StopAnimation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        propertyname: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Comment(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
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
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn SetComment<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn ImplicitAnimations(
        &self,
    ) -> ::windows::runtime::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn SetImplicitAnimations<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ImplicitAnimationCollection>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn StartAnimationGroup<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn StopAnimationGroup<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    #[doc = "*Required features: `UI_Composition_Interactions`, `UI_Dispatching`*"]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::runtime::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn TryGetAnimationController<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        propertyname: Param0,
    ) -> ::windows::runtime::Result<super::AnimationController> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::AnimationController>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for InteractionTrackerInertiaModifier {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Composition.Interactions.InteractionTrackerInertiaModifier;{4d3a0c6b-c508-5029-a47a-cbf64636f010})" ) ;
}
unsafe impl ::windows::runtime::Interface for InteractionTrackerInertiaModifier {
    type Vtable = IInteractionTrackerInertiaModifier_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1295649899,
        50440,
        20521,
        [164, 122, 203, 246, 70, 54, 240, 16],
    );
}
impl ::windows::runtime::RuntimeName for InteractionTrackerInertiaModifier {
    const NAME: &'static str =
        "Microsoft.UI.Composition.Interactions.InteractionTrackerInertiaModifier";
}
impl ::std::convert::From<InteractionTrackerInertiaModifier> for ::windows::runtime::IUnknown {
    fn from(value: InteractionTrackerInertiaModifier) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&InteractionTrackerInertiaModifier> for ::windows::runtime::IUnknown {
    fn from(value: &InteractionTrackerInertiaModifier) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for InteractionTrackerInertiaModifier
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &'a InteractionTrackerInertiaModifier
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<InteractionTrackerInertiaModifier> for ::windows::runtime::IInspectable {
    fn from(value: InteractionTrackerInertiaModifier) -> Self {
        value.0
    }
}
impl ::std::convert::From<&InteractionTrackerInertiaModifier> for ::windows::runtime::IInspectable {
    fn from(value: &InteractionTrackerInertiaModifier) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for InteractionTrackerInertiaModifier
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a InteractionTrackerInertiaModifier
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<InteractionTrackerInertiaModifier> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: InteractionTrackerInertiaModifier) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&InteractionTrackerInertiaModifier> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &InteractionTrackerInertiaModifier) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject>
    for InteractionTrackerInertiaModifier
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject>
    for &InteractionTrackerInertiaModifier
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::std::convert::TryInto::<super::IAnimationObject>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<InteractionTrackerInertiaModifier>
    for ::windows::Foundation::IClosable
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: InteractionTrackerInertiaModifier) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&InteractionTrackerInertiaModifier>
    for ::windows::Foundation::IClosable
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: &InteractionTrackerInertiaModifier) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::Foundation::IClosable>
    for InteractionTrackerInertiaModifier
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::Foundation::IClosable>
    for &InteractionTrackerInertiaModifier
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::Foundation::IClosable> {
        ::std::convert::TryInto::<::windows::Foundation::IClosable>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::From<InteractionTrackerInertiaModifier> for super::CompositionObject {
    fn from(value: InteractionTrackerInertiaModifier) -> Self {
        ::std::convert::Into::<super::CompositionObject>::into(&value)
    }
}
impl ::std::convert::From<&InteractionTrackerInertiaModifier> for super::CompositionObject {
    fn from(value: &InteractionTrackerInertiaModifier) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject>
    for InteractionTrackerInertiaModifier
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::CompositionObject>::into(
            self,
        ))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject>
    for &InteractionTrackerInertiaModifier
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::CompositionObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::std::marker::Send for InteractionTrackerInertiaModifier {}
unsafe impl ::std::marker::Sync for InteractionTrackerInertiaModifier {}
#[doc = "*Required features: `UI_Composition_Interactions`*"]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct InteractionTrackerInertiaMotion(pub ::windows::runtime::IInspectable);
impl InteractionTrackerInertiaMotion {
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Condition(&self) -> ::windows::runtime::Result<super::ExpressionAnimation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ExpressionAnimation>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn SetCondition<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ExpressionAnimation>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Motion(&self) -> ::windows::runtime::Result<super::ExpressionAnimation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ExpressionAnimation>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn SetMotion<'a, Param0: ::windows::runtime::IntoParam<'a, super::ExpressionAnimation>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, super::Compositor>>(
        compositor: Param0,
    ) -> ::windows::runtime::Result<InteractionTrackerInertiaMotion> {
        Self::IInteractionTrackerInertiaMotionStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                compositor.into_param().abi(),
                &mut result__,
            )
            .from_abi::<InteractionTrackerInertiaMotion>(result__)
        })
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn PopulatePropertyInfo<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, super::AnimationPropertyInfo>,
    >(
        &self,
        propertyname: Param0,
        propertyinfo: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                propertyinfo.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Compositor(&self) -> ::windows::runtime::Result<super::Compositor> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Compositor>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<super::CompositionPropertySet> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn StartAnimation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, super::CompositionAnimation>,
    >(
        &self,
        propertyname: Param0,
        animation: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                animation.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn StopAnimation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        propertyname: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Comment(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
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
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn SetComment<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn ImplicitAnimations(
        &self,
    ) -> ::windows::runtime::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn SetImplicitAnimations<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ImplicitAnimationCollection>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn StartAnimationGroup<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn StopAnimationGroup<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    #[doc = "*Required features: `UI_Composition_Interactions`, `UI_Dispatching`*"]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::runtime::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn TryGetAnimationController<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        propertyname: Param0,
    ) -> ::windows::runtime::Result<super::AnimationController> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::AnimationController>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn IInteractionTrackerInertiaMotionStatics<
        R,
        F: FnOnce(&IInteractionTrackerInertiaMotionStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            InteractionTrackerInertiaMotion,
            IInteractionTrackerInertiaMotionStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for InteractionTrackerInertiaMotion {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Composition.Interactions.InteractionTrackerInertiaMotion;{91f662c0-3141-5b5e-862f-cfc60bee8cd6})" ) ;
}
unsafe impl ::windows::runtime::Interface for InteractionTrackerInertiaMotion {
    type Vtable = IInteractionTrackerInertiaMotion_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2448843456,
        12609,
        23390,
        [134, 47, 207, 198, 11, 238, 140, 214],
    );
}
impl ::windows::runtime::RuntimeName for InteractionTrackerInertiaMotion {
    const NAME: &'static str =
        "Microsoft.UI.Composition.Interactions.InteractionTrackerInertiaMotion";
}
impl ::std::convert::From<InteractionTrackerInertiaMotion> for ::windows::runtime::IUnknown {
    fn from(value: InteractionTrackerInertiaMotion) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&InteractionTrackerInertiaMotion> for ::windows::runtime::IUnknown {
    fn from(value: &InteractionTrackerInertiaMotion) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for InteractionTrackerInertiaMotion
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &'a InteractionTrackerInertiaMotion
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<InteractionTrackerInertiaMotion> for ::windows::runtime::IInspectable {
    fn from(value: InteractionTrackerInertiaMotion) -> Self {
        value.0
    }
}
impl ::std::convert::From<&InteractionTrackerInertiaMotion> for ::windows::runtime::IInspectable {
    fn from(value: &InteractionTrackerInertiaMotion) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for InteractionTrackerInertiaMotion
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a InteractionTrackerInertiaMotion
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<InteractionTrackerInertiaMotion> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: InteractionTrackerInertiaMotion) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&InteractionTrackerInertiaMotion> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &InteractionTrackerInertiaMotion) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject>
    for InteractionTrackerInertiaMotion
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject>
    for &InteractionTrackerInertiaMotion
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::std::convert::TryInto::<super::IAnimationObject>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<InteractionTrackerInertiaMotion> for ::windows::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: InteractionTrackerInertiaMotion) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&InteractionTrackerInertiaMotion>
    for ::windows::Foundation::IClosable
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: &InteractionTrackerInertiaMotion) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::Foundation::IClosable>
    for InteractionTrackerInertiaMotion
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::Foundation::IClosable>
    for &InteractionTrackerInertiaMotion
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::Foundation::IClosable> {
        ::std::convert::TryInto::<::windows::Foundation::IClosable>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::From<InteractionTrackerInertiaMotion> for InteractionTrackerInertiaModifier {
    fn from(value: InteractionTrackerInertiaMotion) -> Self {
        ::std::convert::Into::<InteractionTrackerInertiaModifier>::into(&value)
    }
}
impl ::std::convert::From<&InteractionTrackerInertiaMotion> for InteractionTrackerInertiaModifier {
    fn from(value: &InteractionTrackerInertiaMotion) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, InteractionTrackerInertiaModifier>
    for InteractionTrackerInertiaMotion
{
    fn into_param(self) -> ::windows::runtime::Param<'a, InteractionTrackerInertiaModifier> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<InteractionTrackerInertiaModifier>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, InteractionTrackerInertiaModifier>
    for &InteractionTrackerInertiaMotion
{
    fn into_param(self) -> ::windows::runtime::Param<'a, InteractionTrackerInertiaModifier> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<InteractionTrackerInertiaModifier>::into(
                ::std::clone::Clone::clone(self),
            ),
        )
    }
}
impl ::std::convert::From<InteractionTrackerInertiaMotion> for super::CompositionObject {
    fn from(value: InteractionTrackerInertiaMotion) -> Self {
        ::std::convert::Into::<super::CompositionObject>::into(&value)
    }
}
impl ::std::convert::From<&InteractionTrackerInertiaMotion> for super::CompositionObject {
    fn from(value: &InteractionTrackerInertiaMotion) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject>
    for InteractionTrackerInertiaMotion
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::CompositionObject>::into(
            self,
        ))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject>
    for &InteractionTrackerInertiaMotion
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::CompositionObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::std::marker::Send for InteractionTrackerInertiaMotion {}
unsafe impl ::std::marker::Sync for InteractionTrackerInertiaMotion {}
#[doc = "*Required features: `UI_Composition_Interactions`*"]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct InteractionTrackerInertiaNaturalMotion(pub ::windows::runtime::IInspectable);
impl InteractionTrackerInertiaNaturalMotion {
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Condition(&self) -> ::windows::runtime::Result<super::ExpressionAnimation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ExpressionAnimation>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn SetCondition<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ExpressionAnimation>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn NaturalMotion(&self) -> ::windows::runtime::Result<super::ScalarNaturalMotionAnimation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ScalarNaturalMotionAnimation>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn SetNaturalMotion<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ScalarNaturalMotionAnimation>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, super::Compositor>>(
        compositor: Param0,
    ) -> ::windows::runtime::Result<InteractionTrackerInertiaNaturalMotion> {
        Self::IInteractionTrackerInertiaNaturalMotionStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                compositor.into_param().abi(),
                &mut result__,
            )
            .from_abi::<InteractionTrackerInertiaNaturalMotion>(result__)
        })
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn PopulatePropertyInfo<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, super::AnimationPropertyInfo>,
    >(
        &self,
        propertyname: Param0,
        propertyinfo: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                propertyinfo.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Compositor(&self) -> ::windows::runtime::Result<super::Compositor> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Compositor>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<super::CompositionPropertySet> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn StartAnimation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, super::CompositionAnimation>,
    >(
        &self,
        propertyname: Param0,
        animation: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                animation.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn StopAnimation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        propertyname: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Comment(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
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
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn SetComment<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn ImplicitAnimations(
        &self,
    ) -> ::windows::runtime::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn SetImplicitAnimations<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ImplicitAnimationCollection>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn StartAnimationGroup<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn StopAnimationGroup<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    #[doc = "*Required features: `UI_Composition_Interactions`, `UI_Dispatching`*"]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::runtime::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn TryGetAnimationController<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        propertyname: Param0,
    ) -> ::windows::runtime::Result<super::AnimationController> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::AnimationController>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn IInteractionTrackerInertiaNaturalMotionStatics<
        R,
        F: FnOnce(&IInteractionTrackerInertiaNaturalMotionStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            InteractionTrackerInertiaNaturalMotion,
            IInteractionTrackerInertiaNaturalMotionStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for InteractionTrackerInertiaNaturalMotion {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Composition.Interactions.InteractionTrackerInertiaNaturalMotion;{8c7482e0-185d-56b1-b67f-fca4fcd13cd2})" ) ;
}
unsafe impl ::windows::runtime::Interface for InteractionTrackerInertiaNaturalMotion {
    type Vtable = IInteractionTrackerInertiaNaturalMotion_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2356445920,
        6237,
        22193,
        [182, 127, 252, 164, 252, 209, 60, 210],
    );
}
impl ::windows::runtime::RuntimeName for InteractionTrackerInertiaNaturalMotion {
    const NAME: &'static str =
        "Microsoft.UI.Composition.Interactions.InteractionTrackerInertiaNaturalMotion";
}
impl ::std::convert::From<InteractionTrackerInertiaNaturalMotion> for ::windows::runtime::IUnknown {
    fn from(value: InteractionTrackerInertiaNaturalMotion) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&InteractionTrackerInertiaNaturalMotion>
    for ::windows::runtime::IUnknown
{
    fn from(value: &InteractionTrackerInertiaNaturalMotion) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for InteractionTrackerInertiaNaturalMotion
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &'a InteractionTrackerInertiaNaturalMotion
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<InteractionTrackerInertiaNaturalMotion>
    for ::windows::runtime::IInspectable
{
    fn from(value: InteractionTrackerInertiaNaturalMotion) -> Self {
        value.0
    }
}
impl ::std::convert::From<&InteractionTrackerInertiaNaturalMotion>
    for ::windows::runtime::IInspectable
{
    fn from(value: &InteractionTrackerInertiaNaturalMotion) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for InteractionTrackerInertiaNaturalMotion
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a InteractionTrackerInertiaNaturalMotion
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<InteractionTrackerInertiaNaturalMotion> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: InteractionTrackerInertiaNaturalMotion) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&InteractionTrackerInertiaNaturalMotion> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: &InteractionTrackerInertiaNaturalMotion,
    ) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject>
    for InteractionTrackerInertiaNaturalMotion
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject>
    for &InteractionTrackerInertiaNaturalMotion
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::std::convert::TryInto::<super::IAnimationObject>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<InteractionTrackerInertiaNaturalMotion>
    for ::windows::Foundation::IClosable
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: InteractionTrackerInertiaNaturalMotion) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&InteractionTrackerInertiaNaturalMotion>
    for ::windows::Foundation::IClosable
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: &InteractionTrackerInertiaNaturalMotion,
    ) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::Foundation::IClosable>
    for InteractionTrackerInertiaNaturalMotion
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::Foundation::IClosable>
    for &InteractionTrackerInertiaNaturalMotion
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::Foundation::IClosable> {
        ::std::convert::TryInto::<::windows::Foundation::IClosable>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::From<InteractionTrackerInertiaNaturalMotion>
    for InteractionTrackerInertiaModifier
{
    fn from(value: InteractionTrackerInertiaNaturalMotion) -> Self {
        ::std::convert::Into::<InteractionTrackerInertiaModifier>::into(&value)
    }
}
impl ::std::convert::From<&InteractionTrackerInertiaNaturalMotion>
    for InteractionTrackerInertiaModifier
{
    fn from(value: &InteractionTrackerInertiaNaturalMotion) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, InteractionTrackerInertiaModifier>
    for InteractionTrackerInertiaNaturalMotion
{
    fn into_param(self) -> ::windows::runtime::Param<'a, InteractionTrackerInertiaModifier> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<InteractionTrackerInertiaModifier>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, InteractionTrackerInertiaModifier>
    for &InteractionTrackerInertiaNaturalMotion
{
    fn into_param(self) -> ::windows::runtime::Param<'a, InteractionTrackerInertiaModifier> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<InteractionTrackerInertiaModifier>::into(
                ::std::clone::Clone::clone(self),
            ),
        )
    }
}
impl ::std::convert::From<InteractionTrackerInertiaNaturalMotion> for super::CompositionObject {
    fn from(value: InteractionTrackerInertiaNaturalMotion) -> Self {
        ::std::convert::Into::<super::CompositionObject>::into(&value)
    }
}
impl ::std::convert::From<&InteractionTrackerInertiaNaturalMotion> for super::CompositionObject {
    fn from(value: &InteractionTrackerInertiaNaturalMotion) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject>
    for InteractionTrackerInertiaNaturalMotion
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::CompositionObject>::into(
            self,
        ))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject>
    for &InteractionTrackerInertiaNaturalMotion
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::CompositionObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::std::marker::Send for InteractionTrackerInertiaNaturalMotion {}
unsafe impl ::std::marker::Sync for InteractionTrackerInertiaNaturalMotion {}
#[doc = "*Required features: `UI_Composition_Interactions`*"]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct InteractionTrackerInertiaRestingValue(pub ::windows::runtime::IInspectable);
impl InteractionTrackerInertiaRestingValue {
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Condition(&self) -> ::windows::runtime::Result<super::ExpressionAnimation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ExpressionAnimation>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn SetCondition<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ExpressionAnimation>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn RestingValue(&self) -> ::windows::runtime::Result<super::ExpressionAnimation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ExpressionAnimation>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn SetRestingValue<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ExpressionAnimation>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, super::Compositor>>(
        compositor: Param0,
    ) -> ::windows::runtime::Result<InteractionTrackerInertiaRestingValue> {
        Self::IInteractionTrackerInertiaRestingValueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                compositor.into_param().abi(),
                &mut result__,
            )
            .from_abi::<InteractionTrackerInertiaRestingValue>(result__)
        })
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn PopulatePropertyInfo<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, super::AnimationPropertyInfo>,
    >(
        &self,
        propertyname: Param0,
        propertyinfo: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                propertyinfo.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Compositor(&self) -> ::windows::runtime::Result<super::Compositor> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Compositor>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<super::CompositionPropertySet> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn StartAnimation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, super::CompositionAnimation>,
    >(
        &self,
        propertyname: Param0,
        animation: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                animation.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn StopAnimation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        propertyname: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Comment(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
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
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn SetComment<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn ImplicitAnimations(
        &self,
    ) -> ::windows::runtime::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn SetImplicitAnimations<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ImplicitAnimationCollection>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn StartAnimationGroup<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn StopAnimationGroup<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    #[doc = "*Required features: `UI_Composition_Interactions`, `UI_Dispatching`*"]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::runtime::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn TryGetAnimationController<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        propertyname: Param0,
    ) -> ::windows::runtime::Result<super::AnimationController> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::AnimationController>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn IInteractionTrackerInertiaRestingValueStatics<
        R,
        F: FnOnce(&IInteractionTrackerInertiaRestingValueStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            InteractionTrackerInertiaRestingValue,
            IInteractionTrackerInertiaRestingValueStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for InteractionTrackerInertiaRestingValue {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Composition.Interactions.InteractionTrackerInertiaRestingValue;{1a2b20cd-3371-53ff-a560-f4847b467d73})" ) ;
}
unsafe impl ::windows::runtime::Interface for InteractionTrackerInertiaRestingValue {
    type Vtable = IInteractionTrackerInertiaRestingValue_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        439034061,
        13169,
        21503,
        [165, 96, 244, 132, 123, 70, 125, 115],
    );
}
impl ::windows::runtime::RuntimeName for InteractionTrackerInertiaRestingValue {
    const NAME: &'static str =
        "Microsoft.UI.Composition.Interactions.InteractionTrackerInertiaRestingValue";
}
impl ::std::convert::From<InteractionTrackerInertiaRestingValue> for ::windows::runtime::IUnknown {
    fn from(value: InteractionTrackerInertiaRestingValue) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&InteractionTrackerInertiaRestingValue> for ::windows::runtime::IUnknown {
    fn from(value: &InteractionTrackerInertiaRestingValue) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for InteractionTrackerInertiaRestingValue
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &'a InteractionTrackerInertiaRestingValue
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<InteractionTrackerInertiaRestingValue>
    for ::windows::runtime::IInspectable
{
    fn from(value: InteractionTrackerInertiaRestingValue) -> Self {
        value.0
    }
}
impl ::std::convert::From<&InteractionTrackerInertiaRestingValue>
    for ::windows::runtime::IInspectable
{
    fn from(value: &InteractionTrackerInertiaRestingValue) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for InteractionTrackerInertiaRestingValue
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a InteractionTrackerInertiaRestingValue
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<InteractionTrackerInertiaRestingValue> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: InteractionTrackerInertiaRestingValue) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&InteractionTrackerInertiaRestingValue> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &InteractionTrackerInertiaRestingValue) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject>
    for InteractionTrackerInertiaRestingValue
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject>
    for &InteractionTrackerInertiaRestingValue
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::std::convert::TryInto::<super::IAnimationObject>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<InteractionTrackerInertiaRestingValue>
    for ::windows::Foundation::IClosable
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: InteractionTrackerInertiaRestingValue) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&InteractionTrackerInertiaRestingValue>
    for ::windows::Foundation::IClosable
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: &InteractionTrackerInertiaRestingValue) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::Foundation::IClosable>
    for InteractionTrackerInertiaRestingValue
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::Foundation::IClosable>
    for &InteractionTrackerInertiaRestingValue
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::Foundation::IClosable> {
        ::std::convert::TryInto::<::windows::Foundation::IClosable>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::From<InteractionTrackerInertiaRestingValue>
    for InteractionTrackerInertiaModifier
{
    fn from(value: InteractionTrackerInertiaRestingValue) -> Self {
        ::std::convert::Into::<InteractionTrackerInertiaModifier>::into(&value)
    }
}
impl ::std::convert::From<&InteractionTrackerInertiaRestingValue>
    for InteractionTrackerInertiaModifier
{
    fn from(value: &InteractionTrackerInertiaRestingValue) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, InteractionTrackerInertiaModifier>
    for InteractionTrackerInertiaRestingValue
{
    fn into_param(self) -> ::windows::runtime::Param<'a, InteractionTrackerInertiaModifier> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<InteractionTrackerInertiaModifier>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, InteractionTrackerInertiaModifier>
    for &InteractionTrackerInertiaRestingValue
{
    fn into_param(self) -> ::windows::runtime::Param<'a, InteractionTrackerInertiaModifier> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<InteractionTrackerInertiaModifier>::into(
                ::std::clone::Clone::clone(self),
            ),
        )
    }
}
impl ::std::convert::From<InteractionTrackerInertiaRestingValue> for super::CompositionObject {
    fn from(value: InteractionTrackerInertiaRestingValue) -> Self {
        ::std::convert::Into::<super::CompositionObject>::into(&value)
    }
}
impl ::std::convert::From<&InteractionTrackerInertiaRestingValue> for super::CompositionObject {
    fn from(value: &InteractionTrackerInertiaRestingValue) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject>
    for InteractionTrackerInertiaRestingValue
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::CompositionObject>::into(
            self,
        ))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject>
    for &InteractionTrackerInertiaRestingValue
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::CompositionObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::std::marker::Send for InteractionTrackerInertiaRestingValue {}
unsafe impl ::std::marker::Sync for InteractionTrackerInertiaRestingValue {}
#[doc = "*Required features: `UI_Composition_Interactions`*"]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct InteractionTrackerInertiaStateEnteredArgs(pub ::windows::runtime::IInspectable);
impl InteractionTrackerInertiaStateEnteredArgs {
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn ModifiedRestingPosition(
        &self,
    ) -> ::windows::runtime::Result<
        ::windows::Foundation::IReference<::windows::Foundation::Numerics::Vector3>,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            ( :: windows :: runtime :: Interface :: vtable ( this ) .6 ) ( :: std :: mem :: transmute_copy ( this ) , & mut result__ ) . from_abi :: < ::windows::Foundation:: IReference :: < ::windows::Foundation::Numerics:: Vector3 > > ( result__ )
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn ModifiedRestingScale(
        &self,
    ) -> ::windows::runtime::Result<::windows::Foundation::IReference<f32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::IReference<f32>>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn NaturalRestingPosition(
        &self,
    ) -> ::windows::runtime::Result<::windows::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::Numerics::Vector3 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn NaturalRestingScale(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn PositionVelocityInPixelsPerSecond(
        &self,
    ) -> ::windows::runtime::Result<::windows::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::Numerics::Vector3 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn RequestId(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn ScaleVelocityInPercentPerSecond(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn IsInertiaFromImpulse(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<
            IInteractionTrackerInertiaStateEnteredArgs2,
        >(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn IsFromBinding(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<
            IInteractionTrackerInertiaStateEnteredArgs3,
        >(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for InteractionTrackerInertiaStateEnteredArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Composition.Interactions.InteractionTrackerInertiaStateEnteredArgs;{5b76c949-a4d0-5c9d-9292-7013ae9656c7})" ) ;
}
unsafe impl ::windows::runtime::Interface for InteractionTrackerInertiaStateEnteredArgs {
    type Vtable = IInteractionTrackerInertiaStateEnteredArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1534511433,
        42192,
        23709,
        [146, 146, 112, 19, 174, 150, 86, 199],
    );
}
impl ::windows::runtime::RuntimeName for InteractionTrackerInertiaStateEnteredArgs {
    const NAME: &'static str =
        "Microsoft.UI.Composition.Interactions.InteractionTrackerInertiaStateEnteredArgs";
}
impl ::std::convert::From<InteractionTrackerInertiaStateEnteredArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: InteractionTrackerInertiaStateEnteredArgs) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&InteractionTrackerInertiaStateEnteredArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: &InteractionTrackerInertiaStateEnteredArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for InteractionTrackerInertiaStateEnteredArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &'a InteractionTrackerInertiaStateEnteredArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<InteractionTrackerInertiaStateEnteredArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: InteractionTrackerInertiaStateEnteredArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&InteractionTrackerInertiaStateEnteredArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: &InteractionTrackerInertiaStateEnteredArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for InteractionTrackerInertiaStateEnteredArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a InteractionTrackerInertiaStateEnteredArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for InteractionTrackerInertiaStateEnteredArgs {}
unsafe impl ::std::marker::Sync for InteractionTrackerInertiaStateEnteredArgs {}
#[doc = "*Required features: `UI_Composition_Interactions`*"]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct InteractionTrackerInteractingStateEnteredArgs(pub ::windows::runtime::IInspectable);
impl InteractionTrackerInteractingStateEnteredArgs {
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn RequestId(&self) -> ::windows::runtime::Result<i32> {
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
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn IsFromBinding(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<
            IInteractionTrackerInteractingStateEnteredArgs2,
        >(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for InteractionTrackerInteractingStateEnteredArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Composition.Interactions.InteractionTrackerInteractingStateEnteredArgs;{70d29b84-0931-5f17-a8a1-82f8f8782532})" ) ;
}
unsafe impl ::windows::runtime::Interface for InteractionTrackerInteractingStateEnteredArgs {
    type Vtable = IInteractionTrackerInteractingStateEnteredArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1892850564,
        2353,
        24343,
        [168, 161, 130, 248, 248, 120, 37, 50],
    );
}
impl ::windows::runtime::RuntimeName for InteractionTrackerInteractingStateEnteredArgs {
    const NAME: &'static str =
        "Microsoft.UI.Composition.Interactions.InteractionTrackerInteractingStateEnteredArgs";
}
impl ::std::convert::From<InteractionTrackerInteractingStateEnteredArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: InteractionTrackerInteractingStateEnteredArgs) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&InteractionTrackerInteractingStateEnteredArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: &InteractionTrackerInteractingStateEnteredArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for InteractionTrackerInteractingStateEnteredArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &'a InteractionTrackerInteractingStateEnteredArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<InteractionTrackerInteractingStateEnteredArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: InteractionTrackerInteractingStateEnteredArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&InteractionTrackerInteractingStateEnteredArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: &InteractionTrackerInteractingStateEnteredArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for InteractionTrackerInteractingStateEnteredArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a InteractionTrackerInteractingStateEnteredArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for InteractionTrackerInteractingStateEnteredArgs {}
unsafe impl ::std::marker::Sync for InteractionTrackerInteractingStateEnteredArgs {}
#[doc = "*Required features: `UI_Composition_Interactions`*"]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct InteractionTrackerPositionUpdateOption(pub i32);
impl InteractionTrackerPositionUpdateOption {
    pub const Default: InteractionTrackerPositionUpdateOption =
        InteractionTrackerPositionUpdateOption(0i32);
    pub const AllowActiveCustomScaleAnimation: InteractionTrackerPositionUpdateOption =
        InteractionTrackerPositionUpdateOption(1i32);
}
impl ::std::convert::From<i32> for InteractionTrackerPositionUpdateOption {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for InteractionTrackerPositionUpdateOption {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for InteractionTrackerPositionUpdateOption {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Composition.Interactions.InteractionTrackerPositionUpdateOption;i4)",
    );
}
impl ::windows::runtime::DefaultType for InteractionTrackerPositionUpdateOption {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Composition_Interactions`*"]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct InteractionTrackerRequestIgnoredArgs(pub ::windows::runtime::IInspectable);
impl InteractionTrackerRequestIgnoredArgs {
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn RequestId(&self) -> ::windows::runtime::Result<i32> {
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
}
unsafe impl ::windows::runtime::RuntimeType for InteractionTrackerRequestIgnoredArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Composition.Interactions.InteractionTrackerRequestIgnoredArgs;{c276205e-f7a5-5ba2-ad45-d12c3c339149})" ) ;
}
unsafe impl ::windows::runtime::Interface for InteractionTrackerRequestIgnoredArgs {
    type Vtable = IInteractionTrackerRequestIgnoredArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3262521438,
        63397,
        23458,
        [173, 69, 209, 44, 60, 51, 145, 73],
    );
}
impl ::windows::runtime::RuntimeName for InteractionTrackerRequestIgnoredArgs {
    const NAME: &'static str =
        "Microsoft.UI.Composition.Interactions.InteractionTrackerRequestIgnoredArgs";
}
impl ::std::convert::From<InteractionTrackerRequestIgnoredArgs> for ::windows::runtime::IUnknown {
    fn from(value: InteractionTrackerRequestIgnoredArgs) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&InteractionTrackerRequestIgnoredArgs> for ::windows::runtime::IUnknown {
    fn from(value: &InteractionTrackerRequestIgnoredArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for InteractionTrackerRequestIgnoredArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &'a InteractionTrackerRequestIgnoredArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<InteractionTrackerRequestIgnoredArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: InteractionTrackerRequestIgnoredArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&InteractionTrackerRequestIgnoredArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: &InteractionTrackerRequestIgnoredArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for InteractionTrackerRequestIgnoredArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a InteractionTrackerRequestIgnoredArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for InteractionTrackerRequestIgnoredArgs {}
unsafe impl ::std::marker::Sync for InteractionTrackerRequestIgnoredArgs {}
#[doc = "*Required features: `UI_Composition_Interactions`*"]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct InteractionTrackerValuesChangedArgs(pub ::windows::runtime::IInspectable);
impl InteractionTrackerValuesChangedArgs {
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Position(&self) -> ::windows::runtime::Result<::windows::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::Numerics::Vector3 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn RequestId(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Scale(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for InteractionTrackerValuesChangedArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Composition.Interactions.InteractionTrackerValuesChangedArgs;{9b495bed-1cf7-55c1-82b9-8022cbf3c766})" ) ;
}
unsafe impl ::windows::runtime::Interface for InteractionTrackerValuesChangedArgs {
    type Vtable = IInteractionTrackerValuesChangedArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2605276141,
        7415,
        21953,
        [130, 185, 128, 34, 203, 243, 199, 102],
    );
}
impl ::windows::runtime::RuntimeName for InteractionTrackerValuesChangedArgs {
    const NAME: &'static str =
        "Microsoft.UI.Composition.Interactions.InteractionTrackerValuesChangedArgs";
}
impl ::std::convert::From<InteractionTrackerValuesChangedArgs> for ::windows::runtime::IUnknown {
    fn from(value: InteractionTrackerValuesChangedArgs) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&InteractionTrackerValuesChangedArgs> for ::windows::runtime::IUnknown {
    fn from(value: &InteractionTrackerValuesChangedArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for InteractionTrackerValuesChangedArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &'a InteractionTrackerValuesChangedArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<InteractionTrackerValuesChangedArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: InteractionTrackerValuesChangedArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&InteractionTrackerValuesChangedArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: &InteractionTrackerValuesChangedArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for InteractionTrackerValuesChangedArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a InteractionTrackerValuesChangedArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for InteractionTrackerValuesChangedArgs {}
unsafe impl ::std::marker::Sync for InteractionTrackerValuesChangedArgs {}
#[doc = "*Required features: `UI_Composition_Interactions`*"]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct InteractionTrackerVector2InertiaModifier(pub ::windows::runtime::IInspectable);
impl InteractionTrackerVector2InertiaModifier {
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn PopulatePropertyInfo<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, super::AnimationPropertyInfo>,
    >(
        &self,
        propertyname: Param0,
        propertyinfo: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                propertyinfo.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Compositor(&self) -> ::windows::runtime::Result<super::Compositor> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Compositor>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<super::CompositionPropertySet> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn StartAnimation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, super::CompositionAnimation>,
    >(
        &self,
        propertyname: Param0,
        animation: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                animation.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn StopAnimation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        propertyname: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Comment(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
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
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn SetComment<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn ImplicitAnimations(
        &self,
    ) -> ::windows::runtime::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn SetImplicitAnimations<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ImplicitAnimationCollection>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn StartAnimationGroup<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn StopAnimationGroup<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    #[doc = "*Required features: `UI_Composition_Interactions`, `UI_Dispatching`*"]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::runtime::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn TryGetAnimationController<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        propertyname: Param0,
    ) -> ::windows::runtime::Result<super::AnimationController> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::AnimationController>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for InteractionTrackerVector2InertiaModifier {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Composition.Interactions.InteractionTrackerVector2InertiaModifier;{4b8ed310-cb61-5f0a-b99a-940cdd2c42b1})" ) ;
}
unsafe impl ::windows::runtime::Interface for InteractionTrackerVector2InertiaModifier {
    type Vtable = IInteractionTrackerVector2InertiaModifier_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1267651344,
        52065,
        24330,
        [185, 154, 148, 12, 221, 44, 66, 177],
    );
}
impl ::windows::runtime::RuntimeName for InteractionTrackerVector2InertiaModifier {
    const NAME: &'static str =
        "Microsoft.UI.Composition.Interactions.InteractionTrackerVector2InertiaModifier";
}
impl ::std::convert::From<InteractionTrackerVector2InertiaModifier>
    for ::windows::runtime::IUnknown
{
    fn from(value: InteractionTrackerVector2InertiaModifier) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&InteractionTrackerVector2InertiaModifier>
    for ::windows::runtime::IUnknown
{
    fn from(value: &InteractionTrackerVector2InertiaModifier) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for InteractionTrackerVector2InertiaModifier
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &'a InteractionTrackerVector2InertiaModifier
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<InteractionTrackerVector2InertiaModifier>
    for ::windows::runtime::IInspectable
{
    fn from(value: InteractionTrackerVector2InertiaModifier) -> Self {
        value.0
    }
}
impl ::std::convert::From<&InteractionTrackerVector2InertiaModifier>
    for ::windows::runtime::IInspectable
{
    fn from(value: &InteractionTrackerVector2InertiaModifier) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for InteractionTrackerVector2InertiaModifier
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a InteractionTrackerVector2InertiaModifier
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<InteractionTrackerVector2InertiaModifier> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: InteractionTrackerVector2InertiaModifier,
    ) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&InteractionTrackerVector2InertiaModifier>
    for super::IAnimationObject
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: &InteractionTrackerVector2InertiaModifier,
    ) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject>
    for InteractionTrackerVector2InertiaModifier
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject>
    for &InteractionTrackerVector2InertiaModifier
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::std::convert::TryInto::<super::IAnimationObject>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<InteractionTrackerVector2InertiaModifier>
    for ::windows::Foundation::IClosable
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: InteractionTrackerVector2InertiaModifier,
    ) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&InteractionTrackerVector2InertiaModifier>
    for ::windows::Foundation::IClosable
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: &InteractionTrackerVector2InertiaModifier,
    ) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::Foundation::IClosable>
    for InteractionTrackerVector2InertiaModifier
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::Foundation::IClosable>
    for &InteractionTrackerVector2InertiaModifier
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::Foundation::IClosable> {
        ::std::convert::TryInto::<::windows::Foundation::IClosable>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::From<InteractionTrackerVector2InertiaModifier> for super::CompositionObject {
    fn from(value: InteractionTrackerVector2InertiaModifier) -> Self {
        ::std::convert::Into::<super::CompositionObject>::into(&value)
    }
}
impl ::std::convert::From<&InteractionTrackerVector2InertiaModifier> for super::CompositionObject {
    fn from(value: &InteractionTrackerVector2InertiaModifier) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject>
    for InteractionTrackerVector2InertiaModifier
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::CompositionObject>::into(
            self,
        ))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject>
    for &InteractionTrackerVector2InertiaModifier
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::CompositionObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::std::marker::Send for InteractionTrackerVector2InertiaModifier {}
unsafe impl ::std::marker::Sync for InteractionTrackerVector2InertiaModifier {}
#[doc = "*Required features: `UI_Composition_Interactions`*"]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct InteractionTrackerVector2InertiaNaturalMotion(pub ::windows::runtime::IInspectable);
impl InteractionTrackerVector2InertiaNaturalMotion {
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Condition(&self) -> ::windows::runtime::Result<super::ExpressionAnimation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ExpressionAnimation>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn SetCondition<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ExpressionAnimation>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn NaturalMotion(
        &self,
    ) -> ::windows::runtime::Result<super::Vector2NaturalMotionAnimation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Vector2NaturalMotionAnimation>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn SetNaturalMotion<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Vector2NaturalMotionAnimation>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, super::Compositor>>(
        compositor: Param0,
    ) -> ::windows::runtime::Result<InteractionTrackerVector2InertiaNaturalMotion> {
        Self::IInteractionTrackerVector2InertiaNaturalMotionStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                compositor.into_param().abi(),
                &mut result__,
            )
            .from_abi::<InteractionTrackerVector2InertiaNaturalMotion>(result__)
        })
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn PopulatePropertyInfo<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, super::AnimationPropertyInfo>,
    >(
        &self,
        propertyname: Param0,
        propertyinfo: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                propertyinfo.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Compositor(&self) -> ::windows::runtime::Result<super::Compositor> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Compositor>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<super::CompositionPropertySet> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn StartAnimation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, super::CompositionAnimation>,
    >(
        &self,
        propertyname: Param0,
        animation: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                animation.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn StopAnimation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        propertyname: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Comment(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
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
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn SetComment<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn ImplicitAnimations(
        &self,
    ) -> ::windows::runtime::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn SetImplicitAnimations<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ImplicitAnimationCollection>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn StartAnimationGroup<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn StopAnimationGroup<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    #[doc = "*Required features: `UI_Composition_Interactions`, `UI_Dispatching`*"]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::runtime::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn TryGetAnimationController<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        propertyname: Param0,
    ) -> ::windows::runtime::Result<super::AnimationController> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::AnimationController>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn IInteractionTrackerVector2InertiaNaturalMotionStatics<
        R,
        F: FnOnce(
            &IInteractionTrackerVector2InertiaNaturalMotionStatics,
        ) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            InteractionTrackerVector2InertiaNaturalMotion,
            IInteractionTrackerVector2InertiaNaturalMotionStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for InteractionTrackerVector2InertiaNaturalMotion {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Composition.Interactions.InteractionTrackerVector2InertiaNaturalMotion;{097ba1a6-e077-52d1-86d3-38e3f6619ddf})" ) ;
}
unsafe impl ::windows::runtime::Interface for InteractionTrackerVector2InertiaNaturalMotion {
    type Vtable = IInteractionTrackerVector2InertiaNaturalMotion_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        159097254,
        57463,
        21201,
        [134, 211, 56, 227, 246, 97, 157, 223],
    );
}
impl ::windows::runtime::RuntimeName for InteractionTrackerVector2InertiaNaturalMotion {
    const NAME: &'static str =
        "Microsoft.UI.Composition.Interactions.InteractionTrackerVector2InertiaNaturalMotion";
}
impl ::std::convert::From<InteractionTrackerVector2InertiaNaturalMotion>
    for ::windows::runtime::IUnknown
{
    fn from(value: InteractionTrackerVector2InertiaNaturalMotion) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&InteractionTrackerVector2InertiaNaturalMotion>
    for ::windows::runtime::IUnknown
{
    fn from(value: &InteractionTrackerVector2InertiaNaturalMotion) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for InteractionTrackerVector2InertiaNaturalMotion
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &'a InteractionTrackerVector2InertiaNaturalMotion
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<InteractionTrackerVector2InertiaNaturalMotion>
    for ::windows::runtime::IInspectable
{
    fn from(value: InteractionTrackerVector2InertiaNaturalMotion) -> Self {
        value.0
    }
}
impl ::std::convert::From<&InteractionTrackerVector2InertiaNaturalMotion>
    for ::windows::runtime::IInspectable
{
    fn from(value: &InteractionTrackerVector2InertiaNaturalMotion) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for InteractionTrackerVector2InertiaNaturalMotion
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a InteractionTrackerVector2InertiaNaturalMotion
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<InteractionTrackerVector2InertiaNaturalMotion>
    for super::IAnimationObject
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: InteractionTrackerVector2InertiaNaturalMotion,
    ) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&InteractionTrackerVector2InertiaNaturalMotion>
    for super::IAnimationObject
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: &InteractionTrackerVector2InertiaNaturalMotion,
    ) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject>
    for InteractionTrackerVector2InertiaNaturalMotion
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject>
    for &InteractionTrackerVector2InertiaNaturalMotion
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::std::convert::TryInto::<super::IAnimationObject>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<InteractionTrackerVector2InertiaNaturalMotion>
    for ::windows::Foundation::IClosable
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: InteractionTrackerVector2InertiaNaturalMotion,
    ) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&InteractionTrackerVector2InertiaNaturalMotion>
    for ::windows::Foundation::IClosable
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: &InteractionTrackerVector2InertiaNaturalMotion,
    ) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::Foundation::IClosable>
    for InteractionTrackerVector2InertiaNaturalMotion
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::Foundation::IClosable>
    for &InteractionTrackerVector2InertiaNaturalMotion
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::Foundation::IClosable> {
        ::std::convert::TryInto::<::windows::Foundation::IClosable>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::From<InteractionTrackerVector2InertiaNaturalMotion>
    for InteractionTrackerVector2InertiaModifier
{
    fn from(value: InteractionTrackerVector2InertiaNaturalMotion) -> Self {
        ::std::convert::Into::<InteractionTrackerVector2InertiaModifier>::into(&value)
    }
}
impl ::std::convert::From<&InteractionTrackerVector2InertiaNaturalMotion>
    for InteractionTrackerVector2InertiaModifier
{
    fn from(value: &InteractionTrackerVector2InertiaNaturalMotion) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, InteractionTrackerVector2InertiaModifier>
    for InteractionTrackerVector2InertiaNaturalMotion
{
    fn into_param(self) -> ::windows::runtime::Param<'a, InteractionTrackerVector2InertiaModifier> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            InteractionTrackerVector2InertiaModifier,
        >::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, InteractionTrackerVector2InertiaModifier>
    for &InteractionTrackerVector2InertiaNaturalMotion
{
    fn into_param(self) -> ::windows::runtime::Param<'a, InteractionTrackerVector2InertiaModifier> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            InteractionTrackerVector2InertiaModifier,
        >::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<InteractionTrackerVector2InertiaNaturalMotion>
    for super::CompositionObject
{
    fn from(value: InteractionTrackerVector2InertiaNaturalMotion) -> Self {
        ::std::convert::Into::<super::CompositionObject>::into(&value)
    }
}
impl ::std::convert::From<&InteractionTrackerVector2InertiaNaturalMotion>
    for super::CompositionObject
{
    fn from(value: &InteractionTrackerVector2InertiaNaturalMotion) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject>
    for InteractionTrackerVector2InertiaNaturalMotion
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::CompositionObject>::into(
            self,
        ))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject>
    for &InteractionTrackerVector2InertiaNaturalMotion
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::CompositionObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::std::marker::Send for InteractionTrackerVector2InertiaNaturalMotion {}
unsafe impl ::std::marker::Sync for InteractionTrackerVector2InertiaNaturalMotion {}
#[doc = "*Required features: `UI_Composition_Interactions`*"]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct VisualInteractionSource(pub ::windows::runtime::IInspectable);
impl VisualInteractionSource {
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn IsPositionXRailsEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn SetIsPositionXRailsEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn IsPositionYRailsEnabled(&self) -> ::windows::runtime::Result<bool> {
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
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn SetIsPositionYRailsEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn ManipulationRedirectionMode(
        &self,
    ) -> ::windows::runtime::Result<VisualInteractionSourceRedirectionMode> {
        let this = self;
        unsafe {
            let mut result__: VisualInteractionSourceRedirectionMode = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<VisualInteractionSourceRedirectionMode>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn SetManipulationRedirectionMode(
        &self,
        value: VisualInteractionSourceRedirectionMode,
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
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn PositionXChainingMode(&self) -> ::windows::runtime::Result<InteractionChainingMode> {
        let this = self;
        unsafe {
            let mut result__: InteractionChainingMode = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<InteractionChainingMode>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn SetPositionXChainingMode(
        &self,
        value: InteractionChainingMode,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn PositionXSourceMode(&self) -> ::windows::runtime::Result<InteractionSourceMode> {
        let this = self;
        unsafe {
            let mut result__: InteractionSourceMode = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<InteractionSourceMode>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn SetPositionXSourceMode(
        &self,
        value: InteractionSourceMode,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).15)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn PositionYChainingMode(&self) -> ::windows::runtime::Result<InteractionChainingMode> {
        let this = self;
        unsafe {
            let mut result__: InteractionChainingMode = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<InteractionChainingMode>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn SetPositionYChainingMode(
        &self,
        value: InteractionChainingMode,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).17)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn PositionYSourceMode(&self) -> ::windows::runtime::Result<InteractionSourceMode> {
        let this = self;
        unsafe {
            let mut result__: InteractionSourceMode = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<InteractionSourceMode>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn SetPositionYSourceMode(
        &self,
        value: InteractionSourceMode,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).19)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn ScaleChainingMode(&self) -> ::windows::runtime::Result<InteractionChainingMode> {
        let this = self;
        unsafe {
            let mut result__: InteractionChainingMode = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<InteractionChainingMode>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn SetScaleChainingMode(
        &self,
        value: InteractionChainingMode,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).21)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn ScaleSourceMode(&self) -> ::windows::runtime::Result<InteractionSourceMode> {
        let this = self;
        unsafe {
            let mut result__: InteractionSourceMode = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<InteractionSourceMode>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn SetScaleSourceMode(
        &self,
        value: InteractionSourceMode,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).23)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Source(&self) -> ::windows::runtime::Result<super::Visual> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Visual>(result__)
        }
    }
    #[cfg(feature = "UI_Input")]
    #[doc = "*Required features: `UI_Composition_Interactions`, `UI_Input`*"]
    pub fn TryRedirectForManipulation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Input::PointerPoint>,
    >(
        &self,
        pointerpoint: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).25)(
                ::std::mem::transmute_copy(this),
                pointerpoint.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn DeltaPosition(
        &self,
    ) -> ::windows::runtime::Result<::windows::Foundation::Numerics::Vector3> {
        let this = &::windows::runtime::Interface::cast::<IVisualInteractionSource2>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::Numerics::Vector3 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn DeltaScale(&self) -> ::windows::runtime::Result<f32> {
        let this = &::windows::runtime::Interface::cast::<IVisualInteractionSource2>(self)?;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Position(&self) -> ::windows::runtime::Result<::windows::Foundation::Numerics::Vector3> {
        let this = &::windows::runtime::Interface::cast::<IVisualInteractionSource2>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::Numerics::Vector3 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn PositionVelocity(
        &self,
    ) -> ::windows::runtime::Result<::windows::Foundation::Numerics::Vector3> {
        let this = &::windows::runtime::Interface::cast::<IVisualInteractionSource2>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::Numerics::Vector3 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Scale(&self) -> ::windows::runtime::Result<f32> {
        let this = &::windows::runtime::Interface::cast::<IVisualInteractionSource2>(self)?;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn ScaleVelocity(&self) -> ::windows::runtime::Result<f32> {
        let this = &::windows::runtime::Interface::cast::<IVisualInteractionSource2>(self)?;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn ConfigureCenterPointXModifiers<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            ::windows::Foundation::Collections::IIterable<CompositionConditionalValue>,
        >,
    >(
        &self,
        conditionalvalues: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IVisualInteractionSource2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                conditionalvalues.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn ConfigureCenterPointYModifiers<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            ::windows::Foundation::Collections::IIterable<CompositionConditionalValue>,
        >,
    >(
        &self,
        conditionalvalues: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IVisualInteractionSource2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                conditionalvalues.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn ConfigureDeltaPositionXModifiers<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            ::windows::Foundation::Collections::IIterable<CompositionConditionalValue>,
        >,
    >(
        &self,
        conditionalvalues: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IVisualInteractionSource2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                conditionalvalues.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn ConfigureDeltaPositionYModifiers<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            ::windows::Foundation::Collections::IIterable<CompositionConditionalValue>,
        >,
    >(
        &self,
        conditionalvalues: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IVisualInteractionSource2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).15)(
                ::std::mem::transmute_copy(this),
                conditionalvalues.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn ConfigureDeltaScaleModifiers<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            ::windows::Foundation::Collections::IIterable<CompositionConditionalValue>,
        >,
    >(
        &self,
        conditionalvalues: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IVisualInteractionSource2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).16)(
                ::std::mem::transmute_copy(this),
                conditionalvalues.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn PointerWheelConfig(&self) -> ::windows::runtime::Result<InteractionSourceConfiguration> {
        let this = &::windows::runtime::Interface::cast::<IVisualInteractionSource3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<InteractionSourceConfiguration>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, super::Visual>>(
        source: Param0,
    ) -> ::windows::runtime::Result<VisualInteractionSource> {
        Self::IVisualInteractionSourceStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                source.into_param().abi(),
                &mut result__,
            )
            .from_abi::<VisualInteractionSource>(result__)
        })
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn CreateFromIVisualElement<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::IVisualElement>,
    >(
        source: Param0,
    ) -> ::windows::runtime::Result<VisualInteractionSource> {
        Self::IVisualInteractionSourceStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                source.into_param().abi(),
                &mut result__,
            )
            .from_abi::<VisualInteractionSource>(result__)
        })
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn PopulatePropertyInfo<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, super::AnimationPropertyInfo>,
    >(
        &self,
        propertyname: Param0,
        propertyinfo: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                propertyinfo.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Compositor(&self) -> ::windows::runtime::Result<super::Compositor> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Compositor>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<super::CompositionPropertySet> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn StartAnimation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, super::CompositionAnimation>,
    >(
        &self,
        propertyname: Param0,
        animation: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                animation.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn StopAnimation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        propertyname: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Comment(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
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
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn SetComment<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn ImplicitAnimations(
        &self,
    ) -> ::windows::runtime::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn SetImplicitAnimations<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ImplicitAnimationCollection>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn StartAnimationGroup<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn StopAnimationGroup<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::ICompositionAnimationBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    #[doc = "*Required features: `UI_Composition_Interactions`, `UI_Dispatching`*"]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::runtime::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn TryGetAnimationController<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        propertyname: Param0,
    ) -> ::windows::runtime::Result<super::AnimationController> {
        let this = &::windows::runtime::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::AnimationController>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Interactions`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn IVisualInteractionSourceStatics<
        R,
        F: FnOnce(&IVisualInteractionSourceStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            VisualInteractionSource,
            IVisualInteractionSourceStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IVisualInteractionSourceStatics2<
        R,
        F: FnOnce(&IVisualInteractionSourceStatics2) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            VisualInteractionSource,
            IVisualInteractionSourceStatics2,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for VisualInteractionSource {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Composition.Interactions.VisualInteractionSource;{ea595c95-b9cb-5cd4-bb9c-4934ff329063})" ) ;
}
unsafe impl ::windows::runtime::Interface for VisualInteractionSource {
    type Vtable = IVisualInteractionSource_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3931724949,
        47563,
        23764,
        [187, 156, 73, 52, 255, 50, 144, 99],
    );
}
impl ::windows::runtime::RuntimeName for VisualInteractionSource {
    const NAME: &'static str = "Microsoft.UI.Composition.Interactions.VisualInteractionSource";
}
impl ::std::convert::From<VisualInteractionSource> for ::windows::runtime::IUnknown {
    fn from(value: VisualInteractionSource) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&VisualInteractionSource> for ::windows::runtime::IUnknown {
    fn from(value: &VisualInteractionSource) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for VisualInteractionSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &'a VisualInteractionSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<VisualInteractionSource> for ::windows::runtime::IInspectable {
    fn from(value: VisualInteractionSource) -> Self {
        value.0
    }
}
impl ::std::convert::From<&VisualInteractionSource> for ::windows::runtime::IInspectable {
    fn from(value: &VisualInteractionSource) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for VisualInteractionSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a VisualInteractionSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<VisualInteractionSource> for ICompositionInteractionSource {
    type Error = ::windows::runtime::Error;
    fn try_from(value: VisualInteractionSource) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&VisualInteractionSource> for ICompositionInteractionSource {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &VisualInteractionSource) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ICompositionInteractionSource>
    for VisualInteractionSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ICompositionInteractionSource> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ICompositionInteractionSource>
    for &VisualInteractionSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ICompositionInteractionSource> {
        ::std::convert::TryInto::<ICompositionInteractionSource>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<VisualInteractionSource> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: VisualInteractionSource) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&VisualInteractionSource> for super::IAnimationObject {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &VisualInteractionSource) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject> for VisualInteractionSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimationObject> for &VisualInteractionSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimationObject> {
        ::std::convert::TryInto::<super::IAnimationObject>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<VisualInteractionSource> for ::windows::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: VisualInteractionSource) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&VisualInteractionSource> for ::windows::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &VisualInteractionSource) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::Foundation::IClosable>
    for VisualInteractionSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::Foundation::IClosable>
    for &VisualInteractionSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::Foundation::IClosable> {
        ::std::convert::TryInto::<::windows::Foundation::IClosable>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::From<VisualInteractionSource> for super::CompositionObject {
    fn from(value: VisualInteractionSource) -> Self {
        ::std::convert::Into::<super::CompositionObject>::into(&value)
    }
}
impl ::std::convert::From<&VisualInteractionSource> for super::CompositionObject {
    fn from(value: &VisualInteractionSource) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject> for VisualInteractionSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::CompositionObject>::into(
            self,
        ))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::CompositionObject> for &VisualInteractionSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::CompositionObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::CompositionObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::std::marker::Send for VisualInteractionSource {}
unsafe impl ::std::marker::Sync for VisualInteractionSource {}
#[doc = "*Required features: `UI_Composition_Interactions`*"]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct VisualInteractionSourceRedirectionMode(pub i32);
impl VisualInteractionSourceRedirectionMode {
    pub const Off: VisualInteractionSourceRedirectionMode =
        VisualInteractionSourceRedirectionMode(0i32);
    pub const CapableTouchpadOnly: VisualInteractionSourceRedirectionMode =
        VisualInteractionSourceRedirectionMode(1i32);
    pub const PointerWheelOnly: VisualInteractionSourceRedirectionMode =
        VisualInteractionSourceRedirectionMode(2i32);
    pub const CapableTouchpadAndPointerWheel: VisualInteractionSourceRedirectionMode =
        VisualInteractionSourceRedirectionMode(3i32);
}
impl ::std::convert::From<i32> for VisualInteractionSourceRedirectionMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VisualInteractionSourceRedirectionMode {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for VisualInteractionSourceRedirectionMode {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Composition.Interactions.VisualInteractionSourceRedirectionMode;i4)",
    );
}
impl ::windows::runtime::DefaultType for VisualInteractionSourceRedirectionMode {
    type DefaultType = Self;
}