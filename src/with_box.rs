mod feature {
  pub trait Feature {
    fn action(&self) -> String;
  }
}

mod my_struct {
  use super::feature::Feature;
  use super::my_service::MyService;
  use std::rc::Rc;

  pub struct MyStruct {
    feature: Rc<dyn Feature>,
  }

  impl MyStruct {
    pub fn new(feature: Rc<dyn Feature>) -> MyStruct {
      MyStruct { feature }
    }

    pub fn action(self) -> String {
      let my_service = MyService::new(self.feature.clone());
      println!("{}", my_service.action());
      // Perform some action with my_service...

      self.feature.action().to_owned()
    }
  }
}

mod my_service {
  use super::feature::Feature;
  use std::rc::Rc;

  pub struct MyService {
    feature: Rc<dyn Feature>,
  }

  impl MyService {
    pub fn new(feature: Rc<dyn Feature>) -> MyService {
      MyService { feature }
    }
    pub fn action(&self) -> String {
      let result = self.feature.action();

      format!("{}, {}", result, "from my_service")
    }
  }
}

mod feature_implementation01 {
  use super::feature::Feature;

  #[derive(Debug, Clone)]
  pub struct FeatureImplementation01;

  impl Feature for FeatureImplementation01 {
    fn action(&self) -> String {
      String::from("Feature 01 - with box")
    }
  }
}

mod feature_implementation02 {
  use super::feature::Feature;

  #[derive(Debug)]
  pub struct FeatureImplementation02;

  impl Feature for FeatureImplementation02 {
    fn action(&self) -> String {
      String::from("Feature 02 - with box")
    }
  }
}

pub fn with_box() {
  use feature_implementation01::FeatureImplementation01;
  use feature_implementation02::FeatureImplementation02;
  use my_struct::MyStruct;
  use std::rc::Rc;

  let feature_implementation_01_instance = FeatureImplementation01;
  let my_struct = MyStruct::new(Rc::new(feature_implementation_01_instance));
  let result = my_struct.action();
  println!("···············");
  println!("{:#?}", result);
  println!("···············");

  let feature_implementation_02_instance = FeatureImplementation02;
  let my_struct = MyStruct::new(Rc::new(feature_implementation_02_instance));
  let result = my_struct.action();
  println!("···············");
  println!("{:#?}", result);
  println!("···············");
}
