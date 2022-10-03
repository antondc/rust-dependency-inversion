mod feature {
  pub trait Feature {
    fn action(&self) -> String;
  }
}

mod my_struct {
  use super::feature::Feature;

  pub struct MyStruct<T> {
    feature: T,
  }

  impl<T: Feature> MyStruct<T> {
    pub fn new(feature: T) -> MyStruct<T> {
      MyStruct { feature }
    }

    pub fn action(&self) -> String {
      self.feature.action()
    }
  }
}

mod feature_implementation01 {
  use super::feature::Feature;

  pub struct FeatureImplementation01;

  impl Feature for FeatureImplementation01 {
    fn action(&self) -> String {
      String::from("Feature 01 - with generics")
    }
  }
}

mod feature_implementation02 {
  use super::feature::Feature;

  pub struct FeatureImplementation02;

  impl Feature for FeatureImplementation02 {
    fn action(&self) -> String {
      String::from("Feature 02 - with generics")
    }
  }
}

pub fn with_generics() {
  use feature_implementation01::FeatureImplementation01;
  use feature_implementation02::FeatureImplementation02;
  use my_struct::MyStruct;

  let feature_implementation_01_instance = FeatureImplementation01;
  // If required, we may pass the type of the implementation into the generic with turbofish syntax
  let my_struct = MyStruct::<FeatureImplementation01>::new(feature_implementation_01_instance);
  let result = my_struct.action();
  println!("···············");
  println!("{:#?}", result);
  println!("···············");

  // If required, we may pass the type of the implementation into the generic with turbofish syntax
  let feature_implementation_02_instance = FeatureImplementation02;
  let my_struct = MyStruct::<FeatureImplementation02>::new(feature_implementation_02_instance);
  let result = my_struct.action();
  println!("···············");
  println!("{:#?}", result);
  println!("···············");
}
