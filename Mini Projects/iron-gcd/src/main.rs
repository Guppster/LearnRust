extern crate iron;
#[macro_use] extern crate mime;

use iron::prelude::*;
use iron::status;

fn main()
{
        println!("Serving on http://localhost:3000...");
        Iron::new(get_form).http("localhost:3000").unwrap();
}

#[allow(unused_variables)]
fn get_form(request: &mut Request) -> IronResult 14 fn get_form(request: &mut Request) -> IronResult <Response>
{

}

