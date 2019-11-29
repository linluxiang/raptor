pub trait Message {

}

pub trait IGrain {
}

pub trait Grain {

}

pub struct Context {

}

impl Context {
    pub fn CreateProxy<T: IGrain>() -> dyn IGrain {
       T() 
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