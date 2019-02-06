import requests

url = "http://localhost:3030"
headers = {
    'accept': "application/json",
    'content-type': "application/json"
}

payload = "{\"jsonrpc\": \"2.0\",\"method\": \"say_hello\",\"params\": {\"name\": \"Hyungsuk Kang\"},\"id\":1}"

response = requests.request("POST", url, data=payload, headers=headers)

print(response.text)