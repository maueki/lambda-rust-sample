
# How to run

## Setup

```bash
$ git clone https://github.com/maueki/lambda-rust-sample.git
$ cd lambda-rust-sample
$ npm install
```

## Fix serverless package for rust.

```diff
--- node_modules/serverless-offline/src/ServerlessOffline.js.orig       2019-08-15 12:24:05.992472231 +0900
+++ node_modules/serverless-offline/src/ServerlessOffline.js    2019-08-15 12:24:17.608539162 +0900
@@ -385,7 +385,8 @@
         serviceRuntime.startsWith('nodejs') ||
         serviceRuntime.startsWith('python') ||
         serviceRuntime.startsWith('ruby') ||
-        serviceRuntime.startsWith('go')
+        serviceRuntime.startsWith('go') ||
+        serviceRuntime.startsWith('rust')
       )
     ) {
       this.printBlankLine();
```

## Fix ip address for DynamoDB local in main.rs

TODO

## Run

```bash
$ sls offline start
```

Access to http://localhost:3000/movies

