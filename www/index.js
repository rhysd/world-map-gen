import { Generator } from 'world-map-gen';
import Renderer2D from './2d';

const app = new class {
    constructor() {
        this.generator = Generator.new();

        const selector = document.getElementById('dimension-selector');
        this.dimSelIdx = selector.selectedIndex;
        selector.addEventListener('change', this.onVisualizationChange.bind(this));

        this.widthInput = document.getElementById('width-input');
        this.heightInput = document.getElementById('height-input');
        this.screenRoot = document.getElementById('screen-root');

        this.initRenderer('2d');

        this.paintButton = document.getElementById('paint-button');
        this.paintButton.addEventListener('click', () => {
            this.render();
        });
    }

    getSize() {
        const width = parseInt(this.widthInput.value, 10);
        const height = parseInt(this.heightInput.value, 10);
        if (!isNaN(width) && !isNaN(height)) {
            return [width, height];
        }

        const rect = this.screenRoot.getBoundingClientRect();

        if (!isNaN(width)) {
            // Note: height is NaN
            const cellPix = rect.width / width;
            return [width, Math.floor(rect.height / cellPix)];
        } else if (!isNaN(height)) {
            // Note: width is NaN
            const cellPix = rect.height / height;
            return [Math.floor(rect.width / cellPix), height];
        } /* longer side length is 200 cells by default */ else {
            const max = rect.height > rect.width ? rect.height : rect.width;
            const cellPix = max / 200;
            return [Math.floor(rect.width / cellPix), Math.floor(rect.height / cellPix)];
        }
    }

    initRenderer(dim) {
        const prev = this.screenRoot.firstChild;
        if (prev !== null) {
            this.screenRoot.removeChild(prev);
        }

        switch (dim) {
            case '2d':
                this.renderer = new Renderer2D(this.screenRoot);
                break;
            case '3d':
                throw new Error('3D renderer is not yet implemented');
            default:
                throw new Error(`Unknown context ${dim}`);
        }
    }

    onVisualizationChange(event) {
        const idx = event.target.selectedIndex;
        if (this.dimSelIdx === idx) {
            return;
        }
        this.dimSelIdx = idx;
        const dim = event.target[idx].value;
        this.initRenderer(dim);
        this.render();
    }

    render() {
        // TODO: Loading indicator cannot be displayed since map generation is run in main thread.
        // When map size is very large and it consumes time, CPU core is also consumed for main thread.
        // In the case, no animation is actually rendered.
        // To prevent this, map generation must be run in another thread and Rust can do it.

        // this.paintButton.classList.add('is-loading');
        const [width, height] = this.getSize();
        const board = this.generator.gen(width, height);
        this.renderer.render(board);
        // this.paintButton.classList.remove('is-loading');
    }
}();

app.render();
