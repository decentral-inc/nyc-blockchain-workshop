# Topic 1

## Section 03: Putting it all together

In this section of the seminar, we will build a transaction authentication/authorization server with ECDSA and json-rpc 2.0. We already built interfaces from [Section 02](../rust-crypto) so all you have to do is piece it together

# API endpoints

## verify_transaction

The authentication server has only one endpoint which verifies the transaction.   
  
```  
method: verify_transaction
Returns: whether transaction is valid or not with boolean value
```

See **Example Request** below


### Parameters

`timestamp`: the time when the transaction was processed

`to`: recipient(in public key/ encoded address) of the asset

`sender`: sender of the asset and prover of the transaction

`amount`: amount of the asset to send

`signature`: signature generated from the sender's private(secret) key

`data`: message or reference for the transaction

### Returns

`bool` - whether the transaction is valid(true) or not(false)


### Example Request

```bash
// Request
curl -X POST \
  http://localhost:3031/ \
  -H 'Content-Type: application/json' \
  -H 'Postman-Token: 9e701109-5663-4b24-91d5-183d7a72019c' \
  -H 'cache-control: no-cache' \
  -d '{
	"jsonrpc": "2.0",
	"method": "verify_transaction",
	"params": {"timestamp": 1549893371, "to": "022337a6ba0c0229fb48469bd49745b200f4cdb35459e7033dbd846bee66ee87be", "sender": "02a03b99517daf92dd3925eaf02cc5b6e9a90314a70baaa22e7e5383b1580df730", "amount": 5, "signature": "304402202f8046faf00d945a74c0f42e7e05c7a8360ff4681d57b524c5da79bc2d2058f80220456fe85f731fa07a17361963198c47f2dfd4ee5b6ea9d9932d8a6626ba53d4fe0000", "data": "Bob sends Alice to 5 eth" },
	"id":1
}'

// Result
{
    "jsonrpc": "2.0",
    "result": "true",
    "id": 1
}
```


# Client

A python client is located in the [client folder](./client/). Ready to use (no need to modify it).  
It will be our means of interacting with our rust server.   

To run the client, install request package from pip and locate to the client folder. 
Should be in directory `nyc-blockchain-workshop/Topic1/03_putting_it_all_together/client`  
```bash
pip install request
cd ./client/
```
(note your local path may be different)    
  
  
and run the python code.
```bash
python client.py
# should print "true"

```