mod feature {
  use async_trait::async_trait;

  #[async_trait]
  pub trait Feature: Send + Sync {
    async fn action(&self) -> Result<String, String>;
  }
}

mod my_struct {
  use super::feature::Feature;
  use async_trait::async_trait;
  use std::sync::Arc;

  pub struct MyStruct {
    feature: Arc<dyn Feature>,
  }

  #[async_trait]
  pub trait MyStructTrait: Send + Sync {
    fn new(feature: Arc<dyn Feature>) -> Self;
    async fn action(&self) -> Result<String, String>;
  }

  #[async_trait]
  impl MyStructTrait for MyStruct {
    fn new(feature: Arc<dyn Feature>) -> Self {
      MyStruct { feature }
    }

    async fn action(&self) -> Result<String, String> {
      self.feature.action().await
    }
  }
}

mod feature_implementation01 {
  use super::feature::Feature;
  use async_trait::async_trait;

  #[derive(Debug)]
  pub struct FeatureImplementation01;

  #[async_trait]
  impl Feature for FeatureImplementation01 {
    async fn action(&self) -> Result<String, String> {
      Ok(String::from("Feature 01 - with arc"))
    }
  }
}

mod feature_implementation02 {
  use super::feature::Feature;
  use async_trait::async_trait;

  #[derive(Debug)]
  pub struct FeatureImplementation02;

  #[async_trait]
  impl Feature for FeatureImplementation02 {
    async fn action(&self) -> Result<String, String> {
      Ok(String::from("Feature 02 - with arc"))
    }
  }
}

#[tokio::main]
pub async fn with_arc() {
  use feature_implementation01::FeatureImplementation01;
  use feature_implementation02::FeatureImplementation02;
  use my_struct::{MyStruct, MyStructTrait};
  use std::sync::Arc;

  let feature_implementation_01_instance = FeatureImplementation01;
  let my_struct = MyStruct::new(Arc::new(feature_implementation_01_instance));
  let result = my_struct.action().await.unwrap();
  println!("···············");
  println!("{:#?}", result);
  println!("···············");

  let feature_implementation_02_instance = FeatureImplementation02;
  let my_struct = MyStruct::new(Arc::new(feature_implementation_02_instance));
  let result = my_struct.action().await.unwrap();
  println!("···············");
  println!("{:#?}", result);
  println!("···············");
}
