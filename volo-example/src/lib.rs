#![feature(impl_trait_in_assoc_type)]

use std::{
    collections::HashMap,
    sync::{Arc, Mutex}, time::Instant,
};

use anyhow::{Ok, anyhow};
use lazy_static::lazy_static;
use pilota::FastStr;
use volo_gen::volo::example::{
    DeleteItemResponse, ItemServicePingResultSend, ItemServiceRequestRecv, PingResponse,
    SetItemResponse,
};
type Db = Arc<Mutex<HashMap<FastStr, FastStr>>>;

lazy_static! {
    static ref DB: Db = Arc::new(Mutex::new(HashMap::new()));
}
pub struct S;

#[volo::async_trait]
impl volo_gen::volo::example::ItemService for S {
    async fn get_item(
        &self,
        _req: volo_gen::volo::example::GetItemRequest,
    ) -> ::core::result::Result<volo_gen::volo::example::GetItemResponse, ::volo_thrift::AnyhowError>
    {
        let a = "get_item";
        println!("{:?}",a);
        println!("{}", _req.key.to_string());
        let db = DB.lock().unwrap();
        let value = db.get(&_req.key);
        match value {
            Some(v) => {
                let mut resp = volo_gen::volo::example::GetItemResponse::default();
                resp.value = v.clone();
                Ok(resp)
            }
            None => Ok(Default::default()),
        }
    }

    async fn post_item(
        &self,
        _req: volo_gen::volo::example::PostItemRequest,
    ) -> ::core::result::Result<volo_gen::volo::example::PostItemResponse, ::volo_thrift::AnyhowError>
    {
        let a = "post_item";
        println!("{:?}",a);
        println!("{}", _req.name);
        Ok(Default::default())
    }

    async fn set_item(
        &self,
        _req: volo_gen::volo::example::SetItemRequest,
    ) -> ::core::result::Result<volo_gen::volo::example::SetItemResponse, ::volo_thrift::AnyhowError>
    {
        let a = "set_item";
        println!("{:?}",a);
        println!("{}:{}", _req.kv.key.to_string(), _req.kv.value.to_string());
        let mut db = DB.lock().unwrap();
        db.insert(_req.kv.key, _req.kv.value);
        Ok(SetItemResponse {
            message: FastStr::from("OK"),
        })
    }

    async fn delete_item(
        &self,
        _req: volo_gen::volo::example::DeleteItemRequest,
    ) -> ::core::result::Result<
        volo_gen::volo::example::DeleteItemResponse,
        ::volo_thrift::AnyhowError,
    > {
        let a = "delete_item";
        println!("{:?}",a);
        let mut db = DB.lock().unwrap();
        let mut count:i64 = 0;
        for k in _req.keys.clone() {
            if db.contains_key(&k) {
                db.remove(&k);
                count += 1;
            }
        }
        Ok(DeleteItemResponse { count })
    }

    async fn ping(
        &self,
        _req: volo_gen::volo::example::PingRequest,
    ) -> ::core::result::Result<volo_gen::volo::example::PingResponse, ::volo_thrift::AnyhowError>
    {
        let a = "ping:";
        println!("{:?}",a);
        let b = "PONG";
        if let Some(v) = _req.message.clone() {
            println!("{}", v.to_string());
        } else {
            println!("{:?}",b);
        }
        Ok(PingResponse {
            message: match _req.message {
                Some(v) => v.clone(),
                None => FastStr::from("PONG"),
            },
        })
    }
}

#[derive(Clone)]
pub struct LogService<S>(S);


#[volo::service]
impl<Cx, Req, S> volo::Service<Cx, Req> for LogService<S>
where
    Req: std::fmt::Debug + Send + 'static,
    S: Send + 'static + volo::Service<Cx, Req> + Sync,
    S::Response: std::fmt::Debug,
    S::Error: std::fmt::Debug,
    Cx: Send + 'static,
    anyhow::Error: Into<S::Error>,
{
    async fn call(&self, cx: &mut Cx, req: Req) -> Result<S::Response, S::Error> {
        let now:Instant = std::time::Instant::now();
        tracing::debug!("Received request {:?}", &req);

        let req_str:String = format!("{:?}", req);
        let req_str:&str = req_str.as_str();

        if req_str.starts_with("Ping") {
            println!("Ping");
            return Err(anyhow!("reject").into());
        }

        let resp = self.0.call(cx, req).await;
        tracing::debug!("Sent response {:?}", &resp);
        tracing::info!("Request took {}ms", now.elapsed().as_millis());
        resp
    }
}

pub struct LogLayer;

impl<S> volo::Layer<S> for LogLayer {
    type Service = LogService<S>;

    fn layer(self, inner: S) -> Self::Service {
        LogService(inner)
    }
}
