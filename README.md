### flycheap

- https://developers.google.com/qpx-express/v1/trips/search

### configuration

Place a file `config.toml` in your working directory.

```
email = "notused@yet.com"
google_api_key = "GOOGLE_QPX_EXPRESS_KEY"
requests_per_day = 50
sale_country = "DE"
request_name = "txl_to_otp"

[[trips]]
from = "TXL"
to = "OTP"
dates = ["2016-03-28", "2016-03-29"]

[[trips]]
from = "OTP"
to = "TXL"
dates = ["2016-04-03"]
```
