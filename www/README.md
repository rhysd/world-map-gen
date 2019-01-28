Rust + WebAssembly + TypeScript Demo app with `world-map-gen` package
=====================================================================

This is a demo application for `world-map-gen` wasm package built with
[TypeScript](https://github.com/Microsoft/TypeScript) and [webpack](https://github.com/webpack/webpack).

Snapshot of the built application is put in [`../docs`](../docs) directory and served on
[gh-pages](https://rhysd.github.io/world-map-gen/).

## Development

Install all dependencies by `npm install` before development.

### Development server

Webpack provides a development server with live reload. Please run following command and open
`http://localhost:8080`.

```
npm run start
```

### Build production application

Following command compiles all TypeScript sources, bundles the result into one JavaScript source and
copy all assets (wasm binary, HTML, styles, images...). Built application is put in `dist` directory.

```
npm run build
```

### Run linters

[tslint](https://github.com/palantir/tslint) is used for linting TypeScript sources. And
[stylelint](https://github.com/stylelint/stylelint) is used for linting CSS.

Following command runs linters in parallel.

```
npm run lint
```
