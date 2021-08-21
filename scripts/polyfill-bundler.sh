#!/bin/sh

touch pkg/bundler/temp.js
touch pkg/bundler/temp.d.ts
touch pkg/bundler/temp-package.json

sed 's/BigUint64Array/Uint32Array/g' pkg/bundler/twetch_sdk_bg.js > pkg/bundler/temp.js
sed 's/BigInt/number/g' pkg/bundler/twetch_sdk.d.ts > pkg/bundler/temp.d.ts
sed 's/twetch_sdk\.js/twetch_sdk_bg\.js/g' pkg/bundler/package.json > pkg/bundler/temp-package.json

mv pkg/bundler/temp.js pkg/bundler/twetch_sdk_bg.js
mv pkg/bundler/temp.d.ts pkg/bundler/twetch_sdk.d.ts
mv pkg/bundler/temp-package.json pkg/bundler/package.json
