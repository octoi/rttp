# RTTP 🦀

A RUST based CLI application which parse json to send API requests.

- Blazingly fast ⚡
- Automate requests with a json file 🚀
- Memory efficient 🤯

## Idea

API testing is one of the common task among programmers. We use different tools like [Postman](https://www.postman.com/), [Insomnia](https://insomnia.rest/products/insomnia), [Hoppscotch](https://hoppscotch.io/) etc to get our crap done.
The problem is these apps take a lot of memory, sometimes it is over complicated for our dummy todo rest api.

So the idea of RTTP is simplify these process, you can create a json file with request data, and pass it to the program, kaboom done 🙂

## Documentation

As mentioned RTTP is a RUST based CLI program which parse JSON files & send request accordingly.

RTTP accepts :

1. Single file with 1 request
2. Multiple files with 1 request
3. Single file with many requests
4. Multiple files with many requests

### Run program

```bash
# Using cargo
cargo run ./request.json
cargo run ./request1.json ./request2.json

# Using RTTP
rttp ./request.json
rttp ./request1.json ./request2.json
```

### Craft request file

| Key         | Type              | Description                                                                          |
| ----------- | ----------------- | ------------------------------------------------------------------------------------ |
| name        | string            | request name                                                                         |
| url         | string (required) | Endpoint of your API                                                                 |
| method      | string (required) | Method of request (GET, POST etc)                                                    |
| headers     | Map<any, any>     | An object of key value pairs with your headers                                       |
| body        | Map<any, any>     | An object of key value pairs with your request body                                  |
| request     | Array<request>    | A list of requests with fields mentioned in this table (without the field `request`) |
| show_error  | boolean           | Show error or not                                                                    |
| show_output | boolean           | Show output or not                                                                   |
| show_status | boolean           | Show response status or not                                                          |
| show_time   | boolean           | Show taken time or not                                                               |

For any request the field **method** & **url** are required.

### Examples

1. Send 1 simple request

   ```json
   {
     "name": "get my todos",
     "url": "https://jsonplaceholder.typicode.com/todos/1",
     "method": "GET"
   }
   ```

   Output

   ```
   ---

   NAME: get todos
   URL: https://jsonplaceholder.typicode.com/todos/1
   200 OK

   {
     "userId": 1,
     "id": 1,
     "title": "delectus aut autem",
     "completed": false
   }

   completed in 247.8075ms ✨
   ```

2. Send multiple request from 1 one file

   Whatever supplied on root will be used as global variables

   ```json
   {
     "method": "GET",
     "show_time": false,
     "show_output": false,
     "requests": [
       {
         "url": "https://jsonplaceholder.typicode.com/posts",
         "method": "POST",
         "body": {
           "title": "foo",
           "body": "bar",
           "userId": 1
         },
         "headers": {
           "Content-type": "application/json; charset=UTF-8"
         }
       },
       {
         // we don't need to mention METHOD:GET because it is specified globally
         "url": "https://jsonplaceholder.typicode.com/posts/1",
         "show_output": true // here we are overriding the default setting
       },
       {
         "name": "delete post",
         "url": "https://jsonplaceholder.typicode.com/posts/1",
         "method": "DELETE"
       }
     ]
   }
   ```

   Ouput

   ```
   ---

   NAME: POST:https://jsonplaceholder.typicode.com/posts
   URL: https://jsonplaceholder.typicode.com/posts
   201 Created
   ---

   NAME: GET:https://jsonplaceholder.typicode.com/posts/1
   URL: https://jsonplaceholder.typicode.com/posts/1
   200 OK

   {
     "userId": 1,
     "id": 1,
     "title": "sunt aut facere repellat provident occaecati excepturi optio reprehenderit",
     "body": "quia et suscipit\nsuscipit recusandae consequuntur expedita et cum\nreprehenderit molestiae ut ut quas totam\nnostrum rerum est autem sunt rem eveniet architecto"
   }

   ---

   NAME: DELETE:https://jsonplaceholder.typicode.com/posts/1
   URL: https://jsonplaceholder.typicode.com/posts/1
   200 OK
   ```
