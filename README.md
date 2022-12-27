# compression bug in salvo

```
git clone https://github.com/prabirshrestha/salvo-proxy-compression-bug.git
cd salvo-proxy-compression-bug
cargo run           # starts salvo server in http://0.0.0.0:7878

# in another tab/window
cd client
npm install
npm start       # starts create-react-app in http://0.0.0.0:3000
```

* Open safari in iphone/ipad
* Navigate to http://ip:7878/server (This renders correctly)
* Navigate to http://ip:7878 (this renders the create react app via proxy and renders garbage)


