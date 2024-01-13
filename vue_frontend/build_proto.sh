npx pbjs ../proto/frontend/*.proto --target "static" --es6 --out "./src/generated/temp.js"
npx pbts ./src/generated/temp.js --out "./src/generated/generated.ts" --no-comments
rm ./src/generated/temp.js
