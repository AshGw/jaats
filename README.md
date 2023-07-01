**To Encode a payload run:**
```bash
cargo run -- encode \
  --email 'drip@check.com' \
  --scopes key1=value1,key2=value2,key54=value54 \
  --secret anything \
  --algorithm HS256 \
  --expiration-days 78

```
You can add as many values as needed, this will print out the jwt.
> --algorthm & --expiration-days are optional arguments. defaults to 30 days exp and HS256 for Algorithm. 
It should look like something similar to this
- Header
```json
{
  "typ": "JWT",
  "alg": "HS256"
}
```
- Payload
```json
{
  "email": "drip@check.com",
  "scope": {
    "key54": "value54",
    "key2": "value2",
    "key1": "value1"
  },
  "iat": 1688233213,
  "exp": 1690825213
}
```
<br>

**To Decode a token run:**
```bash
cargo run -- decode --token eyJ0eXAiO..27DBQJASyvQ --secret anything
# --algorithm is optional
```
This will output the payload