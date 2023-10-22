# Json responses

### Success
```json
{
	"status": "success",
	"data": DATA HERE
}
```

### Error
```json
{
	"status": "error",
	"message": "Some error message"
}
```

# User

### Register
Request type: **POST**\
Endpoint: `/api/user/register`\
Data type: **x-www-form-urlencoded**\
Fields:
- username
- password

#### Responses
##### Success
**OK 200**
```json
{
    "data": "User successfully registered",
    "status": "success"
}
```

##### Error
**400 Bad Request**
```json
{
    "message": "User with this name already exists",
    "status": "error"
}
```

### Login
Request type: **POST**\
Endpoint: `/api/user/login`\
Data type: **x-www-form-urlencoded**\
Fields:
- username
- password

#### Responses
##### Success
**OK 200**

##### Error
**400 Bad Request**
```json
{
    "message": "Invalid username or password",
    "status": "error"
}
```
**400 Bad Request**
```json
{
    "message": "Already logged in",
    "status": "error"
}
```


### Logout
Request type: **GET**\
Endpoint: `/api/user/logout`

#### Responses
##### Success
**OK 200**

##### Error
**400 Bad Request**
```json
{
    "message": "Not logged in",
    "status": "error"
}
```
