#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[doc = "*Required features: `UI_Xaml_Media_Media3D`*"]
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct CompositeTransform3D(pub ::windows::runtime::IInspectable);
impl CompositeTransform3D {
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
            CompositeTransform3D,
            ::windows::runtime::IActivationFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Media3D`*"]
    pub fn CenterX(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Media3D`*"]
    pub fn SetCenterX(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Media3D`*"]
    pub fn CenterY(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Media3D`*"]
    pub fn SetCenterY(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Media3D`*"]
    pub fn CenterZ(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Media3D`*"]
    pub fn SetCenterZ(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Media3D`*"]
    pub fn RotationX(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Media3D`*"]
    pub fn SetRotationX(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Media3D`*"]
    pub fn RotationY(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Media3D`*"]
    pub fn SetRotationY(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).15)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Media3D`*"]
    pub fn RotationZ(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Media3D`*"]
    pub fn SetRotationZ(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).17)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Media3D`*"]
    pub fn ScaleX(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Media3D`*"]
    pub fn SetScaleX(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).19)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Media3D`*"]
    pub fn ScaleY(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Media3D`*"]
    pub fn SetScaleY(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).21)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Media3D`*"]
    pub fn ScaleZ(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Media3D`*"]
    pub fn SetScaleZ(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).23)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Media3D`*"]
    pub fn TranslateX(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Media3D`*"]
    pub fn SetTranslateX(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).25)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Media3D`*"]
    pub fn TranslateY(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).26)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Media3D`*"]
    pub fn SetTranslateY(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).27)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Media3D`*"]
    pub fn TranslateZ(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).28)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Media3D`*"]
    pub fn SetTranslateZ(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).29)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Media3D`*"]
    pub fn CenterXProperty() -> ::windows::runtime::Result<super::super::DependencyProperty> {
        Self::ICompositeTransform3DStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Media_Media3D`*"]
    pub fn CenterYProperty() -> ::windows::runtime::Result<super::super::DependencyProperty> {
        Self::ICompositeTransform3DStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Media_Media3D`*"]
    pub fn CenterZProperty() -> ::windows::runtime::Result<super::super::DependencyProperty> {
        Self::ICompositeTransform3DStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Media_Media3D`*"]
    pub fn RotationXProperty() -> ::windows::runtime::Result<super::super::DependencyProperty> {
        Self::ICompositeTransform3DStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Media_Media3D`*"]
    pub fn RotationYProperty() -> ::windows::runtime::Result<super::super::DependencyProperty> {
        Self::ICompositeTransform3DStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Media_Media3D`*"]
    pub fn RotationZProperty() -> ::windows::runtime::Result<super::super::DependencyProperty> {
        Self::ICompositeTransform3DStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Media_Media3D`*"]
    pub fn ScaleXProperty() -> ::windows::runtime::Result<super::super::DependencyProperty> {
        Self::ICompositeTransform3DStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Media_Media3D`*"]
    pub fn ScaleYProperty() -> ::windows::runtime::Result<super::super::DependencyProperty> {
        Self::ICompositeTransform3DStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Media_Media3D`*"]
    pub fn ScaleZProperty() -> ::windows::runtime::Result<super::super::DependencyProperty> {
        Self::ICompositeTransform3DStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Media_Media3D`*"]
    pub fn TranslateXProperty() -> ::windows::runtime::Result<super::super::DependencyProperty> {
        Self::ICompositeTransform3DStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Media_Media3D`*"]
    pub fn TranslateYProperty() -> ::windows::runtime::Result<super::super::DependencyProperty> {
        Self::ICompositeTransform3DStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Media_Media3D`*"]
    pub fn TranslateZProperty() -> ::windows::runtime::Result<super::super::DependencyProperty> {
        Self::ICompositeTransform3DStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Media_Media3D`*"]
    pub fn GetValue<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
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
    #[doc = "*Required features: `UI_Xaml_Media_Media3D`*"]
    pub fn SetValue<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>,
    >(
        &self,
        dp: Param0,
        value: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Media3D`*"]
    pub fn ClearValue<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Media3D`*"]
    pub fn ReadLocalValue<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
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
    #[doc = "*Required features: `UI_Xaml_Media_Media3D`*"]
    pub fn GetAnimationBaseValue<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
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
    #[doc = "*Required features: `UI_Xaml_Media_Media3D`*"]
    pub fn RegisterPropertyChangedCallback<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::DependencyPropertyChangedCallback>,
    >(
        &self,
        dp: Param0,
        callback: Param1,
    ) -> ::windows::runtime::Result<i64> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
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
    #[doc = "*Required features: `UI_Xaml_Media_Media3D`*"]
    pub fn UnregisterPropertyChangedCallback<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
        token: i64,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                token,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Media3D`*"]
    pub fn Dispatcher(&self) -> ::windows::runtime::Result<::windows::UI::Core::CoreDispatcher> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
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
    #[doc = "*Required features: `UI_Xaml_Media_Media3D`, `UI_Dispatching`*"]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    pub fn ICompositeTransform3DStatics<
        R,
        F: FnOnce(&ICompositeTransform3DStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            CompositeTransform3D,
            ICompositeTransform3DStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CompositeTransform3D {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Media.Media3D.CompositeTransform3D;{cbaf163f-c254-5dcf-8ae4-40e21ce1b4ca})" ) ;
}
unsafe impl ::windows::runtime::Interface for CompositeTransform3D {
    type Vtable = ICompositeTransform3D_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3417249343,
        49748,
        24015,
        [138, 228, 64, 226, 28, 225, 180, 202],
    );
}
impl ::windows::runtime::RuntimeName for CompositeTransform3D {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.Media3D.CompositeTransform3D";
}
impl ::core::convert::From<CompositeTransform3D> for ::windows::runtime::IUnknown {
    fn from(value: CompositeTransform3D) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CompositeTransform3D> for ::windows::runtime::IUnknown {
    fn from(value: &CompositeTransform3D) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for CompositeTransform3D {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &'a CompositeTransform3D
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CompositeTransform3D> for ::windows::runtime::IInspectable {
    fn from(value: CompositeTransform3D) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CompositeTransform3D> for ::windows::runtime::IInspectable {
    fn from(value: &CompositeTransform3D) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for CompositeTransform3D
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a CompositeTransform3D
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<CompositeTransform3D> for Transform3D {
    fn from(value: CompositeTransform3D) -> Self {
        ::core::convert::Into::<Transform3D>::into(&value)
    }
}
impl ::core::convert::From<&CompositeTransform3D> for Transform3D {
    fn from(value: &CompositeTransform3D) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, Transform3D> for CompositeTransform3D {
    fn into_param(self) -> ::windows::runtime::Param<'a, Transform3D> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<Transform3D>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, Transform3D> for &CompositeTransform3D {
    fn into_param(self) -> ::windows::runtime::Param<'a, Transform3D> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<Transform3D>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
impl ::core::convert::From<CompositeTransform3D> for super::super::DependencyObject {
    fn from(value: CompositeTransform3D) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&CompositeTransform3D> for super::super::DependencyObject {
    fn from(value: &CompositeTransform3D) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::super::DependencyObject>
    for CompositeTransform3D
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::DependencyObject> {
        ::windows::runtime::Param::Owned(
            ::core::convert::Into::<super::super::DependencyObject>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::super::DependencyObject>
    for &CompositeTransform3D
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::DependencyObject> {
        ::windows::runtime::Param::Owned(
            ::core::convert::Into::<super::super::DependencyObject>::into(
                ::core::clone::Clone::clone(self),
            ),
        )
    }
}
unsafe impl ::core::marker::Send for CompositeTransform3D {}
unsafe impl ::core::marker::Sync for CompositeTransform3D {}
#[repr(transparent)]
#[doc(hidden)]
pub struct ICompositeTransform3D(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICompositeTransform3D {
    type Vtable = ICompositeTransform3D_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3417249343,
        49748,
        24015,
        [138, 228, 64, 226, 28, 225, 180, 202],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositeTransform3D_abi(
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
        result__: *mut f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: f64,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICompositeTransform3DStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICompositeTransform3DStatics {
    type Vtable = ICompositeTransform3DStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3058516353,
        27016,
        23878,
        [133, 138, 34, 77, 183, 8, 157, 196],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositeTransform3DStatics_abi(
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
        result__: *mut ::windows::runtime::RawPtr,
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
        result__: *mut ::windows::runtime::RawPtr,
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
        result__: *mut ::windows::runtime::RawPtr,
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
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMatrix3DHelper(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMatrix3DHelper {
    type Vtable = IMatrix3DHelper_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3532692449,
        39976,
        23352,
        [182, 60, 136, 232, 56, 100, 69, 51],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IMatrix3DHelper_abi(
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
pub struct IMatrix3DHelperStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMatrix3DHelperStatics {
    type Vtable = IMatrix3DHelperStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2467185787,
        9820,
        24045,
        [158, 100, 87, 184, 147, 60, 85, 195],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IMatrix3DHelperStatics_abi(
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
        result__: *mut Matrix3D,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        matrix1: Matrix3D,
        matrix2: Matrix3D,
        result__: *mut Matrix3D,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        m11: f64,
        m12: f64,
        m13: f64,
        m14: f64,
        m21: f64,
        m22: f64,
        m23: f64,
        m24: f64,
        m31: f64,
        m32: f64,
        m33: f64,
        m34: f64,
        offsetx: f64,
        offsety: f64,
        offsetz: f64,
        m44: f64,
        result__: *mut Matrix3D,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        target: Matrix3D,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        target: Matrix3D,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        target: Matrix3D,
        result__: *mut Matrix3D,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPerspectiveTransform3D(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPerspectiveTransform3D {
    type Vtable = IPerspectiveTransform3D_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1074187334,
        26702,
        21738,
        [164, 33, 218, 229, 176, 85, 107, 133],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IPerspectiveTransform3D_abi(
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
        result__: *mut f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: f64,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPerspectiveTransform3DStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPerspectiveTransform3DStatics {
    type Vtable = IPerspectiveTransform3DStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        991341197,
        3810,
        23878,
        [167, 35, 220, 142, 92, 28, 11, 25],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IPerspectiveTransform3DStatics_abi(
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
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITransform3D(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITransform3D {
    type Vtable = ITransform3D_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2951366977,
        11849,
        21308,
        [159, 143, 44, 18, 110, 249, 137, 58],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransform3D_abi(
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
pub struct ITransform3DFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITransform3DFactory {
    type Vtable = ITransform3DFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2613895329,
        4268,
        21273,
        [189, 241, 84, 141, 46, 90, 229, 4],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransform3DFactory_abi(
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
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `UI_Xaml_Media_Media3D`*"]
pub struct Matrix3D {
    pub M11: f64,
    pub M12: f64,
    pub M13: f64,
    pub M14: f64,
    pub M21: f64,
    pub M22: f64,
    pub M23: f64,
    pub M24: f64,
    pub M31: f64,
    pub M32: f64,
    pub M33: f64,
    pub M34: f64,
    pub OffsetX: f64,
    pub OffsetY: f64,
    pub OffsetZ: f64,
    pub M44: f64,
}
impl Matrix3D {}
impl ::core::default::Default for Matrix3D {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for Matrix3D {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("Matrix3D")
            .field("M11", &self.M11)
            .field("M12", &self.M12)
            .field("M13", &self.M13)
            .field("M14", &self.M14)
            .field("M21", &self.M21)
            .field("M22", &self.M22)
            .field("M23", &self.M23)
            .field("M24", &self.M24)
            .field("M31", &self.M31)
            .field("M32", &self.M32)
            .field("M33", &self.M33)
            .field("M34", &self.M34)
            .field("OffsetX", &self.OffsetX)
            .field("OffsetY", &self.OffsetY)
            .field("OffsetZ", &self.OffsetZ)
            .field("M44", &self.M44)
            .finish()
    }
}
impl ::core::cmp::PartialEq for Matrix3D {
    fn eq(&self, other: &Self) -> bool {
        self.M11 == other.M11
            && self.M12 == other.M12
            && self.M13 == other.M13
            && self.M14 == other.M14
            && self.M21 == other.M21
            && self.M22 == other.M22
            && self.M23 == other.M23
            && self.M24 == other.M24
            && self.M31 == other.M31
            && self.M32 == other.M32
            && self.M33 == other.M33
            && self.M34 == other.M34
            && self.OffsetX == other.OffsetX
            && self.OffsetY == other.OffsetY
            && self.OffsetZ == other.OffsetZ
            && self.M44 == other.M44
    }
}
impl ::core::cmp::Eq for Matrix3D {}
unsafe impl ::windows::runtime::Abi for Matrix3D {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for Matrix3D {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"struct(Microsoft.UI.Xaml.Media.Media3D.Matrix3D;f8;f8;f8;f8;f8;f8;f8;f8;f8;f8;f8;f8;f8;f8;f8;f8)" ) ;
}
impl ::windows::runtime::DefaultType for Matrix3D {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Xaml_Media_Media3D`*"]
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct Matrix3DHelper(pub ::windows::runtime::IInspectable);
impl Matrix3DHelper {
    #[doc = "*Required features: `UI_Xaml_Media_Media3D`*"]
    pub fn Identity() -> ::windows::runtime::Result<Matrix3D> {
        Self::IMatrix3DHelperStatics(|this| unsafe {
            let mut result__: Matrix3D = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<Matrix3D>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Media_Media3D`*"]
    pub fn Multiply<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, Matrix3D>,
        Param1: ::windows::runtime::IntoParam<'a, Matrix3D>,
    >(
        matrix1: Param0,
        matrix2: Param1,
    ) -> ::windows::runtime::Result<Matrix3D> {
        Self::IMatrix3DHelperStatics(|this| unsafe {
            let mut result__: Matrix3D = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                matrix1.into_param().abi(),
                matrix2.into_param().abi(),
                &mut result__,
            )
            .from_abi::<Matrix3D>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Media_Media3D`*"]
    pub fn FromElements(
        m11: f64,
        m12: f64,
        m13: f64,
        m14: f64,
        m21: f64,
        m22: f64,
        m23: f64,
        m24: f64,
        m31: f64,
        m32: f64,
        m33: f64,
        m34: f64,
        offsetx: f64,
        offsety: f64,
        offsetz: f64,
        m44: f64,
    ) -> ::windows::runtime::Result<Matrix3D> {
        Self::IMatrix3DHelperStatics(|this| unsafe {
            let mut result__: Matrix3D = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                m11,
                m12,
                m13,
                m14,
                m21,
                m22,
                m23,
                m24,
                m31,
                m32,
                m33,
                m34,
                offsetx,
                offsety,
                offsetz,
                m44,
                &mut result__,
            )
            .from_abi::<Matrix3D>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Media_Media3D`*"]
    pub fn GetHasInverse<'a, Param0: ::windows::runtime::IntoParam<'a, Matrix3D>>(
        target: Param0,
    ) -> ::windows::runtime::Result<bool> {
        Self::IMatrix3DHelperStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                target.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Media_Media3D`*"]
    pub fn GetIsIdentity<'a, Param0: ::windows::runtime::IntoParam<'a, Matrix3D>>(
        target: Param0,
    ) -> ::windows::runtime::Result<bool> {
        Self::IMatrix3DHelperStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                target.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Media_Media3D`*"]
    pub fn Invert<'a, Param0: ::windows::runtime::IntoParam<'a, Matrix3D>>(
        target: Param0,
    ) -> ::windows::runtime::Result<Matrix3D> {
        Self::IMatrix3DHelperStatics(|this| unsafe {
            let mut result__: Matrix3D = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                target.into_param().abi(),
                &mut result__,
            )
            .from_abi::<Matrix3D>(result__)
        })
    }
    pub fn IMatrix3DHelperStatics<
        R,
        F: FnOnce(&IMatrix3DHelperStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            Matrix3DHelper,
            IMatrix3DHelperStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for Matrix3DHelper {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Media.Media3D.Matrix3DHelper;{d2909be1-9c28-5b38-b63c-88e838644533})" ) ;
}
unsafe impl ::windows::runtime::Interface for Matrix3DHelper {
    type Vtable = IMatrix3DHelper_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3532692449,
        39976,
        23352,
        [182, 60, 136, 232, 56, 100, 69, 51],
    );
}
impl ::windows::runtime::RuntimeName for Matrix3DHelper {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.Media3D.Matrix3DHelper";
}
impl ::core::convert::From<Matrix3DHelper> for ::windows::runtime::IUnknown {
    fn from(value: Matrix3DHelper) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&Matrix3DHelper> for ::windows::runtime::IUnknown {
    fn from(value: &Matrix3DHelper) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for Matrix3DHelper {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a Matrix3DHelper {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<Matrix3DHelper> for ::windows::runtime::IInspectable {
    fn from(value: Matrix3DHelper) -> Self {
        value.0
    }
}
impl ::core::convert::From<&Matrix3DHelper> for ::windows::runtime::IInspectable {
    fn from(value: &Matrix3DHelper) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for Matrix3DHelper {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a Matrix3DHelper
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for Matrix3DHelper {}
unsafe impl ::core::marker::Sync for Matrix3DHelper {}
#[doc = "*Required features: `UI_Xaml_Media_Media3D`*"]
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct PerspectiveTransform3D(pub ::windows::runtime::IInspectable);
impl PerspectiveTransform3D {
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
            PerspectiveTransform3D,
            ::windows::runtime::IActivationFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Media3D`*"]
    pub fn Depth(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Media3D`*"]
    pub fn SetDepth(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Media3D`*"]
    pub fn OffsetX(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Media3D`*"]
    pub fn SetOffsetX(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Media3D`*"]
    pub fn OffsetY(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Media3D`*"]
    pub fn SetOffsetY(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Media3D`*"]
    pub fn DepthProperty() -> ::windows::runtime::Result<super::super::DependencyProperty> {
        Self::IPerspectiveTransform3DStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Media_Media3D`*"]
    pub fn OffsetXProperty() -> ::windows::runtime::Result<super::super::DependencyProperty> {
        Self::IPerspectiveTransform3DStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Media_Media3D`*"]
    pub fn OffsetYProperty() -> ::windows::runtime::Result<super::super::DependencyProperty> {
        Self::IPerspectiveTransform3DStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Media_Media3D`*"]
    pub fn GetValue<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
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
    #[doc = "*Required features: `UI_Xaml_Media_Media3D`*"]
    pub fn SetValue<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>,
    >(
        &self,
        dp: Param0,
        value: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Media3D`*"]
    pub fn ClearValue<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Media3D`*"]
    pub fn ReadLocalValue<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
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
    #[doc = "*Required features: `UI_Xaml_Media_Media3D`*"]
    pub fn GetAnimationBaseValue<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
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
    #[doc = "*Required features: `UI_Xaml_Media_Media3D`*"]
    pub fn RegisterPropertyChangedCallback<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::DependencyPropertyChangedCallback>,
    >(
        &self,
        dp: Param0,
        callback: Param1,
    ) -> ::windows::runtime::Result<i64> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
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
    #[doc = "*Required features: `UI_Xaml_Media_Media3D`*"]
    pub fn UnregisterPropertyChangedCallback<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
        token: i64,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                token,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Media3D`*"]
    pub fn Dispatcher(&self) -> ::windows::runtime::Result<::windows::UI::Core::CoreDispatcher> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
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
    #[doc = "*Required features: `UI_Xaml_Media_Media3D`, `UI_Dispatching`*"]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    pub fn IPerspectiveTransform3DStatics<
        R,
        F: FnOnce(&IPerspectiveTransform3DStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            PerspectiveTransform3D,
            IPerspectiveTransform3DStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PerspectiveTransform3D {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Media.Media3D.PerspectiveTransform3D;{4006cc46-684e-54ea-a421-dae5b0556b85})" ) ;
}
unsafe impl ::windows::runtime::Interface for PerspectiveTransform3D {
    type Vtable = IPerspectiveTransform3D_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1074187334,
        26702,
        21738,
        [164, 33, 218, 229, 176, 85, 107, 133],
    );
}
impl ::windows::runtime::RuntimeName for PerspectiveTransform3D {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.Media3D.PerspectiveTransform3D";
}
impl ::core::convert::From<PerspectiveTransform3D> for ::windows::runtime::IUnknown {
    fn from(value: PerspectiveTransform3D) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PerspectiveTransform3D> for ::windows::runtime::IUnknown {
    fn from(value: &PerspectiveTransform3D) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for PerspectiveTransform3D
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &'a PerspectiveTransform3D
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PerspectiveTransform3D> for ::windows::runtime::IInspectable {
    fn from(value: PerspectiveTransform3D) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PerspectiveTransform3D> for ::windows::runtime::IInspectable {
    fn from(value: &PerspectiveTransform3D) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for PerspectiveTransform3D
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a PerspectiveTransform3D
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<PerspectiveTransform3D> for Transform3D {
    fn from(value: PerspectiveTransform3D) -> Self {
        ::core::convert::Into::<Transform3D>::into(&value)
    }
}
impl ::core::convert::From<&PerspectiveTransform3D> for Transform3D {
    fn from(value: &PerspectiveTransform3D) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, Transform3D> for PerspectiveTransform3D {
    fn into_param(self) -> ::windows::runtime::Param<'a, Transform3D> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<Transform3D>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, Transform3D> for &PerspectiveTransform3D {
    fn into_param(self) -> ::windows::runtime::Param<'a, Transform3D> {
        ::windows::runtime::Param::Owned(::core::convert::Into::<Transform3D>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
impl ::core::convert::From<PerspectiveTransform3D> for super::super::DependencyObject {
    fn from(value: PerspectiveTransform3D) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&PerspectiveTransform3D> for super::super::DependencyObject {
    fn from(value: &PerspectiveTransform3D) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::super::DependencyObject>
    for PerspectiveTransform3D
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::DependencyObject> {
        ::windows::runtime::Param::Owned(
            ::core::convert::Into::<super::super::DependencyObject>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::super::DependencyObject>
    for &PerspectiveTransform3D
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::DependencyObject> {
        ::windows::runtime::Param::Owned(
            ::core::convert::Into::<super::super::DependencyObject>::into(
                ::core::clone::Clone::clone(self),
            ),
        )
    }
}
unsafe impl ::core::marker::Send for PerspectiveTransform3D {}
unsafe impl ::core::marker::Sync for PerspectiveTransform3D {}
#[doc = "*Required features: `UI_Xaml_Media_Media3D`*"]
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct Transform3D(pub ::windows::runtime::IInspectable);
impl Transform3D {
    #[doc = "*Required features: `UI_Xaml_Media_Media3D`*"]
    pub fn GetValue<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
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
    #[doc = "*Required features: `UI_Xaml_Media_Media3D`*"]
    pub fn SetValue<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>,
    >(
        &self,
        dp: Param0,
        value: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Media3D`*"]
    pub fn ClearValue<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Media3D`*"]
    pub fn ReadLocalValue<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
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
    #[doc = "*Required features: `UI_Xaml_Media_Media3D`*"]
    pub fn GetAnimationBaseValue<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
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
    #[doc = "*Required features: `UI_Xaml_Media_Media3D`*"]
    pub fn RegisterPropertyChangedCallback<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::DependencyPropertyChangedCallback>,
    >(
        &self,
        dp: Param0,
        callback: Param1,
    ) -> ::windows::runtime::Result<i64> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
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
    #[doc = "*Required features: `UI_Xaml_Media_Media3D`*"]
    pub fn UnregisterPropertyChangedCallback<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
        token: i64,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                token,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Media_Media3D`*"]
    pub fn Dispatcher(&self) -> ::windows::runtime::Result<::windows::UI::Core::CoreDispatcher> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
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
    #[doc = "*Required features: `UI_Xaml_Media_Media3D`, `UI_Dispatching`*"]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for Transform3D {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Media.Media3D.Transform3D;{afea4941-2e49-533c-9f8f-2c126ef9893a})",
    );
}
unsafe impl ::windows::runtime::Interface for Transform3D {
    type Vtable = ITransform3D_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2951366977,
        11849,
        21308,
        [159, 143, 44, 18, 110, 249, 137, 58],
    );
}
impl ::windows::runtime::RuntimeName for Transform3D {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.Media3D.Transform3D";
}
impl ::core::convert::From<Transform3D> for ::windows::runtime::IUnknown {
    fn from(value: Transform3D) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&Transform3D> for ::windows::runtime::IUnknown {
    fn from(value: &Transform3D) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for Transform3D {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a Transform3D {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<Transform3D> for ::windows::runtime::IInspectable {
    fn from(value: Transform3D) -> Self {
        value.0
    }
}
impl ::core::convert::From<&Transform3D> for ::windows::runtime::IInspectable {
    fn from(value: &Transform3D) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for Transform3D {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a Transform3D {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<Transform3D> for super::super::DependencyObject {
    fn from(value: Transform3D) -> Self {
        ::core::convert::Into::<super::super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&Transform3D> for super::super::DependencyObject {
    fn from(value: &Transform3D) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::super::DependencyObject> for Transform3D {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::DependencyObject> {
        ::windows::runtime::Param::Owned(
            ::core::convert::Into::<super::super::DependencyObject>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::super::DependencyObject> for &Transform3D {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::DependencyObject> {
        ::windows::runtime::Param::Owned(
            ::core::convert::Into::<super::super::DependencyObject>::into(
                ::core::clone::Clone::clone(self),
            ),
        )
    }
}
unsafe impl ::core::marker::Send for Transform3D {}
unsafe impl ::core::marker::Sync for Transform3D {}
