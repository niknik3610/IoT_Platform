npx pbjs ../proto/frontend/*.proto --target static-module --wrap es6 --out "./src/generated/temp.js"
npx pbts ./src/generated/temp.js --out "./src/generated/generated.d.ts" --no-comments
rm ./src/generated/temp.js
