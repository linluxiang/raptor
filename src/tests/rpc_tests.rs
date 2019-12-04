// use async_trait::async_trait;
// use raptor_core::abstracts::*;
// use serde::{Deserialize, Serialize};

// struct EchoImp;

// #[async_trait]
// pub trait Echo: IGrain {
//     async fn echo(&self, msg: String) -> String;
// }

// #[async_trait]
// impl Echo for EchoImp {
//     async fn echo(&self, msg: String) -> String {
//         String::from("456")
//     }
// }

// struct Err {}

// #[async_trait]
// impl IGrain for EchoImp {
//     async fn invoke_method<'a, Args, Return, Error>(
//         &self,
//         method_name: String,
//         args: Args,
//     ) -> Result<Return, Error>
//     where
//         Args: Serialize + Deserialize<'a>,
//         Return: Serialize + Deserialize<'a>,
//         Error: Serialize + Deserialize<'a>,
//         Self: Sized,
//     {
//     }
// }
// // // ![IGrain]
// // // implement Echo for GrainReference
// // pub trait Echo {
// //     // ![GrainMethod]
// //     fn echo(&self) -> str;
// // }

// async fn test_call_a_grain() {
//     // let context = Context {};
//     //let instance: Echo = context.create_proxy(String::from("1"));
//     // instance.echo().await?
//     let client = EchoImp {};
//     let result = client.echo(String::from("123")).await;
// }
