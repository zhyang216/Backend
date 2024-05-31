import requests as re
import json
import config
from config import session_requests

data = {
  "name": "test6",
  "amount": "5000",
  "currency_id": "1",
  "portfolio_type": "0",
  "position": [
        {"base_currency_id": "1", "quote_currency_id": "1"},
        {"base_currency_id": "2", "quote_currency_id": "2"},
        {"base_currency_id": "3", "quote_currency_id": "3"}
    ]
}

res = session_requests.post(config.url + 'portfolio', json=data)
print(res)
print(json.loads(res.text))