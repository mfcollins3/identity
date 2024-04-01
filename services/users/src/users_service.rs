// Copyright 2024 Michael F. Collins, III
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
// IN THE SOFTWARE.

use crate::authentication_service::AuthenticationService;
use clap::Parser;
use tonic::transport::Server;
use crate::users::authentication_service_server::AuthenticationServiceServer;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct UsersService {
    #[arg(short, long, env = "APP_PORT", default_value = "40000", help = "the TCP/IP port to listen for incoming requests on")]
    port: u16,
}

impl UsersService {
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Listening on port {}", self.port);
        let addr = format!("127.0.0.1:{}", self.port).parse().unwrap();
        let authentication_service = AuthenticationService::default();
        Server::builder()
            .add_service(AuthenticationServiceServer::new(authentication_service))
            .serve(addr)
            .await?;
    
        Ok(())
    }
}
