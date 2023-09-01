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
``<empty body with 200 status code>``

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
#### Expected response
##### In case of signed up
``<empty body with 200 status code>``
##### In case of not signed up
``nothing``

---
## Show user posts
will give bookmarked first
### Request
``token is required``
#### Method
``POST``
#### Expected response
```json
{
  "posts": [
    {
      "post_id": 1,
      "title": "OMG its so big",
      "description": "story about the best pizza in the world",
      "user_id": 1,
      "bookmarked": true
    },
    {
      "post_id": 2,
      "title": "OMG its so nice",
      "description": "story about the best test task - nice web",
      "user_id": 1,
      "bookmarked": false
    }
  ]
}

```
---
## Create user post
### Request
``token is required``
#### Method
``POST``
#### Expected response
``<empty body with 200 status code>``
#### Expected request
```json
{
  "title": "OMG, BURN THEM ALL",
  "description": "There was a strong theory that Daenerys Targaryen would go full-on Mad Queen rage-monster﻿ ﻿at some point in the series. But tonight it finally happened in truly horrendous fashion: she just decided to set King's Landing on fire. So...what triggered her massacre? What pushed her over the edge?"
}
```
---
## Delete user post
### Request
``token is required``
#### Method
``POST``
#### Path
``/removepost/<post_id>``
#### Expected response
``<empty body with 200 status code>``

---

## Bookmark post
### Request
``token is required``
#### Method
``POST``
#### Path
``/bookmarkpost/<post_id>/<true or false>``
#### Expected response
``<empty body with 200 status code>``

---
# Errors
### If something goes wrong, response will be
```json
{
  "error": "<error_description>"
}
```