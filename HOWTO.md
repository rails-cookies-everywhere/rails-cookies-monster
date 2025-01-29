# Current HOWTO before automatization
## 1. List all Rails releases
Step is gonna stay manual for quite some time.
```shell
git tag | grep -v -E 'rc|alpha|beta|RC|PR|pre' | sort -V
```

## 2. Build a base Dockerfile with Rails
Step has been automated.

## 3. Build a version-specific images
Step has been automated.

## 4. Get the cookies
### Run the server
Step has been automated.

### Query the server and extract the cookies
Step has been automated.

## 5. Check the cookies
Step has been automated.
