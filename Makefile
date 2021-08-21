build-web:
	CC=emcc wasm-pack build --release --out-dir ./pkg/web --target web

build-bundler:
	CC=emcc wasm-pack build --release --out-dir ./pkg/bundler --target bundler && ./scripts/polyfill-bundler.sh

build-nodejs:
	CC=emcc wasm-pack build --release --out-dir ./pkg/node --target nodejs

test-node:
	make build-nodejs && pushd ./examples/node-test && yarn test ; popd
