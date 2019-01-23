import { Generator, Board } from 'world-map-gen';
import { saveAs } from 'file-saver';
import Renderer2D from './2d';
import Renderer3D from './3d';
import { Renderer } from './renderer';

const app = new class {
    private generator: Generator;
    private dim: string;
    private currentBoard: Board | null;
    private widthInput: HTMLInputElement;
    private heightInput: HTMLInputElement;
    private screenRoot: HTMLElement;
    private paintButton: HTMLButtonElement;
    private legends: HTMLElement;
    private renderer: Renderer;
    private downloadJSONButton: HTMLButtonElement;

    constructor() {
        this.generator = Generator.new();
        this.currentBoard = null;

        const selector = document.getElementById('dimension-selector') as HTMLSelectElement;
        const option = selector[selector.selectedIndex] as HTMLOptionElement;
        this.dim = option.value;
        selector.addEventListener('change', this.onVisualizationChange.bind(this));

        this.widthInput = document.getElementById('width-input') as HTMLInputElement;
        this.heightInput = document.getElementById('height-input') as HTMLInputElement;
        this.screenRoot = document.getElementById('screen-root') as HTMLElement;

        this.initRenderer();

        this.paintButton = document.getElementById('paint-button') as HTMLButtonElement;
        this.paintButton.addEventListener('click', () => {
            this.render();
        });

        this.legends = document.getElementById('legends') as HTMLElement;

        this.downloadJSONButton = document.getElementById('download-json-button') as HTMLButtonElement;
        this.downloadJSONButton.addEventListener('click', this.onDownloadJSON.bind(this));
    }

    getSize() {
        const width = parseInt(this.widthInput.value, 10);
        const height = parseInt(this.heightInput.value, 10);
        if (!isNaN(width) && !isNaN(height)) {
            return [width, height];
        }

        if (this.dim === '3d') {
            return [120, 120];
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

    initRenderer() {
        const prev = this.screenRoot.firstChild;
        if (prev !== null) {
            this.screenRoot.removeChild(prev);
        }
        this.currentBoard = null;

        switch (this.dim) {
            case '2d':
                this.renderer = new Renderer2D(this.screenRoot);
                break;
            case '3d':
                this.renderer = new Renderer3D(this.screenRoot);
                break;
            default:
                throw new Error(`Unknown context ${this.dim}`);
        }
    }

    onDownloadJSON(_: Event) {
        if (this.currentBoard === null) {
            return;
        }
        const blob = new Blob([this.currentBoard.as_json()], { type: 'text/plain;charset=utf-8' });
        saveAs(blob, 'board.json');
    }

    onVisualizationChange(event: Event) {
        const selector = event.target as HTMLSelectElement;
        const option = selector[selector.selectedIndex] as HTMLOptionElement;
        const dim = option.value;
        if (this.dim === dim) {
            return;
        }
        this.dim = dim;
        this.initRenderer();
        this.render();
    }

    render() {
        // TODO: Loading indicator cannot be displayed since map generation is run in main thread.
        // When map size is very large and it consumes time, CPU core is also consumed for main thread.
        // In the case, no animation is actually rendered.
        // To prevent this, map generation must be run in another thread and Rust can do it.

        // this.paintButton.classList.add('is-loading');
        this.paintButton.textContent = 'Painting...';
        this.paintButton.classList.add('disabled');
        this.downloadJSONButton.classList.add('disabled');
        // Wait next tick to change text
        window.setTimeout(() => {
            const start = Date.now();
            const [width, height] = this.getSize();
            const board = this.generator.gen_auto(width, height);
            const rendered = this.renderer.render(board);

            this.legends.innerHTML = '';
            for (const legend of rendered.legends.values()) {
                const item = document.createElement('div');
                item.className = 'legend';

                const color = document.createElement('div');
                color.className = 'legend-color';
                color.style.backgroundColor = legend.color || 'none';
                item.appendChild(color);

                const name = document.createElement('div');
                name.className = 'legend-name';
                name.textContent = legend.text;
                item.appendChild(name);

                this.legends.appendChild(item);
            }
            this.currentBoard = board;

            // this.paintButton.classList.remove('is-loading');
            this.paintButton.classList.remove('disabled');
            this.paintButton.textContent = 'Generate';
            this.downloadJSONButton.classList.remove('disabled');
            console.log('Consumed:', Date.now() - start);
        }, 0);
    }
}();

app.render();
