#make this file executable in the run file
npx pbjs ../proto/frontend/*.proto --target static-module --wrap es6 --out "./src/generated/generated.js"
npx pbts ./src/generated/generated.js --out "./src/generated/generated.d.ts" --no-comments
