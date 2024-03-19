chmod +x build_proto.sh
echo "Building Protobuf Files"
./build_proto.sh

echo "\nRunning NPM Install"
npm install

echo "\nRunning Dev Server"
npm run dev
