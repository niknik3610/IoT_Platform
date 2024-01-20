#make this file executable in the run file
mkdir src/generated
npx pbjs ../proto/frontend/*.proto --target static-module --wrap es6 --out "./src/generated/generated.js" --keep-case
npx pbts ./src/generated/generated.js --out "./src/generated/generated.d.ts" --no-comments --keep-case
