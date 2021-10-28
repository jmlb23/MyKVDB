use api::api::Api;

fn main() {
    let api = Api::new("127.0.0.1:8080");
    api.poll();
}
