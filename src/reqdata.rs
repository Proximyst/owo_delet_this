use rocket::{request::{FromRequest, Outcome, Request}, http::Status};
use std::net::IpAddr;

pub struct ReqData {
    pub ip: IpAddr,
}

impl<'a, 'r> FromRequest<'a, 'r> for ReqData {
    type Error = &'static str;

    fn from_request(req: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let ip = req.client_ip().or_else(|| req.remote().map(|i| i.ip()));
        let ip = match ip {
            Some(i) => i,
            None => return Outcome::Failure((Status::raw(500), "couldn't construct a ReqData")),
        };

        Outcome::Success(ReqData {
            ip,
        })
    } 
}