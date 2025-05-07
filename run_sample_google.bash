echo "./target/release/dnsget  --help"
echo "./target/release/dnsget  --record-type ALL  google.com --resolver 127.0.0.1:53 "
echo "./target/release/dnsget  --record-type ALL  google.com "
echo "./target/release/dnsget  --record-type ALL  google.com "
[ ! -e /usr/local/bin/dnsget ] && echo "sudo ln -s $(pwd)/target/release/dnsget /usr/local/bin/dnsget "
[ ! -f ./target/release/dnsget ] && echo "cargo build --release"
./target/release/dnsget  --record-type ALL  google.com
