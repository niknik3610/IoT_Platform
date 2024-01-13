PROTO_DEST=./src/generated
PROTO_SRC=../proto/frontend/

mkdir -p ${PROTO_DEST}

# JavaScript code generation
npx grpc_tools_node_protoc \
    --js_out=import_style=commonjs,binary:${PROTO_DEST} \
    --grpc_out=${PROTO_DEST} \
    --plugin=protoc-gen-grpc=./node_modules/.bin/grpc_tools_node_protoc_plugin \
    -I ../proto/frontend/ \
    ../proto/frontend/*.proto

# TypeScript code generation
npx grpc_tools_node_protoc \
    --plugin=protoc-gen-ts=./node_modules/.bin/protoc-gen-ts \
    --ts_out=${PROTO_DEST} \
    -I ../proto/frontend/ \
    ../proto/frontend/*.proto
