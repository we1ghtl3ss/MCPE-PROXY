mod client;
mod proxy;

use proxy::Proxy;


fn main() {
    let mut proxy: Proxy = Proxy::new("65.21.166.170:19132".parse().unwrap());
    proxy.listen();
}
