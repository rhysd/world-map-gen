SRCS := $(wildcard src/*.rs)

WASMSRCS := \
	pkg/world_map_gen.d.ts \
	pkg/world_map_gen.js \
	pkg/world_map_gen_bg.wasm \

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

wasm: $(WASMSRCS)

$(WASMSRCS): $(SRCS)
	wasm-pack build --release

wasm-release: clean $(WASMSRCS)
	wasm-opt -Oz pkg/world_map_gen_bg.wasm -o tmp.wasm
	mv tmp.wasm pkg/world_map_gen_bg.wasm

webpack-dist: wasm-release www/dist

www/dist: wasm-release
	cd www && npm run build

release-docs: clean-docs webpack-dist $(DOCSSRCS)

$(DOCSSRCS): www/dist
	cp -R www/dist/* docs/

watch:
	tmux split-window -v guard
	tmux last-pane
	cd www && npm start

all: release release-docs

clean-docs:
	rm -rf $(DOCSSRCS)

clean:
	rm -rf pkg www/dist

.PHONY: debug release wasm wasm-release clean webpack-dist clean-docs watch all
