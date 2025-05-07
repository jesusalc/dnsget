# DNSget

DNS Get info. Queries UDP port 53 for DNS records.

## Installation

compile and link

```sh
cargo build --release
sudo ln -s $(pwd)/target/release/dnsget /usr/local/bin/dnsget
```


## Example 1 localhost
```sh
$ dnsget -r 0.0.0.0:53 -t A dev.test

Questions:
A: dev.test.
Answer Records:
A: 127.0.0.1 (TTL 86400)


```
## Example 2 localhost
```sh
$ dnsget -r 0.0.0.0:53 -t ALL dev.test
--- A RECORDS ---

Questions:
A: dev.test.
Answer Records:
A: 127.0.0.1 (TTL 86400)
--- AAAA RECORDS ---

Questions:
AAAA: dev.test.
--- CNAME RECORDS ---

Questions:
CNAME: dev.test.
--- NS RECORDS ---

Questions:
NS: dev.test.
Answer Records:
NS: ns1.dev.test. (TTL 86400)
--- SOA RECORDS ---

Questions:
SOA: dev.test.
Answer Records:
SOA: ns1.dev.test. hostmaster.dev.test. (serial 2025050501, refresh 10800, retry 15, expire 604800) (TTL 86400)
--- MX RECORDS ---

Questions:
MX: dev.test.
Answer Records:
MX: 10 mail.dev.test. (TTL 86400)
--- TXT RECORDS ---

Questions:
TXT: dev.test.
--- PTR RECORDS ---

Questions:
PTR: dev.test.
--- SRV RECORDS ---

Questions:
SRV: dev.test.


```
## Example 2 google.com

```sh
$: dnsget -t ALL google.com
--- A RECORDS ---

Questions:
A: google.com.
Answer Records:
A: 142.250.186.46 (TTL 133)
--- AAAA RECORDS ---

Questions:
AAAA: google.com.
Answer Records:
AAAA: 2a00:1450:4001:803::200e (TTL 166)
--- CNAME RECORDS ---

Questions:
CNAME: google.com.
--- NS RECORDS ---

Questions:
NS: google.com.
Answer Records:
NS: ns4.google.com. (TTL 338718)
NS: ns2.google.com. (TTL 338718)
NS: ns1.google.com. (TTL 338718)
NS: ns3.google.com. (TTL 338718)
--- SOA RECORDS ---

Questions:
SOA: google.com.
Answer Records:
SOA: ns1.google.com. dns-admin.google.com. (serial 755294604, refresh 900, retry 900, expire 1800) (TTL 36)
--- MX RECORDS ---

Questions:
MX: google.com.
Answer Records:
MX: 10 smtp.google.com. (TTL 126)
--- TXT RECORDS ---

Questions:
TXT: google.com.
No DNS records returned.
--- PTR RECORDS ---

Questions:
PTR: google.com.
--- SRV RECORDS ---

Questions:
SRV: google.com.
```

