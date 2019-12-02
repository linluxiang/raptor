// pub struct Context {}

// impl Context {
//     fn get_proxy<T>(&self, grain_id: String, grain: &T) -> T
//     where
//     {
//         let refg = GrainReference{grain_id};
//         let result  = T::from_reference(&refg);
//         result
//     }
//     // GrainType = T;
//     // fn get_proxy<T>(&self, grain_id: String) -> T
//     // where
//     //     T: IGrain,
//     // {
//     //     /*
//     //      public TGrainInterface GetGrain<TGrainInterface>(Guid primaryKey, string grainClassNamePrefix = null) where TGrainInterface : IGrainWithGuidKey
//     //      {
//     //          Type interfaceType = typeof(TGrainInterface);
//     //          var implementation = this.GetGrainClassData(interfaceType, grainClassNamePrefix);
//     //          var grainId = GrainId.GetGrainId(implementation.GetTypeCode(interfaceType), primaryKey, null);
//     //          return this.Cast<TGrainInterface>(this.MakeGrainReferenceFromType(interfaceType, grainId));
//     //      }
//     //     */
//     // }
// }