mod feature {

  pub trait Feature {
    fn action(&self) -> Result<String, String>;
  }
}

mod my_struct {
  use super::feature::Feature;
  use std::rc::Rc;

  pub struct MyStruct {
    feature: Rc<dyn Feature>,
  }

  pub trait MyStructTrait {
    fn new(feature: Rc<dyn Feature>) -> Self;
    fn action(&self) -> Result<String, String>;
  }

  impl MyStructTrait for MyStruct {
    fn new(feature: Rc<dyn Feature>) -> Self {
      MyStruct { feature }
    }

    fn action(&self) -> Result<String, String> {
      self.feature.action()
    }
  }
}

mod feature_implementation01 {
  use super::feature::Feature;

  #[derive(Debug)]
  pub struct FeatureImplementation01;

  impl Feature for FeatureImplementation01 {
    fn action(&self) -> Result<String, String> {
      Ok(String::from("Feature 01 - with rc"))
    }
  }
}

mod feature_implementation02 {
  use super::feature::Feature;

  #[derive(Debug)]
  pub struct FeatureImplementation02;

  impl Feature for FeatureImplementation02 {
    fn action(&self) -> Result<String, String> {
      Ok(String::from("Feature 02 - with rc"))
    }
  }
}

pub fn with_rc() {
  use feature_implementation01::FeatureImplementation01;
  use feature_implementation02::FeatureImplementation02;
  use my_struct::{MyStruct, MyStructTrait};
  use std::rc::Rc;

  let feature_implementation_01_instance = FeatureImplementation01;
  let my_struct = MyStruct::new(Rc::new(feature_implementation_01_instance));
  let result = my_struct.action().unwrap();
  println!("···············");
  println!("{:#?}", result);
  println!("···············");

  let feature_implementation_02_instance = FeatureImplementation02;
  let my_struct = MyStruct::new(Rc::new(feature_implementation_02_instance));
  let result = my_struct.action().unwrap();
  println!("···············");
  println!("{:#?}", result);
  println!("···············");
}
