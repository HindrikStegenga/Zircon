pub trait Asset {}

pub trait AssetLoader {
    fn asset_type(&self) -> &'static str;
    fn load_asset<A: Asset>(&self, bytes: &[u8], asset_destination: &mut A) -> bool;
}

impl<A> Asset for A {}

impl<'a, T: ?Sized> AssetLoader for Box<T>
where
    T: AssetLoader,
{
    fn asset_type(&self) -> &'static str {
        (**self).asset_type()
    }

    fn load_asset<A: Asset>(&self, bytes: &[u8], asset_destination: &mut A) -> bool {
        (**self).load_asset(bytes, asset_destination)
    }
}

/////////////////////////////////////////////////////////////////////
// This is an object-safe equivalent that interoperates seamlessly.

pub(crate) trait ErasedAssetLoader {
    fn asset_type(&self) -> &'static str;
    fn erased_load_asset(&self, bytes: &[u8], asset_destination: &mut dyn Asset) -> bool;
}

impl AssetLoader for dyn ErasedAssetLoader {
    // Depending on the trait method signatures and the upstream
    // impls, could also implement for:
    //
    //   - &'a ErasedGeneric
    //   - &'a (ErasedGeneric + Send)
    //   - &'a (ErasedGeneric + Sync)
    //   - &'a (ErasedGeneric + Send + Sync)
    //   - Box<ErasedGeneric>
    //   - Box<ErasedGeneric + Send>
    //   - Box<ErasedGeneric + Sync>
    //   - Box<ErasedGeneric + Send + Sync>
    fn load_asset<A: Asset>(&self, bytes: &[u8], mut asset_destination: &mut A) -> bool {
        self.erased_load_asset(bytes, &mut asset_destination)
    }

    fn asset_type(&self) -> &'static str {
        self.asset_type()
    }
}

impl<T> ErasedAssetLoader for T
where
    T: AssetLoader,
{
    fn erased_load_asset(&self, bytes: &[u8], mut asset_destination: &mut dyn Asset) -> bool {
        self.load_asset(bytes, &mut asset_destination)
    }
    fn asset_type(&self) -> &'static str {
        self.asset_type()
    }
}

fn test() {
    struct T;

    struct S;
    impl AssetLoader for S {
        fn asset_type(&self) -> &'static str {
            "json"
        }
        fn load_asset<Q: Asset>(&self, _bytes: &[u8], _asset_destination: &mut Q) -> bool {
            false
        }
    }

    // Construct a trait object.
    let trait_object: Box<dyn ErasedAssetLoader> = Box::new(S);

    // Seamlessly invoke the generic method on the trait object.
    //
    // THIS LINE LOOKS LIKE MAGIC. We have a value of type trait
    // object and we are invoking a generic method on it.
    let bytes = [];
    trait_object.load_asset(&bytes, &mut T);
}
