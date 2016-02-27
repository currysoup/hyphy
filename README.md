# hyphy

Async http webserver as a drop in replacement for server-side hyper.

## Design

```
Async Http Listener = AHL

+-----------------------+
| App                   |
+-----------------------+
| Server                |
+-----------------------+
| AHL | AHL | AHL | ... |
+-----+-----+-----+-----+
```
