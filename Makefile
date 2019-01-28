SRCS := $(wildcard src/*.rs)

WASMSRCS := \
	pkg/world_map_gen.d.ts \
	pkg/world_map_gen.js \
	pkg/world_map_gen_bg.wasm \
	pkg/package.json \

DOCSSRCS := $(wildcard docs/*.js) \
	$(wildcard docs/*.wasm) \
	docs/node_modules \
	docs/index.html \
	docs/style.css \

debug: target/debug/world-map-gen

target/debug/world-map-gen: $(SRCS)
	cargo build

release: target/release/world-map-gen

target/release/world-map-gen: $(SRCS)
	cargo build --release

build-wasm-release: $(SRCS)
	wasm-pack build --release

build-wasm-debug: $(SRCS)
	wasm-pack build --dev -- --features wasm_debug

wasm-release: clean build-wasm-release
	wasm-opt -Oz pkg/world_map_gen_bg.wasm -o tmp.wasm
	mv tmp.wasm pkg/world_map_gen_bg.wasm
	cp README.md pkg/
	cp LICENSE.txt pkg/

wasm-debug: build-wasm-debug

www/dist: wasm-release
	cd www && npm run release

release-docs: clean-docs $(DOCSSRCS)

$(DOCSSRCS): www/dist
	cp -R www/dist/* docs/

watch-wasm:
	tmux split-window -v guard
	tmux last-pane
	cd www && npm start

watch:
	cargo watch -x test

all: release release-docs

clean-docs:
	rm -rf $(DOCSSRCS)

clean:
	rm -rf pkg www/dist

.PHONY: \
	debug \
	release \
	wasm \
	wasm-release \
	wasm-debug \
	clean \
	webpack-dist \
	clean-docs \
	watch \
	watch-wasm \
	all \
	build-wasm-release \
	build-wasm-debug
