use jsonrpc_http_server::ServerBuilder;
use api::api::ApiImpl;
use api::api::Api;
use std::env;

fn main() {
    let vector = env::args().collect::<Vec<String>>();
    let args = vector.iter().map(|x| x.as_str()).collect::<Vec<&str>>();
    let (_, tail) = args.split_at(1);
    match tail {
        [] => run_server("127.0.0.1", "8080"),
        ["--help", ..] => print_help(),
        ["--port", port] => run_server("127.0.0.1", port),
        ["--host", host] => run_server(host, "8080"),
        ["--host", host, "--port", port, ..] | ["--port", port, "--host", host, ..] => {
            run_server(host, port)
        }
        _ => {
            println!("KVDB: unrecognized option `{}`", tail.join(" "));
            print_help();
        }
    };
}

fn run_server(host: &str, port: &str) {
    let mut io = jsonrpc_core::IoHandler::new();
	io.extend_with(ApiImpl::to_delegate(ApiImpl::new()));
    let server = ServerBuilder::new(io)
		.threads(3)
		.start_http(&format!("{}:{}",host,port).parse().unwrap())
		.unwrap();

	server.wait();
}

fn print_help() {
    print!("usage: MKDB [--port NUMBER] [--host HOST]")
}
