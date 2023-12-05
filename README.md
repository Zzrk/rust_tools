## json server examples
```
{
  "posts": [
    { "id": 1, "title": "json-server", "author": "typicode" }
  ],
  "comments": [
    { "id": 1, "body": "some comment", "postId": 1 }
  ],
  "profile": { "name": "typicode" }
}
```

#### GET
- /posts
```
[
  { "id": 1, "title": "json-server", "author": "typicode" }
]
```

- /posts/1
```
{ "id": 1, "title": "json-server", "author": "typicode" }
```