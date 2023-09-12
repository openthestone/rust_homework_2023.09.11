#![feature(impl_trait_in_assoc_type)]
use std::collections::HashMap;
use std::sync::Mutex;
use volo::FastStr;
pub struct S{
	pub map: Mutex<HashMap<String,String>>, 
}


use volo_gen::mini_redis::{RequestType,ResponseType,RedisResponse};

#[volo::async_trait]
impl volo_gen::mini_redis::RedisService for S {
	async fn redis_command(
		&self, 
		_req: volo_gen::mini_redis::RedisRequest
	) -> ::core::result::Result<volo_gen::mini_redis::RedisResponse, ::volo_thrift::AnyhowError>{
		match _req.request_type {
			RequestType::Set => {
				self.map.lock().unwrap().insert(_req.key.unwrap().into_string(), _req.value.unwrap().into_string());	
				Ok(RedisResponse{
					value: Some("Ok".into()),
					response_type: ResponseType::Print,
				})
			}
			RequestType::Get => {
				match self.map.lock().unwrap().get(&_req.key.unwrap().into_string()) {
					Some(v) => {
						Ok(RedisResponse{
							value: Some(FastStr::from(v.clone())),
							response_type: ResponseType::Print,
						})
					}
					None => {
						Ok(RedisResponse{
							value: Some("Key not found".into()),
							response_type: ResponseType::Print,
						})
					}
				}
			}
			RequestType::Del => {
				match self.map.lock().unwrap().remove(&_req.key.unwrap().into_string()) {
					Some(_) => {
						Ok(RedisResponse{
							value: Some("Ok".into()),
							response_type: ResponseType::Print,
						})
					}
					None => {
						Ok(RedisResponse{
							value: Some("Key not found".into()),
							response_type: ResponseType::Print,
						})
					}
				}
			}
			RequestType::Ping => {
				Ok(RedisResponse{
					value: Some("PONG".into()),
					response_type: ResponseType::Print,
				})
			}
			RequestType::Subscribe => {Ok(Default::default())}
			RequestType::Publish => {Ok(Default::default())}
		}
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
{
    async fn call(&self, cx: &mut Cx, req: Req) -> Result<S::Response, S::Error> {
        let now = std::time::Instant::now();
        tracing::debug!("Received request {:?}", &req);
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


#[derive(Clone)]
pub struct FilterService<S>(S);

#[volo::service]
impl<Cx, Req, S> volo::Service<Cx, Req> for FilterService<S>
where
    Req: std::fmt::Debug + Send + 'static,
    S: Send + 'static + volo::Service<Cx, Req> + Sync,
    S::Response: std::fmt::Debug,
    S::Error: std::fmt::Debug + From<&'static str>,
    Cx: Send + 'static,

{
    async fn call(&self, cx: &mut Cx, req: Req) -> Result<S::Response, S::Error> {
		let info = format!("{:?}", req);
		if info.contains("Genshin") {
			Err(S::Error::from("Genshin is not allowed"))
		} else {
			self.0.call(cx, req).await
		}
	}
}

pub struct FilterLayer;

impl<S> volo::Layer<S> for FilterLayer {
	type Service = FilterService<S>;

	fn layer(self, inner: S) -> Self::Service {
		FilterService(inner)
	}
}