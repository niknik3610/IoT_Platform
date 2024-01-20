#make this file executable in the run file
mkdir src/generated
npx pbjs ../proto/frontend/*.proto --target static-module --wrap es6 --out "./src/generated/generated.js" --keep-case
npx pbts ./src/generated/generated.js --out "./src/generated/generated.d.ts" --no-comments --keep-case

# PROTO_DEST=./src/generated
# PROTO_SRC=../proto/frontend/
#
# mkdir -p ${PROTO_DEST}
# mkdir -p temp
#
# cp ${PROTO_SRC}/*.proto ./temp/
#
# cd temp
# protoc \
#     --plugin="protoc-gen-ts=../node_modules/.bin/protoc-gen-ts" \
#     --ts_opt=esModuleInterop=true \
#     --js_out="../src/generated" \
#     --ts_out="../src/generated" \
#      *.proto
# cd ..
