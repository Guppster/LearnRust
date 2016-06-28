extern crate iron;
extern crate router;
extern crate urlencoded;
#[macro_use] extern crate mime;

use iron::prelude::*;
use iron::status;
use router::Router;
use std::str::FromStr;
use urlencoded::UrlEncodedBody;

fn main()
{
	let mut router = Router::new();

	router.get("/", get_form);
	router.post("/gcd", post_gcd);

        println!("Serving on http://localhost:3000...");
        Iron::new(router).http("localhost:3000").unwrap();
}

#[allow(unused_variables)]
fn get_form(request: &mut Request) -> IronResult <Response>
{
        let mut response = Response::new();

        response.set_mut(status::Ok);
        response.set_mut(mime!(Text/Html; Charset=Utf8));
        response.set_mut(r#"
                        <title>GCD Calculator</title>
                        <form action="/gcd" method="post">
                        <input type="text" name="n"/>
                        <input type="text" name="n"/>
                        <button type="submit">Compute GCD</button>
                        </form>
                        "#);

        Ok(response)
}

fn post_gcd(request: &mut Request) -> IronResult<Response>
{
	let mut response = Response::new();
}
