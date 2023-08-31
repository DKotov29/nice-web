## Sign up
### Request
#### Method
``POST``
#### Params
``username``  
``password``
#### Example request
```json  
{  
  "username": "SomeUser",  
  "password": "WiTHSOMEGre@tPASSWORD"  
}  
```  
#### Expected response
```http request
nothing
```
---
## Sign in
### Request
#### Method
``POST``
#### Params
``username``  
``password``
#### Example request
```json  
{  
  "username": "SomeUser",  
  "password": "WiTHSOMEGre@tPASSWORD"  
}  
```  
#### Expected response
```json  
{  
  "token": "<token>"  
}  
```  
---
## Sign out
### Request
#### Method
``POST``
#### Params
``no params``
#### Example request
#### Expected response
##### In case of signed up
``<empty body with 200 status code>``
##### In case of not signed up
``nothing``

---
# Errors
### If something goes wrong, response will be
```json
{
  "error": "<error_description>"
}
```