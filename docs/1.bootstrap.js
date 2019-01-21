(window["webpackJsonp"] = window["webpackJsonp"] || []).push([[1],{

/***/ "../pkg sync recursive":
/*!*******************!*\
  !*** ../pkg sync ***!
  \*******************/
/*! no static exports found */
/***/ (function(module, exports) {

eval("function webpackEmptyContext(req) {\n\tvar e = new Error(\"Cannot find module '\" + req + \"'\");\n\te.code = 'MODULE_NOT_FOUND';\n\tthrow e;\n}\nwebpackEmptyContext.keys = function() { return []; };\nwebpackEmptyContext.resolve = webpackEmptyContext;\nmodule.exports = webpackEmptyContext;\nwebpackEmptyContext.id = \"../pkg sync recursive\";\n\n//# sourceURL=webpack:///../pkg_sync?");

/***/ }),

/***/ "../pkg/world_map_gen.js":
/*!*******************************!*\
  !*** ../pkg/world_map_gen.js ***!
  \*******************************/
/*! exports provided: Resolution, LandKind, __wbg_new_ea9a073001008372, __wbg_call_06d69cdad5158109, __wbg_self_df81c815887b068f, __wbg_crypto_456ee09b7417e045, __wbg_getRandomValues_85590dd005d91b46, __wbg_getRandomValues_53fb3b55de90ba82, __wbg_require_fcfcbdfe9a7aebbb, __wbg_randomFillSync_7f42030e46736aa4, __wbindgen_object_drop_ref, __wbindgen_is_undefined, __wbindgen_jsval_eq, Generator, Board, Cell, __wbindgen_throw */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"Resolution\", function() { return Resolution; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"LandKind\", function() { return LandKind; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_new_ea9a073001008372\", function() { return __wbg_new_ea9a073001008372; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_call_06d69cdad5158109\", function() { return __wbg_call_06d69cdad5158109; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_self_df81c815887b068f\", function() { return __wbg_self_df81c815887b068f; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_crypto_456ee09b7417e045\", function() { return __wbg_crypto_456ee09b7417e045; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_getRandomValues_85590dd005d91b46\", function() { return __wbg_getRandomValues_85590dd005d91b46; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_getRandomValues_53fb3b55de90ba82\", function() { return __wbg_getRandomValues_53fb3b55de90ba82; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_require_fcfcbdfe9a7aebbb\", function() { return __wbg_require_fcfcbdfe9a7aebbb; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_randomFillSync_7f42030e46736aa4\", function() { return __wbg_randomFillSync_7f42030e46736aa4; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_object_drop_ref\", function() { return __wbindgen_object_drop_ref; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_is_undefined\", function() { return __wbindgen_is_undefined; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_jsval_eq\", function() { return __wbindgen_jsval_eq; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"Generator\", function() { return Generator; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"Board\", function() { return Board; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"Cell\", function() { return Cell; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_throw\", function() { return __wbindgen_throw; });\n/* harmony import */ var _world_map_gen_bg__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./world_map_gen_bg */ \"../pkg/world_map_gen_bg.wasm\");\n/* tslint:disable */\n\n\n/**\n* Resolution of the board.\n*/\nconst Resolution = Object.freeze({ Low:0,Middle:1,High:2, });\n\nlet cachedTextDecoder = new TextDecoder('utf-8');\n\nlet cachegetUint8Memory = null;\nfunction getUint8Memory() {\n    if (cachegetUint8Memory === null || cachegetUint8Memory.buffer !== _world_map_gen_bg__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer) {\n        cachegetUint8Memory = new Uint8Array(_world_map_gen_bg__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\n    }\n    return cachegetUint8Memory;\n}\n\nfunction getStringFromWasm(ptr, len) {\n    return cachedTextDecoder.decode(getUint8Memory().subarray(ptr, ptr + len));\n}\n\nlet cachedGlobalArgumentPtr = null;\nfunction globalArgumentPtr() {\n    if (cachedGlobalArgumentPtr === null) {\n        cachedGlobalArgumentPtr = _world_map_gen_bg__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_global_argument_ptr\"]();\n    }\n    return cachedGlobalArgumentPtr;\n}\n\nlet cachegetUint32Memory = null;\nfunction getUint32Memory() {\n    if (cachegetUint32Memory === null || cachegetUint32Memory.buffer !== _world_map_gen_bg__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer) {\n        cachegetUint32Memory = new Uint32Array(_world_map_gen_bg__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\n    }\n    return cachegetUint32Memory;\n}\n/**\n* Represents the kind of cell. `preset()` method returns a preset cell constants for\n* the kind.\n*/\nconst LandKind = Object.freeze({ Sea:0,Mountain:1,Forest:2,Plain:3,Town:4,Top:5,Highland:6,DeepSea:7,Path:8, });\n\nconst heap = new Array(32);\n\nheap.fill(undefined);\n\nheap.push(undefined, null, true, false);\n\nlet heap_next = heap.length;\n\nfunction addHeapObject(obj) {\n    if (heap_next === heap.length) heap.push(heap.length + 1);\n    const idx = heap_next;\n    heap_next = heap[idx];\n\n    heap[idx] = obj;\n    return idx;\n}\n\nfunction __wbg_new_ea9a073001008372(arg0, arg1) {\n    let varg0 = getStringFromWasm(arg0, arg1);\n    return addHeapObject(new Function(varg0));\n}\n\nfunction getObject(idx) { return heap[idx]; }\n\nfunction __wbg_call_06d69cdad5158109(arg0, arg1) {\n    return addHeapObject(getObject(arg0).call(getObject(arg1)));\n}\n\nfunction __wbg_self_df81c815887b068f(arg0) {\n    return addHeapObject(getObject(arg0).self);\n}\n\nfunction __wbg_crypto_456ee09b7417e045(arg0) {\n    return addHeapObject(getObject(arg0).crypto);\n}\n\nfunction __wbg_getRandomValues_85590dd005d91b46(arg0) {\n    return addHeapObject(getObject(arg0).getRandomValues);\n}\n\nfunction getArrayU8FromWasm(ptr, len) {\n    return getUint8Memory().subarray(ptr / 1, ptr / 1 + len);\n}\n\nfunction __wbg_getRandomValues_53fb3b55de90ba82(arg0, arg1, arg2) {\n    let varg1 = getArrayU8FromWasm(arg1, arg2);\n    getObject(arg0).getRandomValues(varg1);\n}\n\nfunction __wbg_require_fcfcbdfe9a7aebbb(arg0, arg1) {\n    let varg0 = getStringFromWasm(arg0, arg1);\n    return addHeapObject(__webpack_require__(\"../pkg sync recursive\")(varg0));\n}\n\nfunction __wbg_randomFillSync_7f42030e46736aa4(arg0, arg1, arg2) {\n    let varg1 = getArrayU8FromWasm(arg1, arg2);\n    getObject(arg0).randomFillSync(varg1);\n}\n\nfunction dropObject(idx) {\n    if (idx < 36) return;\n    heap[idx] = heap_next;\n    heap_next = idx;\n}\n\nfunction __wbindgen_object_drop_ref(i) { dropObject(i); }\n\nfunction __wbindgen_is_undefined(idx) {\n    return getObject(idx) === undefined ? 1 : 0;\n}\n\nfunction __wbindgen_jsval_eq(a, b) {\n    return getObject(a) === getObject(b) ? 1 : 0;\n}\n\nfunction freeGenerator(ptr) {\n\n    _world_map_gen_bg__WEBPACK_IMPORTED_MODULE_0__[\"__wbg_generator_free\"](ptr);\n}\n/**\n* Represents random map generator. In contrast to `gen::RandomBoardGen`, it only provides limited\n* functionality. It cannot be initialized with specific seed. And map resolution is always detected\n* from its size.\n*/\nclass Generator {\n\n    static __wrap(ptr) {\n        const obj = Object.create(Generator.prototype);\n        obj.ptr = ptr;\n\n        return obj;\n    }\n\n    free() {\n        const ptr = this.ptr;\n        this.ptr = 0;\n        freeGenerator(ptr);\n    }\n\n    /**\n    * @returns {Generator}\n    */\n    static new() {\n        return Generator.__wrap(_world_map_gen_bg__WEBPACK_IMPORTED_MODULE_0__[\"generator_new\"]());\n    }\n    /**\n    * Generates random map board with given width and height. Parameters are in number of cells.\n    * @param {number} arg0\n    * @param {number} arg1\n    * @returns {Board}\n    */\n    gen_auto(arg0, arg1) {\n        return Board.__wrap(_world_map_gen_bg__WEBPACK_IMPORTED_MODULE_0__[\"generator_gen_auto\"](this.ptr, arg0, arg1));\n    }\n    /**\n    * Generates random map board with given resolution, width and height. Width and height are\n    * in number of cells.\n    * @param {number} arg0\n    * @param {number} arg1\n    * @param {number} arg2\n    * @returns {Board}\n    */\n    gen(arg0, arg1, arg2) {\n        return Board.__wrap(_world_map_gen_bg__WEBPACK_IMPORTED_MODULE_0__[\"generator_gen\"](this.ptr, arg0, arg1, arg2));\n    }\n}\n\nfunction freeBoard(ptr) {\n\n    _world_map_gen_bg__WEBPACK_IMPORTED_MODULE_0__[\"__wbg_board_free\"](ptr);\n}\n/**\n* Represents one board generated by random map generator.\n*/\nclass Board {\n\n    static __wrap(ptr) {\n        const obj = Object.create(Board.prototype);\n        obj.ptr = ptr;\n\n        return obj;\n    }\n\n    free() {\n        const ptr = this.ptr;\n        this.ptr = 0;\n        freeBoard(ptr);\n    }\n\n    /**\n    * @returns {number}\n    */\n    get width() {\n        return _world_map_gen_bg__WEBPACK_IMPORTED_MODULE_0__[\"__wbg_get_board_width\"](this.ptr);\n    }\n    set width(arg0) {\n        return _world_map_gen_bg__WEBPACK_IMPORTED_MODULE_0__[\"__wbg_set_board_width\"](this.ptr, arg0);\n    }\n    /**\n    * @returns {number}\n    */\n    get height() {\n        return _world_map_gen_bg__WEBPACK_IMPORTED_MODULE_0__[\"__wbg_get_board_height\"](this.ptr);\n    }\n    set height(arg0) {\n        return _world_map_gen_bg__WEBPACK_IMPORTED_MODULE_0__[\"__wbg_set_board_height\"](this.ptr, arg0);\n    }\n    /**\n    * Returns cell at position (x, y).\n    * @param {number} arg0\n    * @param {number} arg1\n    * @returns {Cell}\n    */\n    at(arg0, arg1) {\n        return Cell.__wrap(_world_map_gen_bg__WEBPACK_IMPORTED_MODULE_0__[\"board_at\"](this.ptr, arg0, arg1));\n    }\n    /**\n    * Returns color code of land as `#rrggbb` format string from land kind.\n    * @param {number} arg0\n    * @returns {string}\n    */\n    land_color_code(arg0) {\n        const retptr = globalArgumentPtr();\n        _world_map_gen_bg__WEBPACK_IMPORTED_MODULE_0__[\"board_land_color_code\"](retptr, this.ptr, arg0);\n        const mem = getUint32Memory();\n        const rustptr = mem[retptr / 4];\n        const rustlen = mem[retptr / 4 + 1];\n\n        const realRet = getStringFromWasm(rustptr, rustlen).slice();\n        _world_map_gen_bg__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_free\"](rustptr, rustlen * 1);\n        return realRet;\n\n    }\n    /**\n    * Returns RGB color as u32. 0~8 bits for B, 9~16 bits for G, 17~24 bits for R.\n    * When no color is set, returns `None`. It means `undefined` in JavaScript side.\n    * @param {number} arg0\n    * @returns {number | undefined}\n    */\n    land_rgb_color(arg0) {\n        const retptr = globalArgumentPtr();\n\n        _world_map_gen_bg__WEBPACK_IMPORTED_MODULE_0__[\"board_land_rgb_color\"](retptr, this.ptr, arg0);\n        const present = getUint32Memory()[retptr / 4];\n        const value = getUint32Memory()[retptr / 4 + 1];\n        return present === 0 ? undefined : value;\n\n    }\n    /**\n    * Returns legend of land as string from land kind.\n    * @param {number} arg0\n    * @returns {string}\n    */\n    land_legend(arg0) {\n        const retptr = globalArgumentPtr();\n        _world_map_gen_bg__WEBPACK_IMPORTED_MODULE_0__[\"board_land_legend\"](retptr, this.ptr, arg0);\n        const mem = getUint32Memory();\n        const rustptr = mem[retptr / 4];\n        const rustlen = mem[retptr / 4 + 1];\n\n        const realRet = getStringFromWasm(rustptr, rustlen).slice();\n        _world_map_gen_bg__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_free\"](rustptr, rustlen * 1);\n        return realRet;\n\n    }\n}\n\nfunction freeCell(ptr) {\n\n    _world_map_gen_bg__WEBPACK_IMPORTED_MODULE_0__[\"__wbg_cell_free\"](ptr);\n}\n/**\n* Represents one cell in board. In contrast to `land::Land`, it only contains its land kind and\n* altitude in order to reduce total memory size.\n*/\nclass Cell {\n\n    static __wrap(ptr) {\n        const obj = Object.create(Cell.prototype);\n        obj.ptr = ptr;\n\n        return obj;\n    }\n\n    free() {\n        const ptr = this.ptr;\n        this.ptr = 0;\n        freeCell(ptr);\n    }\n\n    /**\n    * @returns {number}\n    */\n    get kind() {\n        return _world_map_gen_bg__WEBPACK_IMPORTED_MODULE_0__[\"__wbg_get_cell_kind\"](this.ptr);\n    }\n    set kind(arg0) {\n        return _world_map_gen_bg__WEBPACK_IMPORTED_MODULE_0__[\"__wbg_set_cell_kind\"](this.ptr, arg0);\n    }\n    /**\n    * @returns {number}\n    */\n    get altitude() {\n        return _world_map_gen_bg__WEBPACK_IMPORTED_MODULE_0__[\"__wbg_get_cell_altitude\"](this.ptr);\n    }\n    set altitude(arg0) {\n        return _world_map_gen_bg__WEBPACK_IMPORTED_MODULE_0__[\"__wbg_set_cell_altitude\"](this.ptr, arg0);\n    }\n}\n\nfunction __wbindgen_throw(ptr, len) {\n    throw new Error(getStringFromWasm(ptr, len));\n}\n\n\n\n//# sourceURL=webpack:///../pkg/world_map_gen.js?");

/***/ }),

/***/ "../pkg/world_map_gen_bg.wasm":
/*!************************************!*\
  !*** ../pkg/world_map_gen_bg.wasm ***!
  \************************************/
/*! exports provided: memory, __wbg_cell_free, __wbg_get_cell_kind, __wbg_set_cell_kind, __wbg_get_cell_altitude, __wbg_set_cell_altitude, __wbg_board_free, __wbg_get_board_width, __wbg_set_board_width, __wbg_get_board_height, __wbg_set_board_height, board_at, board_land_color_code, board_land_rgb_color, board_land_legend, generator_new, generator_gen_auto, generator_gen, __wbg_generator_free, __wbindgen_global_argument_ptr, __wbindgen_free */
/***/ (function(module, exports, __webpack_require__) {

eval("\"use strict\";\n// Instantiate WebAssembly module\nvar wasmExports = __webpack_require__.w[module.i];\n__webpack_require__.r(exports);\n// export exports from WebAssembly module\nfor(var name in wasmExports) if(name != \"__webpack_init__\") exports[name] = wasmExports[name];\n// exec imports from WebAssembly module (for esm order)\n/* harmony import */ var m0 = __webpack_require__(/*! ./world_map_gen */ \"../pkg/world_map_gen.js\");\n\n\n// exec wasm module\nwasmExports[\"__webpack_init__\"]()\n\n//# sourceURL=webpack:///../pkg/world_map_gen_bg.wasm?");

/***/ }),

/***/ "./2d.js":
/*!***************!*\
  !*** ./2d.js ***!
  \***************/
/*! exports provided: default */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"default\", function() { return Renderer2D; });\nclass Renderer2D {\n    constructor(root) {\n        this.canvas = document.createElement('canvas');\n        this.canvas.className = 'screen';\n        root.appendChild(this.canvas);\n\n        this.ctx = this.canvas.getContext('2d');\n    }\n\n    render(board) {\n        const dpr = window.devicePixelRatio || 1;\n        const rect = this.canvas.getBoundingClientRect();\n        this.canvas.width = rect.width * dpr;\n        this.canvas.height = rect.height * dpr;\n\n        // Clear at first\n        this.ctx.clearRect(0, 0, this.canvas.width, this.canvas.height);\n\n        this.ctx.beginPath();\n\n        const width = board.width;\n        const height = board.height;\n        const cellWidth = this.canvas.width / width;\n        const cellHeight = this.canvas.height / height;\n\n        const colors = new Map();\n        for (let x = 0; x < width; x++) {\n            for (let y = 0; y < height; y++) {\n                const cell = board.at(x, y);\n                const kind = cell.kind;\n                let color = colors.get(kind);\n                if (color === undefined) {\n                    color = board.land_color_code(kind);\n                    colors.set(kind, color);\n                }\n                this.ctx.fillStyle = color;\n                this.ctx.fillRect(x * cellWidth, y * cellHeight, cellWidth, cellHeight);\n            }\n        }\n\n        this.ctx.stroke();\n    }\n}\n\n\n//# sourceURL=webpack:///./2d.js?");

/***/ }),

/***/ "./3d.js":
/*!***************!*\
  !*** ./3d.js ***!
  \***************/
/*! exports provided: default */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"default\", function() { return Renderer3D; });\n/* harmony import */ var obelisk_js__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! obelisk.js */ \"./node_modules/obelisk.js/src/obelisk.js\");\n/* harmony import */ var obelisk_js__WEBPACK_IMPORTED_MODULE_0___default = /*#__PURE__*/__webpack_require__.n(obelisk_js__WEBPACK_IMPORTED_MODULE_0__);\n/* harmony import */ var world_map_gen__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! world-map-gen */ \"../pkg/world_map_gen.js\");\n\n\n\nconst CELL_SIZE = 10; // TODO: Temporary\n\nclass Renderer3D {\n    constructor(root) {\n        this.canvas = document.createElement('canvas');\n        this.canvas.className = 'screen';\n        root.appendChild(this.canvas);\n    }\n\n    determineCellSize(width, height) {\n        const both = height + width;\n        const fromHeight = ((this.canvas.height - 200) / both) * 2;\n        const fromWidth = ((this.canvas.width / both) * 2) / Math.sqrt(3);\n        let cellSize = Math.floor(fromHeight > fromWidth ? fromWidth : fromHeight);\n        if (cellSize % 2 === 1) {\n            cellSize--;\n        }\n        return cellSize > 6 ? cellSize : 6;\n    }\n\n    render(board) {\n        const dpr = window.devicePixelRatio || 1;\n        const rect = this.canvas.getBoundingClientRect();\n        this.canvas.width = rect.width * dpr;\n        this.canvas.height = rect.height * dpr;\n\n        const width = board.width;\n        const height = board.height;\n        const cellSize = this.determineCellSize(width, height);\n\n        const point = new obelisk_js__WEBPACK_IMPORTED_MODULE_0__[\"Point\"](this.canvas.width / 2, cellSize + 99);\n        const pixelView = new obelisk_js__WEBPACK_IMPORTED_MODULE_0__[\"PixelView\"](this.canvas, point);\n\n        const cache = new Map();\n        const colors = new Map();\n\n        function kindColor(kind) {\n            const cached = colors.get(kind);\n            if (cached !== undefined) {\n                return cached;\n            }\n            let rgb = board.land_rgb_color(kind);\n            if (rgb === undefined) {\n                rgb = 0xffffff;\n            }\n            const color = new obelisk_js__WEBPACK_IMPORTED_MODULE_0__[\"CubeColor\"]().getByHorizontalColor(rgb);\n            colors.set(kind, color);\n            return color;\n        }\n\n        function calcCube(kind, alt) {\n            const color = kindColor(kind);\n            const height = cellSize + alt * 2;\n            const dim = new obelisk_js__WEBPACK_IMPORTED_MODULE_0__[\"CubeDimension\"](cellSize, cellSize, height);\n            return new obelisk_js__WEBPACK_IMPORTED_MODULE_0__[\"Cube\"](dim, color, /*border:*/ false);\n        }\n\n        function cubeAt(cell) {\n            const kind = cell.kind;\n            const alt = cell.altitude;\n\n            if (kind === world_map_gen__WEBPACK_IMPORTED_MODULE_1__[\"LandKind\"].Town || kind === world_map_gen__WEBPACK_IMPORTED_MODULE_1__[\"LandKind\"].Path) {\n                return calcCube(kind, alt);\n            }\n\n            const cached = cache.get(alt);\n            if (cached !== undefined) {\n                return cached;\n            }\n\n            const cube = calcCube(kind, alt);\n            cache.set(alt, cube);\n            return cube;\n        }\n\n        for (let x = 0; x < width; x++) {\n            for (let y = 0; y < height; y++) {\n                const cube = cubeAt(board.at(x, y));\n                const pt = new obelisk_js__WEBPACK_IMPORTED_MODULE_0__[\"Point3D\"](x * cellSize, y * cellSize, 0);\n                pixelView.renderObject(cube, pt);\n            }\n        }\n    }\n}\n\n\n//# sourceURL=webpack:///./3d.js?");

/***/ }),

/***/ "./index.js":
/*!******************!*\
  !*** ./index.js ***!
  \******************/
/*! no exports provided */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var world_map_gen__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! world-map-gen */ \"../pkg/world_map_gen.js\");\n/* harmony import */ var _2d__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./2d */ \"./2d.js\");\n/* harmony import */ var _3d__WEBPACK_IMPORTED_MODULE_2__ = __webpack_require__(/*! ./3d */ \"./3d.js\");\n\n\n\n\nconst app = new class {\n    constructor() {\n        this.generator = world_map_gen__WEBPACK_IMPORTED_MODULE_0__[\"Generator\"].new();\n\n        const selector = document.getElementById('dimension-selector');\n        this.dim = selector[selector.selectedIndex].value;\n        selector.addEventListener('change', this.onVisualizationChange.bind(this));\n\n        this.widthInput = document.getElementById('width-input');\n        this.heightInput = document.getElementById('height-input');\n        this.screenRoot = document.getElementById('screen-root');\n\n        this.initRenderer();\n\n        this.paintButton = document.getElementById('paint-button');\n        this.paintButton.addEventListener('click', () => {\n            this.render();\n        });\n    }\n\n    getSize() {\n        const width = parseInt(this.widthInput.value, 10);\n        const height = parseInt(this.heightInput.value, 10);\n        if (!isNaN(width) && !isNaN(height)) {\n            return [width, height];\n        }\n\n        if (this.dim === '3d') {\n            return [120, 120];\n        }\n\n        const rect = this.screenRoot.getBoundingClientRect();\n\n        if (!isNaN(width)) {\n            // Note: height is NaN\n            const cellPix = rect.width / width;\n            return [width, Math.floor(rect.height / cellPix)];\n        } else if (!isNaN(height)) {\n            // Note: width is NaN\n            const cellPix = rect.height / height;\n            return [Math.floor(rect.width / cellPix), height];\n        } /* longer side length is 200 cells by default */ else {\n            const max = rect.height > rect.width ? rect.height : rect.width;\n            const cellPix = max / 200;\n            return [Math.floor(rect.width / cellPix), Math.floor(rect.height / cellPix)];\n        }\n    }\n\n    initRenderer() {\n        const prev = this.screenRoot.firstChild;\n        if (prev !== null) {\n            this.screenRoot.removeChild(prev);\n        }\n\n        switch (this.dim) {\n            case '2d':\n                this.renderer = new _2d__WEBPACK_IMPORTED_MODULE_1__[\"default\"](this.screenRoot);\n                break;\n            case '3d':\n                this.renderer = new _3d__WEBPACK_IMPORTED_MODULE_2__[\"default\"](this.screenRoot);\n                break;\n            default:\n                throw new Error(`Unknown context ${dim}`);\n        }\n    }\n\n    onVisualizationChange(event) {\n        const dim = event.target[event.target.selectedIndex].value;\n        if (this.dim === dim) {\n            return;\n        }\n        this.dim = dim;\n        this.initRenderer();\n        this.render();\n    }\n\n    render() {\n        // TODO: Loading indicator cannot be displayed since map generation is run in main thread.\n        // When map size is very large and it consumes time, CPU core is also consumed for main thread.\n        // In the case, no animation is actually rendered.\n        // To prevent this, map generation must be run in another thread and Rust can do it.\n\n        // this.paintButton.classList.add('is-loading');\n        this.paintButton.textContent = 'Painting...';\n        this.paintButton.classList.add('disabled');\n        // Wait next tick to change text\n        window.setTimeout(() => {\n            const start = Date.now();\n            const [width, height] = this.getSize();\n            const board = this.generator.gen_auto(width, height);\n            this.renderer.render(board);\n            // this.paintButton.classList.remove('is-loading');\n            this.paintButton.classList.remove('disabled');\n            this.paintButton.textContent = 'Paint';\n            console.log('Consumed:', Date.now() - start);\n        }, 0);\n    }\n}();\n\napp.render();\n\n\n//# sourceURL=webpack:///./index.js?");

/***/ })

}]);