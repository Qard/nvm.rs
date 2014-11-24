extern crate hyper;

use self::hyper::client::Request;
use self::hyper::Url;

#[deriving(Show)]
pub enum ApiError {
  None,
  InvalidUrl,
  UnableToConnect,
  InvalidHeaders,
  RequestFailure,
  InvalidResponse
}

fn get (url: &str) -> Result<String, ApiError> {
  let url = match Url::parse(url) {
    Ok(url) => url,
    Err(e) => return Err(ApiError::InvalidUrl)
  };

  let req = match Request::get(url) {
    Ok(req) => req,
    Err(e) => return Err(ApiError::UnableToConnect)
  };

  let after_headers = match req.start() {
    Ok(req) => req,
    Err(e) => return Err(ApiError::InvalidHeaders)
  };

  let mut after_send = match after_headers.send() {
    Ok(req) => req,
    Err(e) => return Err(ApiError::RequestFailure)
  };

  match after_send.read_to_string() {
    Ok(req) => Ok(req),
    Err(e) => return Err(ApiError::InvalidResponse)
  }
}

pub fn remote_versions () -> Result<Vec<String>, ApiError> {
  let res = try!(get("https://semver.io/node/versions"));
  Ok(res.lines().map(|s| s.into_string()).collect::<Vec<String>>())
}

pub fn stable () -> Result<String, ApiError> {
  get("https://semver.io/node/stable")
}

pub fn unstable () -> Result<String, ApiError> {
  get("https://semver.io/node/unstable")
}
