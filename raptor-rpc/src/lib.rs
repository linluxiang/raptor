// use async_trait::async_trait;
// use serde::{Deserialize, Serialize};
// use serde::de::DeserializeOwned;
// use erased_serde::{Deserializer, Serialize};

// use serde_traitobject::{Serialize, Deserialize};

// trait IGrain {
//     fn from_reference<T>(proxy: &dyn IGrainReference) -> <Self as IGrainGat>::ConcreteGrainType
//     where Self: IGrainGat<ConcreteGrainType = T>
//     {
//         Self::make_reference(proxy)
//     }
// }

// trait IGrainGat {
//     type ConcreteGrainType;
//     fn make_reference(proxy: &dyn IGrainReference) -> Self::ConcreteGrainType;
// }

// pub trait Return {}

// pub trait Error {}

// pub trait Args<'de>: Serialize + Deserializer<'de> {}

// #[async_trait]
// pub trait IGrain {
//     async fn invoke_method(&self, method_name: String, args: &dyn Args) -> Result<&dyn Return, &dyn Error>;
// }

// #[derive(Serialize, Deserialize)]
// pub(crate) struct GrainReference<'a> {
//     pub grain_id: String,
//     // #[serde(with = "serde_traitobject")]
//     grain_proxy: &'a dyn IGrain
// }

// #[async_trait]
// pub trait IGrain {
//     type Args: Serialize + DeserializeOwned;
//     type Return: Serialize + DeserializeOwned;
//     type Error: Serialize + DeserializeOwned;
//     async fn invoke_method<'a>(
//         &self,
//         method_name: String,
//         args: Self::Args,
//     ) -> Result<Self::Return, Self::Error>
//     where
//         Self: Sized;
// }

// pub(crate) struct GrainReference {
//     pub grain_id: String,
//     grain_proxy: dyn IGrain,
// }

// trait IGrain {

// }

// impl IGrainReference for GrainReference {
// }
