use crate::requests::*;
use crate::response::*;
use crate::error::Result;
use crate::http::Method;
use serde::de::DeserializeOwned;

#[derive(Debug)]
pub struct Client {
    base_url: String,
    token: String,
    client: ureq::Agent,
}

pub fn new(base_url: &str, token: &str) -> Client {
    let client = ureq::Agent::new();
    Client{base_url: base_url.into(), token: token.into(), client}
}

impl Client {
    pub fn call<R: Request>(&self, request: R) -> Result<R::Response> {
        let url = format!("{}{}", self.base_url, request.endpoint());
        let mut req = match R::METHOD {
            Method::Head => self.client.head(&url),
            Method::Get => self.client.get(&url),
            Method::Post => self.client.post(&url),
            Method::Put => self.client.put(&url),
            Method::Delete => self.client.delete(&url),
        };

        req = req.set("Authorization", &format!("Bearer {}", self.token));
        req = req.set("accept", "application/vnd.api+json");
        req = req.set("Content-Type", "application/json");

        for (k, v) in request.headers().iter() {
            req = req.set(k,v);
        }

        let resp = match request.body() {
            Some(body) => {
                println!("{:#?}", body);
                req.send_json(body)
            },
            None => req.call(),
        };

        match resp {
            Ok(resp) => {
                eprintln!("{}", url);

                if resp.status() == 204 {
                    Ok(R::Response::default()) 
                } else {
                    /*
                    let str_result = resp.into_string().unwrap();
                    println!("{:#?}", str_result);
                    Ok(R::Response::default())
                    */
                    let result: R::Response = resp.into_json()
                       .expect("json conversion failed on us");
                    Ok(result)
                }
            },
            Err(err) => {
                if let Some(body) = request.body() {
                    eprintln!("Body: {:#?}", body);
                }
                Err(err.into())
            },
        }
    }

    pub fn fetch_all<R, T>(&self, mut request: R) -> Result<Vec<T>>
    where
        R: Request<Response = PaginatedResponse<Vec<T>>> + Paginated + Clone,
        T: DeserializeOwned,
    {
        let mut current_page = 1;
        let mut all_items = Vec::new();

        loop {
            request.set_page(current_page);
            let response = self.call(request.clone())?;
            all_items.extend(response.data.into_iter());

            if request.get_page() < request.max_page() {
                current_page += 1;
            } else {
                break; // we're done
            }
        }

        Ok(all_items)
    }
}

