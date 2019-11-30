pub(crate) trait Message {}

pub trait IGrain {}

pub trait Grain {}

pub(crate) struct GrainReference {
    pub grain_id: String,
}

impl IGrain for GrainReference {}

pub struct Context {}

impl Context {
    pub fn create_proxy<T: IGrain>(&self, grain_id: String) -> T {
        GrainReference { grain_id } as T
        /*
         public TGrainInterface GetGrain<TGrainInterface>(Guid primaryKey, string grainClassNamePrefix = null) where TGrainInterface : IGrainWithGuidKey
         {
             Type interfaceType = typeof(TGrainInterface);
             var implementation = this.GetGrainClassData(interfaceType, grainClassNamePrefix);
             var grainId = GrainId.GetGrainId(implementation.GetTypeCode(interfaceType), primaryKey, null);
             return this.Cast<TGrainInterface>(this.MakeGrainReferenceFromType(interfaceType, grainId));
         }
        */
    }
}
