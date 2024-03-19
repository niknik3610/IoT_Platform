# Web Frontend

This is the web frontend for the smart home system, built in [Vue](https://vuejs.org/). There is also a commandline version of this frontend, please view the backend reamde for more information.

## Installation and Running

To run the web-frontend you will need to have the newest version of NPM (Node Package Manager) installed. If you need help doing that, follow the [documentation](https://nodejs.org/en/download/package-manager).

Once you have installed node, you can run the serve_frontend.sh script. Before you are able to do this, you will need to give it execution rights using:

```
chmod +x serve_frontend.sh
```

The servce script will then generate the relevant files and start the web-server. You can then access it from your browser at `http://localhost:5173/`. You can change this address to your liking, along with much more, in the vite.config.ts file. For more information view the [Vite documentation](https://vitejs.dev/config/).

Alternatively, you can run the frontend using the included docker container (will be created later). TODO.

## Structure

The web interface is structured like a typical Vue project.

    ├── src/
    │   ├── api_call_links.ts
    │   ├── App.vue
    │   ├── assets/
    │   ├── backend_calls/
    │   ├── components/
    │   ├── generated/
    │   ├── main.ts
    │   ├── router/
    │   ├── shims-vue.d.ts
    │   ├── stores/
    │   ├── types.ts
    │   └── views/

Some specific files and folders to pay attention to are:

-   The file api_call_links.ts: this provides all the JSON api endpoints provided by the backend server.
-   The folder 'backend_calls' contains files which make calls to the JSON api, if you are interested in building your own frontend these might be a good start.
-   The 'generated' folder contains two files, one Typessript type definition file and one native Javascript file using ES6 syntax. These provide the types from the proto/frontend folder, used in making calls to the API framework. For more info on how these are generated view the build_proto.sh script
-   The 'views' folder contains the Vue code to the pages on the site. These are registered in the Vue router, located in router/index.ts
