import requests

url = "http://localhost:3030"
headers = {
    'accept': "application/json",
    'content-type': "application/json"
}

json = {
    "jsonrpc": "2.0",
    "method": "verify_transaction",
    "params": {"timestamp": 1549893371, "to": "022337a6ba0c0229fb48469bd49745b200f4cdb35459e7033dbd846bee66ee87be", "sender": "02a03b99517daf92dd3925eaf02cc5b6e9a90314a70baaa22e7e5383b1580df730", "amount": 5, "signature": "304402202f8046faf00d945a74c0f42e7e05c7a8360ff4681d57b524c5da79bc2d2058f80220456fe85f731fa07a17361963198c47f2dfd4ee5b6ea9d9932d8a6626ba53d4fe0000", "data": "Bob sends Alice to 5 eth"},
    "id": 1
}

response = requests.request("POST", url, json=json, headers=headers)

print(response.text)
